// Copyright 2016-2018 The Tectonic Project
// Licensed under the MIT License.

#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use libc::c_void;
use std::ptr;

pub type scaled_t = i32;
pub type str_number = i32;
pub type pool_pointer = i32;
pub type eight_bits = u8;
pub type UTF16_code = u16;
pub type internal_font_number = i32;
pub type small_number = i16;
pub type glue_ord = u8;
pub type rust_output_handle_t = usize;

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b32x2 {
    pub s1: i32,
    pub s0: i32,
}

#[cfg(not(target_endian = "big"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b32x2 {
    pub s0: i32,
    pub s1: i32,
}

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b16x4 {
    pub s3: u16,
    pub s2: u16,
    pub s1: u16,
    pub s0: u16,
}

#[cfg(not(target_endian = "big"))]
#[repr(C)]
#[derive(Copy, Clone)]
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
    pub int_: i32,
    pub ptr: *mut c_void,
}

#[repr(C)]
pub struct tt_checkpoint {
    pub bytes: *mut u8,
    pub length: usize,
    pub capacity: usize,
    pub position: usize,
    pub restoring: bool,
    pub valid: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_state_record {
    pub mode: i32,
    pub head: i32,
    pub tail: i32,
    pub eTeX_aux: i32,
    pub prev_graf: i32,
    pub mode_line: i32,
    pub aux: memory_word,
}

pub const DVI_BUF_SIZE: usize = 16384;
pub const HALF_BUF: usize = 8192;
pub const FNT_NUM_0: i32 = 171;

pub const MIN_HALFWORD: i32 = -0x0FFFFFFF;
pub const MAX_HALFWORD: i32 = 0x3FFFFFFF;
pub const TEX_NULL: i32 = MIN_HALFWORD;
pub const TEX_INFINITY: i32 = 0x7FFFFFFF;
pub const NULL_FLAG: i32 = -0x40000000;
pub const INVALID_HANDLE: rust_output_handle_t = 0;

pub const TOO_BIG_CHAR: i32 = 65536;
pub const FONT_BASE: i32 = 0;
pub const MEM_TOP: i32 = 4999999;
pub const LIG_TRICK: i32 = MEM_TOP - 12;

pub const LEFT_TO_RIGHT: i32 = 0;
pub const RIGHT_TO_LEFT: i32 = 1;

pub const BEFORE: i32 = 0;
pub const REVERSED: i32 = 1;
pub const DLIST: i32 = 2;

pub const L_CODE: i32 = 4;
pub const R_CODE: i32 = 8;
pub const END_M_CODE: i32 = 3;

pub const NORMAL: u8 = 0;
pub const STRETCHING: u8 = 1;
pub const SHRINKING: u8 = 2;

pub const FIL: u8 = 1;
pub const FILL: u8 = 2;
pub const FILLL: u8 = 3;

pub const SYNCTEX_FIELD_SIZE: usize = 1;
pub const SMALL_NODE_SIZE: i32 = 2;
pub const MEDIUM_NODE_SIZE: i32 = 3;
pub const MOVEMENT_NODE_SIZE: i32 = 3;
pub const EDGE_NODE_SIZE: i32 = 3;
pub const GLUE_SPEC_SIZE: i32 = 4;
pub const NATIVE_NODE_SIZE: i32 = 6;
pub const PIC_NODE_SIZE: i32 = 9;

pub const HLIST_NODE: u16 = 0;
pub const VLIST_NODE: u16 = 1;
pub const RULE_NODE: u16 = 2;
pub const INS_NODE: u16 = 3;
pub const MARK_NODE: u16 = 4;
pub const ADJUST_NODE: u16 = 5;
pub const LIGATURE_NODE: u16 = 6;
pub const WHATSIT_NODE: u16 = 8;
pub const MATH_NODE: u16 = 9;
pub const GLUE_NODE: u16 = 10;
pub const KERN_NODE: u16 = 11;
pub const PENALTY_NODE: u16 = 12;
pub const STYLE_NODE: u16 = 14;
pub const EDGE_NODE: u16 = STYLE_NODE;
pub const MARGIN_KERN_NODE: u16 = 40;

pub const OPEN_NODE: u16 = 0;
pub const WRITE_NODE: u16 = 1;
pub const CLOSE_NODE: u16 = 2;
pub const SPECIAL_NODE: u16 = 3;
pub const LANGUAGE_NODE: u16 = 4;
pub const PDF_SAVE_POS_NODE: u16 = 21;
pub const NATIVE_WORD_NODE: u16 = 40;
pub const NATIVE_WORD_NODE_AT: u16 = 41;
pub const GLYPH_NODE: u16 = 42;
pub const PIC_NODE: u16 = 43;
pub const PDF_NODE: u16 = 44;

pub const SPACE_ADJUSTMENT: u16 = 3;
pub const A_LEADERS: u16 = 100;
pub const C_LEADERS: u16 = 101;

pub const SET1: u8 = 128;
pub const SET_RULE: u8 = 132;
pub const PUT_RULE: u8 = 137;
pub const BOP: u8 = 139;
pub const EOP: u8 = 140;
pub const PUSH: u8 = 141;
pub const POP: u8 = 142;
pub const RIGHT1: u8 = 143;
pub const DOWN1: u8 = 157;
pub const FNT1: u8 = 235;
pub const XXX1: u8 = 239;
pub const XXX4: u8 = 242;
pub const FNT_DEF1: u8 = 243;
pub const PRE: u8 = 247;
pub const POST: u8 = 248;
pub const POST_POST: u8 = 249;
pub const DEFINE_NATIVE_FONT: u8 = 252;
pub const SET_GLYPHS: u8 = 253;
pub const SET_TEXT_AND_GLYPHS: u8 = 254;

pub const XDV_ID_BYTE: u8 = 7;
pub const SPX_ID_BYTE: u8 = 100;

pub const MOV_NONE_SEEN: i16 = 0;
pub const MOV_Y_HERE: i32 = 1;
pub const MOV_Z_HERE: i32 = 2;
pub const MOV_YZ_OK: i32 = 3;
pub const MOV_Y_OK: i32 = 4;
pub const MOV_Z_OK: i32 = 5;
pub const MOV_Y_SEEN: i16 = 6;
pub const MOV_D_FIXED: i32 = 6;
pub const MOV_Z_SEEN: i16 = 12;

pub const AAT_FONT_FLAG: u32 = 0xFFFF;
pub const OTGR_FONT_FLAG: u32 = 0xFFFE;

pub const CS_TOKEN_FLAG: i32 = 0x1FFFFFF;
pub const LEFT_BRACE_TOKEN: i32 = 0x200000;
pub const RIGHT_BRACE_TOKEN: i32 = 0x400000;
pub const FROZEN_CONTROL_SEQUENCE: i32 = 2243226;
pub const END_WRITE: i32 = FROZEN_CONTROL_SEQUENCE + 8;
pub const INSERTED: u8 = 5;
pub const WRITE_TEXT: u8 = 18;

pub const SELECTOR_TERM_ONLY: i32 = 17;
pub const SELECTOR_LOG_ONLY: i32 = 18;
pub const SELECTOR_TERM_AND_LOG: i32 = 19;
pub const SELECTOR_NEW_STRING: i32 = 21;

pub const INT_BASE: i32 = 7826729;
pub const COUNT_BASE: i32 = 7826812;
pub const DIMEN_BASE: i32 = 8941180;

pub const INT_PAR__mag: i32 = 17;
pub const INT_PAR__tracing_online: i32 = 29;
pub const INT_PAR__tracing_output: i32 = 34;
pub const INT_PAR__xetex_interword_space_shaping: i32 = 78;

pub const DIMEN_PAR__h_offset: i32 = 18;
pub const DIMEN_PAR__v_offset: i32 = 19;
pub const DIMEN_PAR__pdf_page_width: i32 = 21;
pub const DIMEN_PAR__pdf_page_height: i32 = 22;

pub static mut dvi_file: rust_output_handle_t = 0;
pub static mut output_file_name: str_number = 0;
pub static mut dvi_buf: *mut eight_bits = ptr::null_mut();
pub static mut dvi_limit: i32 = 0;
pub static mut g: i32 = 0;
pub static mut lq: i32 = 0;
pub static mut lr: i32 = 0;
pub static mut dvi_ptr: i32 = 0;
pub static mut dvi_offset: i32 = 0;
pub static mut dvi_gone: i32 = 0;
pub static mut down_ptr: i32 = 0;
pub static mut right_ptr: i32 = 0;
pub static mut dvi_h: scaled_t = 0;
pub static mut dvi_v: scaled_t = 0;
pub static mut dvi_f: internal_font_number = 0;
pub static mut cur_s: i32 = 0;

macro_rules! mem {
    ($p:expr) => {
        *mem.offset(($p) as isize)
    };
}

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
macro_rules! BOX_lr_mode {
    ($p:expr) => {
        (*mem.offset(($p) as isize)).b16.s0
    };
}
macro_rules! BOX_width {
    ($p:expr) => {
        (*mem.offset((($p) + 1) as isize)).b32.s1
    };
}
macro_rules! BOX_depth {
    ($p:expr) => {
        (*mem.offset((($p) + 2) as isize)).b32.s1
    };
}
macro_rules! BOX_height {
    ($p:expr) => {
        (*mem.offset((($p) + 3) as isize)).b32.s1
    };
}
macro_rules! BOX_shift_amount {
    ($p:expr) => {
        (*mem.offset((($p) + 4) as isize)).b32.s1
    };
}
macro_rules! BOX_list_ptr {
    ($p:expr) => {
        (*mem.offset((($p) + 5) as isize)).b32.s1
    };
}
macro_rules! BOX_glue_sign {
    ($p:expr) => {
        (*mem.offset((($p) + 5) as isize)).b16.s1
    };
}
macro_rules! BOX_glue_order {
    ($p:expr) => {
        (*mem.offset((($p) + 5) as isize)).b16.s0
    };
}
macro_rules! BOX_glue_set {
    ($p:expr) => {
        (*mem.offset((($p) + 6) as isize)).gr
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
macro_rules! EDGE_NODE_edge_dist {
    ($p:expr) => {
        (*mem.offset((($p) + 2) as isize)).b32.s1
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
macro_rules! NATIVE_NODE_glyph_info_ptr {
    ($p:expr) => {
        (*mem.offset((($p) + 5) as isize)).ptr
    };
}
macro_rules! NATIVE_NODE_text {
    ($p:expr) => {
        mem.offset((($p) + NATIVE_NODE_SIZE as i32) as isize) as *mut u16
    };
}
macro_rules! PIC_NODE_path_len {
    ($p:expr) => {
        (*mem.offset((($p) + 4) as isize)).b16.s1
    };
}
macro_rules! PIC_NODE_path {
    ($p:expr) => {
        mem.offset((($p) + PIC_NODE_SIZE as i32) as isize) as *mut u8
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
macro_rules! SYNCTEX_tag {
    ($p:expr, $nodesize:expr) => {
        (*mem.offset((($p) + ($nodesize) as i32 - SYNCTEX_FIELD_SIZE as i32) as isize))
            .b32
            .s0
    };
}
macro_rules! MATH_NODE_lr_dir {
    ($p:expr) => {
        (NODE_subtype!($p) as i32 / R_CODE)
    };
}
macro_rules! MATH_NODE_end_lr_type {
    ($p:expr) => {
        (L_CODE * (NODE_subtype!($p) as i32 / L_CODE) + END_M_CODE)
    };
}
macro_rules! FONT_CHARACTER_INFO {
    ($f:expr, $c:expr) => {
        (*font_info.offset((*char_base.offset(($f) as isize) + ($c) as i32) as isize)).b16
    };
}
macro_rules! FONT_CHARINFO_WIDTH {
    ($f:expr, $info:expr) => {
        (*font_info.offset((*width_base.offset(($f) as isize) + ($info).s3 as i32) as isize))
            .b32
            .s1
    };
}
macro_rules! FONT_CHARACTER_WIDTH {
    ($f:expr, $c:expr) => {
        FONT_CHARINFO_WIDTH!($f, FONT_CHARACTER_INFO!($f, $c))
    };
}
macro_rules! INTPAR {
    (mag) => {
        (*eqtb.offset((INT_BASE + INT_PAR__mag) as isize)).b32.s1
    };
    (tracing_online) => {
        (*eqtb.offset((INT_BASE + INT_PAR__tracing_online) as isize))
            .b32
            .s1
    };
    (tracing_output) => {
        (*eqtb.offset((INT_BASE + INT_PAR__tracing_output) as isize))
            .b32
            .s1
    };
    (xetex_interword_space_shaping) => {
        (*eqtb.offset((INT_BASE + INT_PAR__xetex_interword_space_shaping) as isize))
            .b32
            .s1
    };
}
macro_rules! COUNT_REG {
    ($n:expr) => {
        (*eqtb.offset((COUNT_BASE + ($n) as i32) as isize)).b32.s1
    };
}
macro_rules! DIMENPAR {
    (h_offset) => {
        (*eqtb.offset((DIMEN_BASE + DIMEN_PAR__h_offset) as isize))
            .b32
            .s1
    };
    (v_offset) => {
        (*eqtb.offset((DIMEN_BASE + DIMEN_PAR__v_offset) as isize))
            .b32
            .s1
    };
    (pdf_page_width) => {
        (*eqtb.offset((DIMEN_BASE + DIMEN_PAR__pdf_page_width) as isize))
            .b32
            .s1
    };
    (pdf_page_height) => {
        (*eqtb.offset((DIMEN_BASE + DIMEN_PAR__pdf_page_height) as isize))
            .b32
            .s1
    };
}

#[inline(always)]
pub fn odd(x: i32) -> bool {
    (x & 1) != 0
}

#[inline(always)]
pub unsafe fn is_char_node(p: i32) -> bool {
    unsafe { p >= hi_mem_min }
}

#[inline(always)]
pub unsafe fn cur_length() -> pool_pointer {
    unsafe { pool_ptr - *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) }
}

#[inline(always)]
pub unsafe fn print_c_string(mut s: *const libc::c_char) {
    unsafe {
        while *s != 0 {
            print_char(*s as i32);
            s = s.offset(1);
        }
    }
}

unsafe extern "C" {
    pub static mut job_name: str_number;
    pub static mut term_offset: i32;
    pub static mut file_offset: i32;
    pub static mut max_print_line: i32;
    pub static mut rust_stdout: rust_output_handle_t;
    pub static mut help_ptr: u8;
    pub static mut help_line: [*const libc::c_char; 6];
    pub static mut max_v: scaled_t;
    pub static mut max_h: scaled_t;
    pub static mut cur_h: scaled_t;
    pub static mut cur_v: scaled_t;
    pub static mut cur_h_offset: scaled_t;
    pub static mut cur_v_offset: scaled_t;
    pub static mut cur_page_width: scaled_t;
    pub static mut cur_page_height: scaled_t;
    pub static mut output_file_extension: *const libc::c_char;
    pub static mut name_of_file: *mut libc::c_char;
    pub static mut total_pages: i32;
    pub static mut semantic_pagination_enabled: bool;
    pub static mut last_bop: i32;
    pub static mut selector: i32;
    pub static mut str_start: *mut pool_pointer;
    pub static mut str_ptr: str_number;
    pub static mut str_pool: *mut u16;
    pub static mut pool_ptr: pool_pointer;
    pub static mut pool_size: i32;
    pub static mut init_pool_ptr: pool_pointer;
    pub static mut temp_ptr: i32;
    pub static mut LR_problems: i32;
    pub static mut LR_ptr: i32;
    pub static mut cur_dir: small_number;
    pub static mut dead_cycles: i32;
    pub static mut font_letter_space: *mut scaled_t;
    pub static mut font_glue: *mut i32;
    pub static mut font_mapping: *mut *mut libc::c_void;
    pub static mut font_used: *mut bool;
    pub static mut font_ec: *mut UTF16_code;
    pub static mut font_bc: *mut UTF16_code;
    pub static mut font_info: *mut memory_word;
    pub static mut char_base: *mut i32;
    pub static mut width_base: *mut i32;
    pub static mut max_push: i32;
    pub static mut avail: i32;
    pub static mut rule_ht: scaled_t;
    pub static mut rule_dp: scaled_t;
    pub static mut rule_wd: scaled_t;
    pub static mut xdv_buffer: *mut u8;
    pub static mut pdf_last_x_pos: scaled_t;
    pub static mut pdf_last_y_pos: scaled_t;
    pub static mut xtx_ligature_present: bool;
    pub static mut doing_leaders: bool;
    pub static mut write_open: [bool; 16];
    pub static mut write_file: [rust_output_handle_t; 16];
    pub static mut cur_name: str_number;
    pub static mut cur_area: str_number;
    pub static mut cur_ext: str_number;
    pub static mut log_opened: bool;
    pub static mut font_area: *mut str_number;
    pub static mut font_check: *mut b16x4;
    pub static mut font_size: *mut scaled_t;
    pub static mut font_dsize: *mut scaled_t;
    pub static mut font_name: *mut str_number;
    pub static mut doing_special: bool;
    pub static mut cur_list: list_state_record;
    pub static mut cur_cs: i32;
    pub static mut write_loc: i32;
    pub static mut cur_tok: i32;
    pub static mut def_ref: i32;
    pub static mut shell_escape_enabled: bool;
    pub static mut font_ptr: internal_font_number;
    pub static mut mem: *mut memory_word;
    pub static mut eqtb: *mut memory_word;
    pub static mut hi_mem_min: i32;

    pub fn synctex_sheet(mag: i32);
    pub fn synctex_teehs();
    pub fn synctex_hlist(this_box: i32);
    pub fn synctex_vlist(this_box: i32);
    pub fn synctex_void_hlist(p: i32, this_box: i32);
    pub fn synctex_void_vlist(p: i32, this_box: i32);
    pub fn synctex_current();
    pub fn synctex_kern(p: i32, this_box: i32);
    pub fn synctex_math(p: i32, this_box: i32);
    pub fn synctex_horizontal_rule_or_glue(p: i32, this_box: i32);
    pub fn synctex_tsilh(this_box: i32);
    pub fn synctex_tsilv(this_box: i32);

    pub fn open_log_file();
    pub fn print_nl_cstr(s: *const libc::c_char);
    pub fn print_ln();
    pub fn print_cstr(s: *const libc::c_char);
    pub fn print_char(c: i32);
    pub fn print_int(n: i32);
    pub fn print(s: i32);
    pub fn print_scaled(s: scaled_t);
    pub fn print_raw_char(c: UTF16_code, in_hex: bool);
    pub fn print_file_name(name: str_number, area: str_number, ext: str_number);
    pub fn begin_diagnostic();
    pub fn show_box(p: i32);
    pub fn end_diagnostic(blank_line: bool);
    pub fn error_here_with_diagnostic(
        msg: *const libc::c_char,
    ) -> *mut tectonic_bridge_core::Diagnostic;
    pub fn capture_to_diagnostic(diagnostic: *mut tectonic_bridge_core::Diagnostic);
    pub fn error();
    pub fn fatal_error(msg: *const libc::c_char) -> !;
    pub fn overflow(s: *const libc::c_char, n: i32) -> !;
    pub fn confusion(msg: *const libc::c_char) -> !;
    pub fn _tt_abort(format: *const libc::c_char, ...) -> !;
    pub fn pack_job_name(ext: *const libc::c_char);
    pub fn pack_file_name(name: str_number, area: str_number, ext: str_number);
    pub fn make_name_string() -> str_number;
    pub fn maketexstring(s: *const libc::c_char) -> str_number;
    pub fn prepare_mag();
    pub fn flush_node_list(p: i32);
    pub fn free_node(p: i32, s: i32);
    pub fn get_node(s: i32) -> i32;
    pub fn get_avail() -> i32;
    pub fn new_kern(w: scaled_t) -> i32;
    pub fn new_math(w: scaled_t, s: small_number) -> i32;
    pub fn new_native_word_node(f: internal_font_number, n: i32) -> i32;
    pub fn make_font_def(f: internal_font_number) -> i32;
    pub fn apply_tfm_font_mapping(mapping: *mut libc::c_void, c: i32) -> i32;
    pub fn effective_char(err_p: bool, f: internal_font_number, c: UTF16_code) -> i32;
    pub fn length(s: str_number) -> i32;
    pub fn tex_round(r: f64) -> i32;
    pub fn show_token_list(p: i32, q: i32, l: i32);
    pub fn scan_toks(macro_def: bool, xpand: bool) -> i32;
    pub fn get_token();
    pub fn begin_token_list(p: i32, t: u16);
    pub fn end_token_list();
    pub fn token_show(p: i32);
    pub fn flush_list(p: i32);
    pub fn diagnostic_begin_capture_warning_here() -> *mut tectonic_bridge_core::Diagnostic;

    pub fn tt_checkpoint_field(
        cp: *mut libc::c_void,
        name: *const libc::c_char,
        value: *mut libc::c_void,
        length: usize,
    );
    pub fn tt_checkpoint_array(
        cp: *mut tt_checkpoint,
        name: *const libc::c_char,
        value: *mut *mut libc::c_void,
        count: usize,
        item_size: usize,
    );
    pub fn tt_xetex_checkpoint_handle_valid(handle: usize, output: bool) -> bool;

    pub fn ttstub_output_open(path: *const libc::c_char, is_gz: i32) -> rust_output_handle_t;
    pub fn ttstub_output_close(handle: rust_output_handle_t) -> i32;
    pub fn ttstub_output_flush(handle: rust_output_handle_t) -> i32;
    pub fn ttstub_output_write(
        handle: rust_output_handle_t,
        data: *const libc::c_char,
        len: usize,
    ) -> usize;
    pub fn ttstub_shell_escape(cmd: *const u16, len: usize) -> i32;
    pub fn xmalloc(size: usize) -> *mut libc::c_void;
}

unsafe fn set_justified_native_glyphs(p: i32) {
    unsafe {
        super::xetex_ext::store_justified_native_glyphs(mem.add(p as usize).cast());
    }
}

unsafe fn make_xdv_glyph_array_data(p: i32) -> i32 {
    unsafe { super::xetex_ext::makeXDVGlyphArrayData(mem.add(p as usize).cast()) }
}

#[inline(always)]
unsafe fn cp_value<T>(cp: *mut tt_checkpoint, name: *const libc::c_char, val: *mut T) {
    unsafe {
        tt_checkpoint_field(cp.cast(), name, val as *mut libc::c_void, size_of::<T>());
    }
}

#[inline(always)]
unsafe fn cp_array<T>(
    cp: *mut tt_checkpoint,
    name: *const libc::c_char,
    val: *mut *mut T,
    count: usize,
) {
    unsafe {
        tt_checkpoint_array(
            cp,
            name,
            val as *mut *mut libc::c_void,
            count,
            size_of::<T>(),
        );
    }
}

#[unsafe(export_name = "tt_checkpoint_shipout")]
pub unsafe extern "C" fn tt_checkpoint_shipout(cp: *mut tt_checkpoint) {
    unsafe {
        cp_value(cp, c"dvi_file".as_ptr(), &raw mut dvi_file);
        cp_value(cp, c"output_file_name".as_ptr(), &raw mut output_file_name);
        cp_array(cp, c"dvi_buf".as_ptr(), &raw mut dvi_buf, DVI_BUF_SIZE + 1);
        cp_value(cp, c"dvi_limit".as_ptr(), &raw mut dvi_limit);
        cp_value(cp, c"g".as_ptr(), &raw mut g);
        cp_value(cp, c"lq".as_ptr(), &raw mut lq);
        cp_value(cp, c"lr".as_ptr(), &raw mut lr);
        cp_value(cp, c"dvi_ptr".as_ptr(), &raw mut dvi_ptr);
        cp_value(cp, c"dvi_offset".as_ptr(), &raw mut dvi_offset);
        cp_value(cp, c"dvi_gone".as_ptr(), &raw mut dvi_gone);
        cp_value(cp, c"down_ptr".as_ptr(), &raw mut down_ptr);
        cp_value(cp, c"right_ptr".as_ptr(), &raw mut right_ptr);
        cp_value(cp, c"dvi_h".as_ptr(), &raw mut dvi_h);
        cp_value(cp, c"dvi_v".as_ptr(), &raw mut dvi_v);
        cp_value(cp, c"dvi_f".as_ptr(), &raw mut dvi_f);
        cp_value(cp, c"cur_s".as_ptr(), &raw mut cur_s);
        if (*cp).restoring
            && (dvi_ptr < 0
                || dvi_ptr > DVI_BUF_SIZE as i32
                || dvi_limit < 0
                || dvi_limit > DVI_BUF_SIZE as i32)
        {
            (*cp).valid = false;
        }
        if (*cp).restoring
            && output_file_name != 0
            && !tt_xetex_checkpoint_handle_valid(dvi_file, true)
        {
            (*cp).valid = false;
        }
    }
}

#[unsafe(export_name = "initialize_shipout_variables")]
pub unsafe extern "C" fn initialize_shipout_variables() {
    unsafe {
        output_file_name = 0;
        dvi_buf = xmalloc((DVI_BUF_SIZE + 1) * size_of::<eight_bits>()) as *mut eight_bits;
        dvi_limit = DVI_BUF_SIZE as i32;
        dvi_ptr = 0;
        dvi_offset = 0;
        dvi_gone = 0;
        down_ptr = TEX_NULL;
        right_ptr = TEX_NULL;
        cur_s = -1;
    }
}

#[unsafe(export_name = "deinitialize_shipout_variables")]
pub unsafe extern "C" fn deinitialize_shipout_variables() {
    unsafe {
        libc::free(dvi_buf as *mut libc::c_void);
        dvi_buf = ptr::null_mut();
    }
}

#[inline(always)]
unsafe fn dvi_out(c: eight_bits) {
    unsafe {
        *dvi_buf.offset(dvi_ptr as isize) = c;
        dvi_ptr += 1;
        if dvi_ptr == dvi_limit {
            dvi_swap();
        }
    }
}

#[unsafe(export_name = "ship_out")]
pub unsafe extern "C" fn ship_out(p: i32) {
    unsafe {
        synctex_sheet(INTPAR!(mag));

        if job_name == 0 {
            open_log_file();
        }

        if INTPAR!(tracing_output) > 0 {
            print_nl_cstr(c"".as_ptr());
            print_ln();
            print_cstr(c"Completed box being shipped out".as_ptr());
        }

        if term_offset > max_print_line - 9 {
            print_ln();
        } else if term_offset > 0 || file_offset > 0 {
            print_char(b' ' as i32);
        }

        print_char(b'[' as i32);
        let mut j: u8 = 9;
        while j > 0 && COUNT_REG!(j) == 0 {
            j -= 1;
        }

        for k in 0..=j {
            print_int(COUNT_REG!(k));
            if k < j {
                print_char(b'.' as i32);
            }
        }

        ttstub_output_flush(rust_stdout);

        if INTPAR!(tracing_output) > 0 {
            print_char(b']' as i32);
            begin_diagnostic();
            show_box(p);
            end_diagnostic(true);
        }

        'done: {
            if BOX_height!(p) > MAX_HALFWORD
                || BOX_depth!(p) > MAX_HALFWORD
                || BOX_height!(p) + BOX_depth!(p) + DIMENPAR!(v_offset) > MAX_HALFWORD
                || BOX_width!(p) + DIMENPAR!(h_offset) > MAX_HALFWORD
            {
                error_here_with_diagnostic(c"Huge page cannot be shipped out".as_ptr());
                capture_to_diagnostic(ptr::null_mut());
                help_ptr = 2;
                help_line[1] = c"The page just created is more than 18 feet tall or".as_ptr();
                help_line[0] =
                    c"more than 18 feet wide, so I suspect something went wrong.".as_ptr();
                error();

                if INTPAR!(tracing_output) <= 0 {
                    begin_diagnostic();
                    print_nl_cstr(c"The following box has been deleted:".as_ptr());
                    show_box(p);
                    end_diagnostic(true);
                }
                break 'done;
            }

            if BOX_height!(p) + BOX_depth!(p) + DIMENPAR!(v_offset) > max_v {
                max_v = BOX_height!(p) + BOX_depth!(p) + DIMENPAR!(v_offset);
            }

            if BOX_width!(p) + DIMENPAR!(h_offset) > max_h {
                max_h = BOX_width!(p) + DIMENPAR!(h_offset);
            }

            dvi_h = 0;
            dvi_v = 0;
            cur_h = DIMENPAR!(h_offset);
            dvi_f = FONT_BASE;

            cur_h_offset = DIMENPAR!(h_offset) + 4736287;
            cur_v_offset = DIMENPAR!(v_offset) + 4736287;

            if DIMENPAR!(pdf_page_width) != 0 {
                cur_page_width = DIMENPAR!(pdf_page_width);
            } else {
                cur_page_width = BOX_width!(p) + 2 * cur_h_offset;
            }

            if DIMENPAR!(pdf_page_height) != 0 {
                cur_page_height = DIMENPAR!(pdf_page_height);
            } else {
                cur_page_height = BOX_height!(p) + BOX_depth!(p) + 2 * cur_v_offset;
            }

            if output_file_name == 0 {
                if job_name == 0 {
                    open_log_file();
                }
                pack_job_name(output_file_extension);
                dvi_file = ttstub_output_open(name_of_file, 0);
                if dvi_file == INVALID_HANDLE {
                    _tt_abort(c"cannot open output file \"%s\"".as_ptr(), name_of_file);
                }
                output_file_name = make_name_string();
            }

            if total_pages == 0 {
                dvi_out(PRE);

                if semantic_pagination_enabled {
                    dvi_out(SPX_ID_BYTE);
                } else {
                    dvi_out(XDV_ID_BYTE);
                }

                dvi_four(25400000);
                dvi_four(473628672);

                prepare_mag();
                dvi_four(INTPAR!(mag));

                let output_comment = b"tectonic";
                let l = output_comment.len() as u8;
                dvi_out(l);
                for &byte in output_comment {
                    dvi_out(byte);
                }
            }

            let page_loc = dvi_offset + dvi_ptr;

            dvi_out(BOP);

            for k in 0..10 {
                dvi_four(COUNT_REG!(k));
            }

            dvi_four(last_bop);
            last_bop = page_loc;

            let old_setting = selector;
            selector = SELECTOR_NEW_STRING;
            print_cstr(c"pdf:pagesize ".as_ptr());
            if DIMENPAR!(pdf_page_width) <= 0 || DIMENPAR!(pdf_page_height) <= 0 {
                print_cstr(c"default".as_ptr());
            } else {
                print_cstr(c"width".as_ptr());
                print(b' ' as i32);
                print_scaled(DIMENPAR!(pdf_page_width));
                print_cstr(c"pt".as_ptr());
                print(b' ' as i32);
                print_cstr(c"height".as_ptr());
                print(b' ' as i32);
                print_scaled(DIMENPAR!(pdf_page_height));
                print_cstr(c"pt".as_ptr());
            }
            selector = old_setting;

            dvi_out(XXX1);
            dvi_out(cur_length() as u8);

            let mut s = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
            while s < pool_ptr {
                dvi_out(*str_pool.offset(s as isize) as u8);
                s += 1;
            }

            pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);

            cur_v = BOX_height!(p) + DIMENPAR!(v_offset);
            temp_ptr = p;
            if NODE_type!(p) == VLIST_NODE {
                vlist_out();
            } else {
                hlist_out();
            }

            dvi_out(EOP);
            total_pages += 1;
            cur_s = -1;
        }

        if LR_problems > 0 {
            print_ln();
            print_nl_cstr(c"\\endL or \\endR problem (".as_ptr());
            print_int(LR_problems / 10000);
            print_cstr(c" missing, ".as_ptr());
            print_int(LR_problems % 10000);
            print_cstr(c" extra".as_ptr());
            LR_problems = 0;
            print_char(b')' as i32);
            print_ln();
        }

        if LR_ptr != TEX_NULL || cur_dir != LEFT_TO_RIGHT as small_number {
            confusion(c"LR3".as_ptr());
        }

        if INTPAR!(tracing_output) <= 0 {
            print_char(b']' as i32);
        }

        dead_cycles = 0;
        ttstub_output_flush(rust_stdout);
        flush_node_list(p);
        synctex_teehs();
    }
}

unsafe fn hlist_out() {
    unsafe {
        let mut cur_g: scaled_t = 0;
        let mut cur_glue: f64 = 0.0;
        let this_box = temp_ptr;
        let g_order = BOX_glue_order!(this_box) as glue_ord;
        let g_sign = BOX_glue_sign!(this_box) as u8;

        if INTPAR!(xetex_interword_space_shaping) > 1 {
            let mut p = BOX_list_ptr!(this_box);
            let mut prev_p = this_box + 5;

            while p != TEX_NULL {
                if LLIST_link!(p) != TEX_NULL {
                    if p != TEX_NULL
                        && !is_char_node(p)
                        && NODE_type!(p) == WHATSIT_NODE
                        && (NODE_subtype!(p) == NATIVE_WORD_NODE
                            || NODE_subtype!(p) == NATIVE_WORD_NODE_AT)
                        && *font_letter_space.offset(NATIVE_NODE_font!(p) as isize) == 0
                    {
                        let r = p;
                        let mut k = NATIVE_NODE_length!(r) as i32;
                        let mut q = LLIST_link!(p);

                        'check_next: loop {
                            while q != TEX_NULL
                                && !is_char_node(q)
                                && (NODE_type!(q) == PENALTY_NODE
                                    || NODE_type!(q) == INS_NODE
                                    || NODE_type!(q) == MARK_NODE
                                    || NODE_type!(q) == ADJUST_NODE
                                    || (NODE_type!(q) == WHATSIT_NODE && NODE_subtype!(q) <= 4))
                            {
                                q = LLIST_link!(q);
                            }

                            if q != TEX_NULL && !is_char_node(q) {
                                if NODE_type!(q) == GLUE_NODE
                                    && GLUE_SPEC_shrink_order!(q) == NORMAL as u16
                                {
                                    if GLUE_NODE_glue_ptr!(q)
                                        == *font_glue.offset(NATIVE_NODE_font!(r) as isize)
                                    {
                                        q = LLIST_link!(q);

                                        while q != TEX_NULL
                                            && !is_char_node(q)
                                            && (NODE_type!(q) == PENALTY_NODE
                                                || NODE_type!(q) == INS_NODE
                                                || NODE_type!(q) == MARK_NODE
                                                || NODE_type!(q) == ADJUST_NODE
                                                || (NODE_type!(q) == WHATSIT_NODE
                                                    && NODE_subtype!(q) <= 4))
                                        {
                                            q = LLIST_link!(q);
                                        }

                                        if q != TEX_NULL
                                            && !is_char_node(q)
                                            && NODE_type!(q) == WHATSIT_NODE
                                            && (NODE_subtype!(q) == NATIVE_WORD_NODE
                                                || NODE_subtype!(q) == NATIVE_WORD_NODE_AT)
                                            && (mem!(q + 4).b16.s2 == mem!(r + 4).b16.s2)
                                        {
                                            p = q;
                                            k += 1 + NATIVE_NODE_length!(q) as i32;
                                            q = LLIST_link!(q);
                                            continue 'check_next;
                                        }
                                    } else {
                                        q = LLIST_link!(q);
                                    }

                                    if q != TEX_NULL
                                        && !is_char_node(q)
                                        && NODE_type!(q) == KERN_NODE
                                        && NODE_subtype!(q) == SPACE_ADJUSTMENT
                                    {
                                        q = LLIST_link!(q);

                                        while q != TEX_NULL
                                            && !is_char_node(q)
                                            && (NODE_type!(q) == PENALTY_NODE
                                                || NODE_type!(q) == INS_NODE
                                                || NODE_type!(q) == MARK_NODE
                                                || NODE_type!(q) == ADJUST_NODE
                                                || (NODE_type!(q) == WHATSIT_NODE
                                                    && NODE_subtype!(q) <= 4))
                                        {
                                            q = LLIST_link!(q);
                                        }

                                        if q != TEX_NULL
                                            && !is_char_node(q)
                                            && NODE_type!(q) == WHATSIT_NODE
                                            && (NODE_subtype!(q) == NATIVE_WORD_NODE
                                                || NODE_subtype!(q) == NATIVE_WORD_NODE_AT)
                                            && NATIVE_NODE_font!(q) == NATIVE_NODE_font!(r)
                                        {
                                            p = q;
                                            k += 1 + NATIVE_NODE_length!(q) as i32;
                                            q = LLIST_link!(q);
                                            continue 'check_next;
                                        }
                                    }
                                    break 'check_next;
                                }

                                if q != TEX_NULL
                                    && !is_char_node(q)
                                    && NODE_type!(q) == WHATSIT_NODE
                                    && (NODE_subtype!(q) == NATIVE_WORD_NODE
                                        || NODE_subtype!(q) == NATIVE_WORD_NODE_AT)
                                    && NATIVE_NODE_font!(q) == NATIVE_NODE_font!(r)
                                {
                                    p = q;
                                    q = LLIST_link!(q);
                                    continue 'check_next;
                                }
                            }

                            break 'check_next;
                        }

                        if p != r {
                            if pool_ptr + k > pool_size {
                                overflow(c"pool size".as_ptr(), pool_size - init_pool_ptr);
                            }

                            k = 0;
                            let mut q_run = r;

                            loop {
                                if NODE_type!(q_run) == WHATSIT_NODE {
                                    if NODE_subtype!(q_run) == NATIVE_WORD_NODE
                                        || NODE_subtype!(q_run) == NATIVE_WORD_NODE_AT
                                    {
                                        let text_ptr = NATIVE_NODE_text!(q_run);
                                        for j in 0..NATIVE_NODE_length!(q_run) as usize {
                                            *str_pool.offset(pool_ptr as isize) = *text_ptr.add(j);
                                            pool_ptr += 1;
                                        }
                                        k += BOX_width!(q_run);
                                    }
                                } else if NODE_type!(q_run) == GLUE_NODE {
                                    *str_pool.offset(pool_ptr as isize) = b' ' as u16;
                                    pool_ptr += 1;
                                    g = GLUE_NODE_glue_ptr!(q_run);
                                    k += BOX_width!(g);

                                    if g_sign != NORMAL {
                                        if g_sign == STRETCHING {
                                            if GLUE_SPEC_stretch_order!(g) == g_order as u16 {
                                                k += tex_round(
                                                    BOX_glue_set!(this_box)
                                                        * GLUE_SPEC_stretch!(g) as f64,
                                                );
                                            }
                                        } else {
                                            if GLUE_SPEC_shrink_order!(g) == g_order as u16 {
                                                k -= tex_round(
                                                    BOX_glue_set!(this_box)
                                                        * GLUE_SPEC_shrink!(g) as f64,
                                                );
                                            }
                                        }
                                    }
                                } else if NODE_type!(q_run) == KERN_NODE {
                                    k += BOX_width!(q_run);
                                }

                                if q_run == p {
                                    break;
                                } else {
                                    q_run = LLIST_link!(q_run);
                                }
                            }

                            let mut q_new = new_native_word_node(
                                NATIVE_NODE_font!(r) as internal_font_number,
                                cur_length(),
                            );
                            NODE_subtype!(q_new) = NODE_subtype!(r);

                            let text_ptr = NATIVE_NODE_text!(q_new);
                            let start_idx = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
                            for j in 0..cur_length() as usize {
                                *text_ptr.add(j) =
                                    *str_pool.offset((start_idx as usize + j) as isize);
                            }

                            BOX_width!(q_new) = k;
                            set_justified_native_glyphs(q_new);
                            LLIST_link!(prev_p) = q_new;
                            LLIST_link!(q_new) = LLIST_link!(p);
                            LLIST_link!(p) = TEX_NULL;
                            prev_p = r;
                            p = LLIST_link!(r);

                            while p != TEX_NULL {
                                if !is_char_node(p)
                                    && (NODE_type!(p) == PENALTY_NODE
                                        || NODE_type!(p) == INS_NODE
                                        || NODE_type!(p) == MARK_NODE
                                        || NODE_type!(p) == ADJUST_NODE
                                        || (NODE_type!(p) == WHATSIT_NODE && NODE_subtype!(p) <= 4))
                                {
                                    LLIST_link!(prev_p) = LLIST_link!(p);
                                    LLIST_link!(p) = LLIST_link!(q_new);
                                    LLIST_link!(q_new) = p;
                                    q_new = p;
                                }

                                prev_p = p;
                                p = LLIST_link!(p);
                            }

                            flush_node_list(r);
                            pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
                            p = q_new;
                        }
                    }

                    prev_p = p;
                }

                p = LLIST_link!(p);
            }
        }

        let mut p = BOX_list_ptr!(this_box);
        cur_s += 1;
        if cur_s > 0 {
            dvi_out(PUSH);
        }

        if cur_s > max_push {
            max_push = cur_s;
        }

        let save_loc = dvi_offset + dvi_ptr;
        let base_line = cur_v;
        let mut prev_p = this_box + 5;

        temp_ptr = get_avail();
        LLIST_info!(temp_ptr) = BEFORE;
        LLIST_link!(temp_ptr) = LR_ptr;
        LR_ptr = temp_ptr;

        if BOX_lr_mode!(this_box) == DLIST as u16 {
            if cur_dir == RIGHT_TO_LEFT as small_number {
                cur_dir = LEFT_TO_RIGHT as small_number;
                cur_h -= BOX_width!(this_box);
            } else {
                BOX_lr_mode!(this_box) = 0;
            }
        }

        if cur_dir == RIGHT_TO_LEFT as small_number && BOX_lr_mode!(this_box) != REVERSED as u16 {
            let save_h = cur_h;
            temp_ptr = p;
            p = new_kern(0);
            SYNCTEX_tag!(p, MEDIUM_NODE_SIZE) = 0;
            LLIST_link!(prev_p) = p;
            cur_h = 0;
            LLIST_link!(p) = reverse(this_box, TEX_NULL, &raw mut cur_g, &raw mut cur_glue);
            BOX_width!(p) = -cur_h;
            cur_h = save_h;
            BOX_lr_mode!(this_box) = REVERSED as u16;
        }

        let mut left_edge = cur_h;
        synctex_hlist(this_box);

        'nodes: while p != TEX_NULL {
            'reswitch: loop {
                if is_char_node(p) {
                    if cur_h != dvi_h {
                        movement(cur_h - dvi_h, RIGHT1);
                        dvi_h = cur_h;
                    }

                    if cur_v != dvi_v {
                        movement(cur_v - dvi_v, DOWN1);
                        dvi_v = cur_v;
                    }

                    loop {
                        let f = CHAR_NODE_font!(p) as internal_font_number;
                        let mut c = CHAR_NODE_character!(p);
                        if p != LIG_TRICK && !(*font_mapping.offset(f as isize)).is_null() {
                            c = apply_tfm_font_mapping(*font_mapping.offset(f as isize), c as i32)
                                as u16;
                        }

                        if f != dvi_f {
                            if !(*font_used.offset(f as isize)) {
                                dvi_font_def(f);
                                *font_used.offset(f as isize) = true;
                            }

                            if f <= 64 {
                                dvi_out((f + FNT_NUM_0 - 1) as u8);
                            } else if f <= 256 {
                                dvi_out(FNT1);
                                dvi_out((f - 1) as u8);
                            } else {
                                dvi_out(FNT1 + 1);
                                dvi_out(((f - 1) / 256) as u8);
                                dvi_out(((f - 1) % 256) as u8);
                            }

                            dvi_f = f;
                        }

                        if *font_ec.offset(f as isize) >= c {
                            if *font_bc.offset(f as isize) <= c {
                                if FONT_CHARACTER_INFO!(f, c).s3 > 0 {
                                    if c >= 128 {
                                        dvi_out(SET1);
                                    }
                                    dvi_out(c as u8);
                                    cur_h += FONT_CHARACTER_WIDTH!(f, c);
                                }
                            }
                        }

                        prev_p = LLIST_link!(prev_p);
                        p = LLIST_link!(p);
                        if !is_char_node(p) {
                            break;
                        }
                    }

                    synctex_current();
                    dvi_h = cur_h;
                    continue 'nodes;
                } else {
                    match NODE_type!(p) {
                        HLIST_NODE | VLIST_NODE => {
                            if BOX_list_ptr!(p) == TEX_NULL {
                                if NODE_type!(p) == VLIST_NODE {
                                    synctex_void_vlist(p, this_box);
                                } else {
                                    synctex_void_hlist(p, this_box);
                                }
                                cur_h += BOX_width!(p);
                            } else {
                                let save_h = dvi_h;
                                let save_v = dvi_v;
                                cur_v = base_line + BOX_shift_amount!(p);
                                temp_ptr = p;
                                let edge = cur_h + BOX_width!(p);

                                if cur_dir == RIGHT_TO_LEFT as small_number {
                                    cur_h = edge;
                                }

                                if NODE_type!(p) == VLIST_NODE {
                                    vlist_out();
                                } else {
                                    hlist_out();
                                }

                                dvi_h = save_h;
                                dvi_v = save_v;
                                cur_h = edge;
                                cur_v = base_line;
                            }
                        }

                        RULE_NODE => {
                            rule_ht = BOX_height!(p);
                            rule_dp = BOX_depth!(p);
                            rule_wd = BOX_width!(p);

                            if rule_ht == NULL_FLAG {
                                rule_ht = BOX_height!(this_box);
                            }
                            if rule_dp == NULL_FLAG {
                                rule_dp = BOX_depth!(this_box);
                            }
                            rule_ht += rule_dp;

                            if rule_ht > 0 && rule_wd > 0 {
                                if cur_h != dvi_h {
                                    movement(cur_h - dvi_h, RIGHT1);
                                    dvi_h = cur_h;
                                }
                                cur_v = base_line + rule_dp;
                                if cur_v != dvi_v {
                                    movement(cur_v - dvi_v, DOWN1);
                                    dvi_v = cur_v;
                                }
                                dvi_out(SET_RULE);
                                dvi_four(rule_ht);
                                dvi_four(rule_wd);
                                cur_v = base_line;
                                dvi_h += rule_wd;
                            }

                            cur_h += rule_wd;
                            synctex_horizontal_rule_or_glue(p, this_box);
                        }

                        WHATSIT_NODE => match NODE_subtype!(p) {
                            NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT | GLYPH_NODE => {
                                if cur_h != dvi_h {
                                    movement(cur_h - dvi_h, RIGHT1);
                                    dvi_h = cur_h;
                                }

                                if cur_v != dvi_v {
                                    movement(cur_v - dvi_v, DOWN1);
                                    dvi_v = cur_v;
                                }

                                let f = NATIVE_NODE_font!(p) as internal_font_number;

                                if f != dvi_f {
                                    if !(*font_used.offset(f as isize)) {
                                        dvi_font_def(f);
                                        *font_used.offset(f as isize) = true;
                                    }

                                    if f <= 64 {
                                        dvi_out((f + 170) as u8);
                                    } else if f <= 256 {
                                        dvi_out(FNT1);
                                        dvi_out((f - 1) as u8);
                                    } else {
                                        dvi_out(FNT1 + 1);
                                        dvi_out(((f - 1) / 256) as u8);
                                        dvi_out(((f - 1) % 256) as u8);
                                    }

                                    dvi_f = f;
                                }

                                if NODE_subtype!(p) == GLYPH_NODE {
                                    dvi_out(SET_GLYPHS);
                                    dvi_four(BOX_width!(p));
                                    dvi_two(1);
                                    dvi_four(0);
                                    dvi_four(0);
                                    dvi_two(NATIVE_NODE_glyph!(p));
                                    cur_h += BOX_width!(p);
                                } else {
                                    if NODE_subtype!(p) == NATIVE_WORD_NODE_AT {
                                        if NATIVE_NODE_length!(p) > 0
                                            || !NATIVE_NODE_glyph_info_ptr!(p).is_null()
                                        {
                                            dvi_out(SET_TEXT_AND_GLYPHS);
                                            let len = NATIVE_NODE_length!(p) as i32;
                                            dvi_two(len as UTF16_code);

                                            let text_ptr = NATIVE_NODE_text!(p);
                                            for k in 0..len as usize {
                                                dvi_two(*text_ptr.add(k));
                                            }

                                            let xdv_len = make_xdv_glyph_array_data(p);
                                            for k in 0..xdv_len as isize {
                                                dvi_out(*xdv_buffer.offset(k));
                                            }
                                        }
                                    } else {
                                        if !NATIVE_NODE_glyph_info_ptr!(p).is_null() {
                                            dvi_out(SET_GLYPHS);
                                            let xdv_len = make_xdv_glyph_array_data(p);
                                            for k in 0..xdv_len as isize {
                                                dvi_out(*xdv_buffer.offset(k));
                                            }
                                        }
                                    }

                                    cur_h += BOX_width!(p);
                                }

                                dvi_h = cur_h;
                            }

                            PIC_NODE | PDF_NODE => {
                                let save_h = dvi_h;
                                let save_v = dvi_v;
                                cur_v = base_line;
                                let edge = cur_h + BOX_width!(p);
                                pic_out(p);
                                dvi_h = save_h;
                                dvi_v = save_v;
                                cur_h = edge;
                                cur_v = base_line;
                            }

                            PDF_SAVE_POS_NODE => {
                                pdf_last_x_pos = cur_h + 4736286;
                                pdf_last_y_pos = cur_page_height - cur_v - 4736286;
                            }

                            _ => {
                                out_what(p);
                            }
                        },

                        GLUE_NODE => {
                            g = GLUE_NODE_glue_ptr!(p);
                            rule_wd = BOX_width!(g) - cur_g;

                            if g_sign != NORMAL {
                                if g_sign == STRETCHING {
                                    if GLUE_SPEC_stretch_order!(g) == g_order as u16 {
                                        cur_glue += GLUE_SPEC_stretch!(g) as f64;
                                        let mut glue_temp = BOX_glue_set!(this_box) * cur_glue;

                                        if glue_temp > 1000000000.0 {
                                            glue_temp = 1000000000.0;
                                        } else if glue_temp < -1000000000.0 {
                                            glue_temp = -1000000000.0;
                                        }

                                        cur_g = tex_round(glue_temp);
                                    }
                                } else if GLUE_SPEC_shrink_order!(g) == g_order as u16 {
                                    cur_glue -= GLUE_SPEC_shrink!(g) as f64;
                                    let mut glue_temp = BOX_glue_set!(this_box) * cur_glue;

                                    if glue_temp > 1000000000.0 {
                                        glue_temp = 1000000000.0;
                                    } else if glue_temp < -1000000000.0 {
                                        glue_temp = -1000000000.0;
                                    }

                                    cur_g = tex_round(glue_temp);
                                }
                            }

                            rule_wd += cur_g;

                            if (g_sign == STRETCHING
                                && GLUE_SPEC_stretch_order!(g) == g_order as u16)
                                || (g_sign == SHRINKING
                                    && GLUE_SPEC_shrink_order!(g) == g_order as u16)
                            {
                                if GLUE_SPEC_ref_count!(g) == TEX_NULL {
                                    free_node(g, GLUE_SPEC_SIZE);
                                } else {
                                    GLUE_SPEC_ref_count!(g) -= 1;
                                }

                                if NODE_subtype!(p) < A_LEADERS {
                                    NODE_type!(p) = KERN_NODE;
                                    BOX_width!(p) = rule_wd;
                                } else {
                                    g = get_node(GLUE_SPEC_SIZE);
                                    GLUE_SPEC_stretch_order!(g) = (FILLL + 1) as u16;
                                    GLUE_SPEC_shrink_order!(g) = (FILLL + 1) as u16;
                                    BOX_width!(g) = rule_wd;
                                    GLUE_SPEC_stretch!(g) = 0;
                                    GLUE_SPEC_shrink!(g) = 0;
                                    GLUE_NODE_glue_ptr!(p) = g;
                                }
                            }

                            if NODE_subtype!(p) >= A_LEADERS {
                                let leader_box = GLUE_NODE_leader_ptr!(p);

                                if NODE_type!(leader_box) == RULE_NODE {
                                    rule_ht = BOX_height!(leader_box);
                                    rule_dp = BOX_depth!(leader_box);

                                    if rule_ht == NULL_FLAG {
                                        rule_ht = BOX_height!(this_box);
                                    }
                                    if rule_dp == NULL_FLAG {
                                        rule_dp = BOX_depth!(this_box);
                                    }
                                    rule_ht += rule_dp;

                                    if rule_ht > 0 && rule_wd > 0 {
                                        if cur_h != dvi_h {
                                            movement(cur_h - dvi_h, RIGHT1);
                                            dvi_h = cur_h;
                                        }
                                        cur_v = base_line + rule_dp;
                                        if cur_v != dvi_v {
                                            movement(cur_v - dvi_v, DOWN1);
                                            dvi_v = cur_v;
                                        }
                                        dvi_out(SET_RULE);
                                        dvi_four(rule_ht);
                                        dvi_four(rule_wd);
                                        cur_v = base_line;
                                        dvi_h += rule_wd;
                                    }

                                    cur_h += rule_wd;
                                    synctex_horizontal_rule_or_glue(p, this_box);
                                } else {
                                    let leader_wd = BOX_width!(leader_box);

                                    if leader_wd > 0 && rule_wd > 0 {
                                        rule_wd += 10;

                                        if cur_dir == RIGHT_TO_LEFT as small_number {
                                            cur_h -= 10;
                                        }

                                        let edge = cur_h + rule_wd;
                                        let mut lx = 0;

                                        if NODE_subtype!(p) == A_LEADERS {
                                            let save_h = cur_h;
                                            cur_h = left_edge
                                                + leader_wd * ((cur_h - left_edge) / leader_wd);
                                            if cur_h < save_h {
                                                cur_h = cur_h + leader_wd;
                                            }
                                        } else {
                                            lq = rule_wd / leader_wd;
                                            lr = rule_wd % leader_wd;

                                            if NODE_subtype!(p) == C_LEADERS {
                                                cur_h = cur_h + (lr / 2);
                                            } else {
                                                lx = lr / (lq + 1);
                                                cur_h = cur_h + ((lr - (lq - 1) * lx) / 2);
                                            }
                                        }

                                        while cur_h + leader_wd <= edge {
                                            cur_v = base_line + BOX_shift_amount!(leader_box);

                                            if cur_v != dvi_v {
                                                movement(cur_v - dvi_v, DOWN1);
                                                dvi_v = cur_v;
                                            }

                                            let save_v = dvi_v;

                                            if cur_h != dvi_h {
                                                movement(cur_h - dvi_h, RIGHT1);
                                                dvi_h = cur_h;
                                            }

                                            let save_h = dvi_h;

                                            temp_ptr = leader_box;
                                            if cur_dir == RIGHT_TO_LEFT as small_number {
                                                cur_h += leader_wd;
                                            }

                                            let outer_doing_leaders = doing_leaders;
                                            doing_leaders = true;

                                            if NODE_type!(leader_box) == VLIST_NODE {
                                                vlist_out();
                                            } else {
                                                hlist_out();
                                            }

                                            doing_leaders = outer_doing_leaders;
                                            dvi_v = save_v;
                                            dvi_h = save_h;
                                            cur_v = base_line;
                                            cur_h = save_h + leader_wd + lx;
                                        }

                                        if cur_dir == RIGHT_TO_LEFT as small_number {
                                            cur_h = edge;
                                        } else {
                                            cur_h = edge - 10;
                                        }
                                    } else {
                                        cur_h += rule_wd;
                                        synctex_horizontal_rule_or_glue(p, this_box);
                                    }
                                }
                            } else {
                                cur_h += rule_wd;
                                synctex_horizontal_rule_or_glue(p, this_box);
                            }
                        }

                        MARGIN_KERN_NODE => {
                            cur_h += BOX_width!(p);
                        }

                        KERN_NODE => {
                            synctex_kern(p, this_box);
                            cur_h += BOX_width!(p);
                        }

                        MATH_NODE => {
                            synctex_math(p, this_box);

                            if odd(NODE_subtype!(p) as i32) {
                                if LLIST_info!(LR_ptr) == MATH_NODE_end_lr_type!(p) as i32 {
                                    temp_ptr = LR_ptr;
                                    LR_ptr = LLIST_link!(temp_ptr);
                                    LLIST_link!(temp_ptr) = avail;
                                    avail = temp_ptr;
                                } else {
                                    if NODE_subtype!(p) > L_CODE as u16 {
                                        LR_problems += 1;
                                    }
                                }
                            } else {
                                temp_ptr = get_avail();
                                LLIST_info!(temp_ptr) = MATH_NODE_end_lr_type!(p) as i32;
                                LLIST_link!(temp_ptr) = LR_ptr;
                                LR_ptr = temp_ptr;

                                if MATH_NODE_lr_dir!(p) as small_number != cur_dir {
                                    let save_h = cur_h;
                                    temp_ptr = LLIST_link!(p);
                                    rule_wd = BOX_width!(p);
                                    free_node(p, MEDIUM_NODE_SIZE);
                                    cur_dir = 1 - cur_dir;
                                    p = new_edge(cur_dir, rule_wd);
                                    LLIST_link!(prev_p) = p;
                                    cur_h = cur_h - left_edge + rule_wd;
                                    LLIST_link!(p) = reverse(
                                        this_box,
                                        new_edge(1 - cur_dir, 0),
                                        &raw mut cur_g,
                                        &raw mut cur_glue,
                                    );
                                    EDGE_NODE_edge_dist!(p) = cur_h;
                                    cur_dir = 1 - cur_dir;
                                    cur_h = save_h;
                                    continue 'reswitch;
                                }
                            }

                            NODE_type!(p) = KERN_NODE;
                            cur_h += BOX_width!(p);
                        }

                        LIGATURE_NODE => {
                            mem!(LIG_TRICK) = mem!(p + 1);
                            LLIST_link!(LIG_TRICK) = LLIST_link!(p);
                            p = LIG_TRICK;
                            xtx_ligature_present = true;
                            continue 'reswitch;
                        }

                        EDGE_NODE => {
                            cur_h += BOX_width!(p);
                            left_edge = cur_h + EDGE_NODE_edge_dist!(p);
                            cur_dir = NODE_subtype!(p) as small_number;
                        }

                        _ => {}
                    }

                    break 'reswitch;
                }
            }

            prev_p = p;
            p = LLIST_link!(p);
        }

        synctex_tsilh(this_box);

        while LLIST_info!(LR_ptr) != BEFORE {
            if LLIST_info!(LR_ptr) > L_CODE {
                LR_problems += 10000;
            }

            temp_ptr = LR_ptr;
            LR_ptr = LLIST_link!(temp_ptr);
            LLIST_link!(temp_ptr) = avail;
            avail = temp_ptr;
        }

        temp_ptr = LR_ptr;
        LR_ptr = LLIST_link!(temp_ptr);
        LLIST_link!(temp_ptr) = avail;
        avail = temp_ptr;

        if BOX_lr_mode!(this_box) == DLIST as u16 {
            cur_dir = RIGHT_TO_LEFT as small_number;
        }

        prune_movements(save_loc);
        if cur_s > 0 {
            dvi_pop(save_loc);
        }
        cur_s -= 1;
    }
}

unsafe fn vlist_out() {
    unsafe {
        let mut cur_g: scaled_t = 0;
        let mut cur_glue: f64 = 0.0;
        let this_box = temp_ptr;
        let g_order = BOX_glue_order!(this_box) as glue_ord;
        let g_sign = BOX_glue_sign!(this_box) as u8;
        let mut p = BOX_list_ptr!(this_box);
        let upwards = NODE_subtype!(this_box) == 1;

        cur_s += 1;
        if cur_s > 0 {
            dvi_out(PUSH);
        }

        if cur_s > max_push {
            max_push = cur_s;
        }

        let save_loc = dvi_offset + dvi_ptr;
        let left_edge = cur_h;
        synctex_vlist(this_box);

        if upwards {
            cur_v += BOX_depth!(this_box);
        } else {
            cur_v -= BOX_height!(this_box);
        }

        let top_edge = cur_v;

        while p != TEX_NULL {
            if is_char_node(p) {
                confusion(c"vlistout".as_ptr());
            } else {
                match NODE_type!(p) {
                    HLIST_NODE | VLIST_NODE => {
                        if BOX_list_ptr!(p) == TEX_NULL {
                            if upwards {
                                cur_v -= BOX_depth!(p);
                            } else {
                                cur_v += BOX_height!(p);
                            }

                            if NODE_type!(p) == VLIST_NODE {
                                synctex_void_vlist(p, this_box);
                            } else {
                                synctex_void_hlist(p, this_box);
                            }

                            if upwards {
                                cur_v -= BOX_height!(p);
                            } else {
                                cur_v += BOX_depth!(p);
                            }
                        } else {
                            if upwards {
                                cur_v -= BOX_depth!(p);
                            } else {
                                cur_v += BOX_height!(p);
                            }

                            if cur_v != dvi_v {
                                movement(cur_v - dvi_v, DOWN1);
                                dvi_v = cur_v;
                            }

                            let save_h = dvi_h;
                            let save_v = dvi_v;

                            if cur_dir == RIGHT_TO_LEFT as small_number {
                                cur_h = left_edge - BOX_shift_amount!(p);
                            } else {
                                cur_h = left_edge + BOX_shift_amount!(p);
                            }

                            temp_ptr = p;
                            if NODE_type!(p) == VLIST_NODE {
                                vlist_out();
                            } else {
                                hlist_out();
                            }

                            dvi_h = save_h;
                            dvi_v = save_v;

                            if upwards {
                                cur_v = save_v - BOX_height!(p);
                            } else {
                                cur_v = save_v + BOX_depth!(p);
                            }

                            cur_h = left_edge;
                        }
                    }

                    RULE_NODE => {
                        rule_ht = BOX_height!(p);
                        rule_dp = BOX_depth!(p);
                        rule_wd = BOX_width!(p);

                        if rule_wd == NULL_FLAG {
                            rule_wd = BOX_width!(this_box);
                        }
                        rule_ht += rule_dp;

                        if upwards {
                            cur_v -= rule_ht;
                        } else {
                            cur_v += rule_ht;
                        }

                        if rule_ht > 0 && rule_wd > 0 {
                            if cur_dir == RIGHT_TO_LEFT as small_number {
                                cur_h -= rule_wd;
                            }

                            if cur_h != dvi_h {
                                movement(cur_h - dvi_h, RIGHT1);
                                dvi_h = cur_h;
                            }

                            if cur_v != dvi_v {
                                movement(cur_v - dvi_v, DOWN1);
                                dvi_v = cur_v;
                            }

                            dvi_out(PUT_RULE);
                            dvi_four(rule_ht);
                            dvi_four(rule_wd);
                            cur_h = left_edge;
                        }
                    }

                    WHATSIT_NODE => match NODE_subtype!(p) {
                        GLYPH_NODE => {
                            cur_v = cur_v + BOX_height!(p);
                            cur_h = left_edge;

                            if cur_h != dvi_h {
                                movement(cur_h - dvi_h, RIGHT1);
                                dvi_h = cur_h;
                            }

                            if cur_v != dvi_v {
                                movement(cur_v - dvi_v, DOWN1);
                                dvi_v = cur_v;
                            }

                            let f = NATIVE_NODE_font!(p) as internal_font_number;

                            if f != dvi_f {
                                if !(*font_used.offset(f as isize)) {
                                    dvi_font_def(f);
                                    *font_used.offset(f as isize) = true;
                                }

                                if f <= 64 {
                                    dvi_out((f + 170) as u8);
                                } else if f <= 256 {
                                    dvi_out(FNT1);
                                    dvi_out((f - 1) as u8);
                                } else {
                                    dvi_out(FNT1 + 1);
                                    dvi_out(((f - 1) / 256) as u8);
                                    dvi_out(((f - 1) % 256) as u8);
                                }

                                dvi_f = f;
                            }

                            dvi_out(SET_GLYPHS);
                            dvi_four(0);
                            dvi_two(1);
                            dvi_four(0);
                            dvi_four(0);
                            dvi_two(NATIVE_NODE_glyph!(p));

                            cur_v += BOX_depth!(p);
                            cur_h = left_edge;
                        }

                        PIC_NODE | PDF_NODE => {
                            let save_h = dvi_h;
                            let save_v = dvi_v;
                            cur_v = cur_v + BOX_height!(p);
                            pic_out(p);
                            dvi_h = save_h;
                            dvi_v = save_v;
                            cur_v = save_v + BOX_depth!(p);
                            cur_h = left_edge;
                        }

                        PDF_SAVE_POS_NODE => {
                            pdf_last_x_pos = cur_h + 4736286;
                            pdf_last_y_pos = cur_page_height - cur_v - 4736286;
                        }

                        _ => {
                            out_what(p);
                        }
                    },

                    GLUE_NODE => {
                        g = GLUE_NODE_glue_ptr!(p);
                        rule_ht = BOX_width!(g) - cur_g;

                        if g_sign != NORMAL {
                            if g_sign == STRETCHING {
                                if GLUE_SPEC_stretch_order!(g) == g_order as u16 {
                                    cur_glue += GLUE_SPEC_stretch!(g) as f64;
                                    let mut glue_temp = BOX_glue_set!(this_box) * cur_glue;

                                    if glue_temp > 1000000000.0 {
                                        glue_temp = 1000000000.0;
                                    } else if glue_temp < -1000000000.0 {
                                        glue_temp = -1000000000.0;
                                    }

                                    cur_g = tex_round(glue_temp);
                                }
                            } else if GLUE_SPEC_shrink_order!(g) == g_order as u16 {
                                cur_glue -= GLUE_SPEC_shrink!(g) as f64;
                                let mut glue_temp = BOX_glue_set!(this_box) * cur_glue;

                                if glue_temp > 1000000000.0 {
                                    glue_temp = 1000000000.0;
                                } else if glue_temp < -1000000000.0 {
                                    glue_temp = -1000000000.0;
                                }

                                cur_g = tex_round(glue_temp);
                            }
                        }

                        rule_ht += cur_g;

                        if NODE_subtype!(p) >= A_LEADERS {
                            let leader_box = GLUE_NODE_leader_ptr!(p);

                            if NODE_type!(leader_box) == RULE_NODE {
                                rule_wd = BOX_width!(leader_box);
                                rule_dp = 0;

                                if rule_wd == NULL_FLAG {
                                    rule_wd = BOX_width!(this_box);
                                }
                                rule_ht += rule_dp;

                                if upwards {
                                    cur_v -= rule_ht;
                                } else {
                                    cur_v += rule_ht;
                                }

                                if rule_ht > 0 && rule_wd > 0 {
                                    if cur_dir == RIGHT_TO_LEFT as small_number {
                                        cur_h -= rule_wd;
                                    }

                                    if cur_h != dvi_h {
                                        movement(cur_h - dvi_h, RIGHT1);
                                        dvi_h = cur_h;
                                    }

                                    if cur_v != dvi_v {
                                        movement(cur_v - dvi_v, DOWN1);
                                        dvi_v = cur_v;
                                    }

                                    dvi_out(PUT_RULE);
                                    dvi_four(rule_ht);
                                    dvi_four(rule_wd);
                                    cur_h = left_edge;
                                }
                            } else {
                                let leader_ht = BOX_height!(leader_box) + BOX_depth!(leader_box);

                                if leader_ht > 0 && rule_ht > 0 {
                                    rule_ht += 10;
                                    let edge = cur_v + rule_ht;
                                    let mut lx = 0;

                                    if NODE_subtype!(p) == A_LEADERS {
                                        let save_v = cur_v;
                                        cur_v =
                                            top_edge + leader_ht * ((cur_v - top_edge) / leader_ht);
                                        if cur_v < save_v {
                                            cur_v = cur_v + leader_ht;
                                        }
                                    } else {
                                        lq = rule_ht / leader_ht;
                                        lr = rule_ht % leader_ht;

                                        if mem!(p).b16.s0 == C_LEADERS {
                                            cur_v = cur_v + (lr / 2);
                                        } else {
                                            lx = lr / (lq + 1);
                                            cur_v = cur_v + ((lr - (lq - 1) * lx) / 2);
                                        }
                                    }

                                    while cur_v + leader_ht <= edge {
                                        if cur_dir == RIGHT_TO_LEFT as small_number {
                                            cur_h = left_edge - BOX_shift_amount!(leader_box);
                                        } else {
                                            cur_h = left_edge + BOX_shift_amount!(leader_box);
                                        }

                                        if cur_h != dvi_h {
                                            movement(cur_h - dvi_h, RIGHT1);
                                            dvi_h = cur_h;
                                        }

                                        let save_h = dvi_h;
                                        cur_v += BOX_height!(leader_box);

                                        if cur_v != dvi_v {
                                            movement(cur_v - dvi_v, DOWN1);
                                            dvi_v = cur_v;
                                        }

                                        let save_v = dvi_v;
                                        temp_ptr = leader_box;
                                        let outer_doing_leaders = doing_leaders;
                                        doing_leaders = true;

                                        if NODE_type!(leader_box) == VLIST_NODE {
                                            vlist_out();
                                        } else {
                                            hlist_out();
                                        }

                                        doing_leaders = outer_doing_leaders;
                                        dvi_v = save_v;
                                        dvi_h = save_h;
                                        cur_h = left_edge;
                                        cur_v = save_v - BOX_height!(leader_box) + leader_ht + lx;
                                    }

                                    cur_v = edge - 10;
                                } else {
                                    if upwards {
                                        cur_v -= rule_ht;
                                    } else {
                                        cur_v += rule_ht;
                                    }
                                }
                            }
                        } else {
                            if upwards {
                                cur_v -= rule_ht;
                            } else {
                                cur_v += rule_ht;
                            }
                        }
                    }

                    KERN_NODE => {
                        if upwards {
                            cur_v -= BOX_width!(p);
                        } else {
                            cur_v += BOX_width!(p);
                        }
                    }

                    _ => {}
                }
            }

            p = LLIST_link!(p);
        }

        synctex_tsilv(this_box);
        prune_movements(save_loc);

        if cur_s > 0 {
            dvi_pop(save_loc);
        }
        cur_s -= 1;
    }
}

unsafe fn reverse(this_box: i32, t: i32, cur_g: *mut scaled_t, cur_glue: *mut f64) -> i32 {
    unsafe {
        let g_order = BOX_glue_order!(this_box) as glue_ord;
        let g_sign = BOX_glue_sign!(this_box) as u8;
        let mut l = t;
        let mut p = temp_ptr;
        let mut m = MIN_HALFWORD;
        let mut n = MIN_HALFWORD;

        loop {
            while p != TEX_NULL {
                'reswitch: loop {
                    if is_char_node(p) {
                        loop {
                            let f = CHAR_NODE_font!(p) as internal_font_number;
                            let c = CHAR_NODE_character!(p);
                            cur_h +=
                                FONT_CHARACTER_WIDTH!(f, effective_char(true, f, c) as UTF16_code);
                            let q = LLIST_link!(p);
                            LLIST_link!(p) = l;
                            l = p;
                            p = q;
                            if !is_char_node(p) {
                                break;
                            }
                        }
                        break 'reswitch;
                    } else {
                        let q = LLIST_link!(p);
                        let mut advance_cur_h = true;

                        match NODE_type!(p) {
                            HLIST_NODE | VLIST_NODE | RULE_NODE | KERN_NODE => {
                                rule_wd = BOX_width!(p);
                            }

                            WHATSIT_NODE => {
                                if NODE_subtype!(p) == NATIVE_WORD_NODE
                                    || NODE_subtype!(p) == NATIVE_WORD_NODE_AT
                                    || NODE_subtype!(p) == GLYPH_NODE
                                    || NODE_subtype!(p) == PIC_NODE
                                    || NODE_subtype!(p) == PDF_NODE
                                {
                                    rule_wd = BOX_width!(p);
                                } else {
                                    advance_cur_h = false;
                                }
                            }

                            GLUE_NODE => {
                                g = GLUE_NODE_glue_ptr!(p);
                                rule_wd = BOX_width!(g) - *cur_g;

                                if g_sign != NORMAL {
                                    if g_sign == STRETCHING {
                                        if GLUE_SPEC_stretch_order!(g) == g_order as u16 {
                                            *cur_glue = *cur_glue + GLUE_SPEC_stretch!(g) as f64;
                                            let mut glue_temp = BOX_glue_set!(this_box) * *cur_glue;

                                            if glue_temp > 1000000000.0 {
                                                glue_temp = 1000000000.0;
                                            } else if glue_temp < -1000000000.0 {
                                                glue_temp = -1000000000.0;
                                            }

                                            *cur_g = tex_round(glue_temp);
                                        }
                                    } else if GLUE_SPEC_shrink_order!(g) == g_order as u16 {
                                        *cur_glue = *cur_glue - GLUE_SPEC_shrink!(g) as f64;
                                        let mut glue_temp = BOX_glue_set!(this_box) * *cur_glue;

                                        if glue_temp > 1000000000.0 {
                                            glue_temp = 1000000000.0;
                                        } else if glue_temp < -1000000000.0 {
                                            glue_temp = -1000000000.0;
                                        }

                                        *cur_g = tex_round(glue_temp);
                                    }
                                }

                                rule_wd += *cur_g;

                                if (g_sign == STRETCHING && mem!(g).b16.s1 == g_order as u16)
                                    || (g_sign == SHRINKING && mem!(g).b16.s0 == g_order as u16)
                                {
                                    if GLUE_SPEC_ref_count!(g) == TEX_NULL {
                                        free_node(g, GLUE_SPEC_SIZE);
                                    } else {
                                        GLUE_SPEC_ref_count!(g) -= 1;
                                    }

                                    if NODE_subtype!(p) < A_LEADERS {
                                        NODE_type!(p) = KERN_NODE;
                                        BOX_width!(p) = rule_wd;
                                    } else {
                                        g = get_node(GLUE_SPEC_SIZE);
                                        GLUE_SPEC_stretch_order!(g) = (FILLL + 1) as u16;
                                        GLUE_SPEC_shrink_order!(g) = (FILLL + 1) as u16;
                                        BOX_width!(g) = rule_wd;
                                        GLUE_SPEC_stretch!(g) = 0;
                                        GLUE_SPEC_shrink!(g) = 0;
                                        GLUE_NODE_glue_ptr!(p) = g;
                                    }
                                }
                            }

                            LIGATURE_NODE => {
                                flush_node_list(LIGATURE_NODE_lig_ptr!(p));
                                temp_ptr = p;
                                p = get_avail();
                                mem!(p) = mem!(temp_ptr + 1);
                                LLIST_link!(p) = q;
                                free_node(temp_ptr, SMALL_NODE_SIZE);
                                continue 'reswitch;
                            }

                            MATH_NODE => {
                                rule_wd = BOX_width!(p);

                                if odd(BOX_lr_mode!(p) as i32) {
                                    if LLIST_info!(LR_ptr) != MATH_NODE_end_lr_type!(p) as i32 {
                                        NODE_type!(p) = KERN_NODE;
                                        LR_problems += 1;
                                    } else {
                                        temp_ptr = LR_ptr;
                                        LR_ptr = LLIST_link!(temp_ptr);
                                        LLIST_link!(temp_ptr) = avail;
                                        avail = temp_ptr;

                                        if n > MIN_HALFWORD {
                                            n -= 1;
                                            NODE_subtype!(p) -= 1;
                                        } else {
                                            NODE_type!(p) = KERN_NODE;

                                            if m > MIN_HALFWORD {
                                                m -= 1;
                                            } else {
                                                free_node(p, MEDIUM_NODE_SIZE);
                                                LLIST_link!(t) = q;
                                                BOX_width!(t) = rule_wd;
                                                EDGE_NODE_edge_dist!(t) = -cur_h - rule_wd;
                                                return l;
                                            }
                                        }
                                    }
                                } else {
                                    temp_ptr = get_avail();
                                    LLIST_info!(temp_ptr) = MATH_NODE_end_lr_type!(p) as i32;
                                    LLIST_link!(temp_ptr) = LR_ptr;
                                    LR_ptr = temp_ptr;

                                    if n > MIN_HALFWORD
                                        || MATH_NODE_lr_dir!(p) as small_number != cur_dir
                                    {
                                        n += 1;
                                        NODE_subtype!(p) += 1;
                                    } else {
                                        NODE_type!(p) = KERN_NODE;
                                        m += 1;
                                    }
                                }
                            }

                            EDGE_NODE => {
                                confusion(c"LR2".as_ptr());
                            }

                            _ => {
                                advance_cur_h = false;
                            }
                        }

                        if advance_cur_h {
                            cur_h += rule_wd;
                        }

                        LLIST_link!(p) = l;

                        if NODE_type!(p) == KERN_NODE {
                            if rule_wd == 0 || l == TEX_NULL {
                                free_node(p, MEDIUM_NODE_SIZE);
                                p = l;
                            }
                        }

                        l = p;
                        p = q;
                        break 'reswitch;
                    }
                }
            }

            if t == TEX_NULL && m == MIN_HALFWORD && n == MIN_HALFWORD {
                return l;
            }

            p = new_math(0, LLIST_info!(LR_ptr) as small_number);
            LR_problems += 10000;
        }

        l
    }
}

