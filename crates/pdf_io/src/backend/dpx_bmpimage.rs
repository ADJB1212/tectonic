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
#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn, static_mut_refs, dangerous_implicit_autorefs)]
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: ::core::ffi::c_int) -> size_t;
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut ::core::ffi::c_char, len: size_t) -> ssize_t;
    #[link_name = "__pdf_impl_tt_get_unsigned_byte"]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
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
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_stream"]
    fn pdf_new_stream(flags: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_stream"]
    fn pdf_add_stream(stream: *mut pdf_obj, stream_data_ptr: *const ::core::ffi::c_void, stream_data_len: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_stream_dict"]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_stream_set_predictor"]
    fn pdf_stream_set_predictor(
        stream: *mut pdf_obj, predictor: ::core::ffi::c_int, columns: int32_t, bpc: ::core::ffi::c_int, colors: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_ximage_init_image_info"]
    fn pdf_ximage_init_image_info(info: *mut ximage_info);
    #[link_name = "__pdf_impl_pdf_ximage_set_image"]
    fn pdf_ximage_set_image(ximage: *mut pdf_ximage, info: *mut ::core::ffi::c_void, resource: *mut pdf_obj);
    static mut dpx_conf: _dpx_conf;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdr_info {
    pub offset: ::core::ffi::c_uint,
    pub hsize: ::core::ffi::c_uint,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_int,
    pub compression: ::core::ffi::c_int,
    pub bit_count: ::core::ffi::c_ushort,
    pub psize: ::core::ffi::c_int,
    pub x_pix_per_meter: ::core::ffi::c_uint,
    pub y_pix_per_meter: ::core::ffi::c_uint,
}
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_normal_mode: dpx_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub keep_cache: ::core::ffi::c_int,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
pub const STREAM_COMPRESS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const DIB_FILE_HEADER_SIZE: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const DIB_CORE_HEADER_SIZE: ::core::ffi::c_uint = 12 as ::core::ffi::c_uint;
pub const DIB_INFO_HEADER_SIZE: ::core::ffi::c_uint = 40 as ::core::ffi::c_uint;
pub const DIB_INFO_HEADER_SIZE2: ::core::ffi::c_uint = 64 as ::core::ffi::c_uint;
pub const DIB_INFO_HEADER_SIZE4: ::core::ffi::c_uint = 108 as ::core::ffi::c_uint;
pub const DIB_INFO_HEADER_SIZE5: ::core::ffi::c_uint = 124 as ::core::ffi::c_uint;
pub const DIB_COMPRESS_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DIB_COMPRESS_RLE8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DIB_COMPRESS_RLE4: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_check_for_bmp")]
pub unsafe extern "C-unwind" fn check_for_bmp(mut handle: rust_input_handle_t) -> ::core::ffi::c_int {
    let mut sigbytes: [::core::ffi::c_uchar; 2] = [0; 2];
    if handle == INVALID_HANDLE {
        return 0 as ::core::ffi::c_int;
    }
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    if ttstub_input_read(
        handle,
        &raw mut sigbytes as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 2]>() as size_t,
    ) as usize
        != ::core::mem::size_of::<[::core::ffi::c_uchar; 2]>() as usize
        || sigbytes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 'B' as i32
        || sigbytes[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 'M' as i32
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
fn density(header: &crate::bmp::Header) -> (f64, f64) {
    if unsafe { dpx_conf.compat_mode } == dpx_mode_compat_mode {
        (72.0 / 100.0, 72.0 / 100.0)
    } else if header.x_density > 0 && header.y_density > 0 {
        (72.0 / (header.x_density as f64 * 0.0254), 72.0 / (header.y_density as f64 * 0.0254))
    } else {
        (1.0, 1.0)
    }
}

#[unsafe(export_name = "__pdf_impl_bmp_get_bbox")]
pub unsafe extern "C-unwind" fn bmp_get_bbox(
    handle: rust_input_handle_t, width: *mut u32, height: *mut u32, xdensity: *mut f64, ydensity: *mut f64,
) -> core::ffi::c_int {
    crate::backend::runtime::input_seek(handle, 0, SEEK_SET);
    let mut input = crate::compat::input::ProbeInput { handle };
    match crate::bmp::Header::read(&mut input) {
        Ok(header) => {
            let (x, y) = density(&header);
            unsafe {
                *width = header.width;
                *height = header.height.unsigned_abs();
                *xdensity = x;
                *ydensity = y;
            }
            0
        }
        Err(error) => {
            pdf_warning!("{}", error.message.to_string_lossy());
            -1
        }
    }
}

#[unsafe(export_name = "__pdf_impl_bmp_include_image")]
pub unsafe extern "C-unwind" fn bmp_include_image(ximage: *mut pdf_ximage, handle: rust_input_handle_t) -> core::ffi::c_int {
    crate::backend::runtime::input_seek(handle, 0, SEEK_SET);
    let mut input = crate::compat::input::ProbeInput { handle };
    let raster = match crate::bmp::read_raster(&mut input) {
        Ok(raster) => raster,
        Err(error) => {
            pdf_warning!("{}", error.message.to_string_lossy());
            return -1;
        }
    };
    let header = &raster.header;
    let (xdensity, ydensity) = density(header);
    let mut info = ximage_info {
        flags: 0,
        width: header.width as i32,
        height: header.height.unsigned_abs() as i32,
        bits_per_component: if header.bits == 24 { 8 } else { header.bits as i32 },
        num_components: if header.bits == 24 { 3 } else { 1 },
        min_dpi: 0,
        xdensity,
        ydensity,
    };
    unsafe {
        let stream = pdf_new_stream(STREAM_COMPRESS);
        let dict = pdf_stream_dict(stream);
        let colorspace = if header.bits < 24 {
            let array = pdf_new_array();
            pdf_add_array(array, pdf_new_name(c"Indexed".as_ptr()));
            pdf_add_array(array, pdf_new_name(c"DeviceRGB".as_ptr()));
            pdf_add_array(array, pdf_new_number((raster.palette.len() / 3 - 1) as f64));
            pdf_add_array(array, pdf_new_string(raster.palette.as_ptr().cast(), raster.palette.len()));
            array
        } else {
            pdf_new_name(c"DeviceRGB".as_ptr())
        };
        pdf_add_dict(dict, pdf_new_name(c"ColorSpace".as_ptr()), colorspace);
        if header.height > 0 {
            for row in raster.bytes.chunks_exact(raster.row_bytes).rev() {
                pdf_add_stream(stream, row.as_ptr().cast(), row.len() as i32);
            }
        } else {
            pdf_add_stream(stream, raster.bytes.as_ptr().cast(), raster.bytes.len() as i32);
        }
        if header.bits >= 24 && info.height > 64 {
            pdf_stream_set_predictor(stream, 15, info.width, info.bits_per_component, info.num_components);
        }
        pdf_ximage_set_image(ximage, (&mut info as *mut ximage_info).cast(), stream);
    }
    0
}
unsafe extern "C" {
    fn memset(__b: *mut ::core::ffi::c_void, __c: ::core::ffi::c_int, __len: size_t) -> *mut ::core::ffi::c_void;
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
pub unsafe extern "C" fn __pdf_entry_check_for_bmp(mut handle: rust_input_handle_t, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { check_for_bmp(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_bmp_get_bbox(
    mut handle: rust_input_handle_t, mut width: *mut ::core::ffi::c_uint, mut height: *mut ::core::ffi::c_uint,
    mut xdensity: *mut ::core::ffi::c_double, mut ydensity: *mut ::core::ffi::c_double, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { bmp_get_bbox(handle, width, height, xdensity, ydensity) }) {
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
pub unsafe extern "C" fn __pdf_entry_bmp_include_image(
    mut ximage: *mut pdf_ximage, mut handle: rust_input_handle_t, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { bmp_include_image(ximage, handle) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
