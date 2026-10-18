//! Own translated allocations until explicit release or document completion.

use super::document;
use core::ffi::c_void;
use std::{
    collections::HashSet,
    hash::{BuildHasherDefault, Hasher},
};

#[derive(Default)]
pub(crate) struct AddressHasher(u64);

impl Hasher for AddressHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write_usize(&mut self, address: usize) {
        self.0 = ((address as u64) >> 3).wrapping_mul(0x9e3779b97f4a7c15);
    }
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 = self.0.wrapping_mul(0x100000001b3) ^ u64::from(byte);
        }
    }
}

pub(crate) type AllocationSet = HashSet<usize, BuildHasherDefault<AddressHasher>>;

pub(crate) fn grow_capacity(
    current: usize,
    required: usize,
    item_size: usize,
    minimum: usize,
) -> usize {
    let limit = u32::MAX as usize / item_size;
    if required > limit {
        crate::backend::runtime::raise_error(crate::error::PdfError::new(
            crate::error::PdfErrorKind::Unsupported,
            c"PDF allocation is too large".to_owned(),
        ));
    }
    required.max(current.saturating_mul(2).max(minimum).min(limit))
}

unsafe fn retain(pointer: *mut c_void) {
    let document = document::try_current();
    if !pointer.is_null() && !document.is_null() {
        // SAFETY: Only the coordinator accesses this allocation registry.
        unsafe {
            (*document).allocations.insert(pointer as usize);
        }
    }
}

pub(crate) unsafe fn release(pointer: *mut c_void) {
    let document = document::try_current();
    if !document.is_null() {
        unsafe {
            (*document).allocations.remove(&(pointer as usize));
        }
    }
}

pub(crate) unsafe fn malloc(size: usize) -> *mut c_void {
    unsafe {
        let pointer = libc::malloc(size);
        retain(pointer);
        pointer
    }
}

pub(crate) unsafe fn realloc(pointer: *mut c_void, size: usize) -> *mut c_void {
    unsafe {
        let result = libc::realloc(pointer, size);
        if !result.is_null() || size == 0 {
            release(pointer);
        }
        retain(result);
        result
    }
}

pub(crate) unsafe fn free(pointer: *mut c_void) {
    unsafe {
        release(pointer);
        libc::free(pointer);
    }
}