#[unsafe(export_name = "new_edge")]
pub unsafe extern "C" fn new_edge(s: small_number, w: scaled_t) -> i32 {
    unsafe {
        let p = get_node(EDGE_NODE_SIZE);
        NODE_type!(p) = EDGE_NODE;
        NODE_subtype!(p) = s as u16;
        BOX_width!(p) = w;
        EDGE_NODE_edge_dist!(p) = 0;
        p
    }
}

#[unsafe(export_name = "out_what")]
pub unsafe extern "C" fn out_what(p: i32) {
    unsafe {
        match mem!(p).b16.s0 {
            OPEN_NODE | WRITE_NODE | CLOSE_NODE => {
                if doing_leaders {
                    return;
                }

                let j = mem!(p + 1).b32.s0;
                if mem!(p).b16.s0 == WRITE_NODE {
                    write_out(p);
                    return;
                }

                if (j as usize) < 16 && write_open[j as usize] {
                    ttstub_output_close(write_file[j as usize]);
                }

                if mem!(p).b16.s0 == CLOSE_NODE {
                    if (j as usize) < 16 {
                        write_open[j as usize] = false;
                    }
                    return;
                }

                if j >= 16 {
                    return;
                }

                cur_name = mem!(p + 1).b32.s1;
                cur_area = mem!(p + 2).b32.s0;
                cur_ext = mem!(p + 2).b32.s1;
                if length(cur_ext) == 0 {
                    cur_ext = maketexstring(c".tex".as_ptr());
                }

                pack_file_name(cur_name, cur_area, cur_ext);

                write_file[j as usize] = ttstub_output_open(name_of_file, 0);
                if write_file[j as usize] == INVALID_HANDLE {
                    _tt_abort(c"cannot open output file \"%s\"".as_ptr(), name_of_file);
                }

                write_open[j as usize] = true;

                if log_opened {
                    let old_setting = selector;
                    if INTPAR!(tracing_online) <= 0 {
                        selector = SELECTOR_LOG_ONLY;
                    } else {
                        selector = SELECTOR_TERM_AND_LOG;
                    }
                    print_nl_cstr(c"\\openout".as_ptr());
                    print_int(j);
                    print_cstr(c" = `".as_ptr());
                    print_file_name(cur_name, cur_area, cur_ext);
                    print_cstr(c"'.".as_ptr());
                    print_nl_cstr(c"".as_ptr());
                    print_ln();
                    selector = old_setting;
                }
            }

            SPECIAL_NODE => {
                special_out(p);
            }

            LANGUAGE_NODE => {}

            _ => {
                confusion(c"ext4".as_ptr());
            }
        }
    }
}

