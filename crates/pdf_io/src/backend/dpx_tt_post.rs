/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2019 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn xstrdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> ssize_t;
    #[link_name = "__pdf_impl_tt_get_unsigned_byte"]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_tt_get_signed_pair"]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> ::core::ffi::c_short;
    #[link_name = "__pdf_impl_tt_get_unsigned_quad"]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    #[link_name = "__pdf_impl_sfnt_locate_table"]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
pub type rust_input_handle_t = uintptr_t;
pub type BYTE = ::core::ffi::c_uchar;
pub type USHORT = ::core::ffi::c_ushort;
pub type SHORT = ::core::ffi::c_short;
pub type SFNT_ULONG = uint32_t;
pub type Fixed = uint32_t;
pub type FWord = ::core::ffi::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [::core::ffi::c_char; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: SFNT_ULONG,
    pub num_tables: USHORT,
    pub search_range: USHORT,
    pub entry_selector: USHORT,
    pub range_shift: USHORT,
    pub num_kept_tables: USHORT,
    pub flags: *mut ::core::ffi::c_char,
    pub tables: *mut sfnt_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: ::core::ffi::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_post_table {
    pub Version: Fixed,
    pub italicAngle: Fixed,
    pub underlinePosition: FWord,
    pub underlineThickness: FWord,
    pub isFixedPitch: SFNT_ULONG,
    pub minMemType42: SFNT_ULONG,
    pub maxMemType42: SFNT_ULONG,
    pub minMemType1: SFNT_ULONG,
    pub maxMemType1: SFNT_ULONG,
    pub numberOfGlyphs: USHORT,
    pub glyphNamePtr: *mut *const ::core::ffi::c_char,
    pub names: *mut *mut ::core::ffi::c_char,
    pub count: USHORT,
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
unsafe extern "C-unwind" fn read_v2_post_names(
    mut post: *mut tt_post_table,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut i: USHORT = 0;
    let mut idx: USHORT = 0;
    let mut indices: *mut USHORT = ::core::ptr::null_mut::<USHORT>();
    let mut maxidx: USHORT = 0;
    let mut len: ::core::ffi::c_int = 0;
    (*post).numberOfGlyphs = tt_get_unsigned_pair((*sfont).handle);
    indices = new(((*post).numberOfGlyphs as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<USHORT>() as usize) as uint32_t)
        as *mut USHORT;
    maxidx = 257 as USHORT;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < (*post).numberOfGlyphs as ::core::ffi::c_int {
        idx = tt_get_unsigned_pair((*sfont).handle);
        if idx as ::core::ffi::c_int >= 258 as ::core::ffi::c_int {
            if idx as ::core::ffi::c_int > maxidx as ::core::ffi::c_int {
                maxidx = idx;
            }
        }
        *indices.offset(i as isize) = idx;
        i = i.wrapping_add(1);
    }
    (*post).count = (maxidx as ::core::ffi::c_int - 257 as ::core::ffi::c_int) as USHORT;
    if ((*post).count as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        (*post).names = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    } else {
        (*post).names = new(((*post).count as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
            as uint32_t) as *mut *mut ::core::ffi::c_char;
        i = 0 as USHORT;
        while (i as ::core::ffi::c_int) < (*post).count as ::core::ffi::c_int {
            len = tt_get_unsigned_byte((*sfont).handle) as ::core::ffi::c_int;
            if len > 0 as ::core::ffi::c_int {
                let ref mut fresh0 = *(*post).names.offset(i as isize);
                *fresh0 = new(((len + 1 as ::core::ffi::c_int) as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                    as uint32_t) as *mut ::core::ffi::c_char;
                ttstub_input_read(
                    (*sfont).handle,
                    *(*post).names.offset(i as isize),
                    len as size_t,
                );
                *(*(*post).names.offset(i as isize)).offset(len as isize) =
                    0 as ::core::ffi::c_char;
            } else {
                let ref mut fresh1 = *(*post).names.offset(i as isize);
                *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            i = i.wrapping_add(1);
        }
    }
    (*post).glyphNamePtr = new(((*post).numberOfGlyphs as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
        as uint32_t) as *mut *const ::core::ffi::c_char;
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < (*post).numberOfGlyphs as ::core::ffi::c_int {
        idx = *indices.offset(i as isize);
        if (idx as ::core::ffi::c_int) < 258 as ::core::ffi::c_int {
            let ref mut fresh2 = *(*post).glyphNamePtr.offset(i as isize);
            *fresh2 = macglyphorder[idx as usize];
        } else if (idx as ::core::ffi::c_int - 258 as ::core::ffi::c_int)
            < (*post).count as ::core::ffi::c_int
        {
            let ref mut fresh3 = *(*post).glyphNamePtr.offset(i as isize);
            *fresh3 = *(*post)
                .names
                .offset((idx as ::core::ffi::c_int - 258 as ::core::ffi::c_int) as isize);
        } else {
            pdf_warning!(
                b"Invalid glyph name index number: %u (>= %u)\0" as *const u8
                    as *const ::core::ffi::c_char,
                idx as ::core::ffi::c_int,
                (*post).count as ::core::ffi::c_int + 258 as ::core::ffi::c_int,
            );
            free(indices as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        i = i.wrapping_add(1);
    }
    free(indices as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_tt_read_post_table")]
pub unsafe extern "C-unwind" fn tt_read_post_table(mut sfont: *mut sfnt) -> *mut tt_post_table {
    let mut post: *mut tt_post_table = ::core::ptr::null_mut::<tt_post_table>();
    sfnt_locate_table(sfont, b"post\0" as *const u8 as *const ::core::ffi::c_char);
    post = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<tt_post_table>() as usize) as uint32_t)
        as *mut tt_post_table;
    (*post).Version = tt_get_unsigned_quad((*sfont).handle) as Fixed;
    (*post).italicAngle = tt_get_unsigned_quad((*sfont).handle) as Fixed;
    (*post).underlinePosition = tt_get_signed_pair((*sfont).handle) as FWord;
    (*post).underlineThickness = tt_get_signed_pair((*sfont).handle) as FWord;
    (*post).isFixedPitch = tt_get_unsigned_quad((*sfont).handle);
    (*post).minMemType42 = tt_get_unsigned_quad((*sfont).handle);
    (*post).maxMemType42 = tt_get_unsigned_quad((*sfont).handle);
    (*post).minMemType1 = tt_get_unsigned_quad((*sfont).handle);
    (*post).maxMemType1 = tt_get_unsigned_quad((*sfont).handle);
    (*post).numberOfGlyphs = 0 as USHORT;
    (*post).glyphNamePtr = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    (*post).count = 0 as USHORT;
    (*post).names = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    if (*post).Version as ::core::ffi::c_ulong == 0x10000 as ::core::ffi::c_ulong {
        (*post).numberOfGlyphs = 258 as USHORT;
        (*post).glyphNamePtr = &raw mut macglyphorder as *mut *const ::core::ffi::c_char;
    } else if (*post).Version as ::core::ffi::c_ulong == 0x28000 as ::core::ffi::c_ulong {
        pdf_warning!(
            b"TrueType 'post' version 2.5 found (deprecated)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else if (*post).Version as ::core::ffi::c_ulong == 0x20000 as ::core::ffi::c_ulong {
        if read_v2_post_names(post, sfont) < 0 as ::core::ffi::c_int {
            pdf_warning!(
                b"Invalid version 2.0 'post' table\0" as *const u8 as *const ::core::ffi::c_char,
            );
            tt_release_post_table(post);
            post = ::core::ptr::null_mut::<tt_post_table>();
        }
    } else if !((*post).Version as ::core::ffi::c_ulong == 0x30000 as ::core::ffi::c_ulong
        || (*post).Version as ::core::ffi::c_ulong == 0x40000 as ::core::ffi::c_ulong)
    {
        pdf_warning!(
            b"Unknown 'post' version: %08X, assuming version 3.0\0" as *const u8
                as *const ::core::ffi::c_char,
            (*post).Version,
        );
    }
    return post;
}
#[unsafe(export_name = "__pdf_impl_tt_lookup_post_table")]
pub unsafe extern "C-unwind" fn tt_lookup_post_table(
    mut post: *mut tt_post_table,
    mut glyphname: *const ::core::ffi::c_char,
) -> USHORT {
    let mut gid: USHORT = 0;
    if !(!post.is_null() && !glyphname.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"post && glyphname\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    gid = 0 as USHORT;
    while (gid as ::core::ffi::c_int) < (*post).numberOfGlyphs as ::core::ffi::c_int {
        if !(*(*post).glyphNamePtr.offset(gid as isize)).is_null()
            && streq_ptr(glyphname, *(*post).glyphNamePtr.offset(gid as isize))
                as ::core::ffi::c_int
                != 0
        {
            return gid;
        }
        gid = gid.wrapping_add(1);
    }
    return 0 as USHORT;
}
#[unsafe(export_name = "__pdf_impl_tt_get_glyphname")]
pub unsafe extern "C-unwind" fn tt_get_glyphname(
    mut post: *mut tt_post_table,
    mut gid: USHORT,
) -> *mut ::core::ffi::c_char {
    if (gid as ::core::ffi::c_int) < (*post).numberOfGlyphs as ::core::ffi::c_int
        && !(*(*post).glyphNamePtr.offset(gid as isize)).is_null()
    {
        return crate::backend::runtime::duplicate(*(*post).glyphNamePtr.offset(gid as isize));
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[unsafe(export_name = "__pdf_impl_tt_release_post_table")]
pub unsafe extern "C-unwind" fn tt_release_post_table(mut post: *mut tt_post_table) {
    let mut i: USHORT = 0;
    if post.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"post\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(*post).glyphNamePtr.is_null()
        && (*post).Version as ::core::ffi::c_ulong != 0x10000 as ::core::ffi::c_ulong
    {
        free((*post).glyphNamePtr as *mut ::core::ffi::c_void);
    }
    if !(*post).names.is_null() {
        i = 0 as USHORT;
        while (i as ::core::ffi::c_int) < (*post).count as ::core::ffi::c_int {
            free(*(*post).names.offset(i as isize) as *mut ::core::ffi::c_void);
            i = i.wrapping_add(1);
        }
        free((*post).names as *mut ::core::ffi::c_void);
    }
    (*post).count = 0 as USHORT;
    (*post).glyphNamePtr = ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    (*post).names = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    free(post as *mut ::core::ffi::c_void);
}
static mut macglyphorder: [*const ::core::ffi::c_char; 258] = [
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".null\0" as *const u8 as *const ::core::ffi::c_char,
    b"nonmarkingreturn\0" as *const u8 as *const ::core::ffi::c_char,
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
    b"Adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"dagger\0" as *const u8 as *const ::core::ffi::c_char,
    b"degree\0" as *const u8 as *const ::core::ffi::c_char,
    b"cent\0" as *const u8 as *const ::core::ffi::c_char,
    b"sterling\0" as *const u8 as *const ::core::ffi::c_char,
    b"section\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"paragraph\0" as *const u8 as *const ::core::ffi::c_char,
    b"germandbls\0" as *const u8 as *const ::core::ffi::c_char,
    b"registered\0" as *const u8 as *const ::core::ffi::c_char,
    b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
    b"trademark\0" as *const u8 as *const ::core::ffi::c_char,
    b"acute\0" as *const u8 as *const ::core::ffi::c_char,
    b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"notequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"AE\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"infinity\0" as *const u8 as *const ::core::ffi::c_char,
    b"plusminus\0" as *const u8 as *const ::core::ffi::c_char,
    b"lessequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"greaterequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"yen\0" as *const u8 as *const ::core::ffi::c_char,
    b"mu\0" as *const u8 as *const ::core::ffi::c_char,
    b"partialdiff\0" as *const u8 as *const ::core::ffi::c_char,
    b"summation\0" as *const u8 as *const ::core::ffi::c_char,
    b"product\0" as *const u8 as *const ::core::ffi::c_char,
    b"pi\0" as *const u8 as *const ::core::ffi::c_char,
    b"integral\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char,
    b"Omega\0" as *const u8 as *const ::core::ffi::c_char,
    b"ae\0" as *const u8 as *const ::core::ffi::c_char,
    b"oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char,
    b"logicalnot\0" as *const u8 as *const ::core::ffi::c_char,
    b"radical\0" as *const u8 as *const ::core::ffi::c_char,
    b"florin\0" as *const u8 as *const ::core::ffi::c_char,
    b"approxequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Delta\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char,
    b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char,
    b"nonbreakingspace\0" as *const u8 as *const ::core::ffi::c_char,
    b"Agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"OE\0" as *const u8 as *const ::core::ffi::c_char,
    b"oe\0" as *const u8 as *const ::core::ffi::c_char,
    b"endash\0" as *const u8 as *const ::core::ffi::c_char,
    b"emdash\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteright\0" as *const u8 as *const ::core::ffi::c_char,
    b"divide\0" as *const u8 as *const ::core::ffi::c_char,
    b"lozenge\0" as *const u8 as *const ::core::ffi::c_char,
    b"ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"fraction\0" as *const u8 as *const ::core::ffi::c_char,
    b"currency\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char,
    b"fi\0" as *const u8 as *const ::core::ffi::c_char,
    b"fl\0" as *const u8 as *const ::core::ffi::c_char,
    b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"perthousand\0" as *const u8 as *const ::core::ffi::c_char,
    b"Acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"apple\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"Uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char,
    b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"macron\0" as *const u8 as *const ::core::ffi::c_char,
    b"breve\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"ring\0" as *const u8 as *const ::core::ffi::c_char,
    b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char,
    b"ogonek\0" as *const u8 as *const ::core::ffi::c_char,
    b"caron\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"lslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"Scaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"scaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"Zcaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"zcaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"brokenbar\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"Yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"minus\0" as *const u8 as *const ::core::ffi::c_char,
    b"multiply\0" as *const u8 as *const ::core::ffi::c_char,
    b"onesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"twosuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"threesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"onehalf\0" as *const u8 as *const ::core::ffi::c_char,
    b"onequarter\0" as *const u8 as *const ::core::ffi::c_char,
    b"threequarters\0" as *const u8 as *const ::core::ffi::c_char,
    b"franc\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gbreve\0" as *const u8 as *const ::core::ffi::c_char,
    b"gbreve\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idotaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"Scedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"scedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"cacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"ccaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"dcroat\0" as *const u8 as *const ::core::ffi::c_char,
];
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_read_post_table(
    mut sfont: *mut sfnt,
    _result: *mut *mut tt_post_table,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_read_post_table(sfont) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_lookup_post_table(
    mut post: *mut tt_post_table,
    mut glyphname: *const ::core::ffi::c_char,
    _result: *mut USHORT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_lookup_post_table(post, glyphname) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_glyphname(
    mut post: *mut tt_post_table,
    mut gid: USHORT,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_glyphname(post, gid) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_release_post_table(mut post: *mut tt_post_table) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_release_post_table(post) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
