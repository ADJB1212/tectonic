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
#![allow(
    warnings,
    clippy::all,
    unsafe_op_in_unsafe_fn,
    static_mut_refs,
    dangerous_implicit_autorefs
)]
unsafe extern "C-unwind" {
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
pub type card8 = ::core::ffi::c_uchar;
pub type card16 = ::core::ffi::c_ushort;
pub type c_offsize = ::core::ffi::c_uchar;
pub type l_offset = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cff_index {
    pub count: card16,
    pub offsize: c_offsize,
    pub offset: *mut l_offset,
    pub data: *mut card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cs_ginfo {
    pub flags: ::core::ffi::c_int,
    pub wx: ::core::ffi::c_double,
    pub wy: ::core::ffi::c_double,
    pub bbox: C2RustUnnamed_0,
    pub seac: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub asb: ::core::ffi::c_double,
    pub adx: ::core::ffi::c_double,
    pub ady: ::core::ffi::c_double,
    pub bchar: card8,
    pub achar: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub llx: ::core::ffi::c_double,
    pub lly: ::core::ffi::c_double,
    pub urx: ::core::ffi::c_double,
    pub ury: ::core::ffi::c_double,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const CS_SUBR_NEST_MAX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const CS_TYPE2_DEBUG_STR: [::core::ffi::c_char; 24] =
    crate::backend::c_char_array(*b"Type2 Charstring Parser\0");
pub const CS_BUFFER_ERROR: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const CS_STACK_ERROR: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const CS_PARSE_ERROR: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const CS_PARSE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CS_SUBR_RETURN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CS_CHAR_END: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const cs_hstem: ::core::ffi::c_int = 1;
pub const cs_vstem: ::core::ffi::c_int = 3;
pub const cs_vmoveto: ::core::ffi::c_int = 4;
pub const cs_rlineto: ::core::ffi::c_int = 5;
pub const cs_hlineto: ::core::ffi::c_int = 6;
pub const cs_vlineto: ::core::ffi::c_int = 7;
pub const cs_rrcurveto: ::core::ffi::c_int = 8;
pub const cs_callsubr: ::core::ffi::c_int = 10;
pub const cs_return: ::core::ffi::c_int = 11;
pub const cs_escape: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const cs_endchar: ::core::ffi::c_int = 14;
pub const cs_hstemhm: ::core::ffi::c_int = 18;
pub const cs_hintmask: ::core::ffi::c_int = 19;
pub const cs_cntrmask: ::core::ffi::c_int = 20;
pub const cs_rmoveto: ::core::ffi::c_int = 21;
pub const cs_hmoveto: ::core::ffi::c_int = 22;
pub const cs_vstemhm: ::core::ffi::c_int = 23;
pub const cs_rcurveline: ::core::ffi::c_int = 24;
pub const cs_rlinecurve: ::core::ffi::c_int = 25;
pub const cs_vvcurveto: ::core::ffi::c_int = 26;
pub const cs_hhcurveto: ::core::ffi::c_int = 27;
pub const cs_callgsubr: ::core::ffi::c_int = 29;
pub const cs_vhcurveto: ::core::ffi::c_int = 30;
pub const cs_hvcurveto: ::core::ffi::c_int = 31;
pub const cs_dotsection: ::core::ffi::c_int = 0;
pub const cs_and: ::core::ffi::c_int = 3;
pub const cs_or: ::core::ffi::c_int = 4;
pub const cs_not: ::core::ffi::c_int = 5;
pub const cs_abs: ::core::ffi::c_int = 9;
pub const cs_add: ::core::ffi::c_int = 10;
pub const cs_sub: ::core::ffi::c_int = 11;
pub const cs_div: ::core::ffi::c_int = 12;
pub const cs_neg: ::core::ffi::c_int = 14;
pub const cs_eq: ::core::ffi::c_int = 15;
pub const cs_drop: ::core::ffi::c_int = 18;
pub const cs_put: ::core::ffi::c_int = 20;
pub const cs_get: ::core::ffi::c_int = 21;
pub const cs_ifelse: ::core::ffi::c_int = 22;
pub const cs_random: ::core::ffi::c_int = 23;
pub const cs_mul: ::core::ffi::c_int = 24;
pub const cs_sqrt: ::core::ffi::c_int = 26;
pub const cs_dup: ::core::ffi::c_int = 27;
pub const cs_exch: ::core::ffi::c_int = 28;
pub const cs_index: ::core::ffi::c_int = 29;
pub const cs_roll: ::core::ffi::c_int = 30;
pub const cs_hflex: ::core::ffi::c_int = 34;
pub const cs_flex: ::core::ffi::c_int = 35;
pub const cs_hflex1: ::core::ffi::c_int = 36;
pub const cs_flex1: ::core::ffi::c_int = 37;
unsafe fn subroutines<'a>(index: *mut cff_index) -> Option<crate::type2::Subroutines<'a>> {
    if index.is_null() || unsafe { (*index).count == 0 } {
        return None;
    }
    unsafe {
        let index = &*index;
        if index.offset.is_null() {
            pdf_abort!("Invalid CFF Index offset data");
        }
        let offsets = core::slice::from_raw_parts(index.offset, index.count as usize + 1);
        let length = offsets
            .last()
            .copied()
            .and_then(|v| v.checked_sub(1))
            .filter(|&v| v <= i32::MAX as u32)
            .unwrap_or_else(|| pdf_abort!("Invalid CFF Index offset data"))
            as usize;
        if length > 0 && index.data.is_null() {
            pdf_abort!("Invalid CFF Index offset data");
        }
        let bytes = if length == 0 {
            &[]
        } else {
            core::slice::from_raw_parts(index.data, length)
        };
        Some(crate::type2::Subroutines { offsets, bytes })
    }
}

#[unsafe(export_name = "__pdf_impl_cs_copy_charstring")]
pub unsafe extern "C-unwind" fn cs_copy_charstring(
    dst: *mut card8,
    dstlen: i32,
    src: *mut card8,
    srclen: i32,
    gsubr: *mut cff_index,
    subr: *mut cff_index,
    default_width: f64,
    nominal_width: f64,
    ginfo: *mut cs_ginfo,
) -> i32 {
    if dstlen < 0 || srclen < 0 || (dstlen > 0 && dst.is_null()) || (srclen > 0 && src.is_null()) {
        pdf_abort!("Type2 Charstring Parser: Invalid charstring length.");
    }
    // The synchronous caller owns disjoint source, destination and subroutine buffers.
    let bytes = if srclen == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(src, srclen as usize) }
    };
    let output = if dstlen == 0 {
        &mut []
    } else {
        unsafe { core::slice::from_raw_parts_mut(dst, dstlen as usize) }
    };
    let global = unsafe { subroutines(gsubr) };
    let local = unsafe { subroutines(subr) };
    let state = unsafe { &mut *document_state() };
    let result = crate::type2::copy(
        bytes,
        output,
        global.as_ref(),
        local.as_ref(),
        &mut state.transient,
        &mut state.stack,
        |message| {
            pdf_warning!("{}", message);
        },
    )
    .unwrap_or_else(|error| pdf_abort!("{}", error.message.to_string_lossy()));
    if !ginfo.is_null() {
        unsafe {
            (*ginfo).flags = 0;
            (*ginfo).wx = result
                .width
                .map_or(default_width, |width| nominal_width + width);
        }
    }
    result.length as i32
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_cs_copy_charstring(
    mut dst: *mut card8,
    mut dstlen: ::core::ffi::c_int,
    mut src: *mut card8,
    mut srclen: ::core::ffi::c_int,
    mut gsubr: *mut cff_index,
    mut subr: *mut cff_index,
    mut default_width: ::core::ffi::c_double,
    mut nominal_width: ::core::ffi::c_double,
    mut ginfo: *mut cs_ginfo,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        cs_copy_charstring(
            dst,
            dstlen,
            src,
            srclen,
            gsubr,
            subr,
            default_width,
            nominal_width,
            ginfo,
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

pub(crate) struct DocumentState {
    transient: [f64; 32],
    stack: Vec<f64>,
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self {
            transient: [0.0; 32],
            stack: Vec::with_capacity(48),
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe {
        (*crate::compat::document::current())
            .backend
            .dpx_cs_type2
            .pointer
    }
}
