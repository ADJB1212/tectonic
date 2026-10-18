// Copyright 2026 the Tectonic Project
// Copyright 1998, 1999 Mark A. Wicks
// Copyright 2002-2018 Jin-Hwan Cho, Shunsaku Hirata, and the dvipdfmx team
// SPDX-License-Identifier: GPL-2.0-or-later

use std::{
    borrow::Cow,
    cell::RefCell,
    collections::{HashMap, VecDeque},
    ffi::{CStr, CString, c_char},
    io::Read,
    ptr, slice,
};
use tectonic_bridge_core::{CoreBridgeState, FileFormat, InputId};

const FIXWORD_SCALE: f64 = 1_048_576.0;
const INVALID_HEADER: &str = "Invalid TFM header.";
const OFM_PROBLEM: &str = "OFM file problem.  Table sizes don't agree.";
const CANNOT_PROCEED: &str = "Can't proceed...";
const MESSAGE: i32 = 0;
const WARNING: i32 = 1;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Source {
    Tfm,
    Ofm,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum OpenStep {
    AliasOfm,
    Tfm,
    Ofm,
}

#[derive(Debug)]
struct Metrics {
    design_size: i32,
    first: i32,
    last: i32,
    widths: Vec<i32>,
    heights: Vec<i32>,
    depths: Vec<i32>,
}

struct CharMap {
    width: Vec<usize>,
    height: Vec<usize>,
    depth: Vec<usize>,
}

#[derive(Default)]
struct Fonts {
    names: HashMap<Box<[u8]>, i32>,
    metrics: Vec<Metrics>,
}

struct Diagnostic {
    kind: i32,
    text: CString,
}

#[derive(Default)]
pub(crate) struct DocumentState {
    fonts: RefCell<Fonts>,
    diagnostics: RefCell<VecDeque<Diagnostic>>,
    held_diagnostic: RefCell<Option<CString>>,
    last_error: RefCell<Option<CString>>,
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

fn premature() -> String {
    "File ended prematurely\n".to_string()
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn read(&mut self, count: usize) -> Result<&'a [u8], String> {
        let end = self.pos.checked_add(count).ok_or_else(premature)?;
        let data = self.bytes.get(self.pos..end).ok_or_else(premature)?;
        self.pos = end;
        Ok(data)
    }

    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.read(1)?[0])
    }

    fn u16(&mut self) -> Result<u32, String> {
        let bytes = self.read(2)?;
        Ok(u32::from(u16::from_be_bytes([bytes[0], bytes[1]])))
    }

    fn i32(&mut self) -> Result<i32, String> {
        let bytes = self.read(4)?;
        Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn positive(&mut self, name: &str) -> Result<u32, String> {
        let value = self.i32()?;
        if value < 0 {
            Err(format!("Bad OFM: negative {name}: {value}"))
        } else {
            Ok(value as u32)
        }
    }

    fn skip(&mut self, count: usize) -> Result<(), String> {
        self.read(count).map(|_| ())
    }

    fn seek(&mut self, offset: u64) -> Result<(), String> {
        self.pos = usize::try_from(offset).map_err(|_| premature())?;
        Ok(())
    }

    fn read_i32s(&mut self, count: u32) -> Result<Vec<i32>, String> {
        let count = usize::try_from(count).map_err(|_| premature())?;
        let bytes = self.read(count.checked_mul(4).ok_or_else(premature)?)?;
        let (words, _) = bytes.as_chunks::<4>();
        Ok(words.iter().map(|word| i32::from_be_bytes(*word)).collect())
    }

    fn read_u32s(&mut self, count: u32) -> Result<Vec<u32>, String> {
        let count = usize::try_from(count).map_err(|_| premature())?;
        let bytes = self.read(count.checked_mul(4).ok_or_else(premature)?)?;
        let (words, _) = bytes.as_chunks::<4>();
        Ok(words.iter().map(|word| u32::from_be_bytes(*word)).collect())
    }
}

fn has_metric_suffix(name: &str) -> bool {
    name.rsplit_once('.').is_some_and(|(_, suffix)| {
        suffix.eq_ignore_ascii_case("tfm") || suffix.eq_ignore_ascii_case("ofm")
    })
}

fn open_steps(name: &str) -> &'static [OpenStep] {
    if has_metric_suffix(name) {
        &[OpenStep::Tfm, OpenStep::Ofm]
    } else {
        &[OpenStep::AliasOfm, OpenStep::Tfm, OpenStep::Ofm]
    }
}

fn char_count(kind: &str, first: u32, last: u32) -> Result<u32, String> {
    if last < first {
        Err(format!("{kind} file error: ec({last}) < bc({first}) ???"))
    } else {
        Ok(last - first + 1)
    }
}

fn design_size(header: &[i32]) -> Result<i32, String> {
    if header.len() < 2 {
        return Err(INVALID_HEADER.to_string());
    }
    if header.len() >= 12 {
        let scheme_len = header[2] >> 24;
        if !(0..=39).contains(&scheme_len) {
            return Err(INVALID_HEADER.to_string());
        }
    }
    Ok(header[1])
}

