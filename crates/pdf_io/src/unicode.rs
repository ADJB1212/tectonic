//! Slice-based Unicode operations preserving the PDF engine's legacy encodings.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DecodeError {
    Truncated { needed: usize, remaining: usize },
    InvalidLead(u8),
    InvalidContinuation { offset: usize, byte: u8 },
    UnexpectedLowSurrogate(u16),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EncodeError {
    InvalidCodePoint(i32),
    BufferTooSmall { needed: usize, available: usize },
}

pub(crate) fn is_valid(code_point: i32) -> bool {
    (0..=0x10_ffff).contains(&code_point) && !(0xd800..=0xdfff).contains(&code_point)
}

fn require(bytes: &[u8], needed: usize) -> Result<(), DecodeError> {
    if bytes.len() < needed {
        Err(DecodeError::Truncated {
            needed,
            remaining: bytes.len(),
        })
    } else {
        Ok(())
    }
}

pub(crate) fn decode_utf8(bytes: &[u8]) -> Result<(i32, usize), DecodeError> {
    require(bytes, 1)?;
    let first = bytes[0];
    // Retain overlong and five/six-byte sequences accepted by the legacy decoder.
    let (mut value, length) = match first {
        0..=0x7f => (i32::from(first), 1),
        0xc0..=0xdf => (i32::from(first & 0x1f), 2),
        0xe0..=0xef => (i32::from(first & 0x0f), 3),
        0xf0..=0xf7 => (i32::from(first & 0x07), 4),
        0xf8..=0xfb => (i32::from(first & 0x03), 5),
        0xfc..=0xfd => (i32::from(first & 0x01), 6),
        _ => return Err(DecodeError::InvalidLead(first)),
    };
    require(bytes, length)?;
    for (offset, &byte) in bytes[..length].iter().enumerate().skip(1) {
        if byte & 0xc0 != 0x80 {
            return Err(DecodeError::InvalidContinuation { offset, byte });
        }
        value = (value << 6) | i32::from(byte & 0x3f);
    }
    Ok((value, length))
}

pub(crate) fn decode_utf16be(bytes: &[u8]) -> Result<(i32, usize), DecodeError> {
    require(bytes, 2)?;
    let first = u16::from_be_bytes([bytes[0], bytes[1]]);
    match first {
        0xd800..=0xdbff => {
            require(bytes, 4)?;
            let second = u16::from_be_bytes([bytes[2], bytes[3]]);
            // The legacy decoder masks the second word without validating its range.
            Ok((0x10000 + ((i32::from(first & 0x3ff) << 10) | i32::from(second & 0x3ff)), 4))
        }
        0xdc00..=0xdfff => Err(DecodeError::UnexpectedLowSurrogate(first)),
        _ => Ok((i32::from(first), 2)),
    }
}

fn valid_string(mut bytes: &[u8], decode: impl Fn(&[u8]) -> Result<(i32, usize), DecodeError>) -> bool {
    if bytes.is_empty() {
        return false;
    }
    while !bytes.is_empty() {
        match decode(bytes) {
            Ok((value, consumed)) if is_valid(value) => bytes = &bytes[consumed..],
            _ => return false,
        }
    }
    true
}

pub(crate) fn is_valid_utf8(bytes: &[u8]) -> bool {
    valid_string(bytes, decode_utf8)
}

pub(crate) fn is_valid_utf16be(bytes: &[u8]) -> bool {
    valid_string(bytes, decode_utf16be)
}

fn output(output: &mut [u8], length: usize) -> Result<&mut [u8], EncodeError> {
    let available = output.len();
    output.get_mut(..length).ok_or(EncodeError::BufferTooSmall { needed: length, available })
}

pub(crate) fn encode_utf8(value: i32, buffer: &mut [u8]) -> Result<usize, EncodeError> {
    if !is_valid(value) {
        return Err(EncodeError::InvalidCodePoint(value));
    }
    // DEL historically uses two bytes, unlike the other ASCII values.
    let length = match value {
        0..=0x7e => 1,
        0x7f..=0x7ff => 2,
        0x800..=0xffff => 3,
        _ => 4,
    };
    let bytes = output(buffer, length)?;
    let mut remaining = value as u32;
    for byte in bytes[1..].iter_mut().rev() {
        *byte = 0x80 | (remaining & 0x3f) as u8;
        remaining >>= 6;
    }
    bytes[0] = remaining as u8 | [0, 0, 0xc0, 0xe0, 0xf0][length];
    Ok(length)
}

