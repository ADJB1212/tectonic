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
unsafe extern "C-unwind" {
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_getc(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn fgetc(_: *mut FILE) -> ::core::ffi::c_int;
}
pub type __int64_t = i64;
pub type __darwin_off_t = __int64_t;
pub type int32_t = i32;
pub type uint32_t = u32;
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
    pub _read: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_char, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub _seek: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ::core::ffi::c_int) -> ::core::ffi::c_int>,
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
pub type fixword = int32_t;
fn read_unsigned(input: &mut impl std::io::Read, width: usize) -> u32 {
    let mut bytes = [0; 4];
    if input.read_exact(&mut bytes[4 - width..]).is_err() {
        pdf_abort!("File ended prematurely\n");
    }
    u32::from_be_bytes(bytes)
}

fn skip(input: &mut impl std::io::Read, n: u32) {
    let mut buffer = [0; 4096];
    let mut remaining = n as usize;
    while remaining != 0 {
        let length = remaining.min(buffer.len());
        if input.read_exact(&mut buffer[..length]).is_err() {
            pdf_abort!("File ended prematurely\n");
        }
        remaining -= length;
    }
}

unsafe fn read_file_unsigned(file: *mut FILE, width: usize) -> u32 {
    read_unsigned(&mut unsafe { crate::compat::input::BorrowedFile::from_raw(file.cast()) }, width)
}

fn read_input_unsigned(handle: rust_input_handle_t, width: usize) -> u32 {
    read_unsigned(&mut crate::compat::input::DirectoryInput { handle }, width)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    struct Input {
        data: Cursor<Vec<u8>>,
        calls: usize,
        max_read: usize,
    }

    impl Read for Input {
        fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
            self.calls += 1;
            let length = bytes.len().min(self.max_read);
            self.data.read(&mut bytes[..length])
        }
    }

    #[test]
    fn numeric_reads_handle_short_reads_and_skips_use_bounded_blocks() {
        for width in 1..=4 {
            for max_read in [1, 2, 4096] {
                let mut input = Input {
                    data: Cursor::new(vec![0x12, 0x34, 0x56, 0x78]),
                    calls: 0,
                    max_read,
                };
                let expected = 0x12345678u32 >> (8 * (4 - width));
                assert_eq!(read_unsigned(&mut input, width), expected);
                assert_eq!(input.data.position(), width as u64);
                assert_eq!(input.calls, width.div_ceil(max_read));
            }
        }
        let mut input = Input {
            data: Cursor::new(vec![0; 16385]),
            calls: 0,
            max_read: 4096,
        };
        skip(&mut input, 16385);
        assert_eq!(input.calls, 5);
        assert_eq!(input.data.position(), 16385);
        assert!(crate::backend::runtime::boundary(|| skip(&mut input, 1)).is_err());
        for width in 1..=4 {
            assert!(crate::backend::runtime::boundary(|| read_unsigned(&mut input, width)).is_err());
        }
    }
}