fn metric_at(table: &[i32], index: usize, message: &str) -> Result<i32, String> {
    table.get(index).copied().ok_or_else(|| message.to_string())
}

fn allocate(slots: usize) -> Result<Vec<i32>, String> {
    let mut values = Vec::new();
    values.try_reserve_exact(slots).map_err(|_| {
        format!(
            "Out of memory - asked for {} bytes\n",
            slots.saturating_mul(4)
        )
    })?;
    values.resize(slots, 0);
    Ok(values)
}

fn check_tfm_size(
    file_size: i64,
    claimed: u32,
    expected: u32,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    if file_size < i64::from(claimed) * 4 {
        return Err(CANNOT_PROCEED.to_string());
    }
    if expected != claimed {
        warnings.push(format!(
            "TFM file size is expected to be {} bytes but it says it is {}bytes!",
            i64::from(expected) * 4,
            i64::from(claimed) * 4
        ));
        if file_size > i64::from(expected) * 4 {
            warnings.push("Proceeding nervously...".to_string());
        } else {
            return Err(CANNOT_PROCEED.to_string());
        }
    }
    Ok(())
}

fn wrapping_sum(terms: &[u32]) -> u32 {
    terms.iter().fold(0, |sum, term| sum.wrapping_add(*term))
}

fn parse_tfm(bytes: &[u8], file_size: i64, warnings: &mut Vec<String>) -> Result<Metrics, String> {
    let mut cursor = Cursor::new(bytes);
    let claimed = cursor.u16()?;
    let header_len = cursor.u16()?;
    let first = cursor.u16()?;
    let last = cursor.u16()?;
    let count = char_count("TFM", first, last)?;
    let widths = cursor.u16()?;
    let heights = cursor.u16()?;
    let depths = cursor.u16()?;
    let italic = cursor.u16()?;
    let ligatures = cursor.u16()?;
    let kerns = cursor.u16()?;
    let extens = cursor.u16()?;
    let params = cursor.u16()?;
    let expected = wrapping_sum(&[
        6, count, header_len, widths, heights, depths, italic, ligatures, kerns, extens, params,
    ]);
    check_tfm_size(file_size, claimed, expected, warnings)?;
    if last > 255 {
        return Err(INVALID_HEADER.to_string());
    }

    let header = cursor.read_i32s(header_len)?;
    let info = cursor.read_u32s(count)?;
    let width_table = cursor.read_i32s(widths)?;
    let height_table = cursor.read_i32s(heights)?;
    let depth_table = cursor.read_i32s(depths)?;
    let design = design_size(&header)?;

    let mut width_out = vec![0; 256];
    let mut height_out = vec![0; 256];
    let mut depth_out = vec![0; 256];
    for (offset, word) in info.iter().enumerate() {
        let code = usize::try_from(first).map_err(|_| premature())? + offset;
        let width_index = (word >> 24) as usize;
        let height_index = ((word >> 20) & 0xf) as usize;
        let depth_index = ((word >> 16) & 0xf) as usize;
        let width = metric_at(&width_table, width_index, INVALID_HEADER)?;
        let height = metric_at(&height_table, height_index, INVALID_HEADER)?;
        let depth = metric_at(&depth_table, depth_index, INVALID_HEADER)?;
        *width_out
            .get_mut(code)
            .ok_or_else(|| INVALID_HEADER.to_string())? = width;
        *height_out
            .get_mut(code)
            .ok_or_else(|| INVALID_HEADER.to_string())? = height;
        *depth_out
            .get_mut(code)
            .ok_or_else(|| INVALID_HEADER.to_string())? = depth;
    }

    Ok(Metrics {
        design_size: design,
        first: i32::try_from(first).map_err(|_| INVALID_HEADER.to_string())?,
        last: i32::try_from(last).map_err(|_| INVALID_HEADER.to_string())?,
        widths: width_out,
        heights: height_out,
        depths: depth_out,
    })
}

fn parse_ofm(bytes: &[u8], file_size: i64, warnings: &mut Vec<String>) -> Result<Metrics, String> {
    let mut cursor = Cursor::new(bytes);
    let level = cursor.i32()?;
    let claimed = cursor.positive("wlenfile")?;
    let header_len = cursor.positive("wlenheader")?;
    let first = cursor.positive("bc")?;
    let last = cursor.positive("ec")?;
    let count = char_count("OFM", first, last)?;
    let nwidths = cursor.positive("nwidths")?;
    let nheights = cursor.positive("nheights")?;
    let ndepths = cursor.positive("ndepths")?;
    let italic = cursor.positive("nitcor")?;
    let ligatures = cursor.positive("nlig")?;
    let kerns = cursor.positive("nkern")?;
    let extens = cursor.positive("nextens")?;
    let params = cursor.positive("nfonparm")?;
    let direction = cursor.positive("fontdir")?;
    if direction != 0 {
        warnings.push("I may be interpreting a font direction incorrectly.".to_string());
    }

    let mut repeats_from = None;
    if level == 0 {
        let expected = wrapping_sum(&[
            14,
            count.wrapping_mul(2),
            header_len,
            nwidths,
            nheights,
            ndepths,
            italic,
            ligatures.wrapping_mul(2),
            kerns,
            extens.wrapping_mul(2),
            params,
        ]);
        if i64::from(claimed) != file_size / 4 || claimed != expected {
            return Err(OFM_PROBLEM.to_string());
        }
    } else if level == 1 {
        let start = cursor.positive("nco")?;
        let packed = cursor.positive("nco")?;
        let parameters = cursor.positive("npc")?;
        cursor.seek(u64::from(start.wrapping_sub(header_len)) * 4)?;
        repeats_from = Some((packed, parameters));
    } else {
        return Err("can't handle OFM files with level > 1".to_string());
    }

    let header = cursor.read_i32s(header_len)?;
    let map = if let Some((packed, parameters)) = repeats_from {
        read_level_one_chars(&mut cursor, count, packed, parameters)?
    } else {
        read_level_zero_chars(&mut cursor, count)?
    };
    let width_table = cursor.read_i32s(nwidths)?;
    let height_table = cursor.read_i32s(nheights)?;
    let depth_table = cursor.read_i32s(ndepths)?;
    assemble_ofm(
        first,
        last,
        &map,
        &[&width_table, &height_table, &depth_table],
        design_size(&header)?,
    )
}

