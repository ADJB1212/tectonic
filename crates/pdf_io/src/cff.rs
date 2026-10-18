//! Checked CFF index and dictionary operands.
#![forbid(unsafe_code)]

use crate::{byte_cursor::ByteCursor, error::PdfError, syntax::error};
use std::io::Read;

pub(crate) struct IndexHeader {
    pub(crate) count: u16,
    pub(crate) width: u8,
    pub(crate) offsets: Vec<u32>,
}

impl IndexHeader {
    pub(crate) fn read(input: &mut impl Read) -> Result<Self, PdfError> {
        fn read(input: &mut impl Read, bytes: &mut [u8]) -> Result<(), PdfError> {
            input
                .read_exact(bytes)
                .map_err(|e| PdfError::io(c"File ended prematurely\n".into(), e))
        }
        let mut count = [0; 2];
        read(input, &mut count)?;
        let count = u16::from_be_bytes(count);
        if count == 0 {
            return Ok(Self {
                count,
                width: 0,
                offsets: Vec::new(),
            });
        }
        let mut width = [0];
        read(input, &mut width)?;
        let width = width[0];
        if !(1..=4).contains(&width) {
            return Err(error("invalid offsize data", 2));
        }
        let mut bytes = vec![0; (usize::from(count) + 1) * usize::from(width)];
        read(input, &mut bytes)?;
        let mut cursor = ByteCursor::new(&bytes);
        let mut offsets = Vec::with_capacity(usize::from(count) + 1);
        for _ in 0..=count {
            let offset = cursor
                .read_unsigned(width.into())
                .map_err(|_| error("Invalid CFF Index offset data", cursor.offset()))?;
            if offsets.last().is_some_and(|&previous| offset < previous) {
                return Err(error("Invalid CFF Index offset data", cursor.offset()));
            }
            offsets.push(offset);
        }
        if offsets[0] != 1 {
            return Err(error("Invalid CFF Index offset data", 3));
        }
        Ok(Self {
            count,
            width,
            offsets,
        })
    }

    pub(crate) fn data_len(&self) -> usize {
        self.offsets
            .last()
            .map_or(0, |offset| (*offset - 1) as usize)
    }
}

pub(crate) fn integer(bytes: &[u8]) -> Result<(f64, usize), PdfError> {
    let mut cursor = ByteCursor::new(bytes);
    let mut read = |width| {
        cursor
            .read_signed(width)
            .map_err(|_| error("Invalid CFF integer", cursor.offset()))
    };
    let Some(&first) = bytes.first() else {
        return Err(error("Invalid CFF integer", 0));
    };
    let value = match first {
        28 => {
            read(1)?;
            read(2)?
        }
        29 => {
            read(1)?;
            read(4)?
        }
        32..=246 => i32::from(first) - 139,
        247..=250 => {
            (i32::from(first) - 247) * 256
                + i32::from(
                    *bytes
                        .get(1)
                        .ok_or_else(|| error("Invalid CFF integer", 1))?,
                )
                + 108
        }
        251..=254 => {
            -(i32::from(first) - 251) * 256
                - i32::from(
                    *bytes
                        .get(1)
                        .ok_or_else(|| error("Invalid CFF integer", 1))?,
                )
                - 108
        }
        _ => return Err(error("Invalid CFF integer", 0)),
    };
    let consumed = match first {
        28 => 3,
        29 => 5,
        247..=254 => 2,
        _ => 1,
    };
    Ok((f64::from(value), consumed))
}

pub(crate) fn real(bytes: &[u8]) -> Result<(f64, usize), PdfError> {
    if bytes.first() != Some(&30) {
        return Err(error("Invalid CFF real", 0));
    }
    let mut encoded = String::new();
    for (offset, &byte) in bytes.iter().enumerate().skip(1) {
        for (part, nibble) in [byte >> 4, byte & 15].into_iter().enumerate() {
            match nibble {
                0..=9 => encoded.push(char::from(b'0' + nibble)),
                10 => encoded.push('.'),
                11 => encoded.push('e'),
                12 => encoded.push_str("e-"),
                13 => {}
                14 => encoded.push('-'),
                15 => {
                    if part == 0 && byte != 255 {
                        return Err(error("Invalid CFF real", offset));
                    }
                    let value: f64 = encoded
                        .parse()
                        .map_err(|_| error("Invalid CFF real", offset))?;
                    if !value.is_finite() {
                        return Err(error("Invalid CFF real", offset));
                    }
                    return Ok((value, offset + 1));
                }
                _ => unreachable!(),
            }
            if encoded.len() >= 128 {
                return Err(error("Invalid CFF real", offset));
            }
        }
    }
    Err(error("Invalid CFF real", bytes.len()))
}

