/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.
   Copyright (C) 2007-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn mkstemp(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcat(
        __s1: *mut ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn xstrdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ttstub_input_open(
        path: *const ::core::ffi::c_char,
        format: ttbc_file_format,
        is_gz: ::core::ffi::c_int,
    ) -> rust_input_handle_t;
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
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn remove(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_tt_get_unsigned_quad"]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    static mut dpx_conf: _dpx_conf;
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
pub type rust_input_handle_t = uintptr_t;
pub type ttbc_file_format = ::core::ffi::c_uint;
pub const TTBC_FILE_FORMAT_VF: ttbc_file_format = 33;
pub const TTBC_FILE_FORMAT_TYPE1: ttbc_file_format = 32;
pub const TTBC_FILE_FORMAT_TRUE_TYPE: ttbc_file_format = 36;
pub const TTBC_FILE_FORMAT_TFM: ttbc_file_format = 3;
pub const TTBC_FILE_FORMAT_TEX_PS_HEADER: ttbc_file_format = 30;
pub const TTBC_FILE_FORMAT_TEX: ttbc_file_format = 26;
pub const TTBC_FILE_FORMAT_TECTONIC_PRIMARY: ttbc_file_format = 59;
pub const TTBC_FILE_FORMAT_SFD: ttbc_file_format = 46;
pub const TTBC_FILE_FORMAT_PROGRAM_DATA: ttbc_file_format = 39;
pub const TTBC_FILE_FORMAT_PK: ttbc_file_format = 1;
pub const TTBC_FILE_FORMAT_PICT: ttbc_file_format = 25;
pub const TTBC_FILE_FORMAT_OVF: ttbc_file_format = 23;
pub const TTBC_FILE_FORMAT_OPEN_TYPE: ttbc_file_format = 47;
pub const TTBC_FILE_FORMAT_OFM: ttbc_file_format = 20;
pub const TTBC_FILE_FORMAT_MISC_FONTS: ttbc_file_format = 41;
pub const TTBC_FILE_FORMAT_FONT_MAP: ttbc_file_format = 11;
pub const TTBC_FILE_FORMAT_FORMAT: ttbc_file_format = 10;
pub const TTBC_FILE_FORMAT_ENC: ttbc_file_format = 44;
pub const TTBC_FILE_FORMAT_CNF: ttbc_file_format = 8;
pub const TTBC_FILE_FORMAT_CMAP: ttbc_file_format = 45;
pub const TTBC_FILE_FORMAT_BST: ttbc_file_format = 7;
pub const TTBC_FILE_FORMAT_BIB: ttbc_file_format = 6;
pub const TTBC_FILE_FORMAT_AFM: ttbc_file_format = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub keep_cache: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed,
}
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
static mut _sbuf: [::core::ffi::c_char; 128] = [0; 128];
unsafe extern "C-unwind" fn check_stream_is_truetype(mut handle: rust_input_handle_t) -> bool {
    let mut n: ::core::ffi::c_int = 0;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    n = ttstub_input_read(
        handle,
        &raw mut _sbuf as *mut ::core::ffi::c_char,
        4 as size_t,
    ) as ::core::ffi::c_int;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    if n != 4 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if memcmp(
        &raw mut _sbuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        b"true\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0
        || memcmp(
            &raw mut _sbuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            b"\0\x01\0\0\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0
    {
        return true_0 != 0;
    }
    if memcmp(
        &raw mut _sbuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        b"ttcf\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C-unwind" fn check_stream_is_opentype(mut handle: rust_input_handle_t) -> bool {
    let mut n: ::core::ffi::c_int = 0;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    n = ttstub_input_read(
        handle,
        &raw mut _sbuf as *mut ::core::ffi::c_char,
        4 as size_t,
    ) as ::core::ffi::c_int;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    if n != 4 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if memcmp(
        &raw mut _sbuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        b"OTTO\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C-unwind" fn check_stream_is_type1(mut handle: rust_input_handle_t) -> bool {
    let mut p: *mut ::core::ffi::c_char = &raw mut _sbuf as *mut ::core::ffi::c_char;
    let mut n: ::core::ffi::c_int = 0;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    n = ttstub_input_read(handle, p, 21 as size_t) as ::core::ffi::c_int;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    if n != 21 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        != 0x80 as ::core::ffi::c_int as ::core::ffi::c_char as ::core::ffi::c_int
        || (*p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        || *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            > 3 as ::core::ffi::c_int
    {
        return false_0 != 0;
    }
    if memcmp(
        p.offset(6 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        b"%!PS-AdobeFont\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        14 as size_t,
    ) == 0
        || memcmp(
            p.offset(6 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            b"%!FontType1\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            11 as size_t,
        ) == 0
    {
        return true_0 != 0;
    }
    if memcmp(
        p.offset(6 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
        b"%!PS\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        return true_0 != 0;
    }
    return false_0 != 0;
}
unsafe extern "C-unwind" fn check_stream_is_dfont(mut handle: rust_input_handle_t) -> bool {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut pos: uint32_t = 0;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    tt_get_unsigned_quad(handle);
    pos = tt_get_unsigned_quad(handle);
    crate::backend::runtime::input_seek(
        handle,
        pos.wrapping_add(0x18 as uint32_t) as ssize_t,
        SEEK_SET,
    );
    crate::backend::runtime::input_seek(
        handle,
        pos.wrapping_add(tt_get_unsigned_pair(handle) as uint32_t) as ssize_t,
        SEEK_SET,
    );
    n = tt_get_unsigned_pair(handle) as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i <= n {
        if tt_get_unsigned_quad(handle) as ::core::ffi::c_ulong
            == 0x73666e74 as ::core::ffi::c_ulong
        {
            return true_0 != 0;
        }
        tt_get_unsigned_quad(handle);
        i += 1;
    }
    return false_0 != 0;
}
unsafe extern "C-unwind" fn ensuresuffix(
    mut basename: *const ::core::ffi::c_char,
    mut sfx: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = new((strlen(basename)
        .wrapping_add(strlen(sfx))
        .wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    strcpy(p, basename);
    q = strrchr(p, '.' as i32);
    if q.is_null() && *sfx.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 0 {
        strcat(p, sfx);
    }
    return p;
}
#[unsafe(export_name = "__pdf_impl_dpx_tt_open")]
pub unsafe extern "C-unwind" fn dpx_tt_open(
    mut filename: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
    mut format: ttbc_file_format,
) -> rust_input_handle_t {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut handle: rust_input_handle_t = 0;
    q = ensuresuffix(filename, suffix);
    handle = ttstub_input_open(q, format, 0 as ::core::ffi::c_int);
    free(q as *mut ::core::ffi::c_void);
    return handle;
}
#[unsafe(export_name = "__pdf_impl_dpx_open_type1_file")]
pub unsafe extern "C-unwind" fn dpx_open_type1_file(
    mut filename: *const ::core::ffi::c_char,
) -> rust_input_handle_t {
    let mut handle: rust_input_handle_t = 0;
    handle = ttstub_input_open(filename, TTBC_FILE_FORMAT_TYPE1, 0 as ::core::ffi::c_int);
    if handle == INVALID_HANDLE {
        return INVALID_HANDLE;
    }
    if !check_stream_is_type1(handle) {
        crate::backend::runtime::input_close(handle);
        return INVALID_HANDLE;
    }
    return handle;
}
#[unsafe(export_name = "__pdf_impl_dpx_open_truetype_file")]
pub unsafe extern "C-unwind" fn dpx_open_truetype_file(
    mut filename: *const ::core::ffi::c_char,
) -> rust_input_handle_t {
    let mut handle: rust_input_handle_t = 0;
    handle = ttstub_input_open(
        filename,
        TTBC_FILE_FORMAT_TRUE_TYPE,
        0 as ::core::ffi::c_int,
    );
    if handle == INVALID_HANDLE {
        return INVALID_HANDLE;
    }
    if !check_stream_is_truetype(handle) {
        crate::backend::runtime::input_close(handle);
        return INVALID_HANDLE;
    }
    return handle;
}
#[unsafe(export_name = "__pdf_impl_dpx_open_opentype_file")]
pub unsafe extern "C-unwind" fn dpx_open_opentype_file(
    mut filename: *const ::core::ffi::c_char,
) -> rust_input_handle_t {
    let mut handle: rust_input_handle_t = 0;
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    q = ensuresuffix(
        filename,
        b".otf\0" as *const u8 as *const ::core::ffi::c_char,
    );
    handle = ttstub_input_open(q, TTBC_FILE_FORMAT_OPEN_TYPE, 0 as ::core::ffi::c_int);
    free(q as *mut ::core::ffi::c_void);
    if handle == INVALID_HANDLE {
        return INVALID_HANDLE;
    }
    if !check_stream_is_opentype(handle) {
        crate::backend::runtime::input_close(handle);
        return INVALID_HANDLE;
    }
    return handle;
}
#[unsafe(export_name = "__pdf_impl_dpx_open_dfont_file")]
pub unsafe extern "C-unwind" fn dpx_open_dfont_file(
    mut filename: *const ::core::ffi::c_char,
) -> rust_input_handle_t {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut handle: rust_input_handle_t = 0;
    let mut len: ::core::ffi::c_int = strlen(filename) as ::core::ffi::c_int;
    if len > 6 as ::core::ffi::c_int
        && strncmp(
            filename
                .offset(len as isize)
                .offset(-(6 as ::core::ffi::c_int as isize)),
            b".dfont\0" as *const u8 as *const ::core::ffi::c_char,
            6 as size_t,
        ) != 0
    {
        q = new(((len + 6 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        strcpy(q, filename);
        strcat(q, b"/rsrc\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
        q = crate::backend::runtime::duplicate(filename);
    }
    handle = ttstub_input_open(q, TTBC_FILE_FORMAT_TRUE_TYPE, 0 as ::core::ffi::c_int);
    free(q as *mut ::core::ffi::c_void);
    if handle == INVALID_HANDLE {
        return INVALID_HANDLE;
    }
    if !check_stream_is_dfont(handle) {
        crate::backend::runtime::input_close(handle);
        return INVALID_HANDLE;
    }
    return handle;
}
unsafe extern "C-unwind" fn dpx_get_tmpdir() -> *mut ::core::ffi::c_char {
    let mut i: size_t = 0;
    let mut ret: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut _tmpd: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    _tmpd = getenv(b"TMPDIR\0" as *const u8 as *const ::core::ffi::c_char);
    if _tmpd.is_null() {
        _tmpd = __TMPDIR.as_ptr();
    }
    ret = crate::backend::runtime::duplicate(_tmpd);
    i = strlen(ret);
    while i > 1 as size_t
        && *ret.offset(i.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int == '/' as i32
    {
        *ret.offset(i.wrapping_sub(1 as size_t) as isize) = '\0' as i32 as ::core::ffi::c_char;
        i = i.wrapping_sub(1);
    }
    return ret;
}
pub const __TMPDIR: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"/tmp\0") };
#[unsafe(export_name = "__pdf_impl_dpx_create_temp_file")]
pub unsafe extern "C-unwind" fn dpx_create_temp_file() -> *mut ::core::ffi::c_char {
    let mut tmpdir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: size_t = 0;
    let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    tmpdir = dpx_get_tmpdir();
    n = strlen(tmpdir)
        .wrapping_add(strlen(TEMPLATE.as_ptr()))
        .wrapping_add(1 as size_t);
    tmp = new((n as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    strcpy(tmp, tmpdir);
    free(tmpdir as *mut ::core::ffi::c_void);
    strcat(tmp, TEMPLATE.as_ptr());
    let mut _fd: ::core::ffi::c_int = crate::backend::runtime::temporary_file(tmp);
    if _fd != -(1 as ::core::ffi::c_int) {
        libc::close(_fd);
    } else {
        tmp = mfree(tmp as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
    }
    return tmp;
}
pub const TEMPLATE: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"/dvipdfmx.XXXXXX\0")
};
#[unsafe(export_name = "__pdf_impl_dpx_delete_old_cache")]
pub unsafe extern "C-unwind" fn dpx_delete_old_cache(mut life: ::core::ffi::c_int) {
    if life == -(2 as ::core::ffi::c_int) {
        dpx_conf.file.keep_cache = -(1 as ::core::ffi::c_int);
    }
}
#[unsafe(export_name = "__pdf_impl_dpx_delete_temp_file")]
pub unsafe extern "C-unwind" fn dpx_delete_temp_file(
    mut tmp: *mut ::core::ffi::c_char,
    mut force: ::core::ffi::c_int,
) {
    if tmp.is_null() {
        return;
    }
    if force != 0 || dpx_conf.file.keep_cache != 1 as ::core::ffi::c_int {
        remove(tmp);
    }
    free(tmp as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_dpx_file_apply_filter")]
pub unsafe extern "C-unwind" fn dpx_file_apply_filter(
    mut cmdtmpl: *const ::core::ffi::c_char,
    mut input: *const ::core::ffi::c_char,
    mut output: *const ::core::ffi::c_char,
    mut version: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return -(1 as ::core::ffi::c_int);
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dpx_tt_open(
    mut filename: *const ::core::ffi::c_char,
    mut suffix: *const ::core::ffi::c_char,
    mut format: ttbc_file_format,
    _result: *mut rust_input_handle_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_tt_open(filename, suffix, format) }) {
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
pub unsafe extern "C" fn __pdf_entry_dpx_open_type1_file(
    mut filename: *const ::core::ffi::c_char,
    _result: *mut rust_input_handle_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_open_type1_file(filename) }) {
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
pub unsafe extern "C" fn __pdf_entry_dpx_open_truetype_file(
    mut filename: *const ::core::ffi::c_char,
    _result: *mut rust_input_handle_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_open_truetype_file(filename) }) {
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
pub unsafe extern "C" fn __pdf_entry_dpx_open_opentype_file(
    mut filename: *const ::core::ffi::c_char,
    _result: *mut rust_input_handle_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_open_opentype_file(filename) }) {
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
pub unsafe extern "C" fn __pdf_entry_dpx_open_dfont_file(
    mut filename: *const ::core::ffi::c_char,
    _result: *mut rust_input_handle_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_open_dfont_file(filename) }) {
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
pub unsafe extern "C" fn __pdf_entry_dpx_create_temp_file(
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_create_temp_file() }) {
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
pub unsafe extern "C" fn __pdf_entry_dpx_delete_old_cache(mut life: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_delete_old_cache(life) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dpx_delete_temp_file(
    mut tmp: *mut ::core::ffi::c_char,
    mut force: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_delete_temp_file(tmp, force) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dpx_file_apply_filter(
    mut cmdtmpl: *const ::core::ffi::c_char,
    mut input: *const ::core::ffi::c_char,
    mut output: *const ::core::ffi::c_char,
    mut version: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        dpx_file_apply_filter(cmdtmpl, input, output, version)
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
