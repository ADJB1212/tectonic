#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

pub type UInt8 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Byte = u8;
pub type BytePtr = *mut u8;
pub type UniChar = u16;

pub type Ptr = *mut libc::c_char;
pub type TextPtr = *mut u8;

pub type TECkit_Status = libc::c_long;

pub const kCurrentTECkitVersion: UInt32 = 0x00020004;

pub const kStatus_NoError: TECkit_Status = 0;

pub const kStatusMask_Basic: TECkit_Status = 0x000000FF;
pub const kStatus_OutputBufferFull: TECkit_Status = 1;
pub const kStatus_NeedMoreInput: TECkit_Status = 2;
pub const kStatus_UnmappedChar: TECkit_Status = 3;

pub const kStatusMask_Warning: TECkit_Status = 0x0000FF00;
pub const kStatus_UsedReplacement: TECkit_Status = 0x00000100;

pub const kStatus_InvalidForm: TECkit_Status = -1;
pub const kStatus_ConverterBusy: TECkit_Status = -2;
pub const kStatus_InvalidConverter: TECkit_Status = -3;
pub const kStatus_InvalidMapping: TECkit_Status = -4;
pub const kStatus_BadMappingVersion: TECkit_Status = -5;
pub const kStatus_Exception: TECkit_Status = -6;
pub const kStatus_NameNotFound: TECkit_Status = -7;
pub const kStatus_IncompleteChar: TECkit_Status = -8;
pub const kStatus_CompilationFailed: TECkit_Status = -9;
pub const kStatus_OutOfMemory: TECkit_Status = -10;

pub const kForm_EncodingFormMask: UInt16 = 0x000F;
pub const kForm_Unspecified: UInt16 = 0;
pub const kForm_Bytes: UInt16 = 1;
pub const kForm_UTF8: UInt16 = 2;
pub const kForm_UTF16BE: UInt16 = 3;
pub const kForm_UTF16LE: UInt16 = 4;
pub const kForm_UTF32BE: UInt16 = 5;
pub const kForm_UTF32LE: UInt16 = 6;
