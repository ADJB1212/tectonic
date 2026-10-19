#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use super::xetex_xetexd::{font_area, font_layout_engine, font_size};

use libc::{c_int, c_uint, c_void, size_t};
use std::ptr;
use tectonic_bridge_harfbuzz::sys::hb_font_t;
unsafe extern "C" {
    fn ttxl_get_hb_font(engine: XeTeXLayoutEngine) -> *mut hb_font_t;
    fn getGlyphHeightDepth(
        engine: XeTeXLayoutEngine,
        glyph_id: u32,
        height: *mut f32,
        depth: *mut f32,
    );
    fn ttxl_font_get_point_size(font: XeTeXFont) -> f32;
    fn ttxl_font_points_to_units(font: XeTeXFont, points: f32) -> f32;
    fn ttxl_font_units_to_points(font: XeTeXFont, units: f32) -> f32;
}

use super::xetex_ext::*;
use super::xetex_xetexd::*;

pub type hb_position_t = i32;
pub type hb_codepoint_t = u32;
pub type hb_direction_t = c_uint;

pub const HB_DIRECTION_RTL: hb_direction_t = 5;
pub const HB_DIRECTION_TTB: hb_direction_t = 6;

pub type hb_ot_math_constant_t = c_int;

pub const HB_OT_MATH_CONSTANT_SCRIPT_PERCENT_SCALE_DOWN: hb_ot_math_constant_t = 0;
pub const HB_OT_MATH_CONSTANT_SCRIPT_SCRIPT_PERCENT_SCALE_DOWN: hb_ot_math_constant_t = 1;
pub const HB_OT_MATH_CONSTANT_DELIMITED_SUB_FORMULA_MIN_HEIGHT: hb_ot_math_constant_t = 2;
pub const HB_OT_MATH_CONSTANT_DISPLAY_OPERATOR_MIN_HEIGHT: hb_ot_math_constant_t = 3;
pub const HB_OT_MATH_CONSTANT_MATH_LEADING: hb_ot_math_constant_t = 4;
pub const HB_OT_MATH_CONSTANT_AXIS_HEIGHT: hb_ot_math_constant_t = 5;
pub const HB_OT_MATH_CONSTANT_ACCENT_BASE_HEIGHT: hb_ot_math_constant_t = 6;
pub const HB_OT_MATH_CONSTANT_FLATTENED_ACCENT_BASE_HEIGHT: hb_ot_math_constant_t = 7;
pub const HB_OT_MATH_CONSTANT_SUBSCRIPT_SHIFT_DOWN: hb_ot_math_constant_t = 8;
pub const HB_OT_MATH_CONSTANT_SUBSCRIPT_TOP_MAX: hb_ot_math_constant_t = 9;
pub const HB_OT_MATH_CONSTANT_SUBSCRIPT_BASELINE_DROP_MIN: hb_ot_math_constant_t = 10;
pub const HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP: hb_ot_math_constant_t = 11;
pub const HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP_CRAMPED: hb_ot_math_constant_t = 12;
pub const HB_OT_MATH_CONSTANT_SUPERSCRIPT_BOTTOM_MIN: hb_ot_math_constant_t = 13;
pub const HB_OT_MATH_CONSTANT_SUPERSCRIPT_BASELINE_DROP_MAX: hb_ot_math_constant_t = 14;
pub const HB_OT_MATH_CONSTANT_SUB_SUPERSCRIPT_GAP_MIN: hb_ot_math_constant_t = 15;
pub const HB_OT_MATH_CONSTANT_SUPERSCRIPT_BOTTOM_MAX_WITH_SUBSCRIPT: hb_ot_math_constant_t = 16;
pub const HB_OT_MATH_CONSTANT_SPACE_AFTER_SCRIPT: hb_ot_math_constant_t = 17;
pub const HB_OT_MATH_CONSTANT_UPPER_LIMIT_GAP_MIN: hb_ot_math_constant_t = 18;
pub const HB_OT_MATH_CONSTANT_UPPER_LIMIT_BASELINE_RISE_MIN: hb_ot_math_constant_t = 19;
pub const HB_OT_MATH_CONSTANT_LOWER_LIMIT_GAP_MIN: hb_ot_math_constant_t = 20;
pub const HB_OT_MATH_CONSTANT_LOWER_LIMIT_BASELINE_DROP_MIN: hb_ot_math_constant_t = 21;
pub const HB_OT_MATH_CONSTANT_STACK_TOP_SHIFT_UP: hb_ot_math_constant_t = 22;
pub const HB_OT_MATH_CONSTANT_STACK_TOP_DISPLAY_STYLE_SHIFT_UP: hb_ot_math_constant_t = 23;
pub const HB_OT_MATH_CONSTANT_STACK_BOTTOM_SHIFT_DOWN: hb_ot_math_constant_t = 24;
pub const HB_OT_MATH_CONSTANT_STACK_BOTTOM_DISPLAY_STYLE_SHIFT_DOWN: hb_ot_math_constant_t = 25;
pub const HB_OT_MATH_CONSTANT_STACK_GAP_MIN: hb_ot_math_constant_t = 26;
pub const HB_OT_MATH_CONSTANT_STACK_DISPLAY_STYLE_GAP_MIN: hb_ot_math_constant_t = 27;
pub const HB_OT_MATH_CONSTANT_STRETCH_STACK_TOP_SHIFT_UP: hb_ot_math_constant_t = 28;
pub const HB_OT_MATH_CONSTANT_STRETCH_STACK_BOTTOM_SHIFT_DOWN: hb_ot_math_constant_t = 29;
pub const HB_OT_MATH_CONSTANT_STRETCH_STACK_GAP_ABOVE_MIN: hb_ot_math_constant_t = 30;
pub const HB_OT_MATH_CONSTANT_STRETCH_STACK_GAP_BELOW_MIN: hb_ot_math_constant_t = 31;
pub const HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_SHIFT_UP: hb_ot_math_constant_t = 32;
pub const HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_DISPLAY_STYLE_SHIFT_UP: hb_ot_math_constant_t = 33;
pub const HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_SHIFT_DOWN: hb_ot_math_constant_t = 34;
pub const HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_DISPLAY_STYLE_SHIFT_DOWN: hb_ot_math_constant_t =
    35;
