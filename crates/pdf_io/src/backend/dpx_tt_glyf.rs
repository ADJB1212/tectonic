/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
use crate::compat::allocation::free;
use std::collections::HashMap;

#[derive(Default)]
struct GlyphIndex {
    original: HashMap<u16, u16>,
    subset: HashMap<u16, u16>,
    first_empty: u16,
}

#[derive(Default)]
pub(crate) struct GlyphCaches(HashMap<usize, GlyphIndex>);

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn glyph_indices_survive_sparse_slots_duplicates_growth_and_sorting() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        unsafe {
            let glyphs = tt_build_init();
            for original in 1..8192u16 {
                let subset = 16384 - original;
                tt_add_glyph(glyphs, original, subset);
                assert_eq!(tt_find_glyph(glyphs, original), subset);
                assert_eq!(tt_get_index(glyphs, subset), original);
            }
            assert_eq!(find_empty_slot(glyphs), 1);
            tt_add_glyph(glyphs, 1, 1);
            assert_eq!(tt_find_glyph(glyphs, 1), 16383);
            assert_eq!(find_empty_slot(glyphs), 2);
            core::slice::from_raw_parts_mut((*glyphs).gd, (*glyphs).num_glyphs as usize)
                .sort_unstable_by_key(|glyph| glyph.gid);
            index_glyphs(glyphs);
            assert_eq!(tt_find_glyph(glyphs, 1), 1);
            for (slot, glyph) in
                core::slice::from_raw_parts((*glyphs).gd, (*glyphs).num_glyphs as usize)
                    .iter()
                    .enumerate()
            {
                assert_eq!(tt_get_index(glyphs, glyph.gid), slot as u16);
            }
            assert_eq!(tt_get_index(glyphs, 65535), 0);
            assert_eq!(tt_find_glyph(glyphs, 65535), 0);
            tt_build_finish(glyphs);
            assert!((*glyph_caches()).0.is_empty());
            let next = tt_build_init();
            assert_eq!(tt_find_glyph(next, 1), 0);
            tt_build_finish(next);
        }
    }
}

unsafe fn glyph_caches() -> *mut GlyphCaches {
    core::ptr::addr_of_mut!((*crate::compat::document::current()).glyphs)
}

