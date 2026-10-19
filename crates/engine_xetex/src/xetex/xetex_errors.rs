#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use crate::xetex::xetex_core::*;
use std::ffi::{c_char, c_int, c_void};

pub const BATCH_MODE: u8 = 0;
pub const SCROLL_MODE: u8 = 2;
pub const ERROR_STOP_MODE: u8 = 3;

pub const HISTORY_ERROR_ISSUED: i32 = 2;
pub const HISTORY_FATAL_ERROR: i32 = 3;

unsafe extern "C" {
    pub static mut log_opened: bool;
    pub static mut selector: selector_t;
    pub static mut job_name: i32;
    pub static mut interaction: u8;
    pub static mut history: i32;
    pub static mut halt_on_error_p: c_int;
    pub static mut error_count: i8;
    pub static mut use_err_help: bool;
    pub static mut help_ptr: u8;
    pub static mut help_line: [*const c_char; 6];
    pub static mut rust_stdout: usize;

    pub fn open_log_file();
    pub fn error_here_with_diagnostic(s: *const c_char) -> *mut tectonic_bridge_core::Diagnostic;
    pub fn capture_to_diagnostic(p: *mut tectonic_bridge_core::Diagnostic);
    pub fn close_files_and_terminate();
    pub fn tt_cleanup();
    pub fn ttstub_output_flush(handle: usize) -> libc::c_int;
    pub fn print_char(s: i32);
    pub fn print(s: i32);
    pub fn print_cstr(s: *const c_char);
    pub fn print_nl_cstr(s: *const c_char);
    pub fn print_int(n: i32);
    pub fn print_ln();
    pub fn show_context();
    pub fn give_err_help();
    pub fn _tt_abort(format: *const c_char, ...) -> !;
}

unsafe fn pre_error_message() {
    unsafe {
        if log_opened {
            selector = SELECTOR_TERM_AND_LOG;
        } else {
            selector = SELECTOR_TERM_ONLY;
        }

        if job_name == 0 {
            open_log_file();
        }

        if interaction == BATCH_MODE {
            selector -= 1;
        }

        error_here_with_diagnostic(c"".as_ptr());
    }
}

unsafe fn post_error_message(need_to_print_it: c_int) {
    unsafe {
        capture_to_diagnostic(std::ptr::null_mut());

        if interaction == ERROR_STOP_MODE {
            interaction = SCROLL_MODE;
        }

        if need_to_print_it != 0 && log_opened {
            error();
        }

        history = HISTORY_FATAL_ERROR;
        close_files_and_terminate();
        tt_cleanup();
        ttstub_output_flush(rust_stdout);
    }
}

#[unsafe(export_name = "error")]
pub unsafe extern "C" fn error() {
    unsafe {
        if history < HISTORY_ERROR_ISSUED {
            history = HISTORY_ERROR_ISSUED;
        }

        print_char('.' as i32);
        show_context();
        if halt_on_error_p != 0 {
            history = HISTORY_FATAL_ERROR;
            post_error_message(0);
            _tt_abort(c"halted on potentially-recoverable error as specified".as_ptr());
        }

        error_count += 1;
        if error_count == 100 {
            print_nl_cstr(c"(That makes 100 errors; please try again.)".as_ptr());
            history = HISTORY_FATAL_ERROR;
            post_error_message(0);
            _tt_abort(c"halted after 100 potentially-recoverable errors".as_ptr());
        }

        if interaction > BATCH_MODE {
            selector -= 1;
        }

        if use_err_help {
            print_ln();
            give_err_help();
        } else {
            while help_ptr > 0 {
                help_ptr -= 1;
                print_nl_cstr(help_line[help_ptr as usize]);
            }
        }

        print_ln();
        if interaction > BATCH_MODE {
            selector += 1;
        }
        print_ln();
    }
}

#[unsafe(export_name = "fatal_error")]
pub unsafe extern "C" fn fatal_error(s: *const c_char) {
    unsafe {
        pre_error_message();
        print_cstr(c"Emergency stop".as_ptr());
        print_nl_cstr(s);
        capture_to_diagnostic(std::ptr::null_mut());

        close_files_and_terminate();
        tt_cleanup();
        ttstub_output_flush(rust_stdout);
        _tt_abort(c"%s".as_ptr(), s);
    }
}

#[unsafe(export_name = "overflow")]
pub unsafe extern "C" fn overflow(s: *const c_char, n: i32) {
    unsafe {
        pre_error_message();
        print_cstr(c"TeX capacity exceeded, sorry [".as_ptr());

        print_cstr(s);
        print_char('=' as i32);
        print_int(n);
        print_char(']' as i32);

        help_ptr = 2;
        help_line[1] = c"If you really absolutely need more capacity,".as_ptr();
        help_line[0] = c"you can ask a wizard to enlarge me.".as_ptr();
        post_error_message(1);
        _tt_abort(c"halted on overflow()".as_ptr());
    }
}

#[unsafe(export_name = "confusion")]
pub unsafe extern "C" fn confusion(s: *const c_char) {
    unsafe {
        pre_error_message();

        if history < HISTORY_ERROR_ISSUED {
            print_cstr(c"This can't happen (".as_ptr());
            print_cstr(s);
            print_char(')' as i32);

            help_ptr = 1;
            help_line[0] = c"I'm broken. Please show this to someone who can fix can fix".as_ptr();
        } else {
            print_cstr(c"I can't go on meeting you like this".as_ptr());

            help_ptr = 2;
            help_line[1] = c"One of your faux pas seems to have wounded me deeply...".as_ptr();
            help_line[0] = c"in fact, I'm barely conscious. Please fix it and try again.".as_ptr();
        }

        post_error_message(1);
        _tt_abort(c"halted on confusion()".as_ptr());
    }
}

#[unsafe(export_name = "pdf_error")]
pub unsafe extern "C" fn pdf_error(t: *const c_char, p: *const c_char) {
    unsafe {
        pre_error_message();

        print_cstr(c"Error".as_ptr());

        if !t.is_null() {
            print_cstr(c" (".as_ptr());
            print_cstr(t);
            print(')' as i32);
        }

        print_cstr(c": ".as_ptr());
        print_cstr(p);

        post_error_message(1);
        _tt_abort(c"halted on pdf_error()".as_ptr());
    }
}
