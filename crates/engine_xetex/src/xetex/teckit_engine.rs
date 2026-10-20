#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use super::teckit_c_engine::*;
use super::teckit_common::*;
use super::teckit_format::*;
use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::{canonical_combining_class, compose, decompose_canonical};

pub const kNeedMoreInput: UInt32 = 0xfffffffe;
pub const kInvalidChar: UInt32 = 0xfffffffd;
pub const kUnmappedChar: UInt32 = 0xfffffffc;

pub const matchYes: UInt32 = 1;
pub const matchNo: UInt32 = 0;

pub static offsetsFromUTF8: [UInt32; 6] = [0x00000000, 0x00003080, 0x000E2080, 0x03C82080, 0xFA082080, 0x82082080];

pub static bytesFromUTF8: [UInt8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5,
];

pub static firstByteMark: [UInt8; 7] = [0x00, 0x00, 0xC0, 0xE0, 0xF0, 0xF8, 0xFC];

pub const halfShift: i32 = 10;
pub const halfBase: UInt32 = 0x0010000;
pub const halfMask: UInt32 = 0x3FF;
pub const kSurrogateHighStart: UInt32 = 0xD800;
pub const kSurrogateHighEnd: UInt32 = 0xDBFF;
pub const kSurrogateLowStart: UInt32 = 0xDC00;
pub const byteMask: UInt32 = 0x000000BF;
pub const byteMark: UInt32 = 0x00000080;

#[inline(always)]
pub fn READ_u8(p: UInt8) -> UInt8 {
    p
}

#[inline(always)]
pub fn READ_u16(p: UInt16) -> UInt16 {
    UInt16::from_be(p)
}

#[inline(always)]
pub fn READ_u32(p: UInt32) -> UInt32 {
    UInt32::from_be(p)
}

pub trait Stage {
    unsafe fn getChar(&mut self) -> UInt32;
    unsafe fn Reset(&mut self);
    unsafe fn lookaheadCount(&self) -> UInt32 {
        0
    }
    unsafe fn getPrevStage(&self) -> *mut dyn Stage;
    unsafe fn setPrevStage(&mut self, prev: *mut dyn Stage);
}

pub struct Normalizer {
    pending: String,
    output: std::collections::VecDeque<UInt32>,
    pub prevStage: *mut dyn Stage,
    pub bCompose: bool,
}

impl Normalizer {
    pub fn new(compose: bool) -> Self {
        Self {
            pending: String::new(),
            output: std::collections::VecDeque::new(),
            prevStage: std::ptr::null_mut::<Converter>() as *mut dyn Stage,
            bCompose: compose,
        }
    }

    fn flush_pending(&mut self) {
        if self.bCompose {
            self.output.extend(self.pending.nfc().map(u32::from));
        } else {
            self.output.extend(self.pending.nfd().map(u32::from));
        }
        self.pending.clear();
    }

    fn push_decomposed(&mut self, c: char) {
        if canonical_combining_class(c) == 0 && !self.pending.is_empty() {
            if self.bCompose {
                let mut normalized = self.pending.nfc();
                let first = normalized.next().unwrap();
                let second = normalized.next();
                if second.is_none()
                    && canonical_combining_class(first) == 0
                    && let Some(composed) = compose(first, c)
                {
                    self.pending.clear();
                    self.pending.push(composed);
                    return;
                }
                self.output.push_back(u32::from(first));
                self.output.extend(second.map(u32::from));
                self.output.extend(normalized.map(u32::from));
                self.pending.clear();
            } else {
                self.flush_pending();
            }
        }
        self.pending.push(c);
    }
}

impl Stage for Normalizer {
    unsafe fn getChar(&mut self) -> UInt32 {
        loop {
            if let Some(c) = self.output.pop_front() {
                return c;
            }
            let c = unsafe { (*self.prevStage).getChar() };
            match c {
                kNeedMoreInput | kInvalidChar | kUnmappedChar => return c,
                kEndOfText => {
                    self.flush_pending();
                    self.output.push_back(kEndOfText);
                }
                _ => {
                    let Some(c) = char::from_u32(c) else {
                        return kInvalidChar;
                    };
                    decompose_canonical(c, |part| self.push_decomposed(part));
                }
            }
        }
    }

    unsafe fn Reset(&mut self) {
        self.pending.clear();
        self.output.clear();
    }

    unsafe fn getPrevStage(&self) -> *mut dyn Stage {
        self.prevStage
    }