pub const HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_GAP_MIN: hb_ot_math_constant_t = 36;
pub const HB_OT_MATH_CONSTANT_FRACTION_NUM_DISPLAY_STYLE_GAP_MIN: hb_ot_math_constant_t = 37;
pub const HB_OT_MATH_CONSTANT_FRACTION_RULE_THICKNESS: hb_ot_math_constant_t = 38;
pub const HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_GAP_MIN: hb_ot_math_constant_t = 39;
pub const HB_OT_MATH_CONSTANT_FRACTION_DENOM_DISPLAY_STYLE_GAP_MIN: hb_ot_math_constant_t = 40;
pub const HB_OT_MATH_CONSTANT_SKEWED_FRACTION_HORIZONTAL_GAP: hb_ot_math_constant_t = 41;
pub const HB_OT_MATH_CONSTANT_SKEWED_FRACTION_VERTICAL_GAP: hb_ot_math_constant_t = 42;
pub const HB_OT_MATH_CONSTANT_OVERBAR_VERTICAL_GAP: hb_ot_math_constant_t = 43;
pub const HB_OT_MATH_CONSTANT_OVERBAR_RULE_THICKNESS: hb_ot_math_constant_t = 44;
pub const HB_OT_MATH_CONSTANT_OVERBAR_EXTRA_ASCENDER: hb_ot_math_constant_t = 45;
pub const HB_OT_MATH_CONSTANT_UNDERBAR_VERTICAL_GAP: hb_ot_math_constant_t = 46;
pub const HB_OT_MATH_CONSTANT_UNDERBAR_RULE_THICKNESS: hb_ot_math_constant_t = 47;
pub const HB_OT_MATH_CONSTANT_UNDERBAR_EXTRA_DESCENDER: hb_ot_math_constant_t = 48;
pub const HB_OT_MATH_CONSTANT_RADICAL_VERTICAL_GAP: hb_ot_math_constant_t = 49;
pub const HB_OT_MATH_CONSTANT_RADICAL_DISPLAY_STYLE_VERTICAL_GAP: hb_ot_math_constant_t = 50;
pub const HB_OT_MATH_CONSTANT_RADICAL_RULE_THICKNESS: hb_ot_math_constant_t = 51;
pub const HB_OT_MATH_CONSTANT_RADICAL_EXTRA_ASCENDER: hb_ot_math_constant_t = 52;
pub const HB_OT_MATH_CONSTANT_RADICAL_KERN_BEFORE_DEGREE: hb_ot_math_constant_t = 53;
pub const HB_OT_MATH_CONSTANT_RADICAL_KERN_AFTER_DEGREE: hb_ot_math_constant_t = 54;
pub const HB_OT_MATH_CONSTANT_RADICAL_DEGREE_BOTTOM_RAISE_PERCENT: hb_ot_math_constant_t = 55;

