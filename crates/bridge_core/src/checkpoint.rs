//! I/O transactions used to reconstruct a suspended engine's bridge state.

use crate::{CoreBridgeState, FileFormat, InputId, OutputId};
use serde::{Deserialize, Serialize};
use std::{io::SeekFrom, path::PathBuf};
use tectonic_errors::{Result, anyhow::ensure};
use tectonic_io_base::digest::{Digest, DigestComputer};

const LIMIT: usize = 64 * 1024 * 1024;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub(crate) enum Operation {
    Input {
        name: Option<String>,
        format: FileFormat,
        gz: bool,
        id: Option<usize>,
        path: Option<PathBuf>,
        resolved: Option<String>,
    },
    Output {
        name: Option<String>,
        gz: bool,
        id: Option<usize>,
    },
    Read {
        id: usize,
        requested: usize,
        exact: bool,
        count: usize,
        digest: [u8; 32],
    },
    Getc {
        id: usize,
        byte: Option<u8>,
    },
    Bytes {
        id: usize,
        bytes: Vec<u8>,
    },
    Ungetc {
        id: usize,
        byte: u8,
    },
    Seek {
        id: usize,
        kind: u8,
        offset: u64,
        result: u64,
    },
    Size {
        id: usize,
        size: usize,
    },
    Mtime {
        id: usize,
        time: i64,
    },
    Md5 {
        name: String,
        digest: Option<[u8; 16]>,
    },
    CloseInput {
        id: usize,
        digest: Option<String>,
    },
    Write {
        id: usize,
        bytes: Vec<u8>,
    },
    CharacterWrites(Vec<(u16, u8)>),
    Flush(usize),
    CloseOutput(usize),
}

pub(crate) struct Journal {
    operations: Vec<Operation>,
    bytes: usize,
    valid: bool,
}

impl Journal {
    pub(crate) fn new() -> Self {
        Self {
            operations: Vec::new(),
            bytes: 0,
            valid: true,
        }
    }

    pub(crate) fn invalidate(&mut self) {
        self.valid = false;
        self.operations = Vec::new();
    }

    pub(crate) fn push(&mut self, operation: Operation) {
        if !self.valid {
            return;
        }
        let operation = match operation {
            Operation::Getc { id, byte: Some(byte) } => {
                if let Some(Operation::Bytes { id: previous, bytes }) = self.operations.last_mut()
                    && *previous == id
                    && bytes.len() < 8192
                {
                    bytes.push(byte);
                    self.bytes += 1;
                    if self.bytes > LIMIT {
                        self.invalidate();
                    }
                    return;
                }
                Operation::Bytes { id, bytes: vec![byte] }
            }
            Operation::Write { id, bytes } => {
                if bytes.len() == 1
                    && let Ok(id) = u16::try_from(id)
                {
                    if let Some(Operation::CharacterWrites(previous)) = self.operations.last_mut()
                        && previous.len() < 8192
                    {
                        previous.push((id, bytes[0]));
                        self.bytes += std::mem::size_of::<(u16, u8)>();
                        if self.bytes > LIMIT {
                            self.invalidate();
                        }
                        return;
                    }
                    Operation::CharacterWrites(vec![(id, bytes[0])])
                } else {
                    if let Some(Operation::Write {
                        id: previous,
                        bytes: previous_bytes,
                    }) = self.operations.last_mut()
                        && *previous == id
                        && previous_bytes.len() + bytes.len() <= 8192
                    {
                        self.bytes += bytes.len();
                        previous_bytes.extend(bytes);
                        if self.bytes > LIMIT {
                            self.invalidate();
                        }
                        return;
                    }
                    Operation::Write { id, bytes }
                }
            }
            operation => operation,
        };
        self.bytes += match &operation {
            Operation::Write { bytes, .. } | Operation::Bytes { bytes, .. } => bytes.len() + 128,
            Operation::Input { name, resolved, .. } => name.as_ref().map_or(0, String::len) + resolved.as_ref().map_or(0, String::len) + 256,
            Operation::Md5 { name, .. } => name.len() + 128,
            _ => 128,
        };
        if self.bytes > LIMIT || self.operations.len() >= 262_144 {
            self.invalidate();
            return;
        }
        self.operations.push(operation);
    }

