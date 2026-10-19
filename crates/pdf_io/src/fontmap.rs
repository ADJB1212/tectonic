// Copyright 2026 the Tectonic Project
// Copyright 1998, 1999 Mark A. Wicks
// Copyright 2002-2018 Jin-Hwan Cho, Shunsaku Hirata, and the dvipdfmx team
// SPDX-License-Identifier: GPL-2.0-or-later

use std::{
    collections::{HashMap, hash_map::Entry},
    ffi::{CStr, CString, c_char},
    ops::Range,
    ptr,
    sync::Arc,
};
use tectonic_bridge_core::{CoreBridgeState, InputId};

#[derive(Clone, Copy)]
#[repr(C)]
struct FontMapOptions {
    slant: f64,
    extend: f64,
    bold: f64,
    mapc: i32,
    flags: i32,
    otl_tags: *mut c_char,
    tounicode: *mut c_char,
    design_size: f64,
    charcoll: *mut c_char,
    index: u32,
    style: i32,
    stemv: i32,
    use_glyph_encoding: i32,
}

impl Default for FontMapOptions {
    fn default() -> Self {
        Self {
            slant: 0.0,
            extend: 1.0,
            bold: 0.0,
            mapc: -1,
            flags: 0,
            otl_tags: ptr::null_mut(),
            tounicode: ptr::null_mut(),
            design_size: -1.0,
            charcoll: ptr::null_mut(),
            index: 0,
            style: 0,
            stemv: -1,
            use_glyph_encoding: 0,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct FontMapRecord {
    map_name: *mut c_char,
    font_name: *mut c_char,
    enc_name: *mut c_char,
    sfd_name: *mut c_char,
    subfont_id: *mut c_char,
    opt: FontMapOptions,
}

impl Default for FontMapRecord {
    fn default() -> Self {
        Self {
            map_name: ptr::null_mut(),
            font_name: ptr::null_mut(),
            enc_name: ptr::null_mut(),
            sfd_name: ptr::null_mut(),
            subfont_id: ptr::null_mut(),
            opt: FontMapOptions::default(),
        }
    }
}

struct OwnedRecord {
    record: FontMapRecord,
    _strings: Vec<CString>,
}

fn retain_string(strings: &mut Vec<CString>, value: &[u8]) -> *mut c_char {
    let string = CString::new(value).expect("font-map strings contain no NUL bytes");
    let pointer = string.as_ptr().cast_mut();
    strings.push(string);
    pointer
}

impl OwnedRecord {
    unsafe fn copy_from(key: &[u8], source: &FontMapRecord) -> Box<Self> {
        unsafe {
            let mut record = *source;
            let mut strings = Vec::new();
            for field in [
                &mut record.map_name,
                &mut record.font_name,
                &mut record.enc_name,
                &mut record.sfd_name,
                &mut record.subfont_id,
                &mut record.opt.otl_tags,
                &mut record.opt.tounicode,
                &mut record.opt.charcoll,
            ] {
                if !field.is_null() {
                    let bytes = CStr::from_ptr(*field).to_bytes();
                    *field = retain_string(&mut strings, bytes);
                }
            }
            if !source.map_name.is_null() && CStr::from_ptr(source.map_name).to_bytes() == key {
                record.map_name = ptr::null_mut();
            }
            Box::new(Self {
                record,
                _strings: strings,
            })
        }
    }
}

struct ParsedRecord {
    source: Arc<[u8]>,
    font: Range<usize>,
    encoding: Option<Range<usize>>,
    slant: f64,
    extend: f64,
}

impl ParsedRecord {
    fn materialize(&self) -> Box<OwnedRecord> {
        let mut strings = Vec::new();
        let font_name = retain_string(&mut strings, &self.source[self.font.clone()]);
        let enc_name = self.encoding.as_ref().map_or(ptr::null_mut(), |span| {
            retain_string(&mut strings, &self.source[span.clone()])
        });
        let record = FontMapRecord {
            font_name,
            enc_name,
            opt: FontMapOptions {
                slant: self.slant,
                extend: self.extend,
                ..Default::default()
            },
            ..Default::default()
        };
        Box::new(OwnedRecord {
            record,
            _strings: strings,
        })
    }
}

enum StoredRecord {
    Parsed(ParsedRecord),
    Ready(Box<OwnedRecord>),
}

impl StoredRecord {
    fn as_mut_ptr(&mut self) -> *mut FontMapRecord {
        if let Self::Parsed(parsed) = self {
            *self = Self::Ready(parsed.materialize());
        }
        match self {
            Self::Ready(owned) => &mut owned.record,
            Self::Parsed(_) => unreachable!(),
        }
    }
}

#[derive(Default)]
pub(crate) struct FontMaps {
    records: HashMap<Box<[u8]>, StoredRecord>,
    reader: Option<MapReader>,
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Replace,
    Append,
    Remove,
}

impl Mode {
    fn from_raw(mode: i32) -> Self {
        match mode {
            0 => Self::Replace,
            43 => Self::Append,
            45 => Self::Remove,
            _ => panic!("invalid font-map mode"),
        }
    }
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl Cursor<'_> {
    fn skip_blank(&mut self) {
        while self
            .bytes
            .get(self.position)
            .is_some_and(|b| matches!(b, b' ' | b'\t'))
        {
            self.position += 1;
        }
    }

    fn token(&mut self) -> Option<Range<usize>> {
        let mut start = self.position;
        if *self.bytes.get(start)? == b'"' {
            start += 1;
            self.position = start;
            while let Some(&byte) = self.bytes.get(self.position) {
                if byte == b'\\' {
                    return None;
                }
                if byte == b'"' {
                    let end = self.position;
                    self.position += 1;
                    return Some(start..end);
                }
                self.position += 1;
            }
            None
        } else {
            while self
                .bytes
                .get(self.position)
                .is_some_and(|b| !b.is_ascii_whitespace())
            {
                self.position += 1;
            }
            (self.position > start).then_some(start..self.position)
        }
    }

    fn number(&mut self) -> Result<Option<f64>, ()> {
        let start = self.position;
        if self
            .bytes
            .get(self.position)
            .is_some_and(|b| matches!(b, b'+' | b'-'))
        {
            self.position += 1;
        }
        let mut state = 0;
        let mut digits = 0;
        while let Some(&byte) = self.bytes.get(self.position) {
            match byte {
                b'+' | b'-' if state == 2 => state = 3,
                b'.' if state == 0 => state = 1,
                b'0'..=b'9' => digits += 1,
                b'e' | b'E' if digits > 0 && state != 2 => state = 2,
                _ => break,
            }
            self.position += 1;
        }
        if digits == 0 {
            return Ok(None);
        }
        std::str::from_utf8(&self.bytes[start..self.position])
            .map_err(|_| ())?
            .parse::<f64>()
            .map(Some)
            .map_err(|_| ())
    }
}

fn map_format(line: &[u8]) -> i32 {
    if line.iter().any(|b| matches!(b, b'"' | b'<')) {
        return -1;
    }
    let mut cursor = Cursor {
        bytes: line,
        position: 0,
    };
    let mut count = 0;
    cursor.skip_blank();
    while cursor.position < line.len() {
        if line[cursor.position] == b'-' {
            return 1;
        }
        while line
            .get(cursor.position)
            .is_some_and(|b| !matches!(b, b' ' | b'\t'))
        {
            cursor.position += 1;
        }
        count += 1;
        cursor.skip_blank();
    }
    i32::from(count != 2)
}

struct ParsedLine {
    key: Range<usize>,
    font: Range<usize>,
    encoding: Option<Range<usize>>,
    slant: f64,
    extend: f64,
}

fn parse_dvips(line: &[u8]) -> Option<ParsedLine> {
    let mut cursor = Cursor {
        bytes: line,
        position: 0,
    };
    cursor.skip_blank();
    let key = cursor.token()?;
    if line[key.clone()].contains(&b'@') {
        return None;
    }
    cursor.skip_blank();
    if !matches!(line.get(cursor.position), Some(b'"' | b'<')) {
        cursor.token()?;
        cursor.skip_blank();
    }
    let mut parsed = ParsedLine {
        font: key.clone(),
        key,
        encoding: None,
        slant: 0.0,
        extend: 1.0,
    };
    while cursor.position < line.len() {
        match line[cursor.position] {
            b'<' => {
                cursor.position += 1;
                if matches!(line.get(cursor.position), Some(b'[' | b'<')) {
                    cursor.position += 1;
                }
                cursor.skip_blank();
                if cursor.position < line.len() {
                    let span = cursor.token()?;
                    let value = &line[span.clone()];
                    if value.len() > 4 && value.ends_with(b".enc") {
                        parsed.encoding = Some(span);
                    } else if value != b"default" && value != b"none" {
                        parsed.font = span;
                    } else {
                        parsed.font = parsed.key.clone();
                    }
                }
            }
            b'"' => {
                let span = cursor.token()?;
                let mut options = Cursor {
                    bytes: &line[span],
                    position: 0,
                };
                options.skip_blank();
                while options.position < options.bytes.len() {
                    if let Some(value) = options.number().ok()? {
                        options.skip_blank();
                        if options.position < options.bytes.len() {
                            let token = options.token()?;
                            match &options.bytes[token] {
                                b"SlantFont" => parsed.slant = value,
                                b"ExtendFont" => parsed.extend = value,
                                _ => (),
                            }
                        }
                    } else if options.position < options.bytes.len() {
                        options.token()?;
                    }
                    options.skip_blank();
                }
            }
            _ => return None,
        }
        cursor.skip_blank();
    }
    Some(parsed)
}

struct MapReader {
    source: Arc<[u8]>,
    position: usize,
    line_number: i32,
    format: i32,
    mode: Mode,
    verbose: bool,
    pending: Option<CString>,
}

impl MapReader {
    fn new(source: Vec<u8>, mode: Mode, verbose: bool) -> Self {
        Self {
            source: source.into(),
            position: 0,
            line_number: 0,
            format: 0,
            mode,
            verbose,
            pending: None,
        }
    }

    fn next(&mut self, records: &mut HashMap<Box<[u8]>, StoredRecord>) -> i32 {
        self.pending = None;
        while self.position < self.source.len() {
            let start = self.position;
            let limit = self.source.len().min(start.saturating_add(1023));
            let length = self.source[start..limit]
                .iter()
                .position(|b| matches!(b, b'\r' | b'\n'));
            let end = length.map_or(limit, |length| start + length);
            self.position = end;
            if length.is_some() {
                self.position += 1;
                if self.source[end] == b'\r' && self.source.get(self.position) == Some(&b'\n') {
                    self.position += 1;
                }
            }
            self.line_number += 1;
            let line = &self.source[start..end];
            let end = start
                + line
                    .iter()
                    .position(|b| matches!(b, 0 | b'%'))
                    .unwrap_or(line.len());
            let line = &self.source[start..end];
            if line.iter().all(|b| matches!(b, b' ' | b'\t')) {
                continue;
            }
            let format = map_format(line);
            let mismatch = self.format.signum() * format < 0;
            if !mismatch {
                self.format = self.format.saturating_add(format);
            }
            if !mismatch
                && self.format <= 0
                && !self.verbose
                && let Some(parsed) = parse_dvips(line)
            {
                let key = &line[parsed.key];
                if self.mode == Mode::Remove {
                    records.remove(key);
                } else if self.mode == Mode::Replace || !records.contains_key(key) {
                    let record = ParsedRecord {
                        source: self.source.clone(),
                        font: start + parsed.font.start..start + parsed.font.end,
                        encoding: parsed
                            .encoding
                            .map(|span| start + span.start..start + span.end),
                        slant: parsed.slant,
                        extend: parsed.extend,
                    };
                    records.insert(key.into(), StoredRecord::Parsed(record));
                }
                continue;
            }
            self.pending = Some(CString::new(line).expect("line was truncated at NUL"));
            return if mismatch { 2 } else { 1 };
        }
        0
    }
}

#[unsafe(no_mangle)]
extern "C" fn tt_fontmap_reset() {
    crate::compat::document::with_font_maps(|maps| *maps.borrow_mut() = FontMaps::default());
}

#[unsafe(no_mangle)]
pub(crate) extern "C" fn tt_fontmap_clear() {
    tt_fontmap_reset();
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_fontmap_lookup(key: *const c_char) -> *mut FontMapRecord {
    unsafe {
        if key.is_null() {
            return ptr::null_mut();
        }
        let key = CStr::from_ptr(key).to_bytes();
        crate::compat::document::with_font_maps(|maps| {
            maps.borrow_mut()
                .records
                .get_mut(key)
                .map_or(ptr::null_mut(), StoredRecord::as_mut_ptr)
        })
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_fontmap_insert(
    key: *const c_char,
    record: *const FontMapRecord,
    replace: i32,
) -> *mut FontMapRecord {
    unsafe {
        let key = CStr::from_ptr(key).to_bytes();
        // Copy before borrowing the table: the source may itself be a stored record.
        let owned = OwnedRecord::copy_from(key, &*record);
        crate::compat::document::with_font_maps(|maps| {
            let mut maps = maps.borrow_mut();
            let stored = match maps.records.entry(key.into()) {
                Entry::Occupied(mut entry) => {
                    if replace != 0 {
                        entry.insert(StoredRecord::Ready(owned));
                    }
                    entry.into_mut()
                }
                Entry::Vacant(entry) => entry.insert(StoredRecord::Ready(owned)),
            };
            stored.as_mut_ptr()
        })
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_fontmap_remove(key: *const c_char) {
    unsafe {
        let key = CStr::from_ptr(key).to_bytes();
        crate::compat::document::with_font_maps(|maps| {
            maps.borrow_mut().records.remove(key);
        });
    }
}

#[unsafe(no_mangle)]
extern "C" fn tt_fontmap_load_begin(handle: Option<InputId>, mode: i32, verbose: i32) -> i32 {
    let mut bytes = Vec::new();
    let result = CoreBridgeState::with_global_state(|state| {
        state.input_read_to_end(handle.expect("open font-map handle"), &mut bytes)
    });
    crate::compat::document::with_font_maps(|maps| {
        let mut maps = maps.borrow_mut();
        maps.records
            .reserve(bytes.iter().filter(|&&b| b == b'\n').count());
        maps.reader = Some(MapReader::new(bytes, Mode::from_raw(mode), verbose > 3));
    });
    if result.is_ok() { 0 } else { -1 }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_fontmap_load_next(
    line: *mut *const c_char,
    line_number: *mut i32,
    format: *mut i32,
) -> i32 {
    unsafe {
        crate::compat::document::with_font_maps(|maps| {
            let mut maps = maps.borrow_mut();
            let FontMaps { records, reader } = &mut *maps;
            let reader = reader.as_mut().expect("font-map load in progress");
            let result = reader.next(records);
            *line = reader
                .pending
                .as_ref()
                .map_or(ptr::null(), |line| line.as_ptr());
            *line_number = reader.line_number;
            *format = reader.format;
            result
        })
    }
}

#[unsafe(no_mangle)]
extern "C" fn tt_fontmap_load_end() {
    crate::compat::document::with_font_maps(|maps| maps.borrow_mut().reader = None);
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "C" {
        fn pdf_init_fontmap_record(record: *mut FontMapRecord);
        fn pdf_read_fontmap_line(
            record: *mut FontMapRecord,
            line: *const c_char,
            length: i32,
            format: i32,
        ) -> i32;
        fn pdf_clear_fontmap_record(record: *mut FontMapRecord);
    }

    unsafe fn strings(record: &FontMapRecord) -> Vec<Option<Vec<u8>>> {
        [
            record.map_name,
            record.font_name,
            record.enc_name,
            record.sfd_name,
            record.subfont_id,
            record.opt.otl_tags,
            record.opt.tounicode,
            record.opt.charcoll,
        ]
        .iter()
        .map(|&pointer| {
            (!pointer.is_null()).then(|| unsafe { CStr::from_ptr(pointer) }.to_bytes().to_vec())
        })
        .collect()
    }

    #[test]
    fn dvips_records_match_native_parser_including_options_and_quoted_fields() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        for line in [
            "cmr10 CMR10 <cmr10.pfb",
            "cmr10 <cmr10.pfb",
            "cmr10 CMR10",
            "cmr10 CMR10 <[font.enc <<cmr10.pfb",
            "cmr10 CMR10 <font.enc <cmr10.pfb",
            "\"cmr10\" \"PostScript Name\" <\"font file.pfb\"",
            "cmr10 CMR10 \".167 SlantFont .82 ExtendFont\" <cmr10.pfb",
            "cmr10 CMR10 \"1.2 ExtendFont ReEncodeFont -.2 SlantFont\" <cmr10.pfb",
            "cmr10 CMR10 \"1.25e-2 SlantFont +01.00E+1 ExtendFont\" <cmr10.pfb",
            "cmr10 CMR10 <old.enc <new.enc <old.pfb <cmr10.pfb",
            "cmr10 CMR10 <none",
            "cmr10 CMR10 <default",
            "cmr10 CMR10 <",
        ] {
            let source = line.as_bytes().to_vec();
            let parsed = parse_dvips(&source).expect(line);
            let key = source[parsed.key].to_vec();
            let parsed = ParsedRecord {
                source: source.into(),
                font: parsed.font,
                encoding: parsed.encoding,
                slant: parsed.slant,
                extend: parsed.extend,
            }
            .materialize();
            let line = CString::new(line).unwrap();
            let mut native = FontMapRecord::default();
            // SAFETY: The native parser initializes and owns the temporary record.
            unsafe {
                pdf_init_fontmap_record(&mut native);
                assert_eq!(
                    pdf_read_fontmap_line(
                        &mut native,
                        line.as_ptr(),
                        line.as_bytes().len() as i32,
                        0
                    ),
                    0
                );
                let native = OwnedRecord::copy_from(&key, &native);
                assert_eq!(strings(&parsed.record), strings(&native.record), "{line:?}");
                assert_eq!(parsed.record.opt.slant, native.record.opt.slant);
                assert_eq!(parsed.record.opt.extend, native.record.opt.extend);
            }
            // SAFETY: Only the C parser's allocations are released by C.
            unsafe {
                pdf_clear_fontmap_record(&mut native);
            }
        }
    }

    #[test]
    fn legacy_syntax_escapes_and_malformed_options_are_deferred() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        for line in [
            b"font@SFD@ Font <font.pfb".as_slice(),
            b"font Font <\"font\\056pfb\"",
            b"font Font \"1e SlantFont\"",
            b"font Font <\"unterminated",
            b"font Font garbage",
        ] {
            assert!(parse_dvips(line).is_none(), "{line:?}");
        }
        assert_eq!(map_format(b"font default font.pfb -s 0.2"), 1);
        assert_eq!(map_format(b"font Font"), 0);
        assert_eq!(map_format(b"font Font <font.pfb"), -1);
    }

    #[test]
    fn loading_is_lazy_and_materialized_record_pointers_survive_table_growth() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let source = b"font Font <font.pfb\nother Other <other.pfb\n".to_vec();
        let mut reader = MapReader::new(source, Mode::Append, false);
        let mut records = HashMap::new();
        assert_eq!(reader.next(&mut records), 0);
        assert_eq!(records.len(), 2);
        assert!(
            records
                .values()
                .all(|record| matches!(record, StoredRecord::Parsed(_)))
        );
        let pointer = records.get_mut(b"font".as_slice()).unwrap().as_mut_ptr();
        for index in 0..10000 {
            let source = format!("f{index} Font <font.pfb\n").into_bytes();
            assert_eq!(
                MapReader::new(source, Mode::Append, false).next(&mut records),
                0
            );
        }
        assert_eq!(
            records.get_mut(b"font".as_slice()).unwrap().as_mut_ptr(),
            pointer
        );
        // SAFETY: The boxed record remains owned by the table across growth.
        assert_eq!(
            unsafe { CStr::from_ptr((*pointer).font_name) }.to_bytes(),
            b"font.pfb"
        );
        assert!(matches!(
            records.get(b"other".as_slice()),
            Some(StoredRecord::Parsed(_))
        ));
    }

    #[test]
    fn line_chunks_comments_nuls_crlf_and_format_mismatches_match_legacy_rules() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let mut source =
            format!("%{}\r\nfont Font <font.pfb%ignored\r", " ".repeat(8190)).into_bytes();
        source.extend_from_slice(
            b"other Other <other.pfb\0ignored\nfont default font.pfb -s .2\nlast Last <last.pfb",
        );
        let mut reader = MapReader::new(source, Mode::Append, false);
        let mut records = HashMap::new();
        assert_eq!(reader.next(&mut records), 2);
        assert!(
            reader
                .pending
                .as_ref()
                .unwrap()
                .to_bytes()
                .starts_with(b"font default")
        );
        assert!(records.contains_key(b"font".as_slice()));
        assert!(records.contains_key(b"other".as_slice()));
        assert_eq!(reader.next(&mut records), 0);
        assert!(records.contains_key(b"last".as_slice()));
        assert_eq!(records.len(), 3);
    }

    #[test]
    fn append_replace_remove_and_reset_do_not_reuse_prior_map_contents() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let mut records = HashMap::new();
        for (mode, source, expected) in [
            (
                Mode::Append,
                "font Font <first.pfb",
                Some(b"first.pfb".as_slice()),
            ),
            (
                Mode::Append,
                "font Font <second.pfb",
                Some(b"first.pfb".as_slice()),
            ),
            (
                Mode::Replace,
                "font Font <second.pfb",
                Some(b"second.pfb".as_slice()),
            ),
            (Mode::Remove, "font Font <second.pfb", None),
            (
                Mode::Append,
                "font Font <third.pfb",
                Some(b"third.pfb".as_slice()),
            ),
        ] {
            assert_eq!(
                MapReader::new(source.as_bytes().to_vec(), mode, false).next(&mut records),
                0
            );
            let found = records.get_mut(b"font".as_slice()).map(|record| {
                // SAFETY: The materialized string remains owned by this record.
                unsafe {
                    CStr::from_ptr((*record.as_mut_ptr()).font_name)
                        .to_bytes()
                        .to_vec()
                }
            });
            assert_eq!(found.as_deref(), expected);
        }
        crate::compat::document::with_font_maps(|maps| maps.borrow_mut().records = records);
        tt_fontmap_reset();
        crate::compat::document::with_font_maps(|maps| assert!(maps.borrow().records.is_empty()));
    }

    #[test]
    fn inserted_subfont_links_and_options_outlive_the_callers_strings() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        tt_fontmap_reset();
        let key = CString::new("font00").unwrap();
        let source = {
            let strings: Vec<_> = [
                "font@SFD@",
                "Font.otf",
                "Identity-H",
                "SFD",
                "00",
                "ToUnicode",
                "Adobe-Japan1-4",
                "latn",
            ]
            .into_iter()
            .map(|s| CString::new(s).unwrap())
            .collect();
            let pointers: Vec<_> = strings.iter().map(|s| s.as_ptr().cast_mut()).collect();
            let record = FontMapRecord {
                map_name: pointers[0],
                font_name: pointers[1],
                enc_name: pointers[2],
                sfd_name: pointers[3],
                subfont_id: pointers[4],
                opt: FontMapOptions {
                    tounicode: pointers[5],
                    charcoll: pointers[6],
                    otl_tags: pointers[7],
                    index: 3,
                    flags: 4,
                    mapc: 0x1200,
                    bold: 0.3,
                    slant: -0.2,
                    extend: 0.8,
                    style: 3,
                    stemv: 80,
                    use_glyph_encoding: 1,
                    ..Default::default()
                },
            };
            // SAFETY: All pointers are live C strings and insertion copies them.
            unsafe { tt_fontmap_insert(key.as_ptr(), &record, 1) }
        };
        // SAFETY: The table owns the returned record after the caller's strings drop.
        unsafe {
            assert_eq!(tt_fontmap_lookup(key.as_ptr()), source);
            assert_eq!(CStr::from_ptr((*source).map_name).to_bytes(), b"font@SFD@");
            assert_eq!(CStr::from_ptr((*source).sfd_name).to_bytes(), b"SFD");
            assert_eq!(CStr::from_ptr((*source).subfont_id).to_bytes(), b"00");
            assert_eq!(
                CStr::from_ptr((*source).opt.tounicode).to_bytes(),
                b"ToUnicode"
            );
            assert_eq!((*source).opt.index, 3);
            assert_eq!((*source).opt.flags, 4);
            assert_eq!((*source).opt.extend, 0.8);
            tt_fontmap_remove(key.as_ptr());
            assert!(tt_fontmap_lookup(key.as_ptr()).is_null());
        }
        tt_fontmap_clear();
    }
}
