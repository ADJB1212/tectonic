// Copyright 2016-2018 The Tectonic Project
// Licensed under the MIT License.

/* Customizations for Tectonic:
 *
 * In semantic pagination mode, we don't actually linebreak paragraphs. We
 * just set them as one big line at its natural width no matter what. In the
 * context of this algorithm that means that try_break is always a no-op
 * unless we're trying to break at the very end of the paragraph.
 */

#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports,
    unsafe_op_in_unsafe_fn
)]

pub const AWFUL_BAD: i32 = 0x3FFF_FFFF;

pub const VERY_LOOSE_FIT: usize = 0;
pub const LOOSE_FIT: usize = 1;
pub const DECENT_FIT: usize = 2;
pub const TIGHT_FIT: usize = 3;

pub const MIN_HALFWORD: i32 = -0x0FFF_FFFF;
pub const MAX_HALFWORD: i32 = 0x3FFF_FFFF;
pub const TEX_NULL: i32 = MIN_HALFWORD;

pub const BIGGEST_CHAR: i32 = 0xFFFF;
pub const TOO_BIG_CHAR: i32 = 65536;

pub const INF_PENALTY: i32 = 10000;
pub const EJECT_PENALTY: i32 = -10000;
pub const INF_BAD: i32 = 10000;

pub const NORMAL: i32 = 0;
pub const EXPLICIT: i32 = 1;
pub const SPACE_ADJUSTMENT: i32 = 3;
pub const L_CODE: i32 = 4;
pub const NON_ADDRESS: i32 = 0;

pub const EXACTLY: i32 = 0;
pub const ADDITIONAL: i32 = 1;

pub const UNHYPHENATED: u16 = 0;
pub const HYPHENATED: u16 = 1;

pub const LIG_TAG: u16 = 1;

pub const MEM_TOP: i32 = 4999999;
pub const TEMP_HEAD: i32 = MEM_TOP - 3;
pub const HOLD_HEAD: i32 = MEM_TOP - 4;
pub const ADJUST_HEAD: i32 = MEM_TOP - 5;
pub const ACTIVE_LIST: i32 = MEM_TOP - 7;
pub const PRE_ADJUST_HEAD: i32 = MEM_TOP - 14;
pub const LAST_ACTIVE: i32 = ACTIVE_LIST;

pub const HLIST_NODE: i32 = 0;
pub const VLIST_NODE: i32 = 1;
pub const RULE_NODE: i32 = 2;
pub const DELTA_NODE: u16 = 2;
pub const INS_NODE: i32 = 3;
pub const MARK_NODE: i32 = 4;
pub const ADJUST_NODE: i32 = 5;
pub const LIGATURE_NODE: i32 = 6;
pub const DISC_NODE: i32 = 7;
pub const WHATSIT_NODE: i32 = 8;
pub const MATH_NODE: i32 = 9;
pub const GLUE_NODE: i32 = 10;
pub const KERN_NODE: i32 = 11;
pub const PENALTY_NODE: i32 = 12;

pub const LANGUAGE_NODE: i32 = 4;
pub const NATIVE_WORD_NODE: i32 = 40;
pub const NATIVE_WORD_NODE_AT: i32 = 41;
pub const GLYPH_NODE: i32 = 42;
pub const PIC_NODE: i32 = 43;
pub const PDF_NODE: i32 = 44;

pub const ACTIVE_NODE_SIZE_NORMAL: small_number = 3;
pub const ACTIVE_NODE_SIZE_EXTENDED: small_number = 5;
pub const PASSIVE_NODE_SIZE: i32 = 2;
pub const DELTA_NODE_SIZE: i32 = 7;
pub const SMALL_NODE_SIZE: i32 = 2;
pub const NATIVE_NODE_SIZE: i32 = 6;

pub const MAX_HLIST_STACK: usize = 512;
pub const HYPH_PRIME: i32 = 607;
pub const TRIE_OP_SIZE: i32 = 35111;
pub const MIN_TRIE_OP: i32 = 0;

pub const GLUE_BASE: i32 = 2254340;
pub const LOCAL_BASE: i32 = 2254871;
pub const ETEX_PEN_BASE: i32 = 2255140;
pub const LC_CODE_BASE: i32 = 3370281;
pub const INT_BASE: i32 = 7826729;
pub const DIMEN_BASE: i32 = 8941180;

pub const INT_PAR__pretolerance: i32 = 0;
pub const INT_PAR__tolerance: i32 = 1;
pub const INT_PAR__line_penalty: i32 = 2;
pub const INT_PAR__hyphen_penalty: i32 = 3;
pub const INT_PAR__ex_hyphen_penalty: i32 = 4;
pub const INT_PAR__club_penalty: i32 = 5;
pub const INT_PAR__widow_penalty: i32 = 6;
pub const INT_PAR__display_widow_penalty: i32 = 7;
pub const INT_PAR__broken_penalty: i32 = 8;
pub const INT_PAR__inter_line_penalty: i32 = 13;
pub const INT_PAR__double_hyphen_demerits: i32 = 14;
pub const INT_PAR__final_hyphen_demerits: i32 = 15;
pub const INT_PAR__adj_demerits: i32 = 16;
pub const INT_PAR__looseness: i32 = 19;
pub const INT_PAR__uc_hyph: i32 = 38;
pub const INT_PAR__hang_after: i32 = 41;
pub const INT_PAR__last_line_fit: i32 = 62;
pub const INT_PAR__xetex_protrude_chars: i32 = 68;
pub const INT_PAR__texxet: i32 = 69;
pub const INT_PAR__xetex_use_glyph_metrics: i32 = 72;

pub const DIMEN_PAR__hsize: i32 = 3;
pub const DIMEN_PAR__hang_indent: i32 = 17;
pub const DIMEN_PAR__emergency_stretch: i32 = 20;

pub const GLUE_PAR__left_skip: i32 = 7;
pub const GLUE_PAR__right_skip: i32 = 8;
pub const GLUE_PAR__par_fill_skip: i32 = 14;

pub const LOCAL__par_shape: i32 = 0;

pub const ETEX_PENALTIES_PAR__inter_line_penalties: i32 = 0;
pub const ETEX_PENALTIES_PAR__club_penalties: i32 = 1;
pub const ETEX_PENALTIES_PAR__widow_penalties: i32 = 2;
pub const ETEX_PENALTIES_PAR__display_widow_penalties: i32 = 3;

pub type scaled_t = i32;
pub type UnicodeScalar = i32;
pub type internal_font_number = i32;
pub type font_index = i32;
pub type trie_pointer = i32;
pub type trie_opcode = u16;
pub type hyph_pointer = u16;
pub type str_number = i32;
pub type pool_pointer = i32;
pub type small_number = i16;
pub type UTF16_code = u16;
pub type packed_UTF16_code = u16;
pub type nine_bits = i32;

#[cfg(target_endian = "big")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2 {
    pub s1: i32,
    pub s0: i32,
}

#[cfg(not(target_endian = "big"))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2 {
    pub s0: i32,
    pub s1: i32,
}

#[cfg(target_endian = "big")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b16x4 {
    pub s3: u16,
    pub s2: u16,
    pub s1: u16,
    pub s0: u16,
}

#[cfg(not(target_endian = "big"))]
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub ptr: *mut std::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_state_record {
    pub mode: i16,
    pub head: i32,
    pub tail: i32,
    pub eTeX_aux: i32,
    pub prev_graf: i32,
    pub mode_line: i32,
    pub aux: memory_word,
}

pub enum PreparedLayouts {}

macro_rules! LLIST_link {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b32.s1
    };
}

macro_rules! LLIST_info {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b32.s0
    };
}

macro_rules! NODE_type {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b16.s1
    };
}

macro_rules! NODE_subtype {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b16.s0
    };
}

macro_rules! BOX_width {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s1
    };
}

macro_rules! BOX_shift_amount {
    ($p:expr) => {
        (*mem.offset((($p) + 4) as isize)).b32.s1
    };
}

macro_rules! ACTIVE_NODE_fitness {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b16.s0
    };
}

macro_rules! ACTIVE_NODE_break_node {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s1
    };
}

macro_rules! ACTIVE_NODE_line_number {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s0
    };
}

macro_rules! ACTIVE_NODE_total_demerits {
    ($p:expr) => {
        (*mem.offset((($p) + 2) as isize)).b32.s1
    };
}

macro_rules! ACTIVE_NODE_shortfall {
    ($p:expr) => {
        (*mem.offset((($p) + 3) as isize)).b32.s1
    };
}

macro_rules! ACTIVE_NODE_glue {
    ($p:expr) => {
        (*mem.offset((($p) + 4) as isize)).b32.s1
    };
}

macro_rules! CHAR_NODE_font {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b16.s1
    };
}

macro_rules! CHAR_NODE_character {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b16.s0
    };
}

macro_rules! DELTA_NODE_dwidth {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s1
    };
}

macro_rules! DELTA_NODE_dstretch0 {
    ($p:expr) => {
        (*mem.offset((($p) + 2) as isize)).b32.s1
    };
}

macro_rules! DELTA_NODE_dstretch1 {
    ($p:expr) => {
        (*mem.offset((($p) + 3) as isize)).b32.s1
    };
}

macro_rules! DELTA_NODE_dstretch2 {
    ($p:expr) => {
        (*mem.offset((($p) + 4) as isize)).b32.s1
    };
}

macro_rules! DELTA_NODE_dstretch3 {
    ($p:expr) => {
        (*mem.offset((($p) + 5) as isize)).b32.s1
    };
}

macro_rules! DELTA_NODE_dshrink {
    ($p:expr) => {
        (*mem.offset((($p) + 6) as isize)).b32.s1
    };
}

macro_rules! DISCRETIONARY_NODE_replace_count {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b16.s0
    };
}

macro_rules! DISCRETIONARY_NODE_pre_break {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s0
    };
}

macro_rules! DISCRETIONARY_NODE_post_break {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s1
    };
}

macro_rules! GLUE_NODE_glue_ptr {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s0
    };
}

macro_rules! GLUE_NODE_leader_ptr {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s1
    };
}

macro_rules! LANGUAGE_NODE_what_lang {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s1
    };
}

macro_rules! LANGUAGE_NODE_what_lhm {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b16.s1
    };
}

macro_rules! LANGUAGE_NODE_what_rhm {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b16.s0
    };
}

macro_rules! LIGATURE_NODE_lig_font {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b16.s1
    };
}

macro_rules! LIGATURE_NODE_lig_char {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b16.s0
    };
}

macro_rules! LIGATURE_NODE_lig_ptr {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s1
    };
}

macro_rules! PASSIVE_NODE_prev_break {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s0
    };
}

macro_rules! PASSIVE_NODE_next_break {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s0
    };
}

macro_rules! PASSIVE_NODE_cur_break {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s1
    };
}

macro_rules! PASSIVE_NODE_serial {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b32.s0
    };
}

macro_rules! PENALTY_NODE_penalty {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s1
    };
}

macro_rules! NATIVE_NODE_size {
    ($p:expr) => {
        (*mem.offset((($p) + 4) as isize)).b16.s3
    };
}

macro_rules! NATIVE_NODE_font {
    ($p:expr) => {
        (*mem.offset((($p) + 4) as isize)).b16.s2
    };
}

macro_rules! NATIVE_NODE_length {
    ($p:expr) => {
        (*mem.offset((($p) + 4) as isize)).b16.s1
    };
}

macro_rules! NATIVE_NODE_glyph {
    ($p:expr) => {
        (*mem.offset((($p) + 4) as isize)).b16.s1
    };
}

macro_rules! NATIVE_NODE_glyph_count {
    ($p:expr) => {
        (*mem.offset((($p) + 4) as isize)).b16.s0
    };
}

macro_rules! GLUE_SPEC_ref_count {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b32.s1
    };
}

macro_rules! GLUE_SPEC_stretch_order {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b16.s1
    };
}

macro_rules! GLUE_SPEC_shrink_order {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b16.s0
    };
}

macro_rules! GLUE_SPEC_stretch {
    ($p:expr) => {
        (*mem.offset((($p) + 2) as isize)).b32.s1
    };
}

macro_rules! GLUE_SPEC_shrink {
    ($p:expr) => {
        (*mem.offset((($p) + 3) as isize)).b32.s1
    };
}

macro_rules! GLUEPAR {
    (left_skip) => {
        (*eqtb.offset((GLUE_BASE + GLUE_PAR__left_skip) as isize)).b32.s1
    };
    (right_skip) => {
        (*eqtb.offset((GLUE_BASE + GLUE_PAR__right_skip) as isize)).b32.s1
    };
}

macro_rules! LOCAL {
    (par_shape) => {
        (*eqtb.offset((LOCAL_BASE + LOCAL__par_shape) as isize)).b32.s1
    };
}

macro_rules! DIMENPAR {
    (hsize) => {
        (*eqtb.offset((DIMEN_BASE + DIMEN_PAR__hsize) as isize)).b32.s1
    };
    (hang_indent) => {
        (*eqtb.offset((DIMEN_BASE + DIMEN_PAR__hang_indent) as isize)).b32.s1
    };
    (emergency_stretch) => {
        (*eqtb.offset((DIMEN_BASE + DIMEN_PAR__emergency_stretch) as isize)).b32.s1
    };
}

macro_rules! INTPAR {
    (pretolerance) => {
        (*eqtb.offset((INT_BASE + INT_PAR__pretolerance) as isize)).b32.s1
    };
    (tolerance) => {
        (*eqtb.offset((INT_BASE + INT_PAR__tolerance) as isize)).b32.s1
    };
    (line_penalty) => {
        (*eqtb.offset((INT_BASE + INT_PAR__line_penalty) as isize)).b32.s1
    };
    (hyphen_penalty) => {
        (*eqtb.offset((INT_BASE + INT_PAR__hyphen_penalty) as isize)).b32.s1
    };
    (ex_hyphen_penalty) => {
        (*eqtb.offset((INT_BASE + INT_PAR__ex_hyphen_penalty) as isize)).b32.s1
    };
    (club_penalty) => {
        (*eqtb.offset((INT_BASE + INT_PAR__club_penalty) as isize)).b32.s1
    };
    (widow_penalty) => {
        (*eqtb.offset((INT_BASE + INT_PAR__widow_penalty) as isize)).b32.s1
    };
    (display_widow_penalty) => {
        (*eqtb.offset((INT_BASE + INT_PAR__display_widow_penalty) as isize)).b32.s1
    };
    (broken_penalty) => {
        (*eqtb.offset((INT_BASE + INT_PAR__broken_penalty) as isize)).b32.s1
    };
    (inter_line_penalty) => {
        (*eqtb.offset((INT_BASE + INT_PAR__inter_line_penalty) as isize)).b32.s1
    };
    (double_hyphen_demerits) => {
        (*eqtb.offset((INT_BASE + INT_PAR__double_hyphen_demerits) as isize)).b32.s1
    };
    (final_hyphen_demerits) => {
        (*eqtb.offset((INT_BASE + INT_PAR__final_hyphen_demerits) as isize)).b32.s1
    };
    (adj_demerits) => {
        (*eqtb.offset((INT_BASE + INT_PAR__adj_demerits) as isize)).b32.s1
    };
    (looseness) => {
        (*eqtb.offset((INT_BASE + INT_PAR__looseness) as isize)).b32.s1
    };
    (uc_hyph) => {
        (*eqtb.offset((INT_BASE + INT_PAR__uc_hyph) as isize)).b32.s1
    };
    (hang_after) => {
        (*eqtb.offset((INT_BASE + INT_PAR__hang_after) as isize)).b32.s1
    };
    (last_line_fit) => {
        (*eqtb.offset((INT_BASE + INT_PAR__last_line_fit) as isize)).b32.s1
    };
    (xetex_protrude_chars) => {
        (*eqtb.offset((INT_BASE + INT_PAR__xetex_protrude_chars) as isize)).b32.s1
    };
    (texxet) => {
        (*eqtb.offset((INT_BASE + INT_PAR__texxet) as isize)).b32.s1
    };
    (xetex_use_glyph_metrics) => {
        (*eqtb.offset((INT_BASE + INT_PAR__xetex_use_glyph_metrics) as isize)).b32.s1
    };
}

