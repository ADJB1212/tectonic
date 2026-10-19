#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use core::ffi::c_char;

pub type scaled_t = i32;
pub type small_number = i16;
pub type internal_font_number = i32;
pub type group_code = u8;
pub type str_number = i32;

#[cfg(target_endian = "big")]
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct b32x2 {
    pub s1: i32,
    pub s0: i32,
}

#[cfg(not(target_endian = "big"))]
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct b32x2 {
    pub s0: i32,
    pub s1: i32,
}

#[cfg(target_endian = "big")]
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct b16x4 {
    pub s3: u16,
    pub s2: u16,
    pub s1: u16,
    pub s0: u16,
}

#[cfg(not(target_endian = "big"))]
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct b16x4 {
    pub s0: u16,
    pub s1: u16,
    pub s2: u16,
    pub s3: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: f64,
    pub ptr: *mut core::ffi::c_void,
}

impl Default for memory_word {
    fn default() -> Self {
        memory_word {
            b32: b32x2::default(),
        }
    }
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct list_state_record {
    pub mode: i16,
    pub head: i32,
    pub tail: i32,
    pub eTeX_aux: i32,
    pub prev_graf: i32,
    pub mode_line: i32,
    pub aux: memory_word,
}

// Constants
pub const MIN_HALFWORD: i32 = -0x0FFFFFFF;
pub const MAX_HALFWORD: i32 = 0x3FFFFFFF;
pub const TEX_NULL: i32 = MIN_HALFWORD;
pub const TEX_INFINITY: i32 = 0x7FFFFFFF;
pub const DEFAULT_CODE: i32 = 0x40000000;

pub const NUMBER_MATH_FAMILIES: i32 = 256;
pub const MEM_TOP: i32 = 4999999;
pub const TEMP_HEAD: i32 = MEM_TOP - 3;
pub const GARBAGE: i32 = MEM_TOP - 12;
pub const ADJUST_HEAD: i32 = MEM_TOP - 5;
pub const PRE_ADJUST_HEAD: i32 = MEM_TOP - 14;

pub const GLUE_BASE: usize = 2254340;
pub const LOCAL_BASE: usize = 2254871;
pub const CUR_FONT_LOC: usize = 2255400;
pub const MATH_FONT_BASE: usize = 2255401;
pub const INT_BASE: usize = 7826729;
pub const DEL_CODE_BASE: usize = 7827068;
pub const DIMEN_BASE: usize = 8941180;

pub const GLUE_PAR__above_display_skip: i16 = 3;
pub const GLUE_PAR__below_display_skip: i16 = 4;
pub const GLUE_PAR__above_display_short_skip: i16 = 5;
pub const GLUE_PAR__below_display_short_skip: i16 = 6;
pub const GLUE_PAR__left_skip: i16 = 7;
pub const GLUE_PAR__right_skip: i16 = 8;
pub const GLUE_PAR__thin_mu_skip: i32 = super::xetex_format::GLUE_PAR__thin_mu_skip as i32;
pub const GLUE_PAR__med_mu_skip: i32 = super::xetex_format::GLUE_PAR__med_mu_skip as i32;
pub const GLUE_PAR__thick_mu_skip: i32 = super::xetex_format::GLUE_PAR__thick_mu_skip as i32;

pub const INT_PAR__bin_op_penalty: usize = 9;
pub const INT_PAR__rel_penalty: usize = 10;
pub const INT_PAR__pre_display_penalty: usize = 11;
pub const INT_PAR__post_display_penalty: usize = 12;
pub const INT_PAR__delimiter_factor: usize = 18;
pub const INT_PAR__hang_after: usize = super::xetex_format::INT_PAR__hang_after as usize;
pub const INT_PAR__cur_fam: usize = super::xetex_format::INT_PAR__cur_fam as usize;
pub const INT_PAR__language: usize = super::xetex_format::INT_PAR__language as usize;
pub const INT_PAR__left_hyphen_min: usize = super::xetex_format::INT_PAR__left_hyphen_min as usize;
pub const INT_PAR__right_hyphen_min: usize =
    super::xetex_format::INT_PAR__right_hyphen_min as usize;
pub const INT_PAR__texxet: usize = super::xetex_format::INT_PAR__texxet as usize;
pub const INT_PAR__pre_display_direction: usize =
    super::xetex_format::INT_PAR__pre_display_direction as usize;

pub const DIMEN_PAR__null_delimiter_space: usize =
    super::xetex_format::DIMEN_PAR__null_delimiter_space as usize;
pub const DIMEN_PAR__math_surround: usize = super::xetex_format::DIMEN_PAR__math_surround as usize;
pub const DIMEN_PAR__pre_display_size: usize =
    super::xetex_format::DIMEN_PAR__pre_display_size as usize;
pub const DIMEN_PAR__display_width: usize = super::xetex_format::DIMEN_PAR__display_width as usize;
pub const DIMEN_PAR__display_indent: usize =
    super::xetex_format::DIMEN_PAR__display_indent as usize;
pub const DIMEN_PAR__script_space: usize = super::xetex_format::DIMEN_PAR__script_space as usize;
pub const DIMEN_PAR__hang_indent: usize = super::xetex_format::DIMEN_PAR__hang_indent as usize;
pub const DIMEN_PAR__hsize: usize = super::xetex_format::DIMEN_PAR__hsize as usize;
pub const DIMEN_PAR__delimiter_shortfall: usize =
    super::xetex_format::DIMEN_PAR__delimiter_shortfall as usize;

pub const LOCAL__par_shape: usize = 0;
pub const LOCAL__every_math: usize = super::xetex_format::LOCAL__every_math as usize;
pub const LOCAL__every_display: usize = super::xetex_format::LOCAL__every_display as usize;

pub const NORMAL: i16 = 0;
pub const FIL: usize = 1;
pub const FILL: usize = 2;
pub const FILLL: usize = 3;

pub const MATH_SHIFT_GROUP: u8 = 15;
pub const MATH_LEFT_GROUP: u8 = 16;
pub const MATH_CHOICE_GROUP: u8 = 13;

pub const EVERY_MATH_TEXT: i32 = 9;
pub const EVERY_DISPLAY_TEXT: i32 = 10;

pub const HLIST_NODE: i16 = 0;
pub const VLIST_NODE: i16 = 1;
pub const RULE_NODE: i16 = 2;
pub const INS_NODE: i16 = 3;
pub const MARK_NODE: i16 = 4;
pub const ADJUST_NODE: i16 = 5;
pub const LIGATURE_NODE: i16 = 6;
pub const DISC_NODE: i16 = 7;
pub const WHATSIT_NODE: i16 = 8;
pub const MATH_NODE: i16 = 9;
pub const GLUE_NODE: i16 = 10;
pub const KERN_NODE: i16 = 11;
pub const PENALTY_NODE: i16 = 12;
pub const STYLE_NODE: i16 = 14;
pub const CHOICE_NODE: i16 = 15;
pub const MARGIN_KERN_NODE: i16 = 40;

pub const ORD_NOAD: i16 = 16;
pub const OP_NOAD: i16 = 17;
pub const BIN_NOAD: i16 = 18;
pub const REL_NOAD: i16 = 19;
pub const OPEN_NOAD: i16 = 20;
pub const CLOSE_NOAD: i16 = 21;
pub const PUNCT_NOAD: i16 = 22;
pub const INNER_NOAD: i16 = 23;
pub const RADICAL_NOAD: i16 = 24;
pub const FRACTION_NOAD: i16 = 25;
pub const UNDER_NOAD: i16 = 26;
pub const OVER_NOAD: i16 = 27;
pub const ACCENT_NOAD: i16 = 28;
pub const VCENTER_NOAD: i16 = 29;
pub const LEFT_NOAD: i16 = 30;
pub const RIGHT_NOAD: i16 = 31;

pub const NOAD_SIZE: i32 = 4;
pub const ACCENT_NOAD_SIZE: i32 = 5;
pub const RADICAL_NOAD_SIZE: i32 = 5;
pub const FRACTION_NOAD_SIZE: i32 = 6;
pub const STYLE_NODE_SIZE: i32 = 3;
pub const MEDIUM_NODE_SIZE: i32 = 3;
pub const GLUE_SPEC_SIZE: i32 = 4;
pub const GLYPH_NODE_SIZE: i32 = 5;
pub const BOX_NODE_SIZE: i32 = 8;

pub const RELAX: u8 = 0;
pub const MATH_SHIFT: u8 = 3;
pub const SUP_MARK: u8 = 7;
pub const SPACER: u8 = 10;
pub const LETTER: u8 = 11;
pub const OTHER_CHAR: u8 = 12;
pub const DELIM_NUM: u8 = 15;
pub const ACCENT: u8 = 45;

pub const EXACTLY: i16 = 0;
pub const ADDITIONAL: i16 = 1;

pub const LEFT_TO_RIGHT: i16 = 0;
pub const RIGHT_TO_LEFT: i16 = 1;

pub const L_CODE: i16 = 4;
pub const R_CODE: i32 = 8;
pub const BEFORE: i16 = 0;
pub const AFTER: i16 = 1;
pub const BEGIN_M_CODE: i16 = 2;
pub const END_M_CODE: i16 = 3;
pub const BEGIN_L_CODE: i16 = 6;
pub const END_L_CODE: i16 = 7;

pub const STRETCHING: u16 = 1;
pub const SHRINKING: u16 = 2;
pub const A_LEADERS: u16 = 100;

pub const NATIVE_WORD_NODE: u16 = 40;
pub const NATIVE_WORD_NODE_AT: u16 = 41;
pub const GLYPH_NODE: u16 = 42;
pub const PIC_NODE: u16 = 43;
pub const PDF_NODE: u16 = 44;

pub const DISPLAY_STYLE: i16 = 0;
pub const TEXT_STYLE: i16 = 2;
pub const SCRIPT_STYLE: i16 = 4;
pub const SCRIPT_SCRIPT_STYLE: i16 = 6;

pub const TEXT_SIZE: i32 = 0;
pub const SCRIPT_SIZE: i32 = 256;
pub const SCRIPT_SCRIPT_SIZE: i32 = 512;

pub const TOTAL_MATHSY_PARAMS: i32 = 22;
pub const TOTAL_MATHEX_PARAMS: i32 = 13;
pub const INF_PENALTY: i32 = 10000;
pub const BIGGEST_LANG: i32 = 255;
pub const HMODE: i16 = 104;
pub const MMODE: i16 = 207;
pub const DLIST: u16 = 2;

pub const DELIMITED_CODE: i16 = 3;
pub const ABOVE_CODE: i16 = 0;
pub const OVER_CODE: i16 = 1;
pub const ATOP_CODE: i16 = 2;

pub const FIXED_ACC: u16 = 1;
pub const BOTTOM_ACC: u16 = 2;
pub const MATH_CHAR: i32 = 1;
pub const SUB_BOX: i32 = 2;
pub const SUB_MLIST: i32 = 3;
pub const MATH_TEXT_CHAR: i32 = 4;
pub const EMPTY: i32 = 0;

pub const LIMITS: u16 = 1;
pub const NO_LIMITS: u16 = 2;

pub const QUAD_CODE: usize = 6;
pub const X_HEIGHT_CODE: usize = 5;
pub const SPACE_CODE: usize = 2;

pub const LIG_TAG: u16 = 1;
pub const LIST_TAG: u16 = 2;
pub const EXT_TAG: u16 = 3;

pub const MU_GLUE: u16 = 99;
pub const EXPLICIT: u16 = 1;
pub const COND_MATH_GLUE: u16 = 98;

pub const FONT_BASE: i32 = 0;
pub const OTGR_FONT_FLAG: str_number = 0xFFFEu32 as i32;
pub const AAT_FONT_FLAG: str_number = 0xFFFFu32 as i32;

pub const DISPLAYOPERATORMINHEIGHT: i32 = 3;
pub const ACCENTBASEHEIGHT: i32 = 6;
pub const SUBSCRIPTTOPMAX: i32 = 9;
pub const SUPERSCRIPTBOTTOMMIN: i32 = 13;
pub const SUBSUPERSCRIPTGAPMIN: i32 = 15;
pub const SUPERSCRIPTBOTTOMMAXWITHSUBSCRIPT: i32 = 16;
pub const STACKGAPMIN: i32 = 26;
pub const STACKDISPLAYSTYLEGAPMIN: i32 = 27;
pub const FRACTIONNUMERATORGAPMIN: i32 = 36;
pub const FRACTIONNUMDISPLAYSTYLEGAPMIN: i32 = 37;
pub const FRACTIONDENOMINATORGAPMIN: i32 = 39;
pub const FRACTIONDENOMDISPLAYSTYLEGAPMIN: i32 = 40;
pub const RADICALVERTICALGAP: i32 = 49;
pub const RADICALDISPLAYSTYLEVERTICALGAP: i32 = 50;
pub const RADICALRULETHICKNESS: i32 = 51;

pub const SUP_CMD: i32 = 0;
pub const SUB_CMD: i32 = 1;

// Global C externs
unsafe extern "C" {
    pub static mut cur_cmd: u8;
    pub static mut cur_chr: i32;
    pub static mut cur_val: i32;
    pub static mut cur_val1: i32;
    pub static mut cur_list: list_state_record;
    pub static mut cur_dir: small_number;
    pub static mut cur_lang: u8;
    pub static mut cur_group: group_code;
    pub static mut just_box: i32;
    pub static mut LR_ptr: i32;
    pub static mut LR_problems: i32;
    pub static mut temp_ptr: i32;
    pub static mut avail: i32;
    pub static mut xtx_ligature_present: bool;
    pub static mut semantic_pagination_enabled: bool;
    pub static mut insert_src_special_every_math: bool;
    pub static mut file_line_error_style_p: i32;
    pub static mut save_ptr: i32;
    pub static mut nest_ptr: i32;
    pub static mut help_ptr: u8;
    pub static mut help_line: [*const c_char; 6];

    pub static mut save_stack: *mut memory_word;
    pub static mut mem: *mut memory_word;
    pub static mut eqtb: *mut memory_word;
    pub static mut font_info: *mut memory_word;
    pub static mut font_params: *mut i32;
    pub static mut font_area: *mut str_number;
    pub static mut font_layout_engine: *mut *mut core::ffi::c_void;
    pub static mut param_base: *mut i32;
    pub static mut lig_kern_base: *mut i32;
    pub static mut kern_base: *mut i32;
    pub static mut exten_base: *mut i32;
    pub static mut font_bc: *mut u16;
    pub static mut font_ec: *mut u16;
    pub static mut char_base: *mut i32;
    pub static mut width_base: *mut i32;
    pub static mut height_base: *mut i32;
    pub static mut depth_base: *mut i32;
    pub static mut italic_base: *mut i32;
    pub static mut skew_char: *mut i32;
    pub static mut adjust_tail: i32;
    pub static mut pre_adjust_tail: i32;
    pub static mut total_shrink: [scaled_t; 4];
    pub static mut hi_mem_min: i32;
    pub static mut mem_end: i32;
    pub static mut empty: b32x2;
    pub static mut cur_c: i32;
    pub static mut cur_f: internal_font_number;
    pub static mut cur_i: b16x4;
    pub static mut null_character: b16x4;
    pub static mut tex_remainder: scaled_t;

    pub fn get_token();
    pub fn pop_nest();
    pub fn line_break(d: bool);
    pub fn new_kern(w: scaled_t) -> i32;
    pub fn new_param_glue(n: small_number) -> i32;
    pub fn new_null_box() -> i32;
    pub fn new_math(w: scaled_t, s: small_number) -> i32;
    pub fn just_copy(p: i32, h: i32, t: i32);
    pub fn get_avail() -> i32;
    pub fn effective_char(err: bool, f: internal_font_number, c: u16) -> i32;
    pub fn just_reverse(p: i32);
    pub fn flush_node_list(p: i32);
    pub fn push_math(c: group_code);
    pub fn eq_word_define(p: i32, w: i32);
    pub fn tt_insert_special(ascii_text: *const c_char);
    pub fn begin_token_list(a: i32, t: u16);
    pub fn build_page();
    pub fn back_input();
    pub fn insert_src_special();
    pub fn error_here_with_diagnostic(msg: *const c_char) -> *mut tectonic_bridge_core::Diagnostic;
    pub fn capture_to_diagnostic(diagnostic: *mut tectonic_bridge_core::Diagnostic);
    pub fn error();
    pub fn scan_math_fam_int();
    pub fn scan_usv_num();
    pub fn scan_delimiter_int();
    pub fn get_x_token();
    pub fn scan_math_class_int();
    pub fn print_file_line();
    pub fn print_nl_cstr(s: *const c_char);
    pub fn print_cstr(s: *const c_char);
    pub fn back_error();
    pub fn get_node(s: i32) -> i32;
    pub fn scan_math(p: i32);
    pub fn print_esc_cstr(s: *const c_char);
    pub fn scan_keyword(s: *const c_char) -> bool;
    pub fn scan_fifteen_bit_int();
    pub fn new_choice() -> i32;
    pub fn scan_left_brace();
    pub fn confusion(s: *const c_char) -> !;
    pub fn unsave();
    pub fn new_noad() -> i32;
    pub fn scan_dimen(mu: bool, inf: bool, shortcut: bool);
    pub fn off_save();
    pub fn copy_node_list(p: i32) -> i32;
    pub fn free_node(p: i32, s: i32);
    pub fn new_skip_param(n: small_number) -> i32;
    pub fn hpack(p: i32, w: scaled_t, m: small_number) -> i32;
    pub fn append_to_vlist(b: i32);
    pub fn isOpenTypeMathFont(engine: *mut core::ffi::c_void) -> bool;
    pub fn get_native_mathsy_param(f: i32, n: i32) -> i32;
    pub fn get_native_mathex_param(f: i32, n: i32) -> i32;
    pub fn get_ot_math_constant(f: i32, n: i32) -> i32;
    pub fn get_ot_math_variant(f: i32, g: i32, v: i32, adv: *mut i32, horiz: i32) -> i32;
    pub fn get_ot_assembly_ptr(f: i32, g: i32, horiz: i32) -> *mut core::ffi::c_void;
    pub fn free_ot_assembly(a: *mut core::ffi::c_void);
    pub fn get_ot_math_ital_corr(f: i32, g: i32) -> i32;
    pub fn get_ot_math_accent_pos(f: i32, g: i32) -> i32;
    pub fn get_ot_math_kern(f: i32, g: i32, sf: i32, sg: i32, cmd: i32, shift: i32) -> i32;
    pub fn ot_part_count(a: *const core::ffi::c_void) -> i32;
    pub fn ot_part_glyph(a: *const core::ffi::c_void, i: i32) -> i32;
    pub fn ot_part_is_extender(a: *const core::ffi::c_void, i: i32) -> bool;
    pub fn ot_part_start_connector(f: i32, a: *const core::ffi::c_void, i: i32) -> i32;
    pub fn ot_part_end_connector(f: i32, a: *const core::ffi::c_void, i: i32) -> i32;
    pub fn ot_part_full_advance(f: i32, a: *const core::ffi::c_void, i: i32) -> i32;
    pub fn ot_min_connector_overlap(f: i32) -> i32;
    pub fn new_penalty(m: i32) -> i32;
    pub fn norm_min(h: i32) -> i16;
    pub fn push_nest();
    pub fn new_rule() -> i32;
    pub fn vpackage(p: i32, h: scaled_t, m: small_number, l: scaled_t) -> i32;
    pub fn x_over_n(x: scaled_t, n: i32) -> scaled_t;
    pub fn mult_and_add(n: i32, x: scaled_t, y: scaled_t, max_answer: scaled_t) -> scaled_t;
    pub fn xn_over_d(x: scaled_t, n: i32, d: i32) -> scaled_t;
    pub fn print_size(s: i32);
    pub fn print_char(s: i32);
    pub fn print_int(n: i32);
    pub fn print(s: i32);
    pub fn char_warning(f: internal_font_number, c: i32);
    pub fn new_native_character(f: internal_font_number, c: i32) -> i32;
    pub fn real_get_native_glyph(pNode: *mut core::ffi::c_void, index: u32) -> u16;
    pub fn measure_native_glyph(pNode: *mut core::ffi::c_void, use_glyph_metrics: i32);
    pub fn usingOpenType(engine: *mut core::ffi::c_void) -> bool;
    pub fn new_character(f: internal_font_number, c: u16) -> i32;
    pub fn delete_glue_ref(p: i32);
    pub fn new_glue(q: i32) -> i32;
    pub fn map_char_to_glyph(f: internal_font_number, c: i32) -> i32;
    pub fn new_spec(p: i32) -> i32;
    pub fn tex_round(x: f64) -> i32;
    pub fn half(x: i32) -> i32;
}

// Inline helper macros / functions
#[inline(always)]
pub unsafe fn GLUEPAR(p: i16) -> i32 {
    unsafe { (*eqtb.add(GLUE_BASE + p as usize)).b32.s1 }
}

#[inline(always)]
pub unsafe fn INTPAR(p: usize) -> i32 {
    unsafe { (*eqtb.add(INT_BASE + p)).b32.s1 }
}

#[inline(always)]
pub unsafe fn DIMENPAR(p: usize) -> i32 {
    unsafe { (*eqtb.add(DIMEN_BASE + p)).b32.s1 }
}

#[inline(always)]
pub unsafe fn LOCAL(p: usize) -> i32 {
    unsafe { (*eqtb.add(LOCAL_BASE + p)).b32.s1 }
}

#[inline(always)]
pub unsafe fn MATH_FONT(p: usize) -> i32 {
    unsafe { (*eqtb.add(MATH_FONT_BASE + p)).b32.s1 }
}

#[inline(always)]
pub unsafe fn DEL_CODE(p: i32) -> i32 {
    unsafe { (*eqtb.add(DEL_CODE_BASE + p as usize)).b32.s1 }
}

#[inline(always)]
pub unsafe fn LLIST_link(p: i32) -> i32 {
    unsafe { (*mem.add(p as usize)).b32.s1 }
}

#[inline(always)]
pub unsafe fn LLIST_info(p: i32) -> i32 {
    unsafe { (*mem.add(p as usize)).b32.s0 }
}

#[inline(always)]
pub unsafe fn NODE_type(p: i32) -> i16 {
    unsafe { (*mem.add(p as usize)).b16.s1 as i16 }
}

#[inline(always)]
pub unsafe fn NODE_subtype(p: i32) -> i16 {
    unsafe { (*mem.add(p as usize)).b16.s0 as i16 }
}

#[inline(always)]
pub unsafe fn set_NODE_type(p: i32, t: i16) {
    unsafe {
        (*mem.add(p as usize)).b16.s1 = t as u16;
    }
}

#[inline(always)]
pub unsafe fn set_NODE_subtype(p: i32, s: i16) {
    unsafe {
        (*mem.add(p as usize)).b16.s0 = s as u16;
    }
}

#[inline(always)]
pub unsafe fn is_char_node(p: i32) -> bool {
    unsafe { p >= hi_mem_min }
}

#[inline(always)]
pub unsafe fn CHAR_NODE_font(p: i32) -> internal_font_number {
    unsafe { (*mem.add(p as usize)).b16.s1 as internal_font_number }
}

#[inline(always)]
pub unsafe fn CHAR_NODE_character(p: i32) -> i32 {
    unsafe { (*mem.add(p as usize)).b16.s0 as i32 }
}

#[inline(always)]
pub unsafe fn FONT_CHARACTER_INFO(f: internal_font_number, c: i32) -> b16x4 {
    unsafe { (*font_info.add((*char_base.add(f as usize) + c) as usize)).b16 }
}

#[inline(always)]
pub unsafe fn FONT_CHARINFO_WIDTH(f: internal_font_number, info: b16x4) -> scaled_t {
    unsafe {
        (*font_info.add((*width_base.add(f as usize) + info.s3 as i32) as usize))
            .b32
            .s1
    }
}

#[inline(always)]
pub unsafe fn FONT_CHARINFO_HEIGHT(f: internal_font_number, info: b16x4) -> scaled_t {
    unsafe {
        (*font_info.add((*height_base.add(f as usize) + (info.s2 / 16) as i32) as usize))
            .b32
            .s1
    }
}

#[inline(always)]
pub unsafe fn FONT_CHARINFO_DEPTH(f: internal_font_number, info: b16x4) -> scaled_t {
    unsafe {
        (*font_info.add((*depth_base.add(f as usize) + (info.s2 % 16) as i32) as usize))
            .b32
            .s1
    }
}

#[inline(always)]
pub unsafe fn FONT_CHARINFO_ITALCORR(f: internal_font_number, info: b16x4) -> scaled_t {
    unsafe {
        (*font_info.add((*italic_base.add(f as usize) + (info.s1 / 4) as i32) as usize))
            .b32
            .s1
    }
}

#[inline(always)]
pub unsafe fn FONT_CHARACTER_WIDTH(f: internal_font_number, c: i32) -> scaled_t {
    unsafe { FONT_CHARINFO_WIDTH(f, FONT_CHARACTER_INFO(f, c)) }
}

#[inline(always)]
pub unsafe fn GLUE_SPEC_shrink_order(p: i32) -> i16 {
    unsafe { (*mem.add(p as usize)).b16.s0 as i16 }
}

#[inline(always)]
pub unsafe fn set_native_glyph_metrics(p: i32, g: i32) {
    unsafe {
        measure_native_glyph(mem.add(p as usize) as *mut _, g);
    }
}

#[inline(always)]
pub unsafe fn get_native_glyph(p: i32, i: u32) -> i32 {
    unsafe { real_get_native_glyph(mem.add(p as usize) as *mut _, i) as i32 }
}

#[inline(always)]
pub fn math_fam(x: i32) -> i32 {
    (((x as u32) >> 24) & 0xFF) as i32
}

#[inline(always)]
pub fn math_class(x: i32) -> i32 {
    (((x as u32) >> 21) & 0x07) as i32
}

#[inline(always)]
pub fn math_char(x: i32) -> i32 {
    ((x as u32) & 0x1FFFFF) as i32
}

#[inline(always)]
pub fn set_family(x: i32) -> i32 {
    (((x as u32) & 0xFF) << 24) as i32
}

#[inline(always)]
pub fn set_class(x: i32) -> i32 {
    (((x as u32) & 0x07) << 21) as i32
}

#[inline(always)]
pub fn odd(x: i32) -> bool {
    (x & 1) != 0
}

static mut null_delimiter: b16x4 = b16x4 {
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
};
static mut cur_mlist: i32 = 0;
static mut cur_style: small_number = 0;
static mut cur_size: i32 = 0;
static mut cur_mu: scaled_t = 0;
static mut mlist_penalties: bool = false;

#[unsafe(export_name = "initialize_math_variables")]
pub unsafe extern "C" fn initialize_math_variables() {
    unsafe {
        null_delimiter.s3 = 0;
        null_delimiter.s2 = 0;
        null_delimiter.s1 = 0;
        null_delimiter.s0 = 0;
    }
}

#[unsafe(export_name = "init_math")]
pub unsafe extern "C" fn init_math() {
    unsafe {
        let mut w: scaled_t;
        let mut j: i32;
        let mut x: i32;
        let mut l: scaled_t;
        let mut s: scaled_t;
        let mut p: i32;
        let mut q: i32;
        let mut f: internal_font_number;
        let mut n: i32;
        let mut v: scaled_t;
        let mut d: scaled_t;

        get_token();

        if cur_cmd == MATH_SHIFT && cur_list.mode > 0 {
            j = TEX_NULL;
            w = -MAX_HALFWORD;
            if cur_list.head == cur_list.tail {
                pop_nest();
                if cur_list.eTeX_aux == TEX_NULL {
                    x = 0;
                } else if (*mem.add(cur_list.eTeX_aux as usize)).b32.s0 >= R_CODE {
                    x = -1;
                } else {
                    x = 1;
                }
            } else {
                line_break(true);
                if GLUEPAR(GLUE_PAR__right_skip) == 0 {
                    j = new_kern(0);
                } else {
                    j = new_param_glue(GLUE_PAR__right_skip);
                }

                if GLUEPAR(GLUE_PAR__left_skip) == 0 {
                    p = new_kern(0);
                } else {
                    p = new_param_glue(GLUE_PAR__left_skip);
                }

                (*mem.add(p as usize)).b32.s1 = j;

                j = new_null_box();
                (*mem.add((j + 1) as usize)).b32.s1 = (*mem.add((just_box + 1) as usize)).b32.s1;
                (*mem.add((j + 4) as usize)).b32.s1 = (*mem.add((just_box + 4) as usize)).b32.s1;
                (*mem.add((j + 5) as usize)).b32.s1 = p;
                (*mem.add((j + 5) as usize)).b16.s0 = (*mem.add((just_box + 5) as usize)).b16.s0;
                (*mem.add((j + 5) as usize)).b16.s1 = (*mem.add((just_box + 5) as usize)).b16.s1;
                (*mem.add((j + 6) as usize)).gr = (*mem.add((just_box + 6) as usize)).gr;

                v = (*mem.add((just_box + 4) as usize)).b32.s1;
                if cur_list.eTeX_aux == TEX_NULL {
                    x = 0;
                } else if (*mem.add(cur_list.eTeX_aux as usize)).b32.s0 >= R_CODE {
                    x = -1;
                } else {
                    x = 1;
                }
                if x >= 0 {
                    p = (*mem.add((just_box + 5) as usize)).b32.s1;
                    (*mem.add(TEMP_HEAD as usize)).b32.s1 = TEX_NULL;
                } else {
                    v = -v - (*mem.add((just_box + 1) as usize)).b32.s1;
                    p = new_math(0, BEGIN_L_CODE);
                    (*mem.add(TEMP_HEAD as usize)).b32.s1 = p;
                    just_copy(
                        (*mem.add((just_box + 5) as usize)).b32.s1,
                        p,
                        new_math(0, END_L_CODE),
                    );
                    cur_dir = RIGHT_TO_LEFT;
                }
                v = v + 2
                    * (*font_info.add(
                        QUAD_CODE
                            + *param_base.add((*eqtb.add(CUR_FONT_LOC)).b32.s1 as usize) as usize,
                    ))
                    .b32
                    .s1;
                if INTPAR(INT_PAR__texxet) > 0 {
                    temp_ptr = get_avail();
                    (*mem.add(temp_ptr as usize)).b32.s0 = BEFORE as i32;
                    (*mem.add(temp_ptr as usize)).b32.s1 = LR_ptr;
                    LR_ptr = temp_ptr;
                }

                'done: while p != TEX_NULL {
                    let mut found = false;
                    loop {
                        if is_char_node(p) {
                            f = CHAR_NODE_font(p);
                            d = FONT_CHARACTER_WIDTH(
                                f,
                                effective_char(true, f, CHAR_NODE_character(p) as u16),
                            );
                            found = true;
                            break;
                        }
                        match (*mem.add(p as usize)).b16.s1 {
                            0 | 1 | 2 => {
                                d = (*mem.add((p + 1) as usize)).b32.s1;
                                found = true;
                                break;
                            }
                            6 => {
                                *mem.add(GARBAGE as usize) = *mem.add((p + 1) as usize);
                                (*mem.add(GARBAGE as usize)).b32.s1 = (*mem.add(p as usize)).b32.s1;
                                p = GARBAGE;
                                xtx_ligature_present = true;
                                continue; // reswitch
                            }
                            11 | 40 => {
                                d = (*mem.add((p + 1) as usize)).b32.s1;
                            }
                            9 => {
                                d = (*mem.add((p + 1) as usize)).b32.s1;
                                if INTPAR(INT_PAR__texxet) > 0 {
                                    if odd((*mem.add(p as usize)).b16.s0 as i32) {
                                        if (*mem.add(LR_ptr as usize)).b32.s0
                                            == (L_CODE
                                                * ((*mem.add(p as usize)).b16.s0 / L_CODE as u16)
                                                    as i16
                                                + 3)
                                                as i32
                                        {
                                            temp_ptr = LR_ptr;
                                            LR_ptr = (*mem.add(temp_ptr as usize)).b32.s1;
                                            (*mem.add(temp_ptr as usize)).b32.s1 = avail;
                                            avail = temp_ptr;
                                        } else if ((*mem.add(p as usize)).b16.s0 as i16) > L_CODE {
                                            w = MAX_HALFWORD;
                                            break 'done;
                                        }
                                    } else {
                                        temp_ptr = get_avail();
                                        (*mem.add(temp_ptr as usize)).b32.s0 = (L_CODE
                                            * ((*mem.add(p as usize)).b16.s0 / L_CODE as u16)
                                                as i16
                                            + 3)
                                            as i32;
                                        (*mem.add(temp_ptr as usize)).b32.s1 = LR_ptr;
                                        LR_ptr = temp_ptr;
                                        if (((*mem.add(p as usize)).b16.s0 as i32) / R_CODE)
                                            != cur_dir as i32
                                        {
                                            just_reverse(p);
                                            p = TEMP_HEAD;
                                        }
                                    }
                                } else if ((*mem.add(p as usize)).b16.s0 as i16) >= L_CODE {
                                    w = MAX_HALFWORD;
                                    break 'done;
                                }
                            }
                            14 => {
                                d = (*mem.add((p + 1) as usize)).b32.s1;
                                cur_dir = (*mem.add(p as usize)).b16.s0 as small_number;
                            }
                            10 => {
                                q = (*mem.add((p + 1) as usize)).b32.s0;
                                d = (*mem.add((q + 1) as usize)).b32.s1;
                                if (*mem.add((just_box + 5) as usize)).b16.s1 == STRETCHING {
                                    if (*mem.add((just_box + 5) as usize)).b16.s0
                                        == (*mem.add(q as usize)).b16.s1
                                        && (*mem.add((q + 2) as usize)).b32.s1 != 0
                                    {
                                        v = MAX_HALFWORD;
                                    }
                                } else if (*mem.add((just_box + 5) as usize)).b16.s1 == SHRINKING {
                                    if (*mem.add((just_box + 5) as usize)).b16.s0
                                        == (*mem.add(q as usize)).b16.s0
                                        && (*mem.add((q + 3) as usize)).b32.s1 != 0
                                    {
                                        v = MAX_HALFWORD;
                                    }
                                }
                                if (*mem.add(p as usize)).b16.s0 >= A_LEADERS {
                                    found = true;
                                    break;
                                }
                            }
                            8 => {
                                let sub = (*mem.add(p as usize)).b16.s0;
                                if sub == NATIVE_WORD_NODE
                                    || sub == NATIVE_WORD_NODE_AT
                                    || sub == GLYPH_NODE
                                    || sub == PIC_NODE
                                    || sub == PDF_NODE
                                {
                                    d = (*mem.add((p + 1) as usize)).b32.s1;
                                    found = true;
                                    break;
                                } else {
                                    d = 0;
                                }
                            }
                            _ => {
                                d = 0;
                            }
                        }
                        break;
                    }

                    if found {
                        if v < MAX_HALFWORD {
                            v = v + d;
                            w = v;
                        } else {
                            w = MAX_HALFWORD;
                            break 'done;
                        }
                    } else {
                        if v < MAX_HALFWORD {
                            v = v + d;
                        }
                    }
                    p = LLIST_link(p);
                }

                if INTPAR(INT_PAR__texxet) > 0 {
                    while LR_ptr != TEX_NULL {
                        temp_ptr = LR_ptr;
                        LR_ptr = (*mem.add(temp_ptr as usize)).b32.s1;
                        (*mem.add(temp_ptr as usize)).b32.s1 = avail;
                        avail = temp_ptr;
                    }
                    if LR_problems != 0 {
                        w = MAX_HALFWORD;
                        LR_problems = 0;
                    }
                }
                cur_dir = LEFT_TO_RIGHT;
                flush_node_list((*mem.add(TEMP_HEAD as usize)).b32.s1);
            }

            if LOCAL(LOCAL__par_shape) == TEX_NULL {
                if DIMENPAR(DIMEN_PAR__hang_indent) != 0
                    && ((INTPAR(INT_PAR__hang_after) >= 0
                        && cur_list.prev_graf + 2 > INTPAR(INT_PAR__hang_after))
                        || (cur_list.prev_graf + 1 < -INTPAR(INT_PAR__hang_after)))
                {
                    l = DIMENPAR(DIMEN_PAR__hsize) - (DIMENPAR(DIMEN_PAR__hang_indent)).abs();
                    if DIMENPAR(DIMEN_PAR__hang_indent) > 0 {
                        s = DIMENPAR(DIMEN_PAR__hang_indent);
                    } else {
                        s = 0;
                    }
                } else {
                    l = DIMENPAR(DIMEN_PAR__hsize);
                    s = 0;
                }
            } else {
                n = (*mem.add(LOCAL(LOCAL__par_shape) as usize)).b32.s0;
                if cur_list.prev_graf + 2 >= n {
                    p = LOCAL(LOCAL__par_shape) + 2 * n;
                } else {
                    p = LOCAL(LOCAL__par_shape) + 2 * (cur_list.prev_graf + 2);
                }
                s = (*mem.add((p - 1) as usize)).b32.s1;
                l = (*mem.add(p as usize)).b32.s1;
            }

            push_math(MATH_SHIFT_GROUP);
            cur_list.mode = MMODE;
            eq_word_define((INT_BASE + INT_PAR__cur_fam) as i32, -1);
            eq_word_define((DIMEN_BASE + DIMEN_PAR__pre_display_size) as i32, w);
            cur_list.eTeX_aux = j;
            eq_word_define((INT_BASE + INT_PAR__pre_display_direction) as i32, x);
            eq_word_define((DIMEN_BASE + DIMEN_PAR__display_width) as i32, l);
            eq_word_define((DIMEN_BASE + DIMEN_PAR__display_indent) as i32, s);

            if semantic_pagination_enabled {
                tt_insert_special(b"tdux:cs dmath\0".as_ptr() as *const c_char);
            }

            if LOCAL(LOCAL__every_display) != TEX_NULL {
                begin_token_list(LOCAL(LOCAL__every_display), EVERY_DISPLAY_TEXT as u16);
            }
            if nest_ptr == 1 {
                build_page();
            }
        } else {
            back_input();
            push_math(MATH_SHIFT_GROUP);
            eq_word_define((INT_BASE + INT_PAR__cur_fam) as i32, -1);
            if insert_src_special_every_math {
                insert_src_special();
            }

            if semantic_pagination_enabled {
                tt_insert_special(b"tdux:cs math\0".as_ptr() as *const c_char);
            }

            if LOCAL(LOCAL__every_math) != TEX_NULL {
                begin_token_list(LOCAL(LOCAL__every_math), EVERY_MATH_TEXT as u16);
            }
        }
    }
}

