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
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn isprint(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vsnprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        _: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ref_obj"]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_dev_put_image"]
    fn pdf_dev_put_image(
        xobj_id: ::core::ffi::c_int,
        ti: *mut transform_info,
        ref_x: ::core::ffi::c_double,
        ref_y: ::core::ffi::c_double,
        rect: *mut pdf_rect,
    ) -> ::core::ffi::c_int;
    static mut dpx_conf: _dpx_conf;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_dvi_dev_xpos"]
    fn dvi_dev_xpos() -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_dvi_dev_ypos"]
    fn dvi_dev_ypos() -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_dvi_link_annot"]
    fn dvi_link_annot(flag: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_dvi_set_linkmode"]
    fn dvi_set_linkmode(mode: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_dvi_set_phantom_height"]
    fn dvi_set_phantom_height(height: ::core::ffi::c_double, depth: ::core::ffi::c_double);
    #[link_name = "__pdf_impl_dvi_tag_depth"]
    fn dvi_tag_depth();
    #[link_name = "__pdf_impl_dvi_untag_depth"]
    fn dvi_untag_depth();
    #[link_name = "__pdf_impl_dvi_is_tracking_boxes"]
    fn dvi_is_tracking_boxes() -> bool;
    #[link_name = "__pdf_impl_dvi_set_compensation"]
    fn dvi_set_compensation(x: ::core::ffi::c_double, y: ::core::ffi::c_double);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_doc_get_dictionary"]
    fn pdf_doc_get_dictionary(category: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_doc_get_reference"]
    fn pdf_doc_get_reference(category: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_doc_current_page_number"]
    fn pdf_doc_current_page_number() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_doc_current_page_resources"]
    fn pdf_doc_current_page_resources() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_doc_ref_page"]
    fn pdf_doc_ref_page(page_no: ::core::ffi::c_uint) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_doc_begin_grabbing"]
    fn pdf_doc_begin_grabbing(
        ident: *const ::core::ffi::c_char,
        ref_x: ::core::ffi::c_double,
        ref_y: ::core::ffi::c_double,
        cropbox: *const pdf_rect,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_doc_end_grabbing"]
    fn pdf_doc_end_grabbing(attrib: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_doc_begin_annot"]
    fn pdf_doc_begin_annot(dict: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_doc_end_annot"]
    fn pdf_doc_end_annot();
    static mut global_names: *mut ht_table;
    #[link_name = "__pdf_impl_dpx_stack_init"]
    fn dpx_stack_init(st: *mut dpx_stack);
    #[link_name = "__pdf_impl_dpx_stack_pop"]
    fn dpx_stack_pop(st: *mut dpx_stack) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_dpx_stack_push"]
    fn dpx_stack_push(st: *mut dpx_stack, data: *mut ::core::ffi::c_void);
    #[link_name = "__pdf_impl_dpx_stack_depth"]
    fn dpx_stack_depth(st: *mut dpx_stack) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_dpx_stack_top"]
    fn dpx_stack_top(st: *mut dpx_stack) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_names_add_object"]
    fn pdf_names_add_object(
        names: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
        object: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_names_lookup_reference"]
    fn pdf_names_lookup_reference(
        names: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_names_lookup_object"]
    fn pdf_names_lookup_object(
        names: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_names_close_object"]
    fn pdf_names_close_object(
        names: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_dump"]
    fn dump(start: *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_spc_color_check_special"]
    fn spc_color_check_special(
        buffer: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
    ) -> bool;
    #[link_name = "__pdf_impl_spc_color_setup_handler"]
    fn spc_color_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_dvipdfmx_check_special"]
    fn spc_dvipdfmx_check_special(buf: *const ::core::ffi::c_char, len: ::core::ffi::c_int)
    -> bool;
    #[link_name = "__pdf_impl_spc_dvipdfmx_setup_handler"]
    fn spc_dvipdfmx_setup_handler(
        sph: *mut spc_handler,
        spe: *mut spc_env,
        ap: *mut spc_arg,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_dvips_at_begin_document"]
    fn spc_dvips_at_begin_document() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_dvips_at_end_document"]
    fn spc_dvips_at_end_document() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_dvips_at_begin_page"]
    fn spc_dvips_at_begin_page() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_dvips_at_end_page"]
    fn spc_dvips_at_end_page() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_dvips_check_special"]
    fn spc_dvips_check_special(
        buffer: *const ::core::ffi::c_char,
        size: ::core::ffi::c_int,
    ) -> bool;
    #[link_name = "__pdf_impl_spc_dvips_setup_handler"]
    fn spc_dvips_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_html_at_begin_page"]
    fn spc_html_at_begin_page() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_html_at_end_page"]
    fn spc_html_at_end_page() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_html_at_begin_document"]
    fn spc_html_at_begin_document() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_html_at_end_document"]
    fn spc_html_at_end_document() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_html_check_special"]
    fn spc_html_check_special(buffer: *const ::core::ffi::c_char, size: ::core::ffi::c_int)
    -> bool;
    #[link_name = "__pdf_impl_spc_html_setup_handler"]
    fn spc_html_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_misc_check_special"]
    fn spc_misc_check_special(buffer: *const ::core::ffi::c_char, size: ::core::ffi::c_int)
    -> bool;
    #[link_name = "__pdf_impl_spc_misc_setup_handler"]
    fn spc_misc_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_misc_at_begin_document"]
    fn spc_misc_at_begin_document() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_misc_at_end_document"]
    fn spc_misc_at_end_document() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_misc_at_begin_page"]
    fn spc_misc_at_begin_page() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_misc_at_begin_form"]
    fn spc_misc_at_begin_form() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_misc_at_end_form"]
    fn spc_misc_at_end_form() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_pdfm_at_begin_document"]
    fn spc_pdfm_at_begin_document() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_pdfm_at_end_document"]
    fn spc_pdfm_at_end_document() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_pdfm_at_end_page"]
    fn spc_pdfm_at_end_page() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_pdfm_check_special"]
    fn spc_pdfm_check_special(buffer: *const ::core::ffi::c_char, size: ::core::ffi::c_int)
    -> bool;
    #[link_name = "__pdf_impl_spc_pdfm_setup_handler"]
    fn spc_pdfm_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_tpic_at_begin_page"]
    fn spc_tpic_at_begin_page() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_tpic_at_end_page"]
    fn spc_tpic_at_end_page() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_tpic_at_begin_document"]
    fn spc_tpic_at_begin_document() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_tpic_at_end_document"]
    fn spc_tpic_at_end_document() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_tpic_check_special"]
    fn spc_tpic_check_special(buffer: *const ::core::ffi::c_char, size: ::core::ffi::c_int)
    -> bool;
    #[link_name = "__pdf_impl_spc_tpic_setup_handler"]
    fn spc_tpic_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_spc_xtx_check_special"]
    fn spc_xtx_check_special(buffer: *const ::core::ffi::c_char, size: ::core::ffi::c_int) -> bool;
    #[link_name = "__pdf_impl_spc_xtx_setup_handler"]
    fn spc_xtx_setup_handler(
        handle: *mut spc_handler,
        spe: *mut spc_env,
        args: *mut spc_arg,
    ) -> ::core::ffi::c_int;
}
pub type __builtin_va_list = *mut ::core::ffi::c_char;
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type va_list = __builtin_va_list;
use crate::compat::renderer::pdf_coord;
use crate::compat::renderer::pdf_rect;
use crate::compat::renderer::pdf_tmatrix;
use crate::compat::renderer::transform_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_env {
    pub x_user: ::core::ffi::c_double,
    pub y_user: ::core::ffi::c_double,
    pub mag: ::core::ffi::c_double,
    pub pg: ::core::ffi::c_int,
    pub info: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub is_drawable: ::core::ffi::c_int,
    pub rect: pdf_rect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_arg {
    pub curptr: *const ::core::ffi::c_char,
    pub endptr: *const ::core::ffi::c_char,
    pub base: *const ::core::ffi::c_char,
    pub command: *const ::core::ffi::c_char,
}
pub type spc_handler_fn_ptr =
    Option<unsafe extern "C-unwind" fn(*mut spc_env, *mut spc_arg) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const ::core::ffi::c_char,
    pub exec: spc_handler_fn_ptr,
}
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
    pub bodhk_func: Option<unsafe extern "C-unwind" fn() -> ::core::ffi::c_int>,
    pub eodhk_func: Option<unsafe extern "C-unwind" fn() -> ::core::ffi::c_int>,
    pub bophk_func: Option<unsafe extern "C-unwind" fn() -> ::core::ffi::c_int>,
    pub eophk_func: Option<unsafe extern "C-unwind" fn() -> ::core::ffi::c_int>,
    pub bofhk_func: Option<unsafe extern "C-unwind" fn() -> ::core::ffi::c_int>,
    pub eofhk_func: Option<unsafe extern "C-unwind" fn() -> ::core::ffi::c_int>,
    pub check_func:
        Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, ::core::ffi::c_int) -> bool>,
    pub setup_func: Option<
        unsafe extern "C-unwind" fn(
            *mut spc_handler,
            *mut spc_env,
            *mut spc_arg,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dpx_stack {
    pub size: ::core::ffi::c_int,
    pub top: *mut stack_elem,
    pub bottom: *mut stack_elem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_elem {
    pub data: *mut ::core::ffi::c_void,
    pub prev: *mut stack_elem,
}
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[unsafe(export_name = "__pdf_impl_spc_begin_annot")]
pub unsafe extern "C-unwind" fn spc_begin_annot(
    mut spe: *mut spc_env,
    mut dict: *mut pdf_obj,
) -> ::core::ffi::c_int {
    pdf_doc_begin_annot(dict);
    dvi_tag_depth();
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_end_annot")]
pub unsafe extern "C-unwind" fn spc_end_annot(mut spe: *mut spc_env) -> ::core::ffi::c_int {
    dvi_untag_depth();
    pdf_doc_end_annot();
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_resume_annot")]
pub unsafe extern "C-unwind" fn spc_resume_annot(mut spe: *mut spc_env) -> ::core::ffi::c_int {
    dvi_link_annot(1 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_suspend_annot")]
pub unsafe extern "C-unwind" fn spc_suspend_annot(mut spe: *mut spc_env) -> ::core::ffi::c_int {
    dvi_link_annot(0 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_is_tracking_boxes")]
pub unsafe extern "C-unwind" fn spc_is_tracking_boxes(mut spe: *mut spc_env) -> ::core::ffi::c_int {
    return dvi_is_tracking_boxes() as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_set_linkmode")]
pub unsafe extern "C-unwind" fn spc_set_linkmode(
    mut spe: *mut spc_env,
    mut mode: ::core::ffi::c_int,
) {
    dvi_set_linkmode(mode);
}
#[unsafe(export_name = "__pdf_impl_spc_set_phantom")]
pub unsafe extern "C-unwind" fn spc_set_phantom(
    mut spe: *mut spc_env,
    mut height: ::core::ffi::c_double,
    mut depth: ::core::ffi::c_double,
) {
    dvi_set_phantom_height(height, depth);
}
const RKEYS: [*const ::core::ffi::c_char; 11] = [
    b"xpos\0" as *const u8 as *const ::core::ffi::c_char,
    b"ypos\0" as *const u8 as *const ::core::ffi::c_char,
    b"thispage\0" as *const u8 as *const ::core::ffi::c_char,
    b"prevpage\0" as *const u8 as *const ::core::ffi::c_char,
    b"nextpage\0" as *const u8 as *const ::core::ffi::c_char,
    b"resources\0" as *const u8 as *const ::core::ffi::c_char,
    b"pages\0" as *const u8 as *const ::core::ffi::c_char,
    b"names\0" as *const u8 as *const ::core::ffi::c_char,
    b"catalog\0" as *const u8 as *const ::core::ffi::c_char,
    b"docinfo\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
pub const K_OBJ__XPOS: ::core::ffi::c_int = 0;
pub const K_OBJ__YPOS: ::core::ffi::c_int = 1;
pub const K_OBJ__THISPAGE: ::core::ffi::c_int = 2;
pub const K_OBJ__PREVPAGE: ::core::ffi::c_int = 3;
pub const K_OBJ__NEXTPAGE: ::core::ffi::c_int = 4;
pub const K_OBJ__RESOURCES: ::core::ffi::c_int = 5;
pub const K_OBJ__PAGES: ::core::ffi::c_int = 6;
pub const K_OBJ__NAMES: ::core::ffi::c_int = 7;
pub const K_OBJ__CATALOG: ::core::ffi::c_int = 8;
pub const K_OBJ__DOCINFO: ::core::ffi::c_int = 9;
unsafe extern "C-unwind" fn ispageref(mut key: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if strlen(key) <= strlen(b"page\0" as *const u8 as *const ::core::ffi::c_char)
        || memcmp(
            key as *const ::core::ffi::c_void,
            b"page\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"page\0" as *const u8 as *const ::core::ffi::c_char),
        ) != 0
    {
        return 0 as ::core::ffi::c_int;
    } else {
        p = key.offset(4 as ::core::ffi::c_int as isize);
        while *p as ::core::ffi::c_int != 0
            && *p as ::core::ffi::c_int >= '0' as i32
            && *p as ::core::ffi::c_int <= '9' as i32
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int != '\0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_spc_lookup_reference")]
pub unsafe extern "C-unwind" fn spc_lookup_reference(
    mut key: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut value: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut k: ::core::ffi::c_int = 0;
    if key.is_null() {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    k = 0 as ::core::ffi::c_int;
    while !RKEYS[k as usize].is_null() && strcmp(key, RKEYS[k as usize]) != 0 {
        k += 1;
    }
    match k {
        K_OBJ__XPOS => {
            cp.x = dvi_dev_xpos();
            cp.y = dvi_dev_ypos();
            pdf_dev_transform(&raw mut cp, ::core::ptr::null::<pdf_tmatrix>());
            value = pdf_new_number(floor(cp.x / 0.01f64 + 0.5f64) * 0.01f64);
        }
        K_OBJ__YPOS => {
            cp.x = dvi_dev_xpos();
            cp.y = dvi_dev_ypos();
            pdf_dev_transform(&raw mut cp, ::core::ptr::null::<pdf_tmatrix>());
            value = pdf_new_number(floor(cp.y / 0.01f64 + 0.5f64) * 0.01f64);
        }
        K_OBJ__THISPAGE => {
            value =
                pdf_doc_get_reference(b"@THISPAGE\0" as *const u8 as *const ::core::ffi::c_char);
        }
        K_OBJ__PREVPAGE => {
            value =
                pdf_doc_get_reference(b"@PREVPAGE\0" as *const u8 as *const ::core::ffi::c_char);
        }
        K_OBJ__NEXTPAGE => {
            value =
                pdf_doc_get_reference(b"@NEXTPAGE\0" as *const u8 as *const ::core::ffi::c_char);
        }
        K_OBJ__PAGES => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Pages\0" as *const u8 as *const ::core::ffi::c_char,
            ));
        }
        K_OBJ__NAMES => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Names\0" as *const u8 as *const ::core::ffi::c_char,
            ));
        }
        K_OBJ__RESOURCES => {
            value = pdf_ref_obj(pdf_doc_current_page_resources());
        }
        K_OBJ__CATALOG => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Catalog\0" as *const u8 as *const ::core::ffi::c_char,
            ));
        }
        K_OBJ__DOCINFO => {
            value = pdf_ref_obj(pdf_doc_get_dictionary(
                b"Info\0" as *const u8 as *const ::core::ffi::c_char,
            ));
        }
        _ => {
            if ispageref(key) != 0 {
                value = pdf_doc_ref_page(
                    atoi(key.offset(4 as ::core::ffi::c_int as isize)) as ::core::ffi::c_uint
                );
            } else {
                value = pdf_names_lookup_reference(
                    global_names as *mut ht_table,
                    key as *const ::core::ffi::c_void,
                    strlen(key) as ::core::ffi::c_int,
                );
            }
        }
    }
    if value.is_null() {
        pdf_abort!(
            b"Object reference %s not exist.\0" as *const u8 as *const ::core::ffi::c_char,
            key,
        );
    }
    return value;
}
#[unsafe(export_name = "__pdf_impl_spc_lookup_object")]
pub unsafe extern "C-unwind" fn spc_lookup_object(
    mut key: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut value: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cp: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut k: ::core::ffi::c_int = 0;
    if key.is_null() {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    k = 0 as ::core::ffi::c_int;
    while !RKEYS[k as usize].is_null() && strcmp(key, RKEYS[k as usize]) != 0 {
        k += 1;
    }
    match k {
        K_OBJ__XPOS => {
            cp.x = dvi_dev_xpos();
            cp.y = dvi_dev_ypos();
            pdf_dev_transform(&raw mut cp, ::core::ptr::null::<pdf_tmatrix>());
            value = pdf_new_number(floor(cp.x / 0.01f64 + 0.5f64) * 0.01f64);
        }
        K_OBJ__YPOS => {
            cp.x = dvi_dev_xpos();
            cp.y = dvi_dev_ypos();
            pdf_dev_transform(&raw mut cp, ::core::ptr::null::<pdf_tmatrix>());
            value = pdf_new_number(floor(cp.y / 0.01f64 + 0.5f64) * 0.01f64);
        }
        K_OBJ__THISPAGE => {
            value =
                pdf_doc_get_dictionary(b"@THISPAGE\0" as *const u8 as *const ::core::ffi::c_char);
        }
        K_OBJ__PAGES => {
            value = pdf_doc_get_dictionary(b"Pages\0" as *const u8 as *const ::core::ffi::c_char);
        }
        K_OBJ__NAMES => {
            value = pdf_doc_get_dictionary(b"Names\0" as *const u8 as *const ::core::ffi::c_char);
        }
        K_OBJ__RESOURCES => {
            value = pdf_doc_current_page_resources();
        }
        K_OBJ__CATALOG => {
            value = pdf_doc_get_dictionary(b"Catalog\0" as *const u8 as *const ::core::ffi::c_char);
        }
        K_OBJ__DOCINFO => {
            value = pdf_doc_get_dictionary(b"Info\0" as *const u8 as *const ::core::ffi::c_char);
        }
        _ => {
            value = pdf_names_lookup_object(
                global_names as *mut ht_table,
                key as *const ::core::ffi::c_void,
                strlen(key) as ::core::ffi::c_int,
            );
        }
    }
    return value;
}
#[unsafe(export_name = "__pdf_impl_spc_push_object")]
pub unsafe extern "C-unwind" fn spc_push_object(
    mut spe: *mut spc_env,
    mut key: *const ::core::ffi::c_char,
    mut value: *mut pdf_obj,
) {
    if key.is_null() || value.is_null() {
        return;
    }
    pdf_names_add_object(
        global_names as *mut ht_table,
        key as *const ::core::ffi::c_void,
        strlen(key) as ::core::ffi::c_int,
        value,
    );
}
#[unsafe(export_name = "__pdf_impl_spc_flush_object")]
pub unsafe extern "C-unwind" fn spc_flush_object(
    mut spe: *mut spc_env,
    mut key: *const ::core::ffi::c_char,
) {
    pdf_names_close_object(
        global_names as *mut ht_table,
        key as *const ::core::ffi::c_void,
        strlen(key) as ::core::ffi::c_int,
    );
}
#[unsafe(export_name = "__pdf_impl_spc_clear_objects")]
pub unsafe extern "C-unwind" fn spc_clear_objects(mut spe: *mut spc_env) {}

#[unsafe(export_name = "__pdf_impl_spc_get_coord")]
pub unsafe extern "C-unwind" fn spc_get_coord(
    mut spe: *mut spc_env,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
) {
    let __pdf_state = unsafe { document_state() };
    if !(!x.is_null() && !y.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"x && y\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if dpx_stack_depth(&raw mut (*__pdf_state).coords) > 0 as ::core::ffi::c_int {
        let mut p: *mut pdf_coord = dpx_stack_top(&raw mut (*__pdf_state).coords) as *mut pdf_coord;
        *x = (*p).x;
        *y = (*p).y;
    } else {
        *y = 0.0f64;
        *x = *y;
    };
}
#[unsafe(export_name = "__pdf_impl_spc_push_coord")]
pub unsafe extern "C-unwind" fn spc_push_coord(
    mut spe: *mut spc_env,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_coord = ::core::ptr::null_mut::<pdf_coord>();
    p = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_coord>() as usize) as uint32_t)
        as *mut pdf_coord;
    (*p).x = x;
    (*p).y = y;
    dpx_stack_push(
        &raw mut (*__pdf_state).coords,
        p as *mut ::core::ffi::c_void,
    );
    dvi_set_compensation(x, y);
}
#[unsafe(export_name = "__pdf_impl_spc_pop_coord")]
pub unsafe extern "C-unwind" fn spc_pop_coord(mut spe: *mut spc_env) {
    let __pdf_state = unsafe { document_state() };
    let mut x: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    let mut p: *mut pdf_coord = ::core::ptr::null_mut::<pdf_coord>();
    p = dpx_stack_pop(&raw mut (*__pdf_state).coords) as *mut pdf_coord;
    if !p.is_null() {
        free(p as *mut ::core::ffi::c_void);
    }
    spc_get_coord(spe, &raw mut x, &raw mut y);
    dvi_set_compensation(x, y);
}

#[unsafe(export_name = "__pdf_impl_spc_set_fixed_point")]
pub unsafe extern "C-unwind" fn spc_set_fixed_point(
    mut spe: *mut spc_env,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_coord = ::core::ptr::null_mut::<pdf_coord>();
    p = dpx_stack_top(&raw mut (*__pdf_state).pt_fixee) as *mut pdf_coord;
    if !p.is_null() {
        (*p).x = x;
        (*p).y = y;
    } else {
        p = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<pdf_coord>() as usize) as uint32_t)
            as *mut pdf_coord;
        (*p).x = x;
        (*p).y = y;
        dpx_stack_push(
            &raw mut (*__pdf_state).pt_fixee,
            p as *mut ::core::ffi::c_void,
        );
    };
}
#[unsafe(export_name = "__pdf_impl_spc_get_fixed_point")]
pub unsafe extern "C-unwind" fn spc_get_fixed_point(
    mut spe: *mut spc_env,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_coord = ::core::ptr::null_mut::<pdf_coord>();
    if !(!x.is_null() && !y.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"x && y\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    p = dpx_stack_top(&raw mut (*__pdf_state).pt_fixee) as *mut pdf_coord;
    if !p.is_null() {
        *x = (*p).x;
        *y = (*p).y;
    } else {
        *x = 0.0f64;
        *y = 0.0f64;
    };
}
#[unsafe(export_name = "__pdf_impl_spc_put_fixed_point")]
pub unsafe extern "C-unwind" fn spc_put_fixed_point(
    mut spe: *mut spc_env,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_coord = ::core::ptr::null_mut::<pdf_coord>();
    p = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_coord>() as usize) as uint32_t)
        as *mut pdf_coord;
    (*p).x = x;
    (*p).y = y;
    dpx_stack_push(
        &raw mut (*__pdf_state).pt_fixee,
        p as *mut ::core::ffi::c_void,
    );
}
#[unsafe(export_name = "__pdf_impl_spc_dup_fixed_point")]
pub unsafe extern "C-unwind" fn spc_dup_fixed_point(mut spe: *mut spc_env) {
    let __pdf_state = unsafe { document_state() };
    let mut p1: *mut pdf_coord = ::core::ptr::null_mut::<pdf_coord>();
    let mut p2: *mut pdf_coord = ::core::ptr::null_mut::<pdf_coord>();
    p1 = dpx_stack_top(&raw mut (*__pdf_state).pt_fixee) as *mut pdf_coord;
    p2 = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_coord>() as usize) as uint32_t)
        as *mut pdf_coord;
    if !p1.is_null() {
        (*p2).x = (*p1).x;
        (*p2).y = (*p1).y;
    } else {
        (*p2).x = 0.0f64;
        (*p2).y = 0.0f64;
    }
    dpx_stack_push(
        &raw mut (*__pdf_state).pt_fixee,
        p2 as *mut ::core::ffi::c_void,
    );
}
#[unsafe(export_name = "__pdf_impl_spc_pop_fixed_point")]
pub unsafe extern "C-unwind" fn spc_pop_fixed_point(mut spe: *mut spc_env) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_coord = ::core::ptr::null_mut::<pdf_coord>();
    p = dpx_stack_pop(&raw mut (*__pdf_state).pt_fixee) as *mut pdf_coord;
    if !p.is_null() {
        free(p as *mut ::core::ffi::c_void);
    }
}
#[unsafe(export_name = "__pdf_impl_spc_clear_fixed_point")]
pub unsafe extern "C-unwind" fn spc_clear_fixed_point(mut spe: *mut spc_env) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut pdf_coord = ::core::ptr::null_mut::<pdf_coord>();
    loop {
        p = dpx_stack_pop(&raw mut (*__pdf_state).pt_fixee) as *mut pdf_coord;
        if p.is_null() {
            break;
        }
        free(p as *mut ::core::ffi::c_void);
    }
}
#[unsafe(export_name = "__pdf_impl_spc_get_current_point")]
pub unsafe extern "C-unwind" fn spc_get_current_point(
    mut spe: *mut spc_env,
    mut cp: *mut pdf_coord,
) {
    let mut xoff: ::core::ffi::c_double = 0.;
    let mut yoff: ::core::ffi::c_double = 0.;
    if spe.is_null() || cp.is_null() {
        return;
    }
    spc_get_coord(spe, &raw mut xoff, &raw mut yoff);
    (*cp).x = (*spe).x_user - xoff;
    (*cp).y = (*spe).y_user - yoff;
}
#[unsafe(export_name = "__pdf_impl_spc_put_image")]
pub unsafe extern "C-unwind" fn spc_put_image(
    mut spe: *mut spc_env,
    mut res_id: ::core::ffi::c_int,
    mut ti: *mut transform_info,
    mut xpos: ::core::ffi::c_double,
    mut ypos: ::core::ffi::c_double,
) {
    let mut xoff: ::core::ffi::c_double = 0.;
    let mut yoff: ::core::ffi::c_double = 0.;
    spc_get_coord(spe, &raw mut xoff, &raw mut yoff);
    pdf_dev_put_image(
        res_id,
        ti,
        xpos - xoff,
        ypos - yoff,
        &raw mut (*spe).info.rect,
    );
    (*spe).info.is_drawable = 1 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn spc_handler_unknown(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> ::core::ffi::c_int {
    if !(!spe.is_null() && !args.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"spe && args\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*args).curptr = (*args).endptr;
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn init_special(
    mut special: *mut spc_handler,
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
    mut p: *const ::core::ffi::c_char,
    mut size: uint32_t,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
    mut mag: ::core::ffi::c_double,
) {
    (*special).key = ::core::ptr::null::<::core::ffi::c_char>();
    (*special).exec = Some(spc_handler_unknown);
    (*spe).x_user = x_user;
    (*spe).y_user = y_user;
    (*spe).mag = mag;
    (*spe).pg = pdf_doc_current_page_number();
    (*spe).info.is_drawable = 0 as ::core::ffi::c_int;
    (*spe).info.rect.llx = 0.0f64;
    (*spe).info.rect.lly = 0.0f64;
    (*spe).info.rect.urx = 0.0f64;
    (*spe).info.rect.ury = 0.0f64;
    (*args).curptr = p;
    (*args).endptr = (*args).curptr.offset(size as isize);
    (*args).base = (*args).curptr;
    (*args).command = ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C-unwind" fn check_garbage(mut args: *mut spc_arg) {
    if args.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"args\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*args).curptr >= (*args).endptr {
        return;
    }
    skip_white(&raw mut (*args).curptr, (*args).endptr);
    if (*args).curptr < (*args).endptr {
        pdf_warning!(
            b"Unparsed material at end of special ignored.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        dump((*args).curptr, (*args).endptr);
    }
}
const KNOWN_SPECIALS: [C2RustUnnamed_0; 9] = [
    C2RustUnnamed_0 {
        key: b"pdf:\0" as *const u8 as *const ::core::ffi::c_char,
        bodhk_func: Some(
            spc_pdfm_at_begin_document as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        eodhk_func: Some(
            spc_pdfm_at_end_document as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        bophk_func: None,
        eophk_func: Some(
            spc_pdfm_at_end_page as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        bofhk_func: None,
        eofhk_func: None,
        check_func: Some(
            spc_pdfm_check_special
                as unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> bool,
        ),
        setup_func: Some(
            spc_pdfm_setup_handler
                as unsafe extern "C-unwind" fn(
                    *mut spc_handler,
                    *mut spc_env,
                    *mut spc_arg,
                ) -> ::core::ffi::c_int,
        ),
    },
    C2RustUnnamed_0 {
        key: b"x:\0" as *const u8 as *const ::core::ffi::c_char,
        bodhk_func: None,
        eodhk_func: None,
        bophk_func: None,
        eophk_func: None,
        bofhk_func: None,
        eofhk_func: None,
        check_func: Some(
            spc_xtx_check_special
                as unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> bool,
        ),
        setup_func: Some(
            spc_xtx_setup_handler
                as unsafe extern "C-unwind" fn(
                    *mut spc_handler,
                    *mut spc_env,
                    *mut spc_arg,
                ) -> ::core::ffi::c_int,
        ),
    },
    C2RustUnnamed_0 {
        key: b"dvipdfmx:\0" as *const u8 as *const ::core::ffi::c_char,
        bodhk_func: None,
        eodhk_func: None,
        bophk_func: None,
        eophk_func: None,
        bofhk_func: None,
        eofhk_func: None,
        check_func: Some(
            spc_dvipdfmx_check_special
                as unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> bool,
        ),
        setup_func: Some(
            spc_dvipdfmx_setup_handler
                as unsafe extern "C-unwind" fn(
                    *mut spc_handler,
                    *mut spc_env,
                    *mut spc_arg,
                ) -> ::core::ffi::c_int,
        ),
    },
    C2RustUnnamed_0 {
        key: b"ps:\0" as *const u8 as *const ::core::ffi::c_char,
        bodhk_func: Some(
            spc_dvips_at_begin_document as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        eodhk_func: Some(
            spc_dvips_at_end_document as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        bophk_func: Some(
            spc_dvips_at_begin_page as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        eophk_func: Some(
            spc_dvips_at_end_page as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        bofhk_func: None,
        eofhk_func: None,
        check_func: Some(
            spc_dvips_check_special
                as unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> bool,
        ),
        setup_func: Some(
            spc_dvips_setup_handler
                as unsafe extern "C-unwind" fn(
                    *mut spc_handler,
                    *mut spc_env,
                    *mut spc_arg,
                ) -> ::core::ffi::c_int,
        ),
    },
    C2RustUnnamed_0 {
        key: b"color\0" as *const u8 as *const ::core::ffi::c_char,
        bodhk_func: None,
        eodhk_func: None,
        bophk_func: None,
        eophk_func: None,
        bofhk_func: None,
        eofhk_func: None,
        check_func: Some(
            spc_color_check_special
                as unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> bool,
        ),
        setup_func: Some(
            spc_color_setup_handler
                as unsafe extern "C-unwind" fn(
                    *mut spc_handler,
                    *mut spc_env,
                    *mut spc_arg,
                ) -> ::core::ffi::c_int,
        ),
    },
    C2RustUnnamed_0 {
        key: b"tpic\0" as *const u8 as *const ::core::ffi::c_char,
        bodhk_func: Some(
            spc_tpic_at_begin_document as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        eodhk_func: Some(
            spc_tpic_at_end_document as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        bophk_func: Some(
            spc_tpic_at_begin_page as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        eophk_func: Some(
            spc_tpic_at_end_page as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        bofhk_func: None,
        eofhk_func: None,
        check_func: Some(
            spc_tpic_check_special
                as unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> bool,
        ),
        setup_func: Some(
            spc_tpic_setup_handler
                as unsafe extern "C-unwind" fn(
                    *mut spc_handler,
                    *mut spc_env,
                    *mut spc_arg,
                ) -> ::core::ffi::c_int,
        ),
    },
    C2RustUnnamed_0 {
        key: b"html:\0" as *const u8 as *const ::core::ffi::c_char,
        bodhk_func: Some(
            spc_html_at_begin_document as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        eodhk_func: Some(
            spc_html_at_end_document as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        bophk_func: Some(
            spc_html_at_begin_page as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        eophk_func: Some(
            spc_html_at_end_page as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        bofhk_func: None,
        eofhk_func: None,
        check_func: Some(
            spc_html_check_special
                as unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> bool,
        ),
        setup_func: Some(
            spc_html_setup_handler
                as unsafe extern "C-unwind" fn(
                    *mut spc_handler,
                    *mut spc_env,
                    *mut spc_arg,
                ) -> ::core::ffi::c_int,
        ),
    },
    C2RustUnnamed_0 {
        key: b"compat\0" as *const u8 as *const ::core::ffi::c_char,
        bodhk_func: Some(
            spc_misc_at_begin_document as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        eodhk_func: Some(
            spc_misc_at_end_document as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        bophk_func: Some(
            spc_misc_at_begin_page as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        eophk_func: None,
        bofhk_func: Some(
            spc_misc_at_begin_form as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        eofhk_func: Some(
            spc_misc_at_end_form as unsafe extern "C-unwind" fn() -> ::core::ffi::c_int,
        ),
        check_func: Some(
            spc_misc_check_special
                as unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> bool,
        ),
        setup_func: Some(
            spc_misc_setup_handler
                as unsafe extern "C-unwind" fn(
                    *mut spc_handler,
                    *mut spc_env,
                    *mut spc_arg,
                ) -> ::core::ffi::c_int,
        ),
    },
    C2RustUnnamed_0 {
        key: ::core::ptr::null::<::core::ffi::c_char>(),
        bodhk_func: None,
        eodhk_func: None,
        bophk_func: None,
        eophk_func: None,
        bofhk_func: None,
        eofhk_func: None,
        check_func: None,
        setup_func: None,
    },
];
#[unsafe(export_name = "__pdf_impl_spc_begin_form")]
pub unsafe extern "C-unwind" fn spc_begin_form(
    mut spe: *mut spc_env,
    mut ident: *const ::core::ffi::c_char,
    mut cp: pdf_coord,
    mut cropbox: *mut pdf_rect,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut xobj_id: ::core::ffi::c_int = 0;
    xobj_id = pdf_doc_begin_grabbing(ident, cp.x, cp.y, cropbox);
    if xobj_id < 0 as ::core::ffi::c_int {
        error = -(1 as ::core::ffi::c_int);
    } else {
        i = 0 as ::core::ffi::c_int;
        while !KNOWN_SPECIALS[i as usize].key.is_null() {
            if KNOWN_SPECIALS[i as usize].bofhk_func.is_some() {
                error = KNOWN_SPECIALS[i as usize]
                    .bofhk_func
                    .expect("non-null function pointer")();
            }
            i += 1;
        }
    }
    return error;
}
#[unsafe(export_name = "__pdf_impl_spc_end_form")]
pub unsafe extern "C-unwind" fn spc_end_form(
    mut spe: *mut spc_env,
    mut attr: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    pdf_doc_end_grabbing(attr);
    i = 0 as ::core::ffi::c_int;
    while !KNOWN_SPECIALS[i as usize].key.is_null() {
        if KNOWN_SPECIALS[i as usize].eofhk_func.is_some() {
            error = KNOWN_SPECIALS[i as usize]
                .eofhk_func
                .expect("non-null function pointer")();
        }
        i += 1;
    }
    return error;
}
#[unsafe(export_name = "__pdf_impl_spc_exec_at_begin_page")]
pub unsafe extern "C-unwind" fn spc_exec_at_begin_page() -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while !KNOWN_SPECIALS[i as usize].key.is_null() {
        if KNOWN_SPECIALS[i as usize].bophk_func.is_some() {
            error = KNOWN_SPECIALS[i as usize]
                .bophk_func
                .expect("non-null function pointer")();
        }
        i = i.wrapping_add(1);
    }
    return error;
}
#[unsafe(export_name = "__pdf_impl_spc_exec_at_end_page")]
pub unsafe extern "C-unwind" fn spc_exec_at_end_page() -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while !KNOWN_SPECIALS[i as usize].key.is_null() {
        if KNOWN_SPECIALS[i as usize].eophk_func.is_some() {
            error = KNOWN_SPECIALS[i as usize]
                .eophk_func
                .expect("non-null function pointer")();
        }
        i = i.wrapping_add(1);
    }
    return error;
}
#[unsafe(export_name = "__pdf_impl_spc_exec_at_begin_document")]
pub unsafe extern "C-unwind" fn spc_exec_at_begin_document() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while !KNOWN_SPECIALS[i as usize].key.is_null() {
        if KNOWN_SPECIALS[i as usize].bodhk_func.is_some() {
            error = KNOWN_SPECIALS[i as usize]
                .bodhk_func
                .expect("non-null function pointer")();
        }
        i = i.wrapping_add(1);
    }
    dpx_stack_init(&raw mut (*__pdf_state).coords);
    dpx_stack_init(&raw mut (*__pdf_state).pt_fixee);
    return error;
}
#[unsafe(export_name = "__pdf_impl_spc_exec_at_end_document")]
pub unsafe extern "C-unwind" fn spc_exec_at_end_document() -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_uint = 0;
    let mut p: *mut pdf_coord = ::core::ptr::null_mut::<pdf_coord>();
    i = 0 as ::core::ffi::c_uint;
    while !KNOWN_SPECIALS[i as usize].key.is_null() {
        if KNOWN_SPECIALS[i as usize].eodhk_func.is_some() {
            error = KNOWN_SPECIALS[i as usize]
                .eodhk_func
                .expect("non-null function pointer")();
        }
        i = i.wrapping_add(1);
    }
    loop {
        p = dpx_stack_pop(&raw mut (*__pdf_state).coords) as *mut pdf_coord;
        if p.is_null() {
            break;
        }
        free(p as *mut ::core::ffi::c_void);
    }
    loop {
        p = dpx_stack_pop(&raw mut (*__pdf_state).pt_fixee) as *mut pdf_coord;
        if p.is_null() {
            break;
        }
        free(p as *mut ::core::ffi::c_void);
    }
    return error;
}
unsafe extern "C-unwind" fn print_error(
    mut name: *const ::core::ffi::c_char,
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ebuf: [::core::ffi::c_char; 64] = [0; 64];
    let mut i: ::core::ffi::c_int = 0;
    let mut pg: ::core::ffi::c_int = (*spe).pg;
    let mut c: pdf_coord = pdf_coord { x: 0., y: 0. };
    c.x = (*spe).x_user;
    c.y = (*spe).y_user;
    pdf_dev_transform(&raw mut c, ::core::ptr::null::<pdf_tmatrix>());
    if !(*ap).command.is_null() && !name.is_null() {
        pdf_warning!(
            b"Interpreting special command %s (%s) failed.\0" as *const u8
                as *const ::core::ffi::c_char,
            (*ap).command,
            name,
        );
        pdf_warning!(
            b">> at page=\"%d\" position=\"(%g, %g)\" (in PDF)\0" as *const u8
                as *const ::core::ffi::c_char,
            pg,
            c.x,
            c.y,
        );
    }
    i = 0 as ::core::ffi::c_int;
    p = (*ap).base;
    while i < 63 as ::core::ffi::c_int && p < (*ap).endptr {
        if isprint(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            let fresh0 = i;
            i = i + 1;
            ebuf[fresh0 as usize] = *p;
        } else {
            if !((i + 4 as ::core::ffi::c_int) < 63 as ::core::ffi::c_int) {
                break;
            }
            i += sprintf(
                (&raw mut ebuf as *mut ::core::ffi::c_char).offset(i as isize),
                b"\\x%02x\0" as *const u8 as *const ::core::ffi::c_char,
                *p as ::core::ffi::c_uchar as ::core::ffi::c_int,
            );
        }
        p = p.offset(1);
    }
    ebuf[i as usize] = '\0' as i32 as ::core::ffi::c_char;
    if (*ap).curptr < (*ap).endptr {
        loop {
            let fresh1 = i;
            i = i - 1;
            if !(fresh1 > 60 as ::core::ffi::c_int) {
                break;
            }
            ebuf[i as usize] = '.' as i32 as ::core::ffi::c_char;
        }
    }
    pdf_warning!(
        b">> xxx \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut ebuf as *mut ::core::ffi::c_char,
    );
    if (*ap).curptr < (*ap).endptr {
        i = 0 as ::core::ffi::c_int;
        p = (*ap).curptr;
        while i < 63 as ::core::ffi::c_int && p < (*ap).endptr {
            if isprint(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
                let fresh2 = i;
                i = i + 1;
                ebuf[fresh2 as usize] = *p;
            } else {
                if !((i + 4 as ::core::ffi::c_int) < 63 as ::core::ffi::c_int) {
                    break;
                }
                i += sprintf(
                    (&raw mut ebuf as *mut ::core::ffi::c_char).offset(i as isize),
                    b"\\x%02x\0" as *const u8 as *const ::core::ffi::c_char,
                    *p as ::core::ffi::c_uchar as ::core::ffi::c_int,
                );
            }
            p = p.offset(1);
        }
        ebuf[i as usize] = '\0' as i32 as ::core::ffi::c_char;
        if (*ap).curptr < (*ap).endptr {
            loop {
                let fresh3 = i;
                i = i - 1;
                if !(fresh3 > 60 as ::core::ffi::c_int) {
                    break;
                }
                ebuf[i as usize] = '.' as i32 as ::core::ffi::c_char;
            }
        }
        pdf_warning!(
            b">> Reading special command stopped around >>%s<<\0" as *const u8
                as *const ::core::ffi::c_char,
            &raw mut ebuf as *mut ::core::ffi::c_char,
        );
        (*ap).curptr = (*ap).endptr;
    }
}
#[unsafe(export_name = "__pdf_impl_spc_exec_special")]
pub unsafe extern "C-unwind" fn spc_exec_special(
    mut buffer: *const ::core::ffi::c_char,
    mut size: int32_t,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
    mut mag: ::core::ffi::c_double,
    mut is_drawable: *mut ::core::ffi::c_int,
    mut rect: *mut pdf_rect,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut found: bool = false;
    let mut spe: spc_env = spc_env {
        x_user: 0.,
        y_user: 0.,
        mag: 0.,
        pg: 0,
        info: C2RustUnnamed {
            is_drawable: 0,
            rect: pdf_rect {
                llx: 0.,
                lly: 0.,
                urx: 0.,
                ury: 0.,
            },
        },
    };
    let mut args: spc_arg = spc_arg {
        curptr: ::core::ptr::null::<::core::ffi::c_char>(),
        endptr: ::core::ptr::null::<::core::ffi::c_char>(),
        base: ::core::ptr::null::<::core::ffi::c_char>(),
        command: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    let mut special: spc_handler = spc_handler {
        key: ::core::ptr::null::<::core::ffi::c_char>(),
        exec: None,
    };
    if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
        pdf_message!(b"Executing special command: \0" as *const u8 as *const ::core::ffi::c_char,);
        dump(buffer, buffer.offset(size as isize));
    }
    init_special(
        &raw mut special,
        &raw mut spe,
        &raw mut args,
        buffer,
        size as uint32_t,
        x_user,
        y_user,
        mag,
    );
    i = 0 as ::core::ffi::c_int;
    while !KNOWN_SPECIALS[i as usize].key.is_null() {
        found = KNOWN_SPECIALS[i as usize]
            .check_func
            .expect("non-null function pointer")(buffer, size as ::core::ffi::c_int);
        if found {
            error = KNOWN_SPECIALS[i as usize]
                .setup_func
                .expect("non-null function pointer")(
                &raw mut special, &raw mut spe, &raw mut args
            );
            if error == 0 {
                error =
                    special.exec.expect("non-null function pointer")(&raw mut spe, &raw mut args);
            }
            if error != 0 {
                print_error(KNOWN_SPECIALS[i as usize].key, &raw mut spe, &raw mut args);
            } else {
                if !is_drawable.is_null() {
                    *is_drawable = spe.info.is_drawable;
                }
                if !rect.is_null() {
                    (*rect).llx = spe.info.rect.llx;
                    (*rect).lly = spe.info.rect.lly;
                    (*rect).urx = spe.info.rect.urx;
                    (*rect).ury = spe.info.rect.ury;
                }
            }
            break;
        } else {
            i += 1;
        }
    }
    check_garbage(&raw mut args);
    return error;
}
unsafe extern "C" {
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_begin_annot(
    mut spe: *mut spc_env,
    mut dict: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_begin_annot(spe, dict) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_end_annot(
    mut spe: *mut spc_env,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_end_annot(spe) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_resume_annot(
    mut spe: *mut spc_env,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_resume_annot(spe) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_suspend_annot(
    mut spe: *mut spc_env,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_suspend_annot(spe) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_is_tracking_boxes(
    mut spe: *mut spc_env,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_is_tracking_boxes(spe) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_set_linkmode(
    mut spe: *mut spc_env,
    mut mode: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_set_linkmode(spe, mode) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_set_phantom(
    mut spe: *mut spc_env,
    mut height: ::core::ffi::c_double,
    mut depth: ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_set_phantom(spe, height, depth) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_lookup_reference(
    mut key: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_lookup_reference(key) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_lookup_object(
    mut key: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_lookup_object(key) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_push_object(
    mut spe: *mut spc_env,
    mut key: *const ::core::ffi::c_char,
    mut value: *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_push_object(spe, key, value) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_flush_object(
    mut spe: *mut spc_env,
    mut key: *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_flush_object(spe, key) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_clear_objects(mut spe: *mut spc_env) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_clear_objects(spe) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_get_coord(
    mut spe: *mut spc_env,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_get_coord(spe, x, y) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_push_coord(
    mut spe: *mut spc_env,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_push_coord(spe, x, y) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_pop_coord(mut spe: *mut spc_env) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_pop_coord(spe) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_set_fixed_point(
    mut spe: *mut spc_env,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_set_fixed_point(spe, x, y) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_get_fixed_point(
    mut spe: *mut spc_env,
    mut x: *mut ::core::ffi::c_double,
    mut y: *mut ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_get_fixed_point(spe, x, y) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_put_fixed_point(
    mut spe: *mut spc_env,
    mut x: ::core::ffi::c_double,
    mut y: ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_put_fixed_point(spe, x, y) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_dup_fixed_point(mut spe: *mut spc_env) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_dup_fixed_point(spe) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_pop_fixed_point(mut spe: *mut spc_env) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_pop_fixed_point(spe) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_clear_fixed_point(mut spe: *mut spc_env) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_clear_fixed_point(spe) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_get_current_point(
    mut spe: *mut spc_env,
    mut cp: *mut pdf_coord,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_get_current_point(spe, cp) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_put_image(
    mut spe: *mut spc_env,
    mut res_id: ::core::ffi::c_int,
    mut ti: *mut transform_info,
    mut xpos: ::core::ffi::c_double,
    mut ypos: ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        spc_put_image(spe, res_id, ti, xpos, ypos)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_spc_begin_form(
    mut spe: *mut spc_env,
    mut ident: *const ::core::ffi::c_char,
    mut cp: pdf_coord,
    mut cropbox: *mut pdf_rect,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_begin_form(spe, ident, cp, cropbox) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_end_form(
    mut spe: *mut spc_env,
    mut attr: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_end_form(spe, attr) }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_exec_at_begin_page(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_exec_at_begin_page() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_exec_at_end_page(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_exec_at_end_page() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_exec_at_begin_document(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_exec_at_begin_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_exec_at_end_document(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { spc_exec_at_end_document() }) {
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
pub unsafe extern "C" fn __pdf_entry_spc_exec_special(
    mut buffer: *const ::core::ffi::c_char,
    mut size: int32_t,
    mut x_user: ::core::ffi::c_double,
    mut y_user: ::core::ffi::c_double,
    mut mag: ::core::ffi::c_double,
    mut is_drawable: *mut ::core::ffi::c_int,
    mut rect: *mut pdf_rect,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        spc_exec_special(buffer, size, x_user, y_user, mag, is_drawable, rect)
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

pub(crate) struct DocumentState {
    coords: dpx_stack,
    pt_fixee: dpx_stack,
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self {
            coords: dpx_stack {
                size: 0,
                top: ::core::ptr::null_mut::<stack_elem>(),
                bottom: ::core::ptr::null_mut::<stack_elem>(),
            },
            pt_fixee: dpx_stack {
                size: 0,
                top: ::core::ptr::null_mut::<stack_elem>(),
                bottom: ::core::ptr::null_mut::<stack_elem>(),
            },
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe {
        (*crate::compat::document::current())
            .backend
            .dpx_specials
            .pointer
    }
}
