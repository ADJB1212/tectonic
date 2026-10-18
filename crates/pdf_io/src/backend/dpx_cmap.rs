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
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_open(
        path: *const ::core::ffi::c_char,
        format: ttbc_file_format,
        is_gz: ::core::ffi::c_int,
    ) -> rust_input_handle_t;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    static mut CSI_IDENTITY: CIDSysInfo;
    static mut dpx_conf: _dpx_conf;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_CMap_parse_check_sig"]
    fn CMap_parse_check_sig(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_parse"]
    fn CMap_parse(cmap: *mut CMap, handle: rust_input_handle_t) -> ::core::ffi::c_int;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
pub type uintptr_t = usize;
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
pub struct CMap_cache {
    pub num: ::core::ffi::c_int,
    pub max: ::core::ffi::c_int,
    pub cmaps: *mut *mut CMap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub keep_cache: ::core::ffi::c_int,
}
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
#[inline]
unsafe extern "C-unwind" fn streq_ptr(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
    }
    return false_0 != 0;
}
pub const MAP_IS_CID: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const MAP_IS_NAME: ::core::ffi::c_int = 2;
pub const MAP_IS_CODE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const MAP_IS_NOTDEF: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const MAP_IS_UNDEF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAP_TYPE_MASK: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;
pub const MAP_LOOKUP_END: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAP_LOOKUP_CONTINUE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const CMAP_DEBUG_STR: [::core::ffi::c_char; 5] = crate::backend::c_char_array(*b"CMap\0");
pub const MEM_ALLOC_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const CID_MAX_CID: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const CID_MAX: ::core::ffi::c_int = CID_MAX_CID;
pub const CID_NOTDEF_CHAR: [::core::ffi::c_char; 3] = crate::backend::c_char_array(*b"\0\0\0");
pub const UCS_NOTDEF_CHAR: [::core::ffi::c_char; 3] = crate::backend::c_char_array(*b"\xFF\xFD\0");
pub const CMAP_TYPE_IDENTITY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CMAP_TYPE_CODE_TO_CID: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CMAP_TYPE_TO_UNICODE: ::core::ffi::c_int = 2;
pub const CMAP_TYPE_CID_TO_CODE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const CMAP_PROF_TYPE_INBYTES_MIN: ::core::ffi::c_int = 0;
pub const CMAP_PROF_TYPE_INBYTES_MAX: ::core::ffi::c_int = 1;
pub const CMAP_PROF_TYPE_OUTBYTES_MIN: ::core::ffi::c_int = 2;
pub const CMAP_PROF_TYPE_OUTBYTES_MAX: ::core::ffi::c_int = 3;

