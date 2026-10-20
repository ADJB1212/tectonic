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
#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn, static_mut_refs, dangerous_implicit_autorefs)]
unsafe extern "C-unwind" {
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut ::core::ffi::c_char, len: size_t) -> ssize_t;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_tt_get_signed_pair"]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> ::core::ffi::c_short;
    #[link_name = "__pdf_impl_put_big_endian"]
    fn put_big_endian(s: *mut ::core::ffi::c_void, q: SFNT_LONG, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone, Default)]
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
unsafe fn read_sfnt_table(sfont: *mut sfnt, tag: &std::ffi::CStr) -> Vec<u8> {
    let len = sfnt_find_table_len(sfont, tag.as_ptr()) as usize;
    sfnt_locate_table(sfont, tag.as_ptr());
    let mut data = vec![0; len];
    if ttstub_input_read((*sfont).handle, data.as_mut_ptr().cast(), len) != len as isize {
        pdf_abort!("File ended prematurely\n");
    }
    data
}

fn parse_table<'a, T: read_fonts::FontRead<'a, Args = ()>>(data: &'a [u8], tag: &str) -> T {
    T::read(data.into()).unwrap_or_else(|_| pdf_abort!("Invalid TrueType {tag} table"))
}

fn alloc_table<T: Copy>(value: T) -> *mut T {
    let size = u32::try_from(core::mem::size_of::<T>()).expect("font table fits in u32");
    unsafe {
        let result = new(size).cast::<T>();
        result.write(value);
        result
    }
}