unsafe fn dvi_native_font_def(f: internal_font_number) {
    unsafe {
        dvi_out(DEFINE_NATIVE_FONT);
        dvi_four(f - 1);
        let font_def_length = make_font_def(f);

        for i in 0..font_def_length as isize {
            dvi_out(*xdv_buffer.offset(i));
        }
    }
}

#[unsafe(export_name = "dvi_font_def")]
pub unsafe extern "C" fn dvi_font_def(f: internal_font_number) {
    unsafe {
        if *font_area.offset(f as isize) as u32 == AAT_FONT_FLAG
            || *font_area.offset(f as isize) as u32 == OTGR_FONT_FLAG
        {
            dvi_native_font_def(f);
        } else {
            if f <= 256 {
                dvi_out(FNT_DEF1);
                dvi_out((f - 1) as u8);
            } else {
                dvi_out(FNT_DEF1 + 1);
                dvi_out(((f - 1) / 256) as u8);
                dvi_out(((f - 1) % 256) as u8);
            }

            let fc = *font_check.offset(f as isize);
            dvi_out(fc.s3 as u8);
            dvi_out(fc.s2 as u8);
            dvi_out(fc.s1 as u8);
            dvi_out(fc.s0 as u8);
            dvi_four(*font_size.offset(f as isize));
            dvi_four(*font_dsize.offset(f as isize));
            dvi_out(length(*font_area.offset(f as isize)) as u8);

            let mut l = 0;
            let font_nom = *font_name.offset(f as isize);
            let mut k = *str_start.offset((font_nom - 65536) as isize);

            while l == 0 && k < *str_start.offset((font_nom + 1 - 65536) as isize) {
                if *str_pool.offset(k as isize) == b':' as u16 {
                    l = k - *str_start.offset((font_nom - 65536) as isize);
                }
                k += 1;
            }

            if l == 0 {
                l = length(font_nom);
            }

            dvi_out(l as u8);

            {
                let area = *font_area.offset(f as isize);
                let mut k_loop = *str_start.offset((area - 65536) as isize);
                let for_end = *str_start.offset((area + 1 - 65536) as isize) - 1;
                while k_loop <= for_end {
                    dvi_out(*str_pool.offset(k_loop as isize) as u8);
                    k_loop += 1;
                }
            }

            {
                let mut k_loop = *str_start.offset((font_nom - 65536) as isize);
                let for_end = *str_start.offset((font_nom - 65536) as isize) + l - 1;
                while k_loop <= for_end {
                    dvi_out(*str_pool.offset(k_loop as isize) as u8);
                    k_loop += 1;
                }
            }
        }
    }
}