#[unsafe(export_name = "start_eq_no")]
pub unsafe extern "C" fn start_eq_no() {
    unsafe {
        (*save_stack.add(save_ptr as usize)).b32.s1 = cur_chr;
        save_ptr += 1;

        push_math(MATH_SHIFT_GROUP);
        eq_word_define((INT_BASE + INT_PAR__cur_fam) as i32, -1);
        if insert_src_special_every_math {
            insert_src_special();
        }
        if LOCAL(LOCAL__every_math) != TEX_NULL {
            begin_token_list(LOCAL(LOCAL__every_math), EVERY_MATH_TEXT as u16);
        }
    }
}

#[unsafe(export_name = "math_limit_switch")]
pub unsafe extern "C" fn math_limit_switch() {
    unsafe {
        if cur_list.head != cur_list.tail {
            if (*mem.add(cur_list.tail as usize)).b16.s1 == OP_NOAD as u16 {
                (*mem.add(cur_list.tail as usize)).b16.s0 = cur_chr as u16;
                return;
            }
        }
        error_here_with_diagnostic(
            b"Limit controls must follow a math operator\0".as_ptr() as *const c_char
        );
        capture_to_diagnostic(core::ptr::null_mut());
        help_ptr = 1;
        help_line[0] = b"I'm ignoring this misplaced \\limits or \\nolimits command.\0".as_ptr()
            as *const c_char;
        error();
    }
}

unsafe fn scan_delimiter(p: i32, r: bool) {
    unsafe {
        if r {
            if cur_chr == 1 {
                cur_val1 = 0x40000000;
                scan_math_fam_int();
                cur_val1 += cur_val * 0x200000;
                scan_usv_num();
                cur_val += cur_val1;
            } else {
                scan_delimiter_int();
            }
        } else {
            loop {
                get_x_token();
                if !(cur_cmd == SPACER || cur_cmd == RELAX) {
                    break;
                }
            }

            match cur_cmd {
                LETTER | OTHER_CHAR => {
                    cur_val = DEL_CODE(cur_chr);
                }
                DELIM_NUM => {
                    if cur_chr == 1 {
                        cur_val1 = 0x40000000;
                        scan_math_class_int();
                        scan_math_fam_int();
                        cur_val1 += cur_val * 0x20000;
                        scan_usv_num();
                        cur_val += cur_val1;
                    } else {
                        scan_delimiter_int();
                    }
                }
                _ => {
                    cur_val = -1;
                }
            }
        }

        if cur_val < 0 {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! \0".as_ptr() as *const c_char);
            }
            print_cstr(b"Missing delimiter (. inserted)\0".as_ptr() as *const c_char);
            help_ptr = 6;
            help_line[5] = b"I was expecting to see something like `(' or `\\{' or\0".as_ptr()
                as *const c_char;
            help_line[4] = b"`\\}' here. If you typed, e.g., `{' instead of `\\{', you\0".as_ptr()
                as *const c_char;
            help_line[3] = b"should probably delete the `{' by typing `1' now, so that\0".as_ptr()
                as *const c_char;
            help_line[2] =
                b"braces don't get unbalanced. Otherwise just proceed.\0".as_ptr() as *const c_char;
            help_line[1] = b"Acceptable delimiters are characters whose \\delcode is\0".as_ptr()
                as *const c_char;
            help_line[0] = b"nonnegative, or you can use `\\delimiter <delimiter code>'.\0".as_ptr()
                as *const c_char;
            back_error();
            cur_val = 0;
        }

        if cur_val >= 0x40000000 {
            (*mem.add(p as usize)).b16.s3 =
                (((cur_val % 0x200000) / 0x10000) * 0x100 + (cur_val / 0x200000) % 0x100) as u16;
            (*mem.add(p as usize)).b16.s2 = (cur_val % 0x10000) as u16;
            (*mem.add(p as usize)).b16.s1 = 0;
            (*mem.add(p as usize)).b16.s0 = 0;
        } else {
            (*mem.add(p as usize)).b16.s3 = ((cur_val / 0x100000) % 16) as u16;
            (*mem.add(p as usize)).b16.s2 = ((cur_val / 0x1000) % 0x100) as u16;
            (*mem.add(p as usize)).b16.s1 = ((cur_val / 0x100) % 16) as u16;
            (*mem.add(p as usize)).b16.s0 = (cur_val % 0x100) as u16;
        }
    }
}

#[unsafe(export_name = "math_radical")]
pub unsafe extern "C" fn math_radical() {
    unsafe {
        (*mem.add(cur_list.tail as usize)).b32.s1 = get_node(RADICAL_NOAD_SIZE);
        cur_list.tail = LLIST_link(cur_list.tail);

        (*mem.add(cur_list.tail as usize)).b16.s1 = RADICAL_NOAD as u16;
        (*mem.add(cur_list.tail as usize)).b16.s0 = NORMAL as u16;
        (*mem.add((cur_list.tail + 1) as usize)).b32 = empty;
        (*mem.add((cur_list.tail + 3) as usize)).b32 = empty;
        (*mem.add((cur_list.tail + 2) as usize)).b32 = empty;
        scan_delimiter(cur_list.tail + 4, true);
        scan_math(cur_list.tail + 1);
    }
}