    unsafe fn setPrevStage(&mut self, prev: *mut dyn Stage) {
        self.prevStage = prev;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_search_returns_first_equal_item_or_insertion_position() {
        let values = [0u32, 1, 1, 127, 128, 255];
        let words = values.map(|value| (value as u16).to_be());
        let dwords = values.map(u32::to_be);
        let bytes = values.map(|value| value as u8);
        for count in 0..=values.len() {
            for value in 0..=256 {
                let expected = values[..count].iter().position(|&item| item >= value).unwrap_or(count);
                unsafe {
                    assert_eq!(binary_search_u32(dwords.as_ptr(), count as u32, value), dwords.as_ptr().add(expected));
                    assert_eq!(binary_search_u16(words.as_ptr(), count as u32, value), words.as_ptr().add(expected));
                    assert_eq!(binary_search_u8(bytes.as_ptr(), count as u32, value), bytes.as_ptr().add(expected));
                }
            }
        }
        unsafe {
            assert!(binary_search_u32(std::ptr::null(), 0, 1).is_null());
            assert!(binary_search_u16(std::ptr::null(), 0, 1).is_null());
            assert!(binary_search_u8(std::ptr::null(), 0, 1).is_null());
        }
    }

    #[test]
    fn normalization_matches_unicode_iterator_for_starters_and_combining_sequences() {
        let mut input = String::from("\u{301}\u{315}\u{300}");
        for code in 0..=0x10ffff {
            if let Some(c) = char::from_u32(code) {
                input.push(c);
                if code % 127 == 0 {
                    input.push_str("a\u{315}\u{300}\u{1100}\u{1161}\u{11a8}\u{9c7}\u{9be}");
                }
            }
        }
        for compose in [false, true] {
            let mut normalizer = Normalizer::new(compose);
            for c in input.chars() {
                decompose_canonical(c, |part| normalizer.push_decomposed(part));
            }
            normalizer.flush_pending();
            let actual: String = normalizer.output.into_iter().map(|c| char::from_u32(c).unwrap()).collect();
            let expected: String = if compose { input.nfc().collect() } else { input.nfd().collect() };
            assert_eq!(actual, expected, "compose {compose}");
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MatchedSpan {
    pub start: UInt16,
    pub limit: UInt16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MatchInfo {
    pub classIndex: UInt32,
    pub groupRepeats: libc::c_int,
    pub matchedSpan: MatchedSpan,
}

#[repr(C)]
pub struct sgrStackItem {
    pub link: *mut sgrStackItem,
    pub savedGroupRepeats: libc::c_int,
}

pub struct Pass {
    pub oBuffer: *mut UInt32,
    pub oBufSize: libc::c_long,
    pub oBufEnd: libc::c_long,
    pub oBufPtr: libc::c_long,
    pub prevStage: *mut dyn Stage,

    pub pattern: *const MatchElem,
    pub patternLength: libc::c_int,
    pub direction: libc::c_int,
    pub info: [MatchInfo; 256],
    pub infoLimit: libc::c_int,
    pub matchElems: libc::c_int,
    pub matchedLength: libc::c_int,

    pub groupRepeats: libc::c_int,
    pub sgrStack: *mut sgrStackItem,

    pub converter: *mut Converter,
    pub tableHeader: *const TableHeader,

    pub pageBase: *const Byte,
    pub lookupBase: *const Lookup,
    pub matchClassBase: *const Byte,
    pub repClassBase: *const Byte,
    pub stringListBase: *const Byte,
    pub stringRuleData: *const Byte,
    pub planeMap: *const Byte,

    pub iBuffer: *mut UInt32,
    pub iBufSize: libc::c_long,
    pub iBufStart: libc::c_long,
    pub iBufEnd: libc::c_long,
    pub iBufPtr: libc::c_long,

    pub bInputIsUnicode: bool,
    pub bOutputIsUnicode: bool,
    pub bSupplementaryChars: bool,
    pub numPageMaps: UInt8,
}

unsafe fn binary_search_u32(array: *const UInt32, count: UInt32, value: UInt32) -> *const UInt32 {
    unsafe {
        if count == 0 {
            return array;
        }
        let items = std::slice::from_raw_parts(array, count as usize);
        array.add(items.partition_point(|&item| READ_u32(item) < value))
    }
}

unsafe fn binary_search_u16(array: *const UInt16, count: UInt32, value: UInt32) -> *const UInt16 {
    unsafe {
        if count == 0 {
            return array;
        }
        let items = std::slice::from_raw_parts(array, count as usize);
        array.add(items.partition_point(|&item| (READ_u16(item) as UInt32) < value))
    }
}

unsafe fn binary_search_u8(array: *const UInt8, count: UInt32, value: UInt32) -> *const UInt8 {
    unsafe {
        if count == 0 {
            return array;
        }
        let items = std::slice::from_raw_parts(array, count as usize);
        array.add(items.partition_point(|&item| (item as UInt32) < value))
    }
}

impl Pass {
    pub unsafe fn new(inTable: *const TableHeader, cnv: *mut Converter) -> Self {
        unsafe {
            let bInputIsUnicode = ((READ_u32((*inTable).type_) & 0xFF000000) >> 24) == b'U' as u32;
            let bOutputIsUnicode = (READ_u32((*inTable).type_) & 0x000000FF) == b'U' as u32;
            let bSupplementaryChars = (READ_u32((*inTable).flags) & kTableFlags_Supplementary) != 0;

            let mut numPageMaps: UInt8 = 1;
            let table_bytes = inTable as *const Byte;
            let mut pageBase = table_bytes.add(READ_u32((*inTable).pageBase) as usize);
            let lookupBase = table_bytes.add(READ_u32((*inTable).lookupBase) as usize) as *const Lookup;
            let matchClassBase = table_bytes.add(READ_u32((*inTable).matchClassBase) as usize);
            let repClassBase = table_bytes.add(READ_u32((*inTable).repClassBase) as usize);
            let stringListBase = table_bytes.add(READ_u32((*inTable).stringListBase) as usize);
            let stringRuleData = table_bytes.add(READ_u32((*inTable).stringRuleData) as usize);

            let mut planeMap = std::ptr::null();
            if bInputIsUnicode && bSupplementaryChars {
                planeMap = pageBase;
                pageBase = pageBase.add(20);
                numPageMaps = READ_u8(*planeMap.add(17));
            }

            let iBufSize = ((READ_u8((*inTable).maxMatch) as libc::c_long
                + READ_u8((*inTable).maxPre) as libc::c_long
                + READ_u8((*inTable).maxPost) as libc::c_long
                + 7)
                & !0x0003) as libc::c_long;
            let iBuffer = libc::malloc(iBufSize as usize * std::mem::size_of::<UInt32>()) as *mut UInt32;

            let oBufSize = ((READ_u8((*inTable).maxOutput) as libc::c_long + 7) & !0x0003) as libc::c_long;
            let oBuffer = libc::malloc(oBufSize as usize * std::mem::size_of::<UInt32>()) as *mut UInt32;

            Self {
                oBuffer,
                oBufSize,
                oBufEnd: 0,
                oBufPtr: 0,
                prevStage: std::ptr::null_mut::<Converter>() as *mut dyn Stage,

                pattern: std::ptr::null(),
                patternLength: 0,
                direction: 0,
                info: [MatchInfo::default(); 256],
                infoLimit: 0,
                matchElems: 0,
                matchedLength: 0,
                groupRepeats: 0,
                sgrStack: std::ptr::null_mut(),

                converter: cnv,
                tableHeader: inTable,

                pageBase,
                lookupBase,
                matchClassBase,
                repClassBase,
                stringListBase,
                stringRuleData,
                planeMap,

                iBuffer,
                iBufSize,
                iBufStart: 0,
                iBufEnd: 0,
                iBufPtr: 0,

                bInputIsUnicode,
                bOutputIsUnicode,
                bSupplementaryChars,
                numPageMaps,
            }
        }
    }

    pub unsafe fn outputChar(&mut self, c: UInt32) {
        unsafe {
            if self.oBufEnd < self.oBufSize {
                *self.oBuffer.add(self.oBufEnd as usize) = c;
                self.oBufEnd += 1;
            }
        }
    }

    pub unsafe fn inputChar(&mut self, inIndex: libc::c_long) -> UInt32 {
        unsafe {
            let mut target = self.iBufPtr + inIndex;
            if inIndex < 0 {
                if target < 0 {
                    target += self.iBufSize;
                }
                if self.iBufPtr < self.iBufStart {
                    if target >= self.iBufStart || target < self.iBufPtr {
                        return *self.iBuffer.add(target as usize);
                    }
                } else {
                    if target >= self.iBufStart && target < self.iBufPtr {
                        return *self.iBuffer.add(target as usize);
                    }
                }
                return kEndOfText;
            } else {
                if target >= self.iBufSize {
                    target -= self.iBufSize;
                }
                if self.iBufPtr == self.iBufEnd {
                    let ch = (*self.prevStage).getChar();
                    if ch == kNeedMoreInput || ch == kInvalidChar || ch == kUnmappedChar {
                        return ch;
                    }
                    *self.iBuffer.add(self.iBufEnd as usize) = ch;
                    self.iBufEnd += 1;
                    if self.iBufEnd == self.iBufSize {
                        self.iBufEnd = 0;
                    }
                    if self.iBufEnd == self.iBufStart {
                        self.iBufStart += 1;
                        if self.iBufStart == self.iBufSize {
                            self.iBufStart = 0;
                        }
                    }
                }
                let mut index = self.iBufPtr;
                while index != target {
                    if index == self.iBufSize - 1 {
                        index = 0;
                    } else {
                        index += 1;
                    }
                    if index == self.iBufEnd {
                        let ch = (*self.prevStage).getChar();
                        if ch == kNeedMoreInput || ch == kInvalidChar || ch == kUnmappedChar {
                            return ch;
                        }
                        *self.iBuffer.add(self.iBufEnd as usize) = ch;
                        self.iBufEnd += 1;
                        if self.iBufEnd == self.iBufSize {
                            self.iBufEnd = 0;
                        }
                        if self.iBufEnd == self.iBufStart {
                            self.iBufStart += 1;
                            if self.iBufStart == self.iBufSize {
                                self.iBufStart = 0;
                            }
                        }
                    }
                }
                return *self.iBuffer.add(index as usize);
            }
        }
    }

    pub unsafe fn advanceInput(&mut self, numChars: libc::c_uint) {
        unsafe {
            for _ in 0..numChars {
                if self.iBufPtr == self.iBufEnd {
                    *self.iBuffer.add(self.iBufEnd as usize) = (*self.prevStage).getChar();
                    self.iBufEnd += 1;
                    if self.iBufEnd == self.iBufStart {
                        self.iBufStart += 1;
                        if self.iBufStart == self.iBufSize {
                            self.iBufStart = 0;
                        }
                    }
                    if self.iBufEnd == self.iBufSize {
                        self.iBufEnd = 0;
                    }
                }
                self.iBufPtr += 1;
                if self.iBufPtr == self.iBufSize {
                    self.iBufPtr = 0;
                }
            }
        }
    }

    pub unsafe fn classMatch(&self, classNumber: UInt32, inChar: UInt32) -> libc::c_long {
        unsafe {
            let offset = READ_u32(*(self.matchClassBase as *const UInt32).add(classNumber as usize));
            let mut classPtr = self.matchClassBase.add(offset as usize) as *const UInt32;
            let memberCount = READ_u32(*classPtr);
            classPtr = classPtr.add(1);

            if self.bInputIsUnicode {
                if self.bSupplementaryChars {
                    let p = binary_search_u32(classPtr, memberCount, inChar);
                    if READ_u32(*p) == inChar {
                        return (p.offset_from(classPtr)) as i64;
                    }
                } else {
                    let ptr16 = classPtr as *const UInt16;
                    let p = binary_search_u16(ptr16, memberCount, inChar);
                    if (READ_u16(*p) as UInt32) == inChar {
                        return (p.offset_from(ptr16)) as i64;
                    }
                }
            } else {
                let ptr8 = classPtr as *const UInt8;
                let p = binary_search_u8(ptr8, memberCount, inChar);
                if (READ_u8(*p) as UInt32) == inChar {
                    return (p.offset_from(ptr8)) as i64;
                }
            }
            -1
        }
    }

    pub unsafe fn repClassMember(&self, classNumber: UInt32, index: UInt32) -> UInt32 {
        unsafe {
            let offset = READ_u32(*(self.repClassBase as *const UInt32).add(classNumber as usize));
            let mut classPtr = self.repClassBase.add(offset as usize) as *const UInt32;
            let memberCount = READ_u32(*classPtr);
            classPtr = classPtr.add(1);

            if index < memberCount {
                if self.bOutputIsUnicode {
                    if self.bSupplementaryChars {
                        READ_u32(*classPtr.add(index as usize))
                    } else {
                        READ_u16(*(classPtr as *const UInt16).add(index as usize)) as UInt32
                    }
                } else {
                    READ_u8(*(classPtr as *const UInt8).add(index as usize)) as UInt32
                }
            } else {
                0
            }
        }
    }

    pub unsafe fn match_pattern(&mut self, mut index: libc::c_int, mut repeats: libc::c_int, mut textLoc: libc::c_int) -> UInt32 {
        unsafe {
            let mut _rval = matchNo;

            'restart: loop {
                if repeats == 0 {
                    if index == self.matchElems {
                        self.matchedLength = textLoc;
                    }
                    if index < self.infoLimit {
                        self.info[index as usize].matchedSpan.start = textLoc as UInt16;
                    }
                }

                if index >= self.patternLength {
                    _rval = matchYes;
                    break 'restart;
                }

                if index == 0 && repeats == 0 {
                    self.sgrStack = std::ptr::null_mut();
                }

                let m = &*self.pattern.add(index as usize);
                let repeatMin = (READ_u8(m.flags.repeat) >> 4) as libc::c_int;
                let repeatMax = (READ_u8(m.flags.repeat) & 0x0f) as libc::c_int;
                let rawType = READ_u8(m.flags.type_);
                let negate = (rawType & kMatchElem_Negate) != 0;
                let type_ = if (rawType & kMatchElem_NonLit) != 0 {
                    rawType & kMatchElem_TypeMask
                } else {
                    0
                };

                if type_ == kMatchElem_Type_BGroup {
                    self.info[index as usize].groupRepeats = repeats;
                    if repeats < repeatMax {
                        let mut altIndex = index;
                        loop {
                            let mr = self.match_pattern(altIndex + 1, 0, textLoc);
                            if mr != matchNo {
                                _rval = mr;
                                break 'restart;
                            }
                            altIndex += READ_u8((*self.pattern.add(altIndex as usize)).value.bgroup.dNext) as libc::c_int;
                            if (READ_u8((*self.pattern.add(altIndex as usize)).flags.type_) & kMatchElem_TypeMask) != kMatchElem_Type_OR {
                                break;
                            }
                        }
                    }
                    if repeats >= repeatMin {
                        let dAfter = READ_u8(m.value.bgroup.dAfter) as libc::c_int;
                        let mr = self.match_pattern(index + dAfter, 0, textLoc);
                        if mr == matchYes {
                            if index < self.infoLimit {
                                self.info[index as usize].matchedSpan.limit = textLoc as UInt16;
                                for i in (index + 1..(index + dAfter)).rev() {
                                    if i < self.infoLimit {
                                        if (self.info[i as usize].matchedSpan.start as libc::c_int) > textLoc {
                                            self.info[i as usize].matchedSpan.start = textLoc as UInt16;
                                        }
                                        if (self.info[i as usize].matchedSpan.limit as libc::c_int) > textLoc {
                                            self.info[i as usize].matchedSpan.limit = textLoc as UInt16;
                                        }
                                    }
                                }
                            }
                        }
                        _rval = mr;
                        break 'restart;
                    }
                    _rval = matchNo;
                    break 'restart;
                } else if type_ == kMatchElem_Type_OR || type_ == kMatchElem_Type_EGroup {
                    let startIndex = index - READ_u8(m.value.egroup.dStart) as libc::c_int;
                    let mr = self.match_pattern(startIndex, self.info[startIndex as usize].groupRepeats + 1, textLoc);
                    _rval = mr;
                    break 'restart;
                } else {
                    while repeats < repeatMin {
                        let inChar = self.inputChar(textLoc as libc::c_long);
                        if inChar == kInvalidChar || inChar == kNeedMoreInput || inChar == kUnmappedChar {
                            _rval = inChar;
                            break 'restart;
                        }
                        let mut matches = false;
                        match type_ {
                            0 => {
                                matches = (READ_u32(m.value.usv.data) & kUSVMask) == inChar;
                            }
                            kMatchElem_Type_Class => {
                                let classIndex = self.classMatch(READ_u16(m.value.cls.index) as UInt32, inChar);
                                matches = classIndex != -1;
                                if matches && repeats == 0 && index < self.infoLimit {
                                    self.info[index as usize].classIndex = classIndex as UInt32;
                                }
                            }
                            kMatchElem_Type_ANY => {
                                matches = inChar != kEndOfText;
                            }
                            kMatchElem_Type_EOS => {
                                matches = inChar == kEndOfText;
                            }
                            _ => {}
                        }
                        matches = matches != negate;
                        if !matches {
                            _rval = matchNo;
                            break 'restart;
                        }
                        repeats += 1;
                        textLoc += self.direction;
                    }

                    if index < self.infoLimit {
                        self.info[index as usize].matchedSpan.limit = textLoc as UInt16;
                    }

                    if repeatMin == repeatMax {
                        index += 1;
                        repeats = 0;
                        continue 'restart;
                    }

                    if repeats < repeatMax {
                        let inChar = self.inputChar(textLoc as libc::c_long);
                        if inChar == kInvalidChar || inChar == kNeedMoreInput || inChar == kUnmappedChar {
                            _rval = inChar;
                            break 'restart;
                        }
                        let mut matches = false;
                        match type_ {
                            0 => {
                                matches = (READ_u32(m.value.usv.data) & kUSVMask) == inChar;
                            }
                            kMatchElem_Type_Class => {
                                let classIndex = self.classMatch(READ_u16(m.value.cls.index) as UInt32, inChar);
                                matches = classIndex != -1;
                                if matches && repeats == 0 && index < self.infoLimit {
                                    self.info[index as usize].classIndex = classIndex as UInt32;
                                }
                            }
                            kMatchElem_Type_ANY => {
                                matches = inChar != kEndOfText;
                            }
                            kMatchElem_Type_EOS => {
                                matches = inChar == kEndOfText;
                            }
                            _ => {}
                        }
                        matches = matches != negate;
                        if matches {
                            let mr = self.match_pattern(index, repeats + 1, textLoc + self.direction);
                            if mr != matchNo {
                                _rval = mr;
                                break 'restart;
                            }
                        }
                    }

                    let mr = self.match_pattern(index + 1, 0, textLoc);
                    _rval = mr;
                    break 'restart;
                }
            }

            if _rval == matchNo {
                if index < self.infoLimit {
                    self.info[index as usize].matchedSpan.limit = textLoc as UInt16;
                }
            }

            _rval
        }
    }

    pub unsafe fn DoMapping(&mut self) -> UInt32 {
        unsafe {
            let inChar = self.inputChar(0);
            if inChar == kNeedMoreInput || inChar == kInvalidChar || inChar == kUnmappedChar {
                return inChar;
            }
            if inChar == kEndOfText {
                self.outputChar(kEndOfText);
                return inChar;
            }
            self.matchedLength = 1;

            let mut lookup: *const Lookup;
            if self.bInputIsUnicode {
                let mut charIndex: UInt16 = 0;
                if (self.lookupBase as *const UInt8) == self.pageBase {
                    // pass with no rules
                } else {
                    let plane = (inChar >> 16) as UInt8;
                    let mut pageMap: *const UInt8 = std::ptr::null();
                    if self.bSupplementaryChars {
                        if plane < 17 && READ_u8(*self.planeMap.add(plane as usize)) != 0xff {
                            pageMap = self.pageBase.add(256 * READ_u8(*self.planeMap.add(plane as usize)) as usize);
                            let page = ((inChar >> 8) & 0xff) as usize;
                            if READ_u8(*pageMap.add(page)) != 0xff {
                                let charMapBase = self.pageBase.add(256 * self.numPageMaps as usize) as *const UInt16;
                                let charMap = charMapBase.add(256 * READ_u8(*pageMap.add(page)) as usize);
                                charIndex = READ_u16(*charMap.add((inChar & 0xff) as usize));
                            }
                        }
                    } else if plane == 0 {
                        pageMap = self.pageBase;
                        let page = ((inChar >> 8) & 0xff) as usize;
                        if READ_u8(*pageMap.add(page)) != 0xff {
                            let charMapBase = self.pageBase.add(256 * self.numPageMaps as usize) as *const UInt16;
                            let charMap = charMapBase.add(256 * READ_u8(*pageMap.add(page)) as usize);
                            charIndex = READ_u16(*charMap.add((inChar & 0xff) as usize));
                        }
                    }
                }
                lookup = self.lookupBase.add(charIndex as usize);
            } else {
                if self.pageBase != (self.tableHeader as *const Byte) {
                    let pageNumber = READ_u8(*self.pageBase.add(inChar as usize)) as libc::c_long;
                    if pageNumber == 0 {
                        lookup = self.lookupBase.add(inChar as usize);
                    } else {
                        let nextChar = self.inputChar(1);
                        if nextChar == kNeedMoreInput || nextChar == kInvalidChar || nextChar == kUnmappedChar {
                            return nextChar;
                        }
                        if nextChar == kEndOfText {
                            lookup = self.lookupBase.add(inChar as usize);
                        } else {
                            lookup = self.lookupBase.add((pageNumber * 256 + nextChar as libc::c_long) as usize);
                            if READ_u8((*lookup).rules.type_) == kLookupType_IllegalDBCS {
                                lookup = self.lookupBase.add(inChar as usize);
                            } else {
                                self.matchedLength = 2;
                            }
                        }
                    }
                } else {
                    lookup = self.lookupBase.add(inChar as usize);
                }
            }

            let ruleType = READ_u8((*lookup).rules.type_);
            if ruleType == kLookupType_StringRules || (ruleType & kLookupType_RuleTypeMask) == kLookupType_ExtStringRules {
                let mut ruleList = (self.stringListBase as *const UInt32).add(READ_u16((*lookup).rules.ruleIndex) as usize);
                let mut matched = false;
                let mut allowInsertion = true;
                let mut ruleCount = READ_u8((*lookup).rules.ruleCount) as libc::c_int;
                if (ruleType & kLookupType_RuleTypeMask) == kLookupType_ExtStringRules {
                    ruleCount += 256 * (ruleType & kLookupType_ExtRuleCountMask) as libc::c_int;
                }

                while ruleCount > 0 {
                    ruleCount -= 1;
                    let rule = self.stringRuleData.add(READ_u32(*ruleList) as usize) as *const StringRule;
                    ruleList = ruleList.add(1);

                    self.matchElems = READ_u8((*rule).matchLength) as libc::c_int;
                    if self.matchElems == 0 && !allowInsertion {
                        continue;
                    }
                    self.patternLength = self.matchElems + READ_u8((*rule).postLength) as libc::c_int;
                    self.pattern = rule.add(1) as *const MatchElem;
                    self.direction = 1;
                    self.infoLimit = self.matchElems;

                    for i in 0..self.infoLimit as usize {
                        self.info[i].matchedSpan.start = 0;
                        self.info[i].matchedSpan.limit = 0;
                    }

                    let mut mr = self.match_pattern(0, 0, 0);
                    if mr == matchYes {
                        if self.matchedLength == 0 && !allowInsertion {
                            continue;
                        }
                        self.pattern = self.pattern.add(self.patternLength as usize);
                        self.patternLength = READ_u8((*rule).preLength) as libc::c_int;
                        if self.patternLength > 0 {
                            self.direction = -1;
                            self.infoLimit = 0;
                            self.matchElems = -1;
                            mr = self.match_pattern(0, 0, -1);
                        }
                        if mr == matchYes {
                            let mut r = self.pattern.add(self.patternLength as usize) as *const RepElem;
                            for _ in 0..READ_u8((*rule).repLength) {
                                match READ_u8((*r).flags.type_) {
                                    kRepElem_Literal => {
                                        self.outputChar(READ_u32((*r).value));
                                    }
                                    kRepElem_Class => {
                                        let myInfo = &self.info[READ_u8((*r).flags.matchIndex) as usize];
                                        if myInfo.matchedSpan.start < myInfo.matchedSpan.limit {
                                            self.outputChar(self.repClassMember(READ_u16((*r).flags.repClass) as UInt32, myInfo.classIndex));
                                        }
                                    }
                                    kRepElem_Copy => {
                                        let myInfo = &self.info[READ_u8((*r).flags.matchIndex) as usize];
                                        for j in myInfo.matchedSpan.start..myInfo.matchedSpan.limit {
                                            let ch = self.inputChar(j as libc::c_long);
                                            self.outputChar(ch);
                                        }
                                    }
                                    kRepElem_Unmapped => {
                                        if self.bOutputIsUnicode == self.bInputIsUnicode {
                                            self.outputChar(inChar);
                                        } else {
                                            match (*self.converter).unmappedBehavior as UInt32 {
                                                kOptionsUnmapped_DontUseReplacementChar => {
                                                    return kUnmappedChar;
                                                }
                                                kOptionsUnmapped_UseReplacementCharWithWarning => {
                                                    (*self.converter).warningStatus |= kStatus_UsedReplacement as UInt32;
                                                    self.outputChar(READ_u32((*self.tableHeader).replacementChar));
                                                }
                                                _ => {
                                                    self.outputChar(READ_u32((*self.tableHeader).replacementChar));
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                                r = r.add(1);
                            }
                            if self.matchedLength > 0 {
                                matched = true;
                                break;
                            } else {
                                allowInsertion = false;
                            }
                        } else if mr != matchNo {
                            return mr;
                        }
                    } else if mr != matchNo {
                        return mr;
                    }
                }

                if !matched {
                    if self.bOutputIsUnicode == self.bInputIsUnicode {
                        self.outputChar(inChar);
                    } else {
                        match (*self.converter).unmappedBehavior as UInt32 {
                            kOptionsUnmapped_DontUseReplacementChar => {
                                return kUnmappedChar;
                            }
                            kOptionsUnmapped_UseReplacementCharWithWarning => {
                                (*self.converter).warningStatus |= kStatus_UsedReplacement as UInt32;
                                self.outputChar(READ_u32((*self.tableHeader).replacementChar));
                            }
                            _ => {
                                self.outputChar(READ_u32((*self.tableHeader).replacementChar));
                            }
                        }
                    }
                    self.matchedLength = 1;
                }
            } else if ruleType == kLookupType_Unmapped {
                if self.bOutputIsUnicode == self.bInputIsUnicode {
                    self.outputChar(inChar);
                } else {
                    match (*self.converter).unmappedBehavior as UInt32 {
                        kOptionsUnmapped_DontUseReplacementChar => {
                            return kUnmappedChar;
                        }
                        kOptionsUnmapped_UseReplacementCharWithWarning => {
                            (*self.converter).warningStatus |= kStatus_UsedReplacement as UInt32;
                            self.outputChar(READ_u32((*self.tableHeader).replacementChar));
                        }
                        _ => {
                            self.outputChar(READ_u32((*self.tableHeader).replacementChar));
                        }
                    }
                }
            } else {
                if self.bOutputIsUnicode {
                    let usv = READ_u32((*lookup).usv);
                    if usv <= 0x0010ffff {
                        self.outputChar(usv);
                    }
                } else {
                    let count = READ_u8((*lookup).bytes.count);
                    for i in 0..count as usize {
                        self.outputChar(READ_u8((*lookup).bytes.data[i]) as UInt32);
                    }
                }
            }

            self.advanceInput(self.matchedLength as libc::c_uint);
            0
        }
    }
}

impl Drop for Pass {
    fn drop(&mut self) {
        unsafe {
            if !self.oBuffer.is_null() {
                libc::free(self.oBuffer as *mut libc::c_void);
                self.oBuffer = std::ptr::null_mut();
            }
            if !self.iBuffer.is_null() {
                libc::free(self.iBuffer as *mut libc::c_void);
                self.iBuffer = std::ptr::null_mut();
            }
        }
    }
}

impl Stage for Pass {
    unsafe fn getChar(&mut self) -> UInt32 {
        unsafe {
            while self.oBufPtr == self.oBufEnd {
                self.oBufPtr = 0;
                self.oBufEnd = 0;
                let c = self.DoMapping();
                if c == kNeedMoreInput || c == kInvalidChar || c == kUnmappedChar {
                    return c;
                }
            }
            let c = *self.oBuffer.add(self.oBufPtr as usize);
            self.oBufPtr += 1;
            c
        }
    }

    unsafe fn Reset(&mut self) {
        self.iBufStart = 0;
        self.iBufEnd = 0;
        self.iBufPtr = 0;
        self.oBufPtr = 0;
        self.oBufEnd = 0;
    }

    unsafe fn lookaheadCount(&self) -> UInt32 {
        if self.iBufEnd < self.iBufPtr {
            (self.iBufEnd + (self.iBufSize - self.iBufPtr)) as UInt32
        } else {
            (self.iBufEnd - self.iBufPtr) as UInt32
        }
    }

    unsafe fn getPrevStage(&self) -> *mut dyn Stage {
        self.prevStage
    }

    unsafe fn setPrevStage(&mut self, prev: *mut dyn Stage) {
        self.prevStage = prev;
    }
}

pub struct Converter {
    pub oBuffer: *mut UInt32,
    pub oBufSize: libc::c_long,
    pub oBufEnd: libc::c_long,
    pub oBufPtr: libc::c_long,
    pub prevStage: *mut dyn Stage,

    pub table: *mut Byte,
    pub finalStage: *mut dyn Stage,
    pub data: *const Byte,
    pub dataPtr: UInt32,
    pub dataLen: UInt32,
    pub inputComplete: bool,
    pub unmappedBehavior: Byte,
    pub forward: bool,
    pub inputForm: Byte,
    pub outputForm: Byte,
    pub savedBytes: [Byte; 8],
    pub savedCount: UInt32,
    pub pendingOutputChar: UInt32,
    pub status: libc::c_long,
    pub warningStatus: UInt32,
}

impl Converter {
    pub unsafe fn new(inTable: *const Byte, inTableSize: UInt32, inForward: bool, inForm: UInt16, outForm: UInt16) -> Box<Self> {
        unsafe {
            let mut cnv = Box::new(Self {
                oBuffer: std::ptr::null_mut(),
                oBufSize: 0,
                oBufEnd: 0,
                oBufPtr: 0,
                prevStage: std::ptr::null_mut::<Converter>() as *mut dyn Stage,

                table: std::ptr::null_mut(),
                finalStage: std::ptr::null_mut::<Converter>() as *mut dyn Stage,
                data: std::ptr::null(),
                dataPtr: 0,
                dataLen: 0,
                inputComplete: false,
                unmappedBehavior: 0,
                forward: inForward,
                inputForm: (inForm & kForm_EncodingFormMask) as Byte,
                outputForm: (outForm & kForm_EncodingFormMask) as Byte,
                savedBytes: [0; 8],
                savedCount: 0,
                pendingOutputChar: kInvalidChar,
                status: kStatus_NoError,
                warningStatus: 0,
            });

            let cnv_ptr: *mut Converter = &mut *cnv;
            cnv.finalStage = cnv_ptr as *mut dyn Stage;

            let mut normForm: UInt16 = 0;
            if !inTable.is_null() {
                let mut fh = inTable as *const FileHeader;
                if READ_u32((*fh).type_) == kMagicNumberCmp {
                    let mut uncompressedLen: u64 = READ_u32((*fh).version) as u64;
                    cnv.table = libc::malloc(uncompressedLen as usize) as *mut Byte;
                    if cnv.table.is_null() {
                        cnv.status = kStatus_OutOfMemory;
                        return cnv;
                    }

                    let res = tectonic_bridge_flate::tectonic_flate_decompress(
                        cnv.table,
                        &mut uncompressedLen,
                        inTable.add(2 * std::mem::size_of::<UInt32>()),
                        (inTableSize as usize - 2 * std::mem::size_of::<UInt32>()) as u64,
                    );
                    if (res as i32) < 0 {
                        cnv.status = kStatus_InvalidMapping;
                        return cnv;
                    }
                    fh = cnv.table as *const FileHeader;
                }

                if READ_u32((*fh).type_) != kMagicNumber {
                    cnv.status = kStatus_InvalidMapping;
                    return cnv;
                }
                if (READ_u32((*fh).version) & 0xFFFF0000) > (kCurrentFileVersion & 0xFFFF0000) {
                    cnv.status = kStatus_BadMappingVersion;
                    return cnv;
                }

                if cnv.table.is_null() {
                    cnv.table = libc::malloc(inTableSize as usize) as *mut Byte;
                    if cnv.table.is_null() {
                        cnv.status = kStatus_OutOfMemory;
                        return cnv;
                    }
                    std::ptr::copy_nonoverlapping(inTable, cnv.table, inTableSize as usize);
                }

                fh = cnv.table as *const FileHeader;
                let nameOffsets = cnv.table.add(std::mem::size_of::<FileHeader>()) as *const UInt32;
                let mut tableBase = nameOffsets.add(READ_u32((*fh).numNames) as usize);
                let mut numTables = READ_u32((*fh).numFwdTables);
                if !cnv.forward {
                    tableBase = tableBase.add(numTables as usize);
                    numTables = READ_u32((*fh).numRevTables);
                }

                let targetFlags = if cnv.forward {
                    READ_u32((*fh).formFlagsRHS)
                } else {
                    READ_u32((*fh).formFlagsLHS)
                };

                if (targetFlags & kFlags_Unicode) != 0 {
                    if (cnv.outputForm as UInt16) < kForm_UTF8 || (cnv.outputForm as UInt16) > kForm_UTF32LE {
                        cnv.status = kStatus_InvalidForm;
                        return cnv;
                    }
                } else {
                    if (cnv.outputForm as UInt16) != kForm_Bytes {
                        cnv.status = kStatus_InvalidForm;
                        return cnv;
                    }
                }

                let sourceFlags = if cnv.forward {
                    READ_u32((*fh).formFlagsLHS)
                } else {
                    READ_u32((*fh).formFlagsRHS)
                };

                if (sourceFlags & kFlags_Unicode) != 0 {
                    if (cnv.inputForm as UInt16) < kForm_UTF8 || (cnv.inputForm as UInt16) > kForm_UTF32LE {
                        cnv.status = kStatus_InvalidForm;
                        return cnv;
                    }
                    let mut n: *mut dyn Stage = std::ptr::null_mut::<Converter>() as *mut dyn Stage;
                    if (sourceFlags & kFlags_ExpectsNFD) != 0 {
                        n = Box::into_raw(Box::new(Normalizer::new(false)));
                        normForm = kForm_NFD;
                    } else if (sourceFlags & kFlags_ExpectsNFC) != 0 {
                        n = Box::into_raw(Box::new(Normalizer::new(true)));
                        normForm = kForm_NFC;
                    }
                    if !n.is_null() {
                        (*n).setPrevStage(cnv.finalStage);
                        cnv.finalStage = n;
                    }
                } else {
                    if (cnv.inputForm as UInt16) != kForm_Bytes {
                        cnv.status = kStatus_InvalidForm;
                        return cnv;
                    }
                }

                for i in 0..numTables {
                    let t = cnv.table.add(READ_u32(*tableBase.add(i as usize)) as usize) as *const TableHeader;
                    let mut p: *mut dyn Stage = std::ptr::null_mut::<Converter>() as *mut dyn Stage;
                    match READ_u32((*t).type_) {
                        kTableType_BB | kTableType_BU | kTableType_UU | kTableType_UB => {
                            p = Box::into_raw(Box::new(Pass::new(t, cnv_ptr)));
                            normForm = 0;
                        }
                        kTableType_NFC => {
                            p = Box::into_raw(Box::new(Normalizer::new(true)));
                            normForm = kForm_NFC;
                        }
                        kTableType_NFD => {
                            p = Box::into_raw(Box::new(Normalizer::new(false)));
                            normForm = kForm_NFD;
                        }
                        _ => {}
                    }
                    if p.is_null() {
                        cnv.status = kStatus_InvalidMapping;
                        return cnv;
                    }
                    (*p).setPrevStage(cnv.finalStage);
                    cnv.finalStage = p;
                }
            } else {
                if (cnv.inputForm as UInt16) < kForm_UTF8
                    || (cnv.inputForm as UInt16) > kForm_UTF32LE
                    || (cnv.outputForm as UInt16) < kForm_UTF8
                    || (cnv.outputForm as UInt16) > kForm_UTF32LE
                {
                    cnv.status = kStatus_InvalidForm;
                    return cnv;
                }
            }

            if (cnv.outputForm as UInt16) >= kForm_UTF8 && (cnv.outputForm as UInt16) <= kForm_UTF32LE {
                let mut n: *mut dyn Stage = std::ptr::null_mut::<Converter>() as *mut dyn Stage;
                if (outForm & kForm_NormalizationMask) == kForm_NFD && normForm != kForm_NFD {
                    n = Box::into_raw(Box::new(Normalizer::new(false)));
                } else if (outForm & kForm_NormalizationMask) == kForm_NFC && normForm != kForm_NFC {
                    n = Box::into_raw(Box::new(Normalizer::new(true)));
                }
                if !n.is_null() {
                    (*n).setPrevStage(cnv.finalStage);
                    cnv.finalStage = n;
                }
            }

            cnv
        }
    }

    pub fn creationStatus(&self) -> libc::c_long {
        self.status
    }

    pub fn IsForward(&self) -> bool {
        self.forward
    }

    pub unsafe fn GetFlags(&self, sourceFlags: *mut UInt32, targetFlags: *mut UInt32) {
        unsafe {
            let fh = self.table as *const FileHeader;
            if self.forward {
                *sourceFlags = READ_u32((*fh).formFlagsLHS);
                *targetFlags = READ_u32((*fh).formFlagsRHS);
            } else {
                *sourceFlags = READ_u32((*fh).formFlagsRHS);
                *targetFlags = READ_u32((*fh).formFlagsLHS);
            }
        }
    }

    pub unsafe fn GetNamePtr(&self, inNameID: UInt16, outNamePtr: *mut *const Byte, outNameLen: *mut UInt32) -> bool {
        unsafe { getNamePtrFromTable(self.table, inNameID, outNamePtr, outNameLen) }
    }

    pub unsafe fn _savePendingBytes(&mut self) {
        unsafe {
            self.dataPtr -= self.savedCount;
            while self.dataPtr < self.dataLen {
                self.savedBytes[self.savedCount as usize] = *self.data.add(self.dataPtr as usize);
                self.savedCount += 1;
                self.dataPtr += 1;
            }
        }
    }

    pub unsafe fn _getCharFn(&mut self) -> UInt32 {
        unsafe {
            if self.savedCount > 0 {
                return self._getCharWithSavedBytes();
            }

            let mut rval: UInt32 = 0;

            match self.inputForm as UInt16 {
                kForm_UTF8 => {
                    let extraBytes = bytesFromUTF8[*self.data.add(self.dataPtr as usize) as usize] as UInt32;
                    if self.dataPtr + extraBytes + 1 > self.dataLen {
                        if self.inputComplete {
                            return kInvalidChar;
                        } else {
                            self._savePendingBytes();
                            return kNeedMoreInput;
                        }
                    }
                    for _ in 0..=extraBytes {
                        rval += *self.data.add(self.dataPtr as usize) as UInt32;
                        self.dataPtr += 1;
                        rval <<= 6;
                    }
                    rval >>= 6;
                    rval -= offsetsFromUTF8[extraBytes as usize];
                }
                kForm_UTF16BE => {
                    if self.dataPtr + 2 > self.dataLen {
                        if self.inputComplete {
                            return kInvalidChar;
                        } else {
                            self._savePendingBytes();
                            return kNeedMoreInput;
                        }
                    }
                    rval = u16::from_be_bytes(std::slice::from_raw_parts(self.data.add(self.dataPtr as usize), 2).try_into().unwrap()) as UInt32;
                    self.dataPtr += 2;
                    if rval >= kSurrogateHighStart && rval <= kSurrogateHighEnd {
                        self.dataPtr -= 2;
                        if self.dataPtr + 4 > self.dataLen {
                            if self.inputComplete {
                                return kInvalidChar;
                            } else {
                                self._savePendingBytes();
                                return kNeedMoreInput;
                            }
                        }
                        self.dataPtr += 2;
                        let low =
                            u16::from_be_bytes(std::slice::from_raw_parts(self.data.add(self.dataPtr as usize), 2).try_into().unwrap()) as UInt32;
                        self.dataPtr += 2;
                        rval = ((rval - kSurrogateHighStart) << halfShift) + (low - kSurrogateLowStart) + halfBase;
                    }
                }
                kForm_UTF16LE => {
                    if self.dataPtr + 2 > self.dataLen {
                        if self.inputComplete {
                            return kInvalidChar;
                        } else {
                            self._savePendingBytes();
                            return kNeedMoreInput;
                        }
                    }
                    rval = u16::from_le_bytes(std::slice::from_raw_parts(self.data.add(self.dataPtr as usize), 2).try_into().unwrap()) as UInt32;
                    self.dataPtr += 2;
                    if rval >= kSurrogateHighStart && rval <= kSurrogateHighEnd {
                        self.dataPtr -= 2;
                        if self.dataPtr + 4 > self.dataLen {
                            if self.inputComplete {
                                return kInvalidChar;
                            } else {
                                self._savePendingBytes();
                                return kNeedMoreInput;
                            }
                        }
                        self.dataPtr += 2;
                        let low =
                            u16::from_le_bytes(std::slice::from_raw_parts(self.data.add(self.dataPtr as usize), 2).try_into().unwrap()) as UInt32;
                        self.dataPtr += 2;
                        rval = ((rval - kSurrogateHighStart) << halfShift) + (low - kSurrogateLowStart) + halfBase;
                    }
                }
                kForm_UTF32BE => {
                    if self.dataPtr + 4 > self.dataLen {
                        if self.inputComplete {
                            return kInvalidChar;
                        } else {
                            self._savePendingBytes();
                            return kNeedMoreInput;
                        }
                    }
                    rval = u32::from_be_bytes(std::slice::from_raw_parts(self.data.add(self.dataPtr as usize), 4).try_into().unwrap());
                    self.dataPtr += 4;
                }
                kForm_UTF32LE => {
                    if self.dataPtr + 4 > self.dataLen {
                        if self.inputComplete {
                            return kInvalidChar;
                        } else {
                            self._savePendingBytes();
                            return kNeedMoreInput;
                        }
                    }
                    rval = u32::from_le_bytes(std::slice::from_raw_parts(self.data.add(self.dataPtr as usize), 4).try_into().unwrap());
                    self.dataPtr += 4;
                }
                _ => {}
            }

            rval
        }
    }

    pub unsafe fn _getCharWithSavedBytes(&mut self) -> UInt32 {
        unsafe {
            let mut rval: UInt32 = 0;
            let data_byte = |savedCount: UInt32, savedBytes: &[Byte; 8], data: *const Byte, idx: UInt32| -> UInt32 {
                if idx < savedCount {
                    savedBytes[idx as usize] as UInt32
                } else {
                    *data.add((idx - savedCount) as usize) as UInt32
                }
            };
            let word_bytes = |position: UInt32| -> [u8; 2] {
                std::array::from_fn(|i| data_byte(self.savedCount, &self.savedBytes, self.data, position + i as u32) as u8)
            };
            let dword_bytes = |position: UInt32| -> [u8; 4] {
                std::array::from_fn(|i| data_byte(self.savedCount, &self.savedBytes, self.data, position + i as u32) as u8)
            };

            match self.inputForm as UInt16 {
                kForm_UTF8 => {
                    let b = data_byte(self.savedCount, &self.savedBytes, self.data, self.dataPtr);
                    let extraBytes = bytesFromUTF8[b as usize] as UInt32;
                    if self.dataPtr + extraBytes + 1 > self.savedCount + self.dataLen {
                        if self.inputComplete {
                            return kInvalidChar;
                        } else {
                            self._savePendingBytes();
                            return kNeedMoreInput;
                        }
                    }
                    for _ in 0..=extraBytes {
                        rval += data_byte(self.savedCount, &self.savedBytes, self.data, self.dataPtr);
                        self.dataPtr += 1;
                        rval <<= 6;
                    }
                    rval >>= 6;
                    rval -= offsetsFromUTF8[extraBytes as usize];
                }
                kForm_UTF16BE => {
                    if self.dataPtr + 2 > self.savedCount + self.dataLen {
                        if self.inputComplete {
                            return kInvalidChar;
                        } else {
                            self._savePendingBytes();
                            return kNeedMoreInput;
                        }
                    }
                    rval = u16::from_be_bytes(word_bytes(self.dataPtr)) as UInt32;
                    self.dataPtr += 2;
                    if rval >= kSurrogateHighStart && rval <= kSurrogateHighEnd {
                        self.dataPtr -= 2;
                        if self.dataPtr + 4 > self.savedCount + self.dataLen {
                            if self.inputComplete {
                                return kInvalidChar;
                            } else {
                                self._savePendingBytes();
                                return kNeedMoreInput;
                            }
                        }
                        self.dataPtr += 2;
                        let low = u16::from_be_bytes(word_bytes(self.dataPtr)) as UInt32;
                        self.dataPtr += 2;
                        rval = ((rval - kSurrogateHighStart) << halfShift) + (low - kSurrogateLowStart) + halfBase;
                    }
                }
                kForm_UTF16LE => {
                    if self.dataPtr + 2 > self.savedCount + self.dataLen {
                        if self.inputComplete {
                            return kInvalidChar;
                        } else {
                            self._savePendingBytes();
                            return kNeedMoreInput;
                        }
                    }
                    rval = u16::from_le_bytes(word_bytes(self.dataPtr)) as UInt32;
                    self.dataPtr += 2;
                    if rval >= kSurrogateHighStart && rval <= kSurrogateHighEnd {
                        self.dataPtr -= 2;
                        if self.dataPtr + 4 > self.savedCount + self.dataLen {
                            if self.inputComplete {
                                return kInvalidChar;
                            } else {
                                self._savePendingBytes();
                                return kNeedMoreInput;
                            }
                        }
                        self.dataPtr += 2;
                        let low = u16::from_le_bytes(word_bytes(self.dataPtr)) as UInt32;
                        self.dataPtr += 2;
                        rval = ((rval - kSurrogateHighStart) << halfShift) + (low - kSurrogateLowStart) + halfBase;
                    }
                }
                kForm_UTF32BE => {
                    if self.dataPtr + 4 > self.savedCount + self.dataLen {
                        if self.inputComplete {
                            return kInvalidChar;
                        } else {
                            self._savePendingBytes();
                            return kNeedMoreInput;
                        }
                    }
                    rval = u32::from_be_bytes(dword_bytes(self.dataPtr));
                    self.dataPtr += 4;
                }
                kForm_UTF32LE => {
                    if self.dataPtr + 4 > self.savedCount + self.dataLen {
                        if self.inputComplete {
                            return kInvalidChar;
                        } else {
                            self._savePendingBytes();
                            return kNeedMoreInput;
                        }
                    }
                    rval = u32::from_le_bytes(dword_bytes(self.dataPtr));
                    self.dataPtr += 4;
                }
                _ => {}
            }

            if self.dataPtr >= self.savedCount {
                self.dataPtr -= self.savedCount;
                self.savedCount = 0;
            }

            rval
        }
    }

    pub unsafe fn ConvertBufferOpt(
        cnv: *mut Self, inBuffer: *const Byte, inLength: UInt32, inUsed: *mut UInt32, outBuffer: *mut Byte, outLength: UInt32, outUsed: *mut UInt32,
        inOptions: UInt32, lookaheadCount: *mut UInt32,
    ) -> TECkit_Status {
        unsafe {
            let mut outPtr: UInt32 = 0;
            let mut rval: TECkit_Status = kStatus_NoError;

            (*cnv).data = inBuffer;
            (*cnv).dataLen = inLength;
            (*cnv).dataPtr = 0;
            (*cnv).inputComplete = (inOptions & kOptionsMask_InputComplete) == kOptionsComplete_InputIsComplete;
            (*cnv).unmappedBehavior = (inOptions & kOptionsMask_UnmappedBehavior) as Byte;

            let mut c: UInt32;
            let mut skip_fetch = false;
            if (*cnv).pendingOutputChar != kInvalidChar {
                c = (*cnv).pendingOutputChar;
                (*cnv).pendingOutputChar = kInvalidChar;
                skip_fetch = true;
            } else {
                c = 0;
            }

            'convert_loop: loop {
                if !skip_fetch {
                    c = (*(*cnv).finalStage).getChar();
                }
                skip_fetch = false;

                match c {
                    kEndOfText => {
                        rval = kStatus_NoError;
                        break 'convert_loop;
                    }
                    kNeedMoreInput => {
                        rval = kStatus_NeedMoreInput;
                        break 'convert_loop;
                    }
                    kInvalidChar => {
                        rval = kStatus_IncompleteChar;
                        break 'convert_loop;
                    }
                    kUnmappedChar => {
                        rval = kStatus_UnmappedChar;
                        break 'convert_loop;
                    }
                    _ => match (*cnv).outputForm as UInt16 {
                        kForm_Bytes => {
                            if outPtr == outLength {
                                (*cnv).pendingOutputChar = c;
                                rval = kStatus_OutputBufferFull;
                                break 'convert_loop;
                            }
                            *outBuffer.add(outPtr as usize) = c as Byte;
                            outPtr += 1;
                        }
                        kForm_UTF8 => {
                            let bytesToWrite: usize;
                            if c < 0x80 {
                                bytesToWrite = 1;
                            } else if c < 0x800 {
                                bytesToWrite = 2;
                            } else if c < 0x10000 {
                                bytesToWrite = 3;
                            } else if c < 0x200000 {
                                bytesToWrite = 4;
                            } else {
                                bytesToWrite = 2;
                                c = 0x0000fffd;
                            }
                            if outPtr + bytesToWrite as UInt32 > outLength {
                                (*cnv).pendingOutputChar = c;
                                rval = kStatus_OutputBufferFull;
                                break 'convert_loop;
                            }
                            outPtr += bytesToWrite as UInt32;
                            let mut write_pos = outPtr;
                            let mut temp_c = c;
                            match bytesToWrite {
                                4 => {
                                    write_pos -= 1;
                                    *outBuffer.add(write_pos as usize) = ((temp_c | byteMark) & byteMask) as Byte;
                                    temp_c >>= 6;
                                    write_pos -= 1;
                                    *outBuffer.add(write_pos as usize) = ((temp_c | byteMark) & byteMask) as Byte;
                                    temp_c >>= 6;
                                    write_pos -= 1;
                                    *outBuffer.add(write_pos as usize) = ((temp_c | byteMark) & byteMask) as Byte;
                                    temp_c >>= 6;
                                    write_pos -= 1;
                                    *outBuffer.add(write_pos as usize) = (temp_c | firstByteMark[bytesToWrite] as UInt32) as Byte;
                                }
                                3 => {
                                    write_pos -= 1;
                                    *outBuffer.add(write_pos as usize) = ((temp_c | byteMark) & byteMask) as Byte;
                                    temp_c >>= 6;
                                    write_pos -= 1;
                                    *outBuffer.add(write_pos as usize) = ((temp_c | byteMark) & byteMask) as Byte;
                                    temp_c >>= 6;
                                    write_pos -= 1;
                                    *outBuffer.add(write_pos as usize) = (temp_c | firstByteMark[bytesToWrite] as UInt32) as Byte;
                                }
                                2 => {
                                    write_pos -= 1;
                                    *outBuffer.add(write_pos as usize) = ((temp_c | byteMark) & byteMask) as Byte;
                                    temp_c >>= 6;
                                    write_pos -= 1;
                                    *outBuffer.add(write_pos as usize) = (temp_c | firstByteMark[bytesToWrite] as UInt32) as Byte;
                                }
                                1 => {
                                    write_pos -= 1;
                                    *outBuffer.add(write_pos as usize) = (temp_c | firstByteMark[bytesToWrite] as UInt32) as Byte;
                                }
                                _ => {}
                            }
                        }
                        kForm_UTF16BE => {
                            if c < 0x00010000 {
                                if outPtr + 2 > outLength {
                                    (*cnv).pendingOutputChar = c;
                                    rval = kStatus_OutputBufferFull;
                                    break 'convert_loop;
                                }
                                std::slice::from_raw_parts_mut(outBuffer.add(outPtr as usize), 2).copy_from_slice(&(c as u16).to_be_bytes());
                                outPtr += 2;
                            } else {
                                if outPtr + 4 > outLength {
                                    (*cnv).pendingOutputChar = c;
                                    rval = kStatus_OutputBufferFull;
                                    break 'convert_loop;
                                }
                                let c_sub = c - halfBase;
                                let hi = (c_sub >> halfShift) + kSurrogateHighStart;
                                let lo = (c_sub & halfMask) + kSurrogateLowStart;
                                let output = std::slice::from_raw_parts_mut(outBuffer.add(outPtr as usize), 4);
                                output[..2].copy_from_slice(&(hi as u16).to_be_bytes());
                                output[2..].copy_from_slice(&(lo as u16).to_be_bytes());
                                outPtr += 4;
                            }
                        }
                        kForm_UTF16LE => {
                            if c < 0x00010000 {
                                if outPtr + 2 > outLength {
                                    (*cnv).pendingOutputChar = c;
                                    rval = kStatus_OutputBufferFull;
                                    break 'convert_loop;
                                }
                                std::slice::from_raw_parts_mut(outBuffer.add(outPtr as usize), 2).copy_from_slice(&(c as u16).to_le_bytes());
                                outPtr += 2;
                            } else {
                                if outPtr + 4 > outLength {
                                    (*cnv).pendingOutputChar = c;
                                    rval = kStatus_OutputBufferFull;
                                    break 'convert_loop;
                                }
                                let c_sub = c - halfBase;
                                let hi = (c_sub >> halfShift) + kSurrogateHighStart;
                                let lo = (c_sub & halfMask) + kSurrogateLowStart;
                                let output = std::slice::from_raw_parts_mut(outBuffer.add(outPtr as usize), 4);
                                output[..2].copy_from_slice(&(hi as u16).to_le_bytes());
                                output[2..].copy_from_slice(&(lo as u16).to_le_bytes());
                                outPtr += 4;
                            }
                        }
                        kForm_UTF32BE => {
                            if outPtr + 4 > outLength {
                                (*cnv).pendingOutputChar = c;
                                rval = kStatus_OutputBufferFull;
                                break 'convert_loop;
                            }
                            std::slice::from_raw_parts_mut(outBuffer.add(outPtr as usize), 4).copy_from_slice(&c.to_be_bytes());
                            outPtr += 4;
                        }
                        kForm_UTF32LE => {
                            if outPtr + 4 > outLength {
                                (*cnv).pendingOutputChar = c;
                                rval = kStatus_OutputBufferFull;
                                break 'convert_loop;
                            }
                            std::slice::from_raw_parts_mut(outBuffer.add(outPtr as usize), 4).copy_from_slice(&c.to_le_bytes());
                            outPtr += 4;
                        }
                        _ => {}
                    },
                }
            }

            if !inUsed.is_null() {
                *inUsed = (*cnv).dataPtr;
            }
            if !outUsed.is_null() {
                *outUsed = outPtr;
            }
            if !lookaheadCount.is_null() {
                *lookaheadCount = 0;
                let self_dyn = cnv as *mut dyn Stage;
                let mut s = (*cnv).finalStage;
                while !s.is_null() && !std::ptr::addr_eq(s, self_dyn) {
                    *lookaheadCount += (*s).lookaheadCount();
                    s = (*s).getPrevStage();
                }
            }

            rval |= (*cnv).warningStatus as TECkit_Status;
            if (rval & kStatusMask_Basic) == kStatus_NoError {
                (*cnv).Reset();
            }

            rval
        }
    }

    pub unsafe fn Reset(&mut self) {
        unsafe {
            self.pendingOutputChar = kInvalidChar;
            self.savedCount = 0;
            self.dataPtr = 0;
            self.dataLen = 0;
            self.warningStatus = 0;
            let self_dyn = self as *mut Converter as *mut dyn Stage;
            let mut s = self.finalStage;
            while !s.is_null() && !std::ptr::addr_eq(s, self_dyn) {
                (*s).Reset();
                s = (*s).getPrevStage();
            }
        }
    }

    pub unsafe fn Validate(cnv: *const Converter) -> bool {
        unsafe {
            if cnv.is_null() {
                return false;
            }
            if (*cnv).status != kStatus_NoError {
                return false;
            }
            if !(*cnv).table.is_null() {
                let fh = (*cnv).table as *const FileHeader;
                if READ_u32((*fh).type_) != kMagicNumber {
                    return false;
                }
            }
            true
        }
    }
}

impl Drop for Converter {
    fn drop(&mut self) {
        unsafe {
            let self_dyn = self as *mut Converter as *mut dyn Stage;
            let mut curr = self.finalStage;
            while !curr.is_null() && !std::ptr::addr_eq(curr, self_dyn) {
                let prev = (*curr).getPrevStage();
                let _ = Box::from_raw(curr);
                curr = prev;
            }
            if !self.table.is_null() {
                libc::free(self.table as *mut libc::c_void);
                self.table = std::ptr::null_mut();
            }
        }
    }
}

impl Stage for Converter {
    unsafe fn getChar(&mut self) -> UInt32 {
        unsafe {
            if self.dataPtr >= self.savedCount + self.dataLen {
                return if self.inputComplete { kEndOfText } else { kNeedMoreInput };
            }
            if self.inputForm == kForm_Bytes as Byte {
                let ch = *self.data.add(self.dataPtr as usize);
                self.dataPtr += 1;
                return ch as UInt32;
            }
            self._getCharFn()
        }
    }

    unsafe fn Reset(&mut self) {
        unsafe {
            self.Reset();
        }
    }

    unsafe fn lookaheadCount(&self) -> UInt32 {
        0
    }

    unsafe fn getPrevStage(&self) -> *mut dyn Stage {
        std::ptr::null_mut::<Converter>() as *mut dyn Stage
    }

    unsafe fn setPrevStage(&mut self, _prev: *mut dyn Stage) {}
}

pub unsafe fn getNamePtrFromTable(table: *const Byte, nameID: UInt16, outNamePtr: *mut *const Byte, outNameLen: *mut UInt32) -> bool {
    unsafe {
        let fh = table as *const FileHeader;
        let nameOffsets = table.add(std::mem::size_of::<FileHeader>()) as *const UInt32;
        for i in 0..READ_u32((*fh).numNames) {
            let n = table.add(READ_u32(*nameOffsets.add(i as usize)) as usize) as *const NameRec;
            if READ_u16((*n).nameID) == nameID {
                *outNameLen = READ_u16((*n).nameLength) as UInt32;
                *outNamePtr = (n as *const Byte).add(std::mem::size_of::<NameRec>());
                return true;
            }
        }
        false
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_CreateConverter")]
pub unsafe extern "C" fn TECkit_CreateConverter(
    mapping: *mut Byte, mappingSize: UInt32, mapForward: Byte, inputForm: UInt16, outputForm: UInt16, converter: *mut TECkit_Converter,
) -> TECkit_Status {
    unsafe {
        *converter = std::ptr::null_mut();
        let cnv = Converter::new(mapping, mappingSize, mapForward != 0, inputForm, outputForm);
        let status = cnv.creationStatus();
        if status == kStatus_NoError {
            *converter = Box::into_raw(cnv) as *mut Opaque_TECkit_Converter;
        }
        status
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_DisposeConverter")]
pub unsafe extern "C" fn TECkit_DisposeConverter(converter: TECkit_Converter) -> TECkit_Status {
    unsafe {
        let cnv = converter as *mut Converter;
        if !Converter::Validate(cnv) {
            kStatus_InvalidConverter
        } else {
            drop(Box::from_raw(cnv));
            kStatus_NoError
        }
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_GetConverterName")]
pub unsafe extern "C" fn TECkit_GetConverterName(
    converter: TECkit_Converter, nameID: UInt16, nameBuffer: *mut Byte, bufferSize: UInt32, nameLength: *mut UInt32,
) -> TECkit_Status {
    unsafe {
        let cnv = converter as *mut Converter;
        if !Converter::Validate(cnv) {
            return kStatus_InvalidConverter;
        }
        let mut namePtr: *const Byte = std::ptr::null();
        if (*cnv).GetNamePtr(nameID, &mut namePtr, nameLength) {
            let copyBytes = std::cmp::min(*nameLength, bufferSize);
            if copyBytes > 0 {
                std::ptr::copy_nonoverlapping(namePtr, nameBuffer, copyBytes as usize);
            }
            kStatus_NoError
        } else {
            kStatus_NameNotFound
        }
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_GetConverterFlags")]
pub unsafe extern "C" fn TECkit_GetConverterFlags(converter: TECkit_Converter, sourceFlags: *mut UInt32, targetFlags: *mut UInt32) -> TECkit_Status {
    unsafe {
        let cnv = converter as *mut Converter;
        if !Converter::Validate(cnv) {
            kStatus_InvalidConverter
        } else {
            (*cnv).GetFlags(sourceFlags, targetFlags);
            kStatus_NoError
        }
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_ResetConverter")]
pub unsafe extern "C" fn TECkit_ResetConverter(converter: TECkit_Converter) -> TECkit_Status {
    unsafe {
        let cnv = converter as *mut Converter;
        if !Converter::Validate(cnv) {
            kStatus_InvalidConverter
        } else {
            (*cnv).Reset();
            kStatus_NoError
        }
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_ConvertBufferOpt")]
pub unsafe extern "C" fn TECkit_ConvertBufferOpt(
    converter: TECkit_Converter, inBuffer: *const Byte, inLength: UInt32, inUsed: *mut UInt32, outBuffer: *mut Byte, outLength: UInt32,
    outUsed: *mut UInt32, inOptions: UInt32, lookaheadCount: *mut UInt32,
) -> TECkit_Status {
    unsafe {
        let cnv = converter as *mut Converter;
        if !Converter::Validate(cnv) {
            kStatus_InvalidConverter
        } else {
            Converter::ConvertBufferOpt(cnv, inBuffer, inLength, inUsed, outBuffer, outLength, outUsed, inOptions, lookaheadCount)
        }
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_ConvertBuffer")]
pub unsafe extern "C" fn TECkit_ConvertBuffer(
    converter: TECkit_Converter, inBuffer: *const Byte, inLength: UInt32, inUsed: *mut UInt32, outBuffer: *mut Byte, outLength: UInt32,
    outUsed: *mut UInt32, inputIsComplete: Byte,
) -> TECkit_Status {
    unsafe {
        TECkit_ConvertBufferOpt(
            converter,
            inBuffer,
            inLength,
            inUsed,
            outBuffer,
            outLength,
            outUsed,
            (kOptionsUnmapped_UseReplacementCharSilently as UInt32)
                + if inputIsComplete != 0 {
                    kOptionsComplete_InputIsComplete as UInt32
                } else {
                    0
                },
            std::ptr::null_mut(),
        )
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_FlushOpt")]
pub unsafe extern "C" fn TECkit_FlushOpt(
    converter: TECkit_Converter, outBuffer: *mut Byte, outLength: UInt32, outUsed: *mut UInt32, inOptions: UInt32, lookaheadCount: *mut UInt32,
) -> TECkit_Status {
    unsafe {
        let cnv = converter as *mut Converter;
        if !Converter::Validate(cnv) {
            kStatus_InvalidConverter
        } else {
            Converter::ConvertBufferOpt(
                cnv,
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                outBuffer,
                outLength,
                outUsed,
                inOptions | (kOptionsComplete_InputIsComplete as UInt32),
                lookaheadCount,
            )
        }
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_Flush")]
pub unsafe extern "C" fn TECkit_Flush(converter: TECkit_Converter, outBuffer: *mut Byte, outLength: UInt32, outUsed: *mut UInt32) -> TECkit_Status {
    unsafe {
        TECkit_FlushOpt(
            converter,
            outBuffer,
            outLength,
            outUsed,
            kOptionsUnmapped_UseReplacementCharSilently as UInt32,
            std::ptr::null_mut(),
        )
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_GetMappingFlags")]
pub unsafe extern "C" fn TECkit_GetMappingFlags(
    mapping: *mut Byte, mappingSize: UInt32, lhsFlags: *mut UInt32, rhsFlags: *mut UInt32,
) -> TECkit_Status {
    unsafe {
        if mapping.is_null() {
            return kStatus_InvalidMapping;
        }
        let mut fh = mapping as *const FileHeader;
        let mut header = std::mem::MaybeUninit::<FileHeader>::uninit();
        let mut status = kStatus_NoError;

        if READ_u32((*fh).type_) == kMagicNumberCmp {
            let mut uncompressedLen = std::mem::size_of::<FileHeader>() as u64;
            let res = tectonic_bridge_flate::tectonic_flate_decompress(
                header.as_mut_ptr() as *mut Byte,
                &mut uncompressedLen,
                mapping.add(2 * std::mem::size_of::<UInt32>()),
                (mappingSize as usize - 2 * std::mem::size_of::<UInt32>()) as u64,
            );
            if !matches!(
                res,
                tectonic_bridge_flate::FlateResult::Success | tectonic_bridge_flate::FlateResult::BufError
            ) {
                status = kStatus_InvalidMapping;
            }
            fh = header.as_ptr();
        }

        if status == kStatus_NoError && READ_u32((*fh).type_) == kMagicNumber {
            if (READ_u32((*fh).version) & 0xFFFF0000) > (kCurrentFileVersion & 0xFFFF0000) {
                status = kStatus_BadMappingVersion;
            } else {
                *lhsFlags = READ_u32((*fh).formFlagsLHS);
                *rhsFlags = READ_u32((*fh).formFlagsRHS);
            }
        } else {
            status = kStatus_InvalidMapping;
        }

        status
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_GetMappingName")]
pub unsafe extern "C" fn TECkit_GetMappingName(
    mapping: *mut Byte, mappingSize: UInt32, nameID: UInt16, nameBuffer: *mut Byte, bufferSize: UInt32, nameLength: *mut UInt32,
) -> TECkit_Status {
    unsafe {
        if mapping.is_null() {
            return kStatus_InvalidMapping;
        }
        let mut buf: *mut libc::c_void = std::ptr::null_mut();
        let mut status = kStatus_NoError;
        let mut fh = mapping as *const FileHeader;
        let mut header = std::mem::MaybeUninit::<FileHeader>::uninit();

        if READ_u32((*fh).type_) == kMagicNumberCmp {
            let mut uncompressedLen = std::mem::size_of::<FileHeader>() as u64;
            let res = tectonic_bridge_flate::tectonic_flate_decompress(
                header.as_mut_ptr() as *mut Byte,
                &mut uncompressedLen,
                mapping.add(2 * std::mem::size_of::<UInt32>()),
                (mappingSize as usize - 2 * std::mem::size_of::<UInt32>()) as u64,
            );
            if !matches!(
                res,
                tectonic_bridge_flate::FlateResult::Success | tectonic_bridge_flate::FlateResult::BufError
            ) {
                status = kStatus_InvalidMapping;
            } else {
                fh = header.as_ptr();
                uncompressedLen = READ_u32((*fh).headerLength) as u64;
                buf = libc::malloc(uncompressedLen as usize);
                if buf.is_null() {
                    status = kStatus_OutOfMemory;
                } else {
                    let res2 = tectonic_bridge_flate::tectonic_flate_decompress(
                        buf as *mut Byte,
                        &mut uncompressedLen,
                        mapping.add(2 * std::mem::size_of::<UInt32>()),
                        (mappingSize as usize - 2 * std::mem::size_of::<UInt32>()) as u64,
                    );
                    if !matches!(
                        res2,
                        tectonic_bridge_flate::FlateResult::Success | tectonic_bridge_flate::FlateResult::BufError
                    ) {
                        status = kStatus_InvalidMapping;
                    }
                    fh = buf as *const FileHeader;
                }
            }
        }

        if status == kStatus_NoError && READ_u32((*fh).type_) == kMagicNumber {
            if (READ_u32((*fh).version) & 0xFFFF0000) > (kCurrentFileVersion & 0xFFFF0000) {
                status = kStatus_BadMappingVersion;
            } else {
                let mut namePtr: *const Byte = std::ptr::null();
                if getNamePtrFromTable(fh as *const Byte, nameID, &mut namePtr, nameLength) {
                    let copyBytes = std::cmp::min(*nameLength, bufferSize);
                    if copyBytes > 0 {
                        std::ptr::copy_nonoverlapping(namePtr, nameBuffer, copyBytes as usize);
                    }
                } else {
                    status = kStatus_NameNotFound;
                }
            }
        } else {
            status = kStatus_InvalidMapping;
        }

        if !buf.is_null() {
            libc::free(buf);
        }

        status
    }
}

#[unsafe(export_name = "tectonic_rust_TECkit_GetVersion")]
pub unsafe extern "C" fn TECkit_GetVersion() -> UInt32 {
    kCurrentTECkitVersion
}
