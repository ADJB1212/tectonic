//! Byte-oriented PDF and PostScript lexical operations.
#![forbid(unsafe_code)]

use crate::error::{PdfError, PdfErrorKind};

pub(crate) fn error(message: &'static str, offset: usize) -> PdfError {
    PdfError::new(
        PdfErrorKind::MalformedInput,
        std::ffi::CString::new(message).unwrap(),
    )
    .at(offset as u64)
}

pub(crate) fn white(byte: u8) -> bool {
    matches!(byte, 0 | b' ' | b'\t' | b'\x0c' | b'\r' | b'\n')
}

pub(crate) fn delimiter(byte: u8) -> bool {
    white(byte) || b"()/<>[]%".contains(&byte)
}

pub(crate) fn skip_line(bytes: &[u8]) -> usize {
    let mut pos = bytes
        .iter()
        .position(|b| matches!(b, b'\r' | b'\n'))
        .unwrap_or(bytes.len());
    if bytes.get(pos) == Some(&b'\r') {
        pos += 1;
    }
    if bytes.get(pos) == Some(&b'\n') {
        pos += 1;
    }
    pos
}

pub(crate) fn skip_white(bytes: &[u8]) -> usize {
    let mut pos = 0;
    while let Some(&byte) = bytes.get(pos) {
        if byte == b'%' {
            pos += skip_line(&bytes[pos..]);
        } else if white(byte) {
            pos += 1;
        } else {
            break;
        }
    }
    pos
}

pub(crate) fn keyword(bytes: &[u8], word: &[u8]) -> bool {
    bytes.starts_with(word) && bytes.get(word.len()).is_none_or(|&b| delimiter(b))
}

pub(crate) fn number_length(bytes: &[u8], unsigned: bool) -> usize {
    let mut pos = usize::from(!unsigned && bytes.first().is_some_and(|b| matches!(b, b'+' | b'-')));
    while bytes.get(pos).is_some_and(u8::is_ascii_digit) {
        pos += 1;
    }
    if !unsigned && bytes.get(pos) == Some(&b'.') {
        pos += 1;
        while bytes.get(pos).is_some_and(u8::is_ascii_digit) {
            pos += 1;
        }
    }
    pos
}

pub(crate) fn number(bytes: &[u8]) -> Result<(f64, usize), PdfError> {
    let mut pos = skip_white(bytes);
    if !bytes
        .get(pos)
        .is_some_and(|b| b.is_ascii_digit() || b".+-".contains(b))
    {
        return Err(error("Could not find a numeric object.", pos));
    }
    let sign = if bytes[pos] == b'-' { -1.0 } else { 1.0 };
    if matches!(bytes[pos], b'+' | b'-') {
        pos += 1;
        if pos == bytes.len() {
            return Err(error("Could not find a numeric object.", pos));
        }
    }
    let mut value = 0.0;
    let mut decimals = 0.0;
    let mut dot = false;
    while let Some(&b) = bytes.get(pos).filter(|&&b| !delimiter(b)) {
        if b == b'.' && !dot {
            dot = true;
        } else if b.is_ascii_digit() {
            if dot {
                decimals += 1.0;
                value += f64::from(b - b'0') / 10.0f64.powf(decimals);
            } else {
                value = value * 10.0 + f64::from(b) - f64::from(b'0');
            }
        } else {
            return Err(error("Could not find a numeric object.", pos));
        }
        pos += 1;
    }
    Ok((sign * value, pos))
}

pub(crate) fn reference(bytes: &[u8]) -> Option<(u32, u16, usize)> {
    fn unsigned(bytes: &[u8], pos: &mut usize, max: u32) -> Option<u32> {
        let begin = *pos;
        let mut value = 0u32;
        while let Some(b) = bytes.get(*pos).filter(|b| b.is_ascii_digit()) {
            value = value.checked_mul(10)?.checked_add(u32::from(*b - b'0'))?;
            if value > max {
                return None;
            }
            *pos += 1;
        }
        (*pos > begin).then_some(value)
    }
    let mut pos = skip_white(bytes);
    let id = unsigned(bytes, &mut pos, u32::MAX)?;
    if !bytes.get(pos).is_some_and(|&b| white(b)) {
        return None;
    }
    pos += skip_white(&bytes[pos..]);
    let generation = unsigned(bytes, &mut pos, u16::MAX.into())? as u16;
    if !bytes.get(pos).is_some_and(|&b| white(b)) {
        return None;
    }
    pos += skip_white(&bytes[pos..]);
    if !keyword(&bytes[pos..], b"R") {
        return None;
    }
    Some((id, generation, pos + 1))
}

