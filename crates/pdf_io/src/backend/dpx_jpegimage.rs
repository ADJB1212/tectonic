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
pub const JPEG_DEBUG_STR: [::core::ffi::c_char; 5] = crate::backend::c_char_array(*b"JPEG\0");
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
    let Some(mut j_info) = scan_file(handle) else {
        return -1;
    };
    pdf_ximage_init_image_info(&raw mut info);
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
        if !j_info.xmp.is_empty() {
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
    if !j_info.icc.is_empty() {
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
    if j_info.adobe && j_info.num_components as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
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
    if JPEG_copy_stream(&raw mut j_info, stream, handle) < 0 {
        pdf_release_obj(stream);
        return -1;
    }
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
    return 0 as ::core::ffi::c_int;
}
fn scan_file(handle: rust_input_handle_t) -> Option<crate::jpeg::Info> {
    let mut input = crate::compat::input::ProbeInput { handle };
    match crate::jpeg::scan(&mut input, |warning| match warning {
        crate::jpeg::Warning::Message(message) => {
            pdf_warning!("{}", message);
        }
        crate::jpeg::Warning::Resolution { exif, jfif } => {
            pdf_warning!(
                c"JPEG: Inconsistent resolution may have been specified in Exif and JFIF: %gx%g - %gx%g".as_ptr(),
                exif.0, exif.1, jfif.0, jfif.1
            );
        }
    }) {
        Ok(info) => Some(info),
        Err(error) => {
            pdf_warning!("{}", error.message.to_string_lossy());
            None
        }
    }
}

unsafe extern "C-unwind" fn jpeg_get_density(
    info: *mut crate::jpeg::Info,
    xdensity: *mut f64,
    ydensity: *mut f64,
) {
    unsafe {
        *xdensity = 72.0 / (*info).xdpi;
        *ydensity = 72.0 / (*info).ydpi;
    }
}

unsafe extern "C-unwind" fn JPEG_get_iccp(info: *mut crate::jpeg::Info) -> *mut pdf_obj {
    let stream = unsafe { pdf_new_stream(STREAM_COMPRESS) };
    let mut previous = 0;
    let mut count = None;
    for chunk in unsafe { &(*info).icc } {
        if count.is_some_and(|n| {
            n != chunk.count || chunk.id as i32 != previous + 1 || chunk.id > chunk.count
        }) {
            pdf_warning!(
                "Invalid JPEG ICC chunk: {} (p:{}, n:{})",
                chunk.id,
                previous,
                chunk.count
            );
            unsafe {
                pdf_release_obj(stream);
            }
            return core::ptr::null_mut();
        }
        unsafe {
            pdf_add_stream(
                stream,
                chunk.bytes.as_ptr().cast(),
                chunk.bytes.len() as i32,
            );
        }
        previous = chunk.id as i32;
        count = Some(chunk.count);
    }
    stream
}

unsafe extern "C-unwind" fn JPEG_get_XMP(info: *mut crate::jpeg::Info) -> *mut pdf_obj {
    unsafe {
        let stream = pdf_new_stream(STREAM_COMPRESS);
        let dict = pdf_stream_dict(stream);
        pdf_add_dict(
            dict,
            pdf_new_name(c"Type".as_ptr()),
            pdf_new_name(c"Metadata".as_ptr()),
        );
        pdf_add_dict(
            dict,
            pdf_new_name(c"Subtype".as_ptr()),
            pdf_new_name(c"XML".as_ptr()),
        );
        for packet in &(*info).xmp {
            pdf_add_stream(stream, packet.as_ptr().cast(), packet.len() as i32);
        }
        if (*info).xmp.len() > 1 {
            pdf_warning!("JPEG: Multiple XMP segments found in JPEG file. (untested)");
        }
        stream
    }
}

unsafe extern "C-unwind" fn JPEG_copy_stream(
    info: *mut crate::jpeg::Info,
    stream: *mut pdf_obj,
    handle: rust_input_handle_t,
) -> i32 {
    use std::io::{Read, Seek, SeekFrom};
    let info = unsafe { &*info };
    let mut input = crate::compat::input::ProbeInput { handle };
    let copy = || -> std::io::Result<()> {
        let end = input.seek(SeekFrom::End(0))?;
        if info.rest > end {
            return Err(std::io::Error::other("Invalid JPEG data offset"));
        }
        input.seek(SeekFrom::Start(info.rest))?;
        unsafe {
            pdf_add_stream(
                stream,
                info.prefix.as_ptr().cast(),
                info.prefix.len() as i32,
            );
        }
        let mut remaining = end - info.rest;
        let mut buffer = [0; 1024];
        while remaining > 0 {
            let length = remaining.min(buffer.len() as u64) as usize;
            input.read_exact(&mut buffer[..length])?;
            unsafe {
                pdf_add_stream(stream, buffer.as_ptr().cast(), length as i32);
            }
            remaining -= length as u64;
        }
        Ok(())
    };
    match {
        let mut copy = copy;
        copy()
    } {
        Ok(()) => 0,
        Err(_) => {
            pdf_warning!("JPEG: Reading JPEG data failed.");
            -1
        }
    }
}
#[unsafe(export_name = "__pdf_impl_jpeg_get_bbox")]
pub unsafe extern "C-unwind" fn jpeg_get_bbox(
    handle: rust_input_handle_t,
    width: *mut u32,
    height: *mut u32,
    xdensity: *mut f64,
    ydensity: *mut f64,
) -> i32 {
    let Some(mut info) = scan_file(handle) else {
        return -1;
    };
    unsafe {
        *width = info.width.into();
        *height = info.height.into();
        jpeg_get_density(&mut info, xdensity, ydensity);
    }
    0
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
