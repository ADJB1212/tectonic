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
#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn, static_mut_refs, dangerous_implicit_autorefs)]
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isxdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    #[link_name = "__pdf_impl_skip_white_spaces"]
    fn skip_white_spaces(s: *mut *mut ::core::ffi::c_uchar, endptr: *mut ::core::ffi::c_uchar);
    #[link_name = "__pdf_impl_pst_new_obj"]
    fn pst_new_obj(type_0: pst_type, data: *mut ::core::ffi::c_void) -> *mut pst_obj;
    #[link_name = "__pdf_impl_pst_new_mark"]
    fn pst_new_mark() -> *mut pst_obj;
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pst_parse_null"]
    fn pst_parse_null(inbuf: *mut *mut ::core::ffi::c_uchar, inbufend: *mut ::core::ffi::c_uchar) -> *mut pst_obj;
    #[link_name = "__pdf_impl_pst_parse_boolean"]
    fn pst_parse_boolean(inbuf: *mut *mut ::core::ffi::c_uchar, inbufend: *mut ::core::ffi::c_uchar) -> *mut pst_obj;
    #[link_name = "__pdf_impl_pst_parse_name"]
    fn pst_parse_name(inbuf: *mut *mut ::core::ffi::c_uchar, inbufend: *mut ::core::ffi::c_uchar) -> *mut pst_obj;
    #[link_name = "__pdf_impl_pst_parse_number"]
    fn pst_parse_number(inbuf: *mut *mut ::core::ffi::c_uchar, inbufend: *mut ::core::ffi::c_uchar) -> *mut pst_obj;
    #[link_name = "__pdf_impl_pst_parse_string"]
    fn pst_parse_string(inbuf: *mut *mut ::core::ffi::c_uchar, inbufend: *mut ::core::ffi::c_uchar) -> *mut pst_obj;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
