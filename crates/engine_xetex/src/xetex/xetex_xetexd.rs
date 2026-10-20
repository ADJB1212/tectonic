#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use crate::xetex::xetex_constants::*;
use crate::xetex::xetex_core::*;
use crate::xetex::xetex_format::*;

pub type eight_bits = u8;

#[inline]
pub fn odd(x: i32) -> bool {
    (x & 1) != 0
}

pub const MIN_TRIE_OP: i32 = 0;
pub const TRIE_OP_SIZE: i32 = 35111;

pub type UTF16_code = u16;
pub type UTF8_code = u8;
pub type UnicodeScalar = i32;
pub type pool_pointer = i32;
pub type str_number = i32;
pub type packed_UTF16_code = u16;
pub type small_number = i16;
pub type two_choices = u8;
pub type four_choices = u8;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct b32x2 {
    #[cfg(target_endian = "big")]
    pub s1: i32,
    #[cfg(target_endian = "big")]
    pub s0: i32,

    #[cfg(not(target_endian = "big"))]
    pub s0: i32,
    #[cfg(not(target_endian = "big"))]
    pub s1: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct b16x4 {
    #[cfg(target_endian = "big")]
    pub s3: u16,
    #[cfg(target_endian = "big")]
    pub s2: u16,
    #[cfg(target_endian = "big")]
    pub s1: u16,
    #[cfg(target_endian = "big")]
    pub s0: u16,

    #[cfg(not(target_endian = "big"))]
    pub s0: u16,
    #[cfg(not(target_endian = "big"))]
    pub s1: u16,
    #[cfg(not(target_endian = "big"))]
    pub s2: u16,
    #[cfg(not(target_endian = "big"))]
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

impl Default for memory_word {
    fn default() -> Self {
        memory_word { b32: b32x2 { s0: 0, s1: 0 } }
    }
}

impl std::fmt::Debug for memory_word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { write!(f, "memory_word {{ b32: {:?} }}", self.b32) }
    }
}

#[macro_export]
macro_rules! LLIST_link {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! LLIST_info {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! NODE_type {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b16.s1
    };
}

#[macro_export]
macro_rules! NODE_subtype {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b16.s0
    };
}

#[macro_export]
macro_rules! BOX_lr_mode {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b16.s0
    };
}

#[macro_export]
macro_rules! BOX_width {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! BOX_depth {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 2) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! BOX_height {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 3) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! BOX_shift_amount {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 4) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! BOX_list_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 5) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! BOX_glue_sign {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 5) as isize)).b16.s1
    };
}

#[macro_export]
macro_rules! BOX_glue_order {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 5) as isize)).b16.s0
    };
}

#[macro_export]
macro_rules! BOX_glue_set {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 6) as isize)).gr
    };
}

#[macro_export]
macro_rules! ACTIVE_NODE_fitness {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b16.s0
    };
}

#[macro_export]
macro_rules! ACTIVE_NODE_break_node {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! ACTIVE_NODE_line_number {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! ACTIVE_NODE_total_demerits {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 2) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! ACTIVE_NODE_shortfall {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 3) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! ACTIVE_NODE_glue {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 4) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! CHAR_NODE_font {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b16.s1
    };
}

#[macro_export]
macro_rules! CHAR_NODE_character {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b16.s0
    };
}

#[macro_export]
macro_rules! DELTA_NODE_dwidth {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! DELTA_NODE_dstretch0 {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 2) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! DELTA_NODE_dstretch1 {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 3) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! DELTA_NODE_dstretch2 {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 4) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! DELTA_NODE_dstretch3 {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 5) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! DELTA_NODE_dshrink {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 6) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! DISCRETIONARY_NODE_replace_count {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b16.s0
    };
}

#[macro_export]
macro_rules! DISCRETIONARY_NODE_pre_break {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! DISCRETIONARY_NODE_post_break {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! EDGE_NODE_edge_dist {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 2) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! GLUE_NODE_glue_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! GLUE_NODE_leader_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! INSERTION_NODE_float_cost {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! INSERTION_NODE_split_top_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 4) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! INSERTION_NODE_ins_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 4) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! LANGUAGE_NODE_what_lang {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! LANGUAGE_NODE_what_lhm {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b16.s1
    };
}

#[macro_export]
macro_rules! LANGUAGE_NODE_what_rhm {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b16.s0
    };
}

#[macro_export]
macro_rules! LIGATURE_NODE_lig_font {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b16.s1
    };
}

#[macro_export]
macro_rules! LIGATURE_NODE_lig_char {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b16.s0
    };
}

#[macro_export]
macro_rules! LIGATURE_NODE_lig_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! MARK_NODE_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! MARK_NODE_class {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! MATH_NODE_lr_dir {
    ($p:expr) => {
        ($crate::NODE_subtype!($p) as i32 / $crate::xetex::xetex_constants::R_CODE)
    };
}

#[macro_export]
macro_rules! MATH_NODE_end_lr_type {
    ($p:expr) => {
        ($crate::xetex::xetex_constants::L_CODE * ($crate::NODE_subtype!($p) as i32 / $crate::xetex::xetex_constants::L_CODE)
            + $crate::xetex::xetex_constants::END_M_CODE)
    };
}

#[macro_export]
macro_rules! NATIVE_NODE_size {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 4) as isize)).b16.s3
    };
}

#[macro_export]
macro_rules! NATIVE_NODE_font {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 4) as isize)).b16.s2
    };
}

#[macro_export]
macro_rules! NATIVE_NODE_length {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 4) as isize)).b16.s1
    };
}

#[macro_export]
macro_rules! NATIVE_NODE_glyph {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 4) as isize)).b16.s1
    };
}

#[macro_export]
macro_rules! NATIVE_NODE_glyph_count {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 4) as isize)).b16.s0
    };
}

#[macro_export]
macro_rules! NATIVE_NODE_glyph_info_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 5) as isize)).ptr
    };
}

#[macro_export]
macro_rules! NATIVE_NODE_text {
    ($p:expr) => {
        $crate::xetex::xetex_xetexd::mem.offset((($p) + $crate::xetex::xetex_constants::NATIVE_NODE_SIZE) as isize) as *mut u16
    };
}

#[macro_export]
macro_rules! PAGE_INS_NODE_broken_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! PAGE_INS_NODE_broken_ins {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! PAGE_INS_NODE_last_ins_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 2) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! PAGE_INS_NODE_best_ins_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 2) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! PASSIVE_NODE_prev_break {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! PASSIVE_NODE_next_break {
    ($p:expr) => {
        $crate::PASSIVE_NODE_prev_break!($p)
    };
}

#[macro_export]
macro_rules! PASSIVE_NODE_cur_break {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! PASSIVE_NODE_serial {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! PENALTY_NODE_penalty {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! PIC_NODE_path_len {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 4) as isize)).b16.s1
    };
}

