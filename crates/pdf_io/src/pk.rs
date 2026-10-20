//! Bounded PK glyph raster decoding with a reusable row buffer.
#![forbid(unsafe_code)]

use crate::{error::PdfError, syntax::error};

pub(crate) struct Header {
    pub(crate) packet_length: u32,
    pub(crate) code: i32,
    pub(crate) width: i32,
    pub(crate) dx: i32,
    pub(crate) dy: i32,
    pub(crate) bitmap_width: u32,
    pub(crate) bitmap_height: u32,
    pub(crate) horizontal_offset: i32,
    pub(crate) vertical_offset: i32,
}

pub(crate) fn header(input: &mut impl std::io::Read, opcode: u8) -> Result<Header, PdfError> {
    let long = opcode & 7 == 7;
    let short = opcode & 4 == 0;
    let mut bytes = [0; 36];
    let length = if short {
        10
    } else if long {
        36
    } else {
        16
    };
    input
        .read_exact(&mut bytes[..length])
        .map_err(|e| PdfError::io(c"File ended prematurely\n".into(), e))?;
    let mut cursor = crate::byte_cursor::ByteCursor::new(&bytes[..length]);
    let mut read = |width| {
        cursor
            .read_unsigned(width)
            .map_err(|_| error("Invalid PK character header.", cursor.offset()))
    };
    let packet = if short {
        u32::from(opcode & 3) * 256 + read(1)?
    } else if long {
        read(4)?
    } else {
        u32::from(opcode & 3) * 65536 + read(2)?
    };
    let code = read(if long { 4 } else { 1 })? as i32;
    let width = read(if long { 4 } else { 3 })? as i32;
    let dx = if long {
        read(4)? as i32
    } else {
        (read(if short { 1 } else { 2 })? << 16) as i32
    };
    let dy = if long { read(4)? as i32 } else { 0 };
    let field = if short {
        1
    } else if long {
        4
    } else {
        2
    };
    let bitmap_width = read(field)?;
    let bitmap_height = read(field)?;
    let horizontal_offset = cursor
        .read_signed(field)
        .map_err(|_| error("Invalid PK character header.", cursor.offset()))?;
    let vertical_offset = cursor
        .read_signed(field)
        .map_err(|_| error("Invalid PK character header.", cursor.offset()))?;
    let packet_length = packet
        .checked_sub(if short {
            8
        } else if long {
            28
        } else {
            13
        })
        .filter(|&n| n <= i32::MAX as u32)
        .ok_or_else(|| error("Invalid PK packet length.", 0))?;
    if bitmap_width > i32::MAX as u32 || bitmap_height > i32::MAX as u32 {
        return Err(error("PK raster is too large.", 0));
    }
    Ok(Header {
        packet_length,
        code,
        width,
        dx,
        dy,
        bitmap_width,
        bitmap_height,
        horizontal_offset,
        vertical_offset,
    })
}

struct Nibbles<'a> {
    bytes: &'a [u8],
    position: usize,
}
impl Nibbles<'_> {
    fn next(&mut self) -> Result<u8, PdfError> {
        let byte = self
            .bytes
            .get(self.position / 2)
            .copied()
            .ok_or_else(|| error("EOD reached while unpacking pk_packed_num.", self.position / 2))?;
        let value = if self.position.is_multiple_of(2) { byte >> 4 } else { byte & 15 };
        self.position += 1;
        Ok(value)
    }
    fn number(&mut self, dynamic: u8) -> Result<usize, PdfError> {
        let first = self.next()?;
        let value = match first {
            0 => {
                let mut digits = 0;
                let mut value = loop {
                    let nibble = self.next()?;
                    digits += 1;
                    if nibble != 0 {
                        break usize::from(nibble);
                    }
                    if digits > std::mem::size_of::<usize>() * 2 {
                        return Err(error("Invalid PK run length.", self.position / 2));
                    }
                };
                for _ in 0..digits {
                    value = value
                        .checked_mul(16)
                        .and_then(|v| v.checked_add(usize::from(self.next().ok()?)))
                        .ok_or_else(|| error("Invalid PK run length.", self.position / 2))?;
                }
                let adjustment = (13 - i32::from(dynamic)) * 16 + i32::from(dynamic) - 15;
                value
                    .checked_add_signed(adjustment as isize)
                    .ok_or_else(|| error("Invalid PK run length.", self.position / 2))?
            }
            value if value <= dynamic => usize::from(value),
            value @ 1..=13 => usize::from(value - dynamic - 1) * 16 + usize::from(self.next()?) + usize::from(dynamic) + 1,
            _ => return Err(error("Invalid PK repeat count.", self.position / 2)),
        };
        if value == 0 {
            return Err(error("Invalid PK run length.", self.position / 2));
        }
        Ok(value)
    }
}

