/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
use crate::backend::dpx_pdfdev::pdf_sprint_number;
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strcpy(__dst: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_read(handle: rust_input_handle_t, data: *mut ::core::ffi::c_char, len: size_t) -> ssize_t;
    fn ttstub_input_getc(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn sprintf(_: *mut ::core::ffi::c_char, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_ref_obj"]
    fn pdf_ref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_stream"]
    fn pdf_new_stream(flags: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_stream"]
    fn pdf_add_stream(stream: *mut pdf_obj, stream_data_ptr: *const ::core::ffi::c_void, stream_data_len: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_font_get_resource"]
    fn pdf_font_get_resource(font: *mut pdf_font) -> *mut pdf_obj;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_tt_skip_bytes"]
    fn tt_skip_bytes(n: ::core::ffi::c_uint, handle: rust_input_handle_t);
    #[link_name = "__pdf_impl_tt_get_unsigned_byte"]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_tt_get_signed_byte"]
    fn tt_get_signed_byte(handle: rust_input_handle_t) -> ::core::ffi::c_schar;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_tt_get_signed_pair"]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> ::core::ffi::c_short;
    #[link_name = "__pdf_impl_tt_get_unsigned_triple"]
    fn tt_get_unsigned_triple(handle: rust_input_handle_t) -> ::core::ffi::c_uint;
    #[link_name = "__pdf_impl_tt_get_signed_quad"]
    fn tt_get_signed_quad(handle: rust_input_handle_t) -> int32_t;
    #[link_name = "__pdf_impl_tt_get_unsigned_num"]
    fn tt_get_unsigned_num(handle: rust_input_handle_t, num: ::core::ffi::c_uchar) -> uint32_t;
    #[link_name = "__pdf_impl_tt_get_positive_quad"]
    fn tt_get_positive_quad(handle: rust_input_handle_t, type_0: *const ::core::ffi::c_char, name: *const ::core::ffi::c_char) -> uint32_t;
    static mut work_buffer: [::core::ffi::c_char; 0];
    #[link_name = "__pdf_impl_pdf_encoding_used_by_type3"]
    fn pdf_encoding_used_by_type3(enc_id: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_pdf_encoding_get_name"]
    fn pdf_encoding_get_name(enc_id: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_encoding_get_encoding"]
    fn pdf_encoding_get_encoding(enc_id: ::core::ffi::c_int) -> *mut *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_tfm_open"]
    fn tfm_open(tex_name: *const ::core::ffi::c_char, must_exist: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tfm_get_design_size"]
    fn tfm_get_design_size(font_id: ::core::ffi::c_int) -> ::core::ffi::c_double;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
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
use crate::compat::renderer::pdf_rect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pk_header_ {
    pub pkt_len: uint32_t,
    pub chrcode: int32_t,
    pub wd: int32_t,
    pub dx: int32_t,
    pub dy: int32_t,
    pub bm_wd: uint32_t,
    pub bm_ht: uint32_t,
    pub bm_hoff: int32_t,
    pub bm_voff: int32_t,
    pub dyn_f: ::core::ffi::c_int,
    pub run_color: ::core::ffi::c_int,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
pub const STREAM_COMPRESS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const PKFONT_DPI_DEFAULT: ::core::ffi::c_uint = 600 as ::core::ffi::c_uint;

#[unsafe(export_name = "__pdf_impl_PKFont_set_dpi")]
pub unsafe extern "C-unwind" fn PKFont_set_dpi(mut dpi: ::core::ffi::c_int) {
    let __pdf_state = unsafe { document_state() };
    if dpi <= 0 as ::core::ffi::c_int {
        pdf_abort!(b"Invalid DPI: %d\n\0" as *const u8 as *const ::core::ffi::c_char, dpi,);
    }
    (*__pdf_state).base_dpi = dpi as ::core::ffi::c_uint;
}
unsafe extern "C-unwind" fn truedpi(
    mut ident: *const ::core::ffi::c_char, mut point_size: ::core::ffi::c_double, mut bdpi: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let __pdf_state = unsafe { document_state() };
    let mut dpi: ::core::ffi::c_uint = bdpi;
    let mut design_size: ::core::ffi::c_double = 0.;
    let mut tfm_id: ::core::ffi::c_int = 0;
    tfm_id = tfm_open(ident, 0 as ::core::ffi::c_int);
    if tfm_id < 0 as ::core::ffi::c_int {
        return dpi;
    }
    design_size = tfm_get_design_size(tfm_id);
    if design_size <= 0.0f64 {
        pdf_warning!(b"DESGIN_SIZE <= 0.0? (TFM=\"%s\")\0" as *const u8 as *const ::core::ffi::c_char, ident,);
    } else {
        dpi = (floor((*__pdf_state).base_dpi as ::core::ffi::c_double * point_size / design_size / 1.0f64 + 0.5f64) * 1.0f64) as ::core::ffi::c_uint;
    }
    return dpi;
}
unsafe extern "C-unwind" fn dpx_open_pk_font_at(mut ident: *const ::core::ffi::c_char, mut dpi: ::core::ffi::c_uint) -> rust_input_handle_t {
    pdf_warning!(
        b"Tectonic unable to generate PK font \"%s\" (dpi %u) on-the-fly\0" as *const u8 as *const ::core::ffi::c_char,
        ident,
        dpi,
    );
    return INVALID_HANDLE;
}
#[unsafe(export_name = "__pdf_impl_pdf_font_open_pkfont")]
pub unsafe extern "C-unwind" fn pdf_font_open_pkfont(
    mut font: *mut pdf_font, mut ident: *const ::core::ffi::c_char, mut index: ::core::ffi::c_int, mut encoding_id: ::core::ffi::c_int,
    mut embedding: ::core::ffi::c_int, mut point_size: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut dpi: ::core::ffi::c_uint = 0;
    let mut handle: rust_input_handle_t = 0;
    if ident.is_null() || point_size <= 0.0f64 {
        return -(1 as ::core::ffi::c_int);
    }
    if embedding == 0 {
        pdf_warning!(
            b"Ignoring no-embed option for PK font: %s\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    }
    if index != 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Ignoring font index option for PK font: %s\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
        );
    }
    dpi = truedpi(ident, point_size, (*__pdf_state).base_dpi);
    handle = dpx_open_pk_font_at(ident, dpi);
    if handle == 0 {
        return -(1 as ::core::ffi::c_int);
    }
    crate::backend::runtime::input_close(handle);
    (*font).fontname = new((strlen(ident).wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize) as uint32_t) as *mut ::core::ffi::c_char;
    strcpy((*font).fontname, ident);
    if encoding_id >= 0 as ::core::ffi::c_int {
        pdf_encoding_used_by_type3(encoding_id);
        pdf_warning!(
            b"PK font is found for font \"%s\" but non built-in encoding \"%s\" is specified.\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
            pdf_encoding_get_name(encoding_id),
        );
        pdf_warning!(b">> Assuming this is for glyph name assignment.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe fn decode_pk(stream: *mut pdf_obj, width: u32, height: u32, dynamic: i32, color: i32, data: *mut u8, length: u32) -> i32 {
    if length > 0 && data.is_null() {
        pdf_warning!("Insufficient bitmap pk data.");
        return -1;
    }
    let bytes = if length == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(data, length as usize) }
    };
    let Ok(dynamic) = u8::try_from(dynamic) else {
        return -1;
    };
    match crate::pk::decode(
        bytes,
        width,
        height,
        dynamic,
        color != 0,
        |row| unsafe {
            pdf_add_stream(stream, row.as_ptr().cast(), row.len() as i32);
        },
        |message| {
            pdf_warning!("{}", message);
        },
    ) {
        Ok(()) => 0,
        Err(error) => {
            pdf_warning!("{}", error.message.to_string_lossy());
            -1
        }
    }
}

unsafe extern "C-unwind" fn pk_decode_packed(
    stream: *mut pdf_obj, width: u32, height: u32, dynamic: i32, color: i32, data: *mut u8, length: u32,
) -> i32 {
    unsafe { decode_pk(stream, width, height, dynamic, color, data, length) }
}

unsafe extern "C-unwind" fn pk_decode_bitmap(
    stream: *mut pdf_obj, width: u32, height: u32, dynamic: i32, color: i32, data: *mut u8, length: u32,
) -> i32 {
    unsafe { decode_pk(stream, width, height, dynamic, color, data, length) }
}
unsafe extern "C-unwind" fn do_preamble(mut handle: rust_input_handle_t) {
    if ttstub_input_getc(handle) == 89 as ::core::ffi::c_int {
        tt_skip_bytes(tt_get_unsigned_byte(handle) as ::core::ffi::c_uint, handle);
        tt_skip_bytes(16 as ::core::ffi::c_uint, handle);
    } else {
        pdf_abort!(b"embed_pk_font: PK ID byte is incorrect.  Are you sure this is a PK file?\0" as *const u8 as *const ::core::ffi::c_char,);
    };
}
unsafe extern "C-unwind" fn read_pk_char_header(h: *mut pk_header_, opcode: core::ffi::c_uchar, handle: rust_input_handle_t) -> i32 {
    let mut input = crate::compat::input::ProbeInput { handle };
    let parsed = match crate::pk::header(&mut input, opcode) {
        Ok(header) => header,
        Err(error) => {
            pdf_warning!("{}", error.message.to_string_lossy());
            return -1;
        }
    };
    if !(0..=255).contains(&parsed.code) {
        pdf_warning!("Unable to handle long characters in PK files: code=0x{:04x}", parsed.code);
        return -1;
    }
    unsafe {
        (*h).pkt_len = parsed.packet_length;
        (*h).chrcode = parsed.code;
        (*h).wd = parsed.width;
        (*h).dx = parsed.dx;
        (*h).dy = parsed.dy;
        (*h).bm_wd = parsed.bitmap_width;
        (*h).bm_ht = parsed.bitmap_height;
        (*h).bm_hoff = parsed.horizontal_offset;
        (*h).bm_voff = parsed.vertical_offset;
        (*h).dyn_f = (opcode / 16) as i32;
        (*h).run_color = i32::from(opcode & 8 != 0);
    }
    0
}
unsafe extern "C-unwind" fn create_pk_CharProc_stream(
    mut pkh: *mut pk_header_, mut chrwid: ::core::ffi::c_double, mut pkt_ptr: *mut ::core::ffi::c_uchar, mut pkt_len: uint32_t,
) -> *mut pdf_obj {
    let mut stream: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut llx: int32_t = 0;
    let mut lly: int32_t = 0;
    let mut urx: int32_t = 0;
    let mut ury: int32_t = 0;
    let mut len: ::core::ffi::c_int = 0;
    llx = (*pkh).bm_hoff.checked_neg().unwrap_or_else(|| pdf_abort!("Invalid PK glyph bounds."));
    lly = ((*pkh).bm_voff as uint32_t).wrapping_sub((*pkh).bm_ht) as int32_t;
    urx = (*pkh).bm_wd.wrapping_sub((*pkh).bm_hoff as uint32_t) as int32_t;
    ury = (*pkh).bm_voff;
    stream = pdf_new_stream(STREAM_COMPRESS);
    len = pdf_sprint_number(&raw mut work_buffer as *mut ::core::ffi::c_char, chrwid);
    len += sprintf(
        (&raw mut work_buffer as *mut ::core::ffi::c_char).offset(len as isize),
        b" 0 %d %d %d %d d1\n\0" as *const u8 as *const ::core::ffi::c_char,
        llx,
        lly,
        urx,
        ury,
    );
    pdf_add_stream(
        stream,
        &raw mut work_buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len,
    );
    if (*pkh).bm_wd != 0 as uint32_t && (*pkh).bm_ht != 0 as uint32_t && pkt_len > 0 as uint32_t {
        len = sprintf(
            &raw mut work_buffer as *mut ::core::ffi::c_char,
            b"q\n%u 0 0 %u %d %d cm\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*pkh).bm_wd,
            (*pkh).bm_ht,
            llx,
            lly,
        );
        pdf_add_stream(
            stream,
            &raw mut work_buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            len,
        );
        len = sprintf(
            &raw mut work_buffer as *mut ::core::ffi::c_char,
            b"BI\n/W %u\n/H %u\n/IM true\n/BPC 1\nID \0" as *const u8 as *const ::core::ffi::c_char,
            (*pkh).bm_wd,
            (*pkh).bm_ht,
        );
        pdf_add_stream(
            stream,
            &raw mut work_buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            len,
        );
        let result = if (*pkh).dyn_f == 14 as ::core::ffi::c_int {
            pk_decode_bitmap(stream, (*pkh).bm_wd, (*pkh).bm_ht, (*pkh).dyn_f, (*pkh).run_color, pkt_ptr, pkt_len)
        } else {
            pk_decode_packed(stream, (*pkh).bm_wd, (*pkh).bm_ht, (*pkh).dyn_f, (*pkh).run_color, pkt_ptr, pkt_len)
        };
        if result < 0 {
            pdf_release_obj(stream);
            return core::ptr::null_mut();
        }
        len = sprintf(
            &raw mut work_buffer as *mut ::core::ffi::c_char,
            b"\nEI\nQ\0" as *const u8 as *const ::core::ffi::c_char,
        );
        pdf_add_stream(
            stream,
            &raw mut work_buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            len,
        );
    }
    return stream;
}
pub const PK_XXX1: ::core::ffi::c_int = 240 as ::core::ffi::c_int;
pub const PK_XXX2: ::core::ffi::c_int = 241;
pub const PK_XXX3: ::core::ffi::c_int = 242;
pub const PK_XXX4: ::core::ffi::c_int = 243;
pub const PK_YYY: ::core::ffi::c_int = 244;
pub const PK_POST: ::core::ffi::c_int = 245 as ::core::ffi::c_int;
pub const PK_NO_OP: ::core::ffi::c_int = 246;
pub const PK_PRE: ::core::ffi::c_int = 247;
#[unsafe(export_name = "__pdf_impl_pdf_font_load_pkfont")]
pub unsafe extern "C-unwind" fn pdf_font_load_pkfont(mut font: *mut pdf_font) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut fontdict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut usedchars: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ident: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut dpi: ::core::ffi::c_uint = 0;
    let mut handle: rust_input_handle_t = 0;
    let mut point_size: ::core::ffi::c_double = 0.;
    let mut pix2charu: ::core::ffi::c_double = 0.;
    let mut opcode: ::core::ffi::c_int = 0;
    let mut code: ::core::ffi::c_int = 0;
    let mut firstchar: ::core::ffi::c_int = 0;
    let mut lastchar: ::core::ffi::c_int = 0;
    let mut prev: ::core::ffi::c_int = 0;
    let mut charprocs: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut procset: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut encoding: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut tmp_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut widths: [::core::ffi::c_double; 256] = [0.; 256];
    let mut bbox: pdf_rect = pdf_rect {
        llx: 0.,
        lly: 0.,
        urx: 0.,
        ury: 0.,
    };
    let mut charavail: [::core::ffi::c_char; 256] = [0; 256];
    let mut encoding_id: ::core::ffi::c_int = 0;
    let mut enc_vec: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*font).reference.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    ident = (*font).filename;
    point_size = (*font).point_size;
    usedchars = (*font).usedchars;
    encoding_id = (*font).encoding_id;
    if encoding_id < 0 as ::core::ffi::c_int {
        enc_vec = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    } else {
        enc_vec = pdf_encoding_get_encoding(encoding_id);
    }
    if !(!ident.is_null() && !usedchars.is_null() && point_size > 0.0f64) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ident && usedchars && point_size > 0.0\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    dpi = truedpi(ident, point_size, (*__pdf_state).base_dpi);
    handle = dpx_open_pk_font_at(ident, dpi);
    if handle == 0 {
        pdf_abort!(
            b"Could not find/open PK font file: %s (at %udpi)\0" as *const u8 as *const ::core::ffi::c_char,
            ident,
            dpi,
        );
    }
    memset(
        &raw mut charavail as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        256 as size_t,
    );
    charprocs = pdf_new_dict();
    pix2charu = 72.0f64 * 1000.0f64 / (*__pdf_state).base_dpi as ::core::ffi::c_double / point_size;
    bbox.lly = ::core::f64::INFINITY;
    bbox.llx = bbox.lly;
    bbox.ury = -::core::f64::INFINITY;
    bbox.urx = bbox.ury;
    loop {
        opcode = ttstub_input_getc(handle);
        if !(opcode >= 0 as ::core::ffi::c_int && opcode != PK_POST) {
            break;
        }
        if opcode < 240 as ::core::ffi::c_int {
            let mut pkh: pk_header_ = pk_header_ {
                pkt_len: 0,
                chrcode: 0,
                wd: 0,
                dx: 0,
                dy: 0,
                bm_wd: 0,
                bm_ht: 0,
                bm_hoff: 0,
                bm_voff: 0,
                dyn_f: 0,
                run_color: 0,
            };
            error = read_pk_char_header(&raw mut pkh, opcode as ::core::ffi::c_uchar, handle);
            if error != 0 {
                pdf_abort!(b"Error in reading PK character header.\0" as *const u8 as *const ::core::ffi::c_char,);
            } else if charavail[(pkh.chrcode & 0xff as int32_t) as usize] != 0 {
                pdf_warning!(
                    b"More than two bitmap image for single glyph?: font=\"%s\" code=0x%02x\0" as *const u8 as *const ::core::ffi::c_char,
                    ident,
                    pkh.chrcode,
                );
            }
            if *usedchars.offset((pkh.chrcode & 0xff as int32_t) as isize) == 0 {
                tt_skip_bytes(pkh.pkt_len as ::core::ffi::c_uint, handle);
            } else {
                let mut charname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut charproc: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                let mut pkt_ptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                let mut bytesread: size_t = 0;
                let mut charwidth: ::core::ffi::c_double = 0.;
                charwidth = floor(
                    1000.0f64 * pkh.wd as ::core::ffi::c_double
                        / (((1 as ::core::ffi::c_int) << 20 as ::core::ffi::c_int) as ::core::ffi::c_double * pix2charu)
                        / 0.1f64
                        + 0.5f64,
                ) * 0.1f64;
                widths[(pkh.chrcode & 0xff as int32_t) as usize] = charwidth;
                bbox.llx = if bbox.llx < -pkh.bm_hoff as ::core::ffi::c_double {
                    bbox.llx
                } else {
                    -pkh.bm_hoff as ::core::ffi::c_double
                };
                bbox.lly = if bbox.lly < pkh.bm_voff as ::core::ffi::c_double - pkh.bm_ht as ::core::ffi::c_double {
                    bbox.lly
                } else {
                    pkh.bm_voff as ::core::ffi::c_double - pkh.bm_ht as ::core::ffi::c_double
                };
                bbox.urx = if bbox.urx > pkh.bm_wd as ::core::ffi::c_double - pkh.bm_hoff as ::core::ffi::c_double {
                    bbox.urx
                } else {
                    pkh.bm_wd as ::core::ffi::c_double - pkh.bm_hoff as ::core::ffi::c_double
                };
                bbox.ury = if bbox.ury > pkh.bm_voff as ::core::ffi::c_double {
                    bbox.ury
                } else {
                    pkh.bm_voff as ::core::ffi::c_double
                };
                use std::io::{Read, Seek, SeekFrom};
                let mut input = crate::compat::input::ProbeInput { handle };
                let position = input.stream_position().unwrap_or_else(|_| pdf_abort!("Invalid PK packet offset."));
                let end = input.seek(SeekFrom::End(0)).unwrap_or_else(|_| pdf_abort!("Invalid PK packet offset."));
                input
                    .seek(SeekFrom::Start(position))
                    .unwrap_or_else(|_| pdf_abort!("Invalid PK packet offset."));
                if u64::from(pkh.pkt_len) > end.saturating_sub(position) {
                    pdf_abort!("File ended prematurely\n");
                }
                let mut packet = vec![0; pkh.pkt_len as usize];
                input.read_exact(&mut packet).unwrap_or_else(|_| pdf_abort!("File ended prematurely\n"));
                charproc = create_pk_CharProc_stream(&raw mut pkh, charwidth, packet.as_mut_ptr(), packet.len() as u32);
                if charproc.is_null() {
                    pdf_abort!(b"Unpacking PK character data failed.\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                if encoding_id >= 0 as ::core::ffi::c_int && !enc_vec.is_null() {
                    charname = *enc_vec.offset((pkh.chrcode & 0xff as int32_t) as isize);
                    if charname.is_null() {
                        pdf_warning!(
                            b"\".notdef\" glyph used in font (code=0x%02x): %s\0" as *const u8 as *const ::core::ffi::c_char,
                            pkh.chrcode,
                            ident,
                        );
                        charname = &raw mut work_buffer as *mut ::core::ffi::c_char;
                        sprintf(
                            charname,
                            b"x%02X\0" as *const u8 as *const ::core::ffi::c_char,
                            pkh.chrcode as ::core::ffi::c_uchar as ::core::ffi::c_int,
                        );
                    }
                } else {
                    charname = &raw mut work_buffer as *mut ::core::ffi::c_char;
                    sprintf(
                        charname,
                        b"x%02X\0" as *const u8 as *const ::core::ffi::c_char,
                        pkh.chrcode as ::core::ffi::c_uchar as ::core::ffi::c_int,
                    );
                }
                pdf_add_dict(charprocs, pdf_new_name(charname), pdf_ref_obj(charproc));
                pdf_release_obj(charproc);
            }
            charavail[(pkh.chrcode & 0xff as int32_t) as usize] = 1 as ::core::ffi::c_char;
        } else {
            match opcode {
                PK_XXX1 | PK_XXX2 | PK_XXX3 | PK_XXX4 => {
                    let mut len: int32_t = tt_get_unsigned_num(handle, (opcode - PK_XXX1) as ::core::ffi::c_uchar) as int32_t;
                    if len < 0 as int32_t {
                        pdf_warning!(b"PK: Special with %d bytes???\0" as *const u8 as *const ::core::ffi::c_char, len,);
                    } else {
                        tt_skip_bytes(len as ::core::ffi::c_uint, handle);
                    }
                }
                PK_YYY => {
                    tt_skip_bytes(4 as ::core::ffi::c_uint, handle);
                }
                PK_PRE => {
                    do_preamble(handle);
                }
                PK_NO_OP | _ => {}
            }
        }
    }
    crate::backend::runtime::input_close(handle);
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        if *usedchars.offset(code as isize) as ::core::ffi::c_int != 0 && charavail[code as usize] == 0 {
            pdf_warning!(
                b"Missing glyph code=0x%02x in PK font \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                code,
                ident,
            );
        }
        code += 1;
    }
    fontdict = pdf_font_get_resource(font);
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"CharProcs\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj(charprocs),
    );
    pdf_release_obj(charprocs);
    procset = pdf_new_dict();
    tmp_array = pdf_new_array();
    pdf_add_array(tmp_array, pdf_new_name(b"PDF\0" as *const u8 as *const ::core::ffi::c_char));
    pdf_add_array(tmp_array, pdf_new_name(b"ImageB\0" as *const u8 as *const ::core::ffi::c_char));
    pdf_add_dict(procset, pdf_new_name(b"ProcSet\0" as *const u8 as *const ::core::ffi::c_char), tmp_array);
    pdf_add_dict(fontdict, pdf_new_name(b"Resources\0" as *const u8 as *const ::core::ffi::c_char), procset);
    tmp_array = pdf_new_array();
    prev = -(2 as ::core::ffi::c_int);
    firstchar = 255 as ::core::ffi::c_int;
    lastchar = 0 as ::core::ffi::c_int;
    code = 0 as ::core::ffi::c_int;
    while code < 256 as ::core::ffi::c_int {
        let mut charname_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if *usedchars.offset(code as isize) != 0 {
            if code < firstchar {
                firstchar = code;
            }
            if code > lastchar {
                lastchar = code;
            }
            if code != prev + 1 as ::core::ffi::c_int {
                pdf_add_array(tmp_array, pdf_new_number(code as ::core::ffi::c_double));
            }
            if encoding_id >= 0 as ::core::ffi::c_int && !enc_vec.is_null() {
                charname_0 = *enc_vec.offset(code as ::core::ffi::c_uchar as isize);
                if charname_0.is_null() {
                    charname_0 = &raw mut work_buffer as *mut ::core::ffi::c_char;
                    sprintf(
                        charname_0,
                        b"x%02X\0" as *const u8 as *const ::core::ffi::c_char,
                        code as ::core::ffi::c_uchar as ::core::ffi::c_int,
                    );
                }
            } else {
                charname_0 = &raw mut work_buffer as *mut ::core::ffi::c_char;
                sprintf(
                    charname_0,
                    b"x%02X\0" as *const u8 as *const ::core::ffi::c_char,
                    code as ::core::ffi::c_uchar as ::core::ffi::c_int,
                );
            }
            pdf_add_array(tmp_array, pdf_new_name(charname_0));
            prev = code;
        }
        code += 1;
    }
    if firstchar > lastchar {
        pdf_release_obj(tmp_array);
        pdf_abort!(
            b"Unexpected error: firstchar > lastchar (%d %d)\0" as *const u8 as *const ::core::ffi::c_char,
            firstchar,
            lastchar,
        );
    }
    if encoding_id < 0 as ::core::ffi::c_int || enc_vec.is_null() {
        encoding = pdf_new_dict();
        pdf_add_dict(
            encoding,
            pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"Encoding\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_dict(
            encoding,
            pdf_new_name(b"Differences\0" as *const u8 as *const ::core::ffi::c_char),
            tmp_array,
        );
        pdf_add_dict(
            fontdict,
            pdf_new_name(b"Encoding\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_ref_obj(encoding),
        );
        pdf_release_obj(encoding);
    } else {
        pdf_release_obj(tmp_array);
    }
    tmp_array = pdf_new_array();
    pdf_add_array(tmp_array, pdf_new_number(bbox.llx));
    pdf_add_array(tmp_array, pdf_new_number(bbox.lly));
    pdf_add_array(tmp_array, pdf_new_number(bbox.urx));
    pdf_add_array(tmp_array, pdf_new_number(bbox.ury));
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char),
        tmp_array,
    );
    tmp_array = pdf_new_array();
    code = firstchar;
    while code <= lastchar {
        if *usedchars.offset(code as isize) != 0 {
            pdf_add_array(tmp_array, pdf_new_number(widths[code as usize]));
        } else {
            pdf_add_array(tmp_array, pdf_new_number(0 as ::core::ffi::c_int as ::core::ffi::c_double));
        }
        code += 1;
    }
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"Widths\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj(tmp_array),
    );
    pdf_release_obj(tmp_array);
    tmp_array = pdf_new_array();
    pdf_add_array(tmp_array, pdf_new_number(0.001f64 * pix2charu));
    pdf_add_array(tmp_array, pdf_new_number(0.0f64));
    pdf_add_array(tmp_array, pdf_new_number(0.0f64));
    pdf_add_array(tmp_array, pdf_new_number(0.001f64 * pix2charu));
    pdf_add_array(tmp_array, pdf_new_number(0.0f64));
    pdf_add_array(tmp_array, pdf_new_number(0.0f64));
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"FontMatrix\0" as *const u8 as *const ::core::ffi::c_char),
        tmp_array,
    );
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"FirstChar\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(firstchar as ::core::ffi::c_double),
    );
    pdf_add_dict(
        fontdict,
        pdf_new_name(b"LastChar\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(lastchar as ::core::ffi::c_double),
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" {
    fn memset(__b: *mut ::core::ffi::c_void, __c: ::core::ffi::c_int, __len: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_PKFont_set_dpi(mut dpi: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { PKFont_set_dpi(dpi) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_font_open_pkfont(
    mut font: *mut pdf_font, mut ident: *const ::core::ffi::c_char, mut index: ::core::ffi::c_int, mut encoding_id: ::core::ffi::c_int,
    mut embedding: ::core::ffi::c_int, mut point_size: ::core::ffi::c_double, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_open_pkfont(font, ident, index, encoding_id, embedding, point_size) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_font_load_pkfont(mut font: *mut pdf_font, _result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_font_load_pkfont(font) }) {
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
    base_dpi: ::core::ffi::c_uint,
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self {
            base_dpi: PKFONT_DPI_DEFAULT,
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe { (*crate::compat::document::current()).backend.dpx_pkfont.pointer }
}
