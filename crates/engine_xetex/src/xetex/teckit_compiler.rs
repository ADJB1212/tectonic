#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use super::teckit_common::*;

pub const kCompilerOpts_FormMask: UInt32 = 0x0000000F;
pub const kCompilerOpts_Compress: UInt32 = 0x00000010;
pub const kCompilerOpts_XML: UInt32 = 0x00000020;

pub type TECkit_ErrorFn =
    Option<unsafe extern "C" fn(userData: *mut libc::c_void, msg: *const libc::c_char, param: *const libc::c_char, line: UInt32)>;

unsafe extern "C" {
    pub fn TECkit_Compile(
        txt: *mut libc::c_char, len: UInt32, doCompression: Byte, errFunc: TECkit_ErrorFn, userData: *mut libc::c_void, outTable: *mut *mut Byte,
        outLen: *mut UInt32,
    ) -> TECkit_Status;

    pub fn TECkit_CompileOpt(
        txt: *mut libc::c_char, len: UInt32, errFunc: TECkit_ErrorFn, userData: *mut libc::c_void, outTable: *mut *mut Byte, outLen: *mut UInt32,
        opts: UInt32,
    ) -> TECkit_Status;

    pub fn TECkit_DisposeCompiled(table: *mut Byte);

    pub fn TECkit_GetCompilerVersion() -> UInt32;

    pub fn TECkit_GetUnicodeName(usv: UInt32) -> *const libc::c_char;

    pub fn TECkit_GetTECkitName(usv: UInt32) -> *mut libc::c_char;

    pub fn TECkit_GetUnicodeValue(name: *mut libc::c_char) -> libc::c_int;
}