macro_rules! ETEX_PENALTIES_PAR {
    (inter_line_penalties) => {
        (*eqtb.offset((ETEX_PEN_BASE + ETEX_PENALTIES_PAR__inter_line_penalties) as isize)).b32.s1
    };
    (club_penalties) => {
        (*eqtb.offset((ETEX_PEN_BASE + ETEX_PENALTIES_PAR__club_penalties) as isize)).b32.s1
    };
    (widow_penalties) => {
        (*eqtb.offset((ETEX_PEN_BASE + ETEX_PENALTIES_PAR__widow_penalties) as isize)).b32.s1
    };
    (display_widow_penalties) => {
        (*eqtb.offset((ETEX_PEN_BASE + ETEX_PENALTIES_PAR__display_widow_penalties) as isize))
            .b32
            .s1
    };
}

macro_rules! LC_CODE {
    ($c:expr) => {
        (*eqtb.offset((LC_CODE_BASE + ($c) as i32) as isize)).b32.s1
    };
}

#[inline]
pub unsafe fn native_node_text(p: i32) -> *mut u16 {
    mem.offset((p + NATIVE_NODE_SIZE) as isize) as *mut u16
}

#[inline]
pub unsafe fn font_character_info(f: internal_font_number, c: i32) -> b16x4 {
    (*font_info.offset((*char_base.offset(f as isize) + c) as isize)).b16
}

#[inline]
pub unsafe fn font_charinfo_width(f: internal_font_number, info: b16x4) -> scaled_t {
    (*font_info.offset((*width_base.offset(f as isize) + info.s3 as i32) as isize)).b32.s1
}

#[inline]
pub unsafe fn font_character_width(f: internal_font_number, c: i32) -> scaled_t {
    font_charinfo_width(f, font_character_info(f, c))
}

#[inline]
pub fn odd(x: i32) -> bool {
    (x & 1) != 0
}

#[inline]
pub unsafe fn is_char_node(p: i32) -> bool {
    p >= hi_mem_min
}

#[inline]
pub unsafe fn is_non_discardable_node(p: i32) -> bool {
    (NODE_type!(p) as i32) < MATH_NODE
}

unsafe extern "C" {
    pub static mut pack_begin_line: i32;
    pub static mut cur_list: list_state_record;
    pub static mut temp_ptr: i32;
    pub static mut avail: i32;
    pub static mut hi_mem_min: i32;
    pub static mut active_width: [scaled_t; 7];
    pub static mut cur_lang: u8;
    pub static mut hyph_index: trie_pointer;
    pub static mut trie_not_ready: bool;
    pub static mut trie_trc: *mut u16;
    pub static mut trie_trl: *mut trie_pointer;
    pub static mut trie_tro: *mut trie_pointer;
    pub static mut hyph_start: trie_pointer;
    pub static mut font_in_short_display: i32;
    pub static mut global_prev_p: i32;
    pub static mut first_p: i32;
    pub static mut xtx_ligature_present: bool;
    pub static mut hyphen_char: *mut i32;
    pub static mut font_bchar: *mut nine_bits;
    pub static mut bchar_label: *mut font_index;
    pub static mut font_info: *mut memory_word;
    pub static mut char_base: *mut i32;
    pub static mut width_base: *mut i32;
    pub static mut height_base: *mut i32;
    pub static mut depth_base: *mut i32;
    pub static mut italic_base: *mut i32;
    pub static mut lig_kern_base: *mut i32;
    pub static mut kern_base: *mut i32;
    pub static mut arith_error: bool;
    pub static mut adjust_tail: i32;
    pub static mut pre_adjust_tail: i32;
    pub static mut just_box: i32;
    pub static mut semantic_pagination_enabled: bool;
    pub static mut last_rightmost_char: i32;
    pub static mut last_leftmost_char: i32;
    pub static mut hyf: [u8; 4097];
    pub static mut hc: [i32; 4099];
    pub static mut hu: [i32; 4097];
    pub static mut hyph_word: *mut str_number;
    pub static mut hyph_list: *mut i32;
    pub static mut hyph_link: *mut hyph_pointer;
    pub static mut str_start: *mut pool_pointer;
    pub static mut str_pool: *mut packed_UTF16_code;
    pub static mut max_hyph_char: i32;
    pub static mut op_start: [i32; 256];
    pub static mut hyf_distance: [small_number; TRIE_OP_SIZE as usize + 1];
    pub static mut hyf_num: [small_number; TRIE_OP_SIZE as usize + 1];
    pub static mut hyf_next: [trie_opcode; TRIE_OP_SIZE as usize + 1];
    pub static mut hyphen_passed: small_number;
    pub static mut init_list: i32;
    pub static mut init_lig: bool;
    pub static mut init_lft: bool;
    pub static mut cur_l: i32;
    pub static mut cur_r: i32;
    pub static mut cur_q: i32;
    pub static mut lig_stack: i32;
    pub static mut ligature_present: bool;
    pub static mut lft_hit: bool;
    pub static mut rt_hit: bool;
    pub static mut hlist_stack_level: i16;
    pub static mut hlist_stack: [i32; MAX_HLIST_STACK + 1];
    pub static mut help_ptr: u8;
    pub static mut help_line: [*const std::ffi::c_char; 6];
    pub static mut mem: *mut memory_word;
    pub static mut eqtb: *mut memory_word;
    pub static mut hf: internal_font_number;

    pub fn new_penalty(m: i32) -> i32;
    pub fn delete_glue_ref(p: i32);
    pub fn flush_node_list(p: i32);
    pub fn new_param_glue(n: small_number) -> i32;
    pub fn pop_nest();
    pub fn init_trie();
    pub fn get_node(s: i32) -> i32;
    pub fn free_node(p: i32, s: i32);
    pub fn effective_char(init_font: bool, f: internal_font_number, uc: UTF16_code) -> i32;
    pub fn max_hyphenatable_length() -> i32;
    pub fn new_native_word_node(f: internal_font_number, n: i32) -> i32;
    pub fn new_spec(p: i32) -> i32;
    pub fn new_math(w: scaled_t, s: small_number) -> i32;
    pub fn get_avail() -> i32;
    pub fn prev_rightmost(s: i32, e: i32) -> i32;
    pub fn char_pw(p: i32, side: small_number) -> scaled_t;
    pub fn new_margin_kern(w: scaled_t, p: i32, side: small_number) -> i32;
    pub fn prepare_native_line_layouts(lines: *const i32, count: usize) -> *mut PreparedLayouts;
    pub fn ttxl_release_prepared_layouts(layouts: *mut PreparedLayouts);
    pub fn hpack(p: i32, w: scaled_t, m: small_number) -> i32;
    pub fn append_to_vlist(b: i32);
    pub fn fract(x: i32, n: i32, d: i32, max_answer: i32) -> i32;
    pub fn badness(t: scaled_t, s: scaled_t) -> i32;
    pub fn length(s: str_number) -> i32;
    pub fn new_disc() -> i32;
    pub fn new_native_character(f: internal_font_number, c: UnicodeScalar) -> i32;
    pub fn new_character(f: internal_font_number, c: UTF16_code) -> i32;
    pub fn flush_list(p: i32);
    pub fn new_lig_item(c: u16) -> i32;
    pub fn new_ligature(f: internal_font_number, c: u16, q: i32) -> i32;
    pub fn new_kern(w: scaled_t) -> i32;
    pub fn error_here_with_diagnostic(msg: *const std::ffi::c_char) -> *mut tectonic_bridge_core::Diagnostic;
    pub fn capture_to_diagnostic(diagnostic: *mut tectonic_bridge_core::Diagnostic);
    pub fn error();
    pub fn pdf_error(t: *const std::ffi::c_char, p: *const std::ffi::c_char) -> !;
    pub fn confusion(s: *const std::ffi::c_char) -> !;
}

pub unsafe fn set_native_metrics(p: i32, use_glyph_metrics: bool) {
    unsafe {
        super::xetex_ext::measure_native_node(mem.add(p as usize).cast(), use_glyph_metrics.into());
    }
}

static mut passive: i32 = 0;
static mut cur_active_width: [scaled_t; 7] = [0; 7];
static mut background: [scaled_t; 7] = [0; 7];
static mut break_width: [scaled_t; 7] = [0; 7];
static mut best_place: [i32; 4] = [0; 4];
static mut best_pl_line: [i32; 4] = [0; 4];
static mut disc_width: scaled_t = 0;
static mut no_shrink_error_yet: bool = false;
static mut cur_p: i32 = 0;
static mut second_pass: bool = false;
static mut final_pass: bool = false;
static mut threshold: i32 = 0;
static mut minimal_demerits: [i32; 4] = [0; 4];
static mut minimum_demerits: i32 = 0;
static mut easy_line: i32 = 0;
static mut last_special_line: i32 = 0;
static mut first_width: scaled_t = 0;
static mut second_width: scaled_t = 0;
static mut first_indent: scaled_t = 0;
static mut second_indent: scaled_t = 0;
static mut best_bet: i32 = 0;
static mut fewest_demerits: i32 = 0;
static mut best_line: i32 = 0;
static mut actual_looseness: i32 = 0;
static mut line_diff: i32 = 0;
static mut hn: small_number = 0;
static mut ha: i32 = 0;
static mut hb: i32 = 0;
static mut hyf_char: i32 = 0;
static mut init_cur_lang: u8 = 0;
static mut l_hyf: i32 = 0;
static mut r_hyf: i32 = 0;
static mut init_l_hyf: i32 = 0;
static mut init_r_hyf: i32 = 0;
static mut hyf_bchar: i32 = 0;
static mut last_line_fill: i32 = 0;
static mut do_last_line_fit: bool = false;
static mut active_node_size: small_number = 0;
static mut fill_width: [scaled_t; 3] = [0; 3];
static mut best_pl_short: [scaled_t; 4] = [0; 4];
static mut best_pl_glue: [scaled_t; 4] = [0; 4];

#[unsafe(export_name = "get_native_usv")]
pub unsafe extern "C" fn get_native_usv(p: i32, i: i32) -> UnicodeScalar {
    let c = *native_node_text(p).offset(i as isize) as i32;
    if (0xD800..0xDC00).contains(&c) {
        0x10000 + (c - 0xD800) * 0x400 + *native_node_text(p).offset((i + 1) as isize) as i32 - 0xDC00
    } else {
        c
    }
}

/* Break a paragraph into lines (XTTP:843).
 *
 * d: true if we are breaking a partial paragraph preceding display math mode
 *
 * Should only be called in horizontal mode. Will leave horizontal and place
 * the output in the enclosing vertical list.
 *
 * `cur_list.head` is the non-empty hlist to be broken. `prev_graf` tells the
 * starting line number (0 unless we're continuing after display math). After
 * completion, `just_box` will point to the final box created.
 */
