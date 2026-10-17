//! Numeric conversions used by the PDF compatibility adapters.

use std::io::Write;

pub(crate) const POWERS_OF_TEN: [u32; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];
pub(crate) const INVERSE_POWERS_OF_TEN: [f64; 10] = [
    1.0,
    0.1,
    0.01,
    0.001,
    0.0001,
    0.00001,
    0.000001,
    0.0000001,
    0.00000001,
    0.000000001,
];
pub(crate) const DECIMAL_BUFFER_SIZE: usize = 320;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FormatError {
    InvalidPrecision(i32),
    BufferTooSmall { needed: usize, available: usize },
}

fn copy_output(bytes: &[u8], buffer: &mut [u8]) -> Result<usize, FormatError> {
    if buffer.len() < bytes.len() {
        return Err(FormatError::BufferTooSmall {
            needed: bytes.len(),
            available: buffer.len(),
        });
    }
    buffer[..bytes.len()].copy_from_slice(bytes);
    Ok(bytes.len())
}

pub(crate) fn format_integer(value: i32, buffer: &mut [u8]) -> Result<usize, FormatError> {
    let mut bytes = [0; 11];
    let mut start = bytes.len();
    let mut magnitude = value.unsigned_abs();
    loop {
        start -= 1;
        bytes[start] = b'0' + (magnitude % 10) as u8;
        magnitude /= 10;
        if magnitude == 0 {
            break;
        }
    }
    if value < 0 {
        start -= 1;
        bytes[start] = b'-';
    }
    copy_output(&bytes[start..], buffer)
}

pub(crate) fn format_decimal(
    value: f64,
    precision: i32,
    buffer: &mut [u8],
) -> Result<usize, FormatError> {
    let Some(&scale) = usize::try_from(precision)
        .ok()
        .and_then(|p| POWERS_OF_TEN.get(p))
    else {
        return Err(FormatError::InvalidPrecision(precision));
    };
    let mut bytes = [0; DECIMAL_BUFFER_SIZE];
    let mut length = 0;
    let negative = value < 0.0;
    let magnitude = if negative { -value } else { value };
    if negative {
        bytes[length] = b'-';
        length += 1;
    }
    let mut integer = magnitude.trunc();
    let mut fraction = ((magnitude - integer) * f64::from(scale) + 0.5) as u32;
    if fraction == scale {
        fraction = 0;
        integer += 1.0;
    }
    if integer != 0.0 {
        if integer <= f64::from(i32::MAX) {
            length += format_integer(integer as i32, &mut bytes[length..]).unwrap();
        } else if integer.is_nan() {
            let text = if integer.is_sign_negative() {
                &b"-nan"[..]
            } else {
                &b"nan"[..]
            };
            length += copy_output(text, &mut bytes[length..]).unwrap();
        } else {
            let available = bytes.len() - length;
            let mut output = &mut bytes[length..];
            write!(output, "{integer:.0}").unwrap();
            length += available - output.len();
        }
    } else if fraction == 0 {
        bytes[0] = b'0';
        length = 1;
    }
    if fraction != 0 {
        bytes[length] = b'.';
        length += 1;
        let end = length + precision as usize;
        for byte in bytes[length..end].iter_mut().rev() {
            *byte = b'0' + (fraction % 10) as u8;
            fraction /= 10;
        }
        length = end;
        while bytes[length - 1] == b'0' {
            length -= 1;
        }
    }
    copy_output(&bytes[..length], buffer)
}

pub(crate) fn scaled_fixword(value: i32, fixword: i32) -> i32 {
    let product = i64::from(value) * i64::from(fixword);
    let rounded = ((product.unsigned_abs() + (1 << 19)) >> 20) as u32;
    if product < 0 {
        rounded.wrapping_neg() as i32
    } else {
        rounded as i32
    }
}

pub(crate) fn write_big_endian(value: i32, buffer: &mut [u8]) {
    let mut remaining = value;
    for byte in buffer.iter_mut().rev() {
        *byte = remaining as u8;
        remaining >>= 8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_output_preserves_rounding_precision_and_zero() {
        for (value, precision, expected) in [
            (0.0, 3, "0"),
            (-0.0, 3, "0"),
            (-0.0004, 3, "0"),
            (0.125, 2, ".13"),
            (-0.125, 2, "-.13"),
            (1.125, 2, "1.13"),
            (9.9999, 3, "10"),
            (-9.9999, 3, "-10"),
            (1.5, 0, "2"),
            (-1.5, 0, "-2"),
            (0.5, 0, "1"),
            (-0.5, 0, "-1"),
            (1.25, 8, "1.25"),
            (0.000000001, 9, ".000000001"),
            (2147483647.0, 3, "2147483647"),
            (2147483648.5, 1, "2147483648.5"),
            (1e30, 8, "1000000000000000019884624838656"),
            (f64::INFINITY, 8, "inf"),
            (f64::NEG_INFINITY, 8, "-inf"),
            (f64::NAN, 8, "nan"),
        ] {
            let mut bytes = [0xaa; DECIMAL_BUFFER_SIZE];
            let length = format_decimal(value, precision, &mut bytes).unwrap();
            assert_eq!(
                &bytes[..length],
                expected.as_bytes(),
                "{value}, precision {precision}"
            );
            assert_eq!(bytes[length], 0xaa);
            let mut short = [0xaa; DECIMAL_BUFFER_SIZE];
            assert_eq!(
                format_decimal(value, precision, &mut short[..length - 1]),
                Err(FormatError::BufferTooSmall {
                    needed: length,
                    available: length - 1
                })
            );
            assert!(short.iter().all(|&byte| byte == 0xaa));
        }
        for precision in [-1, 10, i32::MAX] {
            assert_eq!(
                format_decimal(1.0, precision, &mut []),
                Err(FormatError::InvalidPrecision(precision))
            );
        }
        assert_eq!(
            format_decimal(f64::MAX, 9, &mut [0; DECIMAL_BUFFER_SIZE]),
            Ok(309)
        );
    }

    #[test]
    fn integers_cover_signed_boundaries_and_short_buffers() {
        for value in [i32::MIN, -1, 0, 1, i32::MAX] {
            let mut bytes = [0xaa; 11];
            let expected = value.to_string();
            let length = format_integer(value, &mut bytes).unwrap();
            assert_eq!(&bytes[..length], expected.as_bytes());
            for available in 0..length {
                let mut bytes = [0xaa; 11];
                assert!(format_integer(value, &mut bytes[..available]).is_err());
                assert_eq!(bytes, [0xaa; 11]);
            }
        }
    }

    #[test]
    fn fixed_point_rounds_magnitudes_and_wraps_results() {
        for (value, fixword, expected) in [
            (1, 1 << 19, 1),
            (-1, 1 << 19, -1),
            (1, (1 << 19) - 1, 0),
            (12345, 1 << 20, 12345),
            (12345, -(1 << 20), -12345),
            (i32::MIN, 1 << 20, i32::MIN),
            (i32::MIN, i32::MIN, 0),
        ] {
            assert_eq!(scaled_fixword(value, fixword), expected);
        }
    }

    #[test]
    fn big_endian_output_truncates_or_sign_extends() {
        for length in 0..=8 {
            let mut bytes = [0; 8];
            write_big_endian(-0x1234568, &mut bytes[..length]);
            assert_eq!(
                &bytes[..length],
                &(-0x1234568i64).to_be_bytes()[8 - length..]
            );
        }
    }
}