#[unsafe(export_name = "math_ac")]
pub unsafe extern "C" fn math_ac() {
    unsafe {
        let mut c: i32;

        if cur_cmd == ACCENT {
            error_here_with_diagnostic(b"Please use \0".as_ptr() as *const c_char);
            print_esc_cstr(b"mathaccent\0".as_ptr() as *const c_char);
            print_cstr(b" for accents in math mode\0".as_ptr() as *const c_char);
            capture_to_diagnostic(core::ptr::null_mut());
            help_ptr = 2;
            help_line[1] = b"I'm changing \\accent to \\mathaccent here; wish me luck.\0".as_ptr()
                as *const c_char;
            help_line[0] = b"(Accents are not the same in formulas as they are in text.)\0".as_ptr()
                as *const c_char;
            error();
        }

        (*mem.add(cur_list.tail as usize)).b32.s1 = get_node(ACCENT_NOAD_SIZE);
        cur_list.tail = LLIST_link(cur_list.tail);

        (*mem.add(cur_list.tail as usize)).b16.s1 = ACCENT_NOAD as u16;
        (*mem.add(cur_list.tail as usize)).b16.s0 = NORMAL as u16;
        (*mem.add((cur_list.tail + 1) as usize)).b32 = empty;
        (*mem.add((cur_list.tail + 3) as usize)).b32 = empty;
        (*mem.add((cur_list.tail + 2) as usize)).b32 = empty;
        (*mem.add((cur_list.tail + 4) as usize)).b32.s1 = MATH_CHAR;

        if cur_chr == 1 {
            if scan_keyword(b"fixed\0".as_ptr() as *const c_char) {
                (*mem.add(cur_list.tail as usize)).b16.s0 = FIXED_ACC;
            } else if scan_keyword(b"bottom\0".as_ptr() as *const c_char) {
                if scan_keyword(b"fixed\0".as_ptr() as *const c_char) {
                    (*mem.add(cur_list.tail as usize)).b16.s0 = BOTTOM_ACC + 1;
                } else {
                    (*mem.add(cur_list.tail as usize)).b16.s0 = BOTTOM_ACC;
                }
            }
            scan_math_class_int();
            c = set_class(cur_val);
            scan_math_fam_int();
            c += set_family(cur_val);
            scan_usv_num();
            cur_val += c;
        } else {
            scan_fifteen_bit_int();
            cur_val =
                set_class(cur_val / 4096) + set_family((cur_val % 4096) / 256) + (cur_val % 256);
        }

        (*mem.add((cur_list.tail + 4) as usize)).b16.s0 = (cur_val % 65536) as u16;
        if math_class(cur_val) == 7
            && INTPAR(INT_PAR__cur_fam) >= 0
            && INTPAR(INT_PAR__cur_fam) < NUMBER_MATH_FAMILIES
        {
            (*mem.add((cur_list.tail + 4) as usize)).b16.s1 = INTPAR(INT_PAR__cur_fam) as u16;
        } else {
            (*mem.add((cur_list.tail + 4) as usize)).b16.s1 = math_fam(cur_val) as u16;
        }
        (*mem.add((cur_list.tail + 4) as usize)).b16.s1 +=
            ((math_char(cur_val) / 65536) * 256) as u16;
        scan_math(cur_list.tail + 1);
    }
}

#[unsafe(export_name = "append_choices")]
pub unsafe extern "C" fn append_choices() {
    unsafe {
        (*mem.add(cur_list.tail as usize)).b32.s1 = new_choice();
        cur_list.tail = LLIST_link(cur_list.tail);

        save_ptr += 1;
        (*save_stack.add((save_ptr - 1) as usize)).b32.s1 = 0;
        push_math(MATH_CHOICE_GROUP);
        scan_left_brace();
    }
}

#[unsafe(export_name = "fin_mlist")]
pub unsafe extern "C" fn fin_mlist(p: i32) -> i32 {
    unsafe {
        let q: i32;
        if cur_list.aux.b32.s1 != TEX_NULL {
            (*mem.add((cur_list.aux.b32.s1 + 3) as usize)).b32.s1 = SUB_MLIST;
            (*mem.add((cur_list.aux.b32.s1 + 3) as usize)).b32.s0 =
                (*mem.add(cur_list.head as usize)).b32.s1;
            if p == TEX_NULL {
                q = cur_list.aux.b32.s1;
            } else {
                q = (*mem.add((cur_list.aux.b32.s1 + 2) as usize)).b32.s0;
                if (*mem.add(q as usize)).b16.s1 != LEFT_NOAD as u16
                    || cur_list.eTeX_aux == TEX_NULL
                {
                    confusion(b"right\0".as_ptr() as *const c_char);
                }
                (*mem.add((cur_list.aux.b32.s1 + 2) as usize)).b32.s0 =
                    (*mem.add(cur_list.eTeX_aux as usize)).b32.s1;
                (*mem.add(cur_list.eTeX_aux as usize)).b32.s1 = cur_list.aux.b32.s1;
                (*mem.add(cur_list.aux.b32.s1 as usize)).b32.s1 = p;
            }
        } else {
            (*mem.add(cur_list.tail as usize)).b32.s1 = p;
            q = (*mem.add(cur_list.head as usize)).b32.s1;
        }
        pop_nest();
        q
    }
}

#[unsafe(export_name = "build_choices")]
pub unsafe extern "C" fn build_choices() {
    unsafe {
        let p: i32;
        unsave();
        p = fin_mlist(TEX_NULL);
        match (*save_stack.add((save_ptr - 1) as usize)).b32.s1 {
            0 => {
                (*mem.add((cur_list.tail + 1) as usize)).b32.s0 = p;
            }
            1 => {
                (*mem.add((cur_list.tail + 1) as usize)).b32.s1 = p;
            }
            2 => {
                (*mem.add((cur_list.tail + 2) as usize)).b32.s0 = p;
            }
            3 => {
                (*mem.add((cur_list.tail + 2) as usize)).b32.s1 = p;
                save_ptr -= 1;
                return;
            }
            _ => {}
        }
        (*save_stack.add((save_ptr - 1) as usize)).b32.s1 += 1;
        push_math(MATH_CHOICE_GROUP);
        scan_left_brace();
    }
}

#[unsafe(export_name = "sub_sup")]
pub unsafe extern "C" fn sub_sup() {
    unsafe {
        let mut t: small_number = EMPTY as small_number;
        let mut p: i32 = TEX_NULL;

        if cur_list.tail != cur_list.head {
            let ntype = (*mem.add(cur_list.tail as usize)).b16.s1 as i16;
            if ntype >= ORD_NOAD && ntype < LEFT_NOAD {
                p = cur_list.tail + 2 + cur_cmd as i32 - 7;
                t = (*mem.add(p as usize)).b32.s1 as small_number;
            }
        }

        if p == TEX_NULL || t != EMPTY as small_number {
            (*mem.add(cur_list.tail as usize)).b32.s1 = new_noad();
            cur_list.tail = LLIST_link(cur_list.tail);

            p = cur_list.tail + 2 + cur_cmd as i32 - 7;
            if t != EMPTY as small_number {
                if cur_cmd == SUP_MARK {
                    error_here_with_diagnostic(b"Double superscript\0".as_ptr() as *const c_char);
                    capture_to_diagnostic(core::ptr::null_mut());
                    help_ptr = 1;
                    help_line[0] =
                        b"I treat `x^1^2' essentially like `x^1{}^2'.\0".as_ptr() as *const c_char;
                } else {
                    error_here_with_diagnostic(b"Double subscript\0".as_ptr() as *const c_char);
                    capture_to_diagnostic(core::ptr::null_mut());
                    help_ptr = 1;
                    help_line[0] =
                        b"I treat `x_1_2' essentially like `x_1{}_2'.\0".as_ptr() as *const c_char;
                }
                error();
            }
        }
        scan_math(p);
    }
}

#[unsafe(export_name = "math_fraction")]
pub unsafe extern "C" fn math_fraction() {
    unsafe {
        let c: small_number = cur_chr as small_number;

        if cur_list.aux.b32.s1 != TEX_NULL {
            if c >= DELIMITED_CODE {
                scan_delimiter(GARBAGE, false);
                scan_delimiter(GARBAGE, false);
            }

            if c % DELIMITED_CODE == ABOVE_CODE {
                scan_dimen(false, false, false);
            }

            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! \0".as_ptr() as *const c_char);
            }
            print_cstr(b"Ambiguous; you need another { and }\0".as_ptr() as *const c_char);
            help_ptr = 3;
            help_line[2] = b"I'm ignoring this fraction specification, since I don't\0".as_ptr()
                as *const c_char;
            help_line[1] = b"know whether a construction like `x \\over y \\over z'\0".as_ptr()
                as *const c_char;
            help_line[0] = b"means `{x \\over y} \\over z' or `x \\over {y \\over z}'.\0".as_ptr()
                as *const c_char;
            error();
        } else {
            cur_list.aux.b32.s1 = get_node(FRACTION_NOAD_SIZE);
            (*mem.add(cur_list.aux.b32.s1 as usize)).b16.s1 = FRACTION_NOAD as u16;
            (*mem.add(cur_list.aux.b32.s1 as usize)).b16.s0 = NORMAL as u16;
            (*mem.add((cur_list.aux.b32.s1 + 2) as usize)).b32.s1 = SUB_MLIST;
            (*mem.add((cur_list.aux.b32.s1 + 2) as usize)).b32.s0 =
                (*mem.add(cur_list.head as usize)).b32.s1;
            (*mem.add((cur_list.aux.b32.s1 + 3) as usize)).b32 = empty;
            (*mem.add((cur_list.aux.b32.s1 + 4) as usize)).b16 = null_delimiter;
            (*mem.add((cur_list.aux.b32.s1 + 5) as usize)).b16 = null_delimiter;
            (*mem.add(cur_list.head as usize)).b32.s1 = TEX_NULL;

            cur_list.tail = cur_list.head;

            if c >= DELIMITED_CODE {
                scan_delimiter(cur_list.aux.b32.s1 + 4, false);
                scan_delimiter(cur_list.aux.b32.s1 + 5, false);
            }

            match c % DELIMITED_CODE {
                ABOVE_CODE => {
                    scan_dimen(false, false, false);
                    (*mem.add((cur_list.aux.b32.s1 + 1) as usize)).b32.s1 = cur_val;
                }
                OVER_CODE => {
                    (*mem.add((cur_list.aux.b32.s1 + 1) as usize)).b32.s1 = DEFAULT_CODE;
                }
                ATOP_CODE => {
                    (*mem.add((cur_list.aux.b32.s1 + 1) as usize)).b32.s1 = 0;
                }
                _ => {}
            }
        }
    }
}

#[unsafe(export_name = "math_left_right")]
pub unsafe extern "C" fn math_left_right() {
    unsafe {
        let t: small_number = cur_chr as small_number;
        let p: i32;
        let q: i32;

        if t != LEFT_NOAD && cur_group != MATH_LEFT_GROUP {
            if cur_group == MATH_SHIFT_GROUP {
                scan_delimiter(GARBAGE, false);
                error_here_with_diagnostic(b"Extra \0".as_ptr() as *const c_char);
                if t == 1 {
                    print_esc_cstr(b"middle\0".as_ptr() as *const c_char);
                    help_ptr = 1;
                    help_line[0] = b"I'm ignoring a \\middle that had no matching \\left.\0"
                        .as_ptr() as *const c_char;
                } else {
                    print_esc_cstr(b"right\0".as_ptr() as *const c_char);
                    help_ptr = 1;
                    help_line[0] = b"I'm ignoring a \\right that had no matching \\left.\0".as_ptr()
                        as *const c_char;
                }
                capture_to_diagnostic(core::ptr::null_mut());
                error();
            } else {
                off_save();
            }
        } else {
            p = new_noad();
            (*mem.add(p as usize)).b16.s1 = t as u16;
            scan_delimiter(p + 1, false);
            if t == 1 {
                (*mem.add(p as usize)).b16.s1 = RIGHT_NOAD as u16;
                (*mem.add(p as usize)).b16.s0 = 1;
            }
            if t == LEFT_NOAD {
                q = p;
            } else {
                q = fin_mlist(p);
                unsave();
            }
            if t != RIGHT_NOAD {
                push_math(MATH_LEFT_GROUP);
                (*mem.add(cur_list.head as usize)).b32.s1 = q;
                cur_list.tail = p;
                cur_list.eTeX_aux = p;
            } else {
                (*mem.add(cur_list.tail as usize)).b32.s1 = new_noad();
                cur_list.tail = LLIST_link(cur_list.tail);

                (*mem.add(cur_list.tail as usize)).b16.s1 = INNER_NOAD as u16;
                (*mem.add((cur_list.tail + 1) as usize)).b32.s1 = SUB_MLIST;
                (*mem.add((cur_list.tail + 1) as usize)).b32.s0 = q;
            }
        }
    }
}

unsafe fn app_display(j: i32, mut b: i32, mut d: scaled_t) {
    unsafe {
        let mut z: scaled_t;
        let mut s: scaled_t;
        let mut e: scaled_t;
        let x: i32;
        let mut p: i32;
        let mut q: i32;
        let mut r: i32;
        let mut t: i32;
        let mut u: i32;

        s = DIMENPAR(DIMEN_PAR__display_indent);
        x = INTPAR(INT_PAR__pre_display_direction);

        if x == 0 {
            (*mem.add((b + 4) as usize)).b32.s1 = s + d;
        } else {
            z = DIMENPAR(DIMEN_PAR__display_width);
            p = b;
            if x > 0 {
                e = z - d - (*mem.add((p + 1) as usize)).b32.s1;
            } else {
                e = d;
                d = z - e - (*mem.add((p + 1) as usize)).b32.s1;
            }
            if j != TEX_NULL {
                b = copy_node_list(j);
                (*mem.add((b + 3) as usize)).b32.s1 = (*mem.add((p + 3) as usize)).b32.s1;
                (*mem.add((b + 2) as usize)).b32.s1 = (*mem.add((p + 2) as usize)).b32.s1;
                s = s - (*mem.add((b + 4) as usize)).b32.s1;
                d = d + s;
                e = e + (*mem.add((b + 1) as usize)).b32.s1 - z - s;
            }
            if (*mem.add(p as usize)).b16.s0 == DLIST {
                q = p;
            } else {
                r = (*mem.add((p + 5) as usize)).b32.s1;
                free_node(p, BOX_NODE_SIZE);
                if r == TEX_NULL {
                    confusion(b"LR4\0".as_ptr() as *const c_char);
                }
                if x > 0 {
                    p = r;
                    loop {
                        q = r;
                        r = LLIST_link(r);
                        if r == TEX_NULL {
                            break;
                        }
                    }
                } else {
                    p = TEX_NULL;
                    q = r;
                    loop {
                        t = (*mem.add(r as usize)).b32.s1;
                        (*mem.add(r as usize)).b32.s1 = p;
                        p = r;
                        r = t;
                        if r == TEX_NULL {
                            break;
                        }
                    }
                }
            }
            if j == TEX_NULL {
                r = new_kern(0);
                t = new_kern(0);
            } else {
                r = (*mem.add((b + 5) as usize)).b32.s1;
                t = (*mem.add(r as usize)).b32.s1;
            }
            u = new_math(0, END_M_CODE);
            if NODE_type(t) == GLUE_NODE {
                let mut j_glue = new_skip_param(GLUE_PAR__right_skip);
                (*mem.add(q as usize)).b32.s1 = j_glue;
                (*mem.add(j_glue as usize)).b32.s1 = u;
                j_glue = (*mem.add((t + 1) as usize)).b32.s0;
                (*mem.add(temp_ptr as usize)).b16.s1 = (*mem.add(j_glue as usize)).b16.s1;
                (*mem.add(temp_ptr as usize)).b16.s0 = (*mem.add(j_glue as usize)).b16.s0;
                (*mem.add((temp_ptr + 1) as usize)).b32.s1 =
                    e - (*mem.add((j_glue + 1) as usize)).b32.s1;
                (*mem.add((temp_ptr + 2) as usize)).b32.s1 =
                    -(*mem.add((j_glue + 2) as usize)).b32.s1;
                (*mem.add((temp_ptr + 3) as usize)).b32.s1 =
                    -(*mem.add((j_glue + 3) as usize)).b32.s1;
                (*mem.add(u as usize)).b32.s1 = t;
            } else {
                (*mem.add((t + 1) as usize)).b32.s1 = e;
                (*mem.add(t as usize)).b32.s1 = u;
                (*mem.add(q as usize)).b32.s1 = t;
            }
            u = new_math(0, BEGIN_M_CODE);
            if NODE_type(r) == GLUE_NODE {
                let mut j_glue = new_skip_param(GLUE_PAR__left_skip);
                (*mem.add(u as usize)).b32.s1 = j_glue;
                (*mem.add(j_glue as usize)).b32.s1 = p;
                j_glue = (*mem.add((r + 1) as usize)).b32.s0;
                (*mem.add(temp_ptr as usize)).b16.s1 = (*mem.add(j_glue as usize)).b16.s1;
                (*mem.add(temp_ptr as usize)).b16.s0 = (*mem.add(j_glue as usize)).b16.s0;
                (*mem.add((temp_ptr + 1) as usize)).b32.s1 =
                    d - (*mem.add((j_glue + 1) as usize)).b32.s1;
                (*mem.add((temp_ptr + 2) as usize)).b32.s1 =
                    -(*mem.add((j_glue + 2) as usize)).b32.s1;
                (*mem.add((temp_ptr + 3) as usize)).b32.s1 =
                    -(*mem.add((j_glue + 3) as usize)).b32.s1;
                (*mem.add(r as usize)).b32.s1 = u;
            } else {
                (*mem.add((r + 1) as usize)).b32.s1 = d;
                (*mem.add(r as usize)).b32.s1 = p;
                (*mem.add(u as usize)).b32.s1 = r;
                if j == TEX_NULL {
                    b = hpack(u, 0, ADDITIONAL);
                    (*mem.add((b + 4) as usize)).b32.s1 = s;
                } else {
                    (*mem.add((b + 5) as usize)).b32.s1 = u;
                }
            }
        }
        append_to_vlist(b);
    }
}