#[unsafe(export_name = "line_break")]
pub unsafe extern "C" fn line_break(d: bool) {
    let mut auto_breaking: bool;
    let mut prev_p: i32;
    let mut q: i32;
    let mut r: i32;
    let mut s: i32;
    let mut prev_s: i32;
    let mut f: internal_font_number;
    let mut j: small_number;
    let mut c: UnicodeScalar;
    let mut l: i32;
    let mut i: i32;
    let mut for_end_1: i32;

    pack_begin_line = cur_list.mode_line;

    LLIST_link!(TEMP_HEAD) = LLIST_link!(cur_list.head);

    // Remove trailing space or glue if present; add infinite penalty then par_fill_skip
    if is_char_node(cur_list.tail) {
        let pen = new_penalty(INF_PENALTY);
        LLIST_link!(cur_list.tail) = pen;
        cur_list.tail = pen;
    } else if (NODE_type!(cur_list.tail) as i32) != GLUE_NODE {
        let pen = new_penalty(INF_PENALTY);
        LLIST_link!(cur_list.tail) = pen;
        cur_list.tail = pen;
    } else {
        NODE_type!(cur_list.tail) = PENALTY_NODE as u16;
        delete_glue_ref(GLUE_NODE_glue_ptr!(cur_list.tail));
        flush_node_list(GLUE_NODE_leader_ptr!(cur_list.tail));
        PENALTY_NODE_penalty!(cur_list.tail) = INF_PENALTY;
    }

    last_line_fill = new_param_glue(GLUE_PAR__par_fill_skip as small_number);
    LLIST_link!(cur_list.tail) = last_line_fill;

    init_cur_lang = (cur_list.prev_graf % 65536) as u8;
    init_l_hyf = cur_list.prev_graf / 0x0400000;
    init_r_hyf = (cur_list.prev_graf / 65536) % 64;

    pop_nest();

    no_shrink_error_yet = true;

    if GLUE_SPEC_shrink_order!(GLUEPAR!(left_skip)) as i32 != NORMAL && GLUE_SPEC_shrink!(GLUEPAR!(left_skip)) != 0 {
        GLUEPAR!(left_skip) = finite_shrink(GLUEPAR!(left_skip));
    }

    if GLUE_SPEC_shrink_order!(GLUEPAR!(right_skip)) as i32 != NORMAL && GLUE_SPEC_shrink!(GLUEPAR!(right_skip)) != 0 {
        GLUEPAR!(right_skip) = finite_shrink(GLUEPAR!(right_skip));
    }

    q = GLUEPAR!(left_skip);
    r = GLUEPAR!(right_skip);

    background[1] = BOX_width!(q) + BOX_width!(r);
    background[2] = 0;
    background[3] = 0;
    background[4] = 0;
    background[5] = 0;
    background[2 + GLUE_SPEC_stretch_order!(q) as usize] = GLUE_SPEC_stretch!(q);
    background[2 + GLUE_SPEC_stretch_order!(r) as usize] += GLUE_SPEC_stretch!(r);
    background[6] = GLUE_SPEC_shrink!(q) + GLUE_SPEC_shrink!(r);

    // 1631: check for special treatment of last line of paragraph (\lastlinefit > 0)
    do_last_line_fit = false;
    active_node_size = ACTIVE_NODE_SIZE_NORMAL;

    if INTPAR!(last_line_fit) > 0 {
        q = GLUE_NODE_glue_ptr!(last_line_fill);

        if GLUE_SPEC_stretch!(q) > 0 && GLUE_SPEC_stretch_order!(q) as i32 > NORMAL {
            if background[3] == 0 && background[4] == 0 && background[5] == 0 {
                do_last_line_fit = true;
                active_node_size = ACTIVE_NODE_SIZE_EXTENDED;
                fill_width[0] = 0;
                fill_width[1] = 0;
                fill_width[2] = 0;
                fill_width[(GLUE_SPEC_stretch_order!(q) - 1) as usize] = GLUE_SPEC_stretch!(q);
            }
        }
    }

    minimum_demerits = AWFUL_BAD;
    minimal_demerits[TIGHT_FIT] = AWFUL_BAD;
    minimal_demerits[DECENT_FIT] = AWFUL_BAD;
    minimal_demerits[LOOSE_FIT] = AWFUL_BAD;
    minimal_demerits[VERY_LOOSE_FIT] = AWFUL_BAD;

    // Prep relating to par_shape (877)
    if LOCAL!(par_shape) == TEX_NULL {
        if DIMENPAR!(hang_indent) == 0 {
            last_special_line = 0;
            second_width = DIMENPAR!(hsize);
            second_indent = 0;
        } else {
            last_special_line = INTPAR!(hang_after).abs();

            if INTPAR!(hang_after) < 0 {
                first_width = DIMENPAR!(hsize) - DIMENPAR!(hang_indent).abs();
                if DIMENPAR!(hang_indent) >= 0 {
                    first_indent = DIMENPAR!(hang_indent);
                } else {
                    first_indent = 0;
                }
                second_width = DIMENPAR!(hsize);
                second_indent = 0;
            } else {
                first_width = DIMENPAR!(hsize);
                first_indent = 0;
                second_width = DIMENPAR!(hsize) - DIMENPAR!(hang_indent).abs();
                if DIMENPAR!(hang_indent) >= 0 {
                    second_indent = DIMENPAR!(hang_indent);
                } else {
                    second_indent = 0;
                }
            }
        }
    } else {
        last_special_line = LLIST_info!(LOCAL!(par_shape)) - 1;
        second_width = (*mem.offset((LOCAL!(par_shape) + 2 * (last_special_line + 1)) as isize)).b32.s1;
        second_indent = (*mem.offset((LOCAL!(par_shape) + 2 * last_special_line + 1) as isize)).b32.s1;
    }

    if INTPAR!(looseness) == 0 {
        easy_line = last_special_line;
    } else {
        easy_line = MAX_HALFWORD;
    }

    // Start finding optimal breakpoints (892)
    threshold = INTPAR!(pretolerance);

    if threshold >= 0 {
        second_pass = false;
        final_pass = false;
    } else {
        threshold = INTPAR!(tolerance);
        second_pass = true;
        final_pass = DIMENPAR!(emergency_stretch) <= 0;
    }

    'outer: loop {
        if threshold > INF_BAD {
            threshold = INF_BAD;
        }

        if second_pass {
            if trie_not_ready {
                init_trie();
            }

            cur_lang = init_cur_lang;
            l_hyf = init_l_hyf;
            r_hyf = init_r_hyf;

            if *trie_trc.offset((hyph_start + cur_lang as i32) as isize) as u32 != cur_lang as u32 {
                hyph_index = 0;
            } else {
                hyph_index = *trie_trl.offset((hyph_start + cur_lang as i32) as isize);
            }
        }

        q = get_node(active_node_size as i32);
        NODE_type!(q) = UNHYPHENATED;
        ACTIVE_NODE_fitness!(q) = DECENT_FIT as u16;
        LLIST_link!(q) = LAST_ACTIVE;
        ACTIVE_NODE_break_node!(q) = TEX_NULL;
        ACTIVE_NODE_line_number!(q) = cur_list.prev_graf + 1;
        ACTIVE_NODE_total_demerits!(q) = 0;
        LLIST_link!(ACTIVE_LIST) = q;

        if do_last_line_fit {
            ACTIVE_NODE_shortfall!(q) = 0;
            ACTIVE_NODE_glue!(q) = 0;
        }

        active_width[1] = background[1];
        active_width[2] = background[2];
        active_width[3] = background[3];
        active_width[4] = background[4];
        active_width[5] = background[5];
        active_width[6] = background[6];
        passive = TEX_NULL;
        font_in_short_display = 0;
        cur_p = LLIST_link!(TEMP_HEAD);
        auto_breaking = true;

        prev_p = cur_p;
        global_prev_p = cur_p;
        first_p = cur_p;

        while cur_p != TEX_NULL && LLIST_link!(ACTIVE_LIST) != LAST_ACTIVE {
            if is_char_node(cur_p) {
                prev_p = cur_p;
                global_prev_p = cur_p;

                loop {
                    f = CHAR_NODE_font!(cur_p) as i32;
                    let eff_char = effective_char(true, f, CHAR_NODE_character!(cur_p) as u16);
                    active_width[1] += font_character_width(f, eff_char);
                    cur_p = LLIST_link!(cur_p);
                    if !is_char_node(cur_p) {
                        break;
                    }
                }
            }

            match NODE_type!(cur_p) as i32 {
                HLIST_NODE | VLIST_NODE | RULE_NODE => {
                    active_width[1] += BOX_width!(cur_p);
                }

                WHATSIT_NODE => {
                    if NODE_subtype!(cur_p) as i32 == LANGUAGE_NODE {
                        cur_lang = LANGUAGE_NODE_what_lang!(cur_p) as u8;
                        l_hyf = LANGUAGE_NODE_what_lhm!(cur_p) as i32;
                        r_hyf = LANGUAGE_NODE_what_rhm!(cur_p) as i32;

                        if *trie_trc.offset((hyph_start + cur_lang as i32) as isize) as u32 != cur_lang as u32 {
                            hyph_index = 0;
                        } else {
                            hyph_index = *trie_trl.offset((hyph_start + cur_lang as i32) as isize);
                        }
                    } else if matches!(
                        NODE_subtype!(cur_p) as i32,
                        NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT | GLYPH_NODE | PIC_NODE | PDF_NODE
                    ) {
                        active_width[1] += BOX_width!(cur_p);
                    }
                }

                GLUE_NODE => {
                    if auto_breaking {
                        if is_char_node(prev_p) {
                            try_break(0, UNHYPHENATED as small_number);
                        } else if is_non_discardable_node(prev_p) {
                            try_break(0, UNHYPHENATED as small_number);
                        } else if NODE_type!(prev_p) as i32 == KERN_NODE && NODE_subtype!(prev_p) as i32 != EXPLICIT {
                            try_break(0, UNHYPHENATED as small_number);
                        }
                    }

                    q = GLUE_NODE_glue_ptr!(cur_p);

                    if GLUE_SPEC_shrink_order!(q) as i32 != NORMAL && GLUE_SPEC_shrink!(q) != 0 {
                        q = finite_shrink(q);
                        GLUE_NODE_glue_ptr!(cur_p) = q;
                    }

                    active_width[1] += BOX_width!(q);
                    active_width[2 + GLUE_SPEC_stretch_order!(q) as usize] += GLUE_SPEC_stretch!(q);
                    active_width[6] += GLUE_SPEC_shrink!(q);

                    if second_pass && auto_breaking {
                        'done1: {
                            prev_s = cur_p;
                            s = LLIST_link!(prev_s);

                            if s == TEX_NULL {
                                break 'done1;
                            }

                            'skip_to_ha: loop {
                                if is_char_node(s) {
                                    c = CHAR_NODE_character!(s) as i32;
                                    hf = CHAR_NODE_font!(s) as i32;
                                } else if NODE_type!(s) as i32 == LIGATURE_NODE {
                                    if LIGATURE_NODE_lig_ptr!(s) == TEX_NULL {
                                        prev_s = s;
                                        s = LLIST_link!(prev_s);
                                        continue;
                                    }

                                    q = LIGATURE_NODE_lig_ptr!(s);
                                    c = CHAR_NODE_character!(q) as i32;
                                    hf = CHAR_NODE_font!(q) as i32;
                                } else if NODE_type!(s) as i32 == KERN_NODE && NODE_subtype!(s) as i32 == NORMAL {
                                    prev_s = s;
                                    s = LLIST_link!(prev_s);
                                    continue;
                                } else if NODE_type!(s) as i32 == MATH_NODE && NODE_subtype!(s) as i32 >= L_CODE {
                                    prev_s = s;
                                    s = LLIST_link!(prev_s);
                                    continue;
                                } else if NODE_type!(s) as i32 == WHATSIT_NODE {
                                    if NODE_subtype!(s) as i32 == NATIVE_WORD_NODE || NODE_subtype!(s) as i32 == NATIVE_WORD_NODE_AT {
                                        let mut l_idx = 0;
                                        while l_idx < NATIVE_NODE_length!(s) as i32 {
                                            c = get_native_usv(s, l_idx);
                                            if LC_CODE!(c) != 0 {
                                                hf = NATIVE_NODE_font!(s) as i32;
                                                prev_s = s;
                                                if LC_CODE!(c) == c || INTPAR!(uc_hyph) > 0 {
                                                    break 'skip_to_ha;
                                                } else {
                                                    break 'done1;
                                                }
                                            }

                                            if c >= 65536 {
                                                l_idx += 1;
                                            }
                                            l_idx += 1;
                                        }
                                    }

                                    if NODE_subtype!(s) as i32 == LANGUAGE_NODE {
                                        cur_lang = LANGUAGE_NODE_what_lang!(s) as u8;
                                        l_hyf = LANGUAGE_NODE_what_lhm!(s) as i32;
                                        r_hyf = LANGUAGE_NODE_what_rhm!(s) as i32;

                                        if *trie_trc.offset((hyph_start + cur_lang as i32) as isize) as u32 != cur_lang as u32 {
                                            hyph_index = 0;
                                        } else {
                                            hyph_index = *trie_trl.offset((hyph_start + cur_lang as i32) as isize);
                                        }
                                    }

                                    prev_s = s;
                                    s = LLIST_link!(prev_s);
                                    continue;
                                } else {
                                    break 'done1;
                                }

                                if hyph_index == 0 || c > 255 {
                                    hc[0] = LC_CODE!(c);
                                } else if *trie_trc.offset((hyph_index + c) as isize) as i32 != c {
                                    hc[0] = 0;
                                } else {
                                    hc[0] = *trie_tro.offset((hyph_index + c) as isize) as i32;
                                }

                                if hc[0] != 0 {
                                    if hc[0] == c || INTPAR!(uc_hyph) > 0 {
                                        break 'skip_to_ha;
                                    } else {
                                        break 'done1;
                                    }
                                }

                                prev_s = s;
                                s = LLIST_link!(prev_s);
                            }

                            hyf_char = *hyphen_char.offset(hf as isize);
                            if hyf_char < 0 || hyf_char > BIGGEST_CHAR {
                                break 'done1;
                            }

                            ha = prev_s;

                            if l_hyf + r_hyf > max_hyphenatable_length() {
                                break 'done1;
                            }

                            if ha != TEX_NULL
                                && ha < hi_mem_min
                                && NODE_type!(ha) as i32 == WHATSIT_NODE
                                && (NODE_subtype!(ha) as i32 == NATIVE_WORD_NODE || NODE_subtype!(ha) as i32 == NATIVE_WORD_NODE_AT)
                            {
                                s = LLIST_link!(ha);

                                'chk_after: loop {
                                    if !is_char_node(s) {
                                        match NODE_type!(s) as i32 {
                                            LIGATURE_NODE => {}
                                            KERN_NODE => {
                                                if NODE_subtype!(s) as i32 != NORMAL {
                                                    break 'chk_after;
                                                }
                                            }
                                            WHATSIT_NODE | GLUE_NODE | PENALTY_NODE | INS_NODE | ADJUST_NODE | MARK_NODE => {
                                                break 'chk_after;
                                            }
                                            _ => {
                                                break 'done1;
                                            }
                                        }
                                    }
                                    s = LLIST_link!(s);
                                }

                                hn = 0;

                                'restart: loop {
                                    for_end_1 = NATIVE_NODE_length!(ha) as i32;

                                    let mut l_idx = 0;
                                    while l_idx < for_end_1 {
                                        c = get_native_usv(ha, l_idx);

                                        if hyph_index == 0 || c > 255 {
                                            hc[0] = LC_CODE!(c);
                                        } else if *trie_trc.offset((hyph_index + c) as isize) as i32 != c {
                                            hc[0] = 0;
                                        } else {
                                            hc[0] = *trie_tro.offset((hyph_index + c) as isize) as i32;
                                        }

                                        if hc[0] == 0 {
                                            if hn > 0 {
                                                q = new_native_word_node(hf, NATIVE_NODE_length!(ha) as i32 - l_idx);
                                                NODE_subtype!(q) = NODE_subtype!(ha);

                                                for i_idx in l_idx..NATIVE_NODE_length!(ha) as i32 {
                                                    *native_node_text(q).offset((i_idx - l_idx) as isize) =
                                                        *native_node_text(ha).offset(i_idx as isize);
                                                }

                                                set_native_metrics(q, INTPAR!(xetex_use_glyph_metrics) > 0);
                                                LLIST_link!(q) = LLIST_link!(ha);
                                                LLIST_link!(ha) = q;
                                                NATIVE_NODE_length!(ha) = l_idx as u16;
                                                set_native_metrics(ha, INTPAR!(xetex_use_glyph_metrics) > 0);
                                                break 'restart;
                                            }
                                        } else if hn == 0 && l_idx > 0 {
                                            q = new_native_word_node(hf, NATIVE_NODE_length!(ha) as i32 - l_idx);
                                            NODE_subtype!(q) = NODE_subtype!(ha);

                                            for i_idx in l_idx..NATIVE_NODE_length!(ha) as i32 {
                                                *native_node_text(q).offset((i_idx - l_idx) as isize) = *native_node_text(ha).offset(i_idx as isize);
                                            }

                                            set_native_metrics(q, INTPAR!(xetex_use_glyph_metrics) > 0);
                                            LLIST_link!(q) = LLIST_link!(ha);
                                            LLIST_link!(ha) = q;
                                            NATIVE_NODE_length!(ha) = l_idx as u16;
                                            set_native_metrics(ha, INTPAR!(xetex_use_glyph_metrics) > 0);
                                            ha = LLIST_link!(ha);
                                            continue 'restart;
                                        } else if hn as i32 == max_hyphenatable_length() {
                                            break 'restart;
                                        } else {
                                            hn += 1;

                                            if c < 65536 {
                                                hu[hn as usize] = c;
                                                hc[hn as usize] = hc[0];
                                            } else {
                                                hu[hn as usize] = (c - 65536) / 1024 + 0xD800;
                                                hc[hn as usize] = (hc[0] - 65536) / 1024 + 0xD800;
                                                hn += 1;
                                                hu[hn as usize] = c % 1024 + 0xDC00;
                                                hc[hn as usize] = hc[0] % 1024 + 0xDC00;
                                                l_idx += 1;
                                            }

                                            hyf_bchar = TOO_BIG_CHAR;
                                        }

                                        l_idx += 1;
                                    }
                                    break 'restart;
                                }
                            } else {
                                hn = 0;

                                'skip_hb: loop {
                                    if is_char_node(s) {
                                        if CHAR_NODE_font!(s) as i32 != hf {
                                            break 'skip_hb;
                                        }

                                        hyf_bchar = CHAR_NODE_character!(s) as i32;
                                        c = hyf_bchar;

                                        if hyph_index == 0 || c > 255 {
                                            hc[0] = LC_CODE!(c);
                                        } else if *trie_trc.offset((hyph_index + c) as isize) as i32 != c {
                                            hc[0] = 0;
                                        } else {
                                            hc[0] = *trie_tro.offset((hyph_index + c) as isize) as i32;
                                        }

                                        if hc[0] == 0 || hc[0] > max_hyph_char || hn as i32 == max_hyphenatable_length() {
                                            break 'skip_hb;
                                        }

                                        hb = s;
                                        hn += 1;
                                        hu[hn as usize] = c;
                                        hc[hn as usize] = hc[0];
                                        hyf_bchar = TOO_BIG_CHAR;
                                    } else if NODE_type!(s) as i32 == LIGATURE_NODE {
                                        if LIGATURE_NODE_lig_font!(s) as i32 != hf {
                                            break 'skip_hb;
                                        }

                                        j = hn;
                                        q = LIGATURE_NODE_lig_ptr!(s);

                                        if q > TEX_NULL {
                                            hyf_bchar = CHAR_NODE_character!(q) as i32;
                                        }

                                        while q > TEX_NULL {
                                            c = CHAR_NODE_character!(q) as i32;

                                            if hyph_index == 0 || c > 255 {
                                                hc[0] = LC_CODE!(c);
                                            } else if *trie_trc.offset((hyph_index + c) as isize) as i32 != c {
                                                hc[0] = 0;
                                            } else {
                                                hc[0] = *trie_tro.offset((hyph_index + c) as isize) as i32;
                                            }

                                            if hc[0] == 0 || hc[0] > max_hyph_char || j as i32 == max_hyphenatable_length() {
                                                break 'skip_hb;
                                            }

                                            j += 1;
                                            hu[j as usize] = c;
                                            hc[j as usize] = hc[0];
                                            q = LLIST_link!(q);
                                        }

                                        hb = s;
                                        hn = j;

                                        if odd(NODE_subtype!(s) as i32) {
                                            hyf_bchar = *font_bchar.offset(hf as isize) as i32;
                                        } else {
                                            hyf_bchar = TOO_BIG_CHAR;
                                        }
                                    } else if NODE_type!(s) as i32 == KERN_NODE && NODE_subtype!(s) as i32 == NORMAL {
                                        hb = s;
                                        hyf_bchar = *font_bchar.offset(hf as isize) as i32;
                                    } else {
                                        break 'skip_hb;
                                    }

                                    s = LLIST_link!(s);
                                }
                            }

                            if (hn as i32) < l_hyf + r_hyf {
                                break 'done1;
                            }

                            loop {
                                if !is_char_node(s) {
                                    match NODE_type!(s) as i32 {
                                        LIGATURE_NODE => {}
                                        KERN_NODE => {
                                            if NODE_subtype!(s) as i32 != NORMAL {
                                                break;
                                            }
                                        }
                                        WHATSIT_NODE | GLUE_NODE | PENALTY_NODE | INS_NODE | ADJUST_NODE | MARK_NODE => {
                                            break;
                                        }
                                        MATH_NODE => {
                                            if NODE_subtype!(s) as i32 >= L_CODE {
                                                break;
                                            } else {
                                                break 'done1;
                                            }
                                        }
                                        _ => {
                                            break 'done1;
                                        }
                                    }
                                }

                                s = LLIST_link!(s);
                            }

                            hyphenate();
                        }
                    }
                }

                KERN_NODE => {
                    if NODE_subtype!(cur_p) as i32 == EXPLICIT {
                        if !is_char_node(LLIST_link!(cur_p)) && auto_breaking {
                            if NODE_type!(LLIST_link!(cur_p)) as i32 == GLUE_NODE {
                                try_break(0, UNHYPHENATED as small_number);
                            }
                        }
                        active_width[1] += BOX_width!(cur_p);
                    } else {
                        active_width[1] += BOX_width!(cur_p);
                    }
                }

                LIGATURE_NODE => {
                    f = LIGATURE_NODE_lig_font!(cur_p) as i32;
                    xtx_ligature_present = true;
                    active_width[1] += font_character_width(f, effective_char(true, f, LIGATURE_NODE_lig_char!(cur_p) as u16));
                }

                DISC_NODE => {
                    s = DISCRETIONARY_NODE_pre_break!(cur_p);
                    disc_width = 0;

                    if s == TEX_NULL {
                        try_break(INTPAR!(ex_hyphen_penalty), HYPHENATED as small_number);
                    } else {
                        loop {
                            if is_char_node(s) {
                                f = CHAR_NODE_font!(s) as i32;
                                let eff_char = effective_char(true, f, CHAR_NODE_character!(s) as u16);
                                disc_width += font_character_width(f, eff_char);
                            } else {
                                match NODE_type!(s) as i32 {
                                    LIGATURE_NODE => {
                                        f = LIGATURE_NODE_lig_font!(s) as i32;
                                        xtx_ligature_present = true;
                                        let eff_char = effective_char(true, f, LIGATURE_NODE_lig_char!(s) as u16);
                                        disc_width += font_character_width(f, eff_char);
                                    }

                                    HLIST_NODE | VLIST_NODE | RULE_NODE | KERN_NODE => {
                                        disc_width += BOX_width!(s);
                                    }

                                    WHATSIT_NODE => {
                                        if matches!(
                                            NODE_subtype!(s) as i32,
                                            NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT | GLYPH_NODE | PIC_NODE | PDF_NODE
                                        ) {
                                            disc_width += BOX_width!(s);
                                        } else {
                                            confusion(c"disc3a".as_ptr());
                                        }
                                    }

                                    _ => {
                                        confusion(c"disc3".as_ptr());
                                    }
                                }
                            }

                            s = LLIST_link!(s);
                            if s == TEX_NULL {
                                break;
                            }
                        }

                        active_width[1] += disc_width;
                        try_break(INTPAR!(hyphen_penalty), HYPHENATED as small_number);
                        active_width[1] -= disc_width;
                    }

                    r = DISCRETIONARY_NODE_replace_count!(cur_p) as i32;
                    s = LLIST_link!(cur_p);

                    while r > 0 {
                        if is_char_node(s) {
                            f = CHAR_NODE_font!(s) as i32;
                            let eff_char = effective_char(true, f, CHAR_NODE_character!(s) as u16);
                            active_width[1] += font_character_width(f, eff_char);
                        } else {
                            match NODE_type!(s) as i32 {
                                LIGATURE_NODE => {
                                    f = LIGATURE_NODE_lig_font!(s) as i32;
                                    xtx_ligature_present = true;
                                    let eff_char = effective_char(true, f, LIGATURE_NODE_lig_char!(s) as u16);
                                    active_width[1] += font_character_width(f, eff_char);
                                }

                                HLIST_NODE | VLIST_NODE | RULE_NODE | KERN_NODE => {
                                    active_width[1] += BOX_width!(s);
                                }

                                WHATSIT_NODE => {
                                    if matches!(
                                        NODE_subtype!(s) as i32,
                                        NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT | GLYPH_NODE | PIC_NODE | PDF_NODE
                                    ) {
                                        active_width[1] += BOX_width!(s);
                                    } else {
                                        confusion(c"disc4a".as_ptr());
                                    }
                                }

                                _ => {
                                    confusion(c"disc4".as_ptr());
                                }
                            }
                        }

                        r -= 1;
                        s = LLIST_link!(s);
                    }

                    prev_p = cur_p;
                    global_prev_p = cur_p;
                    cur_p = s;
                    continue;
                }

                MATH_NODE => {
                    if (NODE_subtype!(cur_p) as i32) < L_CODE {
                        auto_breaking = odd(NODE_subtype!(cur_p) as i32);
                    }

                    if !is_char_node(LLIST_link!(cur_p)) && auto_breaking {
                        if NODE_type!(LLIST_link!(cur_p)) as i32 == GLUE_NODE {
                            try_break(0, UNHYPHENATED as small_number);
                        }
                    }

                    active_width[1] += BOX_width!(cur_p);
                }

                PENALTY_NODE => {
                    try_break(PENALTY_NODE_penalty!(cur_p), UNHYPHENATED as small_number);
                }

                MARK_NODE | INS_NODE | ADJUST_NODE => {}

                _ => {
                    confusion(c"paragraph".as_ptr());
                }
            }

            prev_p = cur_p;
            global_prev_p = cur_p;
            cur_p = LLIST_link!(cur_p);
        }

        if cur_p == TEX_NULL {
            try_break(EJECT_PENALTY, HYPHENATED as small_number);

            if LLIST_link!(ACTIVE_LIST) != LAST_ACTIVE {
                r = LLIST_link!(ACTIVE_LIST);
                fewest_demerits = MAX_HALFWORD;

                loop {
                    if NODE_type!(r) != DELTA_NODE {
                        if ACTIVE_NODE_total_demerits!(r) < fewest_demerits {
                            fewest_demerits = ACTIVE_NODE_total_demerits!(r);
                            best_bet = r;
                        }
                    }

                    r = LLIST_link!(r);
                    if r == LAST_ACTIVE {
                        break;
                    }
                }

                best_line = ACTIVE_NODE_line_number!(best_bet);

                if INTPAR!(looseness) == 0 {
                    break 'outer;
                }

                r = LLIST_link!(ACTIVE_LIST);
                actual_looseness = 0;

                loop {
                    if NODE_type!(r) != DELTA_NODE {
                        line_diff = ACTIVE_NODE_line_number!(r) - best_line;

                        if (line_diff < actual_looseness && INTPAR!(looseness) <= line_diff)
                            || (line_diff > actual_looseness && INTPAR!(looseness) >= line_diff)
                        {
                            best_bet = r;
                            actual_looseness = line_diff;
                            fewest_demerits = ACTIVE_NODE_total_demerits!(r);
                        } else if line_diff == actual_looseness && ACTIVE_NODE_total_demerits!(r) < fewest_demerits {
                            best_bet = r;
                            fewest_demerits = ACTIVE_NODE_total_demerits!(r);
                        }
                    }

                    r = LLIST_link!(r);
                    if r == LAST_ACTIVE {
                        break;
                    }
                }

                best_line = ACTIVE_NODE_line_number!(best_bet);

                if actual_looseness == INTPAR!(looseness) || final_pass {
                    break 'outer;
                }
            }
        }

        // 894: clean up memory by removing break nodes
        q = LLIST_link!(ACTIVE_LIST);

        while q != LAST_ACTIVE {
            cur_p = LLIST_link!(q);

            if NODE_type!(q) == DELTA_NODE {
                free_node(q, DELTA_NODE_SIZE);
            } else {
                free_node(q, active_node_size as i32);
            }

            q = cur_p;
        }

        q = passive;

        while q != TEX_NULL {
            cur_p = LLIST_link!(q);
            free_node(q, PASSIVE_NODE_SIZE);
            q = cur_p;
        }

        if !second_pass {
            threshold = INTPAR!(tolerance);
            second_pass = true;
            final_pass = DIMENPAR!(emergency_stretch) <= 0;
        } else {
            background[2] += DIMENPAR!(emergency_stretch);
            final_pass = true;
        }
    }

    if do_last_line_fit {
        if ACTIVE_NODE_shortfall!(best_bet) == 0 {
            do_last_line_fit = false;
        } else {
            q = new_spec(GLUE_NODE_glue_ptr!(last_line_fill));
            delete_glue_ref(GLUE_NODE_glue_ptr!(last_line_fill));
            BOX_width!(q) += ACTIVE_NODE_shortfall!(best_bet) - ACTIVE_NODE_glue!(best_bet);
            GLUE_SPEC_stretch!(q) = 0;
            GLUE_NODE_glue_ptr!(last_line_fill) = q;
        }
    }

    post_line_break(d);

    // Clean up by removing break nodes (894, again)
    q = LLIST_link!(ACTIVE_LIST);

    while q != ACTIVE_LIST {
        let next = LLIST_link!(q);

        if NODE_type!(q) == DELTA_NODE {
            free_node(q, DELTA_NODE_SIZE);
        } else {
            free_node(q, active_node_size as i32);
        }

        q = next;
    }

    q = passive;

    while q != TEX_NULL {
        let next = LLIST_link!(q);
        free_node(q, PASSIVE_NODE_SIZE);
        q = next;
    }

    pack_begin_line = 0;
}

