/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2020 by Jin-Hwan Cho and Shunsaku Hirata,
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
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __big: *const ::core::ffi::c_char,
        __little: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn ttstub_input_open(
        path: *const ::core::ffi::c_char,
        format: ttbc_file_format,
        is_gz: ::core::ffi::c_int,
    ) -> rust_input_handle_t;
    fn ttstub_input_read_partial(
        handle: rust_input_handle_t,
        data: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> ssize_t;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_link_obj"]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_number_value"]
    fn pdf_number_value(number: *mut pdf_obj) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pdf_set_string"]
    fn pdf_set_string(object: *mut pdf_obj, str: *mut ::core::ffi::c_uchar, length: size_t);
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
    #[link_name = "__pdf_impl_pdf_foreach_dict"]
    fn pdf_foreach_dict(
        dict: *mut pdf_obj,
        proc_0: Option<
            unsafe extern "C-unwind" fn(
                *mut pdf_obj,
                *mut pdf_obj,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        pdata: *mut ::core::ffi::c_void,
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
    #[link_name = "__pdf_impl_pdf_compare_reference"]
    fn pdf_compare_reference(ref1: *mut pdf_obj, ref2: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_deref_obj"]
    fn pdf_deref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_color_copycolor"]
    fn pdf_color_copycolor(color1: *mut pdf_color, color2: *const pdf_color);
    #[link_name = "__pdf_impl_pdf_color_set"]
    fn pdf_color_set(sc: *mut pdf_color, fc: *mut pdf_color);
    #[link_name = "__pdf_impl_pdf_color_push"]
    fn pdf_color_push(sc: *mut pdf_color, fc: *mut pdf_color);
    #[link_name = "__pdf_impl_pdf_color_pop"]
    fn pdf_color_pop();
    #[link_name = "__pdf_impl_pdf_color_get_current"]
    fn pdf_color_get_current(sc: *mut *mut pdf_color, fc: *mut *mut pdf_color);
    #[link_name = "__pdf_impl_transform_info_clear"]
    fn transform_info_clear(info: *mut transform_info);
    #[link_name = "__pdf_impl_pdf_sprint_matrix"]
    fn pdf_sprint_matrix(
        buf: *mut ::core::ffi::c_char,
        p: *const pdf_tmatrix,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_reset_color"]
    fn pdf_dev_reset_color(force: ::core::ffi::c_int);
    fn spc_warn(spe: *mut spc_env, fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_spc_lookup_reference"]
    fn spc_lookup_reference(ident: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_spc_lookup_object"]
    fn spc_lookup_object(ident: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_spc_begin_annot"]
    fn spc_begin_annot(spe: *mut spc_env, annot_dict: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_end_annot"]
    fn spc_end_annot(spe: *mut spc_env) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_resume_annot"]
    fn spc_resume_annot(spe: *mut spc_env) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_suspend_annot"]
    fn spc_suspend_annot(spe: *mut spc_env) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_begin_form"]
    fn spc_begin_form(
        spe: *mut spc_env,
        ident: *const ::core::ffi::c_char,
        cp: pdf_coord,
        cropbox: *mut pdf_rect,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_end_form"]
    fn spc_end_form(spe: *mut spc_env, attr: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_is_tracking_boxes"]
    fn spc_is_tracking_boxes(spe: *mut spc_env) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_push_object"]
    fn spc_push_object(spe: *mut spc_env, key: *const ::core::ffi::c_char, value: *mut pdf_obj);
    #[link_name = "__pdf_impl_spc_flush_object"]
    fn spc_flush_object(spe: *mut spc_env, key: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_spc_clear_objects"]
    fn spc_clear_objects(spe: *mut spc_env);
    #[link_name = "__pdf_impl_spc_put_image"]
    fn spc_put_image(
        spe: *mut spc_env,
        res_id: ::core::ffi::c_int,
        ti: *mut transform_info,
        xpos: ::core::ffi::c_double,
        ypos: ::core::ffi::c_double,
    );
    #[link_name = "__pdf_impl_spc_get_current_point"]
    fn spc_get_current_point(spe: *mut spc_env, cp: *mut pdf_coord);
    #[link_name = "__pdf_impl_spc_get_coord"]
    fn spc_get_coord(
        spe: *mut spc_env,
        x: *mut ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
    );
    #[link_name = "__pdf_impl_spc_push_coord"]
    fn spc_push_coord(spe: *mut spc_env, x: ::core::ffi::c_double, y: ::core::ffi::c_double);
    #[link_name = "__pdf_impl_spc_pop_coord"]
    fn spc_pop_coord(spe: *mut spc_env);
    static mut dpx_conf: _dpx_conf;
    static mut work_buffer: [::core::ffi::c_char; 0];
    #[link_name = "__pdf_impl_min4"]
    fn min4(
        v1: ::core::ffi::c_double,
        v2: ::core::ffi::c_double,
        v3: ::core::ffi::c_double,
        v4: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_max4"]
    fn max4(
        v1: ::core::ffi::c_double,
        v2: ::core::ffi::c_double,
        v3: ::core::ffi::c_double,
        v4: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_parse_c_ident"]
    fn parse_c_ident(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_pdf_init_fontmap_record"]
    fn pdf_init_fontmap_record(mrec: *mut fontmap_rec);
    #[link_name = "__pdf_impl_pdf_clear_fontmap_record"]
    fn pdf_clear_fontmap_record(mrec: *mut fontmap_rec);
    #[link_name = "__pdf_impl_pdf_load_fontmap_file"]
    fn pdf_load_fontmap_file(
        filename: *const ::core::ffi::c_char,
        mode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_read_fontmap_line"]
    fn pdf_read_fontmap_line(
        mrec: *mut fontmap_rec,
        mline: *const ::core::ffi::c_char,
        mline_strlen: ::core::ffi::c_int,
        format: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_append_fontmap_record"]
    fn pdf_append_fontmap_record(
        kp: *const ::core::ffi::c_char,
        mrec: *const fontmap_rec,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_remove_fontmap_record"]
    fn pdf_remove_fontmap_record(kp: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_insert_fontmap_record"]
    fn pdf_insert_fontmap_record(
        kp: *const ::core::ffi::c_char,
        mrec: *const fontmap_rec,
    ) -> *mut fontmap_rec;
    #[link_name = "__pdf_impl_is_pdfm_mapline"]
    fn is_pdfm_mapline(mline: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_doc_get_dictionary"]
    fn pdf_doc_get_dictionary(category: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_doc_current_page_number"]
    fn pdf_doc_current_page_number() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_doc_current_page_resources"]
    fn pdf_doc_current_page_resources() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_doc_add_names"]
    fn pdf_doc_add_names(
        category: *const ::core::ffi::c_char,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
        value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_doc_set_bop_content"]
    fn pdf_doc_set_bop_content(str: *const ::core::ffi::c_char, length: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_doc_set_eop_content"]
    fn pdf_doc_set_eop_content(str: *const ::core::ffi::c_char, length: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_doc_add_page_content"]
    fn pdf_doc_add_page_content(buffer: *const ::core::ffi::c_char, length: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_doc_add_page_resource"]
    fn pdf_doc_add_page_resource(
        category: *const ::core::ffi::c_char,
        resource_name: *const ::core::ffi::c_char,
        resources: *mut pdf_obj,
    );
    #[link_name = "__pdf_impl_pdf_doc_begin_article"]
    fn pdf_doc_begin_article(article_id: *const ::core::ffi::c_char, info: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_doc_add_bead"]
    fn pdf_doc_add_bead(
        article_id: *const ::core::ffi::c_char,
        bead_id: *const ::core::ffi::c_char,
        page_no: ::core::ffi::c_int,
        rect: *const pdf_rect,
    );
    #[link_name = "__pdf_impl_pdf_doc_bookmarks_up"]
    fn pdf_doc_bookmarks_up() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_doc_bookmarks_down"]
    fn pdf_doc_bookmarks_down() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_doc_bookmarks_add"]
    fn pdf_doc_bookmarks_add(dict: *mut pdf_obj, is_open: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_doc_bookmarks_depth"]
    fn pdf_doc_bookmarks_depth() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_doc_add_annot"]
    fn pdf_doc_add_annot(
        page_no: ::core::ffi::c_uint,
        rect: *const pdf_rect,
        annot_dict: *mut pdf_obj,
        new_annot: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_doc_expand_box"]
    fn pdf_doc_expand_box(rect: *const pdf_rect);
    #[link_name = "__pdf_impl_pdf_doc_set_bgcolor"]
    fn pdf_doc_set_bgcolor(color: *const pdf_color);
    #[link_name = "__pdf_impl_pdf_dev_concat"]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_transform"]
    fn pdf_dev_transform(p: *mut pdf_coord, M: *const pdf_tmatrix);
    #[link_name = "__pdf_impl_pdf_dev_gsave"]
    fn pdf_dev_gsave() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_grestore"]
    fn pdf_dev_grestore() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_xgstate_push"]
    fn pdf_dev_xgstate_push(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_dev_xgstate_pop"]
    fn pdf_dev_xgstate_pop();
    #[link_name = "__pdf_impl_pdf_dev_reset_xgstate"]
    fn pdf_dev_reset_xgstate(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_parse_ident"]
    fn parse_ident(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_val_ident"]
    fn parse_val_ident(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_opt_ident"]
    fn parse_opt_ident(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_pdf_object"]
    fn parse_pdf_object(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_object_extended"]
    fn parse_pdf_object_extended(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        pf: *mut pdf_file,
        unknown_handler: Option<
            unsafe extern "C-unwind" fn(
                *mut *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut ::core::ffi::c_void,
            ) -> *mut pdf_obj,
        >,
        user_data: *mut ::core::ffi::c_void,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_tainted_dict"]
    fn parse_pdf_tainted_dict(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        unknown_handler: Option<
            unsafe extern "C-unwind" fn(
                *mut *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut ::core::ffi::c_void,
            ) -> *mut pdf_obj,
        >,
        user_data: *mut ::core::ffi::c_void,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_ximage_findresource"]
    fn pdf_ximage_findresource(ident: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ximage_load_image"]
    fn pdf_ximage_load_image(
        ident: *const ::core::ffi::c_char,
        filename: *const ::core::ffi::c_char,
        options: load_options,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ximage_reserve"]
    fn pdf_ximage_reserve(ident: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ximage_get_subtype"]
    fn pdf_ximage_get_subtype(xobj_id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ximage_set_attr"]
    fn pdf_ximage_set_attr(
        xobj_id: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        xdensity: ::core::ffi::c_double,
        ydensity: ::core::ffi::c_double,
        llx: ::core::ffi::c_double,
        lly: ::core::ffi::c_double,
        urx: ::core::ffi::c_double,
        ury: ::core::ffi::c_double,
    );
    #[link_name = "__pdf_impl_spc_util_read_dimtrns"]
    fn spc_util_read_dimtrns(
        spe: *mut spc_env,
        dimtrns: *mut transform_info,
        args: *mut spc_arg,
        syntax: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_util_read_blahblah"]
    fn spc_util_read_blahblah(
        spe: *mut spc_env,
        dimtrns: *mut transform_info,
        page_no: *mut ::core::ffi::c_int,
        bbox_type: *mut pdf_page_boundary,
        args: *mut spc_arg,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_util_read_pdfcolor"]
    fn spc_util_read_pdfcolor(
        spe: *mut spc_env,
        colorspec: *mut pdf_color,
        args: *mut spc_arg,
        defaultcolor: *mut pdf_color,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_UC_is_valid"]
    fn UC_is_valid(ucv: int32_t) -> bool;
    #[link_name = "__pdf_impl_UC_UTF8_is_valid_string"]
    fn UC_UTF8_is_valid_string(
        p: *const ::core::ffi::c_uchar,
        endptr: *const ::core::ffi::c_uchar,
    ) -> bool;
    #[link_name = "__pdf_impl_UC_UTF16BE_encode_char"]
    fn UC_UTF16BE_encode_char(
        ucv: int32_t,
        dstpp: *mut *mut ::core::ffi::c_uchar,
        endptr: *mut ::core::ffi::c_uchar,
    ) -> size_t;
    #[link_name = "__pdf_impl_UC_UTF8_decode_char"]
    fn UC_UTF8_decode_char(
        pp: *mut *const ::core::ffi::c_uchar,
        endptr: *const ::core::ffi::c_uchar,
    ) -> int32_t;
    #[link_name = "__pdf_impl_CMap_decode"]
    fn CMap_decode(
        cmap: *mut CMap,
        inbuf: *mut *const ::core::ffi::c_uchar,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut ::core::ffi::c_uchar,
        outbytesleft: *mut size_t,
    ) -> size_t;
    #[link_name = "__pdf_impl_CMap_cache_get"]
    fn CMap_cache_get(id: ::core::ffi::c_int) -> *mut CMap;
    #[link_name = "__pdf_impl_CMap_cache_find"]
    fn CMap_cache_find(cmap_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_tmatrix {
    pub a: ::core::ffi::c_double,
    pub b: ::core::ffi::c_double,
    pub c: ::core::ffi::c_double,
    pub d: ::core::ffi::c_double,
    pub e: ::core::ffi::c_double,
    pub f: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: ::core::ffi::c_double,
    pub lly: ::core::ffi::c_double,
    pub urx: ::core::ffi::c_double,
    pub ury: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_coord {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_info {
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
    pub depth: ::core::ffi::c_double,
    pub matrix: pdf_tmatrix,
    pub bbox: pdf_rect,
    pub flags: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_env {
    pub x_user: ::core::ffi::c_double,
    pub y_user: ::core::ffi::c_double,
    pub mag: ::core::ffi::c_double,
    pub pg: ::core::ffi::c_int,
    pub info: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub is_drawable: ::core::ffi::c_int,
    pub rect: pdf_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_arg {
    pub curptr: *const ::core::ffi::c_char,
    pub endptr: *const ::core::ffi::c_char,
    pub base: *const ::core::ffi::c_char,
    pub command: *const ::core::ffi::c_char,
}
pub type spc_handler_fn_ptr =
    Option<unsafe extern "C-unwind" fn(*mut spc_env, *mut spc_arg) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const ::core::ffi::c_char,
    pub exec: spc_handler_fn_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_pdf_ {
    pub annot_dict: *mut pdf_obj,
    pub lowest_level: ::core::ffi::c_int,
    pub cd: tounicode,
    pub pageresources: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tounicode {
    pub cmap_id: ::core::ffi::c_int,
    pub unescape_backslash: ::core::ffi::c_int,
    pub taintkeys: *mut pdf_obj,
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
pub struct C2RustUnnamed_0 {
    pub sfd_name: *mut ::core::ffi::c_char,
    pub subfont_id: *mut ::core::ffi::c_char,
}
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_3,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: ::core::ffi::c_int,
    pub profile: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
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
pub struct load_options {
    pub page_no: ::core::ffi::c_int,
    pub bbox_type: pdf_page_boundary,
    pub dict: *mut pdf_obj,
}
pub type pdf_page_boundary = ::core::ffi::c_uint;
pub const pdf_page_boundary_bleedbox: pdf_page_boundary = 5;
pub const pdf_page_boundary_trimbox: pdf_page_boundary = 4;
pub const pdf_page_boundary_artbox: pdf_page_boundary = 3;
pub const pdf_page_boundary_cropbox: pdf_page_boundary = 1;
pub const pdf_page_boundary_mediabox: pdf_page_boundary = 2;
pub const pdf_page_boundary__auto: pdf_page_boundary = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
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
#[inline]
unsafe extern "C-unwind" fn strstartswith(
    mut s: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0 as ::core::ffi::c_int {
        return s.offset(length as isize);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
pub const PDF_NUMBER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_STRING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PDF_NAME: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PDF_ARRAY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PDF_DICT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PDF_STREAM: ::core::ffi::c_int = 7;
pub const PDF_INDIRECT: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const INFO_HAS_USER_BBOX: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const INFO_HAS_WIDTH: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const INFO_HAS_HEIGHT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const INFO_DO_HIDE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const WORK_BUFFER_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const FONTMAP_RMODE_REPLACE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FONTMAP_RMODE_APPEND: ::core::ffi::c_int = '+' as i32;
pub const FONTMAP_RMODE_REMOVE: ::core::ffi::c_int = '-' as i32;
pub const PDF_XOBJECT_TYPE_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut _pdf_stat: spc_pdf_ = spc_pdf_ {
    annot_dict: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
    lowest_level: 255 as ::core::ffi::c_int,
    cd: tounicode {
        cmap_id: -(1 as ::core::ffi::c_int),
        unescape_backslash: 0 as ::core::ffi::c_int,
        taintkeys: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
    },
    pageresources: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
};
unsafe extern "C-unwind" fn parse_pdf_reference(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut user_data: *mut ::core::ffi::c_void,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    skip_white(start, end);
    name = parse_opt_ident(start, end);
    if !name.is_null() {
        result = spc_lookup_reference(name);
        if result.is_null() {
            pdf_warning!(
                b"Could not find the named reference (@%s).\0" as *const u8
                    as *const ::core::ffi::c_char,
                name,
            );
        }
        free(name as *mut ::core::ffi::c_void);
    } else {
        pdf_warning!(
            b"Could not find a reference name.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        result = ::core::ptr::null_mut::<pdf_obj>();
    }
    return result;
}
unsafe extern "C-unwind" fn spc_handler_pdfm__init(
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = dp as *mut spc_pdf_;
    static mut default_taintkeys: [*const ::core::ffi::c_char; 12] = [
        b"Title\0" as *const u8 as *const ::core::ffi::c_char,
        b"Author\0" as *const u8 as *const ::core::ffi::c_char,
        b"Subject\0" as *const u8 as *const ::core::ffi::c_char,
        b"Keywords\0" as *const u8 as *const ::core::ffi::c_char,
        b"Creator\0" as *const u8 as *const ::core::ffi::c_char,
        b"Producer\0" as *const u8 as *const ::core::ffi::c_char,
        b"Contents\0" as *const u8 as *const ::core::ffi::c_char,
        b"Subj\0" as *const u8 as *const ::core::ffi::c_char,
        b"TU\0" as *const u8 as *const ::core::ffi::c_char,
        b"T\0" as *const u8 as *const ::core::ffi::c_char,
        b"TM\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut i: ::core::ffi::c_int = 0;
    (*sd).annot_dict = ::core::ptr::null_mut::<pdf_obj>();
    (*sd).lowest_level = 255 as ::core::ffi::c_int;
    (*sd).cd.taintkeys = pdf_new_array();
    i = 0 as ::core::ffi::c_int;
    while !default_taintkeys[i as usize].is_null() {
        pdf_add_array(
            (*sd).cd.taintkeys,
            pdf_new_name(default_taintkeys[i as usize]),
        );
        i += 1;
    }
    (*sd).pageresources = ::core::ptr::null_mut::<pdf_obj>();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm__clean(
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = dp as *mut spc_pdf_;
    if !(*sd).annot_dict.is_null() {
        pdf_warning!(
            b"Unbalanced bann and eann found.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        pdf_release_obj((*sd).annot_dict);
    }
    (*sd).lowest_level = 255 as ::core::ffi::c_int;
    (*sd).annot_dict = ::core::ptr::null_mut::<pdf_obj>();
    pdf_release_obj((*sd).cd.taintkeys);
    (*sd).cd.taintkeys = ::core::ptr::null_mut::<pdf_obj>();
    if !(*sd).pageresources.is_null() {
        pdf_release_obj((*sd).pageresources);
    }
    (*sd).pageresources = ::core::ptr::null_mut::<pdf_obj>();
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_pdfm_at_begin_document")]
pub unsafe extern "C-unwind" fn spc_pdfm_at_begin_document() -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    return spc_handler_pdfm__init(sd as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_spc_pdfm_at_end_document")]
pub unsafe extern "C-unwind" fn spc_pdfm_at_end_document() -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    return spc_handler_pdfm__clean(sd as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn spc_handler_pdfm_bop(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    if (*args).curptr < (*args).endptr {
        pdf_doc_set_bop_content(
            (*args).curptr,
            (*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
    }
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_eop(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    if (*args).curptr < (*args).endptr {
        pdf_doc_set_eop_content(
            (*args).curptr,
            (*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
    }
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn safeputresdent(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(!kp.is_null() && !vp.is_null() && !dp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"kp && vp && dp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    key = pdf_name_value(kp);
    if !pdf_lookup_dict(dp as *mut pdf_obj, key).is_null() {
        pdf_warning!(
            b"Object \"%s\" already defined in dict! (ignored)\0" as *const u8
                as *const ::core::ffi::c_char,
            key,
        );
    } else {
        pdf_add_dict(dp as *mut pdf_obj, pdf_link_obj(kp), pdf_link_obj(vp));
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn safeputresdict(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if !(!kp.is_null() && !vp.is_null() && !dp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"kp && vp && dp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    key = pdf_name_value(kp);
    dict = pdf_lookup_dict(dp as *mut pdf_obj, key);
    if !dict.is_null() && pdf_obj_typeof(dict) == PDF_INDIRECT {
        if !vp.is_null() && pdf_obj_typeof(vp) == PDF_INDIRECT {
            if pdf_compare_reference(dict, vp) == 0 {
                return 0 as ::core::ffi::c_int;
            } else {
                dict = pdf_deref_obj(dict);
                pdf_release_obj(dict);
            }
        }
    }
    if pdf_obj_typeof(vp) == PDF_INDIRECT {
        if !dict.is_null() {
            let mut dst: *mut pdf_obj = pdf_deref_obj(vp);
            if !dst.is_null() {
                if pdf_obj_typeof(dst) == PDF_DICT {
                    pdf_foreach_dict(
                        dict,
                        Some(
                            safeputresdent
                                as unsafe extern "C-unwind" fn(
                                    *mut pdf_obj,
                                    *mut pdf_obj,
                                    *mut ::core::ffi::c_void,
                                )
                                    -> ::core::ffi::c_int,
                        ),
                        dst as *mut ::core::ffi::c_void,
                    );
                    pdf_release_obj(dst);
                } else {
                    pdf_warning!(
                        b"Invalid type (not DICT) for page/form resource dict entry: key=\"%s\"\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        key,
                    );
                    pdf_release_obj(dst);
                    return -(1 as ::core::ffi::c_int);
                }
            }
        }
        pdf_add_dict(dp as *mut pdf_obj, pdf_new_name(key), pdf_link_obj(vp));
    } else if pdf_obj_typeof(vp) == PDF_DICT {
        if !dict.is_null() {
            pdf_foreach_dict(
                vp,
                Some(
                    safeputresdent
                        as unsafe extern "C-unwind" fn(
                            *mut pdf_obj,
                            *mut pdf_obj,
                            *mut ::core::ffi::c_void,
                        )
                            -> ::core::ffi::c_int,
                ),
                dict as *mut ::core::ffi::c_void,
            );
        } else {
            pdf_add_dict(dp as *mut pdf_obj, pdf_new_name(key), pdf_link_obj(vp));
        }
    } else {
        pdf_warning!(
            b"Invalid type (not DICT) for page/form resource dict entry: key=\"%s\"\0" as *const u8
                as *const ::core::ffi::c_char,
            key,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn putpageresources(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut resource_name: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(!kp.is_null() && !vp.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"kp && vp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    resource_name = pdf_name_value(kp);
    pdf_doc_add_page_resource(
        dp as *const ::core::ffi::c_char,
        resource_name,
        pdf_link_obj(vp),
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn forallresourcecategory(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut category: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(!kp.is_null() && !vp.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"kp && vp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    category = pdf_name_value(kp);
    match pdf_obj_typeof(vp) {
        PDF_DICT => {
            r = pdf_foreach_dict(
                vp,
                Some(
                    putpageresources
                        as unsafe extern "C-unwind" fn(
                            *mut pdf_obj,
                            *mut pdf_obj,
                            *mut ::core::ffi::c_void,
                        )
                            -> ::core::ffi::c_int,
                ),
                category as *mut ::core::ffi::c_void,
            );
        }
        PDF_INDIRECT => {
            let mut obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            obj = pdf_deref_obj(vp);
            if obj.is_null() {
                pdf_warning!(
                    b"Can't deref object for page resource: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    category,
                );
                r = -(1 as ::core::ffi::c_int);
            } else if pdf_obj_typeof(obj) != PDF_DICT {
                pdf_warning!(
                    b"Invalid object type for page resource: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    category,
                );
                r = -(1 as ::core::ffi::c_int);
            } else {
                let mut res_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                res_dict = pdf_doc_current_page_resources();
                dict = pdf_lookup_dict(res_dict, category);
                if dict.is_null() {
                    pdf_add_dict(res_dict, pdf_new_name(category), pdf_link_obj(vp));
                } else {
                    if pdf_obj_typeof(dict) == PDF_INDIRECT {
                        dict = pdf_deref_obj(dict);
                        pdf_release_obj(dict);
                    }
                    pdf_foreach_dict(
                        dict,
                        Some(
                            safeputresdent
                                as unsafe extern "C-unwind" fn(
                                    *mut pdf_obj,
                                    *mut pdf_obj,
                                    *mut ::core::ffi::c_void,
                                )
                                    -> ::core::ffi::c_int,
                        ),
                        obj as *mut ::core::ffi::c_void,
                    );
                    pdf_add_dict(res_dict, pdf_new_name(category), pdf_link_obj(vp));
                }
                pdf_release_obj(obj);
            }
        }
        _ => {
            pdf_warning!(
                b"Invalid object type for page resource specified for \"%s\"\0" as *const u8
                    as *const ::core::ffi::c_char,
                category,
            );
        }
    }
    return r;
}
#[unsafe(export_name = "__pdf_impl_spc_pdfm_at_end_page")]
pub unsafe extern "C-unwind" fn spc_pdfm_at_end_page() -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    if !(*sd).pageresources.is_null() {
        pdf_foreach_dict(
            (*sd).pageresources,
            Some(
                forallresourcecategory
                    as unsafe extern "C-unwind" fn(
                        *mut pdf_obj,
                        *mut pdf_obj,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_put(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut obj1: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut obj2: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    ident = parse_opt_ident(&raw mut (*ap).curptr, (*ap).endptr);
    if ident.is_null() {
        pdf_warning!(b"Missing object identifier.\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    obj1 = spc_lookup_object(ident);
    if obj1.is_null() {
        pdf_warning!(
            b"Specified object not exist: %s\0" as *const u8 as *const ::core::ffi::c_char,
            ident
        );
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    obj2 = parse_pdf_object_extended(
        &raw mut (*ap).curptr,
        (*ap).endptr,
        ::core::ptr::null_mut::<pdf_file>(),
        Some(
            parse_pdf_reference
                as unsafe extern "C-unwind" fn(
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> *mut pdf_obj,
        ),
        spe as *mut ::core::ffi::c_void,
    );
    if obj2.is_null() {
        pdf_warning!(
            b"Missing (an) object(s) to put into \"%s\"!\0" as *const u8
                as *const ::core::ffi::c_char,
            ident
        );
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    match pdf_obj_typeof(obj1) {
        PDF_DICT => {
            if pdf_obj_typeof(obj2) != PDF_DICT {
                pdf_warning!(
                    b"Inconsistent object type for \"put\" (expecting DICT): %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    ident
                );
                error = -(1 as ::core::ffi::c_int);
            } else if streq_ptr(
                ident,
                b"resources\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                error = pdf_foreach_dict(
                    obj2,
                    Some(
                        safeputresdict
                            as unsafe extern "C-unwind" fn(
                                *mut pdf_obj,
                                *mut pdf_obj,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    obj1 as *mut ::core::ffi::c_void,
                );
            } else {
                pdf_merge_dict(obj1, obj2);
            }
        }
        PDF_STREAM => {
            if pdf_obj_typeof(obj2) == PDF_DICT {
                pdf_merge_dict(pdf_stream_dict(obj1), obj2);
            } else if pdf_obj_typeof(obj2) == PDF_STREAM {
                pdf_warning!(
                    b"\"put\" operation not supported for STREAM <- STREAM: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    ident
                );
                error = -(1 as ::core::ffi::c_int);
            } else {
                pdf_warning!(
                    b"Invalid type: expecting a DICT or STREAM: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    ident
                );
                error = -(1 as ::core::ffi::c_int);
            }
        }
        PDF_ARRAY => {
            pdf_add_array(obj1, pdf_link_obj(obj2));
            while (*ap).curptr < (*ap).endptr {
                let mut obj3: *mut pdf_obj = parse_pdf_object_extended(
                    &raw mut (*ap).curptr,
                    (*ap).endptr,
                    ::core::ptr::null_mut::<pdf_file>(),
                    Some(
                        parse_pdf_reference
                            as unsafe extern "C-unwind" fn(
                                *mut *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                                *mut ::core::ffi::c_void,
                            )
                                -> *mut pdf_obj,
                    ),
                    spe as *mut ::core::ffi::c_void,
                );
                if obj3.is_null() {
                    break;
                }
                pdf_add_array(obj1, obj3);
                skip_white(&raw mut (*ap).curptr, (*ap).endptr);
            }
        }
        _ => {
            pdf_warning!(
                b"Can't \"put\" object into non-DICT/STREAM/ARRAY type object: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                ident
            );
            error = -(1 as ::core::ffi::c_int);
        }
    }
    pdf_release_obj(obj2);
    free(ident as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C-unwind" fn calculate_size_utf16(
    mut p: *const ::core::ffi::c_uchar,
    mut endptr: *const ::core::ffi::c_uchar,
) -> size_t {
    let mut len: size_t = 0 as size_t;
    while p < endptr {
        let mut c: ::core::ffi::c_uchar = *p;
        if (c as ::core::ffi::c_int) < 0x80 as ::core::ffi::c_int {
            len = len.wrapping_add(2 as size_t);
            p = p.offset(1 as ::core::ffi::c_int as isize);
        } else if (c as ::core::ffi::c_int) < 0xe0 as ::core::ffi::c_int {
            len = len.wrapping_add(2 as size_t);
            p = p.offset(2 as ::core::ffi::c_int as isize);
        } else if (c as ::core::ffi::c_int) < 0xf0 as ::core::ffi::c_int {
            len = len.wrapping_add(2 as size_t);
            p = p.offset(3 as ::core::ffi::c_int as isize);
        } else if (c as ::core::ffi::c_int) < 0xf8 as ::core::ffi::c_int {
            len = len.wrapping_add(4 as size_t);
            p = p.offset(4 as ::core::ffi::c_int as isize);
        } else if (c as ::core::ffi::c_int) < 0xfc as ::core::ffi::c_int {
            len = len.wrapping_add(4 as size_t);
            p = p.offset(5 as ::core::ffi::c_int as isize);
        } else if (c as ::core::ffi::c_int) < 0xfe as ::core::ffi::c_int {
            len = len.wrapping_add(4 as size_t);
            p = p.offset(6 as ::core::ffi::c_int as isize);
        }
    }
    return len;
}
unsafe extern "C-unwind" fn reencode_string_from_utf8_to_utf16be(
    mut instring: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut strptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut length: size_t = 0;
    let mut non_ascii: ::core::ffi::c_int = 0;
    let mut p: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut endptr: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    if instring.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"instring\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(!instring.is_null() && pdf_obj_typeof(instring) == 3 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"PDF_OBJ_STRINGTYPE(instring)\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    strptr = pdf_string_value(instring) as *mut ::core::ffi::c_uchar;
    length = pdf_string_length(instring) as size_t;
    p = strptr;
    endptr = strptr.offset(length as isize);
    non_ascii = 0 as ::core::ffi::c_int;
    while p < endptr {
        if *p as ::core::ffi::c_int > 127 as ::core::ffi::c_int {
            non_ascii += 1;
        }
        p = p.offset(1);
    }
    if non_ascii == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if !UC_UTF8_is_valid_string(strptr, endptr) {
        error = -(1 as ::core::ffi::c_int);
    } else {
        let mut q: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut buf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut limptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut len: size_t = 0;
        p = strptr;
        len = calculate_size_utf16(p, endptr).wrapping_add(2 as size_t);
        buf = new((len as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        q = buf;
        limptr = buf.offset(len as isize);
        *q.offset(0 as ::core::ffi::c_int as isize) = 0xfe as ::core::ffi::c_uchar;
        *q.offset(1 as ::core::ffi::c_int as isize) = 0xff as ::core::ffi::c_uchar;
        q = q.offset(2 as ::core::ffi::c_int as isize);
        while p < endptr && q < limptr && error == 0 {
            let mut ucv: int32_t = 0;
            let mut count: size_t = 0;
            ucv = UC_UTF8_decode_char(&raw mut p, endptr);
            if !UC_is_valid(ucv) {
                error = -(1 as ::core::ffi::c_int);
            } else {
                count = UC_UTF16BE_encode_char(ucv, &raw mut q, limptr);
                if count == 0 as size_t {
                    error = -(1 as ::core::ffi::c_int);
                }
            }
        }
        if error == 0 {
            pdf_set_string(
                instring,
                buf,
                q.offset_from(buf) as ::core::ffi::c_long as size_t,
            );
        }
        free(buf as *mut ::core::ffi::c_void);
    }
    return error;
}
unsafe extern "C-unwind" fn reencode_string(
    mut cmap: *mut CMap,
    mut instring: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if instring.is_null() || !(!instring.is_null() && pdf_obj_typeof(instring) == PDF_STRING) {
        return -(1 as ::core::ffi::c_int);
    }
    if !cmap.is_null() {
        let mut obuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut obufcur: *mut ::core::ffi::c_uchar =
            ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut inbufcur: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
        let mut inbufleft: size_t = 0;
        let mut obufleft: size_t = 0;
        let mut obufsize: size_t = 0;
        inbufleft = pdf_string_length(instring) as size_t;
        inbufcur = pdf_string_value(instring) as *const ::core::ffi::c_uchar;
        obufsize = inbufleft
            .wrapping_mul(4 as size_t)
            .wrapping_add(2 as size_t);
        obuf = new((obufsize as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        *obuf.offset(0 as ::core::ffi::c_int as isize) = 0xfe as ::core::ffi::c_uchar;
        *obuf.offset(1 as ::core::ffi::c_int as isize) = 0xff as ::core::ffi::c_uchar;
        obufcur = obuf.offset(2 as ::core::ffi::c_int as isize);
        obufleft = obufsize.wrapping_sub(2 as size_t);
        CMap_decode(
            cmap,
            &raw mut inbufcur,
            &raw mut inbufleft,
            &raw mut obufcur,
            &raw mut obufleft,
        );
        if inbufleft > 0 as size_t {
            error = -(1 as ::core::ffi::c_int);
        }
        if error == 0 {
            pdf_set_string(instring, obuf, obufsize.wrapping_sub(obufleft));
        }
        free(obuf as *mut ::core::ffi::c_void);
    }
    return error;
}
unsafe extern "C-unwind" fn need_reencode(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut cd: *mut tounicode,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_uint = 0;
    let mut tk: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if !(!cd.is_null() && !(*cd).taintkeys.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cd && cd->taintkeys\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(pdf_obj_typeof(kp) == 4 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"pdf_obj_typeof(kp) == PDF_NAME\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(pdf_obj_typeof(vp) == 3 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"pdf_obj_typeof(vp) == PDF_STRING\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    i = 0 as ::core::ffi::c_uint;
    while i < pdf_array_length((*cd).taintkeys) {
        tk = pdf_get_array((*cd).taintkeys, i as ::core::ffi::c_int);
        if !(!tk.is_null() && pdf_obj_typeof(tk) == 4 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            pdf_abort!(
                c"assertion failed: %s".as_ptr(),
                b"tk && pdf_obj_typeof(tk) == PDF_NAME\0" as *const u8
                    as *const ::core::ffi::c_char
            );
        } else {
        };
        if streq_ptr(pdf_name_value(kp), pdf_name_value(tk)) {
            r = 1 as ::core::ffi::c_int;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if r != 0 {
        if pdf_string_length(vp) >= 2 as ::core::ffi::c_uint
            && memcmp(
                pdf_string_value(vp),
                b"\xFE\xFF\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                2 as size_t,
            ) == 0
        {
            r = 0 as ::core::ffi::c_int;
        }
    }
    return r;
}
unsafe extern "C-unwind" fn modify_strings(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cd: *mut tounicode = dp as *mut tounicode;
    if !(pdf_obj_typeof(kp) == 4 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"pdf_obj_typeof(kp) == PDF_NAME\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match pdf_obj_typeof(vp) {
        PDF_STRING => {
            if !cd.is_null()
                && (*cd).cmap_id >= 0 as ::core::ffi::c_int
                && !(*cd).taintkeys.is_null()
            {
                let mut cmap: *mut CMap = CMap_cache_get((*cd).cmap_id);
                if need_reencode(kp, vp, cd) != 0 {
                    r = reencode_string(cmap, vp);
                }
            } else if dpx_conf.compat_mode as ::core::ffi::c_uint
                == dpx_mode_xdv_mode as ::core::ffi::c_int as ::core::ffi::c_uint
                && !cd.is_null()
                && !(*cd).taintkeys.is_null()
            {
                if need_reencode(kp, vp, cd) != 0 {
                    r = reencode_string_from_utf8_to_utf16be(vp);
                }
            }
            if r < 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Input string conversion (to UTF16BE) failed for %s...\0" as *const u8
                        as *const ::core::ffi::c_char,
                    pdf_name_value(kp),
                );
            }
        }
        PDF_ARRAY => {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while (i as ::core::ffi::c_uint) < pdf_array_length(vp) {
                let mut obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                obj = pdf_get_array(vp, i);
                r = modify_strings(kp, obj, dp);
                if r < 0 as ::core::ffi::c_int {
                    break;
                }
                i += 1;
            }
        }
        PDF_DICT => {
            r = pdf_foreach_dict(
                vp,
                Some(
                    modify_strings
                        as unsafe extern "C-unwind" fn(
                            *mut pdf_obj,
                            *mut pdf_obj,
                            *mut ::core::ffi::c_void,
                        )
                            -> ::core::ffi::c_int,
                ),
                dp,
            );
        }
        PDF_STREAM => {
            r = pdf_foreach_dict(
                pdf_stream_dict(vp),
                Some(
                    modify_strings
                        as unsafe extern "C-unwind" fn(
                            *mut pdf_obj,
                            *mut pdf_obj,
                            *mut ::core::ffi::c_void,
                        )
                            -> ::core::ffi::c_int,
                ),
                dp,
            );
        }
        _ => {}
    }
    return r;
}
unsafe extern "C-unwind" fn parse_pdf_dict_with_tounicode(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut cd: *mut tounicode,
) -> *mut pdf_obj {
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if dpx_conf.compat_mode as ::core::ffi::c_uint
        != dpx_mode_xdv_mode as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cd).cmap_id < 0 as ::core::ffi::c_int
    {
        dict = parse_pdf_object_extended(
            pp,
            endptr,
            ::core::ptr::null_mut::<pdf_file>(),
            Some(
                parse_pdf_reference
                    as unsafe extern "C-unwind" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *mut pdf_obj,
            ),
            NULL,
        );
        if !dict.is_null() && !(!dict.is_null() && pdf_obj_typeof(dict) == PDF_DICT) {
            pdf_warning!(
                b"Dictionary type object expected but non-dictionary type found.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            pdf_release_obj(dict);
            dict = ::core::ptr::null_mut::<pdf_obj>();
        }
    } else {
        if !cd.is_null() && (*cd).unescape_backslash != 0 {
            dict = parse_pdf_tainted_dict(
                pp,
                endptr,
                Some(
                    parse_pdf_reference
                        as unsafe extern "C-unwind" fn(
                            *mut *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut ::core::ffi::c_void,
                        ) -> *mut pdf_obj,
                ),
                NULL,
            );
        } else {
            dict = parse_pdf_object_extended(
                pp,
                endptr,
                ::core::ptr::null_mut::<pdf_file>(),
                Some(
                    parse_pdf_reference
                        as unsafe extern "C-unwind" fn(
                            *mut *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                            *mut ::core::ffi::c_void,
                        ) -> *mut pdf_obj,
                ),
                NULL,
            );
        }
        if !dict.is_null() {
            if !(!dict.is_null() && pdf_obj_typeof(dict) == PDF_DICT) {
                pdf_warning!(
                    b"Dictionary type object expected but non-dictionary type found.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                pdf_release_obj(dict);
                dict = ::core::ptr::null_mut::<pdf_obj>();
            } else {
                pdf_foreach_dict(
                    dict,
                    Some(
                        modify_strings
                            as unsafe extern "C-unwind" fn(
                                *mut pdf_obj,
                                *mut pdf_obj,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    cd as *mut ::core::ffi::c_void,
                );
            }
        }
    }
    return dict;
}
unsafe extern "C-unwind" fn set_rect_for_annot(
    mut spe: *mut spc_env,
    mut rect: *mut pdf_rect,
    mut ti: transform_info,
) {
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut cp1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut cp2: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut cp3: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut cp4: pdf_coord = pdf_coord { x: 0., y: 0. };
    spc_get_current_point(spe, &raw mut cp);
    if ti.flags & INFO_HAS_USER_BBOX != 0 {
        cp1.x = cp.x + ti.bbox.llx;
        cp1.y = cp.y + ti.bbox.lly;
        cp2.x = cp.x + ti.bbox.urx;
        cp2.y = cp.y + ti.bbox.lly;
        cp3.x = cp.x + ti.bbox.urx;
        cp3.y = cp.y + ti.bbox.ury;
        cp4.x = cp.x + ti.bbox.llx;
        cp4.y = cp.y + ti.bbox.ury;
    } else {
        cp1.x = cp.x;
        cp1.y = cp.y - (*spe).mag * ti.depth;
        cp2.x = cp.x + (*spe).mag * ti.width;
        cp2.y = cp.y - (*spe).mag * ti.depth;
        cp3.x = cp.x + (*spe).mag * ti.width;
        cp3.y = cp.y + (*spe).mag * ti.height;
        cp4.x = cp.x;
        cp4.y = cp.y + (*spe).mag * ti.height;
    }
    pdf_dev_transform(&raw mut cp1, ::core::ptr::null::<pdf_tmatrix>());
    pdf_dev_transform(&raw mut cp2, ::core::ptr::null::<pdf_tmatrix>());
    pdf_dev_transform(&raw mut cp3, ::core::ptr::null::<pdf_tmatrix>());
    pdf_dev_transform(&raw mut cp4, ::core::ptr::null::<pdf_tmatrix>());
    (*rect).llx = min4(cp1.x, cp2.x, cp3.x, cp4.x);
    (*rect).lly = min4(cp1.y, cp2.y, cp3.y, cp4.y);
    (*rect).urx = max4(cp1.x, cp2.x, cp3.x, cp4.x);
    (*rect).ury = max4(cp1.y, cp2.y, cp3.y, cp4.y);
}
unsafe extern "C-unwind" fn spc_handler_pdfm_annot(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    let mut annot_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut rect: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ti: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '@' as i32
    {
        ident = parse_opt_ident(&raw mut (*args).curptr, (*args).endptr);
        skip_white(&raw mut (*args).curptr, (*args).endptr);
    }
    transform_info_clear(&raw mut ti);
    if spc_util_read_dimtrns(spe, &raw mut ti, args, 0 as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    if ti.flags & INFO_HAS_USER_BBOX != 0
        && (ti.flags & INFO_HAS_WIDTH != 0 || ti.flags & INFO_HAS_HEIGHT != 0)
    {
        pdf_warning!(
            b"You can't specify both bbox and width/height.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    annot_dict =
        parse_pdf_dict_with_tounicode(&raw mut (*args).curptr, (*args).endptr, &raw mut (*sd).cd);
    if annot_dict.is_null() {
        pdf_warning!(
            b"Could not find dictionary object.\0" as *const u8 as *const ::core::ffi::c_char
        );
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    } else if !(!annot_dict.is_null() && pdf_obj_typeof(annot_dict) == PDF_DICT) {
        pdf_warning!(
            b"Invalid type: not dictionary object.\0" as *const u8 as *const ::core::ffi::c_char
        );
        free(ident as *mut ::core::ffi::c_void);
        pdf_release_obj(annot_dict);
        return -(1 as ::core::ffi::c_int);
    }
    set_rect_for_annot(spe, &raw mut rect, ti);
    if !ident.is_null() {
        spc_push_object(spe, ident, pdf_link_obj(annot_dict));
    }
    pdf_doc_add_annot(
        pdf_doc_current_page_number() as ::core::ffi::c_uint,
        &raw mut rect,
        annot_dict,
        1 as ::core::ffi::c_int,
    );
    if !ident.is_null() {
        free(ident as *mut ::core::ffi::c_void);
    }
    pdf_release_obj(annot_dict);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_bann(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*sd).annot_dict.is_null() {
        pdf_warning!(
            b"Can't begin an annotation when one is pending.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '@' as i32
    {
        ident = parse_opt_ident(&raw mut (*args).curptr, (*args).endptr);
        skip_white(&raw mut (*args).curptr, (*args).endptr);
    }
    (*sd).annot_dict =
        parse_pdf_dict_with_tounicode(&raw mut (*args).curptr, (*args).endptr, &raw mut (*sd).cd);
    if (*sd).annot_dict.is_null() {
        pdf_warning!(
            b"Ignoring annotation with invalid dictionary.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        if !ident.is_null() {
            free(ident as *mut ::core::ffi::c_void);
        }
        return -(1 as ::core::ffi::c_int);
    } else if !(!(*sd).annot_dict.is_null() && pdf_obj_typeof((*sd).annot_dict) == PDF_DICT) {
        pdf_warning!(
            b"Invalid type: not a dictionary object.\0" as *const u8 as *const ::core::ffi::c_char
        );
        pdf_release_obj((*sd).annot_dict);
        (*sd).annot_dict = ::core::ptr::null_mut::<pdf_obj>();
        if !ident.is_null() {
            free(ident as *mut ::core::ffi::c_void);
        }
        return -(1 as ::core::ffi::c_int);
    }
    error = spc_begin_annot(spe, pdf_link_obj((*sd).annot_dict));
    if !ident.is_null() {
        spc_push_object(spe, ident, pdf_link_obj((*sd).annot_dict));
        free(ident as *mut ::core::ffi::c_void);
    }
    return error;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_eann(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*sd).annot_dict.is_null() {
        pdf_warning!(
            b"Tried to end an annotation without starting one!\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    error = spc_end_annot(spe);
    pdf_release_obj((*sd).annot_dict);
    (*sd).annot_dict = ::core::ptr::null_mut::<pdf_obj>();
    return error;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_xann(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut rect: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut ti: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    if spc_is_tracking_boxes(spe) == 0 {
        (*args).curptr = (*args).endptr;
        return 0 as ::core::ffi::c_int;
    }
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    transform_info_clear(&raw mut ti);
    if spc_util_read_dimtrns(spe, &raw mut ti, args, 0 as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if ti.flags & INFO_HAS_USER_BBOX != 0
        && (ti.flags & INFO_HAS_WIDTH != 0 || ti.flags & INFO_HAS_HEIGHT != 0)
    {
        pdf_warning!(
            b"You can't specify both bbox and width/height.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    set_rect_for_annot(spe, &raw mut rect, ti);
    pdf_doc_expand_box(&raw mut rect);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_bcolor(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fc: pdf_color = pdf_color {
        res_id: 0,
        type_0: 0,
        num_components: 0,
        spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        values: [0.; 32],
        pattern_id: 0,
    };
    let mut sc: pdf_color = pdf_color {
        res_id: 0,
        type_0: 0,
        num_components: 0,
        spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        values: [0.; 32],
        pattern_id: 0,
    };
    let mut pfc: *mut pdf_color = ::core::ptr::null_mut::<pdf_color>();
    let mut psc: *mut pdf_color = ::core::ptr::null_mut::<pdf_color>();
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    pdf_color_get_current(&raw mut psc, &raw mut pfc);
    if (*ap).curptr < (*ap).endptr
        && (*(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'f' as i32
            || *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 's' as i32)
    {
        pdf_color_copycolor(&raw mut sc, psc);
        pdf_color_copycolor(&raw mut fc, pfc);
        while error == 0 && (*ap).curptr < (*ap).endptr {
            if (*ap).curptr
                <= (*ap)
                    .endptr
                    .offset(strlen(b"fill\0" as *const u8 as *const ::core::ffi::c_char) as isize)
                && memcmp(
                    (*ap).curptr as *const ::core::ffi::c_void,
                    b"fill\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"fill\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            {
                (*ap).curptr = (*ap)
                    .curptr
                    .offset(strlen(b"fill\0" as *const u8 as *const ::core::ffi::c_char) as isize);
                skip_white(&raw mut (*ap).curptr, (*ap).endptr);
                error = spc_util_read_pdfcolor(spe, &raw mut fc, ap, pfc);
            } else if (*ap).curptr
                <= (*ap)
                    .endptr
                    .offset(strlen(b"stroke\0" as *const u8 as *const ::core::ffi::c_char) as isize)
                && memcmp(
                    (*ap).curptr as *const ::core::ffi::c_void,
                    b"stroke\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"stroke\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            {
                (*ap).curptr = (*ap).curptr.offset(strlen(
                    b"stroke\0" as *const u8 as *const ::core::ffi::c_char,
                ) as isize);
                skip_white(&raw mut (*ap).curptr, (*ap).endptr);
                error = spc_util_read_pdfcolor(spe, &raw mut sc, ap, psc);
            }
            skip_white(&raw mut (*ap).curptr, (*ap).endptr);
        }
    } else {
        error = spc_util_read_pdfcolor(spe, &raw mut fc, ap, pfc);
        if error == 0 {
            if (*ap).curptr < (*ap).endptr {
                error = spc_util_read_pdfcolor(spe, &raw mut sc, ap, psc);
            } else {
                pdf_color_copycolor(&raw mut sc, &raw mut fc);
            }
        }
    }
    if error != 0 {
        pdf_warning!(b"Invalid color specification?\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        skip_white(&raw mut (*ap).curptr, (*ap).endptr);
        pdf_color_push(&raw mut sc, &raw mut fc);
    }
    return error;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_scolor(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fc: pdf_color = pdf_color {
        res_id: 0,
        type_0: 0,
        num_components: 0,
        spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        values: [0.; 32],
        pattern_id: 0,
    };
    let mut sc: pdf_color = pdf_color {
        res_id: 0,
        type_0: 0,
        num_components: 0,
        spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        values: [0.; 32],
        pattern_id: 0,
    };
    let mut pfc: *mut pdf_color = ::core::ptr::null_mut::<pdf_color>();
    let mut psc: *mut pdf_color = ::core::ptr::null_mut::<pdf_color>();
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    pdf_color_get_current(&raw mut psc, &raw mut pfc);
    if (*ap).curptr < (*ap).endptr
        && (*(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 'f' as i32
            || *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 's' as i32)
    {
        pdf_color_copycolor(&raw mut sc, psc);
        pdf_color_copycolor(&raw mut fc, pfc);
        while error == 0 && (*ap).curptr < (*ap).endptr {
            if (*ap).curptr
                <= (*ap)
                    .endptr
                    .offset(strlen(b"fill\0" as *const u8 as *const ::core::ffi::c_char) as isize)
                && memcmp(
                    (*ap).curptr as *const ::core::ffi::c_void,
                    b"fill\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"fill\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            {
                (*ap).curptr = (*ap)
                    .curptr
                    .offset(strlen(b"fill\0" as *const u8 as *const ::core::ffi::c_char) as isize);
                skip_white(&raw mut (*ap).curptr, (*ap).endptr);
                error = spc_util_read_pdfcolor(spe, &raw mut fc, ap, pfc);
            } else if (*ap).curptr
                <= (*ap)
                    .endptr
                    .offset(strlen(b"stroke\0" as *const u8 as *const ::core::ffi::c_char) as isize)
                && memcmp(
                    (*ap).curptr as *const ::core::ffi::c_void,
                    b"stroke\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"stroke\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            {
                (*ap).curptr = (*ap).curptr.offset(strlen(
                    b"stroke\0" as *const u8 as *const ::core::ffi::c_char,
                ) as isize);
                skip_white(&raw mut (*ap).curptr, (*ap).endptr);
                error = spc_util_read_pdfcolor(spe, &raw mut sc, ap, psc);
            }
            skip_white(&raw mut (*ap).curptr, (*ap).endptr);
        }
    } else {
        error = spc_util_read_pdfcolor(spe, &raw mut fc, ap, pfc);
        if error == 0 {
            if (*ap).curptr < (*ap).endptr {
                error = spc_util_read_pdfcolor(spe, &raw mut sc, ap, psc);
            } else {
                pdf_color_copycolor(&raw mut sc, &raw mut fc);
            }
        }
    }
    if error != 0 {
        pdf_warning!(b"Invalid color specification?\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        pdf_color_set(&raw mut sc, &raw mut fc);
    }
    return error;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_ecolor(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    pdf_color_pop();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_btrans(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut ti: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    transform_info_clear(&raw mut ti);
    if spc_util_read_dimtrns(spe, &raw mut ti, args, 0 as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    spc_get_current_point(spe, &raw mut cp);
    M.a = ti.matrix.a;
    M.b = ti.matrix.b;
    M.c = ti.matrix.c;
    M.d = ti.matrix.d;
    M.e = ti.matrix.e;
    M.f = ti.matrix.f;
    M.e += (1.0f64 - M.a) * cp.x - M.c * cp.y;
    M.f += (1.0f64 - M.d) * cp.y - M.b * cp.x;
    pdf_dev_gsave();
    pdf_dev_concat(&raw mut M);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_etrans(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    pdf_dev_grestore();
    pdf_dev_reset_color(0 as ::core::ffi::c_int);
    pdf_dev_reset_xgstate(0 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_outline(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    let mut item_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut level: ::core::ffi::c_int = 0;
    let mut is_open: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut current_depth: ::core::ffi::c_int = 0;
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr.offset(3 as ::core::ffi::c_int as isize) < (*args).endptr
        && *(*args).curptr as ::core::ffi::c_int == '[' as i32
    {
        (*args).curptr = (*args).curptr.offset(1);
        if *(*args).curptr as ::core::ffi::c_int == '-' as i32 {
            (*args).curptr = (*args).curptr.offset(1);
        } else {
            is_open = 1 as ::core::ffi::c_int;
        }
        (*args).curptr = (*args).curptr.offset(1);
    }
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    tmp = parse_pdf_object(
        &raw mut (*args).curptr,
        (*args).endptr,
        ::core::ptr::null_mut::<pdf_file>(),
    );
    if tmp.is_null() {
        pdf_warning!(
            b"Missing number for outline item depth.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    } else if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NUMBER) {
        pdf_release_obj(tmp);
        pdf_warning!(
            b"Expecting number for outline item depth.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    item_dict = ::core::ptr::null_mut::<pdf_obj>();
    level = pdf_number_value(tmp) as ::core::ffi::c_int;
    pdf_release_obj(tmp);
    (*sd).lowest_level = if (*sd).lowest_level < level {
        (*sd).lowest_level
    } else {
        level
    };
    level += 1 as ::core::ffi::c_int - (*sd).lowest_level;
    item_dict =
        parse_pdf_dict_with_tounicode(&raw mut (*args).curptr, (*args).endptr, &raw mut (*sd).cd);
    if item_dict.is_null() {
        pdf_warning!(b"Ignoring invalid dictionary.\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    current_depth = pdf_doc_bookmarks_depth();
    if current_depth > level {
        loop {
            let fresh9 = current_depth;
            current_depth = current_depth - 1;
            if !(fresh9 > level) {
                break;
            }
            pdf_doc_bookmarks_up();
        }
    } else if current_depth < level {
        loop {
            let fresh10 = current_depth;
            current_depth = current_depth + 1;
            if !(fresh10 < level) {
                break;
            }
            pdf_doc_bookmarks_down();
        }
    }
    pdf_doc_bookmarks_add(item_dict, is_open);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_article(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut info_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&raw mut (*args).curptr, (*args).endptr);
    if ident.is_null() {
        pdf_warning!(
            b"Article name expected but not found.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    info_dict =
        parse_pdf_dict_with_tounicode(&raw mut (*args).curptr, (*args).endptr, &raw mut (*sd).cd);
    if info_dict.is_null() {
        pdf_warning!(
            b"Ignoring article with invalid info dictionary.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    pdf_doc_begin_article(ident, pdf_link_obj(info_dict));
    spc_push_object(spe, ident, info_dict);
    free(ident as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_bead(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    let mut article: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut article_info: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut article_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut rect: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut page_no: ::core::ffi::c_int = 0;
    let mut ti: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '@' as i32
    {
        pdf_warning!(
            b"Article identifier expected but not found.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    article_name = parse_opt_ident(&raw mut (*args).curptr, (*args).endptr);
    if article_name.is_null() {
        pdf_warning!(
            b"Article reference expected but not found.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    transform_info_clear(&raw mut ti);
    if spc_util_read_dimtrns(spe, &raw mut ti, args, 0 as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        free(article_name as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    if ti.flags & INFO_HAS_USER_BBOX != 0
        && (ti.flags & INFO_HAS_WIDTH != 0 || ti.flags & INFO_HAS_HEIGHT != 0)
    {
        pdf_warning!(
            b"You can't specify both bbox and width/height.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        free(article_name as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '<' as i32
    {
        article_info = pdf_new_dict();
    } else {
        article_info = parse_pdf_dict_with_tounicode(
            &raw mut (*args).curptr,
            (*args).endptr,
            &raw mut (*sd).cd,
        );
        if article_info.is_null() {
            pdf_warning!(
                b"Error in reading dictionary.\0" as *const u8 as *const ::core::ffi::c_char
            );
            free(article_name as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
    }
    article = spc_lookup_object(article_name);
    if !article.is_null() {
        pdf_merge_dict(article, article_info);
        pdf_release_obj(article_info);
    } else {
        pdf_doc_begin_article(article_name, pdf_link_obj(article_info));
        spc_push_object(spe, article_name, article_info);
    }
    page_no = pdf_doc_current_page_number();
    set_rect_for_annot(spe, &raw mut rect, ti);
    pdf_doc_add_bead(
        article_name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        page_no,
        &raw mut rect,
    );
    free(article_name as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_image(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut xobj_id: ::core::ffi::c_int = 0;
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fspec: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut ti: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    let mut options: load_options = load_options {
        page_no: 1 as ::core::ffi::c_int,
        bbox_type: pdf_page_boundary__auto,
        dict: ::core::ptr::null_mut::<pdf_obj>(),
    };
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '@' as i32
    {
        ident = parse_opt_ident(&raw mut (*args).curptr, (*args).endptr);
        skip_white(&raw mut (*args).curptr, (*args).endptr);
    }
    transform_info_clear(&raw mut ti);
    if spc_util_read_blahblah(
        spe,
        &raw mut ti,
        &raw mut options.page_no,
        &raw mut options.bbox_type,
        args,
    ) < 0 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Reading option field in pdf:image failed.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    fspec = parse_pdf_object(
        &raw mut (*args).curptr,
        (*args).endptr,
        ::core::ptr::null_mut::<pdf_file>(),
    );
    if fspec.is_null() {
        pdf_warning!(
            b"Missing filename string for pdf:image.\0" as *const u8 as *const ::core::ffi::c_char
        );
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    } else if !(!fspec.is_null() && pdf_obj_typeof(fspec) == PDF_STRING) {
        pdf_warning!(
            b"Missing filename string for pdf:image.\0" as *const u8 as *const ::core::ffi::c_char
        );
        pdf_release_obj(fspec);
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        options.dict = parse_pdf_object_extended(
            &raw mut (*args).curptr,
            (*args).endptr,
            ::core::ptr::null_mut::<pdf_file>(),
            Some(
                parse_pdf_reference
                    as unsafe extern "C-unwind" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *mut pdf_obj,
            ),
            spe as *mut ::core::ffi::c_void,
        );
    }
    xobj_id = pdf_ximage_load_image(
        ident,
        pdf_string_value(fspec) as *const ::core::ffi::c_char,
        options,
    );
    if xobj_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Could not find image resource...\0" as *const u8 as *const ::core::ffi::c_char
        );
        pdf_release_obj(fspec);
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    if ti.flags & INFO_DO_HIDE == 0 {
        spc_put_image(spe, xobj_id, &raw mut ti, (*spe).x_user, (*spe).y_user);
    }
    if !ident.is_null() {
        if dpx_conf.compat_mode as ::core::ffi::c_uint
            == dpx_mode_compat_mode as ::core::ffi::c_int as ::core::ffi::c_uint
            && pdf_ximage_get_subtype(xobj_id) == PDF_XOBJECT_TYPE_IMAGE
        {
            pdf_ximage_set_attr(
                xobj_id,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                1.0f64,
                1.0f64,
                0.0f64,
                0.0f64,
                0.0f64,
                0.0f64,
            );
        }
        free(ident as *mut ::core::ffi::c_void);
    }
    pdf_release_obj(fspec);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_dest(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut name: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    name = parse_pdf_object(
        &raw mut (*args).curptr,
        (*args).endptr,
        ::core::ptr::null_mut::<pdf_file>(),
    );
    if name.is_null() {
        pdf_warning!(
            b"PDF string expected for destination name but not found.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    } else if !(!name.is_null() && pdf_obj_typeof(name) == PDF_STRING) {
        pdf_warning!(
            b"PDF string expected for destination name but invalid type.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        pdf_release_obj(name);
        return -(1 as ::core::ffi::c_int);
    }
    array = parse_pdf_object_extended(
        &raw mut (*args).curptr,
        (*args).endptr,
        ::core::ptr::null_mut::<pdf_file>(),
        Some(
            parse_pdf_reference
                as unsafe extern "C-unwind" fn(
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> *mut pdf_obj,
        ),
        spe as *mut ::core::ffi::c_void,
    );
    if array.is_null() {
        pdf_warning!(
            b"No destination specified for pdf:dest.\0" as *const u8 as *const ::core::ffi::c_char
        );
        pdf_release_obj(name);
        return -(1 as ::core::ffi::c_int);
    } else if !(!array.is_null() && pdf_obj_typeof(array) == PDF_ARRAY) {
        pdf_warning!(
            b"Destination not specified as an array object!\0" as *const u8
                as *const ::core::ffi::c_char
        );
        pdf_release_obj(name);
        pdf_release_obj(array);
        return -(1 as ::core::ffi::c_int);
    }
    pdf_doc_add_names(
        b"Dests\0" as *const u8 as *const ::core::ffi::c_char,
        pdf_string_value(name),
        pdf_string_length(name) as ::core::ffi::c_int,
        array,
    );
    pdf_release_obj(name);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_names(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut category: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut key: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut value: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut i: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    category = parse_pdf_object(
        &raw mut (*args).curptr,
        (*args).endptr,
        ::core::ptr::null_mut::<pdf_file>(),
    );
    if category.is_null() {
        pdf_warning!(
            b"PDF name expected but not found.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    } else if !(!category.is_null() && pdf_obj_typeof(category) == PDF_NAME) {
        pdf_warning!(
            b"PDF name expected but not found.\0" as *const u8 as *const ::core::ffi::c_char
        );
        pdf_release_obj(category);
        return -(1 as ::core::ffi::c_int);
    }
    tmp = parse_pdf_object_extended(
        &raw mut (*args).curptr,
        (*args).endptr,
        ::core::ptr::null_mut::<pdf_file>(),
        Some(
            parse_pdf_reference
                as unsafe extern "C-unwind" fn(
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> *mut pdf_obj,
        ),
        spe as *mut ::core::ffi::c_void,
    );
    if tmp.is_null() {
        pdf_warning!(
            b"PDF object expected but not found.\0" as *const u8 as *const ::core::ffi::c_char
        );
        pdf_release_obj(category);
        return -(1 as ::core::ffi::c_int);
    } else if !tmp.is_null() && pdf_obj_typeof(tmp) == PDF_ARRAY {
        size = pdf_array_length(tmp) as ::core::ffi::c_int;
        if size % 2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            pdf_warning!(
                b"Array size not multiple of 2 for pdf:names.\0" as *const u8
                    as *const ::core::ffi::c_char
            );
            pdf_release_obj(category);
            pdf_release_obj(tmp);
            return -(1 as ::core::ffi::c_int);
        }
        i = 0 as ::core::ffi::c_int;
        while i < size / 2 as ::core::ffi::c_int {
            key = pdf_get_array(tmp, 2 as ::core::ffi::c_int * i);
            value = pdf_get_array(tmp, 2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int);
            if !(!key.is_null() && pdf_obj_typeof(key) == PDF_STRING) {
                pdf_warning!(
                    b"Name tree key must be string.\0" as *const u8 as *const ::core::ffi::c_char
                );
                pdf_release_obj(category);
                pdf_release_obj(tmp);
                return -(1 as ::core::ffi::c_int);
            } else if pdf_doc_add_names(
                pdf_name_value(category),
                pdf_string_value(key),
                pdf_string_length(key) as ::core::ffi::c_int,
                pdf_link_obj(value),
            ) < 0 as ::core::ffi::c_int
            {
                pdf_warning!(
                    b"Failed to add Name tree entry...\0" as *const u8
                        as *const ::core::ffi::c_char
                );
                pdf_release_obj(category);
                pdf_release_obj(tmp);
                return -(1 as ::core::ffi::c_int);
            }
            i += 1;
        }
        pdf_release_obj(tmp);
    } else if !tmp.is_null() && pdf_obj_typeof(tmp) == PDF_STRING {
        key = tmp;
        value = parse_pdf_object_extended(
            &raw mut (*args).curptr,
            (*args).endptr,
            ::core::ptr::null_mut::<pdf_file>(),
            Some(
                parse_pdf_reference
                    as unsafe extern "C-unwind" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *mut pdf_obj,
            ),
            spe as *mut ::core::ffi::c_void,
        );
        if value.is_null() {
            pdf_release_obj(category);
            pdf_release_obj(key);
            pdf_warning!(
                b"PDF object expected but not found.\0" as *const u8 as *const ::core::ffi::c_char
            );
            return -(1 as ::core::ffi::c_int);
        }
        if pdf_doc_add_names(
            pdf_name_value(category),
            pdf_string_value(key),
            pdf_string_length(key) as ::core::ffi::c_int,
            value,
        ) < 0 as ::core::ffi::c_int
        {
            pdf_warning!(
                b"Failed to add Name tree entry...\0" as *const u8 as *const ::core::ffi::c_char
            );
            pdf_release_obj(category);
            pdf_release_obj(key);
            return -(1 as ::core::ffi::c_int);
        }
        pdf_release_obj(key);
    } else {
        pdf_release_obj(tmp);
        pdf_release_obj(category);
        pdf_warning!(
            b"Invalid object type for pdf:names.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    pdf_release_obj(category);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_docinfo(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    let mut docinfo: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    dict =
        parse_pdf_dict_with_tounicode(&raw mut (*args).curptr, (*args).endptr, &raw mut (*sd).cd);
    if dict.is_null() {
        pdf_warning!(
            b"Dictionary object expected but not found.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    docinfo = pdf_doc_get_dictionary(b"Info\0" as *const u8 as *const ::core::ffi::c_char);
    pdf_merge_dict(docinfo, dict);
    pdf_release_obj(dict);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_docview(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    let mut catalog: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut pref_old: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut pref_add: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    dict =
        parse_pdf_dict_with_tounicode(&raw mut (*args).curptr, (*args).endptr, &raw mut (*sd).cd);
    if dict.is_null() {
        pdf_warning!(
            b"Dictionary object expected but not found.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    catalog = pdf_doc_get_dictionary(b"Catalog\0" as *const u8 as *const ::core::ffi::c_char);
    pref_old = pdf_lookup_dict(
        catalog,
        b"ViewerPreferences\0" as *const u8 as *const ::core::ffi::c_char,
    );
    pref_add = pdf_lookup_dict(
        dict,
        b"ViewerPreferences\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !pref_old.is_null() && !pref_add.is_null() {
        pdf_merge_dict(pref_old, pref_add);
        pdf_remove_dict(
            dict,
            b"ViewerPreferences\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    pdf_merge_dict(catalog, dict);
    pdf_release_obj(dict);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_close(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&raw mut (*args).curptr, (*args).endptr);
    if !ident.is_null() {
        spc_flush_object(spe, ident);
        free(ident as *mut ::core::ffi::c_void);
    } else {
        pdf_warning!(
            b"pdf:close without an argument no longer supported!\0" as *const u8
                as *const ::core::ffi::c_char
        );
        spc_clear_objects(spe);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_object(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut object: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&raw mut (*args).curptr, (*args).endptr);
    if ident.is_null() {
        pdf_warning!(
            b"Could not find a object identifier.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    object = parse_pdf_object_extended(
        &raw mut (*args).curptr,
        (*args).endptr,
        ::core::ptr::null_mut::<pdf_file>(),
        Some(
            parse_pdf_reference
                as unsafe extern "C-unwind" fn(
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> *mut pdf_obj,
        ),
        spe as *mut ::core::ffi::c_void,
    );
    if object.is_null() {
        pdf_warning!(
            b"Could not find an object definition for \"%s\".\0" as *const u8
                as *const ::core::ffi::c_char,
            ident
        );
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    } else {
        spc_push_object(spe, ident, object);
    }
    free(ident as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_content(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        let mut M: pdf_tmatrix = pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        };
        let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
        spc_get_current_point(spe, &raw mut cp);
        M.a = 1.0f64;
        M.b = 0.0f64;
        M.c = 0.0f64;
        M.d = 1.0f64;
        M.e = cp.x;
        M.f = cp.y;
        let fresh2 = len;
        len = len + 1;
        *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh2 as isize) =
            ' ' as i32 as ::core::ffi::c_char;
        let fresh3 = len;
        len = len + 1;
        *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh3 as isize) =
            'q' as i32 as ::core::ffi::c_char;
        let fresh4 = len;
        len = len + 1;
        *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh4 as isize) =
            ' ' as i32 as ::core::ffi::c_char;
        len += pdf_sprint_matrix(
            (&raw mut work_buffer as *mut ::core::ffi::c_char).offset(len as isize),
            &raw mut M,
        );
        let fresh5 = len;
        len = len + 1;
        *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh5 as isize) =
            ' ' as i32 as ::core::ffi::c_char;
        let fresh6 = len;
        len = len + 1;
        *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh6 as isize) =
            'c' as i32 as ::core::ffi::c_char;
        let fresh7 = len;
        len = len + 1;
        *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh7 as isize) =
            'm' as i32 as ::core::ffi::c_char;
        let fresh8 = len;
        len = len + 1;
        *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh8 as isize) =
            ' ' as i32 as ::core::ffi::c_char;
        pdf_doc_add_page_content(
            &raw mut work_buffer as *mut ::core::ffi::c_char,
            len as ::core::ffi::c_uint,
        );
        len =
            (*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long as ::core::ffi::c_int;
        pdf_doc_add_page_content((*args).curptr, len as ::core::ffi::c_uint);
        pdf_doc_add_page_content(
            b" Q\0" as *const u8 as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_uint,
        );
    }
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_literal(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut direct: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    while (*args).curptr < (*args).endptr {
        if (*args).curptr.offset(7 as ::core::ffi::c_int as isize) <= (*args).endptr
            && !strstartswith(
                (*args).curptr,
                b"reverse\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            (*args).curptr = (*args).curptr.offset(7 as ::core::ffi::c_int as isize);
            pdf_warning!(
                b"The special \"pdf:literal reverse ...\" is no longer supported.\nIgnore the \"reverse\" option.\0"
                as * const u8 as * const ::core::ffi::c_char,
            );
        } else {
            if !((*args).curptr.offset(6 as ::core::ffi::c_int as isize) <= (*args).endptr
                && !strstartswith(
                    (*args).curptr,
                    b"direct\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null())
            {
                break;
            }
            direct = 1 as ::core::ffi::c_int;
            (*args).curptr = (*args).curptr.offset(6 as ::core::ffi::c_int as isize);
        }
        skip_white(&raw mut (*args).curptr, (*args).endptr);
    }
    if (*args).curptr < (*args).endptr {
        let mut M: pdf_tmatrix = pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        };
        let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
        spc_get_current_point(spe, &raw mut cp);
        if direct == 0 {
            M.d = 1.0f64;
            M.a = M.d;
            M.c = 0.0f64;
            M.b = M.c;
            M.e = cp.x;
            M.f = cp.y;
            pdf_dev_concat(&raw mut M);
        }
        pdf_doc_add_page_content(
            b" \0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_uint,
        );
        pdf_doc_add_page_content(
            (*args).curptr,
            (*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
        if direct == 0 {
            M.e = -cp.x;
            M.f = -cp.y;
            pdf_dev_concat(&raw mut M);
        }
    }
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_bcontent(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut xpos: ::core::ffi::c_double = 0.;
    let mut ypos: ::core::ffi::c_double = 0.;
    pdf_dev_gsave();
    spc_get_coord(spe, &raw mut xpos, &raw mut ypos);
    M.a = 1.0f64;
    M.b = 0.0f64;
    M.c = 0.0f64;
    M.d = 1.0f64;
    M.e = (*spe).x_user - xpos;
    M.f = (*spe).y_user - ypos;
    pdf_dev_concat(&raw mut M);
    spc_push_coord(spe, (*spe).x_user, (*spe).y_user);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_econtent(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    spc_pop_coord(spe);
    pdf_dev_grestore();
    pdf_dev_reset_color(0 as ::core::ffi::c_int);
    pdf_dev_reset_xgstate(0 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_code(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        pdf_doc_add_page_content(
            b" \0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_uint,
        );
        pdf_doc_add_page_content(
            (*args).curptr,
            (*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
        (*args).curptr = (*args).endptr;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_do_nothing(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
pub const STRING_STREAM: ::core::ffi::c_int = 0;
pub const FILE_STREAM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn spc_handler_pdfm_stream_with_type(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
    mut type_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fstream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut nb_read: ssize_t = 0;
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut instring: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&raw mut (*args).curptr, (*args).endptr);
    if ident.is_null() {
        pdf_warning!(
            b"Missing objname for pdf:(f)stream.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    tmp = parse_pdf_object(
        &raw mut (*args).curptr,
        (*args).endptr,
        ::core::ptr::null_mut::<pdf_file>(),
    );
    if tmp.is_null() {
        pdf_warning!(
            b"Missing input string for pdf:(f)stream.\0" as *const u8 as *const ::core::ffi::c_char
        );
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    } else if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_STRING) {
        pdf_warning!(
            b"Invalid type of input string for pdf:(f)stream.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        pdf_release_obj(tmp);
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    instring = pdf_string_value(tmp) as *mut ::core::ffi::c_char;
    match type_0 {
        FILE_STREAM => {
            if instring.is_null() {
                pdf_warning!(
                    b"Missing filename for pdf:fstream.\0" as *const u8
                        as *const ::core::ffi::c_char
                );
                pdf_release_obj(tmp);
                free(ident as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            handle = ttstub_input_open(instring, TTBC_FILE_FORMAT_PICT, 0 as ::core::ffi::c_int);
            if handle == INVALID_HANDLE {
                pdf_warning!(
                    b"Could not open file: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    instring
                );
                pdf_release_obj(tmp);
                free(ident as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            fstream = pdf_new_stream(STREAM_COMPRESS);
            loop {
                nb_read = ttstub_input_read_partial(
                    handle,
                    &raw mut work_buffer as *mut ::core::ffi::c_char,
                    WORK_BUFFER_SIZE as size_t,
                );
                if !(nb_read > 0 as ssize_t) {
                    break;
                }
                pdf_add_stream(
                    fstream,
                    &raw mut work_buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    nb_read as ::core::ffi::c_int,
                );
            }
            crate::backend::runtime::input_close(handle);
        }
        STRING_STREAM => {
            fstream = pdf_new_stream(STREAM_COMPRESS);
            pdf_add_stream(
                fstream,
                pdf_string_value(tmp),
                pdf_string_length(tmp) as ::core::ffi::c_int,
            );
        }
        _ => {
            pdf_release_obj(tmp);
            free(ident as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
    }
    pdf_release_obj(tmp);
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '<' as i32
    {
        let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        stream_dict = pdf_stream_dict(fstream);
        tmp = parse_pdf_object_extended(
            &raw mut (*args).curptr,
            (*args).endptr,
            ::core::ptr::null_mut::<pdf_file>(),
            Some(
                parse_pdf_reference
                    as unsafe extern "C-unwind" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *mut pdf_obj,
            ),
            spe as *mut ::core::ffi::c_void,
        );
        if tmp.is_null() {
            pdf_warning!(
                b"Parsing dictionary failed.\0" as *const u8 as *const ::core::ffi::c_char
            );
            pdf_release_obj(fstream);
            free(ident as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        } else if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_DICT) {
            pdf_warning!(
                b"Expecting dictionary type object but non-dictionary type found.\0" as *const u8
                    as *const ::core::ffi::c_char
            );
            pdf_release_obj(fstream);
            pdf_release_obj(tmp);
            free(ident as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        if !pdf_lookup_dict(tmp, b"Length\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            pdf_remove_dict(tmp, b"Length\0" as *const u8 as *const ::core::ffi::c_char);
        } else if !pdf_lookup_dict(tmp, b"Filter\0" as *const u8 as *const ::core::ffi::c_char)
            .is_null()
        {
            pdf_remove_dict(tmp, b"Filter\0" as *const u8 as *const ::core::ffi::c_char);
        }
        pdf_merge_dict(stream_dict, tmp);
        pdf_release_obj(tmp);
    }
    spc_push_object(spe, ident, fstream);
    free(ident as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_stream(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    return spc_handler_pdfm_stream_with_type(spe, args, STRING_STREAM);
}
unsafe extern "C-unwind" fn spc_handler_pdfm_fstream(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    return spc_handler_pdfm_stream_with_type(spe, args, FILE_STREAM);
}
unsafe extern "C-unwind" fn spc_handler_pdfm_bform(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0;
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cropbox: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut ti: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&raw mut (*args).curptr, (*args).endptr);
    if ident.is_null() {
        pdf_warning!(
            b"A form XObject must have name.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    transform_info_clear(&raw mut ti);
    if spc_util_read_dimtrns(spe, &raw mut ti, args, 0 as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    if ti.flags & INFO_HAS_USER_BBOX != 0 {
        if ti.bbox.urx - ti.bbox.llx == 0.0f64 || ti.bbox.ury - ti.bbox.lly == 0.0f64 {
            pdf_warning!(
                b"Bounding box has a zero dimension.\0" as *const u8 as *const ::core::ffi::c_char
            );
            free(ident as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        cropbox.llx = ti.bbox.llx;
        cropbox.lly = ti.bbox.lly;
        cropbox.urx = ti.bbox.urx;
        cropbox.ury = ti.bbox.ury;
    } else {
        if ti.width == 0.0f64 || ti.depth + ti.height == 0.0f64 {
            pdf_warning!(
                b"Bounding box has a zero dimension.\0" as *const u8 as *const ::core::ffi::c_char
            );
            free(ident as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        cropbox.llx = 0.0f64;
        cropbox.lly = -ti.depth;
        cropbox.urx = ti.width;
        cropbox.ury = ti.height;
    }
    spc_get_current_point(spe, &raw mut cp);
    error = spc_begin_form(spe, ident, cp, &raw mut cropbox);
    if error < 0 as ::core::ffi::c_int {
        pdf_warning!(b"Couldn't start form object.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    free(ident as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_eform(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0;
    let mut attrib: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        attrib = parse_pdf_object_extended(
            &raw mut (*args).curptr,
            (*args).endptr,
            ::core::ptr::null_mut::<pdf_file>(),
            Some(
                parse_pdf_reference
                    as unsafe extern "C-unwind" fn(
                        *mut *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_void,
                    ) -> *mut pdf_obj,
            ),
            spe as *mut ::core::ffi::c_void,
        );
        if !attrib.is_null() && !(!attrib.is_null() && pdf_obj_typeof(attrib) == PDF_DICT) {
            pdf_release_obj(attrib);
            attrib = ::core::ptr::null_mut::<pdf_obj>();
        }
    }
    if !(*sd).pageresources.is_null() {
        pdf_foreach_dict(
            (*sd).pageresources,
            Some(
                forallresourcecategory
                    as unsafe extern "C-unwind" fn(
                        *mut pdf_obj,
                        *mut pdf_obj,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
        );
    }
    error = spc_end_form(spe, attrib);
    return error;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_uxobj(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut xobj_id: ::core::ffi::c_int = 0;
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ti: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    ident = parse_opt_ident(&raw mut (*args).curptr, (*args).endptr);
    if ident.is_null() {
        pdf_warning!(b"No object identifier given.\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    transform_info_clear(&raw mut ti);
    if (*args).curptr < (*args).endptr {
        if spc_util_read_dimtrns(spe, &raw mut ti, args, 0 as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        {
            free(ident as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
    }
    xobj_id = pdf_ximage_findresource(ident);
    if xobj_id < 0 as ::core::ffi::c_int {
        xobj_id = pdf_ximage_reserve(ident);
    }
    spc_put_image(spe, xobj_id, &raw mut ti, (*spe).x_user, (*spe).y_user);
    free(ident as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_link(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    return spc_resume_annot(spe);
}
unsafe extern "C-unwind" fn spc_handler_pdfm_nolink(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    return spc_suspend_annot(spe);
}
unsafe extern "C-unwind" fn spc_handler_pdfm_pagesize(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_bgcolor(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0;
    let mut colorspec: pdf_color = pdf_color {
        res_id: 0,
        type_0: 0,
        num_components: 0,
        spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        values: [0.; 32],
        pattern_id: 0,
    };
    error = spc_util_read_pdfcolor(
        spe,
        &raw mut colorspec,
        args,
        ::core::ptr::null_mut::<pdf_color>(),
    );
    if error != 0 {
        pdf_warning!(b"No valid color specified?\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        pdf_doc_set_bgcolor(&raw mut colorspec);
    }
    return error;
}
pub const THEBUFFLENGTH: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn spc_handler_pdfm_mapline(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut mrec: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    let mut map_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut opchr: ::core::ffi::c_char = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut count: ::core::ffi::c_int = 0;
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr >= (*ap).endptr {
        pdf_warning!(b"Empty mapline special?\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    opchr = *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize);
    if opchr as ::core::ffi::c_int == '-' as i32 || opchr as ::core::ffi::c_int == '+' as i32 {
        (*ap).curptr = (*ap).curptr.offset(1);
    }
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    match opchr as ::core::ffi::c_int {
        45 => {
            map_name = parse_ident(&raw mut (*ap).curptr, (*ap).endptr);
            if !map_name.is_null() {
                pdf_remove_fontmap_record(map_name);
                free(map_name as *mut ::core::ffi::c_void);
            } else {
                pdf_warning!(
                    b"Invalid fontmap line: Missing TFM name.\0" as *const u8
                        as *const ::core::ffi::c_char
                );
                error = -(1 as ::core::ffi::c_int);
            }
        }
        _ => {
            p = (*ap).curptr;
            q = &raw mut buffer as *mut ::core::ffi::c_char;
            count = 0 as ::core::ffi::c_int;
            while p < (*ap).endptr && count < THEBUFFLENGTH - 1 as ::core::ffi::c_int {
                let fresh0 = p;
                p = p.offset(1);
                let fresh1 = q;
                q = q.offset(1);
                *fresh1 = *fresh0;
                count += 1;
            }
            if count == THEBUFFLENGTH - 1 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Invalid fontmap line: Too long a line.\0" as *const u8
                        as *const ::core::ffi::c_char
                );
                *q = 0 as ::core::ffi::c_char;
                return -(1 as ::core::ffi::c_int);
            }
            *q = '\0' as i32 as ::core::ffi::c_char;
            mrec = new((1 as ::core::ffi::c_int as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<fontmap_rec>() as usize)
                as uint32_t) as *mut fontmap_rec;
            pdf_init_fontmap_record(mrec);
            error = pdf_read_fontmap_line(
                mrec,
                &raw mut buffer as *mut ::core::ffi::c_char,
                (*ap).endptr.offset_from((*ap).curptr) as ::core::ffi::c_long as ::core::ffi::c_int,
                is_pdfm_mapline(&raw mut buffer as *mut ::core::ffi::c_char),
            );
            if error != 0 {
                pdf_warning!(b"Invalid fontmap line.\0" as *const u8 as *const ::core::ffi::c_char);
            } else if opchr as ::core::ffi::c_int == '+' as i32 {
                pdf_append_fontmap_record((*mrec).map_name, mrec);
            } else {
                pdf_insert_fontmap_record((*mrec).map_name, mrec);
            }
            pdf_clear_fontmap_record(mrec);
            free(mrec as *mut ::core::ffi::c_void);
        }
    }
    if error == 0 {
        (*ap).curptr = (*ap).endptr;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_mapfile(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut mapfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mode: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        return 0 as ::core::ffi::c_int;
    }
    match *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
        45 => {
            mode = FONTMAP_RMODE_REMOVE;
            (*args).curptr = (*args).curptr.offset(1);
        }
        43 => {
            mode = FONTMAP_RMODE_APPEND;
            (*args).curptr = (*args).curptr.offset(1);
        }
        _ => {
            mode = FONTMAP_RMODE_REPLACE;
        }
    }
    mapfile = parse_val_ident(&raw mut (*args).curptr, (*args).endptr);
    if mapfile.is_null() {
        pdf_warning!(b"No fontmap file specified.\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    } else {
        error = pdf_load_fontmap_file(mapfile, mode);
    }
    free(mapfile as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_tounicode(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    let mut cmap_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut taint_keys: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    (*sd).cd.cmap_id = -(1 as ::core::ffi::c_int);
    (*sd).cd.unescape_backslash = 0 as ::core::ffi::c_int;
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        pdf_warning!(
            b"Missing CMap name for pdf:tounicode.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    cmap_name = parse_ident(&raw mut (*args).curptr, (*args).endptr);
    if cmap_name.is_null() {
        pdf_warning!(
            b"Missing ToUnicode mapping name...\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*sd).cd.cmap_id = CMap_cache_find(cmap_name);
    if (*sd).cd.cmap_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Failed to load ToUnicode mapping: %s\0" as *const u8 as *const ::core::ffi::c_char,
            cmap_name
        );
        free(cmap_name as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    if (*sd).cd.cmap_id >= 0 as ::core::ffi::c_int {
        if !strstr(
            cmap_name,
            b"RKSJ\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
            || !strstr(
                cmap_name,
                b"B5\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
            || !strstr(
                cmap_name,
                b"GBK\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
            || !strstr(
                cmap_name,
                b"KSC\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            (*sd).cd.unescape_backslash = 1 as ::core::ffi::c_int;
        }
    }
    free(cmap_name as *mut ::core::ffi::c_void);
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        taint_keys = parse_pdf_object(
            &raw mut (*args).curptr,
            (*args).endptr,
            ::core::ptr::null_mut::<pdf_file>(),
        );
        if !taint_keys.is_null() {
            if !taint_keys.is_null() && pdf_obj_typeof(taint_keys) == PDF_ARRAY {
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while (i as ::core::ffi::c_uint) < pdf_array_length(taint_keys) {
                    let mut key: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                    key = pdf_get_array(taint_keys, i);
                    if !key.is_null() && pdf_obj_typeof(key) == PDF_NAME {
                        pdf_add_array((*sd).cd.taintkeys, pdf_link_obj(key));
                    } else {
                        pdf_warning!(
                            b"Invalid argument specified in pdf:tounicode special.\0" as *const u8
                                as *const ::core::ffi::c_char
                        );
                    }
                    i += 1;
                }
            } else {
                pdf_warning!(
                    b"Invalid argument specified in pdf:unicode special.\0" as *const u8
                        as *const ::core::ffi::c_char
                );
            }
            pdf_release_obj(taint_keys);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_pageresources(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_pdf_ = &raw mut _pdf_stat;
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    dict = parse_pdf_object_extended(
        &raw mut (*args).curptr,
        (*args).endptr,
        ::core::ptr::null_mut::<pdf_file>(),
        Some(
            parse_pdf_reference
                as unsafe extern "C-unwind" fn(
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> *mut pdf_obj,
        ),
        spe as *mut ::core::ffi::c_void,
    );
    if dict.is_null() {
        pdf_warning!(
            b"Dictionary object expected but not found.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    if !(*sd).pageresources.is_null() {
        pdf_release_obj((*sd).pageresources);
    }
    (*sd).pageresources = dict;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_bxgstate(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    obj = parse_pdf_object_extended(
        &raw mut (*args).curptr,
        (*args).endptr,
        ::core::ptr::null_mut::<pdf_file>(),
        Some(
            parse_pdf_reference
                as unsafe extern "C-unwind" fn(
                    *mut *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> *mut pdf_obj,
        ),
        spe as *mut ::core::ffi::c_void,
    );
    if obj.is_null() {
        pdf_warning!(
            b"Could not find an object definition.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    } else if !(!obj.is_null() && pdf_obj_typeof(obj) == PDF_DICT) {
        pdf_warning!(
            b"Parsed object for ExtGState not a dictionary object!\0" as *const u8
                as *const ::core::ffi::c_char
        );
        pdf_release_obj(obj);
        return -(1 as ::core::ffi::c_int);
    }
    pdf_dev_xgstate_push(obj);
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdfm_exgstate(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    pdf_dev_xgstate_pop();
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdft_compat_page(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        pdf_doc_add_page_content(
            b" \0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_uint,
        );
        pdf_doc_add_page_content(
            (*args).curptr,
            (*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long as ::core::ffi::c_int
                as ::core::ffi::c_uint,
        );
    }
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
static mut pdfm_handlers: [spc_handler; 88] = unsafe {
    [
        spc_handler {
            key: b"annotation\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_annot
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"annotate\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_annot
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"annot\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_annot
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"ann\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_annot
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"outline\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_outline
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"out\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_outline
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"article\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_article
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"art\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_article
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bead\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bead
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"thread\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bead
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"destination\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_dest
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"dest\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_dest
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"object\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_object
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"obj\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_object
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"docinfo\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_docinfo
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"docview\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_docview
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"content\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_content
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"put\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_put
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"close\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_close
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bop\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bop
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"eop\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_eop
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"image\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_image
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"img\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_image
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"epdf\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_image
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"link\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_link
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"nolink\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_nolink
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"begincolor\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bcolor\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bc\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"setcolor\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_scolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"scolor\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_scolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"sc\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_scolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"endcolor\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_ecolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"ecolor\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_ecolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"ec\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_ecolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"begingray\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bgray\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bg\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"endgray\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_ecolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"egray\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_ecolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"eg\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_ecolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bgcolor\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bgcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bgc\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bgcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bbc\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bgcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bbg\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bgcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"pagesize\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_pagesize
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bannot\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bann
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"beginann\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bann
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bann\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bann
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"eannot\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_eann
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"endann\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_eann
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"eann\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_eann
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"btrans\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_btrans
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"begintransform\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_btrans
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"begintrans\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_btrans
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bt\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_btrans
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"etrans\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_etrans
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"endtransform\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_etrans
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"endtrans\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_etrans
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"et\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_etrans
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bform\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bform
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"beginxobj\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bform
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bxobj\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bform
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"eform\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_eform
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"endxobj\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_eform
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"exobj\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_eform
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"usexobj\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_uxobj
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"uxobj\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_uxobj
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"tounicode\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_tounicode
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"literal\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_literal
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"stream\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_stream
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"fstream\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_fstream
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"names\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_names
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"mapline\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_mapline
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"mapfile\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_mapfile
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bcontent\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bcontent
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"econtent\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_econtent
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"code\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_code
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"minorversion\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_do_nothing
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"majorversion\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_do_nothing
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"encrypt\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_do_nothing
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"pageresources\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_pageresources
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"trailerid\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_do_nothing
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"xannot\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_xann
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"extendann\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_xann
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"xann\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_xann
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bxgstate\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_bxgstate
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"exgstate\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfm_exgstate
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
    ]
};
static mut pdft_compat_handlers: [spc_handler; 2] = unsafe {
    [
        spc_handler {
            key: b"direct\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdft_compat_page
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"page\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdft_compat_page
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
    ]
};
#[unsafe(export_name = "__pdf_impl_spc_pdfm_check_special")]
pub unsafe extern "C-unwind" fn spc_pdfm_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> bool {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = buf;
    endptr = p.offset(len as isize);
    skip_white(&raw mut p, endptr);
    if p.offset(strlen(b"pdf:\0" as *const u8 as *const ::core::ffi::c_char) as isize) <= endptr
        && memcmp(
            p as *const ::core::ffi::c_void,
            b"pdf:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"pdf:\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[unsafe(export_name = "__pdf_impl_spc_pdfm_setup_handler")]
pub unsafe extern "C-unwind" fn spc_pdfm_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: size_t = 0;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(!sph.is_null() && !spe.is_null() && !ap.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"sph && spe && ap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    if (*ap)
        .curptr
        .offset(strlen(b"pdf:\0" as *const u8 as *const ::core::ffi::c_char) as isize)
        >= (*ap).endptr
        || memcmp(
            (*ap).curptr as *const ::core::ffi::c_void,
            b"pdf:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"pdf:\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0
    {
        pdf_warning!(b"Not pdf: special???\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    (*ap).curptr = (*ap)
        .curptr
        .offset(strlen(b"pdf:\0" as *const u8 as *const ::core::ffi::c_char) as isize);
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    q = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        let mut is_pdft_compat: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*ap).curptr < (*ap).endptr {
            if *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ':' as i32
            {
                is_pdft_compat = 1 as ::core::ffi::c_int;
                (*ap).curptr = (*ap).curptr.offset(1);
            }
        }
        if is_pdft_compat != 0 {
            i = 0 as size_t;
            while i
                < (::core::mem::size_of::<[spc_handler; 2]>() as usize)
                    .wrapping_div(::core::mem::size_of::<spc_handler>() as usize)
            {
                if strcmp(q, pdft_compat_handlers[i as usize].key) == 0 {
                    (*ap).command = pdft_compat_handlers[i as usize].key;
                    (*sph).key = b"pdf:\0" as *const u8 as *const ::core::ffi::c_char;
                    (*sph).exec = pdft_compat_handlers[i as usize].exec;
                    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
                    error = 0 as ::core::ffi::c_int;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
        } else {
            i = 0 as size_t;
            while i
                < (::core::mem::size_of::<[spc_handler; 88]>() as usize)
                    .wrapping_div(::core::mem::size_of::<spc_handler>() as usize)
            {
                if strcmp(q, pdfm_handlers[i as usize].key) == 0 {
                    (*ap).command = pdfm_handlers[i as usize].key;
                    (*sph).key = b"pdf:\0" as *const u8 as *const ::core::ffi::c_char;
                    (*sph).exec = pdfm_handlers[i as usize].exec;
                    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
                    error = 0 as ::core::ffi::c_int;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
        }
        free(q as *mut ::core::ffi::c_void);
    }
    return error;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub unsafe extern "C" fn __pdf_entry_spc_pdfm_at_begin_document(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_pdfm_at_begin_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_pdfm_at_end_document(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_pdfm_at_end_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_pdfm_at_end_page(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_pdfm_at_end_page() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_pdfm_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_pdfm_check_special(buf, len) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_pdfm_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_pdfm_setup_handler(sph, spe, ap) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