unsafe fn movement(mut w: scaled_t, o: eight_bits) {
    unsafe {
        let q_orig = get_node(MOVEMENT_NODE_SIZE);
        let mut q = q_orig;
        mem!(q + 1).b32.s1 = w;
        mem!(q + 2).b32.s1 = dvi_offset + dvi_ptr;

        if o == DOWN1 {
            mem!(q).b32.s1 = down_ptr;
            down_ptr = q;
        } else {
            mem!(q).b32.s1 = right_ptr;
            right_ptr = q;
        }

        let mut p = mem!(q).b32.s1;
        let mut mstate = MOV_NONE_SEEN;
        let mut found_match = false;

        while p != TEX_NULL {
            if mem!(p + 1).b32.s1 == w {
                match mstate as i32 + mem!(p).b32.s0 {
                    val if val == (MOV_NONE_SEEN as i32 + MOV_YZ_OK)
                        || val == (MOV_NONE_SEEN as i32 + MOV_Y_OK)
                        || val == (MOV_Z_SEEN as i32 + MOV_YZ_OK)
                        || val == (MOV_Z_SEEN as i32 + MOV_Y_OK) =>
                    {
                        if mem!(p + 2).b32.s1 < dvi_gone {
                            break;
                        } else {
                            let mut k = mem!(p + 2).b32.s1 - dvi_offset;
                            if k < 0 {
                                k += DVI_BUF_SIZE as i32;
                            }
                            *dvi_buf.offset(k as isize) += 5;
                            mem!(p).b32.s0 = MOV_Y_HERE;
                            found_match = true;
                            break;
                        }
                    }

                    val if val == (MOV_NONE_SEEN as i32 + MOV_Z_OK)
                        || val == (MOV_Y_SEEN as i32 + MOV_YZ_OK)
                        || val == (MOV_Y_SEEN as i32 + MOV_Z_OK) =>
                    {
                        if mem!(p + 2).b32.s1 < dvi_gone {
                            break;
                        } else {
                            let mut k = mem!(p + 2).b32.s1 - dvi_offset;
                            if k < 0 {
                                k += DVI_BUF_SIZE as i32;
                            }
                            *dvi_buf.offset(k as isize) += 10;
                            mem!(p).b32.s0 = MOV_Z_HERE;
                            found_match = true;
                            break;
                        }
                    }

                    val if val == (MOV_NONE_SEEN as i32 + MOV_Y_HERE)
                        || val == (MOV_NONE_SEEN as i32 + MOV_Z_HERE)
                        || val == (MOV_Y_SEEN as i32 + MOV_Z_HERE)
                        || val == (MOV_Z_SEEN as i32 + MOV_Y_HERE) =>
                    {
                        found_match = true;
                        break;
                    }

                    _ => {}
                }
            } else {
                match mstate as i32 + mem!(p).b32.s0 {
                    val if val == (MOV_NONE_SEEN as i32 + MOV_Y_HERE) => {
                        mstate = MOV_Y_SEEN;
                    }
                    val if val == (MOV_NONE_SEEN as i32 + MOV_Z_HERE) => {
                        mstate = MOV_Z_SEEN;
                    }
                    val if val == (MOV_Y_SEEN as i32 + MOV_Z_HERE)
                        || val == (MOV_Z_SEEN as i32 + MOV_Y_HERE) =>
                    {
                        break;
                    }
                    _ => {}
                }
            }

            p = LLIST_link!(p);
        }

        if !found_match {
            mem!(q).b32.s0 = MOV_YZ_OK;

            if w.abs() >= 0x800000 {
                dvi_out(o + 3);
                dvi_four(w);
                return;
            }

            if w.abs() >= 0x8000 {
                dvi_out(o + 2);
                if w < 0 {
                    w += 0x1000000;
                }
                dvi_out((w / 0x10000) as u8);
                w %= 0x10000;
                dvi_out((w / 256) as u8);
                dvi_out((w % 256) as u8);
                return;
            }

            if w.abs() >= 128 {
                dvi_out(o + 1);
                if w < 0 {
                    w += 0x10000;
                }
                dvi_out((w / 256) as u8);
                dvi_out((w % 256) as u8);
                return;
            }

            dvi_out(o);
            if w < 0 {
                w += 256;
            }
            dvi_out((w % 256) as u8);
            return;
        }

        mem!(q).b32.s0 = mem!(p).b32.s0;

        if mem!(q).b32.s0 == MOV_Y_HERE {
            dvi_out(o + 4);

            while mem!(q).b32.s1 != p {
                q = LLIST_link!(q);

                match mem!(q).b32.s0 {
                    MOV_YZ_OK => {
                        mem!(q).b32.s0 = MOV_Z_OK;
                    }
                    MOV_Y_OK => {
                        mem!(q).b32.s0 = MOV_D_FIXED;
                    }
                    _ => {}
                }
            }
        } else {
            dvi_out(o + 9);

            while mem!(q).b32.s1 != p {
                q = LLIST_link!(q);

                match mem!(q).b32.s0 {
                    MOV_YZ_OK => {
                        mem!(q).b32.s0 = MOV_Y_OK;
                    }
                    MOV_Z_OK => {
                        mem!(q).b32.s0 = MOV_D_FIXED;
                    }
                    _ => {}
                }
            }
        }
    }
}

