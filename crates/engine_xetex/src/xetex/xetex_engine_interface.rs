#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use std::ffi::{c_char, c_int, c_void};

pub const HISTORY_FATAL_ERROR: c_int = 3;

unsafe extern "C" {
    pub static mut halt_on_error_p: c_int;
    pub static mut in_initex_mode: bool;
    pub static mut synctex_enabled: bool;
    pub static mut semantic_pagination_enabled: bool;
    pub static mut shell_escape_enabled: bool;

    pub fn tt_pdf_document_begin() -> *mut c_void;
    pub fn tt_pdf_document_end(doc: *mut c_void);
    pub fn ttbc_global_engine_enter(api: *mut c_void) -> *mut c_void;
    pub fn ttbc_global_engine_exit();
    pub fn setjmp(env: *mut c_void) -> c_int;
    pub fn tt_run_engine(
        dump_name: *const c_char,
        input_file_name: *const c_char,
        build_date: libc::time_t,
    ) -> super::xetex_xetexd::tt_history_t;
}

#[unsafe(export_name = "tectonic_rust_tt_xetex_set_int_variable")]
pub unsafe extern "C" fn tt_xetex_set_int_variable(var_name: *const c_char, value: c_int) -> c_int {
    unsafe {
        if libc::strcmp(var_name, c"halt_on_error_p".as_ptr()) == 0 {
            halt_on_error_p = value;
        } else if libc::strcmp(var_name, c"in_initex_mode".as_ptr()) == 0 {
            in_initex_mode = value != 0;
        } else if libc::strcmp(var_name, c"synctex_enabled".as_ptr()) == 0 {
            synctex_enabled = value != 0;
        } else if libc::strcmp(var_name, c"semantic_pagination_enabled".as_ptr()) == 0 {
            semantic_pagination_enabled = value != 0;
        } else if libc::strcmp(var_name, c"shell_escape_enabled".as_ptr()) == 0 {
            shell_escape_enabled = value != 0;
        } else {
            return 1;
        }
        0
    }
}

#[unsafe(export_name = "tectonic_rust_tt_xetex_set_string_variable")]
pub unsafe extern "C" fn tt_xetex_set_string_variable(
    var_name: *const c_char,
    value: *const c_char,
) -> c_int {
    1
}

#[unsafe(export_name = "tectonic_rust_tt_engine_xetex_main")]
pub unsafe extern "C" fn tt_engine_xetex_main(
    api: *mut c_void,
    dump_name: *const c_char,
    input_file_name: *const c_char,
    build_date: u64,
) -> c_int {
    unsafe {
        let pdf_document = tt_pdf_document_begin();
        if pdf_document.is_null() {
            return HISTORY_FATAL_ERROR;
        }

        let jmpbuf = ttbc_global_engine_enter(api);
        if setjmp(jmpbuf) != 0 {
            ttbc_global_engine_exit();
            tt_pdf_document_end(pdf_document);
            return HISTORY_FATAL_ERROR;
        }

        let rv = tt_run_engine(dump_name, input_file_name, build_date as libc::time_t);
        ttbc_global_engine_exit();
        tt_pdf_document_end(pdf_document);
        rv as c_int
    }
}

#[cfg(not(target_os = "windows"))]
#[unsafe(export_name = "tectonic_rust___terrible_aarch64_musl_linker_hack_never_call_me")]
pub unsafe extern "C" fn __terrible_aarch64_musl_linker_hack_never_call_me() {
    unsafe {
        unsafe extern "C" {
            fn sigsetjmp(env: *mut c_void, savesigs: c_int) -> c_int;
        }
        let mut buf1 = [0u8; 1024];
        let mut buf2 = [0u8; 1024];
        setjmp(buf1.as_mut_ptr().cast());
        sigsetjmp(buf2.as_mut_ptr().cast(), 0);
    }
}