#[unsafe(export_name = "after_math")]
pub unsafe extern "C" fn after_math() {
    unsafe {
        let mut l: bool;
        let mut danger: bool;
        let mut m: i32;
        let mut p: i32;
        let mut a: i32;
        let mut b: i32;
        let mut w: scaled_t;
        let mut z: scaled_t;
        let mut e: scaled_t;
        let mut q: scaled_t;
        let mut d: scaled_t;
        let mut s: scaled_t;
        let mut g1: i16;
        let mut g2: i16;
        let mut r: i32;
        let mut t: i32;
        let mut pre_t: i32;
        let mut j: i32 = TEX_NULL;

        danger = false;

        if cur_list.mode == MMODE {
            j = cur_list.eTeX_aux;
        }

        let f2 = MATH_FONT(2);
        let f2_script = MATH_FONT(2 + SCRIPT_SIZE as usize);
        let f2_sscript = MATH_FONT(2 + SCRIPT_SCRIPT_SIZE as usize);

        if ((*font_params.add(f2 as usize) < TOTAL_MATHSY_PARAMS)
            && !(*font_area.add(f2 as usize) == OTGR_FONT_FLAG
                && isOpenTypeMathFont(*font_layout_engine.add(f2 as usize))))
            || ((*font_params.add(f2_script as usize) < TOTAL_MATHSY_PARAMS)
                && !(*font_area.add(f2_script as usize) == OTGR_FONT_FLAG
                    && isOpenTypeMathFont(*font_layout_engine.add(f2_script as usize))))
            || ((*font_params.add(f2_sscript as usize) < TOTAL_MATHSY_PARAMS)
                && !(*font_area.add(f2_sscript as usize) == OTGR_FONT_FLAG
                    && isOpenTypeMathFont(*font_layout_engine.add(f2_sscript as usize))))
        {
            error_here_with_diagnostic(
                b"Math formula deleted: Insufficient symbol fonts\0".as_ptr() as *const c_char,
            );
            capture_to_diagnostic(core::ptr::null_mut());
            help_ptr = 3;
            help_line[2] =
                b"Sorry, but I can't typeset math unless \\textfont 2\0".as_ptr() as *const c_char;
            help_line[1] =
                b"and \\scriptfont 2 and \\scriptscriptfont 2 have all\0".as_ptr() as *const c_char;
            help_line[0] =
                b"the \\fontdimen values needed in math symbol fonts.\0".as_ptr() as *const c_char;
            error();
            flush_math();
            danger = true;
        } else {
            let f3_text = MATH_FONT(3 + TEXT_SIZE as usize);
            let f3_script = MATH_FONT(3 + SCRIPT_SIZE as usize);
            let f3_sscript = MATH_FONT(3 + SCRIPT_SCRIPT_SIZE as usize);

            if ((*font_params.add(f3_text as usize) < TOTAL_MATHEX_PARAMS)
                && !(*font_area.add(f3_text as usize) == OTGR_FONT_FLAG
                    && isOpenTypeMathFont(*font_layout_engine.add(f3_text as usize))))
                || ((*font_params.add(f3_script as usize) < TOTAL_MATHEX_PARAMS)
                    && !(*font_area.add(f3_script as usize) == OTGR_FONT_FLAG
                        && isOpenTypeMathFont(*font_layout_engine.add(f3_script as usize))))
                || ((*font_params.add(f3_sscript as usize) < TOTAL_MATHEX_PARAMS)
                    && !(*font_area.add(f3_sscript as usize) == OTGR_FONT_FLAG
                        && isOpenTypeMathFont(*font_layout_engine.add(f3_sscript as usize))))
            {
                error_here_with_diagnostic(
                    b"Math formula deleted: Insufficient extension fonts\0".as_ptr()
                        as *const c_char,
                );
                capture_to_diagnostic(core::ptr::null_mut());
                help_ptr = 3;
                help_line[2] = b"Sorry, but I can't typeset math unless \\textfont 3\0".as_ptr()
                    as *const c_char;
                help_line[1] = b"and \\scriptfont 3 and \\scriptscriptfont 3 have all\0".as_ptr()
                    as *const c_char;
                help_line[0] = b"the \\fontdimen values needed in math extension fonts.\0".as_ptr()
                    as *const c_char;
                error();
                flush_math();
                danger = true;
            }
        }

        m = cur_list.mode as i32;
        l = false;
        p = fin_mlist(TEX_NULL);

        if cur_list.mode as i32 == -m {
            get_x_token();
            if cur_cmd != MATH_SHIFT {
                error_here_with_diagnostic(
                    b"Display math should end with $$\0".as_ptr() as *const c_char
                );
                capture_to_diagnostic(core::ptr::null_mut());
                help_ptr = 2;
                help_line[1] = b"The `$' that I just saw supposedly matches a previous `$$'.\0"
                    .as_ptr() as *const c_char;
                help_line[0] = b"So I shall assume that you typed `$$' both times.\0".as_ptr()
                    as *const c_char;
                back_error();
            }
            cur_mlist = p;
            cur_style = TEXT_STYLE;
            mlist_penalties = false;
            mlist_to_hlist();
            a = hpack((*mem.add(TEMP_HEAD as usize)).b32.s1, 0, ADDITIONAL);
            (*mem.add(a as usize)).b16.s0 = DLIST;
            unsave();
            save_ptr -= 1;
            if (*save_stack.add(save_ptr as usize)).b32.s1 == 1 {
                l = true;
            }
            danger = false;
            if cur_list.mode == MMODE {
                j = cur_list.eTeX_aux;
            }

            let f2 = MATH_FONT(2);
            let f2_script = MATH_FONT(2 + SCRIPT_SIZE as usize);
            let f2_sscript = MATH_FONT(2 + SCRIPT_SCRIPT_SIZE as usize);

            if ((*font_params.add(f2 as usize) < TOTAL_MATHSY_PARAMS)
                && !(*font_area.add(f2 as usize) == OTGR_FONT_FLAG
                    && isOpenTypeMathFont(*font_layout_engine.add(f2 as usize))))
                || ((*font_params.add(f2_script as usize) < TOTAL_MATHSY_PARAMS)
                    && !(*font_area.add(f2_script as usize) == OTGR_FONT_FLAG
                        && isOpenTypeMathFont(*font_layout_engine.add(f2_script as usize))))
                || ((*font_params.add(f2_sscript as usize) < TOTAL_MATHSY_PARAMS)
                    && !(*font_area.add(f2_sscript as usize) == OTGR_FONT_FLAG
                        && isOpenTypeMathFont(*font_layout_engine.add(f2_sscript as usize))))
            {
                error_here_with_diagnostic(
                    b"Math formula deleted: Insufficient symbol fonts\0".as_ptr() as *const c_char,
                );
                capture_to_diagnostic(core::ptr::null_mut());
                help_ptr = 3;
                help_line[2] = b"Sorry, but I can't typeset math unless \\textfont 2\0".as_ptr()
                    as *const c_char;
                help_line[1] = b"and \\scriptfont 2 and \\scriptscriptfont 2 have all\0".as_ptr()
                    as *const c_char;
                help_line[0] = b"the \\fontdimen values needed in math symbol fonts.\0".as_ptr()
                    as *const c_char;
                error();
                flush_math();
                danger = true;
            } else {
                let f3_text = MATH_FONT(3 + TEXT_SIZE as usize);
                let f3_script = MATH_FONT(3 + SCRIPT_SIZE as usize);
                let f3_sscript = MATH_FONT(3 + SCRIPT_SCRIPT_SIZE as usize);

                if ((*font_params.add(f3_text as usize) < TOTAL_MATHEX_PARAMS)
                    && !(*font_area.add(f3_text as usize) == OTGR_FONT_FLAG
                        && isOpenTypeMathFont(*font_layout_engine.add(f3_text as usize))))
                    || ((*font_params.add(f3_script as usize) < TOTAL_MATHEX_PARAMS)
                        && !(*font_area.add(f3_script as usize) == OTGR_FONT_FLAG
                            && isOpenTypeMathFont(*font_layout_engine.add(f3_script as usize))))
                    || ((*font_params.add(f3_sscript as usize) < TOTAL_MATHEX_PARAMS)
                        && !(*font_area.add(f3_sscript as usize) == OTGR_FONT_FLAG
                            && isOpenTypeMathFont(*font_layout_engine.add(f3_sscript as usize))))
                {
                    error_here_with_diagnostic(
                        b"Math formula deleted: Insufficient extension fonts\0".as_ptr()
                            as *const c_char,
                    );
                    capture_to_diagnostic(core::ptr::null_mut());
                    help_ptr = 3;
                    help_line[2] = b"Sorry, but I can't typeset math unless \\textfont 3\0".as_ptr()
                        as *const c_char;
                    help_line[1] = b"and \\scriptfont 3 and \\scriptscriptfont 3 have all\0"
                        .as_ptr() as *const c_char;
                    help_line[0] = b"the \\fontdimen values needed in math extension fonts.\0"
                        .as_ptr() as *const c_char;
                    error();
                    flush_math();
                    danger = true;
                }
            }
            m = cur_list.mode as i32;
            p = fin_mlist(TEX_NULL);
        } else {
            a = TEX_NULL;
        }

        if m < 0 {
            (*mem.add(cur_list.tail as usize)).b32.s1 =
                new_math(DIMENPAR(DIMEN_PAR__math_surround), BEFORE);
            cur_list.tail = LLIST_link(cur_list.tail);

            cur_mlist = p;
            cur_style = TEXT_STYLE;
            mlist_penalties = cur_list.mode > 0;
            mlist_to_hlist();
            (*mem.add(cur_list.tail as usize)).b32.s1 = (*mem.add(TEMP_HEAD as usize)).b32.s1;
            while (*mem.add(cur_list.tail as usize)).b32.s1 != TEX_NULL {
                cur_list.tail = LLIST_link(cur_list.tail);
            }

            (*mem.add(cur_list.tail as usize)).b32.s1 =
                new_math(DIMENPAR(DIMEN_PAR__math_surround), AFTER);
            cur_list.tail = LLIST_link(cur_list.tail);

            cur_list.aux.b32.s0 = 1000;

            if semantic_pagination_enabled {
                tt_insert_special(b"tdux:ce math\0".as_ptr() as *const c_char);
            }

            unsave();
        } else {
            if a == TEX_NULL {
                get_x_token();
                if cur_cmd != MATH_SHIFT {
                    error_here_with_diagnostic(
                        b"Display math should end with $$\0".as_ptr() as *const c_char
                    );
                    capture_to_diagnostic(core::ptr::null_mut());
                    help_ptr = 2;
                    help_line[1] = b"The `$' that I just saw supposedly matches a previous `$$'.\0"
                        .as_ptr() as *const c_char;
                    help_line[0] = b"So I shall assume that you typed `$$' both times.\0".as_ptr()
                        as *const c_char;
                    back_error();
                }
            }
            cur_mlist = p;
            cur_style = DISPLAY_STYLE;
            mlist_penalties = false;
            mlist_to_hlist();
            p = (*mem.add(TEMP_HEAD as usize)).b32.s1;
            adjust_tail = ADJUST_HEAD;
            pre_adjust_tail = PRE_ADJUST_HEAD;
            b = hpack(p, 0, ADDITIONAL);
            p = (*mem.add((b + 5) as usize)).b32.s1;
            t = adjust_tail;
            adjust_tail = TEX_NULL;
            pre_t = pre_adjust_tail;
            pre_adjust_tail = TEX_NULL;
            w = (*mem.add((b + 1) as usize)).b32.s1;
            z = DIMENPAR(DIMEN_PAR__display_width);
            s = DIMENPAR(DIMEN_PAR__display_indent);
            if INTPAR(INT_PAR__pre_display_direction) < 0 {
                s = -s - z;
            }
            if a == TEX_NULL || danger {
                e = 0;
                q = 0;
            } else {
                e = (*mem.add((a + 1) as usize)).b32.s1;
                q = e + math_quad(TEXT_SIZE);
            }
            if w + q > z {
                if e != 0
                    && ((w - total_shrink[NORMAL as usize] + q <= z)
                        || total_shrink[FIL] != 0
                        || total_shrink[FILL] != 0
                        || total_shrink[FILLL] != 0)
                {
                    free_node(b, BOX_NODE_SIZE);
                    b = hpack(p, z - q, EXACTLY);
                } else {
                    e = 0;
                    if w > z {
                        free_node(b, BOX_NODE_SIZE);
                        b = hpack(p, z, EXACTLY);
                    }
                }
                w = (*mem.add((b + 1) as usize)).b32.s1;
            }
            (*mem.add(b as usize)).b16.s0 = DLIST;
            d = half(z - w);
            if e > 0 && d < 2 * e {
                d = half(z - w - e);
                if p != TEX_NULL {
                    if !is_char_node(p) {
                        if NODE_type(p) == GLUE_NODE {
                            d = 0;
                        }
                    }
                }
            }

            (*mem.add(cur_list.tail as usize)).b32.s1 =
                new_penalty(INTPAR(INT_PAR__pre_display_penalty));
            cur_list.tail = LLIST_link(cur_list.tail);

            if d + s <= DIMENPAR(DIMEN_PAR__pre_display_size) || l {
                g1 = GLUE_PAR__above_display_skip;
                g2 = GLUE_PAR__below_display_skip;
            } else {
                g1 = GLUE_PAR__above_display_short_skip;
                g2 = GLUE_PAR__below_display_short_skip;
            }

            if l && e == 0 {
                app_display(j, a, 0);
                (*mem.add(cur_list.tail as usize)).b32.s1 = new_penalty(INF_PENALTY);
                cur_list.tail = LLIST_link(cur_list.tail);
            } else {
                (*mem.add(cur_list.tail as usize)).b32.s1 = new_param_glue(g1);
                cur_list.tail = LLIST_link(cur_list.tail);
            }

            if e != 0 {
                r = new_kern(z - w - e - d);
                if l {
                    (*mem.add(a as usize)).b32.s1 = r;
                    (*mem.add(r as usize)).b32.s1 = b;
                    b = a;
                    d = 0;
                } else {
                    (*mem.add(b as usize)).b32.s1 = r;
                    (*mem.add(r as usize)).b32.s1 = a;
                }
                b = hpack(b, 0, ADDITIONAL);
            }
            app_display(j, b, d);

            if a != TEX_NULL && e == 0 && !l {
                (*mem.add(cur_list.tail as usize)).b32.s1 = new_penalty(INF_PENALTY);
                cur_list.tail = LLIST_link(cur_list.tail);
                app_display(j, a, z - (*mem.add((a + 1) as usize)).b32.s1);
                g2 = 0;
            }

            if t != ADJUST_HEAD {
                (*mem.add(cur_list.tail as usize)).b32.s1 = (*mem.add(ADJUST_HEAD as usize)).b32.s1;
                cur_list.tail = t;
            }
            if pre_t != PRE_ADJUST_HEAD {
                (*mem.add(cur_list.tail as usize)).b32.s1 =
                    (*mem.add(PRE_ADJUST_HEAD as usize)).b32.s1;
                cur_list.tail = pre_t;
            }

            (*mem.add(cur_list.tail as usize)).b32.s1 =
                new_penalty(INTPAR(INT_PAR__post_display_penalty));
            cur_list.tail = LLIST_link(cur_list.tail);

            if g2 > 0 {
                (*mem.add(cur_list.tail as usize)).b32.s1 = new_param_glue(g2);
                cur_list.tail = LLIST_link(cur_list.tail);
            }
            flush_node_list(j);

            if semantic_pagination_enabled {
                tt_insert_special(b"tdux:ce dmath\0".as_ptr() as *const c_char);
            }

            resume_after_display();
        }
    }
}

#[unsafe(export_name = "resume_after_display")]
pub unsafe extern "C" fn resume_after_display() {
    unsafe {
        if cur_group != MATH_SHIFT_GROUP {
            confusion(b"display\0".as_ptr() as *const c_char);
        }

        unsave();
        cur_list.prev_graf += 3;
        push_nest();
        cur_list.mode = HMODE;
        cur_list.aux.b32.s0 = 1000;
        if INTPAR(INT_PAR__language) <= 0 {
            cur_lang = 0;
        } else if INTPAR(INT_PAR__language) > BIGGEST_LANG {
            cur_lang = 0;
        } else {
            cur_lang = INTPAR(INT_PAR__language) as u8;
        }
        cur_list.aux.b32.s1 = cur_lang as i32;
        cur_list.prev_graf = (i32::from(norm_min(INTPAR(INT_PAR__left_hyphen_min))) * 64
            + i32::from(norm_min(INTPAR(INT_PAR__right_hyphen_min))))
            * 65536
            + cur_lang as i32;

        get_x_token();
        if cur_cmd != SPACER {
            back_input();
        }

        if nest_ptr == 1 {
            build_page();
        }
    }
}

