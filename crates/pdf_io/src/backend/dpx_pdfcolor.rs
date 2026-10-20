/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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
use crate::backend::dpx_pdfdev::pdf_dev_get_param;
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcpy(__dst: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sprintf(_: *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn snprintf(__str: *mut ::core::ffi::c_char, __size: size_t, __format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_version"]
    fn pdf_get_version() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_ref_obj"]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_link_obj"]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_stream"]
    fn pdf_new_stream(flags: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_stream"]
    fn pdf_add_stream(stream: *mut pdf_obj, stream_data_ptr: *const ::core::ffi::c_void, stream_data_len: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_stream_dict"]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    static mut dpx_conf: _dpx_conf;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_sget_unsigned_pair"]
    fn sget_unsigned_pair(_: *mut ::core::ffi::c_uchar) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_pdf_dev_reset_color"]
    fn pdf_dev_reset_color(force: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_doc_add_page_resource"]
    fn pdf_doc_add_page_resource(category: *const ::core::ffi::c_char, resource_name: *const ::core::ffi::c_char, resources: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_get_resource_reference"]
    fn pdf_get_resource_reference(res_id: ::core::ffi::c_int) -> *mut pdf_obj;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
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
pub type iccSig = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iccbased_cdata {
    pub sig: int32_t,
    pub checksum: [::core::ffi::c_uchar; 16],
    pub colorspace: ::core::ffi::c_int,
    pub alternate: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub count: ::core::ffi::c_uint,
    pub capacity: ::core::ffi::c_uint,
    pub colorspaces: *mut pdf_colorspace,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_colorspace {
    pub ident: *mut ::core::ffi::c_char,
    pub subtype: ::core::ffi::c_int,
    pub resource: *mut pdf_obj,
    pub reference: *mut pdf_obj,
    pub cdata: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub keep_cache: ::core::ffi::c_int,
}
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iccHeader {
    pub size: ::core::ffi::c_int,
    pub CMMType: iccSig,
    pub version: int32_t,
    pub devClass: iccSig,
    pub colorSpace: iccSig,
    pub PCS: iccSig,
    pub creationDate: [::core::ffi::c_char; 12],
    pub acsp: iccSig,
    pub platform: iccSig,
    pub flags: [::core::ffi::c_char; 4],
    pub devMnfct: iccSig,
    pub devModel: iccSig,
    pub devAttr: [::core::ffi::c_char; 8],
    pub intent: int32_t,
    pub illuminant: iccXYZNumber,
    pub creator: iccSig,
    pub ID: [::core::ffi::c_uchar; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iccXYZNumber {
    pub X: int32_t,
    pub Y: int32_t,
    pub Z: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub current: ::core::ffi::c_int,
    pub stroke: [pdf_color; 128],
    pub fill: [pdf_color; 128],
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
pub const STREAM_COMPRESS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_CMYK: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const PDF_COLORSPACE_TYPE_RGB: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const PDF_COLORSPACE_TYPE_SPOT: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const PDF_COLORSPACE_TYPE_GRAY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const PDF_COLORSPACE_TYPE_INVALID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_DEVICEGRAY: ::core::ffi::c_int = 1;
pub const PDF_COLORSPACE_TYPE_DEVICERGB: ::core::ffi::c_int = 2;
pub const PDF_COLORSPACE_TYPE_DEVICECMYK: ::core::ffi::c_int = 3;
pub const PDF_COLORSPACE_TYPE_CALGRAY: ::core::ffi::c_int = 4;
pub const PDF_COLORSPACE_TYPE_CALRGB: ::core::ffi::c_int = 5;
pub const PDF_COLORSPACE_TYPE_LAB: ::core::ffi::c_int = 6;
pub const PDF_COLORSPACE_TYPE_ICCBASED: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_INDEXED: ::core::ffi::c_int = 10;
pub const PDF_COLORSPACE_TYPE_PATTERN: ::core::ffi::c_int = 11;
pub const PDF_DEV_PARAM_COLORMODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_pdf_color_type")]
pub unsafe extern "C-unwind" fn pdf_color_type(mut color: *const pdf_color) -> ::core::ffi::c_int {
    if color.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"color\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    return (*color).type_0;
}
#[unsafe(export_name = "__pdf_impl_pdf_color_rgbcolor")]
pub unsafe extern "C-unwind" fn pdf_color_rgbcolor(
    mut color: *mut pdf_color, mut r: ::core::ffi::c_double, mut g: ::core::ffi::c_double, mut b: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if color.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"color\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if r < 0.0f64 || r > 1.0f64 {
        pdf_warning!(b"Invalid color value specified: red=%g\0" as *const u8 as *const ::core::ffi::c_char, r,);
        return -(1 as ::core::ffi::c_int);
    }
    if g < 0.0f64 || g > 1.0f64 {
        pdf_warning!(b"Invalid color value specified: green=%g\0" as *const u8 as *const ::core::ffi::c_char, g,);
        return -(1 as ::core::ffi::c_int);
    }
    if b < 0.0f64 || b > 1.0f64 {
        pdf_warning!(b"Invalid color value specified: blue=%g\0" as *const u8 as *const ::core::ffi::c_char, b,);
        return -(1 as ::core::ffi::c_int);
    }
    (*color).values[0 as ::core::ffi::c_int as usize] = r;
    (*color).values[1 as ::core::ffi::c_int as usize] = g;
    (*color).values[2 as ::core::ffi::c_int as usize] = b;
    (*color).res_id = -(1 as ::core::ffi::c_int);
    (*color).type_0 = PDF_COLORSPACE_TYPE_RGB;
    (*color).num_components = 3 as ::core::ffi::c_int;
    (*color).spot_color_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_color_cmykcolor")]
pub unsafe extern "C-unwind" fn pdf_color_cmykcolor(
    mut color: *mut pdf_color, mut c: ::core::ffi::c_double, mut m: ::core::ffi::c_double, mut y: ::core::ffi::c_double, mut k: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if color.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"color\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if c < 0.0f64 || c > 1.0f64 {
        pdf_warning!(b"Invalid color value specified: cyan=%g\0" as *const u8 as *const ::core::ffi::c_char, c,);
        return -(1 as ::core::ffi::c_int);
    }
    if m < 0.0f64 || m > 1.0f64 {
        pdf_warning!(
            b"Invalid color value specified: magenta=%g\0" as *const u8 as *const ::core::ffi::c_char,
            m,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if y < 0.0f64 || y > 1.0f64 {
        pdf_warning!(
            b"Invalid color value specified: yellow=%g\0" as *const u8 as *const ::core::ffi::c_char,
            y,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if k < 0.0f64 || k > 1.0f64 {
        pdf_warning!(b"Invalid color value specified: black=%g\0" as *const u8 as *const ::core::ffi::c_char, k,);
        return -(1 as ::core::ffi::c_int);
    }
    (*color).values[0 as ::core::ffi::c_int as usize] = c;
    (*color).values[1 as ::core::ffi::c_int as usize] = m;
    (*color).values[2 as ::core::ffi::c_int as usize] = y;
    (*color).values[3 as ::core::ffi::c_int as usize] = k;
    (*color).res_id = -(1 as ::core::ffi::c_int);
    (*color).type_0 = PDF_COLORSPACE_TYPE_CMYK;
    (*color).num_components = 4 as ::core::ffi::c_int;
    (*color).spot_color_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_color_graycolor")]
pub unsafe extern "C-unwind" fn pdf_color_graycolor(mut color: *mut pdf_color, mut g: ::core::ffi::c_double) -> ::core::ffi::c_int {
    if color.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"color\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if g < 0.0f64 || g > 1.0f64 {
        pdf_warning!(b"Invalid color value specified: gray=%g\0" as *const u8 as *const ::core::ffi::c_char, g,);
        return -(1 as ::core::ffi::c_int);
    }
    (*color).values[0 as ::core::ffi::c_int as usize] = g;
    (*color).res_id = -(1 as ::core::ffi::c_int);
    (*color).type_0 = PDF_COLORSPACE_TYPE_GRAY;
    (*color).num_components = 1 as ::core::ffi::c_int;
    (*color).spot_color_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_color_spotcolor")]
pub unsafe extern "C-unwind" fn pdf_color_spotcolor(
    mut color: *mut pdf_color, mut name: *mut ::core::ffi::c_char, mut c: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if color.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"color\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if c < 0.0f64 || c > 1.0f64 {
        pdf_warning!(b"Invalid color value specified: grade=%g\0" as *const u8 as *const ::core::ffi::c_char, c,);
        return -(1 as ::core::ffi::c_int);
    }
    (*color).values[0 as ::core::ffi::c_int as usize] = c;
    (*color).values[1 as ::core::ffi::c_int as usize] = 0.0f64;
    (*color).res_id = 1 as ::core::ffi::c_int;
    (*color).type_0 = PDF_COLORSPACE_TYPE_SPOT;
    (*color).num_components = 2 as ::core::ffi::c_int;
    (*color).spot_color_name = name;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_color_copycolor")]
pub unsafe extern "C-unwind" fn pdf_color_copycolor(mut color1: *mut pdf_color, mut color2: *const pdf_color) {
    if !(!color1.is_null() && !color2.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"color1 && color2\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    memcpy(
        color1 as *mut ::core::ffi::c_void,
        color2 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<pdf_color>() as size_t,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_color_brighten_color")]
pub unsafe extern "C-unwind" fn pdf_color_brighten_color(mut dst: *mut pdf_color, mut src: *const pdf_color, mut f: ::core::ffi::c_double) {
    if !(!dst.is_null() && !src.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"dst && src\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*src).type_0 != PDF_COLORSPACE_TYPE_RGB && (*src).type_0 != PDF_COLORSPACE_TYPE_CMYK && (*src).type_0 != PDF_COLORSPACE_TYPE_GRAY {
        pdf_color_copycolor(dst, src);
        return;
    }
    if f == 1.0f64 {
        pdf_color_graycolor(dst, 1.0f64);
    } else {
        let mut f0: ::core::ffi::c_double = 0.;
        let mut f1: ::core::ffi::c_double = 0.;
        let mut n: ::core::ffi::c_int = 0;
        pdf_color_copycolor(dst, src);
        n = (*src).num_components;
        f1 = if n == 4 as ::core::ffi::c_int { 0.0f64 } else { f };
        f0 = 1.0f64 - f;
        loop {
            let fresh0 = n;
            n = n - 1;
            if !(fresh0 != 0) {
                break;
            }
            (*dst).values[n as usize] = f0 * (*src).values[n as usize] + f1;
        }
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_color_is_white")]
pub unsafe extern "C-unwind" fn pdf_color_is_white(mut color: *const pdf_color) -> bool {
    let mut n: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
    if color.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"color\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    match (*color).type_0 {
        PDF_COLORSPACE_TYPE_GRAY | PDF_COLORSPACE_TYPE_RGB => {
            f = 1.0f64;
        }
        PDF_COLORSPACE_TYPE_CMYK => {
            f = 0.0f64;
        }
        _ => return false_0 != 0,
    }
    n = (*color).num_components;
    loop {
        let fresh1 = n;
        n = n - 1;
        if !(fresh1 != 0) {
            break;
        }
        if (*color).values[n as usize] != f {
            return false_0 != 0;
        }
    }
    return true_0 != 0;
}
#[unsafe(export_name = "__pdf_impl_pdf_color_set_color")]
pub unsafe extern "C-unwind" fn pdf_color_set_color(
    mut color: *const pdf_color, mut buffer: *mut ::core::ffi::c_char, mut buffer_len: size_t, mut mask: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut estimate: size_t = 0 as size_t;
    if (*color).num_components > 0 as ::core::ffi::c_int {
        estimate = estimate
            .wrapping_add((5 as ::core::ffi::c_int * ((*color).num_components + 1 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int) as size_t);
    }
    estimate = estimate.wrapping_add(strlen(b" /DeiceGray CS\0" as *const u8 as *const ::core::ffi::c_char));
    if estimate.wrapping_add(1 as size_t) > buffer_len {
        pdf_warning!(b"Not enough buffer space allocated for writing set_color op...\0" as *const u8 as *const ::core::ffi::c_char,);
        return 0 as ::core::ffi::c_int;
    }
    match pdf_color_type(color) {
        PDF_COLORSPACE_TYPE_DEVICEGRAY => {
            len += sprintf(
                buffer.offset(len as isize),
                b" /DeviceGray %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                'C' as i32 | mask as ::core::ffi::c_int,
                'S' as i32 | mask as ::core::ffi::c_int,
            );
            i = 0 as ::core::ffi::c_int;
            while i < (*color).num_components {
                len += sprintf(
                    buffer.offset(len as isize),
                    b" %g\0" as *const u8 as *const ::core::ffi::c_char,
                    floor((*color).values[i as usize] / 0.001f64 + 0.5f64) * 0.001f64,
                );
                i += 1;
            }
            len += sprintf(
                buffer.offset(len as isize),
                b" %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                'S' as i32 | mask as ::core::ffi::c_int,
                'C' as i32 | mask as ::core::ffi::c_int,
            );
        }
        PDF_COLORSPACE_TYPE_DEVICERGB => {
            len += sprintf(
                buffer.offset(len as isize),
                b" /DeviceRGB %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                'C' as i32 | mask as ::core::ffi::c_int,
                'S' as i32 | mask as ::core::ffi::c_int,
            );
            i = 0 as ::core::ffi::c_int;
            while i < (*color).num_components {
                len += sprintf(
                    buffer.offset(len as isize),
                    b" %g\0" as *const u8 as *const ::core::ffi::c_char,
                    floor((*color).values[i as usize] / 0.001f64 + 0.5f64) * 0.001f64,
                );
                i += 1;
            }
            len += sprintf(
                buffer.offset(len as isize),
                b" %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                'S' as i32 | mask as ::core::ffi::c_int,
                'C' as i32 | mask as ::core::ffi::c_int,
            );
        }
        PDF_COLORSPACE_TYPE_DEVICECMYK => {
            len += sprintf(
                buffer.offset(len as isize),
                b" /DeviceCMYK %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                'C' as i32 | mask as ::core::ffi::c_int,
                'S' as i32 | mask as ::core::ffi::c_int,
            );
            i = 0 as ::core::ffi::c_int;
            while i < (*color).num_components {
                len += sprintf(
                    buffer.offset(len as isize),
                    b" %g\0" as *const u8 as *const ::core::ffi::c_char,
                    floor((*color).values[i as usize] / 0.001f64 + 0.5f64) * 0.001f64,
                );
                i += 1;
            }
            len += sprintf(
                buffer.offset(len as isize),
                b" %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                'S' as i32 | mask as ::core::ffi::c_int,
                'C' as i32 | mask as ::core::ffi::c_int,
            );
        }
        PDF_COLORSPACE_TYPE_GRAY => {
            i = 0 as ::core::ffi::c_int;
            while i < (*color).num_components {
                len += sprintf(
                    buffer.offset(len as isize),
                    b" %g\0" as *const u8 as *const ::core::ffi::c_char,
                    floor((*color).values[i as usize] / 0.001f64 + 0.5f64) * 0.001f64,
                );
                i += 1;
            }
            len += sprintf(
                buffer.offset(len as isize),
                b" %c\0" as *const u8 as *const ::core::ffi::c_char,
                'G' as i32 | mask as ::core::ffi::c_int,
            );
        }
        PDF_COLORSPACE_TYPE_RGB => {
            i = 0 as ::core::ffi::c_int;
            while i < (*color).num_components {
                len += sprintf(
                    buffer.offset(len as isize),
                    b" %g\0" as *const u8 as *const ::core::ffi::c_char,
                    floor((*color).values[i as usize] / 0.001f64 + 0.5f64) * 0.001f64,
                );
                i += 1;
            }
            len += sprintf(
                buffer.offset(len as isize),
                b" %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                'R' as i32 | mask as ::core::ffi::c_int,
                'G' as i32 | mask as ::core::ffi::c_int,
            );
        }
        PDF_COLORSPACE_TYPE_CMYK => {
            i = 0 as ::core::ffi::c_int;
            while i < (*color).num_components {
                len += sprintf(
                    buffer.offset(len as isize),
                    b" %g\0" as *const u8 as *const ::core::ffi::c_char,
                    floor((*color).values[i as usize] / 0.001f64 + 0.5f64) * 0.001f64,
                );
                i += 1;
            }
            len += sprintf(
                buffer.offset(len as isize),
                b" %c\0" as *const u8 as *const ::core::ffi::c_char,
                'K' as i32 | mask as ::core::ffi::c_int,
            );
        }
        PDF_COLORSPACE_TYPE_SPOT => {
            len = sprintf(
                buffer.offset(len as isize),
                b" /%s %c%c %g %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                (*color).spot_color_name,
                'C' as i32 | mask as ::core::ffi::c_int,
                'S' as i32 | mask as ::core::ffi::c_int,
                floor((*color).values[0 as ::core::ffi::c_int as usize] / 0.001f64 + 0.5f64) * 0.001f64,
                'S' as i32 | mask as ::core::ffi::c_int,
                'C' as i32 | mask as ::core::ffi::c_int,
            );
        }
        PDF_COLORSPACE_TYPE_CALGRAY | PDF_COLORSPACE_TYPE_CALRGB | PDF_COLORSPACE_TYPE_LAB | PDF_COLORSPACE_TYPE_INDEXED => {
            let mut res_name: [::core::ffi::c_char; 16] = [0; 16];
            snprintf(
                &raw mut res_name as *mut ::core::ffi::c_char,
                16 as size_t,
                b"XC%d\0" as *const u8 as *const ::core::ffi::c_char,
                (*color).res_id & 0xffff as ::core::ffi::c_int,
            );
            res_name[15 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            len += sprintf(
                buffer.offset(len as isize),
                b" /%s %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut res_name as *mut ::core::ffi::c_char,
                'C' as i32 | mask as ::core::ffi::c_int,
                'S' as i32 | mask as ::core::ffi::c_int,
            );
            i = 0 as ::core::ffi::c_int;
            while i < (*color).num_components {
                len += sprintf(
                    buffer.offset(len as isize),
                    b" %g\0" as *const u8 as *const ::core::ffi::c_char,
                    floor((*color).values[i as usize] / 0.001f64 + 0.5f64) * 0.001f64,
                );
                i += 1;
            }
            len += sprintf(
                buffer.offset(len as isize),
                b" %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                'S' as i32 | mask as ::core::ffi::c_int,
                'C' as i32 | mask as ::core::ffi::c_int,
            );
            pdf_doc_add_page_resource(
                b"ColorSpace\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut res_name as *mut ::core::ffi::c_char,
                pdf_get_resource_reference((*color).res_id),
            );
        }
        PDF_COLORSPACE_TYPE_PATTERN => {
            let mut res_name_0: [::core::ffi::c_char; 16] = [0; 16];
            if (*color).res_id < 0 as ::core::ffi::c_int {
                len += sprintf(
                    buffer.offset(len as isize),
                    b" /Pattern %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                    'C' as i32 | mask as ::core::ffi::c_int,
                    'S' as i32 | mask as ::core::ffi::c_int,
                );
            } else {
                snprintf(
                    &raw mut res_name_0 as *mut ::core::ffi::c_char,
                    16 as size_t,
                    b"XC%d\0" as *const u8 as *const ::core::ffi::c_char,
                    (*color).res_id & 0xffff as ::core::ffi::c_int,
                );
                res_name_0[15 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
                len += sprintf(
                    buffer.offset(len as isize),
                    b" /%s %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut res_name_0 as *mut ::core::ffi::c_char,
                    'C' as i32 | mask as ::core::ffi::c_int,
                    'S' as i32 | mask as ::core::ffi::c_int,
                );
                i = 0 as ::core::ffi::c_int;
                while i < (*color).num_components {
                    len += sprintf(
                        buffer.offset(len as isize),
                        b" %g\0" as *const u8 as *const ::core::ffi::c_char,
                        floor((*color).values[i as usize] / 0.001f64 + 0.5f64) * 0.001f64,
                    );
                    i += 1;
                }
                pdf_doc_add_page_resource(
                    b"ColorSpace\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut res_name_0 as *mut ::core::ffi::c_char,
                    pdf_get_resource_reference((*color).res_id),
                );
            }
            snprintf(
                &raw mut res_name_0 as *mut ::core::ffi::c_char,
                16 as size_t,
                b"XP%d\0" as *const u8 as *const ::core::ffi::c_char,
                (*color).pattern_id & 0xffff as ::core::ffi::c_int,
            );
            res_name_0[15 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            len += sprintf(
                buffer.offset(len as isize),
                b" /%s %c%c%c\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut res_name_0 as *mut ::core::ffi::c_char,
                'S' as i32 | mask as ::core::ffi::c_int,
                'C' as i32 | mask as ::core::ffi::c_int,
                'N' as i32 | mask as ::core::ffi::c_int,
            );
            pdf_doc_add_page_resource(
                b"Pattern\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut res_name_0 as *mut ::core::ffi::c_char,
                pdf_get_resource_reference((*color).pattern_id),
            );
        }
        _ => {
            let mut res_name_1: [::core::ffi::c_char; 16] = [0; 16];
            snprintf(
                &raw mut res_name_1 as *mut ::core::ffi::c_char,
                8 as size_t,
                b"XC%d\0" as *const u8 as *const ::core::ffi::c_char,
                (*color).res_id & 0xffff as ::core::ffi::c_int,
            );
            res_name_1[8 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
            len += sprintf(
                buffer.offset(len as isize),
                b" /%s %c%c\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut res_name_1 as *mut ::core::ffi::c_char,
                'C' as i32 | mask as ::core::ffi::c_int,
                'S' as i32 | mask as ::core::ffi::c_int,
            );
            i = 0 as ::core::ffi::c_int;
            while i < (*color).num_components {
                len += sprintf(
                    buffer.offset(len as isize),
                    b" %g\0" as *const u8 as *const ::core::ffi::c_char,
                    floor((*color).values[i as usize] / 0.001f64 + 0.5f64) * 0.001f64,
                );
                i += 1;
            }
            len += sprintf(
                buffer.offset(len as isize),
                b" %c%c%c\0" as *const u8 as *const ::core::ffi::c_char,
                'S' as i32 | mask as ::core::ffi::c_int,
                'C' as i32 | mask as ::core::ffi::c_int,
                'N' as i32 | mask as ::core::ffi::c_int,
            );
            pdf_doc_add_page_resource(
                b"ColorSpace\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut res_name_1 as *mut ::core::ffi::c_char,
                pdf_get_resource_reference((*color).res_id),
            );
        }
    }
    return len;
}

#[unsafe(export_name = "__pdf_impl_pdf_color_compare")]
pub unsafe extern "C-unwind" fn pdf_color_compare(mut color1: *const pdf_color, mut color2: *const pdf_color) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    if (*color1).type_0 != PDF_COLORSPACE_TYPE_GRAY
        && (*color1).type_0 != PDF_COLORSPACE_TYPE_RGB
        && (*color1).type_0 != PDF_COLORSPACE_TYPE_CMYK
        && (*color1).type_0 != PDF_COLORSPACE_TYPE_SPOT
        || (*color2).type_0 != PDF_COLORSPACE_TYPE_GRAY
            && (*color2).type_0 != PDF_COLORSPACE_TYPE_RGB
            && (*color2).type_0 != PDF_COLORSPACE_TYPE_CMYK
            && (*color2).type_0 != PDF_COLORSPACE_TYPE_SPOT
        || (*color1).type_0 != (*color2).type_0
    {
        return -(1 as ::core::ffi::c_int);
    }
    n = (*color1).num_components;
    loop {
        n -= 1;
        if !(n >= 0 as ::core::ffi::c_int) {
            break;
        }
        if (*color1).values[n as usize] != (*color2).values[n as usize] {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if !(*color1).spot_color_name.is_null() && !(*color2).spot_color_name.is_null() {
        return strcmp((*color1).spot_color_name, (*color2).spot_color_name);
    }
    return 0 as ::core::ffi::c_int;
}

pub const DEV_COLOR_STACK_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;

#[unsafe(export_name = "__pdf_impl_pdf_color_clear_stack")]
pub unsafe extern "C-unwind" fn pdf_color_clear_stack() {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).color_stack.current > 0 as ::core::ffi::c_int {
        pdf_warning!(b"You've mistakenly made a global color change within nested colors.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    loop {
        let fresh2 = (*__pdf_state).color_stack.current;
        (*__pdf_state).color_stack.current = (*__pdf_state).color_stack.current - 1;
        if !(fresh2 != 0) {
            break;
        }
        free((*__pdf_state).color_stack.stroke[(*__pdf_state).color_stack.current as usize].spot_color_name as *mut ::core::ffi::c_void);
        free((*__pdf_state).color_stack.fill[(*__pdf_state).color_stack.current as usize].spot_color_name as *mut ::core::ffi::c_void);
    }
    (*__pdf_state).color_stack.current = 0 as ::core::ffi::c_int;
    pdf_color_graycolor(&raw mut (*__pdf_state).color_stack.stroke as *mut pdf_color, 0.0f64);
    pdf_color_graycolor(&raw mut (*__pdf_state).color_stack.fill as *mut pdf_color, 0.0f64);
}
#[unsafe(export_name = "__pdf_impl_pdf_color_set")]
pub unsafe extern "C-unwind" fn pdf_color_set(mut sc: *mut pdf_color, mut fc: *mut pdf_color) {
    let __pdf_state = unsafe { document_state() };
    pdf_color_copycolor(
        (&raw mut (*__pdf_state).color_stack.stroke as *mut pdf_color).offset((*__pdf_state).color_stack.current as isize) as *mut pdf_color,
        sc,
    );
    pdf_color_copycolor(
        (&raw mut (*__pdf_state).color_stack.fill as *mut pdf_color).offset((*__pdf_state).color_stack.current as isize) as *mut pdf_color,
        fc,
    );
    pdf_dev_reset_color(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_pdf_color_push")]
pub unsafe extern "C-unwind" fn pdf_color_push(mut sc: *mut pdf_color, mut fc: *mut pdf_color) {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).color_stack.current >= DEV_COLOR_STACK_MAX - 1 as ::core::ffi::c_int {
        pdf_warning!(b"Color stack overflow. Just ignore.\0" as *const u8 as *const ::core::ffi::c_char,);
    } else {
        (*__pdf_state).color_stack.current += 1;
        pdf_color_set(sc, fc);
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_color_pop")]
pub unsafe extern "C-unwind" fn pdf_color_pop() {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).color_stack.current <= 0 as ::core::ffi::c_int {
        pdf_warning!(b"Color stack underflow. Just ignore.\0" as *const u8 as *const ::core::ffi::c_char,);
    } else {
        (*__pdf_state).color_stack.current -= 1;
        pdf_dev_reset_color(1 as ::core::ffi::c_int);
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_color_get_current")]
pub unsafe extern "C-unwind" fn pdf_color_get_current(mut sc: *mut *mut pdf_color, mut fc: *mut *mut pdf_color) {
    let __pdf_state = unsafe { document_state() };
    *sc = (&raw mut (*__pdf_state).color_stack.stroke as *mut pdf_color).offset((*__pdf_state).color_stack.current as isize) as *mut pdf_color;
    *fc = (&raw mut (*__pdf_state).color_stack.fill as *mut pdf_color).offset((*__pdf_state).color_stack.current as isize) as *mut pdf_color;
}
const NULLBYTES16: [::core::ffi::c_uchar; 16] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
const ICC_VERSIONS: [C2RustUnnamed_1; 11] = [
    C2RustUnnamed_1 {
        major: 0 as ::core::ffi::c_int,
        minor: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_1 {
        major: 0 as ::core::ffi::c_int,
        minor: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_1 {
        major: 0 as ::core::ffi::c_int,
        minor: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_1 {
        major: 0x2 as ::core::ffi::c_int,
        minor: 0x10 as ::core::ffi::c_int,
    },
    C2RustUnnamed_1 {
        major: 0x2 as ::core::ffi::c_int,
        minor: 0x20 as ::core::ffi::c_int,
    },
    C2RustUnnamed_1 {
        major: 0x4 as ::core::ffi::c_int,
        minor: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_1 {
        major: 0x4 as ::core::ffi::c_int,
        minor: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_1 {
        major: 0x4 as ::core::ffi::c_int,
        minor: 0x20 as ::core::ffi::c_int,
    },
    C2RustUnnamed_1 {
        major: 0x4 as ::core::ffi::c_int,
        minor: 0x20 as ::core::ffi::c_int,
    },
    C2RustUnnamed_1 {
        major: 0x4 as ::core::ffi::c_int,
        minor: 0x20 as ::core::ffi::c_int,
    },
    C2RustUnnamed_1 {
        major: 0x4 as ::core::ffi::c_int,
        minor: 0x20 as ::core::ffi::c_int,
    },
];
fn iccp_version_supported(major: i32, minor: i32, pdf_version: i32) -> i32 {
    let index = (pdf_version - 10) as usize;
    i32::from(
        ICC_VERSIONS
            .get(index)
            .is_some_and(|version| (major, minor) <= (version.major, version.minor)),
    )
}

const fn str2iccSig(bytes: [u8; 4]) -> iccSig {
    u32::from_be_bytes(bytes)
}
pub const iccNullSig: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
fn iccp_init_iccHeader(icch: &mut iccHeader) {
    icch.size = 0 as ::core::ffi::c_int;
    icch.CMMType = iccNullSig as iccSig;
    icch.version = 0xffffff as ::core::ffi::c_int as int32_t;
    icch.devClass = iccNullSig as iccSig;
    icch.colorSpace = iccNullSig as iccSig;
    icch.PCS = iccNullSig as iccSig;
    icch.creationDate = [0; 12];
    icch.acsp = str2iccSig(*b"ascp");
    icch.platform = iccNullSig as iccSig;
    icch.flags = [0; 4];
    icch.devMnfct = iccNullSig as iccSig;
    icch.devModel = iccNullSig as iccSig;
    icch.devAttr = [0; 8];
    icch.intent = 0 as ::core::ffi::c_int as int32_t;
    icch.illuminant.X = 0 as ::core::ffi::c_int as int32_t;
    icch.illuminant.Y = 0 as ::core::ffi::c_int as int32_t;
    icch.illuminant.Z = 0 as ::core::ffi::c_int as int32_t;
    icch.creator = iccNullSig as iccSig;
    icch.ID = [0; 16];
}
pub const ICC_INTENT_PERCEPTUAL: ::core::ffi::c_int = 0;
pub const ICC_INTENT_RELATIVE: ::core::ffi::c_int = 1;
pub const ICC_INTENT_SATURATION: ::core::ffi::c_int = 2;
pub const ICC_INTENT_ABSOLUTE: ::core::ffi::c_int = 3;
unsafe extern "C-unwind" fn init_iccbased_cdata(mut cdata: *mut iccbased_cdata) {
    if cdata.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"cdata\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    (*cdata).sig =
        (('i' as i32) << 24 as ::core::ffi::c_int | ('c' as i32) << 16 as ::core::ffi::c_int | ('c' as i32) << 8 as ::core::ffi::c_int | 'b' as i32)
            as int32_t;
    memset(
        &raw mut (*cdata).checksum as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        16 as size_t,
    );
    (*cdata).colorspace = PDF_COLORSPACE_TYPE_INVALID;
    (*cdata).alternate = -(1 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn release_iccbased_cdata(mut cdata: *mut iccbased_cdata) {
    if !(!cdata.is_null()
        && (*cdata).sig
            == ('i' as i32) << 24 as ::core::ffi::c_int
                | ('c' as i32) << 16 as ::core::ffi::c_int
                | ('c' as i32) << 8 as ::core::ffi::c_int
                | 'b' as i32) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"check_sig(cdata, 'i', 'c', 'c', 'b')\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    free(cdata as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn get_num_components_iccbased(mut cdata: *const iccbased_cdata) -> ::core::ffi::c_int {
    let mut num_components: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(!cdata.is_null()
        && (*cdata).sig
            == ('i' as i32) << 24 as ::core::ffi::c_int
                | ('c' as i32) << 16 as ::core::ffi::c_int
                | ('c' as i32) << 8 as ::core::ffi::c_int
                | 'b' as i32) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"check_sig(cdata, 'i', 'c', 'c', 'b')\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match (*cdata).colorspace {
        PDF_COLORSPACE_TYPE_RGB => {
            num_components = 3 as ::core::ffi::c_int;
        }
        PDF_COLORSPACE_TYPE_CMYK => {
            num_components = 4 as ::core::ffi::c_int;
        }
        PDF_COLORSPACE_TYPE_GRAY => {
            num_components = 1 as ::core::ffi::c_int;
        }
        PDF_COLORSPACE_TYPE_LAB => {
            num_components = 3 as ::core::ffi::c_int;
        }
        _ => {}
    }
    return num_components;
}
unsafe extern "C-unwind" fn compare_iccbased(
    mut ident1: *const ::core::ffi::c_char, mut cdata1: *const iccbased_cdata, mut ident2: *const ::core::ffi::c_char,
    mut cdata2: *const iccbased_cdata,
) -> ::core::ffi::c_int {
    if !cdata1.is_null() && !cdata2.is_null() {
        if !(!cdata1.is_null()
            && (*cdata1).sig
                == ('i' as i32) << 24 as ::core::ffi::c_int
                    | ('c' as i32) << 16 as ::core::ffi::c_int
                    | ('c' as i32) << 8 as ::core::ffi::c_int
                    | 'b' as i32) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            pdf_abort!(
                c"assertion failed: %s".as_ptr(),
                b"check_sig(cdata1, 'i', 'c', 'c', 'b')\0" as *const u8 as *const ::core::ffi::c_char
            );
        } else {
        };
        if !(!cdata2.is_null()
            && (*cdata2).sig
                == ('i' as i32) << 24 as ::core::ffi::c_int
                    | ('c' as i32) << 16 as ::core::ffi::c_int
                    | ('c' as i32) << 8 as ::core::ffi::c_int
                    | 'b' as i32) as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            pdf_abort!(
                c"assertion failed: %s".as_ptr(),
                b"check_sig(cdata2, 'i', 'c', 'c', 'b')\0" as *const u8 as *const ::core::ffi::c_char
            );
        } else {
        };
        if memcmp(
            &raw const (*cdata1).checksum as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            NULLBYTES16.as_ptr() as *const ::core::ffi::c_void,
            16 as size_t,
        ) != 0
            && memcmp(
                &raw const (*cdata2).checksum as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                NULLBYTES16.as_ptr() as *const ::core::ffi::c_void,
                16 as size_t,
            ) != 0
        {
            return memcmp(
                &raw const (*cdata1).checksum as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                &raw const (*cdata2).checksum as *const ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                16 as size_t,
            );
        }
        if (*cdata1).colorspace != (*cdata2).colorspace {
            return (*cdata1).colorspace - (*cdata2).colorspace;
        }
    }
    if !ident1.is_null() && !ident2.is_null() {
        return strcmp(ident1, ident2);
    }
    return -(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_iccp_check_colorspace")]
pub unsafe extern "C-unwind" fn iccp_check_colorspace(colortype: i32, profile: *const ::core::ffi::c_void, proflen: i32) -> i32 {
    if profile.is_null() || proflen < 128 {
        return -1;
    }
    let header = unsafe { &*profile.cast::<[u8; 128]>() };
    let colorspace = u32::from_be_bytes(header[16..20].try_into().unwrap());
    let expected = match colortype {
        PDF_COLORSPACE_TYPE_CALRGB | PDF_COLORSPACE_TYPE_RGB => str2iccSig(*b"RGB "),
        PDF_COLORSPACE_TYPE_CALGRAY | PDF_COLORSPACE_TYPE_GRAY => str2iccSig(*b"GRAY"),
        PDF_COLORSPACE_TYPE_CMYK => str2iccSig(*b"CMYK"),
        _ => return -1,
    };
    if colorspace == expected { 0 } else { -1 }
}
#[unsafe(export_name = "__pdf_impl_iccp_get_rendering_intent")]
pub unsafe extern "C-unwind" fn iccp_get_rendering_intent(profile: *const ::core::ffi::c_void, proflen: i32) -> *mut pdf_obj {
    if profile.is_null() || proflen < 128 {
        return core::ptr::null_mut();
    }
    let header = unsafe { &*profile.cast::<[u8; 128]>() };
    let intent = (u32::from_be_bytes(header[64..68].try_into().unwrap()) >> 16) & 0xff;
    let name = match intent as i32 {
        ICC_INTENT_SATURATION => c"Saturation",
        ICC_INTENT_PERCEPTUAL => c"Perceptual",
        ICC_INTENT_ABSOLUTE => c"AbsoluteColorimetric",
        ICC_INTENT_RELATIVE => c"RelativeColorimetric",
        _ => {
            pdf_warning!("Invalid rendering intent type: {intent}");
            return core::ptr::null_mut();
        }
    };
    unsafe { pdf_new_name(name.as_ptr()) }
}
fn iccp_unpack_header(icch: &mut iccHeader, profile: &[u8]) -> i32 {
    if profile.len() < 128 || profile.len() % 4 != 0 {
        pdf_warning!("Profile size: {}", profile.len());
        return -1;
    }
    let word = |offset| u32::from_be_bytes(profile[offset..offset + 4].try_into().unwrap());
    icch.size = word(0) as i32;
    if icch.size as usize != profile.len() {
        pdf_warning!("ICC Profile size: {}(header) != {}", icch.size, profile.len());
        return -1;
    }
    icch.CMMType = word(4);
    icch.version = word(8) as i32;
    icch.devClass = word(12);
    icch.colorSpace = word(16);
    icch.PCS = word(20);
    icch.creationDate = core::array::from_fn(|i| profile[24 + i] as ::core::ffi::c_char);
    icch.acsp = word(36);
    if icch.acsp != str2iccSig(*b"acsp") {
        pdf_warning!(
            b"Invalid ICC profile: not \"acsp\" - %c%c%c%c \0".as_ptr().cast(),
            profile[36] as i32,
            profile[37] as i32,
            profile[38] as i32,
            profile[39] as i32,
        );
        return -1;
    }
    icch.platform = word(40);
    icch.flags = core::array::from_fn(|i| profile[44 + i] as ::core::ffi::c_char);
    icch.devMnfct = word(48);
    icch.devModel = word(52);
    icch.devAttr = core::array::from_fn(|i| profile[56 + i] as ::core::ffi::c_char);
    icch.intent = word(64) as i32;
    icch.illuminant = iccXYZNumber {
        X: word(68) as i32,
        Y: word(72) as i32,
        Z: word(76) as i32,
    };
    icch.creator = word(80);
    icch.ID.copy_from_slice(&profile[84..100]);
    for (offset, &byte) in profile[100..128].iter().enumerate() {
        if byte != 0 {
            pdf_warning!("Reserved pad not zero: {byte:02x} (at offset {} in ICC profile header.)", offset + 100,);
            return -1;
        }
    }
    0
}
pub const ICC_HEAD_SECT1_START: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ICC_HEAD_SECT1_LENGTH: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const ICC_HEAD_SECT2_START: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const ICC_HEAD_SECT2_LENGTH: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ICC_HEAD_SECT3_START: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const ICC_HEAD_SECT3_LENGTH: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
fn iccp_get_checksum(profile: &[u8]) -> [u8; 16] {
    use md5::{Digest, Md5};
    let mut digest = Md5::new();
    digest.update(&profile[..56]);
    digest.update([0; 12]);
    digest.update(&profile[68..84]);
    digest.update([0; 16]);
    digest.update(&profile[100..]);
    digest.finalize().into()
}
fn print_icc_signature(label: &str, signature: iccSig) {
    let bytes = signature.to_be_bytes();
    let value = if signature == 0 {
        "(null)"
    } else if bytes.iter().all(|byte| (32..=126).contains(byte)) {
        core::str::from_utf8(&bytes).unwrap()
    } else {
        "(invalid)"
    };
    pdf_message!("pdf_color>> {label}:\t{value}\n");
}

fn print_icc_bytes(bytes: &[u8]) {
    for (index, byte) in bytes.iter().enumerate() {
        let separator = if index == 0 { "" } else { ":" };
        pdf_message!("{separator}{byte:02x}");
    }
}

fn print_iccp_header(icch: &iccHeader, checksum: Option<&[u8; 16]>) {
    pdf_message!("\npdf_color>> ICC Profile Info\n");
    pdf_message!("pdf_color>> Profile Size:\t{} bytes\n", icch.size);
    print_icc_signature("CMM Type", icch.CMMType);
    pdf_message!(
        "pdf_color>> Profile Version:\t{}.{}.{}\n",
        (icch.version >> 24) & 0xff,
        (icch.version >> 20) & 0xf,
        (icch.version >> 16) & 0xf,
    );
    print_icc_signature("Device Class", icch.devClass);
    print_icc_signature("Color Space", icch.colorSpace);
    print_icc_signature("Connection Space", icch.PCS);
    pdf_message!("pdf_color>> Creation Date:\t");
    for (index, pair) in icch.creationDate.chunks_exact(2).enumerate() {
        let value = u16::from_be_bytes([pair[0] as u8, pair[1] as u8]);
        if index == 0 {
            pdf_message!("{value:04}");
        } else {
            pdf_message!(":{value:02}");
        }
    }
    pdf_message!("\n");
    print_icc_signature("Primary Platform", icch.platform);
    pdf_message!("pdf_color>> Profile Flags:\t");
    print_icc_bytes(&icch.flags.map(|byte| byte as u8));
    pdf_message!("\n");
    print_icc_signature("Device Mnfct", icch.devMnfct);
    print_icc_signature("Device Model", icch.devModel);
    pdf_message!("pdf_color>> Device Attr:\t");
    print_icc_bytes(&icch.devAttr.map(|byte| byte as u8));
    let intent = match (icch.intent >> 16) & 0xff {
        ICC_INTENT_SATURATION => "Saturation",
        ICC_INTENT_PERCEPTUAL => "Perceptual",
        ICC_INTENT_ABSOLUTE => "Absolute Colorimetric",
        ICC_INTENT_RELATIVE => "Relative Colorimetric",
        _ => "(invalid)",
    };
    pdf_message!("\npdf_color>> Rendering Intent:\t{intent}\n");
    print_icc_signature("Creator", icch.creator);
    pdf_message!(
        "pdf_color>> Illuminant (XYZ):\t{:.3} {:.3} {:.3}\n",
        icch.illuminant.X as f64 / 65536.0,
        icch.illuminant.Y as f64 / 65536.0,
        icch.illuminant.Z as f64 / 65536.0,
    );
    pdf_message!("pdf_color>> Checksum:\t");
    if icch.ID == [0; 16] {
        pdf_message!("(null)");
    } else {
        print_icc_bytes(&icch.ID);
    }
    pdf_message!("\n");
    if let Some(checksum) = checksum {
        pdf_message!("pdf_color>> Calculated:\t");
        print_icc_bytes(checksum);
        pdf_message!("\n");
    }
}
fn iccp_devClass_allowed(dev_class: iccSig) -> i32 {
    i32::from(
        [*b"scnr", *b"mntr", *b"prtr", *b"spac"]
            .iter()
            .any(|signature| dev_class == str2iccSig(*signature)),
    )
}
#[unsafe(export_name = "__pdf_impl_iccp_load_profile")]
pub unsafe extern "C-unwind" fn iccp_load_profile(
    mut ident: *const ::core::ffi::c_char, mut profile: *const ::core::ffi::c_void, mut proflen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cspc_id: ::core::ffi::c_int = 0;
    let mut resource: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut icch: iccHeader = iccHeader {
        size: 0,
        CMMType: 0,
        version: 0,
        devClass: 0,
        colorSpace: 0,
        PCS: 0,
        creationDate: [0; 12],
        acsp: 0,
        platform: 0,
        flags: [0; 4],
        devMnfct: 0,
        devModel: 0,
        devAttr: [0; 8],
        intent: 0,
        illuminant: iccXYZNumber { X: 0, Y: 0, Z: 0 },
        creator: 0,
        ID: [0; 16],
    };
    let mut colorspace: ::core::ffi::c_int = 0;
    let mut checksum: [::core::ffi::c_uchar; 16] = [0; 16];
    let mut cdata: *mut iccbased_cdata = ::core::ptr::null_mut::<iccbased_cdata>();
    iccp_init_iccHeader(&mut icch);
    let profile_bytes = if profile.is_null() || proflen <= 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(profile.cast::<u8>(), proflen as usize) }
    };
    if iccp_unpack_header(&mut icch, profile_bytes) < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Invalid ICC profile header in \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
        print_iccp_header(&icch, None);
        return -(1 as ::core::ffi::c_int);
    }
    if iccp_version_supported(
        icch.version as ::core::ffi::c_int >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
        icch.version as ::core::ffi::c_int >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
        pdf_get_version(),
    ) == 0
    {
        pdf_warning!(
            b"ICC profile format spec. version %d.%01d.%01d not supported in current PDF version setting.\0" as *const u8
                as *const ::core::ffi::c_char,
            icch.version >> 24 as ::core::ffi::c_int & 0xff as int32_t,
            icch.version >> 20 as ::core::ffi::c_int & 0xf as int32_t,
            icch.version >> 16 as ::core::ffi::c_int & 0xf as int32_t,
        );
        pdf_warning!(b"ICC profile not embedded.\0" as *const u8 as *const ::core::ffi::c_char,);
        print_iccp_header(&icch, None);
        return -(1 as ::core::ffi::c_int);
    }
    if iccp_devClass_allowed(icch.devClass) == 0 {
        pdf_warning!(b"Unsupported ICC Profile Device Class:\0" as *const u8 as *const ::core::ffi::c_char,);
        print_iccp_header(&icch, None);
        return -(1 as ::core::ffi::c_int);
    }
    if icch.colorSpace == str2iccSig(*b"RGB ") {
        colorspace = PDF_COLORSPACE_TYPE_RGB;
    } else if icch.colorSpace == str2iccSig(*b"GRAY") {
        colorspace = PDF_COLORSPACE_TYPE_GRAY;
    } else if icch.colorSpace == str2iccSig(*b"CMYK") {
        colorspace = PDF_COLORSPACE_TYPE_CMYK;
    } else {
        pdf_warning!(b"Unsupported input color space.\0" as *const u8 as *const ::core::ffi::c_char,);
        print_iccp_header(&icch, None);
        return -(1 as ::core::ffi::c_int);
    }
    checksum = iccp_get_checksum(profile_bytes);
    if icch.ID != [0; 16] && icch.ID != checksum {
        pdf_warning!(b"Invalid ICC profile: Inconsistent checksum.\0" as *const u8 as *const ::core::ffi::c_char,);
        print_iccp_header(&icch, Some(&checksum));
        return -(1 as ::core::ffi::c_int);
    }
    cdata = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<iccbased_cdata>() as usize) as uint32_t)
        as *mut iccbased_cdata;
    init_iccbased_cdata(cdata);
    (*cdata).colorspace = colorspace;
    memcpy(
        &raw mut (*cdata).checksum as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut checksum as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        16 as size_t,
    );
    cspc_id = pdf_colorspace_findresource(ident, PDF_COLORSPACE_TYPE_ICCBASED, cdata as *const ::core::ffi::c_void);
    if cspc_id >= 0 as ::core::ffi::c_int {
        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
            pdf_message!(b"(ICCP:[id=%d])\0" as *const u8 as *const ::core::ffi::c_char, cspc_id,);
        }
        release_iccbased_cdata(cdata);
        return cspc_id;
    }
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        print_iccp_header(&icch, Some(&checksum));
    }
    resource = pdf_new_array();
    stream = pdf_new_stream(STREAM_COMPRESS);
    pdf_add_array(resource, pdf_new_name(b"ICCBased\0" as *const u8 as *const ::core::ffi::c_char));
    pdf_add_array(resource, pdf_ref_obj(stream));
    stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"N\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(get_num_components_iccbased(cdata) as ::core::ffi::c_double),
    );
    pdf_add_stream(stream, profile, proflen);
    pdf_release_obj(stream);
    cspc_id = pdf_colorspace_defineresource(ident, PDF_COLORSPACE_TYPE_ICCBASED, cdata as *mut ::core::ffi::c_void, resource);
    return cspc_id;
}

unsafe extern "C-unwind" fn pdf_colorspace_findresource(
    mut ident: *const ::core::ffi::c_char, mut type_0: ::core::ffi::c_int, mut cdata: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut colorspace: *mut pdf_colorspace = ::core::ptr::null_mut::<pdf_colorspace>();
    let mut cspc_id: ::core::ffi::c_int = 0;
    let mut cmp: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    cspc_id = 0 as ::core::ffi::c_int;
    while cmp != 0 && (cspc_id as ::core::ffi::c_uint) < (*__pdf_state).cspc_cache.count {
        colorspace = (*__pdf_state).cspc_cache.colorspaces.offset(cspc_id as isize) as *mut pdf_colorspace;
        if !((*colorspace).subtype != type_0) {
            match (*colorspace).subtype {
                PDF_COLORSPACE_TYPE_ICCBASED => {
                    cmp = compare_iccbased(
                        ident,
                        cdata as *const iccbased_cdata,
                        (*colorspace).ident,
                        (*colorspace).cdata as *const iccbased_cdata,
                    );
                }
                _ => {}
            }
            if cmp == 0 {
                return cspc_id;
            }
        }
        cspc_id += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn pdf_init_colorspace_struct(mut colorspace: *mut pdf_colorspace) {
    if colorspace.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"colorspace\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*colorspace).ident = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*colorspace).subtype = PDF_COLORSPACE_TYPE_INVALID;
    (*colorspace).resource = ::core::ptr::null_mut::<pdf_obj>();
    (*colorspace).reference = ::core::ptr::null_mut::<pdf_obj>();
    (*colorspace).cdata = NULL;
}
unsafe extern "C-unwind" fn pdf_clean_colorspace_struct(mut colorspace: *mut pdf_colorspace) {
    if colorspace.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"colorspace\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    free((*colorspace).ident as *mut ::core::ffi::c_void);
    pdf_release_obj((*colorspace).resource);
    pdf_release_obj((*colorspace).reference);
    (*colorspace).resource = ::core::ptr::null_mut::<pdf_obj>();
    (*colorspace).reference = ::core::ptr::null_mut::<pdf_obj>();
    if !(*colorspace).cdata.is_null() {
        match (*colorspace).subtype {
            PDF_COLORSPACE_TYPE_ICCBASED => {
                release_iccbased_cdata((*colorspace).cdata as *mut iccbased_cdata);
            }
            _ => {}
        }
    }
    (*colorspace).cdata = NULL;
    (*colorspace).subtype = PDF_COLORSPACE_TYPE_INVALID;
}
unsafe extern "C-unwind" fn pdf_flush_colorspace(mut colorspace: *mut pdf_colorspace) {
    if colorspace.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"colorspace\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_release_obj((*colorspace).resource);
    pdf_release_obj((*colorspace).reference);
    (*colorspace).resource = ::core::ptr::null_mut::<pdf_obj>();
    (*colorspace).reference = ::core::ptr::null_mut::<pdf_obj>();
}
unsafe extern "C-unwind" fn pdf_colorspace_defineresource(
    mut ident: *const ::core::ffi::c_char, mut subtype: ::core::ffi::c_int, mut cdata: *mut ::core::ffi::c_void, mut resource: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut cspc_id: ::core::ffi::c_int = 0;
    let mut colorspace: *mut pdf_colorspace = ::core::ptr::null_mut::<pdf_colorspace>();
    if (*__pdf_state).cspc_cache.count >= (*__pdf_state).cspc_cache.capacity {
        (*__pdf_state).cspc_cache.capacity = (*__pdf_state).cspc_cache.capacity.wrapping_add(16 as ::core::ffi::c_uint);
        (*__pdf_state).cspc_cache.colorspaces = renew(
            (*__pdf_state).cspc_cache.colorspaces as *mut ::core::ffi::c_void,
            ((*__pdf_state).cspc_cache.capacity as uint32_t as usize).wrapping_mul(::core::mem::size_of::<pdf_colorspace>() as usize) as uint32_t,
        ) as *mut pdf_colorspace;
    }
    cspc_id = (*__pdf_state).cspc_cache.count as ::core::ffi::c_int;
    colorspace = (*__pdf_state).cspc_cache.colorspaces.offset(cspc_id as isize) as *mut pdf_colorspace;
    pdf_init_colorspace_struct(colorspace);
    if !ident.is_null() {
        (*colorspace).ident = new((strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
            as *mut ::core::ffi::c_char;
        strcpy((*colorspace).ident, ident);
    }
    (*colorspace).subtype = subtype;
    (*colorspace).cdata = cdata;
    (*colorspace).resource = resource;
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(b"(ColorSpace:%s\0" as *const u8 as *const ::core::ffi::c_char, ident,);
        if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
            match subtype {
                PDF_COLORSPACE_TYPE_ICCBASED => {
                    pdf_message!(b"[ICCBased]\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                PDF_COLORSPACE_TYPE_CALRGB => {
                    pdf_message!(b"[CalRGB]\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                PDF_COLORSPACE_TYPE_CALGRAY => {
                    pdf_message!(b"[CalGray]\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                _ => {}
            }
        }
        pdf_message!(b")\0" as *const u8 as *const ::core::ffi::c_char);
    }
    (*__pdf_state).cspc_cache.count = (*__pdf_state).cspc_cache.count.wrapping_add(1);
    return cspc_id;
}
#[unsafe(export_name = "__pdf_impl_pdf_get_colorspace_reference")]
pub unsafe extern "C-unwind" fn pdf_get_colorspace_reference(mut cspc_id: ::core::ffi::c_int) -> *mut pdf_obj {
    let __pdf_state = unsafe { document_state() };
    let mut colorspace: *mut pdf_colorspace = ::core::ptr::null_mut::<pdf_colorspace>();
    colorspace = (*__pdf_state).cspc_cache.colorspaces.offset(cspc_id as isize) as *mut pdf_colorspace;
    if (*colorspace).reference.is_null() {
        (*colorspace).reference = pdf_ref_obj((*colorspace).resource);
        pdf_release_obj((*colorspace).resource);
        (*colorspace).resource = ::core::ptr::null_mut::<pdf_obj>();
    }
    return pdf_link_obj((*colorspace).reference);
}
#[unsafe(export_name = "__pdf_impl_pdf_init_colors")]
pub unsafe extern "C-unwind" fn pdf_init_colors() {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).cspc_cache.count = 0 as ::core::ffi::c_uint;
    (*__pdf_state).cspc_cache.capacity = 0 as ::core::ffi::c_uint;
    (*__pdf_state).cspc_cache.colorspaces = ::core::ptr::null_mut::<pdf_colorspace>();
}
#[unsafe(export_name = "__pdf_impl_pdf_close_colors")]
pub unsafe extern "C-unwind" fn pdf_close_colors() {
    let __pdf_state = unsafe { document_state() };
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (*__pdf_state).cspc_cache.count {
        let mut colorspace: *mut pdf_colorspace = ::core::ptr::null_mut::<pdf_colorspace>();
        colorspace = (*__pdf_state).cspc_cache.colorspaces.offset(i as isize) as *mut pdf_colorspace;
        pdf_flush_colorspace(colorspace);
        pdf_clean_colorspace_struct(colorspace);
        i = i.wrapping_add(1);
    }
    (*__pdf_state).cspc_cache.colorspaces = mfree((*__pdf_state).cspc_cache.colorspaces as *mut ::core::ffi::c_void) as *mut pdf_colorspace;
    (*__pdf_state).cspc_cache.capacity = 0 as ::core::ffi::c_uint;
    (*__pdf_state).cspc_cache.count = (*__pdf_state).cspc_cache.capacity;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn memcmp(__s1: *const ::core::ffi::c_void, __s2: *const ::core::ffi::c_void, __n: size_t) -> ::core::ffi::c_int;
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
    fn memset(__b: *mut ::core::ffi::c_void, __c: ::core::ffi::c_int, __len: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_color_type(mut color: *const pdf_color, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_type(color) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_rgbcolor(
    mut color: *mut pdf_color, mut r: ::core::ffi::c_double, mut g: ::core::ffi::c_double, mut b: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_rgbcolor(color, r, g, b) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_cmykcolor(
    mut color: *mut pdf_color, mut c: ::core::ffi::c_double, mut m: ::core::ffi::c_double, mut y: ::core::ffi::c_double,
    mut k: ::core::ffi::c_double, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_cmykcolor(color, c, m, y, k) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_graycolor(
    mut color: *mut pdf_color, mut g: ::core::ffi::c_double, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_graycolor(color, g) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_spotcolor(
    mut color: *mut pdf_color, mut name: *mut ::core::ffi::c_char, mut c: ::core::ffi::c_double, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_spotcolor(color, name, c) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_copycolor(mut color1: *mut pdf_color, mut color2: *const pdf_color) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_copycolor(color1, color2) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_color_brighten_color(
    mut dst: *mut pdf_color, mut src: *const pdf_color, mut f: ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_brighten_color(dst, src, f) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_color_is_white(mut color: *const pdf_color, _result: *mut bool) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_is_white(color) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_set_color(
    mut color: *const pdf_color, mut buffer: *mut ::core::ffi::c_char, mut buffer_len: size_t, mut mask: ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_set_color(color, buffer, buffer_len, mask) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_compare(
    mut color1: *const pdf_color, mut color2: *const pdf_color, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_compare(color1, color2) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_clear_stack() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_clear_stack() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_color_set(mut sc: *mut pdf_color, mut fc: *mut pdf_color) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_set(sc, fc) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_color_push(mut sc: *mut pdf_color, mut fc: *mut pdf_color) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_push(sc, fc) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_color_pop() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_pop() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_color_get_current(mut sc: *mut *mut pdf_color, mut fc: *mut *mut pdf_color) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_get_current(sc, fc) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_iccp_check_colorspace(
    mut colortype: ::core::ffi::c_int, mut profile: *const ::core::ffi::c_void, mut proflen: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { iccp_check_colorspace(colortype, profile, proflen) }) {
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
pub unsafe extern "C" fn __pdf_entry_iccp_get_rendering_intent(
    mut profile: *const ::core::ffi::c_void, mut proflen: ::core::ffi::c_int, _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { iccp_get_rendering_intent(profile, proflen) }) {
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
pub unsafe extern "C" fn __pdf_entry_iccp_load_profile(
    mut ident: *const ::core::ffi::c_char, mut profile: *const ::core::ffi::c_void, mut proflen: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { iccp_load_profile(ident, profile, proflen) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_colorspace_reference(mut cspc_id: ::core::ffi::c_int, _result: *mut *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_colorspace_reference(cspc_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_init_colors() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_init_colors() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_close_colors() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_close_colors() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}

pub(crate) struct DocumentState {
    current_fill: pdf_color,
    current_stroke: pdf_color,
    default_color: pdf_color,
    color_stack: C2RustUnnamed_2,
    cspc_cache: C2RustUnnamed,
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self {
            current_fill: pdf_color {
                res_id: -(1 as ::core::ffi::c_int),
                type_0: PDF_COLORSPACE_TYPE_GRAY,
                num_components: 1 as ::core::ffi::c_int,
                spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                values: [
                    0.0f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                    0.,
                ],
                pattern_id: -(1 as ::core::ffi::c_int),
            },
            current_stroke: pdf_color {
                res_id: -(1 as ::core::ffi::c_int),
                type_0: PDF_COLORSPACE_TYPE_GRAY,
                num_components: 1 as ::core::ffi::c_int,
                spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                values: [
                    0.0f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                    0.,
                ],
                pattern_id: -(1 as ::core::ffi::c_int),
            },
            default_color: pdf_color {
                res_id: -(1 as ::core::ffi::c_int),
                type_0: PDF_COLORSPACE_TYPE_GRAY,
                num_components: 1 as ::core::ffi::c_int,
                spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                values: [
                    0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
                    0., 0., 0., 0., 0., 0., 0., 0.,
                ],
                pattern_id: -(1 as ::core::ffi::c_int),
            },
            color_stack: C2RustUnnamed_2 {
                current: 0,
                stroke: [pdf_color {
                    res_id: 0,
                    type_0: 0,
                    num_components: 0,
                    spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    values: [0.; 32],
                    pattern_id: 0,
                }; 128],
                fill: [pdf_color {
                    res_id: 0,
                    type_0: 0,
                    num_components: 0,
                    spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    values: [0.; 32],
                    pattern_id: 0,
                }; 128],
            },
            cspc_cache: C2RustUnnamed {
                count: 0 as ::core::ffi::c_uint,
                capacity: 0 as ::core::ffi::c_uint,
                colorspaces: ::core::ptr::null_mut::<pdf_colorspace>(),
            },
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe { (*crate::compat::document::current()).backend.dpx_pdfcolor.pointer }
}
