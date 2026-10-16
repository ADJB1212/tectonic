// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

use rayon::prelude::*;
use std::{slice, sync::OnceLock, thread};

const PARALLEL_THRESHOLD: usize = 256 * 1024;

extern "C" {
    fn tt_pdf_filter_png_rows(
        raster: *const u8,
        destination: *mut u8,
        row_bytes: i32,
        bytes_per_pixel: i32,
        first_row: i32,
        last_row: i32,
    );
}

fn pool() -> Option<&'static rayon::ThreadPool> {
    static POOL: OnceLock<Option<rayon::ThreadPool>> = OnceLock::new();
    POOL.get_or_init(|| {
        let threads = thread::available_parallelism().map_or(1, usize::from);
        if threads < 2 {
            return None;
        }
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads.min(16))
            .thread_name(|i| format!("tectonic-image-{i}"))
            .build()
            .ok()
    })
    .as_ref()
}

fn filter_parallel(
    source: &[u8],
    destination: &mut [u8],
    row_bytes: usize,
    bytes_per_pixel: i32,
    pool: &rayon::ThreadPool,
) {
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
#[no_mangle]
unsafe extern "C" fn tt_pdf_filter_png_parallel(
    raster: *const u8,
    destination: *mut u8,
    row_bytes: i32,
    rows: i32,
    bytes_per_pixel: i32,
) {
    if rows == 0 {
        return;
    }
    if row_bytes == 0 {
        slice::from_raw_parts_mut(destination, rows as usize).fill(0);
        return;
    }
    let source = slice::from_raw_parts(raster, row_bytes as usize * rows as usize);
    let destination =
        slice::from_raw_parts_mut(destination, (row_bytes as usize + 1) * rows as usize);
    if source.len() >= PARALLEL_THRESHOLD {
        if let Some(pool) = pool() {
            filter_parallel(
                source,
                destination,
                row_bytes as usize,
                bytes_per_pixel,
                pool,
            );
            return;
        }
    }
    tt_pdf_filter_png_rows(
        source.as_ptr(),
        destination.as_mut_ptr(),
        row_bytes,
        bytes_per_pixel,
        0,
        rows,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parallel_png_predictors_match_serial_rows() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(3)
            .build()
            .unwrap();
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
                filter_parallel(
                    &source,
                    &mut actual,
                    row_bytes,
                    bytes_per_pixel as i32,
                    &pool,
                );
                assert_eq!(
                    actual, expected,
                    "bytes_per_pixel={bytes_per_pixel}, rows={rows}"
                );
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
        filter_parallel(&source, &mut output, row_bytes, 3, pool);
        let start = std::time::Instant::now();
        for _ in 0..3 {
            // SAFETY: The buffers have the sizes required for these raster dimensions.
            unsafe {
                tt_pdf_filter_png_rows(
                    source.as_ptr(),
                    output.as_mut_ptr(),
                    row_bytes as i32,
                    3,
                    0,
                    rows as i32,
                );
            }
            std::hint::black_box(&output);
        }
        let serial = start.elapsed();
        let start = std::time::Instant::now();
        for _ in 0..3 {
            filter_parallel(&source, &mut output, row_bytes, 3, pool);
            std::hint::black_box(&output);
        }
        let parallel = start.elapsed();
        eprintln!("12 MiB PDF image, 3 iterations, {} workers: serial={serial:?}, parallel={parallel:?}, speedup={:.2}x", pool.current_num_threads(), serial.as_secs_f64() / parallel.as_secs_f64());
    }
}