pub(crate) struct DictEntry {
    pub(crate) operator: usize,
    pub(crate) values: Vec<f64>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum DictError {
    Parse,
    Overflow,
    Underflow,
}

pub(crate) fn dictionary(
    bytes: &[u8],
    operator: impl Fn(usize) -> Option<bool>,
) -> Result<(Vec<DictEntry>, bool), DictError> {
    let mut pos = 0;
    let mut stack = Vec::new();
    let mut entries = Vec::new();
    while let Some(&byte) = bytes.get(pos) {
        if byte < 22 {
            pos += 1;
            let id = if byte == 12 {
                let second = *bytes.get(pos).ok_or(DictError::Parse)?;
                pos += 1;
                if second >= 39 {
                    return Err(DictError::Parse);
                }
                usize::from(second) + 22
            } else {
                usize::from(byte)
            };
            if let Some(scalar) = operator(id) {
                if scalar {
                    entries.push(DictEntry {
                        operator: id,
                        values: vec![stack.pop().ok_or(DictError::Underflow)?],
                    });
                } else if !stack.is_empty() {
                    entries.push(DictEntry {
                        operator: id,
                        values: std::mem::take(&mut stack),
                    });
                }
            }
        } else if matches!(byte, 22..=27 | 255) {
            pos += 1;
        } else {
            if stack.len() >= 64 {
                return Err(DictError::Overflow);
            }
            let (value, consumed) = if byte == 30 {
                real(&bytes[pos..])
            } else {
                integer(&bytes[pos..])
            }
            .map_err(|_| DictError::Parse)?;
            stack.push(value);
            pos += consumed;
        }
    }
    Ok((entries, !stack.is_empty()))
}

struct Reader<'a, R> {
    input: &'a mut R,
    offset: usize,
}

impl<R: Read> Reader<'_, R> {
    fn value(&mut self, width: usize) -> Result<u32, PdfError> {
        let mut bytes = [0; 4];
        self.input.read_exact(&mut bytes[..width]).map_err(|e| {
            PdfError::io(c"File ended prematurely\n".into(), e).at(self.offset as u64)
        })?;
        self.offset += width;
        ByteCursor::new(&bytes[..width])
            .read_unsigned(width)
            .map_err(|_| error("Invalid CFF value", self.offset))
    }
    fn byte(&mut self) -> Result<u8, PdfError> {
        self.value(1).map(|v| v as u8)
    }
    fn pair(&mut self) -> Result<u16, PdfError> {
        self.value(2).map(|v| v as u16)
    }
}

pub(crate) enum Charset {
    Glyphs(Vec<u16>),
    Ranges { format: u8, ranges: Vec<(u16, u16)> },
}

pub(crate) fn charset(input: &mut impl Read, glyphs: u16) -> Result<(Charset, usize), PdfError> {
    let mut input = Reader { input, offset: 0 };
    let format = input.byte()?;
    let mut remaining = usize::from(
        glyphs
            .checked_sub(1)
            .ok_or_else(|| error("Charset data possibly broken", 0))?,
    );
    if format == 0 {
        let mut values = Vec::with_capacity(remaining);
        for _ in 0..remaining {
            values.push(input.pair()?);
        }
        return Ok((Charset::Glyphs(values), input.offset));
    }
    if !matches!(format, 1 | 2) {
        return Err(error("Unknown Charset format", 0));
    }
    let mut ranges = Vec::new();
    while remaining > 0 {
        let first = input.pair()?;
        let left = if format == 1 {
            u16::from(input.byte()?)
        } else {
            input.pair()?
        };
        let count = usize::from(left) + 1;
        if count > remaining || first.checked_add(left).is_none() {
            return Err(error("Charset data possibly broken", input.offset));
        }
        ranges.push((first, left));
        remaining -= count;
    }
    Ok((Charset::Ranges { format, ranges }, input.offset))
}

pub(crate) enum EncodingValues {
    Codes(Vec<u8>),
    Ranges(Vec<(u8, u8)>),
}
pub(crate) struct Encoding {
    pub(crate) format: u8,
    pub(crate) values: EncodingValues,
    pub(crate) supplements: Vec<(u8, u16)>,
}

pub(crate) fn encoding(input: &mut impl Read) -> Result<(Encoding, usize), PdfError> {
    let mut input = Reader { input, offset: 0 };
    let format = input.byte()?;
    let count = input.byte()?;
    let values = match format & 127 {
        0 => {
            let mut values = Vec::with_capacity(count.into());
            for _ in 0..count {
                values.push(input.byte()?);
            }
            EncodingValues::Codes(values)
        }
        1 => {
            let mut ranges = Vec::with_capacity(count.into());
            for _ in 0..count {
                let first = input.byte()?;
                let left = input.byte()?;
                if first.checked_add(left).is_none() {
                    return Err(error("Invalid CFF encoding range", input.offset));
                }
                ranges.push((first, left));
            }
            EncodingValues::Ranges(ranges)
        }
        _ => return Err(error("Unknown Encoding format", 0)),
    };
    let mut supplements = Vec::new();
    if format & 128 != 0 {
        let count = input.byte()?;
        for _ in 0..count {
            supplements.push((input.byte()?, input.pair()?));
        }
    }
    Ok((
        Encoding {
            format,
            values,
            supplements,
        },
        input.offset,
    ))
}