#[unsafe(export_name = "__pdf_impl_tt_pack_head_table")]
pub unsafe extern "C-unwind" fn tt_pack_head_table(mut table: *mut tt_head_table) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if table.is_null() {
        pdf_abort!(b"passed NULL pointer\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    data = new((54 as ::core::ffi::c_ulong as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
        as *mut ::core::ffi::c_char;
    p = data;
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).version as SFNT_LONG, 4 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).fontRevision as SFNT_LONG, 4 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).checkSumAdjustment as SFNT_LONG,
        4 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).magicNumber as SFNT_LONG, 4 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).flags as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).unitsPerEm as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
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
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).xMin as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).yMin as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).xMax as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).yMax as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).macStyle as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
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
pub unsafe extern "C-unwind" fn tt_read_head_table(sfont: *mut sfnt) -> *mut tt_head_table {
    let data = read_sfnt_table(sfont, c"head");
    let header: read_fonts::tables::head::Head<'_> = parse_table(&data, "head");
    alloc_table(tt_head_table {
        version: u32::from_be_bytes(header.version().to_be_bytes()),
        fontRevision: header.font_revision().to_bits() as u32,
        checkSumAdjustment: header.checksum_adjustment(),
        magicNumber: header.magic_number(),
        flags: header.offset_data().read_at(header.flags_byte_range().start).unwrap(),
        unitsPerEm: header.units_per_em(),
        created: header.created().to_be_bytes(),
        modified: header.modified().to_be_bytes(),
        xMin: header.x_min(),
        yMin: header.y_min(),
        xMax: header.x_max(),
        yMax: header.y_max(),
        macStyle: header.offset_data().read_at(header.mac_style_byte_range().start).unwrap(),
        lowestRecPPEM: header.lowest_rec_ppem(),
        fontDirectionHint: header.font_direction_hint(),
        indexToLocFormat: header.index_to_loc_format(),
        glyphDataFormat: header.glyph_data_format(),
    })
}
#[unsafe(export_name = "__pdf_impl_tt_pack_maxp_table")]
pub unsafe extern "C-unwind" fn tt_pack_maxp_table(mut table: *mut tt_maxp_table) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    data = new((32 as ::core::ffi::c_ulong as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
        as *mut ::core::ffi::c_char;
    p = data;
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).version as SFNT_LONG, 4 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).numGlyphs as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).maxPoints as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).maxContours as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
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
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).maxZones as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*table).maxTwilightPoints as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).maxStorage as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
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
pub unsafe extern "C-unwind" fn tt_read_maxp_table(sfont: *mut sfnt) -> *mut tt_maxp_table {
    let data = read_sfnt_table(sfont, c"maxp");
    let header: read_fonts::tables::maxp::Maxp<'_> = parse_table(&data, "maxp");
    let version = u32::from_be_bytes(header.version().to_be_bytes());
    if version == 0x10000 && data.len() < 32 {
        pdf_abort!("Invalid TrueType maxp table");
    }
    alloc_table(tt_maxp_table {
        version,
        numGlyphs: header.num_glyphs(),
        maxPoints: header.max_points().unwrap_or(0),
        maxContours: header.max_contours().unwrap_or(0),
        maxComponentPoints: header.max_composite_points().unwrap_or(0),
        maxComponentContours: header.max_composite_contours().unwrap_or(0),
        maxZones: header.max_zones().unwrap_or(0),
        maxTwilightPoints: header.max_twilight_points().unwrap_or(0),
        maxStorage: header.max_storage().unwrap_or(0),
        maxFunctionDefs: header.max_function_defs().unwrap_or(0),
        maxInstructionDefs: header.max_instruction_defs().unwrap_or(0),
        maxStackElements: header.max_stack_elements().unwrap_or(0),
        maxSizeOfInstructions: header.max_size_of_instructions().unwrap_or(0),
        maxComponentElements: header.max_component_elements().unwrap_or(0),
        maxComponentDepth: header.max_component_depth().unwrap_or(0),
    })
}
#[unsafe(export_name = "__pdf_impl_tt_pack_hhea_table")]
pub unsafe extern "C-unwind" fn tt_pack_hhea_table(mut table: *mut tt_hhea_table) -> *mut ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    data = new((36 as ::core::ffi::c_ulong as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
        as *mut ::core::ffi::c_char;
    p = data;
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).version as SFNT_LONG, 4 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).ascent as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).descent as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).lineGap as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
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
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).xMaxExtent as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
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
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*table).caretOffset as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
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
pub unsafe extern "C-unwind" fn tt_read_hhea_table(sfont: *mut sfnt) -> *mut tt_hhea_table {
    let data = read_sfnt_table(sfont, c"hhea");
    let header: read_fonts::tables::hhea::Hhea<'_> = parse_table(&data, "hhea");
    if header.metric_data_format() != 0 {
        pdf_abort!("unknown metricDataFormat");
    }
    let num_metrics = header.number_of_h_metrics();
    let metrics_len = sfnt_find_table_len(sfont, c"hmtx".as_ptr());
    alloc_table(tt_hhea_table {
        version: u32::from_be_bytes(header.version().to_be_bytes()),
        ascent: header.ascender().to_i16(),
        descent: header.descender().to_i16(),
        lineGap: header.line_gap().to_i16(),
        advanceWidthMax: header.advance_width_max().to_u16(),
        minLeftSideBearing: header.min_left_side_bearing().to_i16(),
        minRightSideBearing: header.min_right_side_bearing().to_i16(),
        xMaxExtent: header.x_max_extent().to_i16(),
        caretSlopeRise: header.caret_slope_rise(),
        caretSlopeRun: header.caret_slope_run(),
        caretOffset: header.caret_offset(),
        reserved: core::array::from_fn(|i| header.offset_data().read_at(header.reserved1_byte_range().start + i * 2).unwrap()),
        metricDataFormat: header.metric_data_format(),
        numOfLongHorMetrics: num_metrics,
        numOfExSideBearings: metrics_len.wrapping_sub(u32::from(num_metrics) * 4).wrapping_div(2) as u16,
    })
}
#[unsafe(export_name = "__pdf_impl_tt_read_vhea_table")]
pub unsafe extern "C-unwind" fn tt_read_vhea_table(sfont: *mut sfnt) -> *mut tt_vhea_table {
    let data = read_sfnt_table(sfont, c"vhea");
    let header: read_fonts::tables::vhea::Vhea<'_> = parse_table(&data, "vhea");
    let num_metrics = header.number_of_long_ver_metrics();
    let metrics_len = sfnt_find_table_len(sfont, c"vmtx".as_ptr());
    alloc_table(tt_vhea_table {
        version: u32::from_be_bytes(header.version().to_be_bytes()),
        vertTypoAscender: header.ascender().to_i16(),
        vertTypoDescender: header.descender().to_i16(),
        vertTypoLineGap: header.line_gap().to_i16(),
        advanceHeightMax: header.advance_height_max().to_u16() as i16,
        minTopSideBearing: header.min_top_side_bearing().to_i16(),
        minBottomSideBearing: header.min_bottom_side_bearing().to_i16(),
        yMaxExtent: header.y_max_extent().to_i16(),
        caretSlopeRise: header.caret_slope_rise(),
        caretSlopeRun: header.caret_slope_run(),
        caretOffset: header.caret_offset(),
        reserved: core::array::from_fn(|i| header.offset_data().read_at(header.reserved1_byte_range().start + i * 2).unwrap()),
        metricDataFormat: header.metric_data_format(),
        numOfLongVerMetrics: num_metrics,
        numOfExSideBearings: metrics_len.wrapping_sub(u32::from(num_metrics) * 4).wrapping_div(2) as u16,
    })
}
#[unsafe(export_name = "__pdf_impl_tt_read_VORG_table")]
pub unsafe extern "C-unwind" fn tt_read_VORG_table(sfont: *mut sfnt) -> *mut tt_VORG_table {
    if sfnt_find_table_pos(sfont, c"VORG".as_ptr()) == 0 {
        return core::ptr::null_mut();
    }
    let data = read_sfnt_table(sfont, c"VORG");
    let header: read_fonts::tables::vorg::Vorg<'_> = parse_table(&data, "VORG");
    if header.version().to_be_bytes() != [0, 1, 0, 0] {
        pdf_abort!("Unsupported VORG version.");
    }
    let records = header.vert_origin_y_metrics();
    if records.len() != usize::from(header.num_vert_origin_y_metrics()) {
        pdf_abort!("Invalid TrueType VORG table");
    }
    let metrics = new((records.len() * core::mem::size_of::<tt_vertOriginYMetrics>()) as u32).cast::<tt_vertOriginYMetrics>();
    for (i, record) in records.iter().enumerate() {
        metrics.add(i).write(tt_vertOriginYMetrics {
            glyphIndex: record.glyph_index().to_u16(),
            vertOriginY: record.vert_origin_y(),
        });
    }
    alloc_table(tt_VORG_table {
        defaultVertOriginY: header.default_vert_origin_y(),
        numVertOriginYMetrics: header.num_vert_origin_y_metrics(),
        vertOriginYMetrics: metrics,
    })
}
#[unsafe(export_name = "__pdf_impl_tt_read_longMetrics")]
pub unsafe extern "C-unwind" fn tt_read_longMetrics(
    mut sfont: *mut sfnt, mut numGlyphs: USHORT, mut numLongMetrics: USHORT, mut numExSideBearings: USHORT,
) -> *mut tt_longMetrics {
    let mut m: *mut tt_longMetrics = ::core::ptr::null_mut::<tt_longMetrics>();
    let mut gid: USHORT = 0;
    let mut last_adv: USHORT = 0 as USHORT;
    let mut last_esb: SHORT = 0 as SHORT;
    m = new((numGlyphs as uint32_t as usize).wrapping_mul(::core::mem::size_of::<tt_longMetrics>() as usize) as uint32_t) as *mut tt_longMetrics;
    gid = 0 as USHORT;
    while (gid as ::core::ffi::c_int) < numGlyphs as ::core::ffi::c_int {
        if (gid as ::core::ffi::c_int) < numLongMetrics as ::core::ffi::c_int {
            last_adv = tt_get_unsigned_pair((*sfont).handle);
        }
        if (gid as ::core::ffi::c_int) < numLongMetrics as ::core::ffi::c_int + numExSideBearings as ::core::ffi::c_int {
            last_esb = tt_get_signed_pair((*sfont).handle);
        }
        (*m.offset(gid as isize)).advance = last_adv;
        (*m.offset(gid as isize)).sideBearing = last_esb;
        gid = gid.wrapping_add(1);
    }
    return m;
}
#[unsafe(export_name = "__pdf_impl_tt_read_os2__table")]
pub unsafe extern "C-unwind" fn tt_read_os2__table(sfont: *mut sfnt) -> *mut tt_os2__table {
    if sfnt_find_table_pos(sfont, c"OS/2".as_ptr()) == 0 {
        return alloc_table(tt_os2__table {
            sTypoAscender: 880,
            sTypoDescender: -120,
            usWeightClass: 400,
            ..Default::default()
        });
    }
    let mut data = read_sfnt_table(sfont, c"OS/2");
    if data.len() < 68 {
        pdf_abort!("Invalid TrueType OS/2 table");
    }
    // Early Mac fonts omit the final ten bytes of the version 0 table.
    data.resize(data.len().max(78), 0);
    let header: read_fonts::tables::os2::Os2<'_> = parse_table(&data, "OS/2");
    alloc_table(tt_os2__table {
        version: header.version(),
        xAvgCharWidth: header.x_avg_char_width(),
        usWeightClass: header.us_weight_class(),
        usWidthClass: header.us_width_class(),
        fsType: header.fs_type() as i16,
        ySubscriptXSize: header.y_subscript_x_size(),
        ySubscriptYSize: header.y_subscript_y_size(),
        ySubscriptXOffset: header.y_subscript_x_offset(),
        ySubscriptYOffset: header.y_subscript_y_offset(),
        ySuperscriptXSize: header.y_superscript_x_size(),
        ySuperscriptYSize: header.y_superscript_y_size(),
        ySuperscriptXOffset: header.y_superscript_x_offset(),
        ySuperscriptYOffset: header.y_superscript_y_offset(),
        yStrikeoutSize: header.y_strikeout_size(),
        yStrikeoutPosition: header.y_strikeout_position(),
        sFamilyClass: header.s_family_class(),
        panose: header.panose_10().try_into().unwrap(),
        ulUnicodeRange1: header.ul_unicode_range_1(),
        ulUnicodeRange2: header.ul_unicode_range_2(),
        ulUnicodeRange3: header.ul_unicode_range_3(),
        ulUnicodeRange4: header.ul_unicode_range_4(),
        achVendID: header.ach_vend_id().to_be_bytes().map(|byte| byte as i8),
        fsSelection: header.offset_data().read_at(header.fs_selection_byte_range().start).unwrap(),
        usFirstCharIndex: header.us_first_char_index(),
        usLastCharIndex: header.us_last_char_index(),
        sTypoAscender: header.s_typo_ascender(),
        sTypoDescender: header.s_typo_descender(),
        sTypoLineGap: header.s_typo_line_gap(),
        usWinAscent: header.us_win_ascent(),
        usWinDescent: header.us_win_descent(),
        ulCodePageRange1: header.ul_code_page_range_1().unwrap_or(0),
        ulCodePageRange2: header.ul_code_page_range_2().unwrap_or(0),
        sxHeight: header.sx_height().unwrap_or(0),
        sCapHeight: header.s_cap_height().unwrap_or(0),
        usDefaultChar: header.us_default_char().unwrap_or(0),
        usBreakChar: header.us_break_char().unwrap_or(0),
        usMaxContext: header.us_max_context().unwrap_or(0),
    })
}
fn tt_get_name(table: &read_fonts::tables::name::Name<'_>, output: &mut [u8], ids: (u16, u16, u16, u16)) -> usize {
    use read_fonts::Offset;
    let (platform, encoding, language, name_id) = ids;
    let Some(record) = table.name_record().iter().find(|record| {
        record.platform_id() == platform
            && record.encoding_id() == encoding
            && (language == 0xffff || record.language_id() == language)
            && record.name_id().to_u16() == name_id
    }) else {
        return 0;
    };
    let start = record.string_offset().non_null().unwrap_or(0);
    let end = start + record.length() as usize;
    let data = table.string_data();
    let bytes = data.as_bytes().get(start..end).unwrap_or_else(|| {
        pdf_abort!("Invalid TrueType name table");
    });
    let utf16 = ids == (3, 1, 0x409, 6);
    let length = if utf16 { bytes.len() / 2 } else { bytes.len() };
    let copied = length.min(output.len() - 1);
    if copied < length {
        pdf_warning!("Name string too long ({length}), truncating to {}", output.len());
    }
    if utf16 {
        for (output, pair) in output.iter_mut().zip(bytes.as_chunks::<2>().0).take(copied) {
            *output = pair[1];
        }
    } else {
        output[..copied].copy_from_slice(&bytes[..copied]);
    }
    output[copied] = 0;
    copied
}

