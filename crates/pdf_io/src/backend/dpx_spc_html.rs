/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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
#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn, static_mut_refs, dangerous_implicit_autorefs)]
use crate::backend::dpx_pdfdev::transform_info_clear;
use crate::backend::dpx_pdfdraw::pdf_dev_transform;
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn isspace(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn tan(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atof(_: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn strcat(__s1: *mut ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcpy(__dst: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_link_obj"]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_null"]
    fn pdf_new_null() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_string"]
    fn pdf_new_string(str: *const ::core::ffi::c_void, length: size_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_string_value"]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_lookup_dict"]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> ::core::ffi::c_int;
    fn spc_warn(spe: *mut spc_env, fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_spc_begin_annot"]
    fn spc_begin_annot(spe: *mut spc_env, annot_dict: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_end_annot"]
    fn spc_end_annot(spe: *mut spc_env) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_put_image"]
    fn spc_put_image(
        spe: *mut spc_env, res_id: ::core::ffi::c_int, ti: *mut transform_info, xpos: ::core::ffi::c_double, ypos: ::core::ffi::c_double,
    );
    #[link_name = "__pdf_impl_parse_float_decimal"]
    fn parse_float_decimal(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_c_ident"]
    fn parse_c_ident(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_doc_get_reference"]
    fn pdf_doc_get_reference(category: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_doc_add_names"]
    fn pdf_doc_add_names(
        category: *const ::core::ffi::c_char, key: *const ::core::ffi::c_void, keylen: ::core::ffi::c_int, value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_xgstate_push"]
    fn pdf_dev_xgstate_push(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_dev_xgstate_pop"]
    fn pdf_dev_xgstate_pop();
    #[link_name = "__pdf_impl_pdf_ximage_load_image"]
    fn pdf_ximage_load_image(ident: *const ::core::ffi::c_char, filename: *const ::core::ffi::c_char, options: load_options) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
use crate::compat::renderer::pdf_coord;
use crate::compat::renderer::pdf_rect;
use crate::compat::renderer::pdf_tmatrix;
use crate::compat::renderer::transform_info;
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
pub type spc_handler_fn_ptr = Option<unsafe extern "C-unwind" fn(*mut spc_env, *mut spc_arg) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const ::core::ffi::c_char,
    pub exec: spc_handler_fn_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_html_ {
    pub opts: C2RustUnnamed_0,
    pub link_dict: *mut pdf_obj,
    pub baseurl: *mut ::core::ffi::c_char,
    pub pending_type: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub extensions: ::core::ffi::c_int,
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
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846264338327950288f64;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
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
pub const INFO_HAS_WIDTH: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const INFO_HAS_HEIGHT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const ANCHOR_TYPE_HREF: ::core::ffi::c_int = 0;
pub const ANCHOR_TYPE_NAME: ::core::ffi::c_int = 1;

unsafe extern "C-unwind" fn parse_key_val(
    mut pp: *mut *const ::core::ffi::c_char, mut endptr: *const ::core::ffi::c_char, mut kp: *mut *mut ::core::ffi::c_char,
    mut vp: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut q: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut k: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    p = *pp;
    while p < endptr && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    v = ::core::ptr::null_mut::<::core::ffi::c_char>();
    k = v;
    q = p;
    n = 0 as ::core::ffi::c_int;
    while p < endptr
        && (*p as ::core::ffi::c_int >= 'a' as i32 && *p as ::core::ffi::c_int <= 'z' as i32
            || *p as ::core::ffi::c_int >= 'A' as i32 && *p as ::core::ffi::c_int <= 'Z' as i32
            || *p as ::core::ffi::c_int >= '0' as i32 && *p as ::core::ffi::c_int <= '9' as i32
            || *p as ::core::ffi::c_int == '-' as i32
            || *p as ::core::ffi::c_int == ':' as i32)
    {
        n += 1;
        p = p.offset(1);
    }
    if n == 0 as ::core::ffi::c_int {
        *vp = ::core::ptr::null_mut::<::core::ffi::c_char>();
        *kp = *vp;
        return -(1 as ::core::ffi::c_int);
    }
    k = new(((n + 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
        as *mut ::core::ffi::c_char;
    memcpy(k as *mut ::core::ffi::c_void, q as *const ::core::ffi::c_void, n as size_t);
    *k.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    if p.offset(2 as ::core::ffi::c_int as isize) >= endptr
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '=' as i32
        || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '"' as i32
            && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\'' as i32
    {
        k = mfree(k as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
        *pp = p;
        error = -(1 as ::core::ffi::c_int);
    } else {
        let mut qchr: ::core::ffi::c_char = *p.offset(1 as ::core::ffi::c_int as isize);
        p = p.offset(2 as ::core::ffi::c_int as isize);
        q = p;
        n = 0 as ::core::ffi::c_int;
        while p < endptr && *p as ::core::ffi::c_int != qchr as ::core::ffi::c_int {
            p = p.offset(1);
            n += 1;
        }
        if p == endptr || *p as ::core::ffi::c_int != qchr as ::core::ffi::c_int {
            error = -(1 as ::core::ffi::c_int);
        } else {
            v = new(
                ((n + 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
            ) as *mut ::core::ffi::c_char;
            memcpy(v as *mut ::core::ffi::c_void, q as *const ::core::ffi::c_void, n as size_t);
            *v.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
            p = p.offset(1);
        }
    }
    *kp = k;
    *vp = v;
    *pp = p;
    return error;
}
pub const HTML_TAG_NAME_MAX: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const HTML_TAG_TYPE_EMPTY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HTML_TAG_TYPE_OPEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const HTML_TAG_TYPE_CLOSE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn read_html_tag(
    mut name: *mut ::core::ffi::c_char, mut attr: *mut pdf_obj, mut type_0: *mut ::core::ffi::c_int, mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = *pp;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while p < endptr && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    if p >= endptr || *p as ::core::ffi::c_int != '<' as i32 {
        return -(1 as ::core::ffi::c_int);
    }
    *type_0 = HTML_TAG_TYPE_OPEN;
    p = p.offset(1);
    while p < endptr && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    if p < endptr && *p as ::core::ffi::c_int == '/' as i32 {
        *type_0 = HTML_TAG_TYPE_CLOSE;
        p = p.offset(1);
        while p < endptr && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            p = p.offset(1);
        }
    }
    n = 0 as ::core::ffi::c_int;
    while p < endptr
        && n < HTML_TAG_NAME_MAX
        && !(*p as ::core::ffi::c_int == '>' as i32
            || *p as ::core::ffi::c_int == '/' as i32
            || isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0)
    {
        *name.offset(n as isize) = *p;
        n += 1;
        p = p.offset(1);
    }
    *name.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    if n == 0 as ::core::ffi::c_int
        || p == endptr
        || !(*p as ::core::ffi::c_int == '>' as i32
            || *p as ::core::ffi::c_int == '/' as i32
            || isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0)
    {
        *pp = p;
        return -(1 as ::core::ffi::c_int);
    }
    while p < endptr && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    while p < endptr && error == 0 && *p as ::core::ffi::c_int != '/' as i32 && *p as ::core::ffi::c_int != '>' as i32 {
        let mut kp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut vp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        error = parse_key_val(&raw mut p, endptr, &raw mut kp, &raw mut vp);
        if error == 0 {
            if !kp.is_null() {
                let mut _p: *mut ::core::ffi::c_char = kp;
                while *_p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    if *_p as ::core::ffi::c_int >= 'A' as i32 && *_p as ::core::ffi::c_int <= 'Z' as i32 {
                        *_p = (*_p as ::core::ffi::c_int - 'A' as i32 + 'a' as i32) as ::core::ffi::c_char;
                    }
                    _p = _p.offset(1);
                }
            }
            pdf_add_dict(
                attr,
                pdf_new_name(kp),
                pdf_new_string(vp as *const ::core::ffi::c_void, strlen(vp).wrapping_add(1 as size_t)),
            );
            free(kp as *mut ::core::ffi::c_void);
            free(vp as *mut ::core::ffi::c_void);
        }
        while p < endptr && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            p = p.offset(1);
        }
    }
    if error != 0 {
        *pp = p;
        return error;
    }
    if p < endptr && *p as ::core::ffi::c_int == '/' as i32 {
        *type_0 = HTML_TAG_TYPE_EMPTY;
        p = p.offset(1);
        while p < endptr && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            p = p.offset(1);
        }
    }
    if p == endptr || *p as ::core::ffi::c_int != '>' as i32 {
        *pp = p;
        return -(1 as ::core::ffi::c_int);
    }
    p = p.offset(1);
    if !name.is_null() {
        let mut _p_0: *mut ::core::ffi::c_char = name;
        while *_p_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if *_p_0 as ::core::ffi::c_int >= 'A' as i32 && *_p_0 as ::core::ffi::c_int <= 'Z' as i32 {
                *_p_0 = (*_p_0 as ::core::ffi::c_int - 'A' as i32 + 'a' as i32) as ::core::ffi::c_char;
            }
            _p_0 = _p_0.offset(1);
        }
    }
    *pp = p;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_html__init(mut dp: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    (*sd).link_dict = ::core::ptr::null_mut::<pdf_obj>();
    (*sd).baseurl = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*sd).pending_type = -(1 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_html__clean(mut spe: *mut spc_env, mut dp: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    free((*sd).baseurl as *mut ::core::ffi::c_void);
    if (*sd).pending_type >= 0 as ::core::ffi::c_int || !(*sd).link_dict.is_null() {
        pdf_warning!(b"Unclosed html anchor found.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    pdf_release_obj((*sd).link_dict);
    (*sd).pending_type = -(1 as ::core::ffi::c_int);
    (*sd).baseurl = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*sd).link_dict = ::core::ptr::null_mut::<pdf_obj>();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_html__bophook(mut spe: *mut spc_env, mut dp: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    if (*sd).pending_type >= 0 as ::core::ffi::c_int {
        pdf_warning!(b"...html anchor continues from previous page processed...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_html__eophook(mut spe: *mut spc_env, mut dp: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut sd: *mut spc_html_ = dp as *mut spc_html_;
    if (*sd).pending_type >= 0 as ::core::ffi::c_int {
        pdf_warning!(b"Unclosed html anchor at end-of-page!\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn fqurl(mut baseurl: *const ::core::ffi::c_char, mut name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    len = strlen(name) as ::core::ffi::c_int;
    if !baseurl.is_null() {
        len = (len as size_t).wrapping_add(strlen(baseurl).wrapping_add(1 as size_t)) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
    q = new(((len + 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
        as *mut ::core::ffi::c_char;
    *q = '\0' as i32 as ::core::ffi::c_char;
    if !baseurl.is_null() && *baseurl.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0 {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        strcpy(q, baseurl);
        p = q.offset(strlen(q) as isize).offset(-(1 as ::core::ffi::c_int as isize));
        if *p as ::core::ffi::c_int == '/' as i32 {
            *p = '\0' as i32 as ::core::ffi::c_char;
        }
        if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0
            && *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '/' as i32
        {
            strcat(q, b"/\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    strcat(q, name);
    return q;
}
unsafe extern "C-unwind" fn html_open_link(
    mut spe: *mut spc_env, mut name: *const ::core::ffi::c_char, mut sd: *mut spc_html_,
) -> ::core::ffi::c_int {
    let mut color: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut url: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"name\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if !(*sd).link_dict.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"sd->link_dict == NULL\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*sd).link_dict = pdf_new_dict();
    pdf_add_dict(
        (*sd).link_dict,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Annot\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        (*sd).link_dict,
        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Link\0" as *const u8 as *const ::core::ffi::c_char),
    );
    color = pdf_new_array();
    pdf_add_array(color, pdf_new_number(0.0f64));
    pdf_add_array(color, pdf_new_number(0.0f64));
    pdf_add_array(color, pdf_new_number(1.0f64));
    pdf_add_dict((*sd).link_dict, pdf_new_name(b"C\0" as *const u8 as *const ::core::ffi::c_char), color);
    url = fqurl((*sd).baseurl, name);
    if *url.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32 {
        pdf_add_dict(
            (*sd).link_dict,
            pdf_new_name(b"Dest\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                url.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                strlen(url.offset(1 as ::core::ffi::c_int as isize)),
            ),
        );
    } else {
        let mut action: *mut pdf_obj = pdf_new_dict();
        pdf_add_dict(
            action,
            pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"Action\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_dict(
            action,
            pdf_new_name(b"S\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"URI\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_dict(
            action,
            pdf_new_name(b"URI\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(url as *const ::core::ffi::c_void, strlen(url)),
        );
        pdf_add_dict(
            (*sd).link_dict,
            pdf_new_name(b"A\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_link_obj(action),
        );
        pdf_release_obj(action);
    }
    free(url as *mut ::core::ffi::c_void);
    spc_begin_annot(spe, pdf_link_obj((*sd).link_dict));
    (*sd).pending_type = ANCHOR_TYPE_HREF;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn html_open_dest(
    mut spe: *mut spc_env, mut name: *const ::core::ffi::c_char, mut sd: *mut spc_html_,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0;
    let mut array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut page_ref: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    cp.x = (*spe).x_user;
    cp.y = (*spe).y_user;
    pdf_dev_transform(&raw mut cp, ::core::ptr::null::<pdf_tmatrix>());
    page_ref = pdf_doc_get_reference(b"@THISPAGE\0" as *const u8 as *const ::core::ffi::c_char);
    if page_ref.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"page_ref\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    array = pdf_new_array();
    pdf_add_array(array, page_ref);
    pdf_add_array(array, pdf_new_name(b"XYZ\0" as *const u8 as *const ::core::ffi::c_char));
    pdf_add_array(array, pdf_new_null());
    pdf_add_array(array, pdf_new_number(cp.y + 24.0f64));
    pdf_add_array(array, pdf_new_null());
    error = pdf_doc_add_names(
        b"Dests\0" as *const u8 as *const ::core::ffi::c_char,
        name as *const ::core::ffi::c_void,
        strlen(name) as ::core::ffi::c_int,
        array,
    );
    if error != 0 {
        pdf_warning!(b"Failed to add named destination: %s\0" as *const u8 as *const ::core::ffi::c_char, name);
    }
    (*sd).pending_type = ANCHOR_TYPE_NAME;
    return error;
}
unsafe extern "C-unwind" fn spc_html__anchor_open(mut spe: *mut spc_env, mut attr: *mut pdf_obj, mut sd: *mut spc_html_) -> ::core::ffi::c_int {
    let mut href: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut name: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*sd).pending_type >= 0 as ::core::ffi::c_int || !(*sd).link_dict.is_null() {
        pdf_warning!(b"Nested html anchors found!\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    href = pdf_lookup_dict(attr, b"href\0" as *const u8 as *const ::core::ffi::c_char);
    name = pdf_lookup_dict(attr, b"name\0" as *const u8 as *const ::core::ffi::c_char);
    if !href.is_null() && !name.is_null() {
        pdf_warning!(b"Sorry, you can't have both \"href\" and \"name\" in anchor tag...\0" as *const u8 as *const ::core::ffi::c_char);
        error = -(1 as ::core::ffi::c_int);
    } else if !href.is_null() {
        error = html_open_link(spe, pdf_string_value(href) as *const ::core::ffi::c_char, sd);
    } else if !name.is_null() {
        error = html_open_dest(spe, pdf_string_value(name) as *const ::core::ffi::c_char, sd);
    } else {
        pdf_warning!(b"You should have \"href\" or \"name\" in anchor tag!\0" as *const u8 as *const ::core::ffi::c_char);
        error = -(1 as ::core::ffi::c_int);
    }
    return error;
}
unsafe extern "C-unwind" fn spc_html__anchor_close(mut spe: *mut spc_env, mut sd: *mut spc_html_) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    match (*sd).pending_type {
        ANCHOR_TYPE_HREF => {
            if !(*sd).link_dict.is_null() {
                spc_end_annot(spe);
                pdf_release_obj((*sd).link_dict);
                (*sd).link_dict = ::core::ptr::null_mut::<pdf_obj>();
                (*sd).pending_type = -(1 as ::core::ffi::c_int);
            } else {
                pdf_warning!(b"Closing html anchor (link) without starting!\0" as *const u8 as *const ::core::ffi::c_char);
                error = -(1 as ::core::ffi::c_int);
            }
        }
        ANCHOR_TYPE_NAME => {
            (*sd).pending_type = -(1 as ::core::ffi::c_int);
        }
        _ => {
            pdf_warning!(b"No corresponding opening tag for html anchor.\0" as *const u8 as *const ::core::ffi::c_char);
            error = -(1 as ::core::ffi::c_int);
        }
    }
    return error;
}
unsafe extern "C-unwind" fn spc_html__base_empty(mut spe: *mut spc_env, mut attr: *mut pdf_obj, mut sd: *mut spc_html_) -> ::core::ffi::c_int {
    let mut href: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut vp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    href = pdf_lookup_dict(attr, b"href\0" as *const u8 as *const ::core::ffi::c_char);
    if href.is_null() {
        pdf_warning!(b"\"href\" not found for \"base\" tag!\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    vp = pdf_string_value(href) as *mut ::core::ffi::c_char;
    if !(*sd).baseurl.is_null() {
        pdf_warning!(
            b"\"baseurl\" changed: \"%s\" --> \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            (*sd).baseurl,
            vp
        );
        free((*sd).baseurl as *mut ::core::ffi::c_void);
    }
    (*sd).baseurl = new(
        (strlen(vp).wrapping_add(1 as size_t) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t
    ) as *mut ::core::ffi::c_char;
    strcpy((*sd).baseurl, vp);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn atopt(input: *const ::core::ffi::c_char) -> f64 {
    let mut cursor = input;
    let end = input.add(strlen(input));
    let number = parse_float_decimal(&mut cursor, end);
    if number.is_null() {
        pdf_warning!(c"Invalid length value: %s (%c)".as_ptr(), input, *cursor as ::core::ffi::c_int);
        return 0.0;
    }
    let value = atof(number);
    free(number.cast());
    let unit = parse_c_ident(&mut cursor, end);
    if unit.is_null() {
        return value;
    }
    let bytes = std::ffi::CStr::from_ptr(unit).to_bytes();
    let factor = if bytes == b"px" {
        1.0
    } else {
        super::dpx_dpxutil::unit_to_bp(bytes).unwrap_or_else(|| {
            pdf_warning!(c"Unknown unit of measure: %s".as_ptr(), unit);
            1.0
        })
    };
    free(unit.cast());
    value * factor
}
unsafe extern "C-unwind" fn create_xgstate(mut a: ::core::ffi::c_double) -> *mut pdf_obj {
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    dict = pdf_new_dict();
    pdf_add_dict(
        dict,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"ExtGState\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(dict, pdf_new_name(b"ca\0" as *const u8 as *const ::core::ffi::c_char), pdf_new_number(a));
    return dict;
}
unsafe extern "C-unwind" fn spc_html__img_empty(mut spe: *mut spc_env, mut attr: *mut pdf_obj) -> ::core::ffi::c_int {
    let mut src: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
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
    let mut id: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut alpha: ::core::ffi::c_double = 1.0f64;
    pdf_warning!(b"html \"img\" tag found (not completed, plese don't use!).\0" as *const u8 as *const ::core::ffi::c_char);
    src = pdf_lookup_dict(attr, b"src\0" as *const u8 as *const ::core::ffi::c_char);
    if src.is_null() {
        pdf_warning!(b"\"src\" attribute not found for \"img\" tag!\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    transform_info_clear(&raw mut ti);
    obj = pdf_lookup_dict(attr, b"width\0" as *const u8 as *const ::core::ffi::c_char);
    if !obj.is_null() {
        ti.width = atopt(pdf_string_value(obj) as *const ::core::ffi::c_char);
        ti.flags |= INFO_HAS_WIDTH;
    }
    obj = pdf_lookup_dict(attr, b"height\0" as *const u8 as *const ::core::ffi::c_char);
    if !obj.is_null() {
        ti.height = atopt(pdf_string_value(obj) as *const ::core::ffi::c_char);
        ti.flags |= INFO_HAS_HEIGHT;
    }
    obj = pdf_lookup_dict(attr, b"svg:opacity\0" as *const u8 as *const ::core::ffi::c_char);
    if !obj.is_null() {
        alpha = atof(pdf_string_value(obj) as *const ::core::ffi::c_char);
        if alpha < 0.0f64 || alpha > 1.0f64 {
            pdf_warning!(
                b"Invalid opacity value: %s\0" as *const u8 as *const ::core::ffi::c_char,
                pdf_string_value(obj) as *mut ::core::ffi::c_char
            );
            alpha = 1.0f64;
        }
    }
    obj = pdf_lookup_dict(attr, b"svg:transform\0" as *const u8 as *const ::core::ffi::c_char);
    if !obj.is_null() {
        let mut p: *const ::core::ffi::c_char = pdf_string_value(obj) as *const ::core::ffi::c_char;
        let mut N: pdf_tmatrix = pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        };
        while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            p = p.offset(1);
        }
        while *p as ::core::ffi::c_int != 0 && error == 0 {
            N.a = 1.0f64;
            N.b = 0.0f64;
            N.c = 0.0f64;
            N.d = 1.0f64;
            N.e = 0.0f64;
            N.f = 0.0f64;
            error = cvt_a_to_tmatrix(&raw mut N, p, &raw mut p);
            if error == 0 {
                N.f = -N.f;
                let mut _tmp_a: ::core::ffi::c_double = 0.;
                let mut _tmp_b: ::core::ffi::c_double = 0.;
                let mut _tmp_c: ::core::ffi::c_double = 0.;
                let mut _tmp_d: ::core::ffi::c_double = 0.;
                _tmp_a = ti.matrix.a;
                _tmp_b = ti.matrix.b;
                _tmp_c = ti.matrix.c;
                _tmp_d = ti.matrix.d;
                ti.matrix.a = N.a * _tmp_a + N.b * _tmp_c;
                ti.matrix.b = N.a * _tmp_b + N.b * _tmp_d;
                ti.matrix.c = N.c * _tmp_a + N.d * _tmp_c;
                ti.matrix.d = N.c * _tmp_b + N.d * _tmp_d;
                ti.matrix.e += N.e * _tmp_a + N.f * _tmp_c;
                ti.matrix.f += N.e * _tmp_b + N.f * _tmp_d;
                while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
                    p = p.offset(1);
                }
                if *p as ::core::ffi::c_int == ',' as i32 {
                    p = p.offset(1);
                    while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
                        p = p.offset(1);
                    }
                }
            }
        }
    }
    if error != 0 {
        pdf_warning!(b"Error in html \"img\" tag attribute.\0" as *const u8 as *const ::core::ffi::c_char);
        return error;
    }
    id = pdf_ximage_load_image(
        ::core::ptr::null::<::core::ffi::c_char>(),
        pdf_string_value(src) as *const ::core::ffi::c_char,
        options,
    );
    if id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Could not find/load image: %s\0" as *const u8 as *const ::core::ffi::c_char,
            pdf_string_value(src) as *mut ::core::ffi::c_char
        );
        error = -(1 as ::core::ffi::c_int);
    } else {
        if alpha != 0.0f64 {
            let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            dict = create_xgstate(alpha);
            pdf_dev_xgstate_push(dict);
        }
        spc_put_image(spe, id, &raw mut ti, (*spe).x_user, (*spe).y_user);
        if alpha != 0.0f64 {
            pdf_dev_xgstate_pop();
        }
    }
    return error;
}
unsafe extern "C-unwind" fn spc_handler_html_default(mut spe: *mut spc_env, mut ap: *mut spc_arg) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut sd: *mut spc_html_ = &raw mut (*__pdf_state)._html_state;
    let mut name: [::core::ffi::c_char; 128] = [0; 128];
    let mut attr: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut type_0: ::core::ffi::c_int = HTML_TAG_TYPE_OPEN;
    if (*ap).curptr >= (*ap).endptr {
        return 0 as ::core::ffi::c_int;
    }
    attr = pdf_new_dict();
    error = read_html_tag(
        &raw mut name as *mut ::core::ffi::c_char,
        attr,
        &raw mut type_0,
        &raw mut (*ap).curptr,
        (*ap).endptr,
    );
    if error != 0 {
        pdf_release_obj(attr);
        return error;
    }
    if streq_ptr(
        &raw mut name as *mut ::core::ffi::c_char,
        b"a\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        match type_0 {
            HTML_TAG_TYPE_OPEN => {
                error = spc_html__anchor_open(spe, attr, sd);
            }
            HTML_TAG_TYPE_CLOSE => {
                error = spc_html__anchor_close(spe, sd);
            }
            _ => {
                pdf_warning!(b"Empty html anchor tag???\0" as *const u8 as *const ::core::ffi::c_char);
                error = -(1 as ::core::ffi::c_int);
            }
        }
    } else if streq_ptr(
        &raw mut name as *mut ::core::ffi::c_char,
        b"base\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if type_0 == HTML_TAG_TYPE_CLOSE {
            pdf_warning!(b"Close tag for \"base\"???\0" as *const u8 as *const ::core::ffi::c_char);
            error = -(1 as ::core::ffi::c_int);
        } else {
            error = spc_html__base_empty(spe, attr, sd);
        }
    } else if streq_ptr(
        &raw mut name as *mut ::core::ffi::c_char,
        b"img\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        if type_0 == HTML_TAG_TYPE_CLOSE {
            pdf_warning!(b"Close tag for \"img\"???\0" as *const u8 as *const ::core::ffi::c_char);
            error = -(1 as ::core::ffi::c_int);
        } else {
            error = spc_html__img_empty(spe, attr);
        }
    }
    pdf_release_obj(attr);
    while (*ap).curptr < (*ap).endptr
        && isspace(*(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
    {
        (*ap).curptr = (*ap).curptr.offset(1);
    }
    return error;
}
unsafe extern "C-unwind" fn cvt_a_to_tmatrix(
    mut M: *mut pdf_tmatrix, mut ptr: *const ::core::ffi::c_char, mut nextptr: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ptr;
    let mut n: ::core::ffi::c_int = 0;
    let mut v: [::core::ffi::c_double; 6] = [0.; 6];
    const TKEYS: [*const ::core::ffi::c_char; 7] = [
        b"matrix\0" as *const u8 as *const ::core::ffi::c_char,
        b"translate\0" as *const u8 as *const ::core::ffi::c_char,
        b"scale\0" as *const u8 as *const ::core::ffi::c_char,
        b"rotate\0" as *const u8 as *const ::core::ffi::c_char,
        b"skewX\0" as *const u8 as *const ::core::ffi::c_char,
        b"skewY\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut k: ::core::ffi::c_int = 0;
    while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    q = parse_c_ident(&raw mut p, p.offset(strlen(p) as isize));
    if q.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    k = 0 as ::core::ffi::c_int;
    while !TKEYS[k as usize].is_null() && strcmp(q, TKEYS[k as usize]) != 0 {
        k += 1;
    }
    free(q as *mut ::core::ffi::c_void);
    while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int != '(' as i32 || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    p = p.offset(1);
    while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    n = 0 as ::core::ffi::c_int;
    while n < 6 as ::core::ffi::c_int && *p as ::core::ffi::c_int != 0 && *p as ::core::ffi::c_int != ')' as i32 {
        q = parse_float_decimal(&raw mut p, p.offset(strlen(p) as isize));
        if q.is_null() {
            break;
        }
        v[n as usize] = atof(q);
        if *p as ::core::ffi::c_int == ',' as i32 {
            p = p.offset(1);
        }
        while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == ',' as i32 {
            p = p.offset(1);
            while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
                p = p.offset(1);
            }
        }
        free(q as *mut ::core::ffi::c_void);
        n += 1;
    }
    if *p as ::core::ffi::c_int != ')' as i32 {
        return -(1 as ::core::ffi::c_int);
    }
    p = p.offset(1);
    match k {
        K_TRNS__MATRIX => {
            if n != 6 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            (*M).a = v[0 as ::core::ffi::c_int as usize];
            (*M).c = v[1 as ::core::ffi::c_int as usize];
            (*M).b = v[2 as ::core::ffi::c_int as usize];
            (*M).d = v[3 as ::core::ffi::c_int as usize];
            (*M).e = v[4 as ::core::ffi::c_int as usize];
            (*M).f = v[5 as ::core::ffi::c_int as usize];
        }
        K_TRNS__TRANSLATE => {
            if n != 1 as ::core::ffi::c_int && n != 2 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            (*M).d = 1.0f64;
            (*M).a = (*M).d;
            (*M).b = 0.0f64;
            (*M).c = (*M).b;
            (*M).e = v[0 as ::core::ffi::c_int as usize];
            (*M).f = if n == 2 as ::core::ffi::c_int {
                v[1 as ::core::ffi::c_int as usize]
            } else {
                0.0f64
            };
        }
        K_TRNS__SCALE => {
            if n != 1 as ::core::ffi::c_int && n != 2 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            (*M).a = v[0 as ::core::ffi::c_int as usize];
            (*M).d = if n == 2 as ::core::ffi::c_int {
                v[1 as ::core::ffi::c_int as usize]
            } else {
                v[0 as ::core::ffi::c_int as usize]
            };
            (*M).b = 0.0f64;
            (*M).c = (*M).b;
            (*M).f = 0.0f64;
            (*M).e = (*M).f;
        }
        K_TRNS__ROTATE => {
            if n != 1 as ::core::ffi::c_int && n != 3 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            (*M).a = cos(v[0 as ::core::ffi::c_int as usize] * M_PI / 180.0f64);
            (*M).c = sin(v[0 as ::core::ffi::c_int as usize] * M_PI / 180.0f64);
            (*M).b = -(*M).c;
            (*M).d = (*M).a;
            (*M).e = if n == 3 as ::core::ffi::c_int {
                v[1 as ::core::ffi::c_int as usize]
            } else {
                0.0f64
            };
            (*M).f = if n == 3 as ::core::ffi::c_int {
                v[2 as ::core::ffi::c_int as usize]
            } else {
                0.0f64
            };
        }
        K_TRNS__SKEWX => {
            if n != 1 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            (*M).d = 1.0f64;
            (*M).a = (*M).d;
            (*M).c = 0.0f64;
            (*M).b = tan(v[0 as ::core::ffi::c_int as usize] * M_PI / 180.0f64);
        }
        K_TRNS__SKEWY => {
            if n != 1 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            (*M).d = 1.0f64;
            (*M).a = (*M).d;
            (*M).c = tan(v[0 as ::core::ffi::c_int as usize] * M_PI / 180.0f64);
            (*M).b = 0.0f64;
        }
        _ => {}
    }
    if !nextptr.is_null() {
        *nextptr = p;
    }
    return 0 as ::core::ffi::c_int;
}
pub const K_TRNS__MATRIX: ::core::ffi::c_int = 0;
pub const K_TRNS__TRANSLATE: ::core::ffi::c_int = 1;
pub const K_TRNS__SCALE: ::core::ffi::c_int = 2;
pub const K_TRNS__ROTATE: ::core::ffi::c_int = 3;
pub const K_TRNS__SKEWX: ::core::ffi::c_int = 4;
pub const K_TRNS__SKEWY: ::core::ffi::c_int = 5;
#[unsafe(export_name = "__pdf_impl_spc_html_at_begin_document")]
pub unsafe extern "C-unwind" fn spc_html_at_begin_document() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut sd: *mut spc_html_ = &raw mut (*__pdf_state)._html_state;
    return spc_handler_html__init(sd as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_spc_html_at_begin_page")]
pub unsafe extern "C-unwind" fn spc_html_at_begin_page() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut sd: *mut spc_html_ = &raw mut (*__pdf_state)._html_state;
    return spc_handler_html__bophook(::core::ptr::null_mut::<spc_env>(), sd as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_spc_html_at_end_page")]
pub unsafe extern "C-unwind" fn spc_html_at_end_page() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut sd: *mut spc_html_ = &raw mut (*__pdf_state)._html_state;
    return spc_handler_html__eophook(::core::ptr::null_mut::<spc_env>(), sd as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_spc_html_at_end_document")]
pub unsafe extern "C-unwind" fn spc_html_at_end_document() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut sd: *mut spc_html_ = &raw mut (*__pdf_state)._html_state;
    return spc_handler_html__clean(::core::ptr::null_mut::<spc_env>(), sd as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_spc_html_check_special")]
pub unsafe extern "C-unwind" fn spc_html_check_special(mut buffer: *const ::core::ffi::c_char, mut size: ::core::ffi::c_int) -> bool {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = buffer;
    endptr = p.offset(size as isize);
    while p < endptr && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    size = endptr.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int;
    if size as size_t >= strlen(b"html:\0" as *const u8 as *const ::core::ffi::c_char)
        && memcmp(
            p as *const ::core::ffi::c_void,
            b"html:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"html:\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[unsafe(export_name = "__pdf_impl_spc_html_setup_handler")]
pub unsafe extern "C-unwind" fn spc_html_setup_handler(mut sph: *mut spc_handler, mut spe: *mut spc_env, mut ap: *mut spc_arg) -> ::core::ffi::c_int {
    if !(!sph.is_null() && !spe.is_null() && !ap.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"sph && spe && ap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    while (*ap).curptr < (*ap).endptr
        && isspace(*(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
    {
        (*ap).curptr = (*ap).curptr.offset(1);
    }
    if (*ap)
        .curptr
        .offset(strlen(b"html:\0" as *const u8 as *const ::core::ffi::c_char) as isize)
        > (*ap).endptr
        || memcmp(
            (*ap).curptr as *const ::core::ffi::c_void,
            b"html:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"html:\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*ap).command = b"\0" as *const u8 as *const ::core::ffi::c_char;
    (*sph).key = b"html:\0" as *const u8 as *const ::core::ffi::c_char;
    (*sph).exec =
        Some(spc_handler_html_default as unsafe extern "C-unwind" fn(*mut spc_env, *mut spc_arg) -> ::core::ffi::c_int) as spc_handler_fn_ptr;
    (*ap).curptr = (*ap)
        .curptr
        .offset(strlen(b"html:\0" as *const u8 as *const ::core::ffi::c_char) as isize);
    while (*ap).curptr < (*ap).endptr
        && isspace(*(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
    {
        (*ap).curptr = (*ap).curptr.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn memcmp(__s1: *const ::core::ffi::c_void, __s2: *const ::core::ffi::c_void, __n: size_t) -> ::core::ffi::c_int;
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_html_at_begin_document(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_html_at_begin_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_html_at_begin_page(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_html_at_begin_page() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_html_at_end_page(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_html_at_end_page() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_html_at_end_document(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_html_at_end_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_html_check_special(
    mut buffer: *const ::core::ffi::c_char, mut size: ::core::ffi::c_int, _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_html_check_special(buffer, size) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_html_setup_handler(
    mut sph: *mut spc_handler, mut spe: *mut spc_env, mut ap: *mut spc_arg, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_html_setup_handler(sph, spe, ap) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

pub(crate) struct DocumentState {
    _html_state: spc_html_,
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self {
            _html_state: spc_html_ {
                opts: C2RustUnnamed_0 {
                    extensions: 0 as ::core::ffi::c_int,
                },
                link_dict: ::core::ptr::null_mut::<pdf_obj>(),
                baseurl: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                pending_type: -(1 as ::core::ffi::c_int),
            },
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe { (*crate::compat::document::current()).backend.dpx_spc_html.pointer }
}
