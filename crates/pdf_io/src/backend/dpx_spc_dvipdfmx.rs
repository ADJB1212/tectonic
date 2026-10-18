/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
#![allow(
    warnings,
    clippy::all,
    unsafe_op_in_unsafe_fn,
    static_mut_refs,
    dangerous_implicit_autorefs
)]
use crate::backend::dpx_pdfdev::transform_info_clear;
use crate::compat::allocation::free;
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
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_boolean_value"]
    fn pdf_boolean_value(object: *mut pdf_obj) -> ::core::ffi::c_char;
    fn spc_warn(spe: *mut spc_env, fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_spc_set_linkmode"]
    fn spc_set_linkmode(spe: *mut spc_env, mode: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_spc_set_phantom"]
    fn spc_set_phantom(
        spe: *mut spc_env,
        height: ::core::ffi::c_double,
        depth: ::core::ffi::c_double,
    );
    #[link_name = "__pdf_impl_parse_c_ident"]
    fn parse_c_ident(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_parse_pdf_boolean"]
    fn parse_pdf_boolean(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_spc_util_read_dimtrns"]
    fn spc_util_read_dimtrns(
        spe: *mut spc_env,
        dimtrns: *mut transform_info,
        args: *mut spc_arg,
        syntax: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
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
pub const INFO_HAS_HEIGHT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn spc_handler_null(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_dvipdfmx_catch_phantom(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut mode: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0;
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    let mut b: *mut pdf_obj = parse_pdf_boolean(&raw mut (*args).curptr, (*args).endptr);
    if b.is_null() {
        pdf_warning!(
            b"A boolean value expected but not found...\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    mode = pdf_boolean_value(b) as ::core::ffi::c_int;
    pdf_release_obj(b);
    spc_set_linkmode(spe, mode);
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if mode == 1 as ::core::ffi::c_int && (*args).curptr < (*args).endptr {
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
        error = spc_util_read_dimtrns(spe, &raw mut ti, args, 0 as ::core::ffi::c_int);
        if error != 0 {
            return -(1 as ::core::ffi::c_int);
        }
        if ti.flags & INFO_HAS_HEIGHT != 0 {
            spc_set_phantom(spe, ti.height, ti.depth);
        }
        skip_white(&raw mut (*args).curptr, (*args).endptr);
    }
    return 0 as ::core::ffi::c_int;
}
const DVIPDFMX_HANDLERS: [spc_handler; 2] = [
    spc_handler {
        key: b"config\0" as *const u8 as *const ::core::ffi::c_char,
        exec: Some(
            spc_handler_null
                as unsafe extern "C-unwind" fn(*mut spc_env, *mut spc_arg) -> ::core::ffi::c_int,
        ),
    },
    spc_handler {
        key: b"catch_phantom\0" as *const u8 as *const ::core::ffi::c_char,
        exec: Some(
            spc_handler_dvipdfmx_catch_phantom
                as unsafe extern "C-unwind" fn(*mut spc_env, *mut spc_arg) -> ::core::ffi::c_int,
        ),
    },
];
#[unsafe(export_name = "__pdf_impl_spc_dvipdfmx_check_special")]
pub unsafe extern "C-unwind" fn spc_dvipdfmx_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> bool {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = buf;
    endptr = p.offset(len as isize);
    skip_white(&raw mut p, endptr);
    if p.offset(strlen(b"dvipdfmx:\0" as *const u8 as *const ::core::ffi::c_char) as isize)
        <= endptr
        && memcmp(
            p as *const ::core::ffi::c_void,
            b"dvipdfmx:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"dvipdfmx:\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[unsafe(export_name = "__pdf_impl_spc_dvipdfmx_setup_handler")]
pub unsafe extern "C-unwind" fn spc_dvipdfmx_setup_handler(
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
        .offset(strlen(b"dvipdfmx:\0" as *const u8 as *const ::core::ffi::c_char) as isize)
        >= (*ap).endptr
        || memcmp(
            (*ap).curptr as *const ::core::ffi::c_void,
            b"dvipdfmx:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"dvipdfmx:\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0
    {
        pdf_warning!(b"Not dvipdfmx: special???\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    (*ap).curptr = (*ap)
        .curptr
        .offset(strlen(b"dvipdfmx:\0" as *const u8 as *const ::core::ffi::c_char) as isize);
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    q = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        i = 0 as size_t;
        while i
            < (::core::mem::size_of::<[spc_handler; 2]>() as usize)
                .wrapping_div(::core::mem::size_of::<spc_handler>() as usize)
        {
            if streq_ptr(q, DVIPDFMX_HANDLERS[i as usize].key) {
                (*ap).command = DVIPDFMX_HANDLERS[i as usize].key;
                (*sph).key = b"dvipdfmx:\0" as *const u8 as *const ::core::ffi::c_char;
                (*sph).exec = DVIPDFMX_HANDLERS[i as usize].exec;
                skip_white(&raw mut (*ap).curptr, (*ap).endptr);
                error = 0 as ::core::ffi::c_int;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        free(q as *mut ::core::ffi::c_void);
    }
    return error;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_dvipdfmx_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_dvipdfmx_check_special(buf, len) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_dvipdfmx_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_dvipdfmx_setup_handler(sph, spe, ap) })
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
