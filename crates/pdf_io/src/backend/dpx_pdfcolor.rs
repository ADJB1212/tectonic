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
#![allow(
    warnings,
    clippy::all,
    unsafe_op_in_unsafe_fn,
    static_mut_refs,
    dangerous_implicit_autorefs
)]
use crate::backend::dpx_pdfdev::pdf_dev_get_param;
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn isprint(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    fn pdf_doc_add_page_resource(
        category: *const ::core::ffi::c_char,
        resource_name: *const ::core::ffi::c_char,
        resources: *mut pdf_obj,
    );
    #[link_name = "__pdf_impl_pdf_get_resource_reference"]
    fn pdf_get_resource_reference(res_id: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_MD5_init"]
    fn MD5_init(ctx: *mut MD5_CONTEXT);
    #[link_name = "__pdf_impl_MD5_write"]
    fn MD5_write(
        ctx: *mut MD5_CONTEXT,
        inbuf: *const ::core::ffi::c_uchar,
        inlen: ::core::ffi::c_uint,
    );
    #[link_name = "__pdf_impl_MD5_final"]
    fn MD5_final(outbuf: *mut ::core::ffi::c_uchar, ctx: *mut MD5_CONTEXT);
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
pub struct MD5_CONTEXT {
    pub A: uint32_t,
    pub B: uint32_t,
    pub C: uint32_t,
    pub D: uint32_t,
    pub nblocks: size_t,
    pub buf: [::core::ffi::c_uchar; 64],
    pub count: ::core::ffi::c_int,
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
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
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
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"color\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*color).type_0;
}
#[unsafe(export_name = "__pdf_impl_pdf_color_rgbcolor")]
pub unsafe extern "C-unwind" fn pdf_color_rgbcolor(
    mut color: *mut pdf_color,
    mut r: ::core::ffi::c_double,
    mut g: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if color.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"color\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if r < 0.0f64 || r > 1.0f64 {
        pdf_warning!(
            b"Invalid color value specified: red=%g\0" as *const u8 as *const ::core::ffi::c_char,
            r,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if g < 0.0f64 || g > 1.0f64 {
        pdf_warning!(
            b"Invalid color value specified: green=%g\0" as *const u8 as *const ::core::ffi::c_char,
            g,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if b < 0.0f64 || b > 1.0f64 {
        pdf_warning!(
            b"Invalid color value specified: blue=%g\0" as *const u8 as *const ::core::ffi::c_char,
            b,
        );
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
    mut color: *mut pdf_color,
    mut c: ::core::ffi::c_double,
    mut m: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut k: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if color.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"color\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if c < 0.0f64 || c > 1.0f64 {
        pdf_warning!(
            b"Invalid color value specified: cyan=%g\0" as *const u8 as *const ::core::ffi::c_char,
            c,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if m < 0.0f64 || m > 1.0f64 {
        pdf_warning!(
            b"Invalid color value specified: magenta=%g\0" as *const u8
                as *const ::core::ffi::c_char,
            m,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if y < 0.0f64 || y > 1.0f64 {
        pdf_warning!(
            b"Invalid color value specified: yellow=%g\0" as *const u8
                as *const ::core::ffi::c_char,
            y,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if k < 0.0f64 || k > 1.0f64 {
        pdf_warning!(
            b"Invalid color value specified: black=%g\0" as *const u8 as *const ::core::ffi::c_char,
            k,
        );
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
pub unsafe extern "C-unwind" fn pdf_color_graycolor(
    mut color: *mut pdf_color,
    mut g: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if color.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"color\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if g < 0.0f64 || g > 1.0f64 {
        pdf_warning!(
            b"Invalid color value specified: gray=%g\0" as *const u8 as *const ::core::ffi::c_char,
            g,
        );
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
    mut color: *mut pdf_color,
    mut name: *mut ::core::ffi::c_char,
    mut c: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if color.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"color\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if c < 0.0f64 || c > 1.0f64 {
        pdf_warning!(
            b"Invalid color value specified: grade=%g\0" as *const u8 as *const ::core::ffi::c_char,
            c,
        );
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
pub unsafe extern "C-unwind" fn pdf_color_copycolor(
    mut color1: *mut pdf_color,
    mut color2: *const pdf_color,
) {
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
pub unsafe extern "C-unwind" fn pdf_color_brighten_color(
    mut dst: *mut pdf_color,
    mut src: *const pdf_color,
    mut f: ::core::ffi::c_double,
) {
    if !(!dst.is_null() && !src.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"dst && src\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*src).type_0 != PDF_COLORSPACE_TYPE_RGB
        && (*src).type_0 != PDF_COLORSPACE_TYPE_CMYK
        && (*src).type_0 != PDF_COLORSPACE_TYPE_GRAY
    {
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
        f1 = if n == 4 as ::core::ffi::c_int {
            0.0f64
        } else {
            f
        };
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
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"color\0" as *const u8 as *const ::core::ffi::c_char
        );
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
    mut color: *const pdf_color,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_len: size_t,
    mut mask: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut estimate: size_t = 0 as size_t;
    if (*color).num_components > 0 as ::core::ffi::c_int {
        estimate = estimate.wrapping_add(
            (5 as ::core::ffi::c_int * ((*color).num_components + 1 as ::core::ffi::c_int)
                + 4 as ::core::ffi::c_int) as size_t,
        );
    }
    estimate = estimate.wrapping_add(strlen(
        b" /DeiceGray CS\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    if estimate.wrapping_add(1 as size_t) > buffer_len {
        pdf_warning!(
            b"Not enough buffer space allocated for writing set_color op...\0" as *const u8
                as *const ::core::ffi::c_char,
        );
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
                floor((*color).values[0 as ::core::ffi::c_int as usize] / 0.001f64 + 0.5f64)
                    * 0.001f64,
                'S' as i32 | mask as ::core::ffi::c_int,
                'C' as i32 | mask as ::core::ffi::c_int,
            );
        }
        PDF_COLORSPACE_TYPE_CALGRAY
        | PDF_COLORSPACE_TYPE_CALRGB
        | PDF_COLORSPACE_TYPE_LAB
        | PDF_COLORSPACE_TYPE_INDEXED => {
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
#[unsafe(no_mangle)]
pub static mut current_fill: pdf_color = pdf_color {
    res_id: -(1 as ::core::ffi::c_int),
    type_0: PDF_COLORSPACE_TYPE_GRAY,
    num_components: 1 as ::core::ffi::c_int,
    spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    values: [
        0.0f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    ],
    pattern_id: -(1 as ::core::ffi::c_int),
};
#[unsafe(no_mangle)]
pub static mut current_stroke: pdf_color = pdf_color {
    res_id: -(1 as ::core::ffi::c_int),
    type_0: PDF_COLORSPACE_TYPE_GRAY,
    num_components: 1 as ::core::ffi::c_int,
    spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    values: [
        0.0f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    ],
    pattern_id: -(1 as ::core::ffi::c_int),
};
#[unsafe(export_name = "__pdf_impl_pdf_color_compare")]
pub unsafe extern "C-unwind" fn pdf_color_compare(
    mut color1: *const pdf_color,
    mut color2: *const pdf_color,
) -> ::core::ffi::c_int {
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
#[unsafe(no_mangle)]
pub static mut default_color: pdf_color = pdf_color {
    res_id: -(1 as ::core::ffi::c_int),
    type_0: PDF_COLORSPACE_TYPE_GRAY,
    num_components: 1 as ::core::ffi::c_int,
    spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    values: [
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    ],
    pattern_id: -(1 as ::core::ffi::c_int),
};
pub const DEV_COLOR_STACK_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
static mut color_stack: C2RustUnnamed_2 = C2RustUnnamed_2 {
    current: 0,
    stroke: [pdf_color {
        res_id: 0,
        type_0: 0,
        num_components: 0,
        spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        values: [0.; 32],
        pattern_id: 0,
    }; 128],
    fill: [pdf_color {
        res_id: 0,
        type_0: 0,
        num_components: 0,
        spot_color_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        values: [0.; 32],
        pattern_id: 0,
    }; 128],
};
#[unsafe(export_name = "__pdf_impl_pdf_color_clear_stack")]
pub unsafe extern "C-unwind" fn pdf_color_clear_stack() {
    if color_stack.current > 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"You've mistakenly made a global color change within nested colors.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    loop {
        let fresh2 = color_stack.current;
        color_stack.current = color_stack.current - 1;
        if !(fresh2 != 0) {
            break;
        }
        free(
            color_stack.stroke[color_stack.current as usize].spot_color_name
                as *mut ::core::ffi::c_void,
        );
        free(
            color_stack.fill[color_stack.current as usize].spot_color_name
                as *mut ::core::ffi::c_void,
        );
    }
    color_stack.current = 0 as ::core::ffi::c_int;
    pdf_color_graycolor(&raw mut color_stack.stroke as *mut pdf_color, 0.0f64);
    pdf_color_graycolor(&raw mut color_stack.fill as *mut pdf_color, 0.0f64);
}
#[unsafe(export_name = "__pdf_impl_pdf_color_set")]
pub unsafe extern "C-unwind" fn pdf_color_set(mut sc: *mut pdf_color, mut fc: *mut pdf_color) {
    pdf_color_copycolor(
        (&raw mut color_stack.stroke as *mut pdf_color).offset(color_stack.current as isize)
            as *mut pdf_color,
        sc,
    );
    pdf_color_copycolor(
        (&raw mut color_stack.fill as *mut pdf_color).offset(color_stack.current as isize)
            as *mut pdf_color,
        fc,
    );
    pdf_dev_reset_color(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_pdf_color_push")]
pub unsafe extern "C-unwind" fn pdf_color_push(mut sc: *mut pdf_color, mut fc: *mut pdf_color) {
    if color_stack.current >= DEV_COLOR_STACK_MAX - 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"Color stack overflow. Just ignore.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        color_stack.current += 1;
        pdf_color_set(sc, fc);
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_color_pop")]
pub unsafe extern "C-unwind" fn pdf_color_pop() {
    if color_stack.current <= 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Color stack underflow. Just ignore.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        color_stack.current -= 1;
        pdf_dev_reset_color(1 as ::core::ffi::c_int);
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_color_get_current")]
pub unsafe extern "C-unwind" fn pdf_color_get_current(
    mut sc: *mut *mut pdf_color,
    mut fc: *mut *mut pdf_color,
) {
    *sc = (&raw mut color_stack.stroke as *mut pdf_color).offset(color_stack.current as isize)
        as *mut pdf_color;
    *fc = (&raw mut color_stack.fill as *mut pdf_color).offset(color_stack.current as isize)
        as *mut pdf_color;
}
static mut nullbytes16: [::core::ffi::c_uchar; 16] = [
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
static mut icc_versions: [C2RustUnnamed_1; 11] = [
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
unsafe extern "C-unwind" fn iccp_version_supported(
    mut major: ::core::ffi::c_int,
    mut minor: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut idx: ::core::ffi::c_int = 0;
    idx = pdf_get_version() - 10 as ::core::ffi::c_int;
    if idx < 11 as ::core::ffi::c_int {
        if icc_versions[idx as usize].major < major {
            return 0 as ::core::ffi::c_int;
        } else if icc_versions[idx as usize].major == major
            && icc_versions[idx as usize].minor < minor
        {
            return 0 as ::core::ffi::c_int;
        } else {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn str2iccSig(mut s: *const ::core::ffi::c_void) -> iccSig {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = s as *const ::core::ffi::c_char;
    return ((*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 24 as ::core::ffi::c_int
        | (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int
        | (*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int) as iccSig;
}
pub const iccNullSig: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn iccp_init_iccHeader(mut icch: *mut iccHeader) {
    if icch.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"icch\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*icch).size = 0 as ::core::ffi::c_int;
    (*icch).CMMType = iccNullSig as iccSig;
    (*icch).version = 0xffffff as ::core::ffi::c_int as int32_t;
    (*icch).devClass = iccNullSig as iccSig;
    (*icch).colorSpace = iccNullSig as iccSig;
    (*icch).PCS = iccNullSig as iccSig;
    memset(
        &raw mut (*icch).creationDate as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        12 as size_t,
    );
    (*icch).acsp = str2iccSig(
        b"ascp\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
    );
    (*icch).platform = iccNullSig as iccSig;
    memset(
        &raw mut (*icch).flags as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        4 as size_t,
    );
    (*icch).devMnfct = iccNullSig as iccSig;
    (*icch).devModel = iccNullSig as iccSig;
    memset(
        &raw mut (*icch).devAttr as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        8 as size_t,
    );
    (*icch).intent = 0 as ::core::ffi::c_int as int32_t;
    (*icch).illuminant.X = 0 as ::core::ffi::c_int as int32_t;
    (*icch).illuminant.Y = 0 as ::core::ffi::c_int as int32_t;
    (*icch).illuminant.Z = 0 as ::core::ffi::c_int as int32_t;
    (*icch).creator = iccNullSig as iccSig;
    memset(
        &raw mut (*icch).ID as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        16 as size_t,
    );
}
pub const ICC_INTENT_PERCEPTUAL: ::core::ffi::c_int = 0;
pub const ICC_INTENT_RELATIVE: ::core::ffi::c_int = 1;
pub const ICC_INTENT_SATURATION: ::core::ffi::c_int = 2;
pub const ICC_INTENT_ABSOLUTE: ::core::ffi::c_int = 3;
unsafe extern "C-unwind" fn init_iccbased_cdata(mut cdata: *mut iccbased_cdata) {
    if cdata.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cdata\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*cdata).sig = (('i' as i32) << 24 as ::core::ffi::c_int
        | ('c' as i32) << 16 as ::core::ffi::c_int
        | ('c' as i32) << 8 as ::core::ffi::c_int
        | 'b' as i32) as int32_t;
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
unsafe extern "C-unwind" fn get_num_components_iccbased(
    mut cdata: *const iccbased_cdata,
) -> ::core::ffi::c_int {
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
    mut ident1: *const ::core::ffi::c_char,
    mut cdata1: *const iccbased_cdata,
    mut ident2: *const ::core::ffi::c_char,
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
                b"check_sig(cdata1, 'i', 'c', 'c', 'b')\0" as *const u8
                    as *const ::core::ffi::c_char
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
                b"check_sig(cdata2, 'i', 'c', 'c', 'b')\0" as *const u8
                    as *const ::core::ffi::c_char
            );
        } else {
        };
        if memcmp(
            &raw const (*cdata1).checksum as *const ::core::ffi::c_uchar
                as *const ::core::ffi::c_void,
            &raw mut nullbytes16 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            16 as size_t,
        ) != 0
            && memcmp(
                &raw const (*cdata2).checksum as *const ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                &raw mut nullbytes16 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                16 as size_t,
            ) != 0
        {
            return memcmp(
                &raw const (*cdata1).checksum as *const ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
                &raw const (*cdata2).checksum as *const ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
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
pub unsafe extern "C-unwind" fn iccp_check_colorspace(
    mut colortype: ::core::ffi::c_int,
    mut profile: *const ::core::ffi::c_void,
    mut proflen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut colorspace: iccSig = 0;
    let mut p: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    if profile.is_null() || proflen < 128 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    p = profile as *const ::core::ffi::c_uchar;
    colorspace =
        str2iccSig(p.offset(16 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void);
    match colortype {
        PDF_COLORSPACE_TYPE_CALRGB | PDF_COLORSPACE_TYPE_RGB => {
            if colorspace
                != str2iccSig(
                    b"RGB \0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                )
            {
                return -(1 as ::core::ffi::c_int);
            }
        }
        PDF_COLORSPACE_TYPE_CALGRAY | PDF_COLORSPACE_TYPE_GRAY => {
            if colorspace
                != str2iccSig(
                    b"GRAY\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                )
            {
                return -(1 as ::core::ffi::c_int);
            }
        }
        PDF_COLORSPACE_TYPE_CMYK => {
            if colorspace
                != str2iccSig(
                    b"CMYK\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                )
            {
                return -(1 as ::core::ffi::c_int);
            }
        }
        _ => return -(1 as ::core::ffi::c_int),
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_iccp_get_rendering_intent")]
pub unsafe extern "C-unwind" fn iccp_get_rendering_intent(
    mut profile: *const ::core::ffi::c_void,
    mut proflen: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut ri: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut p: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut intent: int32_t = 0;
    if profile.is_null() || proflen < 128 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    p = profile as *const ::core::ffi::c_uchar;
    intent = ((*p.offset(64 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 24 as ::core::ffi::c_int
        | (*p.offset(65 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int
        | (*p.offset(66 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | *p.offset(67 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as int32_t;
    match (intent >> 16 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_int {
        ICC_INTENT_SATURATION => {
            ri = pdf_new_name(b"Saturation\0" as *const u8 as *const ::core::ffi::c_char);
        }
        ICC_INTENT_PERCEPTUAL => {
            ri = pdf_new_name(b"Perceptual\0" as *const u8 as *const ::core::ffi::c_char);
        }
        ICC_INTENT_ABSOLUTE => {
            ri = pdf_new_name(b"AbsoluteColorimetric\0" as *const u8 as *const ::core::ffi::c_char);
        }
        ICC_INTENT_RELATIVE => {
            ri = pdf_new_name(b"RelativeColorimetric\0" as *const u8 as *const ::core::ffi::c_char);
        }
        _ => {
            pdf_warning!(
                b"Invalid rendering intent type: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (intent >> 16 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_int,
            );
            ri = ::core::ptr::null_mut::<pdf_obj>();
        }
    }
    return ri;
}
unsafe extern "C-unwind" fn iccp_unpack_header(
    mut icch: *mut iccHeader,
    mut profile: *const ::core::ffi::c_void,
    mut proflen: ::core::ffi::c_int,
    mut check_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut endptr: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    if check_size != 0 {
        if profile.is_null()
            || proflen < 128 as ::core::ffi::c_int
            || proflen % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            pdf_warning!(
                b"Profile size: %d\0" as *const u8 as *const ::core::ffi::c_char,
                proflen,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    p = profile as *const ::core::ffi::c_uchar;
    endptr = p.offset(128 as ::core::ffi::c_int as isize);
    (*icch).size = ((*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 24 as ::core::ffi::c_int
        | (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int
        | (*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as int32_t as ::core::ffi::c_int;
    if check_size != 0 {
        if (*icch).size != proflen {
            pdf_warning!(
                b"ICC Profile size: %d(header) != %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*icch).size,
                proflen,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).CMMType = str2iccSig(p as *const ::core::ffi::c_void);
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).version = ((*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 24 as ::core::ffi::c_int
        | (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int
        | (*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as int32_t;
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).devClass = str2iccSig(p as *const ::core::ffi::c_void);
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).colorSpace = str2iccSig(p as *const ::core::ffi::c_void);
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).PCS = str2iccSig(p as *const ::core::ffi::c_void);
    p = p.offset(4 as ::core::ffi::c_int as isize);
    memcpy(
        &raw mut (*icch).creationDate as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        12 as size_t,
    );
    p = p.offset(12 as ::core::ffi::c_int as isize);
    (*icch).acsp = str2iccSig(p as *const ::core::ffi::c_void);
    if (*icch).acsp
        != str2iccSig(
            b"acsp\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        )
    {
        pdf_warning!(
            b"Invalid ICC profile: not \"acsp\" - %c%c%c%c \0" as *const u8
                as *const ::core::ffi::c_char,
            *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    }
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).platform = str2iccSig(p as *const ::core::ffi::c_void);
    p = p.offset(4 as ::core::ffi::c_int as isize);
    memcpy(
        &raw mut (*icch).flags as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        4 as size_t,
    );
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).devMnfct = str2iccSig(p as *const ::core::ffi::c_void);
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).devModel = str2iccSig(p as *const ::core::ffi::c_void);
    p = p.offset(4 as ::core::ffi::c_int as isize);
    memcpy(
        &raw mut (*icch).devAttr as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        8 as size_t,
    );
    p = p.offset(8 as ::core::ffi::c_int as isize);
    (*icch).intent = ((*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 24 as ::core::ffi::c_int
        | (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int
        | (*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as int32_t;
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).illuminant.X = ((*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 24 as ::core::ffi::c_int
        | (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int
        | (*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as int32_t;
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).illuminant.Y = ((*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 24 as ::core::ffi::c_int
        | (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int
        | (*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as int32_t;
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).illuminant.Z = ((*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 24 as ::core::ffi::c_int
        | (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int
        | (*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as int32_t;
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*icch).creator = str2iccSig(p as *const ::core::ffi::c_void);
    p = p.offset(4 as ::core::ffi::c_int as isize);
    memcpy(
        &raw mut (*icch).ID as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        16 as size_t,
    );
    p = p.offset(16 as ::core::ffi::c_int as isize);
    while p < endptr {
        if *p as ::core::ffi::c_int != '\0' as i32 {
            pdf_warning!(
                b"Reserved pad not zero: %02x (at offset %d in ICC profile header.)\0" as *const u8
                    as *const ::core::ffi::c_char,
                *p as ::core::ffi::c_int,
                128 as ::core::ffi::c_int
                    - endptr.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int,
            );
            return -(1 as ::core::ffi::c_int);
        }
        p = p.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
pub const ICC_HEAD_SECT1_START: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ICC_HEAD_SECT1_LENGTH: ::core::ffi::c_int = 56 as ::core::ffi::c_int;
pub const ICC_HEAD_SECT2_START: ::core::ffi::c_int = 68 as ::core::ffi::c_int;
pub const ICC_HEAD_SECT2_LENGTH: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ICC_HEAD_SECT3_START: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const ICC_HEAD_SECT3_LENGTH: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn iccp_get_checksum(
    mut checksum: *mut ::core::ffi::c_uchar,
    mut profile: *const ::core::ffi::c_void,
    mut proflen: ::core::ffi::c_int,
) {
    let mut p: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    p = profile as *const ::core::ffi::c_uchar;
    MD5_init(&raw mut md5);
    MD5_write(
        &raw mut md5,
        p.offset(ICC_HEAD_SECT1_START as isize),
        ICC_HEAD_SECT1_LENGTH as ::core::ffi::c_uint,
    );
    MD5_write(
        &raw mut md5,
        &raw mut nullbytes16 as *mut ::core::ffi::c_uchar,
        12 as ::core::ffi::c_uint,
    );
    MD5_write(
        &raw mut md5,
        p.offset(ICC_HEAD_SECT2_START as isize),
        ICC_HEAD_SECT2_LENGTH as ::core::ffi::c_uint,
    );
    MD5_write(
        &raw mut md5,
        &raw mut nullbytes16 as *mut ::core::ffi::c_uchar,
        16 as ::core::ffi::c_uint,
    );
    MD5_write(
        &raw mut md5,
        p.offset(ICC_HEAD_SECT3_START as isize),
        ICC_HEAD_SECT3_LENGTH as ::core::ffi::c_uint,
    );
    MD5_write(
        &raw mut md5,
        p.offset(128 as ::core::ffi::c_int as isize),
        (proflen - 128 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    MD5_final(checksum, &raw mut md5);
}
unsafe extern "C-unwind" fn print_iccp_header(
    mut icch: *mut iccHeader,
    mut checksum: *mut ::core::ffi::c_uchar,
) {
    let mut i: ::core::ffi::c_int = 0;
    if icch.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"icch\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdf_message!(b"pdf_color>> ICC Profile Info\n\0" as *const u8 as *const ::core::ffi::c_char,);
    pdf_message!(
        b"pdf_color>> Profile Size:\t%d bytes\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*icch).size,
    );
    if (*icch).CMMType == 0 as iccSig {
        pdf_message!(
            b"pdf_color>> %s:\t(null)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"CMM Type\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if isprint(
        ((*icch).CMMType >> 24 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
    ) == 0
        || isprint(
            ((*icch).CMMType >> 16 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(
            ((*icch).CMMType >> 8 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(((*icch).CMMType & 0xff as iccSig) as ::core::ffi::c_int) == 0
    {
        pdf_message!(
            b"pdf_color>> %s:\t(invalid)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"CMM Type\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        pdf_message!(
            b"pdf_color>> %s:\t%c%c%c%c\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"CMM Type\0" as *const u8 as *const ::core::ffi::c_char,
            (*icch).CMMType >> 24 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).CMMType >> 16 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).CMMType >> 8 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).CMMType & 0xff as iccSig,
        );
    }
    pdf_message!(
        b"pdf_color>> Profile Version:\t%d.%01d.%01d\n\0" as *const u8
            as *const ::core::ffi::c_char,
        (*icch).version >> 24 as ::core::ffi::c_int & 0xff as int32_t,
        (*icch).version >> 20 as ::core::ffi::c_int & 0xf as int32_t,
        (*icch).version >> 16 as ::core::ffi::c_int & 0xf as int32_t,
    );
    if (*icch).devClass == 0 as iccSig {
        pdf_message!(
            b"pdf_color>> %s:\t(null)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Device Class\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if isprint(
        ((*icch).devClass >> 24 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
    ) == 0
        || isprint(
            ((*icch).devClass >> 16 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(
            ((*icch).devClass >> 8 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(((*icch).devClass & 0xff as iccSig) as ::core::ffi::c_int) == 0
    {
        pdf_message!(
            b"pdf_color>> %s:\t(invalid)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Device Class\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        pdf_message!(
            b"pdf_color>> %s:\t%c%c%c%c\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Device Class\0" as *const u8 as *const ::core::ffi::c_char,
            (*icch).devClass >> 24 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).devClass >> 16 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).devClass >> 8 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).devClass & 0xff as iccSig,
        );
    }
    if (*icch).colorSpace == 0 as iccSig {
        pdf_message!(
            b"pdf_color>> %s:\t(null)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Color Space\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if isprint(
        ((*icch).colorSpace >> 24 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
    ) == 0
        || isprint(
            ((*icch).colorSpace >> 16 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(
            ((*icch).colorSpace >> 8 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(((*icch).colorSpace & 0xff as iccSig) as ::core::ffi::c_int) == 0
    {
        pdf_message!(
            b"pdf_color>> %s:\t(invalid)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Color Space\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        pdf_message!(
            b"pdf_color>> %s:\t%c%c%c%c\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Color Space\0" as *const u8 as *const ::core::ffi::c_char,
            (*icch).colorSpace >> 24 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).colorSpace >> 16 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).colorSpace >> 8 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).colorSpace & 0xff as iccSig,
        );
    }
    if (*icch).PCS == 0 as iccSig {
        pdf_message!(
            b"pdf_color>> %s:\t(null)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Connection Space\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if isprint(
        ((*icch).PCS >> 24 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
    ) == 0
        || isprint(((*icch).PCS >> 16 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int)
            == 0
        || isprint(((*icch).PCS >> 8 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int)
            == 0
        || isprint(((*icch).PCS & 0xff as iccSig) as ::core::ffi::c_int) == 0
    {
        pdf_message!(
            b"pdf_color>> %s:\t(invalid)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Connection Space\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        pdf_message!(
            b"pdf_color>> %s:\t%c%c%c%c\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Connection Space\0" as *const u8 as *const ::core::ffi::c_char,
            (*icch).PCS >> 24 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).PCS >> 16 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).PCS >> 8 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).PCS & 0xff as iccSig,
        );
    }
    pdf_message!(b"pdf_color>> Creation Date:\t\0" as *const u8 as *const ::core::ffi::c_char,);
    i = 0 as ::core::ffi::c_int;
    while i < 12 as ::core::ffi::c_int {
        if i == 0 as ::core::ffi::c_int {
            pdf_message!(
                b"%04u\0" as *const u8 as *const ::core::ffi::c_char,
                sget_unsigned_pair(
                    &raw mut (*icch).creationDate as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                ) as ::core::ffi::c_int,
            );
        } else {
            pdf_message!(
                b":%02u\0" as *const u8 as *const ::core::ffi::c_char,
                sget_unsigned_pair(
                    (&raw mut (*icch).creationDate as *mut ::core::ffi::c_char).offset(i as isize)
                        as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_uchar,
                ) as ::core::ffi::c_int,
            );
        }
        i += 2 as ::core::ffi::c_int;
    }
    pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    if (*icch).platform == 0 as iccSig {
        pdf_message!(
            b"pdf_color>> %s:\t(null)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Primary Platform\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if isprint(
        ((*icch).platform >> 24 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
    ) == 0
        || isprint(
            ((*icch).platform >> 16 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(
            ((*icch).platform >> 8 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(((*icch).platform & 0xff as iccSig) as ::core::ffi::c_int) == 0
    {
        pdf_message!(
            b"pdf_color>> %s:\t(invalid)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Primary Platform\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        pdf_message!(
            b"pdf_color>> %s:\t%c%c%c%c\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Primary Platform\0" as *const u8 as *const ::core::ffi::c_char,
            (*icch).platform >> 24 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).platform >> 16 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).platform >> 8 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).platform & 0xff as iccSig,
        );
    }
    pdf_message!(
        b"pdf_color>> Profile Flags:\t%02x:%02x:%02x:%02x\n\0" as *const u8
            as *const ::core::ffi::c_char,
        (*icch).flags[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*icch).flags[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*icch).flags[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*icch).flags[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
    );
    if (*icch).devMnfct == 0 as iccSig {
        pdf_message!(
            b"pdf_color>> %s:\t(null)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Device Mnfct\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if isprint(
        ((*icch).devMnfct >> 24 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
    ) == 0
        || isprint(
            ((*icch).devMnfct >> 16 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(
            ((*icch).devMnfct >> 8 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(((*icch).devMnfct & 0xff as iccSig) as ::core::ffi::c_int) == 0
    {
        pdf_message!(
            b"pdf_color>> %s:\t(invalid)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Device Mnfct\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        pdf_message!(
            b"pdf_color>> %s:\t%c%c%c%c\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Device Mnfct\0" as *const u8 as *const ::core::ffi::c_char,
            (*icch).devMnfct >> 24 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).devMnfct >> 16 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).devMnfct >> 8 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).devMnfct & 0xff as iccSig,
        );
    }
    if (*icch).devModel == 0 as iccSig {
        pdf_message!(
            b"pdf_color>> %s:\t(null)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Device Model\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if isprint(
        ((*icch).devModel >> 24 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
    ) == 0
        || isprint(
            ((*icch).devModel >> 16 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(
            ((*icch).devModel >> 8 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(((*icch).devModel & 0xff as iccSig) as ::core::ffi::c_int) == 0
    {
        pdf_message!(
            b"pdf_color>> %s:\t(invalid)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Device Model\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        pdf_message!(
            b"pdf_color>> %s:\t%c%c%c%c\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Device Model\0" as *const u8 as *const ::core::ffi::c_char,
            (*icch).devModel >> 24 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).devModel >> 16 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).devModel >> 8 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).devModel & 0xff as iccSig,
        );
    }
    pdf_message!(b"pdf_color>> Device Attr:\t\0" as *const u8 as *const ::core::ffi::c_char,);
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        if i == 0 as ::core::ffi::c_int {
            pdf_message!(
                b"%02x\0" as *const u8 as *const ::core::ffi::c_char,
                (*icch).devAttr[i as usize] as ::core::ffi::c_int,
            );
        } else {
            pdf_message!(
                b":%02x\0" as *const u8 as *const ::core::ffi::c_char,
                (*icch).devAttr[i as usize] as ::core::ffi::c_int,
            );
        }
        i += 1;
    }
    pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdf_message!(b"pdf_color>> Rendering Intent:\t\0" as *const u8 as *const ::core::ffi::c_char,);
    match ((*icch).intent >> 16 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_int {
        ICC_INTENT_SATURATION => {
            pdf_message!(b"Saturation\0" as *const u8 as *const ::core::ffi::c_char);
        }
        ICC_INTENT_PERCEPTUAL => {
            pdf_message!(b"Perceptual\0" as *const u8 as *const ::core::ffi::c_char);
        }
        ICC_INTENT_ABSOLUTE => {
            pdf_message!(b"Absolute Colorimetric\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        ICC_INTENT_RELATIVE => {
            pdf_message!(b"Relative Colorimetric\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        _ => {
            pdf_message!(b"(invalid)\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    if (*icch).creator == 0 as iccSig {
        pdf_message!(
            b"pdf_color>> %s:\t(null)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Creator\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if isprint(
        ((*icch).creator >> 24 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
    ) == 0
        || isprint(
            ((*icch).creator >> 16 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(
            ((*icch).creator >> 8 as ::core::ffi::c_int & 0xff as iccSig) as ::core::ffi::c_int,
        ) == 0
        || isprint(((*icch).creator & 0xff as iccSig) as ::core::ffi::c_int) == 0
    {
        pdf_message!(
            b"pdf_color>> %s:\t(invalid)\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Creator\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        pdf_message!(
            b"pdf_color>> %s:\t%c%c%c%c\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Creator\0" as *const u8 as *const ::core::ffi::c_char,
            (*icch).creator >> 24 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).creator >> 16 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).creator >> 8 as ::core::ffi::c_int & 0xff as iccSig,
            (*icch).creator & 0xff as iccSig,
        );
    }
    pdf_message!(b"pdf_color>> Illuminant (XYZ):\t\0" as *const u8 as *const ::core::ffi::c_char,);
    pdf_message!(
        b"%.3f %.3f %.3f\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*icch).illuminant.X as ::core::ffi::c_double
            / 0x10000 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*icch).illuminant.Y as ::core::ffi::c_double
            / 0x10000 as ::core::ffi::c_int as ::core::ffi::c_double,
        (*icch).illuminant.Z as ::core::ffi::c_double
            / 0x10000 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    pdf_message!(b"pdf_color>> Checksum:\t\0" as *const u8 as *const ::core::ffi::c_char);
    if memcmp(
        &raw mut (*icch).ID as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        &raw mut nullbytes16 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        16 as size_t,
    ) == 0
    {
        pdf_message!(b"(null)\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        i = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            if i == 0 as ::core::ffi::c_int {
                pdf_message!(
                    b"%02x\0" as *const u8 as *const ::core::ffi::c_char,
                    (*icch).ID[i as usize] as ::core::ffi::c_int,
                );
            } else {
                pdf_message!(
                    b":%02x\0" as *const u8 as *const ::core::ffi::c_char,
                    (*icch).ID[i as usize] as ::core::ffi::c_int,
                );
            }
            i += 1;
        }
    }
    pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    if !checksum.is_null() {
        pdf_message!(b"pdf_color>> Calculated:\t\0" as *const u8 as *const ::core::ffi::c_char,);
        i = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            if i == 0 as ::core::ffi::c_int {
                pdf_message!(
                    b"%02x\0" as *const u8 as *const ::core::ffi::c_char,
                    *checksum.offset(i as isize) as ::core::ffi::c_int,
                );
            } else {
                pdf_message!(
                    b":%02x\0" as *const u8 as *const ::core::ffi::c_char,
                    *checksum.offset(i as isize) as ::core::ffi::c_int,
                );
            }
            i += 1;
        }
        pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
unsafe extern "C-unwind" fn iccp_devClass_allowed(
    mut dev_class: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut colormode: ::core::ffi::c_int = 0;
    colormode = pdf_dev_get_param(PDF_DEV_PARAM_COLORMODE);
    match colormode {
        _ => {}
    }
    if dev_class as iccSig
        != str2iccSig(
            b"scnr\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        )
        && dev_class as iccSig
            != str2iccSig(
                b"mntr\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            )
        && dev_class as iccSig
            != str2iccSig(
                b"prtr\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            )
        && dev_class as iccSig
            != str2iccSig(
                b"spac\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            )
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_iccp_load_profile")]
pub unsafe extern "C-unwind" fn iccp_load_profile(
    mut ident: *const ::core::ffi::c_char,
    mut profile: *const ::core::ffi::c_void,
    mut proflen: ::core::ffi::c_int,
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
    iccp_init_iccHeader(&raw mut icch);
    if iccp_unpack_header(&raw mut icch, profile, proflen, 1 as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Invalid ICC profile header in \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
        print_iccp_header(
            &raw mut icch,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        );
        return -(1 as ::core::ffi::c_int);
    }
    if iccp_version_supported(
        icch.version as ::core::ffi::c_int >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
        icch.version as ::core::ffi::c_int >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
    ) == 0
    {
        pdf_warning!(
            b"ICC profile format spec. version %d.%01d.%01d not supported in current PDF version setting.\0"
            as * const u8 as * const ::core::ffi::c_char, icch.version >> 24 as
            ::core::ffi::c_int & 0xff as int32_t, icch.version >> 20 as
            ::core::ffi::c_int & 0xf as int32_t, icch.version >> 16 as ::core::ffi::c_int
            & 0xf as int32_t,
        );
        pdf_warning!(b"ICC profile not embedded.\0" as *const u8 as *const ::core::ffi::c_char,);
        print_iccp_header(
            &raw mut icch,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        );
        return -(1 as ::core::ffi::c_int);
    }
    if iccp_devClass_allowed(icch.devClass as ::core::ffi::c_int) == 0 {
        pdf_warning!(
            b"Unsupported ICC Profile Device Class:\0" as *const u8 as *const ::core::ffi::c_char,
        );
        print_iccp_header(
            &raw mut icch,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        );
        return -(1 as ::core::ffi::c_int);
    }
    if icch.colorSpace
        == str2iccSig(
            b"RGB \0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        )
    {
        colorspace = PDF_COLORSPACE_TYPE_RGB;
    } else if icch.colorSpace
        == str2iccSig(
            b"GRAY\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        )
    {
        colorspace = PDF_COLORSPACE_TYPE_GRAY;
    } else if icch.colorSpace
        == str2iccSig(
            b"CMYK\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        )
    {
        colorspace = PDF_COLORSPACE_TYPE_CMYK;
    } else {
        pdf_warning!(
            b"Unsupported input color space.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        print_iccp_header(
            &raw mut icch,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        );
        return -(1 as ::core::ffi::c_int);
    }
    iccp_get_checksum(
        &raw mut checksum as *mut ::core::ffi::c_uchar,
        profile,
        proflen,
    );
    if memcmp(
        &raw mut icch.ID as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        &raw mut nullbytes16 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        16 as size_t,
    ) != 0
        && memcmp(
            &raw mut icch.ID as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            &raw mut checksum as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            16 as size_t,
        ) != 0
    {
        pdf_warning!(
            b"Invalid ICC profile: Inconsistent checksum.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        print_iccp_header(
            &raw mut icch,
            &raw mut checksum as *mut ::core::ffi::c_uchar,
        );
        return -(1 as ::core::ffi::c_int);
    }
    cdata = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<iccbased_cdata>() as usize)
        as uint32_t) as *mut iccbased_cdata;
    init_iccbased_cdata(cdata);
    (*cdata).colorspace = colorspace;
    memcpy(
        &raw mut (*cdata).checksum as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut checksum as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        16 as size_t,
    );
    cspc_id = pdf_colorspace_findresource(
        ident,
        PDF_COLORSPACE_TYPE_ICCBASED,
        cdata as *const ::core::ffi::c_void,
    );
    if cspc_id >= 0 as ::core::ffi::c_int {
        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
            pdf_message!(
                b"(ICCP:[id=%d])\0" as *const u8 as *const ::core::ffi::c_char,
                cspc_id,
            );
        }
        release_iccbased_cdata(cdata);
        return cspc_id;
    }
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        print_iccp_header(
            &raw mut icch,
            &raw mut checksum as *mut ::core::ffi::c_uchar,
        );
    }
    resource = pdf_new_array();
    stream = pdf_new_stream(STREAM_COMPRESS);
    pdf_add_array(
        resource,
        pdf_new_name(b"ICCBased\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_array(resource, pdf_ref_obj(stream));
    stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"N\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(get_num_components_iccbased(cdata) as ::core::ffi::c_double),
    );
    pdf_add_stream(stream, profile, proflen);
    pdf_release_obj(stream);
    cspc_id = pdf_colorspace_defineresource(
        ident,
        PDF_COLORSPACE_TYPE_ICCBASED,
        cdata as *mut ::core::ffi::c_void,
        resource,
    );
    return cspc_id;
}
static mut cspc_cache: C2RustUnnamed = C2RustUnnamed {
    count: 0 as ::core::ffi::c_uint,
    capacity: 0 as ::core::ffi::c_uint,
    colorspaces: ::core::ptr::null::<pdf_colorspace>() as *mut pdf_colorspace,
};
unsafe extern "C-unwind" fn pdf_colorspace_findresource(
    mut ident: *const ::core::ffi::c_char,
    mut type_0: ::core::ffi::c_int,
    mut cdata: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut colorspace: *mut pdf_colorspace = ::core::ptr::null_mut::<pdf_colorspace>();
    let mut cspc_id: ::core::ffi::c_int = 0;
    let mut cmp: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    cspc_id = 0 as ::core::ffi::c_int;
    while cmp != 0 && (cspc_id as ::core::ffi::c_uint) < cspc_cache.count {
        colorspace = cspc_cache.colorspaces.offset(cspc_id as isize) as *mut pdf_colorspace;
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
    mut ident: *const ::core::ffi::c_char,
    mut subtype: ::core::ffi::c_int,
    mut cdata: *mut ::core::ffi::c_void,
    mut resource: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut cspc_id: ::core::ffi::c_int = 0;
    let mut colorspace: *mut pdf_colorspace = ::core::ptr::null_mut::<pdf_colorspace>();
    if cspc_cache.count >= cspc_cache.capacity {
        cspc_cache.capacity = cspc_cache.capacity.wrapping_add(16 as ::core::ffi::c_uint);
        cspc_cache.colorspaces = renew(
            cspc_cache.colorspaces as *mut ::core::ffi::c_void,
            (cspc_cache.capacity as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<pdf_colorspace>() as usize)
                as uint32_t,
        ) as *mut pdf_colorspace;
    }
    cspc_id = cspc_cache.count as ::core::ffi::c_int;
    colorspace = cspc_cache.colorspaces.offset(cspc_id as isize) as *mut pdf_colorspace;
    pdf_init_colorspace_struct(colorspace);
    if !ident.is_null() {
        (*colorspace).ident = new(
            (strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy((*colorspace).ident, ident);
    }
    (*colorspace).subtype = subtype;
    (*colorspace).cdata = cdata;
    (*colorspace).resource = resource;
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(
            b"(ColorSpace:%s\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
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
    cspc_cache.count = cspc_cache.count.wrapping_add(1);
    return cspc_id;
}
#[unsafe(export_name = "__pdf_impl_pdf_get_colorspace_reference")]
pub unsafe extern "C-unwind" fn pdf_get_colorspace_reference(
    mut cspc_id: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_colorspace = ::core::ptr::null_mut::<pdf_colorspace>();
    colorspace = cspc_cache.colorspaces.offset(cspc_id as isize) as *mut pdf_colorspace;
    if (*colorspace).reference.is_null() {
        (*colorspace).reference = pdf_ref_obj((*colorspace).resource);
        pdf_release_obj((*colorspace).resource);
        (*colorspace).resource = ::core::ptr::null_mut::<pdf_obj>();
    }
    return pdf_link_obj((*colorspace).reference);
}
#[unsafe(export_name = "__pdf_impl_pdf_init_colors")]
pub unsafe extern "C-unwind" fn pdf_init_colors() {
    cspc_cache.count = 0 as ::core::ffi::c_uint;
    cspc_cache.capacity = 0 as ::core::ffi::c_uint;
    cspc_cache.colorspaces = ::core::ptr::null_mut::<pdf_colorspace>();
}
#[unsafe(export_name = "__pdf_impl_pdf_close_colors")]
pub unsafe extern "C-unwind" fn pdf_close_colors() {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < cspc_cache.count {
        let mut colorspace: *mut pdf_colorspace = ::core::ptr::null_mut::<pdf_colorspace>();
        colorspace = cspc_cache.colorspaces.offset(i as isize) as *mut pdf_colorspace;
        pdf_flush_colorspace(colorspace);
        pdf_clean_colorspace_struct(colorspace);
        i = i.wrapping_add(1);
    }
    cspc_cache.colorspaces =
        mfree(cspc_cache.colorspaces as *mut ::core::ffi::c_void) as *mut pdf_colorspace;
    cspc_cache.capacity = 0 as ::core::ffi::c_uint;
    cspc_cache.count = cspc_cache.capacity;
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
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_color_type(
    mut color: *const pdf_color,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
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
    mut color: *mut pdf_color,
    mut r: ::core::ffi::c_double,
    mut g: ::core::ffi::c_double,
    mut b: ::core::ffi::c_double,
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
    mut color: *mut pdf_color,
    mut c: ::core::ffi::c_double,
    mut m: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut k: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
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
    mut color: *mut pdf_color,
    mut g: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
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
    mut color: *mut pdf_color,
    mut name: *mut ::core::ffi::c_char,
    mut c: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_copycolor(
    mut color1: *mut pdf_color,
    mut color2: *const pdf_color,
) -> i32 {
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
    mut dst: *mut pdf_color,
    mut src: *const pdf_color,
    mut f: ::core::ffi::c_double,
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_is_white(
    mut color: *const pdf_color,
    _result: *mut bool,
) -> i32 {
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
    mut color: *const pdf_color,
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_len: size_t,
    mut mask: ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_color_set_color(color, buffer, buffer_len, mask)
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_compare(
    mut color1: *const pdf_color,
    mut color2: *const pdf_color,
    _result: *mut ::core::ffi::c_int,
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_set(
    mut sc: *mut pdf_color,
    mut fc: *mut pdf_color,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_color_set(sc, fc) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_color_push(
    mut sc: *mut pdf_color,
    mut fc: *mut pdf_color,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_pdf_color_get_current(
    mut sc: *mut *mut pdf_color,
    mut fc: *mut *mut pdf_color,
) -> i32 {
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
    mut colortype: ::core::ffi::c_int,
    mut profile: *const ::core::ffi::c_void,
    mut proflen: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        iccp_check_colorspace(colortype, profile, proflen)
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
pub unsafe extern "C" fn __pdf_entry_iccp_get_rendering_intent(
    mut profile: *const ::core::ffi::c_void,
    mut proflen: ::core::ffi::c_int,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        iccp_get_rendering_intent(profile, proflen)
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
pub unsafe extern "C" fn __pdf_entry_iccp_load_profile(
    mut ident: *const ::core::ffi::c_char,
    mut profile: *const ::core::ffi::c_void,
    mut proflen: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        iccp_load_profile(ident, profile, proflen)
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_colorspace_reference(
    mut cspc_id: ::core::ffi::c_int,
    _result: *mut *mut pdf_obj,
) -> i32 {
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
