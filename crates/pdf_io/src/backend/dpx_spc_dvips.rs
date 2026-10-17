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
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn xmalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn xrealloc(
        old_address: *mut ::core::ffi::c_void,
        new_size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ttstub_input_open(
        path: *const ::core::ffi::c_char,
        format: ttbc_file_format,
        is_gz: ::core::ffi::c_int,
    ) -> rust_input_handle_t;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_transform_info_clear"]
    fn transform_info_clear(info: *mut transform_info);
    fn spc_warn(spe: *mut spc_env, fmt: *const ::core::ffi::c_char, ...);
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
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_pdf_ximage_load_image"]
    fn pdf_ximage_load_image(
        ident: *const ::core::ffi::c_char,
        filename: *const ::core::ffi::c_char,
        options: load_options,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_mps_exec_inline"]
    fn mps_exec_inline(
        buffer: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        x_user: ::core::ffi::c_double,
        y_user: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_mps_stack_depth"]
    fn mps_stack_depth() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_mps_eop_cleanup"]
    fn mps_eop_cleanup();
    #[link_name = "__pdf_impl_pdf_dev_concat"]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_gsave"]
    fn pdf_dev_gsave() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_grestore"]
    fn pdf_dev_grestore() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_current_depth"]
    fn pdf_dev_current_depth() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_grestore_to"]
    fn pdf_dev_grestore_to(depth: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
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
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
#[inline]
unsafe extern "C-unwind" fn strstartswith(
    mut s: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0 as ::core::ffi::c_int {
        return s.offset(length as isize);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
static mut block_pending: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut pending_x: ::core::ffi::c_double = 0.0f64;
static mut pending_y: ::core::ffi::c_double = 0.0f64;
static mut position_set: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut ps_headers: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char;
static mut num_ps_headers: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn spc_handler_ps_header(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut pro: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ps_header: rust_input_handle_t = 0;
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr.offset(1 as ::core::ffi::c_int as isize) >= (*args).endptr
        || *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '=' as i32
    {
        pdf_warning!(
            b"No filename specified for PSfile special.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*args).curptr = (*args).curptr.offset(1);
    pro = crate::backend::runtime::allocate(
        ((*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as size_t,
    ) as *mut ::core::ffi::c_char;
    strncpy(
        pro,
        (*args).curptr,
        (*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long as size_t,
    );
    *pro.offset((*args).endptr.offset_from((*args).curptr) as ::core::ffi::c_long as isize) =
        0 as ::core::ffi::c_char;
    ps_header = ttstub_input_open(pro, TTBC_FILE_FORMAT_TEX_PS_HEADER, 0 as ::core::ffi::c_int);
    if ps_header == 0 {
        pdf_warning!(
            b"PS header %s not found.\0" as *const u8 as *const ::core::ffi::c_char,
            pro
        );
        free(pro as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    crate::backend::runtime::input_close(ps_header);
    if num_ps_headers & 0xf as ::core::ffi::c_int == 0 {
        ps_headers = crate::backend::runtime::reallocate(
            ps_headers as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                .wrapping_mul((num_ps_headers + 16 as ::core::ffi::c_int) as size_t),
        ) as *mut *mut ::core::ffi::c_char;
    }
    let fresh1 = num_ps_headers;
    num_ps_headers = num_ps_headers + 1;
    let ref mut fresh2 = *ps_headers.offset(fresh1 as isize);
    *fresh2 = pro;
    (*args).curptr = (*args).endptr;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn parse_filename(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = *pp;
    let mut qchar: ::core::ffi::c_char = 0;
    let mut n: ::core::ffi::c_int = 0;
    if p.is_null() || p >= endptr {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else if *p as ::core::ffi::c_int == '"' as i32 || *p as ::core::ffi::c_int == '\'' as i32 {
        let fresh0 = p;
        p = p.offset(1);
        qchar = *fresh0;
    } else {
        qchar = ' ' as i32 as ::core::ffi::c_char;
    }
    n = 0 as ::core::ffi::c_int;
    q = p;
    while p < endptr && *p as ::core::ffi::c_int != qchar as ::core::ffi::c_int {
        n += 1;
        p = p.offset(1);
    }
    if qchar as ::core::ffi::c_int != ' ' as i32 {
        if *p as ::core::ffi::c_int != qchar as ::core::ffi::c_int {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        p = p.offset(1);
    }
    if q.is_null() || n == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    r = new(((n + 1 as ::core::ffi::c_int) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    memcpy(
        r as *mut ::core::ffi::c_void,
        q as *const ::core::ffi::c_void,
        n as size_t,
    );
    *r.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    *pp = p;
    return r;
}
unsafe extern "C-unwind" fn spc_handler_ps_file(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut form_id: ::core::ffi::c_int = 0;
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
    if !(!spe.is_null() && !args.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && args\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr.offset(1 as ::core::ffi::c_int as isize) >= (*args).endptr
        || *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != '=' as i32
    {
        pdf_warning!(
            b"No filename specified for PSfile special.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*args).curptr = (*args).curptr.offset(1);
    filename = parse_filename(&raw mut (*args).curptr, (*args).endptr);
    if filename.is_null() {
        pdf_warning!(
            b"No filename specified for PSfile special.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    transform_info_clear(&raw mut ti);
    if spc_util_read_dimtrns(spe, &raw mut ti, args, 1 as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        free(filename as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    form_id = pdf_ximage_load_image(
        ::core::ptr::null::<::core::ffi::c_char>(),
        filename,
        options,
    );
    if form_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Failed to read image file: %s\0" as *const u8 as *const ::core::ffi::c_char,
            filename
        );
        free(filename as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    free(filename as *mut ::core::ffi::c_void);
    spc_put_image(spe, form_id, &raw mut ti, (*spe).x_user, (*spe).y_user);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_ps_plotfile(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut form_id: ::core::ffi::c_int = 0;
    let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: transform_info = transform_info {
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
    if !(!spe.is_null() && !args.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && args\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_warning!(
        b"\"ps: plotfile\" found (not properly implemented)\0" as *const u8
            as *const ::core::ffi::c_char
    );
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    filename = parse_filename(&raw mut (*args).curptr, (*args).endptr);
    if filename.is_null() {
        pdf_warning!(
            b"Expecting filename but not found...\0" as *const u8 as *const ::core::ffi::c_char
        );
        return -(1 as ::core::ffi::c_int);
    }
    form_id = pdf_ximage_load_image(
        ::core::ptr::null::<::core::ffi::c_char>(),
        filename,
        options,
    );
    if form_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Could not open PS file: %s\0" as *const u8 as *const ::core::ffi::c_char,
            filename
        );
        error = -(1 as ::core::ffi::c_int);
    } else {
        transform_info_clear(&raw mut p);
        p.matrix.d = -1.0f64;
        spc_put_image(
            spe,
            form_id,
            &raw mut p,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
            0 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
    }
    free(filename as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C-unwind" fn spc_handler_ps_literal(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut st_depth: ::core::ffi::c_int = 0;
    let mut gs_depth: ::core::ffi::c_int = 0;
    let mut x_user: ::core::ffi::c_double = 0.;
    let mut y_user: ::core::ffi::c_double = 0.;
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    spc_get_current_point(spe, &raw mut cp);
    if (*args)
        .curptr
        .offset(strlen(b":[begin]\0" as *const u8 as *const ::core::ffi::c_char) as isize)
        <= (*args).endptr
        && !strstartswith(
            (*args).curptr,
            b":[begin]\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        block_pending += 1;
        position_set = 1 as ::core::ffi::c_int;
        pending_x = cp.x;
        x_user = pending_x;
        pending_y = cp.y;
        y_user = pending_y;
        (*args).curptr = (*args)
            .curptr
            .offset(strlen(b":[begin]\0" as *const u8 as *const ::core::ffi::c_char) as isize);
    } else if (*args)
        .curptr
        .offset(strlen(b":[end]\0" as *const u8 as *const ::core::ffi::c_char) as isize)
        <= (*args).endptr
        && !strstartswith(
            (*args).curptr,
            b":[end]\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        if block_pending <= 0 as ::core::ffi::c_int {
            pdf_warning!(
                b"No corresponding ::[begin] found.\0" as *const u8 as *const ::core::ffi::c_char
            );
            return -(1 as ::core::ffi::c_int);
        }
        block_pending -= 1;
        position_set = 0 as ::core::ffi::c_int;
        x_user = pending_x;
        y_user = pending_y;
        (*args).curptr = (*args)
            .curptr
            .offset(strlen(b":[end]\0" as *const u8 as *const ::core::ffi::c_char) as isize);
    } else if (*args).curptr < (*args).endptr
        && *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ':' as i32
    {
        x_user = if position_set != 0 { pending_x } else { cp.x };
        y_user = if position_set != 0 { pending_y } else { cp.y };
        (*args).curptr = (*args).curptr.offset(1);
    } else {
        position_set = 1 as ::core::ffi::c_int;
        pending_x = cp.x;
        x_user = pending_x;
        pending_y = cp.y;
        y_user = pending_y;
    }
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        st_depth = mps_stack_depth();
        gs_depth = pdf_dev_current_depth();
        error = mps_exec_inline(&raw mut (*args).curptr, (*args).endptr, x_user, y_user);
        if error != 0 {
            pdf_warning!(
                b"Interpreting PS code failed!!! Output might be broken!!!\0" as *const u8
                    as *const ::core::ffi::c_char
            );
            pdf_dev_grestore_to(gs_depth);
        } else if st_depth != mps_stack_depth() {
            pdf_warning!(
                b"Stack not empty after execution of inline PostScript code.\0" as *const u8
                    as *const ::core::ffi::c_char
            );
            pdf_warning!(
                b">> Your macro package makes some assumption on internal behaviour of DVI drivers.\0"
                as * const u8 as * const ::core::ffi::c_char
            );
            pdf_warning!(
                b">> It may not compatible with dvipdfmx.\0" as *const u8
                    as *const ::core::ffi::c_char
            );
        }
    }
    return error;
}
unsafe extern "C-unwind" fn spc_handler_ps_trickscmd(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    pdf_warning!(
        b"PSTricks commands are disallowed in Tectonic\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    (*args).curptr = (*args).endptr;
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn spc_handler_ps_tricksobj(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    pdf_warning!(
        b"PSTricks commands are disallowed in Tectonic\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    (*args).curptr = (*args).endptr;
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn spc_handler_ps_default(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0;
    let mut st_depth: ::core::ffi::c_int = 0;
    let mut gs_depth: ::core::ffi::c_int = 0;
    if !(!spe.is_null() && !args.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && args\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_dev_gsave();
    st_depth = mps_stack_depth();
    gs_depth = pdf_dev_current_depth();
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    M.d = 1.0f64;
    M.a = M.d;
    M.c = 0.0f64;
    M.b = M.c;
    M.e = (*spe).x_user;
    M.f = (*spe).y_user;
    pdf_dev_concat(&raw mut M);
    error = mps_exec_inline(
        &raw mut (*args).curptr,
        (*args).endptr,
        (*spe).x_user,
        (*spe).y_user,
    );
    M.e = -(*spe).x_user;
    M.f = -(*spe).y_user;
    pdf_dev_concat(&raw mut M);
    if error != 0 {
        pdf_warning!(
            b"Interpreting PS code failed!!! Output might be broken!!!\0" as *const u8
                as *const ::core::ffi::c_char
        );
    } else if st_depth != mps_stack_depth() {
        pdf_warning!(
            b"Stack not empty after execution of inline PostScript code.\0" as *const u8
                as *const ::core::ffi::c_char
        );
        pdf_warning!(
            b">> Your macro package makes some assumption on internal behaviour of DVI drivers.\0"
                as *const u8 as *const ::core::ffi::c_char
        );
        pdf_warning!(
            b">> It may not compatible with dvipdfmx.\0" as *const u8 as *const ::core::ffi::c_char
        );
    }
    pdf_dev_grestore_to(gs_depth);
    pdf_dev_grestore();
    return error;
}
static mut dvips_handlers: [spc_handler; 10] = unsafe {
    [
        spc_handler {
            key: b"header\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_ps_header
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"PSfile\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_ps_file
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"psfile\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_ps_file
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"ps: plotfile \0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_ps_plotfile
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"PS: plotfile \0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_ps_plotfile
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"PS:\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_ps_literal
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"ps:\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_ps_literal
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"PST:\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_ps_trickscmd
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"pst:\0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_ps_tricksobj
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
        spc_handler {
            key: b"\" \0" as *const u8 as *const ::core::ffi::c_char,
            exec: Some(
                spc_handler_ps_default
                    as unsafe extern "C-unwind" fn(
                        *mut spc_env,
                        *mut spc_arg,
                    ) -> ::core::ffi::c_int,
            ),
        },
    ]
};
#[unsafe(export_name = "__pdf_impl_spc_dvips_at_begin_document")]
pub unsafe extern "C-unwind" fn spc_dvips_at_begin_document() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_dvips_at_end_document")]
pub unsafe extern "C-unwind" fn spc_dvips_at_end_document() -> ::core::ffi::c_int {
    if !ps_headers.is_null() {
        while num_ps_headers > 0 as ::core::ffi::c_int {
            num_ps_headers -= 1;
            free(*ps_headers.offset(num_ps_headers as isize) as *mut ::core::ffi::c_void);
        }
        ps_headers = mfree(ps_headers as *mut ::core::ffi::c_void) as *mut *mut ::core::ffi::c_char;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_dvips_at_begin_page")]
pub unsafe extern "C-unwind" fn spc_dvips_at_begin_page() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_dvips_at_end_page")]
pub unsafe extern "C-unwind" fn spc_dvips_at_end_page() -> ::core::ffi::c_int {
    mps_eop_cleanup();
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_dvips_check_special")]
pub unsafe extern "C-unwind" fn spc_dvips_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> bool {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: size_t = 0;
    p = buf;
    endptr = p.offset(len as isize);
    skip_white(&raw mut p, endptr);
    if p >= endptr {
        return false_0 != 0;
    }
    len = endptr.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int;
    i = 0 as size_t;
    while i
        < (::core::mem::size_of::<[spc_handler; 10]>() as usize)
            .wrapping_div(::core::mem::size_of::<spc_handler>() as usize)
    {
        if len as size_t >= strlen(dvips_handlers[i as usize].key)
            && memcmp(
                p as *const ::core::ffi::c_void,
                dvips_handlers[i as usize].key as *const ::core::ffi::c_void,
                strlen(dvips_handlers[i as usize].key),
            ) == 0
        {
            return true_0 != 0;
        }
        i = i.wrapping_add(1);
    }
    return false_0 != 0;
}
#[unsafe(export_name = "__pdf_impl_spc_dvips_setup_handler")]
pub unsafe extern "C-unwind" fn spc_dvips_setup_handler(
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
        if (*args)
            .curptr
            .offset(strlen(b" plotfile \0" as *const u8 as *const ::core::ffi::c_char) as isize)
            <= (*args).endptr
            && !strstartswith(
                (*args).curptr,
                b" plotfile \0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            (*args).curptr = (*args).curptr.offset(strlen(
                b" plotfile \0" as *const u8 as *const ::core::ffi::c_char,
            ) as isize);
        }
    } else if (*args).curptr.offset(1 as ::core::ffi::c_int as isize) < (*args).endptr
        && *(*args).curptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '"' as i32
        && *(*args).curptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32
    {
        (*args).curptr = (*args).curptr.offset(2 as ::core::ffi::c_int as isize);
    }
    keylen = (*args).curptr.offset_from(key) as ::core::ffi::c_long as ::core::ffi::c_int;
    if keylen < 1 as ::core::ffi::c_int {
        pdf_warning!(b"Not ps: special???\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    i = 0 as size_t;
    while i
        < (::core::mem::size_of::<[spc_handler; 10]>() as usize)
            .wrapping_div(::core::mem::size_of::<spc_handler>() as usize)
    {
        if keylen as size_t == strlen(dvips_handlers[i as usize].key)
            && strncmp(key, dvips_handlers[i as usize].key, keylen as size_t) == 0
        {
            skip_white(&raw mut (*args).curptr, (*args).endptr);
            (*args).command = dvips_handlers[i as usize].key;
            (*handle).key = b"ps:\0" as *const u8 as *const ::core::ffi::c_char;
            (*handle).exec = dvips_handlers[i as usize].exec;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_dvips_at_begin_document(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_dvips_at_begin_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_dvips_at_end_document(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_dvips_at_end_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_dvips_at_begin_page(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_dvips_at_begin_page() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_dvips_at_end_page(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_dvips_at_end_page() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_dvips_check_special(
    mut buf: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_dvips_check_special(buf, len) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_dvips_setup_handler(
    mut handle: *mut spc_handler,
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        spc_dvips_setup_handler(handle, spe, args)
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
