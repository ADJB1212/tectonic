/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_seek(handle: rust_input_handle_t, offset: ssize_t, whence: ::core::ffi::c_int) -> size_t;
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut ::core::ffi::c_char, len: size_t) -> ssize_t;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_tt_get_unsigned_quad"]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_stream"]
    fn pdf_new_stream(flags: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_stream"]
    fn pdf_add_stream(stream: *mut pdf_obj, stream_data_ptr: *const ::core::ffi::c_void, stream_data_len: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_stream_dict"]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
pub type rust_input_handle_t = uintptr_t;
pub type BYTE = ::core::ffi::c_uchar;
pub type USHORT = ::core::ffi::c_ushort;
pub type SFNT_ULONG = uint32_t;
pub type SFNT_LONG = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table {
    pub tag: [::core::ffi::c_char; 4],
    pub check_sum: SFNT_ULONG,
    pub offset: SFNT_ULONG,
    pub length: SFNT_ULONG,
    pub data: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt_table_directory {
    pub version: SFNT_ULONG,
    pub num_tables: USHORT,
    pub search_range: USHORT,
    pub entry_selector: USHORT,
    pub range_shift: USHORT,
    pub num_kept_tables: USHORT,
    pub flags: *mut ::core::ffi::c_char,
    pub tables: *mut sfnt_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfnt {
    pub type_0: ::core::ffi::c_int,
    pub directory: *mut sfnt_table_directory,
    pub handle: rust_input_handle_t,
    pub offset: SFNT_ULONG,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
pub const STREAM_COMPRESS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const SFNT_TABLE_REQUIRED: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const SFNT_TYPE_TRUETYPE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const SFNT_TYPE_OPENTYPE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const SFNT_TYPE_POSTSCRIPT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const SFNT_TYPE_TTC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const SFNT_TYPE_DFONT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SFNT_TRUETYPE: ::core::ffi::c_ulong = 0x10000 as ::core::ffi::c_ulong;
pub const SFNT_MAC_TRUE: ::core::ffi::c_ulong = 0x74727565 as ::core::ffi::c_ulong;
pub const SFNT_OPENTYPE: ::core::ffi::c_ulong = 0x10000 as ::core::ffi::c_ulong;
pub const SFNT_POSTSCRIPT: ::core::ffi::c_ulong = 0x4f54544f as ::core::ffi::c_ulong;
pub const SFNT_TTC: ::core::ffi::c_ulong = 0x74746366 as ::core::ffi::c_ulong;
#[unsafe(export_name = "__pdf_impl_sfnt_open")]
pub unsafe extern "C-unwind" fn sfnt_open(mut handle: rust_input_handle_t) -> *mut sfnt {
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut type_0: SFNT_ULONG = 0;
    if (handle == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!("assertion failed: handle");
    } else {
    };
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    sfont = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<sfnt>() as usize) as uint32_t) as *mut sfnt;
    (*sfont).handle = handle;
    type_0 = tt_get_unsigned_quad((*sfont).handle);
    if type_0 as ::core::ffi::c_ulong == SFNT_TRUETYPE || type_0 as ::core::ffi::c_ulong == SFNT_MAC_TRUE {
        (*sfont).type_0 = SFNT_TYPE_TRUETYPE;
    } else if type_0 as ::core::ffi::c_ulong == SFNT_OPENTYPE {
        (*sfont).type_0 = SFNT_TYPE_OPENTYPE;
    } else if type_0 as ::core::ffi::c_ulong == SFNT_POSTSCRIPT {
        (*sfont).type_0 = SFNT_TYPE_POSTSCRIPT;
    } else if type_0 as ::core::ffi::c_ulong == SFNT_TTC {
        (*sfont).type_0 = SFNT_TYPE_TTC;
    }
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    (*sfont).directory = ::core::ptr::null_mut::<sfnt_table_directory>();
    (*sfont).offset = 0 as SFNT_ULONG;
    return sfont;
}
#[unsafe(export_name = "__pdf_impl_dfont_open")]
pub unsafe extern "C-unwind" fn dfont_open(mut handle: rust_input_handle_t, mut index: ::core::ffi::c_int) -> *mut sfnt {
    let mut sfont: *mut sfnt = ::core::ptr::null_mut::<sfnt>();
    let mut rdata_pos: SFNT_ULONG = 0;
    let mut map_pos: SFNT_ULONG = 0;
    let mut tags_pos: SFNT_ULONG = 0;
    let mut types_pos: SFNT_ULONG = 0;
    let mut res_pos: SFNT_ULONG = 0;
    let mut tag: SFNT_ULONG = 0;
    let mut tags_num: USHORT = 0;
    let mut types_num: USHORT = 0;
    let mut i: USHORT = 0;
    if (handle == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!("assertion failed: handle");
    } else {
    };
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    sfont = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<sfnt>() as usize) as uint32_t) as *mut sfnt;
    (*sfont).handle = handle;
    rdata_pos = tt_get_unsigned_quad((*sfont).handle);
    map_pos = tt_get_unsigned_quad((*sfont).handle);
    crate::backend::runtime::input_seek((*sfont).handle, map_pos.wrapping_add(0x18 as SFNT_ULONG) as ssize_t, SEEK_SET);
    tags_pos = map_pos.wrapping_add(tt_get_unsigned_pair((*sfont).handle) as SFNT_ULONG);
    crate::backend::runtime::input_seek((*sfont).handle, tags_pos as ssize_t, SEEK_SET);
    tags_num = tt_get_unsigned_pair((*sfont).handle);
    i = 0 as USHORT;
    while i as ::core::ffi::c_int <= tags_num as ::core::ffi::c_int {
        tag = tt_get_unsigned_quad((*sfont).handle);
        types_num = tt_get_unsigned_pair((*sfont).handle);
        types_pos = tags_pos.wrapping_add(tt_get_unsigned_pair((*sfont).handle) as SFNT_ULONG);
        if tag as ::core::ffi::c_ulong == 0x73666e74 as ::core::ffi::c_ulong {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i as ::core::ffi::c_int > tags_num as ::core::ffi::c_int {
        free(sfont as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<sfnt>();
    }
    crate::backend::runtime::input_seek((*sfont).handle, types_pos as ssize_t, SEEK_SET);
    if index > types_num as ::core::ffi::c_int {
        pdf_abort!("Invalid index {index} for dfont.");
    }
    i = 0 as USHORT;
    while i as ::core::ffi::c_int <= types_num as ::core::ffi::c_int {
        tt_get_unsigned_pair((*sfont).handle);
        tt_get_unsigned_pair((*sfont).handle);
        res_pos = tt_get_unsigned_quad((*sfont).handle);
        tt_get_unsigned_quad((*sfont).handle);
        if i as ::core::ffi::c_int == index {
            break;
        }
        i = i.wrapping_add(1);
    }
    crate::backend::runtime::input_seek((*sfont).handle, 0 as ssize_t, SEEK_SET);
    (*sfont).type_0 = SFNT_TYPE_DFONT;
    (*sfont).directory = ::core::ptr::null_mut::<sfnt_table_directory>();
    (*sfont).offset = (res_pos as ::core::ffi::c_ulong & 0xffffff as ::core::ffi::c_ulong)
        .wrapping_add(rdata_pos as ::core::ffi::c_ulong)
        .wrapping_add(4 as ::core::ffi::c_ulong) as SFNT_ULONG;
    return sfont;
}
unsafe extern "C-unwind" fn release_directory(mut td: *mut sfnt_table_directory) {
    let mut i: ::core::ffi::c_uint = 0;
    if !td.is_null() {
        if !(*td).tables.is_null() {
            i = 0 as ::core::ffi::c_uint;
            while i < (*td).num_tables as ::core::ffi::c_uint {
                free((*(*td).tables.offset(i as isize)).data as *mut ::core::ffi::c_void);
                i = i.wrapping_add(1);
            }
            free((*td).tables as *mut ::core::ffi::c_void);
        }
        free((*td).flags as *mut ::core::ffi::c_void);
        free(td as *mut ::core::ffi::c_void);
    }
}
#[unsafe(export_name = "__pdf_impl_sfnt_close")]
pub unsafe extern "C-unwind" fn sfnt_close(mut sfont: *mut sfnt) {
    if !sfont.is_null() {
        if !(*sfont).directory.is_null() {
            release_directory((*sfont).directory);
        }
        free(sfont as *mut ::core::ffi::c_void);
    }
}
#[unsafe(export_name = "__pdf_impl_put_big_endian")]
pub unsafe extern "C-unwind" fn put_big_endian(mut s: *mut ::core::ffi::c_void, mut q: SFNT_LONG, mut n: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if n <= 0 {
        return n;
    }
    // The C API guarantees n writable bytes and supplies no separate capacity.
    let length = n as usize;
    let count = length.min(4);
    let mut bytes = [0; 4];
    crate::numbers::write_big_endian(q, &mut bytes[..count]);
    unsafe {
        core::ptr::write_bytes(s.cast::<u8>(), if q < 0 { 0xff } else { 0 }, length - count);
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), s.cast::<u8>().add(length - count), count);
    }
    n
}
extern "C-unwind" fn max2floor(mut n: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let mut val: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while n > 1 as ::core::ffi::c_uint {
        n = n.wrapping_div(2 as ::core::ffi::c_uint);
        val *= 2 as ::core::ffi::c_int;
    }
    return val as ::core::ffi::c_uint;
}
extern "C-unwind" fn log2floor(mut n: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    let mut val: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while n > 1 as ::core::ffi::c_uint {
        n = n.wrapping_div(2 as ::core::ffi::c_uint);
        val = val.wrapping_add(1);
    }
    return val;
}
unsafe extern "C-unwind" fn sfnt_calc_checksum(mut data: *mut ::core::ffi::c_void, mut length: SFNT_ULONG) -> SFNT_ULONG {
    // SFNT table storage remains owned by the translated document state.
    let bytes = if length == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(data.cast::<u8>(), length as usize) }
    };
    crate::sfnt::checksum(bytes)
}
unsafe extern "C-unwind" fn find_table_index(mut td: *mut sfnt_table_directory, mut tag: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut idx: ::core::ffi::c_int = 0;
    if td.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    idx = 0 as ::core::ffi::c_int;
    while idx < (*td).num_tables as ::core::ffi::c_int {
        if memcmp(
            &raw mut (*(*td).tables.offset(idx as isize)).tag as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            tag as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0
        {
            return idx;
        }
        idx += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_sfnt_set_table")]
pub unsafe extern "C-unwind" fn sfnt_set_table(
    mut sfont: *mut sfnt, mut tag: *const ::core::ffi::c_char, mut data: *mut ::core::ffi::c_void, mut length: SFNT_ULONG,
) {
    let mut td: *mut sfnt_table_directory = ::core::ptr::null_mut::<sfnt_table_directory>();
    let mut idx: ::core::ffi::c_int = 0;
    if sfont.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!("assertion failed: sfont");
    } else {
    };
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0 as ::core::ffi::c_int {
        idx = (*td).num_tables as ::core::ffi::c_int;
        (*td).num_tables = (*td).num_tables.wrapping_add(1);
        (*td).tables = renew(
            (*td).tables as *mut ::core::ffi::c_void,
            ((*td).num_tables as uint32_t as usize).wrapping_mul(::core::mem::size_of::<sfnt_table>() as usize) as uint32_t,
        ) as *mut sfnt_table;
        memcpy(
            &raw mut (*(*td).tables.offset(idx as isize)).tag as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            tag as *const ::core::ffi::c_void,
            4 as size_t,
        );
    }
    (*(*td).tables.offset(idx as isize)).check_sum = sfnt_calc_checksum(data, length);
    (*(*td).tables.offset(idx as isize)).offset = 0 as SFNT_ULONG;
    (*(*td).tables.offset(idx as isize)).length = length;
    let ref mut fresh1 = (*(*td).tables.offset(idx as isize)).data;
    *fresh1 = data as *mut ::core::ffi::c_char;
}
#[unsafe(export_name = "__pdf_impl_sfnt_find_table_len")]
pub unsafe extern "C-unwind" fn sfnt_find_table_len(mut sfont: *mut sfnt, mut tag: *const ::core::ffi::c_char) -> SFNT_ULONG {
    let mut length: SFNT_ULONG = 0;
    let mut td: *mut sfnt_table_directory = ::core::ptr::null_mut::<sfnt_table_directory>();
    let mut idx: ::core::ffi::c_int = 0;
    if !(!sfont.is_null() && !tag.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!("assertion failed: sfont && tag");
    } else {
    };
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0 as ::core::ffi::c_int {
        length = 0 as SFNT_ULONG;
    } else {
        length = (*(*td).tables.offset(idx as isize)).length;
    }
    return length;
}
#[unsafe(export_name = "__pdf_impl_sfnt_find_table_pos")]
pub unsafe extern "C-unwind" fn sfnt_find_table_pos(mut sfont: *mut sfnt, mut tag: *const ::core::ffi::c_char) -> SFNT_ULONG {
    let mut offset: SFNT_ULONG = 0;
    let mut td: *mut sfnt_table_directory = ::core::ptr::null_mut::<sfnt_table_directory>();
    let mut idx: ::core::ffi::c_int = 0;
    if !(!sfont.is_null() && !tag.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!("assertion failed: sfont && tag");
    } else {
    };
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0 as ::core::ffi::c_int {
        offset = 0 as SFNT_ULONG;
    } else {
        offset = (*(*td).tables.offset(idx as isize)).offset;
    }
    return offset;
}
#[unsafe(export_name = "__pdf_impl_sfnt_locate_table")]
pub unsafe extern "C-unwind" fn sfnt_locate_table(mut sfont: *mut sfnt, mut tag: *const ::core::ffi::c_char) -> SFNT_ULONG {
    let mut offset: SFNT_ULONG = 0;
    if !(!sfont.is_null() && !tag.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!("assertion failed: sfont && tag");
    } else {
    };
    offset = sfnt_find_table_pos(sfont, tag);
    if offset == 0 as SFNT_ULONG {
        pdf_abort!(b"sfnt: table not found...\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    crate::backend::runtime::input_seek((*sfont).handle, offset as ssize_t, SEEK_SET);
    return offset;
}
#[unsafe(export_name = "__pdf_impl_sfnt_read_table_directory")]
pub unsafe extern "C-unwind" fn sfnt_read_table_directory(mut sfont: *mut sfnt, mut offset: SFNT_ULONG) -> ::core::ffi::c_int {
    let mut td: *mut sfnt_table_directory = ::core::ptr::null_mut::<sfnt_table_directory>();
    let mut i: ::core::ffi::c_uint = 0;
    if sfont.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!("assertion failed: sfont");
    } else {
    };
    if !(*sfont).directory.is_null() {
        release_directory((*sfont).directory);
        (*sfont).directory = core::ptr::null_mut();
    }
    td = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<sfnt_table_directory>() as usize) as uint32_t)
        as *mut sfnt_table_directory;
    core::ptr::write_bytes(td, 0, 1);
    (*sfont).directory = td;
    if ((*sfont).handle == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!("assertion failed: sfont->handle");
    } else {
    };
    crate::backend::runtime::input_seek((*sfont).handle, offset as ssize_t, SEEK_SET);
    let mut input = crate::compat::input::DirectoryInput { handle: (*sfont).handle };
    let header = crate::backend::runtime::legacy_result(crate::sfnt::DirectoryHeader::read(&mut input, u64::from(offset)));
    (*td).version = header.version;
    (*td).num_tables = header.num_tables;
    (*td).search_range = header.search_range;
    (*td).entry_selector = header.entry_selector;
    (*td).range_shift = header.range_shift;
    (*td).flags = new(((*td).num_tables as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t)
        as *mut ::core::ffi::c_char;
    (*td).tables =
        new(((*td).num_tables as uint32_t as usize).wrapping_mul(::core::mem::size_of::<sfnt_table>() as usize) as uint32_t) as *mut sfnt_table;
    core::ptr::write_bytes((*td).tables, 0, usize::from((*td).num_tables));
    i = 0 as ::core::ffi::c_uint;
    while i < (*td).num_tables as ::core::ffi::c_uint {
        let record = crate::backend::runtime::legacy_result(crate::sfnt::TableRecord::read(&mut input, u64::from(offset) + 12 + u64::from(i) * 16));
        (*(*td).tables.offset(i as isize)).tag = record.tag.map(|byte| byte as ::core::ffi::c_char);
        (*(*td).tables.offset(i as isize)).check_sum = record.checksum;
        (*(*td).tables.offset(i as isize)).offset = record.offset.wrapping_add((*sfont).offset);
        (*(*td).tables.offset(i as isize)).length = record.length;
        let ref mut fresh0 = (*(*td).tables.offset(i as isize)).data;
        *fresh0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        *(*td).flags.offset(i as isize) = 0 as ::core::ffi::c_char;
        i = i.wrapping_add(1);
    }
    (*td).num_kept_tables = 0 as USHORT;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_sfnt_require_table")]
pub unsafe extern "C-unwind" fn sfnt_require_table(
    mut sfont: *mut sfnt, mut tag: *const ::core::ffi::c_char, mut must_exist: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut td: *mut sfnt_table_directory = ::core::ptr::null_mut::<sfnt_table_directory>();
    let mut idx: ::core::ffi::c_int = 0;
    if !(!sfont.is_null() && !(*sfont).directory.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!("assertion failed: sfont && sfont->directory");
    } else {
    };
    td = (*sfont).directory;
    idx = find_table_index(td, tag);
    if idx < 0 as ::core::ffi::c_int {
        if must_exist != 0 {
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        let ref mut fresh2 = *(*td).flags.offset(idx as isize);
        *fresh2 = (*fresh2 as ::core::ffi::c_int | SFNT_TABLE_REQUIRED) as ::core::ffi::c_char;
        (*td).num_kept_tables = (*td).num_kept_tables.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}

const PADBYTES: [::core::ffi::c_uchar; 4] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[unsafe(export_name = "__pdf_impl_sfnt_create_FontFile_stream")]
pub unsafe extern "C-unwind" fn sfnt_create_FontFile_stream(mut sfont: *mut sfnt) -> *mut pdf_obj {
    let __pdf_state = unsafe { document_state() };
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut td: *mut sfnt_table_directory = ::core::ptr::null_mut::<sfnt_table_directory>();
    let mut offset: ::core::ffi::c_int = 0;
    let mut nb_read: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut sr: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(!sfont.is_null() && !(*sfont).directory.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!("assertion failed: sfont && sfont->directory");
    } else {
    };
    stream = pdf_new_stream(STREAM_COMPRESS);
    td = (*sfont).directory;
    p = &raw mut (*__pdf_state).wbuf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char;
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*td).version as SFNT_LONG, 4 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, (*td).num_kept_tables as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    sr = max2floor((*td).num_kept_tables as ::core::ffi::c_uint).wrapping_mul(16 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, sr as SFNT_LONG, 2 as ::core::ffi::c_int) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        log2floor((*td).num_kept_tables as ::core::ffi::c_uint) as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    p = p.offset(put_big_endian(
        p as *mut ::core::ffi::c_void,
        (*td).num_kept_tables as SFNT_LONG * 16 as SFNT_LONG - sr as SFNT_LONG,
        2 as ::core::ffi::c_int,
    ) as isize);
    pdf_add_stream(
        stream,
        &raw mut (*__pdf_state).wbuf as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        12 as ::core::ffi::c_int,
    );
    offset = 12 as ::core::ffi::c_int + 16 as ::core::ffi::c_int * (*td).num_kept_tables as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*td).num_tables as ::core::ffi::c_int {
        if *(*td).flags.offset(i as isize) as ::core::ffi::c_int & SFNT_TABLE_REQUIRED != 0 {
            if offset % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                offset += 4 as ::core::ffi::c_int - offset % 4 as ::core::ffi::c_int;
            }
            p = &raw mut (*__pdf_state).wbuf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char;
            memcpy(
                p as *mut ::core::ffi::c_void,
                &raw mut (*(*td).tables.offset(i as isize)).tag as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                4 as size_t,
            );
            p = p.offset(4 as ::core::ffi::c_int as isize);
            p = p.offset(put_big_endian(
                p as *mut ::core::ffi::c_void,
                (*(*td).tables.offset(i as isize)).check_sum as SFNT_LONG,
                4 as ::core::ffi::c_int,
            ) as isize);
            p = p.offset(put_big_endian(p as *mut ::core::ffi::c_void, offset as SFNT_LONG, 4 as ::core::ffi::c_int) as isize);
            p = p.offset(put_big_endian(
                p as *mut ::core::ffi::c_void,
                (*(*td).tables.offset(i as isize)).length as SFNT_LONG,
                4 as ::core::ffi::c_int,
            ) as isize);
            pdf_add_stream(
                stream,
                &raw mut (*__pdf_state).wbuf as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
            );
            offset = (offset as SFNT_ULONG).wrapping_add((*(*td).tables.offset(i as isize)).length) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        i += 1;
    }
    offset = 12 as ::core::ffi::c_int + 16 as ::core::ffi::c_int * (*td).num_kept_tables as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*td).num_tables as ::core::ffi::c_int {
        if *(*td).flags.offset(i as isize) as ::core::ffi::c_int & SFNT_TABLE_REQUIRED != 0 {
            if offset % 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                length = 4 as ::core::ffi::c_int - offset % 4 as ::core::ffi::c_int;
                pdf_add_stream(stream, PADBYTES.as_ptr() as *const ::core::ffi::c_void, length);
                offset += length;
            }
            if (*(*td).tables.offset(i as isize)).data.is_null() {
                if (*sfont).handle == 0 {
                    pdf_release_obj(stream);
                    pdf_abort!(b"Font file not opened or already closed...\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                length = (*(*td).tables.offset(i as isize)).length as ::core::ffi::c_int;
                crate::backend::runtime::input_seek((*sfont).handle, (*(*td).tables.offset(i as isize)).offset as ssize_t, SEEK_SET);
                while length > 0 as ::core::ffi::c_int {
                    nb_read = ttstub_input_read(
                        (*sfont).handle,
                        &raw mut (*__pdf_state).wbuf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
                        (if length < 1024 as ::core::ffi::c_int {
                            length
                        } else {
                            1024 as ::core::ffi::c_int
                        }) as size_t,
                    ) as ::core::ffi::c_int;
                    if nb_read < 0 as ::core::ffi::c_int {
                        pdf_release_obj(stream);
                        pdf_abort!(b"Reading file failed...\0" as *const u8 as *const ::core::ffi::c_char,);
                    } else if nb_read > 0 as ::core::ffi::c_int {
                        pdf_add_stream(
                            stream,
                            &raw mut (*__pdf_state).wbuf as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                            nb_read,
                        );
                    }
                    length -= nb_read;
                }
            } else {
                pdf_add_stream(
                    stream,
                    (*(*td).tables.offset(i as isize)).data as *const ::core::ffi::c_void,
                    (*(*td).tables.offset(i as isize)).length as ::core::ffi::c_int,
                );
                let ref mut fresh3 = (*(*td).tables.offset(i as isize)).data;
                *fresh3 = mfree((*(*td).tables.offset(i as isize)).data as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
            }
            offset = (offset as SFNT_ULONG).wrapping_add((*(*td).tables.offset(i as isize)).length) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        i += 1;
    }
    stream_dict = pdf_stream_dict(stream);
    pdf_add_dict(
        stream_dict,
        pdf_new_name(b"Length1\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(offset as ::core::ffi::c_double),
    );
    return stream;
}
unsafe extern "C" {
    fn memcmp(__s1: *const ::core::ffi::c_void, __s2: *const ::core::ffi::c_void, __n: size_t) -> ::core::ffi::c_int;
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_sfnt_open(mut handle: rust_input_handle_t, _result: *mut *mut sfnt) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sfnt_open(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_dfont_open(mut handle: rust_input_handle_t, mut index: ::core::ffi::c_int, _result: *mut *mut sfnt) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dfont_open(handle, index) }) {
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
pub unsafe extern "C" fn __pdf_entry_sfnt_close(mut sfont: *mut sfnt) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sfnt_close(sfont) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_put_big_endian(
    mut s: *mut ::core::ffi::c_void, mut q: SFNT_LONG, mut n: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { put_big_endian(s, q, n) }) {
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
pub unsafe extern "C" fn __pdf_entry_sfnt_set_table(
    mut sfont: *mut sfnt, mut tag: *const ::core::ffi::c_char, mut data: *mut ::core::ffi::c_void, mut length: SFNT_ULONG,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sfnt_set_table(sfont, tag, data, length) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_sfnt_find_table_len(mut sfont: *mut sfnt, mut tag: *const ::core::ffi::c_char, _result: *mut SFNT_ULONG) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sfnt_find_table_len(sfont, tag) }) {
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
pub unsafe extern "C" fn __pdf_entry_sfnt_find_table_pos(mut sfont: *mut sfnt, mut tag: *const ::core::ffi::c_char, _result: *mut SFNT_ULONG) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sfnt_find_table_pos(sfont, tag) }) {
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
pub unsafe extern "C" fn __pdf_entry_sfnt_locate_table(mut sfont: *mut sfnt, mut tag: *const ::core::ffi::c_char, _result: *mut SFNT_ULONG) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sfnt_locate_table(sfont, tag) }) {
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
pub unsafe extern "C" fn __pdf_entry_sfnt_read_table_directory(
    mut sfont: *mut sfnt, mut offset: SFNT_ULONG, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sfnt_read_table_directory(sfont, offset) }) {
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
pub unsafe extern "C" fn __pdf_entry_sfnt_require_table(
    mut sfont: *mut sfnt, mut tag: *const ::core::ffi::c_char, mut must_exist: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sfnt_require_table(sfont, tag, must_exist) }) {
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
pub unsafe extern "C" fn __pdf_entry_sfnt_create_FontFile_stream(mut sfont: *mut sfnt, _result: *mut *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { sfnt_create_FontFile_stream(sfont) }) {
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
    wbuf: [::core::ffi::c_uchar; 1024],
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self { wbuf: [0; 1024] }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe { (*crate::compat::document::current()).backend.dpx_sfnt.pointer }
}
