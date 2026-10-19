#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use super::teckit_common::*;
use std::ffi::{c_char, c_int, c_void};

pub const AAT_FONT_FLAG: u32 = 0xFFFF;
pub const OTGR_FONT_FLAG: u32 = 0xFFFE;

pub const pdfbox_crop: i32 = 1;
pub const pdfbox_media: i32 = 2;
pub const pdfbox_bleed: i32 = 3;
pub const pdfbox_trim: i32 = 4;
pub const pdfbox_art: i32 = 5;
pub const pdfbox_none: i32 = 6;

// XeTeX extension commands (code - XETEX_INT where XETEX_INT = 27)
pub const XeTeX_count_glyphs: i32 = 28 - 27;
pub const XeTeX_count_features: i32 = 35 - 27;
pub const XeTeX_feature_code: i32 = 36 - 27;
pub const XeTeX_find_feature_by_name: i32 = 37 - 27;
pub const XeTeX_is_exclusive_feature: i32 = 38 - 27;
pub const XeTeX_count_selectors: i32 = 39 - 27;
pub const XeTeX_selector_code: i32 = 40 - 27;
pub const XeTeX_find_selector_by_name: i32 = 41 - 27;
pub const XeTeX_is_default_selector: i32 = 42 - 27;
pub const XeTeX_OT_count_scripts: i32 = 43 - 27;
pub const XeTeX_OT_count_languages: i32 = 44 - 27;
pub const XeTeX_OT_count_features: i32 = 45 - 27;
pub const XeTeX_OT_script_code: i32 = 46 - 27;
pub const XeTeX_OT_language_code: i32 = 47 - 27;
pub const XeTeX_OT_feature_code: i32 = 48 - 27;
pub const XeTeX_map_char_to_glyph_code: i32 = 49 - 27;

pub const XETEX_FEATURE_NAME_CODE: i32 = 35;
pub const XETEX_SELECTOR_NAME_CODE: i32 = 36;

pub const width_offset: usize = 1;
pub const depth_offset: usize = 2;
pub const height_offset: usize = 3;
pub const native_info_offset: usize = 4;
pub const native_glyph_info_offset: usize = 5;
pub const native_glyph_info_size: usize = 10;
pub const NATIVE_NODE_SIZE: usize = 6;

pub const FONT_BASE: i32 = 0;
pub const LEFT_SIDE: i32 = 0;
pub const RIGHT_SIDE: i32 = 1;

pub const FONT_FLAGS_COLORED: u32 = 0x01;
pub const FONT_FLAGS_VERTICAL: u32 = 0x02;

pub const XDV_FLAG_VERTICAL: u16 = 0x0100;
pub const XDV_FLAG_COLORED: u16 = 0x0200;
pub const XDV_FLAG_EXTEND: u16 = 0x1000;
pub const XDV_FLAG_SLANT: u16 = 0x2000;
pub const XDV_FLAG_EMBOLDEN: u16 = 0x4000;

pub const AUTO: i32 = 0;
pub const UTF8: i32 = 1;
pub const UTF16BE: i32 = 2;
pub const UTF16LE: i32 = 3;
pub const RAW: i32 = 4;
pub const ICUMAPPING: i32 = 5;

pub const kGSUB: u32 = u32::from_be_bytes(*b"GSUB");
pub const kGPOS: u32 = u32::from_be_bytes(*b"GPOS");
pub const HB_TAG_NONE: u32 = 0;

pub const UBRK_LINE: i32 = 2;
pub const U_ZERO_ERROR: i32 = 0;
pub const UBIDI_MIXED: i32 = 2;
pub const UBIDI_RTL: i32 = 1;

pub type Fixed = i32;
pub type scaled_t = i32;
pub type UErrorCode = i32;
pub type UBreakIterator = c_void;
pub type UConverter = c_void;
pub type UBiDi = c_void;
pub type UChar = u16;
pub type RawPlatformFontRef = *mut c_void;
pub type XeTeXFont = *mut c_void;
pub type XeTeXLayoutEngine = *mut c_void;
pub type PreparedLayouts = super::xetex_linebreak::PreparedLayouts;
pub type rust_input_handle_t = usize;
pub type ttbc_file_format = c_int;
pub type TECkit_Converter = *mut c_void;

