use libc::{c_char, c_int, ssize_t, time_t};
use tectonic_bridge_core::Diagnostic;

pub type rust_input_handle_t = usize;
pub type rust_output_handle_t = usize;
pub const INVALID_HANDLE: usize = 0;
pub const TTBC_FILE_FORMAT_PICT: c_int = 25;
pub const TTBC_FILE_FORMAT_TEX: c_int = 26;

unsafe extern "C" {
    pub fn ttstub_output_open(path: *const c_char, is_gz: c_int) -> rust_output_handle_t;
    pub fn ttstub_output_putc(handle: rust_output_handle_t, c: c_int) -> c_int;
    pub fn ttstub_output_write(handle: rust_output_handle_t, data: *const c_char, len: usize) -> usize;
    pub fn ttstub_output_close(handle: rust_output_handle_t) -> c_int;
    pub fn ttstub_output_flush(handle: rust_output_handle_t) -> c_int;
    pub fn ttstub_input_open(path: *const c_char, format: c_int, is_gz: c_int) -> rust_input_handle_t;
    pub fn ttstub_input_close(handle: rust_input_handle_t) -> c_int;
    pub fn ttstub_input_get_size(handle: rust_input_handle_t) -> usize;
    pub fn ttstub_input_get_mtime(handle: rust_input_handle_t) -> time_t;
    pub fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: c_int) -> usize;
    pub fn ttstub_input_read(handle: rust_input_handle_t, data: *mut c_char, len: usize) -> ssize_t;
    pub fn ttstub_get_file_md5(path: *const c_char, digest: *mut c_char) -> c_int;
    pub fn ttstub_issue_warning(format: *const c_char, ...);
    pub fn ttstub_issue_error(format: *const c_char, ...);
    pub fn ttstub_diag_printf(diag: *mut Diagnostic, format: *const c_char, ...);
}
