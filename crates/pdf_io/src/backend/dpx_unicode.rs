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
#![allow(non_snake_case)]
use crate::unicode;

// The compatibility API requires start/end to delimit one live allocation.
unsafe fn with_input<T>(start: *const u8, end: *const u8, read: impl FnOnce(&[u8]) -> T) -> T {
    if start >= end {
        read(&[])
    } else {
        read(unsafe { core::slice::from_raw_parts(start, end.offset_from(start) as usize) })
    }
}

#[unsafe(export_name = "__pdf_impl_UC_is_valid")]
pub extern "C-unwind" fn UC_is_valid(ucv: i32) -> bool {
    unicode::is_valid(ucv)
}

#[unsafe(export_name = "__pdf_impl_UC_UTF16BE_is_valid_string")]
pub unsafe extern "C-unwind" fn UC_UTF16BE_is_valid_string(
    p: *const u8,
    endptr: *const u8,
) -> bool {
    unsafe { with_input(p, endptr, unicode::is_valid_utf16be) }
}

#[unsafe(export_name = "__pdf_impl_UC_UTF8_is_valid_string")]
pub unsafe extern "C-unwind" fn UC_UTF8_is_valid_string(p: *const u8, endptr: *const u8) -> bool {
    unsafe { with_input(p, endptr, unicode::is_valid_utf8) }
}

#[unsafe(export_name = "__pdf_impl_UC_UTF16BE_decode_char")]
pub unsafe extern "C-unwind" fn UC_UTF16BE_decode_char(
    pp: *mut *const u8,
    endptr: *const u8,
) -> i32 {
    match unsafe { with_input(*pp, endptr, unicode::decode_utf16be) } {
        Ok((value, consumed)) => {
            unsafe {
                *pp = (*pp).add(consumed);
            }
            value
        }
        Err(_) => -1,
    }
}

#[unsafe(export_name = "__pdf_impl_UC_UTF8_decode_char")]
pub unsafe extern "C-unwind" fn UC_UTF8_decode_char(pp: *mut *const u8, endptr: *const u8) -> i32 {
    match unsafe { with_input(*pp, endptr, unicode::decode_utf8) } {
        Ok((value, consumed)) => {
            unsafe {
                *pp = (*pp).add(consumed);
            }
            value
        }
        Err(_) => -1,
    }
}

#[unsafe(export_name = "__pdf_impl_UC_UTF16BE_encode_char")]
pub unsafe extern "C-unwind" fn UC_UTF16BE_encode_char(
    ucv: i32,
    pp: *mut *mut u8,
    endptr: *mut u8,
) -> usize {
    unsafe { encode(ucv, pp, endptr, unicode::encode_utf16be) }
}

#[unsafe(export_name = "__pdf_impl_UC_UTF8_encode_char")]
pub unsafe extern "C-unwind" fn UC_UTF8_encode_char(
    ucv: i32,
    pp: *mut *mut u8,
    endptr: *mut u8,
) -> usize {
    if pp.is_null() || unsafe { (*pp).is_null() } || endptr.is_null() {
        crate::backend::runtime::raise(c"assertion failed: pp && *pp && endptr".to_owned());
    }
    unsafe { encode(ucv, pp, endptr, unicode::encode_utf8) }
}

// The caller supplies a writable start/end range; pp advances on success.
unsafe fn encode(
    value: i32,
    pp: *mut *mut u8,
    end: *mut u8,
    encode: fn(i32, &mut [u8]) -> Result<usize, unicode::EncodeError>,
) -> usize {
    let start = unsafe { *pp };
    let available = if start >= end {
        0
    } else {
        unsafe { end.offset_from(start) as usize }
    };
    let mut bytes = [0; 4];
    match encode(value, &mut bytes[..available.min(4)]) {
        Ok(written) => {
            unsafe {
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), start, written);
                *pp = start.add(written);
            }
            written
        }
        Err(_) => 0,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_UC_is_valid(mut ucv: i32, _result: *mut bool) -> i32 {
    match crate::backend::runtime::boundary(|| UC_is_valid(ucv)) {
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
pub unsafe extern "C" fn __pdf_entry_UC_UTF16BE_is_valid_string(
    mut p: *const ::core::ffi::c_uchar,
    mut endptr: *const ::core::ffi::c_uchar,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { UC_UTF16BE_is_valid_string(p, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_UC_UTF8_is_valid_string(
    mut p: *const ::core::ffi::c_uchar,
    mut endptr: *const ::core::ffi::c_uchar,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { UC_UTF8_is_valid_string(p, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_UC_UTF16BE_decode_char(
    mut pp: *mut *const ::core::ffi::c_uchar,
    mut endptr: *const ::core::ffi::c_uchar,
    _result: *mut i32,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { UC_UTF16BE_decode_char(pp, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_UC_UTF16BE_encode_char(
    mut ucv: i32,
    mut pp: *mut *mut ::core::ffi::c_uchar,
    mut endptr: *mut ::core::ffi::c_uchar,
    _result: *mut usize,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { UC_UTF16BE_encode_char(ucv, pp, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_UC_UTF8_decode_char(
    mut pp: *mut *const ::core::ffi::c_uchar,
    mut endptr: *const ::core::ffi::c_uchar,
    _result: *mut i32,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { UC_UTF8_decode_char(pp, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_UC_UTF8_encode_char(
    mut ucv: i32,
    mut pp: *mut *mut ::core::ffi::c_uchar,
    mut endptr: *mut ::core::ffi::c_uchar,
    _result: *mut usize,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { UC_UTF8_encode_char(ucv, pp, endptr) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
