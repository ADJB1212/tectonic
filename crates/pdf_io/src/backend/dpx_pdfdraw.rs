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
use crate::backend::dpx_pdfdev::{
    pdf_dev_get_param, pdf_sprint_coord, pdf_sprint_length, pdf_sprint_matrix, pdf_sprint_rect,
};
use crate::compat::allocation::free;
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
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_link_obj"]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_name_value"]
    fn pdf_name_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_get_array"]
    fn pdf_get_array(array: *mut pdf_obj, idx: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_array_length"]
    fn pdf_array_length(array: *mut pdf_obj) -> ::core::ffi::c_uint;
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_merge_dict"]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_lookup_dict"]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_dict_keys"]
    fn pdf_dict_keys(dict: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(
        dict: *mut pdf_obj,
        key: *mut pdf_obj,
        value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_compare_object"]
    fn pdf_compare_object(obj1: *mut pdf_obj, obj2: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_graycolor"]
    fn pdf_color_graycolor(color: *mut pdf_color, g: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_copycolor"]
    fn pdf_color_copycolor(color1: *mut pdf_color, color2: *const pdf_color);
    #[link_name = "__pdf_impl_pdf_color_compare"]
    fn pdf_color_compare(color1: *const pdf_color, color2: *const pdf_color) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_set_color"]
    fn pdf_color_set_color(
        color: *const pdf_color,
        buffer: *mut ::core::ffi::c_char,
        buff_len: size_t,
        mask: ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_reset_fonts"]
    fn pdf_dev_reset_fonts(newpage: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_graphics_mode"]
    fn graphics_mode();
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
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_doc_add_page_content"]
    fn pdf_doc_add_page_content(buffer: *const ::core::ffi::c_char, length: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_doc_add_page_resource"]
    fn pdf_doc_add_page_resource(
        category: *const ::core::ffi::c_char,
        resource_name: *const ::core::ffi::c_char,
        resources: *mut pdf_obj,
    );
    #[link_name = "__pdf_impl_parse_pdf_dict"]
    fn parse_pdf_dict(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
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
pub type pdf_gstate = pdf_gstate_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_gstate_ {
    pub cp: pdf_coord,
    pub matrix: pdf_tmatrix,
    pub strokecolor: pdf_color,
    pub fillcolor: pdf_color,
    pub linedash: C2RustUnnamed,
    pub linewidth: ::core::ffi::c_double,
    pub linecap: ::core::ffi::c_int,
    pub linejoin: ::core::ffi::c_int,
    pub miterlimit: ::core::ffi::c_double,
    pub flatness: ::core::ffi::c_int,
    pub path: pdf_path,
    pub flags: ::core::ffi::c_int,
    pub extgstate: *mut pdf_obj,
}
pub type pdf_path = pdf_path_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_path_ {
    pub num_paths: ::core::ffi::c_uint,
    pub max_paths: ::core::ffi::c_uint,
    pub path: *mut pa_elem,
}
pub type pa_elem = pa_elem_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_elem_ {
    pub type_0: ::core::ffi::c_int,
    pub p: [pdf_coord; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub num_dash: ::core::ffi::c_int,
    pub pattern: [::core::ffi::c_double; 16],
    pub offset: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xgs_res {
    pub object: *mut pdf_obj,
    pub accumlated: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub opchr: ::core::ffi::c_char,
    pub n_pts: ::core::ffi::c_int,
    pub strkey: *const ::core::ffi::c_char,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846264338327950288f64;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
pub const PDF_DEV_PARAM_COLORMODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_FILL_RULE_NONZERO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PDF_FILL_RULE_EVENODD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const OUR_EPSILON: ::core::ffi::c_double = 2.5e-16f64;
fn inverse_matrix(matrix: pdf_tmatrix) -> Option<pdf_tmatrix> {
    let determinant = matrix.a * matrix.d - matrix.b * matrix.c;
    if determinant.abs() < OUR_EPSILON {
        return None;
    }
    Some(pdf_tmatrix {
        a: matrix.d / determinant,
        b: -matrix.b / determinant,
        c: -matrix.c / determinant,
        d: matrix.a / determinant,
        e: (matrix.c * matrix.f - matrix.d * matrix.e) / determinant,
        f: (matrix.b * matrix.e - matrix.a * matrix.f) / determinant,
    })
}

fn inversematrix(output: &mut pdf_tmatrix, matrix: pdf_tmatrix) -> i32 {
    if let Some(inverse) = inverse_matrix(matrix) {
        *output = inverse;
        0
    } else {
        pdf_warning!("Inverting matrix with zero determinant...");
        -1
    }
}

fn pdf_coord__equal(p1: pdf_coord, p2: pdf_coord) -> i32 {
    i32::from((p1.x - p2.x).abs() < 1.0e-7 && (p1.y - p2.y).abs() < 1.0e-7)
}

fn pdf_coord__transform(point: &mut pdf_coord, matrix: pdf_tmatrix) -> i32 {
    let pdf_coord { x, y } = *point;
    point.x = x * matrix.a + y * matrix.c + matrix.e;
    point.y = x * matrix.b + y * matrix.d + matrix.f;
    0
}

fn pdf_coord__itransform(point: &mut pdf_coord, matrix: pdf_tmatrix) -> i32 {
    match inverse_matrix(matrix) {
        Some(inverse) => pdf_coord__transform(point, inverse),
        None => {
            pdf_warning!("Inverting matrix with zero determinant...");
            -1
        }
    }
}

fn pdf_coord__dtransform(point: &mut pdf_coord, matrix: pdf_tmatrix) -> i32 {
    let pdf_coord { x, y } = *point;
    point.x = x * matrix.a + y * matrix.c;
    point.y = x * matrix.b + y * matrix.d;
    0
}

fn pdf_coord__idtransform(point: &mut pdf_coord, matrix: pdf_tmatrix) -> i32 {
    match inverse_matrix(matrix) {
        Some(inverse) => pdf_coord__dtransform(point, inverse),
        None => {
            pdf_warning!("Inverting matrix with zero determinant...");
            -1
        }
    }
}

#[unsafe(export_name = "__pdf_impl_pdf_invertmatrix")]
pub unsafe extern "C-unwind" fn pdf_invertmatrix(matrix: *mut pdf_tmatrix) {
    if matrix.is_null() {
        pdf_abort!("assertion failed: M");
    }
    let matrix = unsafe { &mut *matrix };
    *matrix = inverse_matrix(*matrix).unwrap_or_else(|| {
        pdf_warning!("Inverting matrix with zero determinant...");
        pdf_tmatrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    });
}
const PETYPES: [C2RustUnnamed_0; 7] = [
    C2RustUnnamed_0 {
        opchr: 'm' as i32 as ::core::ffi::c_char,
        n_pts: 1 as ::core::ffi::c_int,
        strkey: b"moveto\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_0 {
        opchr: 'l' as i32 as ::core::ffi::c_char,
        n_pts: 1 as ::core::ffi::c_int,
        strkey: b"lineto\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_0 {
        opchr: 'c' as i32 as ::core::ffi::c_char,
        n_pts: 3 as ::core::ffi::c_int,
        strkey: b"curveto\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_0 {
        opchr: 'v' as i32 as ::core::ffi::c_char,
        n_pts: 2 as ::core::ffi::c_int,
        strkey: b"vcurveto\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_0 {
        opchr: 'y' as i32 as ::core::ffi::c_char,
        n_pts: 2 as ::core::ffi::c_int,
        strkey: b"ycurveto\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_0 {
        opchr: 'h' as i32 as ::core::ffi::c_char,
        n_pts: 0 as ::core::ffi::c_int,
        strkey: b"closepath\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_0 {
        opchr: ' ' as i32 as ::core::ffi::c_char,
        n_pts: 0 as ::core::ffi::c_int,
        strkey: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
pub const PE_TYPE__INVALID: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const PE_TYPE__MOVETO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PE_TYPE__LINETO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PE_TYPE__CURVETO: ::core::ffi::c_int = 2;
pub const PE_TYPE__CURVETO_V: ::core::ffi::c_int = 3;
pub const PE_TYPE__CURVETO_Y: ::core::ffi::c_int = 4;
pub const PE_TYPE__CLOSEPATH: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PE_TYPE__TERMINATE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const GS_FLAG_CURRENTPOINT_SET: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const FORMAT_BUFF_LEN: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

unsafe extern "C-unwind" fn init_a_path(mut p: *mut pdf_path) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*p).num_paths = 0 as ::core::ffi::c_uint;
    (*p).max_paths = 0 as ::core::ffi::c_uint;
    (*p).path = ::core::ptr::null_mut::<pa_elem>();
}
unsafe extern "C-unwind" fn pdf_path__clearpath(mut p: *mut pdf_path) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*p).num_paths = 0 as ::core::ffi::c_uint;
}
unsafe extern "C-unwind" fn pdf_path__growpath(
    mut p: *mut pdf_path,
    mut max_pe: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if max_pe <= (*p).max_paths {
        return 0 as ::core::ffi::c_int;
    }
    (*p).max_paths = crate::compat::allocation::grow_capacity(
        (*p).max_paths as usize,
        max_pe as usize,
        core::mem::size_of::<pa_elem>(),
        8,
    ) as u32;
    (*p).path = renew(
        (*p).path as *mut ::core::ffi::c_void,
        ((*p).max_paths as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<pa_elem>() as usize) as uint32_t,
    ) as *mut pa_elem;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn clear_a_path(mut p: *mut pdf_path) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*p).path = mfree((*p).path as *mut ::core::ffi::c_void) as *mut pa_elem;
    (*p).num_paths = 0 as ::core::ffi::c_uint;
    (*p).max_paths = 0 as ::core::ffi::c_uint;
}
unsafe extern "C-unwind" fn pdf_path__copypath(
    mut p1: *mut pdf_path,
    mut p0: *const pdf_path,
) -> ::core::ffi::c_int {
    let mut pe0: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    let mut pe1: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    let mut i: ::core::ffi::c_uint = 0;
    pdf_path__growpath(p1, (*p0).num_paths);
    i = 0 as ::core::ffi::c_uint;
    while i < (*p0).num_paths {
        pe1 = (*p1).path.offset(i as isize) as *mut pa_elem;
        pe0 = (*p0).path.offset(i as isize) as *mut pa_elem;
        (*pe1).type_0 = (*pe0).type_0;
        (*pe1).p[0 as ::core::ffi::c_int as usize].x = (*pe0).p[0 as ::core::ffi::c_int as usize].x;
        (*pe1).p[0 as ::core::ffi::c_int as usize].y = (*pe0).p[0 as ::core::ffi::c_int as usize].y;
        (*pe1).p[1 as ::core::ffi::c_int as usize].x = (*pe0).p[1 as ::core::ffi::c_int as usize].x;
        (*pe1).p[1 as ::core::ffi::c_int as usize].y = (*pe0).p[1 as ::core::ffi::c_int as usize].y;
        (*pe1).p[2 as ::core::ffi::c_int as usize].x = (*pe0).p[2 as ::core::ffi::c_int as usize].x;
        (*pe1).p[2 as ::core::ffi::c_int as usize].y = (*pe0).p[2 as ::core::ffi::c_int as usize].y;
        i = i.wrapping_add(1);
    }
    (*p1).num_paths = (*p0).num_paths;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdf_path__moveto(
    mut pa: *mut pdf_path,
    mut cp: *mut pdf_coord,
    mut p0: *const pdf_coord,
) -> ::core::ffi::c_int {
    let mut pe: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    pdf_path__growpath(pa, (*pa).num_paths.wrapping_add(1 as ::core::ffi::c_uint));
    if (*pa).num_paths > 0 as ::core::ffi::c_uint {
        pe = (*pa)
            .path
            .offset((*pa).num_paths.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
            as *mut pa_elem;
        if (*pe).type_0 == PE_TYPE__MOVETO {
            (*cp).x = (*p0).x;
            (*pe).p[0 as ::core::ffi::c_int as usize].x = (*cp).x;
            (*cp).y = (*p0).y;
            (*pe).p[0 as ::core::ffi::c_int as usize].y = (*cp).y;
            return 0 as ::core::ffi::c_int;
        }
    }
    let fresh6 = (*pa).num_paths;
    (*pa).num_paths = (*pa).num_paths.wrapping_add(1);
    pe = (*pa).path.offset(fresh6 as isize) as *mut pa_elem;
    (*pe).type_0 = PE_TYPE__MOVETO;
    (*cp).x = (*p0).x;
    (*pe).p[0 as ::core::ffi::c_int as usize].x = (*cp).x;
    (*cp).y = (*p0).y;
    (*pe).p[0 as ::core::ffi::c_int as usize].y = (*cp).y;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdf_path__next_pe(
    pa: *mut pdf_path,
    cp: *const pdf_coord,
) -> *mut pa_elem {
    let needed = unsafe { (*pa).num_paths }
        .checked_add(2)
        .unwrap_or_else(|| {
            pdf_abort!("Too many path elements");
        });
    unsafe { pdf_path__growpath(pa, needed) };
    let cp = unsafe { *cp };
    let path = unsafe { &mut *pa };
    let insert_move = if path.num_paths == 0 {
        true
    } else {
        let previous = unsafe { path.path.add(path.num_paths as usize - 1) };
        match unsafe { (*previous).type_0 } {
            PE_TYPE__MOVETO => {
                unsafe { (*previous).p[0] = cp };
                false
            }
            PE_TYPE__LINETO => pdf_coord__equal(unsafe { (*previous).p[0] }, cp) == 0,
            PE_TYPE__CURVETO => pdf_coord__equal(unsafe { (*previous).p[2] }, cp) == 0,
            PE_TYPE__CURVETO_Y | PE_TYPE__CURVETO_V => {
                pdf_coord__equal(unsafe { (*previous).p[1] }, cp) == 0
            }
            PE_TYPE__CLOSEPATH => true,
            _ => false,
        }
    };
    if insert_move {
        unsafe {
            path.path.add(path.num_paths as usize).write(pa_elem {
                type_0: PE_TYPE__MOVETO,
                p: [cp; 3],
            });
        }
        path.num_paths += 1;
    }
    let next = unsafe { path.path.add(path.num_paths as usize) };
    path.num_paths += 1;
    next
}
unsafe extern "C-unwind" fn pdf_path__transform(
    pa: *mut pdf_path,
    matrix: *const pdf_tmatrix,
) -> i32 {
    if pa.is_null() || matrix.is_null() {
        pdf_abort!("assertion failed: pa && M");
    }
    let matrix = unsafe { *matrix };
    let path = unsafe { &*pa };
    for index in 0..path.num_paths as usize {
        let element = unsafe { path.path.add(index) };
        let kind = unsafe { (*element).type_0 };
        let count = PETYPES
            .get(kind as usize)
            .map_or(0, |kind| kind.n_pts as usize);
        let points = unsafe {
            core::slice::from_raw_parts_mut((&raw mut (*element).p).cast::<pdf_coord>(), count)
        };
        for point in points.iter_mut().rev() {
            pdf_coord__transform(point, matrix);
        }
    }
    0
}
unsafe extern "C-unwind" fn pdf_path__lineto(
    mut pa: *mut pdf_path,
    mut cp: *mut pdf_coord,
    mut p0: *const pdf_coord,
) -> ::core::ffi::c_int {
    let mut pe: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    pe = pdf_path__next_pe(pa, cp);
    (*pe).type_0 = PE_TYPE__LINETO;
    (*cp).x = (*p0).x;
    (*pe).p[0 as ::core::ffi::c_int as usize].x = (*cp).x;
    (*cp).y = (*p0).y;
    (*pe).p[0 as ::core::ffi::c_int as usize].y = (*cp).y;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdf_path__curveto(
    mut pa: *mut pdf_path,
    mut cp: *mut pdf_coord,
    mut p0: *const pdf_coord,
    mut p1: *const pdf_coord,
    mut p2: *const pdf_coord,
) -> ::core::ffi::c_int {
    let mut pe: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    pe = pdf_path__next_pe(pa, cp);
    if pdf_coord__equal(unsafe { *cp }, unsafe { *p0 }) != 0 {
        (*pe).type_0 = PE_TYPE__CURVETO_V;
        (*pe).p[0 as ::core::ffi::c_int as usize].x = (*p1).x;
        (*pe).p[0 as ::core::ffi::c_int as usize].y = (*p1).y;
        (*cp).x = (*p2).x;
        (*pe).p[1 as ::core::ffi::c_int as usize].x = (*cp).x;
        (*cp).y = (*p2).y;
        (*pe).p[1 as ::core::ffi::c_int as usize].y = (*cp).y;
    } else if pdf_coord__equal(unsafe { *p1 }, unsafe { *p2 }) != 0 {
        (*pe).type_0 = PE_TYPE__CURVETO_Y;
        (*pe).p[0 as ::core::ffi::c_int as usize].x = (*p0).x;
        (*pe).p[0 as ::core::ffi::c_int as usize].y = (*p0).y;
        (*cp).x = (*p1).x;
        (*pe).p[1 as ::core::ffi::c_int as usize].x = (*cp).x;
        (*cp).y = (*p1).y;
        (*pe).p[1 as ::core::ffi::c_int as usize].y = (*cp).y;
    } else {
        (*pe).type_0 = PE_TYPE__CURVETO;
        (*pe).p[0 as ::core::ffi::c_int as usize].x = (*p0).x;
        (*pe).p[0 as ::core::ffi::c_int as usize].y = (*p0).y;
        (*pe).p[1 as ::core::ffi::c_int as usize].x = (*p1).x;
        (*pe).p[1 as ::core::ffi::c_int as usize].y = (*p1).y;
        (*cp).x = (*p2).x;
        (*pe).p[2 as ::core::ffi::c_int as usize].x = (*cp).x;
        (*cp).y = (*p2).y;
        (*pe).p[2 as ::core::ffi::c_int as usize].y = (*cp).y;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdf_path__elliptarc(
    mut pa: *mut pdf_path,
    mut cp: *mut pdf_coord,
    mut ca: *const pdf_coord,
    mut r_x: ::core::ffi::c_double,
    mut r_y: ::core::ffi::c_double,
    mut xar: ::core::ffi::c_double,
    mut a_0: ::core::ffi::c_double,
    mut a_1: ::core::ffi::c_double,
    mut a_d: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b: ::core::ffi::c_double = 0.;
    let mut b_x: ::core::ffi::c_double = 0.;
    let mut b_y: ::core::ffi::c_double = 0.;
    let mut d_a: ::core::ffi::c_double = 0.;
    let mut q: ::core::ffi::c_double = 0.;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p3: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut e0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut e1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut T: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut n_c: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if fabs(r_x) < OUR_EPSILON || fabs(r_y) < OUR_EPSILON {
        return -(1 as ::core::ffi::c_int);
    }
    if a_d < 0 as ::core::ffi::c_int {
        while a_1 > a_0 {
            a_1 -= 360.0f64;
        }
    } else {
        while a_1 < a_0 {
            a_0 -= 360.0f64;
        }
    }
    d_a = a_1 - a_0;
    n_c = 1 as ::core::ffi::c_int;
    while fabs(d_a) > 90.0f64 * n_c as ::core::ffi::c_double {
        n_c += 1;
    }
    d_a /= n_c as ::core::ffi::c_double;
    if fabs(d_a) < OUR_EPSILON {
        return -(1 as ::core::ffi::c_int);
    }
    a_0 *= M_PI / 180.0f64;
    a_1 *= M_PI / 180.0f64;
    d_a *= M_PI / 180.0f64;
    xar *= M_PI / 180.0f64;
    T.a = cos(xar);
    T.c = -sin(xar);
    T.b = -T.c;
    T.d = T.a;
    T.e = 0.0f64;
    T.f = 0.0f64;
    b = 4.0f64 * (1.0f64 - cos(0.5f64 * d_a)) / (3.0f64 * sin(0.5f64 * d_a));
    b_x = r_x * b;
    b_y = r_y * b;
    p0.x = r_x * cos(a_0);
    p0.y = r_y * sin(a_0);
    pdf_coord__transform(&mut p0, T);
    p0.x += (*ca).x;
    p0.y += (*ca).y;
    if (*pa).num_paths == 0 as ::core::ffi::c_uint {
        pdf_path__moveto(pa, cp, &raw mut p0);
    } else if pdf_coord__equal(unsafe { *cp }, p0) == 0 {
        pdf_path__lineto(pa, cp, &raw mut p0);
    }
    i = 0 as ::core::ffi::c_int;
    while error == 0 && i < n_c {
        q = a_0 + i as ::core::ffi::c_double * d_a;
        e0.x = cos(q);
        e0.y = sin(q);
        e1.x = cos(q + d_a);
        e1.y = sin(q + d_a);
        p0.x = r_x * e0.x;
        p0.y = r_y * e0.y;
        p3.x = r_x * e1.x;
        p3.y = r_y * e1.y;
        p1.x = -b_x * e0.y;
        p1.y = b_y * e0.x;
        p2.x = b_x * e1.y;
        p2.y = -b_y * e1.x;
        pdf_coord__transform(&mut p0, T);
        pdf_coord__transform(&mut p1, T);
        pdf_coord__transform(&mut p2, T);
        pdf_coord__transform(&mut p3, T);
        p0.x += (*ca).x;
        p0.y += (*ca).y;
        p3.x += (*ca).x;
        p3.y += (*ca).y;
        p1.x += p0.x;
        p1.y += p0.y;
        p2.x += p3.x;
        p2.y += p3.y;
        error = pdf_path__curveto(pa, &raw mut p0, &raw mut p1, &raw mut p2, &raw mut p3);
        (*cp).x = p3.x;
        (*cp).y = p3.y;
        i += 1;
    }
    return error;
}
unsafe extern "C-unwind" fn pdf_path__closepath(
    mut pa: *mut pdf_path,
    mut cp: *mut pdf_coord,
) -> ::core::ffi::c_int {
    let mut pe: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    let mut i: ::core::ffi::c_int = 0;
    i = (*pa).num_paths.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        pe = (*pa).path.offset(i as isize) as *mut pa_elem;
        if (*pe).type_0 == PE_TYPE__MOVETO {
            break;
        }
        i -= 1;
    }
    if pe.is_null() || i < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    (*cp).x = (*pe).p[0 as ::core::ffi::c_int as usize].x;
    (*cp).y = (*pe).p[0 as ::core::ffi::c_int as usize].y;
    pdf_path__growpath(pa, (*pa).num_paths.wrapping_add(1 as ::core::ffi::c_uint));
    let fresh7 = (*pa).num_paths;
    (*pa).num_paths = (*pa).num_paths.wrapping_add(1);
    pe = (*pa).path.offset(fresh7 as isize) as *mut pa_elem;
    (*pe).type_0 = PE_TYPE__CLOSEPATH;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdf_path__isarect(
    mut pa: *mut pdf_path,
    mut f_ir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pe0: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    let mut pe1: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    let mut pe2: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    let mut pe3: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    let mut pe4: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    if (*pa).num_paths == 5 as ::core::ffi::c_uint {
        pe0 = (*pa).path.offset(0 as ::core::ffi::c_int as isize) as *mut pa_elem;
        pe1 = (*pa).path.offset(1 as ::core::ffi::c_int as isize) as *mut pa_elem;
        pe2 = (*pa).path.offset(2 as ::core::ffi::c_int as isize) as *mut pa_elem;
        pe3 = (*pa).path.offset(3 as ::core::ffi::c_int as isize) as *mut pa_elem;
        pe4 = (*pa).path.offset(4 as ::core::ffi::c_int as isize) as *mut pa_elem;
        if (*pe0).type_0 == PE_TYPE__MOVETO
            && (*pe1).type_0 == PE_TYPE__LINETO
            && (*pe2).type_0 == PE_TYPE__LINETO
            && (*pe3).type_0 == PE_TYPE__LINETO
            && (*pe4).type_0 == PE_TYPE__CLOSEPATH
        {
            if (*pe1).p[0 as ::core::ffi::c_int as usize].y
                - (*pe0).p[0 as ::core::ffi::c_int as usize].y
                == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && (*pe2).p[0 as ::core::ffi::c_int as usize].x
                    - (*pe1).p[0 as ::core::ffi::c_int as usize].x
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && (*pe3).p[0 as ::core::ffi::c_int as usize].y
                    - (*pe2).p[0 as ::core::ffi::c_int as usize].y
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                if (*pe1).p[0 as ::core::ffi::c_int as usize].x
                    - (*pe0).p[0 as ::core::ffi::c_int as usize].x
                    == (*pe2).p[0 as ::core::ffi::c_int as usize].x
                        - (*pe3).p[0 as ::core::ffi::c_int as usize].x
                {
                    return 1 as ::core::ffi::c_int;
                }
            } else if f_ir != 0
                && (*pe1).p[0 as ::core::ffi::c_int as usize].x
                    - (*pe0).p[0 as ::core::ffi::c_int as usize].x
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && (*pe2).p[0 as ::core::ffi::c_int as usize].y
                    - (*pe1).p[0 as ::core::ffi::c_int as usize].y
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                && (*pe3).p[0 as ::core::ffi::c_int as usize].x
                    - (*pe2).p[0 as ::core::ffi::c_int as usize].x
                    == 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                if (*pe1).p[0 as ::core::ffi::c_int as usize].y
                    - (*pe0).p[0 as ::core::ffi::c_int as usize].y
                    == (*pe2).p[0 as ::core::ffi::c_int as usize].y
                        - (*pe3).p[0 as ::core::ffi::c_int as usize].y
                {
                    return 1 as ::core::ffi::c_int;
                }
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
fn INVERTIBLE_MATRIX(matrix: pdf_tmatrix) -> i32 {
    if (matrix.a * matrix.d - matrix.b * matrix.c).abs() < OUR_EPSILON {
        pdf_warning!("Transformation matrix not invertible.");
        pdf_warning!(
            "--- M = [{} {} {} {} {} {}]",
            matrix.a,
            matrix.b,
            matrix.c,
            matrix.d,
            matrix.e,
            matrix.f,
        );
        -1
    } else {
        0
    }
}
unsafe extern "C-unwind" fn pdf_dev__rectshape(
    mut r: *const pdf_rect,
    mut M: *const pdf_tmatrix,
    mut opchr: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut buf: *mut ::core::ffi::c_char =
        &raw mut (*__pdf_state).fmt_buf as *mut ::core::ffi::c_char;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut isclip: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut wd: ::core::ffi::c_double = 0.;
    let mut ht: ::core::ffi::c_double = 0.;
    if !(!r.is_null()
        && (opchr as ::core::ffi::c_int == 'f' as i32
            || opchr as ::core::ffi::c_int == 'F' as i32
            || opchr as ::core::ffi::c_int == 's' as i32
            || opchr as ::core::ffi::c_int == 'S' as i32
            || opchr as ::core::ffi::c_int == 'b' as i32
            || opchr as ::core::ffi::c_int == 'B' as i32
            || opchr as ::core::ffi::c_int == 'W' as i32)) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"r && PT_OP_VALID(opchr)\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    isclip = if opchr as ::core::ffi::c_int == 'W' as i32 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if !M.is_null() && (isclip != 0 || INVERTIBLE_MATRIX(unsafe { *M }) == 0) {
        return -(1 as ::core::ffi::c_int);
    }
    graphics_mode();
    if isclip == 0 {
        pdf_dev_gsave();
    }
    pdf_dev_newpath();
    if isclip == 0 && !M.is_null() {
        pdf_dev_concat(M);
    }
    p.x = (*r).llx;
    p.y = (*r).lly;
    wd = (*r).urx - (*r).llx;
    ht = (*r).ury - (*r).lly;
    let fresh25 = len;
    len = len + 1;
    *buf.offset(fresh25 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len += pdf_sprint_coord(buf.offset(len as isize), &raw mut p);
    let fresh26 = len;
    len = len + 1;
    *buf.offset(fresh26 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len += pdf_sprint_length(buf.offset(len as isize), wd);
    let fresh27 = len;
    len = len + 1;
    *buf.offset(fresh27 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len += pdf_sprint_length(buf.offset(len as isize), ht);
    let fresh28 = len;
    len = len + 1;
    *buf.offset(fresh28 as isize) = ' ' as i32 as ::core::ffi::c_char;
    let fresh29 = len;
    len = len + 1;
    *buf.offset(fresh29 as isize) = 'r' as i32 as ::core::ffi::c_char;
    let fresh30 = len;
    len = len + 1;
    *buf.offset(fresh30 as isize) = 'e' as i32 as ::core::ffi::c_char;
    let fresh31 = len;
    len = len + 1;
    *buf.offset(fresh31 as isize) = ' ' as i32 as ::core::ffi::c_char;
    let fresh32 = len;
    len = len + 1;
    *buf.offset(fresh32 as isize) = opchr;
    pdf_doc_add_page_content(buf, len as ::core::ffi::c_uint);
    if isclip != 0 {
        pdf_dev_newpath();
    } else {
        pdf_dev_grestore();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdf_dev__flushpath(
    mut pa: *mut pdf_path,
    mut opchr: ::core::ffi::c_char,
    mut rule: ::core::ffi::c_int,
    mut ignore_rule: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut pe: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    let mut pe1: *mut pa_elem = ::core::ptr::null_mut::<pa_elem>();
    let mut b: *mut ::core::ffi::c_char =
        &raw mut (*__pdf_state).fmt_buf as *mut ::core::ffi::c_char;
    let mut b_len: ::core::ffi::c_int = FORMAT_BUFF_LEN;
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut pt: *mut pdf_coord = ::core::ptr::null_mut::<pdf_coord>();
    let mut n_pts: ::core::ffi::c_int = 0;
    let mut n_seg: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut isrect: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if !(!pa.is_null()
        && (opchr as ::core::ffi::c_int == 'f' as i32
            || opchr as ::core::ffi::c_int == 'F' as i32
            || opchr as ::core::ffi::c_int == 's' as i32
            || opchr as ::core::ffi::c_int == 'S' as i32
            || opchr as ::core::ffi::c_int == 'b' as i32
            || opchr as ::core::ffi::c_int == 'B' as i32
            || opchr as ::core::ffi::c_int == 'W' as i32)) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"pa && PT_OP_VALID(opchr)\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*pa).num_paths <= 0 as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    graphics_mode();
    isrect = pdf_path__isarect(pa, ignore_rule);
    if isrect != 0 {
        pe = (*pa).path.offset(0 as ::core::ffi::c_int as isize) as *mut pa_elem;
        pe1 = (*pa).path.offset(2 as ::core::ffi::c_int as isize) as *mut pa_elem;
        r.llx = (*pe).p[0 as ::core::ffi::c_int as usize].x;
        r.lly = (*pe).p[0 as ::core::ffi::c_int as usize].y;
        r.urx = (*pe1).p[0 as ::core::ffi::c_int as usize].x
            - (*pe).p[0 as ::core::ffi::c_int as usize].x;
        r.ury = (*pe1).p[0 as ::core::ffi::c_int as usize].y
            - (*pe).p[0 as ::core::ffi::c_int as usize].y;
        let fresh15 = len;
        len = len + 1;
        *b.offset(fresh15 as isize) = ' ' as i32 as ::core::ffi::c_char;
        len += pdf_sprint_rect(b.offset(len as isize), &raw mut r);
        let fresh16 = len;
        len = len + 1;
        *b.offset(fresh16 as isize) = ' ' as i32 as ::core::ffi::c_char;
        let fresh17 = len;
        len = len + 1;
        *b.offset(fresh17 as isize) = 'r' as i32 as ::core::ffi::c_char;
        let fresh18 = len;
        len = len + 1;
        *b.offset(fresh18 as isize) = 'e' as i32 as ::core::ffi::c_char;
        pdf_doc_add_page_content(b, len as ::core::ffi::c_uint);
        len = 0 as ::core::ffi::c_int;
    } else {
        n_seg = (*pa).num_paths as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        len = 0 as ::core::ffi::c_int;
        pe = (*pa).path.offset(0 as ::core::ffi::c_int as isize) as *mut pa_elem;
        while i < n_seg {
            n_pts = if !pe.is_null()
                && (*pe).type_0 > PE_TYPE__INVALID
                && (*pe).type_0 < PE_TYPE__TERMINATE
            {
                PETYPES[(*pe).type_0 as usize].n_pts
            } else {
                0 as ::core::ffi::c_int
            };
            j = 0 as ::core::ffi::c_int;
            pt = (&raw mut (*pe).p as *mut pdf_coord).offset(0 as ::core::ffi::c_int as isize)
                as *mut pdf_coord;
            while j < n_pts {
                let fresh19 = len;
                len = len + 1;
                *b.offset(fresh19 as isize) = ' ' as i32 as ::core::ffi::c_char;
                len += pdf_sprint_coord(b.offset(len as isize), pt);
                j += 1;
                pt = pt.offset(1);
            }
            let fresh20 = len;
            len = len + 1;
            *b.offset(fresh20 as isize) = ' ' as i32 as ::core::ffi::c_char;
            let fresh21 = len;
            len = len + 1;
            *b.offset(fresh21 as isize) = (if !pe.is_null()
                && (*pe).type_0 > PE_TYPE__INVALID
                && (*pe).type_0 < PE_TYPE__TERMINATE
            {
                PETYPES[(*pe).type_0 as usize].opchr as ::core::ffi::c_int
            } else {
                ' ' as i32
            }) as ::core::ffi::c_char;
            if len + 128 as ::core::ffi::c_int > b_len {
                pdf_doc_add_page_content(b, len as ::core::ffi::c_uint);
                len = 0 as ::core::ffi::c_int;
            }
            pe = pe.offset(1);
            i += 1;
        }
        if len > 0 as ::core::ffi::c_int {
            pdf_doc_add_page_content(b, len as ::core::ffi::c_uint);
            len = 0 as ::core::ffi::c_int;
        }
    }
    let fresh22 = len;
    len = len + 1;
    *b.offset(fresh22 as isize) = ' ' as i32 as ::core::ffi::c_char;
    let fresh23 = len;
    len = len + 1;
    *b.offset(fresh23 as isize) = opchr;
    if rule == PDF_FILL_RULE_EVENODD {
        let fresh24 = len;
        len = len + 1;
        *b.offset(fresh24 as isize) = '*' as i32 as ::core::ffi::c_char;
    }
    pdf_doc_add_page_content(b, len as ::core::ffi::c_uint);
    return 0 as ::core::ffi::c_int;
}

const DEFAULT_XGS: &[u8] = b"<< /Type /ExtGState   /LW 1 /LC 0 /LJ 0 /ML 10 /D [[] 0]   /RI /RelativeColorimetric /SA false /BM /Normal /SMask /None   /AIS false /TK false /CA 1 /ca 1   /OP false /op false /OPM 0 /FL 1>>\0";
unsafe extern "C-unwind" fn init_xgstate() {
    let __pdf_state = unsafe { document_state() };
    dpx_stack_init(&raw mut (*__pdf_state).xgs_stack);
    (*__pdf_state).xgs_count = 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn clear_xgstate() {
    let __pdf_state = unsafe { document_state() };
    let mut xgs: *mut xgs_res = ::core::ptr::null_mut::<xgs_res>();
    loop {
        xgs = dpx_stack_pop(&raw mut (*__pdf_state).xgs_stack) as *mut xgs_res;
        if xgs.is_null() {
            break;
        }
        pdf_release_obj((*xgs).object);
        pdf_release_obj((*xgs).accumlated);
        free(xgs as *mut ::core::ffi::c_void);
    }
}

unsafe extern "C-unwind" fn init_a_gstate(mut gs: *mut pdf_gstate) {
    (*gs).cp.x = 0.0f64;
    (*gs).cp.y = 0.0f64;
    (*gs).matrix.a = 1.0f64;
    (*gs).matrix.b = 0.0f64;
    (*gs).matrix.c = 0.0f64;
    (*gs).matrix.d = 1.0f64;
    (*gs).matrix.e = 0.0f64;
    (*gs).matrix.f = 0.0f64;
    pdf_color_graycolor(&raw mut (*gs).strokecolor, 0.0f64);
    pdf_color_graycolor(&raw mut (*gs).fillcolor, 0.0f64);
    (*gs).linedash.num_dash = 0 as ::core::ffi::c_int;
    (*gs).linedash.offset = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*gs).linecap = 0 as ::core::ffi::c_int;
    (*gs).linejoin = 0 as ::core::ffi::c_int;
    (*gs).linewidth = 1.0f64;
    (*gs).miterlimit = 10.0f64;
    (*gs).flatness = 1 as ::core::ffi::c_int;
    (*gs).flags = 0 as ::core::ffi::c_int;
    init_a_path(&raw mut (*gs).path);
    (*gs).extgstate = ::core::ptr::null_mut::<pdf_obj>();
}
unsafe extern "C-unwind" fn clear_a_gstate(mut gs: *mut pdf_gstate) {
    clear_a_path(&raw mut (*gs).path);
    if !(*gs).extgstate.is_null() {
        pdf_release_obj((*gs).extgstate);
    }
    memset(
        gs as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pdf_gstate>() as size_t,
    );
}
unsafe extern "C-unwind" fn copy_a_gstate(mut gs1: *mut pdf_gstate, mut gs2: *mut pdf_gstate) {
    let mut i: ::core::ffi::c_int = 0;
    if !(!gs1.is_null() && !gs2.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"gs1 && gs2\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*gs1).cp.x = (*gs2).cp.x;
    (*gs1).cp.y = (*gs2).cp.y;
    (*gs1).matrix.a = (*gs2).matrix.a;
    (*gs1).matrix.b = (*gs2).matrix.b;
    (*gs1).matrix.c = (*gs2).matrix.c;
    (*gs1).matrix.d = (*gs2).matrix.d;
    (*gs1).matrix.e = (*gs2).matrix.e;
    (*gs1).matrix.f = (*gs2).matrix.f;
    pdf_path__copypath(&raw mut (*gs1).path, &raw mut (*gs2).path);
    (*gs1).linedash.num_dash = (*gs2).linedash.num_dash;
    i = 0 as ::core::ffi::c_int;
    while i < (*gs2).linedash.num_dash {
        (*gs1).linedash.pattern[i as usize] = (*gs2).linedash.pattern[i as usize];
        i += 1;
    }
    (*gs1).linedash.offset = (*gs2).linedash.offset;
    (*gs1).linecap = (*gs2).linecap;
    (*gs1).linejoin = (*gs2).linejoin;
    (*gs1).linewidth = (*gs2).linewidth;
    (*gs1).miterlimit = (*gs2).miterlimit;
    (*gs1).flatness = (*gs2).flatness;
    pdf_color_copycolor(&raw mut (*gs1).fillcolor, &raw mut (*gs2).fillcolor);
    pdf_color_copycolor(&raw mut (*gs1).strokecolor, &raw mut (*gs2).strokecolor);
    (*gs1).extgstate = if !(*gs2).extgstate.is_null() {
        pdf_link_obj((*gs2).extgstate)
    } else {
        ::core::ptr::null_mut::<pdf_obj>()
    };
}
unsafe extern "C-unwind" fn pdf_dev_set_xgstate(
    mut diff: *mut pdf_obj,
    mut accumlated: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut buf: [::core::ffi::c_char; 64] = [0; 64];
    let mut res_name: [::core::ffi::c_char; 16] = [0; 16];
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut id: ::core::ffi::c_int = 0;
    id = (*__pdf_state).xgs_count;
    snprintf(
        &raw mut res_name as *mut ::core::ffi::c_char,
        16 as size_t,
        b"DPX_GS%d\0" as *const u8 as *const ::core::ffi::c_char,
        id,
    );
    res_name[15 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    len += snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        64 as size_t,
        b" /%s gs\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut res_name as *mut ::core::ffi::c_char,
    );
    pdf_doc_add_page_content(
        &raw mut buf as *mut ::core::ffi::c_char,
        len as ::core::ffi::c_uint,
    );
    pdf_doc_add_page_resource(
        b"ExtGState\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut res_name as *mut ::core::ffi::c_char,
        pdf_link_obj(diff),
    );
    if !(*gs).extgstate.is_null() {
        pdf_release_obj((*gs).extgstate);
    }
    (*gs).extgstate = pdf_link_obj(accumlated);
    (*__pdf_state).xgs_count += 1;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_reset_xgstate")]
pub unsafe extern "C-unwind" fn pdf_dev_reset_xgstate(
    mut force: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = ::core::ptr::null_mut::<pdf_gstate>();
    let mut current: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut target: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut keys: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut diff: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut xgs: *mut xgs_res = ::core::ptr::null_mut::<xgs_res>();
    let mut i: ::core::ffi::c_int = 0;
    let mut need_reset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    gs = dpx_stack_top(gss) as *mut pdf_gstate;
    xgs = dpx_stack_top(&raw mut (*__pdf_state).xgs_stack) as *mut xgs_res;
    if !xgs.is_null() {
        target = pdf_link_obj((*xgs).accumlated);
    } else {
        let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if (*gs).extgstate.is_null() && force == 0 {
            return 0 as ::core::ffi::c_int;
        }
        ptr = DEFAULT_XGS.as_ptr() as *const ::core::ffi::c_char;
        endptr = ptr.offset(strlen(ptr) as isize);
        target = parse_pdf_dict(&raw mut ptr, endptr, ::core::ptr::null_mut::<pdf_file>());
    }
    if !(*gs).extgstate.is_null() {
        current = pdf_link_obj((*gs).extgstate);
    } else {
        let mut ptr_0: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut endptr_0: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        ptr_0 = DEFAULT_XGS.as_ptr() as *const ::core::ffi::c_char;
        endptr_0 = ptr_0.offset(strlen(ptr_0) as isize);
        current = parse_pdf_dict(
            &raw mut ptr_0,
            endptr_0,
            ::core::ptr::null_mut::<pdf_file>(),
        );
    }
    diff = pdf_new_dict();
    keys = pdf_dict_keys(target);
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < pdf_array_length(keys) {
        let mut key: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut value1: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut value2: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut is_diff: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        key = pdf_get_array(keys, i);
        value1 = pdf_lookup_dict(target, pdf_name_value(key));
        value2 = pdf_lookup_dict(current, pdf_name_value(key));
        is_diff = pdf_compare_object(value1, value2);
        if is_diff != 0 {
            pdf_add_dict(diff, pdf_link_obj(key), pdf_link_obj(value1));
            need_reset = 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    pdf_release_obj(keys);
    if need_reset != 0 {
        pdf_dev_set_xgstate(diff, target);
    }
    pdf_release_obj(diff);
    pdf_release_obj(current);
    pdf_release_obj(target);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_xgstate_push")]
pub unsafe extern "C-unwind" fn pdf_dev_xgstate_push(mut object: *mut pdf_obj) {
    let __pdf_state = unsafe { document_state() };
    let mut target: *mut xgs_res = ::core::ptr::null_mut::<xgs_res>();
    let mut current: *mut xgs_res = ::core::ptr::null_mut::<xgs_res>();
    let mut accumlated: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    target = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<xgs_res>() as usize) as uint32_t)
        as *mut xgs_res;
    (*target).object = object;
    current = dpx_stack_top(&raw mut (*__pdf_state).xgs_stack) as *mut xgs_res;
    if current.is_null() {
        let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        ptr = DEFAULT_XGS.as_ptr() as *const ::core::ffi::c_char;
        endptr = ptr.offset(strlen(ptr) as isize);
        accumlated = parse_pdf_dict(&raw mut ptr, endptr, ::core::ptr::null_mut::<pdf_file>());
    } else {
        accumlated = pdf_new_dict();
        pdf_merge_dict(accumlated, (*current).accumlated);
    }
    pdf_merge_dict(accumlated, object);
    (*target).accumlated = accumlated;
    dpx_stack_push(
        &raw mut (*__pdf_state).xgs_stack,
        target as *mut ::core::ffi::c_void,
    );
    pdf_dev_set_xgstate((*target).object, (*target).accumlated);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_xgstate_pop")]
pub unsafe extern "C-unwind" fn pdf_dev_xgstate_pop() {
    let __pdf_state = unsafe { document_state() };
    let mut current: *mut xgs_res = ::core::ptr::null_mut::<xgs_res>();
    let mut target: *mut xgs_res = ::core::ptr::null_mut::<xgs_res>();
    let mut accumlated: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut revert: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut keys: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut i: ::core::ffi::c_int = 0;
    current = dpx_stack_pop(&raw mut (*__pdf_state).xgs_stack) as *mut xgs_res;
    target = dpx_stack_top(&raw mut (*__pdf_state).xgs_stack) as *mut xgs_res;
    if current.is_null() {
        pdf_warning!(
            b"Too many pop operation for ExtGState!\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    if target.is_null() {
        let mut ptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        ptr = DEFAULT_XGS.as_ptr() as *const ::core::ffi::c_char;
        endptr = ptr.offset(strlen(ptr) as isize);
        accumlated = parse_pdf_dict(&raw mut ptr, endptr, ::core::ptr::null_mut::<pdf_file>());
    } else {
        accumlated = pdf_link_obj((*target).accumlated);
    }
    keys = pdf_dict_keys((*current).object);
    revert = pdf_new_dict();
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < pdf_array_length(keys) {
        let mut key: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut value: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        key = pdf_get_array(keys, i);
        value = pdf_lookup_dict(accumlated, pdf_name_value(key));
        if value.is_null() {
            pdf_warning!(
                b"No previous ExtGState entry known for \"%s\", ignoring...\0" as *const u8
                    as *const ::core::ffi::c_char,
                pdf_name_value(key),
            );
        } else {
            pdf_add_dict(revert, pdf_link_obj(key), pdf_link_obj(value));
        }
        i += 1;
    }
    pdf_dev_set_xgstate(revert, accumlated);
    pdf_release_obj(revert);
    pdf_release_obj(keys);
    pdf_release_obj(accumlated);
    pdf_release_obj((*current).object);
    pdf_release_obj((*current).accumlated);
    free(current as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_init_gstates")]
pub unsafe extern "C-unwind" fn pdf_dev_init_gstates() {
    let __pdf_state = unsafe { document_state() };
    let mut gs: *mut pdf_gstate = ::core::ptr::null_mut::<pdf_gstate>();
    loop {
        gs = dpx_stack_pop(&raw mut (*__pdf_state).gs_stack) as *mut pdf_gstate;
        if gs.is_null() {
            break;
        }
        clear_a_gstate(gs);
        free(gs as *mut ::core::ffi::c_void);
    }
    dpx_stack_init(&raw mut (*__pdf_state).gs_stack);
    gs = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_gstate>() as usize) as uint32_t)
        as *mut pdf_gstate;
    init_a_gstate(gs);
    dpx_stack_push(
        &raw mut (*__pdf_state).gs_stack,
        gs as *mut ::core::ffi::c_void,
    );
    init_xgstate();
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_clear_gstates")]
pub unsafe extern "C-unwind" fn pdf_dev_clear_gstates() {
    let __pdf_state = unsafe { document_state() };
    let mut gs: *mut pdf_gstate = ::core::ptr::null_mut::<pdf_gstate>();
    if dpx_stack_depth(&raw mut (*__pdf_state).gs_stack) > 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"GS stack depth is not zero at the end of the document.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    loop {
        gs = dpx_stack_pop(&raw mut (*__pdf_state).gs_stack) as *mut pdf_gstate;
        if gs.is_null() {
            break;
        }
        clear_a_gstate(gs);
        free(gs as *mut ::core::ffi::c_void);
    }
    clear_xgstate();
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_gsave")]
pub unsafe extern "C-unwind" fn pdf_dev_gsave() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gs0: *mut pdf_gstate = ::core::ptr::null_mut::<pdf_gstate>();
    let mut gs1: *mut pdf_gstate = ::core::ptr::null_mut::<pdf_gstate>();
    gs0 = dpx_stack_top(&raw mut (*__pdf_state).gs_stack) as *mut pdf_gstate;
    gs1 = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_gstate>() as usize) as uint32_t)
        as *mut pdf_gstate;
    init_a_gstate(gs1);
    copy_a_gstate(gs1, gs0);
    dpx_stack_push(
        &raw mut (*__pdf_state).gs_stack,
        gs1 as *mut ::core::ffi::c_void,
    );
    pdf_doc_add_page_content(
        b" q\0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_uint,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_grestore")]
pub unsafe extern "C-unwind" fn pdf_dev_grestore() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gs: *mut pdf_gstate = ::core::ptr::null_mut::<pdf_gstate>();
    if dpx_stack_depth(&raw mut (*__pdf_state).gs_stack) <= 1 as ::core::ffi::c_int {
        pdf_warning!(b"Too many grestores.\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    gs = dpx_stack_pop(&raw mut (*__pdf_state).gs_stack) as *mut pdf_gstate;
    clear_a_gstate(gs);
    free(gs as *mut ::core::ffi::c_void);
    pdf_doc_add_page_content(
        b" Q\0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_uint,
    );
    pdf_dev_reset_fonts(0 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_push_gstate")]
pub unsafe extern "C-unwind" fn pdf_dev_push_gstate() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs0: *mut pdf_gstate = ::core::ptr::null_mut::<pdf_gstate>();
    gs0 = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_gstate>() as usize) as uint32_t)
        as *mut pdf_gstate;
    init_a_gstate(gs0);
    dpx_stack_push(gss, gs0 as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_pop_gstate")]
pub unsafe extern "C-unwind" fn pdf_dev_pop_gstate() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = ::core::ptr::null_mut::<pdf_gstate>();
    if dpx_stack_depth(gss) <= 1 as ::core::ffi::c_int {
        pdf_warning!(b"Too many grestores.\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    gs = dpx_stack_pop(gss) as *mut pdf_gstate;
    clear_a_gstate(gs);
    free(gs as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_current_depth")]
pub unsafe extern "C-unwind" fn pdf_dev_current_depth() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    return dpx_stack_depth(&raw mut (*__pdf_state).gs_stack) - 1 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_grestore_to")]
pub unsafe extern "C-unwind" fn pdf_dev_grestore_to(mut depth: ::core::ffi::c_int) {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = ::core::ptr::null_mut::<pdf_gstate>();
    if !(depth >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"depth >= 0\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if dpx_stack_depth(gss) > depth + 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"Closing pending transformations at end of page/XObject.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    while dpx_stack_depth(gss) > depth + 1 as ::core::ffi::c_int {
        pdf_doc_add_page_content(
            b" Q\0" as *const u8 as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_uint,
        );
        gs = dpx_stack_pop(gss) as *mut pdf_gstate;
        clear_a_gstate(gs);
        free(gs as *mut ::core::ffi::c_void);
    }
    pdf_dev_reset_fonts(0 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_currentpoint")]
pub unsafe extern "C-unwind" fn pdf_dev_currentpoint(mut p: *mut pdf_coord) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*p).x = (*cpt).x;
    (*p).y = (*cpt).y;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_currentmatrix")]
pub unsafe extern "C-unwind" fn pdf_dev_currentmatrix(
    mut M: *mut pdf_tmatrix,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut CTM: *mut pdf_tmatrix = &raw mut (*gs).matrix;
    if M.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"M\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*M).a = (*CTM).a;
    (*M).b = (*CTM).b;
    (*M).c = (*CTM).c;
    (*M).d = (*CTM).d;
    (*M).e = (*CTM).e;
    (*M).f = (*CTM).f;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_set_color")]
pub unsafe extern "C-unwind" fn pdf_dev_set_color(
    mut color: *const pdf_color,
    mut mask: ::core::ffi::c_char,
    mut force: ::core::ffi::c_int,
) {
    let __pdf_state = unsafe { document_state() };
    let mut len: ::core::ffi::c_int = 0;
    let mut gs: *mut pdf_gstate =
        dpx_stack_top(&raw mut (*__pdf_state).gs_stack) as *mut pdf_gstate;
    let mut current: *mut pdf_color = if mask as ::core::ffi::c_int != 0 {
        &raw mut (*gs).fillcolor
    } else {
        &raw mut (*gs).strokecolor
    };
    if !(pdf_dev_get_param(PDF_DEV_PARAM_COLORMODE) != 0
        && (force != 0 || pdf_color_compare(color, current) != 0))
    {
        return;
    }
    graphics_mode();
    len = pdf_color_set_color(
        color,
        &raw mut (*__pdf_state).fmt_buf as *mut ::core::ffi::c_char,
        FORMAT_BUFF_LEN as size_t,
        mask,
    );
    pdf_doc_add_page_content(
        &raw mut (*__pdf_state).fmt_buf as *mut ::core::ffi::c_char,
        len as ::core::ffi::c_uint,
    );
    pdf_color_copycolor(current, color);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_concat")]
pub unsafe extern "C-unwind" fn pdf_dev_concat(mut M: *const pdf_tmatrix) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut CTM: *mut pdf_tmatrix = &raw mut (*gs).matrix;
    let mut W: pdf_tmatrix = pdf_tmatrix {
        a: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        b: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        c: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        d: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        e: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        f: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    };
    let mut buf: *mut ::core::ffi::c_char =
        &raw mut (*__pdf_state).fmt_buf as *mut ::core::ffi::c_char;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if M.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"M\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if fabs((*M).a * (*M).d - (*M).b * (*M).c) < OUR_EPSILON {
        pdf_warning!(
            b"Transformation matrix not invertible.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        pdf_warning!(
            b"--- M = [%g %g %g %g %g %g]\0" as *const u8 as *const ::core::ffi::c_char,
            (*M).a,
            (*M).b,
            (*M).c,
            (*M).d,
            (*M).e,
            (*M).f,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if fabs((*M).a - 1.0f64) > OUR_EPSILON
        || fabs((*M).b) > OUR_EPSILON
        || fabs((*M).c) > OUR_EPSILON
        || fabs((*M).d - 1.0f64) > OUR_EPSILON
        || fabs((*M).e) > OUR_EPSILON
        || fabs((*M).f) > OUR_EPSILON
    {
        let fresh33 = len;
        len = len + 1;
        *buf.offset(fresh33 as isize) = ' ' as i32 as ::core::ffi::c_char;
        len += pdf_sprint_matrix(buf.offset(len as isize), M);
        let fresh34 = len;
        len = len + 1;
        *buf.offset(fresh34 as isize) = ' ' as i32 as ::core::ffi::c_char;
        let fresh35 = len;
        len = len + 1;
        *buf.offset(fresh35 as isize) = 'c' as i32 as ::core::ffi::c_char;
        let fresh36 = len;
        len = len + 1;
        *buf.offset(fresh36 as isize) = 'm' as i32 as ::core::ffi::c_char;
        pdf_doc_add_page_content(buf, len as ::core::ffi::c_uint);
        let mut _tmp_a: ::core::ffi::c_double = 0.;
        let mut _tmp_b: ::core::ffi::c_double = 0.;
        let mut _tmp_c: ::core::ffi::c_double = 0.;
        let mut _tmp_d: ::core::ffi::c_double = 0.;
        _tmp_a = (*CTM).a;
        _tmp_b = (*CTM).b;
        _tmp_c = (*CTM).c;
        _tmp_d = (*CTM).d;
        (*CTM).a = (*M).a * _tmp_a + (*M).b * _tmp_c;
        (*CTM).b = (*M).a * _tmp_b + (*M).b * _tmp_d;
        (*CTM).c = (*M).c * _tmp_a + (*M).d * _tmp_c;
        (*CTM).d = (*M).c * _tmp_b + (*M).d * _tmp_d;
        (*CTM).e += (*M).e * _tmp_a + (*M).f * _tmp_c;
        (*CTM).f += (*M).e * _tmp_b + (*M).f * _tmp_d;
    }
    inversematrix(&mut W, unsafe { *M });
    pdf_path__transform(cpa, &raw mut W);
    pdf_coord__transform(unsafe { &mut *cpt }, W);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_setmiterlimit")]
pub unsafe extern "C-unwind" fn pdf_dev_setmiterlimit(
    mut mlimit: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut buf: *mut ::core::ffi::c_char =
        &raw mut (*__pdf_state).fmt_buf as *mut ::core::ffi::c_char;
    if (*gs).miterlimit != mlimit {
        let fresh3 = len;
        len = len + 1;
        *buf.offset(fresh3 as isize) = ' ' as i32 as ::core::ffi::c_char;
        len += pdf_sprint_length(buf.offset(len as isize), mlimit);
        let fresh4 = len;
        len = len + 1;
        *buf.offset(fresh4 as isize) = ' ' as i32 as ::core::ffi::c_char;
        let fresh5 = len;
        len = len + 1;
        *buf.offset(fresh5 as isize) = 'M' as i32 as ::core::ffi::c_char;
        pdf_doc_add_page_content(buf, len as ::core::ffi::c_uint);
        (*gs).miterlimit = mlimit;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_setlinecap")]
pub unsafe extern "C-unwind" fn pdf_dev_setlinecap(
    mut capstyle: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut buf: *mut ::core::ffi::c_char =
        &raw mut (*__pdf_state).fmt_buf as *mut ::core::ffi::c_char;
    if (*gs).linecap != capstyle {
        len = sprintf(
            buf,
            b" %d J\0" as *const u8 as *const ::core::ffi::c_char,
            capstyle,
        );
        pdf_doc_add_page_content(buf, len as ::core::ffi::c_uint);
        (*gs).linecap = capstyle;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_setlinejoin")]
pub unsafe extern "C-unwind" fn pdf_dev_setlinejoin(
    mut joinstyle: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut buf: *mut ::core::ffi::c_char =
        &raw mut (*__pdf_state).fmt_buf as *mut ::core::ffi::c_char;
    if (*gs).linejoin != joinstyle {
        len = sprintf(
            buf,
            b" %d j\0" as *const u8 as *const ::core::ffi::c_char,
            joinstyle,
        );
        pdf_doc_add_page_content(buf, len as ::core::ffi::c_uint);
        (*gs).linejoin = joinstyle;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_setlinewidth")]
pub unsafe extern "C-unwind" fn pdf_dev_setlinewidth(
    mut width: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut buf: *mut ::core::ffi::c_char =
        &raw mut (*__pdf_state).fmt_buf as *mut ::core::ffi::c_char;
    if (*gs).linewidth != width {
        let fresh0 = len;
        len = len + 1;
        *buf.offset(fresh0 as isize) = ' ' as i32 as ::core::ffi::c_char;
        len += pdf_sprint_length(buf.offset(len as isize), width);
        let fresh1 = len;
        len = len + 1;
        *buf.offset(fresh1 as isize) = ' ' as i32 as ::core::ffi::c_char;
        let fresh2 = len;
        len = len + 1;
        *buf.offset(fresh2 as isize) = 'w' as i32 as ::core::ffi::c_char;
        pdf_doc_add_page_content(buf, len as ::core::ffi::c_uint);
        (*gs).linewidth = width;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_setdash")]
pub unsafe extern "C-unwind" fn pdf_dev_setdash(
    mut count: ::core::ffi::c_int,
    mut pattern: *mut ::core::ffi::c_double,
    mut offset: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut buf: *mut ::core::ffi::c_char =
        &raw mut (*__pdf_state).fmt_buf as *mut ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int = 0;
    (*gs).linedash.num_dash = count;
    (*gs).linedash.offset = offset;
    pdf_doc_add_page_content(
        b" [\0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_uint,
    );
    i = 0 as ::core::ffi::c_int;
    while i < count {
        *buf.offset(0 as ::core::ffi::c_int as isize) = ' ' as i32 as ::core::ffi::c_char;
        len = pdf_sprint_length(
            buf.offset(1 as ::core::ffi::c_int as isize),
            *pattern.offset(i as isize),
        );
        pdf_doc_add_page_content(buf, (len + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        (*gs).linedash.pattern[i as usize] = *pattern.offset(i as isize);
        i += 1;
    }
    pdf_doc_add_page_content(
        b"] \0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_uint,
    );
    len = pdf_sprint_length(buf, offset);
    pdf_doc_add_page_content(buf, len as ::core::ffi::c_uint);
    pdf_doc_add_page_content(
        b" d\0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_uint,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_clip")]
pub unsafe extern "C-unwind" fn pdf_dev_clip() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    return pdf_dev__flushpath(
        cpa,
        'W' as i32 as ::core::ffi::c_char,
        PDF_FILL_RULE_NONZERO,
        0 as ::core::ffi::c_int,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_eoclip")]
pub unsafe extern "C-unwind" fn pdf_dev_eoclip() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    return pdf_dev__flushpath(
        cpa,
        'W' as i32 as ::core::ffi::c_char,
        PDF_FILL_RULE_EVENODD,
        0 as ::core::ffi::c_int,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_flushpath")]
pub unsafe extern "C-unwind" fn pdf_dev_flushpath(
    mut p_op: ::core::ffi::c_char,
    mut fill_rule: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    error = pdf_dev__flushpath(cpa, p_op, fill_rule, 1 as ::core::ffi::c_int);
    pdf_path__clearpath(cpa);
    (*gs).flags &= !GS_FLAG_CURRENTPOINT_SET;
    return error;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_newpath")]
pub unsafe extern "C-unwind" fn pdf_dev_newpath() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut p: *mut pdf_path = &raw mut (*gs).path;
    if (*p).num_paths > 0 as ::core::ffi::c_uint {
        pdf_path__clearpath(p);
    }
    pdf_doc_add_page_content(
        b" n\0" as *const u8 as *const ::core::ffi::c_char,
        2 as ::core::ffi::c_uint,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_moveto")]
pub unsafe extern "C-unwind" fn pdf_dev_moveto(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut p: pdf_coord = pdf_coord { x: 0., y: 0. };
    p.x = x;
    p.y = y;
    return pdf_path__moveto(cpa, cpt, &raw mut p);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_rmoveto")]
pub unsafe extern "C-unwind" fn pdf_dev_rmoveto(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut p: pdf_coord = pdf_coord { x: 0., y: 0. };
    p.x = (*cpt).x + x;
    p.y = (*cpt).y + y;
    return pdf_path__moveto(cpa, cpt, &raw mut p);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_lineto")]
pub unsafe extern "C-unwind" fn pdf_dev_lineto(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    p0.x = x;
    p0.y = y;
    return pdf_path__lineto(cpa, cpt, &raw mut p0);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_rlineto")]
pub unsafe extern "C-unwind" fn pdf_dev_rlineto(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    p0.x = x + (*cpt).x;
    p0.y = y + (*cpt).y;
    return pdf_path__lineto(cpa, cpt, &raw mut p0);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_curveto")]
pub unsafe extern "C-unwind" fn pdf_dev_curveto(
    mut x0: ::core::ffi::c_double,
    mut y0: ::core::ffi::c_double,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    p0.x = x0;
    p0.y = y0;
    p1.x = x1;
    p1.y = y1;
    p2.x = x2;
    p2.y = y2;
    return pdf_path__curveto(cpa, cpt, &raw mut p0, &raw mut p1, &raw mut p2);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_rcurveto")]
pub unsafe extern "C-unwind" fn pdf_dev_rcurveto(
    mut x0: ::core::ffi::c_double,
    mut y0: ::core::ffi::c_double,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    p0.x = x0 + (*cpt).x;
    p0.y = y0 + (*cpt).y;
    p1.x = x1 + (*cpt).x;
    p1.y = y1 + (*cpt).y;
    p2.x = x2 + (*cpt).x;
    p2.y = y2 + (*cpt).y;
    return pdf_path__curveto(cpa, cpt, &raw mut p0, &raw mut p1, &raw mut p2);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_closepath")]
pub unsafe extern "C-unwind" fn pdf_dev_closepath() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    return pdf_path__closepath(cpa, cpt);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_dtransform")]
pub unsafe extern "C-unwind" fn pdf_dev_dtransform(
    mut p: *mut pdf_coord,
    mut M: *const pdf_tmatrix,
) {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut CTM: *mut pdf_tmatrix = &raw mut (*gs).matrix;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_coord__dtransform(unsafe { &mut *p }, unsafe {
        *(if !M.is_null() {
            M
        } else {
            CTM as *const pdf_tmatrix
        })
    });
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_idtransform")]
pub unsafe extern "C-unwind" fn pdf_dev_idtransform(
    mut p: *mut pdf_coord,
    mut M: *const pdf_tmatrix,
) {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut CTM: *mut pdf_tmatrix = &raw mut (*gs).matrix;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_coord__idtransform(unsafe { &mut *p }, unsafe {
        *(if !M.is_null() {
            M
        } else {
            CTM as *const pdf_tmatrix
        })
    });
}
pub(crate) unsafe fn pdf_dev_transform(mut p: *mut pdf_coord, mut M: *const pdf_tmatrix) {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut CTM: *mut pdf_tmatrix = &raw mut (*gs).matrix;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_coord__transform(unsafe { &mut *p }, unsafe {
        *(if !M.is_null() {
            M
        } else {
            CTM as *const pdf_tmatrix
        })
    });
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_itransform")]
pub unsafe extern "C-unwind" fn pdf_dev_itransform(
    mut p: *mut pdf_coord,
    mut M: *const pdf_tmatrix,
) {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut CTM: *mut pdf_tmatrix = &raw mut (*gs).matrix;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_coord__itransform(unsafe { &mut *p }, unsafe {
        *(if !M.is_null() {
            M
        } else {
            CTM as *const pdf_tmatrix
        })
    });
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_arc")]
pub unsafe extern "C-unwind" fn pdf_dev_arc(
    mut c_x: ::core::ffi::c_double,
    mut c_y: ::core::ffi::c_double,
    mut r: ::core::ffi::c_double,
    mut a_0: ::core::ffi::c_double,
    mut a_1: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut c: pdf_coord = pdf_coord { x: 0., y: 0. };
    c.x = c_x;
    c.y = c_y;
    return pdf_path__elliptarc(
        cpa,
        cpt,
        &raw mut c,
        r,
        r,
        0.0f64,
        a_0,
        a_1,
        1 as ::core::ffi::c_int,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_arcn")]
pub unsafe extern "C-unwind" fn pdf_dev_arcn(
    mut c_x: ::core::ffi::c_double,
    mut c_y: ::core::ffi::c_double,
    mut r: ::core::ffi::c_double,
    mut a_0: ::core::ffi::c_double,
    mut a_1: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut c: pdf_coord = pdf_coord { x: 0., y: 0. };
    c.x = c_x;
    c.y = c_y;
    return pdf_path__elliptarc(
        cpa,
        cpt,
        &raw mut c,
        r,
        r,
        0.0f64,
        a_0,
        a_1,
        -(1 as ::core::ffi::c_int),
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_arcx")]
pub unsafe extern "C-unwind" fn pdf_dev_arcx(
    mut c_x: ::core::ffi::c_double,
    mut c_y: ::core::ffi::c_double,
    mut r_x: ::core::ffi::c_double,
    mut r_y: ::core::ffi::c_double,
    mut a_0: ::core::ffi::c_double,
    mut a_1: ::core::ffi::c_double,
    mut a_d: ::core::ffi::c_int,
    mut xar: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut c: pdf_coord = pdf_coord { x: 0., y: 0. };
    c.x = c_x;
    c.y = c_y;
    return pdf_path__elliptarc(cpa, cpt, &raw mut c, r_x, r_y, xar, a_0, a_1, a_d);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_bspline")]
pub unsafe extern "C-unwind" fn pdf_dev_bspline(
    mut x0: ::core::ffi::c_double,
    mut y0: ::core::ffi::c_double,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut gss: *mut dpx_stack = &raw mut (*__pdf_state).gs_stack;
    let mut gs: *mut pdf_gstate = dpx_stack_top(gss) as *mut pdf_gstate;
    let mut cpa: *mut pdf_path = &raw mut (*gs).path;
    let mut cpt: *mut pdf_coord = &raw mut (*gs).cp;
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p3: pdf_coord = pdf_coord { x: 0., y: 0. };
    p1.x = x0 + 2.0f64 * (x1 - x0) / 3.0f64;
    p1.y = y0 + 2.0f64 * (y1 - y0) / 3.0f64;
    p2.x = x1 + (x2 - x1) / 3.0f64;
    p2.y = y1 + (y2 - y1) / 3.0f64;
    p3.x = x2;
    p3.y = y2;
    return pdf_path__curveto(cpa, cpt, &raw mut p1, &raw mut p2, &raw mut p3);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_rectstroke")]
pub unsafe extern "C-unwind" fn pdf_dev_rectstroke(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
    mut M: *const pdf_tmatrix,
) -> ::core::ffi::c_int {
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    r.llx = x;
    r.lly = y;
    r.urx = x + w;
    r.ury = y + h;
    return pdf_dev__rectshape(&raw mut r, M, 'S' as i32 as ::core::ffi::c_char);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_rectfill")]
pub unsafe extern "C-unwind" fn pdf_dev_rectfill(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    r.llx = x;
    r.lly = y;
    r.urx = x + w;
    r.ury = y + h;
    return pdf_dev__rectshape(
        &raw mut r,
        ::core::ptr::null::<pdf_tmatrix>(),
        'f' as i32 as ::core::ffi::c_char,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_rectclip")]
pub unsafe extern "C-unwind" fn pdf_dev_rectclip(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    r.llx = x;
    r.lly = y;
    r.urx = x + w;
    r.ury = y + h;
    return pdf_dev__rectshape(
        &raw mut r,
        ::core::ptr::null::<pdf_tmatrix>(),
        'W' as i32 as ::core::ffi::c_char,
    );
}
unsafe extern "C" {
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
#[repr(C)]
pub struct pdf_file {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_invertmatrix(mut M: *mut pdf_tmatrix) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_invertmatrix(M) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_reset_xgstate(
    mut force: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_reset_xgstate(force) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_xgstate_push(mut object: *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_xgstate_push(object) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_xgstate_pop() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_xgstate_pop() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_init_gstates() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_init_gstates() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_clear_gstates() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_clear_gstates() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_gsave(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_gsave() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_grestore(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_grestore() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_push_gstate(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_push_gstate() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_pop_gstate(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_pop_gstate() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_current_depth(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_current_depth() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_grestore_to(mut depth: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_grestore_to(depth) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_currentpoint(
    mut p: *mut pdf_coord,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_currentpoint(p) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_currentmatrix(
    mut M: *mut pdf_tmatrix,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_currentmatrix(M) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_set_color(
    mut color: *const pdf_color,
    mut mask: ::core::ffi::c_char,
    mut force: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_set_color(color, mask, force) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_concat(
    mut M: *const pdf_tmatrix,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_concat(M) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_setmiterlimit(
    mut mlimit: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_setmiterlimit(mlimit) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_setlinecap(
    mut capstyle: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_setlinecap(capstyle) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_setlinejoin(
    mut joinstyle: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_setlinejoin(joinstyle) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_setlinewidth(
    mut width: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_setlinewidth(width) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_setdash(
    mut count: ::core::ffi::c_int,
    mut pattern: *mut ::core::ffi::c_double,
    mut offset: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_setdash(count, pattern, offset) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_clip(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_clip() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_eoclip(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_eoclip() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_flushpath(
    mut p_op: ::core::ffi::c_char,
    mut fill_rule: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_flushpath(p_op, fill_rule) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_newpath(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_newpath() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_moveto(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_moveto(x, y) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_rmoveto(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_rmoveto(x, y) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_lineto(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_lineto(x, y) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_rlineto(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_rlineto(x, y) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_curveto(
    mut x0: ::core::ffi::c_double,
    mut y0: ::core::ffi::c_double,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_curveto(x0, y0, x1, y1, x2, y2) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_rcurveto(
    mut x0: ::core::ffi::c_double,
    mut y0: ::core::ffi::c_double,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_rcurveto(x0, y0, x1, y1, x2, y2) })
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_closepath(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_closepath() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_dtransform(
    mut p: *mut pdf_coord,
    mut M: *const pdf_tmatrix,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_dtransform(p, M) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_idtransform(
    mut p: *mut pdf_coord,
    mut M: *const pdf_tmatrix,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_idtransform(p, M) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_itransform(
    mut p: *mut pdf_coord,
    mut M: *const pdf_tmatrix,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_itransform(p, M) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_arc(
    mut c_x: ::core::ffi::c_double,
    mut c_y: ::core::ffi::c_double,
    mut r: ::core::ffi::c_double,
    mut a_0: ::core::ffi::c_double,
    mut a_1: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_arc(c_x, c_y, r, a_0, a_1) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_arcn(
    mut c_x: ::core::ffi::c_double,
    mut c_y: ::core::ffi::c_double,
    mut r: ::core::ffi::c_double,
    mut a_0: ::core::ffi::c_double,
    mut a_1: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_arcn(c_x, c_y, r, a_0, a_1) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_arcx(
    mut c_x: ::core::ffi::c_double,
    mut c_y: ::core::ffi::c_double,
    mut r_x: ::core::ffi::c_double,
    mut r_y: ::core::ffi::c_double,
    mut a_0: ::core::ffi::c_double,
    mut a_1: ::core::ffi::c_double,
    mut a_d: ::core::ffi::c_int,
    mut xar: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_dev_arcx(c_x, c_y, r_x, r_y, a_0, a_1, a_d, xar)
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_bspline(
    mut x0: ::core::ffi::c_double,
    mut y0: ::core::ffi::c_double,
    mut x1: ::core::ffi::c_double,
    mut y1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut y2: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_bspline(x0, y0, x1, y1, x2, y2) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_rectstroke(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
    mut M: *const pdf_tmatrix,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_rectstroke(x, y, w, h, M) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_rectfill(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_rectfill(x, y, w, h) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_rectclip(
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
    mut w: ::core::ffi::c_double,
    mut h: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_rectclip(x, y, w, h) }) {
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
    fmt_buf: [::core::ffi::c_char; 1024],
    xgs_stack: dpx_stack,
    xgs_count: ::core::ffi::c_int,
    gs_stack: dpx_stack,
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self {
            fmt_buf: [0; 1024],
            xgs_stack: dpx_stack {
                size: 0,
                top: ::core::ptr::null_mut::<stack_elem>(),
                bottom: ::core::ptr::null_mut::<stack_elem>(),
            },
            xgs_count: 0 as ::core::ffi::c_int,
            gs_stack: dpx_stack {
                size: 0 as ::core::ffi::c_int,
                top: ::core::ptr::null_mut::<stack_elem>(),
                bottom: ::core::ptr::null_mut::<stack_elem>(),
            },
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe {
        (*crate::compat::document::current())
            .backend
            .dpx_pdfdraw
            .pointer
    }
}
