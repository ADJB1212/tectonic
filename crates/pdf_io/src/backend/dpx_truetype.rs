/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2007-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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
/*
 * The 'name' table should be preserved since it contains copyright
 * information, but it might cause problem when there are invalid
 * table entries (wrongly encoded text which is often the case in
 * CJK fonts). Acrobat does not use 'name' table. Unicode TrueType
 * fonts may have 10K bytes 'name' table...
 *
 * We preserve the 'OS/2' table too, since it contains the license
 * information. PDF applications should use this table to decide
 * whether the font is embedded only for the purpose of preview &
 * printing. Otherwise, we must encrypt the document. Acrobat does
 * not use 'OS/2' table, though...
 */
#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn, static_mut_refs, dangerous_implicit_autorefs)]
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcpy(__dst: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn sprintf(_: *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ref_obj"]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_array_length"]
    fn pdf_array_length(array: *mut pdf_obj) -> ::core::ffi::c_uint;
    #[link_name = "__pdf_impl_pdf_merge_dict"]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_stream_length"]
    fn pdf_stream_length(stream: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_get_resource"]
    fn pdf_font_get_resource(font: *mut pdf_font) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_font_get_descriptor"]
    fn pdf_font_get_descriptor(font: *mut pdf_font) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_check_tfm_widths"]
    fn pdf_check_tfm_widths(
        ident: *const ::core::ffi::c_char, widths: *mut ::core::ffi::c_double, firstchar: ::core::ffi::c_int, lastchar: ::core::ffi::c_int,
        usedchars: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_agl_chop_suffix"]
    fn agl_chop_suffix(glyphname: *const ::core::ffi::c_char, suffix: *mut *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_agl_name_is_unicode"]
    fn agl_name_is_unicode(glyphname: *const ::core::ffi::c_char) -> bool;
    #[link_name = "__pdf_impl_agl_name_convert_unicode"]
    fn agl_name_convert_unicode(glyphname: *const ::core::ffi::c_char) -> int32_t;
    #[link_name = "__pdf_impl_agl_suffix_to_otltag"]
    fn agl_suffix_to_otltag(suffix: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    #[link_name = "__pdf_impl_agl_lookup_list"]
    fn agl_lookup_list(glyphname: *const ::core::ffi::c_char) -> *mut agl_name;
    static mut dpx_conf: _dpx_conf;
    #[link_name = "__pdf_impl_dpx_open_truetype_file"]
    fn dpx_open_truetype_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    #[link_name = "__pdf_impl_dpx_open_dfont_file"]
    fn dpx_open_dfont_file(filename: *const ::core::ffi::c_char) -> rust_input_handle_t;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_encoding_get_encoding"]
    fn pdf_encoding_get_encoding(enc_id: ::core::ffi::c_int) -> *mut *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_put_big_endian"]
    fn put_big_endian(s: *mut ::core::ffi::c_void, q: SFNT_LONG, n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_sfnt_open"]
    fn sfnt_open(handle: rust_input_handle_t) -> *mut sfnt;
    #[link_name = "__pdf_impl_dfont_open"]
    fn dfont_open(handle: rust_input_handle_t, index: ::core::ffi::c_int) -> *mut sfnt;
    #[link_name = "__pdf_impl_sfnt_close"]
    fn sfnt_close(sfont: *mut sfnt);
    #[link_name = "__pdf_impl_sfnt_read_table_directory"]
    fn sfnt_read_table_directory(sfont: *mut sfnt, offset: SFNT_ULONG) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_sfnt_set_table"]
    fn sfnt_set_table(sfont: *mut sfnt, tag: *const ::core::ffi::c_char, data: *mut ::core::ffi::c_void, length: SFNT_ULONG);
    #[link_name = "__pdf_impl_sfnt_require_table"]
    fn sfnt_require_table(sfont: *mut sfnt, tag: *const ::core::ffi::c_char, must_exist: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_sfnt_create_FontFile_stream"]
    fn sfnt_create_FontFile_stream(sfont: *mut sfnt) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_ttc_read_offset"]
    fn ttc_read_offset(sfont: *mut sfnt, ttc_idx: SFNT_ULONG) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_tt_get_fontdesc"]
    fn tt_get_fontdesc(
        sfont: *mut sfnt, embed: *mut ::core::ffi::c_int, stemv: ::core::ffi::c_int, type_0: ::core::ffi::c_int, fontname: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_tt_cmap_read"]
    fn tt_cmap_read(sfont: *mut sfnt, platform: USHORT, encoding: USHORT) -> *mut tt_cmap;
    #[link_name = "__pdf_impl_tt_cmap_lookup"]
    fn tt_cmap_lookup(cmap: *mut tt_cmap, cc: SFNT_ULONG) -> USHORT;
    #[link_name = "__pdf_impl_tt_cmap_release"]
    fn tt_cmap_release(cmap: *mut tt_cmap);
    #[link_name = "__pdf_impl_tt_build_init"]
    fn tt_build_init() -> *mut tt_glyphs;
    #[link_name = "__pdf_impl_tt_build_finish"]
    fn tt_build_finish(g: *mut tt_glyphs);
    #[link_name = "__pdf_impl_tt_add_glyph"]
    fn tt_add_glyph(g: *mut tt_glyphs, gid: USHORT, new_gid: USHORT) -> USHORT;
    #[link_name = "__pdf_impl_tt_get_index"]
    fn tt_get_index(g: *mut tt_glyphs, gid: USHORT) -> USHORT;
    #[link_name = "__pdf_impl_tt_find_glyph"]
    fn tt_find_glyph(g: *mut tt_glyphs, gid: USHORT) -> USHORT;
    #[link_name = "__pdf_impl_tt_build_tables"]
    fn tt_build_tables(sfont: *mut sfnt, g: *mut tt_glyphs) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_new"]
    fn otl_gsub_new() -> *mut otl_gsub;
    #[link_name = "__pdf_impl_otl_gsub_release"]
    fn otl_gsub_release(gsub_list: *mut otl_gsub);
    #[link_name = "__pdf_impl_otl_gsub_select"]
    fn otl_gsub_select(
        gsub_list: *mut otl_gsub, script: *const ::core::ffi::c_char, language: *const ::core::ffi::c_char, feature: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_add_feat"]
    fn otl_gsub_add_feat(
        gsub_list: *mut otl_gsub, script: *const ::core::ffi::c_char, language: *const ::core::ffi::c_char, feature: *const ::core::ffi::c_char,
        sfont: *mut sfnt,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_apply"]
    fn otl_gsub_apply(gsub_list: *mut otl_gsub, gid: *mut USHORT) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_apply_alt"]
    fn otl_gsub_apply_alt(gsub_list: *mut otl_gsub, alt_idx: USHORT, gid: *mut USHORT) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_gsub_apply_lig"]
    fn otl_gsub_apply_lig(gsub_list: *mut otl_gsub, gid_in: *mut USHORT, num_gids: USHORT, gid_out: *mut USHORT) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tt_read_post_table"]
    fn tt_read_post_table(sfont: *mut sfnt) -> *mut tt_post_table;
    #[link_name = "__pdf_impl_tt_release_post_table"]
    fn tt_release_post_table(post: *mut tt_post_table);
    #[link_name = "__pdf_impl_tt_lookup_post_table"]
    fn tt_lookup_post_table(post: *mut tt_post_table, glyphname: *const ::core::ffi::c_char) -> USHORT;
    #[link_name = "__pdf_impl_tt_get_ps_fontname"]
    fn tt_get_ps_fontname(sfont: *mut sfnt, dest: *mut ::core::ffi::c_char, destlen: USHORT) -> USHORT;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type rust_input_handle_t = uintptr_t;
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
pub struct sfnt {
    pub type_0: ::core::ffi::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
pub type SFNT_ULONG = uint32_t;
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
pub struct sfnt_table {
    pub tag: [::core::ffi::c_char; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut ::core::ffi::c_char,
}
pub type USHORT = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub keep_cache: ::core::ffi::c_int,
}
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub name: *const ::core::ffi::c_char,
    pub must_exist: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_glyphs {
    pub num_glyphs: USHORT,
    pub max_glyphs: USHORT,
    pub last_gid: USHORT,
    pub emsize: USHORT,
    pub dw: USHORT,
    pub default_advh: USHORT,
    pub default_tsb: SHORT,
    pub gd: *mut tt_glyph_desc,
    pub used_slot: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_glyph_desc {
    pub gid: USHORT,
    pub ogid: USHORT,
    pub advw: USHORT,
    pub advh: USHORT,
    pub lsb: SHORT,
    pub tsb: SHORT,
    pub llx: SHORT,
    pub lly: SHORT,
    pub urx: SHORT,
    pub ury: SHORT,
    pub length: SFNT_ULONG,
    pub data: *mut BYTE,
}
pub type BYTE = ::core::ffi::c_uchar;
pub type SHORT = ::core::ffi::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glyph_mapper {
    pub codetogid: *mut tt_cmap,
    pub gsub: *mut otl_gsub,
    pub sfont: *mut sfnt,
    pub nametogid: *mut tt_post_table,
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
pub type FWord = ::core::ffi::c_short;
pub type Fixed = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_cmap {
    pub format: USHORT,
    pub platform: USHORT,
    pub encoding: USHORT,
    pub language: SFNT_ULONG,
    pub map: *mut ::core::ffi::c_void,
}
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
pub type SFNT_LONG = int32_t;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
#[inline]
unsafe extern "C-unwind" fn streq_ptr(mut s1: *const ::core::ffi::c_char, mut s2: *const ::core::ffi::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
    }
    return false_0 != 0;
}
pub const PDF_FONT_FONTTYPE_TRUETYPE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SFNT_TYPE_TRUETYPE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const SFNT_TYPE_TTC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const SFNT_TYPE_DFONT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const TT_MAC: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TT_WIN: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TT_WIN_UNICODE: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
pub const TT_WIN_UCS4: ::core::ffi::c_uint = 10 as ::core::ffi::c_uint;
pub const TT_MAC_ROMAN: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
#[unsafe(export_name = "__pdf_impl_pdf_font_open_truetype")]
pub unsafe extern "C-unwind" fn pdf_font_open_truetype(
    mut font: *mut pdf_font, mut ident: *const ::core::ffi::c_char, mut index: ::core::ffi::c_int, mut encoding_id: ::core::ffi::c_int,
    mut embedding: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fontdict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut descriptor: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    let mut length: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"font\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if ident.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"ident\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    handle = dpx_open_truetype_file(ident);
    if handle == 0 {
        handle = dpx_open_dfont_file(ident);
        if handle == 0 {
            return -(1 as ::core::ffi::c_int);
        }
        sfont = dfont_open(handle, index);
    } else {
        sfont = sfnt_open(handle);
    }
    if sfont.is_null() {
        pdf_warning!(b"Could not open TrueType font: %s\0" as *const u8 as *const ::core::ffi::c_char, ident,);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if (*sfont).type_0 == SFNT_TYPE_TTC {
        let mut offset: SFNT_ULONG = 0;
        offset = ttc_read_offset(sfont, index as SFNT_ULONG);
        if offset == 0 as SFNT_ULONG {
            pdf_warning!(b"Invalid TTC index in %s.\0" as *const u8 as *const ::core::ffi::c_char, ident,);
            error = -(1 as ::core::ffi::c_int);
        } else {
            error = sfnt_read_table_directory(sfont, offset);
        }
    } else {
        error = sfnt_read_table_directory(sfont, (*sfont).offset);
    }
    if error != 0 {
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    fontdict = pdf_font_get_resource(font);
    descriptor = pdf_font_get_descriptor(font);
    if !(!fontdict.is_null() && !descriptor.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"fontdict && descriptor\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    let mut fontname: [::core::ffi::c_char; 256] = [0; 256];
    let mut n: ::core::ffi::c_int = 0;
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    memset(
        &raw mut fontname as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        256 as size_t,
    );
    length = tt_get_ps_fontname(sfont, &raw mut fontname as *mut ::core::ffi::c_char, 255 as USHORT) as ::core::ffi::c_int;
    if length < 1 as ::core::ffi::c_int {
        length = (if strlen(ident) < 255 as size_t { strlen(ident) } else { 255 as size_t }) as ::core::ffi::c_int;
        memcpy(
            &raw mut fontname as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ident as *const ::core::ffi::c_void,
            length as size_t,
        );
    }
    fontname[length as usize] = '\0' as i32 as ::core::ffi::c_char;
    n = 0 as ::core::ffi::c_int;
    while n < length {
        if fontname[n as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            memmove(
                (&raw mut fontname as *mut ::core::ffi::c_char).offset(n as isize) as *mut ::core::ffi::c_void,
                (&raw mut fontname as *mut ::core::ffi::c_char)
                    .offset(n as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                (length - n - 1 as ::core::ffi::c_int) as size_t,
            );
        }
        n += 1;
    }
    if strlen(&raw mut fontname as *mut ::core::ffi::c_char) == 0 as size_t {
        pdf_warning!(
            b"Can't find valid fontname for \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
        error = -(1 as ::core::ffi::c_int);
    } else {
        (*font).fontname = new(
            (strlen(&raw mut fontname as *mut ::core::ffi::c_char).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy((*font).fontname, &raw mut fontname as *mut ::core::ffi::c_char);
        tmp = tt_get_fontdesc(
            sfont,
            &raw mut embedding,
            -(1 as ::core::ffi::c_int),
            1 as ::core::ffi::c_int,
            &raw mut fontname as *mut ::core::ffi::c_char,
        );
        if tmp.is_null() {
            pdf_warning!(
                b"Could not obtain necessary font info: %s\0" as *const u8 as *const ::core::ffi::c_char,
                ident,
            );
            error = -(1 as ::core::ffi::c_int);
        } else {
            if !(pdf_obj_typeof(tmp) == 6 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                pdf_abort!(
                    c"assertion failed: %s".as_ptr(),
                    b"pdf_obj_typeof(tmp) == PDF_DICT\0" as *const u8 as *const ::core::ffi::c_char
                );
            } else {
            };
            pdf_merge_dict(descriptor, tmp);
            pdf_release_obj(tmp);
        }
    }
    if error != 0 {
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if embedding == 0 {
        pdf_warning!(
            b"Font file=\"%s\" can't be embedded due to liscence restrictions.\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
        error = -(1 as ::core::ffi::c_int);
    }
    sfnt_close(sfont);
    crate::backend::runtime::input_close(handle);
    if error == 0 {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"Font\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"TrueType\0" as *const u8 as *const ::core::ffi::c_char),
        );
        (*font).subtype = PDF_FONT_FONTTYPE_TRUETYPE;
    }
    return error;
}
const REQUIRED_TABLE: [C2RustUnnamed_2; 13] = [
    C2RustUnnamed_2 {
        name: b"OS/2\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"head\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"loca\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"name\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"hmtx\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"cvt \0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"prep\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 0 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
        must_exist: 1 as ::core::ffi::c_int,
    },
    C2RustUnnamed_2 {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        must_exist: 0 as ::core::ffi::c_int,
    },
];
unsafe extern "C-unwind" fn do_widths(mut font: *mut pdf_font, mut widths: *mut ::core::ffi::c_double) {
    let mut fontdict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut code: ::core::ffi::c_int = 0;
    let mut firstchar: ::core::ffi::c_int = 0;
    let mut lastchar: ::core::ffi::c_int = 0;
    let mut usedchars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    fontdict = (*font).resource;
    usedchars = (*font).usedchars;
    firstchar = 255 as ::core::ffi::c_int;
    lastchar = 0 as ::core::ffi::c_int;
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        if *usedchars.offset(code as isize) != 0 {
            if code < firstchar {
                firstchar = code;
            }
            if code > lastchar {
                lastchar = code;
            }
        }
        code += 1;
    }
    if firstchar > lastchar {
        pdf_warning!(b"No glyphs actually used???\0" as *const u8 as *const ::core::ffi::c_char,);
        return;
    }
    pdf_check_tfm_widths((*font).ident, widths, firstchar, lastchar, usedchars);
    array = pdf_new_array();
    code = firstchar;
    while code <= lastchar {
        if *usedchars.offset(code as isize) != 0 {
            pdf_add_array(array, pdf_new_number(floor(*widths.offset(code as isize) / 0.1f64 + 0.5f64) * 0.1f64));
        } else {
            pdf_add_array(array, pdf_new_number(0.0f64));
        }
        code += 1;
    }
    if pdf_array_length(array) > 0 as ::core::ffi::c_uint {
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"Widths\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(array),
        );
    }
    pdf_release_obj(array);
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"FirstChar\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(firstchar as ::core::ffi::c_double),
    );
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"LastChar\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(lastchar as ::core::ffi::c_double),
    );
}
unsafe extern "C-unwind" fn do_builtin_encoding(
    mut font: *mut pdf_font, mut usedchars: *const ::core::ffi::c_char, mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut glyphs: *mut tt_glyphs = ::core::ptr::null_mut::<tt_glyphs>();
    let mut cmap_table: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ttcm: *mut tt_cmap = ::core::ptr::null_mut::<tt_cmap>();
    let mut gid: USHORT = 0;
    let mut idx: USHORT = 0;
    let mut code: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut widths: [::core::ffi::c_double; 256] = [0.; 256];
    ttcm = tt_cmap_read(sfont, TT_MAC as USHORT, TT_MAC_ROMAN as USHORT);
    if ttcm.is_null() {
        pdf_warning!(b"Could not read Mac-Roman TrueType cmap table...\0" as *const u8 as *const ::core::ffi::c_char,);
        return -(1 as ::core::ffi::c_int);
    }
    cmap_table =
        new((274 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
            as *mut ::core::ffi::c_char;
    memset(cmap_table as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, 274 as size_t);
    put_big_endian(cmap_table as *mut ::core::ffi::c_void, 0 as SFNT_LONG, 2 as ::core::ffi::c_int);
    put_big_endian(
        cmap_table.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        1 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(4 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        1 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(6 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        0 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        12 as SFNT_LONG,
        4 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(12 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        0 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(14 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        262 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(16 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        0 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    glyphs = tt_build_init();
    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
        pdf_message!(b"[glyphs:/.notdef\0" as *const u8 as *const ::core::ffi::c_char);
    }
    count = 1 as ::core::ffi::c_int;
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        if !(*usedchars.offset(code as isize) == 0) {
            if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
                pdf_message!(b"/.c0x%02x\0" as *const u8 as *const ::core::ffi::c_char, code,);
            }
            gid = tt_cmap_lookup(ttcm, code as SFNT_ULONG);
            if gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Glyph for character code=0x%02x missing in font font-file=\"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                    code,
                    (*font).filename,
                );
                idx = 0 as USHORT;
            } else {
                idx = tt_find_glyph(glyphs, gid);
                if idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    idx = tt_add_glyph(glyphs, gid, count as USHORT);
                }
            }
            *cmap_table.offset((18 as ::core::ffi::c_int + code) as isize) =
                (idx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
            count += 1;
        }
        code += 1;
    }
    tt_cmap_release(ttcm);
    if dpx_conf.verbose_level > 2 as ::core::ffi::c_int {
        pdf_message!(b"]\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if tt_build_tables(sfont, glyphs) < 0 as ::core::ffi::c_int {
        pdf_warning!(b"Packing TrueType font into SFNT failed!\0" as *const u8 as *const ::core::ffi::c_char,);
        tt_build_finish(glyphs);
        free(cmap_table as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        if *usedchars.offset(code as isize) != 0 {
            idx = tt_get_index(glyphs, *cmap_table.offset((18 as ::core::ffi::c_int + code) as isize) as USHORT);
            widths[code as usize] = floor(
                1000.0f64 * (*(*glyphs).gd.offset(idx as isize)).advw as ::core::ffi::c_int as ::core::ffi::c_double
                    / (*glyphs).emsize as ::core::ffi::c_int as ::core::ffi::c_double
                    / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    + 0.5f64,
            ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            widths[code as usize] = 0.0f64;
        }
        code += 1;
    }
    do_widths(font, &raw mut widths as *mut ::core::ffi::c_double);
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        pdf_message!(
            b"[%d glyphs]\0" as *const u8 as *const ::core::ffi::c_char,
            (*glyphs).num_glyphs as ::core::ffi::c_int,
        );
    }
    tt_build_finish(glyphs);
    sfnt_set_table(
        sfont,
        b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
        cmap_table as *mut ::core::ffi::c_void,
        274 as SFNT_ULONG,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn agl_decompose_glyphname(
    mut glyphname: *mut ::core::ffi::c_char, mut nptrs: *mut *mut ::core::ffi::c_char, mut size: ::core::ffi::c_int,
    mut suffix: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = glyphname;
    let mut n: ::core::ffi::c_int = 0;
    q = strchr(p, '.' as i32);
    if q.is_null() {
        *suffix = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        *q = '\0' as i32 as ::core::ffi::c_char;
        q = q.offset(1);
        *suffix = q;
    }
    let ref mut fresh3 = *nptrs.offset(0 as ::core::ffi::c_int as isize);
    *fresh3 = p;
    n = 1 as ::core::ffi::c_int;
    while !p.is_null() && *p as ::core::ffi::c_int != 0 {
        p = strchr(p, '_' as i32);
        if p.is_null() || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
            break;
        }
        if n >= size {
            pdf_abort!(b"Uh ah...\0" as *const u8 as *const ::core::ffi::c_char);
        }
        *p = '\0' as i32 as ::core::ffi::c_char;
        p = p.offset(1);
        let ref mut fresh4 = *nptrs.offset(n as isize);
        *fresh4 = p;
        n += 1;
    }
    return n;
}
unsafe extern "C-unwind" fn select_gsub(mut feat: *const ::core::ffi::c_char, mut gm: *mut glyph_mapper) -> ::core::ffi::c_int {
    let mut idx: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if feat.is_null() || *feat as ::core::ffi::c_int == 0 as ::core::ffi::c_int || gm.is_null() || (*gm).gsub.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    idx = otl_gsub_select(
        (*gm).gsub,
        b"*\0" as *const u8 as *const ::core::ffi::c_char,
        b"*\0" as *const u8 as *const ::core::ffi::c_char,
        feat,
    );
    if idx >= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        pdf_message!(
            b"\ntrutype>> Try loading OTL GSUB for \"*.*.%s\"...\0" as *const u8 as *const ::core::ffi::c_char,
            feat,
        );
    }
    error = otl_gsub_add_feat(
        (*gm).gsub,
        b"*\0" as *const u8 as *const ::core::ffi::c_char,
        b"*\0" as *const u8 as *const ::core::ffi::c_char,
        feat,
        (*gm).sfont,
    );
    if error == 0 {
        idx = otl_gsub_select(
            (*gm).gsub,
            b"*\0" as *const u8 as *const ::core::ffi::c_char,
            b"*\0" as *const u8 as *const ::core::ffi::c_char,
            feat,
        );
        return if idx >= 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        };
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn selectglyph(
    mut in_0: USHORT, mut suffix: *const ::core::ffi::c_char, mut gm: *mut glyph_mapper, mut out: *mut USHORT,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut t: [::core::ffi::c_char; 5] = [0; 5];
    let mut r: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(!suffix.is_null() && !gm.is_null() && !out.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"suffix && gm && out\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(!suffix.is_null() && *suffix as ::core::ffi::c_int != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"suffix && *suffix != 0\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    s = new(
        (strlen(suffix).wrapping_add(1 as size_t) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy(s, suffix);
    r = agl_suffix_to_otltag(s);
    if !r.is_null() {
        error = select_gsub(r, gm);
        if error == 0 {
            error = otl_gsub_apply((*gm).gsub, &raw mut in_0);
        }
    } else {
        if strlen(s) > 4 as size_t {
            error = -(1 as ::core::ffi::c_int);
        } else if strlen(s) == 4 as size_t {
            error = select_gsub(s, gm);
        } else {
            memset(
                &raw mut t as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ' ' as i32,
                4 as size_t,
            );
            t[4 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
            memcpy(
                &raw mut t as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                s as *const ::core::ffi::c_void,
                strlen(s),
            );
            error = select_gsub(&raw mut t as *mut ::core::ffi::c_char, gm);
        }
        if error == 0 {
            error = otl_gsub_apply((*gm).gsub, &raw mut in_0);
        } else {
            q = s.offset(strlen(s) as isize).offset(-(1 as ::core::ffi::c_int as isize));
            while q > s && *q as ::core::ffi::c_int >= '0' as i32 && *q as ::core::ffi::c_int <= '9' as i32 {
                q = q.offset(-1);
            }
            if q == s {
                error = -(1 as ::core::ffi::c_int);
            } else {
                n = atoi(q.offset(1 as ::core::ffi::c_int as isize)) - 1 as ::core::ffi::c_int;
                *q.offset(1 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
                if strlen(s) > 4 as size_t {
                    error = -(1 as ::core::ffi::c_int);
                } else {
                    memset(
                        &raw mut t as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        ' ' as i32,
                        4 as size_t,
                    );
                    t[4 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
                    memcpy(
                        &raw mut t as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        s as *const ::core::ffi::c_void,
                        strlen(s),
                    );
                    error = select_gsub(s, gm);
                    if error == 0 {
                        error = otl_gsub_apply_alt((*gm).gsub, n as USHORT, &raw mut in_0);
                    }
                }
            }
        }
    }
    free(s as *mut ::core::ffi::c_void);
    *out = in_0;
    return error;
}
unsafe extern "C-unwind" fn composeglyph(
    mut glyphs: *mut USHORT, mut n_glyphs: ::core::ffi::c_int, mut feat: *const ::core::ffi::c_char, mut gm: *mut glyph_mapper, mut gid: *mut USHORT,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut t: [::core::ffi::c_char; 5] = [
        ' ' as i32 as ::core::ffi::c_char,
        ' ' as i32 as ::core::ffi::c_char,
        ' ' as i32 as ::core::ffi::c_char,
        ' ' as i32 as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
    ];
    if !(!glyphs.is_null() && n_glyphs > 0 as ::core::ffi::c_int && !gm.is_null() && !gid.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"glyphs && n_glyphs > 0 && gm && gid\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if feat.is_null() || *feat.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\0' as i32 {
        error = select_gsub(b"(?lig|lig?|?cmp|cmp?|frac|afrc)\0" as *const u8 as *const ::core::ffi::c_char, gm);
    } else if strlen(feat) > 4 as size_t {
        error = -(1 as ::core::ffi::c_int);
    } else {
        memcpy(
            &raw mut t as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            feat as *const ::core::ffi::c_void,
            strlen(feat),
        );
        error = select_gsub(&raw mut t as *mut ::core::ffi::c_char, gm);
    }
    if error == 0 {
        error = otl_gsub_apply_lig((*gm).gsub, glyphs, n_glyphs as USHORT, gid);
    }
    return error;
}
unsafe extern "C-unwind" fn composeuchar(
    mut unicodes: *mut int32_t, mut n_unicodes: ::core::ffi::c_int, mut feat: *const ::core::ffi::c_char, mut gm: *mut glyph_mapper,
    mut gid: *mut USHORT,
) -> ::core::ffi::c_int {
    let mut gids: *mut USHORT = ::core::ptr::null_mut::<USHORT>();
    let mut i: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*gm).codetogid.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    gids = new((n_unicodes as uint32_t as usize).wrapping_mul(::core::mem::size_of::<USHORT>() as usize) as uint32_t) as *mut USHORT;
    i = 0 as ::core::ffi::c_int;
    while error == 0 && i < n_unicodes {
        *gids.offset(i as isize) = tt_cmap_lookup((*gm).codetogid, *unicodes.offset(i as isize) as SFNT_ULONG);
        error = if *gids.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            -(1 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_int
        };
        i += 1;
    }
    if error == 0 {
        error = composeglyph(gids, n_unicodes, feat, gm, gid);
    }
    free(gids as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C-unwind" fn findposttable(
    mut glyph_name: *const ::core::ffi::c_char, mut gid: *mut USHORT, mut gm: *mut glyph_mapper,
) -> ::core::ffi::c_int {
    if (*gm).nametogid.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    *gid = tt_lookup_post_table((*gm).nametogid, glyph_name);
    return if *gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C-unwind" fn findcomposite(
    mut glyphname: *const ::core::ffi::c_char, mut gid: *mut USHORT, mut gm: *mut glyph_mapper,
) -> ::core::ffi::c_int {
    let mut gname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut suffix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gids: [USHORT; 32] = [0; 32];
    let mut nptrs: [*mut ::core::ffi::c_char; 32] = [::core::ptr::null_mut::<::core::ffi::c_char>(); 32];
    let mut i: ::core::ffi::c_int = 0;
    let mut n_comp: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    error = findposttable(glyphname, gid, gm);
    if error == 0 {
        return 0 as ::core::ffi::c_int;
    }
    gname = new((strlen(glyphname).wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    strcpy(gname, glyphname);
    memset(
        &raw mut gids as *mut USHORT as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (32 as size_t).wrapping_mul(::core::mem::size_of::<USHORT>() as size_t),
    );
    n_comp = agl_decompose_glyphname(
        gname,
        &raw mut nptrs as *mut *mut ::core::ffi::c_char,
        32 as ::core::ffi::c_int,
        &raw mut suffix,
    );
    error = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while error == 0 && i < n_comp {
        error = resolve_glyph(nptrs[i as usize], (&raw mut gids as *mut USHORT).offset(i as isize) as *mut USHORT, gm);
        if error != 0 {
            pdf_warning!(
                b"Could not resolve glyph \"%s\" (%dth component of glyph \"%s\").\0" as *const u8 as *const ::core::ffi::c_char,
                nptrs[i as usize],
                i,
                glyphname,
            );
        }
        i += 1;
    }
    if error == 0 {
        if !suffix.is_null()
            && (streq_ptr(suffix, b"liga\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
                || streq_ptr(suffix, b"dlig\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
                || streq_ptr(suffix, b"hlig\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
                || streq_ptr(suffix, b"frac\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
                || streq_ptr(suffix, b"ccmp\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
                || streq_ptr(suffix, b"afrc\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0)
        {
            error = composeglyph(&raw mut gids as *mut USHORT, n_comp, suffix, gm, gid);
        } else {
            error = composeglyph(&raw mut gids as *mut USHORT, n_comp, ::core::ptr::null::<::core::ffi::c_char>(), gm, gid);
            if error == 0 && !suffix.is_null() {
                error = selectglyph(*gid, suffix, gm, gid);
            }
        }
    }
    free(gname as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C-unwind" fn findparanoiac(
    mut glyphname: *const ::core::ffi::c_char, mut gid: *mut USHORT, mut gm: *mut glyph_mapper,
) -> ::core::ffi::c_int {
    let mut agln: *mut agl_name = ::core::ptr::null_mut::<agl_name>();
    let mut idx: USHORT = 0 as USHORT;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    agln = agl_lookup_list(glyphname);
    while !agln.is_null() && idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if !(*agln).suffix.is_null() {
            error = findparanoiac((*agln).name, &raw mut idx, gm);
            if error != 0 {
                return error;
            }
            error = selectglyph(idx, (*agln).suffix, gm, &raw mut idx);
            if error != 0 {
                pdf_warning!(
                    b"Variant \"%s\" for glyph \"%s\" might not be found.\0" as *const u8 as *const ::core::ffi::c_char,
                    (*agln).suffix,
                    (*agln).name,
                );
                pdf_warning!(b"Using glyph name without suffix instead...\0" as *const u8 as *const ::core::ffi::c_char,);
                error = 0 as ::core::ffi::c_int;
            }
        } else if (*agln).n_components == 1 as ::core::ffi::c_int {
            idx = tt_cmap_lookup((*gm).codetogid, (*agln).unicodes[0 as ::core::ffi::c_int as usize] as SFNT_ULONG);
        } else if (*agln).n_components > 1 as ::core::ffi::c_int {
            if dpx_conf.verbose_level >= 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Glyph \"%s\" looks like a composite glyph...\0" as *const u8 as *const ::core::ffi::c_char,
                    (*agln).name,
                );
            }
            error = composeuchar(
                &raw mut (*agln).unicodes as *mut int32_t,
                (*agln).n_components,
                ::core::ptr::null::<::core::ffi::c_char>(),
                gm,
                &raw mut idx,
            );
            if dpx_conf.verbose_level >= 0 as ::core::ffi::c_int {
                if error != 0 {
                    pdf_warning!(b"Not found...\0" as *const u8 as *const ::core::ffi::c_char,);
                } else {
                    let mut _i: ::core::ffi::c_int = 0;
                    let mut _n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut _p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    let mut _buf: [::core::ffi::c_char; 256] = [0; 256];
                    pdf_warning!(
                        b">> Composite glyph glyph-name=\"%s\" found at glyph-id=\"%u\".\0" as *const u8 as *const ::core::ffi::c_char,
                        (*agln).name,
                        idx as ::core::ffi::c_int,
                    );
                    _p = &raw mut _buf as *mut ::core::ffi::c_char;
                    _i = 0 as ::core::ffi::c_int;
                    while _i < (*agln).n_components && _n < 245 as ::core::ffi::c_int {
                        let fresh0 = _n;
                        _n = _n + 1;
                        *_p.offset(fresh0 as isize) = (if _i == 0 as ::core::ffi::c_int { '<' as i32 } else { ' ' as i32 }) as ::core::ffi::c_char;
                        if (*agln).unicodes[_i as usize] >= 0x10000 as int32_t {
                            _n += sprintf(
                                _p.offset(_n as isize),
                                b"U+%06X\0" as *const u8 as *const ::core::ffi::c_char,
                                (*agln).unicodes[_i as usize],
                            );
                        } else {
                            _n += sprintf(
                                _p.offset(_n as isize),
                                b"U+%04X\0" as *const u8 as *const ::core::ffi::c_char,
                                (*agln).unicodes[_i as usize],
                            );
                        }
                        let fresh1 = _n;
                        _n = _n + 1;
                        *_p.offset(fresh1 as isize) = (if _i == (*agln).n_components - 1 as ::core::ffi::c_int {
                            '>' as i32
                        } else {
                            ',' as i32
                        }) as ::core::ffi::c_char;
                        _i += 1;
                    }
                    let fresh2 = _n;
                    _n = _n + 1;
                    *_p.offset(fresh2 as isize) = '\0' as i32 as ::core::ffi::c_char;
                    pdf_warning!(
                        b">> Input Unicode seq.=\"%s\" ==> glyph-id=\"%u\" in font-file=\"_please_try_-v_\".\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &raw mut _buf as *mut ::core::ffi::c_char,
                        idx as ::core::ffi::c_int,
                    );
                }
            }
        } else {
            if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                pdf_abort!(c"assertion failed: %s".as_ptr(), b"0\0" as *const u8 as *const ::core::ffi::c_char);
            } else {
            };
        }
        agln = (*agln).alternate as *mut agl_name;
    }
    *gid = idx;
    return if idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C-unwind" fn resolve_glyph(
    mut glyphname: *const ::core::ffi::c_char, mut gid: *mut USHORT, mut gm: *mut glyph_mapper,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut suffix: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ucv: int32_t = 0;
    if glyphname.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"glyphname\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    error = findposttable(glyphname, gid, gm);
    if error == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*gm).codetogid.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    name = agl_chop_suffix(glyphname, &raw mut suffix);
    if name.is_null() {
        error = -(1 as ::core::ffi::c_int);
    } else if agl_name_is_unicode(name) {
        ucv = agl_name_convert_unicode(name);
        *gid = tt_cmap_lookup((*gm).codetogid, ucv as SFNT_ULONG);
        error = if *gid as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            -(1 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_int
        };
    } else {
        error = findparanoiac(name, gid, gm);
    }
    if error == 0 && !suffix.is_null() {
        error = selectglyph(*gid, suffix, gm, gid);
        if error != 0 {
            pdf_warning!(
                b"Variant \"%s\" for glyph \"%s\" might not be found.\0" as *const u8 as *const ::core::ffi::c_char,
                suffix,
                name,
            );
            pdf_warning!(b"Using glyph name without suffix instead...\0" as *const u8 as *const ::core::ffi::c_char,);
            error = 0 as ::core::ffi::c_int;
        }
    }
    free(suffix as *mut ::core::ffi::c_void);
    free(name as *mut ::core::ffi::c_void);
    return error;
}
unsafe extern "C-unwind" fn setup_glyph_mapper(mut gm: *mut glyph_mapper, mut sfont: *mut sfnt) -> ::core::ffi::c_int {
    (*gm).sfont = sfont;
    (*gm).nametogid = tt_read_post_table(sfont);
    (*gm).codetogid = tt_cmap_read(sfont, TT_WIN as USHORT, TT_WIN_UCS4 as USHORT);
    if (*gm).codetogid.is_null() {
        (*gm).codetogid = tt_cmap_read(sfont, TT_WIN as USHORT, TT_WIN_UNICODE as USHORT);
    }
    if (*gm).nametogid.is_null() && (*gm).codetogid.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*gm).gsub = otl_gsub_new();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn clean_glyph_mapper(mut gm: *mut glyph_mapper) {
    if !(*gm).gsub.is_null() {
        otl_gsub_release((*gm).gsub);
    }
    if !(*gm).codetogid.is_null() {
        tt_cmap_release((*gm).codetogid);
    }
    if !(*gm).nametogid.is_null() {
        tt_release_post_table((*gm).nametogid);
    }
    (*gm).gsub = ::core::ptr::null_mut::<otl_gsub>();
    (*gm).codetogid = ::core::ptr::null_mut::<tt_cmap>();
    (*gm).nametogid = ::core::ptr::null_mut::<tt_post_table>();
    (*gm).sfont = ::core::ptr::null_mut::<sfnt>();
}
unsafe extern "C-unwind" fn do_custom_encoding(
    mut font: *mut pdf_font, mut encoding: *mut *mut ::core::ffi::c_char, mut usedchars: *const ::core::ffi::c_char, mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut glyphs: *mut tt_glyphs = ::core::ptr::null_mut::<tt_glyphs>();
    let mut cmap_table: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut code: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut widths: [::core::ffi::c_double; 256] = [0.; 256];
    let mut gm: glyph_mapper = glyph_mapper {
        codetogid: ::core::ptr::null_mut::<tt_cmap>(),
        gsub: ::core::ptr::null_mut::<otl_gsub>(),
        sfont: ::core::ptr::null_mut::<sfnt>(),
        nametogid: ::core::ptr::null_mut::<tt_post_table>(),
    };
    let mut idx: USHORT = 0;
    let mut gid: USHORT = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(!font.is_null() && !encoding.is_null() && !usedchars.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font && encoding && usedchars && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    error = setup_glyph_mapper(&raw mut gm, sfont);
    if error != 0 {
        pdf_warning!(
            b"No post table nor Unicode cmap found in font: %s\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).filename,
        );
        pdf_warning!(b">> I can't find glyphs without this!\0" as *const u8 as *const ::core::ffi::c_char,);
        return -(1 as ::core::ffi::c_int);
    }
    cmap_table =
        new((274 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
            as *mut ::core::ffi::c_char;
    memset(cmap_table as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, 274 as size_t);
    put_big_endian(cmap_table as *mut ::core::ffi::c_void, 0 as SFNT_LONG, 2 as ::core::ffi::c_int);
    put_big_endian(
        cmap_table.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        1 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(4 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        1 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(6 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        0 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        12 as SFNT_LONG,
        4 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(12 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        0 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(14 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        262 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    put_big_endian(
        cmap_table.offset(16 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        0 as SFNT_LONG,
        2 as ::core::ffi::c_int,
    );
    glyphs = tt_build_init();
    count = 1 as ::core::ffi::c_int;
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        if !(*usedchars.offset(code as isize) == 0) {
            if (*encoding.offset(code as isize)).is_null()
                || streq_ptr(*encoding.offset(code as isize), b".notdef\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int != 0
            {
                pdf_warning!(
                    b"Character code=\"0x%02X\" mapped to \".notdef\" glyph used in font font-file=\"%s\"\0" as *const u8
                        as *const ::core::ffi::c_char,
                    code,
                    (*font).filename,
                );
                pdf_warning!(b">> Maybe incorrect encoding specified?\0" as *const u8 as *const ::core::ffi::c_char,);
                idx = 0 as USHORT;
            } else {
                if !strchr(*encoding.offset(code as isize), '_' as i32).is_null() {
                    error = findcomposite(*encoding.offset(code as isize), &raw mut gid, &raw mut gm);
                } else {
                    error = resolve_glyph(*encoding.offset(code as isize), &raw mut gid, &raw mut gm);
                }
                if error != 0 {
                    pdf_warning!(
                        b"Glyph \"%s\" not available in font \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                        *encoding.offset(code as isize),
                        (*font).filename,
                    );
                } else if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
                    pdf_message!(
                        b"truetype>> Glyph glyph-name=\"%s\" found at glyph-id=\"%u\".\n\0" as *const u8 as *const ::core::ffi::c_char,
                        *encoding.offset(code as isize),
                        gid as ::core::ffi::c_int,
                    );
                }
                idx = tt_find_glyph(glyphs, gid);
                if idx as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    idx = tt_add_glyph(glyphs, gid, count as USHORT);
                    count += 1;
                }
            }
            *cmap_table.offset((18 as ::core::ffi::c_int + code) as isize) =
                (idx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_char;
        }
        code += 1;
    }
    clean_glyph_mapper(&raw mut gm);
    if tt_build_tables(sfont, glyphs) < 0 as ::core::ffi::c_int {
        pdf_warning!(b"Packing TrueType font into SFNT file faild...\0" as *const u8 as *const ::core::ffi::c_char,);
        tt_build_finish(glyphs);
        free(cmap_table as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        if *usedchars.offset(code as isize) != 0 {
            idx = tt_get_index(glyphs, *cmap_table.offset((18 as ::core::ffi::c_int + code) as isize) as USHORT);
            widths[code as usize] = floor(
                1000.0f64 * (*(*glyphs).gd.offset(idx as isize)).advw as ::core::ffi::c_int as ::core::ffi::c_double
                    / (*glyphs).emsize as ::core::ffi::c_int as ::core::ffi::c_double
                    / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    + 0.5f64,
            ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            widths[code as usize] = 0.0f64;
        }
        code += 1;
    }
    do_widths(font, &raw mut widths as *mut ::core::ffi::c_double);
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        pdf_message!(
            b"[%d glyphs]\0" as *const u8 as *const ::core::ffi::c_char,
            (*glyphs).num_glyphs as ::core::ffi::c_int,
        );
    }
    tt_build_finish(glyphs);
    sfnt_set_table(
        sfont,
        b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
        cmap_table as *mut ::core::ffi::c_void,
        274 as SFNT_ULONG,
    );
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_load_truetype")]
pub unsafe extern "C-unwind" fn pdf_font_load_truetype(mut font: *mut pdf_font) -> ::core::ffi::c_int {
    let mut descriptor: *mut pdf_obj = (*font).descriptor;
    let mut ident: *mut ::core::ffi::c_char = (*font).filename;
    let mut encoding_id: ::core::ffi::c_int = (*font).encoding_id;
    let mut usedchars: *mut ::core::ffi::c_char = (*font).usedchars;
    let mut index: ::core::ffi::c_int = (*font).index as ::core::ffi::c_int;
    let mut enc_vec: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut fontfile: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut i: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*font).reference.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    handle = dpx_open_truetype_file(ident);
    if handle == INVALID_HANDLE {
        handle = dpx_open_dfont_file(ident);
        if handle == INVALID_HANDLE {
            pdf_abort!(
                b"Unable to open TrueType/dfont font file: %s\0" as *const u8 as *const ::core::ffi::c_char,
                ident,
            );
        }
        sfont = dfont_open(handle, index);
    } else {
        sfont = sfnt_open(handle);
    }
    if sfont.is_null() {
        crate::backend::runtime::input_close(handle);
        pdf_abort!(
            b"Unable to open TrueType/dfont file: %s\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    } else if (*sfont).type_0 != SFNT_TYPE_TRUETYPE && (*sfont).type_0 != SFNT_TYPE_TTC && (*sfont).type_0 != SFNT_TYPE_DFONT {
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        pdf_abort!(
            b"Font \"%s\" not a TrueType/dfont font?\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    }
    if (*sfont).type_0 == SFNT_TYPE_TTC {
        let mut offset: SFNT_ULONG = 0;
        offset = ttc_read_offset(sfont, index as SFNT_ULONG);
        if offset == 0 as SFNT_ULONG {
            pdf_abort!(b"Invalid TTC index in %s.\0" as *const u8 as *const ::core::ffi::c_char, ident,);
        }
        error = sfnt_read_table_directory(sfont, offset);
    } else {
        error = sfnt_read_table_directory(sfont, (*sfont).offset);
    }
    if error != 0 {
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        pdf_abort!(
            b"Reading SFND table dir failed for font-file=\"%s\"... Not a TrueType font?\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    }
    if encoding_id < 0 as ::core::ffi::c_int {
        error = do_builtin_encoding(font, usedchars, sfont);
    } else {
        enc_vec = pdf_encoding_get_encoding(encoding_id);
        error = do_custom_encoding(font, enc_vec, usedchars, sfont);
    }
    if error != 0 {
        sfnt_close(sfont);
        crate::backend::runtime::input_close(handle);
        pdf_abort!(
            b"Error occured while creating font subfont for \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while !REQUIRED_TABLE[i as usize].name.is_null() {
        if sfnt_require_table(sfont, REQUIRED_TABLE[i as usize].name, REQUIRED_TABLE[i as usize].must_exist) < 0 as ::core::ffi::c_int {
            sfnt_close(sfont);
            crate::backend::runtime::input_close(handle);
            pdf_abort!(
                b"Required TrueType table \"%s\" does not exist in font: %s\0" as *const u8 as *const ::core::ffi::c_char,
                REQUIRED_TABLE[i as usize].name,
                ident,
            );
        }
        i += 1;
    }
    fontfile = sfnt_create_FontFile_stream(sfont);
    if fontfile.is_null() {
        pdf_abort!(
            b"Could not created FontFile stream for \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    }
    sfnt_close(sfont);
    crate::backend::runtime::input_close(handle);
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        pdf_message!(b"[%d bytes]\0" as *const u8 as *const ::core::ffi::c_char, pdf_stream_length(fontfile),);
    }
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"FontFile2\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj(fontfile),
    );
    pdf_release_obj(fontfile);
    return 0 as ::core::ffi::c_int;
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
    fn memmove(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __len: size_t) -> *mut ::core::ffi::c_void;
    fn memset(__b: *mut ::core::ffi::c_void, __c: ::core::ffi::c_int, __len: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[repr(C)]
pub struct otl_gsub {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_open_truetype(
    mut font: *mut pdf_font, mut ident: *const ::core::ffi::c_char, mut index: ::core::ffi::c_int, mut encoding_id: ::core::ffi::c_int,
    mut embedding: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_open_truetype(font, ident, index, encoding_id, embedding) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_font_load_truetype(mut font: *mut pdf_font, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_load_truetype(font) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
