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
    #[link_name = "__pdf_impl_pdf_ximage_init_image_info"]
    fn pdf_ximage_init_image_info(info: *mut ximage_info);
    #[link_name = "__pdf_impl_pdf_ximage_set_image"]
    fn pdf_ximage_set_image(
        ximage: *mut pdf_ximage,
        info: *mut ::core::ffi::c_void,
        resource: *mut pdf_obj,
    );
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
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
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
pub unsafe extern "C-unwind" fn check_for_bmp(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
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
unsafe extern "C-unwind" fn get_density(
    mut xdensity: *mut ::core::ffi::c_double,
    mut ydensity: *mut ::core::ffi::c_double,
    mut hdr: *mut hdr_info,
) {
    if dpx_conf.compat_mode as ::core::ffi::c_uint
        == dpx_mode_compat_mode as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *ydensity = 72.0f64 / 100.0f64;
        *xdensity = *ydensity;
    } else if (*hdr).x_pix_per_meter > 0 as ::core::ffi::c_uint
        && (*hdr).y_pix_per_meter > 0 as ::core::ffi::c_uint
    {
        *xdensity = 72.0f64 / ((*hdr).x_pix_per_meter as ::core::ffi::c_double * 0.0254f64);
        *ydensity = 72.0f64 / ((*hdr).y_pix_per_meter as ::core::ffi::c_double * 0.0254f64);
    } else {
        *xdensity = 1.0f64;
        *ydensity = 1.0f64;
    };
}
#[unsafe(export_name = "__pdf_impl_bmp_get_bbox")]
pub unsafe extern "C-unwind" fn bmp_get_bbox(
    mut handle: rust_input_handle_t,
    mut width: *mut ::core::ffi::c_uint,
    mut height: *mut ::core::ffi::c_uint,
    mut xdensity: *mut ::core::ffi::c_double,
    mut ydensity: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    let mut hdr: hdr_info = hdr_info {
        offset: 0 as ::core::ffi::c_uint,
        hsize: 0 as ::core::ffi::c_uint,
        width: 0 as ::core::ffi::c_uint,
        height: 0 as ::core::ffi::c_int,
        compression: 0 as ::core::ffi::c_int,
        bit_count: 0 as ::core::ffi::c_ushort,
        psize: 0 as ::core::ffi::c_int,
        x_pix_per_meter: 0 as ::core::ffi::c_uint,
        y_pix_per_meter: 0 as ::core::ffi::c_uint,
    };
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    r = read_header(handle, &raw mut hdr);
    *width = hdr.width;
    *height = (if hdr.height < 0 as ::core::ffi::c_int {
        -hdr.height
    } else {
        hdr.height
    }) as ::core::ffi::c_uint;
    get_density(xdensity, ydensity, &raw mut hdr);
    return r;
}
#[unsafe(export_name = "__pdf_impl_bmp_include_image")]
pub unsafe extern "C-unwind" fn bmp_include_image(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut colorspace: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
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
    let mut hdr: hdr_info = hdr_info {
        offset: 0 as ::core::ffi::c_uint,
        hsize: 0 as ::core::ffi::c_uint,
        width: 0 as ::core::ffi::c_uint,
        height: 0 as ::core::ffi::c_int,
        compression: 0 as ::core::ffi::c_int,
        bit_count: 0 as ::core::ffi::c_ushort,
        psize: 0 as ::core::ffi::c_int,
        x_pix_per_meter: 0 as ::core::ffi::c_uint,
        y_pix_per_meter: 0 as ::core::ffi::c_uint,
    };
    let mut num_palette: ::core::ffi::c_int = 0;
    let mut flip: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    pdf_ximage_init_image_info(&raw mut info);
    colorspace = ::core::ptr::null_mut::<pdf_obj>();
    stream_dict = colorspace;
    stream = stream_dict;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    if read_header(handle, &raw mut hdr) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    get_density(&raw mut info.xdensity, &raw mut info.ydensity, &raw mut hdr);
    info.width = hdr.width as ::core::ffi::c_int;
    info.height = hdr.height;
    if info.height < 0 as ::core::ffi::c_int {
        info.height = -info.height;
        flip = 0 as ::core::ffi::c_int;
    } else {
        flip = 1 as ::core::ffi::c_int;
    }
    if (hdr.bit_count as ::core::ffi::c_int) < 24 as ::core::ffi::c_int {
        if hdr.bit_count as ::core::ffi::c_int != 1 as ::core::ffi::c_int
            && hdr.bit_count as ::core::ffi::c_int != 4 as ::core::ffi::c_int
            && hdr.bit_count as ::core::ffi::c_int != 8 as ::core::ffi::c_int
        {
            pdf_warning!(
                b"Unsupported palette size: %hu\0" as *const u8 as *const ::core::ffi::c_char,
                hdr.bit_count as ::core::ffi::c_int,
            );
            return -(1 as ::core::ffi::c_int);
        }
        num_palette =
            hdr.offset
                .wrapping_sub(hdr.hsize)
                .wrapping_sub(DIB_FILE_HEADER_SIZE as ::core::ffi::c_uint)
                .wrapping_div(hdr.psize as ::core::ffi::c_uint) as ::core::ffi::c_int;
        info.bits_per_component = hdr.bit_count as ::core::ffi::c_int;
        info.num_components = 1 as ::core::ffi::c_int;
    } else if hdr.bit_count as ::core::ffi::c_int == 24 as ::core::ffi::c_int {
        num_palette = 1 as ::core::ffi::c_int;
        info.bits_per_component = 8 as ::core::ffi::c_int;
        info.num_components = 3 as ::core::ffi::c_int;
    } else {
        pdf_warning!(
            b"Unkown/Unsupported BMP bitCount value: %hu\0" as *const u8
                as *const ::core::ffi::c_char,
            hdr.bit_count as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if info.width == 0 as ::core::ffi::c_int
        || info.height == 0 as ::core::ffi::c_int
        || num_palette < 1 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Invalid BMP file: width=%u, height=%d, #palette=%d\0" as *const u8
                as *const ::core::ffi::c_char,
            info.width,
            info.height,
            num_palette,
        );
        return -(1 as ::core::ffi::c_int);
    }
    stream = pdf_new_stream(STREAM_COMPRESS);
    stream_dict = pdf_stream_dict(stream);
    if (hdr.bit_count as ::core::ffi::c_int) < 24 as ::core::ffi::c_int {
        let mut lookup: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut palette: *mut ::core::ffi::c_uchar =
            ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut bgrq: [::core::ffi::c_uchar; 4] = [0; 4];
        palette = new(
            ((num_palette * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_uchar;
        i = 0 as ::core::ffi::c_int;
        while i < num_palette {
            if ttstub_input_read(
                handle,
                &raw mut bgrq as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
                hdr.psize as size_t,
            ) != hdr.psize as ssize_t
            {
                pdf_warning!(
                    b"Reading file failed...\0" as *const u8 as *const ::core::ffi::c_char,
                );
                free(palette as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            *palette.offset((3 as ::core::ffi::c_int * i) as isize) =
                bgrq[2 as ::core::ffi::c_int as usize];
            *palette.offset((3 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize) =
                bgrq[1 as ::core::ffi::c_int as usize];
            *palette.offset((3 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as isize) =
                bgrq[0 as ::core::ffi::c_int as usize];
            i += 1;
        }
        lookup = pdf_new_string(
            palette as *const ::core::ffi::c_void,
            (num_palette * 3 as ::core::ffi::c_int) as size_t,
        );
        free(palette as *mut ::core::ffi::c_void);
        colorspace = pdf_new_array();
        pdf_add_array(
            colorspace,
            pdf_new_name(b"Indexed\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_array(
            colorspace,
            pdf_new_name(b"DeviceRGB\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_array(
            colorspace,
            pdf_new_number((num_palette - 1 as ::core::ffi::c_int) as ::core::ffi::c_double),
        );
        pdf_add_array(colorspace, lookup);
    } else {
        colorspace = pdf_new_name(b"DeviceRGB\0" as *const u8 as *const ::core::ffi::c_char);
    }
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"ColorSpace\0" as *const u8 as *const ::core::ffi::c_char),
        colorspace,
    );
    let mut rowbytes: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut stream_data_ptr: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    rowbytes = (info.width * hdr.bit_count as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
        / 8 as ::core::ffi::c_int;
    crate::backend::runtime::input_seek(handle, hdr.offset as ssize_t, SEEK_SET);
    if hdr.compression == DIB_COMPRESS_NONE {
        let mut dib_rowbytes: ::core::ffi::c_int = 0;
        let mut padding: ::core::ffi::c_int = 0;
        padding = if rowbytes % 4 as ::core::ffi::c_int != 0 {
            4 as ::core::ffi::c_int - rowbytes % 4 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        dib_rowbytes = rowbytes + padding;
        stream_data_ptr = new(((rowbytes * info.height + padding) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        n = 0 as ::core::ffi::c_int;
        while n < info.height {
            p = stream_data_ptr.offset((n * rowbytes) as isize);
            if ttstub_input_read(
                handle,
                p as *mut ::core::ffi::c_char,
                dib_rowbytes as size_t,
            ) != dib_rowbytes as ssize_t
            {
                pdf_warning!(
                    b"Reading BMP raster data failed...\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                pdf_release_obj(stream);
                free(stream_data_ptr as *mut ::core::ffi::c_void);
                return -(1 as ::core::ffi::c_int);
            }
            n += 1;
        }
    } else if hdr.compression == DIB_COMPRESS_RLE8 {
        stream_data_ptr = new(((rowbytes * info.height) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        if read_raster_rle8(stream_data_ptr, info.width, info.height, handle)
            < 0 as ::core::ffi::c_int
        {
            pdf_warning!(
                b"Reading BMP raster data failed...\0" as *const u8 as *const ::core::ffi::c_char,
            );
            pdf_release_obj(stream);
            free(stream_data_ptr as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
    } else if hdr.compression == DIB_COMPRESS_RLE4 {
        stream_data_ptr = new(((rowbytes * info.height) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        if read_raster_rle4(stream_data_ptr, info.width, info.height, handle)
            < 0 as ::core::ffi::c_int
        {
            pdf_warning!(
                b"Reading BMP raster data failed...\0" as *const u8 as *const ::core::ffi::c_char,
            );
            pdf_release_obj(stream);
            free(stream_data_ptr as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        pdf_warning!(
            b"Unknown/Unsupported compression type for BMP image: %d\0" as *const u8
                as *const ::core::ffi::c_char,
            hdr.compression,
        );
        pdf_release_obj(stream);
        return -(1 as ::core::ffi::c_int);
    }
    if hdr.bit_count as ::core::ffi::c_int == 24 as ::core::ffi::c_int {
        n = 0 as ::core::ffi::c_int;
        while n < info.width * info.height * 3 as ::core::ffi::c_int {
            let mut g: ::core::ffi::c_uchar = 0;
            g = *stream_data_ptr.offset(n as isize);
            *stream_data_ptr.offset(n as isize) =
                *stream_data_ptr.offset((n + 2 as ::core::ffi::c_int) as isize);
            *stream_data_ptr.offset((n + 2 as ::core::ffi::c_int) as isize) = g;
            n += 3 as ::core::ffi::c_int;
        }
    }
    if flip != 0 {
        n = info.height - 1 as ::core::ffi::c_int;
        while n >= 0 as ::core::ffi::c_int {
            p = stream_data_ptr.offset((n * rowbytes) as isize);
            pdf_add_stream(stream, p as *const ::core::ffi::c_void, rowbytes);
            n -= 1;
        }
    } else {
        pdf_add_stream(
            stream,
            stream_data_ptr as *const ::core::ffi::c_void,
            rowbytes * info.height,
        );
    }
    free(stream_data_ptr as *mut ::core::ffi::c_void);
    if hdr.bit_count as ::core::ffi::c_int >= 24 as ::core::ffi::c_int
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
unsafe extern "C-unwind" fn read_header(
    mut handle: rust_input_handle_t,
    mut hdr: *mut hdr_info,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 142] = [0; 142];
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    p = &raw mut buf as *mut ::core::ffi::c_uchar;
    if ttstub_input_read(
        handle,
        &raw mut buf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        (DIB_FILE_HEADER_SIZE + 4 as ::core::ffi::c_int) as size_t,
    ) != (DIB_FILE_HEADER_SIZE + 4 as ::core::ffi::c_int) as ssize_t
    {
        pdf_warning!(
            b"Could not read BMP file header...\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'B' as i32
        || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'M' as i32
    {
        pdf_warning!(
            b"File not starting with 'B' 'M'... Not a BMP file?\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    p = p.offset(2 as ::core::ffi::c_int as isize);
    p = p.offset(4 as ::core::ffi::c_int as isize);
    if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
        + ((*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int)
        + ((*p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 24 as ::core::ffi::c_int)
        != 0 as ::core::ffi::c_int
    {
        pdf_warning!(b"Not a BMP file???\0" as *const u8 as *const ::core::ffi::c_char);
        return -(1 as ::core::ffi::c_int);
    }
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*hdr).offset = (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
        + ((*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int)
        + ((*p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 24 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (*hdr).hsize = (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int)
        + ((*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int)
        + ((*p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 24 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
    p = p.offset(4 as ::core::ffi::c_int as isize);
    if ttstub_input_read(
        handle,
        p as *mut ::core::ffi::c_char,
        (*hdr).hsize.wrapping_sub(4 as ::core::ffi::c_uint) as size_t,
    ) != (*hdr).hsize.wrapping_sub(4 as ::core::ffi::c_uint) as ssize_t
    {
        pdf_warning!(
            b"Could not read BMP file header...\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    match (*hdr).hsize {
        12 => {
            (*hdr).width = (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
            p = p.offset(2 as ::core::ffi::c_int as isize);
            (*hdr).height = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int);
            p = p.offset(2 as ::core::ffi::c_int as isize);
            (*hdr).x_pix_per_meter = 0 as ::core::ffi::c_uint;
            (*hdr).y_pix_per_meter = 0 as ::core::ffi::c_uint;
            if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int)
                != 1 as ::core::ffi::c_int
            {
                pdf_warning!(
                    b"Unknown bcPlanes value in BMP COREHEADER.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            p = p.offset(2 as ::core::ffi::c_int as isize);
            (*hdr).bit_count = (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int))
                as ::core::ffi::c_ushort;
            p = p.offset(2 as ::core::ffi::c_int as isize);
            (*hdr).compression = DIB_COMPRESS_NONE;
            (*hdr).psize = 3 as ::core::ffi::c_int;
        }
        40 | 64 | 108 | 124 => {
            (*hdr).width = (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int)
                + ((*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 16 as ::core::ffi::c_int)
                + ((*p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 24 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
            p = p.offset(4 as ::core::ffi::c_int as isize);
            (*hdr).height = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int)
                + ((*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 16 as ::core::ffi::c_int)
                + ((*p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 24 as ::core::ffi::c_int);
            p = p.offset(4 as ::core::ffi::c_int as isize);
            if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int)
                != 1 as ::core::ffi::c_int
            {
                pdf_warning!(
                    b"Unknown biPlanes value in BMP INFOHEADER.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            p = p.offset(2 as ::core::ffi::c_int as isize);
            (*hdr).bit_count = (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int))
                as ::core::ffi::c_ushort;
            p = p.offset(2 as ::core::ffi::c_int as isize);
            (*hdr).compression = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int)
                + ((*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 16 as ::core::ffi::c_int)
                + ((*p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    << 24 as ::core::ffi::c_int);
            p = p.offset(4 as ::core::ffi::c_int as isize);
            p = p.offset(4 as ::core::ffi::c_int as isize);
            (*hdr).x_pix_per_meter =
                (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int)
                    + ((*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 16 as ::core::ffi::c_int)
                    + ((*p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 24 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
            p = p.offset(4 as ::core::ffi::c_int as isize);
            (*hdr).y_pix_per_meter =
                (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + ((*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int)
                    + ((*p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 16 as ::core::ffi::c_int)
                    + ((*p.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        << 24 as ::core::ffi::c_int)) as ::core::ffi::c_uint;
            p = p.offset(4 as ::core::ffi::c_int as isize);
            (*hdr).psize = 4 as ::core::ffi::c_int;
        }
        _ => {
            pdf_warning!(b"Unknown BMP header type.\0" as *const u8 as *const ::core::ffi::c_char,);
            return -(1 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn read_raster_rle8(
    mut data_ptr: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut b0: ::core::ffi::c_uchar = 0;
    let mut b1: ::core::ffi::c_uchar = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    let mut rowbytes: ::core::ffi::c_int = 0;
    let mut eol: ::core::ffi::c_int = 0;
    let mut eoi: ::core::ffi::c_int = 0;
    p = data_ptr;
    rowbytes = width;
    memset(
        data_ptr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (rowbytes * height) as size_t,
    );
    v = 0 as ::core::ffi::c_int;
    eoi = 0 as ::core::ffi::c_int;
    while v < height && eoi == 0 {
        h = 0 as ::core::ffi::c_int;
        eol = 0 as ::core::ffi::c_int;
        while h < width && eol == 0 {
            b0 = tt_get_unsigned_byte(handle);
            b1 = tt_get_unsigned_byte(handle);
            count += 2 as ::core::ffi::c_int;
            p = data_ptr.offset((v * rowbytes) as isize).offset(h as isize);
            if b0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                match b1 as ::core::ffi::c_int {
                    0 => {
                        eol = 1 as ::core::ffi::c_int;
                    }
                    1 => {
                        eoi = 1 as ::core::ffi::c_int;
                    }
                    2 => {
                        h += tt_get_unsigned_byte(handle) as ::core::ffi::c_int;
                        v += tt_get_unsigned_byte(handle) as ::core::ffi::c_int;
                        count += 2 as ::core::ffi::c_int;
                    }
                    _ => {
                        h += b1 as ::core::ffi::c_int;
                        if h > width {
                            pdf_warning!(
                                b"RLE decode failed...\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        if ttstub_input_read(handle, p as *mut ::core::ffi::c_char, b1 as size_t)
                            != b1 as ssize_t
                        {
                            return -(1 as ::core::ffi::c_int);
                        }
                        count += b1 as ::core::ffi::c_int;
                        if b1 as ::core::ffi::c_int % 2 as ::core::ffi::c_int != 0 {
                            tt_get_unsigned_byte(handle);
                            count += 1;
                        }
                    }
                }
            } else {
                h += b0 as ::core::ffi::c_int;
                if h > width {
                    pdf_warning!(
                        b"RLE decode failed...\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                memset(
                    p as *mut ::core::ffi::c_void,
                    b1 as ::core::ffi::c_int,
                    b0 as size_t,
                );
            }
        }
        if eol == 0 && eoi == 0 {
            b0 = tt_get_unsigned_byte(handle);
            b1 = tt_get_unsigned_byte(handle);
            if b0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                pdf_warning!(b"RLE decode failed...\0" as *const u8 as *const ::core::ffi::c_char,);
                return -(1 as ::core::ffi::c_int);
            } else if b1 as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int {
                eoi = 1 as ::core::ffi::c_int;
            } else if b1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                pdf_warning!(b"RLE decode failed...\0" as *const u8 as *const ::core::ffi::c_char,);
                return -(1 as ::core::ffi::c_int);
            }
        }
        v += 1;
    }
    return count;
}
unsafe extern "C-unwind" fn read_raster_rle4(
    mut data_ptr: *mut ::core::ffi::c_uchar,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut b0: ::core::ffi::c_uchar = 0;
    let mut b1: ::core::ffi::c_uchar = 0;
    let mut b: ::core::ffi::c_uchar = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut v: ::core::ffi::c_int = 0;
    let mut rowbytes: ::core::ffi::c_int = 0;
    let mut eol: ::core::ffi::c_int = 0;
    let mut eoi: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut nbytes: ::core::ffi::c_int = 0;
    p = data_ptr;
    rowbytes = (width + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    memset(
        data_ptr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (rowbytes * height) as size_t,
    );
    v = 0 as ::core::ffi::c_int;
    eoi = 0 as ::core::ffi::c_int;
    while v < height && eoi == 0 {
        h = 0 as ::core::ffi::c_int;
        eol = 0 as ::core::ffi::c_int;
        while h < width && eol == 0 {
            b0 = tt_get_unsigned_byte(handle);
            b1 = tt_get_unsigned_byte(handle);
            count += 2 as ::core::ffi::c_int;
            p = data_ptr
                .offset((v * rowbytes) as isize)
                .offset((h / 2 as ::core::ffi::c_int) as isize);
            if b0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                match b1 as ::core::ffi::c_int {
                    0 => {
                        eol = 1 as ::core::ffi::c_int;
                    }
                    1 => {
                        eoi = 1 as ::core::ffi::c_int;
                    }
                    2 => {
                        h += tt_get_unsigned_byte(handle) as ::core::ffi::c_int;
                        v += tt_get_unsigned_byte(handle) as ::core::ffi::c_int;
                        count += 2 as ::core::ffi::c_int;
                    }
                    _ => {
                        if h + b1 as ::core::ffi::c_int > width {
                            pdf_warning!(
                                b"RLE decode failed...\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                        nbytes = (b1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            / 2 as ::core::ffi::c_int;
                        if h % 2 as ::core::ffi::c_int != 0 {
                            i = 0 as ::core::ffi::c_int;
                            while i < nbytes {
                                b = tt_get_unsigned_byte(handle);
                                let fresh0 = p;
                                p = p.offset(1);
                                *fresh0 = (*fresh0 as ::core::ffi::c_int
                                    | b as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                                        & 0xf as ::core::ffi::c_int)
                                    as ::core::ffi::c_uchar;
                                *p = ((b as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uchar;
                                i += 1;
                            }
                        } else if ttstub_input_read(
                            handle,
                            p as *mut ::core::ffi::c_char,
                            nbytes as size_t,
                        ) != nbytes as ssize_t
                        {
                            return -(1 as ::core::ffi::c_int);
                        }
                        h += b1 as ::core::ffi::c_int;
                        count += nbytes;
                        if nbytes % 2 as ::core::ffi::c_int != 0 {
                            tt_get_unsigned_byte(handle);
                            count += 1;
                        }
                    }
                }
            } else {
                if h + b0 as ::core::ffi::c_int > width {
                    pdf_warning!(
                        b"RLE decode failed...\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                if h % 2 as ::core::ffi::c_int != 0 {
                    let fresh1 = p;
                    p = p.offset(1);
                    *fresh1 = (b1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        & 0xf as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    b1 = ((b1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        | b1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                            & 0xf as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                    b0 = b0.wrapping_sub(1);
                    h += 1;
                }
                nbytes =
                    (b0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
                memset(
                    p as *mut ::core::ffi::c_void,
                    b1 as ::core::ffi::c_int,
                    nbytes as size_t,
                );
                h += b0 as ::core::ffi::c_int;
                if h % 2 as ::core::ffi::c_int != 0 {
                    let ref mut fresh2 = *p.offset((nbytes - 1 as ::core::ffi::c_int) as isize);
                    *fresh2 = (*fresh2 as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int)
                        as ::core::ffi::c_uchar;
                }
            }
        }
        if eol == 0 && eoi == 0 {
            b0 = tt_get_unsigned_byte(handle);
            b1 = tt_get_unsigned_byte(handle);
            if b0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"No EOL/EOI marker. RLE decode failed...\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            } else if b1 as ::core::ffi::c_int == 0x1 as ::core::ffi::c_int {
                eoi = 1 as ::core::ffi::c_int;
            } else if b1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"No EOL/EOI marker. RLE decode failed...\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
        }
        v += 1;
    }
    return count;
}
unsafe extern "C" {
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
pub unsafe extern "C" fn __pdf_entry_check_for_bmp(
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
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
    mut handle: rust_input_handle_t,
    mut width: *mut ::core::ffi::c_uint,
    mut height: *mut ::core::ffi::c_uint,
    mut xdensity: *mut ::core::ffi::c_double,
    mut ydensity: *mut ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        bmp_get_bbox(handle, width, height, xdensity, ydensity)
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
pub unsafe extern "C" fn __pdf_entry_bmp_include_image(
    mut ximage: *mut pdf_ximage,
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_int,
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
