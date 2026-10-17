/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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
unsafe extern "C-unwind" {
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_string"]
    fn pdf_new_string(str: *const ::core::ffi::c_void, length: size_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(
        dict: *mut pdf_obj,
        key: *mut pdf_obj,
        value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_stream"]
    fn pdf_new_stream(flags: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_stream"]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const ::core::ffi::c_void,
        stream_data_len: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_stream_dict"]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    static mut CSI_IDENTITY: CIDSysInfo;
    static mut CSI_UNICODE: CIDSysInfo;
    #[link_name = "__pdf_impl_CMap_is_valid"]
    fn CMap_is_valid(cmap: *mut CMap) -> bool;
    #[link_name = "__pdf_impl_CMap_is_Identity"]
    fn CMap_is_Identity(cmap: *mut CMap) -> bool;
    #[link_name = "__pdf_impl_CMap_get_name"]
    fn CMap_get_name(cmap: *mut CMap) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_CMap_get_wmode"]
    fn CMap_get_wmode(cmap: *mut CMap) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_CMap_get_CIDSysInfo"]
    fn CMap_get_CIDSysInfo(cmap: *mut CMap) -> *mut CIDSysInfo;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_pdf_defineresource"]
    fn pdf_defineresource(
        category: *const ::core::ffi::c_char,
        resname: *const ::core::ffi::c_char,
        object: *mut pdf_obj,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_findresource"]
    fn pdf_findresource(
        category: *const ::core::ffi::c_char,
        resname: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sbuf {
    pub buf: *mut ::core::ffi::c_char,
    pub curptr: *mut ::core::ffi::c_char,
    pub limptr: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub start: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const MAP_IS_CID: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const MAP_IS_NAME: ::core::ffi::c_int = 2;
pub const MAP_IS_CODE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const MAP_IS_NOTDEF: ::core::ffi::c_int = 8;
pub const MAP_IS_UNDEF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAP_TYPE_MASK: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;
pub const MAP_LOOKUP_CONTINUE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const CMAP_DEBUG_STR: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"CMap\0") };
pub const CMAP_TYPE_IDENTITY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CMAP_TYPE_TO_UNICODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_RES_FLUSH_IMMEDIATE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn block_count(
    mut mtab: *mut mapDef,
    mut c: ::core::ffi::c_int,
) -> size_t {
    let mut count: size_t = 0 as size_t;
    let mut n: size_t = 0;
    n = (*mtab.offset(c as isize)).len.wrapping_sub(1 as size_t);
    c += 1 as ::core::ffi::c_int;
    while c < 256 as ::core::ffi::c_int {
        if (*mtab.offset(c as isize)).flag & MAP_LOOKUP_CONTINUE != 0
            || (if (*mtab.offset(c as isize)).flag & MAP_TYPE_MASK != MAP_IS_UNDEF {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) == 0
            || (*mtab.offset(c as isize)).flag & MAP_TYPE_MASK != MAP_IS_CID
                && (*mtab.offset(c as isize)).flag & MAP_TYPE_MASK != MAP_IS_CODE
            || (*mtab.offset((c - 1 as ::core::ffi::c_int) as isize)).len
                != (*mtab.offset(c as isize)).len
        {
            break;
        }
        if !(memcmp(
            (*mtab.offset((c - 1 as ::core::ffi::c_int) as isize)).code
                as *const ::core::ffi::c_void,
            (*mtab.offset(c as isize)).code as *const ::core::ffi::c_void,
            n,
        ) == 0
            && (*(*mtab.offset((c - 1 as ::core::ffi::c_int) as isize))
                .code
                .offset(n as isize) as ::core::ffi::c_int)
                < 255 as ::core::ffi::c_int
            && *(*mtab.offset((c - 1 as ::core::ffi::c_int) as isize))
                .code
                .offset(n as isize) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int
                == *(*mtab.offset(c as isize)).code.offset(n as isize) as ::core::ffi::c_int)
        {
            break;
        }
        count = count.wrapping_add(1);
        c += 1;
    }
    return count;
}
unsafe extern "C-unwind" fn sputx(
    mut c: ::core::ffi::c_uchar,
    mut s: *mut *mut ::core::ffi::c_char,
    mut end: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut hi: ::core::ffi::c_char =
        (c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as ::core::ffi::c_char;
    let mut lo: ::core::ffi::c_char =
        (c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as ::core::ffi::c_char;
    if (*s).offset(2 as ::core::ffi::c_int as isize) > end {
        pdf_abort!(b"Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    **s = (if (hi as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
        hi as ::core::ffi::c_int + '0' as i32
    } else {
        hi as ::core::ffi::c_int + '7' as i32
    }) as ::core::ffi::c_char;
    *(*s).offset(1 as ::core::ffi::c_int as isize) =
        (if (lo as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
            lo as ::core::ffi::c_int + '0' as i32
        } else {
            lo as ::core::ffi::c_int + '7' as i32
        }) as ::core::ffi::c_char;
    *s = (*s).offset(2 as ::core::ffi::c_int as isize);
    return 2 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn write_map(
    mut mtab: *mut mapDef,
    mut count: size_t,
    mut codestr: *mut ::core::ffi::c_uchar,
    mut depth: size_t,
    mut wbuf: *mut sbuf,
    mut stream: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut c: size_t = 0;
    let mut i: size_t = 0;
    let mut block_length: size_t = 0;
    let mut mtab1: *mut mapDef = ::core::ptr::null_mut::<mapDef>();
    let mut blocks: [C2RustUnnamed_1; 129] = [C2RustUnnamed_1 { start: 0, count: 0 }; 129];
    let mut num_blocks: size_t = 0 as size_t;
    c = 0 as size_t;
    while c < 256 as size_t {
        *codestr.offset(depth as isize) = (c & 0xff as size_t) as ::core::ffi::c_uchar;
        if (*mtab.offset(c as isize)).flag & MAP_LOOKUP_CONTINUE != 0 {
            mtab1 = (*mtab.offset(c as isize)).next as *mut mapDef;
            count = write_map(
                mtab1,
                count,
                codestr,
                depth.wrapping_add(1 as size_t),
                wbuf,
                stream,
            ) as size_t;
        } else if if (*mtab.offset(c as isize)).flag & MAP_TYPE_MASK != MAP_IS_UNDEF {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } != 0
        {
            match (*mtab.offset(c as isize)).flag & MAP_TYPE_MASK {
                MAP_IS_CID | MAP_IS_CODE => {
                    block_length = block_count(mtab, c as ::core::ffi::c_int);
                    if block_length >= BLOCK_LEN_MIN as size_t {
                        blocks[num_blocks as usize].start = c as ::core::ffi::c_int;
                        blocks[num_blocks as usize].count = block_length as ::core::ffi::c_int;
                        num_blocks = num_blocks.wrapping_add(1);
                        c = c.wrapping_add(block_length);
                    } else {
                        let fresh6 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh6 = '<' as i32 as ::core::ffi::c_char;
                        i = 0 as size_t;
                        while i <= depth {
                            sputx(
                                *codestr.offset(i as isize),
                                &raw mut (*wbuf).curptr,
                                (*wbuf).limptr,
                            );
                            i = i.wrapping_add(1);
                        }
                        let fresh7 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh7 = '>' as i32 as ::core::ffi::c_char;
                        let fresh8 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh8 = ' ' as i32 as ::core::ffi::c_char;
                        let fresh9 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh9 = '<' as i32 as ::core::ffi::c_char;
                        i = 0 as size_t;
                        while i < (*mtab.offset(c as isize)).len {
                            sputx(
                                *(*mtab.offset(c as isize)).code.offset(i as isize),
                                &raw mut (*wbuf).curptr,
                                (*wbuf).limptr,
                            );
                            i = i.wrapping_add(1);
                        }
                        let fresh10 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh10 = '>' as i32 as ::core::ffi::c_char;
                        let fresh11 = (*wbuf).curptr;
                        (*wbuf).curptr = (*wbuf).curptr.offset(1);
                        *fresh11 = '\n' as i32 as ::core::ffi::c_char;
                        count = count.wrapping_add(1);
                    }
                }
                MAP_IS_NAME => {
                    pdf_abort!(
                        b"%s: Unexpected error...\0" as *const u8 as *const ::core::ffi::c_char,
                        CMAP_DEBUG_STR.as_ptr(),
                    );
                }
                MAP_IS_NOTDEF => {}
                _ => {
                    pdf_abort!(
                        b"%s: Unknown mapping type: %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        CMAP_DEBUG_STR.as_ptr(),
                        (*mtab.offset(c as isize)).flag & MAP_TYPE_MASK,
                    );
                }
            }
        }
        if count >= 100 as size_t || (*wbuf).curptr >= (*wbuf).limptr {
            let mut fmt_buf: [::core::ffi::c_char; 32] = [0; 32];
            if count > 100 as size_t {
                pdf_abort!(
                    b"Unexpected error....: %zu\0" as *const u8 as *const ::core::ffi::c_char,
                    count,
                );
            }
            sprintf(
                &raw mut fmt_buf as *mut ::core::ffi::c_char,
                b"%zu beginbfchar\n\0" as *const u8 as *const ::core::ffi::c_char,
                count,
            );
            pdf_add_stream(
                stream,
                &raw mut fmt_buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(&raw mut fmt_buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
            );
            pdf_add_stream(
                stream,
                (*wbuf).buf as *const ::core::ffi::c_void,
                (*wbuf).curptr.offset_from((*wbuf).buf) as ::core::ffi::c_long
                    as ::core::ffi::c_int,
            );
            (*wbuf).curptr = (*wbuf).buf;
            pdf_add_stream(
                stream,
                b"endbfchar\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                strlen(b"endbfchar\n\0" as *const u8 as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int,
            );
            count = 0 as size_t;
        }
        c = c.wrapping_add(1);
    }
    if num_blocks > 0 as size_t {
        let mut fmt_buf_0: [::core::ffi::c_char; 35] = [0; 35];
        if count > 0 as size_t {
            sprintf(
                &raw mut fmt_buf_0 as *mut ::core::ffi::c_char,
                b"%zu beginbfchar\n\0" as *const u8 as *const ::core::ffi::c_char,
                count,
            );
            pdf_add_stream(
                stream,
                &raw mut fmt_buf_0 as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(&raw mut fmt_buf_0 as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
            );
            pdf_add_stream(
                stream,
                (*wbuf).buf as *const ::core::ffi::c_void,
                (*wbuf).curptr.offset_from((*wbuf).buf) as ::core::ffi::c_long
                    as ::core::ffi::c_int,
            );
            (*wbuf).curptr = (*wbuf).buf;
            pdf_add_stream(
                stream,
                b"endbfchar\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                strlen(b"endbfchar\n\0" as *const u8 as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int,
            );
            count = 0 as size_t;
        }
        sprintf(
            &raw mut fmt_buf_0 as *mut ::core::ffi::c_char,
            b"%zu beginbfrange\n\0" as *const u8 as *const ::core::ffi::c_char,
            num_blocks,
        );
        pdf_add_stream(
            stream,
            &raw mut fmt_buf_0 as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(&raw mut fmt_buf_0 as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
        );
        i = 0 as size_t;
        while i < num_blocks {
            let mut j: size_t = 0;
            c = blocks[i as usize].start as size_t;
            let fresh12 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh12 = '<' as i32 as ::core::ffi::c_char;
            j = 0 as size_t;
            while j < depth {
                sputx(
                    *codestr.offset(j as isize),
                    &raw mut (*wbuf).curptr,
                    (*wbuf).limptr,
                );
                j = j.wrapping_add(1);
            }
            sputx(
                c as ::core::ffi::c_uchar,
                &raw mut (*wbuf).curptr,
                (*wbuf).limptr,
            );
            let fresh13 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh13 = '>' as i32 as ::core::ffi::c_char;
            let fresh14 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh14 = ' ' as i32 as ::core::ffi::c_char;
            let fresh15 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh15 = '<' as i32 as ::core::ffi::c_char;
            j = 0 as size_t;
            while j < depth {
                sputx(
                    *codestr.offset(j as isize),
                    &raw mut (*wbuf).curptr,
                    (*wbuf).limptr,
                );
                j = j.wrapping_add(1);
            }
            sputx(
                c.wrapping_add(blocks[i as usize].count as size_t) as ::core::ffi::c_uchar,
                &raw mut (*wbuf).curptr,
                (*wbuf).limptr,
            );
            let fresh16 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh16 = '>' as i32 as ::core::ffi::c_char;
            let fresh17 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh17 = ' ' as i32 as ::core::ffi::c_char;
            let fresh18 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh18 = '<' as i32 as ::core::ffi::c_char;
            j = 0 as size_t;
            while j < (*mtab.offset(c as isize)).len {
                sputx(
                    *(*mtab.offset(c as isize)).code.offset(j as isize),
                    &raw mut (*wbuf).curptr,
                    (*wbuf).limptr,
                );
                j = j.wrapping_add(1);
            }
            let fresh19 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh19 = '>' as i32 as ::core::ffi::c_char;
            let fresh20 = (*wbuf).curptr;
            (*wbuf).curptr = (*wbuf).curptr.offset(1);
            *fresh20 = '\n' as i32 as ::core::ffi::c_char;
            i = i.wrapping_add(1);
        }
        pdf_add_stream(
            stream,
            (*wbuf).buf as *const ::core::ffi::c_void,
            (*wbuf).curptr.offset_from((*wbuf).buf) as ::core::ffi::c_long as ::core::ffi::c_int,
        );
        (*wbuf).curptr = (*wbuf).buf;
        pdf_add_stream(
            stream,
            b"endbfrange\n\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            strlen(b"endbfrange\n\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int,
        );
    }
    return count as ::core::ffi::c_int;
}
pub const BLOCK_LEN_MIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn write_string(
    mut outptr: *mut *mut ::core::ffi::c_char,
    mut endptr: *mut ::core::ffi::c_char,
    mut string_data: *const ::core::ffi::c_char,
) {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    s = string_data;
    length = (if !string_data.is_null() {
        strlen(string_data)
    } else {
        0 as size_t
    }) as ::core::ffi::c_int;
    p = *outptr;
    let fresh21 = p;
    p = p.offset(1);
    *fresh21 = '(' as i32 as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < length {
        let mut ch: ::core::ffi::c_uchar = 0;
        ch = *s.offset(i as isize) as ::core::ffi::c_uchar;
        if (ch as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
            || ch as ::core::ffi::c_int > 126 as ::core::ffi::c_int
        {
            p = p.offset(sprintf(
                p,
                b"\\%03o\0" as *const u8 as *const ::core::ffi::c_char,
                ch as ::core::ffi::c_int,
            ) as isize);
        } else {
            match ch as ::core::ffi::c_int {
                40 | 41 | 92 => {
                    let fresh22 = p;
                    p = p.offset(1);
                    *fresh22 = '\\' as i32 as ::core::ffi::c_char;
                    let fresh23 = p;
                    p = p.offset(1);
                    *fresh23 = ch as ::core::ffi::c_char;
                }
                _ => {
                    let fresh24 = p;
                    p = p.offset(1);
                    *fresh24 = ch as ::core::ffi::c_char;
                }
            }
        }
        i += 1;
    }
    let fresh25 = p;
    p = p.offset(1);
    *fresh25 = ')' as i32 as ::core::ffi::c_char;
    *outptr = p;
}
unsafe extern "C-unwind" fn write_name(
    mut outptr: *mut *mut ::core::ffi::c_char,
    mut endptr: *mut ::core::ffi::c_char,
    mut name_data: *const ::core::ffi::c_char,
) {
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    s = name_data;
    length = (if !name_data.is_null() {
        strlen(name_data)
    } else {
        0 as size_t
    }) as ::core::ffi::c_int;
    p = *outptr;
    let fresh26 = p;
    p = p.offset(1);
    *fresh26 = '/' as i32 as ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < length {
        if (*s.offset(i as isize) as ::core::ffi::c_int) < '!' as i32
            || *s.offset(i as isize) as ::core::ffi::c_int > '~' as i32
            || *s.offset(i as isize) as ::core::ffi::c_int == '#' as i32
            || (*s.offset(i as isize) as ::core::ffi::c_int == '(' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == ')' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '/' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '<' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '>' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '[' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == ']' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '{' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '}' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '%' as i32)
        {
            let fresh27 = p;
            p = p.offset(1);
            *fresh27 = '#' as i32 as ::core::ffi::c_char;
            sputx(
                *s.offset(i as isize) as ::core::ffi::c_uchar,
                &raw mut p,
                endptr,
            );
        } else {
            let fresh28 = p;
            p = p.offset(1);
            *fresh28 = *s.offset(i as isize);
        }
        i += 1;
    }
    *outptr = p;
}
pub const CMAP_BEGIN: [::core::ffi::c_char; 62] = unsafe {
    ::core::mem::transmute::<[u8; 62], [::core::ffi::c_char; 62]>(
        *b"/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n\0",
    )
};
pub const CMAP_END: [::core::ffi::c_char; 63] = unsafe {
    ::core::mem::transmute::<[u8; 63], [::core::ffi::c_char; 63]>(
        *b"endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n\0",
    )
};
#[unsafe(export_name = "__pdf_impl_CMap_create_stream")]
pub unsafe extern "C-unwind" fn CMap_create_stream(mut cmap: *mut CMap) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut csi: *mut CIDSysInfo = ::core::ptr::null_mut::<CIDSysInfo>();
    let mut wbuf: sbuf = sbuf {
        buf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        curptr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        limptr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut ranges: *mut rangeDef = ::core::ptr::null_mut::<rangeDef>();
    let mut codestr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut count: size_t = 0 as size_t;
    if cmap.is_null() || !CMap_is_valid(cmap) {
        pdf_warning!(b"Invalid CMap\0" as *const u8 as *const ::core::ffi::c_char);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if (*cmap).type_0 == CMAP_TYPE_IDENTITY {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    stream = pdf_new_stream(STREAM_COMPRESS);
    stream_dict = pdf_stream_dict(stream);
    csi = CMap_get_CIDSysInfo(cmap);
    if csi.is_null() {
        csi = if (*cmap).type_0 != CMAP_TYPE_TO_UNICODE {
            &raw mut CSI_IDENTITY
        } else {
            &raw mut CSI_UNICODE
        };
    }
    if (*cmap).type_0 != CMAP_TYPE_TO_UNICODE {
        let mut csi_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        csi_dict = pdf_new_dict();
        pdf_add_dict(
            csi_dict,
            pdf_new_name(b"Registry\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                (*csi).registry as *const ::core::ffi::c_void,
                strlen((*csi).registry),
            ),
        );
        pdf_add_dict(
            csi_dict,
            pdf_new_name(b"Ordering\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                (*csi).ordering as *const ::core::ffi::c_void,
                strlen((*csi).ordering),
            ),
        );
        pdf_add_dict(
            csi_dict,
            pdf_new_name(b"Supplement\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number((*csi).supplement as ::core::ffi::c_double),
        );
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"CMap\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"CMapName\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name((*cmap).name),
        );
        pdf_add_dict(
            stream_dict,
            pdf_new_name(b"CIDSystemInfo\0" as *const u8 as *const ::core::ffi::c_char),
            csi_dict,
        );
        if (*cmap).wmode != 0 as ::core::ffi::c_int {
            pdf_add_dict(
                stream_dict,
                pdf_new_name(b"WMode\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_new_number((*cmap).wmode as ::core::ffi::c_double),
            );
        }
    }
    if !(*cmap).useCMap.is_null() {
        pdf_abort!(
            b"UseCMap found (not supported yet)...\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    wbuf.buf = new((40960 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    codestr = new(((*cmap).profile.maxBytesIn as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    memset(
        codestr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (*cmap).profile.maxBytesIn,
    );
    wbuf.curptr = wbuf.buf;
    wbuf.limptr = wbuf
        .buf
        .offset(WBUF_SIZE as isize)
        .offset(
            -((2 as size_t).wrapping_mul(
                (*cmap)
                    .profile
                    .maxBytesIn
                    .wrapping_add((*cmap).profile.maxBytesOut),
            ) as isize),
        )
        .offset(16 as ::core::ffi::c_int as isize);
    pdf_add_stream(
        stream,
        CMAP_BEGIN.as_ptr() as *const ::core::ffi::c_void,
        strlen(CMAP_BEGIN.as_ptr()) as ::core::ffi::c_int,
    );
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"/CMapName \0" as *const u8 as *const ::core::ffi::c_char,
    ) as isize);
    write_name(&raw mut wbuf.curptr, wbuf.limptr, (*cmap).name);
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b" def\n\0" as *const u8 as *const ::core::ffi::c_char,
    ) as isize);
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"/CMapType %d def\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*cmap).type_0,
    ) as isize);
    if (*cmap).wmode != 0 as ::core::ffi::c_int && (*cmap).type_0 != CMAP_TYPE_TO_UNICODE {
        wbuf.curptr = wbuf.curptr.offset(sprintf(
            wbuf.curptr,
            b"/WMode %d def\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*cmap).wmode,
        ) as isize);
    }
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"/CIDSystemInfo <<\n\0" as *const u8 as *const ::core::ffi::c_char,
    ) as isize);
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"  /Registry \0" as *const u8 as *const ::core::ffi::c_char,
    ) as isize);
    write_string(&raw mut wbuf.curptr, wbuf.limptr, (*csi).registry);
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
    ) as isize);
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"  /Ordering \0" as *const u8 as *const ::core::ffi::c_char,
    ) as isize);
    write_string(&raw mut wbuf.curptr, wbuf.limptr, (*csi).ordering);
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
    ) as isize);
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"  /Supplement %d\n>> def\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*csi).supplement,
    ) as isize);
    pdf_add_stream(
        stream,
        wbuf.buf as *const ::core::ffi::c_void,
        wbuf.curptr.offset_from(wbuf.buf) as ::core::ffi::c_long as ::core::ffi::c_int,
    );
    wbuf.curptr = wbuf.buf;
    ranges = (*cmap).codespace.ranges as *mut rangeDef;
    wbuf.curptr = wbuf.curptr.offset(sprintf(
        wbuf.curptr,
        b"%d begincodespacerange\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*cmap).codespace.num,
    ) as isize);
    i = 0 as size_t;
    while i < (*cmap).codespace.num as size_t {
        let fresh0 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh0 = '<' as i32 as ::core::ffi::c_char;
        j = 0 as size_t;
        while j < (*ranges.offset(i as isize)).dim {
            sputx(
                *(*ranges.offset(i as isize)).codeLo.offset(j as isize),
                &raw mut wbuf.curptr,
                wbuf.limptr,
            );
            j = j.wrapping_add(1);
        }
        let fresh1 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh1 = '>' as i32 as ::core::ffi::c_char;
        let fresh2 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh2 = ' ' as i32 as ::core::ffi::c_char;
        let fresh3 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh3 = '<' as i32 as ::core::ffi::c_char;
        j = 0 as size_t;
        while j < (*ranges.offset(i as isize)).dim {
            sputx(
                *(*ranges.offset(i as isize)).codeHi.offset(j as isize),
                &raw mut wbuf.curptr,
                wbuf.limptr,
            );
            j = j.wrapping_add(1);
        }
        let fresh4 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh4 = '>' as i32 as ::core::ffi::c_char;
        let fresh5 = wbuf.curptr;
        wbuf.curptr = wbuf.curptr.offset(1);
        *fresh5 = '\n' as i32 as ::core::ffi::c_char;
        i = i.wrapping_add(1);
    }
    pdf_add_stream(
        stream,
        wbuf.buf as *const ::core::ffi::c_void,
        wbuf.curptr.offset_from(wbuf.buf) as ::core::ffi::c_long as ::core::ffi::c_int,
    );
    wbuf.curptr = wbuf.buf;
    pdf_add_stream(
        stream,
        b"endcodespacerange\n\0" as *const u8 as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        strlen(b"endcodespacerange\n\0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int,
    );
    if !(*cmap).mapTbl.is_null() {
        count = write_map(
            (*cmap).mapTbl,
            0 as size_t,
            codestr,
            0 as size_t,
            &raw mut wbuf,
            stream,
        ) as size_t;
        if count > 0 as size_t {
            let mut fmt_buf: [::core::ffi::c_char; 32] = [0; 32];
            if count > 100 as size_t {
                pdf_abort!(
                    b"Unexpected error....: %zu\0" as *const u8 as *const ::core::ffi::c_char,
                    count,
                );
            }
            sprintf(
                &raw mut fmt_buf as *mut ::core::ffi::c_char,
                b"%zu beginbfchar\n\0" as *const u8 as *const ::core::ffi::c_char,
                count,
            );
            pdf_add_stream(
                stream,
                &raw mut fmt_buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(&raw mut fmt_buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int,
            );
            pdf_add_stream(
                stream,
                wbuf.buf as *const ::core::ffi::c_void,
                wbuf.curptr.offset_from(wbuf.buf) as ::core::ffi::c_long as ::core::ffi::c_int,
            );
            pdf_add_stream(
                stream,
                b"endbfchar\n\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                strlen(b"endbfchar\n\0" as *const u8 as *const ::core::ffi::c_char)
                    as ::core::ffi::c_int,
            );
            count = 0 as size_t;
            wbuf.curptr = wbuf.buf;
        }
    }
    pdf_add_stream(
        stream,
        CMAP_END.as_ptr() as *const ::core::ffi::c_void,
        strlen(CMAP_END.as_ptr()) as ::core::ffi::c_int,
    );
    free(codestr as *mut ::core::ffi::c_void);
    free(wbuf.buf as *mut ::core::ffi::c_void);
    return stream;
}
pub const WBUF_SIZE: ::core::ffi::c_int = 40960 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_CMap_create_stream(
    mut cmap: *mut CMap,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { CMap_create_stream(cmap) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
