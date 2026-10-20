// Copyright 2002-2016 Jin-Hwan Cho, Shunsaku Hirata, and the dvipdfmx team
// SPDX-License-Identifier: GPL-2.0-or-later

pub(crate) use super::dpx_pdfobj::pdf_obj;
use super::dpx_pdfobj::{pdf_link_obj, pdf_ref_obj, pdf_release_obj};
use crate::{
    compat::objects as handles,
    objects::{Resource, ResourceId},
};
use core::ffi::c_char;
use std::{collections::HashMap, ffi::CStr};

const CATEGORIES: [&[u8]; 9] = [
    b"Font",
    b"CIDFont",
    b"Encoding",
    b"CMap",
    b"XObject",
    b"ColorSpace",
    b"Shading",
    b"Pattern",
    b"ExtGState",
];

pub(crate) struct DocumentState {
    resources: [Vec<Resource>; 9],
    names: [HashMap<Box<[u8]>, usize>; 9],
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self {
            resources: std::array::from_fn(|_| Vec::new()),
            names: std::array::from_fn(|_| HashMap::new()),
        }
    }
}

unsafe fn document_state() -> *mut DocumentState {
    unsafe { (*crate::compat::document::current()).backend.dpx_pdfresource.pointer }
}

unsafe fn category_id(category: *const c_char) -> usize {
    let name = unsafe { CStr::from_ptr(category).to_bytes() };
    CATEGORIES
        .iter()
        .position(|&candidate| candidate == name)
        .unwrap_or_else(|| pdf_abort!(c"Unknown resource category: %s".as_ptr(), category))
}

unsafe fn flush(resource: &Resource) {
    unsafe {
        pdf_release_obj(handles::pointer(resource.reference));
        pdf_release_obj(handles::pointer(resource.object));
    }
}

unsafe fn replacement_warning(resname: *const c_char, category: *const c_char) {
    pdf_warning!(c"Resource %s (category: %s) already defined...".as_ptr(), resname, category);
}

#[unsafe(export_name = "__pdf_impl_pdf_init_resources")]
pub unsafe extern "C-unwind" fn pdf_init_resources() {
    unsafe {
        pdf_close_resources();
    }
}

#[unsafe(export_name = "__pdf_impl_pdf_close_resources")]
pub unsafe extern "C-unwind" fn pdf_close_resources() {
    for category in 0..CATEGORIES.len() {
        // Release in category and insertion order, without holding a container borrow.
        let entries = unsafe { std::mem::take(&mut (*document_state()).resources[category]) };
        unsafe { (*document_state()).names[category].clear() };
        for entry in entries {
            unsafe {
                flush(&entry);
            }
        }
    }
}

#[unsafe(export_name = "__pdf_impl_pdf_defineresource")]
pub unsafe extern "C-unwind" fn pdf_defineresource(category: *const c_char, resname: *const c_char, object: *mut pdf_obj, flags: i32) -> i32 {
    unsafe {
        assert!(!category.is_null() && !object.is_null());
        let category_index = category_id(category);
        let name = if resname.is_null() {
            None
        } else {
            Some(CStr::from_ptr(resname).to_bytes())
        };
        let entries = &(*document_state()).resources[category_index];
        let found = name.and_then(|name| (*document_state()).names[category_index].get(name).copied());
        let index = found.unwrap_or(entries.len());
        let id = ResourceId::new(category_index, index).unwrap_or_else(|| pdf_abort!("Too many PDF resources"));
        if let Some(index) = found {
            replacement_warning(resname, category);
            let old = std::mem::replace(
                &mut (&mut (*document_state()).resources[category_index])[index],
                Resource {
                    object: None,
                    reference: None,
                },
            );
            flush(&old);
        }
        let mut entry = Resource {
            object: handles::id(object),
            reference: None,
        };
        if flags & 1 != 0 {
            entry.reference = handles::id(pdf_ref_obj(object));
            entry.object = None;
            pdf_release_obj(object);
        }
        let entries = &mut (*document_state()).resources[category_index];
        if found.is_some() {
            entries[index] = entry;
        } else {
            entries.push(entry);
            if let Some(name) = name.filter(|name| !name.is_empty()) {
                (*document_state()).names[category_index].insert(Box::from(name), index);
            }
        }
        id.0 as i32
    }
}

#[unsafe(export_name = "__pdf_impl_pdf_findresource")]
pub unsafe extern "C-unwind" fn pdf_findresource(category: *const c_char, resname: *const c_char) -> i32 {
    unsafe {
        assert!(!category.is_null() && !resname.is_null());
        let category = category_id(category);
        let name = CStr::from_ptr(resname).to_bytes();
        (*document_state()).names[category]
            .get(name)
            .copied()
            .and_then(|index| ResourceId::new(category, index))
            .map_or(-1, |id| id.0 as i32)
    }
}

#[unsafe(export_name = "__pdf_impl_pdf_get_resource_reference")]
pub unsafe extern "C-unwind" fn pdf_get_resource_reference(rc_id: i32) -> *mut pdf_obj {
    unsafe {
        let id = ResourceId(rc_id as u32);
        if id.category() >= CATEGORIES.len() {
            pdf_abort!("Invalid category ID: {}", id.category());
        }
        let entries = &(*document_state()).resources[id.category()];
        let Some(entry) = entries.get(id.index()) else {
            pdf_abort!("Invalid resource ID: {}", id.index());
        };
        let mut reference = entry.reference;
        if reference.is_none() {
            let object = handles::pointer(entry.object);
            if object.is_null() {
                pdf_abort!("Undefined object...");
            }
            reference = handles::id(pdf_ref_obj(object));
            (&mut (*document_state()).resources[id.category()])[id.index()].reference = reference;
        }
        pdf_link_obj(handles::pointer(reference))
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_init_resources() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_init_resources() }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_close_resources() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_close_resources() }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_defineresource(
    mut category: *const ::core::ffi::c_char, mut resname: *const ::core::ffi::c_char, mut object: *mut pdf_obj, mut flags: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_defineresource(category, resname, object, flags) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_findresource(
    mut category: *const ::core::ffi::c_char, mut resname: *const ::core::ffi::c_char, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_findresource(category, resname) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_resource_reference(mut rc_id: ::core::ffi::c_int, _result: *mut *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_resource_reference(rc_id) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
