use std::{ffi::CString, fmt, io};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PdfErrorKind {
    MalformedInput,
    Unsupported,
    Io,
    InvalidState,
    Backend,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum IoOperation {
    Read,
    Seek,
}

#[derive(Debug)]
pub(crate) struct PdfError {
    pub(crate) kind: PdfErrorKind,
    pub(crate) message: CString,
    pub(crate) file: Option<Vec<u8>>,
    pub(crate) offset: Option<u64>,
    pub(crate) operation: Option<IoOperation>,
    source: Option<io::Error>,
}

impl PdfError {
    pub(crate) fn new(kind: PdfErrorKind, message: CString) -> Self {
        Self {
            kind,
            message,
            file: None,
            offset: None,
            operation: None,
            source: None,
        }
    }

    pub(crate) fn io(message: CString, source: io::Error) -> Self {
        Self {
            source: Some(source),
            ..Self::new(PdfErrorKind::Io, message)
        }
    }

    pub(crate) fn at(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub(crate) fn during(mut self, operation: IoOperation) -> Self {
        self.operation = Some(operation);
        self
    }

    pub(crate) fn in_file(mut self, file: &[u8]) -> Self {
        self.file = Some(file.to_vec());
        self
    }
}

impl fmt::Display for PdfError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(output, "{:?}: ", self.kind)?;
        if let Some(operation) = self.operation {
            write!(output, "{operation:?}: ")?;
        }
        if let Some(file) = &self.file {
            write!(output, "{}: ", String::from_utf8_lossy(file))?;
        }
        if let Some(offset) = self.offset {
            write!(output, "offset {offset}: ")?;
        }
        write!(output, "{}", self.message.to_string_lossy())
    }
}

impl std::error::Error for PdfError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|source| source as _)
    }
}
