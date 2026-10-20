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
    static mut dpx_conf: _dpx_conf;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    fn tt_tfm_next_diagnostic(kind: *mut ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn tt_tfm_open(
        name: *const ::core::ffi::c_char, must_exist: ::core::ffi::c_int, verbose: ::core::ffi::c_int, error: *mut *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn tt_tfm_close_all();
    fn tt_tfm_reset();
    fn tt_tfm_get_fw_width(font_id: ::core::ffi::c_int, ch: int32_t, error: *mut *const ::core::ffi::c_char) -> int32_t;
    fn tt_tfm_get_fw_height(font_id: ::core::ffi::c_int, ch: int32_t, error: *mut *const ::core::ffi::c_char) -> int32_t;
    fn tt_tfm_get_fw_depth(font_id: ::core::ffi::c_int, ch: int32_t, error: *mut *const ::core::ffi::c_char) -> int32_t;
    fn tt_tfm_string_width(
        font_id: ::core::ffi::c_int, text: *const ::core::ffi::c_uchar, length: ::core::ffi::c_uint, error: *mut *const ::core::ffi::c_char,
    ) -> int32_t;
    fn tt_tfm_get_width(font_id: ::core::ffi::c_int, ch: int32_t, error: *mut *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn tt_tfm_get_design_size(font_id: ::core::ffi::c_int, error: *mut *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn tt_tfm_is_jfm(font_id: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn tt_tfm_exists(name: *const ::core::ffi::c_char) -> bool;
}
pub type int32_t = i32;
pub type fixword = int32_t;
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
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
unsafe extern "C-unwind" fn tfm_flush(mut error: *const ::core::ffi::c_char) {
    let mut text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut kind: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        text = tt_tfm_next_diagnostic(&raw mut kind);
        if text.is_null() {
            break;
        }
        if kind == 0 as ::core::ffi::c_int {
            pdf_message!(b"%s\0" as *const u8 as *const ::core::ffi::c_char, text);
        } else {
            pdf_warning!(b"%s\0" as *const u8 as *const ::core::ffi::c_char, text);
        }
    }
    if !error.is_null() {
        pdf_abort!(b"%s\0" as *const u8 as *const ::core::ffi::c_char, error);
    }
}
#[unsafe(export_name = "__pdf_impl_tfm_reset_global_state")]
pub unsafe extern "C-unwind" fn tfm_reset_global_state() {
    tt_tfm_reset();
}
#[unsafe(export_name = "__pdf_impl_tfm_open")]
pub unsafe extern "C-unwind" fn tfm_open(mut tfm_name: *const ::core::ffi::c_char, mut must_exist: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut error: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut id: ::core::ffi::c_int = tt_tfm_open(tfm_name, must_exist, dpx_conf.verbose_level, &raw mut error);
    tfm_flush(error);
    return id;
}
#[unsafe(export_name = "__pdf_impl_tfm_close_all")]
pub unsafe extern "C-unwind" fn tfm_close_all() {
    tt_tfm_close_all();
}
#[unsafe(export_name = "__pdf_impl_tfm_get_fw_width")]
pub unsafe extern "C-unwind" fn tfm_get_fw_width(mut font_id: ::core::ffi::c_int, mut ch: int32_t) -> fixword {
    let mut error: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut width: fixword = tt_tfm_get_fw_width(font_id, ch, &raw mut error) as fixword;
    tfm_flush(error);
    return width;
}
#[unsafe(export_name = "__pdf_impl_tfm_get_fw_height")]
pub unsafe extern "C-unwind" fn tfm_get_fw_height(mut font_id: ::core::ffi::c_int, mut ch: int32_t) -> fixword {
    let mut error: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut height: fixword = tt_tfm_get_fw_height(font_id, ch, &raw mut error) as fixword;
    tfm_flush(error);
    return height;
}
#[unsafe(export_name = "__pdf_impl_tfm_get_fw_depth")]
pub unsafe extern "C-unwind" fn tfm_get_fw_depth(mut font_id: ::core::ffi::c_int, mut ch: int32_t) -> fixword {
    let mut error: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut depth: fixword = tt_tfm_get_fw_depth(font_id, ch, &raw mut error) as fixword;
    tfm_flush(error);
    return depth;
}
#[unsafe(export_name = "__pdf_impl_tfm_get_width")]
pub unsafe extern "C-unwind" fn tfm_get_width(mut font_id: ::core::ffi::c_int, mut ch: int32_t) -> ::core::ffi::c_double {
    let mut error: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut width: ::core::ffi::c_double = tt_tfm_get_width(font_id, ch, &raw mut error);
    tfm_flush(error);
    return width;
}
#[unsafe(export_name = "__pdf_impl_tfm_string_width")]
pub unsafe extern "C-unwind" fn tfm_string_width(
    mut font_id: ::core::ffi::c_int, mut s: *const ::core::ffi::c_uchar, mut len: ::core::ffi::c_uint,
) -> fixword {
    let mut error: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut width: fixword = tt_tfm_string_width(font_id, s, len, &raw mut error) as fixword;
    tfm_flush(error);
    return width;
}
#[unsafe(export_name = "__pdf_impl_tfm_get_design_size")]
pub unsafe extern "C-unwind" fn tfm_get_design_size(mut font_id: ::core::ffi::c_int) -> ::core::ffi::c_double {
    let mut error: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut design: ::core::ffi::c_double = tt_tfm_get_design_size(font_id, &raw mut error);
    tfm_flush(error);
    return design;
}
#[unsafe(export_name = "__pdf_impl_tfm_is_jfm")]
pub unsafe extern "C-unwind" fn tfm_is_jfm(mut font_id: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return tt_tfm_is_jfm(font_id);
}
#[unsafe(export_name = "__pdf_impl_tfm_exists")]
pub unsafe extern "C-unwind" fn tfm_exists(mut tfm_name: *const ::core::ffi::c_char) -> bool {
    return tt_tfm_exists(tfm_name);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tfm_reset_global_state() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tfm_reset_global_state() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tfm_open(
    mut tfm_name: *const ::core::ffi::c_char, mut must_exist: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tfm_open(tfm_name, must_exist) }) {
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
pub unsafe extern "C" fn __pdf_entry_tfm_close_all() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tfm_close_all() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tfm_get_fw_width(mut font_id: ::core::ffi::c_int, mut ch: int32_t, _result: *mut fixword) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tfm_get_fw_width(font_id, ch) }) {
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
pub unsafe extern "C" fn __pdf_entry_tfm_get_fw_height(mut font_id: ::core::ffi::c_int, mut ch: int32_t, _result: *mut fixword) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tfm_get_fw_height(font_id, ch) }) {
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
pub unsafe extern "C" fn __pdf_entry_tfm_get_fw_depth(mut font_id: ::core::ffi::c_int, mut ch: int32_t, _result: *mut fixword) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tfm_get_fw_depth(font_id, ch) }) {
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
pub unsafe extern "C" fn __pdf_entry_tfm_get_width(mut font_id: ::core::ffi::c_int, mut ch: int32_t, _result: *mut ::core::ffi::c_double) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tfm_get_width(font_id, ch) }) {
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
pub unsafe extern "C" fn __pdf_entry_tfm_string_width(
    mut font_id: ::core::ffi::c_int, mut s: *const ::core::ffi::c_uchar, mut len: ::core::ffi::c_uint, _result: *mut fixword,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tfm_string_width(font_id, s, len) }) {
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
pub unsafe extern "C" fn __pdf_entry_tfm_get_design_size(mut font_id: ::core::ffi::c_int, _result: *mut ::core::ffi::c_double) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tfm_get_design_size(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_tfm_is_jfm(mut font_id: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tfm_is_jfm(font_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_tfm_exists(mut tfm_name: *const ::core::ffi::c_char, _result: *mut bool) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tfm_exists(tfm_name) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
