//! Checked JPEG metadata with owned application segments.
#![forbid(unsafe_code)]

use crate::{error::PdfError, syntax::error};
use std::io::{Read, Seek, SeekFrom};

pub(crate) struct IccChunk {
    pub(crate) id: u8,
    pub(crate) count: u8,
    pub(crate) bytes: Vec<u8>,
}
pub(crate) struct Info {
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) bits_per_component: u8,
    pub(crate) num_components: u8,
    pub(crate) xdpi: f64,
    pub(crate) ydpi: f64,
    pub(crate) adobe: bool,
    pub(crate) icc: Vec<IccChunk>,
    pub(crate) xmp: Vec<Vec<u8>>,
    pub(crate) prefix: Vec<u8>,
    pub(crate) rest: u64,
}

pub(crate) enum Warning {
    Message(&'static str),
    Resolution { exif: (f64, f64), jfif: (f64, f64) },
}

fn io(e: std::io::Error) -> PdfError {
    PdfError::io(c"JPEG: Not a JPEG file?".into(), e)
}
fn read<const N: usize>(input: &mut impl Read) -> Result<[u8; N], PdfError> {
    let mut bytes = [0; N];
    input.read_exact(&mut bytes).map_err(io)?;
    Ok(bytes)
}
fn sof(marker: u8) -> bool {
    matches!(marker, 192..=195 | 197..=199 | 201..=203 | 205..=207)
}

pub(crate) fn scan(input: &mut (impl Read + Seek), mut warning: impl FnMut(Warning)) -> Result<Info, PdfError> {
    input.seek(SeekFrom::Start(0)).map_err(io)?;
    let mut info = Info {
        width: 0,
        height: 0,
        bits_per_component: 0,
        num_components: 0,
        xdpi: 0.0,
        ydpi: 0.0,
        adobe: false,
        icc: Vec::new(),
        xmp: Vec::new(),
        prefix: Vec::new(),
        rest: 0,
    };
    let mut count = 0;
    let mut passthrough = None;
    let mut bytes = Vec::new();
    loop {
        if count == 1024 {
            passthrough = Some(input.stream_position().map_err(io)?);
        }
        if read::<1>(input)? != [255] {
            return Err(error("JPEG: Not a JPEG file?", count));
        }
        let marker = loop {
            let byte = read::<1>(input)?[0];
            if byte != 0 && byte != 255 {
                break byte;
            }
        };
        if count == 0 && marker != 216 {
            return Err(error("JPEG: Not a JPEG file?", 0));
        }
        if marker == 216 || matches!(marker, 208..=215) {
            if count < 1024 {
                info.prefix.extend_from_slice(&[255, marker]);
            }
            count += 1;
            continue;
        }
        let length = u16::from_be_bytes(read(input)?);
        if length < 2 {
            return Err(error("JPEG: Invalid segment length.", count));
        }
        bytes.resize(length as usize - 2, 0);
        input.read_exact(&mut bytes).map_err(io)?;
        let mut skip = false;
        match marker {
            marker if sof(marker) => {
                if bytes.len() < 6 {
                    return Err(error("JPEG: Truncated frame header.", count));
                }
                info.bits_per_component = bytes[0];
                info.height = u16::from_be_bytes([bytes[1], bytes[2]]);
                info.width = u16::from_be_bytes([bytes[3], bytes[4]]);
                info.num_components = bytes[5];
                if bytes.len() < 6 + usize::from(info.num_components) * 3 || info.width == 0 || info.height == 0 || info.num_components == 0 {
                    return Err(error("JPEG: Invalid frame header.", count));
                }
            }
            224 if bytes.starts_with(b"JFIF\0") => {
                if bytes.len() < 14 {
                    return Err(error("JPEG: Truncated JFIF header.", count));
                }
                let thumbnail = 3 * usize::from(bytes[12]) * usize::from(bytes[13]);
                if bytes.len() - 14 < thumbnail {
                    return Err(error("JPEG: Truncated JFIF thumbnail.", count));
                }
                let x = u16::from_be_bytes([bytes[8], bytes[9]]);
                let y = u16::from_be_bytes([bytes[10], bytes[11]]);
                let density = match bytes[7] {
                    1 => (f64::from(x), f64::from(y)),
                    2 => (f64::from(x) * 2.54, f64::from(y) * 2.54),
                    _ => (72.0, 72.0),
                };
                (info.xdpi, info.ydpi) = density;
            }
            225 if bytes.starts_with(b"Exif\0") => {
                if let Some(density) = exif_density(&bytes[5..], &mut warning) {
                    if info.xdpi < 0.1 && info.ydpi < 0.1 {
                        (info.xdpi, info.ydpi) = density;
                    } else if (density.0 + 0.5).floor() != (info.xdpi + 0.5).floor() || (density.1 + 0.5).floor() != (info.ydpi + 0.5).floor() {
                        warning(Warning::Resolution {
                            exif: density,
                            jfif: (info.xdpi, info.ydpi),
                        });
                    }
                }
            }
            225 if bytes.len() > 29 && bytes.starts_with(b"http://ns.adobe.com/xap/1.0/\0") => {
                info.xmp.push(bytes[29..].to_vec());
                skip = true;
            }
            225 => skip = bytes.len() <= 5 || !bytes.starts_with(b"http:") || bytes.len() > 29,
            226 => {
                if bytes.starts_with(b"ICC_PROFILE\0") {
                    if bytes.len() < 14 {
                        return Err(error("JPEG: Truncated ICC chunk.", count));
                    }
                    info.icc.push(IccChunk {
                        id: bytes[12],
                        count: bytes[13],
                        bytes: bytes[14..].to_vec(),
                    });
                }
                skip = true;
            }
            238 if bytes.starts_with(b"Adobe") => {
                if bytes.len() < 12 {
                    return Err(error("JPEG: Truncated Adobe header.", count));
                }
                info.adobe = true;
            }
            224 if bytes.len() <= 5 => {}
            224..=239 => skip = true,
            _ => {}
        }
        if !skip && count < 1024 {
            info.prefix.extend_from_slice(&[255, marker]);
            info.prefix.extend_from_slice(&length.to_be_bytes());
            info.prefix.extend_from_slice(&bytes);
        }
        if sof(marker) {
            info.rest = if let Some(position) = passthrough {
                position
            } else {
                input.stream_position().map_err(io)?
            };
            if info.xdpi < 0.1 && info.ydpi < 0.1 {
                info.xdpi = 72.0;
                info.ydpi = 72.0;
            }
            return Ok(info);
        }
        count += 1;
    }
}

fn exif_density(bytes: &[u8], warning: &mut impl FnMut(Warning)) -> Option<(f64, f64)> {
    let start = bytes.iter().position(|&b| b != 0)?;
    let bytes = &bytes[start..];
    if bytes.len() <= 8 {
        return None;
    }
    let little = match &bytes[..2] {
        b"II" => true,
        b"MM" => false,
        _ => {
            warning(Warning::Message("JPEG: Invalid value in Exif TIFF header."));
            return None;
        }
    };
    let value = |pos: usize, width: usize| -> Option<u32> {
        let data = bytes.get(pos..pos.checked_add(width)?)?;
        Some(if little {
            data.iter().rev().fold(0, |v, &b| v * 256 + u32::from(b))
        } else {
            data.iter().fold(0, |v, &b| v * 256 + u32::from(b))
        })
    };
    if value(2, 2)? != 42 {
        warning(Warning::Message("JPEG: Invalid value in Exif TIFF header."));
        return None;
    }
    let mut pos = value(4, 4)? as usize;
    let count = value(pos, 2)?;
    pos += 2;
    let (mut x, mut y, mut unit) = (0.0, 0.0, 1.0);
    let (mut ms_x, mut ms_y, mut ms_unit) = (0, 0, 0.0);
    for _ in 0..count {
        let tag = value(pos, 2)?;
        let kind = value(pos + 2, 2)?;
        let count = value(pos + 4, 4)?;
        let field = pos + 8;
        let data = match kind {
            1 | 7 => value(field, 1)?,
            3 => value(field, 2)?,
            4 | 5 | 9 | 10 => value(field, 4)?,
            _ => 0,
        };
        bytes.get(pos..pos.checked_add(12)?)?;
        pos += 12;
        let rational = if matches!(kind, 5 | 10) {
            let num = value(data as usize, 4)? as i32;
            let den = value((data as usize).checked_add(4)?, 4)? as i32;
            if den == 0 { None } else { num.checked_div(den).map(f64::from) }
        } else {
            None
        };
        match tag {
            282 => {
                if let Some(v) = rational {
                    x = v;
                }
            }
            283 => {
                if let Some(v) = rational {
                    y = v;
                }
            }
            296 => {
                if data == 2 {
                    unit = 1.0;
                } else if data == 3 {
                    unit = 2.54;
                }
            }
            20752 => {
                if kind != 1 || count != 1 {
                    warning(Warning::Message("JPEG: Invalid data for ResolutionUnit in Exif chunk."));
                    return None;
                }
                // Preserve the legacy Microsoft extension's additional value read.
                ms_unit = if value(pos, 1)? == 1 { 0.0254 } else { 0.0 };
                pos += 4;
            }
            20753 | 20754 => {
                if kind != 4 || count != 1 {
                    warning(Warning::Message(if tag == 20753 {
                        "JPEG: Invalid data for PixelPerUnitX in Exif chunk."
                    } else {
                        "JPEG: Invalid data for PixelPerUnitY in Exif chunk."
                    }));
                    return None;
                }
                let density = value(pos, 4)?;
                pos += 4;
                if tag == 20753 {
                    ms_x = density;
                } else {
                    ms_y = density;
                }
            }
            _ => {}
        }
    }
    if x > 0.0 && y > 0.0 {
        Some((x * unit, y * unit))
    } else if ms_x > 0 && ms_y > 0 && ms_unit > 0.0 {
        Some((f64::from(ms_x) * ms_unit, f64::from(ms_y) * ms_unit))
    } else {
        Some((72.0 * unit, 72.0 * unit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn segment(marker: u8, data: &[u8]) -> Vec<u8> {
        [&[255, marker], (data.len() as u16 + 2).to_be_bytes().as_slice(), data].concat()
    }
    fn fixture() -> Vec<u8> {
        [
            &[255, 216][..],
            &segment(224, &[b'J', b'F', b'I', b'F', 0, 1, 2, 1, 1, 44, 0, 150, 0, 0]),
            &segment(192, &[8, 0, 2, 0, 3, 1, 1, 0x11, 0]),
            &[255, 218, 0, 2, 1, 255, 217],
        ]
        .concat()
    }

    #[test]
    fn density_and_encoded_bytes_are_preserved() {
        let bytes = fixture();
        let info = scan(&mut Cursor::new(&bytes), |_| {}).unwrap();
        assert_eq!((info.width, info.height, info.num_components), (3, 2, 1));
        assert_eq!((info.xdpi, info.ydpi), (300.0, 150.0));
        assert_eq!([info.prefix, bytes[info.rest as usize..].to_vec()].concat(), bytes);
    }

    #[test]
    fn segment_truncation_and_bad_exif_offsets_are_checked() {
        let bytes = fixture();
        for end in 0..bytes.len() - 7 {
            assert!(scan(&mut Cursor::new(&bytes[..end]), |_| {}).is_err());
        }
        for data in [&[0, 0][..], &[0, 1], &[255, 255]] {
            let bytes = [&[255, 216, 255, 224][..], data].concat();
            assert!(scan(&mut Cursor::new(bytes), |_| {}).is_err());
        }
        assert!(exif_density(b"II*\0\xff\xff\xff\xff\0", &mut |_| {}).is_none());
        assert!(exif_density(b"II*\0\x08\0\0\0\x01\0", &mut |_| {}).is_none());
    }
}
