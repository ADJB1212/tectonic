/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Matthias Franz,
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
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __nitems: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn rewind(_: *mut FILE);
    #[link_name = "__pdf_impl_get_unsigned_byte"]
    fn get_unsigned_byte(_: *mut FILE) -> ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_get_unsigned_pair"]
    fn get_unsigned_pair(_: *mut FILE) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_get_unsigned_quad"]
    fn get_unsigned_quad(_: *mut FILE) -> uint32_t;
    #[link_name = "__pdf_impl_seek_relative"]
    fn seek_relative(file: *mut FILE, pos: int32_t);
    #[link_name = "__pdf_impl_file_size"]
    fn file_size(file: *mut FILE) -> int32_t;
    static mut work_buffer: [::core::ffi::c_char; 0];
    #[link_name = "__pdf_impl_pdf_get_version_major"]
    fn pdf_get_version_major() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_version_minor"]
    fn pdf_get_version_minor() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
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
    #[link_name = "__pdf_impl_pdf_check_version"]
    fn pdf_check_version(
        major: ::core::ffi::c_int,
        minor: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_ximage_init_image_info"]
    fn pdf_ximage_init_image_info(info: *mut ximage_info);
    #[link_name = "__pdf_impl_pdf_ximage_set_image"]
    fn pdf_ximage_set_image(
        ximage: *mut pdf_ximage,
        info: *mut ::core::ffi::c_void,
        resource: *mut pdf_obj,
    );
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const WORK_BUFFER_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const JP2_BOX_JP__: ::core::ffi::c_int = 0x6a502020 as ::core::ffi::c_int;
pub const JP2_BOX_FTYP: ::core::ffi::c_int = 0x66747970 as ::core::ffi::c_int;
pub const JP2_BOX_JP2H: ::core::ffi::c_uint = 1785737832 as ::core::ffi::c_uint;
pub const JP2_BOX_IHDR: ::core::ffi::c_uint = 1768449138 as ::core::ffi::c_uint;
pub const JP2_BOX_BPCC: ::core::ffi::c_uint = 1651532643 as ::core::ffi::c_uint;
pub const JP2_BOX_COLR: ::core::ffi::c_uint = 1668246642 as ::core::ffi::c_uint;
pub const JP2_BOX_PCLR: ::core::ffi::c_uint = 1885564018 as ::core::ffi::c_uint;
pub const JP2_BOX_CMAP: ::core::ffi::c_uint = 1668112752 as ::core::ffi::c_uint;
pub const JP2_BOX_CDEF: ::core::ffi::c_uint = 1667523942 as ::core::ffi::c_uint;
pub const JP2_BOX_RES_: ::core::ffi::c_uint = 1919251232 as ::core::ffi::c_uint;
pub const JP2_BOX_RESC: ::core::ffi::c_uint = 1919251299 as ::core::ffi::c_uint;
pub const JP2_BOX_RESD: ::core::ffi::c_uint = 1919251300 as ::core::ffi::c_uint;
pub const JP2_BOX_JP2C: ::core::ffi::c_uint = 1785737827 as ::core::ffi::c_uint;
pub const JPX_BOX_LBL_: ::core::ffi::c_uint = 1818389536 as ::core::ffi::c_uint;
unsafe extern "C-unwind" fn scan_file(
    info: *mut ximage_info,
    smask: *mut core::ffi::c_int,
    fp: *mut FILE,
) -> core::ffi::c_int {
    let mut input = unsafe { crate::compat::input::BorrowedFile::from_raw(fp.cast()) };
    match crate::jp2::scan(&mut input, |message| {
        pdf_warning!("{}", message);
    }) {
        Ok(metadata) => unsafe {
            (*info).width = metadata.width;
            (*info).height = metadata.height;
            (*info).num_components = metadata.components;
            (*info).xdensity = metadata.x_density;
            (*info).ydensity = metadata.y_density;
            *smask = i32::from(metadata.soft_mask);
            0
        },
        Err(error) => {
            pdf_warning!("{}", error.message.to_string_lossy());
            -1
        }
    }
}

#[unsafe(export_name = "__pdf_impl_check_for_jp2")]
pub unsafe extern "C-unwind" fn check_for_jp2(fp: *mut FILE) -> core::ffi::c_int {
    if fp.is_null() {
        return 0;
    }
    let mut input = unsafe { crate::compat::input::BorrowedFile::from_raw(fp.cast()) };
    i32::from(crate::jp2::probe(&mut input).unwrap_or(false))
}
#[unsafe(export_name = "__pdf_impl_jp2_include_image")]
pub unsafe extern "C-unwind" fn jp2_include_image(
    mut ximage: *mut pdf_ximage,
    mut fp: *mut FILE,
) -> ::core::ffi::c_int {
    let mut smask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
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
    if pdf_check_version(1 as ::core::ffi::c_int, 5 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"JPEG 2000 support requires PDF version >= 1.5  (Current setting %d.%d)\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            pdf_get_version_major(),
            pdf_get_version_minor(),
        );
        return -(1 as ::core::ffi::c_int);
    }
    pdf_ximage_init_image_info(&raw mut info);
    stream_dict = ::core::ptr::null_mut::<pdf_obj>();
    stream = stream_dict;
    rewind(fp);
    if scan_file(&raw mut info, &raw mut smask, fp) < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"JPEG2000: Reading JPEG 2000 file failed.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    stream = pdf_new_stream(0 as ::core::ffi::c_int);
    stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Filter\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"JPXDecode\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if smask != 0 {
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"SMaskInData\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(1 as ::core::ffi::c_int as ::core::ffi::c_double),
        );
    }
    use std::io::{Read, Seek};
    let mut input = crate::compat::input::BorrowedFile::from_raw(fp.cast());
    if input.rewind().is_err() {
        super::dpx_pdfobj::pdf_release_obj(stream.cast());
        return -1;
    }
    let mut buffer = [0; 4096];
    loop {
        match input.read(&mut buffer) {
            Ok(0) => break,
            Ok(length) => pdf_add_stream(stream, buffer.as_ptr().cast(), length as i32),
            Err(_) => {
                super::dpx_pdfobj::pdf_release_obj(stream.cast());
                pdf_warning!("JPEG2000: Reading JPEG 2000 file failed.");
                return -1;
            }
        }
    }
    pdf_ximage_set_image(ximage, &raw mut info as *mut ::core::ffi::c_void, stream);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_jp2_get_bbox")]
pub unsafe extern "C-unwind" fn jp2_get_bbox(
    mut fp: *mut FILE,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut xdensity: *mut ::core::ffi::c_double,
    mut ydensity: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    let mut smask: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
    pdf_ximage_init_image_info(&raw mut info);
    rewind(fp);
    r = scan_file(&raw mut info, &raw mut smask, fp);
    *width = info.width;
    *height = info.height;
    *xdensity = info.xdensity;
    *ydensity = info.ydensity;
    return r;
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
pub struct pdf_ximage_ {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_check_for_jp2(
    mut fp: *mut FILE,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { check_for_jp2(fp) }) {
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
pub unsafe extern "C" fn __pdf_entry_jp2_include_image(
    mut ximage: *mut pdf_ximage,
    mut fp: *mut FILE,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { jp2_include_image(ximage, fp) }) {
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
pub unsafe extern "C" fn __pdf_entry_jp2_get_bbox(
    mut fp: *mut FILE,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut xdensity: *mut ::core::ffi::c_double,
    mut ydensity: *mut ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        jp2_get_bbox(fp, width, height, xdensity, ydensity)
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