fn zero_indexes(count: u32) -> Result<Vec<usize>, String> {
    let count = usize::try_from(count).map_err(|_| premature())?;
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| OFM_PROBLEM.to_string())?;
    values.resize(count, 0);
    Ok(values)
}

fn read_level_zero_chars(cursor: &mut Cursor<'_>, count: u32) -> Result<CharMap, String> {
    let needed = u64::from(count).checked_mul(8).ok_or_else(premature)?;
    if needed > u64::try_from(cursor.bytes.len().saturating_sub(cursor.pos)).unwrap_or(0) {
        return Err(premature());
    }
    let slots = usize::try_from(count).map_err(|_| premature())?;
    let mut map = CharMap {
        width: Vec::new(),
        height: Vec::new(),
        depth: Vec::new(),
    };
    map.width
        .try_reserve_exact(slots)
        .map_err(|_| OFM_PROBLEM.to_string())?;
    map.height
        .try_reserve_exact(slots)
        .map_err(|_| OFM_PROBLEM.to_string())?;
    map.depth
        .try_reserve_exact(slots)
        .map_err(|_| OFM_PROBLEM.to_string())?;
    for _ in 0..count {
        map.width
            .push(usize::try_from(cursor.u16()?).map_err(|_| premature())?);
        map.height.push(usize::from(cursor.u8()?));
        map.depth.push(usize::from(cursor.u8()?));
        cursor.skip(4)?;
    }
    Ok(map)
}

fn read_level_one_chars(
    cursor: &mut Cursor<'_>,
    count: u32,
    packed: u32,
    parameters: u32,
) -> Result<CharMap, String> {
    let mut map = CharMap {
        width: zero_indexes(count)?,
        height: zero_indexes(count)?,
        depth: zero_indexes(count)?,
    };
    let infos = u64::from(packed) / (3 + u64::from(parameters) / 2);
    let mut index = 0u32;
    let mut read = 0u64;
    while index < count && read < infos {
        let width = usize::try_from(cursor.u16()?).map_err(|_| premature())?;
        let height = usize::from(cursor.u8()?);
        let depth = usize::from(cursor.u8()?);
        cursor.skip(4)?;
        let repeats = cursor.u16()?;
        let pending =
            u64::from(parameters) * 2 + u64::from(u32::from(parameters.is_multiple_of(2))) * 2;
        if pending > u64::try_from(cursor.bytes.len().saturating_sub(cursor.pos)).unwrap_or(0) {
            return Err(premature());
        }
        for _ in 0..parameters {
            cursor.u16()?;
        }
        if parameters.is_multiple_of(2) {
            cursor.u16()?;
        }
        // The native loop writes one past the table when `repeats` lands on
        // the character count. Reject that instead of overflowing.
        if u64::from(index) + u64::from(repeats) >= u64::from(count) {
            return Err("OFM \"repeats\" causes number of characters to be exceeded.".to_string());
        }
        let slot = usize::try_from(index).map_err(|_| premature())?;
        map.width[slot] = width;
        map.height[slot] = height;
        map.depth[slot] = depth;
        for follow in 1..=repeats {
            let copied = slot + usize::try_from(follow).map_err(|_| premature())?;
            map.width[copied] = width;
            map.height[copied] = height;
            map.depth[copied] = depth;
        }
        index += repeats + 1;
        read += 1;
    }
    Ok(map)
}

