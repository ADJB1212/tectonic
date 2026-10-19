#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

pub type scaled_t = i32;

pub type selector_t = i32;
pub const SELECTOR_FILE_0: selector_t = 0;
pub const SELECTOR_FILE_15: selector_t = 15;
pub const SELECTOR_NO_PRINT: selector_t = 16;
pub const SELECTOR_TERM_ONLY: selector_t = 17;
pub const SELECTOR_LOG_ONLY: selector_t = 18;
pub const SELECTOR_TERM_AND_LOG: selector_t = 19;
pub const SELECTOR_PSEUDO: selector_t = 20;
pub const SELECTOR_NEW_STRING: selector_t = 21;

pub const FONT_FLAGS_COLORED: u32 = 0x01;
pub const FONT_FLAGS_VERTICAL: u32 = 0x02;

#[cfg(target_endian = "big")]
pub const US_NATIVE_UTF16: &str = "UTF16BE";
#[cfg(not(target_endian = "big"))]
pub const US_NATIVE_UTF16: &str = "UTF16LE";