unsafe fn index_glyphs(g: *mut tt_glyphs) {
    if let Some(index) = (*glyph_caches()).0.get_mut(&(g as usize)) {
        index.original.clear();
        index.subset.clear();
        for (slot, glyph) in core::slice::from_raw_parts((*g).gd, (*g).num_glyphs as usize)
            .iter()
            .enumerate()
        {
            index.original.entry(glyph.ogid).or_insert(glyph.gid);
            index.subset.insert(glyph.gid, slot as u16);
        }
    }
}
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
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
    #[link_name = "__pdf_impl_sfnt_find_table_pos"]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_sfnt_locate_table"]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_sfnt_set_table"]
    fn sfnt_set_table(
        sfont: *mut sfnt,
        tag: *const ::core::ffi::c_char,
        data: *mut ::core::ffi::c_void,
        length: SFNT_ULONG,
    );
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_tt_pack_head_table"]
    fn tt_pack_head_table(table: *mut tt_head_table) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_tt_read_head_table"]
    fn tt_read_head_table(sfont: *mut sfnt) -> *mut tt_head_table;
    #[link_name = "__pdf_impl_tt_pack_hhea_table"]
    fn tt_pack_hhea_table(table: *mut tt_hhea_table) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_tt_read_hhea_table"]
    fn tt_read_hhea_table(sfont: *mut sfnt) -> *mut tt_hhea_table;
    #[link_name = "__pdf_impl_tt_pack_maxp_table"]
    fn tt_pack_maxp_table(table: *mut tt_maxp_table) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_tt_read_maxp_table"]
    fn tt_read_maxp_table(sfont: *mut sfnt) -> *mut tt_maxp_table;
    #[link_name = "__pdf_impl_tt_read_vhea_table"]
    fn tt_read_vhea_table(sfont: *mut sfnt) -> *mut tt_vhea_table;
    #[link_name = "__pdf_impl_tt_read_longMetrics"]
    fn tt_read_longMetrics(
        sfont: *mut sfnt,
        numGlyphs: USHORT,
        numLongMetrics: USHORT,
        numExSideBearings: USHORT,
    ) -> *mut tt_longMetrics;
    #[link_name = "__pdf_impl_tt_read_os2__table"]
    fn tt_read_os2__table(sfont: *mut sfnt) -> *mut tt_os2__table;
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
pub struct tt_glyph_desc {
    pub gid: USHORT,
    pub ogid: USHORT,
    pub advw: USHORT,
    pub advh: USHORT,
    pub lsb: SHORT,
    pub tsb: SHORT,
    pub llx: SHORT,
    pub lly: SHORT,
    pub urx: SHORT,
    pub ury: SHORT,
    pub length: SFNT_ULONG,
    pub data: *mut BYTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_glyphs {
    pub num_glyphs: USHORT,
    pub max_glyphs: USHORT,
    pub last_gid: USHORT,
    pub emsize: USHORT,
    pub dw: USHORT,
    pub default_advh: USHORT,
    pub default_tsb: SHORT,
    pub gd: *mut tt_glyph_desc,
    pub used_slot: *mut ::core::ffi::c_uchar,
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
pub struct tt_longMetrics {
    pub advance: USHORT,
    pub sideBearing: SHORT,
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
pub const SFNT_TYPE_TRUETYPE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const SFNT_TYPE_TTC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const SFNT_TYPE_DFONT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const TT_HEAD_TABLE_SIZE: ::core::ffi::c_ulong = 54 as ::core::ffi::c_ulong;
pub const TT_MAXP_TABLE_SIZE: ::core::ffi::c_ulong = 32 as ::core::ffi::c_ulong;
pub const TT_HHEA_TABLE_SIZE: ::core::ffi::c_ulong = 36 as ::core::ffi::c_ulong;
pub const NUM_GLYPH_LIMIT: ::core::ffi::c_int = 65534 as ::core::ffi::c_int;
pub const GLYPH_ARRAY_ALLOC_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn find_empty_slot(mut g: *mut tt_glyphs) -> USHORT {
    let mut gid: USHORT = 0;
    if g.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"g\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    gid = (*glyph_caches())
        .0
        .get(&(g as usize))
        .map_or(0, |index| index.first_empty);
    while (gid as ::core::ffi::c_int) < NUM_GLYPH_LIMIT {
        if *(*g)
            .used_slot
            .offset((gid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << 7 as ::core::ffi::c_int - gid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
            == 0
        {
            break;
        }
        gid = gid.wrapping_add(1);
    }
    if gid as ::core::ffi::c_int == NUM_GLYPH_LIMIT {
        pdf_abort!(b"No empty glyph slot available.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    if let Some(index) = (*glyph_caches()).0.get_mut(&(g as usize)) {
        index.first_empty = gid;
    }
    return gid;
}
#[unsafe(export_name = "__pdf_impl_tt_find_glyph")]
pub unsafe extern "C-unwind" fn tt_find_glyph(mut g: *mut tt_glyphs, mut gid: USHORT) -> USHORT {
    if let Some(index) = (*glyph_caches()).0.get(&(g as usize)) {
        return index.original.get(&gid).copied().unwrap_or(0);
    }
    let mut idx: USHORT = 0;
    let mut new_gid: USHORT = 0 as USHORT;
    if g.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"g\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    idx = 0 as USHORT;
    while (idx as ::core::ffi::c_int) < (*g).num_glyphs as ::core::ffi::c_int {
        if gid as ::core::ffi::c_int == (*(*g).gd.offset(idx as isize)).ogid as ::core::ffi::c_int {
            new_gid = (*(*g).gd.offset(idx as isize)).gid;
            break;
        } else {
            idx = idx.wrapping_add(1);
        }
    }
    return new_gid;
}
#[unsafe(export_name = "__pdf_impl_tt_get_index")]
pub unsafe extern "C-unwind" fn tt_get_index(mut g: *mut tt_glyphs, mut gid: USHORT) -> USHORT {
    if let Some(index) = (*glyph_caches()).0.get(&(g as usize)) {
        return index.subset.get(&gid).copied().unwrap_or(0);
    }
    let mut idx: USHORT = 0;
    if g.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"g\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    idx = 0 as USHORT;
    while (idx as ::core::ffi::c_int) < (*g).num_glyphs as ::core::ffi::c_int {
        if gid as ::core::ffi::c_int == (*(*g).gd.offset(idx as isize)).gid as ::core::ffi::c_int {
            break;
        }
        idx = idx.wrapping_add(1);
    }
    if idx as ::core::ffi::c_int == (*g).num_glyphs as ::core::ffi::c_int {
        idx = 0 as USHORT;
    }
    return idx;
}
#[unsafe(export_name = "__pdf_impl_tt_add_glyph")]
pub unsafe extern "C-unwind" fn tt_add_glyph(
    mut g: *mut tt_glyphs,
    mut gid: USHORT,
    mut new_gid: USHORT,
) -> USHORT {
    if g.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"g\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if *(*g)
        .used_slot
        .offset((new_gid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int)
            << 7 as ::core::ffi::c_int - new_gid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
        != 0
    {
        pdf_warning!(
            b"Slot %u already used.\0" as *const u8 as *const ::core::ffi::c_char,
            new_gid as ::core::ffi::c_int,
        );
    } else {
        if (*g).num_glyphs as ::core::ffi::c_int + 1 as ::core::ffi::c_int >= NUM_GLYPH_LIMIT {
            pdf_abort!(b"Too many glyphs.\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if (*g).num_glyphs as ::core::ffi::c_int >= (*g).max_glyphs as ::core::ffi::c_int {
            (*g).max_glyphs = ((*g).max_glyphs as usize * 2)
                .max(GLYPH_ARRAY_ALLOC_SIZE as usize)
                .min(NUM_GLYPH_LIMIT as usize) as u16;
            (*g).gd = renew(
                (*g).gd as *mut ::core::ffi::c_void,
                ((*g).max_glyphs as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<tt_glyph_desc>() as usize)
                    as uint32_t,
            ) as *mut tt_glyph_desc;
        }
        (*(*g).gd.offset((*g).num_glyphs as isize)).gid = new_gid;
        (*(*g).gd.offset((*g).num_glyphs as isize)).ogid = gid;
        (*(*g).gd.offset((*g).num_glyphs as isize)).length = 0 as SFNT_ULONG;
        let ref mut fresh0 = (*(*g).gd.offset((*g).num_glyphs as isize)).data;
        *fresh0 = ::core::ptr::null_mut::<BYTE>();
        let ref mut fresh1 = *(*g)
            .used_slot
            .offset((new_gid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
        *fresh1 = (*fresh1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int)
                << 7 as ::core::ffi::c_int
                    - new_gid as ::core::ffi::c_int % 8 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        if let Some(index) = (*glyph_caches()).0.get_mut(&(g as usize)) {
            index.original.entry(gid).or_insert(new_gid);
            index.subset.insert(new_gid, (*g).num_glyphs);
        }
        (*g).num_glyphs =
            ((*g).num_glyphs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as USHORT;
    }
    if new_gid as ::core::ffi::c_int > (*g).last_gid as ::core::ffi::c_int {
        (*g).last_gid = new_gid;
    }
    return new_gid;
}
#[unsafe(export_name = "__pdf_impl_tt_build_init")]
pub unsafe extern "C-unwind" fn tt_build_init() -> *mut tt_glyphs {
    let mut g: *mut tt_glyphs = ::core::ptr::null_mut::<tt_glyphs>();
    g = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<tt_glyphs>() as usize) as uint32_t)
        as *mut tt_glyphs;
    (*g).num_glyphs = 0 as USHORT;
    (*g).max_glyphs = 0 as USHORT;
    (*g).last_gid = 0 as USHORT;
    (*g).emsize = 1 as USHORT;
    (*g).default_advh = 0 as USHORT;
    (*g).default_tsb = 0 as SHORT;
    (*g).gd = ::core::ptr::null_mut::<tt_glyph_desc>();
    (*g).used_slot = new((8192 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    memset(
        (*g).used_slot as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        8192 as size_t,
    );
    (*glyph_caches())
        .0
        .insert(g as usize, GlyphIndex::default());
    tt_add_glyph(g, 0 as USHORT, 0 as USHORT);
    return g;
}
#[unsafe(export_name = "__pdf_impl_tt_build_finish")]
pub unsafe extern "C-unwind" fn tt_build_finish(mut g: *mut tt_glyphs) {
    if !g.is_null() {
        (*glyph_caches()).0.remove(&(g as usize));
        if !(*g).gd.is_null() {
            let mut idx: USHORT = 0;
            idx = 0 as USHORT;
            while (idx as ::core::ffi::c_int) < (*g).num_glyphs as ::core::ffi::c_int {
                free((*(*g).gd.offset(idx as isize)).data as *mut ::core::ffi::c_void);
                idx = idx.wrapping_add(1);
            }
            free((*g).gd as *mut ::core::ffi::c_void);
        }
        free((*g).used_slot as *mut ::core::ffi::c_void);
        free(g as *mut ::core::ffi::c_void);
    }
}
#[inline]
unsafe extern "C-unwind" fn glyf_cmp(
    mut v1: *const ::core::ffi::c_void,
    mut v2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut cmp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sv1: *const tt_glyph_desc = ::core::ptr::null::<tt_glyph_desc>();
    let mut sv2: *const tt_glyph_desc = ::core::ptr::null::<tt_glyph_desc>();
    sv1 = v1 as *const tt_glyph_desc;
    sv2 = v2 as *const tt_glyph_desc;
    if (*sv1).gid as ::core::ffi::c_int == (*sv2).gid as ::core::ffi::c_int {
        cmp = 0 as ::core::ffi::c_int;
    } else if ((*sv1).gid as ::core::ffi::c_int) < (*sv2).gid as ::core::ffi::c_int {
        cmp = -(1 as ::core::ffi::c_int);
    } else {
        cmp = 1 as ::core::ffi::c_int;
    }
    return cmp;
}
pub const ARG_1_AND_2_ARE_WORDS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const WE_HAVE_A_SCALE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const MORE_COMPONENT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const WE_HAVE_AN_X_AND_Y_SCALE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const WE_HAVE_A_TWO_BY_TWO: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_tt_build_tables")]
pub unsafe extern "C-unwind" fn tt_build_tables(
    mut sfont: *mut sfnt,
    mut g: *mut tt_glyphs,
) -> ::core::ffi::c_int {
    let mut hmtx_table_data: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut loca_table_data: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut glyf_table_data: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut hmtx_table_size: SFNT_ULONG = 0;
    let mut loca_table_size: SFNT_ULONG = 0;
    let mut glyf_table_size: SFNT_ULONG = 0;
    let mut head: *mut tt_head_table = ::core::ptr::null_mut::<tt_head_table>();
    let mut hhea: *mut tt_hhea_table = ::core::ptr::null_mut::<tt_hhea_table>();
    let mut maxp: *mut tt_maxp_table = ::core::ptr::null_mut::<tt_maxp_table>();
    let mut hmtx: *mut tt_longMetrics = ::core::ptr::null_mut::<tt_longMetrics>();
    let mut vmtx: *mut tt_longMetrics = ::core::ptr::null_mut::<tt_longMetrics>();
    let mut os2: *mut tt_os2__table = ::core::ptr::null_mut::<tt_os2__table>();
    let mut location: *mut SFNT_ULONG = ::core::ptr::null_mut::<SFNT_ULONG>();
    let mut offset: SFNT_ULONG = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut w_stat: *mut USHORT = ::core::ptr::null_mut::<USHORT>();
    if g.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"g\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if sfont.is_null() || (*sfont).handle == INVALID_HANDLE {
        pdf_abort!(b"File not opened.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*sfont).type_0 != SFNT_TYPE_TRUETYPE
        && (*sfont).type_0 != SFNT_TYPE_TTC
        && (*sfont).type_0 != SFNT_TYPE_DFONT
    {
        pdf_abort!(b"Invalid font type\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*g).num_glyphs as ::core::ffi::c_int > NUM_GLYPH_LIMIT {
        pdf_abort!(b"Too many glyphs.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    head = tt_read_head_table(sfont);
    hhea = tt_read_hhea_table(sfont);
    maxp = tt_read_maxp_table(sfont);
    if (*hhea).metricDataFormat as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        pdf_abort!(b"Unknown metricDataFormat.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*g).emsize = (*head).unitsPerEm;
    sfnt_locate_table(sfont, b"hmtx\0" as *const u8 as *const ::core::ffi::c_char);
    hmtx = tt_read_longMetrics(
        sfont,
        (*maxp).numGlyphs,
        (*hhea).numOfLongHorMetrics,
        (*hhea).numOfExSideBearings,
    );
    os2 = tt_read_os2__table(sfont);
    if !os2.is_null() {
        (*g).default_advh = ((*os2).sTypoAscender as ::core::ffi::c_int
            - (*os2).sTypoDescender as ::core::ffi::c_int) as USHORT;
        (*g).default_tsb = ((*g).default_advh as ::core::ffi::c_int
            - (*os2).sTypoAscender as ::core::ffi::c_int) as SHORT;
    }
    if sfnt_find_table_pos(sfont, b"vmtx\0" as *const u8 as *const ::core::ffi::c_char)
        > 0 as SFNT_ULONG
    {
        let mut vhea: *mut tt_vhea_table = ::core::ptr::null_mut::<tt_vhea_table>();
        vhea = tt_read_vhea_table(sfont);
        sfnt_locate_table(sfont, b"vmtx\0" as *const u8 as *const ::core::ffi::c_char);
        vmtx = tt_read_longMetrics(
            sfont,
            (*maxp).numGlyphs,
            (*vhea).numOfLongVerMetrics,
            (*vhea).numOfExSideBearings,
        );
        free(vhea as *mut ::core::ffi::c_void);
    } else {
        vmtx = ::core::ptr::null_mut::<tt_longMetrics>();
    }
    sfnt_locate_table(sfont, b"loca\0" as *const u8 as *const ::core::ffi::c_char);
    location = new(
        (((*maxp).numGlyphs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<SFNT_ULONG>() as usize) as uint32_t,
    ) as *mut SFNT_ULONG;
    if (*head).indexToLocFormat as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i <= (*maxp).numGlyphs as ::core::ffi::c_int {
            *location.offset(i as isize) =
                (2 as SFNT_ULONG).wrapping_mul(tt_get_unsigned_pair((*sfont).handle) as SFNT_ULONG);
            i += 1;
        }
    } else if (*head).indexToLocFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i <= (*maxp).numGlyphs as ::core::ffi::c_int {
            *location.offset(i as isize) = tt_get_unsigned_quad((*sfont).handle);
            i += 1;
        }
    } else {
        pdf_abort!(b"Unknown IndexToLocFormat.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    w_stat = new(
        (((*g).emsize as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<USHORT>() as usize) as uint32_t,
    ) as *mut USHORT;
    memset(
        w_stat as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<USHORT>() as size_t)
            .wrapping_mul(((*g).emsize as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as size_t),
    );
    offset = sfnt_locate_table(sfont, b"glyf\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as ::core::ffi::c_int;
    while i < NUM_GLYPH_LIMIT {
        let mut gid: USHORT = 0;
        let mut loc: SFNT_ULONG = 0;
        let mut len: SFNT_ULONG = 0;
        let mut p: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        let mut endptr: *mut BYTE = ::core::ptr::null_mut::<BYTE>();
        let mut number_of_contours: SHORT = 0;
        if i >= (*g).num_glyphs as ::core::ffi::c_int {
            break;
        }
        gid = (*(*g).gd.offset(i as isize)).ogid;
        if gid as ::core::ffi::c_int >= (*maxp).numGlyphs as ::core::ffi::c_int {
            pdf_abort!(
                b"Invalid glyph index (gid %u)\0" as *const u8 as *const ::core::ffi::c_char,
                gid as ::core::ffi::c_int,
            );
        }
        loc = *location.offset(gid as isize);
        len = (*location.offset((gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .wrapping_sub(loc);
        (*(*g).gd.offset(i as isize)).advw = (*hmtx.offset(gid as isize)).advance;
        (*(*g).gd.offset(i as isize)).lsb = (*hmtx.offset(gid as isize)).sideBearing;
        if !vmtx.is_null() {
            (*(*g).gd.offset(i as isize)).advh = (*vmtx.offset(gid as isize)).advance;
            (*(*g).gd.offset(i as isize)).tsb = (*vmtx.offset(gid as isize)).sideBearing;
        } else {
            (*(*g).gd.offset(i as isize)).advh = (*g).default_advh;
            (*(*g).gd.offset(i as isize)).tsb = (*g).default_tsb;
        }
        (*(*g).gd.offset(i as isize)).length = len;
        let ref mut fresh2 = (*(*g).gd.offset(i as isize)).data;
        *fresh2 = ::core::ptr::null_mut::<BYTE>();
        if (*(*g).gd.offset(i as isize)).advw as ::core::ffi::c_int
            <= (*g).emsize as ::core::ffi::c_int
        {
            let ref mut fresh3 = *w_stat.offset((*(*g).gd.offset(i as isize)).advw as isize);
            *fresh3 = (*fresh3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as USHORT;
        } else {
            let ref mut fresh4 = *w_stat
                .offset(((*g).emsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
            *fresh4 = (*fresh4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as USHORT;
        }
        if !(len == 0 as SFNT_ULONG) {
            if len < 10 as SFNT_ULONG {
                pdf_abort!(
                    b"Invalid TrueType glyph data (gid %u).\0" as *const u8
                        as *const ::core::ffi::c_char,
                    gid as ::core::ffi::c_int,
                );
            }
            p = new(
                (len as usize).wrapping_mul(::core::mem::size_of::<BYTE>() as usize) as uint32_t,
            ) as *mut BYTE;
            let ref mut fresh5 = (*(*g).gd.offset(i as isize)).data;
            *fresh5 = p;
            endptr = p.offset(len as isize);
            crate::backend::runtime::input_seek(
                (*sfont).handle,
                offset.wrapping_add(loc) as ssize_t,
                SEEK_SET,
            );
            number_of_contours = tt_get_signed_pair((*sfont).handle);
            p = p.offset(put_big_endian(
                p as *mut ::core::ffi::c_void,
                number_of_contours as SFNT_LONG,
                2 as ::core::ffi::c_int,
            ) as isize);
            (*(*g).gd.offset(i as isize)).llx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).lly = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).urx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).ury = tt_get_signed_pair((*sfont).handle);
            if vmtx.is_null() {
                (*(*g).gd.offset(i as isize)).tsb = ((*g).default_advh as ::core::ffi::c_int
                    - (*g).default_tsb as ::core::ffi::c_int
                    - (*(*g).gd.offset(i as isize)).ury as ::core::ffi::c_int)
                    as SHORT;
            }
            p = p.offset(put_big_endian(
                p as *mut ::core::ffi::c_void,
                (*(*g).gd.offset(i as isize)).llx as SFNT_LONG,
                2 as ::core::ffi::c_int,
            ) as isize);
            p = p.offset(put_big_endian(
                p as *mut ::core::ffi::c_void,
                (*(*g).gd.offset(i as isize)).lly as SFNT_LONG,
                2 as ::core::ffi::c_int,
            ) as isize);
            p = p.offset(put_big_endian(
                p as *mut ::core::ffi::c_void,
                (*(*g).gd.offset(i as isize)).urx as SFNT_LONG,
                2 as ::core::ffi::c_int,
            ) as isize);
            p = p.offset(put_big_endian(
                p as *mut ::core::ffi::c_void,
                (*(*g).gd.offset(i as isize)).ury as SFNT_LONG,
                2 as ::core::ffi::c_int,
            ) as isize);
            ttstub_input_read(
                (*sfont).handle,
                p as *mut ::core::ffi::c_char,
                len.wrapping_sub(10 as SFNT_ULONG) as size_t,
            );
            if (number_of_contours as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                let mut flags: USHORT = 0;
                let mut cgid: USHORT = 0;
                let mut new_gid: USHORT = 0;
                loop {
                    if p >= endptr {
                        pdf_abort!(
                            b"Invalid TrueType glyph data (gid %u): %u bytes\0" as *const u8
                                as *const ::core::ffi::c_char,
                            gid as ::core::ffi::c_int,
                            len,
                        );
                    }
                    flags = ((*p as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                        | *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as USHORT;
                    p = p.offset(2 as ::core::ffi::c_int as isize);
                    cgid = ((*p as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                        | *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as USHORT;
                    if cgid as ::core::ffi::c_int >= (*maxp).numGlyphs as ::core::ffi::c_int {
                        pdf_abort!(
                            b"Invalid gid (%u > %u) in composite glyph %u.\0" as *const u8
                                as *const ::core::ffi::c_char,
                            cgid as ::core::ffi::c_int,
                            (*maxp).numGlyphs as ::core::ffi::c_int,
                            gid as ::core::ffi::c_int,
                        );
                    }
                    new_gid = tt_find_glyph(g, cgid);
                    if new_gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        new_gid = tt_add_glyph(g, cgid, find_empty_slot(g));
                    }
                    p = p.offset(put_big_endian(
                        p as *mut ::core::ffi::c_void,
                        new_gid as SFNT_LONG,
                        2 as ::core::ffi::c_int,
                    ) as isize);
                    p = p.offset(
                        (if flags as ::core::ffi::c_int & ARG_1_AND_2_ARE_WORDS != 0 {
                            4 as ::core::ffi::c_int
                        } else {
                            2 as ::core::ffi::c_int
                        }) as isize,
                    );
                    if flags as ::core::ffi::c_int & WE_HAVE_A_SCALE != 0 {
                        p = p.offset(2 as ::core::ffi::c_int as isize);
                    } else if flags as ::core::ffi::c_int & WE_HAVE_AN_X_AND_Y_SCALE != 0 {
                        p = p.offset(4 as ::core::ffi::c_int as isize);
                    } else if flags as ::core::ffi::c_int & WE_HAVE_A_TWO_BY_TWO != 0 {
                        p = p.offset(8 as ::core::ffi::c_int as isize);
                    }
                    if !(flags as ::core::ffi::c_int & MORE_COMPONENT != 0) {
                        break;
                    }
                }
            }
        }
        i += 1;
    }
    free(location as *mut ::core::ffi::c_void);
    free(hmtx as *mut ::core::ffi::c_void);
    free(vmtx as *mut ::core::ffi::c_void);
    let mut max_count: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    (*g).dw = (*(*g).gd.offset(0 as ::core::ffi::c_int as isize)).advw;
    i = 0 as ::core::ffi::c_int;
    while i < (*g).emsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        if *w_stat.offset(i as isize) as ::core::ffi::c_int > max_count {
            max_count = *w_stat.offset(i as isize) as ::core::ffi::c_int;
            (*g).dw = i as USHORT;
        }
        i += 1;
    }
    free(w_stat as *mut ::core::ffi::c_void);
    qsort(
        (*g).gd as *mut ::core::ffi::c_void,
        (*g).num_glyphs as size_t,
        ::core::mem::size_of::<tt_glyph_desc>() as size_t,
        Some(
            glyf_cmp
                as unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    index_glyphs(g);
    let mut prev: USHORT = 0;
    let mut last_advw: USHORT = 0;
    let mut p_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut padlen: ::core::ffi::c_int = 0;
    let mut num_hm_known: ::core::ffi::c_int = 0;
    glyf_table_size = 0 as SFNT_ULONG;
    num_hm_known = 0 as ::core::ffi::c_int;
    last_advw = (*(*g)
        .gd
        .offset(((*g).num_glyphs as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
    .advw;
    i = (*g).num_glyphs as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        padlen = (if (*(*g).gd.offset(i as isize))
            .length
            .wrapping_rem(4 as SFNT_ULONG)
            != 0
        {
            (4 as SFNT_ULONG).wrapping_sub(
                (*(*g).gd.offset(i as isize))
                    .length
                    .wrapping_rem(4 as SFNT_ULONG),
            )
        } else {
            0 as SFNT_ULONG
        }) as ::core::ffi::c_int;
        glyf_table_size = glyf_table_size.wrapping_add(
            (*(*g).gd.offset(i as isize))
                .length
                .wrapping_add(padlen as SFNT_ULONG),
        );
        if num_hm_known == 0
            && last_advw as ::core::ffi::c_int
                != (*(*g).gd.offset(i as isize)).advw as ::core::ffi::c_int
        {
            (*hhea).numOfLongHorMetrics = ((*(*g).gd.offset(i as isize)).gid as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as USHORT;
            num_hm_known = 1 as ::core::ffi::c_int;
        }
        i -= 1;
    }
    if num_hm_known == 0 {
        (*hhea).numOfLongHorMetrics = 1 as USHORT;
    }
    hmtx_table_size = ((*hhea).numOfLongHorMetrics as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        + ((*g).last_gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int)
        as SFNT_ULONG;
    if (glyf_table_size as ::core::ffi::c_ulong) < 0x20000 as ::core::ffi::c_ulong {
        (*head).indexToLocFormat = 0 as SHORT;
        loca_table_size = (((*g).last_gid as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            * 2 as ::core::ffi::c_int) as SFNT_ULONG;
    } else {
        (*head).indexToLocFormat = 1 as SHORT;
        loca_table_size = (((*g).last_gid as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            * 4 as ::core::ffi::c_int) as SFNT_ULONG;
    }
    p_0 = new((hmtx_table_size as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    hmtx_table_data = p_0;
    q = new((loca_table_size as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    loca_table_data = q;
    glyf_table_data = new((glyf_table_size as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    offset = 0 as SFNT_ULONG;
    prev = 0 as USHORT;
    i = 0 as ::core::ffi::c_int;
    while i < (*g).num_glyphs as ::core::ffi::c_int {
        let mut gap: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        gap = (*(*g).gd.offset(i as isize)).gid as ::core::ffi::c_int
            - prev as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        j = 1 as ::core::ffi::c_int;
        while j <= gap {
            if prev as ::core::ffi::c_int + j
                == (*hhea).numOfLongHorMetrics as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            {
                p_0 = p_0.offset(put_big_endian(
                    p_0 as *mut ::core::ffi::c_void,
                    last_advw as SFNT_LONG,
                    2 as ::core::ffi::c_int,
                ) as isize);
            } else if prev as ::core::ffi::c_int + j
                < (*hhea).numOfLongHorMetrics as ::core::ffi::c_int
            {
                p_0 = p_0.offset(put_big_endian(
                    p_0 as *mut ::core::ffi::c_void,
                    0 as SFNT_LONG,
                    2 as ::core::ffi::c_int,
                ) as isize);
            }
            p_0 = p_0.offset(put_big_endian(
                p_0 as *mut ::core::ffi::c_void,
                0 as SFNT_LONG,
                2 as ::core::ffi::c_int,
            ) as isize);
            if (*head).indexToLocFormat as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                q = q.offset(put_big_endian(
                    q as *mut ::core::ffi::c_void,
                    offset.wrapping_div(2 as SFNT_ULONG) as USHORT as SFNT_LONG,
                    2 as ::core::ffi::c_int,
                ) as isize);
            } else {
                q = q.offset(put_big_endian(
                    q as *mut ::core::ffi::c_void,
                    offset as SFNT_LONG,
                    4 as ::core::ffi::c_int,
                ) as isize);
            }
            j += 1;
        }
        padlen = (if (*(*g).gd.offset(i as isize))
            .length
            .wrapping_rem(4 as SFNT_ULONG)
            != 0
        {
            (4 as SFNT_ULONG).wrapping_sub(
                (*(*g).gd.offset(i as isize))
                    .length
                    .wrapping_rem(4 as SFNT_ULONG),
            )
        } else {
            0 as SFNT_ULONG
        }) as ::core::ffi::c_int;
        if ((*(*g).gd.offset(i as isize)).gid as ::core::ffi::c_int)
            < (*hhea).numOfLongHorMetrics as ::core::ffi::c_int
        {
            p_0 = p_0.offset(put_big_endian(
                p_0 as *mut ::core::ffi::c_void,
                (*(*g).gd.offset(i as isize)).advw as SFNT_LONG,
                2 as ::core::ffi::c_int,
            ) as isize);
        }
        p_0 = p_0.offset(put_big_endian(
            p_0 as *mut ::core::ffi::c_void,
            (*(*g).gd.offset(i as isize)).lsb as SFNT_LONG,
            2 as ::core::ffi::c_int,
        ) as isize);
        if (*head).indexToLocFormat as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            q = q.offset(put_big_endian(
                q as *mut ::core::ffi::c_void,
                offset.wrapping_div(2 as SFNT_ULONG) as USHORT as SFNT_LONG,
                2 as ::core::ffi::c_int,
            ) as isize);
        } else {
            q = q.offset(put_big_endian(
                q as *mut ::core::ffi::c_void,
                offset as SFNT_LONG,
                4 as ::core::ffi::c_int,
            ) as isize);
        }
        memset(
            glyf_table_data.offset(offset as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (*(*g).gd.offset(i as isize))
                .length
                .wrapping_add(padlen as SFNT_ULONG) as size_t,
        );
        memcpy(
            glyf_table_data.offset(offset as isize) as *mut ::core::ffi::c_void,
            (*(*g).gd.offset(i as isize)).data as *const ::core::ffi::c_void,
            (*(*g).gd.offset(i as isize)).length as size_t,
        );
        offset = offset.wrapping_add(
            (*(*g).gd.offset(i as isize))
                .length
                .wrapping_add(padlen as SFNT_ULONG),
        );
        prev = (*(*g).gd.offset(i as isize)).gid;
        free((*(*g).gd.offset(i as isize)).data as *mut ::core::ffi::c_void);
        (*(*g).gd.offset(i as isize)).length = 0 as SFNT_ULONG;
        let ref mut fresh6 = (*(*g).gd.offset(i as isize)).data;
        *fresh6 = ::core::ptr::null_mut::<BYTE>();
        i += 1;
    }
    if (*head).indexToLocFormat as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        q = q.offset(put_big_endian(
            q as *mut ::core::ffi::c_void,
            offset.wrapping_div(2 as SFNT_ULONG) as USHORT as SFNT_LONG,
            2 as ::core::ffi::c_int,
        ) as isize);
    } else {
        q = q.offset(put_big_endian(
            q as *mut ::core::ffi::c_void,
            offset as SFNT_LONG,
            4 as ::core::ffi::c_int,
        ) as isize);
    }
    sfnt_set_table(
        sfont,
        b"hmtx\0" as *const u8 as *const ::core::ffi::c_char,
        hmtx_table_data as *mut ::core::ffi::c_void,
        hmtx_table_size,
    );
    sfnt_set_table(
        sfont,
        b"loca\0" as *const u8 as *const ::core::ffi::c_char,
        loca_table_data as *mut ::core::ffi::c_void,
        loca_table_size,
    );
    sfnt_set_table(
        sfont,
        b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
        glyf_table_data as *mut ::core::ffi::c_void,
        glyf_table_size,
    );
    (*head).checkSumAdjustment = 0 as SFNT_ULONG;
    (*maxp).numGlyphs = ((*g).last_gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as USHORT;
    sfnt_set_table(
        sfont,
        b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
        tt_pack_maxp_table(maxp) as *mut ::core::ffi::c_void,
        TT_MAXP_TABLE_SIZE as SFNT_ULONG,
    );
    sfnt_set_table(
        sfont,
        b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
        tt_pack_hhea_table(hhea) as *mut ::core::ffi::c_void,
        TT_HHEA_TABLE_SIZE as SFNT_ULONG,
    );
    sfnt_set_table(
        sfont,
        b"head\0" as *const u8 as *const ::core::ffi::c_char,
        tt_pack_head_table(head) as *mut ::core::ffi::c_void,
        TT_HEAD_TABLE_SIZE as SFNT_ULONG,
    );
    free(maxp as *mut ::core::ffi::c_void);
    free(hhea as *mut ::core::ffi::c_void);
    free(head as *mut ::core::ffi::c_void);
    free(os2 as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_tt_get_metrics")]
pub unsafe extern "C-unwind" fn tt_get_metrics(
    mut sfont: *mut sfnt,
    mut g: *mut tt_glyphs,
) -> ::core::ffi::c_int {
    let mut head: *mut tt_head_table = ::core::ptr::null_mut::<tt_head_table>();
    let mut hhea: *mut tt_hhea_table = ::core::ptr::null_mut::<tt_hhea_table>();
    let mut maxp: *mut tt_maxp_table = ::core::ptr::null_mut::<tt_maxp_table>();
    let mut hmtx: *mut tt_longMetrics = ::core::ptr::null_mut::<tt_longMetrics>();
    let mut vmtx: *mut tt_longMetrics = ::core::ptr::null_mut::<tt_longMetrics>();
    let mut os2: *mut tt_os2__table = ::core::ptr::null_mut::<tt_os2__table>();
    let mut location: *mut SFNT_ULONG = ::core::ptr::null_mut::<SFNT_ULONG>();
    let mut offset: SFNT_ULONG = 0;
    let mut i: ::core::ffi::c_uint = 0;
    let mut w_stat: *mut USHORT = ::core::ptr::null_mut::<USHORT>();
    if g.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"g\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if sfont.is_null() || (*sfont).handle == INVALID_HANDLE {
        pdf_abort!(b"File not opened.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*sfont).type_0 != SFNT_TYPE_TRUETYPE
        && (*sfont).type_0 != SFNT_TYPE_TTC
        && (*sfont).type_0 != SFNT_TYPE_DFONT
    {
        pdf_abort!(b"Invalid font type\0" as *const u8 as *const ::core::ffi::c_char);
    }
    head = tt_read_head_table(sfont);
    hhea = tt_read_hhea_table(sfont);
    maxp = tt_read_maxp_table(sfont);
    if (*hhea).metricDataFormat as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        pdf_abort!(b"Unknown metricDataFormat.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*g).emsize = (*head).unitsPerEm;
    sfnt_locate_table(sfont, b"hmtx\0" as *const u8 as *const ::core::ffi::c_char);
    hmtx = tt_read_longMetrics(
        sfont,
        (*maxp).numGlyphs,
        (*hhea).numOfLongHorMetrics,
        (*hhea).numOfExSideBearings,
    );
    os2 = tt_read_os2__table(sfont);
    (*g).default_advh = ((*os2).sTypoAscender as ::core::ffi::c_int
        - (*os2).sTypoDescender as ::core::ffi::c_int) as USHORT;
    (*g).default_tsb = ((*g).default_advh as ::core::ffi::c_int
        - (*os2).sTypoAscender as ::core::ffi::c_int) as SHORT;
    if sfnt_find_table_pos(sfont, b"vmtx\0" as *const u8 as *const ::core::ffi::c_char)
        > 0 as SFNT_ULONG
    {
        let mut vhea: *mut tt_vhea_table = ::core::ptr::null_mut::<tt_vhea_table>();
        vhea = tt_read_vhea_table(sfont);
        sfnt_locate_table(sfont, b"vmtx\0" as *const u8 as *const ::core::ffi::c_char);
        vmtx = tt_read_longMetrics(
            sfont,
            (*maxp).numGlyphs,
            (*vhea).numOfLongVerMetrics,
            (*vhea).numOfExSideBearings,
        );
        free(vhea as *mut ::core::ffi::c_void);
    } else {
        vmtx = ::core::ptr::null_mut::<tt_longMetrics>();
    }
    sfnt_locate_table(sfont, b"loca\0" as *const u8 as *const ::core::ffi::c_char);
    location = new(
        (((*maxp).numGlyphs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<SFNT_ULONG>() as usize) as uint32_t,
    ) as *mut SFNT_ULONG;
    if (*head).indexToLocFormat as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_uint;
        while i <= (*maxp).numGlyphs as ::core::ffi::c_uint {
            *location.offset(i as isize) =
                (2 as SFNT_ULONG).wrapping_mul(tt_get_unsigned_pair((*sfont).handle) as SFNT_ULONG);
            i = i.wrapping_add(1);
        }
    } else if (*head).indexToLocFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_uint;
        while i <= (*maxp).numGlyphs as ::core::ffi::c_uint {
            *location.offset(i as isize) = tt_get_unsigned_quad((*sfont).handle);
            i = i.wrapping_add(1);
        }
    } else {
        pdf_abort!(b"Unknown IndexToLocFormat.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    w_stat = new(
        (((*g).emsize as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<USHORT>() as usize) as uint32_t,
    ) as *mut USHORT;
    memset(
        w_stat as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<USHORT>() as size_t)
            .wrapping_mul(((*g).emsize as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as size_t),
    );
    offset = sfnt_locate_table(sfont, b"glyf\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as ::core::ffi::c_uint;
    while i < (*g).num_glyphs as ::core::ffi::c_uint {
        let mut gid: USHORT = 0;
        let mut loc: SFNT_ULONG = 0;
        let mut len: SFNT_ULONG = 0;
        gid = (*(*g).gd.offset(i as isize)).ogid;
        if gid as ::core::ffi::c_int >= (*maxp).numGlyphs as ::core::ffi::c_int {
            pdf_abort!(
                b"Invalid glyph index (gid %u)\0" as *const u8 as *const ::core::ffi::c_char,
                gid as ::core::ffi::c_int,
            );
        }
        loc = *location.offset(gid as isize);
        len = (*location.offset((gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .wrapping_sub(loc);
        (*(*g).gd.offset(i as isize)).advw = (*hmtx.offset(gid as isize)).advance;
        (*(*g).gd.offset(i as isize)).lsb = (*hmtx.offset(gid as isize)).sideBearing;
        if !vmtx.is_null() {
            (*(*g).gd.offset(i as isize)).advh = (*vmtx.offset(gid as isize)).advance;
            (*(*g).gd.offset(i as isize)).tsb = (*vmtx.offset(gid as isize)).sideBearing;
        } else {
            (*(*g).gd.offset(i as isize)).advh = (*g).default_advh;
            (*(*g).gd.offset(i as isize)).tsb = (*g).default_tsb;
        }
        (*(*g).gd.offset(i as isize)).length = len;
        let ref mut fresh7 = (*(*g).gd.offset(i as isize)).data;
        *fresh7 = ::core::ptr::null_mut::<BYTE>();
        if (*(*g).gd.offset(i as isize)).advw as ::core::ffi::c_int
            <= (*g).emsize as ::core::ffi::c_int
        {
            let ref mut fresh8 = *w_stat.offset((*(*g).gd.offset(i as isize)).advw as isize);
            *fresh8 = (*fresh8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as USHORT;
        } else {
            let ref mut fresh9 = *w_stat
                .offset(((*g).emsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize);
            *fresh9 = (*fresh9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as USHORT;
        }
        if !(len == 0 as SFNT_ULONG) {
            if len < 10 as SFNT_ULONG {
                pdf_abort!(
                    b"Invalid TrueType glyph data (gid %u).\0" as *const u8
                        as *const ::core::ffi::c_char,
                    gid as ::core::ffi::c_int,
                );
            }
            crate::backend::runtime::input_seek(
                (*sfont).handle,
                offset.wrapping_add(loc) as ssize_t,
                SEEK_SET,
            );
            tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).llx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).lly = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).urx = tt_get_signed_pair((*sfont).handle);
            (*(*g).gd.offset(i as isize)).ury = tt_get_signed_pair((*sfont).handle);
            if vmtx.is_null() {
                (*(*g).gd.offset(i as isize)).tsb = ((*g).default_advh as ::core::ffi::c_int
                    - (*g).default_tsb as ::core::ffi::c_int
                    - (*(*g).gd.offset(i as isize)).ury as ::core::ffi::c_int)
                    as SHORT;
            }
        }
        i = i.wrapping_add(1);
    }
    free(location as *mut ::core::ffi::c_void);
    free(hmtx as *mut ::core::ffi::c_void);
    free(maxp as *mut ::core::ffi::c_void);
    free(hhea as *mut ::core::ffi::c_void);
    free(head as *mut ::core::ffi::c_void);
    free(os2 as *mut ::core::ffi::c_void);
    free(vmtx as *mut ::core::ffi::c_void);
    let mut max_count: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    (*g).dw = (*(*g).gd.offset(0 as ::core::ffi::c_int as isize)).advw;
    i = 0 as ::core::ffi::c_uint;
    while i < ((*g).emsize as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint {
        if *w_stat.offset(i as isize) as ::core::ffi::c_int > max_count {
            max_count = *w_stat.offset(i as isize) as ::core::ffi::c_int;
            (*g).dw = i as USHORT;
        }
        i = i.wrapping_add(1);
    }
    free(w_stat as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_find_glyph(
    mut g: *mut tt_glyphs,
    mut gid: USHORT,
    _result: *mut USHORT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_find_glyph(g, gid) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_index(
    mut g: *mut tt_glyphs,
    mut gid: USHORT,
    _result: *mut USHORT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_index(g, gid) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_add_glyph(
    mut g: *mut tt_glyphs,
    mut gid: USHORT,
    mut new_gid: USHORT,
    _result: *mut USHORT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_add_glyph(g, gid, new_gid) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_build_init(_result: *mut *mut tt_glyphs) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_build_init() }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_build_finish(mut g: *mut tt_glyphs) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_build_finish(g) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_build_tables(
    mut sfont: *mut sfnt,
    mut g: *mut tt_glyphs,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_build_tables(sfont, g) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_metrics(
    mut sfont: *mut sfnt,
    mut g: *mut tt_glyphs,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_metrics(sfont, g) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