#[derive(Copy, Clone)]
struct PendingLine {
    list: i32,
    width: scaled_t,
    indent: scaled_t,
    penalty: i32,
}

unsafe fn post_line_break(d: bool) {
    let mut q: i32;
    let mut r: i32;
    let mut s: i32;
    let mut p: i32;
    let mut k: i32;
    let mut w: scaled_t;
    let mut glue_break: bool;
    let mut ptmp: i32;
    let mut disc_break: bool;
    let mut post_disc_break: bool;
    let mut cur_width: scaled_t;
    let mut cur_indent: scaled_t;
    let mut t: u16;
    let mut pen: i32;
    let mut cur_line: i32;
    let mut LR_ptr: i32;

    LR_ptr = cur_list.eTeX_aux;

    // Reverse the list of break nodes (907)
    q = ACTIVE_NODE_break_node!(best_bet);
    cur_p = TEX_NULL;

    loop {
        r = q;
        q = PASSIVE_NODE_prev_break!(q);
        PASSIVE_NODE_next_break!(r) = cur_p;
        cur_p = r;
        if q == TEX_NULL {
            break;
        }
    }

    cur_line = cur_list.prev_graf + 1;
    let mut lines: Vec<PendingLine> = Vec::with_capacity((best_line - cur_line).max(0) as usize);

    loop {
        if INTPAR!(texxet) > 0 {
            q = (*mem.offset(TEMP_HEAD as isize)).b32.s1;

            if LR_ptr != TEX_NULL {
                temp_ptr = LR_ptr;
                r = q;

                loop {
                    s = new_math(0, ((*mem.offset(temp_ptr as isize)).b32.s0 - 1) as small_number);
                    (*mem.offset(s as isize)).b32.s1 = r;
                    r = s;
                    temp_ptr = LLIST_link!(temp_ptr);
                    if temp_ptr == TEX_NULL {
                        break;
                    }
                }

                (*mem.offset(TEMP_HEAD as isize)).b32.s1 = r;
            }

            while q != (*mem.offset((cur_p + 1) as isize)).b32.s1 {
                if q < hi_mem_min && (NODE_type!(q) as i32) == MATH_NODE {
                    if odd((*mem.offset(q as isize)).b16.s0 as i32) {
                        if LR_ptr != TEX_NULL
                            && (*mem.offset(LR_ptr as isize)).b32.s0 == (L_CODE * ((*mem.offset(q as isize)).b16.s0 as i32 / L_CODE) + 3)
                        {
                            temp_ptr = LR_ptr;
                            LR_ptr = (*mem.offset(temp_ptr as isize)).b32.s1;
                            (*mem.offset(temp_ptr as isize)).b32.s1 = avail;
                            avail = temp_ptr;
                        }
                    } else {
                        temp_ptr = get_avail();
                        (*mem.offset(temp_ptr as isize)).b32.s0 = L_CODE * ((*mem.offset(q as isize)).b16.s0 as i32 / L_CODE) + 3;
                        (*mem.offset(temp_ptr as isize)).b32.s1 = LR_ptr;
                        LR_ptr = temp_ptr;
                    }
                }

                q = LLIST_link!(q);
            }
        }

        q = PASSIVE_NODE_cur_break!(cur_p);
        disc_break = false;
        post_disc_break = false;
        glue_break = false;

        if q == TEX_NULL {
            q = TEMP_HEAD;

            while LLIST_link!(q) != TEX_NULL {
                q = LLIST_link!(q);
            }
        } else if (NODE_type!(q) as i32) == GLUE_NODE {
            delete_glue_ref(GLUE_NODE_glue_ptr!(q));
            GLUE_NODE_glue_ptr!(q) = GLUEPAR!(right_skip);
            NODE_subtype!(q) = (GLUE_PAR__right_skip + 1) as u16;
            GLUE_SPEC_ref_count!(GLUEPAR!(right_skip)) += 1;
            glue_break = true;
        } else if (NODE_type!(q) as i32) == DISC_NODE {
            t = DISCRETIONARY_NODE_replace_count!(q);

            if t == 0 {
                r = LLIST_link!(q);
            } else {
                r = q;

                while t > 1 {
                    r = LLIST_link!(r);
                    t -= 1;
                }

                s = LLIST_link!(r);
                r = LLIST_link!(s);
                LLIST_link!(s) = TEX_NULL;
                flush_node_list(LLIST_link!(q));
                DISCRETIONARY_NODE_replace_count!(q) = 0;
            }

            if DISCRETIONARY_NODE_post_break!(q) != TEX_NULL {
                s = DISCRETIONARY_NODE_post_break!(q);

                while LLIST_link!(s) != TEX_NULL {
                    s = LLIST_link!(s);
                }

                LLIST_link!(s) = r;

                r = DISCRETIONARY_NODE_post_break!(q);
                DISCRETIONARY_NODE_post_break!(q) = TEX_NULL;
                post_disc_break = true;
            }

            if DISCRETIONARY_NODE_pre_break!(q) != TEX_NULL {
                s = DISCRETIONARY_NODE_pre_break!(q);
                LLIST_link!(q) = s;

                while LLIST_link!(s) != TEX_NULL {
                    s = LLIST_link!(s);
                }

                DISCRETIONARY_NODE_pre_break!(q) = TEX_NULL;
                q = s;
            }

            LLIST_link!(q) = r;
            disc_break = true;
        } else if (NODE_type!(q) as i32) == KERN_NODE {
            BOX_width!(q) = 0;
        } else if (NODE_type!(q) as i32) == MATH_NODE {
            BOX_width!(q) = 0;

            if INTPAR!(texxet) > 0 {
                if odd((*mem.offset(q as isize)).b16.s0 as i32) {
                    if LR_ptr != TEX_NULL
                        && (*mem.offset(LR_ptr as isize)).b32.s0 == (L_CODE * ((*mem.offset(q as isize)).b16.s0 as i32 / L_CODE) + 3)
                    {
                        temp_ptr = LR_ptr;
                        LR_ptr = (*mem.offset(temp_ptr as isize)).b32.s1;
                        (*mem.offset(temp_ptr as isize)).b32.s1 = avail;
                        avail = temp_ptr;
                    }
                } else {
                    temp_ptr = get_avail();
                    (*mem.offset(temp_ptr as isize)).b32.s0 = L_CODE * ((*mem.offset(q as isize)).b16.s0 as i32 / L_CODE) + 3;
                    (*mem.offset(temp_ptr as isize)).b32.s1 = LR_ptr;
                    LR_ptr = temp_ptr;
                }
            }
        }

        if INTPAR!(xetex_protrude_chars) > 0 {
            if disc_break && (is_char_node(q) || (NODE_type!(q) as i32) != DISC_NODE) {
                p = q;
                ptmp = p;
            } else {
                p = prev_rightmost((*mem.offset(TEMP_HEAD as isize)).b32.s1, q);
                ptmp = p;
                p = find_protchar_right((*mem.offset(TEMP_HEAD as isize)).b32.s1, p);
            }

            w = char_pw(p, 1);

            if w != 0 {
                k = new_margin_kern(-w, last_rightmost_char, 1);
                (*mem.offset(k as isize)).b32.s1 = (*mem.offset(ptmp as isize)).b32.s1;
                (*mem.offset(ptmp as isize)).b32.s1 = k;
                if ptmp == q {
                    q = LLIST_link!(q);
                }
            }
        }

        if !glue_break {
            r = new_param_glue(GLUE_PAR__right_skip as small_number);
            LLIST_link!(r) = LLIST_link!(q);
            LLIST_link!(q) = r;
            q = r;
        }

        if INTPAR!(texxet) > 0 {
            if LR_ptr != TEX_NULL {
                s = TEMP_HEAD;
                r = (*mem.offset(s as isize)).b32.s1;

                while r != q {
                    s = r;
                    r = (*mem.offset(s as isize)).b32.s1;
                }

                r = LR_ptr;

                while r != TEX_NULL {
                    temp_ptr = new_math(0, (*mem.offset(r as isize)).b32.s0 as small_number);
                    (*mem.offset(s as isize)).b32.s1 = temp_ptr;
                    s = temp_ptr;
                    r = LLIST_link!(r);
                }

                (*mem.offset(s as isize)).b32.s1 = q;
            }
        }

        // Put \leftskip at the left and detach this line
        r = LLIST_link!(q);
        LLIST_link!(q) = TEX_NULL;
        q = LLIST_link!(TEMP_HEAD);
        LLIST_link!(TEMP_HEAD) = r;

        if INTPAR!(xetex_protrude_chars) > 0 {
            p = q;
            p = find_protchar_left(p, false);
            w = char_pw(p, 0);
            if w != 0 {
                k = new_margin_kern(-w, last_leftmost_char, 0);
                LLIST_link!(k) = q;
                q = k;
            }
        }

        if GLUEPAR!(left_skip) != 0 {
            r = new_param_glue(GLUE_PAR__left_skip as small_number);
            LLIST_link!(r) = q;
            q = r;
        }

        if cur_line > last_special_line {
            cur_width = second_width;
            cur_indent = second_indent;
        } else if LOCAL!(par_shape) == TEX_NULL {
            cur_width = first_width;
            cur_indent = first_indent;
        } else {
            cur_width = (*mem.offset((LOCAL!(par_shape) + 2 * cur_line) as isize)).b32.s1;
            cur_indent = (*mem.offset((LOCAL!(par_shape) + 2 * cur_line - 1) as isize)).b32.s1;
        }

        pen = 0;
        let line_list = q;
        if cur_line + 1 != best_line {
            q = ETEX_PENALTIES_PAR!(inter_line_penalties);

            if q != TEX_NULL {
                r = cur_line;
                if r > PENALTY_NODE_penalty!(q) {
                    r = PENALTY_NODE_penalty!(q);
                }
                pen = PENALTY_NODE_penalty!(q + r);
            } else {
                pen = INTPAR!(inter_line_penalty);
            }

            q = ETEX_PENALTIES_PAR!(club_penalties);

            if q != TEX_NULL {
                r = cur_line - cur_list.prev_graf;
                if r > PENALTY_NODE_penalty!(q) {
                    r = PENALTY_NODE_penalty!(q);
                }
                pen += PENALTY_NODE_penalty!(q + r);
            } else if cur_line == cur_list.prev_graf + 1 {
                pen += INTPAR!(club_penalty);
            }

            if d {
                q = ETEX_PENALTIES_PAR!(display_widow_penalties);
            } else {
                q = ETEX_PENALTIES_PAR!(widow_penalties);
            }

            if q != TEX_NULL {
                r = best_line - cur_line - 1;
                if r > PENALTY_NODE_penalty!(q) {
                    r = PENALTY_NODE_penalty!(q);
                }
                pen += PENALTY_NODE_penalty!(q + r);
            } else if cur_line + 2 == best_line {
                if d {
                    pen += INTPAR!(display_widow_penalty);
                } else {
                    pen += INTPAR!(widow_penalty);
                }
            }

            if disc_break {
                pen += INTPAR!(broken_penalty);
            }
        }

        lines.push(PendingLine {
            list: line_list,
            width: cur_width,
            indent: cur_indent,
            penalty: pen,
        });

        cur_line += 1;
        cur_p = PASSIVE_NODE_next_break!(cur_p);

        if cur_p != TEX_NULL {
            if !post_disc_break {
                r = TEMP_HEAD;

                loop {
                    q = LLIST_link!(r);

                    if q == PASSIVE_NODE_cur_break!(cur_p) {
                        break;
                    }
                    if is_char_node(q) {
                        break;
                    }
                    if is_non_discardable_node(q) {
                        break;
                    }
                    if (NODE_type!(q) as i32) == KERN_NODE && NODE_subtype!(q) as i32 != EXPLICIT && NODE_subtype!(q) as i32 != SPACE_ADJUSTMENT {
                        break;
                    }

                    r = q;

                    if (NODE_type!(q) as i32) == MATH_NODE && INTPAR!(texxet) > 0 {
                        if odd((*mem.offset(q as isize)).b16.s0 as i32) {
                            if LR_ptr != TEX_NULL
                                && (*mem.offset(LR_ptr as isize)).b32.s0 == (L_CODE * ((*mem.offset(q as isize)).b16.s0 as i32 / L_CODE) + 3)
                            {
                                temp_ptr = LR_ptr;
                                LR_ptr = (*mem.offset(temp_ptr as isize)).b32.s1;
                                (*mem.offset(temp_ptr as isize)).b32.s1 = avail;
                                avail = temp_ptr;
                            }
                        } else {
                            temp_ptr = get_avail();
                            (*mem.offset(temp_ptr as isize)).b32.s0 = L_CODE * ((*mem.offset(q as isize)).b16.s0 as i32 / L_CODE) + 3;
                            (*mem.offset(temp_ptr as isize)).b32.s1 = LR_ptr;
                            LR_ptr = temp_ptr;
                        }
                    }
                }

                if r != TEMP_HEAD {
                    LLIST_link!(r) = TEX_NULL;
                    flush_node_list(LLIST_link!(TEMP_HEAD));
                    LLIST_link!(TEMP_HEAD) = q;
                }
            }
        }

        if cur_p == TEX_NULL {
            break;
        }
    }

    if cur_line != best_line || LLIST_link!(TEMP_HEAD) != TEX_NULL {
        confusion(c"line breaking".as_ptr());
    }

    let line_heads: Vec<i32> = lines.iter().map(|item| item.list).collect();
    let prepared = prepare_native_line_layouts(line_heads.as_ptr(), line_heads.len());

    for line in &lines {
        adjust_tail = ADJUST_HEAD;
        pre_adjust_tail = PRE_ADJUST_HEAD;
        if semantic_pagination_enabled {
            just_box = hpack(line.list, 0, ADDITIONAL as small_number);
        } else {
            just_box = hpack(line.list, line.width, EXACTLY as small_number);
        }
        BOX_shift_amount!(just_box) = line.indent;

        if PRE_ADJUST_HEAD != pre_adjust_tail {
            LLIST_link!(cur_list.tail) = LLIST_link!(PRE_ADJUST_HEAD);
            cur_list.tail = pre_adjust_tail;
        }
        pre_adjust_tail = TEX_NULL;
        append_to_vlist(just_box);
        if ADJUST_HEAD != adjust_tail {
            LLIST_link!(cur_list.tail) = LLIST_link!(ADJUST_HEAD);
            cur_list.tail = adjust_tail;
        }
        adjust_tail = TEX_NULL;
        if line.penalty != 0 {
            r = new_penalty(line.penalty);
            LLIST_link!(cur_list.tail) = r;
            cur_list.tail = r;
        }
    }

    ttxl_release_prepared_layouts(prepared);

    cur_list.prev_graf = best_line - 1;
    cur_list.eTeX_aux = LR_ptr;
}

