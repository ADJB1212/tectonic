//! Checked Type 2 charstring expansion with borrowed subroutine storage.
#![forbid(unsafe_code)]

use crate::{
    error::{PdfError, PdfErrorKind},
    syntax::error,
};

pub(crate) struct Subroutines<'a> {
    pub(crate) offsets: &'a [u32],
    pub(crate) bytes: &'a [u8],
}

impl Subroutines<'_> {
    fn get(&self, operand: i32) -> Result<&[u8], PdfError> {
        let count = self.offsets.len().saturating_sub(1);
        let bias = if count < 1240 {
            107
        } else if count < 33900 {
            1131
        } else {
            32768
        };
        let id = operand
            .checked_add(bias)
            .filter(|&id| id >= 0 && (id as usize) < count)
            .ok_or_else(|| error("Type2 Charstring Parser: Invalid Subr index.", 0))? as usize;
        let start = self.offsets[id]
            .checked_sub(1)
            .ok_or_else(|| error("Invalid CFF Index offset data", id))? as usize;
        let end = self.offsets[id + 1]
            .checked_sub(1)
            .ok_or_else(|| error("Invalid CFF Index offset data", id + 1))? as usize;
        self.bytes.get(start..end).ok_or_else(|| error("Invalid CFF Index offset data", id))
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Flow {
    Continue,
    Return,
    End,
}

struct Parser<'a, F> {
    stack: &'a mut Vec<f64>,
    transient: &'a mut [f64; 32],
    output: &'a mut [u8],
    written: usize,
    phase: u8,
    stems: usize,
    width: Option<f64>,
    warning: F,
}