unsafe fn prune_movements(l: i32) {
    unsafe {
        while down_ptr != TEX_NULL {
            if mem!(down_ptr + 2).b32.s1 < l {
                break;
            }

            let p = down_ptr;
            down_ptr = mem!(p).b32.s1;
            free_node(p, MOVEMENT_NODE_SIZE);
        }

        while right_ptr != TEX_NULL {
            if mem!(right_ptr + 2).b32.s1 < l {
                return;
            }
            let p = right_ptr;
            right_ptr = mem!(p).b32.s1;
            free_node(p, MOVEMENT_NODE_SIZE);
        }
    }
}

unsafe fn special_out(p: i32) {
    unsafe {
        if cur_h != dvi_h {
            movement(cur_h - dvi_h, RIGHT1);
            dvi_h = cur_h;
        }
        if cur_v != dvi_v {
            movement(cur_v - dvi_v, DOWN1);
            dvi_v = cur_v;
        }
        doing_special = true;
        let old_setting = selector;
        selector = SELECTOR_NEW_STRING;
        show_token_list(
            mem!(mem!(p + 1).b32.s1).b32.s1,
            TEX_NULL,
            pool_size - pool_ptr,
        );
        selector = old_setting;

        if pool_ptr + 1 > pool_size {
            overflow(c"pool size".as_ptr(), pool_size - init_pool_ptr);
        }

        let len = cur_length();
        if len < 256 {
            dvi_out(XXX1);
            dvi_out(len as u8);
        } else {
            dvi_out(XXX4);
            dvi_four(len);
        }

        let start = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
        for k in start..pool_ptr {
            dvi_out(*str_pool.offset(k as isize) as u8);
        }
        pool_ptr = start;
        doing_special = false;
    }
}