unsafe fn try_break(mut pi: i32, break_type: small_number) {
    let mut r: i32;
    let mut prev_r: i32;
    let mut old_l: i32;
    let mut no_break_yet: bool;
    let mut prev_prev_r: i32 = TEX_NULL;
    let mut s: i32;
    let mut q: i32;
    let mut v: i32;
    let mut t: i32;
    let mut f: internal_font_number;
    let mut l: i32;
    let mut node_r_stays_active: bool;
    let mut line_width: scaled_t = 0;
    let mut fit_class: usize = DECENT_FIT;
    let mut b: i32 = 0;
    let mut d: i32;
    let mut artificial_demerits: bool;
    let mut shortfall: scaled_t;
    let mut g: scaled_t = 0;

    if semantic_pagination_enabled && cur_p != TEX_NULL {
        return;
    }

    if pi.abs() >= INF_PENALTY {
        if pi > 0 {
            return;
        }
        pi = EJECT_PENALTY;
    }

    no_break_yet = true;
    prev_r = ACTIVE_LIST;
    old_l = 0;
    cur_active_width[1] = active_width[1];
    cur_active_width[2] = active_width[2];
    cur_active_width[3] = active_width[3];
    cur_active_width[4] = active_width[4];
    cur_active_width[5] = active_width[5];
    cur_active_width[6] = active_width[6];

    loop {
        r = LLIST_link!(prev_r);

        if NODE_type!(r) == DELTA_NODE {
            cur_active_width[1] += DELTA_NODE_dwidth!(r);
            cur_active_width[2] += DELTA_NODE_dstretch0!(r);
            cur_active_width[3] += DELTA_NODE_dstretch1!(r);
            cur_active_width[4] += DELTA_NODE_dstretch2!(r);
            cur_active_width[5] += DELTA_NODE_dstretch3!(r);
            cur_active_width[6] += DELTA_NODE_dshrink!(r);
            prev_prev_r = prev_r;
            prev_r = r;
            continue;
        }

        l = ACTIVE_NODE_line_number!(r);

        if l > old_l {
            if minimum_demerits < AWFUL_BAD && (old_l != easy_line || r == LAST_ACTIVE) {
                if no_break_yet {
                    no_break_yet = false;
                    break_width[1] = background[1];
                    break_width[2] = background[2];
                    break_width[3] = background[3];
                    break_width[4] = background[4];
                    break_width[5] = background[5];
                    break_width[6] = background[6];
                    s = cur_p;

                    if break_type as u16 > UNHYPHENATED {
                        if cur_p != TEX_NULL {
                            t = DISCRETIONARY_NODE_replace_count!(cur_p) as i32;
                            v = cur_p;
                            s = DISCRETIONARY_NODE_post_break!(cur_p);

                            while t > 0 {
                                t -= 1;
                                v = LLIST_link!(v);

                                if is_char_node(v) {
                                    f = CHAR_NODE_font!(v) as i32;
                                    let eff_char = effective_char(true, f, CHAR_NODE_character!(v) as u16);
                                    break_width[1] -= font_character_width(f, eff_char);
                                } else {
                                    match NODE_type!(v) as i32 {
                                        LIGATURE_NODE => {
                                            f = LIGATURE_NODE_lig_font!(v) as i32;
                                            xtx_ligature_present = true;
                                            let eff_char = effective_char(true, f, LIGATURE_NODE_lig_char!(v) as u16);
                                            break_width[1] -= font_character_width(f, eff_char);
                                        }

                                        HLIST_NODE | VLIST_NODE | RULE_NODE | KERN_NODE => {
                                            break_width[1] -= BOX_width!(v);
                                        }

                                        WHATSIT_NODE => {
                                            if matches!(
                                                NODE_subtype!(v) as i32,
                                                NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT | GLYPH_NODE | PIC_NODE | PDF_NODE
                                            ) {
                                                break_width[1] -= BOX_width!(v);
                                            } else {
                                                confusion(c"disc1a".as_ptr());
                                            }
                                        }

                                        _ => {
                                            confusion(c"disc1".as_ptr());
                                        }
                                    }
                                }
                            }

                            while s != TEX_NULL {
                                if is_char_node(s) {
                                    f = CHAR_NODE_font!(s) as i32;
                                    let eff_char = effective_char(true, f, CHAR_NODE_character!(s) as u16);
                                    break_width[1] += font_character_width(f, eff_char);
                                } else {
                                    match (*mem.offset(s as isize)).b16.s1 as i32 {
                                        LIGATURE_NODE => {
                                            f = LIGATURE_NODE_lig_font!(s) as i32;
                                            xtx_ligature_present = true;
                                            let eff_char = effective_char(true, f, LIGATURE_NODE_lig_char!(s) as u16);
                                            break_width[1] += font_character_width(f, eff_char);
                                        }

                                        HLIST_NODE | VLIST_NODE | RULE_NODE | KERN_NODE => {
                                            break_width[1] += BOX_width!(s);
                                        }

                                        WHATSIT_NODE => {
                                            if matches!(
                                                NODE_subtype!(s) as i32,
                                                NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT | GLYPH_NODE | PIC_NODE | PDF_NODE
                                            ) {
                                                break_width[1] += BOX_width!(s);
                                            } else {
                                                confusion(c"disc2a".as_ptr());
                                            }
                                        }

                                        _ => {
                                            confusion(c"disc2".as_ptr());
                                        }
                                    }
                                }

                                s = LLIST_link!(s);
                            }

                            break_width[1] += disc_width;
                            if DISCRETIONARY_NODE_post_break!(cur_p) == TEX_NULL {
                                s = LLIST_link!(v);
                            }
                        }
                    }

                    while s != TEX_NULL {
                        if is_char_node(s) {
                            break;
                        }

                        match NODE_type!(s) as i32 {
                            GLUE_NODE => {
                                v = GLUE_NODE_glue_ptr!(s);
                                break_width[1] -= BOX_width!(v);
                                break_width[2 + GLUE_SPEC_stretch_order!(v) as usize] -= GLUE_SPEC_stretch!(v);
                                break_width[6] -= GLUE_SPEC_shrink!(v);
                            }

                            PENALTY_NODE => {}

                            MATH_NODE => {
                                break_width[1] -= BOX_width!(s);
                            }

                            KERN_NODE => {
                                if NODE_subtype!(s) as i32 != EXPLICIT {
                                    break;
                                }
                                break_width[1] -= BOX_width!(s);
                            }

                            _ => {
                                break;
                            }
                        }

                        s = LLIST_link!(s);
                    }
                }

                if NODE_type!(prev_r) == DELTA_NODE {
                    DELTA_NODE_dwidth!(prev_r) += -cur_active_width[1] + break_width[1];
                    DELTA_NODE_dstretch0!(prev_r) += -cur_active_width[2] + break_width[2];
                    DELTA_NODE_dstretch1!(prev_r) += -cur_active_width[3] + break_width[3];
                    DELTA_NODE_dstretch2!(prev_r) += -cur_active_width[4] + break_width[4];
                    DELTA_NODE_dstretch3!(prev_r) += -cur_active_width[5] + break_width[5];
                    DELTA_NODE_dshrink!(prev_r) += -cur_active_width[6] + break_width[6];
                } else if prev_r == ACTIVE_LIST {
                    active_width[1] = break_width[1];
                    active_width[2] = break_width[2];
                    active_width[3] = break_width[3];
                    active_width[4] = break_width[4];
                    active_width[5] = break_width[5];
                    active_width[6] = break_width[6];
                } else {
                    q = get_node(DELTA_NODE_SIZE);
                    LLIST_link!(q) = r;
                    NODE_type!(q) = DELTA_NODE;
                    NODE_subtype!(q) = 0;
                    DELTA_NODE_dwidth!(q) = break_width[1] - cur_active_width[1];
                    DELTA_NODE_dstretch0!(q) = break_width[2] - cur_active_width[2];
                    DELTA_NODE_dstretch1!(q) = break_width[3] - cur_active_width[3];
                    DELTA_NODE_dstretch2!(q) = break_width[4] - cur_active_width[4];
                    DELTA_NODE_dstretch3!(q) = break_width[5] - cur_active_width[5];
                    DELTA_NODE_dshrink!(q) = break_width[6] - cur_active_width[6];
                    LLIST_link!(prev_r) = q;
                    prev_prev_r = prev_r;
                    prev_r = q;
                }

                if INTPAR!(adj_demerits).abs() >= MAX_HALFWORD - minimum_demerits {
                    minimum_demerits = AWFUL_BAD - 1;
                } else {
                    minimum_demerits += INTPAR!(adj_demerits).abs();
                }

                for fc in VERY_LOOSE_FIT..=TIGHT_FIT {
                    if minimal_demerits[fc] <= minimum_demerits {
                        q = get_node(PASSIVE_NODE_SIZE);
                        LLIST_link!(q) = passive;
                        passive = q;
                        PASSIVE_NODE_cur_break!(q) = cur_p;
                        PASSIVE_NODE_prev_break!(q) = best_place[fc];

                        q = get_node(active_node_size as i32);
                        ACTIVE_NODE_break_node!(q) = passive;
                        ACTIVE_NODE_line_number!(q) = best_pl_line[fc] + 1;
                        ACTIVE_NODE_fitness!(q) = fc as u16;
                        NODE_type!(q) = break_type as u16;
                        ACTIVE_NODE_total_demerits!(q) = minimal_demerits[fc];

                        if do_last_line_fit {
                            ACTIVE_NODE_shortfall!(q) = best_pl_short[fc];
                            ACTIVE_NODE_glue!(q) = best_pl_glue[fc];
                        }

                        LLIST_link!(q) = r;
                        LLIST_link!(prev_r) = q;
                        prev_r = q;
                    }

                    minimal_demerits[fc] = MAX_HALFWORD;
                }

                minimum_demerits = MAX_HALFWORD;

                if r != LAST_ACTIVE {
                    q = get_node(DELTA_NODE_SIZE);
                    LLIST_link!(q) = r;
                    NODE_type!(q) = DELTA_NODE;
                    NODE_subtype!(q) = 0;
                    DELTA_NODE_dwidth!(q) = cur_active_width[1] - break_width[1];
                    DELTA_NODE_dstretch0!(q) = cur_active_width[2] - break_width[2];
                    DELTA_NODE_dstretch1!(q) = cur_active_width[3] - break_width[3];
                    DELTA_NODE_dstretch2!(q) = cur_active_width[4] - break_width[4];
                    DELTA_NODE_dstretch3!(q) = cur_active_width[5] - break_width[5];
                    DELTA_NODE_dshrink!(q) = cur_active_width[6] - break_width[6];
                    LLIST_link!(prev_r) = q;
                    prev_prev_r = prev_r;
                    prev_r = q;
                }
            }

            if r == LAST_ACTIVE {
                return;
            }

            if l > easy_line {
                line_width = second_width;
                old_l = MAX_HALFWORD - 1;
            } else {
                old_l = l;

                if l > last_special_line {
                    line_width = second_width;
                } else if LOCAL!(par_shape) == TEX_NULL {
                    line_width = first_width;
                } else {
                    line_width = (*mem.offset((LOCAL!(par_shape) + 2 * l) as isize)).b32.s1;
                }
            }
        }

        if semantic_pagination_enabled {
            line_width = cur_active_width[1];
            artificial_demerits = true;
            shortfall = 0;
        } else {
            artificial_demerits = false;
            shortfall = line_width - cur_active_width[1];

            if INTPAR!(xetex_protrude_chars) > 1 {
                shortfall += total_pw(r, cur_p);
            }
        }

        let mut jumped_to_found = false;

        if shortfall > 0 {
            if cur_active_width[3] != 0 || cur_active_width[4] != 0 || cur_active_width[5] != 0 {
                if do_last_line_fit {
                    'not_found: {
                        if cur_p == TEX_NULL {
                            if ACTIVE_NODE_shortfall!(r) == 0 || ACTIVE_NODE_glue!(r) <= 0 {
                                break 'not_found;
                            }

                            if cur_active_width[3] != fill_width[0] || cur_active_width[4] != fill_width[1] || cur_active_width[5] != fill_width[2] {
                                break 'not_found;
                            }

                            if ACTIVE_NODE_shortfall!(r) > 0 {
                                g = cur_active_width[2];
                            } else {
                                g = cur_active_width[6];
                            }

                            if g <= 0 {
                                break 'not_found;
                            }

                            arith_error = false;
                            g = fract(g, ACTIVE_NODE_shortfall!(r), ACTIVE_NODE_glue!(r), MAX_HALFWORD);
                            if INTPAR!(last_line_fit) < 1000 {
                                g = fract(g, INTPAR!(last_line_fit), 1000, MAX_HALFWORD);
                            }

                            if arith_error {
                                if ACTIVE_NODE_shortfall!(r) > 0 {
                                    g = MAX_HALFWORD;
                                } else {
                                    g = -MAX_HALFWORD;
                                }
                            }

                            if g > 0 {
                                if g > shortfall {
                                    g = shortfall;
                                }

                                if g > 7230584 && cur_active_width[2] < 1663497 {
                                    b = INF_BAD;
                                    fit_class = VERY_LOOSE_FIT;
                                    jumped_to_found = true;
                                    break 'not_found;
                                }

                                b = badness(g, cur_active_width[2]);

                                if b > 12 {
                                    if b > 99 {
                                        fit_class = VERY_LOOSE_FIT;
                                    } else {
                                        fit_class = LOOSE_FIT;
                                    }
                                } else {
                                    fit_class = DECENT_FIT;
                                }

                                jumped_to_found = true;
                                break 'not_found;
                            } else if g < 0 {
                                if -g > cur_active_width[6] {
                                    g = -cur_active_width[6];
                                }

                                b = badness(-g, cur_active_width[6]);
                                if b > 12 {
                                    fit_class = TIGHT_FIT;
                                } else {
                                    fit_class = DECENT_FIT;
                                }

                                jumped_to_found = true;
                                break 'not_found;
                            }
                        }
                    }

                    if !jumped_to_found {
                        shortfall = 0;
                    }
                }

                if !jumped_to_found {
                    b = 0;
                    fit_class = DECENT_FIT;
                }
            } else {
                'done1_stretch: {
                    if shortfall > 7230584 && cur_active_width[2] < 1663497 {
                        b = INF_BAD;
                        fit_class = VERY_LOOSE_FIT;
                        break 'done1_stretch;
                    }

                    b = badness(shortfall, cur_active_width[2]);
                    if b > 12 {
                        if b > 99 {
                            fit_class = VERY_LOOSE_FIT;
                        } else {
                            fit_class = LOOSE_FIT;
                        }
                    } else {
                        fit_class = DECENT_FIT;
                    }
                }
            }
        } else {
            if -shortfall > cur_active_width[6] {
                b = INF_BAD + 1;
            } else {
                b = badness(-shortfall, cur_active_width[6]);
            }

            if b > 12 {
                fit_class = TIGHT_FIT;
            } else {
                fit_class = DECENT_FIT;
            }
        }

        if !jumped_to_found && do_last_line_fit {
            if cur_p == TEX_NULL {
                shortfall = 0;
            }

            if shortfall > 0 {
                g = cur_active_width[2];
            } else if shortfall < 0 {
                g = cur_active_width[6];
            } else {
                g = 0;
            }
        }

        // found:
        let should_deactivate: bool;

        if b > INF_BAD || pi == EJECT_PENALTY {
            if final_pass && minimum_demerits == AWFUL_BAD && LLIST_link!(r) == LAST_ACTIVE && prev_r == ACTIVE_LIST {
                artificial_demerits = true;
                node_r_stays_active = false;
                should_deactivate = false;
            } else if b > threshold {
                should_deactivate = true;
                node_r_stays_active = false;
            } else {
                node_r_stays_active = false;
                should_deactivate = false;
            }
        } else {
            prev_r = r;
            if b > threshold {
                continue;
            }
            node_r_stays_active = true;
            should_deactivate = false;
        }

        if !should_deactivate {
            if artificial_demerits {
                d = 0;
            } else {
                d = INTPAR!(line_penalty) + b;
                if d.abs() >= 10000 {
                    d = 100000000;
                } else {
                    d *= d;
                }

                if pi != 0 {
                    if pi > 0 {
                        d += pi * pi;
                    } else if pi > EJECT_PENALTY {
                        d -= pi * pi;
                    }
                }

                if break_type as u16 == HYPHENATED && NODE_type!(r) == HYPHENATED {
                    if cur_p != TEX_NULL {
                        d += INTPAR!(double_hyphen_demerits);
                    } else {
                        d += INTPAR!(final_hyphen_demerits);
                    }
                }

                if (fit_class as i32 - ACTIVE_NODE_fitness!(r) as i32).abs() > 1 {
                    d += INTPAR!(adj_demerits);
                }
            }

            d += ACTIVE_NODE_total_demerits!(r);

            if d <= minimal_demerits[fit_class] {
                minimal_demerits[fit_class] = d;
                best_place[fit_class] = ACTIVE_NODE_break_node!(r);
                best_pl_line[fit_class] = l;

                if do_last_line_fit {
                    best_pl_short[fit_class] = shortfall;
                    best_pl_glue[fit_class] = g;
                }

                if d < minimum_demerits {
                    minimum_demerits = d;
                }
            }

            if node_r_stays_active {
                continue;
            }
        }

        // deactivate:
        LLIST_link!(prev_r) = LLIST_link!(r);
        free_node(r, active_node_size as i32);

        if prev_r == ACTIVE_LIST {
            r = LLIST_link!(ACTIVE_LIST);

            if NODE_type!(r) == DELTA_NODE {
                active_width[1] += DELTA_NODE_dwidth!(r);
                active_width[2] += DELTA_NODE_dstretch0!(r);
                active_width[3] += DELTA_NODE_dstretch1!(r);
                active_width[4] += DELTA_NODE_dstretch2!(r);
                active_width[5] += DELTA_NODE_dstretch3!(r);
                active_width[6] += DELTA_NODE_dshrink!(r);
                cur_active_width[1] = active_width[1];
                cur_active_width[2] = active_width[2];
                cur_active_width[3] = active_width[3];
                cur_active_width[4] = active_width[4];
                cur_active_width[5] = active_width[5];
                cur_active_width[6] = active_width[6];
                LLIST_link!(ACTIVE_LIST) = LLIST_link!(r);
                free_node(r, DELTA_NODE_SIZE);
            }
        } else if NODE_type!(prev_r) == DELTA_NODE {
            r = LLIST_link!(prev_r);

            if r == LAST_ACTIVE {
                cur_active_width[1] -= DELTA_NODE_dwidth!(prev_r);
                cur_active_width[2] -= DELTA_NODE_dstretch0!(prev_r);
                cur_active_width[3] -= DELTA_NODE_dstretch1!(prev_r);
                cur_active_width[4] -= DELTA_NODE_dstretch2!(prev_r);
                cur_active_width[5] -= DELTA_NODE_dstretch3!(prev_r);
                cur_active_width[6] -= DELTA_NODE_dshrink!(prev_r);
                LLIST_link!(prev_prev_r) = LAST_ACTIVE;
                free_node(prev_r, DELTA_NODE_SIZE);
                prev_r = prev_prev_r;
            } else if NODE_type!(r) == DELTA_NODE {
                cur_active_width[1] += DELTA_NODE_dwidth!(r);
                cur_active_width[2] += DELTA_NODE_dstretch0!(r);
                cur_active_width[3] += DELTA_NODE_dstretch1!(r);
                cur_active_width[4] += DELTA_NODE_dstretch2!(r);
                cur_active_width[5] += DELTA_NODE_dstretch3!(r);
                cur_active_width[6] += DELTA_NODE_dshrink!(r);
                DELTA_NODE_dwidth!(prev_r) += DELTA_NODE_dwidth!(r);
                DELTA_NODE_dstretch0!(prev_r) += DELTA_NODE_dstretch0!(r);
                DELTA_NODE_dstretch1!(prev_r) += DELTA_NODE_dstretch1!(r);
                DELTA_NODE_dstretch2!(prev_r) += DELTA_NODE_dstretch2!(r);
                DELTA_NODE_dstretch3!(prev_r) += DELTA_NODE_dstretch3!(r);
                DELTA_NODE_dshrink!(prev_r) += DELTA_NODE_dshrink!(r);
                LLIST_link!(prev_r) = LLIST_link!(r);
                free_node(r, DELTA_NODE_SIZE);
            }
        }
    }
}

