//! Bounded JPEG 2000 box traversal. Codestream bytes remain opaque.
#![forbid(unsafe_code)]

use crate::{error::PdfError, syntax::error};
use std::io::{Read, Seek, SeekFrom};

fn io(e: std::io::Error) -> PdfError {
    PdfError::io(c"JPEG2000: Reading JPEG 2000 file failed.".into(), e)
}
fn position(input: &mut impl Seek) -> Result<u64, PdfError> {
    input.stream_position().map_err(io)
}
fn seek(input: &mut impl Seek, pos: u64) -> Result<(), PdfError> {
    input.seek(SeekFrom::Start(pos)).map(|_| ()).map_err(io)
}
fn read<const N: usize>(input: &mut impl Read) -> Result<[u8; N], PdfError> {
    let mut bytes = [0; N];
    input.read_exact(&mut bytes).map_err(io)?;
    Ok(bytes)
}

struct BoxHeader {
    kind: [u8; 4],
    start: u64,
    end: u64,
}

fn box_header(
    input: &mut (impl Read + Seek),
    end: u64,
    to_end: bool,
) -> Result<BoxHeader, PdfError> {
    let start = position(input)?;
    if end.saturating_sub(start) < 8 {
        return Err(error("JPEG2000: Truncated box header.", start as usize));
    }
    let length = u32::from_be_bytes(read(input)?);
    let kind = read(input)?;
    let (length, header) = match length {
        0 if to_end => (end - start, 8),
        1 => {
            if end - start < 16 {
                return Err(error("JPEG2000: Truncated box header.", start as usize));
            }
            let length = u64::from_be_bytes(read(input)?);
            if length > u32::MAX.into() {
                return Err(error(
                    "JPEG2000: LBox value in JP2 file >32 bits.\nI can't handle this!",
                    start as usize,
                ));
            }
            (length, 16)
        }
        length => (u64::from(length), 8),
    };
    if length < header || length > end - start {
        return Err(error("JPEG2000: Invalid box length.", start as usize));
    }
    Ok(BoxHeader {
        kind,
        start: start + header,
        end: start + length,
    })
}

pub(crate) fn probe(input: &mut (impl Read + Seek)) -> Result<bool, PdfError> {
    seek(input, 0)?;
    let end = input.seek(SeekFrom::End(0)).map_err(io)?;
    seek(input, 0)?;
    if end < 12 || read::<12>(input)? != *b"\0\0\0\x0cjP  \r\n\x87\n" {
        return Ok(false);
    }
    let header = box_header(input, end, false)?;
    if header.kind != *b"ftyp"
        || header.end - header.start < 8
        || (header.end - header.start) % 4 != 0
    {
        return Ok(false);
    }
    let brand = read::<4>(input)?;
    read::<4>(input)?;
    let mut supported = brand == *b"jp2 ";
    while position(input)? < header.end {
        let compatible = read::<4>(input)?;
        if brand == *b"jpx " && compatible == *b"jpxb" {
            supported = true;
        }
    }
    Ok(supported)
}

pub(crate) struct Metadata {
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) components: i32,
    pub(crate) x_density: f64,
    pub(crate) y_density: f64,
    pub(crate) soft_mask: bool,
}

