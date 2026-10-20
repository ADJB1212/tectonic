/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2020 by Jin-Hwan Cho and Shunsaku Hirata,
   the dvipdfmx project team.

   Copyright (C) 2012-2015 by Khaled Hosny <khaledhosny@eglug.org>

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
#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn, static_mut_refs, dangerous_implicit_autorefs)]
use crate::backend::dpx_pdfdev::{pdf_dev_set_dirmode, pdf_dev_set_rect};
use crate::compat::allocation::free;
use crate::numbers::scaled_fixword as sqxfw;
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn round(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strtol(__str: *const ::core::ffi::c_char, __endptr: *mut *mut ::core::ffi::c_char, __base: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn strtoul(__str: *const ::core::ffi::c_char, __endptr: *mut *mut ::core::ffi::c_char, __base: ::core::ffi::c_int) -> ::core::ffi::c_ulong;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcpy(__dst: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strncpy(__dst: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char, __n: size_t) -> *mut ::core::ffi::c_char;
    fn strrchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    #[cfg_attr(windows, link_name = "_stricmp")]
    fn strcasecmp(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn xmalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_open(path: *const ::core::ffi::c_char, format: ttbc_file_format, is_gz: ::core::ffi::c_int) -> rust_input_handle_t;
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: ::core::ffi::c_int) -> size_t;
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut ::core::ffi::c_char, len: size_t) -> ssize_t;
    fn ttstub_input_getc(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn ttstub_input_ungetc(handle: rust_input_handle_t, ch: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(_: *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_tt_skip_bytes"]
    fn tt_skip_bytes(n: ::core::ffi::c_uint, handle: rust_input_handle_t);
    #[link_name = "__pdf_impl_tt_get_unsigned_byte"]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_tt_get_unsigned_quad"]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    #[link_name = "__pdf_impl_tt_get_signed_quad"]
    fn tt_get_signed_quad(handle: rust_input_handle_t) -> int32_t;
    #[link_name = "__pdf_impl_tt_get_unsigned_num"]
    fn tt_get_unsigned_num(handle: rust_input_handle_t, num: ::core::ffi::c_uchar) -> uint32_t;
    #[link_name = "__pdf_impl_tt_get_positive_quad"]
    fn tt_get_positive_quad(handle: rust_input_handle_t, type_0: *const ::core::ffi::c_char, name: *const ::core::ffi::c_char) -> uint32_t;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_number_value"]
    fn pdf_number_value(number: *mut pdf_obj) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pdf_string_value"]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_string_length"]
    fn pdf_string_length(object: *mut pdf_obj) -> ::core::ffi::c_uint;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_get_array"]
    fn pdf_get_array(array: *mut pdf_obj, idx: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_array_length"]
    fn pdf_array_length(array: *mut pdf_obj) -> ::core::ffi::c_uint;
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_rgbcolor"]
    fn pdf_color_rgbcolor(color: *mut pdf_color, r: ::core::ffi::c_double, g: ::core::ffi::c_double, b: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_push"]
    fn pdf_color_push(sc: *mut pdf_color, fc: *mut pdf_color);
    #[link_name = "__pdf_impl_pdf_color_pop"]
    fn pdf_color_pop();
    #[link_name = "__pdf_impl_pdf_dev_set_string"]
    fn pdf_dev_set_string(
        xpos: spt_t, ypos: spt_t, instr_ptr: *const ::core::ffi::c_void, instr_len: size_t, text_width: spt_t, font_id: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_dev_set_rule"]
    fn pdf_dev_set_rule(xpos: spt_t, ypos: spt_t, width: spt_t, height: spt_t);
    #[link_name = "__pdf_impl_pdf_dev_locate_font"]
    fn pdf_dev_locate_font(font_name: *const ::core::ffi::c_char, ptsize: spt_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_graphics_mode"]
    fn graphics_mode();
    #[link_name = "__pdf_impl_pdf_dev_begin_actualtext"]
    fn pdf_dev_begin_actualtext(unicodes: *mut uint16_t, len: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_dev_end_actualtext"]
    fn pdf_dev_end_actualtext();
    #[link_name = "__pdf_impl_pdf_dev_font_minbytes"]
    fn pdf_dev_font_minbytes(font_id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tt_pdf_special_supported(data: *const ::core::ffi::c_uchar, length: size_t) -> ::core::ffi::c_int;
    fn tt_pdf_preflight_pages(
        handle: rust_input_handle_t, offsets: *const uint32_t, count: size_t, file_size: uint32_t, xdv: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tt_pdf_scan_page(
        handle: rust_input_handle_t, start: uint32_t, end: uint32_t, xdv: ::core::ffi::c_int, output: *mut tt_pdf_scanned_page,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_close"]
    fn cff_close(cff: *mut cff_font);
    static mut dpx_conf: _dpx_conf;
    #[link_name = "__pdf_impl_dpx_open_type1_file"]
    fn dpx_open_type1_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_dpx_open_truetype_file"]
    fn dpx_open_truetype_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_dpx_open_opentype_file"]
    fn dpx_open_opentype_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_dpx_open_dfont_file"]
    fn dpx_open_dfont_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_dpx_util_read_length"]
    fn dpx_util_read_length(
        vp: *mut ::core::ffi::c_double, mag: ::core::ffi::c_double, curptr: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_parse_float_decimal"]
    fn parse_float_decimal(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_c_string"]
    fn parse_c_string(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_c_ident"]
    fn parse_c_ident(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut landscape_mode: ::core::ffi::c_int;
    static mut paper_height: ::core::ffi::c_double;
    static mut paper_width: ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pdf_lookup_fontmap_record"]
    fn pdf_lookup_fontmap_record(kp: *const ::core::ffi::c_char) -> *mut fontmap_rec;
    #[link_name = "__pdf_impl_pdf_insert_native_fontmap_record"]
    fn pdf_insert_native_fontmap_record(
        filename: *const ::core::ffi::c_char, index: uint32_t, layout_dir: ::core::ffi::c_int, extend: ::core::ffi::c_int, slant: ::core::ffi::c_int,
        embolden: ::core::ffi::c_int,
    ) -> *mut fontmap_rec;
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_doc_begin_page"]
    fn pdf_doc_begin_page(scale: ::core::ffi::c_double, x_origin: ::core::ffi::c_double, y_origin: ::core::ffi::c_double);
    #[link_name = "__pdf_impl_pdf_doc_end_page"]
    fn pdf_doc_end_page();
    #[link_name = "__pdf_impl_pdf_doc_add_page_content"]
    fn pdf_doc_add_page_content(buffer: *const ::core::ffi::c_char, length: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_doc_add_page_resource"]
    fn pdf_doc_add_page_resource(category: *const ::core::ffi::c_char, resource_name: *const ::core::ffi::c_char, resources: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_doc_break_annot"]
    fn pdf_doc_break_annot();
    #[link_name = "__pdf_impl_pdf_doc_expand_box"]
    fn pdf_doc_expand_box(rect: *const pdf_rect);
    #[link_name = "__pdf_impl_pdf_dev_gsave"]
    fn pdf_dev_gsave() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_grestore"]
    fn pdf_dev_grestore() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_dump"]
    fn dump(start: *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_parse_ident"]
    fn parse_ident(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_pdf_number"]
    fn parse_pdf_number(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_string"]
    fn parse_pdf_string(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_array"]
    fn parse_pdf_array(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char, pf: *mut pdf_file) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_defineresource"]
    fn pdf_defineresource(
        category: *const ::core::ffi::c_char, resname: *const ::core::ffi::c_char, object: *mut pdf_obj, flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_resource_reference"]
    fn pdf_get_resource_reference(res_id: ::core::ffi::c_int) -> *mut pdf_obj;
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
    #[link_name = "__pdf_impl_spc_exec_at_begin_page"]
    fn spc_exec_at_begin_page() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_exec_at_end_page"]
    fn spc_exec_at_end_page() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_exec_special"]
    fn spc_exec_special(
        p: *const ::core::ffi::c_char, size: int32_t, x_user: ::core::ffi::c_double, y_user: ::core::ffi::c_double, mag: ::core::ffi::c_double,
        is_drawable: *mut ::core::ffi::c_int, rect: *mut pdf_rect,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_lookup_sfd_record"]
    fn lookup_sfd_record(rec_id: ::core::ffi::c_int, code: ::core::ffi::c_uchar) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_sfd_load_record"]
    fn sfd_load_record(sfd_name: *const ::core::ffi::c_char, subfont_id: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_t1char_get_metrics"]
    fn t1char_get_metrics(src: *mut card8, srclen: ::core::ffi::c_int, subrs: *mut cff_index, ginfo: *mut t1_ginfo) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_t1_load_font"]
    fn t1_load_font(enc_vec: *mut *mut ::core::ffi::c_char, mode: ::core::ffi::c_int, handle: rust_input_handle_t) -> *mut cff_font;
    #[link_name = "__pdf_impl_tfm_open"]
    fn tfm_open(tex_name: *const ::core::ffi::c_char, must_exist: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tfm_close_all"]
    fn tfm_close_all();
    #[link_name = "__pdf_impl_tfm_get_fw_width"]
    fn tfm_get_fw_width(font_id: ::core::ffi::c_int, ch: int32_t) -> fixword;
    #[link_name = "__pdf_impl_tfm_get_fw_height"]
    fn tfm_get_fw_height(font_id: ::core::ffi::c_int, ch: int32_t) -> fixword;
    #[link_name = "__pdf_impl_tfm_get_fw_depth"]
    fn tfm_get_fw_depth(font_id: ::core::ffi::c_int, ch: int32_t) -> fixword;
    #[link_name = "__pdf_impl_tfm_is_jfm"]
    fn tfm_is_jfm(font_id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_ttc_read_offset"]
    fn ttc_read_offset(sfont: *mut sfnt, ttc_idx: SFNT_ULONG) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_tt_read_head_table"]
    fn tt_read_head_table(sfont: *mut sfnt) -> *mut tt_head_table;
    #[link_name = "__pdf_impl_tt_read_hhea_table"]
    fn tt_read_hhea_table(sfont: *mut sfnt) -> *mut tt_hhea_table;
    #[link_name = "__pdf_impl_tt_read_maxp_table"]
    fn tt_read_maxp_table(sfont: *mut sfnt) -> *mut tt_maxp_table;
    #[link_name = "__pdf_impl_tt_read_vhea_table"]
    fn tt_read_vhea_table(sfont: *mut sfnt) -> *mut tt_vhea_table;
    #[link_name = "__pdf_impl_tt_read_longMetrics"]
    fn tt_read_longMetrics(sfont: *mut sfnt, numGlyphs: USHORT, numLongMetrics: USHORT, numExSideBearings: USHORT) -> *mut tt_longMetrics;
    #[link_name = "__pdf_impl_vf_locate_font"]
    fn vf_locate_font(tex_name: *const ::core::ffi::c_char, ptsize: spt_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_vf_set_char"]
    fn vf_set_char(ch: int32_t, vf_font: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_vf_close_all_fonts"]
    fn vf_close_all_fonts();
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
pub type off_t = __darwin_off_t;
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
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub _read: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_char, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub _seek: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3],
    pub _nbuf: [::core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type fixword = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_color {
    pub res_id: ::core::ffi::c_int,
    pub type_0: ::core::ffi::c_int,
    pub num_components: ::core::ffi::c_int,
    pub spot_color_name: *mut ::core::ffi::c_char,
    pub values: [::core::ffi::c_double; 32],
    pub pattern_id: ::core::ffi::c_int,
}
pub type spt_t = ::core::ffi::c_int;
use crate::compat::renderer::pdf_rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvi_registers {
    pub h: int32_t,
    pub v: int32_t,
    pub w: int32_t,
    pub x: int32_t,
    pub y: int32_t,
    pub z: int32_t,
    pub d: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_def {
    pub tex_id: int32_t,
    pub point_size: spt_t,
    pub design_size: spt_t,
    pub font_name: *mut ::core::ffi::c_char,
    pub font_id: ::core::ffi::c_int,
    pub used: ::core::ffi::c_int,
    pub native: ::core::ffi::c_int,
    pub rgba_color: uint32_t,
    pub rgba_used: uint8_t,
    pub face_index: uint32_t,
    pub layout_dir: ::core::ffi::c_int,
    pub extend: ::core::ffi::c_int,
    pub slant: ::core::ffi::c_int,
    pub embolden: ::core::ffi::c_int,
}
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
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvi_header {
    pub unit_num: uint32_t,
    pub unit_den: uint32_t,
    pub mag: uint32_t,
    pub media_width: uint32_t,
    pub media_height: uint32_t,
    pub stackdepth: ::core::ffi::c_uint,
    pub comment: [::core::ffi::c_char; 257],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loaded_font {
    pub type_0: ::core::ffi::c_int,
    pub font_id: ::core::ffi::c_int,
    pub subfont_id: ::core::ffi::c_int,
    pub tfm_id: ::core::ffi::c_int,
    pub size: spt_t,
    pub source: ::core::ffi::c_int,
    pub rgba_color: uint32_t,
    pub rgba_used: uint8_t,
    pub xgs_id: ::core::ffi::c_int,
    pub gm: *mut gm,
    pub shift_gid: ::core::ffi::c_int,
    pub num_glyphs: uint16_t,
    pub layout_dir: ::core::ffi::c_int,
    pub extend: ::core::ffi::c_float,
    pub slant: ::core::ffi::c_float,
    pub embolden: ::core::ffi::c_float,
    pub is_unicode: ::core::ffi::c_int,
    pub minbytes: ::core::ffi::c_int,
    pub padbytes: [::core::ffi::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gm {
    pub advance: spt_t,
    pub ascent: spt_t,
    pub descent: spt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spt_coord {
    pub x: spt_t,
    pub y: spt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvi_lr {
    pub state: ::core::ffi::c_int,
    pub font: ::core::ffi::c_int,
    pub buf_index: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_opt {
    pub slant: ::core::ffi::c_double,
    pub extend: ::core::ffi::c_double,
    pub bold: ::core::ffi::c_double,
    pub mapc: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub otl_tags: *mut ::core::ffi::c_char,
    pub tounicode: *mut ::core::ffi::c_char,
    pub design_size: ::core::ffi::c_double,
    pub charcoll: *mut ::core::ffi::c_char,
    pub index: uint32_t,
    pub style: ::core::ffi::c_int,
    pub stemv: ::core::ffi::c_int,
    pub use_glyph_encoding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_rec {
    pub map_name: *mut ::core::ffi::c_char,
    pub font_name: *mut ::core::ffi::c_char,
    pub enc_name: *mut ::core::ffi::c_char,
    pub charmap: C2RustUnnamed_0,
    pub opt: fontmap_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub sfd_name: *mut ::core::ffi::c_char,
    pub subfont_id: *mut ::core::ffi::c_char,
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
pub struct C2RustUnnamed_4 {
    pub llx: ::core::ffi::c_double,
    pub lly: ::core::ffi::c_double,
    pub urx: ::core::ffi::c_double,
    pub ury: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_ginfo {
    pub use_seac: ::core::ffi::c_int,
    pub wx: ::core::ffi::c_double,
    pub wy: ::core::ffi::c_double,
    pub bbox: C2RustUnnamed_4,
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
pub struct tt_pdf_special_span {
    pub offset: size_t,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_pdf_scanned_page {
    pub data: *const ::core::ffi::c_uchar,
    pub length: size_t,
    pub specials: *const tt_pdf_special_span,
    pub special_count: size_t,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
#[inline]
unsafe extern "C-unwind" fn streq_ptr(mut s1: *const ::core::ffi::c_char, mut s2: *const ::core::ffi::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
    }
    return false_0 != 0;
}
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SET_CHAR_127: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const SET1: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const SET2: ::core::ffi::c_int = 129;
pub const SET3: ::core::ffi::c_int = 130;
pub const SET4: ::core::ffi::c_int = 131;
pub const SET_RULE: ::core::ffi::c_int = 132;
pub const PUT1: ::core::ffi::c_int = 133 as ::core::ffi::c_int;
pub const PUT2: ::core::ffi::c_int = 134;
pub const PUT3: ::core::ffi::c_int = 135;
pub const PUT4: ::core::ffi::c_int = 136;
pub const PUT_RULE: ::core::ffi::c_int = 137;
pub const NOP: ::core::ffi::c_int = 138;
pub const BOP: ::core::ffi::c_int = 139;
pub const EOP: ::core::ffi::c_int = 140;
pub const PUSH: ::core::ffi::c_int = 141;
pub const POP: ::core::ffi::c_int = 142;
pub const RIGHT1: ::core::ffi::c_int = 143 as ::core::ffi::c_int;
pub const RIGHT2: ::core::ffi::c_int = 144;
pub const RIGHT3: ::core::ffi::c_int = 145;
pub const RIGHT4: ::core::ffi::c_int = 146;
pub const W0: ::core::ffi::c_int = 147;
pub const W1: ::core::ffi::c_int = 148 as ::core::ffi::c_int;
pub const W2: ::core::ffi::c_int = 149;
pub const W3: ::core::ffi::c_int = 150;
pub const W4: ::core::ffi::c_int = 151;
pub const X0: ::core::ffi::c_int = 152;
pub const X1: ::core::ffi::c_int = 153 as ::core::ffi::c_int;
pub const X2: ::core::ffi::c_int = 154;
pub const X3: ::core::ffi::c_int = 155;
pub const X4: ::core::ffi::c_int = 156;
pub const DOWN1: ::core::ffi::c_int = 157 as ::core::ffi::c_int;
pub const DOWN2: ::core::ffi::c_int = 158;
pub const DOWN3: ::core::ffi::c_int = 159;
pub const DOWN4: ::core::ffi::c_int = 160;
pub const Y0: ::core::ffi::c_int = 161;
pub const Y1: ::core::ffi::c_int = 162 as ::core::ffi::c_int;
pub const Y2: ::core::ffi::c_int = 163;
pub const Y3: ::core::ffi::c_int = 164;
pub const Y4: ::core::ffi::c_int = 165;
pub const Z0: ::core::ffi::c_int = 166;
pub const Z1: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const Z2: ::core::ffi::c_int = 168;
pub const Z3: ::core::ffi::c_int = 169;
pub const Z4: ::core::ffi::c_int = 170;
pub const FNT_NUM_0: ::core::ffi::c_int = 171 as ::core::ffi::c_int;
pub const FNT_NUM_63: ::core::ffi::c_int = 234 as ::core::ffi::c_int;
pub const FNT1: ::core::ffi::c_int = 235 as ::core::ffi::c_int;
pub const FNT2: ::core::ffi::c_int = 236;
pub const FNT3: ::core::ffi::c_int = 237;
pub const FNT4: ::core::ffi::c_int = 238;
pub const XXX1: ::core::ffi::c_int = 239 as ::core::ffi::c_int;
pub const XXX2: ::core::ffi::c_int = 240;
pub const XXX3: ::core::ffi::c_int = 241;
pub const XXX4: ::core::ffi::c_int = 242;
pub const FNT_DEF1: ::core::ffi::c_int = 243 as ::core::ffi::c_int;
pub const FNT_DEF2: ::core::ffi::c_int = 244;
pub const FNT_DEF3: ::core::ffi::c_int = 245;
pub const FNT_DEF4: ::core::ffi::c_int = 246;
pub const PRE: ::core::ffi::c_int = 247 as ::core::ffi::c_int;
pub const DVI_ID: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DVIV_ID: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const XDV_ID_OLD: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const XDV_ID: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const POST: ::core::ffi::c_int = 248 as ::core::ffi::c_int;
pub const POST_POST: ::core::ffi::c_int = 249 as ::core::ffi::c_int;
pub const PADDING: ::core::ffi::c_int = 223 as ::core::ffi::c_int;
pub const BEGIN_REFLECT: ::core::ffi::c_int = 250;
pub const END_REFLECT: ::core::ffi::c_int = 251;
pub const XDV_NATIVE_FONT_DEF: ::core::ffi::c_int = 252;
pub const XDV_GLYPHS: ::core::ffi::c_int = 253;
pub const XDV_TEXT_AND_GLYPHS: ::core::ffi::c_int = 254;
pub const PTEXDIR: ::core::ffi::c_int = 255;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const PDF_NUMBER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_STRING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SFNT_TYPE_TTC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const SFNT_TYPE_DFONT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const DVI_STACK_DEPTH_MAX: ::core::ffi::c_uint = 256 as ::core::ffi::c_uint;
pub const TEX_FONTS_ALLOC_SIZE: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const VF_NESTING_MAX: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;

#[unsafe(export_name = "__pdf_impl_get_origin")]
pub unsafe extern "C-unwind" fn get_origin(mut x: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let __pdf_state = unsafe { document_state() };
    return if x != 0 {
        (*__pdf_state).dev_origin_x
    } else {
        (*__pdf_state).dev_origin_y
    };
}
pub const LTYPESETTING: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RTYPESETTING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SKIMMING: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

pub const PHYSICAL: ::core::ffi::c_int = 1;
pub const VIRTUAL: ::core::ffi::c_int = 2;
pub const NATIVE: ::core::ffi::c_int = 4;
pub const DVI: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

unsafe extern "C-unwind" fn need_more_fonts(mut n: ::core::ffi::c_uint) {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).num_loaded_fonts.wrapping_add(n) > (*__pdf_state).max_loaded_fonts {
        (*__pdf_state).max_loaded_fonts = (*__pdf_state).max_loaded_fonts.wrapping_add(TEX_FONTS_ALLOC_SIZE);
        (*__pdf_state).loaded_fonts = renew(
            (*__pdf_state).loaded_fonts as *mut ::core::ffi::c_void,
            ((*__pdf_state).max_loaded_fonts as uint32_t as usize).wrapping_mul(::core::mem::size_of::<loaded_font>() as usize) as uint32_t,
        ) as *mut loaded_font as *mut loaded_font;
    }
}

pub const XDV_FLAG_VERTICAL: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const XDV_FLAG_COLORED: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const XDV_FLAG_EXTEND: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const XDV_FLAG_SLANT: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const XDV_FLAG_EMBOLDEN: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;

pub const DVI_PAGE_BUF_CHUNK: ::core::ffi::c_uint = 0x10000 as ::core::ffi::c_uint;

unsafe extern "C-unwind" fn get_and_buffer_unsigned_byte(mut handle: rust_input_handle_t) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut ch: ::core::ffi::c_int = 0;
    ch = ttstub_input_getc(handle);
    if ch < 0 as ::core::ffi::c_int {
        pdf_abort!(b"File ended prematurely\n\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    if (*__pdf_state).dvi_page_buf_index >= (*__pdf_state).dvi_page_buf_size {
        (*__pdf_state).dvi_page_buf_size = (*__pdf_state).dvi_page_buf_size.wrapping_add(DVI_PAGE_BUF_CHUNK);
        (*__pdf_state).dvi_page_buffer = renew(
            (*__pdf_state).dvi_page_buffer as *mut ::core::ffi::c_void,
            ((*__pdf_state).dvi_page_buf_size as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize) as uint32_t,
        ) as *mut ::core::ffi::c_uchar;
    }
    let fresh25 = (*__pdf_state).dvi_page_buf_index;
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
    *(*__pdf_state).dvi_page_buffer.offset(fresh25 as isize) = ch as ::core::ffi::c_uchar;
    return ch;
}
unsafe extern "C-unwind" fn get_and_buffer_unsigned_pair(mut handle: rust_input_handle_t) -> ::core::ffi::c_uint {
    let mut pair: ::core::ffi::c_uint = get_and_buffer_unsigned_byte(handle) as ::core::ffi::c_uint;
    pair = pair << 8 as ::core::ffi::c_int | get_and_buffer_unsigned_byte(handle) as ::core::ffi::c_uint;
    return pair;
}
unsafe extern "C-unwind" fn get_and_buffer_bytes(mut handle: rust_input_handle_t, mut count: ::core::ffi::c_uint) {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).dvi_page_buf_index.wrapping_add(count) >= (*__pdf_state).dvi_page_buf_size {
        (*__pdf_state).dvi_page_buf_size = (*__pdf_state).dvi_page_buf_index.wrapping_add(count).wrapping_add(DVI_PAGE_BUF_CHUNK);
        (*__pdf_state).dvi_page_buffer = renew(
            (*__pdf_state).dvi_page_buffer as *mut ::core::ffi::c_void,
            ((*__pdf_state).dvi_page_buf_size as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize) as uint32_t,
        ) as *mut ::core::ffi::c_uchar;
    }
    if ttstub_input_read(
        handle,
        ((*__pdf_state).dvi_page_buffer as *mut ::core::ffi::c_char).offset((*__pdf_state).dvi_page_buf_index as isize),
        count as size_t,
    ) != count as ssize_t
    {
        pdf_abort!(b"File ended prematurely\n\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(count);
}
unsafe extern "C-unwind" fn get_buffered_unsigned_byte() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let fresh12 = (*__pdf_state).dvi_page_buf_index;
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
    return *(*__pdf_state).dvi_page_buffer.offset(fresh12 as isize) as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn get_buffered_unsigned_pair() -> ::core::ffi::c_uint {
    let __pdf_state = unsafe { document_state() };
    let fresh8 = (*__pdf_state).dvi_page_buf_index;
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
    let mut pair: ::core::ffi::c_uint = *(*__pdf_state).dvi_page_buffer.offset(fresh8 as isize) as ::core::ffi::c_uint;
    let fresh9 = (*__pdf_state).dvi_page_buf_index;
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
    pair = pair << 8 as ::core::ffi::c_int | *(*__pdf_state).dvi_page_buffer.offset(fresh9 as isize) as ::core::ffi::c_uint;
    return pair;
}
unsafe extern "C-unwind" fn get_buffered_signed_quad() -> int32_t {
    let __pdf_state = unsafe { document_state() };
    let mut i: ::core::ffi::c_uint = 0;
    let fresh10 = (*__pdf_state).dvi_page_buf_index;
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
    let mut quad: int32_t = *(*__pdf_state).dvi_page_buffer.offset(fresh10 as isize) as int32_t;
    if quad >= 0x80 as int32_t {
        quad = (quad as ::core::ffi::c_int - 0x100 as ::core::ffi::c_int) as int32_t;
    }
    i = 0 as ::core::ffi::c_uint;
    while i < 3 as ::core::ffi::c_uint {
        let fresh11 = (*__pdf_state).dvi_page_buf_index;
        (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
        quad = quad << 8 as ::core::ffi::c_int | *(*__pdf_state).dvi_page_buffer.offset(fresh11 as isize) as int32_t;
        i = i.wrapping_add(1);
    }
    return quad;
}
unsafe extern "C-unwind" fn get_buffered_signed_num(mut num: ::core::ffi::c_uchar) -> int32_t {
    let __pdf_state = unsafe { document_state() };
    let fresh21 = (*__pdf_state).dvi_page_buf_index;
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
    let mut quad: int32_t = *(*__pdf_state).dvi_page_buffer.offset(fresh21 as isize) as int32_t;
    if quad > 0x7f as int32_t {
        quad = (quad as ::core::ffi::c_int - 0x100 as ::core::ffi::c_int) as int32_t;
    }
    let mut current_block_4: u64;
    match num as ::core::ffi::c_int {
        3 => {
            let fresh22 = (*__pdf_state).dvi_page_buf_index;
            (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8 as ::core::ffi::c_int | *(*__pdf_state).dvi_page_buffer.offset(fresh22 as isize) as int32_t;
            current_block_4 = 4139671058212713198;
        }
        2 => {
            current_block_4 = 4139671058212713198;
        }
        1 => {
            current_block_4 = 11897030533752700014;
        }
        _ => {
            current_block_4 = 15619007995458559411;
        }
    }
    match current_block_4 {
        4139671058212713198 => {
            let fresh23 = (*__pdf_state).dvi_page_buf_index;
            (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8 as ::core::ffi::c_int | *(*__pdf_state).dvi_page_buffer.offset(fresh23 as isize) as int32_t;
            current_block_4 = 11897030533752700014;
        }
        _ => {}
    }
    match current_block_4 {
        11897030533752700014 => {
            let fresh24 = (*__pdf_state).dvi_page_buf_index;
            (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8 as ::core::ffi::c_int | *(*__pdf_state).dvi_page_buffer.offset(fresh24 as isize) as int32_t;
        }
        _ => {}
    }
    return quad;
}
unsafe extern "C-unwind" fn get_buffered_unsigned_num(mut num: ::core::ffi::c_uchar) -> int32_t {
    let __pdf_state = unsafe { document_state() };
    let fresh13 = (*__pdf_state).dvi_page_buf_index;
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
    let mut quad: int32_t = *(*__pdf_state).dvi_page_buffer.offset(fresh13 as isize) as int32_t;
    let mut current_block_4: u64;
    match num as ::core::ffi::c_int {
        3 => {
            if quad > 0x7f as int32_t {
                quad = (quad as ::core::ffi::c_int - 0x100 as ::core::ffi::c_int) as int32_t;
            }
            let fresh14 = (*__pdf_state).dvi_page_buf_index;
            (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8 as ::core::ffi::c_int | *(*__pdf_state).dvi_page_buffer.offset(fresh14 as isize) as int32_t;
            current_block_4 = 15152277268355049698;
        }
        2 => {
            current_block_4 = 15152277268355049698;
        }
        1 => {
            current_block_4 = 3155149378587051232;
        }
        _ => {
            current_block_4 = 15619007995458559411;
        }
    }
    match current_block_4 {
        15152277268355049698 => {
            let fresh15 = (*__pdf_state).dvi_page_buf_index;
            (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8 as ::core::ffi::c_int | *(*__pdf_state).dvi_page_buffer.offset(fresh15 as isize) as int32_t;
            current_block_4 = 3155149378587051232;
        }
        _ => {}
    }
    match current_block_4 {
        3155149378587051232 => {
            let fresh16 = (*__pdf_state).dvi_page_buf_index;
            (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(1);
            quad = quad << 8 as ::core::ffi::c_int | *(*__pdf_state).dvi_page_buffer.offset(fresh16 as isize) as int32_t;
        }
        _ => {}
    }
    return quad;
}
#[unsafe(export_name = "__pdf_impl_dvi_npages")]
pub unsafe extern "C-unwind" fn dvi_npages() -> ::core::ffi::c_uint {
    let __pdf_state = unsafe { document_state() };
    return (*__pdf_state).num_pages;
}
static invalid_signature: [::core::ffi::c_char; 53] = crate::backend::c_char_array(*b"Something is wrong. Are you sure this is a DVI file?\0");

unsafe extern "C-unwind" fn check_id_bytes() {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).pre_id_byte != (*__pdf_state).post_id_byte && ((*__pdf_state).pre_id_byte != DVI_ID || (*__pdf_state).post_id_byte != DVIV_ID) {
        pdf_abort!(
            b"Inconsistent DVI id_bytes %d (pre) and %d (post)\0" as *const u8 as *const ::core::ffi::c_char,
            (*__pdf_state).pre_id_byte,
            (*__pdf_state).post_id_byte,
        );
    }
}
unsafe extern "C-unwind" fn need_XeTeX(mut c: ::core::ffi::c_int) {
    if dpx_conf.compat_mode as ::core::ffi::c_uint != dpx_mode_xdv_mode as ::core::ffi::c_int as ::core::ffi::c_uint {
        pdf_abort!(b"DVI opcode %i only valid for XeTeX\0" as *const u8 as *const ::core::ffi::c_char, c,);
    }
}
unsafe extern "C-unwind" fn need_pTeX(mut c: ::core::ffi::c_int) {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).is_ptex == 0 {
        pdf_abort!(b"DVI opcode %i only valid for Ascii pTeX\0" as *const u8 as *const ::core::ffi::c_char, c,);
    }
    (*__pdf_state).has_ptex = 1 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn find_post() -> int32_t {
    let __pdf_state = unsafe { document_state() };
    let mut dvi_size: off_t = 0;
    let mut current: int32_t = 0;
    let mut ch: ::core::ffi::c_int = 0;
    dvi_size = ttstub_input_get_size((*__pdf_state).dvi_handle) as off_t;
    if dvi_size > 0x7fffffff as off_t {
        pdf_abort!(b"DVI file size exceeds 31-bit\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*__pdf_state).dvi_file_size = dvi_size as uint32_t;
    crate::backend::runtime::input_seek((*__pdf_state).dvi_handle, 0 as ssize_t, SEEK_END);
    current = dvi_size as int32_t;
    loop {
        current -= 1;
        crate::backend::runtime::input_seek((*__pdf_state).dvi_handle, current as ssize_t, SEEK_SET);
        ch = ttstub_input_getc((*__pdf_state).dvi_handle);
        if !(ch == PADDING && current > 0 as int32_t) {
            break;
        }
    }
    if (*__pdf_state).dvi_file_size.wrapping_sub(current as uint32_t) < 4 as uint32_t
        || current == 0 as int32_t
        || !(ch == DVI_ID || ch == DVIV_ID || ch == XDV_ID || ch == XDV_ID_OLD)
    {
        pdf_message!(b"DVI ID = %d\n\0" as *const u8 as *const ::core::ffi::c_char, ch);
        pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
    }
    (*__pdf_state).post_id_byte = ch;
    if ch == XDV_ID || ch == XDV_ID_OLD {
        dpx_conf.compat_mode = dpx_mode_xdv_mode;
    }
    (*__pdf_state).is_ptex = (ch == DVIV_ID) as ::core::ffi::c_int;
    current = current - 5 as int32_t;
    crate::backend::runtime::input_seek((*__pdf_state).dvi_handle, current as ssize_t, SEEK_SET);
    ch = ttstub_input_getc((*__pdf_state).dvi_handle);
    if ch != POST_POST {
        pdf_message!(
            b"Found %d where post_post opcode should be\n\0" as *const u8 as *const ::core::ffi::c_char,
            ch,
        );
        pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
    }
    current = tt_get_signed_quad((*__pdf_state).dvi_handle);
    crate::backend::runtime::input_seek((*__pdf_state).dvi_handle, current as ssize_t, SEEK_SET);
    ch = ttstub_input_getc((*__pdf_state).dvi_handle);
    if ch != POST {
        pdf_message!(
            b"Found %d where post_post opcode should be\n\0" as *const u8 as *const ::core::ffi::c_char,
            ch,
        );
        pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
    }
    crate::backend::runtime::input_seek((*__pdf_state).dvi_handle, 0 as ssize_t, SEEK_SET);
    ch = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    if ch != PRE {
        pdf_message!(b"Found %d where PRE was expected\n\0" as *const u8 as *const ::core::ffi::c_char, ch,);
        pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
    }
    ch = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    if !(ch == DVI_ID || ch == XDV_ID || ch == XDV_ID_OLD) {
        pdf_message!(b"DVI ID = %d\n\0" as *const u8 as *const ::core::ffi::c_char, ch);
        pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
    }
    (*__pdf_state).pre_id_byte = ch;
    check_id_bytes();
    return current;
}
unsafe extern "C-unwind" fn get_page_info(mut post_location: int32_t) {
    let __pdf_state = unsafe { document_state() };
    let mut i: ::core::ffi::c_int = 0;
    crate::backend::runtime::input_seek((*__pdf_state).dvi_handle, (post_location + 27 as int32_t) as ssize_t, SEEK_SET);
    (*__pdf_state).num_pages = tt_get_unsigned_pair((*__pdf_state).dvi_handle) as ::core::ffi::c_uint;
    if (*__pdf_state).num_pages == 0 as ::core::ffi::c_uint {
        pdf_abort!(b"Page count is 0!\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
        pdf_message!(
            b"Page count:\t %4d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*__pdf_state).num_pages,
        );
    }
    (*__pdf_state).page_loc =
        new(((*__pdf_state).num_pages as uint32_t as usize).wrapping_mul(::core::mem::size_of::<uint32_t>() as usize) as uint32_t) as *mut uint32_t;
    crate::backend::runtime::input_seek((*__pdf_state).dvi_handle, (post_location + 1 as int32_t) as ssize_t, SEEK_SET);
    *(*__pdf_state)
        .page_loc
        .offset((*__pdf_state).num_pages.wrapping_sub(1 as ::core::ffi::c_uint) as isize) = tt_get_unsigned_quad((*__pdf_state).dvi_handle);
    if (*(*__pdf_state)
        .page_loc
        .offset((*__pdf_state).num_pages.wrapping_sub(1 as ::core::ffi::c_uint) as isize))
    .wrapping_add(41 as uint32_t)
        > (*__pdf_state).dvi_file_size
    {
        pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
    }
    i = (*__pdf_state).num_pages.wrapping_sub(2 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        crate::backend::runtime::input_seek(
            (*__pdf_state).dvi_handle,
            (*(*__pdf_state).page_loc.offset((i + 1 as ::core::ffi::c_int) as isize)).wrapping_add(41 as uint32_t) as ssize_t,
            SEEK_SET,
        );
        *(*__pdf_state).page_loc.offset(i as isize) = tt_get_unsigned_quad((*__pdf_state).dvi_handle);
        if (*(*__pdf_state)
            .page_loc
            .offset((*__pdf_state).num_pages.wrapping_sub(1 as ::core::ffi::c_uint) as isize))
        .wrapping_add(41 as uint32_t)
            > (*__pdf_state).dvi_file_size
        {
            pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
        }
        i -= 1;
    }
}

#[unsafe(export_name = "__pdf_impl_dvi_tell_mag")]
pub unsafe extern "C-unwind" fn dvi_tell_mag() -> ::core::ffi::c_double {
    let __pdf_state = unsafe { document_state() };
    return (*__pdf_state).total_mag;
}
unsafe extern "C-unwind" fn do_scales(mut mag: ::core::ffi::c_double) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).total_mag = (*__pdf_state).dvi_info.mag as ::core::ffi::c_double / 1000.0f64 * mag;
    (*__pdf_state).dvi2pts = (*__pdf_state).dvi_info.unit_num as ::core::ffi::c_double / (*__pdf_state).dvi_info.unit_den as ::core::ffi::c_double;
    (*__pdf_state).dvi2pts *= 72.0f64 / 254000.0f64;
}
unsafe extern "C-unwind" fn get_dvi_info(mut post_location: int32_t) {
    let __pdf_state = unsafe { document_state() };
    crate::backend::runtime::input_seek((*__pdf_state).dvi_handle, (post_location + 5 as int32_t) as ssize_t, SEEK_SET);
    (*__pdf_state).dvi_info.unit_num = tt_get_unsigned_quad((*__pdf_state).dvi_handle);
    (*__pdf_state).dvi_info.unit_den = tt_get_unsigned_quad((*__pdf_state).dvi_handle);
    (*__pdf_state).dvi_info.mag = tt_get_unsigned_quad((*__pdf_state).dvi_handle);
    (*__pdf_state).dvi_info.media_height = tt_get_unsigned_quad((*__pdf_state).dvi_handle);
    (*__pdf_state).dvi_info.media_width = tt_get_unsigned_quad((*__pdf_state).dvi_handle);
    (*__pdf_state).dvi_info.stackdepth = tt_get_unsigned_pair((*__pdf_state).dvi_handle) as ::core::ffi::c_uint;
    if (*__pdf_state).dvi_info.stackdepth > DVI_STACK_DEPTH_MAX {
        pdf_warning!(
            b"DVI need stack depth of %d,\0" as *const u8 as *const ::core::ffi::c_char,
            (*__pdf_state).dvi_info.stackdepth,
        );
        pdf_warning!(
            b"but DVI_STACK_DEPTH_MAX is %d.\0" as *const u8 as *const ::core::ffi::c_char,
            DVI_STACK_DEPTH_MAX,
        );
        pdf_abort!(b"Capacity exceeded.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
        pdf_message!(b"DVI File Info\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdf_message!(
            b"Unit: %u / %u\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*__pdf_state).dvi_info.unit_num,
            (*__pdf_state).dvi_info.unit_den,
        );
        pdf_message!(
            b"Magnification: %u\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*__pdf_state).dvi_info.mag,
        );
        pdf_message!(
            b"Media Height: %u\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*__pdf_state).dvi_info.media_height,
        );
        pdf_message!(
            b"Media Width: %u\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*__pdf_state).dvi_info.media_width,
        );
        pdf_message!(
            b"Stack Depth: %u\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*__pdf_state).dvi_info.stackdepth,
        );
    }
}
#[unsafe(export_name = "__pdf_impl_dvi_comment")]
pub unsafe extern "C-unwind" fn dvi_comment() -> *const ::core::ffi::c_char {
    let __pdf_state = unsafe { document_state() };
    return &raw mut (*__pdf_state).dvi_info.comment as *mut ::core::ffi::c_char;
}
unsafe extern "C-unwind" fn proc_dvilua_font_record(
    mut tex_id: int32_t, mut font_name: *const ::core::ffi::c_char, mut point_size: uint32_t, mut design_size: uint32_t,
) {
    let __pdf_state = unsafe { document_state() };
    let mut file_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut index: uint32_t = 0 as uint32_t;
    let mut embolden: uint32_t = 0 as uint32_t;
    let mut slant: int32_t = 0 as int32_t;
    let mut extend: int32_t = 0x10000 as int32_t;
    if !(*font_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '[' as i32) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font_name[0] == '['\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*__pdf_state).num_def_fonts >= (*__pdf_state).max_def_fonts {
        (*__pdf_state).max_def_fonts = (*__pdf_state).max_def_fonts.wrapping_add(TEX_FONTS_ALLOC_SIZE);
        (*__pdf_state).def_fonts = renew(
            (*__pdf_state).def_fonts as *mut ::core::ffi::c_void,
            ((*__pdf_state).max_def_fonts as uint32_t as usize).wrapping_mul(::core::mem::size_of::<font_def>() as usize) as uint32_t,
        ) as *mut font_def as *mut font_def;
    }
    file_name = new((strlen(font_name).wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    strcpy(file_name, font_name.offset(1 as ::core::ffi::c_int as isize));
    endptr = file_name.offset(strlen(file_name) as isize);
    q = strchr(file_name, ']' as i32);
    if q.is_null() {
        pdf_abort!(b"Syntax error in dvilua fnt_def: no ']' found in font name.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    *q = '\0' as i32 as ::core::ffi::c_char;
    p = q.offset(1 as ::core::ffi::c_int as isize);
    if p < endptr && *p as ::core::ffi::c_int == ':' as i32 {
        p = p.offset(1);
        while *p as ::core::ffi::c_int != '\0' as i32 && p < endptr {
            let mut kvsep: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut delim: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            delim = strchr(p, ';' as i32);
            kvsep = strchr(p, '=' as i32);
            if delim.is_null() {
                delim = endptr;
            }
            if kvsep.is_null() || kvsep >= delim {
                pdf_abort!(
                    b"Syntax error in dvilua fnt_def: not in key=value format: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    font_name,
                );
            }
            *kvsep = '\0' as i32 as ::core::ffi::c_char;
            if strcmp(p, b"index\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                let mut value: uint32_t = strtoul(kvsep.offset(1 as ::core::ffi::c_int as isize), &raw mut q, 10 as ::core::ffi::c_int) as uint32_t;
                if q != delim {
                    pdf_warning!(
                        b"Syntax error in dvilua fnt_def: invalid value specified for \"%s\": %s\0" as *const u8 as *const ::core::ffi::c_char,
                        p,
                        font_name,
                    );
                } else {
                    index = value;
                }
            } else if strcmp(p, b"embolden\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                let mut value_0: int32_t = strtol(kvsep.offset(1 as ::core::ffi::c_int as isize), &raw mut q, 10 as ::core::ffi::c_int) as int32_t;
                if q != delim {
                    pdf_warning!(
                        b"Syntax error in dvilua fnt_def: invalid value specified for \"%s\": %s\0" as *const u8 as *const ::core::ffi::c_char,
                        p,
                        font_name,
                    );
                } else {
                    embolden = value_0 as uint32_t;
                }
            } else if strcmp(p, b"slant\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                let mut value_1: int32_t = strtol(kvsep.offset(1 as ::core::ffi::c_int as isize), &raw mut q, 10 as ::core::ffi::c_int) as int32_t;
                if q != delim {
                    pdf_warning!(
                        b"Syntax error in dvilua fnt_def: invalid value specified for \"%s\": %s\0" as *const u8 as *const ::core::ffi::c_char,
                        p,
                        font_name,
                    );
                } else {
                    slant = value_1;
                }
            } else if strcmp(p, b"extend\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                let mut value_2: int32_t = strtol(kvsep.offset(1 as ::core::ffi::c_int as isize), &raw mut q, 10 as ::core::ffi::c_int) as int32_t;
                if q != delim {
                    pdf_warning!(
                        b"Syntax error in dvilua fnt_def: invalid value specified for \"%s\": %s\0" as *const u8 as *const ::core::ffi::c_char,
                        p,
                        font_name,
                    );
                } else {
                    extend = value_2;
                }
            } else {
                pdf_warning!(
                    b"Ignoring unrecognized/unsupported key \"%s\" in dvilua fnt_def: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    p,
                    font_name,
                );
            }
            p = delim.offset(1 as ::core::ffi::c_int as isize);
        }
    }
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).tex_id = tex_id;
    let ref mut fresh2 = (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).font_name;
    *fresh2 = file_name;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).face_index = index;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).point_size = point_size as spt_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).design_size = design_size as spt_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).used = 0 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).native = 1 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).layout_dir = 0 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).rgba_color = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).rgba_used = 0 as uint8_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).extend = extend as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).slant = slant as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).embolden = embolden as ::core::ffi::c_int;
    (*__pdf_state).num_def_fonts = (*__pdf_state).num_def_fonts.wrapping_add(1);
}
pub const SIG_DVILUA_FNT_DEF: ::core::ffi::c_int =
    ('L' as i32) << 24 as ::core::ffi::c_int | ('u' as i32) << 16 as ::core::ffi::c_int | ('a' as i32) << 8 as ::core::ffi::c_int | 'F' as i32;
unsafe extern "C-unwind" fn read_font_record(mut tex_id: uint32_t) {
    let __pdf_state = unsafe { document_state() };
    let mut dir_length: ::core::ffi::c_int = 0;
    let mut name_length: ::core::ffi::c_int = 0;
    let mut checksum: uint32_t = 0;
    let mut point_size: uint32_t = 0;
    let mut design_size: uint32_t = 0;
    let mut directory: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut font_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*__pdf_state).num_def_fonts >= (*__pdf_state).max_def_fonts {
        (*__pdf_state).max_def_fonts = (*__pdf_state).max_def_fonts.wrapping_add(TEX_FONTS_ALLOC_SIZE);
        (*__pdf_state).def_fonts = renew(
            (*__pdf_state).def_fonts as *mut ::core::ffi::c_void,
            ((*__pdf_state).max_def_fonts as uint32_t as usize).wrapping_mul(::core::mem::size_of::<font_def>() as usize) as uint32_t,
        ) as *mut font_def as *mut font_def;
    }
    checksum = tt_get_unsigned_quad((*__pdf_state).dvi_handle);
    point_size = tt_get_positive_quad(
        (*__pdf_state).dvi_handle,
        b"DVI\0" as *const u8 as *const ::core::ffi::c_char,
        b"point_size\0" as *const u8 as *const ::core::ffi::c_char,
    );
    design_size = tt_get_positive_quad(
        (*__pdf_state).dvi_handle,
        b"DVI\0" as *const u8 as *const ::core::ffi::c_char,
        b"design_size\0" as *const u8 as *const ::core::ffi::c_char,
    );
    dir_length = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    name_length = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    directory = new(
        ((dir_length + 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    if ttstub_input_read((*__pdf_state).dvi_handle, directory, dir_length as size_t) != dir_length as ssize_t {
        pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
    }
    *directory.offset(dir_length as isize) = '\0' as i32 as ::core::ffi::c_char;
    free(directory as *mut ::core::ffi::c_void);
    font_name = new(((name_length + 1 as ::core::ffi::c_int) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    if ttstub_input_read((*__pdf_state).dvi_handle, font_name, name_length as size_t) != name_length as ssize_t {
        pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
    }
    *font_name.offset(name_length as isize) = '\0' as i32 as ::core::ffi::c_char;
    if checksum == SIG_DVILUA_FNT_DEF as uint32_t
        && name_length > 0 as ::core::ffi::c_int
        && *font_name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '[' as i32
    {
        proc_dvilua_font_record(tex_id as int32_t, font_name, point_size, design_size);
        free(font_name as *mut ::core::ffi::c_void);
        return;
    }
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).tex_id = tex_id as int32_t;
    let ref mut fresh1 = (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).font_name;
    *fresh1 = font_name;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).point_size = point_size as spt_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).design_size = design_size as spt_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).used = 0 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).native = 0 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).rgba_color = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).rgba_used = 0 as uint8_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).face_index = 0 as uint32_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).layout_dir = 0 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).extend = 0x10000 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).slant = 0 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).embolden = 0 as ::core::ffi::c_int;
    (*__pdf_state).num_def_fonts = (*__pdf_state).num_def_fonts.wrapping_add(1);
}
unsafe extern "C-unwind" fn read_native_font_record(mut tex_id: uint32_t) {
    let __pdf_state = unsafe { document_state() };
    let mut flags: ::core::ffi::c_uint = 0;
    let mut point_size: uint32_t = 0;
    let mut font_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    let mut index: uint32_t = 0;
    if (*__pdf_state).num_def_fonts >= (*__pdf_state).max_def_fonts {
        (*__pdf_state).max_def_fonts = (*__pdf_state).max_def_fonts.wrapping_add(TEX_FONTS_ALLOC_SIZE);
        (*__pdf_state).def_fonts = renew(
            (*__pdf_state).def_fonts as *mut ::core::ffi::c_void,
            ((*__pdf_state).max_def_fonts as uint32_t as usize).wrapping_mul(::core::mem::size_of::<font_def>() as usize) as uint32_t,
        ) as *mut font_def as *mut font_def;
    }
    point_size = tt_get_positive_quad(
        (*__pdf_state).dvi_handle,
        b"DVI\0" as *const u8 as *const ::core::ffi::c_char,
        b"point_size\0" as *const u8 as *const ::core::ffi::c_char,
    );
    flags = tt_get_unsigned_pair((*__pdf_state).dvi_handle) as ::core::ffi::c_uint;
    len = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    font_name =
        new(((len + 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
            as *mut ::core::ffi::c_char;
    if ttstub_input_read((*__pdf_state).dvi_handle, font_name, len as size_t) != len as ssize_t {
        pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
    }
    *font_name.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    index = tt_get_positive_quad(
        (*__pdf_state).dvi_handle,
        b"DVI\0" as *const u8 as *const ::core::ffi::c_char,
        b"index\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).tex_id = tex_id as int32_t;
    let ref mut fresh0 = (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).font_name;
    *fresh0 = font_name;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).face_index = index;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).point_size = point_size as spt_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).design_size = 655360 as ::core::ffi::c_int as spt_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).used = 0 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).native = 1 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).layout_dir = 0 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).rgba_color = 0xffffffff as ::core::ffi::c_uint as uint32_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).rgba_used = 0 as uint8_t;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).extend = 0x10000 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).slant = 0 as ::core::ffi::c_int;
    (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).embolden = 0 as ::core::ffi::c_int;
    if flags & XDV_FLAG_VERTICAL as ::core::ffi::c_uint != 0 {
        (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).layout_dir = 1 as ::core::ffi::c_int;
    }
    if flags & XDV_FLAG_COLORED as ::core::ffi::c_uint != 0 {
        (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).rgba_color = tt_get_unsigned_quad((*__pdf_state).dvi_handle);
        (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).rgba_used = 1 as uint8_t;
    }
    if flags & XDV_FLAG_EXTEND as ::core::ffi::c_uint != 0 {
        (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).extend =
            tt_get_signed_quad((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    }
    if flags & XDV_FLAG_SLANT as ::core::ffi::c_uint != 0 {
        (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).slant =
            tt_get_signed_quad((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    }
    if flags & XDV_FLAG_EMBOLDEN as ::core::ffi::c_uint != 0 {
        (*(*__pdf_state).def_fonts.offset((*__pdf_state).num_def_fonts as isize)).embolden =
            tt_get_signed_quad((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    }
    (*__pdf_state).num_def_fonts = (*__pdf_state).num_def_fonts.wrapping_add(1);
}
unsafe extern "C-unwind" fn get_dvi_fonts(mut post_location: int32_t) {
    let __pdf_state = unsafe { document_state() };
    let mut code: ::core::ffi::c_int = 0;
    crate::backend::runtime::input_seek((*__pdf_state).dvi_handle, (post_location + 29 as int32_t) as ssize_t, SEEK_SET);
    loop {
        code = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
        if !(code != POST_POST) {
            break;
        }
        match code {
            FNT_DEF1 | FNT_DEF2 | FNT_DEF3 | FNT_DEF4 => {
                read_font_record(tt_get_unsigned_num((*__pdf_state).dvi_handle, (code - FNT_DEF1) as ::core::ffi::c_uchar));
            }
            XDV_NATIVE_FONT_DEF => {
                need_XeTeX(code);
                read_native_font_record(tt_get_signed_quad((*__pdf_state).dvi_handle) as uint32_t);
            }
            _ => {
                pdf_message!(b"Unexpected op code: %3d\n\0" as *const u8 as *const ::core::ffi::c_char, code,);
                pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
            }
        }
    }
    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_uint = 0;
        pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdf_message!(b"DVI file font info\n\0" as *const u8 as *const ::core::ffi::c_char,);
        i = 0 as ::core::ffi::c_uint;
        while i < (*__pdf_state).num_def_fonts {
            pdf_message!(
                b"TeX Font: %10s loaded at ID=%5d, \0" as *const u8 as *const ::core::ffi::c_char,
                (*(*__pdf_state).def_fonts.offset(i as isize)).font_name,
                (*(*__pdf_state).def_fonts.offset(i as isize)).tex_id,
            );
            pdf_message!(
                b"size=%5.2fpt (scaled %4.1f%%)\0" as *const u8 as *const ::core::ffi::c_char,
                (*(*__pdf_state).def_fonts.offset(i as isize)).point_size as ::core::ffi::c_double * (*__pdf_state).dvi2pts,
                100.0f64
                    * ((*(*__pdf_state).def_fonts.offset(i as isize)).point_size as ::core::ffi::c_double
                        / (*(*__pdf_state).def_fonts.offset(i as isize)).design_size as ::core::ffi::c_double),
            );
            pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C-unwind" fn get_comment() {
    let __pdf_state = unsafe { document_state() };
    let mut length: ::core::ffi::c_int = 0;
    crate::backend::runtime::input_seek((*__pdf_state).dvi_handle, 14 as ssize_t, SEEK_SET);
    length = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    if ttstub_input_read(
        (*__pdf_state).dvi_handle,
        &raw mut (*__pdf_state).dvi_info.comment as *mut ::core::ffi::c_char,
        length as size_t,
    ) != length as ssize_t
    {
        pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
    }
    (*__pdf_state).dvi_info.comment[length as usize] = '\0' as i32 as ::core::ffi::c_char;
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(
            b"DVI Comment: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*__pdf_state).dvi_info.comment as *mut ::core::ffi::c_char,
        );
    }
}

unsafe extern "C-unwind" fn clear_state() {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).dvi_state.h = 0 as ::core::ffi::c_int as int32_t;
    (*__pdf_state).dvi_state.v = 0 as ::core::ffi::c_int as int32_t;
    (*__pdf_state).dvi_state.w = 0 as ::core::ffi::c_int as int32_t;
    (*__pdf_state).dvi_state.x = 0 as ::core::ffi::c_int as int32_t;
    (*__pdf_state).dvi_state.y = 0 as ::core::ffi::c_int as int32_t;
    (*__pdf_state).dvi_state.z = 0 as ::core::ffi::c_int as int32_t;
    (*__pdf_state).dvi_state.d = 0 as ::core::ffi::c_uint;
    pdf_dev_set_dirmode(0 as ::core::ffi::c_int);
    (*__pdf_state).dvi_stack_depth = 0 as ::core::ffi::c_int;
    (*__pdf_state).current_font = -(1 as ::core::ffi::c_int);
}

unsafe extern "C-unwind" fn dvi_mark_depth() {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).link_annot != 0
        && (*__pdf_state).marked_depth == (*__pdf_state).tagged_depth
        && (*__pdf_state).dvi_stack_depth == (*__pdf_state).tagged_depth - 1 as ::core::ffi::c_int
    {
        pdf_doc_break_annot();
    }
    (*__pdf_state).marked_depth = (*__pdf_state).dvi_stack_depth;
}
#[unsafe(export_name = "__pdf_impl_dvi_tag_depth")]
pub unsafe extern "C-unwind" fn dvi_tag_depth() {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).tagged_depth = (*__pdf_state).marked_depth;
    dvi_compute_boxes(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_dvi_untag_depth")]
pub unsafe extern "C-unwind" fn dvi_untag_depth() {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).tagged_depth = -(1 as ::core::ffi::c_int);
    dvi_compute_boxes(0 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_dvi_compute_boxes")]
pub unsafe extern "C-unwind" fn dvi_compute_boxes(mut flag: ::core::ffi::c_int) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).compute_boxes = flag;
}
#[unsafe(export_name = "__pdf_impl_dvi_link_annot")]
pub unsafe extern "C-unwind" fn dvi_link_annot(mut flag: ::core::ffi::c_int) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).link_annot = flag;
}
#[unsafe(export_name = "__pdf_impl_dvi_is_tracking_boxes")]
pub unsafe extern "C-unwind" fn dvi_is_tracking_boxes() -> bool {
    let __pdf_state = unsafe { document_state() };
    return (*__pdf_state).compute_boxes != 0 && (*__pdf_state).link_annot != 0 && (*__pdf_state).marked_depth >= (*__pdf_state).tagged_depth;
}
#[unsafe(export_name = "__pdf_impl_dvi_set_linkmode")]
pub unsafe extern "C-unwind" fn dvi_set_linkmode(mut mode: ::core::ffi::c_int) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).catch_phantom = if mode != 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(export_name = "__pdf_impl_dvi_set_phantom_height")]
pub unsafe extern "C-unwind" fn dvi_set_phantom_height(mut height: ::core::ffi::c_double, mut depth: ::core::ffi::c_double) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).phantom_height = height;
    (*__pdf_state).phantom_depth = depth;
    (*__pdf_state).catch_phantom = 2 as ::core::ffi::c_int;
}

#[unsafe(export_name = "__pdf_impl_dvi_set_compensation")]
pub unsafe extern "C-unwind" fn dvi_set_compensation(mut x: ::core::ffi::c_double, mut y: ::core::ffi::c_double) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).compensation.x = round(x / (*__pdf_state).dvi2pts) as spt_t;
    (*__pdf_state).compensation.y = round(y / (*__pdf_state).dvi2pts) as spt_t;
}
unsafe extern "C-unwind" fn set_string(
    mut xpos: spt_t, mut ypos: spt_t, mut instr_ptr: *const ::core::ffi::c_void, mut instr_len: ::core::ffi::c_int, mut width: spt_t,
    mut font_id: ::core::ffi::c_int,
) {
    let __pdf_state = unsafe { document_state() };
    xpos -= (*__pdf_state).compensation.x;
    ypos -= (*__pdf_state).compensation.y;
    pdf_dev_set_string(xpos, ypos, instr_ptr, instr_len as size_t, width, font_id);
}
unsafe extern "C-unwind" fn set_rule(mut xpos: spt_t, mut ypos: spt_t, mut width: spt_t, mut height: spt_t) {
    let __pdf_state = unsafe { document_state() };
    xpos -= (*__pdf_state).compensation.x;
    ypos -= (*__pdf_state).compensation.y;
    pdf_dev_set_rule(xpos, ypos, width, height);
}
unsafe extern "C-unwind" fn calc_rect(mut r: *mut pdf_rect, mut xpos: spt_t, mut ypos: spt_t, mut width: spt_t, mut height: spt_t, mut depth: spt_t) {
    let __pdf_state = unsafe { document_state() };
    xpos -= (*__pdf_state).compensation.x;
    ypos -= (*__pdf_state).compensation.y;
    pdf_dev_set_rect(r, xpos, ypos, width, height, depth);
}
#[unsafe(export_name = "__pdf_impl_dvi_do_special")]
pub unsafe extern "C-unwind" fn dvi_do_special(mut buffer: *const ::core::ffi::c_void, mut size: int32_t) {
    let __pdf_state = unsafe { document_state() };
    let mut x_user: ::core::ffi::c_double = 0.;
    let mut y_user: ::core::ffi::c_double = 0.;
    let mut mag: ::core::ffi::c_double = 0.;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut is_drawable: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rect: pdf_rect = pdf_rect {
        llx: 0.0f64,
        lly: 0.0f64,
        urx: 0.0f64,
        ury: 0.0f64,
    };
    graphics_mode();
    p = buffer as *const ::core::ffi::c_char;
    x_user = (*__pdf_state).dvi_state.h as ::core::ffi::c_double * (*__pdf_state).dvi2pts;
    y_user = -(*__pdf_state).dvi_state.v as ::core::ffi::c_double * (*__pdf_state).dvi2pts;
    mag = dvi_tell_mag();
    if spc_exec_special(p, size, x_user, y_user, mag, &raw mut is_drawable, &raw mut rect) < 0 as ::core::ffi::c_int {
        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
            dump(p, p.offset(size as isize));
        }
        return;
    }
    if dvi_is_tracking_boxes() as ::core::ffi::c_int != 0 && is_drawable != 0 {
        pdf_doc_expand_box(&raw mut rect);
    }
}
#[unsafe(export_name = "__pdf_impl_dvi_unit_size")]
pub unsafe extern "C-unwind" fn dvi_unit_size() -> ::core::ffi::c_double {
    let __pdf_state = unsafe { document_state() };
    return (*__pdf_state).dvi2pts;
}
#[unsafe(export_name = "__pdf_impl_dvi_locate_font")]
pub unsafe extern "C-unwind" fn dvi_locate_font(mut tfm_name: *const ::core::ffi::c_char, mut ptsize: spt_t) -> ::core::ffi::c_uint {
    let __pdf_state = unsafe { document_state() };
    let mut cur_id: ::core::ffi::c_uint = 0;
    let mut name: *const ::core::ffi::c_char = tfm_name;
    let mut subfont_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut font_id: ::core::ffi::c_int = 0;
    let mut mrec: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(
            b"<%s@%.2fpt\0" as *const u8 as *const ::core::ffi::c_char,
            tfm_name,
            ptsize as ::core::ffi::c_double * (*__pdf_state).dvi2pts,
        );
    }
    need_more_fonts(1 as ::core::ffi::c_uint);
    let fresh17 = (*__pdf_state).num_loaded_fonts;
    (*__pdf_state).num_loaded_fonts = (*__pdf_state).num_loaded_fonts.wrapping_add(1);
    cur_id = fresh17;
    mrec = pdf_lookup_fontmap_record(tfm_name);
    if !mrec.is_null() && !(*mrec).charmap.sfd_name.is_null() && !(*mrec).charmap.subfont_id.is_null() {
        subfont_id = sfd_load_record((*mrec).charmap.sfd_name, (*mrec).charmap.subfont_id);
    }
    memset(
        (*__pdf_state).loaded_fonts.offset(cur_id as isize) as *mut loaded_font as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<loaded_font>() as size_t,
    );
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).tfm_id = tfm_open(tfm_name, 1 as ::core::ffi::c_int);
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).subfont_id = subfont_id;
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).size = ptsize;
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).source = VF;
    if mrec.is_null() {
        font_id = vf_locate_font(tfm_name, ptsize);
        if font_id >= 0 as ::core::ffi::c_int {
            (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).type_0 = VIRTUAL;
            (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).font_id = font_id;
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(b"(VF)>\0" as *const u8 as *const ::core::ffi::c_char);
            }
            return cur_id;
        }
    } else if subfont_id >= 0 as ::core::ffi::c_int && !(*mrec).map_name.is_null() {
        let mut mrec1: *mut fontmap_rec = pdf_lookup_fontmap_record((*mrec).map_name);
        if !mrec1.is_null() && (*mrec1).enc_name.is_null() {
            font_id = vf_locate_font((*mrec1).font_name, ptsize);
            if font_id < 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Could not locate Omega Virtual Font \"%s\" for \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                    (*mrec1).font_name,
                    tfm_name,
                );
            } else {
                (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).type_0 = VIRTUAL;
                (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).font_id = font_id;
                if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                    pdf_message!(b"(OVF)>\0" as *const u8 as *const ::core::ffi::c_char);
                }
                return cur_id;
            }
        }
    }
    if !mrec.is_null() && !(*mrec).map_name.is_null() {
        name = (*mrec).map_name;
    } else {
        name = tfm_name;
    }
    font_id = pdf_dev_locate_font(name, ptsize);
    if font_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Could not locate a virtual/physical font for TFM \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
            tfm_name,
        );
        if !mrec.is_null() && !(*mrec).map_name.is_null() {
            let mut mrec1_0: *mut fontmap_rec = pdf_lookup_fontmap_record((*mrec).map_name);
            pdf_warning!(
                b">> This font is mapped to an intermediate 16-bit font \"%s\" with SFD charmap=<%s,%s>,\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*mrec).map_name,
                (*mrec).charmap.sfd_name,
                (*mrec).charmap.subfont_id,
            );
            if mrec1_0.is_null() {
                pdf_warning!(
                    b">> but I couldn't find font mapping for \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                    (*mrec).map_name,
                );
            } else {
                pdf_warning!(
                    b">> and then mapped to a physical font \"%s\" by fontmap.\0" as *const u8 as *const ::core::ffi::c_char,
                    (*mrec1_0).font_name,
                );
                pdf_warning!(
                    b">> Please check if kpathsea library can find this font: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*mrec1_0).font_name,
                );
            }
        } else if !mrec.is_null() && (*mrec).map_name.is_null() {
            let mut finaldot: *mut ::core::ffi::c_char = strrchr((*mrec).font_name, '.' as i32);
            if !finaldot.is_null() && strcasecmp(finaldot, b".pfa\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"This font is mapped to a physical font \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                    (*mrec).font_name,
                );
                pdf_abort!(
                    b"Sorry, pfa format not supported; please convert the font to pfb, e.g., with t1binary.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
                pdf_warning!(
                    b">> This font is mapped to a physical font \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                    (*mrec).font_name,
                );
                pdf_warning!(
                    b">> Please check if kpathsea library can find this font: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*mrec).font_name,
                );
            }
        } else {
            pdf_warning!(b">> There are no valid font mapping entry for this font.\0" as *const u8 as *const ::core::ffi::c_char,);
            pdf_warning!(
                b">> Font file name \"%s\" was assumed but failed to locate that font.\0" as *const u8 as *const ::core::ffi::c_char,
                tfm_name,
            );
        }
        pdf_abort!(b"Cannot proceed without .vf or \"physical\" font for PDF output...\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    memset(
        &raw mut (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).padbytes as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        4 as size_t,
    );
    if !mrec.is_null() {
        if (*mrec).opt.mapc >= 0 as ::core::ffi::c_int {
            (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).padbytes[2 as ::core::ffi::c_int as usize] =
                ((*mrec).opt.mapc >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
        }
        if !(*mrec).enc_name.is_null() && strcmp((*mrec).enc_name, b"unicode\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).is_unicode = 1 as ::core::ffi::c_int;
            if (*mrec).opt.mapc >= 0 as ::core::ffi::c_int {
                (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).padbytes[0 as ::core::ffi::c_int as usize] =
                    ((*mrec).opt.mapc >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
                (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).padbytes[1 as ::core::ffi::c_int as usize] =
                    ((*mrec).opt.mapc >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
            }
        }
    }
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).minbytes = pdf_dev_font_minbytes(font_id);
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).type_0 = PHYSICAL;
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).font_id = font_id;
    if (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).minbytes > 4 as ::core::ffi::c_int {
        pdf_warning!(b"Input encoding requries more than 4 bytes per char... (unsupported)\0" as *const u8 as *const ::core::ffi::c_char,);
        (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).minbytes = 4 as ::core::ffi::c_int;
    }
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(b">\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return cur_id;
}
unsafe extern "C-unwind" fn dvi_locate_native_font(
    mut filename: *const ::core::ffi::c_char, mut index: uint32_t, mut ptsize: spt_t, mut layout_dir: ::core::ffi::c_int,
    mut extend: ::core::ffi::c_int, mut slant: ::core::ffi::c_int, mut embolden: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut cur_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut mrec: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    let mut fontmap_key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut handle: rust_input_handle_t = 0;
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut offset: SFNT_ULONG = 0 as SFNT_ULONG;
    let mut is_dfont: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_type1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(
            b"<%s@%.2fpt\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
            ptsize as ::core::ffi::c_double * (*__pdf_state).dvi2pts,
        );
    }
    handle = dpx_open_dfont_file(filename);
    if handle != INVALID_HANDLE {
        is_dfont = 1 as ::core::ffi::c_int;
    } else {
        handle = dpx_open_type1_file(filename);
        if handle != INVALID_HANDLE {
            is_type1 = 1 as ::core::ffi::c_int;
        } else {
            handle = dpx_open_opentype_file(filename);
            if handle == INVALID_HANDLE && {
                handle = dpx_open_truetype_file(filename);
                handle == INVALID_HANDLE
            } {
                pdf_abort!(
                    b"Cannot proceed without the font: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    filename,
                );
            }
        }
    }
    need_more_fonts(1 as ::core::ffi::c_uint);
    let fresh18 = (*__pdf_state).num_loaded_fonts;
    (*__pdf_state).num_loaded_fonts = (*__pdf_state).num_loaded_fonts.wrapping_add(1);
    cur_id = fresh18 as ::core::ffi::c_int;
    fontmap_key = crate::backend::runtime::allocate(strlen(filename).wrapping_add(40 as size_t)) as *mut ::core::ffi::c_char;
    sprintf(
        fontmap_key,
        b"%s/%u/%c/%d/%d/%d\0" as *const u8 as *const ::core::ffi::c_char,
        filename,
        index,
        if layout_dir == 0 as ::core::ffi::c_int { 'H' as i32 } else { 'V' as i32 },
        extend,
        slant,
        embolden,
    );
    mrec = pdf_lookup_fontmap_record(fontmap_key);
    if mrec.is_null() {
        mrec = pdf_insert_native_fontmap_record(filename, index, layout_dir, extend, slant, embolden);
        if mrec.is_null() {
            pdf_abort!(
                b"Failed to insert font record for font: %s\0" as *const u8 as *const ::core::ffi::c_char,
                filename,
            );
        }
    }
    memset(
        (*__pdf_state).loaded_fonts.offset(cur_id as isize) as *mut loaded_font as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<loaded_font>() as size_t,
    );
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).font_id = pdf_dev_locate_font(fontmap_key, ptsize);
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).size = ptsize;
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).type_0 = NATIVE;
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).minbytes =
        pdf_dev_font_minbytes((*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).font_id);
    if (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).minbytes > 4 as ::core::ffi::c_int {
        pdf_warning!(b"Input encoding requries more than 4 bytes per char... (unsupprted)\0" as *const u8 as *const ::core::ffi::c_char,);
        (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).minbytes = 4 as ::core::ffi::c_int;
    }
    free(fontmap_key as *mut ::core::ffi::c_void);
    if is_type1 != 0 {
        let mut cffont: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
        let mut enc_vec: [*mut ::core::ffi::c_char; 256] = [::core::ptr::null_mut::<::core::ffi::c_char>(); 256];
        let mut num_glyphs: uint16_t = 0;
        let mut i: ::core::ffi::c_int = 0;
        pdf_warning!(b"skipping PFB sanity check -- needs Tectonic I/O update\0" as *const u8 as *const ::core::ffi::c_char,);
        memset(
            &raw mut enc_vec as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (256 as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
        );
        cffont = t1_load_font(&raw mut enc_vec as *mut *mut ::core::ffi::c_char, 0 as ::core::ffi::c_int, handle);
        if cffont.is_null() {
            pdf_abort!(
                b"Failed to read Type 1 font \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                filename,
            );
        }
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            if !enc_vec[i as usize].is_null() {
                free(enc_vec[i as usize] as *mut ::core::ffi::c_void);
            }
            i += 1;
        }
        (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).shift_gid = (*cffont).is_notdef_notzero;
        num_glyphs = (*cffont).num_glyphs as uint16_t;
        (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).num_glyphs = num_glyphs;
        let ref mut fresh19 = (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).gm;
        *fresh19 = new(((num_glyphs as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<gm>() as usize) as uint32_t) as *mut gm;
        i = 0 as ::core::ffi::c_int;
        while i < num_glyphs as ::core::ffi::c_int {
            let mut advance: ::core::ffi::c_double = 0.;
            let mut ascent: ::core::ffi::c_double = 0.;
            let mut descent: ::core::ffi::c_double = 0.;
            let mut cstrings: *mut cff_index = (*cffont).cstrings;
            let mut gm: t1_ginfo = t1_ginfo {
                use_seac: 0,
                wx: 0.,
                wy: 0.,
                bbox: C2RustUnnamed_4 {
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
            let mut gid: uint16_t = 0;
            gid = (if (*cffont).is_notdef_notzero != 0 {
                i + 1 as ::core::ffi::c_int
            } else {
                i
            }) as uint16_t;
            if gid as ::core::ffi::c_int == num_glyphs as ::core::ffi::c_int {
                break;
            }
            t1char_get_metrics(
                (*cstrings)
                    .data
                    .offset(*(*cstrings).offset.offset(gid as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)),
                (*(*cstrings).offset.offset((gid as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                    .wrapping_sub(*(*cstrings).offset.offset(gid as isize)) as ::core::ffi::c_int,
                *(*cffont).subrs.offset(0 as ::core::ffi::c_int as isize),
                &raw mut gm,
            );
            advance = if layout_dir == 0 as ::core::ffi::c_int { gm.wx } else { gm.wy };
            ascent = gm.bbox.ury;
            descent = gm.bbox.lly;
            (*(*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).gm.offset(gid as isize)).advance =
                (ptsize as ::core::ffi::c_double * (advance / 1000.0f64 * (*mrec).opt.extend)) as spt_t;
            (*(*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).gm.offset(gid as isize)).ascent =
                (ptsize as ::core::ffi::c_double * (ascent / 1000.0f64)) as spt_t;
            (*(*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).gm.offset(gid as isize)).descent =
                (ptsize as ::core::ffi::c_double * (descent / 1000.0f64)) as spt_t;
            i += 1;
        }
        cff_close(cffont);
        crate::backend::runtime::input_close(handle);
    } else {
        let mut head: *mut tt_head_table = ::core::ptr::null_mut::<tt_head_table>();
        let mut maxp: *mut tt_maxp_table = ::core::ptr::null_mut::<tt_maxp_table>();
        let mut hhea: *mut tt_hhea_table = ::core::ptr::null_mut::<tt_hhea_table>();
        let mut gm_0: *mut tt_longMetrics = ::core::ptr::null_mut::<tt_longMetrics>();
        let mut ascent_0: spt_t = 0;
        let mut descent_0: spt_t = 0;
        let mut num_glyphs_0: uint16_t = 0;
        let mut i_0: ::core::ffi::c_int = 0;
        if is_dfont != 0 {
            sfont = dfont_open(handle, index as ::core::ffi::c_int);
        } else {
            sfont = sfnt_open(handle);
        }
        if (*sfont).type_0 == SFNT_TYPE_TTC {
            offset = ttc_read_offset(sfont, index as SFNT_ULONG);
        } else if (*sfont).type_0 == SFNT_TYPE_DFONT {
            offset = (*sfont).offset;
        }
        sfnt_read_table_directory(sfont, offset);
        head = tt_read_head_table(sfont);
        maxp = tt_read_maxp_table(sfont);
        hhea = tt_read_hhea_table(sfont);
        ascent_0 =
            (ptsize as ::core::ffi::c_double * ((*hhea).ascent as ::core::ffi::c_double / (*head).unitsPerEm as ::core::ffi::c_double)) as spt_t;
        descent_0 =
            (ptsize as ::core::ffi::c_double * ((*hhea).descent as ::core::ffi::c_double / (*head).unitsPerEm as ::core::ffi::c_double)) as spt_t;
        num_glyphs_0 = (*maxp).numGlyphs as uint16_t;
        (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).num_glyphs = num_glyphs_0;
        if layout_dir == 1 as ::core::ffi::c_int && sfnt_find_table_pos(sfont, b"vmtx\0" as *const u8 as *const ::core::ffi::c_char) > 0 as SFNT_ULONG
        {
            let mut vhea: *mut tt_vhea_table = tt_read_vhea_table(sfont);
            sfnt_locate_table(sfont, b"vmtx\0" as *const u8 as *const ::core::ffi::c_char);
            gm_0 = tt_read_longMetrics(sfont, (*maxp).numGlyphs, (*vhea).numOfLongVerMetrics, (*vhea).numOfExSideBearings);
            free(vhea as *mut ::core::ffi::c_void);
        } else {
            sfnt_locate_table(sfont, b"hmtx\0" as *const u8 as *const ::core::ffi::c_char);
            gm_0 = tt_read_longMetrics(sfont, (*maxp).numGlyphs, (*hhea).numOfLongHorMetrics, (*hhea).numOfExSideBearings);
        }
        if gm_0.is_null() {
            pdf_abort!(b"Failed to read TrueType/OpenType glyph metrics table.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        let ref mut fresh20 = (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).gm;
        *fresh20 = new((num_glyphs_0 as uint32_t as usize).wrapping_mul(::core::mem::size_of::<gm>() as usize) as uint32_t) as *mut gm;
        i_0 = 0 as ::core::ffi::c_int;
        while i_0 < num_glyphs_0 as ::core::ffi::c_int {
            (*(*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).gm.offset(i_0 as isize)).advance = (ptsize as ::core::ffi::c_double
                * ((*gm_0.offset(i_0 as isize)).advance as ::core::ffi::c_double / (*head).unitsPerEm as ::core::ffi::c_double)
                * (*mrec).opt.extend)
                as spt_t;
            (*(*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).gm.offset(i_0 as isize)).ascent = ascent_0;
            (*(*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).gm.offset(i_0 as isize)).descent = descent_0;
            i_0 += 1;
        }
        free(gm_0 as *mut ::core::ffi::c_void);
        free(hhea as *mut ::core::ffi::c_void);
        free(maxp as *mut ::core::ffi::c_void);
        free(head as *mut ::core::ffi::c_void);
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
    }
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).layout_dir = layout_dir;
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).extend = (*mrec).opt.extend as ::core::ffi::c_float;
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).slant = (*mrec).opt.slant as ::core::ffi::c_float;
    (*(*__pdf_state).loaded_fonts.offset(cur_id as isize)).embolden = (*mrec).opt.bold as ::core::ffi::c_float;
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(b">\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return cur_id;
}
#[unsafe(export_name = "__pdf_impl_dvi_dev_xpos")]
pub unsafe extern "C-unwind" fn dvi_dev_xpos() -> ::core::ffi::c_double {
    let __pdf_state = unsafe { document_state() };
    return (*__pdf_state).dvi_state.h as ::core::ffi::c_double * (*__pdf_state).dvi2pts;
}
#[unsafe(export_name = "__pdf_impl_dvi_dev_ypos")]
pub unsafe extern "C-unwind" fn dvi_dev_ypos() -> ::core::ffi::c_double {
    let __pdf_state = unsafe { document_state() };
    return -((*__pdf_state).dvi_state.v as ::core::ffi::c_double * (*__pdf_state).dvi2pts);
}
unsafe extern "C-unwind" fn do_moveto(mut x: int32_t, mut y: int32_t) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).dvi_state.h = x;
    (*__pdf_state).dvi_state.v = y;
}
#[unsafe(export_name = "__pdf_impl_dvi_right")]
pub unsafe extern "C-unwind" fn dvi_right(mut x: int32_t) {
    let __pdf_state = unsafe { document_state() };
    let mut save_h: spt_t = 0;
    let mut save_v: spt_t = 0;
    if (*__pdf_state).lr_mode >= SKIMMING {
        (*__pdf_state).lr_width = (*__pdf_state).lr_width.wrapping_add(x as uint32_t);
        return;
    }
    if (*__pdf_state).lr_mode == RTYPESETTING {
        x = -x;
    }
    save_h = (*__pdf_state).dvi_state.h as spt_t;
    save_v = (*__pdf_state).dvi_state.v as spt_t;
    match (*__pdf_state).dvi_state.d {
        0 => {
            (*__pdf_state).dvi_state.h += x;
        }
        1 => {
            (*__pdf_state).dvi_state.v += x;
        }
        3 => {
            (*__pdf_state).dvi_state.v -= x;
        }
        _ => {}
    }
    if dvi_is_tracking_boxes() as ::core::ffi::c_int != 0 && (*__pdf_state).catch_phantom > 0 as ::core::ffi::c_int {
        let mut rect: pdf_rect = pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        };
        let mut width: spt_t = 0;
        let mut height: spt_t = 0;
        let mut depth: spt_t = 0;
        if (*__pdf_state).catch_phantom == 1 as ::core::ffi::c_int {
            if (*__pdf_state).current_font >= 0 as ::core::ffi::c_int
                && ((*__pdf_state).current_font as ::core::ffi::c_uint) < (*__pdf_state).num_loaded_fonts
            {
                height = (*(*__pdf_state).loaded_fonts.offset((*__pdf_state).current_font as isize)).size;
            } else {
                if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                    pdf_warning!(
                        b"Don't know how to calculate the box height since current font is not set...\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                height = 0.0f64 as spt_t;
            }
            depth = 0.0f64 as spt_t;
        } else {
            height = ((*__pdf_state).phantom_height / (*__pdf_state).dvi2pts) as spt_t;
            depth = ((*__pdf_state).phantom_depth / (*__pdf_state).dvi2pts) as spt_t;
        }
        match (*__pdf_state).dvi_state.d {
            0 => {
                width = ((*__pdf_state).dvi_state.h - save_h as int32_t) as spt_t;
            }
            1 | 2 => {
                width = ((*__pdf_state).dvi_state.v - save_v as int32_t) as spt_t;
            }
            _ => {
                width = ((*__pdf_state).dvi_state.h - save_h as int32_t) as spt_t;
            }
        }
        calc_rect(&raw mut rect, save_h, -save_v, width, height, depth);
        pdf_doc_expand_box(&raw mut rect);
    }
}
#[unsafe(export_name = "__pdf_impl_dvi_down")]
pub unsafe extern "C-unwind" fn dvi_down(mut y: int32_t) {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).lr_mode < SKIMMING {
        match (*__pdf_state).dvi_state.d {
            0 => {
                (*__pdf_state).dvi_state.v += y;
            }
            1 => {
                (*__pdf_state).dvi_state.h -= y;
            }
            3 => {
                (*__pdf_state).dvi_state.h += y;
            }
            _ => {}
        }
    }
}
#[unsafe(export_name = "__pdf_impl_dvi_set")]
pub unsafe extern "C-unwind" fn dvi_set(mut ch: int32_t) {
    let __pdf_state = unsafe { document_state() };
    let mut font: *mut loaded_font = ::core::ptr::null_mut::<loaded_font>();
    let mut width: spt_t = 0;
    let mut height: spt_t = 0;
    let mut depth: spt_t = 0;
    let mut wbuf: [::core::ffi::c_uchar; 4] = [0; 4];
    let mut n: ::core::ffi::c_int = 0;
    let mut cbytes: ::core::ffi::c_int = 0;
    if (*__pdf_state).current_font < 0 as ::core::ffi::c_int {
        pdf_abort!(b"No font selected!\0" as *const u8 as *const ::core::ffi::c_char);
    }
    font = (*__pdf_state).loaded_fonts.offset((*__pdf_state).current_font as isize) as *mut loaded_font as *mut loaded_font;
    if (*font).type_0 == NATIVE {
        if ch >= 0 as int32_t && ch < (*font).num_glyphs as int32_t {
            width = (*(*font).gm.offset(ch as isize)).advance;
        } else {
            pdf_warning!(
                b"Invalid char for dvilua font (ignored): %04x\0" as *const u8 as *const ::core::ffi::c_char,
                ch,
            );
            return;
        }
    } else {
        width = tfm_get_fw_width((*font).tfm_id, ch) as spt_t;
        width = sqxfw((*font).size as int32_t, width as fixword) as spt_t;
    }
    if (*__pdf_state).lr_mode >= SKIMMING {
        (*__pdf_state).lr_width = (*__pdf_state).lr_width.wrapping_add(width as uint32_t);
        return;
    }
    if (*__pdf_state).lr_mode == RTYPESETTING {
        dvi_right(width as int32_t);
    }
    memcpy(
        &raw mut wbuf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut (*font).padbytes as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as size_t,
    );
    match (*font).type_0 {
        PHYSICAL => {
            if ch > 65535 as int32_t {
                if (*font).is_unicode == 0 && tfm_is_jfm((*font).tfm_id) != 0 {
                    wbuf[0 as ::core::ffi::c_int as usize] =
                        (0xd800 as int32_t + (ch - 0x10000 as int32_t >> 10 as ::core::ffi::c_int & 0x3ff as int32_t) >> 8 as ::core::ffi::c_int
                            & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[1 as ::core::ffi::c_int as usize] = (0xd800 as int32_t
                        + (ch - 0x10000 as int32_t >> 10 as ::core::ffi::c_int & 0x3ff as int32_t)
                        & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[2 as ::core::ffi::c_int as usize] =
                        (0xdc00 as int32_t + (ch & 0x3ff as int32_t) >> 8 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[3 as ::core::ffi::c_int as usize] = (0xdc00 as int32_t + (ch & 0x3ff as int32_t) & 0xff as int32_t) as ::core::ffi::c_uchar;
                } else {
                    wbuf[0 as ::core::ffi::c_int as usize] = (ch >> 24 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[1 as ::core::ffi::c_int as usize] = (ch >> 16 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[2 as ::core::ffi::c_int as usize] = (ch >> 8 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[3 as ::core::ffi::c_int as usize] = (ch & 0xff as int32_t) as ::core::ffi::c_uchar;
                }
                n = 4 as ::core::ffi::c_int;
            } else if ch > 255 as int32_t {
                wbuf[2 as ::core::ffi::c_int as usize] = (ch >> 8 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                wbuf[2 as ::core::ffi::c_int as usize] = (ch & 0xff as int32_t) as ::core::ffi::c_uchar;
                n = 2 as ::core::ffi::c_int;
            } else if (*font).subfont_id >= 0 as ::core::ffi::c_int {
                let mut uch: uint16_t = lookup_sfd_record((*font).subfont_id, ch as ::core::ffi::c_uchar) as uint16_t;
                wbuf[2 as ::core::ffi::c_int as usize] =
                    (uch as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                wbuf[3 as ::core::ffi::c_int as usize] = (uch as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                n = 2 as ::core::ffi::c_int;
            } else {
                wbuf[3 as ::core::ffi::c_int as usize] = ch as ::core::ffi::c_uchar;
                n = 1 as ::core::ffi::c_int;
            }
            cbytes = if (*font).minbytes > n { (*font).minbytes } else { n };
            set_string(
                (*__pdf_state).dvi_state.h as spt_t,
                -((*__pdf_state).dvi_state.v as spt_t),
                (&raw mut wbuf as *mut ::core::ffi::c_uchar)
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(-(cbytes as isize)) as *const ::core::ffi::c_void,
                cbytes,
                width,
                (*font).font_id,
            );
            if dvi_is_tracking_boxes() {
                let mut rect: pdf_rect = pdf_rect {
                    llx: 0.,
                    lly: 0.,
                    urx: 0.,
                    ury: 0.,
                };
                height = tfm_get_fw_height((*font).tfm_id, ch) as spt_t;
                depth = tfm_get_fw_depth((*font).tfm_id, ch) as spt_t;
                height = sqxfw((*font).size as int32_t, height as fixword) as spt_t;
                depth = sqxfw((*font).size as int32_t, depth as fixword) as spt_t;
                calc_rect(
                    &raw mut rect,
                    (*__pdf_state).dvi_state.h as spt_t,
                    -((*__pdf_state).dvi_state.v as spt_t),
                    width,
                    height,
                    depth,
                );
                pdf_doc_expand_box(&raw mut rect);
            }
        }
        NATIVE => {
            wbuf[0 as ::core::ffi::c_int as usize] = (ch >> 8 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
            wbuf[1 as ::core::ffi::c_int as usize] = (ch & 0xff as int32_t) as ::core::ffi::c_uchar;
            set_string(
                (*__pdf_state).dvi_state.h as spt_t,
                -((*__pdf_state).dvi_state.v as spt_t),
                &raw mut wbuf as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                2 as ::core::ffi::c_int,
                width,
                (*font).font_id,
            );
            if dvi_is_tracking_boxes() {
                let mut rect_0: pdf_rect = pdf_rect {
                    llx: 0.,
                    lly: 0.,
                    urx: 0.,
                    ury: 0.,
                };
                height = (*(*font).gm.offset(ch as isize)).ascent;
                depth = (*(*font).gm.offset(ch as isize)).descent;
                calc_rect(
                    &raw mut rect_0,
                    (*__pdf_state).dvi_state.h as spt_t,
                    -((*__pdf_state).dvi_state.v as spt_t),
                    width,
                    height,
                    depth,
                );
                pdf_doc_expand_box(&raw mut rect_0);
            }
        }
        VIRTUAL => {
            vf_set_char(ch, (*font).font_id);
        }
        _ => {}
    }
    if (*__pdf_state).lr_mode == LTYPESETTING {
        dvi_right(width as int32_t);
    }
}
#[unsafe(export_name = "__pdf_impl_dvi_put")]
pub unsafe extern "C-unwind" fn dvi_put(mut ch: int32_t) {
    let __pdf_state = unsafe { document_state() };
    let mut font: *mut loaded_font = ::core::ptr::null_mut::<loaded_font>();
    let mut width: spt_t = 0;
    let mut height: spt_t = 0;
    let mut depth: spt_t = 0;
    let mut wbuf: [::core::ffi::c_uchar; 4] = [0; 4];
    let mut n: ::core::ffi::c_int = 0;
    let mut cbytes: ::core::ffi::c_int = 0;
    if (*__pdf_state).current_font < 0 as ::core::ffi::c_int {
        pdf_abort!(b"No font selected!\0" as *const u8 as *const ::core::ffi::c_char);
    }
    font = (*__pdf_state).loaded_fonts.offset((*__pdf_state).current_font as isize) as *mut loaded_font as *mut loaded_font;
    memcpy(
        &raw mut wbuf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut (*font).padbytes as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as size_t,
    );
    match (*font).type_0 {
        PHYSICAL => {
            width = tfm_get_fw_width((*font).tfm_id, ch) as spt_t;
            width = sqxfw((*font).size as int32_t, width as fixword) as spt_t;
            if ch > 65535 as int32_t {
                if (*font).is_unicode == 0 && tfm_is_jfm((*font).tfm_id) != 0 {
                    wbuf[0 as ::core::ffi::c_int as usize] =
                        (0xd800 as int32_t + (ch - 0x10000 as int32_t >> 10 as ::core::ffi::c_int & 0x3ff as int32_t) >> 8 as ::core::ffi::c_int
                            & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[1 as ::core::ffi::c_int as usize] = (0xd800 as int32_t
                        + (ch - 0x10000 as int32_t >> 10 as ::core::ffi::c_int & 0x3ff as int32_t)
                        & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[2 as ::core::ffi::c_int as usize] =
                        (0xdc00 as int32_t + (ch & 0x3ff as int32_t) >> 8 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[3 as ::core::ffi::c_int as usize] = (0xdc00 as int32_t + (ch & 0x3ff as int32_t) & 0xff as int32_t) as ::core::ffi::c_uchar;
                } else {
                    wbuf[0 as ::core::ffi::c_int as usize] = (ch >> 24 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[1 as ::core::ffi::c_int as usize] = (ch >> 16 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[2 as ::core::ffi::c_int as usize] = (ch >> 8 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                    wbuf[3 as ::core::ffi::c_int as usize] = (ch & 0xff as int32_t) as ::core::ffi::c_uchar;
                }
                n = 4 as ::core::ffi::c_int;
            } else if ch > 255 as int32_t {
                wbuf[2 as ::core::ffi::c_int as usize] = (ch >> 8 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
                wbuf[3 as ::core::ffi::c_int as usize] = (ch & 0xff as int32_t) as ::core::ffi::c_uchar;
                n = 2 as ::core::ffi::c_int;
            } else if (*font).subfont_id >= 0 as ::core::ffi::c_int {
                let mut uch: uint16_t = lookup_sfd_record((*font).subfont_id, ch as ::core::ffi::c_uchar) as uint16_t;
                wbuf[2 as ::core::ffi::c_int as usize] =
                    (uch as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                wbuf[3 as ::core::ffi::c_int as usize] = (uch as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                n = 2 as ::core::ffi::c_int;
            } else {
                wbuf[3 as ::core::ffi::c_int as usize] = ch as ::core::ffi::c_uchar;
                n = 1 as ::core::ffi::c_int;
            }
            cbytes = if (*font).minbytes > n { (*font).minbytes } else { n };
            set_string(
                (*__pdf_state).dvi_state.h as spt_t,
                -((*__pdf_state).dvi_state.v as spt_t),
                (&raw mut wbuf as *mut ::core::ffi::c_uchar)
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(-(cbytes as isize)) as *const ::core::ffi::c_void,
                cbytes,
                width,
                (*font).font_id,
            );
            if dvi_is_tracking_boxes() {
                let mut rect: pdf_rect = pdf_rect {
                    llx: 0.,
                    lly: 0.,
                    urx: 0.,
                    ury: 0.,
                };
                height = tfm_get_fw_height((*font).tfm_id, ch) as spt_t;
                depth = tfm_get_fw_depth((*font).tfm_id, ch) as spt_t;
                height = sqxfw((*font).size as int32_t, height as fixword) as spt_t;
                depth = sqxfw((*font).size as int32_t, depth as fixword) as spt_t;
                calc_rect(
                    &raw mut rect,
                    (*__pdf_state).dvi_state.h as spt_t,
                    -((*__pdf_state).dvi_state.v as spt_t),
                    width,
                    height,
                    depth,
                );
                pdf_doc_expand_box(&raw mut rect);
            }
        }
        VIRTUAL => {
            vf_set_char(ch, (*font).font_id);
        }
        _ => {}
    };
}
#[unsafe(export_name = "__pdf_impl_dvi_rule")]
pub unsafe extern "C-unwind" fn dvi_rule(mut width: int32_t, mut height: int32_t) {
    let __pdf_state = unsafe { document_state() };
    if width > 0 as int32_t && height > 0 as int32_t {
        do_moveto((*__pdf_state).dvi_state.h, (*__pdf_state).dvi_state.v);
        match (*__pdf_state).dvi_state.d {
            0 => {
                set_rule(
                    (*__pdf_state).dvi_state.h as spt_t,
                    -((*__pdf_state).dvi_state.v as spt_t),
                    width as spt_t,
                    height as spt_t,
                );
            }
            1 => {
                set_rule(
                    (*__pdf_state).dvi_state.h as spt_t,
                    -((*__pdf_state).dvi_state.v as spt_t) - width as spt_t,
                    height as spt_t,
                    width as spt_t,
                );
            }
            3 => {
                set_rule(
                    (*__pdf_state).dvi_state.h as spt_t - height as spt_t,
                    -((*__pdf_state).dvi_state.v as spt_t),
                    height as spt_t,
                    width as spt_t,
                );
            }
            _ => {}
        }
    }
}
#[unsafe(export_name = "__pdf_impl_dvi_dirchg")]
pub unsafe extern "C-unwind" fn dvi_dirchg(mut dir: ::core::ffi::c_uchar) {
    let __pdf_state = unsafe { document_state() };
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        fprintf(
            crate::backend::runtime::stderr().cast(),
            b"  > dvi_dirchg %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            dir as ::core::ffi::c_int,
        );
    }
    (*__pdf_state).dvi_state.d = dir as ::core::ffi::c_uint;
    pdf_dev_set_dirmode((*__pdf_state).dvi_state.d as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn do_setrule() {
    let __pdf_state = unsafe { document_state() };
    let mut width: int32_t = 0;
    let mut height: int32_t = 0;
    height = get_buffered_signed_quad();
    width = get_buffered_signed_quad();
    match (*__pdf_state).lr_mode {
        LTYPESETTING => {
            dvi_rule(width, height);
            dvi_right(width);
        }
        RTYPESETTING => {
            dvi_right(width);
            dvi_rule(width, height);
        }
        _ => {
            (*__pdf_state).lr_width = (*__pdf_state).lr_width.wrapping_add(width as uint32_t);
        }
    };
}
unsafe extern "C-unwind" fn do_putrule() {
    let __pdf_state = unsafe { document_state() };
    let mut width: int32_t = 0;
    let mut height: int32_t = 0;
    height = get_buffered_signed_quad();
    width = get_buffered_signed_quad();
    match (*__pdf_state).lr_mode {
        LTYPESETTING => {
            dvi_rule(width, height);
        }
        RTYPESETTING => {
            dvi_right(width);
            dvi_rule(width, height);
            dvi_right(-width);
        }
        _ => {}
    };
}
#[unsafe(export_name = "__pdf_impl_dvi_push")]
pub unsafe extern "C-unwind" fn dvi_push() {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).dvi_stack_depth as ::core::ffi::c_uint >= DVI_STACK_DEPTH_MAX {
        pdf_abort!(b"DVI stack exceeded limit.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    let fresh6 = (*__pdf_state).dvi_stack_depth;
    (*__pdf_state).dvi_stack_depth = (*__pdf_state).dvi_stack_depth + 1;
    (*__pdf_state).dvi_stack[fresh6 as usize] = (*__pdf_state).dvi_state;
}
#[unsafe(export_name = "__pdf_impl_dpx_dvi_pop")]
pub unsafe extern "C-unwind" fn dpx_dvi_pop() {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).dvi_stack_depth <= 0 as ::core::ffi::c_int {
        pdf_abort!(b"Tried to pop an empty stack.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*__pdf_state).dvi_stack_depth -= 1;
    (*__pdf_state).dvi_state = (*__pdf_state).dvi_stack[(*__pdf_state).dvi_stack_depth as usize];
    do_moveto((*__pdf_state).dvi_state.h, (*__pdf_state).dvi_state.v);
    pdf_dev_set_dirmode((*__pdf_state).dvi_state.d as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_dvi_w")]
pub unsafe extern "C-unwind" fn dvi_w(mut ch: int32_t) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).dvi_state.w = ch;
    dvi_right(ch);
}
#[unsafe(export_name = "__pdf_impl_dvi_w0")]
pub unsafe extern "C-unwind" fn dvi_w0() {
    let __pdf_state = unsafe { document_state() };
    dvi_right((*__pdf_state).dvi_state.w);
}
#[unsafe(export_name = "__pdf_impl_dvi_x")]
pub unsafe extern "C-unwind" fn dvi_x(mut ch: int32_t) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).dvi_state.x = ch;
    dvi_right(ch);
}
#[unsafe(export_name = "__pdf_impl_dvi_x0")]
pub unsafe extern "C-unwind" fn dvi_x0() {
    let __pdf_state = unsafe { document_state() };
    dvi_right((*__pdf_state).dvi_state.x);
}
#[unsafe(export_name = "__pdf_impl_dvi_y")]
pub unsafe extern "C-unwind" fn dvi_y(mut ch: int32_t) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).dvi_state.y = ch;
    dvi_down(ch);
}
#[unsafe(export_name = "__pdf_impl_dvi_y0")]
pub unsafe extern "C-unwind" fn dvi_y0() {
    let __pdf_state = unsafe { document_state() };
    dvi_down((*__pdf_state).dvi_state.y);
}
#[unsafe(export_name = "__pdf_impl_dvi_z")]
pub unsafe extern "C-unwind" fn dvi_z(mut ch: int32_t) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).dvi_state.z = ch;
    dvi_down(ch);
}
#[unsafe(export_name = "__pdf_impl_dvi_z0")]
pub unsafe extern "C-unwind" fn dvi_z0() {
    let __pdf_state = unsafe { document_state() };
    dvi_down((*__pdf_state).dvi_state.z);
}
unsafe extern "C-unwind" fn skip_fntdef() {
    let __pdf_state = unsafe { document_state() };
    let mut area_len: ::core::ffi::c_int = 0;
    let mut name_len: ::core::ffi::c_int = 0;
    tt_skip_bytes(12 as ::core::ffi::c_uint, (*__pdf_state).dvi_handle);
    area_len = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    name_len = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    tt_skip_bytes((area_len + name_len) as ::core::ffi::c_uint, (*__pdf_state).dvi_handle);
}
unsafe extern "C-unwind" fn do_fntdef(mut tex_id: uint32_t) {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).linear != 0 {
        read_font_record(tex_id);
    } else {
        skip_fntdef();
    }
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_sub(1);
}
#[unsafe(export_name = "__pdf_impl_dvi_set_font")]
pub unsafe extern "C-unwind" fn dvi_set_font(mut font_id: ::core::ffi::c_int) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).current_font = font_id;
}
unsafe extern "C-unwind" fn do_fnt(mut tex_id: uint32_t) {
    let __pdf_state = unsafe { document_state() };
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (*__pdf_state).num_def_fonts {
        if (*(*__pdf_state).def_fonts.offset(i as isize)).tex_id as uint32_t == tex_id {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i == (*__pdf_state).num_def_fonts {
        pdf_abort!(
            b"Tried to select a font that hasn't been defined: id=%d\0" as *const u8 as *const ::core::ffi::c_char,
            tex_id,
        );
    }
    if (*(*__pdf_state).def_fonts.offset(i as isize)).used == 0 {
        let mut font_id: ::core::ffi::c_uint = 0;
        if (*(*__pdf_state).def_fonts.offset(i as isize)).native != 0 {
            font_id = dvi_locate_native_font(
                (*(*__pdf_state).def_fonts.offset(i as isize)).font_name,
                (*(*__pdf_state).def_fonts.offset(i as isize)).face_index,
                (*(*__pdf_state).def_fonts.offset(i as isize)).point_size,
                (*(*__pdf_state).def_fonts.offset(i as isize)).layout_dir,
                (*(*__pdf_state).def_fonts.offset(i as isize)).extend,
                (*(*__pdf_state).def_fonts.offset(i as isize)).slant,
                (*(*__pdf_state).def_fonts.offset(i as isize)).embolden,
            ) as ::core::ffi::c_uint;
        } else {
            font_id = dvi_locate_font(
                (*(*__pdf_state).def_fonts.offset(i as isize)).font_name,
                (*(*__pdf_state).def_fonts.offset(i as isize)).point_size,
            );
        }
        (*(*__pdf_state).loaded_fonts.offset(font_id as isize)).rgba_color = (*(*__pdf_state).def_fonts.offset(i as isize)).rgba_color;
        (*(*__pdf_state).loaded_fonts.offset(font_id as isize)).rgba_used = (*(*__pdf_state).def_fonts.offset(i as isize)).rgba_used;
        if (*(*__pdf_state).loaded_fonts.offset(font_id as isize)).rgba_used as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*(*__pdf_state).loaded_fonts.offset(font_id as isize)).xgs_id = -(1 as ::core::ffi::c_int);
        } else {
            let mut xgs_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            let mut a: ::core::ffi::c_int =
                ((*(*__pdf_state).loaded_fonts.offset(font_id as isize)).rgba_color & 0xff as uint32_t) as ::core::ffi::c_int;
            xgs_dict = pdf_new_dict();
            pdf_add_dict(
                xgs_dict,
                pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_new_name(b"ExtGState\0" as *const u8 as *const ::core::ffi::c_char),
            );
            pdf_add_dict(
                xgs_dict,
                pdf_new_name(b"ca\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_new_number(a as ::core::ffi::c_double / 255.0f64),
            );
            pdf_add_dict(
                xgs_dict,
                pdf_new_name(b"CA\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_new_number(a as ::core::ffi::c_double / 255.0f64),
            );
            (*(*__pdf_state).loaded_fonts.offset(font_id as isize)).xgs_id = pdf_defineresource(
                b"ExtGState\0" as *const u8 as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
                xgs_dict,
                0 as ::core::ffi::c_int,
            );
        }
        (*(*__pdf_state).loaded_fonts.offset(font_id as isize)).source = DVI;
        (*(*__pdf_state).def_fonts.offset(i as isize)).used = 1 as ::core::ffi::c_int;
        (*(*__pdf_state).def_fonts.offset(i as isize)).font_id = font_id as ::core::ffi::c_int;
    }
    (*__pdf_state).current_font = (*(*__pdf_state).def_fonts.offset(i as isize)).font_id;
}
unsafe extern "C-unwind" fn do_xxx(mut size: int32_t) {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).lr_mode < SKIMMING {
        dvi_do_special(
            (*__pdf_state).dvi_page_buffer.offset((*__pdf_state).dvi_page_buf_index as isize) as *const ::core::ffi::c_void,
            size,
        );
    }
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(size as ::core::ffi::c_uint);
}
unsafe extern "C-unwind" fn do_bop() {
    let __pdf_state = unsafe { document_state() };
    let mut i: ::core::ffi::c_uint = 0;
    if (*__pdf_state).processing_page != 0 {
        pdf_abort!(b"Got a bop in the middle of a page!\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    i = 0 as ::core::ffi::c_uint;
    while i < 10 as ::core::ffi::c_uint {
        (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(4 as ::core::ffi::c_uint);
        i = i.wrapping_add(1);
    }
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(4 as ::core::ffi::c_uint);
    clear_state();
    (*__pdf_state).processing_page = 1 as ::core::ffi::c_int;
    pdf_doc_begin_page(dvi_tell_mag(), (*__pdf_state).dev_origin_x, (*__pdf_state).dev_origin_y);
    spc_exec_at_begin_page();
}
unsafe extern "C-unwind" fn do_eop() {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).processing_page = 0 as ::core::ffi::c_int;
    if (*__pdf_state).dvi_stack_depth != 0 as ::core::ffi::c_int {
        pdf_abort!(b"DVI stack depth is not zero at end of page\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    spc_exec_at_end_page();
    pdf_doc_end_page();
}
unsafe extern "C-unwind" fn do_dir() {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).dvi_state.d = get_buffered_unsigned_byte() as ::core::ffi::c_uint;
    pdf_dev_set_dirmode((*__pdf_state).dvi_state.d as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn lr_width_push() {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).lr_width_stack_depth >= DVI_STACK_DEPTH_MAX {
        pdf_abort!(b"Segment width stack exceeded limit.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    let fresh7 = (*__pdf_state).lr_width_stack_depth;
    (*__pdf_state).lr_width_stack_depth = (*__pdf_state).lr_width_stack_depth.wrapping_add(1);
    (*__pdf_state).lr_width_stack[fresh7 as usize] = (*__pdf_state).lr_width;
}
unsafe extern "C-unwind" fn lr_width_pop() {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).lr_width_stack_depth <= 0 as ::core::ffi::c_uint {
        pdf_abort!(b"Tried to pop an empty segment width stack.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*__pdf_state).lr_width_stack_depth = (*__pdf_state).lr_width_stack_depth.wrapping_sub(1);
    (*__pdf_state).lr_width = (*__pdf_state).lr_width_stack[(*__pdf_state).lr_width_stack_depth as usize];
}
unsafe extern "C-unwind" fn dvi_begin_reflect() {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).lr_mode >= SKIMMING {
        (*__pdf_state).lr_mode += 1;
    } else {
        (*__pdf_state).lr_state.buf_index = (*__pdf_state).dvi_page_buf_index;
        (*__pdf_state).lr_state.font = (*__pdf_state).current_font;
        (*__pdf_state).lr_state.state = (*__pdf_state).lr_mode;
        (*__pdf_state).lr_mode = SKIMMING;
        (*__pdf_state).lr_width = 0 as uint32_t;
    };
}
unsafe extern "C-unwind" fn dvi_end_reflect() {
    let __pdf_state = unsafe { document_state() };
    match (*__pdf_state).lr_mode {
        SKIMMING => {
            (*__pdf_state).current_font = (*__pdf_state).lr_state.font;
            (*__pdf_state).dvi_page_buf_index = (*__pdf_state).lr_state.buf_index;
            (*__pdf_state).lr_mode = LTYPESETTING + RTYPESETTING - (*__pdf_state).lr_state.state;
            dvi_right(-((*__pdf_state).lr_width as int32_t));
            lr_width_push();
        }
        LTYPESETTING | RTYPESETTING => {
            lr_width_pop();
            dvi_right(-((*__pdf_state).lr_width as int32_t));
            (*__pdf_state).lr_mode = LTYPESETTING + RTYPESETTING - (*__pdf_state).lr_mode;
        }
        _ => {
            (*__pdf_state).lr_mode -= 1;
        }
    };
}
unsafe extern "C-unwind" fn skip_native_font_def() {
    let __pdf_state = unsafe { document_state() };
    let mut flags: ::core::ffi::c_uint = 0;
    let mut name_length: ::core::ffi::c_int = 0;
    tt_skip_bytes(4 as ::core::ffi::c_uint, (*__pdf_state).dvi_handle);
    flags = tt_get_unsigned_pair((*__pdf_state).dvi_handle) as ::core::ffi::c_uint;
    name_length = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    tt_skip_bytes((name_length + 4 as ::core::ffi::c_int) as ::core::ffi::c_uint, (*__pdf_state).dvi_handle);
    if flags & XDV_FLAG_COLORED as ::core::ffi::c_uint != 0 {
        tt_skip_bytes(4 as ::core::ffi::c_uint, (*__pdf_state).dvi_handle);
    }
    if flags & XDV_FLAG_EXTEND as ::core::ffi::c_uint != 0 {
        tt_skip_bytes(4 as ::core::ffi::c_uint, (*__pdf_state).dvi_handle);
    }
    if flags & XDV_FLAG_SLANT as ::core::ffi::c_uint != 0 {
        tt_skip_bytes(4 as ::core::ffi::c_uint, (*__pdf_state).dvi_handle);
    }
    if flags & XDV_FLAG_EMBOLDEN as ::core::ffi::c_uint != 0 {
        tt_skip_bytes(4 as ::core::ffi::c_uint, (*__pdf_state).dvi_handle);
    }
}
unsafe extern "C-unwind" fn do_native_font_def(mut tex_id: int32_t) {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).linear != 0 {
        read_native_font_record(tex_id as uint32_t);
    } else {
        skip_native_font_def();
    }
    (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_sub(1);
}
unsafe extern "C-unwind" fn skip_glyphs() {
    let __pdf_state = unsafe { document_state() };
    let mut i: ::core::ffi::c_uint = 0;
    let mut slen: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    slen = get_buffered_unsigned_pair();
    i = 0 as ::core::ffi::c_uint;
    while i < slen {
        (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(4 as ::core::ffi::c_uint);
        (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(4 as ::core::ffi::c_uint);
        (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(2 as ::core::ffi::c_uint);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C-unwind" fn do_glyphs(mut do_actual_text: ::core::ffi::c_int) {
    let __pdf_state = unsafe { document_state() };
    let mut font: *mut loaded_font = ::core::ptr::null_mut::<loaded_font>();
    let mut width: spt_t = 0;
    let mut height: spt_t = 0;
    let mut depth: spt_t = 0;
    let mut xloc: *mut spt_t = ::core::ptr::null_mut::<spt_t>();
    let mut yloc: *mut spt_t = ::core::ptr::null_mut::<spt_t>();
    let mut wbuf: [::core::ffi::c_uchar; 2] = [0; 2];
    let mut i: int32_t = 0;
    let mut glyph_id: uint16_t = 0;
    let mut slen: uint16_t = 0 as uint16_t;
    if (*__pdf_state).current_font < 0 as ::core::ffi::c_int {
        pdf_abort!(b"No font selected!\0" as *const u8 as *const ::core::ffi::c_char);
    }
    font = (*__pdf_state).loaded_fonts.offset((*__pdf_state).current_font as isize) as *mut loaded_font as *mut loaded_font;
    if do_actual_text != 0 {
        slen = get_buffered_unsigned_pair() as uint16_t;
        if (*__pdf_state).lr_mode >= SKIMMING {
            i = 0 as ::core::ffi::c_int as int32_t;
            while i < slen as int32_t {
                (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(2 as ::core::ffi::c_uint);
                i += 1;
            }
        } else {
            let mut unicodes: *mut uint16_t =
                new((slen as uint32_t as usize).wrapping_mul(::core::mem::size_of::<uint16_t>() as usize) as uint32_t) as *mut uint16_t;
            i = 0 as ::core::ffi::c_int as int32_t;
            while i < slen as int32_t {
                *unicodes.offset(i as isize) = get_buffered_unsigned_pair() as uint16_t;
                i += 1;
            }
            pdf_dev_begin_actualtext(unicodes, slen as ::core::ffi::c_int);
            free(unicodes as *mut ::core::ffi::c_void);
        }
    }
    width = get_buffered_signed_quad() as spt_t;
    if (*__pdf_state).lr_mode >= SKIMMING {
        (*__pdf_state).lr_width = (*__pdf_state).lr_width.wrapping_add(width as uint32_t);
        skip_glyphs();
        return;
    }
    if (*__pdf_state).lr_mode == RTYPESETTING {
        dvi_right(width as int32_t);
    }
    slen = get_buffered_unsigned_pair() as uint16_t;
    xloc = new((slen as uint32_t as usize).wrapping_mul(::core::mem::size_of::<spt_t>() as usize) as uint32_t) as *mut spt_t;
    yloc = new((slen as uint32_t as usize).wrapping_mul(::core::mem::size_of::<spt_t>() as usize) as uint32_t) as *mut spt_t;
    i = 0 as ::core::ffi::c_int as int32_t;
    while i < slen as int32_t {
        *xloc.offset(i as isize) = get_buffered_signed_quad() as spt_t;
        *yloc.offset(i as isize) = get_buffered_signed_quad() as spt_t;
        i += 1;
    }
    if (*font).rgba_used as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        let mut color: pdf_color = pdf_color {
            res_id: 0,
            type_0: 0,
            num_components: 0,
            spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            values: [0.; 32],
            pattern_id: 0,
        };
        pdf_color_rgbcolor(
            &raw mut color,
            (((*font).rgba_color >> 24 as ::core::ffi::c_int) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                as ::core::ffi::c_double
                / 255 as ::core::ffi::c_int as ::core::ffi::c_double,
            (((*font).rgba_color >> 16 as ::core::ffi::c_int) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                as ::core::ffi::c_double
                / 255 as ::core::ffi::c_int as ::core::ffi::c_double,
            (((*font).rgba_color >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                as ::core::ffi::c_double
                / 255 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
        pdf_color_push(&raw mut color, &raw mut color);
        if (*font).xgs_id >= 0 as ::core::ffi::c_int {
            let mut ref_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            let mut resname: [::core::ffi::c_char; 16] = [0; 16];
            let mut content: [::core::ffi::c_char; 22] = [0; 22];
            sprintf(
                &raw mut resname as *mut ::core::ffi::c_char,
                b"Xtx_Gs_%08x\0" as *const u8 as *const ::core::ffi::c_char,
                (*__pdf_state).current_font,
            );
            ref_0 = pdf_get_resource_reference((*font).xgs_id);
            pdf_doc_add_page_resource(
                b"ExtGState\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut resname as *mut ::core::ffi::c_char,
                ref_0,
            );
            graphics_mode();
            pdf_dev_gsave();
            sprintf(
                &raw mut content as *mut ::core::ffi::c_char,
                b" /%s gs \0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut resname as *mut ::core::ffi::c_char,
            );
            pdf_doc_add_page_content(
                &raw mut content as *mut ::core::ffi::c_char,
                strlen(&raw mut content as *mut ::core::ffi::c_char) as ::core::ffi::c_uint,
            );
        }
    }
    i = 0 as ::core::ffi::c_int as int32_t;
    while i < slen as int32_t {
        let mut advance: spt_t = 0 as spt_t;
        glyph_id = get_buffered_unsigned_pair() as uint16_t;
        if (glyph_id as ::core::ffi::c_int) < (*font).num_glyphs as ::core::ffi::c_int {
            if (*font).shift_gid != 0 {
                glyph_id = glyph_id.wrapping_add(1);
            }
            advance = (*(*font).gm.offset(glyph_id as isize)).advance;
            if dvi_is_tracking_boxes() {
                let mut rect: pdf_rect = pdf_rect {
                    llx: 0.,
                    lly: 0.,
                    urx: 0.,
                    ury: 0.,
                };
                height = (*(*font).gm.offset(glyph_id as isize)).ascent;
                depth = -(*(*font).gm.offset(glyph_id as isize)).descent;
                calc_rect(
                    &raw mut rect,
                    (*__pdf_state).dvi_state.h as spt_t + *xloc.offset(i as isize),
                    -((*__pdf_state).dvi_state.v as spt_t) - *yloc.offset(i as isize),
                    advance,
                    height,
                    depth,
                );
                pdf_doc_expand_box(&raw mut rect);
            }
        }
        wbuf[0 as ::core::ffi::c_int as usize] = (glyph_id as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        wbuf[1 as ::core::ffi::c_int as usize] = (glyph_id as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        set_string(
            (*__pdf_state).dvi_state.h as spt_t + *xloc.offset(i as isize),
            -((*__pdf_state).dvi_state.v as spt_t) - *yloc.offset(i as isize),
            &raw mut wbuf as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            2 as ::core::ffi::c_int,
            advance,
            (*font).font_id,
        );
        i += 1;
    }
    if (*font).rgba_used as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        if (*font).xgs_id >= 0 as ::core::ffi::c_int {
            graphics_mode();
            pdf_dev_grestore();
        }
        pdf_color_pop();
    }
    free(xloc as *mut ::core::ffi::c_void);
    free(yloc as *mut ::core::ffi::c_void);
    if do_actual_text != 0 {
        pdf_dev_end_actualtext();
    }
    if (*__pdf_state).lr_mode == LTYPESETTING {
        dvi_right(width as int32_t);
    }
}
unsafe extern "C-unwind" fn check_postamble() {
    let __pdf_state = unsafe { document_state() };
    let mut code: ::core::ffi::c_int = 0;
    tt_skip_bytes(28 as ::core::ffi::c_uint, (*__pdf_state).dvi_handle);
    loop {
        code = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
        if !(code != POST_POST) {
            break;
        }
        match code {
            FNT_DEF1 | FNT_DEF2 | FNT_DEF3 | FNT_DEF4 => {
                tt_skip_bytes(
                    (code + 1 as ::core::ffi::c_int - FNT_DEF1) as ::core::ffi::c_uint,
                    (*__pdf_state).dvi_handle,
                );
                skip_fntdef();
            }
            XDV_NATIVE_FONT_DEF => {
                tt_skip_bytes(4 as ::core::ffi::c_uint, (*__pdf_state).dvi_handle);
                skip_native_font_def();
            }
            _ => {
                pdf_abort!(b"Unexpected op code (%u) in postamble\0" as *const u8 as *const ::core::ffi::c_char, code,);
            }
        }
    }
    tt_skip_bytes(4 as ::core::ffi::c_uint, (*__pdf_state).dvi_handle);
    (*__pdf_state).post_id_byte = tt_get_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_int;
    if !((*__pdf_state).post_id_byte == DVI_ID
        || (*__pdf_state).post_id_byte == DVIV_ID
        || (*__pdf_state).post_id_byte == XDV_ID
        || (*__pdf_state).post_id_byte == XDV_ID_OLD)
    {
        pdf_message!(b"DVI ID = %d\n\0" as *const u8 as *const ::core::ffi::c_char, (*__pdf_state).post_id_byte,);
        pdf_abort!(&raw const invalid_signature as *const ::core::ffi::c_char);
    }
    check_id_bytes();
    if (*__pdf_state).has_ptex != 0 && (*__pdf_state).post_id_byte != DVIV_ID {
        pdf_abort!(
            b"Saw opcode %i in DVI file not for Ascii pTeX\0" as *const u8 as *const ::core::ffi::c_char,
            PTEXDIR,
        );
    }
    (*__pdf_state).num_pages = 0 as ::core::ffi::c_uint;
}
#[unsafe(export_name = "__pdf_impl_dvi_do_page")]
pub unsafe extern "C-unwind" fn dvi_do_page(
    mut page_paper_height: ::core::ffi::c_double, mut hmargin: ::core::ffi::c_double, mut vmargin: ::core::ffi::c_double,
) {
    let __pdf_state = unsafe { document_state() };
    let mut opcode: ::core::ffi::c_uchar = 0;
    (*__pdf_state).dvi_page_buf_index = 0 as ::core::ffi::c_uint;
    (*__pdf_state).dev_origin_x = hmargin;
    (*__pdf_state).dev_origin_y = page_paper_height - vmargin;
    (*__pdf_state).dvi_stack_depth = 0 as ::core::ffi::c_int;
    loop {
        opcode = get_buffered_unsigned_byte() as ::core::ffi::c_uchar;
        if opcode as ::core::ffi::c_int <= SET_CHAR_127 {
            dvi_set(opcode as int32_t);
        } else if opcode as ::core::ffi::c_int >= FNT_NUM_0 && opcode as ::core::ffi::c_int <= FNT_NUM_63 {
            do_fnt((opcode as ::core::ffi::c_int - FNT_NUM_0) as uint32_t);
        } else {
            let mut current_block_59: u64;
            match opcode as ::core::ffi::c_int {
                SET1 | SET2 | SET3 => {
                    dvi_set(get_buffered_unsigned_num((opcode as ::core::ffi::c_int - SET1) as ::core::ffi::c_uchar));
                    current_block_59 = 10007731352114176167;
                }
                SET4 => {
                    pdf_abort!(b"Multibyte (>24 bits) character not supported!\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                SET_RULE => {
                    do_setrule();
                    current_block_59 = 10007731352114176167;
                }
                PUT1 | PUT2 | PUT3 => {
                    dvi_put(get_buffered_unsigned_num((opcode as ::core::ffi::c_int - PUT1) as ::core::ffi::c_uchar));
                    current_block_59 = 10007731352114176167;
                }
                PUT4 => {
                    pdf_abort!(b"Multibyte (>24 bits) character not supported!\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                PUT_RULE => {
                    do_putrule();
                    current_block_59 = 10007731352114176167;
                }
                BOP => {
                    do_bop();
                    current_block_59 = 10007731352114176167;
                }
                EOP => {
                    do_eop();
                    if (*__pdf_state).linear != 0 {
                        opcode = tt_get_unsigned_byte((*__pdf_state).dvi_handle);
                        if opcode as ::core::ffi::c_int == POST {
                            check_postamble();
                        } else {
                            ttstub_input_ungetc((*__pdf_state).dvi_handle, opcode as ::core::ffi::c_int);
                        }
                    }
                    return;
                }
                PUSH => {
                    dvi_push();
                    if (*__pdf_state).lr_mode >= SKIMMING {
                        lr_width_push();
                    }
                    dvi_mark_depth();
                    current_block_59 = 10007731352114176167;
                }
                POP => {
                    dpx_dvi_pop();
                    if (*__pdf_state).lr_mode >= SKIMMING {
                        lr_width_pop();
                    }
                    dvi_mark_depth();
                    current_block_59 = 10007731352114176167;
                }
                RIGHT1 | RIGHT2 | RIGHT3 | RIGHT4 => {
                    dvi_right(get_buffered_signed_num((opcode as ::core::ffi::c_int - RIGHT1) as ::core::ffi::c_uchar));
                    current_block_59 = 10007731352114176167;
                }
                W0 => {
                    dvi_w0();
                    current_block_59 = 10007731352114176167;
                }
                W1 | W2 | W3 | W4 => {
                    dvi_w(get_buffered_signed_num((opcode as ::core::ffi::c_int - W1) as ::core::ffi::c_uchar));
                    current_block_59 = 10007731352114176167;
                }
                X0 => {
                    dvi_x0();
                    current_block_59 = 10007731352114176167;
                }
                X1 | X2 | X3 | X4 => {
                    dvi_x(get_buffered_signed_num((opcode as ::core::ffi::c_int - X1) as ::core::ffi::c_uchar));
                    current_block_59 = 10007731352114176167;
                }
                DOWN1 | DOWN2 | DOWN3 | DOWN4 => {
                    dvi_down(get_buffered_signed_num((opcode as ::core::ffi::c_int - DOWN1) as ::core::ffi::c_uchar));
                    current_block_59 = 10007731352114176167;
                }
                Y0 => {
                    dvi_y0();
                    current_block_59 = 10007731352114176167;
                }
                Y1 | Y2 | Y3 | Y4 => {
                    dvi_y(get_buffered_signed_num((opcode as ::core::ffi::c_int - Y1) as ::core::ffi::c_uchar));
                    current_block_59 = 10007731352114176167;
                }
                Z0 => {
                    dvi_z0();
                    current_block_59 = 10007731352114176167;
                }
                Z1 | Z2 | Z3 | Z4 => {
                    dvi_z(get_buffered_signed_num((opcode as ::core::ffi::c_int - Z1) as ::core::ffi::c_uchar));
                    current_block_59 = 10007731352114176167;
                }
                FNT1 | FNT2 | FNT3 | FNT4 => {
                    do_fnt(get_buffered_unsigned_num((opcode as ::core::ffi::c_int - FNT1) as ::core::ffi::c_uchar) as uint32_t);
                    current_block_59 = 10007731352114176167;
                }
                XXX1 | XXX2 | XXX3 | XXX4 => {
                    let mut size: int32_t = get_buffered_unsigned_num((opcode as ::core::ffi::c_int - XXX1) as ::core::ffi::c_uchar);
                    if size < 0 as int32_t {
                        pdf_warning!(b"DVI: Special with %d bytes???\0" as *const u8 as *const ::core::ffi::c_char, size,);
                    } else {
                        do_xxx(size);
                    }
                    current_block_59 = 10007731352114176167;
                }
                NOP | FNT_DEF1 | FNT_DEF2 | FNT_DEF3 | FNT_DEF4 => {
                    current_block_59 = 10007731352114176167;
                }
                PTEXDIR => {
                    need_pTeX(opcode as ::core::ffi::c_int);
                    do_dir();
                    current_block_59 = 10007731352114176167;
                }
                XDV_GLYPHS => {
                    need_XeTeX(opcode as ::core::ffi::c_int);
                    do_glyphs(0 as ::core::ffi::c_int);
                    current_block_59 = 10007731352114176167;
                }
                XDV_TEXT_AND_GLYPHS => {
                    need_XeTeX(opcode as ::core::ffi::c_int);
                    do_glyphs(1 as ::core::ffi::c_int);
                    current_block_59 = 10007731352114176167;
                }
                XDV_NATIVE_FONT_DEF => {
                    need_XeTeX(opcode as ::core::ffi::c_int);
                    current_block_59 = 10007731352114176167;
                }
                BEGIN_REFLECT => {
                    need_XeTeX(opcode as ::core::ffi::c_int);
                    dvi_begin_reflect();
                    current_block_59 = 10007731352114176167;
                }
                END_REFLECT => {
                    need_XeTeX(opcode as ::core::ffi::c_int);
                    dvi_end_reflect();
                    current_block_59 = 10007731352114176167;
                }
                POST => {
                    if (*__pdf_state).linear as ::core::ffi::c_int != 0 && (*__pdf_state).processing_page == 0 {
                        (*__pdf_state).num_pages = 0 as ::core::ffi::c_uint;
                        return;
                    }
                    current_block_59 = 18402748268671662874;
                }
                PRE | POST_POST => {
                    current_block_59 = 18402748268671662874;
                }
                _ => {
                    pdf_abort!(b"Unexpected opcode or DVI file ended prematurely\0" as *const u8 as *const ::core::ffi::c_char,);
                }
            }
            match current_block_59 {
                18402748268671662874 => {
                    pdf_abort!(b"Unexpected preamble or postamble in dvi file\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                _ => {}
            }
        }
    }
}
#[unsafe(export_name = "__pdf_impl_dvi_init")]
pub unsafe extern "C-unwind" fn dvi_init(mut dvi_filename: *const ::core::ffi::c_char, mut mag: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let __pdf_state = unsafe { document_state() };
    let mut post_location: int32_t = 0;
    if dvi_filename.is_null() {
        pdf_abort!(b"filename must be specified\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*__pdf_state).dvi_handle = ttstub_input_open(dvi_filename, TTBC_FILE_FORMAT_PROGRAM_DATA, 0 as ::core::ffi::c_int);
    if (*__pdf_state).dvi_handle == INVALID_HANDLE {
        pdf_abort!(b"cannot open \"%s\"\0" as *const u8 as *const ::core::ffi::c_char, dvi_filename,);
    }
    post_location = find_post();
    get_dvi_info(post_location);
    do_scales(mag);
    get_page_info(post_location);
    get_comment();
    get_dvi_fonts(post_location);
    clear_state();
    (*__pdf_state).dvi_page_buf_size = DVI_PAGE_BUF_CHUNK;
    (*__pdf_state).dvi_page_buffer = new(
        ((*__pdf_state).dvi_page_buf_size as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize) as uint32_t
    ) as *mut ::core::ffi::c_uchar;
    return (*__pdf_state).dvi2pts;
}
#[unsafe(export_name = "__pdf_impl_dvi_close")]
pub unsafe extern "C-unwind" fn dvi_close() {
    let __pdf_state = unsafe { document_state() };
    let mut i: ::core::ffi::c_uint = 0;
    if (*__pdf_state).linear != 0 {
        while ttstub_input_getc((*__pdf_state).dvi_handle) != EOF {}
    }
    crate::backend::runtime::input_close((*__pdf_state).dvi_handle);
    (*__pdf_state).dvi_handle = INVALID_HANDLE as rust_input_handle_t;
    if !(*__pdf_state).def_fonts.is_null() {
        i = 0 as ::core::ffi::c_uint;
        while i < (*__pdf_state).num_def_fonts {
            let ref mut fresh3 = (*(*__pdf_state).def_fonts.offset(i as isize)).font_name;
            *fresh3 = mfree((*(*__pdf_state).def_fonts.offset(i as isize)).font_name as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
            i = i.wrapping_add(1);
        }
        free((*__pdf_state).def_fonts as *mut ::core::ffi::c_void);
    }
    (*__pdf_state).def_fonts = ::core::ptr::null_mut::<font_def>();
    (*__pdf_state).page_loc = mfree((*__pdf_state).page_loc as *mut ::core::ffi::c_void) as *mut uint32_t;
    (*__pdf_state).num_pages = 0 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    while i < (*__pdf_state).num_loaded_fonts {
        free((*(*__pdf_state).loaded_fonts.offset(i as isize)).gm as *mut ::core::ffi::c_void);
        let ref mut fresh4 = (*(*__pdf_state).loaded_fonts.offset(i as isize)).gm;
        *fresh4 = ::core::ptr::null_mut::<gm>();
        i = i.wrapping_add(1);
    }
    (*__pdf_state).loaded_fonts = mfree((*__pdf_state).loaded_fonts as *mut ::core::ffi::c_void) as *mut loaded_font;
    (*__pdf_state).num_loaded_fonts = 0 as ::core::ffi::c_uint;
    vf_close_all_fonts();
    tfm_close_all();
    if !(*__pdf_state).dvi_page_buffer.is_null() {
        (*__pdf_state).dvi_page_buffer = mfree((*__pdf_state).dvi_page_buffer as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_uchar;
        (*__pdf_state).dvi_page_buf_size = 0 as ::core::ffi::c_uint;
    }
}

#[unsafe(export_name = "__pdf_impl_dvi_vf_init")]
pub unsafe extern "C-unwind" fn dvi_vf_init(mut dev_font_id: ::core::ffi::c_int) {
    let __pdf_state = unsafe { document_state() };
    dvi_push();
    (*__pdf_state).dvi_state.w = 0 as ::core::ffi::c_int as int32_t;
    (*__pdf_state).dvi_state.x = 0 as ::core::ffi::c_int as int32_t;
    (*__pdf_state).dvi_state.y = 0 as ::core::ffi::c_int as int32_t;
    (*__pdf_state).dvi_state.z = 0 as ::core::ffi::c_int as int32_t;
    if (*__pdf_state).num_saved_fonts < VF_NESTING_MAX {
        let fresh5 = (*__pdf_state).num_saved_fonts;
        (*__pdf_state).num_saved_fonts = (*__pdf_state).num_saved_fonts.wrapping_add(1);
        (*__pdf_state).saved_dvi_font[fresh5 as usize] = (*__pdf_state).current_font;
    } else {
        pdf_abort!(b"Virtual fonts nested too deeply!\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*__pdf_state).current_font = dev_font_id;
}
#[unsafe(export_name = "__pdf_impl_dvi_vf_finish")]
pub unsafe extern "C-unwind" fn dvi_vf_finish() {
    let __pdf_state = unsafe { document_state() };
    dpx_dvi_pop();
    if (*__pdf_state).num_saved_fonts > 0 as ::core::ffi::c_uint {
        (*__pdf_state).num_saved_fonts = (*__pdf_state).num_saved_fonts.wrapping_sub(1);
        (*__pdf_state).current_font = (*__pdf_state).saved_dvi_font[(*__pdf_state).num_saved_fonts as usize];
    } else {
        pdf_abort!(b"Tried to pop an empty font stack\0" as *const u8 as *const ::core::ffi::c_char,);
    };
}
pub const MAX_PWD_LEN: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn scan_special_encrypt(
    mut key_bits: *mut ::core::ffi::c_int, mut permission: *mut int32_t, mut opassword: *mut ::core::ffi::c_char,
    mut upassword: *mut ::core::ffi::c_char, mut curptr: *mut *const ::core::ffi::c_char, mut endptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = *curptr;
    skip_white(&raw mut p, endptr);
    *opassword.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    *upassword.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    while error == 0 && p < endptr {
        let mut kp: *mut ::core::ffi::c_char = parse_c_ident(&raw mut p, endptr);
        if kp.is_null() {
            break;
        }
        let mut obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        skip_white(&raw mut p, endptr);
        if strcmp(kp, b"ownerpw\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            obj = parse_pdf_string(&raw mut p, endptr);
            if !obj.is_null() {
                if !pdf_string_value(obj).is_null() {
                    let mut str_length: ::core::ffi::c_int =
                        (if (MAX_PWD_LEN - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint > pdf_string_length(obj) {
                            pdf_string_length(obj)
                        } else {
                            (MAX_PWD_LEN - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
                        }) as ::core::ffi::c_int;
                    strncpy(opassword, pdf_string_value(obj) as *const ::core::ffi::c_char, str_length as size_t);
                    *opassword.offset(str_length as isize) = '\0' as i32 as ::core::ffi::c_char;
                }
                pdf_release_obj(obj);
            } else {
                error = -(1 as ::core::ffi::c_int);
            }
        } else if strcmp(kp, b"userpw\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            obj = parse_pdf_string(&raw mut p, endptr);
            if !obj.is_null() {
                if !pdf_string_value(obj).is_null() {
                    let mut str_length_0: ::core::ffi::c_int =
                        (if (MAX_PWD_LEN - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint > pdf_string_length(obj) {
                            pdf_string_length(obj)
                        } else {
                            (MAX_PWD_LEN - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
                        }) as ::core::ffi::c_int;
                    strncpy(upassword, pdf_string_value(obj) as *const ::core::ffi::c_char, str_length_0 as size_t);
                    *upassword.offset(str_length_0 as isize) = '\0' as i32 as ::core::ffi::c_char;
                }
                pdf_release_obj(obj);
            } else {
                error = -(1 as ::core::ffi::c_int);
            }
        } else if strcmp(kp, b"length\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            obj = parse_pdf_number(&raw mut p, endptr);
            if !obj.is_null() && (!obj.is_null() && pdf_obj_typeof(obj) == PDF_NUMBER) {
                *key_bits = pdf_number_value(obj) as ::core::ffi::c_uint as ::core::ffi::c_int;
            } else {
                error = -(1 as ::core::ffi::c_int);
            }
            if !obj.is_null() {
                pdf_release_obj(obj);
            }
        } else if strcmp(kp, b"perm\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            obj = parse_pdf_number(&raw mut p, endptr);
            if !obj.is_null() && (!obj.is_null() && pdf_obj_typeof(obj) == PDF_NUMBER) {
                *permission = pdf_number_value(obj) as ::core::ffi::c_uint as int32_t;
            } else {
                error = -(1 as ::core::ffi::c_int);
            }
            if !obj.is_null() {
                pdf_release_obj(obj);
            }
        } else {
            error = -(1 as ::core::ffi::c_int);
        }
        free(kp as *mut ::core::ffi::c_void);
        skip_white(&raw mut p, endptr);
    }
    *curptr = p;
    return error;
}
unsafe extern "C-unwind" fn scan_special_trailerid(
    mut id1: *mut ::core::ffi::c_uchar, mut id2: *mut ::core::ffi::c_uchar, mut curptr: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut id_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut p: *const ::core::ffi::c_char = *curptr;
    skip_white(&raw mut p, endptr);
    id_array = parse_pdf_array(&raw mut p, endptr, ::core::ptr::null_mut::<pdf_file>());
    if !id_array.is_null() {
        if pdf_array_length(id_array) == 2 as ::core::ffi::c_uint {
            let mut tmp1: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            let mut tmp2: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            tmp1 = pdf_get_array(id_array, 0 as ::core::ffi::c_int);
            tmp2 = pdf_get_array(id_array, 1 as ::core::ffi::c_int);
            if !tmp1.is_null()
                && pdf_obj_typeof(tmp1) == PDF_STRING
                && pdf_string_length(tmp1) == 16 as ::core::ffi::c_uint
                && (!tmp2.is_null() && pdf_obj_typeof(tmp2) == PDF_STRING)
                && pdf_string_length(tmp2) == 16 as ::core::ffi::c_uint
            {
                memcpy(id1 as *mut ::core::ffi::c_void, pdf_string_value(tmp1), 16 as size_t);
                memcpy(id2 as *mut ::core::ffi::c_void, pdf_string_value(tmp2), 16 as size_t);
            } else {
                error = -(1 as ::core::ffi::c_int);
            }
        } else {
            error = -(1 as ::core::ffi::c_int);
        }
        pdf_release_obj(id_array);
    } else {
        error = -(1 as ::core::ffi::c_int);
    }
    skip_white(&raw mut p, endptr);
    *curptr = p;
    return error;
}
unsafe extern "C-unwind" fn scan_special_config(
    mut start: *mut *const ::core::ffi::c_char, mut end: *const ::core::ffi::c_char, mut opt_flags: *mut ::core::ffi::c_int,
    mut compression_level: *mut ::core::ffi::c_int, mut annot_grow_x: *mut ::core::ffi::c_double, mut annot_grow_y: *mut ::core::ffi::c_double,
) {
    skip_white(start, end);
    if *start >= end {
        return;
    }
    let mut option: *mut ::core::ffi::c_char = parse_ident(start, end);
    if option.is_null() {
        return;
    }
    let mut arg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    skip_white(start, end);
    if *start < end {
        if **start as ::core::ffi::c_int == '"' as i32 {
            arg = parse_c_string(start, end);
        } else {
            arg = parse_ident(start, end);
        }
    }
    if streq_ptr(option, b"C\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0 && !arg.is_null() && !opt_flags.is_null() {
        let mut num_end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut flags: ::core::ffi::c_int = strtol(arg, &raw mut num_end, 0 as ::core::ffi::c_int) as ::core::ffi::c_uint as ::core::ffi::c_int;
        if num_end == arg {
            pdf_warning!(
                b"Invalid dvipdfmx compatibility flag: '%s'\0" as *const u8 as *const ::core::ffi::c_char,
                arg,
            );
        } else if flags < 0 as ::core::ffi::c_int {
            *opt_flags = -flags;
        } else {
            *opt_flags |= flags;
        }
    } else if streq_ptr(option, b"z\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
        && !arg.is_null()
        && !compression_level.is_null()
    {
        *compression_level = atoi(arg);
    } else if streq_ptr(option, b"g\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
        && !arg.is_null()
        && !annot_grow_x.is_null()
        && !annot_grow_y.is_null()
    {
        let mut comma: *const ::core::ffi::c_char = strchr(arg, ',' as i32);
        let mut arg_ptr: *const ::core::ffi::c_char = arg;
        let mut arg_end: *const ::core::ffi::c_char = arg.offset(strlen(arg) as isize);
        let mut error: ::core::ffi::c_int = 0;
        if !comma.is_null() {
            error = dpx_util_read_length(annot_grow_x, 1.0f64, &raw mut arg_ptr, comma);
            arg_ptr = comma.offset(1 as ::core::ffi::c_int as isize);
            if error == 0 {
                error = dpx_util_read_length(annot_grow_y, 1.0f64, &raw mut arg_ptr, arg_end);
            }
        } else {
            error = dpx_util_read_length(annot_grow_x, 1.0f64, &raw mut arg_ptr, arg_end);
            if error == 0 {
                *annot_grow_y = *annot_grow_x;
            }
        }
        if error != 0 {
            pdf_warning!(
                b"Error reading argument for \"-g\" option: %s\0" as *const u8 as *const ::core::ffi::c_char,
                arg,
            );
        }
    } else {
        pdf_warning!(
            b"Tectonic doesn't support '%s' config special or the argument is missing\0" as *const u8 as *const ::core::ffi::c_char,
            option,
        );
    }
    free(arg as *mut ::core::ffi::c_void);
    free(option as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn scan_special(
    mut wd: *mut ::core::ffi::c_double, mut ht: *mut ::core::ffi::c_double, mut xo: *mut ::core::ffi::c_double, mut yo: *mut ::core::ffi::c_double,
    mut lm: *mut ::core::ffi::c_int, mut majorversion: *mut ::core::ffi::c_int, mut minorversion: *mut ::core::ffi::c_int,
    mut compression_level: *mut ::core::ffi::c_int, mut annot_grow_x: *mut ::core::ffi::c_double, mut annot_grow_y: *mut ::core::ffi::c_double,
    mut enable_encryption: *mut ::core::ffi::c_int, mut key_bits: *mut ::core::ffi::c_int, mut permission: *mut int32_t,
    mut opassword: *mut ::core::ffi::c_char, mut upassword: *mut ::core::ffi::c_char, mut has_id: *mut ::core::ffi::c_int,
    mut id1: *mut ::core::ffi::c_uchar, mut id2: *mut ::core::ffi::c_uchar, mut opt_flags: *mut ::core::ffi::c_int,
    mut buf_0: *const ::core::ffi::c_char, mut size: uint32_t,
) -> ::core::ffi::c_int {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = buf_0;
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ns_pdf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ns_dvipdfmx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmp: ::core::ffi::c_double = 0.;
    endptr = p.offset(size as isize);
    skip_white(&raw mut p, endptr);
    q = parse_c_ident(&raw mut p, endptr);
    if streq_ptr(q, b"pdf\0" as *const u8 as *const ::core::ffi::c_char) {
        skip_white(&raw mut p, endptr);
        if p < endptr && *p as ::core::ffi::c_int == ':' as i32 {
            p = p.offset(1);
            skip_white(&raw mut p, endptr);
            free(q as *mut ::core::ffi::c_void);
            q = parse_c_ident(&raw mut p, endptr);
            ns_pdf = 1 as ::core::ffi::c_int;
        }
    } else if streq_ptr(q, b"x\0" as *const u8 as *const ::core::ffi::c_char) {
        skip_white(&raw mut p, endptr);
        if p < endptr && *p as ::core::ffi::c_int == ':' as i32 {
            p = p.offset(1);
            skip_white(&raw mut p, endptr);
            free(q as *mut ::core::ffi::c_void);
            q = parse_c_ident(&raw mut p, endptr);
        }
    } else if streq_ptr(q, b"dvipdfmx\0" as *const u8 as *const ::core::ffi::c_char) {
        skip_white(&raw mut p, endptr);
        if p < endptr && *p as ::core::ffi::c_int == ':' as i32 {
            p = p.offset(1);
            skip_white(&raw mut p, endptr);
            free(q as *mut ::core::ffi::c_void);
            q = parse_c_ident(&raw mut p, endptr);
            ns_dvipdfmx = 1 as ::core::ffi::c_int;
        }
    }
    skip_white(&raw mut p, endptr);
    if !q.is_null() {
        if streq_ptr(q, b"landscape\0" as *const u8 as *const ::core::ffi::c_char) {
            *lm = 1 as ::core::ffi::c_int;
        } else if ns_pdf != 0 && streq_ptr(q, b"pagesize\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0 {
            while error == 0 && p < endptr {
                let mut kp: *mut ::core::ffi::c_char = parse_c_ident(&raw mut p, endptr);
                if kp.is_null() {
                    break;
                }
                skip_white(&raw mut p, endptr);
                if streq_ptr(kp, b"width\0" as *const u8 as *const ::core::ffi::c_char) {
                    error = dpx_util_read_length(&raw mut tmp, dvi_tell_mag(), &raw mut p, endptr);
                    if error == 0 {
                        *wd = tmp * dvi_tell_mag();
                    }
                } else if streq_ptr(kp, b"height\0" as *const u8 as *const ::core::ffi::c_char) {
                    error = dpx_util_read_length(&raw mut tmp, dvi_tell_mag(), &raw mut p, endptr);
                    if error == 0 {
                        *ht = tmp * dvi_tell_mag();
                    }
                } else if streq_ptr(kp, b"xoffset\0" as *const u8 as *const ::core::ffi::c_char) {
                    error = dpx_util_read_length(&raw mut tmp, dvi_tell_mag(), &raw mut p, endptr);
                    if error == 0 {
                        *xo = tmp * dvi_tell_mag();
                    }
                } else if streq_ptr(kp, b"yoffset\0" as *const u8 as *const ::core::ffi::c_char) {
                    error = dpx_util_read_length(&raw mut tmp, dvi_tell_mag(), &raw mut p, endptr);
                    if error == 0 {
                        *yo = tmp * dvi_tell_mag();
                    }
                } else if streq_ptr(kp, b"default\0" as *const u8 as *const ::core::ffi::c_char) {
                    *wd = paper_width;
                    *ht = paper_height;
                    *lm = landscape_mode;
                    *yo = 72.0f64;
                    *xo = *yo;
                }
                free(kp as *mut ::core::ffi::c_void);
                skip_white(&raw mut p, endptr);
            }
        } else if streq_ptr(q, b"papersize\0" as *const u8 as *const ::core::ffi::c_char) {
            let mut qchr: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
            if *p as ::core::ffi::c_int == '=' as i32 {
                p = p.offset(1);
            }
            skip_white(&raw mut p, endptr);
            if p < endptr && (*p as ::core::ffi::c_int == '\'' as i32 || *p as ::core::ffi::c_int == '"' as i32) {
                qchr = *p;
                p = p.offset(1);
                skip_white(&raw mut p, endptr);
            }
            error = dpx_util_read_length(&raw mut tmp, 1.0f64, &raw mut p, endptr);
            if error == 0 {
                let mut tmp1: ::core::ffi::c_double = 0.;
                skip_white(&raw mut p, endptr);
                if p < endptr && *p as ::core::ffi::c_int == ',' as i32 {
                    p = p.offset(1);
                    skip_white(&raw mut p, endptr);
                }
                error = dpx_util_read_length(&raw mut tmp1, 1.0f64, &raw mut p, endptr);
                if error == 0 {
                    *wd = tmp;
                    *ht = tmp1;
                    skip_white(&raw mut p, endptr);
                }
            }
            if error == 0 && qchr as ::core::ffi::c_int != 0 {
                if p >= endptr || *p as ::core::ffi::c_int != qchr as ::core::ffi::c_int {
                    error = -(1 as ::core::ffi::c_int);
                }
            }
            if error == 0 as ::core::ffi::c_int {
                paper_width = *wd;
                paper_height = *ht;
            }
        } else if !minorversion.is_null()
            && ns_pdf != 0
            && streq_ptr(q, b"minorversion\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
        {
            let mut kv: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if *p as ::core::ffi::c_int == '=' as i32 {
                p = p.offset(1);
            }
            skip_white(&raw mut p, endptr);
            kv = parse_float_decimal(&raw mut p, endptr);
            if !kv.is_null() {
                *minorversion = strtol(kv, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(), 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
                free(kv as *mut ::core::ffi::c_void);
            }
        } else if !majorversion.is_null()
            && ns_pdf != 0
            && streq_ptr(q, b"majorversion\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
        {
            let mut kv_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            if *p as ::core::ffi::c_int == '=' as i32 {
                p = p.offset(1);
            }
            skip_white(&raw mut p, endptr);
            kv_0 = parse_float_decimal(&raw mut p, endptr);
            if !kv_0.is_null() {
                *majorversion = strtol(kv_0, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(), 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
                free(kv_0 as *mut ::core::ffi::c_void);
            }
        } else if !enable_encryption.is_null()
            && ns_pdf != 0
            && streq_ptr(q, b"encrypt\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
        {
            *enable_encryption = 1 as ::core::ffi::c_int;
            error = scan_special_encrypt(key_bits, permission, opassword, upassword, &raw mut p, endptr);
        } else if ns_dvipdfmx != 0 && streq_ptr(q, b"config\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0 {
            scan_special_config(&raw mut p, endptr, opt_flags, compression_level, annot_grow_x, annot_grow_y);
        } else if !has_id.is_null()
            && !id1.is_null()
            && !id2.is_null()
            && ns_pdf != 0
            && strcmp(q, b"trailerid\0" as *const u8 as *const ::core::ffi::c_char) == 0
        {
            error = scan_special_trailerid(id1, id2, &raw mut p, endptr);
            if error != 0 {
                pdf_warning!(b"Invalid argument for pdf:trailerid special.\0" as *const u8 as *const ::core::ffi::c_char,);
                *has_id = 0 as ::core::ffi::c_int;
            } else {
                *has_id = 1 as ::core::ffi::c_int;
            }
        }
        free(q as *mut ::core::ffi::c_void);
    }
    return error;
}

#[unsafe(export_name = "__pdf_impl_dvi_scan_specials")]
pub unsafe extern "C-unwind" fn dvi_scan_specials(
    mut page_no: ::core::ffi::c_int, mut page_width: *mut ::core::ffi::c_double, mut page_height: *mut ::core::ffi::c_double,
    mut x_offset: *mut ::core::ffi::c_double, mut y_offset: *mut ::core::ffi::c_double, mut landscape: *mut ::core::ffi::c_int,
    mut majorversion: *mut ::core::ffi::c_int, mut minorversion: *mut ::core::ffi::c_int, mut compression_level: *mut ::core::ffi::c_int,
    mut annot_grow_x: *mut ::core::ffi::c_double, mut annot_grow_y: *mut ::core::ffi::c_double, mut do_enc: *mut ::core::ffi::c_int,
    mut key_bits: *mut ::core::ffi::c_int, mut permission: *mut int32_t, mut owner_pw: *mut ::core::ffi::c_char,
    mut user_pw: *mut ::core::ffi::c_char, mut has_id: *mut ::core::ffi::c_int, mut id1: *mut ::core::ffi::c_uchar,
    mut id2: *mut ::core::ffi::c_uchar, mut opt_flags: *mut ::core::ffi::c_int,
) {
    let __pdf_state = unsafe { document_state() };
    let mut offset: uint32_t = 0;
    let mut opcode: ::core::ffi::c_uchar = 0;
    let mut len: ::core::ffi::c_uint = 0;
    if page_no == (*__pdf_state).buffered_page || (*__pdf_state).num_pages == 0 as ::core::ffi::c_uint {
        return;
    }
    (*__pdf_state).buffered_page = page_no;
    (*__pdf_state).dvi_page_buf_index = 0 as ::core::ffi::c_uint;
    if (*__pdf_state).linear == 0 {
        if page_no as ::core::ffi::c_uint >= (*__pdf_state).num_pages {
            pdf_abort!(b"Invalid page number: %u\0" as *const u8 as *const ::core::ffi::c_char, page_no,);
        }
        offset = *(*__pdf_state).page_loc.offset(page_no as isize);
        if (*__pdf_state).eligibility_only == 0 {
            let mut end: uint32_t = if ((page_no + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint) < (*__pdf_state).num_pages {
                *(*__pdf_state).page_loc.offset((page_no + 1 as ::core::ffi::c_int) as isize)
            } else {
                (*__pdf_state).dvi_file_size
            };
            let mut page: tt_pdf_scanned_page = tt_pdf_scanned_page {
                data: ::core::ptr::null::<::core::ffi::c_uchar>(),
                length: 0,
                specials: ::core::ptr::null::<tt_pdf_special_span>(),
                special_count: 0,
            };
            if end <= (*__pdf_state).dvi_file_size
                && tt_pdf_scan_page(
                    (*__pdf_state).dvi_handle,
                    offset,
                    end,
                    (dpx_conf.compat_mode as ::core::ffi::c_uint == dpx_mode_xdv_mode as ::core::ffi::c_int as ::core::ffi::c_uint)
                        as ::core::ffi::c_int,
                    &raw mut page,
                ) != 0
            {
                let mut i: size_t = 0;
                if page.length > UINT_MAX.wrapping_sub(DVI_PAGE_BUF_CHUNK) as size_t {
                    pdf_abort!(b"DVI page is too large!\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                if page.length >= (*__pdf_state).dvi_page_buf_size as size_t {
                    (*__pdf_state).dvi_page_buf_size = (page.length as ::core::ffi::c_uint).wrapping_add(DVI_PAGE_BUF_CHUNK);
                    (*__pdf_state).dvi_page_buffer = renew(
                        (*__pdf_state).dvi_page_buffer as *mut ::core::ffi::c_void,
                        ((*__pdf_state).dvi_page_buf_size as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
                            as uint32_t,
                    ) as *mut ::core::ffi::c_uchar;
                }
                memcpy(
                    (*__pdf_state).dvi_page_buffer as *mut ::core::ffi::c_void,
                    page.data as *const ::core::ffi::c_void,
                    page.length,
                );
                (*__pdf_state).dvi_page_buf_index = page.length as ::core::ffi::c_uint;
                i = 0 as size_t;
                while i < page.special_count {
                    let mut span: *const tt_pdf_special_span = page.specials.offset(i as isize) as *const tt_pdf_special_span;
                    let mut buf_0: *const ::core::ffi::c_char =
                        ((*__pdf_state).dvi_page_buffer as *const ::core::ffi::c_char).offset((*span).offset as isize);
                    if scan_special(
                        page_width,
                        page_height,
                        x_offset,
                        y_offset,
                        landscape,
                        majorversion,
                        minorversion,
                        compression_level,
                        annot_grow_x,
                        annot_grow_y,
                        do_enc,
                        key_bits,
                        permission,
                        owner_pw,
                        user_pw,
                        has_id,
                        id1,
                        id2,
                        opt_flags,
                        buf_0,
                        (*span).length as ::core::ffi::c_int as uint32_t,
                    ) != 0
                    {
                        pdf_warning!(
                            b"Reading special command failed: \"%.*s\"\0" as *const u8 as *const ::core::ffi::c_char,
                            (*span).length as ::core::ffi::c_int,
                            buf_0,
                        );
                    }
                    i = i.wrapping_add(1);
                }
                return;
            }
        }
        crate::backend::runtime::input_seek((*__pdf_state).dvi_handle, offset as ssize_t, SEEK_SET);
    }
    loop {
        opcode = get_and_buffer_unsigned_byte((*__pdf_state).dvi_handle) as ::core::ffi::c_uchar;
        if !(opcode as ::core::ffi::c_int != EOP) {
            break;
        }
        if opcode as ::core::ffi::c_int <= SET_CHAR_127 || opcode as ::core::ffi::c_int >= FNT_NUM_0 && opcode as ::core::ffi::c_int <= FNT_NUM_63 {
            continue;
        }
        if opcode as ::core::ffi::c_int == XXX1
            || opcode as ::core::ffi::c_int == XXX2
            || opcode as ::core::ffi::c_int == XXX3
            || opcode as ::core::ffi::c_int == XXX4
        {
            let mut size: uint32_t = get_and_buffer_unsigned_byte((*__pdf_state).dvi_handle) as uint32_t;
            let mut current_block_32: u64;
            match opcode as ::core::ffi::c_int {
                XXX4 => {
                    size = size
                        .wrapping_mul(0x100 as uint32_t)
                        .wrapping_add(get_and_buffer_unsigned_byte((*__pdf_state).dvi_handle) as uint32_t);
                    if size > 0x7fff as uint32_t {
                        pdf_warning!(
                            b"Unsigned number starting with %x exceeds 0x7fffffff\0" as *const u8 as *const ::core::ffi::c_char,
                            size,
                        );
                    }
                    current_block_32 = 9891201208829039439;
                }
                XXX3 => {
                    current_block_32 = 9891201208829039439;
                }
                XXX2 => {
                    current_block_32 = 13520245842808149009;
                }
                _ => {
                    current_block_32 = 18377268871191777778;
                }
            }
            match current_block_32 {
                9891201208829039439 => {
                    size = size
                        .wrapping_mul(0x100 as uint32_t)
                        .wrapping_add(get_and_buffer_unsigned_byte((*__pdf_state).dvi_handle) as uint32_t);
                    current_block_32 = 13520245842808149009;
                }
                _ => {}
            }
            match current_block_32 {
                13520245842808149009 => {
                    size = size
                        .wrapping_mul(0x100 as uint32_t)
                        .wrapping_add(get_and_buffer_unsigned_byte((*__pdf_state).dvi_handle) as uint32_t);
                }
                _ => {}
            }
            if ((*__pdf_state).dvi_page_buf_index as uint32_t).wrapping_add(size) >= (*__pdf_state).dvi_page_buf_size as uint32_t {
                (*__pdf_state).dvi_page_buf_size = ((*__pdf_state).dvi_page_buf_index as uint32_t)
                    .wrapping_add(size)
                    .wrapping_add(DVI_PAGE_BUF_CHUNK as uint32_t) as ::core::ffi::c_uint;
                (*__pdf_state).dvi_page_buffer = renew(
                    (*__pdf_state).dvi_page_buffer as *mut ::core::ffi::c_void,
                    ((*__pdf_state).dvi_page_buf_size as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
                        as uint32_t,
                ) as *mut ::core::ffi::c_uchar;
            }
            if ttstub_input_read(
                (*__pdf_state).dvi_handle,
                (*__pdf_state).dvi_page_buffer.offset((*__pdf_state).dvi_page_buf_index as isize) as *mut ::core::ffi::c_char,
                size as size_t,
            ) != size as ssize_t
            {
                pdf_abort!(b"Reading DVI file failed!\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            if (*__pdf_state).eligibility_only != 0 {
                if tt_pdf_special_supported(
                    (*__pdf_state).dvi_page_buffer.offset((*__pdf_state).dvi_page_buf_index as isize) as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_uchar,
                    size as size_t,
                ) == 0
                {
                    (*__pdf_state).parallel_eligible = 0 as ::core::ffi::c_int;
                }
            } else if scan_special(
                page_width,
                page_height,
                x_offset,
                y_offset,
                landscape,
                majorversion,
                minorversion,
                compression_level,
                annot_grow_x,
                annot_grow_y,
                do_enc,
                key_bits,
                permission,
                owner_pw,
                user_pw,
                has_id,
                id1,
                id2,
                opt_flags,
                (*__pdf_state).dvi_page_buffer.offset((*__pdf_state).dvi_page_buf_index as isize) as *mut ::core::ffi::c_char,
                size,
            ) != 0
            {
                pdf_warning!(
                    b"Reading special command failed: \"%.*s\"\0" as *const u8 as *const ::core::ffi::c_char,
                    size,
                    (*__pdf_state).dvi_page_buffer.offset((*__pdf_state).dvi_page_buf_index as isize) as *mut ::core::ffi::c_char,
                );
            }
            (*__pdf_state).dvi_page_buf_index = (*__pdf_state).dvi_page_buf_index.wrapping_add(size as ::core::ffi::c_uint);
        } else {
            let mut current_block_72: u64;
            match opcode as ::core::ffi::c_int {
                BOP => {
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, 44 as ::core::ffi::c_uint);
                    current_block_72 = 1209030638129645089;
                }
                NOP | PUSH | POP | W0 | X0 | Y0 | Z0 => {
                    current_block_72 = 1209030638129645089;
                }
                SET1 | PUT1 | RIGHT1 | DOWN1 | W1 | X1 | Y1 | Z1 | FNT1 => {
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, 1 as ::core::ffi::c_uint);
                    current_block_72 = 1209030638129645089;
                }
                SET2 | PUT2 | RIGHT2 | DOWN2 | W2 | X2 | Y2 | Z2 | FNT2 => {
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, 2 as ::core::ffi::c_uint);
                    current_block_72 = 1209030638129645089;
                }
                SET3 | PUT3 | RIGHT3 | DOWN3 | W3 | X3 | Y3 | Z3 | FNT3 => {
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, 3 as ::core::ffi::c_uint);
                    current_block_72 = 1209030638129645089;
                }
                SET4 | PUT4 | RIGHT4 | DOWN4 | W4 | X4 | Y4 | Z4 | FNT4 => {
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, 4 as ::core::ffi::c_uint);
                    current_block_72 = 1209030638129645089;
                }
                SET_RULE | PUT_RULE => {
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, 8 as ::core::ffi::c_uint);
                    current_block_72 = 1209030638129645089;
                }
                FNT_DEF1 | FNT_DEF2 | FNT_DEF3 | FNT_DEF4 => {
                    do_fntdef(tt_get_unsigned_num(
                        (*__pdf_state).dvi_handle,
                        (opcode as ::core::ffi::c_int - FNT_DEF1) as ::core::ffi::c_uchar,
                    ));
                    current_block_72 = 1209030638129645089;
                }
                XDV_GLYPHS => {
                    need_XeTeX(opcode as ::core::ffi::c_int);
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, 4 as ::core::ffi::c_uint);
                    len = get_and_buffer_unsigned_pair((*__pdf_state).dvi_handle);
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, len.wrapping_mul(10 as ::core::ffi::c_uint));
                    current_block_72 = 1209030638129645089;
                }
                XDV_TEXT_AND_GLYPHS => {
                    need_XeTeX(opcode as ::core::ffi::c_int);
                    len = get_and_buffer_unsigned_pair((*__pdf_state).dvi_handle);
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, len.wrapping_mul(2 as ::core::ffi::c_uint));
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, 4 as ::core::ffi::c_uint);
                    len = get_and_buffer_unsigned_pair((*__pdf_state).dvi_handle);
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, len.wrapping_mul(10 as ::core::ffi::c_uint));
                    current_block_72 = 1209030638129645089;
                }
                XDV_NATIVE_FONT_DEF => {
                    need_XeTeX(opcode as ::core::ffi::c_int);
                    do_native_font_def(tt_get_signed_quad((*__pdf_state).dvi_handle));
                    current_block_72 = 1209030638129645089;
                }
                BEGIN_REFLECT | END_REFLECT => {
                    need_XeTeX(opcode as ::core::ffi::c_int);
                    current_block_72 = 1209030638129645089;
                }
                PTEXDIR => {
                    need_pTeX(opcode as ::core::ffi::c_int);
                    get_and_buffer_bytes((*__pdf_state).dvi_handle, 1 as ::core::ffi::c_uint);
                    current_block_72 = 1209030638129645089;
                }
                POST => {
                    if (*__pdf_state).linear as ::core::ffi::c_int != 0 && (*__pdf_state).dvi_page_buf_index == 1 as ::core::ffi::c_uint {
                        return;
                    }
                    current_block_72 = 1945552802878553076;
                }
                _ => {
                    current_block_72 = 1945552802878553076;
                }
            }
            match current_block_72 {
                1945552802878553076 => {
                    pdf_abort!(
                        b"Unexpected opcode %d\0" as *const u8 as *const ::core::ffi::c_char,
                        opcode as ::core::ffi::c_int,
                    );
                }
                _ => {}
            }
        }
    }
}
#[unsafe(export_name = "__pdf_impl_dvi_parallel_eligible")]
pub unsafe extern "C-unwind" fn dvi_parallel_eligible() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut i: ::core::ffi::c_uint = 0;
    let mut result: ::core::ffi::c_int = 0;
    if (*__pdf_state).linear != 0 {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_uint;
    while i < (*__pdf_state).num_def_fonts {
        if (*(*__pdf_state).def_fonts.offset(i as isize)).native == 0 {
            let mut vf: rust_input_handle_t = ttstub_input_open(
                (*(*__pdf_state).def_fonts.offset(i as isize)).font_name,
                TTBC_FILE_FORMAT_VF,
                0 as ::core::ffi::c_int,
            );
            if vf != INVALID_HANDLE {
                crate::backend::runtime::input_close(vf);
                return 0 as ::core::ffi::c_int;
            }
        }
        i = i.wrapping_add(1);
    }
    result = tt_pdf_preflight_pages(
        (*__pdf_state).dvi_handle,
        (*__pdf_state).page_loc,
        (*__pdf_state).num_pages as size_t,
        (*__pdf_state).dvi_file_size,
        (dpx_conf.compat_mode as ::core::ffi::c_uint == dpx_mode_xdv_mode as ::core::ffi::c_int as ::core::ffi::c_uint) as ::core::ffi::c_int,
    );
    if result >= 0 as ::core::ffi::c_int {
        (*__pdf_state).buffered_page = -(1 as ::core::ffi::c_int);
        return result;
    }
    (*__pdf_state).eligibility_only = 1 as ::core::ffi::c_int;
    (*__pdf_state).parallel_eligible = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_uint;
    while i < (*__pdf_state).num_pages && (*__pdf_state).parallel_eligible != 0 {
        dvi_scan_specials(
            i as ::core::ffi::c_int,
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
            ::core::ptr::null_mut::<::core::ffi::c_double>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<int32_t>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        i = i.wrapping_add(1);
    }
    (*__pdf_state).eligibility_only = 0 as ::core::ffi::c_int;
    (*__pdf_state).buffered_page = -(1 as ::core::ffi::c_int);
    return (*__pdf_state).parallel_eligible;
}
#[unsafe(export_name = "__pdf_impl_dvi_reset_global_state")]
pub unsafe extern "C-unwind" fn dvi_reset_global_state() {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).buffered_page = -(1 as ::core::ffi::c_int);
    (*__pdf_state).eligibility_only = 0 as ::core::ffi::c_int;
    (*__pdf_state).parallel_eligible = 1 as ::core::ffi::c_int;
    (*__pdf_state).processing_page = 0 as ::core::ffi::c_int;
    (*__pdf_state).num_def_fonts = 0 as ::core::ffi::c_uint;
    (*__pdf_state).max_def_fonts = 0 as ::core::ffi::c_uint;
    (*__pdf_state).compute_boxes = 0 as ::core::ffi::c_int;
    (*__pdf_state).link_annot = 1 as ::core::ffi::c_int;
    (*__pdf_state).num_loaded_fonts = 0 as ::core::ffi::c_uint;
    (*__pdf_state).max_loaded_fonts = 0 as ::core::ffi::c_uint;
}
pub const UINT_MAX: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
unsafe extern "C" {
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
    fn memset(__b: *mut ::core::ffi::c_void, __c: ::core::ffi::c_int, __len: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct __sFILEX {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pdf_file {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_get_origin(mut x: ::core::ffi::c_int, _result: *mut ::core::ffi::c_double) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_origin(x) }) {
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
pub unsafe extern "C" fn __pdf_entry_dvi_npages(_result: *mut ::core::ffi::c_uint) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_npages() }) {
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
pub unsafe extern "C" fn __pdf_entry_dvi_tell_mag(_result: *mut ::core::ffi::c_double) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_tell_mag() }) {
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
pub unsafe extern "C" fn __pdf_entry_dvi_comment(_result: *mut *const ::core::ffi::c_char) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_comment() }) {
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
pub unsafe extern "C" fn __pdf_entry_dvi_tag_depth() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_tag_depth() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_untag_depth() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_untag_depth() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_compute_boxes(mut flag: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_compute_boxes(flag) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_link_annot(mut flag: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_link_annot(flag) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_is_tracking_boxes(_result: *mut bool) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_is_tracking_boxes() }) {
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
pub unsafe extern "C" fn __pdf_entry_dvi_set_linkmode(mut mode: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_set_linkmode(mode) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_set_phantom_height(mut height: ::core::ffi::c_double, mut depth: ::core::ffi::c_double) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_set_phantom_height(height, depth) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_set_compensation(mut x: ::core::ffi::c_double, mut y: ::core::ffi::c_double) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_set_compensation(x, y) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_do_special(mut buffer: *const ::core::ffi::c_void, mut size: int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_do_special(buffer, size) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_unit_size(_result: *mut ::core::ffi::c_double) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_unit_size() }) {
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
pub unsafe extern "C" fn __pdf_entry_dvi_locate_font(
    mut tfm_name: *const ::core::ffi::c_char, mut ptsize: spt_t, _result: *mut ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_locate_font(tfm_name, ptsize) }) {
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
pub unsafe extern "C" fn __pdf_entry_dvi_dev_xpos(_result: *mut ::core::ffi::c_double) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_dev_xpos() }) {
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
pub unsafe extern "C" fn __pdf_entry_dvi_dev_ypos(_result: *mut ::core::ffi::c_double) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_dev_ypos() }) {
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
pub unsafe extern "C" fn __pdf_entry_dvi_right(mut x: int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_right(x) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_down(mut y: int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_down(y) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_set(mut ch: int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_set(ch) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_put(mut ch: int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_put(ch) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_rule(mut width: int32_t, mut height: int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_rule(width, height) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_dirchg(mut dir: ::core::ffi::c_uchar) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_dirchg(dir) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_push() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_push() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dpx_dvi_pop() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_dvi_pop() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_w(mut ch: int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_w(ch) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_w0() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_w0() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_x(mut ch: int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_x(ch) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_x0() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_x0() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_y(mut ch: int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_y(ch) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_y0() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_y0() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_z(mut ch: int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_z(ch) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_z0() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_z0() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_set_font(mut font_id: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_set_font(font_id) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_do_page(
    mut page_paper_height: ::core::ffi::c_double, mut hmargin: ::core::ffi::c_double, mut vmargin: ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_do_page(page_paper_height, hmargin, vmargin) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_init(
    mut dvi_filename: *const ::core::ffi::c_char, mut mag: ::core::ffi::c_double, _result: *mut ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_init(dvi_filename, mag) }) {
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
pub unsafe extern "C" fn __pdf_entry_dvi_close() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_close() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_vf_init(mut dev_font_id: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_vf_init(dev_font_id) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_vf_finish() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_vf_finish() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_scan_specials(
    mut page_no: ::core::ffi::c_int, mut page_width: *mut ::core::ffi::c_double, mut page_height: *mut ::core::ffi::c_double,
    mut x_offset: *mut ::core::ffi::c_double, mut y_offset: *mut ::core::ffi::c_double, mut landscape: *mut ::core::ffi::c_int,
    mut majorversion: *mut ::core::ffi::c_int, mut minorversion: *mut ::core::ffi::c_int, mut compression_level: *mut ::core::ffi::c_int,
    mut annot_grow_x: *mut ::core::ffi::c_double, mut annot_grow_y: *mut ::core::ffi::c_double, mut do_enc: *mut ::core::ffi::c_int,
    mut key_bits: *mut ::core::ffi::c_int, mut permission: *mut int32_t, mut owner_pw: *mut ::core::ffi::c_char,
    mut user_pw: *mut ::core::ffi::c_char, mut has_id: *mut ::core::ffi::c_int, mut id1: *mut ::core::ffi::c_uchar,
    mut id2: *mut ::core::ffi::c_uchar, mut opt_flags: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        dvi_scan_specials(
            page_no,
            page_width,
            page_height,
            x_offset,
            y_offset,
            landscape,
            majorversion,
            minorversion,
            compression_level,
            annot_grow_x,
            annot_grow_y,
            do_enc,
            key_bits,
            permission,
            owner_pw,
            user_pw,
            has_id,
            id1,
            id2,
            opt_flags,
        )
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dvi_parallel_eligible(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_parallel_eligible() }) {
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
pub unsafe extern "C" fn __pdf_entry_dvi_reset_global_state() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dvi_reset_global_state() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}

pub(crate) struct DocumentState {
    dvi_handle: rust_input_handle_t,
    linear: ::core::ffi::c_char,
    page_loc: *mut uint32_t,
    num_pages: ::core::ffi::c_uint,
    dvi_file_size: uint32_t,
    dvi_info: dvi_header,
    dev_origin_y: ::core::ffi::c_double,
    dev_origin_x: ::core::ffi::c_double,
    lr_state: dvi_lr,
    lr_mode: ::core::ffi::c_int,
    lr_width: uint32_t,
    lr_width_stack: [uint32_t; 256],
    lr_width_stack_depth: ::core::ffi::c_uint,
    loaded_fonts: *mut loaded_font,
    max_loaded_fonts: ::core::ffi::c_uint,
    num_loaded_fonts: ::core::ffi::c_uint,
    def_fonts: *mut font_def,
    num_def_fonts: ::core::ffi::c_uint,
    max_def_fonts: ::core::ffi::c_uint,
    compute_boxes: ::core::ffi::c_int,
    link_annot: ::core::ffi::c_int,
    catch_phantom: ::core::ffi::c_int,
    phantom_height: ::core::ffi::c_double,
    phantom_depth: ::core::ffi::c_double,
    dvi_page_buffer: *mut ::core::ffi::c_uchar,
    dvi_page_buf_size: ::core::ffi::c_uint,
    dvi_page_buf_index: ::core::ffi::c_uint,
    post_id_byte: ::core::ffi::c_int,
    pre_id_byte: ::core::ffi::c_int,
    is_ptex: ::core::ffi::c_int,
    has_ptex: ::core::ffi::c_int,
    total_mag: ::core::ffi::c_double,
    dvi2pts: ::core::ffi::c_double,
    dvi_state: dvi_registers,
    dvi_stack: [dvi_registers; 256],
    dvi_stack_depth: ::core::ffi::c_int,
    current_font: ::core::ffi::c_int,
    processing_page: ::core::ffi::c_int,
    marked_depth: ::core::ffi::c_int,
    tagged_depth: ::core::ffi::c_int,
    compensation: spt_coord,
    saved_dvi_font: [::core::ffi::c_int; 16],
    num_saved_fonts: ::core::ffi::c_uint,
    buffered_page: ::core::ffi::c_int,
    eligibility_only: ::core::ffi::c_int,
    parallel_eligible: ::core::ffi::c_int,
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        unsafe {
            Self {
                dvi_handle: INVALID_HANDLE,
                linear: 0 as ::core::ffi::c_char,
                page_loc: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
                num_pages: 0 as ::core::ffi::c_uint,
                dvi_file_size: 0 as uint32_t,
                dvi_info: dvi_header {
                    unit_num: 25400000 as uint32_t,
                    unit_den: 473628672 as uint32_t,
                    mag: 1000 as uint32_t,
                    media_width: 0 as uint32_t,
                    media_height: 0 as uint32_t,
                    stackdepth: 0 as ::core::ffi::c_uint,
                    comment: [
                        '\0' as i32 as ::core::ffi::c_char,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                },
                dev_origin_y: 770.0f64,
                dev_origin_x: 72.0f64,
                lr_state: dvi_lr {
                    state: 0,
                    font: 0,
                    buf_index: 0,
                },
                lr_mode: 0,
                lr_width: 0,
                lr_width_stack: [0; 256],
                lr_width_stack_depth: 0 as ::core::ffi::c_uint,
                loaded_fonts: ::core::ptr::null::<loaded_font>() as *mut loaded_font,
                max_loaded_fonts: 0 as ::core::ffi::c_uint,
                num_loaded_fonts: 0 as ::core::ffi::c_uint,
                def_fonts: ::core::ptr::null::<font_def>() as *mut font_def,
                num_def_fonts: 0 as ::core::ffi::c_uint,
                max_def_fonts: 0 as ::core::ffi::c_uint,
                compute_boxes: 0 as ::core::ffi::c_int,
                link_annot: 1 as ::core::ffi::c_int,
                catch_phantom: 0 as ::core::ffi::c_int,
                phantom_height: 0.0f64,
                phantom_depth: 0.0f64,
                dvi_page_buffer: ::core::ptr::null::<::core::ffi::c_uchar>() as *mut ::core::ffi::c_uchar,
                dvi_page_buf_size: 0,
                dvi_page_buf_index: 0,
                post_id_byte: 0,
                pre_id_byte: 0,
                is_ptex: 0 as ::core::ffi::c_int,
                has_ptex: 0 as ::core::ffi::c_int,
                total_mag: 1.0f64,
                dvi2pts: 1.52018f64,
                dvi_state: dvi_registers {
                    h: 0,
                    v: 0,
                    w: 0,
                    x: 0,
                    y: 0,
                    z: 0,
                    d: 0,
                },
                dvi_stack: [dvi_registers {
                    h: 0,
                    v: 0,
                    w: 0,
                    x: 0,
                    y: 0,
                    z: 0,
                    d: 0,
                }; 256],
                dvi_stack_depth: 0 as ::core::ffi::c_int,
                current_font: -(1 as ::core::ffi::c_int),
                processing_page: 0 as ::core::ffi::c_int,
                marked_depth: 0 as ::core::ffi::c_int,
                tagged_depth: -(1 as ::core::ffi::c_int),
                compensation: spt_coord {
                    x: 0 as spt_t,
                    y: 0 as spt_t,
                },
                saved_dvi_font: [0; 16],
                num_saved_fonts: 0 as ::core::ffi::c_uint,
                buffered_page: -(1 as ::core::ffi::c_int),
                eligibility_only: 0 as ::core::ffi::c_int,
                parallel_eligible: 1 as ::core::ffi::c_int,
            }
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe { (*crate::compat::document::current()).backend.dpx_dvi.pointer }
}
