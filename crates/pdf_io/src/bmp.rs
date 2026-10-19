//! Checked BMP headers and owned raster data.
#![forbid(unsafe_code)]

use crate::{error::PdfError, syntax::error};
use std::io::{Read, Seek, SeekFrom};

pub(crate) struct Header {
    pub(crate) offset: u32,
    pub(crate) size: u32,
    pub(crate) width: u32,
    pub(crate) height: i32,
    pub(crate) bits: u16,
    pub(crate) compression: u32,
    pub(crate) palette_entry: usize,
    pub(crate) x_density: u32,
    pub(crate) y_density: u32,
}

fn read(input: &mut impl Read, bytes: &mut [u8], message: &'static str) -> Result<(), PdfError> {
    input
        .read_exact(bytes)
        .map_err(|e| PdfError::io(std::ffi::CString::new(message).unwrap(), e))
}

impl Header {
    pub(crate) fn read(input: &mut impl Read) -> Result<Self, PdfError> {
        let mut bytes = [0; 138];
        read(input, &mut bytes[..18], "Could not read BMP file header...")?;
        if &bytes[..2] != b"BM" {
            return Err(error(
                "File not starting with 'B' 'M'... Not a BMP file?",
                0,
            ));
        }
        if bytes[6..10] != [0; 4] {
            return Err(error("Not a BMP file???", 6));
        }
        let size = u32::from_le_bytes(bytes[14..18].try_into().unwrap());
        if !matches!(size, 12 | 40 | 64 | 108 | 124) {
            return Err(error("Unknown BMP header type.", 14));
        }
        read(
            input,
            &mut bytes[18..14 + size as usize],
            "Could not read BMP file header...",
        )?;
        Self::parse(&bytes[..14 + size as usize])
    }

    fn parse(bytes: &[u8]) -> Result<Self, PdfError> {
        let word = |pos| u16::from_le_bytes(bytes[pos..pos + 2].try_into().unwrap());
        let quad = |pos| u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap());
        let size = quad(14);
        let core = size == 12;
        let (width, height, planes, bits) = if core {
            (u32::from(word(18)), i32::from(word(20)), word(22), word(24))
        } else {
            (quad(18), quad(22) as i32, word(26), word(28))
        };
        if planes != 1 {
            return Err(error("Unknown bcPlanes value in BMP COREHEADER.", 22));
        }
        if width == 0 || width > i32::MAX as u32 || height == 0 || height == i32::MIN {
            return Err(error("Invalid BMP dimensions.", 18));
        }
        let offset = quad(10);
        if offset < 14 + size {
            return Err(error("Invalid BMP raster offset.", 10));
        }
        Ok(Self {
            offset,
            size,
            width,
            height,
            bits,
            compression: if core { 0 } else { quad(30) },
            palette_entry: if core { 3 } else { 4 },
            x_density: if core { 0 } else { quad(38) },
            y_density: if core { 0 } else { quad(42) },
        })
    }
}

pub(crate) struct Raster {
    pub(crate) header: Header,
    pub(crate) palette: Vec<u8>,
    pub(crate) bytes: Vec<u8>,
    pub(crate) row_bytes: usize,
}

pub(crate) fn read_raster(input: &mut (impl Read + Seek)) -> Result<Raster, PdfError> {
    let header = Header::read(input)?;
    if !matches!(header.bits, 1 | 4 | 8 | 24) {
        return Err(error("Unknown/Unsupported BMP bitCount value.", 28));
    }
    let width = header.width as usize;
    let height = header.height.unsigned_abs() as usize;
    let row_bytes = width
        .checked_mul(header.bits as usize)
        .and_then(|v| v.checked_add(7))
        .map(|v| v / 8)
        .ok_or_else(|| error("BMP raster is too large.", 18))?;
    let len = row_bytes
        .checked_mul(height)
        .filter(|&len| len <= i32::MAX as usize)
        .ok_or_else(|| error("BMP raster is too large.", 18))?;
    let mut palette = Vec::new();
    if header.bits < 24 {
        let count = (header.offset - header.size - 14) as usize / header.palette_entry;
        if count == 0 || count > (1usize << header.bits) {
            return Err(error("Invalid BMP palette size.", 10));
        }
        palette.reserve(count * 3);
        for _ in 0..count {
            let mut entry = [0; 4];
            read(
                input,
                &mut entry[..header.palette_entry],
                "Reading file failed...",
            )?;
            palette.extend_from_slice(&[entry[2], entry[1], entry[0]]);
        }
    }
    input
        .seek(SeekFrom::Start(header.offset.into()))
        .map_err(|e| PdfError::io(c"Invalid BMP raster offset.".into(), e))?;
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(len)
        .map_err(|_| error("BMP raster is too large.", 18))?;
    bytes.resize(len, 0);
    match (header.compression, header.bits) {
        (0, _) => {
            let padding = (4 - row_bytes % 4) % 4;
            for row in bytes.chunks_exact_mut(row_bytes) {
                read(input, row, "Reading BMP raster data failed...")?;
                read(
                    input,
                    &mut [0; 3][..padding],
                    "Reading BMP raster data failed...",
                )?;
            }
        }
        (1, 8) | (2, 4) => decode_rle(input, &mut bytes, width, height, header.bits)?,
        _ => {
            return Err(error(
                "Unknown/Unsupported compression type for BMP image.",
                30,
            ));
        }
    }
    if header.bits == 24 {
        for pixel in bytes.as_chunks_mut::<3>().0 {
            pixel.swap(0, 2);
        }
    }
    Ok(Raster {
        header,
        palette,
        bytes,
        row_bytes,
    })
}

