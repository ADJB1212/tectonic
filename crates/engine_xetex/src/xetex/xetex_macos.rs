#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use super::xetex_checkpoint::{tt_checkpoint_enabled, tt_checkpoint_register_glyph};
use super::xetex_constants::NATIVE_NODE_SIZE;
use super::xetex_ext::xmalloc;
use super::xetex_xetexd::{
    font_area, font_feature_warning, font_layout_engine, font_letter_space, loaded_font_flags, loaded_font_letter_space, mem, memory_word,
    native_font_type_flag,
};

use libc::{c_char, c_int, c_long, c_uint, c_void, size_t};
use std::ptr;

use super::xetex_checkpoint::*;
use super::xetex_constants::*;
use super::xetex_ext::*;
use super::xetex_xetex_ot_math::{D2Fix, Fix2D};
use super::xetex_xetexd::*;

pub type CFTypeRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFMutableDictionaryRef = *mut c_void;
pub type CFArrayRef = *const c_void;
pub type CFMutableArrayRef = *mut c_void;
pub type CFNumberRef = *const c_void;
pub type CFBooleanRef = *const c_void;
pub type CFAttributedStringRef = *const c_void;
pub type CTFontDescriptorRef = *const c_void;
pub type CTFontRef = *const c_void;
pub type CTLineRef = *const c_void;
pub type CTRunRef = *const c_void;
pub type CTTypesetterRef = *const c_void;
pub type CGFontRef = *const c_void;
pub type CGColorRef = *const c_void;

pub type CFIndex = c_long;
pub type CFStringEncoding = u32;
pub type CGFloat = f64;
pub type CGGlyph = u16;
pub type UniChar = u16;

pub const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;
pub const kCFNumberIntType: CFIndex = 9;
pub const kCFNumberFloatType: CFIndex = 12;
pub const kCFNumberDoubleType: CFIndex = 13;
pub const kCFCompareCaseInsensitive: CFIndex = 1;

pub const kCTFontOrientationHorizontal: c_uint = 0;
pub const kCTFontOrientationVertical: c_uint = 1;

pub const fract1: i32 = 0x40000000;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex,
}

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

pub const CGAffineTransformIdentity: CGAffineTransform = CGAffineTransform {
    a: 1.0,
    b: 0.0,
    c: 0.0,
    d: 1.0,
    tx: 0.0,
    ty: 0.0,
};