unsafe fn write_out(p: i32) {
    unsafe {
        let mut q = get_avail();
        mem!(q).b32.s0 = RIGHT_BRACE_TOKEN + b'}' as i32;
        let r = get_avail();
        mem!(q).b32.s1 = r;
        mem!(r).b32.s0 = CS_TOKEN_FLAG + END_WRITE;
        begin_token_list(q, INSERTED as u16);
        begin_token_list(mem!(p + 1).b32.s1, WRITE_TEXT as u16);
        q = get_avail();
        mem!(q).b32.s0 = LEFT_BRACE_TOKEN + b'{' as i32;
        begin_token_list(q, INSERTED as u16);

        let old_mode = cur_list.mode;
        cur_list.mode = 0;
        cur_cs = write_loc;
        q = scan_toks(false, true);
        get_token();

        if cur_tok != CS_TOKEN_FLAG + END_WRITE {
            error_here_with_diagnostic(c"Unbalanced write command".as_ptr());
            capture_to_diagnostic(ptr::null_mut());
            help_ptr = 2;
            help_line[1] = c"On this page there's a \\write with fewer real {'s than }'s.".as_ptr();
            help_line[0] = c"I can't handle that very well; good luck.".as_ptr();
            error();

            loop {
                get_token();
                if cur_tok == CS_TOKEN_FLAG + END_WRITE {
                    break;
                }
            }
        }

        cur_list.mode = old_mode;
        end_token_list();
        let old_setting = selector;
        let j = mem!(p + 1).b32.s0;

        if j == 18 {
            selector = SELECTOR_NEW_STRING;
        } else if (j as usize) < 16 && write_open[j as usize] {
            selector = j as i32;
        } else {
            if j == 17 && selector == SELECTOR_TERM_AND_LOG {
                selector = SELECTOR_LOG_ONLY;
            }
            print_nl_cstr(c"".as_ptr());
        }

        token_show(def_ref);
        print_ln();
        flush_list(def_ref);

        if j == 18 {
            if INTPAR!(tracing_online) <= 0 {
                selector = SELECTOR_LOG_ONLY;
            } else {
                selector = SELECTOR_TERM_AND_LOG;
            }

            if !log_opened {
                selector = SELECTOR_TERM_ONLY;
            }

            if !shell_escape_enabled {
                diagnostic_begin_capture_warning_here();

                print_nl_cstr(c"runsystem(".as_ptr());
                let start = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
                for d in 0..cur_length() {
                    print(*str_pool.offset((start + d) as isize) as i32);
                }

                print_cstr(c")...".as_ptr());
                print_cstr(c"disabled".as_ptr());
                print_char(b'.' as i32);
                capture_to_diagnostic(ptr::null_mut());
                print_nl_cstr(c"".as_ptr());
                print_ln();
            } else {
                let start = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
                ttstub_shell_escape(str_pool.offset(start as isize), cur_length() as usize);
            }

            pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
        }

        selector = old_setting;
    }
}

