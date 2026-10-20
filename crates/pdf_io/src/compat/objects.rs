//! Stable opaque handles and release-triggered writer compatibility.

use crate::{
    backend::dpx_pdfobj::pdf_stream,
    objects::{Object, ObjectId, StreamId, Value},
};

#[derive(Default)]
pub(crate) struct State {
    objects: StableStore<Object>,
    leases: std::cell::UnsafeCell<Vec<u32>>,
    redirects: std::cell::UnsafeCell<std::collections::HashMap<ObjectId, ObjectId>>,
    streams: StableStore<pdf_stream>,
}

#[inline]
unsafe fn state() -> *mut State {
    unsafe { core::ptr::addr_of_mut!((*super::document::current()).objects) }
}

pub(crate) unsafe fn allocate(value: Value) -> *mut Object {
    unsafe {
        let state = &*state();
        let id = ObjectId(state.objects.len());
        (*state.leases.get()).push(1);
        state.objects.insert(Object {
            id,
            label: 0,
            generation: 0,
            flags: 0,
            value,
        });
        state.objects.pointer(id.0)
    }
}

#[inline]
pub(crate) unsafe fn pointer(id: Option<ObjectId>) -> *mut Object {
    unsafe { id.map_or(core::ptr::null_mut(), |id| (*state()).objects.pointer(id.0)) }
}

#[inline]
pub(crate) unsafe fn id(object: *mut Object) -> Option<ObjectId> {
    unsafe { object.as_ref().map(|object| object.id) }
}

pub(crate) unsafe fn count(object: *mut Object) -> u32 {
    unsafe { (&*(*state()).leases.get())[(*object).id.0] }
}

pub(crate) unsafe fn retain(object: *mut Object) {
    unsafe {
        let count = &mut (&mut *(*state()).leases.get())[(*object).id.0];
        *count = count.checked_add(1).expect("PDF compatibility lease overflow");
    }
}

pub(crate) unsafe fn release(object: *mut Object) -> bool {
    unsafe {
        let count = &mut (&mut *(*state()).leases.get())[(*object).id.0];
        *count = count.checked_sub(1).expect("Released PDF compatibility handle");
        *count == 0
    }
}

pub(crate) unsafe fn retire(object: *mut Object) -> Value {
    unsafe { std::mem::replace(&mut (*object).value, Value::Retired) }
}

pub(crate) unsafe fn transfer(source: ObjectId, destination: ObjectId) {
    if source != destination {
        unsafe {
            (*(*state()).redirects.get()).insert(source, destination);
        }
    }
}

pub(crate) unsafe fn resolve(mut id: ObjectId) -> ObjectId {
    unsafe {
        let redirects = &*(*state()).redirects.get();
        for _ in 0..=redirects.len() {
            match redirects.get(&id) {
                Some(&next) => id = next,
                None => return id,
            }
        }
        crate::backend::runtime::raise_error(crate::error::PdfError::new(
            crate::error::PdfErrorKind::InvalidState,
            c"Cycle in transferred PDF identities".to_owned(),
        ))
    }
}

pub(crate) unsafe fn insert_stream(stream: pdf_stream) -> StreamId {
    unsafe { StreamId((*state()).streams.insert(stream)) }
}

pub(crate) unsafe fn stream(object: *mut Object) -> *mut pdf_stream {
    unsafe {
        let Value::Stream(stream) = (*object).value else {
            panic!("Expected PDF stream");
        };
        (*state()).streams.pointer(stream.storage.0)
    }
}

pub(crate) unsafe fn stream_storage(id: StreamId) -> *mut pdf_stream {
    unsafe { (*state()).streams.pointer(id.0) }
}

use std::{
    cell::{Cell, UnsafeCell},
    mem::MaybeUninit,
};

const CHUNK_SIZE: usize = 128;
type Chunk<T> = Vec<UnsafeCell<MaybeUninit<T>>>;

fn chunk_size<T>() -> usize {
    let slots = (16 * 1024 / core::mem::size_of::<T>().max(1)).clamp(1, CHUNK_SIZE);
    1usize << slots.ilog2()
}