#[unsafe(export_name = "__pdf_impl_CMap_set_silent")]
pub unsafe extern "C-unwind" fn CMap_set_silent(mut value: ::core::ffi::c_int) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).__silent = if value != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(export_name = "__pdf_impl_CMap_new")]
pub unsafe extern "C-unwind" fn CMap_new() -> *mut CMap {
    let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
    cmap = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<CMap>() as usize) as uint32_t) as *mut CMap
        as *mut CMap;
    (*cmap).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*cmap).type_0 = CMAP_TYPE_CODE_TO_CID;
    (*cmap).wmode = 0 as ::core::ffi::c_int;
    (*cmap).useCMap = ::core::ptr::null_mut::<CMap>();
    (*cmap).CSI = ::core::ptr::null_mut::<CIDSysInfo>();
    (*cmap).profile.minBytesIn = 65535 as size_t;
    (*cmap).profile.maxBytesIn = 0 as size_t;
    (*cmap).profile.minBytesOut = 65535 as size_t;
    (*cmap).profile.maxBytesOut = 0 as size_t;
    (*cmap).flags = 0 as ::core::ffi::c_int;
    (*cmap).codespace.num = 0 as ::core::ffi::c_uint;
    (*cmap).codespace.max = 10 as ::core::ffi::c_uint;
    (*cmap).codespace.ranges = new((10 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<rangeDef>() as usize)
        as uint32_t) as *mut rangeDef as *mut rangeDef;
    (*cmap).mapTbl = ::core::ptr::null_mut::<mapDef>();
    (*cmap).mapData = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<mapData>() as usize)
        as uint32_t) as *mut mapData as *mut mapData;
    (*(*cmap).mapData).prev = ::core::ptr::null_mut::<mapData>();
    (*(*cmap).mapData).pos = 0 as ::core::ffi::c_int;
    (*(*cmap).mapData).data = new((4096 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    return cmap;
}
#[unsafe(export_name = "__pdf_impl_CMap_release")]
pub unsafe extern "C-unwind" fn CMap_release(mut cmap: *mut CMap) {
    if cmap.is_null() {
        return;
    }
    free((*cmap).name as *mut ::core::ffi::c_void);
    if !(*cmap).CSI.is_null() {
        free((*(*cmap).CSI).registry as *mut ::core::ffi::c_void);
        free((*(*cmap).CSI).ordering as *mut ::core::ffi::c_void);
        free((*cmap).CSI as *mut ::core::ffi::c_void);
    }
    free((*cmap).codespace.ranges as *mut ::core::ffi::c_void);
    if !(*cmap).mapTbl.is_null() {
        mapDef_release((*cmap).mapTbl);
    }
    let mut map: *mut mapData = (*cmap).mapData;
    while !map.is_null() {
        let mut prev: *mut mapData = (*map).prev as *mut mapData;
        free((*map).data as *mut ::core::ffi::c_void);
        free(map as *mut ::core::ffi::c_void);
        map = prev;
    }
    free(cmap as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_CMap_is_Identity")]
pub unsafe extern "C-unwind" fn CMap_is_Identity(mut cmap: *mut CMap) -> bool {
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return streq_ptr(
        (*cmap).name,
        b"Identity-H\0" as *const u8 as *const ::core::ffi::c_char,
    ) as ::core::ffi::c_int
        != 0
        || streq_ptr(
            (*cmap).name,
            b"Identity-V\0" as *const u8 as *const ::core::ffi::c_char,
        ) as ::core::ffi::c_int
            != 0;
}
#[unsafe(export_name = "__pdf_impl_CMap_is_valid")]
pub unsafe extern "C-unwind" fn CMap_is_valid(mut cmap: *mut CMap) -> bool {
    if cmap.is_null()
        || (*cmap).name.is_null()
        || (*cmap).type_0 < CMAP_TYPE_IDENTITY
        || (*cmap).type_0 > CMAP_TYPE_CID_TO_CODE
        || (*cmap).codespace.num < 1 as ::core::ffi::c_uint
        || (*cmap).type_0 != CMAP_TYPE_IDENTITY && (*cmap).mapTbl.is_null()
    {
        return false_0 != 0;
    }
    if !(*cmap).useCMap.is_null() {
        let mut csi1: *mut CIDSysInfo = ::core::ptr::null_mut::<CIDSysInfo>();
        let mut csi2: *mut CIDSysInfo = ::core::ptr::null_mut::<CIDSysInfo>();
        csi1 = CMap_get_CIDSysInfo(cmap);
        csi2 = CMap_get_CIDSysInfo((*cmap).useCMap as *mut CMap);
        if strcmp((*csi1).registry, (*csi2).registry) != 0
            || strcmp((*csi1).ordering, (*csi2).ordering) != 0
        {
            pdf_warning!(
                b"CIDSystemInfo mismatched %s <--> %s\0" as *const u8 as *const ::core::ffi::c_char,
                CMap_get_name(cmap),
                CMap_get_name((*cmap).useCMap as *mut CMap),
            );
            return false_0 != 0;
        }
    }
    return true_0 != 0;
}
#[unsafe(export_name = "__pdf_impl_CMap_get_profile")]
pub unsafe extern "C-unwind" fn CMap_get_profile(
    mut cmap: *mut CMap,
    mut type_0: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match type_0 {
        CMAP_PROF_TYPE_INBYTES_MIN => {
            value = (*cmap).profile.minBytesIn as ::core::ffi::c_int;
        }
        CMAP_PROF_TYPE_INBYTES_MAX => {
            value = (*cmap).profile.maxBytesIn as ::core::ffi::c_int;
        }
        CMAP_PROF_TYPE_OUTBYTES_MIN => {
            value = (*cmap).profile.maxBytesOut as ::core::ffi::c_int;
        }
        CMAP_PROF_TYPE_OUTBYTES_MAX => {
            value = (*cmap).profile.maxBytesOut as ::core::ffi::c_int;
        }
        _ => {
            pdf_abort!(
                b"%s: Unrecognized profile type %d.\0" as *const u8 as *const ::core::ffi::c_char,
                CMAP_DEBUG_STR.as_ptr(),
                type_0,
            );
        }
    }
    return value;
}
unsafe extern "C-unwind" fn handle_undefined(
    mut cmap: *mut CMap,
    mut inbuf: *mut *const ::core::ffi::c_uchar,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut ::core::ffi::c_uchar,
    mut outbytesleft: *mut size_t,
) {
    let mut len: size_t = 0 as size_t;
    if *outbytesleft < 2 as size_t {
        pdf_abort!(
            b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
            CMAP_DEBUG_STR.as_ptr(),
        );
    }
    match (*cmap).type_0 {
        CMAP_TYPE_CODE_TO_CID => {
            memcpy(
                *outbuf as *mut ::core::ffi::c_void,
                CID_NOTDEF_CHAR.as_ptr() as *const ::core::ffi::c_void,
                2 as size_t,
            );
        }
        CMAP_TYPE_TO_UNICODE => {
            memcpy(
                *outbuf as *mut ::core::ffi::c_void,
                UCS_NOTDEF_CHAR.as_ptr() as *const ::core::ffi::c_void,
                2 as size_t,
            );
        }
        _ => {
            pdf_warning!(
                b"Cannot handle undefined mapping for this type of CMap mapping: %d\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*cmap).type_0,
            );
            pdf_warning!(
                b"<0000> is used for .notdef char.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            memset(
                *outbuf as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                2 as size_t,
            );
        }
    }
    *outbuf = (*outbuf).offset(2 as ::core::ffi::c_int as isize);
    *outbytesleft = (*outbytesleft).wrapping_sub(2 as size_t);
    len = bytes_consumed(cmap, *inbuf, *inbytesleft);
    *inbuf = (*inbuf).offset(len as isize);
    *inbytesleft = (*inbytesleft).wrapping_sub(len);
}
#[unsafe(export_name = "__pdf_impl_CMap_decode_char")]
pub unsafe extern "C-unwind" fn CMap_decode_char(
    mut cmap: *mut CMap,
    mut inbuf: *mut *const ::core::ffi::c_uchar,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut ::core::ffi::c_uchar,
    mut outbytesleft: *mut size_t,
) {
    let mut t: *mut mapDef = ::core::ptr::null_mut::<mapDef>();
    let mut p: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut save: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut c: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut count: size_t = 0 as size_t;
    save = *inbuf;
    p = save;
    if (*cmap).type_0 == CMAP_TYPE_IDENTITY {
        if (*inbytesleft).wrapping_rem(2 as size_t) != 0 {
            pdf_abort!(
                b"%s: Invalid/truncated input string.\0" as *const u8 as *const ::core::ffi::c_char,
                CMAP_DEBUG_STR.as_ptr(),
            );
        }
        if *outbytesleft < 2 as size_t {
            pdf_abort!(
                b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                CMAP_DEBUG_STR.as_ptr(),
            );
        }
        memcpy(
            *outbuf as *mut ::core::ffi::c_void,
            *inbuf as *const ::core::ffi::c_void,
            2 as size_t,
        );
        *inbuf = (*inbuf).offset(2 as ::core::ffi::c_int as isize);
        *outbuf = (*outbuf).offset(2 as ::core::ffi::c_int as isize);
        *outbytesleft = (*outbytesleft).wrapping_sub(2 as size_t);
        *inbytesleft = (*inbytesleft).wrapping_sub(2 as size_t);
        return;
    } else if (*cmap).mapTbl.is_null() {
        if !(*cmap).useCMap.is_null() {
            CMap_decode_char(
                (*cmap).useCMap as *mut CMap,
                inbuf,
                inbytesleft,
                outbuf,
                outbytesleft,
            );
            return;
        } else {
            pdf_warning!(
                b"No mapping available for this character.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            handle_undefined(cmap, inbuf, inbytesleft, outbuf, outbytesleft);
            return;
        }
    }
    if (*cmap).mapTbl.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap->mapTbl\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    t = (*cmap).mapTbl;
    while count < *inbytesleft {
        let fresh6 = p;
        p = p.offset(1);
        c = *fresh6;
        count = count.wrapping_add(1);
        if (*t.offset(c as isize)).flag & MAP_LOOKUP_CONTINUE == 0 {
            break;
        }
        t = (*t.offset(c as isize)).next as *mut mapDef;
    }
    if (*t.offset(c as isize)).flag & MAP_LOOKUP_CONTINUE != 0 {
        pdf_abort!(
            b"%s: Premature end of input string.\0" as *const u8 as *const ::core::ffi::c_char,
            CMAP_DEBUG_STR.as_ptr(),
        );
    } else if if (*t.offset(c as isize)).flag & MAP_TYPE_MASK != MAP_IS_UNDEF {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    } == 0
    {
        if !(*cmap).useCMap.is_null() {
            CMap_decode_char(
                (*cmap).useCMap as *mut CMap,
                inbuf,
                inbytesleft,
                outbuf,
                outbytesleft,
            );
            return;
        } else {
            pdf_warning!(
                b"No character mapping available.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            pdf_message!(
                b" CMap name: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                CMap_get_name(cmap),
            );
            pdf_message!(b" input str: \0" as *const u8 as *const ::core::ffi::c_char);
            pdf_message!(b"<\0" as *const u8 as *const ::core::ffi::c_char);
            while save < p {
                pdf_message!(
                    b"%02x\0" as *const u8 as *const ::core::ffi::c_char,
                    *save as ::core::ffi::c_int,
                );
                save = save.offset(1);
            }
            pdf_message!(b">\n\0" as *const u8 as *const ::core::ffi::c_char);
            handle_undefined(cmap, inbuf, inbytesleft, outbuf, outbytesleft);
            return;
        }
    } else {
        match (*t.offset(c as isize)).flag & MAP_TYPE_MASK {
            MAP_IS_NOTDEF => {
                pdf_warning!(
                    b"Character mapped to .notdef found.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            MAP_IS_CID | MAP_IS_CODE => {}
            MAP_IS_NAME => {
                pdf_abort!(
                    b"%s: CharName mapping not supported.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    CMAP_DEBUG_STR.as_ptr(),
                );
            }
            _ => {
                pdf_abort!(
                    b"%s: Unknown mapping type.\0" as *const u8 as *const ::core::ffi::c_char,
                    CMAP_DEBUG_STR.as_ptr(),
                );
            }
        }
        if *outbytesleft >= (*t.offset(c as isize)).len {
            memcpy(
                *outbuf as *mut ::core::ffi::c_void,
                (*t.offset(c as isize)).code as *const ::core::ffi::c_void,
                (*t.offset(c as isize)).len,
            );
        } else {
            pdf_abort!(
                b"%s: Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                CMAP_DEBUG_STR.as_ptr(),
            );
        }
        *outbuf = (*outbuf).offset((*t.offset(c as isize)).len as isize);
        *outbytesleft = (*outbytesleft).wrapping_sub((*t.offset(c as isize)).len);
        if !inbytesleft.is_null() {
            *inbytesleft = (*inbytesleft).wrapping_sub(count);
        }
        *inbuf = p;
    };
}
#[unsafe(export_name = "__pdf_impl_CMap_decode")]
pub unsafe extern "C-unwind" fn CMap_decode(
    mut cmap: *mut CMap,
    mut inbuf: *mut *const ::core::ffi::c_uchar,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut ::core::ffi::c_uchar,
    mut outbytesleft: *mut size_t,
) -> size_t {
    let mut count: size_t = 0;
    if !(!cmap.is_null() && !inbuf.is_null() && !outbuf.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap && inbuf && outbuf\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(!inbytesleft.is_null() && !outbytesleft.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"inbytesleft && outbytesleft\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    count = 0 as size_t;
    while *inbytesleft > 0 as size_t && *outbytesleft > 0 as size_t {
        CMap_decode_char(cmap, inbuf, inbytesleft, outbuf, outbytesleft);
        count = count.wrapping_add(1);
    }
    return count;
}
#[unsafe(export_name = "__pdf_impl_CMap_get_name")]
pub unsafe extern "C-unwind" fn CMap_get_name(mut cmap: *mut CMap) -> *mut ::core::ffi::c_char {
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*cmap).name;
}
#[unsafe(export_name = "__pdf_impl_CMap_get_type")]
pub unsafe extern "C-unwind" fn CMap_get_type(mut cmap: *mut CMap) -> ::core::ffi::c_int {
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*cmap).type_0;
}
#[unsafe(export_name = "__pdf_impl_CMap_get_wmode")]
pub unsafe extern "C-unwind" fn CMap_get_wmode(mut cmap: *mut CMap) -> ::core::ffi::c_int {
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*cmap).wmode;
}
#[unsafe(export_name = "__pdf_impl_CMap_get_CIDSysInfo")]
pub unsafe extern "C-unwind" fn CMap_get_CIDSysInfo(mut cmap: *mut CMap) -> *mut CIDSysInfo {
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*cmap).CSI;
}
#[unsafe(export_name = "__pdf_impl_CMap_set_name")]
pub unsafe extern "C-unwind" fn CMap_set_name(
    mut cmap: *mut CMap,
    mut name: *const ::core::ffi::c_char,
) {
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    free((*cmap).name as *mut ::core::ffi::c_void);
    (*cmap).name = new(
        (strlen(name).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*cmap).name, name);
}
#[unsafe(export_name = "__pdf_impl_CMap_set_type")]
pub unsafe extern "C-unwind" fn CMap_set_type(mut cmap: *mut CMap, mut type_0: ::core::ffi::c_int) {
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*cmap).type_0 = type_0;
}
#[unsafe(export_name = "__pdf_impl_CMap_set_wmode")]
pub unsafe extern "C-unwind" fn CMap_set_wmode(mut cmap: *mut CMap, mut wmode: ::core::ffi::c_int) {
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*cmap).wmode = wmode;
}
#[unsafe(export_name = "__pdf_impl_CMap_set_CIDSysInfo")]
pub unsafe extern "C-unwind" fn CMap_set_CIDSysInfo(
    mut cmap: *mut CMap,
    mut csi: *const CIDSysInfo,
) {
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(*cmap).CSI.is_null() {
        free((*(*cmap).CSI).registry as *mut ::core::ffi::c_void);
        free((*(*cmap).CSI).ordering as *mut ::core::ffi::c_void);
        free((*cmap).CSI as *mut ::core::ffi::c_void);
    }
    if !csi.is_null() && !(*csi).registry.is_null() && !(*csi).ordering.is_null() {
        (*cmap).CSI = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<CIDSysInfo>() as usize)
            as uint32_t) as *mut CIDSysInfo;
        (*(*cmap).CSI).registry = new(
            (strlen((*csi).registry).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy((*(*cmap).CSI).registry, (*csi).registry);
        (*(*cmap).CSI).ordering = new(
            (strlen((*csi).ordering).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_char;
        strcpy((*(*cmap).CSI).ordering, (*csi).ordering);
        (*(*cmap).CSI).supplement = (*csi).supplement;
    } else {
        pdf_warning!(b"Invalid CIDSystemInfo.\0" as *const u8 as *const ::core::ffi::c_char,);
        (*cmap).CSI = ::core::ptr::null_mut::<CIDSysInfo>();
    };
}
#[unsafe(export_name = "__pdf_impl_CMap_set_usecmap")]
pub unsafe extern "C-unwind" fn CMap_set_usecmap(mut cmap: *mut CMap, mut ucmap: *mut CMap) {
    let mut i: ::core::ffi::c_uint = 0;
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if ucmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ucmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if cmap == ucmap {
        pdf_abort!(
            b"%s: Identical CMap object cannot be used for usecmap CMap: 0x%p=0x%p\0" as *const u8
                as *const ::core::ffi::c_char,
            CMAP_DEBUG_STR.as_ptr(),
            cmap,
            ucmap,
        );
    }
    if !CMap_is_valid(ucmap) {
        pdf_abort!(
            b"%s: Invalid CMap.\0" as *const u8 as *const ::core::ffi::c_char,
            CMAP_DEBUG_STR.as_ptr(),
        );
    }
    if streq_ptr((*cmap).name, (*ucmap).name) {
        pdf_abort!(
            b"%s: CMap refering itself not allowed: CMap %s --> %s\0" as *const u8
                as *const ::core::ffi::c_char,
            CMAP_DEBUG_STR.as_ptr(),
            (*cmap).name,
            (*ucmap).name,
        );
    }
    if !(*cmap).CSI.is_null()
        && !(*(*cmap).CSI).registry.is_null()
        && !(*(*cmap).CSI).ordering.is_null()
    {
        if strcmp((*(*cmap).CSI).registry, (*(*ucmap).CSI).registry) != 0
            || strcmp((*(*cmap).CSI).ordering, (*(*ucmap).CSI).ordering) != 0
        {
            pdf_abort!(
                b"%s: CMap %s required by %s have different CSI.\0" as *const u8
                    as *const ::core::ffi::c_char,
                CMAP_DEBUG_STR.as_ptr(),
                CMap_get_name(cmap),
                CMap_get_name(ucmap),
            );
        }
    }
    i = 0 as ::core::ffi::c_uint;
    while i < (*ucmap).codespace.num {
        let mut csr: *mut rangeDef = (*ucmap).codespace.ranges.offset(i as isize);
        CMap_add_codespacerange(cmap, (*csr).codeLo, (*csr).codeHi, (*csr).dim);
        i = i.wrapping_add(1);
    }
    (*cmap).useCMap = ucmap as *mut CMap;
}
unsafe extern "C-unwind" fn CMap_match_codespace(
    mut cmap: *mut CMap,
    mut c: *const ::core::ffi::c_uchar,
    mut dim: size_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_uint = 0;
    let mut pos: ::core::ffi::c_uint = 0;
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    i = 0 as ::core::ffi::c_uint;
    while i < (*cmap).codespace.num {
        let mut csr: *mut rangeDef = (*cmap).codespace.ranges.offset(i as isize);
        if !((*csr).dim != dim) {
            pos = 0 as ::core::ffi::c_uint;
            while (pos as size_t) < dim {
                if *c.offset(pos as isize) as ::core::ffi::c_int
                    > *(*csr).codeHi.offset(pos as isize) as ::core::ffi::c_int
                    || (*c.offset(pos as isize) as ::core::ffi::c_int)
                        < *(*csr).codeLo.offset(pos as isize) as ::core::ffi::c_int
                {
                    break;
                }
                pos = pos.wrapping_add(1);
            }
            if pos as size_t == dim {
                return 0 as ::core::ffi::c_int;
            }
        }
        i = i.wrapping_add(1);
    }
    return -(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_CMap_add_codespacerange")]
pub unsafe extern "C-unwind" fn CMap_add_codespacerange(
    mut cmap: *mut CMap,
    mut codelo: *const ::core::ffi::c_uchar,
    mut codehi: *const ::core::ffi::c_uchar,
    mut dim: size_t,
) -> ::core::ffi::c_int {
    let mut csr: *mut rangeDef = ::core::ptr::null_mut::<rangeDef>();
    let mut i: ::core::ffi::c_uint = 0;
    if !(!cmap.is_null() && dim > 0 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap && dim > 0\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    i = 0 as ::core::ffi::c_uint;
    while i < (*cmap).codespace.num {
        let mut j: size_t = 0;
        let mut overlap: bool = true_0 != 0;
        csr = (*cmap).codespace.ranges.offset(i as isize);
        j = 0 as size_t;
        while j < (if (*csr).dim < dim { (*csr).dim } else { dim })
            && overlap as ::core::ffi::c_int != 0
        {
            if *codelo.offset(j as isize) as ::core::ffi::c_int
                >= *(*csr).codeLo.offset(j as isize) as ::core::ffi::c_int
                && *codelo.offset(j as isize) as ::core::ffi::c_int
                    <= *(*csr).codeHi.offset(j as isize) as ::core::ffi::c_int
                || *codehi.offset(j as isize) as ::core::ffi::c_int
                    >= *(*csr).codeLo.offset(j as isize) as ::core::ffi::c_int
                    && *codehi.offset(j as isize) as ::core::ffi::c_int
                        <= *(*csr).codeHi.offset(j as isize) as ::core::ffi::c_int
            {
                overlap = true_0 != 0;
            } else {
                overlap = false_0 != 0;
            }
            j = j.wrapping_add(1);
        }
        if overlap {
            pdf_warning!(
                b"Overlapping codespace found. (ingored)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        i = i.wrapping_add(1);
    }
    if dim < (*cmap).profile.minBytesIn {
        (*cmap).profile.minBytesIn = dim;
    }
    if dim > (*cmap).profile.maxBytesIn {
        (*cmap).profile.maxBytesIn = dim;
    }
    if (*cmap).codespace.num.wrapping_add(1 as ::core::ffi::c_uint) > (*cmap).codespace.max {
        (*cmap).codespace.max = crate::compat::allocation::grow_capacity(
            (*cmap).codespace.max as usize,
            (*cmap).codespace.num as usize + 1,
            core::mem::size_of::<rangeDef>(),
            10,
        ) as u32;
        (*cmap).codespace.ranges = renew(
            (*cmap).codespace.ranges as *mut ::core::ffi::c_void,
            ((*cmap).codespace.max as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<rangeDef>() as usize) as uint32_t,
        ) as *mut rangeDef as *mut rangeDef;
    }
    csr = (*cmap)
        .codespace
        .ranges
        .offset((*cmap).codespace.num as isize);
    (*csr).dim = dim;
    (*csr).codeHi = get_mem(cmap, dim as ::core::ffi::c_int);
    (*csr).codeLo = get_mem(cmap, dim as ::core::ffi::c_int);
    memcpy(
        (*csr).codeHi as *mut ::core::ffi::c_void,
        codehi as *const ::core::ffi::c_void,
        dim,
    );
    memcpy(
        (*csr).codeLo as *mut ::core::ffi::c_void,
        codelo as *const ::core::ffi::c_void,
        dim,
    );
    (*cmap).codespace.num = (*cmap).codespace.num.wrapping_add(1);
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_CMap_add_notdefchar")]
pub unsafe extern "C-unwind" fn CMap_add_notdefchar(
    mut cmap: *mut CMap,
    mut src: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut dst: CID,
) -> ::core::ffi::c_int {
    return CMap_add_notdefrange(cmap, src, src, srcdim, dst);
}
#[unsafe(export_name = "__pdf_impl_CMap_add_notdefrange")]
pub unsafe extern "C-unwind" fn CMap_add_notdefrange(
    mut cmap: *mut CMap,
    mut srclo: *const ::core::ffi::c_uchar,
    mut srchi: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut dst: CID,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut c: ::core::ffi::c_int = 0;
    let mut cur: *mut mapDef = ::core::ptr::null_mut::<mapDef>();
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if check_range(
        cmap,
        srclo,
        srchi,
        srcdim,
        &raw mut dst as *const ::core::ffi::c_uchar,
        2 as size_t,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if (*cmap).mapTbl.is_null() {
        (*cmap).mapTbl = mapDef_new();
    }
    cur = (*cmap).mapTbl;
    if locate_tbl(&raw mut cur, srclo, srcdim as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    c = *srclo.offset(srcdim.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int;
    while c <= *srchi.offset(srcdim.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int {
        if if (*cur.offset(c as isize)).flag & MAP_TYPE_MASK != MAP_IS_UNDEF {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } != 0
        {
            if (*__pdf_state).__silent == 0 {
                pdf_warning!(
                    b"Trying to redefine already defined code mapping. (ignored)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        } else {
            (*cur.offset(c as isize)).flag = MAP_LOOKUP_END | MAP_IS_NOTDEF;
            let ref mut fresh5 = (*cur.offset(c as isize)).code;
            *fresh5 = get_mem(cmap, 2 as ::core::ffi::c_int);
            (*cur.offset(c as isize)).len = 2 as size_t;
            *(*cur.offset(c as isize))
                .code
                .offset(0 as ::core::ffi::c_int as isize) =
                (dst as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *(*cur.offset(c as isize))
                .code
                .offset(1 as ::core::ffi::c_int as isize) =
                (dst as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        }
        c += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_CMap_add_bfchar")]
pub unsafe extern "C-unwind" fn CMap_add_bfchar(
    mut cmap: *mut CMap,
    mut src: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut dst: *const ::core::ffi::c_uchar,
    mut dstdim: size_t,
) -> ::core::ffi::c_int {
    return CMap_add_bfrange(cmap, src, src, srcdim, dst, dstdim);
}
#[unsafe(export_name = "__pdf_impl_CMap_add_bfrange")]
pub unsafe extern "C-unwind" fn CMap_add_bfrange(
    mut cmap: *mut CMap,
    mut srclo: *const ::core::ffi::c_uchar,
    mut srchi: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut base: *const ::core::ffi::c_uchar,
    mut dstdim: size_t,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut last_byte: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut cur: *mut mapDef = ::core::ptr::null_mut::<mapDef>();
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if check_range(cmap, srclo, srchi, srcdim, base, dstdim) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if (*cmap).mapTbl.is_null() {
        (*cmap).mapTbl = mapDef_new();
    }
    cur = (*cmap).mapTbl;
    if locate_tbl(&raw mut cur, srclo, srcdim as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    c = *srclo.offset(srcdim.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int;
    while c <= *srchi.offset(srcdim.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int {
        if (if (*cur.offset(c as isize)).flag & MAP_TYPE_MASK != MAP_IS_UNDEF {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) == 0
            || (*cur.offset(c as isize)).len < dstdim
        {
            (*cur.offset(c as isize)).flag = MAP_LOOKUP_END | MAP_IS_CODE;
            let ref mut fresh0 = (*cur.offset(c as isize)).code;
            *fresh0 = get_mem(cmap, dstdim as ::core::ffi::c_int);
        }
        (*cur.offset(c as isize)).len = dstdim;
        memcpy(
            (*cur.offset(c as isize)).code as *mut ::core::ffi::c_void,
            base as *const ::core::ffi::c_void,
            dstdim,
        );
        last_byte = c - *srclo.offset(srcdim.wrapping_sub(1 as size_t) as isize)
            as ::core::ffi::c_int
            + *base.offset(dstdim.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int;
        *(*cur.offset(c as isize))
            .code
            .offset(dstdim.wrapping_sub(1 as size_t) as isize) =
            (last_byte & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        i = dstdim.wrapping_sub(2 as size_t) as ::core::ffi::c_int;
        while i >= 0 as ::core::ffi::c_int && last_byte > 255 as ::core::ffi::c_int {
            last_byte = *(*cur.offset(c as isize)).code.offset(i as isize) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int;
            *(*cur.offset(c as isize)).code.offset(i as isize) =
                (last_byte & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            i -= 1;
        }
        c += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_CMap_add_cidchar")]
pub unsafe extern "C-unwind" fn CMap_add_cidchar(
    mut cmap: *mut CMap,
    mut src: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut dst: CID,
) -> ::core::ffi::c_int {
    return CMap_add_cidrange(cmap, src, src, srcdim, dst);
}
#[unsafe(export_name = "__pdf_impl_CMap_add_cidrange")]
pub unsafe extern "C-unwind" fn CMap_add_cidrange(
    mut cmap: *mut CMap,
    mut srclo: *const ::core::ffi::c_uchar,
    mut srchi: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut base: CID,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut i: size_t = 0;
    let mut c: size_t = 0;
    let mut v: size_t = 0;
    let mut cur: *mut mapDef = ::core::ptr::null_mut::<mapDef>();
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if check_range(
        cmap,
        srclo,
        srchi,
        srcdim,
        &raw mut base as *const ::core::ffi::c_uchar,
        2 as size_t,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    if (*cmap).mapTbl.is_null() {
        (*cmap).mapTbl = mapDef_new();
    }
    cur = (*cmap).mapTbl;
    if locate_tbl(&raw mut cur, srclo, srcdim as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    v = 0 as size_t;
    i = 0 as size_t;
    while i < srcdim.wrapping_sub(1 as size_t) {
        v = (v << 8 as ::core::ffi::c_int).wrapping_add(*srclo.offset(i as isize) as size_t);
        i = i.wrapping_add(1);
    }
    c = *srclo.offset(srcdim.wrapping_sub(1 as size_t) as isize) as size_t;
    while c <= *srchi.offset(srcdim.wrapping_sub(1 as size_t) as isize) as size_t {
        if (*cur.offset(c as isize)).flag != 0 as ::core::ffi::c_int {
            if (*__pdf_state).__silent == 0 {
                pdf_warning!(
                    b"Trying to redefine already defined CID mapping. (ignored)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        } else {
            (*cur.offset(c as isize)).flag = MAP_LOOKUP_END | MAP_IS_CID;
            (*cur.offset(c as isize)).len = 2 as size_t;
            let ref mut fresh4 = (*cur.offset(c as isize)).code;
            *fresh4 = get_mem(cmap, 2 as ::core::ffi::c_int);
            *(*cur.offset(c as isize))
                .code
                .offset(0 as ::core::ffi::c_int as isize) =
                (base as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *(*cur.offset(c as isize))
                .code
                .offset(1 as ::core::ffi::c_int as isize) =
                (base as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        }
        if base as ::core::ffi::c_int >= CID_MAX {
            pdf_warning!(b"CID number too large.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        base = base.wrapping_add(1);
        c = c.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn mapDef_release(mut t: *mut mapDef) {
    let mut c: ::core::ffi::c_int = 0;
    if t.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"t\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    c = 0 as ::core::ffi::c_int;
    while c < 256 as ::core::ffi::c_int {
        if (*t.offset(c as isize)).flag & MAP_LOOKUP_CONTINUE != 0 {
            mapDef_release((*t.offset(c as isize)).next as *mut mapDef);
        }
        c += 1;
    }
    free(t as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn mapDef_new() -> *mut mapDef {
    let mut t: *mut mapDef = ::core::ptr::null_mut::<mapDef>();
    let mut c: ::core::ffi::c_int = 0;
    t = new((256 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<mapDef>() as usize) as uint32_t)
        as *mut mapDef;
    c = 0 as ::core::ffi::c_int;
    while c < 256 as ::core::ffi::c_int {
        (*t.offset(c as isize)).flag = MAP_LOOKUP_END | MAP_IS_UNDEF;
        let ref mut fresh2 = (*t.offset(c as isize)).code;
        *fresh2 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let ref mut fresh3 = (*t.offset(c as isize)).next;
        *fresh3 = ::core::ptr::null_mut::<mapDef>();
        c += 1;
    }
    return t;
}
unsafe extern "C-unwind" fn get_mem(
    mut cmap: *mut CMap,
    mut size: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_uchar {
    let mut map: *mut mapData = ::core::ptr::null_mut::<mapData>();
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !(!cmap.is_null() && !(*cmap).mapData.is_null() && size >= 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap && cmap->mapData && size >= 0\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    map = (*cmap).mapData;
    if (*map).pos + size >= MEM_ALLOC_SIZE {
        let mut prev: *mut mapData = map;
        map = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<mapData>() as usize) as uint32_t)
            as *mut mapData as *mut mapData;
        (*map).data = new((4096 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        (*map).prev = prev as *mut mapData;
        (*map).pos = 0 as ::core::ffi::c_int;
        (*cmap).mapData = map;
    }
    p = (*map).data.offset((*map).pos as isize);
    (*map).pos += size;
    return p;
}
unsafe extern "C-unwind" fn locate_tbl(
    mut cur: *mut *mut mapDef,
    mut code: *const ::core::ffi::c_uchar,
    mut dim: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    if !(!cur.is_null() && !(*cur).is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cur && *cur\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    i = 0 as ::core::ffi::c_int;
    while i < dim - 1 as ::core::ffi::c_int {
        c = *code.offset(i as isize) as ::core::ffi::c_int;
        if if (*(*cur).offset(c as isize)).flag & MAP_TYPE_MASK != MAP_IS_UNDEF {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } != 0
        {
            pdf_warning!(b"Ambiguous CMap entry.\0" as *const u8 as *const ::core::ffi::c_char,);
            return -(1 as ::core::ffi::c_int);
        }
        if (*(*cur).offset(c as isize)).next.is_null() {
            let ref mut fresh1 = (*(*cur).offset(c as isize)).next;
            *fresh1 = mapDef_new() as *mut mapDef;
        }
        (*(*cur).offset(c as isize)).flag |= MAP_LOOKUP_CONTINUE;
        *cur = (*(*cur).offset(c as isize)).next as *mut mapDef;
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn bytes_consumed(
    mut cmap: *mut CMap,
    mut instr: *const ::core::ffi::c_uchar,
    mut inbytes: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut pos: size_t = 0;
    let mut longest: size_t = 0 as size_t;
    let mut bytesconsumed: size_t = 0;
    if cmap.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    i = 0 as size_t;
    while i < (*cmap).codespace.num as size_t {
        let mut csr: *mut rangeDef = (*cmap).codespace.ranges.offset(i as isize);
        pos = 0 as size_t;
        while pos
            < (if (*csr).dim < inbytes {
                (*csr).dim
            } else {
                inbytes
            })
        {
            if *instr.offset(pos as isize) as ::core::ffi::c_int
                > *(*csr).codeHi.offset(pos as isize) as ::core::ffi::c_int
                || (*instr.offset(pos as isize) as ::core::ffi::c_int)
                    < *(*csr).codeLo.offset(pos as isize) as ::core::ffi::c_int
            {
                break;
            }
            pos = pos.wrapping_add(1);
        }
        if pos == (*csr).dim {
            return (*csr).dim;
        }
        if pos > longest {
            longest = pos;
        }
        i = i.wrapping_add(1);
    }
    if i == (*cmap).codespace.num as size_t {
        bytesconsumed = (*cmap).profile.minBytesIn;
    } else {
        bytesconsumed = (*cmap).profile.maxBytesIn;
        i = 0 as size_t;
        while i < (*cmap).codespace.num as size_t {
            let mut csr_0: *mut rangeDef = (*cmap).codespace.ranges.offset(i as isize);
            if (*csr_0).dim > longest && (*csr_0).dim < bytesconsumed {
                bytesconsumed = (*csr_0).dim;
            }
            i = i.wrapping_add(1);
        }
    }
    return bytesconsumed;
}
unsafe extern "C-unwind" fn check_range(
    mut cmap: *mut CMap,
    mut srclo: *const ::core::ffi::c_uchar,
    mut srchi: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut dst: *const ::core::ffi::c_uchar,
    mut dstdim: size_t,
) -> ::core::ffi::c_int {
    if srcdim < 1 as size_t
        || dstdim < 1 as size_t
        || (srclo.is_null() || srchi.is_null() || dst.is_null())
        || memcmp(
            srclo as *const ::core::ffi::c_void,
            srchi as *const ::core::ffi::c_void,
            srcdim.wrapping_sub(1 as size_t),
        ) != 0
        || *srclo.offset(srcdim.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            > *srchi.offset(srcdim.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Invalid CMap mapping entry. (ignored)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if CMap_match_codespace(cmap, srclo, srcdim) < 0 as ::core::ffi::c_int
        || CMap_match_codespace(cmap, srchi, srcdim) < 0 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Invalid CMap mapping entry. (ignored)\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if srcdim < (*cmap).profile.minBytesIn {
        (*cmap).profile.minBytesIn = srcdim;
    }
    if srcdim > (*cmap).profile.maxBytesIn {
        (*cmap).profile.maxBytesIn = srcdim;
    }
    if dstdim < (*cmap).profile.minBytesOut {
        (*cmap).profile.minBytesOut = dstdim;
    }
    if dstdim > (*cmap).profile.maxBytesOut {
        (*cmap).profile.maxBytesOut = dstdim;
    }
    return 0 as ::core::ffi::c_int;
}
pub const CMAP_CACHE_ALLOC_SIZE: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;

#[unsafe(export_name = "__pdf_impl_CMap_cache_init")]
pub unsafe extern "C-unwind" fn CMap_cache_init() {
    let __pdf_state = unsafe { document_state() };
    const RANGE_MIN: [::core::ffi::c_uchar; 2] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    const RANGE_MAX: [::core::ffi::c_uchar; 2] = [
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    if !(*__pdf_state).__cache.is_null() {
        pdf_abort!(
            b"%s: Already initialized.\0" as *const u8 as *const ::core::ffi::c_char,
            CMAP_DEBUG_STR.as_ptr(),
        );
    }
    (*__pdf_state).__cache = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<CMap_cache>() as usize)
        as uint32_t) as *mut CMap_cache;
    (*(*__pdf_state).__cache).max = CMAP_CACHE_ALLOC_SIZE as ::core::ffi::c_int;
    (*(*__pdf_state).__cache).cmaps = new(((*(*__pdf_state).__cache).max as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<*mut CMap>() as usize)
        as uint32_t) as *mut *mut CMap;
    (*(*__pdf_state).__cache).num = 0 as ::core::ffi::c_int;
    let ref mut fresh7 = *(*(*__pdf_state).__cache)
        .cmaps
        .offset(0 as ::core::ffi::c_int as isize);
    *fresh7 = CMap_new();
    CMap_set_name(
        *(*(*__pdf_state).__cache)
            .cmaps
            .offset(0 as ::core::ffi::c_int as isize),
        b"Identity-H\0" as *const u8 as *const ::core::ffi::c_char,
    );
    CMap_set_type(
        *(*(*__pdf_state).__cache)
            .cmaps
            .offset(0 as ::core::ffi::c_int as isize),
        CMAP_TYPE_IDENTITY,
    );
    CMap_set_wmode(
        *(*(*__pdf_state).__cache)
            .cmaps
            .offset(0 as ::core::ffi::c_int as isize),
        0 as ::core::ffi::c_int,
    );
    CMap_set_CIDSysInfo(
        *(*(*__pdf_state).__cache)
            .cmaps
            .offset(0 as ::core::ffi::c_int as isize),
        &raw mut CSI_IDENTITY,
    );
    CMap_add_codespacerange(
        *(*(*__pdf_state).__cache)
            .cmaps
            .offset(0 as ::core::ffi::c_int as isize),
        RANGE_MIN.as_ptr(),
        RANGE_MAX.as_ptr(),
        2 as size_t,
    );
    let ref mut fresh8 = *(*(*__pdf_state).__cache)
        .cmaps
        .offset(1 as ::core::ffi::c_int as isize);
    *fresh8 = CMap_new();
    CMap_set_name(
        *(*(*__pdf_state).__cache)
            .cmaps
            .offset(1 as ::core::ffi::c_int as isize),
        b"Identity-V\0" as *const u8 as *const ::core::ffi::c_char,
    );
    CMap_set_type(
        *(*(*__pdf_state).__cache)
            .cmaps
            .offset(1 as ::core::ffi::c_int as isize),
        CMAP_TYPE_IDENTITY,
    );
    CMap_set_wmode(
        *(*(*__pdf_state).__cache)
            .cmaps
            .offset(1 as ::core::ffi::c_int as isize),
        1 as ::core::ffi::c_int,
    );
    CMap_set_CIDSysInfo(
        *(*(*__pdf_state).__cache)
            .cmaps
            .offset(1 as ::core::ffi::c_int as isize),
        &raw mut CSI_IDENTITY,
    );
    CMap_add_codespacerange(
        *(*(*__pdf_state).__cache)
            .cmaps
            .offset(1 as ::core::ffi::c_int as isize),
        RANGE_MIN.as_ptr(),
        RANGE_MAX.as_ptr(),
        2 as size_t,
    );
    (*(*__pdf_state).__cache).num += 2 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_CMap_cache_get")]
pub unsafe extern "C-unwind" fn CMap_cache_get(mut id: ::core::ffi::c_int) -> *mut CMap {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).__cache.is_null() {
        pdf_abort!(
            b"%s: CMap cache not initialized.\0" as *const u8 as *const ::core::ffi::c_char,
            CMAP_DEBUG_STR.as_ptr(),
        );
    }
    if id < 0 as ::core::ffi::c_int || id >= (*(*__pdf_state).__cache).num {
        pdf_abort!(
            b"Invalid CMap ID %d\0" as *const u8 as *const ::core::ffi::c_char,
            id,
        );
    }
    return *(*(*__pdf_state).__cache).cmaps.offset(id as isize);
}
#[unsafe(export_name = "__pdf_impl_CMap_cache_find")]
pub unsafe extern "C-unwind" fn CMap_cache_find(
    mut cmap_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    if (*__pdf_state).__cache.is_null() {
        CMap_cache_init();
    }
    if (*__pdf_state).__cache.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"__cache\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    id = 0 as ::core::ffi::c_int;
    while id < (*(*__pdf_state).__cache).num {
        let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        name = CMap_get_name(*(*(*__pdf_state).__cache).cmaps.offset(id as isize));
        if !name.is_null() && streq_ptr(cmap_name, name) as ::core::ffi::c_int != 0 {
            return id;
        }
        id += 1;
    }
    handle = ttstub_input_open(cmap_name, TTBC_FILE_FORMAT_CMAP, 0 as ::core::ffi::c_int);
    if handle == INVALID_HANDLE {
        return -(1 as ::core::ffi::c_int);
    }
    if CMap_parse_check_sig(handle) < 0 as ::core::ffi::c_int {
        crate::backend::runtime::input_close(handle);
        return -(1 as ::core::ffi::c_int);
    }
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(
            b"(CMap:%s\0" as *const u8 as *const ::core::ffi::c_char,
            cmap_name
        );
    }
    if (*(*__pdf_state).__cache).num >= (*(*__pdf_state).__cache).max {
        (*(*__pdf_state).__cache).max = crate::compat::allocation::grow_capacity(
            (*(*__pdf_state).__cache).max as usize,
            (*(*__pdf_state).__cache).num as usize + 1,
            core::mem::size_of::<*mut CMap>(),
            CMAP_CACHE_ALLOC_SIZE as usize,
        ) as i32;
        (*(*__pdf_state).__cache).cmaps = renew(
            (*(*__pdf_state).__cache).cmaps as *mut ::core::ffi::c_void,
            ((*(*__pdf_state).__cache).max as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<*mut CMap>() as usize) as uint32_t,
        ) as *mut *mut CMap;
    }
    id = (*(*__pdf_state).__cache).num;
    (*(*__pdf_state).__cache).num += 1;
    let ref mut fresh9 = *(*(*__pdf_state).__cache).cmaps.offset(id as isize);
    *fresh9 = CMap_new();
    if CMap_parse(*(*(*__pdf_state).__cache).cmaps.offset(id as isize), handle)
        < 0 as ::core::ffi::c_int
    {
        pdf_abort!(
            b"%s: Parsing CMap file failed.\0" as *const u8 as *const ::core::ffi::c_char,
            CMAP_DEBUG_STR.as_ptr(),
        );
    }
    crate::backend::runtime::input_close(handle);
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(b")\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return id;
}
#[unsafe(export_name = "__pdf_impl_CMap_cache_add")]
pub unsafe extern "C-unwind" fn CMap_cache_add(mut cmap: *mut CMap) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut id: ::core::ffi::c_int = 0;
    let mut cmap_name0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cmap_name1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !CMap_is_valid(cmap) {
        pdf_abort!(
            b"%s: Invalid CMap.\0" as *const u8 as *const ::core::ffi::c_char,
            CMAP_DEBUG_STR.as_ptr(),
        );
    }
    id = 0 as ::core::ffi::c_int;
    while id < (*(*__pdf_state).__cache).num {
        cmap_name0 = CMap_get_name(cmap);
        cmap_name1 = CMap_get_name(*(*(*__pdf_state).__cache).cmaps.offset(id as isize));
        if streq_ptr(cmap_name0, cmap_name1) {
            pdf_abort!(
                b"%s: CMap \"%s\" already defined.\0" as *const u8 as *const ::core::ffi::c_char,
                CMAP_DEBUG_STR.as_ptr(),
                cmap_name0,
            );
        }
        id += 1;
    }
    if (*(*__pdf_state).__cache).num >= (*(*__pdf_state).__cache).max {
        (*(*__pdf_state).__cache).max = crate::compat::allocation::grow_capacity(
            (*(*__pdf_state).__cache).max as usize,
            (*(*__pdf_state).__cache).num as usize + 1,
            core::mem::size_of::<*mut CMap>(),
            CMAP_CACHE_ALLOC_SIZE as usize,
        ) as i32;
        (*(*__pdf_state).__cache).cmaps = renew(
            (*(*__pdf_state).__cache).cmaps as *mut ::core::ffi::c_void,
            ((*(*__pdf_state).__cache).max as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<*mut CMap>() as usize) as uint32_t,
        ) as *mut *mut CMap;
    }
    id = (*(*__pdf_state).__cache).num;
    (*(*__pdf_state).__cache).num += 1;
    let ref mut fresh10 = *(*(*__pdf_state).__cache).cmaps.offset(id as isize);
    *fresh10 = cmap;
    return id;
}
#[unsafe(export_name = "__pdf_impl_CMap_cache_close")]
pub unsafe extern "C-unwind" fn CMap_cache_close() {
    let __pdf_state = unsafe { document_state() };
    if !(*__pdf_state).__cache.is_null() {
        let mut id: ::core::ffi::c_int = 0;
        id = 0 as ::core::ffi::c_int;
        while id < (*(*__pdf_state).__cache).num {
            CMap_release(*(*(*__pdf_state).__cache).cmaps.offset(id as isize));
            id += 1;
        }
        free((*(*__pdf_state).__cache).cmaps as *mut ::core::ffi::c_void);
        (*__pdf_state).__cache =
            mfree((*__pdf_state).__cache as *mut ::core::ffi::c_void) as *mut CMap_cache;
    }
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_set_silent(mut value: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_set_silent(value) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_new(_result: *mut *mut CMap) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_new() }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_release(mut cmap: *mut CMap) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_release(cmap) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_is_Identity(
    mut cmap: *mut CMap,
    _result: *mut bool,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_is_Identity(cmap) }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_is_valid(mut cmap: *mut CMap, _result: *mut bool) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_is_valid(cmap) }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_get_profile(
    mut cmap: *mut CMap,
    mut type_0: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_get_profile(cmap, type_0) }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_decode_char(
    mut cmap: *mut CMap,
    mut inbuf: *mut *const ::core::ffi::c_uchar,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut ::core::ffi::c_uchar,
    mut outbytesleft: *mut size_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        CMap_decode_char(cmap, inbuf, inbytesleft, outbuf, outbytesleft)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_decode(
    mut cmap: *mut CMap,
    mut inbuf: *mut *const ::core::ffi::c_uchar,
    mut inbytesleft: *mut size_t,
    mut outbuf: *mut *mut ::core::ffi::c_uchar,
    mut outbytesleft: *mut size_t,
    _result: *mut size_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        CMap_decode(cmap, inbuf, inbytesleft, outbuf, outbytesleft)
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
pub unsafe extern "C" fn __pdf_entry_CMap_get_name(
    mut cmap: *mut CMap,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_get_name(cmap) }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_get_type(
    mut cmap: *mut CMap,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_get_type(cmap) }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_get_wmode(
    mut cmap: *mut CMap,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_get_wmode(cmap) }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_get_CIDSysInfo(
    mut cmap: *mut CMap,
    _result: *mut *mut CIDSysInfo,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_get_CIDSysInfo(cmap) }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_set_name(
    mut cmap: *mut CMap,
    mut name: *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_set_name(cmap, name) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_set_type(
    mut cmap: *mut CMap,
    mut type_0: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_set_type(cmap, type_0) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_set_wmode(
    mut cmap: *mut CMap,
    mut wmode: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_set_wmode(cmap, wmode) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_set_CIDSysInfo(
    mut cmap: *mut CMap,
    mut csi: *const CIDSysInfo,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_set_CIDSysInfo(cmap, csi) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_set_usecmap(
    mut cmap: *mut CMap,
    mut ucmap: *mut CMap,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_set_usecmap(cmap, ucmap) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_add_codespacerange(
    mut cmap: *mut CMap,
    mut codelo: *const ::core::ffi::c_uchar,
    mut codehi: *const ::core::ffi::c_uchar,
    mut dim: size_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        CMap_add_codespacerange(cmap, codelo, codehi, dim)
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
pub unsafe extern "C" fn __pdf_entry_CMap_add_notdefchar(
    mut cmap: *mut CMap,
    mut src: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut dst: CID,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        CMap_add_notdefchar(cmap, src, srcdim, dst)
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
pub unsafe extern "C" fn __pdf_entry_CMap_add_notdefrange(
    mut cmap: *mut CMap,
    mut srclo: *const ::core::ffi::c_uchar,
    mut srchi: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut dst: CID,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        CMap_add_notdefrange(cmap, srclo, srchi, srcdim, dst)
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
pub unsafe extern "C" fn __pdf_entry_CMap_add_bfchar(
    mut cmap: *mut CMap,
    mut src: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut dst: *const ::core::ffi::c_uchar,
    mut dstdim: size_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        CMap_add_bfchar(cmap, src, srcdim, dst, dstdim)
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
pub unsafe extern "C" fn __pdf_entry_CMap_add_bfrange(
    mut cmap: *mut CMap,
    mut srclo: *const ::core::ffi::c_uchar,
    mut srchi: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut base: *const ::core::ffi::c_uchar,
    mut dstdim: size_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        CMap_add_bfrange(cmap, srclo, srchi, srcdim, base, dstdim)
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
pub unsafe extern "C" fn __pdf_entry_CMap_add_cidchar(
    mut cmap: *mut CMap,
    mut src: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut dst: CID,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_add_cidchar(cmap, src, srcdim, dst) })
    {
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
pub unsafe extern "C" fn __pdf_entry_CMap_add_cidrange(
    mut cmap: *mut CMap,
    mut srclo: *const ::core::ffi::c_uchar,
    mut srchi: *const ::core::ffi::c_uchar,
    mut srcdim: size_t,
    mut base: CID,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        CMap_add_cidrange(cmap, srclo, srchi, srcdim, base)
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
pub unsafe extern "C" fn __pdf_entry_CMap_cache_init() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_cache_init() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_cache_get(
    mut id: ::core::ffi::c_int,
    _result: *mut *mut CMap,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_cache_get(id) }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_cache_find(
    mut cmap_name: *const ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_cache_find(cmap_name) }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_cache_add(
    mut cmap: *mut CMap,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_cache_add(cmap) }) {
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
pub unsafe extern "C" fn __pdf_entry_CMap_cache_close() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_cache_close() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}

pub(crate) struct DocumentState {
    __silent: ::core::ffi::c_int,
    __cache: *mut CMap_cache,
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self {
            __silent: 0 as ::core::ffi::c_int,
            __cache: ::core::ptr::null_mut::<CMap_cache>(),
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe {
        (*crate::compat::document::current())
            .backend
            .dpx_cmap
            .pointer
    }
}
