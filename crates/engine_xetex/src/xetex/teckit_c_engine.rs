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

pub const kFlags_ExpectsNFC: UInt32 = 0x00000001;
pub const kFlags_ExpectsNFD: UInt32 = 0x00000002;
pub const kFlags_GeneratesNFC: UInt32 = 0x00000004;
pub const kFlags_GeneratesNFD: UInt32 = 0x00000008;

pub const kFlags_VisualOrder: UInt32 = 0x00008000;
pub const kFlags_Unicode: UInt32 = 0x00010000;

pub const kNameID_LHS_Name: UInt16 = 0;
pub const kNameID_RHS_Name: UInt16 = 1;
pub const kNameID_LHS_Description: UInt16 = 2;
pub const kNameID_RHS_Description: UInt16 = 3;
pub const kNameID_Version: UInt16 = 4;
pub const kNameID_Contact: UInt16 = 5;
pub const kNameID_RegAuthority: UInt16 = 6;
pub const kNameID_RegName: UInt16 = 7;
pub const kNameID_Copyright: UInt16 = 8;

pub const kForm_NormalizationMask: UInt16 = 0x0F00;
pub const kForm_NFC: UInt16 = 0x0100;
pub const kForm_NFD: UInt16 = 0x0200;

pub const kEndOfText: UInt32 = 0xffffffff;

#[repr(C)]
pub struct Opaque_TECkit_Converter {
    _unused: [u8; 0],
}
pub type TECkit_Converter = *mut Opaque_TECkit_Converter;

pub const kOptionsMask_UnmappedBehavior: UInt32 = 0x000F;
pub const kOptionsUnmapped_UseReplacementCharSilently: UInt32 = 0x00;
pub const kOptionsUnmapped_UseReplacementCharWithWarning: UInt32 = 0x01;
pub const kOptionsUnmapped_DontUseReplacementChar: UInt32 = 0x02;

pub const kOptionsMask_InputComplete: UInt32 = 0x0100;
pub const kOptionsComplete_InputIsComplete: UInt32 = 0x0100;

pub use super::teckit_engine::{
    TECkit_ConvertBuffer, TECkit_ConvertBufferOpt, TECkit_CreateConverter, TECkit_DisposeConverter, TECkit_Flush, TECkit_FlushOpt,
    TECkit_GetConverterFlags, TECkit_GetConverterName, TECkit_GetMappingFlags, TECkit_GetMappingName, TECkit_GetVersion, TECkit_ResetConverter,
};