fn decode_rle(
    input: &mut impl Read,
    output: &mut [u8],
    width: usize,
    height: usize,
    bits: u16,
) -> Result<(), PdfError> {
    fn byte(input: &mut impl Read) -> Result<u8, PdfError> {
        let mut value = [0];
        read(input, &mut value, "Reading BMP raster data failed...")?;
        Ok(value[0])
    }
    let row_bytes = (width * bits as usize).div_ceil(8);
    let (mut x, mut y) = (0, 0);
    loop {
        let count = byte(input)?;
        let value = byte(input)?;
        if count == 0 {
            match value {
                0 => {
                    x = 0;
                    y += 1;
                    if y >= height {
                        return Ok(());
                    }
                    continue;
                }
                1 => return Ok(()),
                2 => {
                    x += byte(input)? as usize;
                    y += byte(input)? as usize;
                    if x > width || y >= height {
                        return Err(error("RLE decode failed...", 0));
                    }
                    continue;
                }
                _ => {}
            }
        }
        let pixels = if count == 0 {
            value as usize
        } else {
            count as usize
        };
        if y >= height || x + pixels > width {
            return Err(error("RLE decode failed...", 0));
        }
        let mut packed = value;
        for i in 0..pixels {
            if count == 0 && (bits == 8 || i % 2 == 0) {
                packed = byte(input)?;
            }
            let pixel = if bits == 8 {
                packed
            } else if i % 2 == 0 {
                packed >> 4
            } else {
                packed & 15
            };
            let pos = y * row_bytes + if bits == 8 { x } else { x / 2 };
            if bits == 8 {
                output[pos] = pixel;
            } else if x % 2 == 0 {
                output[pos] = (output[pos] & 0x0f) | (pixel << 4);
            } else {
                output[pos] = (output[pos] & 0xf0) | pixel;
            }
            x += 1;
        }
        if count == 0 && !(pixels * bits as usize).div_ceil(8).is_multiple_of(2) {
            byte(input)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn bmp(width: i32, height: i32) -> Vec<u8> {
        let mut bytes = vec![0; 54];
        bytes[..2].copy_from_slice(b"BM");
        bytes[10..14].copy_from_slice(&54u32.to_le_bytes());
        bytes[14..18].copy_from_slice(&40u32.to_le_bytes());
        bytes[18..22].copy_from_slice(&width.to_le_bytes());
        bytes[22..26].copy_from_slice(&height.to_le_bytes());
        bytes[26..28].copy_from_slice(&1u16.to_le_bytes());
        bytes[28..30].copy_from_slice(&24u16.to_le_bytes());
        bytes
    }

    #[test]
    fn rejects_header_sizes_before_reading_and_checks_dimensions() {
        for size in [0u32, 3, 125, u32::MAX] {
            let mut bytes = bmp(1, 1);
            bytes[14..18].copy_from_slice(&size.to_le_bytes());
            assert!(Header::read(&mut bytes.as_slice()).is_err());
        }
        for (w, h) in [(0, 1), (-1, 1), (1, 0), (1, i32::MIN)] {
            assert!(Header::read(&mut bmp(w, h).as_slice()).is_err());
        }
        let bytes = bmp(1, 1);
        for end in 0..bytes.len() {
            assert!(Header::read(&mut &bytes[..end]).is_err());
        }
    }

    #[test]
    fn raster_preserves_padding_orientation_and_rgb() {
        let mut bytes = bmp(1, -2);
        bytes.extend_from_slice(&[3, 2, 1, 0, 6, 5, 4, 0]);
        let raster = read_raster(&mut Cursor::new(bytes)).unwrap();
        assert_eq!(raster.header.height, -2);
        assert_eq!(raster.bytes, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn rle_checks_delta_and_preserves_odd_pixel_nibbles() {
        let mut output = [0; 2];
        decode_rle(&mut &[1, 0x10, 2, 0x23, 0, 1][..], &mut output, 3, 1, 4).unwrap();
        assert_eq!(output, [0x12, 0x30]);
        assert!(decode_rle(&mut &[0, 2, 0, 1][..], &mut output, 3, 1, 4).is_err());
        assert!(decode_rle(&mut &[4, 1][..], &mut output, 3, 1, 4).is_err());
        assert!(decode_rle(&mut &[0, 3, 0x12][..], &mut output, 3, 1, 4).is_err());
    }
}
