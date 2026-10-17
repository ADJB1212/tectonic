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
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn isascii(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_set"]
    fn pdf_color_set(sc: *mut pdf_color, fc: *mut pdf_color);
    #[link_name = "__pdf_impl_pdf_color_push"]
    fn pdf_color_push(sc: *mut pdf_color, fc: *mut pdf_color);
    #[link_name = "__pdf_impl_pdf_color_pop"]
    fn pdf_color_pop();
    #[link_name = "__pdf_impl_pdf_color_clear_stack"]
    fn pdf_color_clear_stack();
    fn spc_warn(spe: *mut spc_env, fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_parse_c_ident"]
    fn parse_c_ident(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_doc_set_bgcolor"]
    fn pdf_doc_set_bgcolor(color: *const pdf_color);
    #[link_name = "__pdf_impl_spc_util_read_colorspec"]
    fn spc_util_read_colorspec(
        spe: *mut spc_env,
        colorspec: *mut pdf_color,
        args: *mut spc_arg,
        syntax: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
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
unsafe extern "C-unwind" fn spc_handler_color_push(
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
    error = spc_util_read_colorspec(spe, &raw mut colorspec, args, 1 as ::core::ffi::c_int);
    if error == 0 {
        pdf_color_push(&raw mut colorspec, &raw mut colorspec);
    }
    return error;
}
unsafe extern "C-unwind" fn spc_handler_color_pop(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    pdf_color_pop();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_color_default(
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
    error = spc_util_read_colorspec(spe, &raw mut colorspec, args, 1 as ::core::ffi::c_int);
    if error == 0 {
        pdf_color_clear_stack();
        pdf_color_set(&raw mut colorspec, &raw mut colorspec);
    }
    return error;
}
unsafe extern "C-unwind" fn spc_handler_background(
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
    error = spc_util_read_colorspec(spe, &raw mut colorspec, args, 1 as ::core::ffi::c_int);
    if error == 0 {
        pdf_doc_set_bgcolor(&raw mut colorspec);
    }
    return error;
}
unsafe extern "C-unwind" fn skip_blank(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) {
    let mut p: *const ::core::ffi::c_char = *pp;
    while p < endptr
        && (crate::backend::runtime::is_ascii(*p as ::core::ffi::c_int) != 0
            && (*p as ::core::ffi::c_uchar as ::core::ffi::c_int == ' ' as i32
                || *p as ::core::ffi::c_uchar as ::core::ffi::c_int == '\t' as i32))
    {
        p = p.offset(1);
    }
    *pp = p;
}
#[unsafe(export_name = "__pdf_impl_spc_color_check_special")]
pub unsafe extern "C-unwind" fn spc_color_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> bool {
    let mut r: bool = false_0 != 0;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = buf;
    endptr = p.offset(len as isize);
    skip_blank(&raw mut p, endptr);
    q = parse_c_ident(&raw mut p, endptr);
    if q.is_null() {
        return false_0 != 0;
    } else if streq_ptr(q, b"color\0" as *const u8 as *const ::core::ffi::c_char) {
        r = true_0 != 0;
    } else if streq_ptr(
        q,
        b"background\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        r = true_0 != 0;
    }
    free(q as *mut ::core::ffi::c_void);
    return r;
}
#[unsafe(export_name = "__pdf_impl_spc_color_setup_handler")]
pub unsafe extern "C-unwind" fn spc_color_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
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
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    q = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
    if q.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    if streq_ptr(
        q,
        b"background\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        (*ap).command = b"background\0" as *const u8 as *const ::core::ffi::c_char;
        (*sph).exec = Some(
            spc_handler_background
                as unsafe extern "C-unwind" fn(*mut spc_env, *mut spc_arg) -> ::core::ffi::c_int,
        ) as spc_handler_fn_ptr;
        free(q as *mut ::core::ffi::c_void);
    } else if streq_ptr(q, b"color\0" as *const u8 as *const ::core::ffi::c_char) {
        free(q as *mut ::core::ffi::c_void);
        p = (*ap).curptr;
        q = parse_c_ident(&raw mut p, (*ap).endptr);
        if q.is_null() {
            return -(1 as ::core::ffi::c_int);
        } else if streq_ptr(q, b"push\0" as *const u8 as *const ::core::ffi::c_char) {
            (*ap).command = b"push\0" as *const u8 as *const ::core::ffi::c_char;
            (*sph).exec = Some(
                spc_handler_color_push
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ) as spc_handler_fn_ptr;
            (*ap).curptr = p;
        } else if streq_ptr(q, b"pop\0" as *const u8 as *const ::core::ffi::c_char) {
            (*ap).command = b"pop\0" as *const u8 as *const ::core::ffi::c_char;
            (*sph).exec = Some(
                spc_handler_color_pop
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ) as spc_handler_fn_ptr;
            (*ap).curptr = p;
        } else {
            (*ap).command = b"\0" as *const u8 as *const ::core::ffi::c_char;
            (*sph).exec = Some(
                spc_handler_color_default
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ) as spc_handler_fn_ptr;
        }
        free(q as *mut ::core::ffi::c_void);
    } else {
        pdf_warning!(b"Not color/background special?\0" as *const u8 as *const ::core::ffi::c_char);
        free(q as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    return 0 as ::core::ffi::c_int;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_color_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_color_check_special(buf, len) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_color_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_color_setup_handler(sph, spe, ap) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
