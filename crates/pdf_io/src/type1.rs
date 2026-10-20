//! Owned PFB segments and checked Type 1 decryption.
#![forbid(unsafe_code)]

use crate::{error::PdfError, syntax::error};
use std::io::{Read, Seek, SeekFrom};

fn io(e: std::io::Error) -> PdfError {
    PdfError::io(c"Reading PFB segment failed.".into(), e)
}

pub(crate) fn segment(input: &mut (impl Read + Seek), expected: u8) -> Result<Vec<u8>, PdfError> {
    let mut output = Vec::new();
    let mut file_end = None;
    loop {
        let start = input.stream_position().map_err(io)?;
        let mut marker = [0];
        if input.read(&mut marker).map_err(io)? == 0 {
            break;
        }
        if marker != [128] {
            return Err(error("Not a pfb file?", start as usize));
        }
        let mut kind = [0];
        input.read_exact(&mut kind).map_err(io)?;
        if kind != [expected] {
            input.seek(SeekFrom::Start(start)).map_err(io)?;
            break;
        }
        let mut length = [0; 4];
        input.read_exact(&mut length).map_err(io)?;
        let length = u32::from_le_bytes(length) as usize;
        let new_len = output
            .len()
            .checked_add(length)
            .filter(|&n| n < i32::MAX as usize)
            .ok_or_else(|| error("PFB segment is too large.", start as usize))?;
        let position = input.stream_position().map_err(io)?;
        let end = match file_end {
            Some(end) => end,
            None => {
                let end = input.seek(SeekFrom::End(0)).map_err(io)?;
                input.seek(SeekFrom::Start(position)).map_err(io)?;
                *file_end.insert(end)
            }
        };
        if length as u64 > end.saturating_sub(position) {
            return Err(error("Reading PFB segment failed.", position as usize));
        }
        let old_len = output.len();
        output
            .try_reserve(length)
            .map_err(|_| error("PFB segment is too large.", start as usize))?;
        output.resize(new_len, 0);
        input.read_exact(&mut output[old_len..]).map_err(io)?;
    }
    if output.is_empty() {
        return Err(error("PFB segment length zero?", 0));
    }
    Ok(output)
}

fn update(key: u16, byte: u8) -> u16 {
    key.wrapping_add(byte.into()).wrapping_mul(52845).wrapping_add(22719)
}

pub(crate) fn decrypt(input: &[u8], output: &mut [u8], mut key: u16, skip: usize) -> Result<(), PdfError> {
    let length = input
        .len()
        .checked_sub(skip)
        .filter(|&n| n <= output.len())
        .ok_or_else(|| error("Invalid Type 1 encrypted string length.", 0))?;
    for &byte in &input[..skip] {
        key = update(key, byte);
    }
    for (&byte, target) in input[skip..].iter().zip(&mut output[..length]) {
        *target = byte ^ (key >> 8) as u8;
        key = update(key, byte);
    }
    Ok(())
}

pub(crate) fn decrypt_in_place(bytes: &mut [u8], mut key: u16, skip: usize) -> Result<(), PdfError> {
    if skip > bytes.len() {
        return Err(error("Invalid Type 1 encrypted string length.", 0));
    }
    for &byte in &bytes[..skip] {
        key = update(key, byte);
    }
    for i in skip..bytes.len() {
        let byte = bytes[i];
        bytes[i - skip] = byte ^ (key >> 8) as u8;
        key = update(key, byte);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn coalescing_many_segments_checks_file_end_once() {
        struct Input {
            cursor: Cursor<Vec<u8>>,
            end_seeks: usize,
        }
        impl Read for Input {
            fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
                self.cursor.read(bytes)
            }
        }
        impl Seek for Input {
            fn seek(&mut self, position: SeekFrom) -> std::io::Result<u64> {
                if matches!(position, SeekFrom::End(_)) {
                    self.end_seeks += 1;
                }
                self.cursor.seek(position)
            }
        }
        let mut bytes = Vec::new();
        for _ in 0..128 {
            bytes.extend_from_slice(b"\x80\x01\x01\0\0\0a");
        }
        bytes.extend_from_slice(b"\x80\x03");
        let mut input = Input {
            cursor: Cursor::new(bytes),
            end_seeks: 0,
        };
        assert_eq!(segment(&mut input, 1).unwrap(), vec![b'a'; 128]);
        assert_eq!(input.end_seeks, 1);
        assert_eq!(input.cursor.position(), 128 * 7);
    }

    #[test]
    fn pfb_segments_coalesce_and_leave_the_next_header_unread() {
        let bytes = b"\x80\x01\x02\0\0\0ab\x80\x01\x01\0\0\0c\x80\x02\x01\0\0\0d\x80\x03";
        let mut input = Cursor::new(bytes);
        assert_eq!(segment(&mut input, 1).unwrap(), b"abc");
        assert_eq!(segment(&mut input, 2).unwrap(), b"d");
        assert_eq!(&bytes[input.position() as usize..], b"\x80\x03");
        for end in 1..8 {
            assert!(segment(&mut Cursor::new(&bytes[..end]), 1).is_err());
        }
        assert!(segment(&mut Cursor::new(b"\x80\x01\xff\xff\xff\xff"), 1).is_err());
    }
    #[test]
    fn separate_and_in_place_decryption_match_with_skipped_prefixes() {
        for skip in 0..=16 {
            let encrypted: Vec<_> = (0..16).collect();
            let mut separate = vec![0; 16 - skip];
            decrypt(&encrypted, &mut separate, 4330, skip).unwrap();
            let mut in_place = encrypted.clone();
            decrypt_in_place(&mut in_place, 4330, skip).unwrap();
            assert_eq!(&in_place[..16 - skip], separate);
        }
        assert!(decrypt(&[1], &mut [], 4330, 2).is_err());
        assert!(decrypt_in_place(&mut [1], 4330, 2).is_err());
    }
}
