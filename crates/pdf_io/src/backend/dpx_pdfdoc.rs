/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2008-2019 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
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
#![allow(
    warnings,
    clippy::all,
    unsafe_op_in_unsafe_fn,
    static_mut_refs,
    dangerous_implicit_autorefs
)]
use crate::backend::dpx_pdfdev::pdf_dev_get_param;
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strncpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
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
    #[link_name = "__pdf_impl_pdf_out_init"]
    fn pdf_out_init(
        filename: *const ::core::ffi::c_char,
        id1: *const ::core::ffi::c_uchar,
        id2: *const ::core::ffi::c_uchar,
        ver_major: ::core::ffi::c_int,
        ver_minor: ::core::ffi::c_int,
        compression_level: ::core::ffi::c_int,
        enable_encrypt: ::core::ffi::c_int,
        enable_objstm: ::core::ffi::c_int,
        enable_predictor: ::core::ffi::c_int,
    ) -> *mut pdf_out;
    #[link_name = "__pdf_impl_pdf_out_set_encrypt"]
    fn pdf_out_set_encrypt(
        keybits: ::core::ffi::c_int,
        permission: int32_t,
        opasswd: *const ::core::ffi::c_char,
        upasswd: *const ::core::ffi::c_char,
        use_aes: ::core::ffi::c_int,
        encrypt_metadata: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_out_flush"]
    fn pdf_out_flush();
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ref_obj"]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_link_obj"]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_number_value"]
    fn pdf_number_value(number: *mut pdf_obj) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pdf_new_string"]
    fn pdf_new_string(str: *const ::core::ffi::c_void, length: size_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_string_value"]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_string_length"]
    fn pdf_string_length(object: *mut pdf_obj) -> ::core::ffi::c_uint;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_name_value"]
    fn pdf_name_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_get_array"]
    fn pdf_get_array(array: *mut pdf_obj, idx: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_array_length"]
    fn pdf_array_length(array: *mut pdf_obj) -> ::core::ffi::c_uint;
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_remove_dict"]
    fn pdf_remove_dict(dict: *mut pdf_obj, key: *const ::core::ffi::c_char);
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
    #[link_name = "__pdf_impl_pdf_compare_reference"]
    fn pdf_compare_reference(ref1: *mut pdf_obj, ref2: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_set_info"]
    fn pdf_set_info(obj: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_set_root"]
    fn pdf_set_root(obj: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_files_init"]
    fn pdf_files_init();
    #[link_name = "__pdf_impl_pdf_files_close"]
    fn pdf_files_close();
    #[link_name = "__pdf_impl_pdf_submit_page"]
    fn pdf_submit_page(contents: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_file_get_catalog"]
    fn pdf_file_get_catalog(pf: *mut pdf_file) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_deref_obj"]
    fn pdf_deref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_color_graycolor"]
    fn pdf_color_graycolor(color: *mut pdf_color, g: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_copycolor"]
    fn pdf_color_copycolor(color1: *mut pdf_color, color2: *const pdf_color);
    #[link_name = "__pdf_impl_pdf_color_is_white"]
    fn pdf_color_is_white(color: *const pdf_color) -> bool;
    #[link_name = "__pdf_impl_pdf_init_colors"]
    fn pdf_init_colors();
    #[link_name = "__pdf_impl_pdf_close_colors"]
    fn pdf_close_colors();
    #[link_name = "__pdf_impl_pdf_dev_begin_page_plan"]
    fn pdf_dev_begin_page_plan();
    #[link_name = "__pdf_impl_pdf_dev_end_page_plan"]
    fn pdf_dev_end_page_plan();
    #[link_name = "__pdf_impl_pdf_init_device"]
    fn pdf_init_device(
        unit_conv: ::core::ffi::c_double,
        precision: ::core::ffi::c_int,
        is_bw: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_close_device"]
    fn pdf_close_device();
    #[link_name = "__pdf_impl_pdf_dev_reset_fonts"]
    fn pdf_dev_reset_fonts(newpage: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_dev_reset_color"]
    fn pdf_dev_reset_color(force: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_dev_bop"]
    fn pdf_dev_bop(M: *const pdf_tmatrix);
    #[link_name = "__pdf_impl_pdf_dev_eop"]
    fn pdf_dev_eop();
    static mut dpx_conf: _dpx_conf;
    #[link_name = "__pdf_impl_dpx_util_format_asn_date"]
    fn dpx_util_format_asn_date(
        date_string: *mut ::core::ffi::c_char,
        need_timezone: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_ht_init_table"]
    fn ht_init_table(ht: *mut ht_table, hval_free_fn: hval_free_func);
    #[link_name = "__pdf_impl_ht_clear_table"]
    fn ht_clear_table(ht: *mut ht_table);
    #[link_name = "__pdf_impl_ht_table_size"]
    fn ht_table_size(ht: *mut ht_table) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_ht_lookup_table"]
    fn ht_lookup_table(
        ht: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_ht_append_table"]
    fn ht_append_table(
        ht: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
        value: *mut ::core::ffi::c_void,
    );
    #[link_name = "__pdf_impl_ht_set_iter"]
    fn ht_set_iter(ht: *mut ht_table, iter: *mut ht_iter) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_ht_clear_iter"]
    fn ht_clear_iter(iter: *mut ht_iter);
    #[link_name = "__pdf_impl_ht_iter_getkey"]
    fn ht_iter_getkey(
        iter: *mut ht_iter,
        keylen: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_ht_iter_next"]
    fn ht_iter_next(iter: *mut ht_iter) -> ::core::ffi::c_int;
    static mut ttpi_source_date_epoch: time_t;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_pdf_init_images"]
    fn pdf_init_images();
    #[link_name = "__pdf_impl_pdf_close_images"]
    fn pdf_close_images();
    #[link_name = "__pdf_impl_pdf_ximage_get_reference"]
    fn pdf_ximage_get_reference(xobj_id: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_ximage_load_image"]
    fn pdf_ximage_load_image(
        ident: *const ::core::ffi::c_char,
        filename: *const ::core::ffi::c_char,
        options: load_options,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ximage_defineresource"]
    fn pdf_ximage_defineresource(
        ident: *const ::core::ffi::c_char,
        subtype: ::core::ffi::c_int,
        cdata: *mut ::core::ffi::c_void,
        resource: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ximage_init_form_info"]
    fn pdf_ximage_init_form_info(info: *mut xform_info);
    #[link_name = "__pdf_impl_check_for_jpeg"]
    fn check_for_jpeg(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_dev_rectfill"]
    fn pdf_dev_rectfill(
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        w: ::core::ffi::c_double,
        h: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_gsave"]
    fn pdf_dev_gsave() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_grestore"]
    fn pdf_dev_grestore() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_push_gstate"]
    fn pdf_dev_push_gstate() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_pop_gstate"]
    fn pdf_dev_pop_gstate() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_current_depth"]
    fn pdf_dev_current_depth() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_grestore_to"]
    fn pdf_dev_grestore_to(depth: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_dev_set_color"]
    fn pdf_dev_set_color(
        color: *const pdf_color,
        mask: ::core::ffi::c_char,
        force: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_dev_reset_xgstate"]
    fn pdf_dev_reset_xgstate(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_init_fonts"]
    fn pdf_init_fonts();
    #[link_name = "__pdf_impl_pdf_close_fonts"]
    fn pdf_close_fonts();
    #[link_name = "__pdf_impl_pdf_new_name_tree"]
    fn pdf_new_name_tree() -> *mut ht_table;
    #[link_name = "__pdf_impl_pdf_delete_name_tree"]
    fn pdf_delete_name_tree(names: *mut *mut ht_table);
    #[link_name = "__pdf_impl_pdf_names_add_object"]
    fn pdf_names_add_object(
        names: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
        object: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_names_create_tree"]
    fn pdf_names_create_tree(
        names: *mut ht_table,
        count: *mut ::core::ffi::c_int,
        filter: *mut ht_table,
    ) -> *mut pdf_obj;
    fn tt_pdf_page_recording() -> ::core::ffi::c_int;
    fn tt_pdf_page_bytes(data: *const ::core::ffi::c_char, length: size_t) -> ::core::ffi::c_int;
    fn tt_pdf_page_size() -> size_t;
    fn tt_pdf_pipeline_has_capacity(bytes: size_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_pipeline_drain"]
    fn pdf_pipeline_drain(all: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_init_resources"]
    fn pdf_init_resources();
    #[link_name = "__pdf_impl_pdf_close_resources"]
    fn pdf_close_resources();
    #[link_name = "__pdf_impl_check_for_png"]
    fn check_for_png(handle: rust_input_handle_t) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type __darwin_time_t = ::core::ffi::c_long;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type time_t = libc::time_t;
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
pub struct pdf_color {
    pub res_id: ::core::ffi::c_int,
    pub type_0: ::core::ffi::c_int,
    pub num_components: ::core::ffi::c_int,
    pub spot_color_name: *mut ::core::ffi::c_char,
    pub values: [::core::ffi::c_double; 32],
    pub pattern_id: ::core::ffi::c_int,
}
use crate::compat::renderer::pdf_rect;
use crate::compat::renderer::pdf_tmatrix;
pub type pdf_page_boundary = ::core::ffi::c_uint;
pub const pdf_page_boundary_bleedbox: pdf_page_boundary = 5;
pub const pdf_page_boundary_trimbox: pdf_page_boundary = 4;
pub const pdf_page_boundary_artbox: pdf_page_boundary = 3;
pub const pdf_page_boundary_cropbox: pdf_page_boundary = 1;
pub const pdf_page_boundary_mediabox: pdf_page_boundary = 2;
pub const pdf_page_boundary__auto: pdf_page_boundary = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_dev_setting {
    pub dvi2pts: ::core::ffi::c_double,
    pub precision: ::core::ffi::c_int,
    pub ignore_colors: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_enc_setting {
    pub key_size: ::core::ffi::c_int,
    pub permission: uint32_t,
    pub uplain: *const ::core::ffi::c_char,
    pub oplain: *const ::core::ffi::c_char,
    pub use_aes: ::core::ffi::c_int,
    pub encrypt_metadata: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_obj_setting {
    pub enable_objstm: ::core::ffi::c_int,
    pub enable_predictor: ::core::ffi::c_int,
    pub compression_level: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_setting {
    pub ver_major: ::core::ffi::c_int,
    pub ver_minor: ::core::ffi::c_int,
    pub media_width: ::core::ffi::c_double,
    pub media_height: ::core::ffi::c_double,
    pub annot_grow_amount: C2RustUnnamed,
    pub outline_open_depth: ::core::ffi::c_int,
    pub check_gotos: ::core::ffi::c_int,
    pub enable_manual_thumb: ::core::ffi::c_int,
    pub enable_encrypt: ::core::ffi::c_int,
    pub encrypt: pdf_enc_setting,
    pub device: pdf_dev_setting,
    pub object: pdf_obj_setting,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_table {
    pub count: ::core::ffi::c_int,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut ::core::ffi::c_char,
    pub keylen: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct form_list_node {
    pub q_depth: ::core::ffi::c_int,
    pub form: pdf_form,
    pub prev: *mut form_list_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_form {
    pub ident: *mut ::core::ffi::c_char,
    pub matrix: pdf_tmatrix,
    pub cropbox: pdf_rect,
    pub resources: *mut pdf_obj,
    pub contents: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_doc {
    pub root: C2RustUnnamed_5,
    pub info: *mut pdf_obj,
    pub pages: C2RustUnnamed_4,
    pub outlines: C2RustUnnamed_3,
    pub articles: C2RustUnnamed_2,
    pub names: *mut name_dict,
    pub check_gotos: ::core::ffi::c_int,
    pub gotos: ht_table,
    pub options: C2RustUnnamed_0,
    pub pending_forms: *mut form_list_node,
    pub thumb_basename: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub outline_open_depth: ::core::ffi::c_int,
    pub annot_grow: C2RustUnnamed_1,
    pub enable_manual_thumb: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_dict {
    pub category: *const ::core::ffi::c_char,
    pub data: *mut ht_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub num_entries: ::core::ffi::c_uint,
    pub max_entries: ::core::ffi::c_uint,
    pub entries: *mut pdf_article,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_article {
    pub id: *mut ::core::ffi::c_char,
    pub info: *mut pdf_obj,
    pub num_beads: ::core::ffi::c_uint,
    pub max_beads: ::core::ffi::c_uint,
    pub beads: *mut pdf_bead,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_bead {
    pub id: *mut ::core::ffi::c_char,
    pub page_no: ::core::ffi::c_int,
    pub rect: pdf_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub first: *mut pdf_olitem,
    pub current: *mut pdf_olitem,
    pub current_depth: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_olitem {
    pub dict: *mut pdf_obj,
    pub is_open: ::core::ffi::c_int,
    pub first: *mut pdf_olitem,
    pub parent: *mut pdf_olitem,
    pub next: *mut pdf_olitem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub mediabox: pdf_rect,
    pub bop: *mut pdf_obj,
    pub eop: *mut pdf_obj,
    pub num_entries: ::core::ffi::c_uint,
    pub max_entries: ::core::ffi::c_uint,
    pub entries: *mut pdf_page,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_page {
    pub page_obj: *mut pdf_obj,
    pub page_ref: *mut pdf_obj,
    pub flags: ::core::ffi::c_int,
    pub ref_x: ::core::ffi::c_double,
    pub ref_y: ::core::ffi::c_double,
    pub cropbox: pdf_rect,
    pub resources: *mut pdf_obj,
    pub background: *mut pdf_obj,
    pub contents: *mut pdf_obj,
    pub content_refs: [*mut pdf_obj; 4],
    pub render_pending: ::core::ffi::c_int,
    pub annots: *mut pdf_obj,
    pub beads: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub dict: *mut pdf_obj,
    pub viewerpref: *mut pdf_obj,
    pub pagelabels: *mut pdf_obj,
    pub pages: *mut pdf_obj,
    pub names: *mut pdf_obj,
    pub threads: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_iter {
    pub index: ::core::ffi::c_int,
    pub curr: *mut ::core::ffi::c_void,
    pub hash: *mut ht_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub keep_cache: ::core::ffi::c_int,
}
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_options {
    pub page_no: ::core::ffi::c_int,
    pub bbox_type: pdf_page_boundary,
    pub dict: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xform_info {
    pub flags: ::core::ffi::c_int,
    pub bbox: pdf_rect,
    pub matrix: pdf_tmatrix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub dirty: ::core::ffi::c_int,
    pub broken: ::core::ffi::c_int,
    pub annot_dict: *mut pdf_obj,
    pub rect: pdf_rect,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
#[inline]
unsafe extern "C-unwind" fn release_name_object(value: *mut ::core::ffi::c_void) {
    unsafe { pdf_release_obj(value.cast()) };
}

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
pub const PDF_NUMBER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_STRING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PDF_NAME: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PDF_ARRAY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PDF_DICT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PDF_INDIRECT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const PDF_UNDEFINED: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const STREAM_PAGE_CONTENT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const PDF_OBJ_MAX_DEPTH: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_GRAY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const PDF_DEV_PARAM_COLORMODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_XOBJECT_TYPE_FORM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PDFDOC_PAGES_ALLOC_SIZE: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
pub const PDFDOC_ARTICLE_ALLOC_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PDFDOC_BEAD_ALLOC_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn read_thumbnail(
    mut thumb_filename: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut image_ref: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut xobj_id: ::core::ffi::c_int = 0;
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    let mut options: load_options = load_options {
        page_no: 1 as ::core::ffi::c_int,
        bbox_type: pdf_page_boundary__auto,
        dict: ::core::ptr::null_mut::<pdf_obj>(),
    };
    handle = ttstub_input_open(
        thumb_filename,
        TTBC_FILE_FORMAT_PICT,
        0 as ::core::ffi::c_int,
    );
    if handle == INVALID_HANDLE {
        pdf_warning!(
            b"Could not open thumbnail file \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            thumb_filename,
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if check_for_png(handle) == 0 && check_for_jpeg(handle) == 0 {
        pdf_warning!(
            b"Thumbnail \"%s\" not a png/jpeg file!\0" as *const u8 as *const ::core::ffi::c_char,
            thumb_filename,
        );
        crate::backend::runtime::input_close(handle);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    crate::backend::runtime::input_close(handle);
    xobj_id = pdf_ximage_load_image(thumb_filename, thumb_filename, options);
    if xobj_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Could not read thumbnail file \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
            thumb_filename,
        );
        image_ref = ::core::ptr::null_mut::<pdf_obj>();
    } else {
        image_ref = pdf_ximage_get_reference(xobj_id);
    }
    return image_ref;
}
#[unsafe(no_mangle)]
pub static mut global_names: *mut ht_table = ::core::ptr::null::<ht_table>() as *mut ht_table;
pub const USE_MY_MEDIABOX: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;

unsafe extern "C-unwind" fn pdf_doc_init_catalog(mut p: *mut pdf_doc) {
    (*p).root.viewerpref = ::core::ptr::null_mut::<pdf_obj>();
    (*p).root.pagelabels = ::core::ptr::null_mut::<pdf_obj>();
    (*p).root.pages = ::core::ptr::null_mut::<pdf_obj>();
    (*p).root.names = ::core::ptr::null_mut::<pdf_obj>();
    (*p).root.threads = ::core::ptr::null_mut::<pdf_obj>();
    (*p).root.dict = pdf_new_dict();
    pdf_set_root((*p).root.dict);
}
unsafe extern "C-unwind" fn pdf_doc_close_catalog(mut p: *mut pdf_doc) {
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if !(*p).root.viewerpref.is_null() {
        tmp = pdf_lookup_dict(
            (*p).root.dict,
            b"ViewerPreferences\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if tmp.is_null() {
            pdf_add_dict(
                (*p).root.dict,
                pdf_new_name(b"ViewerPreferences\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_ref_obj((*p).root.viewerpref),
            );
        } else if !tmp.is_null() && pdf_obj_typeof(tmp) == PDF_DICT {
            pdf_merge_dict((*p).root.viewerpref, tmp);
            pdf_add_dict(
                (*p).root.dict,
                pdf_new_name(b"ViewerPreferences\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_ref_obj((*p).root.viewerpref),
            );
        } else {
            pdf_warning!(
                b"Could not modify ViewerPreferences.\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        pdf_release_obj((*p).root.viewerpref);
        (*p).root.viewerpref = ::core::ptr::null_mut::<pdf_obj>();
    }
    if !(*p).root.pagelabels.is_null() {
        tmp = pdf_lookup_dict(
            (*p).root.dict,
            b"PageLabels\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if tmp.is_null() {
            tmp = pdf_new_dict();
            pdf_add_dict(
                tmp,
                pdf_new_name(b"Nums\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_link_obj((*p).root.pagelabels),
            );
            pdf_add_dict(
                (*p).root.dict,
                pdf_new_name(b"PageLabels\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_ref_obj(tmp),
            );
            pdf_release_obj(tmp);
        } else {
            pdf_warning!(
                b"Could not modify PageLabels.\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        pdf_release_obj((*p).root.pagelabels);
        (*p).root.pagelabels = ::core::ptr::null_mut::<pdf_obj>();
    }
    pdf_add_dict(
        (*p).root.dict,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Catalog\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_release_obj((*p).root.dict);
    (*p).root.dict = ::core::ptr::null_mut::<pdf_obj>();
}
unsafe extern "C-unwind" fn doc_resize_page_entries(
    mut p: *mut pdf_doc,
    mut size: ::core::ffi::c_uint,
) {
    if size > (*p).pages.max_entries {
        let mut i: ::core::ffi::c_uint = 0;
        (*p).pages.entries = renew(
            (*p).pages.entries as *mut ::core::ffi::c_void,
            (size as uint32_t as usize).wrapping_mul(::core::mem::size_of::<pdf_page>() as usize)
                as uint32_t,
        ) as *mut pdf_page as *mut pdf_page;
        i = (*p).pages.max_entries;
        while i < size {
            let ref mut fresh4 = (*(*p).pages.entries.offset(i as isize)).page_obj;
            *fresh4 = ::core::ptr::null_mut::<pdf_obj>();
            let ref mut fresh5 = (*(*p).pages.entries.offset(i as isize)).page_ref;
            *fresh5 = ::core::ptr::null_mut::<pdf_obj>();
            (*(*p).pages.entries.offset(i as isize)).flags = 0 as ::core::ffi::c_int;
            let ref mut fresh6 = (*(*p).pages.entries.offset(i as isize)).resources;
            *fresh6 = ::core::ptr::null_mut::<pdf_obj>();
            let ref mut fresh7 = (*(*p).pages.entries.offset(i as isize)).background;
            *fresh7 = ::core::ptr::null_mut::<pdf_obj>();
            let ref mut fresh8 = (*(*p).pages.entries.offset(i as isize)).contents;
            *fresh8 = ::core::ptr::null_mut::<pdf_obj>();
            let ref mut fresh9 = (*(*p).pages.entries.offset(i as isize)).content_refs
                [0 as ::core::ffi::c_int as usize];
            *fresh9 = ::core::ptr::null_mut::<pdf_obj>();
            let ref mut fresh10 = (*(*p).pages.entries.offset(i as isize)).content_refs
                [1 as ::core::ffi::c_int as usize];
            *fresh10 = ::core::ptr::null_mut::<pdf_obj>();
            let ref mut fresh11 = (*(*p).pages.entries.offset(i as isize)).content_refs
                [2 as ::core::ffi::c_int as usize];
            *fresh11 = ::core::ptr::null_mut::<pdf_obj>();
            let ref mut fresh12 = (*(*p).pages.entries.offset(i as isize)).content_refs
                [3 as ::core::ffi::c_int as usize];
            *fresh12 = ::core::ptr::null_mut::<pdf_obj>();
            let ref mut fresh13 = (*(*p).pages.entries.offset(i as isize)).annots;
            *fresh13 = ::core::ptr::null_mut::<pdf_obj>();
            let ref mut fresh14 = (*(*p).pages.entries.offset(i as isize)).beads;
            *fresh14 = ::core::ptr::null_mut::<pdf_obj>();
            i = i.wrapping_add(1);
        }
        (*p).pages.max_entries = size;
    }
}
unsafe extern "C-unwind" fn doc_get_page_entry(
    mut p: *mut pdf_doc,
    mut page_no: ::core::ffi::c_uint,
) -> *mut pdf_page {
    let mut page: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
    if page_no as ::core::ffi::c_ulong > 65535 as ::core::ffi::c_ulong {
        pdf_abort!(
            b"Page number %ul too large!\0" as *const u8 as *const ::core::ffi::c_char,
            page_no,
        );
    } else if page_no == 0 as ::core::ffi::c_uint {
        pdf_abort!(
            b"Invalid Page number %ul.\0" as *const u8 as *const ::core::ffi::c_char,
            page_no,
        );
    }
    if page_no > (*p).pages.max_entries {
        doc_resize_page_entries(p, page_no.wrapping_add(PDFDOC_PAGES_ALLOC_SIZE));
    }
    page = (*p)
        .pages
        .entries
        .offset(page_no.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
        as *mut pdf_page;
    return page;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_set_bop_content")]
pub unsafe extern "C-unwind" fn pdf_doc_set_bop_content(
    mut content: *const ::core::ffi::c_char,
    mut length: ::core::ffi::c_uint,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(*p).pages.bop.is_null() {
        pdf_release_obj((*p).pages.bop);
        (*p).pages.bop = ::core::ptr::null_mut::<pdf_obj>();
    }
    if length > 0 as ::core::ffi::c_uint {
        (*p).pages.bop = pdf_new_stream(STREAM_COMPRESS);
        pdf_add_stream(
            (*p).pages.bop,
            content as *const ::core::ffi::c_void,
            length as ::core::ffi::c_int,
        );
    } else {
        (*p).pages.bop = ::core::ptr::null_mut::<pdf_obj>();
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_set_eop_content")]
pub unsafe extern "C-unwind" fn pdf_doc_set_eop_content(
    mut content: *const ::core::ffi::c_char,
    mut length: ::core::ffi::c_uint,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    if !(*p).pages.eop.is_null() {
        pdf_release_obj((*p).pages.eop);
        (*p).pages.eop = ::core::ptr::null_mut::<pdf_obj>();
    }
    if length > 0 as ::core::ffi::c_uint {
        (*p).pages.eop = pdf_new_stream(STREAM_COMPRESS);
        pdf_add_stream(
            (*p).pages.eop,
            content as *const ::core::ffi::c_void,
            length as ::core::ffi::c_int,
        );
    } else {
        (*p).pages.eop = ::core::ptr::null_mut::<pdf_obj>();
    };
}
unsafe extern "C-unwind" fn pdf_doc_init_docinfo(mut p: *mut pdf_doc) {
    (*p).info = pdf_new_dict();
    pdf_set_info((*p).info);
}
unsafe extern "C-unwind" fn pdf_doc_close_docinfo(mut p: *mut pdf_doc) {
    let mut docinfo: *mut pdf_obj = (*p).info;
    let mut keys: [*const ::core::ffi::c_char; 9] = [
        b"Title\0" as *const u8 as *const ::core::ffi::c_char,
        b"Author\0" as *const u8 as *const ::core::ffi::c_char,
        b"Subject\0" as *const u8 as *const ::core::ffi::c_char,
        b"Keywords\0" as *const u8 as *const ::core::ffi::c_char,
        b"Creator\0" as *const u8 as *const ::core::ffi::c_char,
        b"Producer\0" as *const u8 as *const ::core::ffi::c_char,
        b"CreationDate\0" as *const u8 as *const ::core::ffi::c_char,
        b"ModDate\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut value: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while !keys[i as usize].is_null() {
        value = pdf_lookup_dict(docinfo, keys[i as usize]);
        if !value.is_null() {
            if !(!value.is_null() && pdf_obj_typeof(value) == PDF_STRING) {
                pdf_warning!(
                    b"\"%s\" in DocInfo dictionary not string type.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    keys[i as usize],
                );
                pdf_remove_dict(docinfo, keys[i as usize]);
                pdf_warning!(
                    b"\"%s\" removed from DocInfo.\0" as *const u8 as *const ::core::ffi::c_char,
                    keys[i as usize],
                );
            } else if pdf_string_length(value) == 0 as ::core::ffi::c_uint {
                pdf_remove_dict(docinfo, keys[i as usize]);
            }
        }
        i = i.wrapping_add(1);
    }
    if pdf_lookup_dict(
        docinfo,
        b"Producer\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        let mut banner: [::core::ffi::c_char; 16] =
            crate::backend::c_char_array(*b"xdvipdfmx (0.1)\0");
        pdf_add_dict(
            docinfo,
            pdf_new_name(b"Producer\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                &raw mut banner as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(&raw mut banner as *mut ::core::ffi::c_char),
            ),
        );
    }
    if pdf_lookup_dict(
        docinfo,
        b"CreationDate\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
        && ttpi_source_date_epoch != 0
    {
        let mut now: [::core::ffi::c_char; 80] = [0; 80];
        dpx_util_format_asn_date(
            &raw mut now as *mut ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        );
        pdf_add_dict(
            docinfo,
            pdf_new_name(b"CreationDate\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                &raw mut now as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(&raw mut now as *mut ::core::ffi::c_char),
            ),
        );
    }
    pdf_release_obj(docinfo);
    (*p).info = ::core::ptr::null_mut::<pdf_obj>();
}
unsafe extern "C-unwind" fn pdf_doc_get_page_resources(
    mut p: *mut pdf_doc,
    mut category: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut resources: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut currentpage: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
    let mut res_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if p.is_null() || category.is_null() {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if !(*p).pending_forms.is_null() {
        if !(*(*p).pending_forms).form.resources.is_null() {
            res_dict = (*(*p).pending_forms).form.resources;
        } else {
            (*(*p).pending_forms).form.resources = pdf_new_dict();
            res_dict = (*(*p).pending_forms).form.resources;
        }
    } else {
        currentpage = (*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
        if !(*currentpage).resources.is_null() {
            res_dict = (*currentpage).resources;
        } else {
            (*currentpage).resources = pdf_new_dict();
            res_dict = (*currentpage).resources;
        }
    }
    resources = pdf_lookup_dict(res_dict, category);
    if resources.is_null() {
        resources = pdf_new_dict();
        pdf_add_dict(res_dict, pdf_new_name(category), resources);
    } else if pdf_obj_typeof(resources) == PDF_INDIRECT {
        if resources.is_null() {
            pdf_warning!(
                b"Resource %s already released?\0" as *const u8 as *const ::core::ffi::c_char,
                category,
            );
        } else {
            pdf_release_obj(resources);
        }
    }
    return resources;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_add_page_resource")]
pub unsafe extern "C-unwind" fn pdf_doc_add_page_resource(
    mut category: *const ::core::ffi::c_char,
    mut resource_name: *const ::core::ffi::c_char,
    mut resource_ref: *mut pdf_obj,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut resources: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut duplicate: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    resources = pdf_doc_get_page_resources(p, category);
    if resources.is_null() {
        pdf_warning!(
            b"Can't add object to resource %s\0" as *const u8 as *const ::core::ffi::c_char,
            category,
        );
        return;
    }
    duplicate = pdf_lookup_dict(resources, resource_name);
    if !duplicate.is_null() {
        if pdf_compare_reference(duplicate, resource_ref) != 0 {
            pdf_warning!(
                b"Conflicting page resource found (page: %d, category: %s, name: %s).\0"
                    as *const u8 as *const ::core::ffi::c_char,
                pdf_doc_current_page_number(),
                category,
                resource_name,
            );
            pdf_warning!(b"Ignoring...\0" as *const u8 as *const ::core::ffi::c_char);
        }
        pdf_release_obj(resource_ref);
    } else {
        pdf_add_dict(resources, pdf_new_name(resource_name), resource_ref);
    };
}
unsafe extern "C-unwind" fn doc_flush_page(
    mut p: *mut pdf_doc,
    mut page: *mut pdf_page,
    mut parent_ref: *mut pdf_obj,
) {
    let mut contents_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut count: ::core::ffi::c_uint = 0;
    pdf_add_dict(
        (*page).page_obj,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Page\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        (*page).page_obj,
        pdf_new_name(b"Parent\0" as *const u8 as *const ::core::ffi::c_char),
        parent_ref,
    );
    if (*page).flags & USE_MY_MEDIABOX != 0 {
        let mut mediabox: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        mediabox = pdf_new_array();
        pdf_add_array(
            mediabox,
            pdf_new_number(floor((*page).cropbox.llx / 0.01f64 + 0.5f64) * 0.01f64),
        );
        pdf_add_array(
            mediabox,
            pdf_new_number(floor((*page).cropbox.lly / 0.01f64 + 0.5f64) * 0.01f64),
        );
        pdf_add_array(
            mediabox,
            pdf_new_number(floor((*page).cropbox.urx / 0.01f64 + 0.5f64) * 0.01f64),
        );
        pdf_add_array(
            mediabox,
            pdf_new_number(floor((*page).cropbox.ury / 0.01f64 + 0.5f64) * 0.01f64),
        );
        pdf_add_dict(
            (*page).page_obj,
            pdf_new_name(b"MediaBox\0" as *const u8 as *const ::core::ffi::c_char),
            mediabox,
        );
    }
    count = 0 as ::core::ffi::c_uint;
    contents_array = pdf_new_array();
    if !(*page).content_refs[0 as ::core::ffi::c_int as usize].is_null() {
        pdf_add_array(
            contents_array,
            (*page).content_refs[0 as ::core::ffi::c_int as usize],
        );
        count = count.wrapping_add(1);
    } else if !(*p).pages.bop.is_null()
        && pdf_stream_length((*p).pages.bop) > 0 as ::core::ffi::c_int
    {
        pdf_add_array(contents_array, pdf_ref_obj((*p).pages.bop));
        count = count.wrapping_add(1);
    }
    if !(*page).content_refs[1 as ::core::ffi::c_int as usize].is_null() {
        pdf_add_array(
            contents_array,
            (*page).content_refs[1 as ::core::ffi::c_int as usize],
        );
        count = count.wrapping_add(1);
    }
    if !(*page).content_refs[2 as ::core::ffi::c_int as usize].is_null() {
        pdf_add_array(
            contents_array,
            (*page).content_refs[2 as ::core::ffi::c_int as usize],
        );
        count = count.wrapping_add(1);
    }
    if !(*page).content_refs[3 as ::core::ffi::c_int as usize].is_null() {
        pdf_add_array(
            contents_array,
            (*page).content_refs[3 as ::core::ffi::c_int as usize],
        );
        count = count.wrapping_add(1);
    } else if !(*p).pages.eop.is_null()
        && pdf_stream_length((*p).pages.eop) > 0 as ::core::ffi::c_int
    {
        pdf_add_array(contents_array, pdf_ref_obj((*p).pages.eop));
        count = count.wrapping_add(1);
    }
    if count == 0 as ::core::ffi::c_uint {
        pdf_warning!(
            b"Page with empty content found!!!\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*page).content_refs[0 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<pdf_obj>();
    (*page).content_refs[1 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<pdf_obj>();
    (*page).content_refs[2 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<pdf_obj>();
    (*page).content_refs[3 as ::core::ffi::c_int as usize] = ::core::ptr::null_mut::<pdf_obj>();
    pdf_add_dict(
        (*page).page_obj,
        pdf_new_name(b"Contents\0" as *const u8 as *const ::core::ffi::c_char),
        contents_array,
    );
    if !(*page).annots.is_null() {
        pdf_add_dict(
            (*page).page_obj,
            pdf_new_name(b"Annots\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj((*page).annots),
        );
        pdf_release_obj((*page).annots);
    }
    if !(*page).beads.is_null() {
        pdf_add_dict(
            (*page).page_obj,
            pdf_new_name(b"B\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj((*page).beads),
        );
        pdf_release_obj((*page).beads);
    }
    pdf_release_obj((*page).page_obj);
    pdf_release_obj((*page).page_ref);
    (*page).page_obj = ::core::ptr::null_mut::<pdf_obj>();
    (*page).page_ref = ::core::ptr::null_mut::<pdf_obj>();
    (*page).annots = ::core::ptr::null_mut::<pdf_obj>();
    (*page).beads = ::core::ptr::null_mut::<pdf_obj>();
}
pub const PAGE_CLUSTER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn build_page_tree(
    mut p: *mut pdf_doc,
    mut firstpage: *mut pdf_page,
    mut num_pages: ::core::ffi::c_int,
    mut parent_ref: *mut pdf_obj,
) -> *mut pdf_obj {
    let mut self_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut self_ref: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut kids: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut i: ::core::ffi::c_int = 0;
    self_0 = pdf_new_dict();
    self_ref = if !parent_ref.is_null() {
        pdf_ref_obj(self_0)
    } else {
        pdf_ref_obj((*p).root.pages)
    };
    pdf_add_dict(
        self_0,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Pages\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        self_0,
        pdf_new_name(b"Count\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(num_pages as ::core::ffi::c_double),
    );
    if !parent_ref.is_null() {
        pdf_add_dict(
            self_0,
            pdf_new_name(b"Parent\0" as *const u8 as *const ::core::ffi::c_char),
            parent_ref,
        );
    }
    kids = pdf_new_array();
    if num_pages > 0 as ::core::ffi::c_int && num_pages <= PAGE_CLUSTER {
        i = 0 as ::core::ffi::c_int;
        while i < num_pages {
            let mut page: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
            page = firstpage.offset(i as isize);
            if (*page).page_ref.is_null() {
                (*page).page_ref = pdf_ref_obj((*page).page_obj);
            }
            pdf_add_array(kids, pdf_link_obj((*page).page_ref));
            doc_flush_page(p, page, pdf_link_obj(self_ref));
            i += 1;
        }
    } else if num_pages > 0 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < PAGE_CLUSTER {
            let mut start: ::core::ffi::c_int = 0;
            let mut end: ::core::ffi::c_int = 0;
            start = i * num_pages / PAGE_CLUSTER;
            end = (i + 1 as ::core::ffi::c_int) * num_pages / PAGE_CLUSTER;
            if end - start > 1 as ::core::ffi::c_int {
                let mut subtree: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                subtree = build_page_tree(
                    p,
                    firstpage.offset(start as isize),
                    end - start,
                    pdf_link_obj(self_ref),
                );
                pdf_add_array(kids, pdf_ref_obj(subtree));
                pdf_release_obj(subtree);
            } else {
                let mut page_0: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
                page_0 = firstpage.offset(start as isize);
                if (*page_0).page_ref.is_null() {
                    (*page_0).page_ref = pdf_ref_obj((*page_0).page_obj);
                }
                pdf_add_array(kids, pdf_link_obj((*page_0).page_ref));
                doc_flush_page(p, page_0, pdf_link_obj(self_ref));
            }
            i += 1;
        }
    }
    pdf_add_dict(
        self_0,
        pdf_new_name(b"Kids\0" as *const u8 as *const ::core::ffi::c_char),
        kids,
    );
    pdf_release_obj(self_ref);
    return self_0;
}
unsafe extern "C-unwind" fn pdf_doc_init_page_tree(
    mut p: *mut pdf_doc,
    mut media_width: ::core::ffi::c_double,
    mut media_height: ::core::ffi::c_double,
) {
    (*p).root.pages = pdf_new_dict();
    (*p).pages.num_entries = 0 as ::core::ffi::c_uint;
    (*p).pages.max_entries = 0 as ::core::ffi::c_uint;
    (*p).pages.entries = ::core::ptr::null_mut::<pdf_page>();
    (*p).pages.bop = ::core::ptr::null_mut::<pdf_obj>();
    (*p).pages.eop = ::core::ptr::null_mut::<pdf_obj>();
    (*p).pages.mediabox.llx = 0.0f64;
    (*p).pages.mediabox.lly = 0.0f64;
    (*p).pages.mediabox.urx = media_width;
    (*p).pages.mediabox.ury = media_height;
}
unsafe extern "C-unwind" fn pdf_doc_close_page_tree(mut p: *mut pdf_doc) {
    let mut page_tree_root: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut mediabox: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut page_no: ::core::ffi::c_uint = 0;
    page_no = (*p)
        .pages
        .num_entries
        .wrapping_add(1 as ::core::ffi::c_uint);
    while page_no <= (*p).pages.max_entries {
        let mut page: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
        page = doc_get_page_entry(p, page_no);
        if !(*page).page_obj.is_null() {
            pdf_warning!(
                b"Nonexistent page #%u refered.\0" as *const u8 as *const ::core::ffi::c_char,
                page_no,
            );
            pdf_release_obj((*page).page_ref);
            (*page).page_ref = ::core::ptr::null_mut::<pdf_obj>();
        }
        if !(*page).page_obj.is_null() {
            pdf_warning!(
                b"Entry for a nonexistent page #%u created.\0" as *const u8
                    as *const ::core::ffi::c_char,
                page_no,
            );
            pdf_release_obj((*page).page_obj);
            (*page).page_obj = ::core::ptr::null_mut::<pdf_obj>();
        }
        if !(*page).annots.is_null() {
            pdf_warning!(
                b"Annotation attached to a nonexistent page #%u.\0" as *const u8
                    as *const ::core::ffi::c_char,
                page_no,
            );
            pdf_release_obj((*page).annots);
            (*page).annots = ::core::ptr::null_mut::<pdf_obj>();
        }
        if !(*page).beads.is_null() {
            pdf_warning!(
                b"Article beads attached to a nonexistent page #%u.\0" as *const u8
                    as *const ::core::ffi::c_char,
                page_no,
            );
            pdf_release_obj((*page).beads);
            (*page).beads = ::core::ptr::null_mut::<pdf_obj>();
        }
        if !(*page).resources.is_null() {
            pdf_release_obj((*page).resources);
            (*page).resources = ::core::ptr::null_mut::<pdf_obj>();
        }
        page_no = page_no.wrapping_add(1);
    }
    page_tree_root = build_page_tree(
        p,
        (*p).pages.entries.offset(0 as ::core::ffi::c_int as isize) as *mut pdf_page,
        (*p).pages.num_entries as ::core::ffi::c_int,
        ::core::ptr::null_mut::<pdf_obj>(),
    );
    pdf_merge_dict((*p).root.pages, page_tree_root);
    pdf_release_obj(page_tree_root);
    if !(*p).pages.bop.is_null() {
        pdf_add_stream(
            (*p).pages.bop,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as ::core::ffi::c_int,
        );
        pdf_release_obj((*p).pages.bop);
        (*p).pages.bop = ::core::ptr::null_mut::<pdf_obj>();
    }
    if !(*p).pages.eop.is_null() {
        pdf_add_stream(
            (*p).pages.eop,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as ::core::ffi::c_int,
        );
        pdf_release_obj((*p).pages.eop);
        (*p).pages.eop = ::core::ptr::null_mut::<pdf_obj>();
    }
    mediabox = pdf_new_array();
    pdf_add_array(
        mediabox,
        pdf_new_number(floor((*p).pages.mediabox.llx / 0.01f64 + 0.5f64) * 0.01f64),
    );
    pdf_add_array(
        mediabox,
        pdf_new_number(floor((*p).pages.mediabox.lly / 0.01f64 + 0.5f64) * 0.01f64),
    );
    pdf_add_array(
        mediabox,
        pdf_new_number(floor((*p).pages.mediabox.urx / 0.01f64 + 0.5f64) * 0.01f64),
    );
    pdf_add_array(
        mediabox,
        pdf_new_number(floor((*p).pages.mediabox.ury / 0.01f64 + 0.5f64) * 0.01f64),
    );
    pdf_add_dict(
        (*p).root.pages,
        pdf_new_name(b"MediaBox\0" as *const u8 as *const ::core::ffi::c_char),
        mediabox,
    );
    pdf_add_dict(
        (*p).root.dict,
        pdf_new_name(b"Pages\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj((*p).root.pages),
    );
    pdf_release_obj((*p).root.pages);
    (*p).root.pages = ::core::ptr::null_mut::<pdf_obj>();
    (*p).pages.entries = mfree((*p).pages.entries as *mut ::core::ffi::c_void) as *mut pdf_page;
    (*p).pages.num_entries = 0 as ::core::ffi::c_uint;
    (*p).pages.max_entries = 0 as ::core::ffi::c_uint;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_get_page_count")]
pub unsafe extern "C-unwind" fn pdf_doc_get_page_count(
    mut pf: *mut pdf_file,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut page_tree: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut catalog: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    catalog = pdf_file_get_catalog(pf);
    page_tree = pdf_deref_obj(pdf_lookup_dict(
        catalog,
        b"Pages\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    if !(!page_tree.is_null() && pdf_obj_typeof(page_tree) == PDF_DICT) {
        return 0 as ::core::ffi::c_int;
    }
    let mut tmp: *mut pdf_obj = pdf_deref_obj(pdf_lookup_dict(
        page_tree,
        b"Count\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NUMBER) {
        pdf_release_obj(tmp);
        return 0 as ::core::ffi::c_int;
    }
    count = pdf_number_value(tmp) as ::core::ffi::c_int;
    pdf_release_obj(tmp);
    return count;
}
unsafe extern "C-unwind" fn set_bounding_box(
    mut bbox: *mut pdf_rect,
    mut opt_bbox: pdf_page_boundary,
    mut media_box: *mut pdf_obj,
    mut crop_box: *mut pdf_obj,
    mut art_box: *mut pdf_obj,
    mut trim_box: *mut pdf_obj,
    mut bleed_box: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut box_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if media_box.is_null() {
        pdf_warning!(
            b"MediaBox not found in included PDF...\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if !media_box.is_null() {
        if !(!media_box.is_null() && pdf_obj_typeof(media_box) == PDF_ARRAY)
            || pdf_array_length(media_box) != 4 as ::core::ffi::c_uint
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if !crop_box.is_null() {
        if !(!crop_box.is_null() && pdf_obj_typeof(crop_box) == PDF_ARRAY)
            || pdf_array_length(crop_box) != 4 as ::core::ffi::c_uint
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if !art_box.is_null() {
        if !(!art_box.is_null() && pdf_obj_typeof(art_box) == PDF_ARRAY)
            || pdf_array_length(art_box) != 4 as ::core::ffi::c_uint
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if !trim_box.is_null() {
        if !(!trim_box.is_null() && pdf_obj_typeof(trim_box) == PDF_ARRAY)
            || pdf_array_length(trim_box) != 4 as ::core::ffi::c_uint
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if !bleed_box.is_null() {
        if !(!bleed_box.is_null() && pdf_obj_typeof(bleed_box) == PDF_ARRAY)
            || pdf_array_length(bleed_box) != 4 as ::core::ffi::c_uint
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if opt_bbox as ::core::ffi::c_uint
        == pdf_page_boundary__auto as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !crop_box.is_null() {
            box_0 = pdf_link_obj(crop_box);
        } else if !art_box.is_null() {
            box_0 = pdf_link_obj(art_box);
        } else if !trim_box.is_null() {
            box_0 = pdf_link_obj(trim_box);
        } else if !bleed_box.is_null() {
            box_0 = pdf_link_obj(bleed_box);
        } else {
            box_0 = pdf_link_obj(media_box);
        }
    } else {
        if crop_box.is_null() {
            crop_box = pdf_link_obj(media_box);
        }
        if art_box.is_null() {
            art_box = pdf_link_obj(crop_box);
        }
        if trim_box.is_null() {
            trim_box = pdf_link_obj(crop_box);
        }
        if bleed_box.is_null() {
            bleed_box = pdf_link_obj(crop_box);
        }
        match opt_bbox as ::core::ffi::c_uint {
            1 => {
                box_0 = pdf_link_obj(crop_box);
            }
            2 => {
                box_0 = pdf_link_obj(media_box);
            }
            3 => {
                box_0 = pdf_link_obj(art_box);
            }
            4 => {
                box_0 = pdf_link_obj(trim_box);
            }
            5 => {
                box_0 = pdf_link_obj(bleed_box);
            }
            _ => {
                box_0 = pdf_link_obj(crop_box);
            }
        }
    }
    if box_0.is_null() {
        pdf_warning!(
            b"No appropriate page boudary box found???\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 4 as ::core::ffi::c_int;
        loop {
            let fresh15 = i;
            i = i - 1;
            if !(fresh15 != 0) {
                break;
            }
            let mut x: ::core::ffi::c_double = 0.;
            let mut tmp: *mut pdf_obj = pdf_deref_obj(pdf_get_array(box_0, i));
            if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NUMBER) {
                pdf_release_obj(tmp);
                pdf_release_obj(box_0);
                return -(1 as ::core::ffi::c_int);
            }
            x = pdf_number_value(tmp);
            match i {
                0 => {
                    (*bbox).llx = x;
                }
                1 => {
                    (*bbox).lly = x;
                }
                2 => {
                    (*bbox).urx = x;
                }
                3 => {
                    (*bbox).ury = x;
                }
                _ => {}
            }
            pdf_release_obj(tmp);
        }
        if dpx_conf.compat_mode as ::core::ffi::c_uint
            == dpx_mode_xdv_mode as ::core::ffi::c_int as ::core::ffi::c_uint
            || opt_bbox as ::core::ffi::c_uint
                != pdf_page_boundary__auto as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            i = 4 as ::core::ffi::c_int;
            loop {
                let fresh16 = i;
                i = i - 1;
                if !(fresh16 != 0) {
                    break;
                }
                let mut x_0: ::core::ffi::c_double = 0.;
                let mut tmp_0: *mut pdf_obj = pdf_deref_obj(pdf_get_array(media_box, i));
                if !(!tmp_0.is_null() && pdf_obj_typeof(tmp_0) == PDF_NUMBER) {
                    pdf_release_obj(tmp_0);
                    pdf_release_obj(box_0);
                    return -(1 as ::core::ffi::c_int);
                }
                x_0 = pdf_number_value(tmp_0);
                match i {
                    0 => {
                        if (*bbox).llx < x_0 {
                            (*bbox).llx = x_0;
                        }
                    }
                    1 => {
                        if (*bbox).lly < x_0 {
                            (*bbox).lly = x_0;
                        }
                    }
                    2 => {
                        if (*bbox).urx > x_0 {
                            (*bbox).urx = x_0;
                        }
                    }
                    3 => {
                        if (*bbox).ury > x_0 {
                            (*bbox).ury = x_0;
                        }
                    }
                    _ => {}
                }
                pdf_release_obj(tmp_0);
            }
        }
    }
    pdf_release_obj(box_0);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_transform_matrix(
    mut matrix: *mut pdf_tmatrix,
    mut bbox: *mut pdf_rect,
    mut rotate: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut deg: ::core::ffi::c_double = 0.;
    let mut rot: ::core::ffi::c_int = 0;
    (*matrix).d = 1.0f64;
    (*matrix).a = (*matrix).d;
    (*matrix).c = 0.0f64;
    (*matrix).b = (*matrix).c;
    (*matrix).f = 0.0f64;
    (*matrix).e = (*matrix).f;
    if !rotate.is_null() {
        if !(!rotate.is_null() && pdf_obj_typeof(rotate) == PDF_NUMBER) {
            return -(1 as ::core::ffi::c_int);
        } else {
            deg = pdf_number_value(rotate);
            if deg - deg as ::core::ffi::c_int as ::core::ffi::c_double != 0.0f64 {
                pdf_warning!(
                    b"Invalid value specified for /Rotate: %f\0" as *const u8
                        as *const ::core::ffi::c_char,
                    deg,
                );
                return -(1 as ::core::ffi::c_int);
            } else if deg != 0.0f64 {
                rot = deg as ::core::ffi::c_int;
                if (rot % 90 as ::core::ffi::c_int) as ::core::ffi::c_double != 0.0f64 {
                    pdf_warning!(
                        b"Invalid value specified for /Rotate: %f\0" as *const u8
                            as *const ::core::ffi::c_char,
                        deg,
                    );
                } else {
                    rot = rot % 360 as ::core::ffi::c_int;
                    if rot < 0 as ::core::ffi::c_int {
                        rot += 360 as ::core::ffi::c_int;
                    }
                    match rot {
                        90 => {
                            (*matrix).d = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                            (*matrix).a = (*matrix).d;
                            (*matrix).b = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
                            (*matrix).c = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                            (*matrix).e = (*bbox).llx - (*bbox).lly;
                            (*matrix).f = (*bbox).lly + (*bbox).urx;
                        }
                        180 => {
                            (*matrix).d = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
                            (*matrix).a = (*matrix).d;
                            (*matrix).c = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                            (*matrix).b = (*matrix).c;
                            (*matrix).e = (*bbox).llx + (*bbox).urx;
                            (*matrix).f = (*bbox).lly + (*bbox).ury;
                        }
                        270 => {
                            (*matrix).d = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                            (*matrix).a = (*matrix).d;
                            (*matrix).b = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                            (*matrix).c = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
                            (*matrix).e = (*bbox).llx + (*bbox).ury;
                            (*matrix).f = (*bbox).lly - (*bbox).llx;
                        }
                        _ => {
                            pdf_warning!(
                                b"Invalid value specified for /Rotate: %f\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                deg,
                            );
                        }
                    }
                }
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_get_page")]
pub unsafe extern "C-unwind" fn pdf_doc_get_page(
    mut pf: *mut pdf_file,
    mut page_no: ::core::ffi::c_int,
    mut opt_bbox: pdf_page_boundary,
    mut bbox: *mut pdf_rect,
    mut matrix: *mut pdf_tmatrix,
    mut resources_p: *mut *mut pdf_obj,
) -> *mut pdf_obj {
    let mut current_block: u64;
    let mut catalog: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut page_tree: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut resources: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut rotate: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut art_box: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut trim_box: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut bleed_box: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut media_box: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut crop_box: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    catalog = pdf_file_get_catalog(pf);
    page_tree = pdf_deref_obj(pdf_lookup_dict(
        catalog,
        b"Pages\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    if !(!page_tree.is_null() && pdf_obj_typeof(page_tree) == PDF_DICT) {
        current_block = 17476884660199964226;
    } else {
        let mut count: ::core::ffi::c_int = 0;
        let mut tmp: *mut pdf_obj = pdf_deref_obj(pdf_lookup_dict(
            page_tree,
            b"Count\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NUMBER) {
            pdf_release_obj(tmp);
            current_block = 17476884660199964226;
        } else {
            count = pdf_number_value(tmp) as ::core::ffi::c_int;
            pdf_release_obj(tmp);
            if page_no <= 0 as ::core::ffi::c_int || page_no > count {
                pdf_warning!(
                    b"Page %d does not exist.\0" as *const u8 as *const ::core::ffi::c_char,
                    page_no,
                );
                current_block = 11270100909120839572;
            } else {
                let mut kids: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                let mut tmp_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                let mut depth: ::core::ffi::c_int = PDF_OBJ_MAX_DEPTH;
                let mut page_idx: ::core::ffi::c_int = page_no - 1 as ::core::ffi::c_int;
                let mut kids_length: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                's_62: loop {
                    depth -= 1;
                    if !(depth != 0 && i != kids_length) {
                        current_block = 13321564401369230990;
                        break;
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"MediaBox\0" as *const u8 as *const ::core::ffi::c_char,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(media_box);
                        media_box = tmp_0;
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"CropBox\0" as *const u8 as *const ::core::ffi::c_char,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(crop_box);
                        crop_box = tmp_0;
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"ArtBox\0" as *const u8 as *const ::core::ffi::c_char,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(art_box);
                        art_box = tmp_0;
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"TrimBox\0" as *const u8 as *const ::core::ffi::c_char,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(trim_box);
                        trim_box = tmp_0;
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"BleedBox\0" as *const u8 as *const ::core::ffi::c_char,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(bleed_box);
                        bleed_box = tmp_0;
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"Rotate\0" as *const u8 as *const ::core::ffi::c_char,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(rotate);
                        rotate = tmp_0;
                    }
                    tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"Resources\0" as *const u8 as *const ::core::ffi::c_char,
                    ));
                    if !tmp_0.is_null() {
                        pdf_release_obj(resources);
                        resources = tmp_0;
                    }
                    kids = pdf_deref_obj(pdf_lookup_dict(
                        page_tree,
                        b"Kids\0" as *const u8 as *const ::core::ffi::c_char,
                    ));
                    if kids.is_null() {
                        current_block = 13321564401369230990;
                        break;
                    }
                    if !(!kids.is_null() && pdf_obj_typeof(kids) == PDF_ARRAY) {
                        pdf_release_obj(kids);
                        current_block = 17476884660199964226;
                        break;
                    } else {
                        kids_length = pdf_array_length(kids) as ::core::ffi::c_int;
                        i = 0 as ::core::ffi::c_int;
                        while i < kids_length {
                            let mut count_0: ::core::ffi::c_int = 0;
                            pdf_release_obj(page_tree);
                            page_tree = pdf_deref_obj(pdf_get_array(kids, i));
                            if !(!page_tree.is_null() && pdf_obj_typeof(page_tree) == PDF_DICT) {
                                current_block = 17476884660199964226;
                                break 's_62;
                            }
                            tmp_0 = pdf_deref_obj(pdf_lookup_dict(
                                page_tree,
                                b"Count\0" as *const u8 as *const ::core::ffi::c_char,
                            ));
                            if !tmp_0.is_null() && pdf_obj_typeof(tmp_0) == PDF_NUMBER {
                                count_0 = pdf_number_value(tmp_0) as ::core::ffi::c_int;
                                pdf_release_obj(tmp_0);
                            } else if tmp_0.is_null() {
                                count_0 = 1 as ::core::ffi::c_int;
                            } else {
                                pdf_release_obj(tmp_0);
                                current_block = 17476884660199964226;
                                break 's_62;
                            }
                            if page_idx < count_0 {
                                break;
                            }
                            page_idx -= count_0;
                            i += 1;
                        }
                        pdf_release_obj(kids);
                    }
                }
                match current_block {
                    17476884660199964226 => {}
                    _ => {
                        if depth == 0 || kids_length == i {
                            current_block = 17476884660199964226;
                        } else if !(!resources.is_null() && pdf_obj_typeof(resources) == PDF_DICT) {
                            current_block = 17476884660199964226;
                        } else {
                            if !resources_p.is_null() {
                                *resources_p = pdf_link_obj(resources);
                            }
                            error = set_bounding_box(
                                bbox, opt_bbox, media_box, crop_box, art_box, trim_box, bleed_box,
                            );
                            if error != 0 {
                                current_block = 17476884660199964226;
                            } else {
                                error = set_transform_matrix(matrix, bbox, rotate);
                                if error != 0 {
                                    current_block = 17476884660199964226;
                                } else {
                                    current_block = 16524178054315996098;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        17476884660199964226 => {
            pdf_warning!(
                b"Error found in including PDF image.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block = 11270100909120839572;
        }
        _ => {}
    }
    match current_block {
        11270100909120839572 => {
            pdf_release_obj(page_tree);
            page_tree = ::core::ptr::null_mut::<pdf_obj>();
        }
        _ => {}
    }
    pdf_release_obj(crop_box);
    pdf_release_obj(bleed_box);
    pdf_release_obj(trim_box);
    pdf_release_obj(art_box);
    pdf_release_obj(media_box);
    pdf_release_obj(rotate);
    pdf_release_obj(resources);
    return page_tree;
}
unsafe extern "C-unwind" fn pdf_doc_init_bookmarks(
    mut p: *mut pdf_doc,
    mut bm_open_depth: ::core::ffi::c_int,
) {
    let mut item: *mut pdf_olitem = ::core::ptr::null_mut::<pdf_olitem>();
    (*p).options.outline_open_depth = (if bm_open_depth >= 0 as ::core::ffi::c_int {
        bm_open_depth as ::core::ffi::c_uint
    } else {
        MAX_OUTLINE_DEPTH.wrapping_sub(bm_open_depth as ::core::ffi::c_uint)
    }) as ::core::ffi::c_int;
    (*p).outlines.current_depth = 1 as ::core::ffi::c_int;
    item = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_olitem>() as usize) as uint32_t)
        as *mut pdf_olitem;
    (*item).dict = ::core::ptr::null_mut::<pdf_obj>();
    (*item).next = ::core::ptr::null_mut::<pdf_olitem>();
    (*item).first = ::core::ptr::null_mut::<pdf_olitem>();
    (*item).parent = ::core::ptr::null_mut::<pdf_olitem>();
    (*item).is_open = 1 as ::core::ffi::c_int;
    (*p).outlines.current = item;
    (*p).outlines.first = item;
}
pub const MAX_OUTLINE_DEPTH: ::core::ffi::c_uint = 256 as ::core::ffi::c_uint;
unsafe extern "C-unwind" fn clean_bookmarks(mut item: *mut pdf_olitem) -> ::core::ffi::c_int {
    let mut next: *mut pdf_olitem = ::core::ptr::null_mut::<pdf_olitem>();
    while !item.is_null() {
        next = (*item).next as *mut pdf_olitem;
        pdf_release_obj((*item).dict);
        if !(*item).first.is_null() {
            clean_bookmarks((*item).first as *mut pdf_olitem);
        }
        free(item as *mut ::core::ffi::c_void);
        item = next;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn flush_bookmarks(
    mut node: *mut pdf_olitem,
    mut parent_ref: *mut pdf_obj,
    mut parent_dict: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut item: *mut pdf_olitem = ::core::ptr::null_mut::<pdf_olitem>();
    let mut this_ref: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut prev_ref: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut next_ref: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if (*node).dict.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"node->dict\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    this_ref = pdf_ref_obj((*node).dict);
    pdf_add_dict(
        parent_dict,
        pdf_new_name(b"First\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_link_obj(this_ref),
    );
    retval = 0 as ::core::ffi::c_int;
    item = node;
    prev_ref = ::core::ptr::null_mut::<pdf_obj>();
    while !item.is_null() && !(*item).dict.is_null() {
        if !(*item).first.is_null() && !(*(*item).first).dict.is_null() {
            count = flush_bookmarks((*item).first as *mut pdf_olitem, this_ref, (*item).dict);
            if (*item).is_open != 0 {
                pdf_add_dict(
                    (*item).dict,
                    pdf_new_name(b"Count\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_new_number(count as ::core::ffi::c_double),
                );
                retval += count;
            } else {
                pdf_add_dict(
                    (*item).dict,
                    pdf_new_name(b"Count\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_new_number(-count as ::core::ffi::c_double),
                );
            }
        }
        pdf_add_dict(
            (*item).dict,
            pdf_new_name(b"Parent\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_link_obj(parent_ref),
        );
        if !prev_ref.is_null() {
            pdf_add_dict(
                (*item).dict,
                pdf_new_name(b"Prev\0" as *const u8 as *const ::core::ffi::c_char),
                prev_ref,
            );
        }
        if !(*item).next.is_null() && !(*(*item).next).dict.is_null() {
            next_ref = pdf_ref_obj((*(*item).next).dict);
            pdf_add_dict(
                (*item).dict,
                pdf_new_name(b"Next\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_link_obj(next_ref),
            );
        } else {
            next_ref = ::core::ptr::null_mut::<pdf_obj>();
        }
        pdf_release_obj((*item).dict);
        (*item).dict = ::core::ptr::null_mut::<pdf_obj>();
        prev_ref = this_ref;
        this_ref = next_ref;
        retval += 1;
        item = (*item).next as *mut pdf_olitem;
    }
    pdf_add_dict(
        parent_dict,
        pdf_new_name(b"Last\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_link_obj(prev_ref),
    );
    pdf_release_obj(prev_ref);
    pdf_release_obj((*node).dict);
    (*node).dict = ::core::ptr::null_mut::<pdf_obj>();
    return retval;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_bookmarks_up")]
pub unsafe extern "C-unwind" fn pdf_doc_bookmarks_up() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut parent: *mut pdf_olitem = ::core::ptr::null_mut::<pdf_olitem>();
    let mut item: *mut pdf_olitem = ::core::ptr::null_mut::<pdf_olitem>();
    item = (*p).outlines.current;
    if item.is_null() || (*item).parent.is_null() {
        pdf_warning!(
            b"Can't go up above the bookmark root node!\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    parent = (*item).parent as *mut pdf_olitem;
    item = (*parent).next as *mut pdf_olitem;
    if (*parent).next.is_null() {
        item = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<pdf_olitem>() as usize)
            as uint32_t) as *mut pdf_olitem;
        (*parent).next = item as *mut pdf_olitem;
        (*item).dict = ::core::ptr::null_mut::<pdf_obj>();
        (*item).first = ::core::ptr::null_mut::<pdf_olitem>();
        (*item).next = ::core::ptr::null_mut::<pdf_olitem>();
        (*item).is_open = 0 as ::core::ffi::c_int;
        (*item).parent = (*parent).parent;
    }
    (*p).outlines.current = item;
    (*p).outlines.current_depth -= 1;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_bookmarks_down")]
pub unsafe extern "C-unwind" fn pdf_doc_bookmarks_down() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut item: *mut pdf_olitem = ::core::ptr::null_mut::<pdf_olitem>();
    let mut first: *mut pdf_olitem = ::core::ptr::null_mut::<pdf_olitem>();
    item = (*p).outlines.current;
    if (*item).dict.is_null() {
        let mut tcolor: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut action: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        pdf_warning!(b"Empty bookmark node!\0" as *const u8 as *const ::core::ffi::c_char,);
        pdf_warning!(
            b"You have tried to jump more than 1 level.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        (*item).dict = pdf_new_dict();
        pdf_add_dict(
            (*item).dict,
            pdf_new_name(b"Title\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                TITLE_STRING.as_ptr() as *const ::core::ffi::c_void,
                strlen(TITLE_STRING.as_ptr()),
            ),
        );
        tcolor = pdf_new_array();
        pdf_add_array(tcolor, pdf_new_number(1.0f64));
        pdf_add_array(tcolor, pdf_new_number(0.0f64));
        pdf_add_array(tcolor, pdf_new_number(0.0f64));
        pdf_add_dict(
            (*item).dict,
            pdf_new_name(b"C\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_link_obj(tcolor),
        );
        pdf_release_obj(tcolor);
        pdf_add_dict(
            (*item).dict,
            pdf_new_name(b"F\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(1.0f64),
        );
        action = pdf_new_dict();
        pdf_add_dict(
            action,
            pdf_new_name(b"S\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"JavaScript\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_dict(
            action,
            pdf_new_name(b"JS\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                JS_CODE.as_ptr() as *const ::core::ffi::c_void,
                strlen(JS_CODE.as_ptr()),
            ),
        );
        pdf_add_dict(
            (*item).dict,
            pdf_new_name(b"A\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_link_obj(action),
        );
        pdf_release_obj(action);
    }
    first = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_olitem>() as usize) as uint32_t)
        as *mut pdf_olitem;
    (*item).first = first as *mut pdf_olitem;
    (*first).dict = ::core::ptr::null_mut::<pdf_obj>();
    (*first).is_open = 0 as ::core::ffi::c_int;
    (*first).parent = item as *mut pdf_olitem;
    (*first).next = ::core::ptr::null_mut::<pdf_olitem>();
    (*first).first = ::core::ptr::null_mut::<pdf_olitem>();
    (*p).outlines.current = first;
    (*p).outlines.current_depth += 1;
    return 0 as ::core::ffi::c_int;
}
pub const TITLE_STRING: [::core::ffi::c_char; 11] = crate::backend::c_char_array(*b"<No Title>\0");
pub const JS_CODE: [::core::ffi::c_char; 78] = crate::backend::c_char_array(
    *b"app.alert(\"The author of this document made this bookmark item empty!\", 3, 0)\0",
);
#[unsafe(export_name = "__pdf_impl_pdf_doc_bookmarks_depth")]
pub unsafe extern "C-unwind" fn pdf_doc_bookmarks_depth() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    return (*p).outlines.current_depth;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_bookmarks_add")]
pub unsafe extern "C-unwind" fn pdf_doc_bookmarks_add(
    mut dict: *mut pdf_obj,
    mut is_open: ::core::ffi::c_int,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut item: *mut pdf_olitem = ::core::ptr::null_mut::<pdf_olitem>();
    let mut next: *mut pdf_olitem = ::core::ptr::null_mut::<pdf_olitem>();
    if !(!p.is_null() && !dict.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p && dict\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    item = (*p).outlines.current;
    if item.is_null() {
        item = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<pdf_olitem>() as usize)
            as uint32_t) as *mut pdf_olitem;
        (*item).parent = ::core::ptr::null_mut::<pdf_olitem>();
        (*p).outlines.first = item;
    } else if !(*item).dict.is_null() {
        item = (*item).next as *mut pdf_olitem;
    }
    (*item).dict = dict;
    (*item).first = ::core::ptr::null_mut::<pdf_olitem>();
    (*item).is_open = if is_open < 0 as ::core::ffi::c_int {
        if (*p).outlines.current_depth > (*p).options.outline_open_depth {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }
    } else {
        is_open
    };
    next = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_olitem>() as usize) as uint32_t)
        as *mut pdf_olitem;
    (*item).next = next as *mut pdf_olitem;
    (*next).dict = ::core::ptr::null_mut::<pdf_obj>();
    (*next).parent = (*item).parent;
    (*next).first = ::core::ptr::null_mut::<pdf_olitem>();
    (*next).is_open = -(1 as ::core::ffi::c_int);
    (*next).next = ::core::ptr::null_mut::<pdf_olitem>();
    (*p).outlines.current = item;
    pdf_doc_add_goto(dict);
}
unsafe extern "C-unwind" fn pdf_doc_close_bookmarks(mut p: *mut pdf_doc) {
    let mut catalog: *mut pdf_obj = (*p).root.dict;
    let mut item: *mut pdf_olitem = ::core::ptr::null_mut::<pdf_olitem>();
    let mut count: ::core::ffi::c_int = 0;
    let mut bm_root: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut bm_root_ref: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    item = (*p).outlines.first;
    if !(*item).dict.is_null() {
        bm_root = pdf_new_dict();
        bm_root_ref = pdf_ref_obj(bm_root);
        count = flush_bookmarks(item, bm_root_ref, bm_root);
        pdf_add_dict(
            bm_root,
            pdf_new_name(b"Count\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(count as ::core::ffi::c_double),
        );
        pdf_add_dict(
            catalog,
            pdf_new_name(b"Outlines\0" as *const u8 as *const ::core::ffi::c_char),
            bm_root_ref,
        );
        pdf_release_obj(bm_root);
    }
    clean_bookmarks(item);
    (*p).outlines.first = ::core::ptr::null_mut::<pdf_olitem>();
    (*p).outlines.current = ::core::ptr::null_mut::<pdf_olitem>();
    (*p).outlines.current_depth = 0 as ::core::ffi::c_int;
}
const name_dict_categories: [*const ::core::ffi::c_char; 10] = [
    b"Dests\0" as *const u8 as *const ::core::ffi::c_char,
    b"AP\0" as *const u8 as *const ::core::ffi::c_char,
    b"JavaScript\0" as *const u8 as *const ::core::ffi::c_char,
    b"Pages\0" as *const u8 as *const ::core::ffi::c_char,
    b"Templates\0" as *const u8 as *const ::core::ffi::c_char,
    b"IDS\0" as *const u8 as *const ::core::ffi::c_char,
    b"URLS\0" as *const u8 as *const ::core::ffi::c_char,
    b"EmbeddedFiles\0" as *const u8 as *const ::core::ffi::c_char,
    b"AlternatePresentations\0" as *const u8 as *const ::core::ffi::c_char,
    b"Renditions\0" as *const u8 as *const ::core::ffi::c_char,
];
pub const NUM_NAME_CATEGORY: usize = (::core::mem::size_of::<[*const ::core::ffi::c_char; 10]>()
    as usize)
    .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize);
unsafe extern "C-unwind" fn pdf_doc_init_names(
    mut p: *mut pdf_doc,
    mut check_gotos: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_uint = 0;
    (*p).root.names = ::core::ptr::null_mut::<pdf_obj>();
    (*p).names = new(
        ((::core::mem::size_of::<[*const ::core::ffi::c_char; 10]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
            .wrapping_add(1 as usize) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<name_dict>() as usize) as uint32_t,
    ) as *mut name_dict;
    i = 0 as ::core::ffi::c_uint;
    while (i as usize) < NUM_NAME_CATEGORY {
        let ref mut fresh0 = (*(*p).names.offset(i as isize)).category;
        *fresh0 = name_dict_categories[i as usize];
        let ref mut fresh1 = (*(*p).names.offset(i as isize)).data;
        *fresh1 = if strcmp(
            name_dict_categories[i as usize],
            b"Dests\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        {
            ::core::ptr::null_mut::<ht_table>()
        } else {
            pdf_new_name_tree()
        };
        i = i.wrapping_add(1);
    }
    let ref mut fresh2 = (*(*p).names.offset(NUM_NAME_CATEGORY as isize)).category;
    *fresh2 = ::core::ptr::null::<::core::ffi::c_char>();
    let ref mut fresh3 = (*(*p).names.offset(NUM_NAME_CATEGORY as isize)).data;
    *fresh3 = ::core::ptr::null_mut::<ht_table>();
    (*p).check_gotos = check_gotos;
    ht_init_table(&raw mut (*p).gotos, Some(release_name_object));
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_add_names")]
pub unsafe extern "C-unwind" fn pdf_doc_add_names(
    mut category: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_void,
    mut keylen: ::core::ffi::c_int,
    mut value: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while !(*(*p).names.offset(i as isize)).category.is_null() {
        if streq_ptr((*(*p).names.offset(i as isize)).category, category) {
            break;
        }
        i = i.wrapping_add(1);
    }
    if (*(*p).names.offset(i as isize)).category.is_null() {
        pdf_warning!(
            b"Unknown name dictionary category \"%s\".\0" as *const u8
                as *const ::core::ffi::c_char,
            category,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*(*p).names.offset(i as isize)).data.is_null() {
        let ref mut fresh17 = (*(*p).names.offset(i as isize)).data;
        *fresh17 = pdf_new_name_tree();
    }
    return pdf_names_add_object((*(*p).names.offset(i as isize)).data, key, keylen, value);
}
unsafe extern "C-unwind" fn pdf_doc_add_goto(mut annot_dict: *mut pdf_obj) {
    let __pdf_state = unsafe { document_state() };
    let mut current_block: u64;
    let mut subtype: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut A: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut S: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut D: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut D_new: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut dest: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut key: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut destlen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*__pdf_state).pdoc.check_gotos == 0 {
        return;
    }
    subtype = pdf_deref_obj(pdf_lookup_dict(
        annot_dict,
        b"Subtype\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    if !subtype.is_null() {
        if !subtype.is_null() && pdf_obj_typeof(subtype) == PDF_UNDEFINED {
            current_block = 8964283831476451391;
        } else if !(!subtype.is_null() && pdf_obj_typeof(subtype) == PDF_NAME) {
            current_block = 8430929072491527990;
        } else if strcmp(
            pdf_name_value(subtype),
            b"Link\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        {
            current_block = 693908575666729868;
        } else {
            current_block = 6937071982253665452;
        }
    } else {
        current_block = 6937071982253665452;
    }
    match current_block {
        6937071982253665452 => {
            dict = annot_dict;
            key = b"Dest\0" as *const u8 as *const ::core::ffi::c_char;
            D = pdf_deref_obj(pdf_lookup_dict(annot_dict, key));
            if !D.is_null() && pdf_obj_typeof(D) == PDF_UNDEFINED {
                current_block = 8964283831476451391;
            } else {
                A = pdf_deref_obj(pdf_lookup_dict(
                    annot_dict,
                    b"A\0" as *const u8 as *const ::core::ffi::c_char,
                ));
                if !A.is_null() {
                    if !A.is_null() && pdf_obj_typeof(A) == PDF_UNDEFINED {
                        current_block = 8964283831476451391;
                    } else if !D.is_null() || !(!A.is_null() && pdf_obj_typeof(A) == PDF_DICT) {
                        current_block = 8430929072491527990;
                    } else {
                        S = pdf_deref_obj(pdf_lookup_dict(
                            A,
                            b"S\0" as *const u8 as *const ::core::ffi::c_char,
                        ));
                        if !S.is_null() && pdf_obj_typeof(S) == PDF_UNDEFINED {
                            current_block = 8964283831476451391;
                        } else if !(!S.is_null() && pdf_obj_typeof(S) == PDF_NAME) {
                            current_block = 8430929072491527990;
                        } else if strcmp(
                            pdf_name_value(S),
                            b"GoTo\0" as *const u8 as *const ::core::ffi::c_char,
                        ) != 0
                        {
                            current_block = 693908575666729868;
                        } else {
                            dict = A;
                            key = b"D\0" as *const u8 as *const ::core::ffi::c_char;
                            D = pdf_deref_obj(pdf_lookup_dict(A, key));
                            current_block = 4495394744059808450;
                        }
                    }
                } else {
                    current_block = 4495394744059808450;
                }
                match current_block {
                    8964283831476451391 => {}
                    8430929072491527990 => {}
                    693908575666729868 => {}
                    _ => {
                        if !D.is_null() && pdf_obj_typeof(D) == PDF_STRING {
                            dest = pdf_string_value(D) as *mut ::core::ffi::c_char;
                            destlen = pdf_string_length(D) as ::core::ffi::c_int;
                            D_new = ht_lookup_table(
                                &raw mut (*__pdf_state).pdoc.gotos,
                                dest as *const ::core::ffi::c_void,
                                destlen,
                            ) as *mut pdf_obj;
                            if D_new.is_null() {
                                let mut buf: [::core::ffi::c_char; 10] = [0; 10];
                                sprintf(
                                    &raw mut buf as *mut ::core::ffi::c_char,
                                    b"%x\0" as *const u8 as *const ::core::ffi::c_char,
                                    ht_table_size(&raw mut (*__pdf_state).pdoc.gotos),
                                );
                                D_new = pdf_new_string(
                                    &raw mut buf as *mut ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                    strlen(&raw mut buf as *mut ::core::ffi::c_char),
                                );
                                ht_append_table(
                                    &raw mut (*__pdf_state).pdoc.gotos,
                                    dest as *const ::core::ffi::c_void,
                                    destlen,
                                    D_new as *mut ::core::ffi::c_void,
                                );
                            }
                            let mut key_obj: *mut pdf_obj = pdf_new_name(key);
                            if pdf_add_dict(dict, key_obj, pdf_link_obj(D_new)) == 0 {
                                pdf_release_obj(key_obj);
                            }
                            current_block = 693908575666729868;
                        } else if !D.is_null() && pdf_obj_typeof(D) == PDF_ARRAY {
                            current_block = 693908575666729868;
                        } else if !D.is_null() && pdf_obj_typeof(D) == PDF_UNDEFINED {
                            current_block = 8964283831476451391;
                        } else {
                            current_block = 8430929072491527990;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        8964283831476451391 => {
            pdf_warning!(
                b"Cannot optimize PDF annotations. Output file may be broken. Please restart with option \"-C 0x10\"\n\0"
                as * const u8 as * const ::core::ffi::c_char,
            );
        }
        8430929072491527990 => {
            pdf_warning!(
                b"Unknown PDF annotation format. Output file may be broken.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        _ => {}
    }
    pdf_release_obj(subtype);
    pdf_release_obj(A);
    pdf_release_obj(S);
    pdf_release_obj(D);
}
unsafe extern "C-unwind" fn warn_undef_dests(mut dests: *mut ht_table, mut gotos: *mut ht_table) {
    let mut iter: ht_iter = ht_iter {
        index: 0,
        curr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        hash: ::core::ptr::null_mut::<ht_table>(),
    };
    if ht_set_iter(gotos, &raw mut iter) < 0 as ::core::ffi::c_int {
        return;
    }
    loop {
        let mut keylen: ::core::ffi::c_int = 0;
        let mut key: *mut ::core::ffi::c_char = ht_iter_getkey(&raw mut iter, &raw mut keylen);
        if ht_lookup_table(dests, key as *const ::core::ffi::c_void, keylen).is_null() {
            let mut dest: *mut ::core::ffi::c_char =
                new(((keylen + 1 as ::core::ffi::c_int) as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                    as uint32_t) as *mut ::core::ffi::c_char;
            memcpy(
                dest as *mut ::core::ffi::c_void,
                key as *const ::core::ffi::c_void,
                keylen as size_t,
            );
            *dest.offset(keylen as isize) = 0 as ::core::ffi::c_char;
            pdf_warning!(
                b"PDF destination \"%s\" not defined.\0" as *const u8 as *const ::core::ffi::c_char,
                dest,
            );
            free(dest as *mut ::core::ffi::c_void);
        }
        if !(ht_iter_next(&raw mut iter) >= 0 as ::core::ffi::c_int) {
            break;
        }
    }
    ht_clear_iter(&raw mut iter);
}
unsafe extern "C-unwind" fn pdf_doc_close_names(mut p: *mut pdf_doc) {
    let __pdf_state = unsafe { document_state() };
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while !(*(*p).names.offset(i as isize)).category.is_null() {
        if !(*(*p).names.offset(i as isize)).data.is_null() {
            let mut data: *mut ht_table = (*(*p).names.offset(i as isize)).data;
            let mut name_tree: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            let mut count: ::core::ffi::c_int = 0;
            if (*__pdf_state).pdoc.check_gotos == 0
                || strcmp(
                    (*(*p).names.offset(i as isize)).category,
                    b"Dests\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
            {
                name_tree = pdf_names_create_tree(
                    data,
                    &raw mut count,
                    ::core::ptr::null_mut::<ht_table>(),
                );
            } else {
                name_tree =
                    pdf_names_create_tree(data, &raw mut count, &raw mut (*__pdf_state).pdoc.gotos);
                if dpx_conf.verbose_level > 0 as ::core::ffi::c_int && count < (*data).count {
                    pdf_message!(
                        b"\nRemoved %d unused PDF destinations\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*data).count - count,
                    );
                }
                if count < (*__pdf_state).pdoc.gotos.count {
                    warn_undef_dests(data, &raw mut (*__pdf_state).pdoc.gotos);
                }
            }
            if !name_tree.is_null() {
                if (*p).root.names.is_null() {
                    (*p).root.names = pdf_new_dict();
                }
                pdf_add_dict(
                    (*p).root.names,
                    pdf_new_name((*(*p).names.offset(i as isize)).category),
                    pdf_ref_obj(name_tree),
                );
                pdf_release_obj(name_tree);
            }
            pdf_delete_name_tree(&raw mut (*(*p).names.offset(i as isize)).data);
        }
        i = i.wrapping_add(1);
    }
    if !(*p).root.names.is_null() {
        tmp = pdf_lookup_dict(
            (*p).root.dict,
            b"Names\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if tmp.is_null() {
            pdf_add_dict(
                (*p).root.dict,
                pdf_new_name(b"Names\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_ref_obj((*p).root.names),
            );
        } else if !tmp.is_null() && pdf_obj_typeof(tmp) == PDF_DICT {
            pdf_merge_dict((*p).root.names, tmp);
            pdf_add_dict(
                (*p).root.dict,
                pdf_new_name(b"Names\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_ref_obj((*p).root.names),
            );
        } else {
            pdf_warning!(
                b"Could not modify Names dictionary.\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        pdf_release_obj((*p).root.names);
        (*p).root.names = ::core::ptr::null_mut::<pdf_obj>();
    }
    (*p).names = mfree((*p).names as *mut ::core::ffi::c_void) as *mut name_dict;
    ht_clear_table(&raw mut (*p).gotos);
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_add_annot")]
pub unsafe extern "C-unwind" fn pdf_doc_add_annot(
    mut page_no: ::core::ffi::c_uint,
    mut rect: *const pdf_rect,
    mut annot_dict: *mut pdf_obj,
    mut new_annot: ::core::ffi::c_int,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut page: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
    let mut rect_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    page = doc_get_page_entry(p, page_no);
    if (*page).annots.is_null() {
        (*page).annots = pdf_new_array();
    }
    let mut mediabox: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    pdf_doc_get_mediabox(page_no, &raw mut mediabox);
    if (*rect).llx < mediabox.llx
        || (*rect).urx > mediabox.urx
        || (*rect).lly < mediabox.lly
        || (*rect).ury > mediabox.ury
    {
        pdf_warning!(
            b"Annotation out of page boundary.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        pdf_warning!(
            b"Current page's MediaBox: [%g %g %g %g]\0" as *const u8 as *const ::core::ffi::c_char,
            mediabox.llx,
            mediabox.lly,
            mediabox.urx,
            mediabox.ury,
        );
        pdf_warning!(
            b"Annotation: [%g %g %g %g]\0" as *const u8 as *const ::core::ffi::c_char,
            (*rect).llx,
            (*rect).lly,
            (*rect).urx,
            (*rect).ury,
        );
        pdf_warning!(
            b"Maybe incorrect paper size specified.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*rect).llx > (*rect).urx || (*rect).lly > (*rect).ury {
        pdf_warning!(
            b"Rectangle with negative width/height: [%g %g %g %g]\0" as *const u8
                as *const ::core::ffi::c_char,
            (*rect).llx,
            (*rect).lly,
            (*rect).urx,
            (*rect).ury,
        );
    }
    rect_array = pdf_new_array();
    pdf_add_array(
        rect_array,
        pdf_new_number(floor((*rect).llx / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        rect_array,
        pdf_new_number(floor((*rect).lly / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        rect_array,
        pdf_new_number(floor((*rect).urx / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        rect_array,
        pdf_new_number(floor((*rect).ury / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_dict(
        annot_dict,
        pdf_new_name(b"Rect\0" as *const u8 as *const ::core::ffi::c_char),
        rect_array,
    );
    pdf_add_array((*page).annots, pdf_ref_obj(annot_dict));
    if new_annot != 0 {
        pdf_doc_add_goto(annot_dict);
    }
}
unsafe extern "C-unwind" fn pdf_doc_init_articles(mut p: *mut pdf_doc) {
    (*p).root.threads = ::core::ptr::null_mut::<pdf_obj>();
    (*p).articles.num_entries = 0 as ::core::ffi::c_uint;
    (*p).articles.max_entries = 0 as ::core::ffi::c_uint;
    (*p).articles.entries = ::core::ptr::null_mut::<pdf_article>();
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_begin_article")]
pub unsafe extern "C-unwind" fn pdf_doc_begin_article(
    mut article_id: *const ::core::ffi::c_char,
    mut article_info: *mut pdf_obj,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut article: *mut pdf_article = ::core::ptr::null_mut::<pdf_article>();
    if article_id.is_null() || strlen(article_id) == 0 as size_t {
        pdf_abort!(
            b"Article thread without internal identifier.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if (*p).articles.num_entries >= (*p).articles.max_entries {
        (*p).articles.max_entries = (*p)
            .articles
            .max_entries
            .wrapping_add(PDFDOC_ARTICLE_ALLOC_SIZE as ::core::ffi::c_uint);
        (*p).articles.entries = renew(
            (*p).articles.entries as *mut ::core::ffi::c_void,
            ((*p).articles.max_entries as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<pdf_article>() as usize)
                as uint32_t,
        ) as *mut pdf_article as *mut pdf_article;
    }
    article = (*p)
        .articles
        .entries
        .offset((*p).articles.num_entries as isize) as *mut pdf_article;
    (*article).id = new(
        (strlen(article_id).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*article).id, article_id);
    (*article).info = article_info;
    (*article).num_beads = 0 as ::core::ffi::c_uint;
    (*article).max_beads = 0 as ::core::ffi::c_uint;
    (*article).beads = ::core::ptr::null_mut::<pdf_bead>();
    (*p).articles.num_entries = (*p).articles.num_entries.wrapping_add(1);
}
unsafe extern "C-unwind" fn find_bead(
    mut article: *mut pdf_article,
    mut bead_id: *const ::core::ffi::c_char,
) -> *mut pdf_bead {
    let mut bead: *mut pdf_bead = ::core::ptr::null_mut::<pdf_bead>();
    let mut i: ::core::ffi::c_uint = 0;
    bead = ::core::ptr::null_mut::<pdf_bead>();
    i = 0 as ::core::ffi::c_uint;
    while i < (*article).num_beads {
        if streq_ptr((*(*article).beads.offset(i as isize)).id, bead_id) {
            bead = (*article).beads.offset(i as isize) as *mut pdf_bead;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    return bead;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_add_bead")]
pub unsafe extern "C-unwind" fn pdf_doc_add_bead(
    mut article_id: *const ::core::ffi::c_char,
    mut bead_id: *const ::core::ffi::c_char,
    mut page_no: ::core::ffi::c_int,
    mut rect: *const pdf_rect,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut article: *mut pdf_article = ::core::ptr::null_mut::<pdf_article>();
    let mut bead: *mut pdf_bead = ::core::ptr::null_mut::<pdf_bead>();
    let mut i: ::core::ffi::c_uint = 0;
    if article_id.is_null() {
        pdf_abort!(
            b"No article identifier specified.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    article = ::core::ptr::null_mut::<pdf_article>();
    i = 0 as ::core::ffi::c_uint;
    while i < (*p).articles.num_entries {
        if streq_ptr((*(*p).articles.entries.offset(i as isize)).id, article_id) {
            article = (*p).articles.entries.offset(i as isize) as *mut pdf_article;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if article.is_null() {
        pdf_abort!(
            b"Specified article thread that doesn't exist.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    bead = if !bead_id.is_null() {
        find_bead(article, bead_id)
    } else {
        ::core::ptr::null_mut::<pdf_bead>()
    };
    if bead.is_null() {
        if (*article).num_beads >= (*article).max_beads {
            (*article).max_beads = (*article)
                .max_beads
                .wrapping_add(PDFDOC_BEAD_ALLOC_SIZE as ::core::ffi::c_uint);
            (*article).beads = renew(
                (*article).beads as *mut ::core::ffi::c_void,
                ((*article).max_beads as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<pdf_bead>() as usize)
                    as uint32_t,
            ) as *mut pdf_bead as *mut pdf_bead;
            i = (*article).num_beads;
            while i < (*article).max_beads {
                let ref mut fresh18 = (*(*article).beads.offset(i as isize)).id;
                *fresh18 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                (*(*article).beads.offset(i as isize)).page_no = -(1 as ::core::ffi::c_int);
                i = i.wrapping_add(1);
            }
        }
        bead = (*article).beads.offset((*article).num_beads as isize) as *mut pdf_bead;
        if !bead_id.is_null() {
            (*bead).id = new(
                (strlen(bead_id).wrapping_add(1 as size_t) as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                    as uint32_t,
            ) as *mut ::core::ffi::c_char;
            strcpy((*bead).id, bead_id);
        } else {
            (*bead).id = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        (*article).num_beads = (*article).num_beads.wrapping_add(1);
    }
    (*bead).rect.llx = (*rect).llx;
    (*bead).rect.lly = (*rect).lly;
    (*bead).rect.urx = (*rect).urx;
    (*bead).rect.ury = (*rect).ury;
    (*bead).page_no = page_no;
}
unsafe extern "C-unwind" fn make_article(
    mut p: *mut pdf_doc,
    mut article: *mut pdf_article,
    mut bead_ids: *mut *const ::core::ffi::c_char,
    mut num_beads: ::core::ffi::c_uint,
    mut article_info: *mut pdf_obj,
) -> *mut pdf_obj {
    let mut art_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut first: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut prev: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut last: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if article.is_null() {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    art_dict = pdf_new_dict();
    last = ::core::ptr::null_mut::<pdf_obj>();
    prev = last;
    first = prev;
    n = (if !bead_ids.is_null() {
        num_beads
    } else {
        (*article).num_beads
    }) as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let mut bead: *mut pdf_bead = ::core::ptr::null_mut::<pdf_bead>();
        bead = if !bead_ids.is_null() {
            find_bead(article, *bead_ids.offset(i as isize))
        } else {
            (*article).beads.offset(i as isize) as *mut pdf_bead
        };
        if !(bead.is_null() || (*bead).page_no < 0 as ::core::ffi::c_int) {
            last = pdf_new_dict();
            if prev.is_null() {
                first = last;
                pdf_add_dict(
                    first,
                    pdf_new_name(b"T\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_ref_obj(art_dict),
                );
            } else {
                pdf_add_dict(
                    prev,
                    pdf_new_name(b"N\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_ref_obj(last),
                );
                pdf_add_dict(
                    last,
                    pdf_new_name(b"V\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_ref_obj(prev),
                );
                if prev != first {
                    pdf_release_obj(prev);
                }
            }
            let mut page: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
            let mut rect: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            page = doc_get_page_entry(p, (*bead).page_no as ::core::ffi::c_uint);
            if (*page).beads.is_null() {
                (*page).beads = pdf_new_array();
            }
            pdf_add_dict(
                last,
                pdf_new_name(b"P\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_link_obj((*page).page_ref),
            );
            rect = pdf_new_array();
            pdf_add_array(
                rect,
                pdf_new_number(floor((*bead).rect.llx / 0.01f64 + 0.5f64) * 0.01f64),
            );
            pdf_add_array(
                rect,
                pdf_new_number(floor((*bead).rect.lly / 0.01f64 + 0.5f64) * 0.01f64),
            );
            pdf_add_array(
                rect,
                pdf_new_number(floor((*bead).rect.urx / 0.01f64 + 0.5f64) * 0.01f64),
            );
            pdf_add_array(
                rect,
                pdf_new_number(floor((*bead).rect.ury / 0.01f64 + 0.5f64) * 0.01f64),
            );
            pdf_add_dict(
                last,
                pdf_new_name(b"R\0" as *const u8 as *const ::core::ffi::c_char),
                rect,
            );
            pdf_add_array((*page).beads, pdf_ref_obj(last));
            prev = last;
        }
        i += 1;
    }
    if !first.is_null() && !last.is_null() {
        pdf_add_dict(
            last,
            pdf_new_name(b"N\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(first),
        );
        pdf_add_dict(
            first,
            pdf_new_name(b"V\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(last),
        );
        if first != last {
            pdf_release_obj(last);
        }
        pdf_add_dict(
            art_dict,
            pdf_new_name(b"F\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(first),
        );
        if !article_info.is_null() {
            pdf_add_dict(
                art_dict,
                pdf_new_name(b"I\0" as *const u8 as *const ::core::ffi::c_char),
                article_info,
            );
        } else if !(*article).info.is_null() {
            pdf_add_dict(
                art_dict,
                pdf_new_name(b"I\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_ref_obj((*article).info),
            );
            pdf_release_obj((*article).info);
            (*article).info = ::core::ptr::null_mut::<pdf_obj>();
        }
        pdf_release_obj(first);
    } else {
        pdf_release_obj(art_dict);
        art_dict = ::core::ptr::null_mut::<pdf_obj>();
    }
    return art_dict;
}
unsafe extern "C-unwind" fn clean_article(mut article: *mut pdf_article) {
    if article.is_null() {
        return;
    }
    if !(*article).beads.is_null() {
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < (*article).num_beads {
            free((*(*article).beads.offset(i as isize)).id as *mut ::core::ffi::c_void);
            i = i.wrapping_add(1);
        }
        (*article).beads = mfree((*article).beads as *mut ::core::ffi::c_void) as *mut pdf_bead;
    }
    (*article).id = mfree((*article).id as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
    (*article).num_beads = 0 as ::core::ffi::c_uint;
    (*article).max_beads = 0 as ::core::ffi::c_uint;
}
unsafe extern "C-unwind" fn pdf_doc_close_articles(mut p: *mut pdf_doc) {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (*p).articles.num_entries {
        let mut article: *mut pdf_article = ::core::ptr::null_mut::<pdf_article>();
        article = (*p).articles.entries.offset(i as isize) as *mut pdf_article;
        if !(*article).beads.is_null() {
            let mut art_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            art_dict = make_article(
                p,
                article,
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                0 as ::core::ffi::c_uint,
                ::core::ptr::null_mut::<pdf_obj>(),
            );
            if (*p).root.threads.is_null() {
                (*p).root.threads = pdf_new_array();
            }
            pdf_add_array((*p).root.threads, pdf_ref_obj(art_dict));
            pdf_release_obj(art_dict);
        }
        clean_article(article);
        i = i.wrapping_add(1);
    }
    (*p).articles.entries =
        mfree((*p).articles.entries as *mut ::core::ffi::c_void) as *mut pdf_article;
    (*p).articles.num_entries = 0 as ::core::ffi::c_uint;
    (*p).articles.max_entries = 0 as ::core::ffi::c_uint;
    if !(*p).root.threads.is_null() {
        pdf_add_dict(
            (*p).root.dict,
            pdf_new_name(b"Threads\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj((*p).root.threads),
        );
        pdf_release_obj((*p).root.threads);
        (*p).root.threads = ::core::ptr::null_mut::<pdf_obj>();
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_set_mediabox")]
pub unsafe extern "C-unwind" fn pdf_doc_set_mediabox(
    mut page_no: ::core::ffi::c_uint,
    mut mediabox: *const pdf_rect,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut page: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
    if page_no == 0 as ::core::ffi::c_uint {
        (*p).pages.mediabox.llx = (*mediabox).llx;
        (*p).pages.mediabox.lly = (*mediabox).lly;
        (*p).pages.mediabox.urx = (*mediabox).urx;
        (*p).pages.mediabox.ury = (*mediabox).ury;
    } else {
        page = doc_get_page_entry(p, page_no);
        (*page).cropbox.llx = (*mediabox).llx;
        (*page).cropbox.lly = (*mediabox).lly;
        (*page).cropbox.urx = (*mediabox).urx;
        (*page).cropbox.ury = (*mediabox).ury;
        (*page).flags |= USE_MY_MEDIABOX;
    };
}
unsafe extern "C-unwind" fn pdf_doc_get_mediabox(
    mut page_no: ::core::ffi::c_uint,
    mut mediabox: *mut pdf_rect,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut page: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
    if page_no == 0 as ::core::ffi::c_uint {
        (*mediabox).llx = (*p).pages.mediabox.llx;
        (*mediabox).lly = (*p).pages.mediabox.lly;
        (*mediabox).urx = (*p).pages.mediabox.urx;
        (*mediabox).ury = (*p).pages.mediabox.ury;
    } else {
        page = doc_get_page_entry(p, page_no);
        if (*page).flags & USE_MY_MEDIABOX != 0 {
            (*mediabox).llx = (*page).cropbox.llx;
            (*mediabox).lly = (*page).cropbox.lly;
            (*mediabox).urx = (*page).cropbox.urx;
            (*mediabox).ury = (*page).cropbox.ury;
        } else {
            (*mediabox).llx = (*p).pages.mediabox.llx;
            (*mediabox).lly = (*p).pages.mediabox.lly;
            (*mediabox).urx = (*p).pages.mediabox.urx;
            (*mediabox).ury = (*p).pages.mediabox.ury;
        }
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_current_page_resources")]
pub unsafe extern "C-unwind" fn pdf_doc_current_page_resources() -> *mut pdf_obj {
    let __pdf_state = unsafe { document_state() };
    let mut resources: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut currentpage: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
    if !(*p).pending_forms.is_null() {
        if !(*(*p).pending_forms).form.resources.is_null() {
            resources = (*(*p).pending_forms).form.resources;
        } else {
            (*(*p).pending_forms).form.resources = pdf_new_dict();
            resources = (*(*p).pending_forms).form.resources;
        }
    } else {
        currentpage = (*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
        if !(*currentpage).resources.is_null() {
            resources = (*currentpage).resources;
        } else {
            (*currentpage).resources = pdf_new_dict();
            resources = (*currentpage).resources;
        }
    }
    return resources;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_get_dictionary")]
pub unsafe extern "C-unwind" fn pdf_doc_get_dictionary(
    mut category: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if category.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"category\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if streq_ptr(
        category,
        b"Names\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if (*p).root.names.is_null() {
            (*p).root.names = pdf_new_dict();
        }
        dict = (*p).root.names;
    } else if streq_ptr(
        category,
        b"Pages\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if (*p).root.pages.is_null() {
            (*p).root.pages = pdf_new_dict();
        }
        dict = (*p).root.pages;
    } else if streq_ptr(
        category,
        b"Catalog\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if (*p).root.dict.is_null() {
            (*p).root.dict = pdf_new_dict();
        }
        dict = (*p).root.dict;
    } else if streq_ptr(
        category,
        b"Info\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if (*p).info.is_null() {
            (*p).info = pdf_new_dict();
        }
        dict = (*p).info;
    } else if streq_ptr(
        category,
        b"@THISPAGE\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        let mut currentpage: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
        currentpage = (*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
        dict = (*currentpage).page_obj;
    }
    if dict.is_null() {
        pdf_abort!(
            b"Document dict. \"%s\" not exist. \0" as *const u8 as *const ::core::ffi::c_char,
            category,
        );
    }
    return dict;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_current_page_number")]
pub unsafe extern "C-unwind" fn pdf_doc_current_page_number() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    return (*p)
        .pages
        .num_entries
        .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_ref_page")]
pub unsafe extern "C-unwind" fn pdf_doc_ref_page(mut page_no: ::core::ffi::c_uint) -> *mut pdf_obj {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut page: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
    page = doc_get_page_entry(p, page_no);
    if (*page).page_obj.is_null() {
        (*page).page_obj = pdf_new_dict();
        (*page).page_ref = pdf_ref_obj((*page).page_obj);
    }
    return pdf_link_obj((*page).page_ref);
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_get_reference")]
pub unsafe extern "C-unwind" fn pdf_doc_get_reference(
    mut category: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut ref_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut page_no: ::core::ffi::c_int = 0;
    if category.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"category\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    page_no = pdf_doc_current_page_number();
    if streq_ptr(
        category,
        b"@THISPAGE\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        ref_0 = pdf_doc_ref_page(page_no as ::core::ffi::c_uint);
    } else if streq_ptr(
        category,
        b"@PREVPAGE\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if page_no <= 1 as ::core::ffi::c_int {
            pdf_abort!(
                b"Reference to previous page, but no pages have been completed yet.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        ref_0 = pdf_doc_ref_page((page_no - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
    } else if streq_ptr(
        category,
        b"@NEXTPAGE\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        ref_0 = pdf_doc_ref_page((page_no + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
    }
    if ref_0.is_null() {
        pdf_abort!(
            b"Reference to \"%s\" not exist. \0" as *const u8 as *const ::core::ffi::c_char,
            category,
        );
    }
    return ref_0;
}
unsafe extern "C-unwind" fn pdf_doc_new_page(mut p: *mut pdf_doc) {
    let mut currentpage: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
    if (*p).pages.num_entries >= (*p).pages.max_entries {
        doc_resize_page_entries(
            p,
            (*p).pages.max_entries.wrapping_add(PDFDOC_PAGES_ALLOC_SIZE),
        );
    }
    currentpage = (*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
    if (*currentpage).page_ref.is_null() {
        (*currentpage).page_obj = pdf_new_dict();
        (*currentpage).page_ref = pdf_ref_obj((*currentpage).page_obj);
    }
    (*currentpage).background = ::core::ptr::null_mut::<pdf_obj>();
    (*currentpage).contents = pdf_new_stream(STREAM_COMPRESS | STREAM_PAGE_CONTENT);
    (*currentpage).resources = pdf_new_dict();
    (*currentpage).render_pending = 0 as ::core::ffi::c_int;
    (*currentpage).annots = ::core::ptr::null_mut::<pdf_obj>();
    (*currentpage).beads = ::core::ptr::null_mut::<pdf_obj>();
}
unsafe extern "C-unwind" fn pdf_doc_finish_page(mut p: *mut pdf_doc) {
    let mut currentpage: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
    if !(*p).pending_forms.is_null() {
        pdf_abort!(
            b"A pending form XObject at the end of page.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    currentpage = (*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
    if (*currentpage).page_obj.is_null() {
        (*currentpage).page_obj = pdf_new_dict();
    }
    if !(*p).pages.bop.is_null() && pdf_stream_length((*p).pages.bop) > 0 as ::core::ffi::c_int {
        (*currentpage).content_refs[0 as ::core::ffi::c_int as usize] = pdf_ref_obj((*p).pages.bop);
    } else {
        (*currentpage).content_refs[0 as ::core::ffi::c_int as usize] =
            ::core::ptr::null_mut::<pdf_obj>();
    }
    if !(*currentpage).background.is_null() {
        if pdf_stream_length((*currentpage).background) > 0 as ::core::ffi::c_int {
            (*currentpage).content_refs[1 as ::core::ffi::c_int as usize] =
                pdf_ref_obj((*currentpage).background);
            pdf_add_stream(
                (*currentpage).background,
                b"\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                1 as ::core::ffi::c_int,
            );
        }
        pdf_release_obj((*currentpage).background);
        (*currentpage).background = ::core::ptr::null_mut::<pdf_obj>();
    } else {
        (*currentpage).content_refs[1 as ::core::ffi::c_int as usize] =
            ::core::ptr::null_mut::<pdf_obj>();
    }
    (*currentpage).content_refs[2 as ::core::ffi::c_int as usize] =
        pdf_ref_obj((*currentpage).contents);
    if (*currentpage).render_pending == 0 {
        pdf_add_stream(
            (*currentpage).contents,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            1 as ::core::ffi::c_int,
        );
    }
    pdf_release_obj((*currentpage).contents);
    (*currentpage).contents = ::core::ptr::null_mut::<pdf_obj>();
    if !(*p).pages.eop.is_null() && pdf_stream_length((*p).pages.eop) > 0 as ::core::ffi::c_int {
        (*currentpage).content_refs[3 as ::core::ffi::c_int as usize] = pdf_ref_obj((*p).pages.eop);
    } else {
        (*currentpage).content_refs[3 as ::core::ffi::c_int as usize] =
            ::core::ptr::null_mut::<pdf_obj>();
    }
    if !(*currentpage).resources.is_null() {
        let mut procset: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        procset = pdf_new_array();
        pdf_add_array(
            procset,
            pdf_new_name(b"PDF\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_array(
            procset,
            pdf_new_name(b"Text\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_array(
            procset,
            pdf_new_name(b"ImageC\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_array(
            procset,
            pdf_new_name(b"ImageB\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_array(
            procset,
            pdf_new_name(b"ImageI\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_dict(
            (*currentpage).resources,
            pdf_new_name(b"ProcSet\0" as *const u8 as *const ::core::ffi::c_char),
            procset,
        );
        pdf_add_dict(
            (*currentpage).page_obj,
            pdf_new_name(b"Resources\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj((*currentpage).resources),
        );
        pdf_release_obj((*currentpage).resources);
        (*currentpage).resources = ::core::ptr::null_mut::<pdf_obj>();
    }
    if (*p).options.enable_manual_thumb != 0 {
        let mut thumb_filename: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut thumb_ref: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        thumb_filename = new(
            (strlen((*p).thumb_basename).wrapping_add(7 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        sprintf(
            thumb_filename,
            b"%s.%ld\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).thumb_basename,
            (*p).pages
                .num_entries
                .wrapping_rem(99999 as ::core::ffi::c_int as ::core::ffi::c_uint)
                as ::core::ffi::c_long
                + 1 as ::core::ffi::c_long,
        );
        thumb_ref = read_thumbnail(thumb_filename);
        free(thumb_filename as *mut ::core::ffi::c_void);
        if !thumb_ref.is_null() {
            pdf_add_dict(
                (*currentpage).page_obj,
                pdf_new_name(b"Thumb\0" as *const u8 as *const ::core::ffi::c_char),
                thumb_ref,
            );
        }
    }
    (*p).pages.num_entries = (*p).pages.num_entries.wrapping_add(1);
}

#[unsafe(export_name = "__pdf_impl_pdf_doc_set_bgcolor")]
pub unsafe extern "C-unwind" fn pdf_doc_set_bgcolor(mut color: *const pdf_color) {
    let __pdf_state = unsafe { document_state() };
    if !color.is_null() {
        pdf_color_copycolor(&raw mut (*__pdf_state).bgcolor, color);
    } else {
        pdf_color_graycolor(&raw mut (*__pdf_state).bgcolor, 1.0f64);
    };
}
unsafe extern "C-unwind" fn doc_fill_page_background(mut p: *mut pdf_doc) {
    let __pdf_state = unsafe { document_state() };
    let mut currentpage: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut cm: ::core::ffi::c_int = 0;
    let mut saved_content: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    cm = pdf_dev_get_param(PDF_DEV_PARAM_COLORMODE);
    if cm == 0 || pdf_color_is_white(&raw mut (*__pdf_state).bgcolor) as ::core::ffi::c_int != 0 {
        return;
    }
    pdf_doc_get_mediabox(
        pdf_doc_current_page_number() as ::core::ffi::c_uint,
        &raw mut r,
    );
    currentpage = (*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
    if currentpage.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"currentpage\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*currentpage).background.is_null() {
        (*currentpage).background = pdf_new_stream(STREAM_COMPRESS);
    }
    saved_content = (*currentpage).contents;
    (*currentpage).contents = (*currentpage).background;
    pdf_dev_gsave();
    pdf_dev_set_color(
        &raw mut (*__pdf_state).bgcolor,
        0x20 as ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    pdf_dev_rectfill(r.llx, r.lly, r.urx - r.llx, r.ury - r.lly);
    pdf_dev_grestore();
    (*currentpage).contents = saved_content;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_begin_page")]
pub unsafe extern "C-unwind" fn pdf_doc_begin_page(
    mut scale: ::core::ffi::c_double,
    mut x_origin: ::core::ffi::c_double,
    mut y_origin: ::core::ffi::c_double,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    M.a = scale;
    M.b = 0.0f64;
    M.c = 0.0f64;
    M.d = scale;
    M.e = x_origin;
    M.f = y_origin;
    pdf_doc_new_page(p);
    pdf_dev_begin_page_plan();
    pdf_dev_bop(&raw mut M);
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_end_page")]
pub unsafe extern "C-unwind" fn pdf_doc_end_page() {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    pdf_dev_eop();
    if tt_pdf_page_recording() != 0 {
        let mut page: *mut pdf_page =
            (*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
        while tt_pdf_pipeline_has_capacity(tt_pdf_page_size()) == 0 {
            pdf_pipeline_drain(0 as ::core::ffi::c_int);
        }
        (*page).render_pending = pdf_submit_page((*page).contents);
        if (*page).render_pending == 0 {
            pdf_abort!(b"Could not submit PDF page\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    pdf_dev_end_page_plan();
    doc_fill_page_background(p);
    pdf_doc_finish_page(p);
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_add_page_content")]
pub unsafe extern "C-unwind" fn pdf_doc_add_page_content(
    mut buffer: *const ::core::ffi::c_char,
    mut length: ::core::ffi::c_uint,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut currentpage: *mut pdf_page = ::core::ptr::null_mut::<pdf_page>();
    if tt_pdf_page_bytes(buffer, length as size_t) != 0 {
        return;
    }
    if !(*p).pending_forms.is_null() {
        pdf_add_stream(
            (*(*p).pending_forms).form.contents,
            buffer as *const ::core::ffi::c_void,
            length as ::core::ffi::c_int,
        );
    } else {
        currentpage = (*p).pages.entries.offset((*p).pages.num_entries as isize) as *mut pdf_page;
        pdf_add_stream(
            (*currentpage).contents,
            buffer as *const ::core::ffi::c_void,
            length as ::core::ffi::c_int,
        );
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_open_document")]
pub unsafe extern "C-unwind" fn pdf_open_document(
    mut filename: *const ::core::ffi::c_char,
    mut creator: *const ::core::ffi::c_char,
    mut id1: *const ::core::ffi::c_uchar,
    mut id2: *const ::core::ffi::c_uchar,
    mut settings: pdf_setting,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    pdf_out_init(
        filename,
        id1,
        id2,
        settings.ver_major,
        settings.ver_minor,
        settings.object.compression_level,
        settings.enable_encrypt,
        settings.object.enable_objstm,
        settings.object.enable_predictor,
    );
    pdf_files_init();
    pdf_doc_init_catalog(p);
    if settings.enable_encrypt != 0 {
        pdf_out_set_encrypt(
            settings.encrypt.key_size,
            settings.encrypt.permission as int32_t,
            settings.encrypt.oplain,
            settings.encrypt.uplain,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    }
    (*p).options.annot_grow.x = settings.annot_grow_amount.x;
    (*p).options.annot_grow.y = settings.annot_grow_amount.y;
    (*p).options.outline_open_depth = settings.outline_open_depth;
    pdf_init_resources();
    pdf_init_colors();
    pdf_init_fonts();
    pdf_init_images();
    pdf_doc_init_docinfo(p);
    if !creator.is_null() {
        pdf_add_dict(
            (*p).info,
            pdf_new_name(b"Creator\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(creator as *const ::core::ffi::c_void, strlen(creator)),
        );
    }
    pdf_doc_init_bookmarks(p, settings.outline_open_depth);
    pdf_doc_init_articles(p);
    pdf_doc_init_names(p, settings.check_gotos);
    pdf_doc_init_page_tree(p, settings.media_width, settings.media_height);
    pdf_doc_set_bgcolor(::core::ptr::null::<pdf_color>());
    (*p).options.enable_manual_thumb = settings.enable_manual_thumb;
    if (*p).options.enable_manual_thumb != 0 {
        let mut fn_len: size_t = strlen(filename);
        if fn_len > 4 as size_t
            && strncmp(
                b".pdf\0" as *const u8 as *const ::core::ffi::c_char,
                filename
                    .offset(fn_len as isize)
                    .offset(-(4 as ::core::ffi::c_int as isize)),
                4 as size_t,
            ) == 0
        {
            (*p).thumb_basename = new((fn_len.wrapping_sub(4 as size_t).wrapping_add(1 as size_t)
                as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            strncpy(
                (*p).thumb_basename,
                filename,
                fn_len.wrapping_sub(4 as size_t),
            );
            *(*p)
                .thumb_basename
                .offset(fn_len.wrapping_sub(4 as size_t) as isize) = 0 as ::core::ffi::c_char;
        } else {
            (*p).thumb_basename = new((fn_len.wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            strcpy((*p).thumb_basename, filename);
        }
    }
    (*p).pending_forms = ::core::ptr::null_mut::<form_list_node>();
    pdf_init_device(
        settings.device.dvi2pts,
        settings.device.precision,
        settings.device.ignore_colors,
    );
    global_names = pdf_new_name_tree();
}
#[unsafe(export_name = "__pdf_impl_pdf_close_document")]
pub unsafe extern "C-unwind" fn pdf_close_document() {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    pdf_pipeline_drain(1 as ::core::ffi::c_int);
    pdf_delete_name_tree(&raw mut global_names);
    pdf_close_device();
    pdf_doc_close_articles(p);
    pdf_doc_close_names(p);
    pdf_doc_close_bookmarks(p);
    pdf_doc_close_page_tree(p);
    pdf_doc_close_docinfo(p);
    pdf_doc_close_catalog(p);
    pdf_close_images();
    pdf_close_fonts();
    pdf_close_colors();
    pdf_close_resources();
    pdf_files_close();
    pdf_out_flush();
    free((*p).thumb_basename as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn pdf_doc_make_xform(
    mut xform: *mut pdf_obj,
    mut bbox: *mut pdf_rect,
    mut matrix: *mut pdf_tmatrix,
    mut resources: *mut pdf_obj,
    mut attrib: *mut pdf_obj,
) {
    let mut xform_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    xform_dict = pdf_stream_dict(xform);
    pdf_add_dict(
        xform_dict,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"XObject\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        xform_dict,
        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Form\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        xform_dict,
        pdf_new_name(b"FormType\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(1.0f64),
    );
    if bbox.is_null() {
        pdf_abort!(b"No BoundingBox supplied.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    tmp = pdf_new_array();
    pdf_add_array(
        tmp,
        pdf_new_number(floor((*bbox).llx / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        tmp,
        pdf_new_number(floor((*bbox).lly / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        tmp,
        pdf_new_number(floor((*bbox).urx / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_array(
        tmp,
        pdf_new_number(floor((*bbox).ury / 0.001f64 + 0.5f64) * 0.001f64),
    );
    pdf_add_dict(
        xform_dict,
        pdf_new_name(b"BBox\0" as *const u8 as *const ::core::ffi::c_char),
        tmp,
    );
    if !matrix.is_null() {
        tmp = pdf_new_array();
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).a / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).b / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).c / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).d / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).e / 0.001f64 + 0.5f64) * 0.001f64),
        );
        pdf_add_array(
            tmp,
            pdf_new_number(floor((*matrix).f / 0.001f64 + 0.5f64) * 0.001f64),
        );
        pdf_add_dict(
            xform_dict,
            pdf_new_name(b"Matrix\0" as *const u8 as *const ::core::ffi::c_char),
            tmp,
        );
    }
    if !attrib.is_null() {
        pdf_merge_dict(xform_dict, attrib);
    }
    pdf_add_dict(
        xform_dict,
        pdf_new_name(b"Resources\0" as *const u8 as *const ::core::ffi::c_char),
        resources,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_begin_grabbing")]
pub unsafe extern "C-unwind" fn pdf_doc_begin_grabbing(
    mut ident: *const ::core::ffi::c_char,
    mut ref_x: ::core::ffi::c_double,
    mut ref_y: ::core::ffi::c_double,
    mut cropbox: *const pdf_rect,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut xobj_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut form: *mut pdf_form = ::core::ptr::null_mut::<pdf_form>();
    let mut fnode: *mut form_list_node = ::core::ptr::null_mut::<form_list_node>();
    let mut info: xform_info = xform_info {
        flags: 0,
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
    };
    pdf_dev_push_gstate();
    fnode = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<form_list_node>() as usize)
        as uint32_t) as *mut form_list_node;
    (*fnode).prev = (*p).pending_forms;
    (*fnode).q_depth = pdf_dev_current_depth();
    form = &raw mut (*fnode).form;
    (*form).matrix.a = 1.0f64;
    (*form).matrix.b = 0.0f64;
    (*form).matrix.c = 0.0f64;
    (*form).matrix.d = 1.0f64;
    (*form).matrix.e = -ref_x;
    (*form).matrix.f = -ref_y;
    (*form).cropbox.llx = ref_x + (*cropbox).llx;
    (*form).cropbox.lly = ref_y + (*cropbox).lly;
    (*form).cropbox.urx = ref_x + (*cropbox).urx;
    (*form).cropbox.ury = ref_y + (*cropbox).ury;
    (*form).contents = pdf_new_stream(STREAM_COMPRESS);
    (*form).resources = pdf_new_dict();
    pdf_ximage_init_form_info(&raw mut info);
    info.matrix.a = 1.0f64;
    info.matrix.b = 0.0f64;
    info.matrix.c = 0.0f64;
    info.matrix.d = 1.0f64;
    info.matrix.e = -ref_x;
    info.matrix.f = -ref_y;
    info.bbox.llx = (*cropbox).llx;
    info.bbox.lly = (*cropbox).lly;
    info.bbox.urx = (*cropbox).urx;
    info.bbox.ury = (*cropbox).ury;
    xobj_id = pdf_ximage_defineresource(
        ident,
        PDF_XOBJECT_TYPE_FORM,
        &raw mut info as *mut ::core::ffi::c_void,
        pdf_link_obj((*form).contents),
    );
    (*p).pending_forms = fnode;
    pdf_dev_reset_fonts(1 as ::core::ffi::c_int);
    pdf_dev_reset_color(1 as ::core::ffi::c_int);
    pdf_dev_reset_xgstate(1 as ::core::ffi::c_int);
    return xobj_id;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_end_grabbing")]
pub unsafe extern "C-unwind" fn pdf_doc_end_grabbing(mut attrib: *mut pdf_obj) {
    let __pdf_state = unsafe { document_state() };
    let mut form: *mut pdf_form = ::core::ptr::null_mut::<pdf_form>();
    let mut procset: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut fnode: *mut form_list_node = ::core::ptr::null_mut::<form_list_node>();
    if (*p).pending_forms.is_null() {
        pdf_warning!(
            b"Tried to close a nonexistent form XOject.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    fnode = (*p).pending_forms;
    form = &raw mut (*fnode).form;
    pdf_dev_grestore_to((*fnode).q_depth);
    procset = pdf_new_array();
    pdf_add_array(
        procset,
        pdf_new_name(b"PDF\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_array(
        procset,
        pdf_new_name(b"Text\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_array(
        procset,
        pdf_new_name(b"ImageC\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_array(
        procset,
        pdf_new_name(b"ImageB\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_array(
        procset,
        pdf_new_name(b"ImageI\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        (*form).resources,
        pdf_new_name(b"ProcSet\0" as *const u8 as *const ::core::ffi::c_char),
        procset,
    );
    pdf_doc_make_xform(
        (*form).contents,
        &raw mut (*form).cropbox,
        &raw mut (*form).matrix,
        pdf_ref_obj((*form).resources),
        attrib,
    );
    pdf_release_obj((*form).resources);
    pdf_release_obj((*form).contents);
    pdf_release_obj(attrib);
    (*p).pending_forms = (*fnode).prev;
    pdf_dev_pop_gstate();
    pdf_dev_reset_fonts(1 as ::core::ffi::c_int);
    pdf_dev_reset_color(0 as ::core::ffi::c_int);
    pdf_dev_reset_xgstate(1 as ::core::ffi::c_int);
    free(fnode as *mut ::core::ffi::c_void);
}

unsafe extern "C-unwind" fn reset_box() {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).breaking_state.rect.lly = ::core::f64::INFINITY;
    (*__pdf_state).breaking_state.rect.llx = (*__pdf_state).breaking_state.rect.lly;
    (*__pdf_state).breaking_state.rect.ury = -::core::f64::INFINITY;
    (*__pdf_state).breaking_state.rect.urx = (*__pdf_state).breaking_state.rect.ury;
    (*__pdf_state).breaking_state.dirty = 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_begin_annot")]
pub unsafe extern "C-unwind" fn pdf_doc_begin_annot(mut dict: *mut pdf_obj) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).breaking_state.annot_dict = dict;
    (*__pdf_state).breaking_state.broken = 0 as ::core::ffi::c_int;
    reset_box();
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_end_annot")]
pub unsafe extern "C-unwind" fn pdf_doc_end_annot() {
    let __pdf_state = unsafe { document_state() };
    pdf_doc_break_annot();
    if !(*__pdf_state).breaking_state.annot_dict.is_null() {
        pdf_release_obj((*__pdf_state).breaking_state.annot_dict);
    }
    (*__pdf_state).breaking_state.annot_dict = ::core::ptr::null_mut::<pdf_obj>();
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_break_annot")]
pub unsafe extern "C-unwind" fn pdf_doc_break_annot() {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_doc = &raw mut (*__pdf_state).pdoc;
    let mut g_x: ::core::ffi::c_double = (*p).options.annot_grow.x;
    let mut g_y: ::core::ffi::c_double = (*p).options.annot_grow.y;
    if (*__pdf_state).breaking_state.dirty != 0 {
        let mut annot_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut annot_copy: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut rect: pdf_rect = pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        };
        annot_dict = (*__pdf_state).breaking_state.annot_dict;
        annot_copy = pdf_new_dict();
        pdf_merge_dict(annot_copy, (*__pdf_state).breaking_state.annot_dict);
        (*__pdf_state).breaking_state.annot_dict = annot_copy;
        rect = (*__pdf_state).breaking_state.rect;
        rect.llx -= g_x;
        rect.lly -= g_y;
        rect.urx += g_x;
        rect.ury += g_y;
        pdf_doc_add_annot(
            pdf_doc_current_page_number() as ::core::ffi::c_uint,
            &raw mut rect,
            annot_dict,
            ((*__pdf_state).breaking_state.broken == 0) as ::core::ffi::c_int,
        );
        pdf_release_obj(annot_dict);
        (*__pdf_state).breaking_state.broken = 1 as ::core::ffi::c_int;
    }
    reset_box();
}
#[unsafe(export_name = "__pdf_impl_pdf_doc_expand_box")]
pub unsafe extern "C-unwind" fn pdf_doc_expand_box(mut rect: *const pdf_rect) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).breaking_state.rect.llx = if (*__pdf_state).breaking_state.rect.llx < (*rect).llx
    {
        (*__pdf_state).breaking_state.rect.llx
    } else {
        (*rect).llx
    };
    (*__pdf_state).breaking_state.rect.lly = if (*__pdf_state).breaking_state.rect.lly < (*rect).lly
    {
        (*__pdf_state).breaking_state.rect.lly
    } else {
        (*rect).lly
    };
    (*__pdf_state).breaking_state.rect.urx = if (*__pdf_state).breaking_state.rect.urx > (*rect).urx
    {
        (*__pdf_state).breaking_state.rect.urx
    } else {
        (*rect).urx
    };
    (*__pdf_state).breaking_state.rect.ury = if (*__pdf_state).breaking_state.rect.ury > (*rect).ury
    {
        (*__pdf_state).breaking_state.rect.ury
    } else {
        (*rect).ury
    };
    (*__pdf_state).breaking_state.dirty = 1 as ::core::ffi::c_int;
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_out {
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_set_bop_content(
    mut content: *const ::core::ffi::c_char,
    mut length: ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_set_bop_content(content, length) })
    {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_set_eop_content(
    mut content: *const ::core::ffi::c_char,
    mut length: ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_set_eop_content(content, length) })
    {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_add_page_resource(
    mut category: *const ::core::ffi::c_char,
    mut resource_name: *const ::core::ffi::c_char,
    mut resource_ref: *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_doc_add_page_resource(category, resource_name, resource_ref)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_get_page_count(
    mut pf: *mut pdf_file,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_get_page_count(pf) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_get_page(
    mut pf: *mut pdf_file,
    mut page_no: ::core::ffi::c_int,
    mut opt_bbox: pdf_page_boundary,
    mut bbox: *mut pdf_rect,
    mut matrix: *mut pdf_tmatrix,
    mut resources_p: *mut *mut pdf_obj,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_doc_get_page(pf, page_no, opt_bbox, bbox, matrix, resources_p)
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_bookmarks_up(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_bookmarks_up() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_bookmarks_down(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_bookmarks_down() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_bookmarks_depth(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_bookmarks_depth() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_bookmarks_add(
    mut dict: *mut pdf_obj,
    mut is_open: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_bookmarks_add(dict, is_open) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_add_names(
    mut category: *const ::core::ffi::c_char,
    mut key: *const ::core::ffi::c_void,
    mut keylen: ::core::ffi::c_int,
    mut value: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_doc_add_names(category, key, keylen, value)
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_add_annot(
    mut page_no: ::core::ffi::c_uint,
    mut rect: *const pdf_rect,
    mut annot_dict: *mut pdf_obj,
    mut new_annot: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_doc_add_annot(page_no, rect, annot_dict, new_annot)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_begin_article(
    mut article_id: *const ::core::ffi::c_char,
    mut article_info: *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_doc_begin_article(article_id, article_info)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_add_bead(
    mut article_id: *const ::core::ffi::c_char,
    mut bead_id: *const ::core::ffi::c_char,
    mut page_no: ::core::ffi::c_int,
    mut rect: *const pdf_rect,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_doc_add_bead(article_id, bead_id, page_no, rect)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_set_mediabox(
    mut page_no: ::core::ffi::c_uint,
    mut mediabox: *const pdf_rect,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_set_mediabox(page_no, mediabox) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_current_page_resources(
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_current_page_resources() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_get_dictionary(
    mut category: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_get_dictionary(category) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_current_page_number(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_current_page_number() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_ref_page(
    mut page_no: ::core::ffi::c_uint,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_ref_page(page_no) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_get_reference(
    mut category: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_get_reference(category) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_set_bgcolor(mut color: *const pdf_color) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_set_bgcolor(color) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_begin_page(
    mut scale: ::core::ffi::c_double,
    mut x_origin: ::core::ffi::c_double,
    mut y_origin: ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_doc_begin_page(scale, x_origin, y_origin)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_end_page() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_end_page() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_add_page_content(
    mut buffer: *const ::core::ffi::c_char,
    mut length: ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_add_page_content(buffer, length) })
    {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_open_document(
    mut filename: *const ::core::ffi::c_char,
    mut creator: *const ::core::ffi::c_char,
    mut id1: *const ::core::ffi::c_uchar,
    mut id2: *const ::core::ffi::c_uchar,
    mut settings: pdf_setting,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_open_document(filename, creator, id1, id2, settings)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_close_document() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_close_document() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_begin_grabbing(
    mut ident: *const ::core::ffi::c_char,
    mut ref_x: ::core::ffi::c_double,
    mut ref_y: ::core::ffi::c_double,
    mut cropbox: *const pdf_rect,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_doc_begin_grabbing(ident, ref_x, ref_y, cropbox)
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
pub unsafe extern "C" fn __pdf_entry_pdf_doc_end_grabbing(mut attrib: *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_end_grabbing(attrib) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_begin_annot(mut dict: *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_begin_annot(dict) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_end_annot() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_end_annot() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_break_annot() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_break_annot() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_doc_expand_box(mut rect: *const pdf_rect) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_doc_expand_box(rect) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}

pub(crate) struct DocumentState {
    pdoc: pdf_doc,
    bgcolor: pdf_color,
    breaking_state: C2RustUnnamed_7,
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        unsafe {
            Self {
                pdoc: pdf_doc {
                    root: C2RustUnnamed_5 {
                        dict: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
                        viewerpref: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
                        pagelabels: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
                        pages: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
                        names: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
                        threads: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
                    },
                    info: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
                    pages: C2RustUnnamed_4 {
                        mediabox: pdf_rect {
                            llx: 0.,
                            lly: 0.,
                            urx: 0.,
                            ury: 0.,
                        },
                        bop: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
                        eop: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
                        num_entries: 0,
                        max_entries: 0,
                        entries: ::core::ptr::null::<pdf_page>() as *mut pdf_page,
                    },
                    outlines: C2RustUnnamed_3 {
                        first: ::core::ptr::null::<pdf_olitem>() as *mut pdf_olitem,
                        current: ::core::ptr::null::<pdf_olitem>() as *mut pdf_olitem,
                        current_depth: 0,
                    },
                    articles: C2RustUnnamed_2 {
                        num_entries: 0,
                        max_entries: 0,
                        entries: ::core::ptr::null::<pdf_article>() as *mut pdf_article,
                    },
                    names: ::core::ptr::null::<name_dict>() as *mut name_dict,
                    check_gotos: 0,
                    gotos: ht_table {
                        count: 0,
                        hval_free_fn: None,
                        table: [::core::ptr::null::<ht_entry>() as *mut ht_entry; 503],
                    },
                    options: C2RustUnnamed_0 {
                        outline_open_depth: 0,
                        annot_grow: C2RustUnnamed_1 { x: 0., y: 0. },
                        enable_manual_thumb: 0,
                    },
                    pending_forms: ::core::ptr::null::<form_list_node>() as *mut form_list_node,
                    thumb_basename: ::core::ptr::null::<::core::ffi::c_char>()
                        as *mut ::core::ffi::c_char,
                },
                bgcolor: pdf_color {
                    res_id: -(1 as ::core::ffi::c_int),
                    type_0: PDF_COLORSPACE_TYPE_GRAY,
                    num_components: 1 as ::core::ffi::c_int,
                    spot_color_name: ::core::ptr::null::<::core::ffi::c_char>()
                        as *mut ::core::ffi::c_char,
                    values: [
                        1.0f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                    ],
                    pattern_id: -(1 as ::core::ffi::c_int),
                },
                breaking_state: C2RustUnnamed_7 {
                    dirty: 0 as ::core::ffi::c_int,
                    broken: 0 as ::core::ffi::c_int,
                    annot_dict: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
                    rect: pdf_rect {
                        llx: 0.0f64,
                        lly: 0.0f64,
                        urx: 0.0f64,
                        ury: 0.0f64,
                    },
                },
            }
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe {
        (*crate::compat::document::current())
            .backend
            .dpx_pdfdoc
            .pointer
    }
}
