// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

use crate::FlateResult;
use flate2::{Compress, Compression, FlushCompress, Status};
use rayon::prelude::*;
use std::{
    cell::RefCell,
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, OnceLock},
    thread,
};

thread_local! {
    static OVERRIDE_POOL: RefCell<Option<Option<Arc<rayon::ThreadPool>>>> = const { RefCell::new(None) };
}

/// Restores the calling thread's compression executor when dropped.
pub struct CompressionPoolGuard {
    previous: Option<Option<Arc<rayon::ThreadPool>>>,
    _thread: PhantomData<Rc<()>>,
}

/// Use the PDF executor for compression; None forces serial compression.
pub fn use_compression_pool(pool: Option<Arc<rayon::ThreadPool>>) -> CompressionPoolGuard {
    let previous = OVERRIDE_POOL.with(|current| current.replace(Some(pool)));
    CompressionPoolGuard {
        previous,
        _thread: PhantomData,
    }
}

impl Drop for CompressionPoolGuard {
    fn drop(&mut self) {
        OVERRIDE_POOL.with(|current| current.replace(self.previous.take()));
    }
}

const CHUNK_SIZE: usize = 256 * 1024;
const PARALLEL_THRESHOLD: usize = 4 * CHUNK_SIZE;
const DICTIONARY_SIZE: usize = 32 * 1024;

fn pool() -> Option<&'static rayon::ThreadPool> {
    static POOL: OnceLock<Option<rayon::ThreadPool>> = OnceLock::new();
    POOL.get_or_init(|| {
        let threads = thread::available_parallelism().map_or(1, usize::from);
        if threads < 2 {
            return None;
        }
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads.min(16))
            .thread_name(|i| format!("tectonic-pdf-{i}"))
            .build()
            .ok()
    })
    .as_ref()
}

fn chunk_bound(length: usize) -> usize {
    length + (length >> 12) + (length >> 14) + (length >> 25) + 32
}

pub(super) fn compress(input: &[u8], output: &mut [u8], level: u32) -> Result<usize, FlateResult> {
    if level > 9 {
        return Err(FlateResult::OtherError);
    }
    if input.len() >= PARALLEL_THRESHOLD {
        let required = input.chunks(CHUNK_SIZE).try_fold(4usize, |size, chunk| {
            size.checked_add(chunk_bound(chunk.len()))
        });
        let configured = OVERRIDE_POOL.with(|current| current.borrow().clone());
        if let Some(pool) = configured {
            if required.is_some_and(|size| size <= output.len()) {
                return match pool {
                    Some(pool) => compress_parallel(input, output, level, &pool),
                    None => compress_chunks(input, output, level, false),
                };
            }
            return compress_serial(input, output, level);
        }
        if rayon::current_thread_index().is_some() {
            if required.is_some_and(|size| size <= output.len()) {
                return compress_chunks(input, output, level, true);
            }
            return compress_serial(input, output, level);
        }
        if let Some(pool) = pool() {
            if required.is_some_and(|size| size <= output.len()) {
                return compress_parallel(input, output, level, pool);
            }
        }
    }
    compress_serial(input, output, level)
}

fn compress_serial(input: &[u8], output: &mut [u8], level: u32) -> Result<usize, FlateResult> {
    let mut compressor = Compress::new(Compression::new(level), true);
    match compressor.compress(input, output, FlushCompress::Finish) {
        Ok(Status::StreamEnd) => Ok(compressor.total_out() as usize),
        Ok(_) => Err(FlateResult::BufError),
        Err(_) => Err(FlateResult::OtherError),
    }
}

struct Chunk<'a> {
    input: &'a [u8],
    dictionary: &'a [u8],
    output: &'a mut [u8],
    first: bool,
    last: bool,
    result: Result<(usize, u32), FlateResult>,
}

