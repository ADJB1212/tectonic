//! Checked reads from borrowed binary data.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReadError {
    Truncated {
        offset: usize,
        needed: usize,
        remaining: usize,
    },
    InvalidWidth {
        offset: usize,
        width: usize,
    },
}

pub(crate) struct ByteCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> ByteCursor<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub(crate) fn offset(&self) -> usize {
        self.offset
    }

    pub(crate) fn remaining(&self) -> usize {
        self.bytes.len() - self.offset
    }

    pub(crate) fn read(&mut self, length: usize) -> Result<&'a [u8], ReadError> {
        if length > self.remaining() {
            return Err(ReadError::Truncated {
                offset: self.offset,
                needed: length,
                remaining: self.remaining(),
            });
        }
        let start = self.offset;
        self.offset += length;
        Ok(&self.bytes[start..self.offset])
    }

    #[allow(dead_code)]
    pub(crate) fn skip(&mut self, length: usize) -> Result<(), ReadError> {
        self.read(length).map(|_| ())
    }

    pub(crate) fn read_unsigned(&mut self, width: usize) -> Result<u32, ReadError> {
        if !(1..=4).contains(&width) {
            return Err(ReadError::InvalidWidth {
                offset: self.offset,
                width,
            });
        }
        Ok(self
            .read(width)?
            .iter()
            .fold(0, |value, &byte| (value << 8) | u32::from(byte)))
    }

    pub(crate) fn read_signed(&mut self, width: usize) -> Result<i32, ReadError> {
        let value = self.read_unsigned(width)?;
        let shift = (4 - width) * 8;
        Ok(((value << shift) as i32) >> shift)
    }

    #[allow(dead_code)]
    pub(crate) fn read_u8(&mut self) -> Result<u8, ReadError> {
        self.read_unsigned(1).map(|value| value as u8)
    }

    pub(crate) fn read_u16(&mut self) -> Result<u16, ReadError> {
        self.read_unsigned(2).map(|value| value as u16)
    }

    pub(crate) fn read_u32(&mut self) -> Result<u32, ReadError> {
        self.read_unsigned(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_big_endian_values_at_unaligned_offsets() {
        let mut cursor = ByteCursor::new(&[0, 0x80, 1, 0xff, 0xfe, 0xdc, 0xba, 0x98, 0x76]);
        cursor.skip(1).unwrap();
        assert_eq!(cursor.read_u8(), Ok(0x80));
        assert_eq!(cursor.read_u16(), Ok(0x01ff));
        assert_eq!(cursor.read_u32(), Ok(0xfedcba98));
        assert_eq!(cursor.read_u8(), Ok(0x76));
        assert_eq!(cursor.remaining(), 0);
    }

    #[test]
    fn sign_extends_all_supported_widths() {
        for width in 1..=4 {
            assert_eq!(ByteCursor::new(&[0xff; 4]).read_signed(width), Ok(-1));
            assert_eq!(
                ByteCursor::new(&[0x80, 0, 0, 0]).read_signed(width),
                Ok(i32::MIN >> ((4 - width) * 8))
            );
            assert_eq!(
                ByteCursor::new(&[0x7f, 0xff, 0xff, 0xff]).read_signed(width),
                Ok(i32::MAX >> ((4 - width) * 8))
            );
        }
    }

    #[test]
    fn failures_preserve_offsets_and_distinguish_invalid_widths() {
        let mut cursor = ByteCursor::new(&[1, 2, 3]);
        cursor.skip(1).unwrap();
        for length in [3, usize::MAX] {
            assert_eq!(
                cursor.skip(length),
                Err(ReadError::Truncated {
                    offset: 1,
                    needed: length,
                    remaining: 2,
                })
            );
            assert_eq!(cursor.offset(), 1);
        }
        for width in [0, 5, usize::MAX] {
            assert_eq!(
                cursor.read_unsigned(width),
                Err(ReadError::InvalidWidth { offset: 1, width })
            );
            assert_eq!(cursor.offset(), 1);
        }
        assert_eq!(cursor.read_u16(), Ok(0x0203));
        assert_eq!(cursor.read(0), Ok(&[][..]));
        assert_eq!(
            cursor.read_u8(),
            Err(ReadError::Truncated {
                offset: 3,
                needed: 1,
                remaining: 0
            })
        );
    }
}