pub type hb_ot_math_kern_t = c_uint;
pub const HB_OT_MATH_KERN_TOP_RIGHT: hb_ot_math_kern_t = 0;
pub const HB_OT_MATH_KERN_TOP_LEFT: hb_ot_math_kern_t = 1;
pub const HB_OT_MATH_KERN_BOTTOM_RIGHT: hb_ot_math_kern_t = 2;
pub const HB_OT_MATH_KERN_BOTTOM_LEFT: hb_ot_math_kern_t = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct hb_ot_math_glyph_variant_t {
    pub glyph: hb_codepoint_t,
    pub advance: hb_position_t,
}

pub type hb_ot_math_glyph_part_flags_t = c_uint;
pub const HB_MATH_GLYPH_PART_FLAG_EXTENDER: hb_ot_math_glyph_part_flags_t = 0x00000001;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct hb_ot_math_glyph_part_t {
    pub glyph: hb_codepoint_t,
    pub start_connector_length: hb_position_t,
    pub end_connector_length: hb_position_t,
    pub full_advance: hb_position_t,
    pub flags: hb_ot_math_glyph_part_flags_t,
}

#[repr(C)]
pub struct GlyphAssembly {
    pub count: c_uint,
    pub parts: *mut hb_ot_math_glyph_part_t,
}

unsafe extern "C" {
    fn hb_ot_math_get_constant(
        font: *mut hb_font_t,
        constant: hb_ot_math_constant_t,
    ) -> hb_position_t;

    fn hb_ot_math_get_glyph_variants(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
        direction: hb_direction_t,
        start_offset: c_uint,
        variants_count: *mut c_uint,
        variants: *mut hb_ot_math_glyph_variant_t,
    ) -> c_uint;

    fn hb_ot_math_get_glyph_assembly(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
        direction: hb_direction_t,
        start_offset: c_uint,
        parts_count: *mut c_uint,
        parts: *mut hb_ot_math_glyph_part_t,
        italics_correction: *mut hb_position_t,
    ) -> c_uint;

    fn hb_ot_math_get_glyph_italics_correction(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
    ) -> hb_position_t;

    fn hb_ot_math_get_glyph_top_accent_attachment(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
    ) -> hb_position_t;

    fn hb_ot_math_get_min_connector_overlap(
        font: *mut hb_font_t,
        direction: hb_direction_t,
    ) -> hb_position_t;

    fn hb_ot_math_get_glyph_kerning(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
        kern: hb_ot_math_kern_t,
        correction_height: hb_position_t,
    ) -> hb_position_t;
}

#[inline]
pub fn D2Fix(d: f64) -> i32 {
    (d * 65536.0 + 0.5) as i32
}

#[inline]
pub fn Fix2D(f: i32) -> f64 {
    f as f64 / 65536.0
}

#[unsafe(export_name = "get_ot_math_constant")]
pub unsafe extern "C" fn get_ot_math_constant(f: c_int, n: c_int) -> c_int {
    unsafe {
        let constant: hb_ot_math_constant_t = n;
        let mut rval: hb_position_t = 0;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            let font = getFont(engine);
            let hb_font = ttxl_get_hb_font(engine);
            rval = hb_ot_math_get_constant(hb_font, constant);

            match constant {
                HB_OT_MATH_CONSTANT_SCRIPT_PERCENT_SCALE_DOWN
                | HB_OT_MATH_CONSTANT_SCRIPT_SCRIPT_PERCENT_SCALE_DOWN
                | HB_OT_MATH_CONSTANT_RADICAL_DEGREE_BOTTOM_RAISE_PERCENT => {}
                _ => {
                    rval = D2Fix(ttxl_font_units_to_points(font, rval as f32) as f64);
                }
            }
        }

        rval
    }
}

