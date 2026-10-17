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
pub type fixword = int32_t;
#[unsafe(export_name = "__pdf_impl_get_unsigned_byte")]
pub unsafe extern "C-unwind" fn get_unsigned_byte(mut file: *mut FILE) -> ::core::ffi::c_uchar {
    let mut ch: ::core::ffi::c_int = 0;
    ch = fgetc(file);
    if ch < 0 as ::core::ffi::c_int {
        pdf_abort!(b"File ended prematurely\n\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    return ch as ::core::ffi::c_uchar;
}
#[unsafe(export_name = "__pdf_impl_skip_bytes")]
pub unsafe extern "C-unwind" fn skip_bytes(mut n: ::core::ffi::c_uint, mut file: *mut FILE) {
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 > 0 as ::core::ffi::c_uint) {
            break;
        }
        get_unsigned_byte(file);
    }
}
#[unsafe(export_name = "__pdf_impl_get_signed_byte")]
pub unsafe extern "C-unwind" fn get_signed_byte(mut file: *mut FILE) -> ::core::ffi::c_schar {
    let mut byte: ::core::ffi::c_int = 0;
    byte = get_unsigned_byte(file) as ::core::ffi::c_int;
    if byte >= 0x80 as ::core::ffi::c_int {
        byte -= 0x100 as ::core::ffi::c_int;
    }
    return byte as ::core::ffi::c_schar;
}
#[unsafe(export_name = "__pdf_impl_get_unsigned_pair")]
pub unsafe extern "C-unwind" fn get_unsigned_pair(mut file: *mut FILE) -> ::core::ffi::c_ushort {
    let mut pair: ::core::ffi::c_ushort = get_unsigned_byte(file) as ::core::ffi::c_ushort;
    pair = ((pair as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | get_unsigned_byte(file) as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    return pair;
}
#[unsafe(export_name = "__pdf_impl_sget_unsigned_pair")]
pub unsafe extern "C-unwind" fn sget_unsigned_pair(
    mut s: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_ushort {
    // The legacy API guarantees two readable bytes but supplies no length.
    let bytes = unsafe { core::slice::from_raw_parts(s, 2) };
    crate::byte_cursor::ByteCursor::new(bytes)
        .read_u16()
        .unwrap()
}
#[unsafe(export_name = "__pdf_impl_get_signed_pair")]
pub unsafe extern "C-unwind" fn get_signed_pair(mut file: *mut FILE) -> ::core::ffi::c_short {
    let mut pair: ::core::ffi::c_short = get_signed_byte(file) as ::core::ffi::c_short;
    pair = ((pair as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | get_unsigned_byte(file) as ::core::ffi::c_int) as ::core::ffi::c_short;
    return pair;
}
#[unsafe(export_name = "__pdf_impl_get_unsigned_triple")]
pub unsafe extern "C-unwind" fn get_unsigned_triple(mut file: *mut FILE) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_int = 0;
    let mut triple: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        triple = triple << 8 as ::core::ffi::c_int | get_unsigned_byte(file) as ::core::ffi::c_uint;
        i += 1;
    }
    return triple;
}
#[unsafe(export_name = "__pdf_impl_get_signed_triple")]
pub unsafe extern "C-unwind" fn get_signed_triple(mut file: *mut FILE) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut triple: ::core::ffi::c_int = get_signed_byte(file) as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        triple = triple << 8 as ::core::ffi::c_int | get_unsigned_byte(file) as ::core::ffi::c_int;
        i += 1;
    }
    return triple;
}
#[unsafe(export_name = "__pdf_impl_get_signed_quad")]
pub unsafe extern "C-unwind" fn get_signed_quad(mut file: *mut FILE) -> int32_t {
    let mut i: ::core::ffi::c_int = 0;
    let mut quad: int32_t = get_signed_byte(file) as int32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        quad = quad << 8 as ::core::ffi::c_int | get_unsigned_byte(file) as int32_t;
        i += 1;
    }
    return quad;
}
#[unsafe(export_name = "__pdf_impl_get_unsigned_quad")]
pub unsafe extern "C-unwind" fn get_unsigned_quad(mut file: *mut FILE) -> uint32_t {
    let mut i: ::core::ffi::c_int = 0;
    let mut quad: uint32_t = 0 as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        quad = quad << 8 as ::core::ffi::c_int | get_unsigned_byte(file) as uint32_t;
        i += 1;
    }
    return quad;
}
#[unsafe(export_name = "__pdf_impl_get_unsigned_num")]
pub unsafe extern "C-unwind" fn get_unsigned_num(
    mut file: *mut FILE,
    mut num: ::core::ffi::c_uchar,
) -> uint32_t {
    let mut val: uint32_t = get_unsigned_byte(file) as uint32_t;
    let mut current_block_4: u64;
    match num as ::core::ffi::c_int {
        3 => {
            if val > 0x7f as uint32_t {
                val = val.wrapping_sub(0x100 as uint32_t);
            }
            val = val << 8 as ::core::ffi::c_int | get_unsigned_byte(file) as uint32_t;
            current_block_4 = 14892530444492967771;
        }
        2 => {
            current_block_4 = 14892530444492967771;
        }
        1 => {
            current_block_4 = 1659277425109653256;
        }
        _ => {
            current_block_4 = 15619007995458559411;
        }
    }
    match current_block_4 {
        14892530444492967771 => {
            val = val << 8 as ::core::ffi::c_int | get_unsigned_byte(file) as uint32_t;
            current_block_4 = 1659277425109653256;
        }
        _ => {}
    }
    match current_block_4 {
        1659277425109653256 => {
            val = val << 8 as ::core::ffi::c_int | get_unsigned_byte(file) as uint32_t;
        }
        _ => {}
    }
    return val;
}
#[unsafe(export_name = "__pdf_impl_get_positive_quad")]
pub unsafe extern "C-unwind" fn get_positive_quad(
    mut file: *mut FILE,
    mut type_0: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) -> uint32_t {
    let mut val: int32_t = get_signed_quad(file);
    if val < 0 as int32_t {
        pdf_abort!(
            b"Bad %s: negative %s: %d\0" as *const u8 as *const ::core::ffi::c_char,
            type_0,
            name,
            val,
        );
    }
    return val as uint32_t;
}
#[unsafe(export_name = "__pdf_impl_sqxfw")]
pub unsafe extern "C-unwind" fn sqxfw(mut sq: int32_t, mut fw: fixword) -> int32_t {
    crate::numbers::scaled_fixword(sq, fw)
}
#[unsafe(export_name = "__pdf_impl_tt_skip_bytes")]
pub unsafe extern "C-unwind" fn tt_skip_bytes(
    mut n: ::core::ffi::c_uint,
    mut handle: rust_input_handle_t,
) {
    loop {
        let fresh3 = n;
        n = n.wrapping_sub(1);
        if !(fresh3 > 0 as ::core::ffi::c_uint) {
            break;
        }
        tt_get_unsigned_byte(handle);
    }
}
#[unsafe(export_name = "__pdf_impl_tt_get_unsigned_byte")]
pub unsafe extern "C-unwind" fn tt_get_unsigned_byte(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_uchar {
    let mut ch: ::core::ffi::c_int = 0;
    ch = ttstub_input_getc(handle);
    if ch < 0 as ::core::ffi::c_int {
        pdf_abort!(b"File ended prematurely\n\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    return ch as ::core::ffi::c_uchar;
}
#[unsafe(export_name = "__pdf_impl_tt_get_signed_byte")]
pub unsafe extern "C-unwind" fn tt_get_signed_byte(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_schar {
    let mut byte: ::core::ffi::c_int = 0;
    byte = tt_get_unsigned_byte(handle) as ::core::ffi::c_int;
    if byte >= 0x80 as ::core::ffi::c_int {
        byte -= 0x100 as ::core::ffi::c_int;
    }
    return byte as ::core::ffi::c_schar;
}
#[unsafe(export_name = "__pdf_impl_tt_get_unsigned_pair")]
pub unsafe extern "C-unwind" fn tt_get_unsigned_pair(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_ushort {
    let mut pair: ::core::ffi::c_ushort = tt_get_unsigned_byte(handle) as ::core::ffi::c_ushort;
    pair = ((pair as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | tt_get_unsigned_byte(handle) as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    return pair;
}
#[unsafe(export_name = "__pdf_impl_tt_get_signed_pair")]
pub unsafe extern "C-unwind" fn tt_get_signed_pair(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_short {
    let mut pair: ::core::ffi::c_short = tt_get_signed_byte(handle) as ::core::ffi::c_short;
    pair = ((pair as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | tt_get_unsigned_byte(handle) as ::core::ffi::c_int) as ::core::ffi::c_short;
    return pair;
}
#[unsafe(export_name = "__pdf_impl_tt_get_unsigned_triple")]
pub unsafe extern "C-unwind" fn tt_get_unsigned_triple(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_int = 0;
    let mut triple: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        triple =
            triple << 8 as ::core::ffi::c_int | tt_get_unsigned_byte(handle) as ::core::ffi::c_uint;
        i += 1;
    }
    return triple;
}
#[unsafe(export_name = "__pdf_impl_tt_get_unsigned_quad")]
pub unsafe extern "C-unwind" fn tt_get_unsigned_quad(mut handle: rust_input_handle_t) -> uint32_t {
    let mut i: ::core::ffi::c_int = 0;
    let mut quad: uint32_t = 0 as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        quad = quad << 8 as ::core::ffi::c_int | tt_get_unsigned_byte(handle) as uint32_t;
        i += 1;
    }
    return quad;
}
#[unsafe(export_name = "__pdf_impl_tt_get_signed_quad")]
pub unsafe extern "C-unwind" fn tt_get_signed_quad(mut handle: rust_input_handle_t) -> int32_t {
    let mut i: ::core::ffi::c_int = 0;
    let mut quad: int32_t = tt_get_signed_byte(handle) as int32_t;
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        quad = quad << 8 as ::core::ffi::c_int | tt_get_unsigned_byte(handle) as int32_t;
        i += 1;
    }
    return quad;
}
#[unsafe(export_name = "__pdf_impl_tt_get_unsigned_num")]
pub unsafe extern "C-unwind" fn tt_get_unsigned_num(
    mut handle: rust_input_handle_t,
    mut num: ::core::ffi::c_uchar,
) -> uint32_t {
    let mut val: uint32_t = tt_get_unsigned_byte(handle) as uint32_t;
    let mut current_block_4: u64;
    match num as ::core::ffi::c_int {
        3 => {
            if val > 0x7f as uint32_t {
                val = val.wrapping_sub(0x100 as uint32_t);
            }
            val = val << 8 as ::core::ffi::c_int | tt_get_unsigned_byte(handle) as uint32_t;
            current_block_4 = 13783502858822981806;
        }
        2 => {
            current_block_4 = 13783502858822981806;
        }
        1 => {
            current_block_4 = 2939615421586548314;
        }
        _ => {
            current_block_4 = 15619007995458559411;
        }
    }
    match current_block_4 {
        13783502858822981806 => {
            val = val << 8 as ::core::ffi::c_int | tt_get_unsigned_byte(handle) as uint32_t;
            current_block_4 = 2939615421586548314;
        }
        _ => {}
    }
    match current_block_4 {
        2939615421586548314 => {
            val = val << 8 as ::core::ffi::c_int | tt_get_unsigned_byte(handle) as uint32_t;
        }
        _ => {}
    }
    return val;
}
#[unsafe(export_name = "__pdf_impl_tt_get_positive_quad")]
pub unsafe extern "C-unwind" fn tt_get_positive_quad(
    mut handle: rust_input_handle_t,
    mut type_0: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
) -> uint32_t {
    let mut val: int32_t = tt_get_signed_quad(handle);
    if val < 0 as int32_t {
        pdf_abort!(
            b"Bad %s: negative %s: %d\0" as *const u8 as *const ::core::ffi::c_char,
            type_0,
            name,
            val,
        );
    }
    return val as uint32_t;
}
unsafe extern "C" {}
#[repr(C)]
pub struct __sFILEX {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_get_unsigned_byte(
    mut file: *mut FILE,
    _result: *mut ::core::ffi::c_uchar,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_skip_bytes(
    mut n: ::core::ffi::c_uint,
    mut file: *mut FILE,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { skip_bytes(n, file) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_get_signed_byte(
    mut file: *mut FILE,
    _result: *mut ::core::ffi::c_schar,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_get_unsigned_pair(
    mut file: *mut FILE,
    _result: *mut ::core::ffi::c_ushort,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_sget_unsigned_pair(
    mut s: *mut ::core::ffi::c_uchar,
    _result: *mut ::core::ffi::c_ushort,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_get_signed_pair(
    mut file: *mut FILE,
    _result: *mut ::core::ffi::c_short,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_get_unsigned_triple(
    mut file: *mut FILE,
    _result: *mut ::core::ffi::c_uint,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_get_signed_triple(
    mut file: *mut FILE,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_get_signed_quad(
    mut file: *mut FILE,
    _result: *mut int32_t,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_get_unsigned_quad(
    mut file: *mut FILE,
    _result: *mut uint32_t,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_get_unsigned_num(
    mut file: *mut FILE,
    mut num: ::core::ffi::c_uchar,
    _result: *mut uint32_t,
) -> i32 {
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
    mut file: *mut FILE,
    mut type_0: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    _result: *mut uint32_t,
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
pub unsafe extern "C" fn __pdf_entry_sqxfw(
    mut sq: int32_t,
    mut fw: fixword,
    _result: *mut int32_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sqxfw(sq, fw) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_skip_bytes(
    mut n: ::core::ffi::c_uint,
    mut handle: rust_input_handle_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_skip_bytes(n, handle) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_get_unsigned_byte(
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_uchar,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_signed_byte(
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_schar,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_unsigned_pair(
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_ushort,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_signed_pair(
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_short,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_unsigned_triple(
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_uint,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_unsigned_quad(
    mut handle: rust_input_handle_t,
    _result: *mut uint32_t,
) -> i32 {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_signed_quad(
    mut handle: rust_input_handle_t,
    _result: *mut int32_t,
) -> i32 {
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
    mut handle: rust_input_handle_t,
    mut num: ::core::ffi::c_uchar,
    _result: *mut uint32_t,
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
    mut handle: rust_input_handle_t,
    mut type_0: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    _result: *mut uint32_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        tt_get_positive_quad(handle, type_0, name)
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
