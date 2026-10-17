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
    fn __error() -> *mut ::core::ffi::c_int;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strtod(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    static mut work_buffer: [::core::ffi::c_char; 0];
    #[link_name = "__pdf_impl_cff_get_string"]
    fn cff_get_string(cff: *mut cff_font, id: s_SID) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_cff_add_string"]
    fn cff_add_string(
        cff: *mut cff_font,
        str: *const ::core::ffi::c_char,
        unique: ::core::ffi::c_int,
    ) -> s_SID;
}
pub type __darwin_size_t = usize;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub opname: *const ::core::ffi::c_char,
    pub argtype: ::core::ffi::c_int,
}
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const CFF_INT_MAX: ::core::ffi::c_int = 0x7fffffff as ::core::ffi::c_int;
pub const CFF_INT_MIN: ::core::ffi::c_int =
    -(0x7fffffff as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
pub const CFF_DEBUG_STR: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"CFF\0") };
pub const CFF_TYPE_INTEGER: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const CFF_TYPE_REAL: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const CFF_TYPE_NUMBER: ::core::ffi::c_int = CFF_TYPE_INTEGER | CFF_TYPE_REAL;
pub const CFF_TYPE_BOOLEAN: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const CFF_TYPE_SID: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const CFF_TYPE_ARRAY: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const CFF_TYPE_DELTA: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const CFF_TYPE_ROS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const CFF_TYPE_OFFSET: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
pub const CFF_TYPE_SZOFF: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const WORK_BUFFER_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const CFF_PARSE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CFF_ERROR_PARSE_ERROR: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const CFF_ERROR_STACK_OVERFLOW: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const CFF_ERROR_STACK_UNDERFLOW: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const DICT_ENTRY_MAX: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_cff_new_dict")]
pub unsafe extern "C-unwind" fn cff_new_dict() -> *mut cff_dict {
    let mut dict: *mut cff_dict = ::core::ptr::null_mut::<cff_dict>();
    dict = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_dict>() as usize) as uint32_t)
        as *mut cff_dict;
    (*dict).max = DICT_ENTRY_MAX;
    (*dict).count = 0 as ::core::ffi::c_int;
    (*dict).entries = new(((*dict).max as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<cff_dict_entry>() as usize)
        as uint32_t) as *mut cff_dict_entry;
    return dict;
}
#[unsafe(export_name = "__pdf_impl_cff_release_dict")]
pub unsafe extern "C-unwind" fn cff_release_dict(mut dict: *mut cff_dict) {
    if !dict.is_null() {
        if !(*dict).entries.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < (*dict).count {
                free((*(*dict).entries.offset(i as isize)).values as *mut ::core::ffi::c_void);
                i += 1;
            }
            free((*dict).entries as *mut ::core::ffi::c_void);
        }
        free(dict as *mut ::core::ffi::c_void);
    }
}
pub const CFF_DICT_STACK_LIMIT: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
static mut stack_top: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut arg_stack: [::core::ffi::c_double; 64] = [0.; 64];
pub const CFF_LAST_DICT_OP1: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const CFF_LAST_DICT_OP2: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const CFF_LAST_DICT_OP: ::core::ffi::c_int = CFF_LAST_DICT_OP1 + CFF_LAST_DICT_OP2;
static mut dict_operator: [C2RustUnnamed_2; 61] = [
    C2RustUnnamed_2 {
        opname: b"version\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_SID,
    },
    C2RustUnnamed_2 {
        opname: b"Notice\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_SID,
    },
    C2RustUnnamed_2 {
        opname: b"FullName\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_SID,
    },
    C2RustUnnamed_2 {
        opname: b"FamilyName\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_SID,
    },
    C2RustUnnamed_2 {
        opname: b"Weight\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_SID,
    },
    C2RustUnnamed_2 {
        opname: b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_ARRAY,
    },
    C2RustUnnamed_2 {
        opname: b"BlueValues\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_DELTA,
    },
    C2RustUnnamed_2 {
        opname: b"OtherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_DELTA,
    },
    C2RustUnnamed_2 {
        opname: b"FamilyBlues\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_DELTA,
    },
    C2RustUnnamed_2 {
        opname: b"FamilyOtherBlues\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_DELTA,
    },
    C2RustUnnamed_2 {
        opname: b"StdHW\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"StdVW\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: ::core::ptr::null::<::core::ffi::c_char>(),
        argtype: -(1 as ::core::ffi::c_int),
    },
    C2RustUnnamed_2 {
        opname: b"UniqueID\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"XUID\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_ARRAY,
    },
    C2RustUnnamed_2 {
        opname: b"charset\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_OFFSET,
    },
    C2RustUnnamed_2 {
        opname: b"Encoding\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_OFFSET,
    },
    C2RustUnnamed_2 {
        opname: b"CharStrings\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_OFFSET,
    },
    C2RustUnnamed_2 {
        opname: b"Private\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_SZOFF,
    },
    C2RustUnnamed_2 {
        opname: b"Subrs\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_OFFSET,
    },
    C2RustUnnamed_2 {
        opname: b"defaultWidthX\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"nominalWidthX\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"Copyright\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_SID,
    },
    C2RustUnnamed_2 {
        opname: b"IsFixedPitch\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_BOOLEAN,
    },
    C2RustUnnamed_2 {
        opname: b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"UnderlinePosition\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"UnderlineThickness\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"PaintType\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"CharstringType\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"FontMatrix\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_ARRAY,
    },
    C2RustUnnamed_2 {
        opname: b"StrokeWidth\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"BlueScale\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"BlueShift\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"BlueFuzz\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"StemSnapH\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_DELTA,
    },
    C2RustUnnamed_2 {
        opname: b"StemSnapV\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_DELTA,
    },
    C2RustUnnamed_2 {
        opname: b"ForceBold\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_BOOLEAN,
    },
    C2RustUnnamed_2 {
        opname: ::core::ptr::null::<::core::ffi::c_char>(),
        argtype: -(1 as ::core::ffi::c_int),
    },
    C2RustUnnamed_2 {
        opname: ::core::ptr::null::<::core::ffi::c_char>(),
        argtype: -(1 as ::core::ffi::c_int),
    },
    C2RustUnnamed_2 {
        opname: b"LanguageGroup\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"ExpansionFactor\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"InitialRandomSeed\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"SyntheticBase\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"PostScript\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_SID,
    },
    C2RustUnnamed_2 {
        opname: b"BaseFontName\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_SID,
    },
    C2RustUnnamed_2 {
        opname: b"BaseFontBlend\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_DELTA,
    },
    C2RustUnnamed_2 {
        opname: ::core::ptr::null::<::core::ffi::c_char>(),
        argtype: -(1 as ::core::ffi::c_int),
    },
    C2RustUnnamed_2 {
        opname: ::core::ptr::null::<::core::ffi::c_char>(),
        argtype: -(1 as ::core::ffi::c_int),
    },
    C2RustUnnamed_2 {
        opname: ::core::ptr::null::<::core::ffi::c_char>(),
        argtype: -(1 as ::core::ffi::c_int),
    },
    C2RustUnnamed_2 {
        opname: ::core::ptr::null::<::core::ffi::c_char>(),
        argtype: -(1 as ::core::ffi::c_int),
    },
    C2RustUnnamed_2 {
        opname: ::core::ptr::null::<::core::ffi::c_char>(),
        argtype: -(1 as ::core::ffi::c_int),
    },
    C2RustUnnamed_2 {
        opname: ::core::ptr::null::<::core::ffi::c_char>(),
        argtype: -(1 as ::core::ffi::c_int),
    },
    C2RustUnnamed_2 {
        opname: b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_ROS,
    },
    C2RustUnnamed_2 {
        opname: b"CIDFontVersion\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"CIDFontRevision\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"CIDFontType\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"CIDCount\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"UIDBase\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_NUMBER,
    },
    C2RustUnnamed_2 {
        opname: b"FDArray\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_OFFSET,
    },
    C2RustUnnamed_2 {
        opname: b"FDSelect\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_OFFSET,
    },
    C2RustUnnamed_2 {
        opname: b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
        argtype: CFF_TYPE_SID,
    },
];
unsafe extern "C-unwind" fn get_integer(
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
    mut status: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b0: card8 = 0;
    let mut b1: card8 = 0;
    let mut b2: card8 = 0;
    let fresh4 = *data;
    *data = (*data).offset(1);
    b0 = *fresh4;
    if b0 as ::core::ffi::c_int == 28 as ::core::ffi::c_int
        && *data < endptr.offset(-(2 as ::core::ffi::c_int as isize))
    {
        let fresh5 = *data;
        *data = (*data).offset(1);
        b1 = *fresh5;
        let fresh6 = *data;
        *data = (*data).offset(1);
        b2 = *fresh6;
        result = b1 as ::core::ffi::c_int * 256 as ::core::ffi::c_int + b2 as ::core::ffi::c_int;
        if result as ::core::ffi::c_long > 0x7fff as ::core::ffi::c_long {
            result = (result as ::core::ffi::c_long - 0x10000 as ::core::ffi::c_long)
                as ::core::ffi::c_int;
        }
    } else if b0 as ::core::ffi::c_int == 29 as ::core::ffi::c_int
        && *data < endptr.offset(-(4 as ::core::ffi::c_int as isize))
    {
        let mut i: ::core::ffi::c_int = 0;
        let fresh7 = *data;
        *data = (*data).offset(1);
        result = *fresh7 as ::core::ffi::c_int;
        if result > 0x7f as ::core::ffi::c_int {
            result -= 0x100 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            result = result * 256 as ::core::ffi::c_int + **data as ::core::ffi::c_int;
            *data = (*data).offset(1 as ::core::ffi::c_int as isize);
            i += 1;
        }
    } else if b0 as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 246 as ::core::ffi::c_int
    {
        result = b0 as ::core::ffi::c_int - 139 as ::core::ffi::c_int;
    } else if b0 as ::core::ffi::c_int >= 247 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 250 as ::core::ffi::c_int
    {
        let fresh8 = *data;
        *data = (*data).offset(1);
        b1 = *fresh8;
        result = (b0 as ::core::ffi::c_int - 247 as ::core::ffi::c_int) * 256 as ::core::ffi::c_int
            + b1 as ::core::ffi::c_int
            + 108 as ::core::ffi::c_int;
    } else if b0 as ::core::ffi::c_int >= 251 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 254 as ::core::ffi::c_int
    {
        let fresh9 = *data;
        *data = (*data).offset(1);
        b1 = *fresh9;
        result = -(b0 as ::core::ffi::c_int - 251 as ::core::ffi::c_int)
            * 256 as ::core::ffi::c_int
            - b1 as ::core::ffi::c_int
            - 108 as ::core::ffi::c_int;
    } else {
        *status = CFF_ERROR_PARSE_ERROR;
    }
    return result as ::core::ffi::c_double;
}
unsafe extern "C-unwind" fn get_real(
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
    mut status: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut result: ::core::ffi::c_double = 0.0f64;
    let mut nibble: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fail: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if **data as ::core::ffi::c_int != 30 as ::core::ffi::c_int
        || *data >= endptr.offset(-(1 as ::core::ffi::c_int as isize))
    {
        *status = CFF_ERROR_PARSE_ERROR;
        return 0.0f64;
    }
    *data = (*data).offset(1 as ::core::ffi::c_int as isize);
    pos = 0 as ::core::ffi::c_int;
    while fail == 0 && len < WORK_BUFFER_SIZE - 2 as ::core::ffi::c_int && *data < endptr {
        if pos % 2 as ::core::ffi::c_int != 0 {
            nibble = **data as ::core::ffi::c_int & 0xf as ::core::ffi::c_int;
            *data = (*data).offset(1 as ::core::ffi::c_int as isize);
        } else {
            nibble =
                **data as ::core::ffi::c_int >> 4 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int;
        }
        if nibble >= 0 as ::core::ffi::c_int && nibble <= 0x9 as ::core::ffi::c_int {
            let fresh10 = len;
            len = len + 1;
            *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh10 as isize) =
                (nibble + '0' as i32) as ::core::ffi::c_char;
        } else if nibble == 0xa as ::core::ffi::c_int {
            let fresh11 = len;
            len = len + 1;
            *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh11 as isize) =
                '.' as i32 as ::core::ffi::c_char;
        } else if nibble == 0xb as ::core::ffi::c_int || nibble == 0xc as ::core::ffi::c_int {
            let fresh12 = len;
            len = len + 1;
            *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh12 as isize) =
                'e' as i32 as ::core::ffi::c_char;
            if nibble == 0xc as ::core::ffi::c_int {
                let fresh13 = len;
                len = len + 1;
                *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh13 as isize) =
                    '-' as i32 as ::core::ffi::c_char;
            }
        } else if nibble == 0xe as ::core::ffi::c_int {
            let fresh14 = len;
            len = len + 1;
            *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh14 as isize) =
                '-' as i32 as ::core::ffi::c_char;
        } else if !(nibble == 0xd as ::core::ffi::c_int) {
            if nibble == 0xf as ::core::ffi::c_int {
                let fresh15 = len;
                len = len + 1;
                *(&raw mut work_buffer as *mut ::core::ffi::c_char).offset(fresh15 as isize) =
                    '\0' as i32 as ::core::ffi::c_char;
                if pos % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && **data as ::core::ffi::c_int != 0xff as ::core::ffi::c_int
                {
                    fail = 1 as ::core::ffi::c_int;
                }
                break;
            } else {
                fail = 1 as ::core::ffi::c_int;
            }
        }
        pos += 1;
    }
    if fail != 0 || nibble != 0xf as ::core::ffi::c_int {
        *status = CFF_ERROR_PARSE_ERROR;
    } else {
        let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        result = strtod(&raw mut work_buffer as *mut ::core::ffi::c_char, &raw mut s);
        if *s as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || *crate::backend::runtime::errno() == ERANGE
        {
            *status = CFF_ERROR_PARSE_ERROR;
        }
    }
    return result;
}
unsafe extern "C-unwind" fn add_dict(
    mut dict: *mut cff_dict,
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
    mut status: *mut ::core::ffi::c_int,
) {
    let mut id: ::core::ffi::c_int = 0;
    let mut argtype: ::core::ffi::c_int = 0;
    id = **data as ::core::ffi::c_int;
    if id == 0xc as ::core::ffi::c_int {
        *data = (*data).offset(1 as ::core::ffi::c_int as isize);
        if *data >= endptr || {
            id = **data as ::core::ffi::c_int + CFF_LAST_DICT_OP1;
            id >= CFF_LAST_DICT_OP
        } {
            *status = CFF_ERROR_PARSE_ERROR;
            return;
        }
    } else if id >= CFF_LAST_DICT_OP1 {
        *status = CFF_ERROR_PARSE_ERROR;
        return;
    }
    argtype = dict_operator[id as usize].argtype;
    if dict_operator[id as usize].opname.is_null() || argtype < 0 as ::core::ffi::c_int {
        return;
    }
    if (*dict).count >= (*dict).max {
        (*dict).max += DICT_ENTRY_MAX;
        (*dict).entries = renew(
            (*dict).entries as *mut ::core::ffi::c_void,
            ((*dict).max as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<cff_dict_entry>() as usize)
                as uint32_t,
        ) as *mut cff_dict_entry;
    }
    (*(*dict).entries.offset((*dict).count as isize)).id = id;
    let ref mut fresh16 = (*(*dict).entries.offset((*dict).count as isize)).key;
    *fresh16 = dict_operator[id as usize].opname;
    if argtype == CFF_TYPE_NUMBER
        || argtype == CFF_TYPE_BOOLEAN
        || argtype == CFF_TYPE_SID
        || argtype == CFF_TYPE_OFFSET
    {
        if stack_top < 1 as ::core::ffi::c_int {
            *status = CFF_ERROR_STACK_UNDERFLOW;
            return;
        }
        stack_top -= 1;
        (*(*dict).entries.offset((*dict).count as isize)).count = 1 as ::core::ffi::c_int;
        let ref mut fresh17 = (*(*dict).entries.offset((*dict).count as isize)).values;
        *fresh17 = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize)
            as uint32_t) as *mut ::core::ffi::c_double;
        *(*(*dict).entries.offset((*dict).count as isize))
            .values
            .offset(0 as ::core::ffi::c_int as isize) = arg_stack[stack_top as usize];
        (*dict).count += 1 as ::core::ffi::c_int;
    } else if stack_top > 0 as ::core::ffi::c_int {
        (*(*dict).entries.offset((*dict).count as isize)).count = stack_top;
        let ref mut fresh18 = (*(*dict).entries.offset((*dict).count as isize)).values;
        *fresh18 = new((stack_top as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize)
            as uint32_t) as *mut ::core::ffi::c_double;
        while stack_top > 0 as ::core::ffi::c_int {
            stack_top -= 1;
            *(*(*dict).entries.offset((*dict).count as isize))
                .values
                .offset(stack_top as isize) = arg_stack[stack_top as usize];
        }
        (*dict).count += 1 as ::core::ffi::c_int;
    }
    *data = (*data).offset(1 as ::core::ffi::c_int as isize);
}
#[unsafe(export_name = "__pdf_impl_cff_dict_unpack")]
pub unsafe extern "C-unwind" fn cff_dict_unpack(
    mut data: *mut card8,
    mut endptr: *mut card8,
) -> *mut cff_dict {
    let mut dict: *mut cff_dict = ::core::ptr::null_mut::<cff_dict>();
    let mut status: ::core::ffi::c_int = CFF_PARSE_OK;
    stack_top = 0 as ::core::ffi::c_int;
    dict = cff_new_dict();
    while data < endptr && status == CFF_PARSE_OK {
        if (*data as ::core::ffi::c_int) < 22 as ::core::ffi::c_int {
            add_dict(dict, &raw mut data, endptr, &raw mut status);
        } else if *data as ::core::ffi::c_int == 30 as ::core::ffi::c_int {
            if stack_top < CFF_DICT_STACK_LIMIT {
                arg_stack[stack_top as usize] = get_real(&raw mut data, endptr, &raw mut status);
                stack_top += 1;
            } else {
                status = CFF_ERROR_STACK_OVERFLOW;
            }
        } else if *data as ::core::ffi::c_int == 255 as ::core::ffi::c_int
            || *data as ::core::ffi::c_int >= 22 as ::core::ffi::c_int
                && *data as ::core::ffi::c_int <= 27 as ::core::ffi::c_int
        {
            data = data.offset(1);
        } else if stack_top < CFF_DICT_STACK_LIMIT {
            arg_stack[stack_top as usize] = get_integer(&raw mut data, endptr, &raw mut status);
            stack_top += 1;
        } else {
            status = CFF_ERROR_STACK_OVERFLOW;
        }
    }
    if status != CFF_PARSE_OK {
        pdf_abort!(
            b"%s: Parsing CFF DICT failed. (error=%d)\0" as *const u8 as *const ::core::ffi::c_char,
            CFF_DEBUG_STR.as_ptr(),
            status,
        );
    } else if stack_top != 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"%s: Garbage in CFF DICT data.\0" as *const u8 as *const ::core::ffi::c_char,
            CFF_DEBUG_STR.as_ptr(),
        );
        stack_top = 0 as ::core::ffi::c_int;
    }
    return dict;
}
unsafe extern "C-unwind" fn pack_integer(
    mut dest: *mut card8,
    mut destlen: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if value >= -(107 as ::core::ffi::c_int) && value <= 107 as ::core::ffi::c_int {
        if destlen < 1 as ::core::ffi::c_int {
            pdf_abort!(
                b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                CFF_DEBUG_STR.as_ptr(),
            );
        }
        *dest.offset(0 as ::core::ffi::c_int as isize) =
            (value + 139 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
        len = 1 as ::core::ffi::c_int;
    } else if value >= 108 as ::core::ffi::c_int && value <= 1131 as ::core::ffi::c_int {
        if destlen < 2 as ::core::ffi::c_int {
            pdf_abort!(
                b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                CFF_DEBUG_STR.as_ptr(),
            );
        }
        value = (0xf700 as ::core::ffi::c_uint)
            .wrapping_add(value as ::core::ffi::c_uint)
            .wrapping_sub(108 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        *dest.offset(0 as ::core::ffi::c_int as isize) =
            (value >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
        *dest.offset(1 as ::core::ffi::c_int as isize) =
            (value & 0xff as ::core::ffi::c_int) as card8;
        len = 2 as ::core::ffi::c_int;
    } else if value >= -(1131 as ::core::ffi::c_int) && value <= -(108 as ::core::ffi::c_int) {
        if destlen < 2 as ::core::ffi::c_int {
            pdf_abort!(
                b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                CFF_DEBUG_STR.as_ptr(),
            );
        }
        value = (0xfb00 as ::core::ffi::c_uint)
            .wrapping_sub(value as ::core::ffi::c_uint)
            .wrapping_sub(108 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        *dest.offset(0 as ::core::ffi::c_int as isize) =
            (value >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
        *dest.offset(1 as ::core::ffi::c_int as isize) =
            (value & 0xff as ::core::ffi::c_int) as card8;
        len = 2 as ::core::ffi::c_int;
    } else if value >= -(32768 as ::core::ffi::c_int) && value <= 32767 as ::core::ffi::c_int {
        if destlen < 3 as ::core::ffi::c_int {
            pdf_abort!(
                b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                CFF_DEBUG_STR.as_ptr(),
            );
        }
        *dest.offset(0 as ::core::ffi::c_int as isize) = 28 as card8;
        *dest.offset(1 as ::core::ffi::c_int as isize) =
            (value >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
        *dest.offset(2 as ::core::ffi::c_int as isize) =
            (value & 0xff as ::core::ffi::c_int) as card8;
        len = 3 as ::core::ffi::c_int;
    } else {
        if destlen < 5 as ::core::ffi::c_int {
            pdf_abort!(
                b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                CFF_DEBUG_STR.as_ptr(),
            );
        }
        *dest.offset(0 as ::core::ffi::c_int as isize) = 29 as card8;
        *dest.offset(1 as ::core::ffi::c_int as isize) =
            (value >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
        *dest.offset(2 as ::core::ffi::c_int as isize) =
            (value >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
        *dest.offset(3 as ::core::ffi::c_int as isize) =
            (value >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
        *dest.offset(4 as ::core::ffi::c_int as isize) =
            (value & 0xff as ::core::ffi::c_int) as card8;
        len = 5 as ::core::ffi::c_int;
    }
    return len;
}
unsafe extern "C-unwind" fn pack_real(
    mut dest: *mut card8,
    mut destlen: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pos: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    let mut buffer: [::core::ffi::c_char; 32] = [0; 32];
    if destlen < 2 as ::core::ffi::c_int {
        pdf_abort!(
            b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
            CFF_DEBUG_STR.as_ptr(),
        );
    }
    *dest.offset(0 as ::core::ffi::c_int as isize) = 30 as card8;
    if value == 0.0f64 {
        *dest.offset(1 as ::core::ffi::c_int as isize) = 0xf as card8;
        return 2 as ::core::ffi::c_int;
    }
    if value < 0.0f64 {
        *dest.offset(1 as ::core::ffi::c_int as isize) = 0xe0 as card8;
        value *= -1.0f64;
        pos += 1;
    }
    sprintf(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b"%.13g\0" as *const u8 as *const ::core::ffi::c_char,
        value,
    );
    i = 0 as ::core::ffi::c_int;
    while buffer[i as usize] as ::core::ffi::c_int != '\0' as i32 {
        let mut ch: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
        if buffer[i as usize] as ::core::ffi::c_int == '.' as i32 {
            ch = 0xa as ::core::ffi::c_uchar;
        } else if buffer[i as usize] as ::core::ffi::c_int >= '0' as i32
            && buffer[i as usize] as ::core::ffi::c_int <= '9' as i32
        {
            ch = (buffer[i as usize] as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uchar;
        } else if buffer[i as usize] as ::core::ffi::c_int == 'e' as i32 {
            i += 1;
            ch = (if buffer[i as usize] as ::core::ffi::c_int == '-' as i32 {
                0xc as ::core::ffi::c_int
            } else {
                0xb as ::core::ffi::c_int
            }) as ::core::ffi::c_uchar;
        } else {
            pdf_abort!(
                b"%s: Invalid character.\0" as *const u8 as *const ::core::ffi::c_char,
                CFF_DEBUG_STR.as_ptr(),
            );
        }
        if destlen < pos / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
            pdf_abort!(
                b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                CFF_DEBUG_STR.as_ptr(),
            );
        }
        if pos % 2 as ::core::ffi::c_int != 0 {
            let ref mut fresh22 = *dest.offset((pos / 2 as ::core::ffi::c_int) as isize);
            *fresh22 = (*fresh22 as ::core::ffi::c_int + ch as ::core::ffi::c_int) as card8;
        } else {
            *dest.offset((pos / 2 as ::core::ffi::c_int) as isize) =
                ((ch as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as card8;
        }
        pos += 1;
        i += 1;
    }
    if pos % 2 as ::core::ffi::c_int != 0 {
        let ref mut fresh23 = *dest.offset((pos / 2 as ::core::ffi::c_int) as isize);
        *fresh23 = (*fresh23 as ::core::ffi::c_int + 0xf as ::core::ffi::c_int) as card8;
        pos += 1;
    } else {
        if destlen < pos / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
            pdf_abort!(
                b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                CFF_DEBUG_STR.as_ptr(),
            );
        }
        *dest.offset((pos / 2 as ::core::ffi::c_int) as isize) = 0xff as card8;
        pos += 2 as ::core::ffi::c_int;
    }
    return pos / 2 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn cff_dict_put_number(
    mut value: ::core::ffi::c_double,
    mut dest: *mut card8,
    mut destlen: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nearint: ::core::ffi::c_double = 0.;
    nearint = floor(value + 0.5f64);
    if type_0 == CFF_TYPE_OFFSET {
        let mut lvalue: ::core::ffi::c_int = 0;
        lvalue = value as ::core::ffi::c_int;
        if destlen < 5 as ::core::ffi::c_int {
            pdf_abort!(
                b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                CFF_DEBUG_STR.as_ptr(),
            );
        }
        *dest.offset(0 as ::core::ffi::c_int as isize) = 29 as card8;
        *dest.offset(1 as ::core::ffi::c_int as isize) =
            (lvalue >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
        *dest.offset(2 as ::core::ffi::c_int as isize) =
            (lvalue >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
        *dest.offset(3 as ::core::ffi::c_int as isize) =
            (lvalue >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
        *dest.offset(4 as ::core::ffi::c_int as isize) =
            (lvalue & 0xff as ::core::ffi::c_int) as card8;
        len = 5 as ::core::ffi::c_int;
    } else if value > CFF_INT_MAX as ::core::ffi::c_double
        || value < CFF_INT_MIN as ::core::ffi::c_double
        || fabs(value - nearint) > 1.0e-5f64
    {
        len = pack_real(dest, destlen, value);
    } else {
        len = pack_integer(dest, destlen, nearint as ::core::ffi::c_int);
    }
    return len;
}
unsafe extern "C-unwind" fn put_dict_entry(
    mut de: *mut cff_dict_entry,
    mut dest: *mut card8,
    mut destlen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut type_0: ::core::ffi::c_int = 0;
    let mut id: ::core::ffi::c_int = 0;
    if (*de).count > 0 as ::core::ffi::c_int {
        id = (*de).id;
        if dict_operator[id as usize].argtype == CFF_TYPE_OFFSET
            || dict_operator[id as usize].argtype == CFF_TYPE_SZOFF
        {
            type_0 = CFF_TYPE_OFFSET;
        } else {
            type_0 = CFF_TYPE_NUMBER;
        }
        i = 0 as ::core::ffi::c_int;
        while i < (*de).count {
            len += cff_dict_put_number(
                *(*de).values.offset(i as isize),
                dest.offset(len as isize),
                destlen - len,
                type_0,
            );
            i += 1;
        }
        if id >= 0 as ::core::ffi::c_int && id < CFF_LAST_DICT_OP1 {
            if len + 1 as ::core::ffi::c_int > destlen {
                pdf_abort!(
                    b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                    CFF_DEBUG_STR.as_ptr(),
                );
            }
            let fresh19 = len;
            len = len + 1;
            *dest.offset(fresh19 as isize) = id as card8;
        } else if id >= 0 as ::core::ffi::c_int && id < CFF_LAST_DICT_OP {
            if len + 2 as ::core::ffi::c_int > destlen {
                pdf_abort!(
                    b"in cff_dict_pack(): Buffer overflow\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            let fresh20 = len;
            len = len + 1;
            *dest.offset(fresh20 as isize) = 12 as card8;
            let fresh21 = len;
            len = len + 1;
            *dest.offset(fresh21 as isize) = (id - CFF_LAST_DICT_OP1) as card8;
        } else {
            pdf_abort!(
                b"%s: Invalid CFF DICT operator ID.\0" as *const u8 as *const ::core::ffi::c_char,
                CFF_DEBUG_STR.as_ptr(),
            );
        }
    }
    return len;
}
#[unsafe(export_name = "__pdf_impl_cff_dict_pack")]
pub unsafe extern "C-unwind" fn cff_dict_pack(
    mut dict: *mut cff_dict,
    mut dest: *mut card8,
    mut destlen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*dict).count {
        if streq_ptr(
            (*(*dict).entries.offset(i as isize)).key,
            b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        ) {
            len += put_dict_entry(
                (*dict).entries.offset(i as isize) as *mut cff_dict_entry,
                dest,
                destlen,
            );
            break;
        } else {
            i += 1;
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*dict).count {
        if strcmp(
            (*(*dict).entries.offset(i as isize)).key,
            b"ROS\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        {
            len += put_dict_entry(
                (*dict).entries.offset(i as isize) as *mut cff_dict_entry,
                dest.offset(len as isize),
                destlen - len,
            );
        }
        i += 1;
    }
    return len;
}
#[unsafe(export_name = "__pdf_impl_cff_dict_add")]
pub unsafe extern "C-unwind" fn cff_dict_add(
    mut dict: *mut cff_dict,
    mut key: *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) {
    let mut id: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    id = 0 as ::core::ffi::c_int;
    while id < CFF_LAST_DICT_OP {
        if !key.is_null()
            && !dict_operator[id as usize].opname.is_null()
            && streq_ptr(dict_operator[id as usize].opname, key) as ::core::ffi::c_int != 0
        {
            break;
        }
        id += 1;
    }
    if id == CFF_LAST_DICT_OP {
        pdf_abort!(
            b"%s: Unknown CFF DICT operator.\0" as *const u8 as *const ::core::ffi::c_char,
            CFF_DEBUG_STR.as_ptr(),
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*dict).count {
        if (*(*dict).entries.offset(i as isize)).id == id {
            if (*(*dict).entries.offset(i as isize)).count != count {
                pdf_abort!(
                    b"%s: Inconsistent DICT argument number.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    CFF_DEBUG_STR.as_ptr(),
                );
            }
            return;
        }
        i += 1;
    }
    if (*dict).count + 1 as ::core::ffi::c_int >= (*dict).max {
        (*dict).max += 8 as ::core::ffi::c_int;
        (*dict).entries = renew(
            (*dict).entries as *mut ::core::ffi::c_void,
            ((*dict).max as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<cff_dict_entry>() as usize)
                as uint32_t,
        ) as *mut cff_dict_entry;
    }
    (*(*dict).entries.offset((*dict).count as isize)).id = id;
    let ref mut fresh0 = (*(*dict).entries.offset((*dict).count as isize)).key;
    *fresh0 = dict_operator[id as usize].opname;
    (*(*dict).entries.offset((*dict).count as isize)).count = count;
    if count > 0 as ::core::ffi::c_int {
        let ref mut fresh1 = (*(*dict).entries.offset((*dict).count as isize)).values;
        *fresh1 = new((count as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_double>() as usize)
            as uint32_t) as *mut ::core::ffi::c_double;
        memset(
            (*(*dict).entries.offset((*dict).count as isize)).values as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
                .wrapping_mul(count as size_t),
        );
    } else {
        let ref mut fresh2 = (*(*dict).entries.offset((*dict).count as isize)).values;
        *fresh2 = ::core::ptr::null_mut::<::core::ffi::c_double>();
    }
    (*dict).count += 1 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_cff_dict_remove")]
pub unsafe extern "C-unwind" fn cff_dict_remove(
    mut dict: *mut cff_dict,
    mut key: *const ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*dict).count {
        if streq_ptr(key, (*(*dict).entries.offset(i as isize)).key) {
            (*(*dict).entries.offset(i as isize)).count = 0 as ::core::ffi::c_int;
            let ref mut fresh3 = (*(*dict).entries.offset(i as isize)).values;
            *fresh3 =
                mfree((*(*dict).entries.offset(i as isize)).values as *mut ::core::ffi::c_void)
                    as *mut ::core::ffi::c_double;
        }
        i += 1;
    }
}
#[unsafe(export_name = "__pdf_impl_cff_dict_known")]
pub unsafe extern "C-unwind" fn cff_dict_known(
    mut dict: *mut cff_dict,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*dict).count {
        if streq_ptr(key, (*(*dict).entries.offset(i as isize)).key) as ::core::ffi::c_int != 0
            && (*(*dict).entries.offset(i as isize)).count > 0 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_cff_dict_get")]
pub unsafe extern "C-unwind" fn cff_dict_get(
    mut dict: *mut cff_dict,
    mut key: *const ::core::ffi::c_char,
    mut idx: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut value: ::core::ffi::c_double = 0.0f64;
    let mut i: ::core::ffi::c_int = 0;
    if !(!key.is_null() && !dict.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"key && dict\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    i = 0 as ::core::ffi::c_int;
    while i < (*dict).count {
        if streq_ptr(key, (*(*dict).entries.offset(i as isize)).key) {
            if (*(*dict).entries.offset(i as isize)).count > idx {
                value = *(*(*dict).entries.offset(i as isize))
                    .values
                    .offset(idx as isize);
            } else {
                pdf_abort!(
                    b"%s: Invalid index number.\0" as *const u8 as *const ::core::ffi::c_char,
                    CFF_DEBUG_STR.as_ptr(),
                );
            }
            break;
        } else {
            i += 1;
        }
    }
    if i == (*dict).count {
        pdf_abort!(
            b"%s: DICT entry \"%s\" not found.\0" as *const u8 as *const ::core::ffi::c_char,
            CFF_DEBUG_STR.as_ptr(),
            key,
        );
    }
    return value;
}
#[unsafe(export_name = "__pdf_impl_cff_dict_set")]
pub unsafe extern "C-unwind" fn cff_dict_set(
    mut dict: *mut cff_dict,
    mut key: *const ::core::ffi::c_char,
    mut idx: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    if !(!dict.is_null() && !key.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"dict && key\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    i = 0 as ::core::ffi::c_int;
    while i < (*dict).count {
        if streq_ptr(key, (*(*dict).entries.offset(i as isize)).key) {
            if (*(*dict).entries.offset(i as isize)).count > idx {
                *(*(*dict).entries.offset(i as isize))
                    .values
                    .offset(idx as isize) = value;
            } else {
                pdf_abort!(
                    b"%s: Invalid index number.\0" as *const u8 as *const ::core::ffi::c_char,
                    CFF_DEBUG_STR.as_ptr(),
                );
            }
            break;
        } else {
            i += 1;
        }
    }
    if i == (*dict).count {
        pdf_abort!(
            b"%s: DICT entry \"%s\" not found.\0" as *const u8 as *const ::core::ffi::c_char,
            CFF_DEBUG_STR.as_ptr(),
            key,
        );
    }
}
#[unsafe(export_name = "__pdf_impl_cff_dict_update")]
pub unsafe extern "C-unwind" fn cff_dict_update(mut dict: *mut cff_dict, mut cff: *mut cff_font) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*dict).count {
        if (*(*dict).entries.offset(i as isize)).count > 0 as ::core::ffi::c_int {
            let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut id: ::core::ffi::c_int = 0;
            id = (*(*dict).entries.offset(i as isize)).id;
            if dict_operator[id as usize].argtype == CFF_TYPE_SID {
                str = cff_get_string(
                    cff,
                    *(*(*dict).entries.offset(i as isize))
                        .values
                        .offset(0 as ::core::ffi::c_int as isize) as s_SID,
                );
                *(*(*dict).entries.offset(i as isize))
                    .values
                    .offset(0 as ::core::ffi::c_int as isize) =
                    cff_add_string(cff, str, 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
                free(str as *mut ::core::ffi::c_void);
            } else if dict_operator[id as usize].argtype == CFF_TYPE_ROS {
                str = cff_get_string(
                    cff,
                    *(*(*dict).entries.offset(i as isize))
                        .values
                        .offset(0 as ::core::ffi::c_int as isize) as s_SID,
                );
                *(*(*dict).entries.offset(i as isize))
                    .values
                    .offset(0 as ::core::ffi::c_int as isize) =
                    cff_add_string(cff, str, 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
                free(str as *mut ::core::ffi::c_void);
                str = cff_get_string(
                    cff,
                    *(*(*dict).entries.offset(i as isize))
                        .values
                        .offset(1 as ::core::ffi::c_int as isize) as s_SID,
                );
                *(*(*dict).entries.offset(i as isize))
                    .values
                    .offset(1 as ::core::ffi::c_int as isize) =
                    cff_add_string(cff, str, 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
                free(str as *mut ::core::ffi::c_void);
            }
        }
        i += 1;
    }
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_new_dict(_result: *mut *mut cff_dict) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_new_dict() }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_release_dict(mut dict: *mut cff_dict) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_release_dict(dict) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_dict_unpack(
    mut data: *mut card8,
    mut endptr: *mut card8,
    _result: *mut *mut cff_dict,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_dict_unpack(data, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_dict_pack(
    mut dict: *mut cff_dict,
    mut dest: *mut card8,
    mut destlen: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_dict_pack(dict, dest, destlen) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_dict_add(
    mut dict: *mut cff_dict,
    mut key: *const ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_dict_add(dict, key, count) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_dict_remove(
    mut dict: *mut cff_dict,
    mut key: *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_dict_remove(dict, key) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_dict_known(
    mut dict: *mut cff_dict,
    mut key: *const ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_dict_known(dict, key) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_dict_get(
    mut dict: *mut cff_dict,
    mut key: *const ::core::ffi::c_char,
    mut idx: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_dict_get(dict, key, idx) }) {
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
pub unsafe extern "C" fn __pdf_entry_cff_dict_set(
    mut dict: *mut cff_dict,
    mut key: *const ::core::ffi::c_char,
    mut idx: ::core::ffi::c_int,
    mut value: ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_dict_set(dict, key, idx, value) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cff_dict_update(
    mut dict: *mut cff_dict,
    mut cff: *mut cff_font,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { cff_dict_update(dict, cff) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