const UNKNOWN: hb_ot_math_constant_t = -1;

const MATH_X_HEIGHT: c_int = 5;
const MATH_QUAD: c_int = 6;
const NUM1: c_int = 8;
const NUM2: c_int = 9;
const NUM3: c_int = 10;
const DENOM1: c_int = 11;
const DENOM2: c_int = 12;
const SUP1: c_int = 13;
const SUP2: c_int = 14;
const SUP3: c_int = 15;
const SUB1: c_int = 16;
const SUB2: c_int = 17;
const SUP_DROP: c_int = 18;
const SUB_DROP: c_int = 19;
const DELIM1: c_int = 20;
const DELIM2: c_int = 21;
const AXIS_HEIGHT: c_int = 22;

const TEX_SYM_TO_OT_MAP: &[hb_ot_math_constant_t] = &[
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    HB_OT_MATH_CONSTANT_ACCENT_BASE_HEIGHT,
    UNKNOWN,
    UNKNOWN,
    HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_DISPLAY_STYLE_SHIFT_UP,
    HB_OT_MATH_CONSTANT_FRACTION_NUMERATOR_SHIFT_UP,
    HB_OT_MATH_CONSTANT_STACK_TOP_SHIFT_UP,
    HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_DISPLAY_STYLE_SHIFT_DOWN,
    HB_OT_MATH_CONSTANT_FRACTION_DENOMINATOR_SHIFT_DOWN,
    HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP,
    HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP,
    HB_OT_MATH_CONSTANT_SUPERSCRIPT_SHIFT_UP_CRAMPED,
    HB_OT_MATH_CONSTANT_SUBSCRIPT_SHIFT_DOWN,
    HB_OT_MATH_CONSTANT_SUBSCRIPT_SHIFT_DOWN,
    HB_OT_MATH_CONSTANT_SUPERSCRIPT_BASELINE_DROP_MAX,
    HB_OT_MATH_CONSTANT_SUBSCRIPT_BASELINE_DROP_MIN,
    HB_OT_MATH_CONSTANT_DELIMITED_SUB_FORMULA_MIN_HEIGHT,
    UNKNOWN,
    HB_OT_MATH_CONSTANT_AXIS_HEIGHT,
];

#[unsafe(export_name = "get_native_mathsy_param")]
pub unsafe extern "C" fn get_native_mathsy_param(f: c_int, n: c_int) -> c_int {
    unsafe {
        let mut rval = 0;

        if n == MATH_QUAD {
            rval = *font_size.add(f as usize);
        } else if n == DELIM2 {
            let limit = get_native_mathsy_param(f, DELIM1);
            let val = (1.5 * ((*font_size.add(f as usize)) as f64)) as c_int;
            rval = std::cmp::min(val, limit);
        } else if (n as usize) < TEX_SYM_TO_OT_MAP.len() {
            let ot_index = TEX_SYM_TO_OT_MAP[n as usize];
            if ot_index != UNKNOWN {
                rval = get_ot_math_constant(f, ot_index);
            }
        }

        rval
    }
}

const DEFAULT_RULE_THICKNESS: c_int = 8;
const BIG_OP_SPACING1: c_int = 9;
const BIG_OP_SPACING2: c_int = 10;
const BIG_OP_SPACING3: c_int = 11;
const BIG_OP_SPACING4: c_int = 12;
const BIG_OP_SPACING5: c_int = 13;

const TEX_EXT_TO_OT_MAP: &[hb_ot_math_constant_t] = &[
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    UNKNOWN,
    HB_OT_MATH_CONSTANT_ACCENT_BASE_HEIGHT,
    UNKNOWN,
    UNKNOWN,
    HB_OT_MATH_CONSTANT_FRACTION_RULE_THICKNESS,
    HB_OT_MATH_CONSTANT_UPPER_LIMIT_GAP_MIN,
    HB_OT_MATH_CONSTANT_LOWER_LIMIT_GAP_MIN,
    HB_OT_MATH_CONSTANT_UPPER_LIMIT_BASELINE_RISE_MIN,
    HB_OT_MATH_CONSTANT_LOWER_LIMIT_BASELINE_DROP_MIN,
    HB_OT_MATH_CONSTANT_STACK_GAP_MIN,
];