pub(crate) enum FdSelect {
    Fds(Vec<u8>),
    Ranges(Vec<(u16, u8)>),
}

pub(crate) fn fdselect(input: &mut impl Read, glyphs: u16) -> Result<(FdSelect, usize), PdfError> {
    let mut input = Reader { input, offset: 0 };
    match input.byte()? {
        0 => {
            let mut fds = Vec::with_capacity(glyphs.into());
            for _ in 0..glyphs {
                fds.push(input.byte()?);
            }
            Ok((FdSelect::Fds(fds), input.offset))
        }
        3 => {
            let count = input.pair()?;
            let mut ranges = Vec::with_capacity(count.into());
            for _ in 0..count {
                let first = input.pair()?;
                let fd = input.byte()?;
                if first >= glyphs
                    || ranges
                        .last()
                        .is_some_and(|&(previous, _)| first <= previous)
                {
                    return Err(error("Invalid FDSelect range.", input.offset));
                }
                ranges.push((first, fd));
            }
            if ranges.first().is_none_or(|&(first, _)| first != 0) {
                return Err(error("Range not starting with 0.", input.offset));
            }
            if input.pair()? != glyphs {
                return Err(error(
                    "Sentinel value mismatched with number of glyphs.",
                    input.offset,
                ));
            }
            Ok((FdSelect::Ranges(ranges), input.offset))
        }
        _ => Err(error("Unknown FDSelect format.", 0)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_checks_width_monotonicity_truncation_and_maximum_count() {
        for bytes in [
            b"".as_slice(),
            &[0],
            &[0, 1, 0],
            &[0, 1, 5],
            &[0, 2, 1, 1, 3, 2],
            &[0, 1, 1, 0, 1],
            &[0, 1, 1, 1],
        ] {
            assert!(IndexHeader::read(&mut &bytes[..]).is_err());
        }
        let mut bytes = vec![255, 255, 1];
        bytes.extend_from_slice(&[1; 65536]);
        let header = IndexHeader::read(&mut bytes.as_slice()).unwrap();
        assert_eq!(header.offsets.len(), 65536);
        assert_eq!(header.data_len(), 0);
    }

    #[test]
    fn operands_accept_exact_end_and_reject_truncation() {
        assert_eq!(integer(&[28, 255, 254]).unwrap(), (-2.0, 3));
        assert_eq!(
            integer(&[29, 128, 0, 0, 0]).unwrap(),
            (f64::from(i32::MIN), 5)
        );
        for bytes in [&[28][..], &[29, 1], &[247], &[254]] {
            assert!(integer(bytes).is_err());
        }
        assert_eq!(real(&[30, 0x1a, 0x5f]).unwrap(), (1.5, 3));
        assert!(real(&[30, 0x12]).is_err());
    }

    #[test]
    fn dictionaries_bound_operands_and_always_consume_unknown_operators() {
        let operators = |id| match id {
            0 => Some(true),
            1 => Some(false),
            _ => None,
        };
        let (entries, garbage) = dictionary(&[140, 141, 1, 2, 142, 0], operators).unwrap();
        assert!(!garbage);
        assert_eq!(entries[0].values, [1.0, 2.0]);
        assert_eq!(entries[1].values, [3.0]);
        assert!(matches!(
            dictionary(&[12], operators),
            Err(DictError::Parse)
        ));
        assert!(matches!(
            dictionary(&[0], operators),
            Err(DictError::Underflow)
        ));
        assert!(matches!(
            dictionary(&[139; 65], operators),
            Err(DictError::Overflow)
        ));
        assert!(dictionary(&[2, 12, 1], |_| None).is_ok());
    }

    #[test]
    fn charset_encoding_and_fdselect_ranges_are_bounded() {
        assert!(charset(&mut &[1, 0, 10, 3][..], 4).is_err());
        assert!(charset(&mut &[2, 255, 255, 0, 1][..], 3).is_err());
        assert!(charset(&mut &[0][..], 0).is_err());
        let (Charset::Ranges { ranges, .. }, length) = charset(&mut &[1, 0, 10, 2][..], 4).unwrap()
        else {
            panic!()
        };
        assert_eq!(ranges, [(10, 2)]);
        assert_eq!(length, 4);
        assert!(encoding(&mut &[1, 1, 250, 10][..]).is_err());
        assert!(encoding(&mut &[128, 1, 5, 1, 0][..]).is_err());
        assert!(fdselect(&mut &[3, 0, 0, 0, 1][..], 1).is_err());
        assert!(fdselect(&mut &[3, 0, 2, 0, 0, 0, 0, 0, 1, 0, 2][..], 2).is_err());
        let (FdSelect::Ranges(ranges), length) =
            fdselect(&mut &[3, 0, 1, 0, 0, 4, 0, 2][..], 2).unwrap()
        else {
            panic!()
        };
        assert_eq!(ranges, [(0, 4)]);
        assert_eq!(length, 8);
    }
}
