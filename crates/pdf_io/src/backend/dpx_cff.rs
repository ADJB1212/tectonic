/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn, static_mut_refs, dangerous_implicit_autorefs)]
use crate::compat::allocation::free;
use std::{collections::HashMap, ffi::CStr, sync::LazyLock};

static STANDARD_SIDS: LazyLock<HashMap<&'static [u8], u16>> = LazyLock::new(|| {
    cff_stdstr
        .iter()
        .enumerate()
        .map(|(id, &name)| (unsafe { CStr::from_ptr(name) }.to_bytes(), id as u16))
        .collect()
});

#[derive(Default)]
struct StringIndex {
    names: HashMap<Box<[u8]>, u16>,
    offset_capacity: usize,
    data_capacity: usize,
}

#[derive(Default)]
pub(crate) struct StringCaches(HashMap<usize, StringIndex>);

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn string_indices_preserve_first_matches_and_grow_geometrically() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        unsafe {
            let mut font: cff_font = core::mem::zeroed();
            assert_eq!(cff_add_string(&mut font, c"space".as_ptr(), 1), 1);
            assert_eq!((*font._string).count, 0);
            let mut growths = 0;
            let mut capacity = (0, 0);
            for index in 0..8192 {
                let name = std::ffi::CString::new(format!("custom{index}")).unwrap();
                assert_eq!(cff_add_string(&mut font, name.as_ptr(), 1), index + 391);
                let cache = &*string_index(font._string);
                let current = (cache.offset_capacity, cache.data_capacity);
                growths += usize::from(capacity != current);
                capacity = current;
            }
            assert!(growths <= 24, "{growths} reallocations");
            assert_eq!(cff_add_string(&mut font, c"custom0".as_ptr(), 0), 8192 + 391);
            assert_eq!(cff_add_string(&mut font, c"custom0".as_ptr(), 1), 391);
            assert_eq!(cff_add_string(&mut font, c"space".as_ptr(), 0), 8193 + 391);
            cff_update_string(&mut font);
            (*string_caches()).0.remove(&(font.string as usize));
            for index in 0..8192 {
                let name = std::ffi::CString::new(format!("custom{index}")).unwrap();
                assert_eq!(cff_get_sid(&mut font, name.as_ptr()), index + 391);
                let stored = cff_get_string(&mut font, (index + 391) as u16);
                assert_eq!(CStr::from_ptr(stored), name.as_c_str());
                free(stored.cast());
            }
            assert_eq!(cff_get_sid(&mut font, c"space".as_ptr()), 8193 + 391);
            assert_eq!(cff_get_seac_sid(&mut font, c"space".as_ptr()), 1);
            assert_eq!(cff_get_sid(&mut font, c"missing".as_ptr()), -1);
            cff_release_index(font.string);
            assert!((*string_caches()).0.is_empty());
        }
    }
}

unsafe fn string_caches() -> *mut StringCaches {
    core::ptr::addr_of_mut!((*crate::compat::document::current()).cff_strings)
}