#[unsafe(export_name = "get_native_mathex_param")]
pub unsafe extern "C" fn get_native_mathex_param(f: c_int, n: c_int) -> c_int {
    unsafe {
        let mut rval = 0;

        if n == MATH_QUAD {
            rval = *font_size.add(f as usize);
        } else if (n as usize) < TEX_EXT_TO_OT_MAP.len() {
            let ot_index = TEX_EXT_TO_OT_MAP[n as usize];
            if ot_index != UNKNOWN {
                rval = get_ot_math_constant(f, ot_index);
            }
        }

        rval
    }
}

#[unsafe(export_name = "get_ot_math_variant")]
pub unsafe extern "C" fn get_ot_math_variant(
    f: c_int,
    g: c_int,
    v: c_int,
    adv: *mut i32,
    horiz: c_int,
) -> c_int {
    unsafe {
        let mut rval: hb_codepoint_t = g as hb_codepoint_t;
        *adv = -1;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            let font = getFont(engine);
            let hb_font = ttxl_get_hb_font(engine);
            let mut variant: [hb_ot_math_glyph_variant_t; 1] = [hb_ot_math_glyph_variant_t {
                glyph: 0,
                advance: 0,
            }];
            let mut count: c_uint = 1;
            let direction = if horiz != 0 {
                HB_DIRECTION_RTL
            } else {
                HB_DIRECTION_TTB
            };
            hb_ot_math_get_glyph_variants(
                hb_font,
                g as hb_codepoint_t,
                direction,
                v as c_uint,
                &mut count,
                variant.as_mut_ptr(),
            );

            if count > 0 {
                rval = variant[0].glyph;
                *adv = D2Fix(ttxl_font_units_to_points(font, variant[0].advance as f32) as f64);
            }
        }

        rval as c_int
    }
}

#[unsafe(export_name = "get_ot_assembly_ptr")]
pub unsafe extern "C" fn get_ot_assembly_ptr(f: c_int, g: c_int, horiz: c_int) -> *mut c_void {
    unsafe {
        let mut rval: *mut c_void = ptr::null_mut();

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            let hb_font = ttxl_get_hb_font(engine);
            let direction = if horiz != 0 {
                HB_DIRECTION_RTL
            } else {
                HB_DIRECTION_TTB
            };

            let count = hb_ot_math_get_glyph_assembly(
                hb_font,
                g as hb_codepoint_t,
                direction,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            if count > 0 {
                let a = xmalloc(std::mem::size_of::<GlyphAssembly>()) as *mut GlyphAssembly;
                (*a).count = count;
                (*a).parts =
                    xmalloc((count as usize) * std::mem::size_of::<hb_ot_math_glyph_part_t>())
                        as *mut hb_ot_math_glyph_part_t;
                hb_ot_math_get_glyph_assembly(
                    hb_font,
                    g as hb_codepoint_t,
                    direction,
                    0,
                    &mut (*a).count,
                    (*a).parts,
                    ptr::null_mut(),
                );
                rval = a as *mut c_void;
            }
        }

        rval
    }
}

#[unsafe(export_name = "free_ot_assembly")]
pub unsafe extern "C" fn free_ot_assembly(a: *mut GlyphAssembly) {
    unsafe {
        if a.is_null() {
            return;
        }
        if !(*a).parts.is_null() {
            libc::free((*a).parts as *mut c_void);
        }
        libc::free(a as *mut c_void);
    }
}

#[unsafe(export_name = "get_ot_math_ital_corr")]
pub unsafe extern "C" fn get_ot_math_ital_corr(f: c_int, g: c_int) -> c_int {
    unsafe {
        let mut rval: hb_position_t = 0;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            let font = getFont(engine);
            let hb_font = ttxl_get_hb_font(engine);
            rval = hb_ot_math_get_glyph_italics_correction(hb_font, g as hb_codepoint_t);
            rval = D2Fix(ttxl_font_units_to_points(font, rval as f32) as f64);
        }

        rval
    }
}

