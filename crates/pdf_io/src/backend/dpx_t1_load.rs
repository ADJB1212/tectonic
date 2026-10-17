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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn xstrdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: ::core::ffi::c_int,
    ) -> size_t;
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> ssize_t;
    fn ttstub_input_getc(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_close"]
    fn cff_close(cff: *mut cff_font);
    #[link_name = "__pdf_impl_cff_new_index"]
    fn cff_new_index(count: card16) -> *mut cff_index;
    #[link_name = "__pdf_impl_cff_set_name"]
    fn cff_set_name(cff: *mut cff_font, name: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_get_sid"]
    fn cff_get_sid(cff: *mut cff_font, str: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_add_string"]
    fn cff_add_string(
        cff: *mut cff_font,
        str: *const ::core::ffi::c_char,
        unique: ::core::ffi::c_int,
    ) -> s_SID;
    #[link_name = "__pdf_impl_cff_update_string"]
    fn cff_update_string(cff: *mut cff_font);
    #[link_name = "__pdf_impl_cff_new_dict"]
    fn cff_new_dict() -> *mut cff_dict;
    #[link_name = "__pdf_impl_cff_dict_set"]
    fn cff_dict_set(
        dict: *mut cff_dict,
        key: *const ::core::ffi::c_char,
        idx: ::core::ffi::c_int,
        value: ::core::ffi::c_double,
    );
    #[link_name = "__pdf_impl_cff_dict_add"]
    fn cff_dict_add(
        dict: *mut cff_dict,
        key: *const ::core::ffi::c_char,
        count: ::core::ffi::c_int,
    );
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pst_get_token"]
    fn pst_get_token(
        inbuf: *mut *mut ::core::ffi::c_uchar,
        inbufend: *mut ::core::ffi::c_uchar,
    ) -> *mut pst_obj;
    #[link_name = "__pdf_impl_pst_release_obj"]
    fn pst_release_obj(obj: *mut pst_obj);
    #[link_name = "__pdf_impl_pst_type_of"]
    fn pst_type_of(obj: *mut pst_obj) -> pst_type;
    #[link_name = "__pdf_impl_pst_length_of"]
    fn pst_length_of(obj: *mut pst_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pst_getIV"]
    fn pst_getIV(obj: *mut pst_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pst_getRV"]
    fn pst_getRV(obj: *mut pst_obj) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pst_getSV"]
    fn pst_getSV(obj: *mut pst_obj) -> *mut ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_pst_data_ptr"]
    fn pst_data_ptr(obj: *mut pst_obj) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
pub type rust_input_handle_t = uintptr_t;
pub type card8 = ::core::ffi::c_uchar;
pub type card16 = ::core::ffi::c_ushort;
pub type c_offsize = ::core::ffi::c_uchar;
pub type l_offset = uint32_t;
pub type s_SID = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_index {
    pub count: card16,
    pub offsize: c_offsize,
    pub offset: *mut l_offset,
    pub data: *mut card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_header {
    pub major: card8,
    pub minor: card8,
    pub hdr_size: card8,
    pub offsize: c_offsize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict_entry {
    pub id: ::core::ffi::c_int,
    pub key: *const ::core::ffi::c_char,
    pub count: ::core::ffi::c_int,
    pub values: *mut ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_dict {
    pub max: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub entries: *mut cff_dict_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range1 {
    pub first: s_SID,
    pub n_left: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range2 {
    pub first: s_SID,
    pub n_left: card16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_map {
    pub code: card8,
    pub glyph: s_SID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_encoding {
    pub format: card8,
    pub num_entries: card8,
    pub data: C2RustUnnamed,
    pub num_supps: card8,
    pub supp: *mut cff_map,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub codes: *mut card8,
    pub range1: *mut cff_range1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_charsets {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub glyphs: *mut s_SID,
    pub range1: *mut cff_range1,
    pub range2: *mut cff_range2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_range3 {
    pub first: card16,
    pub fd: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_fdselect {
    pub format: card8,
    pub num_entries: card16,
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub fds: *mut card8,
    pub ranges: *mut cff_range3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_font {
    pub fontname: *mut ::core::ffi::c_char,
    pub header: cff_header,
    pub name: *mut cff_index,
    pub topdict: *mut cff_dict,
    pub string: *mut cff_index,
    pub gsubr: *mut cff_index,
    pub encoding: *mut cff_encoding,
    pub charsets: *mut cff_charsets,
    pub fdselect: *mut cff_fdselect,
    pub cstrings: *mut cff_index,
    pub fdarray: *mut *mut cff_dict,
    pub private: *mut *mut cff_dict,
    pub subrs: *mut *mut cff_index,
    pub offset: l_offset,
    pub gsubr_offset: l_offset,
    pub num_glyphs: card16,
    pub num_fds: card8,
    pub _string: *mut cff_index,
    pub handle: rust_input_handle_t,
    pub filter: ::core::ffi::c_int,
    pub index: ::core::ffi::c_int,
    pub flag: ::core::ffi::c_int,
    pub is_notdef_notzero: ::core::ffi::c_int,
}
pub type pst_type = ::core::ffi::c_int;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
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
pub const CFF_SID_MAX: ::core::ffi::c_int = 64999 as ::core::ffi::c_int;
pub const CS_STR_LEN_MAX: ::core::ffi::c_int = 65536 as ::core::ffi::c_int;
pub const FONTTYPE_FONT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const CFF_STRING_NOTDEF: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const PST_TYPE_BOOLEAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PST_TYPE_INTEGER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PST_TYPE_REAL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PST_TYPE_STRING: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PST_TYPE_NAME: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PST_TYPE_MARK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const T1_EEKEY: ::core::ffi::c_uint = 55665 as ::core::ffi::c_uint;
pub const T1_CHARKEY: ::core::ffi::c_uint = 4330 as ::core::ffi::c_uint;
unsafe extern "C-unwind" fn t1_decrypt(
    mut key: ::core::ffi::c_ushort,
    mut dst: *mut ::core::ffi::c_uchar,
    mut src: *const ::core::ffi::c_uchar,
    mut skip: ::core::ffi::c_int,
    mut len: ::core::ffi::c_int,
) {
    len -= skip;
    loop {
        let fresh4 = skip;
        skip = skip - 1;
        if !(fresh4 != 0) {
            break;
        }
        let fresh5 = src;
        src = src.offset(1);
        key = ((key as ::core::ffi::c_int + *fresh5 as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(52845 as ::core::ffi::c_uint)
            .wrapping_add(22719 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
    }
    loop {
        let fresh6 = len;
        len = len - 1;
        if !(fresh6 != 0) {
            break;
        }
        let fresh7 = src;
        src = src.offset(1);
        let mut c: ::core::ffi::c_uchar = *fresh7;
        let fresh8 = dst;
        dst = dst.offset(1);
        *fresh8 = (c as ::core::ffi::c_int ^ key as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        key = ((key as ::core::ffi::c_int + c as ::core::ffi::c_int) as ::core::ffi::c_uint)
            .wrapping_mul(52845 as ::core::ffi::c_uint)
            .wrapping_add(22719 as ::core::ffi::c_uint) as ::core::ffi::c_ushort;
    }
}
unsafe extern "C-unwind" fn get_next_key(
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
) -> *mut ::core::ffi::c_char {
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    while *start < end && {
        tok = pst_get_token(start, end);
        !tok.is_null()
    } {
        if pst_type_of(tok) == PST_TYPE_NAME {
            key = pst_getSV(tok) as *mut ::core::ffi::c_char;
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            break;
        } else if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
    }
    return key;
}
unsafe extern "C-unwind" fn seek_operator(
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut op: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    while *start < end && {
        tok = pst_get_token(start, end);
        !tok.is_null()
    } {
        if !tok.is_null()
            && pst_type_of(tok) < 0 as ::core::ffi::c_int
            && pst_length_of(tok) as size_t == strlen(op)
            && !strstartswith(pst_data_ptr(tok) as *const ::core::ffi::c_char, op).is_null()
        {
            break;
        }
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
    }
    if tok.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = ::core::ptr::null_mut::<pst_obj>();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn parse_svalue(
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut value: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    tok = pst_get_token(start, end);
    if tok.is_null() {
        return -(1 as ::core::ffi::c_int);
    } else if pst_type_of(tok) == PST_TYPE_NAME || pst_type_of(tok) == PST_TYPE_STRING {
        *value = pst_getSV(tok) as *mut ::core::ffi::c_char;
    } else {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        return -(1 as ::core::ffi::c_int);
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = ::core::ptr::null_mut::<pst_obj>();
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn parse_bvalue(
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut value: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    tok = pst_get_token(start, end);
    if tok.is_null() {
        return -(1 as ::core::ffi::c_int);
    } else if pst_type_of(tok) == PST_TYPE_BOOLEAN {
        *value = pst_getIV(tok) as ::core::ffi::c_double;
    } else {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        return -(1 as ::core::ffi::c_int);
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = ::core::ptr::null_mut::<pst_obj>();
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn parse_nvalue(
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut value: *mut ::core::ffi::c_double,
    mut max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut argn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    tok = pst_get_token(start, end);
    if tok.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (pst_type_of(tok) == PST_TYPE_INTEGER || pst_type_of(tok) == PST_TYPE_REAL)
        && max > 0 as ::core::ffi::c_int
    {
        *value.offset(0 as ::core::ffi::c_int as isize) = pst_getRV(tok);
        argn = 1 as ::core::ffi::c_int;
    } else if pst_type_of(tok) == PST_TYPE_MARK {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        while *start < end
            && {
                tok = pst_get_token(start, end);
                !tok.is_null()
            }
            && (pst_type_of(tok) == PST_TYPE_INTEGER || pst_type_of(tok) == PST_TYPE_REAL)
            && argn < max
        {
            let fresh1 = argn;
            argn = argn + 1;
            *value.offset(fresh1 as isize) = pst_getRV(tok);
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
        }
        if tok.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        if !(!tok.is_null()
            && pst_type_of(tok) < 0 as ::core::ffi::c_int
            && pst_length_of(tok) as size_t
                == strlen(b"]\0" as *const u8 as *const ::core::ffi::c_char)
            && !strstartswith(
                pst_data_ptr(tok) as *const ::core::ffi::c_char,
                b"]\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null())
            && !(!tok.is_null()
                && pst_type_of(tok) < 0 as ::core::ffi::c_int
                && pst_length_of(tok) as size_t
                    == strlen(b"}\0" as *const u8 as *const ::core::ffi::c_char)
                && !strstartswith(
                    pst_data_ptr(tok) as *const ::core::ffi::c_char,
                    b"}\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null())
        {
            argn = -(1 as ::core::ffi::c_int);
        }
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = ::core::ptr::null_mut::<pst_obj>();
    }
    return argn;
}
static mut StandardEncoding: [*const ::core::ffi::c_char; 256] = [
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"space\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclam\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"numbersign\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollar\0" as *const u8 as *const ::core::ffi::c_char,
    b"percent\0" as *const u8 as *const ::core::ffi::c_char,
    b"ampersand\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteright\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asterisk\0" as *const u8 as *const ::core::ffi::c_char,
    b"plus\0" as *const u8 as *const ::core::ffi::c_char,
    b"comma\0" as *const u8 as *const ::core::ffi::c_char,
    b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
    b"period\0" as *const u8 as *const ::core::ffi::c_char,
    b"slash\0" as *const u8 as *const ::core::ffi::c_char,
    b"zero\0" as *const u8 as *const ::core::ffi::c_char,
    b"one\0" as *const u8 as *const ::core::ffi::c_char,
    b"two\0" as *const u8 as *const ::core::ffi::c_char,
    b"three\0" as *const u8 as *const ::core::ffi::c_char,
    b"four\0" as *const u8 as *const ::core::ffi::c_char,
    b"five\0" as *const u8 as *const ::core::ffi::c_char,
    b"six\0" as *const u8 as *const ::core::ffi::c_char,
    b"seven\0" as *const u8 as *const ::core::ffi::c_char,
    b"eight\0" as *const u8 as *const ::core::ffi::c_char,
    b"nine\0" as *const u8 as *const ::core::ffi::c_char,
    b"colon\0" as *const u8 as *const ::core::ffi::c_char,
    b"semicolon\0" as *const u8 as *const ::core::ffi::c_char,
    b"less\0" as *const u8 as *const ::core::ffi::c_char,
    b"equal\0" as *const u8 as *const ::core::ffi::c_char,
    b"greater\0" as *const u8 as *const ::core::ffi::c_char,
    b"question\0" as *const u8 as *const ::core::ffi::c_char,
    b"at\0" as *const u8 as *const ::core::ffi::c_char,
    b"A\0" as *const u8 as *const ::core::ffi::c_char,
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"C\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"E\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"I\0" as *const u8 as *const ::core::ffi::c_char,
    b"J\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"O\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Q\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"U\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
    b"X\0" as *const u8 as *const ::core::ffi::c_char,
    b"Y\0" as *const u8 as *const ::core::ffi::c_char,
    b"Z\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"backslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char,
    b"underscore\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"b\0" as *const u8 as *const ::core::ffi::c_char,
    b"c\0" as *const u8 as *const ::core::ffi::c_char,
    b"d\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"f\0" as *const u8 as *const ::core::ffi::c_char,
    b"g\0" as *const u8 as *const ::core::ffi::c_char,
    b"h\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"j\0" as *const u8 as *const ::core::ffi::c_char,
    b"k\0" as *const u8 as *const ::core::ffi::c_char,
    b"l\0" as *const u8 as *const ::core::ffi::c_char,
    b"m\0" as *const u8 as *const ::core::ffi::c_char,
    b"n\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"p\0" as *const u8 as *const ::core::ffi::c_char,
    b"q\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\0" as *const u8 as *const ::core::ffi::c_char,
    b"s\0" as *const u8 as *const ::core::ffi::c_char,
    b"t\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"v\0" as *const u8 as *const ::core::ffi::c_char,
    b"w\0" as *const u8 as *const ::core::ffi::c_char,
    b"x\0" as *const u8 as *const ::core::ffi::c_char,
    b"y\0" as *const u8 as *const ::core::ffi::c_char,
    b"z\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"bar\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char,
    b"cent\0" as *const u8 as *const ::core::ffi::c_char,
    b"sterling\0" as *const u8 as *const ::core::ffi::c_char,
    b"fraction\0" as *const u8 as *const ::core::ffi::c_char,
    b"yen\0" as *const u8 as *const ::core::ffi::c_char,
    b"florin\0" as *const u8 as *const ::core::ffi::c_char,
    b"section\0" as *const u8 as *const ::core::ffi::c_char,
    b"currency\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char,
    b"fi\0" as *const u8 as *const ::core::ffi::c_char,
    b"fl\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"endash\0" as *const u8 as *const ::core::ffi::c_char,
    b"dagger\0" as *const u8 as *const ::core::ffi::c_char,
    b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"paragraph\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char,
    b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char,
    b"perthousand\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"grave\0" as *const u8 as *const ::core::ffi::c_char,
    b"acute\0" as *const u8 as *const ::core::ffi::c_char,
    b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"macron\0" as *const u8 as *const ::core::ffi::c_char,
    b"breve\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"ring\0" as *const u8 as *const ::core::ffi::c_char,
    b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char,
    b"ogonek\0" as *const u8 as *const ::core::ffi::c_char,
    b"caron\0" as *const u8 as *const ::core::ffi::c_char,
    b"emdash\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"AE\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"OE\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"ae\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"lslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"oe\0" as *const u8 as *const ::core::ffi::c_char,
    b"germandbls\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
];
static mut ISOLatin1Encoding: [*const ::core::ffi::c_char; 256] = [
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"space\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclam\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"numbersign\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollar\0" as *const u8 as *const ::core::ffi::c_char,
    b"percent\0" as *const u8 as *const ::core::ffi::c_char,
    b"ampersand\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asterisk\0" as *const u8 as *const ::core::ffi::c_char,
    b"plus\0" as *const u8 as *const ::core::ffi::c_char,
    b"comma\0" as *const u8 as *const ::core::ffi::c_char,
    b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
    b"period\0" as *const u8 as *const ::core::ffi::c_char,
    b"slash\0" as *const u8 as *const ::core::ffi::c_char,
    b"zero\0" as *const u8 as *const ::core::ffi::c_char,
    b"one\0" as *const u8 as *const ::core::ffi::c_char,
    b"two\0" as *const u8 as *const ::core::ffi::c_char,
    b"three\0" as *const u8 as *const ::core::ffi::c_char,
    b"four\0" as *const u8 as *const ::core::ffi::c_char,
    b"five\0" as *const u8 as *const ::core::ffi::c_char,
    b"six\0" as *const u8 as *const ::core::ffi::c_char,
    b"seven\0" as *const u8 as *const ::core::ffi::c_char,
    b"eight\0" as *const u8 as *const ::core::ffi::c_char,
    b"nine\0" as *const u8 as *const ::core::ffi::c_char,
    b"colon\0" as *const u8 as *const ::core::ffi::c_char,
    b"semicolon\0" as *const u8 as *const ::core::ffi::c_char,
    b"less\0" as *const u8 as *const ::core::ffi::c_char,
    b"equal\0" as *const u8 as *const ::core::ffi::c_char,
    b"greater\0" as *const u8 as *const ::core::ffi::c_char,
    b"question\0" as *const u8 as *const ::core::ffi::c_char,
    b"at\0" as *const u8 as *const ::core::ffi::c_char,
    b"A\0" as *const u8 as *const ::core::ffi::c_char,
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"C\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"E\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"I\0" as *const u8 as *const ::core::ffi::c_char,
    b"J\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"O\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Q\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"U\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
    b"X\0" as *const u8 as *const ::core::ffi::c_char,
    b"Y\0" as *const u8 as *const ::core::ffi::c_char,
    b"Z\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"backslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char,
    b"underscore\0" as *const u8 as *const ::core::ffi::c_char,
    b"grave\0" as *const u8 as *const ::core::ffi::c_char,
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"b\0" as *const u8 as *const ::core::ffi::c_char,
    b"c\0" as *const u8 as *const ::core::ffi::c_char,
    b"d\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"f\0" as *const u8 as *const ::core::ffi::c_char,
    b"g\0" as *const u8 as *const ::core::ffi::c_char,
    b"h\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"j\0" as *const u8 as *const ::core::ffi::c_char,
    b"k\0" as *const u8 as *const ::core::ffi::c_char,
    b"l\0" as *const u8 as *const ::core::ffi::c_char,
    b"m\0" as *const u8 as *const ::core::ffi::c_char,
    b"n\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"p\0" as *const u8 as *const ::core::ffi::c_char,
    b"q\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\0" as *const u8 as *const ::core::ffi::c_char,
    b"s\0" as *const u8 as *const ::core::ffi::c_char,
    b"t\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"v\0" as *const u8 as *const ::core::ffi::c_char,
    b"w\0" as *const u8 as *const ::core::ffi::c_char,
    b"x\0" as *const u8 as *const ::core::ffi::c_char,
    b"y\0" as *const u8 as *const ::core::ffi::c_char,
    b"z\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"bar\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteright\0" as *const u8 as *const ::core::ffi::c_char,
    b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"macron\0" as *const u8 as *const ::core::ffi::c_char,
    b"breve\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"ring\0" as *const u8 as *const ::core::ffi::c_char,
    b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char,
    b"ogonek\0" as *const u8 as *const ::core::ffi::c_char,
    b"caron\0" as *const u8 as *const ::core::ffi::c_char,
    b"space\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char,
    b"cent\0" as *const u8 as *const ::core::ffi::c_char,
    b"sterling\0" as *const u8 as *const ::core::ffi::c_char,
    b"currency\0" as *const u8 as *const ::core::ffi::c_char,
    b"yen\0" as *const u8 as *const ::core::ffi::c_char,
    b"brokenbar\0" as *const u8 as *const ::core::ffi::c_char,
    b"section\0" as *const u8 as *const ::core::ffi::c_char,
    b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"logicalnot\0" as *const u8 as *const ::core::ffi::c_char,
    b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
    b"registered\0" as *const u8 as *const ::core::ffi::c_char,
    b"macron\0" as *const u8 as *const ::core::ffi::c_char,
    b"degree\0" as *const u8 as *const ::core::ffi::c_char,
    b"plusminus\0" as *const u8 as *const ::core::ffi::c_char,
    b"twosuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"threesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"acute\0" as *const u8 as *const ::core::ffi::c_char,
    b"mu\0" as *const u8 as *const ::core::ffi::c_char,
    b"paragraph\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
    b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"onesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char,
    b"onequarter\0" as *const u8 as *const ::core::ffi::c_char,
    b"onehalf\0" as *const u8 as *const ::core::ffi::c_char,
    b"threequarters\0" as *const u8 as *const ::core::ffi::c_char,
    b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
    b"Agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"AE\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"Egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"multiply\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"germandbls\0" as *const u8 as *const ::core::ffi::c_char,
    b"agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"ae\0" as *const u8 as *const ::core::ffi::c_char,
    b"ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"divide\0" as *const u8 as *const ::core::ffi::c_char,
    b"oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C-unwind" fn try_put_or_putinterval(
    mut enc_vec: *mut *mut ::core::ffi::c_char,
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut i: ::core::ffi::c_int = 0;
    let mut num1: ::core::ffi::c_int = 0;
    let mut num2: ::core::ffi::c_int = 0;
    let mut num3: ::core::ffi::c_int = 0;
    tok = pst_get_token(start, end);
    if tok.is_null()
        || !(pst_type_of(tok) == PST_TYPE_INTEGER)
        || {
            num1 = pst_getIV(tok);
            num1 > 255 as ::core::ffi::c_int
        }
        || num1 < 0 as ::core::ffi::c_int
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        return -(1 as ::core::ffi::c_int);
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = ::core::ptr::null_mut::<pst_obj>();
    }
    tok = pst_get_token(start, end);
    if tok.is_null() {
        return -(1 as ::core::ffi::c_int);
    } else if !tok.is_null()
        && pst_type_of(tok) < 0 as ::core::ffi::c_int
        && pst_length_of(tok) as size_t
            == strlen(b"exch\0" as *const u8 as *const ::core::ffi::c_char)
        && !strstartswith(
            pst_data_ptr(tok) as *const ::core::ffi::c_char,
            b"exch\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        tok = pst_get_token(start, end);
        if tok.is_null()
            || !(pst_type_of(tok) == PST_TYPE_INTEGER)
            || {
                num2 = pst_getIV(tok);
                num2 > 255 as ::core::ffi::c_int
            }
            || num2 < 0 as ::core::ffi::c_int
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            return -(1 as ::core::ffi::c_int);
        }
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        tok = pst_get_token(start, end);
        if !(!tok.is_null()
            && pst_type_of(tok) < 0 as ::core::ffi::c_int
            && pst_length_of(tok) as size_t
                == strlen(b"get\0" as *const u8 as *const ::core::ffi::c_char)
            && !strstartswith(
                pst_data_ptr(tok) as *const ::core::ffi::c_char,
                b"get\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null())
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            return -(1 as ::core::ffi::c_int);
        }
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        tok = pst_get_token(start, end);
        if !(!tok.is_null()
            && pst_type_of(tok) < 0 as ::core::ffi::c_int
            && pst_length_of(tok) as size_t
                == strlen(b"put\0" as *const u8 as *const ::core::ffi::c_char)
            && !strstartswith(
                pst_data_ptr(tok) as *const ::core::ffi::c_char,
                b"put\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null())
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            return -(1 as ::core::ffi::c_int);
        }
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        free(*enc_vec.offset(num1 as isize) as *mut ::core::ffi::c_void);
        let ref mut fresh20 = *enc_vec.offset(num1 as isize);
        *fresh20 = crate::backend::runtime::duplicate(*enc_vec.offset(num2 as isize));
    } else if pst_type_of(tok) == PST_TYPE_INTEGER
        && {
            num2 = pst_getIV(tok);
            num2 + num1 <= 255 as ::core::ffi::c_int
        }
        && num2 >= 0 as ::core::ffi::c_int
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        tok = pst_get_token(start, end);
        if !(!tok.is_null()
            && pst_type_of(tok) < 0 as ::core::ffi::c_int
            && pst_length_of(tok) as size_t
                == strlen(b"getinterval\0" as *const u8 as *const ::core::ffi::c_char)
            && !strstartswith(
                pst_data_ptr(tok) as *const ::core::ffi::c_char,
                b"getinterval\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null())
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            return -(1 as ::core::ffi::c_int);
        }
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        tok = pst_get_token(start, end);
        if tok.is_null()
            || !(pst_type_of(tok) == PST_TYPE_INTEGER)
            || {
                num3 = pst_getIV(tok);
                num3 + num2 > 255 as ::core::ffi::c_int
            }
            || num3 < 0 as ::core::ffi::c_int
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            return -(1 as ::core::ffi::c_int);
        }
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        tok = pst_get_token(start, end);
        if !(!tok.is_null()
            && pst_type_of(tok) < 0 as ::core::ffi::c_int
            && pst_length_of(tok) as size_t
                == strlen(b"exch\0" as *const u8 as *const ::core::ffi::c_char)
            && !strstartswith(
                pst_data_ptr(tok) as *const ::core::ffi::c_char,
                b"exch\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null())
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            return -(1 as ::core::ffi::c_int);
        }
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        tok = pst_get_token(start, end);
        if !(!tok.is_null()
            && pst_type_of(tok) < 0 as ::core::ffi::c_int
            && pst_length_of(tok) as size_t
                == strlen(b"putinterval\0" as *const u8 as *const ::core::ffi::c_char)
            && !strstartswith(
                pst_data_ptr(tok) as *const ::core::ffi::c_char,
                b"putinterval\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null())
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            return -(1 as ::core::ffi::c_int);
        }
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        i = 0 as ::core::ffi::c_int;
        while i < num2 {
            if !(*enc_vec.offset((num1 + i) as isize)).is_null() {
                let ref mut fresh21 = *enc_vec.offset((num3 + i) as isize);
                *fresh21 = mfree(*enc_vec.offset((num3 + i) as isize) as *mut ::core::ffi::c_void)
                    as *mut ::core::ffi::c_char;
                let ref mut fresh22 = *enc_vec.offset((num3 + i) as isize);
                *fresh22 = crate::backend::runtime::duplicate(*enc_vec.offset((num1 + i) as isize));
            }
            i += 1;
        }
    } else {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn parse_encoding(
    mut enc_vec: *mut *mut ::core::ffi::c_char,
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut code: ::core::ffi::c_int = 0;
    tok = pst_get_token(start, end);
    if !tok.is_null()
        && pst_type_of(tok) < 0 as ::core::ffi::c_int
        && pst_length_of(tok) as size_t
            == strlen(b"StandardEncoding\0" as *const u8 as *const ::core::ffi::c_char)
        && !strstartswith(
            pst_data_ptr(tok) as *const ::core::ffi::c_char,
            b"StandardEncoding\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        if !enc_vec.is_null() {
            code = 0 as ::core::ffi::c_int;
            while code < 256 as ::core::ffi::c_int {
                if !StandardEncoding[code as usize].is_null()
                    && strcmp(
                        StandardEncoding[code as usize],
                        b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                {
                    let ref mut fresh14 = *enc_vec.offset(code as isize);
                    *fresh14 = new((strlen(StandardEncoding[code as usize])
                        .wrapping_add(1 as size_t) as uint32_t
                        as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                        as uint32_t) as *mut ::core::ffi::c_char;
                    strcpy(
                        *enc_vec.offset(code as isize),
                        StandardEncoding[code as usize],
                    );
                } else {
                    let ref mut fresh15 = *enc_vec.offset(code as isize);
                    *fresh15 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                code += 1;
            }
        }
    } else if !tok.is_null()
        && pst_type_of(tok) < 0 as ::core::ffi::c_int
        && pst_length_of(tok) as size_t
            == strlen(b"ISOLatin1Encoding\0" as *const u8 as *const ::core::ffi::c_char)
        && !strstartswith(
            pst_data_ptr(tok) as *const ::core::ffi::c_char,
            b"ISOLatin1Encoding\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        if !enc_vec.is_null() {
            code = 0 as ::core::ffi::c_int;
            while code < 256 as ::core::ffi::c_int {
                if !ISOLatin1Encoding[code as usize].is_null()
                    && strcmp(
                        ISOLatin1Encoding[code as usize],
                        b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
                    ) != 0 as ::core::ffi::c_int
                {
                    let ref mut fresh16 = *enc_vec.offset(code as isize);
                    *fresh16 = new((strlen(ISOLatin1Encoding[code as usize])
                        .wrapping_add(1 as size_t) as uint32_t
                        as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                        as uint32_t) as *mut ::core::ffi::c_char;
                    strcpy(
                        *enc_vec.offset(code as isize),
                        ISOLatin1Encoding[code as usize],
                    );
                } else {
                    let ref mut fresh17 = *enc_vec.offset(code as isize);
                    *fresh17 = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                code += 1;
            }
        }
    } else if !tok.is_null()
        && pst_type_of(tok) < 0 as ::core::ffi::c_int
        && pst_length_of(tok) as size_t
            == strlen(b"ExpertEncoding\0" as *const u8 as *const ::core::ffi::c_char)
        && !strstartswith(
            pst_data_ptr(tok) as *const ::core::ffi::c_char,
            b"ExpertEncoding\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        if !enc_vec.is_null() {
            pdf_warning!(
                b"ExpertEncoding not supported.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        seek_operator(
            start,
            end,
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        );
        while *start < end && {
            tok = pst_get_token(start, end);
            !tok.is_null()
        } {
            if !tok.is_null()
                && pst_type_of(tok) < 0 as ::core::ffi::c_int
                && pst_length_of(tok) as size_t
                    == strlen(b"def\0" as *const u8 as *const ::core::ffi::c_char)
                && !strstartswith(
                    pst_data_ptr(tok) as *const ::core::ffi::c_char,
                    b"def\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null()
                || !tok.is_null()
                    && pst_type_of(tok) < 0 as ::core::ffi::c_int
                    && pst_length_of(tok) as size_t
                        == strlen(b"readonly\0" as *const u8 as *const ::core::ffi::c_char)
                    && !strstartswith(
                        pst_data_ptr(tok) as *const ::core::ffi::c_char,
                        b"readonly\0" as *const u8 as *const ::core::ffi::c_char,
                    )
                    .is_null()
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = ::core::ptr::null_mut::<pst_obj>();
                }
                break;
            } else if !(!tok.is_null()
                && pst_type_of(tok) < 0 as ::core::ffi::c_int
                && pst_length_of(tok) as size_t
                    == strlen(b"dup\0" as *const u8 as *const ::core::ffi::c_char)
                && !strstartswith(
                    pst_data_ptr(tok) as *const ::core::ffi::c_char,
                    b"dup\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null())
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = ::core::ptr::null_mut::<pst_obj>();
                }
            } else {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = ::core::ptr::null_mut::<pst_obj>();
                }
                tok = pst_get_token(start, end);
                if !tok.is_null()
                    && pst_type_of(tok) < 0 as ::core::ffi::c_int
                    && pst_length_of(tok) as size_t
                        == strlen(b"dup\0" as *const u8 as *const ::core::ffi::c_char)
                    && !strstartswith(
                        pst_data_ptr(tok) as *const ::core::ffi::c_char,
                        b"dup\0" as *const u8 as *const ::core::ffi::c_char,
                    )
                    .is_null()
                {
                    if enc_vec.is_null() {
                        pdf_warning!(
                            b"This kind of type1 fonts are not supported as native fonts.\n                   They are supported if used with tfm fonts.\n\0"
                            as * const u8 as * const ::core::ffi::c_char,
                        );
                    } else {
                        try_put_or_putinterval(enc_vec, start, end);
                    }
                    if !tok.is_null() {
                        pst_release_obj(tok);
                        tok = ::core::ptr::null_mut::<pst_obj>();
                    }
                } else if tok.is_null()
                    || !(pst_type_of(tok) == PST_TYPE_INTEGER)
                    || {
                        code = pst_getIV(tok);
                        code > 255 as ::core::ffi::c_int
                    }
                    || code < 0 as ::core::ffi::c_int
                {
                    if !tok.is_null() {
                        pst_release_obj(tok);
                        tok = ::core::ptr::null_mut::<pst_obj>();
                    }
                } else {
                    if !tok.is_null() {
                        pst_release_obj(tok);
                        tok = ::core::ptr::null_mut::<pst_obj>();
                    }
                    tok = pst_get_token(start, end);
                    if tok.is_null() || !(pst_type_of(tok) == PST_TYPE_NAME) {
                        if !tok.is_null() {
                            pst_release_obj(tok);
                            tok = ::core::ptr::null_mut::<pst_obj>();
                        }
                    } else {
                        if !enc_vec.is_null() {
                            free(*enc_vec.offset(code as isize) as *mut ::core::ffi::c_void);
                            let ref mut fresh18 = *enc_vec.offset(code as isize);
                            *fresh18 = pst_getSV(tok) as *mut ::core::ffi::c_char;
                        }
                        if !tok.is_null() {
                            pst_release_obj(tok);
                            tok = ::core::ptr::null_mut::<pst_obj>();
                        }
                        tok = pst_get_token(start, end);
                        if !(!tok.is_null()
                            && pst_type_of(tok) < 0 as ::core::ffi::c_int
                            && pst_length_of(tok) as size_t
                                == strlen(b"put\0" as *const u8 as *const ::core::ffi::c_char)
                            && !strstartswith(
                                pst_data_ptr(tok) as *const ::core::ffi::c_char,
                                b"put\0" as *const u8 as *const ::core::ffi::c_char,
                            )
                            .is_null())
                        {
                            let ref mut fresh19 = *enc_vec.offset(code as isize);
                            *fresh19 =
                                mfree(*enc_vec.offset(code as isize) as *mut ::core::ffi::c_void)
                                    as *mut ::core::ffi::c_char;
                            if !tok.is_null() {
                                pst_release_obj(tok);
                                tok = ::core::ptr::null_mut::<pst_obj>();
                            }
                        } else if !tok.is_null() {
                            pst_release_obj(tok);
                            tok = ::core::ptr::null_mut::<pst_obj>();
                        }
                    }
                }
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
pub const CFF_GLYPH_MAX: ::core::ffi::c_int = CFF_SID_MAX;
unsafe extern "C-unwind" fn parse_subrs(
    mut font: *mut cff_font,
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut lenIV: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut subrs: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut i: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0;
    let mut max_size: ::core::ffi::c_int = 0;
    let mut offsets: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut lengths: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut data: *mut card8 = ::core::ptr::null_mut::<card8>();
    tok = pst_get_token(start, end);
    if !(pst_type_of(tok) == PST_TYPE_INTEGER) || pst_getIV(tok) < 0 as ::core::ffi::c_int {
        pdf_warning!(b"Parsing Subrs failed.\0" as *const u8 as *const ::core::ffi::c_char,);
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        return -(1 as ::core::ffi::c_int);
    }
    count = pst_getIV(tok);
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = ::core::ptr::null_mut::<pst_obj>();
    }
    if count == 0 as ::core::ffi::c_int {
        let ref mut fresh9 = *(*font).subrs.offset(0 as ::core::ffi::c_int as isize);
        *fresh9 = ::core::ptr::null_mut::<cff_index>();
        return 0 as ::core::ffi::c_int;
    }
    tok = pst_get_token(start, end);
    if !(!tok.is_null()
        && pst_type_of(tok) < 0 as ::core::ffi::c_int
        && pst_length_of(tok) as size_t
            == strlen(b"array\0" as *const u8 as *const ::core::ffi::c_char)
        && !strstartswith(
            pst_data_ptr(tok) as *const ::core::ffi::c_char,
            b"array\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null())
    {
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        return -(1 as ::core::ffi::c_int);
    }
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = ::core::ptr::null_mut::<pst_obj>();
    }
    if mode != 1 as ::core::ffi::c_int {
        max_size = CS_STR_LEN_MAX;
        data = new((max_size as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t)
            as *mut card8;
        offsets = new((count as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
            as uint32_t) as *mut ::core::ffi::c_int;
        lengths = new((count as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
            as uint32_t) as *mut ::core::ffi::c_int;
        memset(
            offsets as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<::core::ffi::c_int>() as size_t).wrapping_mul(count as size_t),
        );
        memset(
            lengths as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<::core::ffi::c_int>() as size_t).wrapping_mul(count as size_t),
        );
    } else {
        max_size = 0 as ::core::ffi::c_int;
        data = ::core::ptr::null_mut::<card8>();
        offsets = ::core::ptr::null_mut::<::core::ffi::c_int>();
        lengths = ::core::ptr::null_mut::<::core::ffi::c_int>();
    }
    offset = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < count {
        let mut idx: ::core::ffi::c_int = 0;
        let mut len: ::core::ffi::c_int = 0;
        tok = pst_get_token(start, end);
        if tok.is_null() {
            free(data as *mut ::core::ffi::c_void);
            free(offsets as *mut ::core::ffi::c_void);
            free(lengths as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        } else if !tok.is_null()
            && pst_type_of(tok) < 0 as ::core::ffi::c_int
            && pst_length_of(tok) as size_t
                == strlen(b"ND\0" as *const u8 as *const ::core::ffi::c_char)
            && !strstartswith(
                pst_data_ptr(tok) as *const ::core::ffi::c_char,
                b"ND\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
            || !tok.is_null()
                && pst_type_of(tok) < 0 as ::core::ffi::c_int
                && pst_length_of(tok) as size_t
                    == strlen(b"|-\0" as *const u8 as *const ::core::ffi::c_char)
                && !strstartswith(
                    pst_data_ptr(tok) as *const ::core::ffi::c_char,
                    b"|-\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null()
            || !tok.is_null()
                && pst_type_of(tok) < 0 as ::core::ffi::c_int
                && pst_length_of(tok) as size_t
                    == strlen(b"def\0" as *const u8 as *const ::core::ffi::c_char)
                && !strstartswith(
                    pst_data_ptr(tok) as *const ::core::ffi::c_char,
                    b"def\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null()
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            break;
        } else if !(!tok.is_null()
            && pst_type_of(tok) < 0 as ::core::ffi::c_int
            && pst_length_of(tok) as size_t
                == strlen(b"dup\0" as *const u8 as *const ::core::ffi::c_char)
            && !strstartswith(
                pst_data_ptr(tok) as *const ::core::ffi::c_char,
                b"dup\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null())
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
        } else {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            tok = pst_get_token(start, end);
            if !(pst_type_of(tok) == PST_TYPE_INTEGER)
                || pst_getIV(tok) < 0 as ::core::ffi::c_int
                || pst_getIV(tok) >= count
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = ::core::ptr::null_mut::<pst_obj>();
                }
                free(data as *mut ::core::ffi::c_void);
                free(offsets as *mut ::core::ffi::c_void);
                free(lengths as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            idx = pst_getIV(tok);
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            tok = pst_get_token(start, end);
            if !(pst_type_of(tok) == PST_TYPE_INTEGER)
                || pst_getIV(tok) < 0 as ::core::ffi::c_int
                || pst_getIV(tok) > CS_STR_LEN_MAX
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = ::core::ptr::null_mut::<pst_obj>();
                }
                return -(1 as ::core::ffi::c_int);
            }
            len = pst_getIV(tok);
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            tok = pst_get_token(start, end);
            if !(!tok.is_null()
                && pst_type_of(tok) < 0 as ::core::ffi::c_int
                && pst_length_of(tok) as size_t
                    == strlen(b"RD\0" as *const u8 as *const ::core::ffi::c_char)
                && !strstartswith(
                    pst_data_ptr(tok) as *const ::core::ffi::c_char,
                    b"RD\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null())
                && !(!tok.is_null()
                    && pst_type_of(tok) < 0 as ::core::ffi::c_int
                    && pst_length_of(tok) as size_t
                        == strlen(b"-|\0" as *const u8 as *const ::core::ffi::c_char)
                    && !strstartswith(
                        pst_data_ptr(tok) as *const ::core::ffi::c_char,
                        b"-|\0" as *const u8 as *const ::core::ffi::c_char,
                    )
                    .is_null())
                && seek_operator(
                    start,
                    end,
                    b"readstring\0" as *const u8 as *const ::core::ffi::c_char,
                ) < 0 as ::core::ffi::c_int
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = ::core::ptr::null_mut::<pst_obj>();
                }
                free(data as *mut ::core::ffi::c_void);
                free(offsets as *mut ::core::ffi::c_void);
                free(lengths as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            *start = (*start).offset(1 as ::core::ffi::c_int as isize);
            if (*start).offset(len as isize) >= end {
                free(data as *mut ::core::ffi::c_void);
                free(offsets as *mut ::core::ffi::c_void);
                free(lengths as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            if mode != 1 as ::core::ffi::c_int {
                if offset + len >= max_size {
                    max_size += CS_STR_LEN_MAX;
                    data = renew(
                        data as *mut ::core::ffi::c_void,
                        (max_size as uint32_t as usize)
                            .wrapping_mul(::core::mem::size_of::<card8>() as usize)
                            as uint32_t,
                    ) as *mut card8;
                }
                if lenIV >= 0 as ::core::ffi::c_int {
                    t1_decrypt(
                        T1_CHARKEY as ::core::ffi::c_ushort,
                        data.offset(offset as isize),
                        *start,
                        lenIV,
                        len,
                    );
                    *offsets.offset(idx as isize) = offset;
                    let ref mut fresh10 = *lengths.offset(idx as isize);
                    *fresh10 = len - lenIV;
                    offset += *fresh10;
                } else if len > 0 as ::core::ffi::c_int {
                    *offsets.offset(idx as isize) = offset;
                    *lengths.offset(idx as isize) = len;
                    memcpy(
                        data.offset(offset as isize) as *mut card8 as *mut ::core::ffi::c_void,
                        *start as *const ::core::ffi::c_void,
                        len as size_t,
                    );
                    offset += len;
                }
            }
            *start = (*start).offset(len as isize);
            i += 1;
        }
    }
    if mode != 1 as ::core::ffi::c_int {
        if (*(*font).subrs.offset(0 as ::core::ffi::c_int as isize)).is_null() {
            let ref mut fresh11 = *(*font).subrs.offset(0 as ::core::ffi::c_int as isize);
            *fresh11 = cff_new_index(count as card16);
            subrs = *fresh11;
            (*subrs).data = new((offset as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<card8>() as usize)
                as uint32_t) as *mut card8;
            offset = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < count {
                *(*subrs).offset.offset(i as isize) =
                    (offset + 1 as ::core::ffi::c_int) as l_offset;
                if *lengths.offset(i as isize) > 0 as ::core::ffi::c_int {
                    memcpy(
                        (*subrs).data.offset(offset as isize) as *mut ::core::ffi::c_void,
                        data.offset(*offsets.offset(i as isize) as isize)
                            as *const ::core::ffi::c_void,
                        *lengths.offset(i as isize) as size_t,
                    );
                    offset += *lengths.offset(i as isize);
                }
                i += 1;
            }
            *(*subrs).offset.offset(count as isize) =
                (offset + 1 as ::core::ffi::c_int) as l_offset;
        } else {
            pdf_warning!(
                b"Already found /Subrs; ignores the other /Subrs dicts.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        free(data as *mut ::core::ffi::c_void);
        free(offsets as *mut ::core::ffi::c_void);
        free(lengths as *mut ::core::ffi::c_void);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn parse_charstrings(
    mut font: *mut cff_font,
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut lenIV: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut charstrings: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut charset: *mut cff_charsets = ::core::ptr::null_mut::<cff_charsets>();
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut i: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut have_notdef: ::core::ffi::c_int = 0;
    let mut max_size: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0;
    tok = pst_get_token(start, end);
    if !(pst_type_of(tok) == PST_TYPE_INTEGER)
        || pst_getIV(tok) < 0 as ::core::ffi::c_int
        || pst_getIV(tok) > CFF_GLYPH_MAX
    {
        let mut s: *mut ::core::ffi::c_uchar = pst_getSV(tok);
        pdf_warning!(
            b"Ignores non dict \"/CharStrings %s ...\"\0" as *const u8
                as *const ::core::ffi::c_char,
            s,
        );
        free(s as *mut ::core::ffi::c_void);
        if !tok.is_null() {
            pst_release_obj(tok);
            tok = ::core::ptr::null_mut::<pst_obj>();
        }
        return 0 as ::core::ffi::c_int;
    }
    count = pst_getIV(tok);
    if !tok.is_null() {
        pst_release_obj(tok);
        tok = ::core::ptr::null_mut::<pst_obj>();
    }
    if mode != 1 as ::core::ffi::c_int {
        charstrings = cff_new_index(count as card16);
        max_size = CS_STR_LEN_MAX;
        (*charstrings).data = new((max_size as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<card8>() as usize)
            as uint32_t) as *mut card8;
    } else {
        charstrings = ::core::ptr::null_mut::<cff_index>();
        max_size = 0 as ::core::ffi::c_int;
    }
    (*font).cstrings = charstrings;
    (*font).charsets = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_charsets>() as usize)
        as uint32_t) as *mut cff_charsets;
    charset = (*font).charsets;
    (*charset).format = 0 as card8;
    (*charset).num_entries = (count - 1 as ::core::ffi::c_int) as card16;
    (*charset).data.glyphs = new(((count - 1 as ::core::ffi::c_int) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<s_SID>() as usize)
        as uint32_t) as *mut s_SID;
    memset(
        (*charset).data.glyphs as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<s_SID>() as size_t)
            .wrapping_mul((count - 1 as ::core::ffi::c_int) as size_t),
    );
    offset = 0 as ::core::ffi::c_int;
    have_notdef = 0 as ::core::ffi::c_int;
    (*font).is_notdef_notzero = 0 as ::core::ffi::c_int;
    seek_operator(
        start,
        end,
        b"begin\0" as *const u8 as *const ::core::ffi::c_char,
    );
    i = 0 as ::core::ffi::c_int;
    while i < count {
        let mut glyph_name: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut len: ::core::ffi::c_int = 0;
        let mut gid: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        tok = pst_get_token(start, end);
        glyph_name = pst_getSV(tok) as *mut ::core::ffi::c_char;
        if i == 0 as ::core::ffi::c_int
            && !glyph_name.is_null()
            && strcmp(
                glyph_name,
                b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
        {
            (*font).is_notdef_notzero = 1 as ::core::ffi::c_int;
        }
        if pst_type_of(tok) == PST_TYPE_NAME {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            if glyph_name.is_null() {
                return -(1 as ::core::ffi::c_int);
            } else if streq_ptr(
                glyph_name,
                b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
            ) {
                gid = 0 as ::core::ffi::c_int;
                have_notdef = 1 as ::core::ffi::c_int;
            } else if have_notdef != 0 {
                gid = i;
            } else if i == count - 1 as ::core::ffi::c_int {
                pdf_warning!(b"No .notdef glyph???\0" as *const u8 as *const ::core::ffi::c_char,);
                return -(1 as ::core::ffi::c_int);
            } else {
                gid = i + 1 as ::core::ffi::c_int;
            }
            if gid > 0 as ::core::ffi::c_int {
                *(*charset)
                    .data
                    .glyphs
                    .offset((gid - 1 as ::core::ffi::c_int) as isize) =
                    cff_add_string(font, glyph_name, 0 as ::core::ffi::c_int);
            }
            free(glyph_name as *mut ::core::ffi::c_void);
            tok = pst_get_token(start, end);
            if !(pst_type_of(tok) == PST_TYPE_INTEGER)
                || pst_getIV(tok) < 0 as ::core::ffi::c_int
                || pst_getIV(tok) > CS_STR_LEN_MAX
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = ::core::ptr::null_mut::<pst_obj>();
                }
                return -(1 as ::core::ffi::c_int);
            }
            len = pst_getIV(tok);
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            tok = pst_get_token(start, end);
            if !(!tok.is_null()
                && pst_type_of(tok) < 0 as ::core::ffi::c_int
                && pst_length_of(tok) as size_t
                    == strlen(b"RD\0" as *const u8 as *const ::core::ffi::c_char)
                && !strstartswith(
                    pst_data_ptr(tok) as *const ::core::ffi::c_char,
                    b"RD\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null())
                && !(!tok.is_null()
                    && pst_type_of(tok) < 0 as ::core::ffi::c_int
                    && pst_length_of(tok) as size_t
                        == strlen(b"-|\0" as *const u8 as *const ::core::ffi::c_char)
                    && !strstartswith(
                        pst_data_ptr(tok) as *const ::core::ffi::c_char,
                        b"-|\0" as *const u8 as *const ::core::ffi::c_char,
                    )
                    .is_null())
                && seek_operator(
                    start,
                    end,
                    b"readstring\0" as *const u8 as *const ::core::ffi::c_char,
                ) < 0 as ::core::ffi::c_int
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = ::core::ptr::null_mut::<pst_obj>();
                }
                return -(1 as ::core::ffi::c_int);
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            if (*start)
                .offset(len as isize)
                .offset(1 as ::core::ffi::c_int as isize)
                >= end
            {
                return -(1 as ::core::ffi::c_int);
            }
            if mode != 1 as ::core::ffi::c_int {
                if offset + len >= max_size {
                    max_size += if len > 65536 as ::core::ffi::c_int {
                        len
                    } else {
                        65536 as ::core::ffi::c_int
                    };
                    (*charstrings).data = renew(
                        (*charstrings).data as *mut ::core::ffi::c_void,
                        (max_size as uint32_t as usize)
                            .wrapping_mul(::core::mem::size_of::<card8>() as usize)
                            as uint32_t,
                    ) as *mut card8;
                }
                if gid == 0 as ::core::ffi::c_int {
                    if lenIV >= 0 as ::core::ffi::c_int {
                        memmove(
                            (*charstrings)
                                .data
                                .offset(len as isize)
                                .offset(-(lenIV as isize))
                                as *mut ::core::ffi::c_void,
                            (*charstrings).data as *const ::core::ffi::c_void,
                            offset as size_t,
                        );
                        j = 1 as ::core::ffi::c_int;
                        while j <= i {
                            let ref mut fresh2 = *(*charstrings).offset.offset(j as isize);
                            *fresh2 = (*fresh2).wrapping_add((len - lenIV) as l_offset);
                            j += 1;
                        }
                    } else {
                        memmove(
                            (*charstrings).data.offset(len as isize) as *mut ::core::ffi::c_void,
                            (*charstrings).data as *const ::core::ffi::c_void,
                            offset as size_t,
                        );
                        j = 1 as ::core::ffi::c_int;
                        while j <= i {
                            let ref mut fresh3 = *(*charstrings).offset.offset(j as isize);
                            *fresh3 = (*fresh3).wrapping_add(len as l_offset);
                            j += 1;
                        }
                    }
                }
            }
            *start = (*start).offset(1 as ::core::ffi::c_int as isize);
            if mode != 1 as ::core::ffi::c_int {
                if lenIV >= 0 as ::core::ffi::c_int {
                    let mut offs: ::core::ffi::c_int = if gid != 0 {
                        offset
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    *(*charstrings).offset.offset(gid as isize) =
                        (offs + 1 as ::core::ffi::c_int) as l_offset;
                    t1_decrypt(
                        T1_CHARKEY as ::core::ffi::c_ushort,
                        (*charstrings).data.offset(offs as isize),
                        *start,
                        lenIV,
                        len,
                    );
                    offset += len - lenIV;
                } else {
                    if gid == 0 as ::core::ffi::c_int {
                        *(*charstrings).offset.offset(gid as isize) = 1 as l_offset;
                        memcpy(
                            (*charstrings).data.offset(0 as ::core::ffi::c_int as isize)
                                as *mut card8
                                as *mut ::core::ffi::c_void,
                            *start as *const ::core::ffi::c_void,
                            len as size_t,
                        );
                    } else {
                        *(*charstrings).offset.offset(gid as isize) =
                            (offset + 1 as ::core::ffi::c_int) as l_offset;
                        memcpy(
                            (*charstrings).data.offset(offset as isize) as *mut card8
                                as *mut ::core::ffi::c_void,
                            *start as *const ::core::ffi::c_void,
                            len as size_t,
                        );
                    }
                    offset += len;
                }
            }
            *start = (*start).offset(len as isize);
            tok = pst_get_token(start, end);
            if !(!tok.is_null()
                && pst_type_of(tok) < 0 as ::core::ffi::c_int
                && pst_length_of(tok) as size_t
                    == strlen(b"ND\0" as *const u8 as *const ::core::ffi::c_char)
                && !strstartswith(
                    pst_data_ptr(tok) as *const ::core::ffi::c_char,
                    b"ND\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null())
                && !(!tok.is_null()
                    && pst_type_of(tok) < 0 as ::core::ffi::c_int
                    && pst_length_of(tok) as size_t
                        == strlen(b"|-\0" as *const u8 as *const ::core::ffi::c_char)
                    && !strstartswith(
                        pst_data_ptr(tok) as *const ::core::ffi::c_char,
                        b"|-\0" as *const u8 as *const ::core::ffi::c_char,
                    )
                    .is_null())
            {
                if !tok.is_null() {
                    pst_release_obj(tok);
                    tok = ::core::ptr::null_mut::<pst_obj>();
                }
                return -(1 as ::core::ffi::c_int);
            }
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            i += 1;
        } else if pst_type_of(tok) < 0 as ::core::ffi::c_int
            && streq_ptr(
                glyph_name,
                b"end\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            break;
        } else {
            if !tok.is_null() {
                pst_release_obj(tok);
                tok = ::core::ptr::null_mut::<pst_obj>();
            }
            return -(1 as ::core::ffi::c_int);
        }
    }
    if mode != 1 as ::core::ffi::c_int {
        *(*charstrings).offset.offset(count as isize) =
            (offset + 1 as ::core::ffi::c_int) as l_offset;
    }
    (*font).num_glyphs = count as card16;
    return 0 as ::core::ffi::c_int;
}
pub const MAX_ARGS: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn parse_part2(
    mut font: *mut cff_font,
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut argv: [::core::ffi::c_double; 127] = [0.; 127];
    let mut argn: ::core::ffi::c_int = 0;
    let mut lenIV: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    while *start < end && {
        key = get_next_key(start, end);
        !key.is_null()
    } {
        if streq_ptr(key, b"Subrs\0" as *const u8 as *const ::core::ffi::c_char) {
            if parse_subrs(font, start, end, lenIV, mode) < 0 as ::core::ffi::c_int {
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
        } else if streq_ptr(
            key,
            b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            if parse_charstrings(font, start, end, lenIV, mode) < 0 as ::core::ffi::c_int {
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
        } else if streq_ptr(key, b"lenIV\0" as *const u8 as *const ::core::ffi::c_char) {
            argn = parse_nvalue(
                start,
                end,
                &raw mut argv as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if argn != 1 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            lenIV = argv[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        } else if streq_ptr(
            key,
            b"BlueValues\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            || streq_ptr(
                key,
                b"OtherBlues\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"FamilyBlues\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"FamilyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"StemSnapH\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"StemSnapV\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            argn = parse_nvalue(
                start,
                end,
                &raw mut argv as *mut ::core::ffi::c_double,
                MAX_ARGS,
            );
            if argn < 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            cff_dict_add(
                *(*font).private.offset(0 as ::core::ffi::c_int as isize),
                key,
                argn,
            );
            loop {
                let fresh0 = argn;
                argn = argn - 1;
                if !(fresh0 > 0 as ::core::ffi::c_int) {
                    break;
                }
                cff_dict_set(
                    *(*font).private.offset(0 as ::core::ffi::c_int as isize),
                    key,
                    argn,
                    if argn == 0 as ::core::ffi::c_int {
                        argv[argn as usize]
                    } else {
                        argv[argn as usize] - argv[(argn - 1 as ::core::ffi::c_int) as usize]
                    },
                );
            }
        } else if streq_ptr(key, b"StdHW\0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
            || streq_ptr(key, b"StdVW\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"BlueScale\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"BlueShift\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"BlueFuzz\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"LanguageGroup\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"ExpansionFactor\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            argn = parse_nvalue(
                start,
                end,
                &raw mut argv as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if argn != 1 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            cff_dict_add(
                *(*font).private.offset(0 as ::core::ffi::c_int as isize),
                key,
                1 as ::core::ffi::c_int,
            );
            cff_dict_set(
                *(*font).private.offset(0 as ::core::ffi::c_int as isize),
                key,
                0 as ::core::ffi::c_int,
                argv[0 as ::core::ffi::c_int as usize],
            );
        } else if streq_ptr(
            key,
            b"ForceBold\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            argn = parse_bvalue(
                start,
                end,
                (&raw mut argv as *mut ::core::ffi::c_double)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_double,
            );
            if argn != 1 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            if argv[0 as ::core::ffi::c_int as usize]
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                cff_dict_add(
                    *(*font).private.offset(0 as ::core::ffi::c_int as isize),
                    key,
                    1 as ::core::ffi::c_int,
                );
                cff_dict_set(
                    *(*font).private.offset(0 as ::core::ffi::c_int as isize),
                    key,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int as ::core::ffi::c_double,
                );
            }
        }
        free(key as *mut ::core::ffi::c_void);
    }
    return 0 as ::core::ffi::c_int;
}
pub const TYPE1_NAME_LEN_MAX: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn parse_part1(
    mut font: *mut cff_font,
    mut enc_vec: *mut *mut ::core::ffi::c_char,
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut strval: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut argv: [::core::ffi::c_double; 127] = [0.; 127];
    let mut argn: ::core::ffi::c_int = 0;
    if seek_operator(
        start,
        end,
        b"begin\0" as *const u8 as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    while *start < end && {
        key = get_next_key(start, end);
        !key.is_null()
    } {
        if streq_ptr(
            key,
            b"Encoding\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            if parse_encoding(enc_vec, start, end) < 0 as ::core::ffi::c_int {
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
        } else if streq_ptr(
            key,
            b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            argn = parse_svalue(start, end, &raw mut strval);
            if argn != 1 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            if strlen(strval) > TYPE1_NAME_LEN_MAX as size_t {
                pdf_warning!(
                    b"FontName too long: %s (%zu bytes)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    strval,
                    strlen(strval),
                );
                *strval.offset(TYPE1_NAME_LEN_MAX as isize) = '\0' as i32 as ::core::ffi::c_char;
            }
            cff_set_name(font, strval);
            free(strval as *mut ::core::ffi::c_void);
        } else if streq_ptr(
            key,
            b"FontType\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            argn = parse_nvalue(
                start,
                end,
                &raw mut argv as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if argn != 1 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            if argv[0 as ::core::ffi::c_int as usize] != 1.0f64 {
                pdf_warning!(
                    b"FontType %d not supported.\0" as *const u8 as *const ::core::ffi::c_char,
                    argv[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
        } else if streq_ptr(
            key,
            b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            || streq_ptr(
                key,
                b"StrokeWidth\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"PaintType\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            argn = parse_nvalue(
                start,
                end,
                &raw mut argv as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if argn != 1 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            if argv[0 as ::core::ffi::c_int as usize] != 0.0f64 {
                cff_dict_add((*font).topdict, key, 1 as ::core::ffi::c_int);
                cff_dict_set(
                    (*font).topdict,
                    key,
                    0 as ::core::ffi::c_int,
                    argv[0 as ::core::ffi::c_int as usize],
                );
            }
        } else if streq_ptr(
            key,
            b"UnderLinePosition\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0
            || streq_ptr(
                key,
                b"UnderLineThickness\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            argn = parse_nvalue(
                start,
                end,
                &raw mut argv as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if argn != 1 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            cff_dict_add((*font).topdict, key, 1 as ::core::ffi::c_int);
            cff_dict_set(
                (*font).topdict,
                key,
                0 as ::core::ffi::c_int,
                argv[0 as ::core::ffi::c_int as usize],
            );
        } else if streq_ptr(
            key,
            b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            argn = parse_nvalue(
                start,
                end,
                &raw mut argv as *mut ::core::ffi::c_double,
                4 as ::core::ffi::c_int,
            );
            if argn != 4 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    4 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            cff_dict_add((*font).topdict, key, 4 as ::core::ffi::c_int);
            loop {
                let fresh12 = argn;
                argn = argn - 1;
                if !(fresh12 > 0 as ::core::ffi::c_int) {
                    break;
                }
                cff_dict_set((*font).topdict, key, argn, argv[argn as usize]);
            }
        } else if streq_ptr(
            key,
            b"FontMatrix\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            argn = parse_nvalue(
                start,
                end,
                &raw mut argv as *mut ::core::ffi::c_double,
                6 as ::core::ffi::c_int,
            );
            if argn != 6 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    6 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            if argv[0 as ::core::ffi::c_int as usize] != 0.001f64
                || argv[1 as ::core::ffi::c_int as usize] != 0.0f64
                || argv[2 as ::core::ffi::c_int as usize] != 0.0f64
                || argv[3 as ::core::ffi::c_int as usize] != 0.001f64
                || argv[4 as ::core::ffi::c_int as usize] != 0.0f64
                || argv[5 as ::core::ffi::c_int as usize] != 0.0f64
            {
                cff_dict_add((*font).topdict, key, 6 as ::core::ffi::c_int);
                loop {
                    let fresh13 = argn;
                    argn = argn - 1;
                    if !(fresh13 > 0 as ::core::ffi::c_int) {
                        break;
                    }
                    cff_dict_set((*font).topdict, key, argn, argv[argn as usize]);
                }
            }
        } else if streq_ptr(key, b"version\0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int
            != 0
            || streq_ptr(key, b"Notice\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"FullName\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"FamilyName\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
            || streq_ptr(key, b"Weight\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int
                != 0
            || streq_ptr(
                key,
                b"Copyright\0" as *const u8 as *const ::core::ffi::c_char,
            ) as ::core::ffi::c_int
                != 0
        {
            argn = parse_svalue(start, end, &raw mut strval);
            if argn != 1 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            let mut sid: s_SID = 0;
            cff_dict_add((*font).topdict, key, 1 as ::core::ffi::c_int);
            sid = cff_get_sid(font, strval) as s_SID;
            if sid as ::core::ffi::c_int == CFF_STRING_NOTDEF {
                sid = cff_add_string(font, strval, 0 as ::core::ffi::c_int);
            }
            cff_dict_set(
                (*font).topdict,
                key,
                0 as ::core::ffi::c_int,
                sid as ::core::ffi::c_double,
            );
            free(strval as *mut ::core::ffi::c_void);
        } else if streq_ptr(
            key,
            b"IsFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            argn = parse_bvalue(
                start,
                end,
                (&raw mut argv as *mut ::core::ffi::c_double)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_double,
            );
            if argn != 1 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%d values expected but only %d read.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    argn,
                );
                free(key as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            if argv[0 as ::core::ffi::c_int as usize] != 0.0f64 {
                cff_dict_add(
                    *(*font).private.offset(0 as ::core::ffi::c_int as isize),
                    key,
                    1 as ::core::ffi::c_int,
                );
                cff_dict_set(
                    *(*font).private.offset(0 as ::core::ffi::c_int as isize),
                    key,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int as ::core::ffi::c_double,
                );
            }
        }
        free(key as *mut ::core::ffi::c_void);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_is_pfb")]
pub unsafe extern "C-unwind" fn is_pfb(mut handle: rust_input_handle_t) -> bool {
    let mut sig: [::core::ffi::c_char; 15] = [0; 15];
    let mut i: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    ch = ttstub_input_getc(handle);
    if ch != 128 as ::core::ffi::c_int
        || {
            ch = ttstub_input_getc(handle);
            ch < 0 as ::core::ffi::c_int
        }
        || ch > 3 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        ch = ttstub_input_getc(handle);
        if ch < 0 as ::core::ffi::c_int {
            return false_0 != 0;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 14 as ::core::ffi::c_int {
        ch = ttstub_input_getc(handle);
        if ch < 0 as ::core::ffi::c_int {
            return false_0 != 0;
        }
        sig[i as usize] = ch as ::core::ffi::c_char;
        i += 1;
    }
    if memcmp(
        &raw mut sig as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        b"%!PS-AdobeFont\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        14 as size_t,
    ) == 0
        || memcmp(
            &raw mut sig as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            b"%!FontType1\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            11 as size_t,
        ) == 0
    {
        return true_0 != 0;
    }
    if memcmp(
        &raw mut sig as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        b"%!PS\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        sig[14 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
        pdf_warning!(
            b"Ambiguous PostScript resource type: %s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut sig as *mut ::core::ffi::c_char,
        );
        return true_0 != 0;
    }
    pdf_warning!(b"Not a PFB font file?\0" as *const u8 as *const ::core::ffi::c_char);
    return false_0 != 0;
}
pub const PFB_SEG_TYPE_ASCII: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PFB_SEG_TYPE_BINARY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn get_pfb_segment(
    mut handle: rust_input_handle_t,
    mut expected_type: ::core::ffi::c_int,
    mut length: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    let mut buffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut bytesread: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut ch: ::core::ffi::c_int = 0;
        let mut slen: ::core::ffi::c_int = 0;
        let mut rlen: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        ch = ttstub_input_getc(handle);
        if ch < 0 as ::core::ffi::c_int {
            break;
        }
        if ch != 128 as ::core::ffi::c_int {
            pdf_abort!(b"Not a pfb file?\0" as *const u8 as *const ::core::ffi::c_char);
        }
        ch = ttstub_input_getc(handle);
        if ch < 0 as ::core::ffi::c_int || ch != expected_type {
            crate::backend::runtime::input_seek(
                handle,
                -(2 as ::core::ffi::c_int) as ssize_t,
                SEEK_CUR,
            );
            break;
        } else {
            slen = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                ch = ttstub_input_getc(handle);
                if ch < 0 as ::core::ffi::c_int {
                    free(buffer as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                }
                slen = slen + (ch << 8 as ::core::ffi::c_int * i);
                i += 1;
            }
            buffer = renew(
                buffer as *mut ::core::ffi::c_void,
                ((bytesread + slen) as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
                    as uint32_t,
            ) as *mut ::core::ffi::c_uchar;
            while slen > 0 as ::core::ffi::c_int {
                rlen = ttstub_input_read(
                    handle,
                    (buffer as *mut ::core::ffi::c_char).offset(bytesread as isize),
                    slen as size_t,
                ) as ::core::ffi::c_int;
                if rlen < 0 as ::core::ffi::c_int {
                    free(buffer as *mut ::core::ffi::c_void);
                    return ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                }
                slen -= rlen;
                bytesread += rlen;
            }
        }
    }
    if bytesread == 0 as ::core::ffi::c_int {
        pdf_abort!(b"PFB segment length zero?\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    buffer = renew(
        buffer as *mut ::core::ffi::c_void,
        ((bytesread + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_uchar;
    *buffer.offset(bytesread as isize) = 0 as ::core::ffi::c_uchar;
    if !length.is_null() {
        *length = bytesread;
    }
    return buffer;
}
#[unsafe(export_name = "__pdf_impl_t1_get_standard_glyph")]
pub unsafe extern "C-unwind" fn t1_get_standard_glyph(
    mut code: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if StandardEncoding[code as usize].is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    return StandardEncoding[code as usize];
}
#[unsafe(export_name = "__pdf_impl_t1_get_fontname")]
pub unsafe extern "C-unwind" fn t1_get_fontname(
    mut handle: rust_input_handle_t,
    mut fontname: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut buffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut start: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut end: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut length: ::core::ffi::c_int = 0;
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fn_found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    buffer = get_pfb_segment(handle, PFB_SEG_TYPE_ASCII, &raw mut length);
    if buffer.is_null() || length == 0 as ::core::ffi::c_int {
        pdf_abort!(
            b"Reading PFB (ASCII part) file failed.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    start = buffer;
    end = buffer.offset(length as isize);
    if seek_operator(
        &raw mut start,
        end,
        b"begin\0" as *const u8 as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        free(buffer as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    while fn_found == 0 && start < end && {
        key = get_next_key(&raw mut start, end);
        !key.is_null()
    } {
        if streq_ptr(
            key,
            b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            let mut strval: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            if parse_svalue(&raw mut start, end, &raw mut strval) == 1 as ::core::ffi::c_int {
                if strlen(strval) > TYPE1_NAME_LEN_MAX as size_t {
                    pdf_warning!(
                        b"FontName \"%s\" too long. (%zu bytes)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        strval,
                        strlen(strval),
                    );
                    *strval.offset(TYPE1_NAME_LEN_MAX as isize) =
                        '\0' as i32 as ::core::ffi::c_char;
                }
                strcpy(fontname, strval);
                free(strval as *mut ::core::ffi::c_void);
                fn_found = 1 as ::core::ffi::c_int;
            }
        }
        free(key as *mut ::core::ffi::c_void);
    }
    free(buffer as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn init_cff_font(mut cff: *mut cff_font) {
    (*cff).handle = INVALID_HANDLE as rust_input_handle_t;
    (*cff).filter = 0 as ::core::ffi::c_int;
    (*cff).fontname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*cff).index = 0 as ::core::ffi::c_int;
    (*cff).flag = FONTTYPE_FONT;
    (*cff).header.major = 1 as card8;
    (*cff).header.minor = 0 as card8;
    (*cff).header.hdr_size = 4 as card8;
    (*cff).header.offsize = 4 as c_offsize;
    (*cff).name = cff_new_index(1 as card16);
    (*cff).topdict = cff_new_dict();
    (*cff).string = ::core::ptr::null_mut::<cff_index>();
    (*cff).gsubr = cff_new_index(0 as card16);
    (*cff).encoding = ::core::ptr::null_mut::<cff_encoding>();
    (*cff).charsets = ::core::ptr::null_mut::<cff_charsets>();
    (*cff).fdselect = ::core::ptr::null_mut::<cff_fdselect>();
    (*cff).cstrings = ::core::ptr::null_mut::<cff_index>();
    (*cff).fdarray = ::core::ptr::null_mut::<*mut cff_dict>();
    (*cff).private = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<*mut cff_dict>() as usize)
        as uint32_t) as *mut *mut cff_dict;
    let ref mut fresh23 = *(*cff).private.offset(0 as ::core::ffi::c_int as isize);
    *fresh23 = cff_new_dict();
    (*cff).subrs = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<*mut cff_index>() as usize)
        as uint32_t) as *mut *mut cff_index;
    let ref mut fresh24 = *(*cff).subrs.offset(0 as ::core::ffi::c_int as isize);
    *fresh24 = ::core::ptr::null_mut::<cff_index>();
    (*cff).offset = 0 as l_offset;
    (*cff).gsubr_offset = 0 as l_offset;
    (*cff).num_glyphs = 0 as card16;
    (*cff).num_fds = 1 as card8;
    (*cff)._string = cff_new_index(0 as card16);
}
#[unsafe(export_name = "__pdf_impl_t1_load_font")]
pub unsafe extern "C-unwind" fn t1_load_font(
    mut enc_vec: *mut *mut ::core::ffi::c_char,
    mut mode: ::core::ffi::c_int,
    mut handle: rust_input_handle_t,
) -> *mut cff_font {
    let mut length: ::core::ffi::c_int = 0;
    let mut cff: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut buffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut start: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut end: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    buffer = get_pfb_segment(handle, PFB_SEG_TYPE_ASCII, &raw mut length);
    if buffer.is_null() || length == 0 as ::core::ffi::c_int {
        pdf_abort!(
            b"Reading PFB (ASCII part) file failed.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    cff = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_font>() as usize) as uint32_t)
        as *mut cff_font;
    init_cff_font(cff);
    start = buffer;
    end = buffer.offset(length as isize);
    if parse_part1(cff, enc_vec, &raw mut start, end) < 0 as ::core::ffi::c_int {
        cff_close(cff);
        free(buffer as *mut ::core::ffi::c_void);
        pdf_abort!(
            b"Reading PFB (ASCII part) file failed.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    free(buffer as *mut ::core::ffi::c_void);
    buffer = get_pfb_segment(handle, PFB_SEG_TYPE_BINARY, &raw mut length);
    if buffer.is_null() || length == 0 as ::core::ffi::c_int {
        cff_close(cff);
        free(buffer as *mut ::core::ffi::c_void);
        pdf_abort!(
            b"Reading PFB (BINARY part) file failed.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        t1_decrypt(
            T1_EEKEY as ::core::ffi::c_ushort,
            buffer,
            buffer,
            0 as ::core::ffi::c_int,
            length,
        );
    }
    start = buffer.offset(4 as ::core::ffi::c_int as isize);
    end = buffer.offset(length as isize);
    if parse_part2(cff, &raw mut start, end, mode) < 0 as ::core::ffi::c_int {
        cff_close(cff);
        free(buffer as *mut ::core::ffi::c_void);
        pdf_abort!(
            b"Reading PFB (BINARY part) file failed.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    free(buffer as *mut ::core::ffi::c_void);
    cff_update_string(cff);
    return cff;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
    fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pst_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_is_pfb(
    mut handle: rust_input_handle_t,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { is_pfb(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_t1_get_standard_glyph(
    mut code: ::core::ffi::c_int,
    _result: *mut *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { t1_get_standard_glyph(code) }) {
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
pub unsafe extern "C" fn __pdf_entry_t1_get_fontname(
    mut handle: rust_input_handle_t,
    mut fontname: *mut ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { t1_get_fontname(handle, fontname) }) {
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
pub unsafe extern "C" fn __pdf_entry_t1_load_font(
    mut enc_vec: *mut *mut ::core::ffi::c_char,
    mut mode: ::core::ffi::c_int,
    mut handle: rust_input_handle_t,
    _result: *mut *mut cff_font,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { t1_load_font(enc_vec, mode, handle) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