#[inline]
pub fn CGAffineTransformMake(a: CGFloat, b: CGFloat, c: CGFloat, d: CGFloat, tx: CGFloat, ty: CGFloat) -> CGAffineTransform {
    CGAffineTransform { a, b, c, d, tx, ty }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FixedPoint {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct GlyphBBox {
    pub xMin: f32,
    pub yMin: f32,
    pub xMax: f32,
    pub yMax: f32,
}

unsafe extern "C" {
    pub static kCFAllocatorDefault: CFTypeRef;
    pub static kCFAllocatorNull: CFTypeRef;
    pub static kCFTypeDictionaryKeyCallBacks: c_void;
    pub static kCFTypeDictionaryValueCallBacks: c_void;
    pub static kCFTypeArrayCallBacks: c_void;
    pub static kCFBooleanTrue: CFBooleanRef;

    pub static kCTFontAttributeName: CFStringRef;
    pub static kCTVerticalFormsAttributeName: CFStringRef;
    pub static kCTFontFeatureTypeSelectorsKey: CFStringRef;
    pub static kCTFontFeatureSelectorNameKey: CFStringRef;
    pub static kCTFontFeatureSelectorIdentifierKey: CFStringRef;
    pub static kCTFontFeatureTypeNameKey: CFStringRef;
    pub static kCTFontFeatureTypeIdentifierKey: CFStringRef;
    pub static kCTKernAttributeName: CFStringRef;
    pub static kCTFontOrientationAttribute: CFStringRef;
    pub static kCTFontFeatureSettingsAttribute: CFStringRef;
    pub static kCTForegroundColorAttributeName: CFStringRef;
    pub static kCTFontCascadeListAttribute: CFStringRef;

    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFEqual(cf1: CFTypeRef, cf2: CFTypeRef) -> bool;
    pub fn CFDictionaryGetValue(theDict: CFDictionaryRef, key: *const c_void) -> *const c_void;
    pub fn CFDictionaryAddValue(theDict: CFMutableDictionaryRef, key: *const c_void, value: *const c_void);
    pub fn CFDictionaryCreate(
        allocator: CFTypeRef, keys: *const *const c_void, values: *const *const c_void, numValues: CFIndex, keyCallBacks: *const c_void,
        valueCallBacks: *const c_void,
    ) -> CFDictionaryRef;
    pub fn CFDictionaryCreateMutable(
        allocator: CFTypeRef, capacity: CFIndex, keyCallBacks: *const c_void, valueCallBacks: *const c_void,
    ) -> CFMutableDictionaryRef;

    pub fn CFArrayGetCount(theArray: CFArrayRef) -> CFIndex;
    pub fn CFArrayGetValueAtIndex(theArray: CFArrayRef, idx: CFIndex) -> *const c_void;
    pub fn CFArrayAppendValue(theArray: CFMutableArrayRef, value: *const c_void);
    pub fn CFArrayCreateMutable(allocator: CFTypeRef, capacity: CFIndex, callBacks: *const c_void) -> CFMutableArrayRef;

    pub fn CFStringCreateWithCharactersNoCopy(
        alloc: CFTypeRef, chars: *const UniChar, numChars: CFIndex, contentsDeallocator: CFTypeRef,
    ) -> CFStringRef;
    pub fn CFStringCreateWithCStringNoCopy(
        alloc: CFTypeRef, cStr: *const c_char, encoding: CFStringEncoding, contentsDeallocator: CFTypeRef,
    ) -> CFStringRef;
    pub fn CFStringCreateWithBytes(
        alloc: CFTypeRef, bytes: *const u8, numBytes: CFIndex, encoding: CFStringEncoding, isExternalRepresentation: bool,
    ) -> CFStringRef;
    pub fn CFStringCompare(theString1: CFStringRef, theString2: CFStringRef, compareOptions: CFIndex) -> CFIndex;
    pub fn CFStringGetCString(theString: CFStringRef, buffer: *mut c_char, bufferSize: CFIndex, encoding: CFStringEncoding) -> bool;

    pub fn CFNumberCreate(allocator: CFTypeRef, theType: CFIndex, valuePtr: *const c_void) -> CFNumberRef;
    pub fn CFNumberGetValue(number: CFNumberRef, theType: CFIndex, valuePtr: *mut c_void) -> u8;
    pub fn CFNumberCompare(number: CFNumberRef, otherNumber: CFNumberRef, context: *mut c_void) -> CFIndex;

    pub fn CFAttributedStringCreate(alloc: CFTypeRef, str: CFStringRef, attributes: CFDictionaryRef) -> CFAttributedStringRef;

    pub fn CTTypesetterCreateWithAttributedString(string: CFAttributedStringRef) -> CTTypesetterRef;
    pub fn CTTypesetterCreateLine(typesetter: CTTypesetterRef, stringRange: CFRange) -> CTLineRef;
    pub fn CTLineCreateJustifiedLine(ct_line: CTLineRef, justificationFactor: CGFloat, justificationWidth: CGFloat) -> CTLineRef;
    pub fn CTLineGetGlyphRuns(ct_line: CTLineRef) -> CFArrayRef;
    pub fn CTLineGetGlyphCount(ct_line: CTLineRef) -> CFIndex;

    pub fn CTRunGetGlyphCount(run: CTRunRef) -> CFIndex;
    pub fn CTRunGetAttributes(run: CTRunRef) -> CFDictionaryRef;
    pub fn CTRunGetTypographicBounds(run: CTRunRef, range: CFRange, ascent: *mut CGFloat, descent: *mut CGFloat, leading: *mut CGFloat) -> CGFloat;
    pub fn CTRunGetGlyphs(run: CTRunRef, range: CFRange, buffer: *mut CGGlyph);
    pub fn CTRunGetPositions(run: CTRunRef, range: CFRange, buffer: *mut CGPoint);
    pub fn CTRunGetAdvances(run: CTRunRef, range: CFRange, buffer: *mut CGSize);

    pub fn CTFontCreateWithFontDescriptor(descriptor: CTFontDescriptorRef, size: CGFloat, matrix: *const CGAffineTransform) -> CTFontRef;
    pub fn CTFontCreateCopyWithAttributes(
        currentFont: CTFontRef, size: CGFloat, matrix: *const CGAffineTransform, attributes: CTFontDescriptorRef,
    ) -> CTFontRef;
    pub fn CTFontDescriptorCreateWithNameAndSize(name: CFStringRef, size: CGFloat) -> CTFontDescriptorRef;
    pub fn CTFontDescriptorCreateWithAttributes(attributes: CFDictionaryRef) -> CTFontDescriptorRef;
    pub fn CTFontCopyFeatures(font: CTFontRef) -> CFArrayRef;
    pub fn CTFontGetBoundingRectsForGlyphs(
        font: CTFontRef, orientation: c_uint, glyphs: *const CGGlyph, boundingRects: *mut CGRect, count: CFIndex,
    ) -> CGRect;
    pub fn CTFontGetAdvancesForGlyphs(font: CTFontRef, orientation: c_uint, glyphs: *const CGGlyph, advances: *mut CGSize, count: CFIndex) -> f64;
    pub fn CTFontGetGlyphsForCharacters(font: CTFontRef, characters: *const UniChar, glyphs: *mut CGGlyph, count: CFIndex) -> bool;
    pub fn CTFontGetGlyphWithName(font: CTFontRef, glyphName: CFStringRef) -> CGGlyph;
    pub fn CTFontCopyGraphicsFont(font: CTFontRef, attributes: *mut CTFontDescriptorRef) -> CGFontRef;

    pub fn CGFontGetNumberOfGlyphs(font: CGFontRef) -> size_t;
    pub fn CGFontCopyGlyphNameForGlyph(font: CGFontRef, glyph: CGGlyph) -> CFStringRef;
    pub fn CGFontRelease(font: CGFontRef);
    pub fn CGColorCreateGenericRGB(red: CGFloat, green: CGFloat, blue: CGFloat, alpha: CGFloat) -> CGColorRef;
    pub fn CGColorRelease(color: CGColorRef);
    pub fn CGRectIsNull(rect: CGRect) -> bool;
}

#[inline]
pub fn TeXtoPSPoints(pts: f64) -> f64 {
    pts * 72.0 / 72.27
}

#[unsafe(export_name = "PStoTeXPoints")]
pub extern "C" fn PStoTeXPoints(pts: f64) -> f64 {
    pts * 72.27 / 72.0
}

#[unsafe(export_name = "FixedPStoTeXPoints")]
pub extern "C" fn FixedPStoTeXPoints(pts: f64) -> i32 {
    D2Fix(PStoTeXPoints(pts))
}

#[unsafe(export_name = "fontFromAttributes")]
pub unsafe extern "C" fn fontFromAttributes(attributes: CFDictionaryRef) -> CTFontRef {
    unsafe { CFDictionaryGetValue(attributes, kCTFontAttributeName as *const c_void) as CTFontRef }
}

#[unsafe(export_name = "fontFromInteger")]
pub unsafe extern "C" fn fontFromInteger(font: i32) -> CTFontRef {
    unsafe {
        let attributes = (*font_layout_engine.add(font as usize)) as CFDictionaryRef;
        fontFromAttributes(attributes)
    }
}

#[unsafe(export_name = "DoAATLayout")]
pub unsafe extern "C" fn DoAATLayout(p: *mut c_void, justify: c_int) {
    unsafe {
        let node = p as *mut memory_word;

        let f = (*node.add(native_info_offset)).b16.s2 as usize;
        if (*font_area.add(f)) != AAT_FONT_FLAG as i32 {
            crate::xetex::xetex_errors::_tt_abort(c"DoAATLayout called for non-AAT font".as_ptr());
        }

        let txtLen = (*node.add(native_info_offset)).b16.s1 as CFIndex;
        let txtPtr = node.add(NATIVE_NODE_SIZE as usize) as *const UniChar;

        let attributes = (*font_layout_engine.add(f)) as CFDictionaryRef;
        let string = CFStringCreateWithCharactersNoCopy(ptr::null(), txtPtr, txtLen, kCFAllocatorNull);
        let attrString = CFAttributedStringCreate(ptr::null(), string, attributes);
        CFRelease(string as CFTypeRef);

        let typesetter = CTTypesetterCreateWithAttributedString(attrString);
        CFRelease(attrString as CFTypeRef);
        let mut ct_line = CTTypesetterCreateLine(typesetter, CFRange { location: 0, length: txtLen });

        if justify != 0 {
            let lineWidth = TeXtoPSPoints(Fix2D((*node.add(width_offset)).b32.s1));
            let justifiedLine = CTLineCreateJustifiedLine(ct_line, TeXtoPSPoints(Fix2D(fract1)), lineWidth);
            if !justifiedLine.is_null() {
                CFRelease(ct_line as CFTypeRef);
                ct_line = justifiedLine;
            }
        }

        let glyphRuns = CTLineGetGlyphRuns(ct_line);
        let runCount = CFArrayGetCount(glyphRuns);
        let mut totalGlyphCount = CTLineGetGlyphCount(ct_line);

        let mut glyph_info: *mut c_void = ptr::null_mut();
        let mut glyphAdvances: *mut i32 = ptr::null_mut();
        let mut width: CGFloat = 0.0;

        if totalGlyphCount > 0 {
            glyph_info = xmalloc((totalGlyphCount as usize) * (native_glyph_info_size as usize));
            let locations = glyph_info as *mut FixedPoint;
            let glyphIDs = locations.add(totalGlyphCount as usize) as *mut u16;
            glyphAdvances = xmalloc((totalGlyphCount as usize) * std::mem::size_of::<i32>()) as *mut i32;
            totalGlyphCount = 0;

            for i in 0..runCount {
                let run = CFArrayGetValueAtIndex(glyphRuns, i) as CTRunRef;
                let count = CTRunGetGlyphCount(run);
                let runAttributes = CTRunGetAttributes(run);
                let vertical = CFDictionaryGetValue(runAttributes, kCTVerticalFormsAttributeName as *const c_void) as CFBooleanRef;

                let glyphs = xmalloc((count as usize) * std::mem::size_of::<CGGlyph>()) as *mut CGGlyph;
                let positions = xmalloc((count as usize) * std::mem::size_of::<CGPoint>()) as *mut CGPoint;
                let advances = xmalloc((count as usize) * std::mem::size_of::<CGSize>()) as *mut CGSize;

                let runWidth = CTRunGetTypographicBounds(run, CFRange { location: 0, length: 0 }, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
                CTRunGetGlyphs(run, CFRange { location: 0, length: 0 }, glyphs);
                CTRunGetPositions(run, CFRange { location: 0, length: 0 }, positions);
                CTRunGetAdvances(run, CFRange { location: 0, length: 0 }, advances);

                for j in 0..count {
                    if !CFEqual(
                        fontFromAttributes(attributes) as CFTypeRef,
                        fontFromAttributes(runAttributes) as CFTypeRef,
                    ) {
                        *glyphIDs.add(totalGlyphCount as usize) = 0;
                    } else {
                        *glyphIDs.add(totalGlyphCount as usize) = *glyphs.add(j as usize);
                    }

                    if vertical == kCFBooleanTrue {
                        (*locations.add(totalGlyphCount as usize)).x = -FixedPStoTeXPoints((*positions.add(j as usize)).y);
                        (*locations.add(totalGlyphCount as usize)).y = FixedPStoTeXPoints((*positions.add(j as usize)).x);
                    } else {
                        (*locations.add(totalGlyphCount as usize)).x = FixedPStoTeXPoints((*positions.add(j as usize)).x);
                        (*locations.add(totalGlyphCount as usize)).y = -FixedPStoTeXPoints((*positions.add(j as usize)).y);
                    }
                    *glyphAdvances.add(totalGlyphCount as usize) = (*advances.add(j as usize)).width as i32;
                    totalGlyphCount += 1;
                }

                width += FixedPStoTeXPoints(runWidth) as CGFloat;
                libc::free(glyphs as *mut c_void);
                libc::free(positions as *mut c_void);
                libc::free(advances as *mut c_void);
            }
        }

        (*node.add(native_info_offset)).b16.s0 = totalGlyphCount as u16;
        (*node.add(native_glyph_info_offset)).ptr = glyph_info;
        if tt_checkpoint_enabled {
            tt_checkpoint_register_glyph(node.offset_from(mem) as i32);
        }

        if justify == 0 {
            (*node.add(width_offset)).b32.s1 = width as i32;

            if totalGlyphCount > 0 && (*font_letter_space.add(f)) != 0 {
                let mut lsDelta: i32 = 0;
                let lsUnit = *font_letter_space.add(f);
                let locations = glyph_info as *mut FixedPoint;
                for i in 0..totalGlyphCount {
                    if *glyphAdvances.add(i as usize) == 0 && lsDelta != 0 {
                        lsDelta -= lsUnit;
                    }
                    (*locations.add(i as usize)).x += lsDelta;
                    lsDelta += lsUnit;
                }
                if lsDelta != 0 {
                    lsDelta -= lsUnit;
                    (*node.add(width_offset)).b32.s1 += lsDelta;
                }
            }
        }

        if !glyphAdvances.is_null() {
            libc::free(glyphAdvances as *mut c_void);
        }
        CFRelease(ct_line as CFTypeRef);
        CFRelease(typesetter as CFTypeRef);
    }
}

unsafe fn getGlyphBBoxFromCTFont(font: CTFontRef, gid: u16, bbox: *mut GlyphBBox) {
    unsafe {
        (*bbox).xMin = 65536.0;
        (*bbox).yMin = 65536.0;
        (*bbox).xMax = -65536.0;
        (*bbox).yMax = -65536.0;

        let glyph = gid as CGGlyph;
        let rect = CTFontGetBoundingRectsForGlyphs(font, 0, &glyph, ptr::null_mut(), 1);

        if CGRectIsNull(rect) {
            (*bbox).xMin = 0.0;
            (*bbox).yMin = 0.0;
            (*bbox).xMax = 0.0;
            (*bbox).yMax = 0.0;
        } else {
            (*bbox).yMin = PStoTeXPoints(rect.origin.y) as f32;
            (*bbox).yMax = PStoTeXPoints(rect.origin.y + rect.size.height) as f32;
            (*bbox).xMin = PStoTeXPoints(rect.origin.x) as f32;
            (*bbox).xMax = PStoTeXPoints(rect.origin.x + rect.size.width) as f32;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGlyphBBox_AAT(attributes: CFDictionaryRef, gid: u16, bbox: *mut GlyphBBox) {
    unsafe {
        let font = fontFromAttributes(attributes);
        getGlyphBBoxFromCTFont(font, gid, bbox);
    }
}

unsafe fn getGlyphWidthFromCTFont(font: CTFontRef, gid: u16) -> f64 {
    unsafe {
        let glyph = gid as CGGlyph;
        PStoTeXPoints(CTFontGetAdvancesForGlyphs(font, kCTFontOrientationHorizontal, &glyph, ptr::null_mut(), 1))
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGlyphWidth_AAT(attributes: CFDictionaryRef, gid: u16) -> f64 {
    unsafe {
        let font = fontFromAttributes(attributes);
        getGlyphWidthFromCTFont(font, gid)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGlyphHeightDepth_AAT(attributes: CFDictionaryRef, gid: u16, ht: *mut f32, dp: *mut f32) {
    unsafe {
        let mut bbox = GlyphBBox::default();
        GetGlyphBBox_AAT(attributes, gid, &mut bbox);
        *ht = bbox.yMax;
        *dp = -bbox.yMin;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGlyphSidebearings_AAT(attributes: CFDictionaryRef, gid: u16, lsb: *mut f32, rsb: *mut f32) {
    unsafe {
        let font = fontFromAttributes(attributes);
        let mut advances = [CGSize { width: 0.0, height: 0.0 }];
        let glyph = gid as CGGlyph;
        let advance = CTFontGetAdvancesForGlyphs(font, 0, &glyph, advances.as_mut_ptr(), 1);
        let mut bbox = GlyphBBox::default();
        getGlyphBBoxFromCTFont(font, gid, &mut bbox);
        *lsb = bbox.xMin;
        *rsb = (PStoTeXPoints(advance) as f32) - bbox.xMax;
    }
}

#[unsafe(export_name = "GetGlyphItalCorr_AAT")]
pub unsafe extern "C" fn GetGlyphItalCorr_AAT(attributes: CFDictionaryRef, gid: u16) -> f64 {
    unsafe {
        let font = fontFromAttributes(attributes);
        let mut advances = [CGSize { width: 0.0, height: 0.0 }];
        let glyph = gid as CGGlyph;
        let advance = CTFontGetAdvancesForGlyphs(font, 0, &glyph, advances.as_mut_ptr(), 1);

        let mut bbox = GlyphBBox::default();
        getGlyphBBoxFromCTFont(font, gid, &mut bbox);

        let psto_tex = PStoTeXPoints(advance);
        if (bbox.xMax as f64) > psto_tex {
            (bbox.xMax as f64) - psto_tex
        } else {
            0.0
        }
    }
}

unsafe fn mapCharToGlyphFromCTFont(font: CTFontRef, mut ch: u32) -> c_int {
    unsafe {
        let mut glyphs: [CGGlyph; 2] = [0, 0];
        let mut txt: [UniChar; 2] = [0, 0];
        let mut len: CFIndex = 1;

        if ch > 0xffff {
            ch -= 0x10000;
            txt[0] = (0xd800 + ch / 1024) as UniChar;
            txt[1] = (0xdc00 + ch % 1024) as UniChar;
            len = 2;
        } else {
            txt[0] = ch as UniChar;
        }

        if CTFontGetGlyphsForCharacters(font, txt.as_ptr(), glyphs.as_mut_ptr(), len) {
            glyphs[0] as c_int
        } else {
            0
        }
    }
}

#[unsafe(export_name = "MapCharToGlyph_AAT")]
pub unsafe extern "C" fn MapCharToGlyph_AAT(attributes: CFDictionaryRef, ch: u32) -> c_int {
    unsafe {
        let font = fontFromAttributes(attributes);
        mapCharToGlyphFromCTFont(font, ch)
    }
}

unsafe fn GetGlyphIDFromCTFont(ctFontRef: CTFontRef, glyphName: *const c_char) -> c_int {
    unsafe {
        let glyphname = CFStringCreateWithCStringNoCopy(kCFAllocatorDefault, glyphName, kCFStringEncodingUTF8, kCFAllocatorNull);
        let rval = CTFontGetGlyphWithName(ctFontRef, glyphname);
        CFRelease(glyphname as CFTypeRef);
        rval as c_int
    }
}

#[unsafe(export_name = "MapGlyphToIndex_AAT")]
pub unsafe extern "C" fn MapGlyphToIndex_AAT(attributes: CFDictionaryRef, glyphName: *const c_char) -> c_int {
    unsafe {
        let font = fontFromAttributes(attributes);
        GetGlyphIDFromCTFont(font, glyphName)
    }
}

#[unsafe(export_name = "GetGlyphNameFromCTFont")]
pub unsafe extern "C" fn GetGlyphNameFromCTFont(ctFontRef: CTFontRef, gid: u16, len: *mut c_int) -> *mut c_char {
    unsafe {
        static mut BUFFER: [c_char; 256] = [0; 256];
        BUFFER[0] = 0;
        *len = 0;

        let cgfont = CTFontCopyGraphicsFont(ctFontRef, ptr::null_mut());
        if !cgfont.is_null() && (gid as size_t) < CGFontGetNumberOfGlyphs(cgfont) {
            let glyphname = CGFontCopyGlyphNameForGlyph(cgfont, gid as CGGlyph);
            if !glyphname.is_null() {
                if CFStringGetCString(glyphname, (&raw mut BUFFER).cast::<c_char>(), 256, kCFStringEncodingUTF8) {
                    *len = libc::strlen((&raw const BUFFER).cast::<c_char>()) as c_int;
                }
                CFRelease(glyphname as CFTypeRef);
            }
            CGFontRelease(cgfont);
        }

        (&raw mut BUFFER).cast::<c_char>()
    }
}

#[unsafe(export_name = "GetFontCharRange_AAT")]
pub unsafe extern "C" fn GetFontCharRange_AAT(attributes: CFDictionaryRef, reqFirst: c_int) -> c_int {
    unsafe {
        if reqFirst != 0 {
            let mut ch: u32 = 0;
            while MapCharToGlyph_AAT(attributes, ch) == 0 && ch < 0x10ffff {
                ch += 1;
            }
            ch as c_int
        } else {
            let mut ch: u32 = 0x10ffff;
            while MapCharToGlyph_AAT(attributes, ch) == 0 && ch > 0 {
                ch -= 1;
            }
            ch as c_int
        }
    }
}

#[unsafe(export_name = "findDictionaryInArrayWithIdentifier")]
pub unsafe extern "C" fn findDictionaryInArrayWithIdentifier(array: CFArrayRef, identifierKey: *const c_void, identifier: c_int) -> CFDictionaryRef {
    unsafe {
        let mut dict: CFDictionaryRef = ptr::null();

        if !array.is_null() {
            let count = CFArrayGetCount(array);
            for i in 0..count {
                let item = CFArrayGetValueAtIndex(array, i) as CFDictionaryRef;
                let itemId = CFDictionaryGetValue(item, identifierKey) as CFNumberRef;
                if !itemId.is_null() {
                    let mut value: c_int = -1;
                    CFNumberGetValue(itemId, kCFNumberIntType, &mut value as *mut c_int as *mut c_void);
                    if value == identifier {
                        dict = item;
                        break;
                    }
                }
            }
        }
        dict
    }
}

#[unsafe(export_name = "findDictionaryInArray")]
pub unsafe extern "C" fn findDictionaryInArray(array: CFArrayRef, nameKey: *const c_void, name: *const c_char, nameLength: c_int) -> CFDictionaryRef {
    unsafe {
        let mut dict: CFDictionaryRef = ptr::null();

        if !array.is_null() {
            let itemName = CFStringCreateWithBytes(ptr::null(), name as *const u8, nameLength as CFIndex, kCFStringEncodingUTF8, false);
            let count = CFArrayGetCount(array);
            for i in 0..count {
                let item = CFArrayGetValueAtIndex(array, i) as CFDictionaryRef;
                let iName = CFDictionaryGetValue(item, nameKey) as CFStringRef;
                if !iName.is_null() && CFStringCompare(itemName, iName, kCFCompareCaseInsensitive) == 0 {
                    dict = item;
                    break;
                }
            }
            CFRelease(itemName as CFTypeRef);
        }
        dict
    }
}

#[unsafe(export_name = "findSelectorByName")]
pub unsafe extern "C" fn findSelectorByName(feature: CFDictionaryRef, name: *const c_char, nameLength: c_int) -> CFNumberRef {
    unsafe {
        let mut feature_selector: CFNumberRef = ptr::null();
        let selectors = CFDictionaryGetValue(feature, kCTFontFeatureTypeSelectorsKey as *const c_void) as CFArrayRef;
        if !selectors.is_null() {
            let s = findDictionaryInArray(selectors, kCTFontFeatureSelectorNameKey as *const c_void, name, nameLength);
            if !s.is_null() {
                feature_selector = CFDictionaryGetValue(s, kCTFontFeatureSelectorIdentifierKey as *const c_void) as CFNumberRef;
            }
        }
        feature_selector
    }
}

unsafe fn createFeatureSettingDictionary(featureTypeIdentifier: CFNumberRef, featureSelectorIdentifier: CFNumberRef) -> CFDictionaryRef {
    unsafe {
        let settingKeys = [
            kCTFontFeatureTypeIdentifierKey as *const c_void,
            kCTFontFeatureSelectorIdentifierKey as *const c_void,
        ];
        let settingValues = [featureTypeIdentifier as *const c_void, featureSelectorIdentifier as *const c_void];

        CFDictionaryCreate(
            kCFAllocatorDefault,
            settingKeys.as_ptr(),
            settingValues.as_ptr(),
            2,
            &kCFTypeDictionaryKeyCallBacks as *const _ as *const c_void,
            &kCFTypeDictionaryValueCallBacks as *const _ as *const c_void,
        )
    }
}

#[unsafe(no_mangle)]
pub static mut kXeTeXEmboldenAttributeName: CFStringRef = ptr::null();

pub unsafe fn strstartswith(mut s: *const c_char, mut prefix: *const c_char) -> *const c_char {
    unsafe {
        while *prefix != 0 {
            if *s != *prefix {
                return ptr::null();
            }
            s = s.add(1);
            prefix = prefix.add(1);
        }
        s
    }
}

pub unsafe fn read_double(s: &mut *const c_char) -> f64 {
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

#[unsafe(export_name = "loadAATfont")]
pub unsafe extern "C" fn loadAATfont(mut descriptor: CTFontDescriptorRef, scaled_size: i32, mut cp1: *const c_char) -> *mut c_void {
    unsafe {
        let ctSize = TeXtoPSPoints(Fix2D(scaled_size));
        let font = CTFontCreateWithFontDescriptor(descriptor, ctSize, ptr::null());
        if font.is_null() {
            return ptr::null_mut();
        }

        let stringAttributes = CFDictionaryCreateMutable(
            ptr::null(),
            0,
            &kCFTypeDictionaryKeyCallBacks as *const _ as *const c_void,
            &kCFTypeDictionaryValueCallBacks as *const _ as *const c_void,
        );
        let attributes = CFDictionaryCreateMutable(
            ptr::null(),
            0,
            &kCFTypeDictionaryKeyCallBacks as *const _ as *const c_void,
            &kCFTypeDictionaryValueCallBacks as *const _ as *const c_void,
        );

        let mut tracking: f64 = 0.0;
        let mut extend: f32 = 1.0;
        let mut slant: f32 = 0.0;
        let mut embolden: f32 = 0.0;
        let mut letterspace: f32 = 0.0;
        let mut rgbValue: u32 = 0;

        if !cp1.is_null() {
            let features = CTFontCopyFeatures(font);
            let featureSettings = CFArrayCreateMutable(ptr::null(), 0, &kCFTypeArrayCallBacks as *const _ as *const c_void);

            while *cp1 != 0 {
                if *cp1 == b':' as c_char || *cp1 == b';' as c_char {
                    cp1 = cp1.add(1);
                }
                while *cp1 == b' ' as c_char || *cp1 == b'\t' as c_char {
                    cp1 = cp1.add(1);
                }
                if *cp1 == 0 {
                    break;
                }

                let mut cp2 = cp1;
                while *cp2 != 0 && *cp2 != b';' as c_char && *cp2 != b':' as c_char {
                    cp2 = cp2.add(1);
                }

                let mut cp3 = cp1;
                while cp3 < cp2 && *cp3 != b'=' as c_char {
                    cp3 = cp3.add(1);
                }
                if cp3 == cp2 {
                    // bad_option
                    if !strstartswith(cp1, b"vertical\0".as_ptr() as *const c_char).is_null() {
                        let mut cp_vert = cp2;
                        if *cp_vert == b';' as c_char || *cp_vert == b':' as c_char {
                            cp_vert = cp_vert.sub(1);
                        }
                        while *cp_vert == 0 || *cp_vert == b' ' as c_char || *cp_vert == b'\t' as c_char {
                            cp_vert = cp_vert.sub(1);
                        }
                        if *cp_vert != 0 {
                            cp_vert = cp_vert.add(1);
                        }
                        if cp_vert == cp1.add(8) {
                            let mut orientation = kCTFontOrientationVertical as c_int;
                            let orientationNumber = CFNumberCreate(ptr::null(), kCFNumberIntType, &mut orientation as *mut c_int as *const c_void);
                            CFDictionaryAddValue(
                                attributes,
                                kCTFontOrientationAttribute as *const c_void,
                                orientationNumber as *const c_void,
                            );
                            CFRelease(orientationNumber as CFTypeRef);
                            CFDictionaryAddValue(
                                stringAttributes,
                                kCTVerticalFormsAttributeName as *const c_void,
                                kCFBooleanTrue as *const c_void,
                            );
                            cp1 = cp2;
                            continue;
                        }
                    }
                    font_feature_warning(cp1.cast(), cp2.offset_from(cp1) as c_int, ptr::null(), 0);
                    cp1 = cp2;
                    continue;
                }

                let feature = findDictionaryInArray(features, kCTFontFeatureTypeNameKey as *const c_void, cp1, cp3.offset_from(cp1) as c_int);
                if !feature.is_null() {
                    let featLen = cp3.offset_from(cp1) as c_int;
                    let mut zeroInteger: c_int = 0;
                    let zero = CFNumberCreate(ptr::null(), kCFNumberIntType, &mut zeroInteger as *mut c_int as *const c_void);
                    cp3 = cp3.add(1);
                    while cp3 < cp2 {
                        while *cp3 == b' ' as c_char || *cp3 == b'\t' as c_char {
                            cp3 = cp3.add(1);
                        }
                        let mut disable = 0;
                        if *cp3 == b'!' as c_char {
                            disable = 1;
                            cp3 = cp3.add(1);
                        }
                        let mut cp4 = cp3;
                        while cp4 < cp2 && *cp4 != b',' as c_char {
                            cp4 = cp4.add(1);
                        }
                        let feature_selector = findSelectorByName(feature, cp3, cp4.offset_from(cp3) as c_int);
                        if !feature_selector.is_null() && CFNumberCompare(feature_selector, zero, ptr::null_mut()) >= 0 {
                            let featureType = CFDictionaryGetValue(feature, kCTFontFeatureTypeIdentifierKey as *const c_void) as CFNumberRef;
                            let featureSetting = createFeatureSettingDictionary(featureType, feature_selector);
                            CFArrayAppendValue(featureSettings, featureSetting as *const c_void);
                            CFRelease(featureSetting as CFTypeRef);
                        } else {
                            font_feature_warning(cp1.cast(), featLen, cp3.cast(), cp4.offset_from(cp3) as c_int);
                        }
                        cp3 = cp4.add(1);
                    }
                    CFRelease(zero as CFTypeRef);
                    cp1 = cp2;
                    continue;
                }

                let ret = readCommonFeatures(cp1, cp2, &mut extend, &mut slant, &mut embolden, &mut letterspace, &mut rgbValue);
                if ret == 1 {
                    cp1 = cp2;
                    continue;
                } else if ret == -1 {
                    font_feature_warning(cp1.cast(), cp2.offset_from(cp1) as c_int, ptr::null(), 0);
                    cp1 = cp2;
                    continue;
                }

                let tr_ptr = strstartswith(cp1, b"tracking\0".as_ptr() as *const c_char);
                if !tr_ptr.is_null() {
                    if *tr_ptr != b'=' as c_char {
                        font_feature_warning(cp1.cast(), cp2.offset_from(cp1) as c_int, ptr::null(), 0);
                        cp1 = cp2;
                        continue;
                    }
                    let mut tr_scan = tr_ptr.add(1);
                    tracking = read_double(&mut tr_scan);
                    let trackingNumber = CFNumberCreate(ptr::null(), kCFNumberDoubleType, &tracking as *const f64 as *const c_void);
                    CFDictionaryAddValue(stringAttributes, kCTKernAttributeName as *const c_void, trackingNumber as *const c_void);
                    CFRelease(trackingNumber as CFTypeRef);
                    cp1 = cp2;
                    continue;
                }

                // bad option fallback
                if !strstartswith(cp1, b"vertical\0".as_ptr() as *const c_char).is_null() {
                    let mut cp_vert = cp2;
                    if *cp_vert == b';' as c_char || *cp_vert == b':' as c_char {
                        cp_vert = cp_vert.sub(1);
                    }
                    while *cp_vert == 0 || *cp_vert == b' ' as c_char || *cp_vert == b'\t' as c_char {
                        cp_vert = cp_vert.sub(1);
                    }
                    if *cp_vert != 0 {
                        cp_vert = cp_vert.add(1);
                    }
                    if cp_vert == cp1.add(8) {
                        let mut orientation = kCTFontOrientationVertical as c_int;
                        let orientationNumber = CFNumberCreate(ptr::null(), kCFNumberIntType, &mut orientation as *mut c_int as *const c_void);
                        CFDictionaryAddValue(
                            attributes,
                            kCTFontOrientationAttribute as *const c_void,
                            orientationNumber as *const c_void,
                        );
                        CFRelease(orientationNumber as CFTypeRef);
                        CFDictionaryAddValue(
                            stringAttributes,
                            kCTVerticalFormsAttributeName as *const c_void,
                            kCFBooleanTrue as *const c_void,
                        );
                        cp1 = cp2;
                        continue;
                    }
                }

                font_feature_warning(cp1.cast(), cp2.offset_from(cp1) as c_int, ptr::null(), 0);
                cp1 = cp2;
            }

            if !features.is_null() {
                CFRelease(features as CFTypeRef);
            }

            if CFArrayGetCount(featureSettings) != 0 {
                CFDictionaryAddValue(
                    attributes,
                    kCTFontFeatureSettingsAttribute as *const c_void,
                    featureSettings as *const c_void,
                );
            }
            CFRelease(featureSettings as CFTypeRef);
        }

        if (loaded_font_flags & (FONT_FLAGS_COLORED) as i8) != 0 {
            let red = (((rgbValue & 0xFF000000) >> 24) as CGFloat) / 255.0;
            let green = (((rgbValue & 0x00FF0000) >> 16) as CGFloat) / 255.0;
            let blue = (((rgbValue & 0x0000FF00) >> 8) as CGFloat) / 255.0;
            let alpha = ((rgbValue & 0x000000FF) as CGFloat) / 255.0;
            let color = CGColorCreateGenericRGB(red, green, blue, alpha);
            CFDictionaryAddValue(stringAttributes, kCTForegroundColorAttributeName as *const c_void, color as *const c_void);
            CGColorRelease(color);
        }

        let mut matrix = CGAffineTransformIdentity;
        if extend != 1.0 || slant != 0.0 {
            matrix = CGAffineTransformMake(extend as CGFloat, 0.0, slant as CGFloat, 1.0, 0.0, 0.0);
        }

        if embolden != 0.0 {
            embolden = (embolden as f64 * Fix2D(scaled_size) / 100.0) as f32;
            let emboldenNumber = CFNumberCreate(ptr::null(), kCFNumberFloatType, &embolden as *const f32 as *const c_void);
            if kXeTeXEmboldenAttributeName.is_null() {
                kXeTeXEmboldenAttributeName = CFStringCreateWithCStringNoCopy(
                    kCFAllocatorDefault,
                    b"XeTeXEmbolden\0".as_ptr() as *const c_char,
                    kCFStringEncodingUTF8,
                    kCFAllocatorNull,
                );
            }
            CFDictionaryAddValue(
                stringAttributes,
                kXeTeXEmboldenAttributeName as *const c_void,
                emboldenNumber as *const c_void,
            );
            CFRelease(emboldenNumber as CFTypeRef);
        }

        if letterspace != 0.0 {
            loaded_font_letter_space = ((letterspace / 100.0) * (scaled_size as f32)) as i32;
        }

        let cascadeList = CFArrayCreateMutable(ptr::null(), 1, &kCFTypeArrayCallBacks as *const _ as *const c_void);
        let lr_name = CFStringCreateWithCStringNoCopy(
            kCFAllocatorDefault,
            b"LastResort\0".as_ptr() as *const c_char,
            kCFStringEncodingUTF8,
            kCFAllocatorNull,
        );
        let lastResort = CTFontDescriptorCreateWithNameAndSize(lr_name, 0.0);
        CFRelease(lr_name as CFTypeRef);
        CFArrayAppendValue(cascadeList, lastResort as *const c_void);
        CFRelease(lastResort as CFTypeRef);
        CFDictionaryAddValue(attributes, kCTFontCascadeListAttribute as *const c_void, cascadeList as *const c_void);
        CFRelease(cascadeList as CFTypeRef);

        descriptor = CTFontDescriptorCreateWithAttributes(attributes);
        CFRelease(attributes as CFTypeRef);
        let actualFont = CTFontCreateCopyWithAttributes(font, ctSize, &matrix, descriptor);
        CFRelease(font as CFTypeRef);
        CFDictionaryAddValue(stringAttributes, kCTFontAttributeName as *const c_void, actualFont as *const c_void);
        CFRelease(actualFont as CFTypeRef);

        native_font_type_flag = AAT_FONT_FLAG as i32;
        stringAttributes as *mut c_void
    }
}
