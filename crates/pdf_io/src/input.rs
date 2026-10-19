use crate::{
    diagnostics::format_message,
    error::{IoOperation, PdfError, PdfErrorKind},
};
use std::io::{Read, Seek, SeekFrom};

pub(crate) fn seek_relative(input: &mut impl Seek, offset: i32) -> Result<(), PdfError> {
    input
        .seek(SeekFrom::Current(i64::from(offset)))
        .map(|_| ())
        .map_err(|error| {
            PdfError::io(
                c"io:  An OS command failed that should not have.\n".to_owned(),
                error,
            )
            .during(IoOperation::Seek)
        })
}

pub(crate) fn file_size(input: &mut impl Seek) -> Result<i32, PdfError> {
    let size = input.seek(SeekFrom::End(0)).map_err(|error| {
        PdfError::io(
            c"io:  An OS command failed that should not have.\n".to_owned(),
            error,
        )
        .during(IoOperation::Seek)
    })?;
    let size = i32::try_from(size).map_err(|_| {
        PdfError::new(
            PdfErrorKind::Unsupported,
            format_message(format_args!(
                "ftell: file size {size} exceeds 0x7fffffff.\n"
            )),
        )
        .at(size)
    })?;
    input.rewind().map_err(|error| {
        PdfError::io(
            c"io:  An OS command failed that should not have.\n".to_owned(),
            error,
        )
        .during(IoOperation::Seek)
    })?;
    Ok(size)
}

#[derive(Clone, Copy)]
pub(crate) enum FontKind {
    TrueType,
    OpenType,
    Type1,
}

pub(crate) fn read_array<const N: usize>(
    input: &mut impl Read,
    offset: u64,
) -> Result<[u8; N], PdfError> {
    let mut bytes = [0; N];
    let mut consumed = 0;
    while consumed < N {
        match input.read(&mut bytes[consumed..]) {
            Ok(0) => {
                return Err(PdfError::new(
                    PdfErrorKind::MalformedInput,
                    c"File ended prematurely\n".to_owned(),
                )
                .at(offset + consumed as u64)
                .during(IoOperation::Read));
            }
            Ok(length) => consumed += length,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
            Err(error) => {
                return Err(PdfError::io(c"File ended prematurely\n".to_owned(), error)
                    .at(offset + consumed as u64)
                    .during(IoOperation::Read));
            }
        }
    }
    Ok(bytes)
}

pub(crate) fn probe_font(input: &mut (impl Read + Seek), kind: FontKind) -> Result<bool, PdfError> {
    input.rewind().map_err(|error| {
        PdfError::io(c"Invalid PDF input seek".to_owned(), error)
            .at(0)
            .during(IoOperation::Seek)
    })?;
    let mut bytes = [0; 21];
    let length = match kind {
        FontKind::Type1 => 21,
        _ => 4,
    };
    let result = input.read_exact(&mut bytes[..length]);
    // Header probes always restore the input, including after a short read.
    input.rewind().map_err(|error| {
        PdfError::io(c"Invalid PDF input seek".to_owned(), error)
            .at(0)
            .during(IoOperation::Seek)
    })?;
    match result {
        Ok(()) => Ok(match kind {
            FontKind::TrueType => matches!(&bytes[..4], b"true" | b"\0\x01\0\0" | b"ttcf"),
            FontKind::OpenType => &bytes[..4] == b"OTTO",
            FontKind::Type1 => {
                bytes[0] == 0x80
                    && bytes[1] <= 3
                    && (bytes[6..].starts_with(b"%!PS-AdobeFont")
                        || bytes[6..].starts_with(b"%!FontType1")
                        || bytes[6..].starts_with(b"%!PS"))
            }
        }),
        Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => Err(PdfError::new(
            PdfErrorKind::MalformedInput,
            c"File ended prematurely\n".to_owned(),
        )
        .at(0)
        .during(IoOperation::Read)),
        Err(error) => Err(PdfError::io(c"File ended prematurely\n".to_owned(), error)
            .at(0)
            .during(IoOperation::Read)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Cursor};

    #[test]
    fn probes_restore_input_and_preserve_font_signatures() {
        for (kind, bytes) in [
            (FontKind::TrueType, &b"true"[..]),
            (FontKind::TrueType, &b"\0\x01\0\0"[..]),
            (FontKind::TrueType, &b"ttcf"[..]),
            (FontKind::OpenType, &b"OTTO"[..]),
            (FontKind::Type1, &b"\x80\x01\0\0\0\0%!PS-AdobeFont x"[..]),
        ] {
            let mut input = Cursor::new(bytes);
            input.set_position(2);
            assert!(probe_font(&mut input, kind).unwrap());
            assert_eq!(input.position(), 0);
            for length in 0..match kind {
                FontKind::Type1 => 21,
                _ => 4,
            } {
                let mut input = Cursor::new(&bytes[..length]);
                let error = probe_font(&mut input, kind).unwrap_err();
                assert_eq!(error.kind, PdfErrorKind::MalformedInput);
                assert_eq!(input.position(), 0);
            }
        }
        assert!(!probe_font(&mut Cursor::new(b"bad!"), FontKind::TrueType).unwrap());
    }

    #[test]
    fn size_and_relative_seeks_preserve_positions_and_overflow_diagnostic() {
        let mut input = Cursor::new(b"abcdef");
        seek_relative(&mut input, 3).unwrap();
        assert_eq!(input.position(), 3);
        assert_eq!(file_size(&mut input).unwrap(), 6);
        assert_eq!(input.position(), 0);

        struct Large;
        impl Seek for Large {
            fn seek(&mut self, _: SeekFrom) -> io::Result<u64> {
                Ok(0x80000000)
            }
        }
        let error = file_size(&mut Large).unwrap_err();
        assert_eq!(error.kind, PdfErrorKind::Unsupported);
        assert_eq!(error.offset, Some(0x80000000));
        assert_eq!(
            error.message.as_c_str(),
            c"ftell: file size 2147483648 exceeds 0x7fffffff.\n"
        );
    }

    #[test]
    fn read_errors_retain_sources_offsets_and_non_utf8_filenames() {
        struct Failing;
        impl Read for Failing {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::PermissionDenied, "denied"))
            }
        }
        let error = read_array::<4>(&mut Failing, 17)
            .unwrap_err()
            .in_file(b"font\xff.otf");
        assert_eq!(error.kind, PdfErrorKind::Io);
        assert_eq!(error.offset, Some(17));
        assert_eq!(error.file.as_deref(), Some(&b"font\xff.otf"[..]));
        let source = std::error::Error::source(&error)
            .unwrap()
            .downcast_ref::<io::Error>()
            .unwrap();
        assert_eq!(source.kind(), io::ErrorKind::PermissionDenied);
        let error = read_array::<4>(&mut Cursor::new(b"ab"), 10).unwrap_err();
        assert_eq!(error.kind, PdfErrorKind::MalformedInput);
        assert_eq!(error.offset, Some(12));
        assert_eq!(read_array::<2>(&mut Cursor::new(b"ok"), 0).unwrap(), *b"ok");
    }
}
