// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

use rayon::prelude::*;
use std::{
    slice,
    sync::{Arc, OnceLock},
    thread,
};

const PARALLEL_THRESHOLD: usize = 256 * 1024;

unsafe extern "C" {
    fn tt_pdf_filter_png_rows(raster: *const u8, destination: *mut u8, row_bytes: i32, bytes_per_pixel: i32, first_row: i32, last_row: i32);
}

pub(crate) fn pool() -> Option<Arc<rayon::ThreadPool>> {
    if crate::pipeline::configured() {
        return crate::pipeline::pool();
    }
    static POOL: OnceLock<Option<Arc<rayon::ThreadPool>>> = OnceLock::new();
    POOL.get_or_init(|| {
        let threads = thread::available_parallelism().map_or(1, usize::from);
        if threads < 2 {
            return None;
        }
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads.min(16))
            .thread_name(|i| format!("tectonic-pdf-io-{i}"))
            .build()
            .ok()
            .map(Arc::new)
    })
    .clone()
}

fn filter_parallel(source: &[u8], destination: &mut [u8], row_bytes: usize, bytes_per_pixel: i32, pool: &rayon::ThreadPool) {
    let rows = source.len() / row_bytes;
    let rows_per_chunk = rows.div_ceil(pool.current_num_threads() * 4);
    pool.install(|| {
        destination
            .par_chunks_mut(rows_per_chunk * (row_bytes + 1))
            .enumerate()
            .for_each(|(index, chunk)| {
                let first_row = index * rows_per_chunk;
                let last_row = first_row + chunk.len() / (row_bytes + 1);
                // SAFETY: Workers read the immutable raster and write disjoint row ranges.
                unsafe {
                    tt_pdf_filter_png_rows(
                        source.as_ptr(),
                        chunk.as_mut_ptr(),
                        row_bytes as i32,
                        bytes_per_pixel,
                        first_row as i32,
                        last_row as i32,
                    );
                }
            });
    });
}

/// Apply the PNG predictor to a raster, processing independent rows concurrently.
///
/// # Safety
/// The buffers must be nonoverlapping and valid for `row_bytes * rows` input
/// bytes and `(row_bytes + 1) * rows` output bytes. Dimensions must be nonnegative
/// and products must fit i32. `bytes_per_pixel` must be positive.
#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_filter_png_parallel(raster: *const u8, destination: *mut u8, row_bytes: i32, rows: i32, bytes_per_pixel: i32) {
    unsafe {
        if rows == 0 {
            return;
        }
        if row_bytes == 0 {
            slice::from_raw_parts_mut(destination, rows as usize).fill(0);
            return;
        }
        let source = slice::from_raw_parts(raster, row_bytes as usize * rows as usize);
        let destination = slice::from_raw_parts_mut(destination, (row_bytes as usize + 1) * rows as usize);
        if source.len() >= PARALLEL_THRESHOLD
            && let Some(pool) = pool()
        {
            filter_parallel(source, destination, row_bytes as usize, bytes_per_pixel, &pool);
            return;
        }
        tt_pdf_filter_png_rows(source.as_ptr(), destination.as_mut_ptr(), row_bytes, bytes_per_pixel, 0, rows);
    }
}

fn filter_tiff_row(source: &[u8], destination: &mut [u8], columns: usize, bpc: usize, colors: usize) {
    match bpc {
        8 => {
            for (index, byte) in destination.iter_mut().enumerate() {
                let previous = if index >= colors { source[index - colors] } else { 0 };
                *byte = source[index].wrapping_sub(previous);
            }
        }
        16 => {
            for (index, bytes) in destination.as_chunks_mut::<2>().0.iter_mut().enumerate() {
                let offset = index * 2;
                let current = u16::from_be_bytes([source[offset], source[offset + 1]]);
                let previous = if index >= colors {
                    let offset = offset - colors * 2;
                    u16::from_be_bytes([source[offset], source[offset + 1]])
                } else {
                    0
                };
                bytes.copy_from_slice(&current.wrapping_sub(previous).to_be_bytes());
            }
        }
        1 | 2 | 4 => {
            destination.fill(0);
            let mask = (1u8 << bpc) - 1;
            let sample = |index: usize| (source[index * bpc / 8] >> (8 - bpc - index * bpc % 8)) & mask;
            for index in 0..columns * colors {
                let previous = if index >= colors { sample(index - colors) } else { 0 };
                let difference = sample(index).wrapping_sub(previous) & mask;
                destination[index * bpc / 8] |= difference << (8 - bpc - index * bpc % 8);
            }
        }
        _ => destination.copy_from_slice(source),
    }
}

