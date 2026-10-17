/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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
use crate::backend::dpx_pdfdraw::pdf_dev_transform;
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn modf(_: ::core::ffi::c_double, _: *mut ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn round(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn labs(_: ::core::ffi::c_long) -> ::core::ffi::c_long;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_link_obj"]
    fn pdf_link_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdfobj_escape_str"]
    fn pdfobj_escape_str(
        buffer: *mut ::core::ffi::c_char,
        size: size_t,
        s: *const ::core::ffi::c_uchar,
        len: size_t,
    ) -> size_t;
    #[link_name = "__pdf_impl_pdf_color_clear_stack"]
    fn pdf_color_clear_stack();
    #[link_name = "__pdf_impl_pdf_color_get_current"]
    fn pdf_color_get_current(sc: *mut *mut pdf_color, fc: *mut *mut pdf_color);
    #[link_name = "__pdf_impl_pdf_lookup_fontmap_record"]
    fn pdf_lookup_fontmap_record(kp: *const ::core::ffi::c_char) -> *mut fontmap_rec;
    #[link_name = "__pdf_impl_pdf_font_findresource"]
    fn pdf_font_findresource(
        tex_name: *const ::core::ffi::c_char,
        font_scale: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_load_font"]
    fn pdf_font_load_font(
        tex_name: *const ::core::ffi::c_char,
        font_scale: ::core::ffi::c_double,
        mrec: *const fontmap_rec,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_font_subtype"]
    fn pdf_get_font_subtype(font_id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_font_reference"]
    fn pdf_get_font_reference(font_id: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_get_font_usedchars"]
    fn pdf_get_font_usedchars(font_id: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_get_font_encoding"]
    fn pdf_get_font_encoding(font_id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_font_wmode"]
    fn pdf_get_font_wmode(font_id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_font_resource_name"]
    fn pdf_font_resource_name(
        font_id: ::core::ffi::c_int,
        buf: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_get_profile"]
    fn CMap_get_profile(cmap: *mut CMap, type_0: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_decode"]
    fn CMap_decode(
        cmap: *mut CMap,
        inbuf: *mut *const ::core::ffi::c_uchar,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut ::core::ffi::c_uchar,
        outbytesleft: *mut size_t,
    ) -> size_t;
    #[link_name = "__pdf_impl_CMap_cache_get"]
    fn CMap_cache_get(id: ::core::ffi::c_int) -> *mut CMap;
    static mut dpx_conf: _dpx_conf;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_doc_add_page_content"]
    fn pdf_doc_add_page_content(buffer: *const ::core::ffi::c_char, length: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_doc_add_page_resource"]
    fn pdf_doc_add_page_resource(
        category: *const ::core::ffi::c_char,
        resource_name: *const ::core::ffi::c_char,
        resources: *mut pdf_obj,
    );
    #[link_name = "__pdf_impl_pdf_dev_init_gstates"]
    fn pdf_dev_init_gstates();
    #[link_name = "__pdf_impl_pdf_dev_clear_gstates"]
    fn pdf_dev_clear_gstates();
    #[link_name = "__pdf_impl_pdf_dev_rectclip"]
    fn pdf_dev_rectclip(
        x: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        w: ::core::ffi::c_double,
        h: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_concat"]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_gsave"]
    fn pdf_dev_gsave() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_grestore"]
    fn pdf_dev_grestore() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_current_depth"]
    fn pdf_dev_current_depth() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_grestore_to"]
    fn pdf_dev_grestore_to(depth: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_dev_set_color"]
    fn pdf_dev_set_color(
        color: *const pdf_color,
        mask: ::core::ffi::c_char,
        force: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_dev_reset_xgstate"]
    fn pdf_dev_reset_xgstate(force: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ximage_get_resname"]
    fn pdf_ximage_get_resname(xobj_id: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_ximage_get_reference"]
    fn pdf_ximage_get_reference(xobj_id: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_ximage_scale_image"]
    fn pdf_ximage_scale_image(
        id: ::core::ffi::c_int,
        M: *mut pdf_tmatrix,
        r: *mut pdf_rect,
        p: *mut transform_info,
    ) -> ::core::ffi::c_int;

}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint16_t = u16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_color {
    pub res_id: ::core::ffi::c_int,
    pub type_0: ::core::ffi::c_int,
    pub num_components: ::core::ffi::c_int,
    pub spot_color_name: *mut ::core::ffi::c_char,
    pub values: [::core::ffi::c_double; 32],
    pub pattern_id: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_dev {
    pub motion_state: ::core::ffi::c_int,
    pub param: dev_param,
    pub unit: dev_unit,
    pub text_state: text_state,
    pub fonts: *mut dev_font,
    pub num_dev_fonts: ::core::ffi::c_int,
    pub max_dev_fonts: ::core::ffi::c_int,
    pub format_buffer: [::core::ffi::c_char; 4097],
    pub render_output: *mut ::core::ffi::c_void,
    pub recording_page: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dev_font {
    pub short_name: [::core::ffi::c_char; 16],
    pub used_on_this_page: ::core::ffi::c_int,
    pub tex_name: *mut ::core::ffi::c_char,
    pub sptsize: spt_t,
    pub font_id: ::core::ffi::c_int,
    pub enc_id: ::core::ffi::c_int,
    pub resource: *mut pdf_obj,
    pub used_chars: *mut ::core::ffi::c_char,
    pub format: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
    pub extend: ::core::ffi::c_double,
    pub slant: ::core::ffi::c_double,
    pub bold: ::core::ffi::c_double,
}
pub type spt_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text_state {
    pub font_id: ::core::ffi::c_int,
    pub offset: spt_t,
    pub ref_x: spt_t,
    pub ref_y: spt_t,
    pub raise: spt_t,
    pub leading: spt_t,
    pub matrix: C2RustUnnamed,
    pub bold_param: ::core::ffi::c_double,
    pub dir_mode: ::core::ffi::c_int,
    pub force_reset: ::core::ffi::c_int,
    pub is_mb: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub slant: ::core::ffi::c_double,
    pub extend: ::core::ffi::c_double,
    pub rotate: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dev_unit {
    pub dvi2pts: ::core::ffi::c_double,
    pub min_bp_val: ::core::ffi::c_int,
    pub precision: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dev_param {
    pub autorotate: ::core::ffi::c_int,
    pub colormode: ::core::ffi::c_int,
}
use crate::compat::renderer::pdf_coord;
use crate::compat::renderer::pdf_rect;
use crate::compat::renderer::pdf_tmatrix;
use crate::compat::renderer::transform_info;
pub const TT_PDF_GRAPHICS_MODE: C2RustUnnamed_4 = 3;
use crate::compat::renderer::tt_pdf_render_font;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_1,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: ::core::ffi::c_int,
    pub profile: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
pub struct C2RustUnnamed_1 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDSysInfo {
    pub registry: *mut ::core::ffi::c_char,
    pub ordering: *mut ::core::ffi::c_char,
    pub supplement: ::core::ffi::c_int,
}
pub const TT_PDF_RULE: C2RustUnnamed_4 = 2;
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
    pub charmap: C2RustUnnamed_2,
    pub opt: fontmap_opt,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub const TT_PDF_DIRECTION: C2RustUnnamed_4 = 4;
pub const TT_PDF_RESET_FONTS: C2RustUnnamed_4 = 5;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const TT_PDF_TEXT: C2RustUnnamed_4 = 1;
pub const TT_PDF_RAW_BYTES: C2RustUnnamed_4 = 0;
use crate::compat::renderer::tt_pdf_render_command;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub const INFO_DO_CLIP: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const PDF_DEV_PARAM_AUTOROTATE: ::core::ffi::c_int = 1;
pub const PDF_DEV_PARAM_COLORMODE: ::core::ffi::c_int = 2;
pub const FONTMAP_OPT_NOEMBED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const FONTMAP_STYLE_BOLD: ::core::ffi::c_int = 1;
pub const FONTMAP_STYLE_ITALIC: ::core::ffi::c_int = 2;
pub const FONTMAP_STYLE_BOLDITALIC: ::core::ffi::c_int = 3;
pub const PDF_FONT_FONTTYPE_TYPE3: ::core::ffi::c_int = 2;
pub const PDF_FONT_FONTTYPE_TYPE0: ::core::ffi::c_int = 4;
pub const CMAP_PROF_TYPE_INBYTES_MIN: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GRAPHICS_MODE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TEXT_MODE: ::core::ffi::c_int = 2;
pub const STRING_MODE: ::core::ffi::c_int = 3;
pub const TEXT_WMODE_HH: ::core::ffi::c_int = 0;
pub const TEXT_WMODE_HV: ::core::ffi::c_int = 1;
pub const TEXT_WMODE_VH: ::core::ffi::c_int = 4;
pub const TEXT_WMODE_VV: ::core::ffi::c_int = 5;
pub const TEXT_WMODE_HD: ::core::ffi::c_int = 3;
pub const TEXT_WMODE_VD: ::core::ffi::c_int = 7;
pub const PDF_FONTTYPE_SIMPLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PDF_FONTTYPE_BITMAP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_FONTTYPE_COMPOSITE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const FORMAT_BUF_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
static mut pdev: pdf_dev = pdf_dev {
    motion_state: 0,
    param: dev_param {
        autorotate: 0,
        colormode: 0,
    },
    unit: dev_unit {
        dvi2pts: 0.,
        min_bp_val: 0,
        precision: 0,
    },
    text_state: text_state {
        font_id: 0,
        offset: 0,
        ref_x: 0,
        ref_y: 0,
        raise: 0,
        leading: 0,
        matrix: C2RustUnnamed {
            slant: 0.,
            extend: 0.,
            rotate: 0,
        },
        bold_param: 0.,
        dir_mode: 0,
        force_reset: 0,
        is_mb: 0,
    },
    fonts: ::core::ptr::null::<dev_font>() as *mut dev_font,
    num_dev_fonts: 0,
    max_dev_fonts: 0,
    format_buffer: [0; 4097],
    render_output: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    recording_page: 0,
};
unsafe extern "C-unwind" fn current_device() -> *mut pdf_dev {
    return &raw mut pdev;
}
unsafe extern "C-unwind" fn dev_out(
    mut p: *mut pdf_dev,
    mut str: *const ::core::ffi::c_char,
    mut len: size_t,
) {
    if !(*p).render_output.is_null() {
        crate::compat::renderer::append_output((*p).render_output, str, len);
    } else {
        pdf_doc_add_page_content(str, len as ::core::ffi::c_uint);
    };
}
unsafe extern "C-unwind" fn pdf_dev_unit_dviunit(mut p: *mut pdf_dev) -> ::core::ffi::c_double {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return 1.0f64 / (*p).unit.dvi2pts;
}
pub const DEV_PRECISION_MAX: ::core::ffi::c_int = 8;
use crate::numbers::{INVERSE_POWERS_OF_TEN as ten_pow_inv, POWERS_OF_TEN as ten_pow};

unsafe extern "C-unwind" fn p_itoa(
    value: ::core::ffi::c_int,
    buf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_uint {
    let mut bytes = [0; 11];
    let length = crate::numbers::format_integer(value, &mut bytes).unwrap();
    // Legacy callers provide space for the number and a trailing NUL.
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf.cast(), length);
        *buf.add(length) = 0;
    }
    length as ::core::ffi::c_uint
}

unsafe extern "C-unwind" fn p_dtoa(
    value: f64,
    prec: ::core::ffi::c_int,
    buf: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut bytes = [0; crate::numbers::DECIMAL_BUFFER_SIZE];
    let length = crate::numbers::format_decimal(value, prec, &mut bytes).unwrap_or_else(|_| {
        crate::backend::runtime::raise(c"Invalid PDF numeric precision".to_owned())
    });
    // Only borrow the bytes actually written; legacy callers supply no capacity.
    unsafe {
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf.cast(), length);
        *buf.add(length) = 0;
    }
    length as ::core::ffi::c_int
}
unsafe extern "C-unwind" fn dev_sprint_bp(
    mut p: *mut pdf_dev,
    mut buf: *mut ::core::ffi::c_char,
    mut value: spt_t,
    mut error: *mut spt_t,
) -> ::core::ffi::c_int {
    let mut value_in_bp: ::core::ffi::c_double = 0.;
    let mut error_in_bp: ::core::ffi::c_double = 0.;
    let mut prec: ::core::ffi::c_int = 0;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    prec = (*p).unit.precision;
    value_in_bp = value as ::core::ffi::c_double * (*p).unit.dvi2pts;
    if !error.is_null() {
        error_in_bp = value_in_bp
            - floor(value_in_bp / ten_pow_inv[prec as usize] + 0.5f64) * ten_pow_inv[prec as usize];
        *error = round(error_in_bp / (*p).unit.dvi2pts) as spt_t;
    }
    return p_dtoa(value_in_bp, prec, buf);
}
unsafe extern "C-unwind" fn pdf_dev_sprint_matrix(
    mut p: *mut pdf_dev,
    mut buf: *mut ::core::ffi::c_char,
    mut M: *const pdf_tmatrix,
) -> size_t {
    let mut len: size_t = 0;
    let mut prec0: ::core::ffi::c_int = 0;
    let mut prec2: ::core::ffi::c_int = 0;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    prec2 = if ((*p).unit.precision + 2 as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        (*p).unit.precision + 2 as ::core::ffi::c_int
    } else {
        8 as ::core::ffi::c_int
    };
    prec0 = if (*p).unit.precision > 2 as ::core::ffi::c_int {
        (*p).unit.precision
    } else {
        2 as ::core::ffi::c_int
    };
    len = p_dtoa((*M).a, prec2, buf) as size_t;
    let fresh0 = len;
    len = len.wrapping_add(1);
    *buf.offset(fresh0 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len = len.wrapping_add(p_dtoa((*M).b, prec2, buf.offset(len as isize)) as size_t);
    let fresh1 = len;
    len = len.wrapping_add(1);
    *buf.offset(fresh1 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len = len.wrapping_add(p_dtoa((*M).c, prec2, buf.offset(len as isize)) as size_t);
    let fresh2 = len;
    len = len.wrapping_add(1);
    *buf.offset(fresh2 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len = len.wrapping_add(p_dtoa((*M).d, prec2, buf.offset(len as isize)) as size_t);
    let fresh3 = len;
    len = len.wrapping_add(1);
    *buf.offset(fresh3 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len = len.wrapping_add(p_dtoa((*M).e, prec0, buf.offset(len as isize)) as size_t);
    let fresh4 = len;
    len = len.wrapping_add(1);
    *buf.offset(fresh4 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len = len.wrapping_add(p_dtoa((*M).f, prec0, buf.offset(len as isize)) as size_t);
    *buf.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    return len;
}
unsafe extern "C-unwind" fn pdf_dev_sprint_rect(
    mut p: *mut pdf_dev,
    mut buf: *mut ::core::ffi::c_char,
    mut rect: *const pdf_rect,
) -> size_t {
    let mut len: size_t = 0;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    len = p_dtoa((*rect).llx, (*p).unit.precision, buf) as size_t;
    let fresh10 = len;
    len = len.wrapping_add(1);
    *buf.offset(fresh10 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len = len
        .wrapping_add(p_dtoa((*rect).lly, (*p).unit.precision, buf.offset(len as isize)) as size_t);
    let fresh11 = len;
    len = len.wrapping_add(1);
    *buf.offset(fresh11 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len = len
        .wrapping_add(p_dtoa((*rect).urx, (*p).unit.precision, buf.offset(len as isize)) as size_t);
    let fresh12 = len;
    len = len.wrapping_add(1);
    *buf.offset(fresh12 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len = len
        .wrapping_add(p_dtoa((*rect).ury, (*p).unit.precision, buf.offset(len as isize)) as size_t);
    *buf.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    return len;
}
unsafe extern "C-unwind" fn pdf_dev_sprint_coord(
    mut p: *mut pdf_dev,
    mut buf: *mut ::core::ffi::c_char,
    mut c: *const pdf_coord,
) -> size_t {
    let mut len: size_t = 0;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    len = p_dtoa((*c).x, (*p).unit.precision, buf) as size_t;
    let fresh13 = len;
    len = len.wrapping_add(1);
    *buf.offset(fresh13 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len = len.wrapping_add(p_dtoa((*c).y, (*p).unit.precision, buf.offset(len as isize)) as size_t);
    *buf.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    return len;
}
unsafe extern "C-unwind" fn pdf_dev_sprint_length(
    mut p: *mut pdf_dev,
    mut buf: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_double,
) -> size_t {
    let mut len: size_t = 0;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    len = p_dtoa(value, (*p).unit.precision, buf) as size_t;
    *buf.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    return len;
}
unsafe extern "C-unwind" fn pdf_dev_sprint_number(
    mut p: *mut pdf_dev,
    mut buf: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_double,
) -> size_t {
    let mut len: size_t = 0;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    len = p_dtoa(value, DEV_PRECISION_MAX, buf) as size_t;
    *buf.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    return len;
}
unsafe extern "C-unwind" fn dev_set_text_matrix(
    mut p: *mut pdf_dev,
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut slant: ::core::ffi::c_double,
    mut extend: ::core::ffi::c_double,
    mut rotate: ::core::ffi::c_int,
) {
    let mut tm: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match rotate {
        TEXT_WMODE_VH => {
            tm.a = slant;
            tm.b = 1.0f64;
            tm.c = -extend;
            tm.d = 0.0f64;
        }
        TEXT_WMODE_HV => {
            tm.a = 0.0f64;
            tm.b = -extend;
            tm.c = 1.0f64;
            tm.d = -slant;
        }
        TEXT_WMODE_HH => {
            tm.a = extend;
            tm.b = 0.0f64;
            tm.c = slant;
            tm.d = 1.0f64;
        }
        TEXT_WMODE_VV => {
            tm.a = 1.0f64;
            tm.b = -slant;
            tm.c = 0.0f64;
            tm.d = extend;
        }
        TEXT_WMODE_HD => {
            tm.a = 0.0f64;
            tm.b = extend;
            tm.c = -1.0f64;
            tm.d = slant;
        }
        TEXT_WMODE_VD => {
            tm.a = -1.0f64;
            tm.b = slant;
            tm.c = 0.0f64;
            tm.d = -extend;
        }
        _ => {}
    }
    tm.e = xpos as ::core::ffi::c_double * (*p).unit.dvi2pts;
    tm.f = ypos as ::core::ffi::c_double * (*p).unit.dvi2pts;
    let fresh34 = len;
    len = len + 1;
    (*p).format_buffer[fresh34 as usize] = ' ' as i32 as ::core::ffi::c_char;
    len = (len as size_t).wrapping_add(pdf_dev_sprint_matrix(
        p,
        (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
        &raw mut tm,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    let fresh35 = len;
    len = len + 1;
    (*p).format_buffer[fresh35 as usize] = ' ' as i32 as ::core::ffi::c_char;
    let fresh36 = len;
    len = len + 1;
    (*p).format_buffer[fresh36 as usize] = 'T' as i32 as ::core::ffi::c_char;
    let fresh37 = len;
    len = len + 1;
    (*p).format_buffer[fresh37 as usize] = 'm' as i32 as ::core::ffi::c_char;
    dev_out(
        p,
        &raw mut (*p).format_buffer as *mut ::core::ffi::c_char,
        len as size_t,
    );
    (*p).text_state.ref_x = xpos;
    (*p).text_state.ref_y = ypos;
    (*p).text_state.matrix.slant = slant;
    (*p).text_state.matrix.extend = extend;
    (*p).text_state.matrix.rotate = rotate;
}
unsafe extern "C-unwind" fn reset_text_state(mut p: *mut pdf_dev) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    dev_out(
        p,
        b" BT\0" as *const u8 as *const ::core::ffi::c_char,
        3 as size_t,
    );
    if (*p).text_state.force_reset != 0
        || (*p).text_state.matrix.slant != 0.0f64
        || (*p).text_state.matrix.extend != 1.0f64
        || (*p).text_state.matrix.rotate != TEXT_WMODE_HH
            && (*p).text_state.matrix.rotate != TEXT_WMODE_VV
    {
        dev_set_text_matrix(
            p,
            0 as spt_t,
            0 as spt_t,
            (*p).text_state.matrix.slant,
            (*p).text_state.matrix.extend,
            (*p).text_state.matrix.rotate,
        );
    }
    (*p).text_state.ref_x = 0 as ::core::ffi::c_int as spt_t;
    (*p).text_state.ref_y = 0 as ::core::ffi::c_int as spt_t;
    (*p).text_state.offset = 0 as ::core::ffi::c_int as spt_t;
    (*p).text_state.force_reset = 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdf_dev_text_mode(mut p: *mut pdf_dev) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match (*p).motion_state {
        STRING_MODE => {
            dev_out(
                p,
                if (*p).text_state.is_mb != 0 {
                    b">]TJ\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b")]TJ\0" as *const u8 as *const ::core::ffi::c_char
                },
                4 as size_t,
            );
        }
        GRAPHICS_MODE => {
            reset_text_state(p);
        }
        TEXT_MODE | _ => {}
    }
    (*p).motion_state = TEXT_MODE;
    (*p).text_state.offset = 0 as ::core::ffi::c_int as spt_t;
}
unsafe extern "C-unwind" fn pdf_dev_graphics_mode(mut p: *mut pdf_dev) {
    if (*p).recording_page != 0 {
        crate::pipeline::page_command(
            TT_PDF_GRAPHICS_MODE as ::core::ffi::c_int,
            0 as int32_t,
            0 as int32_t,
            0 as int32_t,
            0 as int32_t,
        );
        (*p).motion_state = GRAPHICS_MODE;
        (*p).text_state.font_id = -(1 as ::core::ffi::c_int);
        (*p).text_state.bold_param = 0.0f64;
        (*p).text_state.force_reset = 0 as ::core::ffi::c_int;
        return;
    }
    let mut current_block_15: u64;
    match (*p).motion_state {
        STRING_MODE => {
            dev_out(
                p,
                if (*p).text_state.is_mb != 0 {
                    b">]TJ\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b")]TJ\0" as *const u8 as *const ::core::ffi::c_char
                },
                4 as size_t,
            );
            current_block_15 = 14418776834865182077;
        }
        TEXT_MODE => {
            current_block_15 = 14418776834865182077;
        }
        GRAPHICS_MODE | _ => {
            current_block_15 = 2979737022853876585;
        }
    }
    match current_block_15 {
        14418776834865182077 => {
            if (*p).text_state.bold_param != 0.0f64 {
                dev_out(
                    p,
                    b" 0 Tr\0" as *const u8 as *const ::core::ffi::c_char,
                    5 as size_t,
                );
                (*p).text_state.bold_param = 0.0f64;
            }
            dev_out(
                p,
                b" ET\0" as *const u8 as *const ::core::ffi::c_char,
                3 as size_t,
            );
            (*p).text_state.force_reset = 0 as ::core::ffi::c_int;
            (*p).text_state.font_id = -(1 as ::core::ffi::c_int);
        }
        _ => {}
    }
    (*p).motion_state = GRAPHICS_MODE;
}
unsafe extern "C-unwind" fn start_string(
    mut p: *mut pdf_dev,
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut slant: ::core::ffi::c_double,
    mut extend: ::core::ffi::c_double,
    mut rotate: ::core::ffi::c_int,
) {
    let mut delx: spt_t = 0;
    let mut dely: spt_t = 0;
    let mut error_delx: spt_t = 0 as spt_t;
    let mut error_dely: spt_t = 0 as spt_t;
    let mut desired_delx: spt_t = 0;
    let mut desired_dely: spt_t = 0;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    delx = xpos - (*p).text_state.ref_x;
    dely = ypos - (*p).text_state.ref_y;
    match rotate {
        TEXT_WMODE_VH => {
            desired_delx = dely;
            desired_dely = (-(delx as ::core::ffi::c_double
                - dely as ::core::ffi::c_double * slant)
                / extend) as spt_t;
            let fresh22 = len;
            len = len + 1;
            (*p).format_buffer[fresh22 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_delx,
                &raw mut error_dely,
            );
            let fresh23 = len;
            len = len + 1;
            (*p).format_buffer[fresh23 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_dely,
                &raw mut error_delx,
            );
            error_delx = -error_delx;
        }
        TEXT_WMODE_HV => {
            desired_delx = (-(dely as ::core::ffi::c_double
                + delx as ::core::ffi::c_double * slant)
                / extend) as spt_t;
            desired_dely = delx;
            let fresh24 = len;
            len = len + 1;
            (*p).format_buffer[fresh24 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_delx,
                &raw mut error_dely,
            );
            let fresh25 = len;
            len = len + 1;
            (*p).format_buffer[fresh25 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_dely,
                &raw mut error_delx,
            );
            error_dely = -error_dely;
        }
        TEXT_WMODE_HH => {
            desired_delx = ((delx as ::core::ffi::c_double - dely as ::core::ffi::c_double * slant)
                / extend) as spt_t;
            desired_dely = dely;
            let fresh26 = len;
            len = len + 1;
            (*p).format_buffer[fresh26 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_delx,
                &raw mut error_delx,
            );
            let fresh27 = len;
            len = len + 1;
            (*p).format_buffer[fresh27 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_dely,
                &raw mut error_dely,
            );
        }
        TEXT_WMODE_VV => {
            desired_delx = delx;
            desired_dely = ((dely as ::core::ffi::c_double + delx as ::core::ffi::c_double * slant)
                / extend) as spt_t;
            let fresh28 = len;
            len = len + 1;
            (*p).format_buffer[fresh28 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_delx,
                &raw mut error_delx,
            );
            let fresh29 = len;
            len = len + 1;
            (*p).format_buffer[fresh29 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_dely,
                &raw mut error_dely,
            );
        }
        TEXT_WMODE_HD => {
            desired_delx = -((-(dely as ::core::ffi::c_double
                + delx as ::core::ffi::c_double * slant)
                / extend) as spt_t);
            desired_dely = -delx;
            let fresh30 = len;
            len = len + 1;
            (*p).format_buffer[fresh30 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_delx,
                &raw mut error_dely,
            );
            let fresh31 = len;
            len = len + 1;
            (*p).format_buffer[fresh31 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_dely,
                &raw mut error_delx,
            );
            error_delx = -error_delx;
            error_dely = -error_dely;
        }
        TEXT_WMODE_VD => {
            desired_delx = -delx;
            desired_dely = -(((dely as ::core::ffi::c_double
                + delx as ::core::ffi::c_double * slant)
                / extend) as spt_t);
            let fresh32 = len;
            len = len + 1;
            (*p).format_buffer[fresh32 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_delx,
                &raw mut error_delx,
            );
            let fresh33 = len;
            len = len + 1;
            (*p).format_buffer[fresh33 as usize] = ' ' as i32 as ::core::ffi::c_char;
            len += dev_sprint_bp(
                p,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
                desired_dely,
                &raw mut error_dely,
            );
            error_delx = -error_delx;
            error_dely = -error_dely;
        }
        _ => {}
    }
    dev_out(
        p,
        &raw mut (*p).format_buffer as *mut ::core::ffi::c_char,
        len as size_t,
    );
    dev_out(
        p,
        if (*p).text_state.is_mb != 0 {
            b" Td[<\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b" Td[(\0" as *const u8 as *const ::core::ffi::c_char
        },
        5 as size_t,
    );
    (*p).text_state.ref_x = xpos - error_delx;
    (*p).text_state.ref_y = ypos - error_dely;
    (*p).text_state.offset = 0 as ::core::ffi::c_int as spt_t;
}
unsafe extern "C-unwind" fn pdf_dev_string_mode(
    mut p: *mut pdf_dev,
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut slant: ::core::ffi::c_double,
    mut extend: ::core::ffi::c_double,
    mut rotate: ::core::ffi::c_int,
) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    let mut current_block_8: u64;
    match (*p).motion_state {
        GRAPHICS_MODE => {
            reset_text_state(p);
            current_block_8 = 9927591279022764779;
        }
        TEXT_MODE => {
            current_block_8 = 9927591279022764779;
        }
        STRING_MODE | _ => {
            current_block_8 = 13513818773234778473;
        }
    }
    match current_block_8 {
        9927591279022764779 => {
            if (*p).text_state.force_reset != 0 {
                dev_set_text_matrix(p, xpos, ypos, slant, extend, rotate);
                dev_out(
                    p,
                    if (*p).text_state.is_mb != 0 {
                        b"[<\0" as *const u8 as *const ::core::ffi::c_char
                    } else {
                        b"[(\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    2 as size_t,
                );
                (*p).text_state.force_reset = 0 as ::core::ffi::c_int;
            } else {
                start_string(p, xpos, ypos, slant, extend, rotate);
            }
        }
        _ => {}
    }
    (*p).motion_state = STRING_MODE;
}
unsafe extern "C-unwind" fn pdf_dev_set_font(
    mut p: *mut pdf_dev,
    mut font_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut font: *mut dev_font = ::core::ptr::null_mut::<dev_font>();
    let mut text_rotate: ::core::ffi::c_int = 0;
    let mut font_scale: ::core::ffi::c_double = 0.;
    let mut len: ::core::ffi::c_int = 0;
    let mut vert_dir: ::core::ffi::c_int = 0;
    let mut vert_font: ::core::ffi::c_int = 0;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_dev_text_mode(p);
    font = (*p).fonts.offset(font_id as isize) as *mut dev_font;
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*p).text_state.is_mb = if (*font).format == PDF_FONTTYPE_COMPOSITE {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    vert_font = if (*font).wmode != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if (*p).param.autorotate != 0 {
        vert_dir = (*p).text_state.dir_mode;
    } else {
        vert_dir = vert_font;
    }
    text_rotate = vert_font << 2 as ::core::ffi::c_int | vert_dir;
    if (*font).slant != (*p).text_state.matrix.slant
        || (*font).extend != (*p).text_state.matrix.extend
        || (if abs(text_rotate - (*p).text_state.matrix.rotate) % 5 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) != 0
    {
        (*p).text_state.force_reset = 1 as ::core::ffi::c_int;
    }
    (*p).text_state.matrix.slant = (*font).slant;
    (*p).text_state.matrix.extend = (*font).extend;
    (*p).text_state.matrix.rotate = text_rotate;
    if (*p).render_output.is_null() && (*font).resource.is_null() {
        (*font).resource = pdf_get_font_reference((*font).font_id);
        (*font).used_chars = pdf_get_font_usedchars((*font).font_id);
    }
    if (*p).render_output.is_null() && (*font).used_on_this_page == 0 {
        pdf_doc_add_page_resource(
            b"Font\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*font).short_name as *mut ::core::ffi::c_char,
            pdf_link_obj((*font).resource),
        );
        (*font).used_on_this_page = 1 as ::core::ffi::c_int;
    }
    font_scale = (*font).sptsize as ::core::ffi::c_double * (*p).unit.dvi2pts;
    len = sprintf(
        &raw mut (*p).format_buffer as *mut ::core::ffi::c_char,
        b" /%s\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut (*font).short_name as *mut ::core::ffi::c_char,
    );
    let fresh38 = len;
    len = len + 1;
    (*p).format_buffer[fresh38 as usize] = ' ' as i32 as ::core::ffi::c_char;
    len += p_dtoa(
        font_scale,
        if ((*p).unit.precision + 1 as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
            (*p).unit.precision + 1 as ::core::ffi::c_int
        } else {
            8 as ::core::ffi::c_int
        },
        (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
    );
    let fresh39 = len;
    len = len + 1;
    (*p).format_buffer[fresh39 as usize] = ' ' as i32 as ::core::ffi::c_char;
    let fresh40 = len;
    len = len + 1;
    (*p).format_buffer[fresh40 as usize] = 'T' as i32 as ::core::ffi::c_char;
    let fresh41 = len;
    len = len + 1;
    (*p).format_buffer[fresh41 as usize] = 'f' as i32 as ::core::ffi::c_char;
    dev_out(
        p,
        &raw mut (*p).format_buffer as *mut ::core::ffi::c_char,
        len as size_t,
    );
    if (*font).bold > 0.0f64 || (*font).bold != (*p).text_state.bold_param {
        if (*font).bold <= 0.0f64 {
            len = sprintf(
                &raw mut (*p).format_buffer as *mut ::core::ffi::c_char,
                b" 0 Tr\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            len = sprintf(
                &raw mut (*p).format_buffer as *mut ::core::ffi::c_char,
                b" 2 Tr %.6f w\0" as *const u8 as *const ::core::ffi::c_char,
                (*font).bold,
            );
        }
        dev_out(
            p,
            &raw mut (*p).format_buffer as *mut ::core::ffi::c_char,
            len as size_t,
        );
    }
    (*p).text_state.bold_param = (*font).bold;
    (*p).text_state.font_id = font_id;
    return 0 as ::core::ffi::c_int;
}
static mut sbuf0: [::core::ffi::c_uchar; 4096] = [0; 4096];
unsafe extern "C-unwind" fn handle_multibyte_string(
    mut font: *mut dev_font,
    mut str_ptr: *mut *const ::core::ffi::c_uchar,
    mut str_len: *mut size_t,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: size_t = 0;
    p = *str_ptr;
    length = *str_len;
    if (*font).enc_id >= 0 as ::core::ffi::c_int {
        let mut inbuf: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
        let mut outbuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut inbytesleft: size_t = 0;
        let mut outbytesleft: size_t = 0;
        let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
        cmap = CMap_cache_get((*font).enc_id);
        inbuf = p;
        outbuf = &raw mut sbuf0 as *mut ::core::ffi::c_uchar;
        inbytesleft = length;
        outbytesleft = FORMAT_BUF_SIZE as size_t;
        CMap_decode(
            cmap,
            &raw mut inbuf,
            &raw mut inbytesleft,
            &raw mut outbuf,
            &raw mut outbytesleft,
        );
        if inbytesleft != 0 as size_t {
            pdf_warning!(
                b"CMap conversion failed. (%zu bytes remains)\0" as *const u8
                    as *const ::core::ffi::c_char,
                inbytesleft,
            );
            return -(1 as ::core::ffi::c_int);
        }
        length = (FORMAT_BUF_SIZE as size_t).wrapping_sub(outbytesleft);
        p = &raw mut sbuf0 as *mut ::core::ffi::c_uchar;
    }
    *str_ptr = p;
    *str_len = length;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn dev_set_string(
    mut p: *mut pdf_dev,
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut instr_ptr: *const ::core::ffi::c_void,
    mut instr_len: size_t,
    mut width: spt_t,
    mut font_id: ::core::ffi::c_int,
    mut resolved: ::core::ffi::c_int,
) {
    let mut font: *mut dev_font = ::core::ptr::null_mut::<dev_font>();
    let mut str_ptr: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut length: size_t = 0;
    let mut i: size_t = 0;
    let mut len: size_t = 0 as size_t;
    let mut kern: spt_t = 0;
    let mut delh: spt_t = 0;
    let mut delv: spt_t = 0;
    let mut text_xorigin: spt_t = 0;
    let mut text_yorigin: spt_t = 0;
    let mut recording: ::core::ffi::c_int = (*p).recording_page;
    if font_id < 0 as ::core::ffi::c_int || font_id >= (*p).num_dev_fonts {
        pdf_abort!(
            b"Invalid font: %d (%d)\0" as *const u8 as *const ::core::ffi::c_char,
            font_id,
            (*p).num_dev_fonts,
        );
    }
    if recording == 0 && font_id != (*p).text_state.font_id {
        pdf_dev_set_font(p, font_id);
    }
    font = (*p).fonts.offset(font_id as isize) as *mut dev_font;
    if font.is_null() {
        pdf_abort!(b"Currentfont not set.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    text_xorigin = (*p).text_state.ref_x;
    text_yorigin = (*p).text_state.ref_y;
    str_ptr = instr_ptr as *const ::core::ffi::c_uchar;
    length = instr_len;
    if resolved == 0 && (*font).format == PDF_FONTTYPE_COMPOSITE {
        if handle_multibyte_string(font, &raw mut str_ptr, &raw mut length)
            < 0 as ::core::ffi::c_int
        {
            pdf_abort!(
                b"Error in converting input string...\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if !(*font).used_chars.is_null() {
            i = 0 as size_t;
            while i < length {
                let mut cid: ::core::ffi::c_ushort =
                    ((*str_ptr.offset(i as isize) as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                        | *str_ptr.offset(i.wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_int) as ::core::ffi::c_ushort;
                let ref mut fresh16 = *(*font)
                    .used_chars
                    .offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
                *fresh16 = (*fresh16 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int)
                        << 7 as ::core::ffi::c_int
                            - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_char;
                i = i.wrapping_add(2 as size_t);
            }
        }
    } else if resolved == 0 {
        if !(*font).used_chars.is_null() {
            i = 0 as size_t;
            while i < length {
                *(*font)
                    .used_chars
                    .offset(*str_ptr.offset(i as isize) as isize) = 1 as ::core::ffi::c_char;
                i = i.wrapping_add(1);
            }
        }
    }
    if recording != 0 {
        let mut snapshot: tt_pdf_render_font = tt_pdf_render_font {
            name: [0; 16],
            size: 0,
            format: 0,
            wmode: 0,
            extend: 0.,
            slant: 0.,
            bold: 0.,
        };
        if (*font).resource.is_null() {
            (*font).resource = pdf_get_font_reference((*font).font_id);
            (*font).used_chars = pdf_get_font_usedchars((*font).font_id);
            if !(*font).used_chars.is_null() {
                if (*font).format == PDF_FONTTYPE_COMPOSITE {
                    i = 0 as size_t;
                    while i < length {
                        let ref mut fresh17 = *(*font).used_chars.offset(
                            (((*str_ptr.offset(i as isize) as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int
                                | *str_ptr.offset(i.wrapping_add(1 as size_t) as isize)
                                    as ::core::ffi::c_int)
                                / 8 as ::core::ffi::c_int) as isize,
                        );
                        *fresh17 = (*fresh17 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 7 as ::core::ffi::c_int
                                    - ((*str_ptr.offset(i as isize) as ::core::ffi::c_int)
                                        << 8 as ::core::ffi::c_int
                                        | *str_ptr.offset(i.wrapping_add(1 as size_t) as isize)
                                            as ::core::ffi::c_int)
                                        % 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                        i = i.wrapping_add(2 as size_t);
                    }
                } else {
                    i = 0 as size_t;
                    while i < length {
                        *(*font)
                            .used_chars
                            .offset(*str_ptr.offset(i as isize) as isize) =
                            1 as ::core::ffi::c_char;
                        i = i.wrapping_add(1);
                    }
                }
            }
        }
        if (*font).used_on_this_page == 0 {
            pdf_doc_add_page_resource(
                b"Font\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*font).short_name as *mut ::core::ffi::c_char,
                pdf_link_obj((*font).resource),
            );
            (*font).used_on_this_page = 1 as ::core::ffi::c_int;
        }
        memcpy(
            &raw mut snapshot.name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw mut (*font).short_name as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
        );
        snapshot.size = (*font).sptsize as int32_t;
        snapshot.format = (*font).format;
        snapshot.wmode = (*font).wmode;
        snapshot.extend = (*font).extend;
        snapshot.slant = (*font).slant;
        snapshot.bold = (*font).bold;
        crate::pipeline::page_text(
            &snapshot,
            font_id,
            xpos as int32_t,
            ypos as int32_t,
            width as int32_t,
            core::slice::from_raw_parts(str_ptr, length),
        );
        (*p).text_state.font_id = font_id;
        return;
    }
    if (*p).text_state.dir_mode == 0 as ::core::ffi::c_int {
        delh = text_xorigin + (*p).text_state.offset - xpos;
        delv = ypos - text_yorigin;
    } else if (*p).text_state.dir_mode == 1 as ::core::ffi::c_int {
        delh = ypos - text_yorigin + (*p).text_state.offset;
        delv = xpos - text_xorigin;
    } else {
        delh = ypos + text_yorigin + (*p).text_state.offset;
        delv = xpos + text_xorigin;
    }
    if (*p).text_state.force_reset != 0
        || labs(delv as ::core::ffi::c_long) > (*p).unit.min_bp_val as ::core::ffi::c_long
        || labs(delh as ::core::ffi::c_long)
            > (3.0f64 * (*font).extend * (*font).sptsize as ::core::ffi::c_double) as spt_t
                as ::core::ffi::c_long
    {
        pdf_dev_text_mode(p);
        kern = 0 as ::core::ffi::c_int as spt_t;
    } else {
        kern = (1000.0f64 / (*font).extend * delh as ::core::ffi::c_double
            / (*font).sptsize as ::core::ffi::c_double) as spt_t;
    }
    if (*p).motion_state != STRING_MODE {
        pdf_dev_string_mode(
            p,
            xpos,
            ypos,
            (*font).slant,
            (*font).extend,
            (*p).text_state.matrix.rotate,
        );
    } else if kern != 0 as ::core::ffi::c_int {
        (*p).text_state.offset -= (kern as ::core::ffi::c_double
            * (*font).extend
            * ((*font).sptsize as ::core::ffi::c_double / 1000.0f64))
            as spt_t;
        let fresh18 = len;
        len = len.wrapping_add(1);
        (*p).format_buffer[fresh18 as usize] = (if (*p).text_state.is_mb != 0 {
            '>' as i32
        } else {
            ')' as i32
        }) as ::core::ffi::c_char;
        if (*font).wmode != 0 {
            len = len.wrapping_add(p_itoa(
                -(kern as ::core::ffi::c_int),
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
            ) as size_t);
        } else {
            len = len.wrapping_add(p_itoa(
                kern as ::core::ffi::c_int,
                (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
            ) as size_t);
        }
        let fresh19 = len;
        len = len.wrapping_add(1);
        (*p).format_buffer[fresh19 as usize] = (if (*p).text_state.is_mb != 0 {
            '<' as i32
        } else {
            '(' as i32
        }) as ::core::ffi::c_char;
        dev_out(
            p,
            &raw mut (*p).format_buffer as *mut ::core::ffi::c_char,
            len,
        );
        len = 0 as size_t;
    }
    if (*p).text_state.is_mb != 0 {
        if (FORMAT_BUF_SIZE as size_t).wrapping_sub(len) < (2 as size_t).wrapping_mul(length) {
            pdf_abort!(b"Buffer overflow...\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        i = 0 as size_t;
        while i < length {
            let mut first: ::core::ffi::c_int = 0;
            let mut second: ::core::ffi::c_int = 0;
            first = *str_ptr.offset(i as isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int;
            second = *str_ptr.offset(i as isize) as ::core::ffi::c_int & 0xf as ::core::ffi::c_int;
            let fresh20 = len;
            len = len.wrapping_add(1);
            (*p).format_buffer[fresh20 as usize] = (if first >= 10 as ::core::ffi::c_int {
                first + 'W' as i32
            } else {
                first + '0' as i32
            }) as ::core::ffi::c_char;
            let fresh21 = len;
            len = len.wrapping_add(1);
            (*p).format_buffer[fresh21 as usize] = (if second >= 10 as ::core::ffi::c_int {
                second + 'W' as i32
            } else {
                second + '0' as i32
            }) as ::core::ffi::c_char;
            i = i.wrapping_add(1);
        }
    } else {
        len = len.wrapping_add(pdfobj_escape_str(
            (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
            (FORMAT_BUF_SIZE as size_t).wrapping_sub(len),
            str_ptr,
            length,
        ));
    }
    dev_out(
        p,
        &raw mut (*p).format_buffer as *mut ::core::ffi::c_char,
        len,
    );
    (*p).text_state.offset += width;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_set_string")]
pub unsafe extern "C-unwind" fn pdf_dev_set_string(
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut input: *const ::core::ffi::c_void,
    mut length: size_t,
    mut width: spt_t,
    mut font_id: ::core::ffi::c_int,
) {
    dev_set_string(
        current_device(),
        xpos,
        ypos,
        input,
        length,
        width,
        font_id,
        0 as ::core::ffi::c_int,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_init_device")]
pub unsafe extern "C-unwind" fn pdf_init_device(
    mut dvi2pts: ::core::ffi::c_double,
    mut precision: ::core::ffi::c_int,
    mut black_and_white: ::core::ffi::c_int,
) {
    let mut p: *mut pdf_dev = current_device();
    (*p).motion_state = GRAPHICS_MODE;
    (*p).render_output = NULL;
    (*p).recording_page = 0 as ::core::ffi::c_int;
    (*p).unit.dvi2pts = 0.0f64;
    (*p).unit.min_bp_val = 658 as ::core::ffi::c_int;
    (*p).unit.precision = 2 as ::core::ffi::c_int;
    (*p).param.autorotate = 1 as ::core::ffi::c_int;
    (*p).param.colormode = 1 as ::core::ffi::c_int;
    (*p).text_state.font_id = -(1 as ::core::ffi::c_int);
    (*p).text_state.offset = 0 as ::core::ffi::c_int as spt_t;
    (*p).text_state.matrix.slant = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*p).text_state.matrix.extend = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*p).text_state.matrix.rotate = 0 as ::core::ffi::c_int;
    (*p).text_state.bold_param = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*p).text_state.dir_mode = 0 as ::core::ffi::c_int;
    (*p).text_state.force_reset = 0 as ::core::ffi::c_int;
    (*p).text_state.is_mb = 0 as ::core::ffi::c_int;
    if precision < 0 as ::core::ffi::c_int || precision > DEV_PRECISION_MAX {
        pdf_warning!(
            b"Number of decimal digits out of range [0-%d].\0" as *const u8
                as *const ::core::ffi::c_char,
            DEV_PRECISION_MAX,
        );
    }
    if precision < 0 as ::core::ffi::c_int {
        (*p).unit.precision = 0 as ::core::ffi::c_int;
    } else if precision > DEV_PRECISION_MAX {
        (*p).unit.precision = DEV_PRECISION_MAX;
    } else {
        (*p).unit.precision = precision;
    }
    (*p).unit.dvi2pts = dvi2pts;
    (*p).unit.min_bp_val = (floor(
        1.0f64
            / (ten_pow[(*p).unit.precision as usize] as ::core::ffi::c_double * dvi2pts)
            / 1 as ::core::ffi::c_int as ::core::ffi::c_double
            + 0.5f64,
    ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double)
        as ::core::ffi::c_int;
    if (*p).unit.min_bp_val < 0 as ::core::ffi::c_int {
        (*p).unit.min_bp_val = -(*p).unit.min_bp_val;
    }
    (*p).param.colormode = if black_and_white != 0 {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    pdf_dev_graphics_mode(p);
    pdf_color_clear_stack();
    pdf_dev_init_gstates();
    (*p).max_dev_fonts = 0 as ::core::ffi::c_int;
    (*p).num_dev_fonts = (*p).max_dev_fonts;
    (*p).fonts = ::core::ptr::null_mut::<dev_font>();
}
#[unsafe(export_name = "__pdf_impl_pdf_close_device")]
pub unsafe extern "C-unwind" fn pdf_close_device() {
    let mut p: *mut pdf_dev = current_device();
    if !(*p).fonts.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*p).num_dev_fonts {
            free((*(*p).fonts.offset(i as isize)).tex_name as *mut ::core::ffi::c_void);
            pdf_release_obj((*(*p).fonts.offset(i as isize)).resource);
            let ref mut fresh14 = (*(*p).fonts.offset(i as isize)).tex_name;
            *fresh14 = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let ref mut fresh15 = (*(*p).fonts.offset(i as isize)).resource;
            *fresh15 = ::core::ptr::null_mut::<pdf_obj>();
            i += 1;
        }
        free((*p).fonts as *mut ::core::ffi::c_void);
        (*p).fonts = ::core::ptr::null_mut::<dev_font>();
    }
    pdf_dev_clear_gstates();
}
unsafe extern "C-unwind" fn dev_reset_fonts(mut p: *mut pdf_dev, mut newpage: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*p).num_dev_fonts {
        (*(*p).fonts.offset(i as isize)).used_on_this_page = 0 as ::core::ffi::c_int;
        i += 1;
    }
    (*p).text_state.font_id = -(1 as ::core::ffi::c_int);
    (*p).text_state.matrix.slant = 0.0f64;
    (*p).text_state.matrix.extend = 1.0f64;
    (*p).text_state.matrix.rotate = TEXT_WMODE_HH;
    if newpage != 0 {
        (*p).text_state.bold_param = 0.0f64;
    }
    (*p).text_state.is_mb = 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_reset_fonts")]
pub unsafe extern "C-unwind" fn pdf_dev_reset_fonts(mut newpage: ::core::ffi::c_int) {
    if pdev.recording_page != 0 {
        crate::pipeline::page_command(
            TT_PDF_RESET_FONTS as ::core::ffi::c_int,
            newpage as int32_t,
            0 as int32_t,
            0 as int32_t,
            0 as int32_t,
        );
    }
    dev_reset_fonts(current_device(), newpage);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_reset_color")]
pub unsafe extern "C-unwind" fn pdf_dev_reset_color(mut force: ::core::ffi::c_int) {
    let mut sc: *mut pdf_color = ::core::ptr::null_mut::<pdf_color>();
    let mut fc: *mut pdf_color = ::core::ptr::null_mut::<pdf_color>();
    pdf_color_get_current(&raw mut sc, &raw mut fc);
    pdf_dev_set_color(sc, 0 as ::core::ffi::c_char, force);
    pdf_dev_set_color(fc, 0x20 as ::core::ffi::c_char, force);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_bop")]
pub unsafe extern "C-unwind" fn pdf_dev_bop(mut M: *const pdf_tmatrix) {
    let mut p: *mut pdf_dev = current_device();
    pdf_dev_graphics_mode(p);
    (*p).text_state.force_reset = 0 as ::core::ffi::c_int;
    pdf_dev_gsave();
    pdf_dev_concat(M);
    pdf_dev_reset_fonts(1 as ::core::ffi::c_int);
    pdf_dev_reset_color(0 as ::core::ffi::c_int);
    pdf_dev_reset_xgstate(0 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_eop")]
pub unsafe extern "C-unwind" fn pdf_dev_eop() {
    let mut p: *mut pdf_dev = current_device();
    let mut depth: ::core::ffi::c_int = 0;
    pdf_dev_graphics_mode(p);
    depth = pdf_dev_current_depth();
    if depth != 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"Unbalenced q/Q nesting...: %d\0" as *const u8 as *const ::core::ffi::c_char,
            depth,
        );
        pdf_dev_grestore_to(0 as ::core::ffi::c_int);
    } else {
        pdf_dev_grestore();
    };
}
unsafe extern "C-unwind" fn print_fontmap(
    mut font_name: *const ::core::ffi::c_char,
    mut mrec: *mut fontmap_rec,
) {
    if mrec.is_null() {
        return;
    }
    pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdf_message!(
        b"fontmap: %s -> %s\0" as *const u8 as *const ::core::ffi::c_char,
        font_name,
        (*mrec).font_name,
    );
    if !(*mrec).enc_name.is_null() {
        pdf_message!(
            b"(%s)\0" as *const u8 as *const ::core::ffi::c_char,
            (*mrec).enc_name,
        );
    }
    if (*mrec).opt.extend != 1.0f64 {
        pdf_message!(
            b"[extend:%g]\0" as *const u8 as *const ::core::ffi::c_char,
            (*mrec).opt.extend,
        );
    }
    if (*mrec).opt.slant != 0.0f64 {
        pdf_message!(
            b"[slant:%g]\0" as *const u8 as *const ::core::ffi::c_char,
            (*mrec).opt.slant,
        );
    }
    if (*mrec).opt.bold != 0.0f64 {
        pdf_message!(
            b"[bold:%g]\0" as *const u8 as *const ::core::ffi::c_char,
            (*mrec).opt.bold,
        );
    }
    if (*mrec).opt.flags & FONTMAP_OPT_NOEMBED != 0 {
        pdf_message!(b"[noemb]\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*mrec).opt.mapc >= 0 as ::core::ffi::c_int {
        pdf_message!(
            b"[map:<%02x>]\0" as *const u8 as *const ::core::ffi::c_char,
            (*mrec).opt.mapc,
        );
    }
    if !(*mrec).opt.charcoll.is_null() {
        pdf_message!(
            b"[csi:%s]\0" as *const u8 as *const ::core::ffi::c_char,
            (*mrec).opt.charcoll,
        );
    }
    if (*mrec).opt.index != 0 {
        pdf_message!(
            b"[index:%d]\0" as *const u8 as *const ::core::ffi::c_char,
            (*mrec).opt.index,
        );
    }
    match (*mrec).opt.style {
        FONTMAP_STYLE_BOLD => {
            pdf_message!(b"[style:bold]\0" as *const u8 as *const ::core::ffi::c_char);
        }
        FONTMAP_STYLE_ITALIC => {
            pdf_message!(b"[style:italic]\0" as *const u8 as *const ::core::ffi::c_char);
        }
        FONTMAP_STYLE_BOLDITALIC => {
            pdf_message!(b"[style:bolditalic]\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        _ => {}
    }
    pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_locate_font")]
pub unsafe extern "C-unwind" fn pdf_dev_locate_font(
    mut font_name: *const ::core::ffi::c_char,
    mut ptsize: spt_t,
) -> ::core::ffi::c_int {
    let mut p: *mut pdf_dev = current_device();
    let mut i: ::core::ffi::c_int = 0;
    let mut mrec: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    let mut font: *mut dev_font = ::core::ptr::null_mut::<dev_font>();
    if font_name.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if ptsize == 0 as ::core::ffi::c_int {
        pdf_abort!(
            b"pdf_dev_locate_font() called with the zero ptsize.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*p).num_dev_fonts {
        if streq_ptr(font_name, (*(*p).fonts.offset(i as isize)).tex_name) as ::core::ffi::c_int
            != 0
            && ptsize == (*(*p).fonts.offset(i as isize)).sptsize
        {
            return i;
        }
        i += 1;
    }
    if (*p).num_dev_fonts >= (*p).max_dev_fonts {
        (*p).max_dev_fonts += 16 as ::core::ffi::c_int;
        (*p).fonts = renew(
            (*p).fonts as *mut ::core::ffi::c_void,
            ((*p).max_dev_fonts as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<dev_font>() as usize) as uint32_t,
        ) as *mut dev_font;
    }
    font = (*p).fonts.offset((*p).num_dev_fonts as isize) as *mut dev_font;
    mrec = pdf_lookup_fontmap_record(font_name);
    if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
        print_fontmap(font_name, mrec);
    }
    (*font).font_id = pdf_font_findresource(
        font_name,
        ptsize as ::core::ffi::c_double * (*p).unit.dvi2pts,
    );
    if (*font).font_id < 0 as ::core::ffi::c_int {
        (*font).font_id = pdf_font_load_font(
            font_name,
            ptsize as ::core::ffi::c_double * (*p).unit.dvi2pts,
            mrec,
        );
        if (*font).font_id < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    pdf_font_resource_name(
        (*font).font_id,
        &raw mut (*font).short_name as *mut ::core::ffi::c_char,
    );
    (*font).used_on_this_page = 0 as ::core::ffi::c_int;
    (*font).tex_name = new(
        (strlen(font_name).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*font).tex_name, font_name);
    (*font).sptsize = ptsize;
    match pdf_get_font_subtype((*font).font_id) {
        PDF_FONT_FONTTYPE_TYPE3 => {
            (*font).format = PDF_FONTTYPE_BITMAP;
        }
        PDF_FONT_FONTTYPE_TYPE0 => {
            (*font).format = PDF_FONTTYPE_COMPOSITE;
        }
        _ => {
            (*font).format = PDF_FONTTYPE_SIMPLE;
        }
    }
    (*font).wmode = pdf_get_font_wmode((*font).font_id);
    (*font).enc_id = pdf_get_font_encoding((*font).font_id);
    (*font).resource = ::core::ptr::null_mut::<pdf_obj>();
    (*font).used_chars = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).extend = 1.0f64;
    (*font).slant = 0.0f64;
    (*font).bold = 0.0f64;
    if !mrec.is_null() {
        (*font).extend = (*mrec).opt.extend;
        (*font).slant = (*mrec).opt.slant;
        (*font).bold = (*mrec).opt.bold;
    }
    let fresh64 = (*p).num_dev_fonts;
    (*p).num_dev_fonts = (*p).num_dev_fonts + 1;
    return fresh64;
}
unsafe extern "C-unwind" fn dev_sprint_line(
    mut p: *mut pdf_dev,
    mut buf: *mut ::core::ffi::c_char,
    mut width: spt_t,
    mut p0_x: spt_t,
    mut p0_y: spt_t,
    mut p1_x: spt_t,
    mut p1_y: spt_t,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut w: ::core::ffi::c_double = 0.;
    w = width as ::core::ffi::c_double * (*p).unit.dvi2pts;
    len += p_dtoa(
        w,
        if ((*p).unit.precision + 1 as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
            (*p).unit.precision + 1 as ::core::ffi::c_int
        } else {
            8 as ::core::ffi::c_int
        },
        buf.offset(len as isize),
    );
    let fresh52 = len;
    len = len + 1;
    *buf.offset(fresh52 as isize) = ' ' as i32 as ::core::ffi::c_char;
    let fresh53 = len;
    len = len + 1;
    *buf.offset(fresh53 as isize) = 'w' as i32 as ::core::ffi::c_char;
    let fresh54 = len;
    len = len + 1;
    *buf.offset(fresh54 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len += dev_sprint_bp(
        p,
        buf.offset(len as isize),
        p0_x,
        ::core::ptr::null_mut::<spt_t>(),
    );
    let fresh55 = len;
    len = len + 1;
    *buf.offset(fresh55 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len += dev_sprint_bp(
        p,
        buf.offset(len as isize),
        p0_y,
        ::core::ptr::null_mut::<spt_t>(),
    );
    let fresh56 = len;
    len = len + 1;
    *buf.offset(fresh56 as isize) = ' ' as i32 as ::core::ffi::c_char;
    let fresh57 = len;
    len = len + 1;
    *buf.offset(fresh57 as isize) = 'm' as i32 as ::core::ffi::c_char;
    let fresh58 = len;
    len = len + 1;
    *buf.offset(fresh58 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len += dev_sprint_bp(
        p,
        buf.offset(len as isize),
        p1_x,
        ::core::ptr::null_mut::<spt_t>(),
    );
    let fresh59 = len;
    len = len + 1;
    *buf.offset(fresh59 as isize) = ' ' as i32 as ::core::ffi::c_char;
    len += dev_sprint_bp(
        p,
        buf.offset(len as isize),
        p1_y,
        ::core::ptr::null_mut::<spt_t>(),
    );
    let fresh60 = len;
    len = len + 1;
    *buf.offset(fresh60 as isize) = ' ' as i32 as ::core::ffi::c_char;
    let fresh61 = len;
    len = len + 1;
    *buf.offset(fresh61 as isize) = 'l' as i32 as ::core::ffi::c_char;
    let fresh62 = len;
    len = len + 1;
    *buf.offset(fresh62 as isize) = ' ' as i32 as ::core::ffi::c_char;
    let fresh63 = len;
    len = len + 1;
    *buf.offset(fresh63 as isize) = 'S' as i32 as ::core::ffi::c_char;
    return len;
}
pub const PDF_LINE_THICKNESS_MAX: ::core::ffi::c_double = 5.0f64;
unsafe extern "C-unwind" fn dev_set_rule(
    mut p: *mut pdf_dev,
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut width: spt_t,
    mut height: spt_t,
) {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut width_in_bp: ::core::ffi::c_double = 0.;
    pdf_dev_graphics_mode(p);
    let fresh42 = len;
    len = len + 1;
    (*p).format_buffer[fresh42 as usize] = ' ' as i32 as ::core::ffi::c_char;
    let fresh43 = len;
    len = len + 1;
    (*p).format_buffer[fresh43 as usize] = 'q' as i32 as ::core::ffi::c_char;
    let fresh44 = len;
    len = len + 1;
    (*p).format_buffer[fresh44 as usize] = ' ' as i32 as ::core::ffi::c_char;
    width_in_bp =
        (if width < height { width } else { height }) as ::core::ffi::c_double * (*p).unit.dvi2pts;
    if width_in_bp < 0.0f64 || width_in_bp > PDF_LINE_THICKNESS_MAX {
        let mut rect: pdf_rect = pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        };
        rect.llx = (*p).unit.dvi2pts * xpos as ::core::ffi::c_double;
        rect.lly = (*p).unit.dvi2pts * ypos as ::core::ffi::c_double;
        rect.urx = (*p).unit.dvi2pts * width as ::core::ffi::c_double;
        rect.ury = (*p).unit.dvi2pts * height as ::core::ffi::c_double;
        len = (len as size_t).wrapping_add(pdf_dev_sprint_rect(
            p,
            (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
            &raw mut rect,
        )) as ::core::ffi::c_int as ::core::ffi::c_int;
        let fresh45 = len;
        len = len + 1;
        (*p).format_buffer[fresh45 as usize] = ' ' as i32 as ::core::ffi::c_char;
        let fresh46 = len;
        len = len + 1;
        (*p).format_buffer[fresh46 as usize] = 'r' as i32 as ::core::ffi::c_char;
        let fresh47 = len;
        len = len + 1;
        (*p).format_buffer[fresh47 as usize] = 'e' as i32 as ::core::ffi::c_char;
        let fresh48 = len;
        len = len + 1;
        (*p).format_buffer[fresh48 as usize] = ' ' as i32 as ::core::ffi::c_char;
        let fresh49 = len;
        len = len + 1;
        (*p).format_buffer[fresh49 as usize] = 'f' as i32 as ::core::ffi::c_char;
    } else if width > height {
        if (*p).render_output.is_null() && height < (*p).unit.min_bp_val {
            pdf_warning!(
                b"Too thin line: height=%d (%g bp)\0" as *const u8 as *const ::core::ffi::c_char,
                height,
                width_in_bp,
            );
            pdf_warning!(
                b"Please consider using \"-d\" option.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        len += dev_sprint_line(
            p,
            (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
            height,
            xpos,
            ypos + height / 2 as spt_t,
            xpos + width,
            ypos + height / 2 as spt_t,
        );
    } else {
        if (*p).render_output.is_null() && width < (*p).unit.min_bp_val {
            pdf_warning!(
                b"Too thin line: width=%d (%g bp)\0" as *const u8 as *const ::core::ffi::c_char,
                width,
                width_in_bp,
            );
            pdf_warning!(
                b"Please consider using \"-d\" option.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        len += dev_sprint_line(
            p,
            (&raw mut (*p).format_buffer as *mut ::core::ffi::c_char).offset(len as isize),
            width,
            xpos + width / 2 as spt_t,
            ypos,
            xpos + width / 2 as spt_t,
            ypos + height,
        );
    }
    let fresh50 = len;
    len = len + 1;
    (*p).format_buffer[fresh50 as usize] = ' ' as i32 as ::core::ffi::c_char;
    let fresh51 = len;
    len = len + 1;
    (*p).format_buffer[fresh51 as usize] = 'Q' as i32 as ::core::ffi::c_char;
    dev_out(
        p,
        &raw mut (*p).format_buffer as *mut ::core::ffi::c_char,
        len as size_t,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_set_rule")]
pub unsafe extern "C-unwind" fn pdf_dev_set_rule(
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut width: spt_t,
    mut height: spt_t,
) {
    let mut p: *mut pdf_dev = current_device();
    if (*p).recording_page != 0 {
        let mut thickness: ::core::ffi::c_double = (if width < height { width } else { height })
            as ::core::ffi::c_double
            * (*p).unit.dvi2pts;
        if thickness >= 0.0f64
            && thickness <= PDF_LINE_THICKNESS_MAX
            && (if width < height { width } else { height }) < (*p).unit.min_bp_val
        {
            pdf_warning!(
                b"Too thin line: %s=%d (%g bp)\0" as *const u8 as *const ::core::ffi::c_char,
                if width > height {
                    b"height\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"width\0" as *const u8 as *const ::core::ffi::c_char
                },
                if width < height { width } else { height },
                thickness,
            );
            pdf_warning!(
                b"Please consider using \"-d\" option.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        crate::pipeline::page_command(
            TT_PDF_RULE as ::core::ffi::c_int,
            xpos as int32_t,
            ypos as int32_t,
            width as int32_t,
            height as int32_t,
        );
        (*p).motion_state = GRAPHICS_MODE;
        (*p).text_state.font_id = -(1 as ::core::ffi::c_int);
        (*p).text_state.bold_param = 0.0f64;
        (*p).text_state.force_reset = 0 as ::core::ffi::c_int;
        return;
    }
    dev_set_rule(p, xpos, ypos, width, height);
}
pub(crate) unsafe fn pdf_dev_set_rect(
    mut rect: *mut pdf_rect,
    mut x_user: spt_t,
    mut y_user: spt_t,
    mut width: spt_t,
    mut height: spt_t,
    mut depth: spt_t,
) {
    let mut p: *mut pdf_dev = current_device();
    let mut dev_x: ::core::ffi::c_double = 0.;
    let mut dev_y: ::core::ffi::c_double = 0.;
    let mut p0: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p3: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut min_x: ::core::ffi::c_double = 0.;
    let mut min_y: ::core::ffi::c_double = 0.;
    let mut max_x: ::core::ffi::c_double = 0.;
    let mut max_y: ::core::ffi::c_double = 0.;
    dev_x = x_user as ::core::ffi::c_double * (*p).unit.dvi2pts;
    dev_y = y_user as ::core::ffi::c_double * (*p).unit.dvi2pts;
    if (*p).text_state.dir_mode != 0 {
        p0.x = dev_x - (*p).unit.dvi2pts * depth as ::core::ffi::c_double;
        p0.y = dev_y - (*p).unit.dvi2pts * width as ::core::ffi::c_double;
        p1.x = dev_x + (*p).unit.dvi2pts * height as ::core::ffi::c_double;
        p1.y = p0.y;
        p2.x = p1.x;
        p2.y = dev_y;
        p3.x = p0.x;
        p3.y = p2.y;
    } else {
        p0.x = dev_x;
        p0.y = dev_y - (*p).unit.dvi2pts * depth as ::core::ffi::c_double;
        p1.x = dev_x + (*p).unit.dvi2pts * width as ::core::ffi::c_double;
        p1.y = p0.y;
        p2.x = p1.x;
        p2.y = dev_y + (*p).unit.dvi2pts * height as ::core::ffi::c_double;
        p3.x = p0.x;
        p3.y = p2.y;
    }
    pdf_dev_transform(&raw mut p0, ::core::ptr::null::<pdf_tmatrix>());
    pdf_dev_transform(&raw mut p1, ::core::ptr::null::<pdf_tmatrix>());
    pdf_dev_transform(&raw mut p2, ::core::ptr::null::<pdf_tmatrix>());
    pdf_dev_transform(&raw mut p3, ::core::ptr::null::<pdf_tmatrix>());
    min_x = if p0.x < p1.x { p0.x } else { p1.x };
    min_x = if min_x < p2.x { min_x } else { p2.x };
    min_x = if min_x < p3.x { min_x } else { p3.x };
    max_x = if p0.x > p1.x { p0.x } else { p1.x };
    max_x = if max_x > p2.x { max_x } else { p2.x };
    max_x = if max_x > p3.x { max_x } else { p3.x };
    min_y = if p0.y < p1.y { p0.y } else { p1.y };
    min_y = if min_y < p2.y { min_y } else { p2.y };
    min_y = if min_y < p3.y { min_y } else { p3.y };
    max_y = if p0.y > p1.y { p0.y } else { p1.y };
    max_y = if max_y > p2.y { max_y } else { p2.y };
    max_y = if max_y > p3.y { max_y } else { p3.y };
    (*rect).llx = min_x;
    (*rect).lly = min_y;
    (*rect).urx = max_x;
    (*rect).ury = max_y;
}
pub(crate) unsafe fn pdf_dev_get_dirmode() -> ::core::ffi::c_int {
    let mut p: *mut pdf_dev = current_device();
    return (*p).text_state.dir_mode;
}
unsafe extern "C-unwind" fn dev_set_dirmode(mut p: *mut pdf_dev, mut text_dir: ::core::ffi::c_int) {
    let mut font: *mut dev_font = ::core::ptr::null_mut::<dev_font>();
    let mut text_rotate: ::core::ffi::c_int = 0;
    let mut vert_dir: ::core::ffi::c_int = 0;
    let mut vert_font: ::core::ffi::c_int = 0;
    font = if (*p).text_state.font_id < 0 as ::core::ffi::c_int {
        ::core::ptr::null_mut::<dev_font>()
    } else {
        (*p).fonts.offset((*p).text_state.font_id as isize) as *mut dev_font
    };
    vert_font = if !font.is_null() && (*font).wmode != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if (*p).param.autorotate != 0 {
        vert_dir = text_dir;
    } else {
        vert_dir = vert_font;
    }
    text_rotate = vert_font << 2 as ::core::ffi::c_int | vert_dir;
    if !font.is_null()
        && (if abs(text_rotate - (*p).text_state.matrix.rotate) % 5 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) != 0
    {
        (*p).text_state.force_reset = 1 as ::core::ffi::c_int;
    }
    (*p).text_state.matrix.rotate = text_rotate;
    (*p).text_state.dir_mode = text_dir;
}
pub(crate) unsafe fn pdf_dev_set_dirmode(mut text_dir: ::core::ffi::c_int) {
    if pdev.recording_page != 0 && text_dir != pdev.text_state.dir_mode {
        crate::pipeline::page_command(
            TT_PDF_DIRECTION as ::core::ffi::c_int,
            text_dir as int32_t,
            0 as int32_t,
            0 as int32_t,
            0 as int32_t,
        );
    }
    dev_set_dirmode(current_device(), text_dir);
}
unsafe extern "C-unwind" fn dev_set_param_autorotate(
    mut p: *mut pdf_dev,
    mut auto_rotate: ::core::ffi::c_int,
) {
    let mut font: *mut dev_font = ::core::ptr::null_mut::<dev_font>();
    let mut text_rotate: ::core::ffi::c_int = 0;
    let mut vert_font: ::core::ffi::c_int = 0;
    let mut vert_dir: ::core::ffi::c_int = 0;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    font = if (*p).text_state.font_id < 0 as ::core::ffi::c_int {
        ::core::ptr::null_mut::<dev_font>()
    } else {
        (*p).fonts.offset((*p).text_state.font_id as isize) as *mut dev_font
    };
    vert_font = if !font.is_null() && (*font).wmode != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    if auto_rotate != 0 {
        vert_dir = (*p).text_state.dir_mode;
    } else {
        vert_dir = vert_font;
    }
    text_rotate = vert_font << 2 as ::core::ffi::c_int | vert_dir;
    if if abs(text_rotate - (*p).text_state.matrix.rotate) % 5 as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    } != 0
    {
        (*p).text_state.force_reset = 1 as ::core::ffi::c_int;
    }
    (*p).text_state.matrix.rotate = text_rotate;
    (*p).param.autorotate = auto_rotate;
}
pub(crate) unsafe fn pdf_dev_get_param(mut param_type: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut p: *mut pdf_dev = current_device();
    let mut value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    match param_type {
        PDF_DEV_PARAM_AUTOROTATE => {
            value = (*p).param.autorotate;
        }
        PDF_DEV_PARAM_COLORMODE => {
            value = (*p).param.colormode;
        }
        _ => {
            pdf_abort!(
                b"Unknown device parameter: %d\0" as *const u8 as *const ::core::ffi::c_char,
                param_type,
            );
        }
    }
    return value;
}
pub(crate) unsafe fn pdf_dev_set_param(
    mut param_type: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) {
    let mut p: *mut pdf_dev = current_device();
    match param_type {
        PDF_DEV_PARAM_AUTOROTATE => {
            dev_set_param_autorotate(p, value);
        }
        PDF_DEV_PARAM_COLORMODE => {
            (*p).param.colormode = value;
        }
        _ => {
            pdf_abort!(
                b"Unknown device parameter: %d\0" as *const u8 as *const ::core::ffi::c_char,
                param_type,
            );
        }
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_put_image")]
pub unsafe extern "C-unwind" fn pdf_dev_put_image(
    mut id: ::core::ffi::c_int,
    mut ti: *mut transform_info,
    mut ref_x: ::core::ffi::c_double,
    mut ref_y: ::core::ffi::c_double,
    mut rect: *mut pdf_rect,
) -> ::core::ffi::c_int {
    let mut p: *mut pdf_dev = current_device();
    let mut res_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut M: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut M1: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut r: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    M.a = (*ti).matrix.a;
    M.b = (*ti).matrix.b;
    M.c = (*ti).matrix.c;
    M.d = (*ti).matrix.d;
    M.e = (*ti).matrix.e;
    M.f = (*ti).matrix.f;
    M.e += ref_x;
    M.f += ref_y;
    if (*p).param.autorotate != 0 && (*p).text_state.dir_mode != 0 {
        let mut tmp: ::core::ffi::c_double = 0.;
        tmp = -M.a;
        M.a = M.b;
        M.b = tmp;
        tmp = -M.c;
        M.c = M.d;
        M.d = tmp;
    }
    pdf_dev_graphics_mode(p);
    pdf_dev_gsave();
    pdf_ximage_scale_image(id, &raw mut M1, &raw mut r, ti);
    let mut _tmp_a: ::core::ffi::c_double = 0.;
    let mut _tmp_b: ::core::ffi::c_double = 0.;
    let mut _tmp_c: ::core::ffi::c_double = 0.;
    let mut _tmp_d: ::core::ffi::c_double = 0.;
    _tmp_a = M.a;
    _tmp_b = M.b;
    _tmp_c = M.c;
    _tmp_d = M.d;
    M.a = M1.a * _tmp_a + M1.b * _tmp_c;
    M.b = M1.a * _tmp_b + M1.b * _tmp_d;
    M.c = M1.c * _tmp_a + M1.d * _tmp_c;
    M.d = M1.c * _tmp_b + M1.d * _tmp_d;
    M.e += M1.e * _tmp_a + M1.f * _tmp_c;
    M.f += M1.e * _tmp_b + M1.f * _tmp_d;
    pdf_dev_concat(&raw mut M);
    if (*ti).flags & INFO_DO_CLIP != 0 {
        pdf_dev_rectclip(r.llx, r.lly, r.urx - r.llx, r.ury - r.lly);
    }
    res_name = pdf_ximage_get_resname(id);
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    buf = new(
        (strlen(res_name).wrapping_add(6 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    len = sprintf(
        buf,
        b" /%s Do\0" as *const u8 as *const ::core::ffi::c_char,
        res_name,
    );
    dev_out(p, buf, len as size_t);
    free(buf as *mut ::core::ffi::c_void);
    if !rect.is_null() {
        let mut r1: pdf_rect = pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        };
        pdf_dev_set_rect(
            &raw mut r1,
            round(r.llx / (*p).unit.dvi2pts) as spt_t,
            round(r.lly / (*p).unit.dvi2pts) as spt_t,
            round((r.urx - r.llx) / (*p).unit.dvi2pts) as spt_t,
            round((r.ury - r.lly) / (*p).unit.dvi2pts) as spt_t,
            0 as spt_t,
        );
        (*rect).llx = r1.llx;
        (*rect).lly = r1.lly;
        (*rect).urx = r1.urx;
        (*rect).ury = r1.ury;
    }
    pdf_dev_grestore();
    pdf_doc_add_page_resource(
        b"XObject\0" as *const u8 as *const ::core::ffi::c_char,
        res_name,
        pdf_ximage_get_reference(id),
    );
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn transform_info_clear(mut info: *mut transform_info) {
    (*info).width = 0.0f64;
    (*info).height = 0.0f64;
    (*info).depth = 0.0f64;
    (*info).bbox.llx = 0.0f64;
    (*info).bbox.lly = 0.0f64;
    (*info).bbox.urx = 0.0f64;
    (*info).bbox.ury = 0.0f64;
    (*info).matrix.a = 1.0f64;
    (*info).matrix.b = 0.0f64;
    (*info).matrix.c = 0.0f64;
    (*info).matrix.d = 1.0f64;
    (*info).matrix.e = 0.0f64;
    (*info).matrix.f = 0.0f64;
    (*info).flags = 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_begin_actualtext")]
pub unsafe extern "C-unwind" fn pdf_dev_begin_actualtext(
    mut unicodes: *mut uint16_t,
    mut count: ::core::ffi::c_int,
) {
    let mut p: *mut pdf_dev = current_device();
    let mut len: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut pdf_doc_enc: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < count {
        if *unicodes.offset(i as isize) as ::core::ffi::c_int > 0xff as ::core::ffi::c_int
            || *unicodes.offset(i as isize) as ::core::ffi::c_int > 0x7f as ::core::ffi::c_int
                && (*unicodes.offset(i as isize) as ::core::ffi::c_int) < 0xa1 as ::core::ffi::c_int
        {
            pdf_doc_enc = 0 as ::core::ffi::c_int;
            break;
        } else {
            i += 1;
        }
    }
    pdf_dev_graphics_mode(p);
    dev_out(
        p,
        b"\n/Span << /ActualText (\0" as *const u8 as *const ::core::ffi::c_char,
        23 as size_t,
    );
    if pdf_doc_enc == 0 {
        dev_out(
            p,
            b"\xFE\xFF\0" as *const u8 as *const ::core::ffi::c_char,
            2 as size_t,
        );
    }
    loop {
        let fresh65 = count;
        count = count - 1;
        if !(fresh65 > 0 as ::core::ffi::c_int) {
            break;
        }
        let mut s: [::core::ffi::c_uchar; 2] = [
            (*unicodes as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar,
            (*unicodes as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar,
        ];
        let mut buf: [::core::ffi::c_char; 32] = [0; 32];
        i = pdf_doc_enc;
        len = 0 as ::core::ffi::c_int;
        while i < 2 as ::core::ffi::c_int {
            let mut c: ::core::ffi::c_uchar = s[i as usize];
            if c as ::core::ffi::c_int == '(' as i32
                || c as ::core::ffi::c_int == ')' as i32
                || c as ::core::ffi::c_int == '\\' as i32
            {
                len += sprintf(
                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                    b"\\%c\0" as *const u8 as *const ::core::ffi::c_char,
                    c as ::core::ffi::c_int,
                );
            } else if (c as ::core::ffi::c_int) < ' ' as i32 {
                len += sprintf(
                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                    b"\\%03o\0" as *const u8 as *const ::core::ffi::c_char,
                    c as ::core::ffi::c_int,
                );
            } else {
                len += sprintf(
                    (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize),
                    b"%c\0" as *const u8 as *const ::core::ffi::c_char,
                    c as ::core::ffi::c_int,
                );
            }
            i += 1;
        }
        dev_out(p, &raw mut buf as *mut ::core::ffi::c_char, len as size_t);
        unicodes = unicodes.offset(1);
    }
    dev_out(
        p,
        b") >> BDC\0" as *const u8 as *const ::core::ffi::c_char,
        8 as size_t,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_end_actualtext")]
pub unsafe extern "C-unwind" fn pdf_dev_end_actualtext() {
    let mut p: *mut pdf_dev = current_device();
    pdf_dev_graphics_mode(p);
    dev_out(
        p,
        b" EMC\0" as *const u8 as *const ::core::ffi::c_char,
        4 as size_t,
    );
}
#[unsafe(export_name = "__pdf_impl_graphics_mode")]
pub unsafe extern "C-unwind" fn graphics_mode() {
    let mut p: *mut pdf_dev = current_device();
    pdf_dev_graphics_mode(p);
}
pub(crate) unsafe fn dev_unit_dviunit() -> ::core::ffi::c_double {
    let mut p: *mut pdf_dev = current_device();
    return pdf_dev_unit_dviunit(p);
}
pub(crate) unsafe fn pdf_sprint_matrix(
    mut buf: *mut ::core::ffi::c_char,
    mut M: *const pdf_tmatrix,
) -> ::core::ffi::c_int {
    let mut p: *mut pdf_dev = current_device();
    return pdf_dev_sprint_matrix(p, buf, M) as ::core::ffi::c_int;
}
pub(crate) unsafe fn pdf_sprint_rect(
    mut buf: *mut ::core::ffi::c_char,
    mut rect: *const pdf_rect,
) -> ::core::ffi::c_int {
    let mut p: *mut pdf_dev = current_device();
    return pdf_dev_sprint_rect(p, buf, rect) as ::core::ffi::c_int;
}
pub(crate) unsafe fn pdf_sprint_coord(
    mut buf: *mut ::core::ffi::c_char,
    mut c: *const pdf_coord,
) -> ::core::ffi::c_int {
    let mut p: *mut pdf_dev = current_device();
    return pdf_dev_sprint_coord(p, buf, c) as ::core::ffi::c_int;
}
pub(crate) unsafe fn pdf_sprint_length(
    mut buf: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut p: *mut pdf_dev = current_device();
    return pdf_dev_sprint_length(p, buf, value) as ::core::ffi::c_int;
}
pub(crate) unsafe fn pdf_sprint_number(
    mut buf: *mut ::core::ffi::c_char,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut p: *mut pdf_dev = current_device();
    return pdf_dev_sprint_number(p, buf, value) as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_get_font_wmode")]
pub unsafe extern "C-unwind" fn pdf_dev_get_font_wmode(
    mut font_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut pdf_dev = current_device();
    let mut font: *mut dev_font = ::core::ptr::null_mut::<dev_font>();
    font = (*p).fonts.offset(font_id as isize) as *mut dev_font;
    if !font.is_null() {
        return (*font).wmode;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_font_minbytes")]
pub unsafe extern "C-unwind" fn pdf_dev_font_minbytes(
    mut font_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut pdf_dev = current_device();
    let mut font: *mut dev_font = ::core::ptr::null_mut::<dev_font>();
    let mut minbytes: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    font = (*p).fonts.offset(font_id as isize) as *mut dev_font;
    if !font.is_null() && (*font).format == PDF_FONTTYPE_COMPOSITE {
        let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
        cmap = CMap_cache_get((*font).enc_id);
        minbytes = CMap_get_profile(cmap, CMAP_PROF_TYPE_INBYTES_MIN);
    }
    return minbytes;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_reset_global_state")]
pub unsafe extern "C-unwind" fn pdf_dev_reset_global_state() {
    let mut p: *mut pdf_dev = current_device();
    (*p).fonts = ::core::ptr::null_mut::<dev_font>();
    (*p).num_dev_fonts = 0 as ::core::ffi::c_int;
    (*p).max_dev_fonts = 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_begin_page_plan")]
pub unsafe extern "C-unwind" fn pdf_dev_begin_page_plan() {
    let mut p: *mut pdf_dev = current_device();
    (*p).recording_page = crate::pipeline::page_begin(
        (*p).unit.dvi2pts,
        (*p).unit.precision,
        (*p).param.autorotate,
        (*p).text_state.dir_mode,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_dev_end_page_plan")]
pub unsafe extern "C-unwind" fn pdf_dev_end_page_plan() {
    pdev.recording_page = 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn render_page(
    mut commands: *const tt_pdf_render_command,
    mut count: size_t,
    mut fonts: *const tt_pdf_render_font,
    mut font_count: size_t,
    mut unit: ::core::ffi::c_double,
    mut precision: ::core::ffi::c_int,
    mut autorotate: ::core::ffi::c_int,
    mut direction: ::core::ffi::c_int,
    mut output: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut device: pdf_dev = pdf_dev {
        motion_state: 0 as ::core::ffi::c_int,
        param: dev_param {
            autorotate: 0,
            colormode: 0,
        },
        unit: dev_unit {
            dvi2pts: 0.,
            min_bp_val: 0,
            precision: 0,
        },
        text_state: text_state {
            font_id: 0,
            offset: 0,
            ref_x: 0,
            ref_y: 0,
            raise: 0,
            leading: 0,
            matrix: C2RustUnnamed {
                slant: 0.,
                extend: 0.,
                rotate: 0,
            },
            bold_param: 0.,
            dir_mode: 0,
            force_reset: 0,
            is_mb: 0,
        },
        fonts: ::core::ptr::null_mut::<dev_font>(),
        num_dev_fonts: 0,
        max_dev_fonts: 0,
        format_buffer: [0; 4097],
        render_output: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        recording_page: 0,
    };
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    device.render_output = output;
    device.unit.dvi2pts = unit;
    device.unit.precision = precision;
    device.unit.min_bp_val = (floor(
        1.0f64
            / (ten_pow[precision as usize] as ::core::ffi::c_double * unit)
            / 1 as ::core::ffi::c_int as ::core::ffi::c_double
            + 0.5f64,
    ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double)
        as ::core::ffi::c_int;
    device.param.autorotate = autorotate;
    device.text_state.font_id = -(1 as ::core::ffi::c_int);
    device.text_state.matrix.extend = 1.0f64;
    device.text_state.dir_mode = direction;
    device.motion_state = GRAPHICS_MODE;
    device.num_dev_fonts = font_count as ::core::ffi::c_int;
    device.fonts = calloc(
        if font_count != 0 {
            font_count
        } else {
            1 as size_t
        },
        ::core::mem::size_of::<dev_font>() as size_t,
    ) as *mut dev_font;
    if device.fonts.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    i = 0 as size_t;
    while i < font_count {
        let mut font: *mut dev_font = device.fonts.offset(i as isize) as *mut dev_font;
        memcpy(
            &raw mut (*font).short_name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            &raw const (*fonts.offset(i as isize)).name as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
        );
        (*font).sptsize = (*fonts.offset(i as isize)).size as spt_t;
        (*font).format = (*fonts.offset(i as isize)).format;
        (*font).wmode = (*fonts.offset(i as isize)).wmode;
        (*font).extend = (*fonts.offset(i as isize)).extend;
        (*font).slant = (*fonts.offset(i as isize)).slant;
        (*font).bold = (*fonts.offset(i as isize)).bold;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    's_84: loop {
        if !(i < count) {
            current_block = 7659304154607701039;
            break;
        }
        let mut command: *const tt_pdf_render_command =
            commands.offset(i as isize) as *const tt_pdf_render_command;
        match (*command).kind {
            0 => {
                dev_out(
                    &raw mut device,
                    (*command).data as *const ::core::ffi::c_char,
                    (*command).length,
                );
            }
            1 => {
                if (*command).font < 0 as ::core::ffi::c_int
                    || (*command).font as size_t >= font_count
                {
                    current_block = 10538671624953381877;
                    break;
                }
                if (*fonts.offset((*command).font as isize)).format == PDF_FONTTYPE_COMPOSITE {
                    if (*command).length > (FORMAT_BUF_SIZE / 2 as ::core::ffi::c_int) as size_t
                        || (*command).length.wrapping_rem(2 as size_t) != 0
                    {
                        current_block = 10538671624953381877;
                        break;
                    }
                } else {
                    let mut escaped: size_t = 0 as size_t;
                    j = 0 as size_t;
                    while j < (*command).length {
                        let mut c: ::core::ffi::c_uchar = *(*command).data.offset(j as isize);
                        if escaped > (FORMAT_BUF_SIZE - 4 as ::core::ffi::c_int) as size_t {
                            current_block = 10538671624953381877;
                            break 's_84;
                        }
                        escaped = escaped.wrapping_add(
                            (if (c as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
                                || c as ::core::ffi::c_int > 126 as ::core::ffi::c_int
                            {
                                4 as ::core::ffi::c_int
                            } else if c as ::core::ffi::c_int == '(' as i32
                                || c as ::core::ffi::c_int == ')' as i32
                                || c as ::core::ffi::c_int == '\\' as i32
                            {
                                2 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            }) as size_t,
                        );
                        j = j.wrapping_add(1);
                    }
                }
                dev_set_string(
                    &raw mut device,
                    (*command).x as spt_t,
                    (*command).y as spt_t,
                    (*command).data as *const ::core::ffi::c_void,
                    (*command).length,
                    (*command).width as spt_t,
                    (*command).font,
                    1 as ::core::ffi::c_int,
                );
            }
            2 => {
                dev_set_rule(
                    &raw mut device,
                    (*command).x as spt_t,
                    (*command).y as spt_t,
                    (*command).width as spt_t,
                    (*command).height as spt_t,
                );
            }
            3 => {
                pdf_dev_graphics_mode(&raw mut device);
            }
            4 => {
                dev_set_dirmode(&raw mut device, (*command).x as ::core::ffi::c_int);
            }
            5 => {
                dev_reset_fonts(&raw mut device, (*command).x as ::core::ffi::c_int);
            }
            _ => {
                current_block = 10538671624953381877;
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    match current_block {
        10538671624953381877 => {
            free(device.fonts as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
        _ => {
            free(device.fonts as *mut ::core::ffi::c_void);
            return 0 as ::core::ffi::c_int;
        }
    };
}
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
unsafe extern "C" {
    fn calloc(__count: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(_: *mut ::core::ffi::c_void);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_set_string(
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut input: *const ::core::ffi::c_void,
    mut length: size_t,
    mut width: spt_t,
    mut font_id: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_dev_set_string(xpos, ypos, input, length, width, font_id)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_init_device(
    mut dvi2pts: ::core::ffi::c_double,
    mut precision: ::core::ffi::c_int,
    mut black_and_white: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_init_device(dvi2pts, precision, black_and_white)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_close_device() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_close_device() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_reset_fonts(mut newpage: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_reset_fonts(newpage) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_reset_color(mut force: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_reset_color(force) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_bop(mut M: *const pdf_tmatrix) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_bop(M) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_eop() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_eop() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_locate_font(
    mut font_name: *const ::core::ffi::c_char,
    mut ptsize: spt_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_locate_font(font_name, ptsize) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_set_rule(
    mut xpos: spt_t,
    mut ypos: spt_t,
    mut width: spt_t,
    mut height: spt_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_dev_set_rule(xpos, ypos, width, height)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_put_image(
    mut id: ::core::ffi::c_int,
    mut ti: *mut transform_info,
    mut ref_x: ::core::ffi::c_double,
    mut ref_y: ::core::ffi::c_double,
    mut rect: *mut pdf_rect,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_dev_put_image(id, ti, ref_x, ref_y, rect)
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_begin_actualtext(
    mut unicodes: *mut uint16_t,
    mut count: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_begin_actualtext(unicodes, count) })
    {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_end_actualtext() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_end_actualtext() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_graphics_mode() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { graphics_mode() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_get_font_wmode(
    mut font_id: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_get_font_wmode(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_font_minbytes(
    mut font_id: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_font_minbytes(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dev_reset_global_state() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_reset_global_state() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_begin_page_plan() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_begin_page_plan() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_dev_end_page_plan() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dev_end_page_plan() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
