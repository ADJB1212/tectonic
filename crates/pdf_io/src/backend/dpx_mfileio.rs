/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_getc(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn ttstub_input_ungetc(
        handle: rust_input_handle_t,
        ch: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn fgetc(_: *mut FILE) -> ::core::ffi::c_int;
    fn fseek(_: *mut FILE, _: ::core::ffi::c_long, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ftell(_: *mut FILE) -> ::core::ffi::c_long;
    fn rewind(_: *mut FILE);
    fn ungetc(_: ::core::ffi::c_int, _: *mut FILE) -> ::core::ffi::c_int;
}
pub type __int64_t = i64;
pub type __darwin_off_t = __int64_t;
pub type int32_t = i32;
pub type uintptr_t = usize;
pub type rust_input_handle_t = uintptr_t;
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_seek_relative")]
pub unsafe extern "C-unwind" fn seek_relative(file: *mut FILE, pos: int32_t) {
    let mut input = unsafe { crate::compat::input::BorrowedFile::from_raw(file.cast()) };
    crate::backend::runtime::legacy_result(crate::input::seek_relative(&mut input, pos));
}
#[unsafe(export_name = "__pdf_impl_file_size")]
pub unsafe extern "C-unwind" fn file_size(file: *mut FILE) -> int32_t {
    let mut input = unsafe { crate::compat::input::BorrowedFile::from_raw(file.cast()) };
    crate::backend::runtime::legacy_result(crate::input::file_size(&mut input))
}
#[unsafe(export_name = "__pdf_impl_mfgets")]
pub unsafe extern "C-unwind" fn mfgets(
    mut buffer: *mut ::core::ffi::c_char,
    mut length: ::core::ffi::c_int,
    mut file: *mut FILE,
) -> *mut ::core::ffi::c_char {
    if length <= 0 {
        return core::ptr::null_mut();
    }
    // The C API requires length writable bytes, including the terminator.
    let output = unsafe {
        core::slice::from_raw_parts_mut(
            buffer.cast::<core::mem::MaybeUninit<u8>>(),
            length as usize,
        )
    };
    crate::line::read_line(
        output,
        || {
            let byte = unsafe { fgetc(file) };
            (byte >= 0).then_some(byte as u8)
        },
        |byte| unsafe {
            ungetc(byte.into(), file);
        },
    )
    .map_or(core::ptr::null_mut(), |_| buffer)
}
#[unsafe(no_mangle)]
pub static mut work_buffer: [::core::ffi::c_char; 1024] = [0; 1024];
#[unsafe(export_name = "__pdf_impl_tt_mfgets")]
pub unsafe extern "C-unwind" fn tt_mfgets(
    mut buffer: *mut ::core::ffi::c_char,
    mut length: ::core::ffi::c_int,
    mut file: rust_input_handle_t,
) -> *mut ::core::ffi::c_char {
    if length <= 0 {
        return core::ptr::null_mut();
    }
    // The C API requires length writable bytes, including the terminator.
    let output = unsafe {
        core::slice::from_raw_parts_mut(
            buffer.cast::<core::mem::MaybeUninit<u8>>(),
            length as usize,
        )
    };
    crate::line::read_line(
        output,
        || {
            let byte = unsafe { ttstub_input_getc(file) };
            (byte >= 0).then_some(byte as u8)
        },
        |byte| unsafe {
            ttstub_input_ungetc(file, byte.into());
        },
    )
    .map_or(core::ptr::null_mut(), |_| buffer)
}
#[repr(C)]
pub struct __sFILEX {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_seek_relative(mut file: *mut FILE, mut pos: int32_t) -> i32 {
    match crate::backend::runtime::result_boundary(|| unsafe {
        let mut input = crate::compat::input::BorrowedFile::from_raw(file.cast());
        crate::input::seek_relative(&mut input, pos)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_file_size(mut file: *mut FILE, _result: *mut int32_t) -> i32 {
    match crate::backend::runtime::result_boundary(|| unsafe {
        let mut input = crate::compat::input::BorrowedFile::from_raw(file.cast());
        crate::input::file_size(&mut input)
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
pub unsafe extern "C" fn __pdf_entry_mfgets(
    mut buffer: *mut ::core::ffi::c_char,
    mut length: ::core::ffi::c_int,
    mut file: *mut FILE,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { mfgets(buffer, length, file) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_mfgets(
    mut buffer: *mut ::core::ffi::c_char,
    mut length: ::core::ffi::c_int,
    mut file: rust_input_handle_t,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_mfgets(buffer, length, file) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