pub(crate) fn decode(
    bytes: &[u8], width: u32, height: u32, dynamic: u8, mut color: bool, mut emit: impl FnMut(&[u8]), mut warning: impl FnMut(&'static str),
) -> Result<(), PdfError> {
    let width = width as usize;
    let height = height as usize;
    let row_bytes = width.checked_add(7).map(|n| n / 8).ok_or_else(|| error("PK raster is too large.", 0))?;
    let pixels = width.checked_mul(height).ok_or_else(|| error("PK raster is too large.", 0))?;
    if row_bytes.checked_mul(height).is_none_or(|n| n > i32::MAX as usize) {
        return Err(error("PK raster is too large.", 0));
    }
    if width == 0 || height == 0 {
        return Ok(());
    }
    if dynamic > 14 {
        return Err(error("Invalid PK packing parameter.", 0));
    }
    let mut row = vec![0; row_bytes];
    if dynamic == 14 {
        if color {
            warning("run_color != 0 for bitmap pk data?");
        }
        if bytes.len() < pixels.div_ceil(8) {
            return Err(error("Insufficient bitmap pk data.", bytes.len()));
        }
        for y in 0..height {
            row.fill(0);
            for x in 0..width {
                let bit = y * width + x;
                // Retain the historical output bit position for non-byte-aligned widths.
                if bytes[bit / 8] & (128 >> (bit % 8)) == 0 {
                    row[x / 8] |= 128 >> (bit % 8);
                }
            }
            emit(&row);
        }
        return Ok(());
    }
    let mut input = Nibbles { bytes, position: 0 };
    let (mut y, mut run) = (0, 0usize);
    while y < height {
        row.fill(255);
        let (mut x, mut repeat) = (0, 0);
        while x < width {
            if run == 0 {
                let start = input.position;
                let nibble = input.next()?;
                if nibble >= 14 {
                    if repeat != 0 {
                        warning("Second repeat count for this row!");
                    }
                    repeat = if nibble == 14 { input.number(dynamic)? } else { 1 };
                    continue;
                }
                input.position = start;
                run = input.number(dynamic)?;
                color = !color;
            }
            let count = run.min(width - x);
            if !color {
                let end = x + count;
                let first_byte = x / 8;
                let last_byte = (end - 1) / 8;
                let first_mask = 255u8 >> (x % 8);
                let last_mask = 255u8 << (7 - (end - 1) % 8);
                if first_byte == last_byte {
                    row[first_byte] &= !(first_mask & last_mask);
                } else {
                    row[first_byte] &= !first_mask;
                    row[first_byte + 1..last_byte].fill(0);
                    row[last_byte] &= !last_mask;
                }
            }
            run -= count;
            x += count;
        }
        if repeat >= height - y {
            return Err(error("Invalid PK repeat count.", input.position / 2));
        }
        for _ in 0..=repeat {
            emit(&row);
        }
        y += repeat + 1;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packed_runs_clear_partial_and_full_bytes() {
        for prefix in 1..=13 {
            for run in 1..=13 {
                for suffix in 1..=13 {
                    let bytes = [(prefix << 4) | run, suffix << 4];
                    let width = usize::from(prefix + run + suffix);
                    let mut expected = vec![255; width.div_ceil(8)];
                    for bit in usize::from(prefix)..usize::from(prefix + run) {
                        expected[bit / 8] &= !(128 >> (bit % 8));
                    }
                    let mut actual = Vec::new();
                    decode(&bytes, width as u32, 1, 13, false, |row| actual.extend_from_slice(row), |_| {}).unwrap();
                    assert_eq!(actual, expected, "prefix={prefix}, run={run}, suffix={suffix}");
                }
            }
        }
        let mut actual = Vec::new();
        decode(&[0x00, 0x01, 0x00, 0x20], 4096, 1, 13, true, |row| actual.extend_from_slice(row), |_| {}).unwrap();
        assert_eq!(actual, vec![0; 512]);
    }

    #[test]
    fn packed_runs_and_repeat_rows_preserve_output() {
        let mut output = Vec::new();
        decode(&[0xf4], 4, 2, 13, true, |row| output.extend_from_slice(row), |_| {}).unwrap();
        assert_eq!(output, [15, 15]);
        assert!(decode(&[0xf4], 4, 1, 13, true, |_| {}, |_| {}).is_err());
        assert!(decode(&[0], 4, 1, 13, true, |_| {}, |_| {}).is_err());
        assert!(decode(&[0; 32], 4, 1, 0, true, |_| {}, |_| {}).is_err());
    }
    #[test]
    fn bitmap_checks_input_length_and_preserves_polarity() {
        let mut output = Vec::new();
        decode(&[0xa5, 0xff], 8, 2, 14, false, |row| output.extend_from_slice(row), |_| {}).unwrap();
        assert_eq!(output, [0x5a, 0]);
        assert!(decode(&[], 8, 1, 14, false, |_| {}, |_| {}).is_err());
        assert!(decode(&[], u32::MAX, u32::MAX, 14, false, |_| {}, |_| {}).is_err());
    }

    #[test]
    fn packet_header_lengths_cannot_underflow_and_fields_sign_extend() {
        let bytes = [9, 65, 0, 0, 1, 2, 8, 1, 255, 254];
        let parsed = header(&mut bytes.as_slice(), 0).unwrap();
        assert_eq!(
            (parsed.packet_length, parsed.code, parsed.horizontal_offset, parsed.vertical_offset),
            (1, 65, -1, -2)
        );
        for end in 0..bytes.len() {
            assert!(header(&mut &bytes[..end], 0).is_err());
        }
        let mut bad = bytes;
        bad[0] = 7;
        assert!(header(&mut bad.as_slice(), 0).is_err());
        assert!(header(&mut &[255; 36][..], 7).is_err());
    }
}
