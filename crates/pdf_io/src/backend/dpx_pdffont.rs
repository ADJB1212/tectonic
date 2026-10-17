/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2008-2018 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __big: *const ::core::ffi::c_char,
        __little: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    #[link_name = "__pdf_impl_pdf_lookup_fontmap_record"]
    fn pdf_lookup_fontmap_record(kp: *const ::core::ffi::c_char) -> *mut fontmap_rec;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ref_obj"]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_link_obj"]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_lookup_dict"]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(
        dict: *mut pdf_obj,
        key: *mut pdf_obj,
        value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_stream_length"]
    fn pdf_stream_length(stream: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_agl_init_map"]
    fn agl_init_map();
    #[link_name = "__pdf_impl_agl_close_map"]
    fn agl_close_map();
    #[link_name = "__pdf_impl_pdf_font_cidfont_lookup_cache"]
    fn pdf_font_cidfont_lookup_cache(
        fonts: *mut pdf_font,
        count: ::core::ffi::c_int,
        map_name: *const ::core::ffi::c_char,
        cmap_csi: *mut CIDSysInfo,
        fmap_opt: *const fontmap_opt,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_open_cidfont"]
    fn pdf_font_open_cidfont(
        font: *mut pdf_font,
        map_name: *const ::core::ffi::c_char,
        cmap_csi: *mut CIDSysInfo,
        fmap_opt: *const fontmap_opt,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_load_cidfont"]
    fn pdf_font_load_cidfont(font: *mut pdf_font);
    #[link_name = "__pdf_impl_t1_load_UnicodeCMap"]
    fn t1_load_UnicodeCMap(
        font_name: *const ::core::ffi::c_char,
        otl_tags: *const ::core::ffi::c_char,
        wmode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_is_Identity"]
    fn CMap_is_Identity(cmap: *mut CMap) -> bool;
    #[link_name = "__pdf_impl_CMap_get_wmode"]
    fn CMap_get_wmode(cmap: *mut CMap) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_get_CIDSysInfo"]
    fn CMap_get_CIDSysInfo(cmap: *mut CMap) -> *mut CIDSysInfo;
    #[link_name = "__pdf_impl_CMap_cache_init"]
    fn CMap_cache_init();
    #[link_name = "__pdf_impl_CMap_cache_get"]
    fn CMap_cache_get(id: ::core::ffi::c_int) -> *mut CMap;
    #[link_name = "__pdf_impl_CMap_cache_find"]
    fn CMap_cache_find(cmap_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_cache_close"]
    fn CMap_cache_close();
    static mut dpx_conf: _dpx_conf;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_init_genrand"]
    fn init_genrand(s: ::core::ffi::c_ulonglong);
    #[link_name = "__pdf_impl_genrand_int31"]
    fn genrand_int31() -> ::core::ffi::c_long;
    #[link_name = "__pdf_impl_pdf_init_encodings"]
    fn pdf_init_encodings();
    #[link_name = "__pdf_impl_pdf_close_encodings"]
    fn pdf_close_encodings();
    #[link_name = "__pdf_impl_pdf_encoding_complete"]
    fn pdf_encoding_complete();
    #[link_name = "__pdf_impl_pdf_encoding_findresource"]
    fn pdf_encoding_findresource(enc_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_encoding_obj"]
    fn pdf_get_encoding_obj(enc_id: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_encoding_is_predefined"]
    fn pdf_encoding_is_predefined(enc_id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_encoding_get_name"]
    fn pdf_encoding_get_name(enc_id: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_encoding_add_usedchars"]
    fn pdf_encoding_add_usedchars(
        encoding_id: ::core::ffi::c_int,
        is_used: *const ::core::ffi::c_char,
    );
    #[link_name = "__pdf_impl_pdf_encoding_get_tounicode"]
    fn pdf_encoding_get_tounicode(encoding_id: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_load_ToUnicode_stream"]
    fn pdf_load_ToUnicode_stream(ident: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_font_open_pkfont"]
    fn pdf_font_open_pkfont(
        font: *mut pdf_font,
        ident: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
        encoding_id: ::core::ffi::c_int,
        embedding: ::core::ffi::c_int,
        point_size: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_load_pkfont"]
    fn pdf_font_load_pkfont(font: *mut pdf_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_PKFont_set_dpi"]
    fn PKFont_set_dpi(dpi: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_font_open_truetype"]
    fn pdf_font_open_truetype(
        font: *mut pdf_font,
        ident: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
        encoding_id: ::core::ffi::c_int,
        embedding: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_load_truetype"]
    fn pdf_font_load_truetype(font: *mut pdf_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otf_load_Unicode_CMap"]
    fn otf_load_Unicode_CMap(
        map_name: *const ::core::ffi::c_char,
        ttc_index: uint32_t,
        otl_opts: *const ::core::ffi::c_char,
        wmode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otf_try_load_GID_to_CID_map"]
    fn otf_try_load_GID_to_CID_map(
        map_name: *const ::core::ffi::c_char,
        ttc_index: uint32_t,
        wmode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_load_type0"]
    fn pdf_font_load_type0(font: *mut pdf_font);
    #[link_name = "__pdf_impl_pdf_font_open_type0"]
    fn pdf_font_open_type0(
        font: *mut pdf_font,
        cid_id: ::core::ffi::c_int,
        wmode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_open_type1"]
    fn pdf_font_open_type1(
        font: *mut pdf_font,
        ident: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
        encoding_id: ::core::ffi::c_int,
        embedding: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_load_type1"]
    fn pdf_font_load_type1(font: *mut pdf_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_open_type1c"]
    fn pdf_font_open_type1c(
        font: *mut pdf_font,
        ident: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
        encoding_id: ::core::ffi::c_int,
        embedding: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_load_type1c"]
    fn pdf_font_load_type1c(font: *mut pdf_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tfm_open"]
    fn tfm_open(
        tex_name: *const ::core::ffi::c_char,
        must_exist: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tfm_get_width"]
    fn tfm_get_width(font_id: ::core::ffi::c_int, ch: int32_t) -> ::core::ffi::c_double;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
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
    pub type0: C2RustUnnamed_1,
    pub cid: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub csi: CIDSysInfo,
    pub options: cid_opt,
    pub need_vmetrics: ::core::ffi::c_int,
    pub usedchars_v: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub descendant: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub count: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
    pub fonts: *mut pdf_font,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub keep_cache: ::core::ffi::c_int,
}
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_5,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: ::core::ffi::c_int,
    pub profile: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut ::core::ffi::c_uchar,
    pub prev: *mut mapData,
    pub pos: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: ::core::ffi::c_int,
    pub len: size_t,
    pub code: *mut ::core::ffi::c_uchar,
    pub next: *mut mapDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub num: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
    pub ranges: *mut rangeDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rangeDef {
    pub dim: size_t,
    pub codeLo: *mut ::core::ffi::c_uchar,
    pub codeHi: *mut ::core::ffi::c_uchar,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const FONTMAP_OPT_NOEMBED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const FONTMAP_OPT_VERT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const FONTMAP_STYLE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PDF_NAME: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PDF_STREAM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const PDF_FONT_FONTTYPE_TYPE1: ::core::ffi::c_int = 0;
pub const PDF_FONT_FONTTYPE_TYPE1C: ::core::ffi::c_int = 1;
pub const PDF_FONT_FONTTYPE_TYPE3: ::core::ffi::c_int = 2;
pub const PDF_FONT_FONTTYPE_TRUETYPE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PDF_FONT_FONTTYPE_TYPE0: ::core::ffi::c_int = 4;
pub const PDF_FONT_FONTTYPE_CIDTYPE0: ::core::ffi::c_int = 5;
pub const PDF_FONT_FONTTYPE_CIDTYPE2: ::core::ffi::c_int = 6;
pub const PDF_FONT_FLAG_NOEMBED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PDF_FONT_FLAG_BASEFONT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const PDF_FONT_FLAG_USEDCHAR_SHARED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const PDF_FONT_FLAG_IS_ALIAS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const PDF_FONT_FLAG_IS_REENCODE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const FONT_STYLE_NONE: ::core::ffi::c_int = FONTMAP_STYLE_NONE;
#[unsafe(export_name = "__pdf_impl_pdf_font_set_dpi")]
pub unsafe extern "C-unwind" fn pdf_font_set_dpi(mut font_dpi: ::core::ffi::c_int) {
    PKFont_set_dpi(font_dpi);
}
static mut unique_tag_state: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut unique_tags_deterministic: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_pdf_font_reset_unique_tag_state")]
pub unsafe extern "C-unwind" fn pdf_font_reset_unique_tag_state() {
    unique_tag_state = 1 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_set_deterministic_unique_tags")]
pub unsafe extern "C-unwind" fn pdf_font_set_deterministic_unique_tags(
    mut value: ::core::ffi::c_int,
) {
    unique_tags_deterministic = value;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_make_uniqueTag")]
pub unsafe extern "C-unwind" fn pdf_font_make_uniqueTag(mut tag: *mut ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_char = 0;
    if unique_tags_deterministic != 0 {
        snprintf(
            tag,
            7 as size_t,
            b"%06d\0" as *const u8 as *const ::core::ffi::c_char,
            unique_tag_state,
        );
        unique_tag_state += 1;
        return;
    }
    if unique_tag_state != 0 {
        init_genrand(0 as ::core::ffi::c_ulonglong);
        unique_tag_state = 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 6 as ::core::ffi::c_int {
        ch = (genrand_int31() % 26 as ::core::ffi::c_long) as ::core::ffi::c_char;
        *tag.offset(i as isize) = (ch as ::core::ffi::c_int + 'A' as i32) as ::core::ffi::c_char;
        i += 1;
    }
    *tag.offset(6 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
}
unsafe extern "C-unwind" fn init_CIDSysInfo(mut csi: *mut CIDSysInfo) {
    (*csi).registry = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*csi).ordering = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*csi).supplement = 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdf_init_font_struct(mut font: *mut pdf_font) {
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*font).ident = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).subtype = -(1 as ::core::ffi::c_int);
    (*font).font_id = -(1 as ::core::ffi::c_int);
    (*font).fontname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    memset(
        &raw mut (*font).uniqueID as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        7 as size_t,
    );
    (*font).index = 0 as uint32_t;
    (*font).encoding_id = -(1 as ::core::ffi::c_int);
    (*font).reference = ::core::ptr::null_mut::<pdf_obj>();
    (*font).resource = ::core::ptr::null_mut::<pdf_obj>();
    (*font).descriptor = ::core::ptr::null_mut::<pdf_obj>();
    (*font).point_size = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*font).design_size = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*font).usedchars = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).flags = 0 as ::core::ffi::c_int;
    (*font).type0.descendant = -(1 as ::core::ffi::c_int);
    (*font).type0.wmode = 0 as ::core::ffi::c_int;
    init_CIDSysInfo(&raw mut (*font).cid.csi);
    (*font).cid.need_vmetrics = 0 as ::core::ffi::c_int;
    (*font).cid.usedchars_v = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).cid.options.embed = 0 as ::core::ffi::c_int;
    (*font).cid.options.style = FONT_STYLE_NONE;
    (*font).cid.options.stemv = 0 as ::core::ffi::c_int;
    init_CIDSysInfo(&raw mut (*font).cid.options.csi);
}
unsafe extern "C-unwind" fn pdf_flush_font(mut font: *mut pdf_font) {
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut uniqueTag: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if font.is_null() {
        return;
    }
    if (*font).flags & PDF_FONT_FLAG_IS_ALIAS != 0 || (*font).flags & PDF_FONT_FLAG_IS_REENCODE != 0
    {
        return;
    }
    if !(*font).resource.is_null() && !(*font).reference.is_null() {
        match (*font).subtype {
            PDF_FONT_FONTTYPE_TYPE3
            | PDF_FONT_FONTTYPE_TYPE0
            | PDF_FONT_FONTTYPE_CIDTYPE0
            | PDF_FONT_FONTTYPE_CIDTYPE2 => {}
            _ => {
                if (*font).flags & PDF_FONT_FLAG_NOEMBED != 0 {
                    pdf_add_dict(
                        (*font).resource,
                        pdf_new_name(b"BaseFont\0" as *const u8 as *const ::core::ffi::c_char),
                        pdf_new_name((*font).fontname),
                    );
                    if !(*font).descriptor.is_null() {
                        pdf_add_dict(
                            (*font).descriptor,
                            pdf_new_name(b"FontName\0" as *const u8 as *const ::core::ffi::c_char),
                            pdf_new_name((*font).fontname),
                        );
                    }
                } else {
                    if (*font).fontname.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0
                    {
                        pdf_abort!(
                            c"assertion failed: %s".as_ptr(),
                            b"font->fontname\0" as *const u8 as *const ::core::ffi::c_char
                        );
                    } else {
                    };
                    fontname = new(((7 as size_t)
                        .wrapping_add(strlen((*font).fontname))
                        .wrapping_add(1 as size_t) as uint32_t
                        as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                        as uint32_t) as *mut ::core::ffi::c_char;
                    uniqueTag = pdf_font_get_uniqueTag(font);
                    sprintf(
                        fontname,
                        b"%6s+%s\0" as *const u8 as *const ::core::ffi::c_char,
                        uniqueTag,
                        (*font).fontname,
                    );
                    pdf_add_dict(
                        (*font).resource,
                        pdf_new_name(b"BaseFont\0" as *const u8 as *const ::core::ffi::c_char),
                        pdf_new_name(fontname),
                    );
                    if !(*font).descriptor.is_null() {
                        pdf_add_dict(
                            (*font).descriptor,
                            pdf_new_name(b"FontName\0" as *const u8 as *const ::core::ffi::c_char),
                            pdf_new_name(fontname),
                        );
                    }
                    free(fontname as *mut ::core::ffi::c_void);
                }
                if !(*font).descriptor.is_null() {
                    pdf_add_dict(
                        (*font).resource,
                        pdf_new_name(
                            b"FontDescriptor\0" as *const u8 as *const ::core::ffi::c_char,
                        ),
                        pdf_ref_obj((*font).descriptor),
                    );
                }
            }
        }
    }
    pdf_release_obj((*font).resource);
    pdf_release_obj((*font).descriptor);
    pdf_release_obj((*font).reference);
    (*font).reference = ::core::ptr::null_mut::<pdf_obj>();
    (*font).resource = ::core::ptr::null_mut::<pdf_obj>();
    (*font).descriptor = ::core::ptr::null_mut::<pdf_obj>();
}
unsafe extern "C-unwind" fn pdf_clean_font_struct(mut font: *mut pdf_font) {
    if font.is_null() {
        return;
    }
    if !(*font).resource.is_null() {
        pdf_warning!(
            b"font \"%s\" not properly released?\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).ident,
        );
    }
    free((*font).ident as *mut ::core::ffi::c_void);
    free((*font).filename as *mut ::core::ffi::c_void);
    free((*font).fontname as *mut ::core::ffi::c_void);
    if !(*font).usedchars.is_null() {
        if (*font).flags & PDF_FONT_FLAG_USEDCHAR_SHARED == 0 {
            free((*font).usedchars as *mut ::core::ffi::c_void);
        }
    }
    if !(*font).cid.csi.registry.is_null() {
        free((*font).cid.csi.registry as *mut ::core::ffi::c_void);
    }
    if !(*font).cid.csi.ordering.is_null() {
        free((*font).cid.csi.ordering as *mut ::core::ffi::c_void);
    }
    if !(*font).cid.options.csi.registry.is_null() {
        free((*font).cid.options.csi.registry as *mut ::core::ffi::c_void);
    }
    if !(*font).cid.options.csi.ordering.is_null() {
        free((*font).cid.options.csi.ordering as *mut ::core::ffi::c_void);
    }
    if !(*font).cid.usedchars_v.is_null() {
        free((*font).cid.usedchars_v as *mut ::core::ffi::c_void);
    }
    (*font).ident = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).fontname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).usedchars = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).cid.csi.registry = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).cid.csi.ordering = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).cid.options.csi.registry = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).cid.options.csi.ordering = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).cid.usedchars_v = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
pub const CACHE_ALLOC_SIZE: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
static mut font_cache: C2RustUnnamed_2 = C2RustUnnamed_2 {
    count: 0 as ::core::ffi::c_int,
    capacity: 0 as ::core::ffi::c_int,
    fonts: ::core::ptr::null::<pdf_font>() as *mut pdf_font,
};
#[unsafe(export_name = "__pdf_impl_pdf_init_fonts")]
pub unsafe extern "C-unwind" fn pdf_init_fonts() {
    if !font_cache.fonts.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font_cache.fonts == NULL\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    agl_init_map();
    CMap_cache_init();
    pdf_init_encodings();
    font_cache.count = 0 as ::core::ffi::c_int;
    font_cache.capacity = CACHE_ALLOC_SIZE as ::core::ffi::c_int;
    font_cache.fonts = new((font_cache.capacity as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_font>() as usize)
        as uint32_t) as *mut pdf_font;
}
#[unsafe(export_name = "__pdf_impl_pdf_abort_fonts")]
pub unsafe extern "C-unwind" fn pdf_abort_fonts() {
    let mut i: ::core::ffi::c_int = 0;
    if font_cache.fonts.is_null() {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < font_cache.count {
        let mut font: *mut pdf_font = font_cache.fonts.offset(i as isize) as *mut pdf_font;
        if (*font).flags & (PDF_FONT_FLAG_IS_ALIAS | PDF_FONT_FLAG_IS_REENCODE) == 0 {
            pdf_release_obj((*font).resource);
            pdf_release_obj((*font).descriptor);
            pdf_release_obj((*font).reference);
        }
        (*font).reference = ::core::ptr::null_mut::<pdf_obj>();
        (*font).descriptor = (*font).reference;
        (*font).resource = (*font).descriptor;
        pdf_clean_font_struct(font);
        i += 1;
    }
    free(font_cache.fonts as *mut ::core::ffi::c_void);
    font_cache.fonts = ::core::ptr::null_mut::<pdf_font>();
    font_cache.capacity = 0 as ::core::ffi::c_int;
    font_cache.count = font_cache.capacity;
    CMap_cache_close();
    pdf_close_encodings();
    agl_close_map();
}
unsafe extern "C-unwind" fn GET_FONT(mut font_id: ::core::ffi::c_int) -> *mut pdf_font {
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    if font_id >= 0 as ::core::ffi::c_int && font_id < font_cache.count {
        font = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
        if (*font).flags & PDF_FONT_FLAG_IS_ALIAS != 0 {
            font = font_cache.fonts.offset((*font).font_id as isize) as *mut pdf_font;
        }
    }
    return font;
}
#[unsafe(export_name = "__pdf_impl_pdf_get_font_data")]
pub unsafe extern "C-unwind" fn pdf_get_font_data(
    mut font_id: ::core::ffi::c_int,
) -> *mut pdf_font {
    if font_id < 0 as ::core::ffi::c_int || font_id >= font_cache.count {
        pdf_abort!(
            b"Invalid font ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            font_id,
        );
    }
    return font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
}
#[unsafe(export_name = "__pdf_impl_pdf_get_font_ident")]
pub unsafe extern "C-unwind" fn pdf_get_font_ident(
    mut font_id: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    if font_id < 0 as ::core::ffi::c_int || font_id >= font_cache.count {
        pdf_abort!(
            b"Invalid font ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            font_id,
        );
    }
    font = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
    return (*font).ident;
}
#[unsafe(export_name = "__pdf_impl_pdf_get_font_reference")]
pub unsafe extern "C-unwind" fn pdf_get_font_reference(
    mut font_id: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    if font_id < 0 as ::core::ffi::c_int || font_id >= font_cache.count {
        pdf_abort!(
            b"Invalid font ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            font_id,
        );
    }
    font = GET_FONT(font_id);
    if (*font).flags & PDF_FONT_FLAG_IS_REENCODE != 0 {
        font = GET_FONT((*font).font_id);
    }
    if (*font).reference.is_null() {
        (*font).reference = pdf_ref_obj(pdf_font_get_resource(font));
    }
    if (*font).subtype == PDF_FONT_FONTTYPE_TYPE0 {
        if pdf_lookup_dict(
            (*font).resource,
            b"DescendantFonts\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            let mut array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            array = pdf_new_array();
            pdf_add_array(array, pdf_get_font_reference((*font).type0.descendant));
            pdf_add_dict(
                (*font).resource,
                pdf_new_name(b"DescendantFonts\0" as *const u8 as *const ::core::ffi::c_char),
                array,
            );
        }
    }
    return pdf_link_obj((*font).reference);
}
#[unsafe(export_name = "__pdf_impl_pdf_get_font_resource")]
pub unsafe extern "C-unwind" fn pdf_get_font_resource(
    mut font_id: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    if font_id < 0 as ::core::ffi::c_int || font_id >= font_cache.count {
        pdf_abort!(
            b"Invalid font ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            font_id,
        );
    }
    font = GET_FONT(font_id);
    if (*font).flags & PDF_FONT_FLAG_IS_REENCODE != 0 {
        font = GET_FONT((*font).font_id);
    }
    if (*font).resource.is_null() {
        (*font).resource = pdf_new_dict();
    }
    return (*font).resource;
}
#[unsafe(export_name = "__pdf_impl_pdf_get_font_usedchars")]
pub unsafe extern "C-unwind" fn pdf_get_font_usedchars(
    mut font_id: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    if font_id < 0 as ::core::ffi::c_int || font_id >= font_cache.count {
        pdf_abort!(
            b"Invalid font ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            font_id,
        );
    }
    font = GET_FONT(font_id);
    if (*font).flags & PDF_FONT_FLAG_IS_REENCODE != 0 {
        font = GET_FONT((*font).font_id);
    }
    if (*font).subtype != PDF_FONT_FONTTYPE_TYPE0 {
        if (*font).usedchars.is_null() {
            (*font).usedchars = new((256 as ::core::ffi::c_int as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t) as *mut ::core::ffi::c_char;
            memset(
                (*font).usedchars as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (256 as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
            );
        }
    }
    return (*font).usedchars;
}
#[unsafe(export_name = "__pdf_impl_pdf_get_font_wmode")]
pub unsafe extern "C-unwind" fn pdf_get_font_wmode(
    mut font_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    if font_id < 0 as ::core::ffi::c_int || font_id >= font_cache.count {
        pdf_abort!(
            b"Invalid font ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            font_id,
        );
    }
    font = GET_FONT(font_id);
    if (*font).flags & PDF_FONT_FLAG_IS_REENCODE != 0 {
        font = GET_FONT((*font).font_id);
    }
    if (*font).subtype == PDF_FONT_FONTTYPE_TYPE0 {
        return (*font).type0.wmode;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_get_font_subtype")]
pub unsafe extern "C-unwind" fn pdf_get_font_subtype(
    mut font_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    if font_id < 0 as ::core::ffi::c_int || font_id >= font_cache.count {
        pdf_abort!(
            b"Invalid font ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            font_id,
        );
    }
    font = GET_FONT(font_id);
    if (*font).flags & PDF_FONT_FLAG_IS_REENCODE != 0 {
        font = GET_FONT((*font).font_id);
    }
    return (*font).subtype;
}
#[unsafe(export_name = "__pdf_impl_pdf_get_font_encoding")]
pub unsafe extern "C-unwind" fn pdf_get_font_encoding(
    mut font_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    if font_id < 0 as ::core::ffi::c_int || font_id >= font_cache.count {
        pdf_abort!(
            b"Invalid font ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            font_id,
        );
    }
    font = GET_FONT(font_id);
    return (*font).encoding_id;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_resource_name")]
pub unsafe extern "C-unwind" fn pdf_font_resource_name(
    mut font_id: ::core::ffi::c_int,
    mut buff: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    if font_id < 0 as ::core::ffi::c_int || font_id >= font_cache.count {
        pdf_abort!(
            b"Invalid font ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            font_id,
        );
    }
    font = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
    if (*font).flags & PDF_FONT_FLAG_IS_ALIAS != 0 {
        font_id = (*font).font_id;
    }
    font = GET_FONT(font_id);
    if (*font).flags & PDF_FONT_FLAG_IS_REENCODE != 0 {
        font_id = (*font).font_id;
    }
    len = sprintf(
        buff,
        b"F%d\0" as *const u8 as *const ::core::ffi::c_char,
        font_id,
    );
    return len;
}
unsafe extern "C-unwind" fn try_load_ToUnicode_CMap(mut font: *mut pdf_font) -> ::core::ffi::c_int {
    let mut tounicode: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cmap_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut mrec: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*font).subtype == PDF_FONT_FONTTYPE_TYPE0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*font).ident.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font->ident\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    mrec = pdf_lookup_fontmap_record((*font).ident);
    if !mrec.is_null() && !(*mrec).opt.tounicode.is_null() {
        cmap_name = (*mrec).opt.tounicode;
    } else {
        cmap_name = (*font).ident;
    }
    tounicode = pdf_load_ToUnicode_stream(cmap_name);
    if tounicode.is_null() {
        if !mrec.is_null() && !(*mrec).opt.tounicode.is_null() {
            pdf_warning!(
                b"Failed to read ToUnicode mapping \"%s\"...\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*mrec).opt.tounicode,
            );
        }
    } else {
        if pdf_obj_typeof(tounicode) != PDF_STREAM {
            pdf_abort!(
                b"Object returned by pdf_load_ToUnicode_stream() not stream object! (This must be bug)\0"
                as * const u8 as * const ::core::ffi::c_char,
            );
        } else if pdf_stream_length(tounicode) > 0 as ::core::ffi::c_int {
            let mut fontdict: *mut pdf_obj = pdf_font_get_resource(font);
            pdf_add_dict(
                fontdict,
                pdf_new_name(b"ToUnicode\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_ref_obj(tounicode),
            );
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(
                    b"pdf_font>> ToUnicode CMap \"%s\" attached to font id=\"%s\".\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    cmap_name,
                    (*font).ident,
                );
            }
        }
        pdf_release_obj(tounicode);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_close_fonts")]
pub unsafe extern "C-unwind" fn pdf_close_fonts() {
    let mut font_id: ::core::ffi::c_int = 0;
    font_id = 0 as ::core::ffi::c_int;
    while font_id < font_cache.count {
        let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
        font = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
        if !((*font).flags & PDF_FONT_FLAG_IS_ALIAS != 0
            || (*font).flags & PDF_FONT_FLAG_IS_REENCODE != 0
            || (*font).reference.is_null())
        {
            if !((*font).subtype == PDF_FONT_FONTTYPE_CIDTYPE0
                || (*font).subtype == PDF_FONT_FONTTYPE_CIDTYPE2)
            {
                if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                    if (*font).subtype != PDF_FONT_FONTTYPE_TYPE0 {
                        pdf_message!(
                            b"(%s\0" as *const u8 as *const ::core::ffi::c_char,
                            (*font).filename,
                        );
                        if dpx_conf.verbose_level > 2 as ::core::ffi::c_int
                            && (*font).flags & PDF_FONT_FLAG_NOEMBED == 0
                        {
                            pdf_message!(
                                b"[%s+%s]\0" as *const u8 as *const ::core::ffi::c_char,
                                pdf_font_get_uniqueTag(font),
                                (*font).fontname,
                            );
                        } else if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
                            pdf_message!(
                                b"[%s]\0" as *const u8 as *const ::core::ffi::c_char,
                                (*font).fontname,
                            );
                        }
                        if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
                            if (*font).encoding_id >= 0 as ::core::ffi::c_int {
                                pdf_message!(
                                    b"[%s]\0" as *const u8 as *const ::core::ffi::c_char,
                                    pdf_encoding_get_name((*font).encoding_id),
                                );
                            } else {
                                pdf_message!(
                                    b"[built-in]\0" as *const u8 as *const ::core::ffi::c_char,
                                );
                            }
                        }
                    }
                }
                try_load_ToUnicode_CMap(font);
                match (*font).subtype {
                    PDF_FONT_FONTTYPE_TYPE1 => {
                        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                            pdf_message!(b"[Type1]\0" as *const u8 as *const ::core::ffi::c_char,);
                        }
                        if (*font).flags & PDF_FONT_FLAG_BASEFONT == 0 {
                            pdf_font_load_type1(font);
                        }
                    }
                    PDF_FONT_FONTTYPE_TYPE1C => {
                        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                            pdf_message!(b"[Type1C]\0" as *const u8 as *const ::core::ffi::c_char,);
                        }
                        pdf_font_load_type1c(font);
                    }
                    PDF_FONT_FONTTYPE_TRUETYPE => {
                        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                            pdf_message!(
                                b"[TrueType]\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                        }
                        pdf_font_load_truetype(font);
                    }
                    PDF_FONT_FONTTYPE_TYPE3 => {
                        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                            pdf_message!(
                                b"[Type3/PK]\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                        }
                        pdf_font_load_pkfont(font);
                    }
                    PDF_FONT_FONTTYPE_TYPE0 => {
                        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                            pdf_message!(b"[Type0]\0" as *const u8 as *const ::core::ffi::c_char,);
                        }
                        pdf_font_load_type0(font);
                    }
                    _ => {}
                }
                if (*font).encoding_id >= 0 as ::core::ffi::c_int {
                    if (*font).subtype != PDF_FONT_FONTTYPE_TYPE0 {
                        pdf_encoding_add_usedchars((*font).encoding_id, (*font).usedchars);
                    }
                }
                if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                    if (*font).subtype != PDF_FONT_FONTTYPE_TYPE0 {
                        pdf_message!(b")\0" as *const u8 as *const ::core::ffi::c_char);
                    }
                }
            }
        }
        font_id += 1;
    }
    pdf_encoding_complete();
    font_id = 0 as ::core::ffi::c_int;
    while font_id < font_cache.count {
        let mut font_0: *mut pdf_font = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
        if !((*font_0).flags & PDF_FONT_FLAG_IS_ALIAS != 0
            || (*font_0).flags & PDF_FONT_FLAG_IS_REENCODE != 0
            || (*font_0).reference.is_null())
        {
            match (*font_0).subtype {
                PDF_FONT_FONTTYPE_CIDTYPE0 | PDF_FONT_FONTTYPE_CIDTYPE2 => {
                    pdf_font_load_cidfont(font_0);
                }
                _ => {}
            }
        }
        font_id += 1;
    }
    font_id = 0 as ::core::ffi::c_int;
    while font_id < font_cache.count {
        let mut font_1: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
        font_1 = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
        if (*font_1).flags & PDF_FONT_FLAG_IS_ALIAS != 0
            || (*font_1).flags & PDF_FONT_FLAG_IS_REENCODE != 0
            || (*font_1).reference.is_null()
        {
            pdf_flush_font(font_1);
            pdf_clean_font_struct(font_1);
        } else {
            if (*font_1).encoding_id >= 0 as ::core::ffi::c_int
                && (*font_1).subtype != PDF_FONT_FONTTYPE_TYPE0
                && (*font_1).subtype != PDF_FONT_FONTTYPE_CIDTYPE0
                && (*font_1).subtype != PDF_FONT_FONTTYPE_CIDTYPE2
            {
                let mut enc_obj: *mut pdf_obj = pdf_get_encoding_obj((*font_1).encoding_id);
                let mut tounicode: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                if !enc_obj.is_null() {
                    if (*font_1).subtype == PDF_FONT_FONTTYPE_TRUETYPE {
                        if pdf_encoding_is_predefined((*font_1).encoding_id) != 0
                            && (!enc_obj.is_null() && pdf_obj_typeof(enc_obj) == PDF_NAME)
                        {
                            pdf_add_dict(
                                (*font_1).resource,
                                pdf_new_name(
                                    b"Encoding\0" as *const u8 as *const ::core::ffi::c_char,
                                ),
                                pdf_link_obj(enc_obj),
                            );
                        }
                    } else {
                        pdf_add_dict(
                            (*font_1).resource,
                            pdf_new_name(b"Encoding\0" as *const u8 as *const ::core::ffi::c_char),
                            if !enc_obj.is_null() && pdf_obj_typeof(enc_obj) == PDF_NAME {
                                pdf_link_obj(enc_obj)
                            } else {
                                pdf_ref_obj(enc_obj)
                            },
                        );
                    }
                }
                if pdf_lookup_dict(
                    (*font_1).resource,
                    b"ToUnicode\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null()
                    && {
                        tounicode = pdf_encoding_get_tounicode((*font_1).encoding_id);
                        !tounicode.is_null()
                    }
                {
                    if !tounicode.is_null() {
                        pdf_add_dict(
                            (*font_1).resource,
                            pdf_new_name(b"ToUnicode\0" as *const u8 as *const ::core::ffi::c_char),
                            pdf_ref_obj(tounicode),
                        );
                    }
                }
            } else if (*font_1).subtype == PDF_FONT_FONTTYPE_TRUETYPE {
                pdf_add_dict(
                    (*font_1).resource,
                    pdf_new_name(b"Encoding\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_new_name(b"MacRomanEncoding\0" as *const u8 as *const ::core::ffi::c_char),
                );
            }
            pdf_flush_font(font_1);
            pdf_clean_font_struct(font_1);
        }
        font_id += 1;
    }
    free(font_cache.fonts as *mut ::core::ffi::c_void);
    font_cache.fonts = ::core::ptr::null_mut::<pdf_font>();
    font_cache.count = 0 as ::core::ffi::c_int;
    font_cache.capacity = 0 as ::core::ffi::c_int;
    CMap_cache_close();
    pdf_close_encodings();
    agl_close_map();
}
#[unsafe(export_name = "__pdf_impl_pdf_font_findresource")]
pub unsafe extern "C-unwind" fn pdf_font_findresource(
    mut ident: *const ::core::ffi::c_char,
    mut scale: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut font_id: ::core::ffi::c_int = 0;
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    font_id = 0 as ::core::ffi::c_int;
    while font_id < font_cache.count {
        let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
        font = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
        match (*font).subtype {
            PDF_FONT_FONTTYPE_TYPE1
            | PDF_FONT_FONTTYPE_TYPE1C
            | PDF_FONT_FONTTYPE_TRUETYPE
            | PDF_FONT_FONTTYPE_TYPE0 => {
                if strcmp(ident, (*font).ident) == 0 {
                    found = 1 as ::core::ffi::c_int;
                }
            }
            PDF_FONT_FONTTYPE_TYPE3 => {
                if strcmp(ident, (*font).ident) == 0 && scale == (*font).point_size {
                    found = 1 as ::core::ffi::c_int;
                }
            }
            _ => {}
        }
        if found != 0 {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(
                    b"\npdf_font>> Font \"%s\" (enc_id=%d) found at id=%d.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*font).ident,
                    (*font).encoding_id,
                    font_id,
                );
            }
            break;
        } else {
            font_id += 1;
        }
    }
    return if found != 0 {
        font_id
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
unsafe extern "C-unwind" fn create_font_alias(
    mut ident: *const ::core::ffi::c_char,
    mut font_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut this_id: ::core::ffi::c_int = 0;
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    let mut src: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    if font_id < 0 as ::core::ffi::c_int || font_id >= font_cache.count {
        return -(1 as ::core::ffi::c_int);
    }
    src = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
    this_id = font_cache.count;
    if font_cache.count >= font_cache.capacity {
        font_cache.capacity = (font_cache.capacity as ::core::ffi::c_uint)
            .wrapping_add(CACHE_ALLOC_SIZE) as ::core::ffi::c_int
            as ::core::ffi::c_int;
        font_cache.fonts = renew(
            font_cache.fonts as *mut ::core::ffi::c_void,
            (font_cache.capacity as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<pdf_font>() as usize) as uint32_t,
        ) as *mut pdf_font;
    }
    font = font_cache.fonts.offset(this_id as isize) as *mut pdf_font;
    pdf_init_font_struct(font);
    (*font).ident = new(
        (strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*font).ident, ident);
    (*font).font_id = font_id;
    (*font).subtype = (*src).subtype;
    (*font).encoding_id = (*src).encoding_id;
    (*font).flags |= PDF_FONT_FLAG_IS_ALIAS;
    font_cache.count += 1;
    return this_id;
}
unsafe extern "C-unwind" fn create_font_reencoded(
    mut ident: *const ::core::ffi::c_char,
    mut font_id: ::core::ffi::c_int,
    mut cmap_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut this_id: ::core::ffi::c_int = 0;
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    if font_cache.count + 1 as ::core::ffi::c_int >= font_cache.capacity {
        font_cache.capacity = (font_cache.capacity as ::core::ffi::c_uint)
            .wrapping_add(CACHE_ALLOC_SIZE) as ::core::ffi::c_int
            as ::core::ffi::c_int;
        font_cache.fonts = renew(
            font_cache.fonts as *mut ::core::ffi::c_void,
            (font_cache.capacity as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<pdf_font>() as usize) as uint32_t,
        ) as *mut pdf_font;
    }
    this_id = font_cache.count;
    font = font_cache.fonts.offset(this_id as isize) as *mut pdf_font;
    pdf_init_font_struct(font);
    (*font).ident = new(
        (strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*font).ident, ident);
    (*font).font_id = font_id;
    (*font).subtype = PDF_FONT_FONTTYPE_TYPE0;
    (*font).encoding_id = cmap_id;
    (*font).flags |= PDF_FONT_FLAG_IS_REENCODE;
    (*font).flags |= PDF_FONT_FLAG_USEDCHAR_SHARED;
    font_cache.count += 1;
    return this_id;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_load_font")]
pub unsafe extern "C-unwind" fn pdf_font_load_font(
    mut ident: *const ::core::ffi::c_char,
    mut font_scale: ::core::ffi::c_double,
    mut mrec: *const fontmap_rec,
) -> ::core::ffi::c_int {
    let mut font_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut font: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    let mut encoding_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut cmap_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut fontname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    fontname = if !mrec.is_null() {
        (*mrec).font_name as *const ::core::ffi::c_char
    } else {
        ident
    };
    if !mrec.is_null() && (*mrec).opt.use_glyph_encoding != 0 {
        let mut wmode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !(*mrec).enc_name.is_null() {
            if strcmp(
                (*mrec).enc_name,
                b"Identity-V\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                wmode = 1 as ::core::ffi::c_int;
            } else if strcmp(
                (*mrec).enc_name,
                b"Identity-H\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0
            {
                wmode = 0 as ::core::ffi::c_int;
            } else {
                pdf_warning!(
                    b"Unexpected encoding specified for xdv: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*mrec).enc_name,
                );
            }
            cmap_id = otf_try_load_GID_to_CID_map((*mrec).font_name, (*mrec).opt.index, wmode);
        }
    }
    if cmap_id < 0 as ::core::ffi::c_int && !mrec.is_null() && !(*mrec).enc_name.is_null() {
        if strcmp(
            (*mrec).enc_name,
            b"unicode\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            cmap_id = otf_load_Unicode_CMap(
                (*mrec).font_name,
                (*mrec).opt.index,
                (*mrec).opt.otl_tags,
                if (*mrec).opt.flags & FONTMAP_OPT_VERT != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                },
            );
            if cmap_id < 0 as ::core::ffi::c_int {
                cmap_id = t1_load_UnicodeCMap(
                    (*mrec).font_name,
                    (*mrec).opt.otl_tags,
                    if (*mrec).opt.flags & FONTMAP_OPT_VERT != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    },
                );
            }
        } else if strstr(
            (*mrec).enc_name,
            b".enc\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
            || !strstr(
                (*mrec).enc_name,
                b".cmap\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            cmap_id = CMap_cache_find((*mrec).enc_name);
        }
        if cmap_id < 0 as ::core::ffi::c_int {
            encoding_id = pdf_encoding_findresource((*mrec).enc_name);
        }
    }
    if !mrec.is_null() && !(*mrec).enc_name.is_null() {
        if cmap_id < 0 as ::core::ffi::c_int && encoding_id < 0 as ::core::ffi::c_int {
            pdf_warning!(
                b"Could not read encoding file: %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*mrec).enc_name,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    if !mrec.is_null() && cmap_id >= 0 as ::core::ffi::c_int {
        let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
        let mut csi: *mut CIDSysInfo = ::core::ptr::null_mut::<CIDSysInfo>();
        let mut wmode_0: ::core::ffi::c_int = 0;
        let mut cid_id: ::core::ffi::c_int = 0;
        cmap = CMap_cache_get(cmap_id);
        csi = if CMap_is_Identity(cmap) as ::core::ffi::c_int != 0 {
            ::core::ptr::null_mut::<CIDSysInfo>()
        } else {
            CMap_get_CIDSysInfo(cmap)
        };
        wmode_0 = CMap_get_wmode(cmap);
        cid_id = pdf_font_cidfont_lookup_cache(
            font_cache.fonts,
            font_cache.count,
            (*mrec).font_name,
            csi,
            &raw const (*mrec).opt,
        );
        if cid_id >= 0 as ::core::ffi::c_int {
            font_id = 0 as ::core::ffi::c_int;
            while font_id < font_cache.count {
                font = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
                if !((*font).subtype != PDF_FONT_FONTTYPE_TYPE0) {
                    if (*font).type0.wmode == wmode_0 && (*font).type0.descendant == cid_id {
                        break;
                    }
                }
                font_id += 1;
            }
            if font_id >= 0 as ::core::ffi::c_int && font_id < font_cache.count {
                font = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
                if (*font).encoding_id == cmap_id {
                    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                        pdf_message!(
                            b"\npdf_font>> Type0 font \"%s\" cmap_id=<%s,%d> found at font_id=<%s,%d>.\n\0"
                            as * const u8 as * const ::core::ffi::c_char, (* mrec)
                            .font_name, (* mrec).enc_name, cmap_id,
                            pdf_get_font_ident(font_id), font_id,
                        );
                    }
                    if strcmp(ident, (*font).ident) == 0 {
                        return font_id;
                    } else {
                        return create_font_alias(ident, font_id);
                    }
                } else {
                    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                        pdf_message!(
                            b"\npdf_font>> Type0 font \"%s\" cmap_id=<%s,%d> found at font_id=<%s,%d>. (re-encoded)\n\0"
                            as * const u8 as * const ::core::ffi::c_char, (* mrec)
                            .font_name, (* mrec).enc_name, cmap_id,
                            pdf_get_font_ident(font_id), font_id,
                        );
                    }
                    return create_font_reencoded(ident, font_id, cmap_id);
                }
            }
        }
        if font_cache.count + 1 as ::core::ffi::c_int >= font_cache.capacity {
            font_cache.capacity = (font_cache.capacity as ::core::ffi::c_uint)
                .wrapping_add(CACHE_ALLOC_SIZE)
                as ::core::ffi::c_int as ::core::ffi::c_int;
            font_cache.fonts = renew(
                font_cache.fonts as *mut ::core::ffi::c_void,
                (font_cache.capacity as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<pdf_font>() as usize)
                    as uint32_t,
            ) as *mut pdf_font;
        }
        if cid_id < 0 as ::core::ffi::c_int {
            let mut cidfont: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
            cid_id = font_cache.count;
            cidfont = font_cache.fonts.offset(cid_id as isize) as *mut pdf_font;
            pdf_init_font_struct(cidfont);
            if pdf_font_open_cidfont(cidfont, fontname, csi, &raw const (*mrec).opt)
                < 0 as ::core::ffi::c_int
            {
                pdf_clean_font_struct(cidfont);
                return -(1 as ::core::ffi::c_int);
            }
            font_cache.count += 1;
        }
        font_id = font_cache.count;
        font = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
        pdf_init_font_struct(font);
        if pdf_font_open_type0(font, cid_id, wmode_0) < 0 as ::core::ffi::c_int {
            pdf_clean_font_struct(font);
            return -(1 as ::core::ffi::c_int);
        }
        (*font).ident = new(
            (strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy((*font).ident, ident);
        (*font).subtype = PDF_FONT_FONTTYPE_TYPE0;
        (*font).encoding_id = cmap_id;
        font_cache.count += 1;
        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
            pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            pdf_message!(
                b"pdf_font>> Type0 font \"%s\" \0" as *const u8 as *const ::core::ffi::c_char,
                fontname,
            );
            pdf_message!(
                b"cmap_id=<%s,%d> \0" as *const u8 as *const ::core::ffi::c_char,
                (*mrec).enc_name,
                (*font).encoding_id,
            );
            pdf_message!(
                b"font_id=<%s,%d>.\0" as *const u8 as *const ::core::ffi::c_char,
                ident,
                font_id,
            );
            pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
    } else {
        font_id = 0 as ::core::ffi::c_int;
        while font_id < font_cache.count {
            font = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
            if !((*font).flags & PDF_FONT_FLAG_IS_ALIAS != 0) {
                match (*font).subtype {
                    PDF_FONT_FONTTYPE_TYPE1
                    | PDF_FONT_FONTTYPE_TYPE1C
                    | PDF_FONT_FONTTYPE_TRUETYPE => {
                        if strcmp(fontname, (*font).filename) == 0
                            && encoding_id == (*font).encoding_id
                            && (mrec.is_null() || (*mrec).opt.index == (*font).index)
                        {
                            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                                pdf_message!(
                                    b"\npdf_font>> Simple font \"%s\" (enc_id=%d) found at id=%d.\n\0"
                                    as * const u8 as * const ::core::ffi::c_char, fontname,
                                    encoding_id, font_id,
                                );
                            }
                            if strcmp(ident, (*font).ident) == 0 {
                                return font_id;
                            } else {
                                return create_font_alias(ident, font_id);
                            }
                        }
                    }
                    PDF_FONT_FONTTYPE_TYPE3 => {
                        if strcmp(fontname, (*font).filename) == 0
                            && font_scale == (*font).point_size
                        {
                            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                                pdf_message!(
                                    b"\npdf_font>> Simple font \"%s\" (enc_id=%d) found at id=%d.\n\0"
                                    as * const u8 as * const ::core::ffi::c_char, fontname,
                                    encoding_id, font_id,
                                );
                            }
                            if strcmp(ident, (*font).ident) == 0 {
                                return font_id;
                            } else {
                                return create_font_alias(ident, font_id);
                            }
                        }
                    }
                    _ => {}
                }
            }
            font_id += 1;
        }
        font_id = font_cache.count;
        if font_cache.count >= font_cache.capacity {
            font_cache.capacity = (font_cache.capacity as ::core::ffi::c_uint)
                .wrapping_add(CACHE_ALLOC_SIZE)
                as ::core::ffi::c_int as ::core::ffi::c_int;
            font_cache.fonts = renew(
                font_cache.fonts as *mut ::core::ffi::c_void,
                (font_cache.capacity as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<pdf_font>() as usize)
                    as uint32_t,
            ) as *mut pdf_font;
        }
        font = font_cache.fonts.offset(font_id as isize) as *mut pdf_font;
        pdf_init_font_struct(font);
        (*font).ident = new(
            (strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy((*font).ident, ident);
        (*font).encoding_id = encoding_id;
        (*font).filename = new(
            (strlen(fontname).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        (*font).point_size = font_scale;
        strcpy((*font).filename, fontname);
        (*font).index = if !mrec.is_null() && (*mrec).opt.index != 0 {
            (*mrec).opt.index
        } else {
            0 as uint32_t
        };
        (*font).flags |= if !mrec.is_null() && (*mrec).opt.flags & FONTMAP_OPT_NOEMBED != 0 {
            PDF_FONT_FLAG_NOEMBED
        } else {
            0 as ::core::ffi::c_int
        };
        if pdf_font_open_type1(
            font,
            (*font).filename,
            (*font).index as ::core::ffi::c_int,
            (*font).encoding_id,
            (if (*font).flags & PDF_FONT_FLAG_NOEMBED != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }),
        ) >= 0 as ::core::ffi::c_int
        {
            (*font).subtype = PDF_FONT_FONTTYPE_TYPE1;
        } else if pdf_font_open_type1c(
            font,
            (*font).filename,
            (*font).index as ::core::ffi::c_int,
            (*font).encoding_id,
            (if (*font).flags & PDF_FONT_FLAG_NOEMBED != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }),
        ) >= 0 as ::core::ffi::c_int
        {
            (*font).subtype = PDF_FONT_FONTTYPE_TYPE1C;
        } else if pdf_font_open_truetype(
            font,
            (*font).filename,
            (*font).index as ::core::ffi::c_int,
            (*font).encoding_id,
            (if (*font).flags & PDF_FONT_FLAG_NOEMBED != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }),
        ) >= 0 as ::core::ffi::c_int
        {
            (*font).subtype = PDF_FONT_FONTTYPE_TRUETYPE;
        } else if pdf_font_open_pkfont(
            font,
            (*font).filename,
            (*font).index as ::core::ffi::c_int,
            (*font).encoding_id,
            (if (*font).flags & PDF_FONT_FLAG_NOEMBED != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }),
            (*font).point_size,
        ) >= 0 as ::core::ffi::c_int
        {
            (*font).subtype = PDF_FONT_FONTTYPE_TYPE3;
        } else {
            pdf_clean_font_struct(font);
            return -(1 as ::core::ffi::c_int);
        }
        font_cache.count += 1;
        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
            pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            pdf_message!(
                b"pdf_font>> Simple font \"%s\" \0" as *const u8 as *const ::core::ffi::c_char,
                fontname,
            );
            pdf_message!(
                b"enc_id=<%s,%d> \0" as *const u8 as *const ::core::ffi::c_char,
                if !mrec.is_null() && !(*mrec).enc_name.is_null() {
                    (*mrec).enc_name as *const ::core::ffi::c_char
                } else {
                    b"builtin\0" as *const u8 as *const ::core::ffi::c_char
                },
                (*font).encoding_id,
            );
            pdf_message!(
                b"font_id=<%s,%d>.\0" as *const u8 as *const ::core::ffi::c_char,
                ident,
                font_id,
            );
            pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    return font_id;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_get_resource")]
pub unsafe extern "C-unwind" fn pdf_font_get_resource(mut font: *mut pdf_font) -> *mut pdf_obj {
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*font).resource.is_null() {
        (*font).resource = pdf_new_dict();
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"Font\0" as *const u8 as *const ::core::ffi::c_char),
        );
        match (*font).subtype {
            PDF_FONT_FONTTYPE_TYPE1 | PDF_FONT_FONTTYPE_TYPE1C => {
                pdf_add_dict(
                    (*font).resource,
                    pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_new_name(b"Type1\0" as *const u8 as *const ::core::ffi::c_char),
                );
            }
            PDF_FONT_FONTTYPE_TYPE3 => {
                pdf_add_dict(
                    (*font).resource,
                    pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_new_name(b"Type3\0" as *const u8 as *const ::core::ffi::c_char),
                );
            }
            PDF_FONT_FONTTYPE_TRUETYPE => {
                pdf_add_dict(
                    (*font).resource,
                    pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
                    pdf_new_name(b"TrueType\0" as *const u8 as *const ::core::ffi::c_char),
                );
            }
            _ => {}
        }
    }
    return (*font).resource;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_get_descriptor")]
pub unsafe extern "C-unwind" fn pdf_font_get_descriptor(mut font: *mut pdf_font) -> *mut pdf_obj {
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*font).subtype == PDF_FONT_FONTTYPE_TYPE0 {
        return ::core::ptr::null_mut::<pdf_obj>();
    } else if (*font).descriptor.is_null() {
        (*font).descriptor = pdf_new_dict();
        pdf_add_dict(
            (*font).descriptor,
            pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"FontDescriptor\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    return (*font).descriptor;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_get_uniqueTag")]
pub unsafe extern "C-unwind" fn pdf_font_get_uniqueTag(
    mut font: *mut pdf_font,
) -> *mut ::core::ffi::c_char {
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*font).uniqueID[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '\0' as i32 {
        pdf_font_make_uniqueTag(&raw mut (*font).uniqueID as *mut ::core::ffi::c_char);
    }
    return &raw mut (*font).uniqueID as *mut ::core::ffi::c_char;
}
#[unsafe(export_name = "__pdf_impl_pdf_check_tfm_widths")]
pub unsafe extern "C-unwind" fn pdf_check_tfm_widths(
    mut ident: *const ::core::ffi::c_char,
    mut widths: *mut ::core::ffi::c_double,
    mut firstchar: ::core::ffi::c_int,
    mut lastchar: ::core::ffi::c_int,
    mut usedchars: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tfm_id: ::core::ffi::c_int = 0;
    let mut code: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sum: ::core::ffi::c_double = 0.0f64;
    let mut tolerance: ::core::ffi::c_double = 1.0f64;
    let mut override_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    tfm_id = tfm_open(ident, 0 as ::core::ffi::c_int);
    if tfm_id < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    code = firstchar;
    while code <= lastchar {
        if *usedchars.offset(code as isize) != 0 {
            let mut width: ::core::ffi::c_double = 0.;
            let mut diff: ::core::ffi::c_double = 0.;
            width = 1000.0f64 * tfm_get_width(tfm_id, code as int32_t);
            diff = *widths.offset(code as isize) - width;
            diff = if diff < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                -diff
            } else {
                diff
            };
            if override_0 != 0 {
                *widths.offset(code as isize) = width;
            } else if diff > tolerance {
                if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                    pdf_warning!(
                        b"Intolerable difference in glyph width: font=%s, char=%d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        ident,
                        code,
                    );
                    pdf_warning!(
                        b"font: %g vs. tfm: %g\0" as *const u8 as *const ::core::ffi::c_char,
                        *widths.offset(code as isize),
                        width,
                    );
                }
                sum += diff;
            }
            count += 1;
        }
        code += 1;
    }
    error = if sum > 0.5f64 * count as ::core::ffi::c_double * tolerance {
        -(1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
    return if override_0 != 0 {
        0 as ::core::ffi::c_int
    } else {
        error
    };
}
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_set_dpi(mut font_dpi: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_set_dpi(font_dpi) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_reset_unique_tag_state() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_reset_unique_tag_state() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_set_deterministic_unique_tags(
    mut value: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_font_set_deterministic_unique_tags(value)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_make_uniqueTag(
    mut tag: *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_make_uniqueTag(tag) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_init_fonts() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_init_fonts() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_abort_fonts() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_abort_fonts() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_get_font_data(
    mut font_id: ::core::ffi::c_int,
    _result: *mut *mut pdf_font,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_font_data(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_font_ident(
    mut font_id: ::core::ffi::c_int,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_font_ident(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_font_reference(
    mut font_id: ::core::ffi::c_int,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_font_reference(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_font_resource(
    mut font_id: ::core::ffi::c_int,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_font_resource(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_font_usedchars(
    mut font_id: ::core::ffi::c_int,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_font_usedchars(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_font_wmode(
    mut font_id: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_font_wmode(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_font_subtype(
    mut font_id: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_font_subtype(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_font_encoding(
    mut font_id: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_font_encoding(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_font_resource_name(
    mut font_id: ::core::ffi::c_int,
    mut buff: *mut ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_resource_name(font_id, buff) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_close_fonts() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_close_fonts() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_findresource(
    mut ident: *const ::core::ffi::c_char,
    mut scale: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_findresource(ident, scale) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_font_load_font(
    mut ident: *const ::core::ffi::c_char,
    mut font_scale: ::core::ffi::c_double,
    mut mrec: *const fontmap_rec,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_font_load_font(ident, font_scale, mrec)
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
pub unsafe extern "C" fn __pdf_entry_pdf_font_get_resource(
    mut font: *mut pdf_font,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_get_resource(font) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_font_get_descriptor(
    mut font: *mut pdf_font,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_get_descriptor(font) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_font_get_uniqueTag(
    mut font: *mut pdf_font,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_get_uniqueTag(font) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_check_tfm_widths(
    mut ident: *const ::core::ffi::c_char,
    mut widths: *mut ::core::ffi::c_double,
    mut firstchar: ::core::ffi::c_int,
    mut lastchar: ::core::ffi::c_int,
    mut usedchars: *const ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_check_tfm_widths(ident, widths, firstchar, lastchar, usedchars)
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