unsafe fn math_x_height(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 5)
        } else {
            (*font_info.add((5 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn math_quad(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 6)
        } else {
            (*font_info.add((6 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn num1(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 8)
        } else {
            (*font_info.add((8 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn num2(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 9)
        } else {
            (*font_info.add((9 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn num3(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 10)
        } else {
            (*font_info.add((10 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn denom1(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 11)
        } else {
            (*font_info.add((11 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn denom2(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 12)
        } else {
            (*font_info.add((12 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn sup1(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 13)
        } else {
            (*font_info.add((13 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn sup2(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 14)
        } else {
            (*font_info.add((14 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn sup3(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 15)
        } else {
            (*font_info.add((15 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn sub1(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 16)
        } else {
            (*font_info.add((16 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn sub2(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 17)
        } else {
            (*font_info.add((17 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn sup_drop(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 18)
        } else {
            (*font_info.add((18 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn sub_drop(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 19)
        } else {
            (*font_info.add((19 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn delim1(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 20)
        } else {
            (*font_info.add((20 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn delim2(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 21)
        } else {
            (*font_info.add((21 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn axis_height(size_code: i32) -> scaled_t {
    unsafe {
        let f = MATH_FONT((2 + size_code) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathsy_param(f, 22)
        } else {
            (*font_info.add((22 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn default_rule_thickness() -> scaled_t {
    unsafe {
        let f = MATH_FONT((3 + cur_size) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathex_param(f, 8)
        } else {
            (*font_info.add((8 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn big_op_spacing1() -> scaled_t {
    unsafe {
        let f = MATH_FONT((3 + cur_size) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathex_param(f, 9)
        } else {
            (*font_info.add((9 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn big_op_spacing2() -> scaled_t {
    unsafe {
        let f = MATH_FONT((3 + cur_size) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathex_param(f, 10)
        } else {
            (*font_info.add((10 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn big_op_spacing3() -> scaled_t {
    unsafe {
        let f = MATH_FONT((3 + cur_size) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathex_param(f, 11)
        } else {
            (*font_info.add((11 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn big_op_spacing4() -> scaled_t {
    unsafe {
        let f = MATH_FONT((3 + cur_size) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathex_param(f, 12)
        } else {
            (*font_info.add((12 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn big_op_spacing5() -> scaled_t {
    unsafe {
        let f = MATH_FONT((3 + cur_size) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            get_native_mathex_param(f, 13)
        } else {
            (*font_info.add((13 + *param_base.add(f as usize)) as usize))
                .b32
                .s1
        }
    }
}

unsafe fn fraction_rule(t: scaled_t) -> i32 {
    unsafe {
        let p = new_rule();
        (*mem.add((p + 3) as usize)).b32.s1 = t;
        (*mem.add((p + 2) as usize)).b32.s1 = 0;
        p
    }
}

unsafe fn overbar(b: i32, k: scaled_t, t: scaled_t) -> i32 {
    unsafe {
        let mut p = new_kern(k);
        (*mem.add(p as usize)).b32.s1 = b;
        let q = fraction_rule(t);
        (*mem.add(q as usize)).b32.s1 = p;
        p = new_kern(t);
        (*mem.add(p as usize)).b32.s1 = q;
        vpackage(p, 0, ADDITIONAL, MAX_HALFWORD)
    }
}

unsafe fn math_glue(g: i32, m: scaled_t) -> i32 {
    unsafe {
        let mut n = x_over_n(m, 65536);
        let mut f = tex_remainder;
        if f < 0 {
            n -= 1;
            f += 65536;
        }
        let p = get_node(GLUE_SPEC_SIZE);
        (*mem.add((p + 1) as usize)).b32.s1 = mult_and_add(
            n,
            (*mem.add((g + 1) as usize)).b32.s1,
            xn_over_d((*mem.add((g + 1) as usize)).b32.s1, f, 65536),
            MAX_HALFWORD,
        );
        (*mem.add(p as usize)).b16.s1 = (*mem.add(g as usize)).b16.s1;
        if (*mem.add(p as usize)).b16.s1 == NORMAL as u16 {
            (*mem.add((p + 2) as usize)).b32.s1 = mult_and_add(
                n,
                (*mem.add((g + 2) as usize)).b32.s1,
                xn_over_d((*mem.add((g + 2) as usize)).b32.s1, f, 65536),
                MAX_HALFWORD,
            );
        } else {
            (*mem.add((p + 2) as usize)).b32.s1 = (*mem.add((g + 2) as usize)).b32.s1;
        }
        (*mem.add(p as usize)).b16.s0 = (*mem.add(g as usize)).b16.s0;
        if GLUE_SPEC_shrink_order(p) == NORMAL {
            (*mem.add((p + 3) as usize)).b32.s1 = mult_and_add(
                n,
                (*mem.add((g + 3) as usize)).b32.s1,
                xn_over_d((*mem.add((g + 3) as usize)).b32.s1, f, 65536),
                MAX_HALFWORD,
            );
        } else {
            (*mem.add((p + 3) as usize)).b32.s1 = (*mem.add((g + 3) as usize)).b32.s1;
        }
        p
    }
}

unsafe fn math_kern(p: i32, m: scaled_t) {
    unsafe {
        if (*mem.add(p as usize)).b16.s0 == MU_GLUE {
            let mut n = x_over_n(m, 65536);
            let mut f = tex_remainder;
            if f < 0 {
                n -= 1;
                f += 65536;
            }
            (*mem.add((p + 1) as usize)).b32.s1 = mult_and_add(
                n,
                (*mem.add((p + 1) as usize)).b32.s1,
                xn_over_d((*mem.add((p + 1) as usize)).b32.s1, f, 65536),
                MAX_HALFWORD,
            );
            set_NODE_subtype(p, EXPLICIT as i16);
        }
    }
}

#[unsafe(export_name = "flush_math")]
pub unsafe extern "C" fn flush_math() {
    unsafe {
        flush_node_list((*mem.add(cur_list.head as usize)).b32.s1);
        flush_node_list(cur_list.aux.b32.s1);
        (*mem.add(cur_list.head as usize)).b32.s1 = TEX_NULL;
        cur_list.tail = cur_list.head;
        cur_list.aux.b32.s1 = TEX_NULL;
    }
}

unsafe fn clean_box(p: i32, s: small_number) -> i32 {
    unsafe {
        let q: i32;
        let mut save_style: small_number;
        let x: i32;
        let r: i32;

        match (*mem.add(p as usize)).b32.s1 {
            1 => {
                cur_mlist = new_noad();
                *mem.add((cur_mlist + 1) as usize) = *mem.add(p as usize);
                save_style = cur_style;
                cur_style = s;
                mlist_penalties = false;
                mlist_to_hlist();
                q = (*mem.add(TEMP_HEAD as usize)).b32.s1;
                cur_style = save_style;
                if cur_style < SCRIPT_STYLE {
                    cur_size = TEXT_SIZE;
                } else {
                    cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                }
                cur_mu = x_over_n(math_quad(cur_size), 18);
            }
            2 => {
                q = (*mem.add(p as usize)).b32.s0;
            }
            3 => {
                cur_mlist = (*mem.add(p as usize)).b32.s0;
                save_style = cur_style;
                cur_style = s;
                mlist_penalties = false;
                mlist_to_hlist();
                q = (*mem.add(TEMP_HEAD as usize)).b32.s1;
                cur_style = save_style;
                if cur_style < SCRIPT_STYLE {
                    cur_size = TEXT_SIZE;
                } else {
                    cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                }
                cur_mu = x_over_n(math_quad(cur_size), 18);
            }
            _ => {
                q = new_null_box();
            }
        }

        if is_char_node(q) || q == TEX_NULL {
            x = hpack(q, 0, ADDITIONAL);
        } else if (*mem.add(q as usize)).b32.s1 == TEX_NULL
            && NODE_type(q) <= VLIST_NODE
            && (*mem.add((q + 4) as usize)).b32.s1 == 0
        {
            x = q;
        } else {
            x = hpack(q, 0, ADDITIONAL);
        }

        let q_box = (*mem.add((x + 5) as usize)).b32.s1;
        if is_char_node(q_box) {
            let r_node = (*mem.add(q_box as usize)).b32.s1;
            if r_node != TEX_NULL {
                if (*mem.add(r_node as usize)).b32.s1 == TEX_NULL {
                    if !is_char_node(r_node) {
                        if NODE_type(r_node) == KERN_NODE {
                            free_node(r_node, MEDIUM_NODE_SIZE);
                            (*mem.add(q_box as usize)).b32.s1 = TEX_NULL;
                        }
                    }
                }
            }
        }
        x
    }
}

unsafe fn fetch(a: i32) {
    unsafe {
        cur_c = (*mem.add(a as usize)).b16.s0 as i32;
        cur_f = MATH_FONT((((*mem.add(a as usize)).b16.s1 % 256) as i32 + cur_size) as usize);
        cur_c += (((*mem.add(a as usize)).b16.s1 / 256) as i32) * 65536;

        if cur_f == FONT_BASE {
            error_here_with_diagnostic(b"\0".as_ptr() as *const c_char);
            print_size(cur_size);
            print_char(' ' as i32);
            print_int(((*mem.add(a as usize)).b16.s1 % 256) as i32);
            print_cstr(b" is undefined (character \0".as_ptr() as *const c_char);
            print(cur_c);
            print_char(')' as i32);
            capture_to_diagnostic(core::ptr::null_mut());
            help_ptr = 4;
            help_line[3] = b"Somewhere in the math formula just ended, you used the\0".as_ptr()
                as *const c_char;
            help_line[2] = b"stated character from an undefined font family. For example,\0"
                .as_ptr() as *const c_char;
            help_line[1] = b"plain TeX doesn't allow \\it or \\sl in subscripts. Proceed,\0"
                .as_ptr() as *const c_char;
            help_line[0] =
                b"and I'll try to forget that I needed that character.\0".as_ptr() as *const c_char;
            error();
            cur_i = null_character;
            (*mem.add(a as usize)).b32.s1 = EMPTY;
        } else if *font_area.add(cur_f as usize) == AAT_FONT_FLAG
            || *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
        {
            cur_i = null_character;
        } else {
            if cur_c >= i32::from(*font_bc.add(cur_f as usize))
                && cur_c <= i32::from(*font_ec.add(cur_f as usize))
            {
                cur_i = FONT_CHARACTER_INFO(cur_f, cur_c);
            } else {
                cur_i = null_character;
            }
            if cur_i.s3 == 0 {
                char_warning(cur_f, cur_c);
                (*mem.add(a as usize)).b32.s1 = EMPTY;
                cur_i = null_character;
            }
        }
    }
}

unsafe fn make_over(q: i32) {
    unsafe {
        (*mem.add((q + 1) as usize)).b32.s0 = overbar(
            clean_box(q + 1, 2 * (cur_style / 2) + 1),
            3 * default_rule_thickness(),
            default_rule_thickness(),
        );
        (*mem.add((q + 1) as usize)).b32.s1 = SUB_BOX;
    }
}

unsafe fn make_under(q: i32) {
    unsafe {
        let x = clean_box(q + 1, cur_style);
        let p = new_kern(3 * default_rule_thickness());
        (*mem.add(x as usize)).b32.s1 = p;
        (*mem.add(p as usize)).b32.s1 = fraction_rule(default_rule_thickness());
        let y = vpackage(x, 0, ADDITIONAL, MAX_HALFWORD);
        let delta = (*mem.add((y + 3) as usize)).b32.s1
            + (*mem.add((y + 2) as usize)).b32.s1
            + default_rule_thickness();
        (*mem.add((y + 3) as usize)).b32.s1 = (*mem.add((x + 3) as usize)).b32.s1;
        (*mem.add((y + 2) as usize)).b32.s1 = delta - (*mem.add((y + 3) as usize)).b32.s1;
        (*mem.add((q + 1) as usize)).b32.s0 = y;
        (*mem.add((q + 1) as usize)).b32.s1 = SUB_BOX;
    }
}

unsafe fn make_vcenter(q: i32) {
    unsafe {
        let v = (*mem.add((q + 1) as usize)).b32.s0;
        if NODE_type(v) != VLIST_NODE {
            confusion(b"vcenter\0".as_ptr() as *const c_char);
        }
        let delta = (*mem.add((v + 3) as usize)).b32.s1 + (*mem.add((v + 2) as usize)).b32.s1;
        (*mem.add((v + 3) as usize)).b32.s1 = axis_height(cur_size) + half(delta);
        (*mem.add((v + 2) as usize)).b32.s1 = delta - (*mem.add((v + 3) as usize)).b32.s1;
    }
}

unsafe fn make_radical(q: i32) {
    unsafe {
        let x: i32;
        let mut y: i32;
        let f: internal_font_number;
        let rule_thickness: scaled_t;
        let delta: scaled_t;
        let mut clr: scaled_t;

        f = MATH_FONT((((*mem.add((q + 4) as usize)).b16.s3 % 256) as i32 + cur_size) as usize);
        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            rule_thickness = get_ot_math_constant(f, RADICALRULETHICKNESS);
        } else {
            rule_thickness = default_rule_thickness();
        }

        x = clean_box(q + 1, 2 * (cur_style / 2) + 1);

        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            if cur_style < TEXT_STYLE {
                clr = get_ot_math_constant(f, RADICALDISPLAYSTYLEVERTICALGAP);
            } else {
                clr = get_ot_math_constant(f, RADICALVERTICALGAP);
            }
        } else if cur_style < TEXT_STYLE {
            clr = rule_thickness + (math_x_height(cur_size).abs() / 4);
        } else {
            clr = rule_thickness;
            clr += clr.abs() / 4;
        }

        y = var_delimiter(
            q + 4,
            cur_size,
            (*mem.add((x + 3) as usize)).b32.s1
                + (*mem.add((x + 2) as usize)).b32.s1
                + clr
                + rule_thickness,
        );

        if *font_area.add(f as usize) == OTGR_FONT_FLAG
            && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
        {
            (*mem.add((y + 2) as usize)).b32.s1 = (*mem.add((y + 3) as usize)).b32.s1
                + (*mem.add((y + 2) as usize)).b32.s1
                - rule_thickness;
            (*mem.add((y + 3) as usize)).b32.s1 = rule_thickness;
        }

        delta = (*mem.add((y + 2) as usize)).b32.s1
            - ((*mem.add((x + 3) as usize)).b32.s1 + (*mem.add((x + 2) as usize)).b32.s1 + clr);
        if delta > 0 {
            clr += half(delta);
        }
        (*mem.add((y + 4) as usize)).b32.s1 = -((*mem.add((x + 3) as usize)).b32.s1 + clr);
        (*mem.add(y as usize)).b32.s1 = overbar(x, clr, (*mem.add((y + 3) as usize)).b32.s1);
        (*mem.add((q + 1) as usize)).b32.s0 = hpack(y, 0, ADDITIONAL);
        (*mem.add((q + 1) as usize)).b32.s1 = SUB_BOX;
    }
}

unsafe fn compute_ot_math_accent_pos(p: i32) -> scaled_t {
    unsafe {
        let s: scaled_t;
        if (*mem.add((p + 1) as usize)).b32.s1 == MATH_CHAR {
            fetch(p + 1);
            let q = new_native_character(cur_f, cur_c);
            let g = get_native_glyph(q, 0);
            s = get_ot_math_accent_pos(cur_f, g);
        } else if (*mem.add((p + 1) as usize)).b32.s1 == SUB_MLIST {
            let r = (*mem.add((p + 1) as usize)).b32.s0;
            if r != TEX_NULL && (*mem.add(r as usize)).b16.s1 == ACCENT_NOAD as u16 {
                s = compute_ot_math_accent_pos(r);
            } else {
                s = TEX_INFINITY;
            }
        } else {
            s = TEX_INFINITY;
        }
        s
    }
}

unsafe fn make_math_accent(q: i32) {
    unsafe {
        let mut p: i32;
        let mut x: i32;
        let mut y: i32;
        let mut a: i32;
        let mut c: i32;
        let mut g: i32;
        let f: internal_font_number;
        let mut i: b16x4;
        let mut s: scaled_t;
        let mut sa: scaled_t;
        let mut h: scaled_t;
        let mut delta: scaled_t;
        let mut w: scaled_t;
        let mut w2: scaled_t = 0;
        let mut ot_assembly_ptr: *mut core::ffi::c_void = core::ptr::null_mut();

        fetch(q + 4);
        x = TEX_NULL;

        if *font_area.add(cur_f as usize) == AAT_FONT_FLAG
            || *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
        {
            c = cur_c;
            f = cur_f;
            if !((*mem.add(q as usize)).b16.s0 == BOTTOM_ACC
                || (*mem.add(q as usize)).b16.s0 == (BOTTOM_ACC + 1))
            {
                s = compute_ot_math_accent_pos(q);
            } else {
                s = 0;
            }
            x = clean_box(q + 1, 2 * (cur_style / 2) + 1);
            w = (*mem.add((x + 1) as usize)).b32.s1;
            h = (*mem.add((x + 3) as usize)).b32.s1;
        } else if cur_i.s3 > 0 {
            i = cur_i;
            c = cur_c;
            f = cur_f;
            s = 0;
            if (*mem.add((q + 1) as usize)).b32.s1 == MATH_CHAR {
                fetch(q + 1);
                if (cur_i.s1 % 4) == LIG_TAG {
                    a = *lig_kern_base.add(cur_f as usize) + cur_i.s0 as i32;
                    cur_i = (*font_info.add(a as usize)).b16;
                    if cur_i.s3 > 128 {
                        a = *lig_kern_base.add(cur_f as usize)
                            + 256 * cur_i.s1 as i32
                            + cur_i.s0 as i32
                            + 32768
                            - 256 * 128;
                        cur_i = (*font_info.add(a as usize)).b16;
                    }
                    'lig_loop: loop {
                        if cur_i.s2 as i32 == *skew_char.add(cur_f as usize) {
                            if cur_i.s1 >= 128 {
                                if cur_i.s3 <= 128 {
                                    s = (*font_info.add(
                                        (*kern_base.add(cur_f as usize)
                                            + 256 * cur_i.s1 as i32
                                            + cur_i.s0 as i32)
                                            as usize,
                                    ))
                                    .b32
                                    .s1;
                                }
                            }
                            break 'lig_loop;
                        }
                        if cur_i.s3 >= 128 {
                            break 'lig_loop;
                        }
                        a = a + cur_i.s3 as i32 + 1;
                        cur_i = (*font_info.add(a as usize)).b16;
                    }
                }
            }

            x = clean_box(q + 1, 2 * (cur_style / 2) + 1);
            w = (*mem.add((x + 1) as usize)).b32.s1;
            h = (*mem.add((x + 3) as usize)).b32.s1;

            loop {
                if (i.s1 % 4) != LIST_TAG {
                    break;
                }
                y = i.s0 as i32;
                i = FONT_CHARACTER_INFO(f, y);
                if i.s3 == 0 {
                    break;
                }
                if FONT_CHARINFO_WIDTH(f, i) > w {
                    break;
                }
                c = y;
            }
        } else {
            f = cur_f;
            c = cur_c;
            s = 0;
            w = 0;
            h = 0;
        }

        if x != TEX_NULL {
            if *font_area.add(f as usize) == OTGR_FONT_FLAG
                && isOpenTypeMathFont(*font_layout_engine.add(f as usize))
            {
                if (*mem.add(q as usize)).b16.s0 == BOTTOM_ACC
                    || (*mem.add(q as usize)).b16.s0 == (BOTTOM_ACC + 1)
                {
                    delta = 0;
                } else if h < get_ot_math_constant(f, ACCENTBASEHEIGHT) {
                    delta = h;
                } else {
                    delta = get_ot_math_constant(f, ACCENTBASEHEIGHT);
                }
            } else if h
                < (*font_info.add((X_HEIGHT_CODE + *param_base.add(f as usize) as usize) as usize))
                    .b32
                    .s1
            {
                delta = h;
            } else {
                delta = (*font_info
                    .add((X_HEIGHT_CODE + *param_base.add(f as usize) as usize) as usize))
                .b32
                .s1;
            }

            if (*mem.add((q + 2) as usize)).b32.s1 != EMPTY
                || (*mem.add((q + 3) as usize)).b32.s1 != EMPTY
            {
                if (*mem.add((q + 1) as usize)).b32.s1 == MATH_CHAR {
                    flush_node_list(x);
                    x = new_noad();
                    *mem.add((x + 1) as usize) = *mem.add((q + 1) as usize);
                    *mem.add((x + 2) as usize) = *mem.add((q + 2) as usize);
                    *mem.add((x + 3) as usize) = *mem.add((q + 3) as usize);
                    (*mem.add((q + 2) as usize)).b32 = empty;
                    (*mem.add((q + 3) as usize)).b32 = empty;
                    (*mem.add((q + 1) as usize)).b32.s1 = SUB_MLIST;
                    (*mem.add((q + 1) as usize)).b32.s0 = x;
                    x = clean_box(q + 1, cur_style);
                    delta = delta + (*mem.add((x + 3) as usize)).b32.s1 - h;
                    h = (*mem.add((x + 3) as usize)).b32.s1;
                }
            }

            y = char_box(f, c);

            if *font_area.add(f as usize) == AAT_FONT_FLAG
                || *font_area.add(f as usize) == OTGR_FONT_FLAG
            {
                p = get_node(GLYPH_NODE_SIZE);
                set_NODE_type(p, WHATSIT_NODE);
                (*mem.add(p as usize)).b16.s0 = GLYPH_NODE;
                (*mem.add((p + 4) as usize)).b16.s2 = f as u16;
                (*mem.add((p + 4) as usize)).b16.s1 =
                    get_native_glyph((*mem.add((y + 5) as usize)).b32.s1, 0) as u16;
                set_native_glyph_metrics(p, 1);
                free_node(
                    (*mem.add((y + 5) as usize)).b32.s1,
                    (*mem.add(((*mem.add((y + 5) as usize)).b32.s1 + 4) as usize))
                        .b16
                        .s3 as i32,
                );
                (*mem.add((y + 5) as usize)).b32.s1 = p;

                if odd((*mem.add(q as usize)).b16.s0 as i32) {
                    set_native_glyph_metrics(p, 1);
                } else {
                    c = (*mem.add((p + 4) as usize)).b16.s1 as i32;
                    a = 0;
                    loop {
                        g = get_ot_math_variant(f, c, a, &mut w2, 1);
                        if w2 > 0 && w2 <= w {
                            (*mem.add((p + 4) as usize)).b16.s1 = g as u16;
                            set_native_glyph_metrics(p, 1);
                            a += 1;
                        }
                        if w2 < 0 || w2 >= w {
                            break;
                        }
                    }
                    if w2 < 0 {
                        ot_assembly_ptr = get_ot_assembly_ptr(f, c, 1);
                        if !ot_assembly_ptr.is_null() {
                            free_node(p, GLYPH_NODE_SIZE);
                            p = build_opentype_assembly(f, ot_assembly_ptr, w, true);
                            (*mem.add((y + 5) as usize)).b32.s1 = p;
                        } else {
                            set_native_glyph_metrics(p, 1);
                        }
                    } else {
                        set_native_glyph_metrics(p, 1);
                    }
                }

                (*mem.add((y + 1) as usize)).b32.s1 = (*mem.add((p + 1) as usize)).b32.s1;
                (*mem.add((y + 3) as usize)).b32.s1 = (*mem.add((p + 3) as usize)).b32.s1;
                (*mem.add((y + 2) as usize)).b32.s1 = (*mem.add((p + 2) as usize)).b32.s1;

                if (*mem.add(q as usize)).b16.s0 == BOTTOM_ACC
                    || (*mem.add(q as usize)).b16.s0 == (BOTTOM_ACC + 1)
                {
                    if (*mem.add((y + 3) as usize)).b32.s1 < 0 {
                        (*mem.add((y + 3) as usize)).b32.s1 = 0;
                    }
                } else if (*mem.add((y + 2) as usize)).b32.s1 < 0 {
                    (*mem.add((y + 2) as usize)).b32.s1 = 0;
                }

                if p != TEX_NULL
                    && !is_char_node(p)
                    && NODE_type(p) == WHATSIT_NODE
                    && (*mem.add(p as usize)).b16.s0 == GLYPH_NODE
                {
                    sa = get_ot_math_accent_pos(f, (*mem.add((p + 4) as usize)).b16.s1 as i32);
                    if sa == TEX_INFINITY {
                        sa = half((*mem.add((y + 1) as usize)).b32.s1);
                    }
                } else {
                    sa = half((*mem.add((y + 1) as usize)).b32.s1);
                }

                if (*mem.add(q as usize)).b16.s0 == BOTTOM_ACC
                    || (*mem.add(q as usize)).b16.s0 == (BOTTOM_ACC + 1)
                    || s == TEX_INFINITY
                {
                    s = half(w);
                }
                (*mem.add((y + 4) as usize)).b32.s1 = s - sa;
            } else {
                (*mem.add((y + 4) as usize)).b32.s1 =
                    s + half(w - (*mem.add((y + 1) as usize)).b32.s1);
            }

            (*mem.add((y + 1) as usize)).b32.s1 = 0;

            if (*mem.add(q as usize)).b16.s0 == BOTTOM_ACC
                || (*mem.add(q as usize)).b16.s0 == (BOTTOM_ACC + 1)
            {
                (*mem.add(x as usize)).b32.s1 = y;
                y = vpackage(x, 0, ADDITIONAL, MAX_HALFWORD);
                (*mem.add((y + 4) as usize)).b32.s1 = -(h - (*mem.add((y + 3) as usize)).b32.s1);
            } else {
                p = new_kern(-delta);
                (*mem.add(p as usize)).b32.s1 = x;
                (*mem.add(y as usize)).b32.s1 = p;
                y = vpackage(y, 0, ADDITIONAL, MAX_HALFWORD);
                if (*mem.add((y + 3) as usize)).b32.s1 < h {
                    p = new_kern(h - (*mem.add((y + 3) as usize)).b32.s1);
                    (*mem.add(p as usize)).b32.s1 = (*mem.add((y + 5) as usize)).b32.s1;
                    (*mem.add((y + 5) as usize)).b32.s1 = p;
                    (*mem.add((y + 3) as usize)).b32.s1 = h;
                }
            }

            (*mem.add((y + 1) as usize)).b32.s1 = (*mem.add((x + 1) as usize)).b32.s1;
            (*mem.add((q + 1) as usize)).b32.s0 = y;
            (*mem.add((q + 1) as usize)).b32.s1 = SUB_BOX;
        }

        free_ot_assembly(ot_assembly_ptr);
    }
}

unsafe fn make_fraction(q: i32) {
    unsafe {
        let p: i32;
        let v: i32;
        let mut x: i32;
        let mut y: i32;
        let mut z: i32;
        let mut delta: scaled_t;
        let mut delta1: scaled_t;
        let mut delta2: scaled_t;
        let mut shift_up: scaled_t;
        let mut shift_down: scaled_t;
        let mut clr: scaled_t;

        if (*mem.add((q + 1) as usize)).b32.s1 == DEFAULT_CODE {
            (*mem.add((q + 1) as usize)).b32.s1 = default_rule_thickness();
        }

        x = clean_box(q + 2, cur_style + 2 - 2 * (cur_style / 6));
        z = clean_box(q + 3, 2 * (cur_style / 2) + 3 - 2 * (cur_style / 6));

        if (*mem.add((x + 1) as usize)).b32.s1 < (*mem.add((z + 1) as usize)).b32.s1 {
            x = rebox(x, (*mem.add((z + 1) as usize)).b32.s1);
        } else {
            z = rebox(z, (*mem.add((x + 1) as usize)).b32.s1);
        }

        if cur_style < TEXT_STYLE {
            shift_up = num1(cur_size);
            shift_down = denom1(cur_size);
        } else {
            shift_down = denom2(cur_size);
            if (*mem.add((q + 1) as usize)).b32.s1 != 0 {
                shift_up = num2(cur_size);
            } else {
                shift_up = num3(cur_size);
            }
        }

        if (*mem.add((q + 1) as usize)).b32.s1 == 0 {
            if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
            {
                if cur_style < TEXT_STYLE {
                    clr = get_ot_math_constant(cur_f, STACKDISPLAYSTYLEGAPMIN);
                } else {
                    clr = get_ot_math_constant(cur_f, STACKGAPMIN);
                }
            } else if cur_style < TEXT_STYLE {
                clr = 7 * default_rule_thickness();
            } else {
                clr = 3 * default_rule_thickness();
            }

            delta = half(
                clr - ((shift_up - (*mem.add((x + 2) as usize)).b32.s1)
                    - ((*mem.add((z + 3) as usize)).b32.s1 - shift_down)),
            );

            if delta > 0 {
                shift_up += delta;
                shift_down += delta;
            }
        } else {
            if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
            {
                delta = half((*mem.add((q + 1) as usize)).b32.s1);

                if cur_style < TEXT_STYLE {
                    clr = get_ot_math_constant(cur_f, FRACTIONNUMDISPLAYSTYLEGAPMIN);
                } else {
                    clr = get_ot_math_constant(cur_f, FRACTIONNUMERATORGAPMIN);
                }

                delta1 = clr
                    - ((shift_up - (*mem.add((x + 2) as usize)).b32.s1)
                        - (axis_height(cur_size) + delta));

                if cur_style < TEXT_STYLE {
                    clr = get_ot_math_constant(cur_f, FRACTIONDENOMDISPLAYSTYLEGAPMIN);
                } else {
                    clr = get_ot_math_constant(cur_f, FRACTIONDENOMINATORGAPMIN);
                }

                delta2 = clr
                    - ((axis_height(cur_size) - delta)
                        - ((*mem.add((z + 3) as usize)).b32.s1 - shift_down));
            } else {
                if cur_style < TEXT_STYLE {
                    clr = 3 * (*mem.add((q + 1) as usize)).b32.s1;
                } else {
                    clr = (*mem.add((q + 1) as usize)).b32.s1;
                }
                delta = half((*mem.add((q + 1) as usize)).b32.s1);
                delta1 = clr
                    - ((shift_up - (*mem.add((x + 2) as usize)).b32.s1)
                        - (axis_height(cur_size) + delta));
                delta2 = clr
                    - ((axis_height(cur_size) - delta)
                        - ((*mem.add((z + 3) as usize)).b32.s1 - shift_down));
            }

            if delta1 > 0 {
                shift_up += delta1;
            }
            if delta2 > 0 {
                shift_down += delta2;
            }
        }

        v = new_null_box();
        set_NODE_type(v, VLIST_NODE);
        (*mem.add((v + 3) as usize)).b32.s1 = shift_up + (*mem.add((x + 3) as usize)).b32.s1;
        (*mem.add((v + 2) as usize)).b32.s1 = (*mem.add((z + 2) as usize)).b32.s1 + shift_down;
        (*mem.add((v + 1) as usize)).b32.s1 = (*mem.add((x + 1) as usize)).b32.s1;

        let p_v: i32;
        if (*mem.add((q + 1) as usize)).b32.s1 == 0 {
            p_v = new_kern(
                (shift_up - (*mem.add((x + 2) as usize)).b32.s1)
                    - ((*mem.add((z + 3) as usize)).b32.s1 - shift_down),
            );
            (*mem.add(p_v as usize)).b32.s1 = z;
        } else {
            y = fraction_rule((*mem.add((q + 1) as usize)).b32.s1);
            let p1 = new_kern(
                (axis_height(cur_size) - delta)
                    - ((*mem.add((z + 3) as usize)).b32.s1 - shift_down),
            );
            (*mem.add(y as usize)).b32.s1 = p1;
            (*mem.add(p1 as usize)).b32.s1 = z;
            p_v = new_kern(
                (shift_up - (*mem.add((x + 2) as usize)).b32.s1) - (axis_height(cur_size) + delta),
            );
            (*mem.add(p_v as usize)).b32.s1 = y;
        }

        (*mem.add(x as usize)).b32.s1 = p_v;
        (*mem.add((v + 5) as usize)).b32.s1 = x;

        if cur_style < TEXT_STYLE {
            delta = delim1(cur_size);
        } else {
            delta = delim2(cur_size);
        }

        x = var_delimiter(q + 4, cur_size, delta);
        (*mem.add(x as usize)).b32.s1 = v;
        z = var_delimiter(q + 5, cur_size, delta);
        (*mem.add(v as usize)).b32.s1 = z;
        (*mem.add((q + 1) as usize)).b32.s1 = hpack(x, 0, ADDITIONAL);
    }
}

unsafe fn make_op(q: i32) -> scaled_t {
    unsafe {
        let mut delta: scaled_t;
        let mut p: i32;
        let v: i32;
        let mut x: i32;
        let mut y: i32;
        let mut z: i32;
        let mut c: u16;
        let mut i: b16x4;
        let mut shift_up: scaled_t;
        let mut shift_down: scaled_t;
        let mut h1: scaled_t;
        let mut h2: scaled_t = 0;
        let mut n: i32;
        let mut g: i32;
        let mut ot_assembly_ptr: *mut core::ffi::c_void = core::ptr::null_mut();
        let save_f: internal_font_number;

        if (*mem.add(q as usize)).b16.s0 == NORMAL as u16 && cur_style < TEXT_STYLE {
            (*mem.add(q as usize)).b16.s0 = LIMITS;
        }
        delta = 0;

        if (*mem.add((q + 1) as usize)).b32.s1 == MATH_CHAR {
            fetch(q + 1);
            if !(*font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                && usingOpenType(*font_layout_engine.add(cur_f as usize)))
            {
                if cur_style < TEXT_STYLE && (cur_i.s1 % 4) == LIST_TAG {
                    c = cur_i.s0;
                    i = FONT_CHARACTER_INFO(cur_f, c as i32);
                    if i.s3 > 0 {
                        cur_c = c as i32;
                        cur_i = i;
                        (*mem.add((q + 1) as usize)).b16.s0 = c;
                    }
                }
                delta = FONT_CHARINFO_ITALCORR(cur_f, cur_i);
            }
            x = clean_box(q + 1, cur_style);
            if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
            {
                p = (*mem.add((x + 5) as usize)).b32.s1;
                if p != TEX_NULL
                    && !is_char_node(p)
                    && NODE_type(p) == WHATSIT_NODE
                    && (*mem.add(p as usize)).b16.s0 == GLYPH_NODE
                {
                    let mut assembly_built = false;
                    if cur_style < TEXT_STYLE {
                        h1 = get_ot_math_constant(cur_f, DISPLAYOPERATORMINHEIGHT);
                        let five_fourths = (((*mem.add((p + 3) as usize)).b32.s1
                            + (*mem.add((p + 2) as usize)).b32.s1)
                            as f64)
                            * 5.0
                            / 4.0;
                        if (h1 as f64) < five_fourths {
                            h1 = five_fourths as scaled_t;
                        }
                        c = (*mem.add((p + 4) as usize)).b16.s1;
                        n = 0;
                        loop {
                            g = get_ot_math_variant(cur_f, c as i32, n, &mut h2, 0);
                            if h2 > 0 {
                                (*mem.add((p + 4) as usize)).b16.s1 = g as u16;
                                set_native_glyph_metrics(p, 1);
                            }
                            n += 1;
                            if h2 < 0 || h2 >= h1 {
                                break;
                            }
                        }
                        if h2 < 0 {
                            ot_assembly_ptr = get_ot_assembly_ptr(cur_f, c as i32, 0);
                            if !ot_assembly_ptr.is_null() {
                                free_node(p, GLYPH_NODE_SIZE);
                                p = build_opentype_assembly(cur_f, ot_assembly_ptr, h1, false);
                                (*mem.add((x + 5) as usize)).b32.s1 = p;
                                delta = 0;
                                assembly_built = true;
                            }
                        } else {
                            set_native_glyph_metrics(p, 1);
                        }
                    }
                    if !assembly_built {
                        delta = get_ot_math_ital_corr(
                            cur_f,
                            (*mem.add((p + 4) as usize)).b16.s1 as i32,
                        );
                    }
                    (*mem.add((x + 1) as usize)).b32.s1 = (*mem.add((p + 1) as usize)).b32.s1;
                    (*mem.add((x + 3) as usize)).b32.s1 = (*mem.add((p + 3) as usize)).b32.s1;
                    (*mem.add((x + 2) as usize)).b32.s1 = (*mem.add((p + 2) as usize)).b32.s1;
                }
            }
            if (*mem.add((q + 3) as usize)).b32.s1 != EMPTY
                && (*mem.add(q as usize)).b16.s0 != LIMITS
            {
                (*mem.add((x + 1) as usize)).b32.s1 -= delta;
            }
            (*mem.add((x + 4) as usize)).b32.s1 =
                half((*mem.add((x + 3) as usize)).b32.s1 - (*mem.add((x + 2) as usize)).b32.s1)
                    - axis_height(cur_size);
            (*mem.add((q + 1) as usize)).b32.s1 = SUB_BOX;
            (*mem.add((q + 1) as usize)).b32.s0 = x;
        }
        save_f = cur_f;
        if (*mem.add(q as usize)).b16.s0 == LIMITS {
            x = clean_box(q + 2, 2 * (cur_style / 4) + 4 + (cur_style % 2));
            y = clean_box(q + 1, cur_style);
            z = clean_box(q + 3, 2 * (cur_style / 4) + 5);
            v = new_null_box();
            set_NODE_type(v, VLIST_NODE);
            (*mem.add((v + 1) as usize)).b32.s1 = (*mem.add((y + 1) as usize)).b32.s1;
            if (*mem.add((x + 1) as usize)).b32.s1 > (*mem.add((v + 1) as usize)).b32.s1 {
                (*mem.add((v + 1) as usize)).b32.s1 = (*mem.add((x + 1) as usize)).b32.s1;
            }
            if (*mem.add((z + 1) as usize)).b32.s1 > (*mem.add((v + 1) as usize)).b32.s1 {
                (*mem.add((v + 1) as usize)).b32.s1 = (*mem.add((z + 1) as usize)).b32.s1;
            }
            x = rebox(x, (*mem.add((v + 1) as usize)).b32.s1);
            y = rebox(y, (*mem.add((v + 1) as usize)).b32.s1);
            z = rebox(z, (*mem.add((v + 1) as usize)).b32.s1);
            (*mem.add((x + 4) as usize)).b32.s1 = half(delta);
            (*mem.add((z + 4) as usize)).b32.s1 = -(*mem.add((x + 4) as usize)).b32.s1;
            (*mem.add((v + 3) as usize)).b32.s1 = (*mem.add((y + 3) as usize)).b32.s1;
            (*mem.add((v + 2) as usize)).b32.s1 = (*mem.add((y + 2) as usize)).b32.s1;
            cur_f = save_f;

            if (*mem.add((q + 2) as usize)).b32.s1 == EMPTY {
                free_node(x, BOX_NODE_SIZE);
                (*mem.add((v + 5) as usize)).b32.s1 = y;
            } else {
                shift_up = big_op_spacing3() - (*mem.add((x + 2) as usize)).b32.s1;
                if shift_up < big_op_spacing1() {
                    shift_up = big_op_spacing1();
                }
                p = new_kern(shift_up);
                (*mem.add(p as usize)).b32.s1 = y;
                (*mem.add(x as usize)).b32.s1 = p;
                p = new_kern(big_op_spacing5());
                (*mem.add(p as usize)).b32.s1 = x;
                (*mem.add((v + 5) as usize)).b32.s1 = p;
                (*mem.add((v + 3) as usize)).b32.s1 = (*mem.add((v + 3) as usize)).b32.s1
                    + big_op_spacing5()
                    + (*mem.add((x + 3) as usize)).b32.s1
                    + (*mem.add((x + 2) as usize)).b32.s1
                    + shift_up;
            }

            if (*mem.add((q + 3) as usize)).b32.s1 == EMPTY {
                free_node(z, BOX_NODE_SIZE);
            } else {
                shift_down = big_op_spacing4() - (*mem.add((z + 3) as usize)).b32.s1;
                if shift_down < big_op_spacing2() {
                    shift_down = big_op_spacing2();
                }
                p = new_kern(shift_down);
                (*mem.add(y as usize)).b32.s1 = p;
                (*mem.add(p as usize)).b32.s1 = z;
                p = new_kern(big_op_spacing5());
                (*mem.add(z as usize)).b32.s1 = p;
                (*mem.add((v + 2) as usize)).b32.s1 = (*mem.add((v + 2) as usize)).b32.s1
                    + big_op_spacing5()
                    + (*mem.add((z + 3) as usize)).b32.s1
                    + (*mem.add((z + 2) as usize)).b32.s1
                    + shift_down;
            }
            (*mem.add((q + 1) as usize)).b32.s1 = v;
        }
        free_ot_assembly(ot_assembly_ptr);
        delta
    }
}

unsafe fn make_ord(q: i32) {
    unsafe {
        let mut a: i32;
        let mut p: i32;
        let mut r: i32;

        loop {
            if (*mem.add((q + 3) as usize)).b32.s1 == EMPTY {
                if (*mem.add((q + 2) as usize)).b32.s1 == EMPTY {
                    if (*mem.add((q + 1) as usize)).b32.s1 == MATH_CHAR {
                        p = (*mem.add(q as usize)).b32.s1;
                        if p != TEX_NULL {
                            if (*mem.add(p as usize)).b16.s1 >= ORD_NOAD as u16
                                && (*mem.add(p as usize)).b16.s1 <= PUNCT_NOAD as u16
                            {
                                if (*mem.add((p + 1) as usize)).b32.s1 == MATH_CHAR {
                                    if ((*mem.add((p + 1) as usize)).b16.s1 % 256)
                                        == ((*mem.add((q + 1) as usize)).b16.s1 % 256)
                                    {
                                        (*mem.add((q + 1) as usize)).b32.s1 = MATH_TEXT_CHAR;
                                        fetch(q + 1);
                                        if (cur_i.s1 % 4) == LIG_TAG {
                                            a = *lig_kern_base.add(cur_f as usize)
                                                + cur_i.s0 as i32;
                                            cur_c = (*mem.add((p + 1) as usize)).b16.s0 as i32;
                                            cur_i = (*font_info.add(a as usize)).b16;
                                            if cur_i.s3 > 128 {
                                                a = *lig_kern_base.add(cur_f as usize)
                                                    + 256 * cur_i.s1 as i32
                                                    + cur_i.s0 as i32
                                                    + 32768
                                                    - 256 * 128;
                                                cur_i = (*font_info.add(a as usize)).b16;
                                            }
                                            loop {
                                                if cur_i.s2 as i32 == cur_c {
                                                    if cur_i.s3 <= 128 {
                                                        if cur_i.s1 >= 128 {
                                                            p = new_kern(
                                                                (*font_info.add(
                                                                    (*kern_base.add(cur_f as usize)
                                                                        + 256 * cur_i.s1 as i32
                                                                        + cur_i.s0 as i32)
                                                                        as usize,
                                                                ))
                                                                .b32
                                                                .s1,
                                                            );
                                                            (*mem.add(p as usize)).b32.s1 =
                                                                (*mem.add(q as usize)).b32.s1;
                                                            (*mem.add(q as usize)).b32.s1 = p;
                                                            return;
                                                        } else {
                                                            match cur_i.s1 {
                                                                1 | 5 => {
                                                                    (*mem.add((q + 1) as usize))
                                                                        .b16
                                                                        .s0 = cur_i.s0;
                                                                }
                                                                2 | 6 => {
                                                                    (*mem.add((p + 1) as usize))
                                                                        .b16
                                                                        .s0 = cur_i.s0;
                                                                }
                                                                3 | 7 | 11 => {
                                                                    r = new_noad();
                                                                    (*mem.add((r + 1) as usize))
                                                                        .b16
                                                                        .s0 = cur_i.s0;
                                                                    (*mem.add((r + 1) as usize))
                                                                        .b16
                                                                        .s1 = (*mem
                                                                        .add((q + 1) as usize))
                                                                    .b16
                                                                    .s1 % 256;
                                                                    (*mem.add(q as usize)).b32.s1 =
                                                                        r;
                                                                    (*mem.add(r as usize)).b32.s1 =
                                                                        p;
                                                                    if cur_i.s1 < 11 {
                                                                        (*mem.add(
                                                                            (r + 1) as usize,
                                                                        ))
                                                                        .b32
                                                                        .s1 = MATH_CHAR;
                                                                    } else {
                                                                        (*mem.add(
                                                                            (r + 1) as usize,
                                                                        ))
                                                                        .b32
                                                                        .s1 = MATH_TEXT_CHAR;
                                                                    }
                                                                }
                                                                _ => {
                                                                    (*mem.add(q as usize)).b32.s1 =
                                                                        (*mem.add(p as usize))
                                                                            .b32
                                                                            .s1;
                                                                    (*mem.add((q + 1) as usize))
                                                                        .b16
                                                                        .s0 = cur_i.s0;
                                                                    *mem.add((q + 3) as usize) =
                                                                        *mem.add((p + 3) as usize);
                                                                    *mem.add((q + 2) as usize) =
                                                                        *mem.add((p + 2) as usize);
                                                                    free_node(p, NOAD_SIZE);
                                                                }
                                                            }
                                                            if cur_i.s1 > 3 {
                                                                return;
                                                            }
                                                            (*mem.add((q + 1) as usize)).b32.s1 =
                                                                MATH_CHAR;
                                                            break; // restart outer loop
                                                        }
                                                    }
                                                }
                                                if cur_i.s3 >= 128 {
                                                    return;
                                                }
                                                a = a + cur_i.s3 as i32 + 1;
                                                cur_i = (*font_info.add(a as usize)).b16;
                                            }
                                            continue; // restart outer loop
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            break;
        }
    }
}

unsafe fn attach_hkern_to_new_hlist(q: i32, delta: scaled_t) -> i32 {
    unsafe {
        let mut y: i32;
        let z = new_kern(delta);
        if (*mem.add((q + 1) as usize)).b32.s1 == TEX_NULL {
            (*mem.add((q + 1) as usize)).b32.s1 = z;
        } else {
            y = (*mem.add((q + 1) as usize)).b32.s1;
            while (*mem.add(y as usize)).b32.s1 != TEX_NULL {
                y = LLIST_link(y);
            }
            (*mem.add(y as usize)).b32.s1 = z;
        }
        (*mem.add((q + 1) as usize)).b32.s1
    }
}

unsafe fn make_scripts(q: i32, delta: scaled_t) {
    unsafe {
        let mut p: i32;
        let mut x: i32;
        let mut y: i32;
        let mut z: i32;
        let mut shift_up: scaled_t;
        let mut shift_down: scaled_t;
        let mut clr: scaled_t;
        let mut sub_kern: scaled_t;
        let mut sup_kern: scaled_t;
        let mut script_c: i32;
        let mut script_g: u16;
        let mut script_f: internal_font_number;
        let sup_g: u16;
        let sup_f: internal_font_number;
        let sub_g: u16;
        let sub_f: internal_font_number;
        let t: i32;
        let mut save_f: internal_font_number;
        let mut script_head: i32;
        let mut script_ptr: i32;
        let mut saved_math_style: small_number;
        let mut this_math_style: small_number;

        p = (*mem.add((q + 1) as usize)).b32.s1;
        script_c = TEX_NULL;
        script_g = 0;
        script_f = 0;
        sup_kern = 0;
        sub_kern = 0;

        if is_char_node(p)
            || (p != TEX_NULL
                && !is_char_node(p)
                && NODE_type(p) == WHATSIT_NODE
                && (*mem.add(p as usize)).b16.s0 == GLYPH_NODE)
        {
            shift_up = 0;
            shift_down = 0;
        } else {
            z = hpack(p, 0, ADDITIONAL);
            if cur_style < SCRIPT_STYLE {
                t = SCRIPT_SIZE;
            } else {
                t = SCRIPT_SCRIPT_SIZE;
            }
            shift_up = (*mem.add((z + 3) as usize)).b32.s1 - sup_drop(t);
            shift_down = (*mem.add((z + 2) as usize)).b32.s1 + sub_drop(t);
            free_node(z, BOX_NODE_SIZE);
        }

        if (*mem.add((q + 2) as usize)).b32.s1 == EMPTY {
            script_head = q + 3;
            script_c = TEX_NULL;
            script_g = 0;
            script_f = 0;
            this_math_style = 2 * (cur_style / 4) + 5;

            if (*mem.add(script_head as usize)).b32.s1 == SUB_MLIST {
                script_ptr = LLIST_info(script_head);
                script_head = TEX_NULL;

                while script_ptr >= 0 && script_ptr <= mem_end {
                    match NODE_type(script_ptr) {
                        KERN_NODE | GLUE_NODE | CHOICE_NODE => {}
                        STYLE_NODE => {
                            this_math_style = NODE_subtype(script_ptr);
                        }
                        ORD_NOAD | OP_NOAD | BIN_NOAD | REL_NOAD | OPEN_NOAD | CLOSE_NOAD
                        | PUNCT_NOAD => {
                            script_head = script_ptr + 1;
                            script_ptr = TEX_NULL;
                        }
                        _ => {
                            script_ptr = TEX_NULL;
                        }
                    }

                    if script_ptr >= 0 && script_ptr <= mem_end {
                        if NODE_type(script_ptr) == CHOICE_NODE {
                            match this_math_style / 2 {
                                0 => {
                                    script_ptr = (*mem.add((script_ptr + 1) as usize)).b32.s0;
                                }
                                1 => {
                                    script_ptr = (*mem.add((script_ptr + 1) as usize)).b32.s1;
                                }
                                2 => {
                                    script_ptr = (*mem.add((script_ptr + 2) as usize)).b32.s0;
                                }
                                3 => {
                                    script_ptr = (*mem.add((script_ptr + 2) as usize)).b32.s1;
                                }
                                _ => {}
                            }
                        } else {
                            script_ptr = LLIST_link(script_ptr);
                        }
                    }
                }
            }

            if script_head >= 0
                && script_head <= mem_end
                && (*mem.add(script_head as usize)).b32.s1 == MATH_CHAR
            {
                save_f = cur_f;
                saved_math_style = cur_style;
                cur_style = this_math_style;

                if cur_style < SCRIPT_STYLE {
                    cur_size = TEXT_SIZE;
                } else {
                    cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                }

                cur_mu = x_over_n(math_quad(cur_size), 18);
                fetch(script_head);

                if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                    && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
                {
                    script_c = new_native_character(cur_f, cur_c);
                    script_g = get_native_glyph(script_c, 0) as u16;
                    script_f = cur_f;
                }

                cur_f = save_f;
                cur_style = saved_math_style;

                if cur_style < SCRIPT_STYLE {
                    cur_size = TEXT_SIZE;
                } else {
                    cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                }

                cur_mu = x_over_n(math_quad(cur_size), 18);
            }

            sub_g = script_g;
            sub_f = script_f;
            save_f = cur_f;

            x = clean_box(q + 3, 2 * (cur_style / 4) + 5);
            cur_f = save_f;
            (*mem.add((x + 1) as usize)).b32.s1 += DIMENPAR(DIMEN_PAR__script_space);

            if shift_down < sub1(cur_size) {
                shift_down = sub1(cur_size);
            }

            if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
            {
                clr = (*mem.add((x + 3) as usize)).b32.s1
                    - get_ot_math_constant(cur_f, SUBSCRIPTTOPMAX);
            } else {
                clr =
                    (*mem.add((x + 3) as usize)).b32.s1 - ((math_x_height(cur_size) * 4).abs() / 5);
            }

            if shift_down < clr {
                shift_down = clr;
            }

            (*mem.add((x + 4) as usize)).b32.s1 = shift_down;

            if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
            {
                if p != TEX_NULL
                    && !is_char_node(p)
                    && NODE_type(p) == WHATSIT_NODE
                    && (*mem.add(p as usize)).b16.s0 == GLYPH_NODE
                {
                    sub_kern = get_ot_math_kern(
                        (*mem.add((p + 4) as usize)).b16.s2 as i32,
                        (*mem.add((p + 4) as usize)).b16.s1 as i32,
                        sub_f,
                        sub_g as i32,
                        SUB_CMD,
                        shift_down,
                    );

                    if sub_kern != 0 {
                        p = attach_hkern_to_new_hlist(q, sub_kern);
                    }
                }
            }
        } else {
            script_head = q + 2;
            script_c = TEX_NULL;
            script_g = 0;
            script_f = 0;
            this_math_style = 2 * (cur_style / 4) + 5;

            if (*mem.add(script_head as usize)).b32.s1 == SUB_MLIST {
                script_ptr = LLIST_info(script_head);
                script_head = TEX_NULL;

                while script_ptr >= 0 && script_ptr <= mem_end {
                    match NODE_type(script_ptr) {
                        KERN_NODE | GLUE_NODE | CHOICE_NODE => {}
                        STYLE_NODE => {
                            this_math_style = NODE_subtype(script_ptr);
                        }
                        ORD_NOAD | OP_NOAD | BIN_NOAD | REL_NOAD | OPEN_NOAD | CLOSE_NOAD
                        | PUNCT_NOAD => {
                            script_head = script_ptr + 1;
                            script_ptr = TEX_NULL;
                        }
                        _ => {
                            script_ptr = TEX_NULL;
                        }
                    }

                    if script_ptr >= 0 && script_ptr <= mem_end {
                        if NODE_type(script_ptr) == CHOICE_NODE {
                            match this_math_style / 2 {
                                0 => {
                                    script_ptr = (*mem.add((script_ptr + 1) as usize)).b32.s0;
                                }
                                1 => {
                                    script_ptr = (*mem.add((script_ptr + 1) as usize)).b32.s1;
                                }
                                2 => {
                                    script_ptr = (*mem.add((script_ptr + 2) as usize)).b32.s0;
                                }
                                3 => {
                                    script_ptr = (*mem.add((script_ptr + 2) as usize)).b32.s1;
                                }
                                _ => {}
                            }
                        } else {
                            script_ptr = LLIST_link(script_ptr);
                        }
                    }
                }
            }

            if script_head >= 0
                && script_head <= mem_end
                && (*mem.add(script_head as usize)).b32.s1 == MATH_CHAR
            {
                save_f = cur_f;
                saved_math_style = cur_style;
                cur_style = this_math_style;

                if cur_style < SCRIPT_STYLE {
                    cur_size = TEXT_SIZE;
                } else {
                    cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                }

                cur_mu = x_over_n(math_quad(cur_size), 18);
                fetch(script_head);

                if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                    && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
                {
                    script_c = new_native_character(cur_f, cur_c);
                    script_g = get_native_glyph(script_c, 0) as u16;
                    script_f = cur_f;
                }

                cur_f = save_f;
                cur_style = saved_math_style;

                if cur_style < SCRIPT_STYLE {
                    cur_size = TEXT_SIZE;
                } else {
                    cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                }

                cur_mu = x_over_n(math_quad(cur_size), 18);
            }

            sup_g = script_g;
            sup_f = script_f;
            save_f = cur_f;

            x = clean_box(q + 2, 2 * (cur_style / 4) + 4 + (cur_style % 2));
            cur_f = save_f;
            (*mem.add((x + 1) as usize)).b32.s1 += DIMENPAR(DIMEN_PAR__script_space);

            if odd(cur_style as i32) {
                clr = sup3(cur_size);
            } else if cur_style < TEXT_STYLE {
                clr = sup1(cur_size);
            } else {
                clr = sup2(cur_size);
            }

            if shift_up < clr {
                shift_up = clr;
            }

            if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
            {
                clr = (*mem.add((x + 2) as usize)).b32.s1
                    + get_ot_math_constant(cur_f, SUPERSCRIPTBOTTOMMIN);
            } else {
                clr = (*mem.add((x + 2) as usize)).b32.s1 + (math_x_height(cur_size).abs() / 4);
            }

            if shift_up < clr {
                shift_up = clr;
            }

            if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
            {
                if (*mem.add((q + 3) as usize)).b32.s1 == EMPTY {
                    if p != TEX_NULL
                        && !is_char_node(p)
                        && NODE_type(p) == WHATSIT_NODE
                        && (*mem.add(p as usize)).b16.s0 == GLYPH_NODE
                    {
                        sup_kern = get_ot_math_kern(
                            (*mem.add((p + 4) as usize)).b16.s2 as i32,
                            (*mem.add((p + 4) as usize)).b16.s1 as i32,
                            sup_f,
                            sup_g as i32,
                            SUP_CMD,
                            shift_up,
                        );

                        if sup_kern != 0 {
                            p = attach_hkern_to_new_hlist(q, sup_kern);
                        }
                    }
                }
            }

            if (*mem.add((q + 3) as usize)).b32.s1 == EMPTY {
                (*mem.add((x + 4) as usize)).b32.s1 = -shift_up;
            } else {
                save_f = cur_f;
                script_head = q + 3;
                script_c = TEX_NULL;
                script_g = 0;
                script_f = 0;
                this_math_style = 2 * (cur_style / 4) + 5;

                if (*mem.add(script_head as usize)).b32.s1 == SUB_MLIST {
                    script_ptr = LLIST_info(script_head);
                    script_head = TEX_NULL;

                    while script_ptr >= 0 && script_ptr <= mem_end {
                        match NODE_type(script_ptr) {
                            KERN_NODE | GLUE_NODE | CHOICE_NODE => {}
                            STYLE_NODE => {
                                this_math_style = NODE_subtype(script_ptr);
                            }
                            ORD_NOAD | OP_NOAD | BIN_NOAD | REL_NOAD | OPEN_NOAD | CLOSE_NOAD
                            | PUNCT_NOAD => {
                                script_head = script_ptr + 1;
                                script_ptr = TEX_NULL;
                            }
                            _ => {
                                script_ptr = TEX_NULL;
                            }
                        }

                        if script_ptr >= 0 && script_ptr <= mem_end {
                            if NODE_type(script_ptr) == CHOICE_NODE {
                                match this_math_style / 2 {
                                    0 => {
                                        script_ptr = (*mem.add((script_ptr + 1) as usize)).b32.s0;
                                    }
                                    1 => {
                                        script_ptr = (*mem.add((script_ptr + 1) as usize)).b32.s1;
                                    }
                                    2 => {
                                        script_ptr = (*mem.add((script_ptr + 2) as usize)).b32.s0;
                                    }
                                    3 => {
                                        script_ptr = (*mem.add((script_ptr + 2) as usize)).b32.s1;
                                    }
                                    _ => {}
                                }
                            } else {
                                script_ptr = LLIST_link(script_ptr);
                            }
                        }
                    }
                }

                if script_head >= 0
                    && script_head <= mem_end
                    && (*mem.add(script_head as usize)).b32.s1 == MATH_CHAR
                {
                    save_f = cur_f;
                    saved_math_style = cur_style;
                    cur_style = this_math_style;

                    if cur_style < SCRIPT_STYLE {
                        cur_size = TEXT_SIZE;
                    } else {
                        cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                    }

                    cur_mu = x_over_n(math_quad(cur_size), 18);
                    fetch(script_head);

                    if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                        && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
                    {
                        script_c = new_native_character(cur_f, cur_c);
                        script_g = get_native_glyph(script_c, 0) as u16;
                        script_f = cur_f;
                    }

                    cur_f = save_f;
                    cur_style = saved_math_style;

                    if cur_style < SCRIPT_STYLE {
                        cur_size = TEXT_SIZE;
                    } else {
                        cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                    }

                    cur_mu = x_over_n(math_quad(cur_size), 18);
                }

                sub_g = script_g;
                sub_f = script_f;

                y = clean_box(q + 3, 2 * (cur_style / 4) + 5);
                cur_f = save_f;
                (*mem.add((y + 1) as usize)).b32.s1 += DIMENPAR(DIMEN_PAR__script_space);

                if shift_down < sub2(cur_size) {
                    shift_down = sub2(cur_size);
                }

                if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                    && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
                {
                    clr = get_ot_math_constant(cur_f, SUBSUPERSCRIPTGAPMIN)
                        - ((shift_up - (*mem.add((x + 2) as usize)).b32.s1)
                            - ((*mem.add((y + 3) as usize)).b32.s1 - shift_down));
                } else {
                    clr = 4 * default_rule_thickness()
                        - ((shift_up - (*mem.add((x + 2) as usize)).b32.s1)
                            - ((*mem.add((y + 3) as usize)).b32.s1 - shift_down));
                }

                if clr > 0 {
                    shift_down += clr;
                    if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                        && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
                    {
                        clr = get_ot_math_constant(cur_f, SUPERSCRIPTBOTTOMMAXWITHSUBSCRIPT)
                            - (shift_up - (*mem.add((x + 2) as usize)).b32.s1);
                    } else {
                        clr = ((math_x_height(cur_size) * 4).abs() / 5)
                            - (shift_up - (*mem.add((x + 2) as usize)).b32.s1);
                    }
                    if clr > 0 {
                        shift_up += clr;
                        shift_down -= clr;
                    }
                }

                if *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                    && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize))
                {
                    if p != TEX_NULL
                        && !is_char_node(p)
                        && NODE_type(p) == WHATSIT_NODE
                        && (*mem.add(p as usize)).b16.s0 == GLYPH_NODE
                    {
                        sub_kern = get_ot_math_kern(
                            (*mem.add((p + 4) as usize)).b16.s2 as i32,
                            (*mem.add((p + 4) as usize)).b16.s1 as i32,
                            sub_f,
                            sub_g as i32,
                            SUB_CMD,
                            shift_down,
                        );

                        if sub_kern != 0 {
                            p = attach_hkern_to_new_hlist(q, sub_kern);
                        }
                    }

                    if (*mem.add((q + 3) as usize)).b32.s1 == EMPTY {
                        if p != TEX_NULL
                            && !is_char_node(p)
                            && NODE_type(p) == WHATSIT_NODE
                            && (*mem.add(p as usize)).b16.s0 == GLYPH_NODE
                        {
                            sup_kern = get_ot_math_kern(
                                (*mem.add((p + 4) as usize)).b16.s2 as i32,
                                (*mem.add((p + 4) as usize)).b16.s1 as i32,
                                sup_f,
                                sup_g as i32,
                                SUP_CMD,
                                shift_up,
                            );

                            if sup_kern != 0 {
                                p = attach_hkern_to_new_hlist(q, sup_kern);
                            }
                        }
                    }
                } else {
                    sup_kern = 0;
                    sub_kern = 0;
                }

                (*mem.add((x + 4) as usize)).b32.s1 = sup_kern + delta - sub_kern;
                p = new_kern(
                    (shift_up - (*mem.add((x + 2) as usize)).b32.s1)
                        - ((*mem.add((y + 3) as usize)).b32.s1 - shift_down),
                );
                (*mem.add(x as usize)).b32.s1 = p;
                (*mem.add(p as usize)).b32.s1 = y;
                x = vpackage(x, 0, ADDITIONAL, MAX_HALFWORD);
                (*mem.add((x + 4) as usize)).b32.s1 = shift_down;
            }
        }

        if (*mem.add((q + 1) as usize)).b32.s1 == TEX_NULL {
            (*mem.add((q + 1) as usize)).b32.s1 = x;
        } else {
            p = (*mem.add((q + 1) as usize)).b32.s1;
            while (*mem.add(p as usize)).b32.s1 != TEX_NULL {
                p = LLIST_link(p);
            }
            (*mem.add(p as usize)).b32.s1 = x;
        }
    }
}

unsafe fn make_left_right(
    q: i32,
    style: small_number,
    max_d: scaled_t,
    max_h: scaled_t,
) -> small_number {
    unsafe {
        let delta: scaled_t;
        let mut delta1: scaled_t;
        let mut delta2: scaled_t;

        cur_style = style;
        if cur_style < SCRIPT_STYLE {
            cur_size = TEXT_SIZE;
        } else {
            cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
        }
        cur_mu = x_over_n(math_quad(cur_size), 18);

        delta2 = max_d + axis_height(cur_size);
        delta1 = max_h + max_d - delta2;
        if delta2 > delta1 {
            delta1 = delta2;
        }
        let mut d = (delta1 / 500) * INTPAR(INT_PAR__delimiter_factor);
        delta2 = delta1 + delta1 - DIMENPAR(DIMEN_PAR__delimiter_shortfall);
        if d < delta2 {
            d = delta2;
        }
        (*mem.add((q + 1) as usize)).b32.s1 = var_delimiter(q + 1, cur_size, d);
        ((*mem.add(q as usize)).b16.s1 as i16) - (LEFT_NOAD - 20)
    }
}

unsafe fn mlist_to_hlist() {
    unsafe {
        let mlist: i32;
        let penalties: bool;
        let style: small_number;
        let mut save_style: small_number;
        let mut q: i32;
        let mut r: i32;
        let mut r_type: small_number;
        let mut t: small_number;
        let mut p: i32 = TEX_NULL;
        let mut x: i32;
        let mut y: i32;
        let mut z: i32;
        let mut pen: i32;
        let mut s: i32;
        let mut max_h: scaled_t;
        let mut max_d: scaled_t;
        let mut delta: scaled_t;

        mlist = cur_mlist;
        penalties = mlist_penalties;
        style = cur_style;
        q = mlist;
        r = TEX_NULL;
        r_type = OP_NOAD;
        max_h = 0;
        max_d = 0;

        if cur_style < SCRIPT_STYLE {
            cur_size = TEXT_SIZE;
        } else {
            cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
        }
        cur_mu = x_over_n(math_quad(cur_size), 18);

        'nodes: while q != TEX_NULL {
            'reswitch: loop {
                delta = 0;
                match (*mem.add(q as usize)).b16.s1 as i16 {
                    BIN_NOAD => match r_type {
                        BIN_NOAD | OP_NOAD | REL_NOAD | OPEN_NOAD | PUNCT_NOAD | LEFT_NOAD => {
                            (*mem.add(q as usize)).b16.s1 = ORD_NOAD as u16;
                            continue 'reswitch;
                        }
                        _ => {}
                    },
                    REL_NOAD | CLOSE_NOAD | PUNCT_NOAD | RIGHT_NOAD => {
                        if r_type == BIN_NOAD {
                            (*mem.add(r as usize)).b16.s1 = ORD_NOAD as u16;
                        }
                        if (*mem.add(q as usize)).b16.s1 == RIGHT_NOAD as u16 {
                            // goto lab80;
                            r = q;
                            r_type = (*mem.add(r as usize)).b16.s1 as small_number;
                            if r_type == RIGHT_NOAD {
                                r_type = LEFT_NOAD;
                                cur_style = style;
                                if cur_style < SCRIPT_STYLE {
                                    cur_size = TEXT_SIZE;
                                } else {
                                    cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                                }
                                cur_mu = x_over_n(math_quad(cur_size), 18);
                            }
                            q = LLIST_link(q);
                            continue 'nodes;
                        }
                    }
                    LEFT_NOAD => {
                        // goto lab80;
                        r = q;
                        r_type = (*mem.add(r as usize)).b16.s1 as small_number;
                        if r_type == RIGHT_NOAD {
                            r_type = LEFT_NOAD;
                            cur_style = style;
                            if cur_style < SCRIPT_STYLE {
                                cur_size = TEXT_SIZE;
                            } else {
                                cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                            }
                            cur_mu = x_over_n(math_quad(cur_size), 18);
                        }
                        q = LLIST_link(q);
                        continue 'nodes;
                    }
                    FRACTION_NOAD => {
                        make_fraction(q);
                        // goto lab82;
                        let z_box = hpack((*mem.add((q + 1) as usize)).b32.s1, 0, ADDITIONAL);
                        if (*mem.add((z_box + 3) as usize)).b32.s1 > max_h {
                            max_h = (*mem.add((z_box + 3) as usize)).b32.s1;
                        }
                        if (*mem.add((z_box + 2) as usize)).b32.s1 > max_d {
                            max_d = (*mem.add((z_box + 2) as usize)).b32.s1;
                        }
                        free_node(z_box, BOX_NODE_SIZE);
                        r = q;
                        r_type = (*mem.add(r as usize)).b16.s1 as small_number;
                        if r_type == RIGHT_NOAD {
                            r_type = LEFT_NOAD;
                            cur_style = style;
                            if cur_style < SCRIPT_STYLE {
                                cur_size = TEXT_SIZE;
                            } else {
                                cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                            }
                            cur_mu = x_over_n(math_quad(cur_size), 18);
                        }
                        q = LLIST_link(q);
                        continue 'nodes;
                    }
                    OP_NOAD => {
                        delta = make_op(q);
                        if (*mem.add(q as usize)).b16.s0 == LIMITS {
                            // goto lab82;
                            let z_box = hpack((*mem.add((q + 1) as usize)).b32.s1, 0, ADDITIONAL);
                            if (*mem.add((z_box + 3) as usize)).b32.s1 > max_h {
                                max_h = (*mem.add((z_box + 3) as usize)).b32.s1;
                            }
                            if (*mem.add((z_box + 2) as usize)).b32.s1 > max_d {
                                max_d = (*mem.add((z_box + 2) as usize)).b32.s1;
                            }
                            free_node(z_box, BOX_NODE_SIZE);
                            r = q;
                            r_type = (*mem.add(r as usize)).b16.s1 as small_number;
                            if r_type == RIGHT_NOAD {
                                r_type = LEFT_NOAD;
                                cur_style = style;
                                if cur_style < SCRIPT_STYLE {
                                    cur_size = TEXT_SIZE;
                                } else {
                                    cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                                }
                                cur_mu = x_over_n(math_quad(cur_size), 18);
                            }
                            q = LLIST_link(q);
                            continue 'nodes;
                        }
                    }
                    ORD_NOAD => {
                        make_ord(q);
                    }
                    OPEN_NOAD | INNER_NOAD => {}
                    RADICAL_NOAD => {
                        make_radical(q);
                    }
                    OVER_NOAD => {
                        make_over(q);
                    }
                    UNDER_NOAD => {
                        make_under(q);
                    }
                    ACCENT_NOAD => {
                        make_math_accent(q);
                    }
                    VCENTER_NOAD => {
                        make_vcenter(q);
                    }
                    STYLE_NODE => {
                        cur_style = (*mem.add(q as usize)).b16.s0 as small_number;
                        if cur_style < SCRIPT_STYLE {
                            cur_size = TEXT_SIZE;
                        } else {
                            cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                        }
                        cur_mu = x_over_n(math_quad(cur_size), 18);
                        // goto lab81;
                        q = LLIST_link(q);
                        continue 'nodes;
                    }
                    CHOICE_NODE => {
                        match cur_style / 2 {
                            0 => {
                                p = (*mem.add((q + 1) as usize)).b32.s0;
                                (*mem.add((q + 1) as usize)).b32.s0 = TEX_NULL;
                            }
                            1 => {
                                p = (*mem.add((q + 1) as usize)).b32.s1;
                                (*mem.add((q + 1) as usize)).b32.s1 = TEX_NULL;
                            }
                            2 => {
                                p = (*mem.add((q + 2) as usize)).b32.s0;
                                (*mem.add((q + 2) as usize)).b32.s0 = TEX_NULL;
                            }
                            3 => {
                                p = (*mem.add((q + 2) as usize)).b32.s1;
                                (*mem.add((q + 2) as usize)).b32.s1 = TEX_NULL;
                            }
                            _ => {}
                        }
                        flush_node_list((*mem.add((q + 1) as usize)).b32.s0);
                        flush_node_list((*mem.add((q + 1) as usize)).b32.s1);
                        flush_node_list((*mem.add((q + 2) as usize)).b32.s0);
                        flush_node_list((*mem.add((q + 2) as usize)).b32.s1);
                        set_NODE_type(q, STYLE_NODE);
                        (*mem.add(q as usize)).b16.s0 = cur_style as u16;
                        (*mem.add((q + 1) as usize)).b32.s1 = 0;
                        (*mem.add((q + 2) as usize)).b32.s1 = 0;
                        if p != TEX_NULL {
                            z = (*mem.add(q as usize)).b32.s1;
                            (*mem.add(q as usize)).b32.s1 = p;
                            while (*mem.add(p as usize)).b32.s1 != TEX_NULL {
                                p = LLIST_link(p);
                            }
                            (*mem.add(p as usize)).b32.s1 = z;
                        }
                        // goto lab81;
                        q = LLIST_link(q);
                        continue 'nodes;
                    }
                    INS_NODE | MARK_NODE | ADJUST_NODE | WHATSIT_NODE | PENALTY_NODE
                    | DISC_NODE => {
                        // goto lab81;
                        q = LLIST_link(q);
                        continue 'nodes;
                    }
                    RULE_NODE => {
                        if (*mem.add((q + 3) as usize)).b32.s1 > max_h {
                            max_h = (*mem.add((q + 3) as usize)).b32.s1;
                        }
                        if (*mem.add((q + 2) as usize)).b32.s1 > max_d {
                            max_d = (*mem.add((q + 2) as usize)).b32.s1;
                        }
                        // goto lab81;
                        q = LLIST_link(q);
                        continue 'nodes;
                    }
                    GLUE_NODE => {
                        if (*mem.add(q as usize)).b16.s0 == MU_GLUE {
                            x = (*mem.add((q + 1) as usize)).b32.s0;
                            y = math_glue(x, cur_mu);
                            delete_glue_ref(x);
                            (*mem.add((q + 1) as usize)).b32.s0 = y;
                            (*mem.add(q as usize)).b16.s0 = NORMAL as u16;
                        } else if cur_size != TEXT_SIZE
                            && (*mem.add(q as usize)).b16.s0 == COND_MATH_GLUE
                        {
                            p = (*mem.add(q as usize)).b32.s1;
                            if p != TEX_NULL {
                                if NODE_type(p) == GLUE_NODE || NODE_type(p) == KERN_NODE {
                                    (*mem.add(q as usize)).b32.s1 = (*mem.add(p as usize)).b32.s1;
                                    (*mem.add(p as usize)).b32.s1 = TEX_NULL;
                                    flush_node_list(p);
                                }
                            }
                        }
                        // goto lab81;
                        q = LLIST_link(q);
                        continue 'nodes;
                    }
                    KERN_NODE => {
                        math_kern(q, cur_mu);
                        // goto lab81;
                        q = LLIST_link(q);
                        continue 'nodes;
                    }
                    _ => {
                        confusion(b"mlist1\0".as_ptr() as *const c_char);
                    }
                }
                break;
            }

            match (*mem.add((q + 1) as usize)).b32.s1 {
                1 | 4 => {
                    fetch(q + 1);
                    if *font_area.add(cur_f as usize) == AAT_FONT_FLAG
                        || *font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                    {
                        z = new_native_character(cur_f, cur_c);
                        p = get_node(GLYPH_NODE_SIZE);
                        set_NODE_type(p, WHATSIT_NODE);
                        (*mem.add(p as usize)).b16.s0 = GLYPH_NODE;
                        (*mem.add((p + 4) as usize)).b16.s2 = cur_f as u16;
                        (*mem.add((p + 4) as usize)).b16.s1 = get_native_glyph(z, 0) as u16;
                        set_native_glyph_metrics(p, 1);
                        free_node(z, (*mem.add((z + 4) as usize)).b16.s3 as i32);
                        delta = get_ot_math_ital_corr(
                            cur_f,
                            (*mem.add((p + 4) as usize)).b16.s1 as i32,
                        );
                        if (*mem.add((q + 1) as usize)).b32.s1 == MATH_TEXT_CHAR
                            && !(*font_area.add(cur_f as usize) == OTGR_FONT_FLAG
                                && isOpenTypeMathFont(*font_layout_engine.add(cur_f as usize)))
                        {
                            delta = 0;
                        }
                        if (*mem.add((q + 3) as usize)).b32.s1 == EMPTY && delta != 0 {
                            (*mem.add(p as usize)).b32.s1 = new_kern(delta);
                            delta = 0;
                        }
                    } else if cur_i.s3 > 0 {
                        delta = FONT_CHARINFO_ITALCORR(cur_f, cur_i);
                        p = new_character(cur_f, cur_c as u16);
                        if (*mem.add((q + 1) as usize)).b32.s1 == MATH_TEXT_CHAR
                            && (*font_info.add(
                                (SPACE_CODE + *param_base.add(cur_f as usize) as usize) as usize,
                            ))
                            .b32
                            .s1 != 0
                        {
                            delta = 0;
                        }
                        if (*mem.add((q + 3) as usize)).b32.s1 == EMPTY && delta != 0 {
                            (*mem.add(p as usize)).b32.s1 = new_kern(delta);
                            delta = 0;
                        }
                    } else {
                        p = TEX_NULL;
                    }
                }
                0 => {
                    p = TEX_NULL;
                }
                2 => {
                    p = (*mem.add((q + 1) as usize)).b32.s0;
                }
                3 => {
                    cur_mlist = (*mem.add((q + 1) as usize)).b32.s0;
                    save_style = cur_style;
                    mlist_penalties = false;
                    mlist_to_hlist();
                    cur_style = save_style;
                    if cur_style < SCRIPT_STYLE {
                        cur_size = TEXT_SIZE;
                    } else {
                        cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                    }
                    cur_mu = x_over_n(math_quad(cur_size), 18);
                    p = hpack((*mem.add(TEMP_HEAD as usize)).b32.s1, 0, ADDITIONAL);
                }
                _ => {
                    confusion(b"mlist2\0".as_ptr() as *const c_char);
                }
            }

            (*mem.add((q + 1) as usize)).b32.s1 = p;
            if !((*mem.add((q + 3) as usize)).b32.s1 == EMPTY
                && (*mem.add((q + 2) as usize)).b32.s1 == EMPTY)
            {
                make_scripts(q, delta);
            }

            // check_dimensions (lab82)
            z = hpack((*mem.add((q + 1) as usize)).b32.s1, 0, ADDITIONAL);
            if (*mem.add((z + 3) as usize)).b32.s1 > max_h {
                max_h = (*mem.add((z + 3) as usize)).b32.s1;
            }
            if (*mem.add((z + 2) as usize)).b32.s1 > max_d {
                max_d = (*mem.add((z + 2) as usize)).b32.s1;
            }
            free_node(z, BOX_NODE_SIZE);

            // done_with_noad (lab80)
            r = q;
            r_type = (*mem.add(r as usize)).b16.s1 as small_number;
            if r_type == RIGHT_NOAD {
                r_type = LEFT_NOAD;
                cur_style = style;
                if cur_style < SCRIPT_STYLE {
                    cur_size = TEXT_SIZE;
                } else {
                    cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                }
                cur_mu = x_over_n(math_quad(cur_size), 18);
            }

            // done_with_node (lab81)
            q = LLIST_link(q);
        }

        if r_type == BIN_NOAD {
            (*mem.add(r as usize)).b16.s1 = ORD_NOAD as u16;
        }

        p = TEMP_HEAD;
        (*mem.add(p as usize)).b32.s1 = TEX_NULL;
        q = mlist;
        r_type = 0;
        cur_style = style;
        if cur_style < SCRIPT_STYLE {
            cur_size = TEXT_SIZE;
        } else {
            cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
        }
        cur_mu = x_over_n(math_quad(cur_size), 18);

        while q != TEX_NULL {
            t = ORD_NOAD;
            s = NOAD_SIZE;
            pen = INF_PENALTY;
            match (*mem.add(q as usize)).b16.s1 as i16 {
                OP_NOAD | OPEN_NOAD | CLOSE_NOAD | PUNCT_NOAD | INNER_NOAD => {
                    t = (*mem.add(q as usize)).b16.s1 as small_number;
                }
                BIN_NOAD => {
                    t = BIN_NOAD;
                    pen = INTPAR(INT_PAR__bin_op_penalty);
                }
                REL_NOAD => {
                    t = REL_NOAD;
                    pen = INTPAR(INT_PAR__rel_penalty);
                }
                ORD_NOAD | VCENTER_NOAD | OVER_NOAD | UNDER_NOAD => {}
                RADICAL_NOAD => {
                    s = RADICAL_NOAD_SIZE;
                }
                ACCENT_NOAD => {
                    s = ACCENT_NOAD_SIZE;
                }
                FRACTION_NOAD => {
                    s = FRACTION_NOAD_SIZE;
                }
                LEFT_NOAD | RIGHT_NOAD => {
                    t = make_left_right(q, style, max_d, max_h);
                }
                STYLE_NODE => {
                    cur_style = (*mem.add(q as usize)).b16.s0 as small_number;
                    s = STYLE_NODE_SIZE;
                    if cur_style < SCRIPT_STYLE {
                        cur_size = TEXT_SIZE;
                    } else {
                        cur_size = SCRIPT_SIZE * (((cur_style - 2) / 2) as i32);
                    }
                    cur_mu = x_over_n(math_quad(cur_size), 18);
                    // lab83: delete_q
                    r = q;
                    q = LLIST_link(q);
                    free_node(r, s);
                    continue;
                }
                WHATSIT_NODE | PENALTY_NODE | RULE_NODE | DISC_NODE | ADJUST_NODE | INS_NODE
                | MARK_NODE | GLUE_NODE | KERN_NODE => {
                    (*mem.add(p as usize)).b32.s1 = q;
                    p = q;
                    q = LLIST_link(q);
                    (*mem.add(p as usize)).b32.s1 = TEX_NULL;
                    continue;
                }
                _ => {
                    confusion(b"mlist3\0".as_ptr() as *const c_char);
                }
            }

            if r_type > 0 {
                const OFFSET_TABLE: [&[u8; 8]; 8] = [
                    b"02340001",
                    b"22*40001",
                    b"33**3**3",
                    b"44*04004",
                    b"00*00000",
                    b"02340001",
                    b"11*11111",
                    b"12341011",
                ];

                let row = (r_type - ORD_NOAD) as usize;
                let col = (t - ORD_NOAD) as usize;
                match OFFSET_TABLE[row][col] {
                    b'0' => {
                        x = 0;
                    }
                    b'1' => {
                        if cur_style < SCRIPT_STYLE {
                            x = GLUE_PAR__thin_mu_skip;
                        } else {
                            x = 0;
                        }
                    }
                    b'2' => {
                        x = GLUE_PAR__thin_mu_skip;
                    }
                    b'3' => {
                        if cur_style < SCRIPT_STYLE {
                            x = GLUE_PAR__med_mu_skip;
                        } else {
                            x = 0;
                        }
                    }
                    b'4' => {
                        if cur_style < SCRIPT_STYLE {
                            x = GLUE_PAR__thick_mu_skip;
                        } else {
                            x = 0;
                        }
                    }
                    _ => {
                        confusion(b"mlist4\0".as_ptr() as *const c_char);
                    }
                }

                if x != 0 {
                    y = math_glue((*eqtb.add(GLUE_BASE + x as usize)).b32.s1, cur_mu);
                    z = new_glue(y);
                    (*mem.add(y as usize)).b32.s1 = TEX_NULL;
                    (*mem.add(p as usize)).b32.s1 = z;
                    p = z;
                    (*mem.add(z as usize)).b16.s0 = (x + 1) as u16;
                }
            }

            if (*mem.add((q + 1) as usize)).b32.s1 != TEX_NULL {
                (*mem.add(p as usize)).b32.s1 = (*mem.add((q + 1) as usize)).b32.s1;
                loop {
                    p = LLIST_link(p);
                    if (*mem.add(p as usize)).b32.s1 == TEX_NULL {
                        break;
                    }
                }
            }

            if penalties {
                if (*mem.add(q as usize)).b32.s1 != TEX_NULL {
                    if pen < INF_PENALTY {
                        r_type = (*mem.add((*mem.add(q as usize)).b32.s1 as usize)).b16.s1
                            as small_number;
                        if r_type != PENALTY_NODE && r_type != REL_NOAD {
                            z = new_penalty(pen);
                            (*mem.add(p as usize)).b32.s1 = z;
                            p = z;
                        }
                    }
                }
            }

            if (*mem.add(q as usize)).b16.s1 == RIGHT_NOAD as u16 {
                t = OPEN_NOAD;
            }
            r_type = t;

            // lab83: delete_q
            r = q;
            q = LLIST_link(q);
            free_node(r, s);
        }
    }
}

unsafe fn rebox(mut b: i32, w: scaled_t) -> i32 {
    unsafe {
        let mut p: i32;
        let f: internal_font_number;
        let v: scaled_t;
        if (*mem.add((b + 1) as usize)).b32.s1 != w
            && (*mem.add((b + 5) as usize)).b32.s1 != TEX_NULL
        {
            if NODE_type(b) == VLIST_NODE {
                b = hpack(b, 0, ADDITIONAL);
            }
            p = (*mem.add((b + 5) as usize)).b32.s1;
            if is_char_node(p) && (*mem.add(p as usize)).b32.s1 == TEX_NULL {
                f = CHAR_NODE_font(p);
                v = FONT_CHARACTER_WIDTH(f, effective_char(true, f, CHAR_NODE_character(p) as u16));
                if v != (*mem.add((b + 1) as usize)).b32.s1 {
                    (*mem.add(p as usize)).b32.s1 =
                        new_kern((*mem.add((b + 1) as usize)).b32.s1 - v);
                }
            }
            free_node(b, BOX_NODE_SIZE);
            b = new_glue(12);
            (*mem.add(b as usize)).b32.s1 = p;
            while (*mem.add(p as usize)).b32.s1 != TEX_NULL {
                p = LLIST_link(p);
            }
            (*mem.add(p as usize)).b32.s1 = new_glue(12);
            hpack(b, w, EXACTLY)
        } else {
            (*mem.add((b + 1) as usize)).b32.s1 = w;
            b
        }
    }
}

unsafe fn var_delimiter(d: i32, s: i32, v: scaled_t) -> i32 {
    unsafe {
        let mut b: i32;
        let mut ot_assembly_ptr: *mut core::ffi::c_void;
        let mut f: internal_font_number;
        let mut g: internal_font_number;
        let mut c: i32 = 0;
        let mut x: i32;
        let mut y: i32;
        let m: i32;
        let mut n: i32;
        let mut u: scaled_t;
        let mut w: scaled_t;
        let mut q: b16x4 = b16x4 {
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
        };
        let r: b16x4;
        let mut z: i32;
        let mut large_attempt: bool;

        f = FONT_BASE;
        w = 0;
        large_attempt = false;
        z = ((*mem.add(d as usize)).b16.s3 % 256) as i32;
        x = (*mem.add(d as usize)).b16.s2 as i32
            + (((*mem.add(d as usize)).b16.s3 / 256) as i32) * 65536;
        ot_assembly_ptr = core::ptr::null_mut();

        'search: loop {
            if z != 0 || x != 0 {
                z = z + s + 256;
                loop {
                    z -= 256;
                    g = MATH_FONT(z as usize);
                    if g != FONT_BASE {
                        if *font_area.add(g as usize) == OTGR_FONT_FLAG
                            && usingOpenType(*font_layout_engine.add(g as usize))
                        {
                            x = map_char_to_glyph(g, x);
                            f = g;
                            c = x;
                            w = 0;
                            n = 0;
                            u = 0;
                            loop {
                                y = get_ot_math_variant(g, x, n, &mut u, 0);
                                if u > w {
                                    c = y;
                                    w = u;
                                    if u >= v {
                                        break 'search;
                                    }
                                }
                                n += 1;
                                if u < 0 {
                                    break;
                                }
                            }
                            ot_assembly_ptr = get_ot_assembly_ptr(g, x, 0);
                            if !ot_assembly_ptr.is_null() {
                                break 'search;
                            }
                        } else {
                            y = x;
                            if y >= i32::from(*font_bc.add(g as usize))
                                && y <= i32::from(*font_ec.add(g as usize))
                            {
                                loop {
                                    q = FONT_CHARACTER_INFO(g, y);
                                    if q.s3 > 0 {
                                        if (q.s1 % 4) == EXT_TAG {
                                            f = g;
                                            c = y;
                                            break 'search;
                                        }
                                        u = FONT_CHARINFO_HEIGHT(g, q) + FONT_CHARINFO_DEPTH(g, q);
                                        if u > w {
                                            f = g;
                                            c = y;
                                            w = u;
                                            if u >= v {
                                                break 'search;
                                            }
                                        }
                                        if (q.s1 % 4) == LIST_TAG {
                                            y = q.s0 as i32;
                                            continue;
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                    }
                    if z < SCRIPT_SIZE {
                        break;
                    }
                }
            }
            if large_attempt {
                break 'search;
            }
            large_attempt = true;
            z = ((*mem.add(d as usize)).b16.s1 % 256) as i32;
            x = (*mem.add(d as usize)).b16.s0 as i32
                + (((*mem.add(d as usize)).b16.s1 / 256) as i32) * 65536;
        }

        if f != FONT_BASE {
            if !(*font_area.add(f as usize) == OTGR_FONT_FLAG
                && usingOpenType(*font_layout_engine.add(f as usize)))
            {
                if (q.s1 % 4) == EXT_TAG {
                    b = new_null_box();
                    set_NODE_type(b, VLIST_NODE);
                    let r_char =
                        (*font_info.add((*exten_base.add(f as usize) + q.s0 as i32) as usize)).b16;
                    c = r_char.s0 as i32;
                    u = height_plus_depth(f, c as u16);
                    w = 0;
                    q = FONT_CHARACTER_INFO(f, effective_char(true, f, c as u16));
                    (*mem.add((b + 1) as usize)).b32.s1 =
                        FONT_CHARINFO_WIDTH(f, q) + FONT_CHARINFO_ITALCORR(f, q);
                    c = r_char.s1 as i32;
                    if c != 0 {
                        w += height_plus_depth(f, c as u16);
                    }
                    c = r_char.s2 as i32;
                    if c != 0 {
                        w += height_plus_depth(f, c as u16);
                    }
                    c = r_char.s3 as i32;
                    if c != 0 {
                        w += height_plus_depth(f, c as u16);
                    }
                    n = 0;
                    if u > 0 {
                        while w < v {
                            w += u;
                            n += 1;
                            if r_char.s2 != 0 {
                                w += u;
                            }
                        }
                    }
                    c = r_char.s1 as i32;
                    if c != 0 {
                        stack_into_box(b, f, c as u16);
                    }
                    c = r_char.s0 as i32;
                    for _ in 1..=n {
                        stack_into_box(b, f, c as u16);
                    }
                    c = r_char.s2 as i32;
                    if c != 0 {
                        stack_into_box(b, f, c as u16);
                        c = r_char.s0 as i32;
                        for _ in 1..=n {
                            stack_into_box(b, f, c as u16);
                        }
                    }
                    c = r_char.s3 as i32;
                    if c != 0 {
                        stack_into_box(b, f, c as u16);
                    }
                    (*mem.add((b + 2) as usize)).b32.s1 = w - (*mem.add((b + 3) as usize)).b32.s1;
                } else {
                    b = char_box(f, c);
                }
            } else if !ot_assembly_ptr.is_null() {
                b = build_opentype_assembly(f, ot_assembly_ptr, v, false);
            } else {
                b = new_null_box();
                set_NODE_type(b, VLIST_NODE);
                let gnode = get_node(GLYPH_NODE_SIZE);
                (*mem.add((b + 5) as usize)).b32.s1 = gnode;
                set_NODE_type(gnode, WHATSIT_NODE);
                (*mem.add(gnode as usize)).b16.s0 = GLYPH_NODE;
                (*mem.add((gnode + 4) as usize)).b16.s2 = f as u16;
                (*mem.add((gnode + 4) as usize)).b16.s1 = c as u16;
                set_native_glyph_metrics(gnode, 1);
                (*mem.add((b + 1) as usize)).b32.s1 = (*mem.add((gnode + 1) as usize)).b32.s1;
                (*mem.add((b + 3) as usize)).b32.s1 = (*mem.add((gnode + 3) as usize)).b32.s1;
                (*mem.add((b + 2) as usize)).b32.s1 = (*mem.add((gnode + 2) as usize)).b32.s1;
            }
        } else {
            b = new_null_box();
            (*mem.add((b + 1) as usize)).b32.s1 = DIMENPAR(DIMEN_PAR__null_delimiter_space);
        }

        (*mem.add((b + 4) as usize)).b32.s1 =
            half((*mem.add((b + 3) as usize)).b32.s1 - (*mem.add((b + 2) as usize)).b32.s1)
                - axis_height(s);
        free_ot_assembly(ot_assembly_ptr);
        b
    }
}

unsafe fn char_box(f: internal_font_number, c: i32) -> i32 {
    unsafe {
        let q: b16x4;
        let b: i32;
        let p: i32;

        if *font_area.add(f as usize) == AAT_FONT_FLAG
            || *font_area.add(f as usize) == OTGR_FONT_FLAG
        {
            b = new_null_box();
            p = new_native_character(f, c);
            (*mem.add((b + 5) as usize)).b32.s1 = p;
            (*mem.add((b + 3) as usize)).b32.s1 = (*mem.add((p + 3) as usize)).b32.s1;
            (*mem.add((b + 1) as usize)).b32.s1 = (*mem.add((p + 1) as usize)).b32.s1;
            if (*mem.add((p + 2) as usize)).b32.s1 < 0 {
                (*mem.add((b + 2) as usize)).b32.s1 = 0;
            } else {
                (*mem.add((b + 2) as usize)).b32.s1 = (*mem.add((p + 2) as usize)).b32.s1;
            }
        } else {
            q = FONT_CHARACTER_INFO(f, effective_char(true, f, c as u16));
            b = new_null_box();
            (*mem.add((b + 1) as usize)).b32.s1 =
                FONT_CHARINFO_WIDTH(f, q) + FONT_CHARINFO_ITALCORR(f, q);
            (*mem.add((b + 3) as usize)).b32.s1 = FONT_CHARINFO_HEIGHT(f, q);
            (*mem.add((b + 2) as usize)).b32.s1 = FONT_CHARINFO_DEPTH(f, q);
            p = get_avail();
            (*mem.add(p as usize)).b16.s0 = c as u16;
            (*mem.add(p as usize)).b16.s1 = f as u16;
        }
        (*mem.add((b + 5) as usize)).b32.s1 = p;
        b
    }
}

unsafe fn stack_into_box(b: i32, f: internal_font_number, c: u16) {
    unsafe {
        let p = char_box(f, c as i32);
        (*mem.add(p as usize)).b32.s1 = (*mem.add((b + 5) as usize)).b32.s1;
        (*mem.add((b + 5) as usize)).b32.s1 = p;
        (*mem.add((b + 3) as usize)).b32.s1 = (*mem.add((p + 3) as usize)).b32.s1;
    }
}

unsafe fn height_plus_depth(f: internal_font_number, c: u16) -> scaled_t {
    unsafe {
        let q = FONT_CHARACTER_INFO(f, effective_char(true, f, c as u16));
        FONT_CHARINFO_HEIGHT(f, q) + FONT_CHARINFO_DEPTH(f, q)
    }
}

unsafe fn stack_glyph_into_box(b: i32, f: internal_font_number, g: i32) {
    unsafe {
        let mut p: i32;
        let mut q: i32;
        p = get_node(GLYPH_NODE_SIZE);
        set_NODE_type(p, WHATSIT_NODE);
        (*mem.add(p as usize)).b16.s0 = GLYPH_NODE;
        (*mem.add((p + 4) as usize)).b16.s2 = f as u16;
        (*mem.add((p + 4) as usize)).b16.s1 = g as u16;
        set_native_glyph_metrics(p, 1);

        if NODE_type(b) == HLIST_NODE {
            q = (*mem.add((b + 5) as usize)).b32.s1;
            if q == TEX_NULL {
                (*mem.add((b + 5) as usize)).b32.s1 = p;
            } else {
                while (*mem.add(q as usize)).b32.s1 != TEX_NULL {
                    q = LLIST_link(q);
                }
                (*mem.add(q as usize)).b32.s1 = p;
                if (*mem.add((b + 3) as usize)).b32.s1 < (*mem.add((p + 3) as usize)).b32.s1 {
                    (*mem.add((b + 3) as usize)).b32.s1 = (*mem.add((p + 3) as usize)).b32.s1;
                }
                if (*mem.add((b + 2) as usize)).b32.s1 < (*mem.add((p + 2) as usize)).b32.s1 {
                    (*mem.add((b + 2) as usize)).b32.s1 = (*mem.add((p + 2) as usize)).b32.s1;
                }
            }
        } else {
            (*mem.add(p as usize)).b32.s1 = (*mem.add((b + 5) as usize)).b32.s1;
            (*mem.add((b + 5) as usize)).b32.s1 = p;
            (*mem.add((b + 3) as usize)).b32.s1 = (*mem.add((p + 3) as usize)).b32.s1;
            if (*mem.add((b + 1) as usize)).b32.s1 < (*mem.add((p + 1) as usize)).b32.s1 {
                (*mem.add((b + 1) as usize)).b32.s1 = (*mem.add((p + 1) as usize)).b32.s1;
            }
        }
    }
}

unsafe fn stack_glue_into_box(b: i32, min: scaled_t, max: scaled_t) {
    unsafe {
        let p: i32;
        let mut q: i32;
        q = new_spec(0);
        (*mem.add((q + 1) as usize)).b32.s1 = min;
        (*mem.add((q + 2) as usize)).b32.s1 = max - min;
        p = new_glue(q);

        if NODE_type(b) == HLIST_NODE {
            q = (*mem.add((b + 5) as usize)).b32.s1;
            if q == TEX_NULL {
                (*mem.add((b + 5) as usize)).b32.s1 = p;
            } else {
                while (*mem.add(q as usize)).b32.s1 != TEX_NULL {
                    q = LLIST_link(q);
                }
                (*mem.add(q as usize)).b32.s1 = p;
            }
        } else {
            (*mem.add(p as usize)).b32.s1 = (*mem.add((b + 5) as usize)).b32.s1;
            (*mem.add((b + 5) as usize)).b32.s1 = p;
            (*mem.add((b + 3) as usize)).b32.s1 = (*mem.add((p + 3) as usize)).b32.s1;
            (*mem.add((b + 1) as usize)).b32.s1 = (*mem.add((p + 1) as usize)).b32.s1;
        }
    }
}

unsafe fn build_opentype_assembly(
    f: internal_font_number,
    a: *mut core::ffi::c_void,
    s: scaled_t,
    horiz: bool,
) -> i32 {
    unsafe {
        let b: i32;
        let mut n: i32;
        let mut g: i32;
        let mut p: i32;
        let mut s_max: scaled_t;
        let mut o: scaled_t;
        let mut oo: scaled_t;
        let mut prev_o: scaled_t;
        let min_o: scaled_t;
        let mut no_extenders: bool;
        let mut nat: scaled_t;
        let mut str: scaled_t;

        b = new_null_box();
        if horiz {
            set_NODE_type(b, HLIST_NODE);
        } else {
            set_NODE_type(b, VLIST_NODE);
        }

        n = -1;
        no_extenders = true;
        min_o = ot_min_connector_overlap(f);

        loop {
            n += 1;
            s_max = 0;
            prev_o = 0;
            let part_cnt = ot_part_count(a);
            for i in 0..part_cnt {
                if ot_part_is_extender(a, i) {
                    no_extenders = false;
                    for _ in 1..=n {
                        o = ot_part_start_connector(f, a, i);
                        if min_o < o {
                            o = min_o;
                        }
                        if prev_o < o {
                            o = prev_o;
                        }
                        s_max = s_max - o + ot_part_full_advance(f, a, i);
                        prev_o = ot_part_end_connector(f, a, i);
                    }
                } else {
                    o = ot_part_start_connector(f, a, i);
                    if min_o < o {
                        o = min_o;
                    }
                    if prev_o < o {
                        o = prev_o;
                    }
                    s_max = s_max - o + ot_part_full_advance(f, a, i);
                    prev_o = ot_part_end_connector(f, a, i);
                }
            }
            if s_max >= s || no_extenders {
                break;
            }
        }

        prev_o = 0;
        let part_cnt = ot_part_count(a);
        for i in 0..part_cnt {
            if ot_part_is_extender(a, i) {
                for _ in 1..=n {
                    o = ot_part_start_connector(f, a, i);
                    if prev_o < o {
                        o = prev_o;
                    }
                    oo = o;
                    if min_o < o {
                        o = min_o;
                    }
                    if oo > 0 {
                        stack_glue_into_box(b, -oo, -o);
                    }
                    g = ot_part_glyph(a, i);
                    stack_glyph_into_box(b, f, g);
                    prev_o = ot_part_end_connector(f, a, i);
                }
            } else {
                o = ot_part_start_connector(f, a, i);
                if prev_o < o {
                    o = prev_o;
                }
                oo = o;
                if min_o < o {
                    o = min_o;
                }
                if oo > 0 {
                    stack_glue_into_box(b, -oo, -o);
                }
                g = ot_part_glyph(a, i);
                stack_glyph_into_box(b, f, g);
                prev_o = ot_part_end_connector(f, a, i);
            }
        }

        p = (*mem.add((b + 5) as usize)).b32.s1;
        nat = 0;
        str = 0;
        while p != TEX_NULL {
            if NODE_type(p) == WHATSIT_NODE {
                if horiz {
                    nat += (*mem.add((p + 1) as usize)).b32.s1;
                } else {
                    nat +=
                        (*mem.add((p + 3) as usize)).b32.s1 + (*mem.add((p + 2) as usize)).b32.s1;
                }
            } else if NODE_type(p) == GLUE_NODE {
                let gspec = (*mem.add((p + 1) as usize)).b32.s0;
                nat += (*mem.add((gspec + 1) as usize)).b32.s1;
                str += (*mem.add((gspec + 2) as usize)).b32.s1;
            }
            p = LLIST_link(p);
        }

        o = 0;
        if s > nat && str > 0 {
            o = s - nat;
            if o > str {
                o = str;
            }
            (*mem.add((b + 5) as usize)).b16.s0 = NORMAL as u16;
            (*mem.add((b + 5) as usize)).b16.s1 = STRETCHING;
            (*mem.add((b + 6) as usize)).gr = o as f64 / str as f64;
            if horiz {
                (*mem.add((b + 1) as usize)).b32.s1 =
                    nat + tex_round(str as f64 * (*mem.add((b + 6) as usize)).gr);
            } else {
                (*mem.add((b + 3) as usize)).b32.s1 =
                    nat + tex_round(str as f64 * (*mem.add((b + 6) as usize)).gr);
            }
        } else if horiz {
            (*mem.add((b + 1) as usize)).b32.s1 = nat;
        } else {
            (*mem.add((b + 3) as usize)).b32.s1 = nat;
        }

        b
    }
}