unsafe fn hyphenate() {
    let mut i: small_number;
    let mut j: small_number;
    let mut l: small_number;
    let mut q: i32;
    let mut r: i32;
    let mut s: i32;
    let mut bchar: i32;
    let mut major_tail: i32;
    let mut minor_tail: i32;
    let mut c: UnicodeScalar = 0;
    let mut c_loc: small_number;
    let mut r_count: i32;
    let mut hyf_node: i32;
    let mut z: trie_pointer;
    let mut v: i32;
    let mut h: hyph_pointer;
    let mut k: str_number;
    let mut u: pool_pointer;

    for idx in 0..=hn as usize {
        hyf[idx] = 0;
    }

    h = hc[1] as hyph_pointer;
    hn += 1;
    hc[hn as usize] = cur_lang as i32;

    for idx in 2..=hn as usize {
        h = ((h as i32 + h as i32 + hc[idx]) % HYPH_PRIME) as hyph_pointer;
    }

    'lookup: loop {
        k = *hyph_word.offset(h as isize);
        if k == 0 {
            break 'lookup;
        }

        if length(k) == hn as i32 {
            j = 1;
            u = *str_start.offset((k - 65536) as isize);
            let mut matched = true;
            while (j as usize) <= hn as usize {
                if *str_pool.offset(u as isize) as i32 != hc[j as usize] {
                    matched = false;
                    break;
                }
                j += 1;
                u += 1;
            }

            if matched {
                s = *hyph_list.offset(h as isize);
                while s != TEX_NULL {
                    hyf[(*mem.offset(s as isize)).b32.s0 as usize] = 1;
                    s = LLIST_link!(s);
                }
                hn -= 1;
                // goto found
                return hyphenate_found();
            }
        }

        h = *hyph_link.offset(h as isize);
        if h == 0 {
            break 'lookup;
        }
        h -= 1;
    }

    // not_found:
    hn -= 1;
    if *trie_trc.offset((cur_lang as i32 + 1) as isize) as u32 != cur_lang as u32 {
        return;
    }

    hc[0] = 0;
    hc[(hn + 1) as usize] = 0;
    hc[(hn + 2) as usize] = max_hyph_char;

    for j_idx in 0..=(hn - r_hyf as small_number + 1) as usize {
        z = *trie_trl.offset((cur_lang as i32 + 1) as isize) + hc[j_idx];
        l = j_idx as small_number;

        while hc[l as usize] == *trie_trc.offset(z as isize) as i32 {
            if *trie_tro.offset(z as isize) as i32 != MIN_TRIE_OP {
                v = *trie_tro.offset(z as isize) as i32;
                loop {
                    v += op_start[cur_lang as usize];
                    i = l - hyf_distance[v as usize];
                    if (hyf_num[v as usize] as u8) > hyf[i as usize] {
                        hyf[i as usize] = hyf_num[v as usize] as u8;
                    }
                    v = hyf_next[v as usize] as i32;
                    if v == MIN_TRIE_OP {
                        break;
                    }
                }
            }

            l += 1;
            z = *trie_trl.offset(z as isize) + hc[l as usize];
        }
    }

    hyphenate_found();
}