    pub(crate) fn encode(&self) -> Result<Vec<u8>> {
        ensure!(self.valid, "I/O cannot be checkpointed");
        let bytes = serde_json::to_vec(&self.operations)?;
        ensure!(bytes.len() <= LIMIT, "I/O checkpoint exceeds size limit");
        Ok(bytes)
    }
}

pub(crate) fn hash(bytes: &[u8]) -> [u8; 32] {
    DigestComputer::digest(bytes).into()
}

struct OperationsVisitor<F>(F);

impl<'de, F: FnMut(Operation) -> Result<()>> serde::de::Visitor<'de> for OperationsVisitor<F> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a checkpoint operation sequence")
    }

    fn visit_seq<A: serde::de::SeqAccess<'de>>(mut self, mut sequence: A) -> std::result::Result<(), A::Error> {
        while let Some(operation) = sequence.next_element()? {
            (self.0)(operation).map_err(serde::de::Error::custom)?;
        }
        Ok(())
    }
}

impl CoreBridgeState<'_> {
    pub(crate) fn record(&mut self, operation: Operation) {
        if let Some(journal) = &mut self.journal {
            journal.push(operation);
        }
    }

    pub(crate) fn invalidate_checkpoint(&mut self) {
        if let Some(journal) = &mut self.journal {
            journal.invalidate();
        }
    }

    /// Enable recording before the engine performs any I/O.
    pub fn record_checkpoint_io(&mut self) {
        self.journal = Some(Journal::new());
    }

    /// Whether the recorded I/O can still accompany a native checkpoint.
    pub fn checkpoint_io_ready(&self) -> bool {
        self.journal.as_ref().is_some_and(|journal| journal.valid)
    }

    /// Check a one-based native handle before restoring references to it.
    pub fn checkpoint_handle_valid(&self, handle: usize, output: bool) -> bool {
        let Some(index) = handle.checked_sub(1) else {
            return false;
        };
        if output {
            self.output_handles.get(index).is_some_and(Option::is_some)
        } else {
            self.input_handles.get(index).is_some_and(Option::is_some)
        }
    }

    /// Capture native state and matching I/O as one checkpoint transaction.
    pub fn capture_engine_checkpoint(&mut self, native: &[u8]) {
        if let Some(journal) = &self.journal
            && let Ok(io) = journal.encode()
        {
            self.hooks.event_engine_checkpoint(native, &io);
        }
    }

    /// Replay and validate the engine's prefix without executing TeX tokens.
    /// The caller must discard this bridge and reset its driver on failure.
    pub fn replay_checkpoint_io(&mut self, bytes: &[u8]) -> Result<()> {
        ensure!(bytes.len() <= LIMIT, "I/O checkpoint exceeds size limit");
        ensure!(
            self.input_handles.is_empty() && self.output_handles.is_empty(),
            "checkpoint replay requires a fresh bridge"
        );
        self.record_checkpoint_io();
        let mut read_budget = 512 * 1024 * 1024usize;
        let mut count = 0;
        let replay = |operation: Operation| -> Result<()> {
            count += 1;
            ensure!(count <= 262_144, "too many checkpoint operations");
            match &operation {
                Operation::Input {
                    name,
                    format,
                    gz,
                    id,
                    path,
                    resolved,
                } => {
                    let actual = match name {
                        Some(name) => self.input_open(name, *format, *gz),
                        None => self.input_open_primary(),
                    };
                    ensure!(actual.map(InputId::idx) == *id, "input availability changed");
                    ensure!(self.latest_input_path == *path, "input resolution changed");
                    ensure!(
                        actual.map(|id| self.input_handles[id.idx()].as_ref().unwrap().name().to_owned()) == *resolved,
                        "input name changed"
                    );
                }
                Operation::Output { name, gz, id } => {
                    let actual = match name {
                        Some(name) => self.output_open(name, *gz),
                        None => self.output_open_stdout(),
                    };
                    ensure!(actual.map(OutputId::idx) == *id, "output availability changed");
                }
                Operation::Read {
                    id,
                    requested,
                    exact,
                    count,
                    digest,
                } => {
                    ensure!(
                        *requested <= LIMIT && *count <= *requested && *requested <= read_budget,
                        "invalid checkpoint read"
                    );
                    read_budget -= *requested;
                    let id = self.checked_input(*id)?;
                    let mut buffer = vec![0; *requested];
                    let actual = if *exact {
                        self.input_read(id, &mut buffer)?;
                        *requested
                    } else {
                        self.input_read_partial(id, &mut buffer)?
                    };
                    ensure!(
                        actual == *count && hash(&buffer[..actual]) == *digest,
                        "input bytes changed before checkpoint"
                    );
                }
                Operation::Getc { id, byte } => {
                    let id = self.checked_input(*id)?;
                    ensure!(self.input_getc(id).ok() == *byte, "input byte changed before checkpoint");
                }
                Operation::Bytes { id, bytes } => {
                    ensure!(bytes.len() <= 8192 && bytes.len() <= read_budget, "invalid checkpoint bytes");
                    read_budget -= bytes.len();
                    let id = self.checked_input(*id)?;
                    for byte in bytes {
                        ensure!(self.input_getc(id).ok() == Some(*byte), "input byte changed before checkpoint");
                    }
                }
                Operation::Ungetc { id, byte } => {
                    let id = self.checked_input(*id)?;
                    self.input_ungetc(id, *byte)?;
                }
                Operation::Seek { id, kind, offset, result } => {
                    let id = self.checked_input(*id)?;
                    let from = match kind {
                        0 => SeekFrom::Start(*offset),
                        1 => SeekFrom::End(*offset as i64),
                        2 => SeekFrom::Current(*offset as i64),
                        _ => return Err(tectonic_errors::Error::msg("invalid checkpoint seek")),
                    };
                    ensure!(self.input_seek(id, from)? == *result, "input seek changed");
                }
                Operation::Size { id, size } => {
                    let id = self.checked_input(*id)?;
                    ensure!(self.input_get_size(id) == *size, "input size changed");
                }
                Operation::Mtime { id, time } => {
                    let id = self.checked_input(*id)?;
                    ensure!(self.input_get_mtime(id) == *time, "input modification time changed");
                }
                Operation::Md5 { name, digest } => {
                    let mut actual = [0; 16];
                    let failed = self.get_file_md5(name, &mut actual);
                    ensure!(
                        failed == digest.is_none() && digest.is_none_or(|digest| actual == digest),
                        "file MD5 changed"
                    );
                }
                Operation::CloseInput { id, .. } => {
                    let id = self.checked_input(*id)?;
                    ensure!(!self.input_close(id), "input close failed");
                }
                Operation::Write { id, bytes } => {
                    let id = self.checked_output(*id)?;
                    ensure!(!self.output_write(id, bytes), "output replay failed");
                }
                Operation::CharacterWrites(bytes) => {
                    ensure!(bytes.len() <= 8192, "invalid checkpoint writes");
                    for (id, byte) in bytes {
                        let id = self.checked_output(usize::from(*id))?;
                        ensure!(!self.output_write(id, &[*byte]), "output replay failed");
                    }
                }
                Operation::Flush(id) => {
                    let id = self.checked_output(*id)?;
                    ensure!(!self.output_flush(id), "output flush failed");
                }
                Operation::CloseOutput(id) => {
                    let id = self.checked_output(*id)?;
                    ensure!(!self.output_close(id), "output close failed");
                }
            }
            ensure!(
                self.journal.as_ref().and_then(|journal| journal.operations.last()) == Some(&operation),
                "checkpoint I/O observations changed"
            );
            Ok(())
        };
        use serde::Deserializer;
        let mut decoder = serde_json::Deserializer::from_slice(bytes);
        decoder.deserialize_seq(OperationsVisitor(replay))?;
        decoder.end()?;
        Ok(())
    }

    fn checked_input(&self, index: usize) -> Result<InputId> {
        ensure!(
            self.input_handles.get(index).is_some_and(Option::is_some),
            "invalid checkpoint input handle"
        );
        Ok(InputId::new(index + 1).unwrap())
    }

    fn checked_output(&self, index: usize) -> Result<OutputId> {
        ensure!(
            self.output_handles.get(index).is_some_and(Option::is_some),
            "invalid checkpoint output handle"
        );
        Ok(OutputId::new(index + 1).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CoreBridgeLauncher, MinimalDriver};
    use std::{
        cell::RefCell,
        collections::HashMap,
        io::{self, Cursor},
        rc::Rc,
    };
    use tectonic_io_base::{InputHandle, InputOrigin, IoProvider, OpenResult, OutputHandle};
    use tectonic_status_base::NoopStatusBackend;

    #[test]
    fn character_io_does_not_exhaust_the_checkpoint_journal() {
        let mut journal = Journal::new();
        for i in 0..300_000 {
            journal.push(Operation::Write {
                id: i % 2,
                bytes: vec![b'x'],
            });
        }
        for _ in 0..300_000 {
            journal.push(Operation::Getc { id: 0, byte: Some(b'y') });
        }
        let encoded = journal.encode().unwrap();
        assert!(encoded.len() < 5_000_000);
        let write_storage: usize = journal
            .operations
            .iter()
            .map(|operation| match operation {
                Operation::CharacterWrites(bytes) => bytes.capacity() * std::mem::size_of::<(u16, u8)>(),
                _ => 0,
            })
            .sum();
        assert!(write_storage < 2_000_000);
        let operations: Vec<Operation> = serde_json::from_slice(&encoded).unwrap();
        let count = operations
            .iter()
            .map(|operation| match operation {
                Operation::Write { bytes, .. } | Operation::Bytes { bytes, .. } => bytes.len(),
                Operation::CharacterWrites(bytes) => bytes.len(),
                _ => 0,
            })
            .sum::<usize>();
        assert_eq!(count, 600_000);
    }

    #[test]
    fn character_writes_preserve_handles_outside_the_compact_range() {
        let mut journal = Journal::new();
        for id in [0, u16::MAX as usize, u16::MAX as usize + 1, usize::MAX] {
            journal.push(Operation::Write { id, bytes: vec![b'x'] });
        }
        let encoded = journal.encode().unwrap();
        let operations: Vec<Operation> = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(
            operations,
            vec![
                Operation::CharacterWrites(vec![(0, b'x'), (u16::MAX, b'x')]),
                Operation::Write {
                    id: u16::MAX as usize + 1,
                    bytes: vec![b'x']
                },
                Operation::Write {
                    id: usize::MAX,
                    bytes: vec![b'x']
                },
            ]
        );
        journal.invalidate();
        assert_eq!(journal.operations.capacity(), 0);
    }

    #[derive(Clone, Default)]
    struct Writer(Rc<RefCell<Vec<u8>>>);

    impl io::Write for Writer {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0.borrow_mut().extend(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct Provider {
        files: HashMap<String, Vec<u8>>,
        output: Writer,
    }

    impl IoProvider for Provider {
        fn input_open_name(&mut self, name: &str, _: &mut dyn crate::StatusBackend) -> OpenResult<InputHandle> {
            match self.files.get(name) {
                Some(bytes) => OpenResult::Ok(InputHandle::new(name, Cursor::new(bytes.clone()), InputOrigin::Filesystem)),
                None => OpenResult::NotAvailable,
            }
        }
        fn output_open_name(&mut self, name: &str) -> OpenResult<OutputHandle> {
            self.output.0.borrow_mut().clear();
            OpenResult::Ok(OutputHandle::new(name, self.output.clone()))
        }
    }

    fn driver(bytes: &[u8]) -> MinimalDriver<Provider> {
        MinimalDriver::new(Provider {
            files: HashMap::from([("source.tex".to_owned(), bytes.to_vec())]),
            output: Writer::default(),
        })
    }

    fn record_prefix(close: bool) -> Vec<u8> {
        let mut driver = driver(b"prefix-old");
        CoreBridgeLauncher::new(&mut driver, &mut NoopStatusBackend::default())
            .with_global_lock(|state| {
                state.record_checkpoint_io();
                let input = state.input_open("source.tex", FileFormat::Tex, false).unwrap();
                assert_eq!(state.input_getc(input).unwrap(), b'p');
                state.input_ungetc(input, b'p')?;
                assert_eq!(state.input_seek(input, SeekFrom::Current(0))?, 0);
                state.input_read(input, &mut [0; 7])?;
                if close {
                    state.input_close(input);
                }
                state.journal.as_ref().unwrap().encode()
            })
            .unwrap()
    }

    #[test]
    fn replay_validates_prefix_and_restores_pushback_and_position() {
        let journal = record_prefix(false);
        let mut driver = driver(b"prefix-new suffix");
        CoreBridgeLauncher::new(&mut driver, &mut NoopStatusBackend::default())
            .with_global_lock(|state| {
                state.replay_checkpoint_io(&journal)?;
                let input = state.checked_input(0)?;
                let mut bytes = Vec::new();
                state.input_read_to_end(input, &mut bytes)?;
                assert_eq!(bytes, b"new suffix");
                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn replay_detects_changed_closed_dependencies_and_negative_lookups() {
        let journal = record_prefix(true);
        let mut changed = driver(b"prefix-new");
        let result =
            CoreBridgeLauncher::new(&mut changed, &mut NoopStatusBackend::default()).with_global_lock(|state| state.replay_checkpoint_io(&journal));
        assert!(result.is_err());
        let mut absent = driver(b"prefix-old");
        let journal = CoreBridgeLauncher::new(&mut absent, &mut NoopStatusBackend::default())
            .with_global_lock(|state| {
                state.record_checkpoint_io();
                assert!(state.input_open("missing", FileFormat::Tex, false).is_none());
                state.journal.as_ref().unwrap().encode()
            })
            .unwrap();
        let mut present = driver(b"prefix-old");
        present.0.files.insert("missing".to_owned(), Vec::new());
        let result =
            CoreBridgeLauncher::new(&mut present, &mut NoopStatusBackend::default()).with_global_lock(|state| state.replay_checkpoint_io(&journal));
        assert!(result.is_err());
    }

    #[test]
    fn replay_validates_file_md5_queries_including_missing_files() {
        for present in [false, true] {
            let mut original = driver(b"original");
            if !present {
                original.0.files.clear();
            }
            let journal = CoreBridgeLauncher::new(&mut original, &mut NoopStatusBackend::default())
                .with_global_lock(|state| {
                    state.record_checkpoint_io();
                    assert_eq!(state.get_file_md5("source.tex", &mut [0; 16]), !present);
                    state.journal.as_ref().unwrap().encode()
                })
                .unwrap();
            let mut matching = driver(b"original");
            if !present {
                matching.0.files.clear();
            }
            CoreBridgeLauncher::new(&mut matching, &mut NoopStatusBackend::default())
                .with_global_lock(|state| state.replay_checkpoint_io(&journal))
                .unwrap();
            let mut changed = driver(b"changed");
            assert!(
                CoreBridgeLauncher::new(&mut changed, &mut NoopStatusBackend::default())
                    .with_global_lock(|state| state.replay_checkpoint_io(&journal))
                    .is_err()
            );
        }
    }

    #[test]
    fn replay_reconstructs_gzip_output_and_flush_boundaries() {
        let mut original = driver(b"");
        let journal = CoreBridgeLauncher::new(&mut original, &mut NoopStatusBackend::default())
            .with_global_lock(|state| {
                state.record_checkpoint_io();
                let output = state.output_open("document.synctex.gz", true).unwrap();
                assert!(!state.output_write(output, b"prefix"));
                assert!(!state.output_flush(output));
                state.journal.as_ref().unwrap().encode()
            })
            .unwrap();
        let mut resumed = driver(b"");
        CoreBridgeLauncher::new(&mut resumed, &mut NoopStatusBackend::default())
            .with_global_lock(|state| {
                state.replay_checkpoint_io(&journal)?;
                let output = state.checked_output(0)?;
                assert!(!state.output_write(output, b"suffix"));
                assert!(!state.output_close(output));
                Ok(())
            })
            .unwrap();
        let mut bytes = Vec::new();
        io::Read::read_to_end(&mut flate2::read::GzDecoder::new(resumed.0.output.0.borrow().as_slice()), &mut bytes).unwrap();
        assert_eq!(bytes, b"prefixsuffix");
    }

    #[test]
    fn malformed_journals_reject_invalid_handles_before_access() {
        let bytes = serde_json::to_vec(&vec![Operation::CloseInput {
            id: usize::MAX,
            digest: None,
        }])
        .unwrap();
        let mut driver = driver(b"");
        let result =
            CoreBridgeLauncher::new(&mut driver, &mut NoopStatusBackend::default()).with_global_lock(|state| state.replay_checkpoint_io(&bytes));
        assert!(result.is_err());
    }
}
