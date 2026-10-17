/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strtoul(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn strcat(
        __s1: *mut ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    #[link_name = "__pdf_impl_pdf_get_version"]
    fn pdf_get_version() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_version_major"]
    fn pdf_get_version_major() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_version_minor"]
    fn pdf_get_version_minor() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_number_value"]
    fn pdf_number_value(number: *mut pdf_obj) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pdf_string_value"]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_name_value"]
    fn pdf_name_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_remove_dict"]
    fn pdf_remove_dict(dict: *mut pdf_obj, key: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_pdf_lookup_dict"]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(
        dict: *mut pdf_obj,
        key: *mut pdf_obj,
        value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CIDFont_type0_open"]
    fn CIDFont_type0_open(
        font: *mut pdf_font,
        name: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
        opt: *mut cid_opt,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CIDFont_type0_dofont"]
    fn CIDFont_type0_dofont(font: *mut pdf_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CIDFont_type0_open_from_t1"]
    fn CIDFont_type0_open_from_t1(
        font: *mut pdf_font,
        name: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
        opt: *mut cid_opt,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CIDFont_type0_open_from_t1c"]
    fn CIDFont_type0_open_from_t1c(
        font: *mut pdf_font,
        name: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
        opt: *mut cid_opt,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CIDFont_type0_t1dofont"]
    fn CIDFont_type0_t1dofont(font: *mut pdf_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CIDFont_type0_t1cdofont"]
    fn CIDFont_type0_t1cdofont(font: *mut pdf_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CIDFont_type2_open"]
    fn CIDFont_type2_open(
        font: *mut pdf_font,
        name: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
        opt: *mut cid_opt,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CIDFont_type2_dofont"]
    fn CIDFont_type2_dofont(font: *mut pdf_font) -> ::core::ffi::c_int;
    static mut dpx_conf: _dpx_conf;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_parse_pdf_dict"]
    fn parse_pdf_dict(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
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
pub struct CIDSysInfo {
    pub registry: *mut ::core::ffi::c_char,
    pub ordering: *mut ::core::ffi::c_char,
    pub supplement: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cid_opt {
    pub csi: CIDSysInfo,
    pub style: ::core::ffi::c_int,
    pub embed: ::core::ffi::c_int,
    pub stemv: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_font {
    pub ident: *mut ::core::ffi::c_char,
    pub font_id: ::core::ffi::c_int,
    pub subtype: ::core::ffi::c_int,
    pub filename: *mut ::core::ffi::c_char,
    pub encoding_id: ::core::ffi::c_int,
    pub index: uint32_t,
    pub fontname: *mut ::core::ffi::c_char,
    pub uniqueID: [::core::ffi::c_char; 7],
    pub reference: *mut pdf_obj,
    pub resource: *mut pdf_obj,
    pub descriptor: *mut pdf_obj,
    pub usedchars: *mut ::core::ffi::c_char,
    pub flags: ::core::ffi::c_int,
    pub point_size: ::core::ffi::c_double,
    pub design_size: ::core::ffi::c_double,
    pub type0: C2RustUnnamed_0,
    pub cid: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub csi: CIDSysInfo,
    pub options: cid_opt,
    pub need_vmetrics: ::core::ffi::c_int,
    pub usedchars_v: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub descendant: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub registry: *const ::core::ffi::c_char,
    pub ordering: *const ::core::ffi::c_char,
    pub supplement: [::core::ffi::c_int; 21],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub name: *const ::core::ffi::c_char,
    pub index: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub fontname: *const ::core::ffi::c_char,
    pub fontdict: *const ::core::ffi::c_char,
    pub descriptor: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub keep_cache: ::core::ffi::c_int,
}
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
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
pub const FONTMAP_OPT_NOEMBED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const FONTMAP_STYLE_BOLD: ::core::ffi::c_int = 1;
pub const FONTMAP_STYLE_ITALIC: ::core::ffi::c_int = 2;
pub const FONTMAP_STYLE_BOLDITALIC: ::core::ffi::c_int = 3;
pub const PDF_FONT_FONTTYPE_TYPE1: ::core::ffi::c_int = 0;
pub const PDF_FONT_FONTTYPE_TYPE1C: ::core::ffi::c_int = 1;
pub const PDF_FONT_FONTTYPE_TRUETYPE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PDF_FONT_FONTTYPE_CIDTYPE0: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PDF_FONT_FONTTYPE_CIDTYPE2: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PDF_FONT_FLAG_BASEFONT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const CIDFONT_FLAG_TYPE1: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const CIDFONT_FLAG_TYPE1C: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
pub const CIDFONT_FLAG_TRUETYPE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
pub const FONT_STYLE_BOLD: ::core::ffi::c_int = FONTMAP_STYLE_BOLD;
pub const FONT_STYLE_ITALIC: ::core::ffi::c_int = FONTMAP_STYLE_ITALIC;
pub const FONT_STYLE_BOLDITALIC: ::core::ffi::c_int = FONTMAP_STYLE_BOLDITALIC;
pub const CIDFONT_FORCE_FIXEDPITCH: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
static mut CIDFont_stdcc_def: [C2RustUnnamed_1; 7] = [
    C2RustUnnamed_1 {
        registry: b"Adobe\0" as *const u8 as *const ::core::ffi::c_char,
        ordering: b"UCS\0" as *const u8 as *const ::core::ffi::c_char,
        supplement: [
            -(1 as ::core::ffi::c_int),
            -(1 as ::core::ffi::c_int),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ],
    },
    C2RustUnnamed_1 {
        registry: b"Adobe\0" as *const u8 as *const ::core::ffi::c_char,
        ordering: b"GB1\0" as *const u8 as *const ::core::ffi::c_char,
        supplement: [
            -(1 as ::core::ffi::c_int),
            -(1 as ::core::ffi::c_int),
            0 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
        ],
    },
    C2RustUnnamed_1 {
        registry: b"Adobe\0" as *const u8 as *const ::core::ffi::c_char,
        ordering: b"CNS1\0" as *const u8 as *const ::core::ffi::c_char,
        supplement: [
            -(1 as ::core::ffi::c_int),
            -(1 as ::core::ffi::c_int),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
        ],
    },
    C2RustUnnamed_1 {
        registry: b"Adobe\0" as *const u8 as *const ::core::ffi::c_char,
        ordering: b"Japan1\0" as *const u8 as *const ::core::ffi::c_char,
        supplement: [
            -(1 as ::core::ffi::c_int),
            -(1 as ::core::ffi::c_int),
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            5 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
            6 as ::core::ffi::c_int,
        ],
    },
    C2RustUnnamed_1 {
        registry: b"Adobe\0" as *const u8 as *const ::core::ffi::c_char,
        ordering: b"Korea1\0" as *const u8 as *const ::core::ffi::c_char,
        supplement: [
            -(1 as ::core::ffi::c_int),
            -(1 as ::core::ffi::c_int),
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        ],
    },
    C2RustUnnamed_1 {
        registry: b"Adobe\0" as *const u8 as *const ::core::ffi::c_char,
        ordering: b"Identity\0" as *const u8 as *const ::core::ffi::c_char,
        supplement: [
            -(1 as ::core::ffi::c_int),
            -(1 as ::core::ffi::c_int),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ],
    },
    C2RustUnnamed_1 {
        registry: ::core::ptr::null::<::core::ffi::c_char>(),
        ordering: ::core::ptr::null::<::core::ffi::c_char>(),
        supplement: [
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ],
    },
];
pub const SUP_IDX_MAX: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const ACC_START: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ACC_END: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut registry_Adobe: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"Adobe\0") };
static mut ordering_Identity: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"Identity\0") };
static mut ordering_UCS: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"UCS\0") };
#[unsafe(no_mangle)]
pub static mut CSI_IDENTITY: CIDSysInfo = unsafe {
    CIDSysInfo {
        registry: &raw const registry_Adobe as *mut ::core::ffi::c_char,
        ordering: &raw const ordering_Identity as *mut ::core::ffi::c_char,
        supplement: 0 as ::core::ffi::c_int,
    }
};
#[unsafe(no_mangle)]
pub static mut CSI_UNICODE: CIDSysInfo = unsafe {
    CIDSysInfo {
        registry: &raw const registry_Adobe as *mut ::core::ffi::c_char,
        ordering: &raw const ordering_UCS as *mut ::core::ffi::c_char,
        supplement: 0 as ::core::ffi::c_int,
    }
};
static mut CIDFont_stdcc_alias: [C2RustUnnamed_2; 19] = [
    C2RustUnnamed_2 {
        name: b"AU\0" as *const u8 as *const ::core::ffi::c_char,
        index: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"AG1\0" as *const u8 as *const ::core::ffi::c_char,
        index: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"AC1\0" as *const u8 as *const ::core::ffi::c_char,
        index: 2 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"AJ1\0" as *const u8 as *const ::core::ffi::c_char,
        index: 3 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"AK1\0" as *const u8 as *const ::core::ffi::c_char,
        index: 4 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"AI\0" as *const u8 as *const ::core::ffi::c_char,
        index: 5 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"UCS\0" as *const u8 as *const ::core::ffi::c_char,
        index: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"GB1\0" as *const u8 as *const ::core::ffi::c_char,
        index: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"CNS1\0" as *const u8 as *const ::core::ffi::c_char,
        index: 2 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"Japan1\0" as *const u8 as *const ::core::ffi::c_char,
        index: 3 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"Korea1\0" as *const u8 as *const ::core::ffi::c_char,
        index: 4 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"Identity\0" as *const u8 as *const ::core::ffi::c_char,
        index: 5 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"U\0" as *const u8 as *const ::core::ffi::c_char,
        index: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"G\0" as *const u8 as *const ::core::ffi::c_char,
        index: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"C\0" as *const u8 as *const ::core::ffi::c_char,
        index: 2 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"J\0" as *const u8 as *const ::core::ffi::c_char,
        index: 3 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"K\0" as *const u8 as *const ::core::ffi::c_char,
        index: 4 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"I\0" as *const u8 as *const ::core::ffi::c_char,
        index: 5 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        index: 0 as ::core::ffi::c_int,
    },
];
#[unsafe(no_mangle)]
pub static mut opt_flags_cidfont: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_CIDFont_is_ACCFont")]
pub unsafe extern "C-unwind" fn CIDFont_is_ACCFont(mut font: *mut pdf_font) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut csi: *mut CIDSysInfo = ::core::ptr::null_mut::<CIDSysInfo>();
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    csi = &raw mut (*font).cid.csi;
    i = ACC_START;
    while i <= ACC_END {
        if strcmp((*csi).registry, CIDFont_stdcc_def[i as usize].registry) == 0
            && strcmp((*csi).ordering, CIDFont_stdcc_def[i as usize].ordering) == 0
        {
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_CIDFont_is_UCSFont")]
pub unsafe extern "C-unwind" fn CIDFont_is_UCSFont(mut font: *mut pdf_font) -> ::core::ffi::c_int {
    let mut csi: *mut CIDSysInfo = ::core::ptr::null_mut::<CIDSysInfo>();
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    csi = &raw mut (*font).cid.csi;
    if strcmp(
        (*csi).ordering,
        b"UCS\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            (*csi).ordering,
            b"UCS2\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_CIDFont_get_usedchars")]
pub unsafe extern "C-unwind" fn CIDFont_get_usedchars(
    mut font: *mut pdf_font,
) -> *mut ::core::ffi::c_char {
    if (*font).usedchars.is_null() {
        (*font).usedchars = new((8192 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memset(
            (*font).usedchars as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (8192 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
    }
    return (*font).usedchars;
}
#[unsafe(export_name = "__pdf_impl_CIDFont_get_usedchars_v")]
pub unsafe extern "C-unwind" fn CIDFont_get_usedchars_v(
    mut font: *mut pdf_font,
) -> *mut ::core::ffi::c_char {
    if (*font).cid.usedchars_v.is_null() {
        (*font).cid.usedchars_v = new((8192 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memset(
            (*font).cid.usedchars_v as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (8192 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        );
    }
    return (*font).cid.usedchars_v;
}
unsafe extern "C-unwind" fn source_font_type(mut font: *mut pdf_font) -> ::core::ffi::c_int {
    let mut type_0: ::core::ffi::c_int = PDF_FONT_FONTTYPE_CIDTYPE0;
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*font).flags & CIDFONT_FLAG_TYPE1 != 0 {
        type_0 = PDF_FONT_FONTTYPE_TYPE1;
    } else if (*font).flags & CIDFONT_FLAG_TYPE1C != 0 {
        type_0 = PDF_FONT_FONTTYPE_TYPE1C;
    } else if (*font).flags & CIDFONT_FLAG_TRUETYPE != 0 {
        type_0 = PDF_FONT_FONTTYPE_TRUETYPE;
    }
    return type_0;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_load_cidfont")]
pub unsafe extern "C-unwind" fn pdf_font_load_cidfont(mut font: *mut pdf_font) {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if font.is_null() || (*font).reference.is_null() {
        return;
    }
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(
            b":%s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
    }
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        if !(*font).fontname.is_null() {
            pdf_message!(
                b"[%s]\0" as *const u8 as *const ::core::ffi::c_char,
                (*font).fontname,
            );
        }
    }
    match (*font).subtype {
        PDF_FONT_FONTTYPE_CIDTYPE0 => {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(b"[CIDFontType0]\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            match source_font_type(font) {
                PDF_FONT_FONTTYPE_TYPE1 => {
                    error = CIDFont_type0_t1dofont(font);
                }
                PDF_FONT_FONTTYPE_TYPE1C => {
                    error = CIDFont_type0_t1cdofont(font);
                }
                _ => {
                    error = CIDFont_type0_dofont(font);
                }
            }
        }
        PDF_FONT_FONTTYPE_CIDTYPE2 => {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(b"[CIDFontType2]\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            error = CIDFont_type2_dofont(font);
        }
        _ => {}
    }
    if error != 0 {
        pdf_abort!(
            b"Error occurred while loading font: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
    }
}
static mut cid_basefont: [C2RustUnnamed_3; 21] = [
    C2RustUnnamed_3 {
        fontname: b"Ryumin-Light\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype/CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (Japan1) /Supplement 2 >> /DW 1000 /W [  231   632 500  8718 [500 500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /CapHeight 709 /Ascent 723 /Descent -241 /StemV 69 /FontBBox [-170 -331 1024 903] /ItalicAngle 0 /Flags 6 /Style << /Panose <010502020300000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"GothicBBB-Medium\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype/CIDFontType0 /CIDSystemInfo <<  /Registry (Adobe) /Ordering (Japan1) /Supplement 2 >> /DW 1000 /W [  231   632 500  8718 [500 500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /CapHeight 737 /Ascent 752 /Descent -271 /StemV 99 /FontBBox [-174 -268 1001 944] /ItalicAngle 0 /Flags 4 /Style << /Panose <0801020b0500000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"MHei-Medium-Acro\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype /CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (CNS1) /Supplement 0 >> /DW 1000 /W [13648 13742 500 17603 [500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 752 /CapHeight 737 /Descent -271 /StemV 58 /FontBBox [-45 -250 1015 887] /ItalicAngle 0 /Flags 4 /XHeight 553 /Style << /Panose <000001000600000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"MSung-Light-Acro\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype /CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (CNS1) /Supplement 0 >> /DW 1000 /W [13648 13742 500 17603 [500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 752 /CapHeight 737 /Descent -271 /StemV 58 /FontBBox [-160 -249 1015 888] /ItalicAngle 0 /Flags 6 /XHeight 553 /Style << /Panose <000000000400000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"STSong-Light-Acro\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype /CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (GB1) /Supplement 2 >> /DW 1000 /W [  814 939 500  7716 [500] 22355 [500 500] 22357 [500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 752 /CapHeight 737 /Descent -271 /StemV 58 /FontBBox [-25 -254 1000 880] /ItalicAngle 0 /Flags 6 /XHeight 599 /Style << /Panose <000000000400000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"STHeiti-Regular-Acro\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype /CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (GB1) /Supplement 1 >> /DW 1000 /W [  814 939 500  7716 [500] 22355 [500 500] 22357 [500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 752 /CapHeight 737 /Descent -271 /StemV 58 /FontBBox [-34 -250 1000 882] /ItalicAngle 0 /Flags 4 /XHeight 599 /Style << /Panose <000001000600000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"HeiseiKakuGo-W5-Acro\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype /CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (Japan1) /Supplement  2 >> /DW 1000 /W [  231   632 500  8718 [500 500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 752 /CapHeight 737 /Descent -221 /StemV 114 /FontBBox [-92 -250 1010 922] /ItalicAngle 0 /Flags 4 /XHeight 553 /Style << /Panose <0801020b0600000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"HeiseiMin-W3-Acro\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype /CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (Japan1) /Supplement 2 >> /DW 1000 /W [  231   632 500  8718 [500 500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 723 /CapHeight 709 /Descent -241 /StemV 69 /FontBBox [-123 -257 1001 910] /ItalicAngle 0 /Flags 6 /XHeight 450 /Style << /Panose <010502020400000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"HYGoThic-Medium-Acro\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype /CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (Korea1) /Supplement 1 >> /DW 1000 /W [   97 [500]  8094  8190 500 ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 752 /CapHeight 737 /Descent -271 /StemV 58 /FontBBox [-6 -145 1003 880] /ItalicAngle 0 /Flags 4 /XHeight 553 /Style << /Panose <000001000600000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"HYSMyeongJo-Medium-Acro\0" as *const u8
            as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype /CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (Korea1) /Supplement 1 >> /DW 1000 /W [   97 [500]  8094  8190 500 ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 752 /CapHeight 737 /Descent -271 /StemV 58 /FontBBox [-0 -148 1001 880] /ItalicAngle 0 /Flags 6 /XHeight 553 /Style << /Panose <000000000600000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"MSungStd-Light-Acro\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype /CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (CNS1) /Supplement 4 >> /DW 1000 /W [13648 13742 500 17603 [500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 880 /CapHeight 662 /Descent -120 /StemV 54 /FontBBox [-160 -249 1015 1071] /ItalicAngle 0 /Flags 6 /Style << /Panose <000000000400000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"STSongStd-Light-Acro\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype /CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (GB1) /Supplement 4 >> /DW 1000 /W [  814 939 500  7716 [500] 22355 [500 500] 22357 [500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 880 /CapHeight 626 /Descent -120 /StemV 44 /FontBBox [-134 -254 1001 905] /ItalicAngle 0 /Flags 6 /Style << /Panose <000000000400000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"HYSMyeongJoStd-Medium-Acro\0" as *const u8
            as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype /CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (Korea1) /Supplement 2 >> /DW 1000 /W [   97 [500]  8094  8190 500 ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 880 /CapHeight 720 /Descent -120 /StemV 60 /FontBBox [-28 -148 1001 880] /ItalicAngle 0 /Flags 6 /Style << /Panose <000000000600000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"AdobeMingStd-Light-Acro\0" as *const u8
            as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype/CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (CNS1) /Supplement 4 >> /DW 1000 /W [13648 13742 500 17603 [500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 880 /Descent -120 /StemV 48 /CapHeight 731 /FontBBox [-38 -121 1002 918] /ItalicAngle 0 /Flags 6 /XHeight 466 /Style << /Panose <000002020300000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"AdobeSongStd-Light-Acro\0" as *const u8
            as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype/CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (GB1) /Supplement 4 >> /DW 1000 /W [  814 939 500  7716 [500] 22355 [500 500] 22357 [500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 880 /Descent -120 /StemV 66 /CapHeight 626 /FontBBox [-134 -254 1001 905] /ItalicAngle 0 /Flags 6 /XHeight 416 /Style << /Panose <000002020300000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"KozMinPro-Regular-Acro\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype/CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (Japan1) /Supplement 4 >> /DW 1000 /W [  231   632 500  8718 [500 500]  9738  9757 250  9758  9778 333 12063 12087 500 ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 880 /Descent -120 /StemV 86 /CapHeight 740 /FontBBox [-195 -272 1110 1075] /ItalicAngle 0 /Flags 6 /XHeight 502 /Style << /Panose <000002020400000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"KozGoPro-Medium-Acro\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype/CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering(Japan1) /Supplement 4 >> /DW 1000 /W [  231   632 500  8718 [500 500]  9738  9757 250  9758  9778 333 12063 12087 500 ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 880 /Descent -120 /StemV 99 /CapHeight 763 /FontBBox [-149 -374 1254 1008] /ItalicAngle 0 /Flags 4 /XHeight 549 /Style << /Panose <0000020b0700000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"AdobeMyungjoStd-Medium-Acro\0" as *const u8
            as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype/CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (Korea1) /Supplement 2 >> /DW 1000 /W [   97 [500]  8094  8190 500 ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 880 /Descent -120 /StemV 99 /CapHeight 719 /FontBBox [-28 -148 1001 880] /ItalicAngle 0 /Flags 6 /XHeight 478 /Style << /Panose <000002020600000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"KozMinProVI-Regular\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype/CIDFontType0 /CIDSystemInfo <<   /Registry (Adobe)   /Ordering (Japan1)   /Supplement 6 >> /DW 1000 /W [  231   632 500   8718 [500 500]   9738  9757 250   9758  9778 333   12063 12087 500 ]        >>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 880 /Descent -120 /StemV 86 /CapHeight 742 /FontBBox [-437 -340 1144 1317] /ItalicAngle 0 /Flags 6 /XHeight 503 /Style <<   /Panose <000002020400000000000000> >>      >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: b"AdobeHeitiStd-Regular\0" as *const u8 as *const ::core::ffi::c_char,
        fontdict: b"<< /Subtype/CIDFontType0 /CIDSystemInfo << /Registry (Adobe) /Ordering (GB1) /Supplement 4 >> /DW 1000 /W [  814 939 500  7716 [500] 22355 [500 500] 22357 [500] ]>>\0"
            as *const u8 as *const ::core::ffi::c_char,
        descriptor: b"<< /Ascent 880 /Descent -120 /StemV 66 /CapHeight 626 /FontBBox [-134 -254 1001 905] /ItalicAngle 0 /Flags 6 /XHeight 416 /Style << /Panose <000002020300000000000000> >> >>\0"
            as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_3 {
        fontname: ::core::ptr::null::<::core::ffi::c_char>(),
        fontdict: ::core::ptr::null::<::core::ffi::c_char>(),
        descriptor: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
unsafe extern "C-unwind" fn CIDFont_base_open(
    mut font: *mut pdf_font,
    mut name: *const ::core::ffi::c_char,
    mut opt: *mut cid_opt,
) -> ::core::ffi::c_int {
    let mut fontdict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut descriptor: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut idx: ::core::ffi::c_int = 0;
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    idx = 0 as ::core::ffi::c_int;
    while !cid_basefont[idx as usize].fontname.is_null() {
        if streq_ptr(name, cid_basefont[idx as usize].fontname) as ::core::ffi::c_int != 0
            || strlen(name)
                == strlen(cid_basefont[idx as usize].fontname).wrapping_sub(strlen(
                    b"-Acro\0" as *const u8 as *const ::core::ffi::c_char,
                ))
                && strncmp(
                    name,
                    cid_basefont[idx as usize].fontname,
                    strlen(cid_basefont[idx as usize].fontname).wrapping_sub(strlen(
                        b"-Acro\0" as *const u8 as *const ::core::ffi::c_char,
                    )),
                ) == 0
        {
            break;
        }
        idx += 1;
    }
    if cid_basefont[idx as usize].fontname.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    fontname = new(
        (strlen(name).wrapping_add(12 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    memset(
        fontname as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        strlen(name).wrapping_add(12 as size_t),
    );
    strcpy(fontname, name);
    match (*opt).style {
        FONT_STYLE_BOLD => {
            strcat(
                fontname,
                b",Bold\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        FONT_STYLE_ITALIC => {
            strcat(
                fontname,
                b",Italic\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        FONT_STYLE_BOLDITALIC => {
            strcat(
                fontname,
                b",BoldItalic\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        _ => {}
    }
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    start = cid_basefont[idx as usize].fontdict;
    end = start.offset(strlen(start) as isize);
    fontdict = parse_pdf_dict(&raw mut start, end, ::core::ptr::null_mut::<pdf_file>());
    start = cid_basefont[idx as usize].descriptor;
    end = start.offset(strlen(start) as isize);
    descriptor = parse_pdf_dict(&raw mut start, end, ::core::ptr::null_mut::<pdf_file>());
    if !(!fontdict.is_null() && !descriptor.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"fontdict && descriptor\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*font).fontname = fontname;
    (*font).flags |= PDF_FONT_FLAG_BASEFONT;
    let mut registry: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ordering: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut supplement: ::core::ffi::c_int = 0;
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    tmp = pdf_lookup_dict(
        fontdict,
        b"CIDSystemInfo\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !(!tmp.is_null() && pdf_obj_typeof(tmp) == 6 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"tmp && pdf_obj_typeof(tmp) == PDF_DICT\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    registry = pdf_string_value(pdf_lookup_dict(
        tmp,
        b"Registry\0" as *const u8 as *const ::core::ffi::c_char,
    )) as *mut ::core::ffi::c_char;
    ordering = pdf_string_value(pdf_lookup_dict(
        tmp,
        b"Ordering\0" as *const u8 as *const ::core::ffi::c_char,
    )) as *mut ::core::ffi::c_char;
    supplement = pdf_number_value(pdf_lookup_dict(
        tmp,
        b"Supplement\0" as *const u8 as *const ::core::ffi::c_char,
    )) as ::core::ffi::c_int;
    (*font).cid.csi.registry = new(
        (strlen(registry).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    (*font).cid.csi.ordering = new(
        (strlen(ordering).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*font).cid.csi.registry, registry);
    strcpy((*font).cid.csi.ordering, ordering);
    (*font).cid.csi.supplement = supplement;
    let mut tmp_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut type_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    tmp_0 = pdf_lookup_dict(
        fontdict,
        b"Subtype\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !(!tmp_0.is_null() && pdf_obj_typeof(tmp_0) == 4 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"tmp != NULL && pdf_obj_typeof(tmp) == PDF_NAME\0" as *const u8
                as *const ::core::ffi::c_char
        );
    } else {
    };
    type_0 = pdf_name_value(tmp_0);
    if streq_ptr(
        type_0,
        b"CIDFontType0\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        (*font).subtype = PDF_FONT_FONTTYPE_CIDTYPE0;
    } else if streq_ptr(
        type_0,
        b"CIDFontType2\0" as *const u8 as *const ::core::ffi::c_char,
    ) {
        (*font).subtype = PDF_FONT_FONTTYPE_CIDTYPE2;
    } else {
        pdf_abort!(
            b"Unknown CIDFontType \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            type_0,
        );
    }
    if opt_flags_cidfont & CIDFONT_FORCE_FIXEDPITCH != 0 {
        if !pdf_lookup_dict(fontdict, b"W\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            pdf_remove_dict(fontdict, b"W\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if !pdf_lookup_dict(fontdict, b"W2\0" as *const u8 as *const ::core::ffi::c_char).is_null()
        {
            pdf_remove_dict(fontdict, b"W2\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Font\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"BaseFont\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(fontname),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"FontDescriptor\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"FontName\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(fontname),
    );
    (*font).resource = fontdict;
    (*font).descriptor = descriptor;
    (*opt).embed = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_cidfont_lookup_cache")]
pub unsafe extern "C-unwind" fn pdf_font_cidfont_lookup_cache(
    mut fonts: *mut pdf_font,
    mut count: ::core::ffi::c_int,
    mut map_name: *const ::core::ffi::c_char,
    mut cmap_csi: *mut CIDSysInfo,
    mut fmap_opt: *const fontmap_opt,
) -> ::core::ffi::c_int {
    let mut font_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    let mut opt: cid_opt = cid_opt {
        csi: CIDSysInfo {
            registry: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            ordering: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            supplement: 0,
        },
        style: 0,
        embed: 0,
        stemv: 0,
    };
    let mut has_csi: ::core::ffi::c_int = 0;
    if fonts.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"fonts\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    opt.style = (*fmap_opt).style;
    opt.embed = if (*fmap_opt).flags & FONTMAP_OPT_NOEMBED != 0 {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    opt.csi.registry = ::core::ptr::null_mut::<::core::ffi::c_char>();
    opt.csi.ordering = ::core::ptr::null_mut::<::core::ffi::c_char>();
    opt.csi.supplement = 0 as ::core::ffi::c_int;
    has_csi = get_cidsysinfo(&raw mut opt.csi, map_name, fmap_opt);
    opt.stemv = (*fmap_opt).stemv;
    if has_csi == 0 && !cmap_csi.is_null() {
        opt.csi.registry = new(
            (strlen((*cmap_csi).registry).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy(opt.csi.registry, (*cmap_csi).registry);
        opt.csi.ordering = new(
            (strlen((*cmap_csi).ordering).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy(opt.csi.ordering, (*cmap_csi).ordering);
        opt.csi.supplement = (*cmap_csi).supplement;
        has_csi = 1 as ::core::ffi::c_int;
    }
    font_id = 0 as ::core::ffi::c_int;
    while font_id < count {
        font = fonts.offset(font_id as isize) as *mut pdf_font;
        if !((*font).subtype != PDF_FONT_FONTTYPE_CIDTYPE0
            && (*font).subtype != PDF_FONT_FONTTYPE_CIDTYPE2)
        {
            if strcmp((*font).filename, map_name) == 0
                && (*font).cid.options.style == opt.style
                && (*font).index == (*fmap_opt).index
            {
                if (*font).cid.options.embed == opt.embed {
                    if has_csi == 0 {
                        if !((*font).subtype == PDF_FONT_FONTTYPE_CIDTYPE2) {
                            break;
                        }
                    } else if strcmp((*font).cid.csi.registry, opt.csi.registry) == 0
                        && strcmp((*font).cid.csi.ordering, opt.csi.ordering) == 0
                    {
                        if (*font).subtype == PDF_FONT_FONTTYPE_CIDTYPE2 {
                            (*font).cid.csi.supplement =
                                if opt.csi.supplement > (*font).cid.csi.supplement {
                                    opt.csi.supplement
                                } else {
                                    (*font).cid.csi.supplement
                                };
                        }
                        break;
                    }
                } else if (*font).flags & PDF_FONT_FLAG_BASEFONT != 0 {
                    break;
                }
            }
        }
        font_id += 1;
    }
    release_opt(&raw mut opt);
    return if font_id < count {
        font_id
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_font_open_cidfont")]
pub unsafe extern "C-unwind" fn pdf_font_open_cidfont(
    mut font: *mut pdf_font,
    mut map_name: *const ::core::ffi::c_char,
    mut cmap_csi: *mut CIDSysInfo,
    mut fmap_opt: *const fontmap_opt,
) -> ::core::ffi::c_int {
    let mut opt: cid_opt = cid_opt {
        csi: CIDSysInfo {
            registry: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            ordering: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            supplement: 0,
        },
        style: 0,
        embed: 0,
        stemv: 0,
    };
    let mut has_csi: ::core::ffi::c_int = 0;
    opt.style = (*fmap_opt).style;
    opt.embed = if (*fmap_opt).flags & FONTMAP_OPT_NOEMBED != 0 {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    opt.csi.registry = ::core::ptr::null_mut::<::core::ffi::c_char>();
    opt.csi.ordering = ::core::ptr::null_mut::<::core::ffi::c_char>();
    opt.csi.supplement = 0 as ::core::ffi::c_int;
    has_csi = get_cidsysinfo(&raw mut opt.csi, map_name, fmap_opt);
    opt.stemv = (*fmap_opt).stemv;
    if has_csi == 0 && !cmap_csi.is_null() {
        opt.csi.registry = new(
            (strlen((*cmap_csi).registry).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy(opt.csi.registry, (*cmap_csi).registry);
        opt.csi.ordering = new(
            (strlen((*cmap_csi).ordering).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy(opt.csi.ordering, (*cmap_csi).ordering);
        opt.csi.supplement = (*cmap_csi).supplement;
        has_csi = 1 as ::core::ffi::c_int;
    }
    if CIDFont_type0_open(
        font,
        map_name,
        (*fmap_opt).index as ::core::ffi::c_int,
        &raw mut opt,
    ) < 0 as ::core::ffi::c_int
        && CIDFont_type2_open(
            font,
            map_name,
            (*fmap_opt).index as ::core::ffi::c_int,
            &raw mut opt,
        ) < 0 as ::core::ffi::c_int
        && CIDFont_type0_open_from_t1(
            font,
            map_name,
            (*fmap_opt).index as ::core::ffi::c_int,
            &raw mut opt,
        ) < 0 as ::core::ffi::c_int
        && CIDFont_type0_open_from_t1c(
            font,
            map_name,
            (*fmap_opt).index as ::core::ffi::c_int,
            &raw mut opt,
        ) < 0 as ::core::ffi::c_int
        && CIDFont_base_open(font, map_name, &raw mut opt) < 0 as ::core::ffi::c_int
    {
        release_opt(&raw mut opt);
        return -(1 as ::core::ffi::c_int);
    }
    (*font).filename = new(
        (strlen(map_name).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*font).filename, map_name);
    (*font).ident = new(
        (strlen(map_name).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*font).ident, map_name);
    (*font).index = (*fmap_opt).index;
    (*font).cid.options = opt;
    if !(*font).cid.csi.registry.is_null() && !(*font).cid.csi.ordering.is_null() {
        if !cmap_csi.is_null() {
            if strcmp((*font).cid.csi.registry, (*cmap_csi).registry) != 0
                || strcmp((*font).cid.csi.ordering, (*cmap_csi).ordering) != 0
            {
                pdf_warning!(
                    b"Inconsistent ROS found:\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
                pdf_message!(
                    b"\tFont: %s-%s-%d\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*font).cid.csi.registry,
                    (*font).cid.csi.ordering,
                    (*font).cid.csi.supplement,
                );
                pdf_message!(
                    b"\tCMap: %s-%s-%d\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*cmap_csi).registry,
                    (*cmap_csi).ordering,
                    (*cmap_csi).supplement,
                );
                pdf_abort!(
                    b"Incompatible CMap specified for this font.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if (*font).cid.csi.supplement < (*cmap_csi).supplement {
                (*font).cid.csi.supplement = (*cmap_csi).supplement;
            }
        }
    } else {
        if !((*font).subtype == 6 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_long
            != 0
        {
            pdf_abort!(
                c"assertion failed: %s".as_ptr(),
                b"font->subtype == PDF_FONT_FONTTYPE_CIDTYPE2\0" as *const u8
                    as *const ::core::ffi::c_char
            );
        } else {
        };
        if !cmap_csi.is_null() {
            (*font).cid.csi.registry = new((strlen((*cmap_csi).registry).wrapping_add(1 as size_t)
                as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            strcpy((*font).cid.csi.registry, (*cmap_csi).registry);
            (*font).cid.csi.ordering = new((strlen((*cmap_csi).ordering).wrapping_add(1 as size_t)
                as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            strcpy((*font).cid.csi.ordering, (*cmap_csi).ordering);
            (*font).cid.csi.supplement = (*cmap_csi).supplement;
        } else {
            (*font).cid.csi.registry = new((strlen(
                b"Adobe\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .wrapping_add(1 as size_t) as uint32_t
                as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            strcpy(
                (*font).cid.csi.registry,
                b"Adobe\0" as *const u8 as *const ::core::ffi::c_char,
            );
            (*font).cid.csi.ordering = new((strlen(
                b"Identity\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .wrapping_add(1 as size_t) as uint32_t
                as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            strcpy(
                (*font).cid.csi.ordering,
                b"Identity\0" as *const u8 as *const ::core::ffi::c_char,
            );
            (*font).cid.csi.supplement = 0 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn release_opt(mut opt: *mut cid_opt) {
    if !(*opt).csi.registry.is_null() {
        free((*opt).csi.registry as *mut ::core::ffi::c_void);
    }
    if !(*opt).csi.ordering.is_null() {
        free((*opt).csi.ordering as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C-unwind" fn get_cidsysinfo(
    mut csi: *mut CIDSysInfo,
    mut map_name: *const ::core::ffi::c_char,
    mut fmap_opt: *const fontmap_opt,
) -> ::core::ffi::c_int {
    let mut has_csi: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sup_idx: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut csi_idx: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut m: ::core::ffi::c_int = 0;
    let mut n: size_t = 0;
    sup_idx = pdf_get_version() - 10 as ::core::ffi::c_int;
    sup_idx = if sup_idx > SUP_IDX_MAX {
        SUP_IDX_MAX
    } else {
        sup_idx
    };
    if fmap_opt.is_null() || (*fmap_opt).charcoll.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while !CIDFont_stdcc_alias[i as usize].name.is_null() {
        n = strlen(CIDFont_stdcc_alias[i as usize].name);
        if !strstartswith((*fmap_opt).charcoll, CIDFont_stdcc_alias[i as usize].name).is_null() {
            csi_idx = CIDFont_stdcc_alias[i as usize].index;
            (*csi).registry = new((strlen(CIDFont_stdcc_def[csi_idx as usize].registry)
                .wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            strcpy(
                (*csi).registry,
                CIDFont_stdcc_def[csi_idx as usize].registry,
            );
            (*csi).ordering = new((strlen(CIDFont_stdcc_def[csi_idx as usize].ordering)
                .wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            strcpy(
                (*csi).ordering,
                CIDFont_stdcc_def[csi_idx as usize].ordering,
            );
            if strlen((*fmap_opt).charcoll) > n {
                (*csi).supplement = strtoul(
                    (*fmap_opt).charcoll.offset(n as isize) as *mut ::core::ffi::c_char,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    10 as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
            } else {
                (*csi).supplement =
                    CIDFont_stdcc_def[csi_idx as usize].supplement[sup_idx as usize];
            }
            has_csi = 1 as ::core::ffi::c_int;
            break;
        } else {
            i += 1;
        }
    }
    if has_csi == 0 {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        p = (*fmap_opt).charcoll;
        p = strchr((*fmap_opt).charcoll, '-' as i32);
        if p.is_null()
            || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
        {
            pdf_abort!(
                b"String can't be converted to REGISTRY-ORDERING-SUPPLEMENT: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*fmap_opt).charcoll,
            );
        }
        p = p.offset(1);
        q = strchr(p, '-' as i32);
        if q.is_null()
            || *q.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32
        {
            pdf_abort!(
                b"String can't be converted to REGISTRY-ORDERING-SUPPLEMENT: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*fmap_opt).charcoll,
            );
        }
        q = q.offset(1);
        if isdigit(
            *q.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                as ::core::ffi::c_int,
        ) == 0
        {
            pdf_abort!(
                b"String can't be converted to REGISTRY-ORDERING-SUPPLEMENT: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*fmap_opt).charcoll,
            );
        }
        n = strlen((*fmap_opt).charcoll)
            .wrapping_sub(strlen(p))
            .wrapping_sub(1 as size_t);
        (*csi).registry = new((n.wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memcpy(
            (*csi).registry as *mut ::core::ffi::c_void,
            (*fmap_opt).charcoll as *const ::core::ffi::c_void,
            n,
        );
        *(*csi).registry.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
        m = strlen(p).wrapping_sub(strlen(q)).wrapping_sub(1 as size_t) as ::core::ffi::c_int;
        (*csi).ordering = new(((m + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memcpy(
            (*csi).ordering as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            m as size_t,
        );
        *(*csi).ordering.offset(m as isize) = '\0' as i32 as ::core::ffi::c_char;
        (*csi).supplement = strtoul(
            q,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        has_csi = 1 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while !CIDFont_stdcc_def[i as usize].ordering.is_null() {
            if !CIDFont_stdcc_def[i as usize].registry.is_null()
                && streq_ptr((*csi).registry, CIDFont_stdcc_def[i as usize].registry)
                    as ::core::ffi::c_int
                    != 0
                && streq_ptr((*csi).ordering, CIDFont_stdcc_def[i as usize].ordering)
                    as ::core::ffi::c_int
                    != 0
            {
                csi_idx = i;
                break;
            } else {
                i += 1;
            }
        }
    }
    if !csi.is_null() && csi_idx >= 0 as ::core::ffi::c_int {
        if (*csi).supplement > CIDFont_stdcc_def[csi_idx as usize].supplement[sup_idx as usize]
            && (*fmap_opt).flags & FONTMAP_OPT_NOEMBED != 0
        {
            pdf_warning!(
                b"Highest supplement number supported in PDF-%d.%d for %s-%s is %d.\0" as *const u8
                    as *const ::core::ffi::c_char,
                pdf_get_version_major(),
                pdf_get_version_minor(),
                (*csi).registry,
                (*csi).ordering,
                CIDFont_stdcc_def[csi_idx as usize].supplement[sup_idx as usize],
            );
            pdf_warning!(
                b"Some character may not shown without embedded font (--> %s).\0" as *const u8
                    as *const ::core::ffi::c_char,
                map_name,
            );
        }
    }
    return has_csi;
}
#[unsafe(export_name = "__pdf_impl_CIDFont_set_flags")]
pub unsafe extern "C-unwind" fn CIDFont_set_flags(mut flags: ::core::ffi::c_int) {
    opt_flags_cidfont |= flags;
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_is_ACCFont(
    mut font: *mut pdf_font,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_is_ACCFont(font) }) {
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_is_UCSFont(
    mut font: *mut pdf_font,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_is_UCSFont(font) }) {
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_get_usedchars(
    mut font: *mut pdf_font,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_get_usedchars(font) }) {
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_get_usedchars_v(
    mut font: *mut pdf_font,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_get_usedchars_v(font) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_font_load_cidfont(mut font: *mut pdf_font) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_load_cidfont(font) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_cidfont_lookup_cache(
    mut fonts: *mut pdf_font,
    mut count: ::core::ffi::c_int,
    mut map_name: *const ::core::ffi::c_char,
    mut cmap_csi: *mut CIDSysInfo,
    mut fmap_opt: *const fontmap_opt,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_font_cidfont_lookup_cache(fonts, count, map_name, cmap_csi, fmap_opt)
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
pub unsafe extern "C" fn __pdf_entry_pdf_font_open_cidfont(
    mut font: *mut pdf_font,
    mut map_name: *const ::core::ffi::c_char,
    mut cmap_csi: *mut CIDSysInfo,
    mut fmap_opt: *const fontmap_opt,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_font_open_cidfont(font, map_name, cmap_csi, fmap_opt)
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
pub unsafe extern "C" fn __pdf_entry_CIDFont_set_flags(mut flags: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CIDFont_set_flags(flags) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