fn assemble_ofm(
    first: u32,
    last: u32,
    map: &CharMap,
    tables: &[&[i32]; 3],
    design: i32,
) -> Result<Metrics, String> {
    let slots = usize::try_from(last)
        .ok()
        .and_then(|last| last.checked_add(1))
        .ok_or_else(|| OFM_PROBLEM.to_string())?;
    let mut width_out = allocate(slots)?;
    let mut height_out = allocate(slots)?;
    let mut depth_out = allocate(slots)?;
    let [widths, heights, depths] = tables;
    let base = usize::try_from(first).map_err(|_| OFM_PROBLEM.to_string())?;
    for index in 0..map.width.len() {
        let code = base
            .checked_add(index)
            .ok_or_else(|| OFM_PROBLEM.to_string())?;
        let width = metric_at(widths, map.width[index], OFM_PROBLEM)?;
        let height = metric_at(heights, map.height[index], OFM_PROBLEM)?;
        let depth = metric_at(depths, map.depth[index], OFM_PROBLEM)?;
        *width_out
            .get_mut(code)
            .ok_or_else(|| OFM_PROBLEM.to_string())? = width;
        *height_out
            .get_mut(code)
            .ok_or_else(|| OFM_PROBLEM.to_string())? = height;
        *depth_out
            .get_mut(code)
            .ok_or_else(|| OFM_PROBLEM.to_string())? = depth;
    }
    Ok(Metrics {
        design_size: design,
        first: i32::try_from(first).map_err(|_| OFM_PROBLEM.to_string())?,
        last: i32::try_from(last).map_err(|_| OFM_PROBLEM.to_string())?,
        widths: width_out,
        heights: height_out,
        depths: depth_out,
    })
}

fn parse(
    source: Source,
    bytes: &[u8],
    file_size: i64,
    warnings: &mut Vec<String>,
) -> Result<Metrics, String> {
    match source {
        Source::Tfm => parse_tfm(bytes, file_size, warnings),
        Source::Ofm => parse_ofm(bytes, file_size, warnings),
    }
}

fn clear_fonts() {
    with_fonts(|fonts| {
        let mut fonts = fonts.borrow_mut();
        fonts.names.clear();
        fonts.metrics.clear();
    });
}

fn clear_diagnostics() {
    with_diagnostics(|queue| queue.borrow_mut().clear());
    with_held_diagnostic(|held| *held.borrow_mut() = None);
}

fn push_diagnostic(kind: i32, text: &str) {
    let Ok(text) = CString::new(text) else {
        return;
    };
    with_diagnostics(|queue| queue.borrow_mut().push_back(Diagnostic { kind, text }));
}

fn publish(error: *mut *const c_char, message: Option<&str>) {
    if error.is_null() {
        return;
    }
    let pointer = match message {
        None => {
            with_last_error(|slot| *slot.borrow_mut() = None);
            ptr::null()
        }
        Some(message) => {
            let text = CString::new(message).unwrap_or_else(|_| c"Invalid TFM header.".to_owned());
            with_last_error(|slot| {
                let mut slot = slot.borrow_mut();
                *slot = Some(text);
                slot.as_ref().expect("error stored").as_ptr()
            })
        }
    };
    unsafe { *error = pointer };
}

fn finish<T: Default>(result: Result<T, String>, error: *mut *const c_char) -> T {
    match result {
        Ok(value) => {
            publish(error, None);
            value
        }
        Err(message) => {
            publish(error, Some(&message));
            T::default()
        }
    }
}

fn find_font(name: &[u8]) -> Option<i32> {
    with_fonts(|fonts| fonts.borrow().names.get(name).copied())
}

fn intern(name: &[u8], metrics: Metrics) -> i32 {
    with_fonts(|fonts| {
        let mut fonts = fonts.borrow_mut();
        if let Some(&id) = fonts.names.get(name) {
            return id;
        }
        let id = i32::try_from(fonts.metrics.len()).unwrap_or(-1);
        if id >= 0 {
            fonts.metrics.push(metrics);
            fonts.names.insert(Box::from(name), id);
        }
        id
    })
}

fn metric_value(metrics: &Metrics, ch: i32, values: &[i32]) -> Result<i32, String> {
    if ch < metrics.first || ch > metrics.last {
        return Err(format!("Invalid char: {ch}\n"));
    }
    values
        .get(usize::try_from(ch).unwrap_or(usize::MAX))
        .copied()
        .ok_or_else(|| format!("Invalid char: {ch}\n"))
}

fn with_metrics<T>(
    id: i32,
    select: impl FnOnce(&Metrics) -> Result<T, String>,
) -> Result<T, String> {
    with_fonts(|fonts| {
        let fonts = fonts.borrow();
        let font = usize::try_from(id)
            .ok()
            .and_then(|index| fonts.metrics.get(index))
            .ok_or_else(|| format!("TFM: Invalid TFM ID: {id}"))?;
        select(font)
    })
}

struct CloseInput(InputId);

impl Drop for CloseInput {
    fn drop(&mut self) {
        CoreBridgeState::with_global_state(|state| {
            state.input_close(self.0);
        });
    }
}

fn read_file(name: &str, format: FileFormat) -> Result<Option<(Vec<u8>, u64)>, String> {
    let id = CoreBridgeState::with_global_state(|state| state.input_open(name, format, false));
    let Some(id) = id else {
        return Ok(None);
    };
    let _close = CloseInput(id);
    CoreBridgeState::with_global_state(|state| {
        let size = u64::try_from(state.input_get_size(id)).unwrap_or(u64::MAX);
        let mut bytes = Vec::new();
        state
            .get_input(id)
            .read_to_end(&mut bytes)
            .map_err(|_| premature())?;
        Ok(Some((bytes, size)))
    })
}