#[unsafe(export_name = "get_ot_math_accent_pos")]
pub unsafe extern "C" fn get_ot_math_accent_pos(f: c_int, g: c_int) -> c_int {
    unsafe {
        let mut rval: hb_position_t = 0x7fffffff;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            let font = getFont(engine);
            let hb_font = ttxl_get_hb_font(engine);
            rval = hb_ot_math_get_glyph_top_accent_attachment(hb_font, g as hb_codepoint_t);
            rval = D2Fix(ttxl_font_units_to_points(font, rval as f32) as f64);
        }

        rval
    }
}

#[unsafe(export_name = "ot_min_connector_overlap")]
pub unsafe extern "C" fn ot_min_connector_overlap(f: c_int) -> c_int {
    unsafe {
        let mut rval: hb_position_t = 0;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            let font = getFont(engine);
            let hb_font = ttxl_get_hb_font(engine);
            rval = hb_ot_math_get_min_connector_overlap(hb_font, HB_DIRECTION_RTL);
            rval = D2Fix(ttxl_font_units_to_points(font, rval as f32) as f64);
        }

        rval
    }
}

unsafe fn getMathKernAt(f: c_int, g: c_int, height: c_int, side: hb_ot_math_kern_t) -> c_int {
    unsafe {
        let mut rval: hb_position_t = 0;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            let hb_font = ttxl_get_hb_font(engine);
            rval = hb_ot_math_get_glyph_kerning(hb_font, g as hb_codepoint_t, side, height);
        }

        rval
    }
}

unsafe fn glyph_height(f: c_int, g: c_int) -> f32 {
    unsafe {
        let mut rval: f32 = 0.0;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            getGlyphHeightDepth(engine, g as u32, &mut rval, ptr::null_mut());
        }

        rval
    }
}

unsafe fn glyph_depth(f: c_int, g: c_int) -> f32 {
    unsafe {
        let mut rval: f32 = 0.0;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            getGlyphHeightDepth(engine, g as u32, ptr::null_mut(), &mut rval);
        }

        rval
    }
}

const SUP_CMD: c_int = 0;
const SUB_CMD: c_int = 1;

#[unsafe(export_name = "get_ot_math_kern")]
pub unsafe extern "C" fn get_ot_math_kern(
    f: c_int,
    g: c_int,
    sf: c_int,
    sg: c_int,
    cmd: c_int,
    shift_scaled: c_int,
) -> c_int {
    unsafe {
        let mut rval: c_int = 0;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32
            && (*font_area.add(sf as usize)) == OTGR_FONT_FLAG as i32
        {
            let engine_f = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            let font = getFont(engine_f);

            let engine_sf = (*font_layout_engine.add(sf as usize)) as XeTeXLayoutEngine;
            let sfont = getFont(engine_sf);

            let g_height = ttxl_font_points_to_units(font, glyph_height(f, g)) as c_int;
            let g_depth = ttxl_font_points_to_units(font, glyph_depth(f, g)) as c_int;

            let sg_height = ttxl_font_points_to_units(sfont, glyph_height(sf, sg)) as c_int;
            let sg_depth = ttxl_font_points_to_units(sfont, glyph_depth(sf, sg)) as c_int;

            let shift = ttxl_font_points_to_units(font, Fix2D(shift_scaled) as f32) as c_int;

            let f_size = ttxl_font_get_point_size(font);
            let sf_size = ttxl_font_get_point_size(sfont);

            assert!(f_size != 0.0);

            let scale_factor = sf_size / f_size;

            if cmd == SUP_CMD {
                let kern = getMathKernAt(
                    f,
                    g,
                    shift - (scale_factor * (sg_depth as f32)) as c_int,
                    HB_OT_MATH_KERN_TOP_RIGHT,
                );

                let skern = getMathKernAt(sf, sg, -sg_depth, HB_OT_MATH_KERN_BOTTOM_LEFT);
                let top_kern = kern + (scale_factor * (skern as f32)) as c_int;

                let kern2 = getMathKernAt(f, g, g_height, HB_OT_MATH_KERN_TOP_RIGHT);
                let skern2 = getMathKernAt(
                    sf,
                    sg,
                    ((g_height - shift) as f32 / scale_factor) as c_int,
                    HB_OT_MATH_KERN_BOTTOM_LEFT,
                );
                let bot_kern = kern2 + (scale_factor * (skern2 as f32)) as c_int;

                rval = if top_kern > bot_kern {
                    top_kern
                } else {
                    bot_kern
                };
            } else if cmd == SUB_CMD {
                let kern = getMathKernAt(
                    f,
                    g,
                    (scale_factor * (sg_height as f32)) as c_int - shift,
                    HB_OT_MATH_KERN_BOTTOM_RIGHT,
                );

                let skern = getMathKernAt(sf, sg, sg_height, HB_OT_MATH_KERN_TOP_LEFT);
                let top_kern = kern + (scale_factor * (skern as f32)) as c_int;

                let kern2 = getMathKernAt(f, g, -g_depth, HB_OT_MATH_KERN_BOTTOM_RIGHT);
                let skern2 = getMathKernAt(
                    sf,
                    sg,
                    ((shift - g_depth) as f32 / scale_factor) as c_int,
                    HB_OT_MATH_KERN_TOP_LEFT,
                );
                let bot_kern = kern2 + (scale_factor * (skern2 as f32)) as c_int;

                rval = if top_kern > bot_kern {
                    top_kern
                } else {
                    bot_kern
                };
            } else {
                panic!("unreachable math cmd");
            }

            rval = D2Fix(ttxl_font_units_to_points(font, rval as f32) as f64);
        }

        rval
    }
}