pub(crate) fn scan(
    input: &mut (impl Read + Seek),
    mut warning: impl FnMut(&'static str),
) -> Result<Metadata, PdfError> {
    if !probe(input)? {
        return Err(error("JPEG2000: Reading JPEG 2000 file failed.", 0));
    }
    let after_type = position(input)?;
    let end = input.seek(SeekFrom::End(0)).map_err(io)?;
    seek(input, after_type)?;
    let mut info = Metadata {
        width: 0,
        height: 0,
        components: 0,
        x_density: 1.0,
        y_density: 1.0,
        soft_mask: false,
    };
    let mut have_header = false;
    while position(input)? < end {
        let header = box_header(input, end, true)?;
        match &header.kind {
            b"jp2h" => {
                parse_header(input, header.end, &mut info, &mut warning)?;
                have_header = true;
            }
            b"jp2c" if !have_header => {
                warning("JPEG2000: JPEG 2000 Codestream box found before JP2 Header box.")
            }
            _ => {}
        }
        seek(input, header.end)?;
    }
    if !have_header {
        return Err(error(
            "JPEG2000: No JP2 Header box found. Not a JP2/JPX baseline file?",
            end as usize,
        ));
    }
    Ok(info)
}

fn parse_header(
    input: &mut (impl Read + Seek),
    end: u64,
    info: &mut Metadata,
    warning: &mut impl FnMut(&'static str),
) -> Result<(), PdfError> {
    let mut have_image = false;
    while position(input)? < end {
        let header = box_header(input, end, false)?;
        match &header.kind {
            b"ihdr" => {
                if header.end - header.start < 14 {
                    return Err(error(
                        "JPEG2000: Truncated image header.",
                        header.start as usize,
                    ));
                }
                let height = u32::from_be_bytes(read(input)?);
                let width = u32::from_be_bytes(read(input)?);
                let components = u16::from_be_bytes(read(input)?);
                read::<4>(input)?;
                if width == 0
                    || height == 0
                    || width > i32::MAX as u32
                    || height > i32::MAX as u32
                    || components == 0
                {
                    return Err(error(
                        "JPEG2000: Invalid image dimensions.",
                        header.start as usize,
                    ));
                }
                info.width = width as i32;
                info.height = height as i32;
                info.components = components.into();
                have_image = true;
            }
            b"res " => resolution(input, header.end, info, warning)?,
            b"cdef" => {
                if header.end - header.start < 2 {
                    return Err(error(
                        "JPEG2000: Inconsistent N value in Channel Definition box.",
                        header.start as usize,
                    ));
                }
                let count = u16::from_be_bytes(read(input)?);
                if header.end - header.start < 2 + u64::from(count) * 6 {
                    return Err(error(
                        "JPEG2000: Inconsistent N value in Channel Definition box.",
                        header.start as usize,
                    ));
                }
                let (mut opacity, mut type0) = (0, false);
                for _ in 0..count {
                    let channel = u16::from_be_bytes(read(input)?);
                    let kind = u16::from_be_bytes(read(input)?);
                    let association = u16::from_be_bytes(read(input)?);
                    if channel > count {
                        warning("JPEG2000: Invalid Cn value in Channel Definition box.");
                    }
                    if matches!(kind, 1 | 2) {
                        opacity += 1;
                    }
                    if kind == 1 && association == 0 {
                        type0 = true;
                    }
                }
                info.soft_mask = opacity == 1 && type0;
                if opacity > 1 {
                    warning("JPEG2000: Unsupported transparency type. (ignored)");
                }
            }
            b"bpcc" | b"colr" | b"pclr" | b"cmap" | b"lbl " => {}
            _ => {
                return Err(error(
                    "JPEG2000: Unknown JPEG 2000 box in JP2 Header box.",
                    header.start as usize,
                ));
            }
        }
        seek(input, header.end)?;
    }
    if !have_image {
        return Err(error(
            "JPEG2000: Expecting JPEG 2000 Image Header box but could not find.",
            end as usize,
        ));
    }
    Ok(())
}

fn resolution(
    input: &mut (impl Read + Seek),
    end: u64,
    info: &mut Metadata,
    warning: &mut impl FnMut(&'static str),
) -> Result<(), PdfError> {
    let mut display = false;
    while position(input)? < end {
        let header = box_header(input, end, false)?;
        if header.kind == *b"resd" || (header.kind == *b"resc" && !display) {
            if header.end - header.start < 10 {
                return Err(error(
                    "JPEG2000: Truncated resolution box.",
                    header.start as usize,
                ));
            }
            let vn = u16::from_be_bytes(read(input)?);
            let vd = u16::from_be_bytes(read(input)?);
            let hn = u16::from_be_bytes(read(input)?);
            let hd = u16::from_be_bytes(read(input)?);
            let [ve, he] = read(input)?;
            if [vn, vd, hn, hd].contains(&0) {
                return Err(error(
                    "JPEG2000: Invalid resolution.",
                    header.start as usize,
                ));
            }
            info.x_density =
                72.0 / (f64::from(hn) / f64::from(hd) * 10.0f64.powf(f64::from(he)) * 0.0254);
            info.y_density =
                72.0 / (f64::from(vn) / f64::from(vd) * 10.0f64.powf(f64::from(ve)) * 0.0254);
            display |= header.kind == *b"resd";
        } else if header.kind != *b"resc" {
            warning("JPEG2000: Unknown JPEG 2000 box type in Resolution box.");
        }
        seek(input, header.end)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn boxed(kind: [u8; 4], bytes: &[u8]) -> Vec<u8> {
        [
            (bytes.len() as u32 + 8).to_be_bytes().as_slice(),
            &kind,
            bytes,
        ]
        .concat()
    }
    fn fixture() -> Vec<u8> {
        let image = [
            2u32.to_be_bytes().as_slice(),
            3u32.to_be_bytes().as_slice(),
            &[0, 3, 7, 7, 0, 0],
        ]
        .concat();
        [
            b"\0\0\0\x0cjP  \r\n\x87\n".as_slice(),
            &boxed(*b"ftyp", b"jp2 \0\0\0\0"),
            &boxed(*b"jp2h", &boxed(*b"ihdr", &image)),
            &boxed(*b"jp2c", &[1, 2, 3]),
        ]
        .concat()
    }

    #[test]
    fn metadata_and_unknown_codestream_bytes_are_preserved() {
        let info = scan(&mut Cursor::new(fixture()), |_| {}).unwrap();
        assert_eq!((info.width, info.height, info.components), (3, 2, 3));
        assert_eq!((info.x_density, info.y_density), (1.0, 1.0));
    }

    #[test]
    fn box_sizes_and_nested_truncations_are_rejected() {
        let bytes = fixture();
        for end in 0..bytes.len() {
            if end == bytes.len() - 11 {
                continue;
            }
            assert!(scan(&mut Cursor::new(&bytes[..end]), |_| {}).is_err());
        }
        for length in [0u32, 1, 7, u32::MAX] {
            let mut bytes = fixture();
            bytes[12..16].copy_from_slice(&length.to_be_bytes());
            assert!(scan(&mut Cursor::new(bytes), |_| {}).is_err());
        }
    }
}