#[unsafe(export_name = "__pdf_impl_get_unsigned_byte")]
pub unsafe extern "C-unwind" fn get_unsigned_byte(file: *mut FILE) -> u8 {
    let byte = fgetc(file);
    if byte < 0 {
        pdf_abort!(b"File ended prematurely\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    byte as u8
}
#[unsafe(export_name = "__pdf_impl_skip_bytes")]
pub unsafe extern "C-unwind" fn skip_bytes(n: u32, file: *mut FILE) {
    skip(&mut crate::compat::input::BorrowedFile::from_raw(file.cast()), n);
}
#[unsafe(export_name = "__pdf_impl_get_signed_byte")]
pub unsafe extern "C-unwind" fn get_signed_byte(file: *mut FILE) -> i8 {
    get_unsigned_byte(file) as i8
}
#[unsafe(export_name = "__pdf_impl_get_unsigned_pair")]
pub unsafe extern "C-unwind" fn get_unsigned_pair(file: *mut FILE) -> u16 {
    read_file_unsigned(file, 2) as u16
}
#[unsafe(export_name = "__pdf_impl_sget_unsigned_pair")]
pub unsafe extern "C-unwind" fn sget_unsigned_pair(mut s: *mut ::core::ffi::c_uchar) -> ::core::ffi::c_ushort {
    // The legacy API guarantees two readable bytes but supplies no length.
    let bytes = unsafe { core::slice::from_raw_parts(s, 2) };
    crate::byte_cursor::ByteCursor::new(bytes).read_u16().unwrap()
}
#[unsafe(export_name = "__pdf_impl_get_signed_pair")]
pub unsafe extern "C-unwind" fn get_signed_pair(file: *mut FILE) -> i16 {
    get_unsigned_pair(file) as i16
}
#[unsafe(export_name = "__pdf_impl_get_unsigned_triple")]
pub unsafe extern "C-unwind" fn get_unsigned_triple(file: *mut FILE) -> u32 {
    read_file_unsigned(file, 3)
}
#[unsafe(export_name = "__pdf_impl_get_signed_triple")]
pub unsafe extern "C-unwind" fn get_signed_triple(file: *mut FILE) -> i32 {
    ((get_unsigned_triple(file) << 8) as i32) >> 8
}
#[unsafe(export_name = "__pdf_impl_get_signed_quad")]
pub unsafe extern "C-unwind" fn get_signed_quad(file: *mut FILE) -> i32 {
    get_unsigned_quad(file) as i32
}
#[unsafe(export_name = "__pdf_impl_get_unsigned_quad")]
pub unsafe extern "C-unwind" fn get_unsigned_quad(file: *mut FILE) -> u32 {
    read_file_unsigned(file, 4)
}
#[unsafe(export_name = "__pdf_impl_get_unsigned_num")]
pub unsafe extern "C-unwind" fn get_unsigned_num(file: *mut FILE, num: u8) -> u32 {
    let width = match num {
        1..=3 => usize::from(num) + 1,
        _ => 1,
    };
    read_file_unsigned(file, width)
}
#[unsafe(export_name = "__pdf_impl_get_positive_quad")]
pub unsafe extern "C-unwind" fn get_positive_quad(file: *mut FILE, type_name: *const ::core::ffi::c_char, name: *const ::core::ffi::c_char) -> u32 {
    let value = get_signed_quad(file);
    if value < 0 {
        pdf_abort!(
            b"Bad %s: negative %s: %d\0" as *const u8 as *const ::core::ffi::c_char,
            type_name,
            name,
            value,
        );
    }
    value as u32
}
#[unsafe(export_name = "__pdf_impl_sqxfw")]
pub extern "C-unwind" fn sqxfw(mut sq: int32_t, mut fw: fixword) -> int32_t {
    crate::numbers::scaled_fixword(sq, fw)
}
#[unsafe(export_name = "__pdf_impl_tt_skip_bytes")]
pub unsafe extern "C-unwind" fn tt_skip_bytes(n: u32, handle: rust_input_handle_t) {
    skip(&mut crate::compat::input::DirectoryInput { handle }, n);
}
#[unsafe(export_name = "__pdf_impl_tt_get_unsigned_byte")]
pub unsafe extern "C-unwind" fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> u8 {
    let byte = ttstub_input_getc(handle);
    if byte < 0 {
        pdf_abort!(b"File ended prematurely\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    byte as u8
}
#[unsafe(export_name = "__pdf_impl_tt_get_signed_byte")]
pub unsafe extern "C-unwind" fn tt_get_signed_byte(handle: rust_input_handle_t) -> i8 {
    tt_get_unsigned_byte(handle) as i8
}
#[unsafe(export_name = "__pdf_impl_tt_get_unsigned_pair")]
pub unsafe extern "C-unwind" fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> u16 {
    read_input_unsigned(handle, 2) as u16
}
#[unsafe(export_name = "__pdf_impl_tt_get_signed_pair")]
pub unsafe extern "C-unwind" fn tt_get_signed_pair(handle: rust_input_handle_t) -> i16 {
    tt_get_unsigned_pair(handle) as i16
}
#[unsafe(export_name = "__pdf_impl_tt_get_unsigned_triple")]
pub unsafe extern "C-unwind" fn tt_get_unsigned_triple(handle: rust_input_handle_t) -> u32 {
    read_input_unsigned(handle, 3)
}
#[unsafe(export_name = "__pdf_impl_tt_get_unsigned_quad")]
pub unsafe extern "C-unwind" fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> u32 {
    read_input_unsigned(handle, 4)
}
#[unsafe(export_name = "__pdf_impl_tt_get_signed_quad")]
pub unsafe extern "C-unwind" fn tt_get_signed_quad(handle: rust_input_handle_t) -> i32 {
    tt_get_unsigned_quad(handle) as i32
}
#[unsafe(export_name = "__pdf_impl_tt_get_unsigned_num")]
pub unsafe extern "C-unwind" fn tt_get_unsigned_num(handle: rust_input_handle_t, num: u8) -> u32 {
    let width = match num {
        1..=3 => usize::from(num) + 1,
        _ => 1,
    };
    read_input_unsigned(handle, width)
}
#[unsafe(export_name = "__pdf_impl_tt_get_positive_quad")]
pub unsafe extern "C-unwind" fn tt_get_positive_quad(
    handle: rust_input_handle_t, type_name: *const ::core::ffi::c_char, name: *const ::core::ffi::c_char,
) -> u32 {
    let value = tt_get_signed_quad(handle);
    if value < 0 {
        pdf_abort!(
            b"Bad %s: negative %s: %d\0" as *const u8 as *const ::core::ffi::c_char,
            type_name,
            name,
            value,
        );
    }
    value as u32
}
#[repr(C)]
pub struct __sFILEX {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_get_unsigned_byte(mut file: *mut FILE, _result: *mut ::core::ffi::c_uchar) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_unsigned_byte(file) }) {
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
pub unsafe extern "C" fn __pdf_entry_skip_bytes(mut n: ::core::ffi::c_uint, mut file: *mut FILE) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { skip_bytes(n, file) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_get_signed_byte(mut file: *mut FILE, _result: *mut ::core::ffi::c_schar) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_signed_byte(file) }) {
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
pub unsafe extern "C" fn __pdf_entry_get_unsigned_pair(mut file: *mut FILE, _result: *mut ::core::ffi::c_ushort) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_unsigned_pair(file) }) {
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
pub unsafe extern "C" fn __pdf_entry_sget_unsigned_pair(mut s: *mut ::core::ffi::c_uchar, _result: *mut ::core::ffi::c_ushort) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sget_unsigned_pair(s) }) {
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
pub unsafe extern "C" fn __pdf_entry_get_signed_pair(mut file: *mut FILE, _result: *mut ::core::ffi::c_short) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_signed_pair(file) }) {
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
pub unsafe extern "C" fn __pdf_entry_get_unsigned_triple(mut file: *mut FILE, _result: *mut ::core::ffi::c_uint) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_unsigned_triple(file) }) {
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
pub unsafe extern "C" fn __pdf_entry_get_signed_triple(mut file: *mut FILE, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_signed_triple(file) }) {
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
pub unsafe extern "C" fn __pdf_entry_get_signed_quad(mut file: *mut FILE, _result: *mut int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_signed_quad(file) }) {
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
pub unsafe extern "C" fn __pdf_entry_get_unsigned_quad(mut file: *mut FILE, _result: *mut uint32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_unsigned_quad(file) }) {
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
pub unsafe extern "C" fn __pdf_entry_get_unsigned_num(mut file: *mut FILE, mut num: ::core::ffi::c_uchar, _result: *mut uint32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_unsigned_num(file, num) }) {
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
pub unsafe extern "C" fn __pdf_entry_get_positive_quad(
    mut file: *mut FILE, mut type_0: *const ::core::ffi::c_char, mut name: *const ::core::ffi::c_char, _result: *mut uint32_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { get_positive_quad(file, type_0, name) }) {
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
pub unsafe extern "C" fn __pdf_entry_sqxfw(mut sq: int32_t, mut fw: fixword, _result: *mut int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| sqxfw(sq, fw)) {
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
pub unsafe extern "C" fn __pdf_entry_tt_skip_bytes(mut n: ::core::ffi::c_uint, mut handle: rust_input_handle_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_skip_bytes(n, handle) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_get_unsigned_byte(mut handle: rust_input_handle_t, _result: *mut ::core::ffi::c_uchar) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_unsigned_byte(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_signed_byte(mut handle: rust_input_handle_t, _result: *mut ::core::ffi::c_schar) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_signed_byte(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_unsigned_pair(mut handle: rust_input_handle_t, _result: *mut ::core::ffi::c_ushort) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_unsigned_pair(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_signed_pair(mut handle: rust_input_handle_t, _result: *mut ::core::ffi::c_short) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_signed_pair(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_unsigned_triple(mut handle: rust_input_handle_t, _result: *mut ::core::ffi::c_uint) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_unsigned_triple(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_unsigned_quad(mut handle: rust_input_handle_t, _result: *mut uint32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_unsigned_quad(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_signed_quad(mut handle: rust_input_handle_t, _result: *mut int32_t) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_signed_quad(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_unsigned_num(
    mut handle: rust_input_handle_t, mut num: ::core::ffi::c_uchar, _result: *mut uint32_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_unsigned_num(handle, num) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_positive_quad(
    mut handle: rust_input_handle_t, mut type_0: *const ::core::ffi::c_char, mut name: *const ::core::ffi::c_char, _result: *mut uint32_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_positive_quad(handle, type_0, name) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