fn filter_tiff(source: &[u8], destination: &mut [u8], columns: usize, bpc: usize, colors: usize, pool: Option<&rayon::ThreadPool>) {
    let row_bytes = (columns * bpc * colors).div_ceil(8);
    if let Some(pool) = pool {
        let rows_per_chunk = (source.len() / row_bytes).div_ceil(pool.current_num_threads() * 4);
        pool.install(|| {
            destination
                .par_chunks_mut(rows_per_chunk * row_bytes)
                .zip(source.par_chunks(rows_per_chunk * row_bytes))
                .for_each(|(destination, source)| {
                    for (source, destination) in source.chunks_exact(row_bytes).zip(destination.chunks_exact_mut(row_bytes)) {
                        filter_tiff_row(source, destination, columns, bpc, colors);
                    }
                });
        });
    } else {
        for (source, destination) in source.chunks_exact(row_bytes).zip(destination.chunks_exact_mut(row_bytes)) {
            filter_tiff_row(source, destination, columns, bpc, colors);
        }
    }
}

/// Apply horizontal differencing to independent image rows.
///
/// # Safety
/// Buffers must be valid and nonoverlapping for `ceil(columns * bpc * colors / 8)
/// * rows` bytes. Dimensions must be nonnegative with products fitting i32;
/// `bpc` and `colors` must be positive.
#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_filter_tiff_parallel(raster: *const u8, destination: *mut u8, columns: i32, rows: i32, bpc: i32, colors: i32) {
    unsafe {
        let length = (columns as usize * bpc as usize * colors as usize).div_ceil(8) * rows as usize;
        if length == 0 {
            return;
        }
        let source = slice::from_raw_parts(raster, length);
        let destination = slice::from_raw_parts_mut(destination, length);
        let pool = if length >= PARALLEL_THRESHOLD { pool() } else { None };
        filter_tiff(source, destination, columns as usize, bpc as usize, colors as usize, pool.as_deref());
    }
}