unsafe fn string_index(strings: *mut cff_index) -> *mut StringIndex {
    (*string_caches()).0.entry(strings as usize).or_insert_with(|| {
        let count = (*strings).count as usize;
        let mut index = StringIndex::default();
        if count != 0 {
            let offsets = core::slice::from_raw_parts((*strings).offset, count + 1);
            index.offset_capacity = count + 1;
            index.data_capacity = offsets[count] as usize - 1;
            for (id, pair) in offsets.windows(2).enumerate() {
                let length = (pair[1] - pair[0]) as usize;
                let bytes = if length == 0 {
                    &[]
                } else {
                    core::slice::from_raw_parts((*strings).data.add(pair[0] as usize - 1), length)
                };
                index.names.entry(Box::from(bytes)).or_insert(id as u16);
            }
        }
        index
    })
}
unsafe extern "C-unwind" {
    fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: ::core::ffi::c_int) -> size_t;
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut ::core::ffi::c_char, len: size_t) -> ssize_t;
    #[link_name = "__pdf_impl_tt_get_unsigned_byte"]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_cff_release_dict"]
    fn cff_release_dict(dict: *mut cff_dict);
    #[link_name = "__pdf_impl_cff_dict_get"]
    fn cff_dict_get(dict: *mut cff_dict, key: *const ::core::ffi::c_char, idx: ::core::ffi::c_int) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_cff_dict_known"]
    fn cff_dict_known(dict: *mut cff_dict, key: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_cff_dict_unpack"]
    fn cff_dict_unpack(data: *mut card8, endptr: *mut card8) -> *mut cff_dict;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn streq_ptr(mut s1: *const ::core::ffi::c_char, mut s2: *const ::core::ffi::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
    }
    return false_0 != 0;
}
pub const FONTTYPE_CIDFONT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const FONTTYPE_FONT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const ENCODING_STANDARD: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const ENCODING_EXPERT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const CHARSETS_ISOADOBE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const CHARSETS_EXPERT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const CHARSETS_EXPSUB: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
pub const HAVE_STANDARD_ENCODING: ::core::ffi::c_int = ENCODING_STANDARD | ENCODING_EXPERT;
pub const HAVE_STANDARD_CHARSETS: ::core::ffi::c_int = CHARSETS_ISOADOBE | CHARSETS_EXPERT | CHARSETS_EXPSUB;
pub const CFF_STDSTR_MAX: ::core::ffi::c_int = 391 as ::core::ffi::c_int;
const cff_stdstr: [*const ::core::ffi::c_char; 391] = [
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
    b"endash\0" as *const u8 as *const ::core::ffi::c_char,
    b"dagger\0" as *const u8 as *const ::core::ffi::c_char,
    b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
    b"paragraph\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char,
    b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char,
    b"perthousand\0" as *const u8 as *const ::core::ffi::c_char,
    b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
    b"grave\0" as *const u8 as *const ::core::ffi::c_char,
    b"acute\0" as *const u8 as *const ::core::ffi::c_char,
    b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"macron\0" as *const u8 as *const ::core::ffi::c_char,
    b"breve\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"ring\0" as *const u8 as *const ::core::ffi::c_char,
    b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char,
    b"ogonek\0" as *const u8 as *const ::core::ffi::c_char,
    b"caron\0" as *const u8 as *const ::core::ffi::c_char,
    b"emdash\0" as *const u8 as *const ::core::ffi::c_char,
    b"AE\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"OE\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char,
    b"ae\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char,
    b"lslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"oe\0" as *const u8 as *const ::core::ffi::c_char,
    b"germandbls\0" as *const u8 as *const ::core::ffi::c_char,
    b"onesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"logicalnot\0" as *const u8 as *const ::core::ffi::c_char,
    b"mu\0" as *const u8 as *const ::core::ffi::c_char,
    b"trademark\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"onehalf\0" as *const u8 as *const ::core::ffi::c_char,
    b"plusminus\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"onequarter\0" as *const u8 as *const ::core::ffi::c_char,
    b"divide\0" as *const u8 as *const ::core::ffi::c_char,
    b"brokenbar\0" as *const u8 as *const ::core::ffi::c_char,
    b"degree\0" as *const u8 as *const ::core::ffi::c_char,
    b"thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"threequarters\0" as *const u8 as *const ::core::ffi::c_char,
    b"twosuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"registered\0" as *const u8 as *const ::core::ffi::c_char,
    b"minus\0" as *const u8 as *const ::core::ffi::c_char,
    b"eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"multiply\0" as *const u8 as *const ::core::ffi::c_char,
    b"threesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"Atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"Otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Scaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"Uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Zcaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"scaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"zcaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Hungarumlautsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollaroldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollarsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"ampersandsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Acutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenleftsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenrightsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"twodotenleader\0" as *const u8 as *const ::core::ffi::c_char,
    b"onedotenleader\0" as *const u8 as *const ::core::ffi::c_char,
    b"zerooldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"oneoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"twooldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"threeoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"fouroldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"fiveoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"sixoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"sevenoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"eightoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"nineoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"commasuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"threequartersemdash\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"questionsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"asuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"bsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"centsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"dsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"esuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"isuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"lsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"msuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"nsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"osuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"rsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"ssuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"tsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"ff\0" as *const u8 as *const ::core::ffi::c_char,
    b"ffi\0" as *const u8 as *const ::core::ffi::c_char,
    b"ffl\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenleftinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenrightinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Circumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"hyphensuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Asmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Csmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Esmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Hsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ismall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Jsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ksmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Msmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Osmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Psmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Qsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Rsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ssmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Usmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Vsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Wsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Xsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ysmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Zsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"colonmonetary\0" as *const u8 as *const ::core::ffi::c_char,
    b"onefitted\0" as *const u8 as *const ::core::ffi::c_char,
    b"rupiah\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tildesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamdownsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"centoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lslashsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Scaronsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Zcaronsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Brevesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Caronsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dotaccentsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Macronsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"figuredash\0" as *const u8 as *const ::core::ffi::c_char,
    b"hypheninferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ogoneksmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ringsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cedillasmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"questiondownsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"oneeighth\0" as *const u8 as *const ::core::ffi::c_char,
    b"threeeighths\0" as *const u8 as *const ::core::ffi::c_char,
    b"fiveeighths\0" as *const u8 as *const ::core::ffi::c_char,
    b"seveneighths\0" as *const u8 as *const ::core::ffi::c_char,
    b"onethird\0" as *const u8 as *const ::core::ffi::c_char,
    b"twothirds\0" as *const u8 as *const ::core::ffi::c_char,
    b"zerosuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"foursuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"fivesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"sixsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"sevensuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"eightsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"ninesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"zeroinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"oneinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"twoinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"threeinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"fourinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"fiveinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"sixinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"seveninferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"eightinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"nineinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"centinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollarinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"commainferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Agravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Acircumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Atildesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aringsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"AEsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccedillasmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Egravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ecircumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Edieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Igravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Iacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Icircumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ethsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ntildesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ogravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ocircumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Otildesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Odieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"OEsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oslashsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ugravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Uacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ucircumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Udieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Yacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thornsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ydieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"001.000\0" as *const u8 as *const ::core::ffi::c_char,
    b"001.001\0" as *const u8 as *const ::core::ffi::c_char,
    b"001.002\0" as *const u8 as *const ::core::ffi::c_char,
    b"001.003\0" as *const u8 as *const ::core::ffi::c_char,
    b"Black\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bold\0" as *const u8 as *const ::core::ffi::c_char,
    b"Book\0" as *const u8 as *const ::core::ffi::c_char,
    b"Light\0" as *const u8 as *const ::core::ffi::c_char,
    b"Medium\0" as *const u8 as *const ::core::ffi::c_char,
    b"Regular\0" as *const u8 as *const ::core::ffi::c_char,
    b"Roman\0" as *const u8 as *const ::core::ffi::c_char,
    b"Semibold\0" as *const u8 as *const ::core::ffi::c_char,
];
pub const CFF_DEBUG_STR: [::core::ffi::c_char; 4] = crate::backend::c_char_array(*b"CFF\0");
unsafe extern "C-unwind" fn get_unsigned(mut handle: rust_input_handle_t, mut n: ::core::ffi::c_int) -> ::core::ffi::c_uint {
    let mut v: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        v = v
            .wrapping_mul(0x100 as ::core::ffi::c_uint)
            .wrapping_add(tt_get_unsigned_byte(handle) as ::core::ffi::c_uint);
    }
    return v;
}
#[unsafe(export_name = "__pdf_impl_cff_open")]
pub unsafe extern "C-unwind" fn cff_open(
    mut handle: rust_input_handle_t, mut offset: ::core::ffi::c_int, mut n: ::core::ffi::c_int,
) -> *mut cff_font {
    let mut cff: *mut cff_font = ::core::ptr::null_mut::<cff_font>();
    let mut idx: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    cff = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_font>() as usize) as uint32_t) as *mut cff_font;
    (*cff).fontname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*cff).index = n;
    (*cff).handle = handle;
    (*cff).offset = offset as l_offset;
    (*cff).filter = 0 as ::core::ffi::c_int;
    (*cff).flag = 0 as ::core::ffi::c_int;
    (*cff).name = ::core::ptr::null_mut::<cff_index>();
    (*cff).topdict = ::core::ptr::null_mut::<cff_dict>();
    (*cff).gsubr = ::core::ptr::null_mut::<cff_index>();
    (*cff).encoding = ::core::ptr::null_mut::<cff_encoding>();
    (*cff).charsets = ::core::ptr::null_mut::<cff_charsets>();
    (*cff).fdselect = ::core::ptr::null_mut::<cff_fdselect>();
    (*cff).cstrings = ::core::ptr::null_mut::<cff_index>();
    (*cff).fdarray = ::core::ptr::null_mut::<*mut cff_dict>();
    (*cff).private = ::core::ptr::null_mut::<*mut cff_dict>();
    (*cff).subrs = ::core::ptr::null_mut::<*mut cff_index>();
    (*cff).num_glyphs = 0 as card16;
    (*cff).num_fds = 0 as card8;
    (*cff).string = ::core::ptr::null_mut::<cff_index>();
    (*cff)._string = ::core::ptr::null_mut::<cff_index>();
    crate::backend::runtime::input_seek((*cff).handle, (*cff).offset.wrapping_add(0 as l_offset) as ssize_t, SEEK_SET);
    (*cff).header.major = tt_get_unsigned_byte((*cff).handle) as card8;
    (*cff).header.minor = tt_get_unsigned_byte((*cff).handle) as card8;
    (*cff).header.hdr_size = tt_get_unsigned_byte((*cff).handle) as card8;
    (*cff).header.offsize = tt_get_unsigned_byte((*cff).handle) as c_offsize;
    if ((*cff).header.offsize as ::core::ffi::c_int) < 1 as ::core::ffi::c_int
        || (*cff).header.offsize as ::core::ffi::c_int > 4 as ::core::ffi::c_int
    {
        pdf_abort!(b"invalid offsize data\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*cff).header.major as ::core::ffi::c_int > 1 as ::core::ffi::c_int || (*cff).header.minor as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"%s: CFF version %u.%u not supported.\0" as *const u8 as *const ::core::ffi::c_char,
            CFF_DEBUG_STR.as_ptr(),
            (*cff).header.major as ::core::ffi::c_int,
            (*cff).header.minor as ::core::ffi::c_int,
        );
        cff_close(cff);
        return ::core::ptr::null_mut::<cff_font>();
    }
    crate::backend::runtime::input_seek(
        (*cff).handle,
        (*cff).offset.wrapping_add((*cff).header.hdr_size as l_offset) as ssize_t,
        SEEK_SET,
    );
    idx = cff_get_index(cff);
    if n > (*idx).count as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"%s: Invalid CFF fontset index number.\0" as *const u8 as *const ::core::ffi::c_char,
            CFF_DEBUG_STR.as_ptr(),
        );
        cff_close(cff);
        return ::core::ptr::null_mut::<cff_font>();
    }
    (*cff).name = idx;
    (*cff).fontname = cff_get_name(cff);
    idx = cff_get_index(cff);
    if n > (*idx).count as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        pdf_abort!(b"CFF Top DICT not exist...\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*cff).topdict = cff_dict_unpack(
        (*idx)
            .data
            .offset(*(*idx).offset.offset(n as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
        (*idx)
            .data
            .offset(*(*idx).offset.offset((n + 1 as ::core::ffi::c_int) as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
    );
    if (*cff).topdict.is_null() {
        pdf_abort!(b"Parsing CFF Top DICT data failed...\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    cff_release_index(idx);
    if cff_dict_known((*cff).topdict, b"CharstringType\0" as *const u8 as *const ::core::ffi::c_char) != 0
        && cff_dict_get(
            (*cff).topdict,
            b"CharstringType\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) != 2 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        pdf_warning!(b"Only Type 2 Charstrings supported...\0" as *const u8 as *const ::core::ffi::c_char,);
        cff_close(cff);
        return ::core::ptr::null_mut::<cff_font>();
    }
    if cff_dict_known((*cff).topdict, b"SyntheticBase\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        pdf_warning!(b"CFF Synthetic font not supported.\0" as *const u8 as *const ::core::ffi::c_char,);
        cff_close(cff);
        return ::core::ptr::null_mut::<cff_font>();
    }
    (*cff).string = cff_get_index(cff);
    (*cff).gsubr_offset = crate::backend::runtime::input_seek((*cff).handle, 0 as ssize_t, SEEK_CUR).wrapping_sub(offset as size_t) as l_offset;
    offset = cff_dict_get(
        (*cff).topdict,
        b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    crate::backend::runtime::input_seek((*cff).handle, (*cff).offset.wrapping_add(offset as l_offset) as ssize_t, SEEK_SET);
    (*cff).num_glyphs = tt_get_unsigned_pair((*cff).handle) as card16;
    if cff_dict_known((*cff).topdict, b"ROS\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        (*cff).flag |= FONTTYPE_CIDFONT;
    } else {
        (*cff).flag |= FONTTYPE_FONT;
    }
    if cff_dict_known((*cff).topdict, b"Encoding\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        offset = cff_dict_get(
            (*cff).topdict,
            b"Encoding\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if offset == 0 as ::core::ffi::c_int {
            (*cff).flag |= ENCODING_STANDARD;
        } else if offset == 1 as ::core::ffi::c_int {
            (*cff).flag |= ENCODING_EXPERT;
        }
    } else {
        (*cff).flag |= ENCODING_STANDARD;
    }
    if cff_dict_known((*cff).topdict, b"charset\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
        offset = cff_dict_get(
            (*cff).topdict,
            b"charset\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if offset == 0 as ::core::ffi::c_int {
            (*cff).flag |= CHARSETS_ISOADOBE;
        } else if offset == 1 as ::core::ffi::c_int {
            (*cff).flag |= CHARSETS_EXPERT;
        } else if offset == 2 as ::core::ffi::c_int {
            (*cff).flag |= CHARSETS_EXPSUB;
        }
    } else {
        (*cff).flag |= CHARSETS_ISOADOBE;
    }
    crate::backend::runtime::input_seek((*cff).handle, (*cff).offset.wrapping_add((*cff).gsubr_offset) as ssize_t, SEEK_SET);
    return cff;
}
#[unsafe(export_name = "__pdf_impl_cff_close")]
pub unsafe extern "C-unwind" fn cff_close(mut cff: *mut cff_font) {
    let mut i: card16 = 0;
    if !cff.is_null() {
        free((*cff).fontname as *mut ::core::ffi::c_void);
        if !(*cff).name.is_null() {
            cff_release_index((*cff).name);
        }
        if !(*cff).topdict.is_null() {
            cff_release_dict((*cff).topdict);
        }
        if !(*cff).string.is_null() {
            cff_release_index((*cff).string);
        }
        if !(*cff).gsubr.is_null() {
            cff_release_index((*cff).gsubr);
        }
        if !(*cff).encoding.is_null() {
            cff_release_encoding((*cff).encoding);
        }
        if !(*cff).charsets.is_null() {
            cff_release_charsets((*cff).charsets);
        }
        if !(*cff).fdselect.is_null() {
            cff_release_fdselect((*cff).fdselect);
        }
        if !(*cff).cstrings.is_null() {
            cff_release_index((*cff).cstrings);
        }
        if !(*cff).fdarray.is_null() {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*cff).num_fds as ::core::ffi::c_int {
                if !(*(*cff).fdarray.offset(i as isize)).is_null() {
                    cff_release_dict(*(*cff).fdarray.offset(i as isize));
                }
                i = i.wrapping_add(1);
            }
            free((*cff).fdarray as *mut ::core::ffi::c_void);
        }
        if !(*cff).private.is_null() {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*cff).num_fds as ::core::ffi::c_int {
                if !(*(*cff).private.offset(i as isize)).is_null() {
                    cff_release_dict(*(*cff).private.offset(i as isize));
                }
                i = i.wrapping_add(1);
            }
            free((*cff).private as *mut ::core::ffi::c_void);
        }
        if !(*cff).subrs.is_null() {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*cff).num_fds as ::core::ffi::c_int {
                if !(*(*cff).subrs.offset(i as isize)).is_null() {
                    cff_release_index(*(*cff).subrs.offset(i as isize));
                }
                i = i.wrapping_add(1);
            }
            free((*cff).subrs as *mut ::core::ffi::c_void);
        }
        if !(*cff)._string.is_null() {
            cff_release_index((*cff)._string);
        }
        free(cff as *mut ::core::ffi::c_void);
    }
}
#[unsafe(export_name = "__pdf_impl_cff_get_name")]
pub unsafe extern "C-unwind" fn cff_get_name(mut cff: *mut cff_font) -> *mut ::core::ffi::c_char {
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: l_offset = 0;
    let mut idx: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    idx = (*cff).name;
    len = (*(*idx).offset.offset(((*cff).index + 1 as ::core::ffi::c_int) as isize)).wrapping_sub(*(*idx).offset.offset((*cff).index as isize));
    fontname = new((len.wrapping_add(1 as l_offset) as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
        as *mut ::core::ffi::c_char;
    memcpy(
        fontname as *mut ::core::ffi::c_void,
        (*idx)
            .data
            .offset(*(*idx).offset.offset((*cff).index as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)) as *const ::core::ffi::c_void,
        len as size_t,
    );
    *fontname.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    return fontname;
}
#[unsafe(export_name = "__pdf_impl_cff_set_name")]
pub unsafe extern "C-unwind" fn cff_set_name(mut cff: *mut cff_font, mut name: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut idx: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    if strlen(name) > 127 as size_t {
        pdf_abort!(b"FontName string length too large...\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    if !(*cff).name.is_null() {
        cff_release_index((*cff).name);
    }
    idx =
        new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_index>() as usize) as uint32_t) as *mut cff_index;
    (*cff).name = idx;
    (*idx).count = 1 as card16;
    (*idx).offsize = 1 as c_offsize;
    (*idx).offset =
        new((2 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<l_offset>() as usize) as uint32_t) as *mut l_offset;
    *(*idx).offset.offset(0 as ::core::ffi::c_int as isize) = 1 as l_offset;
    *(*idx).offset.offset(1 as ::core::ffi::c_int as isize) = strlen(name).wrapping_add(1 as size_t) as l_offset;
    (*idx).data = new((strlen(name) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t) as *mut card8;
    memmove((*idx).data as *mut ::core::ffi::c_void, name as *const ::core::ffi::c_void, strlen(name));
    return (5 as size_t).wrapping_add(strlen(name)) as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_cff_put_header")]
pub unsafe extern "C-unwind" fn cff_put_header(mut cff: *mut cff_font, mut dest: *mut card8, mut destlen: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if destlen < 4 as ::core::ffi::c_int {
        pdf_abort!(b"Not enough space available...\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    let fresh1 = dest;
    dest = dest.offset(1);
    *fresh1 = (*cff).header.major;
    let fresh2 = dest;
    dest = dest.offset(1);
    *fresh2 = (*cff).header.minor;
    let fresh3 = dest;
    dest = dest.offset(1);
    *fresh3 = 4 as card8;
    let fresh4 = dest;
    dest = dest.offset(1);
    *fresh4 = 4 as card8;
    (*cff).header.offsize = 4 as c_offsize;
    return 4 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_cff_get_index_header")]
pub unsafe extern "C-unwind" fn cff_get_index_header(cff: *mut cff_font) -> *mut cff_index {
    unsafe { read_index(cff, false) }
}

#[unsafe(export_name = "__pdf_impl_cff_get_index")]
pub unsafe extern "C-unwind" fn cff_get_index(cff: *mut cff_font) -> *mut cff_index {
    unsafe { read_index(cff, true) }
}

unsafe fn read_index(cff: *mut cff_font, payload: bool) -> *mut cff_index {
    use std::io::{Read, Seek, SeekFrom};
    let mut input = crate::compat::input::ProbeInput {
        handle: unsafe { (*cff).handle },
    };
    let header = crate::cff::IndexHeader::read(&mut input).unwrap_or_else(|error| pdf_abort!("{}", error.message.to_string_lossy()));
    let mut data = Vec::new();
    if payload && header.count != 0 {
        let position = input.stream_position().unwrap_or_else(|_| pdf_abort!("Invalid CFF index position"));
        let end = input.seek(SeekFrom::End(0)).unwrap_or_else(|_| pdf_abort!("Invalid CFF index position"));
        input
            .seek(SeekFrom::Start(position))
            .unwrap_or_else(|_| pdf_abort!("Invalid CFF index position"));
        if header.data_len() as u64 > end.saturating_sub(position) {
            pdf_abort!("File ended prematurely\n");
        }
        data.resize(header.data_len(), 0);
        input.read_exact(&mut data).unwrap_or_else(|_| pdf_abort!("File ended prematurely\n"));
    }
    unsafe {
        let offsets = if header.offsets.is_empty() {
            core::ptr::null_mut()
        } else {
            let output = new((header.offsets.len() * core::mem::size_of::<u32>()) as u32).cast::<u32>();
            core::ptr::copy_nonoverlapping(header.offsets.as_ptr(), output, header.offsets.len());
            output
        };
        let bytes = if !payload || header.count == 0 {
            core::ptr::null_mut()
        } else {
            let output = new(data.len() as u32).cast::<u8>();
            core::ptr::copy_nonoverlapping(data.as_ptr(), output, data.len());
            output
        };
        let index = new(core::mem::size_of::<cff_index>() as u32).cast::<cff_index>();
        index.write(cff_index {
            count: header.count,
            offsize: header.width,
            offset: offsets,
            data: bytes,
        });
        index
    }
}
#[unsafe(export_name = "__pdf_impl_cff_pack_index")]
pub unsafe extern "C-unwind" fn cff_pack_index(mut idx: *mut cff_index, mut dest: *mut card8, mut destlen: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut datalen: size_t = 0;
    let mut i: card16 = 0;
    if ((*idx).count as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        if destlen < 2 as ::core::ffi::c_int {
            pdf_abort!(b"Not enough space available...\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        memset(dest as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, 2 as size_t);
        return 2 as ::core::ffi::c_int;
    }
    len = cff_index_size(idx);
    datalen = (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1 as l_offset) as size_t;
    if destlen < len {
        pdf_abort!(b"Not enough space available...\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    let fresh5 = dest;
    dest = dest.offset(1);
    *fresh5 = ((*idx).count as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
    let fresh6 = dest;
    dest = dest.offset(1);
    *fresh6 = ((*idx).count as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
    if datalen < 0xff as size_t {
        (*idx).offsize = 1 as c_offsize;
        let fresh7 = dest;
        dest = dest.offset(1);
        *fresh7 = 1 as card8;
        i = 0 as card16;
        while i as ::core::ffi::c_int <= (*idx).count as ::core::ffi::c_int {
            let fresh8 = dest;
            dest = dest.offset(1);
            *fresh8 = (*(*idx).offset.offset(i as isize) & 0xff as l_offset) as card8;
            i = i.wrapping_add(1);
        }
    } else if datalen < 0xffff as size_t {
        (*idx).offsize = 2 as c_offsize;
        let fresh9 = dest;
        dest = dest.offset(1);
        *fresh9 = 2 as card8;
        i = 0 as card16;
        while i as ::core::ffi::c_int <= (*idx).count as ::core::ffi::c_int {
            let fresh10 = dest;
            dest = dest.offset(1);
            *fresh10 = (*(*idx).offset.offset(i as isize) >> 8 as ::core::ffi::c_int & 0xff as l_offset) as card8;
            let fresh11 = dest;
            dest = dest.offset(1);
            *fresh11 = (*(*idx).offset.offset(i as isize) & 0xff as l_offset) as card8;
            i = i.wrapping_add(1);
        }
    } else if datalen < 0xffffff as size_t {
        (*idx).offsize = 3 as c_offsize;
        let fresh12 = dest;
        dest = dest.offset(1);
        *fresh12 = 3 as card8;
        i = 0 as card16;
        while i as ::core::ffi::c_int <= (*idx).count as ::core::ffi::c_int {
            let fresh13 = dest;
            dest = dest.offset(1);
            *fresh13 = (*(*idx).offset.offset(i as isize) >> 16 as ::core::ffi::c_int & 0xff as l_offset) as card8;
            let fresh14 = dest;
            dest = dest.offset(1);
            *fresh14 = (*(*idx).offset.offset(i as isize) >> 8 as ::core::ffi::c_int & 0xff as l_offset) as card8;
            let fresh15 = dest;
            dest = dest.offset(1);
            *fresh15 = (*(*idx).offset.offset(i as isize) & 0xff as l_offset) as card8;
            i = i.wrapping_add(1);
        }
    } else {
        (*idx).offsize = 4 as c_offsize;
        let fresh16 = dest;
        dest = dest.offset(1);
        *fresh16 = 4 as card8;
        i = 0 as card16;
        while i as ::core::ffi::c_int <= (*idx).count as ::core::ffi::c_int {
            let fresh17 = dest;
            dest = dest.offset(1);
            *fresh17 = (*(*idx).offset.offset(i as isize) >> 24 as ::core::ffi::c_int & 0xff as l_offset) as card8;
            let fresh18 = dest;
            dest = dest.offset(1);
            *fresh18 = (*(*idx).offset.offset(i as isize) >> 16 as ::core::ffi::c_int & 0xff as l_offset) as card8;
            let fresh19 = dest;
            dest = dest.offset(1);
            *fresh19 = (*(*idx).offset.offset(i as isize) >> 8 as ::core::ffi::c_int & 0xff as l_offset) as card8;
            let fresh20 = dest;
            dest = dest.offset(1);
            *fresh20 = (*(*idx).offset.offset(i as isize) & 0xff as l_offset) as card8;
            i = i.wrapping_add(1);
        }
    }
    memmove(
        dest as *mut ::core::ffi::c_void,
        (*idx).data as *const ::core::ffi::c_void,
        (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1 as l_offset) as size_t,
    );
    return len;
}
#[unsafe(export_name = "__pdf_impl_cff_index_size")]
pub unsafe extern "C-unwind" fn cff_index_size(mut idx: *mut cff_index) -> ::core::ffi::c_int {
    if (*idx).count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut datalen: l_offset = 0;
        datalen = (*(*idx).offset.offset((*idx).count as isize)).wrapping_sub(1 as l_offset);
        if (datalen as ::core::ffi::c_ulong) < 0xff as ::core::ffi::c_ulong {
            (*idx).offsize = 1 as c_offsize;
        } else if (datalen as ::core::ffi::c_ulong) < 0xffff as ::core::ffi::c_ulong {
            (*idx).offsize = 2 as c_offsize;
        } else if (datalen as ::core::ffi::c_ulong) < 0xffffff as ::core::ffi::c_ulong {
            (*idx).offsize = 3 as c_offsize;
        } else {
            (*idx).offsize = 4 as c_offsize;
        }
        return ((3 as ::core::ffi::c_int + (*idx).offsize as ::core::ffi::c_int * ((*idx).count as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
            as l_offset)
            .wrapping_add(datalen) as ::core::ffi::c_int;
    } else {
        return 2 as ::core::ffi::c_int;
    };
}
#[unsafe(export_name = "__pdf_impl_cff_new_index")]
pub unsafe extern "C-unwind" fn cff_new_index(mut count: card16) -> *mut cff_index {
    let mut idx: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    idx =
        new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<cff_index>() as usize) as uint32_t) as *mut cff_index;
    (*idx).count = count;
    (*idx).offsize = 0 as c_offsize;
    if count as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        (*idx).offset = new(((count as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<l_offset>() as usize) as uint32_t) as *mut l_offset;
        *(*idx).offset.offset(0 as ::core::ffi::c_int as isize) = 1 as l_offset;
    } else {
        (*idx).offset = ::core::ptr::null_mut::<l_offset>();
    }
    (*idx).data = ::core::ptr::null_mut::<card8>();
    return idx;
}
#[unsafe(export_name = "__pdf_impl_cff_release_index")]
pub unsafe extern "C-unwind" fn cff_release_index(mut idx: *mut cff_index) {
    if !idx.is_null() {
        (*string_caches()).0.remove(&(idx as usize));
        if !(*idx).data.is_null() {
            free((*idx).data as *mut ::core::ffi::c_void);
        }
        if !(*idx).offset.is_null() {
            free((*idx).offset as *mut ::core::ffi::c_void);
        }
        free(idx as *mut ::core::ffi::c_void);
    }
}
#[unsafe(export_name = "__pdf_impl_cff_get_string")]
pub unsafe extern "C-unwind" fn cff_get_string(mut cff: *mut cff_font, mut id: s_SID) -> *mut ::core::ffi::c_char {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    if (id as ::core::ffi::c_int) < CFF_STDSTR_MAX {
        len = strlen(cff_stdstr[id as usize]) as ::core::ffi::c_int;
        result = new(
            ((len + 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
        ) as *mut ::core::ffi::c_char;
        memcpy(
            result as *mut ::core::ffi::c_void,
            cff_stdstr[id as usize] as *const ::core::ffi::c_void,
            len as size_t,
        );
        *result.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else if !cff.is_null() && !(*cff).string.is_null() {
        let mut strings: *mut cff_index = (*cff).string;
        id = (id as ::core::ffi::c_int - CFF_STDSTR_MAX) as s_SID;
        if (id as ::core::ffi::c_int) < (*strings).count as ::core::ffi::c_int {
            len = (*(*strings).offset.offset((id as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(*(*strings).offset.offset(id as isize)) as ::core::ffi::c_int;
            result = new(
                ((len + 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                    as uint32_t,
            ) as *mut ::core::ffi::c_char;
            memmove(
                result as *mut ::core::ffi::c_void,
                (*strings)
                    .data
                    .offset(*(*strings).offset.offset(id as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as *const ::core::ffi::c_void,
                len as size_t,
            );
            *result.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
    }
    return result;
}
#[unsafe(export_name = "__pdf_impl_cff_get_sid")]
pub unsafe extern "C-unwind" fn cff_get_sid(mut cff: *mut cff_font, mut str: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if cff.is_null() || str.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let bytes = CStr::from_ptr(str).to_bytes();
    if !(*cff).string.is_null() {
        if let Some(&id) = (*string_index((*cff).string)).names.get(bytes) {
            return i32::from(id) + CFF_STDSTR_MAX;
        }
    }
    STANDARD_SIDS.get(bytes).map_or(-1, |&id| i32::from(id))
}
#[unsafe(export_name = "__pdf_impl_cff_get_seac_sid")]
pub unsafe extern "C-unwind" fn cff_get_seac_sid(mut cff: *mut cff_font, mut str: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if cff.is_null() || str.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    STANDARD_SIDS.get(CStr::from_ptr(str).to_bytes()).map_or(-1, |&id| i32::from(id))
}
unsafe extern "C-unwind" fn cff_match_string(mut cff: *mut cff_font, mut str: *const ::core::ffi::c_char, mut sid: s_SID) -> ::core::ffi::c_int {
    let mut i: card16 = 0;
    if (sid as ::core::ffi::c_int) < CFF_STDSTR_MAX {
        return if streq_ptr(str, cff_stdstr[sid as usize]) as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    } else {
        i = (sid as ::core::ffi::c_int - CFF_STDSTR_MAX) as card16;
        if cff.is_null() || (*cff).string.is_null() || i as ::core::ffi::c_int >= (*(*cff).string).count as ::core::ffi::c_int {
            pdf_abort!(b"Invalid SID\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if strlen(str)
            == (*(*(*cff).string)
                .offset
                .offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize))
            .wrapping_sub(*(*(*cff).string).offset.offset(i as isize)) as size_t
        {
            return if memcmp(
                str as *const ::core::ffi::c_void,
                (*(*cff).string)
                    .data
                    .offset(*(*(*cff).string).offset.offset(i as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)) as *const ::core::ffi::c_void,
                strlen(str),
            ) == 0
            {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_cff_update_string")]
pub unsafe extern "C-unwind" fn cff_update_string(mut cff: *mut cff_font) {
    if cff.is_null() {
        pdf_abort!(b"CFF font not opened.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !(*cff).string.is_null() {
        cff_release_index((*cff).string);
    }
    (*cff).string = (*cff)._string;
    (*cff)._string = ::core::ptr::null_mut::<cff_index>();
}
#[unsafe(export_name = "__pdf_impl_cff_add_string")]
pub unsafe extern "C-unwind" fn cff_add_string(mut cff: *mut cff_font, mut str: *const ::core::ffi::c_char, mut unique: ::core::ffi::c_int) -> s_SID {
    let bytes = CStr::from_ptr(str).to_bytes();
    if cff.is_null() {
        pdf_abort!(b"CFF font not opened.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*cff)._string.is_null() {
        (*cff)._string = cff_new_index(0 as card16);
    }
    let strings = (*cff)._string;
    let cache = string_index(strings);
    if unique != 0 {
        if let Some(&id) = STANDARD_SIDS.get(bytes) {
            return id;
        }
        if let Some(&id) = (*cache).names.get(bytes) {
            return id + CFF_STDSTR_MAX as u16;
        }
    }
    let count = (*strings).count as usize;
    if count > u16::MAX as usize - CFF_STDSTR_MAX as usize {
        pdf_abort!("Too many CFF strings");
    }
    let offset = if count == 0 { 1 } else { *(*strings).offset.add(count) as usize };
    let end = offset
        .checked_add(bytes.len())
        .filter(|&end| end <= u32::MAX as usize)
        .unwrap_or_else(|| pdf_abort!("CFF strings are too large"));
    if count + 2 > (*cache).offset_capacity {
        (*cache).offset_capacity =
            crate::compat::allocation::grow_capacity((*cache).offset_capacity, count + 2, core::mem::size_of::<l_offset>(), 16);
        (*strings).offset = renew(
            (*strings).offset.cast(),
            ((*cache).offset_capacity * core::mem::size_of::<l_offset>()) as u32,
        )
        .cast();
    }
    if end - 1 > (*cache).data_capacity {
        (*cache).data_capacity = crate::compat::allocation::grow_capacity((*cache).data_capacity, end - 1, 1, 256);
        (*strings).data = renew((*strings).data.cast(), (*cache).data_capacity as u32).cast();
    }
    if count == 0 {
        *(*strings).offset = 1;
    }
    if !bytes.is_empty() {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), (*strings).data.add(offset - 1), bytes.len());
    }
    *(*strings).offset.add(count + 1) = end as u32;
    (*strings).count += 1;
    (*cache).names.entry(Box::from(bytes)).or_insert(count as u16);
    (count + CFF_STDSTR_MAX as usize) as u16
}
unsafe fn copy_font_array<T: Copy>(values: &[T]) -> *mut T {
    if values.is_empty() {
        return core::ptr::null_mut();
    }
    let length = values
        .len()
        .checked_mul(core::mem::size_of::<T>())
        .and_then(|v| u32::try_from(v).ok())
        .unwrap_or_else(|| pdf_abort!("CFF data is too large."));
    unsafe {
        let output = new(length).cast::<T>();
        core::ptr::copy_nonoverlapping(values.as_ptr(), output, values.len());
        output
    }
}

unsafe fn seek_cff(cff: *mut cff_font, offset: i32) -> crate::compat::input::ProbeInput {
    use std::io::{Seek, SeekFrom};
    let offset = u32::try_from(offset)
        .ok()
        .and_then(|offset| unsafe { (*cff).offset.checked_add(offset) })
        .unwrap_or_else(|| pdf_abort!("Invalid CFF data offset."));
    let mut input = crate::compat::input::ProbeInput {
        handle: unsafe { (*cff).handle },
    };
    input
        .seek(SeekFrom::Start(offset.into()))
        .unwrap_or_else(|_| pdf_abort!("Invalid CFF data offset."));
    input
}

#[unsafe(export_name = "__pdf_impl_cff_read_encoding")]
pub unsafe extern "C-unwind" fn cff_read_encoding(mut cff: *mut cff_font) -> ::core::ffi::c_int {
    let mut encoding: *mut cff_encoding = ::core::ptr::null_mut::<cff_encoding>();
    let mut offset: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    let mut i: card8 = 0;
    if (*cff).topdict.is_null() {
        pdf_abort!(b"Top DICT data not found\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    if cff_dict_known((*cff).topdict, b"Encoding\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*cff).flag |= ENCODING_STANDARD;
        (*cff).encoding = ::core::ptr::null_mut::<cff_encoding>();
        return 0 as ::core::ffi::c_int;
    }
    offset = cff_dict_get(
        (*cff).topdict,
        b"Encoding\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if offset == 0 as ::core::ffi::c_int {
        (*cff).flag |= ENCODING_STANDARD;
        (*cff).encoding = ::core::ptr::null_mut::<cff_encoding>();
        return 0 as ::core::ffi::c_int;
    } else if offset == 1 as ::core::ffi::c_int {
        (*cff).flag |= ENCODING_EXPERT;
        (*cff).encoding = ::core::ptr::null_mut::<cff_encoding>();
        return 0 as ::core::ffi::c_int;
    }
    let mut input = unsafe { seek_cff(cff, offset) };
    let (parsed, length) = crate::cff::encoding(&mut input).unwrap_or_else(|error| pdf_abort!("{}", error.message.to_string_lossy()));
    unsafe {
        let (count, data) = match parsed.values {
            crate::cff::EncodingValues::Codes(codes) => (
                codes.len() as u8,
                C2RustUnnamed {
                    codes: copy_font_array(&codes),
                },
            ),
            crate::cff::EncodingValues::Ranges(ranges) => {
                let ranges: Vec<_> = ranges
                    .into_iter()
                    .map(|(first, n_left)| cff_range1 { first: first.into(), n_left })
                    .collect();
                (
                    ranges.len() as u8,
                    C2RustUnnamed {
                        range1: copy_font_array(&ranges),
                    },
                )
            }
        };
        let supplements: Vec<_> = parsed.supplements.into_iter().map(|(code, glyph)| cff_map { code, glyph }).collect();
        let value = cff_encoding {
            format: parsed.format,
            num_entries: count,
            data,
            num_supps: supplements.len() as u8,
            supp: copy_font_array(&supplements),
        };
        (*cff).encoding = copy_font_array(&[value]);
    }
    length as i32
}
#[unsafe(export_name = "__pdf_impl_cff_pack_encoding")]
pub unsafe extern "C-unwind" fn cff_pack_encoding(
    mut cff: *mut cff_font, mut dest: *mut card8, mut destlen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut encoding: *mut cff_encoding = ::core::ptr::null_mut::<cff_encoding>();
    let mut i: card16 = 0;
    if (*cff).flag & HAVE_STANDARD_ENCODING != 0 || (*cff).encoding.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if destlen < 2 as ::core::ffi::c_int {
        pdf_abort!(b"in cff_pack_encoding(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    encoding = (*cff).encoding;
    let fresh31 = len;
    len = len + 1;
    *dest.offset(fresh31 as isize) = (*encoding).format;
    let fresh32 = len;
    len = len + 1;
    *dest.offset(fresh32 as isize) = (*encoding).num_entries;
    match (*encoding).format as ::core::ffi::c_int & !(0x80 as ::core::ffi::c_int) {
        0 => {
            if destlen < len + (*encoding).num_entries as ::core::ffi::c_int {
                pdf_abort!(b"in cff_pack_encoding(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*encoding).num_entries as ::core::ffi::c_int {
                let fresh33 = len;
                len = len + 1;
                *dest.offset(fresh33 as isize) = *(*encoding).data.codes.offset(i as isize);
                i = i.wrapping_add(1);
            }
        }
        1 => {
            if destlen < len + (*encoding).num_entries as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
                pdf_abort!(b"in cff_pack_encoding(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*encoding).num_entries as ::core::ffi::c_int {
                let fresh34 = len;
                len = len + 1;
                *dest.offset(fresh34 as isize) =
                    ((*(*encoding).data.range1.offset(i as isize)).first as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
                let fresh35 = len;
                len = len + 1;
                *dest.offset(fresh35 as isize) = (*(*encoding).data.range1.offset(i as isize)).n_left;
                i = i.wrapping_add(1);
            }
        }
        _ => {
            pdf_abort!(b"Unknown Encoding format\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    if (*encoding).format as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        if destlen < len + (*encoding).num_supps as ::core::ffi::c_int * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
            pdf_abort!(b"in cff_pack_encoding(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        let fresh36 = len;
        len = len + 1;
        *dest.offset(fresh36 as isize) = (*encoding).num_supps;
        i = 0 as card16;
        while (i as ::core::ffi::c_int) < (*encoding).num_supps as ::core::ffi::c_int {
            let fresh37 = len;
            len = len + 1;
            *dest.offset(fresh37 as isize) = (*(*encoding).supp.offset(i as isize)).code;
            let fresh38 = len;
            len = len + 1;
            *dest.offset(fresh38 as isize) =
                ((*(*encoding).supp.offset(i as isize)).glyph as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh39 = len;
            len = len + 1;
            *dest.offset(fresh39 as isize) =
                ((*(*encoding).supp.offset(i as isize)).glyph as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            i = i.wrapping_add(1);
        }
    }
    return len;
}
#[unsafe(export_name = "__pdf_impl_cff_encoding_lookup")]
pub unsafe extern "C-unwind" fn cff_encoding_lookup(mut cff: *mut cff_font, mut code: card8) -> card16 {
    let mut gid: card16 = 0 as card16;
    let mut encoding: *mut cff_encoding = ::core::ptr::null_mut::<cff_encoding>();
    let mut i: card16 = 0;
    if (*cff).flag & (ENCODING_STANDARD | ENCODING_EXPERT) != 0 {
        pdf_abort!(b"Predefined CFF encoding not supported yet\0" as *const u8 as *const ::core::ffi::c_char,);
    } else if (*cff).encoding.is_null() {
        pdf_abort!(b"Encoding data not available\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    encoding = (*cff).encoding;
    gid = 0 as card16;
    match (*encoding).format as ::core::ffi::c_int & !(0x80 as ::core::ffi::c_int) {
        0 => {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*encoding).num_entries as ::core::ffi::c_int {
                if code as ::core::ffi::c_int == *(*encoding).data.codes.offset(i as isize) as ::core::ffi::c_int {
                    gid = (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as card16;
                    break;
                } else {
                    i = i.wrapping_add(1);
                }
            }
        }
        1 => {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*encoding).num_entries as ::core::ffi::c_int {
                if code as ::core::ffi::c_int >= (*(*encoding).data.range1.offset(i as isize)).first as ::core::ffi::c_int
                    && code as ::core::ffi::c_int
                        <= (*(*encoding).data.range1.offset(i as isize)).first as ::core::ffi::c_int
                            + (*(*encoding).data.range1.offset(i as isize)).n_left as ::core::ffi::c_int
                {
                    gid = (gid as ::core::ffi::c_int
                        + (code as ::core::ffi::c_int - (*(*encoding).data.range1.offset(i as isize)).first as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)) as card16;
                    break;
                } else {
                    gid = (gid as ::core::ffi::c_int
                        + ((*(*encoding).data.range1.offset(i as isize)).n_left as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                        as card16;
                    i = i.wrapping_add(1);
                }
            }
            if i as ::core::ffi::c_int == (*encoding).num_entries as ::core::ffi::c_int {
                gid = 0 as card16;
            }
        }
        _ => {
            pdf_abort!(b"Unknown Encoding format.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    if gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int && (*encoding).format as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
        let mut map: *mut cff_map = ::core::ptr::null_mut::<cff_map>();
        if (*encoding).supp.is_null() {
            pdf_abort!(b"No CFF supplementary encoding data read.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        map = (*encoding).supp;
        i = 0 as card16;
        while (i as ::core::ffi::c_int) < (*encoding).num_supps as ::core::ffi::c_int {
            if code as ::core::ffi::c_int == (*map.offset(i as isize)).code as ::core::ffi::c_int {
                gid = cff_charsets_lookup(cff, (*map.offset(i as isize)).glyph as card16);
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
    }
    return gid;
}
#[unsafe(export_name = "__pdf_impl_cff_release_encoding")]
pub unsafe extern "C-unwind" fn cff_release_encoding(mut encoding: *mut cff_encoding) {
    if !encoding.is_null() {
        match (*encoding).format as ::core::ffi::c_int & !(0x80 as ::core::ffi::c_int) {
            0 => {
                free((*encoding).data.codes as *mut ::core::ffi::c_void);
            }
            1 => {
                free((*encoding).data.range1 as *mut ::core::ffi::c_void);
            }
            _ => {
                pdf_abort!(b"Unknown Encoding format.\0" as *const u8 as *const ::core::ffi::c_char,);
            }
        }
        if (*encoding).format as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int != 0 {
            free((*encoding).supp as *mut ::core::ffi::c_void);
        }
        free(encoding as *mut ::core::ffi::c_void);
    }
}
#[unsafe(export_name = "__pdf_impl_cff_read_charsets")]
pub unsafe extern "C-unwind" fn cff_read_charsets(mut cff: *mut cff_font) -> ::core::ffi::c_int {
    let mut charset: *mut cff_charsets = ::core::ptr::null_mut::<cff_charsets>();
    let mut offset: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    let mut count: card16 = 0;
    let mut i: card16 = 0;
    if (*cff).topdict.is_null() {
        pdf_abort!(b"Top DICT not available\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    if cff_dict_known((*cff).topdict, b"charset\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*cff).flag |= CHARSETS_ISOADOBE;
        (*cff).charsets = ::core::ptr::null_mut::<cff_charsets>();
        return 0 as ::core::ffi::c_int;
    }
    offset = cff_dict_get(
        (*cff).topdict,
        b"charset\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if offset == 0 as ::core::ffi::c_int {
        (*cff).flag |= CHARSETS_ISOADOBE;
        (*cff).charsets = ::core::ptr::null_mut::<cff_charsets>();
        return 0 as ::core::ffi::c_int;
    } else if offset == 1 as ::core::ffi::c_int {
        (*cff).flag |= CHARSETS_EXPERT;
        (*cff).charsets = ::core::ptr::null_mut::<cff_charsets>();
        return 0 as ::core::ffi::c_int;
    } else if offset == 2 as ::core::ffi::c_int {
        (*cff).flag |= CHARSETS_EXPSUB;
        (*cff).charsets = ::core::ptr::null_mut::<cff_charsets>();
        return 0 as ::core::ffi::c_int;
    }
    let mut input = unsafe { seek_cff(cff, offset) };
    let (parsed, length) =
        crate::cff::charset(&mut input, unsafe { (*cff).num_glyphs }).unwrap_or_else(|error| pdf_abort!("{}", error.message.to_string_lossy()));
    unsafe {
        let value = match parsed {
            crate::cff::Charset::Glyphs(glyphs) => cff_charsets {
                format: 0,
                num_entries: glyphs.len() as u16,
                data: C2RustUnnamed_0 {
                    glyphs: copy_font_array(&glyphs),
                },
            },
            crate::cff::Charset::Ranges { format: 1, ranges } => {
                let ranges: Vec<_> = ranges
                    .into_iter()
                    .map(|(first, n_left)| cff_range1 { first, n_left: n_left as u8 })
                    .collect();
                cff_charsets {
                    format: 1,
                    num_entries: ranges.len() as u16,
                    data: C2RustUnnamed_0 {
                        range1: copy_font_array(&ranges),
                    },
                }
            }
            crate::cff::Charset::Ranges { ranges, .. } => {
                let ranges: Vec<_> = ranges.into_iter().map(|(first, n_left)| cff_range2 { first, n_left }).collect();
                cff_charsets {
                    format: 2,
                    num_entries: ranges.len() as u16,
                    data: C2RustUnnamed_0 {
                        range2: copy_font_array(&ranges),
                    },
                }
            }
        };
        (*cff).charsets = copy_font_array(&[value]);
    }
    length as i32
}
#[unsafe(export_name = "__pdf_impl_cff_pack_charsets")]
pub unsafe extern "C-unwind" fn cff_pack_charsets(
    mut cff: *mut cff_font, mut dest: *mut card8, mut destlen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: card16 = 0;
    let mut charset: *mut cff_charsets = ::core::ptr::null_mut::<cff_charsets>();
    if (*cff).flag & HAVE_STANDARD_CHARSETS != 0 || (*cff).charsets.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if destlen < 1 as ::core::ffi::c_int {
        pdf_abort!(b"in cff_pack_charsets(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    charset = (*cff).charsets;
    let fresh40 = len;
    len = len + 1;
    *dest.offset(fresh40 as isize) = (*charset).format;
    match (*charset).format as ::core::ffi::c_int {
        0 => {
            if destlen < len + (*charset).num_entries as ::core::ffi::c_int * 2 as ::core::ffi::c_int {
                pdf_abort!(b"in cff_pack_charsets(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                let mut sid: s_SID = *(*charset).data.glyphs.offset(i as isize);
                let fresh41 = len;
                len = len + 1;
                *dest.offset(fresh41 as isize) = (sid as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
                let fresh42 = len;
                len = len + 1;
                *dest.offset(fresh42 as isize) = (sid as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
                i = i.wrapping_add(1);
            }
        }
        1 => {
            if destlen < len + (*charset).num_entries as ::core::ffi::c_int * 3 as ::core::ffi::c_int {
                pdf_abort!(b"in cff_pack_charsets(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                let fresh43 = len;
                len = len + 1;
                *dest.offset(fresh43 as isize) = ((*(*charset).data.range1.offset(i as isize)).first as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as card8;
                let fresh44 = len;
                len = len + 1;
                *dest.offset(fresh44 as isize) =
                    ((*(*charset).data.range1.offset(i as isize)).first as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
                let fresh45 = len;
                len = len + 1;
                *dest.offset(fresh45 as isize) = (*(*charset).data.range1.offset(i as isize)).n_left;
                i = i.wrapping_add(1);
            }
        }
        2 => {
            if destlen < len + (*charset).num_entries as ::core::ffi::c_int * 4 as ::core::ffi::c_int {
                pdf_abort!(b"in cff_pack_charsets(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                let fresh46 = len;
                len = len + 1;
                *dest.offset(fresh46 as isize) = ((*(*charset).data.range2.offset(i as isize)).first as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as card8;
                let fresh47 = len;
                len = len + 1;
                *dest.offset(fresh47 as isize) =
                    ((*(*charset).data.range2.offset(i as isize)).first as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
                let fresh48 = len;
                len = len + 1;
                *dest.offset(fresh48 as isize) = ((*(*charset).data.range2.offset(i as isize)).n_left as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as card8;
                let fresh49 = len;
                len = len + 1;
                *dest.offset(fresh49 as isize) =
                    ((*(*charset).data.range2.offset(i as isize)).n_left as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
                i = i.wrapping_add(1);
            }
        }
        _ => {
            pdf_abort!(b"Unknown Charset format\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    return len;
}
#[unsafe(export_name = "__pdf_impl_cff_get_glyphname")]
pub unsafe extern "C-unwind" fn cff_get_glyphname(mut cff: *mut cff_font, mut gid: card16) -> *mut ::core::ffi::c_char {
    let mut sid: s_SID = 0;
    sid = cff_charsets_lookup_inverse(cff, gid) as s_SID;
    return cff_get_string(cff, sid);
}
#[unsafe(export_name = "__pdf_impl_cff_glyph_lookup")]
pub unsafe extern "C-unwind" fn cff_glyph_lookup(mut cff: *mut cff_font, mut glyph: *const ::core::ffi::c_char) -> card16 {
    let mut gid: card16 = 0;
    let mut charset: *mut cff_charsets = ::core::ptr::null_mut::<cff_charsets>();
    let mut i: card16 = 0;
    let mut n: card16 = 0;
    if (*cff).flag & (CHARSETS_ISOADOBE | CHARSETS_EXPERT | CHARSETS_EXPSUB) != 0 {
        pdf_abort!(b"Predefined CFF charsets not supported yet\0" as *const u8 as *const ::core::ffi::c_char,);
    } else if (*cff).charsets.is_null() {
        pdf_abort!(b"Charsets data not available\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    if glyph.is_null() || streq_ptr(glyph, b".notdef\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0 {
        return 0 as card16;
    }
    charset = (*cff).charsets;
    gid = 0 as card16;
    match (*charset).format as ::core::ffi::c_int {
        0 => {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                gid = gid.wrapping_add(1);
                if cff_match_string(cff, glyph, *(*charset).data.glyphs.offset(i as isize)) != 0 {
                    return gid;
                }
                i = i.wrapping_add(1);
            }
        }
        1 => {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                n = 0 as card16;
                while n as ::core::ffi::c_int <= (*(*charset).data.range1.offset(i as isize)).n_left as ::core::ffi::c_int {
                    gid = gid.wrapping_add(1);
                    if cff_match_string(
                        cff,
                        glyph,
                        ((*(*charset).data.range1.offset(i as isize)).first as ::core::ffi::c_int + n as ::core::ffi::c_int) as s_SID,
                    ) != 0
                    {
                        return gid;
                    }
                    n = n.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
        }
        2 => {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                n = 0 as card16;
                while n as ::core::ffi::c_int <= (*(*charset).data.range2.offset(i as isize)).n_left as ::core::ffi::c_int {
                    gid = gid.wrapping_add(1);
                    if cff_match_string(
                        cff,
                        glyph,
                        ((*(*charset).data.range2.offset(i as isize)).first as ::core::ffi::c_int + n as ::core::ffi::c_int) as s_SID,
                    ) != 0
                    {
                        return gid;
                    }
                    n = n.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
        }
        _ => {
            pdf_abort!(b"Unknown Charset format\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    return 0 as card16;
}
#[unsafe(export_name = "__pdf_impl_cff_charsets_lookup")]
pub unsafe extern "C-unwind" fn cff_charsets_lookup(mut cff: *mut cff_font, mut cid: card16) -> card16 {
    if (*cff).flag & (CHARSETS_ISOADOBE | CHARSETS_EXPERT | CHARSETS_EXPSUB) != 0 {
        pdf_abort!(b"Predefined CFF charsets not supported yet\0" as *const u8 as *const ::core::ffi::c_char,);
    } else if (*cff).charsets.is_null() {
        pdf_abort!(b"Charsets data not available\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    return cff_charsets_lookup_gid((*cff).charsets, cid);
}
#[unsafe(export_name = "__pdf_impl_cff_charsets_lookup_gid")]
pub unsafe extern "C-unwind" fn cff_charsets_lookup_gid(mut charset: *mut cff_charsets, mut cid: card16) -> card16 {
    let mut gid: card16 = 0 as card16;
    let mut i: card16 = 0;
    if cid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as card16;
    }
    match (*charset).format as ::core::ffi::c_int {
        0 => {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                if cid as ::core::ffi::c_int == *(*charset).data.glyphs.offset(i as isize) as ::core::ffi::c_int {
                    gid = (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as card16;
                    return gid;
                }
                i = i.wrapping_add(1);
            }
        }
        1 => {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                if cid as ::core::ffi::c_int >= (*(*charset).data.range1.offset(i as isize)).first as ::core::ffi::c_int
                    && cid as ::core::ffi::c_int
                        <= (*(*charset).data.range1.offset(i as isize)).first as ::core::ffi::c_int
                            + (*(*charset).data.range1.offset(i as isize)).n_left as ::core::ffi::c_int
                {
                    gid = (gid as ::core::ffi::c_int
                        + (cid as ::core::ffi::c_int - (*(*charset).data.range1.offset(i as isize)).first as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)) as card16;
                    return gid;
                }
                gid = (gid as ::core::ffi::c_int
                    + ((*(*charset).data.range1.offset(i as isize)).n_left as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                    as card16;
                i = i.wrapping_add(1);
            }
        }
        2 => {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                if cid as ::core::ffi::c_int >= (*(*charset).data.range2.offset(i as isize)).first as ::core::ffi::c_int
                    && cid as ::core::ffi::c_int
                        <= (*(*charset).data.range2.offset(i as isize)).first as ::core::ffi::c_int
                            + (*(*charset).data.range2.offset(i as isize)).n_left as ::core::ffi::c_int
                {
                    gid = (gid as ::core::ffi::c_int
                        + (cid as ::core::ffi::c_int - (*(*charset).data.range2.offset(i as isize)).first as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int)) as card16;
                    return gid;
                }
                gid = (gid as ::core::ffi::c_int
                    + ((*(*charset).data.range2.offset(i as isize)).n_left as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                    as card16;
                i = i.wrapping_add(1);
            }
        }
        _ => {
            pdf_abort!(b"Unknown Charset format\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    return 0 as card16;
}
#[unsafe(export_name = "__pdf_impl_cff_charsets_lookup_inverse")]
pub unsafe extern "C-unwind" fn cff_charsets_lookup_inverse(mut cff: *mut cff_font, mut gid: card16) -> card16 {
    if (*cff).flag & (CHARSETS_ISOADOBE | CHARSETS_EXPERT | CHARSETS_EXPSUB) != 0 {
        pdf_abort!(b"Predefined CFF charsets not supported yet\0" as *const u8 as *const ::core::ffi::c_char,);
    } else if (*cff).charsets.is_null() {
        pdf_abort!(b"Charsets data not available\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    if gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as card16;
    }
    return cff_charsets_lookup_cid((*cff).charsets, gid);
}
#[unsafe(export_name = "__pdf_impl_cff_charsets_lookup_cid")]
pub unsafe extern "C-unwind" fn cff_charsets_lookup_cid(mut charset: *mut cff_charsets, mut gid: card16) -> card16 {
    let mut sid: card16 = 0 as card16;
    let mut i: card16 = 0;
    match (*charset).format as ::core::ffi::c_int {
        0 => {
            if gid as ::core::ffi::c_int - 1 as ::core::ffi::c_int >= (*charset).num_entries as ::core::ffi::c_int {
                pdf_abort!(b"Invalid GID.\0" as *const u8 as *const ::core::ffi::c_char);
            }
            sid = *(*charset)
                .data
                .glyphs
                .offset((gid as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) as card16;
        }
        1 => {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                if gid as ::core::ffi::c_int <= (*(*charset).data.range1.offset(i as isize)).n_left as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
                    sid = (gid as ::core::ffi::c_int + (*(*charset).data.range1.offset(i as isize)).first as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as card16;
                    break;
                } else {
                    gid = (gid as ::core::ffi::c_int
                        - ((*(*charset).data.range1.offset(i as isize)).n_left as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                        as card16;
                    i = i.wrapping_add(1);
                }
            }
            if i as ::core::ffi::c_int == (*charset).num_entries as ::core::ffi::c_int {
                pdf_abort!(b"Invalid GID\0" as *const u8 as *const ::core::ffi::c_char);
            }
        }
        2 => {
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*charset).num_entries as ::core::ffi::c_int {
                if gid as ::core::ffi::c_int <= (*(*charset).data.range2.offset(i as isize)).n_left as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
                    sid = (gid as ::core::ffi::c_int + (*(*charset).data.range2.offset(i as isize)).first as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as card16;
                    break;
                } else {
                    gid = (gid as ::core::ffi::c_int
                        - ((*(*charset).data.range2.offset(i as isize)).n_left as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                        as card16;
                    i = i.wrapping_add(1);
                }
            }
            if i as ::core::ffi::c_int == (*charset).num_entries as ::core::ffi::c_int {
                pdf_abort!(b"Invalid GID\0" as *const u8 as *const ::core::ffi::c_char);
            }
        }
        _ => {
            pdf_abort!(b"Unknown Charset format\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    return sid;
}
#[unsafe(export_name = "__pdf_impl_cff_release_charsets")]
pub unsafe extern "C-unwind" fn cff_release_charsets(mut charset: *mut cff_charsets) {
    if !charset.is_null() {
        match (*charset).format as ::core::ffi::c_int {
            0 => {
                free((*charset).data.glyphs as *mut ::core::ffi::c_void);
            }
            1 => {
                free((*charset).data.range1 as *mut ::core::ffi::c_void);
            }
            2 => {
                free((*charset).data.range2 as *mut ::core::ffi::c_void);
            }
            _ => {}
        }
        free(charset as *mut ::core::ffi::c_void);
    }
}
#[unsafe(export_name = "__pdf_impl_cff_read_fdselect")]
pub unsafe extern "C-unwind" fn cff_read_fdselect(mut cff: *mut cff_font) -> ::core::ffi::c_int {
    let mut fdsel: *mut cff_fdselect = ::core::ptr::null_mut::<cff_fdselect>();
    let mut offset: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    let mut i: card16 = 0;
    if (*cff).topdict.is_null() {
        pdf_abort!(b"Top DICT not available\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    if (*cff).flag & FONTTYPE_CIDFONT == 0 {
        return 0 as ::core::ffi::c_int;
    }
    offset = cff_dict_get(
        (*cff).topdict,
        b"FDSelect\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    let mut input = unsafe { seek_cff(cff, offset) };
    let (parsed, length) =
        crate::cff::fdselect(&mut input, unsafe { (*cff).num_glyphs }).unwrap_or_else(|error| pdf_abort!("{}", error.message.to_string_lossy()));
    unsafe {
        let value = match parsed {
            crate::cff::FdSelect::Fds(fds) => cff_fdselect {
                format: 0,
                num_entries: fds.len() as u16,
                data: C2RustUnnamed_1 { fds: copy_font_array(&fds) },
            },
            crate::cff::FdSelect::Ranges(ranges) => {
                let ranges: Vec<_> = ranges.into_iter().map(|(first, fd)| cff_range3 { first, fd }).collect();
                cff_fdselect {
                    format: 3,
                    num_entries: ranges.len() as u16,
                    data: C2RustUnnamed_1 {
                        ranges: copy_font_array(&ranges),
                    },
                }
            }
        };
        (*cff).fdselect = copy_font_array(&[value]);
    }
    length as i32
}
#[unsafe(export_name = "__pdf_impl_cff_pack_fdselect")]
pub unsafe extern "C-unwind" fn cff_pack_fdselect(
    mut cff: *mut cff_font, mut dest: *mut card8, mut destlen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fdsel: *mut cff_fdselect = ::core::ptr::null_mut::<cff_fdselect>();
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: card16 = 0;
    if (*cff).fdselect.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if destlen < 1 as ::core::ffi::c_int {
        pdf_abort!(b"in cff_pack_fdselect(): Buffur overflow\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    fdsel = (*cff).fdselect;
    let fresh50 = len;
    len = len + 1;
    *dest.offset(fresh50 as isize) = (*fdsel).format;
    match (*fdsel).format as ::core::ffi::c_int {
        0 => {
            if (*fdsel).num_entries as ::core::ffi::c_int != (*cff).num_glyphs as ::core::ffi::c_int {
                pdf_abort!(b"in cff_pack_fdselect(): Invalid data\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            if destlen < len + (*fdsel).num_entries as ::core::ffi::c_int {
                pdf_abort!(b"in cff_pack_fdselect(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*fdsel).num_entries as ::core::ffi::c_int {
                let fresh51 = len;
                len = len + 1;
                *dest.offset(fresh51 as isize) = *(*fdsel).data.fds.offset(i as isize);
                i = i.wrapping_add(1);
            }
        }
        3 => {
            if destlen < len + 2 as ::core::ffi::c_int {
                pdf_abort!(b"in cff_pack_fdselect(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            len += 2 as ::core::ffi::c_int;
            i = 0 as card16;
            while (i as ::core::ffi::c_int) < (*fdsel).num_entries as ::core::ffi::c_int {
                if destlen < len + 3 as ::core::ffi::c_int {
                    pdf_abort!(b"in cff_pack_fdselect(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                let fresh52 = len;
                len = len + 1;
                *dest.offset(fresh52 as isize) = ((*(*fdsel).data.ranges.offset(i as isize)).first as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as card8;
                let fresh53 = len;
                len = len + 1;
                *dest.offset(fresh53 as isize) =
                    ((*(*fdsel).data.ranges.offset(i as isize)).first as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
                let fresh54 = len;
                len = len + 1;
                *dest.offset(fresh54 as isize) = (*(*fdsel).data.ranges.offset(i as isize)).fd;
                i = i.wrapping_add(1);
            }
            if destlen < len + 2 as ::core::ffi::c_int {
                pdf_abort!(b"in cff_pack_fdselect(): Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            let fresh55 = len;
            len = len + 1;
            *dest.offset(fresh55 as isize) =
                ((*cff).num_glyphs as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh56 = len;
            len = len + 1;
            *dest.offset(fresh56 as isize) = ((*cff).num_glyphs as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            *dest.offset(1 as ::core::ffi::c_int as isize) =
                (len / 3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            *dest.offset(2 as ::core::ffi::c_int as isize) =
                (len / 3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
        }
        _ => {
            pdf_abort!(b"Unknown FDSelect format.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    return len;
}
#[unsafe(export_name = "__pdf_impl_cff_release_fdselect")]
pub unsafe extern "C-unwind" fn cff_release_fdselect(mut fdselect: *mut cff_fdselect) {
    if !fdselect.is_null() {
        if (*fdselect).format as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            free((*fdselect).data.fds as *mut ::core::ffi::c_void);
        } else if (*fdselect).format as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            free((*fdselect).data.ranges as *mut ::core::ffi::c_void);
        }
        free(fdselect as *mut ::core::ffi::c_void);
    }
}
#[unsafe(export_name = "__pdf_impl_cff_fdselect_lookup")]
pub unsafe extern "C-unwind" fn cff_fdselect_lookup(mut cff: *mut cff_font, mut gid: card16) -> card8 {
    let mut fd: card8 = 0xff as card8;
    let mut fdsel: *mut cff_fdselect = ::core::ptr::null_mut::<cff_fdselect>();
    if (*cff).fdselect.is_null() {
        pdf_abort!(b"in cff_fdselect_lookup(): FDSelect not available\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    fdsel = (*cff).fdselect;
    if gid as ::core::ffi::c_int >= (*cff).num_glyphs as ::core::ffi::c_int {
        pdf_abort!(b"in cff_fdselect_lookup(): Invalid glyph index\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    match (*fdsel).format as ::core::ffi::c_int {
        0 => {
            fd = *(*fdsel).data.fds.offset(gid as isize);
        }
        3 => {
            if gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                fd = (*(*fdsel).data.ranges.offset(0 as ::core::ffi::c_int as isize)).fd;
            } else {
                let mut i: card16 = 0;
                i = 1 as card16;
                while (i as ::core::ffi::c_int) < (*fdsel).num_entries as ::core::ffi::c_int {
                    if (gid as ::core::ffi::c_int) < (*(*fdsel).data.ranges.offset(i as isize)).first as ::core::ffi::c_int {
                        break;
                    }
                    i = i.wrapping_add(1);
                }
                fd = (*(*fdsel).data.ranges.offset((i as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)).fd;
            }
        }
        _ => {
            pdf_abort!(b"in cff_fdselect_lookup(): Invalid FDSelect format\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    if fd as ::core::ffi::c_int >= (*cff).num_fds as ::core::ffi::c_int {
        pdf_abort!(b"in cff_fdselect_lookup(): Invalid Font DICT index\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    return fd;
}
#[unsafe(export_name = "__pdf_impl_cff_read_subrs")]
pub unsafe extern "C-unwind" fn cff_read_subrs(mut cff: *mut cff_font) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut offset: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if (*cff).flag & FONTTYPE_CIDFONT != 0 && (*cff).fdarray.is_null() {
        cff_read_fdarray(cff);
    }
    if (*cff).private.is_null() {
        cff_read_private(cff);
    }
    if (*cff).gsubr.is_null() {
        crate::backend::runtime::input_seek((*cff).handle, (*cff).offset.wrapping_add((*cff).gsubr_offset) as ssize_t, SEEK_SET);
        (*cff).gsubr = cff_get_index(cff);
    }
    (*cff).subrs =
        new(((*cff).num_fds as uint32_t as usize).wrapping_mul(::core::mem::size_of::<*mut cff_index>() as usize) as uint32_t) as *mut *mut cff_index;
    if (*cff).flag & FONTTYPE_CIDFONT != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < (*cff).num_fds as ::core::ffi::c_int {
            if (*(*cff).private.offset(i as isize)).is_null()
                || cff_dict_known(*(*cff).private.offset(i as isize), b"Subrs\0" as *const u8 as *const ::core::ffi::c_char) == 0
            {
                let ref mut fresh21 = *(*cff).subrs.offset(i as isize);
                *fresh21 = ::core::ptr::null_mut::<cff_index>();
            } else {
                offset = cff_dict_get(
                    *(*cff).fdarray.offset(i as isize),
                    b"Private\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
                offset = (offset as ::core::ffi::c_double
                    + cff_dict_get(
                        *(*cff).private.offset(i as isize),
                        b"Subrs\0" as *const u8 as *const ::core::ffi::c_char,
                        0 as ::core::ffi::c_int,
                    )) as ::core::ffi::c_int;
                crate::backend::runtime::input_seek((*cff).handle, (*cff).offset.wrapping_add(offset as l_offset) as ssize_t, SEEK_SET);
                let ref mut fresh22 = *(*cff).subrs.offset(i as isize);
                *fresh22 = cff_get_index(cff);
                len += cff_index_size(*(*cff).subrs.offset(i as isize));
            }
            i += 1;
        }
    } else if (*(*cff).private.offset(0 as ::core::ffi::c_int as isize)).is_null()
        || cff_dict_known(
            *(*cff).private.offset(0 as ::core::ffi::c_int as isize),
            b"Subrs\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        let ref mut fresh23 = *(*cff).subrs.offset(0 as ::core::ffi::c_int as isize);
        *fresh23 = ::core::ptr::null_mut::<cff_index>();
    } else {
        offset = cff_dict_get(
            (*cff).topdict,
            b"Private\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        offset = (offset as ::core::ffi::c_double
            + cff_dict_get(
                *(*cff).private.offset(0 as ::core::ffi::c_int as isize),
                b"Subrs\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            )) as ::core::ffi::c_int;
        crate::backend::runtime::input_seek((*cff).handle, (*cff).offset.wrapping_add(offset as l_offset) as ssize_t, SEEK_SET);
        let ref mut fresh24 = *(*cff).subrs.offset(0 as ::core::ffi::c_int as isize);
        *fresh24 = cff_get_index(cff);
        len += cff_index_size(*(*cff).subrs.offset(0 as ::core::ffi::c_int as isize));
    }
    return len;
}
#[unsafe(export_name = "__pdf_impl_cff_read_fdarray")]
pub unsafe extern "C-unwind" fn cff_read_fdarray(mut cff: *mut cff_font) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut idx: *mut cff_index = ::core::ptr::null_mut::<cff_index>();
    let mut offset: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    let mut i: card16 = 0;
    if (*cff).topdict.is_null() {
        pdf_abort!(b"in cff_read_fdarray(): Top DICT not found\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    if (*cff).flag & FONTTYPE_CIDFONT == 0 {
        return 0 as ::core::ffi::c_int;
    }
    offset = cff_dict_get(
        (*cff).topdict,
        b"FDArray\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    crate::backend::runtime::input_seek((*cff).handle, (*cff).offset.wrapping_add(offset as l_offset) as ssize_t, SEEK_SET);
    idx = cff_get_index(cff);
    (*cff).num_fds = (*idx).count as card8;
    (*cff).fdarray =
        new(((*idx).count as uint32_t as usize).wrapping_mul(::core::mem::size_of::<*mut cff_dict>() as usize) as uint32_t) as *mut *mut cff_dict;
    i = 0 as card16;
    while (i as ::core::ffi::c_int) < (*idx).count as ::core::ffi::c_int {
        let mut data: *mut card8 = (*idx)
            .data
            .offset(*(*idx).offset.offset(i as isize) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        size = (*(*idx).offset.offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)).wrapping_sub(*(*idx).offset.offset(i as isize))
            as ::core::ffi::c_int;
        if size > 0 as ::core::ffi::c_int {
            let ref mut fresh29 = *(*cff).fdarray.offset(i as isize);
            *fresh29 = cff_dict_unpack(data, data.offset(size as isize));
        } else {
            let ref mut fresh30 = *(*cff).fdarray.offset(i as isize);
            *fresh30 = ::core::ptr::null_mut::<cff_dict>();
        }
        i = i.wrapping_add(1);
    }
    len = cff_index_size(idx);
    cff_release_index(idx);
    return len;
}
#[unsafe(export_name = "__pdf_impl_cff_read_private")]
pub unsafe extern "C-unwind" fn cff_read_private(mut cff: *mut cff_font) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut data: *mut card8 = ::core::ptr::null_mut::<card8>();
    let mut offset: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    if (*cff).flag & FONTTYPE_CIDFONT != 0 {
        let mut i: ::core::ffi::c_int = 0;
        if (*cff).fdarray.is_null() {
            cff_read_fdarray(cff);
        }
        (*cff).private = new(((*cff).num_fds as uint32_t as usize).wrapping_mul(::core::mem::size_of::<*mut cff_dict>() as usize) as uint32_t)
            as *mut *mut cff_dict;
        i = 0 as ::core::ffi::c_int;
        while i < (*cff).num_fds as ::core::ffi::c_int {
            if !(*(*cff).fdarray.offset(i as isize)).is_null()
                && cff_dict_known(
                    *(*cff).fdarray.offset(i as isize),
                    b"Private\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                && {
                    size = cff_dict_get(
                        *(*cff).fdarray.offset(i as isize),
                        b"Private\0" as *const u8 as *const ::core::ffi::c_char,
                        0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    size > 0 as ::core::ffi::c_int
                }
            {
                offset = cff_dict_get(
                    *(*cff).fdarray.offset(i as isize),
                    b"Private\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
                crate::backend::runtime::input_seek((*cff).handle, (*cff).offset.wrapping_add(offset as l_offset) as ssize_t, SEEK_SET);
                data = new((size as uint32_t as usize).wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t) as *mut card8;
                if ttstub_input_read((*cff).handle, data as *mut ::core::ffi::c_char, size as size_t) != size as ssize_t {
                    pdf_abort!(b"reading file failed\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                let ref mut fresh25 = *(*cff).private.offset(i as isize);
                *fresh25 = cff_dict_unpack(data, data.offset(size as isize));
                free(data as *mut ::core::ffi::c_void);
                len += size;
            } else {
                let ref mut fresh26 = *(*cff).private.offset(i as isize);
                *fresh26 = ::core::ptr::null_mut::<cff_dict>();
            }
            i += 1;
        }
    } else {
        (*cff).num_fds = 1 as card8;
        (*cff).private =
            new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<*mut cff_dict>() as usize) as uint32_t)
                as *mut *mut cff_dict;
        if cff_dict_known((*cff).topdict, b"Private\0" as *const u8 as *const ::core::ffi::c_char) != 0 && {
            size = cff_dict_get(
                (*cff).topdict,
                b"Private\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            size > 0 as ::core::ffi::c_int
        } {
            offset = cff_dict_get(
                (*cff).topdict,
                b"Private\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            crate::backend::runtime::input_seek((*cff).handle, (*cff).offset.wrapping_add(offset as l_offset) as ssize_t, SEEK_SET);
            data = new((size as uint32_t as usize).wrapping_mul(::core::mem::size_of::<card8>() as usize) as uint32_t) as *mut card8;
            if ttstub_input_read((*cff).handle, data as *mut ::core::ffi::c_char, size as size_t) != size as ssize_t {
                pdf_abort!(b"reading file failed\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            let ref mut fresh27 = *(*cff).private.offset(0 as ::core::ffi::c_int as isize);
            *fresh27 = cff_dict_unpack(data, data.offset(size as isize));
            free(data as *mut ::core::ffi::c_void);
            len += size;
        } else {
            let ref mut fresh28 = *(*cff).private.offset(0 as ::core::ffi::c_int as isize);
            *fresh28 = ::core::ptr::null_mut::<cff_dict>();
            len = 0 as ::core::ffi::c_int;
        }
    }
    return len;
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn memcmp(__s1: *const ::core::ffi::c_void, __s2: *const ::core::ffi::c_void, __n: size_t) -> ::core::ffi::c_int;
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
    fn memmove(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __len: size_t) -> *mut ::core::ffi::c_void;
    fn memset(__b: *mut ::core::ffi::c_void, __c: ::core::ffi::c_int, __len: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_open(
    mut handle: rust_input_handle_t, mut offset: ::core::ffi::c_int, mut n: ::core::ffi::c_int, _result: *mut *mut cff_font,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_open(handle, offset, n) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_close(mut cff: *mut cff_font) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_close(cff) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_get_name(mut cff: *mut cff_font, _result: *mut *mut ::core::ffi::c_char) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_get_name(cff) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_set_name(
    mut cff: *mut cff_font, mut name: *mut ::core::ffi::c_char, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_set_name(cff, name) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_put_header(
    mut cff: *mut cff_font, mut dest: *mut card8, mut destlen: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_put_header(cff, dest, destlen) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_get_index_header(mut cff: *mut cff_font, _result: *mut *mut cff_index) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_get_index_header(cff) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_get_index(mut cff: *mut cff_font, _result: *mut *mut cff_index) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_get_index(cff) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_pack_index(
    mut idx: *mut cff_index, mut dest: *mut card8, mut destlen: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_pack_index(idx, dest, destlen) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_index_size(mut idx: *mut cff_index, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_index_size(idx) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_new_index(mut count: card16, _result: *mut *mut cff_index) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_new_index(count) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_release_index(mut idx: *mut cff_index) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_release_index(idx) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_get_string(mut cff: *mut cff_font, mut id: s_SID, _result: *mut *mut ::core::ffi::c_char) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_get_string(cff, id) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_get_sid(
    mut cff: *mut cff_font, mut str: *const ::core::ffi::c_char, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_get_sid(cff, str) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_get_seac_sid(
    mut cff: *mut cff_font, mut str: *const ::core::ffi::c_char, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_get_seac_sid(cff, str) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_update_string(mut cff: *mut cff_font) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_update_string(cff) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_add_string(
    mut cff: *mut cff_font, mut str: *const ::core::ffi::c_char, mut unique: ::core::ffi::c_int, _result: *mut s_SID,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_add_string(cff, str, unique) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_read_encoding(mut cff: *mut cff_font, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_read_encoding(cff) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_pack_encoding(
    mut cff: *mut cff_font, mut dest: *mut card8, mut destlen: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_pack_encoding(cff, dest, destlen) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_encoding_lookup(mut cff: *mut cff_font, mut code: card8, _result: *mut card16) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_encoding_lookup(cff, code) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_release_encoding(mut encoding: *mut cff_encoding) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_release_encoding(encoding) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_read_charsets(mut cff: *mut cff_font, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_read_charsets(cff) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_pack_charsets(
    mut cff: *mut cff_font, mut dest: *mut card8, mut destlen: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_pack_charsets(cff, dest, destlen) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_get_glyphname(mut cff: *mut cff_font, mut gid: card16, _result: *mut *mut ::core::ffi::c_char) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_get_glyphname(cff, gid) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_glyph_lookup(mut cff: *mut cff_font, mut glyph: *const ::core::ffi::c_char, _result: *mut card16) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_glyph_lookup(cff, glyph) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_charsets_lookup(mut cff: *mut cff_font, mut cid: card16, _result: *mut card16) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_charsets_lookup(cff, cid) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_charsets_lookup_gid(mut charset: *mut cff_charsets, mut cid: card16, _result: *mut card16) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_charsets_lookup_gid(charset, cid) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_charsets_lookup_inverse(mut cff: *mut cff_font, mut gid: card16, _result: *mut card16) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_charsets_lookup_inverse(cff, gid) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_charsets_lookup_cid(mut charset: *mut cff_charsets, mut gid: card16, _result: *mut card16) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_charsets_lookup_cid(charset, gid) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_release_charsets(mut charset: *mut cff_charsets) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_release_charsets(charset) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_read_fdselect(mut cff: *mut cff_font, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_read_fdselect(cff) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_pack_fdselect(
    mut cff: *mut cff_font, mut dest: *mut card8, mut destlen: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_pack_fdselect(cff, dest, destlen) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_release_fdselect(mut fdselect: *mut cff_fdselect) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_release_fdselect(fdselect) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_fdselect_lookup(mut cff: *mut cff_font, mut gid: card16, _result: *mut card8) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_fdselect_lookup(cff, gid) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_read_subrs(mut cff: *mut cff_font, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_read_subrs(cff) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_read_fdarray(mut cff: *mut cff_font, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_read_fdarray(cff) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_read_private(mut cff: *mut cff_font, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_read_private(cff) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
