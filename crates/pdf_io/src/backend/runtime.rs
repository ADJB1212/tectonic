use crate::error::{PdfError, PdfErrorKind};
use std::{
    cell::{Cell, RefCell},
    ffi::{CStr, CString, c_char, c_int, c_void},
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
};
use tectonic_bridge_core::{
    CoreBridgeState, InputId, ttbc_input_close, ttbc_input_seek, ttbc_issue_warning,
    ttbc_output_open_stdout, ttbc_output_write,
};

thread_local! {
    static ERROR: RefCell<Option<CString>> = RefCell::default();
    static QUIET: Cell<i32> = const { Cell::new(0) };
    static LAST_WARNING: Cell<bool> = const { Cell::new(false) };
}

struct Abort(CString);
struct TypedAbort(PdfError);

pub fn is_ascii(value: c_int) -> c_int {
    c_int::from((0..=127).contains(&value))
}

pub fn raise(message: CString) -> ! {
    resume_unwind(Box::new(Abort(message)))
}

pub(crate) fn raise_error(error: PdfError) -> ! {
    resume_unwind(Box::new(TypedAbort(error)))
}

pub(crate) fn legacy_result<T>(result: Result<T, PdfError>) -> T {
    result.unwrap_or_else(|error| raise_error(error))
}

pub(crate) fn catch_backend<T>(call: impl FnOnce() -> Result<T, PdfError>) -> Result<T, PdfError> {
    match catch_unwind(AssertUnwindSafe(call)) {
        Ok(result) => result,
        Err(error) => match backend_error(error) {
            Ok(error) => Err(error),
            Err(error) => resume_unwind(error),
        },
    }
}

fn backend_error(
    error: Box<dyn std::any::Any + Send>,
) -> Result<PdfError, Box<dyn std::any::Any + Send>> {
    match error.downcast::<TypedAbort>() {
        Ok(error) => Ok(error.0),
        Err(error) => match error.downcast::<Abort>() {
            Ok(error) => Ok(PdfError::new(PdfErrorKind::Backend, error.0)),
            Err(error) => Err(error),
        },
    }
}

pub fn boundary<T>(call: impl FnOnce() -> T) -> Result<T, ()> {
    match catch_unwind(AssertUnwindSafe(call)) {
        Ok(value) => Ok(value),
        Err(error) => {
            let message = backend_error(error).map_or_else(
                |_| c"Rust PDF backend failed".to_owned(),
                |error| error.message,
            );
            ERROR.with(|error| *error.borrow_mut() = Some(message));
            Err(())
        }
    }
}

pub(crate) fn result_boundary<T>(call: impl FnOnce() -> Result<T, PdfError>) -> Result<T, ()> {
    boundary(call)?.map_err(|error| {
        ERROR.with(|stored| *stored.borrow_mut() = Some(error.message));
    })
}

pub(crate) fn store_error(error: &PdfError) {
    ERROR.with(|stored| *stored.borrow_mut() = Some(error.message.clone()));
}

#[unsafe(no_mangle)]
pub extern "C" fn tt_pdf_last_error() -> *const c_char {
    ERROR.with(|error| {
        error
            .borrow()
            .as_ref()
            .map_or(c"Rust PDF backend failed".as_ptr(), |error| error.as_ptr())
    })
}

pub fn emit(message: &CStr, warning: bool) {
    if QUIET.with(|quiet| quiet.get()) > i32::from(warning) {
        return;
    }
    let result = CoreBridgeState::with_global_state(|state| {
        emit_to(state, message, warning, LAST_WARNING.with(Cell::get))
    });
    legacy_result(result);
    LAST_WARNING.with(|last| last.set(warning));
}