// Each chunk is filled to its final length before publication and never resized.
// Moving its Vec descriptor does not borrow the published slots mutably.
// Only the engine thread accesses initialized slots.
pub(crate) struct StableStore<T> {
    chunks: UnsafeCell<Vec<Chunk<T>>>,
    len: Cell<usize>,
}

impl<T> Default for StableStore<T> {
    fn default() -> Self {
        Self {
            chunks: UnsafeCell::default(),
            len: Cell::new(0),
        }
    }
}

impl<T> StableStore<T> {
    pub(crate) unsafe fn insert(&self, value: T) -> usize {
        unsafe {
            let index = self.len.get();
            let chunk_size = chunk_size::<T>();
            let chunks = &mut *self.chunks.get();
            if index.is_multiple_of(chunk_size) {
                chunks.push((0..chunk_size).map(|_| UnsafeCell::new(MaybeUninit::uninit())).collect());
            }
            let pointer = chunks[index / chunk_size][index % chunk_size].get().cast::<T>();
            pointer.write(value);
            self.len.set(index + 1);
            index
        }
    }

    #[inline]
    pub(crate) unsafe fn pointer(&self, index: usize) -> *mut T {
        assert!(index < self.len.get());
        let chunk_size = chunk_size::<T>();
        unsafe { (&*self.chunks.get())[index / chunk_size][index % chunk_size].get().cast() }
    }

    pub(crate) fn len(&self) -> usize {
        self.len.get()
    }
}

impl<T> Drop for StableStore<T> {
    fn drop(&mut self) {
        // SAFETY: Published slots were initialized once and are still owned here.
        unsafe {
            let mut remaining = self.len.get();
            let chunk_size = chunk_size::<T>();
            for chunk in self.chunks.get_mut() {
                for slot in chunk.iter().take(remaining.min(chunk_size)) {
                    slot.get().cast::<T>().drop_in_place();
                }
                remaining = remaining.saturating_sub(chunk_size);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    #[test]
    fn large_slots_use_small_chunks_and_preserve_published_addresses() {
        let store = StableStore::<[u8; 4096]>::default();
        unsafe {
            store.insert([7; 4096]);
            let first = store.pointer(0);
            assert_eq!((&*store.chunks.get())[0].len(), 4);
            for byte in 1..10 {
                store.insert([byte; 4096]);
            }
            assert_eq!(store.pointer(0), first);
            assert_eq!(*first, [7; 4096]);
            assert_eq!(*store.pointer(9), [9; 4096]);
            assert_eq!((&*store.chunks.get()).len(), 3);
        }
    }

    struct Tracked {
        drops: Rc<Cell<usize>>,
        value: usize,
    }

    impl Drop for Tracked {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
        }
    }

    #[test]
    fn published_addresses_survive_chunk_growth_and_slots_drop_once() {
        let drops = Rc::new(Cell::new(0));
        let store = StableStore::default();
        unsafe {
            store.insert(Tracked {
                drops: Rc::clone(&drops),
                value: 7,
            });
            let first = store.pointer(0);
            for value in 1..10000 {
                store.insert(Tracked {
                    drops: Rc::clone(&drops),
                    value,
                });
            }
            assert_eq!(store.pointer(0), first);
            assert_eq!((*first).value, 7);
            (*first).value = 42;
            assert_eq!((*store.pointer(0)).value, 42);
            assert_eq!((*store.pointer(9999)).value, 9999);
        }
        drop(store);
        assert_eq!(drops.get(), 10000);
    }

    #[test]
    fn partially_initialized_chunks_drop_during_unwinding() {
        let drops = Rc::new(Cell::new(0));
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let store = StableStore::default();
            for value in 0..CHUNK_SIZE + 1 {
                unsafe {
                    store.insert(Tracked {
                        drops: Rc::clone(&drops),
                        value,
                    });
                }
            }
            panic!("injected arena failure");
        }));
        assert!(result.is_err());
        assert_eq!(drops.get(), CHUNK_SIZE + 1);
    }
}
