use crate::{
    buffer::{BufTy, GlobalBuffer},
    char_info::LexClass,
    ASCIICode, Bibtex, BibtexError, BufPointer,
};
use libc::EOF;
use std::{
    ffi::CStr,
    io::{self, Read},
};
use tectonic_bridge_core::{CoreBridgeState, FileFormat, InputId};

/* Sigh, I'm worried about ungetc() and EOF semantics in Bibtex's I/O, so
 * here's a tiny wrapper that lets us fake it. */

pub(crate) struct PeekableInput {
    id: InputId,
    peek_char: libc::c_int,
    saw_eof: bool,
    buffer: Vec<u8>,
    offset: usize,
    filled: usize,
}

impl PeekableInput {
    pub(crate) fn open(
        ctx: &mut Bibtex<'_, '_>,
        path: &CStr,
        format: FileFormat,
    ) -> Result<PeekableInput, BibtexError> {
        let id = ctx.engine.input_open(path.to_str().unwrap(), format, false);

        if let Some(id) = id {
            Ok(Self::new(id))
        } else {
            Err(BibtexError::Fatal)
        }
    }

    fn new(id: InputId) -> Self {
        Self {
            id,
            peek_char: EOF,
            saw_eof: false,
            buffer: vec![0; 8192],
            offset: 0,
            filled: 0,
        }
    }

    pub(crate) fn close(self, ctx: &mut Bibtex<'_, '_>) -> Result<(), BibtexError> {
        if !ctx.engine.input_close(self.id) {
            Ok(())
        } else {
            Err(BibtexError::Fatal)
        }
    }

    fn getc(&mut self, engine: &mut CoreBridgeState<'_>) -> libc::c_int {
        if self.peek_char != EOF {
            let rv = self.peek_char;
            self.peek_char = EOF;
            return rv;
        }

        if self.saw_eof {
            return EOF;
        }

        if self.offset == self.filled {
            loop {
                match engine.get_input(self.id).read(&mut self.buffer) {
                    Ok(0) => {
                        self.saw_eof = true;
                        return EOF;
                    }
                    Ok(filled) => {
                        self.filled = filled;
                        self.offset = 0;
                        break;
                    }
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(_) => {
                        self.saw_eof = true;
                        return EOF;
                    }
                }
            }
        }

        let c = self.buffer[self.offset];
        self.offset += 1;
        libc::c_int::from(c)
    }

    fn ungetc(&mut self, c: libc::c_int) {
        assert_ne!(c, EOF);
        self.peek_char = c;
    }

    pub fn eof(&mut self, engine: &mut CoreBridgeState<'_>) -> bool {
        if self.saw_eof {
            return true;
        }
        let c = self.getc(engine);
        if c == EOF {
            return true;
        }
        self.ungetc(c);
        false
    }

    fn eoln(&mut self, engine: &mut CoreBridgeState<'_>) -> bool {
        if self.saw_eof {
            return true;
        }
        let c = self.getc(engine);
        if c != EOF {
            self.ungetc(c);
        }
        c == b'\n' as libc::c_int || c == '\r' as libc::c_int || c == EOF
    }
}

