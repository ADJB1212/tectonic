/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn ttstub_input_open(
        path: *const ::core::ffi::c_char,
        format: ttbc_file_format,
        is_gz: ::core::ffi::c_int,
    ) -> rust_input_handle_t;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_string_value"]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_string_length"]
    fn pdf_string_length(object: *mut pdf_obj) -> ::core::ffi::c_uint;
    #[link_name = "__pdf_impl_pdf_merge_dict"]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    #[link_name = "__pdf_impl_transform_info_clear"]
    fn transform_info_clear(info: *mut transform_info);
    fn spc_warn(spe: *mut spc_env, fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_spc_lookup_reference"]
    fn spc_lookup_reference(ident: *const ::core::ffi::c_char) -> *mut pdf_obj;
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
    #[link_name = "__pdf_impl_dpx_util_read_length"]
    fn dpx_util_read_length(
        vp: *mut ::core::ffi::c_double,
        mag: ::core::ffi::c_double,
        curptr: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_dpx_stack_init"]
    fn dpx_stack_init(st: *mut dpx_stack);
    #[link_name = "__pdf_impl_dpx_stack_pop"]
    fn dpx_stack_pop(st: *mut dpx_stack) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_dpx_stack_push"]
    fn dpx_stack_push(st: *mut dpx_stack, data: *mut ::core::ffi::c_void);
    #[link_name = "__pdf_impl_dpx_stack_depth"]
    fn dpx_stack_depth(st: *mut dpx_stack) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_dpx_stack_top"]
    fn dpx_stack_top(st: *mut dpx_stack) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_parse_c_ident"]
    fn parse_c_ident(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_tt_mfgets"]
    fn tt_mfgets(
        buffer: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_int,
        file: rust_input_handle_t,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_doc_add_page_content"]
    fn pdf_doc_add_page_content(buffer: *const ::core::ffi::c_char, length: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_ximage_load_image"]
    fn pdf_ximage_load_image(
        ident: *const ::core::ffi::c_char,
        filename: *const ::core::ffi::c_char,
        options: load_options,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_mps_scan_bbox"]
    fn mps_scan_bbox(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        bbox: *mut pdf_rect,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_concat"]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_parse_number"]
    fn parse_number(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_ident"]
    fn parse_ident(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_opt_ident"]
    fn parse_opt_ident(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_pdf_string"]
    fn parse_pdf_string(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
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
    #[link_name = "__pdf_impl_pdf_font_findresource"]
    fn pdf_font_findresource(
        tex_name: *const ::core::ffi::c_char,
        font_scale: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_font_resource"]
    fn pdf_get_font_resource(font_id: ::core::ffi::c_int) -> *mut pdf_obj;
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
pub struct fontattr {
    pub ident: *mut ::core::ffi::c_char,
    pub size: ::core::ffi::c_double,
    pub attr: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack {
    pub page: ::core::ffi::c_int,
    pub direct: ::core::ffi::c_int,
    pub stack: dpx_stack,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_stack {
    pub stacks: [stack; 256],
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
pub const PDF_DICT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const INFO_HAS_USER_BBOX: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const INFO_HAS_WIDTH: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const INFO_HAS_HEIGHT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const PDFCOLORSTACK_MAX_STACK: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
static mut spc_stack: spc_stack = spc_stack {
    stacks: [stack {
        page: 0,
        direct: 0,
        stack: dpx_stack {
            size: 0,
            top: ::core::ptr::null::<stack_elem>() as *mut stack_elem,
            bottom: ::core::ptr::null::<stack_elem>() as *mut stack_elem,
        },
    }; 256],
};
unsafe extern "C-unwind" fn pdfcolorstack__get_id(
    mut spe: *mut spc_env,
    mut id: *mut ::core::ffi::c_int,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*args).curptr >= (*args).endptr {
        pdf_warning!(
            b"Stack ID number expected but not found.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    q = parse_number(&raw mut (*args).curptr, (*args).endptr);
    if q.is_null() {
        pdf_warning!(
            b"Stack ID number expected but not found.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    *id = atoi(q);
    free(q as *mut ::core::ffi::c_void);
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdfcolorstack__init(
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_stack = dp as *mut spc_stack;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < PDFCOLORSTACK_MAX_STACK {
        (*sd).stacks[i as usize].page = 0 as ::core::ffi::c_int;
        (*sd).stacks[i as usize].direct = 0 as ::core::ffi::c_int;
        dpx_stack_init(&raw mut (*(&raw mut (*sd).stacks as *mut stack).offset(i as isize)).stack);
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdfcolorstack__clean(
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut sd: *mut spc_stack = dp as *mut spc_stack;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < PDFCOLORSTACK_MAX_STACK {
        let mut litstr: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut stk: *mut dpx_stack =
            &raw mut (*(&raw mut (*sd).stacks as *mut stack).offset(i as isize)).stack;
        loop {
            litstr = dpx_stack_pop(stk) as *mut pdf_obj;
            if litstr.is_null() {
                break;
            }
            pdf_release_obj(litstr);
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdfcolorstack__set_litstr(
    mut cp: pdf_coord,
    mut litstr: *mut pdf_obj,
    mut direct: ::core::ffi::c_int,
) {
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    if litstr.is_null() {
        return;
    }
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
        pdf_string_value(litstr) as *const ::core::ffi::c_char,
        pdf_string_length(litstr),
    );
    if direct == 0 {
        M.e = -cp.x;
        M.f = -cp.y;
        pdf_dev_concat(&raw mut M);
    }
}
unsafe extern "C-unwind" fn spc_handler_pdfcolorstackinit(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut st: *mut stack = ::core::ptr::null_mut::<stack>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cp: pdf_coord = pdf_coord {
        x: 0.0f64,
        y: 0.0f64,
    };
    let mut litstr: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        return -(1 as ::core::ffi::c_int);
    }
    if pdfcolorstack__get_id(spe, &raw mut id, args) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if id < 0 as ::core::ffi::c_int || id >= PDFCOLORSTACK_MAX_STACK {
        pdf_warning!(
            b"Invalid stack number specified: %d\0" as *const u8 as *const ::core::ffi::c_char,
            id
        );
        return -(1 as ::core::ffi::c_int);
    }
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    st = (&raw mut spc_stack.stacks as *mut stack).offset(id as isize) as *mut stack;
    if dpx_stack_depth(&raw mut (*st).stack) > 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Stadk ID=%d already initialized?\0" as *const u8 as *const ::core::ffi::c_char,
            id
        );
        return -(1 as ::core::ffi::c_int);
    }
    loop {
        q = parse_c_ident(&raw mut (*args).curptr, (*args).endptr);
        if q.is_null() {
            break;
        }
        if strcmp(q, b"page\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            (*st).page = 1 as ::core::ffi::c_int;
        } else if strcmp(q, b"direct\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            (*st).direct = 1 as ::core::ffi::c_int;
        } else {
            pdf_warning!(
                b"Ignoring unknown option for pdfcolorstack special (init): %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                q
            );
        }
        free(q as *mut ::core::ffi::c_void);
        skip_white(&raw mut (*args).curptr, (*args).endptr);
    }
    if (*args).curptr < (*args).endptr {
        litstr = parse_pdf_string(&raw mut (*args).curptr, (*args).endptr);
        if !litstr.is_null() {
            dpx_stack_push(&raw mut (*st).stack, litstr as *mut ::core::ffi::c_void);
            pdfcolorstack__set_litstr(cp, litstr, (*st).direct);
        }
        skip_white(&raw mut (*args).curptr, (*args).endptr);
    } else {
        pdf_warning!(
            b"No valid PDF literal specified.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdfcolorstack__set(
    mut spe: *mut spc_env,
    mut st: *mut stack,
    mut cp: pdf_coord,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut litstr: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        return -(1 as ::core::ffi::c_int);
    }
    litstr = dpx_stack_pop(&raw mut (*st).stack) as *mut pdf_obj;
    if litstr.is_null() {
        pdf_warning!(b"Stack empty!\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    pdf_release_obj(litstr);
    litstr = parse_pdf_string(&raw mut (*args).curptr, (*args).endptr);
    if !litstr.is_null() {
        dpx_stack_push(&raw mut (*st).stack, litstr as *mut ::core::ffi::c_void);
        pdfcolorstack__set_litstr(cp, litstr, (*st).direct);
        skip_white(&raw mut (*args).curptr, (*args).endptr);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdfcolorstack__push(
    mut spe: *mut spc_env,
    mut st: *mut stack,
    mut cp: pdf_coord,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut litstr: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        return -(1 as ::core::ffi::c_int);
    }
    litstr = parse_pdf_string(&raw mut (*args).curptr, (*args).endptr);
    if !litstr.is_null() {
        dpx_stack_push(&raw mut (*st).stack, litstr as *mut ::core::ffi::c_void);
        pdfcolorstack__set_litstr(cp, litstr, (*st).direct);
        skip_white(&raw mut (*args).curptr, (*args).endptr);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdfcolorstack__current(
    mut spe: *mut spc_env,
    mut st: *mut stack,
    mut cp: pdf_coord,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut litstr: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    litstr = dpx_stack_top(&raw mut (*st).stack) as *mut pdf_obj;
    if !litstr.is_null() {
        pdfcolorstack__set_litstr(cp, litstr, (*st).direct);
        skip_white(&raw mut (*args).curptr, (*args).endptr);
    } else {
        pdf_warning!(b"Stack empty!\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdfcolorstack__pop(
    mut spe: *mut spc_env,
    mut st: *mut stack,
    mut cp: pdf_coord,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut litstr: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if dpx_stack_depth(&raw mut (*st).stack) < 2 as ::core::ffi::c_int {
        pdf_warning!(b"Stack underflow\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    litstr = dpx_stack_pop(&raw mut (*st).stack) as *mut pdf_obj;
    if !litstr.is_null() {
        pdf_release_obj(litstr);
    }
    litstr = dpx_stack_top(&raw mut (*st).stack) as *mut pdf_obj;
    if !litstr.is_null() {
        pdfcolorstack__set_litstr(cp, litstr, (*st).direct);
    }
    return error;
}
unsafe extern "C-unwind" fn spc_handler_pdfcolorstack(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut id: ::core::ffi::c_int = 0;
    let mut command: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut st: *mut stack = ::core::ptr::null_mut::<stack>();
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        return -(1 as ::core::ffi::c_int);
    }
    if pdfcolorstack__get_id(spe, &raw mut id, args) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if id < 0 as ::core::ffi::c_int || id >= PDFCOLORSTACK_MAX_STACK {
        pdf_warning!(
            b"Invalid stack ID specified: %d\0" as *const u8 as *const ::core::ffi::c_char,
            id
        );
        return -(1 as ::core::ffi::c_int);
    }
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    st = (&raw mut spc_stack.stacks as *mut stack).offset(id as isize) as *mut stack;
    if dpx_stack_depth(&raw mut (*st).stack) < 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"Stack ID=%d not properly initialized?\0" as *const u8 as *const ::core::ffi::c_char,
            id
        );
        return -(1 as ::core::ffi::c_int);
    }
    command = parse_c_ident(&raw mut (*args).curptr, (*args).endptr);
    if command.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    spc_get_current_point(spe, &raw mut cp);
    if strcmp(command, b"set\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        error = pdfcolorstack__set(spe, st, cp, args);
    } else if strcmp(
        command,
        b"push\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        error = pdfcolorstack__push(spe, st, cp, args);
    } else if strcmp(command, b"pop\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        error = pdfcolorstack__pop(spe, st, cp, args);
    } else if strcmp(
        command,
        b"current\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        error = pdfcolorstack__current(spe, st, cp, args);
    } else {
        pdf_warning!(
            b"Unknown action: %s\0" as *const u8 as *const ::core::ffi::c_char,
            command
        );
    }
    if error != 0 {
        pdf_warning!(
            b"Error occurred while processing pdfcolorstack: id=%d command=\"%s\"\0" as *const u8
                as *const ::core::ffi::c_char,
            id,
            command
        );
    }
    free(command as *mut ::core::ffi::c_void);
    return error;
}
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
#[unsafe(no_mangle)]
pub static mut fontattrs: *mut fontattr = ::core::ptr::null::<fontattr>() as *mut fontattr;
#[unsafe(no_mangle)]
pub static mut num_fontattrs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut max_fontattrs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn process_fontattr(
    mut ident: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_double,
    mut attr: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut font_id: ::core::ffi::c_int = 0;
    let mut fontdict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if !(!ident.is_null() && !attr.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ident && attr\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    font_id = pdf_font_findresource(ident, size);
    if font_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Could not find specified font resource: %s (%gpt)\0" as *const u8
                as *const ::core::ffi::c_char,
            ident,
            size,
        );
        return -(1 as ::core::ffi::c_int);
    }
    fontdict = pdf_get_font_resource(font_id);
    if fontdict.is_null() {
        pdf_warning!(
            b"Specified object not exist: %s (%gpt)\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
            size,
        );
        return -(1 as ::core::ffi::c_int);
    }
    pdf_merge_dict(fontdict, attr);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_pdffontattr(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut fontattr: *mut fontattr = ::core::ptr::null_mut::<fontattr>();
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut size: ::core::ffi::c_double = 0.0f64;
    let mut attr: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr >= (*ap).endptr {
        return -(1 as ::core::ffi::c_int);
    }
    ident = parse_ident(&raw mut (*ap).curptr, (*ap).endptr);
    if ident.is_null() {
        pdf_warning!(b"Missing a font name.\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr < (*ap).endptr
        && *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '<' as i32
    {
        let mut error: ::core::ffi::c_int =
            dpx_util_read_length(&raw mut size, 1.0f64, &raw mut (*ap).curptr, (*ap).endptr);
        if error != 0 {
            pdf_warning!(
                b"Font size expected but not found.\0" as *const u8 as *const ::core::ffi::c_char
            );
            free(ident as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    }
    attr = parse_pdf_object_extended(
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
    if attr.is_null() {
        pdf_warning!(
            b"Failed to parse a PDF dictionary object: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            ident
        );
        free(ident as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    } else if !(!attr.is_null() && pdf_obj_typeof(attr) == PDF_DICT) {
        pdf_warning!(
            b"PDF dict expected but non-dict object found: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            ident
        );
        free(ident as *mut ::core::ffi::c_void);
        pdf_release_obj(attr);
        return -(1 as ::core::ffi::c_int);
    }
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    if num_fontattrs >= max_fontattrs {
        fontattrs = renew(
            fontattrs as *mut ::core::ffi::c_void,
            ((max_fontattrs + 256 as ::core::ffi::c_int) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<fontattr>() as usize) as uint32_t,
        ) as *mut fontattr;
        max_fontattrs += 256 as ::core::ffi::c_int;
    }
    fontattr = fontattrs.offset(num_fontattrs as isize) as *mut fontattr;
    num_fontattrs += 1 as ::core::ffi::c_int;
    (*fontattr).ident = ident;
    (*fontattr).size = size;
    (*fontattr).attr = attr;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_misc_at_begin_document")]
pub unsafe extern "C-unwind" fn spc_misc_at_begin_document() -> ::core::ffi::c_int {
    let mut sd: *mut spc_stack = &raw mut spc_stack;
    if fontattrs.is_null() {
        fontattrs = new((256 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<fontattr>() as usize)
            as uint32_t) as *mut fontattr;
        num_fontattrs = 0 as ::core::ffi::c_int;
        max_fontattrs = 256 as ::core::ffi::c_int;
    }
    return pdfcolorstack__init(sd as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_spc_misc_at_end_document")]
pub unsafe extern "C-unwind" fn spc_misc_at_end_document() -> ::core::ffi::c_int {
    let mut sd: *mut spc_stack = &raw mut spc_stack;
    if !fontattrs.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < num_fontattrs {
            process_fontattr(
                (*fontattrs.offset(i as isize)).ident,
                (*fontattrs.offset(i as isize)).size,
                (*fontattrs.offset(i as isize)).attr,
            );
            free((*fontattrs.offset(i as isize)).ident as *mut ::core::ffi::c_void);
            pdf_release_obj((*fontattrs.offset(i as isize)).attr);
            i += 1;
        }
        free(fontattrs as *mut ::core::ffi::c_void);
        fontattrs = ::core::ptr::null_mut::<fontattr>();
        num_fontattrs = 0 as ::core::ffi::c_int;
        max_fontattrs = num_fontattrs;
    }
    return pdfcolorstack__clean(sd as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_spc_misc_at_begin_page")]
pub unsafe extern "C-unwind" fn spc_misc_at_begin_page() -> ::core::ffi::c_int {
    let mut sd: *mut spc_stack = &raw mut spc_stack;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < PDFCOLORSTACK_MAX_STACK {
        let mut stk: *mut dpx_stack =
            &raw mut (*(&raw mut (*sd).stacks as *mut stack).offset(i as isize)).stack;
        if (*sd).stacks[i as usize].page != 0 {
            let mut litstr: *mut pdf_obj = dpx_stack_top(stk) as *mut pdf_obj;
            let mut cp: pdf_coord = pdf_coord {
                x: 0.0f64,
                y: 0.0f64,
            };
            if !litstr.is_null() {
                pdfcolorstack__set_litstr(cp, litstr, (*sd).stacks[i as usize].direct);
            }
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_misc_at_begin_form")]
pub unsafe extern "C-unwind" fn spc_misc_at_begin_form() -> ::core::ffi::c_int {
    return spc_misc_at_begin_page();
}
#[unsafe(export_name = "__pdf_impl_spc_misc_at_end_form")]
pub unsafe extern "C-unwind" fn spc_misc_at_end_form() -> ::core::ffi::c_int {
    return spc_misc_at_begin_page();
}
unsafe extern "C-unwind" fn spc_handler_postscriptbox(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut form_id: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
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
    let mut filename: [::core::ffi::c_char; 256] = [0; 256];
    let mut buf: [::core::ffi::c_char; 512] = [0; 512];
    let mut handle: rust_input_handle_t = 0;
    if !(!spe.is_null() && !ap.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*ap).curptr >= (*ap).endptr {
        pdf_warning!(
            b"No width/height/filename given for postscriptbox special.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    len = (*ap).endptr.offset_from((*ap).curptr) as ::core::ffi::c_long as ::core::ffi::c_int;
    len = if (511 as ::core::ffi::c_int) < len {
        511 as ::core::ffi::c_int
    } else {
        len
    };
    memcpy(
        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (*ap).curptr as *const ::core::ffi::c_void,
        len as size_t,
    );
    buf[len as usize] = '\0' as i32 as ::core::ffi::c_char;
    transform_info_clear(&raw mut ti);
    pdf_warning!(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut buf as *mut ::core::ffi::c_char
    );
    if sscanf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"{%lfpt}{%lfpt}{%255[^}]}\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut ti.width,
        &raw mut ti.height,
        &raw mut filename as *mut ::core::ffi::c_char,
    ) != 3 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Syntax error in postscriptbox special?\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*ap).curptr = (*ap).endptr;
    ti.width *= 72.0f64 / 72.27f64;
    ti.height *= 72.0f64 / 72.27f64;
    handle = ttstub_input_open(
        &raw mut filename as *mut ::core::ffi::c_char,
        TTBC_FILE_FORMAT_PICT,
        0 as ::core::ffi::c_int,
    );
    if handle == INVALID_HANDLE {
        pdf_warning!(
            b"Could not open image file: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut filename as *mut ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    ti.flags |= INFO_HAS_WIDTH | INFO_HAS_HEIGHT;
    loop {
        let mut p: *const ::core::ffi::c_char = tt_mfgets(
            &raw mut buf as *mut ::core::ffi::c_char,
            512 as ::core::ffi::c_int,
            handle,
        );
        if p.is_null() {
            break;
        }
        if !(mps_scan_bbox(&raw mut p, p.offset(strlen(p) as isize), &raw mut ti.bbox)
            >= 0 as ::core::ffi::c_int)
        {
            continue;
        }
        ti.flags |= INFO_HAS_USER_BBOX;
        break;
    }
    crate::backend::runtime::input_close(handle);
    form_id = pdf_ximage_load_image(
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut filename as *mut ::core::ffi::c_char,
        options,
    );
    if form_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Failed to load image file: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut filename as *mut ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    spc_put_image(spe, form_id, &raw mut ti, (*spe).x_user, (*spe).y_user);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_null(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
static mut misc_handlers: [spc_handler; 9] = unsafe {
    [
        spc_handler {
            key: b"postscriptbox\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_postscriptbox
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"pdfcolorstackinit\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfcolorstackinit
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"pdfcolorstack\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdfcolorstack
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"pdffontattr\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_pdffontattr
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"landscape\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_null
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"papersize\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_null
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"src:\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_null
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"pos:\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_null
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"om:\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_null
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
    ]
};
#[unsafe(export_name = "__pdf_impl_spc_misc_check_special")]
pub unsafe extern "C-unwind" fn spc_misc_check_special(
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
) -> bool {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: size_t = 0;
    p = buffer;
    endptr = p.offset(size as isize);
    skip_white(&raw mut p, endptr);
    size = endptr.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int;
    i = 0 as size_t;
    while i
        < (::core::mem::size_of::<[spc_handler; 9]>() as usize)
            .wrapping_div(::core::mem::size_of::<spc_handler>() as usize)
    {
        if size as size_t >= strlen(misc_handlers[i as usize].key)
            && strncmp(
                p,
                misc_handlers[i as usize].key,
                strlen(misc_handlers[i as usize].key),
            ) == 0
        {
            return true_0 != 0;
        }
        i = i.wrapping_add(1);
    }
    return false_0 != 0;
}
#[unsafe(export_name = "__pdf_impl_spc_misc_setup_handler")]
pub unsafe extern "C-unwind" fn spc_misc_setup_handler(
    mut handle: *mut spc_handler,
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut key: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut keylen: ::core::ffi::c_int = 0;
    let mut i: size_t = 0;
    if !(!handle.is_null() && !spe.is_null() && !args.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"handle && spe && args\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    key = (*args).curptr;
    while (*args).curptr < (*args).endptr
        && isalpha(
            *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int,
        ) != 0
    {
        (*args).curptr = (*args).curptr.offset(1);
    }
    if (*args).curptr < (*args).endptr
        && *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ':' as i32
    {
        (*args).curptr = (*args).curptr.offset(1);
    }
    keylen = (*args).curptr.offset_from(key) as ::core::ffi::c_long as ::core::ffi::c_int;
    if keylen < 1 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    i = 0 as size_t;
    while i
        < (::core::mem::size_of::<[spc_handler; 9]>() as usize)
            .wrapping_div(::core::mem::size_of::<spc_handler>() as usize)
    {
        if keylen as size_t == strlen(misc_handlers[i as usize].key)
            && strncmp(key, misc_handlers[i as usize].key, keylen as size_t) == 0
        {
            skip_white(&raw mut (*args).curptr, (*args).endptr);
            (*args).command = misc_handlers[i as usize].key;
            (*handle).key = b"???:\0" as *const u8 as *const ::core::ffi::c_char;
            (*handle).exec = misc_handlers[i as usize].exec;
            return 0 as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as ::core::ffi::c_int);
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
pub unsafe extern "C" fn __pdf_entry_spc_misc_at_begin_document(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_misc_at_begin_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_misc_at_end_document(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_misc_at_end_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_misc_at_begin_page(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_misc_at_begin_page() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_misc_at_begin_form(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_misc_at_begin_form() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_misc_at_end_form(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_misc_at_end_form() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_misc_check_special(
    mut buffer: *const ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_misc_check_special(buffer, size) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_misc_setup_handler(
    mut handle: *mut spc_handler,
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_misc_setup_handler(handle, spe, args) })
    {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
