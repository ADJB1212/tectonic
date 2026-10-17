// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

use rayon::prelude::*;
use std::slice;
use tectonic_bridge_flate::tectonic_flate_compress;

#[derive(Clone, Copy)]
pub(crate) struct StreamOptions {
    pub level: u32,
    pub predictor: i32,
    pub columns: i32,
    pub bpc: i32,
    pub colors: i32,
}

pub(crate) fn compress_owned(
    input: Vec<u8>,
    options: StreamOptions,
    pool: Option<&rayon::ThreadPool>,
    timings: &crate::pipeline::WorkerTimings,
) -> Result<Vec<u8>, ()> {
    let start = std::time::Instant::now();
    let input = crate::predictor::filter_owned(
        input,
        options.predictor,
        options.columns,
        options.bpc,
        options.colors,
        pool,
    )?;
    crate::pipeline::record_elapsed(&timings.predictors, start);
    let capacity = input
        .len()
        .checked_add(input.len() / 1000)
        .and_then(|n| n.checked_add(128))
        .ok_or(())?;
    let mut output = vec![0; capacity];
    let mut length = capacity as u64;
    let start = std::time::Instant::now();
    // SAFETY: The owned buffers are disjoint and live until compression finishes.
    let status = unsafe {
        tectonic_flate_compress(
            output.as_mut_ptr(),
            &mut length,
            input.as_ptr(),
            input.len() as u64,
            options.level,
        )
    };
    crate::pipeline::record_elapsed(&timings.compression, start);
    if (status as i32) < 0 {
        return Err(());
    }
    output.truncate(length as usize);
    Ok(output)
}

#[repr(C)]
struct CompressionJob {
    input: *const u8,
    input_len: u64,
    output: *mut u8,
    output_len: u64,
    result: i32,
}

/// Compress independent streams while their owning engine is paused.
///
/// # Safety
/// Jobs and their buffers must be valid for the duration of this call. Output
/// buffers must be disjoint from all other buffers and from the jobs array.
#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_compress_streams(jobs: *mut CompressionJob, count: usize, level: u32) {
    unsafe {
        let jobs = slice::from_raw_parts_mut(jobs, count);
        let mut buffers: Vec<_> = jobs
            .iter_mut()
            .map(|job| {
                (
                    slice::from_raw_parts(job.input, job.input_len as usize),
                    slice::from_raw_parts_mut(job.output, job.output_len as usize),
                    &mut job.output_len,
                    &mut job.result,
                )
            })
            .collect();
        let compress =
            |(input, output, length, result): &mut (&[u8], &mut [u8], &mut u64, &mut i32)| {
                // SAFETY: Each job exclusively owns its output and only reads its input.
                **result = tectonic_flate_compress(
                    output.as_mut_ptr(),
                    *length,
                    input.as_ptr(),
                    input.len() as u64,
                    level,
                ) as i32;
            };
        if count > 1
            && let Some(pool) = crate::predictor::pool()
        {
            pool.install(|| buffers.par_iter_mut().for_each(compress));
            return;
        }
        buffers.iter_mut().for_each(compress);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tectonic_bridge_flate::FlateResult;
    use tectonic_bridge_flate::flate2::read::ZlibDecoder;

    #[test]
    fn independent_streams_round_trip_and_report_individual_errors() {
        let inputs: Vec<Vec<u8>> = (0..64)
            .map(|index| {
                (0..16384 + index * 31)
                    .map(|offset| ((offset + index) % 251) as u8)
                    .collect()
            })
            .collect();
        let mut outputs: Vec<_> = inputs
            .iter()
            .map(|input| vec![0; input.len() + 128])
            .collect();
        outputs[17].truncate(1);
        let mut jobs: Vec<_> = inputs
            .iter()
            .zip(&mut outputs)
            .map(|(input, output)| CompressionJob {
                input: input.as_ptr(),
                input_len: input.len() as u64,
                output: output.as_mut_ptr(),
                output_len: output.len() as u64,
                result: -2,
            })
            .collect();
        // SAFETY: All jobs borrow live, disjoint buffers until compression returns.
        unsafe {
            tt_pdf_compress_streams(jobs.as_mut_ptr(), jobs.len(), 9);
        }
        for (index, ((job, input), output)) in jobs.iter().zip(&inputs).zip(&outputs).enumerate() {
            if index == 17 {
                assert_eq!(job.result, FlateResult::BufError as i32);
                assert_eq!(job.output_len, 0);
            } else {
                assert_eq!(job.result, FlateResult::Success as i32);
                let mut decoded = Vec::new();
                ZlibDecoder::new(&output[..job.output_len as usize])
                    .read_to_end(&mut decoded)
                    .unwrap();
                assert_eq!(&decoded, input);
            }
        }
    }
}
