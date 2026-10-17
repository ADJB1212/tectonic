// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! This crate provides a few extern "C" functions that expose the functionality
//! of the flate2 crate in a C API that can be consumed by other C/C++ code in
//! the Tectonic codebase.
//!
//! If you change the interfaces here, rerun cbindgen as described in the README!

use flate2::{Decompress, FlushDecompress, Status};
use std::{io::Error, slice};

mod compress;

pub use compress::{CompressionPoolGuard, use_compression_pool};

/// Re-export of the flate2 crate linked by this crate.
pub use flate2;

/// Outcomes of (de)flate operations.
#[repr(C)]
#[derive(Debug)]
pub enum FlateResult {
    /// The operation succeeded.
    Success = 0,

    /// The operation succeeded and encountered the end of the input.
    StreamEnd = 1,

    /// The operation failed because a buffer was not big enough or full enough.
    BufError = -1,

    /// The operation failed due to an error other than the ones enumerated
    /// here.
    OtherError = -2,
}

impl From<Status> for FlateResult {
    fn from(r: Status) -> Self {
        match r {
            Status::BufError => FlateResult::BufError,
            Status::Ok => FlateResult::Success,
            Status::StreamEnd => FlateResult::StreamEnd,
        }
    }
}

/// Compress a block of data into one zlib stream, using parallel chunks for
/// large inputs when the output buffer has sufficient capacity.
///
/// Returns nonzero on error.
///
/// # Safety
///
/// The input and output pointers must describe valid, nonoverlapping buffers.
/// `output_len` must point to the output capacity, and receives the written
/// length on success or zero on error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tectonic_flate_compress(
    output_ptr: *mut u8,
    output_len: *mut u64,
    input_ptr: *const u8,
    input_len: u64,
    compression_level: u32,
) -> FlateResult {
    unsafe {
        let input = slice::from_raw_parts(input_ptr, input_len as usize);
        let output = slice::from_raw_parts_mut(output_ptr, *output_len as usize);

        let (size, result) = match compress::compress(input, output, compression_level) {
            Ok(size) => (size as u64, FlateResult::Success),
            Err(error) => (0, error),
        };

        *output_len = size;
        result
    }
}

/// Deompress a block of data. This function maps fairly directly onto the
/// `Decompress::decompress` function provided by `flate2`.
///
/// Returns nonzero on error.
///
/// # Safety
///
/// This is a C API function, so it is unsafe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tectonic_flate_decompress(
    output_ptr: *mut u8,
    output_len: *mut u64,
    input_ptr: *const u8,
    input_len: u64,
) -> FlateResult {
    unsafe {
        let mut dc = Decompress::new(true);

        let input = slice::from_raw_parts(input_ptr, input_len as usize);
        let output = slice::from_raw_parts_mut(output_ptr, *output_len as usize);

        let (size, result) = match dc.decompress(input, output, FlushDecompress::Finish) {
            Ok(status) => (dc.total_out(), status.into()),
            Err(_) => (0, FlateResult::OtherError),
        };

        *output_len = size;
        result
    }
}

/// A progressive decompressor of a complete input stream.
struct Decompressor<'a> {
    deflate: Decompress,
    input_data: &'a [u8],
    amount_written: u64,
    done: bool,
}

impl Decompressor<'_> {
    fn decompress_chunk(&mut self, output: &mut [u8]) -> Result<usize, Error> {
        if self.done {
            return Ok(0);
        }

        let inslice = &self.input_data[self.deflate.total_in() as usize..];
        let status = self
            .deflate
            .decompress(inslice, output, FlushDecompress::None)?;
        let delta = self.deflate.total_out() - self.amount_written;
        self.amount_written = self.deflate.total_out();

        match status {
            Status::Ok => {}
            Status::StreamEnd => {
                self.done = true;
            }
            Status::BufError => {
                return Err(Error::other("incomplete input or too-small output buffer"));
            }
        }

        Ok(delta as usize)
    }
}

/// Allocate a new DEFLATE decompressor.
///
/// # Safety
///
/// This is a C API function, so it is unsafe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tectonic_flate_new_decompressor(
    input_ptr: *const u8,
    input_len: u64,
) -> *mut libc::c_void {
    unsafe {
        let input = slice::from_raw_parts(input_ptr, input_len as usize);

        let dc = Decompressor {
            deflate: Decompress::new(true),
            input_data: input,
            amount_written: 0,
            done: false,
        };

        Box::into_raw(Box::new(dc)).cast::<libc::c_void>()
    }
}

/// Decompress some DEFLATEd data.
///
/// After calling this function, the `input_len` parameter is rewritten with the
/// total number of bytes of compressed data that have been read. The
/// `output_len` parameter is rewritten with the total number of bytes of
/// decompressed data that have been written.
///
/// Returns nonzero on error.
///
/// # Safety
///
/// This is a C API function, so it is unsafe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tectonic_flate_decompress_chunk(
    handle: *mut libc::c_void,
    output_ptr: *mut u8,
    output_len: *mut u64,
) -> libc::c_int {
    unsafe {
        let mut dc = Box::from_raw(handle.cast::<Decompressor>());
        let output = slice::from_raw_parts_mut(output_ptr, *output_len as usize);

        let (amount, flag) = match dc.decompress_chunk(output) {
            Ok(n) => (n, 0),
            Err(_) => (0, 1),
        };

        *output_len = amount as u64;
        Box::leak(dc);
        flag
    }
}

/// Deallocate a DEFLATE decompressor.
///
/// # Safety
///
/// This is a C API function, so it is unsafe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tectonic_flate_free_decompressor(handle: *mut libc::c_void) {
    unsafe {
        let _dc = Box::from_raw(handle.cast::<Decompressor>());
        // The box will be freed as we exit.
    }
}