pub(crate) fn input_ln(
    engine: &mut CoreBridgeState<'_>,
    peekable: &mut PeekableInput,
    buffers: &mut GlobalBuffer,
) -> bool {
    buffers.set_init(BufTy::Base, 0);
    let mut last = 0;
    if peekable.eof(engine) {
        return false;
    }

    // Read up to end-of-line
    while !peekable.eoln(engine) {
        if last >= buffers.len() as BufPointer {
            buffers.grow_all();
        }

        buffers.set_at(BufTy::Base, last, peekable.getc(engine) as ASCIICode);
        last += 1;
    }

    // For side effects - consume the eoln we saw
    let eoln = peekable.getc(engine);
    if eoln == '\r' as libc::c_int {
        let next = peekable.getc(engine);
        if next != '\n' as libc::c_int && next != EOF {
            peekable.ungetc(next);
        }
    }

    // Trim whitespace
    while last > 0 {
        if LexClass::of(buffers.at(BufTy::Base, last - 1)) == LexClass::Whitespace {
            last -= 1;
        } else {
            break;
        }
    }

    buffers.set_init(BufTy::Base, last);

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::Cell,
        io::{Cursor, Seek, SeekFrom},
        rc::Rc,
    };
    use tectonic_bridge_core::{CoreBridgeLauncher, MinimalDriver};
    use tectonic_errors::Result;
    use tectonic_io_base::{InputFeatures, InputHandle, InputOrigin, IoProvider, OpenResult};
    use tectonic_status_base::{NoopStatusBackend, StatusBackend};

    struct CountingInput {
        data: Cursor<Vec<u8>>,
        reads: Rc<Cell<usize>>,
        max_read: usize,
    }

    impl Read for CountingInput {
        fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
            let reads = self.reads.get();
            self.reads.set(reads + 1);
            if reads == 0 {
                return Err(io::ErrorKind::Interrupted.into());
            }
            let len = bytes.len().min(self.max_read);
            self.data.read(&mut bytes[..len])
        }
    }

    impl InputFeatures for CountingInput {
        fn get_size(&mut self) -> Result<usize> {
            Ok(self.data.get_ref().len())
        }

        fn try_seek(&mut self, pos: SeekFrom) -> Result<u64> {
            Ok(self.data.seek(pos)?)
        }
    }

    struct TestIo {
        contents: Vec<u8>,
        reads: Rc<Cell<usize>>,
        max_read: usize,
    }

    impl IoProvider for TestIo {
        fn input_open_name(
            &mut self,
            name: &str,
            _: &mut dyn StatusBackend,
        ) -> OpenResult<InputHandle> {
            OpenResult::Ok(InputHandle::new_read_only(
                name,
                CountingInput {
                    data: Cursor::new(self.contents.clone()),
                    reads: self.reads.clone(),
                    max_read: self.max_read,
                },
                InputOrigin::Other,
            ))
        }
    }

    fn with_input(
        contents: &[u8],
        max_read: usize,
        f: impl FnOnce(&mut CoreBridgeState<'_>, &mut PeekableInput),
    ) -> usize {
        let reads = Rc::new(Cell::new(0));
        let mut hooks = MinimalDriver::new(TestIo {
            contents: contents.to_vec(),
            reads: reads.clone(),
            max_read,
        });
        let mut status = NoopStatusBackend {};
        let mut launcher = CoreBridgeLauncher::new(&mut hooks, &mut status);
        launcher
            .with_global_lock(|engine| {
                let id = engine
                    .input_open("input.bib", FileFormat::Bib, false)
                    .unwrap();
                let mut input = PeekableInput::new(id);
                f(engine, &mut input);
                assert!(!engine.input_close(id));
                Ok(())
            })
            .unwrap();
        reads.get()
    }

    #[test]
    fn character_reads_are_batched_and_preserve_lookahead() {
        let contents: Vec<u8> = (0..25000).map(|i| (i % 256) as u8).collect();
        let reads = with_input(&contents, usize::MAX, |engine, input| {
            for &expected in &contents {
                assert!(!input.eof(engine));
                assert_eq!(input.getc(engine), i32::from(expected));
            }
            for _ in 0..3 {
                assert!(input.eof(engine));
                assert_eq!(input.getc(engine), EOF);
            }
        });
        assert_eq!(reads, 6);
    }

    #[test]
    fn lines_handle_short_reads_crlf_and_terminal_cr() {
        with_input(
            b"alpha  \r\n\r\nbeta\ngamma\rdelta\r",
            1,
            |engine, input| {
                let mut buffers = GlobalBuffer::new();
                for expected in [b"alpha".as_slice(), b"", b"beta", b"gamma", b"delta"] {
                    assert!(input_ln(engine, input, &mut buffers));
                    assert_eq!(
                        &buffers.buffer(BufTy::Base)[..buffers.init(BufTy::Base)],
                        expected
                    );
                }
                assert!(!input_ln(engine, input, &mut buffers));
            },
        );
    }
}
