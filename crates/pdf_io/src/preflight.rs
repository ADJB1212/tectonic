// Copyright 2026 the Tectonic Project
// Copyright 1998, 1999 Mark A. Wicks
// Copyright 2002-2020 Jin-Hwan Cho, Shunsaku Hirata, and the dvipdfmx team
// Copyright 2012-2015 Khaled Hosny
// SPDX-License-Identifier: GPL-2.0-or-later

use std::{
    cell::RefCell,
    io::{Read, Seek, SeekFrom},
    ops::Range,
    slice,
};
use tectonic_bridge_core::{CoreBridgeState, InputId};

struct Page<'a> {
    bytes: &'a [u8],
    position: usize,
}

enum Command {
    Data,
    FontDefinition,
    Special(Range<usize>),
    End,
}

impl<'a> Page<'a> {
    fn take(&mut self, length: usize) -> Result<&'a [u8], ()> {
        let end = self.position.checked_add(length).ok_or(())?;
        let bytes = self.bytes.get(self.position..end).ok_or(())?;
        self.position = end;
        Ok(bytes)
    }

    fn unsigned(&mut self, length: usize) -> Result<usize, ()> {
        Ok(self
            .take(length)?
            .iter()
            .fold(0, |value, byte| value * 256 + usize::from(*byte)))
    }

    fn begin(&mut self) -> Result<(), ()> {
        if self.unsigned(1)? != 139 {
            return Err(());
        }
        self.take(44)?;
        Ok(())
    }

    fn command(&mut self, xdv: bool) -> Result<Command, ()> {
        let opcode = self.unsigned(1)?;
        let mut command = Command::Data;
        let skip = match opcode {
            0..=127 | 138 | 141 | 142 | 147 | 152 | 161 | 166 | 171..=234 => 0,
            128..=131 => opcode - 127,
            132 | 137 => 8,
            133..=136 => opcode - 132,
            140 => return Ok(Command::End),
            143..=146 => opcode - 142,
            148..=151 => opcode - 147,
            153..=156 => opcode - 152,
            157..=160 => opcode - 156,
            162..=165 => opcode - 161,
            167..=170 => opcode - 166,
            235..=238 => opcode - 234,
            239..=242 => {
                let length = self.unsigned(opcode - 238)?;
                // Let the native scanner diagnose oversized special lengths.
                if length > i32::MAX as usize {
                    return Err(());
                }
                let start = self.position;
                self.take(length)?;
                return Ok(Command::Special(start..self.position));
            }
            243..=246 => {
                self.take(opcode - 242 + 12)?;
                let area = self.unsigned(1)?;
                let name = self.unsigned(1)?;
                command = Command::FontDefinition;
                area + name
            }
            250 | 251 if xdv => 0,
            252 if xdv => {
                self.take(8)?;
                let flags = self.unsigned(2)?;
                let name = self.unsigned(1)?;
                command = Command::FontDefinition;
                name + 4 + (flags & 0x7200).count_ones() as usize * 4
            }
            253 | 254 if xdv => {
                if opcode == 254 {
                    let count = self.unsigned(2)?;
                    self.take(count * 2)?;
                }
                self.take(4)?;
                self.unsigned(2)? * 10
            }
            // pTeX direction changes also update native document state.
            _ => return Err(()),
        };
        self.take(skip)?;
        Ok(command)
    }

    fn eligible(&mut self, xdv: bool) -> Result<bool, ()> {
        self.begin()?;
        loop {
            match self.command(xdv)? {
                Command::End => return Ok(true),
                Command::Special(span) => {
                    if !crate::pipeline::special_supported(&self.bytes[span]) {
                        return Ok(false);
                    }
                }
                _ => (),
            }
        }
    }
}

#[repr(C)]
struct Special {
    offset: usize,
    length: usize,
}

#[repr(C)]
struct ScannedPage {
    data: *const u8,
    length: usize,
    specials: *const Special,
    special_count: usize,
}

#[derive(Default)]
struct PageScanner {
    bytes: Vec<u8>,
    definitions: Vec<Range<usize>>,
    specials: Vec<Special>,
}

impl PageScanner {
    fn scan(&mut self, xdv: bool) -> Result<(), ()> {
        self.definitions.clear();
        self.specials.clear();
        let mut page = Page {
            bytes: &self.bytes,
            position: 0,
        };
        page.begin()?;
        let mut removed = 0;
        loop {
            let start = page.position;
            match page.command(xdv)? {
                Command::End => break,
                Command::FontDefinition => {
                    self.definitions.push(start..page.position);
                    removed += page.position - start;
                }
                Command::Special(span) => self.specials.push(Special {
                    offset: span.start - removed,
                    length: span.len(),
                }),
                Command::Data => (),
            }
        }
        let length = page.position;
        let mut read = 0;
        let mut write = 0;
        for span in &self.definitions {
            self.bytes.copy_within(read..span.start, write);
            write += span.start - read;
            read = span.end;
        }
        self.bytes.copy_within(read..length, write);
        self.bytes.truncate(length - removed);
        Ok(())
    }
}