impl Chunk<'_> {
    fn compress(&mut self, level: u32) -> Result<(usize, u32), FlateResult> {
        let mut compressor = Compress::new(Compression::new(level), self.first);
        if !self.dictionary.is_empty() {
            compressor
                .set_dictionary(self.dictionary)
                .map_err(|_| FlateResult::OtherError)?;
        }
        let flush = if self.last {
            FlushCompress::Finish
        } else {
            FlushCompress::Sync
        };
        let status = compressor
            .compress(self.input, self.output, flush)
            .map_err(|_| FlateResult::OtherError)?;
        if compressor.total_in() as usize != self.input.len()
            || compressor.total_out() as usize == self.output.len()
            || (self.last && status != Status::StreamEnd)
        {
            return Err(FlateResult::BufError);
        }
        // SAFETY: Every chunk fits in uInt and the input slice is valid.
        let checksum = unsafe {
            libz_sys::adler32(1, self.input.as_ptr(), self.input.len() as libz_sys::uInt)
        };
        Ok((compressor.total_out() as usize, checksum as u32))
    }
}

fn compress_parallel(
    input: &[u8],
    output: &mut [u8],
    level: u32,
    pool: &rayon::ThreadPool,
) -> Result<usize, FlateResult> {
    pool.install(|| compress_chunks(input, output, level, true))
}

