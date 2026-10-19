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

pub const kMagicNumber: UInt32 = 0x714d6170;
pub const kMagicNumberCmp: UInt32 = 0x7a516d70;

pub const kFileVersion2_1: UInt32 = 0x00020001;
pub const kCurrentFileVersion: UInt32 = 0x00030000;

pub const kTableVersion2: UInt32 = 0x00020000;
pub const kCurrentTableVersion: UInt32 = 0x00030000;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NameRec {
    pub nameID: UInt16,
    pub nameLength: UInt16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FileHeader {
    pub type_: UInt32,
    pub version: UInt32,
    pub headerLength: UInt32,
    pub formFlagsLHS: UInt32,
    pub formFlagsRHS: UInt32,
    pub numNames: UInt32,
    pub numFwdTables: UInt32,
    pub numRevTables: UInt32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TableHeader {
    pub type_: UInt32,
    pub version: UInt32,
    pub length: UInt32,
    pub flags: UInt32,
    pub pageBase: UInt32,
    pub lookupBase: UInt32,
    pub matchClassBase: UInt32,
    pub repClassBase: UInt32,
    pub stringListBase: UInt32,
    pub stringRuleData: UInt32,
    pub maxMatch: UInt8,
    pub maxPre: UInt8,
    pub maxPost: UInt8,
    pub maxOutput: UInt8,
    pub replacementChar: UInt32,
}

pub const kTableType_BB: UInt32 = 0x422d3e42;
pub const kTableType_BU: UInt32 = 0x422d3e55;
pub const kTableType_UB: UInt32 = 0x552d3e42;
pub const kTableType_UU: UInt32 = 0x552d3e55;

pub const kTableType_NFC: UInt32 = 0x4e464320;
pub const kTableType_NFD: UInt32 = 0x4e464420;

pub const kTableFlags_Supplementary: UInt32 = 0x0001;
pub const kTableFlags_DBCS: UInt32 = 0x0002;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LookupRules {
    pub type_: UInt8,
    pub ruleCount: UInt8,
    pub ruleIndex: UInt16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LookupBytes {
    pub count: UInt8,
    pub data: [UInt8; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union Lookup {
    pub rules: LookupRules,
    pub bytes: LookupBytes,
    pub usv: UInt32,
}

pub const kLookupType_StringRules: UInt8 = 0xff;
pub const kLookupType_IllegalDBCS: UInt8 = 0xfe;
pub const kLookupType_Unmapped: UInt8 = 0xfd;

pub const kLookupType_RuleTypeMask: UInt8 = 0xc0;
pub const kLookupType_ExtStringRules: UInt8 = 0x80;
pub const kLookupType_ExtRuleCountMask: UInt8 = 0x3f;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct StringRule {
    pub matchLength: UInt8,
    pub postLength: UInt8,
    pub preLength: UInt8,
    pub repLength: UInt8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchElemFlags {
    pub repeat: UInt8,
    pub type_: UInt8,
    pub reserved: UInt16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchElemBGroup {
    pub reserved: UInt16,
    pub dNext: UInt8,
    pub dAfter: UInt8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchElemEGroup {
    pub reserved: UInt16,
    pub dNext: UInt8,
    pub dStart: UInt8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchElemClass {
    pub reserved: UInt16,
    pub index: UInt16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchElemByte {
    pub reserved: [UInt8; 3],
    pub data: UInt8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchElemUSV {
    pub data: UInt32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union MatchElemValue {
    pub bgroup: MatchElemBGroup,
    pub egroup: MatchElemEGroup,
    pub cls: MatchElemClass,
    pub byte: MatchElemByte,
    pub usv: MatchElemUSV,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union MatchElem {
    pub flags: MatchElemFlags,
    pub value: MatchElemValue,
}

pub const kMatchElem_Negate: UInt8 = 0x80;
pub const kMatchElem_NonLit: UInt8 = 0x40;

pub const kMatchElem_TypeMask: UInt8 = 0x3f;
pub const kMatchElem_Type_Class: UInt8 = 0x01;
pub const kMatchElem_Type_BGroup: UInt8 = 0x02;
pub const kMatchElem_Type_EGroup: UInt8 = 0x03;
pub const kMatchElem_Type_OR: UInt8 = 0x04;
pub const kMatchElem_Type_ANY: UInt8 = 0x05;
pub const kMatchElem_Type_EOS: UInt8 = 0x06;
pub const kMatchElem_Type_Copy: UInt8 = 0x07;

pub const kUSVMask: UInt32 = 0x001fffff;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RepElemFlags {
    pub type_: UInt8,
    pub matchIndex: UInt8,
    pub repClass: UInt16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union RepElem {
    pub flags: RepElemFlags,
    pub value: UInt32,
}

pub const kRepElem_Literal: UInt8 = 0x00;
pub const kRepElem_Class: UInt8 = kMatchElem_Type_Class;
pub const kRepElem_Copy: UInt8 = kMatchElem_Type_Copy;
pub const kRepElem_Unmapped: UInt8 = 0x0f;
