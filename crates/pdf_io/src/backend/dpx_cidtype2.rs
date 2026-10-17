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
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strcat(
        __s1: *mut ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __big: *const ::core::ffi::c_char,
        __little: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    static mut dpx_conf: _dpx_conf;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_dpx_open_truetype_file"]
    fn dpx_open_truetype_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_dpx_open_dfont_file"]
    fn dpx_open_dfont_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_ref_obj"]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_string"]
    fn pdf_new_string(str: *const ::core::ffi::c_void, length: size_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(
        dict: *mut pdf_obj,
        key: *mut pdf_obj,
        value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_stream"]
    fn pdf_new_stream(flags: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_stream"]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const ::core::ffi::c_void,
        stream_data_len: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_stream_length"]
    fn pdf_stream_length(stream: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_check_version"]
    fn pdf_check_version(
        major: ::core::ffi::c_int,
        minor: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_make_uniqueTag"]
    fn pdf_font_make_uniqueTag(tag: *mut ::core::ffi::c_char);
    static mut opt_flags_cidfont: ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_decode_char"]
    fn CMap_decode_char(
        cmap: *mut CMap,
        inbuf: *mut *const ::core::ffi::c_uchar,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut ::core::ffi::c_uchar,
        outbytesleft: *mut size_t,
    );
    #[link_name = "__pdf_impl_CMap_cache_get"]
    fn CMap_cache_get(id: ::core::ffi::c_int) -> *mut CMap;
    #[link_name = "__pdf_impl_CMap_cache_find"]
    fn CMap_cache_find(cmap_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    #[link_name = "__pdf_impl_sfnt_require_table"]
    fn sfnt_require_table(
        sfont: *mut sfnt,
        tag: *const ::core::ffi::c_char,
        must_exist: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_sfnt_create_FontFile_stream"]
    fn sfnt_create_FontFile_stream(sfont: *mut sfnt) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_ttc_read_offset"]
    fn ttc_read_offset(sfont: *mut sfnt, ttc_idx: SFNT_ULONG) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_tt_get_fontdesc"]
    fn tt_get_fontdesc(
        sfont: *mut sfnt,
        embed: *mut ::core::ffi::c_int,
        stemv: ::core::ffi::c_int,
        type_0: ::core::ffi::c_int,
        fontname: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_tt_cmap_read"]
    fn tt_cmap_read(sfont: *mut sfnt, platform: USHORT, encoding: USHORT) -> *mut tt_cmap;
    #[link_name = "__pdf_impl_tt_cmap_lookup"]
    fn tt_cmap_lookup(cmap: *mut tt_cmap, cc: SFNT_ULONG) -> USHORT;
    #[link_name = "__pdf_impl_tt_cmap_release"]
    fn tt_cmap_release(cmap: *mut tt_cmap);
    #[link_name = "__pdf_impl_tt_build_init"]
    fn tt_build_init() -> *mut tt_glyphs;
    #[link_name = "__pdf_impl_tt_build_finish"]
    fn tt_build_finish(g: *mut tt_glyphs);
    #[link_name = "__pdf_impl_tt_add_glyph"]
    fn tt_add_glyph(g: *mut tt_glyphs, gid: USHORT, new_gid: USHORT) -> USHORT;
    #[link_name = "__pdf_impl_tt_get_index"]
    fn tt_get_index(g: *mut tt_glyphs, gid: USHORT) -> USHORT;
    #[link_name = "__pdf_impl_tt_build_tables"]
    fn tt_build_tables(sfont: *mut sfnt, g: *mut tt_glyphs) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tt_get_metrics"]
    fn tt_get_metrics(sfont: *mut sfnt, g: *mut tt_glyphs) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_new"]
    fn otl_gsub_new() -> *mut otl_gsub;
    #[link_name = "__pdf_impl_otl_gsub_release"]
    fn otl_gsub_release(gsub_list: *mut otl_gsub);
    #[link_name = "__pdf_impl_otl_gsub_select"]
    fn otl_gsub_select(
        gsub_list: *mut otl_gsub,
        script: *const ::core::ffi::c_char,
        language: *const ::core::ffi::c_char,
        feature: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_add_feat"]
    fn otl_gsub_add_feat(
        gsub_list: *mut otl_gsub,
        script: *const ::core::ffi::c_char,
        language: *const ::core::ffi::c_char,
        feature: *const ::core::ffi::c_char,
        sfont: *mut sfnt,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_apply"]
    fn otl_gsub_apply(gsub_list: *mut otl_gsub, gid: *mut USHORT) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tt_get_ps_fontname"]
    fn tt_get_ps_fontname(
        sfont: *mut sfnt,
        dest: *mut ::core::ffi::c_char,
        destlen: USHORT,
    ) -> USHORT;
    #[link_name = "__pdf_impl_UC_UTF16BE_decode_char"]
    fn UC_UTF16BE_decode_char(
        pp: *mut *const ::core::ffi::c_uchar,
        endptr: *const ::core::ffi::c_uchar,
    ) -> int32_t;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type rust_input_handle_t = uintptr_t;
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub keep_cache: ::core::ffi::c_int,
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
pub struct cid_opt {
    pub csi: CIDSysInfo,
    pub style: ::core::ffi::c_int,
    pub embed: ::core::ffi::c_int,
    pub stemv: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_font {
    pub ident: *mut ::core::ffi::c_char,
    pub font_id: ::core::ffi::c_int,
    pub subtype: ::core::ffi::c_int,
    pub filename: *mut ::core::ffi::c_char,
    pub encoding_id: ::core::ffi::c_int,
    pub index: uint32_t,
    pub fontname: *mut ::core::ffi::c_char,
    pub uniqueID: [::core::ffi::c_char; 7],
    pub reference: *mut pdf_obj,
    pub resource: *mut pdf_obj,
    pub descriptor: *mut pdf_obj,
    pub usedchars: *mut ::core::ffi::c_char,
    pub flags: ::core::ffi::c_int,
    pub point_size: ::core::ffi::c_double,
    pub design_size: ::core::ffi::c_double,
    pub type0: C2RustUnnamed_1,
    pub cid: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub csi: CIDSysInfo,
    pub options: cid_opt,
    pub need_vmetrics: ::core::ffi::c_int,
    pub usedchars_v: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub descendant: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: ::core::ffi::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
pub type SFNT_ULONG = uint32_t;
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
pub struct sfnt_table {
    pub tag: [::core::ffi::c_char; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut ::core::ffi::c_char,
}
pub type USHORT = ::core::ffi::c_ushort;
pub type CID = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub name: *const ::core::ffi::c_char,
    pub must_exist: ::core::ffi::c_int,
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
pub type BYTE = ::core::ffi::c_uchar;
pub type SHORT = ::core::ffi::c_short;
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
pub struct C2RustUnnamed_3 {
    pub alt1: ::core::ffi::c_ushort,
    pub alt2: ::core::ffi::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_5,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: ::core::ffi::c_int,
    pub profile: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
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
pub const via_cid_to_code: C2RustUnnamed_6 = 1;
pub const via_cid_to_gid: C2RustUnnamed_6 = 2;
pub const glyph_ordering: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub platform: ::core::ffi::c_ushort,
    pub encoding: ::core::ffi::c_ushort,
    pub pdfnames: [*const ::core::ffi::c_char; 5],
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const PDF_NAME_LEN_MAX: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
#[inline]
unsafe extern "C-unwind" fn streq_ptr(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
    }
    return false_0 != 0;
}
pub const FONTMAP_STYLE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FONTMAP_STYLE_BOLD: ::core::ffi::c_int = 1;
pub const FONTMAP_STYLE_ITALIC: ::core::ffi::c_int = 2;
pub const FONTMAP_STYLE_BOLDITALIC: ::core::ffi::c_int = 3;
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PDF_FONT_FONTTYPE_CIDTYPE2: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PDF_FONT_FLAG_BASEFONT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const FONT_STYLE_NONE: ::core::ffi::c_int = FONTMAP_STYLE_NONE;
pub const FONT_STYLE_BOLD: ::core::ffi::c_int = FONTMAP_STYLE_BOLD;
pub const FONT_STYLE_ITALIC: ::core::ffi::c_int = FONTMAP_STYLE_ITALIC;
pub const FONT_STYLE_BOLDITALIC: ::core::ffi::c_int = FONTMAP_STYLE_BOLDITALIC;
pub const CIDFONT_FORCE_FIXEDPITCH: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const SFNT_TYPE_TRUETYPE: ::core::ffi::c_int = 1;
pub const SFNT_TYPE_TTC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const SFNT_TYPE_DFONT: ::core::ffi::c_int = 256;
pub const TT_MAC: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TT_WIN: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TT_WIN_UNICODE: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TT_WIN_SJIS: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TT_WIN_RPC: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TT_WIN_BIG5: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TT_WIN_WANSUNG: ::core::ffi::c_uint = 5 as ::core::ffi::c_uint;
pub const TT_WIN_UCS4: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
pub const TT_MAC_JAPANESE: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TT_MAC_TRADITIONAL_CHINESE: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
pub const TT_MAC_KOREAN: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TT_MAC_SIMPLIFIED_CHINESE: ::core::ffi::c_uint = 25 as ::core::ffi::c_uint;
static mut required_table: [C2RustUnnamed_2; 12] = [
    C2RustUnnamed_2 {
        name: b"OS/2\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"head\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"loca\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"hmtx\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"cvt \0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"prep\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        must_exist: 0 as ::core::ffi::c_int,
    },
];
unsafe extern "C-unwind" fn validate_name(
    mut fontname: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    static mut badstrlist: [*const ::core::ffi::c_char; 5] = [
        b"-WIN-RKSJ-H\0" as *const u8 as *const ::core::ffi::c_char,
        b"-WINP-RKSJ-H\0" as *const u8 as *const ::core::ffi::c_char,
        b"-WING-RKSJ-H\0" as *const u8 as *const ::core::ffi::c_char,
        b"-90pv-RKSJ-H\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    count = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        if *fontname.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            memmove(
                fontname.offset(i as isize) as *mut ::core::ffi::c_void,
                fontname
                    .offset(i as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_void,
                (len - i) as size_t,
            );
            count += 1;
            len -= 1;
        }
        i += 1;
    }
    if count > 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Removed %d null character(s) from fontname --> %s\0" as *const u8
                as *const ::core::ffi::c_char,
            count,
            fontname,
        );
    }
    *fontname.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while !badstrlist[i as usize].is_null() {
        p = strstr(fontname, badstrlist[i as usize]);
        if !p.is_null() && p > fontname {
            pdf_warning!(
                b"Removed string \"%s\" from fontname \"%s\".\0" as *const u8
                    as *const ::core::ffi::c_char,
                badstrlist[i as usize],
                fontname,
            );
            *p.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
            len = p.offset_from(fontname) as ::core::ffi::c_long as ::core::ffi::c_int;
            break;
        } else {
            i += 1;
        }
    }
    if len < 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"No valid character found in fontname string.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
pub const WIN_UCS_INDEX_MAX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KNOWN_ENCODINGS_MAX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
static mut known_encodings: [C2RustUnnamed_7; 12] = [
    C2RustUnnamed_7 {
        platform: TT_WIN as ::core::ffi::c_ushort,
        encoding: TT_WIN_UCS4 as ::core::ffi::c_ushort,
        pdfnames: [
            b"UCSms-UCS4\0" as *const u8 as *const ::core::ffi::c_char,
            b"UCSms-UCS2\0" as *const u8 as *const ::core::ffi::c_char,
            b"UCS4\0" as *const u8 as *const ::core::ffi::c_char,
            b"UCS2\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_7 {
        platform: TT_WIN as ::core::ffi::c_ushort,
        encoding: TT_WIN_UNICODE as ::core::ffi::c_ushort,
        pdfnames: [
            b"UCSms-UCS4\0" as *const u8 as *const ::core::ffi::c_char,
            b"UCSms-UCS2\0" as *const u8 as *const ::core::ffi::c_char,
            b"UCS4\0" as *const u8 as *const ::core::ffi::c_char,
            b"UCS2\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_7 {
        platform: TT_WIN as ::core::ffi::c_ushort,
        encoding: TT_WIN_SJIS as ::core::ffi::c_ushort,
        pdfnames: [
            b"90ms-RKSJ\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_7 {
        platform: TT_WIN as ::core::ffi::c_ushort,
        encoding: TT_WIN_RPC as ::core::ffi::c_ushort,
        pdfnames: [
            b"GBK-EUC\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_7 {
        platform: TT_WIN as ::core::ffi::c_ushort,
        encoding: TT_WIN_BIG5 as ::core::ffi::c_ushort,
        pdfnames: [
            b"ETen-B5\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_7 {
        platform: TT_WIN as ::core::ffi::c_ushort,
        encoding: TT_WIN_WANSUNG as ::core::ffi::c_ushort,
        pdfnames: [
            b"KSCms-UHC\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_7 {
        platform: TT_MAC as ::core::ffi::c_ushort,
        encoding: TT_MAC_JAPANESE as ::core::ffi::c_ushort,
        pdfnames: [
            b"90pv-RKSJ\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_7 {
        platform: TT_MAC as ::core::ffi::c_ushort,
        encoding: TT_MAC_TRADITIONAL_CHINESE as ::core::ffi::c_ushort,
        pdfnames: [
            b"B5pc\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_7 {
        platform: TT_MAC as ::core::ffi::c_ushort,
        encoding: TT_MAC_SIMPLIFIED_CHINESE as ::core::ffi::c_ushort,
        pdfnames: [
            b"GBpc-EUC\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_7 {
        platform: TT_MAC as ::core::ffi::c_ushort,
        encoding: TT_MAC_KOREAN as ::core::ffi::c_ushort,
        pdfnames: [
            b"KSCpc-EUC\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_7 {
        platform: 0 as ::core::ffi::c_ushort,
        encoding: 4 as ::core::ffi::c_ushort,
        pdfnames: [
            b"UCSms-UCS4\0" as *const u8 as *const ::core::ffi::c_char,
            b"UCSms-UCS2\0" as *const u8 as *const ::core::ffi::c_char,
            b"UCS4\0" as *const u8 as *const ::core::ffi::c_char,
            b"UCS2\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_7 {
        platform: 0 as ::core::ffi::c_ushort,
        encoding: 0 as ::core::ffi::c_ushort,
        pdfnames: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
];
unsafe extern "C-unwind" fn find_tocode_cmap(
    mut reg: *const ::core::ffi::c_char,
    mut ord: *const ::core::ffi::c_char,
    mut select: ::core::ffi::c_int,
) -> *mut CMap {
    let mut cmap_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut cmap_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut append: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if reg.is_null()
        || ord.is_null()
        || select < 0 as ::core::ffi::c_int
        || select > KNOWN_ENCODINGS_MAX
    {
        pdf_warning!(b"Character set unknown.\0" as *const u8 as *const ::core::ffi::c_char,);
        return ::core::ptr::null_mut::<CMap>();
    }
    if streq_ptr(ord, b"UCS\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int
        != 0
        && select <= WIN_UCS_INDEX_MAX
    {
        return ::core::ptr::null_mut::<CMap>();
    }
    i = 0 as ::core::ffi::c_int;
    while cmap_id < 0 as ::core::ffi::c_int && i < 5 as ::core::ffi::c_int {
        append = known_encodings[select as usize].pdfnames[i as usize];
        if append.is_null() {
            break;
        }
        cmap_name = new((strlen(reg)
            .wrapping_add(strlen(ord))
            .wrapping_add(strlen(append))
            .wrapping_add(3 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        sprintf(
            cmap_name,
            b"%s-%s-%s\0" as *const u8 as *const ::core::ffi::c_char,
            reg,
            ord,
            append,
        );
        cmap_id = CMap_cache_find(cmap_name);
        free(cmap_name as *mut ::core::ffi::c_void);
        i += 1;
    }
    if cmap_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Could not find CID-to-Code mapping for \"%s-%s\".\0" as *const u8
                as *const ::core::ffi::c_char,
            reg,
            ord,
        );
        pdf_warning!(
            b"I tried to load (one of) the following file(s):\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < 5 as ::core::ffi::c_int {
            append = known_encodings[select as usize].pdfnames[i as usize];
            if append.is_null() {
                break;
            }
            pdf_message!(
                b" %s-%s-%s\0" as *const u8 as *const ::core::ffi::c_char,
                reg,
                ord,
                append,
            );
            i += 1;
        }
        pdf_warning!(
            b"Please check if this file exists.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<CMap>();
    }
    return CMap_cache_get(cmap_id);
}
unsafe extern "C-unwind" fn add_TTCIDHMetrics(
    mut fontdict: *mut pdf_obj,
    mut g: *mut tt_glyphs,
    mut used_chars: *mut ::core::ffi::c_char,
    mut cidtogidmap: *mut ::core::ffi::c_uchar,
    mut last_cid: ::core::ffi::c_ushort,
) {
    let mut cid: ::core::ffi::c_int = 0;
    let mut start: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut prev: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut w_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut an_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut dw: ::core::ffi::c_double = 0.;
    let mut empty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    w_array = pdf_new_array();
    if (*g).dw as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && (*g).dw as ::core::ffi::c_int <= (*g).emsize as ::core::ffi::c_int
    {
        dw = floor(
            1000.0f64 * (*g).dw as ::core::ffi::c_int as ::core::ffi::c_double
                / (*g).emsize as ::core::ffi::c_int as ::core::ffi::c_double
                / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                + 0.5f64,
        ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    } else {
        dw = floor(
            1000.0f64
                * (*(*g).gd.offset(0 as ::core::ffi::c_int as isize)).advw as ::core::ffi::c_int
                    as ::core::ffi::c_double
                / (*g).emsize as ::core::ffi::c_int as ::core::ffi::c_double
                / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                + 0.5f64,
        ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    cid = 0 as ::core::ffi::c_int;
    while cid <= last_cid as ::core::ffi::c_int {
        let mut idx: USHORT = 0;
        let mut gid: USHORT = 0;
        let mut width: ::core::ffi::c_double = 0.;
        if !(*used_chars.offset((cid / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid % 8 as ::core::ffi::c_int
            == 0)
        {
            gid = (if !cidtogidmap.is_null() {
                (*cidtogidmap.offset((2 as ::core::ffi::c_int * cid) as isize)
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *cidtogidmap
                        .offset((2 as ::core::ffi::c_int * cid + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
            } else {
                cid
            }) as USHORT;
            idx = tt_get_index(g, gid);
            if !(cid != 0 as ::core::ffi::c_int
                && idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            {
                width = floor(
                    1000.0f64
                        * (*(*g).gd.offset(idx as isize)).advw as ::core::ffi::c_int
                            as ::core::ffi::c_double
                        / (*g).emsize as ::core::ffi::c_int as ::core::ffi::c_double
                        / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        + 0.5f64,
                ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                if width == dw {
                    if !an_array.is_null() {
                        pdf_add_array(w_array, pdf_new_number(start as ::core::ffi::c_double));
                        pdf_add_array(w_array, an_array);
                        an_array = ::core::ptr::null_mut::<pdf_obj>();
                        empty = 0 as ::core::ffi::c_int;
                    }
                } else {
                    if cid != prev + 1 as ::core::ffi::c_int {
                        if !an_array.is_null() {
                            pdf_add_array(w_array, pdf_new_number(start as ::core::ffi::c_double));
                            pdf_add_array(w_array, an_array);
                            an_array = ::core::ptr::null_mut::<pdf_obj>();
                            empty = 0 as ::core::ffi::c_int;
                        }
                    }
                    if an_array.is_null() {
                        an_array = pdf_new_array();
                        start = cid;
                    }
                    pdf_add_array(an_array, pdf_new_number(width));
                    prev = cid;
                }
            }
        }
        cid += 1;
    }
    if !an_array.is_null() {
        pdf_add_array(w_array, pdf_new_number(start as ::core::ffi::c_double));
        pdf_add_array(w_array, an_array);
        empty = 0 as ::core::ffi::c_int;
    }
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"DW\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(dw),
    );
    if empty == 0 {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"W\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(w_array),
        );
    }
    pdf_release_obj(w_array);
}
unsafe extern "C-unwind" fn add_TTCIDVMetrics(
    mut fontdict: *mut pdf_obj,
    mut g: *mut tt_glyphs,
    mut used_chars: *mut ::core::ffi::c_char,
    mut last_cid: ::core::ffi::c_ushort,
) {
    let mut w2_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut an_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cid: ::core::ffi::c_int = 0;
    let mut defaultVertOriginY: ::core::ffi::c_double = 0.;
    let mut defaultAdvanceHeight: ::core::ffi::c_double = 0.;
    let mut empty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    defaultVertOriginY = floor(
        1000.0f64
            * ((*g).default_advh as ::core::ffi::c_int - (*g).default_tsb as ::core::ffi::c_int)
                as ::core::ffi::c_double
            / (*g).emsize as ::core::ffi::c_int as ::core::ffi::c_double
            / 1 as ::core::ffi::c_int as ::core::ffi::c_double
            + 0.5f64,
    ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    defaultAdvanceHeight = floor(
        1000.0f64 * (*g).default_advh as ::core::ffi::c_int as ::core::ffi::c_double
            / (*g).emsize as ::core::ffi::c_int as ::core::ffi::c_double
            / 1 as ::core::ffi::c_int as ::core::ffi::c_double
            + 0.5f64,
    ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    w2_array = pdf_new_array();
    cid = 0 as ::core::ffi::c_int;
    while cid <= last_cid as ::core::ffi::c_int {
        let mut idx: USHORT = 0;
        let mut vertOriginX: ::core::ffi::c_double = 0.;
        let mut vertOriginY: ::core::ffi::c_double = 0.;
        let mut advanceHeight: ::core::ffi::c_double = 0.;
        if !(*used_chars.offset((cid / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid % 8 as ::core::ffi::c_int
            == 0)
        {
            idx = tt_get_index(g, cid as USHORT);
            if !(cid != 0 as ::core::ffi::c_int
                && idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            {
                advanceHeight = floor(
                    1000.0f64
                        * (*(*g).gd.offset(idx as isize)).advh as ::core::ffi::c_int
                            as ::core::ffi::c_double
                        / (*g).emsize as ::core::ffi::c_int as ::core::ffi::c_double
                        / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        + 0.5f64,
                ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                vertOriginX = floor(
                    1000.0f64
                        * (0.5f64
                            * (*(*g).gd.offset(idx as isize)).advw as ::core::ffi::c_int
                                as ::core::ffi::c_double)
                        / (*g).emsize as ::core::ffi::c_int as ::core::ffi::c_double
                        / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        + 0.5f64,
                ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                vertOriginY = floor(
                    1000.0f64
                        * ((*(*g).gd.offset(idx as isize)).tsb as ::core::ffi::c_int
                            + (*(*g).gd.offset(idx as isize)).ury as ::core::ffi::c_int)
                            as ::core::ffi::c_double
                        / (*g).emsize as ::core::ffi::c_int as ::core::ffi::c_double
                        / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        + 0.5f64,
                ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                if vertOriginY != defaultVertOriginY || advanceHeight != defaultAdvanceHeight {
                    pdf_add_array(w2_array, pdf_new_number(cid as ::core::ffi::c_double));
                    pdf_add_array(w2_array, pdf_new_number(cid as ::core::ffi::c_double));
                    pdf_add_array(w2_array, pdf_new_number(-advanceHeight));
                    pdf_add_array(w2_array, pdf_new_number(vertOriginX));
                    pdf_add_array(w2_array, pdf_new_number(vertOriginY));
                    empty = 0 as ::core::ffi::c_int;
                }
            }
        }
        cid += 1;
    }
    if defaultVertOriginY != 880 as ::core::ffi::c_int as ::core::ffi::c_double
        || defaultAdvanceHeight != 1000 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        an_array = pdf_new_array();
        pdf_add_array(an_array, pdf_new_number(defaultVertOriginY));
        pdf_add_array(an_array, pdf_new_number(-defaultAdvanceHeight));
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"DW2\0" as *const u8 as *const ::core::ffi::c_char),
            an_array,
        );
    }
    if empty == 0 {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"W2\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(w2_array),
        );
    }
    pdf_release_obj(w2_array);
}
unsafe extern "C-unwind" fn fix_CJK_symbols(
    mut code: ::core::ffi::c_ushort,
) -> ::core::ffi::c_ushort {
    let mut alt_code: ::core::ffi::c_ushort = 0;
    static mut CJK_Uni_symbols: [C2RustUnnamed_3; 10] = [
        C2RustUnnamed_3 {
            alt1: 0x2014 as ::core::ffi::c_ushort,
            alt2: 0x2015 as ::core::ffi::c_ushort,
        },
        C2RustUnnamed_3 {
            alt1: 0x2016 as ::core::ffi::c_ushort,
            alt2: 0x2225 as ::core::ffi::c_ushort,
        },
        C2RustUnnamed_3 {
            alt1: 0x203e as ::core::ffi::c_ushort,
            alt2: 0xffe3 as ::core::ffi::c_ushort,
        },
        C2RustUnnamed_3 {
            alt1: 0x2026 as ::core::ffi::c_ushort,
            alt2: 0x22ef as ::core::ffi::c_ushort,
        },
        C2RustUnnamed_3 {
            alt1: 0x2212 as ::core::ffi::c_ushort,
            alt2: 0xff0d as ::core::ffi::c_ushort,
        },
        C2RustUnnamed_3 {
            alt1: 0x301c as ::core::ffi::c_ushort,
            alt2: 0xff5e as ::core::ffi::c_ushort,
        },
        C2RustUnnamed_3 {
            alt1: 0xffe0 as ::core::ffi::c_ushort,
            alt2: 0xa2 as ::core::ffi::c_ushort,
        },
        C2RustUnnamed_3 {
            alt1: 0xffe1 as ::core::ffi::c_ushort,
            alt2: 0xa3 as ::core::ffi::c_ushort,
        },
        C2RustUnnamed_3 {
            alt1: 0xffe2 as ::core::ffi::c_ushort,
            alt2: 0xac as ::core::ffi::c_ushort,
        },
        C2RustUnnamed_3 {
            alt1: 0xffff as ::core::ffi::c_ushort,
            alt2: 0xffff as ::core::ffi::c_ushort,
        },
    ];
    let mut i: ::core::ffi::c_uint = 0;
    alt_code = code;
    i = 0 as ::core::ffi::c_uint;
    while (i as usize) < NUM_CJK_SYMBOLS {
        if CJK_Uni_symbols[i as usize].alt1 as ::core::ffi::c_int == code as ::core::ffi::c_int {
            alt_code = CJK_Uni_symbols[i as usize].alt2;
            break;
        } else if CJK_Uni_symbols[i as usize].alt2 as ::core::ffi::c_int
            == code as ::core::ffi::c_int
        {
            alt_code = CJK_Uni_symbols[i as usize].alt1;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    return alt_code;
}
pub const NUM_CJK_SYMBOLS: usize = (::core::mem::size_of::<[C2RustUnnamed_3; 10]>() as usize)
    .wrapping_div(::core::mem::size_of::<C2RustUnnamed_3>() as usize);
unsafe extern "C-unwind" fn cid_to_code(
    mut cmap: *mut CMap,
    mut cid: CID,
    mut unicode_cmap: ::core::ffi::c_int,
) -> int32_t {
    let mut inbuf: [::core::ffi::c_uchar; 2] = [0; 2];
    let mut outbuf: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut inbytesleft: size_t = 2 as size_t;
    let mut outbytesleft: size_t = 32 as size_t;
    let mut p: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut q: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if cmap.is_null() {
        return cid as int32_t;
    }
    inbuf[0 as ::core::ffi::c_int as usize] = (cid as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int)
        as ::core::ffi::c_uchar;
    inbuf[1 as ::core::ffi::c_int as usize] =
        (cid as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    p = &raw mut inbuf as *mut ::core::ffi::c_uchar;
    q = &raw mut outbuf as *mut ::core::ffi::c_uchar;
    CMap_decode_char(
        cmap,
        &raw mut p,
        &raw mut inbytesleft,
        &raw mut q,
        &raw mut outbytesleft,
    );
    if inbytesleft != 0 as size_t {
        return -(1 as int32_t);
    } else if outbytesleft == 31 as size_t {
        return outbuf[0 as ::core::ffi::c_int as usize] as int32_t;
    } else if outbytesleft == 30 as size_t {
        return ((outbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | outbuf[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
            as int32_t;
    } else if outbytesleft == 28 as size_t {
        if unicode_cmap != 0 {
            let mut uc: int32_t = 0;
            let mut endptr: *const ::core::ffi::c_uchar =
                ::core::ptr::null::<::core::ffi::c_uchar>();
            p = &raw mut outbuf as *mut ::core::ffi::c_uchar;
            endptr = p.offset(4 as ::core::ffi::c_int as isize);
            uc = UC_UTF16BE_decode_char(&raw mut p, endptr);
            if p != endptr {
                pdf_warning!(
                    b"CID=%u mapped to non-single Unicode characters...\0" as *const u8
                        as *const ::core::ffi::c_char,
                    cid as ::core::ffi::c_int,
                );
            }
            return if p == endptr { uc } else { -(1 as int32_t) };
        } else {
            return (outbuf[0 as ::core::ffi::c_int as usize] as int32_t)
                << 24 as ::core::ffi::c_int
                | (outbuf[1 as ::core::ffi::c_int as usize] as int32_t)
                    << 16 as ::core::ffi::c_int
                | (outbuf[2 as ::core::ffi::c_int as usize] as int32_t) << 8 as ::core::ffi::c_int
                | outbuf[3 as ::core::ffi::c_int as usize] as int32_t;
        }
    }
    return -(1 as int32_t);
}
unsafe extern "C-unwind" fn cid_to_gid(mut cmap: *mut CMap, mut cid: CID) -> uint16_t {
    let mut inbuf: [::core::ffi::c_uchar; 2] = [0; 2];
    let mut outbuf: [::core::ffi::c_uchar; 2] = [0; 2];
    let mut inbytesleft: size_t = 2 as size_t;
    let mut outbytesleft: size_t = 2 as size_t;
    let mut p: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut q: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if cmap.is_null() {
        return cid as uint16_t;
    }
    inbuf[0 as ::core::ffi::c_int as usize] = (cid as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int)
        as ::core::ffi::c_uchar;
    inbuf[1 as ::core::ffi::c_int as usize] =
        (cid as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    p = &raw mut inbuf as *mut ::core::ffi::c_uchar;
    q = &raw mut outbuf as *mut ::core::ffi::c_uchar;
    CMap_decode_char(
        cmap,
        &raw mut p,
        &raw mut inbytesleft,
        &raw mut q,
        &raw mut outbytesleft,
    );
    if inbytesleft != 0 as size_t || outbytesleft != 0 as size_t {
        return 0 as uint16_t;
    }
    return ((outbuf[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | outbuf[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int) as uint16_t;
}
#[unsafe(export_name = "__pdf_impl_CIDFont_type2_dofont")]
pub unsafe extern "C-unwind" fn CIDFont_type2_dofont(
    mut font: *mut pdf_font,
) -> ::core::ffi::c_int {
    let mut fontfile: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut h_used_chars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v_used_chars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut used_chars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut glyphs: *mut tt_glyphs = ::core::ptr::null_mut::<tt_glyphs>();
    let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
    let mut ttcmap: *mut tt_cmap = ::core::ptr::null_mut::<tt_cmap>();
    let mut offset: SFNT_ULONG = 0 as SFNT_ULONG;
    let mut cid: CID = 0;
    let mut last_cid: CID = 0;
    let mut cidtogidmap: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut num_glyphs: USHORT = 0;
    let mut maptype: C2RustUnnamed_6 = via_cid_to_code;
    let mut i: ::core::ffi::c_int = 0;
    let mut unicode_cmap: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    if (*font).reference.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"FontDescriptor\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj((*font).descriptor),
    );
    if (*font).flags & PDF_FONT_FLAG_BASEFONT != 0 {
        return 0 as ::core::ffi::c_int;
    }
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    tmp = pdf_new_dict();
    pdf_add_dict(
        tmp,
        pdf_new_name(b"Registry\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_string(
            (*font).cid.csi.registry as *const ::core::ffi::c_void,
            strlen((*font).cid.csi.registry),
        ),
    );
    pdf_add_dict(
        tmp,
        pdf_new_name(b"Ordering\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_string(
            (*font).cid.csi.ordering as *const ::core::ffi::c_void,
            strlen((*font).cid.csi.ordering),
        ),
    );
    pdf_add_dict(
        tmp,
        pdf_new_name(b"Supplement\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number((*font).cid.csi.supplement as ::core::ffi::c_double),
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"CIDSystemInfo\0" as *const u8 as *const ::core::ffi::c_char),
        tmp,
    );
    if (*font).cid.options.embed == 0 && opt_flags_cidfont & CIDFONT_FORCE_FIXEDPITCH != 0 {
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"DW\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(1000.0f64),
        );
        return 0 as ::core::ffi::c_int;
    }
    handle = dpx_open_truetype_file((*font).filename);
    if handle == 0 {
        handle = dpx_open_dfont_file((*font).filename);
        if handle == 0 {
            pdf_warning!(
                b"Could not open TTF/dfont file: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*font).filename,
            );
            return -(1 as ::core::ffi::c_int);
        }
        sfont = dfont_open(handle, (*font).index as ::core::ffi::c_int);
    } else {
        sfont = sfnt_open(handle);
    }
    if sfont.is_null() {
        pdf_warning!(
            b"Could not open TTF file: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    match (*sfont).type_0 {
        SFNT_TYPE_TTC => {
            offset = ttc_read_offset(sfont, (*font).index as SFNT_ULONG);
            if offset == 0 as SFNT_ULONG {
                pdf_warning!(
                    b"Invalid TTC index in %s.\0" as *const u8 as *const ::core::ffi::c_char,
                    (*font).filename,
                );
                sfnt_close(sfont);
                crate::backend::runtime::input_close(handle);
                return -(1 as ::core::ffi::c_int);
            }
        }
        SFNT_TYPE_TRUETYPE => {
            if (*font).index > 0 as uint32_t {
                pdf_warning!(
                    b"Found TrueType font file while expecting TTC file (%s).\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*font).filename,
                );
                sfnt_close(sfont);
                crate::backend::runtime::input_close(handle);
                return -(1 as ::core::ffi::c_int);
            }
            offset = 0 as SFNT_ULONG;
        }
        SFNT_TYPE_DFONT => {
            offset = (*sfont).offset;
        }
        _ => {
            pdf_warning!(
                b"Not a TrueType/TTC font (%s)?\0" as *const u8 as *const ::core::ffi::c_char,
                (*font).filename,
            );
            sfnt_close(sfont);
            crate::backend::runtime::input_close(handle);
            return -(1 as ::core::ffi::c_int);
        }
    }
    if sfnt_read_table_directory(sfont, offset) < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Could not read TrueType table directory (%s).\0" as *const u8
                as *const ::core::ffi::c_char,
            (*font).filename,
        );
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if streq_ptr(
        (*font).cid.csi.registry,
        b"Adobe\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        && streq_ptr(
            (*font).cid.csi.ordering,
            b"Identity\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        maptype = glyph_ordering;
        ttcmap = ::core::ptr::null_mut::<tt_cmap>();
        cmap = ::core::ptr::null_mut::<CMap>();
    } else {
        if !(*font).cid.csi.registry.is_null() && !(*font).cid.csi.ordering.is_null() {
            let mut cmap_name: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut len: size_t = 0;
            let mut cmap_id: ::core::ffi::c_int = 0;
            len = strlen((*font).cid.csi.registry).wrapping_add(strlen((*font).cid.csi.ordering));
            len = len.wrapping_add(strlen((*font).fontname));
            len = len.wrapping_add(3 as size_t);
            cmap_name = new((len.wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            snprintf(
                cmap_name,
                len.wrapping_add(1 as size_t),
                b"%s-%s-%s\0" as *const u8 as *const ::core::ffi::c_char,
                (*font).cid.csi.registry,
                (*font).cid.csi.ordering,
                (*font).fontname,
            );
            cmap_id = CMap_cache_find(cmap_name);
            free(cmap_name as *mut ::core::ffi::c_void);
            if cmap_id >= 0 as ::core::ffi::c_int {
                cmap = CMap_cache_get(cmap_id);
                ttcmap = ::core::ptr::null_mut::<tt_cmap>();
                maptype = via_cid_to_gid;
            }
        }
        if maptype as ::core::ffi::c_uint
            != via_cid_to_gid as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            maptype = via_cid_to_code;
            i = 0 as ::core::ffi::c_int;
            while i <= KNOWN_ENCODINGS_MAX {
                ttcmap = tt_cmap_read(
                    sfont,
                    known_encodings[i as usize].platform as USHORT,
                    known_encodings[i as usize].encoding as USHORT,
                );
                if !ttcmap.is_null() {
                    break;
                }
                i += 1;
            }
            if ttcmap.is_null() {
                pdf_warning!(
                    b"No usable TrueType cmap table found for font \"%s\".\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*font).filename,
                );
                pdf_warning!(
                    b"CID character collection for this font is set to \"%s-%s\"\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*font).cid.csi.registry,
                    (*font).cid.csi.ordering,
                );
                pdf_warning!(
                    b"Cannot continue without this...\0" as *const u8 as *const ::core::ffi::c_char,
                );
                sfnt_close(sfont);
                crate::backend::runtime::input_close(handle);
                return -(1 as ::core::ffi::c_int);
            } else if i <= WIN_UCS_INDEX_MAX {
                unicode_cmap = 1 as ::core::ffi::c_int;
            } else {
                unicode_cmap = 0 as ::core::ffi::c_int;
            }
            if streq_ptr(
                (*font).cid.csi.ordering,
                b"UCS\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
                && i <= WIN_UCS_INDEX_MAX
            {
                cmap = ::core::ptr::null_mut::<CMap>();
            } else {
                cmap = find_tocode_cmap((*font).cid.csi.registry, (*font).cid.csi.ordering, i);
                if cmap.is_null() {
                    sfnt_close(sfont);
                    crate::backend::runtime::input_close(handle);
                    return -(1 as ::core::ffi::c_int);
                }
            }
        }
    }
    glyphs = tt_build_init();
    last_cid = 0 as CID;
    num_glyphs = 1 as USHORT;
    v_used_chars = ::core::ptr::null_mut::<::core::ffi::c_char>();
    h_used_chars = v_used_chars;
    used_chars = h_used_chars;
    let mut c: ::core::ffi::c_int = 0;
    h_used_chars = (*font).usedchars;
    v_used_chars = (*font).cid.usedchars_v;
    if !(!h_used_chars.is_null() || !v_used_chars.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"h_used_chars || v_used_chars\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    c = 0 as ::core::ffi::c_int;
    i = 8191 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        if !h_used_chars.is_null()
            && *h_used_chars.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            last_cid = (i * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as CID;
            c = *h_used_chars.offset(i as isize) as ::core::ffi::c_int;
            break;
        } else {
            i -= 1;
        }
    }
    i = 8191 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        if !v_used_chars.is_null()
            && *v_used_chars.offset(i as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            if i * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                >= last_cid as ::core::ffi::c_int
            {
                c = if i * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                    > last_cid as ::core::ffi::c_int
                {
                    *v_used_chars.offset(i as isize) as ::core::ffi::c_int
                } else {
                    c | *v_used_chars.offset(i as isize) as ::core::ffi::c_int
                };
                last_cid = (i * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as CID;
                break;
            }
        }
        i -= 1;
    }
    if last_cid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            if c >> i & 1 as ::core::ffi::c_int != 0 {
                break;
            }
            last_cid = last_cid.wrapping_sub(1);
            i += 1;
        }
    }
    if !((last_cid as ::core::ffi::c_uint) < 0xffff as ::core::ffi::c_uint) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"last_cid < 0xFFFFu\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    cidtogidmap = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !h_used_chars.is_null() {
        used_chars = h_used_chars;
        cid = 1 as CID;
        while cid as ::core::ffi::c_int <= last_cid as ::core::ffi::c_int {
            let mut code: int32_t = 0;
            let mut gid: uint16_t = 0 as uint16_t;
            if !(*h_used_chars
                .offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int)
                    << 7 as ::core::ffi::c_int
                        - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
                == 0)
            {
                match maptype as ::core::ffi::c_uint {
                    0 => {
                        gid = cid as uint16_t;
                        code = cid as int32_t;
                    }
                    2 => {
                        gid = cid_to_gid(cmap, cid);
                        code = cid as int32_t;
                    }
                    1 => {
                        code = cid_to_code(cmap, cid, unicode_cmap);
                        if code < 0 as int32_t {
                            pdf_warning!(
                                b"Unable to map CID to code: CID=%u\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                cid as ::core::ffi::c_int,
                            );
                        } else {
                            gid = tt_cmap_lookup(ttcmap, code as SFNT_ULONG) as uint16_t;
                            if gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                && unicode_cmap != 0
                            {
                                let mut alt_code: ::core::ffi::c_int = 0;
                                alt_code = fix_CJK_symbols(code as ::core::ffi::c_ushort)
                                    as ::core::ffi::c_int;
                                if alt_code as int32_t != code {
                                    gid =
                                        tt_cmap_lookup(ttcmap, alt_code as SFNT_ULONG) as uint16_t;
                                    if gid as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                        pdf_warning!(
                                            b"Unicode char U+%04x replaced with U+%04x.\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char,
                                            code,
                                            alt_code,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                if gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if code >= 0 as int32_t {
                        pdf_warning!(
                            b"Glyph missing in font. (CID=%u, code=0x%04x)\0" as *const u8
                                as *const ::core::ffi::c_char,
                            cid as ::core::ffi::c_int,
                            code,
                        );
                    }
                }
                gid = tt_add_glyph(glyphs, gid as USHORT, cid as USHORT) as uint16_t;
                num_glyphs = num_glyphs.wrapping_add(1);
            }
            cid = cid.wrapping_add(1);
        }
    }
    if !v_used_chars.is_null() {
        let mut gsub_list: *mut otl_gsub = ::core::ptr::null_mut::<otl_gsub>();
        if maptype as ::core::ffi::c_uint
            != via_cid_to_code as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            gsub_list = ::core::ptr::null_mut::<otl_gsub>();
        } else {
            gsub_list = otl_gsub_new();
            if otl_gsub_add_feat(
                gsub_list,
                b"*\0" as *const u8 as *const ::core::ffi::c_char,
                b"*\0" as *const u8 as *const ::core::ffi::c_char,
                b"vrt2\0" as *const u8 as *const ::core::ffi::c_char,
                sfont,
            ) < 0 as ::core::ffi::c_int
            {
                if otl_gsub_add_feat(
                    gsub_list,
                    b"*\0" as *const u8 as *const ::core::ffi::c_char,
                    b"*\0" as *const u8 as *const ::core::ffi::c_char,
                    b"vert\0" as *const u8 as *const ::core::ffi::c_char,
                    sfont,
                ) < 0 as ::core::ffi::c_int
                {
                    pdf_warning!(
                        b"GSUB feature vrt2/vert not found.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    otl_gsub_release(gsub_list);
                    gsub_list = ::core::ptr::null_mut::<otl_gsub>();
                } else {
                    otl_gsub_select(
                        gsub_list,
                        b"*\0" as *const u8 as *const ::core::ffi::c_char,
                        b"*\0" as *const u8 as *const ::core::ffi::c_char,
                        b"vert\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            } else {
                otl_gsub_select(
                    gsub_list,
                    b"*\0" as *const u8 as *const ::core::ffi::c_char,
                    b"*\0" as *const u8 as *const ::core::ffi::c_char,
                    b"vrt2\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        cid = 1 as CID;
        while cid as ::core::ffi::c_int <= last_cid as ::core::ffi::c_int {
            let mut code_0: int32_t = 0;
            let mut gid_0: uint16_t = 0 as uint16_t;
            if !(*v_used_chars
                .offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int)
                    << 7 as ::core::ffi::c_int
                        - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
                == 0)
            {
                if !(!h_used_chars.is_null()
                    && *h_used_chars
                        .offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int)
                            << 7 as ::core::ffi::c_int
                                - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
                        != 0)
                {
                    match maptype as ::core::ffi::c_uint {
                        0 => {
                            gid_0 = cid as uint16_t;
                            code_0 = cid as int32_t;
                        }
                        2 => {
                            gid_0 = cid_to_gid(cmap, cid);
                            code_0 = cid as int32_t;
                        }
                        1 => {
                            code_0 = cid_to_code(cmap, cid, unicode_cmap);
                            if code_0 < 0 as int32_t {
                                pdf_warning!(
                                    b"Unable to map CID to code: CID=%u\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    cid as ::core::ffi::c_int,
                                );
                            } else {
                                gid_0 = tt_cmap_lookup(ttcmap, code_0 as SFNT_ULONG) as uint16_t;
                                if gid_0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                    && unicode_cmap != 0
                                {
                                    let mut alt_code_0: ::core::ffi::c_int = 0;
                                    alt_code_0 = fix_CJK_symbols(code_0 as ::core::ffi::c_ushort)
                                        as ::core::ffi::c_int;
                                    if alt_code_0 as int32_t != code_0 {
                                        gid_0 = tt_cmap_lookup(ttcmap, alt_code_0 as SFNT_ULONG)
                                            as uint16_t;
                                        if gid_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                            pdf_warning!(
                                                b"Unicode char U+%04x replaced with U+%04x.\0"
                                                    as *const u8
                                                    as *const ::core::ffi::c_char,
                                                code_0,
                                                alt_code_0,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    if gid_0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        if code_0 >= 0 as int32_t {
                            pdf_warning!(
                                b"Glyph missing in font. (CID=%u, code=0x%04x)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                cid as ::core::ffi::c_int,
                                code_0,
                            );
                        }
                    } else if !gsub_list.is_null() {
                        otl_gsub_apply(gsub_list, &raw mut gid_0);
                    }
                    gid_0 = tt_add_glyph(glyphs, gid_0 as USHORT, cid as USHORT) as uint16_t;
                    if !used_chars.is_null() {
                        let ref mut fresh0 = *used_chars
                            .offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
                        *fresh0 = (*fresh0 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 7 as ::core::ffi::c_int
                                    - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    }
                    num_glyphs = num_glyphs.wrapping_add(1);
                }
            }
            cid = cid.wrapping_add(1);
        }
        if !gsub_list.is_null() {
            otl_gsub_release(gsub_list);
        }
        if used_chars.is_null() {
            used_chars = v_used_chars;
        }
    }
    if used_chars.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"used_chars\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    tt_cmap_release(ttcmap);
    if (*font).cid.options.embed != 0 {
        if tt_build_tables(sfont, glyphs) < 0 as ::core::ffi::c_int {
            pdf_warning!(
                b"Could not created FontFile stream.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !cidtogidmap.is_null() {
                free(cidtogidmap as *mut ::core::ffi::c_void);
            }
            sfnt_close(sfont);
            crate::backend::runtime::input_close(handle);
            return -(1 as ::core::ffi::c_int);
        }
        if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
            pdf_message!(
                b"[%u glyphs (Max CID: %u)]\0" as *const u8 as *const ::core::ffi::c_char,
                (*glyphs).num_glyphs as ::core::ffi::c_int,
                last_cid as ::core::ffi::c_int,
            );
        }
    } else if tt_get_metrics(sfont, glyphs) < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Reading glyph metrics failed...\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !cidtogidmap.is_null() {
            free(cidtogidmap as *mut ::core::ffi::c_void);
        }
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if opt_flags_cidfont & CIDFONT_FORCE_FIXEDPITCH != 0 {
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"DW\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(1000.0f64),
        );
    } else {
        add_TTCIDHMetrics(
            (*font).resource,
            glyphs,
            used_chars,
            cidtogidmap,
            last_cid as ::core::ffi::c_ushort,
        );
        if (*font).cid.need_vmetrics != 0 {
            add_TTCIDVMetrics(
                (*font).resource,
                glyphs,
                used_chars,
                last_cid as ::core::ffi::c_ushort,
            );
        }
    }
    if pdf_check_version(2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        let mut cidset: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut cidset_data: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        cidset_data = new(
            (((*glyphs).last_gid as ::core::ffi::c_int / 8 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        memset(
            cidset_data as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((*glyphs).last_gid as ::core::ffi::c_int / 8 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as size_t,
        );
        i = 1 as ::core::ffi::c_int;
        while i <= (*glyphs).last_gid as ::core::ffi::c_int {
            let ref mut fresh1 = *cidset_data.offset((i / 8 as ::core::ffi::c_int) as isize);
            *fresh1 = (*fresh1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int)
                    << 7 as ::core::ffi::c_int - i % 8 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
            i += 1;
        }
        cidset = pdf_new_stream(STREAM_COMPRESS);
        pdf_add_stream(
            cidset,
            cidset_data as *const ::core::ffi::c_void,
            (*glyphs).last_gid as ::core::ffi::c_int / 8 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int,
        );
        free(cidset_data as *mut ::core::ffi::c_void);
        pdf_add_dict(
            (*font).descriptor,
            pdf_new_name(b"CIDSet\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(cidset),
        );
        pdf_release_obj(cidset);
    }
    tt_build_finish(glyphs);
    if (*font).cid.options.embed == 0 {
        free(cidtogidmap as *mut ::core::ffi::c_void);
        sfnt_close(sfont);
        if handle != 0 {
            crate::backend::runtime::input_close(handle);
        }
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while !required_table[i as usize].name.is_null() {
        if sfnt_require_table(
            sfont,
            required_table[i as usize].name,
            required_table[i as usize].must_exist,
        ) < 0 as ::core::ffi::c_int
        {
            pdf_warning!(
                b"Some required TrueType table (%s) does not exist.\0" as *const u8
                    as *const ::core::ffi::c_char,
                required_table[i as usize].name,
            );
            free(cidtogidmap as *mut ::core::ffi::c_void);
            sfnt_close(sfont);
            crate::backend::runtime::input_close(handle);
            return -(1 as ::core::ffi::c_int);
        }
        i += 1;
    }
    fontfile = sfnt_create_FontFile_stream(sfont);
    sfnt_close(sfont);
    if handle != 0 {
        crate::backend::runtime::input_close(handle);
    }
    if fontfile.is_null() {
        pdf_warning!(
            b"Could not created FontFile stream for \"%s\".\0" as *const u8
                as *const ::core::ffi::c_char,
            (*font).filename,
        );
        free(cidtogidmap as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        pdf_message!(
            b"[%d bytes]\0" as *const u8 as *const ::core::ffi::c_char,
            pdf_stream_length(fontfile),
        );
    }
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"FontFile2\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj(fontfile),
    );
    pdf_release_obj(fontfile);
    if cidtogidmap.is_null() {
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"CIDToGIDMap\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"Identity\0" as *const u8 as *const ::core::ffi::c_char),
        );
    } else {
        let mut c2gmstream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        c2gmstream = pdf_new_stream(STREAM_COMPRESS);
        pdf_add_stream(
            c2gmstream,
            cidtogidmap as *const ::core::ffi::c_void,
            (last_cid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int,
        );
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"CIDToGIDMap\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(c2gmstream),
        );
        pdf_release_obj(c2gmstream);
        free(cidtogidmap as *mut ::core::ffi::c_void);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_CIDFont_type2_open")]
pub unsafe extern "C-unwind" fn CIDFont_type2_open(
    mut font: *mut pdf_font,
    mut name: *const ::core::ffi::c_char,
    mut index: ::core::ffi::c_int,
    mut opt: *mut cid_opt,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0;
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut offset: SFNT_ULONG = 0 as SFNT_ULONG;
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    if !(!font.is_null() && !opt.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font && opt\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    handle = dpx_open_truetype_file(name);
    if handle == 0 {
        handle = dpx_open_dfont_file(name);
        if handle == 0 {
            return -(1 as ::core::ffi::c_int);
        }
        sfont = dfont_open(handle, index);
    } else {
        sfont = sfnt_open(handle);
    }
    if sfont.is_null() {
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    match (*sfont).type_0 {
        SFNT_TYPE_TTC => {
            offset = ttc_read_offset(sfont, index as SFNT_ULONG);
        }
        SFNT_TYPE_TRUETYPE => {
            if index > 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Invalid TTC index (not TTC font): %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    name,
                );
                sfnt_close(sfont);
                crate::backend::runtime::input_close(handle);
                return -(1 as ::core::ffi::c_int);
            }
            offset = 0 as SFNT_ULONG;
        }
        SFNT_TYPE_DFONT => {
            offset = (*sfont).offset;
        }
        _ => {
            sfnt_close(sfont);
            if handle != 0 {
                crate::backend::runtime::input_close(handle);
            }
            return -(1 as ::core::ffi::c_int);
        }
    }
    if sfnt_read_table_directory(sfont, offset) < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Reading TrueType table directory failed.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if (*sfont).type_0 == SFNT_TYPE_TTC
        && sfnt_find_table_pos(sfont, b"CFF \0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        sfnt_close(sfont);
        if handle != 0 {
            crate::backend::runtime::input_close(handle);
        }
        return -(1 as ::core::ffi::c_int);
    }
    let mut shortname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut namelen: ::core::ffi::c_int = 0;
    shortname = new((127 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    namelen =
        tt_get_ps_fontname(sfont, shortname, PDF_NAME_LEN_MAX as USHORT) as ::core::ffi::c_int;
    if namelen == 0 as ::core::ffi::c_int {
        memset(
            shortname as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            PDF_NAME_LEN_MAX as size_t,
        );
        strncpy(shortname, name, PDF_NAME_LEN_MAX as size_t);
        namelen = strlen(shortname) as ::core::ffi::c_int;
    }
    error = validate_name(shortname, namelen);
    if error != 0 {
        free(shortname as *mut ::core::ffi::c_void);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    fontname = new(
        (strlen(shortname).wrapping_add(19 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy(fontname, shortname);
    free(shortname as *mut ::core::ffi::c_void);
    if (*opt).embed != 0 && (*opt).style != FONT_STYLE_NONE {
        pdf_warning!(
            b"Embedding disabled due to style option for %s.\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        );
        (*opt).embed = 0 as ::core::ffi::c_int;
    }
    match (*opt).style {
        FONT_STYLE_BOLD => {
            strcat(
                fontname,
                b",Bold\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        FONT_STYLE_ITALIC => {
            strcat(
                fontname,
                b",Italic\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        FONT_STYLE_BOLDITALIC => {
            strcat(
                fontname,
                b",BoldItalic\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        _ => {}
    }
    (*font).fontname = fontname;
    (*font).subtype = PDF_FONT_FONTTYPE_CIDTYPE2;
    if !(*opt).csi.registry.is_null() && !(*opt).csi.ordering.is_null() {
        (*font).cid.csi.registry = new((strlen((*opt).csi.registry).wrapping_add(1 as size_t)
            as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        strcpy((*font).cid.csi.registry, (*opt).csi.registry);
        (*font).cid.csi.ordering = new((strlen((*opt).csi.ordering).wrapping_add(1 as size_t)
            as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        strcpy((*font).cid.csi.ordering, (*opt).csi.ordering);
        (*font).cid.csi.supplement = (*opt).csi.supplement;
    }
    (*font).resource = pdf_new_dict();
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Font\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"CIDFontType2\0" as *const u8 as *const ::core::ffi::c_char),
    );
    (*font).descriptor = tt_get_fontdesc(
        sfont,
        &raw mut (*opt).embed,
        (*opt).stemv,
        0 as ::core::ffi::c_int,
        name,
    );
    if (*font).descriptor.is_null() {
        pdf_warning!(
            b"Could not obtain necessary font info.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if (*opt).embed != 0 {
        let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        pdf_font_make_uniqueTag(&raw mut (*font).uniqueID as *mut ::core::ffi::c_char);
        tmp = new(
            (strlen(fontname).wrapping_add(8 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        sprintf(
            tmp,
            b"%s+%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*font).uniqueID as *mut ::core::ffi::c_char,
            (*font).fontname,
        );
        pdf_add_dict(
            (*font).descriptor,
            pdf_new_name(b"FontName\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(tmp),
        );
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"BaseFont\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(tmp),
        );
        free(tmp as *mut ::core::ffi::c_void);
    } else {
        pdf_add_dict(
            (*font).descriptor,
            pdf_new_name(b"FontName\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name((*font).fontname),
        );
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"BaseFont\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name((*font).fontname),
        );
    }
    sfnt_close(sfont);
    if handle != 0 {
        crate::backend::runtime::input_close(handle);
    }
    return 0 as ::core::ffi::c_int;
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_type2_dofont(
    mut font: *mut pdf_font,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_type2_dofont(font) }) {
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_type2_open(
    mut font: *mut pdf_font,
    mut name: *const ::core::ffi::c_char,
    mut index: ::core::ffi::c_int,
    mut opt: *mut cid_opt,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        CIDFont_type2_open(font, name, index, opt)
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
