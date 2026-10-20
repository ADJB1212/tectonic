//! Checked SFNT directory primitives. Table ownership remains with the caller.

use crate::byte_cursor::{ByteCursor, ReadError};
use crate::error::{PdfError, PdfErrorKind};
use std::io::Read;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct DirectoryHeader {
    pub(crate) version: u32,
    pub(crate) num_tables: u16,
    pub(crate) search_range: u16,
    pub(crate) entry_selector: u16,
    pub(crate) range_shift: u16,
}

impl DirectoryHeader {
    pub(crate) fn read(input: &mut impl Read, offset: u64) -> Result<Self, PdfError> {
        let bytes = crate::input::read_array::<12>(input, offset)?;
        Self::parse(&bytes).map_err(|_| PdfError::new(PdfErrorKind::MalformedInput, c"File ended prematurely\n".to_owned()).at(offset))
    }
    pub(crate) fn parse(bytes: &[u8]) -> Result<Self, ReadError> {
        let mut cursor = ByteCursor::new(bytes);
        Ok(Self {
            version: cursor.read_u32()?,
            num_tables: cursor.read_u16()?,
            search_range: cursor.read_u16()?,
            entry_selector: cursor.read_u16()?,
            range_shift: cursor.read_u16()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TableRecord {
    pub(crate) tag: [u8; 4],
    pub(crate) checksum: u32,
    pub(crate) offset: u32,
    pub(crate) length: u32,
}

impl TableRecord {
    pub(crate) fn read(input: &mut impl Read, offset: u64) -> Result<Self, PdfError> {
        let bytes = crate::input::read_array::<16>(input, offset)?;
        Self::parse(&bytes).map_err(|_| PdfError::new(PdfErrorKind::MalformedInput, c"File ended prematurely\n".to_owned()).at(offset))
    }
    pub(crate) fn parse(bytes: &[u8]) -> Result<Self, ReadError> {
        let mut cursor = ByteCursor::new(bytes);
        Ok(Self {
            tag: cursor.read_u32()?.to_be_bytes(),
            checksum: cursor.read_u32()?,
            offset: cursor.read_u32()?,
            length: cursor.read_u32()?,
        })
    }
}

pub(crate) fn checksum(bytes: &[u8]) -> u32 {
    let mut cursor = ByteCursor::new(bytes);
    let mut checksum = 0u32;
    while cursor.remaining() >= 4 {
        checksum = checksum.wrapping_add(cursor.read_u32().unwrap());
    }
    let mut tail = [0; 4];
    let length = cursor.remaining();
    tail[..length].copy_from_slice(cursor.read(length).unwrap());
    checksum.wrapping_add(u32::from_be_bytes(tail))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_directory_reads_report_truncated_offsets_and_allow_retry() {
        for length in 0..12 {
            let error = DirectoryHeader::read(&mut &vec![0; length][..], 100).unwrap_err();
            assert_eq!(error.kind, PdfErrorKind::MalformedInput);
            assert_eq!(error.offset, Some(100 + length as u64));
        }
        for length in 0..16 {
            let error = TableRecord::read(&mut &vec![0; length][..], 112).unwrap_err();
            assert_eq!(error.offset, Some(112 + length as u64));
        }
        assert!(DirectoryHeader::read(&mut &[0; 12][..], 0).is_ok());
        assert!(TableRecord::read(&mut &[0; 16][..], 12).is_ok());
    }

    #[test]
    fn parses_directory_fields_without_alignment_assumptions() {
        let bytes = [0xff, 0, 1, 0, 0, 0, 3, 0, 32, 0, 1, 0, 16];
        assert_eq!(
            DirectoryHeader::parse(&bytes[1..]),
            Ok(DirectoryHeader {
                version: 0x10000,
                num_tables: 3,
                search_range: 32,
                entry_selector: 1,
                range_shift: 16,
            })
        );
        let bytes = [0xff, b'h', b'e', b'a', b'd', 0x80, 0, 0, 1, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 3];
        assert_eq!(
            TableRecord::parse(&bytes[1..]),
            Ok(TableRecord {
                tag: *b"head",
                checksum: 0x80000001,
                offset: u32::MAX,
                length: 3,
            })
        );
    }

    #[test]
    fn reports_every_truncated_directory_prefix() {
        for length in 0..12 {
            let error = DirectoryHeader::parse(&[0; 12][..length]).unwrap_err();
            let offset = if length < 4 { 0 } else { 4 + (length - 4) / 2 * 2 };
            assert_eq!(
                error,
                ReadError::Truncated {
                    offset,
                    needed: if length < 4 { 4 } else { 2 },
                    remaining: length - offset,
                }
            );
        }
        for length in 0..16 {
            assert_eq!(
                TableRecord::parse(&[0; 16][..length]),
                Err(ReadError::Truncated {
                    offset: length / 4 * 4,
                    needed: 4,
                    remaining: length % 4,
                })
            );
        }
    }

    #[test]
    fn checksum_pads_partial_words_and_wraps() {
        for (bytes, expected) in [
            (&[][..], 0),
            (&[1][..], 0x01000000),
            (&[1, 2][..], 0x01020000),
            (&[1, 2, 3][..], 0x01020300),
            (&[1, 2, 3, 4, 5][..], 0x06020304),
            (&[0xff, 0xff, 0xff, 0xff, 0, 0, 0, 1][..], 0),
        ] {
            assert_eq!(checksum(bytes), expected);
        }
    }
}