pub const TTBC_FILE_FORMAT_MISC_FONTS: ttbc_file_format = 41;
pub const TTBC_FILE_FORMAT_OPEN_TYPE: ttbc_file_format = 47;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct FixedPoint {
    pub x: Fixed,
    pub y: Fixed,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct GlyphBBox {
    pub xMin: f32,
    pub yMin: f32,
    pub xMax: f32,
    pub yMax: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct FloatPoint {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct hb_feature_t {
    pub tag: u32,
    pub value: u32,
    pub start: libc::c_uint,
    pub end: libc::c_uint,
}

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b32x2 {
    pub s1: i32,
    pub s0: i32,
}

#[cfg(not(target_endian = "big"))]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b32x2 {
    pub s0: i32,
    pub s1: i32,
}

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b16x4 {
    pub s3: u16,
    pub s2: u16,
    pub s1: u16,
    pub s0: u16,
}

#[cfg(not(target_endian = "big"))]
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct b16x4 {
    pub s0: u16,
    pub s1: u16,
    pub s2: u16,
    pub s3: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: f64,
    pub ptr: *mut c_void,
}

impl Default for memory_word {
    fn default() -> Self {
        memory_word {
            b32: b32x2::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct tt_checkpoint {
    pub bytes: *mut u8,
    pub length: usize,
    pub capacity: usize,
    pub position: usize,
    pub restoring: bool,
    pub valid: bool,
}

// Mac types and stubs
pub type CFDictionaryRef = *const c_void;
pub type CTFontDescriptorRef = *const c_void;
pub type CTFontRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFArrayRef = *const c_void;
pub type CFNumberRef = *const c_void;
pub type CFBooleanRef = *const c_void;
pub type CGColorRef = *const c_void;
pub type CGFloat = f64;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CGAffineTransform {
    pub a: CGFloat,
    pub b: CGFloat,
    pub c: CGFloat,
    pub d: CGFloat,
    pub tx: CGFloat,
    pub ty: CGFloat,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CFRange {
    pub location: libc::c_long,
    pub length: libc::c_long,
}

#[inline]
pub fn CFRangeMake(loc: libc::c_long, len: libc::c_long) -> CFRange {
    CFRange {
        location: loc,
        length: len,
    }
}

pub const kCFNumberIntType: c_int = 9;
pub const kCFNumberFloatType: c_int = 12;

// Memory word accessors
#[inline]
pub unsafe fn node_width(node: *mut memory_word) -> *mut i32 {
    unsafe { &mut (*node.add(width_offset)).b32.s1 }
}

#[inline]
pub unsafe fn node_depth(node: *mut memory_word) -> *mut i32 {
    unsafe { &mut (*node.add(depth_offset)).b32.s1 }
}

#[inline]
pub unsafe fn node_height(node: *mut memory_word) -> *mut i32 {
    unsafe { &mut (*node.add(height_offset)).b32.s1 }
}

#[inline]
pub unsafe fn native_length(node: *mut memory_word) -> *mut u16 {
    unsafe { &mut (*node.add(native_info_offset)).b16.s1 }
}

#[inline]
pub unsafe fn native_font(node: *mut memory_word) -> *mut u16 {
    unsafe { &mut (*node.add(native_info_offset)).b16.s2 }
}

#[inline]
pub unsafe fn native_glyph_count(node: *mut memory_word) -> *mut u16 {
    unsafe { &mut (*node.add(native_info_offset)).b16.s0 }
}

#[inline]
pub unsafe fn native_glyph_info_ptr(node: *mut memory_word) -> *mut *mut c_void {
    unsafe { &mut (*node.add(native_glyph_info_offset)).ptr }
}

#[inline]
pub unsafe fn native_glyph(p: *mut memory_word) -> *mut u16 {
    unsafe { native_length(p) }
}

#[inline]
unsafe fn QUAD(f: i32) -> scaled_t {
    unsafe {
        let idx = (6 + *param_base.add(f as usize)) as usize;
        (*font_info.add(idx)).b32.s1
    }
}

#[inline]
unsafe fn X_HEIGHT(f: i32) -> scaled_t {
    unsafe {
        let idx = (5 + *param_base.add(f as usize)) as usize;
        (*font_info.add(idx)).b32.s1
    }
}

#[inline]
unsafe fn CAP_HEIGHT(f: i32) -> scaled_t {
    unsafe {
        let idx = (8 + *param_base.add(f as usize)) as usize;
        (*font_info.add(idx)).b32.s1
    }
}

// Byte swap functions
#[inline]
pub fn SWAP16(p: u16) -> u16 {
    p.to_be()
}

#[inline]
pub fn SWAP32(p: u32) -> u32 {
    p.to_be()
}

// Helper utility functions
#[inline]
unsafe fn strstartswith(s: *const c_char, prefix: *const c_char) -> *const c_char {
    unsafe {
        let len = libc::strlen(prefix);
        if libc::strncmp(s, prefix, len) == 0 {
            s.add(len)
        } else {
            std::ptr::null()
        }
    }
}

#[inline]
unsafe fn streq_ptr(s1: *const c_char, s2: *const c_char) -> bool {
    unsafe { libc::strcmp(s1, s2) == 0 }
}

#[inline]
unsafe fn mfree<T>(ptr: *mut T) -> *mut T {
    unsafe {
        if !ptr.is_null() {
            libc::free(ptr as *mut c_void);
        }
        std::ptr::null_mut()
    }
}

// External globals from engine / TeX core
unsafe extern "C" {
    pub static mut name_of_file: *mut c_char;
    pub static mut name_length: c_int;
    pub static mut font_area: *mut i32;
    pub static mut font_layout_engine: *mut *mut c_void;
    pub static mut font_mapping: *mut *mut c_void;
    pub static mut font_flags: *mut c_char;
    pub static mut font_letter_space: *mut scaled_t;
    pub static mut font_size: *mut scaled_t;
    pub static mut font_name: *mut i32;
    pub static mut font_ptr: i32;

    pub static mut loaded_font_mapping: *mut c_void;
    pub static mut loaded_font_flags: c_char;
    pub static mut loaded_font_letter_space: scaled_t;
    pub static mut native_font_type_flag: i32;
    pub static mut xdv_buffer: *mut c_char;
    pub static mut mapped_text: *mut u16;

    pub static mut font_info: *mut memory_word;
    pub static mut param_base: *mut i32;
    pub static mut height_base: *mut scaled_t;
    pub static mut depth_base: *mut scaled_t;
    pub static mut mem: *mut memory_word;

    pub static tt_checkpoint_enabled: bool;
}

// External functions from bridge_core, xetex_layout, ICU, HarfBuzz, Graphite, TeX
unsafe extern "C" {
    pub fn gettexstring(s: i32) -> *mut c_char;
    pub fn maketexstring(s: *const c_char) -> i32;
    pub fn begin_diagnostic();
    pub fn end_diagnostic(blank_line: bool);
    pub fn print_nl(c: i32);
    pub fn print_int(n: i32);
    #[link_name = "print_cstr"]
    pub fn print_c_string(s: *const c_char);
    pub fn print_char(c: i32);
    pub fn print_raw_char(c: u16, in_hex: bool);
    pub fn font_mapping_warning(name: *const c_void, len: i32, warningType: i32);
    pub fn font_feature_warning(
        name: *const c_void,
        len: i32,
        settingNameP: *const c_void,
        setLen: i32,
    );
    pub fn get_tracing_fonts_state() -> i32;
    pub fn xn_over_d(x: scaled_t, n: i32, d: i32) -> scaled_t;

    pub fn xmalloc(size: usize) -> *mut c_void;
    pub fn xcalloc(count: usize, size: usize) -> *mut c_void;
    pub fn xrealloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn xstrdup(s: *const c_char) -> *mut c_char;

    pub fn ttstub_input_open(
        path: *const c_char,
        format: ttbc_file_format,
        is_gz: c_int,
    ) -> rust_input_handle_t;
    pub fn ttstub_input_get_size(handle: rust_input_handle_t) -> usize;
    pub fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut c_char,
        len: usize,
    ) -> libc::ssize_t;
    pub fn ttstub_input_read_partial(
        handle: rust_input_handle_t,
        data: *mut c_char,
        len: usize,
    ) -> libc::ssize_t;
    pub fn ttstub_input_close(handle: rust_input_handle_t) -> c_int;
    pub fn _tt_abort(format: *const c_char, ...) -> !;

    pub fn tt_checkpoint_field(
        cp: *mut c_void,
        name: *const c_char,
        value: *mut c_void,
        length: usize,
    );
    pub fn tt_checkpoint_register_glyph(node: i32);

    // ICU BreakIterator
    #[link_name = "tt_icu_ubrk_open"]
    pub fn ubrk_open(
        type_: c_int,
        locale: *const c_char,
        text: *const UChar,
        textLength: i32,
        status: *mut UErrorCode,
    ) -> *mut UBreakIterator;
    #[link_name = "tt_icu_ubrk_close"]
    pub fn ubrk_close(bi: *mut UBreakIterator);
    #[link_name = "tt_icu_ubrk_set_text"]
    pub fn ubrk_setText(
        bi: *mut UBreakIterator,
        text: *const UChar,
        textLength: i32,
        status: *mut UErrorCode,
    );
    #[link_name = "tt_icu_ubrk_next"]
    pub fn ubrk_next(bi: *mut UBreakIterator) -> i32;

    // ICU Converter
    #[link_name = "tt_icu_ucnv_open"]
    pub fn ucnv_open(converterName: *const c_char, err: *mut UErrorCode) -> *mut UConverter;
    #[link_name = "tt_icu_ucnv_close"]
    pub fn ucnv_close(converter: *mut UConverter);

    // ICU BiDi
    #[link_name = "tt_icu_ubidi_open"]
    pub fn ubidi_open() -> *mut UBiDi;
    #[link_name = "tt_icu_ubidi_close"]
    pub fn ubidi_close(pBiDi: *mut UBiDi);
    #[link_name = "tt_icu_ubidi_set_para"]
    pub fn ubidi_setPara(
        pBiDi: *mut UBiDi,
        text: *const UChar,
        length: i32,
        paraLevel: u8,
        embeddingLevels: *mut c_void,
        pErrorCode: *mut UErrorCode,
    );
    #[link_name = "tt_icu_ubidi_get_direction"]
    pub fn ubidi_getDirection(pBiDi: *const UBiDi) -> c_int;
    #[link_name = "tt_icu_ubidi_count_runs"]
    pub fn ubidi_countRuns(pBiDi: *mut UBiDi, pErrorCode: *mut UErrorCode) -> c_int;
    #[link_name = "tt_icu_ubidi_get_visual_run"]
    pub fn ubidi_getVisualRun(
        pBiDi: *mut UBiDi,
        runIndex: c_int,
        pLogicalStart: *mut i32,
        pLength: *mut i32,
    ) -> c_int;

    // HarfBuzz
    pub fn hb_tag_from_string(str: *const c_char, len: c_int) -> u32;

    // Graphite
    pub fn gr_label_destroy(label: *mut c_void);

    // TECkit
    pub fn TECkit_CreateConverter(
        mapping: *mut u8,
        mappingSize: u32,
        mapForward: u8,
        sourceForm: u16,
        targetForm: u16,
        converter: *mut TECkit_Converter,
    ) -> TECkit_Status;
    pub fn TECkit_DisposeConverter(converter: TECkit_Converter) -> TECkit_Status;
    pub fn TECkit_ResetConverter(converter: TECkit_Converter) -> TECkit_Status;
    pub fn TECkit_ConvertBuffer(
        converter: TECkit_Converter,
        inBuffer: *const u8,
        inLength: u32,
        inUsed: *mut u32,
        outBuffer: *mut u8,
        outLength: u32,
        outUsed: *mut u32,
        inputIsComplete: u8,
    ) -> TECkit_Status;

    // xetex_layout engine & font functions
    pub fn initGraphiteBreaking(
        engine: XeTeXLayoutEngine,
        text: *const u16,
        textLength: i32,
    ) -> bool;
    pub fn findNextGraphiteBreak(engine: XeTeXLayoutEngine) -> c_int;
    pub fn getReqEngine() -> c_char;
    pub fn setReqEngine(req: c_char);
    pub fn set_loaded_font_design_size(size: Fixed);

    pub fn createFont(fontRef: RawPlatformFontRef, point_size: Fixed) -> XeTeXFont;
    pub fn createFontFromFile(filename: *const c_char, index: u32, point_size: Fixed) -> XeTeXFont;
    pub fn deleteFont(font: XeTeXFont);
    pub fn getDesignSize(font: XeTeXFont) -> f64;
    pub fn findFontByName(name: *const c_char, var: *const c_char, size: f64)
    -> RawPlatformFontRef;
    pub fn getFullName(font: RawPlatformFontRef) -> *const c_char;
    pub fn ttxl_platfont_get_desc(font: RawPlatformFontRef) -> *const c_char;

    pub fn hasFontTable(font: XeTeXFont, table_tag: u32) -> bool;
    pub fn createLayoutEngine(
        font: XeTeXFont,
        script: u32,
        language: *mut c_char,
        features: *mut hb_feature_t,
        n_features: c_int,
        shapers: *const *const c_char,
        rgb_value: u32,
        extend: f32,
        slant: f32,
        embolden: f32,
    ) -> XeTeXLayoutEngine;
    pub fn createLayoutEngineBorrowed(
        font: XeTeXFont,
        script: u32,
        language: *mut c_char,
        features: *mut hb_feature_t,
        n_features: c_int,
        shapers: *const *const c_char,
        rgb_value: u32,
        extend: f32,
        slant: f32,
        embolden: f32,
    ) -> XeTeXLayoutEngine;
    pub fn deleteLayoutEngine(this_: XeTeXLayoutEngine);
    pub fn getFont(engine: XeTeXLayoutEngine) -> XeTeXFont;
    pub fn getExtendFactor(engine: XeTeXLayoutEngine) -> f32;
    pub fn getSlantFactor(engine: XeTeXLayoutEngine) -> f32;
    pub fn getEmboldenFactor(engine: XeTeXLayoutEngine) -> f32;
    pub fn getPointSize(engine: XeTeXLayoutEngine) -> f32;
    pub fn getAscentAndDescent(engine: XeTeXLayoutEngine, ascent: *mut f32, descent: *mut f32);
    pub fn getCapAndXHeight(engine: XeTeXLayoutEngine, capheight: *mut f32, xheight: *mut f32);
    pub fn getDefaultDirection(engine: XeTeXLayoutEngine) -> c_int;
    pub fn getRgbValue(engine: XeTeXLayoutEngine) -> u32;
    pub fn getGlyphBounds(engine: XeTeXLayoutEngine, glyph_id: u32, bbox: *mut GlyphBBox);
    pub fn getGlyphWidthFromEngine(engine: XeTeXLayoutEngine, glyph_id: u32) -> f32;
    pub fn getGlyphHeightDepth(
        engine: XeTeXLayoutEngine,
        glyph_id: u32,
        height: *mut f32,
        depth: *mut f32,
    );
    pub fn getGlyphSidebearings(
        engine: XeTeXLayoutEngine,
        glyph_id: u32,
        lsb: *mut f32,
        rsb: *mut f32,
    );
    pub fn getGlyphItalCorr(engine: XeTeXLayoutEngine, glyph_id: u32) -> f32;
    pub fn mapCharToGlyph(engine: XeTeXLayoutEngine, char_code: u32) -> u32;
    pub fn getFontCharRange(engine: XeTeXLayoutEngine, req_first: c_int) -> c_int;
    pub fn mapGlyphToIndex(engine: XeTeXLayoutEngine, glyph_name: *const c_char) -> c_int;
    pub fn usingGraphite(engine: XeTeXLayoutEngine) -> bool;
    pub fn usingOpenType(engine: XeTeXLayoutEngine) -> bool;
    pub fn layoutChars(
        engine: XeTeXLayoutEngine,
        chars: *mut u16,
        offset: i32,
        count: i32,
        max: i32,
        rtl: bool,
    ) -> c_int;
    pub fn getFontFilename(engine: XeTeXLayoutEngine, index: *mut u32) -> *const c_char;
    pub fn freeFontFilename(filename: *const c_char);
    pub fn getGlyphs(engine: XeTeXLayoutEngine, glyphs: *mut u32);
    pub fn getGlyphAdvances(engine: XeTeXLayoutEngine, advances: *mut f32);
    pub fn getGlyphPositions(engine: XeTeXLayoutEngine, positions: *mut FloatPoint);
    pub fn countGraphiteFeatures(engine: XeTeXLayoutEngine) -> u32;
    pub fn getGraphiteFeatureCode(engine: XeTeXLayoutEngine, index: u32) -> u32;
    pub fn countGraphiteFeatureSettings(engine: XeTeXLayoutEngine, feature_id: u32) -> u32;
    pub fn getGraphiteFeatureSettingCode(
        engine: XeTeXLayoutEngine,
        feature_id: u32,
        index: u32,
    ) -> u32;
    pub fn getGraphiteFeatureDefaultSetting(engine: XeTeXLayoutEngine, feature_id: u32) -> u32;
    pub fn getGraphiteFeatureLabel(engine: XeTeXLayoutEngine, feature_id: u32) -> *mut c_char;
    pub fn getGraphiteFeatureSettingLabel(
        engine: XeTeXLayoutEngine,
        feature_id: u32,
        setting_id: u32,
    ) -> *mut c_char;
    pub fn findGraphiteFeature(
        engine: XeTeXLayoutEngine,
        s: *const c_char,
        e: *const c_char,
        f: *mut u32,
        v: *mut c_int,
    ) -> bool;
    pub fn findGraphiteFeatureNamed(
        engine: XeTeXLayoutEngine,
        name: *const c_char,
        namelength: c_int,
    ) -> libc::c_long;
    pub fn findGraphiteFeatureSettingNamed(
        engine: XeTeXLayoutEngine,
        feature_id: c_int,
        name: *const c_char,
        namelength: c_int,
    ) -> libc::c_long;

    pub fn getSlant(font: XeTeXFont) -> Fixed;
    pub fn countGlyphs(font: XeTeXFont) -> libc::c_uint;
    pub fn getGlyphWidth(font: XeTeXFont, gid: u32) -> f32;
    pub fn setFontLayoutDir(font: XeTeXFont, vertical: c_int);
    pub fn countScripts(font: XeTeXFont) -> libc::c_uint;
    pub fn countLanguages(font: XeTeXFont, script: u32) -> libc::c_uint;
    pub fn countFeatures(font: XeTeXFont, script: u32, language: u32) -> libc::c_uint;
    pub fn getIndScript(font: XeTeXFont, index: libc::c_uint) -> u32;
    pub fn getIndLanguage(font: XeTeXFont, script: u32, index: libc::c_uint) -> u32;
    pub fn getIndFeature(font: XeTeXFont, script: u32, language: u32, index: libc::c_uint) -> u32;
    pub fn getGlyphName(font: XeTeXFont, gid: u32, len: *mut c_int) -> *const c_char;
    pub fn freeGlyphName(name: *mut c_char);
    pub fn getCachedGlyphBBox(font_id: u16, glyph_id: u16, bbox: *mut GlyphBBox) -> i32;
    pub fn cacheGlyphBBox(font_id: u16, glyph_id: u16, bbox: *const GlyphBBox);
    pub fn get_cp_code(font_num: i32, code: u32, side: i32) -> i32;

    pub fn getFileNameFromCTFont(ct_font: CTFontRef, index: *mut u32) -> *const c_char;

    // Declared in xetex-xetex0 / xetex-pic
    pub fn prepare_native_line_layouts(lines: *const i32, count: usize) -> *mut PreparedLayouts;
    pub fn count_pdf_file_pages() -> c_int;
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    pub static kXeTeXEmboldenAttributeName: CFStringRef;
    pub static kCTFontAttributeName: *const c_void;
    pub static kCTVerticalFormsAttributeName: *const c_void;
    pub static kCTForegroundColorAttributeName: *const c_void;
    pub static kCTFontFeatureTypeIdentifierKey: *const c_void;
    pub static kCTFontFeatureTypeExclusiveKey: *const c_void;
    pub static kCTFontFeatureTypeSelectorsKey: *const c_void;
    pub static kCTFontFeatureSelectorIdentifierKey: *const c_void;
    pub static kCTFontFeatureSelectorDefaultKey: *const c_void;
    pub static kCTFontFeatureTypeNameKey: *const c_void;
    pub static kCTFontFeatureSelectorNameKey: *const c_void;

    pub fn loadAATfont(
        descriptor: CTFontDescriptorRef,
        scaled_size: i32,
        cp1: *const c_char,
    ) -> *mut c_void;
    pub fn DoAATLayout(node: *mut c_void, justify: c_int);
    pub fn GetGlyphBBox_AAT(fontAttrs: CFDictionaryRef, gid: u16, bbox: *mut GlyphBBox);
    pub fn GetGlyphWidth_AAT(fontAttrs: CFDictionaryRef, gid: u16) -> f64;
    pub fn GetGlyphHeightDepth_AAT(
        fontAttrs: CFDictionaryRef,
        gid: u16,
        ht: *mut f32,
        dp: *mut f32,
    );
    pub fn GetGlyphSidebearings_AAT(
        fontAttrs: CFDictionaryRef,
        gid: u16,
        lsb: *mut f32,
        rsb: *mut f32,
    );
    pub fn GetGlyphItalCorr_AAT(fontAttrs: CFDictionaryRef, gid: u16) -> f64;
    pub fn MapCharToGlyph_AAT(fontAttrs: CFDictionaryRef, ch: u32) -> c_int;
    pub fn MapGlyphToIndex_AAT(attributes: CFDictionaryRef, glyphName: *const c_char) -> c_int;
    pub fn GetGlyphNameFromCTFont(ctFontRef: CTFontRef, gid: u16, len: *mut c_int)
    -> *const c_char;
    pub fn findDictionaryInArray(
        array: CFArrayRef,
        nameKey: *const c_void,
        name: *const c_char,
        nameLength: c_int,
    ) -> CFDictionaryRef;
    pub fn findDictionaryInArrayWithIdentifier(
        array: CFArrayRef,
        identifierKey: *const c_void,
        identifier: c_int,
    ) -> CFDictionaryRef;
    pub fn findSelectorByName(
        feature: CFDictionaryRef,
        name: *const c_char,
        nameLength: c_int,
    ) -> CFNumberRef;
    pub fn GetFontCharRange_AAT(fontAttrs: CFDictionaryRef, reqFirst: c_int) -> c_int;
    pub fn fontFromAttributes(fontAttrs: CFDictionaryRef) -> CTFontRef;
    pub fn fontFromInteger(font: i32) -> CTFontRef;

    pub fn CFRelease(cf: *const c_void);
    pub fn CFArrayGetCount(theArray: CFArrayRef) -> libc::c_long;
    pub fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: libc::c_long) -> *const c_void;
    pub fn CFDictionaryGetValue(theDict: CFDictionaryRef, key: *const c_void) -> *const c_void;
    pub fn CFDictionaryGetValueIfPresent(
        theDict: CFDictionaryRef,
        key: *const c_void,
        value: *mut *const c_void,
    ) -> u8;
    pub fn CFBooleanGetValue(boolean: CFBooleanRef) -> c_int;
    pub fn CFNumberGetValue(
        number: CFNumberRef,
        theType: libc::c_long,
        valuePtr: *mut c_void,
    ) -> u8;
    pub fn CFStringGetLength(theString: CFStringRef) -> libc::c_long;
    pub fn CFStringGetCharacters(theString: CFStringRef, range: CFRange, buffer: *mut UniChar);

    pub fn CTFontGetGlyphCount(font: CTFontRef) -> libc::c_long;
    pub fn CTFontCopyFeatures(font: CTFontRef) -> CFArrayRef;
    pub fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetXHeight(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetCapHeight(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetSlantAngle(font: CTFontRef) -> CGFloat;
    pub fn CTFontGetMatrix(font: CTFontRef) -> CGAffineTransform;
    pub fn CTFontGetSize(font: CTFontRef) -> CGFloat;
    pub fn CGColorGetComponents(color: CGColorRef) -> *const CGFloat;
}

// Module-level static state
static mut brkIter: *mut UBreakIterator = std::ptr::null_mut();
static mut brkLocaleStrNum: i32 = 0;
static mut saved_mapping_name: *mut c_char = std::ptr::null_mut();
static mut xdvBufSize: i32 = 0;
static mut outLength: u32 = 0;

#[unsafe(export_name = "linebreak_start")]
pub unsafe extern "C" fn linebreak_start(
    f: c_int,
    localeStrNum: i32,
    text: *mut u16,
    textLength: i32,
) {
    unsafe {
        let mut status: UErrorCode = U_ZERO_ERROR;
        let locale = gettexstring(localeStrNum);

        if *font_area.add(f as usize) as u32 == OTGR_FONT_FLAG
            && streq_ptr(locale, b"G\0".as_ptr() as *const c_char)
        {
            let engine = *font_layout_engine.add(f as usize) as XeTeXLayoutEngine;
            if initGraphiteBreaking(engine, text, textLength) {
                libc::free(locale as *mut c_void);
                return;
            }
        }

        if localeStrNum != brkLocaleStrNum && !brkIter.is_null() {
            ubrk_close(brkIter);
            brkIter = std::ptr::null_mut();
        }

        if brkIter.is_null() {
            brkIter = ubrk_open(UBRK_LINE, locale, std::ptr::null(), 0, &mut status);
            if status > 0 {
                begin_diagnostic();
                print_nl(b'E' as i32);
                print_c_string(b"rror \0".as_ptr() as *const c_char);
                print_int(status);
                print_c_string(
                    b" creating linebreak iterator for locale `\0".as_ptr() as *const c_char
                );
                print_c_string(locale);
                print_c_string(b"'; trying default locale `en_us'.\0".as_ptr() as *const c_char);
                end_diagnostic(true);
                if !brkIter.is_null() {
                    ubrk_close(brkIter);
                }
                status = U_ZERO_ERROR;
                brkIter = ubrk_open(
                    UBRK_LINE,
                    b"en_us\0".as_ptr() as *const c_char,
                    std::ptr::null(),
                    0,
                    &mut status,
                );
            }
            libc::free(locale as *mut c_void);
            brkLocaleStrNum = localeStrNum;
        } else {
            libc::free(locale as *mut c_void);
        }

        if brkIter.is_null() {
            _tt_abort(
                b"failed to create linebreak iterator, status=%d\0".as_ptr() as *const c_char,
                status as c_int,
            );
        }

        ubrk_setText(brkIter, text, textLength, &mut status);
    }
}

#[unsafe(export_name = "linebreak_next")]
pub unsafe extern "C" fn linebreak_next(f: c_int) -> c_int {
    unsafe {
        if !brkIter.is_null() {
            ubrk_next(brkIter) as c_int
        } else {
            let engine = *font_layout_engine.add(f as usize) as XeTeXLayoutEngine;
            findNextGraphiteBreak(engine)
        }
    }
}

#[unsafe(export_name = "get_encoding_mode_and_info")]
pub unsafe extern "C" fn get_encoding_mode_and_info(info: *mut i32) -> c_int {
    unsafe {
        let mut err: UErrorCode = U_ZERO_ERROR;
        *info = 0;
        if libc::strcasecmp(name_of_file, b"auto\0".as_ptr() as *const c_char) == 0 {
            return AUTO;
        }
        if libc::strcasecmp(name_of_file, b"utf8\0".as_ptr() as *const c_char) == 0 {
            return UTF8;
        }
        if libc::strcasecmp(name_of_file, b"utf16\0".as_ptr() as *const c_char) == 0 {
            #[cfg(target_endian = "big")]
            return UTF16BE;
            #[cfg(not(target_endian = "big"))]
            return UTF16LE;
        }
        if libc::strcasecmp(name_of_file, b"utf16be\0".as_ptr() as *const c_char) == 0 {
            return UTF16BE;
        }
        if libc::strcasecmp(name_of_file, b"utf16le\0".as_ptr() as *const c_char) == 0 {
            return UTF16LE;
        }
        if libc::strcasecmp(name_of_file, b"bytes\0".as_ptr() as *const c_char) == 0 {
            return RAW;
        }

        let cnv = ucnv_open(name_of_file, &mut err);
        if cnv.is_null() {
            begin_diagnostic();
            print_nl(b'U' as i32);
            print_c_string(b"nknown encoding `\0".as_ptr() as *const c_char);
            print_c_string(name_of_file);
            print_c_string(b"'; reading as raw bytes\0".as_ptr() as *const c_char);
            end_diagnostic(true);
            RAW
        } else {
            ucnv_close(cnv);
            *info = maketexstring(name_of_file);
            ICUMAPPING
        }
    }
}

#[unsafe(export_name = "print_utf8_str")]
pub unsafe extern "C" fn print_utf8_str(mut str: *const u8, mut len: c_int) {
    unsafe {
        while len > 0 {
            len -= 1;
            print_raw_char(u16::from(*str), true);
            str = str.add(1);
        }
    }
}

#[unsafe(export_name = "print_chars")]
pub unsafe extern "C" fn print_chars(mut str: *const u16, mut len: c_int) {
    unsafe {
        while len > 0 {
            len -= 1;
            print_char(i32::from(*str));
            str = str.add(1);
        }
    }
}

unsafe fn load_mapping_file(
    s: *const c_char,
    e: *const c_char,
    byteMapping: c_char,
) -> *mut c_void {
    unsafe {
        let mut cnv: TECkit_Converter = std::ptr::null_mut();
        let name_len = e.offset_from(s) as usize;
        let buffer = xmalloc(name_len + 5) as *mut c_char;

        libc::strncpy(buffer, s, name_len);
        *buffer.add(name_len) = 0;
        libc::strcat(buffer, b".tec\0".as_ptr() as *const c_char);

        let map = ttstub_input_open(buffer, TTBC_FILE_FORMAT_MISC_FONTS, 0);
        if map != 0 {
            let mappingSize = ttstub_input_get_size(map);
            let mapping = xmalloc(mappingSize) as *mut u8;
            let r = ttstub_input_read(map, mapping as *mut c_char, mappingSize);

            if r < 0 || (r as usize) != mappingSize {
                _tt_abort(
                    b"could not read mapping file \"%s\"\0".as_ptr() as *const c_char,
                    buffer,
                );
            }

            ttstub_input_close(map);

            #[cfg(target_endian = "big")]
            let utf16_native = kForm_UTF16BE;
            #[cfg(not(target_endian = "big"))]
            let utf16_native = kForm_UTF16LE;

            if byteMapping != 0 {
                TECkit_CreateConverter(
                    mapping,
                    mappingSize as u32,
                    0,
                    utf16_native,
                    kForm_Bytes,
                    &mut cnv,
                );
            } else {
                TECkit_CreateConverter(
                    mapping,
                    mappingSize as u32,
                    1,
                    utf16_native,
                    utf16_native,
                    &mut cnv,
                );
            }

            let buf_len = libc::strlen(buffer);
            if cnv.is_null() {
                font_mapping_warning(buffer as *const c_void, buf_len as i32, 2);
            } else if get_tracing_fonts_state() > 1 {
                font_mapping_warning(buffer as *const c_void, buf_len as i32, 0);
            }

            libc::free(mapping as *mut c_void);
        } else {
            let buf_len = libc::strlen(buffer);
            font_mapping_warning(buffer as *const c_void, buf_len as i32, 1);
        }

        libc::free(buffer as *mut c_void);
        cnv
    }
}

#[unsafe(export_name = "check_for_tfm_font_mapping")]
pub unsafe extern "C" fn check_for_tfm_font_mapping() {
    unsafe {
        let cp = libc::strstr(name_of_file, b":mapping=\0".as_ptr() as *const c_char);
        saved_mapping_name = mfree(saved_mapping_name);
        if !cp.is_null() {
            *cp = 0;
            let mut p = cp.add(9);
            while *p != 0 && (*p as u8) <= b' ' {
                p = p.add(1);
            }
            if *p != 0 {
                saved_mapping_name = xstrdup(p);
            }
        }
    }
}

#[unsafe(export_name = "load_tfm_font_mapping")]
pub unsafe extern "C" fn load_tfm_font_mapping() -> *mut c_void {
    unsafe {
        let mut rval = std::ptr::null_mut();
        if !saved_mapping_name.is_null() {
            let len = libc::strlen(saved_mapping_name);
            rval = load_mapping_file(saved_mapping_name, saved_mapping_name.add(len), 1);
            saved_mapping_name = mfree(saved_mapping_name);
        }
        rval
    }
}

#[unsafe(export_name = "apply_tfm_font_mapping")]
pub unsafe extern "C" fn apply_tfm_font_mapping(cnv: *mut c_void, c: c_int) -> c_int {
    unsafe {
        let in_char: UniChar = c as UniChar;
        let mut out: [Byte; 2] = [0; 2];
        let mut inUsed: UInt32 = 0;
        let mut outUsed: UInt32 = 0;
        TECkit_ConvertBuffer(
            cnv as TECkit_Converter,
            &in_char as *const UniChar as *const Byte,
            std::mem::size_of::<UniChar>() as UInt32,
            &mut inUsed,
            out.as_mut_ptr(),
            out.len() as UInt32,
            &mut outUsed,
            1,
        );
        TECkit_ResetConverter(cnv as TECkit_Converter);
        if outUsed < 1 { 0 } else { out[0] as c_int }
    }
}

#[unsafe(export_name = "read_double")]
pub unsafe extern "C" fn read_double(s: *mut *const c_char) -> f64 {
    unsafe {
        let mut neg = false;
        let mut val = 0.0f64;
        let mut cp = *s;

        while *cp == b' ' as c_char || *cp == b'\t' as c_char {
            cp = cp.add(1);
        }
        if *cp == b'-' as c_char {
            neg = true;
            cp = cp.add(1);
        } else if *cp == b'+' as c_char {
            cp = cp.add(1);
        }

        while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
            val = val * 10.0 + (*cp as u8 - b'0') as f64;
            cp = cp.add(1);
        }
        if *cp == b'.' as c_char {
            let mut dec = 10.0f64;
            cp = cp.add(1);
            while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                val += (*cp as u8 - b'0') as f64 / dec;
                cp = cp.add(1);
                dec *= 10.0;
            }
        }
        *s = cp;

        if neg { -val } else { val }
    }
}

unsafe fn read_tag_with_param(mut cp: *const c_char, param: *mut c_int) -> u32 {
    unsafe {
        let mut cp2 = cp;
        while *cp2 != 0
            && *cp2 != b':' as c_char
            && *cp2 != b';' as c_char
            && *cp2 != b',' as c_char
            && *cp2 != b'=' as c_char
        {
            cp2 = cp2.add(1);
        }

        let tag = hb_tag_from_string(cp, cp2.offset_from(cp) as c_int);

        cp = cp2;
        if *cp == b'=' as c_char {
            let mut neg = false;
            cp = cp.add(1);
            if *cp == b'-' as c_char {
                neg = true;
                cp = cp.add(1);
            }
            while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                *param = *param * 10 + (*cp as u8 - b'0') as c_int;
                cp = cp.add(1);
            }
            if neg {
                *param = -*param;
            }
        }

        tag
    }
}

#[unsafe(export_name = "read_rgb_a")]
pub unsafe extern "C" fn read_rgb_a(cp: *mut *const c_char) -> libc::c_uint {
    unsafe {
        let mut rgbValue: u32 = 0;
        let mut alpha: u32 = 0;
        let mut p = *cp;

        for _ in 0..6 {
            let ch = *p as u8;
            if ch >= b'0' && ch <= b'9' {
                rgbValue = (rgbValue << 4) + (ch - b'0') as u32;
            } else if ch >= b'A' && ch <= b'F' {
                rgbValue = (rgbValue << 4) + (ch - b'A' + 10) as u32;
            } else if ch >= b'a' && ch <= b'f' {
                rgbValue = (rgbValue << 4) + (ch - b'a' + 10) as u32;
            } else {
                return 0x000000FF;
            }
            p = p.add(1);
        }
        rgbValue <<= 8;

        let mut i = 0;
        while i < 2 {
            let ch = *p as u8;
            if ch >= b'0' && ch <= b'9' {
                alpha = (alpha << 4) + (ch - b'0') as u32;
            } else if ch >= b'A' && ch <= b'F' {
                alpha = (alpha << 4) + (ch - b'A' + 10) as u32;
            } else if ch >= b'a' && ch <= b'f' {
                alpha = (alpha << 4) + (ch - b'a' + 10) as u32;
            } else {
                break;
            }
            p = p.add(1);
            i += 1;
        }

        *cp = p;
        if i == 2 {
            rgbValue += alpha;
        } else {
            rgbValue += 0xFF;
        }
        rgbValue
    }
}

pub unsafe fn readCommonFeatures(
    feat: *const c_char,
    end: *const c_char,
    extend: *mut f32,
    slant: *mut f32,
    embolden: *mut f32,
    letterspace: *mut f32,
    rgbValue: *mut u32,
) -> c_int {
    unsafe {
        let mut sep = strstartswith(feat, b"mapping\0".as_ptr() as *const c_char);
        if !sep.is_null() {
            if *sep != b'=' as c_char {
                return -1;
            }
            loaded_font_mapping = load_mapping_file(sep.add(1), end, 0);
            return 1;
        }

        sep = strstartswith(feat, b"extend\0".as_ptr() as *const c_char);
        if !sep.is_null() {
            if *sep != b'=' as c_char {
                return -1;
            }
            sep = sep.add(1);
            *extend = read_double(&mut sep) as f32;
            return 1;
        }

        sep = strstartswith(feat, b"slant\0".as_ptr() as *const c_char);
        if !sep.is_null() {
            if *sep != b'=' as c_char {
                return -1;
            }
            sep = sep.add(1);
            *slant = read_double(&mut sep) as f32;
            return 1;
        }

        sep = strstartswith(feat, b"embolden\0".as_ptr() as *const c_char);
        if !sep.is_null() {
            if *sep != b'=' as c_char {
                return -1;
            }
            sep = sep.add(1);
            *embolden = read_double(&mut sep) as f32;
            return 1;
        }

        sep = strstartswith(feat, b"letterspace\0".as_ptr() as *const c_char);
        if !sep.is_null() {
            if *sep != b'=' as c_char {
                return -1;
            }
            sep = sep.add(1);
            *letterspace = read_double(&mut sep) as f32;
            return 1;
        }

        sep = strstartswith(feat, b"color\0".as_ptr() as *const c_char);
        if !sep.is_null() {
            if *sep != b'=' as c_char {
                return -1;
            }
            sep = sep.add(1);
            let s = sep;
            *rgbValue = read_rgb_a(&mut sep);
            if sep == s.add(6) || sep == s.add(8) {
                loaded_font_flags = (loaded_font_flags as u32 | FONT_FLAGS_COLORED) as c_char;
            } else {
                return -1;
            }
            return 1;
        }

        0
    }
}

unsafe fn readFeatureNumber(
    mut s: *const c_char,
    e: *const c_char,
    f: *mut u32,
    v: *mut c_int,
) -> bool {
    unsafe {
        *f = 0;
        *v = 0;
        if *s < b'0' as c_char || *s > b'9' as c_char {
            return false;
        }
        while *s >= b'0' as c_char && *s <= b'9' as c_char {
            *f = *f * 10 + (*s as u8 - b'0') as u32;
            s = s.add(1);
        }
        while *s == b' ' as c_char || *s == b'\t' as c_char {
            s = s.add(1);
        }
        if *s != b'=' as c_char {
            return false;
        }
        s = s.add(1);

        if *s < b'0' as c_char || *s > b'9' as c_char {
            return false;
        }
        while *s >= b'0' as c_char && *s <= b'9' as c_char {
            *v = *v * 10 + (*s as u8 - b'0') as c_int;
            s = s.add(1);
        }
        while *s == b' ' as c_char || *s == b'\t' as c_char {
            s = s.add(1);
        }
        s == e
    }
}

unsafe fn loadOTfont(
    fontRef: RawPlatformFontRef,
    font: XeTeXFont,
    mut scaled_size: Fixed,
    mut cp1: *mut c_char,
) -> XeTeXLayoutEngine {
    unsafe {
        let mut engine: XeTeXLayoutEngine = std::ptr::null_mut();
        let mut script: u32 = HB_TAG_NONE;
        let mut language: *mut c_char = std::ptr::null_mut();
        let mut features: Vec<hb_feature_t> = Vec::new();
        let mut shapers: Vec<*const c_char> = Vec::new();

        let mut rgbValue: u32 = 0x000000FF;
        let mut extend: f32 = 1.0;
        let mut slant: f32 = 0.0;
        let mut embolden: f32 = 0.0;
        let mut letterspace: f32 = 0.0;

        let reqEngine = getReqEngine();

        if reqEngine == b'O' as c_char || reqEngine == b'G' as c_char {
            if reqEngine == b'O' as c_char {
                shapers.push(c"ot".as_ptr());
            } else if reqEngine == b'G' as c_char {
                shapers.push(c"graphite2".as_ptr());
            }
        }

        if reqEngine == b'G' as c_char {
            let graphiteShaper: [*const c_char; 2] =
                [b"graphite2\0".as_ptr() as *const c_char, std::ptr::null()];
            engine = createLayoutEngineBorrowed(
                font,
                script,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
                graphiteShaper.as_ptr(),
                rgbValue,
                extend,
                slant,
                embolden,
            );
            if engine.is_null() {
                return std::ptr::null_mut();
            }
        }

        if !cp1.is_null() {
            while *cp1 != 0 {
                if *cp1 == b':' as c_char || *cp1 == b';' as c_char || *cp1 == b',' as c_char {
                    cp1 = cp1.add(1);
                }
                while *cp1 == b' ' as c_char || *cp1 == b'\t' as c_char {
                    cp1 = cp1.add(1);
                }
                if *cp1 == 0 {
                    break;
                }

                let mut cp2 = cp1;
                while *cp2 != 0
                    && *cp2 != b':' as c_char
                    && *cp2 != b';' as c_char
                    && *cp2 != b',' as c_char
                {
                    cp2 = cp2.add(1);
                }

                let mut cp3 = strstartswith(cp1, b"script\0".as_ptr() as *const c_char);
                if !cp3.is_null() {
                    if *cp3 != b'=' as c_char {
                        font_feature_warning(
                            cp1 as *const c_void,
                            cp2.offset_from(cp1) as i32,
                            std::ptr::null(),
                            0,
                        );
                        cp1 = cp2;
                        continue;
                    }
                    cp3 = cp3.add(1);
                    script = hb_tag_from_string(cp3, cp2.offset_from(cp3) as c_int);
                    cp1 = cp2;
                    continue;
                }

                cp3 = strstartswith(cp1, b"language\0".as_ptr() as *const c_char);
                if !cp3.is_null() {
                    if *cp3 != b'=' as c_char {
                        font_feature_warning(
                            cp1 as *const c_void,
                            cp2.offset_from(cp1) as i32,
                            std::ptr::null(),
                            0,
                        );
                        cp1 = cp2;
                        continue;
                    }
                    cp3 = cp3.add(1);
                    let len = cp2.offset_from(cp3) as usize;
                    libc::free(language.cast());
                    language = xmalloc(len + 1) as *mut c_char;
                    *language.add(len) = 0;
                    libc::memcpy(language as *mut c_void, cp3 as *const c_void, len);
                    cp1 = cp2;
                    continue;
                }

                cp3 = strstartswith(cp1, b"shaper\0".as_ptr() as *const c_char);
                if !cp3.is_null() {
                    if *cp3 != b'=' as c_char {
                        font_feature_warning(
                            cp1 as *const c_void,
                            cp2.offset_from(cp1) as i32,
                            std::ptr::null(),
                            0,
                        );
                        cp1 = cp2;
                        continue;
                    }
                    cp3 = cp3.add(1);
                    let len = cp2.offset_from(cp3) as usize;
                    let s_copy = xmalloc(len + 1) as *mut c_char;
                    libc::memcpy(s_copy as *mut c_void, cp3 as *const c_void, len);
                    *s_copy.add(len) = 0;
                    shapers.push(s_copy);
                    cp1 = cp2;
                    continue;
                }

                let ret = readCommonFeatures(
                    cp1,
                    cp2,
                    &mut extend,
                    &mut slant,
                    &mut embolden,
                    &mut letterspace,
                    &mut rgbValue,
                );
                if ret == 1 {
                    cp1 = cp2;
                    continue;
                } else if ret == -1 {
                    font_feature_warning(
                        cp1 as *const c_void,
                        cp2.offset_from(cp1) as i32,
                        std::ptr::null(),
                        0,
                    );
                    cp1 = cp2;
                    continue;
                }

                if reqEngine == b'G' as c_char {
                    let mut tag: u32 = 0;
                    let mut value: c_int = 0;
                    if readFeatureNumber(cp1, cp2, &mut tag, &mut value)
                        || findGraphiteFeature(engine, cp1, cp2, &mut tag, &mut value)
                    {
                        features.push(hb_feature_t {
                            tag,
                            value: value as u32,
                            start: 0,
                            end: !0u32,
                        });
                        cp1 = cp2;
                        continue;
                    }
                }

                if *cp1 == b'+' as c_char {
                    let mut param: c_int = 0;
                    let tag = read_tag_with_param(cp1.add(1), &mut param);
                    if param >= 0 {
                        param += 1;
                    }
                    features.push(hb_feature_t {
                        tag,
                        value: param as u32,
                        start: 0,
                        end: !0u32,
                    });
                    cp1 = cp2;
                    continue;
                }

                if *cp1 == b'-' as c_char {
                    cp1 = cp1.add(1);
                    let tag = hb_tag_from_string(cp1, cp2.offset_from(cp1) as c_int);
                    features.push(hb_feature_t {
                        tag,
                        value: 0,
                        start: 0,
                        end: !0u32,
                    });
                    cp1 = cp2;
                    continue;
                }

                if !strstartswith(cp1, b"vertical\0".as_ptr() as *const c_char).is_null() {
                    let mut cp_end = cp2;
                    if *cp_end == b';' as c_char
                        || *cp_end == b':' as c_char
                        || *cp_end == b',' as c_char
                    {
                        cp_end = cp_end.sub(1);
                    }
                    while *cp_end == 0 || *cp_end == b' ' as c_char || *cp_end == b'\t' as c_char {
                        cp_end = cp_end.sub(1);
                    }
                    if *cp_end != 0 {
                        cp_end = cp_end.add(1);
                    }
                    if cp_end == cp1.add(8) {
                        loaded_font_flags =
                            (loaded_font_flags as u32 | FONT_FLAGS_VERTICAL) as c_char;
                        cp1 = cp2;
                        continue;
                    }
                }

                font_feature_warning(
                    cp1 as *const c_void,
                    cp2.offset_from(cp1) as i32,
                    std::ptr::null(),
                    0,
                );
                cp1 = cp2;
            }
        }

        if !shapers.is_empty() {
            shapers.push(std::ptr::null());
        }

        if !engine.is_null() {
            deleteLayoutEngine(engine);
        }

        if embolden != 0.0 {
            embolden = (embolden as f64 * Fix2D(scaled_size) / 100.0) as f32;
        }

        if letterspace != 0.0 {
            loaded_font_letter_space =
                ((letterspace as f64 / 100.0) * scaled_size as f64) as scaled_t;
        }

        if (loaded_font_flags as u32 & FONT_FLAGS_COLORED) == 0 {
            rgbValue = 0x000000FF;
        }

        if (loaded_font_flags as u32 & FONT_FLAGS_VERTICAL) != 0 {
            setFontLayoutDir(font, 1);
        }

        engine = createLayoutEngine(
            font,
            script,
            language,
            if features.is_empty() {
                std::ptr::null_mut()
            } else {
                features.as_mut_ptr()
            },
            features.len() as c_int,
            if shapers.is_empty() {
                std::ptr::null()
            } else {
                shapers.as_ptr()
            },
            rgbValue,
            extend,
            slant,
            embolden,
        );

        if !language.is_null() {
            libc::free(language as *mut c_void);
        }
        if !engine.is_null() {
            native_font_type_flag = OTGR_FONT_FLAG as i32;
        }

        engine
    }
}

unsafe fn splitFontName(
    mut name: *mut c_char,
    var: *mut *mut c_char,
    feat: *mut *mut c_char,
    end: *mut *mut c_char,
    index: *mut c_int,
) {
    unsafe {
        *var = std::ptr::null_mut();
        *feat = std::ptr::null_mut();
        *index = 0;

        if *name == b'[' as c_char {
            let mut withinFileName = true;
            name = name.add(1);
            while *name != 0 {
                if withinFileName && *name == b']' as c_char {
                    withinFileName = false;
                    if (*var).is_null() {
                        *var = name;
                    }
                } else if *name == b':' as c_char {
                    if withinFileName && (*var).is_null() {
                        *var = name;
                        name = name.add(1);
                        while *name >= b'0' as c_char && *name <= b'9' as c_char {
                            *index = *index * 10 + (*name as u8 - b'0') as c_int;
                            name = name.add(1);
                        }
                        name = name.sub(1);
                    } else if !withinFileName && (*feat).is_null() {
                        *feat = name;
                    }
                }
                name = name.add(1);
            }
            *end = name;
        } else {
            while *name != 0 {
                if *name == b'/' as c_char && (*var).is_null() && (*feat).is_null() {
                    *var = name;
                } else if *name == b':' as c_char && (*feat).is_null() {
                    *feat = name;
                }
                name = name.add(1);
            }
            *end = name;
        }

        if (*feat).is_null() {
            *feat = name;
        }
        if (*var).is_null() {
            *var = *feat;
        }
    }
}

pub unsafe fn find_native_font(uname: *mut c_char, mut scaled_size: i32) -> *mut c_void {
    unsafe {
        let mut rval: *mut c_void = std::ptr::null_mut();
        let mut var: *mut c_char = std::ptr::null_mut();
        let mut feat: *mut c_char = std::ptr::null_mut();
        let mut end: *mut c_char = std::ptr::null_mut();
        let name = uname;
        let mut varString: *mut c_char = std::ptr::null_mut();
        let mut featString: *mut c_char = std::ptr::null_mut();
        let mut font: XeTeXFont = std::ptr::null_mut();
        let mut index: c_int = 0;

        loaded_font_mapping = std::ptr::null_mut();
        loaded_font_flags = 0;
        loaded_font_letter_space = 0;

        splitFontName(name, &mut var, &mut feat, &mut end, &mut index);

        let name_len = var.offset_from(name) as usize;
        let nameString = xmalloc(name_len + 1) as *mut c_char;
        libc::strncpy(nameString, name, name_len);
        *nameString.add(name_len) = 0;

        if feat > var {
            let var_len = feat.offset_from(var) as usize - 1;
            varString = xmalloc(var_len + 1) as *mut c_char;
            libc::strncpy(varString, var.add(1), var_len);
            *varString.add(var_len) = 0;
        }

        if end > feat {
            let feat_len = end.offset_from(feat) as usize - 1;
            featString = xmalloc(feat_len + 1) as *mut c_char;
            libc::strncpy(featString, feat.add(1), feat_len);
            *featString.add(feat_len) = 0;
        }

        if *nameString == b'[' as c_char {
            if scaled_size < 0 {
                font = createFontFromFile(nameString.add(1), index as u32, 655360);
                if !font.is_null() {
                    let dsize = D2Fix(getDesignSize(font));
                    if scaled_size == -1000 {
                        scaled_size = dsize;
                    } else {
                        scaled_size = xn_over_d(dsize, -scaled_size, 1000);
                    }
                    deleteFont(font);
                }
            }
            font = createFontFromFile(nameString.add(1), index as u32, scaled_size);
            if !font.is_null() {
                set_loaded_font_design_size(D2Fix(getDesignSize(font)));
                setReqEngine(0);
                if !varString.is_null() {
                    if !strstartswith(varString, b"/AAT\0".as_ptr() as *const c_char).is_null() {
                        setReqEngine(b'A' as c_char);
                    } else if !strstartswith(varString, b"/OT\0".as_ptr() as *const c_char)
                        .is_null()
                        || !strstartswith(varString, b"/ICU\0".as_ptr() as *const c_char).is_null()
                    {
                        setReqEngine(b'O' as c_char);
                    } else if !strstartswith(varString, b"/GR\0".as_ptr() as *const c_char)
                        .is_null()
                    {
                        setReqEngine(b'G' as c_char);
                    }
                }

                rval =
                    loadOTfont(std::ptr::null_mut(), font, scaled_size, featString) as *mut c_void;
                if rval.is_null() {
                    deleteFont(font);
                }
                if !rval.is_null() && get_tracing_fonts_state() > 0 {
                    begin_diagnostic();
                    print_nl(b' ' as i32);
                    print_c_string(b"-> \0".as_ptr() as *const c_char);
                    print_c_string(nameString.add(1));
                    end_diagnostic(false);
                }
            }
        } else {
            let fontRef = findFontByName(nameString, varString, Fix2D(scaled_size));

            if get_tracing_fonts_state() > 0 {
                begin_diagnostic();
                print_nl(b' ' as i32);
                print_c_string(b"-> \0".as_ptr() as *const c_char);
                print_c_string(ttxl_platfont_get_desc(fontRef));
                end_diagnostic(false);
            }

            if !fontRef.is_null() {
                let fullName = getFullName(fontRef);
                let mut total_len = libc::strlen(fullName);
                if !featString.is_null() {
                    total_len += libc::strlen(featString) + 1;
                }
                if !varString.is_null() {
                    total_len += libc::strlen(varString) + 1;
                }
                name_length = total_len as c_int;
                libc::free(name_of_file as *mut c_void);
                name_of_file = xmalloc(name_length as usize + 1) as *mut c_char;
                libc::strcpy(name_of_file, fullName);

                if scaled_size < 0 {
                    font = createFont(fontRef, scaled_size);
                    if !font.is_null() {
                        let dsize = D2Fix(getDesignSize(font));
                        if scaled_size == -1000 {
                            scaled_size = dsize;
                        } else {
                            scaled_size = xn_over_d(dsize, -scaled_size, 1000);
                        }
                        deleteFont(font);
                    }
                }

                font = createFont(fontRef, scaled_size);
                if !font.is_null() {
                    #[cfg(target_os = "macos")]
                    {
                        if getReqEngine() == b'A' as c_char {
                            rval = loadAATfont(fontRef, scaled_size, featString);
                            if rval.is_null() {
                                deleteFont(font);
                            }
                        } else {
                            if getReqEngine() == b'O' as c_char
                                || getReqEngine() == b'G' as c_char
                                || hasFontTable(font, kGSUB)
                                || hasFontTable(font, kGPOS)
                            {
                                rval = loadOTfont(fontRef, font, scaled_size, featString)
                                    as *mut c_void;
                            }
                            if rval.is_null() {
                                rval = loadAATfont(fontRef, scaled_size, featString);
                            }
                            if rval.is_null() {
                                deleteFont(font);
                            }
                        }
                    }
                    #[cfg(not(target_os = "macos"))]
                    {
                        rval = loadOTfont(fontRef, font, scaled_size, featString) as *mut c_void;
                        if rval.is_null() {
                            deleteFont(font);
                        }
                    }
                }

                if !varString.is_null() && *varString != 0 {
                    libc::strcat(name_of_file, b"/\0".as_ptr() as *const c_char);
                    libc::strcat(name_of_file, varString);
                }
                if !featString.is_null() && *featString != 0 {
                    libc::strcat(name_of_file, b":\0".as_ptr() as *const c_char);
                    libc::strcat(name_of_file, featString);
                }
                name_length = libc::strlen(name_of_file) as c_int;
            }
        }

        libc::free(varString as *mut c_void);
        libc::free(featString as *mut c_void);
        libc::free(nameString as *mut c_void);

        rval
    }
}

#[unsafe(export_name = "release_font_engine")]
pub unsafe extern "C" fn release_font_engine(engine: *mut c_void, type_flag: c_int) {
    unsafe {
        #[cfg(target_os = "macos")]
        if type_flag as u32 == AAT_FONT_FLAG {
            CFRelease(engine);
            return;
        }

        if type_flag as u32 == OTGR_FONT_FLAG {
            deleteLayoutEngine(engine as XeTeXLayoutEngine);
        }
    }
}

#[unsafe(export_name = "ot_get_font_metrics")]
pub unsafe extern "C" fn ot_get_font_metrics(
    pEngine: *mut c_void,
    ascent: *mut scaled_t,
    descent: *mut scaled_t,
    xheight: *mut scaled_t,
    capheight: *mut scaled_t,
    slant: *mut scaled_t,
) {
    unsafe {
        let engine = pEngine as XeTeXLayoutEngine;
        let mut a: f32 = 0.0;
        let mut d: f32 = 0.0;

        getAscentAndDescent(engine, &mut a, &mut d);
        *ascent = D2Fix(a as f64);
        *descent = D2Fix(d as f64);

        *slant = D2Fix(
            Fix2D(getSlant(getFont(engine))) * (getExtendFactor(engine) as f64)
                + (getSlantFactor(engine) as f64),
        );

        getCapAndXHeight(engine, &mut a, &mut d);
        *capheight = D2Fix(a as f64);
        *xheight = D2Fix(d as f64);

        if *xheight == 0 {
            let glyphID = mapCharToGlyph(engine, b'x' as u32);
            if glyphID != 0 {
                getGlyphHeightDepth(engine, glyphID, &mut a, &mut d);
                *xheight = D2Fix(a as f64);
            } else {
                *xheight = *ascent / 2;
            }
        }

        if *capheight == 0 {
            let glyphID = mapCharToGlyph(engine, b'X' as u32);
            if glyphID != 0 {
                getGlyphHeightDepth(engine, glyphID, &mut a, &mut d);
                *capheight = D2Fix(a as f64);
            } else {
                *capheight = *ascent;
            }
        }
    }
}

#[unsafe(export_name = "ot_font_get")]
pub unsafe extern "C" fn ot_font_get(what: i32, pEngine: *mut c_void) -> i32 {
    unsafe {
        let engine = pEngine as XeTeXLayoutEngine;
        let fontInst = getFont(engine);
        match what {
            XeTeX_count_glyphs => countGlyphs(fontInst) as i32,
            XeTeX_count_features => countGraphiteFeatures(engine) as i32,
            XeTeX_OT_count_scripts => countScripts(fontInst) as i32,
            _ => 0,
        }
    }
}

#[unsafe(export_name = "ot_font_get_1")]
pub unsafe extern "C" fn ot_font_get_1(what: i32, pEngine: *mut c_void, param: i32) -> i32 {
    unsafe {
        let engine = pEngine as XeTeXLayoutEngine;
        let fontInst = getFont(engine);
        match what {
            XeTeX_OT_count_languages => countLanguages(fontInst, param as u32) as i32,
            XeTeX_OT_script_code => getIndScript(fontInst, param as libc::c_uint) as i32,
            XeTeX_feature_code => getGraphiteFeatureCode(engine, param as u32) as i32,
            XeTeX_is_exclusive_feature => 1,
            XeTeX_count_selectors => countGraphiteFeatureSettings(engine, param as u32) as i32,
            _ => 0,
        }
    }
}

#[unsafe(export_name = "ot_font_get_2")]
pub unsafe extern "C" fn ot_font_get_2(
    what: i32,
    pEngine: *mut c_void,
    param1: i32,
    param2: i32,
) -> i32 {
    unsafe {
        let engine = pEngine as XeTeXLayoutEngine;
        let fontInst = getFont(engine);
        match what {
            XeTeX_OT_language_code => {
                getIndLanguage(fontInst, param1 as u32, param2 as libc::c_uint) as i32
            }
            XeTeX_OT_count_features => countFeatures(fontInst, param1 as u32, param2 as u32) as i32,
            XeTeX_selector_code => {
                getGraphiteFeatureSettingCode(engine, param1 as u32, param2 as u32) as i32
            }
            XeTeX_is_default_selector => {
                if getGraphiteFeatureDefaultSetting(engine, param1 as u32) == param2 as u32 {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}

#[unsafe(export_name = "ot_font_get_3")]
pub unsafe extern "C" fn ot_font_get_3(
    what: i32,
    pEngine: *mut c_void,
    param1: i32,
    param2: i32,
    param3: i32,
) -> i32 {
    unsafe {
        let engine = pEngine as XeTeXLayoutEngine;
        let fontInst = getFont(engine);
        match what {
            XeTeX_OT_feature_code => getIndFeature(
                fontInst,
                param1 as u32,
                param2 as u32,
                param3 as libc::c_uint,
            ) as i32,
            _ => 0,
        }
    }
}

#[unsafe(export_name = "gr_print_font_name")]
pub unsafe extern "C" fn gr_print_font_name(
    what: i32,
    pEngine: *mut c_void,
    param1: i32,
    param2: i32,
) {
    unsafe {
        let engine = pEngine as XeTeXLayoutEngine;
        let mut name: *mut c_char = std::ptr::null_mut();
        match what {
            XETEX_FEATURE_NAME_CODE => {
                name = getGraphiteFeatureLabel(engine, param1 as u32);
            }
            XETEX_SELECTOR_NAME_CODE => {
                name = getGraphiteFeatureSettingLabel(engine, param1 as u32, param2 as u32);
            }
            _ => {}
        }

        if !name.is_null() {
            print_c_string(name);
            gr_label_destroy(name as *mut c_void);
        }
    }
}

#[unsafe(export_name = "gr_font_get_named")]
pub unsafe extern "C" fn gr_font_get_named(what: i32, pEngine: *mut c_void) -> i32 {
    unsafe {
        let mut rval: libc::c_long = -1;
        let engine = pEngine as XeTeXLayoutEngine;
        if what == XeTeX_find_feature_by_name {
            rval = findGraphiteFeatureNamed(engine, name_of_file, name_length);
        }
        rval as i32
    }
}

#[unsafe(export_name = "gr_font_get_named_1")]
pub unsafe extern "C" fn gr_font_get_named_1(what: i32, pEngine: *mut c_void, param: i32) -> i32 {
    unsafe {
        let mut rval: libc::c_long = -1;
        let engine = pEngine as XeTeXLayoutEngine;
        if what == XeTeX_find_selector_by_name {
            rval = findGraphiteFeatureSettingNamed(engine, param, name_of_file, name_length);
        }
        rval as i32
    }
}

#[unsafe(export_name = "tt_checkpoint_ext_reset")]
pub unsafe extern "C" fn tt_checkpoint_ext_reset() {
    unsafe {
        if !brkIter.is_null() {
            ubrk_close(brkIter);
        }
        brkIter = std::ptr::null_mut();
        brkLocaleStrNum = 0;
    }
}

#[unsafe(export_name = "tt_checkpoint_fonts")]
pub unsafe extern "C" fn tt_checkpoint_fonts(cp: *mut tt_checkpoint) {
    unsafe {
        let saved_name = name_of_file;
        let saved_name_length = name_length;
        let saved_flags = loaded_font_flags;
        let saved_spacing = loaded_font_letter_space;
        let saved_type = native_font_type_flag;

        let mut f = FONT_BASE;
        while f <= font_ptr && (*cp).valid {
            let area = *font_area.add(f as usize) as u32;
            if area != OTGR_FONT_FLAG && area != AAT_FONT_FLAG {
                f += 1;
                continue;
            }

            if (*cp).restoring {
                let recipe = gettexstring(*font_name.add(f as usize));
                name_of_file = xstrdup(recipe);
                name_length = libc::strlen(recipe) as c_int;
                let engine = find_native_font(recipe, *font_size.add(f as usize));
                *font_layout_engine.add(f as usize) = engine;
                libc::free(recipe as *mut c_void);
                if engine.is_null() || native_font_type_flag != area as i32 {
                    (*cp).valid = false;
                    break;
                }
                *font_mapping.add(f as usize) = loaded_font_mapping;
            }

            let length = make_font_def(f);
            if length < 7 || length > 4096 {
                (*cp).valid = false;
                break;
            }

            if (*cp).restoring {
                let expected = xmalloc(length as usize) as *mut c_char;
                tt_checkpoint_field(
                    cp.cast(),
                    b"font\0".as_ptr() as *const c_char,
                    expected as *mut c_void,
                    length as usize,
                );
                if libc::memcmp(
                    expected as *const c_void,
                    xdv_buffer as *const c_void,
                    length as usize,
                ) != 0
                {
                    (*cp).valid = false;
                }
                libc::free(expected as *mut c_void);
                libc::free(name_of_file as *mut c_void);
            } else {
                let mut path: [c_char; 257] = [0; 257];
                let path_length = (*xdv_buffer.add(6) as u8) as usize;
                libc::memcpy(
                    path.as_mut_ptr() as *mut c_void,
                    xdv_buffer.add(7) as *const c_void,
                    path_length,
                );
                path[path_length] = 0;
                let handle = ttstub_input_open(path.as_ptr(), TTBC_FILE_FORMAT_OPEN_TYPE, 0);
                if handle == 0 {
                    (*cp).valid = false;
                    break;
                }
                let mut buffer: [c_char; 8192] = [0; 8192];
                let mut count: libc::ssize_t;
                loop {
                    count = ttstub_input_read_partial(handle, buffer.as_mut_ptr(), buffer.len());
                    if count <= 0 {
                        break;
                    }
                }
                if count < 0 {
                    (*cp).valid = false;
                }
                ttstub_input_close(handle);
                tt_checkpoint_field(
                    cp.cast(),
                    b"font\0".as_ptr() as *const c_char,
                    xdv_buffer as *mut c_void,
                    length as usize,
                );
            }

            f += 1;
        }

        name_of_file = saved_name;
        name_length = saved_name_length;
        loaded_font_flags = saved_flags;
        loaded_font_letter_space = saved_spacing;
        native_font_type_flag = saved_type;
    }
}

#[unsafe(export_name = "makeXDVGlyphArrayData")]
pub unsafe extern "C" fn makeXDVGlyphArrayData(pNode: *mut c_void) -> c_int {
    unsafe {
        let p = pNode as *mut memory_word;
        let glyphCount = *native_glyph_count(p) as usize;

        let req_size = glyphCount * native_glyph_info_size + 8;
        if req_size > xdvBufSize as usize {
            libc::free(xdv_buffer as *mut c_void);
            xdvBufSize = (((req_size / 1024) + 1) * 1024) as i32;
            xdv_buffer = xmalloc(xdvBufSize as usize) as *mut c_char;
        }

        let glyph_info = *native_glyph_info_ptr(p);
        let locations = glyph_info as *mut FixedPoint;
        let glyphIDs = locations.add(glyphCount) as *mut u16;

        let mut cp = xdv_buffer as *mut u8;

        let width = *node_width(p);
        *cp = ((width >> 24) & 0xff) as u8;
        cp = cp.add(1);
        *cp = ((width >> 16) & 0xff) as u8;
        cp = cp.add(1);
        *cp = ((width >> 8) & 0xff) as u8;
        cp = cp.add(1);
        *cp = (width & 0xff) as u8;
        cp = cp.add(1);

        *cp = ((glyphCount >> 8) & 0xff) as u8;
        cp = cp.add(1);
        *cp = (glyphCount & 0xff) as u8;
        cp = cp.add(1);

        for i in 0..glyphCount {
            let x = (*locations.add(i)).x;
            let y = (*locations.add(i)).y;
            *cp = ((x >> 24) & 0xff) as u8;
            cp = cp.add(1);
            *cp = ((x >> 16) & 0xff) as u8;
            cp = cp.add(1);
            *cp = ((x >> 8) & 0xff) as u8;
            cp = cp.add(1);
            *cp = (x & 0xff) as u8;
            cp = cp.add(1);
            *cp = ((y >> 24) & 0xff) as u8;
            cp = cp.add(1);
            *cp = ((y >> 16) & 0xff) as u8;
            cp = cp.add(1);
            *cp = ((y >> 8) & 0xff) as u8;
            cp = cp.add(1);
            *cp = (y & 0xff) as u8;
            cp = cp.add(1);
        }

        for i in 0..glyphCount {
            let g = *glyphIDs.add(i);
            *cp = ((g >> 8) & 0xff) as u8;
            cp = cp.add(1);
            *cp = (g & 0xff) as u8;
            cp = cp.add(1);
        }

        cp.offset_from(xdv_buffer as *mut u8) as c_int
    }
}

#[unsafe(export_name = "make_font_def")]
pub unsafe extern "C" fn make_font_def(f: i32) -> c_int {
    unsafe {
        let mut flags: u16 = 0;
        let mut rgba: u32 = 0;
        let mut size: Fixed = 0;
        let mut filename: *const c_char = std::ptr::null();
        let mut index: u32 = 0;
        let mut extend: f32 = 1.0;
        let mut slant: f32 = 0.0;
        let mut embolden: f32 = 0.0;

        let area = *font_area.add(f as usize) as u32;

        let mut handled = false;
        #[cfg(target_os = "macos")]
        if area == AAT_FONT_FLAG {
            let attributes = *font_layout_engine.add(f as usize);
            let font = CFDictionaryGetValue(attributes, kCTFontAttributeName);
            filename = getFileNameFromCTFont(font, &mut index);
            assert!(!filename.is_null());

            if !CFDictionaryGetValue(attributes, kCTVerticalFormsAttributeName).is_null() {
                flags |= XDV_FLAG_VERTICAL;
            }

            let color = CFDictionaryGetValue(attributes, kCTForegroundColorAttributeName);
            if !color.is_null() {
                rgba = cgColorToRGBA32(color);
            }

            let t = CTFontGetMatrix(font);
            extend = t.a as f32;
            slant = t.c as f32;

            let emboldenNumber = if kXeTeXEmboldenAttributeName.is_null() {
                std::ptr::null()
            } else {
                CFDictionaryGetValue(attributes, kXeTeXEmboldenAttributeName)
            };
            if !emboldenNumber.is_null() {
                CFNumberGetValue(
                    emboldenNumber,
                    kCFNumberFloatType.into(),
                    &mut embolden as *mut f32 as *mut c_void,
                );
            }

            let fSize = CTFontGetSize(font);
            size = D2Fix(fSize);
            handled = true;
        }

        if !handled {
            if area == OTGR_FONT_FLAG {
                let engine = *font_layout_engine.add(f as usize) as XeTeXLayoutEngine;
                filename = getFontFilename(engine, &mut index);
                assert!(!filename.is_null());

                rgba = getRgbValue(engine);
                if (*font_flags.add(f as usize) as u32 & FONT_FLAGS_VERTICAL) != 0 {
                    flags |= XDV_FLAG_VERTICAL;
                }

                extend = getExtendFactor(engine);
                slant = getSlantFactor(engine);
                embolden = getEmboldenFactor(engine);

                size = D2Fix(getPointSize(engine) as f64);
            } else {
                _tt_abort(b"bad native font flag in `make_font_def`\0".as_ptr() as *const c_char);
            }
        }

        let filenameLen = libc::strlen(filename);
        let mut fontDefLength = 4 + 2 + 1 + filenameLen + 4;

        if (*font_flags.add(f as usize) as u32 & FONT_FLAGS_COLORED) != 0 {
            fontDefLength += 4;
            flags |= XDV_FLAG_COLORED;
        }

        if extend != 1.0 {
            fontDefLength += 4;
            flags |= XDV_FLAG_EXTEND;
        }
        if slant != 0.0 {
            fontDefLength += 4;
            flags |= XDV_FLAG_SLANT;
        }
        if embolden != 0.0 {
            fontDefLength += 4;
            flags |= XDV_FLAG_EMBOLDEN;
        }

        if fontDefLength > xdvBufSize as usize {
            libc::free(xdv_buffer as *mut c_void);
            xdvBufSize = (((fontDefLength / 1024) + 1) * 1024) as i32;
            xdv_buffer = xmalloc(xdvBufSize as usize) as *mut c_char;
        }

        let mut cp = xdv_buffer as *mut u8;

        std::ptr::write_unaligned(cp.cast::<Fixed>(), SWAP32(size as u32) as Fixed);
        cp = cp.add(4);

        std::ptr::write_unaligned(cp.cast::<u16>(), SWAP16(flags));
        cp = cp.add(2);

        *cp = filenameLen as u8;
        cp = cp.add(1);
        libc::memcpy(cp as *mut c_void, filename as *const c_void, filenameLen);
        cp = cp.add(filenameLen);

        std::ptr::write_unaligned(cp.cast::<u32>(), SWAP32(index));
        cp = cp.add(4);

        if (*font_flags.add(f as usize) as u32 & FONT_FLAGS_COLORED) != 0 {
            std::ptr::write_unaligned(cp.cast::<u32>(), SWAP32(rgba));
            cp = cp.add(4);
        }

        if (flags & XDV_FLAG_EXTEND) != 0 {
            let f_fix = D2Fix(extend as f64);
            std::ptr::write_unaligned(cp.cast::<u32>(), SWAP32(f_fix as u32));
            cp = cp.add(4);
        }
        if (flags & XDV_FLAG_SLANT) != 0 {
            let f_fix = D2Fix(slant as f64);
            std::ptr::write_unaligned(cp.cast::<u32>(), SWAP32(f_fix as u32));
            cp = cp.add(4);
        }
        if (flags & XDV_FLAG_EMBOLDEN) != 0 {
            let f_fix = D2Fix(embolden as f64);
            std::ptr::write_unaligned(cp.cast::<u32>(), SWAP32(f_fix as u32));
            cp = cp.add(4);
        }

        freeFontFilename(filename);

        fontDefLength as c_int
    }
}

#[unsafe(export_name = "apply_mapping")]
pub unsafe extern "C" fn apply_mapping(
    pCnv: *mut c_void,
    txtPtr: *mut u16,
    txtLen: c_int,
) -> c_int {
    unsafe {
        let cnv = pCnv as TECkit_Converter;
        let mut inUsed: UInt32 = 0;
        let mut outUsed: UInt32 = 0;

        let req_len = (txtLen as usize) * std::mem::size_of::<UniChar>() + 32;
        if (outLength as usize) < req_len {
            libc::free(mapped_text as *mut c_void);
            outLength = req_len as UInt32;
            mapped_text = xmalloc(outLength as usize) as *mut u16;
        }

        loop {
            let status = TECkit_ConvertBuffer(
                cnv,
                txtPtr as *const Byte,
                (txtLen as usize * std::mem::size_of::<UniChar>()) as UInt32,
                &mut inUsed,
                mapped_text as *mut Byte,
                outLength,
                &mut outUsed,
                1,
            );
            TECkit_ResetConverter(cnv);

            match status {
                kStatus_NoError => {
                    return (outUsed as usize / std::mem::size_of::<UniChar>()) as c_int;
                }
                kStatus_OutputBufferFull => {
                    outLength += (txtLen as UInt32 * std::mem::size_of::<UniChar>() as UInt32) + 32;
                    libc::free(mapped_text as *mut c_void);
                    mapped_text = xmalloc(outLength as usize) as *mut u16;
                }
                _ => {
                    return 0;
                }
            }
        }
    }
}

#[inline]
unsafe fn snap_zone(value: *mut scaled_t, snap_value: scaled_t, fuzz: scaled_t) {
    unsafe {
        let difference = *value - snap_value;
        if difference <= fuzz && difference >= -fuzz {
            *value = snap_value;
        }
    }
}

#[unsafe(export_name = "get_native_char_height_depth")]
pub unsafe extern "C" fn get_native_char_height_depth(
    font: i32,
    ch: i32,
    height: *mut scaled_t,
    depth: *mut scaled_t,
) {
    unsafe {
        let mut ht: f32 = 0.0;
        let mut dp: f32 = 0.0;

        let area = *font_area.add(font as usize) as u32;

        let mut handled = false;
        #[cfg(target_os = "macos")]
        if area == AAT_FONT_FLAG {
            let attributes = *font_layout_engine.add(font as usize);
            let gid = MapCharToGlyph_AAT(attributes, ch as u32);
            GetGlyphHeightDepth_AAT(attributes, gid as u16, &mut ht, &mut dp);
            handled = true;
        }

        if !handled {
            if area == OTGR_FONT_FLAG {
                let engine = *font_layout_engine.add(font as usize) as XeTeXLayoutEngine;
                let gid = mapCharToGlyph(engine, ch as u32);
                getGlyphHeightDepth(engine, gid, &mut ht, &mut dp);
            } else {
                _tt_abort(
                    b"bad native font flag in `get_native_char_height_depth`\0".as_ptr()
                        as *const c_char,
                );
            }
        }

        *height = D2Fix(ht as f64);
        *depth = D2Fix(dp as f64);

        let fuzz = QUAD(font) / 25;
        snap_zone(depth, 0, fuzz);
        snap_zone(height, 0, fuzz);
        snap_zone(height, X_HEIGHT(font), fuzz);
        snap_zone(height, CAP_HEIGHT(font), fuzz);
    }
}

#[unsafe(export_name = "getnativecharht")]
pub unsafe extern "C" fn getnativecharht(f: i32, c: i32) -> scaled_t {
    unsafe {
        let mut h: scaled_t = 0;
        let mut d: scaled_t = 0;
        get_native_char_height_depth(f, c, &mut h, &mut d);
        h
    }
}

#[unsafe(export_name = "getnativechardp")]
pub unsafe extern "C" fn getnativechardp(f: i32, c: i32) -> scaled_t {
    unsafe {
        let mut h: scaled_t = 0;
        let mut d: scaled_t = 0;
        get_native_char_height_depth(f, c, &mut h, &mut d);
        d
    }
}

#[unsafe(export_name = "get_native_char_sidebearings")]
pub unsafe extern "C" fn get_native_char_sidebearings(
    font: i32,
    ch: i32,
    lsb: *mut scaled_t,
    rsb: *mut scaled_t,
) {
    unsafe {
        let mut l: f32 = 0.0;
        let mut r: f32 = 0.0;

        let area = *font_area.add(font as usize) as u32;

        let mut handled = false;
        #[cfg(target_os = "macos")]
        if area == AAT_FONT_FLAG {
            let attributes = *font_layout_engine.add(font as usize);
            let gid = MapCharToGlyph_AAT(attributes, ch as u32);
            GetGlyphSidebearings_AAT(attributes, gid as u16, &mut l, &mut r);
            handled = true;
        }

        if !handled {
            if area == OTGR_FONT_FLAG {
                let engine = *font_layout_engine.add(font as usize) as XeTeXLayoutEngine;
                let gid = mapCharToGlyph(engine, ch as u32);
                getGlyphSidebearings(engine, gid, &mut l, &mut r);
            } else {
                _tt_abort(
                    b"bad native font flag in `get_native_char_side_bearings`\0".as_ptr()
                        as *const c_char,
                );
            }
        }

        *lsb = D2Fix(l as f64);
        *rsb = D2Fix(r as f64);
    }
}

#[unsafe(export_name = "get_glyph_bounds")]
pub unsafe extern "C" fn get_glyph_bounds(font: i32, edge: i32, gid: i32) -> scaled_t {
    unsafe {
        let mut a: f32 = 0.0;
        let mut b: f32 = 0.0;

        let area = *font_area.add(font as usize) as u32;

        let mut handled = false;
        #[cfg(target_os = "macos")]
        if area == AAT_FONT_FLAG {
            let attributes = *font_layout_engine.add(font as usize);
            if (edge & 1) != 0 {
                GetGlyphSidebearings_AAT(attributes, gid as u16, &mut a, &mut b);
            } else {
                GetGlyphHeightDepth_AAT(attributes, gid as u16, &mut a, &mut b);
            }
            handled = true;
        }

        if !handled {
            if area == OTGR_FONT_FLAG {
                let engine = *font_layout_engine.add(font as usize) as XeTeXLayoutEngine;
                if (edge & 1) != 0 {
                    getGlyphSidebearings(engine, gid as u32, &mut a, &mut b);
                } else {
                    getGlyphHeightDepth(engine, gid as u32, &mut a, &mut b);
                }
            } else {
                _tt_abort(b"bad native font flag in `get_glyph_bounds`\0".as_ptr() as *const c_char);
            }
        }

        D2Fix(if edge <= 2 { a } else { b } as f64)
    }
}

#[unsafe(export_name = "getnativecharic")]
pub unsafe extern "C" fn getnativecharic(f: i32, c: i32) -> scaled_t {
    unsafe {
        let mut lsb: scaled_t = 0;
        let mut rsb: scaled_t = 0;
        get_native_char_sidebearings(f, c, &mut lsb, &mut rsb);
        let letter_space = *font_letter_space.add(f as usize);
        if rsb < 0 {
            letter_space - rsb
        } else {
            letter_space
        }
    }
}

#[unsafe(export_name = "getnativecharwd")]
pub unsafe extern "C" fn getnativecharwd(f: i32, c: i32) -> scaled_t {
    unsafe {
        let mut wd: scaled_t = 0;
        let area = *font_area.add(f as usize) as u32;

        let mut handled = false;
        #[cfg(target_os = "macos")]
        if area == AAT_FONT_FLAG {
            let attributes = *font_layout_engine.add(f as usize);
            let gid = MapCharToGlyph_AAT(attributes, c as u32);
            wd = D2Fix(GetGlyphWidth_AAT(attributes, gid as u16));
            handled = true;
        }

        if !handled {
            if area == OTGR_FONT_FLAG {
                let engine = *font_layout_engine.add(f as usize) as XeTeXLayoutEngine;
                let gid = mapCharToGlyph(engine, c as u32);
                wd = D2Fix(getGlyphWidthFromEngine(engine, gid) as f64);
            } else {
                _tt_abort(
                    b"bad native font flag in `get_native_char_wd`\0".as_ptr() as *const c_char
                );
            }
        }

        wd
    }
}

#[unsafe(export_name = "real_get_native_glyph")]
pub unsafe extern "C" fn real_get_native_glyph(pNode: *mut c_void, index: libc::c_uint) -> u16 {
    unsafe {
        let node = pNode as *mut memory_word;
        let count = *native_glyph_count(node) as usize;
        if (index as usize) >= count {
            0
        } else {
            let locations = *native_glyph_info_ptr(node) as *mut FixedPoint;
            let glyphIDs = locations.add(count) as *mut u16;
            *glyphIDs.add(index as usize)
        }
    }
}

#[unsafe(export_name = "store_justified_native_glyphs")]
pub unsafe extern "C" fn store_justified_native_glyphs(pNode: *mut c_void) {
    unsafe {
        let node = pNode as *mut memory_word;
        let f = *native_font(node) as usize;

        #[cfg(target_os = "macos")]
        if *font_area.add(f) as u32 == AAT_FONT_FLAG {
            DoAATLayout(node as *mut c_void, 1);
            return;
        }

        let savedWidth = *node_width(node);

        measure_native_node(node as *mut c_void, 0);

        if *node_width(node) != savedWidth {
            let justAmount = Fix2D(savedWidth - *node_width(node));

            let glyphCount = *native_glyph_count(node) as usize;
            let locations = *native_glyph_info_ptr(node) as *mut FixedPoint;
            let glyphIDs = locations.add(glyphCount) as *mut u16;
            let mut spaceCount = 0;

            let spaceGlyph = map_char_to_glyph(f as i32, b' ' as i32);
            for i in 0..glyphCount {
                if *glyphIDs.add(i) as i32 == spaceGlyph {
                    spaceCount += 1;
                }
            }

            if spaceCount > 0 {
                let mut adjustment = 0.0f64;
                let mut spaceIndex = 0;
                for i in 0..glyphCount {
                    let loc_x = (*locations.add(i)).x;
                    (*locations.add(i)).x = D2Fix(Fix2D(loc_x) + adjustment);
                    if *glyphIDs.add(i) as i32 == spaceGlyph {
                        spaceIndex += 1;
                        adjustment = justAmount * (spaceIndex as f64) / (spaceCount as f64);
                    }
                }
            } else if glyphCount > 1 {
                for i in 1..glyphCount {
                    let loc_x = (*locations.add(i)).x;
                    (*locations.add(i)).x =
                        D2Fix(Fix2D(loc_x) + justAmount * (i as f64) / ((glyphCount - 1) as f64));
                }
            }

            *node_width(node) = savedWidth;
        }
    }
}

#[unsafe(export_name = "measure_native_node")]
pub unsafe extern "C" fn measure_native_node(pNode: *mut c_void, use_glyph_metrics: c_int) {
    unsafe {
        let node = pNode as *mut memory_word;
        let txtLen = *native_length(node) as i32;
        let txtPtr = node.add(NATIVE_NODE_SIZE) as *mut u16;
        let f = *native_font(node) as usize;

        let mut handled = false;
        #[cfg(target_os = "macos")]
        if *font_area.add(f) as u32 == AAT_FONT_FLAG {
            DoAATLayout(node as *mut c_void, 0);
            handled = true;
        }

        if !handled {
            if *font_area.add(f) as u32 == OTGR_FONT_FLAG {
                let engine = *font_layout_engine.add(f) as XeTeXLayoutEngine;

                let mut locations: *mut FixedPoint = std::ptr::null_mut();
                let mut glyphIDs: *mut u16;
                let mut glyphAdvances: *mut Fixed = std::ptr::null_mut();
                let mut totalGlyphCount: c_int = 0;

                let mut glyph_info: *mut c_void = std::ptr::null_mut();
                let mut positions: *mut FloatPoint = std::ptr::null_mut();
                let mut advances: *mut f32 = std::ptr::null_mut();
                let mut glyphs: *mut u32 = std::ptr::null_mut();

                let pBiDi = ubidi_open();
                let mut errorCode: UErrorCode = U_ZERO_ERROR;
                ubidi_setPara(
                    pBiDi,
                    txtPtr,
                    txtLen,
                    getDefaultDirection(engine) as u8,
                    std::ptr::null_mut(),
                    &mut errorCode,
                );

                let dir = ubidi_getDirection(pBiDi);
                if dir == UBIDI_MIXED {
                    let nRuns = ubidi_countRuns(pBiDi, &mut errorCode);
                    let mut width = 0.0f64;
                    let mut logicalStart: i32 = 0;
                    let mut length: i32 = 0;

                    for runIndex in 0..nRuns {
                        let run_dir =
                            ubidi_getVisualRun(pBiDi, runIndex, &mut logicalStart, &mut length);
                        totalGlyphCount += layoutChars(
                            engine,
                            txtPtr,
                            logicalStart,
                            length,
                            txtLen,
                            run_dir == UBIDI_RTL,
                        );
                    }

                    if totalGlyphCount > 0 {
                        let mut x = 0.0f64;
                        let mut y = 0.0f64;
                        glyph_info = xcalloc(totalGlyphCount as usize, native_glyph_info_size);
                        locations = glyph_info as *mut FixedPoint;
                        glyphIDs = locations.add(totalGlyphCount as usize) as *mut u16;
                        glyphAdvances =
                            xcalloc(totalGlyphCount as usize, std::mem::size_of::<Fixed>())
                                as *mut Fixed;
                        totalGlyphCount = 0;

                        for runIndex in 0..nRuns {
                            let run_dir =
                                ubidi_getVisualRun(pBiDi, runIndex, &mut logicalStart, &mut length);
                            let nGlyphs = layoutChars(
                                engine,
                                txtPtr,
                                logicalStart,
                                length,
                                txtLen,
                                run_dir == UBIDI_RTL,
                            );

                            glyphs =
                                xcalloc(nGlyphs as usize, std::mem::size_of::<u32>()) as *mut u32;
                            positions =
                                xcalloc((nGlyphs + 1) as usize, std::mem::size_of::<FloatPoint>())
                                    as *mut FloatPoint;
                            advances =
                                xcalloc(nGlyphs as usize, std::mem::size_of::<f32>()) as *mut f32;

                            getGlyphs(engine, glyphs);
                            getGlyphAdvances(engine, advances);
                            getGlyphPositions(engine, positions);

                            for i in 0..nGlyphs {
                                *glyphIDs.add(totalGlyphCount as usize) =
                                    *glyphs.add(i as usize) as u16;
                                (*locations.add(totalGlyphCount as usize)).x =
                                    D2Fix((*positions.add(i as usize)).x as f64 + x);
                                (*locations.add(totalGlyphCount as usize)).y =
                                    D2Fix((*positions.add(i as usize)).y as f64 + y);
                                *glyphAdvances.add(totalGlyphCount as usize) =
                                    D2Fix(*advances.add(i as usize) as f64);
                                totalGlyphCount += 1;
                            }

                            x += (*positions.add(nGlyphs as usize)).x as f64;
                            y += (*positions.add(nGlyphs as usize)).y as f64;

                            libc::free(glyphs as *mut c_void);
                            libc::free(positions as *mut c_void);
                            libc::free(advances as *mut c_void);
                        }
                        width = x;
                    }

                    *node_width(node) = D2Fix(width);
                    *native_glyph_count(node) = totalGlyphCount as u16;
                    *native_glyph_info_ptr(node) = glyph_info;
                    if tt_checkpoint_enabled {
                        tt_checkpoint_register_glyph(node.offset_from(mem) as i32);
                    }
                } else {
                    let mut width = 0.0f64;
                    totalGlyphCount =
                        layoutChars(engine, txtPtr, 0, txtLen, txtLen, dir == UBIDI_RTL);

                    glyphs =
                        xcalloc(totalGlyphCount as usize, std::mem::size_of::<u32>()) as *mut u32;
                    positions = xcalloc(
                        (totalGlyphCount + 1) as usize,
                        std::mem::size_of::<FloatPoint>(),
                    ) as *mut FloatPoint;
                    advances =
                        xcalloc(totalGlyphCount as usize, std::mem::size_of::<f32>()) as *mut f32;

                    getGlyphs(engine, glyphs);
                    getGlyphAdvances(engine, advances);
                    getGlyphPositions(engine, positions);

                    if totalGlyphCount > 0 {
                        glyph_info = xcalloc(totalGlyphCount as usize, native_glyph_info_size);
                        locations = glyph_info as *mut FixedPoint;
                        glyphIDs = locations.add(totalGlyphCount as usize) as *mut u16;
                        glyphAdvances =
                            xcalloc(totalGlyphCount as usize, std::mem::size_of::<Fixed>())
                                as *mut Fixed;
                        for i in 0..totalGlyphCount {
                            *glyphIDs.add(i as usize) = *glyphs.add(i as usize) as u16;
                            *glyphAdvances.add(i as usize) =
                                D2Fix(*advances.add(i as usize) as f64);
                            (*locations.add(i as usize)).x =
                                D2Fix((*positions.add(i as usize)).x as f64);
                            (*locations.add(i as usize)).y =
                                D2Fix((*positions.add(i as usize)).y as f64);
                        }
                        width = (*positions.add(totalGlyphCount as usize)).x as f64;
                    }

                    *node_width(node) = D2Fix(width);
                    *native_glyph_count(node) = totalGlyphCount as u16;
                    *native_glyph_info_ptr(node) = glyph_info;
                    if tt_checkpoint_enabled {
                        tt_checkpoint_register_glyph(node.offset_from(mem) as i32);
                    }

                    libc::free(glyphs as *mut c_void);
                    libc::free(positions as *mut c_void);
                    libc::free(advances as *mut c_void);
                }

                ubidi_close(pBiDi);

                let font_ls = *font_letter_space.add(f);
                if font_ls != 0 {
                    let mut lsDelta: Fixed = 0;
                    let lsUnit = font_ls;
                    for i in 0..totalGlyphCount {
                        if *glyphAdvances.add(i as usize) == 0 && lsDelta != 0 {
                            lsDelta -= lsUnit;
                        }
                        (*locations.add(i as usize)).x += lsDelta;
                        lsDelta += lsUnit;
                    }
                    if lsDelta != 0 {
                        lsDelta -= lsUnit;
                        *node_width(node) += lsDelta;
                    }
                }
                libc::free(glyphAdvances as *mut c_void);
            } else {
                _tt_abort(
                    b"bad native font flag in `measure_native_node`\0".as_ptr() as *const c_char
                );
            }
        }

        if use_glyph_metrics == 0 || *native_glyph_count(node) == 0 {
            *node_height(node) = *height_base.add(f);
            *node_depth(node) = *depth_base.add(f);
        } else {
            let locations = *native_glyph_info_ptr(node) as *mut FixedPoint;
            let glyph_cnt = *native_glyph_count(node) as usize;
            let glyphIDs = locations.add(glyph_cnt) as *mut u16;
            let mut yMin = 65536.0f32;
            let mut yMax = -65536.0f32;

            for i in 0..glyph_cnt {
                let y = Fix2D(-(*locations.add(i)).y) as f32;
                let mut bbox = GlyphBBox::default();
                let gid = *glyphIDs.add(i);

                if getCachedGlyphBBox(f as u16, gid, &mut bbox) == 0 {
                    let mut got_bbox = false;
                    #[cfg(target_os = "macos")]
                    if *font_area.add(f) as u32 == AAT_FONT_FLAG {
                        GetGlyphBBox_AAT(*font_layout_engine.add(f), gid, &mut bbox);
                        got_bbox = true;
                    }
                    if !got_bbox && *font_area.add(f) as u32 == OTGR_FONT_FLAG {
                        getGlyphBounds(
                            *font_layout_engine.add(f) as XeTeXLayoutEngine,
                            gid as u32,
                            &mut bbox,
                        );
                    }

                    cacheGlyphBBox(f as u16, gid, &bbox);
                }

                let ht = bbox.yMax;
                let dp = -bbox.yMin;

                if y + ht > yMax {
                    yMax = y + ht;
                }
                if y - dp < yMin {
                    yMin = y - dp;
                }
            }

            *node_height(node) = D2Fix(yMax as f64);
            *node_depth(node) = -D2Fix(yMin as f64);
        }
    }
}

#[unsafe(export_name = "real_get_native_italic_correction")]
pub unsafe extern "C" fn real_get_native_italic_correction(pNode: *mut c_void) -> Fixed {
    unsafe {
        let node = pNode as *mut memory_word;
        let f = *native_font(node) as usize;
        let n = *native_glyph_count(node) as usize;
        if n > 0 {
            let locations = *native_glyph_info_ptr(node) as *mut FixedPoint;
            let glyphIDs = locations.add(n) as *mut u16;
            let last_gid = *glyphIDs.add(n - 1);
            let area = *font_area.add(f) as u32;

            #[cfg(target_os = "macos")]
            if area == AAT_FONT_FLAG {
                return D2Fix(GetGlyphItalCorr_AAT(*font_layout_engine.add(f), last_gid))
                    + *font_letter_space.add(f);
            }

            if area == OTGR_FONT_FLAG {
                let engine = *font_layout_engine.add(f) as XeTeXLayoutEngine;
                return D2Fix(getGlyphItalCorr(engine, last_gid as u32) as f64)
                    + *font_letter_space.add(f);
            }
        }
        0
    }
}

#[unsafe(export_name = "real_get_native_glyph_italic_correction")]
pub unsafe extern "C" fn real_get_native_glyph_italic_correction(pNode: *mut c_void) -> Fixed {
    unsafe {
        let node = pNode as *mut memory_word;
        let gid = *native_glyph(node);
        let f = *native_font(node) as usize;
        let area = *font_area.add(f) as u32;

        #[cfg(target_os = "macos")]
        if area == AAT_FONT_FLAG {
            return D2Fix(GetGlyphItalCorr_AAT(*font_layout_engine.add(f), gid));
        }

        if area == OTGR_FONT_FLAG {
            let engine = *font_layout_engine.add(f) as XeTeXLayoutEngine;
            return D2Fix(getGlyphItalCorr(engine, gid as u32) as f64);
        }

        0
    }
}

#[unsafe(export_name = "measure_native_glyph")]
pub unsafe extern "C" fn measure_native_glyph(pNode: *mut c_void, use_glyph_metrics: c_int) {
    unsafe {
        let node = pNode as *mut memory_word;
        let gid = *native_glyph(node);
        let f = *native_font(node) as usize;
        let area = *font_area.add(f) as u32;

        let mut ht: f32 = 0.0;
        let mut dp: f32 = 0.0;

        let mut handled = false;
        #[cfg(target_os = "macos")]
        if area == AAT_FONT_FLAG {
            let attributes = *font_layout_engine.add(f);
            *node_width(node) = D2Fix(GetGlyphWidth_AAT(attributes, gid));
            if use_glyph_metrics != 0 {
                GetGlyphHeightDepth_AAT(attributes, gid, &mut ht, &mut dp);
            }
            handled = true;
        }

        if !handled {
            if area == OTGR_FONT_FLAG {
                let engine = *font_layout_engine.add(f) as XeTeXLayoutEngine;
                let fontInst = getFont(engine);
                *node_width(node) = D2Fix(getGlyphWidth(fontInst, gid as u32) as f64);
                if use_glyph_metrics != 0 {
                    getGlyphHeightDepth(engine, gid as u32, &mut ht, &mut dp);
                }
            } else {
                _tt_abort(
                    b"bad native font flag in `measure_native_glyph`\0".as_ptr() as *const c_char
                );
            }
        }

        if use_glyph_metrics != 0 {
            *node_height(node) = D2Fix(ht as f64);
            *node_depth(node) = D2Fix(dp as f64);
        } else {
            *node_height(node) = *height_base.add(f);
            *node_depth(node) = *depth_base.add(f);
        }
    }
}

#[unsafe(export_name = "map_char_to_glyph")]
pub unsafe extern "C" fn map_char_to_glyph(font: i32, ch: i32) -> i32 {
    unsafe {
        if ch > 0x10ffff || (ch >= 0xd800 && ch <= 0xdfff) {
            return 0;
        }
        let area = *font_area.add(font as usize) as u32;

        #[cfg(target_os = "macos")]
        if area == AAT_FONT_FLAG {
            return MapCharToGlyph_AAT(*font_layout_engine.add(font as usize), ch as u32);
        }

        if area == OTGR_FONT_FLAG {
            let engine = *font_layout_engine.add(font as usize) as XeTeXLayoutEngine;
            return mapCharToGlyph(engine, ch as u32) as i32;
        }

        _tt_abort(b"bad native font flag in `map_char_to_glyph`\0".as_ptr() as *const c_char);
    }
}

pub unsafe fn map_glyph_to_index(font: i32) -> i32 {
    unsafe {
        let area = *font_area.add(font as usize) as u32;

        #[cfg(target_os = "macos")]
        if area == AAT_FONT_FLAG {
            return MapGlyphToIndex_AAT(*font_layout_engine.add(font as usize), name_of_file);
        }

        if area == OTGR_FONT_FLAG {
            let engine = *font_layout_engine.add(font as usize) as XeTeXLayoutEngine;
            return mapGlyphToIndex(engine, name_of_file);
        }

        _tt_abort(b"bad native font flag in `map_glyph_to_index`\0".as_ptr() as *const c_char);
    }
}

#[unsafe(export_name = "get_font_char_range")]
pub unsafe extern "C" fn get_font_char_range(font: i32, first: c_int) -> i32 {
    unsafe {
        let area = *font_area.add(font as usize) as u32;

        #[cfg(target_os = "macos")]
        if area == AAT_FONT_FLAG {
            return GetFontCharRange_AAT(*font_layout_engine.add(font as usize), first);
        }

        if area == OTGR_FONT_FLAG {
            let engine = *font_layout_engine.add(font as usize) as XeTeXLayoutEngine;
            return getFontCharRange(engine, first);
        }

        _tt_abort(b"bad native font flag in `get_font_char_range'`\0".as_ptr() as *const c_char);
    }
}

#[unsafe(export_name = "D2Fix")]
pub extern "C" fn D2Fix(d: f64) -> Fixed {
    (d * 65536.0 + 0.5) as Fixed
}

#[unsafe(export_name = "Fix2D")]
pub extern "C" fn Fix2D(f: Fixed) -> f64 {
    f as f64 / 65536.0
}

#[unsafe(export_name = "aat_get_font_metrics")]
pub unsafe extern "C" fn aat_get_font_metrics(
    attributes: CFDictionaryRef,
    ascent: *mut i32,
    descent: *mut i32,
    xheight: *mut i32,
    capheight: *mut i32,
    slant: *mut i32,
) {
    unsafe {
        #[cfg(target_os = "macos")]
        {
            let font = fontFromAttributes(attributes);
            *ascent = D2Fix(CTFontGetAscent(font));
            *descent = D2Fix(CTFontGetDescent(font));
            *xheight = D2Fix(CTFontGetXHeight(font));
            *capheight = D2Fix(CTFontGetCapHeight(font));
            *slant = D2Fix((-CTFontGetSlantAngle(font) * std::f64::consts::PI / 180.0).tan());
        }
    }
}

#[unsafe(export_name = "aat_font_get")]
pub unsafe extern "C" fn aat_font_get(what: c_int, attributes: CFDictionaryRef) -> c_int {
    unsafe {
        let mut rval = -1;
        #[cfg(target_os = "macos")]
        {
            let font = fontFromAttributes(attributes);
            match what {
                XeTeX_count_glyphs => {
                    rval = CTFontGetGlyphCount(font) as c_int;
                }
                XeTeX_count_features => {
                    let list = CTFontCopyFeatures(font);
                    if !list.is_null() {
                        rval = CFArrayGetCount(list) as c_int;
                        CFRelease(list);
                    }
                }
                _ => {}
            }
        }
        rval
    }
}

#[unsafe(export_name = "aat_font_get_1")]
pub unsafe extern "C" fn aat_font_get_1(
    what: c_int,
    attributes: CFDictionaryRef,
    param: c_int,
) -> c_int {
    unsafe {
        let mut rval = -1;
        #[cfg(target_os = "macos")]
        {
            let font = fontFromAttributes(attributes);
            match what {
                XeTeX_feature_code => {
                    let features = CTFontCopyFeatures(font);
                    if !features.is_null() {
                        if (CFArrayGetCount(features) as c_int) > param {
                            let feature = CFArrayGetValueAtIndex(features, param as libc::c_long);
                            let identifier =
                                CFDictionaryGetValue(feature, kCTFontFeatureTypeIdentifierKey);
                            if !identifier.is_null() {
                                CFNumberGetValue(
                                    identifier,
                                    kCFNumberIntType.into(),
                                    &mut rval as *mut c_int as *mut c_void,
                                );
                            }
                        }
                        CFRelease(features);
                    }
                }
                XeTeX_is_exclusive_feature => {
                    let features = CTFontCopyFeatures(font);
                    if !features.is_null() {
                        let mut value: CFBooleanRef = std::ptr::null_mut();
                        let feature = findDictionaryInArrayWithIdentifier(
                            features,
                            kCTFontFeatureTypeIdentifierKey,
                            param,
                        );
                        let found = CFDictionaryGetValueIfPresent(
                            feature,
                            kCTFontFeatureTypeExclusiveKey,
                            &mut value as *mut CFBooleanRef as *mut *const c_void,
                        );
                        if found != 0 {
                            rval = CFBooleanGetValue(value);
                        }
                        CFRelease(features);
                    }
                }
                XeTeX_count_selectors => {
                    let features = CTFontCopyFeatures(font);
                    if !features.is_null() {
                        let feature = findDictionaryInArrayWithIdentifier(
                            features,
                            kCTFontFeatureTypeIdentifierKey,
                            param,
                        );
                        if !feature.is_null() {
                            let selectors =
                                CFDictionaryGetValue(feature, kCTFontFeatureTypeSelectorsKey);
                            if !selectors.is_null() {
                                rval = CFArrayGetCount(selectors) as c_int;
                            }
                        }
                        CFRelease(features);
                    }
                }
                _ => {}
            }
        }
        rval
    }
}

#[unsafe(export_name = "aat_font_get_2")]
pub unsafe extern "C" fn aat_font_get_2(
    what: c_int,
    attributes: CFDictionaryRef,
    param1: c_int,
    param2: c_int,
) -> c_int {
    unsafe {
        let mut rval = -1;
        #[cfg(target_os = "macos")]
        {
            let font = fontFromAttributes(attributes);
            let features = CTFontCopyFeatures(font);
            if !features.is_null() {
                let feature = findDictionaryInArrayWithIdentifier(
                    features,
                    kCTFontFeatureTypeIdentifierKey,
                    param1,
                );
                if !feature.is_null() {
                    let selectors = CFDictionaryGetValue(feature, kCTFontFeatureTypeSelectorsKey);
                    if !selectors.is_null() {
                        match what {
                            XeTeX_selector_code => {
                                if (CFArrayGetCount(selectors) as c_int) > param2 {
                                    let selector =
                                        CFArrayGetValueAtIndex(selectors, param2 as libc::c_long);
                                    let identifier = CFDictionaryGetValue(
                                        selector,
                                        kCTFontFeatureSelectorIdentifierKey,
                                    );
                                    if !identifier.is_null() {
                                        CFNumberGetValue(
                                            identifier,
                                            kCFNumberIntType.into(),
                                            &mut rval as *mut c_int as *mut c_void,
                                        );
                                    }
                                }
                            }
                            XeTeX_is_default_selector => {
                                let selector = findDictionaryInArrayWithIdentifier(
                                    selectors,
                                    kCTFontFeatureSelectorIdentifierKey,
                                    param2,
                                );
                                if !selector.is_null() {
                                    let mut isDefault: CFBooleanRef = std::ptr::null_mut();
                                    let found = CFDictionaryGetValueIfPresent(
                                        selector,
                                        kCTFontFeatureSelectorDefaultKey,
                                        &mut isDefault as *mut CFBooleanRef as *mut *const c_void,
                                    );
                                    if found != 0 {
                                        rval = CFBooleanGetValue(isDefault);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                CFRelease(features);
            }
        }
        rval
    }
}

#[unsafe(export_name = "aat_font_get_named")]
pub unsafe extern "C" fn aat_font_get_named(what: c_int, attributes: CFDictionaryRef) -> c_int {
    unsafe {
        let mut rval = -1;
        #[cfg(target_os = "macos")]
        if what == XeTeX_find_feature_by_name {
            let font = fontFromAttributes(attributes);
            let features = CTFontCopyFeatures(font);
            if !features.is_null() {
                let feature = findDictionaryInArray(
                    features,
                    kCTFontFeatureTypeNameKey,
                    name_of_file,
                    name_length,
                );
                if !feature.is_null() {
                    let identifier = CFDictionaryGetValue(feature, kCTFontFeatureTypeIdentifierKey);
                    CFNumberGetValue(
                        identifier,
                        kCFNumberIntType.into(),
                        &mut rval as *mut c_int as *mut c_void,
                    );
                }
                CFRelease(features);
            }
        }
        rval
    }
}

#[unsafe(export_name = "aat_font_get_named_1")]
pub unsafe extern "C" fn aat_font_get_named_1(
    what: c_int,
    attributes: CFDictionaryRef,
    param: c_int,
) -> c_int {
    unsafe {
        let mut rval = -1;
        #[cfg(target_os = "macos")]
        if what == XeTeX_find_selector_by_name {
            let font = fontFromAttributes(attributes);
            let features = CTFontCopyFeatures(font);
            if !features.is_null() {
                let feature = findDictionaryInArrayWithIdentifier(
                    features,
                    kCTFontFeatureTypeIdentifierKey,
                    param,
                );
                if !feature.is_null() {
                    let selector = findSelectorByName(feature, name_of_file, name_length);
                    if !selector.is_null() {
                        CFNumberGetValue(
                            selector,
                            kCFNumberIntType.into(),
                            &mut rval as *mut c_int as *mut c_void,
                        );
                    }
                }
                CFRelease(features);
            }
        }
        rval
    }
}

#[unsafe(export_name = "aat_print_font_name")]
pub unsafe extern "C" fn aat_print_font_name(
    what: c_int,
    attributes: CFDictionaryRef,
    param1: c_int,
    param2: c_int,
) {
    unsafe {
        #[cfg(target_os = "macos")]
        {
            let mut name: CFStringRef = std::ptr::null_mut();
            if what == XETEX_FEATURE_NAME_CODE || what == XETEX_SELECTOR_NAME_CODE {
                let font = fontFromAttributes(attributes);
                let features = CTFontCopyFeatures(font);
                if !features.is_null() {
                    let feature = findDictionaryInArrayWithIdentifier(
                        features,
                        kCTFontFeatureTypeIdentifierKey,
                        param1,
                    );
                    if !feature.is_null() {
                        if what == XETEX_FEATURE_NAME_CODE {
                            name = CFDictionaryGetValue(feature, kCTFontFeatureTypeNameKey)
                                as CFStringRef;
                        } else {
                            let selectors =
                                CFDictionaryGetValue(feature, kCTFontFeatureTypeSelectorsKey);
                            let selector = findDictionaryInArrayWithIdentifier(
                                selectors,
                                kCTFontFeatureSelectorIdentifierKey,
                                param2,
                            );
                            if !selector.is_null() {
                                name = CFDictionaryGetValue(selector, kCTFontFeatureSelectorNameKey)
                                    as CFStringRef;
                            }
                        }
                    }
                    CFRelease(features);
                }
            }

            if !name.is_null() {
                let len = CFStringGetLength(name);
                let buf = xcalloc(len as usize, std::mem::size_of::<UniChar>()) as *mut UniChar;
                CFStringGetCharacters(name, CFRangeMake(0, len), buf);
                print_chars(buf, len as c_int);
                libc::free(buf as *mut c_void);
            }
        }
    }
}

#[cfg(target_os = "macos")]
unsafe fn cgColorToRGBA32(color: CGColorRef) -> u32 {
    unsafe {
        let components = CGColorGetComponents(color);
        let mut rval: u32 = (*components.add(0) * 255.0 + 0.5) as u8 as u32;
        rval <<= 8;
        rval += (*components.add(1) * 255.0 + 0.5) as u8 as u32;
        rval <<= 8;
        rval += (*components.add(2) * 255.0 + 0.5) as u8 as u32;
        rval <<= 8;
        rval += (*components.add(3) * 255.0 + 0.5) as u8 as u32;
        rval
    }
}

#[unsafe(export_name = "print_glyph_name")]
pub unsafe extern "C" fn print_glyph_name(font: i32, gid: i32) {
    unsafe {
        let mut s: *const c_char = std::ptr::null();
        let mut len: c_int = 0;

        let area = *font_area.add(font as usize) as u32;

        let mut handled = false;
        #[cfg(target_os = "macos")]
        if area == AAT_FONT_FLAG {
            s = GetGlyphNameFromCTFont(fontFromInteger(font), gid as u16, &mut len);
            handled = true;
        }

        if !handled {
            if area == OTGR_FONT_FLAG {
                let engine = *font_layout_engine.add(font as usize) as XeTeXLayoutEngine;
                s = getGlyphName(getFont(engine), gid as u32, &mut len);
            } else {
                _tt_abort(b"bad native font flag in `print_glyph_name`\0".as_ptr() as *const c_char);
            }
        }

        if !s.is_null() {
            for i in 0..len {
                print_char(*s.add(i as usize) as u8 as i32);
            }
            freeGlyphName(s as *mut c_char);
        }
    }
}

#[unsafe(export_name = "real_get_native_word_cp")]
pub unsafe extern "C" fn real_get_native_word_cp(pNode: *mut c_void, side: c_int) -> i32 {
    unsafe {
        let node = pNode as *mut memory_word;
        let glyphCount = *native_glyph_count(node) as usize;
        if glyphCount == 0 {
            return 0;
        }

        let locations = *native_glyph_info_ptr(node) as *mut FixedPoint;
        let glyphIDs = locations.add(glyphCount) as *mut u16;
        let f = *native_font(node) as i32;

        let actual_glyph = match side {
            LEFT_SIDE => *glyphIDs,
            RIGHT_SIDE => *glyphIDs.add(glyphCount - 1),
            _ => panic!("invalid side in real_get_native_word_cp"),
        };

        get_cp_code(f, actual_glyph as u32, side)
    }
}
