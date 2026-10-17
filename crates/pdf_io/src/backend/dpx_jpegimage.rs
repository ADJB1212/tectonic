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
#![allow(
    warnings,
    clippy::all,
    unsafe_op_in_unsafe_fn,
    static_mut_refs,
    dangerous_implicit_autorefs
)]
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn xmalloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
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
    fn ttstub_input_getc(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tt_get_unsigned_byte"]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    static mut work_buffer: [::core::ffi::c_char; 0];
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
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
    #[link_name = "__pdf_impl_pdf_stream_length"]
    fn pdf_stream_length(stream: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_stream_dataptr"]
    fn pdf_stream_dataptr(stream: *mut pdf_obj) -> *const ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_check_version"]
    fn pdf_check_version(
        major: ::core::ffi::c_int,
        minor: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_iccp_get_rendering_intent"]
    fn iccp_get_rendering_intent(
        profile: *const ::core::ffi::c_void,
        proflen: ::core::ffi::c_int,
    ) -> *mut pdf_obj;
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
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint16_t = u16;
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
pub const JM_SOI: JPEG_marker = 216;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_info {
    pub height: uint16_t,
    pub width: uint16_t,
    pub bits_per_component: uint8_t,
    pub num_components: uint8_t,
    pub xdpi: ::core::ffi::c_double,
    pub ydpi: ::core::ffi::c_double,
    pub flags: ::core::ffi::c_int,
    pub num_appn: ::core::ffi::c_int,
    pub max_appn: ::core::ffi::c_int,
    pub appn: *mut JPEG_ext,
    pub skipbits: [::core::ffi::c_char; 129],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_ext {
    pub marker: JPEG_marker,
    pub app_sig: JPEG_APPn_sig,
    pub app_data: *mut ::core::ffi::c_void,
}
pub type JPEG_APPn_sig = ::core::ffi::c_uint;
pub const JS_APPn_XMP: JPEG_APPn_sig = 3;
pub const JS_APPn_ICC: JPEG_APPn_sig = 2;
pub const JS_APPn_ADOBE: JPEG_APPn_sig = 1;
pub const JS_APPn_JFIF: JPEG_APPn_sig = 0;
pub type JPEG_marker = ::core::ffi::c_uint;
pub const JM_COM: JPEG_marker = 254;
pub const JM_APP15: JPEG_marker = 239;
pub const JM_APP14: JPEG_marker = 238;
pub const JM_APP2: JPEG_marker = 226;
pub const JM_APP1: JPEG_marker = 225;
pub const JM_APP0: JPEG_marker = 224;
pub const JM_EXP: JPEG_marker = 223;
pub const JM_DHP: JPEG_marker = 222;
pub const JM_DRI: JPEG_marker = 221;
pub const JM_DNL: JPEG_marker = 220;
pub const JM_DQT: JPEG_marker = 219;
pub const JM_SOS: JPEG_marker = 218;
pub const JM_EOI: JPEG_marker = 217;
pub const JM_RST7: JPEG_marker = 215;
pub const JM_RST6: JPEG_marker = 214;
pub const JM_RST5: JPEG_marker = 213;
pub const JM_RST4: JPEG_marker = 212;
pub const JM_RST3: JPEG_marker = 211;
pub const JM_RST2: JPEG_marker = 210;
pub const JM_RST1: JPEG_marker = 209;
pub const JM_RST0: JPEG_marker = 208;
pub const JM_SOF15: JPEG_marker = 207;
pub const JM_SOF14: JPEG_marker = 206;
pub const JM_SOF13: JPEG_marker = 205;
pub const JM_DAC: JPEG_marker = 204;
pub const JM_SOF11: JPEG_marker = 203;
pub const JM_SOF10: JPEG_marker = 202;
pub const JM_SOF9: JPEG_marker = 201;
pub const JM_SOF7: JPEG_marker = 199;
pub const JM_SOF6: JPEG_marker = 198;
pub const JM_DHT: JPEG_marker = 196;
pub const JM_SOF5: JPEG_marker = 197;
pub const JM_SOF3: JPEG_marker = 195;
pub const JM_SOF2: JPEG_marker = 194;
pub const JM_SOF1: JPEG_marker = 193;
pub const JM_SOF0: JPEG_marker = 192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_APPn_XMP {
    pub packet: *mut ::core::ffi::c_uchar,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_APPn_Adobe {
    pub version: uint16_t,
    pub flag0: uint16_t,
    pub flag1: uint16_t,
    pub transform: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_APPn_ICC {
    pub seq_id: uint8_t,
    pub num_chunks: uint8_t,
    pub chunk: *mut ::core::ffi::c_uchar,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPEG_APPn_JFIF {
    pub version: uint16_t,
    pub units: uint8_t,
    pub Xdensity: uint16_t,
    pub Ydensity: uint16_t,
    pub Xthumbnail: uint8_t,
    pub Ythumbnail: uint8_t,
    pub thumbnail: *mut ::core::ffi::c_uchar,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PDF_COLORSPACE_TYPE_CMYK: ::core::ffi::c_int = -4;
pub const PDF_COLORSPACE_TYPE_RGB: ::core::ffi::c_int = -3;
pub const PDF_COLORSPACE_TYPE_GRAY: ::core::ffi::c_int = -1;
pub const JPEG_DEBUG_STR: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"JPEG\0") };
pub const MAX_COUNT: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const HAVE_APPn_JFIF: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const HAVE_APPn_ADOBE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const HAVE_APPn_ICC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const HAVE_APPn_Exif: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const HAVE_APPn_XMP: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_check_for_jpeg")]
pub unsafe extern "C-unwind" fn check_for_jpeg(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut jpeg_sig: [::core::ffi::c_uchar; 2] = [0; 2];
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    if ttstub_input_read(
        handle,
        &raw mut jpeg_sig as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        2 as size_t,
    ) != 2 as ssize_t
    {
        return 0 as ::core::ffi::c_int;
    } else if jpeg_sig[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        != 0xff as ::core::ffi::c_int
        || jpeg_sig[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            != JM_SOI as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_jpeg_include_image")]
pub unsafe extern "C-unwind" fn jpeg_include_image(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut colorspace: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut colortype: ::core::ffi::c_int = 0;
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
    let mut j_info: JPEG_info = JPEG_info {
        height: 0,
        width: 0,
        bits_per_component: 0,
        num_components: 0,
        xdpi: 0.,
        ydpi: 0.,
        flags: 0,
        num_appn: 0,
        max_appn: 0,
        appn: ::core::ptr::null_mut::<JPEG_ext>(),
        skipbits: [0; 129],
    };
    if check_for_jpeg(handle) == 0 {
        pdf_warning!(
            b"%s: Not a JPEG file?\0" as *const u8 as *const ::core::ffi::c_char,
            JPEG_DEBUG_STR.as_ptr(),
        );
        crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
        return -(1 as ::core::ffi::c_int);
    }
    pdf_ximage_init_image_info(&raw mut info);
    JPEG_info_init(&raw mut j_info);
    if JPEG_scan_file(&raw mut j_info, handle) < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"%s: Not a JPEG file?\0" as *const u8 as *const ::core::ffi::c_char,
            JPEG_DEBUG_STR.as_ptr(),
        );
        JPEG_info_clear(&raw mut j_info);
        return -(1 as ::core::ffi::c_int);
    }
    match j_info.num_components as ::core::ffi::c_int {
        1 => {
            colortype = PDF_COLORSPACE_TYPE_GRAY;
        }
        3 => {
            colortype = PDF_COLORSPACE_TYPE_RGB;
        }
        4 => {
            colortype = PDF_COLORSPACE_TYPE_CMYK;
        }
        _ => {
            pdf_warning!(
                b"%s: Unknown color space (num components: %d)\0" as *const u8
                    as *const ::core::ffi::c_char,
                JPEG_DEBUG_STR.as_ptr(),
                info.num_components,
            );
            JPEG_info_clear(&raw mut j_info);
            return -(1 as ::core::ffi::c_int);
        }
    }
    stream = pdf_new_stream(0 as ::core::ffi::c_int);
    stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Filter\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"DCTDecode\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if pdf_check_version(1 as ::core::ffi::c_int, 4 as ::core::ffi::c_int)
        >= 0 as ::core::ffi::c_int
    {
        if j_info.flags & HAVE_APPn_XMP != 0 {
            let mut XMP_stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            XMP_stream = JPEG_get_XMP(&raw mut j_info);
            pdf_add_dict(
                stream_dict,
                pdf_new_name(b"Metadata\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_ref_obj(XMP_stream),
            );
            pdf_release_obj(XMP_stream);
        }
    }
    colorspace = ::core::ptr::null_mut::<pdf_obj>();
    if j_info.flags & HAVE_APPn_ICC != 0 {
        let mut icc_stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut intent: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut cspc_id: ::core::ffi::c_int = 0;
        icc_stream = JPEG_get_iccp(&raw mut j_info);
        if icc_stream.is_null() {
            colorspace = ::core::ptr::null_mut::<pdf_obj>();
        } else {
            if iccp_check_colorspace(
                colortype,
                pdf_stream_dataptr(icc_stream),
                pdf_stream_length(icc_stream),
            ) < 0 as ::core::ffi::c_int
            {
                colorspace = ::core::ptr::null_mut::<pdf_obj>();
            } else {
                cspc_id = iccp_load_profile(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    pdf_stream_dataptr(icc_stream),
                    pdf_stream_length(icc_stream),
                );
                if cspc_id < 0 as ::core::ffi::c_int {
                    colorspace = ::core::ptr::null_mut::<pdf_obj>();
                } else {
                    colorspace = pdf_get_colorspace_reference(cspc_id);
                    intent = iccp_get_rendering_intent(
                        pdf_stream_dataptr(icc_stream),
                        pdf_stream_length(icc_stream),
                    );
                    if !intent.is_null() {
                        pdf_add_dict(
                            stream_dict,
                            pdf_new_name(b"Intent\0" as *const u8 as *const ::core::ffi::c_char),
                            intent,
                        );
                    }
                }
            }
            pdf_release_obj(icc_stream);
        }
    }
    if colorspace.is_null() {
        match colortype {
            PDF_COLORSPACE_TYPE_GRAY => {
                colorspace =
                    pdf_new_name(b"DeviceGray\0" as *const u8 as *const ::core::ffi::c_char);
            }
            PDF_COLORSPACE_TYPE_RGB => {
                colorspace =
                    pdf_new_name(b"DeviceRGB\0" as *const u8 as *const ::core::ffi::c_char);
            }
            PDF_COLORSPACE_TYPE_CMYK => {
                colorspace =
                    pdf_new_name(b"DeviceCMYK\0" as *const u8 as *const ::core::ffi::c_char);
            }
            _ => {}
        }
    }
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"ColorSpace\0" as *const u8 as *const ::core::ffi::c_char),
        colorspace,
    );
    if j_info.flags & HAVE_APPn_ADOBE != 0
        && j_info.num_components as ::core::ffi::c_int == 4 as ::core::ffi::c_int
    {
        let mut decode: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut i: ::core::ffi::c_uint = 0;
        pdf_warning!(
            b"Adobe CMYK JPEG: Inverted color assumed.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        decode = pdf_new_array();
        i = 0 as ::core::ffi::c_uint;
        while i < j_info.num_components as ::core::ffi::c_uint {
            pdf_add_array(decode, pdf_new_number(1.0f64));
            pdf_add_array(decode, pdf_new_number(0.0f64));
            i = i.wrapping_add(1);
        }
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"Decode\0" as *const u8 as *const ::core::ffi::c_char),
            decode,
        );
    }
    JPEG_copy_stream(&raw mut j_info, stream, handle);
    info.width = j_info.width as ::core::ffi::c_int;
    info.height = j_info.height as ::core::ffi::c_int;
    info.bits_per_component = j_info.bits_per_component as ::core::ffi::c_int;
    info.num_components = j_info.num_components as ::core::ffi::c_int;
    jpeg_get_density(
        &raw mut j_info,
        &raw mut info.xdensity,
        &raw mut info.ydensity,
    );
    pdf_ximage_set_image(ximage, &raw mut info as *mut ::core::ffi::c_void, stream);
    JPEG_info_clear(&raw mut j_info);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn jpeg_get_density(
    mut j_info: *mut JPEG_info,
    mut xdensity: *mut ::core::ffi::c_double,
    mut ydensity: *mut ::core::ffi::c_double,
) {
    if (*j_info).xdpi < 0.1f64 && (*j_info).ydpi < 0.1f64 {
        (*j_info).ydpi = 72.0f64;
        (*j_info).xdpi = (*j_info).ydpi;
    }
    *xdensity = 72.0f64 / (*j_info).xdpi;
    *ydensity = 72.0f64 / (*j_info).ydpi;
}
unsafe extern "C-unwind" fn JPEG_info_init(mut j_info: *mut JPEG_info) {
    (*j_info).width = 0 as uint16_t;
    (*j_info).height = 0 as uint16_t;
    (*j_info).bits_per_component = 0 as uint8_t;
    (*j_info).num_components = 0 as uint8_t;
    (*j_info).xdpi = 0.0f64;
    (*j_info).ydpi = 0.0f64;
    (*j_info).flags = 0 as ::core::ffi::c_int;
    (*j_info).num_appn = 0 as ::core::ffi::c_int;
    (*j_info).max_appn = 0 as ::core::ffi::c_int;
    (*j_info).appn = ::core::ptr::null_mut::<JPEG_ext>();
    memset(
        &raw mut (*j_info).skipbits as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (MAX_COUNT / 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
    );
}
unsafe extern "C-unwind" fn JPEG_release_APPn_data(
    mut marker: JPEG_marker,
    mut app_sig: JPEG_APPn_sig,
    mut app_data: *mut ::core::ffi::c_void,
) {
    if marker as ::core::ffi::c_uint == JM_APP0 as ::core::ffi::c_int as ::core::ffi::c_uint
        && app_sig as ::core::ffi::c_uint
            == JS_APPn_JFIF as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut data: *mut JPEG_APPn_JFIF = ::core::ptr::null_mut::<JPEG_APPn_JFIF>();
        data = app_data as *mut JPEG_APPn_JFIF;
        (*data).thumbnail =
            mfree((*data).thumbnail as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_uchar;
        free(data as *mut ::core::ffi::c_void);
    } else if marker as ::core::ffi::c_uint == JM_APP2 as ::core::ffi::c_int as ::core::ffi::c_uint
        && app_sig as ::core::ffi::c_uint
            == JS_APPn_ICC as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut data_0: *mut JPEG_APPn_ICC = ::core::ptr::null_mut::<JPEG_APPn_ICC>();
        data_0 = app_data as *mut JPEG_APPn_ICC;
        (*data_0).chunk =
            mfree((*data_0).chunk as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_uchar;
        free(data_0 as *mut ::core::ffi::c_void);
    } else if marker as ::core::ffi::c_uint == JM_APP14 as ::core::ffi::c_int as ::core::ffi::c_uint
        && app_sig as ::core::ffi::c_uint
            == JS_APPn_ADOBE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut data_1: *mut JPEG_APPn_Adobe = ::core::ptr::null_mut::<JPEG_APPn_Adobe>();
        data_1 = app_data as *mut JPEG_APPn_Adobe;
        free(data_1 as *mut ::core::ffi::c_void);
    } else if marker as ::core::ffi::c_uint == JM_APP1 as ::core::ffi::c_int as ::core::ffi::c_uint
        && app_sig as ::core::ffi::c_uint
            == JS_APPn_XMP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut data_2: *mut JPEG_APPn_XMP = ::core::ptr::null_mut::<JPEG_APPn_XMP>();
        data_2 = app_data as *mut JPEG_APPn_XMP;
        free((*data_2).packet as *mut ::core::ffi::c_void);
        free(data_2 as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C-unwind" fn JPEG_info_clear(mut j_info: *mut JPEG_info) {
    if (*j_info).num_appn > 0 as ::core::ffi::c_int && !(*j_info).appn.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < (*j_info).num_appn {
            JPEG_release_APPn_data(
                (*(*j_info).appn.offset(i as isize)).marker,
                (*(*j_info).appn.offset(i as isize)).app_sig,
                (*(*j_info).appn.offset(i as isize)).app_data,
            );
            i += 1;
        }
        free((*j_info).appn as *mut ::core::ffi::c_void);
    }
    (*j_info).appn = ::core::ptr::null_mut::<JPEG_ext>();
    (*j_info).num_appn = 0 as ::core::ffi::c_int;
    (*j_info).max_appn = 0 as ::core::ffi::c_int;
    (*j_info).flags = 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn JPEG_get_iccp(mut j_info: *mut JPEG_info) -> *mut pdf_obj {
    let mut icc_stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut icc: *mut JPEG_APPn_ICC = ::core::ptr::null_mut::<JPEG_APPn_ICC>();
    let mut i: ::core::ffi::c_int = 0;
    let mut prev_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num_icc_seg: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    icc_stream = pdf_new_stream(STREAM_COMPRESS);
    i = 0 as ::core::ffi::c_int;
    while i < (*j_info).num_appn {
        if !((*(*j_info).appn.offset(i as isize)).marker as ::core::ffi::c_uint
            != JM_APP2 as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*(*j_info).appn.offset(i as isize)).app_sig as ::core::ffi::c_uint
                != JS_APPn_ICC as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            icc = (*(*j_info).appn.offset(i as isize)).app_data as *mut JPEG_APPn_ICC;
            if num_icc_seg < 0 as ::core::ffi::c_int && prev_id == 0 as ::core::ffi::c_int {
                num_icc_seg = (*icc).num_chunks as ::core::ffi::c_int;
            } else if (*icc).seq_id as ::core::ffi::c_int != prev_id + 1 as ::core::ffi::c_int
                || num_icc_seg != (*icc).num_chunks as ::core::ffi::c_int
                || (*icc).seq_id as ::core::ffi::c_int > (*icc).num_chunks as ::core::ffi::c_int
            {
                pdf_warning!(
                    b"Invalid JPEG ICC chunk: %d (p:%d, n:%d)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*icc).seq_id as ::core::ffi::c_int,
                    prev_id,
                    (*icc).num_chunks as ::core::ffi::c_int,
                );
                pdf_release_obj(icc_stream);
                icc_stream = ::core::ptr::null_mut::<pdf_obj>();
                break;
            }
            pdf_add_stream(
                icc_stream,
                (*icc).chunk as *const ::core::ffi::c_void,
                (*icc).length as ::core::ffi::c_int,
            );
            prev_id = (*icc).seq_id as ::core::ffi::c_int;
            num_icc_seg = (*icc).num_chunks as ::core::ffi::c_int;
        }
        i += 1;
    }
    return icc_stream;
}
unsafe extern "C-unwind" fn JPEG_get_XMP(mut j_info: *mut JPEG_info) -> *mut pdf_obj {
    let mut XMP_stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut XMP: *mut JPEG_APPn_XMP = ::core::ptr::null_mut::<JPEG_APPn_XMP>();
    let mut i: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    XMP_stream = pdf_new_stream(STREAM_COMPRESS);
    stream_dict = pdf_stream_dict(XMP_stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Metadata\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"XML\0" as *const u8 as *const ::core::ffi::c_char),
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*j_info).num_appn {
        if !((*(*j_info).appn.offset(i as isize)).marker as ::core::ffi::c_uint
            != JM_APP1 as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*(*j_info).appn.offset(i as isize)).app_sig as ::core::ffi::c_uint
                != JS_APPn_XMP as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            XMP = (*(*j_info).appn.offset(i as isize)).app_data as *mut JPEG_APPn_XMP;
            pdf_add_stream(
                XMP_stream,
                (*XMP).packet as *const ::core::ffi::c_void,
                (*XMP).length as ::core::ffi::c_int,
            );
            count += 1;
        }
        i += 1;
    }
    if count > 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"%s: Multiple XMP segments found in JPEG file. (untested)\0" as *const u8
                as *const ::core::ffi::c_char,
            JPEG_DEBUG_STR.as_ptr(),
        );
    }
    return XMP_stream;
}
unsafe extern "C-unwind" fn JPEG_get_marker(mut handle: rust_input_handle_t) -> JPEG_marker {
    let mut c: ::core::ffi::c_int = 0;
    c = ttstub_input_getc(handle);
    if c != 255 as ::core::ffi::c_int {
        return 4294967295 as JPEG_marker;
    }
    loop {
        c = ttstub_input_getc(handle);
        if c < 0 as ::core::ffi::c_int {
            return 4294967295 as JPEG_marker;
        } else if c > 0 as ::core::ffi::c_int && c < 255 as ::core::ffi::c_int {
            return c as JPEG_marker;
        }
    }
}
unsafe extern "C-unwind" fn add_APPn_marker(
    mut j_info: *mut JPEG_info,
    mut marker: JPEG_marker,
    mut app_sig: ::core::ffi::c_int,
    mut app_data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    if (*j_info).num_appn >= (*j_info).max_appn {
        (*j_info).max_appn += 16 as ::core::ffi::c_int;
        (*j_info).appn = renew(
            (*j_info).appn as *mut ::core::ffi::c_void,
            ((*j_info).max_appn as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<JPEG_ext>() as usize) as uint32_t,
        ) as *mut JPEG_ext;
    }
    n = (*j_info).num_appn;
    (*(*j_info).appn.offset(n as isize)).marker = marker;
    (*(*j_info).appn.offset(n as isize)).app_sig = app_sig as JPEG_APPn_sig;
    let ref mut fresh0 = (*(*j_info).appn.offset(n as isize)).app_data;
    *fresh0 = app_data;
    (*j_info).num_appn += 1 as ::core::ffi::c_int;
    return n;
}
unsafe extern "C-unwind" fn read_APP14_Adobe(
    mut j_info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_ushort {
    let mut app_data: *mut JPEG_APPn_Adobe = ::core::ptr::null_mut::<JPEG_APPn_Adobe>();
    app_data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<JPEG_APPn_Adobe>() as usize)
        as uint32_t) as *mut JPEG_APPn_Adobe;
    (*app_data).version = tt_get_unsigned_pair(handle) as uint16_t;
    (*app_data).flag0 = tt_get_unsigned_pair(handle) as uint16_t;
    (*app_data).flag1 = tt_get_unsigned_pair(handle) as uint16_t;
    (*app_data).transform = tt_get_unsigned_byte(handle) as uint8_t;
    add_APPn_marker(
        j_info,
        JM_APP14,
        JS_APPn_ADOBE as ::core::ffi::c_int,
        app_data as *mut ::core::ffi::c_void,
    );
    return 7 as ::core::ffi::c_ushort;
}
pub const JPEG_EXIF_BIGENDIAN: ::core::ffi::c_int = 0;
pub const JPEG_EXIF_LITTLEENDIAN: ::core::ffi::c_int = 1;
unsafe extern "C-unwind" fn read_exif_bytes(
    mut pp: *mut *mut ::core::ffi::c_uchar,
    mut n: ::core::ffi::c_int,
    mut endian: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut rval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_uchar = *pp;
    let mut i: ::core::ffi::c_int = 0;
    match endian {
        JPEG_EXIF_BIGENDIAN => {
            i = 0 as ::core::ffi::c_int;
            while i < n {
                rval =
                    (rval << 8 as ::core::ffi::c_int) + *p.offset(i as isize) as ::core::ffi::c_int;
                i += 1;
            }
        }
        JPEG_EXIF_LITTLEENDIAN => {
            i = n - 1 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                rval =
                    (rval << 8 as ::core::ffi::c_int) + *p.offset(i as isize) as ::core::ffi::c_int;
                i -= 1;
            }
        }
        _ => {}
    }
    *pp = (*pp).offset(n as isize);
    return rval;
}
pub const JPEG_EXIF_TYPE_BYTE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const JPEG_EXIF_TYPE_ASCII: ::core::ffi::c_int = 2;
pub const JPEG_EXIF_TYPE_SHORT: ::core::ffi::c_int = 3;
pub const JPEG_EXIF_TYPE_LONG: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JPEG_EXIF_TYPE_RATIONAL: ::core::ffi::c_int = 5;
pub const JPEG_EXIF_TYPE_UNDEFINED: ::core::ffi::c_int = 7;
pub const JPEG_EXIF_TYPE_SLONG: ::core::ffi::c_int = 9;
pub const JPEG_EXIF_TYPE_SRATIONAL: ::core::ffi::c_int = 10;
pub const JPEG_EXIF_TAG_XRESOLUTION: ::core::ffi::c_int = 282;
pub const JPEG_EXIF_TAG_YRESOLUTION: ::core::ffi::c_int = 283;
pub const JPEG_EXIF_TAG_RESOLUTIONUNIT: ::core::ffi::c_int = 296;
pub const JPEG_EXIF_TAG_RESUNIT_MS: ::core::ffi::c_int = 20752;
pub const JPEG_EXIF_TAG_XRES_MS: ::core::ffi::c_int = 20753;
pub const JPEG_EXIF_TAG_YRES_MS: ::core::ffi::c_int = 20754;
unsafe extern "C-unwind" fn read_APP1_Exif(
    mut info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
    mut length: size_t,
) -> size_t {
    let mut current_block: u64;
    let mut buffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut endptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut rp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut tiff_header: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut bigendian: ::core::ffi::c_char = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut num_fields: ::core::ffi::c_int = 0;
    let mut tag: ::core::ffi::c_int = 0;
    let mut type_0: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_int = 0;
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut den: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut xres: ::core::ffi::c_double = 0.0f64;
    let mut yres: ::core::ffi::c_double = 0.0f64;
    let mut res_unit: ::core::ffi::c_double = 1.0f64;
    let mut xres_ms: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut yres_ms: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut res_unit_ms: ::core::ffi::c_double = 0.0f64;
    let mut exifxdpi: ::core::ffi::c_double = 0.0f64;
    let mut exifydpi: ::core::ffi::c_double = 0.0f64;
    let mut r: ssize_t = 0;
    buffer = crate::backend::runtime::allocate(length) as *mut ::core::ffi::c_uchar;
    r = ttstub_input_read(handle, buffer as *mut ::core::ffi::c_char, length);
    if !(r < 0 as ssize_t || r as size_t != length) {
        p = buffer;
        endptr = buffer.offset(length as isize);
        while p < buffer.offset(length as isize)
            && *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            p = p.offset(1);
        }
        if !(p.offset(8 as ::core::ffi::c_int as isize) >= endptr) {
            tiff_header = p;
            if *p as ::core::ffi::c_int == 'M' as i32
                && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'M' as i32
            {
                bigendian = JPEG_EXIF_BIGENDIAN as ::core::ffi::c_char;
                current_block = 15652330335145281839;
            } else if *p as ::core::ffi::c_int == 'I' as i32
                && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'I' as i32
            {
                bigendian = JPEG_EXIF_LITTLEENDIAN as ::core::ffi::c_char;
                current_block = 15652330335145281839;
            } else {
                pdf_warning!(
                    b"JPEG: Invalid value in Exif TIFF header.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                current_block = 17070205778953429383;
            }
            match current_block {
                17070205778953429383 => {}
                _ => {
                    p = p.offset(2 as ::core::ffi::c_int as isize);
                    i = read_exif_bytes(
                        &raw mut p,
                        2 as ::core::ffi::c_int,
                        bigendian as ::core::ffi::c_int,
                    );
                    if i != 42 as ::core::ffi::c_int {
                        pdf_warning!(
                            b"JPEG: Invalid value in Exif TIFF header.\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        i = read_exif_bytes(
                            &raw mut p,
                            4 as ::core::ffi::c_int,
                            bigendian as ::core::ffi::c_int,
                        );
                        p = tiff_header.offset(i as isize);
                        num_fields = read_exif_bytes(
                            &raw mut p,
                            2 as ::core::ffi::c_int,
                            bigendian as ::core::ffi::c_int,
                        );
                        loop {
                            let fresh1 = num_fields;
                            num_fields = num_fields - 1;
                            if !(fresh1 > 0 as ::core::ffi::c_int) {
                                current_block = 5372832139739605200;
                                break;
                            }
                            let mut count: ::core::ffi::c_int = 0;
                            tag = read_exif_bytes(
                                &raw mut p,
                                2 as ::core::ffi::c_int,
                                bigendian as ::core::ffi::c_int,
                            );
                            type_0 = read_exif_bytes(
                                &raw mut p,
                                2 as ::core::ffi::c_int,
                                bigendian as ::core::ffi::c_int,
                            );
                            count = read_exif_bytes(
                                &raw mut p,
                                4 as ::core::ffi::c_int,
                                bigendian as ::core::ffi::c_int,
                            );
                            match type_0 {
                                JPEG_EXIF_TYPE_BYTE => {
                                    let fresh2 = p;
                                    p = p.offset(1);
                                    value = *fresh2 as ::core::ffi::c_int;
                                    p = p.offset(3 as ::core::ffi::c_int as isize);
                                }
                                JPEG_EXIF_TYPE_SHORT => {
                                    value = read_exif_bytes(
                                        &raw mut p,
                                        2 as ::core::ffi::c_int,
                                        bigendian as ::core::ffi::c_int,
                                    );
                                    p = p.offset(2 as ::core::ffi::c_int as isize);
                                }
                                JPEG_EXIF_TYPE_LONG | JPEG_EXIF_TYPE_SLONG => {
                                    value = read_exif_bytes(
                                        &raw mut p,
                                        4 as ::core::ffi::c_int,
                                        bigendian as ::core::ffi::c_int,
                                    );
                                }
                                JPEG_EXIF_TYPE_RATIONAL | JPEG_EXIF_TYPE_SRATIONAL => {
                                    value = read_exif_bytes(
                                        &raw mut p,
                                        4 as ::core::ffi::c_int,
                                        bigendian as ::core::ffi::c_int,
                                    );
                                    rp = tiff_header.offset(value as isize);
                                    num = read_exif_bytes(
                                        &raw mut rp,
                                        4 as ::core::ffi::c_int,
                                        bigendian as ::core::ffi::c_int,
                                    );
                                    den = read_exif_bytes(
                                        &raw mut rp,
                                        4 as ::core::ffi::c_int,
                                        bigendian as ::core::ffi::c_int,
                                    );
                                }
                                JPEG_EXIF_TYPE_UNDEFINED => {
                                    let fresh3 = p;
                                    p = p.offset(1);
                                    value = *fresh3 as ::core::ffi::c_int;
                                    p = p.offset(3 as ::core::ffi::c_int as isize);
                                }
                                JPEG_EXIF_TYPE_ASCII | _ => {
                                    value = 0 as ::core::ffi::c_int;
                                    p = p.offset(4 as ::core::ffi::c_int as isize);
                                }
                            }
                            match tag {
                                JPEG_EXIF_TAG_XRESOLUTION => {
                                    if den != 0 as ::core::ffi::c_int {
                                        xres = (num / den) as ::core::ffi::c_double;
                                    }
                                }
                                JPEG_EXIF_TAG_YRESOLUTION => {
                                    if den != 0 as ::core::ffi::c_int {
                                        yres = (num / den) as ::core::ffi::c_double;
                                    }
                                }
                                JPEG_EXIF_TAG_RESOLUTIONUNIT => match value {
                                    2 => {
                                        res_unit = 1.0f64;
                                    }
                                    3 => {
                                        res_unit = 2.54f64;
                                    }
                                    _ => {}
                                },
                                JPEG_EXIF_TAG_RESUNIT_MS => {
                                    if type_0 != JPEG_EXIF_TYPE_BYTE
                                        || count != 1 as ::core::ffi::c_int
                                    {
                                        pdf_warning!(
                                            b"%s: Invalid data for ResolutionUnit in Exif chunk.\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char,
                                            JPEG_DEBUG_STR.as_ptr(),
                                        );
                                        current_block = 17070205778953429383;
                                        break;
                                    } else {
                                        value = read_exif_bytes(
                                            &raw mut p,
                                            1 as ::core::ffi::c_int,
                                            bigendian as ::core::ffi::c_int,
                                        );
                                        p = p.offset(3 as ::core::ffi::c_int as isize);
                                        if value == 1 as ::core::ffi::c_int {
                                            res_unit_ms = 0.0254f64;
                                        } else {
                                            res_unit_ms = 0.0f64;
                                        }
                                    }
                                }
                                JPEG_EXIF_TAG_XRES_MS => {
                                    if type_0 != JPEG_EXIF_TYPE_LONG
                                        || count != 1 as ::core::ffi::c_int
                                    {
                                        pdf_warning!(
                                            b"%s: Invalid data for PixelPerUnitX in Exif chunk.\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char,
                                            JPEG_DEBUG_STR.as_ptr(),
                                        );
                                        current_block = 17070205778953429383;
                                        break;
                                    } else {
                                        value = read_exif_bytes(
                                            &raw mut p,
                                            4 as ::core::ffi::c_int,
                                            bigendian as ::core::ffi::c_int,
                                        );
                                        xres_ms = value as ::core::ffi::c_uint;
                                    }
                                }
                                JPEG_EXIF_TAG_YRES_MS => {
                                    if type_0 != JPEG_EXIF_TYPE_LONG
                                        || count != 1 as ::core::ffi::c_int
                                    {
                                        pdf_warning!(
                                            b"%s: Invalid data for PixelPerUnitY in Exif chunk.\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char,
                                            JPEG_DEBUG_STR.as_ptr(),
                                        );
                                        current_block = 17070205778953429383;
                                        break;
                                    } else {
                                        value = read_exif_bytes(
                                            &raw mut p,
                                            4 as ::core::ffi::c_int,
                                            bigendian as ::core::ffi::c_int,
                                        );
                                        yres_ms = value as ::core::ffi::c_uint;
                                    }
                                }
                                _ => {}
                            }
                        }
                        match current_block {
                            17070205778953429383 => {}
                            _ => {
                                if xres > 0.0f64 && yres > 0.0f64 {
                                    exifxdpi = xres * res_unit;
                                    exifydpi = yres * res_unit;
                                } else if xres_ms > 0 as ::core::ffi::c_uint
                                    && yres_ms > 0 as ::core::ffi::c_uint
                                    && res_unit_ms > 0.0f64
                                {
                                    exifxdpi = xres_ms as ::core::ffi::c_double * res_unit_ms;
                                    exifydpi = yres_ms as ::core::ffi::c_double * res_unit_ms;
                                } else {
                                    exifxdpi = 72.0f64 * res_unit;
                                    exifydpi = 72.0f64 * res_unit;
                                }
                                if (*info).xdpi < 0.1f64 && (*info).ydpi < 0.1f64 {
                                    (*info).xdpi = exifxdpi;
                                    (*info).ydpi = exifydpi;
                                } else {
                                    let mut xxx1: ::core::ffi::c_double = floor(exifxdpi + 0.5f64);
                                    let mut xxx2: ::core::ffi::c_double =
                                        floor((*info).xdpi + 0.5f64);
                                    let mut yyy1: ::core::ffi::c_double = floor(exifydpi + 0.5f64);
                                    let mut yyy2: ::core::ffi::c_double =
                                        floor((*info).ydpi + 0.5f64);
                                    if xxx1 != xxx2 || yyy1 != yyy2 {
                                        pdf_warning!(
                                            b"JPEG: Inconsistent resolution may have been specified in Exif and JFIF: %gx%g - %gx%g\0"
                                            as * const u8 as * const ::core::ffi::c_char, xres *
                                            res_unit, yres * res_unit, (* info).xdpi, (* info).ydpi,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    free(buffer as *mut ::core::ffi::c_void);
    return length;
}
unsafe extern "C-unwind" fn read_APP0_JFIF(
    mut j_info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
) -> size_t {
    let mut app_data: *mut JPEG_APPn_JFIF = ::core::ptr::null_mut::<JPEG_APPn_JFIF>();
    let mut thumb_data_len: size_t = 0;
    app_data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<JPEG_APPn_JFIF>() as usize)
        as uint32_t) as *mut JPEG_APPn_JFIF;
    (*app_data).version = tt_get_unsigned_pair(handle) as uint16_t;
    (*app_data).units = tt_get_unsigned_byte(handle) as uint8_t;
    (*app_data).Xdensity = tt_get_unsigned_pair(handle) as uint16_t;
    (*app_data).Ydensity = tt_get_unsigned_pair(handle) as uint16_t;
    (*app_data).Xthumbnail = tt_get_unsigned_byte(handle) as uint8_t;
    (*app_data).Ythumbnail = tt_get_unsigned_byte(handle) as uint8_t;
    thumb_data_len = (3 as ::core::ffi::c_int
        * (*app_data).Xthumbnail as ::core::ffi::c_int
        * (*app_data).Ythumbnail as ::core::ffi::c_int) as size_t;
    if thumb_data_len > 0 as size_t {
        (*app_data).thumbnail = new((thumb_data_len as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        ttstub_input_read(
            handle,
            (*app_data).thumbnail as *mut ::core::ffi::c_char,
            thumb_data_len,
        );
    } else {
        (*app_data).thumbnail = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    add_APPn_marker(
        j_info,
        JM_APP0,
        JS_APPn_JFIF as ::core::ffi::c_int,
        app_data as *mut ::core::ffi::c_void,
    );
    match (*app_data).units as ::core::ffi::c_int {
        1 => {
            (*j_info).xdpi = (*app_data).Xdensity as ::core::ffi::c_double;
            (*j_info).ydpi = (*app_data).Ydensity as ::core::ffi::c_double;
        }
        2 => {
            (*j_info).xdpi =
                (*app_data).Xdensity as ::core::ffi::c_int as ::core::ffi::c_double * 2.54f64;
            (*j_info).ydpi =
                (*app_data).Ydensity as ::core::ffi::c_int as ::core::ffi::c_double * 2.54f64;
        }
        _ => {
            (*j_info).xdpi = 72.0f64;
            (*j_info).ydpi = 72.0f64;
        }
    }
    return (9 as size_t).wrapping_add(thumb_data_len);
}
unsafe extern "C-unwind" fn read_APP0_JFXX(
    mut handle: rust_input_handle_t,
    mut length: size_t,
) -> size_t {
    tt_get_unsigned_byte(handle);
    crate::backend::runtime::input_seek(
        handle,
        length.wrapping_sub(1 as size_t) as ssize_t,
        SEEK_CUR,
    );
    return length;
}
unsafe extern "C-unwind" fn read_APP1_XMP(
    mut j_info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
    mut length: size_t,
) -> size_t {
    let mut app_data: *mut JPEG_APPn_XMP = ::core::ptr::null_mut::<JPEG_APPn_XMP>();
    app_data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<JPEG_APPn_XMP>() as usize)
        as uint32_t) as *mut JPEG_APPn_XMP;
    (*app_data).length = length;
    (*app_data).packet = new(((*app_data).length as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    ttstub_input_read(
        handle,
        (*app_data).packet as *mut ::core::ffi::c_char,
        (*app_data).length,
    );
    add_APPn_marker(
        j_info,
        JM_APP1,
        JS_APPn_XMP as ::core::ffi::c_int,
        app_data as *mut ::core::ffi::c_void,
    );
    return length;
}
unsafe extern "C-unwind" fn read_APP2_ICC(
    mut j_info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
    mut length: size_t,
) -> size_t {
    let mut app_data: *mut JPEG_APPn_ICC = ::core::ptr::null_mut::<JPEG_APPn_ICC>();
    app_data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<JPEG_APPn_ICC>() as usize)
        as uint32_t) as *mut JPEG_APPn_ICC;
    (*app_data).seq_id = tt_get_unsigned_byte(handle) as uint8_t;
    (*app_data).num_chunks = tt_get_unsigned_byte(handle) as uint8_t;
    (*app_data).length = length.wrapping_sub(2 as size_t);
    (*app_data).chunk = new(((*app_data).length as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    ttstub_input_read(
        handle,
        (*app_data).chunk as *mut ::core::ffi::c_char,
        (*app_data).length,
    );
    add_APPn_marker(
        j_info,
        JM_APP2,
        JS_APPn_ICC as ::core::ffi::c_int,
        app_data as *mut ::core::ffi::c_void,
    );
    return length;
}
unsafe extern "C-unwind" fn JPEG_copy_stream(
    mut j_info: *mut JPEG_info,
    mut stream: *mut pdf_obj,
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut marker: JPEG_marker = 0 as JPEG_marker;
    let mut length: ::core::ffi::c_int = 0;
    let mut found_SOFn: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    count = 0 as ::core::ffi::c_int;
    found_SOFn = 0 as ::core::ffi::c_int;
    while found_SOFn == 0 && count < MAX_COUNT && {
        marker = JPEG_get_marker(handle);
        marker as ::core::ffi::c_uint != 4294967295 as JPEG_marker as ::core::ffi::c_uint
    } {
        if marker as ::core::ffi::c_uint == JM_SOI as ::core::ffi::c_int as ::core::ffi::c_uint
            || marker as ::core::ffi::c_uint >= JM_RST0 as ::core::ffi::c_int as ::core::ffi::c_uint
                && marker as ::core::ffi::c_uint
                    <= JM_RST7 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            *(&raw mut work_buffer as *mut ::core::ffi::c_char)
                .offset(0 as ::core::ffi::c_int as isize) =
                0xff as ::core::ffi::c_int as ::core::ffi::c_char;
            *(&raw mut work_buffer as *mut ::core::ffi::c_char)
                .offset(1 as ::core::ffi::c_int as isize) = marker as ::core::ffi::c_char;
            pdf_add_stream(
                stream,
                &raw mut work_buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                2 as ::core::ffi::c_int,
            );
        } else {
            length = tt_get_unsigned_pair(handle) as ::core::ffi::c_int - 2 as ::core::ffi::c_int;
            match marker as ::core::ffi::c_uint {
                192 | 193 | 194 | 195 | 197 | 198 | 199 | 201 | 202 | 203 | 205 | 206 | 207 => {
                    *(&raw mut work_buffer as *mut ::core::ffi::c_char)
                        .offset(0 as ::core::ffi::c_int as isize) =
                        0xff as ::core::ffi::c_int as ::core::ffi::c_char;
                    *(&raw mut work_buffer as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize) = marker as ::core::ffi::c_char;
                    *(&raw mut work_buffer as *mut ::core::ffi::c_char)
                        .offset(2 as ::core::ffi::c_int as isize) =
                        (length + 2 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    *(&raw mut work_buffer as *mut ::core::ffi::c_char)
                        .offset(3 as ::core::ffi::c_int as isize) =
                        (length + 2 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    pdf_add_stream(
                        stream,
                        &raw mut work_buffer as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        4 as ::core::ffi::c_int,
                    );
                    while length > 0 as ::core::ffi::c_int {
                        let mut nb_read: ::core::ffi::c_int = ttstub_input_read(
                            handle,
                            &raw mut work_buffer as *mut ::core::ffi::c_char,
                            (if length < 1024 as ::core::ffi::c_int {
                                length
                            } else {
                                1024 as ::core::ffi::c_int
                            }) as size_t,
                        )
                            as ::core::ffi::c_int;
                        if nb_read > 0 as ::core::ffi::c_int {
                            pdf_add_stream(
                                stream,
                                &raw mut work_buffer as *mut ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                nb_read,
                            );
                        }
                        length -= nb_read;
                    }
                    found_SOFn = 1 as ::core::ffi::c_int;
                }
                _ => {
                    if (*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int)
                            << 7 as ::core::ffi::c_int - count % 8 as ::core::ffi::c_int
                        != 0
                    {
                        crate::backend::runtime::input_seek(handle, length as ssize_t, SEEK_CUR);
                    } else {
                        *(&raw mut work_buffer as *mut ::core::ffi::c_char)
                            .offset(0 as ::core::ffi::c_int as isize) =
                            0xff as ::core::ffi::c_int as ::core::ffi::c_char;
                        *(&raw mut work_buffer as *mut ::core::ffi::c_char)
                            .offset(1 as ::core::ffi::c_int as isize) =
                            marker as ::core::ffi::c_char;
                        *(&raw mut work_buffer as *mut ::core::ffi::c_char)
                            .offset(2 as ::core::ffi::c_int as isize) =
                            (length + 2 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                                & 0xff as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        *(&raw mut work_buffer as *mut ::core::ffi::c_char)
                            .offset(3 as ::core::ffi::c_int as isize) =
                            (length + 2 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                                as ::core::ffi::c_char;
                        pdf_add_stream(
                            stream,
                            &raw mut work_buffer as *mut ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                        );
                        while length > 0 as ::core::ffi::c_int {
                            let mut nb_read_0: ::core::ffi::c_int = ttstub_input_read(
                                handle,
                                &raw mut work_buffer as *mut ::core::ffi::c_char,
                                (if length < 1024 as ::core::ffi::c_int {
                                    length
                                } else {
                                    1024 as ::core::ffi::c_int
                                }) as size_t,
                            )
                                as ::core::ffi::c_int;
                            if nb_read_0 > 0 as ::core::ffi::c_int {
                                pdf_add_stream(
                                    stream,
                                    &raw mut work_buffer as *mut ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                    nb_read_0,
                                );
                            }
                            length -= nb_read_0;
                        }
                    }
                }
            }
        }
        count += 1;
    }
    let mut total_size: size_t = ttstub_input_get_size(handle);
    let mut pos: size_t = crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_CUR);
    loop {
        length = ttstub_input_read(
            handle,
            &raw mut work_buffer as *mut ::core::ffi::c_char,
            (if (1024 as size_t) < total_size.wrapping_sub(pos) {
                1024 as size_t
            } else {
                total_size.wrapping_sub(pos)
            }),
        ) as ::core::ffi::c_int;
        if !(length > 0 as ::core::ffi::c_int) {
            break;
        }
        pdf_add_stream(
            stream,
            &raw mut work_buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            length,
        );
        pos = pos.wrapping_add(length as size_t);
    }
    return if found_SOFn != 0 {
        0 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
unsafe extern "C-unwind" fn JPEG_scan_file(
    mut j_info: *mut JPEG_info,
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut marker: JPEG_marker = 0 as JPEG_marker;
    let mut found_SOFn: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut app_sig: [::core::ffi::c_char; 128] = [0; 128];
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    count = 0 as ::core::ffi::c_int;
    found_SOFn = 0 as ::core::ffi::c_int;
    while found_SOFn == 0 && {
        marker = JPEG_get_marker(handle);
        marker as ::core::ffi::c_uint != 4294967295 as JPEG_marker as ::core::ffi::c_uint
    } {
        if marker as ::core::ffi::c_uint != JM_SOI as ::core::ffi::c_int as ::core::ffi::c_uint
            && ((marker as ::core::ffi::c_uint)
                < JM_RST0 as ::core::ffi::c_int as ::core::ffi::c_uint
                || marker as ::core::ffi::c_uint
                    > JM_RST7 as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            let mut length: ::core::ffi::c_int =
                tt_get_unsigned_pair(handle) as ::core::ffi::c_int - 2 as ::core::ffi::c_int;
            match marker as ::core::ffi::c_uint {
                192 | 193 | 194 | 195 | 197 | 198 | 199 | 201 | 202 | 203 | 205 | 206 | 207 => {
                    (*j_info).bits_per_component = tt_get_unsigned_byte(handle) as uint8_t;
                    (*j_info).height = tt_get_unsigned_pair(handle) as uint16_t;
                    (*j_info).width = tt_get_unsigned_pair(handle) as uint16_t;
                    (*j_info).num_components = tt_get_unsigned_byte(handle) as uint8_t;
                    found_SOFn = 1 as ::core::ffi::c_int;
                }
                224 => {
                    if length > 5 as ::core::ffi::c_int {
                        if ttstub_input_read(
                            handle,
                            &raw mut app_sig as *mut ::core::ffi::c_char,
                            5 as size_t,
                        ) != 5 as ssize_t
                        {
                            return -(1 as ::core::ffi::c_int);
                        }
                        length -= 5 as ::core::ffi::c_int;
                        if memcmp(
                            &raw mut app_sig as *mut ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            b"JFIF\0\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            5 as size_t,
                        ) == 0
                        {
                            (*j_info).flags |= HAVE_APPn_JFIF;
                            length = (length as size_t).wrapping_sub(read_APP0_JFIF(j_info, handle))
                                as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                        } else if memcmp(
                            &raw mut app_sig as *mut ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            b"JFXX\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            5 as size_t,
                        ) == 0
                        {
                            length = (length as size_t)
                                .wrapping_sub(read_APP0_JFXX(handle, length as size_t))
                                as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                            if count < MAX_COUNT {
                                (*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize] =
                                    ((*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize]
                                        as ::core::ffi::c_int
                                        | (1 as ::core::ffi::c_int)
                                            << 7 as ::core::ffi::c_int
                                                - count % 8 as ::core::ffi::c_int)
                                        as ::core::ffi::c_char;
                            }
                        } else if count < MAX_COUNT {
                            (*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize] =
                                ((*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize]
                                    as ::core::ffi::c_int
                                    | (1 as ::core::ffi::c_int)
                                        << 7 as ::core::ffi::c_int
                                            - count % 8 as ::core::ffi::c_int)
                                    as ::core::ffi::c_char;
                        }
                    } else if count < MAX_COUNT {
                        (*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize] = ((*j_info)
                            .skipbits[(count / 8 as ::core::ffi::c_int) as usize]
                            as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 7 as ::core::ffi::c_int - count % 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    }
                    crate::backend::runtime::input_seek(handle, length as ssize_t, SEEK_CUR);
                }
                225 => {
                    if length > 5 as ::core::ffi::c_int {
                        if ttstub_input_read(
                            handle,
                            &raw mut app_sig as *mut ::core::ffi::c_char,
                            5 as size_t,
                        ) != 5 as ssize_t
                        {
                            return -(1 as ::core::ffi::c_int);
                        }
                        length -= 5 as ::core::ffi::c_int;
                        if memcmp(
                            &raw mut app_sig as *mut ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            b"Exif\0\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            5 as size_t,
                        ) == 0
                        {
                            (*j_info).flags |= HAVE_APPn_Exif;
                            length = (length as size_t).wrapping_sub(read_APP1_Exif(
                                j_info,
                                handle,
                                length as size_t,
                            )) as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                        } else if memcmp(
                            &raw mut app_sig as *mut ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            b"http:\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            5 as size_t,
                        ) == 0
                            && length > 24 as ::core::ffi::c_int
                        {
                            if ttstub_input_read(
                                handle,
                                &raw mut app_sig as *mut ::core::ffi::c_char,
                                24 as size_t,
                            ) != 24 as ssize_t
                            {
                                return -(1 as ::core::ffi::c_int);
                            }
                            length -= 24 as ::core::ffi::c_int;
                            if memcmp(
                                &raw mut app_sig as *mut ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                b"//ns.adobe.com/xap/1.0/\0\0" as *const u8
                                    as *const ::core::ffi::c_char
                                    as *const ::core::ffi::c_void,
                                24 as size_t,
                            ) == 0
                            {
                                (*j_info).flags |= HAVE_APPn_XMP;
                                length = (length as size_t).wrapping_sub(read_APP1_XMP(
                                    j_info,
                                    handle,
                                    length as size_t,
                                )) as ::core::ffi::c_int
                                    as ::core::ffi::c_int;
                            }
                            if count < MAX_COUNT {
                                (*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize] =
                                    ((*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize]
                                        as ::core::ffi::c_int
                                        | (1 as ::core::ffi::c_int)
                                            << 7 as ::core::ffi::c_int
                                                - count % 8 as ::core::ffi::c_int)
                                        as ::core::ffi::c_char;
                            }
                        }
                    } else if count < MAX_COUNT {
                        (*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize] = ((*j_info)
                            .skipbits[(count / 8 as ::core::ffi::c_int) as usize]
                            as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 7 as ::core::ffi::c_int - count % 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    }
                    crate::backend::runtime::input_seek(handle, length as ssize_t, SEEK_CUR);
                }
                226 => {
                    if length >= 14 as ::core::ffi::c_int {
                        if ttstub_input_read(
                            handle,
                            &raw mut app_sig as *mut ::core::ffi::c_char,
                            12 as size_t,
                        ) != 12 as ssize_t
                        {
                            return -(1 as ::core::ffi::c_int);
                        }
                        length -= 12 as ::core::ffi::c_int;
                        if memcmp(
                            &raw mut app_sig as *mut ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            b"ICC_PROFILE\0\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            12 as size_t,
                        ) == 0
                        {
                            (*j_info).flags |= HAVE_APPn_ICC;
                            length = (length as size_t).wrapping_sub(read_APP2_ICC(
                                j_info,
                                handle,
                                length as size_t,
                            )) as ::core::ffi::c_int
                                as ::core::ffi::c_int;
                        }
                    }
                    crate::backend::runtime::input_seek(handle, length as ssize_t, SEEK_CUR);
                    if count < MAX_COUNT {
                        (*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize] = ((*j_info)
                            .skipbits[(count / 8 as ::core::ffi::c_int) as usize]
                            as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 7 as ::core::ffi::c_int - count % 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    }
                }
                238 => {
                    if length > 5 as ::core::ffi::c_int {
                        if ttstub_input_read(
                            handle,
                            &raw mut app_sig as *mut ::core::ffi::c_char,
                            5 as size_t,
                        ) != 5 as ssize_t
                        {
                            return -(1 as ::core::ffi::c_int);
                        }
                        length -= 5 as ::core::ffi::c_int;
                        if memcmp(
                            &raw mut app_sig as *mut ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            b"Adobe\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            5 as size_t,
                        ) == 0
                        {
                            (*j_info).flags |= HAVE_APPn_ADOBE;
                            length -= read_APP14_Adobe(j_info, handle) as ::core::ffi::c_int;
                        } else if count < MAX_COUNT {
                            (*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize] =
                                ((*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize]
                                    as ::core::ffi::c_int
                                    | (1 as ::core::ffi::c_int)
                                        << 7 as ::core::ffi::c_int
                                            - count % 8 as ::core::ffi::c_int)
                                    as ::core::ffi::c_char;
                        }
                    } else if count < MAX_COUNT {
                        (*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize] = ((*j_info)
                            .skipbits[(count / 8 as ::core::ffi::c_int) as usize]
                            as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 7 as ::core::ffi::c_int - count % 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_char;
                    }
                    crate::backend::runtime::input_seek(handle, length as ssize_t, SEEK_CUR);
                }
                _ => {
                    crate::backend::runtime::input_seek(handle, length as ssize_t, SEEK_CUR);
                    if marker as ::core::ffi::c_uint
                        >= JM_APP0 as ::core::ffi::c_int as ::core::ffi::c_uint
                        && marker as ::core::ffi::c_uint
                            <= JM_APP15 as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        if count < MAX_COUNT {
                            (*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize] =
                                ((*j_info).skipbits[(count / 8 as ::core::ffi::c_int) as usize]
                                    as ::core::ffi::c_int
                                    | (1 as ::core::ffi::c_int)
                                        << 7 as ::core::ffi::c_int
                                            - count % 8 as ::core::ffi::c_int)
                                    as ::core::ffi::c_char;
                        }
                    }
                }
            }
        }
        count += 1;
    }
    if (*j_info).xdpi < 0.1f64 && (*j_info).ydpi < 0.1f64 {
        (*j_info).ydpi = 72.0f64;
        (*j_info).xdpi = (*j_info).ydpi;
    }
    return if found_SOFn != 0 {
        0 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
#[unsafe(export_name = "__pdf_impl_jpeg_get_bbox")]
pub unsafe extern "C-unwind" fn jpeg_get_bbox(
    mut handle: rust_input_handle_t,
    mut width: *mut ::core::ffi::c_uint,
    mut height: *mut ::core::ffi::c_uint,
    mut xdensity: *mut ::core::ffi::c_double,
    mut ydensity: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut j_info: JPEG_info = JPEG_info {
        height: 0,
        width: 0,
        bits_per_component: 0,
        num_components: 0,
        xdpi: 0.,
        ydpi: 0.,
        flags: 0,
        num_appn: 0,
        max_appn: 0,
        appn: ::core::ptr::null_mut::<JPEG_ext>(),
        skipbits: [0; 129],
    };
    JPEG_info_init(&raw mut j_info);
    if JPEG_scan_file(&raw mut j_info, handle) < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"%s: Not a JPEG file?\0" as *const u8 as *const ::core::ffi::c_char,
            JPEG_DEBUG_STR.as_ptr(),
        );
        JPEG_info_clear(&raw mut j_info);
        return -(1 as ::core::ffi::c_int);
    }
    *width = j_info.width as ::core::ffi::c_uint;
    *height = j_info.height as ::core::ffi::c_uint;
    jpeg_get_density(&raw mut j_info, xdensity, ydensity);
    JPEG_info_clear(&raw mut j_info);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" {
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_check_for_jpeg(
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { check_for_jpeg(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_jpeg_include_image(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { jpeg_include_image(ximage, handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_jpeg_get_bbox(
    mut handle: rust_input_handle_t,
    mut width: *mut ::core::ffi::c_uint,
    mut height: *mut ::core::ffi::c_uint,
    mut xdensity: *mut ::core::ffi::c_double,
    mut ydensity: *mut ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        jpeg_get_bbox(handle, width, height, xdensity, ydensity)
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