unsafe fn hyphenate_found() {
    let mut i: small_number;
    let mut j: small_number;
    let mut l: small_number;
    let mut q: i32;
    let mut r: i32;
    let mut s: i32;
    let mut bchar: i32;
    let mut major_tail: i32;
    let mut minor_tail: i32;
    let mut c: UnicodeScalar = 0;
    let mut c_loc: small_number;
    let mut r_count: i32;
    let mut hyf_node: i32;

    for idx in 0..=(l_hyf - 1) as usize {
        hyf[idx] = 0;
    }

    for idx in 0..=(r_hyf - 1) as usize {
        hyf[hn as usize - idx] = 0;
    }

    let mut found_odd = false;
    for idx in l_hyf as usize..=(hn as usize - r_hyf as usize) {
        if odd(hyf[idx] as i32) {
            found_odd = true;
            break;
        }
    }

    if !found_odd {
        return;
    }

    // found1:
    if ha != TEX_NULL
        && !is_char_node(ha)
        && (NODE_type!(ha) as i32) == WHATSIT_NODE
        && ((*mem.offset(ha as isize)).b16.s0 as i32 == NATIVE_WORD_NODE || (*mem.offset(ha as isize)).b16.s0 as i32 == NATIVE_WORD_NODE_AT)
    {
        s = cur_p;
        while (*mem.offset(s as isize)).b32.s1 != ha {
            s = LLIST_link!(s);
        }
        hyphen_passed = 0;

        for j_val in l_hyf..=(hn as i32 - r_hyf) {
            if odd(hyf[j_val as usize] as i32) {
                q = new_native_word_node(hf, j_val - hyphen_passed as i32);
                (*mem.offset(q as isize)).b16.s0 = (*mem.offset(ha as isize)).b16.s0;

                for idx in 0..(j_val - hyphen_passed as i32) {
                    *native_node_text(q).offset(idx as isize) = *native_node_text(ha).offset((idx + hyphen_passed as i32) as isize);
                }

                set_native_metrics(q, INTPAR!(xetex_use_glyph_metrics) > 0);
                (*mem.offset(s as isize)).b32.s1 = q;
                s = q;
                q = new_disc();
                (*mem.offset((q + 1) as isize)).b32.s0 = new_native_character(hf, hyf_char);
                (*mem.offset(s as isize)).b32.s1 = q;
                s = q;
                hyphen_passed = j_val as small_number;
            }
        }

        hn = (*mem.offset((ha + 4) as isize)).b16.s1 as small_number;
        q = new_native_word_node(hf, hn as i32 - hyphen_passed as i32);
        (*mem.offset(q as isize)).b16.s0 = (*mem.offset(ha as isize)).b16.s0;

        for idx in 0..(hn as i32 - hyphen_passed as i32) {
            *native_node_text(q).offset(idx as isize) = *native_node_text(ha).offset((idx + hyphen_passed as i32) as isize);
        }

        set_native_metrics(q, INTPAR!(xetex_use_glyph_metrics) > 0);
        (*mem.offset(s as isize)).b32.s1 = q;
        s = q;
        q = (*mem.offset(ha as isize)).b32.s1;
        (*mem.offset(s as isize)).b32.s1 = q;
        (*mem.offset(ha as isize)).b32.s1 = TEX_NULL;
        flush_node_list(ha);
    } else {
        q = (*mem.offset(hb as isize)).b32.s1;
        (*mem.offset(hb as isize)).b32.s1 = TEX_NULL;
        r = (*mem.offset(ha as isize)).b32.s1;
        (*mem.offset(ha as isize)).b32.s1 = TEX_NULL;
        bchar = hyf_bchar;

        'setup: {
            if is_char_node(ha) {
                if (*mem.offset(ha as isize)).b16.s1 as i32 != hf {
                    // found2:
                    s = ha;
                    j = 0;
                    hu[0] = max_hyph_char;
                    init_lig = false;
                    init_list = TEX_NULL;
                    break 'setup;
                } else {
                    init_list = ha;
                    init_lig = false;
                    hu[0] = (*mem.offset(ha as isize)).b16.s0 as i32;
                }
            } else if (NODE_type!(ha) as i32) == LIGATURE_NODE {
                if (*mem.offset((ha + 1) as isize)).b16.s1 as i32 != hf {
                    // found2:
                    s = ha;
                    j = 0;
                    hu[0] = max_hyph_char;
                    init_lig = false;
                    init_list = TEX_NULL;
                    break 'setup;
                } else {
                    init_list = (*mem.offset((ha + 1) as isize)).b32.s1;
                    init_lig = true;
                    init_lft = (*mem.offset(ha as isize)).b16.s0 > 1;
                    hu[0] = (*mem.offset((ha + 1) as isize)).b16.s0 as i32;

                    if init_list == TEX_NULL {
                        if init_lft {
                            hu[0] = max_hyph_char;
                            init_lig = false;
                        }
                    }
                    free_node(ha, SMALL_NODE_SIZE);
                }
            } else {
                if !is_char_node(r) && (NODE_type!(r) as i32) == LIGATURE_NODE && (*mem.offset(r as isize)).b16.s0 > 1 {
                    // found2:
                    s = ha;
                    j = 0;
                    hu[0] = max_hyph_char;
                    init_lig = false;
                    init_list = TEX_NULL;
                    break 'setup;
                }

                j = 1;
                s = ha;
                init_list = TEX_NULL;
                break 'setup;
            }

            s = cur_p;
            while (*mem.offset(s as isize)).b32.s1 != ha {
                s = LLIST_link!(s);
            }
            j = 0;
        }

        // common_ending:
        flush_node_list(r);

        loop {
            l = j;
            j = reconstitute(j, hn, bchar, hyf_char) + 1;

            if hyphen_passed == 0 {
                (*mem.offset(s as isize)).b32.s1 = (*mem.offset(HOLD_HEAD as isize)).b32.s1;
                while (*mem.offset(s as isize)).b32.s1 > TEX_NULL {
                    s = LLIST_link!(s);
                }

                if odd(hyf[(j - 1) as usize] as i32) {
                    l = j;
                    hyphen_passed = j - 1;
                    (*mem.offset(HOLD_HEAD as isize)).b32.s1 = TEX_NULL;
                }
            }

            if hyphen_passed > 0 {
                loop {
                    r = get_node(SMALL_NODE_SIZE);
                    (*mem.offset(r as isize)).b32.s1 = (*mem.offset(HOLD_HEAD as isize)).b32.s1;
                    NODE_type!(r) = DISC_NODE as u16;
                    major_tail = r;
                    r_count = 0;

                    while (*mem.offset(major_tail as isize)).b32.s1 > TEX_NULL {
                        major_tail = LLIST_link!(major_tail);
                        r_count += 1;
                    }

                    i = hyphen_passed;
                    hyf[i as usize] = 0;
                    minor_tail = TEX_NULL;
                    (*mem.offset((r + 1) as isize)).b32.s0 = TEX_NULL;
                    hyf_node = new_character(hf, hyf_char as UTF16_code);

                    if hyf_node != TEX_NULL {
                        i += 1;
                        c = hu[i as usize];
                        hu[i as usize] = hyf_char;
                        (*mem.offset(hyf_node as isize)).b32.s1 = avail;
                        avail = hyf_node;
                    }

                    while l <= i {
                        l = reconstitute(l, i, *font_bchar.offset(hf as isize) as i32, TOO_BIG_CHAR) + 1;

                        if (*mem.offset(HOLD_HEAD as isize)).b32.s1 > TEX_NULL {
                            if minor_tail == TEX_NULL {
                                (*mem.offset((r + 1) as isize)).b32.s0 = (*mem.offset(HOLD_HEAD as isize)).b32.s1;
                            } else {
                                (*mem.offset(minor_tail as isize)).b32.s1 = (*mem.offset(HOLD_HEAD as isize)).b32.s1;
                            }

                            minor_tail = (*mem.offset(HOLD_HEAD as isize)).b32.s1;
                            while (*mem.offset(minor_tail as isize)).b32.s1 > TEX_NULL {
                                minor_tail = LLIST_link!(minor_tail);
                            }
                        }
                    }

                    if hyf_node != TEX_NULL {
                        hu[i as usize] = c;
                        l = i;
                        i -= 1;
                    }

                    minor_tail = TEX_NULL;
                    (*mem.offset((r + 1) as isize)).b32.s1 = TEX_NULL;
                    c_loc = 0;

                    if *bchar_label.offset(hf as isize) != NON_ADDRESS {
                        l -= 1;
                        c = hu[l as usize];
                        c_loc = l;
                        hu[l as usize] = max_hyph_char;
                    }

                    while l < j {
                        loop {
                            l = reconstitute(l, hn, bchar, TOO_BIG_CHAR) + 1;

                            if c_loc > 0 {
                                hu[c_loc as usize] = c;
                                c_loc = 0;
                            }

                            if (*mem.offset(HOLD_HEAD as isize)).b32.s1 > TEX_NULL {
                                if minor_tail == TEX_NULL {
                                    (*mem.offset((r + 1) as isize)).b32.s1 = (*mem.offset(HOLD_HEAD as isize)).b32.s1;
                                } else {
                                    (*mem.offset(minor_tail as isize)).b32.s1 = (*mem.offset(HOLD_HEAD as isize)).b32.s1;
                                }

                                minor_tail = (*mem.offset(HOLD_HEAD as isize)).b32.s1;
                                while (*mem.offset(minor_tail as isize)).b32.s1 > TEX_NULL {
                                    minor_tail = LLIST_link!(minor_tail);
                                }
                            }

                            if l >= j {
                                break;
                            }
                        }

                        while l > j {
                            j = reconstitute(j, hn, bchar, TOO_BIG_CHAR) + 1;
                            (*mem.offset(major_tail as isize)).b32.s1 = (*mem.offset(HOLD_HEAD as isize)).b32.s1;

                            while (*mem.offset(major_tail as isize)).b32.s1 > TEX_NULL {
                                major_tail = LLIST_link!(major_tail);
                                r_count += 1;
                            }
                        }
                    }

                    if r_count > 127 {
                        (*mem.offset(s as isize)).b32.s1 = (*mem.offset(r as isize)).b32.s1;
                        (*mem.offset(r as isize)).b32.s1 = TEX_NULL;
                        flush_node_list(r);
                    } else {
                        (*mem.offset(s as isize)).b32.s1 = r;
                        (*mem.offset(r as isize)).b16.s0 = r_count as u16;
                    }

                    s = major_tail;
                    hyphen_passed = j - 1;
                    (*mem.offset(HOLD_HEAD as isize)).b32.s1 = TEX_NULL;

                    if !odd(hyf[(j - 1) as usize] as i32) {
                        break;
                    }
                }
            }

            if j > hn {
                break;
            }
        }

        (*mem.offset(s as isize)).b32.s1 = q;
        flush_list(init_list);
    }
}

unsafe fn finite_shrink(p: i32) -> i32 {
    let q: i32;

    if no_shrink_error_yet {
        no_shrink_error_yet = false;
        error_here_with_diagnostic(c"Infinite glue shrinkage found in a paragraph".as_ptr());
        capture_to_diagnostic(std::ptr::null_mut());
        help_ptr = 5;
        help_line[4] = c"The paragraph just ended includes some glue that has".as_ptr();
        help_line[3] = c"infinite shrinkability, e.g., `\\hskip 0pt minus 1fil'.".as_ptr();
        help_line[2] = c"Such glue doesn't belong there---it allows a paragraph".as_ptr();
        help_line[1] = c"of any length to fit on one line. But it's safe to proceed,".as_ptr();
        help_line[0] = c"since the offensive shrinkability has been made finite.".as_ptr();
        error();
    }

    q = new_spec(p);
    GLUE_SPEC_shrink_order!(q) = NORMAL as u16;
    delete_glue_ref(p);
    q
}