#[macro_export]
macro_rules! PIC_NODE_path {
    ($p:expr) => {
        $crate::xetex::xetex_xetexd::mem.offset((($p) + $crate::xetex::xetex_constants::PIC_NODE_SIZE) as isize) as *mut u8
    };
}

#[macro_export]
macro_rules! PIC_NODE_total_size {
    ($p:expr) => {
        ($crate::xetex::xetex_constants::PIC_NODE_SIZE
            + ($crate::PIC_NODE_path_len!($p) as i32 + std::mem::size_of::<$crate::xetex::xetex_xetexd::memory_word>() as i32 - 1)
                / std::mem::size_of::<$crate::xetex::xetex_xetexd::memory_word>() as i32)
    };
}

#[macro_export]
macro_rules! WRITE_NODE_tokens {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! SYNCTEX_tag {
    ($p:expr, $nodesize:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + ($nodesize) - $crate::xetex::xetex_constants::SYNCTEX_FIELD_SIZE) as isize))
            .b32
            .s0
    };
}

#[macro_export]
macro_rules! SYNCTEX_line {
    ($p:expr, $nodesize:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + ($nodesize) - $crate::xetex::xetex_constants::SYNCTEX_FIELD_SIZE) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! GLUE_SPEC_ref_count {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! GLUE_SPEC_stretch_order {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b16.s1
    };
}

#[macro_export]
macro_rules! GLUE_SPEC_shrink_order {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b16.s0
    };
}

#[macro_export]
macro_rules! GLUE_SPEC_stretch {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 2) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! GLUE_SPEC_shrink {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 3) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! FONT_CHARACTER_INFO {
    ($f:expr, $c:expr) => {
        (*$crate::xetex::xetex_xetexd::font_info.offset((*$crate::xetex::xetex_xetexd::char_base.offset(($f) as isize) + ($c as i32)) as isize)).b16
    };
}

#[macro_export]
macro_rules! FONT_CHARINFO_WIDTH {
    ($f:expr, $info:expr) => {
        (*$crate::xetex::xetex_xetexd::font_info
            .offset((*$crate::xetex::xetex_xetexd::width_base.offset(($f) as isize) + (($info).s3 as i32)) as isize))
        .b32
        .s1
    };
}

#[macro_export]
macro_rules! FONT_CHARINFO_HEIGHT {
    ($f:expr, $info:expr) => {
        (*$crate::xetex::xetex_xetexd::font_info
            .offset((*$crate::xetex::xetex_xetexd::height_base.offset(($f) as isize) + (($info).s2 as i32 / 16)) as isize))
        .b32
        .s1
    };
}

#[macro_export]
macro_rules! FONT_CHARINFO_DEPTH {
    ($f:expr, $info:expr) => {
        (*$crate::xetex::xetex_xetexd::font_info
            .offset((*$crate::xetex::xetex_xetexd::depth_base.offset(($f) as isize) + (($info).s2 as i32 % 16)) as isize))
        .b32
        .s1
    };
}

#[macro_export]
macro_rules! FONT_CHARINFO_ITALCORR {
    ($f:expr, $info:expr) => {
        (*$crate::xetex::xetex_xetexd::font_info
            .offset((*$crate::xetex::xetex_xetexd::italic_base.offset(($f) as isize) + (($info).s1 as i32 / 4)) as isize))
        .b32
        .s1
    };
}

#[macro_export]
macro_rules! FONT_CHARACTER_WIDTH {
    ($f:expr, $c:expr) => {
        $crate::FONT_CHARINFO_WIDTH!($f, $crate::FONT_CHARACTER_INFO!($f, $c))
    };
}

#[macro_export]
macro_rules! TOKEN_LIST_ref_count {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset(($p) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! ETEX_SA_ref {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! ETEX_SA_ptr {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! ETEX_SA_num {
    ($p:expr) => {
        $crate::ETEX_SA_ptr!($p)
    };
}

#[macro_export]
macro_rules! ETEX_MARK_sa_top_mark {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! ETEX_MARK_sa_first_mark {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 1) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! ETEX_MARK_sa_bot_mark {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 2) as isize)).b32.s0
    };
}

#[macro_export]
macro_rules! ETEX_MARK_sa_split_first_mark {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 2) as isize)).b32.s1
    };
}

#[macro_export]
macro_rules! ETEX_MARK_sa_split_bot_mark {
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::mem.offset((($p) + 3) as isize)).b32.s0
    };
}

