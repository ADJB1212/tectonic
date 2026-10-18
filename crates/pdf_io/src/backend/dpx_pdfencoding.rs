/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2008-2019 by Jin-Hwan Cho, Matthias Franz, and Shunsaku Hirata,
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
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> ssize_t;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_name_value"]
    fn pdf_name_value(object: *mut pdf_obj) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_get_array"]
    fn pdf_get_array(array: *mut pdf_obj, idx: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(
        dict: *mut pdf_obj,
        key: *mut pdf_obj,
        value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_check_version"]
    fn pdf_check_version(
        major: ::core::ffi::c_int,
        minor: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut dpx_conf: _dpx_conf;
    #[link_name = "__pdf_impl_dpx_tt_open"]
    fn dpx_tt_open(
        filename: *const ::core::ffi::c_char,
        suffix: *const ::core::ffi::c_char,
        format: ttbc_file_format,
    ) -> rust_input_handle_t;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdfparse_skip_line"]
    fn pdfparse_skip_line(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_parse_pdf_name"]
    fn parse_pdf_name(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_array"]
    fn parse_pdf_array(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_agl_sput_UTF16BE"]
    fn agl_sput_UTF16BE(
        name: *const ::core::ffi::c_char,
        dstpp: *mut *mut ::core::ffi::c_uchar,
        limptr: *mut ::core::ffi::c_uchar,
        num_fails: *mut ::core::ffi::c_int,
    ) -> int32_t;
    static mut CSI_UNICODE: CIDSysInfo;
    #[link_name = "__pdf_impl_CMap_new"]
    fn CMap_new() -> *mut CMap;
    #[link_name = "__pdf_impl_CMap_release"]
    fn CMap_release(cmap: *mut CMap);
    #[link_name = "__pdf_impl_CMap_set_name"]
    fn CMap_set_name(cmap: *mut CMap, name: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_CMap_set_type"]
    fn CMap_set_type(cmap: *mut CMap, type_0: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_CMap_set_wmode"]
    fn CMap_set_wmode(cmap: *mut CMap, wmode: ::core::ffi::c_int);
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
    #[link_name = "__pdf_impl_CMap_add_codespacerange"]
    fn CMap_add_codespacerange(
        cmap: *mut CMap,
        codelo: *const ::core::ffi::c_uchar,
        codehi: *const ::core::ffi::c_uchar,
        dim: size_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_parse_check_sig"]
    fn CMap_parse_check_sig(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_parse"]
    fn CMap_parse(cmap: *mut CMap, handle: rust_input_handle_t) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_create_stream"]
    fn CMap_create_stream(cmap: *mut CMap) -> *mut pdf_obj;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
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
pub struct pdf_encoding {
    pub ident: *mut ::core::ffi::c_char,
    pub enc_name: *mut ::core::ffi::c_char,
    pub flags: ::core::ffi::c_int,
    pub glyphs: [*mut ::core::ffi::c_char; 256],
    pub is_used: [::core::ffi::c_char; 256],
    pub tounicode: *mut pdf_obj,
    pub resource: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub count: ::core::ffi::c_int,
    pub capacity: ::core::ffi::c_int,
    pub encodings: *mut pdf_encoding,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CMap {
    pub name: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
    pub CSI: *mut CIDSysInfo,
    pub useCMap: *mut CMap,
    pub codespace: C2RustUnnamed_1,
    pub mapTbl: *mut mapDef,
    pub mapData: *mut mapData,
    pub flags: ::core::ffi::c_int,
    pub profile: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub minBytesIn: size_t,
    pub maxBytesIn: size_t,
    pub minBytesOut: size_t,
    pub maxBytesOut: size_t,
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
pub struct mapDef {
    pub flag: ::core::ffi::c_int,
    pub len: size_t,
    pub code: *mut ::core::ffi::c_uchar,
    pub next: *mut mapDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub num: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
    pub ranges: *mut rangeDef,
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
pub struct CIDSysInfo {
    pub registry: *mut ::core::ffi::c_char,
    pub ordering: *mut ::core::ffi::c_char,
    pub supplement: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub const FLAG_IS_PREDEFINED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const FLAG_USED_BY_TYPE3: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn pdf_init_encoding_struct(mut encoding: *mut pdf_encoding) {
    if encoding.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"encoding\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*encoding).ident = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*encoding).enc_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    memset(
        &raw mut (*encoding).glyphs as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (256 as size_t).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t),
    );
    memset(
        &raw mut (*encoding).is_used as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        256 as size_t,
    );
    (*encoding).tounicode = ::core::ptr::null_mut::<pdf_obj>();
    (*encoding).resource = ::core::ptr::null_mut::<pdf_obj>();
    (*encoding).flags = 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn create_encoding_resource(
    mut enc_vec: *mut *mut ::core::ffi::c_char,
    mut baseenc_name: *const ::core::ffi::c_char,
    mut baseenc_vec: *mut *const ::core::ffi::c_char,
    mut is_used: *mut ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut differences: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if enc_vec.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"enc_vec\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !baseenc_name.is_null() {
        if strcmp(
            baseenc_name,
            b"MacRomanEncoding\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
            && strcmp(
                baseenc_name,
                b"MacExpertEncoding\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0
            && strcmp(
                baseenc_name,
                b"WinAnsiEncoding\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0
        {
            pdf_abort!(
                b"Invalid encoding name for BaseEncoding: %s\0" as *const u8
                    as *const ::core::ffi::c_char,
                baseenc_name,
            );
        }
    }
    differences = make_encoding_differences(enc_vec, baseenc_vec, is_used);
    if !differences.is_null() {
        let mut resource: *mut pdf_obj = pdf_new_dict();
        if !baseenc_name.is_null() {
            pdf_add_dict(
                resource,
                pdf_new_name(b"BaseEncoding\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_new_name(baseenc_name),
            );
        }
        pdf_add_dict(
            resource,
            pdf_new_name(b"Differences\0" as *const u8 as *const ::core::ffi::c_char),
            differences,
        );
        return resource;
    } else {
        return if !baseenc_name.is_null() {
            pdf_new_name(baseenc_name)
        } else {
            ::core::ptr::null_mut::<pdf_obj>()
        };
    };
}
unsafe extern "C-unwind" fn pdf_flush_encoding(mut encoding: *mut pdf_encoding) {
    if encoding.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"encoding\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(*encoding).resource.is_null() {
        pdf_release_obj((*encoding).resource);
        (*encoding).resource = ::core::ptr::null_mut::<pdf_obj>();
    }
    if !(*encoding).tounicode.is_null() {
        pdf_release_obj((*encoding).tounicode);
        (*encoding).tounicode = ::core::ptr::null_mut::<pdf_obj>();
    }
}
unsafe extern "C-unwind" fn pdf_clean_encoding_struct(mut encoding: *mut pdf_encoding) {
    let mut code: ::core::ffi::c_int = 0;
    if encoding.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"encoding\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(*encoding).resource.is_null() {
        pdf_abort!(b"Object not flushed.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    pdf_release_obj((*encoding).tounicode);
    free((*encoding).ident as *mut ::core::ffi::c_void);
    free((*encoding).enc_name as *mut ::core::ffi::c_void);
    (*encoding).ident = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*encoding).enc_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        (*encoding).glyphs[code as usize] =
            mfree((*encoding).glyphs[code as usize] as *mut ::core::ffi::c_void)
                as *mut ::core::ffi::c_char;
        code += 1;
    }
    (*encoding).ident = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*encoding).enc_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C-unwind" fn is_similar_charset(
    mut enc_vec: *mut *mut ::core::ffi::c_char,
    mut enc_vec2: *mut *const ::core::ffi::c_char,
) -> bool {
    let mut code: ::core::ffi::c_int = 0;
    let mut same: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        if !(!(*enc_vec.offset(code as isize)).is_null()
            && strcmp(
                *enc_vec.offset(code as isize),
                *enc_vec2.offset(code as isize),
            ) != 0)
            && {
                same += 1;
                same >= 64 as ::core::ffi::c_int
            }
        {
            return true_0 != 0;
        }
        code += 1;
    }
    return false_0 != 0;
}
unsafe extern "C-unwind" fn make_encoding_differences(
    mut enc_vec: *mut *mut ::core::ffi::c_char,
    mut baseenc: *mut *const ::core::ffi::c_char,
    mut is_used: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut differences: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut code: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut skipping: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if enc_vec.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"enc_vec\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    differences = pdf_new_array();
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        if !is_used.is_null() && *is_used.offset(code as isize) == 0
            || (*enc_vec.offset(code as isize)).is_null()
        {
            skipping = 1 as ::core::ffi::c_int;
        } else if baseenc.is_null()
            || (*baseenc.offset(code as isize)).is_null()
            || strcmp(
                *baseenc.offset(code as isize),
                *enc_vec.offset(code as isize),
            ) != 0 as ::core::ffi::c_int
        {
            if skipping != 0 {
                pdf_add_array(differences, pdf_new_number(code as ::core::ffi::c_double));
            }
            pdf_add_array(differences, pdf_new_name(*enc_vec.offset(code as isize)));
            skipping = 0 as ::core::ffi::c_int;
            count += 1;
        } else {
            skipping = 1 as ::core::ffi::c_int;
        }
        code += 1;
    }
    if count == 0 as ::core::ffi::c_int {
        pdf_release_obj(differences);
        differences = ::core::ptr::null_mut::<pdf_obj>();
    }
    return differences;
}
unsafe extern "C-unwind" fn load_encoding_file(
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    let mut enc_name: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut encoding_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut wbuf_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut enc_vec: [*const ::core::ffi::c_char; 256] =
        [::core::ptr::null::<::core::ffi::c_char>(); 256];
    let mut code: ::core::ffi::c_int = 0;
    let mut fsize: ::core::ffi::c_int = 0;
    let mut enc_id: ::core::ffi::c_int = 0;
    if filename.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(
            b"(Encoding:%s\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
    }
    handle = dpx_tt_open(
        filename,
        b".enc\0" as *const u8 as *const ::core::ffi::c_char,
        TTBC_FILE_FORMAT_ENC,
    );
    if handle == INVALID_HANDLE {
        return -(1 as ::core::ffi::c_int);
    }
    fsize = ttstub_input_get_size(handle) as ::core::ffi::c_int;
    wbuf_0 = new(((fsize + 1 as ::core::ffi::c_int) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    *wbuf_0.offset(fsize as isize) = '\0' as i32 as ::core::ffi::c_char;
    if ttstub_input_read(handle, wbuf_0, fsize as size_t) != fsize as ssize_t {
        pdf_abort!(
            b"error reading %s\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
    }
    crate::backend::runtime::input_close(handle);
    p = wbuf_0;
    endptr = wbuf_0.offset(fsize as isize);
    skip_white(&raw mut p, endptr);
    while p < endptr
        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '%' as i32
    {
        pdfparse_skip_line(&raw mut p, endptr);
        skip_white(&raw mut p, endptr);
    }
    if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '/' as i32 {
        enc_name = parse_pdf_name(&raw mut p, endptr);
    }
    skip_white(&raw mut p, endptr);
    encoding_array = parse_pdf_array(&raw mut p, endptr, ::core::ptr::null_mut::<pdf_file>());
    free(wbuf_0 as *mut ::core::ffi::c_void);
    if encoding_array.is_null() {
        pdf_release_obj(enc_name);
        return -(1 as ::core::ffi::c_int);
    }
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        enc_vec[code as usize] = pdf_name_value(pdf_get_array(encoding_array, code));
        code += 1;
    }
    enc_id = pdf_encoding_new_encoding(
        if !enc_name.is_null() {
            pdf_name_value(enc_name)
        } else {
            ::core::ptr::null_mut::<::core::ffi::c_char>()
        },
        filename,
        &raw mut enc_vec as *mut *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    if !enc_name.is_null() {
        if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
            pdf_message!(
                b"[%s]\0" as *const u8 as *const ::core::ffi::c_char,
                pdf_name_value(enc_name),
            );
        }
        pdf_release_obj(enc_name);
    }
    pdf_release_obj(encoding_array);
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(b")\0" as *const u8 as *const ::core::ffi::c_char);
    }
    return enc_id;
}

#[unsafe(export_name = "__pdf_impl_pdf_init_encodings")]
pub unsafe extern "C-unwind" fn pdf_init_encodings() {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).enc_cache.count = 0 as ::core::ffi::c_int;
    (*__pdf_state).enc_cache.capacity = 3 as ::core::ffi::c_int;
    (*__pdf_state).enc_cache.encodings = new(((*__pdf_state).enc_cache.capacity as uint32_t
        as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_encoding>() as usize)
        as uint32_t) as *mut pdf_encoding;
    pdf_encoding_new_encoding(
        b"WinAnsiEncoding\0" as *const u8 as *const ::core::ffi::c_char,
        b"WinAnsiEncoding\0" as *const u8 as *const ::core::ffi::c_char,
        WinAnsiEncoding.as_ptr() as *mut *const ::core::ffi::c_char,
        FLAG_IS_PREDEFINED,
    );
    pdf_encoding_new_encoding(
        b"MacRomanEncoding\0" as *const u8 as *const ::core::ffi::c_char,
        b"MacRomanEncoding\0" as *const u8 as *const ::core::ffi::c_char,
        MacRomanEncoding.as_ptr() as *mut *const ::core::ffi::c_char,
        FLAG_IS_PREDEFINED,
    );
    pdf_encoding_new_encoding(
        b"MacExpertEncoding\0" as *const u8 as *const ::core::ffi::c_char,
        b"MacExpertEncoding\0" as *const u8 as *const ::core::ffi::c_char,
        MacExpertEncoding.as_ptr() as *mut *const ::core::ffi::c_char,
        FLAG_IS_PREDEFINED,
    );
}
unsafe extern "C-unwind" fn pdf_encoding_new_encoding(
    mut enc_name: *const ::core::ffi::c_char,
    mut ident: *const ::core::ffi::c_char,
    mut encoding_vec: *mut *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut enc_id: ::core::ffi::c_int = 0;
    let mut code: ::core::ffi::c_int = 0;
    let mut encoding: *mut pdf_encoding = ::core::ptr::null_mut::<pdf_encoding>();
    enc_id = (*__pdf_state).enc_cache.count;
    let fresh0 = (*__pdf_state).enc_cache.count;
    (*__pdf_state).enc_cache.count = (*__pdf_state).enc_cache.count + 1;
    if fresh0 >= (*__pdf_state).enc_cache.capacity {
        (*__pdf_state).enc_cache.capacity += 16 as ::core::ffi::c_int;
        (*__pdf_state).enc_cache.encodings = renew(
            (*__pdf_state).enc_cache.encodings as *mut ::core::ffi::c_void,
            ((*__pdf_state).enc_cache.capacity as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<pdf_encoding>() as usize)
                as uint32_t,
        ) as *mut pdf_encoding;
    }
    encoding = (*__pdf_state).enc_cache.encodings.offset(enc_id as isize) as *mut pdf_encoding;
    pdf_init_encoding_struct(encoding);
    (*encoding).ident = new(
        (strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*encoding).ident, ident);
    (*encoding).enc_name = new(
        (strlen(enc_name).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*encoding).enc_name, enc_name);
    (*encoding).flags = flags;
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        if !(*encoding_vec.offset(code as isize)).is_null()
            && strcmp(
                *encoding_vec.offset(code as isize),
                b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0
        {
            (*encoding).glyphs[code as usize] = new((strlen(*encoding_vec.offset(code as isize))
                .wrapping_add(1 as size_t)
                as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                as uint32_t)
                as *mut ::core::ffi::c_char;
            strcpy(
                (*encoding).glyphs[code as usize],
                *encoding_vec.offset(code as isize),
            );
        }
        code += 1;
    }
    if flags & FLAG_IS_PREDEFINED != 0 {
        (*encoding).resource = pdf_new_name((*encoding).enc_name);
    }
    return enc_id;
}
#[unsafe(export_name = "__pdf_impl_pdf_encoding_complete")]
pub unsafe extern "C-unwind" fn pdf_encoding_complete() {
    let __pdf_state = unsafe { document_state() };
    let mut enc_id: ::core::ffi::c_int = 0;
    enc_id = 0 as ::core::ffi::c_int;
    while enc_id < (*__pdf_state).enc_cache.count {
        if pdf_encoding_is_predefined(enc_id) == 0 {
            let mut encoding: *mut pdf_encoding =
                (*__pdf_state).enc_cache.encodings.offset(enc_id as isize) as *mut pdf_encoding;
            let mut baseenc_name: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            let mut baseenc_vec: *mut *const ::core::ffi::c_char =
                ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
            let mut with_base: ::core::ffi::c_int = ((*encoding).flags & FLAG_USED_BY_TYPE3 == 0
                || pdf_check_version(1 as ::core::ffi::c_int, 4 as ::core::ffi::c_int)
                    >= 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if !(*encoding).resource.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                pdf_abort!(
                    c"assertion failed: %s".as_ptr(),
                    b"!encoding->resource\0" as *const u8 as *const ::core::ffi::c_char
                );
            } else {
            };
            if !(*encoding).tounicode.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                pdf_abort!(
                    c"assertion failed: %s".as_ptr(),
                    b"!encoding->tounicode\0" as *const u8 as *const ::core::ffi::c_char
                );
            } else {
            };
            if (*encoding).flags & FLAG_IS_PREDEFINED == 0 && with_base != 0 {
                if is_similar_charset(
                    &raw mut (*encoding).glyphs as *mut *mut ::core::ffi::c_char,
                    WinAnsiEncoding.as_ptr() as *mut *const ::core::ffi::c_char,
                ) {
                    baseenc_name = b"WinAnsiEncoding\0" as *const u8 as *const ::core::ffi::c_char;
                    baseenc_vec = WinAnsiEncoding.as_ptr() as *mut *const ::core::ffi::c_char;
                }
            }
            (*encoding).resource = create_encoding_resource(
                &raw mut (*encoding).glyphs as *mut *mut ::core::ffi::c_char,
                baseenc_name,
                baseenc_vec,
                &raw mut (*encoding).is_used as *mut ::core::ffi::c_char,
            );
            (*encoding).tounicode = pdf_create_ToUnicode_CMap(
                (*encoding).enc_name,
                &raw mut (*encoding).glyphs as *mut *mut ::core::ffi::c_char,
                &raw mut (*encoding).is_used as *mut ::core::ffi::c_char,
            );
        }
        enc_id += 1;
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_close_encodings")]
pub unsafe extern "C-unwind" fn pdf_close_encodings() {
    let __pdf_state = unsafe { document_state() };
    let mut enc_id: ::core::ffi::c_int = 0;
    if !(*__pdf_state).enc_cache.encodings.is_null() {
        enc_id = 0 as ::core::ffi::c_int;
        while enc_id < (*__pdf_state).enc_cache.count {
            let mut encoding: *mut pdf_encoding = ::core::ptr::null_mut::<pdf_encoding>();
            encoding =
                (*__pdf_state).enc_cache.encodings.offset(enc_id as isize) as *mut pdf_encoding;
            if !encoding.is_null() {
                pdf_flush_encoding(encoding);
                pdf_clean_encoding_struct(encoding);
            }
            enc_id += 1;
        }
        free((*__pdf_state).enc_cache.encodings as *mut ::core::ffi::c_void);
    }
    (*__pdf_state).enc_cache.encodings = ::core::ptr::null_mut::<pdf_encoding>();
    (*__pdf_state).enc_cache.count = 0 as ::core::ffi::c_int;
    (*__pdf_state).enc_cache.capacity = 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_encoding_findresource")]
pub unsafe extern "C-unwind" fn pdf_encoding_findresource(
    mut enc_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut enc_id: ::core::ffi::c_int = 0;
    let mut encoding: *mut pdf_encoding = ::core::ptr::null_mut::<pdf_encoding>();
    if enc_name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"enc_name\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    enc_id = 0 as ::core::ffi::c_int;
    while enc_id < (*__pdf_state).enc_cache.count {
        encoding = (*__pdf_state).enc_cache.encodings.offset(enc_id as isize) as *mut pdf_encoding;
        if !(*encoding).ident.is_null()
            && streq_ptr(enc_name, (*encoding).ident) as ::core::ffi::c_int != 0
        {
            return enc_id;
        } else if !(*encoding).enc_name.is_null()
            && streq_ptr(enc_name, (*encoding).enc_name) as ::core::ffi::c_int != 0
        {
            return enc_id;
        }
        enc_id += 1;
    }
    return load_encoding_file(enc_name);
}
#[unsafe(export_name = "__pdf_impl_pdf_encoding_get_encoding")]
pub unsafe extern "C-unwind" fn pdf_encoding_get_encoding(
    mut enc_id: ::core::ffi::c_int,
) -> *mut *mut ::core::ffi::c_char {
    let __pdf_state = unsafe { document_state() };
    let mut encoding: *mut pdf_encoding = ::core::ptr::null_mut::<pdf_encoding>();
    if enc_id < 0 as ::core::ffi::c_int || enc_id >= (*__pdf_state).enc_cache.count {
        pdf_abort!(
            b"Invalid encoding id: %d\0" as *const u8 as *const ::core::ffi::c_char,
            enc_id,
        );
    }
    encoding = (*__pdf_state).enc_cache.encodings.offset(enc_id as isize) as *mut pdf_encoding;
    return &raw mut (*encoding).glyphs as *mut *mut ::core::ffi::c_char;
}
#[unsafe(export_name = "__pdf_impl_pdf_get_encoding_obj")]
pub unsafe extern "C-unwind" fn pdf_get_encoding_obj(
    mut enc_id: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let __pdf_state = unsafe { document_state() };
    let mut encoding: *mut pdf_encoding = ::core::ptr::null_mut::<pdf_encoding>();
    if enc_id < 0 as ::core::ffi::c_int || enc_id >= (*__pdf_state).enc_cache.count {
        pdf_abort!(
            b"Invalid encoding id: %d\0" as *const u8 as *const ::core::ffi::c_char,
            enc_id,
        );
    }
    encoding = (*__pdf_state).enc_cache.encodings.offset(enc_id as isize) as *mut pdf_encoding;
    return (*encoding).resource;
}
#[unsafe(export_name = "__pdf_impl_pdf_encoding_is_predefined")]
pub unsafe extern "C-unwind" fn pdf_encoding_is_predefined(
    mut enc_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut encoding: *mut pdf_encoding = ::core::ptr::null_mut::<pdf_encoding>();
    if enc_id < 0 as ::core::ffi::c_int || enc_id >= (*__pdf_state).enc_cache.count {
        pdf_abort!(
            b"Invalid encoding id: %d\0" as *const u8 as *const ::core::ffi::c_char,
            enc_id,
        );
    }
    encoding = (*__pdf_state).enc_cache.encodings.offset(enc_id as isize) as *mut pdf_encoding;
    return if (*encoding).flags & FLAG_IS_PREDEFINED != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_encoding_used_by_type3")]
pub unsafe extern "C-unwind" fn pdf_encoding_used_by_type3(mut enc_id: ::core::ffi::c_int) {
    let __pdf_state = unsafe { document_state() };
    let mut encoding: *mut pdf_encoding = ::core::ptr::null_mut::<pdf_encoding>();
    if enc_id < 0 as ::core::ffi::c_int || enc_id >= (*__pdf_state).enc_cache.count {
        pdf_abort!(
            b"Invalid encoding id: %d\0" as *const u8 as *const ::core::ffi::c_char,
            enc_id,
        );
    }
    encoding = (*__pdf_state).enc_cache.encodings.offset(enc_id as isize) as *mut pdf_encoding;
    (*encoding).flags |= FLAG_USED_BY_TYPE3;
}
#[unsafe(export_name = "__pdf_impl_pdf_encoding_get_name")]
pub unsafe extern "C-unwind" fn pdf_encoding_get_name(
    mut enc_id: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let __pdf_state = unsafe { document_state() };
    let mut encoding: *mut pdf_encoding = ::core::ptr::null_mut::<pdf_encoding>();
    if enc_id < 0 as ::core::ffi::c_int || enc_id >= (*__pdf_state).enc_cache.count {
        pdf_abort!(
            b"Invalid encoding id: %d\0" as *const u8 as *const ::core::ffi::c_char,
            enc_id,
        );
    }
    encoding = (*__pdf_state).enc_cache.encodings.offset(enc_id as isize) as *mut pdf_encoding;
    return (*encoding).enc_name;
}
pub const CMAP_TYPE_TO_UNICODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const WBUF_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;

const RANGE_MIN: [::core::ffi::c_uchar; 1] = [0 as ::core::ffi::c_uint as ::core::ffi::c_uchar];
const RANGE_MAX: [::core::ffi::c_uchar; 1] = [0xff as ::core::ffi::c_uint as ::core::ffi::c_uchar];
#[unsafe(export_name = "__pdf_impl_pdf_encoding_add_usedchars")]
pub unsafe extern "C-unwind" fn pdf_encoding_add_usedchars(
    mut encoding_id: ::core::ffi::c_int,
    mut is_used: *const ::core::ffi::c_char,
) {
    let __pdf_state = unsafe { document_state() };
    let mut encoding: *mut pdf_encoding = ::core::ptr::null_mut::<pdf_encoding>();
    let mut code: ::core::ffi::c_int = 0;
    if encoding_id < 0 as ::core::ffi::c_int || encoding_id >= (*__pdf_state).enc_cache.count {
        pdf_abort!(
            b"Invalid encoding id: %d\0" as *const u8 as *const ::core::ffi::c_char,
            encoding_id,
        );
    }
    if is_used.is_null() || pdf_encoding_is_predefined(encoding_id) != 0 {
        return;
    }
    encoding = (*__pdf_state)
        .enc_cache
        .encodings
        .offset(encoding_id as isize) as *mut pdf_encoding;
    code = 0 as ::core::ffi::c_int;
    while code <= 0xff as ::core::ffi::c_int {
        (*encoding).is_used[code as usize] = ((*encoding).is_used[code as usize]
            as ::core::ffi::c_int
            | *is_used.offset(code as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        code += 1;
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_encoding_get_tounicode")]
pub unsafe extern "C-unwind" fn pdf_encoding_get_tounicode(
    mut encoding_id: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let __pdf_state = unsafe { document_state() };
    if encoding_id < 0 as ::core::ffi::c_int || encoding_id >= (*__pdf_state).enc_cache.count {
        pdf_abort!(
            b"Invalid encoding id: %d\0" as *const u8 as *const ::core::ffi::c_char,
            encoding_id,
        );
    }
    return (*(*__pdf_state)
        .enc_cache
        .encodings
        .offset(encoding_id as isize))
    .tounicode;
}
#[unsafe(export_name = "__pdf_impl_pdf_create_ToUnicode_CMap")]
pub unsafe extern "C-unwind" fn pdf_create_ToUnicode_CMap(
    mut enc_name: *const ::core::ffi::c_char,
    mut enc_vec: *mut *mut ::core::ffi::c_char,
    mut is_used: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let __pdf_state = unsafe { document_state() };
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
    let mut code: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut total_fail: ::core::ffi::c_int = 0;
    let mut cmap_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut endptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !(!enc_name.is_null() && !enc_vec.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"enc_name && enc_vec\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if is_used.is_null() {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    cmap_name = new((strlen(enc_name)
        .wrapping_add(strlen(
            b"-UTF16\0" as *const u8 as *const ::core::ffi::c_char,
        ))
        .wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    sprintf(
        cmap_name,
        b"%s-UTF16\0" as *const u8 as *const ::core::ffi::c_char,
        enc_name,
    );
    cmap = CMap_new();
    CMap_set_name(cmap, cmap_name);
    CMap_set_type(cmap, CMAP_TYPE_TO_UNICODE);
    CMap_set_wmode(cmap, 0 as ::core::ffi::c_int);
    CMap_set_CIDSysInfo(cmap, &raw mut CSI_UNICODE);
    CMap_add_codespacerange(cmap, RANGE_MIN.as_ptr(), RANGE_MAX.as_ptr(), 1 as size_t);
    count = 0 as ::core::ffi::c_int;
    total_fail = 0 as ::core::ffi::c_int;
    code = 0 as ::core::ffi::c_int;
    while code <= 0xff as ::core::ffi::c_int {
        if !(!is_used.is_null() && *is_used.offset(code as isize) == 0) {
            if !(*enc_vec.offset(code as isize)).is_null() {
                let mut len: size_t = 0;
                let mut fail_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                (*__pdf_state).wbuf[0 as ::core::ffi::c_int as usize] =
                    (code & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                p = (&raw mut (*__pdf_state).wbuf as *mut ::core::ffi::c_uchar)
                    .offset(1 as ::core::ffi::c_int as isize);
                endptr = (&raw mut (*__pdf_state).wbuf as *mut ::core::ffi::c_uchar)
                    .offset(WBUF_SIZE as isize);
                len = agl_sput_UTF16BE(
                    *enc_vec.offset(code as isize),
                    &raw mut p,
                    endptr,
                    &raw mut fail_count,
                ) as size_t;
                if len < 1 as size_t && fail_count > 0 as ::core::ffi::c_int {
                    total_fail += 1;
                } else {
                    CMap_add_bfchar(
                        cmap,
                        &raw mut (*__pdf_state).wbuf as *mut ::core::ffi::c_uchar,
                        1 as size_t,
                        (&raw mut (*__pdf_state).wbuf as *mut ::core::ffi::c_uchar)
                            .offset(1 as ::core::ffi::c_int as isize),
                        len,
                    );
                    count += 1;
                }
            }
        }
        code += 1;
    }
    if total_fail > 0 as ::core::ffi::c_int {
        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
            pdf_warning!(
                b"Glyphs with no Unicode mapping found. Removing ToUnicode CMap.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    }
    stream = if count == 0 as ::core::ffi::c_int || total_fail > 0 as ::core::ffi::c_int {
        ::core::ptr::null_mut::<pdf_obj>()
    } else {
        CMap_create_stream(cmap)
    };
    CMap_release(cmap);
    free(cmap_name as *mut ::core::ffi::c_void);
    return stream;
}
#[unsafe(export_name = "__pdf_impl_pdf_load_ToUnicode_stream")]
pub unsafe extern "C-unwind" fn pdf_load_ToUnicode_stream(
    mut ident: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cmap: *mut CMap = ::core::ptr::null_mut::<CMap>();
    let mut handle: rust_input_handle_t = INVALID_HANDLE;
    if ident.is_null() {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    handle = ttstub_input_open(ident, TTBC_FILE_FORMAT_CMAP, 0 as ::core::ffi::c_int);
    if handle == INVALID_HANDLE {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if CMap_parse_check_sig(handle) < 0 as ::core::ffi::c_int {
        crate::backend::runtime::input_close(handle);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    cmap = CMap_new();
    if CMap_parse(cmap, handle) < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Reading CMap file \"%s\" failed.\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    } else {
        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
            pdf_message!(
                b"(CMap:%s)\0" as *const u8 as *const ::core::ffi::c_char,
                ident,
            );
        }
        stream = CMap_create_stream(cmap);
        if stream.is_null() {
            pdf_warning!(
                b"Failed to creat ToUnicode CMap stream for \"%s\".\0" as *const u8
                    as *const ::core::ffi::c_char,
                ident,
            );
        }
    }
    CMap_release(cmap);
    crate::backend::runtime::input_close(handle);
    return stream;
}
const MacRomanEncoding: [*const ::core::ffi::c_char; 256] = [
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"space\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclam\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"numbersign\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollar\0" as *const u8 as *const ::core::ffi::c_char,
    b"percent\0" as *const u8 as *const ::core::ffi::c_char,
    b"ampersand\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asterisk\0" as *const u8 as *const ::core::ffi::c_char,
    b"plus\0" as *const u8 as *const ::core::ffi::c_char,
    b"comma\0" as *const u8 as *const ::core::ffi::c_char,
    b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
    b"period\0" as *const u8 as *const ::core::ffi::c_char,
    b"slash\0" as *const u8 as *const ::core::ffi::c_char,
    b"zero\0" as *const u8 as *const ::core::ffi::c_char,
    b"one\0" as *const u8 as *const ::core::ffi::c_char,
    b"two\0" as *const u8 as *const ::core::ffi::c_char,
    b"three\0" as *const u8 as *const ::core::ffi::c_char,
    b"four\0" as *const u8 as *const ::core::ffi::c_char,
    b"five\0" as *const u8 as *const ::core::ffi::c_char,
    b"six\0" as *const u8 as *const ::core::ffi::c_char,
    b"seven\0" as *const u8 as *const ::core::ffi::c_char,
    b"eight\0" as *const u8 as *const ::core::ffi::c_char,
    b"nine\0" as *const u8 as *const ::core::ffi::c_char,
    b"colon\0" as *const u8 as *const ::core::ffi::c_char,
    b"semicolon\0" as *const u8 as *const ::core::ffi::c_char,
    b"less\0" as *const u8 as *const ::core::ffi::c_char,
    b"equal\0" as *const u8 as *const ::core::ffi::c_char,
    b"greater\0" as *const u8 as *const ::core::ffi::c_char,
    b"question\0" as *const u8 as *const ::core::ffi::c_char,
    b"at\0" as *const u8 as *const ::core::ffi::c_char,
    b"A\0" as *const u8 as *const ::core::ffi::c_char,
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"C\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"E\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"I\0" as *const u8 as *const ::core::ffi::c_char,
    b"J\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"O\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Q\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"U\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
    b"X\0" as *const u8 as *const ::core::ffi::c_char,
    b"Y\0" as *const u8 as *const ::core::ffi::c_char,
    b"Z\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"backslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char,
    b"underscore\0" as *const u8 as *const ::core::ffi::c_char,
    b"grave\0" as *const u8 as *const ::core::ffi::c_char,
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"b\0" as *const u8 as *const ::core::ffi::c_char,
    b"c\0" as *const u8 as *const ::core::ffi::c_char,
    b"d\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"f\0" as *const u8 as *const ::core::ffi::c_char,
    b"g\0" as *const u8 as *const ::core::ffi::c_char,
    b"h\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"j\0" as *const u8 as *const ::core::ffi::c_char,
    b"k\0" as *const u8 as *const ::core::ffi::c_char,
    b"l\0" as *const u8 as *const ::core::ffi::c_char,
    b"m\0" as *const u8 as *const ::core::ffi::c_char,
    b"n\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"p\0" as *const u8 as *const ::core::ffi::c_char,
    b"q\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\0" as *const u8 as *const ::core::ffi::c_char,
    b"s\0" as *const u8 as *const ::core::ffi::c_char,
    b"t\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"v\0" as *const u8 as *const ::core::ffi::c_char,
    b"w\0" as *const u8 as *const ::core::ffi::c_char,
    b"x\0" as *const u8 as *const ::core::ffi::c_char,
    b"y\0" as *const u8 as *const ::core::ffi::c_char,
    b"z\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"bar\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"dagger\0" as *const u8 as *const ::core::ffi::c_char,
    b"degree\0" as *const u8 as *const ::core::ffi::c_char,
    b"cent\0" as *const u8 as *const ::core::ffi::c_char,
    b"sterling\0" as *const u8 as *const ::core::ffi::c_char,
    b"section\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"paragraph\0" as *const u8 as *const ::core::ffi::c_char,
    b"germandbls\0" as *const u8 as *const ::core::ffi::c_char,
    b"registered\0" as *const u8 as *const ::core::ffi::c_char,
    b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
    b"trademark\0" as *const u8 as *const ::core::ffi::c_char,
    b"acute\0" as *const u8 as *const ::core::ffi::c_char,
    b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"notequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"AE\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"infinity\0" as *const u8 as *const ::core::ffi::c_char,
    b"plusminus\0" as *const u8 as *const ::core::ffi::c_char,
    b"lessequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"greaterequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"yen\0" as *const u8 as *const ::core::ffi::c_char,
    b"mu\0" as *const u8 as *const ::core::ffi::c_char,
    b"partialdiff\0" as *const u8 as *const ::core::ffi::c_char,
    b"summation\0" as *const u8 as *const ::core::ffi::c_char,
    b"product\0" as *const u8 as *const ::core::ffi::c_char,
    b"pi\0" as *const u8 as *const ::core::ffi::c_char,
    b"integral\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char,
    b"Omega\0" as *const u8 as *const ::core::ffi::c_char,
    b"ae\0" as *const u8 as *const ::core::ffi::c_char,
    b"oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char,
    b"logicalnot\0" as *const u8 as *const ::core::ffi::c_char,
    b"radical\0" as *const u8 as *const ::core::ffi::c_char,
    b"florin\0" as *const u8 as *const ::core::ffi::c_char,
    b"approxequal\0" as *const u8 as *const ::core::ffi::c_char,
    b"Delta\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char,
    b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char,
    b"space\0" as *const u8 as *const ::core::ffi::c_char,
    b"Agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"OE\0" as *const u8 as *const ::core::ffi::c_char,
    b"oe\0" as *const u8 as *const ::core::ffi::c_char,
    b"endash\0" as *const u8 as *const ::core::ffi::c_char,
    b"emdash\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteright\0" as *const u8 as *const ::core::ffi::c_char,
    b"divide\0" as *const u8 as *const ::core::ffi::c_char,
    b"lozenge\0" as *const u8 as *const ::core::ffi::c_char,
    b"ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"fraction\0" as *const u8 as *const ::core::ffi::c_char,
    b"currency\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char,
    b"fi\0" as *const u8 as *const ::core::ffi::c_char,
    b"fl\0" as *const u8 as *const ::core::ffi::c_char,
    b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"perthousand\0" as *const u8 as *const ::core::ffi::c_char,
    b"Acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"apple\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"Uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char,
    b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"macron\0" as *const u8 as *const ::core::ffi::c_char,
    b"breve\0" as *const u8 as *const ::core::ffi::c_char,
    b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char,
    b"ring\0" as *const u8 as *const ::core::ffi::c_char,
    b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char,
    b"ogonek\0" as *const u8 as *const ::core::ffi::c_char,
    b"caron\0" as *const u8 as *const ::core::ffi::c_char,
];
const MacExpertEncoding: [*const ::core::ffi::c_char; 256] = [
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"space\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Hungarumlautsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"centoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollaroldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollarsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"ampersandsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Acutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenleftsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenrightsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"twodotenleader\0" as *const u8 as *const ::core::ffi::c_char,
    b"onedotenleader\0" as *const u8 as *const ::core::ffi::c_char,
    b"comma\0" as *const u8 as *const ::core::ffi::c_char,
    b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
    b"period\0" as *const u8 as *const ::core::ffi::c_char,
    b"fraction\0" as *const u8 as *const ::core::ffi::c_char,
    b"zerooldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"oneoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"twooldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"threeoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"fouroldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"fiveoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"sixoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"sevenoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"eightoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"nineoldstyle\0" as *const u8 as *const ::core::ffi::c_char,
    b"colon\0" as *const u8 as *const ::core::ffi::c_char,
    b"semicolon\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"threequartersemdash\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"questionsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ethsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"onequarter\0" as *const u8 as *const ::core::ffi::c_char,
    b"onehalf\0" as *const u8 as *const ::core::ffi::c_char,
    b"threequarters\0" as *const u8 as *const ::core::ffi::c_char,
    b"oneeighth\0" as *const u8 as *const ::core::ffi::c_char,
    b"threeeighths\0" as *const u8 as *const ::core::ffi::c_char,
    b"fiveeighths\0" as *const u8 as *const ::core::ffi::c_char,
    b"seveneighths\0" as *const u8 as *const ::core::ffi::c_char,
    b"onethird\0" as *const u8 as *const ::core::ffi::c_char,
    b"twothirds\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"ff\0" as *const u8 as *const ::core::ffi::c_char,
    b"fi\0" as *const u8 as *const ::core::ffi::c_char,
    b"fl\0" as *const u8 as *const ::core::ffi::c_char,
    b"ffi\0" as *const u8 as *const ::core::ffi::c_char,
    b"ffl\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenleftinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenrightinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Circumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"hypheninferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Asmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Bsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Csmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Esmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Fsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Gsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Hsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ismall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Jsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ksmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Msmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Nsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Osmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Psmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Qsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Rsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ssmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Usmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Vsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Wsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Xsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ysmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Zsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"colonmonetary\0" as *const u8 as *const ::core::ffi::c_char,
    b"onefitted\0" as *const u8 as *const ::core::ffi::c_char,
    b"rupiah\0" as *const u8 as *const ::core::ffi::c_char,
    b"Tildesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"asuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"centsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Agravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Acircumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Atildesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aringsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccedillasmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Egravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ecircumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Edieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Iacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Igravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Icircumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ntildesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ogravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ocircumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Odieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Otildesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Uacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ugravesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ucircumflexsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Udieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"eightsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"fourinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"threeinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"sixinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"eightinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"seveninferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Scaronsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"centinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"twoinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"Caronsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"osuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"fiveinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"commainferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Yacutesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollarinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thornsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"nineinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"zeroinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Zcaronsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"AEsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oslashsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"questiondownsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"oneinferior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Lslashsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"Cedillasmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"OEsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"figuredash\0" as *const u8 as *const ::core::ffi::c_char,
    b"hyphensuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamdownsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ydieresissmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"onesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"twosuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"threesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"foursuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"fivesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"sixsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"sevensuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"ninesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"zerosuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"esuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"rsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"tsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"isuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"ssuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"dsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"lsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ogoneksmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Brevesmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Macronsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"bsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"nsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"msuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"commasuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodsuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"Dotaccentsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ringsmall\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
];
const WinAnsiEncoding: [*const ::core::ffi::c_char; 256] = [
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b".notdef\0" as *const u8 as *const ::core::ffi::c_char,
    b"space\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclam\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"numbersign\0" as *const u8 as *const ::core::ffi::c_char,
    b"dollar\0" as *const u8 as *const ::core::ffi::c_char,
    b"percent\0" as *const u8 as *const ::core::ffi::c_char,
    b"ampersand\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"parenright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asterisk\0" as *const u8 as *const ::core::ffi::c_char,
    b"plus\0" as *const u8 as *const ::core::ffi::c_char,
    b"comma\0" as *const u8 as *const ::core::ffi::c_char,
    b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
    b"period\0" as *const u8 as *const ::core::ffi::c_char,
    b"slash\0" as *const u8 as *const ::core::ffi::c_char,
    b"zero\0" as *const u8 as *const ::core::ffi::c_char,
    b"one\0" as *const u8 as *const ::core::ffi::c_char,
    b"two\0" as *const u8 as *const ::core::ffi::c_char,
    b"three\0" as *const u8 as *const ::core::ffi::c_char,
    b"four\0" as *const u8 as *const ::core::ffi::c_char,
    b"five\0" as *const u8 as *const ::core::ffi::c_char,
    b"six\0" as *const u8 as *const ::core::ffi::c_char,
    b"seven\0" as *const u8 as *const ::core::ffi::c_char,
    b"eight\0" as *const u8 as *const ::core::ffi::c_char,
    b"nine\0" as *const u8 as *const ::core::ffi::c_char,
    b"colon\0" as *const u8 as *const ::core::ffi::c_char,
    b"semicolon\0" as *const u8 as *const ::core::ffi::c_char,
    b"less\0" as *const u8 as *const ::core::ffi::c_char,
    b"equal\0" as *const u8 as *const ::core::ffi::c_char,
    b"greater\0" as *const u8 as *const ::core::ffi::c_char,
    b"question\0" as *const u8 as *const ::core::ffi::c_char,
    b"at\0" as *const u8 as *const ::core::ffi::c_char,
    b"A\0" as *const u8 as *const ::core::ffi::c_char,
    b"B\0" as *const u8 as *const ::core::ffi::c_char,
    b"C\0" as *const u8 as *const ::core::ffi::c_char,
    b"D\0" as *const u8 as *const ::core::ffi::c_char,
    b"E\0" as *const u8 as *const ::core::ffi::c_char,
    b"F\0" as *const u8 as *const ::core::ffi::c_char,
    b"G\0" as *const u8 as *const ::core::ffi::c_char,
    b"H\0" as *const u8 as *const ::core::ffi::c_char,
    b"I\0" as *const u8 as *const ::core::ffi::c_char,
    b"J\0" as *const u8 as *const ::core::ffi::c_char,
    b"K\0" as *const u8 as *const ::core::ffi::c_char,
    b"L\0" as *const u8 as *const ::core::ffi::c_char,
    b"M\0" as *const u8 as *const ::core::ffi::c_char,
    b"N\0" as *const u8 as *const ::core::ffi::c_char,
    b"O\0" as *const u8 as *const ::core::ffi::c_char,
    b"P\0" as *const u8 as *const ::core::ffi::c_char,
    b"Q\0" as *const u8 as *const ::core::ffi::c_char,
    b"R\0" as *const u8 as *const ::core::ffi::c_char,
    b"S\0" as *const u8 as *const ::core::ffi::c_char,
    b"T\0" as *const u8 as *const ::core::ffi::c_char,
    b"U\0" as *const u8 as *const ::core::ffi::c_char,
    b"V\0" as *const u8 as *const ::core::ffi::c_char,
    b"W\0" as *const u8 as *const ::core::ffi::c_char,
    b"X\0" as *const u8 as *const ::core::ffi::c_char,
    b"Y\0" as *const u8 as *const ::core::ffi::c_char,
    b"Z\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"backslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"bracketright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char,
    b"underscore\0" as *const u8 as *const ::core::ffi::c_char,
    b"grave\0" as *const u8 as *const ::core::ffi::c_char,
    b"a\0" as *const u8 as *const ::core::ffi::c_char,
    b"b\0" as *const u8 as *const ::core::ffi::c_char,
    b"c\0" as *const u8 as *const ::core::ffi::c_char,
    b"d\0" as *const u8 as *const ::core::ffi::c_char,
    b"e\0" as *const u8 as *const ::core::ffi::c_char,
    b"f\0" as *const u8 as *const ::core::ffi::c_char,
    b"g\0" as *const u8 as *const ::core::ffi::c_char,
    b"h\0" as *const u8 as *const ::core::ffi::c_char,
    b"i\0" as *const u8 as *const ::core::ffi::c_char,
    b"j\0" as *const u8 as *const ::core::ffi::c_char,
    b"k\0" as *const u8 as *const ::core::ffi::c_char,
    b"l\0" as *const u8 as *const ::core::ffi::c_char,
    b"m\0" as *const u8 as *const ::core::ffi::c_char,
    b"n\0" as *const u8 as *const ::core::ffi::c_char,
    b"o\0" as *const u8 as *const ::core::ffi::c_char,
    b"p\0" as *const u8 as *const ::core::ffi::c_char,
    b"q\0" as *const u8 as *const ::core::ffi::c_char,
    b"r\0" as *const u8 as *const ::core::ffi::c_char,
    b"s\0" as *const u8 as *const ::core::ffi::c_char,
    b"t\0" as *const u8 as *const ::core::ffi::c_char,
    b"u\0" as *const u8 as *const ::core::ffi::c_char,
    b"v\0" as *const u8 as *const ::core::ffi::c_char,
    b"w\0" as *const u8 as *const ::core::ffi::c_char,
    b"x\0" as *const u8 as *const ::core::ffi::c_char,
    b"y\0" as *const u8 as *const ::core::ffi::c_char,
    b"z\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"bar\0" as *const u8 as *const ::core::ffi::c_char,
    b"braceright\0" as *const u8 as *const ::core::ffi::c_char,
    b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"Euro\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"florin\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char,
    b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char,
    b"dagger\0" as *const u8 as *const ::core::ffi::c_char,
    b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char,
    b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"perthousand\0" as *const u8 as *const ::core::ffi::c_char,
    b"Scaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"OE\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"Zcaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"quoteright\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"endash\0" as *const u8 as *const ::core::ffi::c_char,
    b"emdash\0" as *const u8 as *const ::core::ffi::c_char,
    b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"trademark\0" as *const u8 as *const ::core::ffi::c_char,
    b"scaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char,
    b"oe\0" as *const u8 as *const ::core::ffi::c_char,
    b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
    b"zcaron\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"space\0" as *const u8 as *const ::core::ffi::c_char,
    b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char,
    b"cent\0" as *const u8 as *const ::core::ffi::c_char,
    b"sterling\0" as *const u8 as *const ::core::ffi::c_char,
    b"currency\0" as *const u8 as *const ::core::ffi::c_char,
    b"yen\0" as *const u8 as *const ::core::ffi::c_char,
    b"brokenbar\0" as *const u8 as *const ::core::ffi::c_char,
    b"section\0" as *const u8 as *const ::core::ffi::c_char,
    b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char,
    b"logicalnot\0" as *const u8 as *const ::core::ffi::c_char,
    b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
    b"registered\0" as *const u8 as *const ::core::ffi::c_char,
    b"macron\0" as *const u8 as *const ::core::ffi::c_char,
    b"degree\0" as *const u8 as *const ::core::ffi::c_char,
    b"plusminus\0" as *const u8 as *const ::core::ffi::c_char,
    b"twosuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"threesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"acute\0" as *const u8 as *const ::core::ffi::c_char,
    b"mu\0" as *const u8 as *const ::core::ffi::c_char,
    b"paragraph\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
    b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"onesuperior\0" as *const u8 as *const ::core::ffi::c_char,
    b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char,
    b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char,
    b"onequarter\0" as *const u8 as *const ::core::ffi::c_char,
    b"onehalf\0" as *const u8 as *const ::core::ffi::c_char,
    b"threequarters\0" as *const u8 as *const ::core::ffi::c_char,
    b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
    b"Agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"AE\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"Egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"Odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"multiply\0" as *const u8 as *const ::core::ffi::c_char,
    b"Oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"Uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"Udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"Yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"Thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"germandbls\0" as *const u8 as *const ::core::ffi::c_char,
    b"agrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"aacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"atilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"adieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"aring\0" as *const u8 as *const ::core::ffi::c_char,
    b"ae\0" as *const u8 as *const ::core::ffi::c_char,
    b"ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
    b"egrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"eacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"edieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"igrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"iacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"idieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"eth\0" as *const u8 as *const ::core::ffi::c_char,
    b"ntilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"ograve\0" as *const u8 as *const ::core::ffi::c_char,
    b"oacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"otilde\0" as *const u8 as *const ::core::ffi::c_char,
    b"odieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"divide\0" as *const u8 as *const ::core::ffi::c_char,
    b"oslash\0" as *const u8 as *const ::core::ffi::c_char,
    b"ugrave\0" as *const u8 as *const ::core::ffi::c_char,
    b"uacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
    b"udieresis\0" as *const u8 as *const ::core::ffi::c_char,
    b"yacute\0" as *const u8 as *const ::core::ffi::c_char,
    b"thorn\0" as *const u8 as *const ::core::ffi::c_char,
    b"ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
];
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pdf_file {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_init_encodings() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_init_encodings() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_encoding_complete() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_encoding_complete() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_close_encodings() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_close_encodings() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_encoding_findresource(
    mut enc_name: *const ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_encoding_findresource(enc_name) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_encoding_get_encoding(
    mut enc_id: ::core::ffi::c_int,
    _result: *mut *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_encoding_get_encoding(enc_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_encoding_obj(
    mut enc_id: ::core::ffi::c_int,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_encoding_obj(enc_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_encoding_is_predefined(
    mut enc_id: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_encoding_is_predefined(enc_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_encoding_used_by_type3(
    mut enc_id: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_encoding_used_by_type3(enc_id) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_encoding_get_name(
    mut enc_id: ::core::ffi::c_int,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_encoding_get_name(enc_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_encoding_add_usedchars(
    mut encoding_id: ::core::ffi::c_int,
    mut is_used: *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_encoding_add_usedchars(encoding_id, is_used)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_encoding_get_tounicode(
    mut encoding_id: ::core::ffi::c_int,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_encoding_get_tounicode(encoding_id) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_create_ToUnicode_CMap(
    mut enc_name: *const ::core::ffi::c_char,
    mut enc_vec: *mut *mut ::core::ffi::c_char,
    mut is_used: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_create_ToUnicode_CMap(enc_name, enc_vec, is_used)
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
pub unsafe extern "C" fn __pdf_entry_pdf_load_ToUnicode_stream(
    mut ident: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_load_ToUnicode_stream(ident) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

pub(crate) struct DocumentState {
    enc_cache: C2RustUnnamed,
    wbuf: [::core::ffi::c_uchar; 1024],
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self {
            enc_cache: C2RustUnnamed {
                count: 0 as ::core::ffi::c_int,
                capacity: 0 as ::core::ffi::c_int,
                encodings: ::core::ptr::null_mut::<pdf_encoding>(),
            },
            wbuf: [0; 1024],
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe {
        (*crate::compat::document::current())
            .backend
            .dpx_pdfencoding
            .pointer
    }
}
