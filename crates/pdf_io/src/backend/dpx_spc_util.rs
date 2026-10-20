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
#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn, static_mut_refs, dangerous_implicit_autorefs)]
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn isascii(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atof(_: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[cfg_attr(windows, link_name = "_stricmp")]
    fn strcasecmp(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_link_obj"]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_name_value"]
    fn pdf_name_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_get_array"]
    fn pdf_get_array(array: *mut pdf_obj, idx: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_color_rgbcolor"]
    fn pdf_color_rgbcolor(color: *mut pdf_color, r: ::core::ffi::c_double, g: ::core::ffi::c_double, b: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_cmykcolor"]
    fn pdf_color_cmykcolor(
        color: *mut pdf_color, c: ::core::ffi::c_double, m: ::core::ffi::c_double, y: ::core::ffi::c_double, k: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_graycolor"]
    fn pdf_color_graycolor(color: *mut pdf_color, g: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_spotcolor"]
    fn pdf_color_spotcolor(color: *mut pdf_color, color_name: *mut ::core::ffi::c_char, c: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_copycolor"]
    fn pdf_color_copycolor(color1: *mut pdf_color, color2: *const pdf_color);
    fn spc_warn(spe: *mut spc_env, fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_spc_lookup_object"]
    fn spc_lookup_object(ident: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_float_decimal"]
    fn parse_float_decimal(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_c_ident"]
    fn parse_c_ident(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_parse_opt_ident"]
    fn parse_opt_ident(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_pdf_name"]
    fn parse_pdf_name(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_defineresource"]
    fn pdf_defineresource(
        category: *const ::core::ffi::c_char, resname: *const ::core::ffi::c_char, object: *mut pdf_obj, flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_findresource"]
    fn pdf_findresource(category: *const ::core::ffi::c_char, resname: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
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
use crate::compat::renderer::transform_info;
pub type pdf_page_boundary = ::core::ffi::c_uint;
pub const pdf_page_boundary_bleedbox: pdf_page_boundary = 5;
pub const pdf_page_boundary_trimbox: pdf_page_boundary = 4;
pub const pdf_page_boundary_artbox: pdf_page_boundary = 3;
pub const pdf_page_boundary_cropbox: pdf_page_boundary = 1;
pub const pdf_page_boundary_mediabox: pdf_page_boundary = 2;
pub const pdf_page_boundary__auto: pdf_page_boundary = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colordef_ {
    pub key: *const ::core::ffi::c_char,
    pub color: pdf_color,
}
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
pub const PDF_NAME: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PDF_ARRAY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_CMYK: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const PDF_COLORSPACE_TYPE_GRAY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const PDF_COLORSPACE_TYPE_INVALID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_DEVICEGRAY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_DEVICERGB: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_DEVICECMYK: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_CALGRAY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_CALRGB: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_LAB: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_ICCBASED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_SEPARATION: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_DEVICEN: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_INDEXED: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_PATTERN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const PDF_COLOR_COMPONENT_MAX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const INFO_HAS_USER_BBOX: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const INFO_HAS_WIDTH: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const INFO_HAS_HEIGHT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const INFO_DO_CLIP: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const INFO_DO_HIDE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn skip_blank(mut pp: *mut *const ::core::ffi::c_char, mut endptr: *const ::core::ffi::c_char) {
    let mut p: *const ::core::ffi::c_char = *pp;
    while p < endptr
        && (crate::backend::runtime::is_ascii(*p as ::core::ffi::c_int) != 0
            && (*p as ::core::ffi::c_uchar as ::core::ffi::c_int == ' ' as i32 || *p as ::core::ffi::c_uchar as ::core::ffi::c_int == '\t' as i32))
    {
        p = p.offset(1);
    }
    *pp = p;
}
#[unsafe(export_name = "__pdf_impl_spc_util_read_numbers")]
pub unsafe extern "C-unwind" fn spc_util_read_numbers(
    mut values: *mut ::core::ffi::c_double, mut num_values: ::core::ffi::c_int, mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    skip_blank(&raw mut (*args).curptr, (*args).endptr);
    count = 0 as ::core::ffi::c_int;
    while count < num_values && (*args).curptr < (*args).endptr {
        q = parse_float_decimal(&raw mut (*args).curptr, (*args).endptr);
        if q.is_null() {
            break;
        }
        *values.offset(count as isize) = atof(q);
        free(q as *mut ::core::ffi::c_void);
        skip_blank(&raw mut (*args).curptr, (*args).endptr);
        count += 1;
    }
    return count;
}
unsafe extern "C-unwind" fn rgb_color_from_hsv(
    mut color: *mut pdf_color, mut h: ::core::ffi::c_double, mut s: ::core::ffi::c_double, mut v: ::core::ffi::c_double,
) {
    let mut r: ::core::ffi::c_double = 0.;
    let mut g: ::core::ffi::c_double = 0.;
    let mut b: ::core::ffi::c_double = 0.;
    if color.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"color\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    b = v;
    g = b;
    r = g;
    if s != 0.0f64 {
        let mut h6: ::core::ffi::c_double = 0.;
        let mut f: ::core::ffi::c_double = 0.;
        let mut v1: ::core::ffi::c_double = 0.;
        let mut v2: ::core::ffi::c_double = 0.;
        let mut v3: ::core::ffi::c_double = 0.;
        let mut i: ::core::ffi::c_int = 0;
        h6 = h * 6 as ::core::ffi::c_int as ::core::ffi::c_double;
        i = h6 as ::core::ffi::c_int;
        f = h6 - i as ::core::ffi::c_double;
        v1 = v * (1 as ::core::ffi::c_int as ::core::ffi::c_double - s);
        v2 = v * (1 as ::core::ffi::c_int as ::core::ffi::c_double - s * f);
        v3 = v * (1 as ::core::ffi::c_int as ::core::ffi::c_double - s * (1 as ::core::ffi::c_int as ::core::ffi::c_double - f));
        match i {
            0 => {
                r = v;
                g = v3;
                b = v1;
            }
            1 => {
                r = v2;
                g = v;
                b = v1;
            }
            2 => {
                r = v1;
                g = v;
                b = v3;
            }
            3 => {
                r = v1;
                g = v2;
                b = v;
            }
            4 => {
                r = v3;
                g = v1;
                b = v;
            }
            5 => {
                r = v;
                g = v1;
                b = v2;
            }
            6 => {
                r = v;
                g = v1;
                b = v2;
            }
            _ => {}
        }
    }
    pdf_color_rgbcolor(color, r, g, b);
}
unsafe extern "C-unwind" fn spc_read_color_color(mut spe: *mut spc_env, mut colorspec: *mut pdf_color, mut ap: *mut spc_arg) -> ::core::ffi::c_int {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cv: [::core::ffi::c_double; 4] = [0.; 4];
    let mut nc: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    q = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
    if q.is_null() {
        pdf_warning!(b"No valid color specified?\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    if streq_ptr(q, b"rgb\0" as *const u8 as *const ::core::ffi::c_char) {
        nc = spc_util_read_numbers(&raw mut cv as *mut ::core::ffi::c_double, 3 as ::core::ffi::c_int, ap);
        if nc != 3 as ::core::ffi::c_int {
            pdf_warning!(b"Invalid value for RGB color specification.\0" as *const u8 as *const ::core::ffi::c_char);
            error = -(1 as ::core::ffi::c_int);
        } else {
            pdf_color_rgbcolor(
                colorspec,
                cv[0 as ::core::ffi::c_int as usize],
                cv[1 as ::core::ffi::c_int as usize],
                cv[2 as ::core::ffi::c_int as usize],
            );
        }
    } else if streq_ptr(q, b"cmyk\0" as *const u8 as *const ::core::ffi::c_char) {
        nc = spc_util_read_numbers(&raw mut cv as *mut ::core::ffi::c_double, 4 as ::core::ffi::c_int, ap);
        if nc != 4 as ::core::ffi::c_int {
            pdf_warning!(b"Invalid value for CMYK color specification.\0" as *const u8 as *const ::core::ffi::c_char);
            error = -(1 as ::core::ffi::c_int);
        } else {
            pdf_color_cmykcolor(
                colorspec,
                cv[0 as ::core::ffi::c_int as usize],
                cv[1 as ::core::ffi::c_int as usize],
                cv[2 as ::core::ffi::c_int as usize],
                cv[3 as ::core::ffi::c_int as usize],
            );
        }
    } else if streq_ptr(q, b"gray\0" as *const u8 as *const ::core::ffi::c_char) {
        nc = spc_util_read_numbers(&raw mut cv as *mut ::core::ffi::c_double, 1 as ::core::ffi::c_int, ap);
        if nc != 1 as ::core::ffi::c_int {
            pdf_warning!(b"Invalid value for gray color specification.\0" as *const u8 as *const ::core::ffi::c_char);
            error = -(1 as ::core::ffi::c_int);
        } else {
            pdf_color_graycolor(colorspec, cv[0 as ::core::ffi::c_int as usize]);
        }
    } else if streq_ptr(q, b"spot\0" as *const u8 as *const ::core::ffi::c_char) {
        let mut color_name: *mut ::core::ffi::c_char = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
        if color_name.is_null() {
            pdf_warning!(b"No valid spot color name specified?\0" as *const u8 as *const ::core::ffi::c_char);
            return -(1 as ::core::ffi::c_int);
        }
        skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        nc = spc_util_read_numbers(&raw mut cv as *mut ::core::ffi::c_double, 1 as ::core::ffi::c_int, ap);
        if nc != 1 as ::core::ffi::c_int {
            pdf_warning!(b"Invalid value for spot color specification.\0" as *const u8 as *const ::core::ffi::c_char);
            error = -(1 as ::core::ffi::c_int);
            free(color_name as *mut ::core::ffi::c_void);
        } else {
            pdf_color_spotcolor(colorspec, color_name, cv[0 as ::core::ffi::c_int as usize]);
        }
    } else if streq_ptr(q, b"hsb\0" as *const u8 as *const ::core::ffi::c_char) {
        nc = spc_util_read_numbers(&raw mut cv as *mut ::core::ffi::c_double, 3 as ::core::ffi::c_int, ap);
        if nc != 3 as ::core::ffi::c_int {
            pdf_warning!(b"Invalid value for HSB color specification.\0" as *const u8 as *const ::core::ffi::c_char);
            error = -(1 as ::core::ffi::c_int);
        } else {
            rgb_color_from_hsv(
                colorspec,
                cv[0 as ::core::ffi::c_int as usize],
                cv[1 as ::core::ffi::c_int as usize],
                cv[2 as ::core::ffi::c_int as usize],
            );
            pdf_warning!(
                b"HSB color converted to RGB: hsb: <%g, %g, %g> ==> rgb: <%g, %g, %g>\0" as *const u8 as *const ::core::ffi::c_char,
                cv[0 as ::core::ffi::c_int as usize],
                cv[1 as ::core::ffi::c_int as usize],
                cv[2 as ::core::ffi::c_int as usize],
                (*colorspec).values[0 as ::core::ffi::c_int as usize],
                (*colorspec).values[1 as ::core::ffi::c_int as usize],
                (*colorspec).values[2 as ::core::ffi::c_int as usize]
            );
        }
    } else {
        error = pdf_color_namedcolor(colorspec, q);
        if error != 0 {
            pdf_warning!(b"Unrecognized color name: %s\0" as *const u8 as *const ::core::ffi::c_char, q);
        }
    }
    free(q as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C-unwind" fn spc_read_color_pdf(mut spe: *mut spc_env, mut colorspec: *mut pdf_color, mut ap: *mut spc_arg) -> ::core::ffi::c_int {
    let mut cv: [::core::ffi::c_double; 32] = [0.; 32];
    let mut nc: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut type_0: ::core::ffi::c_int = PDF_COLORSPACE_TYPE_INVALID;
    let mut isarry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    if *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '@' as i32 {
        let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut res_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        ident = parse_opt_ident(&raw mut (*ap).curptr, (*ap).endptr);
        skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        res_id = pdf_findresource(b"ColorSpace\0" as *const u8 as *const ::core::ffi::c_char, ident);
        let mut cspace: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut csname: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        cspace = spc_lookup_object(ident);
        if cspace.is_null() || !(!cspace.is_null() && pdf_obj_typeof(cspace) == PDF_ARRAY) {
            pdf_warning!(
                b"Couldn't find ColorSpace resource for \"@%s\"... (or not an array object?)\0" as *const u8 as *const ::core::ffi::c_char,
                ident,
            );
            free(ident as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        csname = pdf_get_array(cspace, 0 as ::core::ffi::c_int);
        if csname.is_null() || !(!csname.is_null() && pdf_obj_typeof(csname) == PDF_NAME) {
            pdf_warning!(
                b"Invalid ColorSpace resource for \"@%s\" found...\0" as *const u8 as *const ::core::ffi::c_char,
                ident,
            );
            free(ident as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        name = pdf_name_value(csname);
        if strcmp(name, b"Separation\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            type_0 = PDF_COLORSPACE_TYPE_SEPARATION;
        } else if strcmp(name, b"CalGray\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            type_0 = PDF_COLORSPACE_TYPE_CALGRAY;
            nc = 1 as ::core::ffi::c_int;
        } else if strcmp(name, b"CalRGB\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            type_0 = PDF_COLORSPACE_TYPE_CALRGB;
            nc = 3 as ::core::ffi::c_int;
        } else if strcmp(name, b"LAB\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            type_0 = PDF_COLORSPACE_TYPE_LAB;
            nc = 3 as ::core::ffi::c_int;
        } else if strcmp(name, b"ICCBased\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            type_0 = PDF_COLORSPACE_TYPE_ICCBASED;
            nc = -(1 as ::core::ffi::c_int);
        } else if strcmp(name, b"DeviceN\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            type_0 = PDF_COLORSPACE_TYPE_DEVICEN;
            nc = -(1 as ::core::ffi::c_int);
        } else if strcmp(name, b"Indexed\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            type_0 = PDF_COLORSPACE_TYPE_INDEXED;
            nc = 1 as ::core::ffi::c_int;
        } else if strcmp(name, b"Pattern\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            type_0 = PDF_COLORSPACE_TYPE_PATTERN;
            nc = -(1 as ::core::ffi::c_int);
        } else {
            pdf_warning!(
                b"Specified object \"%s\" not a ColorSpace???\0" as *const u8 as *const ::core::ffi::c_char,
                ident,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if res_id < 0 as ::core::ffi::c_int {
            res_id = pdf_defineresource(
                b"ColorSpace\0" as *const u8 as *const ::core::ffi::c_char,
                ident,
                pdf_link_obj(cspace),
                0 as ::core::ffi::c_int,
            );
        }
        free(ident as *mut ::core::ffi::c_void);
        (*colorspec).res_id = res_id;
        (*colorspec).type_0 = type_0;
        if *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '[' as i32 {
            (*ap).curptr = (*ap).curptr.offset(1);
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
            isarry = 1 as ::core::ffi::c_int;
        }
        if nc > 0 as ::core::ffi::c_int {
            let mut n: ::core::ffi::c_int = 0;
            n = spc_util_read_numbers(&raw mut cv as *mut ::core::ffi::c_double, nc, ap);
            if n != nc {
                pdf_warning!(b"Wrong number of color components...\0" as *const u8 as *const ::core::ffi::c_char,);
                return -(1 as ::core::ffi::c_int);
            }
        } else {
            nc = spc_util_read_numbers(&raw mut cv as *mut ::core::ffi::c_double, PDF_COLOR_COMPONENT_MAX, ap);
        }
        (*colorspec).num_components = nc;
        loop {
            let fresh0 = nc;
            nc = nc - 1;
            if !(fresh0 > 0 as ::core::ffi::c_int) {
                break;
            }
            (*colorspec).values[nc as usize] = cv[nc as usize];
        }
        if type_0 == PDF_COLORSPACE_TYPE_PATTERN {
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
            if *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '@' as i32 {
                pdf_warning!(b"An object reference expected but not found for Pattern!\0" as *const u8 as *const ::core::ffi::c_char,);
                return -(1 as ::core::ffi::c_int);
            }
            ident = parse_opt_ident(&raw mut (*ap).curptr, (*ap).endptr);
            res_id = pdf_findresource(b"Pattern\0" as *const u8 as *const ::core::ffi::c_char, ident);
            if res_id < 0 as ::core::ffi::c_int {
                let mut pattern: *mut pdf_obj = spc_lookup_object(ident);
                res_id = pdf_defineresource(
                    b"Pattern\0" as *const u8 as *const ::core::ffi::c_char,
                    ident,
                    pdf_link_obj(pattern),
                    0 as ::core::ffi::c_int,
                );
            }
            (*colorspec).pattern_id = res_id;
            free(ident as *mut ::core::ffi::c_void);
        }
        if isarry != 0 {
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
            if (*ap).curptr >= (*ap).endptr || *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ']' as i32 {
                pdf_warning!(
                    b"Unbalanced '[' and ']' or wrong number of color components in color specification?\0" as *const u8
                        as *const ::core::ffi::c_char
                );
                error = -(1 as ::core::ffi::c_int);
            } else {
                (*ap).curptr = (*ap).curptr.offset(1);
            }
        }
    } else if *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
        let mut csname_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        csname_0 = parse_pdf_name(&raw mut (*ap).curptr, (*ap).endptr);
        if csname_0.is_null() {
            pdf_warning!(b"Failed to read a name object while parsing colorspecification...\0" as *const u8 as *const ::core::ffi::c_char,);
            return -(1 as ::core::ffi::c_int);
        } else {
            let mut name_0: *mut ::core::ffi::c_char = pdf_name_value(csname_0);
            if strcmp(name_0, b"DeviceGray\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                type_0 = PDF_COLORSPACE_TYPE_DEVICEGRAY;
                nc = 1 as ::core::ffi::c_int;
            } else if strcmp(name_0, b"DeviceRGB\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                type_0 = PDF_COLORSPACE_TYPE_DEVICERGB;
                nc = 3 as ::core::ffi::c_int;
            } else if strcmp(name_0, b"DeviceCMYK\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                type_0 = PDF_COLORSPACE_TYPE_DEVICECMYK;
                nc = 4 as ::core::ffi::c_int;
            } else if strcmp(name_0, b"Pattern\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
                type_0 = PDF_COLORSPACE_TYPE_PATTERN;
                nc = 0 as ::core::ffi::c_int;
            } else {
                pdf_warning!(
                    b"Unknown ColorSpace name specified: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    name_0,
                );
                return -(1 as ::core::ffi::c_int);
            }
            pdf_release_obj(csname_0);
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        }
        (*colorspec).res_id = -(1 as ::core::ffi::c_int);
        (*colorspec).type_0 = type_0;
        if *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '[' as i32 {
            (*ap).curptr = (*ap).curptr.offset(1);
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
            isarry = 1 as ::core::ffi::c_int;
        }
        let mut n_0: ::core::ffi::c_int = spc_util_read_numbers(&raw mut cv as *mut ::core::ffi::c_double, nc, ap);
        if n_0 != nc {
            pdf_warning!(b"Wrong number of color components for this ColorSpace...\0" as *const u8 as *const ::core::ffi::c_char,);
            return -(1 as ::core::ffi::c_int);
        }
        (*colorspec).num_components = nc;
        loop {
            let fresh1 = nc;
            nc = nc - 1;
            if !(fresh1 > 0 as ::core::ffi::c_int) {
                break;
            }
            (*colorspec).values[nc as usize] = cv[nc as usize];
        }
        if type_0 == PDF_COLORSPACE_TYPE_PATTERN {
            let mut ident_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut res_id_0: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
            if *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '@' as i32 {
                pdf_warning!(b"An object reference expected but not found for Pattern!\0" as *const u8 as *const ::core::ffi::c_char,);
                return -(1 as ::core::ffi::c_int);
            }
            ident_0 = parse_opt_ident(&raw mut (*ap).curptr, (*ap).endptr);
            res_id_0 = pdf_findresource(b"Pattern\0" as *const u8 as *const ::core::ffi::c_char, ident_0);
            if res_id_0 < 0 as ::core::ffi::c_int {
                let mut pattern_0: *mut pdf_obj = spc_lookup_object(ident_0);
                res_id_0 = pdf_defineresource(
                    b"Pattern\0" as *const u8 as *const ::core::ffi::c_char,
                    ident_0,
                    pdf_link_obj(pattern_0),
                    0 as ::core::ffi::c_int,
                );
            }
            (*colorspec).pattern_id = res_id_0;
            free(ident_0 as *mut ::core::ffi::c_void);
        }
        if isarry != 0 {
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
            if (*ap).curptr >= (*ap).endptr || *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ']' as i32 {
                pdf_warning!(
                    b"Unbalanced '[' and ']' or wrong number of color components in color specification?\0" as *const u8
                        as *const ::core::ffi::c_char
                );
                error = -(1 as ::core::ffi::c_int);
            } else {
                (*ap).curptr = (*ap).curptr.offset(1);
            }
        }
    } else {
        if *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '[' as i32 {
            (*ap).curptr = (*ap).curptr.offset(1);
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
            isarry = 1 as ::core::ffi::c_int;
        }
        nc = spc_util_read_numbers(&raw mut cv as *mut ::core::ffi::c_double, 4 as ::core::ffi::c_int, ap);
        match nc {
            1 => {
                pdf_color_graycolor(colorspec, cv[0 as ::core::ffi::c_int as usize]);
            }
            3 => {
                pdf_color_rgbcolor(
                    colorspec,
                    cv[0 as ::core::ffi::c_int as usize],
                    cv[1 as ::core::ffi::c_int as usize],
                    cv[2 as ::core::ffi::c_int as usize],
                );
            }
            4 => {
                pdf_color_cmykcolor(
                    colorspec,
                    cv[0 as ::core::ffi::c_int as usize],
                    cv[1 as ::core::ffi::c_int as usize],
                    cv[2 as ::core::ffi::c_int as usize],
                    cv[3 as ::core::ffi::c_int as usize],
                );
            }
            _ => {
                q = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
                if q.is_null() {
                    pdf_warning!(b"No valid color specified?\0" as *const u8 as *const ::core::ffi::c_char);
                    return -(1 as ::core::ffi::c_int);
                }
                error = pdf_color_namedcolor(colorspec, q);
                if error != 0 {
                    pdf_warning!(
                        b"Unrecognized color name: %s, keep the current color\0" as *const u8 as *const ::core::ffi::c_char,
                        q
                    );
                }
                free(q as *mut ::core::ffi::c_void);
            }
        }
        if isarry != 0 {
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
            if (*ap).curptr >= (*ap).endptr || *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ']' as i32 {
                pdf_warning!(b"Unbalanced '[' and ']' in color specification.\0" as *const u8 as *const ::core::ffi::c_char);
                error = -(1 as ::core::ffi::c_int);
            } else {
                (*ap).curptr = (*ap).curptr.offset(1);
            }
        }
    }
    return error;
}
#[unsafe(export_name = "__pdf_impl_spc_util_read_colorspec")]
pub unsafe extern "C-unwind" fn spc_util_read_colorspec(
    mut spe: *mut spc_env, mut colorspec: *mut pdf_color, mut ap: *mut spc_arg, mut syntax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !(!colorspec.is_null() && !spe.is_null() && !ap.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"colorspec && spe && ap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr >= (*ap).endptr {
        return -(1 as ::core::ffi::c_int);
    }
    pdf_color_graycolor(colorspec, 0.0f64);
    if syntax != 0 {
        return spc_read_color_color(spe, colorspec, ap);
    } else {
        return spc_read_color_pdf(spe, colorspec, ap);
    };
}
#[unsafe(export_name = "__pdf_impl_spc_util_read_pdfcolor")]
pub unsafe extern "C-unwind" fn spc_util_read_pdfcolor(
    mut spe: *mut spc_env, mut colorspec: *mut pdf_color, mut ap: *mut spc_arg, mut defaultcolor: *mut pdf_color,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(!colorspec.is_null() && !spe.is_null() && !ap.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"colorspec && spe && ap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    if (*ap).curptr >= (*ap).endptr {
        return -(1 as ::core::ffi::c_int);
    }
    error = spc_read_color_pdf(spe, colorspec, ap);
    if error < 0 as ::core::ffi::c_int && !defaultcolor.is_null() {
        pdf_color_copycolor(colorspec, defaultcolor);
        error = 0 as ::core::ffi::c_int;
    }
    return error;
}
unsafe extern "C-unwind" fn spc_util_read_length(spe: *mut spc_env, vp: *mut f64, ap: *mut spc_arg) -> i32 {
    let number = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
    if number.is_null() {
        return -1;
    }
    let value = atof(number);
    free(number.cast());
    let mut scale = 1.0;
    let mut error = 0;
    skip_white(&raw mut (*ap).curptr, (*ap).endptr);
    let mut allocation = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
    if !allocation.is_null() {
        let mut unit = allocation;
        if std::ffi::CStr::from_ptr(unit).to_bytes().starts_with(b"true") {
            scale /= if (*spe).mag != 0.0 { (*spe).mag } else { 1.0 };
            unit = unit.add(4);
            if *unit == 0 {
                free(allocation.cast());
                skip_white(&raw mut (*ap).curptr, (*ap).endptr);
                allocation = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
                unit = allocation;
            }
        }
        if unit.is_null() {
            pdf_warning!(c"Missing unit of measure after \"true\"".as_ptr());
            error = -1;
        } else {
            match super::dpx_dpxutil::unit_to_bp(std::ffi::CStr::from_ptr(unit).to_bytes()) {
                Some(factor) => scale *= factor,
                None => {
                    pdf_warning!(c"Unknown unit of measure: %s".as_ptr(), unit);
                    error = -1;
                }
            }
            free(allocation.cast());
        }
    }
    *vp = value * scale;
    error
}
unsafe extern "C-unwind" fn make_transmatrix(
    mut M: *mut pdf_tmatrix, mut xoffset: ::core::ffi::c_double, mut yoffset: ::core::ffi::c_double, mut xscale: ::core::ffi::c_double,
    mut yscale: ::core::ffi::c_double, mut rotate: ::core::ffi::c_double,
) {
    let mut c: ::core::ffi::c_double = 0.;
    let mut s: ::core::ffi::c_double = 0.;
    c = cos(rotate);
    s = sin(rotate);
    (*M).a = xscale * c;
    (*M).b = xscale * s;
    (*M).c = -yscale * s;
    (*M).d = yscale * c;
    (*M).e = xoffset;
    (*M).f = yoffset;
}
unsafe extern "C-unwind" fn spc_read_dimtrns_dvips(mut spe: *mut spc_env, mut t: *mut transform_info, mut ap: *mut spc_arg) -> ::core::ffi::c_int {
    const DTKEYS: [*const ::core::ffi::c_char; 15] = [
        b"hoffset\0" as *const u8 as *const ::core::ffi::c_char,
        b"voffset\0" as *const u8 as *const ::core::ffi::c_char,
        b"hsize\0" as *const u8 as *const ::core::ffi::c_char,
        b"vsize\0" as *const u8 as *const ::core::ffi::c_char,
        b"hscale\0" as *const u8 as *const ::core::ffi::c_char,
        b"vscale\0" as *const u8 as *const ::core::ffi::c_char,
        b"angle\0" as *const u8 as *const ::core::ffi::c_char,
        b"clip\0" as *const u8 as *const ::core::ffi::c_char,
        b"llx\0" as *const u8 as *const ::core::ffi::c_char,
        b"lly\0" as *const u8 as *const ::core::ffi::c_char,
        b"urx\0" as *const u8 as *const ::core::ffi::c_char,
        b"ury\0" as *const u8 as *const ::core::ffi::c_char,
        b"rwi\0" as *const u8 as *const ::core::ffi::c_char,
        b"rhi\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut xoffset: ::core::ffi::c_double = 0.;
    let mut yoffset: ::core::ffi::c_double = 0.;
    let mut xscale: ::core::ffi::c_double = 0.;
    let mut yscale: ::core::ffi::c_double = 0.;
    let mut rotate: ::core::ffi::c_double = 0.;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    rotate = 0.0f64;
    yoffset = rotate;
    xoffset = yoffset;
    yscale = 1.0f64;
    xscale = yscale;
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    while error == 0 && (*ap).curptr < (*ap).endptr {
        let mut kp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut vp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut k: ::core::ffi::c_int = 0;
        kp = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
        if kp.is_null() {
            break;
        }
        k = 0 as ::core::ffi::c_int;
        while !DTKEYS[k as usize].is_null() && strcmp(kp, DTKEYS[k as usize]) != 0 {
            k += 1;
        }
        if DTKEYS[k as usize].is_null() {
            pdf_warning!(
                b"Unrecognized dimension/transformation key: %s\0" as *const u8 as *const ::core::ffi::c_char,
                kp
            );
            error = -(1 as ::core::ffi::c_int);
            free(kp as *mut ::core::ffi::c_void);
            break;
        } else {
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
            if k == K__CLIP_0 {
                (*t).flags |= INFO_DO_CLIP;
                free(kp as *mut ::core::ffi::c_void);
            } else {
                if (*ap).curptr < (*ap).endptr && *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '=' as i32 {
                    (*ap).curptr = (*ap).curptr.offset(1);
                    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
                }
                vp = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\'' as i32
                    || *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '"' as i32
                {
                    let mut qchr: ::core::ffi::c_char = *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize);
                    (*ap).curptr = (*ap).curptr.offset(1);
                    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
                    vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
                    if !vp.is_null() && qchr as ::core::ffi::c_int != *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
                        pdf_warning!(b"Syntax error in dimension/transformation specification.\0" as *const u8 as *const ::core::ffi::c_char);
                        error = -(1 as ::core::ffi::c_int);
                        vp = mfree(vp as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
                    }
                    (*ap).curptr = (*ap).curptr.offset(1);
                } else {
                    vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                }
                if error == 0 && vp.is_null() {
                    pdf_warning!(
                        b"Missing value for dimension/transformation: %s\0" as *const u8 as *const ::core::ffi::c_char,
                        kp
                    );
                    error = -(1 as ::core::ffi::c_int);
                }
                free(kp as *mut ::core::ffi::c_void);
                if vp.is_null() || error != 0 {
                    break;
                }
                match k {
                    K_TRN__HOFFSET => {
                        xoffset = atof(vp);
                    }
                    K_TRN__VOFFSET => {
                        yoffset = atof(vp);
                    }
                    K_DIM__HSIZE => {
                        (*t).width = atof(vp);
                        (*t).flags |= INFO_HAS_WIDTH;
                    }
                    K_DIM__VSIZE => {
                        (*t).height = atof(vp);
                        (*t).flags |= INFO_HAS_HEIGHT;
                    }
                    K_TRN__HSCALE => {
                        xscale = atof(vp) / 100.0f64;
                    }
                    K_TRN__VSCALE => {
                        yscale = atof(vp) / 100.0f64;
                    }
                    K_TRN__ANGLE => {
                        rotate = M_PI * atof(vp) / 180.0f64;
                    }
                    K_DIM__LLX => {
                        (*t).bbox.llx = atof(vp);
                        (*t).flags |= INFO_HAS_USER_BBOX;
                    }
                    K_DIM__LLY => {
                        (*t).bbox.lly = atof(vp);
                        (*t).flags |= INFO_HAS_USER_BBOX;
                    }
                    K_DIM__URX => {
                        (*t).bbox.urx = atof(vp);
                        (*t).flags |= INFO_HAS_USER_BBOX;
                    }
                    K_DIM__URY => {
                        (*t).bbox.ury = atof(vp);
                        (*t).flags |= INFO_HAS_USER_BBOX;
                    }
                    K_DIM__RWI => {
                        (*t).width = atof(vp) / 10.0f64;
                        (*t).flags |= INFO_HAS_WIDTH;
                    }
                    K_DIM__RHI => {
                        (*t).height = atof(vp) / 10.0f64;
                        (*t).flags |= INFO_HAS_HEIGHT;
                    }
                    _ => {}
                }
                skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
                free(vp as *mut ::core::ffi::c_void);
            }
        }
    }
    make_transmatrix(&raw mut (*t).matrix, xoffset, yoffset, xscale, yscale, rotate);
    return error;
}
pub const K_TRN__HOFFSET: ::core::ffi::c_int = 0;
pub const K_TRN__VOFFSET: ::core::ffi::c_int = 1;
pub const K_DIM__HSIZE: ::core::ffi::c_int = 2;
pub const K_DIM__VSIZE: ::core::ffi::c_int = 3;
pub const K_TRN__HSCALE: ::core::ffi::c_int = 4;
pub const K_TRN__VSCALE: ::core::ffi::c_int = 5;
pub const K_TRN__ANGLE: ::core::ffi::c_int = 6;
pub const K__CLIP_0: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const K_DIM__LLX: ::core::ffi::c_int = 8;
pub const K_DIM__LLY: ::core::ffi::c_int = 9;
pub const K_DIM__URX: ::core::ffi::c_int = 10;
pub const K_DIM__URY: ::core::ffi::c_int = 11;
pub const K_DIM__RWI: ::core::ffi::c_int = 12;
pub const K_DIM__RHI: ::core::ffi::c_int = 13;
unsafe extern "C-unwind" fn spc_read_dimtrns_pdfm(mut spe: *mut spc_env, mut p: *mut transform_info, mut ap: *mut spc_arg) -> ::core::ffi::c_int {
    let mut has_scale: ::core::ffi::c_int = 0;
    let mut has_xscale: ::core::ffi::c_int = 0;
    let mut has_yscale: ::core::ffi::c_int = 0;
    let mut has_rotate: ::core::ffi::c_int = 0;
    let mut has_matrix: ::core::ffi::c_int = 0;
    let mut _dtkeys: [*const ::core::ffi::c_char; 12] = [
        b"width\0" as *const u8 as *const ::core::ffi::c_char,
        b"height\0" as *const u8 as *const ::core::ffi::c_char,
        b"depth\0" as *const u8 as *const ::core::ffi::c_char,
        b"scale\0" as *const u8 as *const ::core::ffi::c_char,
        b"xscale\0" as *const u8 as *const ::core::ffi::c_char,
        b"yscale\0" as *const u8 as *const ::core::ffi::c_char,
        b"rotate\0" as *const u8 as *const ::core::ffi::c_char,
        b"bbox\0" as *const u8 as *const ::core::ffi::c_char,
        b"matrix\0" as *const u8 as *const ::core::ffi::c_char,
        b"clip\0" as *const u8 as *const ::core::ffi::c_char,
        b"hide\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut xscale: ::core::ffi::c_double = 0.;
    let mut yscale: ::core::ffi::c_double = 0.;
    let mut rotate: ::core::ffi::c_double = 0.;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    has_matrix = 0 as ::core::ffi::c_int;
    has_rotate = has_matrix;
    has_scale = has_rotate;
    has_yscale = has_scale;
    has_xscale = has_yscale;
    yscale = 1.0f64;
    xscale = yscale;
    rotate = 0.0f64;
    (*p).flags |= INFO_DO_CLIP;
    (*p).flags &= !INFO_DO_HIDE;
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    while error == 0 && (*ap).curptr < (*ap).endptr {
        let mut kp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut vp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut k: ::core::ffi::c_int = 0;
        kp = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
        if kp.is_null() {
            break;
        }
        skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        k = 0 as ::core::ffi::c_int;
        while !_dtkeys[k as usize].is_null() && strcmp(_dtkeys[k as usize], kp) != 0 {
            k += 1;
        }
        match k {
            K_DIM__WIDTH => {
                error = spc_util_read_length(spe, &raw mut (*p).width, ap);
                (*p).flags |= INFO_HAS_WIDTH;
            }
            K_DIM__HEIGHT => {
                error = spc_util_read_length(spe, &raw mut (*p).height, ap);
                (*p).flags |= INFO_HAS_HEIGHT;
            }
            K_DIM__DEPTH => {
                error = spc_util_read_length(spe, &raw mut (*p).depth, ap);
                (*p).flags |= INFO_HAS_HEIGHT;
            }
            K_TRN__SCALE => {
                vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    yscale = atof(vp);
                    xscale = yscale;
                    has_scale = 1 as ::core::ffi::c_int;
                    free(vp as *mut ::core::ffi::c_void);
                }
            }
            K_TRN__XSCALE => {
                vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    xscale = atof(vp);
                    has_xscale = 1 as ::core::ffi::c_int;
                    free(vp as *mut ::core::ffi::c_void);
                }
            }
            K_TRN__YSCALE => {
                vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    yscale = atof(vp);
                    has_yscale = 1 as ::core::ffi::c_int;
                    free(vp as *mut ::core::ffi::c_void);
                }
            }
            K_TRN__ROTATE => {
                vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    rotate = M_PI * atof(vp) / 180.0f64;
                    has_rotate = 1 as ::core::ffi::c_int;
                    free(vp as *mut ::core::ffi::c_void);
                }
            }
            K_TRN__BBOX => {
                let mut v: [::core::ffi::c_double; 4] = [0.; 4];
                if spc_util_read_numbers(&raw mut v as *mut ::core::ffi::c_double, 4 as ::core::ffi::c_int, ap) != 4 as ::core::ffi::c_int {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    (*p).bbox.llx = v[0 as ::core::ffi::c_int as usize];
                    (*p).bbox.lly = v[1 as ::core::ffi::c_int as usize];
                    (*p).bbox.urx = v[2 as ::core::ffi::c_int as usize];
                    (*p).bbox.ury = v[3 as ::core::ffi::c_int as usize];
                    (*p).flags |= INFO_HAS_USER_BBOX;
                }
            }
            K_TRN__MATRIX => {
                let mut v_0: [::core::ffi::c_double; 6] = [0.; 6];
                if spc_util_read_numbers(&raw mut v_0 as *mut ::core::ffi::c_double, 6 as ::core::ffi::c_int, ap) != 6 as ::core::ffi::c_int {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    (*p).matrix.a = v_0[0 as ::core::ffi::c_int as usize];
                    (*p).matrix.b = v_0[1 as ::core::ffi::c_int as usize];
                    (*p).matrix.c = v_0[2 as ::core::ffi::c_int as usize];
                    (*p).matrix.d = v_0[3 as ::core::ffi::c_int as usize];
                    (*p).matrix.e = v_0[4 as ::core::ffi::c_int as usize];
                    (*p).matrix.f = v_0[5 as ::core::ffi::c_int as usize];
                    has_matrix = 1 as ::core::ffi::c_int;
                }
            }
            K__CLIP => {
                vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    if atof(vp) != 0. {
                        (*p).flags |= INFO_DO_CLIP;
                    } else {
                        (*p).flags &= !INFO_DO_CLIP;
                    }
                    free(vp as *mut ::core::ffi::c_void);
                }
            }
            K__HIDE => {
                (*p).flags |= INFO_DO_HIDE;
            }
            _ => {
                error = -(1 as ::core::ffi::c_int);
            }
        }
        if error != 0 {
            pdf_warning!(
                b"Unrecognized key or invalid value for dimension/transformation: %s\0" as *const u8 as *const ::core::ffi::c_char,
                kp
            );
        } else {
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        }
        free(kp as *mut ::core::ffi::c_void);
    }
    if error == 0 {
        if has_xscale != 0 && (*p).flags & INFO_HAS_WIDTH != 0 {
            pdf_warning!(b"Can't supply both width and xscale. Ignore xscale.\0" as *const u8 as *const ::core::ffi::c_char);
            xscale = 1.0f64;
        } else if has_yscale != 0 && (*p).flags & INFO_HAS_HEIGHT != 0 {
            pdf_warning!(b"Can't supply both height/depth and yscale. Ignore yscale.\0" as *const u8 as *const ::core::ffi::c_char);
            yscale = 1.0f64;
        } else if has_scale != 0 && (has_xscale != 0 || has_yscale != 0) {
            pdf_warning!(b"Can't supply overall scale along with axis scales.\0" as *const u8 as *const ::core::ffi::c_char);
            error = -(1 as ::core::ffi::c_int);
        } else if has_matrix != 0 && (has_scale != 0 || has_xscale != 0 || has_yscale != 0 || has_rotate != 0) {
            pdf_warning!(
                b"Can't supply transform matrix along with scales or rotate. Ignore scales and rotate.\0" as *const u8 as *const ::core::ffi::c_char
            );
        }
    }
    if has_matrix == 0 {
        make_transmatrix(&raw mut (*p).matrix, 0.0f64, 0.0f64, xscale, yscale, rotate);
    }
    if (*p).flags & INFO_HAS_USER_BBOX == 0 {
        (*p).flags &= !INFO_DO_CLIP;
    }
    return error;
}
pub const K_DIM__WIDTH: ::core::ffi::c_int = 0;
pub const K_DIM__HEIGHT: ::core::ffi::c_int = 1;
pub const K_DIM__DEPTH: ::core::ffi::c_int = 2;
pub const K_TRN__SCALE: ::core::ffi::c_int = 3;
pub const K_TRN__XSCALE: ::core::ffi::c_int = 4;
pub const K_TRN__YSCALE: ::core::ffi::c_int = 5;
pub const K_TRN__ROTATE: ::core::ffi::c_int = 6;
pub const K_TRN__BBOX: ::core::ffi::c_int = 7;
pub const K_TRN__MATRIX: ::core::ffi::c_int = 8;
pub const K__CLIP: ::core::ffi::c_int = 9;
pub const K__HIDE: ::core::ffi::c_int = 10;
#[unsafe(export_name = "__pdf_impl_spc_util_read_dimtrns")]
pub unsafe extern "C-unwind" fn spc_util_read_dimtrns(
    mut spe: *mut spc_env, mut ti: *mut transform_info, mut args: *mut spc_arg, mut syntax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if ti.is_null() || spe.is_null() || args.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if syntax != 0 {
        return spc_read_dimtrns_dvips(spe, ti, args);
    } else {
        return spc_read_dimtrns_pdfm(spe, ti, args);
    };
}
#[unsafe(export_name = "__pdf_impl_spc_util_read_blahblah")]
pub unsafe extern "C-unwind" fn spc_util_read_blahblah(
    mut spe: *mut spc_env, mut p: *mut transform_info, mut page_no: *mut ::core::ffi::c_int, mut bbox_type: *mut pdf_page_boundary,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut has_scale: ::core::ffi::c_int = 0;
    let mut has_xscale: ::core::ffi::c_int = 0;
    let mut has_yscale: ::core::ffi::c_int = 0;
    let mut has_rotate: ::core::ffi::c_int = 0;
    let mut has_matrix: ::core::ffi::c_int = 0;
    let mut _dtkeys: [*const ::core::ffi::c_char; 14] = [
        b"width\0" as *const u8 as *const ::core::ffi::c_char,
        b"height\0" as *const u8 as *const ::core::ffi::c_char,
        b"depth\0" as *const u8 as *const ::core::ffi::c_char,
        b"scale\0" as *const u8 as *const ::core::ffi::c_char,
        b"xscale\0" as *const u8 as *const ::core::ffi::c_char,
        b"yscale\0" as *const u8 as *const ::core::ffi::c_char,
        b"rotate\0" as *const u8 as *const ::core::ffi::c_char,
        b"bbox\0" as *const u8 as *const ::core::ffi::c_char,
        b"matrix\0" as *const u8 as *const ::core::ffi::c_char,
        b"clip\0" as *const u8 as *const ::core::ffi::c_char,
        b"hide\0" as *const u8 as *const ::core::ffi::c_char,
        b"page\0" as *const u8 as *const ::core::ffi::c_char,
        b"pagebox\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut xscale: ::core::ffi::c_double = 0.;
    let mut yscale: ::core::ffi::c_double = 0.;
    let mut rotate: ::core::ffi::c_double = 0.;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    has_matrix = 0 as ::core::ffi::c_int;
    has_rotate = has_matrix;
    has_scale = has_rotate;
    has_yscale = has_scale;
    has_xscale = has_yscale;
    yscale = 1.0f64;
    xscale = yscale;
    rotate = 0.0f64;
    (*p).flags |= INFO_DO_CLIP;
    (*p).flags &= !INFO_DO_HIDE;
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    while error == 0 && (*ap).curptr < (*ap).endptr {
        let mut kp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut vp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut k: ::core::ffi::c_int = 0;
        kp = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
        if kp.is_null() {
            break;
        }
        skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        k = 0 as ::core::ffi::c_int;
        while !_dtkeys[k as usize].is_null() && strcmp(_dtkeys[k as usize], kp) != 0 {
            k += 1;
        }
        match k {
            K_DIM__WIDTH => {
                error = spc_util_read_length(spe, &raw mut (*p).width, ap);
                (*p).flags |= INFO_HAS_WIDTH;
            }
            K_DIM__HEIGHT => {
                error = spc_util_read_length(spe, &raw mut (*p).height, ap);
                (*p).flags |= INFO_HAS_HEIGHT;
            }
            K_DIM__DEPTH => {
                error = spc_util_read_length(spe, &raw mut (*p).depth, ap);
                (*p).flags |= INFO_HAS_HEIGHT;
            }
            K_TRN__SCALE => {
                vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    yscale = atof(vp);
                    xscale = yscale;
                    has_scale = 1 as ::core::ffi::c_int;
                    free(vp as *mut ::core::ffi::c_void);
                }
            }
            K_TRN__XSCALE => {
                vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    xscale = atof(vp);
                    has_xscale = 1 as ::core::ffi::c_int;
                    free(vp as *mut ::core::ffi::c_void);
                }
            }
            K_TRN__YSCALE => {
                vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    yscale = atof(vp);
                    has_yscale = 1 as ::core::ffi::c_int;
                    free(vp as *mut ::core::ffi::c_void);
                }
            }
            K_TRN__ROTATE => {
                vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    rotate = M_PI * atof(vp) / 180.0f64;
                    has_rotate = 1 as ::core::ffi::c_int;
                    free(vp as *mut ::core::ffi::c_void);
                }
            }
            K_TRN__BBOX => {
                let mut v: [::core::ffi::c_double; 4] = [0.; 4];
                if spc_util_read_numbers(&raw mut v as *mut ::core::ffi::c_double, 4 as ::core::ffi::c_int, ap) != 4 as ::core::ffi::c_int {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    (*p).bbox.llx = v[0 as ::core::ffi::c_int as usize];
                    (*p).bbox.lly = v[1 as ::core::ffi::c_int as usize];
                    (*p).bbox.urx = v[2 as ::core::ffi::c_int as usize];
                    (*p).bbox.ury = v[3 as ::core::ffi::c_int as usize];
                    (*p).flags |= INFO_HAS_USER_BBOX;
                }
            }
            K_TRN__MATRIX => {
                let mut v_0: [::core::ffi::c_double; 6] = [0.; 6];
                if spc_util_read_numbers(&raw mut v_0 as *mut ::core::ffi::c_double, 6 as ::core::ffi::c_int, ap) != 6 as ::core::ffi::c_int {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    (*p).matrix.a = v_0[0 as ::core::ffi::c_int as usize];
                    (*p).matrix.b = v_0[1 as ::core::ffi::c_int as usize];
                    (*p).matrix.c = v_0[2 as ::core::ffi::c_int as usize];
                    (*p).matrix.d = v_0[3 as ::core::ffi::c_int as usize];
                    (*p).matrix.e = v_0[4 as ::core::ffi::c_int as usize];
                    (*p).matrix.f = v_0[5 as ::core::ffi::c_int as usize];
                    has_matrix = 1 as ::core::ffi::c_int;
                }
            }
            K__CLIP => {
                vp = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    if atof(vp) != 0. {
                        (*p).flags |= INFO_DO_CLIP;
                    } else {
                        (*p).flags &= !INFO_DO_CLIP;
                    }
                    free(vp as *mut ::core::ffi::c_void);
                }
            }
            K__PAGE => {
                let mut page: ::core::ffi::c_double = 0.;
                if !page_no.is_null() && spc_util_read_numbers(&raw mut page, 1 as ::core::ffi::c_int, ap) == 1 as ::core::ffi::c_int {
                    *page_no = page as ::core::ffi::c_int;
                } else {
                    error = -(1 as ::core::ffi::c_int);
                }
            }
            K__HIDE => {
                (*p).flags |= INFO_DO_HIDE;
            }
            K__PAGEBOX => {
                let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
                q = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
                if !q.is_null() {
                    if !bbox_type.is_null() {
                        if strcasecmp(q, b"cropbox\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
                            *bbox_type = pdf_page_boundary_cropbox;
                        } else if strcasecmp(q, b"mediabox\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
                            *bbox_type = pdf_page_boundary_mediabox;
                        } else if strcasecmp(q, b"artbox\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
                            *bbox_type = pdf_page_boundary_artbox;
                        } else if strcasecmp(q, b"trimbox\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
                            *bbox_type = pdf_page_boundary_trimbox;
                        } else if strcasecmp(q, b"bleedbox\0" as *const u8 as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
                            *bbox_type = pdf_page_boundary_bleedbox;
                        }
                    }
                    free(q as *mut ::core::ffi::c_void);
                } else if !bbox_type.is_null() {
                    *bbox_type = pdf_page_boundary__auto;
                }
            }
            _ => {
                error = -(1 as ::core::ffi::c_int);
            }
        }
        if error != 0 {
            pdf_warning!(
                b"Unrecognized key or invalid value for dimension/transformation: %s\0" as *const u8 as *const ::core::ffi::c_char,
                kp
            );
        } else {
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        }
        free(kp as *mut ::core::ffi::c_void);
    }
    if error == 0 {
        if has_xscale != 0 && (*p).flags & INFO_HAS_WIDTH != 0 {
            pdf_warning!(b"Can't supply both width and xscale. Ignore xscale.\0" as *const u8 as *const ::core::ffi::c_char);
            xscale = 1.0f64;
        } else if has_yscale != 0 && (*p).flags & INFO_HAS_HEIGHT != 0 {
            pdf_warning!(b"Can't supply both height/depth and yscale. Ignore yscale.\0" as *const u8 as *const ::core::ffi::c_char);
            yscale = 1.0f64;
        } else if has_scale != 0 && (has_xscale != 0 || has_yscale != 0) {
            pdf_warning!(b"Can't supply overall scale along with axis scales.\0" as *const u8 as *const ::core::ffi::c_char);
            error = -(1 as ::core::ffi::c_int);
        } else if has_matrix != 0 && (has_scale != 0 || has_xscale != 0 || has_yscale != 0 || has_rotate != 0) {
            pdf_warning!(
                b"Can't supply transform matrix along with scales or rotate. Ignore scales and rotate.\0" as *const u8 as *const ::core::ffi::c_char
            );
        }
    }
    if has_matrix == 0 {
        make_transmatrix(&raw mut (*p).matrix, 0.0f64, 0.0f64, xscale, yscale, rotate);
    }
    if (*p).flags & INFO_HAS_USER_BBOX == 0 {
        (*p).flags &= !INFO_DO_CLIP;
    }
    return error;
}
pub const K__PAGE: ::core::ffi::c_int = 11;
pub const K__PAGEBOX: ::core::ffi::c_int = 12;
const COLORDEFS: [colordef_; 69] = [
    colordef_ {
        key: b"GreenYellow\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.15f64, 0.00f64, 0.69f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Yellow\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.00f64, 1.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Goldenrod\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.10f64, 0.84f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Dandelion\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.29f64, 0.84f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Apricot\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.32f64, 0.52f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Peach\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.50f64, 0.70f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Melon\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.46f64, 0.50f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"YellowOrange\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.42f64, 1.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Orange\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.61f64, 0.87f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"BurntOrange\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.51f64, 1.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Bittersweet\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.75f64, 1.00f64, 0.24f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"RedOrange\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.77f64, 0.87f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Mahogany\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.85f64, 0.87f64, 0.35f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Maroon\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.87f64, 0.68f64, 0.32f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"BrickRed\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.89f64, 0.94f64, 0.28f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Red\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 1.00f64, 1.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"OrangeRed\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 1.00f64, 0.50f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"RubineRed\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 1.00f64, 0.13f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"WildStrawberry\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.96f64, 0.39f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Salmon\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.53f64, 0.38f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"CarnationPink\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.63f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Magenta\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 1.00f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"VioletRed\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.81f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Rhodamine\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.82f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Mulberry\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.34f64, 0.90f64, 0.00f64, 0.02f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"RedViolet\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.07f64, 0.90f64, 0.00f64, 0.34f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Fuchsia\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.47f64, 0.91f64, 0.00f64, 0.08f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Lavender\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.48f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Thistle\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.12f64, 0.59f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Orchid\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.32f64, 0.64f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"DarkOrchid\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.40f64, 0.80f64, 0.20f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Purple\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.45f64, 0.86f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Plum\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.50f64, 1.00f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Violet\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.79f64, 0.88f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"RoyalPurple\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.75f64, 0.90f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"BlueViolet\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.86f64, 0.91f64, 0.00f64, 0.04f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Periwinkle\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.57f64, 0.55f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"CadetBlue\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.62f64, 0.57f64, 0.23f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"CornflowerBlue\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.65f64, 0.13f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"MidnightBlue\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.98f64, 0.13f64, 0.00f64, 0.43f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"NavyBlue\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.94f64, 0.54f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"RoyalBlue\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                1.00f64, 0.50f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Blue\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                1.00f64, 1.00f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Cerulean\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.94f64, 0.11f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Cyan\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                1.00f64, 0.00f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"ProcessBlue\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.96f64, 0.00f64, 0.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"SkyBlue\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.62f64, 0.00f64, 0.12f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Turquoise\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.85f64, 0.00f64, 0.20f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"TealBlue\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.86f64, 0.00f64, 0.34f64, 0.02f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Aquamarine\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.82f64, 0.00f64, 0.30f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"BlueGreen\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.85f64, 0.00f64, 0.33f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Emerald\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                1.00f64, 0.00f64, 0.50f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"JungleGreen\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.99f64, 0.00f64, 0.52f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"SeaGreen\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.69f64, 0.00f64, 0.50f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Green\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                1.00f64, 0.00f64, 1.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"ForestGreen\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.91f64, 0.00f64, 0.88f64, 0.12f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"PineGreen\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.92f64, 0.00f64, 0.59f64, 0.25f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"LimeGreen\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.50f64, 0.00f64, 1.00f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"YellowGreen\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.44f64, 0.00f64, 0.74f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"SpringGreen\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.26f64, 0.00f64, 0.76f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"OliveGreen\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.64f64, 0.00f64, 0.95f64, 0.40f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"RawSienna\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.72f64, 1.00f64, 0.45f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Sepia\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.83f64, 1.00f64, 0.70f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Brown\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.00f64, 0.81f64, 1.00f64, 0.60f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Tan\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_CMYK,
            num_components: 4 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.14f64, 0.42f64, 0.56f64, 0.00f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Gray\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_GRAY,
            num_components: 1 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.5f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"Black\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_GRAY,
            num_components: 1 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                0.0f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: b"White\0" as *const u8 as *const ::core::ffi::c_char,
        color: pdf_color {
            res_id: -(1 as ::core::ffi::c_int),
            type_0: PDF_COLORSPACE_TYPE_GRAY,
            num_components: 1 as ::core::ffi::c_int,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [
                1.0f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
            ],
            pattern_id: -(1 as ::core::ffi::c_int),
        },
    },
    colordef_ {
        key: ::core::ptr::null::<::core::ffi::c_char>(),
        color: pdf_color {
            res_id: 0,
            type_0: 0,
            num_components: 0,
            spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
            values: [0.; 32],
            pattern_id: 0,
        },
    },
];
unsafe extern "C-unwind" fn pdf_color_namedcolor(mut color: *mut pdf_color, mut name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !COLORDEFS[i as usize].key.is_null() {
        if streq_ptr(COLORDEFS[i as usize].key, name) {
            pdf_color_copycolor(color, &COLORDEFS[i as usize].color as *const pdf_color);
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn memcmp(__s1: *const ::core::ffi::c_void, __s2: *const ::core::ffi::c_void, __n: size_t) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_util_read_numbers(
    mut values: *mut ::core::ffi::c_double, mut num_values: ::core::ffi::c_int, mut args: *mut spc_arg, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_util_read_numbers(values, num_values, args) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_util_read_colorspec(
    mut spe: *mut spc_env, mut colorspec: *mut pdf_color, mut ap: *mut spc_arg, mut syntax: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_util_read_colorspec(spe, colorspec, ap, syntax) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_util_read_pdfcolor(
    mut spe: *mut spc_env, mut colorspec: *mut pdf_color, mut ap: *mut spc_arg, mut defaultcolor: *mut pdf_color, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_util_read_pdfcolor(spe, colorspec, ap, defaultcolor) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_util_read_dimtrns(
    mut spe: *mut spc_env, mut ti: *mut transform_info, mut args: *mut spc_arg, mut syntax: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_util_read_dimtrns(spe, ti, args, syntax) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_util_read_blahblah(
    mut spe: *mut spc_env, mut p: *mut transform_info, mut page_no: *mut ::core::ffi::c_int, mut bbox_type: *mut pdf_page_boundary,
    mut ap: *mut spc_arg, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_util_read_blahblah(spe, p, page_no, bbox_type, ap) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