impl<F: FnMut(&str)> Parser<'_, F> {
    fn failure(&self, status: i32) -> PdfError {
        PdfError::new(
            PdfErrorKind::MalformedInput,
            std::ffi::CString::new(format!(
                "Type2 Charstring Parser: Parsing charstring failed: (status={status}, stack={})",
                self.stack.len()
            ))
            .unwrap(),
        )
    }
    fn emit(&mut self, bytes: &[u8]) -> Result<(), PdfError> {
        let remaining = &mut self.output[self.written..];
        let Some(target) = remaining.get_mut(..bytes.len()) else {
            return Err(self.failure(-3));
        };
        target.copy_from_slice(bytes);
        self.written += bytes.len();
        Ok(())
    }
    fn push(&mut self, value: f64) -> Result<(), PdfError> {
        if self.stack.len() >= 48 {
            return Err(self.failure(-2));
        }
        self.stack.push(value);
        Ok(())
    }
    fn pop(&mut self) -> Result<f64, PdfError> {
        self.stack.pop().ok_or_else(|| self.failure(-2))
    }
    fn flush(&mut self) -> Result<(), PdfError> {
        for i in 0..self.stack.len() {
            let value = self.stack[i];
            let rounded = (value + 0.5).floor() as i32;
            if !value.is_finite() || value >= 32768.0 || value <= -32769.0 {
                return Err(error("Type2 Charstring Parser: Argument value too large. (This is bug)", 0));
            }
            if (value - f64::from(rounded)).abs() > 3.0e-5 {
                let integral = value.floor() as i32;
                let fraction = ((value - f64::from(integral)) * 65536.0) as i32;
                self.emit(&[255, (integral >> 8) as u8, integral as u8, (fraction >> 8) as u8, fraction as u8])?;
            } else if (-107..=107).contains(&rounded) {
                self.emit(&[(rounded + 139) as u8])?;
            } else if (108..=1131).contains(&rounded) {
                self.emit(&[((rounded - 108) / 256 + 247) as u8, ((rounded - 108) % 256) as u8])?;
            } else if (-1131..=-108).contains(&rounded) {
                self.emit(&[((-rounded - 108) / 256 + 251) as u8, ((-rounded - 108) % 256) as u8])?;
            } else {
                self.emit(&[28, (rounded >> 8) as u8, rounded as u8])?;
            }
        }
        self.stack.clear();
        Ok(())
    }
    fn operator(&mut self, op: u8, bytes: &[u8], pos: &mut usize) -> Result<Flow, PdfError> {
        match op {
            1 | 3 | 18 | 23 | 19 | 20 => {
                if self.phase < 2 {
                    if self.phase == 0 && !self.stack.len().is_multiple_of(2) {
                        self.width = self.stack.first().copied();
                    }
                    self.stems = self.stems.checked_add(self.stack.len() / 2).ok_or_else(|| self.failure(-1))?;
                }
                self.flush()?;
                self.emit(&[op])?;
                if matches!(op, 19 | 20) {
                    let count = self.stems.div_ceil(8);
                    let end = pos.checked_add(count).ok_or_else(|| self.failure(-1))?;
                    let mask = bytes.get(*pos..end).ok_or_else(|| self.failure(-1))?;
                    self.emit(mask)?;
                    *pos = end;
                    self.phase = 2;
                } else {
                    self.phase = 1;
                }
            }
            4 | 21 | 22 => {
                let width = if op == 21 {
                    !self.stack.len().is_multiple_of(2)
                } else {
                    self.stack.len().is_multiple_of(2)
                };
                if self.phase == 0 && width {
                    self.width = self.stack.first().copied();
                }
                self.flush()?;
                self.emit(&[op])?;
                self.phase = 2;
            }
            14 => {
                match self.stack.len() {
                    1 => {
                        self.width = self.stack.first().copied();
                        self.flush()?;
                    }
                    4 | 5 => {
                        (self.warning)("\"seac\" character deprecated in Type 2 charstring.");
                        return Err(self.failure(-1));
                    }
                    0 => {}
                    _ => (self.warning)("Type2 Charstring Parser: Operand stack not empty."),
                }
                self.emit(&[op])?;
                return Ok(Flow::End);
            }
            5..=8 | 24..=27 | 30 | 31 => {
                if self.phase < 2 {
                    (self.warning)("Type2 Charstring Parser: Broken Type 2 charstring.");
                    return Err(self.failure(-1));
                }
                self.flush()?;
                self.emit(&[op])?;
            }
            _ => {
                (self.warning)(&format!("Type2 Charstring Parser: Unknown charstring operator: 0x{op:02x}"));
                return Err(self.failure(-1));
            }
        }
        Ok(Flow::Continue)
    }
    fn escaped(&mut self, op: u8) -> Result<(), PdfError> {
        let required = match op {
            3 | 4 | 10..=12 | 15 | 20 | 24 | 28..=30 => 2,
            5 | 9 | 14 | 18 | 21 | 26 | 27 => 1,
            22 => 4,
            _ => 0,
        };
        if self.stack.len() < required {
            return Err(self.failure(-2));
        }
        match op {
            0 => {
                (self.warning)("Operator \"dotsection\" deprecated in Type 2 charstring.");
                return Err(self.failure(-1));
            }
            34..=37 => {
                if self.phase < 2 {
                    (self.warning)("Type2 Charstring Parser: Broken Type 2 charstring.");
                    return Err(self.failure(-1));
                }
                self.flush()?;
                self.emit(&[12, op])?;
            }
            3 | 4 | 10..=12 | 15 | 24 => {
                let b = self.pop()?;
                let a = self.pop()?;
                let value = match op {
                    3 => f64::from(a != 0.0 && b != 0.0),
                    4 => f64::from(a != 0.0 || b != 0.0),
                    10 => a + b,
                    11 => a - b,
                    12 => a / b,
                    15 => f64::from(a == b),
                    24 => a * b,
                    _ => unreachable!(),
                };
                self.push(value)?;
            }
            5 | 9 | 14 | 26 => {
                let value = self.pop()?;
                self.push(match op {
                    5 => f64::from(value == 0.0),
                    9 => value.abs(),
                    14 => -value,
                    26 => value.sqrt(),
                    _ => unreachable!(),
                })?;
            }
            18 => {
                self.pop()?;
            }
            20 => {
                let index = self.pop()? as i32;
                let value = self.pop()?;
                let Some(target) = usize::try_from(index).ok().and_then(|i| self.transient.get_mut(i)) else {
                    return Err(self.failure(-2));
                };
                *target = value;
            }
            21 => {
                let index = self.pop()? as i32;
                let value = usize::try_from(index)
                    .ok()
                    .and_then(|i| self.transient.get(i))
                    .copied()
                    .ok_or_else(|| self.failure(-2))?;
                self.push(value)?;
            }
            22 => {
                let b = self.pop()?;
                let a = self.pop()?;
                let second = self.pop()?;
                let first = self.pop()?;
                self.push(if a > b { second } else { first })?;
            }
            23 => {
                (self.warning)("Type2 Charstring Parser: Charstring operator \"random\" found.");
                self.push(1.0)?;
            }
            27 => {
                self.push(*self.stack.last().unwrap())?;
            }
            28 => {
                let n = self.stack.len();
                self.stack.swap(n - 1, n - 2);
            }
            29 => {
                let index = (self.pop()? as i32).max(0) as usize;
                let value = self
                    .stack
                    .len()
                    .checked_sub(index + 1)
                    .and_then(|i| self.stack.get(i))
                    .copied()
                    .ok_or_else(|| self.failure(-2))?;
                self.push(value)?;
            }
            30 => {
                let shift = self.pop()? as i32;
                let count = self.pop()? as i32;
                let count = usize::try_from(count)
                    .ok()
                    .filter(|&n| n > 0 && n <= self.stack.len())
                    .ok_or_else(|| self.failure(-2))?;
                let n = self.stack.len();
                self.stack[n - count..].rotate_right((i64::from(shift).rem_euclid(count as i64)) as usize);
            }
            _ => {
                (self.warning)(&format!("Type2 Charstring Parser: Unknown charstring operator: 0x0c{op:02x}"));
                return Err(self.failure(-1));
            }
        }
        Ok(())
    }
    fn parse(&mut self, bytes: &[u8], global: Option<&Subroutines<'_>>, local: Option<&Subroutines<'_>>, depth: usize) -> Result<Flow, PdfError> {
        if depth > 10 {
            return Err(error("Type2 Charstring Parser: Subroutine nested too deeply.", 0));
        }
        let mut pos = 0;
        while let Some(&byte) = bytes.get(pos) {
            pos += 1;
            let flow = match byte {
                255 => {
                    let end = pos.checked_add(4).ok_or_else(|| self.failure(-1))?;
                    let data: [u8; 4] = bytes.get(pos..end).ok_or_else(|| self.failure(-1))?.try_into().unwrap();
                    self.push(f64::from(i32::from_be_bytes(data)) / 65536.0)?;
                    pos = end;
                    Flow::Continue
                }
                11 => Flow::Return,
                10 | 29 => {
                    let id = self.pop()? as i32;
                    let subroutines = if byte == 10 { local } else { global }
                        .ok_or_else(|| error("Type2 Charstring Parser: Subroutine called but no subroutine found.", pos))?;
                    let data = subroutines.get(id)?;
                    if data.len() > self.output.len() - self.written {
                        return Err(error("Type2 Charstring Parser: Possible buffer overflow.", pos));
                    }
                    match self.parse(data, global, local, depth + 1)? {
                        Flow::Return => Flow::Continue,
                        flow => flow,
                    }
                }
                12 => {
                    let op = *bytes.get(pos).ok_or_else(|| self.failure(-1))?;
                    pos += 1;
                    self.escaped(op)?;
                    Flow::Continue
                }
                28 | 32..=254 => {
                    let (value, consumed) = crate::cff::integer(&bytes[pos - 1..]).map_err(|_| self.failure(-1))?;
                    self.push(value)?;
                    pos += consumed - 1;
                    Flow::Continue
                }
                _ => self.operator(byte, bytes, &mut pos)?,
            };
            if flow == Flow::End && pos < bytes.len() {
                (self.warning)("Type2 Charstring Parser: Garbage after endchar.");
            }
            if flow != Flow::Continue {
                return Ok(flow);
            }
        }
        Ok(Flow::Continue)
    }
}

