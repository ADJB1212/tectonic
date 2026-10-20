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
    fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcpy(__dst: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn sprintf(_: *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_lookup_dict"]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_stream"]
    fn pdf_new_stream(flags: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_stream"]
    fn pdf_add_stream(stream: *mut pdf_obj, stream_data_ptr: *const ::core::ffi::c_void, stream_data_len: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_get_font_data"]
    fn pdf_get_font_data(font_id: ::core::ffi::c_int) -> *mut pdf_font;
    #[link_name = "__pdf_impl_CIDFont_get_usedchars"]
    fn CIDFont_get_usedchars(font: *mut pdf_font) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_CIDFont_get_usedchars_v"]
    fn CIDFont_get_usedchars_v(font: *mut pdf_font) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_CIDFont_is_ACCFont"]
    fn CIDFont_is_ACCFont(font: *mut pdf_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CIDFont_is_UCSFont"]
    fn CIDFont_is_UCSFont(font: *mut pdf_font) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CIDFont_type0_t1create_ToUnicode_stream"]
    fn CIDFont_type0_t1create_ToUnicode_stream(
        filename: *const ::core::ffi::c_char, fontname: *const ::core::ffi::c_char, used_chars: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    static mut dpx_conf: _dpx_conf;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_otf_create_ToUnicode_stream"]
    fn otf_create_ToUnicode_stream(
        map_name: *const ::core::ffi::c_char, ttc_index: uint32_t, basefont: *const ::core::ffi::c_char, used_chars: *const ::core::ffi::c_char,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_load_ToUnicode_stream"]
    fn pdf_load_ToUnicode_stream(ident: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_defineresource"]
    fn pdf_defineresource(
        category: *const ::core::ffi::c_char, resname: *const ::core::ffi::c_char, object: *mut pdf_obj, flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_findresource"]
    fn pdf_findresource(category: *const ::core::ffi::c_char, resname: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_get_resource_reference"]
    fn pdf_get_resource_reference(res_id: ::core::ffi::c_int) -> *mut pdf_obj;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CIDSysInfo {
    pub registry: *mut ::core::ffi::c_char,
    pub ordering: *mut ::core::ffi::c_char,
    pub supplement: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cid_opt {
    pub csi: CIDSysInfo,
    pub style: ::core::ffi::c_int,
    pub embed: ::core::ffi::c_int,
    pub stemv: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_font {
    pub ident: *mut ::core::ffi::c_char,
    pub font_id: ::core::ffi::c_int,
    pub subtype: ::core::ffi::c_int,
    pub filename: *mut ::core::ffi::c_char,
    pub encoding_id: ::core::ffi::c_int,
    pub index: uint32_t,
    pub fontname: *mut ::core::ffi::c_char,
    pub uniqueID: [::core::ffi::c_char; 7],
    pub reference: *mut pdf_obj,
    pub resource: *mut pdf_obj,
    pub descriptor: *mut pdf_obj,
    pub usedchars: *mut ::core::ffi::c_char,
    pub flags: ::core::ffi::c_int,
    pub point_size: ::core::ffi::c_double,
    pub design_size: ::core::ffi::c_double,
    pub type0: C2RustUnnamed_0,
    pub cid: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub csi: CIDSysInfo,
    pub options: cid_opt,
    pub need_vmetrics: ::core::ffi::c_int,
    pub usedchars_v: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub descendant: ::core::ffi::c_int,
    pub wmode: ::core::ffi::c_int,
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
#[inline]
unsafe extern "C-unwind" fn streq_ptr(mut s1: *const ::core::ffi::c_char, mut s2: *const ::core::ffi::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
    }
    return false_0 != 0;
}
pub const STREAM_COMPRESS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PDF_FONT_FONTTYPE_CIDTYPE0: ::core::ffi::c_int = 5;
pub const PDF_FONT_FONTTYPE_CIDTYPE2: ::core::ffi::c_int = 6;
pub const PDF_FONT_FLAG_USEDCHAR_SHARED: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const CIDFONT_FLAG_TYPE1: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const CIDFONT_FLAG_TYPE1C: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn try_load_ToUnicode_file(mut cmap_base: *mut ::core::ffi::c_char) -> *mut pdf_obj {
    let mut tounicode: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cmap_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    cmap_name = new((strlen(cmap_base)
        .wrapping_add(strlen(b"-UTF16\0" as *const u8 as *const ::core::ffi::c_char))
        .wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    sprintf(cmap_name, b"%s-UTF16\0" as *const u8 as *const ::core::ffi::c_char, cmap_base);
    tounicode = pdf_read_ToUnicode_file(cmap_name);
    if tounicode.is_null() {
        sprintf(cmap_name, b"%s-UCS2\0" as *const u8 as *const ::core::ffi::c_char, cmap_base);
        tounicode = pdf_read_ToUnicode_file(cmap_name);
    }
    free(cmap_name as *mut ::core::ffi::c_void);
    return tounicode;
}
unsafe extern "C-unwind" fn Type0Font_attach_ToUnicode_stream(mut font: *mut pdf_font) {
    let mut tounicode: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cidfont: *mut pdf_font = pdf_get_font_data((*font).type0.descendant);
    let mut csi: *mut CIDSysInfo = ::core::ptr::null_mut::<CIDSysInfo>();
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if cidfont.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"cidfont\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if CIDFont_is_ACCFont(cidfont) != 0 {
        return;
    } else if CIDFont_is_UCSFont(cidfont) != 0 {
        tounicode = pdf_read_ToUnicode_file(b"Adobe-Identity-UCS2\0" as *const u8 as *const ::core::ffi::c_char);
        if tounicode.is_null() {
            tounicode = pdf_new_name(b"Identity-H\0" as *const u8 as *const ::core::ffi::c_char);
        }
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"ToUnicode\0" as *const u8 as *const ::core::ffi::c_char),
            tounicode,
        );
        return;
    }
    tounicode = ::core::ptr::null_mut::<pdf_obj>();
    csi = &raw mut (*cidfont).cid.csi;
    if (*cidfont).cid.options.embed != 0 {
        fontname = new((strlen((*cidfont).fontname).wrapping_add(8 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
        sprintf(
            fontname,
            b"%s+%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*cidfont).uniqueID as *mut ::core::ffi::c_char,
            (*cidfont).fontname,
        );
    } else {
        fontname = new((strlen((*cidfont).fontname).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
        strcpy(fontname, (*cidfont).fontname);
    }
    match (*cidfont).subtype {
        PDF_FONT_FONTTYPE_CIDTYPE2 => {
            if strcmp((*csi).registry, b"Adobe\0" as *const u8 as *const ::core::ffi::c_char) == 0
                && strcmp((*csi).ordering, b"Identity\0" as *const u8 as *const ::core::ffi::c_char) == 0
            {
                tounicode = otf_create_ToUnicode_stream((*cidfont).ident, (*cidfont).index, fontname, (*font).usedchars);
            } else {
                let mut cmap_base: *mut ::core::ffi::c_char = new((strlen((*csi).registry)
                    .wrapping_add(strlen((*csi).ordering))
                    .wrapping_add(2 as size_t) as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
                    as uint32_t) as *mut ::core::ffi::c_char;
                sprintf(
                    cmap_base,
                    b"%s-%s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*csi).registry,
                    (*csi).ordering,
                );
                tounicode = try_load_ToUnicode_file(cmap_base);
                free(cmap_base as *mut ::core::ffi::c_void);
            }
        }
        _ => {
            if (*cidfont).flags & CIDFONT_FLAG_TYPE1C != 0 {
                tounicode = otf_create_ToUnicode_stream((*cidfont).ident, (*cidfont).index, fontname, (*font).usedchars);
            } else if (*cidfont).flags & CIDFONT_FLAG_TYPE1 != 0 {
                tounicode = CIDFont_type0_t1create_ToUnicode_stream((*cidfont).ident, fontname, (*font).usedchars);
            } else {
                tounicode = try_load_ToUnicode_file((*cidfont).fontname);
                if tounicode.is_null() {
                    tounicode = otf_create_ToUnicode_stream((*cidfont).ident, (*cidfont).index, fontname, (*font).usedchars);
                }
            }
        }
    }
    free(fontname as *mut ::core::ffi::c_void);
    if !tounicode.is_null() {
        pdf_add_dict(
            (*font).resource,
            pdf_new_name(b"ToUnicode\0" as *const u8 as *const ::core::ffi::c_char),
            tounicode,
        );
    } else {
        pdf_warning!(
            b"Failed to load ToUnicode CMap for font \"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
            (*cidfont).filename,
        );
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_font_load_type0")]
pub unsafe extern "C-unwind" fn pdf_font_load_type0(mut font: *mut pdf_font) {
    if font.is_null() || (*font).reference.is_null() {
        return;
    }
    if pdf_lookup_dict((*font).resource, b"ToUnicode\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
        Type0Font_attach_ToUnicode_stream(font);
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_font_open_type0")]
pub unsafe extern "C-unwind" fn pdf_font_open_type0(
    mut font: *mut pdf_font, mut cid_id: ::core::ffi::c_int, mut wmode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cidfont: *mut pdf_font = ::core::ptr::null_mut::<pdf_font>();
    let mut csi: *mut CIDSysInfo = ::core::ptr::null_mut::<CIDSysInfo>();
    let mut fontname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if cid_id < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    cidfont = pdf_get_font_data(cid_id);
    (*font).type0.wmode = wmode;
    (*font).type0.descendant = cid_id;
    if (*cidfont).cid.options.embed != 0 {
        fontname = new((strlen((*cidfont).fontname).wrapping_add(8 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
        sprintf(
            fontname,
            b"%s+%s\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut (*cidfont).uniqueID as *mut ::core::ffi::c_char,
            (*cidfont).fontname,
        );
    } else {
        fontname = new((strlen((*cidfont).fontname).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
        strcpy(fontname, (*cidfont).fontname);
    }
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        pdf_message!(b"(CID:%s)\0" as *const u8 as *const ::core::ffi::c_char, fontname);
    }
    match (*cidfont).subtype {
        PDF_FONT_FONTTYPE_CIDTYPE0 => {
            (*font).fontname = new((strlen(fontname)
                .wrapping_add(strlen(b"Identity-V\0" as *const u8 as *const ::core::ffi::c_char))
                .wrapping_add(2 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
                as *mut ::core::ffi::c_char;
            sprintf(
                (*font).fontname,
                b"%s-%s\0" as *const u8 as *const ::core::ffi::c_char,
                fontname,
                if wmode != 0 {
                    b"Identity-V\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"Identity-H\0" as *const u8 as *const ::core::ffi::c_char
                },
            );
            (*font).usedchars = CIDFont_get_usedchars(cidfont);
            (*font).flags |= PDF_FONT_FLAG_USEDCHAR_SHARED;
            if wmode != 0 {
                (*cidfont).cid.need_vmetrics = 1 as ::core::ffi::c_int;
            }
        }
        PDF_FONT_FONTTYPE_CIDTYPE2 => {
            (*font).fontname = new((strlen(fontname).wrapping_add(1 as size_t) as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
                as *mut ::core::ffi::c_char;
            strcpy((*font).fontname, fontname);
            csi = &raw mut (*cidfont).cid.csi;
            if csi.is_null()
                || strcmp((*csi).registry, b"Adobe\0" as *const u8 as *const ::core::ffi::c_char) == 0
                    && strcmp((*csi).ordering, b"Identity\0" as *const u8 as *const ::core::ffi::c_char) == 0
            {
                (*font).usedchars = CIDFont_get_usedchars(cidfont);
                (*font).flags |= PDF_FONT_FLAG_USEDCHAR_SHARED;
            } else {
                (*font).usedchars = if wmode != 0 {
                    CIDFont_get_usedchars_v(cidfont)
                } else {
                    CIDFont_get_usedchars(cidfont)
                };
                (*font).flags |= PDF_FONT_FLAG_USEDCHAR_SHARED;
            }
            if wmode != 0 {
                (*cidfont).cid.need_vmetrics = 1 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    free(fontname as *mut ::core::ffi::c_void);
    (*font).resource = pdf_new_dict();
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Font\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"Subtype\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Type0\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"BaseFont\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name((*font).fontname),
    );
    pdf_add_dict(
        (*font).resource,
        pdf_new_name(b"Encoding\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(if wmode != 0 {
            b"Identity-V\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"Identity-H\0" as *const u8 as *const ::core::ffi::c_char
        }),
    );
    return 0 as ::core::ffi::c_int;
}
pub const PDF_RES_FLUSH_IMMEDIATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn create_dummy_CMap() -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut buf: [::core::ffi::c_char; 32] = [0; 32];
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    stream = pdf_new_stream(STREAM_COMPRESS);
    pdf_add_stream(
        stream,
        CMAP_PART0.as_ptr() as *const ::core::ffi::c_void,
        strlen(CMAP_PART0.as_ptr()) as ::core::ffi::c_int,
    );
    pdf_add_stream(
        stream,
        CMAP_PART1.as_ptr() as *const ::core::ffi::c_void,
        strlen(CMAP_PART1.as_ptr()) as ::core::ffi::c_int,
    );
    pdf_add_stream(
        stream,
        b"\n100 beginbfrange\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"\n100 beginbfrange\n\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int,
    );
    i = 0 as ::core::ffi::c_int;
    while i < 0x64 as ::core::ffi::c_int {
        n = sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"<%02X00> <%02XFF> <%02X00>\n\0" as *const u8 as *const ::core::ffi::c_char,
            i,
            i,
            i,
        );
        pdf_add_stream(stream, &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void, n);
        i += 1;
    }
    pdf_add_stream(
        stream,
        b"endbfrange\n\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"endbfrange\n\n\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int,
    );
    pdf_add_stream(
        stream,
        b"\n100 beginbfrange\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"\n100 beginbfrange\n\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int,
    );
    i = 0x64 as ::core::ffi::c_int;
    while i < 0xc8 as ::core::ffi::c_int {
        n = sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"<%02X00> <%02XFF> <%02X00>\n\0" as *const u8 as *const ::core::ffi::c_char,
            i,
            i,
            i,
        );
        pdf_add_stream(stream, &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void, n);
        i += 1;
    }
    pdf_add_stream(
        stream,
        b"endbfrange\n\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"endbfrange\n\n\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int,
    );
    pdf_add_stream(
        stream,
        b"\n48 beginbfrange\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"\n48 beginbfrange\n\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int,
    );
    i = 0xc8 as ::core::ffi::c_int;
    while i <= 0xd7 as ::core::ffi::c_int {
        n = sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"<%02X00> <%02XFF> <%02X00>\n\0" as *const u8 as *const ::core::ffi::c_char,
            i,
            i,
            i,
        );
        pdf_add_stream(stream, &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void, n);
        i += 1;
    }
    i = 0xe0 as ::core::ffi::c_int;
    while i <= 0xff as ::core::ffi::c_int {
        n = sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"<%02X00> <%02XFF> <%02X00>\n\0" as *const u8 as *const ::core::ffi::c_char,
            i,
            i,
            i,
        );
        pdf_add_stream(stream, &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void, n);
        i += 1;
    }
    pdf_add_stream(
        stream,
        b"endbfrange\n\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"endbfrange\n\n\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int,
    );
    pdf_add_stream(
        stream,
        CMAP_PART3.as_ptr() as *const ::core::ffi::c_void,
        strlen(CMAP_PART3.as_ptr()) as ::core::ffi::c_int,
    );
    return stream;
}
pub const CMAP_PART0: [::core::ffi::c_char; 250] = crate::backend::c_char_array(*b"%!PS-Adobe-3.0 Resource-CMap\n%%DocumentNeededResources: ProcSet (CIDInit)\n%%IncludeResource: ProcSet (CIDInit)\n%%BeginResource: CMap (Adobe-Identity-UCS2)\n%%Title: (Adobe-Identity-UCS2 Adobe UCS2 0)\n%%Version: 1.0\n%%Copyright:\n%% ---\n%%EndComments\n\n\0");
pub const CMAP_PART1: [::core::ffi::c_char; 300] = crate::backend::c_char_array(*b"/CIDInit /ProcSet findresource begin\n\n12 dict begin\n\nbegincmap\n\n/CIDSystemInfo 3 dict dup begin\n  /Registry (Adobe) def\n  /Ordering (UCS2) def\n  /Supplement 0 def\nend def\n\n/CMapName /Adobe-Identity-UCS2 def\n/CMapVersion 1.0 def\n/CMapType 2 def\n\n2 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n\0");
pub const CMAP_PART3: [::core::ffi::c_char; 86] =
    crate::backend::c_char_array(*b"endcmap\n\nCMapName currentdict /CMap defineresource pop\n\nend\nend\n\n%%EndResource\n%%EOF\n\0");
unsafe extern "C-unwind" fn pdf_read_ToUnicode_file(mut cmap_name: *const ::core::ffi::c_char) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut res_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if cmap_name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cmap_name\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    res_id = pdf_findresource(b"CMap\0" as *const u8 as *const ::core::ffi::c_char, cmap_name);
    if res_id < 0 as ::core::ffi::c_int {
        if streq_ptr(cmap_name, b"Adobe-Identity-UCS2\0" as *const u8 as *const ::core::ffi::c_char) {
            stream = create_dummy_CMap();
        } else {
            stream = pdf_load_ToUnicode_stream(cmap_name);
        }
        if !stream.is_null() {
            res_id = pdf_defineresource(
                b"CMap\0" as *const u8 as *const ::core::ffi::c_char,
                cmap_name,
                stream,
                PDF_RES_FLUSH_IMMEDIATE,
            );
        }
    }
    return if res_id < 0 as ::core::ffi::c_int {
        ::core::ptr::null_mut::<pdf_obj>()
    } else {
        pdf_get_resource_reference(res_id)
    };
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_load_type0(mut font: *mut pdf_font) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_load_type0(font) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_open_type0(
    mut font: *mut pdf_font, mut cid_id: ::core::ffi::c_int, mut wmode: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_open_type0(font, cid_id, wmode) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
