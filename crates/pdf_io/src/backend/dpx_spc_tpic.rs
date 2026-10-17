/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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
    fn isascii(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn round(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atof(_: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
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
    #[link_name = "__pdf_impl_pdf_ref_obj"]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_boolean"]
    fn pdf_new_boolean(value: ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_string"]
    fn pdf_new_string(str: *const ::core::ffi::c_void, length: size_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_string_value"]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_name_value"]
    fn pdf_name_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
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
    #[link_name = "__pdf_impl_pdf_check_version"]
    fn pdf_check_version(
        major: ::core::ffi::c_int,
        minor: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_brighten_color"]
    fn pdf_color_brighten_color(
        dst: *mut pdf_color,
        src: *const pdf_color,
        f: ::core::ffi::c_double,
    );
    #[link_name = "__pdf_impl_pdf_color_get_current"]
    fn pdf_color_get_current(sc: *mut *mut pdf_color, fc: *mut *mut pdf_color);
    fn spc_warn(spe: *mut spc_env, fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_parse_float_decimal"]
    fn parse_float_decimal(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_c_string"]
    fn parse_c_string(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_c_ident"]
    fn parse_c_ident(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_doc_current_page_resources"]
    fn pdf_doc_current_page_resources() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_doc_add_page_content"]
    fn pdf_doc_add_page_content(buffer: *const ::core::ffi::c_char, length: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_doc_add_page_resource"]
    fn pdf_doc_add_page_resource(
        category: *const ::core::ffi::c_char,
        resource_name: *const ::core::ffi::c_char,
        resources: *mut pdf_obj,
    );
    #[link_name = "__pdf_impl_pdf_dev_setlinewidth"]
    fn pdf_dev_setlinewidth(width: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_setmiterlimit"]
    fn pdf_dev_setmiterlimit(mlimit: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_setlinecap"]
    fn pdf_dev_setlinecap(style: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_setlinejoin"]
    fn pdf_dev_setlinejoin(style: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_setdash"]
    fn pdf_dev_setdash(
        count: ::core::ffi::c_int,
        pattern: *mut ::core::ffi::c_double,
        offset: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_moveto"]
    fn pdf_dev_moveto(x: ::core::ffi::c_double, y: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_lineto"]
    fn pdf_dev_lineto(x0: ::core::ffi::c_double, y0: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_newpath"]
    fn pdf_dev_newpath() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_flushpath"]
    fn pdf_dev_flushpath(
        p_op: ::core::ffi::c_char,
        fill_rule: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_concat"]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_gsave"]
    fn pdf_dev_gsave() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_grestore"]
    fn pdf_dev_grestore() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_arcx"]
    fn pdf_dev_arcx(
        c_x: ::core::ffi::c_double,
        c_y: ::core::ffi::c_double,
        r_x: ::core::ffi::c_double,
        r_y: ::core::ffi::c_double,
        a_0: ::core::ffi::c_double,
        a_1: ::core::ffi::c_double,
        a_d: ::core::ffi::c_int,
        xar: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_bspline"]
    fn pdf_dev_bspline(
        x0: ::core::ffi::c_double,
        y0: ::core::ffi::c_double,
        x1: ::core::ffi::c_double,
        y1: ::core::ffi::c_double,
        x2: ::core::ffi::c_double,
        y2: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_set_color"]
    fn pdf_dev_set_color(
        color: *const pdf_color,
        mask: ::core::ffi::c_char,
        force: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_parse_val_ident"]
    fn parse_val_ident(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
pub struct C2RustUnnamed_0 {
    pub fill: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_tpic_ {
    pub mode: C2RustUnnamed_0,
    pub pen_size: ::core::ffi::c_double,
    pub fill_shape: bool,
    pub fill_color: ::core::ffi::c_double,
    pub points: *mut pdf_coord,
    pub num_points: ::core::ffi::c_int,
    pub max_points: ::core::ffi::c_int,
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
unsafe extern "C-unwind" fn streq_ptr(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
    }
    return false_0 != 0;
}
pub const PDF_STRING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PDF_DICT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PDF_FILL_RULE_NONZERO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MI2DEV: ::core::ffi::c_double = 0.072f64 / 1.0f64;
pub const TPIC_MODE__FILL_SOLID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TPIC_MODE__FILL_OPACITY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TPIC_MODE__FILL_SHAPE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
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
static mut _tpic_state: spc_tpic_ = spc_tpic_ {
    mode: C2RustUnnamed_0 { fill: 0 },
    pen_size: 0.,
    fill_shape: false,
    fill_color: 0.,
    points: ::core::ptr::null::<pdf_coord>() as *mut pdf_coord,
    num_points: 0,
    max_points: 0,
};
unsafe extern "C-unwind" fn tpic__clear(mut tp: *mut spc_tpic_) {
    (*tp).points = mfree((*tp).points as *mut ::core::ffi::c_void) as *mut pdf_coord;
    (*tp).num_points = 0 as ::core::ffi::c_int;
    (*tp).max_points = 0 as ::core::ffi::c_int;
    (*tp).fill_shape = 0 as ::core::ffi::c_int != 0;
    (*tp).fill_color = 0.0f64;
}
unsafe extern "C-unwind" fn create_xgstate(
    mut a: ::core::ffi::c_double,
    mut f_ais: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    dict = pdf_new_dict();
    pdf_add_dict(
        dict,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"ExtGState\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if f_ais != 0 {
        pdf_add_dict(
            dict,
            pdf_new_name(b"AIS\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_boolean(1 as ::core::ffi::c_char),
        );
    }
    pdf_add_dict(
        dict,
        pdf_new_name(b"ca\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(a),
    );
    return dict;
}
unsafe extern "C-unwind" fn check_resourcestatus(
    mut category: *const ::core::ffi::c_char,
    mut resname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut dict1: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut dict2: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    dict1 = pdf_doc_current_page_resources();
    if dict1.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    dict2 = pdf_lookup_dict(dict1, category);
    if !dict2.is_null() && pdf_obj_typeof(dict2) == PDF_DICT {
        if !pdf_lookup_dict(dict2, resname).is_null() {
            return 1 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_linestyle(
    mut pn: ::core::ffi::c_double,
    mut da: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut dp: [::core::ffi::c_double; 2] = [0.; 2];
    pdf_dev_setlinejoin(1 as ::core::ffi::c_int);
    pdf_dev_setmiterlimit(1.4f64);
    pdf_dev_setlinewidth(pn);
    if da > 0.0f64 {
        dp[0 as ::core::ffi::c_int as usize] = da * 72.0f64;
        pdf_dev_setdash(
            1 as ::core::ffi::c_int,
            &raw mut dp as *mut ::core::ffi::c_double,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
        pdf_dev_setlinecap(0 as ::core::ffi::c_int);
    } else if da < 0.0f64 {
        dp[0 as ::core::ffi::c_int as usize] = pn;
        dp[1 as ::core::ffi::c_int as usize] = -da * 72.0f64;
        pdf_dev_setdash(
            2 as ::core::ffi::c_int,
            &raw mut dp as *mut ::core::ffi::c_double,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
        pdf_dev_setlinecap(1 as ::core::ffi::c_int);
    } else {
        pdf_dev_setlinecap(0 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_fillstyle(
    mut g: ::core::ffi::c_double,
    mut a: ::core::ffi::c_double,
    mut f_ais: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut resname: [::core::ffi::c_char; 32] = [0; 32];
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut alp: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if a > 0.0f64 {
        alp = round(100.0f64 * a) as ::core::ffi::c_int;
        sprintf(
            &raw mut resname as *mut ::core::ffi::c_char,
            b"_Tps_a%03d_\0" as *const u8 as *const ::core::ffi::c_char,
            alp,
        );
        if check_resourcestatus(
            b"ExtGState\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut resname as *mut ::core::ffi::c_char,
        ) == 0
        {
            dict = create_xgstate(
                floor(0.01f64 * alp as ::core::ffi::c_double / 0.01f64 + 0.5f64) * 0.01f64,
                f_ais,
            );
            pdf_doc_add_page_resource(
                b"ExtGState\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut resname as *mut ::core::ffi::c_char,
                pdf_ref_obj(dict),
            );
            pdf_release_obj(dict);
        }
        len += sprintf(
            (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
            b" /%s gs\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut resname as *mut ::core::ffi::c_char,
        );
        pdf_doc_add_page_content(
            &raw mut buf as *mut ::core::ffi::c_char,
            len as ::core::ffi::c_uint,
        );
    }
    let mut sc: *mut pdf_color = ::core::ptr::null_mut::<pdf_color>();
    let mut fc: *mut pdf_color = ::core::ptr::null_mut::<pdf_color>();
    let mut new_fc: pdf_color = pdf_color {
        res_id: 0,
        type_0: 0,
        num_components: 0,
        spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        values: [0.; 32],
        pattern_id: 0,
    };
    pdf_color_get_current(&raw mut sc, &raw mut fc);
    pdf_color_brighten_color(&raw mut new_fc, fc, g);
    pdf_dev_set_color(
        &raw mut new_fc,
        0x20 as ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_styles(
    mut tp: *mut spc_tpic_,
    mut c: *const pdf_coord,
    mut f_fs: bool,
    mut f_vp: bool,
    mut pn: ::core::ffi::c_double,
    mut da: ::core::ffi::c_double,
) {
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    M.a = 1.0f64;
    M.b = 0.0f64;
    M.c = 0.0f64;
    M.d = -1.0f64;
    M.e = (*c).x;
    M.f = (*c).y;
    pdf_dev_concat(&raw mut M);
    if f_vp {
        set_linestyle(pn, da);
    }
    if f_fs {
        let mut g: ::core::ffi::c_double = 0.;
        let mut a: ::core::ffi::c_double = 0.;
        let mut f_ais: ::core::ffi::c_int = 0;
        if (*tp).mode.fill == TPIC_MODE__FILL_SOLID || (*tp).fill_color == 0. {
            g = 1.0f64 - (*tp).fill_color;
            a = 0.0f64;
        } else {
            g = 0.0f64;
            a = (*tp).fill_color;
        }
        f_ais = if (*tp).mode.fill == TPIC_MODE__FILL_SHAPE {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        set_fillstyle(g, a, f_ais);
    }
}
unsafe extern "C-unwind" fn showpath(mut f_vp: bool, mut f_fs: bool) {
    if f_vp {
        if f_fs {
            pdf_dev_flushpath('b' as i32 as ::core::ffi::c_char, PDF_FILL_RULE_NONZERO);
        } else {
            pdf_dev_flushpath('S' as i32 as ::core::ffi::c_char, PDF_FILL_RULE_NONZERO);
        }
    } else if f_fs {
        pdf_dev_flushpath('f' as i32 as ::core::ffi::c_char, PDF_FILL_RULE_NONZERO);
    } else {
        pdf_dev_newpath();
    };
}
unsafe extern "C-unwind" fn tpic__polyline(
    mut tp: *mut spc_tpic_,
    mut c: *const pdf_coord,
    mut f_vp: bool,
    mut da: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut pn: ::core::ffi::c_double = (*tp).pen_size;
    let mut f_fs: bool = (*tp).fill_shape;
    let mut i: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    f_fs = if (*(*tp).points.offset(0 as ::core::ffi::c_int as isize)).x
        == (*(*tp)
            .points
            .offset(((*tp).num_points - 1 as ::core::ffi::c_int) as isize))
        .x
        && (*(*tp).points.offset(0 as ::core::ffi::c_int as isize)).y
            == (*(*tp)
                .points
                .offset(((*tp).num_points - 1 as ::core::ffi::c_int) as isize))
            .y
    {
        f_fs as ::core::ffi::c_int
    } else {
        false_0
    } != 0;
    f_vp = if pn > 0.0f64 {
        f_vp as ::core::ffi::c_int
    } else {
        false_0
    } != 0;
    if f_vp as ::core::ffi::c_int != 0 || f_fs as ::core::ffi::c_int != 0 {
        pdf_dev_gsave();
        set_styles(tp, c, f_fs, f_vp, pn, da);
        pdf_dev_moveto(
            (*(*tp).points.offset(0 as ::core::ffi::c_int as isize)).x,
            (*(*tp).points.offset(0 as ::core::ffi::c_int as isize)).y,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*tp).num_points {
            pdf_dev_lineto(
                (*(*tp).points.offset(i as isize)).x,
                (*(*tp).points.offset(i as isize)).y,
            );
            i += 1;
        }
        showpath(f_vp, f_fs);
        pdf_dev_grestore();
    }
    tpic__clear(tp);
    return error;
}
unsafe extern "C-unwind" fn tpic__spline(
    mut tp: *mut spc_tpic_,
    mut c: *const pdf_coord,
    mut f_vp: bool,
    mut da: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut v: [::core::ffi::c_double; 6] = [0.; 6];
    let mut pn: ::core::ffi::c_double = (*tp).pen_size;
    let mut f_fs: bool = (*tp).fill_shape;
    let mut i: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    f_fs = if (*(*tp).points.offset(0 as ::core::ffi::c_int as isize)).x
        == (*(*tp)
            .points
            .offset(((*tp).num_points - 1 as ::core::ffi::c_int) as isize))
        .x
        && (*(*tp).points.offset(0 as ::core::ffi::c_int as isize)).y
            == (*(*tp)
                .points
                .offset(((*tp).num_points - 1 as ::core::ffi::c_int) as isize))
            .y
    {
        f_fs as ::core::ffi::c_int
    } else {
        false_0
    } != 0;
    f_vp = if pn > 0.0f64 {
        f_vp as ::core::ffi::c_int
    } else {
        false_0
    } != 0;
    if f_vp as ::core::ffi::c_int != 0 || f_fs as ::core::ffi::c_int != 0 {
        pdf_dev_gsave();
        set_styles(tp, c, f_fs, f_vp, pn, da);
        pdf_dev_moveto(
            (*(*tp).points.offset(0 as ::core::ffi::c_int as isize)).x,
            (*(*tp).points.offset(0 as ::core::ffi::c_int as isize)).y,
        );
        v[0 as ::core::ffi::c_int as usize] = 0.5f64
            * ((*(*tp).points.offset(0 as ::core::ffi::c_int as isize)).x
                + (*(*tp).points.offset(1 as ::core::ffi::c_int as isize)).x);
        v[1 as ::core::ffi::c_int as usize] = 0.5f64
            * ((*(*tp).points.offset(0 as ::core::ffi::c_int as isize)).y
                + (*(*tp).points.offset(1 as ::core::ffi::c_int as isize)).y);
        pdf_dev_lineto(
            v[0 as ::core::ffi::c_int as usize],
            v[1 as ::core::ffi::c_int as usize],
        );
        i = 1 as ::core::ffi::c_int;
        while i < (*tp).num_points - 1 as ::core::ffi::c_int {
            v[0 as ::core::ffi::c_int as usize] = 0.5f64
                * ((*(*tp).points.offset((i - 1 as ::core::ffi::c_int) as isize)).x
                    + (*(*tp).points.offset(i as isize)).x);
            v[1 as ::core::ffi::c_int as usize] = 0.5f64
                * ((*(*tp).points.offset((i - 1 as ::core::ffi::c_int) as isize)).y
                    + (*(*tp).points.offset(i as isize)).y);
            v[2 as ::core::ffi::c_int as usize] = (*(*tp).points.offset(i as isize)).x;
            v[3 as ::core::ffi::c_int as usize] = (*(*tp).points.offset(i as isize)).y;
            v[4 as ::core::ffi::c_int as usize] = 0.5f64
                * ((*(*tp).points.offset(i as isize)).x
                    + (*(*tp).points.offset((i + 1 as ::core::ffi::c_int) as isize)).x);
            v[5 as ::core::ffi::c_int as usize] = 0.5f64
                * ((*(*tp).points.offset(i as isize)).y
                    + (*(*tp).points.offset((i + 1 as ::core::ffi::c_int) as isize)).y);
            pdf_dev_bspline(
                v[0 as ::core::ffi::c_int as usize],
                v[1 as ::core::ffi::c_int as usize],
                v[2 as ::core::ffi::c_int as usize],
                v[3 as ::core::ffi::c_int as usize],
                v[4 as ::core::ffi::c_int as usize],
                v[5 as ::core::ffi::c_int as usize],
            );
            i += 1;
        }
        pdf_dev_lineto(
            (*(*tp).points.offset(i as isize)).x,
            (*(*tp).points.offset(i as isize)).y,
        );
        showpath(f_vp, f_fs);
        pdf_dev_grestore();
    }
    tpic__clear(tp);
    return error;
}
unsafe extern "C-unwind" fn tpic__arc(
    mut tp: *mut spc_tpic_,
    mut c: *const pdf_coord,
    mut f_vp: bool,
    mut da: ::core::ffi::c_double,
    mut v: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut pn: ::core::ffi::c_double = (*tp).pen_size;
    let mut f_fs: bool = (*tp).fill_shape;
    f_fs = if round(
        fabs(
            *v.offset(4 as ::core::ffi::c_int as isize)
                - *v.offset(5 as ::core::ffi::c_int as isize),
        ) + 0.5f64,
    ) >= 360 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        f_fs as ::core::ffi::c_int
    } else {
        false_0
    } != 0;
    f_vp = if pn > 0.0f64 {
        f_vp as ::core::ffi::c_int
    } else {
        false_0
    } != 0;
    if f_vp as ::core::ffi::c_int != 0 || f_fs as ::core::ffi::c_int != 0 {
        pdf_dev_gsave();
        set_styles(tp, c, f_fs, f_vp, pn, da);
        pdf_dev_newpath();
        pdf_dev_arcx(
            *v.offset(0 as ::core::ffi::c_int as isize),
            *v.offset(1 as ::core::ffi::c_int as isize),
            *v.offset(2 as ::core::ffi::c_int as isize),
            *v.offset(3 as ::core::ffi::c_int as isize),
            *v.offset(4 as ::core::ffi::c_int as isize),
            *v.offset(5 as ::core::ffi::c_int as isize),
            1 as ::core::ffi::c_int,
            0.0f64,
        );
        showpath(f_vp, f_fs);
        pdf_dev_grestore();
    }
    tpic__clear(tp);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_currentpoint(
    mut spe: *mut spc_env,
    mut pg: *mut ::core::ffi::c_int,
    mut cp: *mut pdf_coord,
) -> ::core::ffi::c_int {
    *pg = 0 as ::core::ffi::c_int;
    (*cp).x = (*spe).x_user;
    (*cp).y = (*spe).y_user;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_tpic_pn(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    q = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
    if q.is_null() {
        pdf_warning!(b"Invalid pen size specified?\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    (*tp).pen_size = atof(q) * MI2DEV;
    free(q as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_tpic_pa(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut v: [::core::ffi::c_double; 2] = [0.; 2];
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int && (*ap).curptr < (*ap).endptr {
        q = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
        if q.is_null() {
            pdf_warning!(
                b"Missing numbers for TPIC \"pa\" command.\0" as *const u8
                    as *const ::core::ffi::c_char
            );
            return -(1 as ::core::ffi::c_int);
        }
        v[i as usize] = atof(q);
        free(q as *mut ::core::ffi::c_void);
        skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        i += 1;
    }
    if i != 2 as ::core::ffi::c_int {
        pdf_warning!(
            b"Invalid arg for TPIC \"pa\" command.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*tp).num_points >= (*tp).max_points {
        (*tp).max_points += 256 as ::core::ffi::c_int;
        (*tp).points = renew(
            (*tp).points as *mut ::core::ffi::c_void,
            ((*tp).max_points as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<pdf_coord>() as usize) as uint32_t,
        ) as *mut pdf_coord;
    }
    (*(*tp).points.offset((*tp).num_points as isize)).x =
        v[0 as ::core::ffi::c_int as usize] * MI2DEV;
    (*(*tp).points.offset((*tp).num_points as isize)).y =
        v[1 as ::core::ffi::c_int as usize] * MI2DEV;
    (*tp).num_points += 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_tpic_fp(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: ::core::ffi::c_int = 0;
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*tp).num_points <= 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"Too few points (< 2) for polyline path.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    spc_currentpoint(spe, &raw mut pg, &raw mut cp);
    return tpic__polyline(tp, &raw mut cp, 1 as ::core::ffi::c_int != 0, 0.0f64);
}
unsafe extern "C-unwind" fn spc_handler_tpic_ip(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: ::core::ffi::c_int = 0;
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*tp).num_points <= 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"Too few points (< 2) for polyline path.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    spc_currentpoint(spe, &raw mut pg, &raw mut cp);
    return tpic__polyline(tp, &raw mut cp, 0 as ::core::ffi::c_int != 0, 0.0f64);
}
unsafe extern "C-unwind" fn spc_handler_tpic_da(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut da: ::core::ffi::c_double = 0.0f64;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: ::core::ffi::c_int = 0;
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    q = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        da = atof(q);
        free(q as *mut ::core::ffi::c_void);
    }
    if (*tp).num_points <= 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"Too few points (< 2) for polyline path.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    spc_currentpoint(spe, &raw mut pg, &raw mut cp);
    return tpic__polyline(tp, &raw mut cp, 1 as ::core::ffi::c_int != 0, da);
}
unsafe extern "C-unwind" fn spc_handler_tpic_dt(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut da: ::core::ffi::c_double = 0.0f64;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: ::core::ffi::c_int = 0;
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    q = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        da = -atof(q);
        free(q as *mut ::core::ffi::c_void);
    }
    if (*tp).num_points <= 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"Too few points (< 2) for polyline path.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    spc_currentpoint(spe, &raw mut pg, &raw mut cp);
    return tpic__polyline(tp, &raw mut cp, 1 as ::core::ffi::c_int != 0, da);
}
unsafe extern "C-unwind" fn spc_handler_tpic_sp(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut da: ::core::ffi::c_double = 0.0f64;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: ::core::ffi::c_int = 0;
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    q = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        da = atof(q);
        free(q as *mut ::core::ffi::c_void);
    }
    if (*tp).num_points <= 2 as ::core::ffi::c_int {
        pdf_warning!(
            b"Too few points (< 3) for spline path.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    spc_currentpoint(spe, &raw mut pg, &raw mut cp);
    return tpic__spline(tp, &raw mut cp, 1 as ::core::ffi::c_int != 0, da);
}
unsafe extern "C-unwind" fn spc_handler_tpic_ar(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    let mut v: [::core::ffi::c_double; 6] = [0.; 6];
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: ::core::ffi::c_int = 0;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    i = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int && (*ap).curptr < (*ap).endptr {
        q = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
        if q.is_null() {
            pdf_warning!(
                b"Invalid args. in TPIC \"ar\" command.\0" as *const u8
                    as *const ::core::ffi::c_char
            );
            return -(1 as ::core::ffi::c_int);
        }
        v[i as usize] = atof(q);
        free(q as *mut ::core::ffi::c_void);
        skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        i += 1;
    }
    if i != 6 as ::core::ffi::c_int {
        pdf_warning!(
            b"Invalid arg for TPIC \"ar\" command.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    v[0 as ::core::ffi::c_int as usize] *= MI2DEV;
    v[1 as ::core::ffi::c_int as usize] *= MI2DEV;
    v[2 as ::core::ffi::c_int as usize] *= MI2DEV;
    v[3 as ::core::ffi::c_int as usize] *= MI2DEV;
    v[4 as ::core::ffi::c_int as usize] *= 180.0f64 / M_PI;
    v[5 as ::core::ffi::c_int as usize] *= 180.0f64 / M_PI;
    spc_currentpoint(spe, &raw mut pg, &raw mut cp);
    return tpic__arc(
        tp,
        &raw mut cp,
        1 as ::core::ffi::c_int != 0,
        0.0f64,
        &raw mut v as *mut ::core::ffi::c_double,
    );
}
unsafe extern "C-unwind" fn spc_handler_tpic_ia(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    let mut v: [::core::ffi::c_double; 6] = [0.; 6];
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut pg: ::core::ffi::c_int = 0;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    i = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int && (*ap).curptr < (*ap).endptr {
        q = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
        if q.is_null() {
            pdf_warning!(
                b"Invalid args. in TPIC \"ia\" command.\0" as *const u8
                    as *const ::core::ffi::c_char
            );
            return -(1 as ::core::ffi::c_int);
        }
        v[i as usize] = atof(q);
        free(q as *mut ::core::ffi::c_void);
        skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        i += 1;
    }
    if i != 6 as ::core::ffi::c_int {
        pdf_warning!(
            b"Invalid arg for TPIC \"ia\" command.\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    v[0 as ::core::ffi::c_int as usize] *= MI2DEV;
    v[1 as ::core::ffi::c_int as usize] *= MI2DEV;
    v[2 as ::core::ffi::c_int as usize] *= MI2DEV;
    v[3 as ::core::ffi::c_int as usize] *= MI2DEV;
    v[4 as ::core::ffi::c_int as usize] *= 180.0f64 / M_PI;
    v[5 as ::core::ffi::c_int as usize] *= 180.0f64 / M_PI;
    spc_currentpoint(spe, &raw mut pg, &raw mut cp);
    return tpic__arc(
        tp,
        &raw mut cp,
        0 as ::core::ffi::c_int != 0,
        0.0f64,
        &raw mut v as *mut ::core::ffi::c_double,
    );
}
unsafe extern "C-unwind" fn spc_handler_tpic_sh(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*tp).fill_shape = 1 as ::core::ffi::c_int != 0;
    (*tp).fill_color = 0.5f64;
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    q = parse_float_decimal(&raw mut (*ap).curptr, (*ap).endptr);
    if !q.is_null() {
        let mut g: ::core::ffi::c_double = atof(q);
        free(q as *mut ::core::ffi::c_void);
        if g >= 0.0f64 && g <= 1.0f64 {
            (*tp).fill_color = g;
        } else {
            pdf_warning!(
                b"Invalid fill color specified: %g\n\0" as *const u8 as *const ::core::ffi::c_char,
                g,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_tpic_wh(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*tp).fill_shape = 1 as ::core::ffi::c_int != 0;
    (*tp).fill_color = 0.0f64;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_tpic_bk(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*tp).fill_shape = 1 as ::core::ffi::c_int != 0;
    (*tp).fill_color = 1.0f64;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_tpic_tx(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    if !(!spe.is_null() && !ap.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && ap && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_warning!(
        b"TPIC command \"tx\" not supported.\0" as *const u8 as *const ::core::ffi::c_char
    );
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn spc_handler_tpic__init(
    mut spe: *mut spc_env,
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = dp as *mut spc_tpic_;
    (*tp).pen_size = 1.0f64;
    (*tp).fill_shape = 0 as ::core::ffi::c_int != 0;
    (*tp).fill_color = 0.0f64;
    (*tp).points = ::core::ptr::null_mut::<pdf_coord>();
    (*tp).num_points = 0 as ::core::ffi::c_int;
    (*tp).max_points = 0 as ::core::ffi::c_int;
    if (*tp).mode.fill != TPIC_MODE__FILL_SOLID
        && pdf_check_version(1 as ::core::ffi::c_int, 4 as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Tpic shading support requires PDF version 1.4.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        (*tp).mode.fill = TPIC_MODE__FILL_SOLID;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_tpic__bophook(
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = dp as *mut spc_tpic_;
    if tp.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    tpic__clear(tp);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_tpic__eophook(
    mut spe: *mut spc_env,
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = dp as *mut spc_tpic_;
    if tp.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*tp).num_points > 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Unflushed tpic path at end of the page.\0" as *const u8 as *const ::core::ffi::c_char
        );
    }
    tpic__clear(tp);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_tpic__clean(
    mut spe: *mut spc_env,
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = dp as *mut spc_tpic_;
    if tp.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*tp).num_points > 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Unflushed tpic path at end of the document.\0" as *const u8
                as *const ::core::ffi::c_char
        );
    }
    tpic__clear(tp);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_tpic_set_fill_mode")]
pub unsafe extern "C-unwind" fn tpic_set_fill_mode(mut mode: ::core::ffi::c_int) {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    (*tp).mode.fill = mode;
}
#[unsafe(export_name = "__pdf_impl_spc_tpic_at_begin_page")]
pub unsafe extern "C-unwind" fn spc_tpic_at_begin_page() -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    return spc_handler_tpic__bophook(tp as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_spc_tpic_at_end_page")]
pub unsafe extern "C-unwind" fn spc_tpic_at_end_page() -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    return spc_handler_tpic__eophook(
        ::core::ptr::null_mut::<spc_env>(),
        tp as *mut ::core::ffi::c_void,
    );
}
#[unsafe(export_name = "__pdf_impl_spc_tpic_at_begin_document")]
pub unsafe extern "C-unwind" fn spc_tpic_at_begin_document() -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    return spc_handler_tpic__init(
        ::core::ptr::null_mut::<spc_env>(),
        tp as *mut ::core::ffi::c_void,
    );
}
#[unsafe(export_name = "__pdf_impl_spc_tpic_at_end_document")]
pub unsafe extern "C-unwind" fn spc_tpic_at_end_document() -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    return spc_handler_tpic__clean(
        ::core::ptr::null_mut::<spc_env>(),
        tp as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C-unwind" fn spc_parse_kvpairs(mut ap: *mut spc_arg) -> *mut pdf_obj {
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut kp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut vp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    dict = pdf_new_dict();
    skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
    while error == 0 && (*ap).curptr < (*ap).endptr {
        kp = parse_val_ident(&raw mut (*ap).curptr, (*ap).endptr);
        if kp.is_null() {
            break;
        }
        skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        if (*ap).curptr < (*ap).endptr
            && *(*ap).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '=' as i32
        {
            (*ap).curptr = (*ap).curptr.offset(1);
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
            if (*ap).curptr == (*ap).endptr {
                free(kp as *mut ::core::ffi::c_void);
                error = -(1 as ::core::ffi::c_int);
                break;
            } else {
                vp = parse_c_string(&raw mut (*ap).curptr, (*ap).endptr);
                if vp.is_null() {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    pdf_add_dict(
                        dict,
                        pdf_new_name(kp),
                        pdf_new_string(
                            vp as *const ::core::ffi::c_void,
                            strlen(vp).wrapping_add(1 as size_t),
                        ),
                    );
                    free(vp as *mut ::core::ffi::c_void);
                }
            }
        } else {
            pdf_add_dict(
                dict,
                pdf_new_name(kp),
                pdf_new_boolean(1 as ::core::ffi::c_char),
            );
        }
        free(kp as *mut ::core::ffi::c_void);
        if error == 0 {
            skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        }
    }
    if error != 0 {
        pdf_release_obj(dict);
        dict = ::core::ptr::null_mut::<pdf_obj>();
    }
    return dict;
}
unsafe extern "C-unwind" fn tpic_filter_getopts(
    mut kp: *mut pdf_obj,
    mut vp: *mut pdf_obj,
    mut dp: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = dp as *mut spc_tpic_;
    let mut k: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(!kp.is_null() && !vp.is_null() && !tp.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"kp && vp && tp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    k = pdf_name_value(kp);
    if streq_ptr(k, b"fill-mode\0" as *const u8 as *const ::core::ffi::c_char) {
        if pdf_obj_typeof(vp) != PDF_STRING {
            pdf_warning!(
                b"Invalid value for TPIC option fill-mode...\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            error = -(1 as ::core::ffi::c_int);
        } else {
            v = pdf_string_value(vp) as *mut ::core::ffi::c_char;
            if streq_ptr(v, b"shape\0" as *const u8 as *const ::core::ffi::c_char) {
                (*tp).mode.fill = TPIC_MODE__FILL_SHAPE;
            } else if streq_ptr(v, b"opacity\0" as *const u8 as *const ::core::ffi::c_char) {
                (*tp).mode.fill = TPIC_MODE__FILL_OPACITY;
            } else if streq_ptr(v, b"solid\0" as *const u8 as *const ::core::ffi::c_char) {
                (*tp).mode.fill = TPIC_MODE__FILL_SOLID;
            } else {
                pdf_warning!(
                    b"Invalid value for TPIC option fill-mode: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    v,
                );
                error = -(1 as ::core::ffi::c_int);
            }
        }
    } else {
        pdf_warning!(
            b"Unrecognized option for TPIC special handler: %s\0" as *const u8
                as *const ::core::ffi::c_char,
            k,
        );
        error = -(1 as ::core::ffi::c_int);
    }
    return error;
}
unsafe extern "C-unwind" fn spc_handler_tpic__setopts(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut tp: *mut spc_tpic_ = &raw mut _tpic_state;
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    dict = spc_parse_kvpairs(ap);
    if dict.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    error = pdf_foreach_dict(
        dict,
        Some(
            tpic_filter_getopts
                as unsafe extern "C-unwind" fn(
                    *mut pdf_obj,
                    *mut pdf_obj,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        tp as *mut ::core::ffi::c_void,
    );
    if error == 0 {
        if (*tp).mode.fill != TPIC_MODE__FILL_SOLID
            && pdf_check_version(1 as ::core::ffi::c_int, 4 as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
        {
            pdf_warning!(
                b"Transparent fill mode requires PDF version 1.4.\0" as *const u8
                    as *const ::core::ffi::c_char
            );
            (*tp).mode.fill = TPIC_MODE__FILL_SOLID;
        }
    }
    return error;
}
static mut tpic_handlers: [spc_handler; 13] = unsafe {
    [
        spc_handler {
            key: b"pn\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_pn
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"pa\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_pa
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"fp\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_fp
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"ip\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_ip
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"da\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_da
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"dt\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_dt
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"sp\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_sp
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"ar\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_ar
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"ia\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_ia
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"sh\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_sh
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"wh\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_wh
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"bk\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_bk
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"tx\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_tpic_tx
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
    ]
};
#[unsafe(export_name = "__pdf_impl_spc_tpic_check_special")]
pub unsafe extern "C-unwind" fn spc_tpic_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> bool {
    let mut istpic: bool = false_0 != 0;
    let mut hasnsp: bool = false_0 != 0;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: size_t = 0;
    p = buf;
    endptr = p.offset(len as isize);
    skip_blank(&raw mut p, endptr);
    if p.offset(strlen(b"tpic:\0" as *const u8 as *const ::core::ffi::c_char) as isize) < endptr
        && memcmp(
            p as *const ::core::ffi::c_void,
            b"tpic:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"tpic:\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0
    {
        p = p.offset(strlen(b"tpic:\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        hasnsp = true_0 != 0;
    }
    q = parse_c_ident(&raw mut p, endptr);
    if q.is_null() {
        istpic = false_0 != 0;
    } else if !q.is_null()
        && hasnsp as ::core::ffi::c_int != 0
        && streq_ptr(
            q,
            b"__setopt__\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        istpic = true_0 != 0;
        free(q as *mut ::core::ffi::c_void);
    } else {
        i = 0 as size_t;
        while i
            < (::core::mem::size_of::<[spc_handler; 13]>() as usize)
                .wrapping_div(::core::mem::size_of::<spc_handler>() as usize)
        {
            if streq_ptr(q, tpic_handlers[i as usize].key) {
                istpic = true_0 != 0;
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        free(q as *mut ::core::ffi::c_void);
    }
    return istpic;
}
#[unsafe(export_name = "__pdf_impl_spc_tpic_setup_handler")]
pub unsafe extern "C-unwind" fn spc_tpic_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_uint = 0;
    let mut hasnsp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut error: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
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
    if (*ap)
        .curptr
        .offset(strlen(b"tpic:\0" as *const u8 as *const ::core::ffi::c_char) as isize)
        < (*ap).endptr
        && memcmp(
            (*ap).curptr as *const ::core::ffi::c_void,
            b"tpic:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"tpic:\0" as *const u8 as *const ::core::ffi::c_char),
        ) == 0
    {
        (*ap).curptr = (*ap)
            .curptr
            .offset(strlen(b"tpic:\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        hasnsp = 1 as ::core::ffi::c_int;
    }
    q = parse_c_ident(&raw mut (*ap).curptr, (*ap).endptr);
    if q.is_null() {
        error = -(1 as ::core::ffi::c_int);
    } else if !q.is_null()
        && hasnsp != 0
        && streq_ptr(
            q,
            b"__setopt__\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
    {
        (*ap).command = b"__setopt__\0" as *const u8 as *const ::core::ffi::c_char;
        (*sph).key = b"tpic:\0" as *const u8 as *const ::core::ffi::c_char;
        (*sph).exec = Some(
            spc_handler_tpic__setopts
                as unsafe extern "C-unwind" fn(*mut spc_env, *mut spc_arg) -> ::core::ffi::c_int,
        ) as spc_handler_fn_ptr;
        skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
        error = 0 as ::core::ffi::c_int;
        free(q as *mut ::core::ffi::c_void);
    } else {
        i = 0 as ::core::ffi::c_uint;
        while (i as usize)
            < (::core::mem::size_of::<[spc_handler; 13]>() as usize)
                .wrapping_div(::core::mem::size_of::<spc_handler>() as usize)
        {
            if streq_ptr(q, tpic_handlers[i as usize].key) {
                (*ap).command = tpic_handlers[i as usize].key;
                (*sph).key = b"tpic:\0" as *const u8 as *const ::core::ffi::c_char;
                (*sph).exec = tpic_handlers[i as usize].exec;
                skip_blank(&raw mut (*ap).curptr, (*ap).endptr);
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
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tpic_set_fill_mode(mut mode: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tpic_set_fill_mode(mode) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_tpic_at_begin_page(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_tpic_at_begin_page() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_tpic_at_end_page(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_tpic_at_end_page() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_tpic_at_begin_document(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_tpic_at_begin_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_tpic_at_end_document(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_tpic_at_end_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_tpic_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_tpic_check_special(buf, len) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_tpic_setup_handler(
    mut sph: *mut spc_handler,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_tpic_setup_handler(sph, spe, ap) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