unsafe fn pic_out(p: i32) {
    unsafe {
        if cur_h != dvi_h {
            movement(cur_h - dvi_h, RIGHT1);
            dvi_h = cur_h;
        }

        if cur_v != dvi_v {
            movement(cur_v - dvi_v, DOWN1);
            dvi_v = cur_v;
        }

        let old_setting = selector;
        selector = SELECTOR_NEW_STRING;
        print_cstr(c"pdf:image ".as_ptr());
        print_cstr(c"matrix ".as_ptr());
        print_scaled(mem!(p + 5).b32.s0);
        print(b' ' as i32);
        print_scaled(mem!(p + 5).b32.s1);
        print(b' ' as i32);
        print_scaled(mem!(p + 6).b32.s0);
        print(b' ' as i32);
        print_scaled(mem!(p + 6).b32.s1);
        print(b' ' as i32);
        print_scaled(mem!(p + 7).b32.s0);
        print(b' ' as i32);
        print_scaled(mem!(p + 7).b32.s1);
        print(b' ' as i32);
        print_cstr(c"page ".as_ptr());
        print_int(mem!(p + 4).b16.s0 as i32);
        print(b' ' as i32);

        match mem!(p + 8).b16.s1 {
            1 => {
                print_cstr(c"pagebox cropbox ".as_ptr());
            }
            2 => {
                print_cstr(c"pagebox mediabox ".as_ptr());
            }
            3 => {
                print_cstr(c"pagebox bleedbox ".as_ptr());
            }
            5 => {
                print_cstr(c"pagebox artbox ".as_ptr());
            }
            4 => {
                print_cstr(c"pagebox trimbox ".as_ptr());
            }
            _ => {}
        }

        print(b'(' as i32);
        let path_ptr = PIC_NODE_path!(p);
        for i in 0..PIC_NODE_path_len!(p) as usize {
            print_raw_char(*path_ptr.add(i) as UTF16_code, true);
        }
        print(b')' as i32);

        selector = old_setting;
        let len = cur_length();
        if len < 256 {
            dvi_out(XXX1);
            dvi_out(len as u8);
        } else {
            dvi_out(XXX4);
            dvi_four(len);
        }

        let start = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
        for k in start..pool_ptr {
            dvi_out(*str_pool.offset(k as isize) as u8);
        }

        pool_ptr = start;
    }
}