pub(crate) fn hex_digit(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

pub(crate) fn ps_delimiter(byte: u8) -> bool {
    delimiter(byte) || matches!(byte, b'{' | b'}')
}

pub(crate) enum PsNumber {
    Integer(i32),
    Real(f64),
}

pub(crate) fn ps_number(bytes: &[u8]) -> Option<(PsNumber, usize)> {
    let start = bytes.iter().position(|&b| !white(b))?;
    let len = bytes[start..]
        .iter()
        .position(|&b| ps_delimiter(b))
        .unwrap_or(bytes.len() - start);
    let token = std::str::from_utf8(&bytes[start..start + len]).ok()?;
    if let Some((base, digits)) = token.split_once('#') {
        let base: u32 = base.parse().ok()?;
        if !(2..=36).contains(&base)
            || digits.is_empty()
            || (base == 16 && digits.starts_with(['x', 'X']))
        {
            return None;
        }
        let value = i64::from_str_radix(digits, base).ok()?;
        return Some((PsNumber::Integer(value as i32), start + len));
    }
    if !token.contains(['.', 'e', 'E'])
        && let Ok(value) = token.parse::<i64>()
    {
        return Some((PsNumber::Integer(value as i32), start + len));
    }
    if !token
        .as_bytes()
        .first()
        .is_some_and(|b| b.is_ascii_digit() || b".+-".contains(b))
    {
        return None;
    }
    let value: f64 = token.parse().ok()?;
    value
        .is_finite()
        .then_some((PsNumber::Real(value), start + len))
}

pub(crate) fn ps_name(
    bytes: &[u8],
    mut warning: impl FnMut(&'static str),
) -> Option<(Vec<u8>, usize)> {
    if bytes.first() != Some(&b'/') {
        return None;
    }
    let mut pos = 1;
    let mut count = 0;
    let mut output = Vec::new();
    while let Some(&byte) = bytes.get(pos).filter(|&&b| !ps_delimiter(b)) {
        pos += 1;
        let byte = if byte == b'#' {
            if bytes.len() - pos <= 2 {
                warning("Premature end of input name string.");
                break;
            }
            let hi = hex_digit(bytes[pos]);
            if hi.is_some() {
                pos += 1;
            }
            let lo = if hi.is_some() {
                hex_digit(bytes[pos])
            } else {
                None
            };
            if lo.is_some() {
                pos += 1;
            }
            match hi.zip(lo).map(|(hi, lo)| hi * 16 + lo) {
                Some(byte) if byte != 0 => byte,
                _ => {
                    warning("Invalid char for name object. (ignored)");
                    continue;
                }
            }
        } else {
            byte
        };
        if count < 127 {
            output.push(byte);
        }
        count += 1;
    }
    if count > 127 {
        warning("String too long for name object. Output will be truncated.");
    }
    Some((output, pos))
}

pub(crate) fn ps_hex(bytes: &[u8], mut invalid: impl FnMut(u8)) -> Option<(Vec<u8>, usize)> {
    if bytes.first() != Some(&b'<') || bytes.get(1) == Some(&b'<') {
        return None;
    }
    let mut pos = 1;
    let mut output = Vec::new();
    loop {
        pos += skip_white(&bytes[pos..]);
        let byte = *bytes.get(pos)?;
        if byte == b'>' {
            return Some((output, pos + 1));
        }
        if output.len() == 4096 {
            return None;
        }
        pos += 1;
        let hi = hex_digit(byte).unwrap_or_else(|| {
            invalid(byte);
            0
        });
        pos += skip_white(&bytes[pos..]);
        let byte = *bytes.get(pos)?;
        // Preserve the legacy PostScript parser's discarded odd nibble.
        if byte == b'>' {
            return Some((output, pos + 1));
        }
        pos += 1;
        let lo = hex_digit(byte).unwrap_or_else(|| {
            invalid(byte);
            0
        });
        output.push(hi * 16 + lo);
    }
}

pub(crate) fn name(
    bytes: &[u8],
    mut warning: impl FnMut(&'static str),
) -> Result<(Vec<u8>, usize), PdfError> {
    if bytes.first() != Some(&b'/') {
        return Err(error("Could not find a name object.", 0));
    }
    let mut pos = 1;
    let mut output = Vec::new();
    while let Some(&byte) = bytes.get(pos).filter(|&&b| !delimiter(b)) {
        pos += 1;
        let byte = if byte == b'#' {
            let pair = bytes.get(pos..pos + 2);
            pos = (pos + 2).min(bytes.len());
            let decoded = pair.and_then(|p| Some(hex_digit(p[0])? * 16 + hex_digit(p[1])?));
            match decoded {
                Some(0) => {
                    warning("Null char not allowed in PDF name object. (ignored)");
                    continue;
                }
                Some(byte) => byte,
                None => {
                    warning("Invalid char in PDF name object. (ignored)");
                    continue;
                }
            }
        } else if byte >= 128 {
            warning("Invalid char in PDF name object. (ignored)");
            continue;
        } else {
            byte
        };
        if output.len() == 128 {
            warning("PDF name length too long. (>= 128 bytes)");
        }
        if output.len() < 65536 {
            output.push(byte);
        } else {
            warning("PDF name length too long. (>= 65536 bytes, truncated)");
        }
    }
    if output.is_empty() {
        return Err(error("No valid name object found.", pos));
    }
    Ok((output, pos))
}

pub(crate) fn literal(
    bytes: &[u8],
    limit: usize,
    tainted: bool,
    postscript: bool,
) -> Result<(Vec<u8>, usize), PdfError> {
    if bytes.first() != Some(&b'(') {
        return Err(error("Could not find a string object.", 0));
    }
    let mut pos = 1;
    let mut depth = 0usize;
    let mut output = Vec::new();
    while let Some(&byte) = bytes.get(pos) {
        if byte == b')' && depth == 0 {
            return Ok((output, pos + 1));
        }
        if output.len() >= limit {
            return Err(error("PDF string length too long.", pos));
        }
        pos += 1;
        if tainted && byte >= 128 && pos < bytes.len() {
            if output.len() + 2 > limit {
                return Err(error("PDF string length too long.", pos));
            }
            output.extend_from_slice(&[byte, bytes[pos]]);
            pos += 1;
            continue;
        }
        match byte {
            b'(' => {
                depth += 1;
                output.push(byte);
            }
            b')' => {
                depth -= 1;
                output.push(byte);
            }
            b'\r' => {
                if bytes.get(pos) == Some(&b'\n') {
                    pos += 1;
                }
                output.push(b'\n');
            }
            b'\\' => {
                let Some(&escaped) = bytes.get(pos) else {
                    break;
                };
                pos += 1;
                match escaped {
                    b'n' => output.push(b'\n'),
                    b'r' => output.push(b'\r'),
                    b't' => output.push(b'\t'),
                    b'b' => output.push(8),
                    b'f' => output.push(12),
                    b'\r' => {
                        if bytes.get(pos) == Some(&b'\n') {
                            pos += 1;
                        }
                    }
                    b'\n' => {}
                    b'0'..=b'7' => {
                        let mut value = u16::from(escaped - b'0');
                        for _ in 0..2 {
                            if let Some(&digit @ b'0'..=b'7') = bytes.get(pos) {
                                value = value * 8 + u16::from(digit - b'0');
                                pos += 1;
                            } else {
                                break;
                            }
                        }
                        if !postscript || value <= 255 {
                            output.push(value as u8);
                        }
                    }
                    b'\\' | b'(' | b')' => output.push(escaped),
                    _ if postscript => {}
                    _ => output.push(escaped),
                }
            }
            _ => output.push(byte),
        }
    }
    Err(error(
        "Unbalanced parens/truncated PDF literal string.",
        pos,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncated_tokens_and_escapes_are_checked() {
        for bytes in [b"(".as_slice(), b"(\\", b"((x)", b"(\\123"] {
            assert!(literal(bytes, 65533, false, false).is_err());
        }
        assert!(!keyword(b"tru", b"true"));
        assert!(!keyword(b"truex", b"true"));
        assert!(keyword(b"true]", b"true"));
        assert!(reference(b"4294967296 0 R").is_none());
        assert!(reference(b"1 65536 R").is_none());
        assert!(reference(b"1 0 Rubbish").is_none());
        assert_eq!(
            reference(b"4294967295 65535 R"),
            Some((u32::MAX, u16::MAX, 18))
        );
    }

    #[test]
    fn literal_bytes_escapes_and_tainted_pairs_are_preserved() {
        assert_eq!(
            literal(b"(a(b)\\n\\777\\q\r\nz)tail", 100, false, false)
                .unwrap()
                .0,
            b"a(b)\n\xffq\nz"
        );
        assert_eq!(literal(b"(\x81\\)", 100, true, false).unwrap().0, b"\x81\\");
        assert!(literal(b"(abc)", 2, false, false).is_err());
        assert_eq!(literal(b"(abc)", 3, false, false).unwrap().0, b"abc");
    }

    #[test]
    fn names_use_owned_bytes_and_report_bad_escapes() {
        let mut warnings = Vec::new();
        assert_eq!(
            name(b"/a#20b#00#zz#", |w| warnings.push(w)).unwrap(),
            (b"a b".to_vec(), 13)
        );
        assert_eq!(warnings.len(), 3);
        let input = [b"/".as_slice(), &[b'x'; 65540]].concat();
        assert_eq!(name(&input, |_| {}).unwrap().0.len(), 65536);
    }

    #[test]
    fn whitespace_numbers_and_comments_retain_legacy_rules() {
        assert_eq!(skip_white(b" %hello\r\n\0 12"), 11);
        assert_eq!(number(b" -12.25]").unwrap(), (-12.25, 7));
        assert!(number(b"1.2.3").is_err());
        assert!(number(b"+").is_err());
        assert_eq!(number(b".]").unwrap(), (0.0, 1));
    }
}
