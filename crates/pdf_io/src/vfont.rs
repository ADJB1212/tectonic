//! Checked virtual-font records with owned character packets and byte names.
#![forbid(unsafe_code)]

use crate::{byte_cursor::ByteCursor, error::PdfError, syntax::error};
use std::io::{Read, Seek, SeekFrom};

fn io(e: std::io::Error) -> PdfError {
    PdfError::io(c"VF file ended prematurely.".into(), e)
}

pub(crate) fn header(input: &mut impl Read) -> Result<u32, PdfError> {
    let mut preamble = [0; 3];
    input.read_exact(&mut preamble).map_err(io)?;
    if preamble[..2] != [247, 202] {
        return Err(error("VF file may be corrupt", 0));
    }
    let mut comment = [0; 255];
    input
        .read_exact(&mut comment[..preamble[2] as usize])
        .map_err(io)?;
    let mut fields = [0; 8];
    input.read_exact(&mut fields).map_err(io)?;
    let size = u32::from_be_bytes(fields[4..].try_into().unwrap());
    if size > i32::MAX as u32 {
        return Err(error("Invalid VF design size.", 7 + preamble[2] as usize));
    }
    Ok(size)
}

pub(crate) struct Font {
    pub(crate) id: u32,
    pub(crate) checksum: u32,
    pub(crate) size: u32,
    pub(crate) design_size: u32,
    pub(crate) directory: Vec<u8>,
    pub(crate) name: Vec<u8>,
}
pub(crate) enum Record {
    Font(Font),
    Character { code: u32, packet: Vec<u8> },
    End,
    Unknown(u8),
}

pub(crate) struct Records<R> {
    input: R,
    remaining: u64,
}
impl<R: Read + Seek> Records<R> {
    pub(crate) fn new(mut input: R) -> Result<Self, PdfError> {
        let start = input.stream_position().map_err(io)?;
        let end = input.seek(SeekFrom::End(0)).map_err(io)?;
        input.seek(SeekFrom::Start(start)).map_err(io)?;
        let remaining = end
            .checked_sub(start)
            .ok_or_else(|| error("VF file ended prematurely.", 0))?;
        Ok(Self { input, remaining })
    }
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), PdfError> {
        if bytes.len() as u64 > self.remaining {
            return Err(error("VF file ended prematurely.", 0));
        }
        self.input.read_exact(bytes).map_err(io)?;
        self.remaining -= bytes.len() as u64;
        Ok(())
    }
    fn value(&mut self, width: usize) -> Result<u32, PdfError> {
        let mut bytes = [0; 4];
        self.read(&mut bytes[..width])?;
        ByteCursor::new(&bytes[..width])
            .read_unsigned(width)
            .map_err(|_| error("Invalid VF integer width.", 0))
    }
    fn bytes(&mut self, length: u32) -> Result<Vec<u8>, PdfError> {
        if u64::from(length) > self.remaining || length > i32::MAX as u32 {
            return Err(error("VF file ended prematurely.", 0));
        }
        let mut bytes = vec![0; length as usize];
        self.read(&mut bytes)?;
        Ok(bytes)
    }
    pub(crate) fn next(&mut self) -> Result<Record, PdfError> {
        let opcode = self.value(1)? as u8;
        match opcode {
            243..=246 => {
                let id = self.value((opcode - 242) as usize)?;
                let checksum = self.value(4)?;
                let size = self.value(4)?;
                let design_size = self.value(4)?;
                if size > i32::MAX as u32 || design_size > i32::MAX as u32 {
                    return Err(error("Invalid VF font size.", 0));
                }
                let dir_len = self.value(1)?;
                let name_len = self.value(1)?;
                let directory = self.bytes(dir_len)?;
                let name = self.bytes(name_len)?;
                Ok(Record::Font(Font {
                    id,
                    checksum,
                    size,
                    design_size,
                    directory,
                    name,
                }))
            }
            0..=242 => {
                let (length, code) = if opcode == 242 {
                    (self.value(4)?, self.value(4)?)
                } else {
                    (u32::from(opcode), self.value(1)?)
                };
                self.value(if opcode == 242 { 4 } else { 3 })?;
                if code >= 0x1000000 {
                    return Err(error(
                        "Long character (>24 bits) in VF file.\nI can't handle long characters!\n",
                        0,
                    ));
                }
                Ok(Record::Character {
                    code,
                    packet: self.bytes(length)?,
                })
            }
            248 => Ok(Record::End),
            opcode => Ok(Record::Unknown(opcode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn packet_lengths_and_unsigned_character_codes_are_checked() {
        let bytes = [2, 255, 0, 0, 0, 138, 140, 248];
        let mut records = Records::new(Cursor::new(bytes)).unwrap();
        let Record::Character { code, packet } = records.next().unwrap() else {
            panic!()
        };
        assert_eq!(code, 255);
        assert_eq!(packet, [138, 140]);
        assert!(matches!(records.next().unwrap(), Record::End));
        for end in 0..7 {
            assert!(
                Records::new(Cursor::new(&bytes[..end]))
                    .unwrap()
                    .next()
                    .is_err()
            );
        }
        assert!(
            Records::new(Cursor::new([
                242, 255, 255, 255, 255, 0, 0, 0, 1, 0, 0, 0, 0
            ]))
            .unwrap()
            .next()
            .is_err()
        );
    }
}
