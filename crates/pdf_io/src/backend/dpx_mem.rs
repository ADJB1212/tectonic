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
#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn, static_mut_refs, dangerous_implicit_autorefs)]
use crate::compat::allocation::{free, malloc, realloc};
unsafe extern "C-unwind" {
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[unsafe(export_name = "__pdf_impl_new")]
pub unsafe extern "C-unwind" fn new(mut size: uint32_t) -> *mut ::core::ffi::c_void {
    let mut result: *mut ::core::ffi::c_void = malloc(size as size_t);
    if result.is_null() {
        pdf_abort!(b"Out of memory - asked for %u bytes\n\0" as *const u8 as *const ::core::ffi::c_char, size,);
    }
    return result;
}
#[unsafe(export_name = "__pdf_impl_renew")]
pub unsafe extern "C-unwind" fn renew(mut mem: *mut ::core::ffi::c_void, mut size: uint32_t) -> *mut ::core::ffi::c_void {
    if size != 0 {
        let mut result: *mut ::core::ffi::c_void = realloc(mem, size as size_t);
        if result.is_null() {
            pdf_abort!(b"Out of memory - asked for %u bytes\n\0" as *const u8 as *const ::core::ffi::c_char, size,);
        }
        return result;
    } else {
        free(mem);
        return NULL;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_new(mut size: uint32_t, _result: *mut *mut ::core::ffi::c_void) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { new(size) }) {
        Ok(value) => {
            unsafe {
                crate::compat::allocation::release(value);
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_renew(mut mem: *mut ::core::ffi::c_void, mut size: uint32_t, _result: *mut *mut ::core::ffi::c_void) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { renew(mem, size) }) {
        Ok(value) => {
            unsafe {
                crate::compat::allocation::release(value);
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
