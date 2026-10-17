/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2007-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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
use crate::numbers::scaled_fixword as sqxfw;
unsafe extern "C-unwind" {
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
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> ssize_t;
    fn ttstub_input_close(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tt_skip_bytes"]
    fn tt_skip_bytes(n: ::core::ffi::c_uint, handle: rust_input_handle_t);
    #[link_name = "__pdf_impl_tt_get_unsigned_byte"]
    fn tt_get_unsigned_byte(handle: rust_input_handle_t) -> ::core::ffi::c_uchar;
    #[link_name = "__pdf_impl_tt_get_unsigned_quad"]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    #[link_name = "__pdf_impl_tt_get_unsigned_num"]
    fn tt_get_unsigned_num(handle: rust_input_handle_t, num: ::core::ffi::c_uchar) -> uint32_t;
    #[link_name = "__pdf_impl_tt_get_positive_quad"]
    fn tt_get_positive_quad(
        handle: rust_input_handle_t,
        type_0: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
    ) -> uint32_t;
    static mut dpx_conf: _dpx_conf;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_dvi_vf_init"]
    fn dvi_vf_init(dev_font_id: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_dvi_vf_finish"]
    fn dvi_vf_finish();
    #[link_name = "__pdf_impl_dvi_set_font"]
    fn dvi_set_font(font_id: ::core::ffi::c_int);
    #[link_name = "__pdf_impl_dvi_set"]
    fn dvi_set(ch: int32_t);
    #[link_name = "__pdf_impl_dvi_rule"]
    fn dvi_rule(width: int32_t, height: int32_t);
    #[link_name = "__pdf_impl_dvi_right"]
    fn dvi_right(x: int32_t);
    #[link_name = "__pdf_impl_dvi_put"]
    fn dvi_put(ch: int32_t);
    #[link_name = "__pdf_impl_dvi_push"]
    fn dvi_push();
    #[link_name = "__pdf_impl_dpx_dvi_pop"]
    fn dpx_dvi_pop();
    #[link_name = "__pdf_impl_dvi_w0"]
    fn dvi_w0();
    #[link_name = "__pdf_impl_dvi_w"]
    fn dvi_w(ch: int32_t);
    #[link_name = "__pdf_impl_dvi_x0"]
    fn dvi_x0();
    #[link_name = "__pdf_impl_dvi_x"]
    fn dvi_x(ch: int32_t);
    #[link_name = "__pdf_impl_dvi_down"]
    fn dvi_down(y: int32_t);
    #[link_name = "__pdf_impl_dvi_y"]
    fn dvi_y(ch: int32_t);
    #[link_name = "__pdf_impl_dvi_y0"]
    fn dvi_y0();
    #[link_name = "__pdf_impl_dvi_z"]
    fn dvi_z(ch: int32_t);
    #[link_name = "__pdf_impl_dvi_z0"]
    fn dvi_z0();
    #[link_name = "__pdf_impl_dvi_dirchg"]
    fn dvi_dirchg(dir: ::core::ffi::c_uchar);
    #[link_name = "__pdf_impl_dvi_locate_font"]
    fn dvi_locate_font(name: *const ::core::ffi::c_char, ptsize: spt_t) -> ::core::ffi::c_uint;
    #[link_name = "__pdf_impl_dvi_do_special"]
    fn dvi_do_special(buffer: *const ::core::ffi::c_void, size: int32_t);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_tfm_open"]
    fn tfm_open(
        tex_name: *const ::core::ffi::c_char,
        must_exist: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tfm_is_jfm"]
    fn tfm_is_jfm(font_id: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type __darwin_off_t = __int64_t;
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
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C-unwind" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C-unwind" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3],
    pub _nbuf: [::core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type fixword = int32_t;
pub type spt_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vf {
    pub tex_name: *mut ::core::ffi::c_char,
    pub ptsize: spt_t,
    pub design_size: uint32_t,
    pub num_dev_fonts: ::core::ffi::c_uint,
    pub max_dev_fonts: ::core::ffi::c_uint,
    pub dev_fonts: *mut font_def,
    pub ch_pkt: *mut *mut ::core::ffi::c_uchar,
    pub message_flag: ::core::ffi::c_uchar,
    pub pkt_len: *mut uint32_t,
    pub num_chars: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_def {
    pub font_id: int32_t,
    pub checksum: uint32_t,
    pub size: uint32_t,
    pub design_size: uint32_t,
    pub directory: *mut ::core::ffi::c_char,
    pub name: *mut ::core::ffi::c_char,
    pub tfm_id: ::core::ffi::c_int,
    pub dev_id: ::core::ffi::c_int,
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
pub const SET_CHAR_127: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const SET1: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const SET2: ::core::ffi::c_int = 129;
pub const SET3: ::core::ffi::c_int = 130;
pub const SET4: ::core::ffi::c_int = 131;
pub const SET_RULE: ::core::ffi::c_int = 132;
pub const PUT1: ::core::ffi::c_int = 133 as ::core::ffi::c_int;
pub const PUT2: ::core::ffi::c_int = 134;
pub const PUT3: ::core::ffi::c_int = 135;
pub const PUT4: ::core::ffi::c_int = 136;
pub const PUT_RULE: ::core::ffi::c_int = 137;
pub const NOP: ::core::ffi::c_int = 138;
pub const PUSH: ::core::ffi::c_int = 141;
pub const POP: ::core::ffi::c_int = 142;
pub const RIGHT1: ::core::ffi::c_int = 143 as ::core::ffi::c_int;
pub const RIGHT2: ::core::ffi::c_int = 144;
pub const RIGHT3: ::core::ffi::c_int = 145;
pub const RIGHT4: ::core::ffi::c_int = 146;
pub const W0: ::core::ffi::c_int = 147;
pub const W1: ::core::ffi::c_int = 148 as ::core::ffi::c_int;
pub const W2: ::core::ffi::c_int = 149;
pub const W3: ::core::ffi::c_int = 150;
pub const W4: ::core::ffi::c_int = 151;
pub const X0: ::core::ffi::c_int = 152;
pub const X1: ::core::ffi::c_int = 153 as ::core::ffi::c_int;
pub const X2: ::core::ffi::c_int = 154;
pub const X3: ::core::ffi::c_int = 155;
pub const X4: ::core::ffi::c_int = 156;
pub const DOWN1: ::core::ffi::c_int = 157 as ::core::ffi::c_int;
pub const DOWN2: ::core::ffi::c_int = 158;
pub const DOWN3: ::core::ffi::c_int = 159;
pub const DOWN4: ::core::ffi::c_int = 160;
pub const Y0: ::core::ffi::c_int = 161;
pub const Y1: ::core::ffi::c_int = 162 as ::core::ffi::c_int;
pub const Y2: ::core::ffi::c_int = 163;
pub const Y3: ::core::ffi::c_int = 164;
pub const Y4: ::core::ffi::c_int = 165;
pub const Z0: ::core::ffi::c_int = 166;
pub const Z1: ::core::ffi::c_int = 167 as ::core::ffi::c_int;
pub const Z2: ::core::ffi::c_int = 168;
pub const Z3: ::core::ffi::c_int = 169;
pub const Z4: ::core::ffi::c_int = 170;
pub const FNT_NUM_0: ::core::ffi::c_int = 171 as ::core::ffi::c_int;
pub const FNT_NUM_63: ::core::ffi::c_int = 234 as ::core::ffi::c_int;
pub const FNT1: ::core::ffi::c_int = 235 as ::core::ffi::c_int;
pub const FNT2: ::core::ffi::c_int = 236;
pub const FNT3: ::core::ffi::c_int = 237;
pub const FNT4: ::core::ffi::c_int = 238;
pub const XXX1: ::core::ffi::c_int = 239 as ::core::ffi::c_int;
pub const XXX2: ::core::ffi::c_int = 240;
pub const XXX3: ::core::ffi::c_int = 241;
pub const XXX4: ::core::ffi::c_int = 242;
pub const FNT_DEF1: ::core::ffi::c_int = 243 as ::core::ffi::c_int;
pub const FNT_DEF2: ::core::ffi::c_int = 244;
pub const FNT_DEF3: ::core::ffi::c_int = 245;
pub const FNT_DEF4: ::core::ffi::c_int = 246;
pub const PRE: ::core::ffi::c_int = 247 as ::core::ffi::c_int;
pub const POST: ::core::ffi::c_int = 248 as ::core::ffi::c_int;
pub const PTEXDIR: ::core::ffi::c_int = 255;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
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
pub const VF_ALLOC_SIZE: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const VF_ID: ::core::ffi::c_int = 202 as ::core::ffi::c_int;
static mut vf_fonts: *mut vf = ::core::ptr::null::<vf>() as *mut vf;
static mut num_vf_fonts: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
static mut max_vf_fonts: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
#[unsafe(export_name = "__pdf_impl_vf_reset_global_state")]
pub unsafe extern "C-unwind" fn vf_reset_global_state() {
    num_vf_fonts = 0 as ::core::ffi::c_uint;
    max_vf_fonts = 0 as ::core::ffi::c_uint;
    vf_fonts = ::core::ptr::null_mut::<vf>();
}
unsafe extern "C-unwind" fn read_header(
    mut vf_handle: rust_input_handle_t,
    mut thisfont: ::core::ffi::c_int,
) {
    if tt_get_unsigned_byte(vf_handle) as ::core::ffi::c_int != PRE
        || tt_get_unsigned_byte(vf_handle) as ::core::ffi::c_int != VF_ID
    {
        fprintf(
            crate::backend::runtime::stderr().cast(),
            b"VF file may be corrupt\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    tt_skip_bytes(
        tt_get_unsigned_byte(vf_handle) as ::core::ffi::c_uint,
        vf_handle,
    );
    tt_skip_bytes(4 as ::core::ffi::c_uint, vf_handle);
    (*vf_fonts.offset(thisfont as isize)).design_size = tt_get_positive_quad(
        vf_handle,
        b"VF\0" as *const u8 as *const ::core::ffi::c_char,
        b"design_size\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C-unwind" fn resize_vf_fonts(mut size: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    if size as ::core::ffi::c_uint > max_vf_fonts {
        vf_fonts = renew(
            vf_fonts as *mut ::core::ffi::c_void,
            (size as uint32_t as usize).wrapping_mul(::core::mem::size_of::<vf>() as usize)
                as uint32_t,
        ) as *mut vf;
        i = max_vf_fonts as ::core::ffi::c_int;
        while i < size {
            (*vf_fonts.offset(i as isize)).num_dev_fonts = 0 as ::core::ffi::c_uint;
            (*vf_fonts.offset(i as isize)).max_dev_fonts = 0 as ::core::ffi::c_uint;
            let ref mut fresh9 = (*vf_fonts.offset(i as isize)).dev_fonts;
            *fresh9 = ::core::ptr::null_mut::<font_def>();
            (*vf_fonts.offset(i as isize)).message_flag = 0 as ::core::ffi::c_uchar;
            i += 1;
        }
        max_vf_fonts = size as ::core::ffi::c_uint;
    }
}
unsafe extern "C-unwind" fn resize_one_vf_font(mut a_vf: *mut vf, mut size: ::core::ffi::c_uint) {
    let mut i: ::core::ffi::c_uint = 0;
    if size > (*a_vf).num_chars {
        size = if size > (*a_vf).num_chars.wrapping_add(256 as ::core::ffi::c_uint) {
            size
        } else {
            (*a_vf).num_chars.wrapping_add(256 as ::core::ffi::c_uint)
        };
        (*a_vf).ch_pkt = renew(
            (*a_vf).ch_pkt as *mut ::core::ffi::c_void,
            (size as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_uchar>() as usize)
                as uint32_t,
        ) as *mut *mut ::core::ffi::c_uchar;
        (*a_vf).pkt_len = renew(
            (*a_vf).pkt_len as *mut ::core::ffi::c_void,
            (size as uint32_t as usize).wrapping_mul(::core::mem::size_of::<uint32_t>() as usize)
                as uint32_t,
        ) as *mut uint32_t;
        i = (*a_vf).num_chars;
        while i < size {
            let ref mut fresh5 = *(*a_vf).ch_pkt.offset(i as isize);
            *fresh5 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            *(*a_vf).pkt_len.offset(i as isize) = 0 as uint32_t;
            i = i.wrapping_add(1);
        }
        (*a_vf).num_chars = size;
    }
}
unsafe extern "C-unwind" fn read_a_char_def(
    mut vf_handle: rust_input_handle_t,
    mut thisfont: ::core::ffi::c_int,
    mut pkt_len: uint32_t,
    mut ch: uint32_t,
) {
    let mut pkt: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if ch >= (*vf_fonts.offset(thisfont as isize)).num_chars as uint32_t {
        resize_one_vf_font(
            vf_fonts.offset(thisfont as isize),
            (ch as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint),
        );
    }
    if pkt_len > 0 as uint32_t {
        pkt = new((pkt_len as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        if ttstub_input_read(
            vf_handle,
            pkt as *mut ::core::ffi::c_char,
            pkt_len as size_t,
        ) != pkt_len as ssize_t
        {
            pdf_abort!(b"VF file ended prematurely.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        let ref mut fresh4 = *(*vf_fonts.offset(thisfont as isize))
            .ch_pkt
            .offset(ch as isize);
        *fresh4 = pkt;
    }
    *(*vf_fonts.offset(thisfont as isize))
        .pkt_len
        .offset(ch as isize) = pkt_len;
}
unsafe extern "C-unwind" fn read_a_font_def(
    mut vf_handle: rust_input_handle_t,
    mut font_id: int32_t,
    mut thisfont: ::core::ffi::c_int,
) {
    let mut dev_font: *mut font_def = ::core::ptr::null_mut::<font_def>();
    let mut dir_length: ::core::ffi::c_int = 0;
    let mut name_length: ::core::ffi::c_int = 0;
    if (*vf_fonts.offset(thisfont as isize)).num_dev_fonts
        >= (*vf_fonts.offset(thisfont as isize)).max_dev_fonts
    {
        let ref mut fresh6 = (*vf_fonts.offset(thisfont as isize)).max_dev_fonts;
        *fresh6 = (*fresh6).wrapping_add(VF_ALLOC_SIZE);
        let ref mut fresh7 = (*vf_fonts.offset(thisfont as isize)).dev_fonts;
        *fresh7 = renew(
            (*vf_fonts.offset(thisfont as isize)).dev_fonts as *mut ::core::ffi::c_void,
            ((*vf_fonts.offset(thisfont as isize)).max_dev_fonts as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<font_def>() as usize) as uint32_t,
        ) as *mut font_def;
    }
    dev_font = (*vf_fonts.offset(thisfont as isize))
        .dev_fonts
        .offset((*vf_fonts.offset(thisfont as isize)).num_dev_fonts as isize);
    (*dev_font).font_id = font_id;
    (*dev_font).checksum = tt_get_unsigned_quad(vf_handle);
    (*dev_font).size = tt_get_positive_quad(
        vf_handle,
        b"VF\0" as *const u8 as *const ::core::ffi::c_char,
        b"font_size\0" as *const u8 as *const ::core::ffi::c_char,
    );
    (*dev_font).design_size = tt_get_positive_quad(
        vf_handle,
        b"VF\0" as *const u8 as *const ::core::ffi::c_char,
        b"font_design_size\0" as *const u8 as *const ::core::ffi::c_char,
    );
    dir_length = tt_get_unsigned_byte(vf_handle) as ::core::ffi::c_int;
    name_length = tt_get_unsigned_byte(vf_handle) as ::core::ffi::c_int;
    (*dev_font).directory = new(
        ((dir_length + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    if ttstub_input_read(vf_handle, (*dev_font).directory, dir_length as size_t)
        != dir_length as ssize_t
    {
        pdf_abort!(b"directory read failed\0" as *const u8 as *const ::core::ffi::c_char);
    }
    (*dev_font).name = new(
        ((name_length + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    if ttstub_input_read(vf_handle, (*dev_font).name, name_length as size_t)
        != name_length as ssize_t
    {
        pdf_abort!(b"directory read failed\0" as *const u8 as *const ::core::ffi::c_char);
    }
    *(*dev_font).directory.offset(dir_length as isize) = 0 as ::core::ffi::c_char;
    *(*dev_font).name.offset(name_length as isize) = 0 as ::core::ffi::c_char;
    let ref mut fresh8 = (*vf_fonts.offset(thisfont as isize)).num_dev_fonts;
    *fresh8 = (*fresh8).wrapping_add(1 as ::core::ffi::c_uint);
    (*dev_font).tfm_id = tfm_open((*dev_font).name, 1 as ::core::ffi::c_int);
    (*dev_font).dev_id = dvi_locate_font(
        (*dev_font).name,
        sqxfw(
            (*vf_fonts.offset(thisfont as isize)).ptsize as int32_t,
            (*dev_font).size as fixword,
        ) as spt_t,
    ) as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn process_vf_file(
    mut vf_handle: rust_input_handle_t,
    mut thisfont: ::core::ffi::c_int,
) {
    let mut eof: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut code: ::core::ffi::c_int = 0;
    let mut font_id: uint32_t = 0;
    while eof == 0 {
        code = tt_get_unsigned_byte(vf_handle) as ::core::ffi::c_int;
        match code {
            FNT_DEF1 | FNT_DEF2 | FNT_DEF3 | FNT_DEF4 => {
                font_id = tt_get_unsigned_num(vf_handle, (code - FNT_DEF1) as ::core::ffi::c_uchar);
                read_a_font_def(vf_handle, font_id as int32_t, thisfont);
            }
            _ => {
                if code < 242 as ::core::ffi::c_int {
                    let mut ch: uint32_t = tt_get_unsigned_byte(vf_handle) as uint32_t;
                    tt_skip_bytes(3 as ::core::ffi::c_uint, vf_handle);
                    read_a_char_def(vf_handle, thisfont, code as uint32_t, ch);
                } else if code == 242 as ::core::ffi::c_int {
                    let mut pkt_len: uint32_t = tt_get_positive_quad(
                        vf_handle,
                        b"VF\0" as *const u8 as *const ::core::ffi::c_char,
                        b"pkt_len\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    let mut ch_0: uint32_t = tt_get_unsigned_quad(vf_handle);
                    tt_skip_bytes(4 as ::core::ffi::c_uint, vf_handle);
                    if ch_0 < 0x1000000 as uint32_t {
                        read_a_char_def(vf_handle, thisfont, pkt_len, ch_0);
                    } else {
                        fprintf(
                            crate::backend::runtime::stderr().cast(),
                            b"char=%u\n\0" as *const u8 as *const ::core::ffi::c_char,
                            ch_0,
                        );
                        pdf_abort!(
                            b"Long character (>24 bits) in VF file.\nI can't handle long characters!\n\0"
                            as * const u8 as * const ::core::ffi::c_char,
                        );
                    }
                } else if code == POST {
                    eof = 1 as ::core::ffi::c_int;
                } else {
                    fprintf(
                        crate::backend::runtime::stderr().cast(),
                        b"Quitting on code=%d\n\0" as *const u8 as *const ::core::ffi::c_char,
                        code,
                    );
                    eof = 1 as ::core::ffi::c_int;
                }
            }
        }
    }
}
#[unsafe(export_name = "__pdf_impl_vf_locate_font")]
pub unsafe extern "C-unwind" fn vf_locate_font(
    mut tex_name: *const ::core::ffi::c_char,
    mut ptsize: spt_t,
) -> ::core::ffi::c_int {
    let mut thisfont: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut vf_handle: rust_input_handle_t = INVALID_HANDLE;
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < num_vf_fonts {
        if streq_ptr((*vf_fonts.offset(i as isize)).tex_name, tex_name) as ::core::ffi::c_int != 0
            && (*vf_fonts.offset(i as isize)).ptsize == ptsize
        {
            break;
        }
        i += 1;
    }
    if i as ::core::ffi::c_uint != num_vf_fonts {
        return i;
    }
    vf_handle = ttstub_input_open(tex_name, TTBC_FILE_FORMAT_VF, 0 as ::core::ffi::c_int);
    if vf_handle == INVALID_HANDLE {
        vf_handle = ttstub_input_open(tex_name, TTBC_FILE_FORMAT_OVF, 0 as ::core::ffi::c_int);
    }
    if vf_handle == INVALID_HANDLE {
        return -(1 as ::core::ffi::c_int);
    }
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        fprintf(
            crate::backend::runtime::stderr().cast(),
            b"(VF:%s\0" as *const u8 as *const ::core::ffi::c_char,
            tex_name,
        );
    }
    if num_vf_fonts >= max_vf_fonts {
        resize_vf_fonts(max_vf_fonts.wrapping_add(VF_ALLOC_SIZE) as ::core::ffi::c_int);
    }
    let fresh0 = num_vf_fonts;
    num_vf_fonts = num_vf_fonts.wrapping_add(1);
    thisfont = fresh0 as ::core::ffi::c_int;
    let ref mut fresh1 = (*vf_fonts.offset(thisfont as isize)).tex_name;
    *fresh1 = new(
        (strlen(tex_name).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*vf_fonts.offset(thisfont as isize)).tex_name, tex_name);
    (*vf_fonts.offset(thisfont as isize)).ptsize = ptsize;
    (*vf_fonts.offset(thisfont as isize)).num_chars = 0 as ::core::ffi::c_uint;
    let ref mut fresh2 = (*vf_fonts.offset(thisfont as isize)).ch_pkt;
    *fresh2 = ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
    let ref mut fresh3 = (*vf_fonts.offset(thisfont as isize)).pkt_len;
    *fresh3 = ::core::ptr::null_mut::<uint32_t>();
    read_header(vf_handle, thisfont);
    process_vf_file(vf_handle, thisfont);
    if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
        fprintf(
            crate::backend::runtime::stderr().cast(),
            b")\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    crate::backend::runtime::input_close(vf_handle);
    return thisfont;
}
unsafe extern "C-unwind" fn unsigned_byte(
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut byte: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *start < end {
        let fresh11 = *start;
        *start = (*start).offset(1);
        byte = *fresh11 as ::core::ffi::c_int;
    } else {
        pdf_abort!(
            b"Premature end of DVI byte stream in VF font\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return byte;
}
unsafe extern "C-unwind" fn get_pkt_signed_num(
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut num: ::core::ffi::c_uchar,
) -> int32_t {
    let mut val: int32_t = 0 as int32_t;
    if end.offset_from(*start) as ::core::ffi::c_long > num as ::core::ffi::c_long {
        let fresh16 = *start;
        *start = (*start).offset(1);
        val = *fresh16 as int32_t;
        if val > 0x7f as int32_t {
            val = (val as ::core::ffi::c_int - 0x100 as ::core::ffi::c_int) as int32_t;
        }
        let mut current_block_5: u64;
        match num as ::core::ffi::c_int {
            3 => {
                let fresh17 = *start;
                *start = (*start).offset(1);
                val = val << 8 as ::core::ffi::c_int | *fresh17 as int32_t;
                current_block_5 = 426399902127340518;
            }
            2 => {
                current_block_5 = 426399902127340518;
            }
            1 => {
                current_block_5 = 3356221950756584563;
            }
            _ => {
                current_block_5 = 7815301370352969686;
            }
        }
        match current_block_5 {
            426399902127340518 => {
                let fresh18 = *start;
                *start = (*start).offset(1);
                val = val << 8 as ::core::ffi::c_int | *fresh18 as int32_t;
                current_block_5 = 3356221950756584563;
            }
            _ => {}
        }
        match current_block_5 {
            3356221950756584563 => {
                let fresh19 = *start;
                *start = (*start).offset(1);
                val = val << 8 as ::core::ffi::c_int | *fresh19 as int32_t;
            }
            _ => {}
        }
    } else {
        pdf_abort!(
            b"Premature end of DVI byte stream in VF font\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return val;
}
unsafe extern "C-unwind" fn get_pkt_unsigned_num(
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut num: ::core::ffi::c_uchar,
) -> int32_t {
    let mut val: int32_t = 0 as int32_t;
    if end.offset_from(*start) as ::core::ffi::c_long > num as ::core::ffi::c_long {
        let fresh12 = *start;
        *start = (*start).offset(1);
        val = *fresh12 as int32_t;
        let mut current_block_5: u64;
        match num as ::core::ffi::c_int {
            3 => {
                if val > 0x7f as int32_t {
                    val = (val as ::core::ffi::c_int - 0x100 as ::core::ffi::c_int) as int32_t;
                }
                let fresh13 = *start;
                *start = (*start).offset(1);
                val = val << 8 as ::core::ffi::c_int | *fresh13 as int32_t;
                current_block_5 = 7042427548161126177;
            }
            2 => {
                current_block_5 = 7042427548161126177;
            }
            1 => {
                current_block_5 = 16243942682322371836;
            }
            _ => {
                current_block_5 = 7815301370352969686;
            }
        }
        match current_block_5 {
            7042427548161126177 => {
                let fresh14 = *start;
                *start = (*start).offset(1);
                val = val << 8 as ::core::ffi::c_int | *fresh14 as int32_t;
                current_block_5 = 16243942682322371836;
            }
            _ => {}
        }
        match current_block_5 {
            16243942682322371836 => {
                let fresh15 = *start;
                *start = (*start).offset(1);
                val = val << 8 as ::core::ffi::c_int | *fresh15 as int32_t;
            }
            _ => {}
        }
    } else {
        pdf_abort!(
            b"Premature end of DVI byte stream in VF font\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return val;
}
unsafe extern "C-unwind" fn vf_putrule(
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut ptsize: spt_t,
) {
    let mut height: int32_t = get_pkt_signed_num(start, end, 3 as ::core::ffi::c_uchar);
    let mut width: int32_t = get_pkt_signed_num(start, end, 3 as ::core::ffi::c_uchar);
    dvi_rule(
        sqxfw(ptsize as int32_t, width as fixword),
        sqxfw(ptsize as int32_t, height as fixword),
    );
}
unsafe extern "C-unwind" fn vf_setrule(
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
    mut ptsize: spt_t,
) {
    let mut height: int32_t = get_pkt_signed_num(start, end, 3 as ::core::ffi::c_uchar);
    let mut s_width: int32_t = sqxfw(
        ptsize as int32_t,
        get_pkt_signed_num(start, end, 3 as ::core::ffi::c_uchar) as fixword,
    );
    dvi_rule(s_width, sqxfw(ptsize as int32_t, height as fixword));
    dvi_right(s_width);
}
unsafe extern "C-unwind" fn vf_fnt(mut font_id: int32_t, mut vf_font: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while (i as ::core::ffi::c_uint) < (*vf_fonts.offset(vf_font as isize)).num_dev_fonts {
        if font_id
            == (*(*vf_fonts.offset(vf_font as isize))
                .dev_fonts
                .offset(i as isize))
            .font_id
        {
            break;
        }
        i += 1;
    }
    if (i as ::core::ffi::c_uint) < (*vf_fonts.offset(vf_font as isize)).num_dev_fonts {
        dvi_set_font(
            (*(*vf_fonts.offset(vf_font as isize))
                .dev_fonts
                .offset(i as isize))
            .dev_id,
        );
    } else {
        fprintf(
            crate::backend::runtime::stderr().cast(),
            b"Font_id: %d not found in VF\n\0" as *const u8 as *const ::core::ffi::c_char,
            font_id,
        );
    };
}
unsafe extern "C-unwind" fn vf_xxx(
    mut len: int32_t,
    mut start: *mut *mut ::core::ffi::c_uchar,
    mut end: *mut ::core::ffi::c_uchar,
) {
    if *start <= end.offset(-(len as isize)) {
        let mut buffer: *mut ::core::ffi::c_uchar = new(((len + 1 as int32_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t)
            as *mut ::core::ffi::c_uchar;
        memcpy(
            buffer as *mut ::core::ffi::c_void,
            *start as *const ::core::ffi::c_void,
            len as size_t,
        );
        *buffer.offset(len as isize) = '\0' as i32 as ::core::ffi::c_uchar;
        let mut p: *mut ::core::ffi::c_uchar = buffer;
        while p < buffer.offset(len as isize) && *p as ::core::ffi::c_int == ' ' as i32 {
            p = p.offset(1);
        }
        if memcmp(
            p as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            b"Warning:\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            8 as size_t,
        ) == 0
        {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"VF:%s\0" as *const u8 as *const ::core::ffi::c_char,
                    p.offset(8 as ::core::ffi::c_int as isize),
                );
            }
        } else {
            dvi_do_special(buffer as *const ::core::ffi::c_void, len);
        }
        free(buffer as *mut ::core::ffi::c_void);
    } else {
        pdf_abort!(
            b"Premature end of DVI byte stream in VF font.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    *start = (*start).offset(len as isize);
}
#[unsafe(export_name = "__pdf_impl_vf_set_char")]
pub unsafe extern "C-unwind" fn vf_set_char(mut ch: int32_t, mut vf_font: ::core::ffi::c_int) {
    let mut opcode: ::core::ffi::c_uchar = 0;
    let mut start: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut end: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut ptsize: spt_t = 0;
    let mut default_font: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (vf_font as ::core::ffi::c_uint) < num_vf_fonts {
        ptsize = (*vf_fonts.offset(vf_font as isize)).ptsize;
        if (*vf_fonts.offset(vf_font as isize)).num_dev_fonts > 0 as ::core::ffi::c_uint {
            default_font = (*(*vf_fonts.offset(vf_font as isize))
                .dev_fonts
                .offset(0 as ::core::ffi::c_int as isize))
            .dev_id;
        }
        dvi_vf_init(default_font);
        if ch as ::core::ffi::c_uint >= (*vf_fonts.offset(vf_font as isize)).num_chars || {
            start = *(*vf_fonts.offset(vf_font as isize))
                .ch_pkt
                .offset(ch as isize);
            start.is_null()
        } {
            if tfm_is_jfm(
                (*(*vf_fonts.offset(vf_font as isize))
                    .dev_fonts
                    .offset(0 as ::core::ffi::c_int as isize))
                .tfm_id,
            ) != 0
                && ch < 0x1000000 as int32_t
                && dpx_conf.compat_mode as ::core::ffi::c_uint
                    != dpx_mode_xdv_mode as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if dpx_conf.verbose_level == 1 as ::core::ffi::c_int {
                    if (*vf_fonts.offset(vf_font as isize)).message_flag as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        pdf_warning!(
                            b"Fallback multibyte character in virtual font: VF:%s to TFM:%s\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            (*vf_fonts.offset(vf_font as isize)).tex_name,
                            (*(*vf_fonts.offset(vf_font as isize))
                                .dev_fonts
                                .offset(0 as ::core::ffi::c_int as isize))
                            .name,
                        );
                        (*vf_fonts.offset(vf_font as isize)).message_flag =
                            1 as ::core::ffi::c_uchar;
                    }
                }
                if dpx_conf.verbose_level > 1 as ::core::ffi::c_int {
                    pdf_warning!(
                        b"Fallback multibyte character in virtual font: VF:%s char=0x%06x(%d) to TFM:%s\0"
                        as * const u8 as * const ::core::ffi::c_char, (* vf_fonts
                        .offset(vf_font as isize)).tex_name, ch, ch, (* (* vf_fonts
                        .offset(vf_font as isize)).dev_fonts.offset(0 as
                        ::core::ffi::c_int as isize)).name,
                    );
                }
                dvi_set(ch);
                dvi_vf_finish();
                return;
            }
            fprintf(
                crate::backend::runtime::stderr().cast(),
                b"\nchar=0x%x(%d)\n\0" as *const u8 as *const ::core::ffi::c_char,
                ch,
                ch,
            );
            fprintf(
                crate::backend::runtime::stderr().cast(),
                b"Tried to set a nonexistent character in a virtual font\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            end = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            start = end;
        } else {
            end = start.offset(
                *(*vf_fonts.offset(vf_font as isize))
                    .pkt_len
                    .offset(ch as isize) as isize,
            );
        }
        while !start.is_null() && start < end {
            let fresh10 = start;
            start = start.offset(1);
            opcode = *fresh10;
            match opcode as ::core::ffi::c_int {
                SET1 | SET2 | SET3 => {
                    dvi_set(get_pkt_unsigned_num(
                        &raw mut start,
                        end,
                        (opcode as ::core::ffi::c_int - SET1) as ::core::ffi::c_uchar,
                    ));
                }
                SET4 => {
                    pdf_abort!(
                        b"Multibyte (>24 bits) character in VF packet.\nI can't handle this!\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                SET_RULE => {
                    vf_setrule(&raw mut start, end, ptsize);
                }
                PUT1 | PUT2 | PUT3 => {
                    dvi_put(get_pkt_unsigned_num(
                        &raw mut start,
                        end,
                        (opcode as ::core::ffi::c_int - PUT1) as ::core::ffi::c_uchar,
                    ));
                }
                PUT4 => {
                    pdf_abort!(
                        b"Multibyte (>24 bits) character in VF packet.\nI can't handle this!\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                PUT_RULE => {
                    vf_putrule(&raw mut start, end, ptsize);
                }
                NOP => {}
                PUSH => {
                    dvi_push();
                }
                POP => {
                    dpx_dvi_pop();
                }
                RIGHT1 | RIGHT2 | RIGHT3 | RIGHT4 => {
                    dvi_right(sqxfw(
                        ptsize as int32_t,
                        get_pkt_signed_num(
                            &raw mut start,
                            end,
                            (opcode as ::core::ffi::c_int - RIGHT1) as ::core::ffi::c_uchar,
                        ) as fixword,
                    ));
                }
                W0 => {
                    dvi_w0();
                }
                W1 | W2 | W3 | W4 => {
                    dvi_w(sqxfw(
                        ptsize as int32_t,
                        get_pkt_signed_num(
                            &raw mut start,
                            end,
                            (opcode as ::core::ffi::c_int - W1) as ::core::ffi::c_uchar,
                        ) as fixword,
                    ));
                }
                X0 => {
                    dvi_x0();
                }
                X1 | X2 | X3 | X4 => {
                    dvi_x(sqxfw(
                        ptsize as int32_t,
                        get_pkt_signed_num(
                            &raw mut start,
                            end,
                            (opcode as ::core::ffi::c_int - X1) as ::core::ffi::c_uchar,
                        ) as fixword,
                    ));
                }
                DOWN1 | DOWN2 | DOWN3 | DOWN4 => {
                    dvi_down(sqxfw(
                        ptsize as int32_t,
                        get_pkt_signed_num(
                            &raw mut start,
                            end,
                            (opcode as ::core::ffi::c_int - DOWN1) as ::core::ffi::c_uchar,
                        ) as fixword,
                    ));
                }
                Y0 => {
                    dvi_y0();
                }
                Y1 | Y2 | Y3 | Y4 => {
                    dvi_y(sqxfw(
                        ptsize as int32_t,
                        get_pkt_signed_num(
                            &raw mut start,
                            end,
                            (opcode as ::core::ffi::c_int - Y1) as ::core::ffi::c_uchar,
                        ) as fixword,
                    ));
                }
                Z0 => {
                    dvi_z0();
                }
                Z1 | Z2 | Z3 | Z4 => {
                    dvi_z(sqxfw(
                        ptsize as int32_t,
                        get_pkt_signed_num(
                            &raw mut start,
                            end,
                            (opcode as ::core::ffi::c_int - Z1) as ::core::ffi::c_uchar,
                        ) as fixword,
                    ));
                }
                FNT1 | FNT2 | FNT3 | FNT4 => {
                    vf_fnt(
                        get_pkt_signed_num(
                            &raw mut start,
                            end,
                            (opcode as ::core::ffi::c_int - FNT1) as ::core::ffi::c_uchar,
                        ),
                        vf_font,
                    );
                }
                XXX1 | XXX2 | XXX3 | XXX4 => {
                    let mut len: int32_t = get_pkt_unsigned_num(
                        &raw mut start,
                        end,
                        (opcode as ::core::ffi::c_int - XXX1) as ::core::ffi::c_uchar,
                    );
                    if len < 0 as int32_t {
                        pdf_warning!(
                            b"VF: Special with %d bytes???\0" as *const u8
                                as *const ::core::ffi::c_char,
                            len,
                        );
                    } else {
                        vf_xxx(len, &raw mut start, end);
                    }
                }
                PTEXDIR => {
                    dvi_dirchg(unsigned_byte(&raw mut start, end) as ::core::ffi::c_uchar);
                }
                _ => {
                    if opcode as ::core::ffi::c_int <= SET_CHAR_127 {
                        dvi_set(opcode as int32_t);
                    } else if opcode as ::core::ffi::c_int >= FNT_NUM_0
                        && opcode as ::core::ffi::c_int <= FNT_NUM_63
                    {
                        vf_fnt(opcode as int32_t - FNT_NUM_0 as int32_t, vf_font);
                    } else {
                        fprintf(
                            crate::backend::runtime::stderr().cast(),
                            b"Unexpected opcode: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
                            opcode as ::core::ffi::c_int,
                        );
                        pdf_abort!(
                            b"Unexpected opcode in vf file\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                }
            }
        }
        dvi_vf_finish();
    } else {
        fprintf(
            crate::backend::runtime::stderr().cast(),
            b"vf_set_char: font: %d\0" as *const u8 as *const ::core::ffi::c_char,
            vf_font,
        );
        pdf_abort!(b"Font not loaded\n\0" as *const u8 as *const ::core::ffi::c_char);
    };
}
#[unsafe(export_name = "__pdf_impl_vf_close_all_fonts")]
pub unsafe extern "C-unwind" fn vf_close_all_fonts() {
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    let mut one_font: *mut font_def = ::core::ptr::null_mut::<font_def>();
    i = 0 as ::core::ffi::c_uint;
    while i < num_vf_fonts {
        if !(*vf_fonts.offset(i as isize)).ch_pkt.is_null() {
            j = 0 as ::core::ffi::c_uint;
            while j < (*vf_fonts.offset(i as isize)).num_chars {
                free(*(*vf_fonts.offset(i as isize)).ch_pkt.offset(j as isize)
                    as *mut ::core::ffi::c_void);
                j = j.wrapping_add(1);
            }
            free((*vf_fonts.offset(i as isize)).ch_pkt as *mut ::core::ffi::c_void);
        }
        free((*vf_fonts.offset(i as isize)).pkt_len as *mut ::core::ffi::c_void);
        free((*vf_fonts.offset(i as isize)).tex_name as *mut ::core::ffi::c_void);
        j = 0 as ::core::ffi::c_uint;
        while j < (*vf_fonts.offset(i as isize)).num_dev_fonts {
            one_font = (*vf_fonts.offset(i as isize)).dev_fonts.offset(j as isize) as *mut font_def;
            free((*one_font).directory as *mut ::core::ffi::c_void);
            free((*one_font).name as *mut ::core::ffi::c_void);
            j = j.wrapping_add(1);
        }
        free((*vf_fonts.offset(i as isize)).dev_fonts as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    free(vf_fonts as *mut ::core::ffi::c_void);
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct __sFILEX {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_vf_reset_global_state() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { vf_reset_global_state() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_vf_locate_font(
    mut tex_name: *const ::core::ffi::c_char,
    mut ptsize: spt_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { vf_locate_font(tex_name, ptsize) }) {
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
pub unsafe extern "C" fn __pdf_entry_vf_set_char(
    mut ch: int32_t,
    mut vf_font: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { vf_set_char(ch, vf_font) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_vf_close_all_fonts() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { vf_close_all_fonts() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
