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
use crate::compat::allocation::free;
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
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __big: *const ::core::ffi::c_char,
        __little: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_open(
        path: *const ::core::ffi::c_char,
        format: ttbc_file_format,
        is_gz: ::core::ffi::c_int,
    ) -> rust_input_handle_t;
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
    #[link_name = "__pdf_impl_pdf_link_obj"]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
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
    #[link_name = "__pdf_impl_cff_close"]
    fn cff_close(cff: *mut cff_font);
    #[link_name = "__pdf_impl_cff_put_header"]
    fn cff_put_header(
        cff: *mut cff_font,
        dest: *mut card8,
        destlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    #[link_name = "__pdf_impl_cff_set_name"]
    fn cff_set_name(cff: *mut cff_font, name: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_pack_encoding"]
    fn cff_pack_encoding(
        cff: *mut cff_font,
        dest: *mut card8,
        destlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_pack_charsets"]
    fn cff_pack_charsets(
        cff: *mut cff_font,
        dest: *mut card8,
        destlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_glyph_lookup"]
    fn cff_glyph_lookup(cff: *mut cff_font, glyph: *const ::core::ffi::c_char) -> card16;
    #[link_name = "__pdf_impl_cff_release_charsets"]
    fn cff_release_charsets(charset: *mut cff_charsets);
    #[link_name = "__pdf_impl_cff_get_seac_sid"]
    fn cff_get_seac_sid(cff: *mut cff_font, str: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    static mut dpx_conf: _dpx_conf;
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
    #[link_name = "__pdf_impl_t1char_get_metrics"]
    fn t1char_get_metrics(
        src: *mut card8,
        srclen: ::core::ffi::c_int,
        subrs: *mut cff_index,
        ginfo: *mut t1_ginfo,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_t1char_convert_charstring"]
    fn t1char_convert_charstring(
        dst: *mut card8,
        dstlen: ::core::ffi::c_int,
        src: *mut card8,
        srclen: ::core::ffi::c_int,
        subrs: *mut cff_index,
        default_width: ::core::ffi::c_double,
        nominal_width: ::core::ffi::c_double,
        ginfo: *mut t1_ginfo,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_t1_load_font"]
    fn t1_load_font(
        enc_vec: *mut *mut ::core::ffi::c_char,
        mode: ::core::ffi::c_int,
        handle: rust_input_handle_t,
    ) -> *mut cff_font;
    #[link_name = "__pdf_impl_is_pfb"]
    fn is_pfb(handle: rust_input_handle_t) -> bool;
    #[link_name = "__pdf_impl_t1_get_fontname"]
    fn t1_get_fontname(
        handle: rust_input_handle_t,
        fontname: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_t1_get_standard_glyph"]
    fn t1_get_standard_glyph(code: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type rust_input_handle_t = uintptr_t;
pub type ttbc_file_format = ::core::ffi::c_uint;
pub const TTBC_FILE_FORMAT_VF: ttbc_file_format = 33;
pub const TTBC_FILE_FORMAT_TYPE1: ttbc_file_format = 32;
pub const TTBC_FILE_FORMAT_TRUE_TYPE: ttbc_file_format = 36;
pub const TTBC_FILE_FORMAT_TFM: ttbc_file_format = 3;
pub const TTBC_FILE_FORMAT_TEX_PS_HEADER: ttbc_file_format = 30;
pub const TTBC_FILE_FORMAT_TEX: ttbc_file_format = 26;
pub const TTBC_FILE_FORMAT_TECTONIC_PRIMARY: ttbc_file_format = 59;
pub const TTBC_FILE_FORMAT_SFD: ttbc_file_format = 46;
pub const TTBC_FILE_FORMAT_PROGRAM_DATA: ttbc_file_format = 39;
pub const TTBC_FILE_FORMAT_PK: ttbc_file_format = 1;
pub const TTBC_FILE_FORMAT_PICT: ttbc_file_format = 25;
pub const TTBC_FILE_FORMAT_OVF: ttbc_file_format = 23;
pub const TTBC_FILE_FORMAT_OPEN_TYPE: ttbc_file_format = 47;
pub const TTBC_FILE_FORMAT_OFM: ttbc_file_format = 20;
pub const TTBC_FILE_FORMAT_MISC_FONTS: ttbc_file_format = 41;
pub const TTBC_FILE_FORMAT_FONT_MAP: ttbc_file_format = 11;
pub const TTBC_FILE_FORMAT_FORMAT: ttbc_file_format = 10;
pub const TTBC_FILE_FORMAT_ENC: ttbc_file_format = 44;
pub const TTBC_FILE_FORMAT_CNF: ttbc_file_format = 8;
pub const TTBC_FILE_FORMAT_CMAP: ttbc_file_format = 45;
pub const TTBC_FILE_FORMAT_BST: ttbc_file_format = 7;
pub const TTBC_FILE_FORMAT_BIB: ttbc_file_format = 6;
pub const TTBC_FILE_FORMAT_AFM: ttbc_file_format = 4;
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
pub type card16 = ::core::ffi::c_ushort;
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
pub struct t1_ginfo {
    pub use_seac: ::core::ffi::c_int,
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
pub const PDF_NAME_LEN_MAX: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
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
pub const CS_STR_LEN_MAX: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PDF_FONT_FONTTYPE_TYPE1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PDF_FONT_FLAG_NOEMBED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PDF_FONT_FLAG_BASEFONT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const FONT_FLAG_FIXEDPITCH: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const FONT_FLAG_SERIF: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const FONT_FLAG_SYMBOLIC: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const FONT_FLAG_ITALIC: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const FONT_FLAG_SMALLCAP: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int;
pub const FONT_FLAG_FORCEBOLD: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn is_basefont(mut name: *const ::core::ffi::c_char) -> bool {
    static mut basefonts: [*const ::core::ffi::c_char; 14] = [
        b"Courier\0" as *const u8 as *const ::core::ffi::c_char,
        b"Courier-Bold\0" as *const u8 as *const ::core::ffi::c_char,
        b"Courier-Oblique\0" as *const u8 as *const ::core::ffi::c_char,
        b"Courier-BoldOblique\0" as *const u8 as *const ::core::ffi::c_char,
        b"Helvetica\0" as *const u8 as *const ::core::ffi::c_char,
        b"Helvetica-Bold\0" as *const u8 as *const ::core::ffi::c_char,
        b"Helvetica-Oblique\0" as *const u8 as *const ::core::ffi::c_char,
        b"Helvetica-BoldOblique\0" as *const u8 as *const ::core::ffi::c_char,
        b"Symbol\0" as *const u8 as *const ::core::ffi::c_char,
        b"Times-Roman\0" as *const u8 as *const ::core::ffi::c_char,
        b"Times-Bold\0" as *const u8 as *const ::core::ffi::c_char,
        b"Times-Italic\0" as *const u8 as *const ::core::ffi::c_char,
        b"Times-BoldItalic\0" as *const u8 as *const ::core::ffi::c_char,
        b"ZapfDingbats\0" as *const u8 as *const ::core::ffi::c_char,
    ];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 14 as ::core::ffi::c_int {
        if streq_ptr(name, basefonts[i as usize]) {
            return true_0 != 0;
        }
        i += 1;
    }
    return false_0 != 0;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_open_type1")]
pub unsafe extern "C-unwind" fn pdf_font_open_type1(
    mut font: *mut pdf_font,
    mut ident: *const ::core::ffi::c_char,
    mut index: ::core::ffi::c_int,
    mut encoding_id: ::core::ffi::c_int,
    mut embedding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fontname: [::core::ffi::c_char; 128] = [0; 128];
    if ident.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ident\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if index != 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Ignoring non-zero font index: %s\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    }
    if is_basefont(ident) {
        (*font).fontname = new(
            (strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy((*font).fontname, ident);
        (*font).subtype = PDF_FONT_FONTTYPE_TYPE1;
        (*font).flags |= PDF_FONT_FLAG_NOEMBED;
        (*font).flags |= PDF_FONT_FLAG_BASEFONT;
    } else {
        let mut handle: rust_input_handle_t = 0;
        handle = ttstub_input_open(ident, TTBC_FILE_FORMAT_TYPE1, 0 as ::core::ffi::c_int);
        if handle == INVALID_HANDLE {
            return -(1 as ::core::ffi::c_int);
        }
        if embedding == 0 {
            pdf_warning!(
                b"Ignoring no-embed option for Type1 font: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                ident,
            );
            (*font).flags &= !PDF_FONT_FLAG_NOEMBED;
        }
        memset(
            &raw mut fontname as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (PDF_NAME_LEN_MAX + 1 as ::core::ffi::c_int) as size_t,
        );
        if !is_pfb(handle)
            || t1_get_fontname(handle, &raw mut fontname as *mut ::core::ffi::c_char)
                < 0 as ::core::ffi::c_int
        {
            pdf_abort!(
                b"Failed to read Type 1 font \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                ident,
            );
        }
        crate::backend::runtime::input_close(handle);
        (*font).fontname = new((strlen(&raw mut fontname as *mut ::core::ffi::c_char)
            .wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        strcpy(
            (*font).fontname,
            &raw mut fontname as *mut ::core::ffi::c_char,
        );
        (*font).subtype = PDF_FONT_FONTTYPE_TYPE1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn get_font_attr(mut font: *mut pdf_font, mut cffont: *mut cff_font) {
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut descriptor: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
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
    static mut L_c: [*const ::core::ffi::c_char; 5] = [
        b"H\0" as *const u8 as *const ::core::ffi::c_char,
        b"P\0" as *const u8 as *const ::core::ffi::c_char,
        b"Pi\0" as *const u8 as *const ::core::ffi::c_char,
        b"Rho\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    static mut L_d: [*const ::core::ffi::c_char; 5] = [
        b"p\0" as *const u8 as *const ::core::ffi::c_char,
        b"q\0" as *const u8 as *const ::core::ffi::c_char,
        b"mu\0" as *const u8 as *const ::core::ffi::c_char,
        b"eta\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    static mut L_a: [*const ::core::ffi::c_char; 4] = [
        b"b\0" as *const u8 as *const ::core::ffi::c_char,
        b"h\0" as *const u8 as *const ::core::ffi::c_char,
        b"lambda\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut gm: t1_ginfo = t1_ginfo {
        use_seac: 0,
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
    defaultwidth = 500.0f64;
    nominalwidth = 0.0f64;
    if cff_dict_known(
        (*cffont).topdict,
        b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
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
    if cff_dict_known(
        (*cffont).topdict,
        b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
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
    gid = cff_glyph_lookup(
        cffont,
        b"space\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int;
    if gid >= 0 as ::core::ffi::c_int && gid < (*(*cffont).cstrings).count as ::core::ffi::c_int {
        t1char_get_metrics(
            (*(*cffont).cstrings)
                .data
                .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
            (*(*(*cffont).cstrings)
                .offset
                .offset((gid + 1 as ::core::ffi::c_int) as isize))
            .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                as ::core::ffi::c_int,
            *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize),
            &raw mut gm,
        );
        defaultwidth = gm.wx;
    }
    i = 0 as ::core::ffi::c_int;
    while !L_c[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_c[i as usize]) as ::core::ffi::c_int;
        if gid >= 0 as ::core::ffi::c_int && gid < (*(*cffont).cstrings).count as ::core::ffi::c_int
        {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)),
                (*(*(*cffont).cstrings)
                    .offset
                    .offset((gid + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                    as ::core::ffi::c_int,
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
    while !L_d[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_d[i as usize]) as ::core::ffi::c_int;
        if gid >= 0 as ::core::ffi::c_int && gid < (*(*cffont).cstrings).count as ::core::ffi::c_int
        {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)),
                (*(*(*cffont).cstrings)
                    .offset
                    .offset((gid + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                    as ::core::ffi::c_int,
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
    while !L_a[i as usize].is_null() {
        gid = cff_glyph_lookup(cffont, L_a[i as usize]) as ::core::ffi::c_int;
        if gid >= 0 as ::core::ffi::c_int && gid < (*(*cffont).cstrings).count as ::core::ffi::c_int
        {
            t1char_get_metrics(
                (*(*cffont).cstrings)
                    .data
                    .offset(*(*(*cffont).cstrings).offset.offset(gid as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)),
                (*(*(*cffont).cstrings)
                    .offset
                    .offset((gid + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid as isize))
                    as ::core::ffi::c_int,
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
    fontname = (*font).fontname;
    descriptor = pdf_font_get_descriptor(font);
    if !fontname.is_null()
        && strstr(
            fontname,
            b"Sans\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        flags |= FONT_FLAG_SERIF;
    }
    if !fontname.is_null()
        && !strstr(
            fontname,
            b"Caps\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        flags |= FONT_FLAG_SMALLCAP;
    }
    flags |= FONT_FLAG_SYMBOLIC;
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"CapHeight\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(capheight),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"Ascent\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(ascent),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"Descent\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(descent),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(italicangle),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"StemV\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(stemv),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"Flags\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(flags as ::core::ffi::c_double),
    );
}
unsafe extern "C-unwind" fn add_metrics(
    mut font: *mut pdf_font,
    mut cffont: *mut cff_font,
    mut enc_vec: *mut *mut ::core::ffi::c_char,
    mut widths: *mut ::core::ffi::c_double,
    mut num_glyphs: ::core::ffi::c_int,
) {
    let mut fontdict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut descriptor: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut code: ::core::ffi::c_int = 0;
    let mut firstchar: ::core::ffi::c_int = 0;
    let mut lastchar: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_double = 0.;
    let mut i: ::core::ffi::c_int = 0;
    let mut usedchars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut scaling: ::core::ffi::c_double = 1.0f64;
    let mut norm_widths: [::core::ffi::c_double; 256] = [0.; 256];
    fontdict = pdf_font_get_resource(font);
    descriptor = pdf_font_get_descriptor(font);
    usedchars = (*font).usedchars;
    if cff_dict_known(
        (*cffont).topdict,
        b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        pdf_abort!(b"No FontBBox?\0" as *const u8 as *const ::core::ffi::c_char);
    }
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
        scaling = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    array = pdf_new_array();
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        val = cff_dict_get(
            (*cffont).topdict,
            b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
        pdf_add_array(array, pdf_new_number(floor(val / 1.0f64 + 0.5f64) * 1.0f64));
        i += 1;
    }
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_link_obj(array),
    );
    pdf_release_obj(array);
    array = pdf_new_array();
    if num_glyphs <= 1 as ::core::ffi::c_int {
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
                norm_widths[code as usize] = scaling
                    * *widths
                        .offset(cff_glyph_lookup(cffont, *enc_vec.offset(code as isize)) as isize);
            }
            code += 1;
        }
        if firstchar > lastchar {
            pdf_warning!(
                b"No glyphs actually used???\0" as *const u8 as *const ::core::ffi::c_char,
            );
            pdf_release_obj(array);
            return;
        }
        pdf_check_tfm_widths(
            (*font).ident,
            &raw mut norm_widths as *mut ::core::ffi::c_double,
            firstchar,
            lastchar,
            usedchars,
        );
        code = firstchar;
        while code <= lastchar {
            if *usedchars.offset(code as isize) != 0 {
                pdf_add_array(
                    array,
                    pdf_new_number(floor(norm_widths[code as usize] / 0.1f64 + 0.5f64) * 0.1f64),
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
unsafe extern "C-unwind" fn write_fontfile(
    mut font: *mut pdf_font,
    mut cffont: *mut cff_font,
    mut pdfcharset: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut descriptor: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut fontfile: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut topdict: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut private_size: ::core::ffi::c_int = 0;
    let mut stream_data_len: ::core::ffi::c_int = 0;
    let mut charstring_len: ::core::ffi::c_int = 0;
    let mut topdict_offset: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0;
    let mut stream_data_ptr: *mut card8 = ::core::ptr::null_mut::<card8>();
    let mut wbuf: [card8; 1024] = [0; 1024];
    descriptor = pdf_font_get_descriptor(font);
    topdict = cff_new_index(1 as card16);
    if cff_dict_known(
        (*cffont).topdict,
        b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        cff_dict_add(
            (*cffont).topdict,
            b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
    }
    if cff_dict_known(
        (*cffont).topdict,
        b"charset\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        cff_dict_add(
            (*cffont).topdict,
            b"charset\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
    }
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
    private_size = cff_dict_pack(
        *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
        &raw mut wbuf as *mut card8,
        WBUF_SIZE,
    );
    if cff_dict_known(
        (*cffont).topdict,
        b"Private\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        cff_dict_add(
            (*cffont).topdict,
            b"Private\0" as *const u8 as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
        );
    }
    *(*topdict).offset.offset(1 as ::core::ffi::c_int as isize) =
        (cff_dict_pack((*cffont).topdict, &raw mut wbuf as *mut card8, WBUF_SIZE)
            + 1 as ::core::ffi::c_int) as l_offset;
    charstring_len = cff_index_size((*cffont).cstrings);
    stream_data_len = 4 as ::core::ffi::c_int;
    stream_data_len += cff_index_size((*cffont).name);
    stream_data_len += cff_index_size(topdict);
    stream_data_len += cff_index_size((*cffont).string);
    stream_data_len += cff_index_size((*cffont).gsubr);
    stream_data_len += 2 as ::core::ffi::c_int
        + (*(*cffont).encoding).num_entries as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + (*(*cffont).encoding).num_supps as ::core::ffi::c_int * 3 as ::core::ffi::c_int;
    stream_data_len += 1 as ::core::ffi::c_int
        + (*(*cffont).charsets).num_entries as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
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
        (*cffont).cstrings,
        stream_data_ptr.offset(offset as isize),
        charstring_len,
    );
    if !(*(*cffont).private.offset(0 as ::core::ffi::c_int as isize)).is_null()
        && private_size > 0 as ::core::ffi::c_int
    {
        private_size = cff_dict_pack(
            *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
            stream_data_ptr.offset(offset as isize),
            private_size,
        );
        cff_dict_set(
            (*cffont).topdict,
            b"Private\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            offset as ::core::ffi::c_double,
        );
        cff_dict_set(
            (*cffont).topdict,
            b"Private\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            private_size as ::core::ffi::c_double,
        );
    }
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
    free(stream_data_ptr as *mut ::core::ffi::c_void);
    return offset;
}
pub const WBUF_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_pdf_font_load_type1")]
pub unsafe extern "C-unwind" fn pdf_font_load_type1(mut font: *mut pdf_font) -> ::core::ffi::c_int {
    let mut fontdict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut pdfcharset: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut encoding_id: ::core::ffi::c_int = 0;
    let mut usedchars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut uniqueTag: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fullname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut charset: *mut cff_charsets = ::core::ptr::null_mut::<cff_charsets>();
    let mut enc_vec: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut defaultwidth: ::core::ffi::c_double = 0.;
    let mut nominalwidth: ::core::ffi::c_double = 0.;
    let mut widths: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<::core::ffi::c_double>();
    let mut GIDMap: *mut card16 = ::core::ptr::null_mut::<card16>();
    let mut num_glyphs: card16 = 0 as card16;
    let mut offset: ::core::ffi::c_int = 0;
    let mut code: ::core::ffi::c_int = 0;
    let mut handle: rust_input_handle_t = 0;
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
    fontdict = pdf_font_get_resource(font);
    encoding_id = (*font).encoding_id;
    usedchars = (*font).usedchars;
    ident = (*font).filename;
    fontname = (*font).fontname;
    uniqueTag = pdf_font_get_uniqueTag(font);
    if usedchars.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"usedchars\0" as *const u8 as *const ::core::ffi::c_char
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
    if fontname.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"fontname\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    handle = ttstub_input_open(ident, TTBC_FILE_FORMAT_TYPE1, 0 as ::core::ffi::c_int);
    if handle == INVALID_HANDLE {
        pdf_abort!(
            b"Type1: Could not open Type1 font: %s\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    }
    GIDMap = ::core::ptr::null_mut::<card16>();
    num_glyphs = 0 as card16;
    if encoding_id >= 0 as ::core::ffi::c_int {
        enc_vec = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    } else {
        enc_vec = new((256 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as uint32_t) as *mut *mut ::core::ffi::c_char;
        code = 0 as ::core::ffi::c_int;
        while code <= 0xff as ::core::ffi::c_int {
            let ref mut fresh0 = *enc_vec.offset(code as isize);
            *fresh0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            code += 1;
        }
    }
    cffont = t1_load_font(enc_vec, 0 as ::core::ffi::c_int, handle);
    if cffont.is_null() {
        pdf_abort!(
            b"Could not load Type 1 font: %s\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    }
    crate::backend::runtime::input_close(handle);
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
    if encoding_id >= 0 as ::core::ffi::c_int {
        enc_vec = pdf_encoding_get_encoding(encoding_id);
    } else {
        let mut tounicode: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
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
    cff_set_name(cffont, fullname);
    free(fullname as *mut ::core::ffi::c_void);
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
    GIDMap = new((1024 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<card16>() as usize) as uint32_t)
        as *mut card16;
    pdfcharset = pdf_new_stream(0 as ::core::ffi::c_int);
    let mut prev: ::core::ffi::c_int = 0;
    let mut duplicate: ::core::ffi::c_int = 0;
    let mut gid: ::core::ffi::c_int = 0;
    let mut glyph: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sid: s_SID = 0;
    (*cffont).encoding = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_encoding>() as usize)
        as uint32_t) as *mut cff_encoding;
    (*(*cffont).encoding).format = 1 as card8;
    (*(*cffont).encoding).num_entries = 0 as card8;
    (*(*cffont).encoding).data.range1 = new((256 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_range1>() as usize)
        as uint32_t) as *mut cff_range1;
    (*(*cffont).encoding).num_supps = 0 as card8;
    (*(*cffont).encoding).supp = new((256 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_map>() as usize)
        as uint32_t) as *mut cff_map;
    charset = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_charsets>() as usize)
        as uint32_t) as *mut cff_charsets;
    (*charset).format = 0 as card8;
    (*charset).num_entries = 0 as card16;
    (*charset).data.glyphs = new((1024 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<s_SID>() as usize)
        as uint32_t) as *mut s_SID;
    gid = cff_glyph_lookup(
        cffont,
        b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int;
    if gid < 0 as ::core::ffi::c_int {
        pdf_abort!(
            b"Type 1 font with no \".notdef\" glyph???\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    *GIDMap.offset(0 as ::core::ffi::c_int as isize) = gid as card16;
    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
        pdf_message!(b"[glyphs:/.notdef\0" as *const u8 as *const ::core::ffi::c_char);
    }
    num_glyphs = 1 as card16;
    prev = -(2 as ::core::ffi::c_int);
    code = 0 as ::core::ffi::c_int;
    while code <= 0xff as ::core::ffi::c_int {
        glyph = *enc_vec.offset(code as isize);
        if !(*usedchars.offset(code as isize) == 0) {
            if streq_ptr(
                glyph,
                b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                pdf_warning!(
                    b"Character mapped to .notdef used in font: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    fontname,
                );
                *usedchars.offset(code as isize) = 0 as ::core::ffi::c_char;
            } else {
                gid = cff_glyph_lookup(cffont, glyph) as ::core::ffi::c_int;
                if gid < 1 as ::core::ffi::c_int
                    || gid >= (*(*cffont).cstrings).count as ::core::ffi::c_int
                {
                    pdf_warning!(
                        b"Glyph \"%s\" missing in font \"%s\".\0" as *const u8
                            as *const ::core::ffi::c_char,
                        glyph,
                        fontname,
                    );
                    *usedchars.offset(code as isize) = 0 as ::core::ffi::c_char;
                } else {
                    duplicate = 0 as ::core::ffi::c_int;
                    while duplicate < code {
                        if *usedchars.offset(duplicate as isize) as ::core::ffi::c_int != 0
                            && !(*enc_vec.offset(duplicate as isize)).is_null()
                            && streq_ptr(*enc_vec.offset(duplicate as isize), glyph)
                                as ::core::ffi::c_int
                                != 0
                        {
                            break;
                        }
                        duplicate += 1;
                    }
                    sid = cff_add_string(cffont, glyph, 1 as ::core::ffi::c_int);
                    if duplicate < code {
                        (*(*(*cffont).encoding)
                            .supp
                            .offset((*(*cffont).encoding).num_supps as isize))
                        .code = duplicate as card8;
                        (*(*(*cffont).encoding)
                            .supp
                            .offset((*(*cffont).encoding).num_supps as isize))
                        .glyph = sid;
                        (*(*cffont).encoding).num_supps =
                            ((*(*cffont).encoding).num_supps as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int) as card8;
                    } else {
                        *GIDMap.offset(num_glyphs as isize) = gid as card16;
                        *(*charset)
                            .data
                            .glyphs
                            .offset((*charset).num_entries as isize) = sid;
                        (*charset).num_entries = ((*charset).num_entries as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)
                            as card16;
                        if code != prev + 1 as ::core::ffi::c_int {
                            (*(*cffont).encoding).num_entries = ((*(*cffont).encoding).num_entries
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as card8;
                            (*(*(*cffont).encoding).data.range1.offset(
                                ((*(*cffont).encoding).num_entries as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .first = code as s_SID;
                            (*(*(*cffont).encoding).data.range1.offset(
                                ((*(*cffont).encoding).num_entries as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .n_left = 0 as card8;
                        } else {
                            let ref mut fresh1 = (*(*(*cffont).encoding).data.range1.offset(
                                ((*(*cffont).encoding).num_entries as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .n_left;
                            *fresh1 =
                                (*fresh1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as card8;
                        }
                        prev = code;
                        num_glyphs = num_glyphs.wrapping_add(1);
                        if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
                            pdf_message!(
                                b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
                                glyph,
                            );
                        }
                        pdf_add_stream(
                            pdfcharset,
                            b"/\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            1 as ::core::ffi::c_int,
                        );
                        pdf_add_stream(
                            pdfcharset,
                            glyph as *const ::core::ffi::c_void,
                            strlen(glyph) as ::core::ffi::c_int,
                        );
                    }
                }
            }
        }
        code += 1;
    }
    if (*(*cffont).encoding).num_supps as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        (*(*cffont).encoding).format = ((*(*cffont).encoding).format as ::core::ffi::c_int
            | 0x80 as ::core::ffi::c_int) as card8;
    } else {
        (*(*cffont).encoding).supp =
            mfree((*(*cffont).encoding).supp as *mut ::core::ffi::c_void) as *mut cff_map;
    }
    widths = new(((*(*cffont).cstrings).count as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize)
        as uint32_t) as *mut ::core::ffi::c_double;
    let mut cstring: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut gm: t1_ginfo = t1_ginfo {
        use_seac: 0,
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
    let mut gid_0: card16 = 0;
    let mut gid_orig: card16 = 0;
    let mut dstlen_max: ::core::ffi::c_int = 0;
    let mut srclen: ::core::ffi::c_int = 0;
    let mut srcptr: *mut card8 = ::core::ptr::null_mut::<card8>();
    let mut dstptr: *mut card8 = ::core::ptr::null_mut::<card8>();
    dstlen_max = 0 as ::core::ffi::c_int;
    offset = dstlen_max;
    cstring = cff_new_index((*(*cffont).cstrings).count);
    (*cstring).data = ::core::ptr::null_mut::<card8>();
    *(*cstring).offset.offset(0 as ::core::ffi::c_int as isize) = 1 as l_offset;
    let mut current_block_155: u64;
    gid_0 = 0 as card16;
    while (gid_0 as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
        if offset + CS_STR_LEN_MAX >= dstlen_max {
            dstlen_max += CS_STR_LEN_MAX * 2 as ::core::ffi::c_int;
            (*cstring).data = renew(
                (*cstring).data as *mut ::core::ffi::c_void,
                (dstlen_max as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<card8>() as usize)
                    as uint32_t,
            ) as *mut card8;
        }
        gid_orig = *GIDMap.offset(gid_0 as isize);
        dstptr = (*cstring)
            .data
            .offset(*(*cstring).offset.offset(gid_0 as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        srcptr = (*(*cffont).cstrings)
            .data
            .offset(*(*(*cffont).cstrings).offset.offset(gid_orig as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        srclen = (*(*(*cffont).cstrings)
            .offset
            .offset((gid_orig as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
        .wrapping_sub(*(*(*cffont).cstrings).offset.offset(gid_orig as isize))
            as ::core::ffi::c_int;
        offset += t1char_convert_charstring(
            dstptr,
            CS_STR_LEN_MAX,
            srcptr,
            srclen,
            *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize),
            defaultwidth,
            nominalwidth,
            &raw mut gm,
        );
        *(*cstring)
            .offset
            .offset((gid_0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
            (offset + 1 as ::core::ffi::c_int) as l_offset;
        if gm.use_seac != 0 {
            let mut bchar_gid: ::core::ffi::c_int = 0;
            let mut achar_gid: ::core::ffi::c_int = 0;
            let mut i: ::core::ffi::c_int = 0;
            let mut bchar_name: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            let mut achar_name: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            achar_name = t1_get_standard_glyph(gm.seac.achar as ::core::ffi::c_int);
            achar_gid = cff_glyph_lookup(cffont, achar_name) as ::core::ffi::c_int;
            bchar_name = t1_get_standard_glyph(gm.seac.bchar as ::core::ffi::c_int);
            bchar_gid = cff_glyph_lookup(cffont, bchar_name) as ::core::ffi::c_int;
            if achar_gid < 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Accent char \"%s\" not found. Invalid use of \"seac\" operator.\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    achar_name,
                );
                current_block_155 = 9500030526577190060;
            } else if bchar_gid < 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Base char \"%s\" not found. Invalid use of \"seac\" operator.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    bchar_name,
                );
                current_block_155 = 9500030526577190060;
            } else {
                i = 0 as ::core::ffi::c_int;
                while i < num_glyphs as ::core::ffi::c_int {
                    if *GIDMap.offset(i as isize) as ::core::ffi::c_int == achar_gid {
                        break;
                    }
                    i += 1;
                }
                if i == num_glyphs as ::core::ffi::c_int {
                    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
                        pdf_message!(
                            b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
                            achar_name,
                        );
                    }
                    let fresh2 = num_glyphs;
                    num_glyphs = num_glyphs.wrapping_add(1);
                    *GIDMap.offset(fresh2 as isize) = achar_gid as card16;
                    *(*charset)
                        .data
                        .glyphs
                        .offset((*charset).num_entries as isize) =
                        cff_get_seac_sid(cffont, achar_name) as s_SID;
                    (*charset).num_entries = ((*charset).num_entries as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as card16;
                    pdf_add_stream(
                        pdfcharset,
                        b"/\0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        1 as ::core::ffi::c_int,
                    );
                    pdf_add_stream(
                        pdfcharset,
                        achar_name as *const ::core::ffi::c_void,
                        strlen(achar_name) as ::core::ffi::c_int,
                    );
                }
                i = 0 as ::core::ffi::c_int;
                while i < num_glyphs as ::core::ffi::c_int {
                    if *GIDMap.offset(i as isize) as ::core::ffi::c_int == bchar_gid {
                        break;
                    }
                    i += 1;
                }
                if i == num_glyphs as ::core::ffi::c_int {
                    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
                        pdf_message!(
                            b"/%s\0" as *const u8 as *const ::core::ffi::c_char,
                            bchar_name,
                        );
                    }
                    let fresh3 = num_glyphs;
                    num_glyphs = num_glyphs.wrapping_add(1);
                    *GIDMap.offset(fresh3 as isize) = bchar_gid as card16;
                    *(*charset)
                        .data
                        .glyphs
                        .offset((*charset).num_entries as isize) =
                        cff_get_seac_sid(cffont, bchar_name) as s_SID;
                    (*charset).num_entries = ((*charset).num_entries as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int)
                        as card16;
                    pdf_add_stream(
                        pdfcharset,
                        b"/\0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        1 as ::core::ffi::c_int,
                    );
                    pdf_add_stream(
                        pdfcharset,
                        achar_name as *const ::core::ffi::c_void,
                        strlen(achar_name) as ::core::ffi::c_int,
                    );
                }
                current_block_155 = 12608488225262500095;
            }
        } else {
            current_block_155 = 12608488225262500095;
        }
        match current_block_155 {
            12608488225262500095 => {
                *widths.offset(gid_0 as isize) = gm.wx;
            }
            _ => {}
        }
        gid_0 = gid_0.wrapping_add(1);
    }
    (*cstring).count = num_glyphs;
    cff_release_index(*(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize));
    let ref mut fresh4 = *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize);
    *fresh4 = ::core::ptr::null_mut::<cff_index>();
    (*cffont).subrs = mfree((*cffont).subrs as *mut ::core::ffi::c_void) as *mut *mut cff_index;
    cff_release_index((*cffont).cstrings);
    (*cffont).cstrings = cstring;
    cff_release_charsets((*cffont).charsets);
    (*cffont).charsets = charset;
    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
        pdf_message!(b"]\0" as *const u8 as *const ::core::ffi::c_char);
    }
    cff_dict_update((*cffont).topdict, cffont);
    cff_dict_update(
        *(*cffont).private.offset(0 as ::core::ffi::c_int as isize),
        cffont,
    );
    cff_update_string(cffont);
    add_metrics(
        font,
        cffont,
        enc_vec,
        widths,
        num_glyphs as ::core::ffi::c_int,
    );
    offset = write_fontfile(font, cffont, pdfcharset);
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        pdf_message!(
            b"[%u glyphs][%d bytes]\0" as *const u8 as *const ::core::ffi::c_char,
            num_glyphs as ::core::ffi::c_int,
            offset,
        );
    }
    pdf_release_obj(pdfcharset);
    cff_close(cffont);
    if encoding_id < 0 as ::core::ffi::c_int && !enc_vec.is_null() {
        code = 0 as ::core::ffi::c_int;
        while code < 256 as ::core::ffi::c_int {
            let ref mut fresh5 = *enc_vec.offset(code as isize);
            *fresh5 = mfree(*enc_vec.offset(code as isize) as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char;
            code += 1;
        }
        free(enc_vec as *mut ::core::ffi::c_void);
    }
    free(widths as *mut ::core::ffi::c_void);
    free(GIDMap as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_open_type1(
    mut font: *mut pdf_font,
    mut ident: *const ::core::ffi::c_char,
    mut index: ::core::ffi::c_int,
    mut encoding_id: ::core::ffi::c_int,
    mut embedding: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_font_open_type1(font, ident, index, encoding_id, embedding)
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
pub unsafe extern "C" fn __pdf_entry_pdf_font_load_type1(
    mut font: *mut pdf_font,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_load_type1(font) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
