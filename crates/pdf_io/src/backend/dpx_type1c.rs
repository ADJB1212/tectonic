/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2008-2018 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
   the dvipdfmx project team.

   Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

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
/* Copyright and Trademark Notice ommited. */
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    #[link_name = "__pdf_impl_pdf_merge_dict"]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_lookup_dict"]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const ::core::ffi::c_char) -> *mut pdf_obj;
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
    #[link_name = "__pdf_impl_pdf_stream_dict"]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_stream_length"]
    fn pdf_stream_length(stream: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_stream_dataptr"]
    fn pdf_stream_dataptr(stream: *mut pdf_obj) -> *const ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_check_version"]
    fn pdf_check_version(
        major: ::core::ffi::c_int,
        minor: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_get_uniqueTag"]
    fn pdf_font_get_uniqueTag(font: *mut pdf_font) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_font_get_resource"]
    fn pdf_font_get_resource(font: *mut pdf_font) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_font_get_descriptor"]
    fn pdf_font_get_descriptor(font: *mut pdf_font) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_check_tfm_widths"]
    fn pdf_check_tfm_widths(
        ident: *const ::core::ffi::c_char,
        widths: *mut ::core::ffi::c_double,
        firstchar: ::core::ffi::c_int,
        lastchar: ::core::ffi::c_int,
        usedchars: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut work_buffer: [::core::ffi::c_char; 0];
    #[link_name = "__pdf_impl_cff_open"]
    fn cff_open(
        handle: rust_input_handle_t,
        offset: ::core::ffi::c_int,
        idx: ::core::ffi::c_int,
    ) -> *mut cff_font;
    #[link_name = "__pdf_impl_cff_close"]
    fn cff_close(cff: *mut cff_font);
    #[link_name = "__pdf_impl_cff_put_header"]
    fn cff_put_header(
        cff: *mut cff_font,
        dest: *mut card8,
        destlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_get_index_header"]
    fn cff_get_index_header(cff: *mut cff_font) -> *mut cff_index;
    #[link_name = "__pdf_impl_cff_release_index"]
    fn cff_release_index(idx: *mut cff_index);
    #[link_name = "__pdf_impl_cff_new_index"]
    fn cff_new_index(count: card16) -> *mut cff_index;
    #[link_name = "__pdf_impl_cff_index_size"]
    fn cff_index_size(idx: *mut cff_index) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_pack_index"]
    fn cff_pack_index(
        idx: *mut cff_index,
        dest: *mut card8,
        destlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_get_name"]
    fn cff_get_name(cff: *mut cff_font) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_cff_set_name"]
    fn cff_set_name(cff: *mut cff_font, name: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_read_subrs"]
    fn cff_read_subrs(cff: *mut cff_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_read_encoding"]
    fn cff_read_encoding(cff: *mut cff_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_pack_encoding"]
    fn cff_pack_encoding(
        cff: *mut cff_font,
        dest: *mut card8,
        destlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_encoding_lookup"]
    fn cff_encoding_lookup(cff: *mut cff_font, code: card8) -> card16;
    #[link_name = "__pdf_impl_cff_release_encoding"]
    fn cff_release_encoding(encoding: *mut cff_encoding);
    #[link_name = "__pdf_impl_cff_read_charsets"]
    fn cff_read_charsets(cff: *mut cff_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_pack_charsets"]
    fn cff_pack_charsets(
        cff: *mut cff_font,
        dest: *mut card8,
        destlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_charsets_lookup"]
    fn cff_charsets_lookup(cff: *mut cff_font, cid: card16) -> card16;
    #[link_name = "__pdf_impl_cff_release_charsets"]
    fn cff_release_charsets(charset: *mut cff_charsets);
    #[link_name = "__pdf_impl_cff_charsets_lookup_inverse"]
    fn cff_charsets_lookup_inverse(cff: *mut cff_font, gid: card16) -> card16;
    #[link_name = "__pdf_impl_cff_read_private"]
    fn cff_read_private(cff: *mut cff_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_get_string"]
    fn cff_get_string(cff: *mut cff_font, id: s_SID) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_cff_get_sid"]
    fn cff_get_sid(cff: *mut cff_font, str: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_add_string"]
    fn cff_add_string(
        cff: *mut cff_font,
        str: *const ::core::ffi::c_char,
        unique: ::core::ffi::c_int,
    ) -> s_SID;
    #[link_name = "__pdf_impl_cff_update_string"]
    fn cff_update_string(cff: *mut cff_font);
    #[link_name = "__pdf_impl_cff_dict_set"]
    fn cff_dict_set(
        dict: *mut cff_dict,
        key: *const ::core::ffi::c_char,
        idx: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
    );
    #[link_name = "__pdf_impl_cff_dict_get"]
    fn cff_dict_get(
        dict: *mut cff_dict,
        key: *const ::core::ffi::c_char,
        idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_cff_dict_add"]
    fn cff_dict_add(
        dict: *mut cff_dict,
        key: *const ::core::ffi::c_char,
        count: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_cff_dict_remove"]
    fn cff_dict_remove(dict: *mut cff_dict, key: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_cff_dict_known"]
    fn cff_dict_known(dict: *mut cff_dict, key: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_dict_pack"]
    fn cff_dict_pack(
        dict: *mut cff_dict,
        dest: *mut card8,
        destlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_dict_update"]
    fn cff_dict_update(dict: *mut cff_dict, cff: *mut cff_font);
    #[link_name = "__pdf_impl_cs_copy_charstring"]
    fn cs_copy_charstring(
        dest: *mut card8,
        destlen: ::core::ffi::c_int,
        src: *mut card8,
        srclen: ::core::ffi::c_int,
        gsubr: *mut cff_index,
        subr: *mut cff_index,
        default_width: ::core::ffi::c_double,
        nominal_width: ::core::ffi::c_double,
        ginfo: *mut cs_ginfo,
    ) -> ::core::ffi::c_int;
    static mut dpx_conf: _dpx_conf;
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
    #[link_name = "__pdf_impl_pdf_encoding_get_encoding"]
    fn pdf_encoding_get_encoding(enc_id: ::core::ffi::c_int) -> *mut *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_create_ToUnicode_CMap"]
    fn pdf_create_ToUnicode_CMap(
        enc_name: *const ::core::ffi::c_char,
        enc_vec: *mut *mut ::core::ffi::c_char,
        is_used: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_sfnt_open"]
    fn sfnt_open(handle: rust_input_handle_t) -> *mut sfnt;
    #[link_name = "__pdf_impl_sfnt_close"]
    fn sfnt_close(sfont: *mut sfnt);
    #[link_name = "__pdf_impl_sfnt_read_table_directory"]
    fn sfnt_read_table_directory(sfont: *mut sfnt, offset: SFNT_ULONG) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_sfnt_find_table_pos"]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
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
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
pub const CFF_STDSTR_MAX: ::core::ffi::c_int = 391 as ::core::ffi::c_int;
pub const CS_STR_LEN_MAX: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PDF_FONT_FONTTYPE_TYPE1C: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PDF_FONT_FLAG_NOEMBED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const WORK_BUFFER_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const FONTTYPE_CIDFONT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const FONTTYPE_FONT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const CFF_NOMINALWIDTHX_DEFAULT: ::core::ffi::c_double = 0.0f64;
pub const CFF_DEFAULTWIDTHX_DEFAULT: ::core::ffi::c_double = 0.0f64;
pub const SFNT_TYPE_POSTSCRIPT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const SFNT_TYPE_TTC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_pdf_font_open_type1c")]
pub unsafe extern "C-unwind" fn pdf_font_open_type1c(
    mut font: *mut pdf_font,
    mut ident: *const ::core::ffi::c_char,
    mut index: ::core::ffi::c_int,
    mut encoding_id: ::core::ffi::c_int,
    mut embedding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut descriptor: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut offset: SFNT_ULONG = 0 as SFNT_ULONG;
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if ident.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ident\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    handle = dpx_open_opentype_file(ident);
    if handle == 0 {
        handle = dpx_open_truetype_file(ident);
    }
    if handle == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    sfont = sfnt_open(handle);
    if sfont.is_null() {
        if handle != 0 {
            crate::backend::runtime::input_close(handle);
        }
        return -(1 as ::core::ffi::c_int);
    }
    if (*sfont).type_0 == SFNT_TYPE_TTC {
        offset = ttc_read_offset(sfont, index as SFNT_ULONG);
    }
    if (*sfont).type_0 != SFNT_TYPE_TTC && (*sfont).type_0 != SFNT_TYPE_POSTSCRIPT
        || sfnt_read_table_directory(sfont, offset) < 0 as ::core::ffi::c_int
        || {
            offset =
                sfnt_find_table_pos(sfont, b"CFF \0" as *const u8 as *const ::core::ffi::c_char);
            offset == 0 as SFNT_ULONG
        }
    {
        sfnt_close(sfont);
        if handle != 0 {
            crate::backend::runtime::input_close(handle);
        }
        return -(1 as ::core::ffi::c_int);
    }
    cffont = cff_open(
        (*sfont).handle,
        offset as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if cffont.is_null() {
        pdf_warning!(
            b"Could not read CFF font data: %s\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
        sfnt_close(sfont);
        if handle != 0 {
            crate::backend::runtime::input_close(handle);
        }
        return -(1 as ::core::ffi::c_int);
    }
    if (*cffont).flag & FONTTYPE_CIDFONT != 0 {
        cff_close(cffont);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    fontname = cff_get_name(cffont);
    if fontname.is_null() {
        pdf_warning!(
            b"No valid FontName found in CFF/OpenType font: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            ident,
        );
        cff_close(cffont);
        sfnt_close(sfont);
        if handle != 0 {
            crate::backend::runtime::input_close(handle);
        }
        return -(1 as ::core::ffi::c_int);
    }
    (*font).fontname = fontname;
    cff_close(cffont);
    if embedding == 0 {
        pdf_warning!(
            b"Ignoring no-embed option for Type1C font: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            ident,
        );
        embedding = 1 as ::core::ffi::c_int;
        (*font).flags &= !PDF_FONT_FLAG_NOEMBED;
    }
    if encoding_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Built-in encoding used for CFF/OpenType font.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        pdf_warning!(
            b"CFF font in OpenType font sometimes have strange built-in encoding.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        pdf_warning!(
            b"If you find text is not encoded properly in the generated PDF file,\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        pdf_warning!(
            b"please specify appropriate \".enc\" file in your fontmap.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*font).subtype = PDF_FONT_FONTTYPE_TYPE1C;
    descriptor = pdf_font_get_descriptor(font);
    tmp = tt_get_fontdesc(
        sfont,
        &raw mut embedding,
        -(1 as ::core::ffi::c_int),
        1 as ::core::ffi::c_int,
        fontname,
    );
    if tmp.is_null() {
        pdf_abort!(
            b"Could not obtain neccesary font info from OpenType table.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    pdf_merge_dict(descriptor, tmp);
    pdf_release_obj(tmp);
    if embedding == 0 {
        pdf_warning!(
            b"Font embedding disallowed for \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
        sfnt_close(sfont);
        if handle != 0 {
            crate::backend::runtime::input_close(handle);
        }
        return -(1 as ::core::ffi::c_int);
    }
    sfnt_close(sfont);
    crate::backend::runtime::input_close(handle);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn add_SimpleMetrics(
    mut font: *mut pdf_font,
    mut cffont: *mut cff_font,
    mut widths: *mut ::core::ffi::c_double,
    mut num_glyphs: card16,
) {
    let mut fontdict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut code: ::core::ffi::c_int = 0;
    let mut firstchar: ::core::ffi::c_int = 0;
    let mut lastchar: ::core::ffi::c_int = 0;
    let mut usedchars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut scaling: ::core::ffi::c_double = 1.0f64;
    fontdict = pdf_font_get_resource(font);
    usedchars = (*font).usedchars;
    if cff_dict_known(
        (*cffont).topdict,
        b"FontMatrix\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        scaling = 1000 as ::core::ffi::c_int as ::core::ffi::c_double
            * cff_dict_get(
                (*cffont).topdict,
                b"FontMatrix\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
    } else {
        scaling = 1.0f64;
    }
    array = pdf_new_array();
    if num_glyphs as ::core::ffi::c_int <= 1 as ::core::ffi::c_int {
        lastchar = 0 as ::core::ffi::c_int;
        firstchar = lastchar;
        pdf_add_array(array, pdf_new_number(0.0f64));
    } else {
        firstchar = 255 as ::core::ffi::c_int;
        lastchar = 0 as ::core::ffi::c_int;
        code = 0 as ::core::ffi::c_int;
        while code < 256 as ::core::ffi::c_int {
            if *usedchars.offset(code as isize) != 0 {
                if code < firstchar {
                    firstchar = code;
                }
                if code > lastchar {
                    lastchar = code;
                }
                *widths.offset(code as isize) *= scaling;
            }
            code += 1;
        }
        if firstchar > lastchar {
            pdf_abort!(b"No glyphs used at all!\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        pdf_check_tfm_widths((*font).ident, widths, firstchar, lastchar, usedchars);
        code = firstchar;
        while code <= lastchar {
            if *usedchars.offset(code as isize) != 0 {
                pdf_add_array(
                    array,
                    pdf_new_number(floor(*widths.offset(code as isize) / 0.1f64 + 0.5f64) * 0.1f64),
                );
            } else {
                pdf_add_array(array, pdf_new_number(0.0f64));
            }
            code += 1;
        }
    }
    if pdf_array_length(array) > 0 as ::core::ffi::c_uint {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"Widths\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(array),
        );
    }
    pdf_release_obj(array);
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"FirstChar\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(firstchar as ::core::ffi::c_double),
    );
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"LastChar\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(lastchar as ::core::ffi::c_double),
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_font_load_type1c")]
pub unsafe extern "C-unwind" fn pdf_font_load_type1c(
    mut font: *mut pdf_font,
) -> ::core::ffi::c_int {
    let mut fontdict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut descriptor: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut pdfcharset: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut usedchars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut uniqueTag: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fullname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut handle: rust_input_handle_t = 0;
    let mut encoding_id: ::core::ffi::c_int = 0;
    let mut fontfile: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut enc_vec: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut charstrings: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut topdict: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut cs_idx: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut charset: *mut cff_charsets = ::core::ptr::null_mut::<cff_charsets>();
    let mut encoding: *mut cff_encoding = ::core::ptr::null_mut::<cff_encoding>();
    let mut topdict_offset: ::core::ffi::c_int = 0;
    let mut private_size: ::core::ffi::c_int = 0;
    let mut charstring_len: ::core::ffi::c_int = 0;
    let mut max_len: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream_data_len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream_data_ptr: *mut card8 = ::core::ptr::null_mut::<card8>();
    let mut data: *mut card8 = ::core::ptr::null_mut::<card8>();
    let mut num_glyphs: card16 = 0;
    let mut cs_count: card16 = 0;
    let mut code: card16 = 0;
    let mut ginfo: cs_ginfo = cs_ginfo {
        flags: 0,
        wx: 0.,
        wy: 0.,
        bbox: C2RustUnnamed_6 {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        seac: C2RustUnnamed_5 {
            asb: 0.,
            adx: 0.,
            ady: 0.,
            bchar: 0,
            achar: 0,
        },
    };
    let mut nominal_width: ::core::ffi::c_double = 0.;
    let mut default_width: ::core::ffi::c_double = 0.;
    let mut notdef_width: ::core::ffi::c_double = 0.;
    let mut widths: [::core::ffi::c_double; 256] = [0.; 256];
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*font).reference.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if (*font).flags & PDF_FONT_FLAG_NOEMBED != 0 {
        pdf_abort!(
            b"Only embedded font supported for CFF/OpenType font.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    usedchars = (*font).usedchars;
    fontname = (*font).fontname;
    ident = (*font).filename;
    uniqueTag = pdf_font_get_uniqueTag(font);
    if usedchars.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"usedchars\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if fontname.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"fontname\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if ident.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ident\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    fontdict = pdf_font_get_resource(font);
    descriptor = pdf_font_get_descriptor(font);
    encoding_id = (*font).encoding_id;
    handle = dpx_open_opentype_file(ident);
    if handle == 0 {
        pdf_abort!(
            b"Could not open OpenType font: %s\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    }
    sfont = sfnt_open(handle);
    if sfont.is_null() {
        pdf_abort!(
            b"Could not open OpenType font: %s\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    }
    if sfnt_read_table_directory(sfont, 0 as SFNT_ULONG) < 0 as ::core::ffi::c_int {
        pdf_abort!(
            b"Could not read OpenType table directory: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            ident,
        );
    }
    if (*sfont).type_0 != SFNT_TYPE_POSTSCRIPT || {
        offset = sfnt_find_table_pos(sfont, b"CFF \0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int;
        offset == 0 as ::core::ffi::c_int
    } {
        pdf_abort!(
            b"Not a CFF/OpenType font (or variable font?) (11)?\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    cffont = cff_open(handle, offset, 0 as ::core::ffi::c_int);
    if cffont.is_null() {
        pdf_abort!(b"Could not open CFF font.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    if (*cffont).flag & FONTTYPE_CIDFONT != 0 {
        pdf_abort!(b"This is CIDFont...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    fullname = new(
        (strlen(fontname).wrapping_add(8 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    sprintf(
        fullname,
        b"%6s+%s\0" as *const u8 as *const ::core::ffi::c_char,
        uniqueTag,
        fontname,
    );
    cff_read_charsets(cffont);
    if encoding_id < 0 as ::core::ffi::c_int {
        cff_read_encoding(cffont);
    }
    cff_read_private(cffont);
    cff_read_subrs(cffont);
    (*cffont)._string = cff_new_index(0 as card16);
    charset = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_charsets>() as usize)
        as uint32_t) as *mut cff_charsets;
    (*charset).format = 0 as card8;
    (*charset).num_entries = 0 as card16;
    (*charset).data.glyphs = new((256 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<s_SID>() as usize)
        as uint32_t) as *mut s_SID;
    enc_vec = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if encoding_id >= 0 as ::core::ffi::c_int {
        enc_vec = pdf_encoding_get_encoding(encoding_id);
    } else {
        let mut tounicode: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        enc_vec = new((256 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as uint32_t) as *mut *mut ::core::ffi::c_char;
        code = 0 as card16;
        while (code as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
            if *usedchars.offset(code as isize) != 0 {
                let mut gid: card16 = 0;
                gid = cff_encoding_lookup(cffont, code as card8);
                let ref mut fresh0 = *enc_vec.offset(code as isize);
                *fresh0 = cff_get_string(cffont, cff_charsets_lookup_inverse(cffont, gid) as s_SID);
            } else {
                let ref mut fresh1 = *enc_vec.offset(code as isize);
                *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            code = code.wrapping_add(1);
        }
        if pdf_lookup_dict(
            fontdict,
            b"ToUnicode\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            tounicode = pdf_create_ToUnicode_CMap(fullname, enc_vec, usedchars);
            if !tounicode.is_null() {
                pdf_add_dict(
                    fontdict,
                    pdf_new_name(b"ToUnicode\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_ref_obj(tounicode),
                );
                pdf_release_obj(tounicode);
            }
        }
    }
    encoding = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_encoding>() as usize)
        as uint32_t) as *mut cff_encoding;
    (*encoding).format = 1 as card8;
    (*encoding).num_entries = 0 as card8;
    (*encoding).data.range1 = new((255 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_range1>() as usize)
        as uint32_t) as *mut cff_range1;
    (*encoding).num_supps = 0 as card8;
    (*encoding).supp = new((255 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_map>() as usize)
        as uint32_t) as *mut cff_map;
    offset = cff_dict_get(
        (*cffont).topdict,
        b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    crate::backend::runtime::input_seek(
        (*cffont).handle,
        (*cffont).offset.wrapping_add(offset as l_offset) as ssize_t,
        SEEK_SET,
    );
    cs_idx = cff_get_index_header(cffont);
    offset = crate::backend::runtime::input_seek((*cffont).handle, 0 as ssize_t, SEEK_CUR)
        as ::core::ffi::c_int;
    cs_count = (*cs_idx).count;
    if (cs_count as ::core::ffi::c_int) < 2 as ::core::ffi::c_int {
        pdf_abort!(b"No valid charstring data found.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    charstrings = cff_new_index(257 as card16);
    max_len = 2 as ::core::ffi::c_int * CS_STR_LEN_MAX;
    (*charstrings).data = new((max_len as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<card8>() as usize)
        as uint32_t) as *mut card8;
    charstring_len = 0 as ::core::ffi::c_int;
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
            descriptor,
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
    data = new((65536 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t)
        as *mut card8;
    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
        pdf_message!(b"[glyphs:/.notdef\0" as *const u8 as *const ::core::ffi::c_char);
    }
    size = (*(*cs_idx).offset.offset(1 as ::core::ffi::c_int as isize))
        .wrapping_sub(*(*cs_idx).offset.offset(0 as ::core::ffi::c_int as isize))
        as ::core::ffi::c_int;
    if size > CS_STR_LEN_MAX {
        pdf_abort!(
            b"Charstring too long: gid=%u, %d bytes\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            size,
        );
    }
    *(*charstrings)
        .offset
        .offset(0 as ::core::ffi::c_int as isize) =
        (charstring_len + 1 as ::core::ffi::c_int) as l_offset;
    crate::backend::runtime::input_seek(
        (*cffont).handle,
        (offset as l_offset)
            .wrapping_add(*(*cs_idx).offset.offset(0 as ::core::ffi::c_int as isize))
            .wrapping_sub(1 as l_offset) as ssize_t,
        SEEK_SET,
    );
    ttstub_input_read(
        (*cffont).handle,
        data as *mut ::core::ffi::c_char,
        size as size_t,
    );
    charstring_len += cs_copy_charstring(
        (*charstrings).data.offset(charstring_len as isize),
        max_len - charstring_len,
        data,
        size,
        (*cffont).gsubr,
        *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize),
        default_width,
        nominal_width,
        &raw mut ginfo,
    );
    notdef_width = ginfo.wx;
    num_glyphs = 1 as card16;
    pdfcharset = pdf_new_stream(0 as ::core::ffi::c_int);
    code = 0 as card16;
    while (code as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
        let mut gid_0: card16 = 0;
        let mut j: card16 = 0;
        let mut sid_orig: s_SID = 0;
        let mut sid: s_SID = 0;
        widths[code as usize] = notdef_width;
        if !(*usedchars.offset(code as isize) == 0
            || (*enc_vec.offset(code as isize)).is_null()
            || streq_ptr(
                *enc_vec.offset(code as isize),
                b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0)
        {
            sid_orig = cff_get_sid(cffont, *enc_vec.offset(code as isize)) as s_SID;
            sid = (if (sid_orig as ::core::ffi::c_int) < CFF_STDSTR_MAX {
                sid_orig as ::core::ffi::c_int
            } else {
                cff_add_string(
                    cffont,
                    *enc_vec.offset(code as isize),
                    0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int
            }) as s_SID;
            j = 0 as card16;
            while (j as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                if sid as ::core::ffi::c_int
                    == *(*charset).data.glyphs.offset(j as isize) as ::core::ffi::c_int
                {
                    (*(*encoding).supp.offset((*encoding).num_supps as isize)).code = code as card8;
                    (*(*encoding).supp.offset((*encoding).num_supps as isize)).glyph = sid;
                    *usedchars.offset(code as isize) = 0 as ::core::ffi::c_char;
                    (*encoding).num_supps = ((*encoding).num_supps as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as card8;
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
            if !((j as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int) {
                gid_0 = cff_charsets_lookup(cffont, sid_orig as card16);
                if gid_0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    pdf_warning!(
                        b"Glyph \"%s\" missing in font \"%s\".\0" as *const u8
                            as *const ::core::ffi::c_char,
                        *enc_vec.offset(code as isize),
                        fontname,
                    );
                    pdf_warning!(
                        b"Maybe incorrect encoding specified.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    *usedchars.offset(code as isize) = 0 as ::core::ffi::c_char;
                } else {
                    pdf_add_stream(
                        pdfcharset,
                        b"/\0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        1 as ::core::ffi::c_int,
                    );
                    pdf_add_stream(
                        pdfcharset,
                        *enc_vec.offset(code as isize) as *const ::core::ffi::c_void,
                        strlen(*enc_vec.offset(code as isize)) as ::core::ffi::c_int,
                    );
                    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
                        pdf_message!(
                            b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
                            *enc_vec.offset(code as isize),
                        );
                    }
                    size = (*(*cs_idx)
                        .offset
                        .offset((gid_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .wrapping_sub(*(*cs_idx).offset.offset(gid_0 as isize))
                        as ::core::ffi::c_int;
                    if size > CS_STR_LEN_MAX {
                        pdf_abort!(
                            b"Charstring too long: gid=%u, %d bytes\0" as *const u8
                                as *const ::core::ffi::c_char,
                            gid_0 as ::core::ffi::c_int,
                            size,
                        );
                    }
                    if charstring_len + CS_STR_LEN_MAX >= max_len {
                        max_len = charstring_len + 2 as ::core::ffi::c_int * CS_STR_LEN_MAX;
                        (*charstrings).data = renew(
                            (*charstrings).data as *mut ::core::ffi::c_void,
                            (max_len as uint32_t as usize)
                                .wrapping_mul(::core::mem::size_of::<card8>() as usize)
                                as uint32_t,
                        ) as *mut card8;
                    }
                    *(*charstrings).offset.offset(num_glyphs as isize) =
                        (charstring_len + 1 as ::core::ffi::c_int) as l_offset;
                    crate::backend::runtime::input_seek(
                        (*cffont).handle,
                        (offset as l_offset)
                            .wrapping_add(*(*cs_idx).offset.offset(gid_0 as isize))
                            .wrapping_sub(1 as l_offset) as ssize_t,
                        SEEK_SET,
                    );
                    ttstub_input_read(
                        (*cffont).handle,
                        data as *mut ::core::ffi::c_char,
                        size as size_t,
                    );
                    charstring_len += cs_copy_charstring(
                        (*charstrings).data.offset(charstring_len as isize),
                        max_len - charstring_len,
                        data,
                        size,
                        (*cffont).gsubr,
                        *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize),
                        default_width,
                        nominal_width,
                        &raw mut ginfo,
                    );
                    widths[code as usize] = ginfo.wx;
                    *(*charset)
                        .data
                        .glyphs
                        .offset((*charset).num_entries as isize) = sid;
                    (*charset).num_entries = ((*charset).num_entries as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as card16;
                    num_glyphs = num_glyphs.wrapping_add(1);
                }
            }
        }
        code = code.wrapping_add(1);
    }
    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
        pdf_message!(b"]\0" as *const u8 as *const ::core::ffi::c_char);
    }
    free(data as *mut ::core::ffi::c_void);
    if (*encoding).num_supps as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        (*encoding).format =
            ((*encoding).format as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as card8;
    } else {
        free((*encoding).supp as *mut ::core::ffi::c_void);
    }
    code = 0 as card16;
    while (code as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
        if !(*usedchars.offset(code as isize) == 0
            || (*enc_vec.offset(code as isize)).is_null()
            || streq_ptr(
                *enc_vec.offset(code as isize),
                b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0)
        {
            (*(*encoding)
                .data
                .range1
                .offset((*encoding).num_entries as isize))
            .first = code as s_SID;
            (*(*encoding)
                .data
                .range1
                .offset((*encoding).num_entries as isize))
            .n_left = 0 as card8;
            code = code.wrapping_add(1);
            while (code as ::core::ffi::c_int) < 256 as ::core::ffi::c_int
                && *usedchars.offset(code as isize) as ::core::ffi::c_int != 0
                && !(*enc_vec.offset(code as isize)).is_null()
                && strcmp(
                    *enc_vec.offset(code as isize),
                    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
            {
                let ref mut fresh2 = (*(*encoding)
                    .data
                    .range1
                    .offset((*encoding).num_entries as isize))
                .n_left;
                *fresh2 = (*fresh2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as card8;
                code = code.wrapping_add(1);
            }
            (*encoding).num_entries =
                ((*encoding).num_entries as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as card8;
        }
        code = code.wrapping_add(1);
    }
    if encoding_id < 0 as ::core::ffi::c_int && !enc_vec.is_null() {
        code = 0 as card16;
        while (code as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
            if !(*enc_vec.offset(code as isize)).is_null() {
                free(*enc_vec.offset(code as isize) as *mut ::core::ffi::c_void);
            }
            code = code.wrapping_add(1);
        }
        free(enc_vec as *mut ::core::ffi::c_void);
    }
    cff_release_index(cs_idx);
    *(*charstrings).offset.offset(num_glyphs as isize) =
        (charstring_len + 1 as ::core::ffi::c_int) as l_offset;
    (*charstrings).count = num_glyphs;
    charstring_len = cff_index_size(charstrings);
    (*cffont).num_glyphs = num_glyphs;
    if !(*cffont).charsets.is_null() {
        cff_release_charsets((*cffont).charsets);
    }
    (*cffont).charsets = charset;
    if !(*cffont).encoding.is_null() {
        cff_release_encoding((*cffont).encoding);
    }
    (*cffont).encoding = encoding;
    if !(*cffont).gsubr.is_null() {
        cff_release_index((*cffont).gsubr);
    }
    (*cffont).gsubr = cff_new_index(0 as card16);
    if !(*(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        cff_release_index(*(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize));
    }
    let ref mut fresh3 = *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize);
    *fresh3 = ::core::ptr::null_mut::<cff_index>();
    (*cffont).flag = FONTTYPE_FONT;
    cff_dict_update((*cffont).topdict, cffont);
    if !(*(*cffont).private.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        cff_dict_update(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            cffont,
        );
    }
    cff_update_string(cffont);
    topdict = cff_new_index(1 as card16);
    cff_dict_remove(
        (*cffont).topdict,
        b"UniqueID\0" as *const u8 as *const ::core::ffi::c_char,
    );
    cff_dict_remove(
        (*cffont).topdict,
        b"XUID\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if cff_dict_known(
        (*cffont).topdict,
        b"Encoding\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        cff_dict_add(
            (*cffont).topdict,
            b"Encoding\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
    }
    *(*topdict).offset.offset(1 as ::core::ffi::c_int as isize) = (cff_dict_pack(
        (*cffont).topdict,
        &raw mut work_buffer as *mut ::core::ffi::c_char as *mut card8,
        WORK_BUFFER_SIZE,
    ) + 1 as ::core::ffi::c_int)
        as l_offset;
    private_size = 0 as ::core::ffi::c_int;
    if !(*(*cffont).private.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        cff_dict_remove(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            b"Subrs\0" as *const u8 as *const ::core::ffi::c_char,
        );
        private_size = cff_dict_pack(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            &raw mut work_buffer as *mut ::core::ffi::c_char as *mut card8,
            WORK_BUFFER_SIZE,
        );
    }
    stream_data_len = 4 as ::core::ffi::c_int;
    stream_data_len += cff_set_name(cffont, fullname);
    free(fullname as *mut ::core::ffi::c_void);
    stream_data_len += cff_index_size(topdict);
    stream_data_len += cff_index_size((*cffont).string);
    stream_data_len += cff_index_size((*cffont).gsubr);
    stream_data_len += 2 as ::core::ffi::c_int
        + (*encoding).num_entries as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + (*encoding).num_supps as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
    stream_data_len += 1 as ::core::ffi::c_int
        + (*charset).num_entries as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
    stream_data_len += charstring_len;
    stream_data_len += private_size;
    stream_data_ptr = new((stream_data_len as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<card8>() as usize)
        as uint32_t) as *mut card8;
    offset = 0 as ::core::ffi::c_int;
    offset += cff_put_header(
        cffont,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    offset += cff_pack_index(
        (*cffont).name,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    topdict_offset = offset;
    offset += cff_index_size(topdict);
    offset += cff_pack_index(
        (*cffont).string,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    offset += cff_pack_index(
        (*cffont).gsubr,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"Encoding\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        offset as ::core::ffi::c_double,
    );
    offset += cff_pack_encoding(
        cffont,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"charset\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        offset as ::core::ffi::c_double,
    );
    offset += cff_pack_charsets(
        cffont,
        stream_data_ptr.offset(offset as isize),
        stream_data_len - offset,
    );
    cff_dict_set(
        (*cffont).topdict,
        b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        offset as ::core::ffi::c_double,
    );
    offset += cff_pack_index(
        charstrings,
        stream_data_ptr.offset(offset as isize),
        charstring_len,
    );
    cff_release_index(charstrings);
    cff_dict_set(
        (*cffont).topdict,
        b"Private\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        offset as ::core::ffi::c_double,
    );
    if !(*(*cffont).private.offset(0 as ::core::ffi::c_int as isize)).is_null()
        && private_size > 0 as ::core::ffi::c_int
    {
        private_size = cff_dict_pack(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            stream_data_ptr.offset(offset as isize),
            private_size,
        );
    }
    cff_dict_set(
        (*cffont).topdict,
        b"Private\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        private_size as ::core::ffi::c_double,
    );
    offset += private_size;
    (*topdict).data = new(
        ((*(*topdict).offset.offset(1 as ::core::ffi::c_int as isize)).wrapping_sub(1 as l_offset)
            as usize)
            .wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t,
    ) as *mut card8;
    cff_dict_pack(
        (*cffont).topdict,
        (*topdict).data,
        (*(*topdict).offset.offset(1 as ::core::ffi::c_int as isize)).wrapping_sub(1 as l_offset)
            as ::core::ffi::c_int,
    );
    cff_pack_index(
        topdict,
        stream_data_ptr.offset(topdict_offset as isize),
        cff_index_size(topdict),
    );
    cff_release_index(topdict);
    add_SimpleMetrics(
        font,
        cffont,
        &raw mut widths as *mut ::core::ffi::c_double,
        num_glyphs,
    );
    cff_close(cffont);
    sfnt_close(sfont);
    crate::backend::runtime::input_close(handle);
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        pdf_message!(
            b"[%u/%u glyphs][%d bytes]\0" as *const u8 as *const ::core::ffi::c_char,
            num_glyphs as ::core::ffi::c_int,
            cs_count as ::core::ffi::c_int,
            offset,
        );
    }
    if pdf_check_version(2 as ::core::ffi::c_int, 0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        pdf_add_dict(
            descriptor,
            pdf_new_name(b"CharSet\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                pdf_stream_dataptr(pdfcharset),
                pdf_stream_length(pdfcharset) as size_t,
            ),
        );
    }
    pdf_release_obj(pdfcharset);
    fontfile = pdf_new_stream(STREAM_COMPRESS);
    stream_dict = pdf_stream_dict(fontfile);
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"FontFile3\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj(fontfile),
    );
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Type1C\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_stream(
        fontfile,
        stream_data_ptr as *mut ::core::ffi::c_void,
        offset,
    );
    pdf_release_obj(fontfile);
    free(stream_data_ptr as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_open_type1c(
    mut font: *mut pdf_font,
    mut ident: *const ::core::ffi::c_char,
    mut index: ::core::ffi::c_int,
    mut encoding_id: ::core::ffi::c_int,
    mut embedding: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_font_open_type1c(font, ident, index, encoding_id, embedding)
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
pub unsafe extern "C" fn __pdf_entry_pdf_font_load_type1c(
    mut font: *mut pdf_font,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_load_type1c(font) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
