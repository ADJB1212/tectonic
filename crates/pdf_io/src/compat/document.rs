//! Scoped binding used only by compatibility code and the translated backend.

use crate::{
    backend, diagnostics,
    error::{PdfError, PdfErrorKind},
};
use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    marker::PhantomData,
    rc::Rc,
    sync::atomic::{AtomicPtr, Ordering},
};

thread_local! {
    static CURRENT: Cell<*mut Document> = const { Cell::new(core::ptr::null_mut()) };
}

static CURRENT_ENGINE_DOCUMENT: AtomicPtr<Document> = AtomicPtr::new(core::ptr::null_mut());

#[cfg(test)]
static TEST_DOCUMENT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub(crate) struct Document {
    pub(crate) objects: super::objects::State,
    pub(crate) names: backend::dpx_pdfnames::Storage,
    pub(crate) glyphs: backend::GlyphCaches,
    pub(crate) cff_strings: backend::StringCaches,
    pub(crate) post_names: backend::PostCaches,
    pub(crate) backend: backend::DocumentStates,
    pub(crate) allocations: super::allocation::AllocationSet,
    pub(crate) diagnostics: diagnostics::State,
    font_maps: RefCell<crate::fontmap::FontMaps>,
    page_scanner: RefCell<crate::preflight::PageScanner>,
    tfm: crate::tfm::DocumentState,
}

pub(crate) struct DocumentScope {
    document: Box<Document>,
    _thread: PhantomData<Rc<()>>,
    #[cfg(test)]
    _test_lock: std::sync::MutexGuard<'static, ()>,
}

impl DocumentScope {
    pub(crate) fn new() -> Result<Self, PdfError> {
        if active() {
            return Err(PdfError::new(
                PdfErrorKind::InvalidState,
                c"PDF document scopes cannot be nested".to_owned(),
            ));
        }
        #[cfg(test)]
        let test_lock = TEST_DOCUMENT_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut document = Box::new(Document {
            objects: super::objects::State::default(),
            names: backend::dpx_pdfnames::Storage::default(),
            glyphs: backend::GlyphCaches::default(),
            cff_strings: backend::StringCaches::default(),
            post_names: backend::PostCaches::default(),
            backend: backend::DocumentStates::new(),
            allocations: super::allocation::AllocationSet::default(),
            diagnostics: diagnostics::State::default(),
            font_maps: RefCell::default(),
            page_scanner: RefCell::default(),
            tfm: crate::tfm::DocumentState::default(),
        });
        if CURRENT_ENGINE_DOCUMENT
            .compare_exchange(
                core::ptr::null_mut(),
                &mut *document,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_err()
        {
            return Err(PdfError::new(
                PdfErrorKind::InvalidState,
                c"PDF document scopes cannot overlap".to_owned(),
            ));
        }
        // SAFETY: The engine lock and scoped binding exclude other ABI data users.
        unsafe {
            backend::reset_abi_state();
        }
        CURRENT.with(|current| current.set(&mut *document));
        crate::backend::runtime::clear_error();
        Ok(Self {
            document,
            _thread: PhantomData,
            #[cfg(test)]
            _test_lock: test_lock,
        })
    }
}

impl Drop for DocumentScope {
    fn drop(&mut self) {
        // SAFETY: The engine has returned; no translated operation retains a state borrow.
        unsafe {
            backend::reset_abi_state();
        }
        CURRENT_ENGINE_DOCUMENT.store(core::ptr::null_mut(), Ordering::Release);
        CURRENT.with(|current| {
            assert!(core::ptr::eq(current.get(), &*self.document));
            current.set(core::ptr::null_mut());
        });
        // SAFETY: Engine execution and its compatibility cleanup have returned.
        unsafe {
            for pointer in self.document.allocations.drain() {
                libc::free(pointer as *mut c_void);
            }
        }
    }
}

pub(crate) fn active() -> bool {
    !try_current().is_null()
}

#[inline]
pub(crate) fn try_current() -> *mut Document {
    CURRENT.with(Cell::get)
}

#[inline]
pub(crate) unsafe fn current() -> *mut Document {
    // The engine lock protects this hot path. Thread-local lookup is retained
    // for allocation handoffs and callers outside an engine invocation.
    let document = CURRENT_ENGINE_DOCUMENT.load(Ordering::Relaxed);
    if document.is_null() {
        crate::backend::runtime::raise_error(PdfError::new(
            PdfErrorKind::InvalidState,
            c"No active PDF document".to_owned(),
        ));
    }
    document
}

pub(crate) fn with_diagnostics<T>(callback: impl FnOnce(&mut diagnostics::State) -> T) -> T {
    // SAFETY: This short borrow covers only the diagnostic group on its owning thread.
    unsafe { callback(&mut (*current()).diagnostics) }
}

pub(crate) fn with_font_maps<T>(
    callback: impl FnOnce(&RefCell<crate::fontmap::FontMaps>) -> T,
) -> T {
    unsafe { callback(&(*current()).font_maps) }
}

pub(crate) fn with_page_scanner<T>(
    callback: impl FnOnce(&RefCell<crate::preflight::PageScanner>) -> T,
) -> T {
    unsafe { callback(&(*current()).page_scanner) }
}

pub(crate) fn with_tfm<T>(callback: impl FnOnce(&crate::tfm::DocumentState) -> T) -> T {
    unsafe { callback(&(*current()).tfm) }
}

#[unsafe(no_mangle)]
extern "C" fn tt_pdf_document_begin() -> *mut c_void {
    crate::backend::runtime::result_boundary(|| DocumentScope::new().map(Box::new))
        .map_or(core::ptr::null_mut(), |scope| Box::into_raw(scope).cast())
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_document_end(scope: *mut c_void) {
    if !scope.is_null() {
        // SAFETY: The engine returns the scope created on this thread exactly once.
        unsafe {
            drop(Box::from_raw(scope.cast::<DocumentScope>()));
        }
    }
}