pub(crate) fn emit_to(
    state: &mut CoreBridgeState<'_>,
    message: &CStr,
    warning: bool,
    preceded_by_warning: bool,
) -> Result<(), PdfError> {
    let write = |state: &mut CoreBridgeState<'_>, bytes: &[u8]| -> Result<(), PdfError> {
        let Some(handle) = ttbc_output_open_stdout(state) else {
            return Err(PdfError::new(
                PdfErrorKind::Io,
                c"xdvipdfmx cannot get output logging handle?!".to_owned(),
            ));
        };
        unsafe {
            ttbc_output_write(state, Some(handle), bytes.as_ptr(), bytes.len());
        }
        Ok(())
    };
    if warning {
        if !preceded_by_warning {
            write(state, b"\n")?;
        }
        write(state, b"warning: ")?;
        unsafe {
            ttbc_issue_warning(state, message.as_ptr());
        }
    }
    write(state, message.to_bytes())?;
    if warning {
        write(state, b"\n")?;
    }
    Ok(())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tt_pdf_emit_message(text: *const c_char, warning: c_int) -> c_int {
    boundary(|| emit(unsafe { CStr::from_ptr(text) }, warning != 0)).map_or(-1, |_| 0)
}

#[unsafe(no_mangle)]
pub extern "C" fn shut_up(quietness: c_int) {
    QUIET.with(|quiet| quiet.set(quietness));
}

pub unsafe fn allocate(size: usize) -> *mut c_void {
    let pointer = unsafe { libc::malloc(size.max(1)) };
    if pointer.is_null() {
        raise(CString::new("Out of memory").unwrap());
    }
    pointer
}

pub unsafe fn reallocate(pointer: *mut c_void, size: usize) -> *mut c_void {
    let pointer = unsafe { libc::realloc(pointer, size.max(1)) };
    if pointer.is_null() {
        raise(CString::new("Out of memory").unwrap());
    }
    pointer
}

pub unsafe fn duplicate(text: *const c_char) -> *mut c_char {
    let bytes = unsafe { CStr::from_ptr(text) }.to_bytes_with_nul();
    let result = unsafe { allocate(bytes.len()) }.cast();
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr().cast(), result, bytes.len());
    }
    result
}

pub(crate) fn input_id(handle: usize) -> Option<InputId> {
    // InputId is transparent over NonZeroUsize; zero represents None.
    unsafe { std::mem::transmute(handle) }
}

pub unsafe fn input_seek(handle: usize, offset: isize, whence: c_int) -> usize {
    legacy_result(CoreBridgeState::with_global_state(|state| {
        seek_input(state, handle, offset, whence)
    }))
}

pub(crate) fn seek_input(
    state: &mut CoreBridgeState<'_>,
    handle: usize,
    offset: isize,
    whence: c_int,
) -> Result<usize, PdfError> {
    let mut error = 0;
    let position = unsafe { ttbc_input_seek(state, input_id(handle), offset, whence, &mut error) };
    if error != 0 {
        return Err(PdfError::new(
            PdfErrorKind::InvalidState,
            c"Invalid PDF input seek".to_owned(),
        ));
    }
    Ok(position)
}

pub fn input_close(handle: usize) -> c_int {
    legacy_result(CoreBridgeState::with_global_state(|state| {
        close_input(state, handle)
    }));
    0
}

pub(crate) fn close_input(state: &mut CoreBridgeState<'_>, handle: usize) -> Result<(), PdfError> {
    let error = ttbc_input_close(state, input_id(handle));
    if error != 0 {
        return Err(PdfError::new(
            PdfErrorKind::Io,
            c"Could not close PDF input".to_owned(),
        ));
    }
    Ok(())
}

unsafe extern "C" {
    #[link_name = "tt_pdf_mkstemp"]
    pub fn temporary_file(template: *mut c_char) -> c_int;
    #[link_name = "tt_pdf_errno"]
    pub fn errno() -> *mut c_int;
    #[link_name = "tt_pdf_stdout"]
    pub fn stdout() -> *mut c_void;
    #[link_name = "tt_pdf_stderr"]
    pub fn stderr() -> *mut c_void;
}