pub type pst_type = ::core::ffi::c_int;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const PST_TYPE_UNKNOWN: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C-unwind" fn pst_parse_any(mut inbuf: *mut *mut ::core::ffi::c_uchar, mut inbufend: *mut ::core::ffi::c_uchar) -> *mut pst_obj {
    let mut data: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut cur: *mut ::core::ffi::c_uchar = *inbuf;
    let mut len: ::core::ffi::c_uint = 0;
    while cur < inbufend
        && !(cur == inbufend
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
        cur = cur.offset(1);
    }
    len = cur.offset_from(*inbuf) as ::core::ffi::c_long as ::core::ffi::c_uint;
    data = new(
        (len.wrapping_add(1 as ::core::ffi::c_uint) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_uchar;
    memcpy(data as *mut ::core::ffi::c_void, *inbuf as *const ::core::ffi::c_void, len as size_t);
    *data.offset(len as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    *inbuf = cur;
    return pst_new_obj(PST_TYPE_UNKNOWN, data as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn skip_line(mut inbuf: *mut *mut ::core::ffi::c_uchar, mut inbufend: *mut ::core::ffi::c_uchar) {
    while *inbuf < inbufend && **inbuf as ::core::ffi::c_int != '\n' as i32 && **inbuf as ::core::ffi::c_int != '\r' as i32 {
        *inbuf = (*inbuf).offset(1);
    }
    if *inbuf < inbufend && **inbuf as ::core::ffi::c_int == '\r' as i32 {
        *inbuf = (*inbuf).offset(1);
    }
    if *inbuf < inbufend && **inbuf as ::core::ffi::c_int == '\n' as i32 {
        *inbuf = (*inbuf).offset(1);
    }
}
unsafe extern "C-unwind" fn skip_comments(mut inbuf: *mut *mut ::core::ffi::c_uchar, mut inbufend: *mut ::core::ffi::c_uchar) {
    while *inbuf < inbufend && **inbuf as ::core::ffi::c_int == '%' as i32 {
        skip_line(inbuf, inbufend);
        skip_white_spaces(inbuf, inbufend);
    }
}
#[unsafe(export_name = "__pdf_impl_pst_get_token")]
pub unsafe extern "C-unwind" fn pst_get_token(mut inbuf: *mut *mut ::core::ffi::c_uchar, mut inbufend: *mut ::core::ffi::c_uchar) -> *mut pst_obj {
    let mut obj: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut c: ::core::ffi::c_uchar = 0;
    if !(*inbuf <= inbufend && *inbufend == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"*inbuf <= inbufend && !*inbufend\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    skip_white_spaces(inbuf, inbufend);
    skip_comments(inbuf, inbufend);
    if *inbuf >= inbufend {
        return ::core::ptr::null_mut::<pst_obj>();
    }
    c = **inbuf;
    match c as ::core::ffi::c_int {
        47 => {
            obj = pst_parse_name(inbuf, inbufend);
        }
        91 | 123 => {
            obj = pst_new_mark();
            *inbuf = (*inbuf).offset(1);
        }
        60 => {
            if (*inbuf).offset(1 as ::core::ffi::c_int as isize) >= inbufend {
                return ::core::ptr::null_mut::<pst_obj>();
            }
            c = *(*inbuf).offset(1 as ::core::ffi::c_int as isize);
            if c as ::core::ffi::c_int == '<' as i32 {
                obj = pst_new_mark();
                *inbuf = (*inbuf).offset(2 as ::core::ffi::c_int as isize);
            } else if isxdigit(c as ::core::ffi::c_int) != 0 {
                obj = pst_parse_string(inbuf, inbufend);
            } else if c as ::core::ffi::c_int == '~' as i32 {
                obj = pst_parse_string(inbuf, inbufend);
            }
        }
        40 => {
            obj = pst_parse_string(inbuf, inbufend);
        }
        62 => {
            if (*inbuf).offset(1 as ::core::ffi::c_int as isize) >= inbufend
                || *(*inbuf).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '>' as i32
            {
                pdf_abort!(b"Unexpected end of ASCII hex string marker.\0" as *const u8 as *const ::core::ffi::c_char,);
            } else {
                let mut mark: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
                mark = new(
                    (3 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
                ) as *mut ::core::ffi::c_char;
                *mark.offset(0 as ::core::ffi::c_int as isize) = '>' as i32 as ::core::ffi::c_char;
                *mark.offset(1 as ::core::ffi::c_int as isize) = '>' as i32 as ::core::ffi::c_char;
                *mark.offset(2 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
                obj = pst_new_obj(PST_TYPE_UNKNOWN, mark as *mut ::core::ffi::c_void);
                *inbuf = (*inbuf).offset(2 as ::core::ffi::c_int as isize);
            }
        }
        93 | 125 => {
            let mut mark_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            mark_0 =
                new((2 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
                    as *mut ::core::ffi::c_char;
            *mark_0.offset(0 as ::core::ffi::c_int as isize) = c as ::core::ffi::c_char;
            *mark_0.offset(1 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
            obj = pst_new_obj(PST_TYPE_UNKNOWN, mark_0 as *mut ::core::ffi::c_void);
            *inbuf = (*inbuf).offset(1);
        }
        _ => {
            if c as ::core::ffi::c_int == 't' as i32 || c as ::core::ffi::c_int == 'f' as i32 {
                obj = pst_parse_boolean(inbuf, inbufend);
            } else if c as ::core::ffi::c_int == 'n' as i32 {
                obj = pst_parse_null(inbuf, inbufend);
            } else if c as ::core::ffi::c_int == '+' as i32
                || c as ::core::ffi::c_int == '-' as i32
                || isdigit(c as ::core::ffi::c_int) != 0
                || c as ::core::ffi::c_int == '.' as i32
            {
                obj = pst_parse_number(inbuf, inbufend);
            }
        }
    }
    if obj.is_null() {
        obj = pst_parse_any(inbuf, inbufend);
    }
    return obj;
}
unsafe extern "C" {
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
}
#[repr(C)]
pub struct pst_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pst_get_token(
    mut inbuf: *mut *mut ::core::ffi::c_uchar, mut inbufend: *mut ::core::ffi::c_uchar, _result: *mut *mut pst_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pst_get_token(inbuf, inbufend) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
