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
pub const CS_TYPE2_DEBUG_STR: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"Type2 Charstring Parser\0")
};
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
unsafe extern "C-unwind" fn clear_stack(mut dest: *mut *mut card8, mut limit: *mut card8) {
    let __pdf_state = unsafe { document_state() };
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*__pdf_state).stack_top {
        let mut value: ::core::ffi::c_double = 0.;
        let mut ivalue: ::core::ffi::c_int = 0;
        value = (*__pdf_state).arg_stack[i as usize];
        ivalue = floor(value + 0.5f64) as ::core::ffi::c_int;
        if value >= 0x8000 as ::core::ffi::c_long as ::core::ffi::c_double
            || value
                <= (-(0x8000 as ::core::ffi::c_long) - 1 as ::core::ffi::c_long)
                    as ::core::ffi::c_double
        {
            pdf_abort!(
                b"%s: Argument value too large. (This is bug)\0" as *const u8
                    as *const ::core::ffi::c_char,
                CS_TYPE2_DEBUG_STR.as_ptr(),
            );
        } else if fabs(value - ivalue as ::core::ffi::c_double) > 3.0e-5f64 {
            if limit < (*dest).offset(5 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh7 = *dest;
            *dest = (*dest).offset(1);
            *fresh7 = 255 as card8;
            ivalue = floor(value) as ::core::ffi::c_int;
            let fresh8 = *dest;
            *dest = (*dest).offset(1);
            *fresh8 = (ivalue >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh9 = *dest;
            *dest = (*dest).offset(1);
            *fresh9 = (ivalue & 0xff as ::core::ffi::c_int) as card8;
            ivalue = ((value - ivalue as ::core::ffi::c_double)
                * 0x10000 as ::core::ffi::c_long as ::core::ffi::c_double)
                as ::core::ffi::c_int;
            let fresh10 = *dest;
            *dest = (*dest).offset(1);
            *fresh10 = (ivalue >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh11 = *dest;
            *dest = (*dest).offset(1);
            *fresh11 = (ivalue & 0xff as ::core::ffi::c_int) as card8;
        } else if ivalue >= -(107 as ::core::ffi::c_int) && ivalue <= 107 as ::core::ffi::c_int {
            if limit < (*dest).offset(1 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh12 = *dest;
            *dest = (*dest).offset(1);
            *fresh12 = (ivalue + 139 as ::core::ffi::c_int) as card8;
        } else if ivalue >= 108 as ::core::ffi::c_int && ivalue <= 1131 as ::core::ffi::c_int {
            if limit < (*dest).offset(2 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            ivalue = (0xf700 as ::core::ffi::c_uint)
                .wrapping_add(ivalue as ::core::ffi::c_uint)
                .wrapping_sub(108 as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
            let fresh13 = *dest;
            *dest = (*dest).offset(1);
            *fresh13 = (ivalue >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh14 = *dest;
            *dest = (*dest).offset(1);
            *fresh14 = (ivalue & 0xff as ::core::ffi::c_int) as card8;
        } else if ivalue >= -(1131 as ::core::ffi::c_int) && ivalue <= -(108 as ::core::ffi::c_int)
        {
            if limit < (*dest).offset(2 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            ivalue = (0xfb00 as ::core::ffi::c_uint)
                .wrapping_sub(ivalue as ::core::ffi::c_uint)
                .wrapping_sub(108 as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
            let fresh15 = *dest;
            *dest = (*dest).offset(1);
            *fresh15 = (ivalue >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh16 = *dest;
            *dest = (*dest).offset(1);
            *fresh16 = (ivalue & 0xff as ::core::ffi::c_int) as card8;
        } else if ivalue >= -(32768 as ::core::ffi::c_int) && ivalue <= 32767 as ::core::ffi::c_int
        {
            if limit < (*dest).offset(3 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh17 = *dest;
            *dest = (*dest).offset(1);
            *fresh17 = 28 as card8;
            let fresh18 = *dest;
            *dest = (*dest).offset(1);
            *fresh18 = (ivalue >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh19 = *dest;
            *dest = (*dest).offset(1);
            *fresh19 = (ivalue & 0xff as ::core::ffi::c_int) as card8;
        } else {
            pdf_abort!(
                b"%s: Unexpected error.\0" as *const u8 as *const ::core::ffi::c_char,
                CS_TYPE2_DEBUG_STR.as_ptr(),
            );
        }
        i += 1;
    }
    (*__pdf_state).stack_top = 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn do_operator1(
    mut dest: *mut *mut card8,
    mut limit: *mut card8,
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
) {
    let __pdf_state = unsafe { document_state() };
    let mut op: card8 = **data;
    *data = (*data).offset(1 as ::core::ffi::c_int as isize);
    match op as ::core::ffi::c_int {
        cs_hstemhm | cs_vstemhm | cs_hstem | cs_vstem => {
            if (*__pdf_state).phase == 0 as ::core::ffi::c_int
                && (*__pdf_state).stack_top % 2 as ::core::ffi::c_int != 0
            {
                (*__pdf_state).have_width = 1 as ::core::ffi::c_int;
                (*__pdf_state).width = (*__pdf_state).arg_stack[0 as ::core::ffi::c_int as usize];
            }
            (*__pdf_state).num_stems += (*__pdf_state).stack_top / 2 as ::core::ffi::c_int;
            clear_stack(dest, limit);
            if limit < (*dest).offset(1 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh1 = *dest;
            *dest = (*dest).offset(1);
            *fresh1 = op;
            (*__pdf_state).phase = 1 as ::core::ffi::c_int;
        }
        cs_hintmask | cs_cntrmask => {
            if (*__pdf_state).phase < 2 as ::core::ffi::c_int {
                if (*__pdf_state).phase == 0 as ::core::ffi::c_int
                    && (*__pdf_state).stack_top % 2 as ::core::ffi::c_int != 0
                {
                    (*__pdf_state).have_width = 1 as ::core::ffi::c_int;
                    (*__pdf_state).width =
                        (*__pdf_state).arg_stack[0 as ::core::ffi::c_int as usize];
                }
                (*__pdf_state).num_stems += (*__pdf_state).stack_top / 2 as ::core::ffi::c_int;
            }
            clear_stack(dest, limit);
            if limit < (*dest).offset(1 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh2 = *dest;
            *dest = (*dest).offset(1);
            *fresh2 = op;
            if (*__pdf_state).num_stems > 0 as ::core::ffi::c_int {
                let mut masklen: ::core::ffi::c_int =
                    ((*__pdf_state).num_stems + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int;
                if limit < (*dest).offset(masklen as isize) {
                    (*__pdf_state).status = CS_BUFFER_ERROR;
                    return;
                }
                if endptr < (*data).offset(masklen as isize) {
                    (*__pdf_state).status = CS_PARSE_ERROR;
                    return;
                }
                memmove(
                    *dest as *mut ::core::ffi::c_void,
                    *data as *const ::core::ffi::c_void,
                    masklen as size_t,
                );
                *data = (*data).offset(masklen as isize);
                *dest = (*dest).offset(masklen as isize);
            }
            (*__pdf_state).phase = 2 as ::core::ffi::c_int;
        }
        cs_rmoveto => {
            if (*__pdf_state).phase == 0 as ::core::ffi::c_int
                && (*__pdf_state).stack_top % 2 as ::core::ffi::c_int != 0
            {
                (*__pdf_state).have_width = 1 as ::core::ffi::c_int;
                (*__pdf_state).width = (*__pdf_state).arg_stack[0 as ::core::ffi::c_int as usize];
            }
            clear_stack(dest, limit);
            if limit < (*dest).offset(1 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh3 = *dest;
            *dest = (*dest).offset(1);
            *fresh3 = op;
            (*__pdf_state).phase = 2 as ::core::ffi::c_int;
        }
        cs_hmoveto | cs_vmoveto => {
            if (*__pdf_state).phase == 0 as ::core::ffi::c_int
                && (*__pdf_state).stack_top % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                (*__pdf_state).have_width = 1 as ::core::ffi::c_int;
                (*__pdf_state).width = (*__pdf_state).arg_stack[0 as ::core::ffi::c_int as usize];
            }
            clear_stack(dest, limit);
            if limit < (*dest).offset(1 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh4 = *dest;
            *dest = (*dest).offset(1);
            *fresh4 = op;
            (*__pdf_state).phase = 2 as ::core::ffi::c_int;
        }
        cs_endchar => {
            if (*__pdf_state).stack_top == 1 as ::core::ffi::c_int {
                (*__pdf_state).have_width = 1 as ::core::ffi::c_int;
                (*__pdf_state).width = (*__pdf_state).arg_stack[0 as ::core::ffi::c_int as usize];
                clear_stack(dest, limit);
            } else if (*__pdf_state).stack_top == 4 as ::core::ffi::c_int
                || (*__pdf_state).stack_top == 5 as ::core::ffi::c_int
            {
                pdf_warning!(
                    b"\"seac\" character deprecated in Type 2 charstring.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                (*__pdf_state).status = CS_PARSE_ERROR;
                return;
            } else if (*__pdf_state).stack_top > 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%s: Operand stack not empty.\0" as *const u8 as *const ::core::ffi::c_char,
                    CS_TYPE2_DEBUG_STR.as_ptr(),
                );
            }
            if limit < (*dest).offset(1 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh5 = *dest;
            *dest = (*dest).offset(1);
            *fresh5 = op;
            (*__pdf_state).status = CS_CHAR_END;
        }
        cs_rlineto | cs_hlineto | cs_vlineto | cs_rrcurveto | cs_rcurveline | cs_rlinecurve
        | cs_vvcurveto | cs_hhcurveto | cs_vhcurveto | cs_hvcurveto => {
            if (*__pdf_state).phase < 2 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%s: Broken Type 2 charstring.\0" as *const u8 as *const ::core::ffi::c_char,
                    CS_TYPE2_DEBUG_STR.as_ptr(),
                );
                (*__pdf_state).status = CS_PARSE_ERROR;
                return;
            }
            clear_stack(dest, limit);
            if limit < (*dest).offset(1 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh6 = *dest;
            *dest = (*dest).offset(1);
            *fresh6 = op;
        }
        cs_return | cs_callgsubr | cs_callsubr => {
            pdf_abort!(
                b"%s: Unexpected call(g)subr/return\0" as *const u8 as *const ::core::ffi::c_char,
                CS_TYPE2_DEBUG_STR.as_ptr(),
            );
        }
        _ => {
            pdf_warning!(
                b"%s: Unknown charstring operator: 0x%02x\0" as *const u8
                    as *const ::core::ffi::c_char,
                CS_TYPE2_DEBUG_STR.as_ptr(),
                op as ::core::ffi::c_int,
            );
            (*__pdf_state).status = CS_PARSE_ERROR;
        }
    };
}
unsafe extern "C-unwind" fn do_operator2(
    mut dest: *mut *mut card8,
    mut limit: *mut card8,
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
) {
    let __pdf_state = unsafe { document_state() };
    let mut op: card8 = 0;
    *data = (*data).offset(1 as ::core::ffi::c_int as isize);
    if endptr < (*data).offset(1 as ::core::ffi::c_int as isize) {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
    op = **data;
    *data = (*data).offset(1 as ::core::ffi::c_int as isize);
    match op as ::core::ffi::c_int {
        cs_dotsection => {
            pdf_warning!(
                b"Operator \"dotsection\" deprecated in Type 2 charstring.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            (*__pdf_state).status = CS_PARSE_ERROR;
            return;
        }
        cs_hflex | cs_flex | cs_hflex1 | cs_flex1 => {
            if (*__pdf_state).phase < 2 as ::core::ffi::c_int {
                pdf_warning!(
                    b"%s: Broken Type 2 charstring.\0" as *const u8 as *const ::core::ffi::c_char,
                    CS_TYPE2_DEBUG_STR.as_ptr(),
                );
                (*__pdf_state).status = CS_PARSE_ERROR;
                return;
            }
            clear_stack(dest, limit);
            if limit < (*dest).offset(2 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh20 = *dest;
            *dest = (*dest).offset(1);
            *fresh20 = cs_escape as card8;
            let fresh21 = *dest;
            *dest = (*dest).offset(1);
            *fresh21 = op;
        }
        cs_and => {
            if (*__pdf_state).stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).stack_top -= 1;
            if (*__pdf_state).arg_stack[(*__pdf_state).stack_top as usize] != 0.
                && (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize]
                    != 0.
            {
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] = 1.0f64;
            } else {
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] = 0.0f64;
            }
        }
        cs_or => {
            if (*__pdf_state).stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).stack_top -= 1;
            if (*__pdf_state).arg_stack[(*__pdf_state).stack_top as usize] != 0.
                || (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize]
                    != 0.
            {
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] = 1.0f64;
            } else {
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] = 0.0f64;
            }
        }
        cs_not => {
            if (*__pdf_state).stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            if (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize]
                != 0.
            {
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] = 0.0f64;
            } else {
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] = 1.0f64;
            }
        }
        cs_abs => {
            if (*__pdf_state).stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] = fabs(
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize],
            );
        }
        cs_add => {
            if (*__pdf_state).stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 2 as ::core::ffi::c_int) as usize] += (*__pdf_state)
                .arg_stack[((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize];
            (*__pdf_state).stack_top -= 1;
        }
        cs_sub => {
            if (*__pdf_state).stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 2 as ::core::ffi::c_int) as usize] -= (*__pdf_state)
                .arg_stack[((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize];
            (*__pdf_state).stack_top -= 1;
        }
        cs_div => {
            if (*__pdf_state).stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 2 as ::core::ffi::c_int) as usize] /= (*__pdf_state)
                .arg_stack[((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize];
            (*__pdf_state).stack_top -= 1;
        }
        cs_neg => {
            if (*__pdf_state).stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] *= -1.0f64;
        }
        cs_eq => {
            if (*__pdf_state).stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).stack_top -= 1;
            if (*__pdf_state).arg_stack[(*__pdf_state).stack_top as usize]
                == (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize]
            {
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] = 1.0f64;
            } else {
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] = 0.0f64;
            }
        }
        cs_drop => {
            if (*__pdf_state).stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).stack_top -= 1;
        }
        cs_put => {
            if (*__pdf_state).stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).stack_top -= 1;
            let mut idx: ::core::ffi::c_int =
                (*__pdf_state).arg_stack[(*__pdf_state).stack_top as usize] as ::core::ffi::c_int;
            if (32 as ::core::ffi::c_int) < idx {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).stack_top -= 1;
            (*__pdf_state).trn_array[idx as usize] =
                (*__pdf_state).arg_stack[(*__pdf_state).stack_top as usize];
        }
        cs_get => {
            if (*__pdf_state).stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            let mut idx_0: ::core::ffi::c_int = (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int;
            if (32 as ::core::ffi::c_int) < idx_0 {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] =
                (*__pdf_state).trn_array[idx_0 as usize];
        }
        cs_ifelse => {
            if (*__pdf_state).stack_top < 4 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).stack_top -= 3 as ::core::ffi::c_int;
            if (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top + 1 as ::core::ffi::c_int) as usize]
                > (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top + 2 as ::core::ffi::c_int) as usize]
            {
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] =
                    (*__pdf_state).arg_stack[(*__pdf_state).stack_top as usize];
            }
        }
        cs_mul => {
            if (*__pdf_state).stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 2 as ::core::ffi::c_int) as usize] = (*__pdf_state)
                .arg_stack[((*__pdf_state).stack_top - 2 as ::core::ffi::c_int) as usize]
                * (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize];
            (*__pdf_state).stack_top -= 1;
        }
        cs_sqrt => {
            if (*__pdf_state).stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] = sqrt(
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize],
            );
        }
        cs_dup => {
            if (*__pdf_state).stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            if (48 as ::core::ffi::c_int) < (*__pdf_state).stack_top + 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).arg_stack[(*__pdf_state).stack_top as usize] = (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize];
            (*__pdf_state).stack_top += 1;
        }
        cs_exch => {
            if (*__pdf_state).stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            let mut save: ::core::ffi::c_double = (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 2 as ::core::ffi::c_int) as usize];
            (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 2 as ::core::ffi::c_int) as usize] = (*__pdf_state)
                .arg_stack[((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize];
            (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] = save;
        }
        cs_index => {
            if (*__pdf_state).stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            let mut idx_1: ::core::ffi::c_int = (*__pdf_state).arg_stack
                [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int;
            if idx_1 < 0 as ::core::ffi::c_int {
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] =
                    (*__pdf_state).arg_stack
                        [((*__pdf_state).stack_top - 2 as ::core::ffi::c_int) as usize];
            } else {
                if (*__pdf_state).stack_top < idx_1 + 2 as ::core::ffi::c_int {
                    (*__pdf_state).status = CS_STACK_ERROR;
                    return;
                }
                (*__pdf_state).arg_stack
                    [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize] =
                    (*__pdf_state).arg_stack
                        [((*__pdf_state).stack_top - idx_1 - 2 as ::core::ffi::c_int) as usize];
            }
        }
        cs_roll => {
            if (*__pdf_state).stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            let mut N: ::core::ffi::c_int = 0;
            let mut J: ::core::ffi::c_int = 0;
            (*__pdf_state).stack_top -= 1;
            J = (*__pdf_state).arg_stack[(*__pdf_state).stack_top as usize] as ::core::ffi::c_int;
            (*__pdf_state).stack_top -= 1;
            N = (*__pdf_state).arg_stack[(*__pdf_state).stack_top as usize] as ::core::ffi::c_int;
            if (*__pdf_state).stack_top < N {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            if J > 0 as ::core::ffi::c_int {
                J = J % N;
                loop {
                    let fresh22 = J;
                    J = J - 1;
                    if !(fresh22 > 0 as ::core::ffi::c_int) {
                        break;
                    }
                    let mut save_0: ::core::ffi::c_double = (*__pdf_state).arg_stack
                        [((*__pdf_state).stack_top - 1 as ::core::ffi::c_int) as usize];
                    let mut i: ::core::ffi::c_int =
                        (*__pdf_state).stack_top - 1 as ::core::ffi::c_int;
                    while i > (*__pdf_state).stack_top - N {
                        (*__pdf_state).arg_stack[i as usize] =
                            (*__pdf_state).arg_stack[(i - 1 as ::core::ffi::c_int) as usize];
                        i -= 1;
                    }
                    (*__pdf_state).arg_stack[i as usize] = save_0;
                }
            } else {
                J = -J % N;
                loop {
                    let fresh23 = J;
                    J = J - 1;
                    if !(fresh23 > 0 as ::core::ffi::c_int) {
                        break;
                    }
                    let mut save_1: ::core::ffi::c_double =
                        (*__pdf_state).arg_stack[((*__pdf_state).stack_top - N) as usize];
                    let mut i_0: ::core::ffi::c_int = (*__pdf_state).stack_top - N;
                    while i_0 < (*__pdf_state).stack_top - 1 as ::core::ffi::c_int {
                        (*__pdf_state).arg_stack[i_0 as usize] =
                            (*__pdf_state).arg_stack[(i_0 + 1 as ::core::ffi::c_int) as usize];
                        i_0 += 1;
                    }
                    (*__pdf_state).arg_stack[i_0 as usize] = save_1;
                }
            }
        }
        cs_random => {
            pdf_warning!(
                b"%s: Charstring operator \"random\" found.\0" as *const u8
                    as *const ::core::ffi::c_char,
                CS_TYPE2_DEBUG_STR.as_ptr(),
            );
            if (48 as ::core::ffi::c_int) < (*__pdf_state).stack_top + 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            let fresh24 = (*__pdf_state).stack_top;
            (*__pdf_state).stack_top = (*__pdf_state).stack_top + 1;
            (*__pdf_state).arg_stack[fresh24 as usize] = 1.0f64;
        }
        _ => {
            pdf_warning!(
                b"%s: Unknown charstring operator: 0x0c%02x\0" as *const u8
                    as *const ::core::ffi::c_char,
                CS_TYPE2_DEBUG_STR.as_ptr(),
                op as ::core::ffi::c_int,
            );
            (*__pdf_state).status = CS_PARSE_ERROR;
        }
    };
}
unsafe extern "C-unwind" fn get_integer(mut data: *mut *mut card8, mut endptr: *mut card8) {
    let __pdf_state = unsafe { document_state() };
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut b0: card8 = **data;
    let mut b1: card8 = 0;
    let mut b2: card8 = 0;
    *data = (*data).offset(1 as ::core::ffi::c_int as isize);
    if b0 as ::core::ffi::c_int == 28 as ::core::ffi::c_int {
        if endptr < (*data).offset(2 as ::core::ffi::c_int as isize) {
            (*__pdf_state).status = CS_PARSE_ERROR;
            return;
        }
        b1 = **data;
        b2 = *(*data).offset(1 as ::core::ffi::c_int as isize);
        result = b1 as ::core::ffi::c_int * 256 as ::core::ffi::c_int + b2 as ::core::ffi::c_int;
        if result > 0x7fff as ::core::ffi::c_int {
            result = (result as ::core::ffi::c_long - 0x10000 as ::core::ffi::c_long)
                as ::core::ffi::c_int;
        }
        *data = (*data).offset(2 as ::core::ffi::c_int as isize);
    } else if b0 as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 246 as ::core::ffi::c_int
    {
        result = b0 as ::core::ffi::c_int - 139 as ::core::ffi::c_int;
    } else if b0 as ::core::ffi::c_int >= 247 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 250 as ::core::ffi::c_int
    {
        if endptr < (*data).offset(1 as ::core::ffi::c_int as isize) {
            (*__pdf_state).status = CS_PARSE_ERROR;
            return;
        }
        b1 = **data;
        result = (b0 as ::core::ffi::c_int - 247 as ::core::ffi::c_int) * 256 as ::core::ffi::c_int
            + b1 as ::core::ffi::c_int
            + 108 as ::core::ffi::c_int;
        *data = (*data).offset(1 as ::core::ffi::c_int as isize);
    } else if b0 as ::core::ffi::c_int >= 251 as ::core::ffi::c_int
        && b0 as ::core::ffi::c_int <= 254 as ::core::ffi::c_int
    {
        if endptr < (*data).offset(1 as ::core::ffi::c_int as isize) {
            (*__pdf_state).status = CS_PARSE_ERROR;
            return;
        }
        b1 = **data;
        result = -(b0 as ::core::ffi::c_int - 251 as ::core::ffi::c_int)
            * 256 as ::core::ffi::c_int
            - b1 as ::core::ffi::c_int
            - 108 as ::core::ffi::c_int;
        *data = (*data).offset(1 as ::core::ffi::c_int as isize);
    } else {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
    if (48 as ::core::ffi::c_int) < (*__pdf_state).stack_top + 1 as ::core::ffi::c_int {
        (*__pdf_state).status = CS_STACK_ERROR;
        return;
    }
    let fresh0 = (*__pdf_state).stack_top;
    (*__pdf_state).stack_top = (*__pdf_state).stack_top + 1;
    (*__pdf_state).arg_stack[fresh0 as usize] = result as ::core::ffi::c_double;
}
unsafe extern "C-unwind" fn get_fixed(mut data: *mut *mut card8, mut endptr: *mut card8) {
    let __pdf_state = unsafe { document_state() };
    let mut ivalue: ::core::ffi::c_int = 0;
    let mut rvalue: ::core::ffi::c_double = 0.;
    *data = (*data).offset(1 as ::core::ffi::c_int as isize);
    if endptr < (*data).offset(4 as ::core::ffi::c_int as isize) {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
    ivalue = **data as ::core::ffi::c_int * 0x100 as ::core::ffi::c_int
        + *(*data).offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    rvalue = (if ivalue as ::core::ffi::c_long > 0x7fff as ::core::ffi::c_long {
        ivalue as ::core::ffi::c_long - 0x10000 as ::core::ffi::c_long
    } else {
        ivalue as ::core::ffi::c_long
    }) as ::core::ffi::c_double;
    ivalue = *(*data).offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        * 0x100 as ::core::ffi::c_int
        + *(*data).offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    rvalue +=
        ivalue as ::core::ffi::c_double / 0x10000 as ::core::ffi::c_long as ::core::ffi::c_double;
    if (48 as ::core::ffi::c_int) < (*__pdf_state).stack_top + 1 as ::core::ffi::c_int {
        (*__pdf_state).status = CS_STACK_ERROR;
        return;
    }
    let fresh25 = (*__pdf_state).stack_top;
    (*__pdf_state).stack_top = (*__pdf_state).stack_top + 1;
    (*__pdf_state).arg_stack[fresh25 as usize] = rvalue;
    *data = (*data).offset(4 as ::core::ffi::c_int as isize);
}
unsafe extern "C-unwind" fn get_subr(
    mut subr: *mut *mut card8,
    mut len: *mut ::core::ffi::c_int,
    mut subr_idx: *mut cff_index,
    mut id: ::core::ffi::c_int,
) {
    let mut count: card16 = 0;
    if subr_idx.is_null() {
        pdf_abort!(
            b"%s: Subroutine called but no subroutine found.\0" as *const u8
                as *const ::core::ffi::c_char,
            CS_TYPE2_DEBUG_STR.as_ptr(),
        );
    }
    count = (*subr_idx).count;
    if (count as ::core::ffi::c_int) < 1240 as ::core::ffi::c_int {
        id += 107 as ::core::ffi::c_int;
    } else if (count as ::core::ffi::c_int) < 33900 as ::core::ffi::c_int {
        id += 1131 as ::core::ffi::c_int;
    } else {
        id += 32768 as ::core::ffi::c_int;
    }
    if id > count as ::core::ffi::c_int {
        pdf_abort!(
            b"%s: Invalid Subr index: %d (max=%u)\0" as *const u8 as *const ::core::ffi::c_char,
            CS_TYPE2_DEBUG_STR.as_ptr(),
            id,
            count as ::core::ffi::c_int,
        );
    }
    *len = (*(*subr_idx)
        .offset
        .offset((id + 1 as ::core::ffi::c_int) as isize))
    .wrapping_sub(*(*subr_idx).offset.offset(id as isize)) as ::core::ffi::c_int;
    *subr = (*subr_idx)
        .data
        .offset(*(*subr_idx).offset.offset(id as isize) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
}
unsafe extern "C-unwind" fn do_charstring(
    mut dest: *mut *mut card8,
    mut limit: *mut card8,
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
    mut gsubr_idx: *mut cff_index,
    mut subr_idx: *mut cff_index,
) {
    let __pdf_state = unsafe { document_state() };
    let mut b0: card8 = 0 as card8;
    let mut subr: *mut card8 = ::core::ptr::null_mut::<card8>();
    let mut len: ::core::ffi::c_int = 0;
    if (*__pdf_state).nest > CS_SUBR_NEST_MAX {
        pdf_abort!(
            b"%s: Subroutine nested too deeply.\0" as *const u8 as *const ::core::ffi::c_char,
            CS_TYPE2_DEBUG_STR.as_ptr(),
        );
    }
    (*__pdf_state).nest += 1;
    while *data < endptr && (*__pdf_state).status == CS_PARSE_OK {
        b0 = **data;
        if b0 as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
            get_fixed(data, endptr);
        } else if b0 as ::core::ffi::c_int == cs_return {
            (*__pdf_state).status = CS_SUBR_RETURN;
        } else if b0 as ::core::ffi::c_int == cs_callgsubr {
            if (*__pdf_state).stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
            } else {
                (*__pdf_state).stack_top -= 1;
                get_subr(
                    &raw mut subr,
                    &raw mut len,
                    gsubr_idx,
                    (*__pdf_state).arg_stack[(*__pdf_state).stack_top as usize]
                        as ::core::ffi::c_int,
                );
                if (*dest).offset(len as isize) > limit {
                    pdf_abort!(
                        b"%s: Possible buffer overflow.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        CS_TYPE2_DEBUG_STR.as_ptr(),
                    );
                }
                do_charstring(
                    dest,
                    limit,
                    &raw mut subr,
                    subr.offset(len as isize),
                    gsubr_idx,
                    subr_idx,
                );
                *data = (*data).offset(1 as ::core::ffi::c_int as isize);
            }
        } else if b0 as ::core::ffi::c_int == cs_callsubr {
            if (*__pdf_state).stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
            } else {
                (*__pdf_state).stack_top -= 1;
                get_subr(
                    &raw mut subr,
                    &raw mut len,
                    subr_idx,
                    (*__pdf_state).arg_stack[(*__pdf_state).stack_top as usize]
                        as ::core::ffi::c_int,
                );
                if limit < (*dest).offset(len as isize) {
                    pdf_abort!(
                        b"%s: Possible buffer overflow.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        CS_TYPE2_DEBUG_STR.as_ptr(),
                    );
                }
                do_charstring(
                    dest,
                    limit,
                    &raw mut subr,
                    subr.offset(len as isize),
                    gsubr_idx,
                    subr_idx,
                );
                *data = (*data).offset(1 as ::core::ffi::c_int as isize);
            }
        } else if b0 as ::core::ffi::c_int == cs_escape {
            do_operator2(dest, limit, data, endptr);
        } else if (b0 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
            && b0 as ::core::ffi::c_int != 28 as ::core::ffi::c_int
        {
            do_operator1(dest, limit, data, endptr);
        } else if b0 as ::core::ffi::c_int >= 22 as ::core::ffi::c_int
            && b0 as ::core::ffi::c_int <= 27 as ::core::ffi::c_int
            || b0 as ::core::ffi::c_int == 31 as ::core::ffi::c_int
        {
            (*__pdf_state).status = CS_PARSE_ERROR;
        } else {
            get_integer(data, endptr);
        }
    }
    if (*__pdf_state).status == CS_SUBR_RETURN {
        (*__pdf_state).status = CS_PARSE_OK;
    } else if (*__pdf_state).status == CS_CHAR_END && *data < endptr {
        pdf_warning!(
            b"%s: Garbage after endchar.\0" as *const u8 as *const ::core::ffi::c_char,
            CS_TYPE2_DEBUG_STR.as_ptr(),
        );
    } else if (*__pdf_state).status < CS_PARSE_OK {
        pdf_abort!(
            b"%s: Parsing charstring failed: (status=%d, stack=%d)\0" as *const u8
                as *const ::core::ffi::c_char,
            CS_TYPE2_DEBUG_STR.as_ptr(),
            (*__pdf_state).status,
            (*__pdf_state).stack_top,
        );
    }
    (*__pdf_state).nest -= 1;
}
unsafe extern "C-unwind" fn cs_parse_init() {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).status = CS_PARSE_OK;
    (*__pdf_state).nest = 0 as ::core::ffi::c_int;
    (*__pdf_state).phase = 0 as ::core::ffi::c_int;
    (*__pdf_state).num_stems = 0 as ::core::ffi::c_int;
    (*__pdf_state).stack_top = 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_cs_copy_charstring")]
pub unsafe extern "C-unwind" fn cs_copy_charstring(
    mut dst: *mut card8,
    mut dstlen: ::core::ffi::c_int,
    mut src: *mut card8,
    mut srclen: ::core::ffi::c_int,
    mut gsubr: *mut cff_index,
    mut subr: *mut cff_index,
    mut default_width: ::core::ffi::c_double,
    mut nominal_width: ::core::ffi::c_double,
    mut ginfo: *mut cs_ginfo,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut save: *mut card8 = dst;
    cs_parse_init();
    (*__pdf_state).width = 0.0f64;
    (*__pdf_state).have_width = 0 as ::core::ffi::c_int;
    do_charstring(
        &raw mut dst,
        dst.offset(dstlen as isize),
        &raw mut src,
        src.offset(srclen as isize),
        gsubr,
        subr,
    );
    if !ginfo.is_null() {
        (*ginfo).flags = 0 as ::core::ffi::c_int;
        if (*__pdf_state).have_width != 0 {
            (*ginfo).wx = nominal_width + (*__pdf_state).width;
        } else {
            (*ginfo).wx = default_width;
        }
    }
    return dst.offset_from(save) as ::core::ffi::c_long as ::core::ffi::c_int;
}
unsafe extern "C" {
    fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    status: ::core::ffi::c_int,
    num_stems: ::core::ffi::c_int,
    phase: ::core::ffi::c_int,
    nest: ::core::ffi::c_int,
    have_width: ::core::ffi::c_int,
    width: ::core::ffi::c_double,
    stack_top: ::core::ffi::c_int,
    arg_stack: [::core::ffi::c_double; 48],
    trn_array: [::core::ffi::c_double; 32],
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        unsafe {
            Self {
                status: CS_PARSE_ERROR,
                num_stems: 0 as ::core::ffi::c_int,
                phase: 0 as ::core::ffi::c_int,
                nest: 0 as ::core::ffi::c_int,
                have_width: 0 as ::core::ffi::c_int,
                width: 0.0f64,
                stack_top: 0 as ::core::ffi::c_int,
                arg_stack: [0.; 48],
                trn_array: [0.; 32],
            }
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
