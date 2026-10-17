/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2019 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
#![allow(
    warnings,
    clippy::all,
    unsafe_op_in_unsafe_fn,
    static_mut_refs,
    dangerous_implicit_autorefs
)]
unsafe extern "C-unwind" {
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: ::core::ffi::c_int,
    ) -> size_t;
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> ssize_t;
    #[link_name = "__pdf_impl_tt_get_unsigned_byte"]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_tt_get_signed_byte"]
    fn tt_get_signed_byte(handle: rust_input_handle_t) -> ::core::ffi::c_schar;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_tt_get_signed_pair"]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> ::core::ffi::c_short;
    #[link_name = "__pdf_impl_tt_get_unsigned_quad"]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    #[link_name = "__pdf_impl_put_big_endian"]
    fn put_big_endian(
        s: *mut ::core::ffi::c_void,
        q: SFNT_LONG,
        n: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_sfnt_find_table_len"]
    fn sfnt_find_table_len(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_sfnt_find_table_pos"]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_sfnt_locate_table"]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
pub type rust_input_handle_t = uintptr_t;
pub type BYTE = ::core::ffi::c_uchar;
pub type SFNT_CHAR = ::core::ffi::c_schar;
pub type USHORT = ::core::ffi::c_ushort;
pub type SHORT = ::core::ffi::c_short;
pub type SFNT_ULONG = uint32_t;
pub type SFNT_LONG = int32_t;
pub type Fixed = uint32_t;
pub type FWord = ::core::ffi::c_short;
pub type uFWord = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [::core::ffi::c_char; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: SFNT_ULONG,
    pub num_tables: USHORT,
    pub search_range: USHORT,
    pub entry_selector: USHORT,
    pub range_shift: USHORT,
    pub num_kept_tables: USHORT,
    pub flags: *mut ::core::ffi::c_char,
    pub tables: *mut sfnt_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: ::core::ffi::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_head_table {
    pub version: Fixed,
    pub fontRevision: Fixed,
    pub checkSumAdjustment: SFNT_ULONG,
    pub magicNumber: SFNT_ULONG,
    pub flags: USHORT,
    pub unitsPerEm: USHORT,
    pub created: [BYTE; 8],
    pub modified: [BYTE; 8],
    pub xMin: FWord,
    pub yMin: FWord,
    pub xMax: FWord,
    pub yMax: FWord,
    pub macStyle: USHORT,
    pub lowestRecPPEM: USHORT,
    pub fontDirectionHint: SHORT,
    pub indexToLocFormat: SHORT,
    pub glyphDataFormat: SHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_hhea_table {
    pub version: Fixed,
    pub ascent: FWord,
    pub descent: FWord,
    pub lineGap: FWord,
    pub advanceWidthMax: uFWord,
    pub minLeftSideBearing: FWord,
    pub minRightSideBearing: FWord,
    pub xMaxExtent: FWord,
    pub caretSlopeRise: SHORT,
    pub caretSlopeRun: SHORT,
    pub caretOffset: FWord,
    pub reserved: [SHORT; 4],
    pub metricDataFormat: SHORT,
    pub numOfLongHorMetrics: USHORT,
    pub numOfExSideBearings: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_vhea_table {
    pub version: Fixed,
    pub vertTypoAscender: SHORT,
    pub vertTypoDescender: SHORT,
    pub vertTypoLineGap: SHORT,
    pub advanceHeightMax: SHORT,
    pub minTopSideBearing: SHORT,
    pub minBottomSideBearing: SHORT,
    pub yMaxExtent: SHORT,
    pub caretSlopeRise: SHORT,
    pub caretSlopeRun: SHORT,
    pub caretOffset: SHORT,
    pub reserved: [SHORT; 4],
    pub metricDataFormat: SHORT,
    pub numOfLongVerMetrics: USHORT,
    pub numOfExSideBearings: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_maxp_table {
    pub version: Fixed,
    pub numGlyphs: USHORT,
    pub maxPoints: USHORT,
    pub maxContours: USHORT,
    pub maxComponentPoints: USHORT,
    pub maxComponentContours: USHORT,
    pub maxZones: USHORT,
    pub maxTwilightPoints: USHORT,
    pub maxStorage: USHORT,
    pub maxFunctionDefs: USHORT,
    pub maxInstructionDefs: USHORT,
    pub maxStackElements: USHORT,
    pub maxSizeOfInstructions: USHORT,
    pub maxComponentElements: USHORT,
    pub maxComponentDepth: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_os2__table {
    pub version: USHORT,
    pub xAvgCharWidth: SHORT,
    pub usWeightClass: USHORT,
    pub usWidthClass: USHORT,
    pub fsType: SHORT,
    pub ySubscriptXSize: SHORT,
    pub ySubscriptYSize: SHORT,
    pub ySubscriptXOffset: SHORT,
    pub ySubscriptYOffset: SHORT,
    pub ySuperscriptXSize: SHORT,
    pub ySuperscriptYSize: SHORT,
    pub ySuperscriptXOffset: SHORT,
    pub ySuperscriptYOffset: SHORT,
    pub yStrikeoutSize: SHORT,
    pub yStrikeoutPosition: SHORT,
    pub sFamilyClass: SHORT,
    pub panose: [BYTE; 10],
    pub ulUnicodeRange1: SFNT_ULONG,
    pub ulUnicodeRange2: SFNT_ULONG,
    pub ulUnicodeRange3: SFNT_ULONG,
    pub ulUnicodeRange4: SFNT_ULONG,
    pub achVendID: [SFNT_CHAR; 4],
    pub fsSelection: USHORT,
    pub usFirstCharIndex: USHORT,
    pub usLastCharIndex: USHORT,
    pub sTypoAscender: SHORT,
    pub sTypoDescender: SHORT,
    pub sTypoLineGap: SHORT,
    pub usWinAscent: USHORT,
    pub usWinDescent: USHORT,
    pub ulCodePageRange1: SFNT_ULONG,
    pub ulCodePageRange2: SFNT_ULONG,
    pub sxHeight: SHORT,
    pub sCapHeight: SHORT,
    pub usDefaultChar: USHORT,
    pub usBreakChar: USHORT,
    pub usMaxContext: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_vertOriginYMetrics {
    pub glyphIndex: USHORT,
    pub vertOriginY: SHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_VORG_table {
    pub defaultVertOriginY: SHORT,
    pub numVertOriginYMetrics: USHORT,
    pub vertOriginYMetrics: *mut tt_vertOriginYMetrics,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_longMetrics {
    pub advance: USHORT,
    pub sideBearing: SHORT,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_tt_pack_head_table")]
pub unsafe extern "C-unwind" fn tt_pack_head_table(
    mut table: *mut tt_head_table,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if table.is_null() {
        pdf_abort!(b"passed NULL pointer\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    data = new((54 as ::core::ffi::c_ulong as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    p = data;
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).version as SFNT_LONG,
        4 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).fontRevision as SFNT_LONG,
        4 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).checkSumAdjustment as SFNT_LONG,
        4 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).magicNumber as SFNT_LONG,
        4 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).flags as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).unitsPerEm as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = (*table).created[i as usize] as ::core::ffi::c_char;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = (*table).modified[i as usize] as ::core::ffi::c_char;
        i += 1;
    }
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).xMin as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).yMin as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).xMax as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).yMax as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).macStyle as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).lowestRecPPEM as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).fontDirectionHint as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).indexToLocFormat as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).glyphDataFormat as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    return data;
}
#[unsafe(export_name = "__pdf_impl_tt_read_head_table")]
pub unsafe extern "C-unwind" fn tt_read_head_table(mut sfont: *mut sfnt) -> *mut tt_head_table {
    let mut i: ::core::ffi::c_int = 0;
    let mut table: *mut tt_head_table = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<tt_head_table>() as usize)
        as uint32_t) as *mut tt_head_table;
    sfnt_locate_table(sfont, b"head\0" as *const u8 as *const ::core::ffi::c_char);
    (*table).version = tt_get_unsigned_quad((*sfont).handle) as Fixed;
    (*table).fontRevision = tt_get_unsigned_quad((*sfont).handle) as Fixed;
    (*table).checkSumAdjustment = tt_get_unsigned_quad((*sfont).handle);
    (*table).magicNumber = tt_get_unsigned_quad((*sfont).handle);
    (*table).flags = tt_get_unsigned_pair((*sfont).handle);
    (*table).unitsPerEm = tt_get_unsigned_pair((*sfont).handle);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        (*table).created[i as usize] = tt_get_unsigned_byte((*sfont).handle);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        (*table).modified[i as usize] = tt_get_unsigned_byte((*sfont).handle);
        i += 1;
    }
    (*table).xMin = tt_get_signed_pair((*sfont).handle) as FWord;
    (*table).yMin = tt_get_signed_pair((*sfont).handle) as FWord;
    (*table).xMax = tt_get_signed_pair((*sfont).handle) as FWord;
    (*table).yMax = tt_get_signed_pair((*sfont).handle) as FWord;
    (*table).macStyle = tt_get_signed_pair((*sfont).handle) as USHORT;
    (*table).lowestRecPPEM = tt_get_signed_pair((*sfont).handle) as USHORT;
    (*table).fontDirectionHint = tt_get_signed_pair((*sfont).handle);
    (*table).indexToLocFormat = tt_get_signed_pair((*sfont).handle);
    (*table).glyphDataFormat = tt_get_signed_pair((*sfont).handle);
    return table;
}
#[unsafe(export_name = "__pdf_impl_tt_pack_maxp_table")]
pub unsafe extern "C-unwind" fn tt_pack_maxp_table(
    mut table: *mut tt_maxp_table,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    data = new((32 as ::core::ffi::c_ulong as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    p = data;
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).version as SFNT_LONG,
        4 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).numGlyphs as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxPoints as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxContours as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxComponentPoints as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxComponentContours as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxZones as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxTwilightPoints as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxStorage as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxFunctionDefs as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxInstructionDefs as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxStackElements as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxSizeOfInstructions as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxComponentElements as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxComponentDepth as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    return data;
}
#[unsafe(export_name = "__pdf_impl_tt_read_maxp_table")]
pub unsafe extern "C-unwind" fn tt_read_maxp_table(mut sfont: *mut sfnt) -> *mut tt_maxp_table {
    let mut table: *mut tt_maxp_table = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<tt_maxp_table>() as usize)
        as uint32_t) as *mut tt_maxp_table;
    sfnt_locate_table(sfont, b"maxp\0" as *const u8 as *const ::core::ffi::c_char);
    (*table).version = tt_get_unsigned_quad((*sfont).handle) as Fixed;
    (*table).numGlyphs = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxPoints = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxContours = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxComponentPoints = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxComponentContours = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxZones = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxTwilightPoints = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxStorage = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxFunctionDefs = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxInstructionDefs = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxStackElements = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxSizeOfInstructions = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxComponentElements = tt_get_unsigned_pair((*sfont).handle);
    (*table).maxComponentDepth = tt_get_unsigned_pair((*sfont).handle);
    return table;
}
#[unsafe(export_name = "__pdf_impl_tt_pack_hhea_table")]
pub unsafe extern "C-unwind" fn tt_pack_hhea_table(
    mut table: *mut tt_hhea_table,
) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    data = new((36 as ::core::ffi::c_ulong as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    p = data;
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).version as SFNT_LONG,
        4 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).ascent as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).descent as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).lineGap as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).advanceWidthMax as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).minLeftSideBearing as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).minRightSideBearing as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).xMaxExtent as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).caretSlopeRise as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).caretSlopeRun as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).caretOffset as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        p = p.offset(put_big_endian(
            p as *mut ::core::ffi::c_void,
            (*table).reserved[i as usize] as SFNT_LONG,
            2 as ::core::ffi::c_int,
        ) as isize);
        i += 1;
    }
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).metricDataFormat as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).numOfLongHorMetrics as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    return data;
}
#[unsafe(export_name = "__pdf_impl_tt_read_hhea_table")]
pub unsafe extern "C-unwind" fn tt_read_hhea_table(mut sfont: *mut sfnt) -> *mut tt_hhea_table {
    let mut i: ::core::ffi::c_int = 0;
    let mut len: SFNT_ULONG = 0;
    let mut table: *mut tt_hhea_table = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<tt_hhea_table>() as usize)
        as uint32_t) as *mut tt_hhea_table;
    sfnt_locate_table(sfont, b"hhea\0" as *const u8 as *const ::core::ffi::c_char);
    (*table).version = tt_get_unsigned_quad((*sfont).handle) as Fixed;
    (*table).ascent = tt_get_signed_pair((*sfont).handle) as FWord;
    (*table).descent = tt_get_signed_pair((*sfont).handle) as FWord;
    (*table).lineGap = tt_get_signed_pair((*sfont).handle) as FWord;
    (*table).advanceWidthMax = tt_get_unsigned_pair((*sfont).handle) as uFWord;
    (*table).minLeftSideBearing = tt_get_signed_pair((*sfont).handle) as FWord;
    (*table).minRightSideBearing = tt_get_signed_pair((*sfont).handle) as FWord;
    (*table).xMaxExtent = tt_get_signed_pair((*sfont).handle) as FWord;
    (*table).caretSlopeRise = tt_get_signed_pair((*sfont).handle);
    (*table).caretSlopeRun = tt_get_signed_pair((*sfont).handle);
    (*table).caretOffset = tt_get_signed_pair((*sfont).handle) as FWord;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        (*table).reserved[i as usize] = tt_get_signed_pair((*sfont).handle);
        i += 1;
    }
    (*table).metricDataFormat = tt_get_signed_pair((*sfont).handle);
    if (*table).metricDataFormat as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        pdf_abort!(b"unknown metricDataFormat\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*table).numOfLongHorMetrics = tt_get_unsigned_pair((*sfont).handle);
    len = sfnt_find_table_len(sfont, b"hmtx\0" as *const u8 as *const ::core::ffi::c_char);
    (*table).numOfExSideBearings = len
        .wrapping_sub(
            ((*table).numOfLongHorMetrics as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as SFNT_ULONG,
        )
        .wrapping_div(2 as SFNT_ULONG) as USHORT;
    return table;
}
#[unsafe(export_name = "__pdf_impl_tt_read_vhea_table")]
pub unsafe extern "C-unwind" fn tt_read_vhea_table(mut sfont: *mut sfnt) -> *mut tt_vhea_table {
    let mut i: ::core::ffi::c_int = 0;
    let mut len: SFNT_ULONG = 0;
    let mut table: *mut tt_vhea_table = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<tt_vhea_table>() as usize)
        as uint32_t) as *mut tt_vhea_table;
    sfnt_locate_table(sfont, b"vhea\0" as *const u8 as *const ::core::ffi::c_char);
    (*table).version = tt_get_unsigned_quad((*sfont).handle) as Fixed;
    (*table).vertTypoAscender = tt_get_signed_pair((*sfont).handle);
    (*table).vertTypoDescender = tt_get_signed_pair((*sfont).handle);
    (*table).vertTypoLineGap = tt_get_signed_pair((*sfont).handle);
    (*table).advanceHeightMax = tt_get_signed_pair((*sfont).handle);
    (*table).minTopSideBearing = tt_get_signed_pair((*sfont).handle);
    (*table).minBottomSideBearing = tt_get_signed_pair((*sfont).handle);
    (*table).yMaxExtent = tt_get_signed_pair((*sfont).handle);
    (*table).caretSlopeRise = tt_get_signed_pair((*sfont).handle);
    (*table).caretSlopeRun = tt_get_signed_pair((*sfont).handle);
    (*table).caretOffset = tt_get_signed_pair((*sfont).handle);
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        (*table).reserved[i as usize] = tt_get_signed_pair((*sfont).handle);
        i += 1;
    }
    (*table).metricDataFormat = tt_get_signed_pair((*sfont).handle);
    (*table).numOfLongVerMetrics = tt_get_unsigned_pair((*sfont).handle);
    len = sfnt_find_table_len(sfont, b"vmtx\0" as *const u8 as *const ::core::ffi::c_char);
    (*table).numOfExSideBearings = len
        .wrapping_sub(
            ((*table).numOfLongVerMetrics as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as SFNT_ULONG,
        )
        .wrapping_div(2 as SFNT_ULONG) as USHORT;
    return table;
}
#[unsafe(export_name = "__pdf_impl_tt_read_VORG_table")]
pub unsafe extern "C-unwind" fn tt_read_VORG_table(mut sfont: *mut sfnt) -> *mut tt_VORG_table {
    let mut vorg: *mut tt_VORG_table = ::core::ptr::null_mut::<tt_VORG_table>();
    let mut offset: SFNT_ULONG = 0;
    let mut i: USHORT = 0;
    offset = sfnt_find_table_pos(sfont, b"VORG\0" as *const u8 as *const ::core::ffi::c_char);
    if offset > 0 as SFNT_ULONG {
        vorg = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<tt_VORG_table>() as usize)
            as uint32_t) as *mut tt_VORG_table;
        sfnt_locate_table(sfont, b"VORG\0" as *const u8 as *const ::core::ffi::c_char);
        if tt_get_unsigned_pair((*sfont).handle) as ::core::ffi::c_int != 1 as ::core::ffi::c_int
            || tt_get_unsigned_pair((*sfont).handle) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            pdf_abort!(b"Unsupported VORG version.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        (*vorg).defaultVertOriginY = tt_get_signed_pair((*sfont).handle);
        (*vorg).numVertOriginYMetrics = tt_get_unsigned_pair((*sfont).handle);
        (*vorg).vertOriginYMetrics = new(((*vorg).numVertOriginYMetrics as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<tt_vertOriginYMetrics>() as usize)
            as uint32_t) as *mut tt_vertOriginYMetrics;
        i = 0 as USHORT;
        while (i as ::core::ffi::c_int) < (*vorg).numVertOriginYMetrics as ::core::ffi::c_int {
            (*(*vorg).vertOriginYMetrics.offset(i as isize)).glyphIndex =
                tt_get_unsigned_pair((*sfont).handle);
            (*(*vorg).vertOriginYMetrics.offset(i as isize)).vertOriginY =
                tt_get_signed_pair((*sfont).handle);
            i = i.wrapping_add(1);
        }
    } else {
        vorg = ::core::ptr::null_mut::<tt_VORG_table>();
    }
    return vorg;
}
#[unsafe(export_name = "__pdf_impl_tt_read_longMetrics")]
pub unsafe extern "C-unwind" fn tt_read_longMetrics(
    mut sfont: *mut sfnt,
    mut numGlyphs: USHORT,
    mut numLongMetrics: USHORT,
    mut numExSideBearings: USHORT,
) -> *mut tt_longMetrics {
    let mut m: *mut tt_longMetrics = ::core::ptr::null_mut::<tt_longMetrics>();
    let mut gid: USHORT = 0;
    let mut last_adv: USHORT = 0 as USHORT;
    let mut last_esb: SHORT = 0 as SHORT;
    m = new((numGlyphs as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<tt_longMetrics>() as usize) as uint32_t)
        as *mut tt_longMetrics;
    gid = 0 as USHORT;
    while (gid as ::core::ffi::c_int) < numGlyphs as ::core::ffi::c_int {
        if (gid as ::core::ffi::c_int) < numLongMetrics as ::core::ffi::c_int {
            last_adv = tt_get_unsigned_pair((*sfont).handle);
        }
        if (gid as ::core::ffi::c_int)
            < numLongMetrics as ::core::ffi::c_int + numExSideBearings as ::core::ffi::c_int
        {
            last_esb = tt_get_signed_pair((*sfont).handle);
        }
        (*m.offset(gid as isize)).advance = last_adv;
        (*m.offset(gid as isize)).sideBearing = last_esb;
        gid = gid.wrapping_add(1);
    }
    return m;
}
#[unsafe(export_name = "__pdf_impl_tt_read_os2__table")]
pub unsafe extern "C-unwind" fn tt_read_os2__table(mut sfont: *mut sfnt) -> *mut tt_os2__table {
    let mut table: *mut tt_os2__table = ::core::ptr::null_mut::<tt_os2__table>();
    let mut i: ::core::ffi::c_int = 0;
    table = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<tt_os2__table>() as usize) as uint32_t)
        as *mut tt_os2__table;
    if sfnt_find_table_pos(sfont, b"OS/2\0" as *const u8 as *const ::core::ffi::c_char)
        > 0 as SFNT_ULONG
    {
        sfnt_locate_table(sfont, b"OS/2\0" as *const u8 as *const ::core::ffi::c_char);
        (*table).version = tt_get_unsigned_pair((*sfont).handle);
        (*table).xAvgCharWidth = tt_get_signed_pair((*sfont).handle);
        (*table).usWeightClass = tt_get_unsigned_pair((*sfont).handle);
        (*table).usWidthClass = tt_get_unsigned_pair((*sfont).handle);
        (*table).fsType = tt_get_signed_pair((*sfont).handle);
        (*table).ySubscriptXSize = tt_get_signed_pair((*sfont).handle);
        (*table).ySubscriptYSize = tt_get_signed_pair((*sfont).handle);
        (*table).ySubscriptXOffset = tt_get_signed_pair((*sfont).handle);
        (*table).ySubscriptYOffset = tt_get_signed_pair((*sfont).handle);
        (*table).ySuperscriptXSize = tt_get_signed_pair((*sfont).handle);
        (*table).ySuperscriptYSize = tt_get_signed_pair((*sfont).handle);
        (*table).ySuperscriptXOffset = tt_get_signed_pair((*sfont).handle);
        (*table).ySuperscriptYOffset = tt_get_signed_pair((*sfont).handle);
        (*table).yStrikeoutSize = tt_get_signed_pair((*sfont).handle);
        (*table).yStrikeoutPosition = tt_get_signed_pair((*sfont).handle);
        (*table).sFamilyClass = tt_get_signed_pair((*sfont).handle);
        i = 0 as ::core::ffi::c_int;
        while i < 10 as ::core::ffi::c_int {
            (*table).panose[i as usize] = tt_get_unsigned_byte((*sfont).handle);
            i += 1;
        }
        (*table).ulUnicodeRange1 = tt_get_unsigned_quad((*sfont).handle);
        (*table).ulUnicodeRange2 = tt_get_unsigned_quad((*sfont).handle);
        (*table).ulUnicodeRange3 = tt_get_unsigned_quad((*sfont).handle);
        (*table).ulUnicodeRange4 = tt_get_unsigned_quad((*sfont).handle);
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            (*table).achVendID[i as usize] = tt_get_signed_byte((*sfont).handle);
            i += 1;
        }
        (*table).fsSelection = tt_get_unsigned_pair((*sfont).handle);
        (*table).usFirstCharIndex = tt_get_unsigned_pair((*sfont).handle);
        (*table).usLastCharIndex = tt_get_unsigned_pair((*sfont).handle);
        if sfnt_find_table_len(sfont, b"OS/2\0" as *const u8 as *const ::core::ffi::c_char)
            >= 78 as SFNT_ULONG
        {
            (*table).sTypoAscender = tt_get_signed_pair((*sfont).handle);
            (*table).sTypoDescender = tt_get_signed_pair((*sfont).handle);
            (*table).sTypoLineGap = tt_get_signed_pair((*sfont).handle);
            (*table).usWinAscent = tt_get_unsigned_pair((*sfont).handle);
            (*table).usWinDescent = tt_get_unsigned_pair((*sfont).handle);
            if (*table).version as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                (*table).ulCodePageRange1 = tt_get_unsigned_quad((*sfont).handle);
                (*table).ulCodePageRange2 = tt_get_unsigned_quad((*sfont).handle);
                if (*table).version as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
                    (*table).sxHeight = tt_get_signed_pair((*sfont).handle);
                    (*table).sCapHeight = tt_get_signed_pair((*sfont).handle);
                    (*table).usDefaultChar = tt_get_unsigned_pair((*sfont).handle);
                    (*table).usBreakChar = tt_get_unsigned_pair((*sfont).handle);
                    (*table).usMaxContext = tt_get_unsigned_pair((*sfont).handle);
                }
            }
        }
    } else {
        (*table).sTypoAscender = 880 as SHORT;
        (*table).sTypoDescender = -(120 as ::core::ffi::c_int) as SHORT;
        (*table).usWeightClass = 400 as USHORT;
        (*table).xAvgCharWidth = 0 as SHORT;
        (*table).version = 0 as USHORT;
        (*table).fsType = 0 as SHORT;
        (*table).fsSelection = 0 as USHORT;
        (*table).sFamilyClass = 0 as SHORT;
        i = 0 as ::core::ffi::c_int;
        while i < 10 as ::core::ffi::c_int {
            (*table).panose[i as usize] = 0 as BYTE;
            i += 1;
        }
    }
    return table;
}
unsafe extern "C-unwind" fn tt_get_name(
    mut sfont: *mut sfnt,
    mut dest: *mut ::core::ffi::c_char,
    mut destlen: USHORT,
    mut plat_id: USHORT,
    mut enco_id: USHORT,
    mut lang_id: USHORT,
    mut name_id: USHORT,
) -> USHORT {
    let mut length: USHORT = 0 as USHORT;
    let mut num_names: USHORT = 0;
    let mut string_offset: USHORT = 0;
    let mut name_offset: SFNT_ULONG = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut is_utf16_be: ::core::ffi::c_int = 0;
    name_offset = sfnt_locate_table(sfont, b"name\0" as *const u8 as *const ::core::ffi::c_char);
    is_utf16_be = (plat_id as ::core::ffi::c_int == 3 as ::core::ffi::c_int
        && enco_id as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        && lang_id as ::core::ffi::c_uint == 0x409 as ::core::ffi::c_uint
        && name_id as ::core::ffi::c_int == 6 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    if tt_get_unsigned_pair((*sfont).handle) != 0 {
        pdf_abort!(b"Expecting zero\0" as *const u8 as *const ::core::ffi::c_char);
    }
    num_names = tt_get_unsigned_pair((*sfont).handle);
    string_offset = tt_get_unsigned_pair((*sfont).handle);
    i = 0 as ::core::ffi::c_int;
    while i < num_names as ::core::ffi::c_int {
        let mut p_id: USHORT = 0;
        let mut e_id: USHORT = 0;
        let mut n_id: USHORT = 0;
        let mut l_id: USHORT = 0;
        let mut offset: USHORT = 0;
        p_id = tt_get_unsigned_pair((*sfont).handle);
        e_id = tt_get_unsigned_pair((*sfont).handle);
        l_id = tt_get_unsigned_pair((*sfont).handle);
        n_id = tt_get_unsigned_pair((*sfont).handle);
        length = tt_get_unsigned_pair((*sfont).handle);
        offset = tt_get_unsigned_pair((*sfont).handle);
        if p_id as ::core::ffi::c_int == plat_id as ::core::ffi::c_int
            && e_id as ::core::ffi::c_int == enco_id as ::core::ffi::c_int
            && (lang_id as ::core::ffi::c_uint == 0xffff as ::core::ffi::c_uint
                || l_id as ::core::ffi::c_int == lang_id as ::core::ffi::c_int)
            && n_id as ::core::ffi::c_int == name_id as ::core::ffi::c_int
        {
            if is_utf16_be != 0 {
                length = (length as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as USHORT;
            }
            if length as ::core::ffi::c_int
                > destlen as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                pdf_warning!(
                    b"Name string too long (%u), truncating to %u\0" as *const u8
                        as *const ::core::ffi::c_char,
                    length as ::core::ffi::c_int,
                    destlen as ::core::ffi::c_int,
                );
                length = (destlen as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as USHORT;
            }
            crate::backend::runtime::input_seek(
                (*sfont).handle,
                name_offset
                    .wrapping_add(string_offset as SFNT_ULONG)
                    .wrapping_add(offset as SFNT_ULONG) as ssize_t,
                SEEK_SET,
            );
            if is_utf16_be != 0 {
                j = 0 as ::core::ffi::c_int;
                while j < length as ::core::ffi::c_int {
                    *dest.offset(j as isize) =
                        (tt_get_unsigned_pair((*sfont).handle) as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar as ::core::ffi::c_char;
                    j += 1;
                }
            } else {
                ttstub_input_read(
                    (*sfont).handle,
                    dest as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
                    length as size_t,
                );
            }
            *dest.offset(length as isize) = '\0' as i32 as ::core::ffi::c_char;
            break;
        } else {
            i += 1;
        }
    }
    if i == num_names as ::core::ffi::c_int {
        length = 0 as USHORT;
    }
    return length;
}
#[unsafe(export_name = "__pdf_impl_tt_get_ps_fontname")]
pub unsafe extern "C-unwind" fn tt_get_ps_fontname(
    mut sfont: *mut sfnt,
    mut dest: *mut ::core::ffi::c_char,
    mut destlen: USHORT,
) -> USHORT {
    let mut namelen: USHORT = 0 as USHORT;
    namelen = tt_get_name(
        sfont,
        dest,
        destlen,
        1 as USHORT,
        0 as USHORT,
        0 as USHORT,
        6 as USHORT,
    );
    if namelen as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        || {
            namelen = tt_get_name(
                sfont,
                dest,
                destlen,
                3 as USHORT,
                1 as USHORT,
                0x409 as USHORT,
                6 as USHORT,
            );
            namelen as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        }
        || {
            namelen = tt_get_name(
                sfont,
                dest,
                destlen,
                3 as USHORT,
                5 as USHORT,
                0x412 as USHORT,
                6 as USHORT,
            );
            namelen as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        }
    {
        return namelen;
    }
    pdf_warning!(
        b"No valid PostScript name available\0" as *const u8 as *const ::core::ffi::c_char,
    );
    namelen = tt_get_name(
        sfont,
        dest,
        destlen,
        1 as USHORT,
        0 as USHORT,
        0xffff as USHORT,
        6 as USHORT,
    );
    if namelen as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        namelen = tt_get_name(
            sfont,
            dest,
            destlen,
            1 as USHORT,
            0 as USHORT,
            0 as USHORT,
            1 as USHORT,
        );
    }
    return namelen;
}
unsafe extern "C" {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_pack_head_table(
    mut table: *mut tt_head_table,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_pack_head_table(table) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_read_head_table(
    mut sfont: *mut sfnt,
    _result: *mut *mut tt_head_table,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_read_head_table(sfont) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_pack_maxp_table(
    mut table: *mut tt_maxp_table,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_pack_maxp_table(table) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_read_maxp_table(
    mut sfont: *mut sfnt,
    _result: *mut *mut tt_maxp_table,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_read_maxp_table(sfont) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_pack_hhea_table(
    mut table: *mut tt_hhea_table,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_pack_hhea_table(table) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_read_hhea_table(
    mut sfont: *mut sfnt,
    _result: *mut *mut tt_hhea_table,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_read_hhea_table(sfont) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_read_vhea_table(
    mut sfont: *mut sfnt,
    _result: *mut *mut tt_vhea_table,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_read_vhea_table(sfont) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_read_VORG_table(
    mut sfont: *mut sfnt,
    _result: *mut *mut tt_VORG_table,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_read_VORG_table(sfont) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_read_longMetrics(
    mut sfont: *mut sfnt,
    mut numGlyphs: USHORT,
    mut numLongMetrics: USHORT,
    mut numExSideBearings: USHORT,
    _result: *mut *mut tt_longMetrics,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        tt_read_longMetrics(sfont, numGlyphs, numLongMetrics, numExSideBearings)
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_read_os2__table(
    mut sfont: *mut sfnt,
    _result: *mut *mut tt_os2__table,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_read_os2__table(sfont) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_get_ps_fontname(
    mut sfont: *mut sfnt,
    mut dest: *mut ::core::ffi::c_char,
    mut destlen: USHORT,
    _result: *mut USHORT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_ps_fontname(sfont, dest, destlen) })
    {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
