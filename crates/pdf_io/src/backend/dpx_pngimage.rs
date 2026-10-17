/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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
/*
 * TODO: Check validity
 *
 * Conversion found in
 *
 *  com.sixlegs.image.png - Java package to read and display PNG images
 *  Copyright (C) 1998, 1999, 2001 Chris Nokleberg
 *
 *  http://www.sixlegs.com/software/png/
 *
 */
#![allow(
    warnings,
    clippy::all,
    unsafe_op_in_unsafe_fn,
    static_mut_refs,
    dangerous_implicit_autorefs
)]
unsafe extern "C-unwind" {
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: ::core::ffi::c_int,
    ) -> size_t;
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> ssize_t;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_ref_obj"]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_string"]
    fn pdf_new_string(str: *const ::core::ffi::c_void, length: size_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(
        dict: *mut pdf_obj,
        key: *mut pdf_obj,
        value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_stream"]
    fn pdf_new_stream(flags: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_stream"]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const ::core::ffi::c_void,
        stream_data_len: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_stream_dict"]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_stream_set_predictor"]
    fn pdf_stream_set_predictor(
        stream: *mut pdf_obj,
        predictor: ::core::ffi::c_int,
        columns: int32_t,
        bpc: ::core::ffi::c_int,
        colors: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_check_version"]
    fn pdf_check_version(
        major: ::core::ffi::c_int,
        minor: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_iccp_check_colorspace"]
    fn iccp_check_colorspace(
        colortype: ::core::ffi::c_int,
        profile: *const ::core::ffi::c_void,
        proflen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_iccp_load_profile"]
    fn iccp_load_profile(
        ident: *const ::core::ffi::c_char,
        profile: *const ::core::ffi::c_void,
        proflen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_colorspace_reference"]
    fn pdf_get_colorspace_reference(cspc_id: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_ximage_init_image_info"]
    fn pdf_ximage_init_image_info(info: *mut ximage_info);
    #[link_name = "__pdf_impl_pdf_ximage_set_image"]
    fn pdf_ximage_set_image(
        ximage: *mut pdf_ximage,
        info: *mut ::core::ffi::c_void,
        resource: *mut pdf_obj,
    );
    fn png_sig_cmp(sig: png_const_bytep, start: size_t, num_to_check: size_t)
    -> ::core::ffi::c_int;
    fn png_get_libpng_ver(png_ptr: png_const_structrp) -> png_const_charp;
    fn png_create_read_struct(
        user_png_ver: png_const_charp,
        error_ptr: png_voidp,
        error_fn: png_error_ptr,
        warn_fn: png_error_ptr,
    ) -> png_structp;
    fn png_create_info_struct(png_ptr: png_const_structrp) -> png_infop;
    fn png_read_info(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_set_expand_gray_1_2_4_to_8(png_ptr: png_structrp);
    fn png_set_packing(png_ptr: png_structrp);
    fn png_set_background(
        png_ptr: png_structrp,
        background_color: png_const_color_16p,
        background_gamma_code: ::core::ffi::c_int,
        need_expand: ::core::ffi::c_int,
        background_gamma: ::core::ffi::c_double,
    );
    fn png_set_strip_16(png_ptr: png_structrp);
    fn png_set_gamma(
        png_ptr: png_structrp,
        screen_gamma: ::core::ffi::c_double,
        override_file_gamma: ::core::ffi::c_double,
    );
    fn png_read_update_info(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_read_image(png_ptr: png_structrp, image: png_bytepp);
    fn png_read_end(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_destroy_info_struct(png_ptr: png_const_structrp, info_ptr_ptr: png_infopp);
    fn png_destroy_read_struct(
        png_ptr_ptr: png_structpp,
        info_ptr_ptr: png_infopp,
        end_info_ptr_ptr: png_infopp,
    );
    fn png_set_read_fn(png_ptr: png_structrp, io_ptr: png_voidp, read_data_fn: png_rw_ptr);
    fn png_get_io_ptr(png_ptr: png_const_structrp) -> png_voidp;
    fn png_get_valid(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        flag: png_uint_32,
    ) -> png_uint_32;
    fn png_get_rowbytes(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> size_t;
    fn png_get_image_width(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_uint_32;
    fn png_get_image_height(png_ptr: png_const_structrp, info_ptr: png_const_inforp)
    -> png_uint_32;
    fn png_get_bit_depth(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_byte;
    fn png_get_color_type(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> png_byte;
    fn png_get_x_pixels_per_meter(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_uint_32;
    fn png_get_y_pixels_per_meter(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_uint_32;
    fn png_get_cHRM(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        white_x: *mut ::core::ffi::c_double,
        white_y: *mut ::core::ffi::c_double,
        red_x: *mut ::core::ffi::c_double,
        red_y: *mut ::core::ffi::c_double,
        green_x: *mut ::core::ffi::c_double,
        green_y: *mut ::core::ffi::c_double,
        blue_x: *mut ::core::ffi::c_double,
        blue_y: *mut ::core::ffi::c_double,
    ) -> png_uint_32;
    fn png_get_gAMA(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        file_gamma: *mut ::core::ffi::c_double,
    ) -> png_uint_32;
    fn png_get_PLTE(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        palette: *mut png_colorp,
        num_palette: *mut ::core::ffi::c_int,
    ) -> png_uint_32;
    fn png_get_sRGB(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        file_srgb_intent: *mut ::core::ffi::c_int,
    ) -> png_uint_32;
    fn png_get_iCCP(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        name: png_charpp,
        compression_type: *mut ::core::ffi::c_int,
        profile: png_bytepp,
        proflen: *mut png_uint_32,
    ) -> png_uint_32;
    fn png_get_text(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        text_ptr: *mut png_textp,
        num_text: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn png_get_tRNS(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        trans_alpha: *mut png_bytep,
        num_trans: *mut ::core::ffi::c_int,
        trans_color: *mut png_color_16p,
    ) -> png_uint_32;
    fn png_set_option(
        png_ptr: png_structrp,
        option: ::core::ffi::c_int,
        onoff: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
pub type rust_input_handle_t = uintptr_t;
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
pub type pdf_ximage = pdf_ximage_;
pub type png_byte = ::core::ffi::c_uchar;
pub type png_infopp = *mut *mut png_info;
pub type png_info = png_info_def;
pub type png_structp = *mut png_struct;
pub type png_struct = png_struct_def;
pub type png_structpp = *mut *mut png_struct;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_text = png_text_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_text_struct {
    pub compression: ::core::ffi::c_int,
    pub key: png_charp,
    pub text: png_charp,
    pub text_length: size_t,
    pub itxt_length: size_t,
    pub lang: png_charp,
    pub lang_key: png_charp,
}
pub type png_charp = *mut ::core::ffi::c_char;
pub type png_textp = *mut png_text;
pub type png_bytep = *mut png_byte;
pub type png_uint_32 = ::core::ffi::c_uint;
pub type png_uint_32p = *mut png_uint_32;
pub type png_const_inforp = *const png_info;
pub type png_uint_16 = ::core::ffi::c_ushort;
pub type png_color_16p = *mut png_color_16;
pub type png_color_16 = png_color_16_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub type png_bytepp = *mut *mut png_byte;
pub type png_charpp = *mut *mut ::core::ffi::c_char;
pub type png_color = png_color_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_colorp = *mut png_color;
pub type png_const_color_16p = *const png_color_16;
pub type png_size_t = size_t;
pub type png_voidp = *mut ::core::ffi::c_void;
pub type png_rw_ptr = Option<unsafe extern "C-unwind" fn(png_structp, png_bytep, size_t) -> ()>;
pub type png_const_charp = *const ::core::ffi::c_char;
pub type png_error_ptr = Option<unsafe extern "C-unwind" fn(png_structp, png_const_charp) -> ()>;
pub type png_const_bytep = *const png_byte;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_RGB: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const PDF_COLORSPACE_TYPE_GRAY: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const PNG_DEBUG_STR: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"PNG\0") };
pub const PNG_LIBPNG_VER_STRING: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"1.6.59\0") };
pub const PNG_ITXT_COMPRESSION_NONE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_PALETTE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_COLOR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_ALPHA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_GRAY: ::core::ffi::c_int = 0;
pub const PNG_COLOR_TYPE_PALETTE: ::core::ffi::c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_PALETTE;
pub const PNG_COLOR_TYPE_RGB: ::core::ffi::c_int = 2;
pub const PNG_COLOR_TYPE_RGB_ALPHA: ::core::ffi::c_int = 6;
pub const PNG_COLOR_TYPE_GRAY_ALPHA: ::core::ffi::c_int = 4;
pub const PNG_sRGB_INTENT_PERCEPTUAL: ::core::ffi::c_int = 0;
pub const PNG_sRGB_INTENT_RELATIVE: ::core::ffi::c_int = 1;
pub const PNG_sRGB_INTENT_SATURATION: ::core::ffi::c_int = 2;
pub const PNG_sRGB_INTENT_ABSOLUTE: ::core::ffi::c_int = 3;
pub const PNG_INFO_gAMA: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const PNG_INFO_cHRM: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const PNG_INFO_PLTE: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const PNG_INFO_tRNS: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const PNG_INFO_sRGB: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
pub const PNG_INFO_iCCP: ::core::ffi::c_uint = 0x1000 as ::core::ffi::c_uint;
pub const PNG_BACKGROUND_GAMMA_SCREEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PNG_MAXIMUM_INFLATE_WINDOW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_OPTION_ON: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const DPX_PNG_DEFAULT_GAMMA: ::core::ffi::c_double = 2.2f64;
pub const PDF_TRANS_TYPE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PDF_TRANS_TYPE_BINARY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PDF_TRANS_TYPE_ALPHA: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_check_for_png")]
pub unsafe extern "C-unwind" fn check_for_png(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut sigbytes: [::core::ffi::c_uchar; 8] = [0; 8];
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    if ttstub_input_read(
        handle,
        &raw mut sigbytes as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as size_t,
    ) as usize
        != ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as usize
        || png_sig_cmp(
            &raw mut sigbytes as *mut ::core::ffi::c_uchar as png_const_bytep,
            0 as size_t,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 8]>() as size_t,
        ) != 0
    {
        return 0 as ::core::ffi::c_int;
    } else {
        return 1 as ::core::ffi::c_int;
    };
}
unsafe extern "C-unwind" fn _png_warning_callback(
    mut png_ptr: png_structp,
    mut msg: png_const_charp,
) {
}
unsafe extern "C-unwind" fn _png_read(
    mut png_ptr: png_structp,
    mut outbytes: png_bytep,
    mut n: png_size_t,
) {
    let mut handle: rust_input_handle_t =
        png_get_io_ptr(png_ptr as png_const_structrp) as rust_input_handle_t;
    let mut r: ssize_t = 0;
    r = ttstub_input_read(handle, outbytes as *mut ::core::ffi::c_char, n as size_t);
    if r < 0 as ssize_t || r as size_t != n {
        pdf_abort!(b"error reading PNG\0" as *const u8 as *const ::core::ffi::c_char);
    }
}
#[unsafe(export_name = "__pdf_impl_png_include_image")]
pub unsafe extern "C-unwind" fn png_include_image(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut colorspace: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut mask: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut intent: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_data_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut trans_type: ::core::ffi::c_int = 0;
    let mut info: ximage_info = ximage_info {
        flags: 0,
        width: 0,
        height: 0,
        bits_per_component: 0,
        num_components: 0,
        min_dpi: 0,
        xdensity: 0.,
        ydensity: 0.,
    };
    let mut png_ptr: png_structp = ::core::ptr::null_mut::<png_struct>();
    let mut png_info_ptr: png_infop = ::core::ptr::null_mut::<png_info>();
    let mut bpc: png_byte = 0;
    let mut color_type: png_byte = 0;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut rowbytes: png_uint_32 = 0;
    pdf_ximage_init_image_info(&raw mut info);
    stream = ::core::ptr::null_mut::<pdf_obj>();
    stream_dict = ::core::ptr::null_mut::<pdf_obj>();
    intent = ::core::ptr::null_mut::<pdf_obj>();
    mask = intent;
    colorspace = mask;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    png_ptr = png_create_read_struct(
        png_get_libpng_ver(std::ptr::null()),
        NULL,
        None,
        Some(
            _png_warning_callback
                as unsafe extern "C-unwind" fn(png_structp, png_const_charp) -> (),
        ),
    );
    if png_ptr.is_null() {
        pdf_warning!(
            b"%s: Creating Libpng read struct failed.\0" as *const u8 as *const ::core::ffi::c_char,
            PNG_DEBUG_STR.as_ptr(),
        );
        return -(1 as ::core::ffi::c_int);
    }
    png_info_ptr = png_create_info_struct(png_ptr as png_const_structrp);
    if png_info_ptr.is_null() {
        pdf_warning!(
            b"%s: Creating Libpng info struct failed.\0" as *const u8 as *const ::core::ffi::c_char,
            PNG_DEBUG_STR.as_ptr(),
        );
        png_destroy_read_struct(
            &raw mut png_ptr,
            ::core::ptr::null_mut::<*mut png_info>(),
            ::core::ptr::null_mut::<*mut png_info>(),
        );
        return -(1 as ::core::ffi::c_int);
    }
    png_set_option(
        png_ptr as png_structrp,
        PNG_MAXIMUM_INFLATE_WINDOW,
        PNG_OPTION_ON,
    );
    png_set_read_fn(
        png_ptr as png_structrp,
        handle as png_voidp,
        Some(_png_read as unsafe extern "C-unwind" fn(png_structp, png_bytep, png_size_t) -> ()),
    );
    png_read_info(png_ptr as png_structrp, png_info_ptr as png_inforp);
    color_type = png_get_color_type(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
    );
    width = png_get_image_width(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
    );
    height = png_get_image_height(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
    );
    bpc = png_get_bit_depth(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
    );
    if bpc as ::core::ffi::c_int > 8 as ::core::ffi::c_int {
        if pdf_check_version(1 as ::core::ffi::c_int, 5 as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        {
            pdf_warning!(
                b"%s: 16-bpc PNG requires PDF version 1.5.\0" as *const u8
                    as *const ::core::ffi::c_char,
                PNG_DEBUG_STR.as_ptr(),
            );
            png_set_strip_16(png_ptr as png_structrp);
            bpc = 8 as png_byte;
        }
    } else if (bpc as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        if color_type as ::core::ffi::c_int == PNG_COLOR_TYPE_GRAY
            || color_type as ::core::ffi::c_int == PNG_COLOR_TYPE_GRAY_ALPHA
        {
            png_set_expand_gray_1_2_4_to_8(png_ptr as png_structrp);
        } else {
            png_set_packing(png_ptr as png_structrp);
        }
        bpc = 8 as png_byte;
    }
    if png_get_valid(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
        PNG_INFO_iCCP,
    ) == 0
        && png_get_valid(
            png_ptr as png_const_structrp,
            png_info_ptr as png_const_inforp,
            PNG_INFO_sRGB,
        ) == 0
        && png_get_valid(
            png_ptr as png_const_structrp,
            png_info_ptr as png_const_inforp,
            PNG_INFO_cHRM,
        ) == 0
        && png_get_valid(
            png_ptr as png_const_structrp,
            png_info_ptr as png_const_inforp,
            PNG_INFO_gAMA,
        ) != 0
    {
        let mut G: ::core::ffi::c_double = 1.0f64;
        png_get_gAMA(
            png_ptr as png_const_structrp,
            png_info_ptr as png_const_inforp,
            &raw mut G,
        );
        png_set_gamma(png_ptr as png_structrp, 2.2f64, G);
    }
    trans_type = check_transparency(png_ptr, png_info_ptr);
    png_read_update_info(png_ptr as png_structrp, png_info_ptr as png_inforp);
    rowbytes = png_get_rowbytes(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
    ) as png_uint_32;
    info.width = width as ::core::ffi::c_int;
    info.height = height as ::core::ffi::c_int;
    info.bits_per_component = bpc as ::core::ffi::c_int;
    let mut xppm: png_uint_32 = png_get_x_pixels_per_meter(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
    );
    let mut yppm: png_uint_32 = png_get_y_pixels_per_meter(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
    );
    if xppm > 0 as png_uint_32 {
        info.xdensity = 72.0f64 / 0.0254f64 / xppm as ::core::ffi::c_double;
    }
    if yppm > 0 as png_uint_32 {
        info.ydensity = 72.0f64 / 0.0254f64 / yppm as ::core::ffi::c_double;
    }
    stream = pdf_new_stream(STREAM_COMPRESS);
    stream_dict = pdf_stream_dict(stream);
    stream_data_ptr = new((rowbytes.wrapping_mul(height) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<png_byte>() as usize)
        as uint32_t) as *mut png_byte;
    read_image_data(png_ptr, stream_data_ptr, height, rowbytes);
    intent = get_rendering_intent(png_ptr, png_info_ptr);
    if !intent.is_null() {
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"Intent\0" as *const u8 as *const ::core::ffi::c_char),
            intent,
        );
    }
    match color_type as ::core::ffi::c_int {
        PNG_COLOR_TYPE_PALETTE => {
            colorspace = create_cspace_Indexed(png_ptr, png_info_ptr);
            match trans_type {
                PDF_TRANS_TYPE_BINARY => {
                    mask = create_ckey_mask(png_ptr, png_info_ptr);
                }
                PDF_TRANS_TYPE_ALPHA => {
                    mask = create_soft_mask(png_ptr, png_info_ptr, stream_data_ptr, width, height);
                }
                _ => {}
            }
            info.num_components = 1 as ::core::ffi::c_int;
        }
        PNG_COLOR_TYPE_RGB | PNG_COLOR_TYPE_RGB_ALPHA => {
            if png_get_valid(
                png_ptr as png_const_structrp,
                png_info_ptr as png_const_inforp,
                PNG_INFO_iCCP,
            ) != 0
            {
                colorspace = create_cspace_ICCBased(png_ptr, png_info_ptr);
            } else if !intent.is_null() {
                colorspace = create_cspace_sRGB(png_ptr, png_info_ptr);
            } else {
                colorspace = create_cspace_CalRGB(png_ptr, png_info_ptr);
            }
            if colorspace.is_null() {
                colorspace =
                    pdf_new_name(b"DeviceRGB\0" as *const u8 as *const ::core::ffi::c_char);
            }
            match trans_type {
                PDF_TRANS_TYPE_BINARY => {
                    mask = create_ckey_mask(png_ptr, png_info_ptr);
                }
                PDF_TRANS_TYPE_ALPHA => {
                    mask = strip_soft_mask(
                        png_ptr,
                        png_info_ptr,
                        stream_data_ptr,
                        &raw mut rowbytes,
                        width,
                        height,
                    );
                }
                _ => {
                    mask = ::core::ptr::null_mut::<pdf_obj>();
                }
            }
            info.num_components = 3 as ::core::ffi::c_int;
        }
        PNG_COLOR_TYPE_GRAY | PNG_COLOR_TYPE_GRAY_ALPHA => {
            if png_get_valid(
                png_ptr as png_const_structrp,
                png_info_ptr as png_const_inforp,
                PNG_INFO_iCCP,
            ) != 0
            {
                colorspace = create_cspace_ICCBased(png_ptr, png_info_ptr);
            } else if !intent.is_null() {
                colorspace = create_cspace_sRGB(png_ptr, png_info_ptr);
            } else {
                colorspace = create_cspace_CalGray(png_ptr, png_info_ptr);
            }
            if colorspace.is_null() {
                colorspace =
                    pdf_new_name(b"DeviceGray\0" as *const u8 as *const ::core::ffi::c_char);
            }
            match trans_type {
                PDF_TRANS_TYPE_BINARY => {
                    mask = create_ckey_mask(png_ptr, png_info_ptr);
                }
                PDF_TRANS_TYPE_ALPHA => {
                    mask = strip_soft_mask(
                        png_ptr,
                        png_info_ptr,
                        stream_data_ptr,
                        &raw mut rowbytes,
                        width,
                        height,
                    );
                }
                _ => {
                    mask = ::core::ptr::null_mut::<pdf_obj>();
                }
            }
            info.num_components = 1 as ::core::ffi::c_int;
        }
        _ => {
            pdf_warning!(
                b"%s: Unknown PNG colortype %d.\0" as *const u8 as *const ::core::ffi::c_char,
                PNG_DEBUG_STR.as_ptr(),
                color_type as ::core::ffi::c_int,
            );
        }
    }
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"ColorSpace\0" as *const u8 as *const ::core::ffi::c_char),
        colorspace,
    );
    pdf_add_stream(
        stream,
        stream_data_ptr as *const ::core::ffi::c_void,
        rowbytes.wrapping_mul(height) as ::core::ffi::c_int,
    );
    free(stream_data_ptr as *mut ::core::ffi::c_void);
    if !mask.is_null() {
        if trans_type == PDF_TRANS_TYPE_BINARY {
            pdf_add_dict(
                stream_dict,
                pdf_new_name(b"Mask\0" as *const u8 as *const ::core::ffi::c_char),
                mask,
            );
        } else if trans_type == PDF_TRANS_TYPE_ALPHA {
            if info.bits_per_component >= 8 as ::core::ffi::c_int
                && info.width > 64 as ::core::ffi::c_int
            {
                pdf_stream_set_predictor(
                    mask,
                    2 as ::core::ffi::c_int,
                    info.width as int32_t,
                    info.bits_per_component,
                    1 as ::core::ffi::c_int,
                );
            }
            pdf_add_dict(
                stream_dict,
                pdf_new_name(b"SMask\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_ref_obj(mask),
            );
            pdf_release_obj(mask);
        } else {
            pdf_warning!(
                b"%s: Unknown transparency type...???\0" as *const u8 as *const ::core::ffi::c_char,
                PNG_DEBUG_STR.as_ptr(),
            );
            pdf_release_obj(mask);
        }
    }
    if pdf_check_version(1 as ::core::ffi::c_int, 4 as ::core::ffi::c_int)
        >= 0 as ::core::ffi::c_int
    {
        let mut text_ptr: png_textp = ::core::ptr::null_mut::<png_text>();
        let mut XMP_stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut XMP_stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut i: ::core::ffi::c_int = 0;
        let mut num_text: ::core::ffi::c_int = 0;
        let mut have_XMP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        num_text = png_get_text(
            png_ptr as png_const_structrp,
            png_info_ptr as png_inforp,
            &raw mut text_ptr,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        i = 0 as ::core::ffi::c_int;
        while i < num_text {
            if memcmp(
                (*text_ptr.offset(i as isize)).key as *const ::core::ffi::c_void,
                b"XML:com.adobe.xmp\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                17 as size_t,
            ) == 0
            {
                if (*text_ptr.offset(i as isize)).compression != PNG_ITXT_COMPRESSION_NONE
                    || (*text_ptr.offset(i as isize)).itxt_length == 0 as size_t
                {
                    pdf_warning!(
                        b"%s: Invalid value(s) in iTXt chunk for XMP Metadata.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        PNG_DEBUG_STR.as_ptr(),
                    );
                } else if have_XMP != 0 {
                    pdf_warning!(
                        b"%s: Multiple XMP Metadata. Don't know how to treat it.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        PNG_DEBUG_STR.as_ptr(),
                    );
                } else {
                    XMP_stream = pdf_new_stream(STREAM_COMPRESS);
                    XMP_stream_dict = pdf_stream_dict(XMP_stream);
                    pdf_add_dict(
                        XMP_stream_dict,
                        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
                        pdf_new_name(b"Metadata\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                    pdf_add_dict(
                        XMP_stream_dict,
                        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
                        pdf_new_name(b"XML\0" as *const u8 as *const ::core::ffi::c_char),
                    );
                    pdf_add_stream(
                        XMP_stream,
                        (*text_ptr.offset(i as isize)).text as *const ::core::ffi::c_void,
                        (*text_ptr.offset(i as isize)).itxt_length as ::core::ffi::c_int,
                    );
                    pdf_add_dict(
                        stream_dict,
                        pdf_new_name(b"Metadata\0" as *const u8 as *const ::core::ffi::c_char),
                        pdf_ref_obj(XMP_stream),
                    );
                    pdf_release_obj(XMP_stream);
                    have_XMP = 1 as ::core::ffi::c_int;
                }
            }
            i += 1;
        }
    }
    png_read_end(png_ptr as png_structrp, ::core::ptr::null_mut::<png_info>());
    if !png_info_ptr.is_null() {
        png_destroy_info_struct(png_ptr as png_const_structrp, &raw mut png_info_ptr);
    }
    if !png_ptr.is_null() {
        png_destroy_read_struct(
            &raw mut png_ptr,
            ::core::ptr::null_mut::<*mut png_info>(),
            ::core::ptr::null_mut::<*mut png_info>(),
        );
    }
    if color_type as ::core::ffi::c_int != PNG_COLOR_TYPE_PALETTE
        && info.bits_per_component >= 8 as ::core::ffi::c_int
        && info.height > 64 as ::core::ffi::c_int
    {
        pdf_stream_set_predictor(
            stream,
            15 as ::core::ffi::c_int,
            info.width as int32_t,
            info.bits_per_component,
            info.num_components,
        );
    }
    pdf_ximage_set_image(ximage, &raw mut info as *mut ::core::ffi::c_void, stream);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn check_transparency(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> ::core::ffi::c_int {
    let mut trans_type: ::core::ffi::c_int = 0;
    let mut color_type: png_byte = 0;
    let mut trans_values: png_color_16p = ::core::ptr::null_mut::<png_color_16>();
    let mut trans: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut num_trans: ::core::ffi::c_int = 0;
    color_type = png_get_color_type(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    if color_type as ::core::ffi::c_int == PNG_COLOR_TYPE_RGB_ALPHA
        || color_type as ::core::ffi::c_int == PNG_COLOR_TYPE_GRAY_ALPHA
    {
        trans_type = PDF_TRANS_TYPE_ALPHA;
    } else if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_tRNS,
    ) != 0
        && png_get_tRNS(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            &raw mut trans,
            &raw mut num_trans,
            &raw mut trans_values,
        ) != 0
    {
        match color_type as ::core::ffi::c_int {
            PNG_COLOR_TYPE_PALETTE => {
                trans_type = PDF_TRANS_TYPE_BINARY;
                loop {
                    let fresh1 = num_trans;
                    num_trans = num_trans - 1;
                    if !(fresh1 > 0 as ::core::ffi::c_int) {
                        break;
                    }
                    if !(*trans.offset(num_trans as isize) as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                        && *trans.offset(num_trans as isize) as ::core::ffi::c_int
                            != 0xff as ::core::ffi::c_int)
                    {
                        continue;
                    }
                    trans_type = PDF_TRANS_TYPE_ALPHA;
                    break;
                }
            }
            PNG_COLOR_TYPE_GRAY | PNG_COLOR_TYPE_RGB => {
                trans_type = PDF_TRANS_TYPE_BINARY;
            }
            _ => {
                trans_type = PDF_TRANS_TYPE_NONE;
            }
        }
    } else {
        trans_type = PDF_TRANS_TYPE_NONE;
    }
    if pdf_check_version(1 as ::core::ffi::c_int, 3 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        && trans_type != PDF_TRANS_TYPE_NONE
        || pdf_check_version(1 as ::core::ffi::c_int, 4 as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
            && trans_type == PDF_TRANS_TYPE_ALPHA
    {
        let mut bg: png_color_16 = png_color_16_struct {
            index: 0,
            red: 0,
            green: 0,
            blue: 0,
            gray: 0,
        };
        bg.red = 255 as png_uint_16;
        bg.green = 255 as png_uint_16;
        bg.blue = 255 as png_uint_16;
        bg.gray = 255 as png_uint_16;
        bg.index = 0 as png_byte;
        png_set_background(
            png_ptr as png_structrp,
            &raw mut bg as png_const_color_16p,
            PNG_BACKGROUND_GAMMA_SCREEN,
            0 as ::core::ffi::c_int,
            1.0f64,
        );
        pdf_warning!(
            b"%s: Transparency will be ignored. (no support in PDF ver. < 1.3)\0" as *const u8
                as *const ::core::ffi::c_char,
            PNG_DEBUG_STR.as_ptr(),
        );
        if pdf_check_version(1 as ::core::ffi::c_int, 3 as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        {
            pdf_warning!(
                b"%s: Please use -V 3 option to enable binary transparency support.\0" as *const u8
                    as *const ::core::ffi::c_char,
                PNG_DEBUG_STR.as_ptr(),
            );
        }
        if pdf_check_version(1 as ::core::ffi::c_int, 4 as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        {
            pdf_warning!(
                b"%s: Please use -V 4 option to enable full alpha channel support.\0" as *const u8
                    as *const ::core::ffi::c_char,
                PNG_DEBUG_STR.as_ptr(),
            );
        }
        trans_type = PDF_TRANS_TYPE_NONE;
    }
    return trans_type;
}
unsafe extern "C-unwind" fn get_rendering_intent(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut intent: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut srgb_intent: ::core::ffi::c_int = 0;
    if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_sRGB,
    ) != 0
        && png_get_sRGB(
            png_ptr as png_const_structrp,
            info_ptr as png_const_inforp,
            &raw mut srgb_intent,
        ) != 0
    {
        match srgb_intent {
            PNG_sRGB_INTENT_SATURATION => {
                intent = pdf_new_name(b"Saturation\0" as *const u8 as *const ::core::ffi::c_char);
            }
            PNG_sRGB_INTENT_PERCEPTUAL => {
                intent = pdf_new_name(b"Perceptual\0" as *const u8 as *const ::core::ffi::c_char);
            }
            PNG_sRGB_INTENT_ABSOLUTE => {
                intent = pdf_new_name(
                    b"AbsoluteColorimetric\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            PNG_sRGB_INTENT_RELATIVE => {
                intent = pdf_new_name(
                    b"RelativeColorimetric\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            _ => {
                pdf_warning!(
                    b"%s: Invalid value in PNG sRGB chunk: %d\0" as *const u8
                        as *const ::core::ffi::c_char,
                    PNG_DEBUG_STR.as_ptr(),
                    srgb_intent,
                );
                intent = ::core::ptr::null_mut::<pdf_obj>();
            }
        }
    } else {
        intent = ::core::ptr::null_mut::<pdf_obj>();
    }
    return intent;
}
unsafe extern "C-unwind" fn create_cspace_sRGB(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cal_param: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut color_type: png_byte = 0;
    color_type = png_get_color_type(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    cal_param = make_param_Cal(
        color_type, 2.2f64, 0.3127f64, 0.329f64, 0.64f64, 0.33f64, 0.3f64, 0.6f64, 0.15f64, 0.06f64,
    );
    if cal_param.is_null() {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    colorspace = pdf_new_array();
    match color_type as ::core::ffi::c_int {
        PNG_COLOR_TYPE_RGB | PNG_COLOR_TYPE_RGB_ALPHA | PNG_COLOR_TYPE_PALETTE => {
            pdf_add_array(
                colorspace,
                pdf_new_name(b"CalRGB\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        PNG_COLOR_TYPE_GRAY | PNG_COLOR_TYPE_GRAY_ALPHA => {
            pdf_add_array(
                colorspace,
                pdf_new_name(b"CalGray\0" as *const u8 as *const ::core::ffi::c_char),
            );
        }
        _ => {}
    }
    pdf_add_array(colorspace, cal_param);
    return colorspace;
}
unsafe extern "C-unwind" fn create_cspace_ICCBased(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut csp_id: ::core::ffi::c_int = 0;
    let mut colortype: ::core::ffi::c_int = 0;
    let mut color_type: png_byte = 0;
    let mut name: png_charp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut compression_type: ::core::ffi::c_int = 0;
    let mut profile: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut proflen: png_uint_32 = 0;
    if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_iCCP,
    ) == 0
        || png_get_iCCP(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            &raw mut name,
            &raw mut compression_type,
            &raw mut profile,
            &raw mut proflen,
        ) == 0
    {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    color_type = png_get_color_type(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    if color_type as ::core::ffi::c_int & PNG_COLOR_MASK_COLOR != 0 {
        colortype = PDF_COLORSPACE_TYPE_RGB;
    } else {
        colortype = PDF_COLORSPACE_TYPE_GRAY;
    }
    if iccp_check_colorspace(
        colortype,
        profile as *const ::core::ffi::c_void,
        proflen as ::core::ffi::c_int,
    ) < 0 as ::core::ffi::c_int
    {
        colorspace = ::core::ptr::null_mut::<pdf_obj>();
    } else {
        csp_id = iccp_load_profile(
            name as *const ::core::ffi::c_char,
            profile as *const ::core::ffi::c_void,
            proflen as ::core::ffi::c_int,
        );
        if csp_id < 0 as ::core::ffi::c_int {
            colorspace = ::core::ptr::null_mut::<pdf_obj>();
        } else {
            colorspace = pdf_get_colorspace_reference(csp_id);
        }
    }
    return colorspace;
}
unsafe extern "C-unwind" fn create_cspace_CalRGB(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cal_param: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut xw: ::core::ffi::c_double = 0.;
    let mut yw: ::core::ffi::c_double = 0.;
    let mut xr: ::core::ffi::c_double = 0.;
    let mut yr: ::core::ffi::c_double = 0.;
    let mut xg: ::core::ffi::c_double = 0.;
    let mut yg: ::core::ffi::c_double = 0.;
    let mut xb: ::core::ffi::c_double = 0.;
    let mut yb: ::core::ffi::c_double = 0.;
    let mut G: ::core::ffi::c_double = 0.;
    if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_cHRM,
    ) == 0
        || png_get_cHRM(
            png_ptr as png_const_structrp,
            info_ptr as png_const_inforp,
            &raw mut xw,
            &raw mut yw,
            &raw mut xr,
            &raw mut yr,
            &raw mut xg,
            &raw mut yg,
            &raw mut xb,
            &raw mut yb,
        ) == 0
    {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if xw <= 0.0f64
        || yw < 1.0e-10f64
        || xr < 0.0f64
        || yr < 0.0f64
        || xg < 0.0f64
        || yg < 0.0f64
        || xb < 0.0f64
        || yb < 0.0f64
    {
        pdf_warning!(
            b"%s: Invalid cHRM chunk parameters found.\0" as *const u8
                as *const ::core::ffi::c_char,
            PNG_DEBUG_STR.as_ptr(),
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_gAMA,
    ) != 0
        && png_get_gAMA(
            png_ptr as png_const_structrp,
            info_ptr as png_const_inforp,
            &raw mut G,
        ) != 0
    {
        if G < 1.0e-2f64 {
            pdf_warning!(
                b"%s: Unusual Gamma value: 1.0 / %g\0" as *const u8 as *const ::core::ffi::c_char,
                PNG_DEBUG_STR.as_ptr(),
                G,
            );
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        G = 1.0f64 / G;
    } else {
        G = DPX_PNG_DEFAULT_GAMMA;
    }
    cal_param = make_param_Cal(
        PNG_COLOR_TYPE_RGB as png_byte,
        G,
        xw,
        yw,
        xr,
        yr,
        xg,
        yg,
        xb,
        yb,
    );
    if cal_param.is_null() {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    colorspace = pdf_new_array();
    pdf_add_array(
        colorspace,
        pdf_new_name(b"CalRGB\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_array(colorspace, cal_param);
    return colorspace;
}
unsafe extern "C-unwind" fn create_cspace_CalGray(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cal_param: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut xw: ::core::ffi::c_double = 0.;
    let mut yw: ::core::ffi::c_double = 0.;
    let mut xr: ::core::ffi::c_double = 0.;
    let mut yr: ::core::ffi::c_double = 0.;
    let mut xg: ::core::ffi::c_double = 0.;
    let mut yg: ::core::ffi::c_double = 0.;
    let mut xb: ::core::ffi::c_double = 0.;
    let mut yb: ::core::ffi::c_double = 0.;
    let mut G: ::core::ffi::c_double = 0.;
    if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_cHRM,
    ) == 0
        || png_get_cHRM(
            png_ptr as png_const_structrp,
            info_ptr as png_const_inforp,
            &raw mut xw,
            &raw mut yw,
            &raw mut xr,
            &raw mut yr,
            &raw mut xg,
            &raw mut yg,
            &raw mut xb,
            &raw mut yb,
        ) == 0
    {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if xw <= 0.0f64
        || yw < 1.0e-10f64
        || xr < 0.0f64
        || yr < 0.0f64
        || xg < 0.0f64
        || yg < 0.0f64
        || xb < 0.0f64
        || yb < 0.0f64
    {
        pdf_warning!(
            b"%s: Invalid cHRM chunk parameters found.\0" as *const u8
                as *const ::core::ffi::c_char,
            PNG_DEBUG_STR.as_ptr(),
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_gAMA,
    ) != 0
        && png_get_gAMA(
            png_ptr as png_const_structrp,
            info_ptr as png_const_inforp,
            &raw mut G,
        ) != 0
    {
        if G < 1.0e-2f64 {
            pdf_warning!(
                b"%s: Unusual Gamma value: 1.0 / %g\0" as *const u8 as *const ::core::ffi::c_char,
                PNG_DEBUG_STR.as_ptr(),
                G,
            );
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        G = 1.0f64 / G;
    } else {
        G = DPX_PNG_DEFAULT_GAMMA;
    }
    cal_param = make_param_Cal(
        PNG_COLOR_TYPE_GRAY as png_byte,
        G,
        xw,
        yw,
        xr,
        yr,
        xg,
        yg,
        xb,
        yb,
    );
    if cal_param.is_null() {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    colorspace = pdf_new_array();
    pdf_add_array(
        colorspace,
        pdf_new_name(b"CalGray\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_array(colorspace, cal_param);
    return colorspace;
}
unsafe extern "C-unwind" fn make_param_Cal(
    mut color_type: png_byte,
    mut G: ::core::ffi::c_double,
    mut xw: ::core::ffi::c_double,
    mut yw: ::core::ffi::c_double,
    mut xr: ::core::ffi::c_double,
    mut yr: ::core::ffi::c_double,
    mut xg: ::core::ffi::c_double,
    mut yg: ::core::ffi::c_double,
    mut xb: ::core::ffi::c_double,
    mut yb: ::core::ffi::c_double,
) -> *mut pdf_obj {
    let mut cal_param: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut white_point: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut matrix: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut dev_gamma: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut Xw: ::core::ffi::c_double = 0.;
    let mut Yw: ::core::ffi::c_double = 0.;
    let mut Zw: ::core::ffi::c_double = 0.;
    let mut Xr: ::core::ffi::c_double = 0.;
    let mut Xg: ::core::ffi::c_double = 0.;
    let mut Xb: ::core::ffi::c_double = 0.;
    let mut Yr: ::core::ffi::c_double = 0.;
    let mut Yb: ::core::ffi::c_double = 0.;
    let mut Yg: ::core::ffi::c_double = 0.;
    let mut Zr: ::core::ffi::c_double = 0.;
    let mut Zg: ::core::ffi::c_double = 0.;
    let mut Zb: ::core::ffi::c_double = 0.;
    let mut zw: ::core::ffi::c_double = 0.;
    let mut zr: ::core::ffi::c_double = 0.;
    let mut zg: ::core::ffi::c_double = 0.;
    let mut zb: ::core::ffi::c_double = 0.;
    let mut fr: ::core::ffi::c_double = 0.;
    let mut fg: ::core::ffi::c_double = 0.;
    let mut fb: ::core::ffi::c_double = 0.;
    let mut det: ::core::ffi::c_double = 0.;
    zw = 1 as ::core::ffi::c_int as ::core::ffi::c_double - (xw + yw);
    zr = 1 as ::core::ffi::c_int as ::core::ffi::c_double - (xr + yr);
    zg = 1 as ::core::ffi::c_int as ::core::ffi::c_double - (xg + yg);
    zb = 1 as ::core::ffi::c_int as ::core::ffi::c_double - (xb + yb);
    Xw = xw / yw;
    Yw = 1.0f64;
    Zw = zw / yw;
    det = xr * (yg * zb - zg * yb) - xg * (yr * zb - zr * yb) + xb * (yr * zg - zr * yg);
    if (if det < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        -det
    } else {
        det
    }) < 1.0e-10f64
    {
        pdf_warning!(
            b"Non invertible matrix: Maybe invalid value(s) specified in cHRM chunk.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    fr = (Xw * (yg * zb - zg * yb) - xg * (zb - Zw * yb) + xb * (zg - Zw * yg)) / det;
    fg = (xr * (zb - Zw * yb) - Xw * (yr * zb - zr * yb) + xb * (yr * Zw - zr)) / det;
    fb = (xr * (yg * Zw - zg) - xg * (yr * Zw - zr) + Xw * (yr * zg - zr * yg)) / det;
    Xr = fr * xr;
    Yr = fr * yr;
    Zr = fr * zr;
    Xg = fg * xg;
    Yg = fg * yg;
    Zg = fg * zg;
    Xb = fb * xb;
    Yb = fb * yb;
    Zb = fb * zb;
    if G < 1.0e-2f64 {
        pdf_warning!(
            b"Unusual Gamma specified: 1.0 / %g\0" as *const u8 as *const ::core::ffi::c_char,
            G,
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    cal_param = pdf_new_dict();
    white_point = pdf_new_array();
    pdf_add_array(
        white_point,
        pdf_new_number(floor(Xw / 0.00001f64 + 0.5f64) * 0.00001f64),
    );
    pdf_add_array(
        white_point,
        pdf_new_number(floor(Yw / 0.00001f64 + 0.5f64) * 0.00001f64),
    );
    pdf_add_array(
        white_point,
        pdf_new_number(floor(Zw / 0.00001f64 + 0.5f64) * 0.00001f64),
    );
    pdf_add_dict(
        cal_param,
        pdf_new_name(b"WhitePoint\0" as *const u8 as *const ::core::ffi::c_char),
        white_point,
    );
    if color_type as ::core::ffi::c_int & PNG_COLOR_MASK_COLOR != 0 {
        if G != 1.0f64 {
            dev_gamma = pdf_new_array();
            pdf_add_array(
                dev_gamma,
                pdf_new_number(floor(G / 0.00001f64 + 0.5f64) * 0.00001f64),
            );
            pdf_add_array(
                dev_gamma,
                pdf_new_number(floor(G / 0.00001f64 + 0.5f64) * 0.00001f64),
            );
            pdf_add_array(
                dev_gamma,
                pdf_new_number(floor(G / 0.00001f64 + 0.5f64) * 0.00001f64),
            );
            pdf_add_dict(
                cal_param,
                pdf_new_name(b"Gamma\0" as *const u8 as *const ::core::ffi::c_char),
                dev_gamma,
            );
        }
        matrix = pdf_new_array();
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Xr / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Yr / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Zr / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Xg / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Yg / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Zg / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Xb / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Yb / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_array(
            matrix,
            pdf_new_number(floor(Zb / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
        pdf_add_dict(
            cal_param,
            pdf_new_name(b"Matrix\0" as *const u8 as *const ::core::ffi::c_char),
            matrix,
        );
    } else if G != 1.0f64 {
        pdf_add_dict(
            cal_param,
            pdf_new_name(b"Gamma\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(floor(G / 0.00001f64 + 0.5f64) * 0.00001f64),
        );
    }
    return cal_param;
}
unsafe extern "C-unwind" fn create_cspace_Indexed(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorspace: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut base: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut lookup: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut data_ptr: *mut png_byte = ::core::ptr::null_mut::<png_byte>();
    let mut plte: png_colorp = ::core::ptr::null_mut::<png_color>();
    let mut num_plte: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_PLTE,
    ) == 0
        || png_get_PLTE(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            &raw mut plte,
            &raw mut num_plte,
        ) == 0
    {
        pdf_warning!(
            b"%s: PNG does not have valid PLTE chunk.\0" as *const u8 as *const ::core::ffi::c_char,
            PNG_DEBUG_STR.as_ptr(),
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    colorspace = pdf_new_array();
    pdf_add_array(
        colorspace,
        pdf_new_name(b"Indexed\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_iCCP,
    ) != 0
    {
        base = create_cspace_ICCBased(png_ptr, info_ptr);
    } else if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_sRGB,
    ) != 0
    {
        base = create_cspace_sRGB(png_ptr, info_ptr);
    } else {
        base = create_cspace_CalRGB(png_ptr, info_ptr);
    }
    if base.is_null() {
        base = pdf_new_name(b"DeviceRGB\0" as *const u8 as *const ::core::ffi::c_char);
    }
    pdf_add_array(colorspace, base);
    pdf_add_array(
        colorspace,
        pdf_new_number((num_plte - 1 as ::core::ffi::c_int) as ::core::ffi::c_double),
    );
    data_ptr = new(((num_plte * 3 as ::core::ffi::c_int) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<png_byte>() as usize) as uint32_t)
        as *mut png_byte;
    i = 0 as ::core::ffi::c_int;
    while i < num_plte {
        *data_ptr.offset((3 as ::core::ffi::c_int * i) as isize) = (*plte.offset(i as isize)).red;
        *data_ptr.offset((3 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize) =
            (*plte.offset(i as isize)).green;
        *data_ptr.offset((3 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as isize) =
            (*plte.offset(i as isize)).blue;
        i += 1;
    }
    lookup = pdf_new_string(
        data_ptr as *const ::core::ffi::c_void,
        (num_plte * 3 as ::core::ffi::c_int) as size_t,
    );
    free(data_ptr as *mut ::core::ffi::c_void);
    pdf_add_array(colorspace, lookup);
    return colorspace;
}
unsafe extern "C-unwind" fn create_ckey_mask(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
) -> *mut pdf_obj {
    let mut colorkeys: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut color_type: png_byte = 0;
    let mut trans: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut num_trans: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut colors: png_color_16p = ::core::ptr::null_mut::<png_color_16>();
    if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_tRNS,
    ) == 0
        || png_get_tRNS(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            &raw mut trans,
            &raw mut num_trans,
            &raw mut colors,
        ) == 0
    {
        pdf_warning!(
            b"%s: PNG does not have valid tRNS chunk!\0" as *const u8 as *const ::core::ffi::c_char,
            PNG_DEBUG_STR.as_ptr(),
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    colorkeys = pdf_new_array();
    color_type = png_get_color_type(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    match color_type as ::core::ffi::c_int {
        PNG_COLOR_TYPE_PALETTE => {
            i = 0 as ::core::ffi::c_int;
            while i < num_trans {
                if *trans.offset(i as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    pdf_add_array(colorkeys, pdf_new_number(i as ::core::ffi::c_double));
                    pdf_add_array(colorkeys, pdf_new_number(i as ::core::ffi::c_double));
                } else if *trans.offset(i as isize) as ::core::ffi::c_int
                    != 0xff as ::core::ffi::c_int
                {
                    pdf_warning!(
                        b"%s: You found a bug in pngimage.c.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        PNG_DEBUG_STR.as_ptr(),
                    );
                }
                i += 1;
            }
        }
        PNG_COLOR_TYPE_RGB => {
            pdf_add_array(
                colorkeys,
                pdf_new_number((*colors).red as ::core::ffi::c_double),
            );
            pdf_add_array(
                colorkeys,
                pdf_new_number((*colors).red as ::core::ffi::c_double),
            );
            pdf_add_array(
                colorkeys,
                pdf_new_number((*colors).green as ::core::ffi::c_double),
            );
            pdf_add_array(
                colorkeys,
                pdf_new_number((*colors).green as ::core::ffi::c_double),
            );
            pdf_add_array(
                colorkeys,
                pdf_new_number((*colors).blue as ::core::ffi::c_double),
            );
            pdf_add_array(
                colorkeys,
                pdf_new_number((*colors).blue as ::core::ffi::c_double),
            );
        }
        PNG_COLOR_TYPE_GRAY => {
            pdf_add_array(
                colorkeys,
                pdf_new_number((*colors).gray as ::core::ffi::c_double),
            );
            pdf_add_array(
                colorkeys,
                pdf_new_number((*colors).gray as ::core::ffi::c_double),
            );
        }
        _ => {
            pdf_warning!(
                b"%s: You found a bug in pngimage.c.\0" as *const u8 as *const ::core::ffi::c_char,
                PNG_DEBUG_STR.as_ptr(),
            );
            pdf_release_obj(colorkeys);
            colorkeys = ::core::ptr::null_mut::<pdf_obj>();
        }
    }
    return colorkeys;
}
unsafe extern "C-unwind" fn create_soft_mask(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut image_data_ptr: png_bytep,
    mut width: png_uint_32,
    mut height: png_uint_32,
) -> *mut pdf_obj {
    let mut smask: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut smask_data_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut trans: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut num_trans: ::core::ffi::c_int = 0;
    let mut i: png_uint_32 = 0;
    let mut bpc: png_byte = 0;
    let mut mask: png_byte = 0;
    let mut shift: png_byte = 0;
    if png_get_valid(
        png_ptr as png_const_structrp,
        info_ptr as png_const_inforp,
        PNG_INFO_tRNS,
    ) == 0
        || png_get_tRNS(
            png_ptr as png_const_structrp,
            info_ptr as png_inforp,
            &raw mut trans,
            &raw mut num_trans,
            ::core::ptr::null_mut::<png_color_16p>(),
        ) == 0
    {
        pdf_warning!(
            b"%s: PNG does not have valid tRNS chunk but tRNS is requested.\0" as *const u8
                as *const ::core::ffi::c_char,
            PNG_DEBUG_STR.as_ptr(),
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    bpc = png_get_bit_depth(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    mask = (0xff as ::core::ffi::c_int >> 8 as ::core::ffi::c_int - bpc as ::core::ffi::c_int)
        as png_byte;
    shift = (8 as ::core::ffi::c_int - bpc as ::core::ffi::c_int) as png_byte;
    smask = pdf_new_stream(STREAM_COMPRESS);
    dict = pdf_stream_dict(smask);
    smask_data_ptr = new((width.wrapping_mul(height) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<png_byte>() as usize)
        as uint32_t) as *mut png_byte;
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
        pdf_new_number(width as ::core::ffi::c_double),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Height\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(height as ::core::ffi::c_double),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"ColorSpace\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"DeviceGray\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"BitsPerComponent\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(8 as ::core::ffi::c_int as ::core::ffi::c_double),
    );
    i = 0 as png_uint_32;
    while i < width.wrapping_mul(height) {
        let mut idx: png_byte = (*image_data_ptr.offset(
            (bpc as png_uint_32)
                .wrapping_mul(i)
                .wrapping_div(8 as png_uint_32) as isize,
        ) as ::core::ffi::c_int
            >> (shift as png_uint_32).wrapping_sub(
                (bpc as png_uint_32)
                    .wrapping_mul(i)
                    .wrapping_rem(8 as png_uint_32),
            )
            & mask as ::core::ffi::c_int) as png_byte;
        *smask_data_ptr.offset(i as isize) = (if (idx as ::core::ffi::c_int) < num_trans {
            *trans.offset(idx as isize) as ::core::ffi::c_int
        } else {
            0xff as ::core::ffi::c_int
        }) as png_byte;
        i = i.wrapping_add(1);
    }
    pdf_add_stream(
        smask,
        smask_data_ptr as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        width.wrapping_mul(height) as ::core::ffi::c_int,
    );
    free(smask_data_ptr as *mut ::core::ffi::c_void);
    return smask;
}
unsafe extern "C-unwind" fn strip_soft_mask(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut image_data_ptr: png_bytep,
    mut rowbytes_ptr: png_uint_32p,
    mut width: png_uint_32,
    mut height: png_uint_32,
) -> *mut pdf_obj {
    let mut smask: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut color_type: png_byte = 0;
    let mut bpc: png_byte = 0;
    let mut smask_data_ptr: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut i: png_uint_32 = 0;
    color_type = png_get_color_type(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    bpc = png_get_bit_depth(png_ptr as png_const_structrp, info_ptr as png_const_inforp);
    if color_type as ::core::ffi::c_int & PNG_COLOR_MASK_COLOR != 0 {
        let mut bps: ::core::ffi::c_int = if bpc as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
            4 as ::core::ffi::c_int
        } else {
            8 as ::core::ffi::c_int
        };
        if *rowbytes_ptr as usize
            != ((bps as png_uint_32).wrapping_mul(width) as usize)
                .wrapping_mul(::core::mem::size_of::<png_byte>() as usize)
        {
            pdf_warning!(
                b"%s: Inconsistent rowbytes value.\0" as *const u8 as *const ::core::ffi::c_char,
                PNG_DEBUG_STR.as_ptr(),
            );
            return ::core::ptr::null_mut::<pdf_obj>();
        }
    } else {
        let mut bps_0: ::core::ffi::c_int = if bpc as ::core::ffi::c_int == 8 as ::core::ffi::c_int
        {
            2 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        };
        if *rowbytes_ptr as usize
            != ((bps_0 as png_uint_32).wrapping_mul(width) as usize)
                .wrapping_mul(::core::mem::size_of::<png_byte>() as usize)
        {
            pdf_warning!(
                b"%s: Inconsistent rowbytes value.\0" as *const u8 as *const ::core::ffi::c_char,
                PNG_DEBUG_STR.as_ptr(),
            );
            return ::core::ptr::null_mut::<pdf_obj>();
        }
    }
    smask = pdf_new_stream(STREAM_COMPRESS);
    dict = pdf_stream_dict(smask);
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
        pdf_new_number(width as ::core::ffi::c_double),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"Height\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(height as ::core::ffi::c_double),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"ColorSpace\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"DeviceGray\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"BitsPerComponent\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(bpc as ::core::ffi::c_double),
    );
    smask_data_ptr = new(
        (((bpc as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as png_uint_32)
            .wrapping_mul(width)
            .wrapping_mul(height) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<png_byte>() as usize) as uint32_t,
    ) as *mut png_byte;
    match color_type as ::core::ffi::c_int {
        PNG_COLOR_TYPE_RGB_ALPHA => {
            if bpc as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                i = 0 as png_uint_32;
                while i < width.wrapping_mul(height) {
                    memmove(
                        image_data_ptr.offset((3 as png_uint_32).wrapping_mul(i) as isize)
                            as *mut ::core::ffi::c_void,
                        image_data_ptr.offset((4 as png_uint_32).wrapping_mul(i) as isize)
                            as *const ::core::ffi::c_void,
                        3 as size_t,
                    );
                    *smask_data_ptr.offset(i as isize) = *image_data_ptr.offset(
                        (4 as png_uint_32)
                            .wrapping_mul(i)
                            .wrapping_add(3 as png_uint_32) as isize,
                    );
                    i = i.wrapping_add(1);
                }
                *rowbytes_ptr = ((3 as png_uint_32).wrapping_mul(width) as usize)
                    .wrapping_mul(::core::mem::size_of::<png_byte>() as usize)
                    as png_uint_32;
            } else {
                i = 0 as png_uint_32;
                while i < width.wrapping_mul(height) {
                    memmove(
                        image_data_ptr.offset((6 as png_uint_32).wrapping_mul(i) as isize)
                            as *mut ::core::ffi::c_void,
                        image_data_ptr.offset((8 as png_uint_32).wrapping_mul(i) as isize)
                            as *const ::core::ffi::c_void,
                        6 as size_t,
                    );
                    *smask_data_ptr.offset((2 as png_uint_32).wrapping_mul(i) as isize) =
                        *image_data_ptr.offset(
                            (8 as png_uint_32)
                                .wrapping_mul(i)
                                .wrapping_add(6 as png_uint_32)
                                as isize,
                        );
                    *smask_data_ptr.offset(
                        (2 as png_uint_32)
                            .wrapping_mul(i)
                            .wrapping_add(1 as png_uint_32) as isize,
                    ) = *image_data_ptr.offset(
                        (8 as png_uint_32)
                            .wrapping_mul(i)
                            .wrapping_add(7 as png_uint_32) as isize,
                    );
                    i = i.wrapping_add(1);
                }
                *rowbytes_ptr = ((6 as png_uint_32).wrapping_mul(width) as usize)
                    .wrapping_mul(::core::mem::size_of::<png_byte>() as usize)
                    as png_uint_32;
            }
        }
        PNG_COLOR_TYPE_GRAY_ALPHA => {
            if bpc as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                i = 0 as png_uint_32;
                while i < width.wrapping_mul(height) {
                    *image_data_ptr.offset(i as isize) =
                        *image_data_ptr.offset((2 as png_uint_32).wrapping_mul(i) as isize);
                    *smask_data_ptr.offset(i as isize) = *image_data_ptr.offset(
                        (2 as png_uint_32)
                            .wrapping_mul(i)
                            .wrapping_add(1 as png_uint_32) as isize,
                    );
                    i = i.wrapping_add(1);
                }
                *rowbytes_ptr = (width as usize)
                    .wrapping_mul(::core::mem::size_of::<png_byte>() as usize)
                    as png_uint_32;
            } else {
                i = 0 as png_uint_32;
                while i < width.wrapping_mul(height) {
                    *image_data_ptr.offset((2 as png_uint_32).wrapping_mul(i) as isize) =
                        *image_data_ptr.offset((4 as png_uint_32).wrapping_mul(i) as isize);
                    *image_data_ptr.offset(
                        (2 as png_uint_32)
                            .wrapping_mul(i)
                            .wrapping_add(1 as png_uint_32) as isize,
                    ) = *image_data_ptr.offset(
                        (4 as png_uint_32)
                            .wrapping_mul(i)
                            .wrapping_add(1 as png_uint_32) as isize,
                    );
                    *smask_data_ptr.offset((2 as png_uint_32).wrapping_mul(i) as isize) =
                        *image_data_ptr.offset(
                            (4 as png_uint_32)
                                .wrapping_mul(i)
                                .wrapping_add(2 as png_uint_32)
                                as isize,
                        );
                    *smask_data_ptr.offset(
                        (2 as png_uint_32)
                            .wrapping_mul(i)
                            .wrapping_add(1 as png_uint_32) as isize,
                    ) = *image_data_ptr.offset(
                        (4 as png_uint_32)
                            .wrapping_mul(i)
                            .wrapping_add(3 as png_uint_32) as isize,
                    );
                    i = i.wrapping_add(1);
                }
                *rowbytes_ptr = ((2 as png_uint_32).wrapping_mul(width) as usize)
                    .wrapping_mul(::core::mem::size_of::<png_byte>() as usize)
                    as png_uint_32;
            }
        }
        _ => {
            pdf_warning!(
                b"You found a bug in pngimage.c!\0" as *const u8 as *const ::core::ffi::c_char,
            );
            pdf_release_obj(smask);
            free(smask_data_ptr as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<pdf_obj>();
        }
    }
    pdf_add_stream(
        smask,
        smask_data_ptr as *const ::core::ffi::c_void,
        ((bpc as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as png_uint_32)
            .wrapping_mul(width)
            .wrapping_mul(height) as ::core::ffi::c_int,
    );
    free(smask_data_ptr as *mut ::core::ffi::c_void);
    return smask;
}
unsafe extern "C-unwind" fn read_image_data(
    mut png_ptr: png_structp,
    mut dest_ptr: png_bytep,
    mut height: png_uint_32,
    mut rowbytes: png_uint_32,
) {
    let mut rows_p: png_bytepp = ::core::ptr::null_mut::<*mut png_byte>();
    let mut i: png_uint_32 = 0;
    rows_p = new((height as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<png_bytep>() as usize) as uint32_t)
        as *mut png_bytep as png_bytepp;
    i = 0 as png_uint_32;
    while i < height {
        let ref mut fresh0 = *rows_p.offset(i as isize);
        *fresh0 = dest_ptr.offset(rowbytes.wrapping_mul(i) as isize) as *mut png_byte;
        i = i.wrapping_add(1);
    }
    png_read_image(png_ptr as png_structrp, rows_p);
    free(rows_p as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_png_get_bbox")]
pub unsafe extern "C-unwind" fn png_get_bbox(
    mut handle: rust_input_handle_t,
    mut width: *mut uint32_t,
    mut height: *mut uint32_t,
    mut xdensity: *mut ::core::ffi::c_double,
    mut ydensity: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut png_ptr: png_structp = ::core::ptr::null_mut::<png_struct>();
    let mut png_info_ptr: png_infop = ::core::ptr::null_mut::<png_info>();
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    png_ptr = png_create_read_struct(
        png_get_libpng_ver(std::ptr::null()),
        NULL,
        None,
        Some(
            _png_warning_callback
                as unsafe extern "C-unwind" fn(png_structp, png_const_charp) -> (),
        ),
    );
    if png_ptr.is_null() || {
        png_info_ptr = png_create_info_struct(png_ptr as png_const_structrp);
        png_info_ptr.is_null()
    } {
        pdf_warning!(
            b"%s: Creating Libpng read/info struct failed.\0" as *const u8
                as *const ::core::ffi::c_char,
            PNG_DEBUG_STR.as_ptr(),
        );
        if !png_ptr.is_null() {
            png_destroy_read_struct(
                &raw mut png_ptr,
                ::core::ptr::null_mut::<*mut png_info>(),
                ::core::ptr::null_mut::<*mut png_info>(),
            );
        }
        return -(1 as ::core::ffi::c_int);
    }
    png_set_read_fn(
        png_ptr as png_structrp,
        handle as png_voidp,
        Some(_png_read as unsafe extern "C-unwind" fn(png_structp, png_bytep, png_size_t) -> ()),
    );
    png_read_info(png_ptr as png_structrp, png_info_ptr as png_inforp);
    *width = png_get_image_width(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
    ) as uint32_t;
    *height = png_get_image_height(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
    ) as uint32_t;
    let mut xppm: png_uint_32 = png_get_x_pixels_per_meter(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
    );
    let mut yppm: png_uint_32 = png_get_y_pixels_per_meter(
        png_ptr as png_const_structrp,
        png_info_ptr as png_const_inforp,
    );
    *xdensity = if xppm != 0 {
        72.0f64 / 0.0254f64 / xppm as ::core::ffi::c_double
    } else {
        1.0f64
    };
    *ydensity = if yppm != 0 {
        72.0f64 / 0.0254f64 / yppm as ::core::ffi::c_double
    } else {
        1.0f64
    };
    if !png_info_ptr.is_null() {
        png_destroy_info_struct(png_ptr as png_const_structrp, &raw mut png_info_ptr);
    }
    if !png_ptr.is_null() {
        png_destroy_read_struct(
            &raw mut png_ptr,
            ::core::ptr::null_mut::<*mut png_info>(),
            ::core::ptr::null_mut::<*mut png_info>(),
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pdf_ximage_ {
    _private: [u8; 0],
}
#[repr(C)]
pub struct png_info_def {
    _private: [u8; 0],
}
#[repr(C)]
pub struct png_struct_def {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_check_for_png(
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { check_for_png(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_png_include_image(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { png_include_image(ximage, handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_png_get_bbox(
    mut handle: rust_input_handle_t,
    mut width: *mut uint32_t,
    mut height: *mut uint32_t,
    mut xdensity: *mut ::core::ffi::c_double,
    mut ydensity: *mut ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        png_get_bbox(handle, width, height, xdensity, ydensity)
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