unsafe fn reconstitute(mut j: small_number, n: small_number, mut bchar: i32, mut hchar: i32) -> small_number {
    let mut p: i32;
    let mut t: i32;
    let mut q: b16x4;
    let mut cur_rh: i32;
    let mut test_char: i32;
    let mut w: scaled_t;
    let mut k: font_index;

    hyphen_passed = 0;
    t = HOLD_HEAD;
    w = 0;
    (*mem.offset(HOLD_HEAD as isize)).b32.s1 = TEX_NULL;
    cur_l = hu[j as usize];
    cur_q = t;

    if j == 0 {
        ligature_present = init_lig;
        p = init_list;
        if ligature_present {
            lft_hit = init_lft;
        }

        while p > TEX_NULL {
            (*mem.offset(t as isize)).b32.s1 = get_avail();
            t = LLIST_link!(t);
            (*mem.offset(t as isize)).b16.s1 = hf as u16;
            (*mem.offset(t as isize)).b16.s0 = (*mem.offset(p as isize)).b16.s0;
            p = LLIST_link!(p);
        }
    } else if cur_l < TOO_BIG_CHAR {
        (*mem.offset(t as isize)).b32.s1 = get_avail();
        t = LLIST_link!(t);
        (*mem.offset(t as isize)).b16.s1 = hf as u16;
        (*mem.offset(t as isize)).b16.s0 = cur_l as u16;
    }

    lig_stack = TEX_NULL;

    if j < n {
        cur_r = hu[(j + 1) as usize];
    } else {
        cur_r = bchar;
    }

    if odd(hyf[j as usize] as i32) {
        cur_rh = hchar;
    } else {
        cur_rh = TOO_BIG_CHAR;
    }

    'continue_loop: loop {
        'inner: {
            if cur_l == TOO_BIG_CHAR {
                k = *bchar_label.offset(hf as isize);
                if k == NON_ADDRESS {
                    break 'inner;
                } else {
                    q = (*font_info.offset(k as isize)).b16;
                }
            } else {
                q = font_character_info(hf, effective_char(true, hf, cur_l as u16));
                if (q.s1 % 4) != LIG_TAG {
                    break 'inner;
                }
                k = *lig_kern_base.offset(hf as isize) + q.s0 as i32;
                q = (*font_info.offset(k as isize)).b16;

                if q.s3 > 128 {
                    k = *lig_kern_base.offset(hf as isize) + 256 * q.s1 as i32 + q.s0 as i32 + 32768 - 256 * 128;
                    q = (*font_info.offset(k as isize)).b16;
                }
            }

            if cur_rh < TOO_BIG_CHAR {
                test_char = cur_rh;
            } else {
                test_char = cur_r;
            }

            loop {
                if q.s2 as i32 == test_char {
                    if q.s3 <= 128 {
                        if cur_rh < TOO_BIG_CHAR {
                            hyphen_passed = j;
                            hchar = TOO_BIG_CHAR;
                            cur_rh = TOO_BIG_CHAR;
                            continue 'continue_loop;
                        } else {
                            if hchar < TOO_BIG_CHAR {
                                if odd(hyf[j as usize] as i32) {
                                    hyphen_passed = j;
                                    hchar = TOO_BIG_CHAR;
                                }
                            }

                            if q.s1 < 128 {
                                if cur_l == TOO_BIG_CHAR {
                                    lft_hit = true;
                                }
                                if j == n {
                                    if lig_stack == TEX_NULL {
                                        rt_hit = true;
                                    }
                                }

                                match q.s1 {
                                    1 | 5 => {
                                        cur_l = q.s0 as i32;
                                        ligature_present = true;
                                    }

                                    2 | 6 => {
                                        cur_r = q.s0 as i32;
                                        if lig_stack > TEX_NULL {
                                            (*mem.offset(lig_stack as isize)).b16.s0 = cur_r as u16;
                                        } else {
                                            lig_stack = new_lig_item(cur_r as u16);
                                            if j == n {
                                                bchar = TOO_BIG_CHAR;
                                            } else {
                                                p = get_avail();
                                                (*mem.offset((lig_stack + 1) as isize)).b32.s1 = p;
                                                (*mem.offset(p as isize)).b16.s0 = hu[(j + 1) as usize] as u16;
                                                (*mem.offset(p as isize)).b16.s1 = hf as u16;
                                            }
                                        }
                                    }

                                    3 => {
                                        cur_r = q.s0 as i32;
                                        p = lig_stack;
                                        lig_stack = new_lig_item(cur_r as u16);
                                        (*mem.offset(lig_stack as isize)).b32.s1 = p;
                                    }

                                    7 | 11 => {
                                        if ligature_present {
                                            p = new_ligature(hf, cur_l as u16, (*mem.offset(cur_q as isize)).b32.s1);
                                            if lft_hit {
                                                (*mem.offset(p as isize)).b16.s0 = 2;
                                                lft_hit = false;
                                            }
                                            (*mem.offset(cur_q as isize)).b32.s1 = p;
                                            t = p;
                                            ligature_present = false;
                                        }
                                        cur_q = t;
                                        cur_l = q.s0 as i32;
                                        ligature_present = true;
                                    }

                                    _ => {
                                        cur_l = q.s0 as i32;
                                        ligature_present = true;

                                        if lig_stack > TEX_NULL {
                                            if (*mem.offset((lig_stack + 1) as isize)).b32.s1 > TEX_NULL {
                                                (*mem.offset(t as isize)).b32.s1 = (*mem.offset((lig_stack + 1) as isize)).b32.s1;
                                                t = LLIST_link!(t);
                                                j += 1;
                                            }

                                            p = lig_stack;
                                            lig_stack = (*mem.offset(p as isize)).b32.s1;
                                            free_node(p, SMALL_NODE_SIZE);

                                            if lig_stack == TEX_NULL {
                                                if j < n {
                                                    cur_r = hu[(j + 1) as usize];
                                                } else {
                                                    cur_r = bchar;
                                                }

                                                if odd(hyf[j as usize] as i32) {
                                                    cur_rh = hchar;
                                                } else {
                                                    cur_rh = TOO_BIG_CHAR;
                                                }
                                            } else {
                                                cur_r = (*mem.offset(lig_stack as isize)).b16.s0 as i32;
                                            }
                                        } else if j == n {
                                            break 'inner;
                                        } else {
                                            (*mem.offset(t as isize)).b32.s1 = get_avail();
                                            t = LLIST_link!(t);
                                            (*mem.offset(t as isize)).b16.s1 = hf as u16;
                                            (*mem.offset(t as isize)).b16.s0 = cur_r as u16;
                                            j += 1;

                                            if j < n {
                                                cur_r = hu[(j + 1) as usize];
                                            } else {
                                                cur_r = bchar;
                                            }

                                            if odd(hyf[j as usize] as i32) {
                                                cur_rh = hchar;
                                            } else {
                                                cur_rh = TOO_BIG_CHAR;
                                            }
                                        }
                                    }
                                }

                                if q.s1 > 4 {
                                    if q.s1 != 7 {
                                        break 'inner;
                                    }
                                }

                                continue 'continue_loop;
                            }

                            w = (*font_info.offset((*kern_base.offset(hf as isize) + 256 * q.s1 as i32 + q.s0 as i32) as isize))
                                .b32
                                .s1;
                            break 'inner;
                        }
                    }
                }

                if q.s3 >= 128 {
                    if cur_rh == TOO_BIG_CHAR {
                        break 'inner;
                    } else {
                        cur_rh = TOO_BIG_CHAR;
                        continue 'continue_loop;
                    }
                }

                k = k + q.s3 as i32 + 1;
                q = (*font_info.offset(k as isize)).b16;
            }
        }

        // done:
        if ligature_present {
            p = new_ligature(hf, cur_l as u16, (*mem.offset(cur_q as isize)).b32.s1);

            if lft_hit {
                (*mem.offset(p as isize)).b16.s0 = 2;
                lft_hit = false;
            }

            if rt_hit {
                if lig_stack == TEX_NULL {
                    (*mem.offset(p as isize)).b16.s0 += 1;
                    rt_hit = false;
                }
            }

            (*mem.offset(cur_q as isize)).b32.s1 = p;
            t = p;
            ligature_present = false;
        }

        if w != 0 {
            (*mem.offset(t as isize)).b32.s1 = new_kern(w);
            t = LLIST_link!(t);
            w = 0;
            (*mem.offset((t + 2) as isize)).b32.s0 = 0;
        }

        if lig_stack > TEX_NULL {
            cur_q = t;
            cur_l = (*mem.offset(lig_stack as isize)).b16.s0 as i32;
            ligature_present = true;

            if (*mem.offset((lig_stack + 1) as isize)).b32.s1 > TEX_NULL {
                (*mem.offset(t as isize)).b32.s1 = (*mem.offset((lig_stack + 1) as isize)).b32.s1;
                t = LLIST_link!(t);
                j += 1;
            }

            p = lig_stack;
            lig_stack = (*mem.offset(p as isize)).b32.s1;
            free_node(p, SMALL_NODE_SIZE);

            if lig_stack == TEX_NULL {
                if j < n {
                    cur_r = hu[(j + 1) as usize];
                } else {
                    cur_r = bchar;
                }

                if odd(hyf[j as usize] as i32) {
                    cur_rh = hchar;
                } else {
                    cur_rh = TOO_BIG_CHAR;
                }
            } else {
                cur_r = (*mem.offset(lig_stack as isize)).b16.s0 as i32;
            }

            continue 'continue_loop;
        }

        return j;
    }
}

unsafe fn total_pw(q: i32, p: i32) -> scaled_t {
    let mut l: i32;
    let mut r: i32;

    if (*mem.offset((q + 1) as isize)).b32.s1 == TEX_NULL {
        l = first_p;
    } else {
        l = (*mem.offset(((*mem.offset((q + 1) as isize)).b32.s1 + 1) as isize)).b32.s1;
    }

    r = prev_rightmost(global_prev_p, p);

    if p != TEX_NULL && (NODE_type!(p) as i32) == DISC_NODE && (*mem.offset((p + 1) as isize)).b32.s0 != TEX_NULL {
        r = (*mem.offset((p + 1) as isize)).b32.s0;
        while (*mem.offset(r as isize)).b32.s1 != TEX_NULL {
            r = LLIST_link!(r);
        }
    } else {
        r = find_protchar_right(l, r);
    }

    'done: {
        if l != TEX_NULL && (NODE_type!(l) as i32) == DISC_NODE {
            if (*mem.offset((l + 1) as isize)).b32.s1 != TEX_NULL {
                l = (*mem.offset((l + 1) as isize)).b32.s1;
                break 'done;
            } else {
                let mut n = (*mem.offset(l as isize)).b16.s0 as i32;
                l = LLIST_link!(l);

                while n > 0 {
                    if (*mem.offset(l as isize)).b32.s1 != TEX_NULL {
                        l = LLIST_link!(l);
                    }
                    n -= 1;
                }
            }
        }

        l = find_protchar_left(l, true);
    }

    char_pw(l, 0) + char_pw(r, 1)
}

unsafe fn find_protchar_left(mut l: i32, d: bool) -> i32 {
    let mut t: i32;
    let mut run: bool;

    if (*mem.offset(l as isize)).b32.s1 != TEX_NULL
        && (NODE_type!(l) as i32) == HLIST_NODE
        && (*mem.offset((l + 1) as isize)).b32.s1 == 0
        && (*mem.offset((l + 3) as isize)).b32.s1 == 0
        && (*mem.offset((l + 2) as isize)).b32.s1 == 0
        && (*mem.offset((l + 5) as isize)).b32.s1 == TEX_NULL
    {
        l = LLIST_link!(l);
    } else if d {
        while (*mem.offset(l as isize)).b32.s1 != TEX_NULL && !is_char_node(l) && !is_non_discardable_node(l) {
            l = LLIST_link!(l);
        }
    }

    hlist_stack_level = 0;
    run = true;

    loop {
        t = l;

        while run && (NODE_type!(l) as i32) == HLIST_NODE && (*mem.offset((l + 5) as isize)).b32.s1 != TEX_NULL {
            push_node(l);
            l = (*mem.offset((l + 5) as isize)).b32.s1;
        }

        while run
            && !is_char_node(l)
            && ((NODE_type!(l) as i32) == INS_NODE
                || (NODE_type!(l) as i32) == MARK_NODE
                || (NODE_type!(l) as i32) == ADJUST_NODE
                || (NODE_type!(l) as i32) == PENALTY_NODE
                || ((NODE_type!(l) as i32) == DISC_NODE
                    && (*mem.offset((l + 1) as isize)).b32.s0 == TEX_NULL
                    && (*mem.offset((l + 1) as isize)).b32.s1 == TEX_NULL
                    && (*mem.offset(l as isize)).b16.s0 == 0)
                || ((NODE_type!(l) as i32) == MATH_NODE && (*mem.offset((l + 1) as isize)).b32.s1 == 0)
                || ((NODE_type!(l) as i32) == KERN_NODE
                    && ((*mem.offset((l + 1) as isize)).b32.s1 == 0 || (*mem.offset(l as isize)).b16.s0 as i32 == NORMAL))
                || ((NODE_type!(l) as i32) == GLUE_NODE && (*mem.offset((l + 1) as isize)).b32.s0 == 0)
                || ((NODE_type!(l) as i32) == HLIST_NODE
                    && (*mem.offset((l + 1) as isize)).b32.s1 == 0
                    && (*mem.offset((l + 3) as isize)).b32.s1 == 0
                    && (*mem.offset((l + 2) as isize)).b32.s1 == 0
                    && (*mem.offset((l + 5) as isize)).b32.s1 == TEX_NULL))
        {
            while (*mem.offset(l as isize)).b32.s1 == TEX_NULL && hlist_stack_level > 0 {
                l = pop_node();
            }

            if (*mem.offset(l as isize)).b32.s1 != TEX_NULL {
                l = LLIST_link!(l);
            } else if hlist_stack_level == 0 {
                run = false;
            }
        }

        if t == l {
            break;
        }
    }

    l
}

unsafe fn find_protchar_right(mut l: i32, mut r: i32) -> i32 {
    let mut t: i32;
    let mut run: bool;

    if r == TEX_NULL {
        return TEX_NULL;
    }

    hlist_stack_level = 0;
    run = true;

    loop {
        t = r;

        while run && (NODE_type!(r) as i32) == HLIST_NODE && (*mem.offset((r + 5) as isize)).b32.s1 != TEX_NULL {
            push_node(l);
            push_node(r);
            l = (*mem.offset((r + 5) as isize)).b32.s1;
            r = l;
            while (*mem.offset(r as isize)).b32.s1 != TEX_NULL {
                r = LLIST_link!(r);
            }
        }

        while run
            && !is_char_node(r)
            && ((NODE_type!(r) as i32) == INS_NODE
                || (NODE_type!(r) as i32) == MARK_NODE
                || (NODE_type!(r) as i32) == ADJUST_NODE
                || (NODE_type!(r) as i32) == PENALTY_NODE
                || ((NODE_type!(r) as i32) == DISC_NODE
                    && (*mem.offset((r + 1) as isize)).b32.s0 == TEX_NULL
                    && (*mem.offset((r + 1) as isize)).b32.s1 == TEX_NULL
                    && (*mem.offset(r as isize)).b16.s0 == 0)
                || ((NODE_type!(r) as i32) == MATH_NODE && (*mem.offset((r + 1) as isize)).b32.s1 == 0)
                || ((NODE_type!(r) as i32) == KERN_NODE
                    && ((*mem.offset((r + 1) as isize)).b32.s1 == 0 || (*mem.offset(r as isize)).b16.s0 as i32 == NORMAL))
                || ((NODE_type!(r) as i32) == GLUE_NODE && (*mem.offset((r + 1) as isize)).b32.s0 == 0)
                || ((NODE_type!(r) as i32) == HLIST_NODE
                    && (*mem.offset((r + 1) as isize)).b32.s1 == 0
                    && (*mem.offset((r + 3) as isize)).b32.s1 == 0
                    && (*mem.offset((r + 2) as isize)).b32.s1 == 0
                    && (*mem.offset((r + 5) as isize)).b32.s1 == TEX_NULL))
        {
            while r == l && hlist_stack_level > 0 {
                r = pop_node();
                l = pop_node();
            }

            if r != l && r != TEX_NULL {
                r = prev_rightmost(l, r);
            } else if r == l && hlist_stack_level == 0 {
                run = false;
            }
        }

        if t == r {
            break;
        }
    }

    r
}

unsafe fn push_node(p: i32) {
    if (hlist_stack_level as usize) > MAX_HLIST_STACK {
        pdf_error(c"push_node".as_ptr(), c"stack overflow".as_ptr());
    }
    hlist_stack[hlist_stack_level as usize] = p;
    hlist_stack_level += 1;
}

unsafe fn pop_node() -> i32 {
    hlist_stack_level -= 1;
    if hlist_stack_level < 0 {
        pdf_error(c"pop_node".as_ptr(), c"stack underflow (internal error)".as_ptr());
    }
    hlist_stack[hlist_stack_level as usize]
}
