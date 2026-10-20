/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
use super::dpx_pdfobj::{
    pdf_add_array, pdf_add_dict, pdf_link_obj, pdf_new_array, pdf_new_dict, pdf_new_name, pdf_new_null, pdf_new_string, pdf_new_undefined,
    pdf_obj_typeof, pdf_ref_obj, pdf_release_obj, pdf_string_length, pdf_string_value, pdf_transfer_label,
};
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn isprint(_: ::core::ffi::c_int) -> ::core::ffi::c_int;

    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    #[link_name = "__pdf_impl_ht_lookup_table"]
    fn ht_lookup_table(ht: *mut ht_table, key: *const ::core::ffi::c_void, keylen: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;

    #[link_name = "__pdf_impl_ht_set_iter"]
    fn ht_set_iter(ht: *mut ht_table, iter: *mut ht_iter) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_ht_clear_iter"]
    fn ht_clear_iter(iter: *mut ht_iter);
    #[link_name = "__pdf_impl_ht_iter_getkey"]
    fn ht_iter_getkey(iter: *mut ht_iter, keylen: *mut ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_ht_iter_getval"]
    fn ht_iter_getval(iter: *mut ht_iter) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_ht_iter_next"]
    fn ht_iter_next(iter: *mut ht_iter) -> ::core::ffi::c_int;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct obj_data {
    pub reference: *mut pdf_obj,
    pub object: *mut pdf_obj,
    pub closed: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct named_object {
    pub key: *mut ::core::ffi::c_char,
    pub keylen: ::core::ffi::c_int,
    pub value: *mut pdf_obj,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const PDF_STRING: ::core::ffi::c_int = 3;
pub const PDF_ARRAY: ::core::ffi::c_int = 5;
pub const PDF_DICT: ::core::ffi::c_int = 6;
pub const PDF_STREAM: ::core::ffi::c_int = 7;
pub const PDF_UNDEFINED: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PDF_OBJ_INVALID: ::core::ffi::c_int = 0;
unsafe fn printable_key(mut key: *const ::core::ffi::c_char, mut keylen: ::core::ffi::c_int) -> [::core::ffi::c_char; 36] {
    let mut pkey: [::core::ffi::c_char; 36] = [0; 36];
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut hi: ::core::ffi::c_uchar = 0;
    let mut lo: ::core::ffi::c_uchar = 0;
    i = 0 as ::core::ffi::c_int;
    len = 0 as ::core::ffi::c_int;
    while i < keylen && len < MAX_KEY {
        if isprint(*key.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0 {
            let fresh0 = len;
            len = len + 1;
            pkey[fresh0 as usize] = *key.offset(i as isize);
        } else {
            hi = (*key.offset(i as isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            lo = (*key.offset(i as isize) as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            let fresh1 = len;
            len = len + 1;
            pkey[fresh1 as usize] = '#' as i32 as ::core::ffi::c_char;
            let fresh2 = len;
            len = len + 1;
            pkey[fresh2 as usize] = (if (hi as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                hi as ::core::ffi::c_int + '0' as i32
            } else {
                hi as ::core::ffi::c_int - 10 as ::core::ffi::c_int + 'A' as i32
            }) as ::core::ffi::c_char;
            let fresh3 = len;
            len = len + 1;
            pkey[fresh3 as usize] = (if (lo as ::core::ffi::c_int) < 10 as ::core::ffi::c_int {
                lo as ::core::ffi::c_int + '0' as i32
            } else {
                lo as ::core::ffi::c_int - 10 as ::core::ffi::c_int + 'A' as i32
            }) as ::core::ffi::c_char;
        }
        i += 1;
    }
    pkey[len as usize] = '\0' as i32 as ::core::ffi::c_char;
    return pkey;
}
pub const MAX_KEY: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn hval_free(mut hval: *mut ::core::ffi::c_void) {
    let mut value: *mut obj_data = ::core::ptr::null_mut::<obj_data>();
    value = hval as *mut obj_data;
    if !(*value).reference.is_null() {
        pdf_release_obj((*value).reference);
        (*value).reference = ::core::ptr::null_mut::<pdf_obj>();
    }
    if !(*value).object.is_null() {
        pdf_release_obj((*value).object);
        (*value).object = ::core::ptr::null_mut::<pdf_obj>();
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_new_name_tree")]
pub unsafe extern "C-unwind" fn pdf_new_name_tree() -> *mut ht_table {
    let storage = storage();
    let index = (*storage).tables.insert(ht_table {
        count: 0,
        hval_free_fn: Some(hval_free),
        table: [core::ptr::null_mut(); 503],
    });
    let table = (*storage).tables.pointer(index);
    (*(*storage).indices.get()).insert(table as usize, NameIndex::default());
    table
}
unsafe extern "C-unwind" fn check_objects_defined(mut ht_tab: *mut ht_table) {
    let mut iter: ht_iter = ht_iter {
        index: 0,
        curr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        hash: ::core::ptr::null_mut::<ht_table>(),
    };
    if ht_set_iter(ht_tab, &raw mut iter) >= 0 as ::core::ffi::c_int {
        loop {
            let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut keylen: ::core::ffi::c_int = 0;
            let mut value: *mut obj_data = ::core::ptr::null_mut::<obj_data>();
            key = ht_iter_getkey(&raw mut iter, &raw mut keylen);
            value = ht_iter_getval(&raw mut iter) as *mut obj_data;
            if !(*value).object.is_null() && pdf_obj_typeof((*value).object) == PDF_UNDEFINED {
                pdf_names_add_object(ht_tab, key as *const ::core::ffi::c_void, keylen, pdf_new_null());
                pdf_warning!(
                    b"Object @%s used, but not defined. Replaced by null.\0" as *const u8 as *const ::core::ffi::c_char,
                    printable_key(key, keylen).as_ptr(),
                );
            }
            if !(ht_iter_next(&raw mut iter) >= 0 as ::core::ffi::c_int) {
                break;
            }
        }
        ht_clear_iter(&raw mut iter);
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_delete_name_tree")]
pub unsafe extern "C-unwind" fn pdf_delete_name_tree(mut names: *mut *mut ht_table) {
    assert!(!names.is_null() && !(*names).is_null());
    check_objects_defined(*names);
    for bucket in &mut (**names).table {
        let mut entry = *bucket;
        while !entry.is_null() {
            hval_free((*entry).value);
            entry = (*entry).next;
        }
        *bucket = core::ptr::null_mut();
    }
    (**names).count = 0;
    (*(*storage()).indices.get()).remove(&(*names as usize));
    *names = core::ptr::null_mut();
}
#[unsafe(export_name = "__pdf_impl_pdf_names_add_object")]
pub unsafe extern "C-unwind" fn pdf_names_add_object(
    mut names: *mut ht_table, mut key: *const ::core::ffi::c_void, mut keylen: ::core::ffi::c_int, mut object: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut value: *mut obj_data = ::core::ptr::null_mut::<obj_data>();
    if !(!names.is_null() && !object.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"names && object\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if key.is_null() || keylen < 1 as ::core::ffi::c_int {
        pdf_warning!(b"Null string used for name tree key.\0" as *const u8 as *const ::core::ffi::c_char,);
        return -(1 as ::core::ffi::c_int);
    }
    value = ht_lookup_table(names, key, keylen) as *mut obj_data;
    if value.is_null() {
        value = new_name_value();
        (*value).object = object;
        (*value).reference = ::core::ptr::null_mut::<pdf_obj>();
        (*value).closed = 0 as ::core::ffi::c_int;
        append_name(names, key, keylen, value);
    } else if !(*value).object.is_null() && (!(*value).object.is_null() && pdf_obj_typeof((*value).object) == PDF_UNDEFINED) {
        pdf_transfer_label(object, (*value).object);
        pdf_release_obj((*value).object);
        (*value).object = object;
    } else {
        pdf_warning!(
            b"Object @%s already defined.\0" as *const u8 as *const ::core::ffi::c_char,
            printable_key(key as *const ::core::ffi::c_char, keylen).as_ptr(),
        );
        pdf_release_obj(object);
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_names_reserve")]
pub unsafe extern "C-unwind" fn pdf_names_reserve(
    mut names: *mut ht_table, mut key: *const ::core::ffi::c_void, mut keylen: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut obj_ref: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut value: *mut obj_data = ::core::ptr::null_mut::<obj_data>();
    if names.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"names\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if key.is_null() || keylen < 1 as ::core::ffi::c_int {
        pdf_warning!(b"Null string used for name tree key.\0" as *const u8 as *const ::core::ffi::c_char,);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    value = ht_lookup_table(names, key, keylen) as *mut obj_data;
    if value.is_null() {
        value = new_name_value();
        (*value).object = pdf_new_undefined();
        (*value).reference = ::core::ptr::null_mut::<pdf_obj>();
        (*value).closed = 0 as ::core::ffi::c_int;
        append_name(names, key, keylen, value);
        obj_ref = pdf_ref_obj((*value).object);
    } else if !(*value).object.is_null() && (!(*value).object.is_null() && pdf_obj_typeof((*value).object) == PDF_UNDEFINED) {
        if (*value).reference.is_null() {
            (*value).reference = pdf_ref_obj((*value).object);
        }
        obj_ref = pdf_link_obj((*value).reference);
    } else {
        pdf_warning!(
            b"Object @%s already defined.\0" as *const u8 as *const ::core::ffi::c_char,
            printable_key(key as *const ::core::ffi::c_char, keylen).as_ptr(),
        );
        obj_ref = ::core::ptr::null_mut::<pdf_obj>();
    }
    return obj_ref;
}
#[unsafe(export_name = "__pdf_impl_pdf_names_lookup_reference")]
pub unsafe extern "C-unwind" fn pdf_names_lookup_reference(
    mut names: *mut ht_table, mut key: *const ::core::ffi::c_void, mut keylen: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut value: *mut obj_data = ::core::ptr::null_mut::<obj_data>();
    let mut obj_ref: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if names.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"names\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    value = ht_lookup_table(names, key, keylen) as *mut obj_data;
    if !value.is_null() {
        if (*value).reference.is_null() {
            if !(*value).object.is_null() {
                (*value).reference = pdf_ref_obj((*value).object);
            } else {
                pdf_warning!(
                    b"Can't create object ref for already released object: %s\0" as *const u8 as *const ::core::ffi::c_char,
                    printable_key(key as *const ::core::ffi::c_char, keylen).as_ptr(),
                );
            }
        }
        obj_ref = pdf_link_obj((*value).reference);
    } else {
        obj_ref = pdf_names_reserve(names, key, keylen);
    }
    return obj_ref;
}
#[unsafe(export_name = "__pdf_impl_pdf_names_lookup_object")]
pub unsafe extern "C-unwind" fn pdf_names_lookup_object(
    mut names: *mut ht_table, mut key: *const ::core::ffi::c_void, mut keylen: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut value: *mut obj_data = ::core::ptr::null_mut::<obj_data>();
    if names.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"names\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    value = ht_lookup_table(names, key, keylen) as *mut obj_data;
    if value.is_null() || !(*value).object.is_null() && pdf_obj_typeof((*value).object) == PDF_UNDEFINED {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    if (*value).closed != 0 {
        pdf_warning!(
            b"Object \"%s\" already closed.\0" as *const u8 as *const ::core::ffi::c_char,
            printable_key(key as *const ::core::ffi::c_char, keylen).as_ptr(),
        );
    }
    return (*value).object;
}
#[unsafe(export_name = "__pdf_impl_pdf_names_close_object")]
pub unsafe extern "C-unwind" fn pdf_names_close_object(
    mut names: *mut ht_table, mut key: *const ::core::ffi::c_void, mut keylen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut value: *mut obj_data = ::core::ptr::null_mut::<obj_data>();
    if names.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"names\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    value = ht_lookup_table(names, key, keylen) as *mut obj_data;
    if value.is_null() || !(*value).object.is_null() && pdf_obj_typeof((*value).object) == PDF_UNDEFINED {
        pdf_warning!(
            b"Cannot close undefined object @%s.\0" as *const u8 as *const ::core::ffi::c_char,
            printable_key(key as *const ::core::ffi::c_char, keylen).as_ptr(),
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*value).object.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"value->object\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*value).closed != 0 {
        pdf_warning!(
            b"Object @%s already closed.\0" as *const u8 as *const ::core::ffi::c_char,
            printable_key(key as *const ::core::ffi::c_char, keylen).as_ptr(),
        );
        return -(1 as ::core::ffi::c_int);
    }
    if !(*value).reference.is_null() {
        pdf_release_obj((*value).object);
        (*value).object = ::core::ptr::null_mut::<pdf_obj>();
    }
    (*value).closed = 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
pub const NAME_CLUSTER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn build_name_tree(
    mut first: *mut named_object, mut num_leaves: ::core::ffi::c_int, mut is_root: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut i: ::core::ffi::c_int = 0;
    result = pdf_new_dict();
    if is_root == 0 {
        let mut last: *mut named_object = ::core::ptr::null_mut::<named_object>();
        let mut limits: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        limits = pdf_new_array();
        last = first.offset((num_leaves - 1 as ::core::ffi::c_int) as isize) as *mut named_object;
        pdf_add_array(
            limits,
            pdf_new_string((*first).key as *const ::core::ffi::c_void, (*first).keylen as size_t),
        );
        pdf_add_array(
            limits,
            pdf_new_string((*last).key as *const ::core::ffi::c_void, (*last).keylen as size_t),
        );
        pdf_add_dict(result, pdf_new_name(b"Limits\0" as *const u8 as *const ::core::ffi::c_char), limits);
    }
    if num_leaves > 0 as ::core::ffi::c_int && num_leaves <= 2 as ::core::ffi::c_int * NAME_CLUSTER {
        let mut names: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        names = pdf_new_array();
        i = 0 as ::core::ffi::c_int;
        while i < num_leaves {
            let mut cur: *mut named_object = ::core::ptr::null_mut::<named_object>();
            cur = first.offset(i as isize) as *mut named_object;
            pdf_add_array(names, pdf_new_string((*cur).key as *const ::core::ffi::c_void, (*cur).keylen as size_t));
            match pdf_obj_typeof((*cur).value) {
                PDF_ARRAY | PDF_DICT | PDF_STREAM | PDF_STRING => {
                    pdf_add_array(names, pdf_ref_obj((*cur).value));
                }
                PDF_OBJ_INVALID => {
                    pdf_abort!(
                        b"Invalid object...: %s\0" as *const u8 as *const ::core::ffi::c_char,
                        printable_key((*cur).key, (*cur).keylen).as_ptr(),
                    );
                }
                _ => {
                    pdf_add_array(names, pdf_link_obj((*cur).value));
                }
            }
            pdf_release_obj((*cur).value);
            (*cur).value = ::core::ptr::null_mut::<pdf_obj>();
            i += 1;
        }
        pdf_add_dict(result, pdf_new_name(b"Names\0" as *const u8 as *const ::core::ffi::c_char), names);
    } else if num_leaves > 0 as ::core::ffi::c_int {
        let mut kids: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        kids = pdf_new_array();
        i = 0 as ::core::ffi::c_int;
        while i < NAME_CLUSTER {
            let mut subtree: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            let mut start: ::core::ffi::c_int = 0;
            let mut end: ::core::ffi::c_int = 0;
            start = i * num_leaves / NAME_CLUSTER;
            end = (i + 1 as ::core::ffi::c_int) * num_leaves / NAME_CLUSTER;
            subtree = build_name_tree(first.offset(start as isize) as *mut named_object, end - start, 0 as ::core::ffi::c_int);
            pdf_add_array(kids, pdf_ref_obj(subtree));
            pdf_release_obj(subtree);
            i += 1;
        }
        pdf_add_dict(result, pdf_new_name(b"Kids\0" as *const u8 as *const ::core::ffi::c_char), kids);
    }
    return result;
}
unsafe fn flat_table(ht_tab: *mut ht_table, num_entries: *mut i32, filter: *mut ht_table) -> Vec<named_object> {
    let mut objects = Vec::with_capacity((*ht_tab).count as usize);
    for bucket in (*ht_tab).table {
        let mut entry = bucket;
        while !entry.is_null() {
            let mut key = (*entry).key;
            let mut keylen = (*entry).keylen;
            if !filter.is_null() {
                let replacement = ht_lookup_table(filter, key.cast(), keylen) as *mut pdf_obj;
                if replacement.is_null() {
                    entry = (*entry).next;
                    continue;
                }
                key = pdf_string_value(replacement).cast();
                keylen = pdf_string_length(replacement) as i32;
            }
            let value = (*entry).value as *mut obj_data;
            assert!(!(*value).object.is_null());
            let object = if pdf_obj_typeof((*value).object) == PDF_UNDEFINED {
                pdf_warning!(
                    c"Object @%s\" not defined. Replaced by null.".as_ptr(),
                    printable_key(key, keylen).as_ptr()
                );
                pdf_new_null()
            } else {
                pdf_link_obj((*value).object)
            };
            objects.push(named_object { key, keylen, value: object });
            entry = (*entry).next;
        }
    }
    *num_entries = objects.len() as i32;
    objects
}
#[unsafe(export_name = "__pdf_impl_pdf_names_create_tree")]
pub unsafe extern "C-unwind" fn pdf_names_create_tree(
    mut names: *mut ht_table, mut count: *mut ::core::ffi::c_int, mut filter: *mut ht_table,
) -> *mut pdf_obj {
    let mut flat = flat_table(names, count, filter);
    if flat.is_empty() {
        return core::ptr::null_mut();
    }
    flat.sort_by(|first, second| {
        let first = core::slice::from_raw_parts(first.key.cast::<u8>(), first.keylen as usize);
        let second = core::slice::from_raw_parts(second.key.cast::<u8>(), second.keylen as usize);
        first.cmp(second)
    });
    build_name_tree(flat.as_mut_ptr(), *count, 1)
}
unsafe extern "C" {
    fn memcmp(__s1: *const ::core::ffi::c_void, __s2: *const ::core::ffi::c_void, __n: size_t) -> ::core::ffi::c_int;
}
pub(crate) use super::dpx_pdfobj::pdf_obj;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_new_name_tree(_result: *mut *mut ht_table) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_new_name_tree() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_delete_name_tree(mut names: *mut *mut ht_table) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_delete_name_tree(names) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_names_add_object(
    mut names: *mut ht_table, mut key: *const ::core::ffi::c_void, mut keylen: ::core::ffi::c_int, mut object: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_names_add_object(names, key, keylen, object) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_names_reserve(
    mut names: *mut ht_table, mut key: *const ::core::ffi::c_void, mut keylen: ::core::ffi::c_int, _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_names_reserve(names, key, keylen) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_names_lookup_reference(
    mut names: *mut ht_table, mut key: *const ::core::ffi::c_void, mut keylen: ::core::ffi::c_int, _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_names_lookup_reference(names, key, keylen) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_names_lookup_object(
    mut names: *mut ht_table, mut key: *const ::core::ffi::c_void, mut keylen: ::core::ffi::c_int, _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_names_lookup_object(names, key, keylen) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_names_close_object(
    mut names: *mut ht_table, mut key: *const ::core::ffi::c_void, mut keylen: ::core::ffi::c_int, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_names_close_object(names, key, keylen) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_names_create_tree(
    mut names: *mut ht_table, mut count: *mut ::core::ffi::c_int, mut filter: *mut ht_table, _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_names_create_tree(names, count, filter) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

#[derive(Default)]
pub(crate) struct Storage {
    tables: crate::compat::objects::StableStore<ht_table>,
    entries: crate::compat::objects::StableStore<ht_entry>,
    values: crate::compat::objects::StableStore<obj_data>,
    indices: std::cell::UnsafeCell<std::collections::HashMap<usize, NameIndex>>,
}

struct NameIndex {
    values: std::collections::HashMap<Box<[u8]>, *mut obj_data>,
    tails: [*mut ht_entry; 503],
}

impl Default for NameIndex {
    fn default() -> Self {
        Self {
            values: std::collections::HashMap::new(),
            tails: [core::ptr::null_mut(); 503],
        }
    }
}

pub(crate) unsafe fn lookup_index(table: usize, key: &[u8]) -> Option<*mut core::ffi::c_void> {
    (*(*storage()).indices.get())
        .get(&table)
        .map(|index| index.values.get(key).copied().unwrap_or(core::ptr::null_mut()).cast())
}

unsafe fn storage() -> *mut Storage {
    core::ptr::addr_of_mut!((*crate::compat::document::current()).names)
}

unsafe fn new_name_value() -> *mut obj_data {
    let storage = storage();
    let index = (*storage).values.insert(obj_data {
        reference: core::ptr::null_mut(),
        object: core::ptr::null_mut(),
        closed: 0,
    });
    (*storage).values.pointer(index)
}

unsafe fn append_name(names: *mut ht_table, key: *const core::ffi::c_void, keylen: i32, value: *mut obj_data) {
    let bytes = core::slice::from_raw_parts(key.cast::<u8>(), keylen as usize);
    let hash = bytes
        .iter()
        .fold(0u32, |hash, &byte| hash.wrapping_mul(33).wrapping_add(byte as core::ffi::c_char as u32))
        % 503;
    let mut owned = Box::<[u8]>::from(bytes);
    let key = owned.as_mut_ptr().cast();
    let storage = storage();
    let index = (*storage).entries.insert(ht_entry {
        key,
        keylen,
        value: value.cast(),
        next: core::ptr::null_mut(),
    });
    let entry = (*storage).entries.pointer(index);
    let index = (*(*storage).indices.get()).get_mut(&(names as usize)).expect("PDF name index");
    let tail = &mut index.tails[hash as usize];
    if tail.is_null() {
        (*names).table[hash as usize] = entry;
    } else {
        (**tail).next = entry;
    }
    *tail = entry;
    index.values.insert(owned, value);
    (*names).count += 1;
}