pub(crate) fn filter_owned(
    mut source: Vec<u8>, predictor: i32, columns: i32, bpc: i32, colors: i32, pool: Option<&rayon::ThreadPool>,
) -> Result<Vec<u8>, ()> {
    if predictor == 0 {
        return Ok(source);
    }
    let columns = usize::try_from(columns).map_err(|_| ())?;
    let bpc = usize::try_from(bpc).map_err(|_| ())?;
    let colors = usize::try_from(colors).map_err(|_| ())?;
    if columns == 0 || colors == 0 || !matches!(bpc, 1 | 2 | 4 | 8 | 16) {
        return Err(());
    }
    let row_bytes = columns
        .checked_mul(bpc)
        .and_then(|n| n.checked_mul(colors))
        .and_then(|n| n.checked_add(7))
        .ok_or(())?
        / 8;
    if row_bytes == 0 || row_bytes > i32::MAX as usize || !source.len().is_multiple_of(row_bytes) {
        return Err(());
    }
    let rows = source.len() / row_bytes;
    if rows > i32::MAX as usize {
        return Err(());
    }
    match predictor {
        2 => {
            let filter = |row: &mut [u8]| {
                if bpc == 8 {
                    for index in (colors..row.len()).rev() {
                        row[index] = row[index].wrapping_sub(row[index - colors]);
                    }
                } else if bpc == 16 {
                    for index in (colors..row.len() / 2).rev() {
                        let offset = index * 2;
                        let previous = offset - colors * 2;
                        let value =
                            u16::from_be_bytes([row[offset], row[offset + 1]]).wrapping_sub(u16::from_be_bytes([row[previous], row[previous + 1]]));
                        row[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
                    }
                } else {
                    let mask = (1u8 << bpc) - 1;
                    for index in (colors..columns * colors).rev() {
                        let bit = index * bpc;
                        let previous = (index - colors) * bpc;
                        let shift = 8 - bpc - bit % 8;
                        let current = (row[bit / 8] >> shift) & mask;
                        let previous = (row[previous / 8] >> (8 - bpc - previous % 8)) & mask;
                        let difference = current.wrapping_sub(previous) & mask;
                        row[bit / 8] = (row[bit / 8] & !(mask << shift)) | (difference << shift);
                    }
                    let padding = (8 - columns * colors * bpc % 8) % 8;
                    if padding != 0 {
                        row[row_bytes - 1] &= 255u8 << padding;
                    }
                }
            };
            if let Some(pool) = pool.filter(|_| source.len() >= PARALLEL_THRESHOLD) {
                let rows_per_chunk = rows.div_ceil(pool.current_num_threads() * 4);
                pool.install(|| {
                    source
                        .par_chunks_mut(rows_per_chunk * row_bytes)
                        .for_each(|chunk| chunk.chunks_exact_mut(row_bytes).for_each(filter))
                });
            } else {
                source.chunks_exact_mut(row_bytes).for_each(filter);
            }
            Ok(source)
        }
        15 => {
            let mut destination = vec![0; source.len().checked_add(rows).ok_or(())?];
            let bytes_per_pixel = (bpc * colors).div_ceil(8) as i32;
            if let Some(pool) = pool.filter(|_| source.len() >= PARALLEL_THRESHOLD) {
                filter_parallel(&source, &mut destination, row_bytes, bytes_per_pixel, pool);
            } else {
                // SAFETY: Dimensions and buffer lengths were checked above.
                unsafe {
                    tt_pdf_filter_png_rows(
                        source.as_ptr(),
                        destination.as_mut_ptr(),
                        row_bytes as i32,
                        bytes_per_pixel,
                        0,
                        rows as i32,
                    );
                }
            }
            Ok(destination)
        }
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pack_samples(samples: &[u16], bpc: usize) -> Vec<u8> {
        let mut bytes = vec![0; (samples.len() * bpc).div_ceil(8)];
        for (index, &sample) in samples.iter().enumerate() {
            for bit in 0..bpc {
                let offset = index * bpc + bit;
                bytes[offset / 8] |= (((sample >> (bpc - 1 - bit)) & 1) as u8) << (7 - offset % 8);
            }
        }
        bytes
    }

    #[test]
    fn tiff_predictors_preserve_samples_and_row_boundaries() {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(3).build().unwrap();
        for bpc in [1, 2, 4, 8, 16] {
            for colors in [1, 3, 4] {
                for columns in [1, 7, 257] {
                    for rows in [1, 2, 13, 1025] {
                        let mut source = Vec::new();
                        let mut expected = Vec::new();
                        let mask = ((1u32 << bpc) - 1) as u16;
                        for row in 0..rows {
                            let samples: Vec<u16> = (0..columns * colors)
                                .map(|index| ((index * 4093 + row * 8191 + index / colors * 17) as u16) & mask)
                                .collect();
                            let differences: Vec<u16> = samples
                                .iter()
                                .enumerate()
                                .map(|(index, &sample)| {
                                    let previous = if index >= colors { samples[index - colors] } else { 0 };
                                    sample.wrapping_sub(previous) & mask
                                })
                                .collect();
                            let mut packed = pack_samples(&samples, bpc);
                            let padding = (8 - samples.len() * bpc % 8) % 8;
                            *packed.last_mut().unwrap() |= ((1u16 << padding) - 1) as u8;
                            source.extend(packed);
                            expected.extend(pack_samples(&differences, bpc));
                        }
                        let mut actual = vec![0xff; expected.len()];
                        for pool in [None, Some(&pool)] {
                            filter_tiff(&source, &mut actual, columns, bpc, colors, pool);
                            assert_eq!(actual, expected, "bpc={bpc}, colors={colors}, columns={columns}, rows={rows}");
                            let owned = source.clone();
                            let pointer = owned.as_ptr();
                            let filtered = filter_owned(owned, 2, columns as i32, bpc as i32, colors as i32, pool).unwrap();
                            assert_eq!(filtered, expected);
                            assert_eq!(filtered.as_ptr(), pointer);
                        }
                        // SAFETY: Buffers are nonoverlapping and match the raster dimensions.
                        unsafe {
                            tt_pdf_filter_tiff_parallel(
                                source.as_ptr(),
                                actual.as_mut_ptr(),
                                columns as i32,
                                rows as i32,
                                bpc as i32,
                                colors as i32,
                            );
                        }
                        assert_eq!(actual, expected);
                    }
                }
            }
        }
    }

    #[test]
    fn empty_tiff_rasters_do_not_access_buffers() {
        let pointer = std::ptr::null_mut();
        // SAFETY: Empty rasters do not require valid buffer pointers.
        unsafe {
            tt_pdf_filter_tiff_parallel(pointer, pointer, 0, 10, 8, 3);
            tt_pdf_filter_tiff_parallel(pointer, pointer, 10, 0, 8, 3);
        }
    }

    #[test]
    #[ignore = "manual TIFF image predictor benchmark"]
    fn benchmark_pdf_tiff_predictor() {
        let pool = pool().expect("benchmark requires multiple CPUs");
        let (columns, rows, colors, bpc) = (1024, 4096, 3, 16);
        let source: Vec<_> = (0..columns * rows * colors * 2).map(|i| (i % 251) as u8).collect();
        let mut output = vec![0; source.len()];
        filter_tiff(&source, &mut output, columns, bpc, colors, Some(&pool));
        let measure = |output: &mut [u8], pool| {
            let start = std::time::Instant::now();
            for _ in 0..10 {
                filter_tiff(&source, output, columns, bpc, colors, pool);
                std::hint::black_box(&output);
            }
            start.elapsed()
        };
        let serial = measure(&mut output, None);
        let parallel = measure(&mut output, Some(&pool));
        eprintln!(
            "24 MiB TIFF predictor, 10 iterations, {} workers: serial={serial:?}, parallel={parallel:?}, speedup={:.2}x",
            pool.current_num_threads(),
            serial.as_secs_f64() / parallel.as_secs_f64()
        );
    }

    #[test]
    fn parallel_png_predictors_match_serial_rows() {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(3).build().unwrap();
        for bytes_per_pixel in [1, 2, 3, 4, 6, 8] {
            let row_bytes = 257 * bytes_per_pixel;
            for rows in [1, 2, 13, 1025] {
                let source: Vec<u8> = (0..row_bytes * rows)
                    .map(|index| ((index * 31 + index / row_bytes * 17) % 256) as u8)
                    .collect();
                let mut expected = vec![0; (row_bytes + 1) * rows];
                let mut actual = vec![0; expected.len()];
                // SAFETY: The buffers have the sizes required for these raster dimensions.
                unsafe {
                    tt_pdf_filter_png_rows(
                        source.as_ptr(),
                        expected.as_mut_ptr(),
                        row_bytes as i32,
                        bytes_per_pixel as i32,
                        0,
                        rows as i32,
                    );
                }
                filter_parallel(&source, &mut actual, row_bytes, bytes_per_pixel as i32, &pool);
                assert_eq!(actual, expected, "bytes_per_pixel={bytes_per_pixel}, rows={rows}");
            }
        }
    }

    #[test]
    #[ignore = "manual PDF image predictor benchmark"]
    fn benchmark_pdf_image_predictor() {
        let pool = pool().expect("benchmark requires multiple CPUs");
        let (row_bytes, rows) = (3 * 1024, 4096);
        let source: Vec<_> = (0..row_bytes * rows).map(|i| (i % 251) as u8).collect();
        let mut output = vec![0; (row_bytes + 1) * rows];
        filter_parallel(&source, &mut output, row_bytes, 3, &pool);
        let start = std::time::Instant::now();
        for _ in 0..3 {
            // SAFETY: The buffers have the sizes required for these raster dimensions.
            unsafe {
                tt_pdf_filter_png_rows(source.as_ptr(), output.as_mut_ptr(), row_bytes as i32, 3, 0, rows as i32);
            }
            std::hint::black_box(&output);
        }
        let serial = start.elapsed();
        let start = std::time::Instant::now();
        for _ in 0..3 {
            filter_parallel(&source, &mut output, row_bytes, 3, &pool);
            std::hint::black_box(&output);
        }
        let parallel = start.elapsed();
        eprintln!(
            "12 MiB PDF image, 3 iterations, {} workers: serial={serial:?}, parallel={parallel:?}, speedup={:.2}x",
            pool.current_num_threads(),
            serial.as_secs_f64() / parallel.as_secs_f64()
        );
    }
}
