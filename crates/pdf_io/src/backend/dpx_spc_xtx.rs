/*  This is xdvipdfmx, an extended version of dvipdfmx,
    an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2013-2019 by the dvipdfmx project team.

    Copyright (c) 2006 SIL International
    Originally written by Jonathan Kew

    This file based on spc_pdfm.c, part of the dvipdfmx project:

    Copyright (C) 2002 by Jin-Hwan Cho and Shunsaku Hirata.

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
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    fn xrealloc(
        old_address: *mut ::core::ffi::c_void,
        new_size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_reset_fonts"]
    fn pdf_dev_reset_fonts(newpage: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_dev_reset_color"]
    fn pdf_dev_reset_color(force: ::core::ffi::c_int);
    fn spc_warn(spe: *mut spc_env, fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_spc_set_fixed_point"]
    fn spc_set_fixed_point(spe: *mut spc_env, x: ::core::ffi::c_double, y: ::core::ffi::c_double);
    #[link_name = "__pdf_impl_spc_get_fixed_point"]
    fn spc_get_fixed_point(
        spe: *mut spc_env,
        x: *mut ::core::ffi::c_double,
        y: *mut ::core::ffi::c_double,
    );
    #[link_name = "__pdf_impl_spc_dup_fixed_point"]
    fn spc_dup_fixed_point(spe: *mut spc_env);
    #[link_name = "__pdf_impl_spc_pop_fixed_point"]
    fn spc_pop_fixed_point(spe: *mut spc_env);
    #[link_name = "__pdf_impl_parse_c_ident"]
    fn parse_c_ident(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    static mut work_buffer: [::core::ffi::c_char; 0];
    #[link_name = "__pdf_impl_pdf_doc_add_page_content"]
    fn pdf_doc_add_page_content(buffer: *const ::core::ffi::c_char, length: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_doc_set_bgcolor"]
    fn pdf_doc_set_bgcolor(color: *const pdf_color);
    #[link_name = "__pdf_impl_pdf_dev_concat"]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_gsave"]
    fn pdf_dev_gsave() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_grestore"]
    fn pdf_dev_grestore() -> ::core::ffi::c_int;
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
    #[link_name = "__pdf_impl_spc_util_read_colorspec"]
    fn spc_util_read_colorspec(
        spe: *mut spc_env,
        colorspec: *mut pdf_color,
        args: *mut spc_arg,
        syntax: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_util_read_numbers"]
    fn spc_util_read_numbers(
        values: *mut ::core::ffi::c_double,
        num_values: ::core::ffi::c_int,
        args: *mut spc_arg,
    ) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
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
use crate::compat::renderer::pdf_coord;
use crate::compat::renderer::pdf_rect;
use crate::compat::renderer::pdf_tmatrix;
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
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846264338327950288f64;
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
pub const FONTMAP_RMODE_REPLACE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FONTMAP_RMODE_APPEND: ::core::ffi::c_int = '+' as i32;
pub const FONTMAP_RMODE_REMOVE: ::core::ffi::c_int = '-' as i32;
#[unsafe(export_name = "__pdf_impl_spc_handler_xtx_do_transform")]
pub unsafe extern "C-unwind" fn spc_handler_xtx_do_transform(
    mut spe: *mut spc_env,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
    mut c: ::core::ffi::c_double,
    mut d: ::core::ffi::c_double,
    mut e: ::core::ffi::c_double,
    mut f: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        b: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        c: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        d: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        e: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        f: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    };
    let mut pt: pdf_coord = pdf_coord { x: 0., y: 0. };
    M.a = a;
    M.b = b;
    M.c = c;
    M.d = d;
    M.e = (1.0f64 - M.a) * x_user - M.c * y_user + e;
    M.f = (1.0f64 - M.d) * y_user - M.b * x_user + f;
    pdf_dev_concat(&raw mut M);
    spc_get_fixed_point(spe, &raw mut pt.x, &raw mut pt.y);
    spc_set_fixed_point(spe, x_user - pt.x, y_user - pt.y);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_xtx_scale(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut values: [::core::ffi::c_double; 2] = [0.; 2];
    if spc_util_read_numbers(
        (&raw mut values as *mut ::core::ffi::c_double).offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
        2 as ::core::ffi::c_int,
        args,
    ) < 2 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*args).curptr = (*args).endptr;
    return spc_handler_xtx_do_transform(
        spe,
        (*spe).x_user,
        (*spe).y_user,
        values[0 as ::core::ffi::c_int as usize],
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        values[1 as ::core::ffi::c_int as usize],
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
}
static mut scaleFactors: *mut pdf_coord = ::core::ptr::null::<pdf_coord>() as *mut pdf_coord;
static mut scaleFactorCount: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C-unwind" fn spc_handler_xtx_bscale(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut values: [::core::ffi::c_double; 2] = [0.; 2];
    scaleFactorCount += 1;
    if scaleFactorCount & 0xf as ::core::ffi::c_int == 0 {
        scaleFactors = crate::backend::runtime::reallocate(
            scaleFactors as *mut ::core::ffi::c_void,
            ((scaleFactorCount + 16 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<pdf_coord>() as size_t),
        ) as *mut pdf_coord;
    }
    if spc_util_read_numbers(
        (&raw mut values as *mut ::core::ffi::c_double).offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_double,
        2 as ::core::ffi::c_int,
        args,
    ) < 2 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if fabs(values[0 as ::core::ffi::c_int as usize]) < 1.0e-7f64
        || fabs(values[1 as ::core::ffi::c_int as usize]) < 1.0e-7f64
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*scaleFactors.offset(scaleFactorCount as isize)).x =
        1 as ::core::ffi::c_int as ::core::ffi::c_double / values[0 as ::core::ffi::c_int as usize];
    (*scaleFactors.offset(scaleFactorCount as isize)).y =
        1 as ::core::ffi::c_int as ::core::ffi::c_double / values[1 as ::core::ffi::c_int as usize];
    (*args).curptr = (*args).endptr;
    return spc_handler_xtx_do_transform(
        spe,
        (*spe).x_user,
        (*spe).y_user,
        values[0 as ::core::ffi::c_int as usize],
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        values[1 as ::core::ffi::c_int as usize],
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
}
unsafe extern "C-unwind" fn spc_handler_xtx_escale(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let fresh2 = scaleFactorCount;
    scaleFactorCount = scaleFactorCount - 1;
    let mut factor: pdf_coord = *scaleFactors.offset(fresh2 as isize);
    (*args).curptr = (*args).endptr;
    return spc_handler_xtx_do_transform(
        spe,
        (*spe).x_user,
        (*spe).y_user,
        factor.x,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        factor.y,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
}
unsafe extern "C-unwind" fn spc_handler_xtx_rotate(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut value: ::core::ffi::c_double = 0.;
    if spc_util_read_numbers(&raw mut value, 1 as ::core::ffi::c_int, args)
        < 1 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*args).curptr = (*args).endptr;
    return spc_handler_xtx_do_transform(
        spe,
        (*spe).x_user,
        (*spe).y_user,
        cos(value * M_PI / 180 as ::core::ffi::c_int as ::core::ffi::c_double),
        sin(value * M_PI / 180 as ::core::ffi::c_int as ::core::ffi::c_double),
        -sin(value * M_PI / 180 as ::core::ffi::c_int as ::core::ffi::c_double),
        cos(value * M_PI / 180 as ::core::ffi::c_int as ::core::ffi::c_double),
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
}
#[unsafe(export_name = "__pdf_impl_spc_handler_xtx_gsave")]
pub unsafe extern "C-unwind" fn spc_handler_xtx_gsave(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    pdf_dev_gsave();
    spc_dup_fixed_point(spe);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_handler_xtx_grestore")]
pub unsafe extern "C-unwind" fn spc_handler_xtx_grestore(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    pdf_dev_grestore();
    spc_pop_fixed_point(spe);
    pdf_dev_reset_fonts(0 as ::core::ffi::c_int);
    pdf_dev_reset_color(0 as ::core::ffi::c_int);
    pdf_dev_reset_xgstate(0 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_xtx_papersize(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_xtx_backgroundcolor(
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
    error = spc_util_read_colorspec(spe, &raw mut colorspec, args, 0 as ::core::ffi::c_int);
    if error != 0 {
        pdf_warning!(b"No valid color specified?\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        pdf_doc_set_bgcolor(&raw mut colorspec);
    }
    return error;
}
pub const THEBUFFLENGTH: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn spc_handler_xtx_fontmapline(
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
        pdf_warning!(b"Empty fontmapline special?\0" as *const u8 as *const ::core::ffi::c_char);
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
unsafe extern "C-unwind" fn spc_handler_xtx_fontmapfile(
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
    return error;
}
static mut overlay_name: [::core::ffi::c_char; 256] = [0; 256];
unsafe extern "C-unwind" fn spc_handler_xtx_initoverlay(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        return -(1 as ::core::ffi::c_int);
    }
    strncpy(
        &raw mut overlay_name as *mut ::core::ffi::c_char,
        (*args).curptr,
        (*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long as size_t,
    );
    overlay_name[(*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long as usize] =
        0 as ::core::ffi::c_char;
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_xtx_clipoverlay(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr >= (*args).endptr {
        return -(1 as ::core::ffi::c_int);
    }
    pdf_dev_grestore();
    pdf_dev_gsave();
    if strncmp(
        &raw mut overlay_name as *mut ::core::ffi::c_char,
        (*args).curptr,
        strlen(&raw mut overlay_name as *mut ::core::ffi::c_char),
    ) != 0 as ::core::ffi::c_int
        && strncmp(
            b"all\0" as *const u8 as *const ::core::ffi::c_char,
            (*args).curptr,
            strlen(b"all\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0 as ::core::ffi::c_int
    {
        pdf_doc_add_page_content(
            b" 0 0 m W n\0" as *const u8 as *const ::core::ffi::c_char,
            10 as ::core::ffi::c_uint,
        );
    }
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_xtx_renderingmode(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut value: ::core::ffi::c_double = 0.;
    if spc_util_read_numbers(&raw mut value, 1 as ::core::ffi::c_int, args)
        < 1 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if (value as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || value as ::core::ffi::c_int > 7 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Invalid text rendering mode %d.\n\0" as *const u8 as *const ::core::ffi::c_char,
            value as ::core::ffi::c_int
        );
        return -(1 as ::core::ffi::c_int);
    }
    sprintf(
        &raw mut work_buffer as *mut ::core::ffi::c_char,
        b" %d Tr\0" as *const u8 as *const ::core::ffi::c_char,
        value as ::core::ffi::c_int,
    );
    pdf_doc_add_page_content(
        &raw mut work_buffer as *mut ::core::ffi::c_char,
        strlen(&raw mut work_buffer as *mut ::core::ffi::c_char) as ::core::ffi::c_uint,
    );
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        pdf_doc_add_page_content(
            b" \0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_uint,
        );
        pdf_doc_add_page_content(
            (*args).curptr,
            (*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long
                as ::core::ffi::c_uint,
        );
    }
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_xtx_unsupportedcolor(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    pdf_warning!(
        b"xetex-style \\special{x:%s} is not supported by this driver;\nupdate document or driver to use \\special{color} instead.\0"
        as * const u8 as * const ::core::ffi::c_char, (* args).command
    );
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_xtx_unsupported(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    pdf_warning!(
        b"xetex-style \\special{x:%s} is not supported by this driver.\0" as *const u8
            as *const ::core::ffi::c_char,
        (*args).command
    );
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
static mut xtx_handlers: [spc_handler; 21] = unsafe {
    [
        spc_handler {
            key: b"textcolor\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_unsupportedcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"textcolorpush\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_unsupportedcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"textcolorpop\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_unsupportedcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"rulecolor\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_unsupportedcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"rulecolorpush\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_unsupportedcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"rulecolorpop\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_unsupportedcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"papersize\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_papersize
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"backgroundcolor\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_backgroundcolor
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"gsave\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_gsave
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"grestore\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_grestore
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"scale\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_scale
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bscale\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_bscale
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"escale\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_escale
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"rotate\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_rotate
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"fontmapline\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_fontmapline
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"fontmapfile\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_fontmapfile
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"shadow\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_unsupported
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"colorshadow\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_unsupported
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"renderingmode\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_renderingmode
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"initoverlay\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_initoverlay
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"clipoverlay\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_xtx_clipoverlay
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
    ]
};
#[unsafe(export_name = "__pdf_impl_spc_xtx_check_special")]
pub unsafe extern "C-unwind" fn spc_xtx_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> bool {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = buf;
    endptr = p.offset(len as isize);
    skip_white(&raw mut p, endptr);
    if p.offset(strlen(b"x:\0" as *const u8 as *const ::core::ffi::c_char) as isize) <= endptr
        && memcmp(
            p as *const ::core::ffi::c_void,
            b"x:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"x:\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[unsafe(export_name = "__pdf_impl_spc_xtx_setup_handler")]
pub unsafe extern "C-unwind" fn spc_xtx_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_uint = 0;
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
        .offset(strlen(b"x:\0" as *const u8 as *const ::core::ffi::c_char) as isize)
        >= (*ap).endptr
        || memcmp(
            (*ap).curptr as *const ::core::ffi::c_void,
            b"x:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"x:\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0
    {
        pdf_warning!(b"Not x: special???\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    (*ap).curptr = (*ap)
        .curptr
        .offset(strlen(b"x:\0" as *const u8 as *const ::core::ffi::c_char) as isize);
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    q = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        i = 0 as ::core::ffi::c_uint;
        while (i as usize)
            < (::core::mem::size_of::<[spc_handler; 21]>() as usize)
                .wrapping_div(::core::mem::size_of::<spc_handler>() as usize)
        {
            if streq_ptr(q, xtx_handlers[i as usize].key) {
                (*ap).command = xtx_handlers[i as usize].key;
                (*sph).key = b"x:\0" as *const u8 as *const ::core::ffi::c_char;
                (*sph).exec = xtx_handlers[i as usize].exec;
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
    fn free(_: *mut ::core::ffi::c_void);
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_handler_xtx_do_transform(
    mut spe: *mut spc_env,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
    mut a: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
    mut c: ::core::ffi::c_double,
    mut d: ::core::ffi::c_double,
    mut e: ::core::ffi::c_double,
    mut f: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        spc_handler_xtx_do_transform(spe, x_user, y_user, a, b, c, d, e, f)
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
pub unsafe extern "C" fn __pdf_entry_spc_handler_xtx_gsave(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_handler_xtx_gsave(spe, args) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_handler_xtx_grestore(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_handler_xtx_grestore(spe, args) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_xtx_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_xtx_check_special(buf, len) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_xtx_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_xtx_setup_handler(sph, spe, ap) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
