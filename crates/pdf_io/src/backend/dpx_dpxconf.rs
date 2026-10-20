/* This is DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

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
#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn, static_mut_refs, dangerous_implicit_autorefs)]
unsafe extern "C-unwind" {
    fn strcmp(__s1: *const ::core::ffi::c_char, __s2: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut __stdoutp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}
pub type __int64_t = i64;
pub type __darwin_off_t = __int64_t;
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paper {
    pub name: *const ::core::ffi::c_char,
    pub pswidth: ::core::ffi::c_double,
    pub psheight: ::core::ffi::c_double,
}
pub type FILE = __sFILE;
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
    pub _read: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_char, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub _seek: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3],
    pub _nbuf: [::core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[inline]
unsafe extern "C-unwind" fn streq_ptr(mut s1: *const ::core::ffi::c_char, mut s2: *const ::core::ffi::c_char) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
    }
    return false_0 != 0;
}
#[unsafe(no_mangle)]
pub static mut dpx_conf: _dpx_conf = _dpx_conf {
    is_xbb: 0 as ::core::ffi::c_int,
    verbose_level: 0 as ::core::ffi::c_int,
    compat_mode: dpx_mode_normal_mode,
    ignore_font_license: 0 as ::core::ffi::c_int,
    file: C2RustUnnamed {
        keep_cache: 0 as ::core::ffi::c_int,
    },
};
#[unsafe(no_mangle)]
pub static mut paperspecs: [paper; 22] = [
    paper {
        name: b"letter\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 612.00f64,
        psheight: 792.00f64,
    },
    paper {
        name: b"legal\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 612.00f64,
        psheight: 1008.00f64,
    },
    paper {
        name: b"ledger\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 1224.00f64,
        psheight: 792.00f64,
    },
    paper {
        name: b"tabloid\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 792.00f64,
        psheight: 1224.00f64,
    },
    paper {
        name: b"a6\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 297.638f64,
        psheight: 419.528f64,
    },
    paper {
        name: b"a5\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 419.528f64,
        psheight: 595.276f64,
    },
    paper {
        name: b"a4\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 595.276f64,
        psheight: 841.890f64,
    },
    paper {
        name: b"a3\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 841.890f64,
        psheight: 1190.550f64,
    },
    paper {
        name: b"b6\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 364.25f64,
        psheight: 515.91f64,
    },
    paper {
        name: b"b5\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 515.91f64,
        psheight: 728.50f64,
    },
    paper {
        name: b"b4\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 728.50f64,
        psheight: 1031.81f64,
    },
    paper {
        name: b"b3\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 1031.81f64,
        psheight: 1457.00f64,
    },
    paper {
        name: b"b5var\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 515.91f64,
        psheight: 651.97f64,
    },
    paper {
        name: b"jisb6\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 364.25f64,
        psheight: 515.91f64,
    },
    paper {
        name: b"jisb5\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 515.91f64,
        psheight: 728.50f64,
    },
    paper {
        name: b"jisb4\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 728.50f64,
        psheight: 1031.81f64,
    },
    paper {
        name: b"jisb3\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 1031.81f64,
        psheight: 1457.00f64,
    },
    paper {
        name: b"isob6\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 354.331f64,
        psheight: 498.898f64,
    },
    paper {
        name: b"isob5\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 498.898f64,
        psheight: 708.661f64,
    },
    paper {
        name: b"isob4\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 708.661f64,
        psheight: 1000.630f64,
    },
    paper {
        name: b"isob3\0" as *const u8 as *const ::core::ffi::c_char,
        pswidth: 1000.630f64,
        psheight: 1417.320f64,
    },
    paper {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        pswidth: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        psheight: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
    },
];
#[unsafe(export_name = "__pdf_impl_paperinfo")]
pub unsafe extern "C-unwind" fn paperinfo(mut ppformat: *const ::core::ffi::c_char) -> *const paper {
    let mut ppinfo: *const paper = ::core::ptr::null::<paper>();
    if ppformat.is_null() {
        return ::core::ptr::null::<paper>();
    }
    ppinfo = (&raw const paperspecs as *const paper).offset(0 as ::core::ffi::c_int as isize) as *const paper;
    while !ppinfo.is_null()
        && !(if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
            (*ppinfo).name
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        })
        .is_null()
    {
        if streq_ptr(ppformat, (*ppinfo).name) {
            break;
        }
        ppinfo = if !ppinfo.offset(1 as ::core::ffi::c_int as isize).is_null() && !(*ppinfo.offset(1 as ::core::ffi::c_int as isize)).name.is_null() {
            ppinfo.offset(1 as ::core::ffi::c_int as isize)
        } else {
            ::core::ptr::null::<paper>()
        };
    }
    return if !ppinfo.is_null()
        && !(if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
            (*ppinfo).name
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        })
        .is_null()
    {
        ppinfo
    } else {
        ::core::ptr::null::<paper>()
    };
}
#[unsafe(export_name = "__pdf_impl_dumppaperinfo")]
pub unsafe extern "C-unwind" fn dumppaperinfo() {
    let mut ppinfo: *const paper = ::core::ptr::null::<paper>();
    ppinfo = (&raw const paperspecs as *const paper).offset(0 as ::core::ffi::c_int as isize) as *const paper;
    while !ppinfo.is_null()
        && !(if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
            (*ppinfo).name
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        })
        .is_null()
    {
        let mut wd: ::core::ffi::c_double = 0.;
        let mut ht: ::core::ffi::c_double = 0.;
        wd = if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
            (*ppinfo).pswidth
        } else {
            0.0f64
        };
        ht = if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
            (*ppinfo).psheight
        } else {
            0.0f64
        };
        fprintf(
            crate::backend::runtime::stdout().cast(),
            b"%s: %.2f %.2f (%.2fmm %.2fmm)\n\0" as *const u8 as *const ::core::ffi::c_char,
            if !ppinfo.is_null() && !(*ppinfo).name.is_null() {
                (*ppinfo).name
            } else {
                ::core::ptr::null::<::core::ffi::c_char>()
            },
            wd,
            ht,
            25.4f64 * wd / 72.0f64,
            25.4f64 * ht / 72.0f64,
        );
        ppinfo = if !ppinfo.offset(1 as ::core::ffi::c_int as isize).is_null() && !(*ppinfo.offset(1 as ::core::ffi::c_int as isize)).name.is_null() {
            ppinfo.offset(1 as ::core::ffi::c_int as isize)
        } else {
            ::core::ptr::null::<paper>()
        };
    }
}
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[repr(C)]
pub struct __sFILEX {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_paperinfo(mut ppformat: *const ::core::ffi::c_char, _result: *mut *const paper) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { paperinfo(ppformat) }) {
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
pub unsafe extern "C" fn __pdf_entry_dumppaperinfo() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dumppaperinfo() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
