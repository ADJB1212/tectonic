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
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: ::core::ffi::c_int,
    ) -> size_t;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_string"]
    fn pdf_new_string(str: *const ::core::ffi::c_void, length: size_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(
        dict: *mut pdf_obj,
        key: *mut pdf_obj,
        value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tt_get_unsigned_quad"]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    static mut dpx_conf: _dpx_conf;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_tt_read_post_table"]
    fn tt_read_post_table(sfont: *mut sfnt) -> *mut tt_post_table;
    #[link_name = "__pdf_impl_tt_release_post_table"]
    fn tt_release_post_table(post: *mut tt_post_table);
    #[link_name = "__pdf_impl_tt_read_head_table"]
    fn tt_read_head_table(sfont: *mut sfnt) -> *mut tt_head_table;
    #[link_name = "__pdf_impl_tt_read_os2__table"]
    fn tt_read_os2__table(sfont: *mut sfnt) -> *mut tt_os2__table;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
pub type rust_input_handle_t = uintptr_t;
pub type BYTE = ::core::ffi::c_uchar;
pub type SFNT_CHAR = ::core::ffi::c_schar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_os2__table {
    pub version: USHORT,
    pub xAvgCharWidth: SHORT,
    pub usWeightClass: USHORT,
    pub usWidthClass: USHORT,
    pub fsType: SHORT,
    pub ySubscriptXSize: SHORT,
    pub ySubscriptYSize: SHORT,
    pub ySubscriptXOffset: SHORT,
    pub ySubscriptYOffset: SHORT,
    pub ySuperscriptXSize: SHORT,
    pub ySuperscriptYSize: SHORT,
    pub ySuperscriptXOffset: SHORT,
    pub ySuperscriptYOffset: SHORT,
    pub yStrikeoutSize: SHORT,
    pub yStrikeoutPosition: SHORT,
    pub sFamilyClass: SHORT,
    pub panose: [BYTE; 10],
    pub ulUnicodeRange1: SFNT_ULONG,
    pub ulUnicodeRange2: SFNT_ULONG,
    pub ulUnicodeRange3: SFNT_ULONG,
    pub ulUnicodeRange4: SFNT_ULONG,
    pub achVendID: [SFNT_CHAR; 4],
    pub fsSelection: USHORT,
    pub usFirstCharIndex: USHORT,
    pub usLastCharIndex: USHORT,
    pub sTypoAscender: SHORT,
    pub sTypoDescender: SHORT,
    pub sTypoLineGap: SHORT,
    pub usWinAscent: USHORT,
    pub usWinDescent: USHORT,
    pub ulCodePageRange1: SFNT_ULONG,
    pub ulCodePageRange2: SFNT_ULONG,
    pub sxHeight: SHORT,
    pub sCapHeight: SHORT,
    pub usDefaultChar: USHORT,
    pub usBreakChar: USHORT,
    pub usMaxContext: USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_head_table {
    pub version: Fixed,
    pub fontRevision: Fixed,
    pub checkSumAdjustment: SFNT_ULONG,
    pub magicNumber: SFNT_ULONG,
    pub flags: USHORT,
    pub unitsPerEm: USHORT,
    pub created: [BYTE; 8],
    pub modified: [BYTE; 8],
    pub xMin: FWord,
    pub yMin: FWord,
    pub xMax: FWord,
    pub yMax: FWord,
    pub macStyle: USHORT,
    pub lowestRecPPEM: USHORT,
    pub fontDirectionHint: SHORT,
    pub indexToLocFormat: SHORT,
    pub glyphDataFormat: SHORT,
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
pub const SFNT_TYPE_TTC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_ttc_read_offset")]
pub unsafe extern "C-unwind" fn ttc_read_offset(
    mut sfont: *mut sfnt,
    mut ttc_idx: SFNT_ULONG,
) -> SFNT_ULONG {
    let mut offset: SFNT_ULONG = 0 as SFNT_ULONG;
    let mut num_dirs: SFNT_ULONG = 0 as SFNT_ULONG;
    if sfont.is_null() || (*sfont).handle == INVALID_HANDLE {
        pdf_abort!(b"file not opened\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if (*sfont).type_0 != SFNT_TYPE_TTC {
        pdf_abort!(
            b"ttc_read_offset(): invalid font type\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    crate::backend::runtime::input_seek((*sfont).handle, 4 as ssize_t, SEEK_SET);
    tt_get_unsigned_quad((*sfont).handle);
    num_dirs = tt_get_unsigned_quad((*sfont).handle);
    if ttc_idx > num_dirs.wrapping_sub(1 as SFNT_ULONG) {
        pdf_abort!(b"Invalid TTC index number\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    crate::backend::runtime::input_seek(
        (*sfont).handle,
        (12 as SFNT_ULONG).wrapping_add(ttc_idx.wrapping_mul(4 as SFNT_ULONG)) as ssize_t,
        SEEK_SET,
    );
    offset = tt_get_unsigned_quad((*sfont).handle);
    return offset;
}
pub const FIXEDWIDTH: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const SERIF: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const SYMBOLIC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const SCRIPT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const ITALIC: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const FORCEBOLD: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_tt_get_fontdesc")]
pub unsafe extern "C-unwind" fn tt_get_fontdesc(
    mut sfont: *mut sfnt,
    mut embed: *mut ::core::ffi::c_int,
    mut stemv: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut fontname: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut descriptor: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut bbox: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut flag: ::core::ffi::c_int = SYMBOLIC;
    let mut head: *mut tt_head_table = ::core::ptr::null_mut::<tt_head_table>();
    let mut os2: *mut tt_os2__table = ::core::ptr::null_mut::<tt_os2__table>();
    let mut post: *mut tt_post_table = ::core::ptr::null_mut::<tt_post_table>();
    if sfont.is_null() {
        pdf_abort!(b"font file not opened\0" as *const u8 as *const ::core::ffi::c_char);
    }
    os2 = tt_read_os2__table(sfont);
    head = tt_read_head_table(sfont);
    post = tt_read_post_table(sfont);
    if post.is_null() {
        free(os2 as *mut ::core::ffi::c_void);
        free(head as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    descriptor = pdf_new_dict();
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"FontDescriptor\0" as *const u8 as *const ::core::ffi::c_char),
    );
    if *embed != 0 && !os2.is_null() {
        if (*os2).fsType as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            || (*os2).fsType as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
        {
            *embed = 1 as ::core::ffi::c_int;
        } else if (*os2).fsType as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int != 0 {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Font \"%s\" permits \"Preview & Print\" embedding only **\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    fontname,
                );
            }
            *embed = 1 as ::core::ffi::c_int;
        } else if dpx_conf.ignore_font_license != 0 {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Font \"%s\" may be subject to embedding restrictions **\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    fontname,
                );
            }
            *embed = 1 as ::core::ffi::c_int;
        } else {
            if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Embedding of font \"%s\" disabled due to license restrictions\0" as *const u8
                        as *const ::core::ffi::c_char,
                    fontname,
                );
            }
            *embed = 0 as ::core::ffi::c_int;
        }
    }
    if !os2.is_null() {
        pdf_add_dict(
            descriptor,
            pdf_new_name(b"Ascent\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(
                floor(
                    1000.0f64 * (*os2).sTypoAscender as ::core::ffi::c_int as ::core::ffi::c_double
                        / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                        / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        + 0.5f64,
                ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double,
            ),
        );
        pdf_add_dict(
            descriptor,
            pdf_new_name(b"Descent\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(
                floor(
                    1000.0f64
                        * (*os2).sTypoDescender as ::core::ffi::c_int as ::core::ffi::c_double
                        / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                        / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                        + 0.5f64,
                ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double,
            ),
        );
        if stemv < 0 as ::core::ffi::c_int {
            stemv = ((*os2).usWeightClass as ::core::ffi::c_int as ::core::ffi::c_double / 65.0f64
                * ((*os2).usWeightClass as ::core::ffi::c_int as ::core::ffi::c_double / 65.0f64)
                + 50 as ::core::ffi::c_int as ::core::ffi::c_double)
                as ::core::ffi::c_int;
        }
        pdf_add_dict(
            descriptor,
            pdf_new_name(b"StemV\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(stemv as ::core::ffi::c_double),
        );
        if (*os2).version as ::core::ffi::c_int == 0x2 as ::core::ffi::c_int {
            pdf_add_dict(
                descriptor,
                pdf_new_name(b"CapHeight\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_new_number(
                    floor(
                        1000.0f64
                            * (*os2).sCapHeight as ::core::ffi::c_int as ::core::ffi::c_double
                            / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                            / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                            + 0.5f64,
                    ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double,
                ),
            );
            pdf_add_dict(
                descriptor,
                pdf_new_name(b"XHeight\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_new_number(
                    floor(
                        1000.0f64 * (*os2).sxHeight as ::core::ffi::c_int as ::core::ffi::c_double
                            / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                            / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                            + 0.5f64,
                    ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double,
                ),
            );
        } else {
            pdf_add_dict(
                descriptor,
                pdf_new_name(b"CapHeight\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_new_number(
                    floor(
                        1000.0f64
                            * (*os2).sTypoAscender as ::core::ffi::c_int as ::core::ffi::c_double
                            / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                            / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                            + 0.5f64,
                    ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double,
                ),
            );
        }
        if (*os2).xAvgCharWidth as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            pdf_add_dict(
                descriptor,
                pdf_new_name(b"AvgWidth\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_new_number(
                    floor(
                        1000.0f64
                            * (*os2).xAvgCharWidth as ::core::ffi::c_int as ::core::ffi::c_double
                            / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                            / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                            + 0.5f64,
                    ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double,
                ),
            );
        }
    }
    bbox = pdf_new_array();
    pdf_add_array(
        bbox,
        pdf_new_number(
            floor(
                1000.0f64 * (*head).xMin as ::core::ffi::c_int as ::core::ffi::c_double
                    / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    + 0.5f64,
            ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double,
        ),
    );
    pdf_add_array(
        bbox,
        pdf_new_number(
            floor(
                1000.0f64 * (*head).yMin as ::core::ffi::c_int as ::core::ffi::c_double
                    / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    + 0.5f64,
            ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double,
        ),
    );
    pdf_add_array(
        bbox,
        pdf_new_number(
            floor(
                1000.0f64 * (*head).xMax as ::core::ffi::c_int as ::core::ffi::c_double
                    / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    + 0.5f64,
            ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double,
        ),
    );
    pdf_add_array(
        bbox,
        pdf_new_number(
            floor(
                1000.0f64 * (*head).yMax as ::core::ffi::c_int as ::core::ffi::c_double
                    / (*head).unitsPerEm as ::core::ffi::c_int as ::core::ffi::c_double
                    / 1 as ::core::ffi::c_int as ::core::ffi::c_double
                    + 0.5f64,
            ) * 1 as ::core::ffi::c_int as ::core::ffi::c_double,
        ),
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"FontBBox\0" as *const u8 as *const ::core::ffi::c_char),
        bbox,
    );
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"ItalicAngle\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(
            ((*post).italicAngle as ::core::ffi::c_long % 0x10000 as ::core::ffi::c_long)
                as ::core::ffi::c_double
                / 0x10000 as ::core::ffi::c_long as ::core::ffi::c_double
                + ((*post).italicAngle as ::core::ffi::c_long / 0x10000 as ::core::ffi::c_long)
                    as ::core::ffi::c_double
                - (if (*post).italicAngle as ::core::ffi::c_long / 0x10000 as ::core::ffi::c_long
                    > 0x7fff as ::core::ffi::c_long
                {
                    0x10000 as ::core::ffi::c_long
                } else {
                    0 as ::core::ffi::c_long
                }) as ::core::ffi::c_double,
        ),
    );
    if !os2.is_null() {
        if (*os2).fsSelection as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            != 0
        {
            flag |= ITALIC;
        }
        if (*os2).fsSelection as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            != 0
        {
            flag |= FORCEBOLD;
        }
        if (*os2).sFamilyClass as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int
            != 8 as ::core::ffi::c_int
        {
            flag |= SERIF;
        }
        if (*os2).sFamilyClass as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int
            == 10 as ::core::ffi::c_int
        {
            flag |= SCRIPT;
        }
        if (*post).isFixedPitch != 0 {
            flag |= FIXEDWIDTH;
        }
    }
    pdf_add_dict(
        descriptor,
        pdf_new_name(b"Flags\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(flag as ::core::ffi::c_double),
    );
    if type_0 == 0 as ::core::ffi::c_int && !os2.is_null() {
        let mut styledict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut panose: [::core::ffi::c_uchar; 12] = [0; 12];
        panose[0 as ::core::ffi::c_int as usize] = ((*os2).sFamilyClass as ::core::ffi::c_int
            >> 8 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        panose[1 as ::core::ffi::c_int as usize] = ((*os2).sFamilyClass as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        memcpy(
            (&raw mut panose as *mut ::core::ffi::c_uchar).offset(2 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            &raw mut (*os2).panose as *mut BYTE as *const ::core::ffi::c_void,
            10 as size_t,
        );
        styledict = pdf_new_dict();
        pdf_add_dict(
            styledict,
            pdf_new_name(b"Panose\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                &raw mut panose as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                12 as size_t,
            ),
        );
        pdf_add_dict(
            descriptor,
            pdf_new_name(b"Style\0" as *const u8 as *const ::core::ffi::c_char),
            styledict,
        );
    }
    free(head as *mut ::core::ffi::c_void);
    free(os2 as *mut ::core::ffi::c_void);
    tt_release_post_table(post);
    return descriptor;
}
unsafe extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_ttc_read_offset(
    mut sfont: *mut sfnt,
    mut ttc_idx: SFNT_ULONG,
    _result: *mut SFNT_ULONG,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ttc_read_offset(sfont, ttc_idx) }) {
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
pub unsafe extern "C" fn __pdf_entry_tt_get_fontdesc(
    mut sfont: *mut sfnt,
    mut embed: *mut ::core::ffi::c_int,
    mut stemv: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut fontname: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        tt_get_fontdesc(sfont, embed, stemv, type_0, fontname)
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
