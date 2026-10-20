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
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strcat(__s1: *mut ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcpy(__dst: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strstr(__big: *const ::core::ffi::c_char, __little: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: ::core::ffi::c_int) -> size_t;
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut ::core::ffi::c_char, len: size_t) -> ssize_t;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn sprintf(_: *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
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
    #[link_name = "__pdf_impl_pdf_array_length"]
    fn pdf_array_length(array: *mut pdf_obj) -> ::core::ffi::c_uint;
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_stream"]
    fn pdf_new_stream(flags: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_stream"]
    fn pdf_add_stream(stream: *mut pdf_obj, stream_data_ptr: *const ::core::ffi::c_void, stream_data_len: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_stream_dict"]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_check_version"]
    fn pdf_check_version(major: ::core::ffi::c_int, minor: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_make_uniqueTag"]
    fn pdf_font_make_uniqueTag(tag: *mut ::core::ffi::c_char);
    #[link_name = "__pdf_impl_agl_chop_suffix"]
    fn agl_chop_suffix(glyphname: *const ::core::ffi::c_char, suffix: *mut *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_agl_sput_UTF16BE"]
    fn agl_sput_UTF16BE(
        name: *const ::core::ffi::c_char, dstpp: *mut *mut ::core::ffi::c_uchar, limptr: *mut ::core::ffi::c_uchar,
        num_fails: *mut ::core::ffi::c_int,
    ) -> int32_t;
    #[link_name = "__pdf_impl_agl_name_is_unicode"]
    fn agl_name_is_unicode(glyphname: *const ::core::ffi::c_char) -> bool;
    #[link_name = "__pdf_impl_agl_name_convert_unicode"]
    fn agl_name_convert_unicode(glyphname: *const ::core::ffi::c_char) -> int32_t;
    #[link_name = "__pdf_impl_agl_lookup_list"]
    fn agl_lookup_list(glyphname: *const ::core::ffi::c_char) -> *mut agl_name;
    static mut work_buffer: [::core::ffi::c_char; 0];
    #[link_name = "__pdf_impl_cff_open"]
    fn cff_open(handle: rust_input_handle_t, offset: ::core::ffi::c_int, idx: ::core::ffi::c_int) -> *mut cff_font;
    #[link_name = "__pdf_impl_cff_close"]
    fn cff_close(cff: *mut cff_font);
    #[link_name = "__pdf_impl_cff_put_header"]
    fn cff_put_header(cff: *mut cff_font, dest: *mut card8, destlen: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_get_index_header"]
    fn cff_get_index_header(cff: *mut cff_font) -> *mut cff_index;
    #[link_name = "__pdf_impl_cff_release_index"]
    fn cff_release_index(idx: *mut cff_index);
    #[link_name = "__pdf_impl_cff_new_index"]
    fn cff_new_index(count: card16) -> *mut cff_index;
    #[link_name = "__pdf_impl_cff_index_size"]
    fn cff_index_size(idx: *mut cff_index) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_pack_index"]
    fn cff_pack_index(idx: *mut cff_index, dest: *mut card8, destlen: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_get_name"]
    fn cff_get_name(cff: *mut cff_font) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_cff_set_name"]
    fn cff_set_name(cff: *mut cff_font, name: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_read_subrs"]
    fn cff_read_subrs(cff: *mut cff_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_read_charsets"]
    fn cff_read_charsets(cff: *mut cff_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_pack_charsets"]
    fn cff_pack_charsets(cff: *mut cff_font, dest: *mut card8, destlen: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_glyph_lookup"]
    fn cff_glyph_lookup(cff: *mut cff_font, glyph: *const ::core::ffi::c_char) -> card16;
    #[link_name = "__pdf_impl_cff_charsets_lookup"]
    fn cff_charsets_lookup(cff: *mut cff_font, cid: card16) -> card16;
    #[link_name = "__pdf_impl_cff_release_charsets"]
    fn cff_release_charsets(charset: *mut cff_charsets);
    #[link_name = "__pdf_impl_cff_charsets_lookup_inverse"]
    fn cff_charsets_lookup_inverse(cff: *mut cff_font, gid: card16) -> card16;
    #[link_name = "__pdf_impl_cff_read_fdselect"]
    fn cff_read_fdselect(cff: *mut cff_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_pack_fdselect"]
    fn cff_pack_fdselect(cff: *mut cff_font, dest: *mut card8, destlen: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_fdselect_lookup"]
    fn cff_fdselect_lookup(cff: *mut cff_font, gid: card16) -> card8;
    #[link_name = "__pdf_impl_cff_release_fdselect"]
    fn cff_release_fdselect(fdselect: *mut cff_fdselect);
    #[link_name = "__pdf_impl_cff_read_fdarray"]
    fn cff_read_fdarray(cff: *mut cff_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_read_private"]
    fn cff_read_private(cff: *mut cff_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_get_string"]
    fn cff_get_string(cff: *mut cff_font, id: s_SID) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_cff_get_sid"]
    fn cff_get_sid(cff: *mut cff_font, str: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_add_string"]
    fn cff_add_string(cff: *mut cff_font, str: *const ::core::ffi::c_char, unique: ::core::ffi::c_int) -> s_SID;
    #[link_name = "__pdf_impl_cff_update_string"]
    fn cff_update_string(cff: *mut cff_font);
    #[link_name = "__pdf_impl_cff_new_dict"]
    fn cff_new_dict() -> *mut cff_dict;
    #[link_name = "__pdf_impl_cff_dict_set"]
    fn cff_dict_set(dict: *mut cff_dict, key: *const ::core::ffi::c_char, idx: ::core::ffi::c_int, value: ::core::ffi::c_double);
    #[link_name = "__pdf_impl_cff_dict_get"]
    fn cff_dict_get(dict: *mut cff_dict, key: *const ::core::ffi::c_char, idx: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_cff_dict_add"]
    fn cff_dict_add(dict: *mut cff_dict, key: *const ::core::ffi::c_char, count: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_cff_dict_remove"]
    fn cff_dict_remove(dict: *mut cff_dict, key: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_cff_dict_known"]
    fn cff_dict_known(dict: *mut cff_dict, key: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_dict_pack"]
    fn cff_dict_pack(dict: *mut cff_dict, dest: *mut card8, destlen: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_dict_update"]
    fn cff_dict_update(dict: *mut cff_dict, cff: *mut cff_font);
    static mut CSI_IDENTITY: CIDSysInfo;
    static mut CSI_UNICODE: CIDSysInfo;
    static mut opt_flags_cidfont: ::core::ffi::c_int;
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
    #[link_name = "__pdf_impl_CMap_cache_find"]
    fn CMap_cache_find(cmap_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_cache_add"]
    fn CMap_cache_add(cmap: *mut CMap) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_create_stream"]
    fn CMap_create_stream(cmap: *mut CMap) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_cs_copy_charstring"]
    fn cs_copy_charstring(
        dest: *mut card8, destlen: ::core::ffi::c_int, src: *mut card8, srclen: ::core::ffi::c_int, gsubr: *mut cff_index, subr: *mut cff_index,
        default_width: ::core::ffi::c_double, nominal_width: ::core::ffi::c_double, ginfo: *mut cs_ginfo,
    ) -> ::core::ffi::c_int;
    static mut dpx_conf: _dpx_conf;
    #[link_name = "__pdf_impl_dpx_open_type1_file"]
    fn dpx_open_type1_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_dpx_open_truetype_file"]
    fn dpx_open_truetype_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_dpx_open_opentype_file"]
    fn dpx_open_opentype_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_sfnt_open"]
    fn sfnt_open(handle: rust_input_handle_t) -> *mut sfnt;
    #[link_name = "__pdf_impl_sfnt_close"]
    fn sfnt_close(sfont: *mut sfnt);
    #[link_name = "__pdf_impl_sfnt_read_table_directory"]
    fn sfnt_read_table_directory(sfont: *mut sfnt, offset: SFNT_ULONG) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_sfnt_find_table_pos"]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_sfnt_locate_table"]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_t1char_get_metrics"]
    fn t1char_get_metrics(src: *mut card8, srclen: ::core::ffi::c_int, subrs: *mut cff_index, ginfo: *mut t1_ginfo) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_t1char_convert_charstring"]
    fn t1char_convert_charstring(
        dst: *mut card8, dstlen: ::core::ffi::c_int, src: *mut card8, srclen: ::core::ffi::c_int, subrs: *mut cff_index,
        default_width: ::core::ffi::c_double, nominal_width: ::core::ffi::c_double, ginfo: *mut t1_ginfo,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_t1_load_font"]
    fn t1_load_font(enc_vec: *mut *mut ::core::ffi::c_char, mode: ::core::ffi::c_int, handle: rust_input_handle_t) -> *mut cff_font;
    #[link_name = "__pdf_impl_ttc_read_offset"]
    fn ttc_read_offset(sfont: *mut sfnt, ttc_idx: SFNT_ULONG) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_tt_get_fontdesc"]
    fn tt_get_fontdesc(
        sfont: *mut sfnt, embed: *mut ::core::ffi::c_int, stemv: ::core::ffi::c_int, type_0: ::core::ffi::c_int, fontname: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_tt_read_head_table"]
    fn tt_read_head_table(sfont: *mut sfnt) -> *mut tt_head_table;
    #[link_name = "__pdf_impl_tt_read_hhea_table"]
    fn tt_read_hhea_table(sfont: *mut sfnt) -> *mut tt_hhea_table;
    #[link_name = "__pdf_impl_tt_read_maxp_table"]
    fn tt_read_maxp_table(sfont: *mut sfnt) -> *mut tt_maxp_table;
    #[link_name = "__pdf_impl_tt_read_vhea_table"]
    fn tt_read_vhea_table(sfont: *mut sfnt) -> *mut tt_vhea_table;
    #[link_name = "__pdf_impl_tt_read_VORG_table"]
    fn tt_read_VORG_table(sfont: *mut sfnt) -> *mut tt_VORG_table;
    #[link_name = "__pdf_impl_tt_read_longMetrics"]
    fn tt_read_longMetrics(sfont: *mut sfnt, numGlyphs: USHORT, numLongMetrics: USHORT, numExSideBearings: USHORT) -> *mut tt_longMetrics;
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
    pub type0: C2RustUnnamed_0,
    pub cid: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub csi: CIDSysInfo,
    pub options: cid_opt,
    pub need_vmetrics: ::core::ffi::c_int,
    pub usedchars_v: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
    pub data: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
    pub data: C2RustUnnamed_3,
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
pub union C2RustUnnamed_3 {
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
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub keep_cache: ::core::ffi::c_int,
}
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cs_ginfo {
    pub flags: ::core::ffi::c_int,
    pub wx: ::core::ffi::c_double,
    pub wy: ::core::ffi::c_double,
    pub bbox: C2RustUnnamed_6,
    pub seac: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub asb: ::core::ffi::c_double,
    pub adx: ::core::ffi::c_double,
    pub ady: ::core::ffi::c_double,
    pub bchar: card8,
    pub achar: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub llx: ::core::ffi::c_double,
    pub lly: ::core::ffi::c_double,
    pub urx: ::core::ffi::c_double,
    pub ury: ::core::ffi::c_double,
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
pub type SHORT = ::core::ffi::c_short;
pub type FWord = ::core::ffi::c_short;
pub type BYTE = ::core::ffi::c_uchar;
pub type Fixed = uint32_t;
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
pub type uFWord = ::core::ffi::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_VORG_table {
    pub defaultVertOriginY: SHORT,
    pub numVertOriginYMetrics: USHORT,
    pub vertOriginYMetrics: *mut tt_vertOriginYMetrics,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_vertOriginYMetrics {
    pub glyphIndex: USHORT,
    pub vertOriginY: SHORT,
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
pub type SFNT_CHAR = ::core::ffi::c_schar;
pub type CID = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_8,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: ::core::ffi::c_int,
    pub profile: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub struct C2RustUnnamed_8 {
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
pub struct agl_name {
    pub name: *mut ::core::ffi::c_char,
    pub suffix: *mut ::core::ffi::c_char,
    pub n_components: ::core::ffi::c_int,
    pub unicodes: [int32_t; 16],
    pub alternate: *mut agl_name,
    pub is_predef: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_ginfo {
    pub use_seac: ::core::ffi::c_int,
    pub wx: ::core::ffi::c_double,
    pub wy: ::core::ffi::c_double,
    pub bbox: C2RustUnnamed_10,
    pub seac: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub asb: ::core::ffi::c_double,
    pub adx: ::core::ffi::c_double,
    pub ady: ::core::ffi::c_double,
    pub bchar: card8,
    pub achar: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub llx: ::core::ffi::c_double,
    pub lly: ::core::ffi::c_double,
    pub urx: ::core::ffi::c_double,
    pub ury: ::core::ffi::c_double,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
pub const FONTMAP_STYLE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FONTMAP_STYLE_BOLD: ::core::ffi::c_int = 1;
pub const FONTMAP_STYLE_ITALIC: ::core::ffi::c_int = 2;
pub const FONTMAP_STYLE_BOLDITALIC: ::core::ffi::c_int = 3;
pub const CS_STR_LEN_MAX: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const STREAM_COMPRESS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PDF_FONT_FONTTYPE_CIDTYPE0: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PDF_FONT_FLAG_BASEFONT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const CIDFONT_FLAG_TYPE1: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const CIDFONT_FLAG_TYPE1C: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
pub const FONT_STYLE_NONE: ::core::ffi::c_int = FONTMAP_STYLE_NONE;
pub const FONT_STYLE_BOLD: ::core::ffi::c_int = FONTMAP_STYLE_BOLD;
pub const FONT_STYLE_ITALIC: ::core::ffi::c_int = FONTMAP_STYLE_ITALIC;
pub const FONT_STYLE_BOLDITALIC: ::core::ffi::c_int = FONTMAP_STYLE_BOLDITALIC;
pub const WORK_BUFFER_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const FONTTYPE_CIDFONT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const CFF_NOMINALWIDTHX_DEFAULT: ::core::ffi::c_double = 0.0f64;
pub const CFF_DEFAULTWIDTHX_DEFAULT: ::core::ffi::c_double = 0.0f64;
pub const CIDFONT_FORCE_FIXEDPITCH: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const CID_MAX_CID: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const CID_MAX: ::core::ffi::c_int = CID_MAX_CID;
pub const CMAP_TYPE_CODE_TO_CID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CMAP_TYPE_TO_UNICODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SFNT_TYPE_POSTSCRIPT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const SFNT_TYPE_TTC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn add_CIDHMetrics(
    mut sfont: *mut sfnt, mut fontdict: *mut pdf_obj, mut CIDToGIDMap: *mut ::core::ffi::c_uchar, mut last_cid: ::core::ffi::c_ushort,
    mut maxp: *mut tt_maxp_table, mut head: *mut tt_head_table, mut hmtx: *mut tt_longMetrics,
) {
    let mut w_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut an_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cid: ::core::ffi::c_int = 0;
    let mut start: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut prev: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut defaultAdvanceWidth: ::core::ffi::c_double = 0.;
    let mut empty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    defaultAdvanceWidth = floor(
        1000.0f64 * (*hmtx.offset(0 as ::core::ffi::c_int as isize)).advance as ::core::ffi::c_int as ::core::ffi::c_double
            / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
            / 1 as ::core::ffi::c_int as ::core::ffi::c_double
            + 0.5f64,
    ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    w_array = pdf_new_array();
    cid = 0 as ::core::ffi::c_int;
    while cid <= last_cid as ::core::ffi::c_int {
        let mut gid: USHORT = 0;
        let mut advanceWidth: ::core::ffi::c_double = 0.;
        gid = (if !CIDToGIDMap.is_null() {
            (*CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid) as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | *CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
        } else {
            cid
        }) as USHORT;
        if !(gid as ::core::ffi::c_int >= (*maxp).numGlyphs as ::core::ffi::c_int
            || cid != 0 as ::core::ffi::c_int && gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            advanceWidth = floor(
                1000.0f64 * (*hmtx.offset(gid as isize)).advance as ::core::ffi::c_int as ::core::ffi::c_double
                    / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    + 0.5f64,
            ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            if advanceWidth == defaultAdvanceWidth {
                if !an_array.is_null() {
                    pdf_add_array(w_array, pdf_new_number(start as ::core::ffi::c_double));
                    pdf_add_array(w_array, an_array);
                    an_array = ::core::ptr::null_mut::<pdf_obj>();
                    empty = 0 as ::core::ffi::c_int;
                }
            } else {
                if cid != prev + 1 as ::core::ffi::c_int && !an_array.is_null() {
                    pdf_add_array(w_array, pdf_new_number(start as ::core::ffi::c_double));
                    pdf_add_array(w_array, an_array);
                    an_array = ::core::ptr::null_mut::<pdf_obj>();
                    empty = 0 as ::core::ffi::c_int;
                }
                if an_array.is_null() {
                    an_array = pdf_new_array();
                    start = cid;
                }
                pdf_add_array(an_array, pdf_new_number(advanceWidth));
                prev = cid;
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
        pdf_new_number(defaultAdvanceWidth),
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
unsafe extern "C-unwind" fn add_CIDVMetrics(
    mut sfont: *mut sfnt, mut fontdict: *mut pdf_obj, mut CIDToGIDMap: *mut ::core::ffi::c_uchar, mut last_cid: ::core::ffi::c_ushort,
    mut maxp: *mut tt_maxp_table, mut head: *mut tt_head_table, mut hmtx: *mut tt_longMetrics,
) {
    let mut w2_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut an_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cid: ::core::ffi::c_int = 0;
    let mut vorg: *mut tt_VORG_table = ::core::ptr::null_mut::<tt_VORG_table>();
    let mut vhea: *mut tt_vhea_table = ::core::ptr::null_mut::<tt_vhea_table>();
    let mut vmtx: *mut tt_longMetrics = ::core::ptr::null_mut::<tt_longMetrics>();
    let mut defaultAdvanceHeight: ::core::ffi::c_double = 0.;
    let mut defaultVertOriginY: ::core::ffi::c_double = 0.;
    let mut empty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if sfnt_find_table_pos(sfont, b"VORG\0" as *const u8 as *const ::core::ffi::c_char) <= 0 as SFNT_ULONG {
        return;
    }
    vorg = tt_read_VORG_table(sfont);
    defaultVertOriginY = floor(
        1000.0f64 * (*vorg).defaultVertOriginY as ::core::ffi::c_int as ::core::ffi::c_double
            / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
            / 1 as ::core::ffi::c_int as ::core::ffi::c_double
            + 0.5f64,
    ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    if sfnt_find_table_pos(sfont, b"vhea\0" as *const u8 as *const ::core::ffi::c_char) > 0 as SFNT_ULONG {
        vhea = tt_read_vhea_table(sfont);
    }
    if !vhea.is_null() && sfnt_find_table_pos(sfont, b"vmtx\0" as *const u8 as *const ::core::ffi::c_char) > 0 as SFNT_ULONG {
        sfnt_locate_table(sfont, b"vmtx\0" as *const u8 as *const ::core::ffi::c_char);
        vmtx = tt_read_longMetrics(sfont, (*maxp).numGlyphs, (*vhea).numOfLongVerMetrics, (*vhea).numOfExSideBearings);
    }
    if sfnt_find_table_pos(sfont, b"OS/2\0" as *const u8 as *const ::core::ffi::c_char) <= 0 as SFNT_ULONG {
        let mut os2: *mut tt_os2__table = ::core::ptr::null_mut::<tt_os2__table>();
        os2 = tt_read_os2__table(sfont);
        defaultVertOriginY = floor(
            1000.0f64 * (*os2).sTypoAscender as ::core::ffi::c_int as ::core::ffi::c_double
                / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                + 0.5f64,
        ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        defaultAdvanceHeight = floor(
            1000.0f64 * ((*os2).sTypoAscender as ::core::ffi::c_int - (*os2).sTypoDescender as ::core::ffi::c_int) as ::core::ffi::c_double
                / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                + 0.5f64,
        ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        free(os2 as *mut ::core::ffi::c_void);
    } else {
        defaultAdvanceHeight = 1000 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    w2_array = pdf_new_array();
    cid = 0 as ::core::ffi::c_int;
    while cid <= last_cid as ::core::ffi::c_int {
        let mut i: USHORT = 0;
        let mut gid: USHORT = 0;
        let mut advanceHeight: ::core::ffi::c_double = 0.;
        let mut vertOriginX: ::core::ffi::c_double = 0.;
        let mut vertOriginY: ::core::ffi::c_double = 0.;
        gid = (if !CIDToGIDMap.is_null() {
            (*CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid) as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | *CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
        } else {
            cid
        }) as USHORT;
        if !(gid as ::core::ffi::c_int >= (*maxp).numGlyphs as ::core::ffi::c_int
            || cid != 0 as ::core::ffi::c_int && gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
        {
            advanceHeight = if !vmtx.is_null() {
                floor(
                    1000.0f64 * (*vmtx.offset(gid as isize)).advance as ::core::ffi::c_int as ::core::ffi::c_double
                        / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                        / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        + 0.5f64,
                ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                defaultAdvanceHeight
            };
            vertOriginX = floor(
                1000.0f64 * ((*hmtx.offset(gid as isize)).advance as ::core::ffi::c_int as ::core::ffi::c_double * 0.5f64)
                    / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    + 0.5f64,
            ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
            vertOriginY = defaultVertOriginY;
            i = 0 as USHORT;
            while (i as ::core::ffi::c_int) < (*vorg).numVertOriginYMetrics as ::core::ffi::c_int
                && gid as ::core::ffi::c_int >= (*(*vorg).vertOriginYMetrics.offset(i as isize)).glyphIndex as ::core::ffi::c_int
            {
                if gid as ::core::ffi::c_int == (*(*vorg).vertOriginYMetrics.offset(i as isize)).glyphIndex as ::core::ffi::c_int {
                    vertOriginY = floor(
                        1000.0f64 * (*(*vorg).vertOriginYMetrics.offset(i as isize)).vertOriginY as ::core::ffi::c_int as ::core::ffi::c_double
                            / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                            / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                            + 0.5f64,
                    ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                }
                i = i.wrapping_add(1);
            }
            if vertOriginY != defaultVertOriginY || advanceHeight != defaultAdvanceHeight {
                pdf_add_array(w2_array, pdf_new_number(cid as ::core::ffi::c_double));
                pdf_add_array(w2_array, pdf_new_number(cid as ::core::ffi::c_double));
                pdf_add_array(w2_array, pdf_new_number(-advanceHeight));
                pdf_add_array(w2_array, pdf_new_number(vertOriginX));
                pdf_add_array(w2_array, pdf_new_number(vertOriginY));
                empty = 0 as ::core::ffi::c_int;
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
        pdf_add_dict(fontdict, pdf_new_name(b"DW2\0" as *const u8 as *const ::core::ffi::c_char), an_array);
    }
    if empty == 0 {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"W2\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(w2_array),
        );
    }
    pdf_release_obj(w2_array);
    free((*vorg).vertOriginYMetrics as *mut ::core::ffi::c_void);
    free(vorg as *mut ::core::ffi::c_void);
    free(vmtx as *mut ::core::ffi::c_void);
    free(vhea as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn add_CIDMetrics(
    mut sfont: *mut sfnt, mut fontdict: *mut pdf_obj, mut CIDToGIDMap: *mut ::core::ffi::c_uchar, mut last_cid: ::core::ffi::c_ushort,
    mut need_vmetrics: ::core::ffi::c_int,
) {
    let mut hmtx: *mut tt_longMetrics = ::core::ptr::null_mut::<tt_longMetrics>();
    let mut head: *mut tt_head_table = ::core::ptr::null_mut::<tt_head_table>();
    let mut hhea: *mut tt_hhea_table = ::core::ptr::null_mut::<tt_hhea_table>();
    let mut maxp: *mut tt_maxp_table = ::core::ptr::null_mut::<tt_maxp_table>();
    head = tt_read_head_table(sfont);
    maxp = tt_read_maxp_table(sfont);
    hhea = tt_read_hhea_table(sfont);
    sfnt_locate_table(sfont, b"hmtx\0" as *const u8 as *const ::core::ffi::c_char);
    hmtx = tt_read_longMetrics(sfont, (*maxp).numGlyphs, (*hhea).numOfLongHorMetrics, (*hhea).numOfExSideBearings);
    add_CIDHMetrics(sfont, fontdict, CIDToGIDMap, last_cid, maxp, head, hmtx);
    if need_vmetrics != 0 {
        add_CIDVMetrics(sfont, fontdict, CIDToGIDMap, last_cid, maxp, head, hmtx);
    }
    free(hmtx as *mut ::core::ffi::c_void);
    free(hhea as *mut ::core::ffi::c_void);
    free(maxp as *mut ::core::ffi::c_void);
    free(head as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn write_fontfile(mut font: *mut pdf_font, mut cffont: *mut cff_font) -> ::core::ffi::c_int {
    let mut topdict: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut fdarray: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut private: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut dest: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut destlen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0;
    let mut topdict_offset: ::core::ffi::c_int = 0;
    let mut fdarray_offset: ::core::ffi::c_int = 0;
    topdict = cff_new_index(1 as card16);
    fdarray = cff_new_index((*cffont).num_fds as card16);
    private = cff_new_index((*cffont).num_fds as card16);
    cff_dict_remove((*cffont).topdict, b"UniqueID\0" as *const u8 as *const ::core::ffi::c_char);
    cff_dict_remove((*cffont).topdict, b"XUID\0" as *const u8 as *const ::core::ffi::c_char);
    cff_dict_remove((*cffont).topdict, b"Private\0" as *const u8 as *const ::core::ffi::c_char);
    cff_dict_remove((*cffont).topdict, b"Encoding\0" as *const u8 as *const ::core::ffi::c_char);
    *(*topdict).offset.offset(1 as ::core::ffi::c_int as isize) = (cff_dict_pack(
        (*cffont).topdict,
        &raw mut work_buffer as *mut ::core::ffi::c_char as *mut card8,
        WORK_BUFFER_SIZE,
    ) + 1 as ::core::ffi::c_int) as l_offset;
    i = 0 as ::core::ffi::c_int;
    while i < (*cffont).num_fds as ::core::ffi::c_int {
        size = 0 as ::core::ffi::c_int;
        if !(*cffont).private.is_null() && !(*(*cffont).private.offset(i as isize)).is_null() {
            size = cff_dict_pack(
                *(*cffont).private.offset(i as isize),
                &raw mut work_buffer as *mut ::core::ffi::c_char as *mut card8,
                WORK_BUFFER_SIZE,
            );
            if size < 1 as ::core::ffi::c_int {
                cff_dict_remove(
                    *(*cffont).fdarray.offset(i as isize),
                    b"Private\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        *(*private).offset.offset((i + 1 as ::core::ffi::c_int) as isize) = (*(*private).offset.offset(i as isize)).wrapping_add(size as l_offset);
        *(*fdarray).offset.offset((i + 1 as ::core::ffi::c_int) as isize) = (*(*fdarray).offset.offset(i as isize)).wrapping_add(cff_dict_pack(
            *(*cffont).fdarray.offset(i as isize),
            &raw mut work_buffer as *mut ::core::ffi::c_char as *mut card8,
            WORK_BUFFER_SIZE,
        )
            as l_offset);
        i += 1;
    }
    destlen = 4 as ::core::ffi::c_int;
    destlen += cff_set_name(cffont, (*font).fontname);
    destlen += cff_index_size(topdict);
    destlen += cff_index_size((*cffont).string);
    destlen += cff_index_size((*cffont).gsubr);
    destlen += (*(*cffont).charsets).num_entries as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    destlen += (*(*cffont).fdselect).num_entries as ::core::ffi::c_int * 3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int;
    destlen += cff_index_size((*cffont).cstrings);
    destlen += cff_index_size(fdarray);
    destlen = (destlen as l_offset).wrapping_add((*(*private).offset.offset((*private).count as isize)).wrapping_sub(1 as l_offset))
        as ::core::ffi::c_int as ::core::ffi::c_int;
    dest = new((destlen as uint32_t as usize).wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t) as *mut card8
        as *mut ::core::ffi::c_uchar;
    offset = 0 as ::core::ffi::c_int;
    offset += cff_put_header(cffont, dest.offset(offset as isize), destlen - offset);
    offset += cff_pack_index((*cffont).name, dest.offset(offset as isize), destlen - offset);
    topdict_offset = offset;
    offset += cff_index_size(topdict);
    offset += cff_pack_index((*cffont).string, dest.offset(offset as isize), destlen - offset);
    offset += cff_pack_index((*cffont).gsubr, dest.offset(offset as isize), destlen - offset);
    cff_dict_set(
        (*cffont).topdict,
        b"charset\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        offset as ::core::ffi::c_double,
    );
    offset += cff_pack_charsets(cffont, dest.offset(offset as isize), destlen - offset);
    cff_dict_set(
        (*cffont).topdict,
        b"FDSelect\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        offset as ::core::ffi::c_double,
    );
    offset += cff_pack_fdselect(cffont, dest.offset(offset as isize), destlen - offset);
    cff_dict_set(
        (*cffont).topdict,
        b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        offset as ::core::ffi::c_double,
    );
    offset += cff_pack_index((*cffont).cstrings, dest.offset(offset as isize), cff_index_size((*cffont).cstrings));
    cff_release_index((*cffont).cstrings);
    (*cffont).cstrings = ::core::ptr::null_mut::<cff_index>();
    cff_dict_set(
        (*cffont).topdict,
        b"FDArray\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        offset as ::core::ffi::c_double,
    );
    fdarray_offset = offset;
    offset += cff_index_size(fdarray);
    (*fdarray).data = new(
        ((*(*fdarray).offset.offset((*fdarray).count as isize)).wrapping_sub(1 as l_offset) as usize)
            .wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t,
    ) as *mut card8;
    i = 0 as ::core::ffi::c_int;
    while i < (*cffont).num_fds as ::core::ffi::c_int {
        size = (*(*private).offset.offset((i + 1 as ::core::ffi::c_int) as isize)).wrapping_sub(*(*private).offset.offset(i as isize))
            as ::core::ffi::c_int;
        if !(*(*cffont).private.offset(i as isize)).is_null() && size > 0 as ::core::ffi::c_int {
            cff_dict_pack(*(*cffont).private.offset(i as isize), dest.offset(offset as isize), size);
            cff_dict_set(
                *(*cffont).fdarray.offset(i as isize),
                b"Private\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                size as ::core::ffi::c_double,
            );
            cff_dict_set(
                *(*cffont).fdarray.offset(i as isize),
                b"Private\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                offset as ::core::ffi::c_double,
            );
        }
        cff_dict_pack(
            *(*cffont).fdarray.offset(i as isize),
            (*fdarray)
                .data
                .offset(*(*fdarray).offset.offset(i as isize) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
            (*(*fdarray).offset.offset((*fdarray).count as isize)).wrapping_sub(1 as l_offset) as ::core::ffi::c_int,
        );
        offset += size;
        i += 1;
    }
    cff_pack_index(fdarray, dest.offset(fdarray_offset as isize), cff_index_size(fdarray));
    cff_release_index(fdarray);
    cff_release_index(private);
    (*topdict).data = new(
        ((*(*topdict).offset.offset((*topdict).count as isize)).wrapping_sub(1 as l_offset) as usize)
            .wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t,
    ) as *mut card8;
    cff_dict_pack(
        (*cffont).topdict,
        (*topdict).data,
        (*(*topdict).offset.offset((*topdict).count as isize)).wrapping_sub(1 as l_offset) as ::core::ffi::c_int,
    );
    cff_pack_index(topdict, dest.offset(topdict_offset as isize), cff_index_size(topdict));
    cff_release_index(topdict);
    let mut fontfile: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    fontfile = pdf_new_stream(STREAM_COMPRESS);
    stream_dict = pdf_stream_dict(fontfile);
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"FontFile3\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj(fontfile),
    );
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"CIDFontType0C\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_stream(fontfile, dest as *mut ::core::ffi::c_char as *const ::core::ffi::c_void, offset);
    pdf_release_obj(fontfile);
    free(dest as *mut ::core::ffi::c_void);
    return destlen;
}
unsafe extern "C-unwind" fn CIDFont_type0_add_CIDSet(mut font: *mut pdf_font, mut used_chars: *mut ::core::ffi::c_char, mut last_cid: card16) {
    let mut cidset: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    cidset = pdf_new_stream(STREAM_COMPRESS);
    pdf_add_stream(
        cidset,
        used_chars as *const ::core::ffi::c_void,
        last_cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"CIDSet\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj(cidset),
    );
    pdf_release_obj(cidset);
}
#[unsafe(export_name = "__pdf_impl_CIDFont_type0_dofont")]
pub unsafe extern "C-unwind" fn CIDFont_type0_dofont(mut font: *mut pdf_font) -> ::core::ffi::c_int {
    let mut handle: rust_input_handle_t = 0;
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut charstrings: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut idx: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut charset: *mut cff_charsets = ::core::ptr::null_mut::<cff_charsets>();
    let mut fdselect: *mut cff_fdselect = ::core::ptr::null_mut::<cff_fdselect>();
    let mut charstring_len: ::core::ffi::c_int = 0;
    let mut max_len: ::core::ffi::c_int = 0;
    let mut destlen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut data: *mut card8 = ::core::ptr::null_mut::<card8>();
    let mut num_glyphs: card16 = 0 as card16;
    let mut gid: card16 = 0;
    let mut cid: ::core::ffi::c_int = 0;
    let mut cs_count: card16 = 0;
    let mut last_cid: card16 = 0 as card16;
    let mut fd: ::core::ffi::c_int = 0;
    let mut prev_fd: ::core::ffi::c_int = 0;
    let mut used_chars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut CIDToGIDMap: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"font\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
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
    } else if (*font).cid.options.embed == 0 && opt_flags_cidfont & CIDFONT_FORCE_FIXEDPITCH != 0 {
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"DW\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(1000.0f64),
        );
        return 0 as ::core::ffi::c_int;
    }
    used_chars = (*font).usedchars;
    handle = dpx_open_opentype_file((*font).filename);
    if handle == 0 {
        handle = dpx_open_truetype_file((*font).filename);
        if handle == 0 {
            pdf_warning!(b"Could not open file: %s\0" as *const u8 as *const ::core::ffi::c_char, (*font).filename,);
            return -(1 as ::core::ffi::c_int);
        }
    }
    sfont = sfnt_open(handle);
    if sfont.is_null() {
        pdf_warning!(
            b"Failed to read font file: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if (*sfont).type_0 == SFNT_TYPE_TTC {
        offset = ttc_read_offset(sfont, (*font).index as SFNT_ULONG) as ::core::ffi::c_int;
    }
    if (*sfont).type_0 != SFNT_TYPE_TTC && (*sfont).type_0 != SFNT_TYPE_POSTSCRIPT
        || sfnt_read_table_directory(sfont, offset as SFNT_ULONG) < 0 as ::core::ffi::c_int
        || {
            offset = sfnt_find_table_pos(sfont, b"CFF \0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int;
            offset == 0 as ::core::ffi::c_int
        }
    {
        pdf_warning!(
            b"Not a CFF/OpenType font: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    cffont = cff_open((*sfont).handle, offset, 0 as ::core::ffi::c_int);
    if cffont.is_null() {
        pdf_warning!(
            b"Failed to read CFF font data: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if (*cffont).flag & FONTTYPE_CIDFONT == 0 {
        pdf_warning!(
            b"Unexpected type (CIDFont expected): %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    cff_read_charsets(cffont);
    let mut cid_count: ::core::ffi::c_int = 0;
    if cff_dict_known((*cffont).topdict, b"CIDCount\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        cid_count = cff_dict_get(
            (*cffont).topdict,
            b"CIDCount\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
    } else {
        cid_count = CID_MAX + 1 as ::core::ffi::c_int;
    }
    CIDToGIDMap = new(((2 as ::core::ffi::c_int * cid_count) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize) as uint32_t) as *mut ::core::ffi::c_uchar;
    memset(
        CIDToGIDMap as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (2 as ::core::ffi::c_int * cid_count) as size_t,
    );
    let ref mut fresh0 = *used_chars.offset((0 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
    *fresh0 = (*fresh0 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int % 8 as ::core::ffi::c_int)
        as ::core::ffi::c_char;
    cid = 0 as ::core::ffi::c_int;
    while cid <= CID_MAX {
        if *used_chars.offset((cid / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid % 8 as ::core::ffi::c_int
            != 0
        {
            gid = cff_charsets_lookup(cffont, cid as card16);
            if cid != 0 as ::core::ffi::c_int && gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Glyph for CID %u missing in font \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                    cid as CID as ::core::ffi::c_int,
                    (*font).filename,
                );
                let ref mut fresh1 = *used_chars.offset((cid / 8 as ::core::ffi::c_int) as isize);
                *fresh1 = (*fresh1 as ::core::ffi::c_int & !((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid % 8 as ::core::ffi::c_int))
                    as ::core::ffi::c_char;
            } else {
                *CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid) as isize) =
                    (gid as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                *CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid + 1 as ::core::ffi::c_int) as isize) =
                    (gid as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                last_cid = cid as card16;
                num_glyphs = num_glyphs.wrapping_add(1);
            }
        }
        cid += 1;
    }
    if opt_flags_cidfont & CIDFONT_FORCE_FIXEDPITCH != 0 {
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"DW\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(1000.0f64),
        );
    } else {
        add_CIDMetrics(
            sfont,
            (*font).resource,
            CIDToGIDMap,
            last_cid as ::core::ffi::c_ushort,
            if (*font).cid.need_vmetrics != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            },
        );
    }
    if (*font).cid.options.embed == 0 {
        free(CIDToGIDMap as *mut ::core::ffi::c_void);
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return 0 as ::core::ffi::c_int;
    }
    cff_read_fdselect(cffont);
    cff_read_fdarray(cffont);
    cff_read_private(cffont);
    cff_read_subrs(cffont);
    offset = cff_dict_get(
        (*cffont).topdict,
        b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    crate::backend::runtime::input_seek((*cffont).handle, (*cffont).offset.wrapping_add(offset as l_offset) as ssize_t, SEEK_SET);
    idx = cff_get_index_header(cffont);
    offset = crate::backend::runtime::input_seek((*cffont).handle, 0 as ssize_t, SEEK_CUR) as ::core::ffi::c_int;
    cs_count = (*idx).count;
    if (cs_count as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        pdf_warning!(
            b"No valid charstring data found: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    charset = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_charsets>() as usize) as uint32_t)
        as *mut cff_charsets;
    (*charset).format = 0 as card8;
    (*charset).num_entries = 0 as card16;
    (*charset).data.glyphs = new((num_glyphs as uint32_t as usize).wrapping_mul(::core::mem::size_of::<s_SID>() as usize) as uint32_t) as *mut s_SID;
    fdselect = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_fdselect>() as usize) as uint32_t)
        as *mut cff_fdselect;
    (*fdselect).format = 3 as card8;
    (*fdselect).num_entries = 0 as card16;
    (*fdselect).data.ranges =
        new((num_glyphs as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_range3>() as usize) as uint32_t) as *mut cff_range3;
    charstrings = cff_new_index((num_glyphs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as card16);
    max_len = 2 as ::core::ffi::c_int * CS_STR_LEN_MAX;
    (*charstrings).data = new((max_len as uint32_t as usize).wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t) as *mut card8;
    charstring_len = 0 as ::core::ffi::c_int;
    prev_fd = -(1 as ::core::ffi::c_int);
    gid = 0 as card16;
    data = new((65536 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t) as *mut card8;
    cid = 0 as ::core::ffi::c_int;
    while cid <= last_cid as ::core::ffi::c_int {
        let mut gid_org: ::core::ffi::c_ushort = 0;
        if !(*used_chars.offset((cid / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid % 8 as ::core::ffi::c_int
            == 0)
        {
            gid_org = ((*CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid) as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | *CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
                as ::core::ffi::c_ushort;
            size = (*(*idx).offset.offset((gid_org as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(*(*idx).offset.offset(gid_org as isize)) as ::core::ffi::c_int;
            if size > CS_STR_LEN_MAX {
                pdf_warning!(
                    b"Charstring too long: %s (gid=%u)\0" as *const u8 as *const ::core::ffi::c_char,
                    (*font).filename,
                    gid_org as ::core::ffi::c_int,
                );
                free(data as *mut ::core::ffi::c_void);
                free(charstrings as *mut ::core::ffi::c_void);
                free(fdselect as *mut ::core::ffi::c_void);
                free(charset as *mut ::core::ffi::c_void);
                cff_release_index(idx);
                free(CIDToGIDMap as *mut ::core::ffi::c_void);
                cff_close(cffont);
                sfnt_close(sfont);
                crate::backend::runtime::input_close(handle);
                return -(1 as ::core::ffi::c_int);
            }
            if charstring_len + CS_STR_LEN_MAX >= max_len {
                max_len = charstring_len + 2 as ::core::ffi::c_int * CS_STR_LEN_MAX;
                (*charstrings).data = renew(
                    (*charstrings).data as *mut ::core::ffi::c_void,
                    (max_len as uint32_t as usize).wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t,
                ) as *mut card8;
            }
            *(*charstrings).offset.offset(gid as isize) = (charstring_len + 1 as ::core::ffi::c_int) as l_offset;
            crate::backend::runtime::input_seek(
                (*cffont).handle,
                (offset as l_offset)
                    .wrapping_add(*(*idx).offset.offset(gid_org as isize))
                    .wrapping_sub(1 as l_offset) as ssize_t,
                SEEK_SET,
            );
            ttstub_input_read((*cffont).handle, data as *mut ::core::ffi::c_char, size as size_t);
            fd = cff_fdselect_lookup(cffont, gid_org as card16) as ::core::ffi::c_int;
            charstring_len += cs_copy_charstring(
                (*charstrings).data.offset(charstring_len as isize),
                max_len - charstring_len,
                data,
                size,
                (*cffont).gsubr,
                *(*cffont).subrs.offset(fd as isize),
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                ::core::ptr::null_mut::<cs_ginfo>(),
            );
            if cid > 0 as ::core::ffi::c_int && gid_org as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *(*charset).data.glyphs.offset((*charset).num_entries as isize) = cid as s_SID;
                (*charset).num_entries = ((*charset).num_entries as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as card16;
            }
            if fd != prev_fd {
                (*(*fdselect).data.ranges.offset((*fdselect).num_entries as isize)).first = gid;
                (*(*fdselect).data.ranges.offset((*fdselect).num_entries as isize)).fd = fd as card8;
                (*fdselect).num_entries = ((*fdselect).num_entries as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as card16;
                prev_fd = fd;
            }
            gid = gid.wrapping_add(1);
        }
        cid += 1;
    }
    if gid as ::core::ffi::c_int != num_glyphs as ::core::ffi::c_int {
        pdf_warning!(b"Unexpected error: %s\0" as *const u8 as *const ::core::ffi::c_char, (*font).filename,);
        free(data as *mut ::core::ffi::c_void);
        free(charstrings as *mut ::core::ffi::c_void);
        free(fdselect as *mut ::core::ffi::c_void);
        free(charset as *mut ::core::ffi::c_void);
        cff_release_index(idx);
        free(CIDToGIDMap as *mut ::core::ffi::c_void);
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    free(data as *mut ::core::ffi::c_void);
    cff_release_index(idx);
    free(CIDToGIDMap as *mut ::core::ffi::c_void);
    *(*charstrings).offset.offset(num_glyphs as isize) = (charstring_len + 1 as ::core::ffi::c_int) as l_offset;
    (*charstrings).count = num_glyphs;
    (*cffont).num_glyphs = num_glyphs;
    (*cffont).cstrings = charstrings;
    cff_release_charsets((*cffont).charsets);
    (*cffont).charsets = charset;
    cff_release_fdselect((*cffont).fdselect);
    (*cffont).fdselect = fdselect;
    if !(*cffont).gsubr.is_null() {
        cff_release_index((*cffont).gsubr);
    }
    (*cffont).gsubr = cff_new_index(0 as card16);
    fd = 0 as ::core::ffi::c_int;
    while fd < (*cffont).num_fds as ::core::ffi::c_int {
        if !(*cffont).subrs.is_null() && !(*(*cffont).subrs.offset(fd as isize)).is_null() {
            cff_release_index(*(*cffont).subrs.offset(fd as isize));
            let ref mut fresh2 = *(*cffont).subrs.offset(fd as isize);
            *fresh2 = ::core::ptr::null_mut::<cff_index>();
        }
        if !(*cffont).private.is_null() && !(*(*cffont).private.offset(fd as isize)).is_null() {
            cff_dict_remove(
                *(*cffont).private.offset(fd as isize),
                b"Subrs\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        fd += 1;
    }
    destlen = write_fontfile(font, cffont);
    cff_close(cffont);
    sfnt_close(sfont);
    crate::backend::runtime::input_close(handle);
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        pdf_message!(
            b"[%u/%u glyphs][%d bytes]\0" as *const u8 as *const ::core::ffi::c_char,
            num_glyphs as ::core::ffi::c_int,
            cs_count as ::core::ffi::c_int,
            destlen,
        );
    }
    if pdf_check_version(2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_CIDFont_type0_open_from_t1")]
pub unsafe extern "C-unwind" fn CIDFont_type0_open_from_t1(
    mut font: *mut pdf_font, mut name: *const ::core::ffi::c_char, mut index: ::core::ffi::c_int, mut opt: *mut cid_opt,
) -> ::core::ffi::c_int {
    let mut csi: CIDSysInfo = CIDSysInfo {
        registry: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ordering: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        supplement: 0,
    };
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut handle: rust_input_handle_t = 0;
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"font\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    handle = dpx_open_type1_file(name);
    if handle == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    cffont = t1_load_font(::core::ptr::null_mut::<*mut ::core::ffi::c_char>(), 1 as ::core::ffi::c_int, handle);
    if cffont.is_null() {
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    let mut shortname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fontname_len: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    shortname = cff_get_name(cffont);
    if shortname.is_null() {
        pdf_warning!(b"No valid FontName found: %s\0" as *const u8 as *const ::core::ffi::c_char, name,);
        cff_close(cffont);
        crate::backend::runtime::input_close(handle);
    }
    fontname = new((strlen(shortname).wrapping_add(fontname_len as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    memset(
        fontname as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        strlen(shortname).wrapping_add(fontname_len as size_t),
    );
    strcpy(fontname, shortname);
    free(shortname as *mut ::core::ffi::c_void);
    csi.registry = new(
        (strlen(b"Adobe\0" as *const u8 as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy(csi.registry, b"Adobe\0" as *const u8 as *const ::core::ffi::c_char);
    csi.ordering = new(
        (strlen(b"Identity\0" as *const u8 as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy(csi.ordering, b"Identity\0" as *const u8 as *const ::core::ffi::c_char);
    csi.supplement = 0 as ::core::ffi::c_int;
    if (*opt).style != FONT_STYLE_NONE {
        pdf_warning!(b",Bold, ,Italic, ... not supported for this type of font...\0" as *const u8 as *const ::core::ffi::c_char,);
        (*opt).style = FONT_STYLE_NONE;
    }
    (*font).fontname = fontname;
    (*font).subtype = PDF_FONT_FONTTYPE_CIDTYPE0;
    (*font).cid.csi = csi;
    (*font).flags |= CIDFONT_FLAG_TYPE1;
    (*font).resource = pdf_new_dict();
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Font\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"CIDFontType0\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_font_make_uniqueTag(&raw mut (*font).uniqueID as *mut ::core::ffi::c_char);
    (*font).descriptor = pdf_new_dict();
    let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    tmp = new((strlen((*font).fontname).wrapping_add(8 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
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
    let mut csi_dict: *mut pdf_obj = pdf_new_dict();
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Registry\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_string(csi.registry as *const ::core::ffi::c_void, strlen(csi.registry)),
    );
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Ordering\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_string(csi.ordering as *const ::core::ffi::c_void, strlen(csi.ordering)),
    );
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Supplement\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(csi.supplement as ::core::ffi::c_double),
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"CIDSystemInfo\0" as *const u8 as *const ::core::ffi::c_char),
        csi_dict,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_CIDFont_type0_open")]
pub unsafe extern "C-unwind" fn CIDFont_type0_open(
    mut font: *mut pdf_font, mut name: *const ::core::ffi::c_char, mut index: ::core::ffi::c_int, mut opt: *mut cid_opt,
) -> ::core::ffi::c_int {
    let mut csi: CIDSysInfo = CIDSysInfo {
        registry: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ordering: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        supplement: 0,
    };
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut handle: rust_input_handle_t = 0;
    let mut offset: SFNT_ULONG = 0 as SFNT_ULONG;
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"font\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    handle = dpx_open_opentype_file(name);
    if handle == 0 {
        handle = dpx_open_truetype_file(name);
        if handle == 0 {
            return -(1 as ::core::ffi::c_int);
        }
    }
    sfont = sfnt_open(handle);
    if sfont.is_null() {
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if (*sfont).type_0 == SFNT_TYPE_TTC {
        offset = ttc_read_offset(sfont, index as SFNT_ULONG);
    }
    if (*sfont).type_0 != SFNT_TYPE_TTC && (*sfont).type_0 != SFNT_TYPE_POSTSCRIPT
        || sfnt_read_table_directory(sfont, offset) < 0 as ::core::ffi::c_int
        || {
            offset = sfnt_find_table_pos(sfont, b"CFF \0" as *const u8 as *const ::core::ffi::c_char);
            offset == 0 as SFNT_ULONG
        }
    {
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    cffont = cff_open((*sfont).handle, offset as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    if cffont.is_null() {
        pdf_warning!(b"Cannot read CFF font data\0" as *const u8 as *const ::core::ffi::c_char,);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if (*cffont).flag & FONTTYPE_CIDFONT == 0 {
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    let mut shortname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fontname_len: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    shortname = cff_get_name(cffont);
    if shortname.is_null() {
        pdf_warning!(b"No valid FontName found.\0" as *const u8 as *const ::core::ffi::c_char,);
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    fontname_len += 11 as ::core::ffi::c_int;
    fontname = new((strlen(shortname).wrapping_add(fontname_len as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    memset(
        fontname as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        strlen(shortname).wrapping_add(fontname_len as size_t),
    );
    strcpy(fontname, shortname);
    free(shortname as *mut ::core::ffi::c_void);
    csi.registry = cff_get_string(
        cffont,
        cff_dict_get(
            (*cffont).topdict,
            b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) as s_SID,
    );
    csi.ordering = cff_get_string(
        cffont,
        cff_dict_get(
            (*cffont).topdict,
            b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        ) as s_SID,
    );
    csi.supplement = cff_dict_get(
        (*cffont).topdict,
        b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    cff_close(cffont);
    if (*opt).embed != 0 && (*opt).style != FONT_STYLE_NONE {
        pdf_warning!(
            b"Embedding disabled due to style option for %s.\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        (*opt).embed = 0 as ::core::ffi::c_int;
        match (*opt).style {
            FONT_STYLE_BOLD => {
                strcat(fontname, b",Bold\0" as *const u8 as *const ::core::ffi::c_char);
            }
            FONT_STYLE_ITALIC => {
                strcat(fontname, b",Italic\0" as *const u8 as *const ::core::ffi::c_char);
            }
            FONT_STYLE_BOLDITALIC => {
                strcat(fontname, b",BoldItalic\0" as *const u8 as *const ::core::ffi::c_char);
            }
            _ => {}
        }
    }
    (*font).descriptor = tt_get_fontdesc(sfont, &raw mut (*opt).embed, (*opt).stemv, 0 as ::core::ffi::c_int, name);
    if (*font).descriptor.is_null() {
        pdf_warning!(
            b"Could not obtain necessary font info: %s\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        free(fontname as *mut ::core::ffi::c_void);
        free(csi.registry as *mut ::core::ffi::c_void);
        free(csi.ordering as *mut ::core::ffi::c_void);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    (*font).fontname = fontname;
    (*font).subtype = PDF_FONT_FONTTYPE_CIDTYPE0;
    (*font).cid.csi = csi;
    (*font).resource = pdf_new_dict();
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Font\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"CIDFontType0\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if (*opt).embed != 0 {
        let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = new((strlen((*font).fontname).wrapping_add(8 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
        pdf_font_make_uniqueTag(&raw mut (*font).uniqueID as *mut ::core::ffi::c_char);
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
    let mut csi_dict: *mut pdf_obj = pdf_new_dict();
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Registry\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_string(csi.registry as *const ::core::ffi::c_void, strlen(csi.registry)),
    );
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Ordering\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_string(csi.ordering as *const ::core::ffi::c_void, strlen(csi.ordering)),
    );
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Supplement\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(csi.supplement as ::core::ffi::c_double),
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"CIDSystemInfo\0" as *const u8 as *const ::core::ffi::c_char),
        csi_dict,
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"DW\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(1000 as ::core::ffi::c_int as ::core::ffi::c_double),
    );
    sfnt_close(sfont);
    crate::backend::runtime::input_close(handle);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_CIDFont_type0_open_from_t1c")]
pub unsafe extern "C-unwind" fn CIDFont_type0_open_from_t1c(
    mut font: *mut pdf_font, mut name: *const ::core::ffi::c_char, mut index: ::core::ffi::c_int, mut opt: *mut cid_opt,
) -> ::core::ffi::c_int {
    let mut csi: CIDSysInfo = CIDSysInfo {
        registry: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ordering: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        supplement: 0,
    };
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut handle: rust_input_handle_t = 0;
    let mut offset: SFNT_ULONG = 0 as SFNT_ULONG;
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"font\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    handle = dpx_open_opentype_file(name);
    if handle == 0 {
        handle = dpx_open_truetype_file(name);
        if handle == 0 {
            return -(1 as ::core::ffi::c_int);
        }
    }
    sfont = sfnt_open(handle);
    if sfont.is_null() {
        pdf_warning!(b"Not a CFF/OpenType font: %s\0" as *const u8 as *const ::core::ffi::c_char, name,);
        crate::backend::runtime::input_close(handle);
    }
    if (*sfont).type_0 == SFNT_TYPE_TTC {
        offset = ttc_read_offset(sfont, index as SFNT_ULONG);
    }
    if (*sfont).type_0 != SFNT_TYPE_TTC && (*sfont).type_0 != SFNT_TYPE_POSTSCRIPT
        || sfnt_read_table_directory(sfont, offset) < 0 as ::core::ffi::c_int
        || {
            offset = sfnt_find_table_pos(sfont, b"CFF \0" as *const u8 as *const ::core::ffi::c_char);
            offset == 0 as SFNT_ULONG
        }
    {
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    cffont = cff_open((*sfont).handle, offset as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    if cffont.is_null() {
        pdf_warning!(b"Cannot read CFF font data: %s\0" as *const u8 as *const ::core::ffi::c_char, name,);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if (*cffont).flag & FONTTYPE_CIDFONT != 0 {
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    let mut shortname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fontname_len: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    shortname = cff_get_name(cffont);
    if shortname.is_null() {
        pdf_warning!(b"No valid FontName found: %s\0" as *const u8 as *const ::core::ffi::c_char, name,);
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    fontname = new((strlen(shortname).wrapping_add(fontname_len as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    memset(
        fontname as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        strlen(shortname).wrapping_add(fontname_len as size_t),
    );
    strcpy(fontname, shortname);
    free(shortname as *mut ::core::ffi::c_void);
    csi.registry = new(
        (strlen(b"Adobe\0" as *const u8 as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy(csi.registry, b"Adobe\0" as *const u8 as *const ::core::ffi::c_char);
    csi.ordering = new(
        (strlen(b"Identity\0" as *const u8 as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy(csi.ordering, b"Identity\0" as *const u8 as *const ::core::ffi::c_char);
    csi.supplement = 0 as ::core::ffi::c_int;
    cff_close(cffont);
    (*opt).embed = 1 as ::core::ffi::c_int;
    (*font).descriptor = tt_get_fontdesc(sfont, &raw mut (*opt).embed, (*opt).stemv, 0 as ::core::ffi::c_int, name);
    if (*font).descriptor.is_null() {
        pdf_warning!(
            b"Could not obtain necessary font info: %s\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        free(fontname as *mut ::core::ffi::c_void);
        free(csi.registry as *mut ::core::ffi::c_void);
        free(csi.ordering as *mut ::core::ffi::c_void);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    } else if (*opt).embed == 0 {
        pdf_warning!(
            b"Can\xE2\x80\x99t embed font due to font license: %s\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        free(fontname as *mut ::core::ffi::c_void);
        free(csi.registry as *mut ::core::ffi::c_void);
        free(csi.ordering as *mut ::core::ffi::c_void);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    (*font).fontname = fontname;
    (*font).subtype = PDF_FONT_FONTTYPE_CIDTYPE0;
    (*font).cid.csi = csi;
    (*font).flags |= CIDFONT_FLAG_TYPE1C;
    (*font).resource = pdf_new_dict();
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Font\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"CIDFontType0\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if (*opt).embed != 0 {
        let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = new((strlen((*font).fontname).wrapping_add(8 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
        pdf_font_make_uniqueTag(&raw mut (*font).uniqueID as *mut ::core::ffi::c_char);
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
    let mut csi_dict: *mut pdf_obj = pdf_new_dict();
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Registry\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_string(csi.registry as *const ::core::ffi::c_void, strlen(csi.registry)),
    );
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Ordering\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_string(csi.ordering as *const ::core::ffi::c_void, strlen(csi.ordering)),
    );
    pdf_add_dict(
        csi_dict,
        pdf_new_name(b"Supplement\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(csi.supplement as ::core::ffi::c_double),
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"CIDSystemInfo\0" as *const u8 as *const ::core::ffi::c_char),
        csi_dict,
    );
    sfnt_close(sfont);
    crate::backend::runtime::input_close(handle);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_CIDFont_type0_t1cdofont")]
pub unsafe extern "C-unwind" fn CIDFont_type0_t1cdofont(mut font: *mut pdf_font) -> ::core::ffi::c_int {
    let mut handle: rust_input_handle_t = 0;
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut charstrings: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut idx: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut charstring_len: ::core::ffi::c_int = 0;
    let mut max_len: ::core::ffi::c_int = 0;
    let mut destlen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut data: *mut card8 = ::core::ptr::null_mut::<card8>();
    let mut num_glyphs: card16 = 0;
    let mut gid: card16 = 0;
    let mut last_cid: card16 = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut cid: ::core::ffi::c_int = 0;
    let mut used_chars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut default_width: ::core::ffi::c_double = 0.;
    let mut nominal_width: ::core::ffi::c_double = 0.;
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"font\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if (*font).reference.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"FontDescriptor\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj((*font).descriptor),
    );
    used_chars = (*font).usedchars;
    handle = dpx_open_opentype_file((*font).filename);
    if handle == 0 {
        handle = dpx_open_truetype_file((*font).filename);
        if handle == 0 {
            pdf_warning!(b"Could not open file: %s\0" as *const u8 as *const ::core::ffi::c_char, (*font).filename,);
            return -(1 as ::core::ffi::c_int);
        }
    }
    sfont = sfnt_open(handle);
    if sfont.is_null() {
        pdf_warning!(
            b"Failed to read font data: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if (*sfont).type_0 == SFNT_TYPE_TTC {
        offset = ttc_read_offset(sfont, (*font).index as SFNT_ULONG) as ::core::ffi::c_int;
    }
    if (*sfont).type_0 != SFNT_TYPE_TTC && (*sfont).type_0 != SFNT_TYPE_POSTSCRIPT
        || sfnt_read_table_directory(sfont, offset as SFNT_ULONG) < 0 as ::core::ffi::c_int
        || {
            offset = sfnt_find_table_pos(sfont, b"CFF \0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int;
            offset == 0 as ::core::ffi::c_int
        }
    {
        pdf_warning!(
            b"Not a CFF/OpenType font: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    cffont = cff_open((*sfont).handle, offset, 0 as ::core::ffi::c_int);
    if cffont.is_null() {
        pdf_warning!(
            b"Failed to read CFF font data: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if (*cffont).flag & FONTTYPE_CIDFONT != 0 {
        pdf_warning!(
            b"Unxpected type (CIDFont) found: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    cff_read_private(cffont);
    cff_read_subrs(cffont);
    if !(*(*cffont).private.offset(0 as ::core::ffi::c_int as isize)).is_null()
        && cff_dict_known(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"StdVW\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        let mut stemv: ::core::ffi::c_double = 0.;
        stemv = cff_dict_get(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"StdVW\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        pdf_add_dict(
            (*font).descriptor,
            pdf_new_name(b"StemV\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(stemv),
        );
    }
    if !(*(*cffont).private.offset(0 as ::core::ffi::c_int as isize)).is_null()
        && cff_dict_known(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        default_width = cff_dict_get(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
    } else {
        default_width = CFF_DEFAULTWIDTHX_DEFAULT;
    }
    if !(*(*cffont).private.offset(0 as ::core::ffi::c_int as isize)).is_null()
        && cff_dict_known(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        nominal_width = cff_dict_get(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
    } else {
        nominal_width = CFF_NOMINALWIDTHX_DEFAULT;
    }
    num_glyphs = 0 as card16;
    last_cid = 0 as card16;
    let ref mut fresh6 = *used_chars.offset((0 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
    *fresh6 = (*fresh6 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int % 8 as ::core::ffi::c_int)
        as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < ((*cffont).num_glyphs as ::core::ffi::c_int + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        c = *used_chars.offset(i as isize) as ::core::ffi::c_int;
        j = 7 as ::core::ffi::c_int;
        while j >= 0 as ::core::ffi::c_int {
            if c & (1 as ::core::ffi::c_int) << j != 0 {
                num_glyphs = num_glyphs.wrapping_add(1);
                last_cid = ((i + 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int - j - 1 as ::core::ffi::c_int) as card16;
            }
            j -= 1;
        }
        i += 1;
    }
    let mut fdselect: *mut cff_fdselect = ::core::ptr::null_mut::<cff_fdselect>();
    fdselect = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_fdselect>() as usize) as uint32_t)
        as *mut cff_fdselect;
    (*fdselect).format = 3 as card8;
    (*fdselect).num_entries = 1 as card16;
    (*fdselect).data.ranges =
        new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_range3>() as usize) as uint32_t)
            as *mut cff_range3;
    (*(*fdselect).data.ranges.offset(0 as ::core::ffi::c_int as isize)).first = 0 as card16;
    (*(*fdselect).data.ranges.offset(0 as ::core::ffi::c_int as isize)).fd = 0 as card8;
    (*cffont).fdselect = fdselect;
    let mut charset: *mut cff_charsets = ::core::ptr::null_mut::<cff_charsets>();
    charset = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_charsets>() as usize) as uint32_t)
        as *mut cff_charsets;
    (*charset).format = 0 as card8;
    (*charset).num_entries = (num_glyphs as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as card16;
    (*charset).data.glyphs = new(((num_glyphs as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<s_SID>() as usize) as uint32_t) as *mut s_SID;
    gid = 0 as card16;
    cid = 0 as ::core::ffi::c_int;
    while cid <= last_cid as ::core::ffi::c_int {
        if *used_chars.offset((cid / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid % 8 as ::core::ffi::c_int
            != 0
        {
            if gid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *(*charset)
                    .data
                    .glyphs
                    .offset((gid as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) = cid as s_SID;
            }
            gid = gid.wrapping_add(1);
        }
        cid += 1;
    }
    (*cffont).charsets = charset;
    cff_dict_add(
        (*cffont).topdict,
        b"CIDCount\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"CIDCount\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        (last_cid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
    );
    (*cffont).fdarray = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<*mut cff_dict>() as usize) as uint32_t)
        as *mut *mut cff_dict;
    let ref mut fresh7 = *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize);
    *fresh7 = cff_new_dict();
    cff_dict_add(
        *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize),
        b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize),
        b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        cff_add_string(cffont, (*font).fontname.offset(7 as ::core::ffi::c_int as isize), 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
    );
    cff_dict_add(
        *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize),
        b"Private\0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_int,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize),
        b"Private\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0.0f64,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize),
        b"Private\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0.0f64,
    );
    cff_dict_add(
        (*cffont).topdict,
        b"FDArray\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"FDArray\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0.0f64,
    );
    cff_dict_add(
        (*cffont).topdict,
        b"FDSelect\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"FDSelect\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0.0f64,
    );
    cff_dict_remove((*cffont).topdict, b"UniqueID\0" as *const u8 as *const ::core::ffi::c_char);
    cff_dict_remove((*cffont).topdict, b"XUID\0" as *const u8 as *const ::core::ffi::c_char);
    cff_dict_remove((*cffont).topdict, b"Private\0" as *const u8 as *const ::core::ffi::c_char);
    cff_dict_remove((*cffont).topdict, b"Encoding\0" as *const u8 as *const ::core::ffi::c_char);
    offset = cff_dict_get(
        (*cffont).topdict,
        b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    crate::backend::runtime::input_seek((*cffont).handle, (*cffont).offset.wrapping_add(offset as l_offset) as ssize_t, SEEK_SET);
    idx = cff_get_index_header(cffont);
    offset = crate::backend::runtime::input_seek((*cffont).handle, 0 as ssize_t, SEEK_CUR) as ::core::ffi::c_int;
    if ((*idx).count as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        pdf_warning!(
            b"No valid charstring data found: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        cff_release_index(idx);
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    charstrings = cff_new_index((num_glyphs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as card16);
    max_len = 2 as ::core::ffi::c_int * CS_STR_LEN_MAX;
    (*charstrings).data = new((max_len as uint32_t as usize).wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t) as *mut card8;
    charstring_len = 0 as ::core::ffi::c_int;
    gid = 0 as card16;
    data = new((65536 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t) as *mut card8;
    cid = 0 as ::core::ffi::c_int;
    while cid <= last_cid as ::core::ffi::c_int {
        if !(*used_chars.offset((cid / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid % 8 as ::core::ffi::c_int
            == 0)
        {
            size = (*(*idx).offset.offset((cid + 1 as ::core::ffi::c_int) as isize)).wrapping_sub(*(*idx).offset.offset(cid as isize))
                as ::core::ffi::c_int;
            if size > CS_STR_LEN_MAX {
                pdf_warning!(
                    b"Charstring too long: %s (gid=%u)\0" as *const u8 as *const ::core::ffi::c_char,
                    (*font).filename,
                    cid,
                );
                free(data as *mut ::core::ffi::c_void);
                cff_release_index(charstrings);
                cff_release_index(idx);
                cff_close(cffont);
                sfnt_close(sfont);
                crate::backend::runtime::input_close(handle);
                return -(1 as ::core::ffi::c_int);
            }
            if charstring_len + CS_STR_LEN_MAX >= max_len {
                max_len = charstring_len + 2 as ::core::ffi::c_int * CS_STR_LEN_MAX;
                (*charstrings).data = renew(
                    (*charstrings).data as *mut ::core::ffi::c_void,
                    (max_len as uint32_t as usize).wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t,
                ) as *mut card8;
            }
            *(*charstrings).offset.offset(gid as isize) = (charstring_len + 1 as ::core::ffi::c_int) as l_offset;
            crate::backend::runtime::input_seek(
                (*cffont).handle,
                (offset as l_offset)
                    .wrapping_add(*(*idx).offset.offset(cid as isize))
                    .wrapping_sub(1 as l_offset) as ssize_t,
                SEEK_SET,
            );
            ttstub_input_read((*cffont).handle, data as *mut ::core::ffi::c_char, size as size_t);
            charstring_len += cs_copy_charstring(
                (*charstrings).data.offset(charstring_len as isize),
                max_len - charstring_len,
                data,
                size,
                (*cffont).gsubr,
                *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize),
                default_width,
                nominal_width,
                ::core::ptr::null_mut::<cs_ginfo>(),
            );
            gid = gid.wrapping_add(1);
        }
        cid += 1;
    }
    if gid as ::core::ffi::c_int != num_glyphs as ::core::ffi::c_int {
        pdf_warning!(b"Unexpected error: %s\0" as *const u8 as *const ::core::ffi::c_char, (*font).filename,);
        free(data as *mut ::core::ffi::c_void);
        cff_release_index(charstrings);
        cff_release_index(idx);
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    free(data as *mut ::core::ffi::c_void);
    cff_release_index(idx);
    *(*charstrings).offset.offset(num_glyphs as isize) = (charstring_len + 1 as ::core::ffi::c_int) as l_offset;
    (*charstrings).count = num_glyphs;
    (*cffont).num_glyphs = num_glyphs;
    (*cffont).cstrings = charstrings;
    if !(*cffont).gsubr.is_null() {
        cff_release_index((*cffont).gsubr);
    }
    (*cffont).gsubr = cff_new_index(0 as card16);
    if !(*cffont).subrs.is_null() && !(*(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        cff_release_index(*(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize));
        let ref mut fresh8 = *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize);
        *fresh8 = ::core::ptr::null_mut::<cff_index>();
    }
    if !(*cffont).private.is_null() && !(*(*cffont).private.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        cff_dict_remove(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"Subrs\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    cff_add_string(cffont, b"Adobe\0" as *const u8 as *const ::core::ffi::c_char, 1 as ::core::ffi::c_int);
    cff_add_string(cffont, b"Identity\0" as *const u8 as *const ::core::ffi::c_char, 1 as ::core::ffi::c_int);
    cff_dict_update((*cffont).topdict, cffont);
    if !(*cffont).private.is_null() && !(*(*cffont).private.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        cff_dict_update(*(*cffont).private.offset(0 as ::core::ffi::c_int as isize), cffont);
    }
    cff_update_string(cffont);
    cff_dict_add(
        (*cffont).topdict,
        b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        3 as ::core::ffi::c_int,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        cff_get_sid(cffont, b"Adobe\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_double,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        cff_get_sid(cffont, b"Identity\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_double,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_int,
        0.0f64,
    );
    destlen = write_fontfile(font, cffont);
    let mut CIDToGIDMap: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    CIDToGIDMap = new(
        ((2 as ::core::ffi::c_int * (last_cid as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize) as uint32_t,
    ) as *mut ::core::ffi::c_uchar;
    memset(
        CIDToGIDMap as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (2 as ::core::ffi::c_int * (last_cid as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as size_t,
    );
    cid = 0 as ::core::ffi::c_int;
    while cid <= last_cid as ::core::ffi::c_int {
        if *used_chars.offset((cid / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid % 8 as ::core::ffi::c_int
            != 0
        {
            *CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid) as isize) =
                (cid >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid + 1 as ::core::ffi::c_int) as isize) =
                (cid & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        }
        cid += 1;
    }
    add_CIDMetrics(
        sfont,
        (*font).resource,
        CIDToGIDMap,
        last_cid as ::core::ffi::c_ushort,
        if (*font).cid.need_vmetrics != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        },
    );
    free(CIDToGIDMap as *mut ::core::ffi::c_void);
    cff_close(cffont);
    sfnt_close(sfont);
    crate::backend::runtime::input_close(handle);
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        pdf_message!(
            b"[%u glyphs][%d bytes]\0" as *const u8 as *const ::core::ffi::c_char,
            num_glyphs as ::core::ffi::c_int,
            destlen,
        );
    }
    if pdf_check_version(2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn load_base_CMap(
    mut font_name: *const ::core::ffi::c_char, mut wmode: ::core::ffi::c_int, mut cffont: *mut cff_font,
) -> ::core::ffi::c_int {
    let mut cmap_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
    let mut cmap_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gid: card16 = 0;
    let mut range_min: [::core::ffi::c_uchar; 4] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    let mut range_max: [::core::ffi::c_uchar; 4] = [
        0x7f as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    cmap_name = new((strlen(font_name)
        .wrapping_add(strlen(b"-UCS4-H\0" as *const u8 as *const ::core::ffi::c_char))
        .wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    if wmode != 0 {
        sprintf(cmap_name, b"%s-UCS4-V\0" as *const u8 as *const ::core::ffi::c_char, font_name);
    } else {
        sprintf(cmap_name, b"%s-UCS4-H\0" as *const u8 as *const ::core::ffi::c_char, font_name);
    }
    cmap_id = CMap_cache_find(cmap_name);
    if cmap_id >= 0 as ::core::ffi::c_int {
        free(cmap_name as *mut ::core::ffi::c_void);
        return cmap_id;
    }
    cmap = CMap_new();
    CMap_set_name(cmap, cmap_name);
    CMap_set_type(cmap, CMAP_TYPE_CODE_TO_CID);
    CMap_set_wmode(cmap, wmode);
    CMap_add_codespacerange(
        cmap,
        &raw mut range_min as *mut ::core::ffi::c_uchar,
        &raw mut range_max as *mut ::core::ffi::c_uchar,
        4 as size_t,
    );
    CMap_set_CIDSysInfo(cmap, &raw mut CSI_IDENTITY);
    free(cmap_name as *mut ::core::ffi::c_void);
    gid = 1 as card16;
    while (gid as ::core::ffi::c_int) < (*cffont).num_glyphs as ::core::ffi::c_int {
        let mut ucv: int32_t = 0;
        let mut sid: s_SID = 0;
        let mut glyph: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut suffix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut srcCode: [::core::ffi::c_uchar; 4] = [0; 4];
        sid = cff_charsets_lookup_inverse(cffont, gid) as s_SID;
        glyph = cff_get_string(cffont, sid);
        name = agl_chop_suffix(glyph, &raw mut suffix);
        if name.is_null() {
            free(suffix as *mut ::core::ffi::c_void);
            free(glyph as *mut ::core::ffi::c_void);
        } else if !suffix.is_null() {
            free(name as *mut ::core::ffi::c_void);
            free(suffix as *mut ::core::ffi::c_void);
            free(glyph as *mut ::core::ffi::c_void);
        } else {
            if agl_name_is_unicode(name) {
                ucv = agl_name_convert_unicode(name);
                srcCode[0 as ::core::ffi::c_int as usize] = (ucv >> 24 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                srcCode[1 as ::core::ffi::c_int as usize] = (ucv >> 16 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                srcCode[2 as ::core::ffi::c_int as usize] = (ucv >> 8 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                srcCode[3 as ::core::ffi::c_int as usize] = (ucv & 0xff as int32_t) as ::core::ffi::c_uchar;
                CMap_add_cidchar(cmap, &raw mut srcCode as *mut ::core::ffi::c_uchar, 4 as size_t, gid as CID);
            } else {
                let mut agln: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
                agln = agl_lookup_list(name);
                if agln.is_null() {
                    pdf_warning!(
                        b"Glyph \"%s\" inaccessible (no Unicode mapping)\0" as *const u8 as *const ::core::ffi::c_char,
                        glyph,
                    );
                }
                while !agln.is_null() {
                    if (*agln).n_components > 1 as ::core::ffi::c_int {
                        pdf_warning!(
                            b"Glyph \"%s\" inaccessible (composite character)\0" as *const u8 as *const ::core::ffi::c_char,
                            glyph,
                        );
                    } else if (*agln).n_components == 1 as ::core::ffi::c_int {
                        ucv = (*agln).unicodes[0 as ::core::ffi::c_int as usize];
                        srcCode[0 as ::core::ffi::c_int as usize] = (ucv >> 24 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                        srcCode[1 as ::core::ffi::c_int as usize] = (ucv >> 16 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                        srcCode[2 as ::core::ffi::c_int as usize] = (ucv >> 8 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                        srcCode[3 as ::core::ffi::c_int as usize] = (ucv & 0xff as int32_t) as ::core::ffi::c_uchar;
                        CMap_add_cidchar(cmap, &raw mut srcCode as *mut ::core::ffi::c_uchar, 4 as size_t, gid as CID);
                    }
                    agln = (*agln).alternate as *mut agl_name;
                }
            }
            free(name as *mut ::core::ffi::c_void);
            free(suffix as *mut ::core::ffi::c_void);
            free(glyph as *mut ::core::ffi::c_void);
        }
        gid = gid.wrapping_add(1);
    }
    cmap_id = CMap_cache_add(cmap);
    return cmap_id;
}
#[unsafe(export_name = "__pdf_impl_t1_load_UnicodeCMap")]
pub unsafe extern "C-unwind" fn t1_load_UnicodeCMap(
    mut font_name: *const ::core::ffi::c_char, mut otl_tags: *const ::core::ffi::c_char, mut wmode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cmap_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    if font_name.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    handle = dpx_open_type1_file(font_name);
    if handle == INVALID_HANDLE {
        return -(1 as ::core::ffi::c_int);
    }
    cffont = t1_load_font(::core::ptr::null_mut::<*mut ::core::ffi::c_char>(), 1 as ::core::ffi::c_int, handle);
    crate::backend::runtime::input_close(handle);
    if cffont.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    cmap_id = load_base_CMap(font_name, wmode, cffont);
    cff_close(cffont);
    if cmap_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Failed to create Unicode charmap for font \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
            font_name,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if !otl_tags.is_null() {
        pdf_warning!(b"Glyph substitution not supported for Type1 font yet...\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    return cmap_id;
}
unsafe extern "C-unwind" fn create_ToUnicode_stream(
    mut cffont: *mut cff_font, mut font_name: *const ::core::ffi::c_char, mut used_glyphs: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
    let mut cid: CID = 0;
    let mut gid: card16 = 0;
    let mut glyph_count: ::core::ffi::c_int = 0;
    let mut total_fail_count: ::core::ffi::c_int = 0;
    let mut cmap_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wbuf: [::core::ffi::c_uchar; 1024] = [0; 1024];
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut endptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    const RANGE_MIN: [::core::ffi::c_uchar; 2] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    const RANGE_MAX: [::core::ffi::c_uchar; 2] = [
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    if font_name.is_null() || used_glyphs.is_null() {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    cmap = CMap_new();
    cmap_name = new((strlen(font_name)
        .wrapping_add(strlen(b"-UTF16\0" as *const u8 as *const ::core::ffi::c_char))
        .wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    strcpy(cmap_name, font_name);
    strcat(cmap_name, b"-UTF16\0" as *const u8 as *const ::core::ffi::c_char);
    CMap_set_name(cmap, cmap_name);
    free(cmap_name as *mut ::core::ffi::c_void);
    CMap_set_wmode(cmap, 0 as ::core::ffi::c_int);
    CMap_set_type(cmap, CMAP_TYPE_TO_UNICODE);
    CMap_set_CIDSysInfo(cmap, &raw mut CSI_UNICODE);
    CMap_add_codespacerange(cmap, RANGE_MIN.as_ptr(), RANGE_MAX.as_ptr(), 2 as size_t);
    total_fail_count = 0 as ::core::ffi::c_int;
    glyph_count = total_fail_count;
    p = &raw mut wbuf as *mut ::core::ffi::c_uchar;
    endptr = (&raw mut wbuf as *mut ::core::ffi::c_uchar).offset(WBUF_SIZE as isize);
    cid = 1 as CID;
    while (cid as ::core::ffi::c_int) < (*cffont).num_glyphs as ::core::ffi::c_int {
        if *used_glyphs.offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
            != 0
        {
            let mut glyph: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut len: int32_t = 0;
            let mut fail_count: ::core::ffi::c_int = 0;
            wbuf[0 as ::core::ffi::c_int as usize] =
                (cid as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            wbuf[1 as ::core::ffi::c_int as usize] = (cid as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            p = (&raw mut wbuf as *mut ::core::ffi::c_uchar).offset(2 as ::core::ffi::c_int as isize);
            gid = cff_charsets_lookup_inverse(cffont, cid as card16);
            if !(gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                glyph = cff_get_string(cffont, gid as s_SID);
                if !glyph.is_null() {
                    len = agl_sput_UTF16BE(glyph, &raw mut p, endptr, &raw mut fail_count);
                    if len < 1 as int32_t || fail_count != 0 {
                        total_fail_count += fail_count;
                    } else {
                        CMap_add_bfchar(
                            cmap,
                            &raw mut wbuf as *mut ::core::ffi::c_uchar,
                            2 as size_t,
                            (&raw mut wbuf as *mut ::core::ffi::c_uchar).offset(2 as ::core::ffi::c_int as isize),
                            len as size_t,
                        );
                    }
                    free(glyph as *mut ::core::ffi::c_void);
                }
                glyph_count += 1;
            }
        }
        cid = cid.wrapping_add(1);
    }
    if total_fail_count != 0 as ::core::ffi::c_int && total_fail_count >= glyph_count / 10 as ::core::ffi::c_int {
        pdf_warning!(
            b"%d glyph names (out of %d) missing Unicode mapping.\0" as *const u8 as *const ::core::ffi::c_char,
            total_fail_count,
            glyph_count,
        );
        pdf_warning!(
            b"ToUnicode CMap \"%s-UTF16\" removed.\0" as *const u8 as *const ::core::ffi::c_char,
            font_name,
        );
    } else {
        stream = CMap_create_stream(cmap);
    }
    CMap_release(cmap);
    return stream;
}
pub const WBUF_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_CIDFont_type0_t1create_ToUnicode_stream")]
pub unsafe extern "C-unwind" fn CIDFont_type0_t1create_ToUnicode_stream(
    mut filename: *const ::core::ffi::c_char, mut fontname: *const ::core::ffi::c_char, mut used_chars: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut ref_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut tounicode: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut handle: rust_input_handle_t = 0;
    if filename.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"filename\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if fontname.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"fontname\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if used_chars.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"used_chars\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    handle = dpx_open_type1_file(filename);
    if handle == 0 {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    cffont = t1_load_font(::core::ptr::null_mut::<*mut ::core::ffi::c_char>(), 1 as ::core::ffi::c_int, handle);
    if !cffont.is_null() {
        tounicode = create_ToUnicode_stream(cffont, fontname, used_chars);
        if !tounicode.is_null() {
            ref_0 = pdf_ref_obj(tounicode);
            pdf_release_obj(tounicode);
        }
    }
    crate::backend::runtime::input_close(handle);
    return ref_0;
}
pub const FONT_FLAG_FIXEDPITCH: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const FONT_FLAG_SERIF: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const FONT_FLAG_SYMBOLIC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const FONT_FLAG_ITALIC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const FONT_FLAG_FORCEBOLD: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn get_font_attr(mut font: *mut pdf_font, mut cffont: *mut cff_font) {
    let mut capheight: ::core::ffi::c_double = 0.;
    let mut ascent: ::core::ffi::c_double = 0.;
    let mut descent: ::core::ffi::c_double = 0.;
    let mut italicangle: ::core::ffi::c_double = 0.;
    let mut stemv: ::core::ffi::c_double = 0.;
    let mut defaultwidth: ::core::ffi::c_double = 0.;
    let mut nominalwidth: ::core::ffi::c_double = 0.;
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut gid: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    const L_C: [*const ::core::ffi::c_char; 5] = [
        b"H\0" as *const u8 as *const ::core::ffi::c_char,
        b"P\0" as *const u8 as *const ::core::ffi::c_char,
        b"Pi\0" as *const u8 as *const ::core::ffi::c_char,
        b"Rho\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    const L_D: [*const ::core::ffi::c_char; 5] = [
        b"p\0" as *const u8 as *const ::core::ffi::c_char,
        b"q\0" as *const u8 as *const ::core::ffi::c_char,
        b"mu\0" as *const u8 as *const ::core::ffi::c_char,
        b"eta\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    const L_A: [*const ::core::ffi::c_char; 4] = [
        b"b\0" as *const u8 as *const ::core::ffi::c_char,
        b"h\0" as *const u8 as *const ::core::ffi::c_char,
        b"lambda\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut gm: t1_ginfo = t1_ginfo {
        use_seac: 0,
        wx: 0.,
        wy: 0.,
        bbox: C2RustUnnamed_10 {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        seac: C2RustUnnamed_9 {
            asb: 0.,
            adx: 0.,
            ady: 0.,
            bchar: 0,
            achar: 0,
        },
    };
    defaultwidth = 500.0f64;
    nominalwidth = 0.0f64;
    if cff_dict_known((*cffont).topdict, b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        ascent = cff_dict_get(
            (*cffont).topdict,
            b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char,
            3 as ::core::ffi::c_int,
        );
        capheight = ascent;
        descent = cff_dict_get(
            (*cffont).topdict,
            b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
    } else {
        capheight = 680.0f64;
        ascent = 690.0f64;
        descent = -190.0f64;
    }
    if cff_dict_known(
        *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
        b"StdVW\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        stemv = cff_dict_get(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"StdVW\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
    } else {
        stemv = 88.0f64;
    }
    if cff_dict_known((*cffont).topdict, b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        italicangle = cff_dict_get(
            (*cffont).topdict,
            b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        if italicangle != 0.0f64 {
            flags |= FONT_FLAG_ITALIC;
        }
    } else {
        italicangle = 0.0f64;
    }
    gid = cff_glyph_lookup(cffont, b"space\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int;
    if gid >= 0 as ::core::ffi::c_int && gid < (*(*cffont).cstrings).count as ::core::ffi::c_int {
        t1char_get_metrics(
            (*(*cffont).cstrings)
                .data
                .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
            (*(*(*cffont).cstrings).offset.offset((gid + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize)) as ::core::ffi::c_int,
            *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize),
            &raw mut gm,
        );
        defaultwidth = gm.wx;
    }
    i = 0 as ::core::ffi::c_int;
    while !L_C[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_C[i as usize]) as ::core::ffi::c_int;
        if gid >= 0 as ::core::ffi::c_int && gid < (*(*cffont).cstrings).count as ::core::ffi::c_int {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)),
                (*(*(*cffont).cstrings).offset.offset((gid + 1 as ::core::ffi::c_int) as isize))
                    .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize)) as ::core::ffi::c_int,
                *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize),
                &raw mut gm,
            );
            capheight = gm.bbox.ury;
            break;
        } else {
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while !L_D[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_D[i as usize]) as ::core::ffi::c_int;
        if gid >= 0 as ::core::ffi::c_int && gid < (*(*cffont).cstrings).count as ::core::ffi::c_int {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)),
                (*(*(*cffont).cstrings).offset.offset((gid + 1 as ::core::ffi::c_int) as isize))
                    .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize)) as ::core::ffi::c_int,
                *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize),
                &raw mut gm,
            );
            descent = gm.bbox.lly;
            break;
        } else {
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while !L_A[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_A[i as usize]) as ::core::ffi::c_int;
        if gid >= 0 as ::core::ffi::c_int && gid < (*(*cffont).cstrings).count as ::core::ffi::c_int {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)),
                (*(*(*cffont).cstrings).offset.offset((gid + 1 as ::core::ffi::c_int) as isize))
                    .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize)) as ::core::ffi::c_int,
                *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize),
                &raw mut gm,
            );
            ascent = gm.bbox.ury;
            break;
        } else {
            i += 1;
        }
    }
    if defaultwidth != 0.0f64 {
        cff_dict_add(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        cff_dict_set(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            defaultwidth,
        );
    }
    if nominalwidth != 0.0f64 {
        cff_dict_add(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        cff_dict_set(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            nominalwidth,
        );
    }
    if cff_dict_known(
        *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
        b"ForceBold\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
        && cff_dict_get(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"ForceBold\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) != 0.
    {
        flags |= FONT_FLAG_FORCEBOLD;
    }
    if cff_dict_known(
        *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
        b"IsFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
        && cff_dict_get(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"IsFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) != 0.
    {
        flags |= FONT_FLAG_FIXEDPITCH;
    }
    if !(*font).fontname.is_null() && strstr((*font).fontname, b"Sans\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        flags |= FONT_FLAG_SERIF;
    }
    flags |= FONT_FLAG_SYMBOLIC;
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"CapHeight\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(capheight),
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"Ascent\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(ascent),
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"Descent\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(descent),
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(italicangle),
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"StemV\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(stemv),
    );
    pdf_add_dict(
        (*font).descriptor,
        pdf_new_name(b"Flags\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(flags as ::core::ffi::c_double),
    );
}
unsafe extern "C-unwind" fn add_metrics(
    mut font: *mut pdf_font, mut cffont: *mut cff_font, mut CIDToGIDMap: *mut ::core::ffi::c_uchar, mut widths: *mut ::core::ffi::c_double,
    mut default_width: ::core::ffi::c_double, mut last_cid: CID,
) {
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut val: ::core::ffi::c_double = 0.;
    let mut cid: card16 = 0;
    let mut gid: card16 = 0;
    let mut used_chars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if cff_dict_known((*cffont).topdict, b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        pdf_warning!(b"No FontBBox found: %s\0" as *const u8 as *const ::core::ffi::c_char, (*font).filename,);
    } else {
        tmp = pdf_new_array();
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            val = cff_dict_get((*cffont).topdict, b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char, i);
            pdf_add_array(tmp, pdf_new_number(floor(val / 1.0f64 + 0.5f64) * 1.0f64));
            i += 1;
        }
        pdf_add_dict(
            (*font).descriptor,
            pdf_new_name(b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char),
            tmp,
        );
    }
    used_chars = (*font).usedchars;
    tmp = pdf_new_array();
    cid = 0 as card16;
    while cid as ::core::ffi::c_int <= last_cid as ::core::ffi::c_int {
        if *used_chars.offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
            != 0
        {
            gid = ((*CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int)
                as card16;
            if *widths.offset(gid as isize) != default_width {
                pdf_add_array(tmp, pdf_new_number(cid as ::core::ffi::c_double));
                pdf_add_array(tmp, pdf_new_number(cid as ::core::ffi::c_double));
                pdf_add_array(tmp, pdf_new_number(floor(*widths.offset(gid as isize) / 1.0f64 + 0.5f64) * 1.0f64));
            }
        }
        cid = cid.wrapping_add(1);
    }
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"DW\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(default_width),
    );
    if pdf_array_length(tmp) > 0 as ::core::ffi::c_uint {
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"W\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(tmp),
        );
    }
    pdf_release_obj(tmp);
}
#[unsafe(export_name = "__pdf_impl_CIDFont_type0_t1dofont")]
pub unsafe extern "C-unwind" fn CIDFont_type0_t1dofont(mut font: *mut pdf_font) -> ::core::ffi::c_int {
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut defaultwidth: ::core::ffi::c_double = 0.;
    let mut nominalwidth: ::core::ffi::c_double = 0.;
    let mut num_glyphs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut handle: rust_input_handle_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0;
    let mut used_chars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut last_cid: card16 = 0;
    let mut gid: card16 = 0;
    let mut cid: card16 = 0;
    let mut CIDToGIDMap: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"font\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if (*font).reference.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"FontDescriptor\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj((*font).descriptor),
    );
    handle = dpx_open_type1_file((*font).filename);
    if handle == 0 {
        pdf_warning!(b"Type1: Could not open Type1 font.\0" as *const u8 as *const ::core::ffi::c_char,);
        return -(1 as ::core::ffi::c_int);
    }
    cffont = t1_load_font(::core::ptr::null_mut::<*mut ::core::ffi::c_char>(), 0 as ::core::ffi::c_int, handle);
    if cffont.is_null() {
        pdf_warning!(b"Could not read Type 1 font...\0" as *const u8 as *const ::core::ffi::c_char,);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    crate::backend::runtime::input_close(handle);
    if (*font).fontname.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font->fontname\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*font).usedchars.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font->usedchars\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    used_chars = (*font).usedchars;
    cff_set_name(cffont, (*font).fontname);
    get_font_attr(font, cffont);
    if cff_dict_known(
        *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
        b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        defaultwidth = cff_dict_get(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
    } else {
        defaultwidth = 0.0f64;
    }
    if cff_dict_known(
        *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
        b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        nominalwidth = cff_dict_get(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
    } else {
        nominalwidth = 0.0f64;
    }
    num_glyphs = 0 as ::core::ffi::c_int;
    last_cid = 0 as card16;
    let ref mut fresh3 = *used_chars.offset((0 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
    *fresh3 = (*fresh3 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - 0 as ::core::ffi::c_int % 8 as ::core::ffi::c_int)
        as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < ((*cffont).num_glyphs as ::core::ffi::c_int + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        c = *used_chars.offset(i as isize) as ::core::ffi::c_int;
        j = 7 as ::core::ffi::c_int;
        while j >= 0 as ::core::ffi::c_int {
            if c & (1 as ::core::ffi::c_int) << j != 0 {
                num_glyphs += 1;
                last_cid = ((i + 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int - j - 1 as ::core::ffi::c_int) as card16;
            }
            j -= 1;
        }
        i += 1;
    }
    let mut fdselect: *mut cff_fdselect = ::core::ptr::null_mut::<cff_fdselect>();
    fdselect = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_fdselect>() as usize) as uint32_t)
        as *mut cff_fdselect;
    (*fdselect).format = 3 as card8;
    (*fdselect).num_entries = 1 as card16;
    (*fdselect).data.ranges =
        new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_range3>() as usize) as uint32_t)
            as *mut cff_range3;
    (*(*fdselect).data.ranges.offset(0 as ::core::ffi::c_int as isize)).first = 0 as card16;
    (*(*fdselect).data.ranges.offset(0 as ::core::ffi::c_int as isize)).fd = 0 as card8;
    (*cffont).fdselect = fdselect;
    CIDToGIDMap = new(
        ((2 as ::core::ffi::c_int * (last_cid as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize) as uint32_t,
    ) as *mut ::core::ffi::c_uchar;
    memset(
        CIDToGIDMap as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (2 as ::core::ffi::c_int * (last_cid as ::core::ffi::c_int + 1 as ::core::ffi::c_int)) as size_t,
    );
    let mut charset: *mut cff_charsets = ::core::ptr::null_mut::<cff_charsets>();
    charset = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_charsets>() as usize) as uint32_t)
        as *mut cff_charsets;
    (*charset).format = 0 as card8;
    (*charset).num_entries = (num_glyphs - 1 as ::core::ffi::c_int) as card16;
    (*charset).data.glyphs =
        new(((num_glyphs - 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<s_SID>() as usize) as uint32_t)
            as *mut s_SID;
    gid = 0 as card16;
    cid = 0 as card16;
    while cid as ::core::ffi::c_int <= last_cid as ::core::ffi::c_int {
        if *used_chars.offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
            != 0
        {
            if gid as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *(*charset)
                    .data
                    .glyphs
                    .offset((gid as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) = cid as s_SID;
            }
            *CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid as ::core::ffi::c_int) as isize) =
                (gid as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *CIDToGIDMap.offset((2 as ::core::ffi::c_int * cid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (gid as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            gid = gid.wrapping_add(1);
        }
        cid = cid.wrapping_add(1);
    }
    cff_release_charsets((*cffont).charsets);
    (*cffont).charsets = charset;
    cff_dict_add(
        (*cffont).topdict,
        b"CIDCount\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"CIDCount\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        (last_cid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
    );
    (*cffont).fdarray = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<*mut cff_dict>() as usize) as uint32_t)
        as *mut *mut cff_dict;
    let ref mut fresh4 = *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize);
    *fresh4 = cff_new_dict();
    cff_dict_add(
        *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize),
        b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize),
        b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        cff_add_string(cffont, (*font).fontname.offset(7 as ::core::ffi::c_int as isize), 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
    );
    cff_dict_add(
        *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize),
        b"Private\0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_int,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize),
        b"Private\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0.0f64,
    );
    cff_dict_set(
        *(*cffont).fdarray.offset(0 as ::core::ffi::c_int as isize),
        b"Private\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0.0f64,
    );
    cff_dict_add(
        (*cffont).topdict,
        b"FDArray\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"FDArray\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0.0f64,
    );
    cff_dict_add(
        (*cffont).topdict,
        b"FDSelect\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"FDSelect\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0.0f64,
    );
    cff_dict_add(
        (*cffont).topdict,
        b"charset\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"charset\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0.0f64,
    );
    cff_dict_add(
        (*cffont).topdict,
        b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0.0f64,
    );
    let mut cstring: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut gm: t1_ginfo = t1_ginfo {
        use_seac: 0,
        wx: 0.,
        wy: 0.,
        bbox: C2RustUnnamed_10 {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        seac: C2RustUnnamed_9 {
            asb: 0.,
            adx: 0.,
            ady: 0.,
            bchar: 0,
            achar: 0,
        },
    };
    let mut max: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut widths: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut w_stat: [::core::ffi::c_int; 1001] = [0; 1001];
    let mut max_count: ::core::ffi::c_int = 0;
    let mut dw: ::core::ffi::c_int = 0;
    widths = new((num_glyphs as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize) as uint32_t)
        as *mut ::core::ffi::c_double;
    memset(
        &raw mut w_stat as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t).wrapping_mul(1001 as size_t),
    );
    offset = 0 as ::core::ffi::c_int;
    cstring = cff_new_index(num_glyphs as card16);
    (*cstring).data = ::core::ptr::null_mut::<card8>();
    *(*cstring).offset.offset(0 as ::core::ffi::c_int as isize) = 1 as l_offset;
    gid = 0 as card16;
    cid = 0 as card16;
    while cid as ::core::ffi::c_int <= last_cid as ::core::ffi::c_int {
        if !(*used_chars.offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
            == 0)
        {
            if offset + CS_STR_LEN_MAX >= max {
                max += CS_STR_LEN_MAX * 2 as ::core::ffi::c_int;
                (*cstring).data = renew(
                    (*cstring).data as *mut ::core::ffi::c_void,
                    (max as uint32_t as usize).wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t,
                ) as *mut card8;
            }
            offset += t1char_convert_charstring(
                (*cstring)
                    .data
                    .offset(*(*cstring).offset.offset(gid as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)),
                CS_STR_LEN_MAX,
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(cid as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)),
                (*(*(*cffont).cstrings)
                    .offset
                    .offset((cid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(*(*(*cffont).cstrings).offset.offset(cid as isize)) as ::core::ffi::c_int,
                *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize),
                defaultwidth,
                nominalwidth,
                &raw mut gm,
            );
            *(*cstring).offset.offset((gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (offset + 1 as ::core::ffi::c_int) as l_offset;
            if gm.use_seac != 0 {
                pdf_warning!(
                    b"The \"seac\" command for an accented character found: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*font).filename,
                );
                free(widths as *mut ::core::ffi::c_void);
                cff_release_index(cstring);
                free(CIDToGIDMap as *mut ::core::ffi::c_void);
                cff_close(cffont);
                return -(1 as ::core::ffi::c_int);
            }
            *widths.offset(gid as isize) = gm.wx;
            if gm.wx >= 0.0f64 && gm.wx <= 1000.0f64 {
                w_stat[gm.wx as ::core::ffi::c_int as usize] += 1 as ::core::ffi::c_int;
            }
            gid = gid.wrapping_add(1);
        }
        cid = cid.wrapping_add(1);
    }
    cff_release_index((*cffont).cstrings);
    (*cffont).cstrings = cstring;
    max_count = 0 as ::core::ffi::c_int;
    dw = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i <= 1000 as ::core::ffi::c_int {
        if w_stat[i as usize] > max_count {
            dw = i;
            max_count = w_stat[i as usize];
        }
        i += 1;
    }
    if dw >= 0 as ::core::ffi::c_int {
        add_metrics(font, cffont, CIDToGIDMap, widths, dw as ::core::ffi::c_double, last_cid as CID);
    } else {
        add_metrics(font, cffont, CIDToGIDMap, widths, defaultwidth, last_cid as CID);
    }
    free(widths as *mut ::core::ffi::c_void);
    cff_release_index(*(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize));
    let ref mut fresh5 = *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize);
    *fresh5 = ::core::ptr::null_mut::<cff_index>();
    free(CIDToGIDMap as *mut ::core::ffi::c_void);
    cff_add_string(cffont, b"Adobe\0" as *const u8 as *const ::core::ffi::c_char, 1 as ::core::ffi::c_int);
    cff_add_string(cffont, b"Identity\0" as *const u8 as *const ::core::ffi::c_char, 1 as ::core::ffi::c_int);
    cff_dict_update((*cffont).topdict, cffont);
    cff_dict_update(*(*cffont).private.offset(0 as ::core::ffi::c_int as isize), cffont);
    cff_update_string(cffont);
    cff_dict_add(
        (*cffont).topdict,
        b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        3 as ::core::ffi::c_int,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        cff_get_sid(cffont, b"Adobe\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_double,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        cff_get_sid(cffont, b"Identity\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_double,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_int,
        0.0f64,
    );
    (*cffont).num_glyphs = num_glyphs as card16;
    offset = write_fontfile(font, cffont);
    cff_close(cffont);
    if pdf_check_version(2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        CIDFont_type0_add_CIDSet(font, used_chars, last_cid);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" {
    fn memset(__b: *mut ::core::ffi::c_void, __c: ::core::ffi::c_int, __len: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CIDFont_type0_dofont(mut font: *mut pdf_font, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_type0_dofont(font) }) {
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_type0_open_from_t1(
    mut font: *mut pdf_font, mut name: *const ::core::ffi::c_char, mut index: ::core::ffi::c_int, mut opt: *mut cid_opt,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_type0_open_from_t1(font, name, index, opt) }) {
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_type0_open(
    mut font: *mut pdf_font, mut name: *const ::core::ffi::c_char, mut index: ::core::ffi::c_int, mut opt: *mut cid_opt,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_type0_open(font, name, index, opt) }) {
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_type0_open_from_t1c(
    mut font: *mut pdf_font, mut name: *const ::core::ffi::c_char, mut index: ::core::ffi::c_int, mut opt: *mut cid_opt,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_type0_open_from_t1c(font, name, index, opt) }) {
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_type0_t1cdofont(mut font: *mut pdf_font, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_type0_t1cdofont(font) }) {
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
pub unsafe extern "C" fn __pdf_entry_t1_load_UnicodeCMap(
    mut font_name: *const ::core::ffi::c_char, mut otl_tags: *const ::core::ffi::c_char, mut wmode: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { t1_load_UnicodeCMap(font_name, otl_tags, wmode) }) {
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_type0_t1create_ToUnicode_stream(
    mut filename: *const ::core::ffi::c_char, mut fontname: *const ::core::ffi::c_char, mut used_chars: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_type0_t1create_ToUnicode_stream(filename, fontname, used_chars) }) {
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_type0_t1dofont(mut font: *mut pdf_font, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_type0_t1dofont(font) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
