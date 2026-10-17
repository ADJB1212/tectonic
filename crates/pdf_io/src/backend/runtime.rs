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

pub fn is_ascii(value: c_int) -> c_int {
    c_int::from((0..=127).contains(&value))
}

pub fn raise(message: CString) -> ! {
    resume_unwind(Box::new(Abort(message)))
}

pub fn boundary<T>(call: impl FnOnce() -> T) -> Result<T, ()> {
    match catch_unwind(AssertUnwindSafe(call)) {
        Ok(value) => Ok(value),
        Err(error) => {
            let message = match error.downcast::<Abort>() {
                Ok(error) => error.0,
                Err(_) => CString::new("Rust PDF backend failed").unwrap(),
            };
            ERROR.with(|error| *error.borrow_mut() = Some(message));
            Err(())
        }
    }
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
    CoreBridgeState::with_global_state(|state| {
        let write = |state: &mut CoreBridgeState<'_>, bytes: &[u8]| {
            let Some(handle) = ttbc_output_open_stdout(state) else {
                raise(CString::new("xdvipdfmx cannot get output logging handle?!").unwrap());
            };
            unsafe {
                ttbc_output_write(state, Some(handle), bytes.as_ptr(), bytes.len());
            }
        };
        if warning {
            if !LAST_WARNING.with(Cell::get) {
                write(state, b"\n");
            }
            write(state, b"warning: ");
            unsafe {
                ttbc_issue_warning(state, message.as_ptr());
            }
        }
        write(state, message.to_bytes());
        if warning {
            write(state, b"\n");
        }
    });
    LAST_WARNING.with(|last| last.set(warning));
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

fn input_id(handle: usize) -> Option<InputId> {
    // InputId is transparent over NonZeroUsize; zero represents None.
    unsafe { std::mem::transmute(handle) }
}

pub unsafe fn input_seek(handle: usize, offset: isize, whence: c_int) -> usize {
    let mut error = 0;
    let position = CoreBridgeState::with_global_state(|state| unsafe {
        ttbc_input_seek(state, input_id(handle), offset, whence, &mut error)
    });
    if error != 0 {
        raise(c"Invalid PDF input seek".to_owned());
    }
    position
}

pub fn input_close(handle: usize) -> c_int {
    let error =
        CoreBridgeState::with_global_state(|state| ttbc_input_close(state, input_id(handle)));
    if error != 0 {
        raise(c"Could not close PDF input".to_owned());
    }
    0
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
