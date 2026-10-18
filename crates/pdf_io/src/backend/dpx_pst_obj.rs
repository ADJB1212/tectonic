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
    fn isalnum(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn __error() -> *mut ::core::ffi::c_int;
    fn strtod(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strtol(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_skip_white_spaces"]
    fn skip_white_spaces(s: *mut *mut ::core::ffi::c_uchar, endptr: *mut ::core::ffi::c_uchar);
    #[link_name = "__pdf_impl_xtoi"]
    fn xtoi(c: ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_obj {
    pub type_0: pst_type,
    pub data: *mut ::core::ffi::c_void,
}
pub type pst_type = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_string {
    pub length: ::core::ffi::c_uint,
    pub value: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_name {
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_real {
    pub value: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_integer {
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pst_boolean {
    pub value: ::core::ffi::c_char,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const PST_TYPE_UNKNOWN: ::core::ffi::c_int = -1;
pub const PST_TYPE_NULL: ::core::ffi::c_int = 0;
pub const PST_TYPE_BOOLEAN: ::core::ffi::c_int = 1;
pub const PST_TYPE_INTEGER: ::core::ffi::c_int = 2;
pub const PST_TYPE_REAL: ::core::ffi::c_int = 3;
pub const PST_TYPE_STRING: ::core::ffi::c_int = 5;
pub const PST_TYPE_NAME: ::core::ffi::c_int = 6;
pub const PST_TYPE_MARK: ::core::ffi::c_int = 7;
pub const PST_NAME_LEN_MAX: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const PST_STRING_LEN_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
const pst_const_null: *const ::core::ffi::c_char =
    b"null\0" as *const u8 as *const ::core::ffi::c_char;
const pst_const_mark: *const ::core::ffi::c_char =
    b"mark\0" as *const u8 as *const ::core::ffi::c_char;
#[unsafe(export_name = "__pdf_impl_pst_new_obj")]
pub unsafe extern "C-unwind" fn pst_new_obj(
    mut type_0: pst_type,
    mut data: *mut ::core::ffi::c_void,
) -> *mut pst_obj {
    let mut obj: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    obj = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pst_obj>() as usize) as uint32_t)
        as *mut pst_obj as *mut pst_obj;
    (*obj).type_0 = type_0;
    (*obj).data = data;
    return obj;
}
#[unsafe(export_name = "__pdf_impl_pst_new_mark")]
pub unsafe extern "C-unwind" fn pst_new_mark() -> *mut pst_obj {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    q = new(
        (strlen(pst_const_mark).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy(q, pst_const_mark);
    return pst_new_obj(PST_TYPE_MARK, q as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_pst_release_obj")]
pub unsafe extern "C-unwind" fn pst_release_obj(mut obj: *mut pst_obj) {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match (*obj).type_0 {
        PST_TYPE_BOOLEAN => {
            pst_boolean_release((*obj).data as *mut pst_boolean);
        }
        PST_TYPE_INTEGER => {
            pst_integer_release((*obj).data as *mut pst_integer);
        }
        PST_TYPE_REAL => {
            pst_real_release((*obj).data as *mut pst_real);
        }
        PST_TYPE_NAME => {
            pst_name_release((*obj).data as *mut pst_name);
        }
        PST_TYPE_STRING => {
            pst_string_release((*obj).data as *mut pst_string);
        }
        PST_TYPE_NULL | PST_TYPE_MARK | PST_TYPE_UNKNOWN => {
            free((*obj).data);
        }
        _ => {
            pdf_abort!(
                b"Unrecognized object type: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*obj).type_0,
            );
        }
    }
    free(obj as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_pst_type_of")]
pub unsafe extern "C-unwind" fn pst_type_of(mut obj: *mut pst_obj) -> pst_type {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*obj).type_0;
}
#[unsafe(export_name = "__pdf_impl_pst_length_of")]
pub unsafe extern "C-unwind" fn pst_length_of(mut obj: *mut pst_obj) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match (*obj).type_0 {
        PST_TYPE_BOOLEAN => {
            len = pst_boolean_length() as ::core::ffi::c_int;
        }
        PST_TYPE_INTEGER => {
            len = pst_integer_length() as ::core::ffi::c_int;
        }
        PST_TYPE_REAL => {
            len = pst_real_length() as ::core::ffi::c_int;
        }
        PST_TYPE_NAME => {
            len = pst_name_length((*obj).data as *mut pst_name) as ::core::ffi::c_int;
        }
        PST_TYPE_STRING => {
            len = pst_string_length((*obj).data as *mut pst_string) as ::core::ffi::c_int;
        }
        PST_TYPE_NULL | PST_TYPE_MARK => {
            pdf_abort!(
                b"Operation not defined for this type of object.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        PST_TYPE_UNKNOWN => {
            len = strlen((*obj).data as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        }
        _ => {
            pdf_abort!(
                b"Unrecognized object type: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*obj).type_0,
            );
        }
    }
    return len;
}
#[unsafe(export_name = "__pdf_impl_pst_getIV")]
pub unsafe extern "C-unwind" fn pst_getIV(mut obj: *mut pst_obj) -> ::core::ffi::c_int {
    let mut iv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match (*obj).type_0 {
        PST_TYPE_BOOLEAN => {
            iv = pst_boolean_IV((*obj).data as *mut pst_boolean);
        }
        PST_TYPE_INTEGER => {
            iv = pst_integer_IV((*obj).data as *mut pst_integer);
        }
        PST_TYPE_REAL => {
            iv = pst_real_IV((*obj).data as *mut pst_real);
        }
        PST_TYPE_NAME => {
            iv = pst_name_IV();
        }
        PST_TYPE_STRING => {
            iv = pst_string_IV((*obj).data as *mut pst_string);
        }
        PST_TYPE_NULL | PST_TYPE_MARK => {
            pdf_abort!(
                b"Operation not defined for this type of object.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        PST_TYPE_UNKNOWN => {
            pdf_abort!(
                b"Cannot convert object of type UNKNOWN to integer value.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        _ => {
            pdf_abort!(
                b"Unrecognized object type: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*obj).type_0,
            );
        }
    }
    return iv;
}
#[unsafe(export_name = "__pdf_impl_pst_getRV")]
pub unsafe extern "C-unwind" fn pst_getRV(mut obj: *mut pst_obj) -> ::core::ffi::c_double {
    let mut rv: ::core::ffi::c_double = 0.0f64;
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match (*obj).type_0 {
        PST_TYPE_BOOLEAN => {
            rv = pst_boolean_RV((*obj).data as *mut pst_boolean);
        }
        PST_TYPE_INTEGER => {
            rv = pst_integer_RV((*obj).data as *mut pst_integer);
        }
        PST_TYPE_REAL => {
            rv = pst_real_RV((*obj).data as *mut pst_real);
        }
        PST_TYPE_NAME => {
            rv = pst_name_RV();
        }
        PST_TYPE_STRING => {
            rv = pst_string_RV((*obj).data as *mut pst_string);
        }
        PST_TYPE_NULL | PST_TYPE_MARK => {
            pdf_abort!(
                b"Operation not defined for this type of object.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        PST_TYPE_UNKNOWN => {
            pdf_abort!(
                b"Cannot convert object of type UNKNOWN to real value.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        _ => {
            pdf_abort!(
                b"Unrecognized object type: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*obj).type_0,
            );
        }
    }
    return rv;
}
#[unsafe(export_name = "__pdf_impl_pst_getSV")]
pub unsafe extern "C-unwind" fn pst_getSV(mut obj: *mut pst_obj) -> *mut ::core::ffi::c_uchar {
    let mut sv: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match (*obj).type_0 {
        PST_TYPE_BOOLEAN => {
            sv = pst_boolean_SV((*obj).data as *mut pst_boolean);
        }
        PST_TYPE_INTEGER => {
            sv = pst_integer_SV((*obj).data as *mut pst_integer);
        }
        PST_TYPE_REAL => {
            sv = pst_real_SV((*obj).data as *mut pst_real);
        }
        PST_TYPE_NAME => {
            sv = pst_name_SV((*obj).data as *mut pst_name);
        }
        PST_TYPE_STRING => {
            sv = pst_string_SV((*obj).data as *mut pst_string);
        }
        PST_TYPE_NULL | PST_TYPE_MARK => {
            pdf_abort!(
                b"Operation not defined for this type of object.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        PST_TYPE_UNKNOWN => {
            let mut len: ::core::ffi::c_int = 0;
            len = strlen((*obj).data as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
            if len > 0 as ::core::ffi::c_int {
                sv = new(((len + 1 as ::core::ffi::c_int) as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
                    as uint32_t) as *mut ::core::ffi::c_uchar;
                memcpy(sv as *mut ::core::ffi::c_void, (*obj).data, len as size_t);
                *sv.offset(len as isize) = '\0' as i32 as ::core::ffi::c_uchar;
            } else {
                sv = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            }
        }
        _ => {
            pdf_abort!(
                b"Unrecognized object type: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*obj).type_0,
            );
        }
    }
    return sv;
}
#[unsafe(export_name = "__pdf_impl_pst_data_ptr")]
pub unsafe extern "C-unwind" fn pst_data_ptr(mut obj: *mut pst_obj) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match (*obj).type_0 {
        PST_TYPE_BOOLEAN => {
            p = pst_boolean_data_ptr((*obj).data as *mut pst_boolean) as *mut ::core::ffi::c_char;
        }
        PST_TYPE_INTEGER => {
            p = pst_integer_data_ptr((*obj).data as *mut pst_integer) as *mut ::core::ffi::c_char;
        }
        PST_TYPE_REAL => {
            p = pst_real_data_ptr((*obj).data as *mut pst_real) as *mut ::core::ffi::c_char;
        }
        PST_TYPE_NAME => {
            p = pst_name_data_ptr((*obj).data as *mut pst_name) as *mut ::core::ffi::c_char;
        }
        PST_TYPE_STRING => {
            p = pst_string_data_ptr((*obj).data as *mut pst_string) as *mut ::core::ffi::c_char;
        }
        PST_TYPE_NULL | PST_TYPE_MARK => {
            pdf_abort!(
                b"Operation not defined for this type of object.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        PST_TYPE_UNKNOWN => {
            p = (*obj).data as *mut ::core::ffi::c_char;
        }
        _ => {
            pdf_abort!(
                b"Unrecognized object type: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*obj).type_0,
            );
        }
    }
    return p as *mut ::core::ffi::c_void;
}
unsafe extern "C-unwind" fn pst_boolean_new(mut value: ::core::ffi::c_char) -> *mut pst_boolean {
    let mut obj: *mut pst_boolean = ::core::ptr::null_mut::<pst_boolean>();
    obj = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pst_boolean>() as usize) as uint32_t)
        as *mut pst_boolean;
    (*obj).value = value;
    return obj;
}
unsafe extern "C-unwind" fn pst_boolean_release(mut obj: *mut pst_boolean) {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    free(obj as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn pst_boolean_IV(mut obj: *mut pst_boolean) -> ::core::ffi::c_int {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*obj).value as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pst_boolean_RV(mut obj: *mut pst_boolean) -> ::core::ffi::c_double {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*obj).value as ::core::ffi::c_double;
}
unsafe extern "C-unwind" fn pst_boolean_SV(mut obj: *mut pst_boolean) -> *mut ::core::ffi::c_uchar {
    let mut str: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*obj).value != 0 {
        str = new((5 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        memcpy(
            str as *mut ::core::ffi::c_void,
            b"true\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        *str.offset(4 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    } else {
        str = new((6 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        memcpy(
            str as *mut ::core::ffi::c_void,
            b"false\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            5 as size_t,
        );
        *str.offset(5 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    }
    return str;
}
unsafe extern "C-unwind" fn pst_boolean_length() -> ::core::ffi::c_uint {
    pdf_abort!(
        b"Operation not defined for this type of object.\0" as *const u8
            as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn pst_boolean_data_ptr(
    mut obj: *mut pst_boolean,
) -> *mut ::core::ffi::c_void {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return &raw mut (*obj).value as *mut ::core::ffi::c_void;
}
#[unsafe(export_name = "__pdf_impl_pst_parse_boolean")]
pub unsafe extern "C-unwind" fn pst_parse_boolean(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
) -> *mut pst_obj {
    if (*inbuf).offset(4 as ::core::ffi::c_int as isize) <= inbufend
        && memcmp(
            *inbuf as *const ::core::ffi::c_void,
            b"true\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        && ((*inbuf).offset(4 as ::core::ffi::c_int as isize) == inbufend
            || (*(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '(' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ')' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '/' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '<' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '>' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '[' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ']' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '{' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '}' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '%' as i32)
            || (*(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ' ' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\t' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\u{c}' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\r' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32))
    {
        *inbuf = (*inbuf).offset(4 as ::core::ffi::c_int as isize);
        return pst_new_obj(
            PST_TYPE_BOOLEAN,
            pst_boolean_new(1 as ::core::ffi::c_char) as *mut ::core::ffi::c_void,
        );
    } else if (*inbuf).offset(5 as ::core::ffi::c_int as isize) <= inbufend
        && memcmp(
            *inbuf as *const ::core::ffi::c_void,
            b"false\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            5 as size_t,
        ) == 0 as ::core::ffi::c_int
        && ((*inbuf).offset(5 as ::core::ffi::c_int as isize) == inbufend
            || (*(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '(' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ')' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '/' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '<' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '>' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '[' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ']' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '{' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '}' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '%' as i32)
            || (*(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ' ' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\t' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\u{c}' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\r' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32
                || *(*inbuf).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32))
    {
        *inbuf = (*inbuf).offset(5 as ::core::ffi::c_int as isize);
        return pst_new_obj(
            PST_TYPE_BOOLEAN,
            pst_boolean_new(0 as ::core::ffi::c_char) as *mut ::core::ffi::c_void,
        );
    } else {
        return ::core::ptr::null_mut::<pst_obj>();
    };
}
#[unsafe(export_name = "__pdf_impl_pst_parse_null")]
pub unsafe extern "C-unwind" fn pst_parse_null(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
) -> *mut pst_obj {
    if (*inbuf).offset(4 as ::core::ffi::c_int as isize) <= inbufend
        && memcmp(
            *inbuf as *const ::core::ffi::c_void,
            b"null\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        && ((*inbuf).offset(4 as ::core::ffi::c_int as isize) == inbufend
            || (*(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '(' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ')' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '/' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '<' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '>' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '[' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ']' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '{' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '}' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '%' as i32)
            || (*(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ' ' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\t' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\u{c}' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\r' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32
                || *(*inbuf).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32))
    {
        let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        *inbuf = (*inbuf).offset(4 as ::core::ffi::c_int as isize);
        q = new(
            (strlen(pst_const_null).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy(q, pst_const_null);
        return pst_new_obj(PST_TYPE_NULL, q as *mut ::core::ffi::c_void);
    } else {
        return ::core::ptr::null_mut::<pst_obj>();
    };
}
unsafe extern "C-unwind" fn pst_integer_new(mut value: ::core::ffi::c_int) -> *mut pst_integer {
    let mut obj: *mut pst_integer = ::core::ptr::null_mut::<pst_integer>();
    obj = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pst_integer>() as usize) as uint32_t)
        as *mut pst_integer;
    (*obj).value = value;
    return obj;
}
unsafe extern "C-unwind" fn pst_integer_release(mut obj: *mut pst_integer) {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    free(obj as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn pst_integer_IV(mut obj: *mut pst_integer) -> ::core::ffi::c_int {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*obj).value;
}
unsafe extern "C-unwind" fn pst_integer_RV(mut obj: *mut pst_integer) -> ::core::ffi::c_double {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*obj).value as ::core::ffi::c_double;
}
unsafe extern "C-unwind" fn pst_integer_SV(mut obj: *mut pst_integer) -> *mut ::core::ffi::c_uchar {
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    let mut fmt_buf: [::core::ffi::c_char; 15] = [0; 15];
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    len = sprintf(
        &raw mut fmt_buf as *mut ::core::ffi::c_char,
        b"%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*obj).value,
    );
    value = new(((len + 1 as ::core::ffi::c_int) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    strcpy(value, &raw mut fmt_buf as *mut ::core::ffi::c_char);
    return value as *mut ::core::ffi::c_uchar;
}
unsafe extern "C-unwind" fn pst_integer_data_ptr(
    mut obj: *mut pst_integer,
) -> *mut ::core::ffi::c_void {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return &raw mut (*obj).value as *mut ::core::ffi::c_void;
}
unsafe extern "C-unwind" fn pst_integer_length() -> ::core::ffi::c_uint {
    pdf_abort!(
        b"Operation not defined for this type of object.\0" as *const u8
            as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn pst_real_new(mut value: ::core::ffi::c_double) -> *mut pst_real {
    let mut obj: *mut pst_real = ::core::ptr::null_mut::<pst_real>();
    obj = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pst_real>() as usize) as uint32_t)
        as *mut pst_real;
    (*obj).value = value;
    return obj;
}
unsafe extern "C-unwind" fn pst_real_release(mut obj: *mut pst_real) {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    free(obj as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn pst_real_IV(mut obj: *mut pst_real) -> ::core::ffi::c_int {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*obj).value as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pst_real_RV(mut obj: *mut pst_real) -> ::core::ffi::c_double {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*obj).value;
}
unsafe extern "C-unwind" fn pst_real_SV(mut obj: *mut pst_real) -> *mut ::core::ffi::c_uchar {
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    let mut fmt_buf: [::core::ffi::c_char; 15] = [0; 15];
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    len = sprintf(
        &raw mut fmt_buf as *mut ::core::ffi::c_char,
        b"%.5g\0" as *const u8 as *const ::core::ffi::c_char,
        (*obj).value,
    );
    value = new((len as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    strcpy(value, &raw mut fmt_buf as *mut ::core::ffi::c_char);
    return value as *mut ::core::ffi::c_uchar;
}
unsafe extern "C-unwind" fn pst_real_data_ptr(mut obj: *mut pst_real) -> *mut ::core::ffi::c_void {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return &raw mut (*obj).value as *mut ::core::ffi::c_void;
}
unsafe extern "C-unwind" fn pst_real_length() -> ::core::ffi::c_uint {
    pdf_abort!(
        b"Operation not defined for this type of object.\0" as *const u8
            as *const ::core::ffi::c_char,
    );
}
#[unsafe(export_name = "__pdf_impl_pst_parse_number")]
pub unsafe extern "C-unwind" fn pst_parse_number(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
) -> *mut pst_obj {
    let mut cur: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut lval: ::core::ffi::c_int = 0;
    let mut dval: ::core::ffi::c_double = 0.;
    *crate::backend::runtime::errno() = 0 as ::core::ffi::c_int;
    lval = strtol(
        *inbuf as *mut ::core::ffi::c_char,
        &raw mut cur as *mut ::core::ffi::c_void as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if *crate::backend::runtime::errno() != 0
        || *cur as ::core::ffi::c_int == '.' as i32
        || *cur as ::core::ffi::c_int == 'e' as i32
        || *cur as ::core::ffi::c_int == 'E' as i32
    {
        *crate::backend::runtime::errno() = 0 as ::core::ffi::c_int;
        dval = strtod(
            *inbuf as *mut ::core::ffi::c_char,
            &raw mut cur as *mut ::core::ffi::c_void as *mut *mut ::core::ffi::c_char,
        );
        if *crate::backend::runtime::errno() == 0
            && (cur == inbufend
                || (*cur as ::core::ffi::c_int == '(' as i32
                    || *cur as ::core::ffi::c_int == ')' as i32
                    || *cur as ::core::ffi::c_int == '/' as i32
                    || *cur as ::core::ffi::c_int == '<' as i32
                    || *cur as ::core::ffi::c_int == '>' as i32
                    || *cur as ::core::ffi::c_int == '[' as i32
                    || *cur as ::core::ffi::c_int == ']' as i32
                    || *cur as ::core::ffi::c_int == '{' as i32
                    || *cur as ::core::ffi::c_int == '}' as i32
                    || *cur as ::core::ffi::c_int == '%' as i32)
                || (*cur as ::core::ffi::c_int == ' ' as i32
                    || *cur as ::core::ffi::c_int == '\t' as i32
                    || *cur as ::core::ffi::c_int == '\u{c}' as i32
                    || *cur as ::core::ffi::c_int == '\r' as i32
                    || *cur as ::core::ffi::c_int == '\n' as i32
                    || *cur as ::core::ffi::c_int == '\0' as i32))
        {
            *inbuf = cur;
            return pst_new_obj(
                PST_TYPE_REAL,
                pst_real_new(dval) as *mut ::core::ffi::c_void,
            );
        }
    } else if cur != *inbuf
        && (cur == inbufend
            || (*cur as ::core::ffi::c_int == '(' as i32
                || *cur as ::core::ffi::c_int == ')' as i32
                || *cur as ::core::ffi::c_int == '/' as i32
                || *cur as ::core::ffi::c_int == '<' as i32
                || *cur as ::core::ffi::c_int == '>' as i32
                || *cur as ::core::ffi::c_int == '[' as i32
                || *cur as ::core::ffi::c_int == ']' as i32
                || *cur as ::core::ffi::c_int == '{' as i32
                || *cur as ::core::ffi::c_int == '}' as i32
                || *cur as ::core::ffi::c_int == '%' as i32)
            || (*cur as ::core::ffi::c_int == ' ' as i32
                || *cur as ::core::ffi::c_int == '\t' as i32
                || *cur as ::core::ffi::c_int == '\u{c}' as i32
                || *cur as ::core::ffi::c_int == '\r' as i32
                || *cur as ::core::ffi::c_int == '\n' as i32
                || *cur as ::core::ffi::c_int == '\0' as i32))
    {
        *inbuf = cur;
        return pst_new_obj(
            PST_TYPE_INTEGER,
            pst_integer_new(lval) as *mut ::core::ffi::c_void,
        );
    } else if lval >= 2 as ::core::ffi::c_int
        && lval <= 36 as ::core::ffi::c_int
        && *cur as ::core::ffi::c_int == '#' as i32
        && {
            cur = cur.offset(1);
            isalnum(*cur as ::core::ffi::c_int) != 0
        }
        && (lval != 16 as ::core::ffi::c_int
            || *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'x' as i32
                && *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    != 'X' as i32)
    {
        *crate::backend::runtime::errno() = 0 as ::core::ffi::c_int;
        lval = strtol(
            cur as *mut ::core::ffi::c_char,
            &raw mut cur as *mut ::core::ffi::c_void as *mut *mut ::core::ffi::c_char,
            lval,
        ) as ::core::ffi::c_int;
        if *crate::backend::runtime::errno() == 0
            && (cur == inbufend
                || (*cur as ::core::ffi::c_int == '(' as i32
                    || *cur as ::core::ffi::c_int == ')' as i32
                    || *cur as ::core::ffi::c_int == '/' as i32
                    || *cur as ::core::ffi::c_int == '<' as i32
                    || *cur as ::core::ffi::c_int == '>' as i32
                    || *cur as ::core::ffi::c_int == '[' as i32
                    || *cur as ::core::ffi::c_int == ']' as i32
                    || *cur as ::core::ffi::c_int == '{' as i32
                    || *cur as ::core::ffi::c_int == '}' as i32
                    || *cur as ::core::ffi::c_int == '%' as i32)
                || (*cur as ::core::ffi::c_int == ' ' as i32
                    || *cur as ::core::ffi::c_int == '\t' as i32
                    || *cur as ::core::ffi::c_int == '\u{c}' as i32
                    || *cur as ::core::ffi::c_int == '\r' as i32
                    || *cur as ::core::ffi::c_int == '\n' as i32
                    || *cur as ::core::ffi::c_int == '\0' as i32))
        {
            *inbuf = cur;
            return pst_new_obj(
                PST_TYPE_INTEGER,
                pst_integer_new(lval) as *mut ::core::ffi::c_void,
            );
        }
    }
    return ::core::ptr::null_mut::<pst_obj>();
}
unsafe extern "C-unwind" fn pst_name_new(mut name: *const ::core::ffi::c_char) -> *mut pst_name {
    let mut obj: *mut pst_name = ::core::ptr::null_mut::<pst_name>();
    obj = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pst_name>() as usize) as uint32_t)
        as *mut pst_name;
    (*obj).value = new(
        (strlen(name).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*obj).value, name);
    return obj;
}
unsafe extern "C-unwind" fn pst_name_release(mut obj: *mut pst_name) {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    free((*obj).value as *mut ::core::ffi::c_void);
    free(obj as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn getxpair(mut s: *mut *mut ::core::ffi::c_uchar) -> ::core::ffi::c_int {
    let mut hi: ::core::ffi::c_int = 0;
    let mut lo: ::core::ffi::c_int = 0;
    hi = xtoi(**s as ::core::ffi::c_char);
    if hi < 0 as ::core::ffi::c_int {
        return hi;
    }
    *s = (*s).offset(1);
    lo = xtoi(**s as ::core::ffi::c_char);
    if lo < 0 as ::core::ffi::c_int {
        return lo;
    }
    *s = (*s).offset(1);
    return hi << 4 as ::core::ffi::c_int | lo;
}
#[unsafe(export_name = "__pdf_impl_pst_parse_name")]
pub unsafe extern "C-unwind" fn pst_parse_name(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
) -> *mut pst_obj {
    let mut wbuf: [::core::ffi::c_uchar; 128] = [0; 128];
    let mut c: ::core::ffi::c_uchar = 0;
    let mut p: *mut ::core::ffi::c_uchar = &raw mut wbuf as *mut ::core::ffi::c_uchar;
    let mut cur: *mut ::core::ffi::c_uchar = *inbuf;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *cur as ::core::ffi::c_int != '/' as i32 {
        return ::core::ptr::null_mut::<pst_obj>();
    }
    cur = cur.offset(1);
    while !(cur == inbufend
        || (*cur as ::core::ffi::c_int == '(' as i32
            || *cur as ::core::ffi::c_int == ')' as i32
            || *cur as ::core::ffi::c_int == '/' as i32
            || *cur as ::core::ffi::c_int == '<' as i32
            || *cur as ::core::ffi::c_int == '>' as i32
            || *cur as ::core::ffi::c_int == '[' as i32
            || *cur as ::core::ffi::c_int == ']' as i32
            || *cur as ::core::ffi::c_int == '{' as i32
            || *cur as ::core::ffi::c_int == '}' as i32
            || *cur as ::core::ffi::c_int == '%' as i32)
        || (*cur as ::core::ffi::c_int == ' ' as i32
            || *cur as ::core::ffi::c_int == '\t' as i32
            || *cur as ::core::ffi::c_int == '\u{c}' as i32
            || *cur as ::core::ffi::c_int == '\r' as i32
            || *cur as ::core::ffi::c_int == '\n' as i32
            || *cur as ::core::ffi::c_int == '\0' as i32))
    {
        let fresh0 = cur;
        cur = cur.offset(1);
        c = *fresh0;
        if c as ::core::ffi::c_int == '#' as i32 {
            let mut val: ::core::ffi::c_int = 0;
            if cur.offset(2 as ::core::ffi::c_int as isize) >= inbufend {
                pdf_warning!(
                    b"Premature end of input name string.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                break;
            } else {
                val = getxpair(&raw mut cur);
                if val <= 0 as ::core::ffi::c_int {
                    pdf_warning!(
                        b"Invalid char for name object. (ignored)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    continue;
                } else {
                    c = val as ::core::ffi::c_uchar;
                }
            }
        }
        if len < PST_NAME_LEN_MAX {
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = c;
        }
        len += 1;
    }
    *p = '\0' as i32 as ::core::ffi::c_uchar;
    if len > PST_NAME_LEN_MAX {
        pdf_warning!(
            b"String too long for name object. Output will be truncated.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    *inbuf = cur;
    return pst_new_obj(
        PST_TYPE_NAME,
        pst_name_new(&raw mut wbuf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char)
            as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C-unwind" fn pst_name_IV() -> ::core::ffi::c_int {
    pdf_abort!(
        b"Operation not defined for this type of object.\0" as *const u8
            as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn pst_name_RV() -> ::core::ffi::c_double {
    pdf_abort!(
        b"Operation not defined for this type of object.\0" as *const u8
            as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn pst_name_SV(mut obj: *mut pst_name) -> *mut ::core::ffi::c_uchar {
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    value = new(
        (strlen((*obj).value).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy(value, (*obj).value);
    return value as *mut ::core::ffi::c_uchar;
}
unsafe extern "C-unwind" fn pst_name_data_ptr(mut obj: *mut pst_name) -> *mut ::core::ffi::c_void {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*obj).value as *mut ::core::ffi::c_void;
}
unsafe extern "C-unwind" fn pst_name_length(mut obj: *mut pst_name) -> ::core::ffi::c_uint {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return strlen((*obj).value) as ::core::ffi::c_uint;
}
unsafe extern "C-unwind" fn pst_string_new(
    mut str: *mut ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
) -> *mut pst_string {
    let mut obj: *mut pst_string = ::core::ptr::null_mut::<pst_string>();
    obj = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pst_string>() as usize) as uint32_t)
        as *mut pst_string;
    (*obj).length = len;
    (*obj).value = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if len > 0 as ::core::ffi::c_uint {
        (*obj).value = new((len as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        if !str.is_null() {
            memcpy(
                (*obj).value as *mut ::core::ffi::c_void,
                str as *const ::core::ffi::c_void,
                len as size_t,
            );
        }
    }
    return obj;
}
unsafe extern "C-unwind" fn pst_string_release(mut obj: *mut pst_string) {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    free((*obj).value as *mut ::core::ffi::c_void);
    free(obj as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_pst_parse_string")]
pub unsafe extern "C-unwind" fn pst_parse_string(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
) -> *mut pst_obj {
    if (*inbuf).offset(2 as ::core::ffi::c_int as isize) >= inbufend {
        return ::core::ptr::null_mut::<pst_obj>();
    } else if **inbuf as ::core::ffi::c_int == '(' as i32 {
        return pst_new_obj(
            PST_TYPE_STRING,
            pst_string_parse_literal(inbuf, inbufend) as *mut ::core::ffi::c_void,
        );
    } else if **inbuf as ::core::ffi::c_int == '<' as i32
        && *(*inbuf).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '~' as i32
    {
        pdf_abort!(
            b"ASCII85 string not supported yet.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if **inbuf as ::core::ffi::c_int == '<' as i32 {
        return pst_new_obj(
            PST_TYPE_STRING,
            pst_string_parse_hex(inbuf, inbufend) as *mut ::core::ffi::c_void,
        );
    }
    return ::core::ptr::null_mut::<pst_obj>();
}
unsafe extern "C-unwind" fn ostrtouc(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
    mut valid: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut cur: *mut ::core::ffi::c_uchar = *inbuf;
    let mut val: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while cur < inbufend
        && cur < (*inbuf).offset(3 as ::core::ffi::c_int as isize)
        && (*cur as ::core::ffi::c_int >= '0' as i32 && *cur as ::core::ffi::c_int <= '7' as i32)
    {
        val = val << 3 as ::core::ffi::c_int
            | (*cur as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uint;
        cur = cur.offset(1);
    }
    if val > 255 as ::core::ffi::c_uint || cur == *inbuf {
        *valid = 0 as ::core::ffi::c_uchar;
    } else {
        *valid = 1 as ::core::ffi::c_uchar;
    }
    *inbuf = cur;
    return val as ::core::ffi::c_uchar;
}
unsafe extern "C-unwind" fn esctouc(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
    mut valid: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_uchar {
    let mut unescaped: ::core::ffi::c_uchar = 0;
    let mut escaped: ::core::ffi::c_uchar = 0;
    escaped = **inbuf;
    *valid = 1 as ::core::ffi::c_uchar;
    match escaped as ::core::ffi::c_int {
        92 | 41 | 40 => {
            unescaped = escaped;
            *inbuf = (*inbuf).offset(1);
        }
        110 => {
            unescaped = '\n' as i32 as ::core::ffi::c_uchar;
            *inbuf = (*inbuf).offset(1);
        }
        114 => {
            unescaped = '\r' as i32 as ::core::ffi::c_uchar;
            *inbuf = (*inbuf).offset(1);
        }
        116 => {
            unescaped = '\t' as i32 as ::core::ffi::c_uchar;
            *inbuf = (*inbuf).offset(1);
        }
        98 => {
            unescaped = '\u{8}' as i32 as ::core::ffi::c_uchar;
            *inbuf = (*inbuf).offset(1);
        }
        102 => {
            unescaped = '\u{c}' as i32 as ::core::ffi::c_uchar;
            *inbuf = (*inbuf).offset(1);
        }
        13 => {
            unescaped = 0 as ::core::ffi::c_uchar;
            *valid = 0 as ::core::ffi::c_uchar;
            *inbuf = (*inbuf).offset(
                (if *inbuf < inbufend.offset(-(1 as ::core::ffi::c_int as isize))
                    && *(*inbuf).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '\n' as i32
                {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as isize,
            );
        }
        10 => {
            unescaped = 0 as ::core::ffi::c_uchar;
            *valid = 0 as ::core::ffi::c_uchar;
            *inbuf = (*inbuf).offset(1);
        }
        _ => {
            unescaped = ostrtouc(inbuf, inbufend, valid);
        }
    }
    return unescaped;
}
unsafe extern "C-unwind" fn pst_string_parse_literal(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
) -> *mut pst_string {
    let mut wbuf: [::core::ffi::c_uchar; 4096] = [0; 4096];
    let mut cur: *mut ::core::ffi::c_uchar = *inbuf;
    let mut c: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut balance: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if cur.offset(2 as ::core::ffi::c_int as isize) > inbufend
        || *cur as ::core::ffi::c_int != '(' as i32
    {
        return ::core::ptr::null_mut::<pst_string>();
    }
    cur = cur.offset(1);
    while cur < inbufend && len < PST_STRING_LEN_MAX && balance > 0 as ::core::ffi::c_int {
        let fresh6 = cur;
        cur = cur.offset(1);
        c = *fresh6;
        match c as ::core::ffi::c_int {
            92 => {
                let mut unescaped: ::core::ffi::c_uchar = 0;
                let mut valid: ::core::ffi::c_uchar = 0;
                unescaped = esctouc(&raw mut cur, inbufend, &raw mut valid);
                if valid != 0 {
                    let fresh7 = len;
                    len = len + 1;
                    wbuf[fresh7 as usize] = unescaped;
                }
            }
            40 => {
                balance += 1;
                let fresh8 = len;
                len = len + 1;
                wbuf[fresh8 as usize] = '(' as i32 as ::core::ffi::c_uchar;
            }
            41 => {
                balance -= 1;
                if balance > 0 as ::core::ffi::c_int {
                    let fresh9 = len;
                    len = len + 1;
                    wbuf[fresh9 as usize] = ')' as i32 as ::core::ffi::c_uchar;
                }
            }
            13 => {
                if cur < inbufend && *cur as ::core::ffi::c_int == '\n' as i32 {
                    cur = cur.offset(1);
                }
                let fresh10 = len;
                len = len + 1;
                wbuf[fresh10 as usize] = '\n' as i32 as ::core::ffi::c_uchar;
            }
            _ => {
                let fresh11 = len;
                len = len + 1;
                wbuf[fresh11 as usize] = c;
            }
        }
    }
    if c as ::core::ffi::c_int != ')' as i32 {
        return ::core::ptr::null_mut::<pst_string>();
    }
    *inbuf = cur;
    return pst_string_new(
        &raw mut wbuf as *mut ::core::ffi::c_uchar,
        len as ::core::ffi::c_uint,
    );
}
unsafe extern "C-unwind" fn pst_string_parse_hex(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
) -> *mut pst_string {
    let mut wbuf: [::core::ffi::c_uchar; 4096] = [0; 4096];
    let mut cur: *mut ::core::ffi::c_uchar = *inbuf;
    let mut len: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if cur.offset(2 as ::core::ffi::c_int as isize) > inbufend
        || *cur as ::core::ffi::c_int != '<' as i32
        || *cur as ::core::ffi::c_int == '<' as i32
            && *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '<' as i32
    {
        return ::core::ptr::null_mut::<pst_string>();
    }
    cur = cur.offset(1);
    while cur < inbufend && len < PST_STRING_LEN_MAX as ::core::ffi::c_uint {
        let mut hi: ::core::ffi::c_int = 0;
        let mut lo: ::core::ffi::c_int = 0;
        skip_white_spaces(&raw mut cur, inbufend);
        if *cur as ::core::ffi::c_int == '>' as i32 {
            break;
        }
        let fresh2 = cur;
        cur = cur.offset(1);
        hi = xtoi(*fresh2 as ::core::ffi::c_char);
        if hi < 0 as ::core::ffi::c_int {
            pdf_warning!(
                b"Invalid char for hex string <%x> treated as <0>.\0" as *const u8
                    as *const ::core::ffi::c_char,
                *cur.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int,
            );
            hi = 0 as ::core::ffi::c_int;
        }
        skip_white_spaces(&raw mut cur, inbufend);
        if *cur as ::core::ffi::c_int == '>' as i32 {
            break;
        }
        lo = if cur < inbufend {
            let fresh3 = cur;
            cur = cur.offset(1);
            xtoi(*fresh3 as ::core::ffi::c_char)
        } else {
            0 as ::core::ffi::c_int
        };
        if lo < 0 as ::core::ffi::c_int {
            pdf_warning!(
                b"Invalid char for hex string <%x> treated as <0>.\0" as *const u8
                    as *const ::core::ffi::c_char,
                *cur.offset(-(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int,
            );
            lo = 0 as ::core::ffi::c_int;
        }
        let fresh4 = len;
        len = len.wrapping_add(1);
        wbuf[fresh4 as usize] = (hi << 4 as ::core::ffi::c_int | lo) as ::core::ffi::c_uchar;
    }
    let fresh5 = cur;
    cur = cur.offset(1);
    if *fresh5 as ::core::ffi::c_int != '>' as i32 {
        return ::core::ptr::null_mut::<pst_string>();
    }
    *inbuf = cur;
    return pst_string_new(&raw mut wbuf as *mut ::core::ffi::c_uchar, len);
}
unsafe extern "C-unwind" fn pst_string_IV(mut obj: *mut pst_string) -> ::core::ffi::c_int {
    return pst_string_RV(obj) as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pst_string_RV(mut obj: *mut pst_string) -> ::core::ffi::c_double {
    let mut nobj: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut end: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut rv: ::core::ffi::c_double = 0.;
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    p = (*obj).value;
    end = p.offset((*obj).length as isize);
    nobj = pst_parse_number(&raw mut p, end);
    if nobj.is_null() || p != end {
        pdf_abort!(
            b"Cound not convert string to real value.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    rv = pst_getRV(nobj);
    pst_release_obj(nobj);
    return rv;
}
unsafe extern "C-unwind" fn pst_string_SV(mut obj: *mut pst_string) -> *mut ::core::ffi::c_uchar {
    let mut str: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    str = new(
        ((*obj).length.wrapping_add(1 as ::core::ffi::c_uint) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_uchar;
    memcpy(
        str as *mut ::core::ffi::c_void,
        (*obj).value as *const ::core::ffi::c_void,
        (*obj).length as size_t,
    );
    *str.offset((*obj).length as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    return str;
}
unsafe extern "C-unwind" fn pst_string_data_ptr(
    mut obj: *mut pst_string,
) -> *mut ::core::ffi::c_void {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*obj).value as *mut ::core::ffi::c_void;
}
unsafe extern "C-unwind" fn pst_string_length(mut obj: *mut pst_string) -> ::core::ffi::c_uint {
    if obj.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"obj\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*obj).length;
}
unsafe extern "C" {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pst_new_obj(
    mut type_0: pst_type,
    mut data: *mut ::core::ffi::c_void,
    _result: *mut *mut pst_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_new_obj(type_0, data) }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_new_mark(_result: *mut *mut pst_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_new_mark() }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_release_obj(mut obj: *mut pst_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_release_obj(obj) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pst_type_of(
    mut obj: *mut pst_obj,
    _result: *mut pst_type,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_type_of(obj) }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_length_of(
    mut obj: *mut pst_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_length_of(obj) }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_getIV(
    mut obj: *mut pst_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_getIV(obj) }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_getRV(
    mut obj: *mut pst_obj,
    _result: *mut ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_getRV(obj) }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_getSV(
    mut obj: *mut pst_obj,
    _result: *mut *mut ::core::ffi::c_uchar,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_getSV(obj) }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_data_ptr(
    mut obj: *mut pst_obj,
    _result: *mut *mut ::core::ffi::c_void,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_data_ptr(obj) }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_parse_boolean(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
    _result: *mut *mut pst_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_parse_boolean(inbuf, inbufend) }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_parse_null(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
    _result: *mut *mut pst_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_parse_null(inbuf, inbufend) }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_parse_number(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
    _result: *mut *mut pst_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_parse_number(inbuf, inbufend) }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_parse_name(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
    _result: *mut *mut pst_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_parse_name(inbuf, inbufend) }) {
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
pub unsafe extern "C" fn __pdf_entry_pst_parse_string(
    mut inbuf: *mut *mut ::core::ffi::c_uchar,
    mut inbufend: *mut ::core::ffi::c_uchar,
    _result: *mut *mut pst_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_parse_string(inbuf, inbufend) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
