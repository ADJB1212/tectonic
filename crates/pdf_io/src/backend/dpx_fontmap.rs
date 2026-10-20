/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2017 by Jin-Hwan Cho and Shunsaku Hirata,
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
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isspace(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isascii(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn atof(_: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strtol(__str: *const ::core::ffi::c_char, __endptr: *mut *mut ::core::ffi::c_char, __base: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn strtoul(__str: *const ::core::ffi::c_char, __endptr: *mut *mut ::core::ffi::c_char, __base: ::core::ffi::c_int) -> ::core::ffi::c_ulong;
    fn strcat(__s1: *mut ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcpy(__dst: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strncmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char, __n: size_t) -> ::core::ffi::c_int;
    fn strstr(__big: *const ::core::ffi::c_char, __little: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn xmalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn tt_fontmap_reset();
    fn tt_fontmap_clear();
    fn tt_fontmap_lookup(key: *const ::core::ffi::c_char) -> *mut fontmap_rec;
    fn tt_fontmap_insert(key: *const ::core::ffi::c_char, record: *const fontmap_rec, replace: ::core::ffi::c_int) -> *mut fontmap_rec;
    fn tt_fontmap_remove(key: *const ::core::ffi::c_char);
    fn tt_fontmap_load_begin(handle: rust_input_handle_t, mode: ::core::ffi::c_int, verbose: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tt_fontmap_load_next(
        line: *mut *const ::core::ffi::c_char, line_number: *mut ::core::ffi::c_int, format: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tt_fontmap_load_end();
    static mut dpx_conf: _dpx_conf;
    fn sprintf(_: *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_dpx_tt_open"]
    fn dpx_tt_open(filename: *const ::core::ffi::c_char, suffix: *const ::core::ffi::c_char, format: ttbc_file_format) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_parse_float_decimal"]
    fn parse_float_decimal(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_c_string"]
    fn parse_c_string(pp: *mut *const ::core::ffi::c_char, endptr: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_release_sfd_record"]
    fn release_sfd_record();
    #[link_name = "__pdf_impl_sfd_get_subfont_ids"]
    fn sfd_get_subfont_ids(sfd_name: *const ::core::ffi::c_char, num_subfonts: *mut ::core::ffi::c_int) -> *mut *mut ::core::ffi::c_char;
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
pub struct fontmap_opt {
    pub slant: ::core::ffi::c_double,
    pub extend: ::core::ffi::c_double,
    pub bold: ::core::ffi::c_double,
    pub mapc: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub otl_tags: *mut ::core::ffi::c_char,
    pub tounicode: *mut ::core::ffi::c_char,
    pub design_size: ::core::ffi::c_double,
    pub charcoll: *mut ::core::ffi::c_char,
    pub index: uint32_t,
    pub style: ::core::ffi::c_int,
    pub stemv: ::core::ffi::c_int,
    pub use_glyph_encoding: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fontmap_rec {
    pub map_name: *mut ::core::ffi::c_char,
    pub font_name: *mut ::core::ffi::c_char,
    pub enc_name: *mut ::core::ffi::c_char,
    pub charmap: C2RustUnnamed,
    pub opt: fontmap_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sfd_name: *mut ::core::ffi::c_char,
    pub subfont_id: *mut ::core::ffi::c_char,
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
#[inline]
unsafe extern "C-unwind" fn streq_ptr(mut s1: *const ::core::ffi::c_char, mut s2: *const ::core::ffi::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
    }
    return false_0 != 0;
}
#[inline]
unsafe extern "C-unwind" fn strstartswith(mut s: *const ::core::ffi::c_char, mut prefix: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0 as ::core::ffi::c_int {
        return s.offset(length as isize);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
pub const FONTMAP_RMODE_REPLACE: ::core::ffi::c_int = 0;
pub const FONTMAP_RMODE_APPEND: ::core::ffi::c_int = 43;
pub const FONTMAP_RMODE_REMOVE: ::core::ffi::c_int = 45;
pub const FONTMAP_OPT_NOEMBED: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const FONTMAP_OPT_VERT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const FONTMAP_STYLE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FONTMAP_STYLE_BOLD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FONTMAP_STYLE_ITALIC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FONTMAP_STYLE_BOLDITALIC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_pdf_init_fontmap_record")]
pub unsafe extern "C-unwind" fn pdf_init_fontmap_record(mut mrec: *mut fontmap_rec) {
    if mrec.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"mrec\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    (*mrec).map_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mrec).charmap.sfd_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mrec).charmap.subfont_id = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mrec).opt.mapc = -(1 as ::core::ffi::c_int);
    (*mrec).font_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mrec).enc_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mrec).opt.slant = 0.0f64;
    (*mrec).opt.extend = 1.0f64;
    (*mrec).opt.bold = 0.0f64;
    (*mrec).opt.flags = 0 as ::core::ffi::c_int;
    (*mrec).opt.design_size = -1.0f64;
    (*mrec).opt.tounicode = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mrec).opt.otl_tags = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mrec).opt.index = 0 as uint32_t;
    (*mrec).opt.charcoll = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*mrec).opt.style = FONTMAP_STYLE_NONE;
    (*mrec).opt.stemv = -(1 as ::core::ffi::c_int);
    (*mrec).opt.use_glyph_encoding = 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_clear_fontmap_record")]
pub unsafe extern "C-unwind" fn pdf_clear_fontmap_record(mut mrec: *mut fontmap_rec) {
    if mrec.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"mrec\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    free((*mrec).map_name as *mut ::core::ffi::c_void);
    free((*mrec).charmap.sfd_name as *mut ::core::ffi::c_void);
    free((*mrec).charmap.subfont_id as *mut ::core::ffi::c_void);
    free((*mrec).enc_name as *mut ::core::ffi::c_void);
    free((*mrec).font_name as *mut ::core::ffi::c_void);
    free((*mrec).opt.tounicode as *mut ::core::ffi::c_void);
    free((*mrec).opt.otl_tags as *mut ::core::ffi::c_void);
    free((*mrec).opt.charcoll as *mut ::core::ffi::c_void);
    pdf_init_fontmap_record(mrec);
}
unsafe extern "C-unwind" fn mstrdup(mut s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if s.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    r = new(
        (strlen(s).wrapping_add(1 as size_t) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy(r, s);
    return r;
}
unsafe extern "C-unwind" fn fill_in_defaults(mut mrec: *mut fontmap_rec, mut tex_name: *const ::core::ffi::c_char) {
    if !(*mrec).enc_name.is_null()
        && (streq_ptr((*mrec).enc_name, b"default\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
            || streq_ptr((*mrec).enc_name, b"none\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0)
    {
        (*mrec).enc_name = mfree((*mrec).enc_name as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
    }
    if !(*mrec).font_name.is_null()
        && (streq_ptr((*mrec).font_name, b"default\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
            || streq_ptr((*mrec).font_name, b"none\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0)
    {
        (*mrec).font_name = mfree((*mrec).font_name as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
    }
    if (*mrec).font_name.is_null() {
        (*mrec).font_name = new((strlen(tex_name).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
            as *mut ::core::ffi::c_char;
        strcpy((*mrec).font_name, tex_name);
    }
    (*mrec).map_name = new((strlen(tex_name).wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    strcpy((*mrec).map_name, tex_name);
    if !(*mrec).charmap.sfd_name.is_null() && !(*mrec).enc_name.is_null() && (*mrec).opt.charcoll.is_null() {
        if (streq_ptr((*mrec).enc_name, b"Identity-H\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
            || streq_ptr((*mrec).enc_name, b"Identity-V\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0)
            && (!strstr((*mrec).charmap.sfd_name, b"Uni\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                || !strstr((*mrec).charmap.sfd_name, b"UBig\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                || !strstr((*mrec).charmap.sfd_name, b"UBg\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                || !strstr((*mrec).charmap.sfd_name, b"UGB\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                || !strstr((*mrec).charmap.sfd_name, b"UKS\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                || !strstr((*mrec).charmap.sfd_name, b"UJIS\0" as *const u8 as *const ::core::ffi::c_char).is_null())
        {
            (*mrec).opt.charcoll = new(
                (strlen(b"UCS\0" as *const u8 as *const ::core::ffi::c_char).wrapping_add(1 as size_t) as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
            ) as *mut ::core::ffi::c_char;
            strcpy((*mrec).opt.charcoll, b"UCS\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
}
unsafe extern "C-unwind" fn skip_blank(mut pp: *mut *const ::core::ffi::c_char, mut endptr: *const ::core::ffi::c_char) {
    let mut p: *const ::core::ffi::c_char = *pp;
    if p.is_null() || p >= endptr {
        return;
    }
    while p < endptr
        && (crate::backend::runtime::is_ascii(*p as ::core::ffi::c_int) != 0
            && (*p as ::core::ffi::c_uchar as ::core::ffi::c_int == ' ' as i32 || *p as ::core::ffi::c_uchar as ::core::ffi::c_int == '\t' as i32))
    {
        p = p.offset(1);
    }
    *pp = p;
}
unsafe extern "C-unwind" fn parse_string_value(
    mut pp: *mut *const ::core::ffi::c_char, mut endptr: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = *pp;
    let mut n: ::core::ffi::c_uint = 0;
    if p.is_null() || p >= endptr {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if *p as ::core::ffi::c_int == '"' as i32 {
        q = parse_c_string(&raw mut p, endptr);
    } else {
        n = 0 as ::core::ffi::c_uint;
        while p < endptr && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
            p = p.offset(1);
            n = n.wrapping_add(1);
        }
        if n == 0 as ::core::ffi::c_uint {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        q = new(
            (n.wrapping_add(1 as ::core::ffi::c_uint) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        memcpy(q as *mut ::core::ffi::c_void, *pp as *const ::core::ffi::c_void, n as size_t);
        *q.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    *pp = p;
    return q;
}
unsafe extern "C-unwind" fn parse_integer_value(
    mut pp: *mut *const ::core::ffi::c_char, mut endptr: *const ::core::ffi::c_char, mut base: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = *pp;
    let mut has_sign: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut has_prefix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0;
    if !(base == 0 as ::core::ffi::c_int || base >= 2 as ::core::ffi::c_int && base <= 36 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"base == 0 || (base >= 2 && base <= 36)\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if p.is_null() || p >= endptr {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if *p as ::core::ffi::c_int == '-' as i32 || *p as ::core::ffi::c_int == '+' as i32 {
        p = p.offset(1);
        has_sign = 1 as ::core::ffi::c_int;
    }
    if (base == 0 as ::core::ffi::c_int || base == 16 as ::core::ffi::c_int)
        && p.offset(2 as ::core::ffi::c_int as isize) <= endptr
        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '0' as i32
        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'x' as i32
    {
        p = p.offset(2 as ::core::ffi::c_int as isize);
        has_prefix = 1 as ::core::ffi::c_int;
    }
    if base == 0 as ::core::ffi::c_int {
        if has_prefix != 0 {
            base = 16 as ::core::ffi::c_int;
        } else if p < endptr && *p as ::core::ffi::c_int == '0' as i32 {
            base = 8 as ::core::ffi::c_int;
        } else {
            base = 10 as ::core::ffi::c_int;
        }
    }
    n = 0 as ::core::ffi::c_int;
    while p < endptr
        && (base <= 10 as ::core::ffi::c_int && *p as ::core::ffi::c_int >= '0' as i32 && (*p as ::core::ffi::c_int) < '0' as i32 + base
            || base > 10 as ::core::ffi::c_int
                && (*p as ::core::ffi::c_int >= '0' as i32 && *p as ::core::ffi::c_int <= '9' as i32
                    || *p as ::core::ffi::c_int >= 'a' as i32 && (*p as ::core::ffi::c_int) < 'a' as i32 + (base - 10 as ::core::ffi::c_int)
                    || *p as ::core::ffi::c_int >= 'A' as i32 && (*p as ::core::ffi::c_int) < 'A' as i32 + (base - 10 as ::core::ffi::c_int)))
    {
        p = p.offset(1);
        n += 1;
    }
    if n == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if has_sign != 0 {
        n += 1 as ::core::ffi::c_int;
    }
    if has_prefix != 0 {
        n += 2 as ::core::ffi::c_int;
    }
    q = new(((n + 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
        as *mut ::core::ffi::c_char;
    memcpy(q as *mut ::core::ffi::c_void, *pp as *const ::core::ffi::c_void, n as size_t);
    *q.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    *pp = p;
    return q;
}
unsafe extern "C-unwind" fn fontmap_parse_mapdef_dpm(
    mut mrec: *mut fontmap_rec, mut mapdef: *const ::core::ffi::c_char, mut endptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = mapdef;
    skip_blank(&raw mut p, endptr);
    if p < endptr && *p as ::core::ffi::c_int != '-' as i32 {
        (*mrec).enc_name = parse_string_value(&raw mut p, endptr);
        skip_blank(&raw mut p, endptr);
    }
    if p < endptr && *p as ::core::ffi::c_int != '-' as i32 {
        (*mrec).font_name = parse_string_value(&raw mut p, endptr);
        skip_blank(&raw mut p, endptr);
    }
    if !(*mrec).font_name.is_null() {
        let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        tmp = strip_options((*mrec).font_name, &raw mut (*mrec).opt);
        if !tmp.is_null() {
            free((*mrec).font_name as *mut ::core::ffi::c_void);
            (*mrec).font_name = tmp;
        }
    }
    skip_blank(&raw mut p, endptr);
    while p.offset(1 as ::core::ffi::c_int as isize) < endptr
        && *p as ::core::ffi::c_int != '\r' as i32
        && *p as ::core::ffi::c_int != '\n' as i32
        && *p as ::core::ffi::c_int == '-' as i32
    {
        let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut mopt: ::core::ffi::c_char = *p.offset(1 as ::core::ffi::c_int as isize);
        let mut v: ::core::ffi::c_int = 0;
        p = p.offset(2 as ::core::ffi::c_int as isize);
        skip_blank(&raw mut p, endptr);
        match mopt as ::core::ffi::c_int {
            115 => {
                q = parse_float_decimal(&raw mut p, endptr);
                if q.is_null() {
                    pdf_warning!(b"Missing a number value for 's' option.\0" as *const u8 as *const ::core::ffi::c_char,);
                    return -(1 as ::core::ffi::c_int);
                }
                (*mrec).opt.slant = atof(q);
                free(q as *mut ::core::ffi::c_void);
            }
            101 => {
                q = parse_float_decimal(&raw mut p, endptr);
                if q.is_null() {
                    pdf_warning!(b"Missing a number value for 'e' option.\0" as *const u8 as *const ::core::ffi::c_char,);
                    return -(1 as ::core::ffi::c_int);
                }
                (*mrec).opt.extend = atof(q);
                if (*mrec).opt.extend <= 0.0f64 {
                    pdf_warning!(b"Invalid value for 'e' option: %s\0" as *const u8 as *const ::core::ffi::c_char, q,);
                    return -(1 as ::core::ffi::c_int);
                }
                free(q as *mut ::core::ffi::c_void);
            }
            98 => {
                q = parse_float_decimal(&raw mut p, endptr);
                if q.is_null() {
                    pdf_warning!(b"Missing a number value for 'b' option.\0" as *const u8 as *const ::core::ffi::c_char,);
                    return -(1 as ::core::ffi::c_int);
                }
                (*mrec).opt.bold = atof(q);
                if (*mrec).opt.bold <= 0.0f64 {
                    pdf_warning!(b"Invalid value for 'b' option: %s\0" as *const u8 as *const ::core::ffi::c_char, q,);
                    return -(1 as ::core::ffi::c_int);
                }
                free(q as *mut ::core::ffi::c_void);
            }
            114 => {}
            105 => {
                q = parse_integer_value(&raw mut p, endptr, 10 as ::core::ffi::c_int);
                if q.is_null() {
                    pdf_warning!(b"Missing TTC index number...\0" as *const u8 as *const ::core::ffi::c_char,);
                    return -(1 as ::core::ffi::c_int);
                }
                (*mrec).opt.index = strtoul(q, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(), 10 as ::core::ffi::c_int) as uint32_t;
                free(q as *mut ::core::ffi::c_void);
            }
            112 => {
                q = parse_integer_value(&raw mut p, endptr, 0 as ::core::ffi::c_int);
                if q.is_null() {
                    pdf_warning!(b"Missing a number for 'p' option.\0" as *const u8 as *const ::core::ffi::c_char,);
                    return -(1 as ::core::ffi::c_int);
                }
                v = strtol(q, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(), 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                if v < 0 as ::core::ffi::c_int || v > 16 as ::core::ffi::c_int {
                    pdf_warning!(b"Invalid value for option 'p': %s\0" as *const u8 as *const ::core::ffi::c_char, q,);
                } else {
                    (*mrec).opt.mapc = v << 16 as ::core::ffi::c_int;
                }
                free(q as *mut ::core::ffi::c_void);
            }
            117 => {
                q = parse_string_value(&raw mut p, endptr);
                if !q.is_null() {
                    (*mrec).opt.tounicode = q;
                } else {
                    pdf_warning!(b"Missing string value for option 'u'.\0" as *const u8 as *const ::core::ffi::c_char,);
                    return -(1 as ::core::ffi::c_int);
                }
            }
            118 => {
                q = parse_integer_value(&raw mut p, endptr, 10 as ::core::ffi::c_int);
                if q.is_null() {
                    pdf_warning!(b"Missing a number for 'v' option.\0" as *const u8 as *const ::core::ffi::c_char,);
                    return -(1 as ::core::ffi::c_int);
                }
                (*mrec).opt.stemv = strtol(q, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(), 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                free(q as *mut ::core::ffi::c_void);
            }
            108 => {
                q = parse_string_value(&raw mut p, endptr);
                if !q.is_null() {
                    (*mrec).opt.otl_tags = q;
                } else {
                    pdf_warning!(b"Missing string value for option 'l'.\0" as *const u8 as *const ::core::ffi::c_char,);
                    return -(1 as ::core::ffi::c_int);
                }
            }
            109 => {
                if p.offset(4 as ::core::ffi::c_int as isize) <= endptr
                    && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '<' as i32
                    && *p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '>' as i32
                {
                    p = p.offset(1);
                    q = parse_integer_value(&raw mut p, endptr, 16 as ::core::ffi::c_int);
                    if q.is_null() {
                        pdf_warning!(b"Invalid value for option 'm'.\0" as *const u8 as *const ::core::ffi::c_char,);
                        return -(1 as ::core::ffi::c_int);
                    } else if p < endptr && *p as ::core::ffi::c_int != '>' as i32 {
                        pdf_warning!(b"Invalid value for option 'm': %s\0" as *const u8 as *const ::core::ffi::c_char, q,);
                        free(q as *mut ::core::ffi::c_void);
                        return -(1 as ::core::ffi::c_int);
                    }
                    v = strtol(q, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(), 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
                    (*mrec).opt.mapc = ((v << 8 as ::core::ffi::c_int) as ::core::ffi::c_long & 0xff00 as ::core::ffi::c_long) as ::core::ffi::c_int;
                    free(q as *mut ::core::ffi::c_void);
                    p = p.offset(1);
                } else if p.offset(4 as ::core::ffi::c_int as isize) <= endptr
                    && memcmp(
                        p as *const ::core::ffi::c_void,
                        b"sfd:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        strlen(b"sfd:\0" as *const u8 as *const ::core::ffi::c_char),
                    ) == 0
                {
                    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    let mut rr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
                    p = p.offset(4 as ::core::ffi::c_int as isize);
                    skip_blank(&raw mut p, endptr);
                    q = parse_string_value(&raw mut p, endptr);
                    if q.is_null() {
                        pdf_warning!(b"Missing value for option 'm'.\0" as *const u8 as *const ::core::ffi::c_char,);
                        return -(1 as ::core::ffi::c_int);
                    }
                    r = strchr(q, ',' as i32);
                    if r.is_null() {
                        pdf_warning!(b"Invalid value for option 'm': %s\0" as *const u8 as *const ::core::ffi::c_char, q,);
                        free(q as *mut ::core::ffi::c_void);
                        return -(1 as ::core::ffi::c_int);
                    }
                    *r = 0 as ::core::ffi::c_char;
                    r = r.offset(1);
                    rr = r;
                    skip_blank(&raw mut rr, r.offset(strlen(r) as isize));
                    if *rr as ::core::ffi::c_int == '\0' as i32 {
                        pdf_warning!(b"Invalid value for option 'm': %s,\0" as *const u8 as *const ::core::ffi::c_char, q,);
                        free(q as *mut ::core::ffi::c_void);
                        return -(1 as ::core::ffi::c_int);
                    }
                    (*mrec).charmap.sfd_name = mstrdup(q);
                    (*mrec).charmap.subfont_id = mstrdup(rr);
                    free(q as *mut ::core::ffi::c_void);
                } else if p.offset(4 as ::core::ffi::c_int as isize) < endptr
                    && memcmp(
                        p as *const ::core::ffi::c_void,
                        b"pad:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        strlen(b"pad:\0" as *const u8 as *const ::core::ffi::c_char),
                    ) == 0
                {
                    p = p.offset(4 as ::core::ffi::c_int as isize);
                    skip_blank(&raw mut p, endptr);
                    q = parse_integer_value(&raw mut p, endptr, 16 as ::core::ffi::c_int);
                    if q.is_null() {
                        pdf_warning!(b"Invalid value for option 'm'.\0" as *const u8 as *const ::core::ffi::c_char,);
                        return -(1 as ::core::ffi::c_int);
                    } else if p < endptr && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
                        pdf_warning!(b"Invalid value for option 'm': %s\0" as *const u8 as *const ::core::ffi::c_char, q,);
                        free(q as *mut ::core::ffi::c_void);
                        return -(1 as ::core::ffi::c_int);
                    }
                    v = strtol(q, ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(), 16 as ::core::ffi::c_int) as ::core::ffi::c_int;
                    (*mrec).opt.mapc = ((v << 8 as ::core::ffi::c_int) as ::core::ffi::c_long & 0xff00 as ::core::ffi::c_long) as ::core::ffi::c_int;
                    free(q as *mut ::core::ffi::c_void);
                } else {
                    pdf_warning!(b"Invalid value for option 'm'.\0" as *const u8 as *const ::core::ffi::c_char,);
                    return -(1 as ::core::ffi::c_int);
                }
            }
            119 => {
                if (*mrec).enc_name.is_null() || strcmp((*mrec).enc_name, b"unicode\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
                    pdf_warning!(b"Fontmap option 'w' meaningless for encoding other than \"unicode\".\0" as *const u8 as *const ::core::ffi::c_char,);
                    return -(1 as ::core::ffi::c_int);
                }
                q = parse_integer_value(&raw mut p, endptr, 10 as ::core::ffi::c_int);
                if q.is_null() {
                    pdf_warning!(b"Missing wmode value...\0" as *const u8 as *const ::core::ffi::c_char,);
                    return -(1 as ::core::ffi::c_int);
                }
                if atoi(q) == 1 as ::core::ffi::c_int {
                    (*mrec).opt.flags |= FONTMAP_OPT_VERT;
                } else if atoi(q) == 0 as ::core::ffi::c_int {
                    (*mrec).opt.flags &= !FONTMAP_OPT_VERT;
                } else {
                    pdf_warning!(b"Invalid value for option 'w': %s\0" as *const u8 as *const ::core::ffi::c_char, q,);
                }
                free(q as *mut ::core::ffi::c_void);
            }
            _ => {
                pdf_warning!(
                    b"Unrecognized font map option: '%c'\0" as *const u8 as *const ::core::ffi::c_char,
                    mopt as ::core::ffi::c_int,
                );
                return -(1 as ::core::ffi::c_int);
            }
        }
        skip_blank(&raw mut p, endptr);
    }
    if p < endptr && *p as ::core::ffi::c_int != '\r' as i32 && *p as ::core::ffi::c_int != '\n' as i32 {
        pdf_warning!(
            b"Invalid char in fontmap line: %c\0" as *const u8 as *const ::core::ffi::c_char,
            *p as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn fontmap_parse_mapdef_dps(
    mut mrec: *mut fontmap_rec, mut mapdef: *const ::core::ffi::c_char, mut endptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = mapdef;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    skip_blank(&raw mut p, endptr);
    if *p as ::core::ffi::c_int != '"' as i32 && *p as ::core::ffi::c_int != '<' as i32 {
        if p < endptr {
            q = parse_string_value(&raw mut p, endptr);
            free(q as *mut ::core::ffi::c_void);
            skip_blank(&raw mut p, endptr);
        } else {
            pdf_warning!(b"Missing a PostScript font name.\0" as *const u8 as *const ::core::ffi::c_char,);
            return -(1 as ::core::ffi::c_int);
        }
    }
    if p >= endptr {
        return 0 as ::core::ffi::c_int;
    }
    while p < endptr
        && *p as ::core::ffi::c_int != '\r' as i32
        && *p as ::core::ffi::c_int != '\n' as i32
        && (*p as ::core::ffi::c_int == '<' as i32 || *p as ::core::ffi::c_int == '"' as i32)
    {
        match *p as ::core::ffi::c_int {
            60 => {
                p = p.offset(1);
                if p < endptr && (*p as ::core::ffi::c_int == '[' as i32 || *p as ::core::ffi::c_int == '<' as i32) {
                    p = p.offset(1);
                }
                skip_blank(&raw mut p, endptr);
                q = parse_string_value(&raw mut p, endptr);
                if !q.is_null() {
                    let mut n: ::core::ffi::c_int = strlen(q) as ::core::ffi::c_int;
                    if n > 4 as ::core::ffi::c_int
                        && !strstartswith(
                            q.offset(n as isize).offset(-(4 as ::core::ffi::c_int as isize)),
                            b".enc\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                        .is_null()
                    {
                        (*mrec).enc_name = q;
                    } else {
                        (*mrec).font_name = q;
                    }
                }
                skip_blank(&raw mut p, endptr);
            }
            34 => {
                q = parse_string_value(&raw mut p, endptr);
                if !q.is_null() {
                    let mut r: *const ::core::ffi::c_char = q;
                    let mut e: *const ::core::ffi::c_char = q.offset(strlen(q) as isize);
                    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    skip_blank(&raw mut r, e);
                    while r < e {
                        s = parse_float_decimal(&raw mut r, e);
                        if !s.is_null() {
                            skip_blank(&raw mut r, e);
                            t = parse_string_value(&raw mut r, e);
                            if !t.is_null() {
                                if streq_ptr(t, b"SlantFont\0" as *const u8 as *const ::core::ffi::c_char) {
                                    (*mrec).opt.slant = atof(s);
                                } else if streq_ptr(t, b"ExtendFont\0" as *const u8 as *const ::core::ffi::c_char) {
                                    (*mrec).opt.extend = atof(s);
                                }
                                free(t as *mut ::core::ffi::c_void);
                            }
                            free(s as *mut ::core::ffi::c_void);
                        } else {
                            s = parse_string_value(&raw mut r, e);
                            if !s.is_null() {
                                free(s as *mut ::core::ffi::c_void);
                            }
                        }
                        skip_blank(&raw mut r, e);
                    }
                    free(q as *mut ::core::ffi::c_void);
                }
                skip_blank(&raw mut p, endptr);
            }
            _ => {
                pdf_warning!(b"Found an invalid entry: %s\0" as *const u8 as *const ::core::ffi::c_char, p,);
                return -(1 as ::core::ffi::c_int);
            }
        }
        skip_blank(&raw mut p, endptr);
    }
    if p < endptr && *p as ::core::ffi::c_int != '\r' as i32 && *p as ::core::ffi::c_int != '\n' as i32 {
        pdf_warning!(
            b"Invalid char in fontmap line: %c\0" as *const u8 as *const ::core::ffi::c_char,
            *p as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn chop_sfd_name(
    mut tex_name: *const ::core::ffi::c_char, mut sfd_name: *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut m: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    *sfd_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = strchr(tex_name, '@' as i32);
    if p.is_null() || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 || p == tex_name as *mut ::core::ffi::c_char {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    m = p.offset_from(tex_name) as ::core::ffi::c_long as ::core::ffi::c_int;
    p = p.offset(1);
    q = strchr(p, '@' as i32);
    if q.is_null() || q == p {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    n = q.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int;
    q = q.offset(1);
    len = strlen(tex_name).wrapping_sub(n as size_t) as ::core::ffi::c_int;
    fontname =
        new(((len + 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
            as *mut ::core::ffi::c_char;
    memcpy(fontname as *mut ::core::ffi::c_void, tex_name as *const ::core::ffi::c_void, m as size_t);
    *fontname.offset(m as isize) = '\0' as i32 as ::core::ffi::c_char;
    if *q != 0 {
        strcat(fontname, q);
    }
    *sfd_name =
        new(((n + 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
            as *mut ::core::ffi::c_char;
    memcpy(*sfd_name as *mut ::core::ffi::c_void, p as *const ::core::ffi::c_void, n as size_t);
    *(*sfd_name).offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    return fontname;
}
unsafe extern "C-unwind" fn make_subfont_name(
    mut map_name: *const ::core::ffi::c_char, mut sfd_name: *const ::core::ffi::c_char, mut sub_id: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut tfm_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = strchr(map_name, '@' as i32);
    if p.is_null() || p == map_name as *mut ::core::ffi::c_char {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    m = p.offset_from(map_name) as ::core::ffi::c_long as ::core::ffi::c_int;
    q = strchr(p.offset(1 as ::core::ffi::c_int as isize), '@' as i32);
    if q.is_null() || q == p.offset(1 as ::core::ffi::c_int as isize) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    n = q.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    if strlen(sfd_name) != (n - 2 as ::core::ffi::c_int) as size_t
        || memcmp(
            p.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            sfd_name as *const ::core::ffi::c_void,
            (n - 2 as ::core::ffi::c_int) as size_t,
        ) != 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    tfm_name = new((strlen(map_name)
        .wrapping_sub(n as size_t)
        .wrapping_add(strlen(sub_id))
        .wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    memcpy(tfm_name as *mut ::core::ffi::c_void, map_name as *const ::core::ffi::c_void, m as size_t);
    *tfm_name.offset(m as isize) = '\0' as i32 as ::core::ffi::c_char;
    strcat(tfm_name, sub_id);
    if *q.offset(1 as ::core::ffi::c_int as isize) != 0 {
        strcat(tfm_name, q.offset(1 as ::core::ffi::c_int as isize));
    }
    return tfm_name;
}
#[unsafe(export_name = "__pdf_impl_pdf_append_fontmap_record")]
pub unsafe extern "C-unwind" fn pdf_append_fontmap_record(mut kp: *const ::core::ffi::c_char, mut vp: *const fontmap_rec) -> ::core::ffi::c_int {
    let mut mrec: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    let mut fnt_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sfd_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if kp.is_null() || (vp.is_null() || (*vp).map_name.is_null() || (*vp).font_name.is_null()) {
        pdf_warning!(b"Invalid fontmap record...\0" as *const u8 as *const ::core::ffi::c_char,);
        return -(1 as ::core::ffi::c_int);
    }
    if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
        pdf_message!(b"fontmap>> append key=\"%s\"...\0" as *const u8 as *const ::core::ffi::c_char, kp,);
    }
    fnt_name = chop_sfd_name(kp, &raw mut sfd_name);
    if !fnt_name.is_null() && !sfd_name.is_null() {
        let mut tfm_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut subfont_ids: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        subfont_ids = sfd_get_subfont_ids(sfd_name, &raw mut n);
        if subfont_ids.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        loop {
            let fresh1 = n;
            n = n - 1;
            if !(fresh1 > 0 as ::core::ffi::c_int) {
                break;
            }
            tfm_name = make_subfont_name(kp, sfd_name, *subfont_ids.offset(n as isize));
            if tfm_name.is_null() {
                continue;
            }
            mrec = tt_fontmap_lookup(tfm_name);
            if mrec.is_null() {
                mrec = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<fontmap_rec>() as usize) as uint32_t)
                    as *mut fontmap_rec;
                pdf_init_fontmap_record(mrec);
                (*mrec).map_name = mstrdup(kp);
                (*mrec).charmap.sfd_name = mstrdup(sfd_name);
                (*mrec).charmap.subfont_id = mstrdup(*subfont_ids.offset(n as isize));
                tt_fontmap_insert(tfm_name, mrec, 0 as ::core::ffi::c_int);
                pdf_clear_fontmap_record(mrec);
                free(mrec as *mut ::core::ffi::c_void);
            }
            free(tfm_name as *mut ::core::ffi::c_void);
        }
        free(fnt_name as *mut ::core::ffi::c_void);
        free(sfd_name as *mut ::core::ffi::c_void);
    }
    tt_fontmap_insert(kp, vp, 0 as ::core::ffi::c_int);
    if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
        pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_remove_fontmap_record")]
pub unsafe extern "C-unwind" fn pdf_remove_fontmap_record(mut kp: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut fnt_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sfd_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if kp.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
        pdf_message!(b"fontmap>> remove key=\"%s\"...\0" as *const u8 as *const ::core::ffi::c_char, kp,);
    }
    fnt_name = chop_sfd_name(kp, &raw mut sfd_name);
    if !fnt_name.is_null() && !sfd_name.is_null() {
        let mut tfm_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut subfont_ids: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        subfont_ids = sfd_get_subfont_ids(sfd_name, &raw mut n);
        if subfont_ids.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
            pdf_message!(b"\nfontmap>> Expand @%s@:\0" as *const u8 as *const ::core::ffi::c_char, sfd_name,);
        }
        loop {
            let fresh0 = n;
            n = n - 1;
            if !(fresh0 > 0 as ::core::ffi::c_int) {
                break;
            }
            tfm_name = make_subfont_name(kp, sfd_name, *subfont_ids.offset(n as isize));
            if tfm_name.is_null() {
                continue;
            }
            if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
                pdf_message!(b" %s\0" as *const u8 as *const ::core::ffi::c_char, tfm_name,);
            }
            tt_fontmap_remove(tfm_name);
            free(tfm_name as *mut ::core::ffi::c_void);
        }
        free(fnt_name as *mut ::core::ffi::c_void);
        free(sfd_name as *mut ::core::ffi::c_void);
    }
    tt_fontmap_remove(kp);
    if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
        pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_insert_fontmap_record")]
pub unsafe extern "C-unwind" fn pdf_insert_fontmap_record(mut kp: *const ::core::ffi::c_char, mut vp: *const fontmap_rec) -> *mut fontmap_rec {
    let mut mrec: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    let mut fnt_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sfd_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if kp.is_null() || (vp.is_null() || (*vp).map_name.is_null() || (*vp).font_name.is_null()) {
        pdf_warning!(b"Invalid fontmap record...\0" as *const u8 as *const ::core::ffi::c_char,);
        return ::core::ptr::null_mut::<fontmap_rec>();
    }
    if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
        pdf_message!(b"fontmap>> insert key=\"%s\"...\0" as *const u8 as *const ::core::ffi::c_char, kp,);
    }
    fnt_name = chop_sfd_name(kp, &raw mut sfd_name);
    if !fnt_name.is_null() && !sfd_name.is_null() {
        let mut tfm_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut subfont_ids: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        subfont_ids = sfd_get_subfont_ids(sfd_name, &raw mut n);
        if subfont_ids.is_null() {
            pdf_warning!(b"Could not open SFD file: %s\0" as *const u8 as *const ::core::ffi::c_char, sfd_name,);
            free(fnt_name as *mut ::core::ffi::c_void);
            free(sfd_name as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<fontmap_rec>();
        }
        if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
            pdf_message!(b"\nfontmap>> Expand @%s@:\0" as *const u8 as *const ::core::ffi::c_char, sfd_name,);
        }
        loop {
            let fresh2 = n;
            n = n - 1;
            if !(fresh2 > 0 as ::core::ffi::c_int) {
                break;
            }
            tfm_name = make_subfont_name(kp, sfd_name, *subfont_ids.offset(n as isize));
            if tfm_name.is_null() {
                continue;
            }
            if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
                pdf_message!(b" %s\0" as *const u8 as *const ::core::ffi::c_char, tfm_name,);
            }
            mrec = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<fontmap_rec>() as usize) as uint32_t)
                as *mut fontmap_rec;
            pdf_init_fontmap_record(mrec);
            (*mrec).map_name = mstrdup(kp);
            (*mrec).charmap.sfd_name = mstrdup(sfd_name);
            (*mrec).charmap.subfont_id = mstrdup(*subfont_ids.offset(n as isize));
            tt_fontmap_insert(tfm_name, mrec, 1 as ::core::ffi::c_int);
            pdf_clear_fontmap_record(mrec);
            free(mrec as *mut ::core::ffi::c_void);
            free(tfm_name as *mut ::core::ffi::c_void);
        }
        free(fnt_name as *mut ::core::ffi::c_void);
        free(sfd_name as *mut ::core::ffi::c_void);
    }
    mrec = tt_fontmap_insert(kp, vp, 1 as ::core::ffi::c_int);
    if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
        pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return mrec;
}
#[unsafe(export_name = "__pdf_impl_pdf_read_fontmap_line")]
pub unsafe extern "C-unwind" fn pdf_read_fontmap_line(
    mut mrec: *mut fontmap_rec, mut mline: *const ::core::ffi::c_char, mut mline_len: ::core::ffi::c_int, mut format: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if mrec.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"mrec\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    p = mline;
    endptr = p.offset(mline_len as isize);
    skip_blank(&raw mut p, endptr);
    if p >= endptr {
        return -(1 as ::core::ffi::c_int);
    }
    q = parse_string_value(&raw mut p, endptr);
    if q.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if format > 0 as ::core::ffi::c_int {
        error = fontmap_parse_mapdef_dpm(mrec, p, endptr);
    } else {
        error = fontmap_parse_mapdef_dps(mrec, p, endptr);
    }
    if error == 0 {
        let mut fnt_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut sfd_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        fnt_name = chop_sfd_name(q, &raw mut sfd_name);
        if !fnt_name.is_null() && !sfd_name.is_null() {
            if (*mrec).font_name.is_null() {
                (*mrec).font_name = fnt_name;
            } else {
                free(fnt_name as *mut ::core::ffi::c_void);
            }
            free((*mrec).charmap.sfd_name as *mut ::core::ffi::c_void);
            (*mrec).charmap.sfd_name = sfd_name;
        }
        fill_in_defaults(mrec, q);
    }
    free(q as *mut ::core::ffi::c_void);
    return error;
}
#[unsafe(export_name = "__pdf_impl_is_pdfm_mapline")]
pub unsafe extern "C-unwind" fn is_pdfm_mapline(mut mline: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !strchr(mline, '"' as i32).is_null() || !strchr(mline, '<' as i32).is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    p = mline;
    endptr = p.offset(strlen(mline) as isize);
    skip_blank(&raw mut p, endptr);
    while p < endptr {
        if *p as ::core::ffi::c_int == '-' as i32 {
            return 1 as ::core::ffi::c_int;
        }
        n = n.wrapping_add(1);
        while p < endptr
            && !(crate::backend::runtime::is_ascii(*p as ::core::ffi::c_int) != 0
                && (*p as ::core::ffi::c_uchar as ::core::ffi::c_int == ' ' as i32
                    || *p as ::core::ffi::c_uchar as ::core::ffi::c_int == '\t' as i32))
        {
            p = p.offset(1);
        }
        skip_blank(&raw mut p, endptr);
    }
    return if n == 2 as ::core::ffi::c_uint {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_load_fontmap_file")]
pub unsafe extern "C-unwind" fn pdf_load_fontmap_file(mut filename: *const ::core::ffi::c_char, mut mode: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut mrec: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    let mut handle: rust_input_handle_t = 0;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut llen: ::core::ffi::c_int = 0;
    let mut lpos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut next: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut format: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if filename.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"filename\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if dpx_conf.verbose_level != 0 {
        pdf_message!(b"<FONTMAP:\0" as *const u8 as *const ::core::ffi::c_char);
    }
    handle = dpx_tt_open(filename, b".map\0" as *const u8 as *const ::core::ffi::c_char, TTBC_FILE_FORMAT_FONT_MAP);
    if handle == INVALID_HANDLE {
        pdf_warning!(
            b"Couldn't open font map file \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if tt_fontmap_load_begin(handle, mode, dpx_conf.verbose_level) < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Error reading font map file \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
    }
    while error == 0 && {
        next = tt_fontmap_load_next(&raw mut p, &raw mut lpos, &raw mut format);
        next != 0 as ::core::ffi::c_int
    } {
        llen = strlen(p) as ::core::ffi::c_int;
        endptr = p.offset(llen as isize);
        skip_blank(&raw mut p, endptr);
        if p == endptr {
            continue;
        }
        if next == 2 as ::core::ffi::c_int {
            pdf_warning!(
                b"Found a mismatched fontmap line %d from %s.\0" as *const u8 as *const ::core::ffi::c_char,
                lpos,
                filename,
            );
            pdf_warning!(b"-- Ignore the current input buffer: %s\0" as *const u8 as *const ::core::ffi::c_char, p,);
        } else {
            mrec = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<fontmap_rec>() as usize) as uint32_t)
                as *mut fontmap_rec;
            pdf_init_fontmap_record(mrec);
            error = pdf_read_fontmap_line(mrec, p, endptr.offset_from(p) as ::core::ffi::c_long as ::core::ffi::c_int, format);
            if error != 0 {
                pdf_warning!(
                    b"Invalid map record in fontmap line %d from %s.\0" as *const u8 as *const ::core::ffi::c_char,
                    lpos,
                    filename,
                );
                pdf_warning!(b"-- Ignore the current input buffer: %s\0" as *const u8 as *const ::core::ffi::c_char, p,);
                pdf_clear_fontmap_record(mrec);
                free(mrec as *mut ::core::ffi::c_void);
            } else {
                match mode {
                    FONTMAP_RMODE_REPLACE => {
                        pdf_insert_fontmap_record((*mrec).map_name, mrec);
                    }
                    FONTMAP_RMODE_APPEND => {
                        pdf_append_fontmap_record((*mrec).map_name, mrec);
                    }
                    FONTMAP_RMODE_REMOVE => {
                        pdf_remove_fontmap_record((*mrec).map_name);
                    }
                    _ => {}
                }
                pdf_clear_fontmap_record(mrec);
                free(mrec as *mut ::core::ffi::c_void);
            }
        }
    }
    tt_fontmap_load_end();
    crate::backend::runtime::input_close(handle);
    if dpx_conf.verbose_level != 0 {
        pdf_message!(b">\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return error;
}
#[unsafe(export_name = "__pdf_impl_pdf_insert_native_fontmap_record")]
pub unsafe extern "C-unwind" fn pdf_insert_native_fontmap_record(
    mut path: *const ::core::ffi::c_char, mut index: uint32_t, mut layout_dir: ::core::ffi::c_int, mut extend: ::core::ffi::c_int,
    mut slant: ::core::ffi::c_int, mut embolden: ::core::ffi::c_int,
) -> *mut fontmap_rec {
    let mut fontmap_key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mrec: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    let mut ret: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    if path.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"path\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    fontmap_key = crate::backend::runtime::allocate(strlen(path).wrapping_add(40 as size_t)) as *mut ::core::ffi::c_char;
    sprintf(
        fontmap_key,
        b"%s/%d/%c/%d/%d/%d\0" as *const u8 as *const ::core::ffi::c_char,
        path,
        index,
        if layout_dir == 0 as ::core::ffi::c_int { 'H' as i32 } else { 'V' as i32 },
        extend,
        slant,
        embolden,
    );
    if dpx_conf.verbose_level != 0 {
        pdf_message!(b"<NATIVE-FONTMAP:%s\0" as *const u8 as *const ::core::ffi::c_char, fontmap_key,);
    }
    mrec = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<fontmap_rec>() as usize) as uint32_t)
        as *mut fontmap_rec;
    pdf_init_fontmap_record(mrec);
    (*mrec).map_name = fontmap_key;
    (*mrec).enc_name = mstrdup(if layout_dir == 0 as ::core::ffi::c_int {
        b"Identity-H\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"Identity-V\0" as *const u8 as *const ::core::ffi::c_char
    });
    (*mrec).font_name = mstrdup(path);
    (*mrec).opt.index = index;
    if layout_dir != 0 as ::core::ffi::c_int {
        (*mrec).opt.flags |= FONTMAP_OPT_VERT;
    }
    fill_in_defaults(mrec, fontmap_key);
    free(fontmap_key as *mut ::core::ffi::c_void);
    (*mrec).opt.extend = extend as ::core::ffi::c_double / 65536.0f64;
    (*mrec).opt.slant = slant as ::core::ffi::c_double / 65536.0f64;
    (*mrec).opt.bold = embolden as ::core::ffi::c_double / 65536.0f64;
    (*mrec).opt.use_glyph_encoding = 1 as ::core::ffi::c_int;
    ret = pdf_insert_fontmap_record((*mrec).map_name, mrec);
    pdf_clear_fontmap_record(mrec);
    free(mrec as *mut ::core::ffi::c_void);
    if dpx_conf.verbose_level != 0 {
        pdf_message!(b">\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return ret;
}
#[unsafe(export_name = "__pdf_impl_pdf_lookup_fontmap_record")]
pub unsafe extern "C-unwind" fn pdf_lookup_fontmap_record(mut tfm_name: *const ::core::ffi::c_char) -> *mut fontmap_rec {
    return tt_fontmap_lookup(tfm_name);
}
#[unsafe(export_name = "__pdf_impl_pdf_init_fontmaps")]
pub unsafe extern "C-unwind" fn pdf_init_fontmaps() {
    tt_fontmap_reset();
}
#[unsafe(export_name = "__pdf_impl_pdf_close_fontmaps")]
pub unsafe extern "C-unwind" fn pdf_close_fontmaps() {
    tt_fontmap_clear();
    release_sfd_record();
}
unsafe extern "C-unwind" fn substr(mut str: *mut *const ::core::ffi::c_char, mut stop: ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut sstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    endptr = strchr(*str, stop as ::core::ffi::c_int);
    if endptr.is_null() || endptr == *str {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    sstr = new(
        ((endptr.offset_from(*str) as ::core::ffi::c_long + 1 as ::core::ffi::c_long) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
    ) as *mut ::core::ffi::c_char;
    memcpy(
        sstr as *mut ::core::ffi::c_void,
        *str as *const ::core::ffi::c_void,
        endptr.offset_from(*str) as ::core::ffi::c_long as size_t,
    );
    *sstr.offset(endptr.offset_from(*str) as ::core::ffi::c_long as isize) = '\0' as i32 as ::core::ffi::c_char;
    *str = endptr.offset(1 as ::core::ffi::c_int as isize);
    return sstr;
}
pub const CID_MAPREC_CSI_DELIM: ::core::ffi::c_int = '/' as i32;
unsafe extern "C-unwind" fn strip_options(mut map_name: *const ::core::ffi::c_char, mut opt: *mut fontmap_opt) -> *mut ::core::ffi::c_char {
    let mut font_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut next: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut have_csi: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut have_style: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if opt.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"opt\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    p = map_name;
    font_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*opt).charcoll = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*opt).index = 0 as uint32_t;
    (*opt).style = FONTMAP_STYLE_NONE;
    (*opt).flags = 0 as ::core::ffi::c_int;
    if *p as ::core::ffi::c_int == ':' as i32
        && isdigit(*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
    {
        (*opt).index = strtoul(p.offset(1 as ::core::ffi::c_int as isize), &raw mut next, 10 as ::core::ffi::c_int) as uint32_t;
        if *next as ::core::ffi::c_int == ':' as i32 {
            p = next.offset(1 as ::core::ffi::c_int as isize);
        } else {
            (*opt).index = 0 as uint32_t;
        }
    }
    if *p as ::core::ffi::c_int == '!' as i32 {
        p = p.offset(1);
        if *p as ::core::ffi::c_int == '\0' as i32 {
            pdf_abort!(
                b"Invalid map record: %s (--> %s)\0" as *const u8 as *const ::core::ffi::c_char,
                map_name,
                p,
            );
        }
        (*opt).flags |= FONTMAP_OPT_NOEMBED;
    }
    next = strchr(p, CID_MAPREC_CSI_DELIM);
    if !next.is_null() {
        if next == p as *mut ::core::ffi::c_char {
            pdf_abort!(
                b"Invalid map record: %s (--> %s)\0" as *const u8 as *const ::core::ffi::c_char,
                map_name,
                p,
            );
        }
        font_name = substr(&raw mut p, CID_MAPREC_CSI_DELIM as ::core::ffi::c_char);
        have_csi = 1 as ::core::ffi::c_int;
    } else {
        next = strchr(p, ',' as i32);
        if !next.is_null() {
            if next == p as *mut ::core::ffi::c_char {
                pdf_abort!(
                    b"Invalid map record: %s (--> %s)\0" as *const u8 as *const ::core::ffi::c_char,
                    map_name,
                    p,
                );
            }
            font_name = substr(&raw mut p, ',' as i32 as ::core::ffi::c_char);
            have_style = 1 as ::core::ffi::c_int;
        } else {
            font_name = new((strlen(p).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
                as *mut ::core::ffi::c_char;
            strcpy(font_name, p);
        }
    }
    if have_csi != 0 {
        next = strchr(p, ',' as i32);
        if !next.is_null() {
            (*opt).charcoll = substr(&raw mut p, ',' as i32 as ::core::ffi::c_char);
            have_style = 1 as ::core::ffi::c_int;
        } else if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
            pdf_abort!(b"Invalid map record: %s.\0" as *const u8 as *const ::core::ffi::c_char, map_name,);
        } else {
            (*opt).charcoll = new((strlen(p).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
                as *mut ::core::ffi::c_char;
            strcpy((*opt).charcoll, p);
        }
    }
    if have_style != 0 {
        if !strstartswith(p, b"BoldItalic\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            if *p.offset(10 as ::core::ffi::c_int as isize) != 0 {
                pdf_abort!(
                    b"Invalid map record: %s (--> %s)\0" as *const u8 as *const ::core::ffi::c_char,
                    map_name,
                    p,
                );
            }
            (*opt).style = FONTMAP_STYLE_BOLDITALIC;
        } else if !strstartswith(p, b"Bold\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            if *p.offset(4 as ::core::ffi::c_int as isize) != 0 {
                pdf_abort!(
                    b"Invalid map record: %s (--> %s)\0" as *const u8 as *const ::core::ffi::c_char,
                    map_name,
                    p,
                );
            }
            (*opt).style = FONTMAP_STYLE_BOLD;
        } else if !strstartswith(p, b"Italic\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            if *p.offset(6 as ::core::ffi::c_int as isize) != 0 {
                pdf_abort!(
                    b"Invalid map record: %s (--> %s)\0" as *const u8 as *const ::core::ffi::c_char,
                    map_name,
                    p,
                );
            }
            (*opt).style = FONTMAP_STYLE_ITALIC;
        }
    }
    return font_name;
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn memcmp(__s1: *const ::core::ffi::c_void, __s2: *const ::core::ffi::c_void, __n: size_t) -> ::core::ffi::c_int;
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_init_fontmap_record(mut mrec: *mut fontmap_rec) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_init_fontmap_record(mrec) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_clear_fontmap_record(mut mrec: *mut fontmap_rec) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_clear_fontmap_record(mrec) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_append_fontmap_record(
    mut kp: *const ::core::ffi::c_char, mut vp: *const fontmap_rec, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_append_fontmap_record(kp, vp) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_remove_fontmap_record(mut kp: *const ::core::ffi::c_char, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_remove_fontmap_record(kp) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_insert_fontmap_record(
    mut kp: *const ::core::ffi::c_char, mut vp: *const fontmap_rec, _result: *mut *mut fontmap_rec,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_insert_fontmap_record(kp, vp) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_read_fontmap_line(
    mut mrec: *mut fontmap_rec, mut mline: *const ::core::ffi::c_char, mut mline_len: ::core::ffi::c_int, mut format: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_read_fontmap_line(mrec, mline, mline_len, format) }) {
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
pub unsafe extern "C" fn __pdf_entry_is_pdfm_mapline(mut mline: *const ::core::ffi::c_char, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { is_pdfm_mapline(mline) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_load_fontmap_file(
    mut filename: *const ::core::ffi::c_char, mut mode: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_load_fontmap_file(filename, mode) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_insert_native_fontmap_record(
    mut path: *const ::core::ffi::c_char, mut index: uint32_t, mut layout_dir: ::core::ffi::c_int, mut extend: ::core::ffi::c_int,
    mut slant: ::core::ffi::c_int, mut embolden: ::core::ffi::c_int, _result: *mut *mut fontmap_rec,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_insert_native_fontmap_record(path, index, layout_dir, extend, slant, embolden) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_lookup_fontmap_record(mut tfm_name: *const ::core::ffi::c_char, _result: *mut *mut fontmap_rec) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_lookup_fontmap_record(tfm_name) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_init_fontmaps() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_init_fontmaps() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_close_fontmaps() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_close_fontmaps() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
