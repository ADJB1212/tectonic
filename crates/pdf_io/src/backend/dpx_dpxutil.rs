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
use crate::compat::allocation::free;
unsafe extern "C-unwind" {
    fn atof(_: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strtoll(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_longlong;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut ttpi_source_date_epoch: time_t;
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
pub const INVALID_EPOCH_VALUE: time_t = -(1 as ::core::ffi::c_int) as time_t;
pub const HASH_TABLE_SIZE: ::core::ffi::c_int = 503 as ::core::ffi::c_int;
#[unsafe(export_name = "__pdf_impl_xtoi")]
pub extern "C-unwind" fn xtoi(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
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
pub extern "C-unwind" fn min4(
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
pub extern "C-unwind" fn max4(
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
pub(super) fn unit_to_bp(unit: &[u8]) -> Option<f64> {
    Some(match unit {
        b"pt" => 72.0 / 72.27,
        b"in" => 72.0,
        b"cm" => 72.0 / 2.54,
        b"mm" => 72.0 / 25.4,
        b"bp" => 1.0,
        b"pc" => 12.0 * 72.0 / 72.27,
        b"dd" => 1238.0 / 1157.0 * 72.0 / 72.27,
        b"cc" => 12.0 * 1238.0 / 1157.0 * 72.0 / 72.27,
        b"sp" => 72.0 / (72.27 * 65536.0),
        _ => return None,
    })
}

#[unsafe(export_name = "__pdf_impl_dpx_util_read_length")]
pub unsafe extern "C-unwind" fn dpx_util_read_length(
    vp: *mut f64,
    mag: f64,
    pp: *mut *const ::core::ffi::c_char,
    endptr: *const ::core::ffi::c_char,
) -> i32 {
    let number = parse_float_decimal(pp, endptr);
    if number.is_null() {
        *vp = 0.0;
        return -1;
    }
    let value = atof(number);
    free(number.cast());
    let mut scale = 1.0;
    let mut error = 0;
    skip_white(pp, endptr);
    let mut allocation = parse_c_ident(pp, endptr);
    if !allocation.is_null() {
        let mut unit = allocation;
        if std::ffi::CStr::from_ptr(unit)
            .to_bytes()
            .starts_with(b"true")
        {
            scale /= if mag != 0.0 { mag } else { 1.0 };
            unit = unit.add(4);
            if *unit == 0 {
                free(allocation.cast());
                skip_white(pp, endptr);
                allocation = parse_c_ident(pp, endptr);
                unit = allocation;
            }
        }
        if unit.is_null() {
            pdf_warning!(c"Missing unit of measure after \"true\"".as_ptr());
            error = -1;
        } else {
            match unit_to_bp(std::ffi::CStr::from_ptr(unit).to_bytes()) {
                Some(factor) => scale *= factor,
                None => {
                    pdf_warning!(c"Unknown unit of measure: %s".as_ptr(), unit);
                    error = -1;
                }
            }
            free(allocation.cast());
        }
    }
    *vp = value * scale;
    error
}
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
    date_string: *mut ::core::ffi::c_char,
    need_timezone: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let time = libc::gmtime(&raw mut ttpi_source_date_epoch)
        .as_ref()
        .unwrap_or_else(|| pdf_abort!(c"Invalid timestamp".as_ptr()));
    let timezone = if need_timezone == 0 {
        c""
    } else if time.tm_isdst > 0 {
        c"+01'00'"
    } else {
        c"-00'00'"
    };
    sprintf(
        date_string,
        c"D:%04d%02d%02d%02d%02d%02d%s".as_ptr(),
        time.tm_year + 1900,
        time.tm_mon + 1,
        time.tm_mday,
        time.tm_hour,
        time.tm_min,
        time.tm_sec,
        timezone.as_ptr(),
    )
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
pub unsafe extern "C-unwind" fn dpx_stack_init(stack: *mut dpx_stack) {
    stack.write(dpx_stack {
        size: 0,
        top: core::ptr::null_mut(),
        bottom: core::ptr::null_mut(),
    });
}

#[unsafe(export_name = "__pdf_impl_dpx_stack_push")]
pub unsafe extern "C-unwind" fn dpx_stack_push(
    stack: *mut dpx_stack,
    data: *mut ::core::ffi::c_void,
) {
    let stack = stack.as_mut().unwrap_or_else(|| {
        pdf_abort!(c"assertion failed: %s".as_ptr(), c"stack".as_ptr());
    });
    let elem = new(core::mem::size_of::<stack_elem>() as u32).cast::<stack_elem>();
    elem.write(stack_elem {
        data,
        prev: stack.top,
    });
    stack.top = elem;
    if stack.size == 0 {
        stack.bottom = elem;
    }
    stack.size += 1;
}

#[unsafe(export_name = "__pdf_impl_dpx_stack_pop")]
pub unsafe extern "C-unwind" fn dpx_stack_pop(stack: *mut dpx_stack) -> *mut ::core::ffi::c_void {
    let stack = stack.as_mut().unwrap_or_else(|| {
        pdf_abort!(c"assertion failed: %s".as_ptr(), c"stack".as_ptr());
    });
    if stack.size == 0 {
        return core::ptr::null_mut();
    }
    let elem = stack.top;
    let data = (*elem).data;
    stack.top = (*elem).prev;
    stack.size -= 1;
    if stack.size == 0 {
        stack.bottom = core::ptr::null_mut();
    }
    free(elem.cast());
    data
}

#[unsafe(export_name = "__pdf_impl_dpx_stack_top")]
pub unsafe extern "C-unwind" fn dpx_stack_top(stack: *mut dpx_stack) -> *mut ::core::ffi::c_void {
    if stack.is_null() {
        pdf_abort!(c"assertion failed: %s".as_ptr(), c"stack".as_ptr());
    }
    dpx_stack_at(stack, 0)
}

#[unsafe(export_name = "__pdf_impl_dpx_stack_at")]
pub unsafe extern "C-unwind" fn dpx_stack_at(
    stack: *mut dpx_stack,
    pos: i32,
) -> *mut ::core::ffi::c_void {
    let stack = &*stack;
    if pos >= stack.size {
        return core::ptr::null_mut();
    }
    let mut elem = stack.top;
    for _ in 0..pos {
        if elem.is_null() {
            return core::ptr::null_mut();
        }
        elem = (*elem).prev;
    }
    elem.as_ref()
        .map_or(core::ptr::null_mut(), |elem| elem.data)
}

#[unsafe(export_name = "__pdf_impl_dpx_stack_roll")]
pub unsafe extern "C-unwind" fn dpx_stack_roll(stack: *mut dpx_stack, n: i32, j: i32) {
    let stack = &mut *stack;
    if n <= 1 || n > stack.size {
        return;
    }
    let split = j.rem_euclid(n);
    if split == 0 {
        return;
    }
    let top = stack.top;
    let mut first_tail = top;
    for _ in 1..split {
        first_tail = (*first_tail).prev;
    }
    let mut last = first_tail;
    for _ in split..n {
        last = (*last).prev;
    }
    stack.top = (*first_tail).prev;
    (*first_tail).prev = (*last).prev;
    (*last).prev = top;
    if n == stack.size {
        stack.bottom = first_tail;
    }
}

#[unsafe(export_name = "__pdf_impl_dpx_stack_depth")]
pub unsafe extern "C-unwind" fn dpx_stack_depth(stack: *mut dpx_stack) -> i32 {
    if stack.is_null() {
        pdf_abort!(c"assertion failed: %s".as_ptr(), c"stack".as_ptr());
    }
    (*stack).size
}
#[unsafe(export_name = "__pdf_impl_ht_init_table")]
pub unsafe extern "C-unwind" fn ht_init_table(ht: *mut ht_table, release: hval_free_func) {
    if ht.is_null() {
        pdf_abort!(c"assertion failed: %s".as_ptr(), c"ht".as_ptr());
    }
    ht.write(ht_table {
        count: 0,
        hval_free_fn: release,
        table: [core::ptr::null_mut(); 503],
    });
}

#[unsafe(export_name = "__pdf_impl_ht_clear_table")]
pub unsafe extern "C-unwind" fn ht_clear_table(ht: *mut ht_table) {
    if ht.is_null() {
        pdf_abort!(c"assertion failed: %s".as_ptr(), c"ht".as_ptr());
    }
    for i in 0..503 {
        let mut entry = (*ht).table[i];
        while !entry.is_null() {
            if !(*entry).value.is_null() {
                if let Some(release) = (*ht).hval_free_fn {
                    release((*entry).value);
                }
            }
            (*entry).value = core::ptr::null_mut();
            free((*entry).key.cast());
            (*entry).key = core::ptr::null_mut();
            let next = (*entry).next;
            free(entry.cast());
            entry = next;
        }
        (*ht).table[i] = core::ptr::null_mut();
    }
    (*ht).count = 0;
    (*ht).hval_free_fn = None;
}

#[unsafe(export_name = "__pdf_impl_ht_table_size")]
pub unsafe extern "C-unwind" fn ht_table_size(ht: *mut ht_table) -> i32 {
    if ht.is_null() {
        pdf_abort!(c"assertion failed: %s".as_ptr(), c"ht".as_ptr());
    }
    (*ht).count
}

fn hash_key(key: &[::core::ffi::c_char]) -> usize {
    let hash = key.iter().fold(0u32, |hash, &byte| {
        hash.wrapping_mul(33).wrapping_add(byte as u32)
    });
    (hash % HASH_TABLE_SIZE as u32) as usize
}

unsafe fn hash_bucket(
    ht: *mut ht_table,
    key: *const ::core::ffi::c_void,
    keylen: i32,
) -> *mut *mut ht_entry {
    if ht.is_null() || key.is_null() {
        pdf_abort!(c"assertion failed: %s".as_ptr(), c"ht && key".as_ptr());
    }
    let len =
        usize::try_from(keylen).unwrap_or_else(|_| pdf_abort!(c"Invalid hash key length".as_ptr()));
    let bytes = core::slice::from_raw_parts(key.cast::<::core::ffi::c_char>(), len);
    &raw mut (*ht).table[hash_key(bytes)]
}

unsafe fn find_hash_slot(
    mut slot: *mut *mut ht_entry,
    key: *const ::core::ffi::c_void,
    keylen: i32,
) -> *mut *mut ht_entry {
    while !(*slot).is_null() {
        let entry = *slot;
        if (*entry).keylen == keylen && memcmp((*entry).key.cast(), key, keylen as usize) == 0 {
            break;
        }
        slot = &raw mut (*entry).next;
    }
    slot
}

unsafe fn new_hash_entry(
    key: *const ::core::ffi::c_void,
    keylen: i32,
    value: *mut ::core::ffi::c_void,
) -> *mut ht_entry {
    let entry = new(core::mem::size_of::<ht_entry>() as u32).cast::<ht_entry>();
    let stored_key = new(keylen as u32).cast::<::core::ffi::c_char>();
    core::ptr::copy_nonoverlapping(key.cast(), stored_key, keylen as usize);
    entry.write(ht_entry {
        key: stored_key,
        keylen,
        value,
        next: core::ptr::null_mut(),
    });
    entry
}

#[unsafe(export_name = "__pdf_impl_ht_lookup_table")]
pub unsafe extern "C-unwind" fn ht_lookup_table(
    ht: *mut ht_table,
    key: *const ::core::ffi::c_void,
    keylen: i32,
) -> *mut ::core::ffi::c_void {
    let slot = find_hash_slot(hash_bucket(ht, key, keylen), key, keylen);
    (*slot)
        .as_ref()
        .map_or(core::ptr::null_mut(), |entry| entry.value)
}

#[unsafe(export_name = "__pdf_impl_ht_remove_table")]
pub unsafe extern "C-unwind" fn ht_remove_table(
    ht: *mut ht_table,
    key: *const ::core::ffi::c_void,
    keylen: i32,
) -> i32 {
    let slot = find_hash_slot(hash_bucket(ht, key, keylen), key, keylen);
    let entry = *slot;
    if entry.is_null() {
        return 0;
    }
    free((*entry).key.cast());
    (*entry).key = core::ptr::null_mut();
    (*entry).keylen = 0;
    if !(*entry).value.is_null() {
        if let Some(release) = (*ht).hval_free_fn {
            release((*entry).value);
        }
    }
    (*entry).value = core::ptr::null_mut();
    *slot = (*entry).next;
    free(entry.cast());
    (*ht).count -= 1;
    1
}

#[unsafe(export_name = "__pdf_impl_ht_insert_table")]
pub unsafe extern "C-unwind" fn ht_insert_table(
    ht: *mut ht_table,
    key: *const ::core::ffi::c_void,
    keylen: i32,
    value: *mut ::core::ffi::c_void,
) {
    let slot = find_hash_slot(hash_bucket(ht, key, keylen), key, keylen);
    let entry = *slot;
    if entry.is_null() {
        *slot = new_hash_entry(key, keylen, value);
        (*ht).count += 1;
    } else {
        if !(*entry).value.is_null() {
            if let Some(release) = (*ht).hval_free_fn {
                release((*entry).value);
            }
        }
        (*entry).value = value;
    }
}

#[unsafe(export_name = "__pdf_impl_ht_append_table")]
pub unsafe extern "C-unwind" fn ht_append_table(
    ht: *mut ht_table,
    key: *const ::core::ffi::c_void,
    keylen: i32,
    value: *mut ::core::ffi::c_void,
) {
    let mut slot = hash_bucket(ht, key, keylen);
    while !(*slot).is_null() {
        slot = &raw mut (**slot).next;
    }
    *slot = new_hash_entry(key, keylen, value);
    (*ht).count += 1;
}

#[unsafe(export_name = "__pdf_impl_ht_set_iter")]
pub unsafe extern "C-unwind" fn ht_set_iter(ht: *mut ht_table, iter: *mut ht_iter) -> i32 {
    if ht.is_null() || iter.is_null() {
        pdf_abort!(c"assertion failed: %s".as_ptr(), c"ht && iter".as_ptr());
    }
    for (index, &entry) in (*ht).table.iter().enumerate() {
        if !entry.is_null() {
            iter.write(ht_iter {
                index: index as i32,
                curr: entry.cast(),
                hash: ht,
            });
            return 0;
        }
    }
    -1
}

#[unsafe(export_name = "__pdf_impl_ht_clear_iter")]
pub unsafe extern "C-unwind" fn ht_clear_iter(iter: *mut ht_iter) {
    if !iter.is_null() {
        iter.write(ht_iter {
            index: HASH_TABLE_SIZE,
            curr: core::ptr::null_mut(),
            hash: core::ptr::null_mut(),
        });
    }
}

#[unsafe(export_name = "__pdf_impl_ht_iter_getkey")]
pub unsafe extern "C-unwind" fn ht_iter_getkey(
    iter: *mut ht_iter,
    keylen: *mut i32,
) -> *mut ::core::ffi::c_char {
    let entry = iter
        .as_ref()
        .and_then(|iter| iter.curr.cast::<ht_entry>().as_ref());
    if let Some(entry) = entry {
        *keylen = entry.keylen;
        entry.key
    } else {
        *keylen = 0;
        core::ptr::null_mut()
    }
}

#[unsafe(export_name = "__pdf_impl_ht_iter_getval")]
pub unsafe extern "C-unwind" fn ht_iter_getval(iter: *mut ht_iter) -> *mut ::core::ffi::c_void {
    iter.as_ref()
        .and_then(|iter| iter.curr.cast::<ht_entry>().as_ref())
        .map_or(core::ptr::null_mut(), |entry| entry.value)
}

#[unsafe(export_name = "__pdf_impl_ht_iter_next")]
pub unsafe extern "C-unwind" fn ht_iter_next(iter: *mut ht_iter) -> i32 {
    let iter = iter.as_mut().unwrap_or_else(|| {
        pdf_abort!(c"assertion failed: %s".as_ptr(), c"iter".as_ptr());
    });
    let Some(current) = iter.curr.cast::<ht_entry>().as_ref() else {
        return -1;
    };
    let mut entry = current.next;
    while entry.is_null() {
        iter.index += 1;
        if iter.index >= HASH_TABLE_SIZE {
            break;
        }
        entry = (*iter.hash).table[iter.index as usize];
    }
    iter.curr = entry.cast();
    if entry.is_null() { -1 } else { 0 }
}
pub(super) fn copy_c_string(bytes: &[u8]) -> *mut ::core::ffi::c_char {
    let size = bytes
        .len()
        .checked_add(1)
        .and_then(|len| u32::try_from(len).ok())
        .unwrap_or_else(|| pdf_abort!(c"String too long".as_ptr()));
    unsafe {
        let result = new(size).cast::<::core::ffi::c_char>();
        core::ptr::copy_nonoverlapping(bytes.as_ptr(), result.cast(), bytes.len());
        result.add(bytes.len()).write(0);
        result
    }
}

fn parse_c_literal(input: &[u8]) -> (Option<Vec<u8>>, usize) {
    let mut output = Vec::new();
    let mut cursor = 0;
    while let Some(&byte) = input.get(cursor) {
        match byte {
            b'"' => return (Some(output), cursor + 1),
            b'\n' | b'\r' => return (None, cursor),
            b'\\' => {
                cursor += 1;
                let Some(&escaped) = input.get(cursor) else {
                    break;
                };
                cursor += 1;
                let decoded = match escaped {
                    b'a' => 7,
                    b'b' => 8,
                    b'f' => 12,
                    b'n' => b'\n',
                    b'r' => b'\r',
                    b't' => b'\t',
                    b'v' => 11,
                    b'\\' | b'?' | b'\'' | b'"' => escaped,
                    b'\n' => continue,
                    b'\r' => {
                        if input.get(cursor) == Some(&b'\n') {
                            cursor += 1;
                        }
                        continue;
                    }
                    b'0'..=b'7' => {
                        let mut value = escaped - b'0';
                        for _ in 1..3 {
                            let Some(&digit @ b'0'..=b'7') = input.get(cursor) else {
                                break;
                            };
                            value = value.wrapping_mul(8).wrapping_add(digit - b'0');
                            cursor += 1;
                        }
                        value
                    }
                    b'x' => {
                        let mut value = 0;
                        for _ in 0..2 {
                            let Some(&byte) = input.get(cursor) else {
                                break;
                            };
                            let digit = xtoi(byte as _);
                            if digit < 0 {
                                break;
                            }
                            value = (value << 4) | digit as u8;
                            cursor += 1;
                        }
                        value
                    }
                    _ => {
                        pdf_warning!(
                            c"Unknown escape char sequence: \\%c".as_ptr(),
                            escaped as ::core::ffi::c_int
                        );
                        continue;
                    }
                };
                output.push(decoded);
            }
            _ => {
                output.push(byte);
                cursor += 1;
            }
        }
    }
    (None, cursor)
}

#[unsafe(export_name = "__pdf_impl_parse_c_string")]
pub unsafe extern "C-unwind" fn parse_c_string(
    pp: *mut *const ::core::ffi::c_char,
    endptr: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if *pp >= endptr || **pp != b'"' as ::core::ffi::c_char {
        return core::ptr::null_mut();
    }
    let start = (*pp).add(1);
    let input = core::slice::from_raw_parts(start.cast::<u8>(), endptr.offset_from(start) as usize);
    let (parsed, consumed) = parse_c_literal(input);
    *pp = start.add(consumed);
    parsed.map_or(core::ptr::null_mut(), |bytes| copy_c_string(&bytes))
}

#[unsafe(export_name = "__pdf_impl_parse_c_ident")]
pub unsafe extern "C-unwind" fn parse_c_ident(
    pp: *mut *const ::core::ffi::c_char,
    endptr: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if *pp >= endptr {
        return core::ptr::null_mut();
    }
    let input = core::slice::from_raw_parts((*pp).cast::<u8>(), endptr.offset_from(*pp) as usize);
    if !input[0].is_ascii_alphabetic() && input[0] != b'_' {
        return core::ptr::null_mut();
    }
    let len = input
        .iter()
        .take_while(|&&byte| byte.is_ascii_alphanumeric() || byte == b'_')
        .count();
    let result = copy_c_string(&input[..len]);
    *pp = (*pp).add(len);
    result
}

fn decimal_token(input: &[u8]) -> (usize, bool) {
    #[derive(PartialEq)]
    enum Part {
        Integer,
        Fraction,
        Exponent,
        ExponentSign,
    }

    let mut cursor = usize::from(matches!(input.first(), Some(b'+' | b'-')));
    let mut part = Part::Integer;
    let mut has_digits = false;
    while let Some(&byte) = input.get(cursor) {
        match byte {
            b'0'..=b'9' => has_digits = true,
            b'.' if part == Part::Integer => part = Part::Fraction,
            b'e' | b'E' if has_digits && part != Part::Exponent => part = Part::Exponent,
            b'+' | b'-' if part == Part::Exponent => part = Part::ExponentSign,
            _ => break,
        }
        cursor += 1;
    }
    (cursor, has_digits)
}

#[unsafe(export_name = "__pdf_impl_parse_float_decimal")]
pub unsafe extern "C-unwind" fn parse_float_decimal(
    pp: *mut *const ::core::ffi::c_char,
    endptr: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if *pp >= endptr {
        return core::ptr::null_mut();
    }
    let input = core::slice::from_raw_parts((*pp).cast::<u8>(), endptr.offset_from(*pp) as usize);
    let (len, has_digits) = decimal_token(input);
    let result = if has_digits {
        copy_c_string(&input[..len])
    } else {
        core::ptr::null_mut()
    };
    *pp = (*pp).add(len);
    result
}
unsafe extern "C" {
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_xtoi(
    mut c: ::core::ffi::c_char,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| xtoi(c)) {
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
    match crate::backend::runtime::boundary(|| min4(x1, x2, x3, x4)) {
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
    match crate::backend::runtime::boundary(|| max4(x1, x2, x3, x4)) {
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
                crate::compat::allocation::release(value.cast());
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
                crate::compat::allocation::release(value.cast());
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
                crate::compat::allocation::release(value.cast());
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
