//! Libpng ownership and C-local error containment.

use crate::{
    backend::runtime,
    error::{PdfError, PdfErrorKind},
};
use std::{
    ffi::{CStr, c_char, c_void},
    io::Read,
    ptr::NonNull,
};

unsafe extern "C" {
    fn tt_pdf_png_create(error: *mut u8, handle: usize) -> *mut c_void;
    fn tt_pdf_png_create_info(png: *mut c_void) -> *mut c_void;
    fn tt_pdf_png_destroy(png: *mut c_void, info: *mut c_void);
    fn tt_pdf_png_error(png: *mut c_void) -> *const c_char;
    fn tt_pdf_png_read_info(png: *mut c_void, info: *mut c_void) -> i32;
    fn tt_pdf_png_update_info(png: *mut c_void, info: *mut c_void) -> i32;
    fn tt_pdf_png_read_image(png: *mut c_void, rows: *mut *mut u8) -> i32;
    fn tt_pdf_png_read_end(png: *mut c_void, info: *mut c_void) -> i32;
    fn tt_pdf_png_set_gamma(png: *mut c_void, screen: f64, file: f64) -> i32;
    fn tt_pdf_png_set_background(
        png: *mut c_void,
        color: *const c_void,
        code: i32,
        expand: i32,
        gamma: f64,
    ) -> i32;
}

pub(crate) struct Reader {
    png: NonNull<c_void>,
    info: *mut c_void,
    _error: Box<[u8; 1024]>,
}

impl Reader {
    pub(crate) fn new(handle: usize) -> Result<Self, PdfError> {
        let mut error = Box::new([0; 1024]);
        // SAFETY: The boxed error buffer remains stable until after native destruction.
        let png = NonNull::new(unsafe { tt_pdf_png_create(error.as_mut_ptr(), handle) })
            .ok_or_else(|| {
                PdfError::new(
                    PdfErrorKind::Backend,
                    c"PNG: Creating Libpng read struct failed.".into(),
                )
            })?;
        let mut reader = Self {
            png,
            info: std::ptr::null_mut(),
            _error: error,
        };
        // SAFETY: The owner is initialized before this fallible native allocation.
        reader.info = unsafe { tt_pdf_png_create_info(reader.png.as_ptr()) };
        if reader.info.is_null() {
            return Err(PdfError::new(
                PdfErrorKind::Backend,
                c"PNG: Creating Libpng info struct failed.".into(),
            ));
        }
        Ok(reader)
    }

    pub(crate) fn png(&self) -> *mut c_void {
        self.png.as_ptr()
    }
    pub(crate) fn info(&self) -> *mut c_void {
        self.info
    }
}

impl Drop for Reader {
    fn drop(&mut self) {
        // SAFETY: This owner exclusively destroys both native allocations before its error buffer.
        unsafe {
            tt_pdf_png_destroy(self.png.as_ptr(), self.info);
        }
    }
}

unsafe fn check(png: *mut c_void, status: i32) -> Result<(), PdfError> {
    if status == 0 {
        return Ok(());
    }
    // SAFETY: A failed protected operation leaves a NUL-terminated message in the owner's buffer.
    let message = unsafe { CStr::from_ptr(tt_pdf_png_error(png)) }.to_owned();
    Err(PdfError::new(PdfErrorKind::MalformedInput, message))
}

// All pointers below are borrowed from a live Reader. Each C function catches
// libpng's longjmp before returning, including errors raised by the input callback.
pub(crate) unsafe fn read_info(png: *mut c_void, info: *mut c_void) -> Result<(), PdfError> {
    unsafe { check(png, tt_pdf_png_read_info(png, info)) }
}
pub(crate) unsafe fn update_info(png: *mut c_void, info: *mut c_void) -> Result<(), PdfError> {
    unsafe { check(png, tt_pdf_png_update_info(png, info)) }
}
pub(crate) unsafe fn read_image(png: *mut c_void, rows: *mut *mut u8) -> Result<(), PdfError> {
    unsafe { check(png, tt_pdf_png_read_image(png, rows)) }
}
pub(crate) unsafe fn read_end(png: *mut c_void, info: *mut c_void) -> Result<(), PdfError> {
    unsafe { check(png, tt_pdf_png_read_end(png, info)) }
}
pub(crate) unsafe fn set_gamma(png: *mut c_void, screen: f64, file: f64) -> Result<(), PdfError> {
    unsafe { check(png, tt_pdf_png_set_gamma(png, screen, file)) }
}
pub(crate) unsafe fn set_background(
    png: *mut c_void,
    color: *const c_void,
    code: i32,
    expand: i32,
    gamma: f64,
) -> Result<(), PdfError> {
    unsafe {
        check(
            png,
            tt_pdf_png_set_background(png, color, code, expand, gamma),
        )
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_png_input_read(handle: usize, bytes: *mut u8, length: usize) -> i32 {
    let result = runtime::boundary(|| {
        if length == 0 {
            return Ok(());
        }
        // SAFETY: Libpng supplies a writable buffer of the requested length.
        let bytes = unsafe { std::slice::from_raw_parts_mut(bytes, length) };
        super::input::ProbeInput { handle }.read_exact(bytes)
    });
    if matches!(result, Ok(Ok(()))) { 0 } else { -1 }
}