#[unsafe(export_name = "finalize_dvi_file")]
pub unsafe extern "C" fn finalize_dvi_file() {
    unsafe {
        while cur_s > -1 {
            if cur_s > 0 {
                dvi_out(POP);
            } else {
                dvi_out(EOP);
                total_pages += 1;
            }

            cur_s -= 1;
        }

        if total_pages == 0 {
            print_nl_cstr(c"No pages of output.".as_ptr());
            return;
        }

        if cur_s == -2 {
            return;
        }

        dvi_out(POST);
        dvi_four(last_bop);
        last_bop = dvi_offset + dvi_ptr - 5;
        dvi_four(25400000);
        dvi_four(473628672);
        prepare_mag();
        dvi_four(INTPAR!(mag));
        dvi_four(max_v);
        dvi_four(max_h);
        dvi_out((max_push / 256) as u8);
        dvi_out((max_push % 256) as u8);
        dvi_out(((total_pages / 256) % 256) as u8);
        dvi_out((total_pages % 256) as u8);

        while font_ptr > FONT_BASE {
            if *font_used.offset(font_ptr as isize) {
                dvi_font_def(font_ptr);
            }
            font_ptr -= 1;
        }

        dvi_out(POST_POST);
        dvi_four(last_bop);

        if semantic_pagination_enabled {
            dvi_out(SPX_ID_BYTE);
        } else {
            dvi_out(XDV_ID_BYTE);
        }

        let mut k = 4 + (DVI_BUF_SIZE as i32 - dvi_ptr) % 4;

        while k > 0 {
            dvi_out(223);
            k -= 1;
        }

        if dvi_limit == HALF_BUF as i32 {
            write_to_dvi(HALF_BUF as i32, DVI_BUF_SIZE as i32 - 1);
        }

        if dvi_ptr > TEX_INFINITY - dvi_offset {
            cur_s = -2;
            fatal_error(c"dvi length exceeds 0x7FFFFFFF".as_ptr());
        }

        if dvi_ptr > 0 {
            write_to_dvi(0, dvi_ptr - 1);
        }

        let close_res = ttstub_output_close(dvi_file);

        if close_res == 0 {
            print_nl_cstr(c"Output written on ".as_ptr());
            print(output_file_name);
            print_cstr(c" (".as_ptr());
            print_int(total_pages);
            if total_pages != 1 {
                print_cstr(c" pages".as_ptr());
            } else {
                print_cstr(c" page".as_ptr());
            }
            print_cstr(c", ".as_ptr());
            print_int(dvi_offset + dvi_ptr);
            print_cstr(c" bytes).".as_ptr());
        } else {
            print_nl_cstr(c"Error ".as_ptr());
            print_int(close_res);
            print_cstr(c" (".as_ptr());
            print_c_string(libc::strerror(close_res));
            print_cstr(c") generating output;".as_ptr());
            print_nl_cstr(c"file ".as_ptr());
            print(output_file_name);
            print_cstr(c" may not be valid.".as_ptr());
        }
    }
}

unsafe fn write_to_dvi(a: i32, b: i32) {
    unsafe {
        let n = (b - a + 1) as usize;

        if ttstub_output_write(
            dvi_file,
            dvi_buf.offset(a as isize) as *const libc::c_char,
            n,
        ) != n
        {
            _tt_abort(c"failed to write data to XDV file".as_ptr());
        }
    }
}

unsafe fn dvi_swap() {
    unsafe {
        if dvi_ptr > (TEX_INFINITY - dvi_offset) {
            cur_s = -2;
            fatal_error(c"dvi length exceeds 0x7FFFFFFF".as_ptr());
        }

        if dvi_limit == DVI_BUF_SIZE as i32 {
            write_to_dvi(0, HALF_BUF as i32 - 1);
            dvi_limit = HALF_BUF as i32;
            dvi_offset = dvi_offset + DVI_BUF_SIZE as i32;
            dvi_ptr = 0;
        } else {
            write_to_dvi(HALF_BUF as i32, DVI_BUF_SIZE as i32 - 1);
            dvi_limit = DVI_BUF_SIZE as i32;
        }

        dvi_gone = dvi_gone + HALF_BUF as i32;
    }
}

unsafe fn dvi_four(mut x: i32) {
    unsafe {
        if x >= 0 {
            dvi_out((x / 0x1000000) as u8);
        } else {
            x = x + 0x40000000;
            x = x + 0x40000000;
            dvi_out(((x / 0x1000000) + 128) as u8);
        }

        x = x % 0x1000000;
        dvi_out((x / 0x10000) as u8);

        x = x % 0x10000;
        dvi_out((x / 0x100) as u8);
        dvi_out((x % 0x100) as u8);
    }
}

unsafe fn dvi_two(s: UTF16_code) {
    unsafe {
        dvi_out((s / 0x100) as u8);
        dvi_out((s % 0x100) as u8);
    }
}

unsafe fn dvi_pop(l: i32) {
    unsafe {
        if l == dvi_offset + dvi_ptr && dvi_ptr > 0 {
            dvi_ptr -= 1;
        } else {
            dvi_out(POP);
        }
    }
}
