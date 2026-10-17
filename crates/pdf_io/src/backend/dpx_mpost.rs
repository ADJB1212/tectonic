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
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isspace(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn cos(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sin(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn ceil(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atof(_: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn strtod(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
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
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __nitems: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn rewind(_: *mut FILE);
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_file_size"]
    fn file_size(file: *mut FILE) -> int32_t;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_set_number"]
    fn pdf_set_number(object: *mut pdf_obj, value: ::core::ffi::c_double);
    #[link_name = "__pdf_impl_pdf_number_value"]
    fn pdf_number_value(number: *mut pdf_obj) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pdf_string_value"]
    fn pdf_string_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_string_length"]
    fn pdf_string_length(object: *mut pdf_obj) -> ::core::ffi::c_uint;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_name_value"]
    fn pdf_name_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_get_array"]
    fn pdf_get_array(array: *mut pdf_obj, idx: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_array_length"]
    fn pdf_array_length(array: *mut pdf_obj) -> ::core::ffi::c_uint;
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
    #[link_name = "__pdf_impl_pdf_color_rgbcolor"]
    fn pdf_color_rgbcolor(
        color: *mut pdf_color,
        r: ::core::ffi::c_double,
        g: ::core::ffi::c_double,
        b: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_cmykcolor"]
    fn pdf_color_cmykcolor(
        color: *mut pdf_color,
        c: ::core::ffi::c_double,
        m: ::core::ffi::c_double,
        y: ::core::ffi::c_double,
        k: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_color_graycolor"]
    fn pdf_color_graycolor(color: *mut pdf_color, g: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_transform_info_clear"]
    fn transform_info_clear(info: *mut transform_info);
    #[link_name = "__pdf_impl_dev_unit_dviunit"]
    fn dev_unit_dviunit() -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pdf_dev_set_string"]
    fn pdf_dev_set_string(
        xpos: spt_t,
        ypos: spt_t,
        instr_ptr: *const ::core::ffi::c_void,
        instr_len: size_t,
        text_width: spt_t,
        font_id: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_dev_put_image"]
    fn pdf_dev_put_image(
        xobj_id: ::core::ffi::c_int,
        ti: *mut transform_info,
        ref_x: ::core::ffi::c_double,
        ref_y: ::core::ffi::c_double,
        rect: *mut pdf_rect,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_locate_font"]
    fn pdf_dev_locate_font(
        font_name: *const ::core::ffi::c_char,
        ptsize: spt_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_get_font_wmode"]
    fn pdf_dev_get_font_wmode(font_id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_get_dirmode"]
    fn pdf_dev_get_dirmode() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_set_dirmode"]
    fn pdf_dev_set_dirmode(dir_mode: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_dev_get_param"]
    fn pdf_dev_get_param(param_type: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_set_param"]
    fn pdf_dev_set_param(param_type: ::core::ffi::c_int, value: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_graphics_mode"]
    fn graphics_mode();
    #[link_name = "__pdf_impl_pdf_doc_current_page_number"]
    fn pdf_doc_current_page_number() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_doc_begin_page"]
    fn pdf_doc_begin_page(
        scale: ::core::ffi::c_double,
        x_origin: ::core::ffi::c_double,
        y_origin: ::core::ffi::c_double,
    );
    #[link_name = "__pdf_impl_pdf_doc_end_page"]
    fn pdf_doc_end_page();
    #[link_name = "__pdf_impl_pdf_doc_set_mediabox"]
    fn pdf_doc_set_mediabox(page_no: ::core::ffi::c_uint, mediabox: *const pdf_rect);
    #[link_name = "__pdf_impl_pdf_doc_begin_grabbing"]
    fn pdf_doc_begin_grabbing(
        ident: *const ::core::ffi::c_char,
        ref_x: ::core::ffi::c_double,
        ref_y: ::core::ffi::c_double,
        cropbox: *const pdf_rect,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_doc_end_grabbing"]
    fn pdf_doc_end_grabbing(attrib: *mut pdf_obj);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_pdf_lookup_fontmap_record"]
    fn pdf_lookup_fontmap_record(kp: *const ::core::ffi::c_char) -> *mut fontmap_rec;
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_dev_currentmatrix"]
    fn pdf_dev_currentmatrix(M: *mut pdf_tmatrix) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_currentpoint"]
    fn pdf_dev_currentpoint(cp: *mut pdf_coord) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_setlinewidth"]
    fn pdf_dev_setlinewidth(width: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_setmiterlimit"]
    fn pdf_dev_setmiterlimit(mlimit: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_setlinecap"]
    fn pdf_dev_setlinecap(style: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_setlinejoin"]
    fn pdf_dev_setlinejoin(style: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_setdash"]
    fn pdf_dev_setdash(
        count: ::core::ffi::c_int,
        pattern: *mut ::core::ffi::c_double,
        offset: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_moveto"]
    fn pdf_dev_moveto(x: ::core::ffi::c_double, y: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_rmoveto"]
    fn pdf_dev_rmoveto(x: ::core::ffi::c_double, y: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_closepath"]
    fn pdf_dev_closepath() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_lineto"]
    fn pdf_dev_lineto(x0: ::core::ffi::c_double, y0: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_rlineto"]
    fn pdf_dev_rlineto(x0: ::core::ffi::c_double, y0: ::core::ffi::c_double) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_curveto"]
    fn pdf_dev_curveto(
        x0: ::core::ffi::c_double,
        y0: ::core::ffi::c_double,
        x1: ::core::ffi::c_double,
        y1: ::core::ffi::c_double,
        x2: ::core::ffi::c_double,
        y2: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_rcurveto"]
    fn pdf_dev_rcurveto(
        x0: ::core::ffi::c_double,
        y0: ::core::ffi::c_double,
        x1: ::core::ffi::c_double,
        y1: ::core::ffi::c_double,
        x2: ::core::ffi::c_double,
        y2: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_arc"]
    fn pdf_dev_arc(
        c_x: ::core::ffi::c_double,
        c_y: ::core::ffi::c_double,
        r: ::core::ffi::c_double,
        a_0: ::core::ffi::c_double,
        a_1: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_arcn"]
    fn pdf_dev_arcn(
        c_x: ::core::ffi::c_double,
        c_y: ::core::ffi::c_double,
        r: ::core::ffi::c_double,
        a_0: ::core::ffi::c_double,
        a_1: ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_newpath"]
    fn pdf_dev_newpath() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_clip"]
    fn pdf_dev_clip() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_eoclip"]
    fn pdf_dev_eoclip() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_flushpath"]
    fn pdf_dev_flushpath(
        p_op: ::core::ffi::c_char,
        fill_rule: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_concat"]
    fn pdf_dev_concat(M: *const pdf_tmatrix) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_dtransform"]
    fn pdf_dev_dtransform(p: *mut pdf_coord, M: *const pdf_tmatrix);
    #[link_name = "__pdf_impl_pdf_dev_idtransform"]
    fn pdf_dev_idtransform(p: *mut pdf_coord, M: *const pdf_tmatrix);
    #[link_name = "__pdf_impl_pdf_dev_gsave"]
    fn pdf_dev_gsave() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_grestore"]
    fn pdf_dev_grestore() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_dev_set_color"]
    fn pdf_dev_set_color(
        color: *const pdf_color,
        mask: ::core::ffi::c_char,
        force: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_dump"]
    fn dump(start: *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_pdfparse_skip_line"]
    fn pdfparse_skip_line(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_parse_number"]
    fn parse_number(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_ident"]
    fn parse_ident(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_pdf_name"]
    fn parse_pdf_name(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_string"]
    fn parse_pdf_string(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_dict"]
    fn parse_pdf_dict(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_array"]
    fn parse_pdf_array(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_lookup_sfd_record"]
    fn lookup_sfd_record(
        rec_id: ::core::ffi::c_int,
        code: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_sfd_load_record"]
    fn sfd_load_record(
        sfd_name: *const ::core::ffi::c_char,
        subfont_id: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tfm_open"]
    fn tfm_open(
        tex_name: *const ::core::ffi::c_char,
        must_exist: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tfm_get_width"]
    fn tfm_get_width(font_id: ::core::ffi::c_int, ch: int32_t) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_tfm_string_width"]
    fn tfm_string_width(
        font_id: ::core::ffi::c_int,
        s: *const ::core::ffi::c_uchar,
        len: ::core::ffi::c_uint,
    ) -> fixword;
    #[link_name = "__pdf_impl_tfm_exists"]
    fn tfm_exists(tfm_name: *const ::core::ffi::c_char) -> bool;
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C-unwind" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C-unwind" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3],
    pub _nbuf: [::core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type fixword = int32_t;
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
pub type spt_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_tmatrix {
    pub a: ::core::ffi::c_double,
    pub b: ::core::ffi::c_double,
    pub c: ::core::ffi::c_double,
    pub d: ::core::ffi::c_double,
    pub e: ::core::ffi::c_double,
    pub f: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: ::core::ffi::c_double,
    pub lly: ::core::ffi::c_double,
    pub urx: ::core::ffi::c_double,
    pub ury: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_coord {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_info {
    pub width: ::core::ffi::c_double,
    pub height: ::core::ffi::c_double,
    pub depth: ::core::ffi::c_double,
    pub matrix: pdf_tmatrix,
    pub bbox: pdf_rect,
    pub flags: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp_font {
    pub font_name: *mut ::core::ffi::c_char,
    pub font_id: ::core::ffi::c_int,
    pub tfm_id: ::core::ffi::c_int,
    pub subfont_id: ::core::ffi::c_int,
    pub pt_size: ::core::ffi::c_double,
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
pub struct C2RustUnnamed {
    pub sfd_name: *mut ::core::ffi::c_char,
    pub subfont_id: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct operators {
    pub token: *const ::core::ffi::c_char,
    pub opcode: ::core::ffi::c_int,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846264338327950288f64;
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
pub const PDF_NUMBER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_STRING: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PDF_NAME: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PDF_ARRAY: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PDF_DICT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const INFO_HAS_USER_BBOX: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PDF_DEV_PARAM_AUTOROTATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PDF_DASH_SIZE_MAX: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PDF_FILL_RULE_NONZERO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut Yorigin: ::core::ffi::c_double = 0.;
static mut Xorigin: ::core::ffi::c_double = 0.;
static mut translate_origin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_mps_set_translate_origin")]
pub unsafe extern "C-unwind" fn mps_set_translate_origin(mut v: ::core::ffi::c_int) {
    translate_origin = v;
}
static mut font_stack: [mp_font; 256] = [
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: -(1 as ::core::ffi::c_int),
        tfm_id: -(1 as ::core::ffi::c_int),
        subfont_id: -(1 as ::core::ffi::c_int),
        pt_size: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
    mp_font {
        font_name: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        font_id: 0,
        tfm_id: 0,
        subfont_id: 0,
        pt_size: 0.,
    },
];
static mut currentfont: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn clear_mp_font_struct(mut font: *mut mp_font) {
    if font.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"font\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(*font).font_name.is_null() {
        free((*font).font_name as *mut ::core::ffi::c_void);
    }
    (*font).font_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*font).font_id = -(1 as ::core::ffi::c_int);
    (*font).tfm_id = -(1 as ::core::ffi::c_int);
    (*font).subfont_id = -(1 as ::core::ffi::c_int);
    (*font).pt_size = 0.0f64;
}
pub const MP_CMODE_MPOST: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MP_CMODE_DVIPSK: ::core::ffi::c_int = 1;
pub const MP_CMODE_PTEXVERT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut mp_cmode: ::core::ffi::c_int = MP_CMODE_MPOST;
unsafe extern "C-unwind" fn mp_setfont(
    mut font_name: *const ::core::ffi::c_char,
    mut pt_size: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut name: *const ::core::ffi::c_char = font_name;
    let mut font: *mut mp_font = ::core::ptr::null_mut::<mp_font>();
    let mut subfont_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut mrec: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    font = (if currentfont < 0 as ::core::ffi::c_int {
        ::core::ptr::null_mut::<mp_font>()
    } else {
        (&raw mut font_stack as *mut mp_font).offset(currentfont as isize) as *mut mp_font
    }) as *mut mp_font;
    mrec = pdf_lookup_fontmap_record(font_name);
    if !mrec.is_null()
        && !(*mrec).charmap.sfd_name.is_null()
        && !(*mrec).charmap.subfont_id.is_null()
    {
        subfont_id = sfd_load_record((*mrec).charmap.sfd_name, (*mrec).charmap.subfont_id);
    }
    if !mrec.is_null() && !(*mrec).map_name.is_null() {
        name = (*mrec).map_name;
    } else {
        name = font_name;
    }
    free((*font).font_name as *mut ::core::ffi::c_void);
    (*font).font_name = new(
        (strlen(font_name).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*font).font_name, font_name);
    (*font).subfont_id = subfont_id;
    (*font).pt_size = pt_size;
    (*font).tfm_id = tfm_open(font_name, 0 as ::core::ffi::c_int);
    (*font).font_id = pdf_dev_locate_font(name, (pt_size * dev_unit_dviunit()) as spt_t);
    if (*font).font_id < 0 as ::core::ffi::c_int {
        pdf_abort!(
            b"MPOST: No physical font assigned for \"%s\".\0" as *const u8
                as *const ::core::ffi::c_char,
            font_name,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn save_font() {
    let mut current: *mut mp_font = ::core::ptr::null_mut::<mp_font>();
    let mut next: *mut mp_font = ::core::ptr::null_mut::<mp_font>();
    let fresh23 = currentfont;
    currentfont = currentfont + 1;
    current = (&raw mut font_stack as *mut mp_font).offset(fresh23 as isize) as *mut mp_font
        as *mut mp_font;
    next = (&raw mut font_stack as *mut mp_font).offset(currentfont as isize) as *mut mp_font
        as *mut mp_font;
    if !current.is_null()
        && !(*current).font_name.is_null()
        && (*current).font_id >= 0 as ::core::ffi::c_int
    {
        (*next).font_name = new(
            (strlen((*current).font_name).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy((*next).font_name, (*current).font_name);
        (*next).font_id = (*current).font_id;
        (*next).pt_size = (*current).pt_size;
        (*next).subfont_id = (*current).subfont_id;
        (*next).tfm_id = (*current).tfm_id;
    } else {
        (*next).font_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*next).font_id = -(1 as ::core::ffi::c_int);
        (*next).pt_size = 0.0f64;
        (*next).subfont_id = -(1 as ::core::ffi::c_int);
        (*next).tfm_id = -(1 as ::core::ffi::c_int);
    };
}
unsafe extern "C-unwind" fn restore_font() {
    let mut current: *mut mp_font = ::core::ptr::null_mut::<mp_font>();
    current = (if currentfont < 0 as ::core::ffi::c_int {
        ::core::ptr::null_mut::<mp_font>()
    } else {
        (&raw mut font_stack as *mut mp_font).offset(currentfont as isize) as *mut mp_font
    }) as *mut mp_font;
    if !current.is_null() {
        clear_mp_font_struct(current);
    }
    if currentfont > 0 as ::core::ffi::c_int {
        currentfont -= 1;
    }
}
unsafe extern "C-unwind" fn clear_fonts() {
    while currentfont >= 0 as ::core::ffi::c_int {
        clear_mp_font_struct(
            (&raw mut font_stack as *mut mp_font).offset(currentfont as isize) as *mut mp_font,
        );
        currentfont -= 1;
    }
    currentfont = 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn is_fontname(mut token: *const ::core::ffi::c_char) -> bool {
    let mut mrec: *mut fontmap_rec = ::core::ptr::null_mut::<fontmap_rec>();
    mrec = pdf_lookup_fontmap_record(token);
    if !mrec.is_null() {
        return true_0 != 0;
    }
    return tfm_exists(token);
}
#[unsafe(export_name = "__pdf_impl_mps_scan_bbox")]
pub unsafe extern "C-unwind" fn mps_scan_bbox(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut bbox: *mut pdf_rect,
) -> ::core::ffi::c_int {
    let mut number: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut values: [::core::ffi::c_double; 4] = [0.; 4];
    let mut i: ::core::ffi::c_int = 0;
    while *pp < endptr && isspace(**pp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        *pp = (*pp).offset(1);
    }
    while *pp < endptr && **pp as ::core::ffi::c_int == '%' as i32 {
        if (*pp).offset(14 as ::core::ffi::c_int as isize) < endptr
            && !strstartswith(
                *pp,
                b"%%BoundingBox:\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            *pp = (*pp).offset(14 as ::core::ffi::c_int as isize);
            i = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                skip_white(pp, endptr);
                number = parse_number(pp, endptr);
                if number.is_null() {
                    break;
                }
                values[i as usize] = atof(number);
                free(number as *mut ::core::ffi::c_void);
                i += 1;
            }
            if i < 4 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            } else {
                if translate_origin != 0 {
                    (*bbox).llx = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    (*bbox).lly = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                    (*bbox).urx = values[2 as ::core::ffi::c_int as usize]
                        - values[0 as ::core::ffi::c_int as usize];
                    (*bbox).ury = values[3 as ::core::ffi::c_int as usize]
                        - values[1 as ::core::ffi::c_int as usize];
                    Xorigin = values[0 as ::core::ffi::c_int as usize];
                    Yorigin = values[1 as ::core::ffi::c_int as usize];
                } else {
                    (*bbox).llx = values[0 as ::core::ffi::c_int as usize];
                    (*bbox).lly = values[1 as ::core::ffi::c_int as usize];
                    (*bbox).urx = values[2 as ::core::ffi::c_int as usize];
                    (*bbox).ury = values[3 as ::core::ffi::c_int as usize];
                    Xorigin = 0.0f64;
                    Yorigin = 0.0f64;
                }
                return 0 as ::core::ffi::c_int;
            }
        }
        pdfparse_skip_line(pp, endptr);
        while *pp < endptr && isspace(**pp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            *pp = (*pp).offset(1);
        }
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn skip_prolog(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) {
    let mut found_prolog: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut save: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    save = *start;
    while *start < end {
        if **start as ::core::ffi::c_int != '%' as i32 {
            skip_white(start, end);
        }
        if *start >= end {
            break;
        }
        if !strstartswith(
            *start,
            b"%%EndProlog\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            found_prolog = 1 as ::core::ffi::c_int;
            pdfparse_skip_line(start, end);
            break;
        } else if !strstartswith(
            *start,
            b"%%Page:\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            pdfparse_skip_line(start, end);
            break;
        } else {
            pdfparse_skip_line(start, end);
        }
    }
    if found_prolog == 0 {
        *start = save;
    }
}
pub const ADD: ::core::ffi::c_int = 1;
pub const SUB: ::core::ffi::c_int = 2;
pub const MUL: ::core::ffi::c_int = 3;
pub const DIV: ::core::ffi::c_int = 4;
pub const NEG: ::core::ffi::c_int = 5;
pub const TRUNCATE: ::core::ffi::c_int = 6;
pub const CLEAR: ::core::ffi::c_int = 10;
pub const EXCH: ::core::ffi::c_int = 11;
pub const POP: ::core::ffi::c_int = 12;
pub const NEWPATH: ::core::ffi::c_int = 31;
pub const CLOSEPATH: ::core::ffi::c_int = 32;
pub const MOVETO: ::core::ffi::c_int = 33;
pub const RMOVETO: ::core::ffi::c_int = 34;
pub const CURVETO: ::core::ffi::c_int = 35;
pub const RCURVETO: ::core::ffi::c_int = 36;
pub const LINETO: ::core::ffi::c_int = 37;
pub const RLINETO: ::core::ffi::c_int = 38;
pub const ARC: ::core::ffi::c_int = 39;
pub const ARCN: ::core::ffi::c_int = 40;
pub const FILL: ::core::ffi::c_int = 41;
pub const STROKE: ::core::ffi::c_int = 42;
pub const SHOW: ::core::ffi::c_int = 43;
pub const CLIP: ::core::ffi::c_int = 44;
pub const EOCLIP: ::core::ffi::c_int = 45;
pub const SHOWPAGE: ::core::ffi::c_int = 49;
pub const GSAVE: ::core::ffi::c_int = 50;
pub const GRESTORE: ::core::ffi::c_int = 51;
pub const CONCAT: ::core::ffi::c_int = 52;
pub const SCALE: ::core::ffi::c_int = 53;
pub const TRANSLATE: ::core::ffi::c_int = 54;
pub const ROTATE: ::core::ffi::c_int = 55;
pub const SETLINEWIDTH: ::core::ffi::c_int = 60;
pub const SETDASH: ::core::ffi::c_int = 61;
pub const SETLINECAP: ::core::ffi::c_int = 62;
pub const SETLINEJOIN: ::core::ffi::c_int = 63;
pub const SETMITERLIMIT: ::core::ffi::c_int = 64;
pub const SETGRAY: ::core::ffi::c_int = 70;
pub const SETRGBCOLOR: ::core::ffi::c_int = 71;
pub const SETCMYKCOLOR: ::core::ffi::c_int = 72;
pub const CURRENTPOINT: ::core::ffi::c_int = 80;
pub const IDTRANSFORM: ::core::ffi::c_int = 81;
pub const DTRANSFORM: ::core::ffi::c_int = 82;
pub const FINDFONT: ::core::ffi::c_int = 201;
pub const SCALEFONT: ::core::ffi::c_int = 202;
pub const SETFONT: ::core::ffi::c_int = 203;
pub const CURRENTFONT: ::core::ffi::c_int = 204;
pub const STRINGWIDTH: ::core::ffi::c_int = 210;
pub const DEF: ::core::ffi::c_int = 999;
pub const FSHOW: ::core::ffi::c_int = 1001;
pub const STEXFIG: ::core::ffi::c_int = 1002;
pub const ETEXFIG: ::core::ffi::c_int = 1003;
pub const HLW: ::core::ffi::c_int = 1004;
pub const VLW: ::core::ffi::c_int = 1005;
pub const RD: ::core::ffi::c_int = 1006;
pub const B: ::core::ffi::c_int = 1007;
static mut ps_operators: [operators; 48] = [
    operators {
        token: b"add\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: ADD,
    },
    operators {
        token: b"mul\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: MUL,
    },
    operators {
        token: b"div\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: DIV,
    },
    operators {
        token: b"neg\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: NEG,
    },
    operators {
        token: b"sub\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SUB,
    },
    operators {
        token: b"truncate\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: TRUNCATE,
    },
    operators {
        token: b"clear\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: CLEAR,
    },
    operators {
        token: b"exch\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: EXCH,
    },
    operators {
        token: b"pop\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: POP,
    },
    operators {
        token: b"clip\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: CLIP,
    },
    operators {
        token: b"eoclip\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: EOCLIP,
    },
    operators {
        token: b"closepath\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: CLOSEPATH,
    },
    operators {
        token: b"concat\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: CONCAT,
    },
    operators {
        token: b"newpath\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: NEWPATH,
    },
    operators {
        token: b"moveto\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: MOVETO,
    },
    operators {
        token: b"rmoveto\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: RMOVETO,
    },
    operators {
        token: b"lineto\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: LINETO,
    },
    operators {
        token: b"rlineto\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: RLINETO,
    },
    operators {
        token: b"curveto\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: CURVETO,
    },
    operators {
        token: b"rcurveto\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: RCURVETO,
    },
    operators {
        token: b"arc\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: ARC,
    },
    operators {
        token: b"arcn\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: ARCN,
    },
    operators {
        token: b"stroke\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: STROKE,
    },
    operators {
        token: b"fill\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: FILL,
    },
    operators {
        token: b"show\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SHOW,
    },
    operators {
        token: b"showpage\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SHOWPAGE,
    },
    operators {
        token: b"gsave\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: GSAVE,
    },
    operators {
        token: b"grestore\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: GRESTORE,
    },
    operators {
        token: b"translate\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: TRANSLATE,
    },
    operators {
        token: b"rotate\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: ROTATE,
    },
    operators {
        token: b"scale\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SCALE,
    },
    operators {
        token: b"setlinecap\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETLINECAP,
    },
    operators {
        token: b"setlinejoin\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETLINEJOIN,
    },
    operators {
        token: b"setlinewidth\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETLINEWIDTH,
    },
    operators {
        token: b"setmiterlimit\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETMITERLIMIT,
    },
    operators {
        token: b"setdash\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETDASH,
    },
    operators {
        token: b"setgray\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETGRAY,
    },
    operators {
        token: b"setrgbcolor\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETRGBCOLOR,
    },
    operators {
        token: b"setcmykcolor\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETCMYKCOLOR,
    },
    operators {
        token: b"currentpoint\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: CURRENTPOINT,
    },
    operators {
        token: b"dtransform\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: DTRANSFORM,
    },
    operators {
        token: b"idtransform\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: IDTRANSFORM,
    },
    operators {
        token: b"findfont\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: FINDFONT,
    },
    operators {
        token: b"scalefont\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SCALEFONT,
    },
    operators {
        token: b"setfont\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETFONT,
    },
    operators {
        token: b"currentfont\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: CURRENTFONT,
    },
    operators {
        token: b"stringwidth\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: STRINGWIDTH,
    },
    operators {
        token: b"def\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: DEF,
    },
];
static mut mps_operators: [operators; 28] = [
    operators {
        token: b"fshow\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: FSHOW,
    },
    operators {
        token: b"startTexFig\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: STEXFIG,
    },
    operators {
        token: b"endTexFig\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: ETEXFIG,
    },
    operators {
        token: b"hlw\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: HLW,
    },
    operators {
        token: b"vlw\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: VLW,
    },
    operators {
        token: b"l\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: LINETO,
    },
    operators {
        token: b"r\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: RLINETO,
    },
    operators {
        token: b"c\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: CURVETO,
    },
    operators {
        token: b"m\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: MOVETO,
    },
    operators {
        token: b"p\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: CLOSEPATH,
    },
    operators {
        token: b"n\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: NEWPATH,
    },
    operators {
        token: b"C\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETCMYKCOLOR,
    },
    operators {
        token: b"G\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETGRAY,
    },
    operators {
        token: b"R\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETRGBCOLOR,
    },
    operators {
        token: b"lj\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETLINEJOIN,
    },
    operators {
        token: b"ml\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETMITERLIMIT,
    },
    operators {
        token: b"lc\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETLINECAP,
    },
    operators {
        token: b"S\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: STROKE,
    },
    operators {
        token: b"F\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: FILL,
    },
    operators {
        token: b"q\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: GSAVE,
    },
    operators {
        token: b"Q\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: GRESTORE,
    },
    operators {
        token: b"s\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SCALE,
    },
    operators {
        token: b"t\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: CONCAT,
    },
    operators {
        token: b"sd\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SETDASH,
    },
    operators {
        token: b"rd\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: RD,
    },
    operators {
        token: b"P\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: SHOWPAGE,
    },
    operators {
        token: b"B\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: B,
    },
    operators {
        token: b"W\0" as *const u8 as *const ::core::ffi::c_char,
        opcode: CLIP,
    },
];
pub const NUM_PS_OPERATORS: usize = (::core::mem::size_of::<[operators; 48]>() as usize)
    .wrapping_div(::core::mem::size_of::<operators>() as usize);
pub const NUM_MPS_OPERATORS: usize = (::core::mem::size_of::<[operators; 28]>() as usize)
    .wrapping_div(::core::mem::size_of::<operators>() as usize);
unsafe extern "C-unwind" fn get_opcode(
    mut token: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while (i as usize) < NUM_PS_OPERATORS {
        if streq_ptr(token, ps_operators[i as usize].token) {
            return ps_operators[i as usize].opcode;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as ::core::ffi::c_uint;
    while (i as usize) < NUM_MPS_OPERATORS {
        if streq_ptr(token, mps_operators[i as usize].token) {
            return mps_operators[i as usize].opcode;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as ::core::ffi::c_int);
}
pub const PS_STACK_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut stack: [*mut pdf_obj; 1024] = [::core::ptr::null::<pdf_obj>() as *mut pdf_obj; 1024];
static mut top_stack: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
unsafe extern "C-unwind" fn do_exch() -> ::core::ffi::c_int {
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if top_stack < 2 as ::core::ffi::c_uint {
        return -(1 as ::core::ffi::c_int);
    }
    tmp = stack[top_stack.wrapping_sub(1 as ::core::ffi::c_uint) as usize];
    stack[top_stack.wrapping_sub(1 as ::core::ffi::c_uint) as usize] =
        stack[top_stack.wrapping_sub(2 as ::core::ffi::c_uint) as usize];
    stack[top_stack.wrapping_sub(2 as ::core::ffi::c_uint) as usize] = tmp;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn do_clear() -> ::core::ffi::c_int {
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    while top_stack > 0 as ::core::ffi::c_uint {
        tmp = if top_stack > 0 as ::core::ffi::c_uint {
            top_stack = top_stack.wrapping_sub(1);
            stack[top_stack as usize]
        } else {
            ::core::ptr::null_mut::<pdf_obj>()
        };
        pdf_release_obj(tmp);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pop_get_numbers(
    mut values: *mut ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    loop {
        let fresh18 = count;
        count = count - 1;
        if !(fresh18 > 0 as ::core::ffi::c_int) {
            break;
        }
        tmp = if top_stack > 0 as ::core::ffi::c_uint {
            top_stack = top_stack.wrapping_sub(1);
            stack[top_stack as usize]
        } else {
            ::core::ptr::null_mut::<pdf_obj>()
        };
        if tmp.is_null() {
            pdf_warning!(b"mpost: Stack underflow.\0" as *const u8 as *const ::core::ffi::c_char,);
            break;
        } else if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NUMBER) {
            pdf_warning!(b"mpost: Not a number!\0" as *const u8 as *const ::core::ffi::c_char,);
            pdf_release_obj(tmp);
            break;
        } else {
            *values.offset(count as isize) = pdf_number_value(tmp);
            pdf_release_obj(tmp);
        }
    }
    return count + 1 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn cvr_array(
    mut array: *mut pdf_obj,
    mut values: *mut ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if !(!array.is_null() && pdf_obj_typeof(array) == PDF_ARRAY) {
        pdf_warning!(b"mpost: Not an array!\0" as *const u8 as *const ::core::ffi::c_char,);
    } else {
        let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        loop {
            let fresh22 = count;
            count = count - 1;
            if !(fresh22 > 0 as ::core::ffi::c_int) {
                break;
            }
            tmp = pdf_get_array(array, count);
            if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NUMBER) {
                pdf_warning!(b"mpost: Not a number!\0" as *const u8 as *const ::core::ffi::c_char,);
                break;
            } else {
                *values.offset(count as isize) = pdf_number_value(tmp);
            }
        }
    }
    pdf_release_obj(array);
    return count + 1 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn is_fontdict(mut dict: *mut pdf_obj) -> bool {
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if !(!dict.is_null() && pdf_obj_typeof(dict) == PDF_DICT) {
        return false_0 != 0;
    }
    tmp = pdf_lookup_dict(dict, b"Type\0" as *const u8 as *const ::core::ffi::c_char);
    if tmp.is_null()
        || !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NAME)
        || strcmp(
            pdf_name_value(tmp),
            b"Font\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
    {
        return false_0 != 0;
    }
    tmp = pdf_lookup_dict(
        dict,
        b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if tmp.is_null() || !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NAME) {
        return false_0 != 0;
    }
    tmp = pdf_lookup_dict(
        dict,
        b"FontScale\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if tmp.is_null() || !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NUMBER) {
        return false_0 != 0;
    }
    return true_0 != 0;
}
unsafe extern "C-unwind" fn do_findfont() -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut font_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut font_name: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    font_name = if top_stack > 0 as ::core::ffi::c_uint {
        top_stack = top_stack.wrapping_sub(1);
        stack[top_stack as usize]
    } else {
        ::core::ptr::null_mut::<pdf_obj>()
    };
    if font_name.is_null() {
        return 1 as ::core::ffi::c_int;
    } else if !font_name.is_null() && pdf_obj_typeof(font_name) == PDF_STRING
        || !font_name.is_null() && pdf_obj_typeof(font_name) == PDF_NAME
    {
        font_dict = pdf_new_dict();
        pdf_add_dict(
            font_dict,
            pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"Font\0" as *const u8 as *const ::core::ffi::c_char),
        );
        if !font_name.is_null() && pdf_obj_typeof(font_name) == PDF_STRING {
            pdf_add_dict(
                font_dict,
                pdf_new_name(b"FontName\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_new_name(pdf_string_value(font_name) as *const ::core::ffi::c_char),
            );
            pdf_release_obj(font_name);
        } else {
            pdf_add_dict(
                font_dict,
                pdf_new_name(b"FontName\0" as *const u8 as *const ::core::ffi::c_char),
                font_name,
            );
        }
        pdf_add_dict(
            font_dict,
            pdf_new_name(b"FontScale\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(1.0f64),
        );
        if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
            let fresh21 = top_stack;
            top_stack = top_stack.wrapping_add(1);
            stack[fresh21 as usize] = font_dict;
        } else {
            pdf_warning!(
                b"PS stack overflow including MetaPost file or inline PS code\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            pdf_release_obj(font_dict);
            error = 1 as ::core::ffi::c_int;
        }
    } else {
        error = 1 as ::core::ffi::c_int;
    }
    return error;
}
unsafe extern "C-unwind" fn do_scalefont() -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut font_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut font_scale: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut scale: ::core::ffi::c_double = 0.;
    error = pop_get_numbers(&raw mut scale, 1 as ::core::ffi::c_int);
    if error != 0 {
        return error;
    }
    font_dict = if top_stack > 0 as ::core::ffi::c_uint {
        top_stack = top_stack.wrapping_sub(1);
        stack[top_stack as usize]
    } else {
        ::core::ptr::null_mut::<pdf_obj>()
    };
    if font_dict.is_null() {
        error = 1 as ::core::ffi::c_int;
    } else if is_fontdict(font_dict) {
        font_scale = pdf_lookup_dict(
            font_dict,
            b"FontScale\0" as *const u8 as *const ::core::ffi::c_char,
        );
        pdf_set_number(font_scale, pdf_number_value(font_scale) * scale);
        if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
            let fresh20 = top_stack;
            top_stack = top_stack.wrapping_add(1);
            stack[fresh20 as usize] = font_dict;
        } else {
            pdf_warning!(
                b"PS stack overflow including MetaPost file or inline PS code\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            pdf_release_obj(font_dict);
            error = 1 as ::core::ffi::c_int;
        }
    } else {
        error = 1 as ::core::ffi::c_int;
    }
    return error;
}
unsafe extern "C-unwind" fn do_setfont() -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut font_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut font_scale: ::core::ffi::c_double = 0.;
    let mut font_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    font_dict = if top_stack > 0 as ::core::ffi::c_uint {
        top_stack = top_stack.wrapping_sub(1);
        stack[top_stack as usize]
    } else {
        ::core::ptr::null_mut::<pdf_obj>()
    };
    if !is_fontdict(font_dict) {
        error = 1 as ::core::ffi::c_int;
    } else {
        font_name = pdf_name_value(pdf_lookup_dict(
            font_dict,
            b"FontName\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        font_scale = pdf_number_value(pdf_lookup_dict(
            font_dict,
            b"FontScale\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        error = mp_setfont(font_name, font_scale);
    }
    pdf_release_obj(font_dict);
    return error;
}
unsafe extern "C-unwind" fn do_currentfont() -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut font: *mut mp_font = ::core::ptr::null_mut::<mp_font>();
    let mut font_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    font = (if currentfont < 0 as ::core::ffi::c_int {
        ::core::ptr::null_mut::<mp_font>()
    } else {
        (&raw mut font_stack as *mut mp_font).offset(currentfont as isize) as *mut mp_font
    }) as *mut mp_font;
    if !(!font.is_null()
        && !(*font).font_name.is_null()
        && (*font).font_id >= 0 as ::core::ffi::c_int)
    {
        pdf_warning!(b"Currentfont undefined...\0" as *const u8 as *const ::core::ffi::c_char,);
        return 1 as ::core::ffi::c_int;
    } else {
        font_dict = pdf_new_dict();
        pdf_add_dict(
            font_dict,
            pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"Font\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_dict(
            font_dict,
            pdf_new_name(b"FontName\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name((*font).font_name),
        );
        pdf_add_dict(
            font_dict,
            pdf_new_name(b"FontScale\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number((*font).pt_size),
        );
        if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
            let fresh19 = top_stack;
            top_stack = top_stack.wrapping_add(1);
            stack[fresh19 as usize] = font_dict;
        } else {
            pdf_warning!(b"PS stack overflow...\0" as *const u8 as *const ::core::ffi::c_char,);
            pdf_release_obj(font_dict);
            error = 1 as ::core::ffi::c_int;
        }
    }
    return error;
}
unsafe extern "C-unwind" fn do_show() -> ::core::ffi::c_int {
    let mut font: *mut mp_font = ::core::ptr::null_mut::<mp_font>();
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut text_str: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut length: ::core::ffi::c_int = 0;
    let mut strptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut text_width: ::core::ffi::c_double = 0.;
    font = (if currentfont < 0 as ::core::ffi::c_int {
        ::core::ptr::null_mut::<mp_font>()
    } else {
        (&raw mut font_stack as *mut mp_font).offset(currentfont as isize) as *mut mp_font
    }) as *mut mp_font;
    if !(!font.is_null()
        && !(*font).font_name.is_null()
        && (*font).font_id >= 0 as ::core::ffi::c_int)
    {
        pdf_warning!(b"Currentfont not set.\0" as *const u8 as *const ::core::ffi::c_char,);
        return 1 as ::core::ffi::c_int;
    }
    pdf_dev_currentpoint(&raw mut cp);
    text_str = if top_stack > 0 as ::core::ffi::c_uint {
        top_stack = top_stack.wrapping_sub(1);
        stack[top_stack as usize]
    } else {
        ::core::ptr::null_mut::<pdf_obj>()
    };
    if !(!text_str.is_null() && pdf_obj_typeof(text_str) == PDF_STRING) {
        pdf_release_obj(text_str);
        return 1 as ::core::ffi::c_int;
    }
    if (*font).font_id < 0 as ::core::ffi::c_int {
        pdf_warning!(b"mpost: not set.\0" as *const u8 as *const ::core::ffi::c_char);
        pdf_release_obj(text_str);
        return 1 as ::core::ffi::c_int;
    }
    strptr = pdf_string_value(text_str) as *mut ::core::ffi::c_uchar;
    length = pdf_string_length(text_str) as ::core::ffi::c_int;
    if (*font).tfm_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"mpost: TFM not found for \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
            (*font).font_name,
        );
        pdf_warning!(
            b"mpost: Text width not calculated...\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    text_width = 0.0f64;
    if (*font).subfont_id >= 0 as ::core::ffi::c_int {
        let mut uch: ::core::ffi::c_ushort = 0;
        let mut ustr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut i: ::core::ffi::c_int = 0;
        ustr = new(((length * 2 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        i = 0 as ::core::ffi::c_int;
        while i < length {
            uch = lookup_sfd_record((*font).subfont_id, *strptr.offset(i as isize));
            *ustr.offset((2 as ::core::ffi::c_int * i) as isize) =
                (uch as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *ustr.offset((2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize) =
                (uch as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            if (*font).tfm_id >= 0 as ::core::ffi::c_int {
                text_width += tfm_get_width((*font).tfm_id, *strptr.offset(i as isize) as int32_t);
            }
            i += 1;
        }
        text_width *= (*font).pt_size;
        pdf_dev_set_string(
            (cp.x * dev_unit_dviunit()) as spt_t,
            (cp.y * dev_unit_dviunit()) as spt_t,
            ustr as *const ::core::ffi::c_void,
            (length * 2 as ::core::ffi::c_int) as size_t,
            (text_width * dev_unit_dviunit()) as spt_t,
            (*font).font_id,
        );
        free(ustr as *mut ::core::ffi::c_void);
    } else {
        if (*font).tfm_id >= 0 as ::core::ffi::c_int {
            text_width = tfm_string_width((*font).tfm_id, strptr, length as ::core::ffi::c_uint)
                as ::core::ffi::c_double
                / FWBASE;
            text_width *= (*font).pt_size;
        }
        pdf_dev_set_string(
            (cp.x * dev_unit_dviunit()) as spt_t,
            (cp.y * dev_unit_dviunit()) as spt_t,
            strptr as *const ::core::ffi::c_void,
            length as size_t,
            (text_width * dev_unit_dviunit()) as spt_t,
            (*font).font_id,
        );
    }
    if pdf_dev_get_font_wmode((*font).font_id) != 0 {
        pdf_dev_rmoveto(0.0f64, -text_width);
    } else {
        pdf_dev_rmoveto(text_width, 0.0f64);
    }
    graphics_mode();
    pdf_release_obj(text_str);
    return 0 as ::core::ffi::c_int;
}
pub const FWBASE: ::core::ffi::c_double =
    ((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as ::core::ffi::c_double;
unsafe extern "C-unwind" fn do_mpost_bind_def(
    mut ps_code: *const ::core::ffi::c_char,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    start = ps_code;
    end = start.offset(strlen(start) as isize);
    error = mp_parse_body(&raw mut start, end, x_user, y_user);
    return error;
}
unsafe extern "C-unwind" fn do_texfig_operator(
    mut opcode: ::core::ffi::c_int,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    static mut fig_p: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    static mut in_tfig: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut xobj_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    static mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut values: [::core::ffi::c_double; 6] = [0.; 6];
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    match opcode {
        STEXFIG => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                6 as ::core::ffi::c_int,
            );
            if error == 0 {
                let mut dvi2pts: ::core::ffi::c_double = 0.;
                let mut resname: [::core::ffi::c_char; 256] = [0; 256];
                transform_info_clear(&raw mut fig_p);
                dvi2pts = 1.0f64 / dev_unit_dviunit();
                fig_p.width = values[0 as ::core::ffi::c_int as usize] * dvi2pts;
                fig_p.height = values[1 as ::core::ffi::c_int as usize] * dvi2pts;
                fig_p.bbox.llx = values[2 as ::core::ffi::c_int as usize] * dvi2pts;
                fig_p.bbox.lly = -values[3 as ::core::ffi::c_int as usize] * dvi2pts;
                fig_p.bbox.urx = values[4 as ::core::ffi::c_int as usize] * dvi2pts;
                fig_p.bbox.ury = -values[5 as ::core::ffi::c_int as usize] * dvi2pts;
                fig_p.flags |= INFO_HAS_USER_BBOX;
                sprintf(
                    &raw mut resname as *mut ::core::ffi::c_char,
                    b"__tf%d__\0" as *const u8 as *const ::core::ffi::c_char,
                    count,
                );
                xobj_id = pdf_doc_begin_grabbing(
                    &raw mut resname as *mut ::core::ffi::c_char,
                    fig_p.bbox.llx,
                    fig_p.bbox.ury,
                    &raw mut fig_p.bbox,
                );
                in_tfig = 1 as ::core::ffi::c_int;
                count += 1;
            }
        }
        ETEXFIG => {
            if in_tfig == 0 {
                pdf_abort!(
                    b"endTexFig without valid startTexFig!.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            pdf_doc_end_grabbing(::core::ptr::null_mut::<pdf_obj>());
            pdf_dev_put_image(
                xobj_id,
                &raw mut fig_p,
                x_user,
                y_user,
                ::core::ptr::null_mut::<pdf_rect>(),
            );
            in_tfig = 0 as ::core::ffi::c_int;
        }
        _ => {
            error = 1 as ::core::ffi::c_int;
        }
    }
    return error;
}
pub const DEVICE_RESOLUTION: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn ps_dev_CTM(mut M: *mut pdf_tmatrix) -> ::core::ffi::c_int {
    pdf_dev_currentmatrix(M);
    (*M).a *= DEVICE_RESOLUTION as ::core::ffi::c_double;
    (*M).b *= DEVICE_RESOLUTION as ::core::ffi::c_double;
    (*M).c *= DEVICE_RESOLUTION as ::core::ffi::c_double;
    (*M).d *= DEVICE_RESOLUTION as ::core::ffi::c_double;
    (*M).e *= DEVICE_RESOLUTION as ::core::ffi::c_double;
    (*M).f *= DEVICE_RESOLUTION as ::core::ffi::c_double;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn do_operator(
    mut token: *const ::core::ffi::c_char,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut opcode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut values: [::core::ffi::c_double; 12] = [0.; 12];
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut matrix: pdf_tmatrix = pdf_tmatrix {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        e: 0.,
        f: 0.,
    };
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut color: pdf_color = pdf_color {
        res_id: 0,
        type_0: 0,
        num_components: 0,
        spot_color_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        values: [0.; 32],
        pattern_id: 0,
    };
    opcode = get_opcode(token);
    let mut current_block_294: u64;
    match opcode {
        ADD => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            if error == 0 {
                if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                    let fresh5 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh5 as usize] = pdf_new_number(
                        values[0 as ::core::ffi::c_int as usize]
                            + values[1 as ::core::ffi::c_int as usize],
                    );
                } else {
                    pdf_warning!(
                        b"PS stack overflow including MetaPost file or inline PS code\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    error = 1 as ::core::ffi::c_int;
                }
            }
        }
        MUL => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            if error == 0 {
                if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                    let fresh6 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh6 as usize] = pdf_new_number(
                        values[0 as ::core::ffi::c_int as usize]
                            * values[1 as ::core::ffi::c_int as usize],
                    );
                } else {
                    pdf_warning!(
                        b"PS stack overflow including MetaPost file or inline PS code\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    error = 1 as ::core::ffi::c_int;
                }
            }
        }
        NEG => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if error == 0 {
                if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                    let fresh7 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh7 as usize] =
                        pdf_new_number(-values[0 as ::core::ffi::c_int as usize]);
                } else {
                    pdf_warning!(
                        b"PS stack overflow including MetaPost file or inline PS code\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    error = 1 as ::core::ffi::c_int;
                }
            }
        }
        SUB => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            if error == 0 {
                if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                    let fresh8 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh8 as usize] = pdf_new_number(
                        values[0 as ::core::ffi::c_int as usize]
                            - values[1 as ::core::ffi::c_int as usize],
                    );
                } else {
                    pdf_warning!(
                        b"PS stack overflow including MetaPost file or inline PS code\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    error = 1 as ::core::ffi::c_int;
                }
            }
        }
        DIV => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            if error == 0 {
                if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                    let fresh9 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh9 as usize] = pdf_new_number(
                        values[0 as ::core::ffi::c_int as usize]
                            / values[1 as ::core::ffi::c_int as usize],
                    );
                } else {
                    pdf_warning!(
                        b"PS stack overflow including MetaPost file or inline PS code\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    error = 1 as ::core::ffi::c_int;
                }
            }
        }
        TRUNCATE => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if error == 0 {
                if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                    let fresh10 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh10 as usize] = pdf_new_number(
                        if values[0 as ::core::ffi::c_int as usize]
                            > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                        {
                            floor(values[0 as ::core::ffi::c_int as usize])
                        } else {
                            ceil(values[0 as ::core::ffi::c_int as usize])
                        },
                    );
                } else {
                    pdf_warning!(
                        b"PS stack overflow including MetaPost file or inline PS code\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    error = 1 as ::core::ffi::c_int;
                }
            }
        }
        CLEAR => {
            error = do_clear();
        }
        POP => {
            tmp = if top_stack > 0 as ::core::ffi::c_uint {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                ::core::ptr::null_mut::<pdf_obj>()
            };
            pdf_release_obj(tmp);
        }
        EXCH => {
            error = do_exch();
        }
        MOVETO => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_moveto(
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                );
            }
        }
        RMOVETO => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_rmoveto(
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                );
            }
        }
        LINETO => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_lineto(
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                );
            }
        }
        RLINETO => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_rlineto(
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                );
            }
        }
        CURVETO => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                6 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_curveto(
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                    values[2 as ::core::ffi::c_int as usize],
                    values[3 as ::core::ffi::c_int as usize],
                    values[4 as ::core::ffi::c_int as usize],
                    values[5 as ::core::ffi::c_int as usize],
                );
            }
        }
        RCURVETO => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                6 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_rcurveto(
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                    values[2 as ::core::ffi::c_int as usize],
                    values[3 as ::core::ffi::c_int as usize],
                    values[4 as ::core::ffi::c_int as usize],
                    values[5 as ::core::ffi::c_int as usize],
                );
            }
        }
        CLOSEPATH => {
            error = pdf_dev_closepath();
        }
        ARC => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                5 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_arc(
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                    values[2 as ::core::ffi::c_int as usize],
                    values[3 as ::core::ffi::c_int as usize],
                    values[4 as ::core::ffi::c_int as usize],
                );
            }
        }
        ARCN => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                5 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_arcn(
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                    values[2 as ::core::ffi::c_int as usize],
                    values[3 as ::core::ffi::c_int as usize],
                    values[4 as ::core::ffi::c_int as usize],
                );
            }
        }
        NEWPATH => {
            pdf_dev_newpath();
        }
        STROKE => {
            pdf_dev_flushpath('S' as i32 as ::core::ffi::c_char, PDF_FILL_RULE_NONZERO);
        }
        FILL => {
            pdf_dev_flushpath('f' as i32 as ::core::ffi::c_char, PDF_FILL_RULE_NONZERO);
        }
        CLIP => {
            error = pdf_dev_clip();
        }
        EOCLIP => {
            error = pdf_dev_eoclip();
        }
        GSAVE => {
            error = pdf_dev_gsave();
            save_font();
        }
        GRESTORE => {
            error = pdf_dev_grestore();
            restore_font();
        }
        CONCAT => {
            tmp = if top_stack > 0 as ::core::ffi::c_uint {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                ::core::ptr::null_mut::<pdf_obj>()
            };
            error = cvr_array(
                tmp,
                &raw mut values as *mut ::core::ffi::c_double,
                6 as ::core::ffi::c_int,
            );
            tmp = ::core::ptr::null_mut::<pdf_obj>();
            if error != 0 {
                pdf_warning!(
                    b"Missing array before \"concat\".\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            } else {
                matrix.a = values[0 as ::core::ffi::c_int as usize];
                matrix.b = values[1 as ::core::ffi::c_int as usize];
                matrix.c = values[2 as ::core::ffi::c_int as usize];
                matrix.d = values[3 as ::core::ffi::c_int as usize];
                matrix.e = values[4 as ::core::ffi::c_int as usize];
                matrix.f = values[5 as ::core::ffi::c_int as usize];
                error = pdf_dev_concat(&raw mut matrix);
            }
        }
        SCALE => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            if error == 0 {
                match mp_cmode {
                    _ => {}
                }
                matrix.a = values[0 as ::core::ffi::c_int as usize];
                matrix.b = 0.0f64;
                matrix.c = 0.0f64;
                matrix.d = values[1 as ::core::ffi::c_int as usize];
                matrix.e = 0.0f64;
                matrix.f = 0.0f64;
                error = pdf_dev_concat(&raw mut matrix);
            }
        }
        ROTATE => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if error == 0 {
                values[0 as ::core::ffi::c_int as usize] = values[0 as ::core::ffi::c_int as usize]
                    * M_PI
                    / 180 as ::core::ffi::c_int as ::core::ffi::c_double;
                match mp_cmode {
                    MP_CMODE_DVIPSK | MP_CMODE_MPOST => {
                        matrix.a = cos(values[0 as ::core::ffi::c_int as usize]);
                        matrix.b = -sin(values[0 as ::core::ffi::c_int as usize]);
                        matrix.c = sin(values[0 as ::core::ffi::c_int as usize]);
                        matrix.d = cos(values[0 as ::core::ffi::c_int as usize]);
                        matrix.e = 0.0f64;
                        matrix.f = 0.0f64;
                    }
                    _ => {
                        matrix.a = cos(values[0 as ::core::ffi::c_int as usize]);
                        matrix.b = sin(values[0 as ::core::ffi::c_int as usize]);
                        matrix.c = -sin(values[0 as ::core::ffi::c_int as usize]);
                        matrix.d = cos(values[0 as ::core::ffi::c_int as usize]);
                        matrix.e = 0.0f64;
                        matrix.f = 0.0f64;
                    }
                }
                error = pdf_dev_concat(&raw mut matrix);
            }
        }
        TRANSLATE => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            if error == 0 {
                matrix.a = 1.0f64;
                matrix.b = 0.0f64;
                matrix.c = 0.0f64;
                matrix.d = 1.0f64;
                matrix.e = values[0 as ::core::ffi::c_int as usize];
                matrix.f = values[1 as ::core::ffi::c_int as usize];
                error = pdf_dev_concat(&raw mut matrix);
            }
        }
        SETDASH => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if error == 0 {
                let mut pattern: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                let mut dash: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                let mut i: ::core::ffi::c_int = 0;
                let mut num_dashes: ::core::ffi::c_int = 0;
                let mut dash_values: [::core::ffi::c_double; 16] = [0.; 16];
                let mut offset: ::core::ffi::c_double = 0.;
                offset = values[0 as ::core::ffi::c_int as usize];
                pattern = if top_stack > 0 as ::core::ffi::c_uint {
                    top_stack = top_stack.wrapping_sub(1);
                    stack[top_stack as usize]
                } else {
                    ::core::ptr::null_mut::<pdf_obj>()
                };
                if !(!pattern.is_null() && pdf_obj_typeof(pattern) == PDF_ARRAY) {
                    pdf_release_obj(pattern);
                    error = 1 as ::core::ffi::c_int;
                } else {
                    num_dashes = pdf_array_length(pattern) as ::core::ffi::c_int;
                    if num_dashes > PDF_DASH_SIZE_MAX {
                        pdf_warning!(
                            b"Too many dashes...\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        pdf_release_obj(pattern);
                        error = 1 as ::core::ffi::c_int;
                    } else {
                        i = 0 as ::core::ffi::c_int;
                        while i < num_dashes && error == 0 {
                            dash = pdf_get_array(pattern, i);
                            if !(!dash.is_null() && pdf_obj_typeof(dash) == PDF_NUMBER) {
                                error = 1 as ::core::ffi::c_int;
                            } else {
                                dash_values[i as usize] = pdf_number_value(dash);
                            }
                            i += 1;
                        }
                        pdf_release_obj(pattern);
                        if error == 0 {
                            error = pdf_dev_setdash(
                                num_dashes,
                                &raw mut dash_values as *mut ::core::ffi::c_double,
                                offset,
                            );
                        }
                    }
                }
            }
        }
        SETLINECAP => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_setlinecap(
                    values[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                );
            }
        }
        SETLINEJOIN => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_setlinejoin(
                    values[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                );
            }
        }
        SETLINEWIDTH => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_setlinewidth(values[0 as ::core::ffi::c_int as usize]);
            }
        }
        SETMITERLIMIT => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if error == 0 {
                error = pdf_dev_setmiterlimit(values[0 as ::core::ffi::c_int as usize]);
            }
        }
        SETCMYKCOLOR => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                4 as ::core::ffi::c_int,
            );
            if error == 0 {
                pdf_color_cmykcolor(
                    &raw mut color,
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                    values[2 as ::core::ffi::c_int as usize],
                    values[3 as ::core::ffi::c_int as usize],
                );
                pdf_dev_set_color(
                    &raw mut color,
                    0 as ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                pdf_dev_set_color(
                    &raw mut color,
                    0x20 as ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
            }
        }
        SETGRAY => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            if error == 0 {
                pdf_color_graycolor(&raw mut color, values[0 as ::core::ffi::c_int as usize]);
                pdf_dev_set_color(
                    &raw mut color,
                    0 as ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                pdf_dev_set_color(
                    &raw mut color,
                    0x20 as ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
            }
        }
        SETRGBCOLOR => {
            error = pop_get_numbers(
                &raw mut values as *mut ::core::ffi::c_double,
                3 as ::core::ffi::c_int,
            );
            if error == 0 {
                pdf_color_rgbcolor(
                    &raw mut color,
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                    values[2 as ::core::ffi::c_int as usize],
                );
                pdf_dev_set_color(
                    &raw mut color,
                    0 as ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
                pdf_dev_set_color(
                    &raw mut color,
                    0x20 as ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                );
            }
        }
        SHOWPAGE => {}
        CURRENTPOINT => {
            error = pdf_dev_currentpoint(&raw mut cp);
            if error == 0 {
                if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                    let fresh11 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh11 as usize] = pdf_new_number(cp.x);
                    if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                        let fresh12 = top_stack;
                        top_stack = top_stack.wrapping_add(1);
                        stack[fresh12 as usize] = pdf_new_number(cp.y);
                    } else {
                        pdf_warning!(
                            b"PS stack overflow including MetaPost file or inline PS code\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        error = 1 as ::core::ffi::c_int;
                    }
                } else {
                    pdf_warning!(
                        b"PS stack overflow including MetaPost file or inline PS code\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    error = 1 as ::core::ffi::c_int;
                }
            }
        }
        DTRANSFORM => {
            let mut has_matrix: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            tmp = if top_stack > 0 as ::core::ffi::c_uint {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                ::core::ptr::null_mut::<pdf_obj>()
            };
            if !tmp.is_null() && pdf_obj_typeof(tmp) == PDF_ARRAY {
                error = cvr_array(
                    tmp,
                    &raw mut values as *mut ::core::ffi::c_double,
                    6 as ::core::ffi::c_int,
                );
                tmp = ::core::ptr::null_mut::<pdf_obj>();
                if error != 0 {
                    current_block_294 = 12782750567233516376;
                } else {
                    matrix.a = values[0 as ::core::ffi::c_int as usize];
                    matrix.b = values[1 as ::core::ffi::c_int as usize];
                    matrix.c = values[2 as ::core::ffi::c_int as usize];
                    matrix.d = values[3 as ::core::ffi::c_int as usize];
                    matrix.e = values[4 as ::core::ffi::c_int as usize];
                    matrix.f = values[5 as ::core::ffi::c_int as usize];
                    tmp = if top_stack > 0 as ::core::ffi::c_uint {
                        top_stack = top_stack.wrapping_sub(1);
                        stack[top_stack as usize]
                    } else {
                        ::core::ptr::null_mut::<pdf_obj>()
                    };
                    has_matrix = 1 as ::core::ffi::c_int;
                    current_block_294 = 12015864119786495212;
                }
            } else {
                current_block_294 = 12015864119786495212;
            }
            match current_block_294 {
                12782750567233516376 => {}
                _ => {
                    if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NUMBER) {
                        error = 1 as ::core::ffi::c_int;
                    } else {
                        cp.y = pdf_number_value(tmp);
                        pdf_release_obj(tmp);
                        tmp = if top_stack > 0 as ::core::ffi::c_uint {
                            top_stack = top_stack.wrapping_sub(1);
                            stack[top_stack as usize]
                        } else {
                            ::core::ptr::null_mut::<pdf_obj>()
                        };
                        if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NUMBER) {
                            error = 1 as ::core::ffi::c_int;
                        } else {
                            cp.x = pdf_number_value(tmp);
                            pdf_release_obj(tmp);
                            if has_matrix == 0 {
                                ps_dev_CTM(&raw mut matrix);
                            }
                            pdf_dev_dtransform(&raw mut cp, &raw mut matrix);
                            if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                                let fresh13 = top_stack;
                                top_stack = top_stack.wrapping_add(1);
                                stack[fresh13 as usize] = pdf_new_number(cp.x);
                                if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                                    let fresh14 = top_stack;
                                    top_stack = top_stack.wrapping_add(1);
                                    stack[fresh14 as usize] = pdf_new_number(cp.y);
                                } else {
                                    pdf_warning!(
                                        b"PS stack overflow including MetaPost file or inline PS code\0"
                                        as * const u8 as * const ::core::ffi::c_char,
                                    );
                                    error = 1 as ::core::ffi::c_int;
                                }
                            } else {
                                pdf_warning!(
                                    b"PS stack overflow including MetaPost file or inline PS code\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                error = 1 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
        IDTRANSFORM => {
            let mut has_matrix_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            tmp = if top_stack > 0 as ::core::ffi::c_uint {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                ::core::ptr::null_mut::<pdf_obj>()
            };
            if !tmp.is_null() && pdf_obj_typeof(tmp) == PDF_ARRAY {
                error = cvr_array(
                    tmp,
                    &raw mut values as *mut ::core::ffi::c_double,
                    6 as ::core::ffi::c_int,
                );
                tmp = ::core::ptr::null_mut::<pdf_obj>();
                if error != 0 {
                    current_block_294 = 12782750567233516376;
                } else {
                    matrix.a = values[0 as ::core::ffi::c_int as usize];
                    matrix.b = values[1 as ::core::ffi::c_int as usize];
                    matrix.c = values[2 as ::core::ffi::c_int as usize];
                    matrix.d = values[3 as ::core::ffi::c_int as usize];
                    matrix.e = values[4 as ::core::ffi::c_int as usize];
                    matrix.f = values[5 as ::core::ffi::c_int as usize];
                    tmp = if top_stack > 0 as ::core::ffi::c_uint {
                        top_stack = top_stack.wrapping_sub(1);
                        stack[top_stack as usize]
                    } else {
                        ::core::ptr::null_mut::<pdf_obj>()
                    };
                    has_matrix_0 = 1 as ::core::ffi::c_int;
                    current_block_294 = 13139313344577584540;
                }
            } else {
                current_block_294 = 13139313344577584540;
            }
            match current_block_294 {
                12782750567233516376 => {}
                _ => {
                    if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NUMBER) {
                        error = 1 as ::core::ffi::c_int;
                    } else {
                        cp.y = pdf_number_value(tmp);
                        pdf_release_obj(tmp);
                        tmp = if top_stack > 0 as ::core::ffi::c_uint {
                            top_stack = top_stack.wrapping_sub(1);
                            stack[top_stack as usize]
                        } else {
                            ::core::ptr::null_mut::<pdf_obj>()
                        };
                        if !(!tmp.is_null() && pdf_obj_typeof(tmp) == PDF_NUMBER) {
                            error = 1 as ::core::ffi::c_int;
                        } else {
                            cp.x = pdf_number_value(tmp);
                            pdf_release_obj(tmp);
                            if has_matrix_0 == 0 {
                                ps_dev_CTM(&raw mut matrix);
                            }
                            pdf_dev_idtransform(&raw mut cp, &raw mut matrix);
                            if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                                let fresh15 = top_stack;
                                top_stack = top_stack.wrapping_add(1);
                                stack[fresh15 as usize] = pdf_new_number(cp.x);
                                if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                                    let fresh16 = top_stack;
                                    top_stack = top_stack.wrapping_add(1);
                                    stack[fresh16 as usize] = pdf_new_number(cp.y);
                                } else {
                                    pdf_warning!(
                                        b"PS stack overflow including MetaPost file or inline PS code\0"
                                        as * const u8 as * const ::core::ffi::c_char,
                                    );
                                    error = 1 as ::core::ffi::c_int;
                                }
                            } else {
                                pdf_warning!(
                                    b"PS stack overflow including MetaPost file or inline PS code\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                error = 1 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
        FINDFONT => {
            error = do_findfont();
        }
        SCALEFONT => {
            error = do_scalefont();
        }
        SETFONT => {
            error = do_setfont();
        }
        CURRENTFONT => {
            error = do_currentfont();
        }
        SHOW => {
            error = do_show();
        }
        STRINGWIDTH => {
            error = 1 as ::core::ffi::c_int;
        }
        FSHOW => {
            error = do_mpost_bind_def(
                b"exch findfont exch scalefont setfont show\0" as *const u8
                    as *const ::core::ffi::c_char,
                x_user,
                y_user,
            );
        }
        STEXFIG | ETEXFIG => {
            error = do_texfig_operator(opcode, x_user, y_user);
        }
        HLW => {
            error = do_mpost_bind_def(
                b"0 dtransform exch truncate exch idtransform pop setlinewidth\0" as *const u8
                    as *const ::core::ffi::c_char,
                x_user,
                y_user,
            );
        }
        VLW => {
            error = do_mpost_bind_def(
                b"0 exch dtransform truncate idtransform setlinewidth pop\0" as *const u8
                    as *const ::core::ffi::c_char,
                x_user,
                y_user,
            );
        }
        RD => {
            error = do_mpost_bind_def(
                b"[] 0 setdash\0" as *const u8 as *const ::core::ffi::c_char,
                x_user,
                y_user,
            );
        }
        B => {
            error = do_mpost_bind_def(
                b"gsave fill grestore\0" as *const u8 as *const ::core::ffi::c_char,
                x_user,
                y_user,
            );
        }
        DEF => {
            tmp = if top_stack > 0 as ::core::ffi::c_uint {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                ::core::ptr::null_mut::<pdf_obj>()
            };
            tmp = if top_stack > 0 as ::core::ffi::c_uint {
                top_stack = top_stack.wrapping_sub(1);
                stack[top_stack as usize]
            } else {
                ::core::ptr::null_mut::<pdf_obj>()
            };
        }
        _ => {
            if is_fontname(token) {
                if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                    let fresh17 = top_stack;
                    top_stack = top_stack.wrapping_add(1);
                    stack[fresh17 as usize] = pdf_new_name(token);
                } else {
                    pdf_warning!(
                        b"PS stack overflow including MetaPost file or inline PS code\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    error = 1 as ::core::ffi::c_int;
                }
            } else {
                pdf_warning!(
                    b"Unknown token \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                    token,
                );
                error = 1 as ::core::ffi::c_int;
            }
        }
    }
    return error;
}
unsafe extern "C-unwind" fn mp_parse_body(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut token: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    skip_white(start, end);
    while *start < end && error == 0 {
        if isdigit(**start as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
            || *start < end.offset(-(1 as ::core::ffi::c_int as isize))
                && (**start as ::core::ffi::c_int == '+' as i32
                    || **start as ::core::ffi::c_int == '-' as i32
                    || **start as ::core::ffi::c_int == '.' as i32)
        {
            let mut value: ::core::ffi::c_double = 0.;
            let mut next: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            value = strtod(*start, &raw mut next);
            if next < end as *mut ::core::ffi::c_char
                && strchr(
                    b"<([{/%\0" as *const u8 as *const ::core::ffi::c_char,
                    *next as ::core::ffi::c_int,
                )
                .is_null()
                && isspace(*next as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0
            {
                pdf_warning!(
                    b"Unkown PostScript operator.\0" as *const u8 as *const ::core::ffi::c_char,
                );
                dump(*start, next);
                error = 1 as ::core::ffi::c_int;
            } else if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                let fresh0 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh0 as usize] = pdf_new_number(value);
                *start = next;
            } else {
                pdf_warning!(
                    b"PS stack overflow including MetaPost file or inline PS code\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                error = 1 as ::core::ffi::c_int;
                break;
            }
        } else if **start as ::core::ffi::c_int == '[' as i32 && {
            obj = parse_pdf_array(start, end, ::core::ptr::null_mut::<pdf_file>());
            !obj.is_null()
        } {
            if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                let fresh1 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh1 as usize] = obj;
            } else {
                pdf_warning!(
                    b"PS stack overflow including MetaPost file or inline PS code\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                error = 1 as ::core::ffi::c_int;
                break;
            }
        } else if *start < end.offset(-(1 as ::core::ffi::c_int as isize))
            && (**start as ::core::ffi::c_int == '<' as i32
                && *(*start).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '<' as i32)
            && {
                obj = parse_pdf_dict(start, end, ::core::ptr::null_mut::<pdf_file>());
                !obj.is_null()
            }
        {
            if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                let fresh2 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh2 as usize] = obj;
            } else {
                pdf_warning!(
                    b"PS stack overflow including MetaPost file or inline PS code\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                error = 1 as ::core::ffi::c_int;
                break;
            }
        } else if (**start as ::core::ffi::c_int == '(' as i32
            || **start as ::core::ffi::c_int == '<' as i32)
            && {
                obj = parse_pdf_string(start, end);
                !obj.is_null()
            }
        {
            if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                let fresh3 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh3 as usize] = obj;
            } else {
                pdf_warning!(
                    b"PS stack overflow including MetaPost file or inline PS code\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                error = 1 as ::core::ffi::c_int;
                break;
            }
        } else if **start as ::core::ffi::c_int == '/' as i32 && {
            obj = parse_pdf_name(start, end);
            !obj.is_null()
        } {
            if top_stack < PS_STACK_SIZE as ::core::ffi::c_uint {
                let fresh4 = top_stack;
                top_stack = top_stack.wrapping_add(1);
                stack[fresh4 as usize] = obj;
            } else {
                pdf_warning!(
                    b"PS stack overflow including MetaPost file or inline PS code\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                error = 1 as ::core::ffi::c_int;
                break;
            }
        } else {
            token = parse_ident(start, end);
            if token.is_null() {
                error = 1 as ::core::ffi::c_int;
            } else {
                error = do_operator(token, x_user, y_user);
                free(token as *mut ::core::ffi::c_void);
            }
        }
        skip_white(start, end);
    }
    return error;
}
#[unsafe(export_name = "__pdf_impl_mps_eop_cleanup")]
pub unsafe extern "C-unwind" fn mps_eop_cleanup() {
    clear_fonts();
    do_clear();
}
#[unsafe(export_name = "__pdf_impl_mps_stack_depth")]
pub unsafe extern "C-unwind" fn mps_stack_depth() -> ::core::ffi::c_int {
    return top_stack as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_mps_exec_inline")]
pub unsafe extern "C-unwind" fn mps_exec_inline(
    mut p: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0;
    let mut dirmode: ::core::ffi::c_int = 0;
    let mut autorotate: ::core::ffi::c_int = 0;
    dirmode = pdf_dev_get_dirmode();
    if dirmode != 0 {
        mp_cmode = MP_CMODE_PTEXVERT;
    } else {
        mp_cmode = MP_CMODE_DVIPSK;
    }
    autorotate = pdf_dev_get_param(PDF_DEV_PARAM_AUTOROTATE);
    pdf_dev_set_param(PDF_DEV_PARAM_AUTOROTATE, 0 as ::core::ffi::c_int);
    pdf_dev_moveto(x_user, y_user);
    error = mp_parse_body(p, endptr, x_user, y_user);
    pdf_dev_set_param(PDF_DEV_PARAM_AUTOROTATE, autorotate);
    pdf_dev_set_dirmode(dirmode);
    return error;
}
#[unsafe(export_name = "__pdf_impl_mps_do_page")]
pub unsafe extern "C-unwind" fn mps_do_page(mut image_file: *mut FILE) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bbox: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut size: ::core::ffi::c_int = 0;
    let mut dir_mode: ::core::ffi::c_int = 0;
    rewind(image_file);
    size = file_size(image_file) as ::core::ffi::c_int;
    if size == 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Can't read any byte in the MPS file.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    buffer = new(((size + 1 as ::core::ffi::c_int) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    fread(
        buffer as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        size as size_t,
        image_file,
    );
    *buffer.offset(size as isize) = 0 as ::core::ffi::c_char;
    start = buffer;
    end = buffer.offset(size as isize);
    error = mps_scan_bbox(&raw mut start, end, &raw mut bbox);
    if error != 0 {
        pdf_warning!(
            b"Error occured while scanning MetaPost file headers: Could not find BoundingBox.\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        free(buffer as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    mp_cmode = MP_CMODE_MPOST;
    pdf_doc_begin_page(1.0f64, -Xorigin, -Yorigin);
    pdf_doc_set_mediabox(
        pdf_doc_current_page_number() as ::core::ffi::c_uint,
        &raw mut bbox,
    );
    dir_mode = pdf_dev_get_dirmode();
    pdf_dev_set_param(PDF_DEV_PARAM_AUTOROTATE, 0 as ::core::ffi::c_int);
    skip_prolog(&raw mut start, end);
    error = mp_parse_body(&raw mut start, end, 0.0f64, 0.0f64);
    if error != 0 {
        pdf_warning!(
            b"Errors occured while interpreting MetaPost file.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    pdf_dev_set_param(PDF_DEV_PARAM_AUTOROTATE, 1 as ::core::ffi::c_int);
    pdf_dev_set_dirmode(dir_mode);
    pdf_doc_end_page();
    free(buffer as *mut ::core::ffi::c_void);
    return if error != 0 {
        -(1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(export_name = "__pdf_impl_mps_reset_global_state")]
pub unsafe extern "C-unwind" fn mps_reset_global_state() {
    translate_origin = 0 as ::core::ffi::c_int;
    currentfont = 0 as ::core::ffi::c_int;
    font_stack[0 as ::core::ffi::c_int as usize].font_name =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    font_stack[0 as ::core::ffi::c_int as usize].font_id = -(1 as ::core::ffi::c_int);
    font_stack[0 as ::core::ffi::c_int as usize].tfm_id = -(1 as ::core::ffi::c_int);
    font_stack[0 as ::core::ffi::c_int as usize].subfont_id = -(1 as ::core::ffi::c_int);
    font_stack[0 as ::core::ffi::c_int as usize].pt_size = 0.0f64;
    mp_cmode = MP_CMODE_MPOST;
    top_stack = 0 as ::core::ffi::c_uint;
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct __sFILEX {
    _private: [u8; 0],
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
pub unsafe extern "C" fn __pdf_entry_mps_set_translate_origin(mut v: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { mps_set_translate_origin(v) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_mps_scan_bbox(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut bbox: *mut pdf_rect,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { mps_scan_bbox(pp, endptr, bbox) }) {
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
pub unsafe extern "C" fn __pdf_entry_mps_eop_cleanup() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { mps_eop_cleanup() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_mps_stack_depth(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { mps_stack_depth() }) {
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
pub unsafe extern "C" fn __pdf_entry_mps_exec_inline(
    mut p: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        mps_exec_inline(p, endptr, x_user, y_user)
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
pub unsafe extern "C" fn __pdf_entry_mps_do_page(
    mut image_file: *mut FILE,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { mps_do_page(image_file) }) {
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
pub unsafe extern "C" fn __pdf_entry_mps_reset_global_state() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { mps_reset_global_state() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
