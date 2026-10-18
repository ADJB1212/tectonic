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
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strstr(
        __big: *const ::core::ffi::c_char,
        __little: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
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
    #[link_name = "__pdf_impl_CMap_is_valid"]
    fn CMap_is_valid(cmap: *mut CMap) -> bool;
    #[link_name = "__pdf_impl_CMap_set_name"]
    fn CMap_set_name(cmap: *mut CMap, name: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_CMap_set_type"]
    fn CMap_set_type(cmap: *mut CMap, type_0: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_CMap_set_wmode"]
    fn CMap_set_wmode(cmap: *mut CMap, wmode: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_CMap_set_usecmap"]
    fn CMap_set_usecmap(cmap: *mut CMap, ucmap: *mut CMap);
    #[link_name = "__pdf_impl_CMap_set_CIDSysInfo"]
    fn CMap_set_CIDSysInfo(cmap: *mut CMap, csi: *const CIDSysInfo);
    #[link_name = "__pdf_impl_CMap_add_bfchar"]
    fn CMap_add_bfchar(
        cmap: *mut CMap,
        src: *const ::core::ffi::c_uchar,
        srcdim: size_t,
        dest: *const ::core::ffi::c_uchar,
        destdim: size_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_add_cidchar"]
    fn CMap_add_cidchar(
        cmap: *mut CMap,
        src: *const ::core::ffi::c_uchar,
        srcdim: size_t,
        dest: CID,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_add_bfrange"]
    fn CMap_add_bfrange(
        cmap: *mut CMap,
        srclo: *const ::core::ffi::c_uchar,
        srchi: *const ::core::ffi::c_uchar,
        srcdim: size_t,
        dest: *const ::core::ffi::c_uchar,
        destdim: size_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_add_cidrange"]
    fn CMap_add_cidrange(
        cmap: *mut CMap,
        srclo: *const ::core::ffi::c_uchar,
        hi: *const ::core::ffi::c_uchar,
        srcdim: size_t,
        base: CID,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_add_notdefchar"]
    fn CMap_add_notdefchar(
        cmap: *mut CMap,
        src: *const ::core::ffi::c_uchar,
        srcdim: size_t,
        dst: CID,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_add_notdefrange"]
    fn CMap_add_notdefrange(
        cmap: *mut CMap,
        srclo: *const ::core::ffi::c_uchar,
        srchi: *const ::core::ffi::c_uchar,
        srcdim: size_t,
        dst: CID,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_add_codespacerange"]
    fn CMap_add_codespacerange(
        cmap: *mut CMap,
        codelo: *const ::core::ffi::c_uchar,
        codehi: *const ::core::ffi::c_uchar,
        dim: size_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_cache_get"]
    fn CMap_cache_get(id: ::core::ffi::c_int) -> *mut CMap;
    #[link_name = "__pdf_impl_CMap_cache_find"]
    fn CMap_cache_find(cmap_name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pst_get_token"]
    fn pst_get_token(
        inbuf: *mut *mut ::core::ffi::c_uchar,
        inbufend: *mut ::core::ffi::c_uchar,
    ) -> *mut pst_obj;
    #[link_name = "__pdf_impl_pst_release_obj"]
    fn pst_release_obj(obj: *mut pst_obj);
    #[link_name = "__pdf_impl_pst_type_of"]
    fn pst_type_of(obj: *mut pst_obj) -> pst_type;
    #[link_name = "__pdf_impl_pst_length_of"]
    fn pst_length_of(obj: *mut pst_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pst_getIV"]
    fn pst_getIV(obj: *mut pst_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pst_getSV"]
    fn pst_getSV(obj: *mut pst_obj) -> *mut ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_pst_data_ptr"]
    fn pst_data_ptr(obj: *mut pst_obj) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
pub type rust_input_handle_t = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDSysInfo {
    pub registry: *mut ::core::ffi::c_char,
    pub ordering: *mut ::core::ffi::c_char,
    pub supplement: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rangeDef {
    pub dim: size_t,
    pub codeLo: *mut ::core::ffi::c_uchar,
    pub codeHi: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapDef {
    pub flag: ::core::ffi::c_int,
    pub len: size_t,
    pub code: *mut ::core::ffi::c_uchar,
    pub next: *mut mapDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapData {
    pub data: *mut ::core::ffi::c_uchar,
    pub prev: *mut mapData,
    pub pos: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_0,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: ::core::ffi::c_int,
    pub profile: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub num: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
    pub ranges: *mut rangeDef,
}
pub type CID = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifreader {
    pub cursor: *mut ::core::ffi::c_uchar,
    pub endptr: *mut ::core::ffi::c_uchar,
    pub buf: *mut ::core::ffi::c_uchar,
    pub max: size_t,
    pub handle: rust_input_handle_t,
    pub unread: size_t,
}
pub type pst_type = ::core::ffi::c_int;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
#[inline]
unsafe extern "C-unwind" fn strstartswith(
    mut s: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0 as ::core::ffi::c_int {
        return s.offset(length as isize);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
pub const CID_MAX_CID: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const CID_MAX: ::core::ffi::c_int = CID_MAX_CID;
pub const PST_TYPE_INTEGER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PST_TYPE_STRING: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const PST_TYPE_NAME: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PST_TYPE_MARK: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CMAP_PARSE_DEBUG_STR: [::core::ffi::c_char; 12] =
    crate::backend::c_char_array(*b"CMap_parse:\0");
unsafe extern "C-unwind" fn ifreader_create(
    mut handle: rust_input_handle_t,
    mut size: size_t,
    mut bufsize: size_t,
) -> *mut ifreader {
    let mut reader: *mut ifreader = ::core::ptr::null_mut::<ifreader>();
    reader = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<ifreader>() as usize) as uint32_t)
        as *mut ifreader;
    (*reader).buf = new((bufsize.wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    (*reader).max = bufsize;
    (*reader).handle = handle;
    (*reader).unread = size;
    (*reader).endptr = (*reader).buf;
    (*reader).cursor = (*reader).endptr;
    *(*reader).endptr = 0 as ::core::ffi::c_uchar;
    return reader;
}
unsafe extern "C-unwind" fn ifreader_destroy(mut reader: *mut ifreader) {
    if reader.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"reader\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    free((*reader).buf as *mut ::core::ffi::c_void);
    free(reader as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn ifreader_read(mut reader: *mut ifreader, mut size: size_t) -> size_t {
    let mut bytesread: size_t = 0 as size_t;
    let mut bytesrem: size_t = 0 as size_t;
    if reader.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"reader\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    bytesrem = ((*reader).endptr as size_t).wrapping_sub((*reader).cursor as size_t);
    if size > (*reader).max {
        (*reader).buf = renew(
            (*reader).buf as *mut ::core::ffi::c_void,
            (size.wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_uchar;
        (*reader).max = size;
    }
    if (*reader).unread > 0 as size_t && bytesrem < size {
        bytesread = if (*reader).max.wrapping_sub(bytesrem) < (*reader).unread {
            (*reader).max.wrapping_sub(bytesrem)
        } else {
            (*reader).unread
        };
        memmove(
            (*reader).buf as *mut ::core::ffi::c_void,
            (*reader).cursor as *const ::core::ffi::c_void,
            bytesrem,
        );
        (*reader).cursor = (*reader).buf;
        (*reader).endptr = (*reader).buf.offset(bytesrem as isize);
        if ttstub_input_read(
            (*reader).handle,
            (*reader).endptr as *mut ::core::ffi::c_char,
            bytesread,
        ) as size_t
            != bytesread
        {
            pdf_abort!(b"Reading file failed.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        (*reader).endptr = (*reader).endptr.offset(bytesread as isize);
        (*reader).unread = (*reader).unread.wrapping_sub(bytesread);
    }
    *(*reader).endptr = 0 as ::core::ffi::c_uchar;
    return bytesread.wrapping_add(bytesrem);
}
pub const TOKEN_LEN_MAX: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn check_next_token(
    mut input: *mut ifreader,
    mut key: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cmp: ::core::ffi::c_int = 0;
    let mut token: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if ifreader_read(input, strlen(key)) == 0 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    token = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
    if token.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    str = pst_getSV(token) as *mut ::core::ffi::c_char;
    cmp = if strcmp(str, key) != 0 {
        -(1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
    free(str as *mut ::core::ffi::c_void);
    pst_release_obj(token);
    return cmp;
}
unsafe extern "C-unwind" fn get_coderange(
    mut input: *mut ifreader,
    mut codeLo: *mut ::core::ffi::c_uchar,
    mut codeHi: *mut ::core::ffi::c_uchar,
    mut dim: *mut ::core::ffi::c_int,
    mut maxlen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tok1: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut tok2: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut dim1: ::core::ffi::c_int = 0;
    let mut dim2: ::core::ffi::c_int = 0;
    tok1 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
    if tok1.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    tok2 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
    if tok2.is_null() {
        pst_release_obj(tok1);
        return -(1 as ::core::ffi::c_int);
    }
    if !(pst_type_of(tok1) == PST_TYPE_STRING) || !(pst_type_of(tok2) == PST_TYPE_STRING) {
        pst_release_obj(tok1);
        pst_release_obj(tok2);
        return -(1 as ::core::ffi::c_int);
    }
    dim1 = pst_length_of(tok1);
    dim2 = pst_length_of(tok2);
    if dim1 != dim2 || dim1 > maxlen {
        pst_release_obj(tok1);
        pst_release_obj(tok2);
        return -(1 as ::core::ffi::c_int);
    }
    memcpy(
        codeLo as *mut ::core::ffi::c_void,
        pst_data_ptr(tok1),
        dim1 as size_t,
    );
    memcpy(
        codeHi as *mut ::core::ffi::c_void,
        pst_data_ptr(tok2),
        dim2 as size_t,
    );
    pst_release_obj(tok1);
    pst_release_obj(tok2);
    *dim = dim1;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn do_codespacerange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut codeLo: [::core::ffi::c_uchar; 127] = [0; 127];
    let mut codeHi: [::core::ffi::c_uchar; 127] = [0; 127];
    let mut dim: ::core::ffi::c_int = 0;
    loop {
        let fresh8 = count;
        count = count - 1;
        if !(fresh8 > 0 as ::core::ffi::c_int) {
            break;
        }
        if get_coderange(
            input,
            &raw mut codeLo as *mut ::core::ffi::c_uchar,
            &raw mut codeHi as *mut ::core::ffi::c_uchar,
            &raw mut dim,
            TOKEN_LEN_MAX,
        ) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        CMap_add_codespacerange(
            cmap,
            &raw mut codeLo as *mut ::core::ffi::c_uchar,
            &raw mut codeHi as *mut ::core::ffi::c_uchar,
            dim as size_t,
        );
    }
    return check_next_token(
        input,
        b"endcodespacerange\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn handle_codearray(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut codeLo: *mut ::core::ffi::c_uchar,
    mut dim: ::core::ffi::c_int,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    if dim < 1 as ::core::ffi::c_int {
        pdf_abort!(b"Invalid code range.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    loop {
        let fresh4 = count;
        count = count - 1;
        if !(fresh4 > 0 as ::core::ffi::c_int) {
            break;
        }
        tok = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
        if tok.is_null() {
            return -(1 as ::core::ffi::c_int);
        } else if pst_type_of(tok) == PST_TYPE_STRING {
            CMap_add_bfchar(
                cmap,
                codeLo,
                dim as size_t,
                pst_data_ptr(tok) as *mut ::core::ffi::c_uchar,
                pst_length_of(tok) as size_t,
            );
        } else if pst_type_of(tok) == PST_TYPE_MARK || !(pst_type_of(tok) == PST_TYPE_NAME) {
            pdf_abort!(
                b"%s: Invalid CMap mapping record.\0" as *const u8 as *const ::core::ffi::c_char,
                CMAP_PARSE_DEBUG_STR.as_ptr(),
            );
        } else {
            pdf_abort!(
                b"%s: Mapping to charName not supported.\0" as *const u8
                    as *const ::core::ffi::c_char,
                CMAP_PARSE_DEBUG_STR.as_ptr(),
            );
        }
        pst_release_obj(tok);
        let ref mut fresh5 = *codeLo.offset((dim - 1 as ::core::ffi::c_int) as isize);
        *fresh5 = (*fresh5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    }
    return check_next_token(input, b"]\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C-unwind" fn do_notdefrange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut codeLo: [::core::ffi::c_uchar; 127] = [0; 127];
    let mut codeHi: [::core::ffi::c_uchar; 127] = [0; 127];
    let mut dstCID: ::core::ffi::c_int = 0;
    let mut dim: ::core::ffi::c_int = 0;
    loop {
        let fresh7 = count;
        count = count - 1;
        if !(fresh7 > 0 as ::core::ffi::c_int) {
            break;
        }
        if ifreader_read(input, (TOKEN_LEN_MAX * 3 as ::core::ffi::c_int) as size_t) == 0 as size_t
        {
            return -(1 as ::core::ffi::c_int);
        }
        if get_coderange(
            input,
            &raw mut codeLo as *mut ::core::ffi::c_uchar,
            &raw mut codeHi as *mut ::core::ffi::c_uchar,
            &raw mut dim,
            TOKEN_LEN_MAX,
        ) < 0 as ::core::ffi::c_int
            || {
                tok = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
                tok.is_null()
            }
        {
            return -(1 as ::core::ffi::c_int);
        }
        if pst_type_of(tok) == PST_TYPE_INTEGER {
            dstCID = pst_getIV(tok);
            if dstCID >= 0 as ::core::ffi::c_int && dstCID <= CID_MAX {
                CMap_add_notdefrange(
                    cmap,
                    &raw mut codeLo as *mut ::core::ffi::c_uchar,
                    &raw mut codeHi as *mut ::core::ffi::c_uchar,
                    dim as size_t,
                    dstCID as CID,
                );
            }
        } else {
            pdf_warning!(
                b"%s: Invalid CMap mapping record. (ignored)\0" as *const u8
                    as *const ::core::ffi::c_char,
                CMAP_PARSE_DEBUG_STR.as_ptr(),
            );
        }
        pst_release_obj(tok);
    }
    return check_next_token(
        input,
        b"endnotdefrange\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn do_bfrange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut codeLo: [::core::ffi::c_uchar; 127] = [0; 127];
    let mut codeHi: [::core::ffi::c_uchar; 127] = [0; 127];
    let mut srcdim: ::core::ffi::c_int = 0;
    loop {
        let fresh3 = count;
        count = count - 1;
        if !(fresh3 > 0 as ::core::ffi::c_int) {
            break;
        }
        if ifreader_read(input, (TOKEN_LEN_MAX * 3 as ::core::ffi::c_int) as size_t) == 0 as size_t
        {
            return -(1 as ::core::ffi::c_int);
        }
        if get_coderange(
            input,
            &raw mut codeLo as *mut ::core::ffi::c_uchar,
            &raw mut codeHi as *mut ::core::ffi::c_uchar,
            &raw mut srcdim,
            TOKEN_LEN_MAX,
        ) < 0 as ::core::ffi::c_int
            || {
                tok = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
                tok.is_null()
            }
        {
            return -(1 as ::core::ffi::c_int);
        }
        if pst_type_of(tok) == PST_TYPE_STRING {
            CMap_add_bfrange(
                cmap,
                &raw mut codeLo as *mut ::core::ffi::c_uchar,
                &raw mut codeHi as *mut ::core::ffi::c_uchar,
                srcdim as size_t,
                pst_data_ptr(tok) as *mut ::core::ffi::c_uchar,
                pst_length_of(tok) as size_t,
            );
        } else if pst_type_of(tok) == PST_TYPE_MARK {
            if handle_codearray(
                cmap,
                input,
                &raw mut codeLo as *mut ::core::ffi::c_uchar,
                srcdim,
                codeHi[(srcdim - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                    - codeLo[(srcdim - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                pst_release_obj(tok);
                return -(1 as ::core::ffi::c_int);
            }
        } else {
            pdf_warning!(
                b"%s: Invalid CMap mapping record. (ignored)\0" as *const u8
                    as *const ::core::ffi::c_char,
                CMAP_PARSE_DEBUG_STR.as_ptr(),
            );
        }
        pst_release_obj(tok);
    }
    return check_next_token(
        input,
        b"endbfrange\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn do_cidrange(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tok: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut codeLo: [::core::ffi::c_uchar; 127] = [0; 127];
    let mut codeHi: [::core::ffi::c_uchar; 127] = [0; 127];
    let mut dstCID: ::core::ffi::c_int = 0;
    let mut dim: ::core::ffi::c_int = 0;
    loop {
        let fresh1 = count;
        count = count - 1;
        if !(fresh1 > 0 as ::core::ffi::c_int) {
            break;
        }
        if ifreader_read(input, (TOKEN_LEN_MAX * 3 as ::core::ffi::c_int) as size_t) == 0 as size_t
        {
            return -(1 as ::core::ffi::c_int);
        }
        if get_coderange(
            input,
            &raw mut codeLo as *mut ::core::ffi::c_uchar,
            &raw mut codeHi as *mut ::core::ffi::c_uchar,
            &raw mut dim,
            TOKEN_LEN_MAX,
        ) < 0 as ::core::ffi::c_int
            || {
                tok = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
                tok.is_null()
            }
        {
            return -(1 as ::core::ffi::c_int);
        }
        if pst_type_of(tok) == PST_TYPE_INTEGER {
            dstCID = pst_getIV(tok);
            if dstCID >= 0 as ::core::ffi::c_int && dstCID <= CID_MAX {
                CMap_add_cidrange(
                    cmap,
                    &raw mut codeLo as *mut ::core::ffi::c_uchar,
                    &raw mut codeHi as *mut ::core::ffi::c_uchar,
                    dim as size_t,
                    dstCID as CID,
                );
            }
        } else {
            pdf_warning!(
                b"%s: Invalid CMap mapping record. (ignored)\0" as *const u8
                    as *const ::core::ffi::c_char,
                CMAP_PARSE_DEBUG_STR.as_ptr(),
            );
        }
        pst_release_obj(tok);
    }
    return check_next_token(
        input,
        b"endcidrange\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn do_notdefchar(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tok1: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut tok2: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut dstCID: ::core::ffi::c_int = 0;
    loop {
        let fresh6 = count;
        count = count - 1;
        if !(fresh6 > 0 as ::core::ffi::c_int) {
            break;
        }
        if ifreader_read(input, (TOKEN_LEN_MAX * 2 as ::core::ffi::c_int) as size_t) == 0 as size_t
        {
            return -(1 as ::core::ffi::c_int);
        }
        tok1 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
        if tok1.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        tok2 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
        if tok2.is_null() {
            pst_release_obj(tok1);
            return -(1 as ::core::ffi::c_int);
        }
        if pst_type_of(tok1) == PST_TYPE_STRING && pst_type_of(tok2) == PST_TYPE_INTEGER {
            dstCID = pst_getIV(tok2);
            if dstCID >= 0 as ::core::ffi::c_int && dstCID <= CID_MAX {
                CMap_add_notdefchar(
                    cmap,
                    pst_data_ptr(tok1) as *const ::core::ffi::c_uchar,
                    pst_length_of(tok1) as size_t,
                    dstCID as CID,
                );
            }
        } else {
            pdf_warning!(
                b"%s: Invalid CMap mapping record. (ignored)\0" as *const u8
                    as *const ::core::ffi::c_char,
                CMAP_PARSE_DEBUG_STR.as_ptr(),
            );
        }
        pst_release_obj(tok1);
        pst_release_obj(tok2);
    }
    return check_next_token(
        input,
        b"endnotdefchar\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn do_bfchar(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tok1: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut tok2: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    loop {
        let fresh2 = count;
        count = count - 1;
        if !(fresh2 > 0 as ::core::ffi::c_int) {
            break;
        }
        if ifreader_read(input, (TOKEN_LEN_MAX * 2 as ::core::ffi::c_int) as size_t) == 0 as size_t
        {
            return -(1 as ::core::ffi::c_int);
        }
        tok1 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
        if tok1.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        tok2 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
        if tok2.is_null() {
            pst_release_obj(tok1);
            return -(1 as ::core::ffi::c_int);
        }
        if pst_type_of(tok1) == PST_TYPE_STRING && pst_type_of(tok2) == PST_TYPE_STRING {
            CMap_add_bfchar(
                cmap,
                pst_data_ptr(tok1) as *mut ::core::ffi::c_uchar,
                pst_length_of(tok1) as size_t,
                pst_data_ptr(tok2) as *mut ::core::ffi::c_uchar,
                pst_length_of(tok2) as size_t,
            );
        } else if pst_type_of(tok2) == PST_TYPE_NAME {
            pdf_abort!(
                b"%s: Mapping to charName not supported.\0" as *const u8
                    as *const ::core::ffi::c_char,
                CMAP_PARSE_DEBUG_STR.as_ptr(),
            );
        } else {
            pdf_warning!(
                b"%s: Invalid CMap mapping record. (ignored)\0" as *const u8
                    as *const ::core::ffi::c_char,
                CMAP_PARSE_DEBUG_STR.as_ptr(),
            );
        }
        pst_release_obj(tok1);
        pst_release_obj(tok2);
    }
    return check_next_token(
        input,
        b"endbfchar\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn do_cidchar(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
    mut count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tok1: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut tok2: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut dstCID: ::core::ffi::c_int = 0;
    loop {
        let fresh0 = count;
        count = count - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        if ifreader_read(input, (TOKEN_LEN_MAX * 2 as ::core::ffi::c_int) as size_t) == 0 as size_t
        {
            return -(1 as ::core::ffi::c_int);
        }
        tok1 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
        if tok1.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        tok2 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
        if tok2.is_null() {
            pst_release_obj(tok1);
            return -(1 as ::core::ffi::c_int);
        }
        if pst_type_of(tok1) == PST_TYPE_STRING && pst_type_of(tok2) == PST_TYPE_INTEGER {
            dstCID = pst_getIV(tok2);
            if dstCID >= 0 as ::core::ffi::c_int && dstCID <= CID_MAX {
                CMap_add_cidchar(
                    cmap,
                    pst_data_ptr(tok1) as *const ::core::ffi::c_uchar,
                    pst_length_of(tok1) as size_t,
                    dstCID as CID,
                );
            }
        } else {
            pdf_warning!(
                b"%s: Invalid CMap mapping record. (ignored)\0" as *const u8
                    as *const ::core::ffi::c_char,
                CMAP_PARSE_DEBUG_STR.as_ptr(),
            );
        }
        pst_release_obj(tok1);
        pst_release_obj(tok2);
    }
    return check_next_token(
        input,
        b"endcidchar\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn do_cidsysteminfo(
    mut cmap: *mut CMap,
    mut input: *mut ifreader,
) -> ::core::ffi::c_int {
    let mut tok1: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut tok2: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut csi: CIDSysInfo = CIDSysInfo {
        registry: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ordering: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        supplement: -(1 as ::core::ffi::c_int),
    };
    let mut simpledict: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ifreader_read(input, (TOKEN_LEN_MAX * 2 as ::core::ffi::c_int) as size_t);
    loop {
        tok1 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
        if tok1.is_null() {
            break;
        }
        if pst_type_of(tok1) == PST_TYPE_MARK {
            simpledict = 1 as ::core::ffi::c_int;
            pst_release_obj(tok1);
            break;
        } else if pst_type_of(tok1) < 0 as ::core::ffi::c_int
            && pst_length_of(tok1) as size_t
                == strlen(b"begin\0" as *const u8 as *const ::core::ffi::c_char)
            && memcmp(
                pst_data_ptr(tok1),
                b"begin\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(b"begin\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            simpledict = 0 as ::core::ffi::c_int;
            pst_release_obj(tok1);
            break;
        } else {
            pst_release_obj(tok1);
        }
    }
    tok2 = ::core::ptr::null_mut::<pst_obj>();
    tok1 = tok2;
    while error == 0 && {
        tok1 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
        !tok1.is_null()
    } {
        if pst_type_of(tok1) < 0 as ::core::ffi::c_int
            && pst_length_of(tok1) as size_t
                == strlen(b">>\0" as *const u8 as *const ::core::ffi::c_char)
            && memcmp(
                pst_data_ptr(tok1),
                b">>\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(b">>\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
            && simpledict != 0
        {
            pst_release_obj(tok1);
            break;
        } else if pst_type_of(tok1) < 0 as ::core::ffi::c_int
            && pst_length_of(tok1) as size_t
                == strlen(b"end\0" as *const u8 as *const ::core::ffi::c_char)
            && memcmp(
                pst_data_ptr(tok1),
                b"end\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(b"end\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
            && simpledict == 0
        {
            pst_release_obj(tok1);
            break;
        } else {
            if pst_type_of(tok1) == PST_TYPE_NAME
                && pst_length_of(tok1) as size_t
                    == strlen(b"Registry\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"Registry\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"Registry\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
                && {
                    tok2 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
                    !tok2.is_null()
                }
            {
                if !(pst_type_of(tok2) == PST_TYPE_STRING) {
                    error = -(1 as ::core::ffi::c_int);
                } else if simpledict == 0
                    && check_next_token(input, b"def\0" as *const u8 as *const ::core::ffi::c_char)
                        != 0
                {
                    error = -(1 as ::core::ffi::c_int);
                }
                if error == 0 {
                    csi.registry = pst_getSV(tok2) as *mut ::core::ffi::c_char;
                }
            } else if pst_type_of(tok1) == PST_TYPE_NAME
                && pst_length_of(tok1) as size_t
                    == strlen(b"Ordering\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"Ordering\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"Ordering\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
                && {
                    tok2 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
                    !tok2.is_null()
                }
            {
                if !(pst_type_of(tok2) == PST_TYPE_STRING) {
                    error = -(1 as ::core::ffi::c_int);
                } else if simpledict == 0
                    && check_next_token(input, b"def\0" as *const u8 as *const ::core::ffi::c_char)
                        != 0
                {
                    error = -(1 as ::core::ffi::c_int);
                }
                if error == 0 {
                    csi.ordering = pst_getSV(tok2) as *mut ::core::ffi::c_char;
                }
            } else if pst_type_of(tok1) == PST_TYPE_NAME
                && pst_length_of(tok1) as size_t
                    == strlen(b"Supplement\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"Supplement\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"Supplement\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
                && {
                    tok2 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
                    !tok2.is_null()
                }
            {
                if !(pst_type_of(tok2) == PST_TYPE_INTEGER) {
                    error = -(1 as ::core::ffi::c_int);
                } else if simpledict == 0
                    && check_next_token(input, b"def\0" as *const u8 as *const ::core::ffi::c_char)
                        != 0
                {
                    error = -(1 as ::core::ffi::c_int);
                }
                if error == 0 {
                    csi.supplement = pst_getIV(tok2);
                }
            }
            if !tok2.is_null() {
                pst_release_obj(tok2);
            }
            if !tok1.is_null() {
                pst_release_obj(tok1);
            }
            tok2 = ::core::ptr::null_mut::<pst_obj>();
            tok1 = tok2;
        }
    }
    if error == 0
        && check_next_token(input, b"def\0" as *const u8 as *const ::core::ffi::c_char) != 0
    {
        error = -(1 as ::core::ffi::c_int);
    }
    if error == 0
        && !csi.registry.is_null()
        && !csi.ordering.is_null()
        && csi.supplement >= 0 as ::core::ffi::c_int
    {
        CMap_set_CIDSysInfo(cmap, &raw mut csi);
    }
    free(csi.registry as *mut ::core::ffi::c_void);
    free(csi.ordering as *mut ::core::ffi::c_void);
    return error;
}
pub const INPUT_BUF_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const CMAP_SIG_MAX: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_CMap_parse_check_sig")]
pub unsafe extern "C-unwind" fn CMap_parse_check_sig(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut sig: [::core::ffi::c_char; 65] = [0; 65];
    if handle == INVALID_HANDLE {
        return -(1 as ::core::ffi::c_int);
    }
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    if ttstub_input_read(
        handle,
        &raw mut sig as *mut ::core::ffi::c_char,
        CMAP_SIG_MAX as size_t,
    ) != CMAP_SIG_MAX as ssize_t
    {
        result = -(1 as ::core::ffi::c_int);
    } else {
        sig[CMAP_SIG_MAX as usize] = 0 as ::core::ffi::c_char;
        if strstartswith(
            &raw mut sig as *mut ::core::ffi::c_char,
            b"%!PS\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            result = -(1 as ::core::ffi::c_int);
        } else if !strstr(
            (&raw mut sig as *mut ::core::ffi::c_char).offset(4 as ::core::ffi::c_int as isize),
            b"Resource-CMap\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            result = 0 as ::core::ffi::c_int;
        }
    }
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    return result;
}
#[unsafe(export_name = "__pdf_impl_CMap_parse")]
pub unsafe extern "C-unwind" fn CMap_parse(
    mut cmap: *mut CMap,
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut tok1: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut tok2: *mut pst_obj = ::core::ptr::null_mut::<pst_obj>();
    let mut input: *mut ifreader = ::core::ptr::null_mut::<ifreader>();
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tmpint: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if !(!cmap.is_null() && handle != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap && handle\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    input = ifreader_create(
        handle,
        ttstub_input_get_size(handle),
        (INPUT_BUF_SIZE - 1 as ::core::ffi::c_int) as size_t,
    );
    while status >= 0 as ::core::ffi::c_int {
        tok2 = ::core::ptr::null_mut::<pst_obj>();
        tok1 = tok2;
        ifreader_read(input, (INPUT_BUF_SIZE / 2 as ::core::ffi::c_int) as size_t);
        tok1 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
        if tok1.is_null() {
            break;
        }
        if pst_type_of(tok1) == PST_TYPE_NAME
            && pst_length_of(tok1) as size_t
                == strlen(b"CMapName\0" as *const u8 as *const ::core::ffi::c_char)
            && memcmp(
                pst_data_ptr(tok1),
                b"CMapName\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                strlen(b"CMapName\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            tok2 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
            if tok2.is_null()
                || !(pst_type_of(tok2) == PST_TYPE_NAME || pst_type_of(tok2) == PST_TYPE_STRING)
                || check_next_token(input, b"def\0" as *const u8 as *const ::core::ffi::c_char)
                    < 0 as ::core::ffi::c_int
            {
                status = -(1 as ::core::ffi::c_int);
            } else {
                CMap_set_name(cmap, pst_data_ptr(tok2) as *const ::core::ffi::c_char);
            }
        } else if pst_type_of(tok1) == PST_TYPE_NAME
            && pst_length_of(tok1) as size_t
                == strlen(b"CMapType\0" as *const u8 as *const ::core::ffi::c_char)
            && memcmp(
                pst_data_ptr(tok1),
                b"CMapType\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                strlen(b"CMapType\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            tok2 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
            if tok2.is_null()
                || !(pst_type_of(tok2) == PST_TYPE_INTEGER)
                || check_next_token(input, b"def\0" as *const u8 as *const ::core::ffi::c_char)
                    < 0 as ::core::ffi::c_int
            {
                status = -(1 as ::core::ffi::c_int);
            } else {
                CMap_set_type(cmap, pst_getIV(tok2));
            }
        } else if pst_type_of(tok1) == PST_TYPE_NAME
            && pst_length_of(tok1) as size_t
                == strlen(b"WMode\0" as *const u8 as *const ::core::ffi::c_char)
            && memcmp(
                pst_data_ptr(tok1),
                b"WMode\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(b"WMode\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            tok2 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
            if tok2.is_null()
                || !(pst_type_of(tok2) == PST_TYPE_INTEGER)
                || check_next_token(input, b"def\0" as *const u8 as *const ::core::ffi::c_char)
                    < 0 as ::core::ffi::c_int
            {
                status = -(1 as ::core::ffi::c_int);
            } else {
                CMap_set_wmode(cmap, pst_getIV(tok2));
            }
        } else if pst_type_of(tok1) == PST_TYPE_NAME
            && pst_length_of(tok1) as size_t
                == strlen(b"CIDSystemInfo\0" as *const u8 as *const ::core::ffi::c_char)
            && memcmp(
                pst_data_ptr(tok1),
                b"CIDSystemInfo\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                strlen(b"CIDSystemInfo\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            status = do_cidsysteminfo(cmap, input);
        } else if !(pst_type_of(tok1) == PST_TYPE_NAME
            && pst_length_of(tok1) as size_t
                == strlen(b"Version\0" as *const u8 as *const ::core::ffi::c_char)
            && memcmp(
                pst_data_ptr(tok1),
                b"Version\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                strlen(b"Version\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
            || pst_type_of(tok1) == PST_TYPE_NAME
                && pst_length_of(tok1) as size_t
                    == strlen(b"UIDOffset\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"UIDOffset\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"UIDOffset\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            || pst_type_of(tok1) == PST_TYPE_NAME
                && pst_length_of(tok1) as size_t
                    == strlen(b"XUID\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"XUID\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"XUID\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0)
        {
            if pst_type_of(tok1) == PST_TYPE_NAME {
                tok2 = pst_get_token(&raw mut (*input).cursor, (*input).endptr);
                if !tok2.is_null()
                    && (pst_type_of(tok2) < 0 as ::core::ffi::c_int
                        && pst_length_of(tok2) as size_t
                            == strlen(b"usecmap\0" as *const u8 as *const ::core::ffi::c_char)
                        && memcmp(
                            pst_data_ptr(tok2),
                            b"usecmap\0" as *const u8 as *const ::core::ffi::c_char
                                as *const ::core::ffi::c_void,
                            strlen(b"usecmap\0" as *const u8 as *const ::core::ffi::c_char),
                        ) == 0)
                {
                    let mut id: ::core::ffi::c_int = 0;
                    let mut ucmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
                    id = CMap_cache_find(pst_data_ptr(tok1) as *const ::core::ffi::c_char);
                    if id < 0 as ::core::ffi::c_int {
                        status = -(1 as ::core::ffi::c_int);
                    } else {
                        ucmap = CMap_cache_get(id);
                        CMap_set_usecmap(cmap, ucmap);
                    }
                }
            } else if pst_type_of(tok1) < 0 as ::core::ffi::c_int
                && pst_length_of(tok1) as size_t
                    == strlen(b"begincodespacerange\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"begincodespacerange\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"begincodespacerange\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            {
                status = do_codespacerange(cmap, input, tmpint);
            } else if pst_type_of(tok1) < 0 as ::core::ffi::c_int
                && pst_length_of(tok1) as size_t
                    == strlen(b"beginnotdefrange\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginnotdefrange\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"beginnotdefrange\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            {
                status = do_notdefrange(cmap, input, tmpint);
            } else if pst_type_of(tok1) < 0 as ::core::ffi::c_int
                && pst_length_of(tok1) as size_t
                    == strlen(b"beginnotdefchar\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginnotdefchar\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"beginnotdefchar\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            {
                status = do_notdefchar(cmap, input, tmpint);
            } else if pst_type_of(tok1) < 0 as ::core::ffi::c_int
                && pst_length_of(tok1) as size_t
                    == strlen(b"beginbfrange\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginbfrange\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"beginbfrange\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            {
                status = do_bfrange(cmap, input, tmpint);
            } else if pst_type_of(tok1) < 0 as ::core::ffi::c_int
                && pst_length_of(tok1) as size_t
                    == strlen(b"beginbfchar\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"beginbfchar\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"beginbfchar\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            {
                status = do_bfchar(cmap, input, tmpint);
            } else if pst_type_of(tok1) < 0 as ::core::ffi::c_int
                && pst_length_of(tok1) as size_t
                    == strlen(b"begincidrange\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"begincidrange\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"begincidrange\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            {
                status = do_cidrange(cmap, input, tmpint);
            } else if pst_type_of(tok1) < 0 as ::core::ffi::c_int
                && pst_length_of(tok1) as size_t
                    == strlen(b"begincidchar\0" as *const u8 as *const ::core::ffi::c_char)
                && memcmp(
                    pst_data_ptr(tok1),
                    b"begincidchar\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    strlen(b"begincidchar\0" as *const u8 as *const ::core::ffi::c_char),
                ) == 0
            {
                status = do_cidchar(cmap, input, tmpint);
            } else if pst_type_of(tok1) == PST_TYPE_INTEGER {
                tmpint = pst_getIV(tok1);
            }
        }
        if !tok1.is_null() {
            pst_release_obj(tok1);
        }
        if !tok2.is_null() {
            pst_release_obj(tok2);
        }
    }
    ifreader_destroy(input);
    return if status < 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else {
        CMap_is_valid(cmap) as ::core::ffi::c_int
    };
}
unsafe extern "C" {
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pst_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_parse_check_sig(
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_parse_check_sig(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_parse(
    mut cmap: *mut CMap,
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_parse(cmap, handle) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
