/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2007-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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
    fn ttstub_input_open(
        path: *const ::core::ffi::c_char,
        format: ttbc_file_format,
        is_gz: ::core::ffi::c_int,
    ) -> rust_input_handle_t;
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: ::core::ffi::c_int,
    ) -> size_t;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
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
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_merge_dict"]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(
        dict: *mut pdf_obj,
        key: *mut pdf_obj,
        value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_stream_dict"]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_compare_object"]
    fn pdf_compare_object(obj1: *mut pdf_obj, obj2: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_check_for_pdf"]
    fn check_for_pdf(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    static mut global_names: *mut ht_table;
    static mut work_buffer: [::core::ffi::c_char; 0];
    #[link_name = "__pdf_impl_tt_mfgets"]
    fn tt_mfgets(
        buffer: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_int,
        file: rust_input_handle_t,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_bmp_include_image"]
    fn bmp_include_image(
        ximage: *mut pdf_ximage,
        handle: rust_input_handle_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_check_for_bmp"]
    fn check_for_bmp(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    static mut dpx_conf: _dpx_conf;
    #[link_name = "__pdf_impl_dpx_delete_temp_file"]
    fn dpx_delete_temp_file(tmp: *mut ::core::ffi::c_char, force: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_min4"]
    fn min4(
        v1: ::core::ffi::c_double,
        v2: ::core::ffi::c_double,
        v3: ::core::ffi::c_double,
        v4: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_max4"]
    fn max4(
        v1: ::core::ffi::c_double,
        v2: ::core::ffi::c_double,
        v3: ::core::ffi::c_double,
        v4: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pdf_include_page"]
    fn pdf_include_page(
        ximage: *mut pdf_ximage,
        handle: rust_input_handle_t,
        ident: *const ::core::ffi::c_char,
        options: load_options,
    ) -> ::core::ffi::c_int;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_check_for_jpeg"]
    fn check_for_jpeg(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_jpeg_include_image"]
    fn jpeg_include_image(
        ximage: *mut pdf_ximage,
        handle: rust_input_handle_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
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
    #[link_name = "__pdf_impl_pdf_names_close_object"]
    fn pdf_names_close_object(
        names: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_names_reserve"]
    fn pdf_names_reserve(
        names: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_png_include_image"]
    fn png_include_image(
        ximage: *mut pdf_ximage,
        handle: rust_input_handle_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_check_for_png"]
    fn check_for_png(handle: rust_input_handle_t) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
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
use crate::compat::renderer::pdf_coord;
use crate::compat::renderer::pdf_rect;
use crate::compat::renderer::pdf_tmatrix;
use crate::compat::renderer::transform_info;
pub type pdf_page_boundary = ::core::ffi::c_uint;
pub const pdf_page_boundary_bleedbox: pdf_page_boundary = 5;
pub const pdf_page_boundary_trimbox: pdf_page_boundary = 4;
pub const pdf_page_boundary_artbox: pdf_page_boundary = 3;
pub const pdf_page_boundary_cropbox: pdf_page_boundary = 1;
pub const pdf_page_boundary_mediabox: pdf_page_boundary = 2;
pub const pdf_page_boundary__auto: pdf_page_boundary = 0;
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
pub struct ximage_info {
    pub flags: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub bits_per_component: ::core::ffi::c_int,
    pub num_components: ::core::ffi::c_int,
    pub min_dpi: ::core::ffi::c_int,
    pub xdensity: ::core::ffi::c_double,
    pub ydensity: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xform_info {
    pub flags: ::core::ffi::c_int,
    pub bbox: pdf_rect,
    pub matrix: pdf_tmatrix,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_options {
    pub page_no: ::core::ffi::c_int,
    pub bbox_type: pdf_page_boundary,
    pub dict: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_ximage_ {
    pub ident: *mut ::core::ffi::c_char,
    pub res_name: [::core::ffi::c_char; 16],
    pub subtype: ::core::ffi::c_int,
    pub attr: attr_,
    pub filename: *mut ::core::ffi::c_char,
    pub fullname: *mut ::core::ffi::c_char,
    pub reference: *mut pdf_obj,
    pub resource: *mut pdf_obj,
    pub reserved: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct attr_ {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub xdensity: ::core::ffi::c_double,
    pub ydensity: ::core::ffi::c_double,
    pub bbox: pdf_rect,
    pub page_no: ::core::ffi::c_int,
    pub page_count: ::core::ffi::c_int,
    pub bbox_type: ::core::ffi::c_int,
    pub dict: *mut pdf_obj,
    pub tempfile: ::core::ffi::c_char,
}
pub type pdf_ximage = pdf_ximage_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ic_ {
    pub count: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
    pub ximages: *mut pdf_ximage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opt_ {
    pub cmdtmpl: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub keep_cache: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed,
}
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
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
pub const PDF_STREAM: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const INFO_HAS_USER_BBOX: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const INFO_HAS_WIDTH: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const INFO_HAS_HEIGHT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const PDF_XOBJECT_TYPE_FORM: ::core::ffi::c_int = 0;
pub const PDF_XOBJECT_TYPE_IMAGE: ::core::ffi::c_int = 1;
pub const WORK_BUFFER_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const IMAGE_TYPE_UNKNOWN: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const IMAGE_TYPE_PDF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const IMAGE_TYPE_JPEG: ::core::ffi::c_int = 1;
pub const IMAGE_TYPE_PNG: ::core::ffi::c_int = 2;
pub const IMAGE_TYPE_EPS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const IMAGE_TYPE_BMP: ::core::ffi::c_int = 6;
pub const IMAGE_TYPE_JP2: ::core::ffi::c_int = 7;
static mut _opts: opt_ = opt_ {
    cmdtmpl: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
};
static mut _ic: ic_ = ic_ {
    count: 0 as ::core::ffi::c_int,
    capacity: 0 as ::core::ffi::c_int,
    ximages: ::core::ptr::null::<pdf_ximage>() as *mut pdf_ximage,
};
unsafe extern "C-unwind" fn pdf_init_ximage_struct(mut I: *mut pdf_ximage) {
    (*I).ident = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*I).filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*I).fullname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*I).subtype = -(1 as ::core::ffi::c_int);
    memset(
        &raw mut (*I).res_name as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        16 as size_t,
    );
    (*I).reference = ::core::ptr::null_mut::<pdf_obj>();
    (*I).resource = ::core::ptr::null_mut::<pdf_obj>();
    (*I).reserved = 0 as ::core::ffi::c_int;
    (*I).attr.height = 0 as ::core::ffi::c_int;
    (*I).attr.width = (*I).attr.height;
    (*I).attr.ydensity = 1.0f64;
    (*I).attr.xdensity = (*I).attr.ydensity;
    (*I).attr.bbox.lly = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*I).attr.bbox.llx = (*I).attr.bbox.lly;
    (*I).attr.bbox.ury = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*I).attr.bbox.urx = (*I).attr.bbox.ury;
    (*I).attr.page_no = 1 as ::core::ffi::c_int;
    (*I).attr.page_count = 1 as ::core::ffi::c_int;
    (*I).attr.bbox_type = 0 as ::core::ffi::c_int;
    (*I).attr.dict = ::core::ptr::null_mut::<pdf_obj>();
    (*I).attr.tempfile = 0 as ::core::ffi::c_char;
}
unsafe extern "C-unwind" fn pdf_clean_ximage_struct(mut I: *mut pdf_ximage) {
    free((*I).ident as *mut ::core::ffi::c_void);
    free((*I).filename as *mut ::core::ffi::c_void);
    free((*I).fullname as *mut ::core::ffi::c_void);
    pdf_release_obj((*I).reference);
    pdf_release_obj((*I).resource);
    pdf_release_obj((*I).attr.dict);
    pdf_init_ximage_struct(I);
}
#[unsafe(export_name = "__pdf_impl_pdf_init_images")]
pub unsafe extern "C-unwind" fn pdf_init_images() {
    let mut ic: *mut ic_ = &raw mut _ic;
    (*ic).count = 0 as ::core::ffi::c_int;
    (*ic).capacity = 0 as ::core::ffi::c_int;
    (*ic).ximages = ::core::ptr::null_mut::<pdf_ximage>();
}
#[unsafe(export_name = "__pdf_impl_pdf_close_images")]
pub unsafe extern "C-unwind" fn pdf_close_images() {
    let mut ic: *mut ic_ = &raw mut _ic;
    if !(*ic).ximages.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*ic).count {
            let mut I: *mut pdf_ximage = (*ic).ximages.offset(i as isize);
            if (*I).attr.tempfile != 0 {
                if dpx_conf.verbose_level > 1 as ::core::ffi::c_int
                    && dpx_conf.file.keep_cache != 1 as ::core::ffi::c_int
                {
                    pdf_message!(
                        b"pdf_image>> deleting temporary file \"%s\"\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*I).filename,
                    );
                }
                dpx_delete_temp_file((*I).filename, false_0);
                (*I).filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
                dpx_delete_temp_file((*I).fullname, false_0);
                (*I).fullname = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            pdf_clean_ximage_struct(I);
            i += 1;
        }
        (*ic).ximages = mfree((*ic).ximages as *mut ::core::ffi::c_void) as *mut pdf_ximage;
        (*ic).capacity = 0 as ::core::ffi::c_int;
        (*ic).count = (*ic).capacity;
    }
    _opts.cmdtmpl = mfree(_opts.cmdtmpl as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
}
unsafe extern "C-unwind" fn source_image_type(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut format: ::core::ffi::c_int = IMAGE_TYPE_UNKNOWN;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    if check_for_jpeg(handle) != 0 {
        format = IMAGE_TYPE_JPEG;
    } else if check_for_png(handle) != 0 {
        format = IMAGE_TYPE_PNG;
    } else if check_for_bmp(handle) != 0 {
        format = IMAGE_TYPE_BMP;
    } else if check_for_pdf(handle) != 0 {
        format = IMAGE_TYPE_PDF;
    } else if check_for_ps(handle) != 0 {
        format = IMAGE_TYPE_EPS;
    } else {
        pdf_warning!(
            b"Tectonic was unable to detect an image's format\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        format = IMAGE_TYPE_UNKNOWN;
    }
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    return format;
}
unsafe extern "C-unwind" fn load_image(
    mut ident: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut fullname: *const ::core::ffi::c_char,
    mut format: ::core::ffi::c_int,
    mut handle: rust_input_handle_t,
    mut options: load_options,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ic: *mut ic_ = &raw mut _ic;
    let mut id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut reserved: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut I: *mut pdf_ximage = ::core::ptr::null_mut::<pdf_ximage>();
    if !ident.is_null() {
        id = 0 as ::core::ffi::c_int;
        while id < (*ic).count {
            I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
            if !(*I).ident.is_null() && strcmp(ident, (*I).ident) == 0 {
                if (*I).reserved != 0 {
                    reserved = 1 as ::core::ffi::c_int;
                    break;
                }
            }
            id += 1;
        }
    }
    if reserved == 0 {
        id = (*ic).count;
        if (*ic).count >= (*ic).capacity {
            (*ic).capacity += 16 as ::core::ffi::c_int;
            (*ic).ximages = renew(
                (*ic).ximages as *mut ::core::ffi::c_void,
                ((*ic).capacity as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<pdf_ximage>() as usize)
                    as uint32_t,
            ) as *mut pdf_ximage;
        }
        I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
        pdf_init_ximage_struct(I);
        if !ident.is_null() {
            (*I).ident = new(
                (strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                    as uint32_t,
            ) as *mut ::core::ffi::c_char;
            strcpy((*I).ident, ident);
        }
        (*ic).count += 1;
    }
    if !filename.is_null() {
        (*I).filename = new(
            (strlen(filename).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy((*I).filename, filename);
    }
    if !fullname.is_null() {
        (*I).fullname = new(
            (strlen(fullname).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy((*I).fullname, fullname);
    }
    (*I).attr.page_no = options.page_no;
    (*I).attr.bbox_type = options.bbox_type as ::core::ffi::c_int;
    (*I).attr.dict = options.dict;
    match format {
        IMAGE_TYPE_JPEG => {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(b"[JPEG]\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if jpeg_include_image(I, handle) < 0 as ::core::ffi::c_int {
                current_block = 8486807620883072581;
            } else {
                (*I).subtype = PDF_XOBJECT_TYPE_IMAGE;
                current_block = 8347882395825654554;
            }
        }
        IMAGE_TYPE_JP2 => {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(b"[JP2]\0" as *const u8 as *const ::core::ffi::c_char);
            }
            pdf_warning!(
                b"Tectonic: JP2 not yet supported\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block = 8486807620883072581;
        }
        IMAGE_TYPE_PNG => {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(b"[PNG]\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if png_include_image(I, handle) < 0 as ::core::ffi::c_int {
                current_block = 8486807620883072581;
            } else {
                (*I).subtype = PDF_XOBJECT_TYPE_IMAGE;
                current_block = 8347882395825654554;
            }
        }
        IMAGE_TYPE_BMP => {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(b"[BMP]\0" as *const u8 as *const ::core::ffi::c_char);
            }
            if bmp_include_image(I, handle) < 0 as ::core::ffi::c_int {
                current_block = 8486807620883072581;
            } else {
                (*I).subtype = PDF_XOBJECT_TYPE_IMAGE;
                current_block = 8347882395825654554;
            }
        }
        IMAGE_TYPE_PDF => {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(b"[PDF]\0" as *const u8 as *const ::core::ffi::c_char);
            }
            let mut result: ::core::ffi::c_int = pdf_include_page(I, handle, fullname, options);
            if result != 0 as ::core::ffi::c_int {
                current_block = 8486807620883072581;
            } else {
                if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                    pdf_message!(
                        b",Page:%d\0" as *const u8 as *const ::core::ffi::c_char,
                        (*I).attr.page_no,
                    );
                }
                (*I).subtype = PDF_XOBJECT_TYPE_FORM;
                current_block = 8347882395825654554;
            }
        }
        IMAGE_TYPE_EPS => {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(b"[EPS]\0" as *const u8 as *const ::core::ffi::c_char);
            }
            pdf_warning!(
                b"sorry, PostScript images are not supported by Tectonic\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            pdf_warning!(
                b"for details, please see https://github.com/tectonic-typesetting/tectonic/issues/27\0"
                as * const u8 as * const ::core::ffi::c_char,
            );
            current_block = 8486807620883072581;
        }
        _ => {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_message!(b"[UNKNOWN]\0" as *const u8 as *const ::core::ffi::c_char);
            }
            current_block = 8486807620883072581;
        }
    }
    match current_block {
        8486807620883072581 => {
            pdf_clean_ximage_struct(I);
            return -(1 as ::core::ffi::c_int);
        }
        _ => {
            match (*I).subtype {
                PDF_XOBJECT_TYPE_IMAGE => {
                    sprintf(
                        &raw mut (*I).res_name as *mut ::core::ffi::c_char,
                        b"Im%d\0" as *const u8 as *const ::core::ffi::c_char,
                        id,
                    );
                }
                PDF_XOBJECT_TYPE_FORM => {
                    sprintf(
                        &raw mut (*I).res_name as *mut ::core::ffi::c_char,
                        b"Fm%d\0" as *const u8 as *const ::core::ffi::c_char,
                        id,
                    );
                }
                _ => {
                    pdf_abort!(
                        b"Unknown XObject subtype: %d\0" as *const u8 as *const ::core::ffi::c_char,
                        (*I).subtype,
                    );
                }
            }
            return id;
        }
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_load_image")]
pub unsafe extern "C-unwind" fn pdf_ximage_load_image(
    mut ident: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut options: load_options,
) -> ::core::ffi::c_int {
    let mut ic: *mut ic_ = &raw mut _ic;
    let mut i: ::core::ffi::c_int = 0;
    let mut id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut I: *mut pdf_ximage = ::core::ptr::null_mut::<pdf_ximage>();
    let mut format: ::core::ffi::c_int = 0;
    let mut handle: rust_input_handle_t = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*ic).count {
        I = (*ic).ximages.offset(i as isize) as *mut pdf_ximage;
        if !(*I).filename.is_null() && streq_ptr(filename, (*I).filename) as ::core::ffi::c_int != 0
        {
            id = i;
            break;
        } else {
            i += 1;
        }
    }
    if id >= 0 as ::core::ffi::c_int {
        if (*I).attr.page_no == options.page_no
            && pdf_compare_object((*I).attr.dict, options.dict) == 0
            && (*I).attr.bbox_type as ::core::ffi::c_uint
                == options.bbox_type as ::core::ffi::c_uint
        {
            return id;
        }
    }
    handle = ttstub_input_open(filename, TTBC_FILE_FORMAT_PICT, 0 as ::core::ffi::c_int);
    if handle == INVALID_HANDLE {
        if dpx_conf.compat_mode as ::core::ffi::c_uint
            == dpx_mode_compat_mode as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            pdf_warning!(
                b"Error opening image file \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                filename,
            );
        } else {
            pdf_abort!(
                b"Image inclusion failed for \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                filename,
            );
        }
        return -(1 as ::core::ffi::c_int);
    }
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(
            b"(Image:%s\0" as *const u8 as *const ::core::ffi::c_char,
            filename
        );
    }
    format = source_image_type(handle);
    id = load_image(ident, filename, filename, format, handle, options);
    crate::backend::runtime::input_close(handle);
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(b")\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if id < 0 as ::core::ffi::c_int {
        if dpx_conf.compat_mode as ::core::ffi::c_uint
            == dpx_mode_compat_mode as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if format == IMAGE_TYPE_PDF || format == IMAGE_TYPE_EPS {
                pdf_warning!(
                    b"pdf: image inclusion failed for \"%s\" (page=%d).\0" as *const u8
                        as *const ::core::ffi::c_char,
                    filename,
                    options.page_no,
                );
            } else {
                pdf_warning!(
                    b"pdf: image inclusion failed for \"%s\".\0" as *const u8
                        as *const ::core::ffi::c_char,
                    filename,
                );
            }
        } else if format == IMAGE_TYPE_PDF || format == IMAGE_TYPE_EPS {
            pdf_abort!(
                b"pdf: image inclusion failed for \"%s\" (page=%d).\0" as *const u8
                    as *const ::core::ffi::c_char,
                filename,
                options.page_no,
            );
        } else {
            pdf_abort!(
                b"pdf: image inclusion failed for \"%s\".\0" as *const u8
                    as *const ::core::ffi::c_char,
                filename,
            );
        }
    }
    return id;
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_findresource")]
pub unsafe extern "C-unwind" fn pdf_ximage_findresource(
    mut ident: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ic: *mut ic_ = &raw mut _ic;
    let mut id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut I: *mut pdf_ximage = ::core::ptr::null_mut::<pdf_ximage>();
    id = 0 as ::core::ffi::c_int;
    while id < (*ic).count {
        I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
        if !(*I).ident.is_null() && strcmp(ident, (*I).ident) == 0 {
            return id;
        }
        id += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_init_form_info")]
pub unsafe extern "C-unwind" fn pdf_ximage_init_form_info(mut info: *mut xform_info) {
    (*info).flags = 0 as ::core::ffi::c_int;
    (*info).bbox.llx = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*info).bbox.lly = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*info).bbox.urx = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*info).bbox.ury = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    (*info).matrix.a = 1.0f64;
    (*info).matrix.b = 0.0f64;
    (*info).matrix.c = 0.0f64;
    (*info).matrix.d = 1.0f64;
    (*info).matrix.e = 0.0f64;
    (*info).matrix.f = 0.0f64;
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_init_image_info")]
pub unsafe extern "C-unwind" fn pdf_ximage_init_image_info(mut info: *mut ximage_info) {
    (*info).flags = 0 as ::core::ffi::c_int;
    (*info).width = 0 as ::core::ffi::c_int;
    (*info).height = 0 as ::core::ffi::c_int;
    (*info).bits_per_component = 0 as ::core::ffi::c_int;
    (*info).num_components = 0 as ::core::ffi::c_int;
    (*info).min_dpi = 0 as ::core::ffi::c_int;
    (*info).ydensity = 1.0f64;
    (*info).xdensity = (*info).ydensity;
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_set_image")]
pub unsafe extern "C-unwind" fn pdf_ximage_set_image(
    mut I: *mut pdf_ximage,
    mut image_info: *mut ::core::ffi::c_void,
    mut resource: *mut pdf_obj,
) {
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut info: *mut ximage_info = image_info as *mut ximage_info;
    if !(!resource.is_null() && pdf_obj_typeof(resource) == PDF_STREAM) {
        pdf_abort!(
            b"Image XObject must be of stream type.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*I).subtype = PDF_XOBJECT_TYPE_IMAGE;
    (*I).attr.width = (*info).width;
    (*I).attr.height = (*info).height;
    (*I).attr.xdensity = (*info).xdensity;
    (*I).attr.ydensity = (*info).ydensity;
    dict = pdf_stream_dict(resource);
    pdf_add_dict(
        dict,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"XObject\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Image\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Width\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number((*info).width as ::core::ffi::c_double),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Height\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number((*info).height as ::core::ffi::c_double),
    );
    if (*info).bits_per_component > 0 as ::core::ffi::c_int {
        pdf_add_dict(
            dict,
            pdf_new_name(b"BitsPerComponent\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number((*info).bits_per_component as ::core::ffi::c_double),
        );
    }
    if !(*I).attr.dict.is_null() {
        pdf_merge_dict(dict, (*I).attr.dict);
    }
    if !(*I).ident.is_null() {
        let mut error: ::core::ffi::c_int = 0;
        error = pdf_names_add_object(
            global_names as *mut ht_table,
            (*I).ident as *const ::core::ffi::c_void,
            strlen((*I).ident) as ::core::ffi::c_int,
            pdf_link_obj(resource),
        );
        if !(*I).reference.is_null() {
            pdf_release_obj((*I).reference);
        }
        if error != 0 {
            (*I).reference = pdf_ref_obj(resource);
        } else {
            (*I).reference = pdf_names_lookup_reference(
                global_names as *mut ht_table,
                (*I).ident as *const ::core::ffi::c_void,
                strlen((*I).ident) as ::core::ffi::c_int,
            );
            pdf_names_close_object(
                global_names as *mut ht_table,
                (*I).ident as *const ::core::ffi::c_void,
                strlen((*I).ident) as ::core::ffi::c_int,
            );
        }
        (*I).reserved = 0 as ::core::ffi::c_int;
    } else {
        (*I).reference = pdf_ref_obj(resource);
    }
    pdf_release_obj(resource);
    (*I).resource = ::core::ptr::null_mut::<pdf_obj>();
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_set_form")]
pub unsafe extern "C-unwind" fn pdf_ximage_set_form(
    mut I: *mut pdf_ximage,
    mut form_info: *mut ::core::ffi::c_void,
    mut resource: *mut pdf_obj,
) {
    let mut info: *mut xform_info = form_info as *mut xform_info;
    let mut p1: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p2: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p3: pdf_coord = pdf_coord { x: 0., y: 0. };
    let mut p4: pdf_coord = pdf_coord { x: 0., y: 0. };
    (*I).subtype = PDF_XOBJECT_TYPE_FORM;
    p1.x = (*info).bbox.llx;
    p1.y = (*info).bbox.lly;
    pdf_dev_transform(&raw mut p1, &raw mut (*info).matrix);
    p2.x = (*info).bbox.urx;
    p2.y = (*info).bbox.lly;
    pdf_dev_transform(&raw mut p2, &raw mut (*info).matrix);
    p3.x = (*info).bbox.urx;
    p3.y = (*info).bbox.ury;
    pdf_dev_transform(&raw mut p3, &raw mut (*info).matrix);
    p4.x = (*info).bbox.llx;
    p4.y = (*info).bbox.ury;
    pdf_dev_transform(&raw mut p4, &raw mut (*info).matrix);
    (*I).attr.bbox.llx = min4(p1.x, p2.x, p3.x, p4.x);
    (*I).attr.bbox.lly = min4(p1.y, p2.y, p3.y, p4.y);
    (*I).attr.bbox.urx = max4(p1.x, p2.x, p3.x, p4.x);
    (*I).attr.bbox.ury = max4(p1.y, p2.y, p3.y, p4.y);
    if !(*I).ident.is_null() {
        let mut error: ::core::ffi::c_int = 0;
        error = pdf_names_add_object(
            global_names as *mut ht_table,
            (*I).ident as *const ::core::ffi::c_void,
            strlen((*I).ident) as ::core::ffi::c_int,
            pdf_link_obj(resource),
        );
        if !(*I).reference.is_null() {
            pdf_release_obj((*I).reference);
        }
        if error != 0 {
            (*I).reference = pdf_ref_obj(resource);
        } else {
            (*I).reference = pdf_names_lookup_reference(
                global_names as *mut ht_table,
                (*I).ident as *const ::core::ffi::c_void,
                strlen((*I).ident) as ::core::ffi::c_int,
            );
            pdf_names_close_object(
                global_names as *mut ht_table,
                (*I).ident as *const ::core::ffi::c_void,
                strlen((*I).ident) as ::core::ffi::c_int,
            );
        }
        (*I).reserved = 0 as ::core::ffi::c_int;
    } else {
        (*I).reference = pdf_ref_obj(resource);
    }
    pdf_release_obj(resource);
    (*I).resource = ::core::ptr::null_mut::<pdf_obj>();
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_get_page")]
pub unsafe extern "C-unwind" fn pdf_ximage_get_page(mut I: *mut pdf_ximage) -> ::core::ffi::c_int {
    return (*I).attr.page_no;
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_get_reference")]
pub unsafe extern "C-unwind" fn pdf_ximage_get_reference(
    mut id: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut ic: *mut ic_ = &raw mut _ic;
    let mut I: *mut pdf_ximage = ::core::ptr::null_mut::<pdf_ximage>();
    if id < 0 as ::core::ffi::c_int || id >= (*ic).count {
        pdf_abort!(
            b"Invalid XObject ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            id,
        );
    }
    I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    if (*I).reference.is_null() && !(*I).resource.is_null() {
        (*I).reference = pdf_ref_obj((*I).resource);
    }
    return pdf_link_obj((*I).reference);
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_defineresource")]
pub unsafe extern "C-unwind" fn pdf_ximage_defineresource(
    mut ident: *const ::core::ffi::c_char,
    mut subtype: ::core::ffi::c_int,
    mut info: *mut ::core::ffi::c_void,
    mut resource: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut ic: *mut ic_ = &raw mut _ic;
    let mut id: ::core::ffi::c_int = 0;
    let mut reserved: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut I: *mut pdf_ximage = ::core::ptr::null_mut::<pdf_ximage>();
    if !ident.is_null() {
        id = 0 as ::core::ffi::c_int;
        while id < (*ic).count {
            I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
            if !(*I).ident.is_null() && strcmp(ident, (*I).ident) == 0 && (*I).reserved != 0 {
                reserved = 1 as ::core::ffi::c_int;
                break;
            } else {
                id += 1;
            }
        }
    }
    if reserved == 0 {
        id = (*ic).count;
        if (*ic).count >= (*ic).capacity {
            (*ic).capacity += 16 as ::core::ffi::c_int;
            (*ic).ximages = renew(
                (*ic).ximages as *mut ::core::ffi::c_void,
                ((*ic).capacity as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<pdf_ximage>() as usize)
                    as uint32_t,
            ) as *mut pdf_ximage;
        }
        I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
        pdf_init_ximage_struct(I);
        if !ident.is_null() {
            (*I).ident = new(
                (strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                    as uint32_t,
            ) as *mut ::core::ffi::c_char;
            strcpy((*I).ident, ident);
        }
        (*ic).count += 1;
    }
    match subtype {
        PDF_XOBJECT_TYPE_IMAGE => {
            pdf_ximage_set_image(I, info, resource);
            sprintf(
                &raw mut (*I).res_name as *mut ::core::ffi::c_char,
                b"Im%d\0" as *const u8 as *const ::core::ffi::c_char,
                id,
            );
        }
        PDF_XOBJECT_TYPE_FORM => {
            pdf_ximage_set_form(I, info, resource);
            sprintf(
                &raw mut (*I).res_name as *mut ::core::ffi::c_char,
                b"Fm%d\0" as *const u8 as *const ::core::ffi::c_char,
                id,
            );
        }
        _ => {
            pdf_abort!(
                b"Unknown XObject subtype: %d\0" as *const u8 as *const ::core::ffi::c_char,
                subtype,
            );
        }
    }
    return id;
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_reserve")]
pub unsafe extern "C-unwind" fn pdf_ximage_reserve(
    mut ident: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ic: *mut ic_ = &raw mut _ic;
    let mut id: ::core::ffi::c_int = 0;
    let mut I: *mut pdf_ximage = ::core::ptr::null_mut::<pdf_ximage>();
    id = 0 as ::core::ffi::c_int;
    while id < (*ic).count {
        I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
        if !(*I).ident.is_null() && strcmp(ident, (*I).ident) == 0 {
            pdf_warning!(
                b"XObject ID \"%s\" already used!\0" as *const u8 as *const ::core::ffi::c_char,
                ident,
            );
            return -(1 as ::core::ffi::c_int);
        }
        id += 1;
    }
    id = (*ic).count;
    if (*ic).count >= (*ic).capacity {
        (*ic).capacity += 16 as ::core::ffi::c_int;
        (*ic).ximages = renew(
            (*ic).ximages as *mut ::core::ffi::c_void,
            ((*ic).capacity as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<pdf_ximage>() as usize)
                as uint32_t,
        ) as *mut pdf_ximage;
    }
    I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    pdf_init_ximage_struct(I);
    if !ident.is_null() {
        (*I).ident = new(
            (strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy((*I).ident, ident);
    }
    (*I).reference = pdf_names_reserve(
        global_names as *mut ht_table,
        ident as *const ::core::ffi::c_void,
        strlen(ident) as ::core::ffi::c_int,
    );
    sprintf(
        &raw mut (*I).res_name as *mut ::core::ffi::c_char,
        b"Fm%d\0" as *const u8 as *const ::core::ffi::c_char,
        id,
    );
    (*I).reserved = 1 as ::core::ffi::c_int;
    (*ic).count += 1;
    return id;
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_get_resname")]
pub unsafe extern "C-unwind" fn pdf_ximage_get_resname(
    mut id: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut ic: *mut ic_ = &raw mut _ic;
    let mut I: *mut pdf_ximage = ::core::ptr::null_mut::<pdf_ximage>();
    if id < 0 as ::core::ffi::c_int || id >= (*ic).count {
        pdf_abort!(
            b"Invalid XObject ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            id,
        );
    }
    I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    return &raw mut (*I).res_name as *mut ::core::ffi::c_char;
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_get_subtype")]
pub unsafe extern "C-unwind" fn pdf_ximage_get_subtype(
    mut id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ic: *mut ic_ = &raw mut _ic;
    let mut I: *mut pdf_ximage = ::core::ptr::null_mut::<pdf_ximage>();
    if id < 0 as ::core::ffi::c_int || id >= (*ic).count {
        pdf_abort!(
            b"Invalid XObject ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            id,
        );
    }
    I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    return (*I).subtype;
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_set_attr")]
pub unsafe extern "C-unwind" fn pdf_ximage_set_attr(
    mut id: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut xdensity: ::core::ffi::c_double,
    mut ydensity: ::core::ffi::c_double,
    mut llx: ::core::ffi::c_double,
    mut lly: ::core::ffi::c_double,
    mut urx: ::core::ffi::c_double,
    mut ury: ::core::ffi::c_double,
) {
    let mut ic: *mut ic_ = &raw mut _ic;
    let mut I: *mut pdf_ximage = ::core::ptr::null_mut::<pdf_ximage>();
    if id < 0 as ::core::ffi::c_int || id >= (*ic).count {
        pdf_abort!(
            b"Invalid XObject ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            id,
        );
    }
    I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    (*I).attr.width = width;
    (*I).attr.height = height;
    (*I).attr.xdensity = xdensity;
    (*I).attr.ydensity = ydensity;
    (*I).attr.bbox.llx = llx;
    (*I).attr.bbox.lly = lly;
    (*I).attr.bbox.urx = urx;
    (*I).attr.bbox.ury = ury;
}
unsafe extern "C-unwind" fn scale_to_fit_I(
    mut T: *mut pdf_tmatrix,
    mut p: *mut transform_info,
    mut I: *mut pdf_ximage,
) {
    let mut s_x: ::core::ffi::c_double = 0.;
    let mut s_y: ::core::ffi::c_double = 0.;
    let mut d_x: ::core::ffi::c_double = 0.;
    let mut d_y: ::core::ffi::c_double = 0.;
    let mut wd0: ::core::ffi::c_double = 0.;
    let mut ht0: ::core::ffi::c_double = 0.;
    let mut dp: ::core::ffi::c_double = 0.;
    let mut xscale: ::core::ffi::c_double = 0.;
    let mut yscale: ::core::ffi::c_double = 0.;
    if (*p).flags & INFO_HAS_USER_BBOX != 0 {
        wd0 = (*p).bbox.urx - (*p).bbox.llx;
        ht0 = (*p).bbox.ury - (*p).bbox.lly;
        xscale = (*I).attr.width as ::core::ffi::c_double * (*I).attr.xdensity / wd0;
        yscale = (*I).attr.height as ::core::ffi::c_double * (*I).attr.ydensity / ht0;
        d_x = -(*p).bbox.llx / wd0;
        d_y = -(*p).bbox.lly / ht0;
    } else {
        wd0 = (*I).attr.width as ::core::ffi::c_double * (*I).attr.xdensity;
        ht0 = (*I).attr.height as ::core::ffi::c_double * (*I).attr.ydensity;
        yscale = 1.0f64;
        xscale = yscale;
        d_x = 0.0f64;
        d_y = 0.0f64;
    }
    if wd0 == 0.0f64 {
        pdf_warning!(b"Image width=0.0!\0" as *const u8 as *const ::core::ffi::c_char);
        wd0 = 1.0f64;
    }
    if ht0 == 0.0f64 {
        pdf_warning!(b"Image height=0.0!\0" as *const u8 as *const ::core::ffi::c_char);
        ht0 = 1.0f64;
    }
    if (*p).flags & INFO_HAS_WIDTH != 0 && (*p).flags & INFO_HAS_HEIGHT != 0 {
        s_x = (*p).width * xscale;
        s_y = ((*p).height + (*p).depth) * yscale;
        dp = (*p).depth * yscale;
    } else if (*p).flags & INFO_HAS_WIDTH != 0 {
        s_x = (*p).width * xscale;
        s_y = s_x
            * ((*I).attr.height as ::core::ffi::c_double
                / (*I).attr.width as ::core::ffi::c_double);
        dp = 0.0f64;
    } else if (*p).flags & INFO_HAS_HEIGHT != 0 {
        s_y = ((*p).height + (*p).depth) * yscale;
        s_x = s_y
            * ((*I).attr.width as ::core::ffi::c_double
                / (*I).attr.height as ::core::ffi::c_double);
        dp = (*p).depth * yscale;
    } else {
        s_x = wd0;
        s_y = ht0;
        dp = 0.0f64;
    }
    (*T).a = s_x;
    (*T).c = 0.0f64;
    (*T).b = 0.0f64;
    (*T).d = s_y;
    (*T).e = d_x * s_x / xscale;
    (*T).f = d_y * s_y / yscale - dp;
}
unsafe extern "C-unwind" fn scale_to_fit_F(
    mut T: *mut pdf_tmatrix,
    mut p: *mut transform_info,
    mut I: *mut pdf_ximage,
) {
    let mut s_x: ::core::ffi::c_double = 0.;
    let mut s_y: ::core::ffi::c_double = 0.;
    let mut d_x: ::core::ffi::c_double = 0.;
    let mut d_y: ::core::ffi::c_double = 0.;
    let mut wd0: ::core::ffi::c_double = 0.;
    let mut ht0: ::core::ffi::c_double = 0.;
    let mut dp: ::core::ffi::c_double = 0.;
    if (*p).flags & INFO_HAS_USER_BBOX != 0 {
        wd0 = (*p).bbox.urx - (*p).bbox.llx;
        ht0 = (*p).bbox.ury - (*p).bbox.lly;
        d_x = -(*p).bbox.llx;
        d_y = -(*p).bbox.lly;
    } else {
        wd0 = (*I).attr.bbox.urx - (*I).attr.bbox.llx;
        ht0 = (*I).attr.bbox.ury - (*I).attr.bbox.lly;
        d_x = 0.0f64;
        d_y = 0.0f64;
    }
    if wd0 == 0.0f64 {
        pdf_warning!(b"Image width=0.0!\0" as *const u8 as *const ::core::ffi::c_char);
        wd0 = 1.0f64;
    }
    if ht0 == 0.0f64 {
        pdf_warning!(b"Image height=0.0!\0" as *const u8 as *const ::core::ffi::c_char);
        ht0 = 1.0f64;
    }
    if (*p).flags & INFO_HAS_WIDTH != 0 && (*p).flags & INFO_HAS_HEIGHT != 0 {
        s_x = (*p).width / wd0;
        s_y = ((*p).height + (*p).depth) / ht0;
        dp = (*p).depth;
    } else if (*p).flags & INFO_HAS_WIDTH != 0 {
        s_x = (*p).width / wd0;
        s_y = s_x;
        dp = 0.0f64;
    } else if (*p).flags & INFO_HAS_HEIGHT != 0 {
        s_y = ((*p).height + (*p).depth) / ht0;
        s_x = s_y;
        dp = (*p).depth;
    } else {
        s_y = 1.0f64;
        s_x = s_y;
        dp = 0.0f64;
    }
    (*T).a = s_x;
    (*T).c = 0.0f64;
    (*T).b = 0.0f64;
    (*T).d = s_y;
    (*T).e = s_x * d_x;
    (*T).f = s_y * d_y - dp;
}
#[unsafe(export_name = "__pdf_impl_pdf_ximage_scale_image")]
pub unsafe extern "C-unwind" fn pdf_ximage_scale_image(
    mut id: ::core::ffi::c_int,
    mut M: *mut pdf_tmatrix,
    mut r: *mut pdf_rect,
    mut p: *mut transform_info,
) -> ::core::ffi::c_int {
    let mut ic: *mut ic_ = &raw mut _ic;
    let mut I: *mut pdf_ximage = ::core::ptr::null_mut::<pdf_ximage>();
    if id < 0 as ::core::ffi::c_int || id >= (*ic).count {
        pdf_abort!(
            b"Invalid XObject ID: %d\0" as *const u8 as *const ::core::ffi::c_char,
            id,
        );
    }
    I = (*ic).ximages.offset(id as isize) as *mut pdf_ximage;
    (*M).a = 1.0f64;
    (*M).b = 0.0f64;
    (*M).c = 0.0f64;
    (*M).d = 1.0f64;
    (*M).e = 0.0f64;
    (*M).f = 0.0f64;
    match (*I).subtype {
        PDF_XOBJECT_TYPE_IMAGE => {
            scale_to_fit_I(M, p, I);
            if (*p).flags & INFO_HAS_USER_BBOX != 0 {
                (*r).llx =
                    (*p).bbox.llx / ((*I).attr.width as ::core::ffi::c_double * (*I).attr.xdensity);
                (*r).lly = (*p).bbox.lly
                    / ((*I).attr.height as ::core::ffi::c_double * (*I).attr.ydensity);
                (*r).urx =
                    (*p).bbox.urx / ((*I).attr.width as ::core::ffi::c_double * (*I).attr.xdensity);
                (*r).ury = (*p).bbox.ury
                    / ((*I).attr.height as ::core::ffi::c_double * (*I).attr.ydensity);
            } else {
                (*r).llx = 0.0f64;
                (*r).lly = 0.0f64;
                (*r).urx = 1.0f64;
                (*r).ury = 1.0f64;
            }
        }
        PDF_XOBJECT_TYPE_FORM => {
            scale_to_fit_F(M, p, I);
            if (*p).flags & INFO_HAS_USER_BBOX != 0 {
                (*r).llx = (*p).bbox.llx;
                (*r).lly = (*p).bbox.lly;
                (*r).urx = (*p).bbox.urx;
                (*r).ury = (*p).bbox.ury;
            } else {
                (*r).llx = (*I).attr.bbox.llx;
                (*r).lly = (*I).attr.bbox.lly;
                (*r).urx = (*I).attr.bbox.urx;
                (*r).ury = (*I).attr.bbox.ury;
            }
        }
        _ => {
            if (*p).flags & INFO_HAS_USER_BBOX != 0 {
                (*r).llx = (*p).bbox.llx;
                (*r).lly = (*p).bbox.lly;
                (*r).urx = (*p).bbox.urx;
                (*r).ury = (*p).bbox.ury;
            } else {
                (*r).llx = 0.0f64;
                (*r).lly = 0.0f64;
                (*r).urx = 1.0f64;
                (*r).ury = 1.0f64;
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_set_distiller_template")]
pub unsafe extern "C-unwind" fn set_distiller_template(mut s: *mut ::core::ffi::c_char) {
    free(_opts.cmdtmpl as *mut ::core::ffi::c_void);
    if s.is_null() || *s as ::core::ffi::c_int == '\0' as i32 {
        _opts.cmdtmpl = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        _opts.cmdtmpl = new((strlen(s).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        strcpy(_opts.cmdtmpl, s);
    };
}
#[unsafe(export_name = "__pdf_impl_get_distiller_template")]
pub unsafe extern "C-unwind" fn get_distiller_template() -> *mut ::core::ffi::c_char {
    return _opts.cmdtmpl;
}
unsafe extern "C-unwind" fn check_for_ps(mut handle: rust_input_handle_t) -> ::core::ffi::c_int {
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    tt_mfgets(
        &raw mut work_buffer as *mut ::core::ffi::c_char,
        WORK_BUFFER_SIZE,
        handle,
    );
    if !strstartswith(
        &raw mut work_buffer as *mut ::core::ffi::c_char,
        b"%!\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_error_cleanup_cache")]
pub unsafe extern "C-unwind" fn pdf_error_cleanup_cache() {
    let mut ic: *mut ic_ = &raw mut _ic;
    let mut i: ::core::ffi::c_int = 0;
    let mut I: *mut pdf_ximage = ::core::ptr::null_mut::<pdf_ximage>();
    i = 0 as ::core::ffi::c_int;
    while i < (*ic).count {
        I = (*ic).ximages.offset(i as isize) as *mut pdf_ximage;
        if (*I).attr.tempfile != 0 {
            dpx_delete_temp_file((*I).fullname, false_0);
        }
        i += 1;
    }
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
pub unsafe extern "C" fn __pdf_entry_pdf_init_images() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_init_images() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_close_images() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_close_images() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_load_image(
    mut ident: *const ::core::ffi::c_char,
    mut filename: *const ::core::ffi::c_char,
    mut options: load_options,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_ximage_load_image(ident, filename, options)
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
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_findresource(
    mut ident: *const ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_ximage_findresource(ident) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_init_form_info(mut info: *mut xform_info) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_ximage_init_form_info(info) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_init_image_info(mut info: *mut ximage_info) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_ximage_init_image_info(info) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_set_image(
    mut I: *mut pdf_ximage,
    mut image_info: *mut ::core::ffi::c_void,
    mut resource: *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_ximage_set_image(I, image_info, resource)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_set_form(
    mut I: *mut pdf_ximage,
    mut form_info: *mut ::core::ffi::c_void,
    mut resource: *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_ximage_set_form(I, form_info, resource)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_get_page(
    mut I: *mut pdf_ximage,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_ximage_get_page(I) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_get_reference(
    mut id: ::core::ffi::c_int,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_ximage_get_reference(id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_defineresource(
    mut ident: *const ::core::ffi::c_char,
    mut subtype: ::core::ffi::c_int,
    mut info: *mut ::core::ffi::c_void,
    mut resource: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_ximage_defineresource(ident, subtype, info, resource)
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
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_reserve(
    mut ident: *const ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_ximage_reserve(ident) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_get_resname(
    mut id: ::core::ffi::c_int,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_ximage_get_resname(id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_get_subtype(
    mut id: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_ximage_get_subtype(id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_set_attr(
    mut id: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut xdensity: ::core::ffi::c_double,
    mut ydensity: ::core::ffi::c_double,
    mut llx: ::core::ffi::c_double,
    mut lly: ::core::ffi::c_double,
    mut urx: ::core::ffi::c_double,
    mut ury: ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_ximage_set_attr(id, width, height, xdensity, ydensity, llx, lly, urx, ury)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_ximage_scale_image(
    mut id: ::core::ffi::c_int,
    mut M: *mut pdf_tmatrix,
    mut r: *mut pdf_rect,
    mut p: *mut transform_info,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_ximage_scale_image(id, M, r, p) }) {
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
pub unsafe extern "C" fn __pdf_entry_set_distiller_template(
    mut s: *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { set_distiller_template(s) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_get_distiller_template(
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_distiller_template() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_error_cleanup_cache() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_error_cleanup_cache() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
