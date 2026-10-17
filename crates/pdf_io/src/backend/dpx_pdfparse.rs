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
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isxdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
    -> *mut ::core::ffi::c_char;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_null"]
    fn pdf_new_null() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_boolean"]
    fn pdf_new_boolean(value: ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_number_value"]
    fn pdf_number_value(number: *mut pdf_obj) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pdf_new_string"]
    fn pdf_new_string(str: *const ::core::ffi::c_void, length: size_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_merge_dict"]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_lookup_dict"]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const ::core::ffi::c_char) -> *mut pdf_obj;
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
    #[link_name = "__pdf_impl_pdf_deref_obj"]
    fn pdf_deref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_indirect"]
    fn pdf_new_indirect(pf: *mut pdf_file, label: uint32_t, generation: uint16_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_xtoi"]
    fn xtoi(c: ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint16_t = u16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tainted: ::core::ffi::c_int,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
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
pub const PDF_NUMBER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
static mut parser_state: C2RustUnnamed = C2RustUnnamed {
    tainted: 0 as ::core::ffi::c_int,
};
#[unsafe(export_name = "__pdf_impl_dump")]
pub unsafe extern "C-unwind" fn dump(
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) {
    let mut p: *const ::core::ffi::c_char = start;
    pdf_message!(b"\nCurrent input buffer is -->\0" as *const u8 as *const ::core::ffi::c_char,);
    while p < end && p < start.offset(DUMP_LIMIT as isize) {
        let fresh0 = p;
        p = p.offset(1);
        pdf_message!(
            b"%c\0" as *const u8 as *const ::core::ffi::c_char,
            *fresh0 as ::core::ffi::c_int,
        );
    }
    if p == start.offset(DUMP_LIMIT as isize) {
        pdf_message!(b"...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    pdf_message!(b"<--\n\0" as *const u8 as *const ::core::ffi::c_char);
}
pub const DUMP_LIMIT: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_pdfparse_skip_line")]
pub unsafe extern "C-unwind" fn pdfparse_skip_line(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) {
    while *start < end
        && **start as ::core::ffi::c_int != '\n' as i32
        && **start as ::core::ffi::c_int != '\r' as i32
    {
        *start = (*start).offset(1);
    }
    if *start < end && **start as ::core::ffi::c_int == '\r' as i32 {
        *start = (*start).offset(1);
    }
    if *start < end && **start as ::core::ffi::c_int == '\n' as i32 {
        *start = (*start).offset(1);
    }
}
#[unsafe(export_name = "__pdf_impl_skip_white")]
pub unsafe extern "C-unwind" fn skip_white(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) {
    while *start < end
        && (**start as ::core::ffi::c_int == ' ' as i32
            || **start as ::core::ffi::c_int == '\t' as i32
            || **start as ::core::ffi::c_int == '\u{c}' as i32
            || **start as ::core::ffi::c_int == '\r' as i32
            || **start as ::core::ffi::c_int == '\n' as i32
            || **start as ::core::ffi::c_int == '\0' as i32
            || **start as ::core::ffi::c_int == '%' as i32)
    {
        if **start as ::core::ffi::c_int == '%' as i32 {
            pdfparse_skip_line(start, end);
        } else {
            *start = (*start).offset(1);
        }
    }
}
unsafe extern "C-unwind" fn parsed_string(
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    len = end.offset_from(start) as ::core::ffi::c_long as ::core::ffi::c_int;
    if len > 0 as ::core::ffi::c_int {
        result = new(((len + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memcpy(
            result as *mut ::core::ffi::c_void,
            start as *const ::core::ffi::c_void,
            len as size_t,
        );
        *result.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    return result;
}
#[unsafe(export_name = "__pdf_impl_parse_number")]
pub unsafe extern "C-unwind" fn parse_number(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut number: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    skip_white(start, end);
    p = *start;
    if p < end && (*p as ::core::ffi::c_int == '+' as i32 || *p as ::core::ffi::c_int == '-' as i32)
    {
        p = p.offset(1);
    }
    while p < end && isdigit(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    if p < end && *p as ::core::ffi::c_int == '.' as i32 {
        p = p.offset(1);
        while p < end && isdigit(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            p = p.offset(1);
        }
    }
    number = parsed_string(*start, p);
    *start = p;
    return number;
}
#[unsafe(export_name = "__pdf_impl_parse_unsigned")]
pub unsafe extern "C-unwind" fn parse_unsigned(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut number: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    skip_white(start, end);
    p = *start;
    while p < end {
        if isdigit(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
            break;
        }
        p = p.offset(1);
    }
    number = parsed_string(*start, p);
    *start = p;
    return number;
}
unsafe extern "C-unwind" fn parse_gen_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut valid_chars: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = *start;
    while p < end {
        if strchr(valid_chars, *p as ::core::ffi::c_int).is_null() {
            break;
        }
        p = p.offset(1);
    }
    ident = parsed_string(*start, p);
    *start = p;
    return ident;
}
#[unsafe(export_name = "__pdf_impl_parse_ident")]
pub unsafe extern "C-unwind" fn parse_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut valid_chars: *const ::core::ffi::c_char =
        b"!\"#$&'*+,-.0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz|~\0"
            as *const u8 as *const ::core::ffi::c_char;
    return parse_gen_ident(start, end, valid_chars);
}
#[unsafe(export_name = "__pdf_impl_parse_val_ident")]
pub unsafe extern "C-unwind" fn parse_val_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut valid_chars: *const ::core::ffi::c_char =
        b"!\"#$&'*+,-./0123456789:;?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz|~\0"
            as *const u8 as *const ::core::ffi::c_char;
    return parse_gen_ident(start, end, valid_chars);
}
#[unsafe(export_name = "__pdf_impl_parse_opt_ident")]
pub unsafe extern "C-unwind" fn parse_opt_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if *start < end && **start as ::core::ffi::c_int == '@' as i32 {
        *start = (*start).offset(1);
        return parse_ident(start, end);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_number")]
pub unsafe extern "C-unwind" fn parse_pdf_number(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut v: ::core::ffi::c_double = 0.0f64;
    let mut nddigits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sign: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut has_dot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    p = *pp;
    skip_white(&raw mut p, endptr);
    if p >= endptr
        || isdigit(
            *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int,
        ) == 0
            && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '.' as i32
            && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '+' as i32
            && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32
    {
        pdf_warning!(
            b"Could not find a numeric object.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32 {
        if p.offset(1 as ::core::ffi::c_int as isize) >= endptr {
            pdf_warning!(
                b"Could not find a numeric object.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        sign = -(1 as ::core::ffi::c_int);
        p = p.offset(1);
    } else if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '+' as i32 {
        if p.offset(1 as ::core::ffi::c_int as isize) >= endptr {
            pdf_warning!(
                b"Could not find a numeric object.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        sign = 1 as ::core::ffi::c_int;
        p = p.offset(1);
    }
    while p < endptr
        && !(*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
            || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\t' as i32
            || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\u{c}' as i32
            || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\r' as i32
            || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
            || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
            || (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '(' as i32
                || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ')' as i32
                || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32
                || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '<' as i32
                || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '>' as i32
                || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '[' as i32
                || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ']' as i32
                || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '%' as i32))
    {
        if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '.' as i32 {
            if has_dot != 0 {
                pdf_warning!(
                    b"Could not find a numeric object.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return ::core::ptr::null_mut::<pdf_obj>();
            } else {
                has_dot = 1 as ::core::ffi::c_int;
            }
        } else if isdigit(
            *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int,
        ) != 0
        {
            if has_dot != 0 {
                v += (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - '0' as i32) as ::core::ffi::c_double
                    / pow(
                        10 as ::core::ffi::c_int as ::core::ffi::c_double,
                        (nddigits + 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
                    );
                nddigits += 1;
            } else {
                v = v * 10.0f64
                    + *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        as ::core::ffi::c_double
                    - '0' as i32 as ::core::ffi::c_double;
            }
        } else {
            pdf_warning!(
                b"Could not find a numeric object.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        p = p.offset(1);
    }
    *pp = p;
    return pdf_new_number(sign as ::core::ffi::c_double * v);
}
unsafe extern "C-unwind" fn pn_getc(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = *pp;
    if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32 {
        if p.offset(2 as ::core::ffi::c_int as isize) >= endptr {
            *pp = endptr;
            return -(1 as ::core::ffi::c_int);
        }
        if isxdigit(
            *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int,
        ) == 0
            || isxdigit(
                *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                    as ::core::ffi::c_int,
            ) == 0
        {
            *pp = (*pp).offset(3 as ::core::ffi::c_int as isize);
            return -(1 as ::core::ffi::c_int);
        }
        ch = xtoi(*p.offset(1 as ::core::ffi::c_int as isize)) << 4 as ::core::ffi::c_int;
        ch += xtoi(*p.offset(2 as ::core::ffi::c_int as isize));
        *pp = (*pp).offset(3 as ::core::ffi::c_int as isize);
    } else {
        ch = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *pp = (*pp).offset(1 as ::core::ffi::c_int as isize);
    }
    return ch;
}
pub const PDF_NAME_LEN_MAX: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const PDF_STRING_LEN_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
static mut sbuf: [::core::ffi::c_char; 65536] = [0; 65536];
#[unsafe(export_name = "__pdf_impl_parse_pdf_name")]
pub unsafe extern "C-unwind" fn parse_pdf_name(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut name: [::core::ffi::c_char; 129] = [0; 129];
    let mut ch: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    skip_white(pp, endptr);
    if *pp >= endptr || **pp as ::core::ffi::c_int != '/' as i32 {
        pdf_warning!(b"Could not find a name object.\0" as *const u8 as *const ::core::ffi::c_char,);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    *pp = (*pp).offset(1);
    while *pp < endptr
        && !(**pp as ::core::ffi::c_int == ' ' as i32
            || **pp as ::core::ffi::c_int == '\t' as i32
            || **pp as ::core::ffi::c_int == '\u{c}' as i32
            || **pp as ::core::ffi::c_int == '\r' as i32
            || **pp as ::core::ffi::c_int == '\n' as i32
            || **pp as ::core::ffi::c_int == '\0' as i32
            || (**pp as ::core::ffi::c_int == '(' as i32
                || **pp as ::core::ffi::c_int == ')' as i32
                || **pp as ::core::ffi::c_int == '/' as i32
                || **pp as ::core::ffi::c_int == '<' as i32
                || **pp as ::core::ffi::c_int == '>' as i32
                || **pp as ::core::ffi::c_int == '[' as i32
                || **pp as ::core::ffi::c_int == ']' as i32
                || **pp as ::core::ffi::c_int == '%' as i32))
    {
        ch = pn_getc(pp, endptr);
        if ch < 0 as ::core::ffi::c_int || ch > 0xff as ::core::ffi::c_int {
            pdf_warning!(
                b"Invalid char in PDF name object. (ignored)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        } else if ch == 0 as ::core::ffi::c_int {
            pdf_warning!(
                b"Null char not allowed in PDF name object. (ignored)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        } else if len < PDF_STRING_LEN_MAX + 1 as ::core::ffi::c_int {
            if len == PDF_NAME_LEN_MAX {
                pdf_warning!(
                    b"PDF name length too long. (>= %d bytes)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    PDF_NAME_LEN_MAX,
                );
            }
            let fresh1 = len;
            len = len + 1;
            name[fresh1 as usize] = ch as ::core::ffi::c_char;
        } else {
            pdf_warning!(
                b"PDF name length too long. (>= %d bytes, truncated)\0" as *const u8
                    as *const ::core::ffi::c_char,
                PDF_STRING_LEN_MAX + 1 as ::core::ffi::c_int,
            );
        }
    }
    if len < 1 as ::core::ffi::c_int {
        pdf_warning!(b"No valid name object found.\0" as *const u8 as *const ::core::ffi::c_char,);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    name[len as usize] = '\0' as i32 as ::core::ffi::c_char;
    return pdf_new_name(&raw mut name as *mut ::core::ffi::c_char);
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_boolean")]
pub unsafe extern "C-unwind" fn parse_pdf_boolean(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    skip_white(pp, endptr);
    if (*pp).offset(4 as ::core::ffi::c_int as isize) <= endptr
        && !strstartswith(*pp, b"true\0" as *const u8 as *const ::core::ffi::c_char).is_null()
    {
        if (*pp).offset(4 as ::core::ffi::c_int as isize) == endptr
            || (*(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\t' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\u{c}' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\r' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
                || (*(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '(' as i32
                    || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ')' as i32
                    || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '/' as i32
                    || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '<' as i32
                    || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '>' as i32
                    || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '[' as i32
                    || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ']' as i32
                    || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '%' as i32))
        {
            *pp = (*pp).offset(4 as ::core::ffi::c_int as isize);
            return pdf_new_boolean(1 as ::core::ffi::c_char);
        }
    } else if (*pp).offset(5 as ::core::ffi::c_int as isize) <= endptr
        && !strstartswith(*pp, b"false\0" as *const u8 as *const ::core::ffi::c_char).is_null()
    {
        if (*pp).offset(5 as ::core::ffi::c_int as isize) == endptr
            || (*(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
                || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\t' as i32
                || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\u{c}' as i32
                || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\r' as i32
                || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32
                || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
                || (*(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '(' as i32
                    || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ')' as i32
                    || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '/' as i32
                    || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '<' as i32
                    || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '>' as i32
                    || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '[' as i32
                    || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == ']' as i32
                    || *(*pp).offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '%' as i32))
        {
            *pp = (*pp).offset(5 as ::core::ffi::c_int as isize);
            return pdf_new_boolean(0 as ::core::ffi::c_char);
        }
    }
    pdf_warning!(b"Not a boolean object.\0" as *const u8 as *const ::core::ffi::c_char);
    return ::core::ptr::null_mut::<pdf_obj>();
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_null")]
pub unsafe extern "C-unwind" fn parse_pdf_null(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    skip_white(pp, endptr);
    if (*pp).offset(4 as ::core::ffi::c_int as isize) > endptr {
        pdf_warning!(b"Not a null object.\0" as *const u8 as *const ::core::ffi::c_char);
        return ::core::ptr::null_mut::<pdf_obj>();
    } else if (*pp).offset(4 as ::core::ffi::c_int as isize) < endptr
        && !(*(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ' ' as i32
            || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\t' as i32
            || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\u{c}' as i32
            || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\r' as i32
            || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
            || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
            || (*(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '(' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ')' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '/' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '<' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '>' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '[' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ']' as i32
                || *(*pp).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '%' as i32))
    {
        pdf_warning!(b"Not a null object.\0" as *const u8 as *const ::core::ffi::c_char);
        return ::core::ptr::null_mut::<pdf_obj>();
    } else if !strstartswith(*pp, b"null\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        *pp = (*pp).offset(4 as ::core::ffi::c_int as isize);
        return pdf_new_null();
    }
    pdf_warning!(b"Not a null object.\0" as *const u8 as *const ::core::ffi::c_char);
    return ::core::ptr::null_mut::<pdf_obj>();
}
unsafe extern "C-unwind" fn ps_getescc(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = (*pp).offset(1 as ::core::ffi::c_int as isize);
    match *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
        110 => {
            ch = '\n' as i32;
            p = p.offset(1);
        }
        114 => {
            ch = '\r' as i32;
            p = p.offset(1);
        }
        116 => {
            ch = '\t' as i32;
            p = p.offset(1);
        }
        98 => {
            ch = '\u{8}' as i32;
            p = p.offset(1);
        }
        102 => {
            ch = '\u{c}' as i32;
            p = p.offset(1);
        }
        10 => {
            ch = -(1 as ::core::ffi::c_int);
            p = p.offset(1);
        }
        13 => {
            ch = -(1 as ::core::ffi::c_int);
            p = p.offset(1);
            if p < endptr
                && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
            {
                p = p.offset(1);
            }
        }
        _ => {
            if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\\' as i32
                || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '(' as i32
                || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ')' as i32
            {
                ch = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
                p = p.offset(1);
            } else if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= '0' as i32
                && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '7' as i32
            {
                ch = 0 as ::core::ffi::c_int;
                i = 0 as ::core::ffi::c_int;
                while i < 3 as ::core::ffi::c_int
                    && p < endptr
                    && (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        >= '0' as i32
                        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            <= '7' as i32)
                {
                    ch = (ch << 3 as ::core::ffi::c_int)
                        + (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            - '0' as i32);
                    p = p.offset(1);
                    i += 1;
                }
                ch = ch & 0xff as ::core::ffi::c_int;
            } else {
                ch = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                    as ::core::ffi::c_int;
                p = p.offset(1);
            }
        }
    }
    *pp = p;
    return ch;
}
unsafe extern "C-unwind" fn parse_pdf_literal_string(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut ch: ::core::ffi::c_int = 0;
    let mut op_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = *pp;
    skip_white(&raw mut p, endptr);
    if p >= endptr
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '(' as i32
    {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    p = p.offset(1);
    while p < endptr {
        ch = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        if ch == ')' as i32 && op_count < 1 as ::core::ffi::c_int {
            break;
        }
        if parser_state.tainted != 0 {
            if p.offset(1 as ::core::ffi::c_int as isize) < endptr
                && ch & 0x80 as ::core::ffi::c_int != 0
            {
                if len + 2 as ::core::ffi::c_int >= PDF_STRING_LEN_MAX {
                    pdf_warning!(
                        b"PDF string length too long. (limit: %d)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        PDF_STRING_LEN_MAX,
                    );
                    return ::core::ptr::null_mut::<pdf_obj>();
                }
                let fresh3 = len;
                len = len + 1;
                sbuf[fresh3 as usize] = *p.offset(0 as ::core::ffi::c_int as isize);
                let fresh4 = len;
                len = len + 1;
                sbuf[fresh4 as usize] = *p.offset(1 as ::core::ffi::c_int as isize);
                p = p.offset(2 as ::core::ffi::c_int as isize);
                continue;
            }
        }
        if len + 1 as ::core::ffi::c_int >= PDF_STRING_LEN_MAX {
            pdf_warning!(
                b"PDF string length too long. (limit: %d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                PDF_STRING_LEN_MAX,
            );
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        match ch {
            92 => {
                ch = ps_getescc(&raw mut p, endptr);
                if ch >= 0 as ::core::ffi::c_int {
                    let fresh5 = len;
                    len = len + 1;
                    sbuf[fresh5 as usize] =
                        (ch & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
                }
            }
            13 => {
                p = p.offset(1);
                if p < endptr
                    && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '\n' as i32
                {
                    p = p.offset(1);
                }
                let fresh6 = len;
                len = len + 1;
                sbuf[fresh6 as usize] = '\n' as i32 as ::core::ffi::c_char;
            }
            _ => {
                if ch == '(' as i32 {
                    op_count += 1;
                } else if ch == ')' as i32 {
                    op_count -= 1;
                }
                let fresh7 = len;
                len = len + 1;
                sbuf[fresh7 as usize] = ch as ::core::ffi::c_char;
                p = p.offset(1);
            }
        }
    }
    if op_count > 0 as ::core::ffi::c_int
        || p >= endptr
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ')' as i32
    {
        pdf_warning!(
            b"Unbalanced parens/truncated PDF literal string.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    *pp = p.offset(1 as ::core::ffi::c_int as isize);
    return pdf_new_string(
        &raw mut sbuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
    );
}
unsafe extern "C-unwind" fn parse_pdf_hex_string(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    p = *pp;
    skip_white(&raw mut p, endptr);
    if p >= endptr
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '<' as i32
    {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    p = p.offset(1);
    len = 0 as ::core::ffi::c_int;
    while p < endptr
        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '>' as i32
        && len < PDF_STRING_LEN_MAX
    {
        let mut ch: ::core::ffi::c_int = 0;
        skip_white(&raw mut p, endptr);
        if p >= endptr
            || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '>' as i32
        {
            break;
        }
        ch = xtoi(*p.offset(0 as ::core::ffi::c_int as isize)) << 4 as ::core::ffi::c_int;
        p = p.offset(1);
        skip_white(&raw mut p, endptr);
        if p < endptr
            && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '>' as i32
        {
            ch += xtoi(*p.offset(0 as ::core::ffi::c_int as isize));
            p = p.offset(1);
        }
        let fresh2 = len;
        len = len + 1;
        sbuf[fresh2 as usize] = (ch & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
    }
    if p >= endptr {
        pdf_warning!(
            b"Premature end of input hex string.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    } else if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '>' as i32 {
        pdf_warning!(
            b"PDF string length too long. (limit: %d)\0" as *const u8 as *const ::core::ffi::c_char,
            PDF_STRING_LEN_MAX,
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    *pp = p.offset(1 as ::core::ffi::c_int as isize);
    return pdf_new_string(
        &raw mut sbuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
    );
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_string")]
pub unsafe extern "C-unwind" fn parse_pdf_string(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    skip_white(pp, endptr);
    if (*pp).offset(2 as ::core::ffi::c_int as isize) <= endptr {
        if **pp as ::core::ffi::c_int == '(' as i32 {
            return parse_pdf_literal_string(pp, endptr);
        } else if **pp as ::core::ffi::c_int == '<' as i32
            && (*(*pp).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '>' as i32
                || isxdigit(
                    *(*pp).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int,
                ) != 0)
        {
            return parse_pdf_hex_string(pp, endptr);
        }
    }
    pdf_warning!(b"Could not find a string object.\0" as *const u8 as *const ::core::ffi::c_char,);
    return ::core::ptr::null_mut::<pdf_obj>();
}
unsafe extern "C-unwind" fn parse_pdf_dict_extended(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
    mut unknown_handler: Option<
        unsafe extern "C-unwind" fn(
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> *mut pdf_obj,
    >,
    mut user_data: *mut ::core::ffi::c_void,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = *pp;
    skip_white(&raw mut p, endptr);
    if p.offset(4 as ::core::ffi::c_int as isize) > endptr
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '<' as i32
        || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '<' as i32
    {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    p = p.offset(2 as ::core::ffi::c_int as isize);
    result = pdf_new_dict();
    skip_white(&raw mut p, endptr);
    while p < endptr
        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '>' as i32
    {
        let mut key: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut value: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        skip_white(&raw mut p, endptr);
        key = parse_pdf_name(&raw mut p, endptr);
        if key.is_null() {
            pdf_warning!(
                b"Could not find a key in dictionary object.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            pdf_release_obj(result);
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        skip_white(&raw mut p, endptr);
        value = parse_pdf_object_extended(&raw mut p, endptr, pf, unknown_handler, user_data);
        if value.is_null() {
            pdf_release_obj(key);
            pdf_release_obj(value);
            pdf_release_obj(result);
            pdf_warning!(
                b"Could not find a value in dictionary object.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        pdf_add_dict(result, key, value);
        skip_white(&raw mut p, endptr);
    }
    if p.offset(2 as ::core::ffi::c_int as isize) > endptr
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '>' as i32
        || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '>' as i32
    {
        pdf_warning!(
            b"Syntax error: Dictionary object ended prematurely.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        pdf_release_obj(result);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    *pp = p.offset(2 as ::core::ffi::c_int as isize);
    return result;
}
unsafe extern "C-unwind" fn parse_pdf_array_extended(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
    mut unknown_handler: Option<
        unsafe extern "C-unwind" fn(
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> *mut pdf_obj,
    >,
    mut user_data: *mut ::core::ffi::c_void,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = *pp;
    skip_white(&raw mut p, endptr);
    if p.offset(2 as ::core::ffi::c_int as isize) > endptr
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '[' as i32
    {
        pdf_warning!(
            b"Could not find an array object.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    result = pdf_new_array();
    p = p.offset(1);
    skip_white(&raw mut p, endptr);
    while p < endptr
        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ']' as i32
    {
        let mut elem: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        elem = parse_pdf_object_extended(&raw mut p, endptr, pf, unknown_handler, user_data);
        if elem.is_null() {
            pdf_release_obj(result);
            pdf_warning!(
                b"Could not find a valid object in array object.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        pdf_add_array(result, elem);
        skip_white(&raw mut p, endptr);
    }
    if p >= endptr
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ']' as i32
    {
        pdf_warning!(
            b"Array object ended prematurely.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        pdf_release_obj(result);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    *pp = p.offset(1 as ::core::ffi::c_int as isize);
    return result;
}
unsafe extern "C-unwind" fn parse_pdf_stream(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut dict: *mut pdf_obj,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_length: ::core::ffi::c_int = 0;
    p = *pp;
    skip_white(&raw mut p, endptr);
    if p.offset(6 as ::core::ffi::c_int as isize) > endptr
        || strstartswith(p, b"stream\0" as *const u8 as *const ::core::ffi::c_char).is_null()
    {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    p = p.offset(6 as ::core::ffi::c_int as isize);
    if p < endptr
        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
    {
        p = p.offset(1);
    } else if p.offset(1 as ::core::ffi::c_int as isize) < endptr
        && (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\r' as i32
            && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32)
    {
        p = p.offset(2 as ::core::ffi::c_int as isize);
    }
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut tmp2: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    tmp = pdf_lookup_dict(dict, b"Length\0" as *const u8 as *const ::core::ffi::c_char);
    if !tmp.is_null() {
        tmp2 = pdf_deref_obj(tmp);
        if pdf_obj_typeof(tmp2) != PDF_NUMBER {
            stream_length = -(1 as ::core::ffi::c_int);
        } else {
            stream_length = pdf_number_value(tmp2) as ::core::ffi::c_int;
        }
        pdf_release_obj(tmp2);
    } else {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if stream_length < 0 as ::core::ffi::c_int || p.offset(stream_length as isize) > endptr {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    let mut filters: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    filters = pdf_lookup_dict(dict, b"Filter\0" as *const u8 as *const ::core::ffi::c_char);
    if filters.is_null() && stream_length > 10 as ::core::ffi::c_int {
        result = pdf_new_stream(STREAM_COMPRESS);
    } else {
        result = pdf_new_stream(0 as ::core::ffi::c_int);
    }
    stream_dict = pdf_stream_dict(result);
    pdf_merge_dict(stream_dict, dict);
    pdf_add_stream(result, p as *const ::core::ffi::c_void, stream_length);
    p = p.offset(stream_length as isize);
    if p < endptr
        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\r' as i32
    {
        p = p.offset(1);
    }
    if p < endptr
        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
    {
        p = p.offset(1);
    }
    if p.offset(9 as ::core::ffi::c_int as isize) > endptr
        || memcmp(
            p as *const ::core::ffi::c_void,
            b"endstream\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            9 as size_t,
        ) != 0
    {
        pdf_release_obj(result);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    p = p.offset(9 as ::core::ffi::c_int as isize);
    *pp = p;
    return result;
}
unsafe extern "C-unwind" fn try_pdf_reference(
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut endptr: *mut *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    let mut id: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut r#gen: ::core::ffi::c_ushort = 0 as ::core::ffi::c_ushort;
    if pf.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"pf\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !endptr.is_null() {
        *endptr = start;
    }
    skip_white(&raw mut start, end);
    if start > end.offset(-(5 as ::core::ffi::c_int as isize))
        || isdigit(*start as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0
    {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    while !(*start as ::core::ffi::c_int == ' ' as i32
        || *start as ::core::ffi::c_int == '\t' as i32
        || *start as ::core::ffi::c_int == '\u{c}' as i32
        || *start as ::core::ffi::c_int == '\r' as i32
        || *start as ::core::ffi::c_int == '\n' as i32
        || *start as ::core::ffi::c_int == '\0' as i32)
    {
        if start >= end || isdigit(*start as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        id = id
            .wrapping_mul(10 as ::core::ffi::c_uint)
            .wrapping_add((*start as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uint);
        start = start.offset(1);
    }
    skip_white(&raw mut start, end);
    if start >= end || isdigit(*start as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    while !(*start as ::core::ffi::c_int == ' ' as i32
        || *start as ::core::ffi::c_int == '\t' as i32
        || *start as ::core::ffi::c_int == '\u{c}' as i32
        || *start as ::core::ffi::c_int == '\r' as i32
        || *start as ::core::ffi::c_int == '\n' as i32
        || *start as ::core::ffi::c_int == '\0' as i32)
    {
        if start >= end || isdigit(*start as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        r#gen = (r#gen as ::core::ffi::c_int * 10 as ::core::ffi::c_int
            + (*start as ::core::ffi::c_int - '0' as i32)) as ::core::ffi::c_ushort;
        start = start.offset(1);
    }
    skip_white(&raw mut start, end);
    if start >= end || *start as ::core::ffi::c_int != 'R' as i32 {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    start = start.offset(1);
    if !(start >= end
        || (*start as ::core::ffi::c_int == ' ' as i32
            || *start as ::core::ffi::c_int == '\t' as i32
            || *start as ::core::ffi::c_int == '\u{c}' as i32
            || *start as ::core::ffi::c_int == '\r' as i32
            || *start as ::core::ffi::c_int == '\n' as i32
            || *start as ::core::ffi::c_int == '\0' as i32)
        || (*start as ::core::ffi::c_int == '(' as i32
            || *start as ::core::ffi::c_int == ')' as i32
            || *start as ::core::ffi::c_int == '/' as i32
            || *start as ::core::ffi::c_int == '<' as i32
            || *start as ::core::ffi::c_int == '>' as i32
            || *start as ::core::ffi::c_int == '[' as i32
            || *start as ::core::ffi::c_int == ']' as i32
            || *start as ::core::ffi::c_int == '%' as i32))
    {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if !endptr.is_null() {
        *endptr = start;
    }
    return pdf_new_indirect(pf, id as uint32_t, r#gen as uint16_t);
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_object_extended")]
pub unsafe extern "C-unwind" fn parse_pdf_object_extended(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
    mut unknown_handler: Option<
        unsafe extern "C-unwind" fn(
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> *mut pdf_obj,
    >,
    mut user_data: *mut ::core::ffi::c_void,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut nextptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    skip_white(pp, endptr);
    if *pp >= endptr {
        pdf_warning!(
            b"Could not find any valid object.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    match **pp as ::core::ffi::c_int {
        60 => {
            if *(*pp).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '<' as i32 {
                result = parse_pdf_hex_string(pp, endptr);
            } else {
                let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                result = parse_pdf_dict_extended(pp, endptr, pf, unknown_handler, user_data);
                skip_white(pp, endptr);
                if !result.is_null()
                    && *pp <= endptr.offset(-(15 as ::core::ffi::c_int as isize))
                    && memcmp(
                        *pp as *const ::core::ffi::c_void,
                        b"stream\0" as *const u8 as *const ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        6 as size_t,
                    ) == 0
                {
                    dict = result;
                    result = parse_pdf_stream(pp, endptr, dict);
                    pdf_release_obj(dict);
                }
            }
        }
        40 => {
            result = parse_pdf_string(pp, endptr);
        }
        91 => {
            result = parse_pdf_array_extended(pp, endptr, pf, unknown_handler, user_data);
        }
        47 => {
            result = parse_pdf_name(pp, endptr);
        }
        110 => {
            result = parse_pdf_null(pp, endptr);
        }
        116 | 102 => {
            result = parse_pdf_boolean(pp, endptr);
        }
        43 | 45 | 46 => {
            result = parse_pdf_number(pp, endptr);
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
            if !pf.is_null() && {
                result = try_pdf_reference(*pp, endptr, &raw mut nextptr, pf);
                !result.is_null()
            } {
                *pp = nextptr;
            } else {
                result = parse_pdf_number(pp, endptr);
            }
        }
        _ => {
            if unknown_handler.is_some() {
                result = unknown_handler.expect("non-null function pointer")(pp, endptr, user_data);
            } else {
                pdf_warning!(
                    b"Unknown PDF object type.\0" as *const u8 as *const ::core::ffi::c_char,
                );
                result = ::core::ptr::null_mut::<pdf_obj>();
            }
        }
    }
    return result;
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_dict")]
pub unsafe extern "C-unwind" fn parse_pdf_dict(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    return parse_pdf_dict_extended(pp, endptr, pf, None, NULL);
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_array")]
pub unsafe extern "C-unwind" fn parse_pdf_array(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    return parse_pdf_array_extended(pp, endptr, pf, None, NULL);
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_object")]
pub unsafe extern "C-unwind" fn parse_pdf_object(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    return parse_pdf_object_extended(pp, endptr, pf, None, NULL);
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_tainted_dict")]
pub unsafe extern "C-unwind" fn parse_pdf_tainted_dict(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut unknown_handler: Option<
        unsafe extern "C-unwind" fn(
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> *mut pdf_obj,
    >,
    mut user_data: *mut ::core::ffi::c_void,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    parser_state.tainted = 1 as ::core::ffi::c_int;
    result = parse_pdf_dict_extended(
        pp,
        endptr,
        ::core::ptr::null_mut::<pdf_file>(),
        unknown_handler,
        user_data,
    );
    parser_state.tainted = 0 as ::core::ffi::c_int;
    return result;
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
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pdf_file {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dump(
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dump(start, end) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdfparse_skip_line(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdfparse_skip_line(start, end) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_skip_white(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { skip_white(start, end) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_number(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_number(start, end) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_unsigned(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_unsigned(start, end) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_ident(start, end) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_val_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_val_ident(start, end) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_opt_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_opt_ident(start, end) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_pdf_number(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_number(pp, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_pdf_name(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_name(pp, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_pdf_boolean(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_boolean(pp, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_pdf_null(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_null(pp, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_pdf_string(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_string(pp, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_pdf_object_extended(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
    mut unknown_handler: Option<
        unsafe extern "C-unwind" fn(
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> *mut pdf_obj,
    >,
    mut user_data: *mut ::core::ffi::c_void,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        parse_pdf_object_extended(pp, endptr, pf, unknown_handler, user_data)
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
pub unsafe extern "C" fn __pdf_entry_parse_pdf_dict(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_dict(pp, endptr, pf) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_pdf_array(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_array(pp, endptr, pf) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_pdf_object(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_object(pp, endptr, pf) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_pdf_tainted_dict(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut unknown_handler: Option<
        unsafe extern "C-unwind" fn(
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> *mut pdf_obj,
    >,
    mut user_data: *mut ::core::ffi::c_void,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        parse_pdf_tainted_dict(pp, endptr, unknown_handler, user_data)
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