fn file_available(name: &str, format: FileFormat) -> bool {
    CoreBridgeState::with_global_state(|state| {
        let Some(id) = state.input_open(name, format, false) else {
            return false;
        };
        state.input_close(id);
        true
    })
}

fn locate(requested: &str, verbose: bool) -> Result<Option<(Source, Vec<u8>, i64)>, String> {
    for step in open_steps(requested) {
        let (path, source, format) = match step {
            OpenStep::AliasOfm => (
                Cow::Owned(format!("{requested}.ofm")),
                Source::Ofm,
                FileFormat::Ofm,
            ),
            OpenStep::Tfm => (Cow::Borrowed(requested), Source::Tfm, FileFormat::Tfm),
            OpenStep::Ofm => (Cow::Borrowed(requested), Source::Ofm, FileFormat::Ofm),
        };
        if let Some((bytes, size)) = read_file(&path, format)? {
            if verbose {
                let label = match source {
                    Source::Tfm => "TFM",
                    Source::Ofm => "OFM",
                };
                push_diagnostic(MESSAGE, &format!("({label}:{requested}"));
            }
            if size > 0x1_FFFF_FFFF {
                return Err("TFM/OFM file size exceeds 33-bit".to_string());
            }
            if size < 24 {
                return Err("TFM/OFM file too small to be a valid file.".to_string());
            }
            return Ok(Some((
                source,
                bytes,
                i64::try_from(size).unwrap_or(i64::MAX),
            )));
        }
    }
    Ok(None)
}