fn compress_chunks(
    input: &[u8],
    output: &mut [u8],
    level: u32,
    parallel: bool,
) -> Result<usize, FlateResult> {
    if input.len() <= CHUNK_SIZE {
        return compress(input, output, level);
    }
    let count = input.len().div_ceil(CHUNK_SIZE);
    let mut chunks = Vec::with_capacity(count);
    let mut remaining = &mut *output;
    for (index, bytes) in input.chunks(CHUNK_SIZE).enumerate() {
        let start = index * CHUNK_SIZE;
        let (destination, rest) = remaining.split_at_mut(chunk_bound(bytes.len()));
        remaining = rest;
        chunks.push(Chunk {
            input: bytes,
            dictionary: &input[start.saturating_sub(DICTIONARY_SIZE)..start],
            output: destination,
            first: index == 0,
            last: index + 1 == count,
            result: Err(FlateResult::OtherError),
        });
    }

    // Sync flushes align chunk boundaries; the previous 32 KiB preserves history.
    let compress = |chunk: &mut Chunk<'_>| chunk.result = chunk.compress(level);
    if parallel {
        chunks.par_iter_mut().for_each(compress);
    } else {
        chunks.iter_mut().for_each(compress);
    }

    let mut checksum = 1;
    let mut lengths = Vec::with_capacity(count);
    for chunk in chunks {
        let (written, partial_checksum) = chunk.result?;
        // SAFETY: Combining checksums only uses integer values. Chunk sizes fit z_off_t.
        checksum = unsafe {
            libz_sys::adler32_combine(
                checksum,
                partial_checksum as _,
                chunk.input.len() as libz_sys::z_off_t,
            )
        };
        lengths.push((chunk.output.len(), written));
    }

    let mut source = 0;
    let mut written = 0;
    for (reserved, length) in lengths {
        output.copy_within(source..source + length, written);
        source += reserved;
        written += length;
    }
    output[written..written + 4].copy_from_slice(&(checksum as u32).to_be_bytes());
    Ok(written + 4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::{read::ZlibDecoder, Decompress, FlushDecompress};
    use std::io::Read;

    fn input_data(length: usize) -> Vec<u8> {
        let mut state = 0x1234_5678u32;
        (0..length)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 17;
                state ^= state << 5;
                state as u8
            })
            .collect()
    }

    fn check_stream(compressed: &[u8], input: &[u8]) {
        let mut decoded = Vec::new();
        ZlibDecoder::new(compressed)
            .read_to_end(&mut decoded)
            .unwrap();
        assert_eq!(decoded, input);
        let mut decompressor = Decompress::new(true);
        let mut decoded = vec![0; input.len() + 1];
        assert_eq!(
            decompressor
                .decompress(compressed, &mut decoded, FlushDecompress::Finish)
                .unwrap(),
            Status::StreamEnd
        );
        assert_eq!(decompressor.total_in() as usize, compressed.len());
    }

    #[test]
    fn parallel_compression_emits_one_complete_zlib_stream() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(3)
            .build()
            .unwrap();
        let random = input_data(5 * CHUNK_SIZE + 1);
        for input in [random, vec![b'a'; 5 * CHUNK_SIZE + 1]] {
            for length in [
                CHUNK_SIZE - 1,
                CHUNK_SIZE,
                CHUNK_SIZE + 1,
                input.len() - 1,
                input.len(),
            ] {
                for level in [0, 1, 6, 9] {
                    let input = &input[..length];
                    let mut output = vec![0; input.len() + input.len() / 1000 + 14];
                    let size =
                        compress_parallel(input, &mut output, level, &pool).unwrap_or_else(|_| {
                            panic!("compression failed: length={length}, level={level}")
                        });
                    check_stream(&output[..size], input);
                }
            }
        }
    }

    #[test]
    fn parallel_compression_preserves_history_across_chunk_boundaries() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
        let pattern = input_data(DICTIONARY_SIZE / 2);
        let input: Vec<_> = pattern
            .iter()
            .copied()
            .cycle()
            .take(4 * CHUNK_SIZE)
            .collect();
        let mut output = vec![0; input.len() + input.len() / 1000 + 14];
        let size = compress_parallel(&input, &mut output, 9, &pool)
            .unwrap_or_else(|_| panic!("compression failed"));
        check_stream(&output[..size], &input);
        assert!(size < 2 * pattern.len(), "lost compression history: {size}");
    }

    #[test]
    fn tight_buffers_fall_back_to_serial_compression() {
        let input = vec![0; PARALLEL_THRESHOLD];
        let mut output = vec![0; 2048];
        let size =
            compress(&input, &mut output, 9).unwrap_or_else(|_| panic!("compression failed"));
        check_stream(&output[..size], &input);
    }

    #[test]
    fn compression_reports_short_buffers_and_invalid_levels() {
        let input = input_data(PARALLEL_THRESHOLD);
        let mut output = vec![0; 16];
        let mut output_len = output.len() as u64;
        // SAFETY: Input and output are valid, nonoverlapping buffers.
        let result = unsafe {
            crate::tectonic_flate_compress(
                output.as_mut_ptr(),
                &mut output_len,
                input.as_ptr(),
                input.len() as u64,
                9,
            )
        };
        assert!(matches!(result, FlateResult::BufError));
        assert_eq!(output_len, 0);
        assert!(matches!(
            compress(&input, &mut output, 10),
            Err(FlateResult::OtherError)
        ));
    }

    #[test]
    fn empty_and_small_inputs_round_trip() {
        for input in [b"".as_slice(), b"page contents"] {
            let mut output = vec![0; 128];
            let size =
                compress(input, &mut output, 6).unwrap_or_else(|_| panic!("compression failed"));
            check_stream(&output[..size], input);
        }
    }

    #[test]
    fn worker_count_does_not_change_compressed_bytes() {
        let input = input_data(PARALLEL_THRESHOLD + 71);
        let mut expected = Vec::new();
        for workers in [1, 2, 4] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap();
            let mut output = vec![0; input.len() + input.len() / 1000 + 14];
            let size = compress_parallel(&input, &mut output, 9, &pool).unwrap();
            output.truncate(size);
            if workers == 1 {
                expected = output;
            } else {
                assert_eq!(output, expected);
            }
        }
    }

    #[test]
    #[ignore = "manual PDF stream compression benchmark"]
    fn benchmark_pdf_stream_compression() {
        let pool = pool().expect("benchmark requires multiple CPUs");
        let input = input_data(16 * 1024 * 1024);
        let mut output = vec![0; input.len() + input.len() / 1000 + 14];
        compress_parallel(&input, &mut output, 9, pool).unwrap();
        let start = std::time::Instant::now();
        for _ in 0..3 {
            let mut compressor = Compress::new(Compression::new(9), true);
            assert_eq!(
                compressor
                    .compress(&input, &mut output, FlushCompress::Finish)
                    .unwrap(),
                Status::StreamEnd
            );
            std::hint::black_box(compressor.total_out());
        }
        let serial = start.elapsed();
        let start = std::time::Instant::now();
        for _ in 0..3 {
            std::hint::black_box(compress_parallel(&input, &mut output, 9, pool).unwrap());
        }
        let parallel = start.elapsed();
        eprintln!("16 MiB PDF stream, 3 iterations, {} workers: serial={serial:?}, parallel={parallel:?}, speedup={:.2}x", pool.current_num_threads(), serial.as_secs_f64() / parallel.as_secs_f64());
    }
}
