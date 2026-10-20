/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
   the dvipdfmx project team.

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
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn isspace(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strtol(__str: *const ::core::ffi::c_char, __endptr: *mut *mut ::core::ffi::c_char, __base: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcpy(__dst: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_open(path: *const ::core::ffi::c_char, format: ttbc_file_format, is_gz: ::core::ffi::c_int) -> rust_input_handle_t;
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: ::core::ffi::c_int) -> size_t;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    static mut dpx_conf: _dpx_conf;
    #[link_name = "__pdf_impl_tt_mfgets"]
    fn tt_mfgets(buffer: *mut ::core::ffi::c_char, length: ::core::ffi::c_int, file: rust_input_handle_t) -> *mut ::core::ffi::c_char;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
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
pub struct sfd_file_ {
    pub ident: *mut ::core::ffi::c_char,
    pub sub_id: *mut *mut ::core::ffi::c_char,
    pub rec_id: *mut ::core::ffi::c_int,
    pub max_subfonts: ::core::ffi::c_int,
    pub num_subfonts: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfd_rec_ {
    pub vector: [::core::ffi::c_ushort; 256],
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
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
#[inline]
unsafe extern "C-unwind" fn streq_ptr(mut s1: *const ::core::ffi::c_char, mut s2: *const ::core::ffi::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
    }
    return false_0 != 0;
}
unsafe extern "C-unwind" fn init_sfd_file_(mut sfd: *mut sfd_file_) {
    (*sfd).ident = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*sfd).sub_id = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    (*sfd).rec_id = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*sfd).num_subfonts = 0 as ::core::ffi::c_int;
    (*sfd).max_subfonts = (*sfd).num_subfonts;
}
unsafe extern "C-unwind" fn clean_sfd_file_(mut sfd: *mut sfd_file_) {
    let mut i: ::core::ffi::c_int = 0;
    free((*sfd).ident as *mut ::core::ffi::c_void);
    if !(*sfd).sub_id.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*sfd).num_subfonts {
            free(*(*sfd).sub_id.offset(i as isize) as *mut ::core::ffi::c_void);
            i += 1;
        }
        free((*sfd).sub_id as *mut ::core::ffi::c_void);
    }
    free((*sfd).rec_id as *mut ::core::ffi::c_void);
    init_sfd_file_(sfd);
}

pub const LINE_BUF_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;