pub(crate) struct Output {
    pub(crate) length: usize,
    pub(crate) width: Option<f64>,
}

pub(crate) fn copy(
    bytes: &[u8], output: &mut [u8], global: Option<&Subroutines<'_>>, local: Option<&Subroutines<'_>>, transient: &mut [f64; 32],
    stack: &mut Vec<f64>, warning: impl FnMut(&str),
) -> Result<Output, PdfError> {
    stack.clear();
    let mut parser = Parser {
        stack,
        transient,
        output,
        written: 0,
        phase: 0,
        stems: 0,
        width: None,
        warning,
    };
    parser.parse(bytes, global, local, 0)?;
    Ok(Output {
        length: parser.written,
        width: parser.width,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn expand(bytes: &[u8], capacity: usize) -> Result<Vec<u8>, PdfError> {
        let mut output = vec![0; capacity];
        let result = copy(bytes, &mut output, None, None, &mut [0.0; 32], &mut Vec::new(), |_| {})?;
        output.truncate(result.length);
        Ok(output)
    }
    #[test]
    fn expands_numbers_arithmetic_and_preserves_exact_capacity() {
        assert_eq!(expand(&[140, 141, 12, 10, 22, 14], 3).unwrap(), [142, 22, 14]);
        assert!(expand(&[140, 141, 12, 10, 22, 14], 2).is_err());
        assert_eq!(expand(&[255, 255, 255, 128, 0, 22, 14], 7).unwrap(), [255, 255, 255, 128, 0, 22, 14]);
    }
    #[test]
    fn allows_ten_nested_subroutines_and_rejects_eleven() {
        for depth in [10, 11] {
            let mut bytes = Vec::new();
            let mut offsets = vec![1];
            for index in 0..depth {
                if index + 1 < depth {
                    bytes.extend_from_slice(&[33 + index, if index % 2 == 0 { 29 } else { 10 }]);
                }
                bytes.push(11);
                offsets.push(bytes.len() as u32 + 1);
            }
            let subroutines = Subroutines {
                offsets: &offsets,
                bytes: &bytes,
            };
            let mut output = [0; 100];
            let result = copy(
                &[32, 10, 14],
                &mut output,
                Some(&subroutines),
                Some(&subroutines),
                &mut [0.0; 32],
                &mut Vec::new(),
                |_| {},
            );
            if depth == 10 {
                assert_eq!(&output[..result.unwrap().length], &[14]);
            } else {
                assert!(result.is_err());
            }
        }
    }
    #[test]
    fn checks_truncations_stack_indices_and_subroutine_bounds() {
        for bytes in [
            b"\x0c".as_slice(),
            &[28],
            &[255, 1],
            &[247],
            &[139, 139, 12, 30],
            &[139, 171, 12, 20],
            &[138, 12, 21],
            &[139, 139, 1, 19],
        ] {
            assert!(expand(bytes, 1024).is_err(), "{bytes:?}");
        }
        let subroutines = Subroutines {
            offsets: &[1, 2],
            bytes: &[11],
        };
        assert!(subroutines.get(-108).is_err());
        assert!(subroutines.get(-106).is_err());
        assert_eq!(subroutines.get(-107).unwrap(), [11]);
        let recursive = Subroutines {
            offsets: &[1, 4],
            bytes: &[32, 10, 11],
        };
        assert!(copy(&[32, 10], &mut [0; 100], None, Some(&recursive), &mut [0.0; 32], &mut Vec::new(), |_| {}).is_err());
    }
}
