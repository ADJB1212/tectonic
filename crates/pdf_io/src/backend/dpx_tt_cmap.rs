/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2007-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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
use crate::compat::allocation::free;

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn format_four_search_preserves_deltas_gaps_and_glyph_array_offsets() {
        let mut starts = [10, 20, 65535];
        let mut ends = [12, 22, 65535];
        let mut deltas = [1, 65535, 1];
        let mut offsets = [6, 10, 0];
        let mut glyphs = [1, 0, 65535, 7, 8, 9];
        let mut map = cmap4 {
            segCountX2: 6,
            searchRange: 0,
            entrySelector: 0,
            rangeShift: 0,
            endCount: ends.as_mut_ptr(),
            reservedPad: 0,
            startCount: starts.as_mut_ptr(),
            idDelta: deltas.as_mut_ptr(),
            idRangeOffset: offsets.as_mut_ptr(),
            glyphIndexArray: glyphs.as_mut_ptr(),
        };
        for code in 0..=u16::MAX {
            let expected = match code {
                10 => 2,
                20..=22 => code - 14,
                _ => 0,
            };
            assert_eq!(unsafe { lookup_cmap4(&mut map, code) }, expected);
        }
        offsets.fill(0);
        for code in 0..=u16::MAX {
            let expected = match code {
                10..=12 => code + 1,
                20..=22 => code - 1,
                _ => 0,
            };
            assert_eq!(unsafe { lookup_cmap4(&mut map, code) }, expected);
        }
        map.segCountX2 = 0;
        map.startCount = core::ptr::null_mut();
        assert_eq!(unsafe { lookup_cmap4(&mut map, 0) }, 0);
    }

    #[test]
    fn format_twelve_search_handles_supplementary_ranges_gaps_and_wrapping() {
        let mut groups: Vec<charGroup> = (0..4096)
            .map(|index| charGroup {
                startCharCode: 65536 + index * 128,
                endCharCode: 65551 + index * 128,
                startGlyphID: index * 17,
            })
            .collect();
        let mut map = cmap12 {
            nGroups: groups.len() as u32,
            groups: groups.as_mut_ptr(),
        };
        for group in &groups {
            for offset in [0, 1, 15, 16, 127] {
                let expected = if offset <= 15 { (group.startGlyphID + offset) as u16 } else { 0 };
                assert_eq!(unsafe { lookup_cmap12(&mut map, group.startCharCode + offset) }, expected);
            }
        }
        assert_eq!(unsafe { lookup_cmap12(&mut map, 0) }, 0);
        assert_eq!(unsafe { lookup_cmap12(&mut map, u32::MAX) }, 0);
        map.nGroups = 0;
        map.groups = core::ptr::null_mut();
        assert_eq!(unsafe { lookup_cmap12(&mut map, 0) }, 0);
    }
}
use crate::unicode::is_valid as UC_is_valid;
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn sprintf(_: *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(__str: *mut ::core::ffi::c_char, __size: size_t, __format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    #[cfg_attr(windows, link_name = "_strdup")]
    fn strdup(__s1: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: ::core::ffi::c_int) -> size_t;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tt_get_unsigned_byte"]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_tt_get_signed_pair"]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> ::core::ffi::c_short;
    #[link_name = "__pdf_impl_tt_get_unsigned_quad"]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    #[link_name = "__pdf_impl_sfnt_open"]
    fn sfnt_open(handle: rust_input_handle_t) -> *mut sfnt;
    #[link_name = "__pdf_impl_dfont_open"]
    fn dfont_open(handle: rust_input_handle_t, index: ::core::ffi::c_int) -> *mut sfnt;
    #[link_name = "__pdf_impl_sfnt_close"]
    fn sfnt_close(sfont: *mut sfnt);
    #[link_name = "__pdf_impl_sfnt_read_table_directory"]
    fn sfnt_read_table_directory(sfont: *mut sfnt, offset: SFNT_ULONG) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_sfnt_find_table_pos"]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_sfnt_locate_table"]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_agl_get_unicodes"]
    fn agl_get_unicodes(glyphstr: *const ::core::ffi::c_char, unicodes: *mut int32_t, max_uncodes: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut CSI_UNICODE: CIDSysInfo;
    #[link_name = "__pdf_impl_CMap_set_silent"]
    fn CMap_set_silent(value: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_CMap_new"]
    fn CMap_new() -> *mut CMap;
    #[link_name = "__pdf_impl_CMap_release"]
    fn CMap_release(cmap: *mut CMap);
    #[link_name = "__pdf_impl_CMap_set_name"]
    fn CMap_set_name(cmap: *mut CMap, name: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_CMap_set_type"]
    fn CMap_set_type(cmap: *mut CMap, type_0: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_CMap_set_wmode"]
    fn CMap_set_wmode(cmap: *mut CMap, wmode: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_CMap_set_CIDSysInfo"]
    fn CMap_set_CIDSysInfo(cmap: *mut CMap, csi: *const CIDSysInfo);
    #[link_name = "__pdf_impl_CMap_add_bfchar"]
    fn CMap_add_bfchar(
        cmap: *mut CMap, src: *const ::core::ffi::c_uchar, srcdim: size_t, dest: *const ::core::ffi::c_uchar, destdim: size_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_add_cidchar"]
    fn CMap_add_cidchar(cmap: *mut CMap, src: *const ::core::ffi::c_uchar, srcdim: size_t, dest: CID) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_add_codespacerange"]
    fn CMap_add_codespacerange(
        cmap: *mut CMap, codelo: *const ::core::ffi::c_uchar, codehi: *const ::core::ffi::c_uchar, dim: size_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_decode"]
    fn CMap_decode(
        cmap: *mut CMap, inbuf: *mut *const ::core::ffi::c_uchar, inbytesleft: *mut size_t, outbuf: *mut *mut ::core::ffi::c_uchar,
        outbytesleft: *mut size_t,
    ) -> size_t;
    #[link_name = "__pdf_impl_CMap_cache_get"]
    fn CMap_cache_get(id: ::core::ffi::c_int) -> *mut CMap;
    #[link_name = "__pdf_impl_CMap_cache_find"]
    fn CMap_cache_find(cmap_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_cache_add"]
    fn CMap_cache_add(cmap: *mut CMap) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_create_stream"]
    fn CMap_create_stream(cmap: *mut CMap) -> *mut pdf_obj;
    static mut dpx_conf: _dpx_conf;
    #[link_name = "__pdf_impl_dpx_open_truetype_file"]
    fn dpx_open_truetype_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_dpx_open_opentype_file"]
    fn dpx_open_opentype_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_dpx_open_dfont_file"]
    fn dpx_open_dfont_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_defineresource"]
    fn pdf_defineresource(
        category: *const ::core::ffi::c_char, resname: *const ::core::ffi::c_char, object: *mut pdf_obj, flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_findresource"]
    fn pdf_findresource(category: *const ::core::ffi::c_char, resname: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_resource_reference"]
    fn pdf_get_resource_reference(res_id: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_ttc_read_offset"]
    fn ttc_read_offset(sfont: *mut sfnt, ttc_idx: SFNT_ULONG) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_otl_gsub_new"]
    fn otl_gsub_new() -> *mut otl_gsub;
    #[link_name = "__pdf_impl_otl_gsub_release"]
    fn otl_gsub_release(gsub_list: *mut otl_gsub);
    #[link_name = "__pdf_impl_otl_gsub_select"]
    fn otl_gsub_select(
        gsub_list: *mut otl_gsub, script: *const ::core::ffi::c_char, language: *const ::core::ffi::c_char, feature: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_add_feat"]
    fn otl_gsub_add_feat(
        gsub_list: *mut otl_gsub, script: *const ::core::ffi::c_char, language: *const ::core::ffi::c_char, feature: *const ::core::ffi::c_char,
        sfont: *mut sfnt,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_apply"]
    fn otl_gsub_apply(gsub_list: *mut otl_gsub, gid: *mut USHORT) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_add_feat_list"]
    fn otl_gsub_add_feat_list(gsub_list: *mut otl_gsub, otl_tags: *const ::core::ffi::c_char, sfont: *mut sfnt) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_set_chain"]
    fn otl_gsub_set_chain(gsub_list: *mut otl_gsub, otl_tags: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_apply_chain"]
    fn otl_gsub_apply_chain(gsub_list: *mut otl_gsub, gid: *mut USHORT) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_add_ToUnicode"]
    fn otl_gsub_add_ToUnicode(
        cmap: *mut CMap, used_chars: *mut ::core::ffi::c_char, map_base: *mut int32_t, map_sub: *mut int32_t, num_glyphs: USHORT,
        GIDToCIDMap: *mut uint16_t, sfont: *mut sfnt,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tt_read_post_table"]
    fn tt_read_post_table(sfont: *mut sfnt) -> *mut tt_post_table;
    #[link_name = "__pdf_impl_tt_release_post_table"]
    fn tt_release_post_table(post: *mut tt_post_table);
    #[link_name = "__pdf_impl_tt_get_glyphname"]
    fn tt_get_glyphname(post: *mut tt_post_table, gid: USHORT) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_UC_UTF16BE_encode_char"]
    fn UC_UTF16BE_encode_char(ucv: int32_t, dstpp: *mut *mut ::core::ffi::c_uchar, endptr: *mut ::core::ffi::c_uchar) -> size_t;
    #[link_name = "__pdf_impl_cff_open"]
    fn cff_open(handle: rust_input_handle_t, offset: ::core::ffi::c_int, idx: ::core::ffi::c_int) -> *mut cff_font;
    #[link_name = "__pdf_impl_cff_close"]
    fn cff_close(cff: *mut cff_font);
    #[link_name = "__pdf_impl_cff_read_charsets"]
    fn cff_read_charsets(cff: *mut cff_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_get_glyphname"]
    fn cff_get_glyphname(cff: *mut cff_font, gid: card16) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_cff_get_string"]
    fn cff_get_string(cff: *mut cff_font, id: s_SID) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_cff_dict_get"]
    fn cff_dict_get(dict: *mut cff_dict, key: *const ::core::ffi::c_char, idx: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_cff_dict_known"]
    fn cff_dict_known(dict: *mut cff_dict, key: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tt_read_maxp_table"]
    fn tt_read_maxp_table(sfont: *mut sfnt) -> *mut tt_maxp_table;
}
pub type int32_t = i32;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
pub type rust_input_handle_t = uintptr_t;
pub type BYTE = ::core::ffi::c_uchar;
pub type USHORT = ::core::ffi::c_ushort;
pub type SHORT = ::core::ffi::c_short;
pub type SFNT_ULONG = uint32_t;
pub type Fixed = uint32_t;
pub type FWord = ::core::ffi::c_short;
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
pub struct tt_cmap {
    pub format: USHORT,
    pub platform: USHORT,
    pub encoding: USHORT,
    pub language: SFNT_ULONG,
    pub map: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap12 {
    pub nGroups: SFNT_ULONG,
    pub groups: *mut charGroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charGroup {
    pub startCharCode: SFNT_ULONG,
    pub endCharCode: SFNT_ULONG,
    pub startGlyphID: SFNT_ULONG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap6 {
    pub firstCode: USHORT,
    pub entryCount: USHORT,
    pub glyphIndexArray: *mut USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap4 {
    pub segCountX2: USHORT,
    pub searchRange: USHORT,
    pub entrySelector: USHORT,
    pub rangeShift: USHORT,
    pub endCount: *mut USHORT,
    pub reservedPad: USHORT,
    pub startCount: *mut USHORT,
    pub idDelta: *mut USHORT,
    pub idRangeOffset: *mut USHORT,
    pub glyphIndexArray: *mut USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap2 {
    pub subHeaderKeys: [USHORT; 256],
    pub subHeaders: *mut SubHeader,
    pub glyphIndexArray: *mut USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubHeader {
    pub firstCode: USHORT,
    pub entryCount: USHORT,
    pub idDelta: SHORT,
    pub idRangeOffset: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap0 {
    pub glyphIndexArray: [BYTE; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_0,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: ::core::ffi::c_int,
    pub profile: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut ::core::ffi::c_uchar,
    pub prev: *mut mapData,
    pub pos: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: ::core::ffi::c_int,
    pub len: size_t,
    pub code: *mut ::core::ffi::c_uchar,
    pub next: *mut mapDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub num: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
    pub ranges: *mut rangeDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rangeDef {
    pub dim: size_t,
    pub codeLo: *mut ::core::ffi::c_uchar,
    pub codeHi: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDSysInfo {
    pub registry: *mut ::core::ffi::c_char,
    pub ordering: *mut ::core::ffi::c_char,
    pub supplement: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub keep_cache: ::core::ffi::c_int,
}
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_font {
    pub fontname: *mut ::core::ffi::c_char,
    pub header: cff_header,
    pub name: *mut cff_index,
    pub topdict: *mut cff_dict,
    pub string: *mut cff_index,
    pub gsubr: *mut cff_index,
    pub encoding: *mut cff_encoding,
    pub charsets: *mut cff_charsets,
    pub fdselect: *mut cff_fdselect,
    pub cstrings: *mut cff_index,
    pub fdarray: *mut *mut cff_dict,
    pub private: *mut *mut cff_dict,
    pub subrs: *mut *mut cff_index,
    pub offset: l_offset,
    pub gsubr_offset: l_offset,
    pub num_glyphs: card16,
    pub num_fds: card8,
    pub _string: *mut cff_index,
    pub handle: rust_input_handle_t,
    pub filter: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub flag: ::core::ffi::c_int,
    pub is_notdef_notzero: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_index {
    pub count: card16,
    pub offsize: c_offsize,
    pub offset: *mut l_offset,
    pub data: *mut card8,
}
pub type card8 = ::core::ffi::c_uchar;
pub type l_offset = uint32_t;
pub type c_offsize = ::core::ffi::c_uchar;
pub type card16 = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict {
    pub max: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub entries: *mut cff_dict_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict_entry {
    pub id: ::core::ffi::c_int,
    pub key: *const ::core::ffi::c_char,
    pub count: ::core::ffi::c_int,
    pub values: *mut ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_fdselect {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub fds: *mut card8,
    pub ranges: *mut cff_range3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range3 {
    pub first: card16,
    pub fd: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_charsets {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub glyphs: *mut s_SID,
    pub range1: *mut cff_range1,
    pub range2: *mut cff_range2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range2 {
    pub first: s_SID,
    pub n_left: card16,
}
pub type s_SID = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range1 {
    pub first: s_SID,
    pub n_left: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_encoding {
    pub format: card8,
    pub num_entries: card8,
    pub data: C2RustUnnamed_4,
    pub num_supps: card8,
    pub supp: *mut cff_map,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_map {
    pub code: card8,
    pub glyph: s_SID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub codes: *mut card8,
    pub range1: *mut cff_range1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_header {
    pub major: card8,
    pub minor: card8,
    pub hdr_size: card8,
    pub offsize: c_offsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_post_table {
    pub Version: Fixed,
    pub italicAngle: Fixed,
    pub underlinePosition: FWord,
    pub underlineThickness: FWord,
    pub isFixedPitch: SFNT_ULONG,
    pub minMemType42: SFNT_ULONG,
    pub maxMemType42: SFNT_ULONG,
    pub minMemType1: SFNT_ULONG,
    pub maxMemType1: SFNT_ULONG,
    pub numberOfGlyphs: USHORT,
    pub glyphNamePtr: *mut *const ::core::ffi::c_char,
    pub names: *mut *mut ::core::ffi::c_char,
    pub count: USHORT,
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
pub struct cmap_plat_enc_rec {
    pub platform: ::core::ffi::c_short,
    pub encoding: ::core::ffi::c_short,
}
pub type CID = ::core::ffi::c_ushort;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
pub const SFNT_TYPE_TRUETYPE: ::core::ffi::c_int = 1;
pub const SFNT_TYPE_POSTSCRIPT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const SFNT_TYPE_TTC: ::core::ffi::c_int = 16;
pub const SFNT_TYPE_DFONT: ::core::ffi::c_int = 256;
pub const CMAP_TYPE_CODE_TO_CID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CMAP_TYPE_TO_UNICODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_RES_FLUSH_IMMEDIATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VERBOSE_LEVEL_MIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn read_cmap0(mut sfont: *mut sfnt, mut len: SFNT_ULONG) -> *mut cmap0 {
    let mut map: *mut cmap0 = ::core::ptr::null_mut::<cmap0>();
    let mut i: ::core::ffi::c_uint = 0;
    if len < 256 as SFNT_ULONG {
        pdf_warning!(b"invalid format 0 TT cmap subtable\0" as *const u8 as *const ::core::ffi::c_char,);
        return ::core::ptr::null_mut::<cmap0>();
    }
    map = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cmap0>() as usize) as uint32_t) as *mut cmap0;
    i = 0 as ::core::ffi::c_uint;
    while i < 256 as ::core::ffi::c_uint {
        (*map).glyphIndexArray[i as usize] = tt_get_unsigned_byte((*sfont).handle);
        i = i.wrapping_add(1);
    }
    return map;
}
unsafe extern "C-unwind" fn release_cmap0(mut map: *mut cmap0) {
    free(map as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn lookup_cmap0(mut map: *mut cmap0, mut cc: USHORT) -> USHORT {
    return (if cc as ::core::ffi::c_int > 255 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        (*map).glyphIndexArray[cc as usize] as ::core::ffi::c_int
    }) as USHORT;
}
unsafe extern "C-unwind" fn read_cmap2(mut sfont: *mut sfnt, mut len: SFNT_ULONG) -> *mut cmap2 {
    let mut map: *mut cmap2 = ::core::ptr::null_mut::<cmap2>();
    let mut i: USHORT = 0;
    let mut n: USHORT = 0;
    if len < 512 as SFNT_ULONG {
        pdf_warning!(b"invalid fromt2 TT cmap subtable\0" as *const u8 as *const ::core::ffi::c_char,);
        return ::core::ptr::null_mut::<cmap2>();
    }
    map = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cmap2>() as usize) as uint32_t) as *mut cmap2;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
        (*map).subHeaderKeys[i as usize] = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1);
    }
    n = 0 as USHORT;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
        (*map).subHeaderKeys[i as usize] = ((*map).subHeaderKeys[i as usize] as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as USHORT;
        if (n as ::core::ffi::c_int) < (*map).subHeaderKeys[i as usize] as ::core::ffi::c_int {
            n = (*map).subHeaderKeys[i as usize];
        }
        i = i.wrapping_add(1);
    }
    n = (n as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as USHORT;
    if len < (512 as ::core::ffi::c_int + n as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as SFNT_ULONG {
        pdf_warning!(b"invalid/truncated format2 TT cmap subtable\0" as *const u8 as *const ::core::ffi::c_char,);
        free(map as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<cmap2>();
    }
    (*map).subHeaders = new((n as uint32_t as usize).wrapping_mul(::core::mem::size_of::<SubHeader>() as usize) as uint32_t) as *mut SubHeader;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < n as ::core::ffi::c_int {
        (*(*map).subHeaders.offset(i as isize)).firstCode = tt_get_unsigned_pair((*sfont).handle);
        (*(*map).subHeaders.offset(i as isize)).entryCount = tt_get_unsigned_pair((*sfont).handle);
        (*(*map).subHeaders.offset(i as isize)).idDelta = tt_get_signed_pair((*sfont).handle);
        (*(*map).subHeaders.offset(i as isize)).idRangeOffset = tt_get_unsigned_pair((*sfont).handle);
        if (*(*map).subHeaders.offset(i as isize)).idRangeOffset as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            let ref mut fresh0 = (*(*map).subHeaders.offset(i as isize)).idRangeOffset;
            *fresh0 = (*fresh0 as ::core::ffi::c_int
                - (2 as ::core::ffi::c_int + (n as ::core::ffi::c_int - i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int))
                as USHORT;
        }
        i = i.wrapping_add(1);
    }
    n = (len
        .wrapping_sub(518 as SFNT_ULONG)
        .wrapping_sub((n as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as SFNT_ULONG) as USHORT as ::core::ffi::c_int
        / 2 as ::core::ffi::c_int) as USHORT;
    (*map).glyphIndexArray = new((n as uint32_t as usize).wrapping_mul(::core::mem::size_of::<USHORT>() as usize) as uint32_t) as *mut USHORT;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < n as ::core::ffi::c_int {
        *(*map).glyphIndexArray.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1);
    }
    return map;
}
unsafe extern "C-unwind" fn release_cmap2(mut map: *mut cmap2) {
    if !map.is_null() {
        free((*map).subHeaders as *mut ::core::ffi::c_void);
        free((*map).glyphIndexArray as *mut ::core::ffi::c_void);
        free(map as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C-unwind" fn lookup_cmap2(mut map: *mut cmap2, mut cc: USHORT) -> USHORT {
    let mut idx: USHORT = 0 as USHORT;
    let mut idDelta: SHORT = 0;
    let mut firstCode: USHORT = 0;
    let mut entryCount: USHORT = 0;
    let mut idRangeOffset: USHORT = 0;
    let mut hi: ::core::ffi::c_int = 0;
    let mut lo: ::core::ffi::c_int = 0;
    let mut i: USHORT = 0;
    hi = cc as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
    lo = cc as ::core::ffi::c_int & 0xff as ::core::ffi::c_int;
    i = (*map).subHeaderKeys[hi as usize];
    firstCode = (*(*map).subHeaders.offset(i as isize)).firstCode;
    entryCount = (*(*map).subHeaders.offset(i as isize)).entryCount;
    idDelta = (*(*map).subHeaders.offset(i as isize)).idDelta;
    idRangeOffset = ((*(*map).subHeaders.offset(i as isize)).idRangeOffset as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as USHORT;
    if lo >= firstCode as ::core::ffi::c_int && lo < firstCode as ::core::ffi::c_int + entryCount as ::core::ffi::c_int {
        idRangeOffset = (idRangeOffset as ::core::ffi::c_int + (lo - firstCode as ::core::ffi::c_int)) as USHORT;
        idx = *(*map).glyphIndexArray.offset(idRangeOffset as isize);
        if idx as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            idx = (idx as ::core::ffi::c_int + idDelta as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int) as USHORT;
        }
    }
    return idx;
}
unsafe extern "C-unwind" fn read_cmap4(mut sfont: *mut sfnt, mut len: SFNT_ULONG) -> *mut cmap4 {
    let mut map: *mut cmap4 = ::core::ptr::null_mut::<cmap4>();
    let mut i: USHORT = 0;
    let mut n: USHORT = 0;
    let mut segCount: USHORT = 0;
    if len < 8 as SFNT_ULONG {
        pdf_warning!(b"invalid format 4 TT cmap subtable\0" as *const u8 as *const ::core::ffi::c_char,);
        return ::core::ptr::null_mut::<cmap4>();
    }
    map = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cmap4>() as usize) as uint32_t) as *mut cmap4;
    segCount = tt_get_unsigned_pair((*sfont).handle);
    (*map).segCountX2 = segCount;
    (*map).searchRange = tt_get_unsigned_pair((*sfont).handle);
    (*map).entrySelector = tt_get_unsigned_pair((*sfont).handle);
    (*map).rangeShift = tt_get_unsigned_pair((*sfont).handle);
    segCount = (segCount as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as USHORT;
    (*map).endCount = new((segCount as uint32_t as usize).wrapping_mul(::core::mem::size_of::<USHORT>() as usize) as uint32_t) as *mut USHORT;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < segCount as ::core::ffi::c_int {
        *(*map).endCount.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1);
    }
    (*map).reservedPad = tt_get_unsigned_pair((*sfont).handle);
    (*map).startCount = new((segCount as uint32_t as usize).wrapping_mul(::core::mem::size_of::<USHORT>() as usize) as uint32_t) as *mut USHORT;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < segCount as ::core::ffi::c_int {
        *(*map).startCount.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1);
    }
    (*map).idDelta = new((segCount as uint32_t as usize).wrapping_mul(::core::mem::size_of::<USHORT>() as usize) as uint32_t) as *mut USHORT;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < segCount as ::core::ffi::c_int {
        *(*map).idDelta.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1);
    }
    (*map).idRangeOffset = new((segCount as uint32_t as usize).wrapping_mul(::core::mem::size_of::<USHORT>() as usize) as uint32_t) as *mut USHORT;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < segCount as ::core::ffi::c_int {
        *(*map).idRangeOffset.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1);
    }
    n = len
        .wrapping_sub(16 as SFNT_ULONG)
        .wrapping_sub((8 as ::core::ffi::c_int * segCount as ::core::ffi::c_int) as SFNT_ULONG)
        .wrapping_div(2 as SFNT_ULONG) as USHORT;
    if n as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*map).glyphIndexArray = ::core::ptr::null_mut::<USHORT>();
    } else {
        (*map).glyphIndexArray = new((n as uint32_t as usize).wrapping_mul(::core::mem::size_of::<USHORT>() as usize) as uint32_t) as *mut USHORT;
        i = 0 as USHORT;
        while (i as ::core::ffi::c_int) < n as ::core::ffi::c_int {
            *(*map).glyphIndexArray.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
            i = i.wrapping_add(1);
        }
    }
    return map;
}
unsafe extern "C-unwind" fn release_cmap4(mut map: *mut cmap4) {
    if !map.is_null() {
        free((*map).endCount as *mut ::core::ffi::c_void);
        free((*map).startCount as *mut ::core::ffi::c_void);
        free((*map).idDelta as *mut ::core::ffi::c_void);
        free((*map).idRangeOffset as *mut ::core::ffi::c_void);
        free((*map).glyphIndexArray as *mut ::core::ffi::c_void);
        free(map as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C-unwind" fn lookup_cmap4(mut map: *mut cmap4, mut cc: USHORT) -> USHORT {
    let mut gid: USHORT = 0 as USHORT;
    let mut i: USHORT = 0;
    let mut j: USHORT = 0;
    let mut segCount: USHORT = 0;
    segCount = ((*map).segCountX2 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as USHORT;
    if segCount == 0 {
        return 0;
    }
    let starts = core::slice::from_raw_parts((*map).startCount, segCount as usize);
    let Some(index) = starts.partition_point(|&start| start <= cc).checked_sub(1) else {
        return 0;
    };
    i = index as u16;
    if cc > *(*map).endCount.add(index) {
        return 0;
    }
    if *(*map).idRangeOffset.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        gid = (cc as ::core::ffi::c_int + *(*map).idDelta.offset(i as isize) as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int) as USHORT;
    } else if cc as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
        && *(*map).idRangeOffset.offset(i as isize) as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
    {
        gid = 0 as USHORT;
    } else {
        j = (*(*map).idRangeOffset.offset(i as isize) as ::core::ffi::c_int
            - (segCount as ::core::ffi::c_int - i as ::core::ffi::c_int) * 2 as ::core::ffi::c_int) as USHORT;
        j = (cc as ::core::ffi::c_int - *(*map).startCount.offset(i as isize) as ::core::ffi::c_int
            + j as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as USHORT;
        gid = *(*map).glyphIndexArray.offset(j as isize);
        if gid as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            gid = (gid as ::core::ffi::c_int + *(*map).idDelta.offset(i as isize) as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int) as USHORT;
        }
    }
    return gid;
}
unsafe extern "C-unwind" fn read_cmap6(mut sfont: *mut sfnt, mut len: SFNT_ULONG) -> *mut cmap6 {
    let mut map: *mut cmap6 = ::core::ptr::null_mut::<cmap6>();
    let mut i: USHORT = 0;
    if len < 4 as SFNT_ULONG {
        pdf_warning!(b"invalid foramt 6 TT cmap subtable\0" as *const u8 as *const ::core::ffi::c_char,);
        return ::core::ptr::null_mut::<cmap6>();
    }
    map = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cmap6>() as usize) as uint32_t) as *mut cmap6;
    (*map).firstCode = tt_get_unsigned_pair((*sfont).handle);
    (*map).entryCount = tt_get_unsigned_pair((*sfont).handle);
    (*map).glyphIndexArray =
        new(((*map).entryCount as uint32_t as usize).wrapping_mul(::core::mem::size_of::<USHORT>() as usize) as uint32_t) as *mut USHORT;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < (*map).entryCount as ::core::ffi::c_int {
        *(*map).glyphIndexArray.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
        i = i.wrapping_add(1);
    }
    return map;
}
unsafe extern "C-unwind" fn release_cmap6(mut map: *mut cmap6) {
    if !map.is_null() {
        free((*map).glyphIndexArray as *mut ::core::ffi::c_void);
        free(map as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C-unwind" fn lookup_cmap6(mut map: *mut cmap6, mut cc: USHORT) -> USHORT {
    let mut idx: USHORT = 0;
    idx = (cc as ::core::ffi::c_int - (*map).firstCode as ::core::ffi::c_int) as USHORT;
    if (idx as ::core::ffi::c_int) < (*map).entryCount as ::core::ffi::c_int {
        return *(*map).glyphIndexArray.offset(idx as isize);
    }
    return 0 as USHORT;
}
unsafe extern "C-unwind" fn read_cmap12(mut sfont: *mut sfnt, mut len: SFNT_ULONG) -> *mut cmap12 {
    let mut map: *mut cmap12 = ::core::ptr::null_mut::<cmap12>();
    let mut i: SFNT_ULONG = 0;
    if len < 4 as SFNT_ULONG {
        pdf_warning!(b"invalid format 12 TT cmap subtable\0" as *const u8 as *const ::core::ffi::c_char,);
        return ::core::ptr::null_mut::<cmap12>();
    }
    map = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cmap12>() as usize) as uint32_t) as *mut cmap12;
    (*map).nGroups = tt_get_unsigned_quad((*sfont).handle);
    (*map).groups = new(((*map).nGroups as usize).wrapping_mul(::core::mem::size_of::<charGroup>() as usize) as uint32_t) as *mut charGroup;
    i = 0 as SFNT_ULONG;
    while i < (*map).nGroups {
        (*(*map).groups.offset(i as isize)).startCharCode = tt_get_unsigned_quad((*sfont).handle);
        (*(*map).groups.offset(i as isize)).endCharCode = tt_get_unsigned_quad((*sfont).handle);
        (*(*map).groups.offset(i as isize)).startGlyphID = tt_get_unsigned_quad((*sfont).handle);
        i = i.wrapping_add(1);
    }
    return map;
}
unsafe extern "C-unwind" fn release_cmap12(mut map: *mut cmap12) {
    if !map.is_null() {
        free((*map).groups as *mut ::core::ffi::c_void);
        free(map as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C-unwind" fn lookup_cmap12(mut map: *mut cmap12, mut cccc: SFNT_ULONG) -> USHORT {
    if (*map).nGroups == 0 {
        return 0;
    }
    let groups = core::slice::from_raw_parts((*map).groups, (*map).nGroups as usize);
    let Some(index) = groups.partition_point(|group| group.startCharCode <= cccc).checked_sub(1) else {
        return 0;
    };
    let group = &groups[index];
    if cccc > group.endCharCode {
        return 0;
    }
    cccc.wrapping_sub(group.startCharCode).wrapping_add(group.startGlyphID) as u16
}
#[unsafe(export_name = "__pdf_impl_tt_cmap_read")]
pub unsafe extern "C-unwind" fn tt_cmap_read(mut sfont: *mut sfnt, mut platform: USHORT, mut encoding: USHORT) -> *mut tt_cmap {
    let mut cmap: *mut tt_cmap = ::core::ptr::null_mut::<tt_cmap>();
    let mut offset: SFNT_ULONG = 0;
    let mut length: SFNT_ULONG = 0 as SFNT_ULONG;
    let mut p_id: USHORT = 0;
    let mut e_id: USHORT = 0;
    let mut i: USHORT = 0;
    let mut n_subtabs: USHORT = 0;
    if sfont.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"sfont\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    offset = sfnt_locate_table(sfont, b"cmap\0" as *const u8 as *const ::core::ffi::c_char);
    tt_get_unsigned_pair((*sfont).handle);
    n_subtabs = tt_get_unsigned_pair((*sfont).handle);
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < n_subtabs as ::core::ffi::c_int {
        p_id = tt_get_unsigned_pair((*sfont).handle);
        e_id = tt_get_unsigned_pair((*sfont).handle);
        if p_id as ::core::ffi::c_int != platform as ::core::ffi::c_int || e_id as ::core::ffi::c_int != encoding as ::core::ffi::c_int {
            tt_get_unsigned_quad((*sfont).handle);
            i = i.wrapping_add(1);
        } else {
            offset = offset.wrapping_add(tt_get_unsigned_quad((*sfont).handle));
            break;
        }
    }
    if i as ::core::ffi::c_int == n_subtabs as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<tt_cmap>();
    }
    cmap = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<tt_cmap>() as usize) as uint32_t) as *mut tt_cmap;
    (*cmap).map = NULL;
    (*cmap).platform = platform;
    (*cmap).encoding = encoding;
    crate::backend::runtime::input_seek((*sfont).handle, offset as ssize_t, SEEK_SET);
    (*cmap).format = tt_get_unsigned_pair((*sfont).handle);
    if (*cmap).format as ::core::ffi::c_int <= 6 as ::core::ffi::c_int {
        length = tt_get_unsigned_pair((*sfont).handle) as SFNT_ULONG;
        (*cmap).language = tt_get_unsigned_pair((*sfont).handle) as SFNT_ULONG;
    } else if tt_get_unsigned_pair((*sfont).handle) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        pdf_warning!(b"Unrecognized cmap subtable format.\0" as *const u8 as *const ::core::ffi::c_char,);
        tt_cmap_release(cmap);
        return ::core::ptr::null_mut::<tt_cmap>();
    } else {
        length = tt_get_unsigned_quad((*sfont).handle);
        (*cmap).language = tt_get_unsigned_quad((*sfont).handle);
    }
    match (*cmap).format as ::core::ffi::c_int {
        0 => {
            (*cmap).map = read_cmap0(sfont, length) as *mut ::core::ffi::c_void;
        }
        2 => {
            (*cmap).map = read_cmap2(sfont, length) as *mut ::core::ffi::c_void;
        }
        4 => {
            (*cmap).map = read_cmap4(sfont, length) as *mut ::core::ffi::c_void;
        }
        6 => {
            (*cmap).map = read_cmap6(sfont, length) as *mut ::core::ffi::c_void;
        }
        12 => {
            (*cmap).map = read_cmap12(sfont, length) as *mut ::core::ffi::c_void;
        }
        _ => {
            pdf_warning!(b"Unrecognized OpenType/TrueType cmap format.\0" as *const u8 as *const ::core::ffi::c_char,);
            tt_cmap_release(cmap);
            return ::core::ptr::null_mut::<tt_cmap>();
        }
    }
    if (*cmap).map.is_null() {
        tt_cmap_release(cmap);
        cmap = ::core::ptr::null_mut::<tt_cmap>();
    }
    return cmap;
}
#[unsafe(export_name = "__pdf_impl_tt_cmap_release")]
pub unsafe extern "C-unwind" fn tt_cmap_release(mut cmap: *mut tt_cmap) {
    if !cmap.is_null() {
        if !(*cmap).map.is_null() {
            match (*cmap).format as ::core::ffi::c_int {
                0 => {
                    release_cmap0((*cmap).map as *mut cmap0);
                }
                2 => {
                    release_cmap2((*cmap).map as *mut cmap2);
                }
                4 => {
                    release_cmap4((*cmap).map as *mut cmap4);
                }
                6 => {
                    release_cmap6((*cmap).map as *mut cmap6);
                }
                12 => {
                    release_cmap12((*cmap).map as *mut cmap12);
                }
                _ => {
                    pdf_warning!(
                        b"Unrecognized OpenType/TrueType cmap format: %d\0" as *const u8 as *const ::core::ffi::c_char,
                        (*cmap).format as ::core::ffi::c_int,
                    );
                }
            }
        }
        free(cmap as *mut ::core::ffi::c_void);
    }
}
#[unsafe(export_name = "__pdf_impl_tt_cmap_lookup")]
pub unsafe extern "C-unwind" fn tt_cmap_lookup(mut cmap: *mut tt_cmap, mut cc: SFNT_ULONG) -> USHORT {
    let mut gid: USHORT = 0 as USHORT;
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"cmap\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if cc as ::core::ffi::c_long > 0xffff as ::core::ffi::c_long && ((*cmap).format as ::core::ffi::c_int) < 12 as ::core::ffi::c_int {
        pdf_warning!(b"Four bytes charcode not supported in OpenType/TrueType cmap format 0...6.\0" as *const u8 as *const ::core::ffi::c_char,);
        return 0 as USHORT;
    }
    match (*cmap).format as ::core::ffi::c_int {
        0 => {
            gid = lookup_cmap0((*cmap).map as *mut cmap0, cc as USHORT);
        }
        2 => {
            gid = lookup_cmap2((*cmap).map as *mut cmap2, cc as USHORT);
        }
        4 => {
            gid = lookup_cmap4((*cmap).map as *mut cmap4, cc as USHORT);
        }
        6 => {
            gid = lookup_cmap6((*cmap).map as *mut cmap6, cc as USHORT);
        }
        12 => {
            gid = lookup_cmap12((*cmap).map as *mut cmap12, cc);
        }
        _ => {
            pdf_warning!(
                b"Unrecognized OpenType/TrueType cmap subtable format: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*cmap).format as ::core::ffi::c_int,
            );
        }
    }
    return gid;
}
const SRANGE_MIN: [::core::ffi::c_uchar; 2] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
const SRANGE_MAX: [::core::ffi::c_uchar; 2] = [
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
const LRANGE_MIN: [::core::ffi::c_uchar; 4] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
const LRANGE_MAX: [::core::ffi::c_uchar; 4] = [
    0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
pub const FONTTYPE_CIDFONT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn create_GIDToCIDMap(mut GIDToCIDMap: *mut uint16_t, mut num_glyphs: uint16_t, mut cffont: *mut cff_font) {
    let mut charset: *mut cff_charsets = ::core::ptr::null_mut::<cff_charsets>();
    let mut gid: uint16_t = 0;
    let mut i: uint16_t = 0;
    if GIDToCIDMap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"GIDToCIDMap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if cffont.is_null() || (*cffont).flag & FONTTYPE_CIDFONT == 0 {
        gid = 0 as uint16_t;
        while (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
            *GIDToCIDMap.offset(gid as isize) = gid;
            gid = gid.wrapping_add(1);
        }
        return;
    }
    memset(
        GIDToCIDMap as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (num_glyphs as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
    );
    charset = (*cffont).charsets;
    if charset.is_null() {
        return;
    }
    match (*charset).format as ::core::ffi::c_int {
        0 => {
            let mut cids: *mut s_SID = ::core::ptr::null_mut::<s_SID>();
            cids = (*charset).data.glyphs;
            gid = 1 as uint16_t;
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                *GIDToCIDMap.offset(gid as isize) = *cids.offset(i as isize) as uint16_t;
                gid = gid.wrapping_add(1);
                i = i.wrapping_add(1);
            }
        }
        1 => {
            let mut ranges: *mut cff_range1 = ::core::ptr::null_mut::<cff_range1>();
            let mut cid: card16 = 0;
            let mut count: card16 = 0;
            ranges = (*charset).data.range1;
            gid = 1 as uint16_t;
            i = 0 as uint16_t;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                cid = (*ranges.offset(i as isize)).first as card16;
                count = ((*ranges.offset(i as isize)).n_left as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as card16;
                loop {
                    let fresh7 = count;
                    count = count.wrapping_sub(1);
                    if !(fresh7 as ::core::ffi::c_int > 0 as ::core::ffi::c_int && gid as ::core::ffi::c_int <= num_glyphs as ::core::ffi::c_int) {
                        break;
                    }
                    *GIDToCIDMap.offset(gid as isize) = cid as uint16_t;
                    gid = gid.wrapping_add(1);
                    cid = cid.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
        }
        2 => {
            let mut ranges_0: *mut cff_range2 = ::core::ptr::null_mut::<cff_range2>();
            let mut cid_0: card16 = 0;
            let mut count_0: card16 = 0;
            ranges_0 = (*charset).data.range2;
            if (*charset).num_entries as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                && (*ranges_0.offset(0 as ::core::ffi::c_int as isize)).first as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            {
                gid = 0 as uint16_t;
                while (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
                    *GIDToCIDMap.offset(gid as isize) = gid;
                    gid = gid.wrapping_add(1);
                }
            } else {
                gid = 1 as uint16_t;
                i = 0 as uint16_t;
                while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                    cid_0 = (*ranges_0.offset(i as isize)).first as card16;
                    count_0 = ((*ranges_0.offset(i as isize)).n_left as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as card16;
                    loop {
                        let fresh8 = count_0;
                        count_0 = count_0.wrapping_sub(1);
                        if !(fresh8 as ::core::ffi::c_int > 0 as ::core::ffi::c_int && gid as ::core::ffi::c_int <= num_glyphs as ::core::ffi::c_int)
                        {
                            break;
                        }
                        *GIDToCIDMap.offset(gid as isize) = cid_0 as uint16_t;
                        gid = gid.wrapping_add(1);
                        cid_0 = cid_0.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
            }
        }
        _ => {
            pdf_warning!(
                b"Unknown CFF charset format...: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*charset).format as ::core::ffi::c_int,
            );
        }
    };
}
extern "C-unwind" fn is_PUA_or_presentation(mut uni: ::core::ffi::c_uint) -> bool {
    return uni >= 0x2e80 as ::core::ffi::c_uint && uni <= 0x2ef3 as ::core::ffi::c_uint
        || uni >= 0x2f00 as ::core::ffi::c_uint && uni <= 0x2fd5 as ::core::ffi::c_uint
        || uni >= 0xe000 as ::core::ffi::c_uint && uni <= 0xf8ff as ::core::ffi::c_uint
        || uni >= 0xfb00 as ::core::ffi::c_uint && uni <= 0xfb4f as ::core::ffi::c_uint
        || uni >= 0xf900 as ::core::ffi::c_uint && uni <= 0xfaff as ::core::ffi::c_uint
        || uni >= 0x2f800 as ::core::ffi::c_int as ::core::ffi::c_uint && uni <= 0x2fa1f as ::core::ffi::c_int as ::core::ffi::c_uint
        || uni >= 0xf0000 as ::core::ffi::c_int as ::core::ffi::c_uint && uni <= 0xffffd as ::core::ffi::c_int as ::core::ffi::c_uint
        || uni >= 0x100000 as ::core::ffi::c_int as ::core::ffi::c_uint && uni <= 0x10fffd as ::core::ffi::c_int as ::core::ffi::c_uint
        || uni == 0xad as ::core::ffi::c_uint;
}
unsafe extern "C-unwind" fn lookup_glyph_name(mut post: *mut tt_post_table, mut cffont: *mut cff_font, mut gid: USHORT) -> *mut ::core::ffi::c_char {
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !post.is_null() {
        name = tt_get_glyphname(post, gid);
    }
    if name.is_null() && !cffont.is_null() {
        name = cff_get_glyphname(cffont, gid as card16);
    }
    return name;
}
unsafe extern "C-unwind" fn handle_subst_glyphs(mut cmap: *mut CMap, mut cmap_add: *mut CMap, mut used_chars: *mut ::core::ffi::c_char) -> int32_t {
    let mut count: int32_t = 0 as int32_t;
    let mut cid: int32_t = 0;
    cid = 0 as ::core::ffi::c_int as int32_t;
    while cid < 65536 as int32_t {
        if !(*used_chars.offset((cid / 8 as int32_t) as isize) as ::core::ffi::c_int & (1 as ::core::ffi::c_int) << 7 as int32_t - cid % 8 as int32_t
            == 0)
        {
            let mut buf: [::core::ffi::c_uchar; 256] = [0; 256];
            let mut inbytesleft: size_t = 2 as size_t;
            let mut outbytesleft: size_t = 254 as size_t;
            let mut len: size_t = 0;
            let mut outbuf: *mut ::core::ffi::c_uchar = (&raw mut buf as *mut ::core::ffi::c_uchar).offset(2 as ::core::ffi::c_int as isize);
            let mut inbuf: *const ::core::ffi::c_uchar = &raw mut buf as *mut ::core::ffi::c_uchar;
            buf[0 as ::core::ffi::c_int as usize] = (cid >> 8 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
            buf[1 as ::core::ffi::c_int as usize] = (cid & 0xff as int32_t) as ::core::ffi::c_uchar;
            CMap_decode(cmap_add, &raw mut inbuf, &raw mut inbytesleft, &raw mut outbuf, &raw mut outbytesleft);
            if inbytesleft == 0 as size_t {
                len = (254 as size_t).wrapping_sub(outbytesleft);
                CMap_add_bfchar(
                    cmap,
                    &raw mut buf as *mut ::core::ffi::c_uchar,
                    2 as size_t,
                    (&raw mut buf as *mut ::core::ffi::c_uchar).offset(2 as ::core::ffi::c_int as isize),
                    len,
                );
                let ref mut fresh6 = *used_chars.offset((cid / 8 as int32_t) as isize);
                *fresh6 = (*fresh6 as ::core::ffi::c_int & !((1 as ::core::ffi::c_int) << 7 as int32_t - cid % 8 as int32_t)) as ::core::ffi::c_char;
                count += 1;
            }
        }
        cid += 1;
    }
    return count;
}
unsafe extern "C-unwind" fn add_ToUnicode_via_glyph_name(
    mut cmap: *mut CMap, mut used_chars: *mut ::core::ffi::c_char, mut num_glyphs: USHORT, mut GIDToCIDMap: *mut uint16_t, mut sfont: *mut sfnt,
    mut cffont: *mut cff_font,
) -> int32_t {
    let mut count: int32_t = 0 as int32_t;
    let mut gid: USHORT = 0;
    let mut post: *mut tt_post_table = ::core::ptr::null_mut::<tt_post_table>();
    post = tt_read_post_table(sfont);
    if post.is_null() && cffont.is_null() {
        return count;
    }
    gid = 0 as USHORT;
    while (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
        let mut cid: uint16_t = *GIDToCIDMap.offset(gid as isize);
        if *used_chars.offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
            != 0
        {
            let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut unicodes: [int32_t; 32] = [0; 32];
            let mut unicode_count: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            name = lookup_glyph_name(post, cffont, gid);
            if !name.is_null() {
                unicode_count = agl_get_unicodes(name, &raw mut unicodes as *mut int32_t, MAX_UNICODES);
                free(name as *mut ::core::ffi::c_void);
                if unicode_count > 0 as ::core::ffi::c_int {
                    let mut buf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    let mut endptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                    let mut k: ::core::ffi::c_int = 0;
                    let mut len: size_t = 0 as size_t;
                    buf = new(((unicode_count * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint32_t as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize) as uint32_t)
                        as *mut ::core::ffi::c_uchar;
                    p = buf.offset(2 as ::core::ffi::c_int as isize);
                    endptr = buf.offset((unicode_count * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize);
                    k = 0 as ::core::ffi::c_int;
                    while k < unicode_count {
                        len = len.wrapping_add(UC_UTF16BE_encode_char(unicodes[k as usize], &raw mut p, endptr));
                        k += 1;
                    }
                    *buf.offset(0 as ::core::ffi::c_int as isize) =
                        (cid as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    *buf.offset(1 as ::core::ffi::c_int as isize) = (cid as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                    CMap_add_bfchar(cmap, buf, 2 as size_t, buf.offset(2 as ::core::ffi::c_int as isize), len);
                    let ref mut fresh5 = *used_chars.offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
                    *fresh5 = (*fresh5 as ::core::ffi::c_int
                        & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int))
                        as ::core::ffi::c_char;
                    count += 1;
                    free(buf as *mut ::core::ffi::c_void);
                }
            }
        }
        gid = gid.wrapping_add(1);
    }
    if !post.is_null() {
        tt_release_post_table(post);
    }
    return count;
}
pub const MAX_UNICODES: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn create_inverse_cmap4(mut map_base: *mut int32_t, mut map_sub: *mut int32_t, mut num_glyphs: USHORT, mut map: *mut cmap4) {
    let mut segCount: USHORT = ((*map).segCountX2 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as USHORT;
    let mut i: USHORT = 0;
    let mut j: USHORT = 0;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < segCount as ::core::ffi::c_int {
        let mut c0: USHORT = *(*map).startCount.offset(i as isize);
        let mut c1: USHORT = *(*map).endCount.offset(i as isize);
        let mut d: USHORT = (*(*map).idRangeOffset.offset(i as isize) as ::core::ffi::c_int / 2 as ::core::ffi::c_int
            - (segCount as ::core::ffi::c_int - i as ::core::ffi::c_int)) as USHORT;
        j = 0 as USHORT;
        while j as ::core::ffi::c_int <= c1 as ::core::ffi::c_int - c0 as ::core::ffi::c_int {
            let mut ch: USHORT = (c0 as ::core::ffi::c_int + j as ::core::ffi::c_int) as USHORT;
            let mut gid: USHORT = 0;
            if *(*map).idRangeOffset.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                gid = (ch as ::core::ffi::c_int + *(*map).idDelta.offset(i as isize) as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int) as USHORT;
            } else if c0 as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
                && c1 as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
                && *(*map).idRangeOffset.offset(i as isize) as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
            {
                gid = 0 as USHORT;
            } else {
                gid = (*(*map)
                    .glyphIndexArray
                    .offset((j as ::core::ffi::c_int + d as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *(*map).idDelta.offset(i as isize) as ::core::ffi::c_int
                    & 0xffff as ::core::ffi::c_int) as USHORT;
            }
            if is_PUA_or_presentation(ch as ::core::ffi::c_uint) {
                *map_sub.offset(gid as isize) = ch as int32_t;
            } else {
                *map_base.offset(gid as isize) = ch as int32_t;
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C-unwind" fn create_inverse_cmap12(
    mut map_base: *mut int32_t, mut map_sub: *mut int32_t, mut num_glyphs: USHORT, mut map: *mut cmap12,
) {
    let mut i: SFNT_ULONG = 0;
    let mut ch: SFNT_ULONG = 0;
    i = 0 as SFNT_ULONG;
    while i < (*map).nGroups {
        ch = (*(*map).groups.offset(i as isize)).startCharCode;
        while ch <= (*(*map).groups.offset(i as isize)).endCharCode {
            let mut d: ::core::ffi::c_int = ch.wrapping_sub((*(*map).groups.offset(i as isize)).startCharCode) as ::core::ffi::c_int;
            let mut gid: USHORT = ((*(*map).groups.offset(i as isize)).startGlyphID.wrapping_add(d as SFNT_ULONG) & 0xffff as SFNT_ULONG) as USHORT;
            if is_PUA_or_presentation(ch as ::core::ffi::c_uint) {
                *map_sub.offset(gid as isize) = ch as int32_t;
            } else {
                *map_base.offset(gid as isize) = ch as int32_t;
            }
            ch = ch.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C-unwind" fn create_ToUnicode_cmap(
    mut ttcmap: *mut tt_cmap, mut cmap_name: *const ::core::ffi::c_char, mut cmap_add: *mut CMap, mut used_chars: *const ::core::ffi::c_char,
    mut sfont: *mut sfnt,
) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut map_base: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
    let mut map_sub: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
    let mut gid: USHORT = 0;
    let mut num_glyphs: USHORT = 0 as USHORT;
    if ttcmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"ttcmap\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    let mut maxp: *mut tt_maxp_table = ::core::ptr::null_mut::<tt_maxp_table>();
    maxp = tt_read_maxp_table(sfont);
    if !maxp.is_null() {
        num_glyphs = (*maxp).numGlyphs;
        free(maxp as *mut ::core::ffi::c_void);
    }
    map_base = new((num_glyphs as uint32_t as usize).wrapping_mul(::core::mem::size_of::<int32_t>() as usize) as uint32_t) as *mut int32_t;
    map_sub = new((num_glyphs as uint32_t as usize).wrapping_mul(::core::mem::size_of::<int32_t>() as usize) as uint32_t) as *mut int32_t;
    gid = 0 as USHORT;
    while (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
        *map_base.offset(gid as isize) = -(1 as ::core::ffi::c_int) as int32_t;
        *map_sub.offset(gid as isize) = -(1 as ::core::ffi::c_int) as int32_t;
        gid = gid.wrapping_add(1);
    }
    match (*ttcmap).format as ::core::ffi::c_int {
        4 => {
            create_inverse_cmap4(map_base, map_sub, num_glyphs, (*ttcmap).map as *mut cmap4);
        }
        12 => {
            create_inverse_cmap12(map_base, map_sub, num_glyphs, (*ttcmap).map as *mut cmap12);
        }
        _ => {}
    }
    let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
    let mut count: int32_t = 0;
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut is_cidfont: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
    let mut GIDToCIDMap: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
    let mut used_chars_copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*sfont).type_0 == SFNT_TYPE_POSTSCRIPT {
        let mut offset: SFNT_ULONG = 0;
        offset = sfnt_find_table_pos(sfont, b"CFF \0" as *const u8 as *const ::core::ffi::c_char);
        if offset == 0 as SFNT_ULONG {
            pdf_abort!(b"\"CFF \" table not found. Must be found before... Can't continue.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        cffont = cff_open((*sfont).handle, offset as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        cff_read_charsets(cffont);
    }
    is_cidfont = (!cffont.is_null() && (*cffont).flag & FONTTYPE_CIDFONT != 0) as ::core::ffi::c_int as ::core::ffi::c_char;
    GIDToCIDMap = new((num_glyphs as uint32_t as usize).wrapping_mul(::core::mem::size_of::<uint16_t>() as usize) as uint32_t) as *mut uint16_t;
    if is_cidfont != 0 {
        create_GIDToCIDMap(GIDToCIDMap, num_glyphs as uint16_t, cffont);
    } else {
        gid = 0 as USHORT;
        while (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
            *GIDToCIDMap.offset(gid as isize) = gid as uint16_t;
            gid = gid.wrapping_add(1);
        }
    }
    cmap = CMap_new();
    CMap_set_name(cmap, cmap_name);
    CMap_set_wmode(cmap, 0 as ::core::ffi::c_int);
    CMap_set_type(cmap, CMAP_TYPE_TO_UNICODE);
    CMap_set_CIDSysInfo(cmap, &raw mut CSI_UNICODE);
    CMap_add_codespacerange(cmap, SRANGE_MIN.as_ptr(), SRANGE_MAX.as_ptr(), 2 as size_t);
    count = 0 as ::core::ffi::c_int as int32_t;
    used_chars_copy =
        new((8192 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
            as *mut ::core::ffi::c_char;
    memcpy(
        used_chars_copy as *mut ::core::ffi::c_void,
        used_chars as *const ::core::ffi::c_void,
        8192 as size_t,
    );
    gid = 0 as USHORT;
    while (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
        let mut cid: uint16_t = *GIDToCIDMap.offset(gid as isize);
        if *used_chars_copy.offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
            != 0
        {
            let mut ch: int32_t = 0;
            let mut src: [::core::ffi::c_uchar; 2] = [0; 2];
            let mut dst: [::core::ffi::c_uchar; 4] = [0; 4];
            let mut p: *mut ::core::ffi::c_uchar = &raw mut dst as *mut ::core::ffi::c_uchar;
            let mut endptr: *mut ::core::ffi::c_uchar = (&raw mut dst as *mut ::core::ffi::c_uchar).offset(4 as ::core::ffi::c_int as isize);
            let mut len: size_t = 0;
            ch = *map_base.offset(gid as isize);
            if UC_is_valid(ch) {
                src[0 as ::core::ffi::c_int as usize] =
                    (cid as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                src[1 as ::core::ffi::c_int as usize] = (cid as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                len = UC_UTF16BE_encode_char(ch, &raw mut p, endptr);
                CMap_add_bfchar(
                    cmap,
                    &raw mut src as *mut ::core::ffi::c_uchar,
                    2 as size_t,
                    &raw mut dst as *mut ::core::ffi::c_uchar,
                    len,
                );
                let ref mut fresh3 = *used_chars_copy.offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
                *fresh3 = (*fresh3 as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int))
                    as ::core::ffi::c_char;
                count += 1;
            }
        }
        gid = gid.wrapping_add(1);
    }
    if !cmap_add.is_null() {
        count += handle_subst_glyphs(cmap, cmap_add, used_chars_copy);
    } else {
        count = (count as ::core::ffi::c_int + otl_gsub_add_ToUnicode(cmap, used_chars_copy, map_base, map_sub, num_glyphs, GIDToCIDMap, sfont))
            as int32_t;
    }
    count += add_ToUnicode_via_glyph_name(
        cmap,
        used_chars_copy,
        num_glyphs,
        GIDToCIDMap,
        sfont,
        if is_cidfont as ::core::ffi::c_int != 0 {
            ::core::ptr::null_mut::<cff_font>()
        } else {
            cffont
        },
    );
    if !cffont.is_null() {
        cff_close(cffont);
    }
    gid = 0 as USHORT;
    while (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
        let mut cid_0: uint16_t = *GIDToCIDMap.offset(gid as isize);
        if *used_chars_copy.offset((cid_0 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid_0 as ::core::ffi::c_int % 8 as ::core::ffi::c_int
            != 0
        {
            let mut ch_0: int32_t = 0;
            let mut src_0: [::core::ffi::c_uchar; 2] = [0; 2];
            let mut dst_0: [::core::ffi::c_uchar; 4] = [0; 4];
            let mut p_0: *mut ::core::ffi::c_uchar = &raw mut dst_0 as *mut ::core::ffi::c_uchar;
            let mut endptr_0: *mut ::core::ffi::c_uchar = (&raw mut dst_0 as *mut ::core::ffi::c_uchar).offset(4 as ::core::ffi::c_int as isize);
            let mut len_0: size_t = 0;
            ch_0 = *map_sub.offset(gid as isize);
            if UC_is_valid(ch_0) {
                src_0[0 as ::core::ffi::c_int as usize] =
                    (cid_0 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                src_0[1 as ::core::ffi::c_int as usize] = (cid_0 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                len_0 = UC_UTF16BE_encode_char(ch_0, &raw mut p_0, endptr_0);
                CMap_add_bfchar(
                    cmap,
                    &raw mut src_0 as *mut ::core::ffi::c_uchar,
                    2 as size_t,
                    &raw mut dst_0 as *mut ::core::ffi::c_uchar,
                    len_0,
                );
                let ref mut fresh4 = *used_chars_copy.offset((cid_0 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
                *fresh4 = (*fresh4 as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid_0 as ::core::ffi::c_int % 8 as ::core::ffi::c_int))
                    as ::core::ffi::c_char;
                count += 1;
            }
        }
        gid = gid.wrapping_add(1);
    }
    if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
        gid = 0 as USHORT;
        while (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
            let mut cid_1: uint16_t = *GIDToCIDMap.offset(gid as isize);
            if *used_chars_copy.offset((cid_1 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid_1 as ::core::ffi::c_int % 8 as ::core::ffi::c_int
                != 0
            {
                pdf_warning!(
                    b"Unable to find ToUnicode mapping for glyph CID=%u (GID=%u)\0" as *const u8 as *const ::core::ffi::c_char,
                    cid_1 as ::core::ffi::c_int,
                    gid as ::core::ffi::c_int,
                );
            }
            gid = gid.wrapping_add(1);
        }
    }
    free(GIDToCIDMap as *mut ::core::ffi::c_void);
    free(used_chars_copy as *mut ::core::ffi::c_void);
    if count < 1 as int32_t {
        stream = ::core::ptr::null_mut::<pdf_obj>();
    } else {
        stream = CMap_create_stream(cmap);
    }
    CMap_release(cmap);
    free(map_base as *mut ::core::ffi::c_void);
    free(map_sub as *mut ::core::ffi::c_void);
    return stream;
}
const CMAP_PLAT_ENCS: [cmap_plat_enc_rec; 6] = [
    cmap_plat_enc_rec {
        platform: 3 as ::core::ffi::c_short,
        encoding: 10 as ::core::ffi::c_short,
    },
    cmap_plat_enc_rec {
        platform: 0 as ::core::ffi::c_short,
        encoding: 3 as ::core::ffi::c_short,
    },
    cmap_plat_enc_rec {
        platform: 0 as ::core::ffi::c_short,
        encoding: 4 as ::core::ffi::c_short,
    },
    cmap_plat_enc_rec {
        platform: 0 as ::core::ffi::c_short,
        encoding: 0 as ::core::ffi::c_short,
    },
    cmap_plat_enc_rec {
        platform: 3 as ::core::ffi::c_short,
        encoding: 1 as ::core::ffi::c_short,
    },
    cmap_plat_enc_rec {
        platform: 0 as ::core::ffi::c_short,
        encoding: 1 as ::core::ffi::c_short,
    },
];
#[unsafe(export_name = "__pdf_impl_otf_create_ToUnicode_stream")]
pub unsafe extern "C-unwind" fn otf_create_ToUnicode_stream(
    mut font_name: *const ::core::ffi::c_char, mut ttc_index: uint32_t, mut basefont: *const ::core::ffi::c_char,
    mut used_chars: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut cmap_ref: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cmap_add: *mut CMap = ::core::ptr::null_mut::<CMap>();
    let mut cmap_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut offset: SFNT_ULONG = 0 as SFNT_ULONG;
    let mut ttcmap: *mut tt_cmap = ::core::ptr::null_mut::<tt_cmap>();
    let mut cmap_id: ::core::ffi::c_int = 0;
    let mut cmap_add_id: ::core::ffi::c_int = 0;
    let mut i: size_t = 0;
    cmap_name = new((strlen(basefont)
        .wrapping_add(strlen(b"-UTF16\0" as *const u8 as *const ::core::ffi::c_char))
        .wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    sprintf(cmap_name, b"%s-UTF16\0" as *const u8 as *const ::core::ffi::c_char, basefont);
    cmap_id = pdf_findresource(b"CMap\0" as *const u8 as *const ::core::ffi::c_char, cmap_name);
    if cmap_id >= 0 as ::core::ffi::c_int {
        free(cmap_name as *mut ::core::ffi::c_void);
        cmap_ref = pdf_get_resource_reference(cmap_id);
        return cmap_ref;
    }
    if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
        pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdf_message!(
            b"otf_cmap>> Creating ToUnicode CMap for \"%s\"...\n\0" as *const u8 as *const ::core::ffi::c_char,
            font_name,
        );
    }
    handle = dpx_open_truetype_file(font_name);
    if handle != 0 || {
        handle = dpx_open_opentype_file(font_name);
        handle != 0
    } {
        sfont = sfnt_open(handle);
    } else {
        handle = dpx_open_dfont_file(font_name);
        if handle != 0 {
            sfont = dfont_open(handle, ttc_index as ::core::ffi::c_int);
        } else {
            free(cmap_name as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<pdf_obj>();
        }
    }
    if sfont.is_null() {
        pdf_warning!(
            b"Could not open OpenType/TrueType font file \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            font_name,
        );
        free(cmap_name as *mut ::core::ffi::c_void);
        crate::backend::runtime::input_close(handle);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    match (*sfont).type_0 {
        SFNT_TYPE_DFONT => {
            offset = (*sfont).offset;
        }
        SFNT_TYPE_TTC => {
            offset = ttc_read_offset(sfont, ttc_index as SFNT_ULONG);
            if offset == 0 as SFNT_ULONG {
                pdf_warning!(b"Invalid TTC index for font: %s\0" as *const u8 as *const ::core::ffi::c_char, font_name,);
                sfnt_close(sfont);
                crate::backend::runtime::input_close(handle);
                free(cmap_name as *mut ::core::ffi::c_void);
                return ::core::ptr::null_mut::<pdf_obj>();
            }
        }
        _ => {
            offset = 0 as SFNT_ULONG;
        }
    }
    if sfnt_read_table_directory(sfont, offset) < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Could not read OpenType/TrueType table directory for font: %s\0" as *const u8 as *const ::core::ffi::c_char,
            font_name,
        );
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        free(cmap_name as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    let mut cmap_add_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    len = strlen(font_name)
        .wrapping_add(strlen(b"-UCS32-Add\0" as *const u8 as *const ::core::ffi::c_char))
        .wrapping_add(32 as size_t);
    cmap_add_name =
        new((len as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    snprintf(
        cmap_add_name,
        len,
        b"%s:%d-UCS32-Add\0" as *const u8 as *const ::core::ffi::c_char,
        font_name,
        ttc_index,
    );
    *cmap_add_name.offset(len.wrapping_sub(1 as size_t) as isize) = '\0' as i32 as ::core::ffi::c_char;
    cmap_add_id = CMap_cache_find(cmap_add_name);
    free(cmap_add_name as *mut ::core::ffi::c_void);
    if cmap_add_id < 0 as ::core::ffi::c_int {
        cmap_add = ::core::ptr::null_mut::<CMap>();
    } else {
        cmap_add = CMap_cache_get(cmap_add_id);
    }
    ttcmap = ::core::ptr::null_mut::<tt_cmap>();
    i = 0 as size_t;
    while i < (::core::mem::size_of::<[cmap_plat_enc_rec; 6]>() as usize).wrapping_div(::core::mem::size_of::<cmap_plat_enc_rec>() as usize) {
        ttcmap = tt_cmap_read(
            sfont,
            CMAP_PLAT_ENCS[i as usize].platform as USHORT,
            CMAP_PLAT_ENCS[i as usize].encoding as USHORT,
        );
        if !ttcmap.is_null() {
            if (*ttcmap).format as ::core::ffi::c_int == 4 as ::core::ffi::c_int || (*ttcmap).format as ::core::ffi::c_int == 12 as ::core::ffi::c_int
            {
                break;
            }
            tt_cmap_release(ttcmap);
            ttcmap = ::core::ptr::null_mut::<tt_cmap>();
        }
        i = i.wrapping_add(1);
    }
    if !ttcmap.is_null() {
        let mut cmap_obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        CMap_set_silent(1 as ::core::ffi::c_int);
        cmap_obj = create_ToUnicode_cmap(ttcmap, cmap_name, cmap_add, used_chars, sfont);
        CMap_set_silent(0 as ::core::ffi::c_int);
        if !cmap_obj.is_null() {
            cmap_id = pdf_defineresource(
                b"CMap\0" as *const u8 as *const ::core::ffi::c_char,
                cmap_name,
                cmap_obj,
                PDF_RES_FLUSH_IMMEDIATE,
            );
            cmap_ref = pdf_get_resource_reference(cmap_id);
        }
        tt_cmap_release(ttcmap);
    }
    free(cmap_name as *mut ::core::ffi::c_void);
    sfnt_close(sfont);
    if handle != 0 {
        crate::backend::runtime::input_close(handle);
    }
    if cmap_ref.is_null() {
        pdf_warning!(
            b"Creating ToUnicode CMap failed for \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            font_name,
        );
    }
    return cmap_ref;
}
unsafe extern "C-unwind" fn load_cmap4(
    mut map: *mut cmap4, mut GIDToCIDMap: *mut uint16_t, mut num_glyphs: USHORT, mut gsub_vert: *mut otl_gsub, mut gsub_list: *mut otl_gsub,
    mut cmap: *mut CMap, mut map_base: *mut int32_t, mut map_sub: *mut int32_t,
) {
    let mut c0: USHORT = 0;
    let mut c1: USHORT = 0;
    let mut gid: USHORT = 0;
    let mut cid: USHORT = 0;
    let mut j: USHORT = 0;
    let mut d: USHORT = 0;
    let mut segCount: USHORT = 0;
    let mut ch: USHORT = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_uchar; 4] = [0; 4];
    segCount = ((*map).segCountX2 as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as USHORT;
    i = segCount as ::core::ffi::c_int - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        c0 = *(*map).startCount.offset(i as isize);
        c1 = *(*map).endCount.offset(i as isize);
        d = (*(*map).idRangeOffset.offset(i as isize) as ::core::ffi::c_int / 2 as ::core::ffi::c_int - (segCount as ::core::ffi::c_int - i))
            as USHORT;
        j = 0 as USHORT;
        while j as ::core::ffi::c_int <= c1 as ::core::ffi::c_int - c0 as ::core::ffi::c_int {
            ch = (c0 as ::core::ffi::c_int + j as ::core::ffi::c_int) as USHORT;
            if *(*map).idRangeOffset.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                gid = (ch as ::core::ffi::c_int + *(*map).idDelta.offset(i as isize) as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int) as USHORT;
            } else if c0 as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
                && c1 as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
                && *(*map).idRangeOffset.offset(i as isize) as ::core::ffi::c_int == 0xffff as ::core::ffi::c_int
            {
                gid = 0 as USHORT;
            } else {
                gid = (*(*map)
                    .glyphIndexArray
                    .offset((j as ::core::ffi::c_int + d as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *(*map).idDelta.offset(i as isize) as ::core::ffi::c_int
                    & 0xffff as ::core::ffi::c_int) as USHORT;
            }
            if gid as ::core::ffi::c_int != 0 as ::core::ffi::c_int && gid as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int {
                if !gsub_list.is_null() {
                    otl_gsub_apply_chain(gsub_list, &raw mut gid);
                }
                if !gsub_vert.is_null() {
                    otl_gsub_apply(gsub_vert, &raw mut gid);
                }
                cid = (if (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
                    *GIDToCIDMap.offset(gid as isize) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as USHORT;
                buf[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
                buf[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
                buf[2 as ::core::ffi::c_int as usize] =
                    (ch as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                buf[3 as ::core::ffi::c_int as usize] = (ch as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                CMap_add_cidchar(cmap, &raw mut buf as *mut ::core::ffi::c_uchar, 4 as size_t, cid as CID);
                if !map_base.is_null() && !map_sub.is_null() {
                    if is_PUA_or_presentation(ch as ::core::ffi::c_uint) {
                        *map_sub.offset(gid as isize) = ch as int32_t;
                    } else {
                        *map_base.offset(gid as isize) = ch as int32_t;
                    }
                }
            }
            j = j.wrapping_add(1);
        }
        i -= 1;
    }
}
unsafe extern "C-unwind" fn load_cmap12(
    mut map: *mut cmap12, mut GIDToCIDMap: *mut uint16_t, mut num_glyphs: USHORT, mut gsub_vert: *mut otl_gsub, mut gsub_list: *mut otl_gsub,
    mut cmap: *mut CMap, mut map_base: *mut int32_t, mut map_sub: *mut int32_t,
) {
    let mut i: SFNT_ULONG = 0;
    let mut ch: SFNT_ULONG = 0;
    let mut gid: USHORT = 0;
    let mut cid: USHORT = 0;
    let mut buf: [::core::ffi::c_uchar; 4] = [0; 4];
    i = 0 as SFNT_ULONG;
    while i < (*map).nGroups {
        ch = (*(*map).groups.offset(i as isize)).startCharCode;
        while ch <= (*(*map).groups.offset(i as isize)).endCharCode {
            let mut d: ::core::ffi::c_int = ch.wrapping_sub((*(*map).groups.offset(i as isize)).startCharCode) as ::core::ffi::c_int;
            gid = ((*(*map).groups.offset(i as isize)).startGlyphID.wrapping_add(d as SFNT_ULONG) & 0xffff as SFNT_ULONG) as USHORT;
            if !gsub_list.is_null() {
                otl_gsub_apply_chain(gsub_list, &raw mut gid);
            }
            if !gsub_vert.is_null() {
                otl_gsub_apply(gsub_vert, &raw mut gid);
            }
            cid = (if (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
                *GIDToCIDMap.offset(gid as isize) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as USHORT;
            buf[0 as ::core::ffi::c_int as usize] = (ch >> 24 as ::core::ffi::c_int & 0xff as SFNT_ULONG) as ::core::ffi::c_uchar;
            buf[1 as ::core::ffi::c_int as usize] = (ch >> 16 as ::core::ffi::c_int & 0xff as SFNT_ULONG) as ::core::ffi::c_uchar;
            buf[2 as ::core::ffi::c_int as usize] = (ch >> 8 as ::core::ffi::c_int & 0xff as SFNT_ULONG) as ::core::ffi::c_uchar;
            buf[3 as ::core::ffi::c_int as usize] = (ch & 0xff as SFNT_ULONG) as ::core::ffi::c_uchar;
            CMap_add_cidchar(cmap, &raw mut buf as *mut ::core::ffi::c_uchar, 4 as size_t, cid as CID);
            if !map_base.is_null() && !map_sub.is_null() {
                if is_PUA_or_presentation(ch as ::core::ffi::c_uint) {
                    *map_sub.offset(gid as isize) = ch as int32_t;
                } else {
                    *map_base.offset(gid as isize) = ch as int32_t;
                }
            }
            ch = ch.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
#[unsafe(export_name = "__pdf_impl_otf_load_Unicode_CMap")]
pub unsafe extern "C-unwind" fn otf_load_Unicode_CMap(
    mut map_name: *const ::core::ffi::c_char, mut ttc_index: uint32_t, mut otl_tags: *const ::core::ffi::c_char, mut wmode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cmap_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut cmap_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut offset: SFNT_ULONG = 0 as SFNT_ULONG;
    let mut num_glyphs: uint16_t = 0 as uint16_t;
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    let mut ttcmap: *mut tt_cmap = ::core::ptr::null_mut::<tt_cmap>();
    let mut csi: CIDSysInfo = CIDSysInfo {
        registry: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ordering: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        supplement: 0 as ::core::ffi::c_int,
    };
    let mut GIDToCIDMap: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
    if map_name.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !otl_tags.is_null() {
        let mut len: size_t = 0;
        len = strlen(map_name)
            .wrapping_add(strlen(b"-UCS4-H\0" as *const u8 as *const ::core::ffi::c_char))
            .wrapping_add(strlen(otl_tags))
            .wrapping_add(32 as size_t);
        cmap_name = new((len as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
            as *mut ::core::ffi::c_char;
        if wmode != 0 {
            snprintf(
                cmap_name,
                len,
                b"%s,%03d,%s-UCS4-V\0" as *const u8 as *const ::core::ffi::c_char,
                map_name,
                ttc_index,
                otl_tags,
            );
        } else {
            snprintf(
                cmap_name,
                len,
                b"%s,%03d,%s-UCS4-H\0" as *const u8 as *const ::core::ffi::c_char,
                map_name,
                ttc_index,
                otl_tags,
            );
        }
        *cmap_name.offset(len.wrapping_sub(1 as size_t) as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else {
        let mut len_0: size_t = 0;
        len_0 = strlen(map_name)
            .wrapping_add(strlen(b"-UCS4-H\0" as *const u8 as *const ::core::ffi::c_char))
            .wrapping_add(32 as size_t);
        cmap_name = new((len_0 as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
            as *mut ::core::ffi::c_char;
        if wmode != 0 {
            snprintf(
                cmap_name,
                len_0,
                b"%s,%03d-UCS4-V\0" as *const u8 as *const ::core::ffi::c_char,
                map_name,
                ttc_index,
            );
        } else {
            snprintf(
                cmap_name,
                len_0,
                b"%s,%03d-UCS4-H\0" as *const u8 as *const ::core::ffi::c_char,
                map_name,
                ttc_index,
            );
        }
        *cmap_name.offset(len_0.wrapping_sub(1 as size_t) as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    cmap_id = CMap_cache_find(cmap_name);
    if cmap_id >= 0 as ::core::ffi::c_int {
        free(cmap_name as *mut ::core::ffi::c_void);
        if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
            pdf_message!(b"otf_cmap>> Found at cmap_id=%d.\n\0" as *const u8 as *const ::core::ffi::c_char, cmap_id,);
        }
        return cmap_id;
    }
    if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
        pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdf_message!(
            b"otf_cmap>> Creating Unicode charmap for font=\"%s\" layout=\"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
            map_name,
            if !otl_tags.is_null() {
                otl_tags
            } else {
                b"none\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    }
    handle = dpx_open_truetype_file(map_name);
    if handle == INVALID_HANDLE {
        handle = dpx_open_opentype_file(map_name);
    }
    if handle == INVALID_HANDLE {
        handle = dpx_open_dfont_file(map_name);
        if handle == INVALID_HANDLE {
            free(cmap_name as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        sfont = dfont_open(handle, ttc_index as ::core::ffi::c_int);
    } else {
        sfont = sfnt_open(handle);
    }
    if sfont.is_null() {
        pdf_warning!(
            b"Could not open OpenType/TrueType/dfont font file \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            map_name,
        );
        free(cmap_name as *mut ::core::ffi::c_void);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    match (*sfont).type_0 {
        SFNT_TYPE_TTC => {
            offset = ttc_read_offset(sfont, ttc_index as SFNT_ULONG);
            if offset == 0 as SFNT_ULONG {
                pdf_warning!(
                    b"Offset=0 reurned for font=%s, TTC_index=%d\0" as *const u8 as *const ::core::ffi::c_char,
                    map_name,
                    ttc_index,
                );
                free(cmap_name as *mut ::core::ffi::c_void);
                sfnt_close(sfont);
                crate::backend::runtime::input_close(handle);
                return -(1 as ::core::ffi::c_int);
            }
        }
        SFNT_TYPE_TRUETYPE | SFNT_TYPE_POSTSCRIPT => {
            offset = 0 as SFNT_ULONG;
        }
        SFNT_TYPE_DFONT => {
            offset = (*sfont).offset;
        }
        _ => {
            pdf_warning!(
                b"Not a OpenType/TrueType/TTC font?: %s\0" as *const u8 as *const ::core::ffi::c_char,
                map_name,
            );
            free(cmap_name as *mut ::core::ffi::c_void);
            sfnt_close(sfont);
            crate::backend::runtime::input_close(handle);
            return -(1 as ::core::ffi::c_int);
        }
    }
    if sfnt_read_table_directory(sfont, offset) < 0 as ::core::ffi::c_int {
        pdf_warning!(b"Could not read OpenType/TrueType table directory.\0" as *const u8 as *const ::core::ffi::c_char,);
        free(cmap_name as *mut ::core::ffi::c_void);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    let mut maxp: *mut tt_maxp_table = ::core::ptr::null_mut::<tt_maxp_table>();
    maxp = tt_read_maxp_table(sfont);
    num_glyphs = (*maxp).numGlyphs as uint16_t;
    free(maxp as *mut ::core::ffi::c_void);
    GIDToCIDMap = new((num_glyphs as uint32_t as usize).wrapping_mul(::core::mem::size_of::<uint16_t>() as usize) as uint32_t) as *mut uint16_t;
    memset(
        GIDToCIDMap as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (num_glyphs as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
    );
    if (*sfont).type_0 == SFNT_TYPE_POSTSCRIPT {
        let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
        let mut gid: card16 = 0;
        offset = sfnt_find_table_pos(sfont, b"CFF \0" as *const u8 as *const ::core::ffi::c_char);
        if offset == 0 as SFNT_ULONG {
            pdf_warning!(b"PS OpenType but no \"CFF \" table.. Maybe variable font? (not supported)\0" as *const u8 as *const ::core::ffi::c_char,);
            free(cmap_name as *mut ::core::ffi::c_void);
            free(GIDToCIDMap as *mut ::core::ffi::c_void);
            sfnt_close(sfont);
            crate::backend::runtime::input_close(handle);
            return -(1 as ::core::ffi::c_int);
        }
        cffont = cff_open((*sfont).handle, offset as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        if cffont.is_null() {
            free(cmap_name as *mut ::core::ffi::c_void);
            free(GIDToCIDMap as *mut ::core::ffi::c_void);
            sfnt_close(sfont);
            crate::backend::runtime::input_close(handle);
            return -(1 as ::core::ffi::c_int);
        }
        if (*cffont).flag & FONTTYPE_CIDFONT == 0 {
            csi.registry = strdup(b"Adobe\0" as *const u8 as *const ::core::ffi::c_char);
            csi.ordering = strdup(b"Identity\0" as *const u8 as *const ::core::ffi::c_char);
            csi.supplement = 0 as ::core::ffi::c_int;
            gid = 0 as card16;
            while (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
                *GIDToCIDMap.offset(gid as isize) = gid as uint16_t;
                gid = gid.wrapping_add(1);
            }
        } else {
            if cff_dict_known((*cffont).topdict, b"ROS\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                csi.registry = strdup(b"Adobe\0" as *const u8 as *const ::core::ffi::c_char);
                csi.ordering = strdup(b"Identity\0" as *const u8 as *const ::core::ffi::c_char);
                csi.supplement = 0 as ::core::ffi::c_int;
            } else {
                let mut reg: card16 = 0;
                let mut ord: card16 = 0;
                reg = cff_dict_get(
                    (*cffont).topdict,
                    b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                ) as card16;
                ord = cff_dict_get(
                    (*cffont).topdict,
                    b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) as card16;
                csi.registry = cff_get_string(cffont, reg as s_SID);
                csi.ordering = cff_get_string(cffont, ord as s_SID);
                csi.supplement = cff_dict_get(
                    (*cffont).topdict,
                    b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
                    2 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
            }
            cff_read_charsets(cffont);
            create_GIDToCIDMap(GIDToCIDMap, num_glyphs, cffont);
        }
        cff_close(cffont);
    } else {
        let mut gid_0: uint16_t = 0;
        csi.registry = strdup(b"Adobe\0" as *const u8 as *const ::core::ffi::c_char);
        csi.ordering = strdup(b"Identity\0" as *const u8 as *const ::core::ffi::c_char);
        csi.supplement = 0 as ::core::ffi::c_int;
        gid_0 = 0 as uint16_t;
        while (gid_0 as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
            *GIDToCIDMap.offset(gid_0 as isize) = gid_0;
            gid_0 = gid_0.wrapping_add(1);
        }
    }
    ttcmap = tt_cmap_read(sfont, 3 as USHORT, 10 as USHORT);
    if ttcmap.is_null() {
        ttcmap = tt_cmap_read(sfont, 3 as USHORT, 1 as USHORT);
        if ttcmap.is_null() {
            ttcmap = tt_cmap_read(sfont, 0 as USHORT, 3 as USHORT);
        }
        if ttcmap.is_null() {
            ttcmap = tt_cmap_read(sfont, 0 as USHORT, 4 as USHORT);
        }
    }
    if !ttcmap.is_null() {
        let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
        let mut map_base: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        let mut map_sub: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
        let mut gsub_vert: *mut otl_gsub = ::core::ptr::null_mut::<otl_gsub>();
        let mut gsub_list: *mut otl_gsub = ::core::ptr::null_mut::<otl_gsub>();
        let mut gid_1: uint32_t = 0;
        if wmode == 1 as ::core::ffi::c_int {
            gsub_vert = otl_gsub_new();
            if otl_gsub_add_feat(
                gsub_vert,
                b"*\0" as *const u8 as *const ::core::ffi::c_char,
                b"*\0" as *const u8 as *const ::core::ffi::c_char,
                b"vrt2\0" as *const u8 as *const ::core::ffi::c_char,
                sfont,
            ) < 0 as ::core::ffi::c_int
            {
                if otl_gsub_add_feat(
                    gsub_vert,
                    b"*\0" as *const u8 as *const ::core::ffi::c_char,
                    b"*\0" as *const u8 as *const ::core::ffi::c_char,
                    b"vert\0" as *const u8 as *const ::core::ffi::c_char,
                    sfont,
                ) < 0 as ::core::ffi::c_int
                {
                    pdf_warning!(b"GSUB feature vrt2/vert not found.\0" as *const u8 as *const ::core::ffi::c_char,);
                    otl_gsub_release(gsub_vert);
                    gsub_vert = ::core::ptr::null_mut::<otl_gsub>();
                } else {
                    otl_gsub_select(
                        gsub_vert,
                        b"*\0" as *const u8 as *const ::core::ffi::c_char,
                        b"*\0" as *const u8 as *const ::core::ffi::c_char,
                        b"vert\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            } else {
                otl_gsub_select(
                    gsub_vert,
                    b"*\0" as *const u8 as *const ::core::ffi::c_char,
                    b"*\0" as *const u8 as *const ::core::ffi::c_char,
                    b"vrt2\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        } else {
            gsub_vert = ::core::ptr::null_mut::<otl_gsub>();
        }
        if !otl_tags.is_null() {
            gsub_list = otl_gsub_new();
            if otl_gsub_add_feat_list(gsub_list, otl_tags, sfont) < 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Reading GSUB feature table(s) failed for \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                    otl_tags,
                );
            } else {
                otl_gsub_set_chain(gsub_list, otl_tags);
            }
        } else {
            gsub_list = ::core::ptr::null_mut::<otl_gsub>();
        }
        cmap = CMap_new();
        CMap_set_name(cmap, cmap_name);
        CMap_set_type(cmap, CMAP_TYPE_CODE_TO_CID);
        CMap_set_wmode(cmap, wmode);
        CMap_add_codespacerange(cmap, LRANGE_MIN.as_ptr(), LRANGE_MAX.as_ptr(), 4 as size_t);
        CMap_set_CIDSysInfo(cmap, &raw mut csi);
        map_base = new((num_glyphs as uint32_t as usize).wrapping_mul(::core::mem::size_of::<int32_t>() as usize) as uint32_t) as *mut int32_t;
        map_sub = new((num_glyphs as uint32_t as usize).wrapping_mul(::core::mem::size_of::<int32_t>() as usize) as uint32_t) as *mut int32_t;
        gid_1 = 0 as uint32_t;
        while gid_1 < num_glyphs as uint32_t {
            *map_base.offset(gid_1 as isize) = -(1 as ::core::ffi::c_int) as int32_t;
            *map_sub.offset(gid_1 as isize) = -(1 as ::core::ffi::c_int) as int32_t;
            gid_1 = gid_1.wrapping_add(1);
        }
        match (*ttcmap).format as ::core::ffi::c_int {
            12 => {
                load_cmap12(
                    (*ttcmap).map as *mut cmap12,
                    GIDToCIDMap,
                    num_glyphs as USHORT,
                    gsub_vert,
                    gsub_list,
                    cmap,
                    map_base,
                    map_sub,
                );
            }
            4 => {
                load_cmap4(
                    (*ttcmap).map as *mut cmap4,
                    GIDToCIDMap,
                    num_glyphs as USHORT,
                    gsub_vert,
                    gsub_list,
                    cmap,
                    map_base,
                    map_sub,
                );
            }
            _ => {}
        }
        if !gsub_vert.is_null() {
            otl_gsub_release(gsub_vert);
        }
        if !gsub_list.is_null() {
            otl_gsub_release(gsub_list);
        }
        tt_cmap_release(ttcmap);
        if !otl_tags.is_null() {
            let mut tounicode: *mut CMap = ::core::ptr::null_mut::<CMap>();
            let mut tounicode_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut tounicode_id: ::core::ffi::c_int = 0;
            let mut name_len: size_t = 0;
            name_len = strlen(map_name)
                .wrapping_add(strlen(b"-UCS32-Add\0" as *const u8 as *const ::core::ffi::c_char))
                .wrapping_add(32 as size_t);
            tounicode_name = new((name_len as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
                as *mut ::core::ffi::c_char;
            snprintf(
                tounicode_name,
                name_len,
                b"%s:%d-UCS32-Add\0" as *const u8 as *const ::core::ffi::c_char,
                map_name,
                ttc_index,
            );
            *tounicode_name.offset(name_len.wrapping_sub(1 as size_t) as isize) = '\0' as i32 as ::core::ffi::c_char;
            tounicode_id = CMap_cache_find(tounicode_name);
            if tounicode_id >= 0 as ::core::ffi::c_int {
                tounicode = CMap_cache_get(tounicode_id);
            } else {
                tounicode = CMap_new();
                CMap_set_name(tounicode, tounicode_name);
                CMap_set_type(tounicode, CMAP_TYPE_TO_UNICODE);
                CMap_set_wmode(tounicode, 0 as ::core::ffi::c_int);
                CMap_add_codespacerange(tounicode, SRANGE_MIN.as_ptr(), SRANGE_MAX.as_ptr(), 2 as size_t);
                CMap_set_CIDSysInfo(tounicode, &raw mut CSI_UNICODE);
                CMap_add_bfchar(tounicode, SRANGE_MIN.as_ptr(), 2 as size_t, SRANGE_MAX.as_ptr(), 2 as size_t);
                tounicode_id = CMap_cache_add(tounicode);
            }
            free(tounicode_name as *mut ::core::ffi::c_void);
            gid_1 = 0 as uint32_t;
            while gid_1 < num_glyphs as uint32_t {
                let mut cid: uint16_t = *GIDToCIDMap.offset(gid_1 as isize);
                let mut src: [::core::ffi::c_uchar; 2] = [0; 2];
                let mut dst: [::core::ffi::c_uchar; 4] = [0; 4];
                if cid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                    let mut ch: int32_t = if UC_is_valid(*map_base.offset(gid_1 as isize)) as ::core::ffi::c_int != 0 {
                        *map_base.offset(gid_1 as isize)
                    } else {
                        *map_sub.offset(gid_1 as isize)
                    };
                    if UC_is_valid(ch) {
                        let mut p: *mut ::core::ffi::c_uchar = &raw mut dst as *mut ::core::ffi::c_uchar;
                        let mut endptr: *mut ::core::ffi::c_uchar =
                            (&raw mut dst as *mut ::core::ffi::c_uchar).offset(4 as ::core::ffi::c_int as isize);
                        let mut len_1: size_t = 0;
                        src[0 as ::core::ffi::c_int as usize] =
                            (cid as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        src[1 as ::core::ffi::c_int as usize] = (cid as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        len_1 = UC_UTF16BE_encode_char(ch, &raw mut p, endptr);
                        if len_1 > 0 as size_t {
                            CMap_add_bfchar(
                                tounicode,
                                &raw mut src as *mut ::core::ffi::c_uchar,
                                2 as size_t,
                                &raw mut dst as *mut ::core::ffi::c_uchar,
                                len_1,
                            );
                        }
                    }
                }
                gid_1 = gid_1.wrapping_add(1);
            }
        }
        cmap_id = CMap_cache_add(cmap);
    }
    free(cmap_name as *mut ::core::ffi::c_void);
    free(GIDToCIDMap as *mut ::core::ffi::c_void);
    free(csi.registry as *mut ::core::ffi::c_void);
    free(csi.ordering as *mut ::core::ffi::c_void);
    sfnt_close(sfont);
    crate::backend::runtime::input_close(handle);
    return cmap_id;
}
#[unsafe(export_name = "__pdf_impl_otf_try_load_GID_to_CID_map")]
pub unsafe extern "C-unwind" fn otf_try_load_GID_to_CID_map(
    mut map_name: *const ::core::ffi::c_char, mut ttc_index: uint32_t, mut wmode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cmap_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut offset: SFNT_ULONG = 0 as SFNT_ULONG;
    let mut cmap_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    let mut len: size_t = 0;
    if map_name.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    len = strlen(map_name).wrapping_add(32 as size_t);
    cmap_name =
        new((len as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    snprintf(
        cmap_name,
        len,
        b"%s:%d-%1d-GID\0" as *const u8 as *const ::core::ffi::c_char,
        map_name,
        ttc_index,
        wmode,
    );
    *cmap_name.offset(len.wrapping_sub(1 as size_t) as isize) = '\0' as i32 as ::core::ffi::c_char;
    cmap_id = CMap_cache_find(cmap_name);
    if cmap_id >= 0 as ::core::ffi::c_int {
        free(cmap_name as *mut ::core::ffi::c_void);
        if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
            pdf_message!(
                b"otf_cmap>> GID-to-CID mapping found at cmap_id=%d.\n\0" as *const u8 as *const ::core::ffi::c_char,
                cmap_id,
            );
        }
        return cmap_id;
    }
    handle = dpx_open_truetype_file(map_name);
    if handle == INVALID_HANDLE {
        handle = dpx_open_opentype_file(map_name);
    }
    if handle == INVALID_HANDLE {
        handle = dpx_open_dfont_file(map_name);
        if handle == INVALID_HANDLE {
            free(cmap_name as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        sfont = dfont_open(handle, ttc_index as ::core::ffi::c_int);
    } else {
        sfont = sfnt_open(handle);
    }
    if sfont.is_null() {
        pdf_warning!(
            b"Could not open OpenType/TrueType/dfont font file \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            map_name,
        );
        free(cmap_name as *mut ::core::ffi::c_void);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    match (*sfont).type_0 {
        SFNT_TYPE_TTC => {
            offset = ttc_read_offset(sfont, ttc_index as SFNT_ULONG);
            if offset == 0 as SFNT_ULONG {
                pdf_warning!(
                    b"Invalid TTC index for font \"%s\": %d\0" as *const u8 as *const ::core::ffi::c_char,
                    map_name,
                    ttc_index,
                );
                sfnt_close(sfont);
                crate::backend::runtime::input_close(handle);
                free(cmap_name as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
        }
        SFNT_TYPE_TRUETYPE | SFNT_TYPE_POSTSCRIPT => {
            offset = 0 as SFNT_ULONG;
        }
        SFNT_TYPE_DFONT => {
            offset = (*sfont).offset;
        }
        _ => {
            pdf_warning!(
                b"Not a OpenType/TrueType/TTC font?: %s\0" as *const u8 as *const ::core::ffi::c_char,
                map_name,
            );
            sfnt_close(sfont);
            crate::backend::runtime::input_close(handle);
            free(cmap_name as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
    }
    if sfnt_read_table_directory(sfont, offset) < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Could not read OpenType/TrueType table directory: %s\0" as *const u8 as *const ::core::ffi::c_char,
            map_name,
        );
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        free(cmap_name as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    if (*sfont).type_0 != SFNT_TYPE_POSTSCRIPT {
        free(cmap_name as *mut ::core::ffi::c_void);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if (*sfont).type_0 == SFNT_TYPE_POSTSCRIPT {
        let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
        let mut maxp: *mut tt_maxp_table = ::core::ptr::null_mut::<tt_maxp_table>();
        let csrange: [::core::ffi::c_uchar; 4] = [
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
            0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        ];
        let mut num_glyphs: uint16_t = 0 as uint16_t;
        maxp = tt_read_maxp_table(sfont);
        num_glyphs = (*maxp).numGlyphs as uint16_t;
        free(maxp as *mut ::core::ffi::c_void);
        offset = sfnt_find_table_pos(sfont, b"CFF \0" as *const u8 as *const ::core::ffi::c_char);
        if offset == 0 as SFNT_ULONG {
            pdf_warning!(b"PS OpenType but no \"CFF \" table.. Maybe variable font? (not supported)\0" as *const u8 as *const ::core::ffi::c_char,);
            free(cmap_name as *mut ::core::ffi::c_void);
            sfnt_close(sfont);
            crate::backend::runtime::input_close(handle);
            return -(1 as ::core::ffi::c_int);
        }
        cffont = cff_open((*sfont).handle, offset as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        if !cffont.is_null() && (*cffont).flag & FONTTYPE_CIDFONT != 0 {
            let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
            let mut gid: uint16_t = 0;
            let mut GIDToCIDMap: *mut uint16_t = ::core::ptr::null_mut::<uint16_t>();
            let mut csi: CIDSysInfo = CIDSysInfo {
                registry: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ordering: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                supplement: 0 as ::core::ffi::c_int,
            };
            if cff_dict_known((*cffont).topdict, b"ROS\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                csi.registry = strdup(b"Adobe\0" as *const u8 as *const ::core::ffi::c_char);
                csi.ordering = strdup(b"Identity\0" as *const u8 as *const ::core::ffi::c_char);
                csi.supplement = 0 as ::core::ffi::c_int;
            } else {
                let mut reg: card16 = 0;
                let mut ord: card16 = 0;
                reg = cff_dict_get(
                    (*cffont).topdict,
                    b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                ) as card16;
                ord = cff_dict_get(
                    (*cffont).topdict,
                    b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) as card16;
                csi.registry = cff_get_string(cffont, reg as s_SID);
                csi.ordering = cff_get_string(cffont, ord as s_SID);
                csi.supplement = cff_dict_get(
                    (*cffont).topdict,
                    b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
                    2 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
            }
            cff_read_charsets(cffont);
            GIDToCIDMap =
                new((num_glyphs as uint32_t as usize).wrapping_mul(::core::mem::size_of::<uint16_t>() as usize) as uint32_t) as *mut uint16_t;
            memset(
                GIDToCIDMap as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (num_glyphs as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            create_GIDToCIDMap(GIDToCIDMap, num_glyphs, cffont);
            cmap = CMap_new();
            CMap_set_name(cmap, cmap_name);
            CMap_set_type(cmap, CMAP_TYPE_CODE_TO_CID);
            CMap_set_wmode(cmap, wmode);
            CMap_add_codespacerange(
                cmap,
                (&raw const csrange as *const ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_uchar,
                (&raw const csrange as *const ::core::ffi::c_uchar).offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_uchar,
                2 as size_t,
            );
            CMap_set_CIDSysInfo(cmap, &raw mut csi);
            gid = 0 as uint16_t;
            while (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
                let mut src: [::core::ffi::c_uchar; 2] = [0; 2];
                let mut dst: [::core::ffi::c_uchar; 2] = [0; 2];
                src[0 as ::core::ffi::c_int as usize] =
                    (gid as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                src[1 as ::core::ffi::c_int as usize] = (gid as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                dst[0 as ::core::ffi::c_int as usize] = (*GIDToCIDMap.offset(gid as isize) as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                dst[1 as ::core::ffi::c_int as usize] =
                    (*GIDToCIDMap.offset(gid as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                CMap_add_bfchar(
                    cmap,
                    &raw mut src as *mut ::core::ffi::c_uchar,
                    2 as size_t,
                    &raw mut dst as *mut ::core::ffi::c_uchar,
                    2 as size_t,
                );
                gid = gid.wrapping_add(1);
            }
            cmap_id = CMap_cache_add(cmap);
            if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
                pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                pdf_message!(
                    b"otf_cmap>> Creating GID-to-CID mapping for font=\"%s\"\n\0" as *const u8 as *const ::core::ffi::c_char,
                    map_name,
                );
            }
            free(GIDToCIDMap as *mut ::core::ffi::c_void);
            if !csi.registry.is_null() {
                free(csi.registry as *mut ::core::ffi::c_void);
            }
            if !csi.ordering.is_null() {
                free(csi.ordering as *mut ::core::ffi::c_void);
            }
        }
        if !cffont.is_null() {
            cff_close(cffont);
        }
    }
    free(cmap_name as *mut ::core::ffi::c_void);
    sfnt_close(sfont);
    crate::backend::runtime::input_close(handle);
    return cmap_id;
}
unsafe extern "C" {
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
    fn memset(__b: *mut ::core::ffi::c_void, __c: ::core::ffi::c_int, __len: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[repr(C)]
pub struct otl_gsub {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_cmap_read(
    mut sfont: *mut sfnt, mut platform: USHORT, mut encoding: USHORT, _result: *mut *mut tt_cmap,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_cmap_read(sfont, platform, encoding) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_cmap_release(mut cmap: *mut tt_cmap) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_cmap_release(cmap) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_cmap_lookup(mut cmap: *mut tt_cmap, mut cc: SFNT_ULONG, _result: *mut USHORT) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_cmap_lookup(cmap, cc) }) {
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
pub unsafe extern "C" fn __pdf_entry_otf_create_ToUnicode_stream(
    mut font_name: *const ::core::ffi::c_char, mut ttc_index: uint32_t, mut basefont: *const ::core::ffi::c_char,
    mut used_chars: *const ::core::ffi::c_char, _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otf_create_ToUnicode_stream(font_name, ttc_index, basefont, used_chars) }) {
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
pub unsafe extern "C" fn __pdf_entry_otf_load_Unicode_CMap(
    mut map_name: *const ::core::ffi::c_char, mut ttc_index: uint32_t, mut otl_tags: *const ::core::ffi::c_char, mut wmode: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otf_load_Unicode_CMap(map_name, ttc_index, otl_tags, wmode) }) {
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
pub unsafe extern "C" fn __pdf_entry_otf_try_load_GID_to_CID_map(
    mut map_name: *const ::core::ffi::c_char, mut ttc_index: uint32_t, mut wmode: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otf_try_load_GID_to_CID_map(map_name, ttc_index, wmode) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
