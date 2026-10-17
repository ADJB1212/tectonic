/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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
use crate::unicode::is_valid as UC_is_valid;
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isupper(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strtol(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
    -> *mut ::core::ffi::c_char;
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
    fn strncpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    static mut dpx_conf: _dpx_conf;
    #[link_name = "__pdf_impl_tt_mfgets"]
    fn tt_mfgets(
        buffer: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_int,
        file: rust_input_handle_t,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_dpx_tt_open"]
    fn dpx_tt_open(
        filename: *const ::core::ffi::c_char,
        suffix: *const ::core::ffi::c_char,
        format: ttbc_file_format,
    ) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_ht_init_table"]
    fn ht_init_table(ht: *mut ht_table, hval_free_fn: hval_free_func);
    #[link_name = "__pdf_impl_ht_clear_table"]
    fn ht_clear_table(ht: *mut ht_table);
    #[link_name = "__pdf_impl_ht_lookup_table"]
    fn ht_lookup_table(
        ht: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_ht_append_table"]
    fn ht_append_table(
        ht: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
        value: *mut ::core::ffi::c_void,
    );
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_parse_ident"]
    fn parse_ident(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_UC_UTF16BE_encode_char"]
    fn UC_UTF16BE_encode_char(
        ucv: int32_t,
        dstpp: *mut *mut ::core::ffi::c_uchar,
        endptr: *mut ::core::ffi::c_uchar,
    ) -> size_t;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
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
pub struct agl_name {
    pub name: *mut ::core::ffi::c_char,
    pub suffix: *mut ::core::ffi::c_char,
    pub n_components: ::core::ffi::c_int,
    pub unicodes: [int32_t; 16],
    pub alternate: *mut agl_name,
    pub is_predef: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub keep_cache: ::core::ffi::c_int,
}
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_table {
    pub count: ::core::ffi::c_int,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut ::core::ffi::c_char,
    pub keylen: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub key: *const ::core::ffi::c_char,
    pub otl_tag: *const ::core::ffi::c_char,
    pub suffixes: [*const ::core::ffi::c_char; 16],
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
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
pub const AGL_DEFAULT_LISTFILE: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"glyphlist.txt\0") };
pub const AGL_PREDEF_LISTFILE: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"pdfglyphlist.txt\0")
};
pub const AGL_EXTRA_LISTFILE: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"texglyphlist.txt\0")
};
pub const AGL_MAX_UNICODES: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn agl_new_name() -> *mut agl_name {
    let mut agln: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
    agln = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<agl_name>() as usize) as uint32_t)
        as *mut agl_name;
    (*agln).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*agln).suffix = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*agln).n_components = 0 as ::core::ffi::c_int;
    (*agln).alternate = ::core::ptr::null_mut::<agl_name>();
    (*agln).is_predef = 0 as ::core::ffi::c_int;
    return agln;
}
unsafe extern "C-unwind" fn agl_release_name(mut agln: *mut agl_name) {
    let mut next: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
    while !agln.is_null() {
        next = (*agln).alternate as *mut agl_name;
        free((*agln).name as *mut ::core::ffi::c_void);
        free((*agln).suffix as *mut ::core::ffi::c_void);
        (*agln).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free(agln as *mut ::core::ffi::c_void);
        agln = next;
    }
}
#[unsafe(export_name = "__pdf_impl_agl_chop_suffix")]
pub unsafe extern "C-unwind" fn agl_chop_suffix(
    mut glyphname: *const ::core::ffi::c_char,
    mut suffix: *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    if !(!glyphname.is_null() && !suffix.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"glyphname && suffix\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    p = strchr(glyphname, '.' as i32);
    if !p.is_null() {
        len = strlen(glyphname).wrapping_sub(strlen(p)) as ::core::ffi::c_int;
        if len < 1 as ::core::ffi::c_int {
            name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            *suffix = new((strlen(glyphname) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            strcpy(*suffix, glyphname.offset(1 as ::core::ffi::c_int as isize));
        } else {
            p = p.offset(1);
            name = new(((len + 1 as ::core::ffi::c_int) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            strncpy(name, glyphname, len as size_t);
            *name.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
            if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
                *suffix = ::core::ptr::null_mut::<::core::ffi::c_char>();
            } else {
                *suffix = new((strlen(p).wrapping_add(1 as size_t) as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                    as uint32_t) as *mut ::core::ffi::c_char;
                strcpy(*suffix, p);
            }
        }
    } else {
        name = new(
            (strlen(glyphname).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy(name, glyphname);
        *suffix = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return name;
}
static mut modifiers: [*const ::core::ffi::c_char; 21] = [
    b"acute\0" as *const u8 as *const ::core::ffi::c_char,
    b"breve\0" as *const u8 as *const ::core::ffi::c_char,
    b"caron\0" as *const u8 as *const ::core::ffi::c_char,
    b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"grave\0" as *const u8 as *const ::core::ffi::c_char,
    b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char,
    b"macron\0" as *const u8 as *const ::core::ffi::c_char,
    b"ogonek\0" as *const u8 as *const ::core::ffi::c_char,
    b"ring\0" as *const u8 as *const ::core::ffi::c_char,
    b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"commaaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"slash\0" as *const u8 as *const ::core::ffi::c_char,
    b"ampersand\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclam\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char,
    b"question\0" as *const u8 as *const ::core::ffi::c_char,
    b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C-unwind" fn skip_capital(
    mut p: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut slen: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = 0;
    len = endptr.offset_from(*p) as ::core::ffi::c_long as ::core::ffi::c_int;
    if len >= 2 as ::core::ffi::c_int
        && (**p as ::core::ffi::c_int == 'A' as i32
            && *(*p).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'E' as i32
            || **p as ::core::ffi::c_int == 'O' as i32
                && *(*p).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == 'E' as i32)
    {
        *p = (*p).offset(2 as ::core::ffi::c_int as isize);
        slen = 2 as ::core::ffi::c_int;
    } else if len >= 3 as ::core::ffi::c_int
        && **p as ::core::ffi::c_int == 'E' as i32
        && *(*p).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 't' as i32
        && *(*p).offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'h' as i32
    {
        *p = (*p).offset(3 as ::core::ffi::c_int as isize);
        slen = 3 as ::core::ffi::c_int;
    } else if len >= 5 as ::core::ffi::c_int
        && **p as ::core::ffi::c_int == 'T' as i32
        && *(*p).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'h' as i32
        && *(*p).offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'o' as i32
        && *(*p).offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'r' as i32
        && *(*p).offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'n' as i32
    {
        *p = (*p).offset(5 as ::core::ffi::c_int as isize);
        slen = 5 as ::core::ffi::c_int;
    } else if len >= 1 as ::core::ffi::c_int
        && **p as ::core::ffi::c_int >= 'A' as i32
        && **p as ::core::ffi::c_int <= 'Z' as i32
    {
        *p = (*p).offset(1 as ::core::ffi::c_int as isize);
        slen = 1 as ::core::ffi::c_int;
    }
    return slen;
}
unsafe extern "C-unwind" fn skip_modifier(
    mut p: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> size_t {
    let mut slen: size_t = 0 as size_t;
    let mut len: size_t = 0;
    let mut i: ::core::ffi::c_uint = 0;
    len = endptr.offset_from(*p) as ::core::ffi::c_long as size_t;
    i = 0 as ::core::ffi::c_uint;
    while !modifiers[i as usize].is_null() {
        if len >= strlen(modifiers[i as usize])
            && memcmp(
                *p as *const ::core::ffi::c_void,
                modifiers[i as usize] as *const ::core::ffi::c_void,
                len,
            ) == 0
        {
            slen = strlen(modifiers[i as usize]);
            *p = (*p).offset(slen as isize);
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    return slen;
}
unsafe extern "C-unwind" fn is_smallcap(mut glyphname: *const ::core::ffi::c_char) -> bool {
    let mut len: size_t = 0;
    let mut slen: size_t = 0;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if glyphname.is_null() {
        return false_0 != 0;
    }
    p = glyphname;
    len = strlen(glyphname);
    if len < 6 as size_t
        || strcmp(
            p.offset(len as isize)
                .offset(-(5 as ::core::ffi::c_int as isize)),
            b"small\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        return false_0 != 0;
    }
    endptr = p
        .offset(len as isize)
        .offset(-(5 as ::core::ffi::c_int as isize));
    len = len.wrapping_sub(5 as size_t);
    slen = skip_modifier(&raw mut p, endptr);
    if slen == len {
        return true_0 != 0;
    } else if slen > 0 as size_t {
        return false_0 != 0;
    }
    len = len.wrapping_sub(skip_capital(&raw mut p, endptr) as size_t);
    if len == 0 as size_t {
        return true_0 != 0;
    }
    while len > 0 as size_t {
        slen = skip_modifier(&raw mut p, endptr);
        if slen == 0 as size_t {
            return false_0 != 0;
        }
        len = len.wrapping_sub(slen);
    }
    return true_0 != 0;
}
pub const AGL_VAR_SMCP_IDX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut var_list: [C2RustUnnamed_0; 14] = [
    C2RustUnnamed_0 {
        key: b"small\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: b"smcp\0" as *const u8 as *const ::core::ffi::c_char,
        suffixes: [
            b"sc\0" as *const u8 as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"swash\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: b"swsh\0" as *const u8 as *const ::core::ffi::c_char,
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"superior\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: b"sups\0" as *const u8 as *const ::core::ffi::c_char,
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"inferior\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: b"sinf\0" as *const u8 as *const ::core::ffi::c_char,
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"numerator\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: b"numr\0" as *const u8 as *const ::core::ffi::c_char,
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"denominator\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: b"dnom\0" as *const u8 as *const ::core::ffi::c_char,
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"oldstyle\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: b"onum\0" as *const u8 as *const ::core::ffi::c_char,
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"display\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: ::core::ptr::null::<::core::ffi::c_char>(),
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"text\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: ::core::ptr::null::<::core::ffi::c_char>(),
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"big\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: ::core::ptr::null::<::core::ffi::c_char>(),
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"bigg\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: ::core::ptr::null::<::core::ffi::c_char>(),
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"Big\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: ::core::ptr::null::<::core::ffi::c_char>(),
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: b"Bigg\0" as *const u8 as *const ::core::ffi::c_char,
        otl_tag: ::core::ptr::null::<::core::ffi::c_char>(),
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
    C2RustUnnamed_0 {
        key: ::core::ptr::null::<::core::ffi::c_char>(),
        otl_tag: ::core::ptr::null::<::core::ffi::c_char>(),
        suffixes: [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ],
    },
];
#[unsafe(export_name = "__pdf_impl_agl_suffix_to_otltag")]
pub unsafe extern "C-unwind" fn agl_suffix_to_otltag(
    mut suffix: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !var_list[i as usize].key.is_null() {
        j = 0 as ::core::ffi::c_int;
        while !var_list[i as usize].suffixes[j as usize].is_null() {
            if streq_ptr(suffix, var_list[i as usize].suffixes[j as usize]) {
                return var_list[i as usize].otl_tag;
            }
            j += 1;
        }
        if streq_ptr(suffix, var_list[i as usize].key) {
            return var_list[i as usize].otl_tag;
        }
        if !var_list[i as usize].otl_tag.is_null()
            && streq_ptr(suffix, var_list[i as usize].otl_tag) as ::core::ffi::c_int != 0
        {
            return var_list[i as usize].otl_tag;
        }
        i += 1;
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C-unwind" fn agl_guess_name(mut glyphname: *const ::core::ffi::c_char) -> ssize_t {
    let mut i: ssize_t = 0;
    let mut len: size_t = 0;
    if is_smallcap(glyphname) {
        return AGL_VAR_SMCP_IDX as ssize_t;
    }
    len = strlen(glyphname);
    i = 1 as ssize_t;
    while !var_list[i as usize].key.is_null() {
        if len > strlen(var_list[i as usize].key)
            && streq_ptr(
                glyphname
                    .offset(len as isize)
                    .offset(-(strlen(var_list[i as usize].key) as isize)),
                var_list[i as usize].key,
            ) as ::core::ffi::c_int
                != 0
        {
            return i;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int) as ssize_t;
}
unsafe extern "C-unwind" fn agl_normalized_name(
    mut glyphname: *mut ::core::ffi::c_char,
) -> *mut agl_name {
    let mut agln: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
    let mut suffix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    if glyphname.is_null() {
        return ::core::ptr::null_mut::<agl_name>();
    }
    agln = agl_new_name();
    suffix = strchr(glyphname, '.' as i32);
    if !suffix.is_null() {
        n = strlen(glyphname).wrapping_sub(strlen(suffix)) as ::core::ffi::c_int;
        if *suffix.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32 {
            (*agln).suffix = new((strlen(suffix) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            strcpy(
                (*agln).suffix,
                suffix.offset(1 as ::core::ffi::c_int as isize),
            );
        }
        (*agln).name = new(((n + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memcpy(
            (*agln).name as *mut ::core::ffi::c_void,
            glyphname as *const ::core::ffi::c_void,
            n as size_t,
        );
        *(*agln).name.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else if is_smallcap(glyphname) {
        n = strlen(glyphname).wrapping_sub(5 as size_t) as ::core::ffi::c_int;
        (*agln).suffix = new((3 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        strcpy(
            (*agln).suffix,
            b"sc\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*agln).name = new(((n + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while i < n {
            *(*agln).name.offset(i as isize) = (if isupper(*glyphname.offset(i as isize)
                as ::core::ffi::c_uchar
                as ::core::ffi::c_int)
                != 0
            {
                *glyphname.offset(i as isize) as ::core::ffi::c_int + 32 as ::core::ffi::c_int
            } else {
                *glyphname.offset(i as isize) as ::core::ffi::c_int
            }) as ::core::ffi::c_char;
            i += 1;
        }
        *(*agln).name.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else {
        let mut var_idx: ssize_t = 0;
        var_idx = agl_guess_name(glyphname);
        if var_idx < 0 as ssize_t || var_list[var_idx as usize].key.is_null() {
            n = strlen(glyphname) as ::core::ffi::c_int;
        } else {
            n = strlen(glyphname).wrapping_sub(strlen(var_list[var_idx as usize].key))
                as ::core::ffi::c_int;
            if !var_list[var_idx as usize].suffixes[0 as ::core::ffi::c_int as usize].is_null() {
                (*agln).suffix = new((strlen(
                    var_list[var_idx as usize].suffixes[0 as ::core::ffi::c_int as usize],
                )
                .wrapping_add(1 as size_t) as uint32_t
                    as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                    as uint32_t) as *mut ::core::ffi::c_char;
                strcpy(
                    (*agln).suffix,
                    var_list[var_idx as usize].suffixes[0 as ::core::ffi::c_int as usize],
                );
            } else {
                (*agln).suffix = new((strlen(var_list[var_idx as usize].key)
                    .wrapping_add(1 as size_t) as uint32_t
                    as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                    as uint32_t) as *mut ::core::ffi::c_char;
                strcpy((*agln).suffix, var_list[var_idx as usize].key);
            }
        }
        (*agln).name = new(((n + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memcpy(
            (*agln).name as *mut ::core::ffi::c_void,
            glyphname as *const ::core::ffi::c_void,
            n as size_t,
        );
        *(*agln).name.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    return agln;
}
static mut aglmap: ht_table = ht_table {
    count: 0,
    hval_free_fn: None,
    table: [::core::ptr::null::<ht_entry>() as *mut ht_entry; 503],
};
#[inline]
unsafe extern "C-unwind" fn hval_free(mut hval: *mut ::core::ffi::c_void) {
    agl_release_name(hval as *mut agl_name);
}
#[unsafe(export_name = "__pdf_impl_agl_init_map")]
pub unsafe extern "C-unwind" fn agl_init_map() {
    ht_init_table(
        &raw mut aglmap,
        Some(hval_free as unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> ()),
    );
    agl_load_listfile(AGL_EXTRA_LISTFILE.as_ptr(), 0 as ::core::ffi::c_int);
    if agl_load_listfile(AGL_PREDEF_LISTFILE.as_ptr(), 1 as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Failed to load AGL file \"%s\"...\0" as *const u8 as *const ::core::ffi::c_char,
            AGL_PREDEF_LISTFILE.as_ptr(),
        );
    }
    if agl_load_listfile(AGL_DEFAULT_LISTFILE.as_ptr(), 0 as ::core::ffi::c_int)
        < 0 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Failed to load AGL file \"%s\"...\0" as *const u8 as *const ::core::ffi::c_char,
            AGL_DEFAULT_LISTFILE.as_ptr(),
        );
    }
}
#[unsafe(export_name = "__pdf_impl_agl_close_map")]
pub unsafe extern "C-unwind" fn agl_close_map() {
    ht_clear_table(&raw mut aglmap);
}
pub const WBUF_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn agl_load_listfile(
    mut filename: *const ::core::ffi::c_char,
    mut is_predef: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nextptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut wbuf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut handle: rust_input_handle_t = 0;
    if filename.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    handle = dpx_tt_open(
        filename,
        b".txt\0" as *const u8 as *const ::core::ffi::c_char,
        TTBC_FILE_FORMAT_FONT_MAP,
    );
    if handle == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(
            b"<AGL:%s\0" as *const u8 as *const ::core::ffi::c_char,
            filename
        );
    }
    loop {
        p = tt_mfgets(&raw mut wbuf as *mut ::core::ffi::c_char, WBUF_SIZE, handle);
        if p.is_null() {
            break;
        }
        let mut agln: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
        let mut duplicate: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
        let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut n_unicodes: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut unicodes: [int32_t; 16] = [0; 16];
        endptr = p.offset(strlen(p) as isize);
        skip_white(&raw mut p, endptr);
        if p.is_null()
            || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '#' as i32
            || p >= endptr
        {
            continue;
        }
        nextptr = strchr(p, ';' as i32);
        if nextptr.is_null() || nextptr == p as *mut ::core::ffi::c_char {
            continue;
        }
        name = parse_ident(&raw mut p, nextptr);
        skip_white(&raw mut p, endptr);
        if name.is_null()
            || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != ';' as i32
        {
            pdf_warning!(
                b"Invalid AGL entry: %s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut wbuf as *mut ::core::ffi::c_char,
            );
            free(name as *mut ::core::ffi::c_void);
        } else {
            p = p.offset(1);
            skip_white(&raw mut p, endptr);
            n_unicodes = 0 as ::core::ffi::c_int;
            while p < endptr
                && (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
                    && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        <= '9' as i32
                    || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        >= 'A' as i32
                        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            <= 'F' as i32)
            {
                if n_unicodes >= AGL_MAX_UNICODES {
                    pdf_warning!(
                        b"Too many Unicode values\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    break;
                } else {
                    let fresh4 = n_unicodes;
                    n_unicodes = n_unicodes + 1;
                    unicodes[fresh4 as usize] =
                        strtol(p, &raw mut nextptr, 16 as ::core::ffi::c_int) as int32_t;
                    p = nextptr;
                    skip_white(&raw mut p, endptr);
                }
            }
            if n_unicodes == 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"AGL entry ignored (no mapping): %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &raw mut wbuf as *mut ::core::ffi::c_char,
                );
                free(name as *mut ::core::ffi::c_void);
            } else {
                agln = agl_normalized_name(name);
                (*agln).is_predef = is_predef;
                (*agln).n_components = n_unicodes;
                i = 0 as ::core::ffi::c_int;
                while i < n_unicodes {
                    (*agln).unicodes[i as usize] = unicodes[i as usize];
                    i += 1;
                }
                duplicate = ht_lookup_table(
                    &raw mut aglmap,
                    name as *const ::core::ffi::c_void,
                    strlen(name) as ::core::ffi::c_int,
                ) as *mut agl_name;
                if duplicate.is_null() {
                    ht_append_table(
                        &raw mut aglmap,
                        name as *const ::core::ffi::c_void,
                        strlen(name) as ::core::ffi::c_int,
                        agln as *mut ::core::ffi::c_void,
                    );
                } else {
                    while !(*duplicate).alternate.is_null() {
                        duplicate = (*duplicate).alternate as *mut agl_name;
                    }
                    (*duplicate).alternate = agln as *mut agl_name;
                }
                if dpx_conf.verbose_level > 5 as ::core::ffi::c_int {
                    if !(*agln).suffix.is_null() {
                        pdf_message!(
                            b"agl: %s [%s.%s] -->\0" as *const u8 as *const ::core::ffi::c_char,
                            name,
                            (*agln).name,
                            (*agln).suffix,
                        );
                    } else {
                        pdf_message!(
                            b"agl: %s [%s] -->\0" as *const u8 as *const ::core::ffi::c_char,
                            name,
                            (*agln).name,
                        );
                    }
                    i = 0 as ::core::ffi::c_int;
                    while i < (*agln).n_components {
                        if (*agln).unicodes[i as usize] > 0xffff as int32_t {
                            pdf_message!(
                                b" U+%06X\0" as *const u8 as *const ::core::ffi::c_char,
                                (*agln).unicodes[i as usize],
                            );
                        } else {
                            pdf_message!(
                                b" U+%04X\0" as *const u8 as *const ::core::ffi::c_char,
                                (*agln).unicodes[i as usize],
                            );
                        }
                        i += 1;
                    }
                    pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
                }
                free(name as *mut ::core::ffi::c_void);
                count += 1;
            }
        }
    }
    crate::backend::runtime::input_close(handle);
    if dpx_conf.verbose_level != 0 {
        pdf_message!(b">\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return count;
}
#[unsafe(export_name = "__pdf_impl_agl_lookup_list")]
pub unsafe extern "C-unwind" fn agl_lookup_list(
    mut glyphname: *const ::core::ffi::c_char,
) -> *mut agl_name {
    let mut agln: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
    if glyphname.is_null() {
        return ::core::ptr::null_mut::<agl_name>();
    }
    agln = ht_lookup_table(
        &raw mut aglmap,
        glyphname as *const ::core::ffi::c_void,
        strlen(glyphname) as ::core::ffi::c_int,
    ) as *mut agl_name;
    return agln;
}
#[unsafe(export_name = "__pdf_impl_agl_name_is_unicode")]
pub unsafe extern "C-unwind" fn agl_name_is_unicode(
    mut glyphname: *const ::core::ffi::c_char,
) -> bool {
    let mut c: ::core::ffi::c_char = 0;
    let mut suffix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: size_t = 0;
    let mut len: size_t = 0;
    if glyphname.is_null() {
        return false_0 != 0;
    }
    suffix = strchr(glyphname, '.' as i32);
    len = if !suffix.is_null() {
        suffix.offset_from(glyphname) as ::core::ffi::c_long as size_t
    } else {
        strlen(glyphname)
    };
    if len >= 7 as size_t
        && len.wrapping_sub(3 as size_t).wrapping_rem(4 as size_t) == 0 as size_t
        && !strstartswith(
            glyphname,
            b"uni\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        c = *glyphname.offset(3 as ::core::ffi::c_int as isize);
        if isdigit(c as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
            || c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'F' as i32
        {
            return true_0 != 0;
        } else {
            return false_0 != 0;
        }
    } else if len <= 7 as size_t
        && len >= 5 as size_t
        && *glyphname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'u' as i32
    {
        i = 1 as size_t;
        while i < len.wrapping_sub(1 as size_t) {
            c = *glyphname.offset(i as isize);
            if isdigit(c as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0
                && ((c as ::core::ffi::c_int) < 'A' as i32 || c as ::core::ffi::c_int > 'F' as i32)
            {
                return false_0 != 0;
            }
            i = i.wrapping_add(1);
        }
        return true_0 != 0;
    }
    return false_0 != 0;
}
#[unsafe(export_name = "__pdf_impl_agl_name_convert_unicode")]
pub unsafe extern "C-unwind" fn agl_name_convert_unicode(
    mut glyphname: *const ::core::ffi::c_char,
) -> int32_t {
    let mut ucv: int32_t = -(1 as int32_t);
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !agl_name_is_unicode(glyphname) {
        return -(1 as int32_t);
    }
    if strlen(glyphname) > 7 as size_t
        && *glyphname.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '.' as i32
    {
        pdf_warning!(
            b"Mapping to multiple Unicode characters not supported.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as int32_t);
    }
    if *glyphname.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'n' as i32 {
        p = glyphname.offset(3 as ::core::ffi::c_int as isize);
    } else {
        p = glyphname.offset(1 as ::core::ffi::c_int as isize);
    }
    ucv = 0 as ::core::ffi::c_int as int32_t;
    while *p as ::core::ffi::c_int != '\0' as i32 && *p as ::core::ffi::c_int != '.' as i32 {
        if isdigit(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0
            && ((*p as ::core::ffi::c_int) < 'A' as i32 || *p as ::core::ffi::c_int > 'F' as i32)
        {
            pdf_warning!(
                b"Invalid char %c in Unicode glyph name %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
                *p as ::core::ffi::c_int,
                glyphname,
            );
            return -(1 as int32_t);
        }
        ucv <<= 4 as ::core::ffi::c_int;
        ucv = (ucv as ::core::ffi::c_int
            + if isdigit(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
                *p as ::core::ffi::c_int - '0' as i32
            } else {
                *p as ::core::ffi::c_int - 'A' as i32 + 10 as ::core::ffi::c_int
            }) as int32_t;
        p = p.offset(1);
    }
    if !UC_is_valid(ucv) {
        if ucv < 0x10000 as int32_t {
            pdf_warning!(
                b"Invalid Unicode code value U+%04X.\0" as *const u8 as *const ::core::ffi::c_char,
                ucv,
            );
        } else {
            pdf_warning!(
                b"Invalid Unicode code value U+%06X.\0" as *const u8 as *const ::core::ffi::c_char,
                ucv,
            );
        }
        ucv = -(1 as ::core::ffi::c_int) as int32_t;
    }
    return ucv;
}
unsafe extern "C-unwind" fn xtol(
    mut start: *const ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let fresh0 = len;
        len = len - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        v <<= 4 as ::core::ffi::c_int;
        if isdigit(*start as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            v += *start as ::core::ffi::c_int - '0' as i32;
        } else if *start as ::core::ffi::c_int >= 'A' as i32
            && *start as ::core::ffi::c_int <= 'F' as i32
        {
            v += *start as ::core::ffi::c_int - 'A' as i32 + 10 as ::core::ffi::c_int;
        } else {
            return -(1 as ::core::ffi::c_int);
        }
        start = start.offset(1);
    }
    return v;
}
unsafe extern "C-unwind" fn put_unicode_glyph(
    mut name: *const ::core::ffi::c_char,
    mut dstpp: *mut *mut ::core::ffi::c_uchar,
    mut limptr: *mut ::core::ffi::c_uchar,
) -> int32_t {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut len: int32_t = 0 as int32_t;
    let mut ucv: int32_t = 0;
    p = name;
    ucv = 0 as ::core::ffi::c_int as int32_t;
    if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'n' as i32 {
        p = p.offset(1 as ::core::ffi::c_int as isize);
        ucv = xtol(p, strlen(p) as ::core::ffi::c_int) as int32_t;
        len = (len as size_t).wrapping_add(UC_UTF16BE_encode_char(ucv, dstpp, limptr)) as int32_t
            as int32_t;
    } else {
        p = p.offset(3 as ::core::ffi::c_int as isize);
        while *p as ::core::ffi::c_int != '\0' as i32 {
            ucv = xtol(p, 4 as ::core::ffi::c_int) as int32_t;
            len = (len as size_t).wrapping_add(UC_UTF16BE_encode_char(ucv, dstpp, limptr))
                as int32_t as int32_t;
            p = p.offset(4 as ::core::ffi::c_int as isize);
        }
    }
    return len;
}
#[unsafe(export_name = "__pdf_impl_agl_sput_UTF16BE")]
pub unsafe extern "C-unwind" fn agl_sput_UTF16BE(
    mut glyphstr: *const ::core::ffi::c_char,
    mut dstpp: *mut *mut ::core::ffi::c_uchar,
    mut limptr: *mut ::core::ffi::c_uchar,
    mut fail_count: *mut ::core::ffi::c_int,
) -> int32_t {
    let mut len: int32_t = 0 as int32_t;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !(!glyphstr.is_null() && !dstpp.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"glyphstr && dstpp\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    p = glyphstr;
    endptr = strchr(p, '.' as i32);
    if endptr.is_null() {
        endptr = p.offset(strlen(p) as isize);
    }
    while p < endptr {
        let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut delim: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut sub_len: int32_t = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut agln0: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
        let mut agln1: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
        delim = strchr(p, '_' as i32);
        if delim == p {
            pdf_warning!(
                b"Invalid glyph name component in \"%s\".\0" as *const u8
                    as *const ::core::ffi::c_char,
                glyphstr,
            );
            count += 1;
            if !fail_count.is_null() {
                *fail_count = count;
            }
            return len;
        } else if delim.is_null() || delim > endptr {
            delim = endptr;
        }
        sub_len = delim.offset_from(p) as ::core::ffi::c_long as int32_t;
        name = new(((sub_len + 1 as int32_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memcpy(
            name as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            sub_len as size_t,
        );
        *name.offset(sub_len as isize) = '\0' as i32 as ::core::ffi::c_char;
        if agl_name_is_unicode(name) {
            sub_len = put_unicode_glyph(name, dstpp, limptr);
            if sub_len > 0 as int32_t {
                len += sub_len;
            } else {
                count += 1;
            }
        } else {
            agln1 = agl_lookup_list(name);
            if agln1.is_null()
                || (*agln1).n_components == 1 as ::core::ffi::c_int
                    && ((*agln1).unicodes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_long
                        >= 0xe000 as ::core::ffi::c_long
                        && (*agln1).unicodes[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_long
                            <= 0xf8ff as ::core::ffi::c_long
                        || (*agln1).unicodes[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_long
                            >= 0xf0000 as ::core::ffi::c_long
                            && (*agln1).unicodes[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_long
                                <= 0xffffd as ::core::ffi::c_long
                        || (*agln1).unicodes[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_long
                            >= 0x100000 as ::core::ffi::c_long
                            && (*agln1).unicodes[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_long
                                <= 0x10fffd as ::core::ffi::c_long)
            {
                agln0 = agl_normalized_name(name);
                if !agln0.is_null() {
                    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int
                        && !(*agln0).suffix.is_null()
                    {
                        pdf_warning!(
                            b"agl: fix %s --> %s.%s\0" as *const u8 as *const ::core::ffi::c_char,
                            name,
                            (*agln0).name,
                            (*agln0).suffix,
                        );
                    }
                    agln1 = agl_lookup_list((*agln0).name);
                    agl_release_name(agln0);
                }
            }
            if !agln1.is_null() {
                i = 0 as ::core::ffi::c_int;
                while i < (*agln1).n_components {
                    len = (len as size_t).wrapping_add(UC_UTF16BE_encode_char(
                        (*agln1).unicodes[i as usize],
                        dstpp,
                        limptr,
                    )) as int32_t as int32_t;
                    i += 1;
                }
            } else {
                if dpx_conf.verbose_level != 0 {
                    pdf_warning!(
                        b"No Unicode mapping for glyph name \"%s\" found.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        name,
                    );
                }
                count += 1;
            }
        }
        free(name as *mut ::core::ffi::c_void);
        p = delim.offset(1 as ::core::ffi::c_int as isize);
    }
    if !fail_count.is_null() {
        *fail_count = count;
    }
    return len;
}
#[unsafe(export_name = "__pdf_impl_agl_get_unicodes")]
pub unsafe extern "C-unwind" fn agl_get_unicodes(
    mut glyphstr: *const ::core::ffi::c_char,
    mut unicodes: *mut int32_t,
    mut max_unicodes: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = glyphstr;
    endptr = strchr(p, '.' as i32);
    if endptr.is_null() {
        endptr = p.offset(strlen(p) as isize);
    }
    while p < endptr {
        let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut delim: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut sub_len: int32_t = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut agln0: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
        let mut agln1: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
        delim = strchr(p, '_' as i32);
        if delim == p {
            pdf_warning!(
                b"Invalid glyph name component in \"%s\".\0" as *const u8
                    as *const ::core::ffi::c_char,
                glyphstr,
            );
            return -(1 as ::core::ffi::c_int);
        } else if delim.is_null() || delim > endptr {
            delim = endptr;
        }
        sub_len = delim.offset_from(p) as ::core::ffi::c_long as int32_t;
        name = new(((sub_len + 1 as int32_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memcpy(
            name as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            sub_len as size_t,
        );
        *name.offset(sub_len as isize) = '\0' as i32 as ::core::ffi::c_char;
        if agl_name_is_unicode(name) {
            p = name;
            if *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'n' as i32 {
                if count >= max_unicodes {
                    free(name as *mut ::core::ffi::c_void);
                    return -(1 as ::core::ffi::c_int);
                }
                p = p.offset(1);
                let fresh1 = count;
                count = count + 1;
                *unicodes.offset(fresh1 as isize) =
                    xtol(p, strlen(p) as ::core::ffi::c_int) as int32_t;
            } else {
                p = p.offset(3 as ::core::ffi::c_int as isize);
                while *p as ::core::ffi::c_int != '\0' as i32 {
                    if count >= max_unicodes {
                        free(name as *mut ::core::ffi::c_void);
                        return -(1 as ::core::ffi::c_int);
                    }
                    let fresh2 = count;
                    count = count + 1;
                    *unicodes.offset(fresh2 as isize) = xtol(p, 4 as ::core::ffi::c_int) as int32_t;
                    p = p.offset(4 as ::core::ffi::c_int as isize);
                }
            }
        } else {
            agln1 = agl_lookup_list(name);
            if agln1.is_null()
                || (*agln1).n_components == 1 as ::core::ffi::c_int
                    && ((*agln1).unicodes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_long
                        >= 0xe000 as ::core::ffi::c_long
                        && (*agln1).unicodes[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_long
                            <= 0xf8ff as ::core::ffi::c_long
                        || (*agln1).unicodes[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_long
                            >= 0xf0000 as ::core::ffi::c_long
                            && (*agln1).unicodes[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_long
                                <= 0xffffd as ::core::ffi::c_long
                        || (*agln1).unicodes[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_long
                            >= 0x100000 as ::core::ffi::c_long
                            && (*agln1).unicodes[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_long
                                <= 0x10fffd as ::core::ffi::c_long)
            {
                agln0 = agl_normalized_name(name);
                if !agln0.is_null() {
                    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int
                        && !(*agln0).suffix.is_null()
                    {
                        pdf_warning!(
                            b"agl: fix %s --> %s.%s\0" as *const u8 as *const ::core::ffi::c_char,
                            name,
                            (*agln0).name,
                            (*agln0).suffix,
                        );
                    }
                    agln1 = agl_lookup_list((*agln0).name);
                    agl_release_name(agln0);
                }
            }
            if !agln1.is_null() {
                if count + (*agln1).n_components > max_unicodes {
                    free(name as *mut ::core::ffi::c_void);
                    return -(1 as ::core::ffi::c_int);
                }
                i = 0 as ::core::ffi::c_int;
                while i < (*agln1).n_components {
                    let fresh3 = count;
                    count = count + 1;
                    *unicodes.offset(fresh3 as isize) = (*agln1).unicodes[i as usize];
                    i += 1;
                }
            } else {
                if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
                    pdf_warning!(
                        b"No Unicode mapping for glyph name \"%s\" found.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        name,
                    );
                }
                free(name as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
        }
        free(name as *mut ::core::ffi::c_void);
        p = delim.offset(1 as ::core::ffi::c_int as isize);
    }
    return count;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_agl_chop_suffix(
    mut glyphname: *const ::core::ffi::c_char,
    mut suffix: *mut *mut ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { agl_chop_suffix(glyphname, suffix) }) {
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
pub unsafe extern "C" fn __pdf_entry_agl_suffix_to_otltag(
    mut suffix: *const ::core::ffi::c_char,
    _result: *mut *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { agl_suffix_to_otltag(suffix) }) {
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
pub unsafe extern "C" fn __pdf_entry_agl_init_map() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { agl_init_map() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_agl_close_map() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { agl_close_map() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_agl_lookup_list(
    mut glyphname: *const ::core::ffi::c_char,
    _result: *mut *mut agl_name,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { agl_lookup_list(glyphname) }) {
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
pub unsafe extern "C" fn __pdf_entry_agl_name_is_unicode(
    mut glyphname: *const ::core::ffi::c_char,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { agl_name_is_unicode(glyphname) }) {
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
pub unsafe extern "C" fn __pdf_entry_agl_name_convert_unicode(
    mut glyphname: *const ::core::ffi::c_char,
    _result: *mut int32_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { agl_name_convert_unicode(glyphname) }) {
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
pub unsafe extern "C" fn __pdf_entry_agl_sput_UTF16BE(
    mut glyphstr: *const ::core::ffi::c_char,
    mut dstpp: *mut *mut ::core::ffi::c_uchar,
    mut limptr: *mut ::core::ffi::c_uchar,
    mut fail_count: *mut ::core::ffi::c_int,
    _result: *mut int32_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        agl_sput_UTF16BE(glyphstr, dstpp, limptr, fail_count)
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
pub unsafe extern "C" fn __pdf_entry_agl_get_unicodes(
    mut glyphstr: *const ::core::ffi::c_char,
    mut unicodes: *mut int32_t,
    mut max_unicodes: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        agl_get_unicodes(glyphstr, unicodes, max_unicodes)
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