pub type glue_ord = u8;
pub type group_code = u8;
pub type internal_font_number = i32;
pub type font_index = i32;
pub type nine_bits = i32;
pub type trie_pointer = i32;
pub type trie_opcode = u16;
pub type hyph_pointer = u16;
pub type save_pointer = i32;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct list_state_record {
    pub mode: i16,
    pub head: i32,
    pub tail: i32,
    pub eTeX_aux: i32,
    pub prev_graf: i32,
    pub mode_line: i32,
    pub aux: memory_word,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct input_state_t {
    pub state: u16,
    pub index: u16,
    pub start: i32,
    pub loc: i32,
    pub limit: i32,
    pub name: i32,
    pub synctex_tag: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum tt_history_t {
    HISTORY_SPOTLESS = 0,
    HISTORY_WARNING_ISSUED = 1,
    HISTORY_ERROR_ISSUED = 2,
    HISTORY_FATAL_ERROR = 3,
}

pub use tt_history_t::*;

pub type rust_input_handle_t = usize;
pub type rust_output_handle_t = usize;

pub use super::xetex_io::{UFILE, ttbc_buffered_input};

pub type ttbc_diagnostic_t = tectonic_bridge_core::Diagnostic;

unsafe extern "C" {
    pub static mut shell_escape_enabled: bool;
    pub static mut eqtb: *mut memory_word;
    pub static mut bad: i32;
    pub static mut name_of_file: *mut libc::c_char;
    pub static mut name_of_file16: *mut UTF16_code;
    pub static mut name_length: i32;
    pub static mut name_length16: i32;
    pub static mut buffer: *mut UnicodeScalar;
    pub static mut first: i32;
    pub static mut last: i32;
    pub static mut max_buf_stack: i32;
    pub static mut in_initex_mode: bool;
    pub static mut error_line: i32;
    pub static mut half_error_line: i32;
    pub static mut max_print_line: i32;
    pub static mut max_strings: i32;
    pub static mut strings_free: i32;
    pub static mut string_vacancies: i32;
    pub static mut pool_size: i32;
    pub static mut pool_free: i32;
    pub static mut font_mem_size: i32;
    pub static mut font_max: i32;
    pub static mut hyph_size: i32;
    pub static mut trie_size: i32;
    pub static mut buf_size: i32;
    pub static mut stack_size: i32;
    pub static mut max_in_open: i32;
    pub static mut param_size: i32;
    pub static mut nest_size: i32;
    pub static mut save_size: i32;
    pub static mut expand_depth: i32;
    pub static mut file_line_error_style_p: libc::c_int;
    pub static mut halt_on_error_p: libc::c_int;
    pub static mut quoted_filename: bool;
    pub static mut insert_src_special_auto: bool;
    pub static mut insert_src_special_every_par: bool;
    pub static mut insert_src_special_every_math: bool;
    pub static mut insert_src_special_every_vbox: bool;
    pub static mut str_pool: *mut packed_UTF16_code;
    pub static mut str_start: *mut pool_pointer;
    pub static mut pool_ptr: pool_pointer;
    pub static mut str_ptr: str_number;
    pub static mut init_pool_ptr: pool_pointer;
    pub static mut init_str_ptr: str_number;
    pub static mut rust_stdout: rust_output_handle_t;
    pub static mut log_file: rust_output_handle_t;
    pub static mut selector: selector_t;
    pub static mut dig: [u8; 23];
    pub static mut tally: i32;
    pub static mut term_offset: i32;
    pub static mut file_offset: i32;
    pub static mut trick_buf: [UTF16_code; 256];
    pub static mut trick_count: i32;
    pub static mut first_count: i32;
    pub static mut doing_special: bool;
    pub static mut native_text: *mut UTF16_code;
    pub static mut native_text_size: i32;
    pub static mut native_len: i32;
    pub static mut save_native_len: i32;
    pub static mut interaction: u8;
    pub static mut deletions_allowed: bool;
    pub static mut set_box_allowed: bool;
    pub static mut history: tt_history_t;
    pub static mut error_count: i8;
    pub static mut help_line: [*const libc::c_char; 6];
    pub static mut help_ptr: u8;
    pub static mut use_err_help: bool;
    pub static mut arith_error: bool;
    pub static mut tex_remainder: scaled_t;
    pub static mut randoms: [i32; 55];
    pub static mut j_random: u8;
    pub static mut random_seed: scaled_t;
    pub static mut two_to_the: [i32; 31];
    pub static mut spec_log: [i32; 29];
    pub static mut temp_ptr: i32;
    pub static mut mem: *mut memory_word;
    pub static mut lo_mem_max: i32;
    pub static mut hi_mem_min: i32;
    pub static mut var_used: i32;
    pub static mut dyn_used: i32;
    pub static mut avail: i32;
    pub static mut mem_end: i32;
    pub static mut rover: i32;
    pub static mut last_leftmost_char: i32;
    pub static mut last_rightmost_char: i32;
    pub static mut hlist_stack: [i32; (MAX_HLIST_STACK + 1) as usize];
    pub static mut hlist_stack_level: i16;
    pub static mut first_p: i32;
    pub static mut global_prev_p: i32;
    pub static mut font_in_short_display: i32;
    pub static mut depth_threshold: i32;
    pub static mut breadth_max: i32;
    pub static mut nest: *mut list_state_record;
    pub static mut nest_ptr: i32;
    pub static mut max_nest_stack: i32;
    pub static mut cur_list: list_state_record;
    pub static mut shown_mode: i16;
    pub static mut old_setting: u8;
    pub static mut hash: *mut b32x2;
    pub static mut hash_used: i32;
    pub static mut hash_extra: i32;
    pub static mut hash_top: i32;
    pub static mut eqtb_top: i32;
    pub static mut hash_high: i32;
    pub static mut no_new_control_sequence: bool;
    pub static mut cs_count: i32;
    pub static mut prim: [b32x2; (PRIM_SIZE + 1) as usize];
    pub static mut prim_used: i32;
    pub static mut save_stack: *mut memory_word;
    pub static mut save_ptr: i32;
    pub static mut max_save_stack: i32;
    pub static mut cur_level: u16;
    pub static mut cur_group: group_code;
    pub static mut cur_boundary: i32;
    pub static mut mag_set: i32;
    pub static mut cur_cmd: eight_bits;
    pub static mut cur_chr: i32;
    pub static mut cur_cs: i32;
    pub static mut cur_tok: i32;
    pub static mut input_stack: *mut input_state_t;
    pub static mut input_ptr: i32;
    pub static mut max_in_stack: i32;
    pub static mut cur_input: input_state_t;
    pub static mut in_open: i32;
    pub static mut open_parens: i32;
    pub static mut input_file: *mut *mut UFILE;
    pub static mut line: i32;
    pub static mut line_stack: *mut i32;
    pub static mut source_filename_stack: *mut str_number;
    pub static mut full_source_filename_stack: *mut str_number;
    pub static mut scanner_status: u8;
    pub static mut warning_index: i32;
    pub static mut def_ref: i32;
    pub static mut param_stack: *mut i32;
    pub static mut param_ptr: i32;
    pub static mut max_param_stack: i32;
    pub static mut align_state: i32;
    pub static mut base_ptr: i32;
    pub static mut par_loc: i32;
    pub static mut par_token: i32;
    pub static mut force_eof: bool;
    pub static mut expand_depth_count: i32;
    pub static mut is_in_csname: bool;
    pub static mut cur_mark: [i32; 5];
    pub static mut long_state: u8;
    pub static mut pstack: [i32; 9];
    pub static mut cur_val: i32;
    pub static mut cur_val1: i32;
    pub static mut cur_val_level: u8;
    pub static mut radix: small_number;
    pub static mut cur_order: glue_ord;
    pub static mut read_file: [*mut UFILE; 16];
    pub static mut read_open: [u8; 17];
    pub static mut cond_ptr: i32;
    pub static mut if_limit: u8;
    pub static mut cur_if: small_number;
    pub static mut if_line: i32;
    pub static mut skip_line: i32;
    pub static mut cur_name: str_number;
    pub static mut cur_area: str_number;
    pub static mut cur_ext: str_number;
    pub static mut area_delimiter: pool_pointer;
    pub static mut ext_delimiter: pool_pointer;
    pub static mut file_name_quote_char: UTF16_code;
    pub static mut format_default_length: i32;
    pub static mut TEX_format_default: *mut libc::c_char;
    pub static mut name_in_progress: bool;
    pub static mut job_name: str_number;
    pub static mut log_opened: bool;
    pub static mut output_file_extension: *const libc::c_char;
    pub static mut texmf_log_name: str_number;
    pub static mut font_info: *mut memory_word;
    pub static mut fmem_ptr: font_index;
    pub static mut font_ptr: internal_font_number;
    pub static mut font_check: *mut b16x4;
    pub static mut font_size: *mut scaled_t;
    pub static mut font_dsize: *mut scaled_t;
    pub static mut font_params: *mut font_index;
    pub static mut font_name: *mut str_number;
    pub static mut font_area: *mut str_number;
    pub static mut font_bc: *mut UTF16_code;
    pub static mut font_ec: *mut UTF16_code;
    pub static mut font_glue: *mut i32;
    pub static mut font_used: *mut bool;
    pub static mut hyphen_char: *mut i32;
    pub static mut skew_char: *mut i32;
    pub static mut bchar_label: *mut font_index;
    pub static mut font_bchar: *mut nine_bits;
    pub static mut font_false_bchar: *mut nine_bits;
    pub static mut font_layout_engine: *mut *mut std::ffi::c_void;
    pub static mut font_mapping: *mut *mut std::ffi::c_void;
    pub static mut font_flags: *mut libc::c_char;
    pub static mut font_letter_space: *mut scaled_t;
    pub static mut loaded_font_mapping: *mut std::ffi::c_void;
    pub static mut loaded_font_flags: libc::c_char;
    pub static mut loaded_font_letter_space: scaled_t;
    pub static mut mapped_text: *mut UTF16_code;
    pub static mut xdv_buffer: *mut libc::c_char;
    pub static mut char_base: *mut i32;
    pub static mut width_base: *mut i32;
    pub static mut height_base: *mut i32;
    pub static mut depth_base: *mut i32;
    pub static mut italic_base: *mut i32;
    pub static mut lig_kern_base: *mut i32;
    pub static mut kern_base: *mut i32;
    pub static mut exten_base: *mut i32;
    pub static mut param_base: *mut i32;
    pub static mut null_character: b16x4;
    pub static mut total_pages: i32;
    pub static mut max_v: scaled_t;
    pub static mut max_h: scaled_t;
    pub static mut max_push: i32;
    pub static mut last_bop: i32;
    pub static mut dead_cycles: i32;
    pub static mut doing_leaders: bool;
    pub static mut rule_ht: scaled_t;
    pub static mut rule_dp: scaled_t;
    pub static mut rule_wd: scaled_t;
    pub static mut cur_h: scaled_t;
    pub static mut cur_v: scaled_t;
    pub static mut epochseconds: i32;
    pub static mut microseconds: i32;
    pub static mut total_stretch: [scaled_t; 4];
    pub static mut total_shrink: [scaled_t; 4];
    pub static mut last_badness: i32;
    pub static mut adjust_tail: i32;
    pub static mut pre_adjust_tail: i32;
    pub static mut pack_begin_line: i32;
    pub static mut empty: b32x2;
    pub static mut cur_f: internal_font_number;
    pub static mut cur_c: i32;
    pub static mut cur_i: b16x4;
    pub static mut cur_align: i32;
    pub static mut cur_span: i32;
    pub static mut cur_loop: i32;
    pub static mut align_ptr: i32;
    pub static mut cur_head: i32;
    pub static mut cur_tail: i32;
    pub static mut cur_pre_head: i32;
    pub static mut cur_pre_tail: i32;
    pub static mut just_box: i32;
    pub static mut active_width: [scaled_t; 7];
    pub static mut hc: [i32; 4099];
    pub static mut hf: internal_font_number;
    pub static mut hu: [i32; 4097];
    pub static mut cur_lang: u8;
    pub static mut max_hyph_char: i32;
    pub static mut hyf: [u8; 4097];
    pub static mut init_list: i32;
    pub static mut init_lig: bool;
    pub static mut init_lft: bool;
    pub static mut hyphen_passed: small_number;
    pub static mut cur_l: i32;
    pub static mut cur_r: i32;
    pub static mut cur_q: i32;
    pub static mut lig_stack: i32;
    pub static mut ligature_present: bool;
    pub static mut lft_hit: bool;
    pub static mut rt_hit: bool;
    pub static mut trie_trl: *mut trie_pointer;
    pub static mut trie_tro: *mut trie_pointer;
    pub static mut trie_trc: *mut u16;
    pub static mut hyf_distance: [small_number; (TRIE_OP_SIZE + 1) as usize];
    pub static mut hyf_num: [small_number; (TRIE_OP_SIZE + 1) as usize];
    pub static mut hyf_next: [trie_opcode; (TRIE_OP_SIZE + 1) as usize];
    pub static mut op_start: [i32; 256];
    pub static mut hyph_word: *mut str_number;
    pub static mut hyph_list: *mut i32;
    pub static mut hyph_link: *mut hyph_pointer;
    pub static mut hyph_count: i32;
    pub static mut hyph_next: i32;
    pub static mut trie_used: [trie_opcode; 256];
    pub static mut trie_op_lang: [u8; (TRIE_OP_SIZE + 1) as usize];
    pub static mut trie_op_val: [trie_opcode; (TRIE_OP_SIZE + 1) as usize];
    pub static mut trie_op_ptr: i32;
    pub static mut max_op_used: trie_opcode;
    pub static mut trie_c: *mut packed_UTF16_code;
    pub static mut trie_o: *mut trie_opcode;
    pub static mut trie_l: *mut trie_pointer;
    pub static mut trie_r: *mut trie_pointer;
    pub static mut trie_ptr: trie_pointer;
    pub static mut trie_hash: *mut trie_pointer;
    pub static mut trie_taken: *mut bool;
    pub static mut trie_min: [trie_pointer; 65536];
    pub static mut trie_max: trie_pointer;
    pub static mut trie_not_ready: bool;
    pub static mut best_height_plus_depth: scaled_t;
    pub static mut page_tail: i32;
    pub static mut page_contents: u8;
    pub static mut page_so_far: [scaled_t; 8];
    pub static mut last_glue: i32;
    pub static mut last_penalty: i32;
    pub static mut last_kern: scaled_t;
    pub static mut last_node_type: i32;
    pub static mut insert_penalties: i32;
    pub static mut output_active: bool;
    pub static mut main_f: internal_font_number;
    pub static mut main_i: b16x4;
    pub static mut main_j: b16x4;
    pub static mut main_k: font_index;
    pub static mut main_p: i32;
    pub static mut main_pp: i32;
    pub static mut main_ppp: i32;
    pub static mut main_h: i32;
    pub static mut is_hyph: bool;
    pub static mut space_class: i32;
    pub static mut prev_class: i32;
    pub static mut main_s: i32;
    pub static mut bchar: i32;
    pub static mut false_bchar: i32;
    pub static mut cancel_boundary: bool;
    pub static mut ins_disc: bool;
    pub static mut cur_box: i32;
    pub static mut after_token: i32;
    pub static mut long_help_seen: bool;
    pub static mut format_ident: str_number;
    pub static mut write_file: [rust_output_handle_t; 16];
    pub static mut write_open: [bool; 18];
    pub static mut write_loc: i32;
    pub static mut cur_page_width: scaled_t;
    pub static mut cur_page_height: scaled_t;
    pub static mut cur_h_offset: scaled_t;
    pub static mut cur_v_offset: scaled_t;
    pub static mut pdf_last_x_pos: i32;
    pub static mut pdf_last_y_pos: i32;
    pub static mut eof_seen: *mut bool;
    pub static mut LR_ptr: i32;
    pub static mut LR_problems: i32;
    pub static mut cur_dir: small_number;
    pub static mut pseudo_files: i32;
    pub static mut grp_stack: *mut save_pointer;
    pub static mut if_stack: *mut i32;
    pub static mut max_reg_num: i32;
    pub static mut max_reg_help_line: *const libc::c_char;
    pub static mut sa_root: [i32; 8];
    pub static mut cur_ptr: i32;
    pub static mut sa_null: memory_word;
    pub static mut sa_chain: i32;
    pub static mut sa_level: u16;
    pub static mut hyph_start: trie_pointer;
    pub static mut hyph_index: trie_pointer;
    pub static mut disc_ptr: [i32; 4];
    pub static mut edit_name_start: pool_pointer;
    pub static mut stop_at_space: bool;
    pub static mut native_font_type_flag: i32;
    pub static mut xtx_ligature_present: bool;
    pub static mut delta: scaled_t;
    pub static mut synctex_enabled: libc::c_int;
    pub static mut used_tectonic_coda_tokens: bool;
    pub static mut semantic_pagination_enabled: bool;
    pub static mut gave_char_warning_help: bool;

    pub static mut _xeq_level_array: [u16; (EQTB_SIZE - INT_BASE + 1) as usize];
}

#[macro_export]
macro_rules! XEQ_LEVEL {
    ($i:expr) => {
        $crate::xetex::xetex_xetexd::_xeq_level_array[(($i) as i32 - $crate::xetex::xetex_format::INT_BASE) as usize]
    };
}

unsafe extern "C" {
    pub fn getmd5sum(s: i32, file: bool);
    pub fn init_start_time(source_date_epoch: libc::time_t);
    pub fn get_date_and_time(source_date_epoch: libc::time_t, minutes: *mut i32, day: *mut i32, month: *mut i32, year: *mut i32);
    pub fn get_seconds_and_micros(seconds: *mut i32, micros: *mut i32);
    pub fn getcreationdate();
    pub fn getfilemoddate(s: i32);
    pub fn getfilesize(s: i32);
    pub fn getfiledump(s: i32, offset: libc::c_int, length: libc::c_int);
    pub fn gettexstring(s: str_number) -> *mut libc::c_char;
    pub fn is_new_source(s: str_number, line: libc::c_int) -> bool;
    pub fn make_src_special(s: str_number, line: libc::c_int) -> pool_pointer;
    pub fn remember_source_info(s: str_number, line: libc::c_int);

    pub fn badness(t: scaled_t, s: scaled_t) -> i32;
    pub fn print_word(w: memory_word);
    pub fn show_token_list(p: i32, q: i32, l: i32);
    pub fn runaway();
    pub fn get_avail() -> i32;
    pub fn flush_list(p: i32);
    pub fn get_node(s: i32) -> i32;
    pub fn free_node(p: i32, s: i32);
    pub fn new_null_box() -> i32;
    pub fn new_rule() -> i32;
    pub fn new_ligature(f: internal_font_number, c: u16, q: i32) -> i32;
    pub fn new_lig_item(c: u16) -> i32;
    pub fn new_disc() -> i32;
    pub fn copy_native_glyph_info(src: i32, dest: i32);
    pub fn new_math(w: scaled_t, s: small_number) -> i32;
    pub fn new_spec(p: i32) -> i32;
    pub fn new_param_glue(n: small_number) -> i32;
    pub fn new_glue(q: i32) -> i32;
    pub fn new_skip_param(n: small_number) -> i32;
    pub fn new_kern(w: scaled_t) -> i32;
    pub fn new_penalty(m: i32) -> i32;
    pub fn check_mem(print_locs: bool);
    pub fn search_mem(p: i32);
    pub fn prev_rightmost(s: i32, e: i32) -> i32;
    pub fn get_microinterval() -> i32;
    pub fn round_xn_over_d(x: scaled_t, n: i32, d: i32) -> scaled_t;
    pub fn short_display(p: i32);
    pub fn print_font_and_char(p: i32);
    pub fn print_mark(p: i32);
    pub fn print_rule_dimen(d: scaled_t);
    pub fn print_glue(d: scaled_t, order: i32, s: *const libc::c_char);
    pub fn print_spec(p: i32, s: *const libc::c_char);
    pub fn print_fam_and_char(p: i32);
    pub fn print_delimiter(p: i32);
    pub fn print_subsidiary_data(p: i32, c: UTF16_code);
    pub fn print_style(c: i32);
    pub fn print_skip_param(n: i32);
    pub fn show_node_list(p: i32);
    pub fn show_box(p: i32);
    pub fn short_display_n(p: i32, m: i32);
    pub fn delete_token_ref(p: i32);
    pub fn delete_glue_ref(p: i32);
    pub fn flush_node_list(p: i32);
    pub fn copy_node_list(p: i32) -> i32;
    pub fn print_mode(m: i32);
    pub fn print_in_mode(m: i32);
    pub fn push_nest();
    pub fn pop_nest();
    pub fn show_activities();
    pub fn print_param(n: i32);
    pub fn begin_diagnostic();
    pub fn end_diagnostic(blank_line: bool);
    pub fn print_length_param(n: i32);
    pub fn print_cmd_chr(cmd: u16, chr_code: i32);
    pub fn not_aat_font_error(cmd: i32, c: i32, f: i32);
    pub fn not_aat_gr_font_error(cmd: i32, c: i32, f: i32);
    pub fn not_ot_font_error(cmd: i32, c: i32, f: i32);
    pub fn not_native_font_error(cmd: i32, c: i32, f: i32);
    pub fn show_eqtb(n: i32);
    pub fn id_lookup(j: i32, l: i32) -> i32;
    pub fn prim_lookup(s: str_number) -> i32;
    pub fn restore_trace(p: i32, s: str_number);
    pub fn print_group(e: bool);
    pub fn group_trace(e: bool);
    pub fn pseudo_input() -> bool;
    pub fn pseudo_close();
    pub fn group_warning();
    pub fn if_warning();
    pub fn file_warning();
    pub fn delete_sa_ref(q: i32);
    pub fn show_sa(p: i32, s: str_number);
    pub fn sa_save(p: i32);
    pub fn sa_destroy(p: i32);
    pub fn sa_def(p: i32, e: i32);
    pub fn sa_w_def(p: i32, w: i32);
    pub fn gsa_def(p: i32, e: i32);
    pub fn gsa_w_def(p: i32, w: i32);
    pub fn sa_restore();
    pub fn new_save_level(c: group_code);
    pub fn eq_destroy(w: memory_word);
    pub fn eq_save(p: i32, l: u16);
    pub fn eq_define(p: i32, t: u16, e: i32);
    pub fn eq_word_define(p: i32, w: i32);
    pub fn geq_define(p: i32, t: u16, e: i32);
    pub fn geq_word_define(p: i32, w: i32);
    pub fn save_for_after(t: i32);
    pub fn unsave();
    pub fn prepare_mag();
    pub fn token_show(p: i32);
    pub fn print_meaning();
    pub fn show_cur_cmd_chr();
    pub fn show_context();
    pub fn begin_token_list(p: i32, t: u16);
    pub fn end_token_list();
    pub fn back_input();
    pub fn back_error();
    pub fn ins_error();
    pub fn begin_file_reading();
    pub fn end_file_reading();
    pub fn check_outer_validity();
    pub fn get_next();
    pub fn get_token();
    pub fn macro_call();
    pub fn insert_relax();
    pub fn new_index(i: u16, q: i32);
    pub fn find_sa_element(t: small_number, n: i32, w: bool);
    pub fn expand();
    pub fn get_x_token();
    pub fn x_token();
    pub fn scan_left_brace();
    pub fn scan_optional_equals();
    pub fn scan_keyword(s: *const libc::c_char) -> bool;
    pub fn mu_error();
    pub fn scan_glyph_number(f: internal_font_number);
    pub fn scan_char_class();
    pub fn scan_char_class_not_ignored();
    pub fn scan_eight_bit_int();
    pub fn scan_usv_num();
    pub fn scan_char_num();
    pub fn scan_xetex_math_char_int();
    pub fn scan_math_class_int();
    pub fn scan_math_fam_int();
    pub fn scan_four_bit_int();
    pub fn scan_fifteen_bit_int();
    pub fn scan_delimiter_int();
    pub fn scan_register_num();
    pub fn scan_four_bit_int_or_18();
    pub fn get_x_or_protected();
    pub fn effective_char(err_p: bool, f: internal_font_number, c: u16) -> i32;
    pub fn scan_font_ident();
    pub fn find_font_dimen(writing: bool);
    pub fn scan_something_internal(level: small_number, negative: bool);
    pub fn scan_int();
    pub fn xetex_scan_dimen(mu: bool, inf: bool, shortcut: bool, requires_units: bool);
    pub fn scan_dimen(mu: bool, inf: bool, shortcut: bool);
    pub fn scan_decimal();
    pub fn scan_glue(level: small_number);
    pub fn add_or_sub(x: i32, y: i32, max_answer: i32, negative: bool) -> i32;
    pub fn quotient(n: i32, d: i32) -> i32;
    pub fn fract(x: i32, n: i32, d: i32, max_answer: i32) -> i32;
    pub fn scan_expr();
    pub fn scan_normal_glue();
    pub fn scan_mu_glue();
    pub fn scan_rule_spec() -> i32;
    pub fn scan_general_text();
    pub fn pseudo_start();
    pub fn str_toks_cat(b: pool_pointer, cat: small_number) -> i32;
    pub fn str_toks(b: pool_pointer) -> i32;
    pub fn the_toks() -> i32;
    pub fn ins_the_toks();
    pub fn conv_toks();
    pub fn scan_toks(macro_def: bool, xpand: bool) -> i32;
    pub fn read_toks(n: i32, r: i32, j: i32);
    pub fn pass_text();
    pub fn change_if_limit(l: small_number, p: i32);
    pub fn conditional();
    pub fn begin_name();
    pub fn more_name(c: UTF16_code) -> bool;
    pub fn end_name();
    pub fn pack_file_name(n: str_number, a: str_number, e: str_number);
    pub fn make_name_string() -> str_number;
    pub fn scan_file_name();
    pub fn pack_job_name(s: *const libc::c_char);
    pub fn open_log_file();
    pub fn start_input(primary_input_name: *const libc::c_char);
    pub fn effective_char_info(f: internal_font_number, c: u16) -> b16x4;
    pub fn char_warning(f: internal_font_number, c: i32);
    pub fn new_native_word_node(f: internal_font_number, n: i32) -> i32;
    pub fn new_native_character(f: internal_font_number, c: UnicodeScalar) -> i32;
    pub fn font_feature_warning(featureNameP: *const libc::c_void, featLen: i32, settingNameP: *const libc::c_void, setLen: i32);
    pub fn font_mapping_warning(mappingNameP: *const libc::c_void, mappingNameLen: i32, warningType: i32);
    pub fn graphite_warning();
    pub fn load_native_font(u: i32, nom: str_number, aire: str_number, s: scaled_t) -> internal_font_number;
    pub fn do_locale_linebreaks(s: i32, len: i32);
    pub fn bad_utf8_warning();
    pub fn get_input_normalization_state() -> i32;
    pub fn get_tracing_fonts_state() -> i32;
    pub fn read_font_info(u: i32, nom: str_number, aire: str_number, s: scaled_t) -> internal_font_number;
    pub fn new_character(f: internal_font_number, c: UTF16_code) -> i32;
    pub fn out_what(p: i32);
    pub fn new_edge(s: small_number, w: scaled_t) -> i32;
    pub fn scan_spec(c: group_code, three_codes: bool);
    pub fn char_pw(p: i32, side: small_number) -> scaled_t;
    pub fn new_margin_kern(w: scaled_t, p: i32, side: small_number) -> i32;
    pub fn hpack(p: i32, w: scaled_t, m: small_number) -> i32;
    pub fn vpackage(p: i32, h: scaled_t, m: small_number, l: scaled_t) -> i32;
    pub fn append_to_vlist(b: i32);
    pub fn new_noad() -> i32;
    pub fn new_style(s: small_number) -> i32;
    pub fn new_choice() -> i32;
    pub fn show_info();
    pub fn push_alignment();
    pub fn pop_alignment();
    pub fn get_preamble_token();
    pub fn init_align();
    pub fn init_span(p: i32);
    pub fn init_row();
    pub fn init_col();
    pub fn fin_col() -> bool;
    pub fn fin_row();
    pub fn fin_align();
    pub fn align_peek();
    pub fn max_hyphenatable_length() -> i32;
    pub fn new_trie_op(d: small_number, n: small_number, v: trie_opcode) -> trie_opcode;
    pub fn trie_node(p: trie_pointer) -> trie_pointer;
    pub fn compress_trie(p: trie_pointer) -> trie_pointer;
    pub fn first_fit(p: trie_pointer);
    pub fn trie_pack(p: trie_pointer);
    pub fn trie_fix(p: trie_pointer);
    pub fn init_trie();
    pub fn line_break(d: bool);
    pub fn eTeX_enabled(b: bool, j: u16, k: i32) -> bool;
    pub fn show_save_groups();
    pub fn prune_page_top(p: i32, s: bool) -> i32;
    pub fn vert_break(p: i32, h: scaled_t, d: scaled_t) -> i32;
    pub fn do_marks(a: small_number, l: small_number, q: i32) -> bool;
    pub fn vsplit(n: i32, h: scaled_t) -> i32;
    pub fn print_totals();
    pub fn box_error(n: eight_bits);
    pub fn app_space();
    pub fn insert_dollar_sign();
    pub fn you_cant();
    pub fn report_illegal_case();
    pub fn privileged() -> bool;
    pub fn its_all_over() -> bool;
    pub fn append_glue();
    pub fn append_kern();
    pub fn off_save();
    pub fn extra_right_brace();
    pub fn normal_paragraph();
    pub fn box_end(box_context: i32);
    pub fn begin_box(box_context: i32);
    pub fn scan_box(box_context: i32);
    pub fn package(c: small_number);
    pub fn norm_min(h: i32) -> small_number;
    pub fn new_graf(indented: bool);
    pub fn indent_in_hmode();
    pub fn head_for_vmode();
    pub fn end_graf();
    pub fn begin_insert_or_adjust();
    pub fn make_mark();
    pub fn append_penalty();
    pub fn delete_last();
    pub fn unpackage();
    pub fn append_italic_correction();
    pub fn append_discretionary();
    pub fn build_discretionary();
    pub fn make_accent();
    pub fn align_error();
    pub fn no_align_error();
    pub fn omit_error();
    pub fn do_endv();
    pub fn cs_error();
    pub fn push_math(c: group_code);
    pub fn just_copy(p: i32, h: i32, t: i32);
    pub fn just_reverse(p: i32);
    pub fn scan_math(p: i32);
    pub fn set_math_char(c: i32);
    pub fn resume_after_display();
    pub fn get_r_token();
    pub fn trap_zero_glue();
    pub fn do_register_command(a: small_number);
    pub fn alter_aux();
    pub fn alter_prev_graf();
    pub fn alter_page_so_far();
    pub fn alter_integer();
    pub fn alter_box_dimen();
    pub fn new_font(a: small_number);
    pub fn new_interaction();
    pub fn prefixed_command();
    pub fn do_assignments();
    pub fn open_or_close_in();
    pub fn issue_message();
    pub fn shift_case();
    pub fn show_whatever();
    pub fn new_whatsit(s: small_number, w: small_number);
    pub fn new_write_whatsit(w: small_number);
    pub fn load_picture(is_pdf: bool);
    pub fn scan_and_pack_name();
    pub fn do_extension();
    pub fn fix_language();
    pub fn insert_src_special();
    pub fn append_src_special();
    pub fn handle_right_brace();
    pub fn main_control();
    pub fn give_err_help();
    pub fn close_files_and_terminate();
    pub fn debug_help();
    pub fn flush_str(s: str_number);
    pub fn tokens_to_string(p: i32) -> str_number;
    pub fn scan_pdf_ext_toks();
    pub fn compare_strings();

    pub fn error();
    pub fn fatal_error(s: *const libc::c_char) -> !;
    pub fn overflow(s: *const libc::c_char, n: i32) -> !;
    pub fn confusion(s: *const libc::c_char) -> !;
    pub fn pdf_error(t: *const libc::c_char, p: *const libc::c_char) -> !;

    pub fn initialize_math_variables();
    pub fn init_math();
    pub fn after_math();
    pub fn start_eq_no();
    pub fn math_limit_switch();
    pub fn math_radical();
    pub fn math_ac();
    pub fn append_choices();
    pub fn fin_mlist(p: i32) -> i32;
    pub fn build_choices();
    pub fn sub_sup();
    pub fn math_fraction();
    pub fn math_left_right();
    pub fn flush_math();

    pub fn diagnostic_begin_capture_warning_here() -> *mut ttbc_diagnostic_t;
    pub fn capture_to_diagnostic(diagnostic: *mut ttbc_diagnostic_t);
    pub fn error_here_with_diagnostic(message: *const libc::c_char) -> *mut ttbc_diagnostic_t;
    pub fn print_ln();
    pub fn print_raw_char(s: UTF16_code, incr_offset: bool);
    pub fn print_char(s: i32);
    pub fn print(s: i32);
    pub fn print_cstr(s: *const libc::c_char);
    pub fn print_nl(s: str_number);
    pub fn print_nl_cstr(s: *const libc::c_char);
    pub fn print_esc(s: str_number);
    pub fn print_esc_cstr(s: *const libc::c_char);
    pub fn print_int(n: i32);
    pub fn print_cs(p: i32);
    pub fn sprint_cs(p: i32);
    pub fn print_file_name(n: i32, a: i32, e: i32);
    pub fn print_size(s: i32);
    pub fn print_write_whatsit(s: *const libc::c_char, p: i32);
    pub fn print_native_word(p: i32);
    pub fn print_sa_num(q: i32);
    pub fn print_file_line();
    pub fn print_two(n: i32);
    pub fn print_hex(n: i32);
    pub fn print_roman_int(n: i32);
    pub fn print_current_string();
    pub fn print_scaled(s: scaled_t);
    pub fn print_ucs_code(n: UnicodeScalar);

    pub fn initialize_pagebuilder_variables();
    pub fn build_page();

    pub fn tex_round(d: f64) -> i32;
    pub fn half(x: i32) -> i32;
    pub fn mult_and_add(n: i32, x: scaled_t, y: scaled_t, max_answer: scaled_t) -> scaled_t;
    pub fn x_over_n(x: scaled_t, n: i32) -> scaled_t;
    pub fn xn_over_d(x: scaled_t, n: i32, d: i32) -> scaled_t;
    pub fn init_randoms(seed: i32);
    pub fn unif_rand(x: i32) -> i32;
    pub fn norm_rand() -> i32;

    pub fn initialize_shipout_variables();
    pub fn deinitialize_shipout_variables();
    pub fn ship_out(p: i32);
    pub fn finalize_dvi_file();

    pub fn tt_cleanup();
    pub fn tt_run_engine(dump_name: *const libc::c_char, input_file_name: *const libc::c_char, build_date: libc::time_t) -> tt_history_t;
    pub fn tt_insert_special(ascii_text: *const libc::c_char);

    pub fn measure_native_node(node: *mut std::ffi::c_void, use_glyph_metrics: libc::c_int);
    pub fn measure_native_glyph(node: *mut std::ffi::c_void, use_glyph_metrics: libc::c_int);
    pub fn store_justified_native_glyphs(node: *mut std::ffi::c_void);
    pub fn real_get_native_italic_correction(node: *mut std::ffi::c_void) -> scaled_t;
    pub fn real_get_native_glyph_italic_correction(node: *mut std::ffi::c_void) -> scaled_t;
    pub fn real_get_native_glyph(pNode: *mut std::ffi::c_void, index: libc::c_uint) -> u16;
    pub fn makeXDVGlyphArrayData(p: *mut std::ffi::c_void) -> libc::c_int;
    pub fn real_get_native_word_cp(node: *mut std::ffi::c_void, side: libc::c_int) -> i32;

    pub fn load_pool_strings(spare_size: i32) -> libc::c_int;
    pub fn length(s: str_number) -> i32;
    pub fn make_string() -> str_number;
    pub fn append_str(s: str_number);
    pub fn str_eq_buf(s: str_number, k: i32) -> bool;
    pub fn str_eq_str(s: str_number, t: str_number) -> bool;
    pub fn search_string(search: str_number) -> str_number;
    pub fn slow_make_string() -> str_number;
}

#[inline]
pub fn is_char_node(p: i32) -> bool {
    p >= unsafe { hi_mem_min }
}

#[inline]
pub fn is_non_discardable_node(p: i32) -> bool {
    (unsafe { NODE_type!(p) as i32 }) < MATH_NODE
}

#[inline]
pub unsafe fn print_c_string(mut s: *const libc::c_char) {
    unsafe {
        while *s != 0 {
            print_char(*s as i32);
            s = s.offset(1);
        }
    }
}

#[inline]
pub unsafe fn cur_length() -> pool_pointer {
    unsafe { pool_ptr - *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) }
}

#[macro_export]
macro_rules! set_native_metrics {
    ($p:expr, $g:expr) => {
        $crate::xetex::xetex_xetexd::measure_native_node(
            $crate::xetex::xetex_xetexd::mem.offset(($p) as isize) as *mut std::ffi::c_void,
            $g as libc::c_int,
        )
    };
}

#[macro_export]
macro_rules! set_native_glyph_metrics {
    ($p:expr, $g:expr) => {
        $crate::xetex::xetex_xetexd::measure_native_glyph(
            $crate::xetex::xetex_xetexd::mem.offset(($p) as isize) as *mut std::ffi::c_void,
            $g as libc::c_int,
        )
    };
}

#[macro_export]
macro_rules! set_justified_native_glyphs {
    ($p:expr) => {
        $crate::xetex::xetex_xetexd::store_justified_native_glyphs($crate::xetex::xetex_xetexd::mem.offset(($p) as isize) as *mut std::ffi::c_void)
    };
}

#[macro_export]
macro_rules! get_native_italic_correction {
    ($p:expr) => {
        $crate::xetex::xetex_xetexd::real_get_native_italic_correction(
            $crate::xetex::xetex_xetexd::mem.offset(($p) as isize) as *mut std::ffi::c_void,
        )
    };
}

#[macro_export]
macro_rules! get_native_glyph_italic_correction {
    ($p:expr) => {
        $crate::xetex::xetex_xetexd::real_get_native_glyph_italic_correction(
            $crate::xetex::xetex_xetexd::mem.offset(($p) as isize) as *mut std::ffi::c_void
        )
    };
}

#[macro_export]
macro_rules! get_native_glyph {
    ($p:expr, $i:expr) => {
        $crate::xetex::xetex_xetexd::real_get_native_glyph(
            $crate::xetex::xetex_xetexd::mem.offset(($p) as isize) as *mut std::ffi::c_void,
            $i as libc::c_uint,
        )
    };
}

#[macro_export]
macro_rules! make_xdv_glyph_array_data {
    ($p:expr) => {
        $crate::xetex::xetex_xetexd::makeXDVGlyphArrayData($crate::xetex::xetex_xetexd::mem.offset(($p) as isize) as *mut std::ffi::c_void)
    };
}

#[macro_export]
macro_rules! get_native_word_cp {
    ($p:expr, $s:expr) => {
        $crate::xetex::xetex_xetexd::real_get_native_word_cp(
            $crate::xetex::xetex_xetexd::mem.offset(($p) as isize) as *mut std::ffi::c_void,
            $s as libc::c_int,
        )
    };
}

#[inline]
pub fn math_fam(x: u32) -> u32 {
    (x >> 24) & 0xFF
}

#[inline]
pub fn math_class(x: u32) -> u32 {
    (x >> 21) & 0x07
}

#[inline]
pub fn math_char(x: u32) -> u32 {
    x & 0x1FFFFF
}

#[inline]
pub fn set_family(x: u32) -> u32 {
    (x & 0xFF) << 24
}

#[inline]
pub fn set_class(x: u32) -> u32 {
    (x & 0x07) << 21
}

pub const AUTO: i32 = 0;
pub const UTF8: i32 = 1;
pub const UTF16BE: i32 = 2;
pub const UTF16LE: i32 = 3;
pub const RAW: i32 = 4;
pub const ICUMAPPING: i32 = 5;

pub const EMPTY_STRING: i32 = 65536 + 1;