thread_local! {
    static PAGE_SCANNER: RefCell<PageScanner> = RefCell::default();
}

pub(crate) fn clear_page_scanner() {
    PAGE_SCANNER.with(|scanner| *scanner.borrow_mut() = PageScanner::default());
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_scan_page(
    handle: Option<InputId>,
    start: u32,
    end: u32,
    xdv: i32,
    output: *mut ScannedPage,
) -> i32 {
    unsafe {
        let Some(length) = end.checked_sub(start).filter(|&length| length > 0) else {
            return 0;
        };
        PAGE_SCANNER.with(|scanner| {
            let mut scanner = scanner.borrow_mut();
            scanner.bytes.resize(length as usize, 0);
            let result = CoreBridgeState::with_global_state(|state| {
                let input = state.get_input(handle.expect("open DVI handle"));
                input.seek(SeekFrom::Start(u64::from(start)))?;
                input.read_exact(&mut scanner.bytes)
            });
            if result.is_err() || scanner.scan(xdv != 0).is_err() {
                return 0;
            }
            *output = ScannedPage {
                data: scanner.bytes.as_ptr(),
                length: scanner.bytes.len(),
                specials: scanner.specials.as_ptr(),
                special_count: scanner.specials.len(),
            };
            1
        })
    }
}

fn preflight(
    input: &mut (impl Read + Seek),
    offsets: &[u32],
    file_size: u32,
    xdv: bool,
) -> Result<bool, ()> {
    let mut bytes = Vec::new();
    for (index, &start) in offsets.iter().enumerate() {
        let end = offsets.get(index + 1).copied().unwrap_or(file_size);
        let length = end.checked_sub(start).ok_or(())? as usize;
        if length == 0 || end > file_size {
            return Err(());
        }
        bytes.resize(length, 0);
        input
            .seek(SeekFrom::Start(u64::from(start)))
            .map_err(|_| ())?;
        input.read_exact(&mut bytes).map_err(|_| ())?;
        if !(Page {
            bytes: &bytes,
            position: 0,
        })
        .eligible(xdv)?
        {
            return Ok(false);
        }
    }
    Ok(true)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_pdf_preflight_pages(
    handle: Option<InputId>,
    offsets: *const u32,
    count: usize,
    file_size: u32,
    xdv: i32,
) -> i32 {
    unsafe {
        if count == 0 {
            return 1;
        }
        let offsets = slice::from_raw_parts(offsets, count);
        CoreBridgeState::with_global_state(|state| {
            match preflight(
                state.get_input(handle.expect("open DVI handle")),
                offsets,
                file_size,
                xdv != 0,
            ) {
                Ok(eligible) => i32::from(eligible),
                Err(()) => -1,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn page(commands: &[u8]) -> Vec<u8> {
        let mut bytes = vec![139];
        bytes.extend_from_slice(&[0; 44]);
        bytes.extend_from_slice(commands);
        bytes.push(140);
        bytes
    }

    #[test]
    fn playback_buffer_removes_all_font_definitions_and_preserves_special_offsets() {
        let mut commands = vec![65];
        let mut expected = vec![65];
        let mut payloads = Vec::new();
        for opcode in 243..=246 {
            commands.push(opcode);
            commands.extend(std::iter::repeat_n(0, usize::from(opcode - 242) + 12));
            commands.extend_from_slice(&[1, 2, 239, 140, 247]);
            let payload = format!("pdf:dest (page{opcode}) [ @thispage /Fit ]").into_bytes();
            let mut special = vec![239, payload.len() as u8];
            special.extend_from_slice(&payload);
            commands.extend_from_slice(&special);
            expected.extend_from_slice(&special);
            payloads.push(payload);
        }
        commands.push(252);
        commands.extend_from_slice(&[0; 8]);
        commands.extend_from_slice(&[0x73, 0x00, 3]);
        commands.extend_from_slice(&[239; 3 + 4 + 16]);
        commands.extend_from_slice(&[239, 0, 66]);
        expected.extend_from_slice(&[239, 0, 66]);
        payloads.push(Vec::new());
        let mut scanner = PageScanner {
            bytes: page(&commands),
            ..Default::default()
        };
        scanner.bytes.extend_from_slice(b"ignored postamble bytes");
        scanner.scan(true).unwrap();
        assert_eq!(scanner.bytes, page(&expected));
        assert_eq!(scanner.specials.len(), payloads.len());
        for (span, payload) in scanner.specials.iter().zip(payloads) {
            assert_eq!(
                &scanner.bytes[span.offset..span.offset + span.length],
                payload
            );
        }
    }

    #[test]
    fn playback_scan_validates_unknown_specials_and_rejects_truncation_before_compacting() {
        let payload = b"pdf:literal direct /Unknown BMC";
        let mut commands = vec![239, payload.len() as u8];
        commands.extend_from_slice(payload);
        commands.extend_from_slice(&[243]);
        commands.extend_from_slice(&[0; 13]);
        commands.extend_from_slice(&[0, 1, 65, 66]);
        let original = page(&commands);
        let mut scanner = PageScanner::default();
        for end in 0..original.len() {
            scanner.bytes = original[..end].to_vec();
            assert!(scanner.scan(true).is_err(), "truncation at {end}");
            assert_eq!(scanner.bytes, original[..end]);
        }
        scanner.bytes = original;
        scanner.scan(true).unwrap();
        assert_eq!(scanner.specials.len(), 1);
        assert_eq!(scanner.definitions.len(), 1);
        scanner.bytes = page(b"next page");
        scanner.scan(true).unwrap();
        assert!(scanner.specials.is_empty());
        assert!(scanner.definitions.is_empty());
    }

    #[test]
    fn opcode_arguments_and_font_definitions_cannot_be_mistaken_for_specials() {
        let mut commands = Vec::new();
        for (opcode, length) in [
            (128, 1),
            (131, 4),
            (132, 8),
            (134, 2),
            (136, 4),
            (144, 2),
            (150, 3),
            (155, 3),
            (160, 4),
            (165, 4),
            (170, 4),
            (238, 4),
        ] {
            commands.push(opcode);
            commands.extend(std::iter::repeat_n(239, length));
        }
        for opcode in 243..=246 {
            commands.push(opcode);
            commands.extend(std::iter::repeat_n(0, usize::from(opcode - 242) + 12));
            commands.extend_from_slice(&[2, 3, 239, 255, 247, 248, 249]);
        }
        commands.extend_from_slice(&[65, 138, 141, 142, 147, 152, 161, 166, 171, 234, 250, 251]);
        let bytes = page(&commands);
        assert_eq!(
            Page {
                bytes: &bytes,
                position: 0
            }
            .eligible(true),
            Ok(true)
        );
        assert_eq!(
            Page {
                bytes: &bytes,
                position: 0
            }
            .eligible(false),
            Err(())
        );
    }

    #[test]
    fn glyph_runs_text_and_all_native_font_options_are_skipped_exactly() {
        let mut commands = vec![252];
        commands.extend_from_slice(&[0; 8]);
        commands.extend_from_slice(&[0x73, 0x00, 4]);
        commands.extend_from_slice(&[239; 4 + 4 + 16]);
        commands.push(253);
        commands.extend_from_slice(&[0; 4]);
        commands.extend_from_slice(&[0, 2]);
        commands.extend_from_slice(&[239; 20]);
        commands.extend_from_slice(&[254, 0, 2, 0xd8, 0x34, 0xdd, 0x1e]);
        commands.extend_from_slice(&[0; 4]);
        commands.extend_from_slice(&[0, 1]);
        commands.extend_from_slice(&[239; 10]);
        let bytes = page(&commands);
        assert_eq!(
            Page {
                bytes: &bytes,
                position: 0
            }
            .eligible(true),
            Ok(true)
        );
        for end in 0..bytes.len() {
            assert!(
                Page {
                    bytes: &bytes[..end],
                    position: 0
                }
                .eligible(true)
                .is_err(),
                "truncation at {end}"
            );
        }
    }

    #[test]
    fn all_special_length_encodings_use_the_shared_compatibility_classifier() {
        for width in 1..=4 {
            for (special, expected) in [
                (b"color push rgb 1 0 0".as_slice(), true),
                (b"pdf:literal direct /Unknown BMC".as_slice(), false),
            ] {
                let mut commands = vec![238 + width as u8];
                let length = (special.len() as u32).to_be_bytes();
                commands.extend_from_slice(&length[4 - width..]);
                commands.extend_from_slice(special);
                let bytes = page(&commands);
                assert_eq!(
                    Page {
                        bytes: &bytes,
                        position: 0
                    }
                    .eligible(true),
                    Ok(expected)
                );
            }
        }
    }

    #[test]
    fn page_offsets_empty_documents_and_malformed_ranges_are_handled() {
        let first = page(b"First page");
        let mut bytes = vec![0; 17];
        bytes.extend_from_slice(&first);
        let second = bytes.len() as u32;
        bytes.extend_from_slice(&page(b"Second page"));
        let size = bytes.len() as u32;
        assert_eq!(
            preflight(&mut Cursor::new(&bytes), &[17, second], size, true),
            Ok(true)
        );
        assert_eq!(
            preflight(&mut Cursor::new(&bytes), &[], size, true),
            Ok(true)
        );
        assert_eq!(
            preflight(&mut Cursor::new(&bytes), &[second, 17], size, true),
            Err(())
        );
        assert_eq!(
            preflight(&mut Cursor::new(&bytes), &[17, size + 1], size, true),
            Err(())
        );
        for opcode in [139, 247, 248, 249, 255] {
            let bytes = page(&[opcode]);
            assert_eq!(
                Page {
                    bytes: &bytes,
                    position: 0
                }
                .eligible(true),
                Err(())
            );
        }
    }
}