unsafe extern "C-unwind" fn readline(
    mut buf: *mut ::core::ffi::c_char, mut buf_len: ::core::ffi::c_int, mut handle: rust_input_handle_t,
) -> *mut ::core::ffi::c_char {
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = buf;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while buf_len - n > 0 as ::core::ffi::c_int && {
        q = tt_mfgets(p, buf_len - n, handle);
        !q.is_null()
    } {
        c += 1;
        r = strchr(q, '#' as i32);
        if !r.is_null() {
            *r = ' ' as i32 as ::core::ffi::c_char;
            *r.offset(1 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
        if strlen(q) == 0 as size_t {
            break;
        }
        n = (n as size_t).wrapping_add(strlen(q)) as ::core::ffi::c_int as ::core::ffi::c_int;
        q = q.offset(strlen(q).wrapping_sub(1 as size_t) as isize);
        if *q as ::core::ffi::c_int != '\\' as i32 {
            break;
        }
        n -= 1 as ::core::ffi::c_int;
        p = buf.offset(n as isize);
    }
    if n >= buf_len - 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"Possible buffer overflow in reading SFD file (buffer full, size=%d bytes)\0" as *const u8 as *const ::core::ffi::c_char,
            buf_len - 1 as ::core::ffi::c_int,
        );
    }
    return if c > 0 as ::core::ffi::c_int {
        buf
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    };
}
unsafe extern "C-unwind" fn read_sfd_record(mut rec: *mut sfd_rec_, mut lbuf: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = lbuf;
    let mut q: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut repos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0;
    let mut v1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut v2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut curpos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
        p = p.offset(1);
    }
    while error == 0 && *p as ::core::ffi::c_int != 0 {
        repos = 0 as ::core::ffi::c_int;
        q = p;
        v1 = strtol(p, &raw mut r, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        q = r;
        if q == p
            || !(*q as ::core::ffi::c_int == '\0' as i32 || isspace(*q as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0)
                && *q as ::core::ffi::c_int != ':' as i32
                && *q as ::core::ffi::c_int != '_' as i32
        {
            pdf_warning!(
                b"Unknown token in subfont mapping table: %c\0" as *const u8 as *const ::core::ffi::c_char,
                *q as ::core::ffi::c_int,
            );
            return -(1 as ::core::ffi::c_int);
        }
        match *q as ::core::ffi::c_int {
            58 => {
                if v1 < 0 as ::core::ffi::c_int || v1 > 0xff as ::core::ffi::c_int {
                    pdf_warning!(
                        b"Invalud value for subfont table offset: %d\0" as *const u8 as *const ::core::ffi::c_char,
                        v1,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                repos = 1 as ::core::ffi::c_int;
                q = q.offset(1);
            }
            95 => {
                p = q.offset(1 as ::core::ffi::c_int as isize);
                v2 = strtol(p, &raw mut r, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                q = r;
                if v1 < 0 as ::core::ffi::c_int
                    || v1 as ::core::ffi::c_long > 0xffff as ::core::ffi::c_long
                    || v2 < 0 as ::core::ffi::c_int
                    || v2 as ::core::ffi::c_long > 0xffff as ::core::ffi::c_long
                {
                    pdf_warning!(
                        b"Invalid value in subfont mapping table: 0x%x_0x%x\0" as *const u8 as *const ::core::ffi::c_char,
                        v1,
                        v2,
                    );
                    return -(1 as ::core::ffi::c_int);
                } else if q == p || !(*q as ::core::ffi::c_int == '\0' as i32 || isspace(*q as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0) {
                    pdf_warning!(
                        b"Invalid char in subfont mapping table: %c\0" as *const u8 as *const ::core::ffi::c_char,
                        *q as ::core::ffi::c_int,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
            }
            _ => {
                if v1 < 0 as ::core::ffi::c_int || v1 as ::core::ffi::c_long > 0xffff as ::core::ffi::c_long {
                    pdf_warning!(
                        b"Invalid character code in subfont mapping table: 0x%x\0" as *const u8 as *const ::core::ffi::c_char,
                        v1,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                v2 = v1;
            }
        }
        if repos != 0 {
            curpos = v1;
        } else {
            if v2 < v1 || curpos + (v2 - v1) > 0xff as ::core::ffi::c_int {
                pdf_warning!(
                    b"Invalid range in subfont mapping: curpos=\"0x%02x\" range=\"0x%04x,0x%04x\"\0" as *const u8 as *const ::core::ffi::c_char,
                    curpos,
                    v1,
                    v2,
                );
                return -(1 as ::core::ffi::c_int);
            }
            c = v1;
            while c <= v2 {
                if (*rec).vector[curpos as usize] as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    pdf_warning!(
                        b"Subfont mapping for slot=\"0x%02x\" already defined...\0" as *const u8 as *const ::core::ffi::c_char,
                        curpos,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                if !(curpos >= 0 as ::core::ffi::c_int && curpos <= 255 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                    pdf_abort!(
                        c"assertion failed: %s".as_ptr(),
                        b"curpos >= 0 && curpos <= 255\0" as *const u8 as *const ::core::ffi::c_char
                    );
                } else {
                };
                let fresh1 = curpos;
                curpos = curpos + 1;
                (*rec).vector[fresh1 as usize] = c as ::core::ffi::c_ushort;
                c += 1;
            }
        }
        p = q;
        while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            p = p.offset(1);
        }
    }
    return error;
}
unsafe extern "C-unwind" fn scan_sfd_file(mut sfd: *mut sfd_file_, mut handle: rust_input_handle_t) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut id: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    let mut lpos: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(!sfd.is_null() && handle != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"sfd && handle\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
        pdf_message!(
            b"\nsubfont>> Scanning SFD file \"%s\"...\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*sfd).ident,
        );
    }
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    (*sfd).num_subfonts = 0 as ::core::ffi::c_int;
    (*sfd).max_subfonts = (*sfd).num_subfonts;
    loop {
        p = readline(&raw mut (*__pdf_state).line_buf as *mut ::core::ffi::c_char, LINE_BUF_SIZE, handle);
        if p.is_null() {
            break;
        }
        lpos += 1;
        while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            continue;
        }
        n = 0 as ::core::ffi::c_int;
        q = p;
        while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
            p = p.offset(1);
            n += 1;
        }
        id = new(
            ((n + 1 as ::core::ffi::c_int) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t,
        ) as *mut ::core::ffi::c_char;
        memcpy(id as *mut ::core::ffi::c_void, q as *const ::core::ffi::c_void, n as size_t);
        *id.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
        if (*sfd).num_subfonts >= (*sfd).max_subfonts {
            (*sfd).max_subfonts += 16 as ::core::ffi::c_int;
            (*sfd).sub_id = renew(
                (*sfd).sub_id as *mut ::core::ffi::c_void,
                ((*sfd).max_subfonts as uint32_t as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize) as uint32_t,
            ) as *mut *mut ::core::ffi::c_char;
        }
        if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
            pdf_message!(
                b"subfont>>   id=\"%s\" at line=\"%d\"\n\0" as *const u8 as *const ::core::ffi::c_char,
                id,
                lpos,
            );
        }
        let ref mut fresh3 = *(*sfd).sub_id.offset((*sfd).num_subfonts as isize);
        *fresh3 = id;
        (*sfd).num_subfonts += 1;
    }
    (*sfd).rec_id = new(((*sfd).num_subfonts as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize) as uint32_t)
        as *mut ::core::ffi::c_int;
    n = 0 as ::core::ffi::c_int;
    while n < (*sfd).num_subfonts {
        *(*sfd).rec_id.offset(n as isize) = -(1 as ::core::ffi::c_int);
        n += 1;
    }
    if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
        pdf_message!(
            b"subfont>> %d entries found in SFD file \"%s\".\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*sfd).num_subfonts,
            (*sfd).ident,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn find_sfd_file(mut sfd_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < (*__pdf_state).num_sfd_files {
        if streq_ptr((*(*__pdf_state).sfd_files.offset(i as isize)).ident, sfd_name) {
            id = i;
            break;
        } else {
            i += 1;
        }
    }
    if id < 0 as ::core::ffi::c_int {
        let mut sfd: *mut sfd_file_ = ::core::ptr::null_mut::<sfd_file_>();
        let mut handle: rust_input_handle_t = INVALID_HANDLE;
        if (*__pdf_state).num_sfd_files >= (*__pdf_state).max_sfd_files {
            (*__pdf_state).max_sfd_files += 8 as ::core::ffi::c_int;
            (*__pdf_state).sfd_files = renew(
                (*__pdf_state).sfd_files as *mut ::core::ffi::c_void,
                ((*__pdf_state).max_sfd_files as uint32_t as usize).wrapping_mul(::core::mem::size_of::<sfd_file_>() as usize) as uint32_t,
            ) as *mut sfd_file_;
        }
        sfd = (*__pdf_state).sfd_files.offset((*__pdf_state).num_sfd_files as isize) as *mut sfd_file_;
        init_sfd_file_(sfd);
        (*sfd).ident = new((strlen(sfd_name).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
        strcpy((*sfd).ident, sfd_name);
        handle = ttstub_input_open((*sfd).ident, TTBC_FILE_FORMAT_SFD, 0 as ::core::ffi::c_int);
        if handle == INVALID_HANDLE {
            clean_sfd_file_(sfd);
            return -(1 as ::core::ffi::c_int);
        }
        error = scan_sfd_file(sfd, handle);
        crate::backend::runtime::input_close(handle);
        if error == 0 {
            let fresh2 = (*__pdf_state).num_sfd_files;
            (*__pdf_state).num_sfd_files = (*__pdf_state).num_sfd_files + 1;
            id = fresh2;
        } else {
            pdf_warning!(
                b"Error occured while reading SFD file \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                sfd_name,
            );
            clean_sfd_file_(sfd);
            id = -(1 as ::core::ffi::c_int);
        }
    }
    return id;
}
#[unsafe(export_name = "__pdf_impl_sfd_get_subfont_ids")]
pub unsafe extern "C-unwind" fn sfd_get_subfont_ids(
    mut sfd_name: *const ::core::ffi::c_char, mut num_ids: *mut ::core::ffi::c_int,
) -> *mut *mut ::core::ffi::c_char {
    let __pdf_state = unsafe { document_state() };
    let mut sfd_id: ::core::ffi::c_int = 0;
    if sfd_name.is_null() {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    sfd_id = find_sfd_file(sfd_name);
    if sfd_id < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    }
    if !num_ids.is_null() {
        *num_ids = (*(*__pdf_state).sfd_files.offset(sfd_id as isize)).num_subfonts;
    }
    return (*(*__pdf_state).sfd_files.offset(sfd_id as isize)).sub_id;
}
#[unsafe(export_name = "__pdf_impl_sfd_load_record")]
pub unsafe extern "C-unwind" fn sfd_load_record(
    mut sfd_name: *const ::core::ffi::c_char, mut subfont_id: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut rec_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut sfd: *mut sfd_file_ = ::core::ptr::null_mut::<sfd_file_>();
    let mut handle: rust_input_handle_t = 0;
    let mut sfd_id: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if sfd_name.is_null() || subfont_id.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    sfd_id = find_sfd_file(sfd_name);
    if sfd_id < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    sfd = (*__pdf_state).sfd_files.offset(sfd_id as isize) as *mut sfd_file_;
    i = 0 as ::core::ffi::c_int;
    while i < (*sfd).num_subfonts && strcmp(*(*sfd).sub_id.offset(i as isize), subfont_id) != 0 {
        i += 1;
    }
    if i == (*sfd).num_subfonts {
        pdf_warning!(
            b"Subfont id=\"%s\" not exist in SFD file \"%s\"...\0" as *const u8 as *const ::core::ffi::c_char,
            subfont_id,
            (*sfd).ident,
        );
        return -(1 as ::core::ffi::c_int);
    } else if *(*sfd).rec_id.offset(i as isize) >= 0 as ::core::ffi::c_int {
        return *(*sfd).rec_id.offset(i as isize);
    }
    if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
        pdf_message!(
            b"\nsubfont>> Loading SFD mapping table for <%s,%s>...\0" as *const u8 as *const ::core::ffi::c_char,
            (*sfd).ident,
            subfont_id,
        );
    }
    handle = ttstub_input_open((*sfd).ident, TTBC_FILE_FORMAT_SFD, 0 as ::core::ffi::c_int);
    if handle == INVALID_HANDLE {
        return -(1 as ::core::ffi::c_int);
    }
    loop {
        p = readline(&raw mut (*__pdf_state).line_buf as *mut ::core::ffi::c_char, LINE_BUF_SIZE, handle);
        if p.is_null() {
            break;
        }
        while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            continue;
        }
        q = p;
        while *p as ::core::ffi::c_int != 0 && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) == 0 {
            p = p.offset(1);
        }
        *p = '\0' as i32 as ::core::ffi::c_char;
        p = p.offset(1);
        if streq_ptr(q, subfont_id) {
            if (*__pdf_state).num_sfd_records >= (*__pdf_state).max_sfd_records {
                (*__pdf_state).max_sfd_records += 16 as ::core::ffi::c_int;
                (*__pdf_state).sfd_record = renew(
                    (*__pdf_state).sfd_record as *mut ::core::ffi::c_void,
                    ((*__pdf_state).max_sfd_records as uint32_t as usize).wrapping_mul(::core::mem::size_of::<sfd_rec_>() as usize) as uint32_t,
                ) as *mut sfd_rec_;
            }
            if !(&raw mut (*(*__pdf_state).sfd_record.offset((*__pdf_state).num_sfd_records as isize)).vector as *mut ::core::ffi::c_ushort).is_null()
            {
                let mut __i: ::core::ffi::c_int = 0;
                __i = 0 as ::core::ffi::c_int;
                while __i < 256 as ::core::ffi::c_int {
                    (*(*__pdf_state).sfd_record.offset((*__pdf_state).num_sfd_records as isize)).vector[__i as usize] = 0 as ::core::ffi::c_ushort;
                    __i += 1;
                }
            }
            error = read_sfd_record(
                (*__pdf_state).sfd_record.offset((*__pdf_state).num_sfd_records as isize) as *mut sfd_rec_,
                p,
            );
            if error != 0 {
                pdf_warning!(
                    b"Error occured while reading SFD file: file=\"%s\" subfont_id=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                    (*sfd).ident,
                    subfont_id,
                );
            } else {
                let fresh0 = (*__pdf_state).num_sfd_records;
                (*__pdf_state).num_sfd_records = (*__pdf_state).num_sfd_records + 1;
                rec_id = fresh0;
            }
        }
    }
    if rec_id < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Failed to load subfont mapping table for SFD=\"%s\" subfont_id=\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            (*sfd).ident,
            subfont_id,
        );
    }
    *(*sfd).rec_id.offset(i as isize) = rec_id;
    crate::backend::runtime::input_close(handle);
    if dpx_conf.verbose_level > 3 as ::core::ffi::c_int {
        let mut __i_0: ::core::ffi::c_int = 0;
        if rec_id >= 0 as ::core::ffi::c_int {
            pdf_message!(b" at id=\"%d\"\0" as *const u8 as *const ::core::ffi::c_char, rec_id,);
            pdf_message!(b"\nsubfont>> Content of mapping table:\0" as *const u8 as *const ::core::ffi::c_char,);
            __i_0 = 0 as ::core::ffi::c_int;
            while __i_0 < 256 as ::core::ffi::c_int {
                if __i_0 % 16 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    pdf_message!(b"\nsubfont>>  \0" as *const u8 as *const ::core::ffi::c_char,);
                }
                pdf_message!(
                    b" %04x\0" as *const u8 as *const ::core::ffi::c_char,
                    (*(*__pdf_state).sfd_record.offset(rec_id as isize)).vector[__i_0 as usize] as ::core::ffi::c_int,
                );
                __i_0 += 1;
            }
        }
        pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return rec_id;
}
#[unsafe(export_name = "__pdf_impl_lookup_sfd_record")]
pub unsafe extern "C-unwind" fn lookup_sfd_record(mut rec_id: ::core::ffi::c_int, mut c: ::core::ffi::c_uchar) -> ::core::ffi::c_ushort {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).sfd_record.is_null() || rec_id < 0 as ::core::ffi::c_int || rec_id >= (*__pdf_state).num_sfd_records {
        pdf_abort!(b"Invalid subfont_id: %d\0" as *const u8 as *const ::core::ffi::c_char, rec_id,);
    }
    return (*(*__pdf_state).sfd_record.offset(rec_id as isize)).vector[c as usize];
}
#[unsafe(export_name = "__pdf_impl_release_sfd_record")]
pub unsafe extern "C-unwind" fn release_sfd_record() {
    let __pdf_state = unsafe { document_state() };
    let mut i: ::core::ffi::c_int = 0;
    if !(*__pdf_state).sfd_record.is_null() {
        free((*__pdf_state).sfd_record as *mut ::core::ffi::c_void);
    }
    if !(*__pdf_state).sfd_files.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*__pdf_state).num_sfd_files {
            clean_sfd_file_((*__pdf_state).sfd_files.offset(i as isize) as *mut sfd_file_);
            i += 1;
        }
        free((*__pdf_state).sfd_files as *mut ::core::ffi::c_void);
    }
    (*__pdf_state).sfd_record = ::core::ptr::null_mut::<sfd_rec_>();
    (*__pdf_state).sfd_files = ::core::ptr::null_mut::<sfd_file_>();
    (*__pdf_state).max_sfd_records = 0 as ::core::ffi::c_int;
    (*__pdf_state).num_sfd_records = (*__pdf_state).max_sfd_records;
    (*__pdf_state).max_sfd_files = 0 as ::core::ffi::c_int;
    (*__pdf_state).num_sfd_files = (*__pdf_state).max_sfd_files;
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_sfd_get_subfont_ids(
    mut sfd_name: *const ::core::ffi::c_char, mut num_ids: *mut ::core::ffi::c_int, _result: *mut *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sfd_get_subfont_ids(sfd_name, num_ids) }) {
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
pub unsafe extern "C" fn __pdf_entry_sfd_load_record(
    mut sfd_name: *const ::core::ffi::c_char, mut subfont_id: *const ::core::ffi::c_char, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sfd_load_record(sfd_name, subfont_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_lookup_sfd_record(
    mut rec_id: ::core::ffi::c_int, mut c: ::core::ffi::c_uchar, _result: *mut ::core::ffi::c_ushort,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { lookup_sfd_record(rec_id, c) }) {
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
pub unsafe extern "C" fn __pdf_entry_release_sfd_record() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { release_sfd_record() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}

pub(crate) struct DocumentState {
    sfd_files: *mut sfd_file_,
    num_sfd_files: ::core::ffi::c_int,
    max_sfd_files: ::core::ffi::c_int,
    sfd_record: *mut sfd_rec_,
    max_sfd_records: ::core::ffi::c_int,
    num_sfd_records: ::core::ffi::c_int,
    line_buf: [::core::ffi::c_char; 4096],
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        unsafe {
            Self {
                sfd_files: ::core::ptr::null::<sfd_file_>() as *mut sfd_file_,
                num_sfd_files: 0 as ::core::ffi::c_int,
                max_sfd_files: 0 as ::core::ffi::c_int,
                sfd_record: ::core::ptr::null::<sfd_rec_>() as *mut sfd_rec_,
                max_sfd_records: 0 as ::core::ffi::c_int,
                num_sfd_records: 0 as ::core::ffi::c_int,
                line_buf: [0; 4096],
            }
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe { (*crate::compat::document::current()).backend.dpx_subfont.pointer }
}
