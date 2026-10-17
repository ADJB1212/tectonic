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
use crate::unicode::is_valid as UC_is_valid;
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
    -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: ::core::ffi::c_int,
    ) -> size_t;
    #[link_name = "__pdf_impl_CMap_add_bfchar"]
    fn CMap_add_bfchar(
        cmap: *mut CMap,
        src: *const ::core::ffi::c_uchar,
        srcdim: size_t,
        dest: *const ::core::ffi::c_uchar,
        destdim: size_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_new_opt"]
    fn otl_new_opt() -> *mut otl_opt;
    #[link_name = "__pdf_impl_otl_release_opt"]
    fn otl_release_opt(opt: *mut otl_opt);
    #[link_name = "__pdf_impl_otl_parse_optstring"]
    fn otl_parse_optstring(
        opt: *mut otl_opt,
        optstr: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_otl_match_optrule"]
    fn otl_match_optrule(opt: *mut otl_opt, tag: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_tt_get_signed_byte"]
    fn tt_get_signed_byte(handle: rust_input_handle_t) -> ::core::ffi::c_schar;
    #[link_name = "__pdf_impl_tt_get_unsigned_pair"]
    fn tt_get_unsigned_pair(handle: rust_input_handle_t) -> ::core::ffi::c_ushort;
    #[link_name = "__pdf_impl_tt_get_signed_pair"]
    fn tt_get_signed_pair(handle: rust_input_handle_t) -> ::core::ffi::c_short;
    #[link_name = "__pdf_impl_tt_get_unsigned_quad"]
    fn tt_get_unsigned_quad(handle: rust_input_handle_t) -> uint32_t;
    #[link_name = "__pdf_impl_sfnt_find_table_pos"]
    fn sfnt_find_table_pos(sfont: *mut sfnt, tag: *const ::core::ffi::c_char) -> SFNT_ULONG;
    static mut dpx_conf: _dpx_conf;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_UC_UTF16BE_encode_char"]
    fn UC_UTF16BE_encode_char(
        ucv: int32_t,
        dstpp: *mut *mut ::core::ffi::c_uchar,
        endptr: *mut ::core::ffi::c_uchar,
    ) -> size_t;
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type uint16_t = u16;
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
pub type SFNT_CHAR = ::core::ffi::c_schar;
pub type USHORT = ::core::ffi::c_ushort;
pub type SHORT = ::core::ffi::c_short;
pub type SFNT_ULONG = uint32_t;
pub type Fixed = uint32_t;
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
pub struct otl_gsub {
    pub num_gsubs: ::core::ffi::c_int,
    pub select: ::core::ffi::c_int,
    pub first: *mut gsub_entry,
    pub gsubs: [otl_gsub_tab; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_tab {
    pub script: *mut ::core::ffi::c_char,
    pub language: *mut ::core::ffi::c_char,
    pub feature: *mut ::core::ffi::c_char,
    pub num_subtables: ::core::ffi::c_int,
    pub subtables: *mut otl_gsub_subtab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_subtab {
    pub LookupType: USHORT,
    pub SubstFormat: USHORT,
    pub table: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub single1: *mut otl_gsub_single1,
    pub single2: *mut otl_gsub_single2,
    pub alternate1: *mut otl_gsub_alternate1,
    pub ligature1: *mut otl_gsub_ligature1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_ligature1 {
    pub LigSetCount: USHORT,
    pub LigatureSet: *mut otl_gsub_ligset,
    pub coverage: clt_coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_coverage {
    pub format: USHORT,
    pub count: USHORT,
    pub list: *mut GlyphID,
    pub range: *mut clt_range,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_range {
    pub Start: GlyphID,
    pub End: GlyphID,
    pub StartCoverageIndex: USHORT,
}
pub type GlyphID = USHORT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_ligset {
    pub LigatureCount: USHORT,
    pub Ligature: *mut otl_gsub_ligtab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_ligtab {
    pub LigGlyph: GlyphID,
    pub CompCount: USHORT,
    pub Component: *mut GlyphID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_alternate1 {
    pub AlternateSetCount: USHORT,
    pub AlternateSet: *mut otl_gsub_altset,
    pub coverage: clt_coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_altset {
    pub GlyphCount: USHORT,
    pub Alternate: *mut GlyphID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_single2 {
    pub GlyphCount: USHORT,
    pub Substitute: *mut GlyphID,
    pub coverage: clt_coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_single1 {
    pub DeltaGlyphID: SHORT,
    pub coverage: clt_coverage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsub_entry {
    pub index: ::core::ffi::c_int,
    pub next: *mut gsub_entry,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_record_list {
    pub count: USHORT,
    pub record: *mut clt_record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_record {
    pub tag: [::core::ffi::c_char; 5],
    pub offset: Offset,
}
pub type Offset = USHORT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_number_list {
    pub count: USHORT,
    pub value: *mut USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_feature_table {
    pub FeatureParams: Offset,
    pub LookupListIndex: clt_number_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_lookup_table {
    pub LookupType: USHORT,
    pub LookupFlag: USHORT,
    pub SubTableList: clt_number_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_gsub_header {
    pub version: Fixed,
    pub ScriptList: Offset,
    pub FeatureList: Offset,
    pub LookupList: Offset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_script_table {
    pub DefaultLangSys: Offset,
    pub LangSysRecord: clt_record_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clt_langsys_table {
    pub LookupOrder: Offset,
    pub ReqFeatureIndex: USHORT,
    pub FeatureIndex: clt_number_list,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
pub const OTL_GSUB_TYPE_SINGLE: ::core::ffi::c_int = 1;
pub const OTL_GSUB_TYPE_ALTERNATE: ::core::ffi::c_int = 3;
pub const OTL_GSUB_TYPE_LIGATURE: ::core::ffi::c_int = 4;
pub const OTL_GSUB_TYPE_ESUBST: ::core::ffi::c_int = 7;
pub const VERBOSE_LEVEL_MIN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn clt_read_record(
    mut rec: *mut clt_record,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if !(!rec.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"rec && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        (*rec).tag[i as usize] = tt_get_signed_byte((*sfont).handle) as ::core::ffi::c_char;
        i += 1;
    }
    (*rec).tag[4 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    (*rec).offset = tt_get_unsigned_pair((*sfont).handle) as Offset;
    return 6 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn clt_read_range(
    mut rec: *mut clt_range,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    if !(!rec.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"rec && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*rec).Start = tt_get_unsigned_pair((*sfont).handle) as GlyphID;
    (*rec).End = tt_get_unsigned_pair((*sfont).handle) as GlyphID;
    (*rec).StartCoverageIndex = tt_get_unsigned_pair((*sfont).handle);
    return 6 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn clt_read_record_list(
    mut list: *mut clt_record_list,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if !(!list.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"list && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*list).count = tt_get_unsigned_pair((*sfont).handle);
    len = 2 as ::core::ffi::c_int;
    if (*list).count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*list).record = ::core::ptr::null_mut::<clt_record>();
    } else {
        (*list).record = new(((*list).count as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<clt_record>() as usize)
            as uint32_t) as *mut clt_record;
        i = 0 as ::core::ffi::c_int;
        while i < (*list).count as ::core::ffi::c_int {
            len += clt_read_record((*list).record.offset(i as isize) as *mut clt_record, sfont);
            i += 1;
        }
    }
    return len;
}
unsafe extern "C-unwind" fn clt_release_record_list(mut list: *mut clt_record_list) {
    if !list.is_null() {
        (*list).record = mfree((*list).record as *mut ::core::ffi::c_void) as *mut clt_record;
        (*list).count = 0 as USHORT;
    }
}
unsafe extern "C-unwind" fn clt_read_number_list(
    mut list: *mut clt_number_list,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if !(!list.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"list && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*list).count = tt_get_unsigned_pair((*sfont).handle);
    if (*list).count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*list).value = ::core::ptr::null_mut::<USHORT>();
    } else {
        (*list).value = new(((*list).count as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<USHORT>() as usize)
            as uint32_t) as *mut USHORT;
        i = 0 as ::core::ffi::c_int;
        while i < (*list).count as ::core::ffi::c_int {
            *(*list).value.offset(i as isize) = tt_get_unsigned_pair((*sfont).handle);
            i += 1;
        }
    }
    return 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * (*list).count as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn clt_release_number_list(mut list: *mut clt_number_list) {
    if !list.is_null() {
        (*list).value = mfree((*list).value as *mut ::core::ffi::c_void) as *mut USHORT;
        (*list).count = 0 as USHORT;
    }
}
unsafe extern "C-unwind" fn clt_read_script_table(
    mut tab: *mut clt_script_table,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    if !(!tab.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"tab && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*tab).DefaultLangSys = tt_get_unsigned_pair((*sfont).handle) as Offset;
    len = 2 as ::core::ffi::c_int;
    len += clt_read_record_list(&raw mut (*tab).LangSysRecord, sfont);
    return len;
}
unsafe extern "C-unwind" fn clt_release_script_table(mut tab: *mut clt_script_table) {
    if !tab.is_null() {
        clt_release_record_list(&raw mut (*tab).LangSysRecord);
    }
}
unsafe extern "C-unwind" fn clt_read_langsys_table(
    mut tab: *mut clt_langsys_table,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    if !(!tab.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"tab && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*tab).LookupOrder = tt_get_unsigned_pair((*sfont).handle) as Offset;
    (*tab).ReqFeatureIndex = tt_get_unsigned_pair((*sfont).handle);
    len = 4 as ::core::ffi::c_int;
    len += clt_read_number_list(&raw mut (*tab).FeatureIndex, sfont);
    return len;
}
unsafe extern "C-unwind" fn clt_release_langsys_table(mut tab: *mut clt_langsys_table) {
    if !tab.is_null() {
        clt_release_number_list(&raw mut (*tab).FeatureIndex);
    }
}
unsafe extern "C-unwind" fn clt_read_feature_table(
    mut tab: *mut clt_feature_table,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    if !(!tab.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"tab && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*tab).FeatureParams = tt_get_unsigned_pair((*sfont).handle) as Offset;
    len = 2 as ::core::ffi::c_int;
    len += clt_read_number_list(&raw mut (*tab).LookupListIndex, sfont);
    return len;
}
unsafe extern "C-unwind" fn clt_release_feature_table(mut tab: *mut clt_feature_table) {
    if !tab.is_null() {
        clt_release_number_list(&raw mut (*tab).LookupListIndex);
    }
}
unsafe extern "C-unwind" fn clt_read_lookup_table(
    mut tab: *mut clt_lookup_table,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    if !(!tab.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"tab && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*tab).LookupType = tt_get_unsigned_pair((*sfont).handle);
    (*tab).LookupFlag = tt_get_unsigned_pair((*sfont).handle);
    len = 4 as ::core::ffi::c_int;
    len += clt_read_number_list(&raw mut (*tab).SubTableList, sfont);
    return len;
}
unsafe extern "C-unwind" fn clt_release_lookup_table(mut tab: *mut clt_lookup_table) {
    if !tab.is_null() {
        clt_release_number_list(&raw mut (*tab).SubTableList);
    }
}
unsafe extern "C-unwind" fn clt_read_coverage(
    mut cov: *mut clt_coverage,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if !(!cov.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cov && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*cov).format = tt_get_unsigned_pair((*sfont).handle);
    (*cov).count = tt_get_unsigned_pair((*sfont).handle);
    len = 4 as ::core::ffi::c_int;
    match (*cov).format as ::core::ffi::c_int {
        1 => {
            if (*cov).count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*cov).list = ::core::ptr::null_mut::<GlyphID>();
            } else {
                (*cov).list = new(((*cov).count as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<USHORT>() as usize)
                    as uint32_t) as *mut USHORT as *mut GlyphID;
                i = 0 as ::core::ffi::c_int;
                while i < (*cov).count as ::core::ffi::c_int {
                    *(*cov).list.offset(i as isize) =
                        tt_get_unsigned_pair((*sfont).handle) as GlyphID;
                    i += 1;
                }
            }
            (*cov).range = ::core::ptr::null_mut::<clt_range>();
            len += 2 as ::core::ffi::c_int * (*cov).count as ::core::ffi::c_int;
        }
        2 => {
            if (*cov).count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*cov).range = ::core::ptr::null_mut::<clt_range>();
            } else {
                (*cov).range = new(((*cov).count as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<clt_range>() as usize)
                    as uint32_t) as *mut clt_range;
                i = 0 as ::core::ffi::c_int;
                while i < (*cov).count as ::core::ffi::c_int {
                    len += clt_read_range((*cov).range.offset(i as isize) as *mut clt_range, sfont);
                    i += 1;
                }
            }
            (*cov).list = ::core::ptr::null_mut::<GlyphID>();
        }
        _ => {
            pdf_abort!(b"Unknown coverage format\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    return len;
}
unsafe extern "C-unwind" fn clt_release_coverage(mut cov: *mut clt_coverage) {
    if !cov.is_null() {
        match (*cov).format as ::core::ffi::c_int {
            1 => {
                (*cov).list = mfree((*cov).list as *mut ::core::ffi::c_void) as *mut GlyphID;
            }
            2 => {
                (*cov).range = mfree((*cov).range as *mut ::core::ffi::c_void) as *mut clt_range;
            }
            _ => {
                pdf_abort!(b"Unknown coverage format\0" as *const u8 as *const ::core::ffi::c_char,);
            }
        }
    }
    (*cov).count = 0 as USHORT;
}
unsafe extern "C-unwind" fn clt_lookup_coverage(
    mut cov: *mut clt_coverage,
    mut gid: USHORT,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if cov.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cov\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    match (*cov).format as ::core::ffi::c_int {
        1 => {
            i = 0 as ::core::ffi::c_int;
            while i < (*cov).count as ::core::ffi::c_int {
                if *(*cov).list.offset(i as isize) as ::core::ffi::c_int > gid as ::core::ffi::c_int
                {
                    break;
                }
                if *(*cov).list.offset(i as isize) as ::core::ffi::c_int
                    == gid as ::core::ffi::c_int
                {
                    return i;
                }
                i += 1;
            }
        }
        2 => {
            i = 0 as ::core::ffi::c_int;
            while i < (*cov).count as ::core::ffi::c_int {
                if (gid as ::core::ffi::c_int)
                    < (*(*cov).range.offset(i as isize)).Start as ::core::ffi::c_int
                {
                    break;
                }
                if gid as ::core::ffi::c_int
                    <= (*(*cov).range.offset(i as isize)).End as ::core::ffi::c_int
                {
                    return (*(*cov).range.offset(i as isize)).StartCoverageIndex
                        as ::core::ffi::c_int
                        + gid as ::core::ffi::c_int
                        - (*(*cov).range.offset(i as isize)).Start as ::core::ffi::c_int;
                }
                i += 1;
            }
        }
        _ => {
            pdf_abort!(b"Unknown coverage format\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn otl_gsub_read_single(
    mut subtab: *mut otl_gsub_subtab,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    let mut offset: SFNT_ULONG = 0;
    let mut cov_offset: Offset = 0;
    if !(!subtab.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"subtab && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    offset =
        crate::backend::runtime::input_seek((*sfont).handle, 0 as ssize_t, SEEK_CUR) as SFNT_ULONG;
    (*subtab).LookupType = OTL_GSUB_TYPE_SINGLE as USHORT;
    (*subtab).SubstFormat = tt_get_unsigned_pair((*sfont).handle);
    len = 2 as ::core::ffi::c_int;
    if (*subtab).SubstFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        let mut data: *mut otl_gsub_single1 = ::core::ptr::null_mut::<otl_gsub_single1>();
        data = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<otl_gsub_single1>() as usize)
            as uint32_t) as *mut otl_gsub_single1;
        (*subtab).table.single1 = data;
        cov_offset = tt_get_unsigned_pair((*sfont).handle) as Offset;
        (*data).DeltaGlyphID = tt_get_signed_pair((*sfont).handle);
        len += 4 as ::core::ffi::c_int;
        crate::backend::runtime::input_seek(
            (*sfont).handle,
            offset.wrapping_add(cov_offset as SFNT_ULONG) as ssize_t,
            SEEK_SET,
        );
        len += clt_read_coverage(&raw mut (*data).coverage, sfont);
    } else if (*subtab).SubstFormat as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        let mut data_0: *mut otl_gsub_single2 = ::core::ptr::null_mut::<otl_gsub_single2>();
        let mut count: USHORT = 0;
        data_0 = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<otl_gsub_single2>() as usize)
            as uint32_t) as *mut otl_gsub_single2;
        (*subtab).table.single2 = data_0;
        cov_offset = tt_get_unsigned_pair((*sfont).handle) as Offset;
        (*data_0).GlyphCount = tt_get_unsigned_pair((*sfont).handle);
        len += 4 as ::core::ffi::c_int;
        if (*data_0).GlyphCount as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*data_0).Substitute = ::core::ptr::null_mut::<GlyphID>();
        } else {
            (*data_0).Substitute = new(((*data_0).GlyphCount as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<GlyphID>() as usize)
                as uint32_t) as *mut GlyphID;
            count = 0 as USHORT;
            while (count as ::core::ffi::c_int) < (*data_0).GlyphCount as ::core::ffi::c_int {
                *(*data_0).Substitute.offset(count as isize) =
                    tt_get_unsigned_pair((*sfont).handle) as GlyphID;
                count = count.wrapping_add(1);
            }
            len += 2 as ::core::ffi::c_int * (*data_0).GlyphCount as ::core::ffi::c_int;
        }
        crate::backend::runtime::input_seek(
            (*sfont).handle,
            offset.wrapping_add(cov_offset as SFNT_ULONG) as ssize_t,
            SEEK_SET,
        );
        len += clt_read_coverage(&raw mut (*data_0).coverage, sfont);
    } else {
        pdf_abort!(b"unexpected SubstFormat\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    return len;
}
unsafe extern "C-unwind" fn otl_gsub_read_alternate(
    mut subtab: *mut otl_gsub_subtab,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    let mut i: USHORT = 0;
    let mut j: USHORT = 0;
    let mut offset: SFNT_ULONG = 0;
    let mut cov_offset: Offset = 0;
    let mut altset_offsets: clt_number_list = clt_number_list {
        count: 0,
        value: ::core::ptr::null_mut::<USHORT>(),
    };
    let mut data: *mut otl_gsub_alternate1 = ::core::ptr::null_mut::<otl_gsub_alternate1>();
    if !(!subtab.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"subtab && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    offset =
        crate::backend::runtime::input_seek((*sfont).handle, 0 as ssize_t, SEEK_CUR) as SFNT_ULONG;
    (*subtab).LookupType = OTL_GSUB_TYPE_ALTERNATE as USHORT;
    (*subtab).SubstFormat = tt_get_unsigned_pair((*sfont).handle);
    if (*subtab).SubstFormat as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"Unknown GSUB SubstFormat for Alternate: %u\0" as *const u8
                as *const ::core::ffi::c_char,
            (*subtab).SubstFormat as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    }
    len = 2 as ::core::ffi::c_int;
    data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<otl_gsub_alternate1>() as usize)
        as uint32_t) as *mut otl_gsub_alternate1;
    (*subtab).table.alternate1 = data;
    cov_offset = tt_get_unsigned_pair((*sfont).handle) as Offset;
    len += 2 as ::core::ffi::c_int;
    len += clt_read_number_list(&raw mut altset_offsets, sfont);
    (*data).AlternateSetCount = altset_offsets.count;
    if (*data).AlternateSetCount as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*data).AlternateSet = ::core::ptr::null_mut::<otl_gsub_altset>();
    } else {
        (*data).AlternateSet = new(((*data).AlternateSetCount as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<otl_gsub_altset>() as usize)
            as uint32_t) as *mut otl_gsub_altset;
        i = 0 as USHORT;
        while (i as ::core::ffi::c_int) < (*data).AlternateSetCount as ::core::ffi::c_int {
            let mut altset: *mut otl_gsub_altset = ::core::ptr::null_mut::<otl_gsub_altset>();
            let mut altset_offset: SFNT_ULONG = 0;
            altset = (*data).AlternateSet.offset(i as isize) as *mut otl_gsub_altset;
            altset_offset =
                offset.wrapping_add(*altset_offsets.value.offset(i as isize) as SFNT_ULONG);
            crate::backend::runtime::input_seek(
                (*sfont).handle,
                altset_offset as ssize_t,
                SEEK_SET,
            );
            (*altset).GlyphCount = tt_get_unsigned_pair((*sfont).handle);
            len += 2 as ::core::ffi::c_int;
            if (*altset).GlyphCount as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*altset).Alternate = ::core::ptr::null_mut::<GlyphID>();
            } else {
                (*altset).Alternate = new(((*altset).GlyphCount as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<GlyphID>() as usize)
                    as uint32_t) as *mut GlyphID;
                j = 0 as USHORT;
                while (j as ::core::ffi::c_int) < (*altset).GlyphCount as ::core::ffi::c_int {
                    *(*altset).Alternate.offset(j as isize) =
                        tt_get_unsigned_pair((*sfont).handle) as GlyphID;
                    len += 2 as ::core::ffi::c_int;
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
        clt_release_number_list(&raw mut altset_offsets);
    }
    crate::backend::runtime::input_seek(
        (*sfont).handle,
        offset.wrapping_add(cov_offset as SFNT_ULONG) as ssize_t,
        SEEK_SET,
    );
    len += clt_read_coverage(&raw mut (*data).coverage, sfont);
    return len;
}
unsafe extern "C-unwind" fn otl_gsub_read_ligature(
    mut subtab: *mut otl_gsub_subtab,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    let mut i: USHORT = 0;
    let mut j: USHORT = 0;
    let mut offset: SFNT_ULONG = 0;
    let mut cov_offset: Offset = 0;
    let mut ligset_offsets: clt_number_list = clt_number_list {
        count: 0,
        value: ::core::ptr::null_mut::<USHORT>(),
    };
    let mut data: *mut otl_gsub_ligature1 = ::core::ptr::null_mut::<otl_gsub_ligature1>();
    if !(!subtab.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"subtab && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    offset =
        crate::backend::runtime::input_seek((*sfont).handle, 0 as ssize_t, SEEK_CUR) as SFNT_ULONG;
    (*subtab).LookupType = OTL_GSUB_TYPE_LIGATURE as USHORT;
    (*subtab).SubstFormat = tt_get_unsigned_pair((*sfont).handle);
    if (*subtab).SubstFormat as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
        pdf_warning!(
            b"Unknown GSUB SubstFormat for Ligature: %u\0" as *const u8
                as *const ::core::ffi::c_char,
            (*subtab).SubstFormat as ::core::ffi::c_int,
        );
        return -(1 as ::core::ffi::c_int);
    }
    len = 2 as ::core::ffi::c_int;
    data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<otl_gsub_ligature1>() as usize)
        as uint32_t) as *mut otl_gsub_ligature1;
    (*subtab).table.ligature1 = data;
    cov_offset = tt_get_unsigned_pair((*sfont).handle) as Offset;
    len += 2 as ::core::ffi::c_int;
    len += clt_read_number_list(&raw mut ligset_offsets, sfont);
    (*data).LigSetCount = ligset_offsets.count;
    if (*data).LigSetCount as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        (*data).LigatureSet = ::core::ptr::null_mut::<otl_gsub_ligset>();
    } else {
        (*data).LigatureSet = new(((*data).LigSetCount as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<otl_gsub_ligset>() as usize)
            as uint32_t) as *mut otl_gsub_ligset;
        i = 0 as USHORT;
        while (i as ::core::ffi::c_int) < (*data).LigSetCount as ::core::ffi::c_int {
            let mut ligset_tab: clt_number_list = clt_number_list {
                count: 0,
                value: ::core::ptr::null_mut::<USHORT>(),
            };
            let mut ligset: *mut otl_gsub_ligset = ::core::ptr::null_mut::<otl_gsub_ligset>();
            let mut ligset_offset: SFNT_ULONG = 0;
            let mut count: USHORT = 0;
            ligset = (*data).LigatureSet.offset(i as isize) as *mut otl_gsub_ligset;
            ligset_offset =
                offset.wrapping_add(*ligset_offsets.value.offset(i as isize) as SFNT_ULONG);
            crate::backend::runtime::input_seek(
                (*sfont).handle,
                ligset_offset as ssize_t,
                SEEK_SET,
            );
            len += clt_read_number_list(&raw mut ligset_tab, sfont);
            (*ligset).LigatureCount = ligset_tab.count;
            if ligset_tab.count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*ligset).Ligature = ::core::ptr::null_mut::<otl_gsub_ligtab>();
            } else {
                (*ligset).Ligature = new((ligset_tab.count as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<otl_gsub_ligtab>() as usize)
                    as uint32_t) as *mut otl_gsub_ligtab;
                j = 0 as USHORT;
                while (j as ::core::ffi::c_int) < ligset_tab.count as ::core::ffi::c_int {
                    crate::backend::runtime::input_seek(
                        (*sfont).handle,
                        ligset_offset
                            .wrapping_add(*ligset_tab.value.offset(j as isize) as SFNT_ULONG)
                            as ssize_t,
                        SEEK_SET,
                    );
                    (*(*ligset).Ligature.offset(j as isize)).LigGlyph =
                        tt_get_unsigned_pair((*sfont).handle) as GlyphID;
                    (*(*ligset).Ligature.offset(j as isize)).CompCount =
                        tt_get_unsigned_pair((*sfont).handle);
                    if (*(*ligset).Ligature.offset(j as isize)).CompCount as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let ref mut fresh1 = (*(*ligset).Ligature.offset(j as isize)).Component;
                        *fresh1 = ::core::ptr::null_mut::<GlyphID>();
                    } else {
                        let ref mut fresh2 = (*(*ligset).Ligature.offset(j as isize)).Component;
                        *fresh2 = new((((*(*ligset).Ligature.offset(j as isize)).CompCount
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as uint32_t as usize)
                            .wrapping_mul(::core::mem::size_of::<GlyphID>() as usize)
                            as uint32_t) as *mut GlyphID;
                        count = 0 as USHORT;
                        while (count as ::core::ffi::c_int)
                            < (*(*ligset).Ligature.offset(j as isize)).CompCount
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                        {
                            *(*(*ligset).Ligature.offset(j as isize))
                                .Component
                                .offset(count as isize) =
                                tt_get_unsigned_pair((*sfont).handle) as GlyphID;
                            count = count.wrapping_add(1);
                        }
                        len += 4 as ::core::ffi::c_int
                            + count as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
                    }
                    j = j.wrapping_add(1);
                }
                clt_release_number_list(&raw mut ligset_tab);
            }
            i = i.wrapping_add(1);
        }
    }
    clt_release_number_list(&raw mut ligset_offsets);
    crate::backend::runtime::input_seek(
        (*sfont).handle,
        offset.wrapping_add(cov_offset as SFNT_ULONG) as ssize_t,
        SEEK_SET,
    );
    len += clt_read_coverage(&raw mut (*data).coverage, sfont);
    return len;
}
unsafe extern "C-unwind" fn otl_gsub_release_single(mut subtab: *mut otl_gsub_subtab) {
    if !subtab.is_null() {
        match (*subtab).SubstFormat as ::core::ffi::c_int {
            1 => {
                let mut data: *mut otl_gsub_single1 = ::core::ptr::null_mut::<otl_gsub_single1>();
                data = (*subtab).table.single1;
                if !data.is_null() {
                    clt_release_coverage(&raw mut (*data).coverage);
                    free(data as *mut ::core::ffi::c_void);
                }
                (*subtab).table.single1 = ::core::ptr::null_mut::<otl_gsub_single1>();
            }
            2 => {
                let mut data_0: *mut otl_gsub_single2 = ::core::ptr::null_mut::<otl_gsub_single2>();
                data_0 = (*subtab).table.single2;
                if !data_0.is_null() {
                    free((*data_0).Substitute as *mut ::core::ffi::c_void);
                    clt_release_coverage(&raw mut (*data_0).coverage);
                    free(data_0 as *mut ::core::ffi::c_void);
                }
                (*subtab).table.single2 = ::core::ptr::null_mut::<otl_gsub_single2>();
            }
            _ => {
                pdf_abort!(
                    b"Unknown format for single substitution\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        }
    }
}
unsafe extern "C-unwind" fn otl_gsub_release_ligature(mut subtab: *mut otl_gsub_subtab) {
    if !subtab.is_null() {
        let mut data: *mut otl_gsub_ligature1 = ::core::ptr::null_mut::<otl_gsub_ligature1>();
        let mut i: USHORT = 0;
        let mut j: USHORT = 0;
        data = (*subtab).table.ligature1;
        if !data.is_null() && !(*data).LigatureSet.is_null() {
            i = 0 as USHORT;
            while (i as ::core::ffi::c_int) < (*data).LigSetCount as ::core::ffi::c_int {
                let mut ligset: *mut otl_gsub_ligset = ::core::ptr::null_mut::<otl_gsub_ligset>();
                ligset = (*data).LigatureSet.offset(i as isize) as *mut otl_gsub_ligset;
                j = 0 as USHORT;
                while (j as ::core::ffi::c_int) < (*ligset).LigatureCount as ::core::ffi::c_int {
                    let ref mut fresh0 = (*(*ligset).Ligature.offset(j as isize)).Component;
                    *fresh0 = mfree(
                        (*(*ligset).Ligature.offset(j as isize)).Component
                            as *mut ::core::ffi::c_void,
                    ) as *mut GlyphID;
                    j = j.wrapping_add(1);
                }
                (*ligset).Ligature =
                    mfree((*ligset).Ligature as *mut ::core::ffi::c_void) as *mut otl_gsub_ligtab;
                i = i.wrapping_add(1);
            }
            free((*data).LigatureSet as *mut ::core::ffi::c_void);
        }
        clt_release_coverage(&raw mut (*data).coverage);
        (*data).LigatureSet = ::core::ptr::null_mut::<otl_gsub_ligset>();
        free(data as *mut ::core::ffi::c_void);
        (*subtab).table.ligature1 = ::core::ptr::null_mut::<otl_gsub_ligature1>();
    }
}
unsafe extern "C-unwind" fn otl_gsub_release_alternate(mut subtab: *mut otl_gsub_subtab) {
    if !subtab.is_null() {
        let mut data: *mut otl_gsub_alternate1 = ::core::ptr::null_mut::<otl_gsub_alternate1>();
        let mut i: USHORT = 0;
        data = (*subtab).table.alternate1;
        if !data.is_null() && !(*data).AlternateSet.is_null() {
            i = 0 as USHORT;
            while (i as ::core::ffi::c_int) < (*data).AlternateSetCount as ::core::ffi::c_int {
                let mut altset: *mut otl_gsub_altset = ::core::ptr::null_mut::<otl_gsub_altset>();
                altset = (*data).AlternateSet.offset(i as isize) as *mut otl_gsub_altset;
                (*altset).Alternate =
                    mfree((*altset).Alternate as *mut ::core::ffi::c_void) as *mut GlyphID;
                i = i.wrapping_add(1);
            }
            free((*data).AlternateSet as *mut ::core::ffi::c_void);
        }
        clt_release_coverage(&raw mut (*data).coverage);
        (*data).AlternateSet = ::core::ptr::null_mut::<otl_gsub_altset>();
        free(data as *mut ::core::ffi::c_void);
        (*subtab).table.alternate1 = ::core::ptr::null_mut::<otl_gsub_alternate1>();
    }
}
unsafe extern "C-unwind" fn otl_gsub_read_header(
    mut head: *mut otl_gsub_header,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    if !(!head.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"head && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*head).version = tt_get_unsigned_quad((*sfont).handle) as Fixed;
    (*head).ScriptList = tt_get_unsigned_pair((*sfont).handle) as Offset;
    (*head).FeatureList = tt_get_unsigned_pair((*sfont).handle) as Offset;
    (*head).LookupList = tt_get_unsigned_pair((*sfont).handle) as Offset;
    return 10 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn otl_gsub_read_feat(
    mut gsub: *mut otl_gsub_tab,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut feat_idx: ::core::ffi::c_int = 0;
    let mut script_idx: ::core::ffi::c_int = 0;
    let mut gsub_offset: SFNT_ULONG = 0;
    let mut offset: SFNT_ULONG = 0;
    let mut head: otl_gsub_header = otl_gsub_header {
        version: 0,
        ScriptList: 0,
        FeatureList: 0,
        LookupList: 0,
    };
    let mut subtab: *mut otl_gsub_subtab = ::core::ptr::null_mut::<otl_gsub_subtab>();
    let mut num_subtabs: USHORT = 0 as USHORT;
    let mut feat_bits: [::core::ffi::c_uchar; 8192] = [0; 8192];
    let mut feature_list: clt_record_list = clt_record_list {
        count: 0,
        record: ::core::ptr::null_mut::<clt_record>(),
    };
    let mut script_list: clt_record_list = clt_record_list {
        count: 0,
        record: ::core::ptr::null_mut::<clt_record>(),
    };
    let mut lookup_list: clt_number_list = clt_number_list {
        count: 0,
        value: ::core::ptr::null_mut::<USHORT>(),
    };
    let mut script: *mut otl_opt = ::core::ptr::null_mut::<otl_opt>();
    let mut language: *mut otl_opt = ::core::ptr::null_mut::<otl_opt>();
    let mut feature: *mut otl_opt = ::core::ptr::null_mut::<otl_opt>();
    if !(!gsub.is_null() && !sfont.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"gsub && sfont\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    gsub_offset = sfnt_find_table_pos(sfont, b"GSUB\0" as *const u8 as *const ::core::ffi::c_char);
    if gsub_offset == 0 as SFNT_ULONG {
        return -(1 as ::core::ffi::c_int);
    }
    script = otl_new_opt();
    otl_parse_optstring(script, (*gsub).script);
    language = otl_new_opt();
    otl_parse_optstring(language, (*gsub).language);
    feature = otl_new_opt();
    otl_parse_optstring(feature, (*gsub).feature);
    memset(
        &raw mut feat_bits as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        8192 as size_t,
    );
    crate::backend::runtime::input_seek((*sfont).handle, gsub_offset as ssize_t, SEEK_SET);
    otl_gsub_read_header(&raw mut head, sfont);
    offset = gsub_offset.wrapping_add(head.ScriptList as SFNT_ULONG);
    crate::backend::runtime::input_seek((*sfont).handle, offset as ssize_t, SEEK_SET);
    clt_read_record_list(&raw mut script_list, sfont);
    script_idx = 0 as ::core::ffi::c_int;
    while script_idx < script_list.count as ::core::ffi::c_int {
        if otl_match_optrule(
            script,
            &raw mut (*script_list.record.offset(script_idx as isize)).tag
                as *mut ::core::ffi::c_char,
        ) != 0
        {
            let mut script_tab: clt_script_table = clt_script_table {
                DefaultLangSys: 0,
                LangSysRecord: clt_record_list {
                    count: 0,
                    record: ::core::ptr::null_mut::<clt_record>(),
                },
            };
            let mut langsys_idx: ::core::ffi::c_int = 0;
            offset = gsub_offset
                .wrapping_add(head.ScriptList as SFNT_ULONG)
                .wrapping_add(
                    (*script_list.record.offset(script_idx as isize)).offset as SFNT_ULONG,
                );
            crate::backend::runtime::input_seek((*sfont).handle, offset as ssize_t, SEEK_SET);
            clt_read_script_table(&raw mut script_tab, sfont);
            if otl_match_optrule(
                language,
                b"dflt\0" as *const u8 as *const ::core::ffi::c_char,
            ) != 0
                && script_tab.DefaultLangSys as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                let mut langsys_tab: clt_langsys_table = clt_langsys_table {
                    LookupOrder: 0,
                    ReqFeatureIndex: 0,
                    FeatureIndex: clt_number_list {
                        count: 0,
                        value: ::core::ptr::null_mut::<USHORT>(),
                    },
                };
                if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
                    pdf_message!(
                        b"otl_gsub>> OTL script-language enabled: %c%c%c%c.dflt\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*script_list.record.offset(script_idx as isize)).tag
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int,
                        (*script_list.record.offset(script_idx as isize)).tag
                            [1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int,
                        (*script_list.record.offset(script_idx as isize)).tag
                            [2 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int,
                        (*script_list.record.offset(script_idx as isize)).tag
                            [3 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int,
                    );
                }
                crate::backend::runtime::input_seek(
                    (*sfont).handle,
                    offset.wrapping_add(script_tab.DefaultLangSys as SFNT_ULONG) as ssize_t,
                    SEEK_SET,
                );
                clt_read_langsys_table(&raw mut langsys_tab, sfont);
                if otl_match_optrule(
                    feature,
                    b"____\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0
                    && langsys_tab.ReqFeatureIndex as ::core::ffi::c_int
                        != 0xffff as ::core::ffi::c_int
                {
                    feat_bits[(langsys_tab.ReqFeatureIndex as ::core::ffi::c_int
                        / 8 as ::core::ffi::c_int) as usize] =
                        (feat_bits[(langsys_tab.ReqFeatureIndex as ::core::ffi::c_int
                            / 8 as ::core::ffi::c_int) as usize]
                            as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 7 as ::core::ffi::c_int
                                    - langsys_tab.ReqFeatureIndex as ::core::ffi::c_int
                                        % 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                }
                feat_idx = 0 as ::core::ffi::c_int;
                while feat_idx < langsys_tab.FeatureIndex.count as ::core::ffi::c_int {
                    feat_bits[(*langsys_tab.FeatureIndex.value.offset(feat_idx as isize)
                        as ::core::ffi::c_int
                        / 8 as ::core::ffi::c_int) as usize] =
                        (feat_bits[(*langsys_tab.FeatureIndex.value.offset(feat_idx as isize)
                            as ::core::ffi::c_int
                            / 8 as ::core::ffi::c_int) as usize]
                            as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 7 as ::core::ffi::c_int
                                    - *langsys_tab.FeatureIndex.value.offset(feat_idx as isize)
                                        as ::core::ffi::c_int
                                        % 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                    feat_idx += 1;
                }
                clt_release_langsys_table(&raw mut langsys_tab);
            }
            langsys_idx = 0 as ::core::ffi::c_int;
            while langsys_idx < script_tab.LangSysRecord.count as ::core::ffi::c_int {
                let mut langsys_rec: *mut clt_record = ::core::ptr::null_mut::<clt_record>();
                langsys_rec =
                    script_tab.LangSysRecord.record.offset(langsys_idx as isize) as *mut clt_record;
                if otl_match_optrule(
                    language,
                    &raw mut (*langsys_rec).tag as *mut ::core::ffi::c_char,
                ) != 0
                {
                    let mut langsys_tab_0: clt_langsys_table = clt_langsys_table {
                        LookupOrder: 0,
                        ReqFeatureIndex: 0,
                        FeatureIndex: clt_number_list {
                            count: 0,
                            value: ::core::ptr::null_mut::<USHORT>(),
                        },
                    };
                    if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
                        pdf_message!(
                            b"otl_gsub>> OTL script-language enabled: %c%c%c%c.%c%c%c%c\n\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            (*script_list.record.offset(script_idx as isize)).tag
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            (*script_list.record.offset(script_idx as isize)).tag
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            (*script_list.record.offset(script_idx as isize)).tag
                                [2 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            (*script_list.record.offset(script_idx as isize)).tag
                                [3 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            (*langsys_rec).tag[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            (*langsys_rec).tag[1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            (*langsys_rec).tag[2 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            (*langsys_rec).tag[3 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                        );
                    }
                    crate::backend::runtime::input_seek(
                        (*sfont).handle,
                        offset.wrapping_add((*langsys_rec).offset as SFNT_ULONG) as ssize_t,
                        SEEK_SET,
                    );
                    clt_read_langsys_table(&raw mut langsys_tab_0, sfont);
                    if otl_match_optrule(
                        feature,
                        b"____\0" as *const u8 as *const ::core::ffi::c_char,
                    ) != 0
                        || langsys_tab_0.ReqFeatureIndex as ::core::ffi::c_int
                            != 0xffff as ::core::ffi::c_int
                    {
                        feat_bits[(langsys_tab_0.ReqFeatureIndex as ::core::ffi::c_int
                            / 8 as ::core::ffi::c_int)
                            as usize] =
                            (feat_bits[(langsys_tab_0.ReqFeatureIndex as ::core::ffi::c_int
                                / 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int)
                                    << 7 as ::core::ffi::c_int
                                        - langsys_tab_0.ReqFeatureIndex as ::core::ffi::c_int
                                            % 8 as ::core::ffi::c_int)
                                as ::core::ffi::c_uchar;
                    }
                    feat_idx = 0 as ::core::ffi::c_int;
                    while feat_idx < langsys_tab_0.FeatureIndex.count as ::core::ffi::c_int {
                        feat_bits[(*langsys_tab_0.FeatureIndex.value.offset(feat_idx as isize)
                            as ::core::ffi::c_int
                            / 8 as ::core::ffi::c_int)
                            as usize] = (feat_bits[(*langsys_tab_0
                            .FeatureIndex
                            .value
                            .offset(feat_idx as isize)
                            as ::core::ffi::c_int
                            / 8 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int)
                                << 7 as ::core::ffi::c_int
                                    - *langsys_tab_0.FeatureIndex.value.offset(feat_idx as isize)
                                        as ::core::ffi::c_int
                                        % 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        feat_idx += 1;
                    }
                    clt_release_langsys_table(&raw mut langsys_tab_0);
                }
                langsys_idx += 1;
            }
            clt_release_script_table(&raw mut script_tab);
        }
        script_idx += 1;
    }
    offset = gsub_offset.wrapping_add(head.FeatureList as SFNT_ULONG);
    crate::backend::runtime::input_seek((*sfont).handle, offset as ssize_t, SEEK_SET);
    clt_read_record_list(&raw mut feature_list, sfont);
    offset = gsub_offset.wrapping_add(head.LookupList as SFNT_ULONG);
    crate::backend::runtime::input_seek((*sfont).handle, offset as ssize_t, SEEK_SET);
    clt_read_number_list(&raw mut lookup_list, sfont);
    if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
        pdf_message!(
            b"otl_gsub>> Reading OTL feature(s):\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    feat_idx = 0 as ::core::ffi::c_int;
    while feat_idx < feature_list.count as ::core::ffi::c_int {
        if feat_bits[(feat_idx / 8 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << 7 as ::core::ffi::c_int - feat_idx % 8 as ::core::ffi::c_int
            != 0
            && otl_match_optrule(
                feature,
                &raw mut (*feature_list.record.offset(feat_idx as isize)).tag
                    as *mut ::core::ffi::c_char,
            ) != 0
        {
            let mut feature_table: clt_feature_table = clt_feature_table {
                FeatureParams: 0,
                LookupListIndex: clt_number_list {
                    count: 0,
                    value: ::core::ptr::null_mut::<USHORT>(),
                },
            };
            let mut i: ::core::ffi::c_int = 0;
            if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
                pdf_message!(
                    b" %c%c%c%c\0" as *const u8 as *const ::core::ffi::c_char,
                    (*feature_list.record.offset(feat_idx as isize)).tag
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    (*feature_list.record.offset(feat_idx as isize)).tag
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    (*feature_list.record.offset(feat_idx as isize)).tag
                        [2 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    (*feature_list.record.offset(feat_idx as isize)).tag
                        [3 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                );
            }
            offset = gsub_offset
                .wrapping_add(head.FeatureList as SFNT_ULONG)
                .wrapping_add(
                    (*feature_list.record.offset(feat_idx as isize)).offset as SFNT_ULONG,
                );
            crate::backend::runtime::input_seek((*sfont).handle, offset as ssize_t, SEEK_SET);
            clt_read_feature_table(&raw mut feature_table, sfont);
            i = 0 as ::core::ffi::c_int;
            while i < feature_table.LookupListIndex.count as ::core::ffi::c_int {
                let mut lookup_table: clt_lookup_table = clt_lookup_table {
                    LookupType: 0,
                    LookupFlag: 0,
                    SubTableList: clt_number_list {
                        count: 0,
                        value: ::core::ptr::null_mut::<USHORT>(),
                    },
                };
                let mut ll_idx: ::core::ffi::c_int = 0;
                let mut st_idx: ::core::ffi::c_int = 0;
                let mut r: ::core::ffi::c_int = 0;
                let mut n_st: ::core::ffi::c_int = 0;
                ll_idx =
                    *feature_table.LookupListIndex.value.offset(i as isize) as ::core::ffi::c_int;
                if ll_idx >= lookup_list.count as ::core::ffi::c_int {
                    pdf_abort!(
                        b"invalid Lookup index.\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                offset = gsub_offset
                    .wrapping_add(head.LookupList as SFNT_ULONG)
                    .wrapping_add(*lookup_list.value.offset(ll_idx as isize) as SFNT_ULONG);
                crate::backend::runtime::input_seek((*sfont).handle, offset as ssize_t, SEEK_SET);
                clt_read_lookup_table(&raw mut lookup_table, sfont);
                if lookup_table.LookupType as ::core::ffi::c_int != OTL_GSUB_TYPE_SINGLE
                    && lookup_table.LookupType as ::core::ffi::c_int != OTL_GSUB_TYPE_ALTERNATE
                    && lookup_table.LookupType as ::core::ffi::c_int != OTL_GSUB_TYPE_LIGATURE
                    && lookup_table.LookupType as ::core::ffi::c_int != OTL_GSUB_TYPE_ESUBST
                {
                    if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
                        pdf_warning!(
                            b"Skipping unsupported GSUB subtable: LookupType=%d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            lookup_table.LookupType as ::core::ffi::c_int,
                        );
                    }
                } else {
                    subtab = renew(
                        subtab as *mut ::core::ffi::c_void,
                        ((num_subtabs as ::core::ffi::c_int
                            + lookup_table.SubTableList.count as ::core::ffi::c_int)
                            as uint32_t as usize)
                            .wrapping_mul(::core::mem::size_of::<otl_gsub_subtab>() as usize)
                            as uint32_t,
                    ) as *mut otl_gsub_subtab;
                    n_st = 0 as ::core::ffi::c_int;
                    st_idx = 0 as ::core::ffi::c_int;
                    while st_idx < lookup_table.SubTableList.count as ::core::ffi::c_int {
                        offset = gsub_offset
                            .wrapping_add(head.LookupList as SFNT_ULONG)
                            .wrapping_add(*lookup_list.value.offset(ll_idx as isize) as SFNT_ULONG)
                            .wrapping_add(*lookup_table.SubTableList.value.offset(st_idx as isize)
                                as SFNT_ULONG);
                        crate::backend::runtime::input_seek(
                            (*sfont).handle,
                            offset as ssize_t,
                            SEEK_SET,
                        );
                        match lookup_table.LookupType as ::core::ffi::c_int {
                            OTL_GSUB_TYPE_SINGLE => {
                                r = otl_gsub_read_single(
                                    subtab
                                        .offset((num_subtabs as ::core::ffi::c_int + n_st) as isize)
                                        as *mut otl_gsub_subtab,
                                    sfont,
                                );
                                if r <= 0 as ::core::ffi::c_int {
                                    pdf_warning!(
                                        b"Reading GSUB subtable (single) failed...\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                } else {
                                    if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
                                        pdf_message!(
                                            b"(single)\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                    n_st += 1;
                                }
                            }
                            OTL_GSUB_TYPE_ALTERNATE => {
                                r = otl_gsub_read_alternate(
                                    subtab
                                        .offset((num_subtabs as ::core::ffi::c_int + n_st) as isize)
                                        as *mut otl_gsub_subtab,
                                    sfont,
                                );
                                if r <= 0 as ::core::ffi::c_int {
                                    pdf_warning!(
                                        b"Reading GSUB subtable (alternate) failed...\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                } else {
                                    if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
                                        pdf_message!(
                                            b"(alternate)\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                    n_st += 1;
                                }
                            }
                            OTL_GSUB_TYPE_LIGATURE => {
                                r = otl_gsub_read_ligature(
                                    subtab
                                        .offset((num_subtabs as ::core::ffi::c_int + n_st) as isize)
                                        as *mut otl_gsub_subtab,
                                    sfont,
                                );
                                if r <= 0 as ::core::ffi::c_int {
                                    pdf_warning!(
                                        b"Reading GSUB subtable (ligature) failed...\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                } else {
                                    if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
                                        pdf_message!(
                                            b"(ligature)\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                    n_st += 1;
                                }
                            }
                            OTL_GSUB_TYPE_ESUBST => {
                                let mut SubstFormat: USHORT = 0;
                                let mut ExtensionLookupType: USHORT = 0;
                                let mut ExtensionOffset: SFNT_ULONG = 0;
                                SubstFormat = tt_get_unsigned_pair((*sfont).handle);
                                if !(SubstFormat as ::core::ffi::c_int != 1 as ::core::ffi::c_int) {
                                    ExtensionLookupType = tt_get_unsigned_pair((*sfont).handle);
                                    ExtensionOffset = tt_get_unsigned_quad((*sfont).handle);
                                    crate::backend::runtime::input_seek(
                                        (*sfont).handle,
                                        offset.wrapping_add(ExtensionOffset) as ssize_t,
                                        SEEK_SET,
                                    );
                                    match ExtensionLookupType as ::core::ffi::c_int {
                                        OTL_GSUB_TYPE_SINGLE => {
                                            r = otl_gsub_read_single(
                                                subtab.offset(
                                                    (num_subtabs as ::core::ffi::c_int + n_st)
                                                        as isize,
                                                )
                                                    as *mut otl_gsub_subtab,
                                                sfont,
                                            );
                                            if r <= 0 as ::core::ffi::c_int {
                                                pdf_warning!(
                                                    b"Reading GSUB subtable (ext:single) failed...\0" as * const
                                                    u8 as * const ::core::ffi::c_char,
                                                );
                                            } else {
                                                if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
                                                    pdf_message!(
                                                        b"(ext:single)\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                }
                                                n_st += 1;
                                            }
                                        }
                                        OTL_GSUB_TYPE_ALTERNATE => {
                                            r = otl_gsub_read_alternate(
                                                subtab.offset(
                                                    (num_subtabs as ::core::ffi::c_int + n_st)
                                                        as isize,
                                                )
                                                    as *mut otl_gsub_subtab,
                                                sfont,
                                            );
                                            if r <= 0 as ::core::ffi::c_int {
                                                pdf_warning!(
                                                    b"Reading GSUB subtable (alternate) failed...\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                            } else {
                                                if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
                                                    pdf_message!(
                                                        b"(alternate)\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                }
                                                n_st += 1;
                                            }
                                        }
                                        OTL_GSUB_TYPE_LIGATURE => {
                                            r = otl_gsub_read_ligature(
                                                subtab.offset(
                                                    (num_subtabs as ::core::ffi::c_int + n_st)
                                                        as isize,
                                                )
                                                    as *mut otl_gsub_subtab,
                                                sfont,
                                            );
                                            if r <= 0 as ::core::ffi::c_int {
                                                pdf_warning!(
                                                    b"Reading GSUB subtable (ext:ligature) failed...\0" as *
                                                    const u8 as * const ::core::ffi::c_char,
                                                );
                                            } else {
                                                if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
                                                    pdf_message!(
                                                        b"(ext:ligature)\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                }
                                                n_st += 1;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                        st_idx += 1;
                    }
                    num_subtabs = (num_subtabs as ::core::ffi::c_int + n_st) as USHORT;
                    clt_release_lookup_table(&raw mut lookup_table);
                }
                i += 1;
            }
            clt_release_feature_table(&raw mut feature_table);
        }
        feat_idx += 1;
    }
    if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
        pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdf_message!(
            b"otl_gsub>> %d subtable(s) read.\n\0" as *const u8 as *const ::core::ffi::c_char,
            num_subtabs as ::core::ffi::c_int,
        );
    }
    clt_release_number_list(&raw mut lookup_list);
    clt_release_record_list(&raw mut feature_list);
    clt_release_record_list(&raw mut script_list);
    otl_release_opt(script);
    otl_release_opt(language);
    otl_release_opt(feature);
    if !subtab.is_null() {
        (*gsub).num_subtables = num_subtabs as ::core::ffi::c_int;
        (*gsub).subtables = subtab;
    } else {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn otl_gsub_apply_single(
    mut subtab: *mut otl_gsub_subtab,
    mut gid: *mut USHORT,
) -> ::core::ffi::c_int {
    let mut idx: ::core::ffi::c_int = 0;
    if !(!subtab.is_null() && !gid.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"subtab && gid\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*subtab).SubstFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        let mut data: *mut otl_gsub_single1 = ::core::ptr::null_mut::<otl_gsub_single1>();
        data = (*subtab).table.single1;
        idx = clt_lookup_coverage(&raw mut (*data).coverage, *gid);
        if idx >= 0 as ::core::ffi::c_int {
            *gid =
                (*gid as ::core::ffi::c_int + (*data).DeltaGlyphID as ::core::ffi::c_int) as USHORT;
            return 0 as ::core::ffi::c_int;
        }
    } else if (*subtab).SubstFormat as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        let mut data_0: *mut otl_gsub_single2 = ::core::ptr::null_mut::<otl_gsub_single2>();
        data_0 = (*subtab).table.single2;
        idx = clt_lookup_coverage(&raw mut (*data_0).coverage, *gid);
        if idx >= 0 as ::core::ffi::c_int && idx < (*data_0).GlyphCount as ::core::ffi::c_int {
            *gid = *(*data_0).Substitute.offset(idx as isize) as USHORT;
            return 0 as ::core::ffi::c_int;
        }
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn otl_gsub_apply_alternate(
    mut subtab: *mut otl_gsub_subtab,
    mut alt_idx: USHORT,
    mut gid: *mut USHORT,
) -> ::core::ffi::c_int {
    let mut idx: ::core::ffi::c_int = 0;
    if !(!subtab.is_null() && !gid.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"subtab && gid\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*subtab).SubstFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        let mut data: *mut otl_gsub_alternate1 = ::core::ptr::null_mut::<otl_gsub_alternate1>();
        data = (*subtab).table.alternate1;
        idx = clt_lookup_coverage(&raw mut (*data).coverage, *gid);
        if idx < 0 as ::core::ffi::c_int || idx >= (*data).AlternateSetCount as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        } else {
            let mut altset: *mut otl_gsub_altset = ::core::ptr::null_mut::<otl_gsub_altset>();
            altset = (*data).AlternateSet.offset(idx as isize) as *mut otl_gsub_altset;
            if alt_idx as ::core::ffi::c_int >= (*altset).GlyphCount as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            } else {
                *gid = *(*altset).Alternate.offset(alt_idx as isize) as USHORT;
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn glyph_seq_cmp(
    mut glyph_seq0: *mut GlyphID,
    mut n_glyphs0: USHORT,
    mut glyph_seq1: *mut GlyphID,
    mut n_glyphs1: USHORT,
) -> ::core::ffi::c_int {
    let mut i: USHORT = 0;
    if n_glyphs0 as ::core::ffi::c_int != n_glyphs1 as ::core::ffi::c_int {
        return n_glyphs0 as ::core::ffi::c_int - n_glyphs1 as ::core::ffi::c_int;
    }
    i = 0 as USHORT;
    while (i as ::core::ffi::c_int) < n_glyphs0 as ::core::ffi::c_int {
        if *glyph_seq0.offset(i as isize) as ::core::ffi::c_int
            != *glyph_seq1.offset(i as isize) as ::core::ffi::c_int
        {
            return *glyph_seq0.offset(i as isize) as ::core::ffi::c_int
                - *glyph_seq1.offset(i as isize) as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn otl_gsub_apply_ligature(
    mut subtab: *mut otl_gsub_subtab,
    mut gid_in: *mut USHORT,
    mut num_gids: USHORT,
    mut gid_out: *mut USHORT,
) -> ::core::ffi::c_int {
    let mut idx: ::core::ffi::c_int = 0;
    if !(!subtab.is_null() && !gid_out.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"subtab && gid_out\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if gid_in.is_null() || (num_gids as ::core::ffi::c_int) < 1 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if (*subtab).SubstFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        let mut data: *mut otl_gsub_ligature1 = ::core::ptr::null_mut::<otl_gsub_ligature1>();
        data = (*subtab).table.ligature1;
        idx = clt_lookup_coverage(
            &raw mut (*data).coverage,
            *gid_in.offset(0 as ::core::ffi::c_int as isize),
        );
        if idx >= 0 as ::core::ffi::c_int && idx < (*data).LigSetCount as ::core::ffi::c_int {
            let mut ligset: *mut otl_gsub_ligset = ::core::ptr::null_mut::<otl_gsub_ligset>();
            let mut j: USHORT = 0;
            ligset = (*data).LigatureSet.offset(idx as isize) as *mut otl_gsub_ligset;
            j = 0 as USHORT;
            while (j as ::core::ffi::c_int) < (*ligset).LigatureCount as ::core::ffi::c_int {
                if glyph_seq_cmp(
                    gid_in.offset(1 as ::core::ffi::c_int as isize) as *mut GlyphID,
                    (num_gids as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as USHORT,
                    (*(*ligset).Ligature.offset(j as isize)).Component,
                    ((*(*ligset).Ligature.offset(j as isize)).CompCount as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as USHORT,
                ) == 0
                {
                    *gid_out = (*(*ligset).Ligature.offset(j as isize)).LigGlyph as USHORT;
                    return 0 as ::core::ffi::c_int;
                }
                j = j.wrapping_add(1);
            }
        }
    }
    return -(1 as ::core::ffi::c_int);
}
pub const GSUB_LIST_MAX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_otl_gsub_new")]
pub unsafe extern "C-unwind" fn otl_gsub_new() -> *mut otl_gsub {
    let mut gsub_list: *mut otl_gsub = ::core::ptr::null_mut::<otl_gsub>();
    gsub_list = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<otl_gsub>() as usize) as uint32_t)
        as *mut otl_gsub;
    (*gsub_list).num_gsubs = 0 as ::core::ffi::c_int;
    (*gsub_list).select = -(1 as ::core::ffi::c_int);
    (*gsub_list).first = ::core::ptr::null_mut::<gsub_entry>();
    return gsub_list as *mut otl_gsub;
}
unsafe extern "C-unwind" fn clear_chain(mut gsub_list: *mut otl_gsub) {
    let mut entry: *mut gsub_entry = ::core::ptr::null_mut::<gsub_entry>();
    let mut next: *mut gsub_entry = ::core::ptr::null_mut::<gsub_entry>();
    entry = (*gsub_list).first;
    while !entry.is_null() {
        next = (*entry).next;
        free(entry as *mut ::core::ffi::c_void);
        entry = next;
    }
    (*gsub_list).first = ::core::ptr::null_mut::<gsub_entry>();
}
#[unsafe(export_name = "__pdf_impl_otl_gsub_add_feat")]
pub unsafe extern "C-unwind" fn otl_gsub_add_feat(
    mut gsub_list: *mut otl_gsub,
    mut script: *const ::core::ffi::c_char,
    mut language: *const ::core::ffi::c_char,
    mut feature: *const ::core::ffi::c_char,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0;
    let mut gsub: *mut otl_gsub_tab = ::core::ptr::null_mut::<otl_gsub_tab>();
    if (*gsub_list).num_gsubs > GSUB_LIST_MAX {
        pdf_abort!(b"Too many GSUB features...\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*gsub_list).num_gsubs {
        gsub = (&raw mut (*gsub_list).gsubs as *mut otl_gsub_tab).offset(i as isize)
            as *mut otl_gsub_tab;
        if streq_ptr(script, (*gsub).script) as ::core::ffi::c_int != 0
            && streq_ptr(language, (*gsub).language) as ::core::ffi::c_int != 0
            && streq_ptr(feature, (*gsub).feature) as ::core::ffi::c_int != 0
        {
            (*gsub_list).select = i;
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    gsub =
        (&raw mut (*gsub_list).gsubs as *mut otl_gsub_tab).offset(i as isize) as *mut otl_gsub_tab;
    (*gsub).script = new(
        (strlen(script).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*gsub).script, script);
    (*gsub).language = new(
        (strlen(language).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*gsub).language, language);
    (*gsub).feature = new(
        (strlen(feature).wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy((*gsub).feature, feature);
    if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
        pdf_message!(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdf_message!(
            b"otl_gsub>> Reading \"%s.%s.%s\"...\n\0" as *const u8 as *const ::core::ffi::c_char,
            script,
            language,
            feature,
        );
    }
    retval = otl_gsub_read_feat(gsub, sfont);
    if retval >= 0 as ::core::ffi::c_int {
        (*gsub_list).select = i;
        (*gsub_list).num_gsubs += 1;
    } else {
        if dpx_conf.verbose_level > VERBOSE_LEVEL_MIN {
            pdf_message!(b"otl_gsub>> Failed\n\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        free((*gsub).script as *mut ::core::ffi::c_void);
        free((*gsub).language as *mut ::core::ffi::c_void);
        free((*gsub).feature as *mut ::core::ffi::c_void);
    }
    return retval;
}
unsafe extern "C-unwind" fn scan_otl_tag(
    mut otl_tags: *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut script: *mut ::core::ffi::c_char,
    mut language: *mut ::core::ffi::c_char,
    mut feature: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut period: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !(!script.is_null() && !language.is_null() && !feature.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"script && language && feature\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if otl_tags.is_null() || otl_tags >= endptr {
        return -(1 as ::core::ffi::c_int);
    }
    memset(script as *mut ::core::ffi::c_void, ' ' as i32, 4 as size_t);
    *script.offset(4 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    memset(
        language as *mut ::core::ffi::c_void,
        ' ' as i32,
        4 as size_t,
    );
    *language.offset(4 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    memset(feature as *mut ::core::ffi::c_void, ' ' as i32, 4 as size_t);
    *feature.offset(4 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
    p = otl_tags;
    period = strchr(p, '.' as i32);
    if !period.is_null() && period < endptr {
        if period < p.offset(5 as ::core::ffi::c_int as isize) {
            strncpy(
                script,
                p,
                period.offset_from(p) as ::core::ffi::c_long as size_t,
            );
        } else {
            pdf_warning!(
                b"Invalid OTL script tag found: %s\0" as *const u8 as *const ::core::ffi::c_char,
                p,
            );
            return -(1 as ::core::ffi::c_int);
        }
        p = period.offset(1 as ::core::ffi::c_int as isize);
        period = strchr(p, '.' as i32);
        if !period.is_null() && period < endptr {
            if period < p.offset(5 as ::core::ffi::c_int as isize) {
                strncpy(
                    language,
                    p,
                    period.offset_from(p) as ::core::ffi::c_long as size_t,
                );
            } else {
                pdf_warning!(
                    b"Invalid OTL lanuage tag found: %s\0" as *const u8
                        as *const ::core::ffi::c_char,
                    p,
                );
                return -(1 as ::core::ffi::c_int);
            }
            p = period.offset(1 as ::core::ffi::c_int as isize);
        }
    } else {
        strcpy(script, b"*\0" as *const u8 as *const ::core::ffi::c_char);
        strcpy(language, b"*\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if p.offset(4 as ::core::ffi::c_int as isize) <= endptr {
        strncpy(
            feature,
            p,
            endptr.offset_from(p) as ::core::ffi::c_long as size_t,
        );
        p = endptr;
    } else {
        pdf_warning!(
            b"No valid OTL feature tag specified.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_otl_gsub_release")]
pub unsafe extern "C-unwind" fn otl_gsub_release(mut gsub_list: *mut otl_gsub) {
    let mut gsub: *mut otl_gsub_tab = ::core::ptr::null_mut::<otl_gsub_tab>();
    let mut subtab: *mut otl_gsub_subtab = ::core::ptr::null_mut::<otl_gsub_subtab>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if gsub_list.is_null() {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*gsub_list).num_gsubs {
        gsub = (&raw mut (*gsub_list).gsubs as *mut otl_gsub_tab).offset(i as isize)
            as *mut otl_gsub_tab;
        free((*gsub).script as *mut ::core::ffi::c_void);
        free((*gsub).language as *mut ::core::ffi::c_void);
        free((*gsub).feature as *mut ::core::ffi::c_void);
        j = 0 as ::core::ffi::c_int;
        while j < (*gsub).num_subtables {
            subtab = (*gsub).subtables.offset(j as isize) as *mut otl_gsub_subtab;
            match (*subtab).LookupType as ::core::ffi::c_int {
                OTL_GSUB_TYPE_SINGLE => {
                    otl_gsub_release_single(subtab);
                }
                OTL_GSUB_TYPE_ALTERNATE => {
                    otl_gsub_release_alternate(subtab);
                }
                OTL_GSUB_TYPE_LIGATURE => {
                    otl_gsub_release_ligature(subtab);
                }
                _ => {
                    pdf_abort!(b"???\0" as *const u8 as *const ::core::ffi::c_char);
                }
            }
            j += 1;
        }
        free((*gsub).subtables as *mut ::core::ffi::c_void);
        i += 1;
    }
    clear_chain(gsub_list);
    free(gsub_list as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_otl_gsub_apply")]
pub unsafe extern "C-unwind" fn otl_gsub_apply(
    mut gsub_list: *mut otl_gsub,
    mut gid: *mut USHORT,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut gsub: *mut otl_gsub_tab = ::core::ptr::null_mut::<otl_gsub_tab>();
    let mut subtab: *mut otl_gsub_subtab = ::core::ptr::null_mut::<otl_gsub_subtab>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if gsub_list.is_null() || gid.is_null() {
        return retval;
    }
    i = (*gsub_list).select;
    if i < 0 as ::core::ffi::c_int || i >= (*gsub_list).num_gsubs {
        pdf_abort!(b"GSUB not selected...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    gsub =
        (&raw mut (*gsub_list).gsubs as *mut otl_gsub_tab).offset(i as isize) as *mut otl_gsub_tab;
    j = 0 as ::core::ffi::c_int;
    while retval < 0 as ::core::ffi::c_int && j < (*gsub).num_subtables {
        subtab = (*gsub).subtables.offset(j as isize) as *mut otl_gsub_subtab;
        match (*subtab).LookupType as ::core::ffi::c_int {
            OTL_GSUB_TYPE_SINGLE => {
                retval = otl_gsub_apply_single(subtab, gid);
            }
            _ => {}
        }
        j += 1;
    }
    return retval;
}
#[unsafe(export_name = "__pdf_impl_otl_gsub_apply_alt")]
pub unsafe extern "C-unwind" fn otl_gsub_apply_alt(
    mut gsub_list: *mut otl_gsub,
    mut alt_idx: USHORT,
    mut gid: *mut USHORT,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut gsub: *mut otl_gsub_tab = ::core::ptr::null_mut::<otl_gsub_tab>();
    let mut subtab: *mut otl_gsub_subtab = ::core::ptr::null_mut::<otl_gsub_subtab>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if gsub_list.is_null() || gid.is_null() {
        return retval;
    }
    i = (*gsub_list).select;
    if i < 0 as ::core::ffi::c_int || i >= (*gsub_list).num_gsubs {
        pdf_abort!(b"GSUB not selected...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    gsub =
        (&raw mut (*gsub_list).gsubs as *mut otl_gsub_tab).offset(i as isize) as *mut otl_gsub_tab;
    j = 0 as ::core::ffi::c_int;
    while retval < 0 as ::core::ffi::c_int && j < (*gsub).num_subtables {
        subtab = (*gsub).subtables.offset(j as isize) as *mut otl_gsub_subtab;
        match (*subtab).LookupType as ::core::ffi::c_int {
            OTL_GSUB_TYPE_ALTERNATE => {
                retval = otl_gsub_apply_alternate(subtab, alt_idx, gid);
            }
            _ => {}
        }
        j += 1;
    }
    return retval;
}
#[unsafe(export_name = "__pdf_impl_otl_gsub_apply_lig")]
pub unsafe extern "C-unwind" fn otl_gsub_apply_lig(
    mut gsub_list: *mut otl_gsub,
    mut gid_in: *mut USHORT,
    mut num_gids: USHORT,
    mut gid_out: *mut USHORT,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut gsub: *mut otl_gsub_tab = ::core::ptr::null_mut::<otl_gsub_tab>();
    let mut subtab: *mut otl_gsub_subtab = ::core::ptr::null_mut::<otl_gsub_subtab>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    if gsub_list.is_null() || gid_out.is_null() {
        return retval;
    }
    i = (*gsub_list).select;
    if i < 0 as ::core::ffi::c_int || i >= (*gsub_list).num_gsubs {
        pdf_abort!(b"GSUB not selected...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    gsub =
        (&raw mut (*gsub_list).gsubs as *mut otl_gsub_tab).offset(i as isize) as *mut otl_gsub_tab;
    j = 0 as ::core::ffi::c_int;
    while retval < 0 as ::core::ffi::c_int && j < (*gsub).num_subtables {
        subtab = (*gsub).subtables.offset(j as isize) as *mut otl_gsub_subtab;
        match (*subtab).LookupType as ::core::ffi::c_int {
            OTL_GSUB_TYPE_LIGATURE => {
                retval = otl_gsub_apply_ligature(subtab, gid_in, num_gids, gid_out);
            }
            _ => {}
        }
        j += 1;
    }
    return retval;
}
unsafe extern "C-unwind" fn gsub_find(
    mut gsub_list: *mut otl_gsub,
    mut script: *const ::core::ffi::c_char,
    mut language: *const ::core::ffi::c_char,
    mut feature: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut gsub: *mut otl_gsub_tab = ::core::ptr::null_mut::<otl_gsub_tab>();
    i = 0 as ::core::ffi::c_int;
    while i < (*gsub_list).num_gsubs {
        gsub = (&raw mut (*gsub_list).gsubs as *mut otl_gsub_tab).offset(i as isize)
            as *mut otl_gsub_tab;
        if streq_ptr((*gsub).script, script) as ::core::ffi::c_int != 0
            && streq_ptr((*gsub).language, language) as ::core::ffi::c_int != 0
            && streq_ptr((*gsub).feature, feature) as ::core::ffi::c_int != 0
        {
            return i;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_otl_gsub_select")]
pub unsafe extern "C-unwind" fn otl_gsub_select(
    mut gsub_list: *mut otl_gsub,
    mut script: *const ::core::ffi::c_char,
    mut language: *const ::core::ffi::c_char,
    mut feature: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    (*gsub_list).select = gsub_find(gsub_list, script, language, feature);
    return (*gsub_list).select;
}
#[unsafe(export_name = "__pdf_impl_otl_gsub_set_chain")]
pub unsafe extern "C-unwind" fn otl_gsub_set_chain(
    mut gsub_list: *mut otl_gsub,
    mut otl_tags: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut prev: *mut gsub_entry = ::core::ptr::null_mut::<gsub_entry>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nextptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut script: [::core::ffi::c_char; 5] = [0; 5];
    let mut language: [::core::ffi::c_char; 5] = [0; 5];
    let mut feature: [::core::ffi::c_char; 5] = [0; 5];
    let mut idx: ::core::ffi::c_int = 0;
    clear_chain(gsub_list);
    endptr = otl_tags.offset(strlen(otl_tags) as isize);
    p = otl_tags;
    while p < endptr {
        nextptr = strchr(p, ':' as i32);
        if nextptr.is_null() {
            nextptr = endptr;
        }
        if scan_otl_tag(
            p,
            nextptr,
            &raw mut script as *mut ::core::ffi::c_char,
            &raw mut language as *mut ::core::ffi::c_char,
            &raw mut feature as *mut ::core::ffi::c_char,
        ) >= 0 as ::core::ffi::c_int
        {
            idx = gsub_find(
                gsub_list,
                &raw mut script as *mut ::core::ffi::c_char,
                &raw mut language as *mut ::core::ffi::c_char,
                &raw mut feature as *mut ::core::ffi::c_char,
            );
            if idx >= 0 as ::core::ffi::c_int && idx <= (*gsub_list).num_gsubs {
                let mut entry: *mut gsub_entry = ::core::ptr::null_mut::<gsub_entry>();
                entry = new((1 as ::core::ffi::c_int as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<gsub_entry>() as usize)
                    as uint32_t) as *mut gsub_entry;
                if (*gsub_list).first.is_null() {
                    (*gsub_list).first = entry;
                }
                if !prev.is_null() {
                    (*prev).next = entry;
                }
                (*entry).index = idx;
                prev = entry;
            }
        }
        nextptr = nextptr.offset(1);
        p = nextptr;
    }
    if !prev.is_null() {
        (*prev).next = ::core::ptr::null_mut::<gsub_entry>();
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_otl_gsub_add_feat_list")]
pub unsafe extern "C-unwind" fn otl_gsub_add_feat_list(
    mut gsub_list: *mut otl_gsub,
    mut otl_tags: *const ::core::ffi::c_char,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut nextptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut script: [::core::ffi::c_char; 5] = [0; 5];
    let mut language: [::core::ffi::c_char; 5] = [0; 5];
    let mut feature: [::core::ffi::c_char; 5] = [0; 5];
    let mut idx: ::core::ffi::c_int = 0;
    if gsub_list.is_null() || otl_tags.is_null() || sfont.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    clear_chain(gsub_list);
    endptr = otl_tags.offset(strlen(otl_tags) as isize);
    p = otl_tags;
    while p < endptr {
        nextptr = strchr(p, ':' as i32);
        if nextptr.is_null() {
            nextptr = endptr;
        }
        if scan_otl_tag(
            p,
            nextptr,
            &raw mut script as *mut ::core::ffi::c_char,
            &raw mut language as *mut ::core::ffi::c_char,
            &raw mut feature as *mut ::core::ffi::c_char,
        ) >= 0 as ::core::ffi::c_int
        {
            idx = gsub_find(
                gsub_list,
                &raw mut script as *mut ::core::ffi::c_char,
                &raw mut language as *mut ::core::ffi::c_char,
                &raw mut feature as *mut ::core::ffi::c_char,
            );
            if idx < 0 as ::core::ffi::c_int {
                otl_gsub_add_feat(
                    gsub_list,
                    &raw mut script as *mut ::core::ffi::c_char,
                    &raw mut language as *mut ::core::ffi::c_char,
                    &raw mut feature as *mut ::core::ffi::c_char,
                    sfont,
                );
            }
        }
        nextptr = nextptr.offset(1);
        p = nextptr;
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_otl_gsub_apply_chain")]
pub unsafe extern "C-unwind" fn otl_gsub_apply_chain(
    mut gsub_list: *mut otl_gsub,
    mut gid: *mut USHORT,
) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut gsub: *mut otl_gsub_tab = ::core::ptr::null_mut::<otl_gsub_tab>();
    let mut subtab: *mut otl_gsub_subtab = ::core::ptr::null_mut::<otl_gsub_subtab>();
    let mut entry: *mut gsub_entry = ::core::ptr::null_mut::<gsub_entry>();
    let mut i: ::core::ffi::c_int = 0;
    let mut idx: ::core::ffi::c_int = 0;
    if gsub_list.is_null() || gid.is_null() {
        return retval;
    }
    entry = (*gsub_list).first;
    while !entry.is_null() {
        idx = (*entry).index;
        if !(idx < 0 as ::core::ffi::c_int || idx >= (*gsub_list).num_gsubs) {
            gsub = (&raw mut (*gsub_list).gsubs as *mut otl_gsub_tab).offset(idx as isize)
                as *mut otl_gsub_tab;
            i = 0 as ::core::ffi::c_int;
            retval = -(1 as ::core::ffi::c_int);
            while retval < 0 as ::core::ffi::c_int && i < (*gsub).num_subtables {
                subtab = (*gsub).subtables.offset(i as isize) as *mut otl_gsub_subtab;
                match (*subtab).LookupType as ::core::ffi::c_int {
                    OTL_GSUB_TYPE_SINGLE => {
                        retval = otl_gsub_apply_single(subtab, gid);
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        entry = (*entry).next;
    }
    return retval;
}
unsafe extern "C-unwind" fn add_glyph_if_valid(
    mut cmap: *mut CMap,
    mut used_chars: *mut ::core::ffi::c_char,
    mut map_base: *mut int32_t,
    mut map_sub: *mut int32_t,
    mut num_glyphs: USHORT,
    mut GIDToCIDMap: *mut uint16_t,
    mut gid: USHORT,
    mut gid_sub: USHORT,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut src: [::core::ffi::c_uchar; 2] = [0; 2];
    let mut dst: [::core::ffi::c_uchar; 4] = [0; 4];
    let mut p: *mut ::core::ffi::c_uchar = &raw mut dst as *mut ::core::ffi::c_uchar;
    let mut endptr: *mut ::core::ffi::c_uchar =
        (&raw mut dst as *mut ::core::ffi::c_uchar).offset(4 as ::core::ffi::c_int as isize);
    let mut len: size_t = 0;
    let mut cid_sub: uint16_t = 0;
    if gid_sub as ::core::ffi::c_int >= num_glyphs as ::core::ffi::c_int
        || gid as ::core::ffi::c_int >= num_glyphs as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    cid_sub = *GIDToCIDMap.offset(gid_sub as isize);
    if *used_chars.offset((cid_sub as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int)
            << 7 as ::core::ffi::c_int - cid_sub as ::core::ffi::c_int % 8 as ::core::ffi::c_int
        != 0
    {
        let mut ch: int32_t = *map_base.offset(gid as isize);
        if UC_is_valid(ch) {
            src[0 as ::core::ffi::c_int as usize] =
                (cid_sub as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            src[1 as ::core::ffi::c_int as usize] = (cid_sub as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            len = UC_UTF16BE_encode_char(ch, &raw mut p, endptr);
            CMap_add_bfchar(
                cmap,
                &raw mut src as *mut ::core::ffi::c_uchar,
                2 as size_t,
                &raw mut dst as *mut ::core::ffi::c_uchar,
                len,
            );
            let ref mut fresh4 = *used_chars
                .offset((cid_sub as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
            *fresh4 = (*fresh4 as ::core::ffi::c_int
                & !((1 as ::core::ffi::c_int)
                    << 7 as ::core::ffi::c_int
                        - cid_sub as ::core::ffi::c_int % 8 as ::core::ffi::c_int))
                as ::core::ffi::c_char;
            count = 1 as ::core::ffi::c_int;
        } else {
            ch = *map_sub.offset(gid as isize);
            if UC_is_valid(ch) {
                src[0 as ::core::ffi::c_int as usize] =
                    (cid_sub as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                        & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                src[1 as ::core::ffi::c_int as usize] = (cid_sub as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int)
                    as ::core::ffi::c_uchar;
                len = UC_UTF16BE_encode_char(ch, &raw mut p, endptr);
                CMap_add_bfchar(
                    cmap,
                    &raw mut src as *mut ::core::ffi::c_uchar,
                    2 as size_t,
                    &raw mut dst as *mut ::core::ffi::c_uchar,
                    len,
                );
                let ref mut fresh5 = *used_chars
                    .offset((cid_sub as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
                *fresh5 = (*fresh5 as ::core::ffi::c_int
                    & !((1 as ::core::ffi::c_int)
                        << 7 as ::core::ffi::c_int
                            - cid_sub as ::core::ffi::c_int % 8 as ::core::ffi::c_int))
                    as ::core::ffi::c_char;
                count = 1 as ::core::ffi::c_int;
            }
        }
    }
    return count;
}
unsafe extern "C-unwind" fn add_ToUnicode_single(
    mut cmap: *mut CMap,
    mut used_chars: *mut ::core::ffi::c_char,
    mut subtab: *mut otl_gsub_subtab,
    mut map_base: *mut int32_t,
    mut map_sub: *mut int32_t,
    mut num_glyphs: USHORT,
    mut GIDToCIDMap: *mut uint16_t,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut idx: ::core::ffi::c_int = 0;
    let mut i: USHORT = 0;
    let mut gid: USHORT = 0;
    let mut gid_sub: USHORT = 0;
    if subtab.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"subtab\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*subtab).SubstFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        let mut data: *mut otl_gsub_single1 = ::core::ptr::null_mut::<otl_gsub_single1>();
        let mut cov: *mut clt_coverage = ::core::ptr::null_mut::<clt_coverage>();
        data = (*subtab).table.single1;
        cov = &raw mut (*data).coverage;
        match (*cov).format as ::core::ffi::c_int {
            1 => {
                idx = 0 as ::core::ffi::c_int;
                while idx < (*cov).count as ::core::ffi::c_int {
                    gid = *(*cov).list.offset(idx as isize) as USHORT;
                    gid_sub = (gid as ::core::ffi::c_int
                        + (*data).DeltaGlyphID as ::core::ffi::c_int)
                        as USHORT;
                    count += add_glyph_if_valid(
                        cmap,
                        used_chars,
                        map_base,
                        map_sub,
                        num_glyphs,
                        GIDToCIDMap,
                        gid,
                        gid_sub,
                    );
                    idx += 1;
                }
            }
            2 => {
                i = 0 as USHORT;
                while (i as ::core::ffi::c_int) < (*cov).count as ::core::ffi::c_int {
                    gid = (*(*cov).range.offset(i as isize)).Start as USHORT;
                    while gid as ::core::ffi::c_int
                        <= (*(*cov).range.offset(i as isize)).End as ::core::ffi::c_int
                        && (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int
                    {
                        idx = (*(*cov).range.offset(i as isize)).StartCoverageIndex
                            as ::core::ffi::c_int
                            + gid as ::core::ffi::c_int
                            - (*(*cov).range.offset(i as isize)).Start as ::core::ffi::c_int;
                        gid_sub = (gid as ::core::ffi::c_int
                            + (*data).DeltaGlyphID as ::core::ffi::c_int)
                            as USHORT;
                        count += add_glyph_if_valid(
                            cmap,
                            used_chars,
                            map_base,
                            map_sub,
                            num_glyphs,
                            GIDToCIDMap,
                            gid,
                            gid_sub,
                        );
                        gid = gid.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
            }
            _ => {}
        }
    } else if (*subtab).SubstFormat as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
        let mut data_0: *mut otl_gsub_single2 = ::core::ptr::null_mut::<otl_gsub_single2>();
        let mut cov_0: *mut clt_coverage = ::core::ptr::null_mut::<clt_coverage>();
        data_0 = (*subtab).table.single2;
        cov_0 = &raw mut (*data_0).coverage;
        match (*cov_0).format as ::core::ffi::c_int {
            1 => {
                idx = 0 as ::core::ffi::c_int;
                while idx < (*cov_0).count as ::core::ffi::c_int {
                    gid = *(*cov_0).list.offset(idx as isize) as USHORT;
                    if idx >= 0 as ::core::ffi::c_int
                        && idx < (*data_0).GlyphCount as ::core::ffi::c_int
                    {
                        gid_sub = *(*data_0).Substitute.offset(idx as isize) as USHORT;
                        count += add_glyph_if_valid(
                            cmap,
                            used_chars,
                            map_base,
                            map_sub,
                            num_glyphs,
                            GIDToCIDMap,
                            gid,
                            gid_sub,
                        );
                    }
                    idx += 1;
                }
            }
            2 => {
                i = 0 as USHORT;
                while (i as ::core::ffi::c_int) < (*cov_0).count as ::core::ffi::c_int {
                    gid = (*(*cov_0).range.offset(i as isize)).Start as USHORT;
                    while gid as ::core::ffi::c_int
                        <= (*(*cov_0).range.offset(i as isize)).End as ::core::ffi::c_int
                        && (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int
                    {
                        idx = (*(*cov_0).range.offset(i as isize)).StartCoverageIndex
                            as ::core::ffi::c_int
                            + gid as ::core::ffi::c_int
                            - (*(*cov_0).range.offset(i as isize)).Start as ::core::ffi::c_int;
                        if idx >= 0 as ::core::ffi::c_int
                            && idx < (*data_0).GlyphCount as ::core::ffi::c_int
                        {
                            gid_sub = *(*data_0).Substitute.offset(idx as isize) as USHORT;
                            count += add_glyph_if_valid(
                                cmap,
                                used_chars,
                                map_base,
                                map_sub,
                                num_glyphs,
                                GIDToCIDMap,
                                gid,
                                gid_sub,
                            );
                        }
                        gid = gid.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
            }
            _ => {}
        }
    }
    return count;
}
unsafe extern "C-unwind" fn add_alternate1_inverse_map(
    mut cmap: *mut CMap,
    mut used_chars: *mut ::core::ffi::c_char,
    mut map_base: *mut int32_t,
    mut map_sub: *mut int32_t,
    mut num_glyphs: USHORT,
    mut GIDToCIDMap: *mut uint16_t,
    mut gid: USHORT,
    mut idx: ::core::ffi::c_int,
    mut data: *mut otl_gsub_alternate1,
) -> int32_t {
    let mut count: int32_t = 0 as int32_t;
    if idx >= 0 as ::core::ffi::c_int && idx < (*data).AlternateSetCount as ::core::ffi::c_int {
        let mut altset: *mut otl_gsub_altset = ::core::ptr::null_mut::<otl_gsub_altset>();
        let mut i: USHORT = 0;
        altset = (*data).AlternateSet.offset(idx as isize) as *mut otl_gsub_altset;
        if (*altset).GlyphCount as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return count;
        }
        i = 0 as USHORT;
        while (i as ::core::ffi::c_int) < (*altset).GlyphCount as ::core::ffi::c_int {
            let mut gid_alt: USHORT = *(*altset).Alternate.offset(i as isize) as USHORT;
            count = (count as ::core::ffi::c_int
                + add_glyph_if_valid(
                    cmap,
                    used_chars,
                    map_base,
                    map_sub,
                    num_glyphs,
                    GIDToCIDMap,
                    gid,
                    gid_alt,
                )) as int32_t;
            i = i.wrapping_add(1);
        }
    }
    return count;
}
unsafe extern "C-unwind" fn add_ToUnicode_alternate(
    mut cmap: *mut CMap,
    mut used_chars: *mut ::core::ffi::c_char,
    mut subtab: *mut otl_gsub_subtab,
    mut map_base: *mut int32_t,
    mut map_sub: *mut int32_t,
    mut num_glyphs: USHORT,
    mut GIDToCIDMap: *mut uint16_t,
) -> int32_t {
    let mut count: int32_t = 0 as int32_t;
    let mut i: USHORT = 0;
    let mut gid: USHORT = 0;
    let mut idx: USHORT = 0;
    if subtab.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"subtab\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*subtab).SubstFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        let mut data: *mut otl_gsub_alternate1 = ::core::ptr::null_mut::<otl_gsub_alternate1>();
        let mut cov: *mut clt_coverage = ::core::ptr::null_mut::<clt_coverage>();
        data = (*subtab).table.alternate1;
        cov = &raw mut (*data).coverage;
        match (*cov).format as ::core::ffi::c_int {
            1 => {
                idx = 0 as USHORT;
                while (idx as ::core::ffi::c_int) < (*cov).count as ::core::ffi::c_int {
                    gid = *(*cov).list.offset(idx as isize) as USHORT;
                    if (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
                        count += add_alternate1_inverse_map(
                            cmap,
                            used_chars,
                            map_base,
                            map_sub,
                            num_glyphs,
                            GIDToCIDMap,
                            gid,
                            idx as ::core::ffi::c_int,
                            data,
                        );
                    }
                    idx = idx.wrapping_add(1);
                }
            }
            2 => {
                i = 0 as USHORT;
                while (i as ::core::ffi::c_int) < (*cov).count as ::core::ffi::c_int {
                    gid = (*(*cov).range.offset(i as isize)).Start as USHORT;
                    while gid as ::core::ffi::c_int
                        <= (*(*cov).range.offset(i as isize)).End as ::core::ffi::c_int
                        && (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int
                    {
                        idx = ((*(*cov).range.offset(i as isize)).StartCoverageIndex
                            as ::core::ffi::c_int
                            + gid as ::core::ffi::c_int
                            - (*(*cov).range.offset(i as isize)).Start as ::core::ffi::c_int)
                            as USHORT;
                        count += add_alternate1_inverse_map(
                            cmap,
                            used_chars,
                            map_base,
                            map_sub,
                            num_glyphs,
                            GIDToCIDMap,
                            gid,
                            idx as ::core::ffi::c_int,
                            data,
                        );
                        gid = gid.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
            }
            _ => {}
        }
    }
    return count;
}
unsafe extern "C-unwind" fn add_ligature1_inverse_map(
    mut cmap: *mut CMap,
    mut used_chars: *mut ::core::ffi::c_char,
    mut map_base: *mut int32_t,
    mut map_sub: *mut int32_t,
    mut num_glyphs: USHORT,
    mut GIDToCIDMap: *mut uint16_t,
    mut gid_1: USHORT,
    mut idx: ::core::ffi::c_int,
    mut data: *mut otl_gsub_ligature1,
) -> int32_t {
    let mut count: int32_t = 0 as int32_t;
    if idx >= 0 as ::core::ffi::c_int && idx < (*data).LigSetCount as ::core::ffi::c_int {
        let mut ligset: *mut otl_gsub_ligset = ::core::ptr::null_mut::<otl_gsub_ligset>();
        let mut i: USHORT = 0;
        let mut j: USHORT = 0;
        ligset = (*data).LigatureSet.offset(idx as isize) as *mut otl_gsub_ligset;
        j = 0 as USHORT;
        while (j as ::core::ffi::c_int) < (*ligset).LigatureCount as ::core::ffi::c_int {
            let mut gid_sub: USHORT = (*(*ligset).Ligature.offset(j as isize)).LigGlyph as USHORT;
            if (gid_sub as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
                let mut cid: uint16_t = *GIDToCIDMap.offset(gid_sub as isize);
                if *used_chars
                    .offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int)
                        << 7 as ::core::ffi::c_int
                            - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int
                    != 0
                {
                    let mut ch: int32_t = 0;
                    let mut ucv: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
                    let mut comp_count: USHORT = (*(*ligset).Ligature.offset(j as isize)).CompCount;
                    let mut fail_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    ucv = new((comp_count as uint32_t as usize)
                        .wrapping_mul(::core::mem::size_of::<int32_t>() as usize)
                        as uint32_t) as *mut int32_t;
                    ch = if UC_is_valid(*map_base.offset(gid_1 as isize)) as ::core::ffi::c_int != 0
                    {
                        *map_base.offset(gid_1 as isize)
                    } else {
                        *map_sub.offset(gid_1 as isize)
                    };
                    *ucv.offset(0 as ::core::ffi::c_int as isize) = ch;
                    fail_count += if UC_is_valid(ch) as ::core::ffi::c_int != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    };
                    i = 0 as USHORT;
                    while (i as ::core::ffi::c_int)
                        < (*(*ligset).Ligature.offset(j as isize)).CompCount as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                    {
                        let mut gid: USHORT = *(*(*ligset).Ligature.offset(j as isize))
                            .Component
                            .offset(i as isize)
                            as USHORT;
                        if (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
                            ch = if UC_is_valid(*map_base.offset(gid as isize))
                                as ::core::ffi::c_int
                                != 0
                            {
                                *map_base.offset(gid as isize)
                            } else {
                                *map_sub.offset(gid as isize)
                            };
                            *ucv.offset(
                                (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                            ) = ch;
                            fail_count += if UC_is_valid(ch) as ::core::ffi::c_int != 0 {
                                0 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            };
                        } else {
                            fail_count += 1 as ::core::ffi::c_int;
                        }
                        i = i.wrapping_add(1);
                    }
                    if fail_count == 0 as ::core::ffi::c_int {
                        let mut src: [::core::ffi::c_uchar; 2] = [0; 2];
                        let mut dst: *mut ::core::ffi::c_uchar =
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        let mut p: *mut ::core::ffi::c_uchar =
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        let mut endptr: *mut ::core::ffi::c_uchar =
                            ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        let mut len: size_t = 0 as size_t;
                        src[0 as ::core::ffi::c_int as usize] = (cid as ::core::ffi::c_int
                            >> 8 as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        src[1 as ::core::ffi::c_int as usize] = (cid as ::core::ffi::c_int
                            & 0xff as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        dst = new(
                            ((comp_count as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as uint32_t as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_uchar>() as usize
                                ) as uint32_t,
                        ) as *mut ::core::ffi::c_uchar;
                        p = dst;
                        endptr = dst.offset(
                            (comp_count as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
                        );
                        i = 0 as USHORT;
                        while (i as ::core::ffi::c_int) < comp_count as ::core::ffi::c_int {
                            len = len.wrapping_add(UC_UTF16BE_encode_char(
                                *ucv.offset(i as isize),
                                &raw mut p,
                                endptr,
                            ));
                            i = i.wrapping_add(1);
                        }
                        CMap_add_bfchar(
                            cmap,
                            &raw mut src as *mut ::core::ffi::c_uchar,
                            2 as size_t,
                            dst,
                            len,
                        );
                        let ref mut fresh3 = *used_chars
                            .offset((cid as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
                        *fresh3 = (*fresh3 as ::core::ffi::c_int
                            & !((1 as ::core::ffi::c_int)
                                << 7 as ::core::ffi::c_int
                                    - cid as ::core::ffi::c_int % 8 as ::core::ffi::c_int))
                            as ::core::ffi::c_char;
                        count += 1;
                        free(dst as *mut ::core::ffi::c_void);
                    }
                    free(ucv as *mut ::core::ffi::c_void);
                }
            }
            j = j.wrapping_add(1);
        }
    }
    return count;
}
unsafe extern "C-unwind" fn add_ToUnicode_ligature(
    mut cmap: *mut CMap,
    mut used_chars: *mut ::core::ffi::c_char,
    mut subtab: *mut otl_gsub_subtab,
    mut map_base: *mut int32_t,
    mut map_sub: *mut int32_t,
    mut num_glyphs: USHORT,
    mut GIDToCIDMap: *mut uint16_t,
) -> int32_t {
    let mut count: int32_t = 0 as int32_t;
    let mut i: USHORT = 0;
    let mut idx: USHORT = 0;
    let mut gid: USHORT = 0;
    if subtab.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"subtab\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*subtab).SubstFormat as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        let mut data: *mut otl_gsub_ligature1 = ::core::ptr::null_mut::<otl_gsub_ligature1>();
        let mut cov: *mut clt_coverage = ::core::ptr::null_mut::<clt_coverage>();
        data = (*subtab).table.ligature1;
        cov = &raw mut (*data).coverage;
        match (*cov).format as ::core::ffi::c_int {
            1 => {
                idx = 0 as USHORT;
                while (idx as ::core::ffi::c_int) < (*cov).count as ::core::ffi::c_int {
                    gid = *(*cov).list.offset(idx as isize) as USHORT;
                    if (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
                        count += add_ligature1_inverse_map(
                            cmap,
                            used_chars,
                            map_base,
                            map_sub,
                            num_glyphs,
                            GIDToCIDMap,
                            gid,
                            idx as ::core::ffi::c_int,
                            data,
                        );
                    }
                    idx = idx.wrapping_add(1);
                }
            }
            2 => {
                i = 0 as USHORT;
                while (i as ::core::ffi::c_int) < (*cov).count as ::core::ffi::c_int {
                    gid = (*(*cov).range.offset(i as isize)).Start as USHORT;
                    while gid as ::core::ffi::c_int
                        <= (*(*cov).range.offset(i as isize)).End as ::core::ffi::c_int
                        && (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int
                    {
                        idx = ((*(*cov).range.offset(i as isize)).StartCoverageIndex
                            as ::core::ffi::c_int
                            + gid as ::core::ffi::c_int
                            - (*(*cov).range.offset(i as isize)).Start as ::core::ffi::c_int)
                            as USHORT;
                        if (gid as ::core::ffi::c_int) < num_glyphs as ::core::ffi::c_int {
                            count += add_ligature1_inverse_map(
                                cmap,
                                used_chars,
                                map_base,
                                map_sub,
                                num_glyphs,
                                GIDToCIDMap,
                                gid,
                                idx as ::core::ffi::c_int,
                                data,
                            );
                        }
                        gid = gid.wrapping_add(1);
                    }
                    i = i.wrapping_add(1);
                }
            }
            _ => {}
        }
    }
    return 0 as int32_t;
}
#[unsafe(export_name = "__pdf_impl_otl_gsub_add_ToUnicode")]
pub unsafe extern "C-unwind" fn otl_gsub_add_ToUnicode(
    mut cmap: *mut CMap,
    mut used_chars: *mut ::core::ffi::c_char,
    mut map_base: *mut int32_t,
    mut map_sub: *mut int32_t,
    mut num_glyphs: USHORT,
    mut GIDToCIDMap: *mut uint16_t,
    mut sfont: *mut sfnt,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut gsub_list: *mut otl_gsub = ::core::ptr::null_mut::<otl_gsub>();
    let mut gsub: *mut otl_gsub_tab = ::core::ptr::null_mut::<otl_gsub_tab>();
    let mut subtab: *mut otl_gsub_subtab = ::core::ptr::null_mut::<otl_gsub_subtab>();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    gsub_list = otl_gsub_new();
    otl_gsub_add_feat(
        gsub_list,
        b"*\0" as *const u8 as *const ::core::ffi::c_char,
        b"*\0" as *const u8 as *const ::core::ffi::c_char,
        b"*\0" as *const u8 as *const ::core::ffi::c_char,
        sfont,
    );
    i = 0 as ::core::ffi::c_int;
    while i < (*gsub_list).num_gsubs {
        gsub = (&raw mut (*gsub_list).gsubs as *mut otl_gsub_tab).offset(i as isize)
            as *mut otl_gsub_tab;
        j = 0 as ::core::ffi::c_int;
        while j < (*gsub).num_subtables {
            subtab = (*gsub).subtables.offset(j as isize) as *mut otl_gsub_subtab;
            match (*subtab).LookupType as ::core::ffi::c_int {
                OTL_GSUB_TYPE_SINGLE => {
                    count += add_ToUnicode_single(
                        cmap,
                        used_chars,
                        subtab,
                        map_base,
                        map_sub,
                        num_glyphs,
                        GIDToCIDMap,
                    );
                }
                OTL_GSUB_TYPE_ALTERNATE => {
                    count += add_ToUnicode_alternate(
                        cmap,
                        used_chars,
                        subtab,
                        map_base,
                        map_sub,
                        num_glyphs,
                        GIDToCIDMap,
                    ) as ::core::ffi::c_int;
                }
                OTL_GSUB_TYPE_LIGATURE => {
                    count += add_ToUnicode_ligature(
                        cmap,
                        used_chars,
                        subtab,
                        map_base,
                        map_sub,
                        num_glyphs,
                        GIDToCIDMap,
                    ) as ::core::ffi::c_int;
                }
                _ => {}
            }
            j += 1;
        }
        i += 1;
    }
    otl_gsub_release(gsub_list);
    return count;
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct otl_opt {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_otl_gsub_new(_result: *mut *mut otl_gsub) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otl_gsub_new() }) {
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
pub unsafe extern "C" fn __pdf_entry_otl_gsub_add_feat(
    mut gsub_list: *mut otl_gsub,
    mut script: *const ::core::ffi::c_char,
    mut language: *const ::core::ffi::c_char,
    mut feature: *const ::core::ffi::c_char,
    mut sfont: *mut sfnt,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        otl_gsub_add_feat(gsub_list, script, language, feature, sfont)
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
pub unsafe extern "C" fn __pdf_entry_otl_gsub_release(mut gsub_list: *mut otl_gsub) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otl_gsub_release(gsub_list) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_otl_gsub_apply(
    mut gsub_list: *mut otl_gsub,
    mut gid: *mut USHORT,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otl_gsub_apply(gsub_list, gid) }) {
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
pub unsafe extern "C" fn __pdf_entry_otl_gsub_apply_alt(
    mut gsub_list: *mut otl_gsub,
    mut alt_idx: USHORT,
    mut gid: *mut USHORT,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        otl_gsub_apply_alt(gsub_list, alt_idx, gid)
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
pub unsafe extern "C" fn __pdf_entry_otl_gsub_apply_lig(
    mut gsub_list: *mut otl_gsub,
    mut gid_in: *mut USHORT,
    mut num_gids: USHORT,
    mut gid_out: *mut USHORT,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        otl_gsub_apply_lig(gsub_list, gid_in, num_gids, gid_out)
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
pub unsafe extern "C" fn __pdf_entry_otl_gsub_select(
    mut gsub_list: *mut otl_gsub,
    mut script: *const ::core::ffi::c_char,
    mut language: *const ::core::ffi::c_char,
    mut feature: *const ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        otl_gsub_select(gsub_list, script, language, feature)
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
pub unsafe extern "C" fn __pdf_entry_otl_gsub_set_chain(
    mut gsub_list: *mut otl_gsub,
    mut otl_tags: *const ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otl_gsub_set_chain(gsub_list, otl_tags) }) {
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
pub unsafe extern "C" fn __pdf_entry_otl_gsub_add_feat_list(
    mut gsub_list: *mut otl_gsub,
    mut otl_tags: *const ::core::ffi::c_char,
    mut sfont: *mut sfnt,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        otl_gsub_add_feat_list(gsub_list, otl_tags, sfont)
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
pub unsafe extern "C" fn __pdf_entry_otl_gsub_apply_chain(
    mut gsub_list: *mut otl_gsub,
    mut gid: *mut USHORT,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otl_gsub_apply_chain(gsub_list, gid) }) {
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
pub unsafe extern "C" fn __pdf_entry_otl_gsub_add_ToUnicode(
    mut cmap: *mut CMap,
    mut used_chars: *mut ::core::ffi::c_char,
    mut map_base: *mut int32_t,
    mut map_sub: *mut int32_t,
    mut num_glyphs: USHORT,
    mut GIDToCIDMap: *mut uint16_t,
    mut sfont: *mut sfnt,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        otl_gsub_add_ToUnicode(
            cmap,
            used_chars,
            map_base,
            map_sub,
            num_glyphs,
            GIDToCIDMap,
            sfont,
        )
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
