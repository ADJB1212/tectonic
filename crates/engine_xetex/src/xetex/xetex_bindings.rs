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
use tectonic_bridge_core::CoreBridgeState;

pub const FORMAT_SERIAL: u32 = 33;

pub type ttbc_state_t<'a> = CoreBridgeState<'a>;

unsafe extern "C" {
    pub fn tt_checkpoint_set(bytes: *const u8, length: usize, enabled: bool);
    pub fn tt_checkpoint_skipped() -> u64;
    pub fn tt_checkpoint_restored() -> bool;
    pub fn tt_xetex_checkpoint_emit(bytes: *const u8, length: usize);
    pub fn tt_xetex_checkpoint_ready() -> bool;
    pub fn tt_xetex_checkpoint_layout(checkpoint: *mut c_void, restoring: bool) -> bool;
    pub fn tt_xetex_checkpoint_handle_valid(handle: usize, output: bool) -> bool;
    pub fn tt_xetex_set_int_variable(var_name: *const c_char, value: c_int) -> c_int;
    pub fn tt_engine_xetex_main(api: &mut ttbc_state_t, dump_name: *const c_char, input_file_name: *const c_char, build_date: u64) -> c_int;
}
