use crate::{
    backend::runtime,
    error::{IoOperation, PdfError},
    input::FontKind,
};
use std::{
    ffi::c_void,
    io::{self, Read, Seek, SeekFrom},
    marker::PhantomData,
};
use tectonic_bridge_core::{CoreBridgeState, ttbc_input_read, ttbc_input_read_partial, ttbc_input_seek};

pub(crate) struct ProbeInput {
    pub(crate) handle: usize,
}

pub(crate) struct DirectoryInput {
    pub(crate) handle: usize,
}

impl Read for DirectoryInput {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        let length = CoreBridgeState::with_global_state(|state| unsafe {
            ttbc_input_read_partial(state, runtime::input_id(self.handle), buffer.as_mut_ptr(), buffer.len())
        });
        usize::try_from(length).map_err(|_| io::Error::other("PDF input read failed"))
    }
}

impl Read for ProbeInput {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        // The bridge intentionally retains exact-read diagnostics for legacy probes.
        let length = CoreBridgeState::with_global_state(|state| unsafe {
            ttbc_input_read(state, runtime::input_id(self.handle), buffer.as_mut_ptr(), buffer.len())
        });
        usize::try_from(length).map_err(|_| io::Error::other("PDF input read failed"))
    }
}

impl Seek for ProbeInput {
    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        let (offset, whence) = match position {
            SeekFrom::Start(offset) => (offset as isize, libc::SEEK_SET),
            SeekFrom::Current(offset) => (offset as isize, libc::SEEK_CUR),
            SeekFrom::End(offset) => (offset as isize, libc::SEEK_END),
        };
        let mut error = 0;
        let position =
            CoreBridgeState::with_global_state(|state| unsafe { ttbc_input_seek(state, runtime::input_id(self.handle), offset, whence, &mut error) });
        if error != 0 {
            return Err(io::Error::other("Invalid PDF input seek"));
        }
        Ok(position as u64)
    }
}

pub(crate) fn probe_font(handle: usize, kind: FontKind) -> Result<bool, PdfError> {
    match crate::input::probe_font(&mut ProbeInput { handle }, kind) {
        // Historical probes reject unreadable headers after the bridge emits its warning.
        Err(error) if error.operation == Some(IoOperation::Read) => Ok(false),
        result => result,
    }
}

pub(crate) struct BorrowedFile<'a> {
    file: *mut libc::FILE,
    _borrow: PhantomData<&'a mut libc::FILE>,
}

impl BorrowedFile<'_> {
    /// The caller exclusively borrows a live FILE for the lifetime of this adapter.
    pub(crate) unsafe fn from_raw(file: *mut c_void) -> Self {
        Self {
            file: file.cast(),
            _borrow: PhantomData,
        }
    }
}

impl Seek for BorrowedFile<'_> {
    fn rewind(&mut self) -> io::Result<()> {
        // Preserve stdio rewind's error-indicator reset and historical void result.
        unsafe {
            libc::rewind(self.file);
        }
        Ok(())
    }

    fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
        let (offset, whence) = match position {
            SeekFrom::Start(offset) => (i64::try_from(offset).map_err(io::Error::other)?, libc::SEEK_SET),
            SeekFrom::Current(offset) => (offset, libc::SEEK_CUR),
            SeekFrom::End(offset) => (offset, libc::SEEK_END),
        };
        let offset = libc::c_long::try_from(offset).map_err(io::Error::other)?;
        // SAFETY: The constructor requires an exclusive live FILE borrow.
        unsafe {
            if libc::fseek(self.file, offset, whence) != 0 {
                return Err(io::Error::last_os_error());
            }
            let position = libc::ftell(self.file);
            if position < 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(position as u64)
        }
    }
}

impl Read for BorrowedFile<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        // SAFETY: The constructor requires an exclusive live FILE borrow.
        unsafe {
            let read = libc::fread(buffer.as_mut_ptr().cast(), 1, buffer.len(), self.file);
            if read == 0 && libc::ferror(self.file) != 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(read)
        }
    }
}
