/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_boolean_value"]
    fn pdf_boolean_value(object: *mut pdf_obj) -> ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_number_value"]
    fn pdf_number_value(number: *mut pdf_obj) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_get_array"]
    fn pdf_get_array(array: *mut pdf_obj, idx: ::core::ffi::c_int) -> *mut pdf_obj;
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
    #[link_name = "__pdf_impl_pdf_concat_stream"]
    fn pdf_concat_stream(dst: *mut pdf_obj, src: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_stream_dict"]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_stream_length"]
    fn pdf_stream_length(stream: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_stream_dataptr"]
    fn pdf_stream_dataptr(stream: *mut pdf_obj) -> *const ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_open"]
    fn pdf_open(ident: *const ::core::ffi::c_char, handle: rust_input_handle_t) -> *mut pdf_file;
    #[link_name = "__pdf_impl_pdf_close"]
    fn pdf_close(pf: *mut pdf_file);
    #[link_name = "__pdf_impl_pdf_file_get_catalog"]
    fn pdf_file_get_catalog(pf: *mut pdf_file) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_deref_obj"]
    fn pdf_deref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_import_object"]
    fn pdf_import_object(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_sprint_coord"]
    fn pdf_sprint_coord(buf: *mut ::core::ffi::c_char, p: *const pdf_coord) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_sprint_length"]
    fn pdf_sprint_length(
        buf: *mut ::core::ffi::c_char,
        value: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_doc_get_page"]
    fn pdf_doc_get_page(
        pf: *mut pdf_file,
        page_no: ::core::ffi::c_int,
        opt_bbox: pdf_page_boundary,
        bbox: *mut pdf_rect,
        matrix: *mut pdf_tmatrix,
        resources_p: *mut *mut pdf_obj,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_doc_add_page_content"]
    fn pdf_doc_add_page_content(buffer: *const ::core::ffi::c_char, length: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_ximage_init_form_info"]
    fn pdf_ximage_init_form_info(info: *mut xform_info);
    #[link_name = "__pdf_impl_pdf_ximage_set_form"]
    fn pdf_ximage_set_form(
        ximage: *mut pdf_ximage,
        info: *mut ::core::ffi::c_void,
        resource: *mut pdf_obj,
    );
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_pdf_dev_currentmatrix"]
    fn pdf_dev_currentmatrix(M: *mut pdf_tmatrix) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_transform"]
    fn pdf_dev_transform(p: *mut pdf_coord, M: *const pdf_tmatrix);
    #[link_name = "__pdf_impl_pdf_invertmatrix"]
    fn pdf_invertmatrix(M: *mut pdf_tmatrix);
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_parse_ident"]
    fn parse_ident(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_pdf_name"]
    fn parse_pdf_name(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_number"]
    fn parse_pdf_number(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_string"]
    fn parse_pdf_string(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_dict"]
    fn parse_pdf_dict(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_array"]
    fn parse_pdf_array(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_dpx_stack_init"]
    fn dpx_stack_init(st: *mut dpx_stack);
    #[link_name = "__pdf_impl_dpx_stack_pop"]
    fn dpx_stack_pop(st: *mut dpx_stack) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_dpx_stack_push"]
    fn dpx_stack_push(st: *mut dpx_stack, data: *mut ::core::ffi::c_void);
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uintptr_t = usize;
pub type rust_input_handle_t = uintptr_t;
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
pub type pdf_page_boundary = ::core::ffi::c_uint;
pub const pdf_page_boundary_bleedbox: pdf_page_boundary = 5;
pub const pdf_page_boundary_trimbox: pdf_page_boundary = 4;
pub const pdf_page_boundary_artbox: pdf_page_boundary = 3;
pub const pdf_page_boundary_cropbox: pdf_page_boundary = 1;
pub const pdf_page_boundary_mediabox: pdf_page_boundary = 2;
pub const pdf_page_boundary__auto: pdf_page_boundary = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xform_info {
    pub flags: ::core::ffi::c_int,
    pub bbox: pdf_rect,
    pub matrix: pdf_tmatrix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_options {
    pub page_no: ::core::ffi::c_int,
    pub bbox_type: pdf_page_boundary,
    pub dict: *mut pdf_obj,
}
pub type pdf_ximage = pdf_ximage_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dpx_stack {
    pub size: ::core::ffi::c_int,
    pub top: *mut stack_elem,
    pub bottom: *mut stack_elem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_elem {
    pub data: *mut ::core::ffi::c_void,
    pub prev: *mut stack_elem,
}
pub const action_unknown: action = 0;
pub const action_discard: action = 1;
pub const action_restore: action = 7;
pub const action_save: action = 6;
pub const action_clip: action = 5;
pub const action_trans: action = 4;
pub const action_path: action = 2;
pub const action_rect: action = 3;
pub type action = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct operator {
    pub token: *const ::core::ffi::c_char,
    pub action: action,
    pub n_args: ::core::ffi::c_int,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const PDF_BOOLEAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PDF_NUMBER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_ARRAY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PDF_STREAM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const PDF_NULL: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn get_page_content(
    mut pf: *mut pdf_file,
    mut page: *mut pdf_obj,
) -> *mut pdf_obj {
    let mut contents: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut content_new: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    contents = pdf_deref_obj(pdf_lookup_dict(
        page,
        b"Contents\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    if contents.is_null() {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if !contents.is_null() && pdf_obj_typeof(contents) == PDF_NULL {
        pdf_release_obj(contents);
        contents = pdf_new_stream(0 as ::core::ffi::c_int);
    } else if !contents.is_null() && pdf_obj_typeof(contents) == PDF_ARRAY {
        let mut content_seg: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut i: ::core::ffi::c_int = 0;
        content_new = pdf_new_stream(STREAM_COMPRESS);
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint) < pdf_array_length(contents) {
            content_seg = pdf_deref_obj(pdf_get_array(contents, i));
            if content_seg.is_null() {
                pdf_warning!(
                    b"Could not read page content stream.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                pdf_release_obj(content_new);
                return ::core::ptr::null_mut::<pdf_obj>();
            }
            if !content_seg.is_null() && pdf_obj_typeof(content_seg) == PDF_STREAM {
                pdf_concat_stream(content_new, content_seg);
            } else if !(!content_seg.is_null() && pdf_obj_typeof(content_seg) == PDF_NULL) {
                pdf_warning!(
                    b"Page content not a stream object. Broken PDF file?\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                pdf_release_obj(content_seg);
                pdf_release_obj(content_new);
                pdf_release_obj(contents);
                return ::core::ptr::null_mut::<pdf_obj>();
            }
            pdf_release_obj(content_seg);
            i += 1;
        }
        pdf_release_obj(contents);
        contents = content_new;
    } else {
        if !(!contents.is_null() && pdf_obj_typeof(contents) == PDF_STREAM) {
            pdf_warning!(
                b"Page content not a stream object. Broken PDF file?\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            pdf_release_obj(contents);
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        content_new = pdf_new_stream(STREAM_COMPRESS);
        pdf_concat_stream(content_new, contents);
        pdf_release_obj(contents);
        contents = content_new;
    }
    return contents;
}
#[unsafe(export_name = "__pdf_impl_pdf_include_page")]
pub unsafe extern "C-unwind" fn pdf_include_page(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
    mut ident: *const ::core::ffi::c_char,
    mut options: load_options,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut pf: *mut pdf_file = ::core::ptr::null_mut::<pdf_file>();
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
    let mut contents: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut catalog: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut page: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut resources: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut markinfo: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    pf = pdf_open(ident, handle);
    if pf.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    pdf_ximage_init_form_info(&raw mut info);
    if options.page_no == 0 as ::core::ffi::c_int {
        options.page_no = 1 as ::core::ffi::c_int;
    }
    page = pdf_doc_get_page(
        pf,
        options.page_no,
        options.bbox_type,
        &raw mut info.bbox,
        &raw mut info.matrix,
        &raw mut resources,
    );
    if !page.is_null() {
        catalog = pdf_file_get_catalog(pf);
        markinfo = pdf_deref_obj(pdf_lookup_dict(
            catalog,
            b"MarkInfo\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        if !markinfo.is_null() {
            let mut tmp: *mut pdf_obj = pdf_deref_obj(pdf_lookup_dict(
                markinfo,
                b"Marked\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            pdf_release_obj(markinfo);
            if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_BOOLEAN) {
                pdf_release_obj(tmp);
                pdf_warning!(
                    b"Cannot parse document. Broken PDF file?\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                current_block = 6574256748389622106;
            } else {
                if pdf_boolean_value(tmp) != 0 {
                    pdf_warning!(
                        b"PDF file is tagged... Ignoring tags.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
                pdf_release_obj(tmp);
                current_block = 1054647088692577877;
            }
        } else {
            current_block = 1054647088692577877;
        }
        match current_block {
            6574256748389622106 => {}
            _ => {
                contents = get_page_content(pf, page);
                pdf_release_obj(page);
                page = ::core::ptr::null_mut::<pdf_obj>();
                let mut contents_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                let mut bbox: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                let mut matrix: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                contents_dict = pdf_stream_dict(contents);
                pdf_add_dict(
                    contents_dict,
                    pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_new_name(b"XObject\0" as *const u8 as *const ::core::ffi::c_char),
                );
                pdf_add_dict(
                    contents_dict,
                    pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_new_name(b"Form\0" as *const u8 as *const ::core::ffi::c_char),
                );
                pdf_add_dict(
                    contents_dict,
                    pdf_new_name(b"FormType\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_new_number(1.0f64),
                );
                bbox = pdf_new_array();
                pdf_add_array(bbox, pdf_new_number(info.bbox.llx));
                pdf_add_array(bbox, pdf_new_number(info.bbox.lly));
                pdf_add_array(bbox, pdf_new_number(info.bbox.urx));
                pdf_add_array(bbox, pdf_new_number(info.bbox.ury));
                pdf_add_dict(
                    contents_dict,
                    pdf_new_name(b"BBox\0" as *const u8 as *const ::core::ffi::c_char),
                    bbox,
                );
                matrix = pdf_new_array();
                pdf_add_array(matrix, pdf_new_number(info.matrix.a));
                pdf_add_array(matrix, pdf_new_number(info.matrix.b));
                pdf_add_array(matrix, pdf_new_number(info.matrix.c));
                pdf_add_array(matrix, pdf_new_number(info.matrix.d));
                pdf_add_array(matrix, pdf_new_number(info.matrix.e));
                pdf_add_array(matrix, pdf_new_number(info.matrix.f));
                pdf_add_dict(
                    contents_dict,
                    pdf_new_name(b"Matrix\0" as *const u8 as *const ::core::ffi::c_char),
                    matrix,
                );
                pdf_add_dict(
                    contents_dict,
                    pdf_new_name(b"Resources\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_import_object(resources),
                );
                pdf_release_obj(resources);
                pdf_close(pf);
                pdf_ximage_set_form(ximage, &raw mut info as *mut ::core::ffi::c_void, contents);
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    pdf_release_obj(resources);
    pdf_release_obj(markinfo);
    pdf_release_obj(page);
    pdf_release_obj(contents);
    pdf_close(pf);
    return -(1 as ::core::ffi::c_int);
}
static mut operators: [operator; 22] = [
    operator {
        token: b"b*\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_clip,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"B*\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_clip,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"cm\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_trans,
        n_args: 6 as ::core::ffi::c_int,
    },
    operator {
        token: b"f*\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_clip,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"re\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_rect,
        n_args: 4 as ::core::ffi::c_int,
    },
    operator {
        token: b"W*\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_path,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"b\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_clip,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"B\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_clip,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"c\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_path,
        n_args: 6 as ::core::ffi::c_int,
    },
    operator {
        token: b"f\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_clip,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"F\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_clip,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"h\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_path,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"l\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_path,
        n_args: 2 as ::core::ffi::c_int,
    },
    operator {
        token: b"m\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_path,
        n_args: 2 as ::core::ffi::c_int,
    },
    operator {
        token: b"n\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_path,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"q\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_save,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"Q\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_restore,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"s\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_clip,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"S\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_clip,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"v\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_path,
        n_args: 4 as ::core::ffi::c_int,
    },
    operator {
        token: b"W\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_path,
        n_args: 0 as ::core::ffi::c_int,
    },
    operator {
        token: b"y\0" as *const u8 as *const ::core::ffi::c_char,
        action: action_path,
        n_args: 4 as ::core::ffi::c_int,
    },
];
unsafe extern "C-unwind" fn get_numbers_from_stack(
    mut stack: *mut dpx_stack,
    mut v: *mut ::core::ffi::c_double,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        let mut obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        obj = dpx_stack_pop(stack) as *mut pdf_obj;
        if obj.is_null() {
            error = -(1 as ::core::ffi::c_int);
            break;
        } else if !(!obj.is_null() && pdf_obj_typeof(obj) == PDF_NUMBER) {
            pdf_release_obj(obj);
            error = -(1 as ::core::ffi::c_int);
            break;
        } else {
            *v.offset((n - i - 1 as ::core::ffi::c_int) as isize) = pdf_number_value(obj);
            pdf_release_obj(obj);
            i += 1;
        }
    }
    return error;
}
#[unsafe(export_name = "__pdf_impl_pdf_copy_clip")]
pub unsafe extern "C-unwind" fn pdf_copy_clip(
    mut image_file: rust_input_handle_t,
    mut pageNo: ::core::ffi::c_int,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut page_tree: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut contents: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut depth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut bbox: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut mtrx: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut pf: *mut pdf_file = ::core::ptr::null_mut::<pdf_file>();
    let mut stack: dpx_stack = dpx_stack {
        size: 0,
        top: ::core::ptr::null_mut::<stack_elem>(),
        bottom: ::core::ptr::null_mut::<stack_elem>(),
    };
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    pf = pdf_open(::core::ptr::null::<::core::ffi::c_char>(), image_file);
    if pf.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    pdf_dev_currentmatrix(&raw mut M);
    pdf_invertmatrix(&raw mut M);
    M.e += x_user;
    M.f += y_user;
    page_tree = pdf_doc_get_page(
        pf,
        pageNo,
        pdf_page_boundary__auto,
        &raw mut bbox,
        &raw mut mtrx,
        ::core::ptr::null_mut::<*mut pdf_obj>(),
    );
    if page_tree.is_null() {
        pdf_close(pf);
        return -(1 as ::core::ffi::c_int);
    }
    contents = get_page_content(pf, page_tree);
    pdf_release_obj(page_tree);
    if contents.is_null() {
        pdf_close(pf);
        return -(1 as ::core::ffi::c_int);
    }
    pdf_doc_add_page_content(
        b" \0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_uint,
    );
    p = pdf_stream_dataptr(contents) as *const ::core::ffi::c_char;
    endptr = p.offset(pdf_stream_length(contents) as isize);
    depth = 0 as ::core::ffi::c_int;
    dpx_stack_init(&raw mut stack);
    skip_white(&raw mut p, endptr);
    while p < endptr && error == 0 {
        let mut action: action = action_discard;
        let mut token: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut n_args: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut len: size_t = 0 as size_t;
        if depth > 1 as ::core::ffi::c_int {
            if *p as ::core::ffi::c_int == 'q' as i32 {
                depth += 1;
            }
            if *p as ::core::ffi::c_int == 'Q' as i32 {
                depth -= 1;
            }
            token = parse_ident(&raw mut p, endptr);
            skip_white(&raw mut p, endptr);
            free(token as *mut ::core::ffi::c_void);
        } else {
            match *p as ::core::ffi::c_int {
                45 | 43 | 46 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    obj = parse_pdf_number(&raw mut p, endptr);
                }
                91 => {
                    obj = parse_pdf_array(&raw mut p, endptr, ::core::ptr::null_mut::<pdf_file>());
                }
                47 => {
                    obj = parse_pdf_name(&raw mut p, endptr);
                }
                40 => {
                    obj = parse_pdf_string(&raw mut p, endptr);
                }
                60 => {
                    if p < endptr.offset(-(1 as ::core::ffi::c_int as isize))
                        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == '<' as i32
                    {
                        obj =
                            parse_pdf_dict(&raw mut p, endptr, ::core::ptr::null_mut::<pdf_file>());
                    } else {
                        obj = parse_pdf_string(&raw mut p, endptr);
                    }
                }
                _ => {}
            }
            if !obj.is_null() {
                skip_white(&raw mut p, endptr);
                dpx_stack_push(&raw mut stack, obj as *mut ::core::ffi::c_void);
            } else {
                token = parse_ident(&raw mut p, endptr);
                skip_white(&raw mut p, endptr);
                if token.is_null() {
                    break;
                }
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while (i as usize)
                    < (::core::mem::size_of::<[operator; 22]>() as usize)
                        .wrapping_div(::core::mem::size_of::<operator>() as usize)
                {
                    if strcmp(token, operators[i as usize].token) == 0 {
                        action = operators[i as usize].action;
                        n_args = operators[i as usize].n_args;
                        break;
                    } else {
                        i += 1;
                    }
                }
                match action as ::core::ffi::c_uint {
                    3 => {
                        let mut v: [::core::ffi::c_double; 4] = [0.; 4];
                        error = get_numbers_from_stack(
                            &raw mut stack,
                            &raw mut v as *mut ::core::ffi::c_double,
                            n_args,
                        );
                        if error == 0 {
                            if M.b == 0.0f64 && M.c == 0.0f64 {
                                let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
                                let mut w: ::core::ffi::c_double = 0.;
                                let mut h: ::core::ffi::c_double = 0.;
                                p0.x = v[0 as ::core::ffi::c_int as usize];
                                p0.y = v[1 as ::core::ffi::c_int as usize];
                                w = M.a * v[2 as ::core::ffi::c_int as usize];
                                h = M.d * v[3 as ::core::ffi::c_int as usize];
                                pdf_dev_transform(&raw mut p0, &raw mut M);
                                let fresh0 = len;
                                len = len.wrapping_add(1);
                                buf[fresh0 as usize] = ' ' as i32 as ::core::ffi::c_char;
                                len = len.wrapping_add(pdf_sprint_coord(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    &raw mut p0,
                                ) as size_t);
                                let fresh1 = len;
                                len = len.wrapping_add(1);
                                buf[fresh1 as usize] = ' ' as i32 as ::core::ffi::c_char;
                                len = len.wrapping_add(pdf_sprint_length(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    w,
                                ) as size_t);
                                let fresh2 = len;
                                len = len.wrapping_add(1);
                                buf[fresh2 as usize] = ' ' as i32 as ::core::ffi::c_char;
                                len = len.wrapping_add(pdf_sprint_length(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    h,
                                ) as size_t);
                                len = len.wrapping_add(sprintf(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    b" re\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as size_t);
                            } else {
                                let mut p0_0: pdf_coord = pdf_coord { x: 0., y: 0. };
                                let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
                                let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
                                let mut p3: pdf_coord = pdf_coord { x: 0., y: 0. };
                                let mut w_0: ::core::ffi::c_double = 0.;
                                let mut h_0: ::core::ffi::c_double = 0.;
                                w_0 = v[2 as ::core::ffi::c_int as usize];
                                h_0 = v[3 as ::core::ffi::c_int as usize];
                                p0_0.x = v[0 as ::core::ffi::c_int as usize];
                                p0_0.y = v[1 as ::core::ffi::c_int as usize];
                                p1.x = p0_0.x + w_0;
                                p1.y = p0_0.y;
                                p2.x = p1.x;
                                p2.y = p1.y + h_0;
                                p3.x = p0_0.x;
                                p3.y = p2.y;
                                pdf_dev_transform(&raw mut p0_0, &raw mut M);
                                pdf_dev_transform(&raw mut p1, &raw mut M);
                                pdf_dev_transform(&raw mut p2, &raw mut M);
                                pdf_dev_transform(&raw mut p3, &raw mut M);
                                let fresh3 = len;
                                len = len.wrapping_add(1);
                                buf[fresh3 as usize] = ' ' as i32 as ::core::ffi::c_char;
                                len = len.wrapping_add(pdf_sprint_coord(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    &raw mut p0_0,
                                ) as size_t);
                                len = len.wrapping_add(sprintf(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    b" m\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as size_t);
                                let fresh4 = len;
                                len = len.wrapping_add(1);
                                buf[fresh4 as usize] = ' ' as i32 as ::core::ffi::c_char;
                                len = len.wrapping_add(pdf_sprint_coord(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    &raw mut p1,
                                ) as size_t);
                                len = len.wrapping_add(sprintf(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    b" l\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as size_t);
                                let fresh5 = len;
                                len = len.wrapping_add(1);
                                buf[fresh5 as usize] = ' ' as i32 as ::core::ffi::c_char;
                                len = len.wrapping_add(pdf_sprint_coord(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    &raw mut p2,
                                ) as size_t);
                                len = len.wrapping_add(sprintf(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    b" l\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as size_t);
                                let fresh6 = len;
                                len = len.wrapping_add(1);
                                buf[fresh6 as usize] = ' ' as i32 as ::core::ffi::c_char;
                                len = len.wrapping_add(pdf_sprint_coord(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    &raw mut p3,
                                ) as size_t);
                                len = len.wrapping_add(sprintf(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    b" l h\0" as *const u8 as *const ::core::ffi::c_char,
                                ) as size_t);
                            }
                            pdf_doc_add_page_content(
                                &raw mut buf as *mut ::core::ffi::c_char,
                                len as ::core::ffi::c_uint,
                            );
                        }
                    }
                    2 => {
                        let mut v_0: [::core::ffi::c_double; 6] = [0.; 6];
                        let mut pt: pdf_coord = pdf_coord { x: 0., y: 0. };
                        let mut i_0: ::core::ffi::c_int = 0;
                        error = get_numbers_from_stack(
                            &raw mut stack,
                            &raw mut v_0 as *mut ::core::ffi::c_double,
                            n_args,
                        );
                        if error == 0 {
                            i_0 = 0 as ::core::ffi::c_int;
                            while i_0 < n_args / 2 as ::core::ffi::c_int {
                                pt.x = v_0[(2 as ::core::ffi::c_int * i_0) as usize];
                                pt.y = v_0[(2 as ::core::ffi::c_int * i_0 + 1 as ::core::ffi::c_int)
                                    as usize];
                                pdf_dev_transform(&raw mut pt, &raw mut M);
                                let fresh7 = len;
                                len = len.wrapping_add(1);
                                buf[fresh7 as usize] = ' ' as i32 as ::core::ffi::c_char;
                                len = len.wrapping_add(pdf_sprint_coord(
                                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                    &raw mut pt,
                                ) as size_t);
                                i_0 += 1;
                            }
                            len = len.wrapping_add(sprintf(
                                (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                b" %s\0" as *const u8 as *const ::core::ffi::c_char,
                                token,
                            ) as size_t);
                            pdf_doc_add_page_content(
                                &raw mut buf as *mut ::core::ffi::c_char,
                                len as ::core::ffi::c_uint,
                            );
                        }
                    }
                    4 => {
                        let mut v_1: [::core::ffi::c_double; 6] = [0.; 6];
                        let mut T: pdf_tmatrix = pdf_tmatrix {
                            a: 0.,
                            b: 0.,
                            c: 0.,
                            d: 0.,
                            e: 0.,
                            f: 0.,
                        };
                        error = get_numbers_from_stack(
                            &raw mut stack,
                            &raw mut v_1 as *mut ::core::ffi::c_double,
                            n_args,
                        );
                        if error == 0 {
                            T.a = v_1[0 as ::core::ffi::c_int as usize];
                            T.b = v_1[1 as ::core::ffi::c_int as usize];
                            T.c = v_1[2 as ::core::ffi::c_int as usize];
                            T.d = v_1[3 as ::core::ffi::c_int as usize];
                            T.e = v_1[4 as ::core::ffi::c_int as usize];
                            T.f = v_1[5 as ::core::ffi::c_int as usize];
                            let mut _tmp_a: ::core::ffi::c_double = 0.;
                            let mut _tmp_b: ::core::ffi::c_double = 0.;
                            let mut _tmp_c: ::core::ffi::c_double = 0.;
                            let mut _tmp_d: ::core::ffi::c_double = 0.;
                            _tmp_a = M.a;
                            _tmp_b = M.b;
                            _tmp_c = M.c;
                            _tmp_d = M.d;
                            M.a = T.a * _tmp_a + T.b * _tmp_c;
                            M.b = T.a * _tmp_b + T.b * _tmp_d;
                            M.c = T.c * _tmp_a + T.d * _tmp_c;
                            M.d = T.c * _tmp_b + T.d * _tmp_d;
                            M.e += T.e * _tmp_a + T.f * _tmp_c;
                            M.f += T.e * _tmp_b + T.f * _tmp_d;
                        }
                    }
                    5 => {
                        if *token.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            >= 'a' as i32
                            && *token.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                <= 'z' as i32
                        {
                            len = len.wrapping_add(sprintf(
                                (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                b" h\0" as *const u8 as *const ::core::ffi::c_char,
                            ) as size_t);
                        }
                        if strlen(token) >= 2 as size_t
                            && *token.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                == '*' as i32
                        {
                            len = len.wrapping_add(sprintf(
                                (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                b" W* n\0" as *const u8 as *const ::core::ffi::c_char,
                            ) as size_t);
                        } else {
                            len = len.wrapping_add(sprintf(
                                (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                                b" W n\0" as *const u8 as *const ::core::ffi::c_char,
                            ) as size_t);
                        }
                        pdf_doc_add_page_content(
                            &raw mut buf as *mut ::core::ffi::c_char,
                            len as ::core::ffi::c_uint,
                        );
                    }
                    6 => {
                        depth += 1;
                    }
                    7 => {
                        depth -= 1;
                    }
                    1 => loop {
                        obj = dpx_stack_pop(&raw mut stack) as *mut pdf_obj;
                        if obj.is_null() {
                            break;
                        }
                        pdf_release_obj(obj);
                    },
                    0 => {
                        error = -(1 as ::core::ffi::c_int);
                    }
                    _ => {}
                }
                free(token as *mut ::core::ffi::c_void);
            }
        }
    }
    let mut obj_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    loop {
        obj_0 = dpx_stack_pop(&raw mut stack) as *mut pdf_obj;
        if obj_0.is_null() {
            break;
        }
        pdf_release_obj(obj_0);
    }
    pdf_release_obj(contents);
    pdf_close(pf);
    return error;
}
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
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
#[repr(C)]
pub struct pdf_ximage_ {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_include_page(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
    mut ident: *const ::core::ffi::c_char,
    mut options: load_options,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_include_page(ximage, handle, ident, options)
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
pub unsafe extern "C" fn __pdf_entry_pdf_copy_clip(
    mut image_file: rust_input_handle_t,
    mut pageNo: ::core::ffi::c_int,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_copy_clip(image_file, pageNo, x_user, y_user)
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