#[unsafe(export_name = "ot_part_count")]
pub unsafe extern "C" fn ot_part_count(a: *const GlyphAssembly) -> c_int {
    unsafe { (*a).count as c_int }
}

#[unsafe(export_name = "ot_part_glyph")]
pub unsafe extern "C" fn ot_part_glyph(a: *const GlyphAssembly, i: c_int) -> c_int {
    unsafe { (*(*a).parts.add(i as usize)).glyph as c_int }
}

#[unsafe(export_name = "ot_part_is_extender")]
pub unsafe extern "C" fn ot_part_is_extender(a: *const GlyphAssembly, i: c_int) -> bool {
    unsafe { ((*(*a).parts.add(i as usize)).flags & HB_MATH_GLYPH_PART_FLAG_EXTENDER) != 0 }
}

#[unsafe(export_name = "ot_part_start_connector")]
pub unsafe extern "C" fn ot_part_start_connector(
    f: c_int,
    a: *const GlyphAssembly,
    i: c_int,
) -> c_int {
    unsafe {
        let mut rval = 0;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            let font = getFont(engine);
            rval = D2Fix(ttxl_font_units_to_points(
                font,
                (*(*a).parts.add(i as usize)).start_connector_length as f32,
            ) as f64);
        }

        rval
    }
}

#[unsafe(export_name = "ot_part_end_connector")]
pub unsafe extern "C" fn ot_part_end_connector(
    f: c_int,
    a: *const GlyphAssembly,
    i: c_int,
) -> c_int {
    unsafe {
        let mut rval = 0;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            let font = getFont(engine);
            rval = D2Fix(ttxl_font_units_to_points(
                font,
                (*(*a).parts.add(i as usize)).end_connector_length as f32,
            ) as f64);
        }

        rval
    }
}

#[unsafe(export_name = "ot_part_full_advance")]
pub unsafe extern "C" fn ot_part_full_advance(
    f: c_int,
    a: *const GlyphAssembly,
    i: c_int,
) -> c_int {
    unsafe {
        let mut rval = 0;

        if (*font_area.add(f as usize)) == OTGR_FONT_FLAG as i32 {
            let engine = (*font_layout_engine.add(f as usize)) as XeTeXLayoutEngine;
            let font = getFont(engine);
            rval = D2Fix(ttxl_font_units_to_points(
                font,
                (*(*a).parts.add(i as usize)).full_advance as f32,
            ) as f64);
        }

        rval
    }
}
