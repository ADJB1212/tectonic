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
    fn xstrdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> ssize_t;
    #[link_name = "__pdf_impl_tt_get_unsigned_byte"]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_tt_get_signed_pair"]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> ::core::ffi::c_short;
    #[link_name = "__pdf_impl_tt_get_unsigned_quad"]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    #[link_name = "__pdf_impl_sfnt_locate_table"]
    fn sfnt_locate_table(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    #[link_name = "__pdf_impl_sfnt_find_table_len"]
    fn sfnt_find_table_len(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
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
pub type BYTE = ::core::ffi::c_uchar;
pub type USHORT = ::core::ffi::c_ushort;
pub type SHORT = ::core::ffi::c_short;
pub type SFNT_ULONG = uint32_t;
pub type Fixed = uint32_t;
pub type FWord = ::core::ffi::c_short;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_post_table {
    pub Version: Fixed,
    pub italicAngle: Fixed,
    pub underlinePosition: FWord,
    pub underlineThickness: FWord,
    pub isFixedPitch: SFNT_ULONG,
    pub minMemType42: SFNT_ULONG,
    pub maxMemType42: SFNT_ULONG,
    pub minMemType1: SFNT_ULONG,
    pub maxMemType1: SFNT_ULONG,
    pub numberOfGlyphs: USHORT,
    pub glyphNamePtr: *mut *const ::core::ffi::c_char,
    pub names: *mut *mut ::core::ffi::c_char,
    pub count: USHORT,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
static STANDARD_NAMES: std::sync::LazyLock<[std::ffi::CString; 258]> =
    std::sync::LazyLock::new(|| {
        read_fonts::tables::post::DEFAULT_GLYPH_NAMES
            .map(|name| std::ffi::CString::new(name).unwrap())
    });

fn parse_post_names(
    table: &read_fonts::tables::post::Post<'_>,
) -> Result<(Vec<u16>, Vec<Option<std::ffi::CString>>), read_fonts::ReadError> {
    use read_fonts::{ReadError, types::Version16Dot16};
    match table.version() {
        Version16Dot16::VERSION_1_0 => return Ok(((0..258).collect(), Vec::new())),
        Version16Dot16::VERSION_2_0 => {}
        _ => return Ok((Vec::new(), Vec::new())),
    }
    let count = table.num_glyphs().ok_or(ReadError::OutOfBounds)?;
    let indices: Vec<_> = table
        .glyph_name_index()
        .unwrap_or_default()
        .iter()
        .map(|index| index.get())
        .collect();
    if indices.len() != count as usize {
        return Err(ReadError::OutOfBounds);
    }
    let custom_count = indices
        .iter()
        .copied()
        .max()
        .unwrap_or(257)
        .saturating_sub(257) as usize;
    if custom_count == 0 {
        return Ok((indices, Vec::new()));
    }
    let strings = table.string_data().ok_or(ReadError::OutOfBounds)?;
    let names: Vec<_> = strings
        .iter()
        .take(custom_count)
        .map(|name| {
            let name = name?;
            if name.is_empty() {
                Ok(None)
            } else {
                std::ffi::CString::new(name.as_str())
                    .map(Some)
                    .map_err(|_| ReadError::MalformedData("Invalid PostScript glyph name"))
            }
        })
        .collect::<Result<_, _>>()?;
    if names.len() != custom_count {
        return Err(ReadError::OutOfBounds);
    }
    Ok((indices, names))
}

unsafe fn copy_post_array<T: Copy>(values: &[T]) -> *mut T {
    if values.is_empty() {
        return core::ptr::null_mut();
    }
    let size = values
        .len()
        .checked_mul(core::mem::size_of::<T>())
        .and_then(|size| u32::try_from(size).ok())
        .unwrap_or_else(|| pdf_abort!("PostScript table is too large"));
    let output = unsafe { new(size).cast::<T>() };
    unsafe { core::ptr::copy_nonoverlapping(values.as_ptr(), output, values.len()) };
    output
}

#[unsafe(export_name = "__pdf_impl_tt_read_post_table")]
pub unsafe extern "C-unwind" fn tt_read_post_table(sfont: *mut sfnt) -> *mut tt_post_table {
    use read_fonts::{FontRead, tables::post::Post};
    let length = unsafe { sfnt_find_table_len(sfont, c"post".as_ptr()) } as usize;
    unsafe { sfnt_locate_table(sfont, c"post".as_ptr()) };
    let mut data = vec![0; length];
    let read = unsafe { ttstub_input_read((*sfont).handle, data.as_mut_ptr().cast(), data.len()) };
    if read != length as isize {
        pdf_abort!("File ended prematurely\n");
    }
    let table = Post::read(data.as_slice().into()).unwrap_or_else(|_| {
        pdf_abort!("File ended prematurely\n");
    });
    let (indices, names) = match parse_post_names(&table) {
        Ok(names) => names,
        Err(_) => {
            pdf_warning!("Invalid version 2.0 'post' table");
            return core::ptr::null_mut();
        }
    };
    let version = u32::from_be_bytes(table.version().to_be_bytes());
    match version {
        0x00028000 => pdf_warning!("TrueType 'post' version 2.5 found (deprecated)"),
        0x00010000 | 0x00020000 | 0x00030000 | 0x00040000 => {}
        _ => pdf_warning!("Unknown 'post' version: {version:08X}, assuming version 3.0"),
    }
    let custom: Vec<_> = names
        .iter()
        .map(|name| {
            name.as_ref().map_or(core::ptr::null_mut(), |name| unsafe {
                crate::backend::runtime::duplicate(name.as_ptr())
            })
        })
        .collect();
    let pointers: Vec<_> = indices
        .iter()
        .map(|&index| {
            if (index as usize) < STANDARD_NAMES.len() {
                STANDARD_NAMES[index as usize].as_ptr()
            } else {
                custom[index as usize - STANDARD_NAMES.len()].cast_const()
            }
        })
        .collect();
    let post = tt_post_table {
        Version: version,
        italicAngle: table.italic_angle().to_bits() as u32,
        underlinePosition: table.underline_position().to_i16(),
        underlineThickness: table.underline_thickness().to_i16(),
        isFixedPitch: table.is_fixed_pitch(),
        minMemType42: table.min_mem_type42(),
        maxMemType42: table.max_mem_type42(),
        minMemType1: table.min_mem_type1(),
        maxMemType1: table.max_mem_type1(),
        numberOfGlyphs: indices.len() as u16,
        glyphNamePtr: unsafe { copy_post_array(&pointers) },
        names: unsafe { copy_post_array(&custom) },
        count: custom.len() as u16,
    };
    unsafe { copy_post_array(core::slice::from_ref(&post)) }
}

#[unsafe(export_name = "__pdf_impl_tt_lookup_post_table")]
pub unsafe extern "C-unwind" fn tt_lookup_post_table(
    post: *mut tt_post_table,
    glyphname: *const ::core::ffi::c_char,
) -> USHORT {
    if post.is_null() || glyphname.is_null() {
        pdf_abort!("assertion failed: post && glyphname");
    }
    let post = unsafe { &*post };
    if post.numberOfGlyphs == 0 {
        return 0;
    }
    let glyphname = unsafe { std::ffi::CStr::from_ptr(glyphname) };
    let names =
        unsafe { core::slice::from_raw_parts(post.glyphNamePtr, post.numberOfGlyphs as usize) };
    names
        .iter()
        .position(|&name| !name.is_null() && unsafe { std::ffi::CStr::from_ptr(name) } == glyphname)
        .unwrap_or(0) as u16
}

#[unsafe(export_name = "__pdf_impl_tt_get_glyphname")]
pub unsafe extern "C-unwind" fn tt_get_glyphname(
    post: *mut tt_post_table,
    gid: USHORT,
) -> *mut ::core::ffi::c_char {
    let post = unsafe { &*post };
    if gid >= post.numberOfGlyphs {
        return core::ptr::null_mut();
    }
    let name = unsafe { *post.glyphNamePtr.add(gid as usize) };
    if name.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { crate::backend::runtime::duplicate(name) }
    }
}

#[unsafe(export_name = "__pdf_impl_tt_release_post_table")]
pub unsafe extern "C-unwind" fn tt_release_post_table(post: *mut tt_post_table) {
    if post.is_null() {
        pdf_abort!("assertion failed: post");
    }
    let table = unsafe { &*post };
    if table.count != 0 {
        let names = unsafe { core::slice::from_raw_parts(table.names, table.count as usize) };
        for &name in names {
            unsafe { free(name.cast()) };
        }
    }
    unsafe {
        free(table.names.cast());
        free(table.glyphNamePtr.cast());
        free(post.cast());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_read_post_table(
    mut sfont: *mut sfnt,
    _result: *mut *mut tt_post_table,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_read_post_table(sfont) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_lookup_post_table(
    mut post: *mut tt_post_table,
    mut glyphname: *const ::core::ffi::c_char,
    _result: *mut USHORT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_lookup_post_table(post, glyphname) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_glyphname(
    mut post: *mut tt_post_table,
    mut gid: USHORT,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_get_glyphname(post, gid) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_release_post_table(mut post: *mut tt_post_table) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { tt_release_post_table(post) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