fn open_named(name: &CStr, must_exist: bool, verbose: bool) -> Result<i32, String> {
    let key = name.to_bytes();
    if let Some(id) = find_font(key) {
        return Ok(id);
    }
    let requested = name.to_string_lossy();
    let Some((source, bytes, size)) = locate(&requested, verbose)? else {
        if must_exist {
            return Err(format!("Unable to find TFM file \"{requested}\"."));
        }
        return Ok(-1);
    };
    let mut warnings = Vec::new();
    let parsed = parse(source, &bytes, size, &mut warnings);
    for warning in &warnings {
        push_diagnostic(WARNING, warning);
    }
    let metrics = parsed?;
    if verbose {
        push_diagnostic(MESSAGE, ")");
    }
    Ok(intern(key, metrics))
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_tfm_next_diagnostic(kind: *mut i32) -> *const c_char {
    let Some(diagnostic) = with_diagnostics(|queue| queue.borrow_mut().pop_front()) else {
        return ptr::null();
    };
    if !kind.is_null() {
        unsafe { *kind = diagnostic.kind };
    }
    with_held_diagnostic(|held| {
        let mut held = held.borrow_mut();
        *held = Some(diagnostic.text);
        held.as_ref().expect("diagnostic stored").as_ptr()
    })
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_tfm_open(
    name: *const c_char,
    must_exist: i32,
    verbose: i32,
    error: *mut *const c_char,
) -> i32 {
    clear_diagnostics();
    if name.is_null() {
        let result = if must_exist != 0 {
            Err("Unable to find TFM file \"\".".to_string())
        } else {
            Ok(-1)
        };
        return finish(result, error);
    }
    let name = unsafe { CStr::from_ptr(name) };
    finish(open_named(name, must_exist != 0, verbose > 0), error)
}

#[unsafe(no_mangle)]
extern "C" fn tt_tfm_close_all() {
    clear_fonts();
}

#[unsafe(no_mangle)]
extern "C" fn tt_tfm_reset() {
    clear_fonts();
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_tfm_get_fw_width(font_id: i32, ch: i32, error: *mut *const c_char) -> i32 {
    finish(
        with_metrics(font_id, |metrics| {
            metric_value(metrics, ch, &metrics.widths)
        }),
        error,
    )
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_tfm_get_fw_height(font_id: i32, ch: i32, error: *mut *const c_char) -> i32 {
    finish(
        with_metrics(font_id, |metrics| {
            metric_value(metrics, ch, &metrics.heights)
        }),
        error,
    )
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_tfm_get_fw_depth(font_id: i32, ch: i32, error: *mut *const c_char) -> i32 {
    finish(
        with_metrics(font_id, |metrics| {
            metric_value(metrics, ch, &metrics.depths)
        }),
        error,
    )
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_tfm_get_width(font_id: i32, ch: i32, error: *mut *const c_char) -> f64 {
    finish(
        with_metrics(font_id, |metrics| {
            Ok(f64::from(metric_value(metrics, ch, &metrics.widths)?) / FIXWORD_SCALE)
        }),
        error,
    )
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_tfm_string_width(
    font_id: i32,
    text: *const u8,
    length: u32,
    error: *mut *const c_char,
) -> i32 {
    let bytes = if text.is_null() || length == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(text, length as usize) }
    };
    finish(
        with_metrics(font_id, |metrics| {
            let mut total = 0i32;
            for &byte in bytes {
                total =
                    total.wrapping_add(metric_value(metrics, i32::from(byte), &metrics.widths)?);
            }
            Ok(total)
        }),
        error,
    )
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_tfm_get_design_size(font_id: i32, error: *mut *const c_char) -> f64 {
    finish(
        with_metrics(font_id, |metrics| {
            Ok(f64::from(metrics.design_size) / FIXWORD_SCALE * (72.0 / 72.27))
        }),
        error,
    )
}

#[unsafe(no_mangle)]
extern "C" fn tt_tfm_is_jfm(_font_id: i32) -> i32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn tt_tfm_exists(name: *const c_char) -> bool {
    if name.is_null() {
        return false;
    }
    let name = unsafe { CStr::from_ptr(name) }.to_string_lossy();
    file_available(&name, FileFormat::Ofm) || file_available(&name, FileFormat::Tfm)
}

fn with_fonts<T>(callback: impl FnOnce(&RefCell<Fonts>) -> T) -> T {
    crate::compat::document::with_tfm(|state| callback(&state.fonts))
}

fn with_diagnostics<T>(callback: impl FnOnce(&RefCell<VecDeque<Diagnostic>>) -> T) -> T {
    crate::compat::document::with_tfm(|state| callback(&state.diagnostics))
}

fn with_held_diagnostic<T>(callback: impl FnOnce(&RefCell<Option<CString>>) -> T) -> T {
    crate::compat::document::with_tfm(|state| callback(&state.held_diagnostic))
}

fn with_last_error<T>(callback: impl FnOnce(&RefCell<Option<CString>>) -> T) -> T {
    crate::compat::document::with_tfm(|state| callback(&state.last_error))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_pair(bytes: &mut Vec<u8>, left: u16, right: u16) {
        bytes.extend_from_slice(&left.to_be_bytes());
        bytes.extend_from_slice(&right.to_be_bytes());
    }

    fn push_i32(bytes: &mut Vec<u8>, value: i32) {
        bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn minimal_tfm(design: i32, width: i32, height: i32, depth: i32) -> Vec<u8> {
        let mut bytes = Vec::new();
        push_pair(&mut bytes, 15, 2);
        push_pair(&mut bytes, 0, 0);
        push_pair(&mut bytes, 2, 2);
        push_pair(&mut bytes, 2, 0);
        push_pair(&mut bytes, 0, 0);
        push_pair(&mut bytes, 0, 0);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, design);
        bytes.extend_from_slice(&[1, 0x11, 0, 0]);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, width);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, height);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, depth);
        bytes
    }

    fn parse_case(
        source: Source,
        bytes: &[u8],
        file_size: i64,
    ) -> (Result<Metrics, String>, Vec<String>) {
        let mut warnings = Vec::new();
        let result = parse(source, bytes, file_size, &mut warnings);
        (result, warnings)
    }

    struct Isolate;

    impl Isolate {
        fn new() -> Self {
            clear_fonts();
            clear_diagnostics();
            Self
        }
    }

    impl Drop for Isolate {
        fn drop(&mut self) {
            clear_fonts();
            clear_diagnostics();
        }
    }

    fn reported(error: *const c_char) -> Option<String> {
        if error.is_null() {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(error) }
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }

    #[test]
    fn font_cache_preserves_ids_and_first_metrics_across_growth_and_reset() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let make_metrics = |width| Metrics {
            design_size: 10,
            first: 0,
            last: 0,
            widths: vec![width],
            heights: vec![0],
            depths: vec![0],
        };
        for index in 0..2048 {
            let name = format!("font{index}");
            assert_eq!(intern(name.as_bytes(), make_metrics(index)), index);
        }
        for index in 0..2048 {
            let name = format!("font{index}");
            assert_eq!(find_font(name.as_bytes()), Some(index));
            assert_eq!(intern(name.as_bytes(), make_metrics(-1)), index);
            assert_eq!(
                with_metrics(index, |metrics| Ok(metrics.widths[0])).unwrap(),
                index
            );
        }
        clear_fonts();
        assert_eq!(find_font(b"font0"), None);
        assert_eq!(intern(b"font0", make_metrics(42)), 0);
    }

    #[test]
    fn names_without_a_metric_suffix_try_an_ofm_alias_first() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        assert_eq!(
            open_steps("cmr10"),
            &[OpenStep::AliasOfm, OpenStep::Tfm, OpenStep::Ofm]
        );
        assert_eq!(open_steps("cmr10.tfm"), &[OpenStep::Tfm, OpenStep::Ofm]);
        assert_eq!(open_steps("cmr10.OFM"), &[OpenStep::Tfm, OpenStep::Ofm]);
        assert_eq!(
            open_steps("cmr10.tfm.bak"),
            &[OpenStep::AliasOfm, OpenStep::Tfm, OpenStep::Ofm]
        );
        assert!(!has_metric_suffix("cmr10."));
    }

    #[test]
    fn tfm_metrics_scale_from_fixwords_and_reject_bad_characters() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let _isolate = Isolate::new();
        let design = 10 << 20;
        let bytes = minimal_tfm(design, 524_288, 65_536, 32_768);
        let (metrics, warnings) = parse_case(Source::Tfm, &bytes, bytes.len() as i64);
        let metrics = metrics.expect("minimal tfm");
        assert!(warnings.is_empty());
        let id = intern(b"cmr10", metrics);
        let mut error = ptr::null();
        unsafe {
            assert_eq!(tt_tfm_get_fw_width(id, 0, &mut error), 524_288);
            assert!(error.is_null());
            assert_eq!(tt_tfm_get_fw_height(id, 0, &mut error), 65_536);
            assert_eq!(tt_tfm_get_fw_depth(id, 0, &mut error), 32_768);
            assert_eq!(
                tt_tfm_get_width(id, 0, &mut error),
                524_288.0 / FIXWORD_SCALE
            );
            assert_eq!(
                tt_tfm_get_design_size(id, &mut error),
                f64::from(design) / FIXWORD_SCALE * (72.0 / 72.27)
            );
            let text = [0u8, 0];
            assert_eq!(
                tt_tfm_string_width(id, text.as_ptr(), 2, &mut error),
                1_048_576
            );
            assert_eq!(tt_tfm_get_fw_width(id, 1, &mut error), 0);
            assert_eq!(reported(error).as_deref(), Some("Invalid char: 1\n"));
            assert_eq!(tt_tfm_get_fw_width(4, 0, &mut error), 0);
            assert_eq!(reported(error).as_deref(), Some("TFM: Invalid TFM ID: 4"));
            assert_eq!(tt_tfm_is_jfm(id), 0);
            assert_eq!(tt_tfm_is_jfm(-1), 0);
            assert_eq!(find_font(b"cmr10"), Some(id));
            tt_tfm_close_all();
            assert!(find_font(b"cmr10").is_none());
            assert_eq!(tt_tfm_get_design_size(id, &mut error), 0.0);
            assert_eq!(reported(error).as_deref(), Some("TFM: Invalid TFM ID: 0"));
        }
    }

    #[test]
    fn tfm_size_mismatches_warn_or_stop_before_reading_tables() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let mut bytes = Vec::new();
        push_pair(&mut bytes, 20, 2);
        push_pair(&mut bytes, 0, 0);
        push_pair(&mut bytes, 1, 1);
        push_pair(&mut bytes, 1, 0);
        push_pair(&mut bytes, 0, 0);
        push_pair(&mut bytes, 0, 0);
        let (result, warnings) = parse_case(Source::Tfm, &bytes, 60);
        assert_eq!(result.unwrap_err(), CANNOT_PROCEED);
        assert!(warnings.is_empty());

        let mut padded = minimal_tfm(10 << 20, 1, 1, 1);
        padded.extend_from_slice(&[0, 0, 0, 0]);
        padded[0] = 0;
        padded[1] = 16;
        let (result, warnings) = parse_case(Source::Tfm, &padded, padded.len() as i64);
        assert!(result.is_ok());
        assert_eq!(
            warnings,
            [
                "TFM file size is expected to be 60 bytes but it says it is 64bytes!",
                "Proceeding nervously..."
            ]
        );
    }

    #[test]
    fn tfm_rejects_inverted_ranges_short_headers_and_bad_indexes() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let mut bytes = Vec::new();
        push_pair(&mut bytes, 15, 2);
        push_pair(&mut bytes, 3, 1);
        let (result, _) = parse_case(Source::Tfm, &bytes, 60);
        assert_eq!(result.unwrap_err(), "TFM file error: ec(1) < bc(3) ???");

        let mut header = Vec::new();
        push_pair(&mut header, 11, 1);
        push_pair(&mut header, 0, 0);
        push_pair(&mut header, 1, 1);
        push_pair(&mut header, 1, 0);
        push_pair(&mut header, 0, 0);
        push_pair(&mut header, 0, 0);
        push_i32(&mut header, 0);
        header.extend_from_slice(&[0, 0, 0, 0]);
        push_i32(&mut header, 0);
        push_i32(&mut header, 0);
        push_i32(&mut header, 0);
        let (result, _) = parse_case(Source::Tfm, &header, header.len() as i64);
        assert_eq!(result.unwrap_err(), INVALID_HEADER);

        let mut bad = Vec::new();
        push_pair(&mut bad, 12, 2);
        push_pair(&mut bad, 0, 0);
        push_pair(&mut bad, 1, 1);
        push_pair(&mut bad, 1, 0);
        push_pair(&mut bad, 0, 0);
        push_pair(&mut bad, 0, 0);
        push_i32(&mut bad, 0);
        push_i32(&mut bad, 1 << 20);
        bad.extend_from_slice(&[5, 0, 0, 0]);
        push_i32(&mut bad, 0);
        push_i32(&mut bad, 0);
        push_i32(&mut bad, 0);
        let (result, _) = parse_case(Source::Tfm, &bad, bad.len() as i64);
        assert_eq!(result.unwrap_err(), INVALID_HEADER);
    }

    #[test]
    fn coding_scheme_length_is_validated_when_the_header_is_long_enough() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let mut bytes = Vec::new();
        push_pair(&mut bytes, 22, 12);
        push_pair(&mut bytes, 0, 0);
        push_pair(&mut bytes, 1, 1);
        push_pair(&mut bytes, 1, 0);
        push_pair(&mut bytes, 0, 0);
        push_pair(&mut bytes, 0, 0);
        for word in 0..12 {
            let value = if word == 2 { 40 << 24 } else { 0 };
            push_i32(&mut bytes, value);
        }
        bytes.extend_from_slice(&[0, 0, 0, 0]);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, 0);
        let (result, _) = parse_case(Source::Tfm, &bytes, bytes.len() as i64);
        assert_eq!(result.unwrap_err(), INVALID_HEADER);

        bytes[32] = 39;
        let (result, _) = parse_case(Source::Tfm, &bytes, bytes.len() as i64);
        assert!(result.is_ok());
    }

    fn push_ofm_directory(bytes: &mut Vec<u8>, words: &[i32]) {
        for word in words {
            push_i32(bytes, *word);
        }
    }

    #[test]
    fn ofm_level_zero_reads_split_char_info_and_warns_about_direction() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let mut bytes = Vec::new();
        push_ofm_directory(&mut bytes, &[0, 24, 2, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 1]);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, 8 << 20);
        bytes.extend_from_slice(&[0, 1, 1, 1, 0, 0, 0, 0]);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, 4096);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, 2048);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, 1024);
        assert_eq!(bytes.len(), 96);
        let (metrics, warnings) = parse_case(Source::Ofm, &bytes, 96);
        let metrics = metrics.expect("level 0 ofm");
        assert_eq!(
            warnings,
            ["I may be interpreting a font direction incorrectly."]
        );
        assert_eq!(metrics.widths[0], 4096);
        assert_eq!(metrics.heights[0], 2048);
        assert_eq!(metrics.depths[0], 1024);
        assert_eq!(metrics.design_size, 8 << 20);
    }

    #[test]
    fn ofm_level_one_repeats_copy_metrics_and_reject_overruns() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let mut bytes = Vec::new();
        push_ofm_directory(
            &mut bytes,
            &[1, 0, 2, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 19, 3, 0],
        );
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, 5 << 20);
        bytes.extend_from_slice(&[0, 1, 1, 1, 0, 0, 0, 0, 0, 2, 0, 0]);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, 111);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, 222);
        push_i32(&mut bytes, 0);
        push_i32(&mut bytes, 333);
        let (metrics, warnings) = parse_case(Source::Ofm, &bytes, bytes.len() as i64);
        let metrics = metrics.expect("level 1 ofm");
        assert!(warnings.is_empty());
        assert_eq!(metrics.first, 0);
        assert_eq!(metrics.last, 2);
        assert_eq!(metrics.widths, vec![111, 111, 111]);
        assert_eq!(metrics.heights, vec![222, 222, 222]);
        assert_eq!(metrics.depths, vec![333, 333, 333]);

        bytes[19 * 4 + 8] = 0;
        bytes[19 * 4 + 9] = 3;
        let (result, _) = parse_case(Source::Ofm, &bytes, bytes.len() as i64);
        assert_eq!(
            result.unwrap_err(),
            "OFM \"repeats\" causes number of characters to be exceeded."
        );
    }

    #[test]
    fn ofm_reports_negative_fields_bad_ranges_and_unsupported_levels() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let mut negative = vec![0u8; 8];
        negative[0..4].copy_from_slice(&0i32.to_be_bytes());
        negative[4..8].copy_from_slice(&(-5i32).to_be_bytes());
        let (result, _) = parse_case(Source::Ofm, &negative, 64);
        assert_eq!(result.unwrap_err(), "Bad OFM: negative wlenfile: -5");

        let mut inverted = Vec::new();
        push_ofm_directory(&mut inverted, &[0, 24, 2, 4, 1]);
        let (result, _) = parse_case(Source::Ofm, &inverted, 96);
        assert_eq!(result.unwrap_err(), "OFM file error: ec(1) < bc(4) ???");

        let mut level = vec![0u8; 56];
        level[0..4].copy_from_slice(&2i32.to_be_bytes());
        let (result, warnings) = parse_case(Source::Ofm, &level, 56);
        assert_eq!(result.unwrap_err(), "can't handle OFM files with level > 1");
        assert!(warnings.is_empty());

        level[13 * 4..14 * 4].copy_from_slice(&1i32.to_be_bytes());
        let (result, warnings) = parse_case(Source::Ofm, &level, 56);
        assert_eq!(result.unwrap_err(), "can't handle OFM files with level > 1");
        assert_eq!(
            warnings,
            ["I may be interpreting a font direction incorrectly."]
        );
    }

    #[test]
    fn diagnostics_come_out_in_submission_order() {
        let _document = crate::compat::document::DocumentScope::new().unwrap();
        let _isolate = Isolate::new();
        push_diagnostic(MESSAGE, "(TFM:cmr10");
        push_diagnostic(WARNING, "Proceeding nervously...");
        let mut kind = -1;
        let first = unsafe { tt_tfm_next_diagnostic(&mut kind) };
        assert_eq!(kind, MESSAGE);
        assert_eq!(unsafe { CStr::from_ptr(first) }.to_bytes(), b"(TFM:cmr10");
        let second = unsafe { tt_tfm_next_diagnostic(&mut kind) };
        assert_eq!(kind, WARNING);
        assert_eq!(
            unsafe { CStr::from_ptr(second) }.to_bytes(),
            b"Proceeding nervously..."
        );
        assert!(unsafe { tt_tfm_next_diagnostic(&mut kind) }.is_null());
    }
}