pub(crate) fn encode_utf16be(value: i32, buffer: &mut [u8]) -> Result<usize, EncodeError> {
    if (0x1_0000..=0x10_ffff).contains(&value) {
        let bytes = output(buffer, 4)?;
        let value = value as u32 - 0x10000;
        bytes[..2].copy_from_slice(&(0xd800 | (value >> 10) as u16).to_be_bytes());
        bytes[2..].copy_from_slice(&(0xdc00 | (value & 0x3ff) as u16).to_be_bytes());
        Ok(4)
    } else {
        let value = if (0..=0xffff).contains(&value) { value as u16 } else { 0xfffd };
        output(buffer, 2)?.copy_from_slice(&value.to_be_bytes());
        Ok(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_unicode_boundaries() {
        for value in [0, 0x7e, 0x7f, 0x80, 0x7ff, 0x800, 0xd7ff, 0xe000, 0xffff, 0x10000, 0x10ffff] {
            let mut bytes = [0; 6];
            let length = encode_utf8(value, &mut bytes).unwrap();
            assert_eq!(decode_utf8(&bytes[..length]), Ok((value, length)));
            assert!(is_valid_utf8(&bytes[..length]));
            let length = encode_utf16be(value, &mut bytes).unwrap();
            assert_eq!(decode_utf16be(&bytes[..length]), Ok((value, length)));
            assert!(is_valid_utf16be(&bytes[..length]));
        }
    }

    #[test]
    fn encodes_exact_bytes_at_width_boundaries() {
        for (value, utf8, utf16be) in [
            (0, &b"\0"[..], &b"\0\0"[..]),
            (0x7f, &b"\xc1\xbf"[..], &b"\0\x7f"[..]),
            (0x80, &b"\xc2\x80"[..], &b"\0\x80"[..]),
            (0x7ff, &b"\xdf\xbf"[..], &b"\x07\xff"[..]),
            (0x800, &b"\xe0\xa0\x80"[..], &b"\x08\0"[..]),
            (0xffff, &b"\xef\xbf\xbf"[..], &b"\xff\xff"[..]),
            (0x10000, &b"\xf0\x90\x80\x80"[..], &b"\xd8\0\xdc\0"[..]),
            (0x10ffff, &b"\xf4\x8f\xbf\xbf"[..], &b"\xdb\xff\xdf\xff"[..]),
        ] {
            let mut bytes = [0xaa; 6];
            let length = encode_utf8(value, &mut bytes).unwrap();
            assert_eq!(&bytes[..length], utf8);
            let length = encode_utf16be(value, &mut bytes).unwrap();
            assert_eq!(&bytes[..length], utf16be);
        }
    }

    #[test]
    fn reports_truncation_and_invalid_encoding() {
        for bytes in [
            &b"\xc2\xa2"[..],
            &b"\xe2\x82\xac"[..],
            &b"\xf0\x90\x80\x80"[..],
            &b"\xfc\x80\x80\x80\x80\x80"[..],
        ] {
            for length in 0..bytes.len() {
                assert!(matches!(decode_utf8(&bytes[..length]), Err(DecodeError::Truncated { .. })));
            }
        }
        for length in 0..4 {
            assert!(matches!(
                decode_utf16be(&[0xd8, 0, 0xdc, 0][..length]),
                Err(DecodeError::Truncated { .. })
            ));
        }
        assert_eq!(decode_utf8(&[0x80]), Err(DecodeError::InvalidLead(0x80)));
        assert_eq!(decode_utf8(&[0xff]), Err(DecodeError::InvalidLead(0xff)));
        assert_eq!(decode_utf8(&[0xc2, 0]), Err(DecodeError::InvalidContinuation { offset: 1, byte: 0 }));
        assert_eq!(decode_utf16be(&[0xdc, 0]), Err(DecodeError::UnexpectedLowSurrogate(0xdc00)));
        assert!(!is_valid_utf8(&[0xed, 0xa0, 0x80]));
        assert!(!is_valid_utf8(&[0xf4, 0x90, 0x80, 0x80]));
        assert!(!is_valid_utf8(&[]));
        assert!(!is_valid_utf16be(&[]));
        assert!(!is_valid_utf16be(&[0, 65, 0]));
    }

    #[test]
    fn short_output_buffers_are_unchanged() {
        for value in [0, 0x7f, 0x800, 0x10000, 0x10ffff] {
            for encode in [encode_utf8, encode_utf16be] {
                let mut bytes = [0xaa; 8];
                let needed = encode(value, &mut bytes).unwrap();
                for available in 0..needed {
                    let mut bytes = [0xaa; 8];
                    assert_eq!(
                        encode(value, &mut bytes[..available]),
                        Err(EncodeError::BufferTooSmall { needed, available })
                    );
                    assert_eq!(bytes, [0xaa; 8]);
                }
            }
        }
        for value in [-1, 0xd800, 0xdfff, 0x110000, i32::MAX] {
            assert_eq!(encode_utf8(value, &mut [0; 4]), Err(EncodeError::InvalidCodePoint(value)));
        }
    }
}