#[unsafe(export_name = "__pdf_impl_tt_get_ps_fontname")]
pub unsafe extern "C-unwind" fn tt_get_ps_fontname(sfont: *mut sfnt, dest: *mut ::core::ffi::c_char, destlen: USHORT) -> USHORT {
    use read_fonts::{FontRead, tables::name::Name};
    if destlen == 0 {
        return 0;
    }
    let data = read_sfnt_table(sfont, c"name");
    let table: Name<'_> = parse_table(&data, "name");
    let mut output = vec![0; destlen as usize];
    for ids in [(1, 0, 0, 6), (3, 1, 0x409, 6), (3, 5, 0x412, 6)] {
        let length = tt_get_name(&table, &mut output, ids);
        if length != 0 {
            unsafe { core::ptr::copy_nonoverlapping(output.as_ptr().cast(), dest, length + 1) };
            return length as u16;
        }
    }
    pdf_warning!("No valid PostScript name available");
    let mut length = tt_get_name(&table, &mut output, (1, 0, 0xffff, 6));
    if length == 0 {
        length = tt_get_name(&table, &mut output, (1, 0, 0, 1));
    }
    unsafe { core::ptr::copy_nonoverlapping(output.as_ptr().cast(), dest, length + 1) };
    length as u16
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_pack_head_table(mut table: *mut tt_head_table, _result: *mut *mut ::core::ffi::c_char) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_read_head_table(mut sfont: *mut sfnt, _result: *mut *mut tt_head_table) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_pack_maxp_table(mut table: *mut tt_maxp_table, _result: *mut *mut ::core::ffi::c_char) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_read_maxp_table(mut sfont: *mut sfnt, _result: *mut *mut tt_maxp_table) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_pack_hhea_table(mut table: *mut tt_hhea_table, _result: *mut *mut ::core::ffi::c_char) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_read_hhea_table(mut sfont: *mut sfnt, _result: *mut *mut tt_hhea_table) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_read_vhea_table(mut sfont: *mut sfnt, _result: *mut *mut tt_vhea_table) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_read_VORG_table(mut sfont: *mut sfnt, _result: *mut *mut tt_VORG_table) -> i32 {
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
    mut sfont: *mut sfnt, mut numGlyphs: USHORT, mut numLongMetrics: USHORT, mut numExSideBearings: USHORT, _result: *mut *mut tt_longMetrics,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_read_longMetrics(sfont, numGlyphs, numLongMetrics, numExSideBearings) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_read_os2__table(mut sfont: *mut sfnt, _result: *mut *mut tt_os2__table) -> i32 {
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
    mut sfont: *mut sfnt, mut dest: *mut ::core::ffi::c_char, mut destlen: USHORT, _result: *mut USHORT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_ps_fontname(sfont, dest, destlen) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
