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
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_ref_obj"]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_link_obj"]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_res {
    pub ident: *mut ::core::ffi::c_char,
    pub flags: ::core::ffi::c_int,
    pub category: ::core::ffi::c_int,
    pub cdata: *mut ::core::ffi::c_void,
    pub object: *mut pdf_obj,
    pub reference: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct res_cache {
    pub count: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
    pub resources: *mut pdf_res,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub name: *const ::core::ffi::c_char,
    pub cat_id: ::core::ffi::c_int,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
pub const PDF_RES_FLUSH_IMMEDIATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PDF_RESOURCE_FONT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PDF_RESOURCE_CIDFONT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PDF_RESOURCE_ENCODING: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_RESOURCE_CMAP: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PDF_RESOURCE_XOBJECT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PDF_RESOURCE_COLORSPACE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PDF_RESOURCE_SHADING: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PDF_RESOURCE_PATTERN: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const PDF_RESOURCE_GSTATE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
static mut pdf_resource_categories: [C2RustUnnamed; 9] = [
    C2RustUnnamed {
        name: b"Font\0" as *const u8 as *const ::core::ffi::c_char,
        cat_id: PDF_RESOURCE_FONT,
    },
    C2RustUnnamed {
        name: b"CIDFont\0" as *const u8 as *const ::core::ffi::c_char,
        cat_id: PDF_RESOURCE_CIDFONT,
    },
    C2RustUnnamed {
        name: b"Encoding\0" as *const u8 as *const ::core::ffi::c_char,
        cat_id: PDF_RESOURCE_ENCODING,
    },
    C2RustUnnamed {
        name: b"CMap\0" as *const u8 as *const ::core::ffi::c_char,
        cat_id: PDF_RESOURCE_CMAP,
    },
    C2RustUnnamed {
        name: b"XObject\0" as *const u8 as *const ::core::ffi::c_char,
        cat_id: PDF_RESOURCE_XOBJECT,
    },
    C2RustUnnamed {
        name: b"ColorSpace\0" as *const u8 as *const ::core::ffi::c_char,
        cat_id: PDF_RESOURCE_COLORSPACE,
    },
    C2RustUnnamed {
        name: b"Shading\0" as *const u8 as *const ::core::ffi::c_char,
        cat_id: PDF_RESOURCE_SHADING,
    },
    C2RustUnnamed {
        name: b"Pattern\0" as *const u8 as *const ::core::ffi::c_char,
        cat_id: PDF_RESOURCE_PATTERN,
    },
    C2RustUnnamed {
        name: b"ExtGState\0" as *const u8 as *const ::core::ffi::c_char,
        cat_id: PDF_RESOURCE_GSTATE,
    },
];
pub const PDF_NUM_RESOURCE_CATEGORIES: usize = (::core::mem::size_of::<[C2RustUnnamed; 9]>()
    as usize)
    .wrapping_div(::core::mem::size_of::<C2RustUnnamed>() as usize);
pub const CACHE_ALLOC_SIZE: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
static mut resources: [res_cache; 9] = [res_cache {
    count: 0,
    capacity: 0,
    resources: ::core::ptr::null::<pdf_res>() as *mut pdf_res,
}; 9];
unsafe extern "C-unwind" fn pdf_init_resource(mut res: *mut pdf_res) {
    if res.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"res\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*res).ident = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*res).category = -(1 as ::core::ffi::c_int);
    (*res).flags = 0 as ::core::ffi::c_int;
    (*res).cdata = NULL;
    (*res).object = ::core::ptr::null_mut::<pdf_obj>();
    (*res).reference = ::core::ptr::null_mut::<pdf_obj>();
}
unsafe extern "C-unwind" fn pdf_flush_resource(mut res: *mut pdf_res) {
    if !res.is_null() {
        pdf_release_obj((*res).reference);
        pdf_release_obj((*res).object);
        (*res).reference = ::core::ptr::null_mut::<pdf_obj>();
        (*res).object = ::core::ptr::null_mut::<pdf_obj>();
    }
}
unsafe extern "C-unwind" fn pdf_clean_resource(mut res: *mut pdf_res) {
    if !res.is_null() {
        if !(*res).reference.is_null() || !(*res).object.is_null() {
            pdf_warning!(
                b"Trying to release un-flushed object.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        pdf_release_obj((*res).reference);
        pdf_release_obj((*res).object);
        (*res).ident = mfree((*res).ident as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
        (*res).category = -(1 as ::core::ffi::c_int);
        (*res).flags = 0 as ::core::ffi::c_int;
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_init_resources")]
pub unsafe extern "C-unwind" fn pdf_init_resources() {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while (i as usize) < PDF_NUM_RESOURCE_CATEGORIES {
        resources[i as usize].count = 0 as ::core::ffi::c_int;
        resources[i as usize].capacity = 0 as ::core::ffi::c_int;
        resources[i as usize].resources = ::core::ptr::null_mut::<pdf_res>();
        i = i.wrapping_add(1);
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_close_resources")]
pub unsafe extern "C-unwind" fn pdf_close_resources() {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while (i as usize) < PDF_NUM_RESOURCE_CATEGORIES {
        let mut rc: *mut res_cache = ::core::ptr::null_mut::<res_cache>();
        let mut j: ::core::ffi::c_int = 0;
        rc = (&raw mut resources as *mut res_cache).offset(i as isize) as *mut res_cache;
        j = 0 as ::core::ffi::c_int;
        while j < (*rc).count {
            pdf_flush_resource((*rc).resources.offset(j as isize) as *mut pdf_res);
            pdf_clean_resource((*rc).resources.offset(j as isize) as *mut pdf_res);
            j += 1;
        }
        free((*rc).resources as *mut ::core::ffi::c_void);
        (*rc).count = 0 as ::core::ffi::c_int;
        (*rc).capacity = 0 as ::core::ffi::c_int;
        (*rc).resources = ::core::ptr::null_mut::<pdf_res>();
        i = i.wrapping_add(1);
    }
}
unsafe extern "C-unwind" fn get_category(
    mut category: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while (i as usize) < PDF_NUM_RESOURCE_CATEGORIES {
        if streq_ptr(category, pdf_resource_categories[i as usize].name) {
            return pdf_resource_categories[i as usize].cat_id;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_pdf_defineresource")]
pub unsafe extern "C-unwind" fn pdf_defineresource(
    mut category: *const ::core::ffi::c_char,
    mut resname: *const ::core::ffi::c_char,
    mut object: *mut pdf_obj,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut res_id: ::core::ffi::c_int = 0;
    let mut rc: *mut res_cache = ::core::ptr::null_mut::<res_cache>();
    let mut cat_id: ::core::ffi::c_int = 0;
    let mut res: *mut pdf_res = ::core::ptr::null_mut::<pdf_res>();
    if !(!category.is_null() && !object.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"category && object\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    cat_id = get_category(category);
    if cat_id < 0 as ::core::ffi::c_int {
        pdf_abort!(
            b"Unknown resource category: %s\0" as *const u8 as *const ::core::ffi::c_char,
            category,
        );
    }
    rc = (&raw mut resources as *mut res_cache).offset(cat_id as isize) as *mut res_cache;
    if !resname.is_null() {
        res_id = 0 as ::core::ffi::c_int;
        while res_id < (*rc).count {
            res = (*rc).resources.offset(res_id as isize) as *mut pdf_res;
            if streq_ptr(resname, (*res).ident) {
                pdf_warning!(
                    b"Resource %s (category: %s) already defined...\0" as *const u8
                        as *const ::core::ffi::c_char,
                    resname,
                    category,
                );
                pdf_flush_resource(res);
                (*res).flags = flags;
                if flags & PDF_RES_FLUSH_IMMEDIATE != 0 {
                    (*res).reference = pdf_ref_obj(object);
                    pdf_release_obj(object);
                } else {
                    (*res).object = object;
                }
                return cat_id << 16 as ::core::ffi::c_int | res_id;
            }
            res_id += 1;
        }
    } else {
        res_id = (*rc).count;
    }
    if res_id == (*rc).count {
        if (*rc).count >= (*rc).capacity {
            (*rc).capacity = ((*rc).capacity as ::core::ffi::c_uint).wrapping_add(CACHE_ALLOC_SIZE)
                as ::core::ffi::c_int as ::core::ffi::c_int;
            (*rc).resources = renew(
                (*rc).resources as *mut ::core::ffi::c_void,
                ((*rc).capacity as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<pdf_res>() as usize)
                    as uint32_t,
            ) as *mut pdf_res;
        }
        res = (*rc).resources.offset(res_id as isize) as *mut pdf_res;
        pdf_init_resource(res);
        if !resname.is_null()
            && *resname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != '\0' as i32
        {
            (*res).ident = new(
                (strlen(resname).wrapping_add(1 as size_t) as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                    as uint32_t,
            ) as *mut ::core::ffi::c_char;
            strcpy((*res).ident, resname);
        }
        (*res).category = cat_id;
        (*res).flags = flags;
        if flags & PDF_RES_FLUSH_IMMEDIATE != 0 {
            (*res).reference = pdf_ref_obj(object);
            pdf_release_obj(object);
        } else {
            (*res).object = object;
        }
        (*rc).count += 1;
    }
    return cat_id << 16 as ::core::ffi::c_int | res_id;
}
#[unsafe(export_name = "__pdf_impl_pdf_findresource")]
pub unsafe extern "C-unwind" fn pdf_findresource(
    mut category: *const ::core::ffi::c_char,
    mut resname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut res: *mut pdf_res = ::core::ptr::null_mut::<pdf_res>();
    let mut res_id: ::core::ffi::c_int = 0;
    let mut cat_id: ::core::ffi::c_int = 0;
    let mut rc: *mut res_cache = ::core::ptr::null_mut::<res_cache>();
    if !(!resname.is_null() && !category.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"resname && category\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    cat_id = get_category(category);
    if cat_id < 0 as ::core::ffi::c_int {
        pdf_abort!(
            b"Unknown resource category: %s\0" as *const u8 as *const ::core::ffi::c_char,
            category,
        );
    }
    rc = (&raw mut resources as *mut res_cache).offset(cat_id as isize) as *mut res_cache;
    res_id = 0 as ::core::ffi::c_int;
    while res_id < (*rc).count {
        res = (*rc).resources.offset(res_id as isize) as *mut pdf_res;
        if streq_ptr(resname, (*res).ident) {
            return cat_id << 16 as ::core::ffi::c_int | res_id;
        }
        res_id += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_pdf_get_resource_reference")]
pub unsafe extern "C-unwind" fn pdf_get_resource_reference(
    mut rc_id: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut cat_id: ::core::ffi::c_int = 0;
    let mut res_id: ::core::ffi::c_int = 0;
    let mut rc: *mut res_cache = ::core::ptr::null_mut::<res_cache>();
    let mut res: *mut pdf_res = ::core::ptr::null_mut::<pdf_res>();
    cat_id = rc_id >> 16 as ::core::ffi::c_int & 0xffff as ::core::ffi::c_int;
    res_id = rc_id & 0xffff as ::core::ffi::c_int;
    if cat_id < 0 as ::core::ffi::c_int || cat_id as usize >= PDF_NUM_RESOURCE_CATEGORIES {
        pdf_abort!(
            b"Invalid category ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            cat_id,
        );
    }
    rc = (&raw mut resources as *mut res_cache).offset(cat_id as isize) as *mut res_cache;
    if res_id < 0 as ::core::ffi::c_int || res_id >= (*rc).count {
        pdf_abort!(
            b"Invalid resource ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            res_id,
        );
    }
    res = (*rc).resources.offset(res_id as isize) as *mut pdf_res;
    if (*res).reference.is_null() {
        if (*res).object.is_null() {
            pdf_abort!(b"Undefined object...\0" as *const u8 as *const ::core::ffi::c_char,);
        } else {
            (*res).reference = pdf_ref_obj((*res).object);
        }
    }
    return pdf_link_obj((*res).reference);
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_init_resources() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_init_resources() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_close_resources() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_close_resources() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_defineresource(
    mut category: *const ::core::ffi::c_char,
    mut resname: *const ::core::ffi::c_char,
    mut object: *mut pdf_obj,
    mut flags: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_defineresource(category, resname, object, flags)
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
pub unsafe extern "C" fn __pdf_entry_pdf_findresource(
    mut category: *const ::core::ffi::c_char,
    mut resname: *const ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_findresource(category, resname) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_resource_reference(
    mut rc_id: ::core::ffi::c_int,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_resource_reference(rc_id) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
