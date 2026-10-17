/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn fabs(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn qsort(
        __base: *mut ::core::ffi::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
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
pub struct t1_ginfo {
    pub use_seac: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_chardesc {
    pub flags: ::core::ffi::c_int,
    pub sbw: C2RustUnnamed_3,
    pub bbox: C2RustUnnamed_2,
    pub seac: C2RustUnnamed_1,
    pub num_stems: ::core::ffi::c_int,
    pub stems: [t1_stem; 96],
    pub charpath: *mut t1_cpath,
    pub lastpath: *mut t1_cpath,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_cpath {
    pub type_0: ::core::ffi::c_int,
    pub num_args: ::core::ffi::c_int,
    pub args: [::core::ffi::c_double; 48],
    pub next: *mut t1_cpath,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_stem {
    pub id: ::core::ffi::c_int,
    pub dir: ::core::ffi::c_int,
    pub pos: ::core::ffi::c_double,
    pub del: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub asb: ::core::ffi::c_double,
    pub adx: ::core::ffi::c_double,
    pub ady: ::core::ffi::c_double,
    pub bchar: card8,
    pub achar: card8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub llx: ::core::ffi::c_double,
    pub lly: ::core::ffi::c_double,
    pub urx: ::core::ffi::c_double,
    pub ury: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub sbx: ::core::ffi::c_double,
    pub sby: ::core::ffi::c_double,
    pub wx: ::core::ffi::c_double,
    pub wy: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct t1_stemgroup {
    pub num_stems: ::core::ffi::c_int,
    pub stems: [::core::ffi::c_double; 96],
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const CS_STEM_ZONE_MAX: ::core::ffi::c_int = 96 as ::core::ffi::c_int;
pub const CS_ARG_STACK_MAX: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const CS_SUBR_NEST_MAX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
pub const CS_BUFFER_ERROR: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const CS_STACK_ERROR: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const CS_PARSE_ERROR: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const CS_PARSE_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CS_SUBR_RETURN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const CS_CHAR_END: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const T1_CS_PHASE_INIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const T1_CS_PHASE_PATH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const T1_CS_PHASE_FLEX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;

pub const CS_STEM_GROUP_MAX: ::core::ffi::c_int = CS_STEM_ZONE_MAX;
pub const PS_ARG_STACK_MAX: ::core::ffi::c_int =
    CS_STEM_ZONE_MAX * 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
pub const HSTEM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VSTEM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const T1_CS_FLAG_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const T1_CS_FLAG_USE_HINTMASK: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const T1_CS_FLAG_USE_CNTRMASK: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const T1_CS_FLAG_USE_SEAC: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;

pub const CS_HINT_DECL: ::core::ffi::c_int = -1;
pub const CS_FLEX_CTRL: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const cs_hstem: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const cs_vstem: ::core::ffi::c_int = 3;
pub const cs_vmoveto: ::core::ffi::c_int = 4;
pub const cs_rlineto: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const cs_hlineto: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const cs_vlineto: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const cs_rrcurveto: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const cs_closepath: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const cs_callsubr: ::core::ffi::c_int = 10;
pub const cs_return: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const cs_escape: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const cs_hsbw: ::core::ffi::c_int = 13;
pub const cs_endchar: ::core::ffi::c_int = 14;
pub const cs_hstemhm: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const cs_hintmask: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const cs_cntrmask: ::core::ffi::c_int = 20;
pub const cs_rmoveto: ::core::ffi::c_int = 21;
pub const cs_hmoveto: ::core::ffi::c_int = 22;
pub const cs_vstemhm: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const cs_rcurveline: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const cs_rlinecurve: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const cs_callgsubr: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const cs_vhcurveto: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const cs_hvcurveto: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const cs_dotsection: ::core::ffi::c_int = 0;
pub const cs_vstem3: ::core::ffi::c_int = 1;
pub const cs_hstem3: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const cs_seac: ::core::ffi::c_int = 6;
pub const cs_sbw: ::core::ffi::c_int = 7;
pub const cs_div: ::core::ffi::c_int = 12;
pub const cs_callothersubr: ::core::ffi::c_int = 16;
pub const cs_pop: ::core::ffi::c_int = 17;
pub const cs_setcurrentpoint: ::core::ffi::c_int = 33;
pub const cs_hflex: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const cs_flex: ::core::ffi::c_int = 35;
pub const cs_hflex1: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn stem_compare(
    mut v1: *const ::core::ffi::c_void,
    mut v2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut cmp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s1: *const t1_stem = ::core::ptr::null::<t1_stem>();
    let mut s2: *const t1_stem = ::core::ptr::null::<t1_stem>();
    s1 = v1 as *const t1_stem;
    s2 = v2 as *const t1_stem;
    if (*s1).dir == (*s2).dir {
        if (*s1).pos == (*s2).pos {
            if (*s1).del == (*s2).del {
                cmp = 0 as ::core::ffi::c_int;
            } else {
                cmp = if (*s1).del < (*s2).del {
                    -(1 as ::core::ffi::c_int)
                } else {
                    1 as ::core::ffi::c_int
                };
            }
        } else {
            cmp = if (*s1).pos < (*s2).pos {
                -(1 as ::core::ffi::c_int)
            } else {
                1 as ::core::ffi::c_int
            };
        }
    } else {
        cmp = if (*s1).dir == HSTEM {
            -(1 as ::core::ffi::c_int)
        } else {
            1 as ::core::ffi::c_int
        };
    }
    return cmp;
}
unsafe extern "C-unwind" fn get_stem(
    mut cd: *mut t1_chardesc,
    mut stem_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*cd).num_stems {
        if (*cd).stems[i as usize].id == stem_id {
            break;
        }
        i += 1;
    }
    return if i < (*cd).num_stems {
        i
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
unsafe extern "C-unwind" fn add_stem(
    mut cd: *mut t1_chardesc,
    mut pos: ::core::ffi::c_double,
    mut del: ::core::ffi::c_double,
    mut dir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if cd.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cd\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pos += if dir == HSTEM {
        (*cd).sbw.sby
    } else {
        (*cd).sbw.sbx
    };
    i = 0 as ::core::ffi::c_int;
    while i < (*cd).num_stems {
        if (*cd).stems[i as usize].dir == dir
            && (*cd).stems[i as usize].pos == pos
            && (*cd).stems[i as usize].del == del
        {
            break;
        }
        i += 1;
    }
    if i == (*cd).num_stems {
        if (*cd).num_stems == CS_STEM_ZONE_MAX {
            return -(1 as ::core::ffi::c_int);
        }
        (*cd).stems[i as usize].dir = dir;
        (*cd).stems[i as usize].pos = pos;
        (*cd).stems[i as usize].del = del;
        (*cd).stems[i as usize].id = (*cd).num_stems;
        (*cd).num_stems += 1;
    }
    return (*cd).stems[i as usize].id;
}
unsafe extern "C-unwind" fn copy_args(
    mut args1: *mut ::core::ffi::c_double,
    mut args2: *mut ::core::ffi::c_double,
    mut count: ::core::ffi::c_int,
) {
    loop {
        let fresh0 = count;
        count = count - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        *args1 = *args2;
        args1 = args1.offset(1);
        args2 = args2.offset(1);
    }
}
unsafe extern "C-unwind" fn add_charpath(
    mut cd: *mut t1_chardesc,
    mut type_0: ::core::ffi::c_int,
    mut argv: *mut ::core::ffi::c_double,
    mut argn: ::core::ffi::c_int,
) {
    let __pdf_state = unsafe { document_state() };
    let mut p: *mut t1_cpath = ::core::ptr::null_mut::<t1_cpath>();
    if cd.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cd\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(argn <= 48 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"argn <= CS_ARG_STACK_MAX\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    p = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<t1_cpath>() as usize) as uint32_t)
        as *mut t1_cpath;
    (*p).type_0 = type_0;
    (*p).num_args = argn;
    (*p).next = ::core::ptr::null_mut::<t1_cpath>();
    loop {
        let fresh4 = argn;
        argn = argn - 1;
        if !(fresh4 > 0 as ::core::ffi::c_int) {
            break;
        }
        (*p).args[argn as usize] = *argv.offset(argn as isize);
    }
    if (*cd).charpath.is_null() {
        (*cd).charpath = p;
    }
    if !(*cd).lastpath.is_null() {
        (*(*cd).lastpath).next = p as *mut t1_cpath;
    }
    (*cd).lastpath = p;
    if type_0 >= 0 as ::core::ffi::c_int
        && (*__pdf_state).phase != T1_CS_PHASE_FLEX
        && (type_0 >= cs_vmoveto && type_0 <= cs_closepath
            || type_0 >= cs_rmoveto
                && type_0 <= cs_hvcurveto
                && type_0 != cs_vstemhm
                && type_0 != cs_callgsubr
                && type_0 != 28 as ::core::ffi::c_int)
    {
        (*__pdf_state).phase = T1_CS_PHASE_PATH;
    }
}
unsafe extern "C-unwind" fn init_charpath(mut cd: *mut t1_chardesc) {
    (*cd).flags = T1_CS_FLAG_NONE;
    (*cd).num_stems = 0 as ::core::ffi::c_int;
    (*cd).sbw.wy = 0.0f64;
    (*cd).sbw.wx = (*cd).sbw.wy;
    (*cd).sbw.sby = 0.0f64;
    (*cd).sbw.sbx = (*cd).sbw.sby;
    (*cd).bbox.ury = 0.0f64;
    (*cd).bbox.urx = (*cd).bbox.ury;
    (*cd).bbox.lly = (*cd).bbox.urx;
    (*cd).bbox.llx = (*cd).bbox.lly;
    (*cd).lastpath = ::core::ptr::null_mut::<t1_cpath>();
    (*cd).charpath = (*cd).lastpath;
}
unsafe extern "C-unwind" fn release_charpath(mut cd: *mut t1_chardesc) {
    let mut curr: *mut t1_cpath = ::core::ptr::null_mut::<t1_cpath>();
    let mut next: *mut t1_cpath = ::core::ptr::null_mut::<t1_cpath>();
    if cd.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cd\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    curr = (*cd).charpath;
    while !curr.is_null() {
        next = (*curr).next as *mut t1_cpath;
        free(curr as *mut ::core::ffi::c_void);
        curr = next;
    }
    (*cd).lastpath = ::core::ptr::null_mut::<t1_cpath>();
    (*cd).charpath = (*cd).lastpath;
}
unsafe extern "C-unwind" fn do_operator1(mut cd: *mut t1_chardesc, mut data: *mut *mut card8) {
    let __pdf_state = unsafe { document_state() };
    let mut op: card8 = **data;
    *data = (*data).offset(1 as ::core::ffi::c_int as isize);
    match op as ::core::ffi::c_int {
        cs_closepath => {
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_hsbw => {
            if (*__pdf_state).cs_stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).cs_stack_top -= 1;
            (*cd).sbw.wx = (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize];
            (*cd).sbw.wy = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            (*__pdf_state).cs_stack_top -= 1;
            (*cd).sbw.sbx = (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize];
            (*cd).sbw.sby = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_hstem | cs_vstem => {
            if (*__pdf_state).cs_stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            let mut stem_id: ::core::ffi::c_int = 0;
            stem_id = add_stem(
                cd,
                (*__pdf_state).cs_arg_stack
                    [((*__pdf_state).cs_stack_top - 2 as ::core::ffi::c_int) as usize],
                (*__pdf_state).cs_arg_stack
                    [((*__pdf_state).cs_stack_top - 1 as ::core::ffi::c_int) as usize],
                if op as ::core::ffi::c_int == cs_hstem {
                    HSTEM
                } else {
                    VSTEM
                },
            );
            if stem_id < 0 as ::core::ffi::c_int {
                pdf_warning!(b"Too many hints...\0" as *const u8 as *const ::core::ffi::c_char,);
                (*__pdf_state).status = CS_PARSE_ERROR;
                return;
            }
            let fresh2 = (*__pdf_state).cs_stack_top;
            (*__pdf_state).cs_stack_top = (*__pdf_state).cs_stack_top + 1;
            (*__pdf_state).cs_arg_stack[fresh2 as usize] = stem_id as ::core::ffi::c_double;
            add_charpath(
                cd,
                -(1 as ::core::ffi::c_int),
                (&raw mut (*__pdf_state).cs_arg_stack as *mut ::core::ffi::c_double)
                    .offset(((*__pdf_state).cs_stack_top - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_rmoveto => {
            if (*__pdf_state).cs_stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            if (*__pdf_state).phase < T1_CS_PHASE_PATH {
                (*__pdf_state).cs_arg_stack
                    [((*__pdf_state).cs_stack_top - 2 as ::core::ffi::c_int) as usize] +=
                    (*cd).sbw.sbx;
                (*__pdf_state).cs_arg_stack
                    [((*__pdf_state).cs_stack_top - 1 as ::core::ffi::c_int) as usize] +=
                    (*cd).sbw.sby;
            }
            add_charpath(
                cd,
                op as ::core::ffi::c_int,
                (&raw mut (*__pdf_state).cs_arg_stack as *mut ::core::ffi::c_double)
                    .offset(((*__pdf_state).cs_stack_top - 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_hmoveto | cs_vmoveto => {
            if (*__pdf_state).cs_stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            let mut argn: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            if (*__pdf_state).phase < T1_CS_PHASE_PATH {
                if op as ::core::ffi::c_int == cs_hmoveto {
                    (*__pdf_state).cs_arg_stack
                        [((*__pdf_state).cs_stack_top - 1 as ::core::ffi::c_int) as usize] +=
                        (*cd).sbw.sbx;
                    if (*cd).sbw.sby != 0.0f64 {
                        let fresh3 = (*__pdf_state).cs_stack_top;
                        (*__pdf_state).cs_stack_top = (*__pdf_state).cs_stack_top + 1;
                        (*__pdf_state).cs_arg_stack[fresh3 as usize] = (*cd).sbw.sby;
                        argn = 2 as ::core::ffi::c_int;
                        op = cs_rmoveto as card8;
                    }
                } else {
                    (*__pdf_state).cs_arg_stack
                        [((*__pdf_state).cs_stack_top - 1 as ::core::ffi::c_int) as usize] +=
                        (*cd).sbw.sby;
                    if (*cd).sbw.sbx != 0.0f64 {
                        (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize] =
                            (*__pdf_state).cs_arg_stack
                                [((*__pdf_state).cs_stack_top - 1 as ::core::ffi::c_int) as usize];
                        (*__pdf_state).cs_arg_stack
                            [((*__pdf_state).cs_stack_top - 1 as ::core::ffi::c_int) as usize] =
                            (*cd).sbw.sbx;
                        (*__pdf_state).cs_stack_top += 1;
                        argn = 2 as ::core::ffi::c_int;
                        op = cs_rmoveto as card8;
                    }
                }
            }
            add_charpath(
                cd,
                op as ::core::ffi::c_int,
                (&raw mut (*__pdf_state).cs_arg_stack as *mut ::core::ffi::c_double)
                    .offset(((*__pdf_state).cs_stack_top - argn) as isize)
                    as *mut ::core::ffi::c_double,
                argn,
            );
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_endchar => {
            (*__pdf_state).status = CS_CHAR_END;
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_rlineto => {
            if (*__pdf_state).cs_stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            add_charpath(
                cd,
                op as ::core::ffi::c_int,
                (&raw mut (*__pdf_state).cs_arg_stack as *mut ::core::ffi::c_double)
                    .offset(((*__pdf_state).cs_stack_top - 2 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_hlineto | cs_vlineto => {
            if (*__pdf_state).cs_stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            add_charpath(
                cd,
                op as ::core::ffi::c_int,
                (&raw mut (*__pdf_state).cs_arg_stack as *mut ::core::ffi::c_double)
                    .offset(((*__pdf_state).cs_stack_top - 1 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
                1 as ::core::ffi::c_int,
            );
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_rrcurveto => {
            if (*__pdf_state).cs_stack_top < 6 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            add_charpath(
                cd,
                op as ::core::ffi::c_int,
                (&raw mut (*__pdf_state).cs_arg_stack as *mut ::core::ffi::c_double)
                    .offset(((*__pdf_state).cs_stack_top - 6 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
                6 as ::core::ffi::c_int,
            );
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_vhcurveto | cs_hvcurveto => {
            if (*__pdf_state).cs_stack_top < 4 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            add_charpath(
                cd,
                op as ::core::ffi::c_int,
                (&raw mut (*__pdf_state).cs_arg_stack as *mut ::core::ffi::c_double)
                    .offset(((*__pdf_state).cs_stack_top - 4 as ::core::ffi::c_int) as isize)
                    as *mut ::core::ffi::c_double,
                4 as ::core::ffi::c_int,
            );
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_return => {}
        cs_callsubr => {
            pdf_abort!(b"Unexpected callsubr.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        _ => {
            pdf_warning!(
                b"Unknown charstring operator: 0x%02x\0" as *const u8 as *const ::core::ffi::c_char,
                op as ::core::ffi::c_int,
            );
            (*__pdf_state).status = CS_PARSE_ERROR;
        }
    };
}
unsafe extern "C-unwind" fn do_othersubr0(mut cd: *mut t1_chardesc) {
    let __pdf_state = unsafe { document_state() };
    let mut flex: *mut t1_cpath = ::core::ptr::null_mut::<t1_cpath>();
    let mut cur: *mut t1_cpath = ::core::ptr::null_mut::<t1_cpath>();
    let mut next: *mut t1_cpath = ::core::ptr::null_mut::<t1_cpath>();
    if (*__pdf_state).ps_stack_top < 1 as ::core::ffi::c_int {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
    cur = (*cd).charpath;
    while !cur.is_null() && (*cur).type_0 != CS_FLEX_CTRL {
        cur = (*cur).next as *mut t1_cpath;
    }
    flex = cur;
    let mut i: ::core::ffi::c_uint = 0;
    cur = (*cur).next as *mut t1_cpath;
    i = 1 as ::core::ffi::c_uint;
    while i < 7 as ::core::ffi::c_uint {
        if cur.is_null()
            || (*cur).type_0 != CS_FLEX_CTRL
            || (*cur).num_args != 2 as ::core::ffi::c_int
        {
            (*__pdf_state).status = CS_PARSE_ERROR;
            return;
        }
        if i == 1 as ::core::ffi::c_uint {
            (*flex).args[0 as ::core::ffi::c_int as usize] +=
                (*cur).args[0 as ::core::ffi::c_int as usize];
            (*flex).args[1 as ::core::ffi::c_int as usize] +=
                (*cur).args[1 as ::core::ffi::c_int as usize];
        } else {
            copy_args(
                (&raw mut (*flex).args as *mut ::core::ffi::c_double).offset(
                    (2 as ::core::ffi::c_uint)
                        .wrapping_mul(i)
                        .wrapping_sub(2 as ::core::ffi::c_uint) as isize,
                ) as *mut ::core::ffi::c_double,
                &raw mut (*cur).args as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
            );
        }
        next = (*cur).next as *mut t1_cpath;
        free(cur as *mut ::core::ffi::c_void);
        cur = next;
        i = i.wrapping_add(1);
    }
    if !cur.is_null() {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
    (*flex).type_0 = cs_flex;
    (*__pdf_state).ps_stack_top -= 1;
    (*flex).args[12 as ::core::ffi::c_int as usize] =
        (*__pdf_state).ps_arg_stack[(*__pdf_state).ps_stack_top as usize];
    (*flex).num_args = 13 as ::core::ffi::c_int;
    (*flex).next = ::core::ptr::null_mut::<t1_cpath>();
    (*cd).lastpath = flex;
    (*__pdf_state).phase = T1_CS_PHASE_PATH;
}
unsafe extern "C-unwind" fn do_othersubr1() {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).phase = T1_CS_PHASE_FLEX;
}
unsafe extern "C-unwind" fn do_othersubr2(mut cd: *mut t1_chardesc) {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).phase != T1_CS_PHASE_FLEX || (*cd).lastpath.is_null() {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
    match (*(*cd).lastpath).type_0 {
        cs_rmoveto => {}
        cs_hmoveto => {
            (*(*cd).lastpath).num_args = 2 as ::core::ffi::c_int;
            (*(*cd).lastpath).args[1 as ::core::ffi::c_int as usize] = 0.0f64;
        }
        cs_vmoveto => {
            (*(*cd).lastpath).num_args = 2 as ::core::ffi::c_int;
            (*(*cd).lastpath).args[1 as ::core::ffi::c_int as usize] =
                (*(*cd).lastpath).args[0 as ::core::ffi::c_int as usize];
            (*(*cd).lastpath).args[0 as ::core::ffi::c_int as usize] = 0.0f64;
        }
        _ => {
            (*__pdf_state).status = CS_PARSE_ERROR;
            return;
        }
    }
    (*(*cd).lastpath).type_0 = CS_FLEX_CTRL;
}
unsafe extern "C-unwind" fn do_othersubr3(mut cd: *mut t1_chardesc) {
    (*cd).flags |= T1_CS_FLAG_USE_HINTMASK;
}
unsafe extern "C-unwind" fn do_othersubr12() {
    let __pdf_state = unsafe { document_state() };
    if (*__pdf_state).phase != T1_CS_PHASE_INIT {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
}
unsafe extern "C-unwind" fn do_othersubr13(mut cd: *mut t1_chardesc) {
    let __pdf_state = unsafe { document_state() };
    let mut stemgroups: [t1_stemgroup; 96] = [t1_stemgroup {
        num_stems: 0,
        stems: [0.; 96],
    }; 96];
    let mut num_hgroups: ::core::ffi::c_int = 0;
    let mut num_vgroups: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut stem_id: ::core::ffi::c_int = 0;
    let mut pos: ::core::ffi::c_double = 0.;
    let mut del: ::core::ffi::c_double = 0.;
    if (*__pdf_state).phase != T1_CS_PHASE_INIT {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
    n = 0 as ::core::ffi::c_int;
    while n < CS_STEM_GROUP_MAX {
        stemgroups[n as usize].num_stems = 0 as ::core::ffi::c_int;
        n += 1;
    }
    (*__pdf_state).ps_stack_top -= 1;
    num_hgroups =
        (*__pdf_state).ps_arg_stack[(*__pdf_state).ps_stack_top as usize] as ::core::ffi::c_int;
    if num_hgroups < 0 as ::core::ffi::c_int || num_hgroups > CS_STEM_GROUP_MAX {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
    n = 0 as ::core::ffi::c_int;
    pos = 0.0f64;
    while (*__pdf_state).ps_stack_top >= 2 as ::core::ffi::c_int && n < num_hgroups {
        (*__pdf_state).ps_stack_top -= 1;
        pos += (*__pdf_state).ps_arg_stack[(*__pdf_state).ps_stack_top as usize];
        (*__pdf_state).ps_stack_top -= 1;
        del = (*__pdf_state).ps_arg_stack[(*__pdf_state).ps_stack_top as usize];
        stem_id = add_stem(
            cd,
            if del < 0.0f64 { pos + del } else { pos },
            if del < 0.0f64 { -del } else { del },
            HSTEM,
        );
        stemgroups[n as usize].stems[stemgroups[n as usize].num_stems as usize] =
            stem_id as ::core::ffi::c_double;
        stemgroups[n as usize].num_stems += 1 as ::core::ffi::c_int;
        pos += del;
        if del < 0.0f64 {
            pos = 0.0f64;
            n += 1;
        }
    }
    if n != num_hgroups {
        (*__pdf_state).status = CS_STACK_ERROR;
        return;
    }
    (*__pdf_state).ps_stack_top -= 1;
    num_vgroups =
        (*__pdf_state).ps_arg_stack[(*__pdf_state).ps_stack_top as usize] as ::core::ffi::c_int;
    if num_vgroups < 0 as ::core::ffi::c_int || num_vgroups > CS_STEM_GROUP_MAX {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
    n = 0 as ::core::ffi::c_int;
    pos = 0.0f64;
    while (*__pdf_state).ps_stack_top >= 2 as ::core::ffi::c_int && n < num_vgroups {
        (*__pdf_state).ps_stack_top -= 1;
        pos += (*__pdf_state).ps_arg_stack[(*__pdf_state).ps_stack_top as usize];
        (*__pdf_state).ps_stack_top -= 1;
        del = (*__pdf_state).ps_arg_stack[(*__pdf_state).ps_stack_top as usize];
        stem_id = add_stem(
            cd,
            if del < 0.0f64 { pos + del } else { pos },
            if del < 0.0f64 { -del } else { del },
            VSTEM,
        );
        stemgroups[n as usize].stems[stemgroups[n as usize].num_stems as usize] =
            stem_id as ::core::ffi::c_double;
        stemgroups[n as usize].num_stems += 1 as ::core::ffi::c_int;
        pos += del;
        if del < 0.0f64 {
            pos = 0.0f64;
            n += 1;
        }
    }
    if n != num_vgroups {
        (*__pdf_state).status = CS_STACK_ERROR;
        return;
    }
    n = 0 as ::core::ffi::c_int;
    while n
        < (if num_hgroups > num_vgroups {
            num_hgroups
        } else {
            num_vgroups
        })
    {
        add_charpath(
            cd,
            cs_cntrmask,
            &raw mut (*(&raw mut stemgroups as *mut t1_stemgroup).offset(n as isize)).stems
                as *mut ::core::ffi::c_double,
            stemgroups[n as usize].num_stems,
        );
        n += 1;
    }
    (*cd).flags |= T1_CS_FLAG_USE_CNTRMASK;
}
unsafe extern "C-unwind" fn do_callothersubr(mut cd: *mut t1_chardesc) {
    let __pdf_state = unsafe { document_state() };
    let mut argn: ::core::ffi::c_int = 0;
    let mut subrno: ::core::ffi::c_int = 0;
    if (*__pdf_state).cs_stack_top < 2 as ::core::ffi::c_int {
        (*__pdf_state).status = CS_STACK_ERROR;
        return;
    }
    (*__pdf_state).cs_stack_top -= 1;
    subrno =
        (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize] as ::core::ffi::c_int;
    (*__pdf_state).cs_stack_top -= 1;
    argn = (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize] as ::core::ffi::c_int;
    if (*__pdf_state).cs_stack_top < argn {
        (*__pdf_state).status = CS_STACK_ERROR;
        return;
    }
    if (*__pdf_state).ps_stack_top + argn > PS_ARG_STACK_MAX {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
    loop {
        let fresh7 = argn;
        argn = argn - 1;
        if !(fresh7 > 0 as ::core::ffi::c_int) {
            break;
        }
        (*__pdf_state).cs_stack_top -= 1;
        let fresh8 = (*__pdf_state).ps_stack_top;
        (*__pdf_state).ps_stack_top = (*__pdf_state).ps_stack_top + 1;
        (*__pdf_state).ps_arg_stack[fresh8 as usize] =
            (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize];
    }
    match subrno {
        0 => {
            do_othersubr0(cd);
        }
        1 => {
            do_othersubr1();
        }
        2 => {
            do_othersubr2(cd);
        }
        3 => {
            do_othersubr3(cd);
        }
        12 => {
            do_othersubr12();
        }
        13 => {
            do_othersubr13(cd);
        }
        _ => {
            pdf_abort!(
                b"Unknown othersubr #%d.\0" as *const u8 as *const ::core::ffi::c_char,
                subrno,
            );
        }
    };
}
unsafe extern "C-unwind" fn do_operator2(
    mut cd: *mut t1_chardesc,
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
        cs_sbw => {
            if (*__pdf_state).cs_stack_top < 4 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).cs_stack_top -= 1;
            (*cd).sbw.wy = (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize];
            (*__pdf_state).cs_stack_top -= 1;
            (*cd).sbw.wx = (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize];
            (*__pdf_state).cs_stack_top -= 1;
            (*cd).sbw.sby = (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize];
            (*__pdf_state).cs_stack_top -= 1;
            (*cd).sbw.sbx = (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize];
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_hstem3 | cs_vstem3 => {
            if (*__pdf_state).cs_stack_top < 6 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            let mut i: ::core::ffi::c_int = 0;
            i = 2 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                let mut stem_id: ::core::ffi::c_int = 0;
                stem_id = add_stem(
                    cd,
                    (*__pdf_state).cs_arg_stack[((*__pdf_state).cs_stack_top
                        - 2 as ::core::ffi::c_int * i
                        - 2 as ::core::ffi::c_int)
                        as usize],
                    (*__pdf_state).cs_arg_stack[((*__pdf_state).cs_stack_top
                        - 2 as ::core::ffi::c_int * i
                        - 1 as ::core::ffi::c_int)
                        as usize],
                    if op as ::core::ffi::c_int == cs_hstem3 {
                        HSTEM
                    } else {
                        VSTEM
                    },
                );
                if stem_id < 0 as ::core::ffi::c_int {
                    pdf_warning!(b"Too many hints...\0" as *const u8 as *const ::core::ffi::c_char,);
                    (*__pdf_state).status = CS_PARSE_ERROR;
                    return;
                }
                let fresh5 = (*__pdf_state).cs_stack_top;
                (*__pdf_state).cs_stack_top = (*__pdf_state).cs_stack_top + 1;
                (*__pdf_state).cs_arg_stack[fresh5 as usize] = stem_id as ::core::ffi::c_double;
                add_charpath(
                    cd,
                    -(1 as ::core::ffi::c_int),
                    (&raw mut (*__pdf_state).cs_arg_stack as *mut ::core::ffi::c_double)
                        .offset(((*__pdf_state).cs_stack_top - 1 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_double,
                    1 as ::core::ffi::c_int,
                );
                (*__pdf_state).cs_stack_top -= 1;
                i -= 1;
            }
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_setcurrentpoint => {
            if (*__pdf_state).cs_stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        cs_pop => {
            if (*__pdf_state).ps_stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_PARSE_ERROR;
                return;
            }
            if (*__pdf_state).cs_stack_top + 1 as ::core::ffi::c_int > CS_ARG_STACK_MAX {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).ps_stack_top -= 1;
            let fresh6 = (*__pdf_state).cs_stack_top;
            (*__pdf_state).cs_stack_top = (*__pdf_state).cs_stack_top + 1;
            (*__pdf_state).cs_arg_stack[fresh6 as usize] =
                (*__pdf_state).ps_arg_stack[(*__pdf_state).ps_stack_top as usize];
        }
        cs_dotsection => {}
        cs_div => {
            if (*__pdf_state).cs_stack_top < 2 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*__pdf_state).cs_arg_stack
                [((*__pdf_state).cs_stack_top - 2 as ::core::ffi::c_int) as usize] /=
                (*__pdf_state).cs_arg_stack
                    [((*__pdf_state).cs_stack_top - 1 as ::core::ffi::c_int) as usize];
            (*__pdf_state).cs_stack_top -= 1;
        }
        cs_callothersubr => {
            do_callothersubr(cd);
        }
        cs_seac => {
            if (*__pdf_state).cs_stack_top < 5 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
                return;
            }
            (*cd).flags |= T1_CS_FLAG_USE_SEAC;
            (*__pdf_state).cs_stack_top -= 1;
            (*cd).seac.achar =
                (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize] as card8;
            (*__pdf_state).cs_stack_top -= 1;
            (*cd).seac.bchar =
                (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize] as card8;
            (*__pdf_state).cs_stack_top -= 1;
            (*cd).seac.ady = (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize];
            (*__pdf_state).cs_stack_top -= 1;
            (*cd).seac.adx = (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize];
            (*cd).seac.ady += (*cd).sbw.sby;
            (*__pdf_state).cs_stack_top -= 1;
            (*cd).seac.adx +=
                (*cd).sbw.sbx - (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize];
            (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
        }
        _ => {
            pdf_warning!(
                b"Unknown charstring operator: 0x0c%02x\0" as *const u8
                    as *const ::core::ffi::c_char,
                op as ::core::ffi::c_int,
            );
            (*__pdf_state).status = CS_PARSE_ERROR;
        }
    };
}
unsafe extern "C-unwind" fn put_numbers(
    mut argv: *mut ::core::ffi::c_double,
    mut argn: ::core::ffi::c_int,
    mut dest: *mut *mut card8,
    mut limit: *mut card8,
) {
    let __pdf_state = unsafe { document_state() };
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < argn {
        let mut value: ::core::ffi::c_double = 0.;
        let mut ivalue: ::core::ffi::c_int = 0;
        value = *argv.offset(i as isize);
        ivalue = floor(value + 0.5f64) as ::core::ffi::c_int;
        if value >= 0x8000 as ::core::ffi::c_long as ::core::ffi::c_double
            || value
                <= (-(0x8000 as ::core::ffi::c_long) - 1 as ::core::ffi::c_long)
                    as ::core::ffi::c_double
        {
            pdf_abort!(
                b"Argument value too large. (This is bug)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        } else if fabs(value - ivalue as ::core::ffi::c_double) > 3.0e-5f64 {
            if limit < (*dest).offset(5 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh21 = *dest;
            *dest = (*dest).offset(1);
            *fresh21 = 255 as card8;
            ivalue = floor(value) as ::core::ffi::c_int;
            let fresh22 = *dest;
            *dest = (*dest).offset(1);
            *fresh22 = (ivalue >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh23 = *dest;
            *dest = (*dest).offset(1);
            *fresh23 = (ivalue & 0xff as ::core::ffi::c_int) as card8;
            ivalue = ((value - ivalue as ::core::ffi::c_double)
                * 0x10000 as ::core::ffi::c_long as ::core::ffi::c_double)
                as ::core::ffi::c_int;
            let fresh24 = *dest;
            *dest = (*dest).offset(1);
            *fresh24 = (ivalue >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh25 = *dest;
            *dest = (*dest).offset(1);
            *fresh25 = (ivalue & 0xff as ::core::ffi::c_int) as card8;
        } else if ivalue >= -(107 as ::core::ffi::c_int) && ivalue <= 107 as ::core::ffi::c_int {
            if limit < (*dest).offset(1 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh26 = *dest;
            *dest = (*dest).offset(1);
            *fresh26 = (ivalue + 139 as ::core::ffi::c_int) as card8;
        } else if ivalue >= 108 as ::core::ffi::c_int && ivalue <= 1131 as ::core::ffi::c_int {
            if limit < (*dest).offset(2 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            ivalue = (0xf700 as ::core::ffi::c_uint)
                .wrapping_add(ivalue as ::core::ffi::c_uint)
                .wrapping_sub(108 as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
            let fresh27 = *dest;
            *dest = (*dest).offset(1);
            *fresh27 = (ivalue >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh28 = *dest;
            *dest = (*dest).offset(1);
            *fresh28 = (ivalue & 0xff as ::core::ffi::c_int) as card8;
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
            let fresh29 = *dest;
            *dest = (*dest).offset(1);
            *fresh29 = (ivalue >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh30 = *dest;
            *dest = (*dest).offset(1);
            *fresh30 = (ivalue & 0xff as ::core::ffi::c_int) as card8;
        } else if ivalue >= -(32768 as ::core::ffi::c_int) && ivalue <= 32767 as ::core::ffi::c_int
        {
            if limit < (*dest).offset(3 as ::core::ffi::c_int as isize) {
                (*__pdf_state).status = CS_BUFFER_ERROR;
                return;
            }
            let fresh31 = *dest;
            *dest = (*dest).offset(1);
            *fresh31 = 28 as card8;
            let fresh32 = *dest;
            *dest = (*dest).offset(1);
            *fresh32 = (ivalue >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as card8;
            let fresh33 = *dest;
            *dest = (*dest).offset(1);
            *fresh33 = (ivalue & 0xff as ::core::ffi::c_int) as card8;
        } else {
            pdf_abort!(b"Unexpected error.\0" as *const u8 as *const ::core::ffi::c_char);
        }
        i += 1;
    }
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
    if (*__pdf_state).cs_stack_top + 1 as ::core::ffi::c_int > CS_ARG_STACK_MAX {
        (*__pdf_state).status = CS_STACK_ERROR;
        return;
    }
    let fresh1 = (*__pdf_state).cs_stack_top;
    (*__pdf_state).cs_stack_top = (*__pdf_state).cs_stack_top + 1;
    (*__pdf_state).cs_arg_stack[fresh1 as usize] = result as ::core::ffi::c_double;
}
unsafe extern "C-unwind" fn get_longint(mut data: *mut *mut card8, mut endptr: *mut card8) {
    let __pdf_state = unsafe { document_state() };
    let mut result: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_uint = 0;
    *data = (*data).offset(1 as ::core::ffi::c_int as isize);
    if endptr < (*data).offset(4 as ::core::ffi::c_int as isize) {
        (*__pdf_state).status = CS_PARSE_ERROR;
        return;
    }
    result = **data as ::core::ffi::c_int;
    if result as ::core::ffi::c_long >= 0x80 as ::core::ffi::c_long {
        result =
            (result as ::core::ffi::c_long - 0x100 as ::core::ffi::c_long) as ::core::ffi::c_int;
    }
    *data = (*data).offset(1 as ::core::ffi::c_int as isize);
    i = 1 as ::core::ffi::c_uint;
    while i < 4 as ::core::ffi::c_uint {
        result = result * 256 as ::core::ffi::c_int + **data as ::core::ffi::c_int;
        *data = (*data).offset(1 as ::core::ffi::c_int as isize);
        i = i.wrapping_add(1);
    }
    if (*__pdf_state).cs_stack_top + 1 as ::core::ffi::c_int > CS_ARG_STACK_MAX {
        (*__pdf_state).status = CS_STACK_ERROR;
        return;
    }
    let fresh9 = (*__pdf_state).cs_stack_top;
    (*__pdf_state).cs_stack_top = (*__pdf_state).cs_stack_top + 1;
    (*__pdf_state).cs_arg_stack[fresh9 as usize] = result as ::core::ffi::c_double;
}
unsafe extern "C-unwind" fn t1char_build_charpath(
    mut cd: *mut t1_chardesc,
    mut data: *mut *mut card8,
    mut endptr: *mut card8,
    mut subrs: *mut cff_index,
) {
    let __pdf_state = unsafe { document_state() };
    let mut b0: card8 = 0 as card8;
    let mut subr: *mut card8 = ::core::ptr::null_mut::<card8>();
    let mut len: ::core::ffi::c_int = 0;
    if (*__pdf_state).nest > CS_SUBR_NEST_MAX {
        pdf_abort!(b"Subroutine nested too deeply.\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*__pdf_state).nest += 1;
    while *data < endptr && (*__pdf_state).status == CS_PARSE_OK {
        b0 = **data;
        if b0 as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
            get_longint(data, endptr);
        } else if b0 as ::core::ffi::c_int == cs_return {
            (*__pdf_state).status = CS_SUBR_RETURN;
        } else if b0 as ::core::ffi::c_int == cs_callsubr {
            if (*__pdf_state).cs_stack_top < 1 as ::core::ffi::c_int {
                (*__pdf_state).status = CS_STACK_ERROR;
            } else {
                let mut idx: ::core::ffi::c_int = 0;
                (*__pdf_state).cs_stack_top -= 1;
                idx = (*__pdf_state).cs_arg_stack[(*__pdf_state).cs_stack_top as usize]
                    as ::core::ffi::c_int;
                if subrs.is_null() || idx >= (*subrs).count as ::core::ffi::c_int {
                    pdf_abort!(b"Invalid Subr#.\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                subr = (*subrs)
                    .data
                    .offset(*(*subrs).offset.offset(idx as isize) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize));
                len = (*(*subrs)
                    .offset
                    .offset((idx + 1 as ::core::ffi::c_int) as isize))
                .wrapping_sub(*(*subrs).offset.offset(idx as isize))
                    as ::core::ffi::c_int;
                t1char_build_charpath(cd, &raw mut subr, subr.offset(len as isize), subrs);
                *data = (*data).offset(1 as ::core::ffi::c_int as isize);
            }
        } else if b0 as ::core::ffi::c_int == cs_escape {
            do_operator2(cd, data, endptr);
        } else if (b0 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
            && b0 as ::core::ffi::c_int != 28 as ::core::ffi::c_int
        {
            do_operator1(cd, data);
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
        if !(*data == endptr.offset(-(1 as ::core::ffi::c_int as isize))
            && **data as ::core::ffi::c_int == cs_return)
        {
            pdf_warning!(
                b"Garbage after endchar. (%d bytes)\0" as *const u8 as *const ::core::ffi::c_char,
                endptr.offset_from(*data) as ::core::ffi::c_long as ::core::ffi::c_int,
            );
        }
    } else if (*__pdf_state).status < CS_PARSE_OK {
        pdf_abort!(
            b"Parsing charstring failed: (status=%d, stack=%d)\0" as *const u8
                as *const ::core::ffi::c_char,
            (*__pdf_state).status,
            (*__pdf_state).cs_stack_top,
        );
    }
    (*__pdf_state).nest -= 1;
}
unsafe extern "C-unwind" fn do_postproc(mut cd: *mut t1_chardesc) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut cur: *mut t1_cpath = ::core::ptr::null_mut::<t1_cpath>();
    let mut prev: *mut t1_cpath = ::core::ptr::null_mut::<t1_cpath>();
    let mut next: *mut t1_cpath = ::core::ptr::null_mut::<t1_cpath>();
    let mut x: ::core::ffi::c_double = 0.;
    let mut y: ::core::ffi::c_double = 0.;
    if (*cd).charpath.is_null() {
        return;
    }
    (*cd).bbox.lly = 100000.0f64;
    (*cd).bbox.llx = (*cd).bbox.lly;
    (*cd).bbox.ury = -100000.0f64;
    (*cd).bbox.urx = (*cd).bbox.ury;
    cur = (*cd).charpath;
    prev = ::core::ptr::null_mut::<t1_cpath>();
    y = 0.0f64;
    x = y;
    while !cur.is_null() {
        next = (*cur).next as *mut t1_cpath;
        match (*cur).type_0 {
            cs_rmoveto => {
                x += (*cur).args[0 as ::core::ffi::c_int as usize];
                y += (*cur).args[1 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
            }
            cs_rlineto => {
                x += (*cur).args[0 as ::core::ffi::c_int as usize];
                y += (*cur).args[1 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
                if !prev.is_null()
                    && !cur.is_null()
                    && (*prev).num_args + (*cur).num_args < CS_ARG_STACK_MAX
                {
                    if (*prev).type_0 == cs_rlineto {
                        copy_args(
                            (&raw mut (*prev).args as *mut ::core::ffi::c_double)
                                .offset((*prev).num_args as isize),
                            &raw mut (*cur).args as *mut ::core::ffi::c_double,
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next as *mut t1_cpath;
                        cur = mfree(cur as *mut ::core::ffi::c_void) as *mut t1_cpath;
                    } else if (*prev).type_0 == cs_rrcurveto {
                        copy_args(
                            (&raw mut (*prev).args as *mut ::core::ffi::c_double)
                                .offset((*prev).num_args as isize),
                            &raw mut (*cur).args as *mut ::core::ffi::c_double,
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).type_0 = cs_rcurveline;
                        (*prev).next = next as *mut t1_cpath;
                        cur = mfree(cur as *mut ::core::ffi::c_void) as *mut t1_cpath;
                    }
                }
            }
            cs_hmoveto => {
                x += (*cur).args[0 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
            }
            cs_hlineto => {
                x += (*cur).args[0 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
                if !prev.is_null()
                    && !cur.is_null()
                    && (*prev).num_args + (*cur).num_args < CS_ARG_STACK_MAX
                {
                    if (*prev).type_0 == cs_vlineto
                        && (*prev).num_args % 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                        || (*prev).type_0 == cs_hlineto
                            && (*prev).num_args % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        copy_args(
                            (&raw mut (*prev).args as *mut ::core::ffi::c_double)
                                .offset((*prev).num_args as isize),
                            &raw mut (*cur).args as *mut ::core::ffi::c_double,
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next as *mut t1_cpath;
                        cur = mfree(cur as *mut ::core::ffi::c_void) as *mut t1_cpath;
                    }
                }
            }
            cs_vmoveto => {
                y += (*cur).args[0 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
            }
            cs_vlineto => {
                y += (*cur).args[0 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
                if !prev.is_null()
                    && !cur.is_null()
                    && (*prev).num_args + (*cur).num_args < CS_ARG_STACK_MAX
                {
                    if (*prev).type_0 == cs_hlineto
                        && (*prev).num_args % 2 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                        || (*prev).type_0 == cs_vlineto
                            && (*prev).num_args % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        copy_args(
                            (&raw mut (*prev).args as *mut ::core::ffi::c_double)
                                .offset((*prev).num_args as isize),
                            &raw mut (*cur).args as *mut ::core::ffi::c_double,
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next as *mut t1_cpath;
                        cur = mfree(cur as *mut ::core::ffi::c_void) as *mut t1_cpath;
                    }
                }
            }
            cs_rrcurveto => {
                i = 0 as ::core::ffi::c_uint;
                while i < 3 as ::core::ffi::c_uint {
                    x += (*cur).args[(2 as ::core::ffi::c_uint).wrapping_mul(i) as usize];
                    y += (*cur).args[(2 as ::core::ffi::c_uint)
                        .wrapping_mul(i)
                        .wrapping_add(1 as ::core::ffi::c_uint)
                        as usize];
                    if (*cd).bbox.llx > x {
                        (*cd).bbox.llx = x;
                    }
                    if (*cd).bbox.urx < x {
                        (*cd).bbox.urx = x;
                    }
                    if (*cd).bbox.lly > y {
                        (*cd).bbox.lly = y;
                    }
                    if (*cd).bbox.ury < y {
                        (*cd).bbox.ury = y;
                    }
                    i = i.wrapping_add(1);
                }
                if !prev.is_null()
                    && !cur.is_null()
                    && (*prev).num_args + (*cur).num_args < CS_ARG_STACK_MAX
                {
                    if (*prev).type_0 == cs_rrcurveto {
                        copy_args(
                            (&raw mut (*prev).args as *mut ::core::ffi::c_double)
                                .offset((*prev).num_args as isize),
                            &raw mut (*cur).args as *mut ::core::ffi::c_double,
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next as *mut t1_cpath;
                        cur = mfree(cur as *mut ::core::ffi::c_void) as *mut t1_cpath;
                    } else if (*prev).type_0 == cs_rlineto {
                        copy_args(
                            (&raw mut (*prev).args as *mut ::core::ffi::c_double)
                                .offset((*prev).num_args as isize),
                            &raw mut (*cur).args as *mut ::core::ffi::c_double,
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).type_0 = cs_rlinecurve;
                        (*prev).next = next as *mut t1_cpath;
                        cur = mfree(cur as *mut ::core::ffi::c_void) as *mut t1_cpath;
                    }
                }
            }
            cs_vhcurveto => {
                y += (*cur).args[0 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
                x += (*cur).args[1 as ::core::ffi::c_int as usize];
                y += (*cur).args[2 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
                x += (*cur).args[3 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
                if !prev.is_null()
                    && !cur.is_null()
                    && (*prev).num_args + (*cur).num_args < CS_ARG_STACK_MAX
                {
                    if (*prev).type_0 == cs_hvcurveto
                        && (*prev).num_args / 4 as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                            == 1 as ::core::ffi::c_int
                        || (*prev).type_0 == cs_vhcurveto
                            && (*prev).num_args / 4 as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                    {
                        copy_args(
                            (&raw mut (*prev).args as *mut ::core::ffi::c_double)
                                .offset((*prev).num_args as isize),
                            &raw mut (*cur).args as *mut ::core::ffi::c_double,
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next as *mut t1_cpath;
                        cur = mfree(cur as *mut ::core::ffi::c_void) as *mut t1_cpath;
                    }
                }
            }
            cs_hvcurveto => {
                x += (*cur).args[0 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
                x += (*cur).args[1 as ::core::ffi::c_int as usize];
                y += (*cur).args[2 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
                y += (*cur).args[3 as ::core::ffi::c_int as usize];
                if (*cd).bbox.llx > x {
                    (*cd).bbox.llx = x;
                }
                if (*cd).bbox.urx < x {
                    (*cd).bbox.urx = x;
                }
                if (*cd).bbox.lly > y {
                    (*cd).bbox.lly = y;
                }
                if (*cd).bbox.ury < y {
                    (*cd).bbox.ury = y;
                }
                if !prev.is_null()
                    && !cur.is_null()
                    && (*prev).num_args + (*cur).num_args < CS_ARG_STACK_MAX
                {
                    if (*prev).type_0 == cs_vhcurveto
                        && (*prev).num_args / 4 as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                            == 1 as ::core::ffi::c_int
                        || (*prev).type_0 == cs_hvcurveto
                            && (*prev).num_args / 4 as ::core::ffi::c_int % 2 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                    {
                        copy_args(
                            (&raw mut (*prev).args as *mut ::core::ffi::c_double)
                                .offset((*prev).num_args as isize),
                            &raw mut (*cur).args as *mut ::core::ffi::c_double,
                            (*cur).num_args,
                        );
                        (*prev).num_args += (*cur).num_args;
                        (*prev).next = next as *mut t1_cpath;
                        cur = mfree(cur as *mut ::core::ffi::c_void) as *mut t1_cpath;
                    }
                }
            }
            cs_flex => {
                i = 0 as ::core::ffi::c_uint;
                while i < 6 as ::core::ffi::c_uint {
                    x += (*cur).args[(2 as ::core::ffi::c_uint).wrapping_mul(i) as usize];
                    y += (*cur).args[(2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as usize];
                    if (*cd).bbox.llx > x {
                        (*cd).bbox.llx = x;
                    }
                    if (*cd).bbox.urx < x {
                        (*cd).bbox.urx = x;
                    }
                    if (*cd).bbox.lly > y {
                        (*cd).bbox.lly = y;
                    }
                    if (*cd).bbox.ury < y {
                        (*cd).bbox.ury = y;
                    }
                    i = i.wrapping_add(1);
                }
                if (*cur).args[12 as ::core::ffi::c_int as usize] == 50.0f64 {
                    if (*cur).args[1 as ::core::ffi::c_int as usize] == 0.0f64
                        && (*cur).args[11 as ::core::ffi::c_int as usize] == 0.0f64
                        && (*cur).args[5 as ::core::ffi::c_int as usize] == 0.0f64
                        && (*cur).args[7 as ::core::ffi::c_int as usize] == 0.0f64
                        && (*cur).args[3 as ::core::ffi::c_int as usize]
                            + (*cur).args[9 as ::core::ffi::c_int as usize]
                            == 0.0f64
                    {
                        (*cur).args[1 as ::core::ffi::c_int as usize] =
                            (*cur).args[2 as ::core::ffi::c_int as usize];
                        (*cur).args[2 as ::core::ffi::c_int as usize] =
                            (*cur).args[3 as ::core::ffi::c_int as usize];
                        (*cur).args[3 as ::core::ffi::c_int as usize] =
                            (*cur).args[4 as ::core::ffi::c_int as usize];
                        (*cur).args[4 as ::core::ffi::c_int as usize] =
                            (*cur).args[6 as ::core::ffi::c_int as usize];
                        (*cur).args[5 as ::core::ffi::c_int as usize] =
                            (*cur).args[8 as ::core::ffi::c_int as usize];
                        (*cur).args[6 as ::core::ffi::c_int as usize] =
                            (*cur).args[10 as ::core::ffi::c_int as usize];
                        (*cur).num_args = 7 as ::core::ffi::c_int;
                        (*cur).type_0 = cs_hflex;
                    } else if (*cur).args[5 as ::core::ffi::c_int as usize] == 0.0f64
                        && (*cur).args[7 as ::core::ffi::c_int as usize] == 0.0f64
                        && (*cur).args[1 as ::core::ffi::c_int as usize]
                            + (*cur).args[3 as ::core::ffi::c_int as usize]
                            + (*cur).args[9 as ::core::ffi::c_int as usize]
                            + (*cur).args[11 as ::core::ffi::c_int as usize]
                            == 0 as ::core::ffi::c_int as ::core::ffi::c_double
                    {
                        (*cur).args[5 as ::core::ffi::c_int as usize] =
                            (*cur).args[6 as ::core::ffi::c_int as usize];
                        (*cur).args[6 as ::core::ffi::c_int as usize] =
                            (*cur).args[8 as ::core::ffi::c_int as usize];
                        (*cur).args[7 as ::core::ffi::c_int as usize] =
                            (*cur).args[9 as ::core::ffi::c_int as usize];
                        (*cur).args[8 as ::core::ffi::c_int as usize] =
                            (*cur).args[10 as ::core::ffi::c_int as usize];
                        (*cur).num_args = 9 as ::core::ffi::c_int;
                        (*cur).type_0 = cs_hflex1;
                    }
                }
            }
            CS_HINT_DECL | cs_cntrmask => {}
            _ => {
                pdf_abort!(
                    b"Unexpected Type 2 charstring command %d.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*cur).type_0,
                );
            }
        }
        if !cur.is_null() {
            prev = cur;
        }
        cur = next;
    }
    if (*cd).bbox.llx > (*cd).bbox.urx {
        (*cd).bbox.urx = (*cd).sbw.wx;
        (*cd).bbox.llx = (*cd).bbox.urx;
    }
    if (*cd).bbox.lly > (*cd).bbox.ury {
        (*cd).bbox.ury = (*cd).sbw.wy;
        (*cd).bbox.lly = (*cd).bbox.ury;
    }
}
#[unsafe(export_name = "__pdf_impl_t1char_get_metrics")]
pub unsafe extern "C-unwind" fn t1char_get_metrics(
    mut src: *mut card8,
    mut srclen: ::core::ffi::c_int,
    mut subrs: *mut cff_index,
    mut ginfo: *mut t1_ginfo,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut t1char: t1_chardesc = t1_chardesc {
        flags: 0,
        sbw: C2RustUnnamed_3 {
            sbx: 0.,
            sby: 0.,
            wx: 0.,
            wy: 0.,
        },
        bbox: C2RustUnnamed_2 {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        seac: C2RustUnnamed_1 {
            asb: 0.,
            adx: 0.,
            ady: 0.,
            bchar: 0,
            achar: 0,
        },
        num_stems: 0,
        stems: [t1_stem {
            id: 0,
            dir: 0,
            pos: 0.,
            del: 0.,
        }; 96],
        charpath: ::core::ptr::null_mut::<t1_cpath>(),
        lastpath: ::core::ptr::null_mut::<t1_cpath>(),
    };
    let mut cd: *mut t1_chardesc = ::core::ptr::null_mut::<t1_chardesc>();
    cd = &raw mut t1char;
    init_charpath(cd);
    (*__pdf_state).status = CS_PARSE_OK;
    (*__pdf_state).phase = T1_CS_PHASE_INIT;
    (*__pdf_state).nest = 0 as ::core::ffi::c_int;
    (*__pdf_state).ps_stack_top = 0 as ::core::ffi::c_int;
    (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
    t1char_build_charpath(cd, &raw mut src, src.offset(srclen as isize), subrs);
    if (*__pdf_state).cs_stack_top != 0 as ::core::ffi::c_int
        || (*__pdf_state).ps_stack_top != 0 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Stack not empty. (%d, %d)\0" as *const u8 as *const ::core::ffi::c_char,
            (*__pdf_state).cs_stack_top,
            (*__pdf_state).ps_stack_top,
        );
    }
    do_postproc(cd);
    if !ginfo.is_null() {
        (*ginfo).wx = (*cd).sbw.wx;
        (*ginfo).wy = (*cd).sbw.wy;
        (*ginfo).bbox.llx = (*cd).bbox.llx;
        (*ginfo).bbox.lly = (*cd).bbox.lly;
        (*ginfo).bbox.urx = (*cd).bbox.urx;
        (*ginfo).bbox.ury = (*cd).bbox.ury;
        if (*cd).flags & T1_CS_FLAG_USE_SEAC != 0 {
            (*ginfo).use_seac = 1 as ::core::ffi::c_int;
            (*ginfo).seac.adx = (*cd).seac.adx;
            (*ginfo).seac.ady = (*cd).seac.ady;
            (*ginfo).seac.bchar = (*cd).seac.bchar;
            (*ginfo).seac.achar = (*cd).seac.achar;
        } else {
            (*ginfo).use_seac = 0 as ::core::ffi::c_int;
        }
    }
    release_charpath(cd);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn t1char_encode_charpath(
    mut cd: *mut t1_chardesc,
    mut default_width: ::core::ffi::c_double,
    mut nominal_width: ::core::ffi::c_double,
    mut dst: *mut card8,
    mut endptr: *mut card8,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut save: *mut card8 = ::core::ptr::null_mut::<card8>();
    let mut curr: *mut t1_cpath = ::core::ptr::null_mut::<t1_cpath>();
    if cd.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"cd\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    save = dst;
    curr = (*cd).charpath;
    (*__pdf_state).status = CS_PARSE_OK;
    (*__pdf_state).phase = T1_CS_PHASE_INIT;
    (*__pdf_state).nest = 0 as ::core::ffi::c_int;
    (*__pdf_state).ps_stack_top = 0 as ::core::ffi::c_int;
    (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
    if (*cd).sbw.wx != default_width {
        let mut wx: ::core::ffi::c_double = (*cd).sbw.wx - nominal_width;
        put_numbers(&raw mut wx, 1 as ::core::ffi::c_int, &raw mut dst, endptr);
        if (*__pdf_state).status != CS_PARSE_OK {
            pdf_abort!(
                b"Charstring encoder error: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*__pdf_state).status,
            );
        }
    }
    let mut num_hstems: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num_vstems: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut reset: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut stem: [::core::ffi::c_double; 2] = [0.; 2];
    i = 0 as ::core::ffi::c_int;
    while i < (*cd).num_stems && (*cd).stems[i as usize].dir == HSTEM {
        num_hstems += 1;
        stem[0 as ::core::ffi::c_int as usize] = if reset != 0 {
            (*cd).stems[i as usize].pos
        } else {
            (*cd).stems[i as usize].pos
                - ((*cd).stems[(i - 1 as ::core::ffi::c_int) as usize].pos
                    + (*cd).stems[(i - 1 as ::core::ffi::c_int) as usize].del)
        };
        stem[1 as ::core::ffi::c_int as usize] = (*cd).stems[i as usize].del;
        put_numbers(
            &raw mut stem as *mut ::core::ffi::c_double,
            2 as ::core::ffi::c_int,
            &raw mut dst,
            endptr,
        );
        if (*__pdf_state).status != CS_PARSE_OK {
            pdf_abort!(
                b"Charstring encoder error: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*__pdf_state).status,
            );
        }
        reset = 0 as ::core::ffi::c_int;
        if 2 as ::core::ffi::c_int * num_hstems > CS_ARG_STACK_MAX - 3 as ::core::ffi::c_int {
            if dst.offset(1 as ::core::ffi::c_int as isize) >= endptr {
                pdf_abort!(b"Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            let fresh10 = dst;
            dst = dst.offset(1);
            *fresh10 = (if (*cd).flags & T1_CS_FLAG_USE_HINTMASK != 0 {
                cs_hstemhm
            } else {
                cs_hstem
            }) as card8;
            reset = 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    if reset == 0 as ::core::ffi::c_int {
        if dst.offset(1 as ::core::ffi::c_int as isize) >= endptr {
            pdf_abort!(b"Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char);
        }
        let fresh11 = dst;
        dst = dst.offset(1);
        *fresh11 = (if (*cd).flags & T1_CS_FLAG_USE_HINTMASK != 0 {
            cs_hstemhm
        } else {
            cs_hstem
        }) as card8;
    }
    reset = 1 as ::core::ffi::c_int;
    if (*cd).num_stems - num_hstems > 0 as ::core::ffi::c_int {
        i = num_hstems;
        while i < (*cd).num_stems {
            num_vstems += 1;
            stem[0 as ::core::ffi::c_int as usize] = if reset != 0 {
                (*cd).stems[i as usize].pos
            } else {
                (*cd).stems[i as usize].pos
                    - ((*cd).stems[(i - 1 as ::core::ffi::c_int) as usize].pos
                        + (*cd).stems[(i - 1 as ::core::ffi::c_int) as usize].del)
            };
            stem[1 as ::core::ffi::c_int as usize] = (*cd).stems[i as usize].del;
            put_numbers(
                &raw mut stem as *mut ::core::ffi::c_double,
                2 as ::core::ffi::c_int,
                &raw mut dst,
                endptr,
            );
            if (*__pdf_state).status != CS_PARSE_OK {
                pdf_abort!(
                    b"Charstring encoder error: %d\0" as *const u8 as *const ::core::ffi::c_char,
                    (*__pdf_state).status,
                );
            }
            reset = 0 as ::core::ffi::c_int;
            if 2 as ::core::ffi::c_int * num_vstems > CS_ARG_STACK_MAX - 3 as ::core::ffi::c_int {
                if dst.offset(1 as ::core::ffi::c_int as isize) >= endptr {
                    pdf_abort!(b"Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                let fresh12 = dst;
                dst = dst.offset(1);
                *fresh12 = (if (*cd).flags & T1_CS_FLAG_USE_HINTMASK != 0 {
                    cs_vstemhm
                } else {
                    cs_vstem
                }) as card8;
                reset = 1 as ::core::ffi::c_int;
            }
            i += 1;
        }
        if reset == 0 as ::core::ffi::c_int {
            if dst.offset(1 as ::core::ffi::c_int as isize) >= endptr {
                pdf_abort!(b"Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            if (*cd).flags & T1_CS_FLAG_USE_HINTMASK != 0
                || (*cd).flags & T1_CS_FLAG_USE_CNTRMASK != 0
            {
                if (*curr).type_0 != CS_HINT_DECL && (*curr).type_0 != cs_cntrmask {
                    let fresh13 = dst;
                    dst = dst.offset(1);
                    *fresh13 = cs_vstemhm as card8;
                }
            } else {
                let fresh14 = dst;
                dst = dst.offset(1);
                *fresh14 = cs_vstem as card8;
            }
        }
    }
    while !curr.is_null() && (*curr).type_0 != cs_endchar {
        match (*curr).type_0 {
            CS_HINT_DECL => {
                let mut hintmask: [card8; 12] = [0; 12];
                memset(
                    &raw mut hintmask as *mut card8 as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (((*cd).num_stems + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
                        as size_t,
                );
                while !curr.is_null() && (*curr).type_0 == CS_HINT_DECL {
                    let mut stem_idx: ::core::ffi::c_int = 0;
                    stem_idx = get_stem(
                        cd,
                        (*curr).args[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    );
                    if !(stem_idx < (*cd).num_stems) as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        pdf_abort!(
                            c"assertion failed: %s".as_ptr(),
                            b"stem_idx < cd->num_stems\0" as *const u8
                                as *const ::core::ffi::c_char
                        );
                    } else {
                    };
                    hintmask[(stem_idx / 8 as ::core::ffi::c_int) as usize] = (hintmask
                        [(stem_idx / 8 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int)
                            << 7 as ::core::ffi::c_int - stem_idx % 8 as ::core::ffi::c_int)
                        as card8;
                    curr = (*curr).next as *mut t1_cpath;
                }
                if (*cd).flags & T1_CS_FLAG_USE_HINTMASK != 0 {
                    if dst.offset(
                        (((*cd).num_stems + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as isize,
                    ) >= endptr
                    {
                        pdf_abort!(
                            b"Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    let fresh15 = dst;
                    dst = dst.offset(1);
                    *fresh15 = cs_hintmask as card8;
                    memcpy(
                        dst as *mut ::core::ffi::c_void,
                        &raw mut hintmask as *mut card8 as *const ::core::ffi::c_void,
                        (((*cd).num_stems + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
                            as size_t,
                    );
                    dst = dst.offset(
                        (((*cd).num_stems + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
                            as isize,
                    );
                }
            }
            cs_cntrmask => {
                let mut cntrmask: [card8; 12] = [0; 12];
                let mut stem_idx_0: ::core::ffi::c_int = 0;
                let mut i_0: ::core::ffi::c_int = 0;
                memset(
                    &raw mut cntrmask as *mut card8 as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (((*cd).num_stems + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
                        as size_t,
                );
                i_0 = 0 as ::core::ffi::c_int;
                while i_0 < (*curr).num_args {
                    stem_idx_0 = get_stem(cd, (*curr).args[i_0 as usize] as ::core::ffi::c_int);
                    if !(stem_idx_0 < (*cd).num_stems) as ::core::ffi::c_int as ::core::ffi::c_long
                        != 0
                    {
                        pdf_abort!(
                            c"assertion failed: %s".as_ptr(),
                            b"stem_idx < cd->num_stems\0" as *const u8
                                as *const ::core::ffi::c_char
                        );
                    } else {
                    };
                    cntrmask[(stem_idx_0 / 8 as ::core::ffi::c_int) as usize] = (cntrmask
                        [(stem_idx_0 / 8 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int)
                            << 7 as ::core::ffi::c_int - stem_idx_0 % 8 as ::core::ffi::c_int)
                        as card8;
                    i_0 += 1;
                }
                if dst.offset(
                    (((*cd).num_stems + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as isize,
                ) >= endptr
                {
                    pdf_abort!(b"Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                let fresh16 = dst;
                dst = dst.offset(1);
                *fresh16 = cs_cntrmask as card8;
                memcpy(
                    dst as *mut ::core::ffi::c_void,
                    &raw mut cntrmask as *mut card8 as *const ::core::ffi::c_void,
                    (((*cd).num_stems + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
                        as size_t,
                );
                dst = dst.offset(
                    (((*cd).num_stems + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
                        as isize,
                );
                curr = (*curr).next as *mut t1_cpath;
            }
            cs_rmoveto | cs_hmoveto | cs_vmoveto | cs_rlineto | cs_hlineto | cs_vlineto
            | cs_rrcurveto | cs_hvcurveto | cs_vhcurveto | cs_rlinecurve | cs_rcurveline => {
                put_numbers(
                    &raw mut (*curr).args as *mut ::core::ffi::c_double,
                    (*curr).num_args,
                    &raw mut dst,
                    endptr,
                );
                if (*__pdf_state).status != CS_PARSE_OK {
                    pdf_abort!(
                        b"Charstring encoder error: %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*__pdf_state).status,
                    );
                }
                if dst.offset(1 as ::core::ffi::c_int as isize) >= endptr {
                    pdf_abort!(b"Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                let fresh17 = dst;
                dst = dst.offset(1);
                *fresh17 = (*curr).type_0 as card8;
                curr = (*curr).next as *mut t1_cpath;
            }
            cs_flex | cs_hflex | cs_hflex1 => {
                put_numbers(
                    &raw mut (*curr).args as *mut ::core::ffi::c_double,
                    (*curr).num_args,
                    &raw mut dst,
                    endptr,
                );
                if (*__pdf_state).status != CS_PARSE_OK {
                    pdf_abort!(
                        b"Charstring encoder error: %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*__pdf_state).status,
                    );
                }
                if dst.offset(2 as ::core::ffi::c_int as isize) >= endptr {
                    pdf_abort!(b"Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                let fresh18 = dst;
                dst = dst.offset(1);
                *fresh18 = cs_escape as card8;
                let fresh19 = dst;
                dst = dst.offset(1);
                *fresh19 = (*curr).type_0 as card8;
                curr = (*curr).next as *mut t1_cpath;
            }
            _ => {
                pdf_abort!(
                    b"Unknown Type 2 charstring command: %d\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*curr).type_0,
                );
            }
        }
    }
    if (*cd).flags & T1_CS_FLAG_USE_SEAC != 0 {
        let mut seac: [::core::ffi::c_double; 4] = [0.; 4];
        seac[0 as ::core::ffi::c_int as usize] = (*cd).seac.adx;
        seac[1 as ::core::ffi::c_int as usize] = (*cd).seac.ady;
        seac[2 as ::core::ffi::c_int as usize] = (*cd).seac.bchar as ::core::ffi::c_double;
        seac[3 as ::core::ffi::c_int as usize] = (*cd).seac.achar as ::core::ffi::c_double;
        put_numbers(
            &raw mut seac as *mut ::core::ffi::c_double,
            4 as ::core::ffi::c_int,
            &raw mut dst,
            endptr,
        );
        if (*__pdf_state).status != CS_PARSE_OK {
            pdf_abort!(
                b"Charstring encoder error: %d\0" as *const u8 as *const ::core::ffi::c_char,
                (*__pdf_state).status,
            );
        }
        if dst.offset(2 as ::core::ffi::c_int as isize) >= endptr {
            pdf_abort!(b"Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char);
        }
        pdf_warning!(
            b"Obsolete four arguments of \"endchar\" will be used for Type 1 \"seac\" operator.\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if dst.offset(1 as ::core::ffi::c_int as isize) >= endptr {
        pdf_abort!(b"Buffer overflow.\0" as *const u8 as *const ::core::ffi::c_char);
    }
    let fresh20 = dst;
    dst = dst.offset(1);
    *fresh20 = cs_endchar as card8;
    return dst.offset_from(save) as ::core::ffi::c_long as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_t1char_convert_charstring")]
pub unsafe extern "C-unwind" fn t1char_convert_charstring(
    mut dst: *mut card8,
    mut dstlen: ::core::ffi::c_int,
    mut src: *mut card8,
    mut srclen: ::core::ffi::c_int,
    mut subrs: *mut cff_index,
    mut default_width: ::core::ffi::c_double,
    mut nominal_width: ::core::ffi::c_double,
    mut ginfo: *mut t1_ginfo,
) -> ::core::ffi::c_int {
    let __pdf_state = unsafe { document_state() };
    let mut length: ::core::ffi::c_int = 0;
    let mut t1char: t1_chardesc = t1_chardesc {
        flags: 0,
        sbw: C2RustUnnamed_3 {
            sbx: 0.,
            sby: 0.,
            wx: 0.,
            wy: 0.,
        },
        bbox: C2RustUnnamed_2 {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        seac: C2RustUnnamed_1 {
            asb: 0.,
            adx: 0.,
            ady: 0.,
            bchar: 0,
            achar: 0,
        },
        num_stems: 0,
        stems: [t1_stem {
            id: 0,
            dir: 0,
            pos: 0.,
            del: 0.,
        }; 96],
        charpath: ::core::ptr::null_mut::<t1_cpath>(),
        lastpath: ::core::ptr::null_mut::<t1_cpath>(),
    };
    let mut cd: *mut t1_chardesc = ::core::ptr::null_mut::<t1_chardesc>();
    cd = &raw mut t1char;
    init_charpath(cd);
    (*__pdf_state).status = CS_PARSE_OK;
    (*__pdf_state).phase = T1_CS_PHASE_INIT;
    (*__pdf_state).nest = 0 as ::core::ffi::c_int;
    (*__pdf_state).ps_stack_top = 0 as ::core::ffi::c_int;
    (*__pdf_state).cs_stack_top = 0 as ::core::ffi::c_int;
    t1char_build_charpath(cd, &raw mut src, src.offset(srclen as isize), subrs);
    if (*__pdf_state).cs_stack_top != 0 as ::core::ffi::c_int
        || (*__pdf_state).ps_stack_top != 0 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Stack not empty. (%d, %d)\0" as *const u8 as *const ::core::ffi::c_char,
            (*__pdf_state).cs_stack_top,
            (*__pdf_state).ps_stack_top,
        );
    }
    do_postproc(cd);
    qsort(
        &raw mut (*cd).stems as *mut t1_stem as *mut ::core::ffi::c_void,
        (*cd).num_stems as size_t,
        ::core::mem::size_of::<t1_stem>() as size_t,
        Some(
            stem_compare
                as unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    length = t1char_encode_charpath(
        cd,
        default_width,
        nominal_width,
        dst,
        dst.offset(dstlen as isize),
    );
    if !ginfo.is_null() {
        (*ginfo).wx = (*cd).sbw.wx;
        (*ginfo).wy = (*cd).sbw.wy;
        (*ginfo).bbox.llx = (*cd).bbox.llx;
        (*ginfo).bbox.lly = (*cd).bbox.lly;
        (*ginfo).bbox.urx = (*cd).bbox.urx;
        (*ginfo).bbox.ury = (*cd).bbox.ury;
        if (*cd).flags & T1_CS_FLAG_USE_SEAC != 0 {
            (*ginfo).use_seac = 1 as ::core::ffi::c_int;
            (*ginfo).seac.adx = (*cd).seac.adx;
            (*ginfo).seac.ady = (*cd).seac.ady;
            (*ginfo).seac.bchar = (*cd).seac.bchar;
            (*ginfo).seac.achar = (*cd).seac.achar;
        } else {
            (*ginfo).use_seac = 0 as ::core::ffi::c_int;
        }
    }
    release_charpath(cd);
    return length;
}
unsafe extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_t1char_get_metrics(
    mut src: *mut card8,
    mut srclen: ::core::ffi::c_int,
    mut subrs: *mut cff_index,
    mut ginfo: *mut t1_ginfo,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        t1char_get_metrics(src, srclen, subrs, ginfo)
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
pub unsafe extern "C" fn __pdf_entry_t1char_convert_charstring(
    mut dst: *mut card8,
    mut dstlen: ::core::ffi::c_int,
    mut src: *mut card8,
    mut srclen: ::core::ffi::c_int,
    mut subrs: *mut cff_index,
    mut default_width: ::core::ffi::c_double,
    mut nominal_width: ::core::ffi::c_double,
    mut ginfo: *mut t1_ginfo,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        t1char_convert_charstring(
            dst,
            dstlen,
            src,
            srclen,
            subrs,
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
    phase: ::core::ffi::c_int,
    nest: ::core::ffi::c_int,
    cs_stack_top: ::core::ffi::c_int,
    ps_stack_top: ::core::ffi::c_int,
    cs_arg_stack: [::core::ffi::c_double; 49],
    ps_arg_stack: [::core::ffi::c_double; 194],
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        unsafe {
            Self {
                status: CS_PARSE_ERROR,
                phase: -(1 as ::core::ffi::c_int),
                nest: -(1 as ::core::ffi::c_int),
                cs_stack_top: 0 as ::core::ffi::c_int,
                ps_stack_top: 0 as ::core::ffi::c_int,
                cs_arg_stack: [0.; 49],
                ps_arg_stack: [0.; 194],
            }
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe {
        (*crate::compat::document::current())
            .backend
            .dpx_t1_char
            .pointer
    }
}
