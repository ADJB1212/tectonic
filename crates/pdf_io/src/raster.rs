//! Checked in-place alpha separation for decoded PNG data.
#![forbid(unsafe_code)]

use crate::{error::PdfError, syntax::error};

pub(crate) fn split_alpha(
    bytes: &mut [u8],
    width: usize,
    height: usize,
    colors: usize,
    sample: usize,
) -> Result<(Vec<u8>, usize), PdfError> {
    if !matches!(colors, 1 | 3) || !matches!(sample, 1 | 2) {
        return Err(error("Invalid PNG alpha layout.", 0));
    }
    let pixel_bytes = (colors + 1) * sample;
    let pixels = width
        .checked_mul(height)
        .ok_or_else(|| error("PNG raster is too large.", 0))?;
    let length = pixels
        .checked_mul(pixel_bytes)
        .ok_or_else(|| error("PNG raster is too large.", 0))?;
    if bytes.len() != length {
        return Err(error("PNG: Inconsistent rowbytes value.", 0));
    }
    let mut mask = vec![0; pixels * sample];
    let color_bytes = colors * sample;
    for pixel in 0..pixels {
        let start = pixel * pixel_bytes;
        mask[pixel * sample..(pixel + 1) * sample]
            .copy_from_slice(&bytes[start + color_bytes..start + pixel_bytes]);
        bytes.copy_within(start..start + color_bytes, pixel * color_bytes);
    }
    Ok((mask, width * color_bytes))
}

pub(crate) fn palette_alpha(
    bytes: &[u8],
    width: usize,
    height: usize,
    bits: u8,
    transparency: &[u8],
) -> Result<Vec<u8>, PdfError> {
    if width == 0 || height == 0 {
        return Err(error("Invalid PNG dimensions.", 0));
    }
    if !matches!(bits, 1 | 2 | 4 | 8) {
        return Err(error("Invalid PNG palette depth.", 0));
    }
    let pixels = width
        .checked_mul(height)
        .filter(|&n| n <= i32::MAX as usize)
        .ok_or_else(|| error("PNG raster is too large.", 0))?;
    let row_bytes = width
        .checked_mul(bits.into())
        .and_then(|n| n.checked_add(7))
        .map(|n| n / 8)
        .ok_or_else(|| error("PNG raster is too large.", 0))?;
    let len = row_bytes
        .checked_mul(height)
        .ok_or_else(|| error("PNG raster is too large.", 0))?;
    if bytes.len() != len {
        return Err(error("PNG: Inconsistent rowbytes value.", 0));
    }
    let mut mask = Vec::with_capacity(pixels);
    for row in bytes.chunks_exact(row_bytes) {
        for x in 0..width {
            let bit = x * usize::from(bits);
            let index =
                (row[bit / 8] >> (8 - usize::from(bits) - bit % 8)) & ((1u16 << bits) - 1) as u8;
            mask.push(transparency.get(index as usize).copied().unwrap_or(255));
        }
    }
    Ok(mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_separation_preserves_color_and_sixteen_bit_samples() {
        for colors in [1, 3] {
            for sample in [1, 2] {
                let original: Vec<_> = (0..4 * (colors + 1) * sample).map(|i| i as u8).collect();
                let mut bytes = original.clone();
                let (alpha, row) = split_alpha(&mut bytes, 2, 2, colors, sample).unwrap();
                let expected: Vec<_> = original
                    .chunks_exact((colors + 1) * sample)
                    .flat_map(|p| p[..colors * sample].iter().copied())
                    .collect();
                let expected_alpha: Vec<_> = original
                    .chunks_exact((colors + 1) * sample)
                    .flat_map(|p| p[colors * sample..].iter().copied())
                    .collect();
                assert_eq!(&bytes[..expected.len()], expected);
                assert_eq!(alpha, expected_alpha);
                assert_eq!(row, 2 * colors * sample);
            }
        }
        assert!(split_alpha(&mut [], usize::MAX, 2, 3, 2).is_err());
        assert!(split_alpha(&mut [0; 3], 1, 1, 3, 1).is_err());
    }

    #[test]
    fn palette_rows_have_independent_padding_and_default_opaque_entries() {
        assert_eq!(
            palette_alpha(&[0x40, 0xa0], 3, 2, 1, &[0]).unwrap(),
            [0, 255, 0, 255, 0, 255]
        );
        assert!(palette_alpha(&[0], 2, 2, 8, &[]).is_err());
    }
}
