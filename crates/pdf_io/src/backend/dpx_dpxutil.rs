/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2019 by Jin-Hwan Cho and Shunsaku Hirata,
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
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn islower(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isxdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn __error() -> *mut ::core::ffi::c_int;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn atof(_: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strtoll(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn gmtime(_: *const time_t) -> *mut tm;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut ttpi_source_date_epoch: time_t;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type __darwin_time_t = ::core::ffi::c_long;
pub type size_t = __darwin_size_t;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint32_t = u32;
pub type time_t = libc::time_t;
pub type tm = libc::tm;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_elem {
    pub data: *mut ::core::ffi::c_void,
    pub prev: *mut stack_elem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dpx_stack {
    pub size: ::core::ffi::c_int,
    pub top: *mut stack_elem,
    pub bottom: *mut stack_elem,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut ::core::ffi::c_char,
    pub keylen: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_table {
    pub count: ::core::ffi::c_int,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_iter {
    pub index: ::core::ffi::c_int,
    pub curr: *mut ::core::ffi::c_void,
    pub hash: *mut ht_table,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
pub const INVALID_EPOCH_VALUE: time_t = -(1 as ::core::ffi::c_int) as time_t;
pub const HASH_TABLE_SIZE: ::core::ffi::c_int = 503 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_xtoi")]
pub unsafe extern "C-unwind" fn xtoi(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32 {
        return c as ::core::ffi::c_int - '0' as i32;
    }
    if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'F' as i32 {
        return c as ::core::ffi::c_int - 'A' as i32 + 10 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'f' as i32 {
        return c as ::core::ffi::c_int - 'a' as i32 + 10 as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_min4")]
pub unsafe extern "C-unwind" fn min4(
    mut x1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut x3: ::core::ffi::c_double,
    mut x4: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut v: ::core::ffi::c_double = x1;
    if x2 < v {
        v = x2;
    }
    if x3 < v {
        v = x3;
    }
    if x4 < v {
        v = x4;
    }
    return v;
}
#[unsafe(export_name = "__pdf_impl_max4")]
pub unsafe extern "C-unwind" fn max4(
    mut x1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut x3: ::core::ffi::c_double,
    mut x4: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut v: ::core::ffi::c_double = x1;
    if x2 > v {
        v = x2;
    }
    if x3 > v {
        v = x3;
    }
    if x4 > v {
        v = x4;
    }
    return v;
}
unsafe extern "C-unwind" fn skip_white(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) {
    while *pp < endptr
        && (**pp as ::core::ffi::c_int == ' ' as i32
            || **pp as ::core::ffi::c_int == '\t' as i32
            || **pp as ::core::ffi::c_int == '\u{c}' as i32
            || **pp as ::core::ffi::c_int == '\r' as i32
            || **pp as ::core::ffi::c_int == '\n' as i32
            || **pp as ::core::ffi::c_int == '\0' as i32)
    {
        *pp = (*pp).offset(1);
    }
}
#[unsafe(export_name = "__pdf_impl_dpx_util_read_length")]
pub unsafe extern "C-unwind" fn dpx_util_read_length(
    mut vp: *mut ::core::ffi::c_double,
    mut mag: ::core::ffi::c_double,
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = *pp;
    let mut v: ::core::ffi::c_double = 0.;
    let mut u: ::core::ffi::c_double = 1.0f64;
    let mut _ukeys: [*const ::core::ffi::c_char; 10] = [
        b"pt\0" as *const u8 as *const ::core::ffi::c_char,
        b"in\0" as *const u8 as *const ::core::ffi::c_char,
        b"cm\0" as *const u8 as *const ::core::ffi::c_char,
        b"mm\0" as *const u8 as *const ::core::ffi::c_char,
        b"bp\0" as *const u8 as *const ::core::ffi::c_char,
        b"pc\0" as *const u8 as *const ::core::ffi::c_char,
        b"dd\0" as *const u8 as *const ::core::ffi::c_char,
        b"cc\0" as *const u8 as *const ::core::ffi::c_char,
        b"sp\0" as *const u8 as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut k: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    q = parse_float_decimal(&raw mut p, endptr);
    if q.is_null() {
        *vp = 0.0f64;
        *pp = p;
        return -(1 as ::core::ffi::c_int);
    }
    v = atof(q);
    free(q as *mut ::core::ffi::c_void);
    skip_white(&raw mut p, endptr);
    q = parse_c_ident(&raw mut p, endptr);
    if !q.is_null() {
        let mut qq: *mut ::core::ffi::c_char = q;
        if strlen(q) >= strlen(b"true\0" as *const u8 as *const ::core::ffi::c_char)
            && memcmp(
                q as *const ::core::ffi::c_void,
                b"true\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(b"true\0" as *const u8 as *const ::core::ffi::c_char),
            ) == 0
        {
            u /= if mag != 0.0f64 { mag } else { 1.0f64 };
            q = q.offset(strlen(b"true\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        }
        if strlen(q) == 0 as size_t {
            free(qq as *mut ::core::ffi::c_void);
            skip_white(&raw mut p, endptr);
            q = parse_c_ident(&raw mut p, endptr);
            qq = q;
        }
        if !q.is_null() {
            k = 0 as ::core::ffi::c_int;
            while !_ukeys[k as usize].is_null() && strcmp(_ukeys[k as usize], q) != 0 {
                k += 1;
            }
            match k {
                K_UNIT__PT => {
                    u *= 72.0f64 / 72.27f64;
                }
                K_UNIT__IN => {
                    u *= 72.0f64;
                }
                K_UNIT__CM => {
                    u *= 72.0f64 / 2.54f64;
                }
                K_UNIT__MM => {
                    u *= 72.0f64 / 25.4f64;
                }
                K_UNIT__BP => {
                    u *= 1.0f64;
                }
                K_UNIT__PC => {
                    u *= 12.0f64 * 72.0f64 / 72.27f64;
                }
                K_UNIT__DD => {
                    u *= 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64;
                }
                K_UNIT__CC => {
                    u *= 12.0f64 * 1238.0f64 / 1157.0f64 * 72.0f64 / 72.27f64;
                }
                K_UNIT__SP => {
                    u *=
                        72.0f64 / (72.27f64 * 65536 as ::core::ffi::c_int as ::core::ffi::c_double);
                }
                _ => {
                    pdf_warning!(
                        b"Unknown unit of measure: %s\0" as *const u8 as *const ::core::ffi::c_char,
                        q,
                    );
                    error = -(1 as ::core::ffi::c_int);
                }
            }
            free(qq as *mut ::core::ffi::c_void);
        } else {
            pdf_warning!(
                b"Missing unit of measure after \"true\"\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            error = -(1 as ::core::ffi::c_int);
        }
    }
    *vp = v * u;
    *pp = p;
    return error;
}
pub const K_UNIT__PT: ::core::ffi::c_int = 0;
pub const K_UNIT__IN: ::core::ffi::c_int = 1;
pub const K_UNIT__CM: ::core::ffi::c_int = 2;
pub const K_UNIT__MM: ::core::ffi::c_int = 3;
pub const K_UNIT__BP: ::core::ffi::c_int = 4;
pub const K_UNIT__PC: ::core::ffi::c_int = 5;
pub const K_UNIT__DD: ::core::ffi::c_int = 6;
pub const K_UNIT__CC: ::core::ffi::c_int = 7;
pub const K_UNIT__SP: ::core::ffi::c_int = 8;
#[unsafe(export_name = "__pdf_impl_dpx_util_get_unique_time_if_given")]
pub unsafe extern "C-unwind" fn dpx_util_get_unique_time_if_given() -> time_t {
    let mut source_date_epoch: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut epoch: int64_t = 0;
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ret: time_t = INVALID_EPOCH_VALUE;
    source_date_epoch = getenv(b"SOURCE_DATE_EPOCH\0" as *const u8 as *const ::core::ffi::c_char);
    if !source_date_epoch.is_null() {
        *crate::backend::runtime::errno() = 0 as ::core::ffi::c_int;
        epoch = strtoll(source_date_epoch, &raw mut endptr, 10 as ::core::ffi::c_int) as int64_t;
        if !(epoch < 0 as int64_t
            || *endptr as ::core::ffi::c_int != '\0' as i32
            || *crate::backend::runtime::errno() != 0 as ::core::ffi::c_int)
        {
            ret = epoch as time_t;
        }
    }
    return ret;
}
#[unsafe(export_name = "__pdf_impl_dpx_util_format_asn_date")]
pub unsafe extern "C-unwind" fn dpx_util_format_asn_date(
    mut date_string: *mut ::core::ffi::c_char,
    mut need_timezone: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tz_offset: int32_t = 0 as int32_t;
    let mut bd_time: *mut tm = libc::gmtime(&raw mut ttpi_source_date_epoch);
    if need_timezone != 0 {
        if (*bd_time).tm_isdst > 0 as ::core::ffi::c_int {
            tz_offset = (tz_offset as ::core::ffi::c_int + 3600 as ::core::ffi::c_int) as int32_t;
        }
        sprintf(
            date_string,
            b"D:%04d%02d%02d%02d%02d%02d%c%02d'%02d'\0" as *const u8 as *const ::core::ffi::c_char,
            (*bd_time).tm_year + 1900 as ::core::ffi::c_int,
            (*bd_time).tm_mon + 1 as ::core::ffi::c_int,
            (*bd_time).tm_mday,
            (*bd_time).tm_hour,
            (*bd_time).tm_min,
            (*bd_time).tm_sec,
            if tz_offset > 0 as int32_t {
                '+' as i32
            } else {
                '-' as i32
            },
            abs(tz_offset as ::core::ffi::c_int) / 3600 as ::core::ffi::c_int,
            abs(tz_offset as ::core::ffi::c_int) / 60 as ::core::ffi::c_int
                % 60 as ::core::ffi::c_int,
        );
    } else {
        sprintf(
            date_string,
            b"D:%04d%02d%02d%02d%02d%02d\0" as *const u8 as *const ::core::ffi::c_char,
            (*bd_time).tm_year + 1900 as ::core::ffi::c_int,
            (*bd_time).tm_mon + 1 as ::core::ffi::c_int,
            (*bd_time).tm_mday,
            (*bd_time).tm_hour,
            (*bd_time).tm_min,
            (*bd_time).tm_sec,
        );
    }
    return strlen(date_string) as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_skip_white_spaces")]
pub unsafe extern "C-unwind" fn skip_white_spaces(
    mut s: *mut *mut ::core::ffi::c_uchar,
    mut endptr: *mut ::core::ffi::c_uchar,
) {
    while *s < endptr {
        if !(**s as ::core::ffi::c_int == ' ' as i32
            || **s as ::core::ffi::c_int == '\t' as i32
            || **s as ::core::ffi::c_int == '\u{c}' as i32
            || **s as ::core::ffi::c_int == '\r' as i32
            || **s as ::core::ffi::c_int == '\n' as i32
            || **s as ::core::ffi::c_int == '\0' as i32)
        {
            break;
        }
        *s = (*s).offset(1);
    }
}
#[unsafe(export_name = "__pdf_impl_dpx_stack_init")]
pub unsafe extern "C-unwind" fn dpx_stack_init(mut stack: *mut dpx_stack) {
    (*stack).size = 0 as ::core::ffi::c_int;
    (*stack).top = ::core::ptr::null_mut::<stack_elem>();
    (*stack).bottom = ::core::ptr::null_mut::<stack_elem>();
}
#[unsafe(export_name = "__pdf_impl_dpx_stack_push")]
pub unsafe extern "C-unwind" fn dpx_stack_push(
    mut stack: *mut dpx_stack,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut elem: *mut stack_elem = ::core::ptr::null_mut::<stack_elem>();
    if stack.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"stack\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    elem = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<stack_elem>() as usize) as uint32_t)
        as *mut stack_elem;
    (*elem).prev = (*stack).top as *mut stack_elem;
    (*elem).data = data;
    (*stack).top = elem;
    if (*stack).size == 0 as ::core::ffi::c_int {
        (*stack).bottom = elem;
    }
    (*stack).size += 1;
}
#[unsafe(export_name = "__pdf_impl_dpx_stack_pop")]
pub unsafe extern "C-unwind" fn dpx_stack_pop(
    mut stack: *mut dpx_stack,
) -> *mut ::core::ffi::c_void {
    let mut elem: *mut stack_elem = ::core::ptr::null_mut::<stack_elem>();
    let mut data: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if stack.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"stack\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*stack).size == 0 as ::core::ffi::c_int {
        return NULL;
    }
    data = (*(*stack).top).data;
    elem = (*stack).top;
    (*stack).top = (*elem).prev as *mut stack_elem;
    if (*stack).size == 1 as ::core::ffi::c_int {
        (*stack).bottom = ::core::ptr::null_mut::<stack_elem>();
    }
    free(elem as *mut ::core::ffi::c_void);
    (*stack).size -= 1;
    return data;
}
#[unsafe(export_name = "__pdf_impl_dpx_stack_top")]
pub unsafe extern "C-unwind" fn dpx_stack_top(
    mut stack: *mut dpx_stack,
) -> *mut ::core::ffi::c_void {
    let mut data: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if stack.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"stack\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*stack).size == 0 as ::core::ffi::c_int {
        return NULL;
    }
    data = (*(*stack).top).data;
    return data;
}
#[unsafe(export_name = "__pdf_impl_dpx_stack_at")]
pub unsafe extern "C-unwind" fn dpx_stack_at(
    mut stack: *mut dpx_stack,
    mut pos: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut data: *mut ::core::ffi::c_void = NULL;
    let mut elem: *mut stack_elem = ::core::ptr::null_mut::<stack_elem>();
    if (*stack).size == 0 as ::core::ffi::c_int {
        return NULL;
    }
    elem = (*stack).top;
    while pos > 0 as ::core::ffi::c_int {
        elem = (*elem).prev as *mut stack_elem;
        pos -= 1;
    }
    if !elem.is_null() {
        data = (*elem).data;
    }
    return data;
}
#[unsafe(export_name = "__pdf_impl_dpx_stack_roll")]
pub unsafe extern "C-unwind" fn dpx_stack_roll(
    mut stack: *mut dpx_stack,
    mut n: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
) {
    if n > (*stack).size {
        return;
    }
    if n == 1 as ::core::ffi::c_int {
        return;
    }
    j = j % n;
    if j < 0 as ::core::ffi::c_int {
        j = n + j;
    }
    loop {
        let fresh0 = j;
        j = j - 1;
        if !(fresh0 > 0 as ::core::ffi::c_int) {
            break;
        }
        let mut m: ::core::ffi::c_int = n;
        let mut elem: *mut stack_elem = ::core::ptr::null_mut::<stack_elem>();
        let mut prev: *mut stack_elem = ::core::ptr::null_mut::<stack_elem>();
        let mut top: *mut stack_elem = ::core::ptr::null_mut::<stack_elem>();
        top = (*stack).top;
        elem = top;
        loop {
            m -= 1;
            if !(m > 0 as ::core::ffi::c_int) {
                break;
            }
            elem = (*elem).prev as *mut stack_elem;
        }
        prev = (*elem).prev as *mut stack_elem;
        (*stack).top = (*top).prev as *mut stack_elem;
        (*elem).prev = top as *mut stack_elem;
        (*top).prev = prev as *mut stack_elem;
    }
}
#[unsafe(export_name = "__pdf_impl_dpx_stack_depth")]
pub unsafe extern "C-unwind" fn dpx_stack_depth(mut stack: *mut dpx_stack) -> ::core::ffi::c_int {
    if stack.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"stack\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*stack).size;
}
#[unsafe(export_name = "__pdf_impl_ht_init_table")]
pub unsafe extern "C-unwind" fn ht_init_table(
    mut ht: *mut ht_table,
    mut hval_free_fn: hval_free_func,
) {
    let mut i: ::core::ffi::c_int = 0;
    if ht.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ht\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    i = 0 as ::core::ffi::c_int;
    while i < HASH_TABLE_SIZE {
        (*ht).table[i as usize] = ::core::ptr::null_mut::<ht_entry>();
        i += 1;
    }
    (*ht).count = 0 as ::core::ffi::c_int;
    (*ht).hval_free_fn = hval_free_fn;
}
#[unsafe(export_name = "__pdf_impl_ht_clear_table")]
pub unsafe extern "C-unwind" fn ht_clear_table(mut ht: *mut ht_table) {
    let mut i: ::core::ffi::c_int = 0;
    if ht.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ht\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    i = 0 as ::core::ffi::c_int;
    while i < HASH_TABLE_SIZE {
        let mut hent: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
        let mut next: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
        hent = (*ht).table[i as usize];
        while !hent.is_null() {
            if !(*hent).value.is_null() && (*ht).hval_free_fn.is_some() {
                (*ht).hval_free_fn.expect("non-null function pointer")((*hent).value);
            }
            (*hent).value = NULL;
            if !(*hent).key.is_null() {
                free((*hent).key as *mut ::core::ffi::c_void);
            }
            (*hent).key = ::core::ptr::null_mut::<::core::ffi::c_char>();
            next = (*hent).next;
            free(hent as *mut ::core::ffi::c_void);
            hent = next;
        }
        (*ht).table[i as usize] = ::core::ptr::null_mut::<ht_entry>();
        i += 1;
    }
    (*ht).count = 0 as ::core::ffi::c_int;
    (*ht).hval_free_fn = None;
}
#[unsafe(export_name = "__pdf_impl_ht_table_size")]
pub unsafe extern "C-unwind" fn ht_table_size(mut ht: *mut ht_table) -> ::core::ffi::c_int {
    if ht.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ht\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*ht).count;
}
unsafe extern "C-unwind" fn get_hash(
    mut key: *const ::core::ffi::c_void,
    mut keylen: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut hkey: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < keylen {
        hkey = (hkey << 5 as ::core::ffi::c_int)
            .wrapping_add(hkey)
            .wrapping_add(
                *(key as *const ::core::ffi::c_char).offset(i as isize) as ::core::ffi::c_uint
            );
        i += 1;
    }
    return hkey.wrapping_rem(HASH_TABLE_SIZE as ::core::ffi::c_uint);
}
#[unsafe(export_name = "__pdf_impl_ht_lookup_table")]
pub unsafe extern "C-unwind" fn ht_lookup_table(
    mut ht: *mut ht_table,
    mut key: *const ::core::ffi::c_void,
    mut keylen: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut hent: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
    let mut hkey: ::core::ffi::c_uint = 0;
    if !(!ht.is_null() && !key.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ht && key\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    while !hent.is_null() {
        if (*hent).keylen == keylen
            && memcmp(
                (*hent).key as *const ::core::ffi::c_void,
                key,
                keylen as size_t,
            ) == 0
        {
            return (*hent).value;
        }
        hent = (*hent).next;
    }
    return NULL;
}
#[unsafe(export_name = "__pdf_impl_ht_remove_table")]
pub unsafe extern "C-unwind" fn ht_remove_table(
    mut ht: *mut ht_table,
    mut key: *const ::core::ffi::c_void,
    mut keylen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut hent: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
    let mut prev: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
    let mut hkey: ::core::ffi::c_uint = 0;
    if !(!ht.is_null() && !key.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ht && key\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    prev = ::core::ptr::null_mut::<ht_entry>();
    while !hent.is_null() {
        if (*hent).keylen == keylen
            && memcmp(
                (*hent).key as *const ::core::ffi::c_void,
                key,
                keylen as size_t,
            ) == 0
        {
            break;
        }
        prev = hent;
        hent = (*hent).next;
    }
    if !hent.is_null() {
        (*hent).key = mfree((*hent).key as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
        (*hent).keylen = 0 as ::core::ffi::c_int;
        if !(*hent).value.is_null() && (*ht).hval_free_fn.is_some() {
            (*ht).hval_free_fn.expect("non-null function pointer")((*hent).value);
        }
        (*hent).value = NULL;
        if !prev.is_null() {
            (*prev).next = (*hent).next;
        } else {
            (*ht).table[hkey as usize] = (*hent).next;
        }
        free(hent as *mut ::core::ffi::c_void);
        (*ht).count -= 1;
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
#[unsafe(export_name = "__pdf_impl_ht_insert_table")]
pub unsafe extern "C-unwind" fn ht_insert_table(
    mut ht: *mut ht_table,
    mut key: *const ::core::ffi::c_void,
    mut keylen: ::core::ffi::c_int,
    mut value: *mut ::core::ffi::c_void,
) {
    let mut hent: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
    let mut prev: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
    let mut hkey: ::core::ffi::c_uint = 0;
    if !(!ht.is_null() && !key.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ht && key\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    prev = ::core::ptr::null_mut::<ht_entry>();
    while !hent.is_null() {
        if (*hent).keylen == keylen
            && memcmp(
                (*hent).key as *const ::core::ffi::c_void,
                key,
                keylen as size_t,
            ) == 0
        {
            break;
        }
        prev = hent;
        hent = (*hent).next;
    }
    if !hent.is_null() {
        if !(*hent).value.is_null() && (*ht).hval_free_fn.is_some() {
            (*ht).hval_free_fn.expect("non-null function pointer")((*hent).value);
        }
        (*hent).value = value;
    } else {
        hent = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<ht_entry>() as usize)
            as uint32_t) as *mut ht_entry;
        (*hent).key = new((keylen as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memcpy(
            (*hent).key as *mut ::core::ffi::c_void,
            key,
            keylen as size_t,
        );
        (*hent).keylen = keylen;
        (*hent).value = value;
        (*hent).next = ::core::ptr::null_mut::<ht_entry>();
        if !prev.is_null() {
            (*prev).next = hent;
        } else {
            (*ht).table[hkey as usize] = hent;
        }
        (*ht).count += 1;
    };
}
#[unsafe(export_name = "__pdf_impl_ht_append_table")]
pub unsafe extern "C-unwind" fn ht_append_table(
    mut ht: *mut ht_table,
    mut key: *const ::core::ffi::c_void,
    mut keylen: ::core::ffi::c_int,
    mut value: *mut ::core::ffi::c_void,
) {
    let mut hent: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
    let mut last: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
    let mut hkey: ::core::ffi::c_uint = 0;
    hkey = get_hash(key, keylen);
    hent = (*ht).table[hkey as usize];
    if hent.is_null() {
        hent = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<ht_entry>() as usize)
            as uint32_t) as *mut ht_entry;
        (*ht).table[hkey as usize] = hent;
    } else {
        while !hent.is_null() {
            last = hent;
            hent = (*hent).next;
        }
        hent = new((1 as ::core::ffi::c_int as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<ht_entry>() as usize)
            as uint32_t) as *mut ht_entry;
        (*last).next = hent;
    }
    (*hent).key = new((keylen as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    memcpy(
        (*hent).key as *mut ::core::ffi::c_void,
        key,
        keylen as size_t,
    );
    (*hent).keylen = keylen;
    (*hent).value = value;
    (*hent).next = ::core::ptr::null_mut::<ht_entry>();
    (*ht).count += 1;
}
#[unsafe(export_name = "__pdf_impl_ht_set_iter")]
pub unsafe extern "C-unwind" fn ht_set_iter(
    mut ht: *mut ht_table,
    mut iter: *mut ht_iter,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if !(!ht.is_null() && !iter.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"ht && iter\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    i = 0 as ::core::ffi::c_int;
    while i < HASH_TABLE_SIZE {
        if !(*ht).table[i as usize].is_null() {
            (*iter).index = i;
            (*iter).curr = (*ht).table[i as usize] as *mut ::core::ffi::c_void;
            (*iter).hash = ht;
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_ht_clear_iter")]
pub unsafe extern "C-unwind" fn ht_clear_iter(mut iter: *mut ht_iter) {
    if !iter.is_null() {
        (*iter).index = HASH_TABLE_SIZE;
        (*iter).curr = NULL;
        (*iter).hash = ::core::ptr::null_mut::<ht_table>();
    }
}
#[unsafe(export_name = "__pdf_impl_ht_iter_getkey")]
pub unsafe extern "C-unwind" fn ht_iter_getkey(
    mut iter: *mut ht_iter,
    mut keylen: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut hent: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
    hent = (*iter).curr as *mut ht_entry;
    if !iter.is_null() && !hent.is_null() {
        *keylen = (*hent).keylen;
        return (*hent).key;
    } else {
        *keylen = 0 as ::core::ffi::c_int;
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
}
#[unsafe(export_name = "__pdf_impl_ht_iter_getval")]
pub unsafe extern "C-unwind" fn ht_iter_getval(mut iter: *mut ht_iter) -> *mut ::core::ffi::c_void {
    let mut hent: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
    hent = (*iter).curr as *mut ht_entry;
    if !iter.is_null() && !hent.is_null() {
        return (*hent).value;
    } else {
        return NULL;
    };
}
#[unsafe(export_name = "__pdf_impl_ht_iter_next")]
pub unsafe extern "C-unwind" fn ht_iter_next(mut iter: *mut ht_iter) -> ::core::ffi::c_int {
    let mut hent: *mut ht_entry = ::core::ptr::null_mut::<ht_entry>();
    let mut ht: *mut ht_table = ::core::ptr::null_mut::<ht_table>();
    if iter.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"iter\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    ht = (*iter).hash;
    hent = (*iter).curr as *mut ht_entry;
    hent = (*hent).next;
    while hent.is_null() && {
        (*iter).index += 1;
        (*iter).index < HASH_TABLE_SIZE
    } {
        hent = (*ht).table[(*iter).index as usize];
    }
    (*iter).curr = hent as *mut ::core::ffi::c_void;
    return if !hent.is_null() {
        0 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
unsafe extern "C-unwind" fn read_c_escchar(
    mut r: *mut ::core::ffi::c_char,
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut l: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = *pp;
    match *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
        97 => {
            c = '\u{7}' as i32;
            p = p.offset(1);
        }
        98 => {
            c = '\u{8}' as i32;
            p = p.offset(1);
        }
        102 => {
            c = '\u{c}' as i32;
            p = p.offset(1);
        }
        110 => {
            c = '\n' as i32;
            p = p.offset(1);
        }
        114 => {
            c = '\r' as i32;
            p = p.offset(1);
        }
        116 => {
            c = '\t' as i32;
            p = p.offset(1);
        }
        118 => {
            c = '\u{b}' as i32;
            p = p.offset(1);
        }
        92 | 63 | 39 | 34 => {
            c = *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            p = p.offset(1);
        }
        10 => {
            l = 0 as ::core::ffi::c_int;
            p = p.offset(1);
        }
        13 => {
            p = p.offset(1);
            if p < endptr
                && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '\n' as i32
            {
                p = p.offset(1);
            }
            l = 0 as ::core::ffi::c_int;
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
            let mut i: ::core::ffi::c_int = 0;
            c = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < 3 as ::core::ffi::c_int
                && p < endptr
                && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
                && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '7' as i32
            {
                c = (c << 3 as ::core::ffi::c_int)
                    + (*p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - '0' as i32);
                i += 1;
                p = p.offset(1);
            }
        }
        120 => {
            let mut i_0: ::core::ffi::c_int = 0;
            c = 0 as ::core::ffi::c_int;
            i_0 = 0 as ::core::ffi::c_int;
            p = p.offset(1);
            while i_0 < 2 as ::core::ffi::c_int
                && p < endptr
                && isxdigit(
                    *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar
                        as ::core::ffi::c_int,
                ) != 0
            {
                c = (c << 4 as ::core::ffi::c_int)
                    + (if isdigit(*p.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_uchar
                        as ::core::ffi::c_int)
                        != 0
                    {
                        *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            - '0' as i32
                    } else {
                        (if islower(*p.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_uchar
                            as ::core::ffi::c_int)
                            != 0
                        {
                            *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                - 'a' as i32
                                + 10 as ::core::ffi::c_int
                        } else {
                            *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                                - 'A' as i32
                                + 10 as ::core::ffi::c_int
                        })
                    });
                i_0 += 1;
                p = p.offset(1);
            }
        }
        _ => {
            pdf_warning!(
                b"Unknown escape char sequence: \\%c\0" as *const u8 as *const ::core::ffi::c_char,
                *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            );
            l = 0 as ::core::ffi::c_int;
            p = p.offset(1);
        }
    }
    if !r.is_null() {
        *r = c as ::core::ffi::c_char;
    }
    *pp = p;
    return l;
}
pub const C_QUOTE: ::core::ffi::c_int = 34;
pub const C_ESCAPE: ::core::ffi::c_int = 92;
unsafe extern "C-unwind" fn read_c_litstrc(
    mut q: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s: ::core::ffi::c_int = Q_CONT;
    l = 0 as ::core::ffi::c_int;
    p = *pp;
    while s == Q_CONT && p < endptr {
        match *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
            C_QUOTE => {
                s = Q_TERM;
                p = p.offset(1);
            }
            C_ESCAPE => {
                if !q.is_null() && l == len {
                    s = Q_ERROR_BUFF;
                } else {
                    p = p.offset(1);
                    l += read_c_escchar(
                        if !q.is_null() {
                            q.offset(l as isize) as *mut ::core::ffi::c_char
                        } else {
                            ::core::ptr::null_mut::<::core::ffi::c_char>()
                        },
                        &raw mut p,
                        endptr,
                    );
                }
            }
            10 | 13 => {
                s = Q_ERROR_INVAL;
            }
            _ => {
                if !q.is_null() && l == len {
                    s = Q_ERROR_BUFF;
                } else {
                    if q.is_null() {
                        l += 1;
                    } else {
                        let fresh1 = l;
                        l = l + 1;
                        *q.offset(fresh1 as isize) = *p.offset(0 as ::core::ffi::c_int as isize);
                    }
                    p = p.offset(1);
                }
            }
        }
    }
    if s == Q_TERM {
        if !q.is_null() && l == len {
            s = Q_ERROR_BUFF;
        } else if !q.is_null() {
            let fresh2 = l;
            l = l + 1;
            *q.offset(fresh2 as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
    }
    *pp = p;
    return if s == Q_TERM { l } else { s };
}
pub const Q_TERM: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Q_CONT: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const Q_ERROR_INVAL: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const Q_ERROR_BUFF: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
#[unsafe(export_name = "__pdf_impl_parse_c_string")]
pub unsafe extern "C-unwind" fn parse_c_string(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = *pp;
    let mut l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if p >= endptr || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != C_QUOTE {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    p = p.offset(1);
    l = read_c_litstrc(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        &raw mut p,
        endptr,
    );
    if l >= 0 as ::core::ffi::c_int {
        q = new(((l + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        p = (*pp).offset(1 as ::core::ffi::c_int as isize);
        l = read_c_litstrc(q, l + 1 as ::core::ffi::c_int, &raw mut p, endptr);
    }
    *pp = p;
    return q;
}
#[unsafe(export_name = "__pdf_impl_parse_c_ident")]
pub unsafe extern "C-unwind" fn parse_c_ident(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = *pp;
    let mut n: ::core::ffi::c_int = 0;
    if p >= endptr
        || !(*p as ::core::ffi::c_int == '_' as i32
            || *p as ::core::ffi::c_int >= 'a' as i32 && *p as ::core::ffi::c_int <= 'z' as i32
            || *p as ::core::ffi::c_int >= 'A' as i32 && *p as ::core::ffi::c_int <= 'Z' as i32)
    {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    n = 0 as ::core::ffi::c_int;
    while p < endptr
        && (*p as ::core::ffi::c_int == '_' as i32
            || *p as ::core::ffi::c_int >= 'a' as i32 && *p as ::core::ffi::c_int <= 'z' as i32
            || *p as ::core::ffi::c_int >= 'A' as i32 && *p as ::core::ffi::c_int <= 'Z' as i32
            || *p as ::core::ffi::c_int >= '0' as i32 && *p as ::core::ffi::c_int <= '9' as i32)
    {
        p = p.offset(1);
        n += 1;
    }
    q = new(((n + 1 as ::core::ffi::c_int) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    memcpy(
        q as *mut ::core::ffi::c_void,
        *pp as *const ::core::ffi::c_void,
        n as size_t,
    );
    *q.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    *pp = p;
    return q;
}
#[unsafe(export_name = "__pdf_impl_parse_float_decimal")]
pub unsafe extern "C-unwind" fn parse_float_decimal(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = *pp;
    let mut s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if p >= endptr {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '+' as i32
        || *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
    {
        p = p.offset(1);
    }
    s = 0 as ::core::ffi::c_int;
    n = 0 as ::core::ffi::c_int;
    while p < endptr && s >= 0 as ::core::ffi::c_int {
        match *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
            43 | 45 => {
                if s != 2 as ::core::ffi::c_int {
                    s = -(1 as ::core::ffi::c_int);
                } else {
                    s = 3 as ::core::ffi::c_int;
                    p = p.offset(1);
                }
            }
            46 => {
                if s > 0 as ::core::ffi::c_int {
                    s = -(1 as ::core::ffi::c_int);
                } else {
                    s = 1 as ::core::ffi::c_int;
                    p = p.offset(1);
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                n += 1;
                p = p.offset(1);
            }
            69 | 101 => {
                if n == 0 as ::core::ffi::c_int || s == 2 as ::core::ffi::c_int {
                    s = -(1 as ::core::ffi::c_int);
                } else {
                    s = 2 as ::core::ffi::c_int;
                    p = p.offset(1);
                }
            }
            _ => {
                s = -(1 as ::core::ffi::c_int);
            }
        }
    }
    if n != 0 as ::core::ffi::c_int {
        n = p.offset_from(*pp) as ::core::ffi::c_long as ::core::ffi::c_int;
        q = new(((n + 1 as ::core::ffi::c_int) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memcpy(
            q as *mut ::core::ffi::c_void,
            *pp as *const ::core::ffi::c_void,
            n as size_t,
        );
        *q.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    *pp = p;
    return q;
}
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_xtoi(
    mut c: ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { xtoi(c) }) {
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
pub unsafe extern "C" fn __pdf_entry_min4(
    mut x1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut x3: ::core::ffi::c_double,
    mut x4: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { min4(x1, x2, x3, x4) }) {
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
pub unsafe extern "C" fn __pdf_entry_max4(
    mut x1: ::core::ffi::c_double,
    mut x2: ::core::ffi::c_double,
    mut x3: ::core::ffi::c_double,
    mut x4: ::core::ffi::c_double,
    _result: *mut ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { max4(x1, x2, x3, x4) }) {
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
pub unsafe extern "C" fn __pdf_entry_dpx_util_read_length(
    mut vp: *mut ::core::ffi::c_double,
    mut mag: ::core::ffi::c_double,
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_util_read_length(vp, mag, pp, endptr) })
    {
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
pub unsafe extern "C" fn __pdf_entry_dpx_util_get_unique_time_if_given(
    _result: *mut time_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_util_get_unique_time_if_given() }) {
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
pub unsafe extern "C" fn __pdf_entry_dpx_util_format_asn_date(
    mut date_string: *mut ::core::ffi::c_char,
    mut need_timezone: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        dpx_util_format_asn_date(date_string, need_timezone)
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
pub unsafe extern "C" fn __pdf_entry_skip_white_spaces(
    mut s: *mut *mut ::core::ffi::c_uchar,
    mut endptr: *mut ::core::ffi::c_uchar,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { skip_white_spaces(s, endptr) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dpx_stack_init(mut stack: *mut dpx_stack) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_stack_init(stack) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dpx_stack_push(
    mut stack: *mut dpx_stack,
    mut data: *mut ::core::ffi::c_void,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_stack_push(stack, data) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dpx_stack_pop(
    mut stack: *mut dpx_stack,
    _result: *mut *mut ::core::ffi::c_void,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_stack_pop(stack) }) {
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
pub unsafe extern "C" fn __pdf_entry_dpx_stack_top(
    mut stack: *mut dpx_stack,
    _result: *mut *mut ::core::ffi::c_void,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_stack_top(stack) }) {
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
pub unsafe extern "C" fn __pdf_entry_dpx_stack_at(
    mut stack: *mut dpx_stack,
    mut pos: ::core::ffi::c_int,
    _result: *mut *mut ::core::ffi::c_void,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_stack_at(stack, pos) }) {
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
pub unsafe extern "C" fn __pdf_entry_dpx_stack_roll(
    mut stack: *mut dpx_stack,
    mut n: ::core::ffi::c_int,
    mut j: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_stack_roll(stack, n, j) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dpx_stack_depth(
    mut stack: *mut dpx_stack,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dpx_stack_depth(stack) }) {
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
pub unsafe extern "C" fn __pdf_entry_ht_init_table(
    mut ht: *mut ht_table,
    mut hval_free_fn: hval_free_func,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_init_table(ht, hval_free_fn) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_ht_clear_table(mut ht: *mut ht_table) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_clear_table(ht) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_ht_table_size(
    mut ht: *mut ht_table,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_table_size(ht) }) {
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
pub unsafe extern "C" fn __pdf_entry_ht_lookup_table(
    mut ht: *mut ht_table,
    mut key: *const ::core::ffi::c_void,
    mut keylen: ::core::ffi::c_int,
    _result: *mut *mut ::core::ffi::c_void,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_lookup_table(ht, key, keylen) }) {
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
pub unsafe extern "C" fn __pdf_entry_ht_remove_table(
    mut ht: *mut ht_table,
    mut key: *const ::core::ffi::c_void,
    mut keylen: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_remove_table(ht, key, keylen) }) {
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
pub unsafe extern "C" fn __pdf_entry_ht_insert_table(
    mut ht: *mut ht_table,
    mut key: *const ::core::ffi::c_void,
    mut keylen: ::core::ffi::c_int,
    mut value: *mut ::core::ffi::c_void,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_insert_table(ht, key, keylen, value) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_ht_append_table(
    mut ht: *mut ht_table,
    mut key: *const ::core::ffi::c_void,
    mut keylen: ::core::ffi::c_int,
    mut value: *mut ::core::ffi::c_void,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_append_table(ht, key, keylen, value) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_ht_set_iter(
    mut ht: *mut ht_table,
    mut iter: *mut ht_iter,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_set_iter(ht, iter) }) {
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
pub unsafe extern "C" fn __pdf_entry_ht_clear_iter(mut iter: *mut ht_iter) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_clear_iter(iter) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_ht_iter_getkey(
    mut iter: *mut ht_iter,
    mut keylen: *mut ::core::ffi::c_int,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_iter_getkey(iter, keylen) }) {
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
pub unsafe extern "C" fn __pdf_entry_ht_iter_getval(
    mut iter: *mut ht_iter,
    _result: *mut *mut ::core::ffi::c_void,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_iter_getval(iter) }) {
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
pub unsafe extern "C" fn __pdf_entry_ht_iter_next(
    mut iter: *mut ht_iter,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ht_iter_next(iter) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_c_string(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_c_string(pp, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_c_ident(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_c_ident(pp, endptr) }) {
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
pub unsafe extern "C" fn __pdf_entry_parse_float_decimal(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_float_decimal(pp, endptr) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
