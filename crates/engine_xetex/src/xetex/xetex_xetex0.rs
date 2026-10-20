// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

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

use super::xetex_constants::{
    CALL, CLOSED, EMPTY, END_TEMPLATE, HASH_BASE, INPUT, LEVEL_ONE, MEM_TOP,
    NATIVE_GLYPH_INFO_SIZE, NORMAL, SEMI_SIMPLE_GROUP, TEX_NULL, TOKEN_LIST, VMODE,
};
use super::xetex_xetexd::{
    b16x4, cond_ptr, cur_chr, cur_cmd, cur_cs, cur_group, cur_input, cur_level, cur_list,
    expand_depth_count, hash, help_ptr, hi_mem_min, history, in_open, input_file, lo_mem_max, mem,
    memory_word, nest_ptr, output_active, param_ptr, pseudo_files, read_open, rover,
    scanner_status, shell_escape_enabled,
};

use super::bridge::{
    ttstub_diag_printf, ttstub_input_close, ttstub_issue_warning, ttstub_output_close,
    ttstub_output_flush, ttstub_output_open, ttstub_output_putc,
};
use super::xetex_ext::{
    AAT_FONT_FLAG, LEFT_SIDE, OTGR_FONT_FLAG, PreparedLayouts, RIGHT_SIDE, aat_font_get,
    aat_font_get_1, aat_font_get_2, aat_font_get_named, aat_font_get_named_1, aat_get_font_metrics,
    aat_print_font_name, apply_mapping, apply_tfm_font_mapping, check_for_tfm_font_mapping,
    count_pdf_file_pages, find_native_font, get_cp_code, get_encoding_mode_and_info,
    get_font_char_range, get_glyph_bounds, get_native_char_height_depth,
    get_native_char_sidebearings, getnativechardp, getnativecharht, getnativecharic,
    getnativecharwd, gr_font_get_named, gr_font_get_named_1, gr_print_font_name, linebreak_next,
    linebreak_start, load_tfm_font_mapping, map_char_to_glyph, map_glyph_to_index, ot_font_get,
    ot_font_get_1, ot_font_get_2, ot_font_get_3, ot_get_font_metrics, print_glyph_name,
    print_utf8_str, release_font_engine, usingGraphite, usingOpenType,
};
use super::xetex_io::{
    INVALID_HANDLE, TTBC_FILE_FORMAT_TECTONIC_PRIMARY, TTBC_FILE_FORMAT_TEX, bytesFromUTF8,
    input_line, make_utf16_name, name_of_input_file, offsetsFromUTF8, set_input_file_encoding,
    tt_xetex_open_input, u_close, u_open_in,
};
use super::xetex_linebreak::{native_node_text, set_native_metrics};
use super::xetex_math::{NODE_type, isOpenTypeMathFont, set_native_glyph_metrics};
use super::xetex_texmfmp::{gettexstring, maketexstring};
use super::xetex_xetex_ot_math::get_ot_math_constant;
use super::xetex_xetexd::{HISTORY_WARNING_ISSUED, b32x2};
use super::*;

use super::xetex_checkpoint::*;
use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_synctex::*;
use super::xetex_xetexd::*;
use libc::*;

use crate::{
    BOX_REG, FONT_CHARACTER_INFO, FONT_CHARACTER_WIDTH, FONT_CHARINFO_HEIGHT,
    FONT_CHARINFO_ITALCORR, FONT_CHARINFO_WIDTH,
};
use std::ptr;

// Explicit imports from xetex_format to disambiguate from other glob re-exports
use super::xetex_format::{
    CLOSE_NODE, GLUE_NODE, GLYPH_NODE, HLIST_NODE, INS_NODE, KERN_NODE, LANGUAGE_NODE,
    LIGATURE_NODE, MARK_NODE, MATH_NODE, NATIVE_WORD_NODE, NATIVE_WORD_NODE_AT, OPEN_NODE,
    PDF_NODE, PDF_SAVE_POS_NODE, PENALTY_NODE, PIC_NODE, RULE_NODE, SPECIAL_NODE, VLIST_NODE,
    WHATSIT_NODE, WRITE_NODE,
};
// Explicit imports from xetex_constants to disambiguate from other glob re-exports
use super::xetex_constants::{
    ACCENT, BOTTOM_LEVEL, DELIM_NUM, HMODE, LETTER, MATH_CHOICE_GROUP, MATH_LEFT_GROUP, MATH_SHIFT,
    MATH_SHIFT_GROUP, MMODE, OTHER_CHAR,
};

#[inline]
pub fn IS_LC_HEX(c: i32) -> bool {
    (c >= b'0' as i32 && c <= b'9' as i32) || (c >= b'a' as i32 && c <= b'f' as i32)
}

macro_rules! mem {
    ($p:expr) => {
        *mem.offset(($p) as isize)
    };
}

macro_rules! eqtb {
    ($p:expr) => {
        *eqtb.offset(($p) as isize)
    };
}

macro_rules! LLIST_link {
    ($p:expr) => {
        mem!($p).b32.s1
    };
}

macro_rules! LLIST_info {
    ($p:expr) => {
        mem!($p).b32.s0
    };
}

macro_rules! NODE_type {
    ($p:expr) => {
        mem!($p).b16.s1
    };
}

macro_rules! NODE_subtype {
    ($p:expr) => {
        mem!($p).b16.s0
    };
}

macro_rules! BOX_glue_set {
    ($p:expr) => {
        mem!(($p) + 6).gr
    };
}

macro_rules! NATIVE_NODE_glyph_info_ptr {
    ($p:expr) => {
        mem!(($p) + 5).ptr
    };
}

macro_rules! NATIVE_NODE_glyph_count {
    ($p:expr) => {
        mem!(($p) + 4).b16.s0
    };
}

macro_rules! NATIVE_NODE_size {
    ($p:expr) => {
        mem!(($p) + 4).b16.s3
    };
}

macro_rules! GLUE_SPEC_ref_count {
    ($p:expr) => {
        mem!($p).b32.s1
    };
}

macro_rules! GLUE_SPEC_shrink_order {
    ($p:expr) => {
        mem!($p).b16.s0
    };
}

macro_rules! INSERTION_NODE_split_top_ptr {
    ($p:expr) => {
        mem!(($p) + 4).b32.s1
    };
}

macro_rules! SYNCTEX_tag {
    ($p:expr, $sz:expr) => {
        mem!(($p) + ($sz) - SYNCTEX_FIELD_SIZE).b32.s0
    };
}

macro_rules! SYNCTEX_line {
    ($p:expr, $sz:expr) => {
        mem!(($p) + ($sz) - SYNCTEX_FIELD_SIZE).b32.s1
    };
}

macro_rules! COUNT_REG {
    ($t:expr) => {
        eqtb!(COUNT_BASE + ($t)).b32.s1
    };
}

macro_rules! XEQ_LEVEL {
    ($p:expr) => {
        _xeq_level_array[(($p) - INT_BASE) as usize]
    };
}

macro_rules! INTPAR {
    ($p:expr) => {
        eqtb!(INT_BASE + ($p)).b32.s1
    };
}

macro_rules! DIMENPAR {
    ($p:expr) => {
        eqtb!(DIMEN_BASE + ($p)).b32.s1
    };
}

macro_rules! GLUEPAR {
    ($p:expr) => {
        eqtb!(GLUE_BASE + ($p)).b32.s1
    };
}

#[unsafe(export_name = "int_error")]
pub unsafe extern "C" fn int_error(n: i32) {
    unsafe {
        print_cstr(b" (\0".as_ptr() as *const c_char);
        print_int(n);
        print_char(b')' as i32);
        error();
    }
}

#[unsafe(export_name = "badness")]
pub unsafe extern "C" fn badness(t: scaled_t, s: scaled_t) -> i32 {
    if t == 0 {
        return 0;
    }
    if s <= 0 {
        return INF_BAD;
    }
    let r: i32 = if t <= 7230584 {
        (t * 297) / s
    } else if s >= 1663497 {
        t / (s / 297)
    } else {
        t
    };
    if r > 1290 {
        return INF_BAD;
    }
    (r * r * r + 0x20000) / 0x40000
}

#[unsafe(export_name = "show_token_list")]
pub unsafe extern "C" fn show_token_list(mut p: i32, q: i32, l: i32) {
    unsafe {
        let mut m: i32;
        let mut c: i32;
        let mut match_chr: i32 = b'#' as i32;
        let mut n: UTF16_code = b'0' as UTF16_code;
        tally = 0;

        while p != TEX_NULL && tally < l {
            if p == q {
                first_count = tally;
                trick_count = tally + 1 + error_line - half_error_line;
                if trick_count < error_line {
                    trick_count = error_line;
                }
            }

            if p < hi_mem_min || p > mem_end {
                print_esc_cstr(b"CLOBBERED.\0".as_ptr() as *const c_char);
                return;
            }

            if mem!(p).b32.s0 >= CS_TOKEN_FLAG {
                print_cs(mem!(p).b32.s0 - CS_TOKEN_FLAG);
            } else {
                m = mem!(p).b32.s0 / MAX_CHAR_VAL;
                c = mem!(p).b32.s0 % MAX_CHAR_VAL;

                if mem!(p).b32.s0 < 0 {
                    print_esc_cstr(b"BAD.\0".as_ptr() as *const c_char);
                } else {
                    match (m) as i32 {
                        LEFT_BRACE | RIGHT_BRACE | MATH_SHIFT | TAB_MARK | SUP_MARK | SUB_MARK
                        | SPACER | LETTER | OTHER_CHAR => {
                            print_char(c);
                        }
                        MAC_PARAM => {
                            print_char(c);
                            print_char(c);
                        }
                        OUT_PARAM => {
                            print_char(match_chr);
                            if c <= 9 {
                                print_char(c + 48);
                            } else {
                                print_char(b'!' as i32);
                                return;
                            }
                        }
                        MATCH => {
                            match_chr = c;
                            print_char(c);
                            n += 1;
                            print_char(n as i32);
                            if n > b'9' as UTF16_code {
                                return;
                            }
                        }
                        END_MATCH => {
                            if c == 0 {
                                print_cstr(b"->\0".as_ptr() as *const c_char);
                            }
                        }
                        _ => {
                            print_esc_cstr(b"BAD.\0".as_ptr() as *const c_char);
                        }
                    }
                }
            }
            p = LLIST_link!(p);
        }

        if p != TEX_NULL {
            print_esc_cstr(b"ETC.\0".as_ptr() as *const c_char);
        }
    }
}

#[unsafe(export_name = "runaway")]
pub unsafe extern "C" fn runaway() {
    unsafe {
        let mut p: i32 = TEX_NULL;

        if scanner_status > (SKIPPING) as u8 {
            match (scanner_status) as i32 {
                DEFINING => {
                    print_nl_cstr(b"Runaway definition\0".as_ptr() as *const c_char);
                    p = def_ref;
                }
                MATCHING => {
                    print_nl_cstr(b"Runaway argument\0".as_ptr() as *const c_char);
                    p = TEMP_HEAD;
                }
                ALIGNING => {
                    print_nl_cstr(b"Runaway preamble\0".as_ptr() as *const c_char);
                    p = HOLD_HEAD;
                }
                ABSORBING => {
                    print_nl_cstr(b"Runaway text\0".as_ptr() as *const c_char);
                    p = def_ref;
                }
                _ => {}
            }

            print_char(b'?' as i32);
            print_ln();
            show_token_list(mem!(p).b32.s1, TEX_NULL, error_line - 10);
        }
    }
}

#[unsafe(export_name = "get_avail")]
pub unsafe extern "C" fn get_avail() -> i32 {
    unsafe {
        let mut p: i32 = avail;
        if p != TEX_NULL {
            avail = LLIST_link!(avail);
        } else if mem_end < MEM_TOP {
            mem_end += 1;
            p = mem_end;
        } else {
            hi_mem_min -= 1;
            p = hi_mem_min;
            if is_char_node(lo_mem_max) {
                runaway();
                overflow(
                    b"main memory size\0".as_ptr() as *const c_char,
                    ((MEM_TOP + 1) as usize) as i32,
                );
            }
        }
        mem!(p).b32.s1 = TEX_NULL;
        p
    }
}

#[unsafe(export_name = "flush_list")]
pub unsafe extern "C" fn flush_list(p: i32) {
    unsafe {
        if p != TEX_NULL {
            let mut r = p;
            let mut q;
            loop {
                q = r;
                r = LLIST_link!(r);
                if r == TEX_NULL {
                    break;
                }
            }
            mem!(q).b32.s1 = avail;
            avail = p;
        }
    }
}

#[unsafe(export_name = "get_node")]
pub unsafe extern "C" fn get_node(s: i32) -> i32 {
    unsafe {
        let mut p: i32;
        let mut q: i32;
        let mut r: i32;
        let mut t: i32;

        'restart: loop {
            p = rover;
            loop {
                q = p + mem!(p).b32.s0;
                while mem!(q).b32.s1 == MAX_HALFWORD {
                    t = mem!(q + 1).b32.s1;
                    if q == rover {
                        rover = t;
                    }
                    mem!(t + 1).b32.s0 = mem!(q + 1).b32.s0;
                    mem!(mem!(q + 1).b32.s0 + 1).b32.s1 = t;
                    q += mem!(q).b32.s0;
                }
                r = q - s;
                if r > p + 1 {
                    mem!(p).b32.s0 = r - p;
                    rover = p;
                    break 'restart;
                }
                if r == p {
                    if mem!(p + 1).b32.s1 != p {
                        rover = mem!(p + 1).b32.s1;
                        t = mem!(p + 1).b32.s0;
                        mem!(rover + 1).b32.s0 = t;
                        mem!(t + 1).b32.s1 = rover;
                        break 'restart;
                    }
                }
                mem!(p).b32.s0 = q - p;
                p = mem!(p + 1).b32.s1;
                if p == rover {
                    break;
                }
            }
            if s == 0x40000000 {
                return MAX_HALFWORD;
            }
            if lo_mem_max + 2 < hi_mem_min {
                if lo_mem_max + 2 <= MAX_HALFWORD {
                    if hi_mem_min - lo_mem_max >= 1998 {
                        t = lo_mem_max + 1000;
                    } else {
                        t = lo_mem_max + 1 + (hi_mem_min - lo_mem_max) / 2;
                    }
                    p = mem!(rover + 1).b32.s0;
                    q = lo_mem_max;
                    mem!(p + 1).b32.s1 = q;
                    mem!(rover + 1).b32.s0 = q;
                    if t > MAX_HALFWORD {
                        t = MAX_HALFWORD;
                    }
                    mem!(q + 1).b32.s1 = rover;
                    mem!(q + 1).b32.s0 = p;
                    mem!(q).b32.s1 = MAX_HALFWORD;
                    mem!(q).b32.s0 = t - lo_mem_max;
                    lo_mem_max = t;
                    mem!(lo_mem_max).b32.s1 = TEX_NULL;
                    mem!(lo_mem_max).b32.s0 = TEX_NULL;
                    rover = q;
                    continue 'restart;
                }
            }
            overflow(
                b"main memory size\0".as_ptr() as *const c_char,
                ((MEM_TOP + 1) as usize) as i32,
            );
        }

        mem!(r).b32.s1 = TEX_NULL;
        if s >= MEDIUM_NODE_SIZE {
            mem!(r + s - 1).b32.s0 = cur_input.synctex_tag;
            mem!(r + s - 1).b32.s1 = line;
        }
        r
    }
}

#[unsafe(export_name = "free_node")]
pub unsafe extern "C" fn free_node(p: i32, s: i32) {
    unsafe {
        if tt_checkpoint_enabled {
            tt_checkpoint_unregister_glyph(p);
        }
        let q: i32;
        mem!(p).b32.s0 = s;
        mem!(p).b32.s1 = MAX_HALFWORD;
        q = mem!(rover + 1).b32.s0;
        mem!(p + 1).b32.s0 = q;
        mem!(p + 1).b32.s1 = rover;
        mem!(rover + 1).b32.s0 = p;
        mem!(q + 1).b32.s1 = p;
    }
}

#[unsafe(export_name = "new_null_box")]
pub unsafe extern "C" fn new_null_box() -> i32 {
    unsafe {
        let p = get_node(BOX_NODE_SIZE);
        NODE_type!(p) = (HLIST_NODE) as u16;
        mem!(p).b16.s0 = 0;
        mem!(p + 1).b32.s1 = 0;
        mem!(p + 2).b32.s1 = 0;
        mem!(p + 3).b32.s1 = 0;
        mem!(p + 4).b32.s1 = 0;
        mem!(p + 5).b32.s1 = TEX_NULL;
        mem!(p + 5).b16.s1 = (NORMAL) as u16;
        mem!(p + 5).b16.s0 = (NORMAL) as u16;
        BOX_glue_set!(p) = 0.0;
        p
    }
}

#[unsafe(export_name = "new_rule")]
pub unsafe extern "C" fn new_rule() -> i32 {
    unsafe {
        let p = get_node(RULE_NODE_SIZE);
        NODE_type!(p) = (RULE_NODE) as u16;
        mem!(p).b16.s0 = 0;
        mem!(p + 1).b32.s1 = NULL_FLAG;
        mem!(p + 2).b32.s1 = NULL_FLAG;
        mem!(p + 3).b32.s1 = NULL_FLAG;
        p
    }
}

#[unsafe(export_name = "new_ligature")]
pub unsafe extern "C" fn new_ligature(f: internal_font_number, c: u16, q: i32) -> i32 {
    unsafe {
        let p = get_node(SMALL_NODE_SIZE);
        NODE_type!(p) = (LIGATURE_NODE) as u16;
        mem!(p + 1).b16.s1 = f as u16;
        mem!(p + 1).b16.s0 = c;
        mem!(p + 1).b32.s1 = q;
        mem!(p).b16.s0 = 0;
        p
    }
}

#[unsafe(export_name = "new_lig_item")]
pub unsafe extern "C" fn new_lig_item(c: u16) -> i32 {
    unsafe {
        let p = get_node(SMALL_NODE_SIZE);
        mem!(p).b16.s0 = c;
        mem!(p + 1).b32.s1 = TEX_NULL;
        p
    }
}

#[unsafe(export_name = "new_disc")]
pub unsafe extern "C" fn new_disc() -> i32 {
    unsafe {
        let p = get_node(SMALL_NODE_SIZE);
        NODE_type!(p) = (DISC_NODE) as u16;
        mem!(p).b16.s0 = 0;
        mem!(p + 1).b32.s0 = TEX_NULL;
        mem!(p + 1).b32.s1 = TEX_NULL;
        p
    }
}

#[unsafe(export_name = "copy_native_glyph_info")]
pub unsafe extern "C" fn copy_native_glyph_info(src: i32, dest: i32) {
    unsafe {
        if !NATIVE_NODE_glyph_info_ptr!(src).is_null() {
            let glyph_count = NATIVE_NODE_glyph_count!(src) as i32;
            let bytes = (glyph_count * NATIVE_GLYPH_INFO_SIZE) as usize;
            let ptr = malloc(bytes);
            memcpy(ptr, NATIVE_NODE_glyph_info_ptr!(src), bytes);
            NATIVE_NODE_glyph_info_ptr!(dest) = ptr;
            NATIVE_NODE_glyph_count!(dest) = glyph_count as u16;
            if tt_checkpoint_enabled {
                tt_checkpoint_register_glyph(dest);
            }
        }
    }
}

#[unsafe(export_name = "new_math")]
pub unsafe extern "C" fn new_math(w: scaled_t, s: small_number) -> i32 {
    unsafe {
        let p = get_node(MEDIUM_NODE_SIZE);
        NODE_type!(p) = (MATH_NODE) as u16;
        mem!(p).b16.s0 = s as u16;
        mem!(p + 1).b32.s1 = w;
        p
    }
}

#[unsafe(export_name = "new_spec")]
pub unsafe extern "C" fn new_spec(p: i32) -> i32 {
    unsafe {
        let q = get_node(GLUE_SPEC_SIZE);
        mem!(q) = mem!(p);
        mem!(q).b32.s1 = TEX_NULL;
        mem!(q + 1).b32.s1 = mem!(p + 1).b32.s1;
        mem!(q + 2).b32.s1 = mem!(p + 2).b32.s1;
        mem!(q + 3).b32.s1 = mem!(p + 3).b32.s1;
        q
    }
}

#[unsafe(export_name = "new_param_glue")]
pub unsafe extern "C" fn new_param_glue(n: small_number) -> i32 {
    unsafe {
        let p = get_node(MEDIUM_NODE_SIZE);
        NODE_type!(p) = (GLUE_NODE) as u16;
        mem!(p).b16.s0 = (n + 1) as u16;
        mem!(p + 1).b32.s1 = TEX_NULL;
        let q = eqtb!(GLUE_BASE + n as i32).b32.s1;
        mem!(p + 1).b32.s0 = q;
        GLUE_SPEC_ref_count!(q) += 1;
        p
    }
}

#[unsafe(export_name = "new_glue")]
pub unsafe extern "C" fn new_glue(q: i32) -> i32 {
    unsafe {
        let p = get_node(MEDIUM_NODE_SIZE);
        NODE_type!(p) = (GLUE_NODE) as u16;
        GLUE_SPEC_shrink_order!(p) = (NORMAL) as u16;
        mem!(p + 1).b32.s1 = TEX_NULL;
        mem!(p + 1).b32.s0 = q;
        GLUE_SPEC_ref_count!(q) += 1;
        p
    }
}

#[unsafe(export_name = "new_skip_param")]
pub unsafe extern "C" fn new_skip_param(n: small_number) -> i32 {
    unsafe {
        temp_ptr = new_spec(eqtb!(GLUE_BASE + n as i32).b32.s1);
        let p = new_glue(temp_ptr);
        mem!(temp_ptr).b32.s1 = TEX_NULL;
        mem!(p).b16.s0 = (n + 1) as u16;
        p
    }
}

#[unsafe(export_name = "new_kern")]
pub unsafe extern "C" fn new_kern(w: scaled_t) -> i32 {
    unsafe {
        let p = get_node(MEDIUM_NODE_SIZE);
        NODE_type!(p) = (KERN_NODE) as u16;
        mem!(p).b16.s0 = (NORMAL) as u16;
        mem!(p + 1).b32.s1 = w;
        p
    }
}

#[unsafe(export_name = "new_penalty")]
pub unsafe extern "C" fn new_penalty(m: i32) -> i32 {
    unsafe {
        let p = get_node(MEDIUM_NODE_SIZE);
        NODE_type!(p) = (PENALTY_NODE) as u16;
        mem!(p).b16.s0 = 0;
        mem!(p + 1).b32.s1 = m;
        p
    }
}

#[unsafe(export_name = "prev_rightmost")]
pub unsafe extern "C" fn prev_rightmost(s: i32, e: i32) -> i32 {
    unsafe {
        let mut p = s;
        if p == TEX_NULL {
            return TEX_NULL;
        }
        while mem!(p).b32.s1 != e {
            p = LLIST_link!(p);
            if p == TEX_NULL {
                return TEX_NULL;
            }
        }
        p
    }
}

#[unsafe(export_name = "get_microinterval")]
pub unsafe extern "C" fn get_microinterval() -> i32 {
    unsafe {
        let mut s: i32 = 0;
        let mut m: i32 = 0;
        get_seconds_and_micros(&mut s, &mut m);
        if (s - epochseconds) > 0x7FFF {
            -1
        } else if microseconds > m {
            ((s - 1 - epochseconds) * 65536)
                + ((((m + 1000000 - microseconds) as f64 / 100.0) * 65536.0) / 10000.0) as i32
        } else {
            ((s - epochseconds) * 65536)
                + ((((m - microseconds) as f64 / 100.0) * 65536.0) / 10000.0) as i32
        }
    }
}

#[unsafe(export_name = "short_display")]
pub unsafe extern "C" fn short_display(mut p: i32) {
    unsafe {
        let mut n: i32;

        while p > 0 {
            if is_char_node(p) {
                if p <= mem_end {
                    if mem!(p).b16.s1 != (font_in_short_display) as u16 {
                        if mem!(p).b16.s1 > font_max as u16 {
                            print_char(b'*' as i32);
                        } else {
                            print_esc(
                                (*hash.offset((FONT_ID_BASE + mem!(p).b16.s1 as i32) as isize)).s1,
                            );
                        }
                        print_char(b' ' as i32);
                        font_in_short_display = (mem!(p).b16.s1) as i32;
                    }
                    print(mem!(p).b16.s0 as i32);
                }
            } else {
                match (mem!(p).b16.s1) as i32 {
                    HLIST_NODE | VLIST_NODE | INS_NODE | MARK_NODE | ADJUST_NODE | UNSET_NODE => {
                        print_cstr(b"[]\0".as_ptr() as *const c_char);
                    }
                    WHATSIT_NODE => match (mem!(p).b16.s0) as i32 {
                        NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT => {
                            if mem!(p + 4).b16.s2 != (font_in_short_display) as u16 {
                                print_esc(
                                    (*hash.offset(
                                        (FONT_ID_BASE + mem!(p + 4).b16.s2 as i32) as isize,
                                    ))
                                    .s1,
                                );
                                print_char(b' ' as i32);
                                font_in_short_display = (mem!(p + 4).b16.s2) as i32;
                            }
                            print_native_word(p);
                        }
                        _ => {
                            print_cstr(b"[]\0".as_ptr() as *const c_char);
                        }
                    },
                    RULE_NODE => {
                        print_char(b'|' as i32);
                    }
                    GLUE_NODE => {
                        if mem!(p + 1).b32.s0 != 0 {
                            print_char(b' ' as i32);
                        }
                    }
                    MATH_NODE => {
                        if mem!(p).b16.s0 >= (L_CODE) as u16 {
                            print_cstr(b"[]\0".as_ptr() as *const c_char);
                        } else {
                            print_char(b'$' as i32);
                        }
                    }
                    LIGATURE_NODE => {
                        short_display(mem!(p + 1).b32.s1);
                    }
                    DISC_NODE => {
                        short_display(mem!(p + 1).b32.s0);
                        short_display(mem!(p + 1).b32.s1);
                        n = mem!(p).b16.s0 as i32;
                        while n > 0 {
                            if mem!(p).b32.s1 != TEX_NULL {
                                p = LLIST_link!(p);
                            }
                            n -= 1;
                        }
                    }
                    _ => {}
                }
            }
            p = LLIST_link!(p);
        }
    }
}

#[unsafe(export_name = "print_font_and_char")]
pub unsafe extern "C" fn print_font_and_char(p: i32) {
    unsafe {
        if p > mem_end {
            print_esc_cstr(b"CLOBBERED.\0".as_ptr() as *const c_char);
        } else {
            if mem!(p).b16.s1 > font_max as u16 {
                print_char(b'*' as i32);
            } else {
                print_esc((*hash.offset((FONT_ID_BASE + mem!(p).b16.s1 as i32) as isize)).s1);
            }
            print_char(b' ' as i32);
            print(mem!(p).b16.s0 as i32);
        }
    }
}

#[unsafe(export_name = "print_mark")]
pub unsafe extern "C" fn print_mark(p: i32) {
    unsafe {
        print_char(b'{' as i32);
        if p < hi_mem_min || p > mem_end {
            print_esc_cstr(b"CLOBBERED.\0".as_ptr() as *const c_char);
        } else {
            show_token_list(mem!(p).b32.s1, TEX_NULL, max_print_line - 10);
        }
        print_char(b'}' as i32);
    }
}

#[unsafe(export_name = "print_rule_dimen")]
pub unsafe extern "C" fn print_rule_dimen(d: scaled_t) {
    unsafe {
        if d == NULL_FLAG {
            print_char(b'*' as i32);
        } else {
            print_scaled(d);
        }
    }
}

#[unsafe(export_name = "print_glue")]
pub unsafe extern "C" fn print_glue(d: scaled_t, mut order: i32, s: *const c_char) {
    unsafe {
        print_scaled(d);
        if order < NORMAL as i32 || order > FILLL as i32 {
            print_cstr(b"foul\0".as_ptr() as *const c_char);
        } else if order > NORMAL as i32 {
            print_cstr(b"fil\0".as_ptr() as *const c_char);
            while order > FIL as i32 {
                print_char(b'l' as i32);
                order -= 1;
            }
        } else if !s.is_null() {
            print_cstr(s);
        }
    }
}

#[unsafe(export_name = "print_spec")]
pub unsafe extern "C" fn print_spec(p: i32, s: *const c_char) {
    unsafe {
        if p < 0 || p >= lo_mem_max {
            print_char(b'*' as i32);
        } else {
            print_scaled(mem!(p + 1).b32.s1);
            if !s.is_null() {
                print_cstr(s);
            }
            if mem!(p + 2).b32.s1 != 0 {
                print_cstr(b" plus \0".as_ptr() as *const c_char);
                print_glue(mem!(p + 2).b32.s1, mem!(p).b16.s1 as i32, s);
            }
            if mem!(p + 3).b32.s1 != 0 {
                print_cstr(b" minus \0".as_ptr() as *const c_char);
                print_glue(mem!(p + 3).b32.s1, mem!(p).b16.s0 as i32, s);
            }
        }
    }
}

#[unsafe(export_name = "print_fam_and_char")]
pub unsafe extern "C" fn print_fam_and_char(p: i32) {
    unsafe {
        print_esc_cstr(b"fam\0".as_ptr() as *const c_char);
        print_int(((mem!(p).b16.s1 % 256) % 256) as i32);
        print_char(b' ' as i32);
        let c = mem!(p).b16.s0 as i32 + ((mem!(p).b16.s1 as i32 / 256) * 65536);
        if c < 65536 {
            print(c);
        } else {
            print_char(c);
        }
    }
}

#[unsafe(export_name = "print_delimiter")]
pub unsafe extern "C" fn print_delimiter(p: i32) {
    unsafe {
        let mut a: i32 = (mem!(p).b16.s3 % 256) as i32 * 256
            + (mem!(p).b16.s2 as i32 + (mem!(p).b16.s3 as i32 / 256) * 65536);
        a = a * 4096
            + (mem!(p).b16.s1 % 256) as i32 * 256
            + (mem!(p).b16.s0 as i32 + (mem!(p).b16.s1 as i32 / 256) * 65536);
        if a < 0 {
            print_int(a);
        } else {
            print_hex(a);
        }
    }
}

#[unsafe(export_name = "print_subsidiary_data")]
pub unsafe extern "C" fn print_subsidiary_data(p: i32, c: UTF16_code) {
    unsafe {
        if cur_length() >= depth_threshold {
            if mem!(p).b32.s1 != EMPTY {
                print_cstr(b" []\0".as_ptr() as *const c_char);
            }
        } else {
            *str_pool.offset(pool_ptr as isize) = c;
            pool_ptr += 1;
            temp_ptr = p;

            match (mem!(p).b32.s1) as i32 {
                MATH_CHAR => {
                    print_ln();
                    print_current_string();
                    print_fam_and_char(p);
                }
                SUB_BOX => {
                    show_info();
                }
                SUB_MLIST => {
                    if mem!(p).b32.s0 == TEX_NULL {
                        print_ln();
                        print_current_string();
                        print_cstr(b"{}\0".as_ptr() as *const c_char);
                    } else {
                        show_info();
                    }
                }
                _ => {}
            }

            pool_ptr -= 1;
        }
    }
}

#[unsafe(export_name = "print_style")]
pub unsafe extern "C" fn print_style(c: i32) {
    unsafe {
        match (c / 2) as i32 {
            x if x == DISPLAY_STYLE as i32 / 2 => {
                print_esc_cstr(b"displaystyle\0".as_ptr() as *const c_char)
            }
            x if x == TEXT_STYLE as i32 / 2 => {
                print_esc_cstr(b"textstyle\0".as_ptr() as *const c_char)
            }
            x if x == SCRIPT_STYLE as i32 / 2 => {
                print_esc_cstr(b"scriptstyle\0".as_ptr() as *const c_char)
            }
            x if x == SCRIPT_SCRIPT_STYLE as i32 / 2 => {
                print_esc_cstr(b"scriptscriptstyle\0".as_ptr() as *const c_char)
            }
            _ => print_cstr(b"Unknown style!\0".as_ptr() as *const c_char),
        }
    }
}

#[unsafe(export_name = "print_skip_param")]
pub unsafe extern "C" fn print_skip_param(n: i32) {
    unsafe {
        match (n) as i32 {
            GLUE_PAR__line_skip => print_esc_cstr(b"lineskip\0".as_ptr() as *const c_char),
            GLUE_PAR__baseline_skip => print_esc_cstr(b"baselineskip\0".as_ptr() as *const c_char),
            GLUE_PAR__par_skip => print_esc_cstr(b"parskip\0".as_ptr() as *const c_char),
            GLUE_PAR__above_display_skip => {
                print_esc_cstr(b"abovedisplayskip\0".as_ptr() as *const c_char)
            }
            GLUE_PAR__below_display_skip => {
                print_esc_cstr(b"belowdisplayskip\0".as_ptr() as *const c_char)
            }
            GLUE_PAR__above_display_short_skip => {
                print_esc_cstr(b"abovedisplayshortskip\0".as_ptr() as *const c_char)
            }
            GLUE_PAR__below_display_short_skip => {
                print_esc_cstr(b"belowdisplayshortskip\0".as_ptr() as *const c_char)
            }
            GLUE_PAR__left_skip => print_esc_cstr(b"leftskip\0".as_ptr() as *const c_char),
            GLUE_PAR__right_skip => print_esc_cstr(b"rightskip\0".as_ptr() as *const c_char),
            GLUE_PAR__top_skip => print_esc_cstr(b"topskip\0".as_ptr() as *const c_char),
            GLUE_PAR__split_top_skip => print_esc_cstr(b"splittopskip\0".as_ptr() as *const c_char),
            GLUE_PAR__tab_skip => print_esc_cstr(b"tabskip\0".as_ptr() as *const c_char),
            GLUE_PAR__space_skip => print_esc_cstr(b"spaceskip\0".as_ptr() as *const c_char),
            GLUE_PAR__xspace_skip => print_esc_cstr(b"xspaceskip\0".as_ptr() as *const c_char),
            GLUE_PAR__par_fill_skip => print_esc_cstr(b"parfillskip\0".as_ptr() as *const c_char),
            GLUE_PAR__xetex_linebreak_skip => {
                print_esc_cstr(b"XeTeXlinebreakskip\0".as_ptr() as *const c_char)
            }
            GLUE_PAR__thin_mu_skip => print_esc_cstr(b"thinmuskip\0".as_ptr() as *const c_char),
            GLUE_PAR__med_mu_skip => print_esc_cstr(b"medmuskip\0".as_ptr() as *const c_char),
            GLUE_PAR__thick_mu_skip => print_esc_cstr(b"thickmuskip\0".as_ptr() as *const c_char),
            _ => print_cstr(b"[unknown glue parameter!]\0".as_ptr() as *const c_char),
        }
    }
}

#[unsafe(export_name = "show_node_list")]
pub unsafe extern "C" fn show_node_list(mut p: i32) {
    unsafe {
        let mut n: i32;
        let mut i: i32;
        let mut g: f64;

        if cur_length() > depth_threshold {
            if p > TEX_NULL {
                print_cstr(b" []\0".as_ptr() as *const c_char);
            }
            return;
        }

        n = 0;
        while p > 0 {
            print_ln();
            print_current_string();

            if p > mem_end {
                print_cstr(b"Bad link, display aborted.\0".as_ptr() as *const c_char);
                return;
            }

            n += 1;
            if n > breadth_max {
                print_cstr(b"etc.\0".as_ptr() as *const c_char);
                return;
            }

            if is_char_node(p) {
                print_font_and_char(p);
            } else {
                match (mem!(p).b16.s1) as i32 {
                    HLIST_NODE | VLIST_NODE | UNSET_NODE => {
                        if NODE_type!(p) == (HLIST_NODE) as u16 {
                            print_esc(b'h' as i32);
                        } else if NODE_type!(p) == (VLIST_NODE) as u16 {
                            print_esc(b'v' as i32);
                        } else {
                            print_esc_cstr(b"unset\0".as_ptr() as *const c_char);
                        }

                        print_cstr(b"box(\0".as_ptr() as *const c_char);
                        print_scaled(mem!(p + 3).b32.s1);
                        print_char(b'+' as i32);
                        print_scaled(mem!(p + 2).b32.s1);
                        print_cstr(b")x\0".as_ptr() as *const c_char);
                        print_scaled(mem!(p + 1).b32.s1);

                        if NODE_type!(p) == (UNSET_NODE) as u16 {
                            if mem!(p).b16.s0 != 0 {
                                print_cstr(b" (\0".as_ptr() as *const c_char);
                                print_int((mem!(p).b16.s0 + 1) as i32);
                                print_cstr(b" columns)\0".as_ptr() as *const c_char);
                            }
                            if mem!(p + 6).b32.s1 != 0 {
                                print_cstr(b", stretch \0".as_ptr() as *const c_char);
                                print_glue(
                                    mem!(p + 6).b32.s1,
                                    mem!(p + 5).b16.s0 as i32,
                                    std::ptr::null(),
                                );
                            }
                            if mem!(p + 4).b32.s1 != 0 {
                                print_cstr(b", shrink \0".as_ptr() as *const c_char);
                                print_glue(
                                    mem!(p + 4).b32.s1,
                                    mem!(p + 5).b16.s1 as i32,
                                    std::ptr::null(),
                                );
                            }
                        } else {
                            g = BOX_glue_set!(p);
                            if g != 0.0 && mem!(p + 5).b16.s1 != (NORMAL) as u16 {
                                print_cstr(b", glue set \0".as_ptr() as *const c_char);
                                if mem!(p + 5).b16.s1 == (SHRINKING) as u16 {
                                    print_cstr(b"- \0".as_ptr() as *const c_char);
                                }
                                if g.abs() > 20000.0 {
                                    if g > 0.0 {
                                        print_char(b'>' as i32);
                                    } else {
                                        print_cstr(b"< -\0".as_ptr() as *const c_char);
                                    }
                                    print_glue(
                                        20000 * 65536,
                                        mem!(p + 5).b16.s0 as i32,
                                        std::ptr::null(),
                                    );
                                } else {
                                    print_glue(
                                        tex_round(65536.0 * g),
                                        mem!(p + 5).b16.s0 as i32,
                                        std::ptr::null(),
                                    );
                                }
                            }

                            if mem!(p + 4).b32.s1 != 0 {
                                print_cstr(b", shifted \0".as_ptr() as *const c_char);
                                print_scaled(mem!(p + 4).b32.s1);
                            }

                            if NODE_type!(p) == (HLIST_NODE) as u16
                                && mem!(p).b16.s0 == (DLIST) as u16
                            {
                                print_cstr(b", display\0".as_ptr() as *const c_char);
                            }
                        }

                        *str_pool.offset(pool_ptr as isize) = b'.' as packed_UTF16_code;
                        pool_ptr += 1;
                        show_node_list(mem!(p + 5).b32.s1);
                        pool_ptr -= 1;
                    }

                    RULE_NODE => {
                        print_esc_cstr(b"rule(\0".as_ptr() as *const c_char);
                        print_rule_dimen(mem!(p + 3).b32.s1);
                        print_char(b'+' as i32);
                        print_rule_dimen(mem!(p + 2).b32.s1);
                        print_cstr(b")x\0".as_ptr() as *const c_char);
                        print_rule_dimen(mem!(p + 1).b32.s1);
                    }

                    INS_NODE => {
                        print_esc_cstr(b"insert\0".as_ptr() as *const c_char);
                        print_int(mem!(p).b16.s0 as i32);
                        print_cstr(b", natural size \0".as_ptr() as *const c_char);
                        print_scaled(mem!(p + 3).b32.s1);
                        print_cstr(b"; split(\0".as_ptr() as *const c_char);
                        print_spec(mem!(p + 4).b32.s1, std::ptr::null());
                        print_char(b',' as i32);
                        print_scaled(mem!(p + 2).b32.s1);
                        print_cstr(b"); float cost \0".as_ptr() as *const c_char);
                        print_int(mem!(p + 1).b32.s1);
                        *str_pool.offset(pool_ptr as isize) = b'.' as packed_UTF16_code;
                        pool_ptr += 1;
                        show_node_list(mem!(p + 4).b32.s0);
                        pool_ptr -= 1;
                    }

                    WHATSIT_NODE => match (mem!(p).b16.s0) as i32 {
                        OPEN_NODE => {
                            print_write_whatsit(b"openout\0".as_ptr() as *const c_char, p);
                            print_char(b'=' as i32);
                            print_file_name(
                                mem!(p + 1).b32.s1,
                                mem!(p + 2).b32.s0,
                                mem!(p + 2).b32.s1,
                            );
                        }
                        WRITE_NODE => {
                            print_write_whatsit(b"write\0".as_ptr() as *const c_char, p);
                            print_mark(mem!(p + 1).b32.s1);
                        }
                        CLOSE_NODE => {
                            print_write_whatsit(b"closeout\0".as_ptr() as *const c_char, p);
                        }
                        SPECIAL_NODE => {
                            print_esc_cstr(b"special\0".as_ptr() as *const c_char);
                            print_mark(mem!(p + 1).b32.s1);
                        }
                        LANGUAGE_NODE => {
                            print_esc_cstr(b"setlanguage\0".as_ptr() as *const c_char);
                            print_int(mem!(p + 1).b32.s1);
                            print_cstr(b" (hyphenmin \0".as_ptr() as *const c_char);
                            print_int(mem!(p + 1).b16.s1 as i32);
                            print_char(b',' as i32);
                            print_int(mem!(p + 1).b16.s0 as i32);
                            print_char(b')' as i32);
                        }
                        PDF_SAVE_POS_NODE => {
                            print_esc_cstr(b"pdfsavepos\0".as_ptr() as *const c_char);
                        }
                        NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT => {
                            print_esc(
                                (*hash.offset((FONT_ID_BASE + mem!(p + 4).b16.s2 as i32) as isize))
                                    .s1,
                            );
                            print_char(b' ' as i32);
                            print_native_word(p);
                        }
                        GLYPH_NODE => {
                            print_esc(
                                (*hash.offset((FONT_ID_BASE + mem!(p + 4).b16.s2 as i32) as isize))
                                    .s1,
                            );
                            print_cstr(b" glyph#\0".as_ptr() as *const c_char);
                            print_int(mem!(p + 4).b16.s1 as i32);
                        }
                        PIC_NODE | PDF_NODE => {
                            if mem!(p).b16.s0 == (PIC_NODE) as u16 {
                                print_esc_cstr(b"XeTeXpicfile\0".as_ptr() as *const c_char);
                            } else {
                                print_esc_cstr(b"XeTeXpdffile\0".as_ptr() as *const c_char);
                            }
                            print_cstr(b"( \0".as_ptr() as *const c_char);
                            let path_len = mem!(p + 4).b16.s1 as i32;
                            let path_ptr = (mem.offset((p + PIC_NODE_SIZE) as isize)) as *const u8;
                            for idx in 0..path_len {
                                print_raw_char(*path_ptr.offset(idx as isize) as UTF16_code, true);
                            }
                            print(b'"' as i32);
                        }
                        _ => {
                            print_cstr(b"whatsit?\0".as_ptr() as *const c_char);
                        }
                    },

                    GLUE_NODE => {
                        if mem!(p).b16.s0 >= (A_LEADERS) as u16 {
                            print_esc_cstr(b"\0".as_ptr() as *const c_char);
                            if mem!(p).b16.s0 == (C_LEADERS) as u16 {
                                print_char(b'c' as i32);
                            } else if mem!(p).b16.s0 == (X_LEADERS) as u16 {
                                print_char(b'x' as i32);
                            }
                            print_cstr(b"leaders \0".as_ptr() as *const c_char);
                            print_spec(mem!(p + 1).b32.s0, std::ptr::null());
                            *str_pool.offset(pool_ptr as isize) = b'.' as packed_UTF16_code;
                            pool_ptr += 1;
                            show_node_list(mem!(p + 1).b32.s1);
                            pool_ptr -= 1;
                        } else {
                            print_esc_cstr(b"glue\0".as_ptr() as *const c_char);
                            if GLUE_SPEC_shrink_order!(p) != (NORMAL) as u16 {
                                print_char(b'(' as i32);
                                if mem!(p).b16.s0 < (COND_MATH_GLUE) as u16 {
                                    print_skip_param((mem!(p).b16.s0 - 1) as i32);
                                } else if mem!(p).b16.s0 == (COND_MATH_GLUE) as u16 {
                                    print_esc_cstr(b"nonscript\0".as_ptr() as *const c_char);
                                } else {
                                    print_esc_cstr(b"mskip\0".as_ptr() as *const c_char);
                                }
                                print_char(b')' as i32);
                            }

                            if mem!(p).b16.s0 != (COND_MATH_GLUE) as u16 {
                                print_char(b' ' as i32);
                                if mem!(p).b16.s0 < (COND_MATH_GLUE) as u16 {
                                    print_spec(mem!(p + 1).b32.s0, std::ptr::null());
                                } else {
                                    print_spec(
                                        mem!(p + 1).b32.s0,
                                        b"mu\0".as_ptr() as *const c_char,
                                    );
                                }
                            }
                        }
                    }

                    KERN_NODE => {
                        if mem!(p).b16.s0 != (MU_GLUE) as u16 {
                            print_esc_cstr(b"kern\0".as_ptr() as *const c_char);
                            if mem!(p).b16.s0 != (NORMAL) as u16 {
                                print_char(b' ' as i32);
                            }
                            print_scaled(mem!(p + 1).b32.s1);
                            if NODE_subtype!(p) == (ACC_KERN) as u16 {
                                print_cstr(b" (for accent)\0".as_ptr() as *const c_char);
                            } else if NODE_subtype!(p) == (SPACE_ADJUSTMENT) as u16 {
                                print_cstr(b" (space adjustment)\0".as_ptr() as *const c_char);
                            }
                        } else {
                            print_esc_cstr(b"mkern\0".as_ptr() as *const c_char);
                            print_scaled(mem!(p + 1).b32.s1);
                            print_cstr(b"mu\0".as_ptr() as *const c_char);
                        }
                    }

                    MARGIN_KERN_NODE => {
                        print_esc_cstr(b"kern\0".as_ptr() as *const c_char);
                        print_scaled(mem!(p + 1).b32.s1);
                        if mem!(p).b16.s0 == 0 {
                            print_cstr(b" (left margin)\0".as_ptr() as *const c_char);
                        } else {
                            print_cstr(b" (right margin)\0".as_ptr() as *const c_char);
                        }
                    }

                    MATH_NODE => {
                        if mem!(p).b16.s0 > (AFTER) as u16 {
                            if (mem!(p).b16.s0 & 1) != 0 {
                                print_esc_cstr(b"end\0".as_ptr() as *const c_char);
                            } else {
                                print_esc_cstr(b"begin\0".as_ptr() as *const c_char);
                            }
                            if mem!(p).b16.s0 > (R_CODE) as u16 {
                                print_char(b'R' as i32);
                            } else if mem!(p).b16.s0 > (L_CODE) as u16 {
                                print_char(b'L' as i32);
                            } else {
                                print_char(b'M' as i32);
                            }
                        } else {
                            print_esc_cstr(b"math\0".as_ptr() as *const c_char);
                            if mem!(p).b16.s0 == (BEFORE) as u16 {
                                print_cstr(b"on\0".as_ptr() as *const c_char);
                            } else {
                                print_cstr(b"off\0".as_ptr() as *const c_char);
                            }
                            if mem!(p + 1).b32.s1 != 0 {
                                print_cstr(b", surrounded \0".as_ptr() as *const c_char);
                                print_scaled(mem!(p + 1).b32.s1);
                            }
                        }
                    }

                    LIGATURE_NODE => {
                        print_font_and_char(p + 1);
                        print_cstr(b" (ligature \0".as_ptr() as *const c_char);
                        if mem!(p).b16.s0 > 1 {
                            print_char(b'|' as i32);
                        }
                        font_in_short_display = (mem!(p + 1).b16.s1) as i32;
                        short_display(mem!(p + 1).b32.s1);
                        if (mem!(p).b16.s0 & 1) != 0 {
                            print_char(b'|' as i32);
                        }
                        print_char(b')' as i32);
                    }

                    PENALTY_NODE => {
                        print_esc_cstr(b"penalty \0".as_ptr() as *const c_char);
                        print_int(mem!(p + 1).b32.s1);
                    }

                    DISC_NODE => {
                        print_esc_cstr(b"discretionary\0".as_ptr() as *const c_char);
                        if mem!(p).b16.s0 > 0 {
                            print_cstr(b" replacing \0".as_ptr() as *const c_char);
                            print_int(mem!(p).b16.s0 as i32);
                        }
                        *str_pool.offset(pool_ptr as isize) = b'.' as packed_UTF16_code;
                        pool_ptr += 1;
                        show_node_list(mem!(p + 1).b32.s0);
                        pool_ptr -= 1;
                        *str_pool.offset(pool_ptr as isize) = b'|' as packed_UTF16_code;
                        pool_ptr += 1;
                        show_node_list(mem!(p + 1).b32.s1);
                        pool_ptr -= 1;
                    }

                    MARK_NODE => {
                        print_esc_cstr(b"mark\0".as_ptr() as *const c_char);
                        if mem!(p + 1).b32.s0 != 0 {
                            print_char(b's' as i32);
                            print_int(mem!(p + 1).b32.s0);
                        }
                        print_mark(mem!(p + 1).b32.s1);
                    }

                    ADJUST_NODE => {
                        print_esc_cstr(b"vadjust\0".as_ptr() as *const c_char);
                        if mem!(p).b16.s0 != 0 {
                            print_cstr(b" pre \0".as_ptr() as *const c_char);
                        }
                        *str_pool.offset(pool_ptr as isize) = b'.' as packed_UTF16_code;
                        pool_ptr += 1;
                        show_node_list(mem!(p + 1).b32.s1);
                        pool_ptr -= 1;
                    }

                    STYLE_NODE => {
                        print_style(mem!(p).b16.s0 as i32);
                    }

                    CHOICE_NODE => {
                        print_esc_cstr(b"mathchoice\0".as_ptr() as *const c_char);
                        *str_pool.offset(pool_ptr as isize) = b'D' as packed_UTF16_code;
                        pool_ptr += 1;
                        show_node_list(mem!(p + 1).b32.s0);
                        pool_ptr -= 1;
                        *str_pool.offset(pool_ptr as isize) = b'T' as packed_UTF16_code;
                        pool_ptr += 1;
                        show_node_list(mem!(p + 1).b32.s1);
                        pool_ptr -= 1;
                        *str_pool.offset(pool_ptr as isize) = b'S' as packed_UTF16_code;
                        pool_ptr += 1;
                        show_node_list(mem!(p + 2).b32.s0);
                        pool_ptr -= 1;
                        *str_pool.offset(pool_ptr as isize) = b's' as packed_UTF16_code;
                        pool_ptr += 1;
                        show_node_list(mem!(p + 2).b32.s1);
                        pool_ptr -= 1;
                    }

                    ORD_NOAD | OP_NOAD | BIN_NOAD | REL_NOAD | OPEN_NOAD | CLOSE_NOAD
                    | PUNCT_NOAD | INNER_NOAD | RADICAL_NOAD | OVER_NOAD | UNDER_NOAD
                    | VCENTER_NOAD | ACCENT_NOAD | LEFT_NOAD | RIGHT_NOAD => {
                        match (mem!(p).b16.s1) as i32 {
                            ORD_NOAD => print_esc_cstr(b"mathord\0".as_ptr() as *const c_char),
                            OP_NOAD => print_esc_cstr(b"mathop\0".as_ptr() as *const c_char),
                            BIN_NOAD => print_esc_cstr(b"mathbin\0".as_ptr() as *const c_char),
                            REL_NOAD => print_esc_cstr(b"mathrel\0".as_ptr() as *const c_char),
                            OPEN_NOAD => print_esc_cstr(b"mathopen\0".as_ptr() as *const c_char),
                            CLOSE_NOAD => print_esc_cstr(b"mathclose\0".as_ptr() as *const c_char),
                            PUNCT_NOAD => print_esc_cstr(b"mathpunct\0".as_ptr() as *const c_char),
                            INNER_NOAD => print_esc_cstr(b"mathinner\0".as_ptr() as *const c_char),
                            OVER_NOAD => print_esc_cstr(b"overline\0".as_ptr() as *const c_char),
                            UNDER_NOAD => print_esc_cstr(b"underline\0".as_ptr() as *const c_char),
                            VCENTER_NOAD => print_esc_cstr(b"vcenter\0".as_ptr() as *const c_char),
                            RADICAL_NOAD => {
                                print_esc_cstr(b"radical\0".as_ptr() as *const c_char);
                                print_delimiter(p + 4);
                            }
                            ACCENT_NOAD => {
                                print_esc_cstr(b"accent\0".as_ptr() as *const c_char);
                                print_fam_and_char(p + 4);
                            }
                            LEFT_NOAD => {
                                print_esc_cstr(b"left\0".as_ptr() as *const c_char);
                                print_delimiter(p + 1);
                            }
                            RIGHT_NOAD => {
                                if mem!(p).b16.s0 == (NORMAL) as u16 {
                                    print_esc_cstr(b"right\0".as_ptr() as *const c_char);
                                } else {
                                    print_esc_cstr(b"middle\0".as_ptr() as *const c_char);
                                }
                                print_delimiter(p + 1);
                            }
                            _ => {}
                        }

                        if mem!(p).b16.s1 < (LEFT_NOAD) as u16 {
                            if mem!(p).b16.s0 != (NORMAL) as u16 {
                                if mem!(p).b16.s0 == (LIMITS) as u16 {
                                    print_esc_cstr(b"limits\0".as_ptr() as *const c_char);
                                } else {
                                    print_esc_cstr(b"nolimits\0".as_ptr() as *const c_char);
                                }
                            }
                            print_subsidiary_data(p + 1, b'.' as UTF16_code);
                        }
                        print_subsidiary_data(p + 2, b'^' as UTF16_code);
                        print_subsidiary_data(p + 3, b'_' as UTF16_code);
                    }

                    FRACTION_NOAD => {
                        print_esc_cstr(b"fraction, thickness \0".as_ptr() as *const c_char);
                        if mem!(p + 1).b32.s1 == DEFAULT_CODE {
                            print_cstr(b"= default\0".as_ptr() as *const c_char);
                        } else {
                            print_scaled(mem!(p + 1).b32.s1);
                        }

                        if mem!(p + 4).b16.s3 % 256 != 0
                            || (mem!(p + 4).b16.s2 as i32
                                + (mem!(p + 4).b16.s3 as i32 / 256) * 65536)
                                != 0
                            || mem!(p + 4).b16.s1 % 256 != 0
                            || (mem!(p + 4).b16.s0 as i32
                                + (mem!(p + 4).b16.s1 as i32 / 256) * 65536)
                                != 0
                        {
                            print_cstr(b", left-delimiter \0".as_ptr() as *const c_char);
                            print_delimiter(p + 4);
                        }

                        if mem!(p + 5).b16.s3 % 256 != 0
                            || (mem!(p + 5).b16.s2 as i32
                                + (mem!(p + 5).b16.s3 as i32 / 256) * 65536)
                                != 0
                            || mem!(p + 5).b16.s1 % 256 != 0
                            || (mem!(p + 5).b16.s0 as i32
                                + (mem!(p + 5).b16.s1 as i32 / 256) * 65536)
                                != 0
                        {
                            print_cstr(b", right-delimiter \0".as_ptr() as *const c_char);
                            print_delimiter(p + 5);
                        }

                        print_subsidiary_data(p + 2, b'\\' as UTF16_code);
                        print_subsidiary_data(p + 3, b'/' as UTF16_code);
                    }

                    _ => {
                        print_cstr(b"Unknown node type!\0".as_ptr() as *const c_char);
                    }
                }
            }
            p = LLIST_link!(p);
        }
    }
}

#[unsafe(export_name = "show_box")]
pub unsafe extern "C" fn show_box(p: i32) {
    unsafe {
        depth_threshold = INTPAR!(INT_PAR__show_box_depth);
        breadth_max = INTPAR!(INT_PAR__show_box_breadth);
        if breadth_max <= 0 {
            breadth_max = 5;
        }
        if pool_ptr + depth_threshold >= pool_size {
            depth_threshold = pool_size - pool_ptr - 1;
        }
        show_node_list(p);
        print_ln();
    }
}

#[unsafe(export_name = "short_display_n")]
pub unsafe extern "C" fn short_display_n(p: i32, m: i32) {
    unsafe {
        breadth_max = m;
        depth_threshold = pool_size - pool_ptr - 1;
        show_node_list(p);
    }
}

#[unsafe(export_name = "delete_token_ref")]
pub unsafe extern "C" fn delete_token_ref(p: i32) {
    unsafe {
        if mem!(p).b32.s0 == TEX_NULL {
            flush_list(p);
        } else {
            mem!(p).b32.s0 -= 1;
        }
    }
}

#[unsafe(export_name = "delete_glue_ref")]
pub unsafe extern "C" fn delete_glue_ref(p: i32) {
    unsafe {
        if mem!(p).b32.s1 == TEX_NULL {
            free_node(p, GLUE_SPEC_SIZE);
        } else {
            mem!(p).b32.s1 -= 1;
        }
    }
}

#[unsafe(export_name = "flush_node_list")]
pub unsafe extern "C" fn flush_node_list(mut p: i32) {
    unsafe {
        let mut q: i32;

        while p != TEX_NULL {
            q = mem!(p).b32.s1;

            if is_char_node(p) {
                mem!(p).b32.s1 = avail;
                avail = p;
            } else {
                'node_done: {
                    match (mem!(p).b16.s1) as i32 {
                        HLIST_NODE | VLIST_NODE | UNSET_NODE => {
                            flush_node_list(mem!(p + 5).b32.s1);
                            free_node(p, BOX_NODE_SIZE);
                            break 'node_done;
                        }
                        RULE_NODE => {
                            free_node(p, RULE_NODE_SIZE);
                            break 'node_done;
                        }
                        INS_NODE => {
                            flush_node_list(mem!(p + 4).b32.s0);
                            delete_glue_ref(INSERTION_NODE_split_top_ptr!(p));
                            free_node(p, INS_NODE_SIZE);
                            break 'node_done;
                        }
                        WHATSIT_NODE => {
                            match (mem!(p).b16.s0) as i32 {
                                OPEN_NODE => {
                                    free_node(p, OPEN_NODE_SIZE);
                                }
                                WRITE_NODE | SPECIAL_NODE => {
                                    delete_token_ref(mem!(p + 1).b32.s1);
                                    free_node(p, WRITE_NODE_SIZE);
                                    break 'node_done;
                                }
                                CLOSE_NODE | LANGUAGE_NODE => {
                                    free_node(p, SMALL_NODE_SIZE);
                                }
                                NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT => {
                                    if !NATIVE_NODE_glyph_info_ptr!(p).is_null() {
                                        free(NATIVE_NODE_glyph_info_ptr!(p));
                                        NATIVE_NODE_glyph_info_ptr!(p) = std::ptr::null_mut();
                                        NATIVE_NODE_glyph_count!(p) = 0;
                                    }
                                    free_node(p, NATIVE_NODE_size!(p) as i32);
                                }
                                GLYPH_NODE => {
                                    free_node(p, GLYPH_NODE_SIZE);
                                }
                                PIC_NODE | PDF_NODE => {
                                    free_node(
                                        p,
                                        PIC_NODE_SIZE
                                            + (mem!(p + 4).b16.s1 as i32
                                                + std::mem::size_of::<memory_word>() as i32
                                                - 1)
                                                / std::mem::size_of::<memory_word>() as i32,
                                    );
                                }
                                PDF_SAVE_POS_NODE => {
                                    free_node(p, SMALL_NODE_SIZE);
                                }
                                _ => {
                                    confusion(b"ext3\0".as_ptr() as *const c_char);
                                }
                            }
                            break 'node_done;
                        }
                        GLUE_NODE => {
                            if mem!(mem!(p + 1).b32.s0).b32.s1 == TEX_NULL {
                                free_node(mem!(p + 1).b32.s0, GLUE_SPEC_SIZE);
                            } else {
                                mem!(mem!(p + 1).b32.s0).b32.s1 -= 1;
                            }
                            if mem!(p + 1).b32.s1 != TEX_NULL {
                                flush_node_list(mem!(p + 1).b32.s1);
                            }
                            free_node(p, MEDIUM_NODE_SIZE);
                            break 'node_done;
                        }
                        KERN_NODE | MATH_NODE | PENALTY_NODE => {
                            free_node(p, MEDIUM_NODE_SIZE);
                            break 'node_done;
                        }
                        MARGIN_KERN_NODE => {
                            free_node(p, MARGIN_KERN_NODE_SIZE);
                            break 'node_done;
                        }
                        LIGATURE_NODE => {
                            flush_node_list(mem!(p + 1).b32.s1);
                        }
                        MARK_NODE => {
                            delete_token_ref(mem!(p + 1).b32.s1);
                        }
                        DISC_NODE => {
                            flush_node_list(mem!(p + 1).b32.s0);
                            flush_node_list(mem!(p + 1).b32.s1);
                        }
                        ADJUST_NODE => {
                            flush_node_list(mem!(p + 1).b32.s1);
                        }
                        STYLE_NODE => {
                            free_node(p, STYLE_NODE_SIZE);
                            break 'node_done;
                        }
                        CHOICE_NODE => {
                            flush_node_list(mem!(p + 1).b32.s0);
                            flush_node_list(mem!(p + 1).b32.s1);
                            flush_node_list(mem!(p + 2).b32.s0);
                            flush_node_list(mem!(p + 2).b32.s1);
                            free_node(p, STYLE_NODE_SIZE);
                            break 'node_done;
                        }
                        ORD_NOAD | OP_NOAD | BIN_NOAD | REL_NOAD | OPEN_NOAD | CLOSE_NOAD
                        | PUNCT_NOAD | INNER_NOAD | RADICAL_NOAD | OVER_NOAD | UNDER_NOAD
                        | VCENTER_NOAD | ACCENT_NOAD => {
                            if mem!(p + 1).b32.s1 >= SUB_BOX {
                                flush_node_list(mem!(p + 1).b32.s0);
                            }
                            if mem!(p + 2).b32.s1 >= SUB_BOX {
                                flush_node_list(mem!(p + 2).b32.s0);
                            }
                            if mem!(p + 3).b32.s1 >= SUB_BOX {
                                flush_node_list(mem!(p + 3).b32.s0);
                            }
                            if mem!(p).b16.s1 == (RADICAL_NOAD) as u16 {
                                free_node(p, RADICAL_NOAD_SIZE);
                            } else if mem!(p).b16.s1 == (ACCENT_NOAD) as u16 {
                                free_node(p, ACCENT_NOAD_SIZE);
                            } else {
                                free_node(p, NOAD_SIZE);
                            }
                            break 'node_done;
                        }
                        LEFT_NOAD | RIGHT_NOAD => {
                            free_node(p, NOAD_SIZE);
                            break 'node_done;
                        }
                        FRACTION_NOAD => {
                            flush_node_list(mem!(p + 2).b32.s0);
                            flush_node_list(mem!(p + 3).b32.s0);
                            free_node(p, FRACTION_NOAD_SIZE);
                            break 'node_done;
                        }
                        _ => {
                            confusion(b"flushing\0".as_ptr() as *const c_char);
                        }
                    }
                    free_node(p, SMALL_NODE_SIZE);
                }
            }
            p = q;
        }
    }
}

#[unsafe(export_name = "copy_node_list")]
pub unsafe extern "C" fn copy_node_list(mut p: i32) -> i32 {
    unsafe {
        let h = get_avail();
        let mut q = h;

        while p != TEX_NULL {
            let mut words: usize = 1;
            let r: i32;

            if is_char_node(p) {
                r = get_avail();
            } else {
                match (mem!(p).b16.s1) as i32 {
                    HLIST_NODE | VLIST_NODE | UNSET_NODE => {
                        r = get_node(BOX_NODE_SIZE);
                        SYNCTEX_tag!(r, BOX_NODE_SIZE) = SYNCTEX_tag!(p, BOX_NODE_SIZE);
                        SYNCTEX_line!(r, BOX_NODE_SIZE) = SYNCTEX_line!(p, BOX_NODE_SIZE);
                        mem!(r + 6) = mem!(p + 6);
                        mem!(r + 5) = mem!(p + 5);
                        mem!(r + 5).b32.s1 = copy_node_list(mem!(p + 5).b32.s1);
                        words = 5;
                    }
                    RULE_NODE => {
                        r = get_node(RULE_NODE_SIZE);
                        words = (RULE_NODE_SIZE - 1) as usize;
                    }
                    INS_NODE => {
                        r = get_node(INS_NODE_SIZE);
                        mem!(r + 4) = mem!(p + 4);
                        GLUE_SPEC_ref_count!(mem!(p + 4).b32.s1) += 1;
                        mem!(r + 4).b32.s0 = copy_node_list(mem!(p + 4).b32.s0);
                        words = (INS_NODE_SIZE - 1) as usize;
                    }
                    WHATSIT_NODE => match (mem!(p).b16.s0) as i32 {
                        OPEN_NODE => {
                            r = get_node(OPEN_NODE_SIZE);
                            words = OPEN_NODE_SIZE as usize;
                        }
                        WRITE_NODE | SPECIAL_NODE => {
                            r = get_node(WRITE_NODE_SIZE);
                            mem!(mem!(p + 1).b32.s1).b32.s0 += 1;
                            words = WRITE_NODE_SIZE as usize;
                        }
                        CLOSE_NODE | LANGUAGE_NODE => {
                            r = get_node(SMALL_NODE_SIZE);
                            words = SMALL_NODE_SIZE as usize;
                        }
                        NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT => {
                            words = NATIVE_NODE_size!(p) as usize;
                            r = get_node(words as i32);
                            let mut w_idx = words;
                            while w_idx > 0 {
                                w_idx -= 1;
                                mem!(r + w_idx as i32) = mem!(p + w_idx as i32);
                            }
                            NATIVE_NODE_glyph_info_ptr!(r) = std::ptr::null_mut();
                            NATIVE_NODE_glyph_count!(r) = 0;
                            copy_native_glyph_info(p, r);
                            words = 0;
                        }
                        GLYPH_NODE => {
                            r = get_node(GLYPH_NODE_SIZE);
                            words = GLYPH_NODE_SIZE as usize;
                        }
                        PIC_NODE | PDF_NODE => {
                            words = (PIC_NODE_SIZE
                                + (mem!(p + 4).b16.s1 as i32
                                    + std::mem::size_of::<memory_word>() as i32
                                    - 1)
                                    / std::mem::size_of::<memory_word>() as i32)
                                as usize;
                            r = get_node(words as i32);
                        }
                        PDF_SAVE_POS_NODE => {
                            r = get_node(SMALL_NODE_SIZE);
                            words = SMALL_NODE_SIZE as usize;
                        }
                        _ => {
                            confusion(b"ext2\0".as_ptr() as *const c_char);
                            r = 0;
                        }
                    },
                    GLUE_NODE => {
                        r = get_node(MEDIUM_NODE_SIZE);
                        GLUE_SPEC_ref_count!(mem!(p + 1).b32.s0) += 1;
                        mem!(r + 2).b32.s0 = mem!(p + 2).b32.s0;
                        mem!(r + 2).b32.s1 = mem!(p + 2).b32.s1;
                        mem!(r + 1).b32.s0 = mem!(p + 1).b32.s0;
                        mem!(r + 1).b32.s1 = copy_node_list(mem!(p + 1).b32.s1);
                        words = 1;
                    }
                    KERN_NODE | MATH_NODE | PENALTY_NODE => {
                        r = get_node(MEDIUM_NODE_SIZE);
                        words = MEDIUM_NODE_SIZE as usize;
                    }
                    MARGIN_KERN_NODE => {
                        r = get_node(MARGIN_KERN_NODE_SIZE);
                        words = MARGIN_KERN_NODE_SIZE as usize;
                    }
                    LIGATURE_NODE => {
                        r = get_node(SMALL_NODE_SIZE);
                        mem!(r + 1) = mem!(p + 1);
                        mem!(r + 1).b32.s1 = copy_node_list(mem!(p + 1).b32.s1);
                        words = 1;
                    }
                    DISC_NODE => {
                        r = get_node(SMALL_NODE_SIZE);
                        mem!(r + 1).b32.s0 = copy_node_list(mem!(p + 1).b32.s0);
                        mem!(r + 1).b32.s1 = copy_node_list(mem!(p + 1).b32.s1);
                        words = 1;
                    }
                    MARK_NODE => {
                        r = get_node(SMALL_NODE_SIZE);
                        mem!(mem!(p + 1).b32.s1).b32.s0 += 1;
                        words = SMALL_NODE_SIZE as usize;
                    }
                    ADJUST_NODE => {
                        r = get_node(SMALL_NODE_SIZE);
                        mem!(r + 1).b32.s1 = copy_node_list(mem!(p + 1).b32.s1);
                        words = 1;
                    }
                    _ => {
                        confusion(b"copying\0".as_ptr() as *const c_char);
                        r = 0;
                    }
                }
            }

            while words > 0 {
                words -= 1;
                mem!(r + words as i32) = mem!(p + words as i32);
            }

            mem!(q).b32.s1 = r;
            q = r;
            p = LLIST_link!(p);
        }

        mem!(q).b32.s1 = TEX_NULL;
        q = mem!(h).b32.s1;
        mem!(h).b32.s1 = avail;
        avail = h;
        q
    }
}

#[unsafe(export_name = "print_mode")]
pub unsafe extern "C" fn print_mode(m: i32) {
    unsafe {
        if m > 0 {
            match (m / (MAX_COMMAND as i32 + 1)) as i32 {
                0 => print_cstr(b"vertical mode\0".as_ptr() as *const c_char),
                1 => print_cstr(b"horizontal mode\0".as_ptr() as *const c_char),
                2 => print_cstr(b"display math mode\0".as_ptr() as *const c_char),
                _ => {}
            }
        } else if m == 0 {
            print_cstr(b"no mode\0".as_ptr() as *const c_char);
        } else {
            match ((-m) / (MAX_COMMAND as i32 + 1)) as i32 {
                0 => print_cstr(b"internal vertical mode\0".as_ptr() as *const c_char),
                1 => print_cstr(b"restricted horizontal mode\0".as_ptr() as *const c_char),
                2 => print_cstr(b"math mode\0".as_ptr() as *const c_char),
                _ => {}
            }
        }
    }
}

#[unsafe(export_name = "print_in_mode")]
pub unsafe extern "C" fn print_in_mode(m: i32) {
    unsafe {
        if m > 0 {
            match (m / (MAX_COMMAND as i32 + 1)) as i32 {
                0 => print_cstr(b"' in vertical mode\0".as_ptr() as *const c_char),
                1 => print_cstr(b"' in horizontal mode\0".as_ptr() as *const c_char),
                2 => print_cstr(b"' in display math mode\0".as_ptr() as *const c_char),
                _ => {}
            }
        } else if m == 0 {
            print_cstr(b"' in no mode\0".as_ptr() as *const c_char);
        } else {
            match ((-m) / (MAX_COMMAND as i32 + 1)) as i32 {
                0 => print_cstr(b"' in internal vertical mode\0".as_ptr() as *const c_char),
                1 => print_cstr(b"' in restricted horizontal mode\0".as_ptr() as *const c_char),
                2 => print_cstr(b"' in math mode\0".as_ptr() as *const c_char),
                _ => {}
            }
        }
    }
}

#[unsafe(export_name = "push_nest")]
pub unsafe extern "C" fn push_nest() {
    unsafe {
        if nest_ptr > max_nest_stack {
            max_nest_stack = nest_ptr;
            if nest_ptr == nest_size {
                overflow(
                    b"semantic nest size\0".as_ptr() as *const c_char,
                    (nest_size as usize) as i32,
                );
            }
        }
        *nest.offset(nest_ptr as isize) = cur_list;
        nest_ptr += 1;
        cur_list.head = get_avail();
        cur_list.tail = cur_list.head;
        cur_list.prev_graf = 0;
        cur_list.mode_line = line;
        cur_list.eTeX_aux = TEX_NULL;
    }
}

#[unsafe(export_name = "pop_nest")]
pub unsafe extern "C" fn pop_nest() {
    unsafe {
        mem!(cur_list.head).b32.s1 = avail;
        avail = cur_list.head;
        nest_ptr -= 1;
        cur_list = *nest.offset(nest_ptr as isize);
    }
}

#[unsafe(export_name = "show_activities")]
pub unsafe extern "C" fn show_activities() {
    unsafe {
        let mut p: i32;
        let mut m: i16;
        let mut a: memory_word;
        let mut q: i32;
        let mut r: i32;
        let mut t: i32;

        *nest.offset(nest_ptr as isize) = cur_list;
        print_nl_cstr(b"\0".as_ptr() as *const c_char);
        print_ln();
        p = nest_ptr;
        while p >= 0 {
            m = (*nest.offset(p as isize)).mode;
            a = (*nest.offset(p as isize)).aux;
            print_nl_cstr(b"### \0".as_ptr() as *const c_char);
            print_mode(m as i32);
            print_cstr(b" entered at line \0".as_ptr() as *const c_char);
            print_int((*nest.offset(p as isize)).mode_line.abs());
            if m == HMODE as i16 {
                if (*nest.offset(p as isize)).prev_graf != 0x830000 {
                    print_cstr(b" (language\0".as_ptr() as *const c_char);
                    print_int((*nest.offset(p as isize)).prev_graf % 65536);
                    print_cstr(b":hyphenmin\0".as_ptr() as *const c_char);
                    print_int((*nest.offset(p as isize)).prev_graf / 0x400000);
                    print_char(b',' as i32);
                    print_int(((*nest.offset(p as isize)).prev_graf / 65536) % 64);
                    print_char(b')' as i32);
                }
            }
            if (*nest.offset(p as isize)).mode_line < 0 {
                print_cstr(b" (\\output routine)\0".as_ptr() as *const c_char);
            }
            if p == 0 {
                if PAGE_HEAD != page_tail {
                    print_nl_cstr(b"### current page:\0".as_ptr() as *const c_char);
                    if output_active {
                        print_cstr(b" (held over for next output)\0".as_ptr() as *const c_char);
                    }
                    show_box(mem!(PAGE_HEAD).b32.s1);
                    if page_contents > (EMPTY) as u8 {
                        print_nl_cstr(b"total height \0".as_ptr() as *const c_char);
                        print_totals();
                        print_nl_cstr(b" goal height \0".as_ptr() as *const c_char);
                        print_scaled(*(&raw mut page_so_far).cast::<scaled_t>().offset(0));
                        r = mem!(PAGE_INS_HEAD).b32.s1;
                        while r != PAGE_INS_HEAD {
                            print_ln();
                            print_esc_cstr(b"insert\0".as_ptr() as *const c_char);
                            t = mem!(r).b16.s0 as i32;
                            print_int(t);
                            print_cstr(b" adds \0".as_ptr() as *const c_char);
                            if COUNT_REG!(t) == 1000 {
                                t = mem!(r + 3).b32.s1;
                            } else {
                                t = x_over_n(mem!(r + 3).b32.s1, 1000) * COUNT_REG!(t);
                            }
                            print_scaled(t);
                            if mem!(r).b16.s1 == (SPLIT_UP) as u16 {
                                q = PAGE_HEAD;
                                t = 0;
                                loop {
                                    q = LLIST_link!(q);
                                    if NODE_type!(q) == (INS_NODE) as u16
                                        && mem!(q).b16.s0 == mem!(r).b16.s0
                                    {
                                        t += 1;
                                    }
                                    if q == mem!(r + 1).b32.s0 {
                                        break;
                                    }
                                }
                                print_cstr(b", #\0".as_ptr() as *const c_char);
                                print_int(t);
                                print_cstr(b" might split\0".as_ptr() as *const c_char);
                            }
                            r = LLIST_link!(r);
                        }
                    }
                }
                if mem!(CONTRIB_HEAD).b32.s1 != TEX_NULL {
                    print_nl_cstr(b"### recent contributions:\0".as_ptr() as *const c_char);
                }
            }
            show_box(mem!((*nest.offset(p as isize)).head).b32.s1);
            match ((m.abs() as i32) / (MAX_COMMAND as i32 + 1)) as i32 {
                0 => {
                    print_nl_cstr(b"prevdepth \0".as_ptr() as *const c_char);
                    if a.b32.s1 <= IGNORE_DEPTH {
                        print_cstr(b"ignored\0".as_ptr() as *const c_char);
                    } else {
                        print_scaled(a.b32.s1);
                    }
                    if (*nest.offset(p as isize)).prev_graf != 0 {
                        print_cstr(b", prevgraf \0".as_ptr() as *const c_char);
                        print_int((*nest.offset(p as isize)).prev_graf);
                        if (*nest.offset(p as isize)).prev_graf != 1 {
                            print_cstr(b" lines\0".as_ptr() as *const c_char);
                        } else {
                            print_cstr(b" line\0".as_ptr() as *const c_char);
                        }
                    }
                }
                1 => {
                    print_nl_cstr(b"spacefactor \0".as_ptr() as *const c_char);
                    print_int(a.b32.s0);
                    if m > 0 {
                        if a.b32.s1 > 0 {
                            print_cstr(b", current language \0".as_ptr() as *const c_char);
                            print_int(a.b32.s1);
                        }
                    }
                }
                2 => {
                    if a.b32.s1 != TEX_NULL {
                        print_cstr(b"this will begin denominator of:\0".as_ptr() as *const c_char);
                        show_box(a.b32.s1);
                    }
                }
                _ => {}
            }
            p -= 1;
        }
    }
}

#[unsafe(export_name = "print_param")]
pub unsafe extern "C" fn print_param(n: i32) {
    unsafe {
        match (n) as i32 {
            INT_PAR__pretolerance => print_esc_cstr(b"pretolerance\0".as_ptr() as *const c_char),
            INT_PAR__tolerance => print_esc_cstr(b"tolerance\0".as_ptr() as *const c_char),
            INT_PAR__line_penalty => print_esc_cstr(b"linepenalty\0".as_ptr() as *const c_char),
            INT_PAR__hyphen_penalty => print_esc_cstr(b"hyphenpenalty\0".as_ptr() as *const c_char),
            INT_PAR__ex_hyphen_penalty => {
                print_esc_cstr(b"exhyphenpenalty\0".as_ptr() as *const c_char)
            }
            INT_PAR__club_penalty => print_esc_cstr(b"clubpenalty\0".as_ptr() as *const c_char),
            INT_PAR__widow_penalty => print_esc_cstr(b"widowpenalty\0".as_ptr() as *const c_char),
            INT_PAR__display_widow_penalty => {
                print_esc_cstr(b"displaywidowpenalty\0".as_ptr() as *const c_char)
            }
            INT_PAR__broken_penalty => print_esc_cstr(b"brokenpenalty\0".as_ptr() as *const c_char),
            INT_PAR__bin_op_penalty => print_esc_cstr(b"binoppenalty\0".as_ptr() as *const c_char),
            INT_PAR__rel_penalty => print_esc_cstr(b"relpenalty\0".as_ptr() as *const c_char),
            INT_PAR__pre_display_penalty => {
                print_esc_cstr(b"predisplaypenalty\0".as_ptr() as *const c_char)
            }
            INT_PAR__post_display_penalty => {
                print_esc_cstr(b"postdisplaypenalty\0".as_ptr() as *const c_char)
            }
            INT_PAR__inter_line_penalty => {
                print_esc_cstr(b"interlinepenalty\0".as_ptr() as *const c_char)
            }
            INT_PAR__double_hyphen_demerits => {
                print_esc_cstr(b"doublehyphendemerits\0".as_ptr() as *const c_char)
            }
            INT_PAR__final_hyphen_demerits => {
                print_esc_cstr(b"finalhyphendemerits\0".as_ptr() as *const c_char)
            }
            INT_PAR__adj_demerits => print_esc_cstr(b"adjdemerits\0".as_ptr() as *const c_char),
            INT_PAR__mag => print_esc_cstr(b"mag\0".as_ptr() as *const c_char),
            INT_PAR__delimiter_factor => {
                print_esc_cstr(b"delimiterfactor\0".as_ptr() as *const c_char)
            }
            INT_PAR__looseness => print_esc_cstr(b"looseness\0".as_ptr() as *const c_char),
            INT_PAR__time => print_esc_cstr(b"time\0".as_ptr() as *const c_char),
            INT_PAR__day => print_esc_cstr(b"day\0".as_ptr() as *const c_char),
            INT_PAR__month => print_esc_cstr(b"month\0".as_ptr() as *const c_char),
            INT_PAR__year => print_esc_cstr(b"year\0".as_ptr() as *const c_char),
            INT_PAR__show_box_breadth => {
                print_esc_cstr(b"showboxbreadth\0".as_ptr() as *const c_char)
            }
            INT_PAR__show_box_depth => print_esc_cstr(b"showboxdepth\0".as_ptr() as *const c_char),
            INT_PAR__hbadness => print_esc_cstr(b"hbadness\0".as_ptr() as *const c_char),
            INT_PAR__vbadness => print_esc_cstr(b"vbadness\0".as_ptr() as *const c_char),
            INT_PAR__pausing => print_esc_cstr(b"pausing\0".as_ptr() as *const c_char),
            INT_PAR__tracing_online => print_esc_cstr(b"tracingonline\0".as_ptr() as *const c_char),
            INT_PAR__tracing_macros => print_esc_cstr(b"tracingmacros\0".as_ptr() as *const c_char),
            INT_PAR__tracing_stats => print_esc_cstr(b"tracingstats\0".as_ptr() as *const c_char),
            INT_PAR__tracing_paragraphs => {
                print_esc_cstr(b"tracingparagraphs\0".as_ptr() as *const c_char)
            }
            INT_PAR__tracing_pages => print_esc_cstr(b"tracingpages\0".as_ptr() as *const c_char),
            INT_PAR__tracing_output => print_esc_cstr(b"tracingoutput\0".as_ptr() as *const c_char),
            INT_PAR__tracing_lost_chars => {
                print_esc_cstr(b"tracinglostchars\0".as_ptr() as *const c_char)
            }
            INT_PAR__tracing_commands => {
                print_esc_cstr(b"tracingcommands\0".as_ptr() as *const c_char)
            }
            INT_PAR__tracing_restores => {
                print_esc_cstr(b"tracingrestores\0".as_ptr() as *const c_char)
            }
            INT_PAR__uc_hyph => print_esc_cstr(b"uchyph\0".as_ptr() as *const c_char),
            INT_PAR__output_penalty => print_esc_cstr(b"outputpenalty\0".as_ptr() as *const c_char),
            INT_PAR__max_dead_cycles => {
                print_esc_cstr(b"maxdeadcycles\0".as_ptr() as *const c_char)
            }
            INT_PAR__hang_after => print_esc_cstr(b"hangafter\0".as_ptr() as *const c_char),
            INT_PAR__floating_penalty => {
                print_esc_cstr(b"floatingpenalty\0".as_ptr() as *const c_char)
            }
            INT_PAR__global_defs => print_esc_cstr(b"globaldefs\0".as_ptr() as *const c_char),
            INT_PAR__cur_fam => print_esc_cstr(b"fam\0".as_ptr() as *const c_char),
            INT_PAR__escape_char => print_esc_cstr(b"escapechar\0".as_ptr() as *const c_char),
            INT_PAR__default_hyphen_char => {
                print_esc_cstr(b"defaulthyphenchar\0".as_ptr() as *const c_char)
            }
            INT_PAR__default_skew_char => {
                print_esc_cstr(b"defaultskewchar\0".as_ptr() as *const c_char)
            }
            INT_PAR__end_line_char => print_esc_cstr(b"endlinechar\0".as_ptr() as *const c_char),
            INT_PAR__new_line_char => print_esc_cstr(b"newlinechar\0".as_ptr() as *const c_char),
            INT_PAR__language => print_esc_cstr(b"language\0".as_ptr() as *const c_char),
            INT_PAR__left_hyphen_min => {
                print_esc_cstr(b"lefthyphenmin\0".as_ptr() as *const c_char)
            }
            INT_PAR__right_hyphen_min => {
                print_esc_cstr(b"righthyphenmin\0".as_ptr() as *const c_char)
            }
            INT_PAR__holding_inserts => {
                print_esc_cstr(b"holdinginserts\0".as_ptr() as *const c_char)
            }
            INT_PAR__error_context_lines => {
                print_esc_cstr(b"errorcontextlines\0".as_ptr() as *const c_char)
            }
            INT_PAR__tracing_stack_levels => {
                print_esc_cstr(b"tracingstacklevels\0".as_ptr() as *const c_char)
            }
            INT_PAR__xetex_linebreak_penalty => {
                print_esc_cstr(b"XeTeXlinebreakpenalty\0".as_ptr() as *const c_char)
            }
            INT_PAR__xetex_protrude_chars => {
                print_esc_cstr(b"XeTeXprotrudechars\0".as_ptr() as *const c_char)
            }
            INT_PAR__synctex => print_esc_cstr(b"synctex\0".as_ptr() as *const c_char),
            INT_PAR__tracing_assigns => {
                print_esc_cstr(b"tracingassigns\0".as_ptr() as *const c_char)
            }
            INT_PAR__tracing_groups => print_esc_cstr(b"tracinggroups\0".as_ptr() as *const c_char),
            INT_PAR__tracing_ifs => print_esc_cstr(b"tracingifs\0".as_ptr() as *const c_char),
            INT_PAR__tracing_scan_tokens => {
                print_esc_cstr(b"tracingscantokens\0".as_ptr() as *const c_char)
            }
            INT_PAR__tracing_nesting => {
                print_esc_cstr(b"tracingnesting\0".as_ptr() as *const c_char)
            }
            INT_PAR__pre_display_direction => {
                print_esc_cstr(b"predisplaydirection\0".as_ptr() as *const c_char)
            }
            INT_PAR__last_line_fit => print_esc_cstr(b"lastlinefit\0".as_ptr() as *const c_char),
            INT_PAR__saving_vdiscards => {
                print_esc_cstr(b"savingvdiscards\0".as_ptr() as *const c_char)
            }
            INT_PAR__saving_hyph_codes => {
                print_esc_cstr(b"savinghyphcodes\0".as_ptr() as *const c_char)
            }
            INT_PAR__suppress_fontnotfound_error => {
                print_esc_cstr(b"suppressfontnotfounderror\0".as_ptr() as *const c_char)
            }
            INT_PAR__texxet => print_esc_cstr(b"TeXXeTstate\0".as_ptr() as *const c_char),
            INT_PAR__xetex_upwards => {
                print_esc_cstr(b"XeTeXupwardsmode\0".as_ptr() as *const c_char)
            }
            INT_PAR__xetex_use_glyph_metrics => {
                print_esc_cstr(b"XeTeXuseglyphmetrics\0".as_ptr() as *const c_char)
            }
            INT_PAR__xetex_inter_char_tokens => {
                print_esc_cstr(b"XeTeXinterchartokenstate\0".as_ptr() as *const c_char)
            }
            INT_PAR__xetex_dash_break => {
                print_esc_cstr(b"XeTeXdashbreakstate\0".as_ptr() as *const c_char)
            }
            INT_PAR__xetex_input_normalization => {
                print_esc_cstr(b"XeTeXinputnormalization\0".as_ptr() as *const c_char)
            }
            INT_PAR__xetex_tracing_fonts => {
                print_esc_cstr(b"XeTeXtracingfonts\0".as_ptr() as *const c_char)
            }
            INT_PAR__xetex_interword_space_shaping => {
                print_esc_cstr(b"XeTeXinterwordspaceshaping\0".as_ptr() as *const c_char)
            }
            INT_PAR__xetex_generate_actual_text => {
                print_esc_cstr(b"XeTeXgenerateactualtext\0".as_ptr() as *const c_char)
            }
            INT_PAR__xetex_hyphenatable_length => {
                print_esc_cstr(b"XeTeXhyphenatablelength\0".as_ptr() as *const c_char)
            }
            INT_PAR__pdfoutput => print_esc_cstr(b"pdfoutput\0".as_ptr() as *const c_char),
            _ => print_cstr(b"[unknown int32_t parameter!]\0".as_ptr() as *const c_char),
        }
    }
}

#[unsafe(export_name = "begin_diagnostic")]
pub unsafe extern "C" fn begin_diagnostic() {
    unsafe {
        old_setting = (selector) as u8;
        if INTPAR!(INT_PAR__tracing_online) <= 0 && selector == SELECTOR_TERM_AND_LOG {
            selector -= 1;
            if history == HISTORY_SPOTLESS {
                history = HISTORY_WARNING_ISSUED;
            }
        }
    }
}

#[unsafe(export_name = "end_diagnostic")]
pub unsafe extern "C" fn end_diagnostic(blank_line: bool) {
    unsafe {
        print_nl_cstr(b"\0".as_ptr() as *const c_char);
        if blank_line {
            print_ln();
        }
        selector = (old_setting) as i32;
    }
}

#[unsafe(export_name = "print_length_param")]
pub unsafe extern "C" fn print_length_param(n: i32) {
    unsafe {
        match (n) as i32 {
            DIMEN_PAR__par_indent => print_esc_cstr(b"parindent\0".as_ptr() as *const c_char),
            DIMEN_PAR__math_surround => print_esc_cstr(b"mathsurround\0".as_ptr() as *const c_char),
            DIMEN_PAR__line_skip_limit => {
                print_esc_cstr(b"lineskiplimit\0".as_ptr() as *const c_char)
            }
            DIMEN_PAR__hsize => print_esc_cstr(b"hsize\0".as_ptr() as *const c_char),
            DIMEN_PAR__vsize => print_esc_cstr(b"vsize\0".as_ptr() as *const c_char),
            DIMEN_PAR__max_depth => print_esc_cstr(b"maxdepth\0".as_ptr() as *const c_char),
            DIMEN_PAR__split_max_depth => {
                print_esc_cstr(b"splitmaxdepth\0".as_ptr() as *const c_char)
            }
            DIMEN_PAR__box_max_depth => print_esc_cstr(b"boxmaxdepth\0".as_ptr() as *const c_char),
            DIMEN_PAR__hfuzz => print_esc_cstr(b"hfuzz\0".as_ptr() as *const c_char),
            DIMEN_PAR__vfuzz => print_esc_cstr(b"vfuzz\0".as_ptr() as *const c_char),
            DIMEN_PAR__delimiter_shortfall => {
                print_esc_cstr(b"delimitershortfall\0".as_ptr() as *const c_char)
            }
            DIMEN_PAR__null_delimiter_space => {
                print_esc_cstr(b"nulldelimiterspace\0".as_ptr() as *const c_char)
            }
            DIMEN_PAR__script_space => print_esc_cstr(b"scriptspace\0".as_ptr() as *const c_char),
            DIMEN_PAR__pre_display_size => {
                print_esc_cstr(b"predisplaysize\0".as_ptr() as *const c_char)
            }
            DIMEN_PAR__display_width => print_esc_cstr(b"displaywidth\0".as_ptr() as *const c_char),
            DIMEN_PAR__display_indent => {
                print_esc_cstr(b"displayindent\0".as_ptr() as *const c_char)
            }
            DIMEN_PAR__overfull_rule => print_esc_cstr(b"overfullrule\0".as_ptr() as *const c_char),
            DIMEN_PAR__hang_indent => print_esc_cstr(b"hangindent\0".as_ptr() as *const c_char),
            DIMEN_PAR__h_offset => print_esc_cstr(b"hoffset\0".as_ptr() as *const c_char),
            DIMEN_PAR__v_offset => print_esc_cstr(b"voffset\0".as_ptr() as *const c_char),
            DIMEN_PAR__emergency_stretch => {
                print_esc_cstr(b"emergencystretch\0".as_ptr() as *const c_char)
            }
            DIMEN_PAR__pdf_page_width => {
                print_esc_cstr(b"pdfpagewidth\0".as_ptr() as *const c_char)
            }
            DIMEN_PAR__pdf_page_height => {
                print_esc_cstr(b"pdfpageheight\0".as_ptr() as *const c_char)
            }
            _ => print_cstr(b"[unknown dimen parameter!]\0".as_ptr() as *const c_char),
        }
    }
}

#[unsafe(export_name = "print_cmd_chr")]
pub unsafe extern "C" fn print_cmd_chr(cmd: u16, mut chr_code: i32) {
    unsafe {
        let mut n: i32;
        let font_name_str: str_number;
        let mut quote_char: UTF16_code;

        match (cmd) as i32 {
            LEFT_BRACE => {
                print_cstr(b"begin-group character \0".as_ptr() as *const c_char);
                if chr_code < 65536 {
                    print(chr_code);
                } else {
                    print_char(chr_code);
                }
            }
            RIGHT_BRACE => {
                print_cstr(b"end-group character \0".as_ptr() as *const c_char);
                if chr_code < 65536 {
                    print(chr_code);
                } else {
                    print_char(chr_code);
                }
            }
            MATH_SHIFT => {
                print_cstr(b"math shift character \0".as_ptr() as *const c_char);
                if chr_code < 65536 {
                    print(chr_code);
                } else {
                    print_char(chr_code);
                }
            }
            MAC_PARAM => {
                print_cstr(b"macro parameter character \0".as_ptr() as *const c_char);
                if chr_code < 65536 {
                    print(chr_code);
                } else {
                    print_char(chr_code);
                }
            }
            SUP_MARK => {
                print_cstr(b"superscript character \0".as_ptr() as *const c_char);
                if chr_code < 65536 {
                    print(chr_code);
                } else {
                    print_char(chr_code);
                }
            }
            SUB_MARK => {
                print_cstr(b"subscript character \0".as_ptr() as *const c_char);
                if chr_code < 65536 {
                    print(chr_code);
                } else {
                    print_char(chr_code);
                }
            }
            ENDV => {
                print_cstr(b"end of alignment template\0".as_ptr() as *const c_char);
            }
            SPACER => {
                print_cstr(b"blank space \0".as_ptr() as *const c_char);
                if chr_code < 65536 {
                    print(chr_code);
                } else {
                    print_char(chr_code);
                }
            }
            LETTER => {
                print_cstr(b"the letter \0".as_ptr() as *const c_char);
                if chr_code < 65536 {
                    print(chr_code);
                } else {
                    print_char(chr_code);
                }
            }
            OTHER_CHAR => {
                print_cstr(b"the character \0".as_ptr() as *const c_char);
                if chr_code < 65536 {
                    print(chr_code);
                } else {
                    print_char(chr_code);
                }
            }
            ASSIGN_GLUE | ASSIGN_MU_GLUE => {
                if chr_code < SKIP_BASE {
                    print_skip_param(chr_code - GLUE_BASE);
                } else if chr_code < MU_SKIP_BASE {
                    print_esc_cstr(b"skip\0".as_ptr() as *const c_char);
                    print_int(chr_code - SKIP_BASE);
                } else {
                    print_esc_cstr(b"muskip\0".as_ptr() as *const c_char);
                    print_int(chr_code - MU_SKIP_BASE);
                }
            }
            ASSIGN_TOKS => {
                if chr_code >= TOKS_BASE {
                    print_esc_cstr(b"toks\0".as_ptr() as *const c_char);
                    print_int(chr_code - TOKS_BASE);
                } else {
                    match (chr_code) as i32 {
                        x if x == LOCAL_BASE + LOCAL__output_routine => {
                            print_esc_cstr(b"output\0".as_ptr() as *const c_char)
                        }
                        x if x == LOCAL_BASE + LOCAL__every_par => {
                            print_esc_cstr(b"everypar\0".as_ptr() as *const c_char)
                        }
                        x if x == LOCAL_BASE + LOCAL__every_math => {
                            print_esc_cstr(b"everymath\0".as_ptr() as *const c_char)
                        }
                        x if x == LOCAL_BASE + LOCAL__every_display => {
                            print_esc_cstr(b"everydisplay\0".as_ptr() as *const c_char)
                        }
                        x if x == LOCAL_BASE + LOCAL__every_hbox => {
                            print_esc_cstr(b"everyhbox\0".as_ptr() as *const c_char)
                        }
                        x if x == LOCAL_BASE + LOCAL__every_vbox => {
                            print_esc_cstr(b"everyvbox\0".as_ptr() as *const c_char)
                        }
                        x if x == LOCAL_BASE + LOCAL__every_job => {
                            print_esc_cstr(b"everyjob\0".as_ptr() as *const c_char)
                        }
                        x if x == LOCAL_BASE + LOCAL__every_cr => {
                            print_esc_cstr(b"everycr\0".as_ptr() as *const c_char)
                        }
                        x if x == LOCAL_BASE + LOCAL__every_eof => {
                            print_esc_cstr(b"everyeof\0".as_ptr() as *const c_char)
                        }
                        x if x == LOCAL_BASE + LOCAL__xetex_inter_char_toks => {
                            print_esc_cstr(b"XeTeXinterchartoks\0".as_ptr() as *const c_char)
                        }
                        x if x == LOCAL_BASE + LOCAL__tectonic_coda_tokens => {
                            print_esc_cstr(b"TectonicCodaTokens\0".as_ptr() as *const c_char)
                        }
                        _ => print_esc_cstr(b"errhelp\0".as_ptr() as *const c_char),
                    }
                }
            }
            ASSIGN_INT => {
                if chr_code < COUNT_BASE {
                    print_param(chr_code - INT_BASE);
                } else {
                    print_esc_cstr(b"count\0".as_ptr() as *const c_char);
                    print_int(chr_code - COUNT_BASE);
                }
            }
            ASSIGN_DIMEN => {
                if chr_code < SCALED_BASE {
                    print_length_param(chr_code - DIMEN_BASE);
                } else {
                    print_esc_cstr(b"dimen\0".as_ptr() as *const c_char);
                    print_int(chr_code - SCALED_BASE);
                }
            }
            ACCENT => print_esc_cstr(b"accent\0".as_ptr() as *const c_char),
            ADVANCE => print_esc_cstr(b"advance\0".as_ptr() as *const c_char),
            AFTER_ASSIGNMENT => print_esc_cstr(b"afterassignment\0".as_ptr() as *const c_char),
            AFTER_GROUP => print_esc_cstr(b"aftergroup\0".as_ptr() as *const c_char),
            ASSIGN_FONT_DIMEN => print_esc_cstr(b"fontdimen\0".as_ptr() as *const c_char),
            BEGIN_GROUP => print_esc_cstr(b"begingroup\0".as_ptr() as *const c_char),
            BREAK_PENALTY => print_esc_cstr(b"penalty\0".as_ptr() as *const c_char),
            CHAR_NUM => print_esc_cstr(b"char\0".as_ptr() as *const c_char),
            CS_NAME => print_esc_cstr(b"csname\0".as_ptr() as *const c_char),
            DEF_FONT => print_esc_cstr(b"font\0".as_ptr() as *const c_char),
            DELIM_NUM => {
                if chr_code == 1 {
                    print_esc_cstr(b"Udelimiter\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"delimiter\0".as_ptr() as *const c_char);
                }
            }
            DIVIDE => print_esc_cstr(b"divide\0".as_ptr() as *const c_char),
            END_CS_NAME => print_esc_cstr(b"endcsname\0".as_ptr() as *const c_char),
            END_GROUP => print_esc_cstr(b"endgroup\0".as_ptr() as *const c_char),
            EX_SPACE => print_esc(b' ' as i32),
            EXPAND_AFTER => {
                if chr_code == 0 {
                    print_esc_cstr(b"expandafter\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"unless\0".as_ptr() as *const c_char);
                }
            }
            HALIGN => print_esc_cstr(b"halign\0".as_ptr() as *const c_char),
            HRULE => print_esc_cstr(b"hrule\0".as_ptr() as *const c_char),
            IGNORE_SPACES => {
                if chr_code == 0 {
                    print_esc_cstr(b"ignorespaces\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"primitive\0".as_ptr() as *const c_char);
                }
            }
            INSERT => print_esc_cstr(b"insert\0".as_ptr() as *const c_char),
            ITAL_CORR => print_esc(b'/' as i32),
            MARK => {
                print_esc_cstr(b"mark\0".as_ptr() as *const c_char);
                if chr_code > 0 {
                    print_char(b's' as i32);
                }
            }
            MATH_ACCENT => {
                if chr_code == 1 {
                    print_esc_cstr(b"Umathaccent\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"mathaccent\0".as_ptr() as *const c_char);
                }
            }
            MATH_CHAR_NUM => {
                if chr_code == 2 {
                    print_esc_cstr(b"Umathchar\0".as_ptr() as *const c_char);
                } else if chr_code == 1 {
                    print_esc_cstr(b"Umathcharnum\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"mathchar\0".as_ptr() as *const c_char);
                }
            }
            MATH_CHOICE => print_esc_cstr(b"mathchoice\0".as_ptr() as *const c_char),
            MULTIPLY => print_esc_cstr(b"multiply\0".as_ptr() as *const c_char),
            NO_ALIGN => print_esc_cstr(b"noalign\0".as_ptr() as *const c_char),
            NO_BOUNDARY => print_esc_cstr(b"noboundary\0".as_ptr() as *const c_char),
            NO_EXPAND => {
                if chr_code == 0 {
                    print_esc_cstr(b"noexpand\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"primitive\0".as_ptr() as *const c_char);
                }
            }
            NON_SCRIPT => print_esc_cstr(b"nonscript\0".as_ptr() as *const c_char),
            OMIT => print_esc_cstr(b"omit\0".as_ptr() as *const c_char),
            RADICAL => {
                if chr_code == 1 {
                    print_esc_cstr(b"Uradical\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"radical\0".as_ptr() as *const c_char);
                }
            }
            READ_TO_CS => {
                if chr_code == 0 {
                    print_esc_cstr(b"read\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"readline\0".as_ptr() as *const c_char);
                }
            }
            RELAX => print_esc_cstr(b"relax\0".as_ptr() as *const c_char),
            SET_BOX => print_esc_cstr(b"setbox\0".as_ptr() as *const c_char),
            SET_PREV_GRAF => print_esc_cstr(b"prevgraf\0".as_ptr() as *const c_char),
            SET_SHAPE => match (chr_code) as i32 {
                x if x == LOCAL_BASE + LOCAL__par_shape => {
                    print_esc_cstr(b"parshape\0".as_ptr() as *const c_char)
                }
                x if x == ETEX_PEN_BASE + ETEX_PENALTIES_PAR__inter_line_penalties => {
                    print_esc_cstr(b"interlinepenalties\0".as_ptr() as *const c_char)
                }
                x if x == ETEX_PEN_BASE + ETEX_PENALTIES_PAR__club_penalties => {
                    print_esc_cstr(b"clubpenalties\0".as_ptr() as *const c_char)
                }
                x if x == ETEX_PEN_BASE + ETEX_PENALTIES_PAR__widow_penalties => {
                    print_esc_cstr(b"widowpenalties\0".as_ptr() as *const c_char)
                }
                x if x == ETEX_PEN_BASE + ETEX_PENALTIES_PAR__display_widow_penalties => {
                    print_esc_cstr(b"displaywidowpenalties\0".as_ptr() as *const c_char)
                }
                _ => {}
            },
            THE => {
                if chr_code == 0 {
                    print_esc_cstr(b"the\0".as_ptr() as *const c_char);
                } else if chr_code == 1 {
                    print_esc_cstr(b"unexpanded\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"detokenize\0".as_ptr() as *const c_char);
                }
            }
            TOKS_REGISTER => {
                print_esc_cstr(b"toks\0".as_ptr() as *const c_char);
                if chr_code != 0 {
                    print_sa_num(chr_code);
                }
            }
            VADJUST => print_esc_cstr(b"vadjust\0".as_ptr() as *const c_char),
            VALIGN => {
                if chr_code == 0 {
                    print_esc_cstr(b"valign\0".as_ptr() as *const c_char);
                } else {
                    match (chr_code) as i32 {
                        BEGIN_L_CODE => print_esc_cstr(b"beginL\0".as_ptr() as *const c_char),
                        END_L_CODE => print_esc_cstr(b"endL\0".as_ptr() as *const c_char),
                        BEGIN_R_CODE => print_esc_cstr(b"beginR\0".as_ptr() as *const c_char),
                        _ => print_esc_cstr(b"endR\0".as_ptr() as *const c_char),
                    }
                }
            }
            VCENTER => print_esc_cstr(b"vcenter\0".as_ptr() as *const c_char),
            VRULE => print_esc_cstr(b"vrule\0".as_ptr() as *const c_char),
            PAR_END => print_esc_cstr(b"par\0".as_ptr() as *const c_char),
            INPUT => {
                if chr_code == 0 {
                    print_esc_cstr(b"input\0".as_ptr() as *const c_char);
                } else if chr_code == 2 {
                    print_esc_cstr(b"scantokens\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"endinput\0".as_ptr() as *const c_char);
                }
            }
            TOP_BOT_MARK => {
                match (chr_code % MARKS_CODE) as i32 {
                    FIRST_MARK_CODE => print_esc_cstr(b"firstmark\0".as_ptr() as *const c_char),
                    BOT_MARK_CODE => print_esc_cstr(b"botmark\0".as_ptr() as *const c_char),
                    SPLIT_FIRST_MARK_CODE => {
                        print_esc_cstr(b"splitfirstmark\0".as_ptr() as *const c_char)
                    }
                    SPLIT_BOT_MARK_CODE => {
                        print_esc_cstr(b"splitbotmark\0".as_ptr() as *const c_char)
                    }
                    _ => print_esc_cstr(b"topmark\0".as_ptr() as *const c_char),
                }
                if chr_code >= MARKS_CODE {
                    print_char(b's' as i32);
                }
            }
            REGISTER => {
                let actual_cmd: u16 = if chr_code < 0 || chr_code > 19 {
                    (mem!(chr_code).b16.s1 / 64) as u16
                } else {
                    let c = chr_code as u16;
                    chr_code = TEX_NULL;
                    c
                };
                if actual_cmd == INT_VAL as u16 {
                    print_esc_cstr(b"count\0".as_ptr() as *const c_char);
                } else if actual_cmd == DIMEN_VAL as u16 {
                    print_esc_cstr(b"dimen\0".as_ptr() as *const c_char);
                } else if actual_cmd == GLUE_VAL as u16 {
                    print_esc_cstr(b"skip\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"muskip\0".as_ptr() as *const c_char);
                }
                if chr_code != TEX_NULL {
                    print_sa_num(chr_code);
                }
            }
            SET_AUX => {
                if chr_code == VMODE as i32 {
                    print_esc_cstr(b"prevdepth\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"spacefactor\0".as_ptr() as *const c_char);
                }
            }
            SET_PAGE_INT => {
                if chr_code == 0 {
                    print_esc_cstr(b"deadcycles\0".as_ptr() as *const c_char);
                } else if chr_code == 2 {
                    print_esc_cstr(b"interactionmode\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"insertpenalties\0".as_ptr() as *const c_char);
                }
            }
            SET_BOX_DIMEN => {
                if chr_code == WIDTH_OFFSET {
                    print_esc_cstr(b"wd\0".as_ptr() as *const c_char);
                } else if chr_code == HEIGHT_OFFSET {
                    print_esc_cstr(b"ht\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"dp\0".as_ptr() as *const c_char);
                }
            }
            LAST_ITEM => match (chr_code) as i32 {
                INT_VAL => print_esc_cstr(b"lastpenalty\0".as_ptr() as *const c_char),
                DIMEN_VAL => print_esc_cstr(b"lastkern\0".as_ptr() as *const c_char),
                GLUE_VAL => print_esc_cstr(b"lastskip\0".as_ptr() as *const c_char),
                INPUT_LINE_NO_CODE => print_esc_cstr(b"inputlineno\0".as_ptr() as *const c_char),
                LAST_NODE_TYPE_CODE => print_esc_cstr(b"lastnodetype\0".as_ptr() as *const c_char),
                ETEX_VERSION_CODE => print_esc_cstr(b"eTeXversion\0".as_ptr() as *const c_char),
                XETEX_VERSION_CODE => print_esc_cstr(b"XeTeXversion\0".as_ptr() as *const c_char),
                XETEX_COUNT_GLYPHS_CODE => {
                    print_esc_cstr(b"XeTeXcountglyphs\0".as_ptr() as *const c_char)
                }
                XETEX_COUNT_VARIATIONS_CODE => {
                    print_esc_cstr(b"XeTeXcountvariations\0".as_ptr() as *const c_char)
                }
                XETEX_VARIATION_CODE => {
                    print_esc_cstr(b"XeTeXvariation\0".as_ptr() as *const c_char)
                }
                XETEX_FIND_VARIATION_BY_NAME_CODE => {
                    print_esc_cstr(b"XeTeXfindvariationbyname\0".as_ptr() as *const c_char)
                }
                XETEX_VARIATION_MIN_CODE => {
                    print_esc_cstr(b"XeTeXvariationmin\0".as_ptr() as *const c_char)
                }
                XETEX_VARIATION_MAX_CODE => {
                    print_esc_cstr(b"XeTeXvariationmax\0".as_ptr() as *const c_char)
                }
                XETEX_VARIATION_DEFAULT_CODE => {
                    print_esc_cstr(b"XeTeXvariationdefault\0".as_ptr() as *const c_char)
                }
                XETEX_COUNT_FEATURES_CODE => {
                    print_esc_cstr(b"XeTeXcountfeatures\0".as_ptr() as *const c_char)
                }
                XETEX_FEATURE_CODE_CODE => {
                    print_esc_cstr(b"XeTeXfeaturecode\0".as_ptr() as *const c_char)
                }
                XETEX_FIND_FEATURE_BY_NAME_CODE => {
                    print_esc_cstr(b"XeTeXfindfeaturebyname\0".as_ptr() as *const c_char)
                }
                XETEX_IS_EXCLUSIVE_FEATURE_CODE => {
                    print_esc_cstr(b"XeTeXisexclusivefeature\0".as_ptr() as *const c_char)
                }
                XETEX_COUNT_SELECTORS_CODE => {
                    print_esc_cstr(b"XeTeXcountselectors\0".as_ptr() as *const c_char)
                }
                XETEX_SELECTOR_CODE_CODE => {
                    print_esc_cstr(b"XeTeXselectorcode\0".as_ptr() as *const c_char)
                }
                XETEX_FIND_SELECTOR_BY_NAME_CODE => {
                    print_esc_cstr(b"XeTeXfindselectorbyname\0".as_ptr() as *const c_char)
                }
                XETEX_IS_DEFAULT_SELECTOR_CODE => {
                    print_esc_cstr(b"XeTeXisdefaultselector\0".as_ptr() as *const c_char)
                }
                XETEX_OT_COUNT_SCRIPTS_CODE => {
                    print_esc_cstr(b"XeTeXOTcountscripts\0".as_ptr() as *const c_char)
                }
                XETEX_OT_COUNT_LANGUAGES_CODE => {
                    print_esc_cstr(b"XeTeXOTcountlanguages\0".as_ptr() as *const c_char)
                }
                XETEX_OT_COUNT_FEATURES_CODE => {
                    print_esc_cstr(b"XeTeXOTcountfeatures\0".as_ptr() as *const c_char)
                }
                XETEX_OT_SCRIPT_CODE => {
                    print_esc_cstr(b"XeTeXOTscripttag\0".as_ptr() as *const c_char)
                }
                XETEX_OT_LANGUAGE_CODE => {
                    print_esc_cstr(b"XeTeXOTlanguagetag\0".as_ptr() as *const c_char)
                }
                XETEX_OT_FEATURE_CODE => {
                    print_esc_cstr(b"XeTeXOTfeaturetag\0".as_ptr() as *const c_char)
                }
                XETEX_MAP_CHAR_TO_GLYPH_CODE => {
                    print_esc_cstr(b"XeTeXcharglyph\0".as_ptr() as *const c_char)
                }
                XETEX_GLYPH_INDEX_CODE => {
                    print_esc_cstr(b"XeTeXglyphindex\0".as_ptr() as *const c_char)
                }
                XETEX_GLYPH_BOUNDS_CODE => {
                    print_esc_cstr(b"XeTeXglyphbounds\0".as_ptr() as *const c_char)
                }
                XETEX_FONT_TYPE_CODE => {
                    print_esc_cstr(b"XeTeXfonttype\0".as_ptr() as *const c_char)
                }
                XETEX_FIRST_CHAR_CODE => {
                    print_esc_cstr(b"XeTeXfirstfontchar\0".as_ptr() as *const c_char)
                }
                XETEX_LAST_CHAR_CODE => {
                    print_esc_cstr(b"XeTeXlastfontchar\0".as_ptr() as *const c_char)
                }
                XETEX_PDF_PAGE_COUNT_CODE => {
                    print_esc_cstr(b"XeTeXpdfpagecount\0".as_ptr() as *const c_char)
                }
                CURRENT_GROUP_LEVEL_CODE => {
                    print_esc_cstr(b"currentgrouplevel\0".as_ptr() as *const c_char)
                }
                CURRENT_GROUP_TYPE_CODE => {
                    print_esc_cstr(b"currentgrouptype\0".as_ptr() as *const c_char)
                }
                CURRENT_IF_LEVEL_CODE => {
                    print_esc_cstr(b"currentiflevel\0".as_ptr() as *const c_char)
                }
                CURRENT_IF_TYPE_CODE => {
                    print_esc_cstr(b"currentiftype\0".as_ptr() as *const c_char)
                }
                CURRENT_IF_BRANCH_CODE => {
                    print_esc_cstr(b"currentifbranch\0".as_ptr() as *const c_char)
                }
                FONT_CHAR_WD_CODE => print_esc_cstr(b"fontcharwd\0".as_ptr() as *const c_char),
                FONT_CHAR_HT_CODE => print_esc_cstr(b"fontcharht\0".as_ptr() as *const c_char),
                FONT_CHAR_DP_CODE => print_esc_cstr(b"fontchardp\0".as_ptr() as *const c_char),
                FONT_CHAR_IC_CODE => print_esc_cstr(b"fontcharic\0".as_ptr() as *const c_char),
                PAR_SHAPE_LENGTH_CODE => {
                    print_esc_cstr(b"parshapelength\0".as_ptr() as *const c_char)
                }
                PAR_SHAPE_INDENT_CODE => {
                    print_esc_cstr(b"parshapeindent\0".as_ptr() as *const c_char)
                }
                PAR_SHAPE_DIMEN_CODE => {
                    print_esc_cstr(b"parshapedimen\0".as_ptr() as *const c_char)
                }
                x if x == (ETEX_EXPR - INT_VAL + INT_VAL) => {
                    print_esc_cstr(b"numexpr\0".as_ptr() as *const c_char)
                }
                x if x == (ETEX_EXPR - INT_VAL + DIMEN_VAL) => {
                    print_esc_cstr(b"dimexpr\0".as_ptr() as *const c_char)
                }
                x if x == (ETEX_EXPR - INT_VAL + GLUE_VAL) => {
                    print_esc_cstr(b"glueexpr\0".as_ptr() as *const c_char)
                }
                x if x == (ETEX_EXPR - INT_VAL + MU_VAL) => {
                    print_esc_cstr(b"muexpr\0".as_ptr() as *const c_char)
                }
                GLUE_STRETCH_ORDER_CODE => {
                    print_esc_cstr(b"gluestretchorder\0".as_ptr() as *const c_char)
                }
                GLUE_SHRINK_ORDER_CODE => {
                    print_esc_cstr(b"glueshrinkorder\0".as_ptr() as *const c_char)
                }
                GLUE_STRETCH_CODE => print_esc_cstr(b"gluestretch\0".as_ptr() as *const c_char),
                GLUE_SHRINK_CODE => print_esc_cstr(b"glueshrink\0".as_ptr() as *const c_char),
                MU_TO_GLUE_CODE => print_esc_cstr(b"mutoglue\0".as_ptr() as *const c_char),
                GLUE_TO_MU_CODE => print_esc_cstr(b"gluetomu\0".as_ptr() as *const c_char),
                PDF_LAST_X_POS_CODE => print_esc_cstr(b"pdflastxpos\0".as_ptr() as *const c_char),
                PDF_LAST_Y_POS_CODE => print_esc_cstr(b"pdflastypos\0".as_ptr() as *const c_char),
                ELAPSED_TIME_CODE => print_esc_cstr(b"elapsedtime\0".as_ptr() as *const c_char),
                PDF_SHELL_ESCAPE_CODE => print_esc_cstr(b"shellescape\0".as_ptr() as *const c_char),
                RANDOM_SEED_CODE => print_esc_cstr(b"randomseed\0".as_ptr() as *const c_char),
                _ => print_esc_cstr(b"badness\0".as_ptr() as *const c_char),
            },
            CONVERT => match (chr_code) as i32 {
                NUMBER_CODE => print_esc_cstr(b"number\0".as_ptr() as *const c_char),
                ROMAN_NUMERAL_CODE => print_esc_cstr(b"romannumeral\0".as_ptr() as *const c_char),
                STRING_CODE => print_esc_cstr(b"string\0".as_ptr() as *const c_char),
                MEANING_CODE => print_esc_cstr(b"meaning\0".as_ptr() as *const c_char),
                FONT_NAME_CODE => print_esc_cstr(b"fontname\0".as_ptr() as *const c_char),
                ETEX_REVISION_CODE => print_esc_cstr(b"eTeXrevision\0".as_ptr() as *const c_char),
                EXPANDED_CODE => print_esc_cstr(b"expanded\0".as_ptr() as *const c_char),
                LEFT_MARGIN_KERN_CODE => {
                    print_esc_cstr(b"leftmarginkern\0".as_ptr() as *const c_char)
                }
                RIGHT_MARGIN_KERN_CODE => {
                    print_esc_cstr(b"rightmarginkern\0".as_ptr() as *const c_char)
                }
                PDF_CREATION_DATE_CODE => {
                    print_esc_cstr(b"creationdate\0".as_ptr() as *const c_char)
                }
                PDF_FILE_MOD_DATE_CODE => {
                    print_esc_cstr(b"filemoddate\0".as_ptr() as *const c_char)
                }
                PDF_FILE_SIZE_CODE => print_esc_cstr(b"filesize\0".as_ptr() as *const c_char),
                PDF_MDFIVE_SUM_CODE => print_esc_cstr(b"mdfivesum\0".as_ptr() as *const c_char),
                PDF_FILE_DUMP_CODE => print_esc_cstr(b"filedump\0".as_ptr() as *const c_char),
                PDF_STRCMP_CODE => print_esc_cstr(b"strcmp\0".as_ptr() as *const c_char),
                UNIFORM_DEVIATE_CODE => {
                    print_esc_cstr(b"uniformdeviate\0".as_ptr() as *const c_char)
                }
                NORMAL_DEVIATE_CODE => print_esc_cstr(b"normaldeviate\0".as_ptr() as *const c_char),
                XETEX_REVISION_CODE => print_esc_cstr(b"XeTeXrevision\0".as_ptr() as *const c_char),
                XETEX_VARIATION_NAME_CODE => {
                    print_esc_cstr(b"XeTeXvariationname\0".as_ptr() as *const c_char)
                }
                XETEX_FEATURE_NAME_CODE => {
                    print_esc_cstr(b"XeTeXfeaturename\0".as_ptr() as *const c_char)
                }
                XETEX_SELECTOR_NAME_CODE => {
                    print_esc_cstr(b"XeTeXselectorname\0".as_ptr() as *const c_char)
                }
                XETEX_GLYPH_NAME_CODE => {
                    print_esc_cstr(b"XeTeXglyphname\0".as_ptr() as *const c_char)
                }
                XETEX_UCHAR_CODE => print_esc_cstr(b"Uchar\0".as_ptr() as *const c_char),
                XETEX_UCHARCAT_CODE => print_esc_cstr(b"Ucharcat\0".as_ptr() as *const c_char),
                _ => print_esc_cstr(b"jobname\0".as_ptr() as *const c_char),
            },
            IF_TEST => {
                if chr_code >= UNLESS_CODE {
                    print_esc_cstr(b"unless\0".as_ptr() as *const c_char);
                }
                match (chr_code % UNLESS_CODE) as i32 {
                    IF_CAT_CODE => print_esc_cstr(b"ifcat\0".as_ptr() as *const c_char),
                    IF_INT_CODE => print_esc_cstr(b"ifnum\0".as_ptr() as *const c_char),
                    IF_DIM_CODE => print_esc_cstr(b"ifdim\0".as_ptr() as *const c_char),
                    IF_ODD_CODE => print_esc_cstr(b"ifodd\0".as_ptr() as *const c_char),
                    IF_VMODE_CODE => print_esc_cstr(b"ifvmode\0".as_ptr() as *const c_char),
                    IF_HMODE_CODE => print_esc_cstr(b"ifhmode\0".as_ptr() as *const c_char),
                    IF_MMODE_CODE => print_esc_cstr(b"ifmmode\0".as_ptr() as *const c_char),
                    IF_INNER_CODE => print_esc_cstr(b"ifinner\0".as_ptr() as *const c_char),
                    IF_VOID_CODE => print_esc_cstr(b"ifvoid\0".as_ptr() as *const c_char),
                    IF_HBOX_CODE => print_esc_cstr(b"ifhbox\0".as_ptr() as *const c_char),
                    IF_VBOX_CODE => print_esc_cstr(b"ifvbox\0".as_ptr() as *const c_char),
                    IFX_CODE => print_esc_cstr(b"ifx\0".as_ptr() as *const c_char),
                    IF_EOF_CODE => print_esc_cstr(b"ifeof\0".as_ptr() as *const c_char),
                    IF_TRUE_CODE => print_esc_cstr(b"iftrue\0".as_ptr() as *const c_char),
                    IF_FALSE_CODE => print_esc_cstr(b"iffalse\0".as_ptr() as *const c_char),
                    IF_CASE_CODE => print_esc_cstr(b"ifcase\0".as_ptr() as *const c_char),
                    IF_PRIMITIVE_CODE => print_esc_cstr(b"ifprimitive\0".as_ptr() as *const c_char),
                    IF_DEF_CODE => print_esc_cstr(b"ifdefined\0".as_ptr() as *const c_char),
                    IF_CS_CODE => print_esc_cstr(b"ifcsname\0".as_ptr() as *const c_char),
                    IF_FONT_CHAR_CODE => print_esc_cstr(b"iffontchar\0".as_ptr() as *const c_char),
                    IF_IN_CSNAME_CODE => print_esc_cstr(b"ifincsname\0".as_ptr() as *const c_char),
                    _ => print_esc_cstr(b"if\0".as_ptr() as *const c_char),
                }
            }
            FI_OR_ELSE => {
                if chr_code == FI_CODE {
                    print_esc_cstr(b"fi\0".as_ptr() as *const c_char);
                } else if chr_code == OR_CODE {
                    print_esc_cstr(b"or\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"else\0".as_ptr() as *const c_char);
                }
            }
            TAB_MARK => {
                if chr_code == SPAN_CODE {
                    print_esc_cstr(b"span\0".as_ptr() as *const c_char);
                } else {
                    print_cstr(b"alignment tab character \0".as_ptr() as *const c_char);
                    if chr_code < 65536 {
                        print(chr_code);
                    } else {
                        print_char(chr_code);
                    }
                }
            }
            CAR_RET => {
                if chr_code == CR_CODE {
                    print_esc_cstr(b"cr\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"crcr\0".as_ptr() as *const c_char);
                }
            }
            SET_PAGE_DIMEN => match (chr_code) as i32 {
                0 => print_esc_cstr(b"pagegoal\0".as_ptr() as *const c_char),
                1 => print_esc_cstr(b"pagetotal\0".as_ptr() as *const c_char),
                2 => print_esc_cstr(b"pagestretch\0".as_ptr() as *const c_char),
                3 => print_esc_cstr(b"pagefilstretch\0".as_ptr() as *const c_char),
                4 => print_esc_cstr(b"pagefillstretch\0".as_ptr() as *const c_char),
                5 => print_esc_cstr(b"pagefilllstretch\0".as_ptr() as *const c_char),
                6 => print_esc_cstr(b"pageshrink\0".as_ptr() as *const c_char),
                _ => print_esc_cstr(b"pagedepth\0".as_ptr() as *const c_char),
            },
            STOP => {
                if chr_code == 1 {
                    print_esc_cstr(b"dump\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"end\0".as_ptr() as *const c_char);
                }
            }
            HSKIP => match (chr_code) as i32 {
                SKIP_CODE => print_esc_cstr(b"hskip\0".as_ptr() as *const c_char),
                FIL_CODE => print_esc_cstr(b"hfil\0".as_ptr() as *const c_char),
                FILL_CODE => print_esc_cstr(b"hfill\0".as_ptr() as *const c_char),
                SS_CODE => print_esc_cstr(b"hss\0".as_ptr() as *const c_char),
                _ => print_esc_cstr(b"hfilneg\0".as_ptr() as *const c_char),
            },
            VSKIP => match (chr_code) as i32 {
                SKIP_CODE => print_esc_cstr(b"vskip\0".as_ptr() as *const c_char),
                FIL_CODE => print_esc_cstr(b"vfil\0".as_ptr() as *const c_char),
                FILL_CODE => print_esc_cstr(b"vfill\0".as_ptr() as *const c_char),
                SS_CODE => print_esc_cstr(b"vss\0".as_ptr() as *const c_char),
                _ => print_esc_cstr(b"vfilneg\0".as_ptr() as *const c_char),
            },
            MSKIP => print_esc_cstr(b"mskip\0".as_ptr() as *const c_char),
            KERN => print_esc_cstr(b"kern\0".as_ptr() as *const c_char),
            MKERN => print_esc_cstr(b"mkern\0".as_ptr() as *const c_char),
            HMOVE => {
                if chr_code == 1 {
                    print_esc_cstr(b"moveleft\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"moveright\0".as_ptr() as *const c_char);
                }
            }
            VMOVE => {
                if chr_code == 1 {
                    print_esc_cstr(b"raise\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"lower\0".as_ptr() as *const c_char);
                }
            }
            MAKE_BOX => match (chr_code) as i32 {
                BOX_CODE => print_esc_cstr(b"box\0".as_ptr() as *const c_char),
                COPY_CODE => print_esc_cstr(b"copy\0".as_ptr() as *const c_char),
                LAST_BOX_CODE => print_esc_cstr(b"lastbox\0".as_ptr() as *const c_char),
                VSPLIT_CODE => print_esc_cstr(b"vsplit\0".as_ptr() as *const c_char),
                VTOP_CODE => print_esc_cstr(b"vtop\0".as_ptr() as *const c_char),
                x if x == (VTOP_CODE + VMODE as i32) => {
                    print_esc_cstr(b"vbox\0".as_ptr() as *const c_char)
                }
                _ => print_esc_cstr(b"hbox\0".as_ptr() as *const c_char),
            },
            LEADER_SHIP => {
                if chr_code == A_LEADERS {
                    print_esc_cstr(b"leaders\0".as_ptr() as *const c_char);
                } else if chr_code == C_LEADERS {
                    print_esc_cstr(b"cleaders\0".as_ptr() as *const c_char);
                } else if chr_code == X_LEADERS {
                    print_esc_cstr(b"xleaders\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"shipout\0".as_ptr() as *const c_char);
                }
            }
            START_PAR => {
                if chr_code == 0 {
                    print_esc_cstr(b"noindent\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"indent\0".as_ptr() as *const c_char);
                }
            }
            REMOVE_ITEM => {
                if chr_code == GLUE_NODE as i32 {
                    print_esc_cstr(b"unskip\0".as_ptr() as *const c_char);
                } else if chr_code == KERN_NODE as i32 {
                    print_esc_cstr(b"unkern\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"unpenalty\0".as_ptr() as *const c_char);
                }
            }
            UN_HBOX => {
                if chr_code == COPY_CODE {
                    print_esc_cstr(b"unhcopy\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"unhbox\0".as_ptr() as *const c_char);
                }
            }
            UN_VBOX => {
                if chr_code == COPY_CODE {
                    print_esc_cstr(b"unvcopy\0".as_ptr() as *const c_char);
                } else if chr_code == LAST_BOX_CODE {
                    print_esc_cstr(b"pagediscards\0".as_ptr() as *const c_char);
                } else if chr_code == VSPLIT_CODE {
                    print_esc_cstr(b"splitdiscards\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"unvbox\0".as_ptr() as *const c_char);
                }
            }
            DISCRETIONARY => {
                if chr_code == 1 {
                    print_esc(b'-' as i32);
                } else {
                    print_esc_cstr(b"discretionary\0".as_ptr() as *const c_char);
                }
            }
            EQ_NO => {
                if chr_code == 1 {
                    print_esc_cstr(b"leqno\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"eqno\0".as_ptr() as *const c_char);
                }
            }
            MATH_COMP => match (chr_code) as i32 {
                value if value == ORD_NOAD as i32 => {
                    print_esc_cstr(b"mathord\0".as_ptr() as *const c_char)
                }
                value if value == OP_NOAD as i32 => {
                    print_esc_cstr(b"mathop\0".as_ptr() as *const c_char)
                }
                value if value == BIN_NOAD as i32 => {
                    print_esc_cstr(b"mathbin\0".as_ptr() as *const c_char)
                }
                value if value == REL_NOAD as i32 => {
                    print_esc_cstr(b"mathrel\0".as_ptr() as *const c_char)
                }
                value if value == OPEN_NOAD as i32 => {
                    print_esc_cstr(b"mathopen\0".as_ptr() as *const c_char)
                }
                value if value == CLOSE_NOAD as i32 => {
                    print_esc_cstr(b"mathclose\0".as_ptr() as *const c_char)
                }
                value if value == PUNCT_NOAD as i32 => {
                    print_esc_cstr(b"mathpunct\0".as_ptr() as *const c_char)
                }
                value if value == INNER_NOAD as i32 => {
                    print_esc_cstr(b"mathinner\0".as_ptr() as *const c_char)
                }
                value if value == UNDER_NOAD as i32 => {
                    print_esc_cstr(b"underline\0".as_ptr() as *const c_char)
                }
                _ => print_esc_cstr(b"overline\0".as_ptr() as *const c_char),
            },
            LIMIT_SWITCH => {
                if chr_code == LIMITS as i32 {
                    print_esc_cstr(b"limits\0".as_ptr() as *const c_char);
                } else if chr_code == NO_LIMITS as i32 {
                    print_esc_cstr(b"nolimits\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"displaylimits\0".as_ptr() as *const c_char);
                }
            }
            MATH_STYLE => print_style(chr_code),
            ABOVE => match (chr_code) as i32 {
                OVER_CODE => print_esc_cstr(b"over\0".as_ptr() as *const c_char),
                ATOP_CODE => print_esc_cstr(b"atop\0".as_ptr() as *const c_char),
                x if x == DELIMITED_CODE + ABOVE_CODE => {
                    print_esc_cstr(b"abovewithdelims\0".as_ptr() as *const c_char)
                }
                x if x == DELIMITED_CODE + OVER_CODE => {
                    print_esc_cstr(b"overwithdelims\0".as_ptr() as *const c_char)
                }
                x if x == DELIMITED_CODE + ATOP_CODE => {
                    print_esc_cstr(b"atopwithdelims\0".as_ptr() as *const c_char)
                }
                _ => print_esc_cstr(b"above\0".as_ptr() as *const c_char),
            },
            LEFT_RIGHT => {
                if chr_code == LEFT_NOAD as i32 {
                    print_esc_cstr(b"left\0".as_ptr() as *const c_char);
                } else if chr_code == MIDDLE_NOAD as i32 {
                    print_esc_cstr(b"middle\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"right\0".as_ptr() as *const c_char);
                }
            }
            PREFIX => {
                if chr_code == 1 {
                    print_esc_cstr(b"long\0".as_ptr() as *const c_char);
                } else if chr_code == 2 {
                    print_esc_cstr(b"outer\0".as_ptr() as *const c_char);
                } else if chr_code == 8 {
                    print_esc_cstr(b"protected\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"global\0".as_ptr() as *const c_char);
                }
            }
            DEF => {
                if chr_code == 0 {
                    print_esc_cstr(b"def\0".as_ptr() as *const c_char);
                } else if chr_code == 1 {
                    print_esc_cstr(b"gdef\0".as_ptr() as *const c_char);
                } else if chr_code == 2 {
                    print_esc_cstr(b"edef\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"xdef\0".as_ptr() as *const c_char);
                }
            }
            LET => {
                if chr_code != NORMAL as i32 {
                    print_esc_cstr(b"futurelet\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"let\0".as_ptr() as *const c_char);
                }
            }
            SHORTHAND_DEF => match (chr_code) as i32 {
                CHAR_DEF_CODE => print_esc_cstr(b"chardef\0".as_ptr() as *const c_char),
                MATH_CHAR_DEF_CODE => print_esc_cstr(b"mathchardef\0".as_ptr() as *const c_char),
                XETEX_MATH_CHAR_DEF_CODE => {
                    print_esc_cstr(b"Umathchardef\0".as_ptr() as *const c_char)
                }
                XETEX_MATH_CHAR_NUM_DEF_CODE => {
                    print_esc_cstr(b"Umathcharnumdef\0".as_ptr() as *const c_char)
                }
                COUNT_DEF_CODE => print_esc_cstr(b"countdef\0".as_ptr() as *const c_char),
                DIMEN_DEF_CODE => print_esc_cstr(b"dimendef\0".as_ptr() as *const c_char),
                SKIP_DEF_CODE => print_esc_cstr(b"skipdef\0".as_ptr() as *const c_char),
                MU_SKIP_DEF_CODE => print_esc_cstr(b"muskipdef\0".as_ptr() as *const c_char),
                CHAR_SUB_DEF_CODE => print_esc_cstr(b"charsubdef\0".as_ptr() as *const c_char),
                _ => print_esc_cstr(b"toksdef\0".as_ptr() as *const c_char),
            },
            CHAR_GIVEN => {
                print_esc_cstr(b"char\0".as_ptr() as *const c_char);
                print_hex(chr_code);
            }
            MATH_GIVEN => {
                print_esc_cstr(b"mathchar\0".as_ptr() as *const c_char);
                print_hex(chr_code);
            }
            XETEX_MATH_GIVEN => {
                print_esc_cstr(b"Umathchar\0".as_ptr() as *const c_char);
                print_hex((math_class((chr_code) as u32)) as i32);
                print_hex((math_fam((chr_code) as u32)) as i32);
                print_hex((math_char((chr_code) as u32)) as i32);
            }
            DEF_CODE => {
                if chr_code == CAT_CODE_BASE {
                    print_esc_cstr(b"catcode\0".as_ptr() as *const c_char);
                } else if chr_code == MATH_CODE_BASE {
                    print_esc_cstr(b"mathcode\0".as_ptr() as *const c_char);
                } else if chr_code == LC_CODE_BASE {
                    print_esc_cstr(b"lccode\0".as_ptr() as *const c_char);
                } else if chr_code == UC_CODE_BASE {
                    print_esc_cstr(b"uccode\0".as_ptr() as *const c_char);
                } else if chr_code == SF_CODE_BASE {
                    print_esc_cstr(b"sfcode\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"delcode\0".as_ptr() as *const c_char);
                }
            }
            XETEX_DEF_CODE => {
                if chr_code == SF_CODE_BASE {
                    print_esc_cstr(b"XeTeXcharclass\0".as_ptr() as *const c_char);
                } else if chr_code == MATH_CODE_BASE {
                    print_esc_cstr(b"Umathcodenum\0".as_ptr() as *const c_char);
                } else if chr_code == (MATH_CODE_BASE + 1) {
                    print_esc_cstr(b"Umathcode\0".as_ptr() as *const c_char);
                } else if chr_code == DEL_CODE_BASE {
                    print_esc_cstr(b"Udelcodenum\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"Udelcode\0".as_ptr() as *const c_char);
                }
            }
            DEF_FAMILY => {
                print_size(chr_code - MATH_FONT_BASE);
            }
            HYPH_DATA => {
                if chr_code == 1 {
                    print_esc_cstr(b"patterns\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"hyphenation\0".as_ptr() as *const c_char);
                }
            }
            ASSIGN_FONT_INT => match (chr_code) as i32 {
                0 => print_esc_cstr(b"hyphenchar\0".as_ptr() as *const c_char),
                1 => print_esc_cstr(b"skewchar\0".as_ptr() as *const c_char),
                LP_CODE_BASE => print_esc_cstr(b"lpcode\0".as_ptr() as *const c_char),
                RP_CODE_BASE => print_esc_cstr(b"rpcode\0".as_ptr() as *const c_char),
                _ => {}
            },
            SET_FONT => {
                print_cstr(b"select font \0".as_ptr() as *const c_char);
                font_name_str = *font_name.offset(chr_code as isize);
                if *font_area.offset(chr_code as isize) == (AAT_FONT_FLAG) as i32
                    || *font_area.offset(chr_code as isize) == (OTGR_FONT_FLAG) as i32
                {
                    let for_end = length(font_name_str) - 1;
                    quote_char = b'"' as UTF16_code;
                    for n_idx in 0..=for_end {
                        if *str_pool.offset(
                            (*str_start.offset((font_name_str - 65536) as isize) + n_idx) as isize,
                        ) == b'"' as UTF16_code
                        {
                            quote_char = b'\'' as UTF16_code;
                        }
                    }
                    print_char(quote_char as i32);
                    print(font_name_str);
                    print_char(quote_char as i32);
                } else {
                    print(font_name_str);
                }
                if *font_size.offset(chr_code as isize) != *font_dsize.offset(chr_code as isize) {
                    print_cstr(b" at \0".as_ptr() as *const c_char);
                    print_scaled(*font_size.offset(chr_code as isize));
                    print_cstr(b"pt\0".as_ptr() as *const c_char);
                }
            }
            SET_INTERACTION => match (chr_code) as i32 {
                BATCH_MODE => print_esc_cstr(b"batchmode\0".as_ptr() as *const c_char),
                NONSTOP_MODE => print_esc_cstr(b"nonstopmode\0".as_ptr() as *const c_char),
                SCROLL_MODE => print_esc_cstr(b"scrollmode\0".as_ptr() as *const c_char),
                _ => print_esc_cstr(b"errorstopmode\0".as_ptr() as *const c_char),
            },
            IN_STREAM => {
                if chr_code == 0 {
                    print_esc_cstr(b"closein\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"openin\0".as_ptr() as *const c_char);
                }
            }
            MESSAGE => {
                if chr_code == 0 {
                    print_esc_cstr(b"message\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"errmessage\0".as_ptr() as *const c_char);
                }
            }
            CASE_SHIFT => {
                if chr_code == LC_CODE_BASE {
                    print_esc_cstr(b"lowercase\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"uppercase\0".as_ptr() as *const c_char);
                }
            }
            XRAY => match (chr_code) as i32 {
                SHOW_BOX_CODE => print_esc_cstr(b"showbox\0".as_ptr() as *const c_char),
                SHOW_THE_CODE => print_esc_cstr(b"showthe\0".as_ptr() as *const c_char),
                SHOW_LISTS => print_esc_cstr(b"showlists\0".as_ptr() as *const c_char),
                SHOW_GROUPS => print_esc_cstr(b"showgroups\0".as_ptr() as *const c_char),
                SHOW_TOKENS => print_esc_cstr(b"showtokens\0".as_ptr() as *const c_char),
                SHOW_IFS => print_esc_cstr(b"showifs\0".as_ptr() as *const c_char),
                _ => print_esc_cstr(b"show\0".as_ptr() as *const c_char),
            },
            UNDEFINED_CS => {
                print_cstr(b"undefined\0".as_ptr() as *const c_char);
            }
            CALL | LONG_CALL | OUTER_CALL | LONG_OUTER_CALL => {
                n = cmd as i32 - CALL as i32;
                if mem!(mem!(chr_code).b32.s1).b32.s0 == PROTECTED_TOKEN {
                    n += 4;
                }
                if ((n / 4) & 1) != 0 {
                    print_esc_cstr(b"protected\0".as_ptr() as *const c_char);
                }
                if (n & 1) != 0 {
                    print_esc_cstr(b"long\0".as_ptr() as *const c_char);
                }
                if ((n / 2) & 1) != 0 {
                    print_esc_cstr(b"outer\0".as_ptr() as *const c_char);
                }
                if n > 0 {
                    print_char(b' ' as i32);
                }
                print_cstr(b"macro\0".as_ptr() as *const c_char);
            }
            END_TEMPLATE => {
                print_esc_cstr(b"outer endtemplate\0".as_ptr() as *const c_char);
            }
            EXTENSION => match (chr_code) as i32 {
                value if value == OPEN_NODE as i32 => {
                    print_esc_cstr(b"openout\0".as_ptr() as *const c_char)
                }
                value if value == WRITE_NODE as i32 => {
                    print_esc_cstr(b"write\0".as_ptr() as *const c_char)
                }
                value if value == CLOSE_NODE as i32 => {
                    print_esc_cstr(b"closeout\0".as_ptr() as *const c_char)
                }
                value if value == SPECIAL_NODE as i32 => {
                    print_esc_cstr(b"special\0".as_ptr() as *const c_char)
                }
                IMMEDIATE_CODE => print_esc_cstr(b"immediate\0".as_ptr() as *const c_char),
                SET_LANGUAGE_CODE => print_esc_cstr(b"setlanguage\0".as_ptr() as *const c_char),
                value if value == PDF_SAVE_POS_NODE as i32 => {
                    print_esc_cstr(b"pdfsavepos\0".as_ptr() as *const c_char)
                }
                RESET_TIMER_CODE => print_esc_cstr(b"resettimer\0".as_ptr() as *const c_char),
                SET_RANDOM_SEED_CODE => {
                    print_esc_cstr(b"setrandomseed\0".as_ptr() as *const c_char)
                }
                PIC_FILE_CODE => print_esc_cstr(b"XeTeXpicfile\0".as_ptr() as *const c_char),
                PDF_FILE_CODE => print_esc_cstr(b"XeTeXpdffile\0".as_ptr() as *const c_char),
                GLYPH_CODE => print_esc_cstr(b"XeTeXglyph\0".as_ptr() as *const c_char),
                XETEX_LINEBREAK_LOCALE_EXTENSION_CODE => {
                    print_esc_cstr(b"XeTeXlinebreaklocale\0".as_ptr() as *const c_char)
                }
                XETEX_INPUT_ENCODING_EXTENSION_CODE => {
                    print_esc_cstr(b"XeTeXinputencoding\0".as_ptr() as *const c_char)
                }
                XETEX_DEFAULT_ENCODING_EXTENSION_CODE => {
                    print_esc_cstr(b"XeTeXdefaultencoding\0".as_ptr() as *const c_char)
                }
                _ => print_cstr(b"[unknown extension!]\0".as_ptr() as *const c_char),
            },
            _ => {
                print_cstr(b"[unknown command code!]\0".as_ptr() as *const c_char);
            }
        }
    }
}

#[unsafe(export_name = "not_aat_font_error")]
pub unsafe extern "C" fn not_aat_font_error(cmd: i32, c: i32, f: i32) {
    unsafe {
        error_here_with_diagnostic(b"Cannot use \0".as_ptr() as *const c_char);
        print_cmd_chr(cmd as u16, c);
        print_cstr(b" with \0".as_ptr() as *const c_char);
        print(*font_name.offset(f as isize));
        print_cstr(b"; not an AAT font\0".as_ptr() as *const c_char);
        capture_to_diagnostic(std::ptr::null_mut());
        error();
    }
}

#[unsafe(export_name = "not_aat_gr_font_error")]
pub unsafe extern "C" fn not_aat_gr_font_error(cmd: i32, c: i32, f: i32) {
    unsafe {
        error_here_with_diagnostic(b"Cannot use \0".as_ptr() as *const c_char);
        print_cmd_chr(cmd as u16, c);
        print_cstr(b" with \0".as_ptr() as *const c_char);
        print(*font_name.offset(f as isize));
        print_cstr(b"; not an AAT or Graphite font\0".as_ptr() as *const c_char);
        capture_to_diagnostic(std::ptr::null_mut());
        error();
    }
}

#[unsafe(export_name = "not_ot_font_error")]
pub unsafe extern "C" fn not_ot_font_error(cmd: i32, c: i32, f: i32) {
    unsafe {
        error_here_with_diagnostic(b"Cannot use \0".as_ptr() as *const c_char);
        print_cmd_chr(cmd as u16, c);
        print_cstr(b" with \0".as_ptr() as *const c_char);
        print(*font_name.offset(f as isize));
        print_cstr(b"; not an OpenType Layout font\0".as_ptr() as *const c_char);
        capture_to_diagnostic(std::ptr::null_mut());
        error();
    }
}

#[unsafe(export_name = "not_native_font_error")]
pub unsafe extern "C" fn not_native_font_error(cmd: i32, c: i32, f: i32) {
    unsafe {
        error_here_with_diagnostic(b"Cannot use \0".as_ptr() as *const c_char);
        print_cmd_chr(cmd as u16, c);
        print_cstr(b" with \0".as_ptr() as *const c_char);
        print(*font_name.offset(f as isize));
        print_cstr(b"; not a native platform font\0".as_ptr() as *const c_char);
        capture_to_diagnostic(std::ptr::null_mut());
        error();
    }
}

const CONTROL_SEQUENCE_CACHE_SIZE: usize = 32768;
static mut control_sequence_cache: [i32; CONTROL_SEQUENCE_CACHE_SIZE] =
    [0; CONTROL_SEQUENCE_CACHE_SIZE];

pub(super) unsafe fn reset_control_sequence_cache() {
    unsafe {
        std::ptr::write_bytes(
            (&raw mut control_sequence_cache).cast::<i32>(),
            0,
            CONTROL_SEQUENCE_CACHE_SIZE,
        );
    }
}

#[unsafe(export_name = "id_lookup")]
pub unsafe extern "C" fn id_lookup(j: i32, l: i32) -> i32 {
    unsafe {
        use std::hash::BuildHasher;
        let text = std::slice::from_raw_parts(buffer.add(j as usize), l as usize);
        let slot =
            rustc_hash::FxBuildHasher.hash_one(text) as usize & (CONTROL_SEQUENCE_CACHE_SIZE - 1);
        let cached = (&raw const control_sequence_cache)
            .cast::<i32>()
            .add(slot)
            .read();
        if cached != 0 {
            let string = (*hash.add(cached as usize)).s1;
            if string >= TOO_BIG_CHAR && super::xetex_stringpool::str_eq_buf_bmp(string, j, l) {
                return cached;
            }
        }
        id_lookup_uncached(j, l, slot)
    }
}

#[inline(never)]
unsafe fn id_lookup_uncached(j: i32, l: i32, slot: usize) -> i32 {
    unsafe {
        let mut h: i32 = 0;
        let mut d: i32;
        let mut p: i32;
        let mut k: i32;
        let mut ll = l;

        for k_idx in j..=(j + l - 1) {
            let scalar = *buffer.offset(k_idx as isize);
            ll += i32::from(scalar >= 65536);
            h = h + h + scalar;
            while h >= HASH_PRIME {
                h -= 8501;
            }
        }

        p = h + HASH_BASE;

        loop {
            if (*hash.offset(p as isize)).s1 > 0 {
                if length((*hash.offset(p as isize)).s1) == ll {
                    let string = (*hash.offset(p as isize)).s1;
                    let matched = if ll == l {
                        super::xetex_stringpool::str_eq_buf_bmp(string, j, l)
                    } else {
                        str_eq_buf(string, j)
                    };
                    if matched {
                        (&raw mut control_sequence_cache)
                            .cast::<i32>()
                            .add(slot)
                            .write(p);
                        return p;
                    }
                }
            }

            if (*hash.offset(p as isize)).s0 == 0 {
                if no_new_control_sequence {
                    p = UNDEFINED_CONTROL_SEQUENCE;
                } else {
                    if (*hash.offset(p as isize)).s1 > 0 {
                        if hash_high < hash_extra {
                            hash_high += 1;
                            (*hash.offset(p as isize)).s0 = hash_high + EQTB_SIZE;
                            p = hash_high + EQTB_SIZE;
                        } else {
                            loop {
                                if hash_used == HASH_BASE {
                                    overflow(
                                        b"hash size\0".as_ptr() as *const c_char,
                                        ((HASH_SIZE + hash_extra) as usize) as i32,
                                    );
                                }
                                hash_used -= 1;
                                if (*hash.offset(hash_used as isize)).s1 == 0 {
                                    break;
                                }
                            }
                            (*hash.offset(p as isize)).s0 = hash_used;
                            p = hash_used;
                        }
                    }

                    if pool_ptr + ll > pool_size {
                        overflow(
                            b"pool size\0".as_ptr() as *const c_char,
                            ((pool_size - init_pool_ptr) as usize) as i32,
                        );
                    }

                    d = cur_length();

                    while pool_ptr > *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) {
                        pool_ptr -= 1;
                        *str_pool.offset((pool_ptr + l) as isize) =
                            *str_pool.offset(pool_ptr as isize);
                    }

                    for k_idx in j..=(j + l - 1) {
                        let b = *buffer.offset(k_idx as isize);
                        if b < 65536 {
                            *str_pool.offset(pool_ptr as isize) = b as packed_UTF16_code;
                            pool_ptr += 1;
                        } else {
                            *str_pool.offset(pool_ptr as isize) =
                                (0xD800 + (b - 65536) / 1024) as packed_UTF16_code;
                            pool_ptr += 1;
                            *str_pool.offset(pool_ptr as isize) =
                                (0xDC00 + (b - 65536) % 1024) as packed_UTF16_code;
                            pool_ptr += 1;
                        }
                    }

                    (*hash.offset(p as isize)).s1 = make_string();
                    pool_ptr += d;
                    (&raw mut control_sequence_cache)
                        .cast::<i32>()
                        .add(slot)
                        .write(p);
                }
                return p;
            }

            p = (*hash.offset(p as isize)).s0 as i32;
        }
    }
}

#[unsafe(export_name = "prim_lookup")]
pub unsafe extern "C" fn prim_lookup(s: str_number) -> i32 {
    unsafe {
        let mut h: i32;
        let mut p: i32;
        let mut k: i32;
        let mut j: i32;
        let mut l: i32 = 0;

        if s <= BIGGEST_CHAR {
            if s < 0 {
                return UNDEFINED_PRIMITIVE;
            } else {
                p = (s % PRIM_PRIME) + 1;
            }
        } else {
            j = *str_start.offset((s - 65536) as isize);
            if s == str_ptr {
                l = cur_length();
            } else {
                l = length(s);
            }
            h = *str_pool.offset(j as isize) as i32;
            k = j + 1;
            let for_end = j + l - 1;
            while k <= for_end {
                h = h + h + *str_pool.offset(k as isize) as i32;
                while h >= PRIM_PRIME {
                    h -= PRIM_PRIME;
                }
                k += 1;
            }
            p = h + 1;
        }

        loop {
            if (*(&raw mut prim).cast::<b32x2>().offset(p as isize)).s1 > 65536 {
                if length((*(&raw mut prim).cast::<b32x2>().offset(p as isize)).s1) - 1 == l {
                    if str_eq_str(
                        (*(&raw mut prim).cast::<b32x2>().offset(p as isize)).s1 - 1,
                        s,
                    ) {
                        return p;
                    }
                }
            } else if (*(&raw mut prim).cast::<b32x2>().offset(p as isize)).s1 == 1 + s {
                return p;
            }

            if (*(&raw mut prim).cast::<b32x2>().offset(p as isize)).s0 == 0 {
                if no_new_control_sequence {
                    p = UNDEFINED_PRIMITIVE;
                } else {
                    if (*(&raw mut prim).cast::<b32x2>().offset(p as isize)).s1 > 0 {
                        loop {
                            if prim_used == PRIM_BASE {
                                overflow(
                                    b"primitive size\0".as_ptr() as *const c_char,
                                    (PRIM_SIZE as usize) as i32,
                                );
                            }
                            prim_used -= 1;
                            if (*(&raw mut prim).cast::<b32x2>().offset(prim_used as isize)).s1 == 0
                            {
                                break;
                            }
                        }
                        (*(&raw mut prim).cast::<b32x2>().offset(p as isize)).s0 =
                            (prim_used as u16) as i32;
                        p = prim_used;
                    }
                    (*(&raw mut prim).cast::<b32x2>().offset(p as isize)).s1 = s + 1;
                }
                return p;
            }
            p = (*(&raw mut prim).cast::<b32x2>().offset(p as isize)).s0 as i32;
        }
    }
}

#[unsafe(export_name = "print_group")]
pub unsafe extern "C" fn print_group(e: bool) {
    unsafe {
        match (cur_group) as i32 {
            BOTTOM_LEVEL => {
                print_cstr(b"bottom level\0".as_ptr() as *const c_char);
                return;
            }
            SIMPLE_GROUP | SEMI_SIMPLE_GROUP => {
                if cur_group == (SEMI_SIMPLE_GROUP) as u8 {
                    print_cstr(b"semi \0".as_ptr() as *const c_char);
                }
                print_cstr(b"simple\0".as_ptr() as *const c_char);
            }
            HBOX_GROUP | ADJUSTED_HBOX_GROUP => {
                if cur_group == (ADJUSTED_HBOX_GROUP) as u8 {
                    print_cstr(b"adjusted \0".as_ptr() as *const c_char);
                }
                print_cstr(b"hbox\0".as_ptr() as *const c_char);
            }
            VBOX_GROUP => print_cstr(b"vbox\0".as_ptr() as *const c_char),
            VTOP_GROUP => print_cstr(b"vtop\0".as_ptr() as *const c_char),
            ALIGN_GROUP | NO_ALIGN_GROUP => {
                if cur_group == (NO_ALIGN_GROUP) as u8 {
                    print_cstr(b"no \0".as_ptr() as *const c_char);
                }
                print_cstr(b"align\0".as_ptr() as *const c_char);
            }
            OUTPUT_GROUP => print_cstr(b"output\0".as_ptr() as *const c_char),
            DISC_GROUP => print_cstr(b"disc\0".as_ptr() as *const c_char),
            INSERT_GROUP => print_cstr(b"insert\0".as_ptr() as *const c_char),
            VCENTER_GROUP => print_cstr(b"vcenter\0".as_ptr() as *const c_char),
            MATH_GROUP | MATH_CHOICE_GROUP | MATH_SHIFT_GROUP | MATH_LEFT_GROUP => {
                print_cstr(b"math\0".as_ptr() as *const c_char);
                if cur_group == (MATH_CHOICE_GROUP) as u8 {
                    print_cstr(b" choice\0".as_ptr() as *const c_char);
                } else if cur_group == (MATH_SHIFT_GROUP) as u8 {
                    print_cstr(b" shift\0".as_ptr() as *const c_char);
                } else if cur_group == (MATH_LEFT_GROUP) as u8 {
                    print_cstr(b" left\0".as_ptr() as *const c_char);
                }
            }
            _ => {}
        }
        print_cstr(b" group (level \0".as_ptr() as *const c_char);
        print_int(cur_level as i32);
        print_char(b')' as i32);
        if (*save_stack.offset((save_ptr - 1) as isize)).b32.s1 != 0 {
            if e {
                print_cstr(b" entered at line \0".as_ptr() as *const c_char);
            } else {
                print_cstr(b" at line \0".as_ptr() as *const c_char);
            }
            print_int((*save_stack.offset((save_ptr - 1) as isize)).b32.s1);
        }
    }
}

#[unsafe(export_name = "pseudo_input")]
pub unsafe extern "C" fn pseudo_input() -> bool {
    unsafe {
        let p = mem!(pseudo_files).b32.s0;
        if p == TEX_NULL {
            return false;
        }
        mem!(pseudo_files).b32.s0 = mem!(p).b32.s1;
        let sz = mem!(p).b32.s0;
        if 4 * sz - 3 >= buf_size - last {
            cur_input.loc = first;
            cur_input.limit = last - 1;
            overflow(
                b"buffer size\0".as_ptr() as *const c_char,
                (buf_size as usize) as i32,
            );
        }
        last = first;
        let mut r = p + 1;
        let for_end = p + sz - 1;
        while r <= for_end {
            let w = mem!(r).b16;
            *buffer.offset(last as isize) = w.s3 as UnicodeScalar;
            *buffer.offset((last + 1) as isize) = w.s2 as UnicodeScalar;
            *buffer.offset((last + 2) as isize) = w.s1 as UnicodeScalar;
            *buffer.offset((last + 3) as isize) = w.s0 as UnicodeScalar;
            last += 4;
            r += 1;
        }
        if last >= max_buf_stack {
            max_buf_stack = last + 1;
        }
        while last > first && *buffer.offset((last - 1) as isize) == b' ' as UnicodeScalar {
            last -= 1;
        }
        free_node(p, sz);
        true
    }
}

#[unsafe(export_name = "pseudo_close")]
pub unsafe extern "C" fn pseudo_close() {
    unsafe {
        let p = mem!(pseudo_files).b32.s1;
        let mut q = mem!(pseudo_files).b32.s0;
        mem!(pseudo_files).b32.s1 = avail;
        avail = pseudo_files;
        pseudo_files = p;
        while q != TEX_NULL {
            let next_p = q;
            q = mem!(next_p).b32.s1;
            free_node(next_p, mem!(next_p).b32.s0);
        }
    }
}

#[unsafe(export_name = "group_warning")]
pub unsafe extern "C" fn group_warning() {
    unsafe {
        let mut i: i32;
        let mut w: bool;

        base_ptr = input_ptr;
        *input_stack.offset(base_ptr as isize) = cur_input;
        i = in_open;
        w = false;
        while *grp_stack.offset(i as isize) == cur_boundary && i > 0 {
            if INTPAR!(INT_PAR__tracing_nesting) > 0 {
                while (*input_stack.offset(base_ptr as isize)).state == (TOKEN_LIST) as u16
                    || (*input_stack.offset(base_ptr as isize)).index as i32 > i
                {
                    base_ptr -= 1;
                }
                if (*input_stack.offset(base_ptr as isize)).name > 17 {
                    w = true;
                }
            }
            *grp_stack.offset(i as isize) = (*save_stack.offset(save_ptr as isize)).b32.s1;
            i -= 1;
        }
        if w {
            print_nl_cstr(b"Warning: \0".as_ptr() as *const c_char);
            diagnostic_begin_capture_warning_here();
            print_cstr(b"end of \0".as_ptr() as *const c_char);
            print_group(true);
            print_cstr(b" of a different file\0".as_ptr() as *const c_char);
            print_ln();
            if INTPAR!(INT_PAR__tracing_nesting) > 1 {
                show_context();
            }
            if history == HISTORY_SPOTLESS {
                history = HISTORY_WARNING_ISSUED;
            }
            capture_to_diagnostic(std::ptr::null_mut());
        }
    }
}

#[unsafe(export_name = "if_warning")]
pub unsafe extern "C" fn if_warning() {
    unsafe {
        let mut i: i32;
        let mut w: bool;

        base_ptr = input_ptr;
        *input_stack.offset(base_ptr as isize) = cur_input;
        i = in_open;
        w = false;
        while *if_stack.offset(i as isize) == cond_ptr {
            if INTPAR!(INT_PAR__tracing_nesting) > 0 {
                while (*input_stack.offset(base_ptr as isize)).state == (TOKEN_LIST) as u16
                    || (*input_stack.offset(base_ptr as isize)).index as i32 > i
                {
                    base_ptr -= 1;
                }
                if (*input_stack.offset(base_ptr as isize)).name > 17 {
                    w = true;
                }
            }
            *if_stack.offset(i as isize) = mem!(cond_ptr).b32.s1;
            i -= 1;
        }
        if w {
            print_nl_cstr(b"Warning: \0".as_ptr() as *const c_char);
            diagnostic_begin_capture_warning_here();
            print_cstr(b"end of \0".as_ptr() as *const c_char);
            print_cmd_chr((IF_TEST) as u16, cur_if as i32);
            if if_line != 0 {
                print_cstr(b" entered on line \0".as_ptr() as *const c_char);
                print_int(if_line);
            }
            print_cstr(b" of a different file\0".as_ptr() as *const c_char);
            print_ln();
            if INTPAR!(INT_PAR__tracing_nesting) > 1 {
                show_context();
            }
            capture_to_diagnostic(std::ptr::null_mut());
            if history == HISTORY_SPOTLESS {
                history = HISTORY_WARNING_ISSUED;
            }
        }
    }
}

#[unsafe(export_name = "file_warning")]
pub unsafe extern "C" fn file_warning() {
    unsafe {
        let p = save_ptr;
        let l = cur_level;
        let c = cur_group;
        save_ptr = cur_boundary;
        while *grp_stack.offset(in_open as isize) != save_ptr {
            cur_level -= 1;
            print_nl_cstr(b"Warning: \0".as_ptr() as *const c_char);
            diagnostic_begin_capture_warning_here();
            print_cstr(b"end of file when \0".as_ptr() as *const c_char);
            print_group(true);
            print_cstr(b" is incomplete\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());
            cur_group = (*save_stack.offset(save_ptr as isize)).b16.s0 as group_code;
            save_ptr = (*save_stack.offset(save_ptr as isize)).b32.s1;
        }
        save_ptr = p;
        cur_level = l;
        cur_group = c;
        let p_cond = cond_ptr;
        let l_cond = if_limit;
        let c_cond = cur_if;
        let i_cond = if_line;
        while *if_stack.offset(in_open as isize) != cond_ptr {
            print_nl_cstr(b"Warning: \0".as_ptr() as *const c_char);
            diagnostic_begin_capture_warning_here();
            print_cstr(b"end of file when \0".as_ptr() as *const c_char);
            print_cmd_chr((IF_TEST) as u16, cur_if as i32);
            if if_limit == (FI_CODE as u16) as u8 {
                print_esc_cstr(b"else\0".as_ptr() as *const c_char);
            }
            if if_line != 0 {
                print_cstr(b" entered on line \0".as_ptr() as *const c_char);
                print_int(if_line);
            }
            print_cstr(b" is incomplete\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());
            if_line = mem!(cond_ptr + 1).b32.s1;
            cur_if = (mem!(cond_ptr).b16.s0 as u16) as i16;
            if_limit = (mem!(cond_ptr).b16.s1) as u8;
            cond_ptr = LLIST_link!(cond_ptr);
        }
        cond_ptr = p_cond;
        if_limit = l_cond;
        cur_if = c_cond;
        if_line = i_cond;
        print_ln();
        if INTPAR!(INT_PAR__tracing_nesting) > 1 {
            diagnostic_begin_capture_warning_here();
            show_context();
            capture_to_diagnostic(std::ptr::null_mut());
        }
        if history == HISTORY_SPOTLESS {
            history = HISTORY_WARNING_ISSUED;
        }
    }
}

#[unsafe(export_name = "delete_sa_ref")]
pub unsafe extern "C" fn delete_sa_ref(mut q: i32) {
    unsafe {
        let mut p: i32;
        let mut i: small_number;
        let mut s: small_number;

        mem!(q + 1).b32.s0 -= 1;
        if mem!(q + 1).b32.s0 != TEX_NULL {
            return;
        }
        if mem!(q).b16.s1 < DIMEN_VAL_LIMIT as u16 {
            if mem!(q + 2).b32.s1 == 0 {
                s = WORD_NODE_SIZE as small_number;
            } else {
                return;
            }
        } else {
            if mem!(q).b16.s1 < MU_VAL_LIMIT as u16 {
                if mem!(q + 1).b32.s1 == 0 {
                    delete_glue_ref(0);
                } else {
                    return;
                }
            } else if mem!(q + 1).b32.s1 != TEX_NULL {
                return;
            }
            s = POINTER_NODE_SIZE as small_number;
        }

        loop {
            i = (mem!(q).b16.s1 % 64) as small_number;
            p = q;
            q = mem!(p).b32.s1;
            free_node(p, s as i32);
            if q == TEX_NULL {
                *(&raw mut sa_root).cast::<i32>().offset(i as isize) = TEX_NULL;
                return;
            }
            if (i & 1) != 0 {
                mem!(q + (i as i32 / 2) + 1).b32.s1 = TEX_NULL;
            } else {
                mem!(q + (i as i32 / 2) + 1).b32.s0 = TEX_NULL;
            }
            mem!(q).b16.s0 -= 1;
            s = INDEX_NODE_SIZE as small_number;
            if mem!(q).b16.s0 > 0 {
                break;
            }
        }
    }
}

#[unsafe(export_name = "sa_save")]
pub unsafe extern "C" fn sa_save(p: i32) {
    unsafe {
        let q: i32;
        let mut i: u16;

        if cur_level != sa_level {
            if save_ptr > max_save_stack {
                max_save_stack = save_ptr;
                if max_save_stack > save_size - 7 {
                    overflow(
                        b"save size\0".as_ptr() as *const c_char,
                        (save_size as usize) as i32,
                    );
                }
            }
            (*save_stack.offset(save_ptr as isize)).b16.s1 = (RESTORE_SA) as u16;
            (*save_stack.offset(save_ptr as isize)).b16.s0 = sa_level;
            (*save_stack.offset(save_ptr as isize)).b32.s1 = sa_chain;
            save_ptr += 1;
            sa_chain = TEX_NULL;
            sa_level = cur_level;
        }
        i = mem!(p).b16.s1;
        if i < DIMEN_VAL_LIMIT as u16 {
            if mem!(p + 2).b32.s1 == 0 {
                q = get_node(POINTER_NODE_SIZE);
                i = TOK_VAL_LIMIT as u16;
            } else {
                q = get_node(WORD_NODE_SIZE);
                mem!(q + 2).b32.s1 = mem!(p + 2).b32.s1;
            }
            mem!(q + 1).b32.s1 = TEX_NULL;
        } else {
            q = get_node(POINTER_NODE_SIZE);
            mem!(q + 1).b32.s1 = mem!(p + 1).b32.s1;
        }
        mem!(q + 1).b32.s0 = p;
        mem!(q).b16.s1 = i;
        mem!(q).b16.s0 = mem!(p).b16.s0;
        mem!(q).b32.s1 = sa_chain;
        sa_chain = q;
        mem!(p + 1).b32.s0 += 1;
    }
}

#[unsafe(export_name = "sa_destroy")]
pub unsafe extern "C" fn sa_destroy(p: i32) {
    unsafe {
        if mem!(p).b16.s1 < MU_VAL_LIMIT as u16 {
            delete_glue_ref(mem!(p + 1).b32.s1);
        } else if mem!(p + 1).b32.s1 != TEX_NULL {
            if mem!(p).b16.s1 < BOX_VAL_LIMIT as u16 {
                flush_node_list(mem!(p + 1).b32.s1);
            } else {
                delete_token_ref(mem!(p + 1).b32.s1);
            }
        }
    }
}

#[unsafe(export_name = "sa_def")]
pub unsafe extern "C" fn sa_def(p: i32, e: i32) {
    unsafe {
        mem!(p + 1).b32.s0 += 1;
        if mem!(p + 1).b32.s1 == e {
            sa_destroy(p);
        } else {
            if mem!(p).b16.s0 == cur_level {
                sa_destroy(p);
            } else {
                sa_save(p);
            }
            mem!(p).b16.s0 = cur_level;
            mem!(p + 1).b32.s1 = e;
        }
        delete_sa_ref(p);
    }
}

#[unsafe(export_name = "sa_w_def")]
pub unsafe extern "C" fn sa_w_def(p: i32, w: i32) {
    unsafe {
        mem!(p + 1).b32.s0 += 1;
        if mem!(p + 2).b32.s1 != w {
            if mem!(p).b16.s0 != cur_level {
                sa_save(p);
            }
            mem!(p).b16.s0 = cur_level;
            mem!(p + 2).b32.s1 = w;
        }
        delete_sa_ref(p);
    }
}

#[unsafe(export_name = "gsa_def")]
pub unsafe extern "C" fn gsa_def(p: i32, e: i32) {
    unsafe {
        mem!(p + 1).b32.s0 += 1;
        sa_destroy(p);
        mem!(p).b16.s0 = (LEVEL_ONE) as u16;
        mem!(p + 1).b32.s1 = e;
        delete_sa_ref(p);
    }
}

#[unsafe(export_name = "gsa_w_def")]
pub unsafe extern "C" fn gsa_w_def(p: i32, w: i32) {
    unsafe {
        mem!(p + 1).b32.s0 += 1;
        mem!(p).b16.s0 = (LEVEL_ONE) as u16;
        mem!(p + 2).b32.s1 = w;
        delete_sa_ref(p);
    }
}

#[unsafe(export_name = "sa_restore")]
pub unsafe extern "C" fn sa_restore() {
    unsafe {
        let mut p: i32;

        loop {
            p = mem!(sa_chain + 1).b32.s0;
            if mem!(p).b16.s0 == (LEVEL_ONE) as u16 {
                if mem!(p).b16.s1 >= DIMEN_VAL_LIMIT as u16 {
                    sa_destroy(sa_chain);
                }
            } else {
                if mem!(p).b16.s1 < DIMEN_VAL_LIMIT as u16 {
                    if mem!(sa_chain).b16.s1 < DIMEN_VAL_LIMIT as u16 {
                        mem!(p + 2).b32.s1 = mem!(sa_chain + 2).b32.s1;
                    } else {
                        mem!(p + 2).b32.s1 = 0;
                    }
                } else {
                    sa_destroy(p);
                    mem!(p + 1).b32.s1 = mem!(sa_chain + 1).b32.s1;
                }
                mem!(p).b16.s0 = mem!(sa_chain).b16.s0;
            }
            delete_sa_ref(p);
            p = sa_chain;
            sa_chain = mem!(p).b32.s1;
            if mem!(p).b16.s1 < DIMEN_VAL_LIMIT as u16 {
                free_node(p, WORD_NODE_SIZE);
            } else {
                free_node(p, POINTER_NODE_SIZE);
            }
            if sa_chain == TEX_NULL {
                break;
            }
        }
    }
}

#[unsafe(export_name = "new_save_level")]
pub unsafe extern "C" fn new_save_level(c: group_code) {
    unsafe {
        if save_ptr > max_save_stack {
            max_save_stack = save_ptr;
            if max_save_stack > save_size - 7 {
                overflow(
                    b"save size\0".as_ptr() as *const c_char,
                    (save_size as usize) as i32,
                );
            }
        }
        (*save_stack.offset(save_ptr as isize)).b32.s1 = line;
        save_ptr += 1;
        (*save_stack.offset(save_ptr as isize)).b16.s1 = (LEVEL_BOUNDARY) as u16;
        (*save_stack.offset(save_ptr as isize)).b16.s0 = cur_group as u16;
        (*save_stack.offset(save_ptr as isize)).b32.s1 = cur_boundary;
        if cur_level == (u16::MAX as i32) as u16 {
            overflow(
                b"grouping levels\0".as_ptr() as *const c_char,
                ((u16::MAX as i32) as usize) as i32,
            );
        }
        cur_boundary = save_ptr;
        cur_group = c;
        cur_level += 1;
        save_ptr += 1;
    }
}

#[unsafe(export_name = "eq_destroy")]
pub unsafe extern "C" fn eq_destroy(w: memory_word) {
    unsafe {
        let q: i32;
        match (w.b16.s1) as i32 {
            113 | 114 | 115 | 116 => {
                delete_token_ref(w.b32.s1);
            }
            119 => {
                delete_glue_ref(w.b32.s1);
            }
            120 => {
                q = w.b32.s1;
                if q != TEX_NULL {
                    free_node(q, mem!(q).b32.s0 + mem!(q).b32.s0 + 1);
                }
            }
            121 => {
                flush_node_list(w.b32.s1);
            }
            72 | 91 => {
                if w.b32.s1 < 0 || w.b32.s1 > 19 {
                    delete_sa_ref(w.b32.s1);
                }
            }
            _ => {}
        }
    }
}

#[unsafe(export_name = "eq_save")]
pub unsafe extern "C" fn eq_save(p: i32, l: u16) {
    unsafe {
        if save_ptr > max_save_stack {
            max_save_stack = save_ptr;
            if max_save_stack > save_size - 7 {
                overflow(
                    b"save size\0".as_ptr() as *const c_char,
                    (save_size as usize) as i32,
                );
            }
        }
        if l == (LEVEL_ZERO) as u16 {
            (*save_stack.offset(save_ptr as isize)).b16.s1 = (RESTORE_ZERO) as u16;
        } else {
            *save_stack.offset(save_ptr as isize) = eqtb!(p);
            save_ptr += 1;
            (*save_stack.offset(save_ptr as isize)).b16.s1 = (RESTORE_OLD_VALUE) as u16;
        }
        (*save_stack.offset(save_ptr as isize)).b16.s0 = l;
        (*save_stack.offset(save_ptr as isize)).b32.s1 = p;
        save_ptr += 1;
    }
}

#[unsafe(export_name = "eq_define")]
pub unsafe extern "C" fn eq_define(p: i32, t: u16, e: i32) {
    unsafe {
        if eqtb!(p).b16.s1 == t && eqtb!(p).b32.s1 == e {
            eq_destroy(eqtb!(p));
            return;
        }
        if eqtb!(p).b16.s0 == cur_level {
            eq_destroy(eqtb!(p));
        } else if cur_level > (LEVEL_ONE) as u16 {
            eq_save(p, eqtb!(p).b16.s0);
        }
        eqtb!(p).b16.s0 = cur_level;
        eqtb!(p).b16.s1 = t;
        eqtb!(p).b32.s1 = e;
    }
}

#[unsafe(export_name = "eq_word_define")]
pub unsafe extern "C" fn eq_word_define(p: i32, w: i32) {
    unsafe {
        if eqtb!(p).b32.s1 == w {
            return;
        }
        if XEQ_LEVEL!(p) != cur_level {
            eq_save(p, XEQ_LEVEL!(p));
            XEQ_LEVEL!(p) = cur_level;
        }
        eqtb!(p).b32.s1 = w;
    }
}

#[unsafe(export_name = "geq_define")]
pub unsafe extern "C" fn geq_define(p: i32, t: u16, e: i32) {
    unsafe {
        eq_destroy(eqtb!(p));
        eqtb!(p).b16.s0 = (LEVEL_ONE) as u16;
        eqtb!(p).b16.s1 = t;
        eqtb!(p).b32.s1 = e;
    }
}

#[unsafe(export_name = "geq_word_define")]
pub unsafe extern "C" fn geq_word_define(p: i32, w: i32) {
    unsafe {
        eqtb!(p).b32.s1 = w;
        XEQ_LEVEL!(p) = (LEVEL_ONE) as u16;
    }
}

#[unsafe(export_name = "save_for_after")]
pub unsafe extern "C" fn save_for_after(t: i32) {
    unsafe {
        if cur_level > (LEVEL_ONE) as u16 {
            if save_ptr > max_save_stack {
                max_save_stack = save_ptr;
                if max_save_stack > save_size - 7 {
                    overflow(
                        b"save size\0".as_ptr() as *const c_char,
                        (save_size as usize) as i32,
                    );
                }
            }
            (*save_stack.offset(save_ptr as isize)).b16.s1 = (INSERT_TOKEN) as u16;
            (*save_stack.offset(save_ptr as isize)).b16.s0 = (LEVEL_ZERO) as u16;
            (*save_stack.offset(save_ptr as isize)).b32.s1 = t;
            save_ptr += 1;
        }
    }
}

#[unsafe(export_name = "unsave")]
pub unsafe extern "C" fn unsave() {
    unsafe {
        let mut p: i32;
        let mut l: u16 = 0;
        let mut t: i32;
        let mut a: bool = false;

        if cur_level > (LEVEL_ONE) as u16 {
            cur_level -= 1;
            'search: loop {
                save_ptr -= 1;
                if (*save_stack.offset(save_ptr as isize)).b16.s1 == (LEVEL_BOUNDARY) as u16 {
                    break 'search;
                }
                p = (*save_stack.offset(save_ptr as isize)).b32.s1;
                if (*save_stack.offset(save_ptr as isize)).b16.s1 == (INSERT_TOKEN) as u16 {
                    t = cur_tok;
                    cur_tok = p;
                    if a {
                        p = get_avail();
                        mem!(p).b32.s0 = cur_tok;
                        mem!(p).b32.s1 = cur_input.loc;
                        cur_input.loc = p;
                        cur_input.start = p;
                        if cur_tok < RIGHT_BRACE_LIMIT {
                            if cur_tok < LEFT_BRACE_LIMIT {
                                align_state -= 1;
                            } else {
                                align_state += 1;
                            }
                        }
                    } else {
                        back_input();
                        a = true;
                    }
                    cur_tok = t;
                } else if (*save_stack.offset(save_ptr as isize)).b16.s1 == (RESTORE_SA) as u16 {
                    sa_restore();
                    sa_chain = p;
                    sa_level = (*save_stack.offset(save_ptr as isize)).b16.s0;
                } else {
                    if (*save_stack.offset(save_ptr as isize)).b16.s1 == (RESTORE_OLD_VALUE) as u16
                    {
                        l = (*save_stack.offset(save_ptr as isize)).b16.s0;
                        save_ptr -= 1;
                    } else {
                        *save_stack.offset(save_ptr as isize) = eqtb!(UNDEFINED_CONTROL_SEQUENCE);
                    }
                    if p < INT_BASE || p > EQTB_SIZE {
                        if eqtb!(p).b16.s0 == (LEVEL_ONE) as u16 {
                            eq_destroy(*save_stack.offset(save_ptr as isize));
                        } else {
                            eq_destroy(eqtb!(p));
                            eqtb!(p) = *save_stack.offset(save_ptr as isize);
                        }
                    } else if XEQ_LEVEL!(p) != (LEVEL_ONE) as u16 {
                        eqtb!(p) = *save_stack.offset(save_ptr as isize);
                        XEQ_LEVEL!(p) = l;
                    }
                }
            }

            if *grp_stack.offset(in_open as isize) == cur_boundary {
                group_warning();
            }
            cur_group = (*save_stack.offset(save_ptr as isize)).b16.s0 as group_code;
            cur_boundary = (*save_stack.offset(save_ptr as isize)).b32.s1;
            save_ptr -= 1;
        } else {
            confusion(b"curlevel\0".as_ptr() as *const c_char);
        }
    }
}

#[unsafe(export_name = "prepare_mag")]
pub unsafe extern "C" fn prepare_mag() {
    unsafe {
        if mag_set > 0 && INTPAR!(INT_PAR__mag) != mag_set {
            let errmsg = error_here_with_diagnostic(
                b"Incompatible magnification (\0".as_ptr() as *const c_char
            );
            print_int(INTPAR!(INT_PAR__mag));
            print_cstr(b");\0".as_ptr() as *const c_char);
            print_nl_cstr(b" the previous value will be retained\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, mag_set);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"I can handle only one magnification ratio per job. So I've\0".as_ptr()
                    as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"reverted to the magnification you used earlier on this run.\0".as_ptr()
                    as *const c_char;
            int_error(mag_set);
            geq_word_define(INT_BASE + INT_PAR__mag, mag_set);
        }
        if INTPAR!(INT_PAR__mag) <= 0 || INTPAR!(INT_PAR__mag) > 32768 {
            let errmsg = error_here_with_diagnostic(
                b"Illegal magnification has been changed to 1000\0".as_ptr() as *const c_char,
            );
            ttstub_diag_printf(
                errmsg,
                b" (%d)\0".as_ptr() as *const c_char,
                INTPAR!(INT_PAR__mag),
            );
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 1;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"The magnification ratio must be between 1 and 32768.\0".as_ptr() as *const c_char;
            int_error(INTPAR!(INT_PAR__mag));
            geq_word_define(INT_BASE + INT_PAR__mag, 1000);
        }
        mag_set = INTPAR!(INT_PAR__mag);
    }
}

#[unsafe(export_name = "token_show")]
pub unsafe extern "C" fn token_show(p: i32) {
    unsafe {
        if p != TEX_NULL {
            show_token_list(mem!(p).b32.s1, TEX_NULL, 10000000);
        }
    }
}

#[unsafe(export_name = "print_meaning")]
pub unsafe extern "C" fn print_meaning() {
    unsafe {
        print_cmd_chr((cur_cmd) as u16, cur_chr);
        if cur_cmd >= (CALL) as u8 {
            print_char(b':' as i32);
            print_ln();
            token_show(cur_chr);
        } else if cur_cmd == (TOP_BOT_MARK) as u8 && cur_chr < 5 {
            print_char(b':' as i32);
            print_ln();
            token_show(*(&raw mut cur_mark).cast::<i32>().offset(cur_chr as isize));
        }
    }
}

#[unsafe(export_name = "show_cur_cmd_chr")]
pub unsafe extern "C" fn show_cur_cmd_chr() {
    unsafe {
        let mut n: i32;
        let mut l: i32;
        let mut p: i32;

        begin_diagnostic();
        diagnostic_begin_capture_warning_here();
        print_nl(b'{' as i32);
        if cur_list.mode != shown_mode {
            print_mode(cur_list.mode as i32);
            print_cstr(b": \0".as_ptr() as *const c_char);
            shown_mode = cur_list.mode;
        }
        print_cmd_chr((cur_cmd) as u16, cur_chr);
        if INTPAR!(INT_PAR__tracing_ifs) > 0 {
            if cur_cmd >= (IF_TEST) as u8 && cur_cmd <= (FI_OR_ELSE) as u8 {
                print_cstr(b": \0".as_ptr() as *const c_char);
                if cur_cmd == (FI_OR_ELSE) as u8 {
                    print_cmd_chr((IF_TEST) as u16, cur_if as i32);
                    print_char(b' ' as i32);
                    n = 0;
                    l = if_line;
                } else {
                    n = 1;
                    l = line;
                }
                p = cond_ptr;
                while p != TEX_NULL {
                    n += 1;
                    p = LLIST_link!(p);
                }
                print_cstr(b"(level \0".as_ptr() as *const c_char);
                print_int(n);
                print_char(b')' as i32);
                if l != 0 {
                    print_cstr(b" entered on line \0".as_ptr() as *const c_char);
                    print_int(l);
                }
            }
        }
        print_char(b'}' as i32);
        capture_to_diagnostic(std::ptr::null_mut());
        end_diagnostic(false);
    }
}

#[unsafe(export_name = "show_context")]
pub unsafe extern "C" fn show_context() {
    unsafe {
        let mut saved_selector: selector_t;
        let mut nn: i32;
        let mut bottom_line: bool;
        let mut i: i32;
        let mut j: i32;
        let mut l: i32;
        let mut m: i32;
        let mut n: i32;
        let mut p: i32;
        let mut q: i32;

        base_ptr = input_ptr;
        *input_stack.offset(base_ptr as isize) = cur_input;
        nn = -1;
        bottom_line = false;

        loop {
            cur_input = *input_stack.offset(base_ptr as isize);
            if cur_input.state != (TOKEN_LIST) as u16 {
                if cur_input.name > 19 || base_ptr == 0 {
                    bottom_line = true;
                }
            }
            if base_ptr == input_ptr || bottom_line || nn < INTPAR!(INT_PAR__error_context_lines) {
                if base_ptr == input_ptr
                    || cur_input.state != (TOKEN_LIST) as u16
                    || cur_input.index != (BACKED_UP) as u16
                    || cur_input.loc != TEX_NULL
                {
                    tally = 0;
                    saved_selector = selector;
                    if cur_input.state != (TOKEN_LIST) as u16 {
                        if cur_input.name <= 17 {
                            if cur_input.name == 0 {
                                if base_ptr == 0 {
                                    print_nl_cstr(b"<*>\0".as_ptr() as *const c_char);
                                } else {
                                    print_nl_cstr(b"<insert> \0".as_ptr() as *const c_char);
                                }
                            } else {
                                print_nl_cstr(b"<read \0".as_ptr() as *const c_char);
                                if cur_input.name == 17 {
                                    print_char(b'*' as i32);
                                } else {
                                    print_int(cur_input.name - 1);
                                }
                                print_char(b'>' as i32);
                            }
                        } else {
                            print_nl_cstr(b"l.\0".as_ptr() as *const c_char);
                            if cur_input.index as i32 == in_open {
                                print_int(line);
                            } else {
                                print_int(*line_stack.offset((cur_input.index + 1) as isize));
                            }
                        }
                        print_char(b' ' as i32);
                        l = tally;
                        tally = 0;
                        selector = SELECTOR_PSEUDO;
                        trick_count = 1000000;
                        if *buffer.offset(cur_input.limit as isize)
                            == INTPAR!(INT_PAR__end_line_char)
                        {
                            j = cur_input.limit;
                        } else {
                            j = cur_input.limit + 1;
                        }
                        if j > 0 {
                            i = cur_input.start;
                            let for_end = j - 1;
                            while i <= for_end {
                                if i == cur_input.loc {
                                    first_count = tally;
                                    trick_count = tally + 1 + error_line - half_error_line;
                                    if trick_count < error_line {
                                        trick_count = error_line;
                                    }
                                }
                                print_char(*buffer.offset(i as isize) as i32);
                                i += 1;
                            }
                        }
                    } else {
                        match (cur_input.index) as i32 {
                            PARAMETER => print_nl_cstr(b"<argument> \0".as_ptr() as *const c_char),
                            U_TEMPLATE | V_TEMPLATE => {
                                print_nl_cstr(b"<template> \0".as_ptr() as *const c_char)
                            }
                            BACKED_UP | BACKED_UP_CHAR => {
                                if cur_input.loc == TEX_NULL {
                                    print_nl_cstr(b"<recently read> \0".as_ptr() as *const c_char);
                                } else {
                                    print_nl_cstr(
                                        b"<to be read again> \0".as_ptr() as *const c_char
                                    );
                                }
                            }
                            INSERTED => {
                                print_nl_cstr(b"<inserted text> \0".as_ptr() as *const c_char)
                            }
                            MACRO => {
                                print_ln();
                                print_cs(cur_input.name);
                            }
                            OUTPUT_TEXT => print_nl_cstr(b"<output> \0".as_ptr() as *const c_char),
                            EVERY_PAR_TEXT => {
                                print_nl_cstr(b"<everypar> \0".as_ptr() as *const c_char)
                            }
                            EVERY_MATH_TEXT => {
                                print_nl_cstr(b"<everymath> \0".as_ptr() as *const c_char)
                            }
                            EVERY_DISPLAY_TEXT => {
                                print_nl_cstr(b"<everydisplay> \0".as_ptr() as *const c_char)
                            }
                            EVERY_HBOX_TEXT => {
                                print_nl_cstr(b"<everyhbox> \0".as_ptr() as *const c_char)
                            }
                            EVERY_VBOX_TEXT => {
                                print_nl_cstr(b"<everyvbox> \0".as_ptr() as *const c_char)
                            }
                            EVERY_JOB_TEXT => {
                                print_nl_cstr(b"<everyjob> \0".as_ptr() as *const c_char)
                            }
                            EVERY_CR_TEXT => {
                                print_nl_cstr(b"<everycr> \0".as_ptr() as *const c_char)
                            }
                            MARK_TEXT => print_nl_cstr(b"<mark> \0".as_ptr() as *const c_char),
                            EVERY_EOF_TEXT => {
                                print_nl_cstr(b"<everyeof> \0".as_ptr() as *const c_char)
                            }
                            INTER_CHAR_TEXT => {
                                print_nl_cstr(b"<XeTeXinterchartoks> \0".as_ptr() as *const c_char)
                            }
                            WRITE_TEXT => print_nl_cstr(b"<write> \0".as_ptr() as *const c_char),
                            TECTONIC_CODA_TEXT => {
                                print_nl_cstr(b"<TectonicCodaTokens> \0".as_ptr() as *const c_char)
                            }
                            _ => print_nl(b'?' as i32),
                        }
                        l = tally;
                        tally = 0;
                        selector = SELECTOR_PSEUDO;
                        trick_count = 1000000;
                        if cur_input.index < (MACRO) as u16 {
                            show_token_list(cur_input.start, cur_input.loc, 100000);
                        } else {
                            show_token_list(mem!(cur_input.start).b32.s1, cur_input.loc, 100000);
                        }
                    }
                    selector = saved_selector;
                    if trick_count == 1000000 {
                        first_count = tally;
                        trick_count = tally + 1 + error_line - half_error_line;
                        if trick_count < error_line {
                            trick_count = error_line;
                        }
                    }
                    if tally < trick_count {
                        m = tally - first_count;
                    } else {
                        m = trick_count - first_count;
                    }
                    if l + first_count <= half_error_line {
                        p = 0;
                        n = l + first_count;
                    } else {
                        print_cstr(b"...\0".as_ptr() as *const c_char);
                        p = l + first_count - half_error_line + 3;
                        n = half_error_line;
                    }
                    q = p;
                    let for_end = first_count - 1;
                    while q <= for_end {
                        print_char(
                            *(&raw mut trick_buf)
                                .cast::<UTF16_code>()
                                .offset((q % error_line) as isize)
                                as i32,
                        );
                        q += 1;
                    }
                    print_ln();
                    q = 1;
                    while q <= n {
                        print_raw_char(b' ' as UTF16_code, true);
                        q += 1;
                    }
                    if m + n <= error_line {
                        p = first_count + m;
                    } else {
                        p = first_count + (error_line - n - 3);
                    }
                    q = first_count;
                    let for_end = p - 1;
                    while q <= for_end {
                        print_char(
                            *(&raw mut trick_buf)
                                .cast::<UTF16_code>()
                                .offset((q % error_line) as isize)
                                as i32,
                        );
                        q += 1;
                    }
                    if m + n > error_line {
                        print_cstr(b"...\0".as_ptr() as *const c_char);
                    }
                    nn += 1;
                }
            } else if nn == INTPAR!(INT_PAR__error_context_lines) {
                print_nl_cstr(b"...\0".as_ptr() as *const c_char);
                nn += 1;
            }
            if bottom_line {
                break;
            }
            base_ptr -= 1;
        }
        cur_input = *input_stack.offset(input_ptr as isize);
    }
}

#[unsafe(export_name = "begin_token_list")]
pub unsafe extern "C" fn begin_token_list(p: i32, t: u16) {
    unsafe {
        if input_ptr > max_in_stack {
            max_in_stack = input_ptr;
            if input_ptr == stack_size {
                overflow(
                    b"input stack size\0".as_ptr() as *const c_char,
                    (stack_size as usize) as i32,
                );
            }
        }
        *input_stack.offset(input_ptr as isize) = cur_input;
        input_ptr += 1;
        cur_input.state = (TOKEN_LIST) as u16;
        cur_input.start = p;
        cur_input.index = t;

        if t >= (MACRO) as u16 {
            mem!(p).b32.s0 += 1;
            if t == (MACRO) as u16 {
                cur_input.limit = param_ptr;
            } else {
                cur_input.loc = mem!(p).b32.s1;
                if INTPAR!(INT_PAR__tracing_macros) > 1 {
                    begin_diagnostic();
                    diagnostic_begin_capture_warning_here();
                    print_nl_cstr(b"\0".as_ptr() as *const c_char);
                    match (t) as i32 {
                        MARK_TEXT => print_esc_cstr(b"mark\0".as_ptr() as *const c_char),
                        WRITE_TEXT => print_esc_cstr(b"write\0".as_ptr() as *const c_char),
                        _ => print_cmd_chr(
                            ASSIGN_TOKS as u16,
                            t as i32 + LOCAL_BASE + LOCAL__output_routine - OUTPUT_TEXT as i32,
                        ),
                    }
                    print_cstr(b"->\0".as_ptr() as *const c_char);
                    token_show(p);
                    capture_to_diagnostic(std::ptr::null_mut());
                    end_diagnostic(false);
                }
            }
        } else {
            cur_input.loc = p;
        }
    }
}

#[unsafe(export_name = "end_token_list")]
pub unsafe extern "C" fn end_token_list() {
    unsafe {
        if cur_input.index >= (BACKED_UP) as u16 {
            if cur_input.index <= (INSERTED) as u16 {
                flush_list(cur_input.start);
            } else {
                delete_token_ref(cur_input.start);
                if cur_input.index == (MACRO) as u16 {
                    while param_ptr > cur_input.limit {
                        param_ptr -= 1;
                        flush_list(*param_stack.offset(param_ptr as isize));
                    }
                }
            }
        } else if cur_input.index == (U_TEMPLATE) as u16 {
            if align_state > 500000 {
                align_state = 0;
            } else {
                fatal_error(
                    b"(interwoven alignment preambles are not allowed)\0".as_ptr() as *const c_char,
                );
            }
        }
        input_ptr -= 1;
        cur_input = *input_stack.offset(input_ptr as isize);
    }
}

#[unsafe(export_name = "back_input")]
pub unsafe extern "C" fn back_input() {
    unsafe {
        let mut p: i32;
        while cur_input.state == (TOKEN_LIST) as u16
            && cur_input.loc == TEX_NULL
            && cur_input.index != (V_TEMPLATE) as u16
        {
            end_token_list();
        }
        p = get_avail();
        mem!(p).b32.s0 = cur_tok;
        if cur_tok < RIGHT_BRACE_LIMIT {
            if cur_tok < LEFT_BRACE_LIMIT {
                align_state -= 1;
            } else {
                align_state += 1;
            }
        }
        if input_ptr > max_in_stack {
            max_in_stack = input_ptr;
            if input_ptr == stack_size {
                overflow(
                    b"input stack size\0".as_ptr() as *const c_char,
                    (stack_size as usize) as i32,
                );
            }
        }
        *input_stack.offset(input_ptr as isize) = cur_input;
        input_ptr += 1;
        cur_input.state = (TOKEN_LIST) as u16;
        cur_input.start = p;
        cur_input.index = (BACKED_UP) as u16;
        cur_input.loc = p;
    }
}

#[unsafe(export_name = "back_error")]
pub unsafe extern "C" fn back_error() {
    unsafe {
        back_input();
        error();
    }
}

#[unsafe(export_name = "ins_error")]
pub unsafe extern "C" fn ins_error() {
    unsafe {
        back_input();
        cur_input.index = (INSERTED) as u16;
        error();
    }
}

#[unsafe(export_name = "begin_file_reading")]
pub unsafe extern "C" fn begin_file_reading() {
    unsafe {
        if in_open as i32 == max_in_open {
            overflow(
                b"text input levels\0".as_ptr() as *const c_char,
                (max_in_open as usize) as i32,
            );
        }
        if first == buf_size {
            overflow(
                b"buffer size\0".as_ptr() as *const c_char,
                (buf_size as usize) as i32,
            );
        }
        in_open += 1;
        if input_ptr > max_in_stack {
            max_in_stack = input_ptr;
            if input_ptr == stack_size {
                overflow(
                    b"input stack size\0".as_ptr() as *const c_char,
                    (stack_size as usize) as i32,
                );
            }
        }
        *input_stack.offset(input_ptr as isize) = cur_input;
        input_ptr += 1;
        cur_input.index = (in_open) as u16;
        *source_filename_stack.offset(cur_input.index as isize) = 0;
        *full_source_filename_stack.offset(cur_input.index as isize) = 0;
        *eof_seen.offset(cur_input.index as isize) = false;
        *grp_stack.offset(cur_input.index as isize) = cur_boundary;
        *if_stack.offset(cur_input.index as isize) = cond_ptr;
        *line_stack.offset(cur_input.index as isize) = line;
        cur_input.start = first;
        cur_input.state = (MID_LINE) as u16;
        cur_input.name = 0;
        cur_input.synctex_tag = 0;
    }
}

#[unsafe(export_name = "end_file_reading")]
pub unsafe extern "C" fn end_file_reading() {
    unsafe {
        first = cur_input.start;
        line = *line_stack.offset(cur_input.index as isize);

        if cur_input.name == 18 || cur_input.name == 19 {
            pseudo_close();
        } else if cur_input.name > 17 {
            u_close(*input_file.offset(cur_input.index as isize));
            *input_file.offset(cur_input.index as isize) = std::ptr::null_mut();
        }

        input_ptr -= 1;
        cur_input = *input_stack.offset(input_ptr as isize);
        in_open -= 1;
    }
}

#[unsafe(export_name = "check_outer_validity")]
pub unsafe extern "C" fn check_outer_validity() {
    unsafe {
        let mut p: i32;
        let q: i32;

        if scanner_status != (NORMAL) as u8 {
            deletions_allowed = false;

            if cur_cs != 0 {
                if cur_input.state == (TOKEN_LIST) as u16
                    || cur_input.name < 1
                    || cur_input.name > 17
                {
                    p = get_avail();
                    mem!(p).b32.s0 = CS_TOKEN_FLAG + cur_cs;
                    begin_token_list(p, (BACKED_UP) as u16);
                }
                cur_cmd = (SPACER) as u8;
                cur_chr = b' ' as i32;
            }

            if scanner_status > (SKIPPING) as u8 {
                runaway();

                if cur_cs == 0 {
                    error_here_with_diagnostic(b"File ended\0".as_ptr() as *const c_char);
                } else {
                    cur_cs = 0;
                    error_here_with_diagnostic(
                        b"Forbidden control sequence found\0".as_ptr() as *const c_char
                    );
                }

                p = get_avail();
                match (scanner_status) as i32 {
                    DEFINING => {
                        print_cstr(b" while scanning definition\0".as_ptr() as *const c_char);
                        mem!(p).b32.s0 = RIGHT_BRACE_TOKEN + b'}' as i32;
                    }
                    MATCHING => {
                        print_cstr(b" while scanning use\0".as_ptr() as *const c_char);
                        mem!(p).b32.s0 = par_token;
                        long_state = (OUTER_CALL) as u8;
                    }
                    ALIGNING => {
                        print_cstr(b" while scanning preamble\0".as_ptr() as *const c_char);
                        mem!(p).b32.s0 = RIGHT_BRACE_TOKEN + b'}' as i32;
                        let q_align = p;
                        p = get_avail();
                        mem!(p).b32.s1 = q_align;
                        mem!(p).b32.s0 = CS_TOKEN_FLAG + FROZEN_CR;
                        align_state = -1000000;
                    }
                    ABSORBING => {
                        print_cstr(b" while scanning text\0".as_ptr() as *const c_char);
                        mem!(p).b32.s0 = RIGHT_BRACE_TOKEN + b'}' as i32;
                    }
                    _ => {}
                }

                begin_token_list(p, (INSERTED) as u16);
                print_cstr(b" of \0".as_ptr() as *const c_char);
                sprint_cs(warning_index);
                capture_to_diagnostic(std::ptr::null_mut());
                help_ptr = 4;
                *(&raw mut help_line).cast::<*const libc::c_char>().offset(3) =
                    b"I suspect you have forgotten a `}', causing me\0".as_ptr() as *const c_char;
                *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                    b"to read past where you wanted me to stop.\0".as_ptr() as *const c_char;
                *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                    b"I'll try to recover; but if the error is serious,\0".as_ptr()
                        as *const c_char;
                *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                    b"you'd better type `E' or `X' now and fix your file.\0".as_ptr()
                        as *const c_char;
                error();
            } else {
                error_here_with_diagnostic(b"Incomplete \0".as_ptr() as *const c_char);
                print_cmd_chr((IF_TEST) as u16, cur_if as i32);
                print_cstr(b"; all text was ignored after line \0".as_ptr() as *const c_char);
                print_int(skip_line);
                capture_to_diagnostic(std::ptr::null_mut());
                help_ptr = 3;
                *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                    b"A forbidden control sequence occurred in skipped text.\0".as_ptr()
                        as *const c_char;
                *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                    b"This kind of error happens when you say `\\if...' and forget\0".as_ptr()
                        as *const c_char;
                *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                    b"the matching `\\fi'. I've inserted a `\\fi'; this might work.\0".as_ptr()
                        as *const c_char;

                if cur_cs != 0 {
                    cur_cs = 0;
                } else {
                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                        b"The file ended while I was skipping conditional text.\0".as_ptr()
                            as *const c_char;
                }

                cur_tok = CS_TOKEN_FLAG + FROZEN_FI;
                ins_error();
            }

            deletions_allowed = true;
        }
    }
}

#[unsafe(export_name = "get_next")]
pub unsafe extern "C" fn get_next() {
    unsafe {
        if cur_input.state == TOKEN_LIST as u16 && cur_input.loc != TEX_NULL {
            let node = *mem.add(cur_input.loc as usize);
            let token = node.b32.s0;
            let (command, character, control_sequence) = if token >= CS_TOKEN_FLAG {
                let control_sequence = token - CS_TOKEN_FLAG;
                let definition = *eqtb.add(control_sequence as usize);
                let command = definition.b16.s1 as u8;
                if command >= OUTER_CALL as u8 {
                    get_next_slow();
                    return;
                }
                (command, definition.b32.s1, control_sequence)
            } else {
                let command = (token / MAX_CHAR_VAL) as u8;
                if command == OUT_PARAM as u8 {
                    get_next_slow();
                    return;
                }
                (command, token % MAX_CHAR_VAL, 0)
            };
            if align_state == 0 && command >= TAB_MARK as u8 && command <= CAR_RET as u8 {
                get_next_slow();
                return;
            }
            cur_input.loc = node.b32.s1;
            cur_cmd = command;
            cur_chr = character;
            cur_cs = control_sequence;
            if token < CS_TOKEN_FLAG {
                if command == LEFT_BRACE as u8 {
                    align_state += 1;
                } else if command == RIGHT_BRACE as u8 {
                    align_state -= 1;
                }
            }
            return;
        } else if cur_input.state != TOKEN_LIST as u16 && cur_input.loc <= cur_input.limit {
            let character = *buffer.add(cur_input.loc as usize);
            if (0..=BIGGEST_USV).contains(&character) && !(0xD800..0xDC00).contains(&character) {
                let command = (*eqtb.add((CAT_CODE_BASE + character) as usize)).b32.s1 as u8;
                if command == LETTER as u8 || command == OTHER_CHAR as u8 {
                    cur_input.loc += 1;
                    cur_input.state = MID_LINE as u16;
                    cur_cmd = command;
                    cur_chr = character;
                    cur_cs = 0;
                    return;
                }
            }
        }
        get_next_slow();
    }
}

#[cfg(test)]
mod token_tests {
    use super::*;

    #[test]
    fn token_list_fast_path_matches_full_scanner() {
        unsafe {
            let saved = (
                mem,
                eqtb,
                buffer,
                cur_input,
                align_state,
                cur_cmd,
                cur_chr,
                cur_cs,
            );
            let result = std::panic::catch_unwind(|| {
                let zero: memory_word = std::mem::zeroed();
                let mut nodes = [zero; 4];
                let mut definitions = vec![zero; (CAT_CODE_BASE + BIGGEST_USV + 1) as usize];
                mem = nodes.as_mut_ptr();
                eqtb = definitions.as_mut_ptr();
                nodes[1].b32.s1 = 2;
                for is_control_sequence in [false, true] {
                    let commands = if is_control_sequence { OUTER_CALL } else { 16 };
                    for command in 0..commands {
                        if !is_control_sequence && command == OUT_PARAM {
                            continue;
                        }
                        for character in [0, 32, 65535, 65536, 0x10ffff] {
                            for alignment in [-100, 0, 1, 1000000] {
                                if alignment == 0 && (TAB_MARK..=CAR_RET).contains(&command) {
                                    continue;
                                }
                                nodes[1].b32.s0 = if is_control_sequence {
                                    definitions[7].b16.s1 = command as u16;
                                    definitions[7].b32.s1 = character;
                                    CS_TOKEN_FLAG + 7
                                } else {
                                    command * MAX_CHAR_VAL + character
                                };
                                let reset = || {
                                    cur_input.state = TOKEN_LIST as u16;
                                    cur_input.loc = 1;
                                    align_state = alignment;
                                    cur_cmd = 255;
                                    cur_chr = -1;
                                    cur_cs = -1;
                                };
                                reset();
                                get_next_slow();
                                let expected =
                                    (cur_cmd, cur_chr, cur_cs, cur_input.loc, align_state);
                                reset();
                                get_next();
                                assert_eq!(
                                    (cur_cmd, cur_chr, cur_cs, cur_input.loc, align_state),
                                    expected
                                );
                            }
                        }
                    }
                }
                let mut source = [0; 4];
                buffer = source.as_mut_ptr();
                for state in [MID_LINE, SKIP_BLANKS, NEW_LINE] {
                    for command in [LETTER, OTHER_CHAR] {
                        for character in [
                            0, 32, 97, 0x3b1, 0xd800, 0xdbff, 0xdc00, 0xffff, 0x10000, 0x10ffff,
                        ] {
                            source[1] = character;
                            definitions[(CAT_CODE_BASE + character) as usize].b32.s1 = command;
                            let reset = || {
                                cur_input.state = state as u16;
                                cur_input.loc = 1;
                                cur_input.limit = 1;
                                cur_cmd = 255;
                                cur_chr = -1;
                                cur_cs = -1;
                            };
                            reset();
                            get_next_slow();
                            let expected = (
                                cur_cmd,
                                cur_chr,
                                cur_cs,
                                cur_input.loc,
                                cur_input.state,
                                align_state,
                            );
                            reset();
                            get_next();
                            assert_eq!(
                                (
                                    cur_cmd,
                                    cur_chr,
                                    cur_cs,
                                    cur_input.loc,
                                    cur_input.state,
                                    align_state
                                ),
                                expected
                            );
                        }
                    }
                }
            });
            (
                mem,
                eqtb,
                buffer,
                cur_input,
                align_state,
                cur_cmd,
                cur_chr,
                cur_cs,
            ) = saved;
            if let Err(error) = result {
                std::panic::resume_unwind(error);
            }
        }
    }
}

#[inline(never)]
unsafe fn get_next_slow() {
    unsafe {
        let mut k: i32;
        let t: i32;
        let mut cat: u8;
        let mut c: UnicodeScalar;
        let mut lower: UTF16_code;
        let mut d: small_number;
        let mut sup_count: small_number;

        'restart: loop {
            cur_cs = 0;

            if cur_input.state != (TOKEN_LIST) as u16 {
                'texswitch: loop {
                    if cur_input.loc <= cur_input.limit {
                        cur_chr = *buffer.offset(cur_input.loc as isize);
                        cur_input.loc += 1;

                        if cur_chr >= 0xD800
                            && cur_chr < 0xDC00
                            && cur_input.loc <= cur_input.limit
                            && *buffer.offset(cur_input.loc as isize) >= 0xDC00
                            && *buffer.offset(cur_input.loc as isize) < 0xE000
                        {
                            lower = (*buffer.offset(cur_input.loc as isize) - 0xDC00) as UTF16_code;
                            cur_input.loc += 1;
                            cur_chr = 65536 + (cur_chr - 0xD800) * 1024 + lower as i32;
                        }

                        'reswitch: loop {
                            cur_cmd = ((*eqtb.offset((CAT_CODE_BASE + cur_chr) as isize)).b32.s1
                                as u16) as u8;

                            let state_cmd = cur_input.state + (cur_cmd) as u16;
                            match (state_cmd) as i32 {
                                x if x == ((MID_LINE + IGNORE) as u16) as i32
                                    || x == ((SKIP_BLANKS + IGNORE) as u16) as i32
                                    || x == ((NEW_LINE + IGNORE) as u16) as i32
                                    || x == SKIP_BLANKS + SPACER
                                    || x == NEW_LINE + SPACER =>
                                {
                                    continue 'texswitch;
                                }
                                x if x == MID_LINE + ESCAPE
                                    || x == SKIP_BLANKS + ESCAPE
                                    || x == NEW_LINE + ESCAPE =>
                                {
                                    if cur_input.loc > cur_input.limit {
                                        cur_cs = NULL_CS;
                                    } else {
                                        'start_cs: loop {
                                            k = cur_input.loc;
                                            cur_chr = *buffer.offset(k as isize);
                                            cat = (*eqtb.offset((CAT_CODE_BASE + cur_chr) as isize))
                                                .b32
                                                .s1
                                                as u8;
                                            k += 1;

                                            if cat as u16 == (LETTER) as u16
                                                || cat as u16 == (SPACER) as u16
                                            {
                                                cur_input.state = (SKIP_BLANKS) as u16;
                                            } else {
                                                cur_input.state = (MID_LINE) as u16;
                                            }

                                            if cat as u16 == (LETTER) as u16 && k <= cur_input.limit
                                            {
                                                loop {
                                                    cur_chr = *buffer.offset(k as isize);
                                                    cat = (*eqtb
                                                        .offset((CAT_CODE_BASE + cur_chr) as isize))
                                                    .b32
                                                    .s1
                                                        as u8;
                                                    k += 1;
                                                    if cat as u16 != (LETTER) as u16
                                                        || k > cur_input.limit
                                                    {
                                                        break;
                                                    }
                                                }

                                                if cat as u16 == (SUP_MARK) as u16
                                                    && *buffer.offset(k as isize) == cur_chr
                                                    && k < cur_input.limit
                                                {
                                                    sup_count = 2;
                                                    while sup_count < 6
                                                        && k + 2 * sup_count as i32 - 2
                                                            <= cur_input.limit
                                                        && *buffer.offset(
                                                            (k + sup_count as i32 - 1) as isize,
                                                        ) == cur_chr
                                                    {
                                                        sup_count += 1;
                                                    }

                                                    let sup_count_save = sup_count;
                                                    let mut old_style = false;
                                                    for d_idx in 1..=sup_count_save {
                                                        if !IS_LC_HEX(*buffer.offset(
                                                            (k + sup_count as i32 - 2
                                                                + d_idx as i32)
                                                                as isize,
                                                        )) {
                                                            c = *buffer.offset((k + 1) as isize);
                                                            if c < 128 {
                                                                if c < 64 {
                                                                    *buffer
                                                                        .offset((k - 1) as isize) =
                                                                        c + 64;
                                                                } else {
                                                                    *buffer
                                                                        .offset((k - 1) as isize) =
                                                                        c - 64;
                                                                }
                                                                d = 2;
                                                                cur_input.limit -= d as i32;
                                                                while k <= cur_input.limit {
                                                                    *buffer.offset(k as isize) =
                                                                        *buffer.offset(
                                                                            (k + d as i32) as isize,
                                                                        );
                                                                    k += 1;
                                                                }
                                                                continue 'start_cs;
                                                            } else {
                                                                sup_count = 0;
                                                            }
                                                            old_style = true;
                                                            break;
                                                        }
                                                    }

                                                    if sup_count > 0 {
                                                        cur_chr = 0;
                                                        for d_idx in 1..=sup_count {
                                                            c = *buffer.offset(
                                                                (k + sup_count as i32 - 2
                                                                    + d_idx as i32)
                                                                    as isize,
                                                            );
                                                            if c <= b'9' as i32 {
                                                                cur_chr =
                                                                    16 * cur_chr + c - b'0' as i32;
                                                            } else {
                                                                cur_chr = 16 * cur_chr + c
                                                                    - b'a' as i32
                                                                    + 10;
                                                            }
                                                        }
                                                        if cur_chr > BIGGEST_USV {
                                                            cur_chr = *buffer.offset(k as isize);
                                                        } else {
                                                            *buffer.offset((k - 1) as isize) =
                                                                cur_chr;
                                                            d = 2 * sup_count - 1;
                                                            cur_input.limit -= d as i32;
                                                            while k <= cur_input.limit {
                                                                *buffer.offset(k as isize) =
                                                                    *buffer.offset(
                                                                        (k + d as i32) as isize,
                                                                    );
                                                                k += 1;
                                                            }
                                                            continue 'start_cs;
                                                        }
                                                    }
                                                }

                                                if cat as u16 != (LETTER) as u16 {
                                                    k -= 1;
                                                }
                                                if k > cur_input.loc + 1 {
                                                    cur_cs =
                                                        id_lookup(cur_input.loc, k - cur_input.loc);
                                                    cur_input.loc = k;
                                                    break 'start_cs;
                                                }
                                            } else {
                                                if cat as u16 == (SUP_MARK) as u16
                                                    && *buffer.offset(k as isize) == cur_chr
                                                    && k < cur_input.limit
                                                {
                                                    sup_count = 2;
                                                    while sup_count < 6
                                                        && k + 2 * sup_count as i32 - 2
                                                            <= cur_input.limit
                                                        && *buffer.offset(
                                                            (k + sup_count as i32 - 1) as isize,
                                                        ) == cur_chr
                                                    {
                                                        sup_count += 1;
                                                    }
                                                    let sup_count_save = sup_count;
                                                    for d_idx in 1..=sup_count_save {
                                                        if !IS_LC_HEX(*buffer.offset(
                                                            (k + sup_count as i32 - 2
                                                                + d_idx as i32)
                                                                as isize,
                                                        )) {
                                                            c = *buffer.offset((k + 1) as isize);
                                                            if c < 128 {
                                                                if c < 64 {
                                                                    *buffer
                                                                        .offset((k - 1) as isize) =
                                                                        c + 64;
                                                                } else {
                                                                    *buffer
                                                                        .offset((k - 1) as isize) =
                                                                        c - 64;
                                                                }
                                                                d = 2;
                                                                cur_input.limit -= d as i32;
                                                                while k <= cur_input.limit {
                                                                    *buffer.offset(k as isize) =
                                                                        *buffer.offset(
                                                                            (k + d as i32) as isize,
                                                                        );
                                                                    k += 1;
                                                                }
                                                                continue 'start_cs;
                                                            } else {
                                                                sup_count = 0;
                                                            }
                                                        }
                                                    }
                                                    if sup_count > 0 {
                                                        cur_chr = 0;
                                                        for d_idx in 1..=sup_count {
                                                            c = *buffer.offset(
                                                                (k + sup_count as i32 - 2
                                                                    + d_idx as i32)
                                                                    as isize,
                                                            );
                                                            if c <= b'9' as i32 {
                                                                cur_chr =
                                                                    16 * cur_chr + c - b'0' as i32;
                                                            } else {
                                                                cur_chr = 16 * cur_chr + c
                                                                    - b'a' as i32
                                                                    + 10;
                                                            }
                                                        }
                                                        if cur_chr > BIGGEST_USV {
                                                            cur_chr = *buffer.offset(k as isize);
                                                        } else {
                                                            *buffer.offset((k - 1) as isize) =
                                                                cur_chr;
                                                            d = 2 * sup_count - 1;
                                                            cur_input.limit -= d as i32;
                                                            while k <= cur_input.limit {
                                                                *buffer.offset(k as isize) =
                                                                    *buffer.offset(
                                                                        (k + d as i32) as isize,
                                                                    );
                                                                k += 1;
                                                            }
                                                            continue 'start_cs;
                                                        }
                                                    }
                                                }
                                            }

                                            if *buffer.offset(cur_input.loc as isize) > 65535 {
                                                cur_cs = id_lookup(cur_input.loc, 1);
                                                cur_input.loc += 1;
                                                break 'start_cs;
                                            }

                                            cur_cs = SINGLE_BASE
                                                + *buffer.offset(cur_input.loc as isize);
                                            cur_input.loc += 1;
                                            break 'start_cs;
                                        }
                                    }

                                    cur_cmd = (eqtb!(cur_cs).b16.s1) as u8;
                                    cur_chr = eqtb!(cur_cs).b32.s1;
                                    if cur_cmd >= (OUTER_CALL) as u8 {
                                        check_outer_validity();
                                    }
                                    break 'texswitch;
                                }
                                x if x == MID_LINE + ACTIVE_CHAR
                                    || x == SKIP_BLANKS + ACTIVE_CHAR
                                    || x == NEW_LINE + ACTIVE_CHAR =>
                                {
                                    cur_cs = cur_chr + 1;
                                    cur_cmd = (eqtb!(cur_cs).b16.s1) as u8;
                                    cur_chr = eqtb!(cur_cs).b32.s1;
                                    cur_input.state = (MID_LINE) as u16;
                                    if cur_cmd >= (OUTER_CALL) as u8 {
                                        check_outer_validity();
                                    }
                                    break 'texswitch;
                                }
                                x if x == MID_LINE + SUP_MARK
                                    || x == SKIP_BLANKS + SUP_MARK
                                    || x == NEW_LINE + SUP_MARK =>
                                {
                                    if cur_chr == *buffer.offset(cur_input.loc as isize) {
                                        if cur_input.loc < cur_input.limit {
                                            sup_count = 2;
                                            while sup_count < 6
                                                && cur_input.loc + 2 * sup_count as i32 - 2
                                                    <= cur_input.limit
                                                && cur_chr
                                                    == *buffer.offset(
                                                        (cur_input.loc + sup_count as i32 - 1)
                                                            as isize,
                                                    )
                                            {
                                                sup_count += 1;
                                            }

                                            let mut not_exp = false;
                                            for d_idx in 1..=sup_count {
                                                if !IS_LC_HEX(*buffer.offset(
                                                    (cur_input.loc + sup_count as i32 - 2
                                                        + d_idx as i32)
                                                        as isize,
                                                )) {
                                                    c = *buffer
                                                        .offset((cur_input.loc + 1) as isize);
                                                    if c < 128 {
                                                        cur_input.loc += 2;
                                                        if c < 64 {
                                                            cur_chr = c + 64;
                                                        } else {
                                                            cur_chr = c - 64;
                                                        }
                                                        continue 'reswitch;
                                                    }
                                                    not_exp = true;
                                                    break;
                                                }
                                            }

                                            if !not_exp {
                                                cur_chr = 0;
                                                for d_idx in 1..=sup_count {
                                                    c = *buffer.offset(
                                                        (cur_input.loc + sup_count as i32 - 2
                                                            + d_idx as i32)
                                                            as isize,
                                                    );
                                                    if c <= b'9' as i32 {
                                                        cur_chr = 16 * cur_chr + c - b'0' as i32;
                                                    } else {
                                                        cur_chr =
                                                            16 * cur_chr + c - b'a' as i32 + 10;
                                                    }
                                                }
                                                if cur_chr > BIGGEST_USV {
                                                    cur_chr =
                                                        *buffer.offset(cur_input.loc as isize);
                                                } else {
                                                    cur_input.loc += 2 * sup_count as i32 - 1;
                                                    continue 'reswitch;
                                                }
                                            }
                                        }
                                    }
                                    cur_input.state = (MID_LINE) as u16;
                                    break 'texswitch;
                                }
                                x if x == MID_LINE + INVALID_CHAR
                                    || x == SKIP_BLANKS + INVALID_CHAR
                                    || x == NEW_LINE + INVALID_CHAR =>
                                {
                                    error_here_with_diagnostic(
                                        b"Text line contains an invalid character\0".as_ptr()
                                            as *const c_char,
                                    );
                                    capture_to_diagnostic(std::ptr::null_mut());
                                    help_ptr = 2;
                                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                        b"A funny symbol that I can't read has just been input.\0"
                                            .as_ptr()
                                            as *const c_char;
                                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                        b"Continue, and I'll forget that it ever happened.\0"
                                            .as_ptr()
                                            as *const c_char;
                                    deletions_allowed = false;
                                    error();
                                    deletions_allowed = true;
                                    continue 'restart;
                                }
                                x if x == MID_LINE + SPACER => {
                                    cur_input.state = (SKIP_BLANKS) as u16;
                                    cur_chr = b' ' as i32;
                                    break 'texswitch;
                                }
                                x if x == MID_LINE + CAR_RET => {
                                    cur_input.loc = cur_input.limit + 1;
                                    cur_cmd = (SPACER) as u8;
                                    cur_chr = b' ' as i32;
                                    break 'texswitch;
                                }
                                x if x == ((MID_LINE + COMMENT) as u16) as i32
                                    || x == ((SKIP_BLANKS + COMMENT) as u16) as i32
                                    || x == ((NEW_LINE + COMMENT) as u16) as i32
                                    || x == SKIP_BLANKS + CAR_RET =>
                                {
                                    cur_input.loc = cur_input.limit + 1;
                                    continue 'texswitch;
                                }
                                x if x == NEW_LINE + CAR_RET => {
                                    cur_input.loc = cur_input.limit + 1;
                                    cur_cs = par_loc;
                                    cur_cmd = (eqtb!(cur_cs).b16.s1) as u8;
                                    cur_chr = eqtb!(cur_cs).b32.s1;
                                    if cur_cmd >= (OUTER_CALL) as u8 {
                                        check_outer_validity();
                                    }
                                    break 'texswitch;
                                }
                                x if x == MID_LINE + LEFT_BRACE => {
                                    align_state += 1;
                                    break 'texswitch;
                                }
                                x if x == SKIP_BLANKS + LEFT_BRACE
                                    || x == NEW_LINE + LEFT_BRACE =>
                                {
                                    cur_input.state = (MID_LINE) as u16;
                                    align_state += 1;
                                    break 'texswitch;
                                }
                                x if x == MID_LINE + RIGHT_BRACE => {
                                    align_state -= 1;
                                    break 'texswitch;
                                }
                                x if x == SKIP_BLANKS + RIGHT_BRACE
                                    || x == NEW_LINE + RIGHT_BRACE =>
                                {
                                    cur_input.state = (MID_LINE) as u16;
                                    align_state -= 1;
                                    break 'texswitch;
                                }
                                x if x == ((SKIP_BLANKS + MATH_SHIFT) as u16) as i32
                                    || x == ((SKIP_BLANKS + TAB_MARK) as u16) as i32
                                    || x == ((SKIP_BLANKS + MAC_PARAM) as u16) as i32
                                    || x == ((SKIP_BLANKS + SUB_MARK) as u16) as i32
                                    || x == ((SKIP_BLANKS + LETTER) as u16) as i32
                                    || x == ((SKIP_BLANKS + OTHER_CHAR) as u16) as i32
                                    || x == ((NEW_LINE + MATH_SHIFT) as u16) as i32
                                    || x == ((NEW_LINE + TAB_MARK) as u16) as i32
                                    || x == ((NEW_LINE + MAC_PARAM) as u16) as i32
                                    || x == NEW_LINE + SUB_MARK
                                    || x == NEW_LINE + LETTER
                                    || x == NEW_LINE + OTHER_CHAR =>
                                {
                                    cur_input.state = (MID_LINE) as u16;
                                    break 'texswitch;
                                }
                                _ => {
                                    break 'texswitch;
                                }
                            }
                        }
                    } else {
                        cur_input.state = (NEW_LINE) as u16;

                        if cur_input.name > 17 {
                            line += 1;
                            first = cur_input.start;

                            if !force_eof {
                                if cur_input.name <= 19 {
                                    if pseudo_input() {
                                        cur_input.limit = last;
                                    } else if eqtb!(LOCAL_BASE + LOCAL__every_eof).b32.s1
                                        != TEX_NULL
                                        && !*eof_seen.offset(cur_input.index as isize)
                                    {
                                        cur_input.limit = first - 1;
                                        *eof_seen.offset(cur_input.index as isize) = true;
                                        begin_token_list(
                                            eqtb!(LOCAL_BASE + LOCAL__every_eof).b32.s1,
                                            (EVERY_EOF_TEXT) as u16,
                                        );
                                        continue 'restart;
                                    } else {
                                        force_eof = true;
                                    }
                                } else {
                                    if input_line(*input_file.offset(cur_input.index as isize)) != 0
                                    {
                                        cur_input.limit = last;
                                    } else if eqtb!(LOCAL_BASE + LOCAL__every_eof).b32.s1
                                        != TEX_NULL
                                        && !*eof_seen.offset(cur_input.index as isize)
                                    {
                                        cur_input.limit = first - 1;
                                        *eof_seen.offset(cur_input.index as isize) = true;
                                        begin_token_list(
                                            eqtb!(LOCAL_BASE + LOCAL__every_eof).b32.s1,
                                            (EVERY_EOF_TEXT) as u16,
                                        );
                                        continue 'restart;
                                    } else {
                                        force_eof = true;
                                    }
                                }
                            }

                            if force_eof {
                                if INTPAR!(INT_PAR__tracing_nesting) > 0 {
                                    if *grp_stack.offset(in_open as isize) != cur_boundary
                                        || *if_stack.offset(in_open as isize) != cond_ptr
                                    {
                                        file_warning();
                                    }
                                }
                                if cur_input.name >= 19 {
                                    print_char(b')' as i32);
                                    open_parens -= 1;
                                    ttstub_output_flush(rust_stdout);
                                }
                                force_eof = false;
                                end_file_reading();
                                check_outer_validity();
                                continue 'restart;
                            }

                            if INTPAR!(INT_PAR__end_line_char) < 0
                                || INTPAR!(INT_PAR__end_line_char) > 255
                            {
                                cur_input.limit -= 1;
                            } else {
                                *buffer.offset(cur_input.limit as isize) =
                                    INTPAR!(INT_PAR__end_line_char) as UnicodeScalar;
                            }

                            first = cur_input.limit + 1;
                            cur_input.loc = cur_input.start;
                        } else {
                            if cur_input.name != 0 {
                                cur_cmd = 0;
                                cur_chr = 0;
                                return;
                            }

                            if input_ptr > 0 {
                                end_file_reading();
                                continue 'restart;
                            }

                            if !used_tectonic_coda_tokens
                                && eqtb!(LOCAL_BASE + LOCAL__tectonic_coda_tokens).b32.s1
                                    != TEX_NULL
                            {
                                used_tectonic_coda_tokens = true;
                                begin_token_list(
                                    eqtb!(LOCAL_BASE + LOCAL__tectonic_coda_tokens).b32.s1,
                                    (TECTONIC_CODA_TEXT) as u16,
                                );
                                continue 'restart;
                            }

                            if selector < SELECTOR_LOG_ONLY {
                                open_log_file();
                            }

                            fatal_error(b"*** (job aborted, no legal \\end found)\0".as_ptr()
                                as *const c_char);
                        }
                        continue 'texswitch;
                    }
                }
            } else if cur_input.loc != TEX_NULL {
                let t_tok = mem!(cur_input.loc).b32.s0;
                cur_input.loc = LLIST_link!(cur_input.loc);

                if t_tok >= CS_TOKEN_FLAG {
                    cur_cs = t_tok - CS_TOKEN_FLAG;
                    cur_cmd = (eqtb!(cur_cs).b16.s1) as u8;
                    cur_chr = eqtb!(cur_cs).b32.s1;

                    if cur_cmd >= (OUTER_CALL) as u8 {
                        if cur_cmd == (DONT_EXPAND) as u8 {
                            cur_cs = mem!(cur_input.loc).b32.s0 - CS_TOKEN_FLAG;
                            cur_input.loc = TEX_NULL;
                            cur_cmd = (eqtb!(cur_cs).b16.s1) as u8;
                            cur_chr = eqtb!(cur_cs).b32.s1;
                            if cur_cmd > (MAX_COMMAND) as u8 {
                                cur_cmd = (RELAX) as u8;
                                cur_chr = NO_EXPAND_FLAG;
                            }
                        } else {
                            check_outer_validity();
                        }
                    }
                } else {
                    cur_cmd = ((t_tok / MAX_CHAR_VAL) as u16) as u8;
                    cur_chr = t_tok % MAX_CHAR_VAL;

                    match (cur_cmd) as i32 {
                        LEFT_BRACE => {
                            align_state += 1;
                        }
                        RIGHT_BRACE => {
                            align_state -= 1;
                        }
                        OUT_PARAM => {
                            begin_token_list(
                                *param_stack
                                    .offset((cur_input.limit as i32 + cur_chr - 1) as isize),
                                (PARAMETER) as u16,
                            );
                            continue 'restart;
                        }
                        _ => {}
                    }
                }
            } else {
                end_token_list();
                continue 'restart;
            }

            if cur_cmd <= (CAR_RET) as u8 && cur_cmd >= (TAB_MARK) as u8 && align_state == 0 {
                if scanner_status == (ALIGNING) as u8 || cur_align == TEX_NULL {
                    fatal_error(
                        b"(interwoven alignment preambles are not allowed)\0".as_ptr()
                            as *const c_char,
                    );
                }
                cur_cmd = (mem!(cur_align + 5).b32.s0 as u16) as u8;
                mem!(cur_align + 5).b32.s0 = cur_chr;
                if cur_cmd == (OMIT) as u8 {
                    begin_token_list(OMIT_TEMPLATE, (V_TEMPLATE) as u16);
                } else {
                    begin_token_list(mem!(cur_align + 2).b32.s1, (V_TEMPLATE) as u16);
                }
                align_state = 1000000;
                continue 'restart;
            }

            break 'restart;
        }
    }
}

#[unsafe(export_name = "get_token")]
pub unsafe extern "C" fn get_token() {
    unsafe {
        no_new_control_sequence = false;
        get_next();
        no_new_control_sequence = true;
        if cur_cs == 0 {
            cur_tok = (cur_cmd as i32 * MAX_CHAR_VAL) + cur_chr;
        } else {
            cur_tok = CS_TOKEN_FLAG + cur_cs;
        }
    }
}

#[unsafe(export_name = "macro_call")]
pub unsafe extern "C" fn macro_call() {
    unsafe {
        let mut r: i32;
        let mut p: i32 = TEX_NULL;
        let mut q: i32;
        let mut s: i32;
        let mut t: i32;
        let mut u: i32;
        let mut v: i32;
        let mut rbrace_ptr: i32 = TEX_NULL;
        let mut n: small_number;
        let mut unbalance: i32;
        let mut m: i32 = 0;
        let ref_count: i32;
        let mut save_scanner_status: u8;
        let save_warning_index: i32;
        let mut match_chr: UTF16_code = 0;

        save_scanner_status = scanner_status;
        save_warning_index = warning_index;
        warning_index = cur_cs;
        ref_count = cur_chr;
        r = mem!(ref_count).b32.s1;
        n = 0;

        if INTPAR!(INT_PAR__tracing_macros) > 0 {
            begin_diagnostic();
            diagnostic_begin_capture_warning_here();

            if INTPAR!(INT_PAR__tracing_stack_levels) > 0 {
                if input_ptr < INTPAR!(INT_PAR__tracing_stack_levels) {
                    v = input_ptr;
                    print_ln();
                    print_char(b'~' as i32);
                    while v > 0 {
                        print_char(b'.' as i32);
                        v -= 1;
                    }
                    print_cs(warning_index);
                    token_show(ref_count);
                } else {
                    print_char(b'~' as i32);
                    print_char(b'~' as i32);
                    print_cs(warning_index);
                }
            } else {
                print_ln();
                print_cs(warning_index);
                token_show(ref_count);
            }

            capture_to_diagnostic(std::ptr::null_mut());
            end_diagnostic(false);
        }

        if mem!(r).b32.s0 == PROTECTED_TOKEN {
            r = LLIST_link!(r);
        }

        if mem!(r).b32.s0 != END_MATCH_TOKEN {
            scanner_status = (MATCHING) as u8;
            unbalance = 0;
            long_state = (eqtb!(cur_cs).b16.s1) as u8;
            if long_state >= (OUTER_CALL) as u8 {
                long_state -= 2;
            }

            'match_loop: loop {
                mem!(TEMP_HEAD).b32.s1 = TEX_NULL;
                if mem!(r).b32.s0 >= END_MATCH_TOKEN || mem!(r).b32.s0 < MATCH_TOKEN {
                    s = TEX_NULL;
                } else {
                    match_chr = (mem!(r).b32.s0 - MATCH_TOKEN) as UTF16_code;
                    s = mem!(r).b32.s1;
                    r = s;
                    p = TEMP_HEAD;
                    m = 0;
                }

                'arg_scan: loop {
                    get_token();

                    if cur_tok == mem!(r).b32.s0 {
                        r = LLIST_link!(r);
                        if mem!(r).b32.s0 >= MATCH_TOKEN && mem!(r).b32.s0 <= END_MATCH_TOKEN {
                            if cur_tok < LEFT_BRACE_LIMIT {
                                align_state -= 1;
                            }
                            break 'arg_scan;
                        } else {
                            continue 'arg_scan;
                        }
                    }

                    if s != r {
                        if s == TEX_NULL {
                            error_here_with_diagnostic(b"Use of \0".as_ptr() as *const c_char);
                            sprint_cs(warning_index);
                            print_cstr(b" doesn't match its definition\0".as_ptr() as *const c_char);
                            capture_to_diagnostic(std::ptr::null_mut());
                            help_ptr = 4;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(3) =
                                b"If you say, e.g., `\\def\\a1{...}', then you must always\0"
                                    .as_ptr() as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                                b"put `1' after `\\a', since control sequence names are\0".as_ptr()
                                    as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                b"made up of letters only. The macro here has not been\0".as_ptr()
                                    as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                b"followed by the required stuff, so I'm ignoring it.\0".as_ptr()
                                    as *const c_char;
                            error();
                            scanner_status = (save_scanner_status) as u8;
                            warning_index = save_warning_index;
                            return;
                        } else {
                            t = s;
                            loop {
                                q = get_avail();
                                mem!(p).b32.s1 = q;
                                mem!(q).b32.s0 = mem!(t).b32.s0;
                                p = q;
                                m += 1;
                                u = mem!(t).b32.s1;
                                v = s;
                                let mut advance_r = false;
                                loop {
                                    if u == r {
                                        if cur_tok != mem!(v).b32.s0 {
                                            break;
                                        } else {
                                            r = mem!(v).b32.s1;
                                            advance_r = true;
                                            break;
                                        }
                                    }
                                    if mem!(u).b32.s0 != mem!(v).b32.s0 {
                                        break;
                                    }
                                    u = LLIST_link!(u);
                                    v = LLIST_link!(v);
                                }
                                if advance_r {
                                    continue 'arg_scan;
                                }
                                t = LLIST_link!(t);
                                if t == r {
                                    break;
                                }
                            }
                            r = s;
                        }
                    }

                    if cur_tok == par_token {
                        if long_state != (LONG_CALL) as u8 {
                            if long_state == (CALL) as u8 {
                                runaway();
                                error_here_with_diagnostic(
                                    b"Paragraph ended before \0".as_ptr() as *const c_char
                                );
                                sprint_cs(warning_index);
                                print_cstr(b" was complete\0".as_ptr() as *const c_char);
                                capture_to_diagnostic(std::ptr::null_mut());
                                help_ptr = 3;
                                *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                                    b"I suspect you've forgotten a `}', causing me to apply this\0"
                                        .as_ptr()
                                        as *const c_char;
                                *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                    b"control sequence to too much text. How can we recover?\0"
                                        .as_ptr()
                                        as *const c_char;
                                *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                    b"My plan is to forget the whole thing and hope for the best.\0"
                                        .as_ptr()
                                        as *const c_char;
                                back_error();
                            }
                            *(&raw mut pstack).cast::<i32>().offset(n as isize) =
                                mem!(TEMP_HEAD).b32.s1;
                            align_state -= unbalance;
                            for m_idx in 0..=n {
                                flush_list(*(&raw mut pstack).cast::<i32>().offset(m_idx as isize));
                            }
                            scanner_status = (save_scanner_status) as u8;
                            warning_index = save_warning_index;
                            return;
                        }
                    }

                    if cur_tok < RIGHT_BRACE_LIMIT {
                        if cur_tok < LEFT_BRACE_LIMIT {
                            unbalance = 1;
                            loop {
                                q = avail;
                                if q == TEX_NULL {
                                    q = get_avail();
                                } else {
                                    avail = mem!(q).b32.s1;
                                    mem!(q).b32.s1 = TEX_NULL;
                                }
                                mem!(p).b32.s1 = q;
                                mem!(q).b32.s0 = cur_tok;
                                p = q;

                                get_token();
                                if cur_tok == par_token {
                                    if long_state != (LONG_CALL) as u8 {
                                        if long_state == (CALL) as u8 {
                                            runaway();
                                            error_here_with_diagnostic(
                                                b"Paragraph ended before \0".as_ptr()
                                                    as *const c_char,
                                            );
                                            sprint_cs(warning_index);
                                            print_cstr(b" was complete\0".as_ptr() as *const c_char);
                                            capture_to_diagnostic(std::ptr::null_mut());
                                            help_ptr = 3;
                                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) = b"I suspect you've forgotten a `}', causing me to apply this\0".as_ptr() as *const c_char;
                                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) = b"control sequence to too much text. How can we recover?\0".as_ptr() as *const c_char;
                                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) = b"My plan is to forget the whole thing and hope for the best.\0".as_ptr() as *const c_char;
                                            back_error();
                                        }
                                        *(&raw mut pstack).cast::<i32>().offset(n as isize) =
                                            mem!(TEMP_HEAD).b32.s1;
                                        align_state -= unbalance;
                                        for m_idx in 0..=n {
                                            flush_list(
                                                *(&raw mut pstack)
                                                    .cast::<i32>()
                                                    .offset(m_idx as isize),
                                            );
                                        }
                                        scanner_status = (save_scanner_status) as u8;
                                        warning_index = save_warning_index;
                                        return;
                                    }
                                }
                                if cur_tok < RIGHT_BRACE_LIMIT {
                                    if cur_tok < LEFT_BRACE_LIMIT {
                                        unbalance += 1;
                                    } else {
                                        unbalance -= 1;
                                        if unbalance == 0 {
                                            break;
                                        }
                                    }
                                }
                            }
                            rbrace_ptr = p;
                            q = get_avail();
                            mem!(p).b32.s1 = q;
                            mem!(q).b32.s0 = cur_tok;
                            p = q;
                        } else {
                            back_input();
                            error_here_with_diagnostic(b"Argument of \0".as_ptr() as *const c_char);
                            sprint_cs(warning_index);
                            print_cstr(b" has an extra }\0".as_ptr() as *const c_char);
                            capture_to_diagnostic(std::ptr::null_mut());
                            help_ptr = 6;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(5) =
                                b"I've run across a `}' that doesn't seem to match anything.\0"
                                    .as_ptr() as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(4) =
                                b"For example, `\\def\\a#1{...}' and `\\a}' would produce\0"
                                    .as_ptr() as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(3) =
                                b"this error. If you simply proceed now, the `\\par' that\0"
                                    .as_ptr() as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                                b"I've just inserted will cause me to report a runaway\0".as_ptr()
                                    as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                b"argument that might be the root of the problem. But if\0".as_ptr()
                                    as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                b"your `}' was spurious, just type `2' and it will go away.\0"
                                    .as_ptr() as *const c_char;
                            align_state += 1;
                            long_state = (CALL) as u8;
                            cur_tok = par_token;
                            ins_error();
                            continue 'arg_scan;
                        }
                    } else {
                        if cur_tok == SPACE_TOKEN {
                            if mem!(r).b32.s0 <= END_MATCH_TOKEN && mem!(r).b32.s0 >= MATCH_TOKEN {
                                continue 'arg_scan;
                            }
                        }
                        q = get_avail();
                        mem!(p).b32.s1 = q;
                        mem!(q).b32.s0 = cur_tok;
                        p = q;
                    }

                    m += 1;
                    if mem!(r).b32.s0 > END_MATCH_TOKEN || mem!(r).b32.s0 < MATCH_TOKEN {
                        continue 'arg_scan;
                    }
                    break 'arg_scan;
                }

                if s != TEX_NULL {
                    if m == 1 && mem!(p).b32.s0 < RIGHT_BRACE_LIMIT {
                        mem!(rbrace_ptr).b32.s1 = TEX_NULL;
                        mem!(p).b32.s1 = avail;
                        avail = p;
                        p = mem!(TEMP_HEAD).b32.s1;
                        *(&raw mut pstack).cast::<i32>().offset(n as isize) = mem!(p).b32.s1;
                        mem!(p).b32.s1 = avail;
                        avail = p;
                    } else {
                        *(&raw mut pstack).cast::<i32>().offset(n as isize) =
                            mem!(TEMP_HEAD).b32.s1;
                    }
                    n += 1;
                    if INTPAR!(INT_PAR__tracing_macros) > 0 {
                        if INTPAR!(INT_PAR__tracing_stack_levels) == 0
                            || input_ptr < INTPAR!(INT_PAR__tracing_stack_levels)
                        {
                            begin_diagnostic();
                            diagnostic_begin_capture_warning_here();
                            print_nl(match_chr as i32);
                            print_int(n as i32);
                            print_cstr(b"<-\0".as_ptr() as *const c_char);
                            show_token_list(
                                *(&raw mut pstack).cast::<i32>().offset((n - 1) as isize),
                                TEX_NULL,
                                1000,
                            );
                            capture_to_diagnostic(std::ptr::null_mut());
                            end_diagnostic(false);
                        }
                    }
                }

                if mem!(r).b32.s0 == END_MATCH_TOKEN {
                    break 'match_loop;
                }
            }
        }

        while cur_input.state == (TOKEN_LIST) as u16
            && cur_input.loc == TEX_NULL
            && cur_input.index != (V_TEMPLATE) as u16
        {
            end_token_list();
        }

        begin_token_list(ref_count, (MACRO) as u16);
        cur_input.name = warning_index;
        cur_input.loc = mem!(r).b32.s1;

        if n > 0 {
            if param_ptr + n as i32 > max_param_stack {
                max_param_stack = param_ptr + n as i32;
                if max_param_stack > param_size {
                    overflow(
                        b"parameter stack size\0".as_ptr() as *const c_char,
                        (param_size as usize) as i32,
                    );
                }
            }
            for m_idx in 0..=(n - 1) {
                *param_stack.offset((param_ptr + m_idx as i32) as isize) =
                    *(&raw mut pstack).cast::<i32>().offset(m_idx as isize);
            }
            param_ptr += n as i32;
        }

        scanner_status = (save_scanner_status) as u8;
        warning_index = save_warning_index;
    }
}

#[unsafe(export_name = "insert_relax")]
pub unsafe extern "C" fn insert_relax() {
    unsafe {
        cur_tok = CS_TOKEN_FLAG + cur_cs;
        back_input();
        cur_tok = CS_TOKEN_FLAG + FROZEN_RELAX;
        back_input();
        cur_input.index = (INSERTED) as u16;
    }
}

#[unsafe(export_name = "new_index")]
pub unsafe extern "C" fn new_index(i: u16, q: i32) {
    unsafe {
        cur_ptr = get_node(INDEX_NODE_SIZE);
        mem!(cur_ptr).b16.s1 = i;
        mem!(cur_ptr).b16.s0 = 0;
        mem!(cur_ptr).b32.s1 = q;
        let mut k: i32 = 1;
        while k <= (INDEX_NODE_SIZE - 1) {
            mem!(cur_ptr + k) = sa_null;
            k += 1;
        }
    }
}

#[unsafe(export_name = "find_sa_element")]
pub unsafe extern "C" fn find_sa_element(t: small_number, n: i32, w: bool) {
    unsafe {
        let mut q: i32;
        let mut i: small_number;

        cur_ptr = *(&raw mut sa_root).cast::<i32>().offset(t as isize);
        if cur_ptr == TEX_NULL {
            if !w {
                return;
            }
            new_index(t as u16, TEX_NULL);
            *(&raw mut sa_root).cast::<i32>().offset(t as isize) = cur_ptr;
            q = cur_ptr;
            i = (n / 0x40000) as small_number;
            new_index(i as u16, q);
            if (i & 1) != 0 {
                mem!(q + (i as i32 / 2) + 1).b32.s1 = cur_ptr;
            } else {
                mem!(q + (i as i32 / 2) + 1).b32.s0 = cur_ptr;
            }
            mem!(q).b16.s0 += 1;
            q = cur_ptr;
            i = ((n / 4096) % 64) as small_number;
            new_index(i as u16, q);
            if (i & 1) != 0 {
                mem!(q + (i as i32 / 2) + 1).b32.s1 = cur_ptr;
            } else {
                mem!(q + (i as i32 / 2) + 1).b32.s0 = cur_ptr;
            }
            mem!(q).b16.s0 += 1;
            q = cur_ptr;
            i = ((n / 64) % 64) as small_number;
            new_index(i as u16, q);
            if (i & 1) != 0 {
                mem!(q + (i as i32 / 2) + 1).b32.s1 = cur_ptr;
            } else {
                mem!(q + (i as i32 / 2) + 1).b32.s0 = cur_ptr;
            }
            mem!(q).b16.s0 += 1;
            q = cur_ptr;
            i = (n % 64) as small_number;
            init_leaf(t, i, q, n);
            return;
        }

        q = cur_ptr;
        i = (n / 0x40000) as small_number;
        cur_ptr = if (i & 1) != 0 {
            mem!(q + (i as i32 / 2) + 1).b32.s1
        } else {
            mem!(q + (i as i32 / 2) + 1).b32.s0
        };
        if cur_ptr == TEX_NULL {
            if !w {
                return;
            }
            new_index(i as u16, q);
            if (i & 1) != 0 {
                mem!(q + (i as i32 / 2) + 1).b32.s1 = cur_ptr;
            } else {
                mem!(q + (i as i32 / 2) + 1).b32.s0 = cur_ptr;
            }
            mem!(q).b16.s0 += 1;
            q = cur_ptr;
            i = ((n / 4096) % 64) as small_number;
            new_index(i as u16, q);
            if (i & 1) != 0 {
                mem!(q + (i as i32 / 2) + 1).b32.s1 = cur_ptr;
            } else {
                mem!(q + (i as i32 / 2) + 1).b32.s0 = cur_ptr;
            }
            mem!(q).b16.s0 += 1;
            q = cur_ptr;
            i = ((n / 64) % 64) as small_number;
            new_index(i as u16, q);
            if (i & 1) != 0 {
                mem!(q + (i as i32 / 2) + 1).b32.s1 = cur_ptr;
            } else {
                mem!(q + (i as i32 / 2) + 1).b32.s0 = cur_ptr;
            }
            mem!(q).b16.s0 += 1;
            q = cur_ptr;
            i = (n % 64) as small_number;
            init_leaf(t, i, q, n);
            return;
        }

        q = cur_ptr;
        i = ((n / 4096) % 64) as small_number;
        cur_ptr = if (i & 1) != 0 {
            mem!(q + (i as i32 / 2) + 1).b32.s1
        } else {
            mem!(q + (i as i32 / 2) + 1).b32.s0
        };
        if cur_ptr == TEX_NULL {
            if !w {
                return;
            }
            new_index(i as u16, q);
            if (i & 1) != 0 {
                mem!(q + (i as i32 / 2) + 1).b32.s1 = cur_ptr;
            } else {
                mem!(q + (i as i32 / 2) + 1).b32.s0 = cur_ptr;
            }
            mem!(q).b16.s0 += 1;
            q = cur_ptr;
            i = ((n / 64) % 64) as small_number;
            new_index(i as u16, q);
            if (i & 1) != 0 {
                mem!(q + (i as i32 / 2) + 1).b32.s1 = cur_ptr;
            } else {
                mem!(q + (i as i32 / 2) + 1).b32.s0 = cur_ptr;
            }
            mem!(q).b16.s0 += 1;
            q = cur_ptr;
            i = (n % 64) as small_number;
            init_leaf(t, i, q, n);
            return;
        }

        q = cur_ptr;
        i = ((n / 64) % 64) as small_number;
        cur_ptr = if (i & 1) != 0 {
            mem!(q + (i as i32 / 2) + 1).b32.s1
        } else {
            mem!(q + (i as i32 / 2) + 1).b32.s0
        };
        if cur_ptr == TEX_NULL {
            if !w {
                return;
            }
            new_index(i as u16, q);
            if (i & 1) != 0 {
                mem!(q + (i as i32 / 2) + 1).b32.s1 = cur_ptr;
            } else {
                mem!(q + (i as i32 / 2) + 1).b32.s0 = cur_ptr;
            }
            mem!(q).b16.s0 += 1;
            q = cur_ptr;
            i = (n % 64) as small_number;
            init_leaf(t, i, q, n);
            return;
        }

        q = cur_ptr;
        i = (n % 64) as small_number;
        cur_ptr = if (i & 1) != 0 {
            mem!(q + (i as i32 / 2) + 1).b32.s1
        } else {
            mem!(q + (i as i32 / 2) + 1).b32.s0
        };
        if cur_ptr == TEX_NULL && w {
            init_leaf(t, i, q, n);
        }

        unsafe fn init_leaf(t: small_number, i: small_number, q: i32, n: i32) {
            unsafe {
                if t == MARK_VAL as small_number {
                    cur_ptr = get_node(MARK_CLASS_NODE_SIZE);
                    mem!(cur_ptr + 1) = sa_null;
                    mem!(cur_ptr + 2) = sa_null;
                    mem!(cur_ptr + 3) = sa_null;
                } else {
                    if (t as u16) <= DIMEN_VAL as u16 {
                        cur_ptr = get_node(WORD_NODE_SIZE);
                        mem!(cur_ptr + 2).b32.s1 = 0;
                        mem!(cur_ptr + 1).b32.s1 = n;
                    } else {
                        cur_ptr = get_node(POINTER_NODE_SIZE);
                        if (t as u16) <= MU_VAL as u16 {
                            mem!(cur_ptr + 1).b32.s1 = 0;
                            GLUE_SPEC_ref_count!(0) += 1;
                        } else {
                            mem!(cur_ptr + 1).b32.s1 = TEX_NULL;
                        }
                    }
                    mem!(cur_ptr + 1).b32.s0 = TEX_NULL;
                }
                mem!(cur_ptr).b16.s1 = (64 * t as i32 + i as i32) as u16;
                mem!(cur_ptr).b16.s0 = 1;
                mem!(cur_ptr).b32.s1 = q;
                if (i & 1) != 0 {
                    mem!(q + (i as i32 / 2) + 1).b32.s1 = cur_ptr;
                } else {
                    mem!(q + (i as i32 / 2) + 1).b32.s0 = cur_ptr;
                }
                mem!(q).b16.s0 += 1;
            }
        }
    }
}

#[unsafe(export_name = "expand")]
pub unsafe extern "C" fn expand() {
    unsafe {
        let mut t: i32;
        let b: bool;
        let mut p: i32;
        let q: i32;
        let r: i32;
        let mut j: i32;
        let cv_backup: i32;
        let cvl_backup: small_number;
        let radix_backup: small_number;
        let co_backup: small_number;
        let backup_backup: i32;
        let mut save_scanner_status: u8;

        expand_depth_count += 1;
        if expand_depth_count >= expand_depth {
            overflow(
                b"expansion depth\0".as_ptr() as *const c_char,
                (expand_depth as usize) as i32,
            );
        }

        cv_backup = cur_val;
        cvl_backup = (cur_val_level) as i16;
        radix_backup = radix as small_number;
        co_backup = cur_order as small_number;
        backup_backup = mem!(BACKUP_HEAD).b32.s1;

        'reswitch: loop {
            if cur_cmd < (CALL) as u8 {
                if INTPAR!(INT_PAR__tracing_commands) > 1 {
                    show_cur_cmd_chr();
                }

                match (cur_cmd) as i32 {
                    TOP_BOT_MARK => {
                        t = cur_chr % 5;
                        if cur_chr >= 5 {
                            scan_register_num();
                        } else {
                            cur_val = 0;
                        }

                        if cur_val == 0 {
                            cur_ptr = *(&raw mut cur_mark).cast::<i32>().offset(t as isize);
                        } else {
                            find_sa_element(MARK_VAL as small_number, cur_val, false);
                            if cur_ptr != TEX_NULL {
                                cur_ptr = if (t & 1) != 0 {
                                    mem!(cur_ptr + (t / 2) + 1).b32.s1
                                } else {
                                    mem!(cur_ptr + (t / 2) + 1).b32.s0
                                };
                            }
                        }

                        if cur_ptr != TEX_NULL {
                            begin_token_list(cur_ptr, (MARK_TEXT) as u16);
                        }
                    }
                    EXPAND_AFTER => {
                        if cur_chr == 0 {
                            get_token();
                            t = cur_tok;
                            get_token();
                            if cur_cmd > (MAX_COMMAND) as u8 {
                                expand();
                            } else {
                                back_input();
                            }
                            cur_tok = t;
                            back_input();
                        } else {
                            get_token();
                            if cur_cmd == (IF_TEST) as u8 && cur_chr != IF_CASE_CODE {
                                cur_chr += UNLESS_CODE;
                                continue 'reswitch;
                            }
                            error_here_with_diagnostic(
                                b"You can't use `\0".as_ptr() as *const c_char
                            );
                            print_esc_cstr(b"unless\0".as_ptr() as *const c_char);
                            print_cstr(b"' before `\0".as_ptr() as *const c_char);
                            print_cmd_chr((cur_cmd) as u16, cur_chr);
                            print_char(b'\'' as i32);
                            capture_to_diagnostic(std::ptr::null_mut());
                            help_ptr = 1;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                b"Continue, and I'll forget that it ever happened.\0".as_ptr()
                                    as *const c_char;
                            back_error();
                        }
                    }
                    NO_EXPAND => {
                        if cur_chr == 0 {
                            save_scanner_status = scanner_status;
                            scanner_status = (NORMAL) as u8;
                            get_token();
                            scanner_status = (save_scanner_status) as u8;
                            t = cur_tok;
                            back_input();
                            if t >= CS_TOKEN_FLAG {
                                p = get_avail();
                                mem!(p).b32.s0 = CS_TOKEN_FLAG + FROZEN_DONT_EXPAND;
                                mem!(p).b32.s1 = cur_input.loc;
                                cur_input.start = p;
                                cur_input.loc = p;
                            }
                        } else {
                            save_scanner_status = scanner_status;
                            scanner_status = (NORMAL) as u8;
                            get_token();
                            scanner_status = (save_scanner_status) as u8;
                            if cur_cs < HASH_BASE {
                                cur_cs = prim_lookup(cur_cs - SINGLE_BASE);
                            } else {
                                cur_cs = prim_lookup((*hash.offset(cur_cs as isize)).s1);
                            }
                            if cur_cs != UNDEFINED_PRIMITIVE {
                                t = eqtb!(PRIM_EQTB_BASE + cur_cs).b16.s1 as i32;
                                if t > MAX_COMMAND as i32 {
                                    cur_cmd = (t as u16) as u8;
                                    cur_chr = eqtb!(PRIM_EQTB_BASE + cur_cs).b32.s1;
                                    cur_tok = (cur_cmd as i32 * MAX_CHAR_VAL) + cur_chr;
                                    cur_cs = 0;
                                    continue 'reswitch;
                                } else {
                                    back_input();
                                    p = get_avail();
                                    mem!(p).b32.s0 = CS_TOKEN_FLAG + FROZEN_PRIMITIVE;
                                    mem!(p).b32.s1 = cur_input.loc;
                                    cur_input.loc = p;
                                    cur_input.start = p;
                                }
                            }
                        }
                    }
                    CS_NAME => {
                        let r_cs = get_avail();
                        let mut p_cs = r_cs;
                        b = is_in_csname;
                        is_in_csname = true;
                        loop {
                            get_x_token();
                            if cur_cs == 0 {
                                let q_tok = get_avail();
                                mem!(p_cs).b32.s1 = q_tok;
                                mem!(q_tok).b32.s0 = cur_tok;
                                p_cs = q_tok;
                            }
                            if cur_cs != 0 {
                                break;
                            }
                        }
                        if cur_cmd != (END_CS_NAME) as u8 {
                            error_here_with_diagnostic(b"Missing \0".as_ptr() as *const c_char);
                            print_esc_cstr(b"endcsname\0".as_ptr() as *const c_char);
                            print_cstr(b" inserted\0".as_ptr() as *const c_char);
                            capture_to_diagnostic(std::ptr::null_mut());
                            help_ptr = 2;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                b"The control sequence marked <to be read again> should\0".as_ptr()
                                    as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                b"not appear between \\csname and \\endcsname.\0".as_ptr()
                                    as *const c_char;
                            back_error();
                        }
                        is_in_csname = b;
                        j = first;
                        p = mem!(r_cs).b32.s1;
                        while p != TEX_NULL {
                            if j >= max_buf_stack {
                                max_buf_stack = j + 1;
                                if max_buf_stack == buf_size {
                                    overflow(
                                        b"buffer size\0".as_ptr() as *const c_char,
                                        (buf_size as usize) as i32,
                                    );
                                }
                            }
                            *buffer.offset(j as isize) =
                                (mem!(p).b32.s0 % MAX_CHAR_VAL) as UnicodeScalar;
                            j += 1;
                            p = LLIST_link!(p);
                        }
                        if j > first + 1 || *buffer.offset(first as isize) > 65535 {
                            no_new_control_sequence = false;
                            cur_cs = id_lookup(first, j - first);
                            no_new_control_sequence = true;
                        } else if j == first {
                            cur_cs = NULL_CS;
                        } else {
                            cur_cs = SINGLE_BASE + *buffer.offset(first as isize);
                        }
                        flush_list(r_cs);
                        if eqtb!(cur_cs).b16.s1 == (UNDEFINED_CS) as u16 {
                            eq_define(cur_cs, (RELAX) as u16, TOO_BIG_USV);
                        }
                        cur_tok = cur_cs + CS_TOKEN_FLAG;
                        back_input();
                    }
                    CONVERT => {
                        conv_toks();
                    }
                    THE => {
                        ins_the_toks();
                    }
                    IF_TEST => {
                        conditional();
                    }
                    FI_OR_ELSE => {
                        if INTPAR!(INT_PAR__tracing_ifs) > 0 {
                            if INTPAR!(INT_PAR__tracing_commands) <= 1 {
                                show_cur_cmd_chr();
                            }
                        }
                        if cur_chr > if_limit as i32 {
                            if if_limit == (IF_CODE as u16) as u8 {
                                insert_relax();
                            } else {
                                error_here_with_diagnostic(b"Extra \0".as_ptr() as *const c_char);
                                print_cmd_chr((FI_OR_ELSE) as u16, cur_chr);
                                capture_to_diagnostic(std::ptr::null_mut());
                                help_ptr = 1;
                                *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                    b"I'm ignoring this; it doesn't match any \\if.\0".as_ptr()
                                        as *const c_char;
                                error();
                            }
                        } else {
                            while cur_chr != FI_CODE {
                                pass_text();
                            }
                            if *if_stack.offset(in_open as isize) == cond_ptr {
                                if_warning();
                            }
                            p = cond_ptr;
                            if_line = mem!(p + 1).b32.s1;
                            cur_if = (mem!(p).b16.s0) as i16;
                            if_limit = (mem!(p).b16.s1) as u8;
                            cond_ptr = mem!(p).b32.s1;
                            free_node(p, IF_NODE_SIZE);
                        }
                    }
                    INPUT => {
                        if cur_chr == 1 {
                            force_eof = true;
                        } else if cur_chr == 2 {
                            pseudo_start();
                        } else if name_in_progress {
                            insert_relax();
                        } else {
                            start_input(std::ptr::null());
                        }
                    }
                    _ => {
                        error_here_with_diagnostic(
                            b"Undefined control sequence\0".as_ptr() as *const c_char
                        );
                        capture_to_diagnostic(std::ptr::null_mut());
                        help_ptr = 5;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(4) =
                            b"The control sequence at the end of the top line\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(3) =
                            b"of your error message was never \\def'ed. If you have\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                            b"misspelled it (e.g., `\\hobx'), type `I' and the correct\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                            b"spelling (e.g., `I\\hbox'). Otherwise just continue,\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                            b"and I'll forget about whatever was undefined.\0".as_ptr()
                                as *const c_char;
                        error();
                    }
                }
            } else if cur_cmd < (END_TEMPLATE) as u8 {
                macro_call();
            } else {
                cur_tok = CS_TOKEN_FLAG + FROZEN_ENDV;
                back_input();
            }
            break 'reswitch;
        }

        cur_val = cv_backup;
        cur_val_level = (cvl_backup) as u8;
        radix = (radix_backup as i32) as i16;
        cur_order = co_backup as glue_ord;
        mem!(BACKUP_HEAD).b32.s1 = backup_backup;
        expand_depth_count -= 1;
    }
}

#[unsafe(export_name = "get_x_token")]
pub unsafe extern "C" fn get_x_token() {
    unsafe {
        loop {
            get_next();
            if tt_checkpoint_enabled {
                tt_checkpoint_before_input();
            }
            if cur_cmd <= (MAX_COMMAND) as u8 {
                break;
            }
            if cur_cmd >= (CALL) as u8 {
                if cur_cmd < (END_TEMPLATE) as u8 {
                    macro_call();
                } else {
                    cur_cs = FROZEN_ENDV;
                    cur_cmd = (ENDV) as u8;
                    break;
                }
            } else {
                expand();
            }
        }
        if cur_cs == 0 {
            cur_tok = (cur_cmd as i32 * MAX_CHAR_VAL) + cur_chr;
        } else {
            cur_tok = CS_TOKEN_FLAG + cur_cs;
        }
    }
}

#[unsafe(export_name = "x_token")]
pub unsafe extern "C" fn x_token() {
    unsafe {
        while cur_cmd > (MAX_COMMAND) as u8 {
            expand();
            get_next();
        }
        if cur_cs == 0 {
            cur_tok = (cur_cmd as i32 * MAX_CHAR_VAL) + cur_chr;
        } else {
            cur_tok = CS_TOKEN_FLAG + cur_cs;
        }
    }
}

#[unsafe(export_name = "scan_left_brace")]
pub unsafe extern "C" fn scan_left_brace() {
    unsafe {
        loop {
            get_x_token();
            if cur_cmd != (SPACER) as u8 && cur_cmd != (RELAX) as u8 {
                break;
            }
        }
        if cur_cmd != (LEFT_BRACE) as u8 {
            error_here_with_diagnostic(b"Missing { inserted\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 4;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(3) =
                b"A left brace was mandatory here, so I've put one in.\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                b"You might want to delete and/or insert some corrections\0".as_ptr()
                    as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"so that I will find a matching right brace soon.\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"(If you're confused by all this, try typing `I}' now.)\0".as_ptr()
                    as *const c_char;
            back_error();
            cur_tok = LEFT_BRACE_TOKEN + b'{' as i32;
            cur_cmd = (LEFT_BRACE) as u8;
            cur_chr = b'{' as i32;
            align_state += 1;
        }
    }
}

#[unsafe(export_name = "scan_optional_equals")]
pub unsafe extern "C" fn scan_optional_equals() {
    unsafe {
        loop {
            get_x_token();
            if cur_cmd != (SPACER) as u8 {
                break;
            }
        }
        if cur_tok != OTHER_TOKEN + 61 {
            back_input();
        }
    }
}

#[unsafe(export_name = "scan_keyword")]
pub unsafe extern "C" fn scan_keyword(s: *const c_char) -> bool {
    unsafe {
        let mut p = BACKUP_HEAD;
        let mut q: i32;
        let save_cur_cs: i32;

        mem!(p).b32.s1 = TEX_NULL;
        let slen = strlen(s);

        if slen == 1 {
            let c = *s as u8 as i32;
            save_cur_cs = cur_cs;
            loop {
                get_x_token();
                if cur_cs == 0 && (cur_chr == c || cur_chr == c - 32) {
                    q = get_avail();
                    mem!(p).b32.s1 = q;
                    mem!(q).b32.s0 = cur_tok;
                    p = q;
                    flush_list(mem!(BACKUP_HEAD).b32.s1);
                    return true;
                } else if cur_cmd != (SPACER) as u8 || p != BACKUP_HEAD {
                    back_input();
                    if p != BACKUP_HEAD {
                        begin_token_list(mem!(BACKUP_HEAD).b32.s1, (BACKED_UP) as u16);
                    }
                    cur_cs = save_cur_cs;
                    return false;
                }
            }
        }

        let mut i: usize = 0;
        while i < slen {
            get_x_token();
            let sc = *s.add(i) as u8 as i32;
            if cur_cs == 0 && (cur_chr == sc || cur_chr == sc - 32) {
                q = get_avail();
                mem!(p).b32.s1 = q;
                mem!(q).b32.s0 = cur_tok;
                p = q;
                i += 1;
            } else if cur_cmd != (SPACER) as u8 || p != BACKUP_HEAD {
                back_input();
                if p != BACKUP_HEAD {
                    begin_token_list(mem!(BACKUP_HEAD).b32.s1, (BACKED_UP) as u16);
                }
                return false;
            }
        }
        flush_list(mem!(BACKUP_HEAD).b32.s1);
        true
    }
}

#[unsafe(export_name = "mu_error")]
pub unsafe extern "C" fn mu_error() {
    unsafe {
        error_here_with_diagnostic(b"Incompatible glue units\0".as_ptr() as *const c_char);
        capture_to_diagnostic(std::ptr::null_mut());
        help_ptr = 1;
        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
            b"I'm going to assume that 1mu=1pt when they're mixed.\0".as_ptr() as *const c_char;
        error();
    }
}

#[unsafe(export_name = "scan_glyph_number")]
pub unsafe extern "C" fn scan_glyph_number(f: internal_font_number) {
    unsafe {
        if scan_keyword(b"/\0".as_ptr() as *const c_char) {
            scan_and_pack_name();
            cur_val = map_glyph_to_index(f);
            cur_val_level = (INT_VAL) as u8;
        } else if scan_keyword(b"u\0".as_ptr() as *const c_char) {
            scan_char_num();
            cur_val = map_char_to_glyph(f, cur_val);
            cur_val_level = (INT_VAL) as u8;
        } else {
            scan_int();
        }
    }
}

#[unsafe(export_name = "scan_char_class")]
pub unsafe extern "C" fn scan_char_class() {
    unsafe {
        scan_int();
        if cur_val < 0 || cur_val > CHAR_CLASS_LIMIT {
            let errmsg =
                error_here_with_diagnostic(b"Bad character class\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"A character class must be between 0 and 4096.\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_char_class_not_ignored")]
pub unsafe extern "C" fn scan_char_class_not_ignored() {
    unsafe {
        scan_int();
        if cur_val < 0 || cur_val > CHAR_CLASS_LIMIT {
            let errmsg =
                error_here_with_diagnostic(b"Bad character class\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"A class for inter-character transitions must be between 0 and 4095.\0".as_ptr()
                    as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_eight_bit_int")]
pub unsafe extern "C" fn scan_eight_bit_int() {
    unsafe {
        scan_int();
        if cur_val < 0 || cur_val > 255 {
            let errmsg =
                error_here_with_diagnostic(b"Bad register code\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"A register code or char class must be between 0 and 255.\0".as_ptr()
                    as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_usv_num")]
pub unsafe extern "C" fn scan_usv_num() {
    unsafe {
        scan_int();
        if cur_val < 0 || cur_val > BIGGEST_USV {
            let errmsg =
                error_here_with_diagnostic(b"Bad character code\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"A Unicode scalar value must be between 0 and \"10FFFF.\0".as_ptr()
                    as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_char_num")]
pub unsafe extern "C" fn scan_char_num() {
    unsafe {
        scan_int();
        if cur_val < 0 || cur_val > BIGGEST_CHAR {
            let errmsg =
                error_here_with_diagnostic(b"Bad character code\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"A character number must be between 0 and 65535.\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_xetex_math_char_int")]
pub unsafe extern "C" fn scan_xetex_math_char_int() {
    unsafe {
        scan_int();
        if math_char((cur_val) as u32) == (ACTIVE_MATH_CHAR) as u32 {
            if cur_val != ACTIVE_MATH_CHAR {
                let errmsg = error_here_with_diagnostic(
                    b"Bad active XeTeX math code\0".as_ptr() as *const c_char
                );
                ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
                capture_to_diagnostic(std::ptr::null_mut());
                help_ptr = 2;
                *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                    b"Since I ignore class and family for active math chars,\0".as_ptr()
                        as *const c_char;
                *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                    b"I changed this one to \"1FFFFF.\0".as_ptr() as *const c_char;
                int_error(cur_val);
                cur_val = ACTIVE_MATH_CHAR;
            }
        } else if math_char((cur_val) as u32) > (BIGGEST_USV) as u32 {
            let errmsg = error_here_with_diagnostic(
                b"Bad XeTeX math character code\0".as_ptr() as *const c_char
            );
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"Since I expected a character number between 0 and \"10FFFF,\0".as_ptr()
                    as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_math")]
pub unsafe extern "C" fn scan_math(p: i32) {
    unsafe {
        let mut c: i32;

        'restart: loop {
            loop {
                get_x_token();
                if cur_cmd != (SPACER) as u8 && cur_cmd != (RELAX) as u8 {
                    break;
                }
            }

            'reswitch: loop {
                match (cur_cmd) as i32 {
                    11 | 12 | 68 => {
                        c = (*eqtb.offset((MATH_CODE_BASE + cur_chr) as isize)).b32.s1;
                        if math_char((c) as u32) == (ACTIVE_MATH_CHAR) as u32 {
                            cur_cs = cur_chr + 1;
                            cur_cmd = (eqtb!(cur_cs).b16.s1) as u8;
                            cur_chr = eqtb!(cur_cs).b32.s1;
                            x_token();
                            back_input();
                            continue 'restart;
                        }
                        break 'reswitch;
                    }
                    16 => {
                        scan_char_num();
                        cur_chr = cur_val;
                        cur_cmd = (CHAR_GIVEN) as u8;
                        continue 'reswitch;
                    }
                    17 => {
                        if cur_chr == 2 {
                            scan_math_class_int();
                            c = (set_class((cur_val) as u32)) as i32;
                            scan_math_fam_int();
                            c += (set_family((cur_val) as u32)) as i32;
                            scan_usv_num();
                            c += cur_val;
                        } else if cur_chr == 1 {
                            scan_xetex_math_char_int();
                            c = cur_val;
                        } else {
                            scan_fifteen_bit_int();
                            c = (set_class((cur_val / 4096) as u32)
                                + set_family(((cur_val % 4096) / 256) as u32)
                                + (cur_val % 256) as u32) as i32;
                        }
                        break 'reswitch;
                    }
                    69 => {
                        c = (set_class((cur_chr / 4096) as u32)
                            + set_family(((cur_chr % 4096) / 256) as u32)
                            + (cur_chr % 256) as u32) as i32;
                        break 'reswitch;
                    }
                    70 => {
                        c = cur_chr;
                        break 'reswitch;
                    }
                    15 => {
                        if cur_chr == 1 {
                            scan_math_class_int();
                            c = (set_class((cur_val) as u32)) as i32;
                            scan_math_fam_int();
                            c += (set_family((cur_val) as u32)) as i32;
                            scan_usv_num();
                            c += cur_val;
                        } else {
                            scan_delimiter_int();
                            let d_val = cur_val / 4096;
                            c = (set_class((d_val / 4096) as u32)
                                + set_family(((d_val % 4096) / 256) as u32)
                                + (d_val % 256) as u32) as i32;
                        }
                        break 'reswitch;
                    }
                    _ => {
                        back_input();
                        scan_left_brace();
                        (*save_stack.offset(save_ptr as isize)).b32.s1 = p;
                        save_ptr += 1;
                        push_math((MATH_GROUP) as u8);
                        return;
                    }
                }
            }

            mem!(p).b32.s1 = MATH_CHAR;
            mem!(p).b16.s0 = (c % 65536) as u16;
            if math_class((c) as u32) == 7
                && INTPAR!(INT_PAR__cur_fam) >= 0
                && INTPAR!(INT_PAR__cur_fam) < NUMBER_MATH_FAMILIES
            {
                mem!(p).b16.s1 = INTPAR!(INT_PAR__cur_fam) as u16;
            } else {
                mem!(p).b16.s1 = math_fam((c) as u32) as u16;
            }
            mem!(p).b16.s1 += ((math_char((c) as u32) / 65536) * 256) as u16;
            break 'restart;
        }
    }
}

#[unsafe(export_name = "set_math_char")]
pub unsafe extern "C" fn set_math_char(c: i32) {
    unsafe {
        let p: i32;
        let ch: UnicodeScalar;

        if math_char((c) as u32) == (ACTIVE_MATH_CHAR) as u32 {
            cur_cs = cur_chr + 1;
            cur_cmd = (eqtb!(cur_cs).b16.s1) as u8;
            cur_chr = eqtb!(cur_cs).b32.s1;
            x_token();
            back_input();
        } else {
            p = new_noad();
            mem!(p + 1).b32.s1 = MATH_CHAR;
            ch = (math_char((c) as u32)) as i32;
            mem!(p + 1).b16.s0 = (ch % 65536) as u16;
            mem!(p + 1).b16.s1 = math_fam((c) as u32) as u16;
            if math_class((c) as u32) == 7 {
                if INTPAR!(INT_PAR__cur_fam) >= 0
                    && INTPAR!(INT_PAR__cur_fam) < NUMBER_MATH_FAMILIES
                {
                    mem!(p + 1).b16.s1 = INTPAR!(INT_PAR__cur_fam) as u16;
                }
                mem!(p).b16.s1 = (ORD_NOAD) as u16;
            } else {
                mem!(p).b16.s1 = (ORD_NOAD as i32 + (math_class((c) as u32)) as i32) as u16;
            }
            mem!(p + 1).b16.s1 += ((ch / 65536) * 256) as u16;
            mem!(cur_list.tail).b32.s1 = p;
            cur_list.tail = p;
        }
    }
}

#[unsafe(export_name = "scan_math_class_int")]
pub unsafe extern "C" fn scan_math_class_int() {
    unsafe {
        scan_int();
        if cur_val < 0 || cur_val > 7 {
            let errmsg = error_here_with_diagnostic(b"Bad math class\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"Since I expected to read a number between 0 and 7,\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_math_fam_int")]
pub unsafe extern "C" fn scan_math_fam_int() {
    unsafe {
        scan_int();
        if cur_val < 0 || cur_val > (NUMBER_MATH_FAMILIES - 1) {
            let errmsg = error_here_with_diagnostic(b"Bad math family\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"Since I expected to read a number between 0 and 255,\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_four_bit_int")]
pub unsafe extern "C" fn scan_four_bit_int() {
    unsafe {
        scan_int();
        if cur_val < 0 || cur_val > 15 {
            let errmsg = error_here_with_diagnostic(b"Bad number\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"Since I expected to read a number between 0 and 15,\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_fifteen_bit_int")]
pub unsafe extern "C" fn scan_fifteen_bit_int() {
    unsafe {
        scan_int();
        if cur_val < 0 || cur_val > 32767 {
            let errmsg = error_here_with_diagnostic(b"Bad mathchar\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"A mathchar number must be between 0 and 32767.\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_delimiter_int")]
pub unsafe extern "C" fn scan_delimiter_int() {
    unsafe {
        scan_int();
        if cur_val < 0 || cur_val > 0x7FFFFFF {
            let errmsg =
                error_here_with_diagnostic(b"Bad delimiter code\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"A numeric delimiter code must be between 0 and 2^{27}-1.\0".as_ptr()
                    as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_register_num")]
pub unsafe extern "C" fn scan_register_num() {
    unsafe {
        scan_int();
        if cur_val < 0 || cur_val > max_reg_num {
            let errmsg =
                error_here_with_diagnostic(b"Bad register code\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) = max_reg_help_line;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "scan_four_bit_int_or_18")]
pub unsafe extern "C" fn scan_four_bit_int_or_18() {
    unsafe {
        scan_int();
        if cur_val < 0 || (cur_val > 15 && cur_val != 18) {
            let errmsg = error_here_with_diagnostic(b"Bad number\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"Since I expected to read a number between 0 and 15,\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"I changed this one to zero.\0".as_ptr() as *const c_char;
            int_error(cur_val);
            cur_val = 0;
        }
    }
}

#[unsafe(export_name = "get_x_or_protected")]
pub unsafe extern "C" fn get_x_or_protected() {
    unsafe {
        loop {
            get_token();
            if cur_cmd <= (MAX_COMMAND) as u8 {
                return;
            }
            if cur_cmd >= (CALL) as u8 && cur_cmd < (END_TEMPLATE) as u8 {
                if mem!(mem!(cur_chr).b32.s1).b32.s0 == PROTECTED_TOKEN {
                    return;
                }
            }
            expand();
        }
    }
}

#[unsafe(export_name = "effective_char")]
pub unsafe extern "C" fn effective_char(err_p: bool, f: internal_font_number, mut c: u16) -> i32 {
    unsafe {
        if !xtx_ligature_present && !(*font_mapping.offset(f as isize)).is_null() {
            c = apply_tfm_font_mapping(*font_mapping.offset(f as isize), c as UnicodeScalar) as u16;
        }
        xtx_ligature_present = false;
        c as i32
    }
}

#[unsafe(export_name = "scan_font_ident")]
pub unsafe extern "C" fn scan_font_ident() {
    unsafe {
        let mut f: internal_font_number;
        let m: i32;

        loop {
            get_x_token();
            if cur_cmd != (SPACER) as u8 {
                break;
            }
        }

        if cur_cmd == (DEF_FONT) as u8 {
            f = eqtb!(CUR_FONT_LOC).b32.s1;
        } else if cur_cmd == (SET_FONT) as u8 {
            f = cur_chr;
        } else if cur_cmd == (DEF_FAMILY) as u8 {
            m = cur_chr;
            scan_math_fam_int();
            f = eqtb!(m + cur_val).b32.s1;
        } else {
            error_here_with_diagnostic(b"Missing font identifier\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"I was looking for a control sequence whose\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"current meaning has been defined by \\font.\0".as_ptr() as *const c_char;
            back_error();
            f = FONT_BASE;
        }
        cur_val = f;
    }
}

#[unsafe(export_name = "find_font_dimen")]
pub unsafe extern "C" fn find_font_dimen(writing: bool) {
    unsafe {
        let f: internal_font_number;
        let n: i32;

        scan_int();
        n = cur_val;
        scan_font_ident();
        f = cur_val;

        if n <= 0 {
            cur_val = fmem_ptr;
        } else {
            if writing
                && n <= SPACE_SHRINK_CODE
                && n >= SPACE_CODE
                && *font_glue.offset(f as isize) != TEX_NULL
            {
                delete_glue_ref(*font_glue.offset(f as isize));
                *font_glue.offset(f as isize) = TEX_NULL;
            }
            if n > *font_params.offset(f as isize) {
                if f < font_ptr {
                    cur_val = fmem_ptr;
                } else {
                    loop {
                        if fmem_ptr == font_mem_size {
                            overflow(
                                b"font memory\0".as_ptr() as *const c_char,
                                (font_mem_size as usize) as i32,
                            );
                        }
                        (*font_info.offset(fmem_ptr as isize)).b32.s1 = 0;
                        fmem_ptr += 1;
                        *font_params.offset(f as isize) += 1;
                        if n == *font_params.offset(f as isize) {
                            break;
                        }
                    }
                    cur_val = fmem_ptr - 1;
                }
            } else {
                cur_val = n + *param_base.offset(f as isize);
            }
        }

        if cur_val == fmem_ptr {
            error_here_with_diagnostic(b"Font \0".as_ptr() as *const c_char);
            print_esc((*hash.offset((FONT_ID_BASE + f) as isize)).s1);
            print_cstr(b" has only \0".as_ptr() as *const c_char);
            print_int(*font_params.offset(f as isize));
            print_cstr(b" fontdimen parameters\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"To increase the number of font parameters, you must\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"use \\fontdimen immediately after the \\font is loaded.\0".as_ptr()
                    as *const c_char;
            error();
        }
    }
}

#[unsafe(export_name = "scan_something_internal")]
pub unsafe extern "C" fn scan_something_internal(level: small_number, negative: bool) {
    unsafe {
        let mut m: i32;
        let mut n: i32;
        let mut k: i32;
        let kk: i32;
        let mut q: i32;
        let mut r: i32;
        let mut tx: i32;
        let i: b16x4;
        let mut p: i32;

        'restart: loop {
            m = cur_chr;

            match (cur_cmd) as i32 {
                DEF_CODE => {
                    scan_usv_num();
                    if m == MATH_CODE_BASE {
                        cur_val1 = (*eqtb.offset((MATH_CODE_BASE + cur_val) as isize)).b32.s1;
                        if math_char((cur_val1) as u32) == (ACTIVE_MATH_CHAR) as u32 {
                            cur_val1 = 0x8000;
                        } else if math_class((cur_val1) as u32) > 7
                            || math_fam((cur_val1) as u32) > 15
                            || math_char((cur_val1) as u32) > 255
                        {
                            let errmsg = error_here_with_diagnostic(
                                b"Extended mathchar used as mathchar\0".as_ptr() as *const c_char,
                            );
                            ttstub_diag_printf(
                                errmsg,
                                b" (%d)\0".as_ptr() as *const c_char,
                                cur_val1,
                            );
                            capture_to_diagnostic(std::ptr::null_mut());
                            help_ptr = 2;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                b"A mathchar number must be between 0 and \"7FFF.\0".as_ptr()
                                    as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                b"I changed this one to zero.\0".as_ptr() as *const c_char;
                            int_error(cur_val1);
                            cur_val1 = 0;
                        }
                        cur_val1 = (math_class((cur_val1) as u32) * 0x1000
                            + math_fam((cur_val1) as u32) * 0x100
                            + math_char((cur_val1) as u32))
                            as i32;
                        cur_val = cur_val1;
                        cur_val_level = (INT_VAL) as u8;
                    } else if m == DEL_CODE_BASE {
                        cur_val1 = (*eqtb.offset((DEL_CODE_BASE + cur_val) as isize)).b32.s1;
                        if cur_val1 >= 0x40000000 {
                            error_here_with_diagnostic(
                                b"Extended delcode used as delcode\0".as_ptr() as *const c_char,
                            );
                            capture_to_diagnostic(std::ptr::null_mut());
                            help_ptr = 2;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                b"I can only go up to 2147483647='17777777777=\"7FFFFFFF,\0"
                                    .as_ptr() as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                b"I changed this one to zero.\0".as_ptr() as *const c_char;
                            error();
                            cur_val = 0;
                            cur_val_level = (INT_VAL) as u8;
                        } else {
                            cur_val = cur_val1;
                            cur_val_level = (INT_VAL) as u8;
                        }
                    } else if m < SF_CODE_BASE {
                        cur_val = eqtb!(m + cur_val).b32.s1;
                        cur_val_level = (INT_VAL) as u8;
                    } else if m < MATH_CODE_BASE {
                        cur_val = eqtb!(m + cur_val).b32.s1 % 65536;
                        cur_val_level = (INT_VAL) as u8;
                    } else {
                        cur_val = eqtb!(m + cur_val).b32.s1;
                        cur_val_level = (INT_VAL) as u8;
                    }
                }
                XETEX_DEF_CODE => {
                    scan_usv_num();
                    if m == SF_CODE_BASE {
                        cur_val = (*eqtb.offset((SF_CODE_BASE + cur_val) as isize)).b32.s1 / 65536;
                        cur_val_level = (INT_VAL) as u8;
                    } else if m == MATH_CODE_BASE {
                        cur_val = (*eqtb.offset((MATH_CODE_BASE + cur_val) as isize)).b32.s1;
                        cur_val_level = (INT_VAL) as u8;
                    } else if m == MATH_CODE_BASE + 1 {
                        error_here_with_diagnostic(
                            b"Can't use \\Umathcode as a number (try \\Umathcodenum)\0".as_ptr()
                                as *const c_char,
                        );
                        capture_to_diagnostic(std::ptr::null_mut());
                        help_ptr = 2;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                            b"\\Umathcode is for setting a mathcode from separate values;\0"
                                .as_ptr() as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                            b"use \\Umathcodenum to access them as single values.\0".as_ptr()
                                as *const c_char;
                        error();
                        cur_val = 0;
                        cur_val_level = (INT_VAL) as u8;
                    } else if m == DEL_CODE_BASE {
                        cur_val = (*eqtb.offset((DEL_CODE_BASE + cur_val) as isize)).b32.s1;
                        cur_val_level = (INT_VAL) as u8;
                    } else {
                        error_here_with_diagnostic(
                            b"Can't use \\Udelcode as a number (try \\Udelcodenum)\0".as_ptr()
                                as *const c_char,
                        );
                        capture_to_diagnostic(std::ptr::null_mut());
                        help_ptr = 2;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                            b"\\Udelcode is for setting a delcode from separate values;\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                            b"use \\Udelcodenum to access them as single values.\0".as_ptr()
                                as *const c_char;
                        error();
                        cur_val = 0;
                        cur_val_level = (INT_VAL) as u8;
                    }
                }
                TOKS_REGISTER | ASSIGN_TOKS | DEF_FAMILY | SET_FONT | DEF_FONT => {
                    if level != TOK_VAL as small_number {
                        error_here_with_diagnostic(
                            b"Missing number, treated as zero\0".as_ptr() as *const c_char
                        );
                        capture_to_diagnostic(std::ptr::null_mut());
                        help_ptr = 3;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                            b"A number should have been here; I inserted `0'.\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                            b"(If you can't figure out why I needed to see a number,\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                            b"look up `weird error' in the index to The TeXbook.)\0".as_ptr()
                                as *const c_char;
                        back_error();
                        cur_val = 0;
                        cur_val_level = (DIMEN_VAL) as u8;
                    } else if cur_cmd <= (ASSIGN_TOKS) as u8 {
                        if cur_cmd < (ASSIGN_TOKS) as u8 {
                            if m == 0 {
                                scan_register_num();
                                if cur_val < 256 {
                                    cur_val = (*eqtb.offset((TOKS_BASE + cur_val) as isize)).b32.s1;
                                } else {
                                    find_sa_element(TOK_VAL as small_number, cur_val, false);
                                    if cur_ptr == TEX_NULL {
                                        cur_val = TEX_NULL;
                                    } else {
                                        cur_val = mem!(cur_ptr + 1).b32.s1;
                                    }
                                }
                            } else {
                                cur_val = mem!(m + 1).b32.s1;
                            }
                        } else if cur_chr == LOCAL_BASE + LOCAL__xetex_inter_char_toks {
                            scan_char_class_not_ignored();
                            cur_ptr = cur_val;
                            scan_char_class_not_ignored();
                            find_sa_element(
                                INTER_CHAR_VAL as small_number,
                                cur_ptr * CHAR_CLASS_LIMIT + cur_val,
                                false,
                            );
                            if cur_ptr == TEX_NULL {
                                cur_val = TEX_NULL;
                            } else {
                                cur_val = mem!(cur_ptr + 1).b32.s1;
                            }
                        } else {
                            cur_val = eqtb!(m).b32.s1;
                        }
                        cur_val_level = (TOK_VAL) as u8;
                    } else {
                        back_input();
                        scan_font_ident();
                        cur_val = FONT_ID_BASE + cur_val;
                        cur_val_level = (IDENT_VAL) as u8;
                    }
                }
                ASSIGN_INT => {
                    cur_val = eqtb!(m).b32.s1;
                    cur_val_level = (INT_VAL) as u8;
                }
                ASSIGN_DIMEN => {
                    cur_val = eqtb!(m).b32.s1;
                    cur_val_level = (DIMEN_VAL) as u8;
                }
                ASSIGN_GLUE => {
                    cur_val = eqtb!(m).b32.s1;
                    cur_val_level = (GLUE_VAL) as u8;
                }
                ASSIGN_MU_GLUE => {
                    cur_val = eqtb!(m).b32.s1;
                    cur_val_level = (MU_VAL) as u8;
                }
                SET_AUX => {
                    if (cur_list.mode as i32).abs() != m {
                        error_here_with_diagnostic(b"Improper \0".as_ptr() as *const c_char);
                        print_cmd_chr((SET_AUX) as u16, m);
                        capture_to_diagnostic(std::ptr::null_mut());
                        help_ptr = 4;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(3) =
                            b"You can refer to \\spacefactor only in horizontal mode;\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                            b"you can refer to \\prevdepth only in vertical mode; and\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                            b"neither of these is meaningful inside \\write. So\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                            b"I'm forgetting what you said and using zero instead.\0".as_ptr()
                                as *const c_char;
                        error();
                        if level != TOK_VAL as small_number {
                            cur_val = 0;
                            cur_val_level = (DIMEN_VAL) as u8;
                        } else {
                            cur_val = 0;
                            cur_val_level = (INT_VAL) as u8;
                        }
                    } else if m == VMODE as i32 {
                        cur_val = cur_list.aux.b32.s1;
                        cur_val_level = (DIMEN_VAL) as u8;
                    } else {
                        cur_val = cur_list.aux.b32.s0;
                        cur_val_level = (INT_VAL) as u8;
                    }
                }
                SET_PREV_GRAF => {
                    if cur_list.mode == 0 {
                        cur_val = 0;
                        cur_val_level = (INT_VAL) as u8;
                    } else {
                        *nest.offset(nest_ptr as isize) = cur_list;
                        p = nest_ptr;
                        while (nest.offset(p as isize).read().mode as i32).abs() != VMODE as i32 {
                            p -= 1;
                        }
                        cur_val = (*nest.offset(p as isize)).prev_graf;
                        cur_val_level = (INT_VAL) as u8;
                    }
                }
                SET_PAGE_INT => {
                    if m == 0 {
                        cur_val = dead_cycles;
                    } else if m == 2 {
                        cur_val = (interaction) as i32;
                    } else {
                        cur_val = insert_penalties;
                    }
                    cur_val_level = (INT_VAL) as u8;
                }
                SET_PAGE_DIMEN => {
                    if page_contents == (EMPTY) as u8 && !output_active {
                        if m == 0 {
                            cur_val = MAX_HALFWORD;
                        } else {
                            cur_val = 0;
                        }
                    } else {
                        cur_val = *(&raw mut page_so_far).cast::<scaled_t>().offset(m as isize);
                    }
                    cur_val_level = (DIMEN_VAL) as u8;
                }
                SET_SHAPE => {
                    if m > LOCAL_BASE + LOCAL__par_shape {
                        scan_int();
                        if eqtb!(m).b32.s1 == TEX_NULL || cur_val < 0 {
                            cur_val = 0;
                        } else {
                            if cur_val > mem!(eqtb!(m).b32.s1 + 1).b32.s1 {
                                cur_val = mem!(eqtb!(m).b32.s1 + 1).b32.s1;
                            }
                            cur_val = mem!(eqtb!(m).b32.s1 + cur_val + 1).b32.s1;
                        }
                    } else if eqtb!(LOCAL_BASE + LOCAL__par_shape).b32.s1 == TEX_NULL {
                        cur_val = 0;
                    } else {
                        cur_val = mem!(eqtb!(LOCAL_BASE + LOCAL__par_shape).b32.s1).b32.s0;
                    }
                    cur_val_level = (INT_VAL) as u8;
                }
                SET_BOX_DIMEN => {
                    scan_register_num();
                    if cur_val < 256 {
                        q = eqtb!(BOX_BASE + cur_val).b32.s1;
                    } else {
                        find_sa_element(4, cur_val, false);
                        if cur_ptr == TEX_NULL {
                            q = TEX_NULL;
                        } else {
                            q = mem!(cur_ptr + 1).b32.s1;
                        }
                    }
                    if q == TEX_NULL {
                        cur_val = 0;
                    } else {
                        cur_val = mem!(q + m).b32.s1;
                    }
                    cur_val_level = (DIMEN_VAL) as u8;
                }
                CHAR_GIVEN | MATH_GIVEN | XETEX_MATH_GIVEN => {
                    cur_val = cur_chr;
                    cur_val_level = (INT_VAL) as u8;
                }
                ASSIGN_FONT_DIMEN => {
                    find_font_dimen(false);
                    (*font_info.offset(fmem_ptr as isize)).b32.s1 = 0;
                    cur_val = (*font_info.offset(cur_val as isize)).b32.s1;
                    cur_val_level = (DIMEN_VAL) as u8;
                }
                ASSIGN_FONT_INT => {
                    scan_font_ident();
                    if m == 0 {
                        cur_val = *hyphen_char.offset(cur_val as isize);
                        cur_val_level = (INT_VAL) as u8;
                    } else if m == 1 {
                        cur_val = *skew_char.offset(cur_val as isize);
                        cur_val_level = (INT_VAL) as u8;
                    } else {
                        n = cur_val;
                        if *font_area.offset(n as isize) == (AAT_FONT_FLAG) as i32
                            || *font_area.offset(n as isize) == (OTGR_FONT_FLAG) as i32
                        {
                            scan_glyph_number(n);
                        } else {
                            scan_char_num();
                        }
                        k = cur_val;
                        match (m) as i32 {
                            LP_CODE_BASE => {
                                cur_val = get_cp_code(n, (k as u16) as u32, LEFT_SIDE);
                                cur_val_level = (INT_VAL) as u8;
                            }
                            RP_CODE_BASE => {
                                cur_val = get_cp_code(n, (k as u16) as u32, RIGHT_SIDE);
                                cur_val_level = (INT_VAL) as u8;
                            }
                            _ => {}
                        }
                    }
                }
                REGISTER => {
                    if m < 0 || m > 19 {
                        cur_val_level = ((mem!(m).b16.s1 / 64) as small_number) as u8;
                        if (cur_val_level as u16) < GLUE_VAL as u16 {
                            cur_val = mem!(m + 2).b32.s1;
                        } else {
                            cur_val = mem!(m + 1).b32.s1;
                        }
                    } else {
                        scan_register_num();
                        cur_val_level = (m as small_number) as u8;
                        if cur_val > 255 {
                            find_sa_element((cur_val_level) as i16, cur_val, false);
                            if cur_ptr == TEX_NULL {
                                cur_val = 0;
                            } else if (cur_val_level as u16) < GLUE_VAL as u16 {
                                cur_val = mem!(cur_ptr + 2).b32.s1;
                            } else {
                                cur_val = mem!(cur_ptr + 1).b32.s1;
                            }
                        } else {
                            match (cur_val_level as u16) as i32 {
                                INT_VAL => cur_val = COUNT_REG!(cur_val),
                                DIMEN_VAL => {
                                    cur_val =
                                        (*eqtb.offset((SCALED_BASE + cur_val) as isize)).b32.s1
                                }
                                GLUE_VAL => {
                                    cur_val = (*eqtb.offset((SKIP_BASE + cur_val) as isize)).b32.s1
                                }
                                MU_VAL => {
                                    cur_val =
                                        (*eqtb.offset((MU_SKIP_BASE + cur_val) as isize)).b32.s1
                                }
                                _ => {}
                            }
                        }
                    }
                }
                LAST_ITEM => {
                    if m >= INPUT_LINE_NO_CODE {
                        if m >= ETEX_GLUE {
                            if m < ETEX_MU {
                                match (m) as i32 {
                                    MU_TO_GLUE_CODE => scan_mu_glue(),
                                    _ => {}
                                }
                                cur_val_level = (GLUE_VAL) as u8;
                            } else if m < ETEX_EXPR {
                                match (m) as i32 {
                                    GLUE_TO_MU_CODE => scan_normal_glue(),
                                    _ => {}
                                }
                                cur_val_level = (MU_VAL) as u8;
                            } else {
                                cur_val_level = ((m - ETEX_EXPR) as small_number) as u8;
                                scan_expr();
                            }

                            while cur_val_level > (level) as u8 {
                                if cur_val_level as u16 == (GLUE_VAL) as u16 {
                                    m = cur_val;
                                    cur_val = mem!(m + 1).b32.s1;
                                    delete_glue_ref(m);
                                } else if cur_val_level as u16 == (MU_VAL) as u16 {
                                    mu_error();
                                }
                                cur_val_level -= 1;
                            }

                            if negative {
                                if (cur_val_level as u16) >= (GLUE_VAL) as u16 {
                                    m = cur_val;
                                    cur_val = new_spec(m);
                                    delete_glue_ref(m);
                                    mem!(cur_val + 1).b32.s1 = -mem!(cur_val + 1).b32.s1;
                                    mem!(cur_val + 2).b32.s1 = -mem!(cur_val + 2).b32.s1;
                                    mem!(cur_val + 3).b32.s1 = -mem!(cur_val + 3).b32.s1;
                                } else {
                                    cur_val = -cur_val;
                                }
                            }
                            return;
                        }

                        if m >= XETEX_DIM {
                            match (m) as i32 {
                                XETEX_GLYPH_BOUNDS_CODE => {
                                    if *font_area.offset(eqtb!(CUR_FONT_LOC).b32.s1 as isize)
                                        == (AAT_FONT_FLAG) as i32
                                        || *font_area.offset(eqtb!(CUR_FONT_LOC).b32.s1 as isize)
                                            == (OTGR_FONT_FLAG) as i32
                                    {
                                        scan_int();
                                        n = cur_val;
                                        if n < 1 || n > 4 {
                                            error_here_with_diagnostic(b"\\\\XeTeXglyphbounds requires an edge index from 1 to 4;\0".as_ptr() as *const c_char);
                                            print_nl_cstr(
                                                b"I don't know anything about edge \0".as_ptr()
                                                    as *const c_char,
                                            );
                                            print_int(n);
                                            capture_to_diagnostic(std::ptr::null_mut());
                                            error();
                                            cur_val = 0;
                                        } else {
                                            scan_int();
                                            cur_val = get_glyph_bounds(
                                                eqtb!(CUR_FONT_LOC).b32.s1,
                                                n,
                                                (cur_val as u32) as i32,
                                            );
                                        }
                                    } else {
                                        not_native_font_error(
                                            LAST_ITEM as i32,
                                            m,
                                            eqtb!(CUR_FONT_LOC).b32.s1,
                                        );
                                        cur_val = 0;
                                    }
                                }
                                FONT_CHAR_WD_CODE | FONT_CHAR_HT_CODE | FONT_CHAR_DP_CODE
                                | FONT_CHAR_IC_CODE => {
                                    scan_font_ident();
                                    q = cur_val;
                                    scan_usv_num();
                                    if *font_area.offset(q as isize) == (AAT_FONT_FLAG) as i32
                                        || *font_area.offset(q as isize) == (OTGR_FONT_FLAG) as i32
                                    {
                                        match (m) as i32 {
                                            FONT_CHAR_WD_CODE => {
                                                cur_val = getnativecharwd(q, cur_val)
                                            }
                                            FONT_CHAR_HT_CODE => {
                                                cur_val = getnativecharht(q, cur_val)
                                            }
                                            FONT_CHAR_DP_CODE => {
                                                cur_val = getnativechardp(q, cur_val)
                                            }
                                            FONT_CHAR_IC_CODE => {
                                                cur_val = getnativecharic(q, cur_val)
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        if *font_bc.offset(q as isize) as i32 <= cur_val
                                            && *font_ec.offset(q as isize) as i32 >= cur_val
                                        {
                                            let info = (*font_info.offset(
                                                (*char_base.offset(q as isize)
                                                    + effective_char(true, q, cur_val as u16))
                                                    as isize,
                                            ))
                                            .b16;
                                            match (m) as i32 {
                                                FONT_CHAR_WD_CODE => {
                                                    cur_val = (*font_info.offset(
                                                        (*width_base.offset(q as isize)
                                                            + info.s3 as i32)
                                                            as isize,
                                                    ))
                                                    .b32
                                                    .s1
                                                }
                                                FONT_CHAR_HT_CODE => {
                                                    cur_val = (*font_info.offset(
                                                        (*height_base.offset(q as isize)
                                                            + (info.s2 as i32 / 16))
                                                            as isize,
                                                    ))
                                                    .b32
                                                    .s1
                                                }
                                                FONT_CHAR_DP_CODE => {
                                                    cur_val = (*font_info.offset(
                                                        (*depth_base.offset(q as isize)
                                                            + (info.s2 as i32 % 16))
                                                            as isize,
                                                    ))
                                                    .b32
                                                    .s1
                                                }
                                                FONT_CHAR_IC_CODE => {
                                                    cur_val = (*font_info.offset(
                                                        (*italic_base.offset(q as isize)
                                                            + (info.s1 as i32 / 4))
                                                            as isize,
                                                    ))
                                                    .b32
                                                    .s1
                                                }
                                                _ => {}
                                            }
                                        } else {
                                            cur_val = 0;
                                        }
                                    }
                                }
                                PAR_SHAPE_LENGTH_CODE
                                | PAR_SHAPE_INDENT_CODE
                                | PAR_SHAPE_DIMEN_CODE => {
                                    q = cur_chr - PAR_SHAPE_LENGTH_CODE;
                                    scan_int();
                                    if eqtb!(LOCAL_BASE + LOCAL__par_shape).b32.s1 == TEX_NULL
                                        || cur_val <= 0
                                    {
                                        cur_val = 0;
                                    } else {
                                        if q == 2 {
                                            q = cur_val % 2;
                                            cur_val = (cur_val + q) / 2;
                                        }
                                        if cur_val
                                            > mem!(eqtb!(LOCAL_BASE + LOCAL__par_shape).b32.s1)
                                                .b32
                                                .s0
                                        {
                                            cur_val =
                                                mem!(eqtb!(LOCAL_BASE + LOCAL__par_shape).b32.s1)
                                                    .b32
                                                    .s0;
                                        }
                                        cur_val = mem!(
                                            eqtb!(LOCAL_BASE + LOCAL__par_shape).b32.s1
                                                + 2 * cur_val
                                                - q
                                        )
                                        .b32
                                        .s1;
                                    }
                                    cur_val_level = (DIMEN_VAL) as u8;
                                }
                                GLUE_STRETCH_CODE | GLUE_SHRINK_CODE => {
                                    scan_normal_glue();
                                    q = cur_val;
                                    if m == GLUE_STRETCH_CODE {
                                        cur_val = mem!(q + 2).b32.s1;
                                    } else {
                                        cur_val = mem!(q + 3).b32.s1;
                                    }
                                    delete_glue_ref(q);
                                }
                                _ => {}
                            }
                            cur_val_level = (DIMEN_VAL) as u8;
                        } else {
                            match (m) as i32 {
                                INPUT_LINE_NO_CODE => cur_val = line,
                                BADNESS_CODE => cur_val = last_badness,
                                ELAPSED_TIME_CODE => cur_val = get_microinterval(),
                                RANDOM_SEED_CODE => cur_val = random_seed,
                                PDF_SHELL_ESCAPE_CODE => {
                                    cur_val = if shell_escape_enabled { 1 } else { 0 }
                                }
                                ETEX_VERSION_CODE => cur_val = ETEX_VERSION,
                                XETEX_VERSION_CODE => cur_val = XETEX_VERSION,
                                XETEX_COUNT_GLYPHS_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (AAT_FONT_FLAG) as i32 {
                                        cur_val = aat_font_get(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                        );
                                    } else if *font_area.offset(n as isize)
                                        == (OTGR_FONT_FLAG) as i32
                                    {
                                        cur_val = ot_font_get(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                        );
                                    } else {
                                        cur_val = 0;
                                    }
                                }
                                XETEX_COUNT_FEATURES_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (AAT_FONT_FLAG) as i32 {
                                        cur_val = aat_font_get(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                        );
                                    } else if *font_area.offset(n as isize)
                                        == (OTGR_FONT_FLAG) as i32
                                        && usingGraphite(*font_layout_engine.offset(n as isize))
                                    {
                                        cur_val = ot_font_get(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                        );
                                    } else {
                                        cur_val = 0;
                                    }
                                }
                                XETEX_VARIATION_CODE
                                | XETEX_VARIATION_MIN_CODE
                                | XETEX_VARIATION_MAX_CODE
                                | XETEX_VARIATION_DEFAULT_CODE
                                | XETEX_COUNT_VARIATIONS_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    cur_val = 0;
                                }
                                XETEX_FEATURE_CODE_CODE
                                | XETEX_IS_EXCLUSIVE_FEATURE_CODE
                                | XETEX_COUNT_SELECTORS_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (AAT_FONT_FLAG) as i32 {
                                        scan_int();
                                        k = cur_val;
                                        cur_val = aat_font_get_1(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                            k,
                                        );
                                    } else if *font_area.offset(n as isize)
                                        == (OTGR_FONT_FLAG) as i32
                                        && usingGraphite(*font_layout_engine.offset(n as isize))
                                    {
                                        scan_int();
                                        k = cur_val;
                                        cur_val = ot_font_get_1(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                            k,
                                        );
                                    } else {
                                        not_aat_gr_font_error(LAST_ITEM as i32, m, n);
                                        cur_val = -1;
                                    }
                                }
                                XETEX_SELECTOR_CODE_CODE | XETEX_IS_DEFAULT_SELECTOR_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (AAT_FONT_FLAG) as i32 {
                                        scan_int();
                                        k = cur_val;
                                        scan_int();
                                        cur_val = aat_font_get_2(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                            k,
                                            cur_val,
                                        );
                                    } else if *font_area.offset(n as isize)
                                        == (OTGR_FONT_FLAG) as i32
                                        && usingGraphite(*font_layout_engine.offset(n as isize))
                                    {
                                        scan_int();
                                        k = cur_val;
                                        scan_int();
                                        cur_val = ot_font_get_2(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                            k,
                                            cur_val,
                                        );
                                    } else {
                                        not_aat_gr_font_error(LAST_ITEM as i32, m, n);
                                        cur_val = -1;
                                    }
                                }
                                XETEX_FIND_VARIATION_BY_NAME_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (AAT_FONT_FLAG) as i32 {
                                        scan_and_pack_name();
                                        cur_val = aat_font_get_named(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                        );
                                    } else {
                                        not_aat_font_error(LAST_ITEM as i32, m, n);
                                        cur_val = -1;
                                    }
                                }
                                XETEX_FIND_FEATURE_BY_NAME_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (AAT_FONT_FLAG) as i32 {
                                        scan_and_pack_name();
                                        cur_val = aat_font_get_named(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                        );
                                    } else if *font_area.offset(n as isize)
                                        == (OTGR_FONT_FLAG) as i32
                                        && usingGraphite(*font_layout_engine.offset(n as isize))
                                    {
                                        scan_and_pack_name();
                                        cur_val = gr_font_get_named(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                        );
                                    } else {
                                        not_aat_gr_font_error(LAST_ITEM as i32, m, n);
                                        cur_val = -1;
                                    }
                                }
                                XETEX_FIND_SELECTOR_BY_NAME_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (AAT_FONT_FLAG) as i32 {
                                        scan_int();
                                        k = cur_val;
                                        scan_and_pack_name();
                                        cur_val = aat_font_get_named_1(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                            k,
                                        );
                                    } else if *font_area.offset(n as isize)
                                        == (OTGR_FONT_FLAG) as i32
                                        && usingGraphite(*font_layout_engine.offset(n as isize))
                                    {
                                        scan_int();
                                        k = cur_val;
                                        scan_and_pack_name();
                                        cur_val = gr_font_get_named_1(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                            k,
                                        );
                                    } else {
                                        not_aat_gr_font_error(LAST_ITEM as i32, m, n);
                                        cur_val = -1;
                                    }
                                }
                                XETEX_OT_COUNT_SCRIPTS_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (OTGR_FONT_FLAG) as i32
                                        && usingOpenType(*font_layout_engine.offset(n as isize))
                                    {
                                        cur_val = ot_font_get(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                        );
                                    } else {
                                        cur_val = 0;
                                    }
                                }
                                XETEX_OT_COUNT_LANGUAGES_CODE | XETEX_OT_SCRIPT_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (OTGR_FONT_FLAG) as i32
                                        && usingOpenType(*font_layout_engine.offset(n as isize))
                                    {
                                        scan_int();
                                        cur_val = ot_font_get_1(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                            cur_val,
                                        );
                                    } else {
                                        not_ot_font_error(LAST_ITEM as i32, m, n);
                                        cur_val = -1;
                                    }
                                }
                                XETEX_OT_COUNT_FEATURES_CODE | XETEX_OT_LANGUAGE_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (OTGR_FONT_FLAG) as i32
                                        && usingOpenType(*font_layout_engine.offset(n as isize))
                                    {
                                        scan_int();
                                        k = cur_val;
                                        scan_int();
                                        cur_val = ot_font_get_2(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                            k,
                                            cur_val,
                                        );
                                    } else {
                                        not_ot_font_error(LAST_ITEM as i32, m, n);
                                        cur_val = -1;
                                    }
                                }
                                XETEX_OT_FEATURE_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (OTGR_FONT_FLAG) as i32
                                        && usingOpenType(*font_layout_engine.offset(n as isize))
                                    {
                                        scan_int();
                                        k = cur_val;
                                        scan_int();
                                        let kk_val = cur_val;
                                        scan_int();
                                        cur_val = ot_font_get_3(
                                            m - XETEX_INT,
                                            *font_layout_engine.offset(n as isize),
                                            k,
                                            kk_val,
                                            cur_val,
                                        );
                                    } else {
                                        not_ot_font_error(LAST_ITEM as i32, m, n);
                                        cur_val = -1;
                                    }
                                }
                                XETEX_MAP_CHAR_TO_GLYPH_CODE => {
                                    if *font_area.offset(eqtb!(CUR_FONT_LOC).b32.s1 as isize)
                                        == (AAT_FONT_FLAG) as i32
                                        || *font_area.offset(eqtb!(CUR_FONT_LOC).b32.s1 as isize)
                                            == (OTGR_FONT_FLAG) as i32
                                    {
                                        scan_int();
                                        n = cur_val;
                                        cur_val = map_char_to_glyph(eqtb!(CUR_FONT_LOC).b32.s1, n);
                                    } else {
                                        not_native_font_error(
                                            LAST_ITEM as i32,
                                            m,
                                            eqtb!(CUR_FONT_LOC).b32.s1,
                                        );
                                        cur_val = 0;
                                    }
                                }
                                XETEX_GLYPH_INDEX_CODE => {
                                    if *font_area.offset(eqtb!(CUR_FONT_LOC).b32.s1 as isize)
                                        == (AAT_FONT_FLAG) as i32
                                        || *font_area.offset(eqtb!(CUR_FONT_LOC).b32.s1 as isize)
                                            == (OTGR_FONT_FLAG) as i32
                                    {
                                        scan_and_pack_name();
                                        cur_val = map_glyph_to_index(eqtb!(CUR_FONT_LOC).b32.s1);
                                    } else {
                                        not_native_font_error(
                                            LAST_ITEM as i32,
                                            m,
                                            eqtb!(CUR_FONT_LOC).b32.s1,
                                        );
                                        cur_val = 0;
                                    }
                                }
                                XETEX_FONT_TYPE_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (AAT_FONT_FLAG) as i32 {
                                        cur_val = 1;
                                    } else if *font_area.offset(n as isize)
                                        == (OTGR_FONT_FLAG) as i32
                                        && usingOpenType(*font_layout_engine.offset(n as isize))
                                    {
                                        cur_val = 2;
                                    } else if *font_area.offset(n as isize)
                                        == (OTGR_FONT_FLAG) as i32
                                        && usingGraphite(*font_layout_engine.offset(n as isize))
                                    {
                                        cur_val = 3;
                                    } else {
                                        cur_val = 0;
                                    }
                                }
                                XETEX_FIRST_CHAR_CODE | XETEX_LAST_CHAR_CODE => {
                                    scan_font_ident();
                                    n = cur_val;
                                    if *font_area.offset(n as isize) == (AAT_FONT_FLAG) as i32
                                        || *font_area.offset(n as isize) == (OTGR_FONT_FLAG) as i32
                                    {
                                        cur_val = get_font_char_range(
                                            n,
                                            (m == XETEX_FIRST_CHAR_CODE) as i32,
                                        );
                                    } else {
                                        if m == XETEX_FIRST_CHAR_CODE {
                                            cur_val = *font_bc.offset(n as isize) as i32;
                                        } else {
                                            cur_val = *font_ec.offset(n as isize) as i32;
                                        }
                                    }
                                }
                                PDF_LAST_X_POS_CODE => cur_val = pdf_last_x_pos,
                                PDF_LAST_Y_POS_CODE => cur_val = pdf_last_y_pos,
                                XETEX_PDF_PAGE_COUNT_CODE => {
                                    scan_and_pack_name();
                                    cur_val = count_pdf_file_pages();
                                }
                                CURRENT_GROUP_LEVEL_CODE => cur_val = cur_level as i32 - 1,
                                CURRENT_GROUP_TYPE_CODE => cur_val = cur_group as i32,
                                CURRENT_IF_LEVEL_CODE => {
                                    q = cond_ptr;
                                    cur_val = 0;
                                    while q != TEX_NULL {
                                        cur_val += 1;
                                        q = LLIST_link!(q);
                                    }
                                }
                                CURRENT_IF_TYPE_CODE => {
                                    if cond_ptr == TEX_NULL {
                                        cur_val = 0;
                                    } else if cur_if < (UNLESS_CODE as u16) as i16 {
                                        cur_val = cur_if as i32 + 1;
                                    } else {
                                        cur_val = -(cur_if as i32 - 31);
                                    }
                                }
                                CURRENT_IF_BRANCH_CODE => {
                                    if if_limit == (OR_CODE as u16) as u8
                                        || if_limit == (ELSE_CODE as u16) as u8
                                    {
                                        cur_val = 1;
                                    } else if if_limit == (FI_CODE as u16) as u8 {
                                        cur_val = -1;
                                    } else {
                                        cur_val = 0;
                                    }
                                }
                                GLUE_STRETCH_ORDER_CODE | GLUE_SHRINK_ORDER_CODE => {
                                    scan_normal_glue();
                                    q = cur_val;
                                    if m == GLUE_STRETCH_ORDER_CODE {
                                        cur_val = mem!(q).b16.s1 as i32;
                                    } else {
                                        cur_val = mem!(q).b16.s0 as i32;
                                    }
                                    delete_glue_ref(q);
                                }
                                _ => {}
                            }
                            cur_val_level = (INT_VAL) as u8;
                        }
                    } else {
                        cur_val = 0;
                        tx = cur_list.tail;
                        if tx < hi_mem_min {
                            if NODE_type!(tx) == (MATH_NODE) as u16
                                && mem!(tx).b16.s0 == (END_M_CODE) as u16
                            {
                                r = cur_list.head;
                                loop {
                                    q = r;
                                    r = mem!(q).b32.s1;
                                    if r == tx {
                                        break;
                                    }
                                }
                                tx = q;
                            }
                        }

                        if cur_chr == LAST_NODE_TYPE_CODE {
                            cur_val_level = (INT_VAL) as u8;
                            if tx == cur_list.head || cur_list.mode == 0 {
                                cur_val = -1;
                            }
                        } else {
                            cur_val_level = (cur_chr as small_number) as u8;
                        }

                        if tx < hi_mem_min && cur_list.mode != 0 {
                            match (cur_chr) as i32 {
                                x if x == INT_VAL as i32 => {
                                    if NODE_type!(tx) == (PENALTY_NODE) as u16 {
                                        cur_val = mem!(tx + 1).b32.s1;
                                    }
                                }
                                x if x == DIMEN_VAL as i32 => {
                                    if NODE_type!(tx) == (KERN_NODE) as u16 {
                                        cur_val = mem!(tx + 1).b32.s1;
                                    }
                                }
                                x if x == GLUE_VAL as i32 => {
                                    if NODE_type!(tx) == (GLUE_NODE) as u16 {
                                        cur_val = mem!(tx + 1).b32.s0;
                                        if mem!(tx).b16.s0 == (MU_GLUE) as u16 {
                                            cur_val_level = (MU_VAL) as u8;
                                        }
                                    }
                                }
                                LAST_NODE_TYPE_CODE => {
                                    if NODE_type!(tx) <= (UNSET_NODE) as u16 {
                                        cur_val = mem!(tx).b16.s1 as i32 + 1;
                                    } else {
                                        cur_val = UNSET_NODE as i32 + 2;
                                    }
                                }
                                _ => {}
                            }
                        } else if cur_list.mode == VMODE as i16 && tx == cur_list.head {
                            match (cur_chr) as i32 {
                                x if x == INT_VAL as i32 => cur_val = last_penalty,
                                x if x == DIMEN_VAL as i32 => cur_val = last_kern,
                                x if x == GLUE_VAL as i32 => {
                                    if last_glue != MAX_HALFWORD {
                                        cur_val = last_glue;
                                    }
                                }
                                LAST_NODE_TYPE_CODE => cur_val = last_node_type,
                                _ => {}
                            }
                        }
                    }
                }
                IGNORE_SPACES => {
                    if cur_chr == 1 {
                        get_token();
                        if cur_cs < HASH_BASE {
                            cur_cs = prim_lookup(cur_cs - SINGLE_BASE);
                        } else {
                            cur_cs = prim_lookup((*hash.offset(cur_cs as isize)).s1);
                        }
                        if cur_cs != UNDEFINED_PRIMITIVE {
                            cur_cmd = (eqtb!(PRIM_EQTB_BASE + cur_cs).b16.s1) as u8;
                            cur_chr = eqtb!(PRIM_EQTB_BASE + cur_cs).b32.s1;
                            cur_cs = PRIM_EQTB_BASE + cur_cs;
                            cur_tok = CS_TOKEN_FLAG + cur_cs;
                        } else {
                            cur_cmd = (RELAX) as u8;
                            cur_chr = 0;
                            cur_tok = CS_TOKEN_FLAG + FROZEN_RELAX;
                            cur_cs = FROZEN_RELAX;
                        }
                        continue 'restart;
                    }
                }
                _ => {
                    error_here_with_diagnostic(b"You can't use `\0".as_ptr() as *const c_char);
                    print_cmd_chr((cur_cmd) as u16, cur_chr);
                    print_cstr(b"' after \0".as_ptr() as *const c_char);
                    print_esc_cstr(b"the\0".as_ptr() as *const c_char);
                    capture_to_diagnostic(std::ptr::null_mut());
                    help_ptr = 1;
                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                        b"I'm forgetting what you said and using zero instead.\0".as_ptr()
                            as *const c_char;
                    error();
                    cur_val = 0;
                    if level != TOK_VAL as small_number {
                        cur_val_level = (DIMEN_VAL) as u8;
                    } else {
                        cur_val_level = (INT_VAL) as u8;
                    }
                }
            }
            break 'restart;
        }

        while cur_val_level > (level) as u8 {
            if cur_val_level as u16 == (GLUE_VAL) as u16 {
                cur_val = mem!(cur_val + 1).b32.s1;
            } else if cur_val_level as u16 == (MU_VAL) as u16 {
                mu_error();
            }
            cur_val_level -= 1;
        }

        if negative {
            if (cur_val_level as u16) >= (GLUE_VAL) as u16 {
                cur_val = new_spec(cur_val);
                mem!(cur_val + 1).b32.s1 = -mem!(cur_val + 1).b32.s1;
                mem!(cur_val + 2).b32.s1 = -mem!(cur_val + 2).b32.s1;
                mem!(cur_val + 3).b32.s1 = -mem!(cur_val + 3).b32.s1;
            } else {
                cur_val = -cur_val;
            }
        } else if (cur_val_level as u16) >= (GLUE_VAL) as u16
            && (cur_val_level as u16) <= (MU_VAL) as u16
        {
            GLUE_SPEC_ref_count!(cur_val) += 1;
        }
    }
}

#[unsafe(export_name = "scan_int")]
pub unsafe extern "C" fn scan_int() {
    unsafe {
        let mut negative: bool;
        let mut m: i32;
        let mut d: u8;
        let mut vacuous: bool;
        let mut ok_so_far: bool;

        radix = 0;
        ok_so_far = true;
        negative = false;

        loop {
            loop {
                get_x_token();
                if cur_cmd != (SPACER) as u8 {
                    break;
                }
            }

            if cur_tok == OTHER_TOKEN + b'-' as i32 {
                negative = !negative;
                cur_tok = OTHER_TOKEN + b'+' as i32;
            }
            if cur_tok != OTHER_TOKEN + b'+' as i32 {
                break;
            }
        }

        loop {
            if cur_tok == ALPHA_TOKEN {
                get_token();

                if cur_tok < CS_TOKEN_FLAG {
                    cur_val = cur_chr;
                    if cur_cmd <= (RIGHT_BRACE) as u8 {
                        if cur_cmd == (RIGHT_BRACE) as u8 {
                            align_state += 1;
                        } else {
                            align_state -= 1;
                        }
                    }
                } else if cur_tok < CS_TOKEN_FLAG + SINGLE_BASE {
                    cur_val = cur_tok - (CS_TOKEN_FLAG + ACTIVE_BASE);
                } else {
                    cur_val = cur_tok - (CS_TOKEN_FLAG + SINGLE_BASE);
                }

                if cur_val > BIGGEST_USV {
                    error_here_with_diagnostic(
                        b"Improper alphabetic constant\0".as_ptr() as *const c_char
                    );
                    capture_to_diagnostic(std::ptr::null_mut());

                    help_ptr = 2;
                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                        b"A one-character control sequence belongs after a ` mark.\0".as_ptr()
                            as *const c_char;
                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                        b"So I'm essentially inserting \\0 here.\0".as_ptr() as *const c_char;
                    cur_val = b'0' as i32;
                    back_error();
                } else {
                    get_x_token();
                    if cur_cmd != (SPACER) as u8 {
                        back_input();
                    }
                }
                break;
            } else if cur_tok == CS_TOKEN_FLAG + FROZEN_PRIMITIVE {
                get_token();

                if cur_cs < HASH_BASE {
                    cur_cs = prim_lookup(cur_cs - SINGLE_BASE);
                } else {
                    cur_cs = prim_lookup((*hash.offset(cur_cs as isize)).s1);
                }

                if cur_cs != UNDEFINED_PRIMITIVE {
                    cur_cmd = ((*eqtb.offset((PRIM_EQTB_BASE + cur_cs) as isize)).b16.s1) as u8;
                    cur_chr = (*eqtb.offset((PRIM_EQTB_BASE + cur_cs) as isize)).b32.s1;
                    cur_cs = PRIM_EQTB_BASE + cur_cs;
                    cur_tok = CS_TOKEN_FLAG + cur_cs;
                } else {
                    cur_cmd = (RELAX) as u8;
                    cur_chr = 0;
                    cur_tok = CS_TOKEN_FLAG + FROZEN_RELAX;
                    cur_cs = FROZEN_RELAX;
                }
                continue;
            } else if cur_cmd >= (MIN_INTERNAL) as u8 && cur_cmd <= (MAX_INTERNAL) as u8 {
                scan_something_internal((INT_VAL) as i16, false);
                break;
            } else {
                radix = 10;
                m = 0xCCCCCCC;

                if cur_tok == OCTAL_TOKEN {
                    radix = 8;
                    m = 0x10000000;
                    get_x_token();
                } else if cur_tok == HEX_TOKEN {
                    radix = 16;
                    m = 0x8000000;
                    get_x_token();
                }

                vacuous = true;
                cur_val = 0;

                loop {
                    if cur_tok < ZERO_TOKEN + (radix) as i32
                        && cur_tok >= ZERO_TOKEN
                        && cur_tok <= ZERO_TOKEN + 9
                    {
                        d = (cur_tok - ZERO_TOKEN) as u8;
                    } else if radix == 16 {
                        if cur_tok <= A_TOKEN + 5 && cur_tok >= A_TOKEN {
                            d = (cur_tok - A_TOKEN + 10) as u8;
                        } else if cur_tok <= OTHER_A_TOKEN + 5 && cur_tok >= OTHER_A_TOKEN {
                            d = (cur_tok - OTHER_A_TOKEN + 10) as u8;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }

                    vacuous = false;

                    if cur_val >= m && (cur_val > m || d > 7 || radix != 10) {
                        if ok_so_far {
                            error_here_with_diagnostic(
                                b"Number too big\0".as_ptr() as *const c_char
                            );
                            capture_to_diagnostic(std::ptr::null_mut());

                            help_ptr = 2;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                b"I can only go up to 2147483647='17777777777=\"7FFFFFFF,\0"
                                    .as_ptr() as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                b"so I'm using that number instead of yours.\0".as_ptr()
                                    as *const c_char;
                            error();
                            cur_val = TEX_INFINITY;
                            ok_so_far = false;
                        }
                    } else {
                        cur_val = cur_val * (radix) as i32 + d as i32;
                    }

                    get_x_token();
                }

                if vacuous {
                    error_here_with_diagnostic(
                        b"Missing number, treated as zero\0".as_ptr() as *const c_char
                    );
                    capture_to_diagnostic(std::ptr::null_mut());

                    help_ptr = 3;
                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                        b"A number should have been here; I inserted `0'.\0".as_ptr()
                            as *const c_char;
                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                        b"(If you can't figure out why I needed to see a number,\0".as_ptr()
                            as *const c_char;
                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                        b"look up `weird error' in the index to The TeXbook.)\0".as_ptr()
                            as *const c_char;
                    back_error();
                } else if cur_cmd != (SPACER) as u8 {
                    back_input();
                }
                break;
            }
        }

        if negative {
            cur_val = -cur_val;
        }
    }
}

unsafe fn round_decimals(mut k: u8) -> scaled_t {
    unsafe {
        let mut a: i32 = 0;

        while k > 0 {
            k -= 1;
            a = (a + *(&raw mut dig).cast::<u8>().offset(k as isize) as i32 * 0x20000) / 10;
        }

        (a + 1) / 2
    }
}

#[unsafe(export_name = "xetex_scan_dimen")]
pub unsafe extern "C" fn xetex_scan_dimen(
    mu: bool,
    inf: bool,
    shortcut: bool,
    requires_units: bool,
) {
    unsafe {
        let mut negative: bool;
        let mut f: i32;
        let mut num: i32 = 0;
        let mut denom: i32 = 0;
        let mut k: u8;
        let mut kk: u8;
        let mut p: i32;
        let mut q: i32;
        let mut v: scaled_t = 0;
        let mut save_cur_val: i32;

        f = 0;
        arith_error = false;
        cur_order = (NORMAL) as u8;
        negative = false;

        'outer: {
            if !shortcut {
                negative = false;

                loop {
                    loop {
                        get_x_token();
                        if cur_cmd != (SPACER) as u8 {
                            break;
                        }
                    }

                    if cur_tok == OTHER_TOKEN + b'-' as i32 {
                        negative = !negative;
                        cur_tok = OTHER_TOKEN + b'+' as i32;
                    }
                    if cur_tok != OTHER_TOKEN + b'+' as i32 {
                        break;
                    }
                }

                if cur_cmd >= (MIN_INTERNAL) as u8 && cur_cmd <= (MAX_INTERNAL) as u8 {
                    if mu {
                        scan_something_internal((MU_VAL) as i16, false);
                        if cur_val_level >= (GLUE_VAL) as u8 {
                            v = mem!(cur_val + 1).b32.s1;
                            delete_glue_ref(cur_val);
                            cur_val = v;
                        }

                        if cur_val_level == (MU_VAL) as u8 {
                            break 'outer;
                        }
                        if cur_val_level != (INT_VAL) as u8 {
                            mu_error();
                        }
                    } else {
                        scan_something_internal((DIMEN_VAL) as i16, false);
                        if cur_val_level == (DIMEN_VAL) as u8 {
                            break 'outer;
                        }
                    }
                } else {
                    back_input();

                    if cur_tok == CONTINENTAL_POINT_TOKEN {
                        cur_tok = POINT_TOKEN;
                    }

                    if cur_tok != POINT_TOKEN {
                        scan_int();
                    } else {
                        radix = 10;
                        cur_val = 0;
                    }

                    if cur_tok == CONTINENTAL_POINT_TOKEN {
                        cur_tok = POINT_TOKEN;
                    }

                    if radix == 10 && cur_tok == POINT_TOKEN {
                        k = 0;
                        p = TEX_NULL;
                        get_token();

                        loop {
                            get_x_token();
                            if cur_tok > ZERO_TOKEN + 9 || cur_tok < ZERO_TOKEN {
                                break;
                            }

                            if k < 17 {
                                q = get_avail();
                                mem!(q).b32.s1 = p;
                                mem!(q).b32.s0 = cur_tok - ZERO_TOKEN;
                                p = q;
                                k += 1;
                            }
                        }

                        for kk in (1..=k).rev() {
                            *(&raw mut dig).cast::<u8>().offset((kk - 1) as isize) =
                                mem!(p).b32.s0 as u8;
                            q = p;
                            p = LLIST_link!(p);
                            mem!(q).b32.s1 = avail;
                            avail = q;
                        }

                        f = round_decimals(k);
                        if cur_cmd != (SPACER) as u8 {
                            back_input();
                        }
                    }
                }
            }

            if cur_val < 0 {
                negative = !negative;
                cur_val = -cur_val;
            }

            if requires_units {
                'units: {
                    if inf {
                        if scan_keyword(b"fil\0".as_ptr() as *const c_char) {
                            cur_order = (FIL) as u8;

                            while scan_keyword(b"l\0".as_ptr() as *const c_char) {
                                if cur_order == (FILLL) as u8 {
                                    error_here_with_diagnostic(
                                        b"Illegal unit of measure (replaced with filll)\0".as_ptr()
                                            as *const c_char,
                                    );
                                    capture_to_diagnostic(std::ptr::null_mut());

                                    help_ptr = 1;
                                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                        b"I dddon't go any higher than filll.\0".as_ptr()
                                            as *const c_char;
                                    error();
                                } else {
                                    cur_order += 1;
                                }
                            }

                            break 'units;
                        }
                    }

                    save_cur_val = cur_val;

                    loop {
                        get_x_token();
                        if cur_cmd != (SPACER) as u8 {
                            break;
                        }
                    }

                    if cur_cmd < (MIN_INTERNAL) as u8 || cur_cmd > (MAX_INTERNAL) as u8 {
                        back_input();
                    } else {
                        if mu {
                            scan_something_internal((MU_VAL) as i16, false);
                            if cur_val_level >= (GLUE_VAL) as u8 {
                                v = mem!(cur_val + 1).b32.s1;
                                delete_glue_ref(cur_val);
                                cur_val = v;
                            }
                            if cur_val_level != (MU_VAL) as u8 {
                                mu_error();
                            }
                        } else {
                            scan_something_internal((DIMEN_VAL) as i16, false);
                        }

                        v = cur_val;
                        cur_val =
                            mult_and_add(save_cur_val, v, xn_over_d(v, f, 65536), MAX_HALFWORD);
                        break 'outer;
                    }

                    if !mu {
                        let mut matched = false;
                        if scan_keyword(b"em\0".as_ptr() as *const c_char) {
                            v = (*font_info.offset(
                                (QUAD_CODE
                                    + *param_base.offset(eqtb!(CUR_FONT_LOC).b32.s1 as isize))
                                    as isize,
                            ))
                            .b32
                            .s1;
                            matched = true;
                        } else if scan_keyword(b"ex\0".as_ptr() as *const c_char) {
                            v = (*font_info.offset(
                                (X_HEIGHT_CODE
                                    + *param_base.offset(eqtb!(CUR_FONT_LOC).b32.s1 as isize))
                                    as isize,
                            ))
                            .b32
                            .s1;
                            matched = true;
                        }

                        if matched {
                            get_x_token();
                            if cur_cmd != (SPACER) as u8 {
                                back_input();
                            }
                            cur_val =
                                mult_and_add(save_cur_val, v, xn_over_d(v, f, 65536), MAX_HALFWORD);
                            break 'outer;
                        }
                    }

                    if mu {
                        if scan_keyword(b"mu\0".as_ptr() as *const c_char) {
                            break 'units;
                        } else {
                            error_here_with_diagnostic(
                                b"Illegal unit of measure (mu inserted)\0".as_ptr()
                                    as *const c_char,
                            );
                            capture_to_diagnostic(std::ptr::null_mut());

                            help_ptr = 4;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(3) =
                                b"The unit of measurement in math glue must be mu.\0".as_ptr()
                                    as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                                b"To recover gracefully from this error, it's best to\0".as_ptr()
                                    as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                b"delete the erroneous units; e.g., type `2' to delete\0".as_ptr()
                                    as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                b"two letters. (See Chapter 27 of The TeXbook.)\0".as_ptr()
                                    as *const c_char;
                            error();
                            break 'units;
                        }
                    }

                    if scan_keyword(b"true\0".as_ptr() as *const c_char) {
                        prepare_mag();
                        if INTPAR!(INT_PAR__mag) != 1000 {
                            cur_val = xn_over_d(cur_val, 1000, INTPAR!(INT_PAR__mag));
                            f = (1000 * f + 65536 * tex_remainder) / INTPAR!(INT_PAR__mag);
                            cur_val = cur_val + (f / 65536);
                            f = f % 65536;
                        }
                    }

                    if scan_keyword(b"pt\0".as_ptr() as *const c_char) {
                        break 'units;
                    }

                    let mut goto_done = false;
                    let mut goto_done2 = false;

                    if scan_keyword(b"in\0".as_ptr() as *const c_char) {
                        num = 7227;
                        denom = 100;
                    } else if scan_keyword(b"pc\0".as_ptr() as *const c_char) {
                        num = 12;
                        denom = 1;
                    } else if scan_keyword(b"cm\0".as_ptr() as *const c_char) {
                        num = 7227;
                        denom = 254;
                    } else if scan_keyword(b"mm\0".as_ptr() as *const c_char) {
                        num = 7227;
                        denom = 2540;
                    } else if scan_keyword(b"bp\0".as_ptr() as *const c_char) {
                        num = 7227;
                        denom = 7200;
                    } else if scan_keyword(b"dd\0".as_ptr() as *const c_char) {
                        num = 1238;
                        denom = 1157;
                    } else if scan_keyword(b"cc\0".as_ptr() as *const c_char) {
                        num = 14856;
                        denom = 1157;
                    } else if scan_keyword(b"sp\0".as_ptr() as *const c_char) {
                        goto_done = true;
                    } else {
                        error_here_with_diagnostic(
                            b"Illegal unit of measure (pt inserted)\0".as_ptr() as *const c_char,
                        );
                        capture_to_diagnostic(std::ptr::null_mut());

                        help_ptr = 6;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(5) =
                            b"Dimensions can be in units of em, ex, in, pt, pc,\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(4) =
                            b"cm, mm, dd, cc, bp, or sp; but yours is a new one!\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(3) =
                            b"I'll assume that you meant to say pt, for printer's points.\0"
                                .as_ptr() as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                            b"To recover gracefully from this error, it's best to\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                            b"delete the erroneous units; e.g., type `2' to delete\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                            b"two letters. (See Chapter 27 of The TeXbook.)\0".as_ptr()
                                as *const c_char;
                        error();
                        goto_done2 = true;
                    }

                    if !goto_done && !goto_done2 {
                        cur_val = xn_over_d(cur_val, num, denom);
                        f = (num * f + 65536 * tex_remainder) / denom;
                        cur_val = cur_val + (f / 65536);
                        f = f % 65536;
                    }

                    if goto_done {
                        get_x_token();
                        if cur_cmd != (SPACER) as u8 {
                            back_input();
                        }
                        break 'outer;
                    }
                }

                if cur_val >= 16384 {
                    arith_error = true;
                } else {
                    cur_val = cur_val * 65536 + f;
                }

                get_x_token();
                if cur_cmd != (SPACER) as u8 {
                    back_input();
                }
            } else {
                if cur_val >= 16384 {
                    arith_error = true;
                } else {
                    cur_val = cur_val * 65536 + f;
                }
            }
        }

        if arith_error || cur_val.abs() >= 0x40000000 {
            error_here_with_diagnostic(b"Dimension too large\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());

            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"I can't work with sizes bigger than about 19 feet.\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"Continue and I'll use the largest value I can.\0".as_ptr() as *const c_char;
            error();
            cur_val = MAX_HALFWORD;
            arith_error = false;
        }

        if negative {
            cur_val = -cur_val;
        }
    }
}

#[unsafe(export_name = "scan_dimen")]
pub unsafe extern "C" fn scan_dimen(mu: bool, inf: bool, shortcut: bool) {
    unsafe {
        xetex_scan_dimen(mu, inf, shortcut, true);
    }
}

#[unsafe(export_name = "scan_decimal")]
pub unsafe extern "C" fn scan_decimal() {
    unsafe {
        xetex_scan_dimen(false, false, false, false);
    }
}

#[unsafe(export_name = "scan_glue")]
pub unsafe extern "C" fn scan_glue(level: u8) {
    unsafe {
        let mut negative: bool;
        let q: i32;
        let mu: bool = level == (MU_VAL) as u8;

        negative = false;

        loop {
            loop {
                get_x_token();
                if cur_cmd != (SPACER) as u8 {
                    break;
                }
            }

            if cur_tok == OTHER_TOKEN + 45 {
                negative = !negative;
                cur_tok = OTHER_TOKEN + 43;
            }
            if cur_tok != OTHER_TOKEN + 43 {
                break;
            }
        }

        if cur_cmd >= (MIN_INTERNAL) as u8 && cur_cmd <= (MAX_INTERNAL) as u8 {
            scan_something_internal((level) as i16, negative);
            if cur_val_level >= (GLUE_VAL) as u8 {
                if cur_val_level != level {
                    mu_error();
                }
                return;
            }

            if cur_val_level == (INT_VAL) as u8 {
                scan_dimen(mu, false, true);
            } else if level == (MU_VAL) as u8 {
                mu_error();
            }
        } else {
            back_input();
            scan_dimen(mu, false, false);
            if negative {
                cur_val = -cur_val;
            }
        }

        q = new_spec(0);
        mem!(q + 1).b32.s1 = cur_val;

        if scan_keyword(b"plus\0".as_ptr() as *const c_char) {
            scan_dimen(mu, true, false);
            mem!(q + 2).b32.s1 = cur_val;
            mem!(q).b16.s1 = cur_order as u16;
        }

        if scan_keyword(b"minus\0".as_ptr() as *const c_char) {
            scan_dimen(mu, true, false);
            mem!(q + 3).b32.s1 = cur_val;
            mem!(q).b16.s0 = cur_order as u16;
        }

        cur_val = q;
    }
}

#[unsafe(export_name = "add_or_sub")]
pub unsafe extern "C" fn add_or_sub(x: i32, mut y: i32, max_answer: i32, negative: bool) -> i32 {
    unsafe {
        let a: i32;
        if negative {
            y = -y;
        }
        if x >= 0 {
            if y <= max_answer - x {
                a = x + y;
            } else {
                arith_error = true;
                a = 0;
            }
        } else if y >= -max_answer - x {
            a = x + y;
        } else {
            arith_error = true;
            a = 0;
        }
        a
    }
}

#[unsafe(export_name = "quotient")]
pub unsafe extern "C" fn quotient(mut n: i32, mut d: i32) -> i32 {
    unsafe {
        let mut negative: bool;
        let mut a: i32;
        if d == 0 {
            arith_error = true;
            a = 0;
        } else {
            if d > 0 {
                negative = false;
            } else {
                d = -d;
                negative = true;
            }
            if n < 0 {
                n = -n;
                negative = !negative;
            }
            a = n / d;
            n = n - a * d;
            d = n - d;
            if d + n >= 0 {
                a += 1;
            }
            if negative {
                a = -a;
            }
        }
        a
    }
}

#[unsafe(export_name = "fract")]
pub unsafe extern "C" fn fract(mut x: i32, mut n: i32, mut d: i32, max_answer: i32) -> i32 {
    unsafe {
        let mut negative: bool;
        let mut a: i32;
        let mut f: i32;
        let h: i32;
        let mut r: i32;
        let mut t: i32;

        if d == 0 {
            arith_error = true;
            return 0;
        }
        a = 0;
        if d > 0 {
            negative = false;
        } else {
            d = -d;
            negative = true;
        }
        if x < 0 {
            x = -x;
            negative = !negative;
        } else if x == 0 {
            return 0;
        }
        if n < 0 {
            n = -n;
            negative = !negative;
        }
        t = n / d;
        if t > max_answer / x {
            arith_error = true;
            return 0;
        }
        a = t * x;
        n = n - t * d;
        if n == 0 {
            if negative {
                a = -a;
            }
            return a;
        }
        t = x / d;
        if t > (max_answer - a) / n {
            arith_error = true;
            return 0;
        }
        a = a + t * n;
        x = x - t * d;
        if x == 0 {
            if negative {
                a = -a;
            }
            return a;
        }
        if x < n {
            t = x;
            x = n;
            n = t;
        }
        f = 0;
        r = (d / 2) - d;
        h = -r;

        loop {
            if odd(n) {
                r = r + x;
                if r >= 0 {
                    r = r - d;
                    f += 1;
                }
            }
            n = n / 2;
            if n == 0 {
                break;
            }
            if x < h {
                x = x + x;
            } else {
                t = x - d;
                x = t + x;
                f = f + n;
                if x < n {
                    if x == 0 {
                        break;
                    }
                    t = x;
                    x = n;
                    n = t;
                }
            }
        }

        if f > (max_answer - a) {
            arith_error = true;
            return 0;
        }
        a = a + f;
        if negative {
            a = -a;
        }
        a
    }
}

#[unsafe(export_name = "scan_expr")]
pub unsafe extern "C" fn scan_expr() {
    unsafe {
        let a: bool;
        let mut b: bool;
        let mut l: u8;
        let mut r: u8;
        let mut s: u8;
        let mut o: u8;
        let mut e: i32;
        let mut t: i32;
        let mut f: i32;
        let mut n: i32;
        let mut p: i32;
        let q: i32;

        l = cur_val_level;
        a = arith_error;
        b = false;
        p = TEX_NULL;

        expand_depth_count += 1;
        if expand_depth_count >= expand_depth {
            overflow(b"expansion depth\0".as_ptr() as *const c_char, expand_depth);
        }

        'restart: loop {
            r = EXPR_NONE as u8;
            e = 0;
            s = EXPR_NONE as u8;
            t = 0;
            n = 0;

            'continue_: loop {
                if s == EXPR_NONE as u8 {
                    o = l;
                } else {
                    o = (INT_VAL) as u8;
                }

                loop {
                    get_x_token();
                    if cur_cmd != (SPACER) as u8 {
                        break;
                    }
                }

                if cur_tok == (OTHER_TOKEN + 40) {
                    let q_node = get_node(EXPR_NODE_SIZE);
                    mem!(q_node).b32.s1 = p;
                    mem!(q_node).b16.s1 = l as u16;
                    mem!(q_node).b16.s0 = (4 * s + r) as u16;
                    mem!(q_node + 1).b32.s1 = e;
                    mem!(q_node + 2).b32.s1 = t;
                    mem!(q_node + 3).b32.s1 = n;
                    p = q_node;
                    l = o;
                    continue 'restart;
                }

                back_input();
                if o == (INT_VAL) as u8 {
                    scan_int();
                } else if o == (DIMEN_VAL) as u8 {
                    scan_dimen(false, false, false);
                } else if o == (GLUE_VAL) as u8 {
                    scan_normal_glue();
                } else {
                    scan_mu_glue();
                }
                f = cur_val;

                'found: loop {
                    loop {
                        get_x_token();
                        if cur_cmd != (SPACER) as u8 {
                            break;
                        }
                    }

                    if cur_tok == (OTHER_TOKEN + 43) {
                        o = EXPR_ADD as u8;
                    } else if cur_tok == (OTHER_TOKEN + 45) {
                        o = EXPR_SUB as u8;
                    } else if cur_tok == (OTHER_TOKEN + 42) {
                        o = EXPR_MULT as u8;
                    } else if cur_tok == (OTHER_TOKEN + 47) {
                        o = EXPR_DIV as u8;
                    } else {
                        o = EXPR_NONE as u8;
                        if p == TEX_NULL {
                            if cur_cmd != (RELAX) as u8 {
                                back_input();
                            }
                        } else if cur_tok != (OTHER_TOKEN + 41) {
                            error_here_with_diagnostic(
                                b"Missing ) inserted for expression\0".as_ptr() as *const c_char,
                            );
                            capture_to_diagnostic(std::ptr::null_mut());

                            help_ptr = 1;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                b"I was expecting to see `+', `-', `*', `/', or `)'. Didn't.\0"
                                    .as_ptr() as *const c_char;
                            back_error();
                        }
                    }

                    arith_error = b;
                    if l == (INT_VAL) as u8 || s > EXPR_SUB as u8 {
                        if f < -TEX_INFINITY {
                            arith_error = true;
                            f = 0;
                        }
                    } else if l == (DIMEN_VAL) as u8 {
                        if f.abs() > MAX_HALFWORD {
                            arith_error = true;
                            f = 0;
                        }
                    } else {
                        if mem!(f + 1).b32.s1.abs() > MAX_HALFWORD
                            || mem!(f + 2).b32.s1.abs() > MAX_HALFWORD
                            || mem!(f + 3).b32.s1.abs() > MAX_HALFWORD
                        {
                            arith_error = true;
                            delete_glue_ref(f);
                            f = new_spec(0);
                        }
                    }

                    match (s) as i32 {
                        0 => {
                            if l >= (GLUE_VAL) as u8 && o != EXPR_NONE as u8 {
                                t = new_spec(f);
                                delete_glue_ref(f);
                                if mem!(t + 2).b32.s1 == 0 {
                                    mem!(t).b16.s1 = NORMAL as u16;
                                }
                                if mem!(t + 3).b32.s1 == 0 {
                                    mem!(t).b16.s0 = NORMAL as u16;
                                }
                            } else {
                                t = f;
                            }
                        }
                        3 => {
                            if o == EXPR_DIV as u8 {
                                n = f;
                                o = EXPR_SCALE as u8;
                            } else if l == (INT_VAL) as u8 {
                                t = mult_and_add(t, f, 0, TEX_INFINITY);
                            } else if l == (DIMEN_VAL) as u8 {
                                t = mult_and_add(t, f, 0, MAX_HALFWORD);
                            } else {
                                mem!(t + 1).b32.s1 =
                                    mult_and_add(mem!(t + 1).b32.s1, f, 0, MAX_HALFWORD);
                                mem!(t + 2).b32.s1 =
                                    mult_and_add(mem!(t + 2).b32.s1, f, 0, MAX_HALFWORD);
                                mem!(t + 3).b32.s1 =
                                    mult_and_add(mem!(t + 3).b32.s1, f, 0, MAX_HALFWORD);
                            }
                        }
                        4 => {
                            if l < (GLUE_VAL) as u8 {
                                t = quotient(t, f);
                            } else {
                                mem!(t + 1).b32.s1 = quotient(mem!(t + 1).b32.s1, f);
                                mem!(t + 2).b32.s1 = quotient(mem!(t + 2).b32.s1, f);
                                mem!(t + 3).b32.s1 = quotient(mem!(t + 3).b32.s1, f);
                            }
                        }
                        5 => {
                            if l == (INT_VAL) as u8 {
                                t = fract(t, n, f, TEX_INFINITY);
                            } else if l == (DIMEN_VAL) as u8 {
                                t = fract(t, n, f, MAX_HALFWORD);
                            } else {
                                mem!(t + 1).b32.s1 = fract(mem!(t + 1).b32.s1, n, f, MAX_HALFWORD);
                                mem!(t + 2).b32.s1 = fract(mem!(t + 2).b32.s1, n, f, MAX_HALFWORD);
                                mem!(t + 3).b32.s1 = fract(mem!(t + 3).b32.s1, n, f, MAX_HALFWORD);
                            }
                        }
                        _ => {}
                    }

                    if o > EXPR_SUB as u8 {
                        s = o;
                    } else {
                        s = EXPR_NONE as u8;
                        if r == EXPR_NONE as u8 {
                            e = t;
                        } else if l == (INT_VAL) as u8 {
                            e = add_or_sub(e, t, TEX_INFINITY, r == EXPR_SUB as u8);
                        } else if l == (DIMEN_VAL) as u8 {
                            e = add_or_sub(e, t, MAX_HALFWORD, r == EXPR_SUB as u8);
                        } else {
                            mem!(e + 1).b32.s1 = add_or_sub(
                                mem!(e + 1).b32.s1,
                                mem!(t + 1).b32.s1,
                                MAX_HALFWORD,
                                r == EXPR_SUB as u8,
                            );
                            if mem!(e).b16.s1 == mem!(t).b16.s1 {
                                mem!(e + 2).b32.s1 = add_or_sub(
                                    mem!(e + 2).b32.s1,
                                    mem!(t + 2).b32.s1,
                                    MAX_HALFWORD,
                                    r == EXPR_SUB as u8,
                                );
                            } else if mem!(e).b16.s1 < mem!(t).b16.s1 && mem!(t + 2).b32.s1 != 0 {
                                mem!(e + 2).b32.s1 = mem!(t + 2).b32.s1;
                                mem!(e).b16.s1 = mem!(t).b16.s1;
                            }
                            if mem!(e).b16.s0 == mem!(t).b16.s0 {
                                mem!(e + 3).b32.s1 = add_or_sub(
                                    mem!(e + 3).b32.s1,
                                    mem!(t + 3).b32.s1,
                                    MAX_HALFWORD,
                                    r == EXPR_SUB as u8,
                                );
                            } else if mem!(e).b16.s0 < mem!(t).b16.s0 && mem!(t + 3).b32.s1 != 0 {
                                mem!(e + 3).b32.s1 = mem!(t + 3).b32.s1;
                                mem!(e).b16.s0 = mem!(t).b16.s0;
                            }
                            delete_glue_ref(t);
                            if mem!(e + 2).b32.s1 == 0 {
                                mem!(e).b16.s1 = NORMAL as u16;
                            }
                            if mem!(e + 3).b32.s1 == 0 {
                                mem!(e).b16.s0 = NORMAL as u16;
                            }
                        }
                        r = o;
                    }

                    b = arith_error;
                    if o != EXPR_NONE as u8 {
                        continue 'continue_;
                    }

                    if p != TEX_NULL {
                        f = e;
                        let q_node = p;
                        e = mem!(q_node + 1).b32.s1;
                        t = mem!(q_node + 2).b32.s1;
                        n = mem!(q_node + 3).b32.s1;
                        s = (mem!(q_node).b16.s0 / 4) as u8;
                        r = (mem!(q_node).b16.s0 % 4) as u8;
                        l = mem!(q_node).b16.s1 as u8;
                        p = mem!(q_node).b32.s1;
                        free_node(q_node, EXPR_NODE_SIZE);
                        continue 'found;
                    }

                    break 'restart;
                }
            }
        }

        expand_depth_count -= 1;

        if b {
            error_here_with_diagnostic(b"Arithmetic overflow\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());

            help_ptr = 2;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                b"I can't evaluate this expression,\0".as_ptr() as *const c_char;
            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                b"since the result is out of range.\0".as_ptr() as *const c_char;
            error();

            if l >= (GLUE_VAL) as u8 {
                delete_glue_ref(e);
                e = 0;
                GLUE_SPEC_ref_count!(e) += 1;
            } else {
                e = 0;
            }
        }

        arith_error = a;
        cur_val = e;
        cur_val_level = l;
    }
}

#[unsafe(export_name = "scan_normal_glue")]
pub unsafe extern "C" fn scan_normal_glue() {
    unsafe {
        scan_glue((GLUE_VAL) as u8);
    }
}

#[unsafe(export_name = "scan_mu_glue")]
pub unsafe extern "C" fn scan_mu_glue() {
    unsafe {
        scan_glue((MU_VAL) as u8);
    }
}

#[unsafe(export_name = "scan_rule_spec")]
pub unsafe extern "C" fn scan_rule_spec() -> i32 {
    unsafe {
        let q: i32 = new_rule();
        if cur_cmd == (VRULE) as u8 {
            mem!(q + 1).b32.s1 = DEFAULT_RULE;
        } else {
            mem!(q + 3).b32.s1 = DEFAULT_RULE;
            mem!(q + 2).b32.s1 = 0;
        }

        loop {
            if scan_keyword(b"width\0".as_ptr() as *const c_char) {
                scan_dimen(false, false, false);
                mem!(q + 1).b32.s1 = cur_val;
                continue;
            }
            if scan_keyword(b"height\0".as_ptr() as *const c_char) {
                scan_dimen(false, false, false);
                mem!(q + 3).b32.s1 = cur_val;
                continue;
            }
            if scan_keyword(b"depth\0".as_ptr() as *const c_char) {
                scan_dimen(false, false, false);
                mem!(q + 2).b32.s1 = cur_val;
                continue;
            }
            break;
        }
        q
    }
}

#[unsafe(export_name = "scan_general_text")]
pub unsafe extern "C" fn scan_general_text() {
    unsafe {
        let s: u8 = scanner_status;
        let w: i32 = warning_index;
        let d: i32 = def_ref;
        scanner_status = (ABSORBING) as u8;
        warning_index = cur_cs;
        def_ref = get_avail();
        mem!(def_ref).b32.s0 = TEX_NULL;
        let mut p: i32 = def_ref;
        scan_left_brace();
        let mut unbalance: i32 = 1;

        loop {
            get_token();
            if cur_tok < RIGHT_BRACE_LIMIT {
                if cur_cmd < (RIGHT_BRACE) as u8 {
                    unbalance += 1;
                } else {
                    unbalance -= 1;
                    if unbalance == 0 {
                        break;
                    }
                }
            }
            let q: i32 = get_avail();
            mem!(p).b32.s1 = q;
            mem!(q).b32.s0 = cur_tok;
            p = q;
        }

        let q: i32 = mem!(def_ref).b32.s1;
        mem!(def_ref).b32.s1 = avail;
        avail = def_ref;
        if q == TEX_NULL {
            cur_val = TEMP_HEAD;
        } else {
            cur_val = p;
        }
        mem!(TEMP_HEAD).b32.s1 = q;
        scanner_status = s;
        warning_index = w;
        def_ref = d;
    }
}

#[unsafe(export_name = "pseudo_start")]
pub unsafe extern "C" fn pseudo_start() {
    unsafe {
        let saved_selector: selector_t;
        let s: str_number;
        let mut l: pool_pointer;
        let mut m: pool_pointer;
        let p: i32;
        let mut q: i32;
        let mut r: i32 = 0;
        let mut w: b16x4 = std::mem::zeroed();
        let nl: i32;
        let mut sz: i32;

        scan_general_text();
        saved_selector = selector;
        selector = (SELECTOR_NEW_STRING as u8) as i32;
        token_show(TEMP_HEAD);
        selector = saved_selector;
        flush_list(mem!(TEMP_HEAD).b32.s1);

        if pool_ptr + 1 > pool_size {
            overflow(
                b"pool size\0".as_ptr() as *const c_char,
                (pool_size - init_pool_ptr) as i32,
            );
        }
        s = make_string();
        *str_pool.offset(pool_ptr as isize) = b' ' as u16;
        l = *str_start.offset((s - 65536) as isize);
        nl = INTPAR!(INT_PAR__new_line_char);
        p = get_avail();
        q = p;

        while l < pool_ptr {
            m = l;
            while l < pool_ptr && *str_pool.offset(l as isize) as i32 != nl {
                l += 1;
            }
            sz = (l - m + 7) / 4;
            if sz == 1 {
                sz = 2;
            }
            r = get_node(sz);
            mem!(q).b32.s1 = r;
            q = r;
            mem!(q).b32.s0 = sz;

            while sz > 2 {
                sz -= 1;
                r += 1;
                w.s3 = *str_pool.offset(m as isize);
                w.s2 = *str_pool.offset((m + 1) as isize);
                w.s1 = *str_pool.offset((m + 2) as isize);
                w.s0 = *str_pool.offset((m + 3) as isize);
                mem!(r).b16 = w;
                m = m + 4;
            }

            w.s3 = b' ' as u16;
            w.s2 = b' ' as u16;
            w.s1 = b' ' as u16;
            w.s0 = b' ' as u16;
            if l > m {
                w.s3 = *str_pool.offset(m as isize);
                if l > m + 1 {
                    w.s2 = *str_pool.offset((m + 1) as isize);
                    if l > m + 2 {
                        w.s1 = *str_pool.offset((m + 2) as isize);
                        if l > m + 3 {
                            w.s0 = *str_pool.offset((m + 3) as isize);
                        }
                    }
                }
            }
            mem!(r + 1).b16 = w;
            if *str_pool.offset(l as isize) as i32 == nl {
                l += 1;
            }
        }

        mem!(p).b32.s0 = mem!(p).b32.s1;
        mem!(p).b32.s1 = pseudo_files;
        pseudo_files = p;
        str_ptr -= 1;
        pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);

        begin_file_reading();
        line = 0;
        cur_input.limit = cur_input.start;
        cur_input.loc = cur_input.limit + 1;

        if INTPAR!(INT_PAR__tracing_scan_tokens) > 0 {
            if term_offset > max_print_line - 3 {
                print_ln();
            } else if term_offset > 0 || file_offset > 0 {
                print_char(b' ' as i32);
            }
            cur_input.name = 19;
            print_cstr(b"( \0".as_ptr() as *const c_char);
            open_parens += 1;
            ttstub_output_flush(rust_stdout);
        } else {
            cur_input.name = 18;
            cur_input.synctex_tag = 0;
        }
    }
}

#[unsafe(export_name = "str_toks_cat")]
pub unsafe extern "C" fn str_toks_cat(b: pool_pointer, cat: u8) -> i32 {
    unsafe {
        let mut p: i32;
        let mut q: i32;
        let mut t: i32;
        let mut k: pool_pointer;

        if pool_ptr + 1 > pool_size {
            overflow(
                b"pool size\0".as_ptr() as *const c_char,
                (pool_size - init_pool_ptr) as i32,
            );
        }

        p = TEMP_HEAD;
        LLIST_link!(p) = TEX_NULL;
        k = b;

        while k < pool_ptr {
            t = *str_pool.offset(k as isize) as i32;

            if t == b' ' as i32 && cat == 0 {
                t = SPACE_TOKEN;
            } else {
                if t >= 0xD800
                    && t < 0xDC00
                    && k + 1 < pool_ptr
                    && *str_pool.offset((k + 1) as isize) >= 0xDC00
                    && *str_pool.offset((k + 1) as isize) < 0xE000
                {
                    k += 1;
                    t = 0x10000
                        + ((t - 0xD800) << 10)
                        + (*str_pool.offset(k as isize) as i32 - 0xDC00);
                }

                if cat == 0 {
                    t = OTHER_TOKEN + t;
                } else if cat == ACTIVE_CHAR as u8 {
                    t = CS_TOKEN_FLAG + 1 + t;
                } else {
                    t = MAX_CHAR_VAL * cat as i32 + t;
                }
            }

            q = avail;
            if q == TEX_NULL {
                q = get_avail();
            } else {
                avail = LLIST_link!(q);
                LLIST_link!(q) = TEX_NULL;
            }

            LLIST_link!(p) = q;
            LLIST_info!(q) = t;
            p = q;
            k += 1;
        }

        pool_ptr = b;
        p
    }
}

#[unsafe(export_name = "str_toks")]
pub unsafe extern "C" fn str_toks(b: pool_pointer) -> i32 {
    unsafe { str_toks_cat(b, 0) }
}

#[unsafe(export_name = "the_toks")]
pub unsafe extern "C" fn the_toks() -> i32 {
    unsafe {
        let saved_selector: selector_t;
        let mut p: i32;
        let mut q: i32;
        let mut r: i32;
        let mut b: pool_pointer;
        let c: u8;

        if odd(cur_chr) {
            c = cur_chr as u8;
            scan_general_text();
            if c == 1 {
                return cur_val;
            } else {
                saved_selector = selector;
                selector = (SELECTOR_NEW_STRING as u8) as i32;
                b = pool_ptr;
                p = get_avail();
                mem!(p).b32.s1 = mem!(TEMP_HEAD).b32.s1;
                token_show(p);
                flush_list(p);
                selector = saved_selector;
                return str_toks(b);
            }
        }

        get_x_token();
        scan_something_internal((TOK_VAL) as i16, false);

        if cur_val_level >= (IDENT_VAL) as u8 {
            p = TEMP_HEAD;
            mem!(p).b32.s1 = TEX_NULL;

            if cur_val_level == (IDENT_VAL) as u8 {
                q = get_avail();
                mem!(p).b32.s1 = q;
                mem!(q).b32.s0 = CS_TOKEN_FLAG + cur_val;
                p = q;
            } else if cur_val != TEX_NULL {
                r = mem!(cur_val).b32.s1;
                while r != TEX_NULL {
                    q = avail;
                    if q == TEX_NULL {
                        q = get_avail();
                    } else {
                        avail = mem!(q).b32.s1;
                        mem!(q).b32.s1 = TEX_NULL;
                    }
                    mem!(p).b32.s1 = q;
                    mem!(q).b32.s0 = mem!(r).b32.s0;
                    p = q;
                    r = LLIST_link!(r);
                }
            }
            p
        } else {
            saved_selector = selector;
            selector = (SELECTOR_NEW_STRING as u8) as i32;
            b = pool_ptr;

            match (cur_val_level) as i32 {
                0 => {
                    print_int(cur_val);
                }
                1 => {
                    print_scaled(cur_val);
                    print_cstr(b"pt\0".as_ptr() as *const c_char);
                }
                2 => {
                    print_spec(cur_val, b"pt\0".as_ptr() as *const c_char);
                    delete_glue_ref(cur_val);
                }
                3 => {
                    print_spec(cur_val, b"mu\0".as_ptr() as *const c_char);
                    delete_glue_ref(cur_val);
                }
                _ => {}
            }

            selector = saved_selector;
            str_toks(b)
        }
    }
}

#[unsafe(export_name = "ins_the_toks")]
pub unsafe extern "C" fn ins_the_toks() {
    unsafe {
        mem!(GARBAGE).b32.s1 = the_toks();
        begin_token_list(mem!(TEMP_HEAD).b32.s1, (INSERTED) as u16);
    }
}

#[unsafe(export_name = "conv_toks")]
pub unsafe extern "C" fn conv_toks() {
    unsafe {
        let saved_selector: selector_t;
        let save_warning_index: i32;
        let save_def_ref: i32;
        let boolvar: bool;
        let mut s: str_number = 0;
        let mut u: str_number;
        let c: u8 = cur_chr as u8;
        let mut save_scanner_status: u8;
        let mut b: pool_pointer;
        let mut fnt: i32 = 0;
        let mut arg1: i32 = 0;
        let mut arg2: i32 = 0;
        let font_name_str: str_number;
        let mut quote_char: UTF16_code;
        let mut cat: u8 = 0;
        let saved_chr: UnicodeScalar;
        let mut p: i32 = TEX_NULL;
        let q: i32;
        let mut j: i32 = 0;

        match (c as i32) as i32 {
            NUMBER_CODE | ROMAN_NUMERAL_CODE => {
                scan_int();
            }
            STRING_CODE | MEANING_CODE => {
                save_scanner_status = scanner_status;
                scanner_status = (NORMAL) as u8;
                get_token();
                scanner_status = save_scanner_status;
            }
            FONT_NAME_CODE => {
                scan_font_ident();
            }
            ETEX_REVISION_CODE => {}
            EXPANDED_CODE => {
                save_scanner_status = scanner_status;
                save_warning_index = warning_index;
                save_def_ref = def_ref;
                if *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) < pool_ptr {
                    u = make_string();
                } else {
                    u = 0;
                }
                scan_pdf_ext_toks();
                warning_index = save_warning_index;
                scanner_status = save_scanner_status;
                begin_token_list(mem!(def_ref).b32.s1, (INSERTED) as u16);
                mem!(def_ref).b32.s1 = avail;
                avail = def_ref;
                def_ref = save_def_ref;
                if u != 0 {
                    str_ptr -= 1;
                }
                return;
            }
            LEFT_MARGIN_KERN_CODE | RIGHT_MARGIN_KERN_CODE => {
                scan_register_num();
                if cur_val < 256 {
                    p = eqtb!(BOX_BASE + cur_val).b32.s1;
                } else {
                    find_sa_element(4, cur_val, false);
                    if cur_ptr == TEX_NULL {
                        p = TEX_NULL;
                    } else {
                        p = mem!(cur_ptr + 1).b32.s1;
                    }
                }
                if p == TEX_NULL || NODE_type!(p) != HLIST_NODE as u16 {
                    pdf_error(
                        b"marginkern\0".as_ptr() as *const c_char,
                        b"a non-empty hbox expected\0".as_ptr() as *const c_char,
                    );
                }
            }
            PDF_CREATION_DATE_CODE => {
                b = pool_ptr;
                getcreationdate();
                mem!(GARBAGE).b32.s1 = str_toks(b);
                begin_token_list(mem!(TEMP_HEAD).b32.s1, (INSERTED) as u16);
            }
            PDF_FILE_MOD_DATE_CODE => {
                save_scanner_status = scanner_status;
                save_warning_index = warning_index;
                save_def_ref = def_ref;
                if *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) < pool_ptr {
                    u = make_string();
                } else {
                    u = 0;
                }
                scan_pdf_ext_toks();
                if selector == (SELECTOR_NEW_STRING as u8) as i32 {
                    pdf_error(
                        b"tokens\0".as_ptr() as *const c_char,
                        b"tokens_to_string() called while selector = new_string\0".as_ptr()
                            as *const c_char,
                    );
                }
                saved_selector = selector;
                selector = (SELECTOR_NEW_STRING as u8) as i32;
                show_token_list(
                    mem!(def_ref).b32.s1,
                    TEX_NULL,
                    (pool_size - pool_ptr) as i32,
                );
                selector = saved_selector;
                s = make_string();
                delete_token_ref(def_ref);
                def_ref = save_def_ref;
                warning_index = save_warning_index;
                scanner_status = save_scanner_status;
                b = pool_ptr;
                getfilemoddate(s);
                mem!(GARBAGE).b32.s1 = str_toks(b);

                if s == str_ptr - 1 {
                    str_ptr -= 1;
                    pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
                }

                begin_token_list(mem!(TEMP_HEAD).b32.s1, (INSERTED) as u16);
                if u != 0 {
                    str_ptr -= 1;
                }
                return;
            }
            PDF_FILE_SIZE_CODE => {
                save_scanner_status = scanner_status;
                save_warning_index = warning_index;
                save_def_ref = def_ref;
                if *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) < pool_ptr {
                    u = make_string();
                } else {
                    u = 0;
                }
                scan_pdf_ext_toks();
                if selector == (SELECTOR_NEW_STRING as u8) as i32 {
                    pdf_error(
                        b"tokens\0".as_ptr() as *const c_char,
                        b"tokens_to_string() called while selector = new_string\0".as_ptr()
                            as *const c_char,
                    );
                }
                saved_selector = selector;
                selector = (SELECTOR_NEW_STRING as u8) as i32;
                show_token_list(
                    mem!(def_ref).b32.s1,
                    TEX_NULL,
                    (pool_size - pool_ptr) as i32,
                );
                selector = saved_selector;
                s = make_string();
                delete_token_ref(def_ref);
                def_ref = save_def_ref;
                warning_index = save_warning_index;
                scanner_status = save_scanner_status;
                b = pool_ptr;
                getfilesize(s);
                mem!(GARBAGE).b32.s1 = str_toks(b);

                if s == str_ptr - 1 {
                    str_ptr -= 1;
                    pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
                }

                begin_token_list(mem!(TEMP_HEAD).b32.s1, (INSERTED) as u16);
                if u != 0 {
                    str_ptr -= 1;
                }
                return;
            }
            PDF_MDFIVE_SUM_CODE => {
                save_scanner_status = scanner_status;
                save_warning_index = warning_index;
                save_def_ref = def_ref;
                if *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) < pool_ptr {
                    u = make_string();
                } else {
                    u = 0;
                }
                boolvar = scan_keyword(b"file\0".as_ptr() as *const c_char);
                scan_pdf_ext_toks();
                if selector == (SELECTOR_NEW_STRING as u8) as i32 {
                    pdf_error(
                        b"tokens\0".as_ptr() as *const c_char,
                        b"tokens_to_string() called while selector = new_string\0".as_ptr()
                            as *const c_char,
                    );
                }
                saved_selector = selector;
                selector = (SELECTOR_NEW_STRING as u8) as i32;
                show_token_list(
                    mem!(def_ref).b32.s1,
                    TEX_NULL,
                    (pool_size - pool_ptr) as i32,
                );
                selector = saved_selector;
                s = make_string();
                delete_token_ref(def_ref);
                def_ref = save_def_ref;
                warning_index = save_warning_index;
                scanner_status = save_scanner_status;
                b = pool_ptr;
                getmd5sum(s, boolvar);
                mem!(GARBAGE).b32.s1 = str_toks(b);

                if s == str_ptr - 1 {
                    str_ptr -= 1;
                    pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
                }

                begin_token_list(mem!(TEMP_HEAD).b32.s1, (INSERTED) as u16);
                if u != 0 {
                    str_ptr -= 1;
                }
                return;
            }
            PDF_FILE_DUMP_CODE => {
                let mut i_dump: i32;
                save_scanner_status = scanner_status;
                save_warning_index = warning_index;
                save_def_ref = def_ref;
                if *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) < pool_ptr {
                    u = make_string();
                } else {
                    u = 0;
                }
                cur_val = 0;

                if scan_keyword(b"offset\0".as_ptr() as *const c_char) {
                    scan_int();
                    if cur_val < 0 {
                        error_here_with_diagnostic(b"Bad file offset\0".as_ptr() as *const c_char);
                        capture_to_diagnostic(std::ptr::null_mut());
                        help_ptr = 2;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                            b"A file offset must be between 0 and 2^_31_-1,\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                            b"I changed this one to zero.\0".as_ptr() as *const c_char;
                        int_error(cur_val);
                        cur_val = 0;
                    }
                }
                i_dump = cur_val;
                cur_val = 0;

                if scan_keyword(b"length\0".as_ptr() as *const c_char) {
                    scan_int();
                    if cur_val < 0 {
                        error_here_with_diagnostic(b"Bad dump length\0".as_ptr() as *const c_char);
                        capture_to_diagnostic(std::ptr::null_mut());
                        help_ptr = 2;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                            b"A dump length must be between 0 and 2^_31_-1,\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                            b"I changed this one to zero.\0".as_ptr() as *const c_char;
                        int_error(cur_val);
                        cur_val = 0;
                    }
                }
                j = cur_val;

                scan_pdf_ext_toks();
                if selector == (SELECTOR_NEW_STRING as u8) as i32 {
                    pdf_error(
                        b"tokens\0".as_ptr() as *const c_char,
                        b"tokens_to_string() called while selector = new_string\0".as_ptr()
                            as *const c_char,
                    );
                }
                saved_selector = selector;
                selector = (SELECTOR_NEW_STRING as u8) as i32;
                show_token_list(
                    mem!(def_ref).b32.s1,
                    TEX_NULL,
                    (pool_size - pool_ptr) as i32,
                );
                selector = saved_selector;
                s = make_string();
                delete_token_ref(def_ref);
                def_ref = save_def_ref;
                warning_index = save_warning_index;
                scanner_status = save_scanner_status;
                b = pool_ptr;
                getfiledump(s, i_dump, j);
                mem!(GARBAGE).b32.s1 = str_toks(b);

                if s == str_ptr - 1 {
                    str_ptr -= 1;
                    pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
                }

                begin_token_list(mem!(TEMP_HEAD).b32.s1, (INSERTED) as u16);
                if u != 0 {
                    str_ptr -= 1;
                }
                return;
            }
            PDF_STRCMP_CODE => {
                save_scanner_status = scanner_status;
                save_warning_index = warning_index;
                save_def_ref = def_ref;
                if *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) < pool_ptr {
                    u = make_string();
                } else {
                    u = 0;
                }
                compare_strings();
                def_ref = save_def_ref;
                warning_index = save_warning_index;
                scanner_status = save_scanner_status;
                if u != 0 {
                    str_ptr -= 1;
                }
            }
            XETEX_UCHAR_CODE => {
                scan_usv_num();
            }
            XETEX_UCHARCAT_CODE => {
                scan_usv_num();
                saved_chr = cur_val;
                scan_int();

                if cur_val < LEFT_BRACE
                    || cur_val > ACTIVE_CHAR
                    || cur_val == OUT_PARAM
                    || cur_val == IGNORE
                {
                    error_here_with_diagnostic(b"Invalid code (\0".as_ptr() as *const c_char);
                    print_int(cur_val);
                    print_cstr(b"), should be in the ranges 1..4, 6..8, 10..13\0".as_ptr()
                        as *const c_char);
                    capture_to_diagnostic(std::ptr::null_mut());

                    help_ptr = 1;
                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                        b"I'm going to use 12 instead of that illegal code value.\0".as_ptr()
                            as *const c_char;
                    error();
                    cat = 12;
                } else {
                    cat = cur_val as u8;
                }
                cur_val = saved_chr;
            }
            XETEX_REVISION_CODE => {}
            XETEX_VARIATION_NAME_CODE => {
                scan_font_ident();
                fnt = cur_val;
                if *font_area.offset(fnt as isize) == (AAT_FONT_FLAG) as i32 {
                    scan_int();
                    arg1 = cur_val;
                    arg2 = 0;
                } else {
                    not_aat_font_error(CONVERT, c as i32, fnt);
                }
            }
            XETEX_FEATURE_NAME_CODE => {
                scan_font_ident();
                fnt = cur_val;
                if *font_area.offset(fnt as isize) == (AAT_FONT_FLAG) as i32
                    || (*font_area.offset(fnt as isize) == (OTGR_FONT_FLAG) as i32
                        && usingGraphite(*font_layout_engine.offset(fnt as isize)))
                {
                    scan_int();
                    arg1 = cur_val;
                    arg2 = 0;
                } else {
                    not_aat_gr_font_error(CONVERT, c as i32, fnt);
                }
            }
            XETEX_SELECTOR_NAME_CODE => {
                scan_font_ident();
                fnt = cur_val;
                if *font_area.offset(fnt as isize) == (AAT_FONT_FLAG) as i32
                    || (*font_area.offset(fnt as isize) == (OTGR_FONT_FLAG) as i32
                        && usingGraphite(*font_layout_engine.offset(fnt as isize)))
                {
                    scan_int();
                    arg1 = cur_val;
                    scan_int();
                    arg2 = cur_val;
                } else {
                    not_aat_gr_font_error(CONVERT, c as i32, fnt);
                }
            }
            XETEX_GLYPH_NAME_CODE => {
                scan_font_ident();
                fnt = cur_val;
                if *font_area.offset(fnt as isize) == (AAT_FONT_FLAG) as i32
                    || *font_area.offset(fnt as isize) == (OTGR_FONT_FLAG) as i32
                {
                    scan_int();
                    arg1 = cur_val;
                } else {
                    not_native_font_error(CONVERT, c as i32, fnt);
                }
            }
            JOB_NAME_CODE => {
                if job_name == 0 {
                    open_log_file();
                }
            }
            UNIFORM_DEVIATE_CODE => {
                scan_int();
            }
            NORMAL_DEVIATE_CODE => {}
            _ => {}
        }

        saved_selector = selector;
        selector = (SELECTOR_NEW_STRING as u8) as i32;
        b = pool_ptr;

        match (c as i32) as i32 {
            NUMBER_CODE => {
                print_int(cur_val);
            }
            ROMAN_NUMERAL_CODE => {
                print_roman_int(cur_val);
            }
            STRING_CODE => {
                if cur_cs != 0 {
                    sprint_cs(cur_cs);
                } else {
                    print_char(cur_chr);
                }
            }
            MEANING_CODE => {
                print_meaning();
            }
            FONT_NAME_CODE => {
                font_name_str = *font_name.offset(cur_val as isize);
                if *font_area.offset(cur_val as isize) == (AAT_FONT_FLAG) as i32
                    || *font_area.offset(cur_val as isize) == (OTGR_FONT_FLAG) as i32
                {
                    quote_char = b'"' as UTF16_code;
                    for i in 0..(length(font_name_str) as i32) {
                        if *str_pool.offset(
                            (*str_start.offset((font_name_str - 65536) as isize) + i) as isize,
                        ) == b'"' as u16
                        {
                            quote_char = b'\'' as UTF16_code;
                        }
                    }
                    print_char(quote_char as i32);
                    print(font_name_str);
                    print_char(quote_char as i32);
                } else {
                    print(font_name_str);
                }

                if *font_size.offset(cur_val as isize) != *font_dsize.offset(cur_val as isize) {
                    print_cstr(b" at \0".as_ptr() as *const c_char);
                    print_scaled(*font_size.offset(cur_val as isize));
                    print_cstr(b"pt\0".as_ptr() as *const c_char);
                }
            }
            ETEX_REVISION_CODE => {
                print_cstr(b".6\0".as_ptr() as *const c_char);
            }
            LEFT_MARGIN_KERN_CODE => {
                p = mem!(p + 5).b32.s1;
                while p != TEX_NULL
                    && ((p < hi_mem_min
                        && (NODE_type!(p) == INS_NODE as u16
                            || NODE_type!(p) == MARK_NODE as u16
                            || NODE_type!(p) == ADJUST_NODE as u16
                            || NODE_type!(p) == PENALTY_NODE as u16
                            || (NODE_type!(p) == DISC_NODE as u16
                                && mem!(p + 1).b32.s0 == TEX_NULL
                                && mem!(p + 1).b32.s1 == TEX_NULL
                                && mem!(p).b16.s0 == 0)
                            || (NODE_type!(p) == MATH_NODE as u16 && mem!(p + 1).b32.s1 == 0)
                            || (NODE_type!(p) == KERN_NODE as u16
                                && (mem!(p + 1).b32.s1 == 0 || mem!(p).b16.s0 == NORMAL as u16))
                            || (NODE_type!(p) == GLUE_NODE as u16 && mem!(p + 1).b32.s0 == 0)
                            || (NODE_type!(p) == HLIST_NODE as u16
                                && mem!(p + 1).b32.s1 == 0
                                && mem!(p + 3).b32.s1 == 0
                                && mem!(p + 2).b32.s1 == 0
                                && mem!(p + 5).b32.s1 == TEX_NULL)))
                        || (p < hi_mem_min
                            && NODE_type!(p) == GLUE_NODE as u16
                            && mem!(p).b16.s0 == (GLUE_PAR__left_skip + 1) as u16))
                {
                    p = LLIST_link!(p);
                }

                if p != TEX_NULL
                    && p < hi_mem_min
                    && NODE_type!(p) == MARGIN_KERN_NODE as u16
                    && mem!(p).b16.s0 == 0
                {
                    print_scaled(mem!(p + 1).b32.s1);
                } else {
                    print(b'0' as i32);
                }
                print_cstr(b"pt\0".as_ptr() as *const c_char);
            }
            RIGHT_MARGIN_KERN_CODE => {
                let q_box = mem!(p + 5).b32.s1;
                p = prev_rightmost(q_box, TEX_NULL);
                while p != TEX_NULL
                    && ((p < hi_mem_min
                        && (NODE_type!(p) == INS_NODE as u16
                            || NODE_type!(p) == MARK_NODE as u16
                            || NODE_type!(p) == ADJUST_NODE as u16
                            || NODE_type!(p) == PENALTY_NODE as u16
                            || (NODE_type!(p) == DISC_NODE as u16
                                && mem!(p + 1).b32.s0 == TEX_NULL
                                && mem!(p + 1).b32.s1 == TEX_NULL
                                && mem!(p).b16.s0 == 0)
                            || (NODE_type!(p) == MATH_NODE as u16 && mem!(p + 1).b32.s1 == 0)
                            || (NODE_type!(p) == KERN_NODE as u16
                                && (mem!(p + 1).b32.s1 == 0 || mem!(p).b16.s0 == NORMAL as u16))
                            || (NODE_type!(p) == GLUE_NODE as u16 && mem!(p + 1).b32.s0 == 0)
                            || (NODE_type!(p) == HLIST_NODE as u16
                                && mem!(p + 1).b32.s1 == 0
                                && mem!(p + 3).b32.s1 == 0
                                && mem!(p + 2).b32.s1 == 0
                                && mem!(p + 5).b32.s1 == TEX_NULL)))
                        || (p < hi_mem_min
                            && NODE_type!(p) == GLUE_NODE as u16
                            && mem!(p).b16.s0 == (GLUE_PAR__right_skip + 1) as u16))
                {
                    p = prev_rightmost(q_box, p);
                }

                if p != TEX_NULL
                    && p < hi_mem_min
                    && NODE_type!(p) == MARGIN_KERN_NODE as u16
                    && mem!(p).b16.s0 == 1
                {
                    print_scaled(mem!(p + 1).b32.s1);
                } else {
                    print(b'0' as i32);
                }
                print_cstr(b"pt\0".as_ptr() as *const c_char);
            }
            PDF_STRCMP_CODE => {
                print_int(cur_val);
            }
            UNIFORM_DEVIATE_CODE => {
                print_int(unif_rand(cur_val));
            }
            NORMAL_DEVIATE_CODE => {
                print_int(norm_rand());
            }
            XETEX_UCHAR_CODE | XETEX_UCHARCAT_CODE => {
                print_char(cur_val);
            }
            XETEX_REVISION_CODE => {
                print_cstr(b".999994\0".as_ptr() as *const c_char);
            }
            XETEX_VARIATION_NAME_CODE => {
                if *font_area.offset(fnt as isize) == (AAT_FONT_FLAG) as i32 {
                    aat_print_font_name(
                        c as i32,
                        *font_layout_engine.offset(fnt as isize),
                        arg1,
                        arg2,
                    );
                }
            }
            XETEX_FEATURE_NAME_CODE | XETEX_SELECTOR_NAME_CODE => {
                if *font_area.offset(fnt as isize) == (AAT_FONT_FLAG) as i32 {
                    aat_print_font_name(
                        c as i32,
                        *font_layout_engine.offset(fnt as isize),
                        arg1,
                        arg2,
                    );
                } else if *font_area.offset(fnt as isize) == (OTGR_FONT_FLAG) as i32
                    && usingGraphite(*font_layout_engine.offset(fnt as isize))
                {
                    gr_print_font_name(
                        c as i32,
                        *font_layout_engine.offset(fnt as isize),
                        arg1,
                        arg2,
                    );
                }
            }
            XETEX_GLYPH_NAME_CODE => {
                if *font_area.offset(fnt as isize) == (AAT_FONT_FLAG) as i32
                    || *font_area.offset(fnt as isize) == (OTGR_FONT_FLAG) as i32
                {
                    print_glyph_name(fnt, arg1);
                }
            }
            JOB_NAME_CODE => {
                print_file_name(job_name, 0, 0);
            }
            _ => {}
        }

        selector = saved_selector;
        mem!(GARBAGE).b32.s1 = str_toks_cat(b, cat);
        begin_token_list(mem!(TEMP_HEAD).b32.s1, (INSERTED) as u16);
    }
}

#[unsafe(export_name = "scan_toks")]
pub unsafe extern "C" fn scan_toks(macro_def: bool, xpand: bool) -> i32 {
    unsafe {
        let mut t: i32;
        let mut s: i32;
        let mut p: i32;
        let mut q: i32;
        let mut unbalance: i32;
        let mut hash_brace: i32;

        if macro_def {
            scanner_status = (DEFINING) as u8;
        } else {
            scanner_status = (ABSORBING) as u8;
        }
        warning_index = cur_cs;
        def_ref = get_avail();
        mem!(def_ref).b32.s0 = TEX_NULL;
        p = def_ref;
        hash_brace = 0;
        t = ZERO_TOKEN;

        'header: {
            if macro_def {
                loop {
                    get_token();
                    if cur_tok < RIGHT_BRACE_LIMIT {
                        break;
                    }
                    if cur_cmd == (MAC_PARAM) as u8 {
                        s = MATCH_TOKEN + cur_chr;
                        get_token();
                        if cur_tok < LEFT_BRACE_LIMIT {
                            hash_brace = cur_tok;
                            q = get_avail();
                            mem!(p).b32.s1 = q;
                            mem!(q).b32.s0 = cur_tok;
                            p = q;

                            q = get_avail();
                            mem!(p).b32.s1 = q;
                            mem!(q).b32.s0 = END_MATCH_TOKEN;
                            p = q;
                            break 'header;
                        }

                        if t == (ZERO_TOKEN + 9) {
                            error_here_with_diagnostic(
                                b"You already have nine parameters\0".as_ptr() as *const c_char,
                            );
                            capture_to_diagnostic(std::ptr::null_mut());
                            help_ptr = 2;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                b"I'm going to ignore the # sign you just used,\0".as_ptr()
                                    as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                b"as well as the token that followed it.\0".as_ptr()
                                    as *const c_char;
                            error();
                            continue;
                        } else {
                            t += 1;
                            if cur_tok != t {
                                error_here_with_diagnostic(
                                    b"Parameters must be numbered consecutively\0".as_ptr()
                                        as *const c_char,
                                );
                                help_ptr = 2;
                                *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                    b"I've inserted the digit you should have used after the #.\0"
                                        .as_ptr()
                                        as *const c_char;
                                *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                    b"Type `1' to delete what you did use.\0".as_ptr()
                                        as *const c_char;
                                back_error();
                            }
                            cur_tok = s;
                        }
                    }

                    q = get_avail();
                    mem!(p).b32.s1 = q;
                    mem!(q).b32.s0 = cur_tok;
                    p = q;
                }

                q = get_avail();
                mem!(p).b32.s1 = q;
                mem!(q).b32.s0 = END_MATCH_TOKEN;
                p = q;

                if cur_cmd == (RIGHT_BRACE) as u8 {
                    error_here_with_diagnostic(b"Missing { inserted\0".as_ptr() as *const c_char);
                    capture_to_diagnostic(std::ptr::null_mut());
                    align_state += 1;
                    help_ptr = 2;
                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                        b"Where was the left brace? You said something like `\\def\\a}',\0".as_ptr()
                            as *const c_char;
                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                        b"which I'm going to interpret as `\\def\\a{}'.\0".as_ptr()
                            as *const c_char;
                    error();
                    // goto found
                    scanner_status = (NORMAL) as u8;
                    if hash_brace != 0 {
                        q = get_avail();
                        mem!(p).b32.s1 = q;
                        mem!(q).b32.s0 = hash_brace;
                        p = q;
                    }
                    return p;
                }
            } else {
                scan_left_brace();
            }
        }

        unbalance = 1;
        loop {
            if xpand {
                loop {
                    get_next();
                    if cur_cmd >= (CALL) as u8 {
                        if mem!(mem!(cur_chr).b32.s1).b32.s0 == PROTECTED_TOKEN {
                            cur_cmd = (RELAX) as u8;
                            cur_chr = NO_EXPAND_FLAG;
                        }
                    }
                    if cur_cmd <= (MAX_COMMAND) as u8 {
                        break;
                    }
                    if cur_cmd != (THE) as u8 {
                        expand();
                    } else {
                        q = the_toks();
                        if mem!(TEMP_HEAD).b32.s1 != TEX_NULL {
                            mem!(p).b32.s1 = mem!(TEMP_HEAD).b32.s1;
                            p = q;
                        }
                    }
                }
                x_token();
            } else {
                get_token();
            }

            if cur_tok < RIGHT_BRACE_LIMIT {
                if cur_cmd < (RIGHT_BRACE) as u8 {
                    unbalance += 1;
                } else {
                    unbalance -= 1;
                    if unbalance == 0 {
                        break;
                    }
                }
            } else if cur_cmd == (MAC_PARAM) as u8 {
                if macro_def {
                    s = cur_tok;
                    if xpand {
                        get_x_token();
                    } else {
                        get_token();
                    }
                    if cur_cmd != (MAC_PARAM) as u8 {
                        if cur_tok <= ZERO_TOKEN || cur_tok > t {
                            error_here_with_diagnostic(
                                b"Illegal parameter number in definition of \0".as_ptr()
                                    as *const c_char,
                            );
                            sprint_cs(warning_index);
                            capture_to_diagnostic(std::ptr::null_mut());
                            help_ptr = 3;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(2) =
                                b"You meant to type ## instead of #, right?\0".as_ptr()
                                    as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                                b"Or maybe a } was forgotten somewhere earlier, and things\0"
                                    .as_ptr() as *const c_char;
                            *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                                b"are all screwed up? I'm going to assume that you meant ##.\0"
                                    .as_ptr() as *const c_char;
                            back_error();
                            cur_tok = s;
                        } else {
                            cur_tok = (OUT_PARAM_TOKEN - 48) + cur_chr;
                        }
                    }
                }
            }

            q = get_avail();
            mem!(p).b32.s1 = q;
            mem!(q).b32.s0 = cur_tok;
            p = q;
        }

        scanner_status = (NORMAL) as u8;
        if hash_brace != 0 {
            q = get_avail();
            mem!(p).b32.s1 = q;
            mem!(q).b32.s0 = hash_brace;
            p = q;
        }
        p
    }
}

#[unsafe(export_name = "read_toks")]
pub unsafe extern "C" fn read_toks(n: i32, r: i32, j: i32) {
    unsafe {
        let mut p: i32;
        let mut q: i32;
        let s: i32;
        let m: usize;

        scanner_status = (DEFINING) as u8;
        warning_index = r;
        def_ref = get_avail();
        mem!(def_ref).b32.s0 = TEX_NULL;
        p = def_ref;

        q = get_avail();
        mem!(p).b32.s1 = q;
        mem!(q).b32.s0 = END_MATCH_TOKEN;
        p = q;

        if n < 0 || n > 15 {
            m = 16;
        } else {
            m = n as usize;
        }

        s = align_state;
        align_state = 1000000;

        loop {
            begin_file_reading();
            cur_input.name = (m + 1) as i32;

            if *(&raw mut read_open).cast::<u8>().offset(m as isize) == (CLOSED) as u8 {
                _tt_abort(b"terminal input forbidden\0".as_ptr() as *const c_char);
            } else if *(&raw mut read_open).cast::<u8>().offset(m as isize) == (JUST_OPEN) as u8 {
                if input_line(*(&raw mut read_file).cast::<*mut UFILE>().offset(m as isize)) != 0 {
                    *(&raw mut read_open).cast::<u8>().offset(m as isize) = NORMAL as u8;
                } else {
                    u_close(*(&raw mut read_file).cast::<*mut UFILE>().offset(m as isize));
                    *(&raw mut read_open).cast::<u8>().offset(m as isize) = (CLOSED) as u8;
                }
            } else {
                if input_line(*(&raw mut read_file).cast::<*mut UFILE>().offset(m as isize)) == 0 {
                    u_close(*(&raw mut read_file).cast::<*mut UFILE>().offset(m as isize));
                    *(&raw mut read_open).cast::<u8>().offset(m as isize) = (CLOSED) as u8;
                    if align_state != 1000000 {
                        runaway();
                        error_here_with_diagnostic(
                            b"File ended within \0".as_ptr() as *const c_char
                        );
                        print_esc_cstr(b"read\0".as_ptr() as *const c_char);
                        capture_to_diagnostic(std::ptr::null_mut());
                        help_ptr = 1;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                            b"This \\read has unbalanced braces.\0".as_ptr() as *const c_char;
                        align_state = 1000000;
                        cur_input.limit = 0;
                        error();
                    }
                }
            }

            cur_input.limit = last;

            if INTPAR!(INT_PAR__end_line_char) < 0 || INTPAR!(INT_PAR__end_line_char) > 255 {
                cur_input.limit -= 1;
            } else {
                *buffer.offset(cur_input.limit as isize) =
                    (INTPAR!(INT_PAR__end_line_char) as u8) as i32;
            }

            first = cur_input.limit + 1;
            cur_input.loc = cur_input.start;
            cur_input.state = (NEW_LINE as u8) as u16;

            if j == 1 {
                while cur_input.loc <= cur_input.limit {
                    cur_chr = *buffer.offset(cur_input.loc as isize) as i32;
                    cur_input.loc += 1;
                    if cur_chr == b' ' as i32 {
                        cur_tok = SPACE_TOKEN;
                    } else {
                        cur_tok = cur_chr + OTHER_TOKEN;
                    }

                    q = get_avail();
                    mem!(p).b32.s1 = q;
                    mem!(q).b32.s0 = cur_tok;
                    p = q;
                }
            } else {
                loop {
                    get_token();
                    if cur_tok == 0 {
                        break;
                    }

                    if align_state < 1000000 {
                        loop {
                            get_token();
                            if cur_tok == 0 {
                                break;
                            }
                        }
                        align_state = 1000000;
                        break;
                    }

                    q = get_avail();
                    mem!(p).b32.s1 = q;
                    mem!(q).b32.s0 = cur_tok;
                    p = q;
                }
            }

            end_file_reading();
            if align_state == 1000000 {
                break;
            }
        }

        cur_val = def_ref;
        scanner_status = (NORMAL) as u8;
        align_state = s;
    }
}

#[unsafe(export_name = "pass_text")]
pub unsafe extern "C" fn pass_text() {
    unsafe {
        let mut l: i32;
        let mut save_scanner_status: u8;

        save_scanner_status = scanner_status;
        scanner_status = (SKIPPING) as u8;
        l = 0;
        skip_line = line;

        loop {
            get_next();
            if cur_cmd == (FI_OR_ELSE) as u8 {
                if l == 0 {
                    break;
                }
                if cur_chr == FI_CODE {
                    l -= 1;
                }
            } else if cur_cmd == (IF_TEST) as u8 {
                l += 1;
            }
        }

        scanner_status = (save_scanner_status) as u8;
        if INTPAR!(INT_PAR__tracing_ifs) > 0 {
            show_cur_cmd_chr();
        }
    }
}

#[unsafe(export_name = "change_if_limit")]
pub unsafe extern "C" fn change_if_limit(l: u8, p: i32) {
    unsafe {
        let mut q: i32;
        if p == cond_ptr {
            if_limit = l;
        } else {
            q = cond_ptr;
            loop {
                if q == TEX_NULL {
                    confusion(b"if\0".as_ptr() as *const c_char);
                }
                if mem!(q).b32.s1 == p {
                    mem!(q).b16.s1 = l as u16;
                    return;
                }
                q = LLIST_link!(q);
            }
        }
    }
}

#[unsafe(export_name = "conditional")]
pub unsafe extern "C" fn conditional() {
    unsafe {
        let mut b: bool = false;
        let e: bool;
        let mut r: u8 = 0;
        let mut m: i32 = 0;
        let mut n: i32 = 0;
        let mut p: i32;
        let mut q: i32;
        let mut save_scanner_status: u8;
        let save_cond_ptr: i32;
        let this_if: u8;
        let is_unless: bool;

        if INTPAR!(INT_PAR__tracing_ifs) > 0 {
            if INTPAR!(INT_PAR__tracing_commands) <= 1 {
                show_cur_cmd_chr();
            }
        }

        p = get_node(IF_NODE_SIZE);
        mem!(p).b32.s1 = cond_ptr;
        mem!(p).b16.s1 = if_limit as u16;
        mem!(p).b16.s0 = cur_if as u16;
        mem!(p + 1).b32.s1 = if_line;
        cond_ptr = p;
        cur_if = (cur_chr) as i16;
        if_limit = IF_CODE as u8;
        if_line = line;

        save_cond_ptr = cond_ptr;
        is_unless = cur_chr >= UNLESS_CODE;
        this_if = (cur_chr % UNLESS_CODE) as u8;

        'switch: {
            match (this_if as i32) as i32 {
                IF_CHAR_CODE | IF_CAT_CODE => {
                    get_x_token();
                    if cur_cmd == (RELAX) as u8 {
                        if cur_chr == NO_EXPAND_FLAG {
                            cur_cmd = (ACTIVE_CHAR) as u8;
                            cur_chr = cur_tok - (CS_TOKEN_FLAG + ACTIVE_BASE);
                        }
                    }

                    if cur_cmd > (ACTIVE_CHAR) as u8 || cur_chr > BIGGEST_USV {
                        m = RELAX;
                        n = TOO_BIG_USV;
                    } else {
                        m = (cur_cmd) as i32;
                        n = cur_chr;
                    }

                    get_x_token();
                    if cur_cmd == (RELAX) as u8 {
                        if cur_chr == NO_EXPAND_FLAG {
                            cur_cmd = (ACTIVE_CHAR) as u8;
                            cur_chr = cur_tok - (CS_TOKEN_FLAG + ACTIVE_BASE);
                        }
                    }

                    if cur_cmd > (ACTIVE_CHAR) as u8 || cur_chr > BIGGEST_USV {
                        cur_cmd = (RELAX) as u8;
                        cur_chr = TOO_BIG_USV;
                    }

                    if this_if as i32 == IF_CHAR_CODE {
                        b = n == cur_chr;
                    } else {
                        b = m == (cur_cmd) as i32;
                    }
                }

                IF_INT_CODE | IF_DIM_CODE => {
                    if this_if as i32 == IF_INT_CODE {
                        scan_int();
                    } else {
                        scan_dimen(false, false, false);
                    }
                    n = cur_val;

                    loop {
                        get_x_token();
                        if cur_cmd != (SPACER) as u8 {
                            break;
                        }
                    }

                    if cur_tok >= OTHER_TOKEN + 60 && cur_tok <= OTHER_TOKEN + 62 {
                        r = (cur_tok - OTHER_TOKEN) as u8;
                    } else {
                        error_here_with_diagnostic(
                            b"Missing = inserted for \0".as_ptr() as *const c_char
                        );
                        print_cmd_chr((IF_TEST) as u16, this_if as i32);
                        capture_to_diagnostic(std::ptr::null_mut());

                        help_ptr = 1;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                            b"I was expecting to see `<', `=', or `>'. Didn't.\0".as_ptr()
                                as *const c_char;
                        back_error();
                        r = b'=';
                    }

                    if this_if as i32 == IF_INT_CODE {
                        scan_int();
                    } else {
                        scan_dimen(false, false, false);
                    }

                    match (r) as i32 {
                        60 => {
                            b = n < cur_val;
                        }
                        61 => {
                            b = n == cur_val;
                        }
                        62 => {
                            b = n > cur_val;
                        }
                        _ => {}
                    }
                }

                IF_ODD_CODE => {
                    scan_int();
                    b = odd(cur_val);
                }

                IF_VMODE_CODE => {
                    b = cur_list.mode.abs() == (VMODE) as i16;
                }

                IF_HMODE_CODE => {
                    b = cur_list.mode.abs() == (HMODE) as i16;
                }

                IF_MMODE_CODE => {
                    b = cur_list.mode.abs() == (MMODE) as i16;
                }

                IF_INNER_CODE => {
                    b = cur_list.mode < 0;
                }

                IF_VOID_CODE | IF_HBOX_CODE | IF_VBOX_CODE => {
                    scan_register_num();
                    if cur_val < 256 {
                        p = eqtb!(BOX_BASE + cur_val).b32.s1;
                    } else {
                        find_sa_element(4, cur_val, false);
                        if cur_ptr == TEX_NULL {
                            p = TEX_NULL;
                        } else {
                            p = mem!(cur_ptr + 1).b32.s1;
                        }
                    }

                    if this_if as i32 == IF_VOID_CODE {
                        b = p == TEX_NULL;
                    } else if p == TEX_NULL {
                        b = false;
                    } else if this_if as i32 == IF_HBOX_CODE {
                        b = NODE_type!(p) == HLIST_NODE as u16;
                    } else {
                        b = NODE_type!(p) == VLIST_NODE as u16;
                    }
                }

                IFX_CODE => {
                    save_scanner_status = scanner_status;
                    scanner_status = (NORMAL) as u8;
                    get_next();
                    n = cur_cs;
                    p = (cur_cmd) as i32;
                    q = cur_chr;
                    get_next();

                    if cur_cmd != (p) as u8 {
                        b = false;
                    } else if cur_cmd < (CALL) as u8 {
                        b = cur_chr == q;
                    } else {
                        p = mem!(cur_chr).b32.s1;
                        q = mem!(eqtb!(n).b32.s1).b32.s1;
                        if p == q {
                            b = true;
                        } else {
                            while p != TEX_NULL && q != TEX_NULL {
                                if mem!(p).b32.s0 != mem!(q).b32.s0 {
                                    p = TEX_NULL;
                                } else {
                                    p = LLIST_link!(p);
                                    q = LLIST_link!(q);
                                }
                            }
                            b = p == TEX_NULL && q == TEX_NULL;
                        }
                    }
                    scanner_status = save_scanner_status;
                }

                IF_EOF_CODE => {
                    scan_four_bit_int_or_18();
                    if cur_val == 18 {
                        b = !shell_escape_enabled;
                    } else {
                        b = *(&raw mut read_open).cast::<u8>().offset(cur_val as isize)
                            == (CLOSED) as u8;
                    }
                }

                IF_TRUE_CODE => {
                    b = true;
                }

                IF_FALSE_CODE => {
                    b = false;
                }

                IF_DEF_CODE => {
                    save_scanner_status = scanner_status;
                    scanner_status = (NORMAL) as u8;
                    get_next();
                    b = cur_cmd != (UNDEFINED_CS) as u8;
                    scanner_status = save_scanner_status;
                }

                IF_CS_CODE => {
                    n = get_avail();
                    p = n;
                    e = is_in_csname;
                    is_in_csname = true;

                    loop {
                        get_x_token();
                        if cur_cs == 0 {
                            q = get_avail();
                            mem!(p).b32.s1 = q;
                            mem!(q).b32.s0 = cur_tok;
                            p = q;
                        }
                        if cur_cs != 0 {
                            break;
                        }
                    }

                    if cur_cmd != (END_CS_NAME) as u8 {
                        error_here_with_diagnostic(b"Missing \0".as_ptr() as *const c_char);
                        print_esc_cstr(b"endcsname\0".as_ptr() as *const c_char);
                        print_cstr(b" inserted\0".as_ptr() as *const c_char);
                        capture_to_diagnostic(std::ptr::null_mut());

                        help_ptr = 2;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(1) =
                            b"The control sequence marked <to be read again> should\0".as_ptr()
                                as *const c_char;
                        *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                            b"not appear between \\csname and \\endcsname.\0".as_ptr()
                                as *const c_char;
                        back_error();
                    }

                    m = first;
                    p = mem!(n).b32.s1;

                    while p != TEX_NULL {
                        if m >= max_buf_stack {
                            max_buf_stack = m + 1;
                            if max_buf_stack == buf_size {
                                overflow(b"buffer size\0".as_ptr() as *const c_char, buf_size);
                            }
                        }

                        *buffer.offset(m as isize) = ((mem!(p).b32.s0 % MAX_CHAR_VAL) as u8) as i32;
                        m += 1;
                        p = LLIST_link!(p);
                    }

                    if m == first {
                        cur_cs = NULL_CS;
                    } else if m == first + 1 {
                        cur_cs = SINGLE_BASE + *buffer.offset(first as isize) as i32;
                    } else {
                        cur_cs = id_lookup(first, m - first);
                    }

                    flush_list(n);
                    b = eqtb!(cur_cs).b16.s1 != (UNDEFINED_CS) as u16;
                    is_in_csname = e;
                }

                IF_IN_CSNAME_CODE => {
                    b = is_in_csname;
                }

                IF_FONT_CHAR_CODE => {
                    scan_font_ident();
                    n = cur_val;
                    scan_usv_num();

                    if *font_area.offset(n as isize) == (AAT_FONT_FLAG) as i32
                        || *font_area.offset(n as isize) == (OTGR_FONT_FLAG) as i32
                    {
                        b = map_char_to_glyph(n, cur_val) > 0;
                    } else {
                        if *font_bc.offset(n as isize) <= (cur_val) as u16
                            && *font_ec.offset(n as isize) >= (cur_val) as u16
                        {
                            b = (*font_info.offset(
                                (*char_base.offset(n as isize)
                                    + effective_char(true, n, (cur_val) as u16))
                                    as isize,
                            ))
                            .b16
                            .s3 > 0;
                        } else {
                            b = false;
                        }
                    }
                }

                IF_CASE_CODE => {
                    scan_int();
                    n = cur_val;

                    if INTPAR!(INT_PAR__tracing_commands) > 1 {
                        begin_diagnostic();
                        diagnostic_begin_capture_warning_here();
                        print_cstr(b"{case \0".as_ptr() as *const c_char);
                        print_int(n);
                        print_char(b'}' as i32);
                        capture_to_diagnostic(std::ptr::null_mut());
                        end_diagnostic(false);
                    }

                    while n != 0 {
                        pass_text();

                        if cond_ptr == save_cond_ptr {
                            if cur_chr == OR_CODE {
                                n -= 1;
                            } else {
                                break 'switch;
                            }
                        } else if cur_chr == FI_CODE {
                            if *if_stack.offset(in_open as isize) == cond_ptr {
                                if_warning();
                            }
                            p = cond_ptr;
                            if_line = mem!(p + 1).b32.s1;
                            cur_if = (mem!(p).b16.s0 as i32) as i16;
                            if_limit = mem!(p).b16.s1 as u8;
                            cond_ptr = mem!(p).b32.s1;
                            free_node(p, IF_NODE_SIZE);
                        }
                    }
                    change_if_limit(OR_CODE as u8, save_cond_ptr);
                    return;
                }

                IF_PRIMITIVE_CODE => {
                    save_scanner_status = scanner_status;
                    scanner_status = (NORMAL) as u8;
                    get_next();
                    scanner_status = save_scanner_status;
                    if cur_cs < HASH_BASE {
                        m = prim_lookup(cur_cs - SINGLE_BASE);
                    } else {
                        m = prim_lookup((*hash.offset(cur_cs as isize)).s1);
                    }
                    b = cur_cmd != (UNDEFINED_CS) as u8
                        && m != UNDEFINED_PRIMITIVE
                        && cur_cmd == ((*eqtb.offset((PRIM_EQTB_BASE + m) as isize)).b16.s1) as u8
                        && cur_chr == (*eqtb.offset((PRIM_EQTB_BASE + m) as isize)).b32.s1;
                }
                _ => {}
            }

            if is_unless {
                b = !b;
            }

            if INTPAR!(INT_PAR__tracing_commands) > 1 {
                begin_diagnostic();
                diagnostic_begin_capture_warning_here();
                if b {
                    print_cstr(b"{true}\0".as_ptr() as *const c_char);
                } else {
                    print_cstr(b"{false}\0".as_ptr() as *const c_char);
                }
                capture_to_diagnostic(std::ptr::null_mut());
                end_diagnostic(false);
            }

            if b {
                change_if_limit(ELSE_CODE as u8, save_cond_ptr);
                return;
            }

            loop {
                pass_text();

                if cond_ptr == save_cond_ptr {
                    if cur_chr != OR_CODE {
                        break 'switch;
                    }

                    error_here_with_diagnostic(b"Extra \0".as_ptr() as *const c_char);
                    print_esc_cstr(b"or\0".as_ptr() as *const c_char);
                    capture_to_diagnostic(std::ptr::null_mut());

                    help_ptr = 1;
                    *(&raw mut help_line).cast::<*const libc::c_char>().offset(0) =
                        b"I'm ignoring this; it doesn't match any \\if.\0".as_ptr()
                            as *const c_char;
                    error();
                } else if cur_chr == FI_CODE {
                    if *if_stack.offset(in_open as isize) == cond_ptr {
                        if_warning();
                    }
                    p = cond_ptr;
                    if_line = mem!(p + 1).b32.s1;
                    cur_if = (mem!(p).b16.s0 as i32) as i16;
                    if_limit = mem!(p).b16.s1 as u8;
                    cond_ptr = mem!(p).b32.s1;
                    free_node(p, IF_NODE_SIZE);
                }
            }
        }

        // common_ending:
        if cur_chr == FI_CODE {
            if *if_stack.offset(in_open as isize) == cond_ptr {
                if_warning();
            }
            p = cond_ptr;
            if_line = mem!(p + 1).b32.s1;
            cur_if = (mem!(p).b16.s0 as i32) as i16;
            if_limit = mem!(p).b16.s1 as u8;
            cond_ptr = mem!(p).b32.s1;
            free_node(p, IF_NODE_SIZE);
        } else {
            if_limit = FI_CODE as u8;
        }
    }
}

#[inline]
unsafe fn mfree<T>(ptr: *mut T) -> *mut T {
    if !ptr.is_null() {
        libc::free(ptr as *mut libc::c_void);
    }
    std::ptr::null_mut()
}

#[inline]
fn IS_DIR_SEP(c: u32) -> bool {
    c == b'/' as u32
}

macro_rules! BOX_width {
    ($p:expr) => {
        mem!(($p) + 1).b32.s1
    };
}

macro_rules! CHAR_NODE_character {
    ($p:expr) => {
        mem!($p).b16.s0
    };
}

macro_rules! CHAR_NODE_font {
    ($p:expr) => {
        mem!($p).b16.s1
    };
}

macro_rules! NATIVE_NODE_font {
    ($p:expr) => {
        mem!(($p) + 4).b16.s2
    };
}

macro_rules! NATIVE_NODE_length {
    ($p:expr) => {
        mem!(($p) + 4).b16.s1
    };
}

macro_rules! NATIVE_NODE_text {
    ($p:expr) => {
        mem.offset((($p) + NATIVE_NODE_SIZE) as isize) as *mut u16
    };
}

macro_rules! DISCRETIONARY_NODE_replace_count {
    ($p:expr) => {
        mem!($p).b16.s0
    };
}

macro_rules! FONT_CHARACTER_INFO {
    ($f:expr, $c:expr) => {
        (*font_info.offset((*char_base.offset(($f) as isize) + ($c as i32)) as isize)).b16
    };
}

macro_rules! FONT_CHARINFO_WIDTH {
    ($f:expr, $info:expr) => {
        (*font_info.offset((*width_base.offset(($f) as isize) + (($info).s3 as i32)) as isize))
            .b32
            .s1
    };
}

macro_rules! FONT_CHARINFO_HEIGHT {
    ($f:expr, $info:expr) => {
        (*font_info
            .offset((*height_base.offset(($f) as isize) + (($info).s2 as i32 / 16)) as isize))
        .b32
        .s1
    };
}

macro_rules! FONT_CHARINFO_DEPTH {
    ($f:expr, $info:expr) => {
        (*font_info.offset((*depth_base.offset(($f) as isize) + (($info).s2 as i32 % 16)) as isize))
            .b32
            .s1
    };
}

macro_rules! set_help_line {
    ($idx:expr, $str:expr) => {
        *(&raw mut help_line)
            .cast::<*const libc::c_char>()
            .offset($idx) = $str;
    };
}

macro_rules! get_native_word_cp {
    ($p:expr, $side:expr) => {
        real_get_native_word_cp(mem.offset(($p) as isize) as *mut c_void, ($side) as c_int)
    };
}

#[repr(C)]
pub struct NativeLayoutRequest {
    pub engine: *mut c_void,
    pub text: *const u16,
    pub length: usize,
}

unsafe extern "C" {
    fn ttstub_input_getc(handle: rust_input_handle_t) -> c_int;
    fn get_loaded_font_design_size() -> i32;
    pub fn ttxl_prepare_layouts(
        requests: *const NativeLayoutRequest,
        count: usize,
    ) -> *mut PreparedLayouts;
}

// 1. begin_name
#[unsafe(export_name = "begin_name")]
pub unsafe extern "C" fn begin_name() {
    area_delimiter = 0;
    ext_delimiter = 0;
    quoted_filename = false;
    file_name_quote_char = 0;
}

// 2. more_name
#[unsafe(export_name = "more_name")]
pub unsafe extern "C" fn more_name(c: UTF16_code) -> bool {
    if stop_at_space && file_name_quote_char == 0 && c == b' ' as UTF16_code {
        return false;
    }

    if stop_at_space && file_name_quote_char != 0 && c == file_name_quote_char {
        file_name_quote_char = 0;
        return true;
    }

    if stop_at_space
        && file_name_quote_char == 0
        && (c == b'"' as UTF16_code || c == b'\'' as UTF16_code)
    {
        file_name_quote_char = c;
        quoted_filename = true;
        return true;
    }

    if pool_ptr + 1 > pool_size {
        overflow(
            b"pool size\0".as_ptr() as *const c_char,
            pool_size - init_pool_ptr,
        );
    }

    *str_pool.offset(pool_ptr as isize) = c;
    pool_ptr += 1;

    if IS_DIR_SEP(c as u32) {
        area_delimiter = cur_length();
        ext_delimiter = 0;
    } else if c == b'.' as UTF16_code {
        ext_delimiter = cur_length();
    }

    true
}

// 3. end_name
#[unsafe(export_name = "end_name")]
pub unsafe extern "C" fn end_name() {
    super::xetex_stringpool::truncate_string_index(str_ptr);
    let mut temp_str: str_number;
    let mut j: pool_pointer;

    if str_ptr + 3 > max_strings {
        overflow(
            b"number of strings\0".as_ptr() as *const c_char,
            max_strings - init_str_ptr,
        );
    }

    if area_delimiter == 0 {
        cur_area = EMPTY_STRING;
    } else {
        cur_area = str_ptr;
        *str_start.offset(((str_ptr + 1) - 65536) as isize) =
            *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) + area_delimiter;
        str_ptr += 1;
        temp_str = search_string(cur_area);

        if temp_str > 0 {
            cur_area = temp_str;
            str_ptr -= 1;

            j = *str_start.offset(((str_ptr + 1) - 65536) as isize);
            while j <= pool_ptr - 1 {
                *str_pool.offset((j - area_delimiter) as isize) = *str_pool.offset(j as isize);
                j += 1;
            }

            pool_ptr = pool_ptr - area_delimiter;
        }
    }

    if ext_delimiter == 0 {
        cur_ext = EMPTY_STRING;
        cur_name = slow_make_string();
    } else {
        cur_name = str_ptr;
        *str_start.offset(((str_ptr + 1) - 65536) as isize) =
            *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) + ext_delimiter
                - area_delimiter
                - 1;
        str_ptr += 1;

        cur_ext = make_string();
        str_ptr -= 1;
        temp_str = search_string(cur_name);

        if temp_str > 0 {
            cur_name = temp_str;
            str_ptr -= 1;

            j = *str_start.offset(((str_ptr + 1) - 65536) as isize);
            while j <= pool_ptr - 1 {
                *str_pool.offset((j - ext_delimiter + area_delimiter + 1) as isize) =
                    *str_pool.offset(j as isize);
                j += 1;
            }

            pool_ptr = pool_ptr - ext_delimiter + area_delimiter + 1;
        }

        cur_ext = slow_make_string();
    }
}

// 4. pack_file_name
#[unsafe(export_name = "pack_file_name")]
pub unsafe extern "C" fn pack_file_name(n: str_number, a: str_number, e: str_number) {
    let buf_len = ((length(a) + length(n) + length(e)) * 3 + 1) as usize;
    let work_buffer = xmalloc(buf_len) as *mut c_char;
    *work_buffer = 0;

    let a_utf8 = gettexstring(a);
    strcat(work_buffer, a_utf8);
    free(a_utf8 as *mut c_void);

    let n_utf8 = gettexstring(n);
    strcat(work_buffer, n_utf8);
    free(n_utf8 as *mut c_void);

    let e_utf8 = gettexstring(e);
    strcat(work_buffer, e_utf8);
    free(e_utf8 as *mut c_void);

    name_length = strlen(work_buffer) as i32;

    free(name_of_file as *mut c_void);
    name_of_file = xmalloc((name_length as usize) + 1) as *mut c_char;
    strcpy(name_of_file, work_buffer);
    free(work_buffer as *mut c_void);
}

// 5. make_name_string
#[unsafe(export_name = "make_name_string")]
pub unsafe extern "C" fn make_name_string() -> str_number {
    let mut k: i32;
    let save_area_delimiter: pool_pointer;
    let save_ext_delimiter: pool_pointer;
    let save_name_in_progress: bool;
    let save_stop_at_space: bool;

    if pool_ptr + name_length > pool_size || str_ptr == max_strings || cur_length() > 0 {
        return b'?' as str_number;
    }

    make_utf16_name();

    k = 0;
    while k < name_length16 {
        *str_pool.offset(pool_ptr as isize) = *name_of_file16.offset(k as isize);
        pool_ptr += 1;
        k += 1;
    }

    let Result: str_number = make_string();

    save_area_delimiter = area_delimiter;
    save_ext_delimiter = ext_delimiter;
    save_name_in_progress = name_in_progress;
    save_stop_at_space = stop_at_space;
    name_in_progress = true;
    begin_name();
    stop_at_space = false;
    k = 0;

    while k < name_length16 && more_name(*name_of_file16.offset(k as isize)) {
        k += 1;
    }

    stop_at_space = save_stop_at_space;
    end_name();
    name_in_progress = save_name_in_progress;
    area_delimiter = save_area_delimiter;
    ext_delimiter = save_ext_delimiter;

    Result
}

// 6. scan_file_name_braced
#[unsafe(export_name = "scan_file_name_braced")]
pub unsafe extern "C" fn scan_file_name_braced() {
    let save_scanner_status: u8 = scanner_status;
    let save_def_ref: i32 = def_ref;
    let save_cur_cs: i32 = cur_cs;
    let s: str_number;
    let mut i: pool_pointer;
    let save_stop_at_space: bool;

    cur_cs = warning_index;
    scan_toks(false, true);

    old_setting = (selector) as u8;
    selector = SELECTOR_NEW_STRING;
    show_token_list(mem!(def_ref).b32.s1, TEX_NULL, pool_size - pool_ptr);
    selector = (old_setting) as i32;
    s = make_string();
    delete_token_ref(def_ref);
    def_ref = save_def_ref;
    cur_cs = save_cur_cs;
    scanner_status = save_scanner_status;
    save_stop_at_space = stop_at_space;

    begin_name();

    i = *str_start.offset((s - TOO_BIG_CHAR) as isize);
    let end_i = *str_start.offset((s + 1 - TOO_BIG_CHAR) as isize);
    while i < end_i {
        more_name(*str_pool.offset(i as isize));
        i += 1;
    }

    stop_at_space = save_stop_at_space;
}

// 7. scan_file_name
#[unsafe(export_name = "scan_file_name")]
pub unsafe extern "C" fn scan_file_name() {
    let save_warning_index: i32 = warning_index;
    warning_index = cur_cs;

    loop {
        get_x_token();
        if !(cur_cmd == (SPACER) as u8 || cur_cmd == (RELAX) as u8) {
            break;
        }
    }

    back_input();

    if cur_cmd == (LEFT_BRACE) as u8 {
        scan_file_name_braced();
    } else {
        name_in_progress = true;
        begin_name();

        loop {
            get_x_token();
            if cur_cmd != (SPACER) as u8 {
                break;
            }
        }

        loop {
            if cur_cmd > (OTHER_CHAR) as u8 || cur_chr > BIGGEST_CHAR {
                back_input();
                break;
            }

            if !more_name(cur_chr as UTF16_code) {
                break;
            }

            get_x_token();
        }
    }

    end_name();
    name_in_progress = false;
    warning_index = save_warning_index;
}

// 8. pack_job_name
#[unsafe(export_name = "pack_job_name")]
pub unsafe extern "C" fn pack_job_name(s: *const c_char) {
    cur_area = EMPTY_STRING;
    cur_ext = maketexstring(s);
    cur_name = job_name;
    pack_file_name(cur_name, cur_area, cur_ext);
}

// 9. open_log_file
#[unsafe(export_name = "open_log_file")]
pub unsafe extern "C" fn open_log_file() {
    let local_old_setting: u8;
    let mut k: i32;
    let mut l: i32;

    local_old_setting = (selector) as u8;
    if job_name == 0 {
        job_name = maketexstring(b"texput\0".as_ptr() as *const c_char);
    }

    pack_job_name(b".log\0".as_ptr() as *const c_char);

    log_file = ttstub_output_open(name_of_file, 0);
    if log_file == INVALID_HANDLE {
        _tt_abort(
            b"cannot open log file output \"%s\"\0".as_ptr() as *const c_char,
            name_of_file,
        );
    }

    texmf_log_name = make_name_string();
    selector = SELECTOR_LOG_ONLY;
    log_opened = true;

    *input_stack.offset(input_ptr as isize) = cur_input;

    print_nl_cstr(b"**\0".as_ptr() as *const c_char);
    l = (*input_stack.offset(0)).limit;
    if *buffer.offset(l as isize) as i32 == INTPAR!(INT_PAR__end_line_char) {
        l -= 1;
    }

    k = 1;
    while k <= l {
        print(*buffer.offset(k as isize) as i32);
        k += 1;
    }

    print_ln();
    selector = (local_old_setting + 2) as i32;
}

// 10. start_input
#[unsafe(export_name = "start_input")]
pub unsafe extern "C" fn start_input(primary_input_name: *const c_char) {
    let mut format: c_int = TTBC_FILE_FORMAT_TEX;
    let mut temp_str: str_number;

    if !primary_input_name.is_null() {
        format = TTBC_FILE_FORMAT_TECTONIC_PRIMARY;

        name_in_progress = true;
        begin_name();
        stop_at_space = false;

        let mut cp = primary_input_name as *const u8;

        if pool_ptr + (strlen(primary_input_name) * 2) as i32 >= pool_size {
            _tt_abort(
                b"string pool overflow [%i bytes]\0".as_ptr() as *const c_char,
                pool_size as c_int,
            );
        }

        while *cp != 0 {
            let mut rval = *cp as u32;
            cp = cp.add(1);
            let extraBytes = bytesFromUTF8[rval as usize];
            match extraBytes {
                5 => {
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                }
                4 => {
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                }
                3 => {
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                }
                2 => {
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                }
                1 => {
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                }
                _ => {}
            }
            rval -= offsetsFromUTF8[extraBytes as usize];
            if rval > 0xffff {
                rval -= 0x10000;
                *str_pool.offset(pool_ptr as isize) = (0xd800 + rval / 0x0400) as u16;
                pool_ptr += 1;
                *str_pool.offset(pool_ptr as isize) = (0xdc00 + rval % 0x0400) as u16;
                pool_ptr += 1;
            } else {
                *str_pool.offset(pool_ptr as isize) = rval as u16;
                pool_ptr += 1;
            }

            if IS_DIR_SEP(rval) {
                area_delimiter = cur_length();
                ext_delimiter = 0;
            } else if rval == b'.' as u32 {
                ext_delimiter = cur_length();
            }
        }

        stop_at_space = true;
        end_name();
        name_in_progress = false;
    } else {
        scan_file_name();
    }

    pack_file_name(cur_name, cur_area, cur_ext);

    begin_file_reading();

    if u_open_in(
        input_file.offset(cur_input.index as isize),
        format,
        b"rb\0".as_ptr() as *const c_char,
        INTPAR!(INT_PAR__xetex_default_input_mode),
        INTPAR!(INT_PAR__xetex_default_input_encoding),
    ) == 0
    {
        _tt_abort(
            b"failed to open input file \"%s\"\0".as_ptr() as *const c_char,
            name_of_file,
        );
    }

    make_utf16_name();
    name_in_progress = true;
    begin_name();
    stop_at_space = false;
    let mut k = 0;
    while k < name_length16 && more_name(*name_of_file16.offset(k as isize)) {
        k += 1;
    }
    stop_at_space = true;
    end_name();
    name_in_progress = false;

    cur_input.name = make_name_string();
    *source_filename_stack.offset(in_open as isize) = cur_input.name;

    *full_source_filename_stack.offset(in_open as isize) = maketexstring(name_of_input_file);
    if cur_input.name == str_ptr - 1 {
        temp_str = search_string(cur_input.name);
        if temp_str > 0 {
            cur_input.name = temp_str;
            str_ptr -= 1;
            pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
        }
    }

    if job_name == 0 {
        job_name = cur_name;
        open_log_file();
    }

    if term_offset + length(*full_source_filename_stack.offset(in_open as isize))
        > max_print_line - 2
    {
        print_ln();
    } else if term_offset > 0 || file_offset > 0 {
        print_char(b' ' as i32);
    }
    print_char(b'(' as i32);
    open_parens += 1;
    print(*full_source_filename_stack.offset(in_open as isize));
    ttstub_output_flush(rust_stdout);

    if INTPAR!(INT_PAR__tracing_stack_levels) > 0 {
        let mut v: i32;

        begin_diagnostic();
        diagnostic_begin_capture_warning_here();
        print_ln();
        print_char(b'~' as i32);
        v = input_ptr - 1;
        if v < INTPAR!(INT_PAR__tracing_stack_levels) {
            while v > 0 {
                print_char(b'.' as i32);
                v -= 1;
            }
        } else {
            print_char(b'~' as i32);
        }
        print_cstr(b"INPUT \0".as_ptr() as *const c_char);
        print(cur_name);
        print(cur_ext);
        print_ln();
        capture_to_diagnostic(std::ptr::null_mut());
        end_diagnostic(false);
    }

    cur_input.state = (NEW_LINE as u8) as u16;

    synctex_start_input();

    line = 1;
    input_line(*input_file.offset(cur_input.index as isize));
    cur_input.limit = last;

    if INTPAR!(INT_PAR__end_line_char) < 0 || INTPAR!(INT_PAR__end_line_char) > 255 {
        cur_input.limit -= 1;
    } else {
        *buffer.offset(cur_input.limit as isize) = INTPAR!(INT_PAR__end_line_char) as UnicodeScalar;
    }

    first = cur_input.limit + 1;
    cur_input.loc = cur_input.start;
}

// 11. effective_char_info
#[unsafe(export_name = "effective_char_info")]
pub unsafe extern "C" fn effective_char_info(f: internal_font_number, mut c: u16) -> b16x4 {
    if !xtx_ligature_present && !(*font_mapping.offset(f as isize)).is_null() {
        c = apply_tfm_font_mapping(*font_mapping.offset(f as isize), c as i32) as u16;
    }

    xtx_ligature_present = false;
    FONT_CHARACTER_INFO!(f, c)
}

// 12. char_warning
#[unsafe(export_name = "char_warning")]
pub unsafe extern "C" fn char_warning(f: internal_font_number, c: i32) {
    let mut local_old_setting: i32;

    if INTPAR!(INT_PAR__tracing_lost_chars) > 0 {
        local_old_setting = INTPAR!(INT_PAR__tracing_online);
        if INTPAR!(INT_PAR__tracing_lost_chars) > 1 {
            INTPAR!(INT_PAR__tracing_online) = 1;
        }

        if INTPAR!(INT_PAR__tracing_lost_chars) > 2 {
            if (file_line_error_style_p) != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! \0".as_ptr() as *const c_char);
            }
            print_cstr(b"Missing character: There is no \0".as_ptr() as *const c_char);
        } else {
            begin_diagnostic();
            diagnostic_begin_capture_warning_here();
            print_nl_cstr(b"Missing character: There is no \0".as_ptr() as *const c_char);
        }

        if c < 65536 {
            print(c);
        } else {
            print_char(c);
        }

        print_cstr(b" (\0".as_ptr() as *const c_char);

        if *font_area.offset(f as isize) == (AAT_FONT_FLAG) as i32
            || *font_area.offset(f as isize) == (OTGR_FONT_FLAG) as i32
        {
            print_ucs_code(c);
        } else {
            print_hex(c);
        }

        print_char(b')' as i32);
        print_cstr(b" in font \0".as_ptr() as *const c_char);
        print(*font_name.offset(f as isize));

        if INTPAR!(INT_PAR__tracing_lost_chars) < 3 {
            print_char(b'!' as i32);
        }

        INTPAR!(INT_PAR__tracing_online) = local_old_setting;

        if INTPAR!(INT_PAR__tracing_lost_chars) > 2 {
            help_ptr = 0;
            error();
        } else {
            capture_to_diagnostic(std::ptr::null_mut());
            end_diagnostic(false);
        }
    }

    {
        let fn_str = gettexstring(*font_name.offset(f as isize));
        let prev_selector = selector;
        selector = SELECTOR_NEW_STRING;
        if c < 0x10000 {
            print(c);
        } else {
            print_char(c);
        }
        selector = prev_selector;
        let s = make_string();
        let chr = gettexstring(s);
        str_ptr -= 1;
        pool_ptr = *str_start.offset((str_ptr - 0x10000) as isize);

        ttstub_issue_warning(
            b"could not represent character \"%s\" (0x%x) in font \"%s\"\0".as_ptr()
                as *const c_char,
            chr,
            c,
            fn_str,
        );

        free(fn_str as *mut c_void);
        free(chr as *mut c_void);

        if !gave_char_warning_help {
            ttstub_issue_warning(
                b"  you may need to load the `fontspec` package and use (e.g.) \\setmainfont to\0"
                    .as_ptr() as *const c_char,
            );
            ttstub_issue_warning(
                b"  choose a different font that covers the unrepresentable character(s)\0".as_ptr()
                    as *const c_char,
            );
            gave_char_warning_help = true;
        }
    }
}

// 13. new_native_word_node
#[unsafe(export_name = "new_native_word_node")]
pub unsafe extern "C" fn new_native_word_node(f: internal_font_number, n: i32) -> i32 {
    let l = NATIVE_NODE_SIZE
        + (n * std::mem::size_of::<UTF16_code>() as i32
            + std::mem::size_of::<memory_word>() as i32
            - 1)
            / std::mem::size_of::<memory_word>() as i32;
    let q = get_node(l);
    NODE_type!(q) = (WHATSIT_NODE as i16) as u16;

    if INTPAR!(INT_PAR__xetex_generate_actual_text) > 0 {
        NODE_subtype!(q) = (NATIVE_WORD_NODE_AT as i16) as u16;
    } else {
        NODE_subtype!(q) = (NATIVE_WORD_NODE as i16) as u16;
    }

    NATIVE_NODE_size!(q) = (l as i16) as u16;
    NATIVE_NODE_font!(q) = (f as i16) as u16;
    NATIVE_NODE_length!(q) = (n as i16) as u16;
    NATIVE_NODE_glyph_count!(q) = 0;
    NATIVE_NODE_glyph_info_ptr!(q) = std::ptr::null_mut();
    q
}

// 14. new_native_character
#[unsafe(export_name = "new_native_character")]
pub unsafe extern "C" fn new_native_character(
    f: internal_font_number,
    mut c: UnicodeScalar,
) -> i32 {
    let mut p: i32;
    let mut i: i32;
    let len: i32;

    if !(*font_mapping.offset(f as isize)).is_null() {
        if c > 65535 {
            if pool_ptr + 2 > pool_size {
                overflow(
                    b"pool size\0".as_ptr() as *const c_char,
                    pool_size - init_pool_ptr,
                );
            }

            *str_pool.offset(pool_ptr as isize) = ((c - 65536) / 1024 + 0xD800) as u16;
            pool_ptr += 1;
            *str_pool.offset(pool_ptr as isize) = ((c - 65536) % 1024 + 0xDC00) as u16;
            pool_ptr += 1;
        } else {
            if pool_ptr + 1 > pool_size {
                overflow(
                    b"pool size\0".as_ptr() as *const c_char,
                    pool_size - init_pool_ptr,
                );
            }

            *str_pool.offset(pool_ptr as isize) = c as u16;
            pool_ptr += 1;
        }

        len = apply_mapping(
            *font_mapping.offset(f as isize),
            str_pool.offset(*str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) as isize),
            cur_length() as c_int,
        );
        pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);

        i = 0;
        while i < len {
            let mapped_val = *mapped_text.offset(i as isize);
            if mapped_val >= 0xD800 && mapped_val < 0xDC00 {
                c = ((mapped_val - 0xD800) as UnicodeScalar) * 1024
                    + (*mapped_text.offset((i + 1) as isize) as UnicodeScalar)
                    + 9216;
                if map_char_to_glyph(f as c_int, c) == 0 {
                    char_warning(f, c as i32);
                }
                i += 2;
            } else {
                if map_char_to_glyph(f as c_int, mapped_val as UnicodeScalar) == 0 {
                    char_warning(f, mapped_val as i32);
                }
                i += 1;
            }
        }

        p = new_native_word_node(f, len);

        i = 0;
        while i < len {
            *NATIVE_NODE_text!(p).offset(i as isize) = *mapped_text.offset(i as isize);
            i += 1;
        }
    } else {
        if INTPAR!(INT_PAR__tracing_lost_chars) > 0 {
            if map_char_to_glyph(f as c_int, c) == 0 {
                char_warning(f, c as i32);
            }
        }

        p = get_node(NATIVE_NODE_SIZE + 1);
        NODE_type!(p) = (WHATSIT_NODE as i16) as u16;
        NODE_subtype!(p) = (NATIVE_WORD_NODE as i16) as u16;
        NATIVE_NODE_size!(p) = ((NATIVE_NODE_SIZE + 1) as i16) as u16;
        NATIVE_NODE_glyph_count!(p) = 0;
        NATIVE_NODE_glyph_info_ptr!(p) = std::ptr::null_mut();
        NATIVE_NODE_font!(p) = (f as i16) as u16;

        if c > 65535 {
            NATIVE_NODE_length!(p) = 2;
            *NATIVE_NODE_text!(p).offset(0) = ((c - 65536) / 1024 + 0xD800) as u16;
            *NATIVE_NODE_text!(p).offset(1) = ((c - 65536) % 1024 + 0xDC00) as u16;
        } else {
            NATIVE_NODE_length!(p) = 1;
            *NATIVE_NODE_text!(p).offset(0) = c as u16;
        }
    }

    set_native_metrics(p, INTPAR!(INT_PAR__xetex_use_glyph_metrics) > 0);
    p
}

// 15. font_feature_warning
#[unsafe(export_name = "font_feature_warning")]
pub unsafe extern "C" fn font_feature_warning(
    featureNameP: *const c_void,
    featLen: i32,
    settingNameP: *const c_void,
    setLen: i32,
) {
    begin_diagnostic();
    diagnostic_begin_capture_warning_here();
    print_nl_cstr(b"Unknown \0".as_ptr() as *const c_char);
    if setLen > 0 {
        print_cstr(b"selector `\0".as_ptr() as *const c_char);
        print_utf8_str(settingNameP as *const u8, setLen);
        print_cstr(b"' for \0".as_ptr() as *const c_char);
    }
    print_cstr(b"feature `\0".as_ptr() as *const c_char);
    print_utf8_str(featureNameP as *const u8, featLen);
    print_cstr(b"' in font `\0".as_ptr() as *const c_char);
    let mut i = 0;
    while *name_of_file.offset(i) != 0 {
        print_raw_char(*name_of_file.offset(i) as u8 as UTF16_code, true);
        i += 1;
    }
    print_cstr(b"'.\0".as_ptr() as *const c_char);
    capture_to_diagnostic(std::ptr::null_mut());
    end_diagnostic(false);
}

// 16. font_mapping_warning
#[unsafe(export_name = "font_mapping_warning")]
pub unsafe extern "C" fn font_mapping_warning(
    mappingNameP: *const c_void,
    mappingNameLen: i32,
    warningType: i32,
) {
    begin_diagnostic();
    diagnostic_begin_capture_warning_here();
    if warningType == 0 {
        print_nl_cstr(b"Loaded mapping `\0".as_ptr() as *const c_char);
    } else {
        print_nl_cstr(b"Font mapping `\0".as_ptr() as *const c_char);
    }
    print_utf8_str(mappingNameP as *const u8, mappingNameLen);
    print_cstr(b"' for font `\0".as_ptr() as *const c_char);

    let mut i = 0;
    while *name_of_file.offset(i) != 0 {
        print_raw_char(*name_of_file.offset(i) as u8 as UTF16_code, true);
        i += 1;
    }

    match warningType {
        1 => {
            print_cstr(b"' not found.\0".as_ptr() as *const c_char);
        }
        2 => {
            print_cstr(b"' not usable;\0".as_ptr() as *const c_char);
            print_nl_cstr(
                b"bad mapping file or incorrect mapping type.\0".as_ptr() as *const c_char
            );
        }
        _ => {
            print_cstr(b"'.\0".as_ptr() as *const c_char);
        }
    }
    capture_to_diagnostic(std::ptr::null_mut());
    end_diagnostic(false);
}

// 17. graphite_warning
#[unsafe(export_name = "graphite_warning")]
pub unsafe extern "C" fn graphite_warning() {
    begin_diagnostic();
    diagnostic_begin_capture_warning_here();
    print_nl_cstr(b"Font `\0".as_ptr() as *const c_char);

    let mut i = 0;
    while *name_of_file.offset(i) != 0 {
        print_raw_char(*name_of_file.offset(i) as u8 as UTF16_code, true);
        i += 1;
    }

    print_cstr(
        b"' does not support Graphite. Trying OpenType layout instead.\0".as_ptr() as *const c_char,
    );
    capture_to_diagnostic(std::ptr::null_mut());
    end_diagnostic(false);
}

// 18. load_native_font
#[unsafe(export_name = "load_native_font")]
pub unsafe extern "C" fn load_native_font(
    u: i32,
    nom: str_number,
    aire: str_number,
    mut s: scaled_t,
) -> internal_font_number {
    let mut k: i32;
    let num_font_dimens: i32;
    let font_engine: *mut c_void;
    let actual_size: scaled_t;
    let p: i32;
    let mut ascent: scaled_t = 0;
    let mut descent: scaled_t = 0;
    let mut font_slant: scaled_t = 0;
    let mut x_ht: scaled_t = 0;
    let mut cap_ht: scaled_t = 0;
    let mut f: internal_font_number;
    let full_name: str_number;

    font_engine = find_native_font(name_of_file, s);
    if font_engine.is_null() {
        return FONT_BASE;
    }

    if s >= 0 {
        actual_size = s;
    } else if s != -1000 {
        actual_size = xn_over_d(get_loaded_font_design_size(), -s, 1000);
    } else {
        actual_size = get_loaded_font_design_size();
    }

    if pool_ptr + name_length > pool_size {
        overflow(
            b"pool size\0".as_ptr() as *const c_char,
            pool_size - init_pool_ptr,
        );
    }

    k = 0;
    while k < name_length {
        *str_pool.offset(pool_ptr as isize) = *name_of_file.offset(k as isize) as u8 as u16;
        pool_ptr += 1;
        k += 1;
    }

    full_name = make_string();

    f = FONT_BASE + 1;
    while f <= font_ptr {
        if *font_area.offset(f as isize) == native_font_type_flag
            && str_eq_str(*font_name.offset(f as isize), full_name)
            && *font_size.offset(f as isize) == actual_size
        {
            release_font_engine(font_engine, native_font_type_flag);

            str_ptr -= 1;
            pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);

            return f;
        }
        f += 1;
    }

    if native_font_type_flag == (OTGR_FONT_FLAG) as i32 && isOpenTypeMathFont(font_engine) {
        num_font_dimens = 65;
    } else {
        num_font_dimens = 8;
    }

    if font_ptr == font_max || fmem_ptr + num_font_dimens > font_mem_size {
        error_here_with_diagnostic(b"Font \0".as_ptr() as *const c_char);
        sprint_cs(u);
        print_char(b'=' as i32);
        if file_name_quote_char != 0 {
            print_char(file_name_quote_char as i32);
        }
        print_file_name(nom, aire, cur_ext);
        if file_name_quote_char != 0 {
            print_char(file_name_quote_char as i32);
        }
        if s >= 0 {
            print_cstr(b" at \0".as_ptr() as *const c_char);
            print_scaled(s);
            print_cstr(b"pt\0".as_ptr() as *const c_char);
        } else if s != -1000 {
            print_cstr(b" scaled \0".as_ptr() as *const c_char);
            print_int(-s);
        }
        print_cstr(b" not loaded: Not enough room left\0".as_ptr() as *const c_char);
        capture_to_diagnostic(std::ptr::null_mut());

        help_ptr = 4;
        set_help_line!(
            3,
            b"I'm afraid I won't be able to make use of this font,\0".as_ptr() as *const c_char
        );
        set_help_line!(
            2,
            b"because my memory for character-size data is too small.\0".as_ptr() as *const c_char
        );
        set_help_line!(
            1,
            b"If you're really stuck, ask a wizard to enlarge me.\0".as_ptr() as *const c_char
        );
        set_help_line!(
            0,
            b"Or maybe try `I\\font<same font id>=<name of loaded font>'.\0".as_ptr()
                as *const c_char
        );

        error();
        return FONT_BASE;
    }

    font_ptr += 1;
    *font_area.offset(font_ptr as isize) = native_font_type_flag;
    *font_name.offset(font_ptr as isize) = full_name;
    (*font_check.offset(font_ptr as isize)).s3 = 0;
    (*font_check.offset(font_ptr as isize)).s2 = 0;
    (*font_check.offset(font_ptr as isize)).s1 = 0;
    (*font_check.offset(font_ptr as isize)).s0 = 0;
    *font_glue.offset(font_ptr as isize) = TEX_NULL;
    *font_dsize.offset(font_ptr as isize) = get_loaded_font_design_size();
    *font_size.offset(font_ptr as isize) = actual_size;

    if native_font_type_flag == (AAT_FONT_FLAG) as i32 {
        aat_get_font_metrics(
            font_engine,
            &mut ascent,
            &mut descent,
            &mut x_ht,
            &mut cap_ht,
            &mut font_slant,
        );
    } else {
        ot_get_font_metrics(
            font_engine,
            &mut ascent,
            &mut descent,
            &mut x_ht,
            &mut cap_ht,
            &mut font_slant,
        );
    }

    *height_base.offset(font_ptr as isize) = ascent;
    *depth_base.offset(font_ptr as isize) = -descent;
    *font_params.offset(font_ptr as isize) = num_font_dimens;
    *font_bc.offset(font_ptr as isize) = 0;
    *font_ec.offset(font_ptr as isize) = 65535;
    *font_used.offset(font_ptr as isize) = false;
    *hyphen_char.offset(font_ptr as isize) = INTPAR!(INT_PAR__default_hyphen_char);
    *skew_char.offset(font_ptr as isize) = INTPAR!(INT_PAR__default_skew_char);
    *param_base.offset(font_ptr as isize) = fmem_ptr - 1;
    *font_layout_engine.offset(font_ptr as isize) = font_engine;
    *font_mapping.offset(font_ptr as isize) = std::ptr::null_mut();
    *font_letter_space.offset(font_ptr as isize) = loaded_font_letter_space;

    p = new_native_character(font_ptr, b' ' as UnicodeScalar);
    s = BOX_width!(p) + loaded_font_letter_space;
    if !NATIVE_NODE_glyph_info_ptr!(p).is_null() {
        NATIVE_NODE_glyph_info_ptr!(p) = mfree(NATIVE_NODE_glyph_info_ptr!(p));
    }
    free_node(p, NATIVE_NODE_size!(p) as i32);

    (*font_info.offset(fmem_ptr as isize)).b32.s1 = font_slant;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = s;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = s / 2;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = s / 3;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = x_ht;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = *font_size.offset(font_ptr as isize);
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = s / 3;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = cap_ht;
    fmem_ptr += 1;

    if num_font_dimens == 65 {
        (*font_info.offset(fmem_ptr as isize)).b32.s1 = num_font_dimens;
        fmem_ptr += 1;

        k = 0;
        while k <= 55 {
            (*font_info.offset(fmem_ptr as isize)).b32.s1 = get_ot_math_constant(font_ptr, k);
            fmem_ptr += 1;
            k += 1;
        }
    }

    *font_mapping.offset(font_ptr as isize) = loaded_font_mapping;
    *font_flags.offset(font_ptr as isize) = loaded_font_flags;
    font_ptr
}

// 19. do_locale_linebreaks
#[unsafe(export_name = "do_locale_linebreaks")]
pub unsafe extern "C" fn do_locale_linebreaks(s: i32, len: i32) {
    let mut offs: i32;
    let mut prevOffs: i32;
    let mut i: i32;
    let use_penalty: bool;
    let use_skip: bool;

    if INTPAR!(INT_PAR__xetex_linebreak_locale) == 0 || len == 1 {
        mem!(cur_list.tail).b32.s1 = new_native_word_node(main_f, len);
        cur_list.tail = LLIST_link!(cur_list.tail);

        i = 0;
        while i < len {
            *NATIVE_NODE_text!(cur_list.tail).offset(i as isize) =
                *native_text.offset((s + i) as isize);
            i += 1;
        }
        set_native_metrics(cur_list.tail, INTPAR!(INT_PAR__xetex_use_glyph_metrics) > 0);
    } else {
        use_skip = GLUEPAR!(GLUE_PAR__xetex_linebreak_skip) != 0;
        use_penalty = INTPAR!(INT_PAR__xetex_linebreak_penalty) != 0 || !use_skip;
        linebreak_start(
            main_f as c_int,
            INTPAR!(INT_PAR__xetex_linebreak_locale),
            native_text.offset(s as isize),
            len,
        );
        offs = 0;
        loop {
            prevOffs = offs;
            offs = linebreak_next(main_f as c_int);
            if offs > 0 {
                if prevOffs != 0 {
                    if use_penalty {
                        mem!(cur_list.tail).b32.s1 =
                            new_penalty(INTPAR!(INT_PAR__xetex_linebreak_penalty));
                        cur_list.tail = LLIST_link!(cur_list.tail);
                    }
                    if use_skip {
                        mem!(cur_list.tail).b32.s1 =
                            new_param_glue(GLUE_PAR__xetex_linebreak_skip as small_number);
                        cur_list.tail = LLIST_link!(cur_list.tail);
                    }
                }
                mem!(cur_list.tail).b32.s1 = new_native_word_node(main_f, offs - prevOffs);
                cur_list.tail = LLIST_link!(cur_list.tail);

                i = prevOffs;
                while i < offs {
                    *NATIVE_NODE_text!(cur_list.tail).offset((i - prevOffs) as isize) =
                        *native_text.offset((s + i) as isize);
                    i += 1;
                }
                set_native_metrics(cur_list.tail, INTPAR!(INT_PAR__xetex_use_glyph_metrics) > 0);
            }
            if offs < 0 {
                break;
            }
        }
    }
}

// 20. bad_utf8_warning
#[unsafe(export_name = "bad_utf8_warning")]
pub unsafe extern "C" fn bad_utf8_warning() {
    begin_diagnostic();
    diagnostic_begin_capture_warning_here();
    print_nl_cstr(b"Invalid UTF-8 byte or sequence\0".as_ptr() as *const c_char);
    if cur_input.name == 0 {
        print_cstr(b" in terminal input\0".as_ptr() as *const c_char);
    } else {
        print_cstr(b" at line \0".as_ptr() as *const c_char);
        print_int(line);
    }
    print_cstr(b" replaced by U+FFFD.\0".as_ptr() as *const c_char);
    capture_to_diagnostic(std::ptr::null_mut());
    end_diagnostic(false);
}

// 21. get_input_normalization_state
#[unsafe(export_name = "get_input_normalization_state")]
pub unsafe extern "C" fn get_input_normalization_state() -> i32 {
    if eqtb.is_null() {
        0
    } else {
        INTPAR!(INT_PAR__xetex_input_normalization)
    }
}

// 22. get_tracing_fonts_state
#[unsafe(export_name = "get_tracing_fonts_state")]
pub unsafe extern "C" fn get_tracing_fonts_state() -> i32 {
    INTPAR!(INT_PAR__xetex_tracing_fonts)
}

// 23. read_font_info
#[unsafe(export_name = "read_font_info")]
pub unsafe extern "C" fn read_font_info(
    u: i32,
    nom: str_number,
    aire: str_number,
    mut s: scaled_t,
) -> internal_font_number {
    const TTBC_FILE_FORMAT_TFM: c_int = 3;

    let mut k: font_index;
    let mut name_too_long: bool = false;
    let mut file_opened: bool = false;
    let mut lf: i32 = 0;
    let mut lh: i32 = 0;
    let mut bc: i32 = 0;
    let mut ec: i32 = 0;
    let mut nw: i32 = 0;
    let mut nh: i32 = 0;
    let mut nd: i32 = 0;
    let mut ni: i32 = 0;
    let mut nl: i32 = 0;
    let mut nk: i32 = 0;
    let mut ne: i32 = 0;
    let mut np: i32 = 0;
    let f: internal_font_number;
    let mut other_font: internal_font_number = FONT_BASE;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;
    let mut qw: b16x4 = b16x4 {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
    };
    let mut sw: scaled_t;
    let mut bch_label: i32;
    let mut local_bchar: i16;
    let mut z: scaled_t = 0;
    let mut alpha: i32;
    let mut beta: u8;
    let mut tfm_file: rust_input_handle_t = INVALID_HANDLE;
    let mut is_bad_tfm = false;

    pack_file_name(nom, aire, cur_ext);

    if INTPAR!(INT_PAR__xetex_tracing_fonts) > 0 {
        begin_diagnostic();
        diagnostic_begin_capture_warning_here();
        print_nl_cstr(b"Requested font \"\0".as_ptr() as *const c_char);
        print_c_string(name_of_file);
        print(b'"' as i32);
        if s < 0 {
            print_cstr(b" scaled \0".as_ptr() as *const c_char);
            print_int(-s);
        } else {
            print_cstr(b" at \0".as_ptr() as *const c_char);
            print_scaled(s);
            print_cstr(b"pt\0".as_ptr() as *const c_char);
        }
        capture_to_diagnostic(std::ptr::null_mut());
        end_diagnostic(false);
    }

    'done: {
        if quoted_filename {
            other_font = load_native_font(u, nom, aire, s);
            if other_font != FONT_BASE {
                break 'done;
            }
        }

        name_too_long = length(nom) > 255 || length(aire) > 255;
        if name_too_long {
            is_bad_tfm = true;
            break 'done;
        }

        pack_file_name(nom, aire, EMPTY_STRING);
        check_for_tfm_font_mapping();

        tfm_file = tt_xetex_open_input(TTBC_FILE_FORMAT_TFM);
        if tfm_file == INVALID_HANDLE {
            if !quoted_filename {
                other_font = load_native_font(u, nom, aire, s);
                if other_font != FONT_BASE {
                    break 'done;
                }
            }
            is_bad_tfm = true;
            break 'done;
        }

        file_opened = true;

        'read_tfm: {
            macro_rules! readfifteen {
                ($x:ident) => {
                    $x = ttstub_input_getc(tfm_file);
                    if $x > 127 || $x == libc::EOF {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    $x *= 256;
                    $x += ttstub_input_getc(tfm_file);
                };
            }

            readfifteen!(lf);
            readfifteen!(lh);
            readfifteen!(bc);
            readfifteen!(ec);

            if bc > ec + 1 || ec > 255 {
                is_bad_tfm = true;
                break 'read_tfm;
            }

            if bc > 255 {
                bc = 1;
                ec = 0;
            }

            readfifteen!(nw);
            readfifteen!(nh);
            readfifteen!(nd);
            readfifteen!(ni);
            readfifteen!(nl);
            readfifteen!(nk);
            readfifteen!(ne);
            readfifteen!(np);

            if lf != 6 + lh + (ec - bc + 1) + nw + nh + nd + ni + nl + nk + ne + np {
                is_bad_tfm = true;
                break 'read_tfm;
            }
            if nw == 0 || nh == 0 || nd == 0 || ni == 0 {
                is_bad_tfm = true;
                break 'read_tfm;
            }

            lf = lf - 6 - lh;
            if np < 7 {
                lf = lf + 7 - np;
            }

            if font_ptr == font_max || fmem_ptr + lf > font_mem_size {
                _tt_abort(b"not enough memory to load another font\0".as_ptr() as *const c_char);
            }

            f = font_ptr + 1;
            *char_base.offset(f as isize) = fmem_ptr - bc;
            *width_base.offset(f as isize) = *char_base.offset(f as isize) + ec + 1;
            *height_base.offset(f as isize) = *width_base.offset(f as isize) + nw;
            *depth_base.offset(f as isize) = *height_base.offset(f as isize) + nh;
            *italic_base.offset(f as isize) = *depth_base.offset(f as isize) + nd;
            *lig_kern_base.offset(f as isize) = *italic_base.offset(f as isize) + ni;
            *kern_base.offset(f as isize) = *lig_kern_base.offset(f as isize) + nl - 256 * 128;
            *exten_base.offset(f as isize) = *kern_base.offset(f as isize) + 256 * 128 + nk;
            *param_base.offset(f as isize) = *exten_base.offset(f as isize) + ne;

            if lh < 2 {
                is_bad_tfm = true;
                break 'read_tfm;
            }

            a = ttstub_input_getc(tfm_file);
            qw.s3 = a as u16;
            b = ttstub_input_getc(tfm_file);
            qw.s2 = b as u16;
            c = ttstub_input_getc(tfm_file);
            qw.s1 = c as u16;
            d = ttstub_input_getc(tfm_file);
            qw.s0 = d as u16;
            if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                is_bad_tfm = true;
                break 'read_tfm;
            }
            *font_check.offset(f as isize) = qw;

            readfifteen!(z);
            z = z * 256 + ttstub_input_getc(tfm_file);
            z = (z * 16) + (ttstub_input_getc(tfm_file) / 16);
            if z < 65536 {
                is_bad_tfm = true;
                break 'read_tfm;
            }

            while lh > 2 {
                ttstub_input_getc(tfm_file);
                ttstub_input_getc(tfm_file);
                ttstub_input_getc(tfm_file);
                ttstub_input_getc(tfm_file);
                lh -= 1;
            }

            *font_dsize.offset(f as isize) = z;
            if s != -1000 {
                if s >= 0 {
                    z = s;
                } else {
                    z = xn_over_d(z, -s, 1000);
                }
            }

            *font_size.offset(f as isize) = z;

            k = fmem_ptr;
            while k <= *width_base.offset(f as isize) - 1 {
                a = ttstub_input_getc(tfm_file);
                qw.s3 = a as u16;
                b = ttstub_input_getc(tfm_file);
                qw.s2 = b as u16;
                c = ttstub_input_getc(tfm_file);
                qw.s1 = c as u16;
                d = ttstub_input_getc(tfm_file);
                qw.s0 = d as u16;
                if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                (*font_info.offset(k as isize)).b16 = qw;

                if a >= nw || b / 16 >= nh || b % 16 >= nd || c / 4 >= ni {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }

                match c % 4 {
                    1 => {
                        if d >= nl {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }
                    }
                    3 => {
                        if d >= ne {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }
                    }
                    2 => {
                        if d < bc || d > ec {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }

                        let mut not_found = false;
                        while d < k + bc - fmem_ptr {
                            qw = FONT_CHARACTER_INFO!(f, d);
                            if (qw.s1 % 4) != (LIST_TAG as i8) as u16 {
                                not_found = true;
                                break;
                            }
                            d = qw.s0 as i32;
                        }

                        if !not_found && d == k + bc - fmem_ptr {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }
                    }
                    _ => {}
                }
                k += 1;
            }

            alpha = 16;
            while z >= 0x800000 {
                z = z / 2;
                alpha = alpha + alpha;
            }
            beta = (256 / alpha) as u8;
            alpha = alpha * z;

            k = *width_base.offset(f as isize);
            while k <= *lig_kern_base.offset(f as isize) - 1 {
                a = ttstub_input_getc(tfm_file);
                b = ttstub_input_getc(tfm_file);
                c = ttstub_input_getc(tfm_file);
                d = ttstub_input_getc(tfm_file);
                if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                sw = (((((d * z) / 256) + c * z) / 256) + b * z) / (beta as i32);

                if a == 0 {
                    (*font_info.offset(k as isize)).b32.s1 = sw;
                } else if a == 255 {
                    (*font_info.offset(k as isize)).b32.s1 = sw - alpha;
                } else {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                k += 1;
            }

            if (*font_info.offset(*width_base.offset(f as isize) as isize))
                .b32
                .s1
                != 0
            {
                is_bad_tfm = true;
                break 'read_tfm;
            }
            if (*font_info.offset(*height_base.offset(f as isize) as isize))
                .b32
                .s1
                != 0
            {
                is_bad_tfm = true;
                break 'read_tfm;
            }
            if (*font_info.offset(*depth_base.offset(f as isize) as isize))
                .b32
                .s1
                != 0
            {
                is_bad_tfm = true;
                break 'read_tfm;
            }
            if (*font_info.offset(*italic_base.offset(f as isize) as isize))
                .b32
                .s1
                != 0
            {
                is_bad_tfm = true;
                break 'read_tfm;
            }

            bch_label = 32767;
            local_bchar = 256;

            if nl > 0 {
                k = *lig_kern_base.offset(f as isize);
                while k <= *kern_base.offset(f as isize) + 256 * 128 - 1 {
                    a = ttstub_input_getc(tfm_file);
                    qw.s3 = a as u16;
                    b = ttstub_input_getc(tfm_file);
                    qw.s2 = b as u16;
                    c = ttstub_input_getc(tfm_file);
                    qw.s1 = c as u16;
                    d = ttstub_input_getc(tfm_file);
                    qw.s0 = d as u16;
                    if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    (*font_info.offset(k as isize)).b16 = qw;

                    if a > 128 {
                        if 256 * c + d >= nl {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }

                        if a == 255 && k == *lig_kern_base.offset(f as isize) {
                            local_bchar = b as i16;
                        }
                    } else {
                        if b != local_bchar as i32 {
                            if b < bc || b > ec {
                                is_bad_tfm = true;
                                break 'read_tfm;
                            }

                            qw = FONT_CHARACTER_INFO!(f, b);
                            if !(qw.s3 > 0) {
                                is_bad_tfm = true;
                                break 'read_tfm;
                            }
                        }

                        if c < 128 {
                            if d < bc || d > ec {
                                is_bad_tfm = true;
                                break 'read_tfm;
                            }
                            qw = FONT_CHARACTER_INFO!(f, d);
                            if !(qw.s3 > 0) {
                                is_bad_tfm = true;
                                break 'read_tfm;
                            }
                        } else if 256 * (c - 128) + d >= nk {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }

                        if a < 128 && k - *lig_kern_base.offset(f as isize) + a + 1 >= nl {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }
                    }
                    k += 1;
                }

                if a == 255 {
                    bch_label = 256 * c + d;
                }
            }

            k = *kern_base.offset(f as isize) + 256 * 128;
            while k <= *exten_base.offset(f as isize) - 1 {
                a = ttstub_input_getc(tfm_file);
                b = ttstub_input_getc(tfm_file);
                c = ttstub_input_getc(tfm_file);
                d = ttstub_input_getc(tfm_file);
                if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                sw = (((((d * z) / 256) + c * z) / 256) + b * z) / (beta as i32);

                if a == 0 {
                    (*font_info.offset(k as isize)).b32.s1 = sw;
                } else if a == 255 {
                    (*font_info.offset(k as isize)).b32.s1 = sw - alpha;
                } else {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                k += 1;
            }

            k = *exten_base.offset(f as isize);
            while k <= *param_base.offset(f as isize) - 1 {
                a = ttstub_input_getc(tfm_file);
                qw.s3 = a as u16;
                b = ttstub_input_getc(tfm_file);
                qw.s2 = b as u16;
                c = ttstub_input_getc(tfm_file);
                qw.s1 = c as u16;
                d = ttstub_input_getc(tfm_file);
                qw.s0 = d as u16;
                if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                (*font_info.offset(k as isize)).b16 = qw;

                if a != 0 {
                    if a < bc || a > ec {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    qw = FONT_CHARACTER_INFO!(f, a);
                    if !(qw.s3 > 0) {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                }

                if b != 0 {
                    if b < bc || b > ec {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    qw = FONT_CHARACTER_INFO!(f, b);
                    if !(qw.s3 > 0) {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                }

                if c != 0 {
                    if c < bc || c > ec {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    qw = FONT_CHARACTER_INFO!(f, c);
                    if !(qw.s3 > 0) {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                }

                if d < bc || d > ec {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                qw = FONT_CHARACTER_INFO!(f, d);
                if !(qw.s3 > 0) {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                k += 1;
            }

            k = 1;
            while k <= np {
                if k == 1 {
                    sw = ttstub_input_getc(tfm_file);
                    if sw == libc::EOF {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    if sw > 127 {
                        sw = sw - 256;
                    }

                    sw = sw * 256 + ttstub_input_getc(tfm_file);
                    sw = sw * 256 + ttstub_input_getc(tfm_file);
                    (*font_info.offset(*param_base.offset(f as isize) as isize))
                        .b32
                        .s1 = (sw * 16) + (ttstub_input_getc(tfm_file) / 16);
                } else {
                    a = ttstub_input_getc(tfm_file);
                    b = ttstub_input_getc(tfm_file);
                    c = ttstub_input_getc(tfm_file);
                    d = ttstub_input_getc(tfm_file);
                    if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    sw = (((((d * z) / 256) + c * z) / 256) + b * z) / (beta as i32);

                    if a == 0 {
                        (*font_info.offset((*param_base.offset(f as isize) + k - 1) as isize))
                            .b32
                            .s1 = sw;
                    } else if a == 255 {
                        (*font_info.offset((*param_base.offset(f as isize) + k - 1) as isize))
                            .b32
                            .s1 = sw - alpha;
                    } else {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                }
                k += 1;
            }

            k = np + 1;
            while k <= 7 {
                (*font_info.offset((*param_base.offset(f as isize) + k - 1) as isize))
                    .b32
                    .s1 = 0;
                k += 1;
            }

            if np >= 7 {
                *font_params.offset(f as isize) = np;
            } else {
                *font_params.offset(f as isize) = 7;
            }

            *hyphen_char.offset(f as isize) = INTPAR!(INT_PAR__default_hyphen_char);
            *skew_char.offset(f as isize) = INTPAR!(INT_PAR__default_skew_char);
            if bch_label < nl {
                *bchar_label.offset(f as isize) = bch_label + *lig_kern_base.offset(f as isize);
            } else {
                *bchar_label.offset(f as isize) = NON_ADDRESS;
            }
            *font_bchar.offset(f as isize) = local_bchar as i32;
            *font_false_bchar.offset(f as isize) = local_bchar as i32;

            if local_bchar <= ec as i16 {
                if local_bchar >= bc as i16 {
                    qw = FONT_CHARACTER_INFO!(f, local_bchar);
                    if qw.s3 > 0 {
                        *font_false_bchar.offset(f as isize) = TOO_BIG_CHAR;
                    }
                }
            }

            *font_name.offset(f as isize) = nom;
            *font_area.offset(f as isize) = aire;
            *font_bc.offset(f as isize) = (bc) as u16;
            *font_ec.offset(f as isize) = (ec) as u16;
            *font_glue.offset(f as isize) = TEX_NULL;
            *param_base.offset(f as isize) -= 1;
            fmem_ptr = fmem_ptr + lf;
            font_ptr = f;
            other_font = f;
            *font_mapping.offset(f as isize) = load_tfm_font_mapping();
        }
    }

    if is_bad_tfm {
        if INTPAR!(INT_PAR__suppress_fontnotfound_error) == 0 {
            error_here_with_diagnostic(b"Font \0".as_ptr() as *const c_char);
            sprint_cs(u);
            print_char(b'=' as i32);
            if file_name_quote_char != 0 {
                print_char(file_name_quote_char as i32);
            }
            print_file_name(nom, aire, cur_ext);
            if file_name_quote_char != 0 {
                print_char(file_name_quote_char as i32);
            }
            if s >= 0 {
                print_cstr(b" at \0".as_ptr() as *const c_char);
                print_scaled(s);
                print_cstr(b"pt\0".as_ptr() as *const c_char);
            } else if s != -1000 {
                print_cstr(b" scaled \0".as_ptr() as *const c_char);
                print_int(-s);
            }

            if file_opened {
                print_cstr(b" not loadable: Bad metric (TFM) file\0".as_ptr() as *const c_char);
            } else if name_too_long {
                print_cstr(
                    b" not loadable: Metric (TFM) file name too long\0".as_ptr() as *const c_char
                );
            } else {
                print_cstr(
                    b" not loadable: Metric (TFM) file or installed font not found\0".as_ptr()
                        as *const c_char,
                );
            }
            capture_to_diagnostic(std::ptr::null_mut());

            help_ptr = 5;
            set_help_line!(
                4,
                b"I wasn't able to read the size data for this font,\0".as_ptr() as *const c_char
            );
            set_help_line!(
                3,
                b"so I will ignore the font specification.\0".as_ptr() as *const c_char
            );
            set_help_line!(
                2,
                b"[Wizards can fix TFM files using TFtoPL/PLtoTF.]\0".as_ptr() as *const c_char
            );
            set_help_line!(
                1,
                b"You might try inserting a different font spec;\0".as_ptr() as *const c_char
            );
            set_help_line!(
                0,
                b"e.g., type `I\\font<same font id>=<substitute font name>'.\0".as_ptr()
                    as *const c_char
            );

            error();
        }
    }

    if file_opened {
        ttstub_input_close(tfm_file);
    }

    if INTPAR!(INT_PAR__xetex_tracing_fonts) > 0 {
        if other_font == FONT_BASE || file_opened {
            begin_diagnostic();
            diagnostic_begin_capture_warning_here();

            print_nl_cstr(b" -> \0".as_ptr() as *const c_char);
            if other_font == FONT_BASE {
                print_c_string(b"font not found, using \"nullfont\"\0".as_ptr() as *const c_char);
            } else {
                print_c_string(name_of_file);
            }

            capture_to_diagnostic(std::ptr::null_mut());
            end_diagnostic(false);
        }
    }

    other_font
}

// 24. new_character
#[unsafe(export_name = "new_character")]
pub unsafe extern "C" fn new_character(f: internal_font_number, c: UTF16_code) -> i32 {
    let p: i32;
    let ec: u16;
    if *font_area.offset(f as isize) == (AAT_FONT_FLAG) as i32
        || *font_area.offset(f as isize) == (OTGR_FONT_FLAG) as i32
    {
        return new_native_character(f, c as UnicodeScalar);
    }
    ec = effective_char(false, f, c) as u16;
    if (*font_bc.offset(f as isize) as u32) <= ec as u32 {
        if (*font_ec.offset(f as isize) as u32) >= ec as u32 {
            if FONT_CHARACTER_INFO!(f, ec).s3 > 0 {
                p = get_avail();
                mem!(p).b16.s1 = (f as i16) as u16;
                mem!(p).b16.s0 = (c as i16) as u16;
                return p;
            }
        }
    }
    char_warning(f, c as i32);
    TEX_NULL
}

// 25. scan_spec
#[unsafe(export_name = "scan_spec")]
pub unsafe extern "C" fn scan_spec(c: group_code, three_codes: bool) {
    let mut s: i32 = 0;
    let spec_code: u8;
    if three_codes {
        s = (*save_stack.offset(save_ptr as isize)).b32.s1;
    }
    if scan_keyword(b"to\0".as_ptr() as *const c_char) {
        spec_code = EXACTLY as u8;
        scan_dimen(false, false, false);
    } else if scan_keyword(b"spread\0".as_ptr() as *const c_char) {
        spec_code = ADDITIONAL as u8;
        scan_dimen(false, false, false);
    } else {
        spec_code = ADDITIONAL as u8;
        cur_val = 0;
    }

    if three_codes {
        (*save_stack.offset(save_ptr as isize)).b32.s1 = s;
        save_ptr += 1;
    }
    (*save_stack.offset(save_ptr as isize)).b32.s1 = spec_code as i32;
    (*save_stack.offset((save_ptr + 1) as isize)).b32.s1 = cur_val;
    save_ptr += 2;
    new_save_level(c);
    scan_left_brace();
}

// 26. char_pw
#[unsafe(export_name = "char_pw")]
pub unsafe extern "C" fn char_pw(mut p: i32, side: small_number) -> scaled_t {
    let f: internal_font_number;
    let c: i32;
    if side == 0 {
        last_leftmost_char = TEX_NULL;
    } else {
        last_rightmost_char = TEX_NULL;
    }
    if p == TEX_NULL {
        return 0;
    }
    if p != TEX_NULL
        && !is_char_node(p)
        && NODE_type!(p) == (WHATSIT_NODE as i16) as u16
        && (NODE_subtype!(p) == (NATIVE_WORD_NODE as i16) as u16
            || NODE_subtype!(p) == (NATIVE_WORD_NODE_AT as i16) as u16)
    {
        if !NATIVE_NODE_glyph_info_ptr!(p).is_null() {
            f = NATIVE_NODE_font!(p) as internal_font_number;
            return round_xn_over_d(
                (*font_info.offset((QUAD_CODE + *param_base.offset(f as isize)) as isize))
                    .b32
                    .s1,
                get_native_word_cp!(p, side),
                1000,
            );
        } else {
            return 0;
        }
    }
    if p != TEX_NULL
        && !is_char_node(p)
        && NODE_type!(p) == (WHATSIT_NODE as i16) as u16
        && mem!(p).b16.s0 == (GLYPH_NODE as i16) as u16
    {
        f = mem!(p + 4).b16.s2 as internal_font_number;
        return round_xn_over_d(
            (*font_info.offset((QUAD_CODE + *param_base.offset(f as isize)) as isize))
                .b32
                .s1,
            get_cp_code(
                f as c_int,
                (mem!(p + 4).b16.s1 as c_int) as u32,
                side as c_int,
            ),
            1000,
        );
    }
    if !is_char_node(p) {
        if NODE_type!(p) == (LIGATURE_NODE as i16) as u16 {
            p = p + 1;
        } else {
            return 0;
        }
    }
    f = CHAR_NODE_font!(p) as internal_font_number;
    c = get_cp_code(f as c_int, (mem!(p).b16.s0 as c_int) as u32, side as c_int);
    match side {
        0 => {
            last_leftmost_char = p;
        }
        1 => {
            last_rightmost_char = p;
        }
        _ => {}
    }
    if c == 0 {
        return 0;
    }
    round_xn_over_d(
        (*font_info.offset((QUAD_CODE + *param_base.offset(f as isize)) as isize))
            .b32
            .s1,
        c,
        1000,
    )
}

// 27. new_margin_kern
#[unsafe(export_name = "new_margin_kern")]
pub unsafe extern "C" fn new_margin_kern(w: scaled_t, p: i32, side: small_number) -> i32 {
    let k = get_node(MARGIN_KERN_NODE_SIZE);
    NODE_type!(k) = (MARGIN_KERN_NODE as i16) as u16;
    mem!(k).b16.s0 = (side as i16) as u16;
    mem!(k + 1).b32.s1 = w;
    k
}

// 28. prepare_native_line_layouts
#[unsafe(export_name = "prepare_native_line_layouts")]
pub unsafe extern "C" fn prepare_native_line_layouts(
    lines: *const i32,
    count: usize,
) -> *mut PreparedLayouts {
    let mut requests: *mut NativeLayoutRequest = std::ptr::null_mut();
    let mut request_count: usize = 0;
    let mut capacity: usize = 0;

    for line_index in 0..count {
        let mut p = *lines.add(line_index);
        let mut remaining: i32 = 0;
        while p != TEX_NULL {
            if !is_char_node(p)
                && NODE_type!(p) == (WHATSIT_NODE as i16) as u16
                && (NODE_subtype!(p) == (NATIVE_WORD_NODE as i16) as u16
                    || NODE_subtype!(p) == (NATIVE_WORD_NODE_AT as i16) as u16)
                && *font_area.offset(NATIVE_NODE_font!(p) as isize) == (OTGR_FONT_FLAG) as i32
                && remaining <= 0
            {
                let mut end = LLIST_link!(p);
                let mut cursor: i32;
                let mut length: usize = 0;
                loop {
                    if end == TEX_NULL || is_char_node(end) {
                        break;
                    }
                    cursor = end;
                    if NODE_type!(cursor) == (DISC_NODE as i16) as u16 {
                        cursor = LLIST_link!(cursor);
                    }
                    if cursor == TEX_NULL
                        || is_char_node(cursor)
                        || NODE_type!(cursor) != (WHATSIT_NODE as i16) as u16
                        || (NODE_subtype!(cursor) != (NATIVE_WORD_NODE as i16) as u16
                            && NODE_subtype!(cursor) != (NATIVE_WORD_NODE_AT as i16) as u16)
                        || NATIVE_NODE_font!(cursor) != NATIVE_NODE_font!(p)
                    {
                        break;
                    }
                    end = LLIST_link!(cursor);
                }
                if end != LLIST_link!(p) {
                    cursor = p;
                    while cursor != end {
                        if NODE_type!(cursor) == (WHATSIT_NODE as i16) as u16 {
                            length += NATIVE_NODE_length!(cursor) as usize;
                        }
                        cursor = LLIST_link!(cursor);
                    }
                    let text = xmalloc(length * std::mem::size_of::<u16>()) as *mut u16;
                    length = 0;
                    cursor = p;
                    while cursor != end {
                        if NODE_type!(cursor) == (WHATSIT_NODE as i16) as u16 {
                            let cur_len = NATIVE_NODE_length!(cursor) as usize;
                            libc::memcpy(
                                text.add(length) as *mut c_void,
                                NATIVE_NODE_text!(cursor) as *const c_void,
                                cur_len * std::mem::size_of::<u16>(),
                            );
                            length += cur_len;
                        }
                        cursor = LLIST_link!(cursor);
                    }
                    if request_count == capacity {
                        capacity = if capacity != 0 { 2 * capacity } else { 16 };
                        requests = xrealloc(
                            requests as *mut c_void,
                            capacity * std::mem::size_of::<NativeLayoutRequest>(),
                        ) as *mut NativeLayoutRequest;
                    }
                    (*requests.add(request_count)).engine =
                        *font_layout_engine.offset(NATIVE_NODE_font!(p) as isize);
                    (*requests.add(request_count)).text = text;
                    (*requests.add(request_count)).length = length;
                    request_count += 1;
                }
                p = end;
                continue;
            }
            if !is_char_node(p) && NODE_type!(p) == (DISC_NODE as i16) as u16 {
                remaining = DISCRETIONARY_NODE_replace_count!(p) as i32;
            } else {
                remaining -= 1;
            }
            p = LLIST_link!(p);
        }
    }

    let prepared = ttxl_prepare_layouts(requests, request_count);
    for line_index in 0..request_count {
        libc::free((*requests.add(line_index)).text as *mut c_void);
    }
    libc::free(requests as *mut c_void);
    prepared
}

// 29. hpack
#[unsafe(export_name = "hpack")]
pub unsafe extern "C" fn hpack(mut p: i32, mut w: scaled_t, m: small_number) -> i32 {
    let r: i32;
    let mut q: i32;
    let mut h: scaled_t;
    let mut d: scaled_t;
    let mut x: scaled_t;
    let mut s: scaled_t;
    let mut other_font: i32;
    let mut o: glue_ord;
    let mut f: internal_font_number;
    let mut i: b16x4;
    let mut pp: i32;
    let mut ppp: i32 = TEX_NULL;
    let mut total_chars: i32;
    let mut k: i32;

    last_badness = 0;
    r = get_node(BOX_NODE_SIZE);
    NODE_type!(r) = (HLIST_NODE as i16) as u16;
    mem!(r).b16.s0 = 0;
    mem!(r + 4).b32.s1 = 0;
    q = r + 5;
    mem!(q).b32.s1 = p;
    h = 0;
    d = 0;
    x = 0;
    total_stretch[NORMAL as usize] = 0;
    total_shrink[NORMAL as usize] = 0;
    total_stretch[FIL as usize] = 0;
    total_shrink[FIL as usize] = 0;
    total_stretch[FILL as usize] = 0;
    total_shrink[FILL as usize] = 0;
    total_stretch[FILLL as usize] = 0;
    total_shrink[FILLL as usize] = 0;

    if INTPAR!(INT_PAR__texxet) > 0 {
        temp_ptr = get_avail();
        mem!(temp_ptr).b32.s0 = BEFORE;
        mem!(temp_ptr).b32.s1 = LR_ptr;
        LR_ptr = temp_ptr;
    }

    while p != TEX_NULL {
        loop {
            while is_char_node(p) {
                f = CHAR_NODE_font!(p) as internal_font_number;
                i = FONT_CHARACTER_INFO!(f, effective_char(true, f, CHAR_NODE_character!(p)));
                x = x + FONT_CHARINFO_WIDTH!(f, i);
                s = FONT_CHARINFO_HEIGHT!(f, i);
                if s > h {
                    h = s;
                }
                s = FONT_CHARINFO_DEPTH!(f, i);
                if s > d {
                    d = s;
                }
                p = LLIST_link!(p);
            }

            if p != TEX_NULL {
                match mem!(p).b16.s1 as i32 {
                    0 | 1 | 2 | 13 => {
                        x = x + mem!(p + 1).b32.s1;
                        if NODE_type!(p) >= (RULE_NODE as i16) as u16 {
                            s = 0;
                        } else {
                            s = mem!(p + 4).b32.s1;
                        }
                        if mem!(p + 3).b32.s1 - s > h {
                            h = mem!(p + 3).b32.s1 - s;
                        }
                        if mem!(p + 2).b32.s1 + s > d {
                            d = mem!(p + 2).b32.s1 + s;
                        }
                    }
                    3 | 4 | 5 => {
                        if adjust_tail != TEX_NULL || pre_adjust_tail != TEX_NULL {
                            while mem!(q).b32.s1 != p {
                                q = LLIST_link!(q);
                            }
                            if NODE_type!(p) == (ADJUST_NODE as i16) as u16 {
                                if mem!(p).b16.s0 != 0 {
                                    if pre_adjust_tail == TEX_NULL {
                                        confusion(b"pre vadjust\0".as_ptr() as *const c_char);
                                    }
                                    mem!(pre_adjust_tail).b32.s1 = mem!(p + 1).b32.s1;
                                    while mem!(pre_adjust_tail).b32.s1 != TEX_NULL {
                                        pre_adjust_tail = LLIST_link!(pre_adjust_tail);
                                    }
                                } else {
                                    if adjust_tail == TEX_NULL {
                                        confusion(b"pre vadjust\0".as_ptr() as *const c_char);
                                    }
                                    mem!(adjust_tail).b32.s1 = mem!(p + 1).b32.s1;
                                    while mem!(adjust_tail).b32.s1 != TEX_NULL {
                                        adjust_tail = LLIST_link!(adjust_tail);
                                    }
                                }
                                p = LLIST_link!(p);
                                free_node(mem!(q).b32.s1, SMALL_NODE_SIZE);
                            } else {
                                mem!(adjust_tail).b32.s1 = p;
                                adjust_tail = p;
                                p = LLIST_link!(p);
                            }
                            mem!(q).b32.s1 = p;
                            p = q;
                        }
                    }
                    8 => match mem!(p).b16.s0 as i32 {
                        40 | 41 => {
                            if q != r + 5 && NODE_type!(q) == (DISC_NODE as i16) as u16 {
                                k = mem!(q).b16.s0 as i32;
                            } else {
                                k = 0;
                            }
                            while mem!(q).b32.s1 != p {
                                k -= 1;
                                q = LLIST_link!(q);
                                if NODE_type!(q) == (DISC_NODE as i16) as u16 {
                                    k = mem!(q).b16.s0 as i32;
                                }
                            }
                            pp = mem!(p).b32.s1;

                            while k <= 0 && pp != TEX_NULL && !is_char_node(pp) {
                                if NODE_type!(pp) == (WHATSIT_NODE as i16) as u16
                                    && (mem!(pp).b16.s0 == (NATIVE_WORD_NODE as i16) as u16
                                        || mem!(pp).b16.s0 == (NATIVE_WORD_NODE_AT as i16) as u16)
                                    && mem!(pp + 4).b16.s2 == mem!(p + 4).b16.s2
                                {
                                    pp = LLIST_link!(pp);
                                } else if NODE_type!(pp) == (DISC_NODE as i16) as u16 {
                                    ppp = mem!(pp).b32.s1;
                                    if ppp != TEX_NULL
                                        && !is_char_node(ppp)
                                        && NODE_type!(ppp) == (WHATSIT_NODE as i16) as u16
                                        && (mem!(ppp).b16.s0 == (NATIVE_WORD_NODE as i16) as u16
                                            || mem!(ppp).b16.s0
                                                == (NATIVE_WORD_NODE_AT as i16) as u16)
                                        && mem!(ppp + 4).b16.s2 == mem!(p + 4).b16.s2
                                    {
                                        pp = mem!(ppp).b32.s1;
                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }

                            if pp != mem!(p).b32.s1 {
                                total_chars = 0;
                                p = mem!(q).b32.s1;
                                while p != pp {
                                    if NODE_type!(p) == (WHATSIT_NODE as i16) as u16 {
                                        total_chars = total_chars + mem!(p + 4).b16.s1 as i32;
                                    }
                                    ppp = p;
                                    p = LLIST_link!(p);
                                }
                                p = mem!(q).b32.s1;
                                pp = new_native_word_node(
                                    mem!(p + 4).b16.s2 as internal_font_number,
                                    total_chars,
                                );
                                mem!(pp).b16.s0 = mem!(p).b16.s0;
                                mem!(q).b32.s1 = pp;
                                mem!(pp).b32.s1 = mem!(ppp).b32.s1;
                                mem!(ppp).b32.s1 = TEX_NULL;
                                total_chars = 0;
                                ppp = p;
                                loop {
                                    if NODE_type!(ppp) == (WHATSIT_NODE as i16) as u16 {
                                        let for_end = mem!(ppp + 4).b16.s1 as i32 - 1;
                                        k = 0;
                                        while k <= for_end {
                                            *NATIVE_NODE_text!(pp).offset(total_chars as isize) =
                                                *NATIVE_NODE_text!(ppp).offset(k as isize);
                                            total_chars += 1;
                                            k += 1;
                                        }
                                    }
                                    ppp = LLIST_link!(ppp);
                                    if ppp == TEX_NULL {
                                        break;
                                    }
                                }
                                flush_node_list(p);
                                p = mem!(q).b32.s1;
                                set_native_metrics(
                                    p,
                                    INTPAR!(INT_PAR__xetex_use_glyph_metrics) > 0,
                                );
                            }
                            if mem!(p + 3).b32.s1 > h {
                                h = mem!(p + 3).b32.s1;
                            }
                            if mem!(p + 2).b32.s1 > d {
                                d = mem!(p + 2).b32.s1;
                            }
                            x = x + mem!(p + 1).b32.s1;
                        }
                        42 | 43 | 44 => {
                            if mem!(p + 3).b32.s1 > h {
                                h = mem!(p + 3).b32.s1;
                            }
                            if mem!(p + 2).b32.s1 > d {
                                d = mem!(p + 2).b32.s1;
                            }
                            x = x + mem!(p + 1).b32.s1;
                        }
                        _ => {}
                    },
                    10 => {
                        other_font = mem!(p + 1).b32.s0;
                        x = x + mem!(other_font + 1).b32.s1;
                        o = mem!(other_font).b16.s1 as glue_ord;
                        total_stretch[o as usize] =
                            total_stretch[o as usize] + mem!(other_font + 2).b32.s1;
                        o = mem!(other_font).b16.s0 as glue_ord;
                        total_shrink[o as usize] =
                            total_shrink[o as usize] + mem!(other_font + 3).b32.s1;
                        if mem!(p).b16.s0 >= (A_LEADERS as i16) as u16 {
                            other_font = mem!(p + 1).b32.s1;
                            if mem!(other_font + 3).b32.s1 > h {
                                h = mem!(other_font + 3).b32.s1;
                            }
                            if mem!(other_font + 2).b32.s1 > d {
                                d = mem!(other_font + 2).b32.s1;
                            }
                        }
                    }
                    11 | 40 => {
                        x = x + mem!(p + 1).b32.s1;
                    }
                    9 => {
                        x = x + mem!(p + 1).b32.s1;
                        if INTPAR!(INT_PAR__texxet) > 0 {
                            if odd(mem!(p).b16.s0 as i32) {
                                if mem!(LR_ptr).b32.s0
                                    == (L_CODE * (mem!(p).b16.s0 as i32 / L_CODE) + 3)
                                {
                                    temp_ptr = LR_ptr;
                                    LR_ptr = mem!(temp_ptr).b32.s1;
                                    mem!(temp_ptr).b32.s1 = avail;
                                    avail = temp_ptr;
                                } else {
                                    LR_problems += 1;
                                    NODE_type!(p) = (KERN_NODE as i16) as u16;
                                    NODE_subtype!(p) = (EXPLICIT as i16) as u16;
                                }
                            } else {
                                temp_ptr = get_avail();
                                mem!(temp_ptr).b32.s0 =
                                    L_CODE * (mem!(p).b16.s0 as i32 / L_CODE) + 3;
                                mem!(temp_ptr).b32.s1 = LR_ptr;
                                LR_ptr = temp_ptr;
                            }
                        }
                    }
                    6 => {
                        mem!(GARBAGE) = mem!(p + 1);
                        mem!(GARBAGE).b32.s1 = mem!(p).b32.s1;
                        p = GARBAGE;
                        xtx_ligature_present = true;
                        continue;
                    }
                    _ => {}
                }
                p = LLIST_link!(p);
            }
            break;
        }
    }

    if adjust_tail != TEX_NULL {
        mem!(adjust_tail).b32.s1 = TEX_NULL;
    }
    if pre_adjust_tail != TEX_NULL {
        mem!(pre_adjust_tail).b32.s1 = TEX_NULL;
    }
    mem!(r + 3).b32.s1 = h;
    mem!(r + 2).b32.s1 = d;
    if m as i32 == ADDITIONAL {
        w = x + w;
    }
    mem!(r + 1).b32.s1 = w;
    x = w - x;

    let do_common_ending = |r: i32| {
        if output_active {
            print_cstr(b") has occurred while \\output is active\0".as_ptr() as *const c_char);
        } else {
            if pack_begin_line != 0 {
                if pack_begin_line > 0 {
                    print_cstr(b") in paragraph at lines \0".as_ptr() as *const c_char);
                } else {
                    print_cstr(b") in alignment at lines \0".as_ptr() as *const c_char);
                }
                print_int(pack_begin_line.abs());
                print_cstr(b"--\0".as_ptr() as *const c_char);
            } else {
                print_cstr(b") detected at line \0".as_ptr() as *const c_char);
            }
            print_int(line);
        }

        capture_to_diagnostic(std::ptr::null_mut());

        print_ln();
        font_in_short_display = FONT_BASE;
        short_display(mem!(r + 5).b32.s1);
        print_ln();
        begin_diagnostic();
        show_box(r);
        end_diagnostic(true);
    };

    'check_box: {
        if x == 0 {
            mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
            mem!(r + 5).b16.s0 = (NORMAL as i16) as u16;
            BOX_glue_set!(r) = 0.0;
            break 'check_box;
        } else if x > 0 {
            if total_stretch[FILLL as usize] != 0 {
                o = (FILLL) as u8;
            } else if total_stretch[FILL as usize] != 0 {
                o = (FILL) as u8;
            } else if total_stretch[FIL as usize] != 0 {
                o = (FIL) as u8;
            } else {
                o = 0;
            }
            mem!(r + 5).b16.s0 = (o as i16) as u16;
            mem!(r + 5).b16.s1 = (STRETCHING as i16) as u16;
            if total_stretch[o as usize] != 0 {
                BOX_glue_set!(r) = (x as f64) / (total_stretch[o as usize] as f64);
            } else {
                mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
                BOX_glue_set!(r) = 0.0;
            }

            if o == (NORMAL) as u8 {
                if mem!(r + 5).b32.s1 != TEX_NULL {
                    last_badness = badness(x, total_stretch[NORMAL as usize]);
                    if last_badness > INTPAR!(INT_PAR__hbadness) {
                        print_ln();
                        diagnostic_begin_capture_warning_here();
                        if last_badness > 100 {
                            print_nl_cstr(b"Underfull\0".as_ptr() as *const c_char);
                        } else {
                            print_nl_cstr(b"Loose\0".as_ptr() as *const c_char);
                        }
                        print_cstr(b" \\hbox (badness \0".as_ptr() as *const c_char);
                        print_int(last_badness);
                        do_common_ending(r);
                    }
                }
            }
            break 'check_box;
        } else {
            if total_shrink[FILLL as usize] != 0 {
                o = (FILLL) as u8;
            } else if total_shrink[FILL as usize] != 0 {
                o = (FILL) as u8;
            } else if total_shrink[FIL as usize] != 0 {
                o = (FIL) as u8;
            } else {
                o = 0;
            }
            mem!(r + 5).b16.s0 = (o as i16) as u16;
            mem!(r + 5).b16.s1 = (SHRINKING as i16) as u16;
            if total_shrink[o as usize] != 0 {
                BOX_glue_set!(r) = ((-x) as f64) / (total_shrink[o as usize] as f64);
            } else {
                mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
                BOX_glue_set!(r) = 0.0;
            }
            if total_shrink[o as usize] < -x
                && o == (NORMAL) as u8
                && mem!(r + 5).b32.s1 != TEX_NULL
            {
                last_badness = 1000000;
                BOX_glue_set!(r) = 1.0;
                if -x - total_shrink[NORMAL as usize] > DIMENPAR!(DIMEN_PAR__hfuzz)
                    || INTPAR!(INT_PAR__hbadness) < 100
                {
                    if DIMENPAR!(DIMEN_PAR__overfull_rule) > 0
                        && -x - total_shrink[NORMAL as usize] > DIMENPAR!(DIMEN_PAR__hfuzz)
                    {
                        while mem!(q).b32.s1 != TEX_NULL {
                            q = LLIST_link!(q);
                        }
                        mem!(q).b32.s1 = new_rule();
                        mem!(mem!(q).b32.s1 + 1).b32.s1 = DIMENPAR!(DIMEN_PAR__overfull_rule);
                    }
                    print_ln();
                    diagnostic_begin_capture_warning_here();
                    print_nl_cstr(b"Overfull \\hbox (\0".as_ptr() as *const c_char);
                    print_scaled(-x - total_shrink[NORMAL as usize]);
                    print_cstr(b"pt too wide\0".as_ptr() as *const c_char);
                    do_common_ending(r);
                }
            } else if o == (NORMAL) as u8 {
                if mem!(r + 5).b32.s1 != TEX_NULL {
                    last_badness = badness(-x, total_shrink[NORMAL as usize]);
                    if last_badness > INTPAR!(INT_PAR__hbadness) {
                        print_ln();
                        diagnostic_begin_capture_warning_here();
                        print_nl_cstr(b"Tight \\hbox (badness \0".as_ptr() as *const c_char);
                        print_int(last_badness);
                        do_common_ending(r);
                    }
                }
            }
            break 'check_box;
        }
    }

    if INTPAR!(INT_PAR__texxet) > 0 {
        if mem!(LR_ptr).b32.s0 != BEFORE {
            while mem!(q).b32.s1 != TEX_NULL {
                q = LLIST_link!(q);
            }
            loop {
                temp_ptr = q;
                q = new_math(0, (mem!(LR_ptr).b32.s0) as i16);
                mem!(temp_ptr).b32.s1 = q;
                LR_problems = LR_problems + 10000;
                temp_ptr = LR_ptr;
                LR_ptr = mem!(temp_ptr).b32.s1;
                mem!(temp_ptr).b32.s1 = avail;
                avail = temp_ptr;
                if mem!(LR_ptr).b32.s0 == BEFORE {
                    break;
                }
            }
        }
        if LR_problems > 0 {
            print_ln();
            diagnostic_begin_capture_warning_here();
            print_nl_cstr(b"\\endL or \\endR problem (\0".as_ptr() as *const c_char);
            print_int(LR_problems / 10000);
            print_cstr(b" missing, \0".as_ptr() as *const c_char);
            print_int(LR_problems % 10000);
            print_cstr(b" extra\0".as_ptr() as *const c_char);
            LR_problems = 0;
            do_common_ending(r);
        }
        temp_ptr = LR_ptr;
        LR_ptr = mem!(temp_ptr).b32.s1;
        mem!(temp_ptr).b32.s1 = avail;
        avail = temp_ptr;
        if LR_ptr != TEX_NULL {
            confusion(b"LR1\0".as_ptr() as *const c_char);
        }
    }

    r
}

// 30. vpackage
#[unsafe(export_name = "vpackage")]
pub unsafe extern "C" fn vpackage(
    mut p: i32,
    mut h: scaled_t,
    m: small_number,
    l: scaled_t,
) -> i32 {
    let r: i32;
    let mut w: scaled_t;
    let mut d: scaled_t;
    let mut x: scaled_t;
    let mut s: scaled_t;
    let mut other_font: i32;
    let mut o: glue_ord;

    last_badness = 0;
    r = get_node(BOX_NODE_SIZE);
    NODE_type!(r) = (VLIST_NODE as i16) as u16;
    if INTPAR!(INT_PAR__xetex_upwards) > 0 {
        mem!(r).b16.s0 = 1;
    } else {
        mem!(r).b16.s0 = 0;
    }
    mem!(r + 4).b32.s1 = 0;
    mem!(r + 5).b32.s1 = p;
    w = 0;
    d = 0;
    x = 0;
    total_stretch[NORMAL as usize] = 0;
    total_shrink[NORMAL as usize] = 0;
    total_stretch[FIL as usize] = 0;
    total_shrink[FIL as usize] = 0;
    total_stretch[FILL as usize] = 0;
    total_shrink[FILL as usize] = 0;
    total_stretch[FILLL as usize] = 0;
    total_shrink[FILLL as usize] = 0;

    while p != TEX_NULL {
        if is_char_node(p) {
            confusion(b"vpack\0".as_ptr() as *const c_char);
        } else {
            match mem!(p).b16.s1 as i32 {
                0 | 1 | 2 | 13 => {
                    x = x + d + mem!(p + 3).b32.s1;
                    d = mem!(p + 2).b32.s1;
                    if NODE_type!(p) >= (RULE_NODE as i16) as u16 {
                        s = 0;
                    } else {
                        s = mem!(p + 4).b32.s1;
                    }
                    if mem!(p + 1).b32.s1 + s > w {
                        w = mem!(p + 1).b32.s1 + s;
                    }
                }
                8 => {
                    if mem!(p).b16.s0 == (PIC_NODE as i16) as u16
                        || mem!(p).b16.s0 == (PDF_NODE as i16) as u16
                    {
                        x = x + d + mem!(p + 3).b32.s1;
                        d = mem!(p + 2).b32.s1;
                        if mem!(p + 1).b32.s1 > w {
                            w = mem!(p + 1).b32.s1;
                        }
                    }
                }
                10 => {
                    x = x + d;
                    d = 0;
                    other_font = mem!(p + 1).b32.s0;
                    x = x + mem!(other_font + 1).b32.s1;
                    o = mem!(other_font).b16.s1 as glue_ord;
                    total_stretch[o as usize] =
                        total_stretch[o as usize] + mem!(other_font + 2).b32.s1;
                    o = mem!(other_font).b16.s0 as glue_ord;
                    total_shrink[o as usize] =
                        total_shrink[o as usize] + mem!(other_font + 3).b32.s1;
                    if mem!(p).b16.s0 >= (A_LEADERS as i16) as u16 {
                        other_font = mem!(p + 1).b32.s1;
                        if mem!(other_font + 1).b32.s1 > w {
                            w = mem!(other_font + 1).b32.s1;
                        }
                    }
                }
                11 => {
                    x = x + d + mem!(p + 1).b32.s1;
                    d = 0;
                }
                _ => {}
            }
        }
        p = LLIST_link!(p);
    }

    mem!(r + 1).b32.s1 = w;
    if d > l {
        x = x + d - l;
        mem!(r + 2).b32.s1 = l;
    } else {
        mem!(r + 2).b32.s1 = d;
    }
    if m as i32 == ADDITIONAL {
        h = x + h;
    }
    mem!(r + 3).b32.s1 = h;
    x = h - x;

    let do_common_ending = |r: i32| {
        if output_active {
            print_cstr(b") has occurred while \\output is active\0".as_ptr() as *const c_char);
        } else {
            if pack_begin_line != 0 {
                print_cstr(b") in alignment at lines \0".as_ptr() as *const c_char);
                print_int(pack_begin_line.abs());
                print_cstr(b"--\0".as_ptr() as *const c_char);
            } else {
                print_cstr(b") detected at line \0".as_ptr() as *const c_char);
            }
            print_int(line);
            print_ln();
        }

        capture_to_diagnostic(std::ptr::null_mut());

        begin_diagnostic();
        show_box(r);
        end_diagnostic(true);
    };

    if x == 0 {
        mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
        mem!(r + 5).b16.s0 = (NORMAL as i16) as u16;
        BOX_glue_set!(r) = 0.0;
        return r;
    } else if x > 0 {
        if total_stretch[FILLL as usize] != 0 {
            o = (FILLL) as u8;
        } else if total_stretch[FILL as usize] != 0 {
            o = (FILL) as u8;
        } else if total_stretch[FIL as usize] != 0 {
            o = (FIL) as u8;
        } else {
            o = 0;
        }
        mem!(r + 5).b16.s0 = (o as i16) as u16;
        mem!(r + 5).b16.s1 = (STRETCHING as i16) as u16;
        if total_stretch[o as usize] != 0 {
            BOX_glue_set!(r) = (x as f64) / (total_stretch[o as usize] as f64);
        } else {
            mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
            BOX_glue_set!(r) = 0.0;
        }
        if o == (NORMAL) as u8 {
            if mem!(r + 5).b32.s1 != TEX_NULL {
                last_badness = badness(x, total_stretch[NORMAL as usize]);
                if last_badness > INTPAR!(INT_PAR__vbadness) {
                    print_ln();
                    diagnostic_begin_capture_warning_here();
                    if last_badness > 100 {
                        print_nl_cstr(b"Underfull\0".as_ptr() as *const c_char);
                    } else {
                        print_nl_cstr(b"Loose\0".as_ptr() as *const c_char);
                    }
                    print_cstr(b" \\vbox (badness \0".as_ptr() as *const c_char);
                    print_int(last_badness);
                    do_common_ending(r);
                }
            }
        }
        return r;
    } else {
        if total_shrink[FILLL as usize] != 0 {
            o = (FILLL) as u8;
        } else if total_shrink[FILL as usize] != 0 {
            o = (FILL) as u8;
        } else if total_shrink[FIL as usize] != 0 {
            o = (FIL) as u8;
        } else {
            o = 0;
        }
        mem!(r + 5).b16.s0 = (o as i16) as u16;
        mem!(r + 5).b16.s1 = (SHRINKING as i16) as u16;
        if total_shrink[o as usize] != 0 {
            BOX_glue_set!(r) = ((-x) as f64) / (total_shrink[o as usize] as f64);
        } else {
            mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
            BOX_glue_set!(r) = 0.0;
        }
        if total_shrink[o as usize] < -x && o == (NORMAL) as u8 && mem!(r + 5).b32.s1 != TEX_NULL {
            last_badness = 1000000;
            BOX_glue_set!(r) = 1.0;
            if -x - total_shrink[NORMAL as usize] > DIMENPAR!(DIMEN_PAR__vfuzz)
                || INTPAR!(INT_PAR__vbadness) < 100
            {
                print_ln();
                diagnostic_begin_capture_warning_here();
                print_nl_cstr(b"Overfull \\vbox (\0".as_ptr() as *const c_char);
                print_scaled(-x - total_shrink[NORMAL as usize]);
                print_cstr(b"pt too high\0".as_ptr() as *const c_char);
                do_common_ending(r);
            }
        } else if o == (NORMAL) as u8 {
            if mem!(r + 5).b32.s1 != TEX_NULL {
                last_badness = badness(-x, total_shrink[NORMAL as usize]);
                if last_badness > INTPAR!(INT_PAR__vbadness) {
                    print_ln();
                    diagnostic_begin_capture_warning_here();
                    print_nl_cstr(b"Tight \\vbox (badness \0".as_ptr() as *const c_char);
                    print_int(last_badness);
                    do_common_ending(r);
                }
            }
        }
        return r;
    }
}

macro_rules! save_stack {
    ($p:expr) => {
        *save_stack.offset(($p) as isize)
    };
}
macro_rules! nest {
    ($p:expr) => {
        *nest.offset(($p) as isize)
    };
}

macro_rules! font_glue {
    ($p:expr) => {
        *font_glue.offset(($p) as isize)
    };
}

macro_rules! param_base {
    ($p:expr) => {
        *param_base.offset(($p) as isize)
    };
}

macro_rules! font_info {
    ($p:expr) => {
        *font_info.offset(($p) as isize)
    };
}

macro_rules! BOX_REG {
    ($n:expr) => {
        eqtb!(BOX_BASE + ($n) as i32).b32.s1
    };
}

macro_rules! LOCAL {
    (every_cr) => {
        eqtb!(LOCAL_BASE + LOCAL__every_cr).b32.s1
    };
}

// 31. append_to_vlist
#[unsafe(export_name = "append_to_vlist")]
pub unsafe extern "C" fn append_to_vlist(b: i32) {
    let d: scaled_t;
    let p: i32;
    let upwards: bool = INTPAR!(INT_PAR__xetex_upwards) > 0;
    if cur_list.aux.b32.s1 > IGNORE_DEPTH {
        if upwards {
            d = mem!(GLUEPAR!(GLUE_PAR__baseline_skip) + 1).b32.s1
                - cur_list.aux.b32.s1
                - mem!(b + 2).b32.s1;
        } else {
            d = mem!(GLUEPAR!(GLUE_PAR__baseline_skip) + 1).b32.s1
                - cur_list.aux.b32.s1
                - mem!(b + 3).b32.s1;
        }
        if d < DIMENPAR!(DIMEN_PAR__line_skip_limit) {
            p = new_param_glue(GLUE_PAR__line_skip as small_number);
        } else {
            p = new_skip_param(GLUE_PAR__baseline_skip as small_number);
            mem!(temp_ptr + 1).b32.s1 = d;
        }
        mem!(cur_list.tail).b32.s1 = p;
        cur_list.tail = p;
    }
    mem!(cur_list.tail).b32.s1 = b;
    cur_list.tail = b;
    if upwards {
        cur_list.aux.b32.s1 = mem!(b + 3).b32.s1;
    } else {
        cur_list.aux.b32.s1 = mem!(b + 2).b32.s1;
    }
}

// 32. new_noad
#[unsafe(export_name = "new_noad")]
pub unsafe extern "C" fn new_noad() -> i32 {
    let p: i32 = get_node(NOAD_SIZE as i32);
    mem!(p).b16.s1 = ORD_NOAD as u16;
    mem!(p).b16.s0 = NORMAL as u16;
    mem!(p + 1).b32 = empty;
    mem!(p + 3).b32 = empty;
    mem!(p + 2).b32 = empty;
    p
}

// 33. new_style
#[unsafe(export_name = "new_style")]
pub unsafe extern "C" fn new_style(s: small_number) -> i32 {
    let p: i32 = get_node(STYLE_NODE_SIZE as i32);
    NODE_type!(p) = STYLE_NODE as u16;
    mem!(p).b16.s0 = s as u16;
    mem!(p + 1).b32.s1 = 0;
    mem!(p + 2).b32.s1 = 0;
    p
}

// 34. new_choice
#[unsafe(export_name = "new_choice")]
pub unsafe extern "C" fn new_choice() -> i32 {
    let p: i32 = get_node(STYLE_NODE_SIZE as i32);
    NODE_type!(p) = CHOICE_NODE as u16;
    mem!(p).b16.s0 = 0;
    mem!(p + 1).b32.s0 = TEX_NULL;
    mem!(p + 1).b32.s1 = TEX_NULL;
    mem!(p + 2).b32.s0 = TEX_NULL;
    mem!(p + 2).b32.s1 = TEX_NULL;
    p
}

// 35. show_info
#[unsafe(export_name = "show_info")]
pub unsafe extern "C" fn show_info() {
    show_node_list(mem!(temp_ptr).b32.s0);
}

// 36. push_alignment
#[unsafe(export_name = "push_alignment")]
pub unsafe extern "C" fn push_alignment() {
    let p: i32 = get_node(ALIGN_STACK_NODE_SIZE as i32);
    mem!(p).b32.s1 = align_ptr;
    mem!(p).b32.s0 = cur_align;
    mem!(p + 1).b32.s0 = mem!(ALIGN_HEAD).b32.s1;
    mem!(p + 1).b32.s1 = cur_span;
    mem!(p + 2).b32.s1 = cur_loop;
    mem!(p + 3).b32.s1 = align_state;
    mem!(p + 4).b32.s0 = cur_head;
    mem!(p + 4).b32.s1 = cur_tail;
    mem!(p + 5).b32.s0 = cur_pre_head;
    mem!(p + 5).b32.s1 = cur_pre_tail;
    align_ptr = p;
    cur_head = get_avail();
    cur_pre_head = get_avail();
}

// 37. pop_alignment
#[unsafe(export_name = "pop_alignment")]
pub unsafe extern "C" fn pop_alignment() {
    mem!(cur_head).b32.s1 = avail;
    avail = cur_head;
    mem!(cur_pre_head).b32.s1 = avail;
    avail = cur_pre_head;
    let p: i32 = align_ptr;
    cur_tail = mem!(p + 4).b32.s1;
    cur_head = mem!(p + 4).b32.s0;
    cur_pre_tail = mem!(p + 5).b32.s1;
    cur_pre_head = mem!(p + 5).b32.s0;
    align_state = mem!(p + 3).b32.s1;
    cur_loop = mem!(p + 2).b32.s1;
    cur_span = mem!(p + 1).b32.s1;
    mem!(ALIGN_HEAD).b32.s1 = mem!(p + 1).b32.s0;
    cur_align = mem!(p).b32.s0;
    align_ptr = mem!(p).b32.s1;
    free_node(p, ALIGN_STACK_NODE_SIZE as i32);
}

// 38. get_preamble_token
#[unsafe(export_name = "get_preamble_token")]
pub unsafe extern "C" fn get_preamble_token() {
    loop {
        get_token();
        while cur_chr == SPAN_CODE as i32 && (cur_cmd as i32) == TAB_MARK {
            get_token();
            if cur_cmd as i32 > MAX_COMMAND {
                expand();
                get_token();
            }
        }
        if (cur_cmd as i32) == ENDV {
            fatal_error(
                b"(interwoven alignment preambles are not allowed)\0".as_ptr() as *const c_char,
            );
        }
        if (cur_cmd as i32) == ASSIGN_GLUE && cur_chr == (GLUE_BASE + 11) {
            scan_optional_equals();
            scan_glue(GLUE_VAL as u8);
            if INTPAR!(INT_PAR__global_defs) > 0 {
                geq_define(GLUE_BASE + 11, GLUE_REF as u16, cur_val);
            } else {
                eq_define(GLUE_BASE + 11, GLUE_REF as u16, cur_val);
            }
            continue;
        }
        break;
    }
}

// 39. init_align
#[unsafe(export_name = "init_align")]
pub unsafe extern "C" fn init_align() {
    let save_cs_ptr: i32 = cur_cs;
    push_alignment();
    align_state = -1000000;

    if cur_list.mode as i32 == MMODE
        && (cur_list.tail != cur_list.head || cur_list.aux.b32.s1 != TEX_NULL)
    {
        error_here_with_diagnostic(b"Improper \0".as_ptr() as *const c_char);
        print_esc_cstr(b"halign\0".as_ptr() as *const c_char);
        print_cstr(b" inside $$'s\0".as_ptr() as *const c_char);
        capture_to_diagnostic(std::ptr::null_mut());
        help_ptr = 3;
        set_help_line!(
            2,
            b"Displays can use special alignments (like \\eqalignno)\0".as_ptr() as *const c_char
        );
        set_help_line!(
            1,
            b"only if nothing but the alignment itself is between $$'s.\0".as_ptr()
                as *const c_char
        );
        set_help_line!(
            0,
            b"So I've deleted the formulas that preceded this alignment.\0".as_ptr()
                as *const c_char
        );
        error();
        flush_math();
    }

    push_nest();

    if cur_list.mode as i32 == MMODE {
        cur_list.mode = -1;
        cur_list.aux.b32.s1 = nest!(nest_ptr - 2).aux.b32.s1;
    } else if cur_list.mode > 0 {
        cur_list.mode = -cur_list.mode;
    }

    scan_spec(ALIGN_GROUP as group_code, false);
    mem!(ALIGN_HEAD).b32.s1 = TEX_NULL;
    cur_align = ALIGN_HEAD;
    cur_loop = TEX_NULL;
    scanner_status = ALIGNING as u8;
    warning_index = save_cs_ptr;
    align_state = -1000000;

    loop {
        mem!(cur_align).b32.s1 = new_param_glue(GLUE_PAR__tab_skip as small_number);
        cur_align = LLIST_link!(cur_align);
        if (cur_cmd as i32) == CAR_RET {
            break;
        }

        let mut p: i32 = HOLD_HEAD;
        mem!(p).b32.s1 = TEX_NULL;

        loop {
            get_preamble_token();
            if (cur_cmd as i32) == MAC_PARAM {
                break;
            }

            if (cur_cmd as i32) <= CAR_RET
                && (cur_cmd as i32) >= TAB_MARK
                && align_state == -1000000
            {
                if p == HOLD_HEAD && cur_loop == TEX_NULL && (cur_cmd as i32) == TAB_MARK {
                    cur_loop = cur_align;
                } else {
                    error_here_with_diagnostic(
                        b"Missing # inserted in alignment preamble\0".as_ptr() as *const c_char,
                    );
                    capture_to_diagnostic(std::ptr::null_mut());
                    help_ptr = 3;
                    set_help_line!(
                        2,
                        b"There should be exactly one # between &'s, when an\0".as_ptr()
                            as *const c_char
                    );
                    set_help_line!(
                        1,
                        b"\\halign or \\valign is being set up. In this case you had\0".as_ptr()
                            as *const c_char
                    );
                    set_help_line!(
                        0,
                        b"none, so I've put one in; maybe that will work.\0".as_ptr()
                            as *const c_char
                    );
                    back_error();
                    break;
                }
            } else if (cur_cmd as i32) != SPACER || p != HOLD_HEAD {
                mem!(p).b32.s1 = get_avail();
                p = LLIST_link!(p);
                mem!(p).b32.s0 = cur_tok;
            }
        }

        mem!(cur_align).b32.s1 = new_null_box();
        cur_align = LLIST_link!(cur_align);
        mem!(cur_align).b32.s0 = END_SPAN;
        mem!(cur_align + 1).b32.s1 = NULL_FLAG;
        mem!(cur_align + 3).b32.s1 = mem!(HOLD_HEAD).b32.s1;
        p = HOLD_HEAD;
        mem!(p).b32.s1 = TEX_NULL;

        loop {
            get_preamble_token();
            if (cur_cmd as i32) <= CAR_RET
                && (cur_cmd as i32) >= TAB_MARK
                && align_state == -1000000
            {
                break;
            }

            if (cur_cmd as i32) == MAC_PARAM {
                error_here_with_diagnostic(
                    b"Only one # is allowed per tab\0".as_ptr() as *const c_char
                );
                capture_to_diagnostic(std::ptr::null_mut());
                help_ptr = 3;
                set_help_line!(
                    2,
                    b"There should be exactly one # between &'s, when an\0".as_ptr()
                        as *const c_char
                );
                set_help_line!(
                    1,
                    b"\\halign or \\valign is being set up. In this case you had\0".as_ptr()
                        as *const c_char
                );
                set_help_line!(
                    0,
                    b"more than one, so I'm ignoring all but the first.\0".as_ptr()
                        as *const c_char
                );
                error();
                continue;
            }

            mem!(p).b32.s1 = get_avail();
            p = LLIST_link!(p);
            mem!(p).b32.s0 = cur_tok;
        }

        mem!(p).b32.s1 = get_avail();
        p = LLIST_link!(p);
        mem!(p).b32.s0 = CS_TOKEN_FLAG + FROZEN_END_TEMPLATE;
        mem!(cur_align + 2).b32.s1 = mem!(HOLD_HEAD).b32.s1;
    }

    scanner_status = NORMAL as u8;
    new_save_level(ALIGN_GROUP as group_code);

    if LOCAL!(every_cr) != TEX_NULL {
        begin_token_list(LOCAL!(every_cr), EVERY_CR_TEXT as u16);
    }

    align_peek();
}

// 40. init_span
#[unsafe(export_name = "init_span")]
pub unsafe extern "C" fn init_span(p: i32) {
    push_nest();
    if cur_list.mode == -104 {
        cur_list.aux.b32.s0 = 1000;
    } else {
        cur_list.aux.b32.s1 = IGNORE_DEPTH;
        normal_paragraph();
    }
    cur_span = p;
}

// 41. init_row
#[unsafe(export_name = "init_row")]
pub unsafe extern "C" fn init_row() {
    push_nest();
    cur_list.mode = (-105) - cur_list.mode;
    if cur_list.mode == -104 {
        cur_list.aux.b32.s0 = 0;
    } else {
        cur_list.aux.b32.s1 = 0;
    }
    mem!(cur_list.tail).b32.s1 = new_glue(mem!(mem!(ALIGN_HEAD).b32.s1 + 1).b32.s0);
    cur_list.tail = LLIST_link!(cur_list.tail);
    mem!(cur_list.tail).b16.s0 = (GLUE_PAR__tab_skip + 1) as u16;
    cur_align = mem!(mem!(ALIGN_HEAD).b32.s1).b32.s1;
    cur_tail = cur_head;
    cur_pre_tail = cur_pre_head;
    init_span(cur_align);
}

// 42. init_col
#[unsafe(export_name = "init_col")]
pub unsafe extern "C" fn init_col() {
    mem!(cur_align + 5).b32.s0 = cur_cmd as i32;
    if (cur_cmd as i32) == OMIT {
        align_state = 0;
    } else {
        back_input();
        begin_token_list(mem!(cur_align + 3).b32.s1, U_TEMPLATE as u16);
    }
}

// 43. fin_col
#[unsafe(export_name = "fin_col")]
pub unsafe extern "C" fn fin_col() -> bool {
    let mut p: i32;
    let mut q: i32;
    let mut r: i32;
    let s: i32;
    let u: i32;
    let w: scaled_t;
    let mut o: glue_ord;
    let mut n: i32;

    if cur_align == TEX_NULL {
        confusion(b"endv\0".as_ptr() as *const c_char);
    }
    q = mem!(cur_align).b32.s1;
    if q == TEX_NULL {
        confusion(b"endv\0".as_ptr() as *const c_char);
    }
    if align_state < 500000 {
        fatal_error(
            b"(interwoven alignment preambles are not allowed)\0".as_ptr() as *const c_char,
        );
    }
    p = mem!(q).b32.s1;
    if p == TEX_NULL && mem!(cur_align + 5).b32.s0 < CR_CODE {
        if cur_loop != TEX_NULL {
            mem!(q).b32.s1 = new_null_box();
            p = mem!(q).b32.s1;
            mem!(p).b32.s0 = END_SPAN;
            mem!(p + 1).b32.s1 = NULL_FLAG;
            cur_loop = LLIST_link!(cur_loop);
            q = HOLD_HEAD;
            r = mem!(cur_loop + 3).b32.s1;
            while r != TEX_NULL {
                mem!(q).b32.s1 = get_avail();
                q = LLIST_link!(q);
                mem!(q).b32.s0 = mem!(r).b32.s0;
                r = LLIST_link!(r);
            }
            mem!(q).b32.s1 = TEX_NULL;
            mem!(p + 3).b32.s1 = mem!(HOLD_HEAD).b32.s1;
            q = HOLD_HEAD;
            r = mem!(cur_loop + 2).b32.s1;
            while r != TEX_NULL {
                mem!(q).b32.s1 = get_avail();
                q = LLIST_link!(q);
                mem!(q).b32.s0 = mem!(r).b32.s0;
                r = LLIST_link!(r);
            }
            mem!(q).b32.s1 = TEX_NULL;
            mem!(p + 2).b32.s1 = mem!(HOLD_HEAD).b32.s1;
            cur_loop = LLIST_link!(cur_loop);
            mem!(p).b32.s1 = new_glue(mem!(cur_loop + 1).b32.s0);
            NODE_subtype!(LLIST_link!(p)) = (GLUE_PAR__tab_skip + 1) as u16;
        } else {
            error_here_with_diagnostic(
                b"Extra alignment tab has been changed to \0".as_ptr() as *const c_char
            );
            print_esc_cstr(b"cr\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 3;
            set_help_line!(
                2,
                b"You have given more \\span or & marks than there were\0".as_ptr()
                    as *const c_char
            );
            set_help_line!(
                1,
                b"in the preamble to the \\halign or \\valign now in progress.\0".as_ptr()
                    as *const c_char
            );
            set_help_line!(
                0,
                b"So I'll assume that you meant to type \\cr instead.\0".as_ptr() as *const c_char
            );
            mem!(cur_align + 5).b32.s0 = CR_CODE;
            error();
        }
    }
    if mem!(cur_align + 5).b32.s0 != SPAN_CODE {
        unsave();
        new_save_level(ALIGN_GROUP as group_code);
        if cur_list.mode == -104 {
            adjust_tail = cur_tail;
            pre_adjust_tail = cur_pre_tail;
            u = hpack(mem!(cur_list.head).b32.s1, 0, ADDITIONAL as small_number);
            w = mem!(u + 1).b32.s1;
            cur_tail = adjust_tail;
            adjust_tail = TEX_NULL;
            cur_pre_tail = pre_adjust_tail;
            pre_adjust_tail = TEX_NULL;
        } else {
            u = vpackage(mem!(cur_list.head).b32.s1, 0, ADDITIONAL as small_number, 0);
            w = mem!(u + 3).b32.s1;
        }
        n = 0;
        if cur_span != cur_align {
            q = cur_span;
            loop {
                n += 1;
                q = mem!(mem!(q).b32.s1).b32.s1;
                if q == cur_align {
                    break;
                }
            }
            if n > 65535 {
                confusion(b"too many spans\0".as_ptr() as *const c_char);
            }
            q = cur_span;
            while mem!(mem!(q).b32.s0).b32.s1 < n {
                q = mem!(q).b32.s0;
            }
            if mem!(mem!(q).b32.s0).b32.s1 > n {
                let s_node: i32 = get_node(SPAN_NODE_SIZE as i32);
                mem!(s_node).b32.s0 = mem!(q).b32.s0;
                mem!(s_node).b32.s1 = n;
                mem!(q).b32.s0 = s_node;
                mem!(s_node + 1).b32.s1 = w;
            } else if mem!(mem!(q).b32.s0 + 1).b32.s1 < w {
                mem!(mem!(q).b32.s0 + 1).b32.s1 = w;
            }
        } else if w > mem!(cur_align + 1).b32.s1 {
            mem!(cur_align + 1).b32.s1 = w;
        }
        NODE_type!(u) = UNSET_NODE as u16;
        mem!(u).b16.s0 = n as u16;
        if total_stretch[FILLL as usize] != 0 {
            o = FILLL as glue_ord;
        } else if total_stretch[FILL as usize] != 0 {
            o = FILL as glue_ord;
        } else if total_stretch[FIL as usize] != 0 {
            o = FIL as glue_ord;
        } else {
            o = 0;
        }
        mem!(u + 5).b16.s0 = o as u16;
        mem!(u + 6).b32.s1 = total_stretch[o as usize];
        if total_shrink[FILLL as usize] != 0 {
            o = FILLL as glue_ord;
        } else if total_shrink[FILL as usize] != 0 {
            o = FILL as glue_ord;
        } else if total_shrink[FIL as usize] != 0 {
            o = FIL as glue_ord;
        } else {
            o = 0;
        }
        mem!(u + 5).b16.s1 = o as u16;
        mem!(u + 4).b32.s1 = total_shrink[o as usize];
        pop_nest();
        mem!(cur_list.tail).b32.s1 = u;
        cur_list.tail = u;

        mem!(cur_list.tail).b32.s1 = new_glue(mem!(mem!(cur_align).b32.s1 + 1).b32.s0);
        cur_list.tail = LLIST_link!(cur_list.tail);
        mem!(cur_list.tail).b16.s0 = (GLUE_PAR__tab_skip + 1) as u16;
        if mem!(cur_align + 5).b32.s0 >= CR_CODE {
            return true;
        }
        init_span(p);
    }
    align_state = 1000000;
    loop {
        get_x_or_protected();
        if (cur_cmd as i32) != SPACER {
            break;
        }
    }
    cur_align = p;
    init_col();
    false
}

// 44. fin_row
#[unsafe(export_name = "fin_row")]
pub unsafe extern "C" fn fin_row() {
    let p: i32;
    if cur_list.mode == -104 {
        p = hpack(mem!(cur_list.head).b32.s1, 0, ADDITIONAL as small_number);
        pop_nest();
        if cur_pre_head != cur_pre_tail {
            mem!(cur_list.tail).b32.s1 = mem!(cur_pre_head).b32.s1;
            cur_list.tail = cur_pre_tail;
        }
        append_to_vlist(p);
        if cur_head != cur_tail {
            mem!(cur_list.tail).b32.s1 = mem!(cur_head).b32.s1;
            cur_list.tail = cur_tail;
        }
    } else {
        p = vpackage(
            mem!(cur_list.head).b32.s1,
            0,
            ADDITIONAL as small_number,
            MAX_HALFWORD,
        );
        pop_nest();
        mem!(cur_list.tail).b32.s1 = p;
        cur_list.tail = p;
        cur_list.aux.b32.s0 = 1000;
    }
    NODE_type!(p) = UNSET_NODE as u16;
    mem!(p + 6).b32.s1 = 0;
    if LOCAL!(every_cr) != TEX_NULL {
        begin_token_list(LOCAL!(every_cr), EVERY_CR_TEXT as u16);
    }
    align_peek();
}

// 45. fin_align
#[unsafe(export_name = "fin_align")]
pub unsafe extern "C" fn fin_align() {
    let mut p: i32;
    let mut q: i32;
    let mut r: i32;
    let mut s: i32;
    let mut u: i32;
    let mut v: i32;
    let mut t: scaled_t;
    let mut w: scaled_t;
    let mut o: scaled_t;
    let mut n: i32;
    let rule_save: scaled_t;
    let aux_save: memory_word;

    if (cur_group as i32) != ALIGN_GROUP {
        confusion(b"align1\0".as_ptr() as *const c_char);
    }
    unsave();
    if (cur_group as i32) != ALIGN_GROUP {
        confusion(b"align0\0".as_ptr() as *const c_char);
    }
    unsave();
    if nest!(nest_ptr - 1).mode as i32 == MMODE {
        o = DIMENPAR!(DIMEN_PAR__display_indent);
    } else {
        o = 0;
    }
    q = mem!(mem!(ALIGN_HEAD).b32.s1).b32.s1;
    loop {
        flush_list(mem!(q + 3).b32.s1);
        flush_list(mem!(q + 2).b32.s1);
        p = mem!(mem!(q).b32.s1).b32.s1;
        if mem!(q + 1).b32.s1 == NULL_FLAG {
            mem!(q + 1).b32.s1 = 0;
            r = mem!(q).b32.s1;
            s = mem!(r + 1).b32.s0;
            if s != 0 {
                GLUE_SPEC_ref_count!(0) += 1;
                delete_glue_ref(s);
                mem!(r + 1).b32.s0 = 0;
            }
        }
        if mem!(q).b32.s0 != END_SPAN {
            t = mem!(q + 1).b32.s1 + mem!(mem!(mem!(q).b32.s1 + 1).b32.s0 + 1).b32.s1;
            r = mem!(q).b32.s0;
            s = END_SPAN;
            mem!(s).b32.s0 = p;
            n = 1;
            loop {
                mem!(r + 1).b32.s1 = mem!(r + 1).b32.s1 - t;
                u = mem!(r).b32.s0;
                while mem!(r).b32.s1 > n {
                    s = mem!(s).b32.s0;
                    n = mem!(mem!(s).b32.s0).b32.s1 + 1;
                }
                if mem!(r).b32.s1 < n {
                    mem!(r).b32.s0 = mem!(s).b32.s0;
                    mem!(s).b32.s0 = r;
                    mem!(r).b32.s1 -= 1;
                    s = r;
                } else {
                    if mem!(r + 1).b32.s1 > mem!(mem!(s).b32.s0 + 1).b32.s1 {
                        mem!(mem!(s).b32.s0 + 1).b32.s1 = mem!(r + 1).b32.s1;
                    }
                    free_node(r, SPAN_NODE_SIZE as i32);
                }
                r = u;
                if r == END_SPAN {
                    break;
                }
            }
        }
        NODE_type!(q) = UNSET_NODE as u16;
        mem!(q).b16.s0 = 0;
        mem!(q + 3).b32.s1 = 0;
        mem!(q + 2).b32.s1 = 0;
        mem!(q + 5).b16.s0 = NORMAL as u16;
        mem!(q + 5).b16.s1 = NORMAL as u16;
        mem!(q + 6).b32.s1 = 0;
        mem!(q + 4).b32.s1 = 0;
        q = p;
        if q == TEX_NULL {
            break;
        }
    }
    save_ptr -= 2;
    pack_begin_line = -cur_list.mode_line;
    if cur_list.mode == -1 {
        rule_save = DIMENPAR!(DIMEN_PAR__overfull_rule);
        DIMENPAR!(DIMEN_PAR__overfull_rule) = 0;
        p = hpack(
            mem!(ALIGN_HEAD).b32.s1,
            save_stack!(save_ptr + 1).b32.s1,
            save_stack!(save_ptr + 0).b32.s1 as small_number,
        );
        DIMENPAR!(DIMEN_PAR__overfull_rule) = rule_save;
    } else {
        q = mem!(mem!(ALIGN_HEAD).b32.s1).b32.s1;
        loop {
            mem!(q + 3).b32.s1 = mem!(q + 1).b32.s1;
            mem!(q + 1).b32.s1 = 0;
            q = mem!(mem!(q).b32.s1).b32.s1;
            if q == TEX_NULL {
                break;
            }
        }
        p = vpackage(
            mem!(ALIGN_HEAD).b32.s1,
            save_stack!(save_ptr + 1).b32.s1,
            save_stack!(save_ptr + 0).b32.s1 as small_number,
            MAX_HALFWORD,
        );
        q = mem!(mem!(ALIGN_HEAD).b32.s1).b32.s1;
        loop {
            mem!(q + 1).b32.s1 = mem!(q + 3).b32.s1;
            mem!(q + 3).b32.s1 = 0;
            q = mem!(mem!(q).b32.s1).b32.s1;
            if q == TEX_NULL {
                break;
            }
        }
    }
    pack_begin_line = 0;
    q = mem!(cur_list.head).b32.s1;
    s = cur_list.head;
    while q != TEX_NULL {
        if !is_char_node(q) {
            if (NODE_type!(q) as i32) == UNSET_NODE {
                if cur_list.mode == -1 {
                    NODE_type!(q) = HLIST_NODE as u16;
                    mem!(q + 1).b32.s1 = mem!(p + 1).b32.s1;
                    if nest!(nest_ptr - 1).mode as i32 == MMODE {
                        mem!(q).b16.s0 = DLIST as u16;
                    }
                } else {
                    NODE_type!(q) = VLIST_NODE as u16;
                    mem!(q + 3).b32.s1 = mem!(p + 3).b32.s1;
                }
                mem!(q + 5).b16.s0 = mem!(p + 5).b16.s0;
                mem!(q + 5).b16.s1 = mem!(p + 5).b16.s1;
                BOX_glue_set!(q) = BOX_glue_set!(p);
                mem!(q + 4).b32.s1 = o;
                r = mem!(mem!(q + 5).b32.s1).b32.s1;
                let mut p_s: i32 = mem!(mem!(p + 5).b32.s1).b32.s1;
                loop {
                    n = mem!(r).b16.s0 as i32;
                    t = mem!(p_s + 1).b32.s1;
                    w = t;
                    u = HOLD_HEAD;
                    mem!(r).b16.s0 = 0;
                    while n > 0 {
                        n -= 1;
                        p_s = LLIST_link!(p_s);
                        v = mem!(p_s + 1).b32.s0;
                        mem!(u).b32.s1 = new_glue(v);
                        u = LLIST_link!(u);
                        mem!(u).b16.s0 = (GLUE_PAR__tab_skip + 1) as u16;
                        t += mem!(v + 1).b32.s1;
                        if (mem!(p + 5).b16.s1 as i32) == STRETCHING {
                            if mem!(v).b16.s1 == mem!(p + 5).b16.s0 {
                                t += tex_round(BOX_glue_set!(p) * mem!(v + 2).b32.s1 as f64);
                            }
                        } else if (mem!(p + 5).b16.s1 as i32) == SHRINKING {
                            if mem!(v).b16.s0 == mem!(p + 5).b16.s0 {
                                t -= tex_round(BOX_glue_set!(p) * mem!(v + 3).b32.s1 as f64);
                            }
                        }
                        p_s = LLIST_link!(p_s);
                        mem!(u).b32.s1 = new_null_box();
                        u = LLIST_link!(u);
                        t += mem!(p_s + 1).b32.s1;
                        if cur_list.mode == -1 {
                            mem!(u + 1).b32.s1 = mem!(p_s + 1).b32.s1;
                        } else {
                            NODE_type!(u) = VLIST_NODE as u16;
                            mem!(u + 3).b32.s1 = mem!(p_s + 1).b32.s1;
                        }
                    }
                    if cur_list.mode == -1 {
                        mem!(r + 3).b32.s1 = mem!(q + 3).b32.s1;
                        mem!(r + 2).b32.s1 = mem!(q + 2).b32.s1;
                        if t == mem!(r + 1).b32.s1 {
                            mem!(r + 5).b16.s1 = NORMAL as u16;
                            mem!(r + 5).b16.s0 = NORMAL as u16;
                            BOX_glue_set!(r) = 0.0;
                        } else if t > mem!(r + 1).b32.s1 {
                            mem!(r + 5).b16.s1 = STRETCHING as u16;
                            if mem!(r + 6).b32.s1 == 0 {
                                BOX_glue_set!(r) = 0.0;
                            } else {
                                BOX_glue_set!(r) =
                                    (t - mem!(r + 1).b32.s1) as f64 / (mem!(r + 6).b32.s1 as f64);
                            }
                        } else {
                            mem!(r + 5).b16.s0 = mem!(r + 5).b16.s1;
                            mem!(r + 5).b16.s1 = SHRINKING as u16;
                            if mem!(r + 4).b32.s1 == 0 {
                                BOX_glue_set!(r) = 0.0;
                            } else if (mem!(r + 5).b16.s0 as i32) == NORMAL
                                && (mem!(r + 1).b32.s1 - t > mem!(r + 4).b32.s1)
                            {
                                BOX_glue_set!(r) = 1.0;
                            } else {
                                BOX_glue_set!(r) =
                                    (mem!(r + 1).b32.s1 - t) as f64 / (mem!(r + 4).b32.s1 as f64);
                            }
                        }
                        mem!(r + 1).b32.s1 = w;
                        NODE_type!(r) = HLIST_NODE as u16;
                    } else {
                        mem!(r + 1).b32.s1 = mem!(q + 1).b32.s1;
                        if t == mem!(r + 3).b32.s1 {
                            mem!(r + 5).b16.s1 = NORMAL as u16;
                            mem!(r + 5).b16.s0 = NORMAL as u16;
                            BOX_glue_set!(r) = 0.0;
                        } else if t > mem!(r + 3).b32.s1 {
                            mem!(r + 5).b16.s1 = STRETCHING as u16;
                            if mem!(r + 6).b32.s1 == 0 {
                                BOX_glue_set!(r) = 0.0;
                            } else {
                                BOX_glue_set!(r) =
                                    (t - mem!(r + 3).b32.s1) as f64 / (mem!(r + 6).b32.s1 as f64);
                            }
                        } else {
                            mem!(r + 5).b16.s0 = mem!(r + 5).b16.s1;
                            mem!(r + 5).b16.s1 = SHRINKING as u16;
                            if mem!(r + 4).b32.s1 == 0 {
                                BOX_glue_set!(r) = 0.0;
                            } else if (mem!(r + 5).b16.s0 as i32) == NORMAL
                                && (mem!(r + 3).b32.s1 - t > mem!(r + 4).b32.s1)
                            {
                                BOX_glue_set!(r) = 1.0;
                            } else {
                                BOX_glue_set!(r) =
                                    (mem!(r + 3).b32.s1 - t) as f64 / (mem!(r + 4).b32.s1 as f64);
                            }
                        }
                        mem!(r + 3).b32.s1 = w;
                        NODE_type!(r) = VLIST_NODE as u16;
                    }
                    mem!(r + 4).b32.s1 = 0;
                    if u != HOLD_HEAD {
                        mem!(u).b32.s1 = mem!(r).b32.s1;
                        mem!(r).b32.s1 = mem!(HOLD_HEAD).b32.s1;
                        r = u;
                    }
                    r = mem!(mem!(r).b32.s1).b32.s1;
                    p_s = mem!(mem!(p_s).b32.s1).b32.s1;
                    if r == TEX_NULL {
                        break;
                    }
                }
            } else if (NODE_type!(q) as i32) == RULE_NODE {
                if mem!(q + 1).b32.s1 == NULL_FLAG {
                    mem!(q + 1).b32.s1 = mem!(p + 1).b32.s1;
                }
                if mem!(q + 3).b32.s1 == NULL_FLAG {
                    mem!(q + 3).b32.s1 = mem!(p + 3).b32.s1;
                }
                if mem!(q + 2).b32.s1 == NULL_FLAG {
                    mem!(q + 2).b32.s1 = mem!(p + 2).b32.s1;
                }
                if o != 0 {
                    r = mem!(q).b32.s1;
                    mem!(q).b32.s1 = TEX_NULL;
                    q = hpack(q, 0, ADDITIONAL as small_number);
                    mem!(q + 4).b32.s1 = o;
                    mem!(q).b32.s1 = r;
                    mem!(s).b32.s1 = q;
                }
            }
        }
        s = q;
        q = LLIST_link!(q);
    }
    flush_node_list(p);
    pop_alignment();
    aux_save = cur_list.aux;
    p = mem!(cur_list.head).b32.s1;
    q = cur_list.tail;
    pop_nest();
    if cur_list.mode as i32 == MMODE {
        do_assignments();
        if (cur_cmd as i32) != MATH_SHIFT {
            error_here_with_diagnostic(b"Missing $$ inserted\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 2;
            set_help_line!(
                1,
                b"Displays can use special alignments (like \\eqalignno)\0".as_ptr()
                    as *const c_char
            );
            set_help_line!(
                0,
                b"only if nothing but the alignment itself is between $$'s.\0".as_ptr()
                    as *const c_char
            );
            back_error();
        } else {
            get_x_token();
            if (cur_cmd as i32) != MATH_SHIFT {
                error_here_with_diagnostic(
                    b"Display math should end with $$\0".as_ptr() as *const c_char
                );
                capture_to_diagnostic(std::ptr::null_mut());
                help_ptr = 2;
                set_help_line!(
                    1,
                    b"The `$' that I just saw supposedly matches a previous `$$'.\0".as_ptr()
                        as *const c_char
                );
                set_help_line!(
                    0,
                    b"So I shall assume that you typed `$$' both times.\0".as_ptr()
                        as *const c_char
                );
                back_error();
            }
        }
        flush_node_list(cur_list.eTeX_aux);
        pop_nest();
        mem!(cur_list.tail).b32.s1 = new_penalty(INTPAR!(INT_PAR__pre_display_penalty));
        cur_list.tail = LLIST_link!(cur_list.tail);
        mem!(cur_list.tail).b32.s1 = new_param_glue(GLUE_PAR__above_display_skip as small_number);
        cur_list.tail = LLIST_link!(cur_list.tail);
        mem!(cur_list.tail).b32.s1 = p;
        if p != TEX_NULL {
            cur_list.tail = q;
        }
        mem!(cur_list.tail).b32.s1 = new_penalty(INTPAR!(INT_PAR__post_display_penalty));
        cur_list.tail = LLIST_link!(cur_list.tail);
        mem!(cur_list.tail).b32.s1 = new_param_glue(GLUE_PAR__below_display_skip as small_number);
        cur_list.tail = LLIST_link!(cur_list.tail);
        cur_list.aux.b32.s1 = aux_save.b32.s1;
        resume_after_display();
    } else {
        cur_list.aux = aux_save;
        mem!(cur_list.tail).b32.s1 = p;
        if p != TEX_NULL {
            cur_list.tail = q;
        }
        if cur_list.mode as i32 == VMODE {
            build_page();
        }
    }
}

// 46. align_peek
#[unsafe(export_name = "align_peek")]
pub unsafe extern "C" fn align_peek() {
    loop {
        align_state = 1000000;
        loop {
            get_x_or_protected();
            if (cur_cmd as i32) != SPACER {
                break;
            }
        }
        if (cur_cmd as i32) == NO_ALIGN {
            scan_left_brace();
            new_save_level(NO_ALIGN_GROUP as group_code);
            if cur_list.mode == -1 {
                normal_paragraph();
            }
            break;
        } else if (cur_cmd as i32) == RIGHT_BRACE {
            fin_align();
            break;
        } else if (cur_cmd as i32) == CAR_RET && cur_chr == CR_CR_CODE {
            continue;
        } else {
            init_row();
            init_col();
            break;
        }
    }
}

// 47. max_hyphenatable_length
#[unsafe(export_name = "max_hyphenatable_length")]
pub unsafe extern "C" fn max_hyphenatable_length() -> i32 {
    if INTPAR!(INT_PAR__xetex_hyphenatable_length) > HYPHENATABLE_LENGTH_LIMIT as i32 {
        return HYPHENATABLE_LENGTH_LIMIT as i32;
    }
    INTPAR!(INT_PAR__xetex_hyphenatable_length)
}

// 48. eTeX_enabled
#[unsafe(export_name = "eTeX_enabled")]
pub unsafe extern "C" fn eTeX_enabled(b: bool, j: u16, k: i32) -> bool {
    if !b {
        error_here_with_diagnostic(b"Improper \0".as_ptr() as *const c_char);
        print_cmd_chr(j, k);
        capture_to_diagnostic(std::ptr::null_mut());
        help_ptr = 1;
        set_help_line!(
            0,
            b"Sorry, this optional e-TeX feature has been disabled.\0".as_ptr() as *const c_char
        );
        error();
    }
    b
}

// 49. show_save_groups
#[unsafe(export_name = "show_save_groups")]
pub unsafe extern "C" fn show_save_groups() {
    let mut p: i32;
    let mut m: i16;
    let v: save_pointer;
    let l: u16;
    let c: group_code;
    let mut a: i8;
    let mut i: i32;
    let mut j: u16;
    let mut s: *const c_char = std::ptr::null();

    p = nest_ptr as i32;
    nest!(p) = cur_list;
    v = save_ptr;
    l = cur_level;
    c = cur_group;
    save_ptr = cur_boundary;
    cur_level -= 1;
    a = 1;

    print_nl_cstr(b"\0".as_ptr() as *const c_char);
    print_ln();

    loop {
        print_nl_cstr(b"### \0".as_ptr() as *const c_char);
        print_group(true);

        if cur_group as i32 == BOTTOM_LEVEL {
            break;
        }

        loop {
            m = nest!(p).mode;
            if p > 0 {
                p -= 1;
            } else {
                m = VMODE as i16;
            }
            if m != HMODE as i16 {
                break;
            }
        }

        print_cstr(b" (\0".as_ptr() as *const c_char);

        #[derive(PartialEq)]
        enum Target {
            Full,
            Found1,
            Found2,
            Found,
        }

        let mut target = Target::Full;

        match cur_group as i32 {
            SIMPLE_GROUP => {
                p += 1;
                target = Target::Found2;
            }
            HBOX_GROUP | ADJUSTED_HBOX_GROUP => {
                s = b"hbox\0".as_ptr() as *const c_char;
            }
            VBOX_GROUP => {
                s = b"vbox\0".as_ptr() as *const c_char;
            }
            VTOP_GROUP => {
                s = b"vtop\0".as_ptr() as *const c_char;
            }
            ALIGN_GROUP => {
                if a == 0 {
                    if m == -(VMODE as i16) {
                        s = b"halign\0".as_ptr() as *const c_char;
                    } else {
                        s = b"valign\0".as_ptr() as *const c_char;
                    }
                    a = 1;
                    target = Target::Found1;
                } else {
                    if a == 1 {
                        print_cstr(b"align entry\0".as_ptr() as *const c_char);
                    } else {
                        print_esc_cstr(b"cr\0".as_ptr() as *const c_char);
                    }
                    if p >= a as i32 {
                        p -= a as i32;
                    }
                    a = 0;
                    target = Target::Found;
                }
            }
            NO_ALIGN_GROUP => {
                p += 1;
                a = -1;
                print_esc_cstr(b"noalign\0".as_ptr() as *const c_char);
                target = Target::Found2;
            }
            OUTPUT_GROUP => {
                print_esc_cstr(b"output\0".as_ptr() as *const c_char);
                target = Target::Found;
            }
            MATH_GROUP => {
                target = Target::Found2;
            }
            DISC_GROUP | MATH_CHOICE_GROUP => {
                if cur_group as i32 == DISC_GROUP {
                    print_esc_cstr(b"discretionary\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"mathchoice\0".as_ptr() as *const c_char);
                }
                for idx in 1..=3 {
                    if idx <= save_stack!(save_ptr - 2).b32.s1 {
                        print_cstr(b"{}\0".as_ptr() as *const c_char);
                    }
                }
                target = Target::Found2;
            }
            INSERT_GROUP => {
                if save_stack!(save_ptr - 2).b32.s1 == 255 {
                    print_esc_cstr(b"vadjust\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"insert\0".as_ptr() as *const c_char);
                    print_int(save_stack!(save_ptr - 2).b32.s1);
                }
                target = Target::Found2;
            }
            VCENTER_GROUP => {
                s = b"vcenter\0".as_ptr() as *const c_char;
                target = Target::Found1;
            }
            SEMI_SIMPLE_GROUP => {
                p += 1;
                print_esc_cstr(b"begingroup\0".as_ptr() as *const c_char);
                target = Target::Found;
            }
            MATH_SHIFT_GROUP => {
                if m == MMODE as i16 {
                    print_char(b'$' as i32);
                } else if nest!(p).mode as i32 == MMODE {
                    print_cmd_chr(EQ_NO as u16, save_stack!(save_ptr - 2).b32.s1);
                    target = Target::Found;
                }
                if target != Target::Found {
                    print_char(b'$' as i32);
                    target = Target::Found;
                }
            }
            MATH_LEFT_GROUP => {
                if mem!(nest!(p + 1).eTeX_aux).b16.s1 == LEFT_NOAD as u16 {
                    print_esc_cstr(b"left\0".as_ptr() as *const c_char);
                } else {
                    print_esc_cstr(b"middle\0".as_ptr() as *const c_char);
                }
                target = Target::Found;
            }
            _ => {}
        }

        if target == Target::Full {
            i = save_stack!(save_ptr - 4).b32.s1;
            if i != 0 {
                if i < BOX_FLAG {
                    if nest!(p).mode.abs() as i32 == VMODE {
                        j = HMOVE as u16;
                    } else {
                        j = VMOVE as u16;
                    }
                    if i > 0 {
                        print_cmd_chr(j, 0);
                    } else {
                        print_cmd_chr(j, 1);
                    }
                    print_scaled(i.abs());
                    print_cstr(b"pt\0".as_ptr() as *const c_char);
                } else if i < SHIP_OUT_FLAG {
                    if i >= GLOBAL_BOX_FLAG {
                        print_esc_cstr(b"global\0".as_ptr() as *const c_char);
                        i -= GLOBAL_BOX_FLAG - BOX_FLAG;
                    }
                    print_esc_cstr(b"setbox\0".as_ptr() as *const c_char);
                    print_int(i - BOX_FLAG);
                    print_char(b'=' as i32);
                } else {
                    print_cmd_chr(LEADER_SHIP as u16, i - (LEADER_FLAG - A_LEADERS));
                }
            }
            target = Target::Found1;
        }

        if target == Target::Found1 {
            print_esc_cstr(s);
            if save_stack!(save_ptr - 2).b32.s1 != 0 {
                print_char(b' ' as i32);
                if save_stack!(save_ptr - 3).b32.s1 == EXACTLY as i32 {
                    print_cstr(b"to\0".as_ptr() as *const c_char);
                } else {
                    print_cstr(b"spread\0".as_ptr() as *const c_char);
                }
                print_scaled(save_stack!(save_ptr - 2).b32.s1);
                print_cstr(b"pt\0".as_ptr() as *const c_char);
            }
            target = Target::Found2;
        }

        if target == Target::Found2 {
            print_char(b'{' as i32);
            target = Target::Found;
        }

        if target == Target::Found {
            print_char(b')' as i32);
            cur_level -= 1;
            cur_group = save_stack!(save_ptr).b16.s0 as group_code;
            save_ptr = save_stack!(save_ptr).b32.s1;
        }
    }

    save_ptr = v;
    cur_level = l;
    cur_group = c;
}

// 50. vert_break
#[unsafe(export_name = "vert_break")]
pub unsafe extern "C" fn vert_break(mut p: i32, h: scaled_t, d: scaled_t) -> i32 {
    let mut prev_p: i32;
    let mut q: i32;
    let mut r: i32;
    let mut pi: i32 = 0;
    let mut b: i32;
    let mut least_cost: i32;
    let mut best_place: i32 = TEX_NULL;
    let mut prev_dp: scaled_t;

    prev_p = p;
    least_cost = MAX_HALFWORD;
    active_width[1] = 0;
    active_width[2] = 0;
    active_width[3] = 0;
    active_width[4] = 0;
    active_width[5] = 0;
    active_width[6] = 0;
    prev_dp = 0;

    loop {
        let mut do_check_pi = false;
        let mut do_update_heights = false;

        if p == TEX_NULL {
            pi = EJECT_PENALTY;
            do_check_pi = true;
        } else {
            match mem!(p).b16.s1 as i32 {
                0 | 1 | 2 => {
                    active_width[1] = active_width[1] + prev_dp + mem!(p + 3).b32.s1;
                    prev_dp = mem!(p + 2).b32.s1;
                }
                8 => {
                    if (mem!(p).b16.s0 as i32 == PIC_NODE) || (mem!(p).b16.s0 as i32 == PDF_NODE) {
                        active_width[1] = active_width[1] + prev_dp + mem!(p + 3).b32.s1;
                        prev_dp = mem!(p + 2).b32.s1;
                    }
                }
                10 => {
                    if is_non_discardable_node(prev_p) {
                        pi = 0;
                        do_check_pi = true;
                    } else {
                        do_update_heights = true;
                    }
                }
                11 => {
                    let t_val: small_number = if mem!(p).b32.s1 == TEX_NULL {
                        PENALTY_NODE as small_number
                    } else {
                        mem!(mem!(p).b32.s1).b16.s1 as small_number
                    };
                    if t_val as i32 == GLUE_NODE {
                        pi = 0;
                        do_check_pi = true;
                    } else {
                        do_update_heights = true;
                    }
                }
                12 => {
                    pi = mem!(p + 1).b32.s1;
                    do_check_pi = true;
                }
                4 | 3 => {}
                _ => {
                    confusion(b"vertbreak\0".as_ptr() as *const c_char);
                }
            }
        }

        if do_check_pi {
            if pi < INF_PENALTY {
                if active_width[1] < h {
                    if (active_width[3] != 0) || (active_width[4] != 0) || (active_width[5] != 0) {
                        b = 0;
                    } else {
                        b = badness(h - active_width[1], active_width[2]);
                    }
                } else if active_width[1] - h > active_width[6] {
                    b = MAX_HALFWORD;
                } else {
                    b = badness(active_width[1] - h, active_width[6]);
                }
                if b < MAX_HALFWORD {
                    if pi <= EJECT_PENALTY {
                        b = pi;
                    } else if b < INF_BAD {
                        b += pi;
                    } else {
                        b = 100000;
                    }
                }
                if b <= least_cost {
                    best_place = p;
                    least_cost = b;
                    best_height_plus_depth = active_width[1] + prev_dp;
                }
                if (b == MAX_HALFWORD) || (pi <= EJECT_PENALTY) {
                    break;
                }
            }
            if (NODE_type!(p) as i32 >= GLUE_NODE) && (NODE_type!(p) as i32 <= KERN_NODE) {
                do_update_heights = true;
            }
        }

        if do_update_heights {
            if (NODE_type!(p) as i32) == KERN_NODE {
                q = p;
            } else {
                q = mem!(p + 1).b32.s0;
                active_width[(2 + mem!(q).b16.s1) as usize] += mem!(q + 2).b32.s1;
                active_width[6] += mem!(q + 3).b32.s1;
                if ((mem!(q).b16.s0 as i32) != NORMAL) && (mem!(q + 3).b32.s1 != 0) {
                    error_here_with_diagnostic(
                        b"Infinite glue shrinkage found in box being split\0".as_ptr()
                            as *const c_char,
                    );
                    capture_to_diagnostic(std::ptr::null_mut());
                    help_ptr = 4;
                    set_help_line!(
                        3,
                        b"The box you are \\vsplitting contains some infinitely\0".as_ptr()
                            as *const c_char
                    );
                    set_help_line!(
                        2,
                        b"shrinkable glue, e.g., `\\vss' or `\\vskip 0pt minus 1fil'.\0".as_ptr()
                            as *const c_char
                    );
                    set_help_line!(
                        1,
                        b"Such glue doesn't belong there; but you can safely proceed,\0".as_ptr()
                            as *const c_char
                    );
                    set_help_line!(
                        0,
                        b"since the offensive shrinkability has been made finite.\0".as_ptr()
                            as *const c_char
                    );
                    error();
                    r = new_spec(q);
                    GLUE_SPEC_shrink_order!(r) = NORMAL as u16;
                    delete_glue_ref(q);
                    mem!(p + 1).b32.s0 = r;
                    q = r;
                }
            }
            active_width[1] = active_width[1] + prev_dp + mem!(q + 1).b32.s1;
            prev_dp = 0;
        }

        if prev_dp > d {
            active_width[1] = active_width[1] + prev_dp - d;
            prev_dp = d;
        }
        prev_p = p;
        p = mem!(prev_p).b32.s1;
    }

    best_place
}

// 51. vsplit
#[unsafe(export_name = "vsplit")]
pub unsafe extern "C" fn vsplit(n: i32, h: scaled_t) -> i32 {
    let mut v: i32;
    let mut p: i32;
    let mut q: i32;

    cur_val = n;
    if cur_val < 256 {
        v = BOX_REG!(cur_val);
    } else {
        find_sa_element(4, cur_val, false);
        if cur_ptr == TEX_NULL {
            v = TEX_NULL;
        } else {
            v = mem!(cur_ptr + 1).b32.s1;
        }
    }
    flush_node_list(disc_ptr[VSPLIT_CODE as usize]);
    disc_ptr[VSPLIT_CODE as usize] = TEX_NULL;
    if sa_root[MARK_VAL as usize] != TEX_NULL {
        if do_marks(0, 0, sa_root[MARK_VAL as usize]) {
            sa_root[MARK_VAL as usize] = TEX_NULL;
        }
    }
    if cur_mark[SPLIT_FIRST_MARK_CODE as usize] != TEX_NULL {
        delete_token_ref(cur_mark[SPLIT_FIRST_MARK_CODE as usize]);
        cur_mark[SPLIT_FIRST_MARK_CODE as usize] = TEX_NULL;
        delete_token_ref(cur_mark[SPLIT_BOT_MARK_CODE as usize]);
        cur_mark[SPLIT_BOT_MARK_CODE as usize] = TEX_NULL;
    }
    if v == TEX_NULL {
        return TEX_NULL;
    }
    if (NODE_type!(v) as i32) != VLIST_NODE {
        error_here_with_diagnostic(b"\0".as_ptr() as *const c_char);
        print_esc_cstr(b"vsplit\0".as_ptr() as *const c_char);
        print_cstr(b" needs a \0".as_ptr() as *const c_char);
        print_esc_cstr(b"vbox\0".as_ptr() as *const c_char);
        capture_to_diagnostic(std::ptr::null_mut());
        help_ptr = 2;
        set_help_line!(
            1,
            b"The box you are trying to split is an \\hbox.\0".as_ptr() as *const c_char
        );
        set_help_line!(
            0,
            b"I can't split such a box, so I'll leave it alone.\0".as_ptr() as *const c_char
        );
        error();
        return TEX_NULL;
    }
    q = vert_break(mem!(v + 5).b32.s1, h, DIMENPAR!(DIMEN_PAR__split_max_depth));
    p = mem!(v + 5).b32.s1;
    if p == q {
        mem!(v + 5).b32.s1 = TEX_NULL;
    } else {
        loop {
            if (NODE_type!(p) as i32) == MARK_NODE {
                if mem!(p + 1).b32.s0 != 0 {
                    find_sa_element(MARK_VAL as small_number, mem!(p + 1).b32.s0, true);
                    if mem!(cur_ptr + 2).b32.s1 == TEX_NULL {
                        mem!(cur_ptr + 2).b32.s1 = mem!(p + 1).b32.s1;
                        mem!(mem!(p + 1).b32.s1).b32.s0 += 1;
                    } else {
                        delete_token_ref(mem!(cur_ptr + 3).b32.s0);
                    }
                    mem!(cur_ptr + 3).b32.s0 = mem!(p + 1).b32.s1;
                    mem!(mem!(p + 1).b32.s1).b32.s0 += 1;
                } else if cur_mark[SPLIT_FIRST_MARK_CODE as usize] == TEX_NULL {
                    cur_mark[SPLIT_FIRST_MARK_CODE as usize] = mem!(p + 1).b32.s1;
                    cur_mark[SPLIT_BOT_MARK_CODE as usize] =
                        cur_mark[SPLIT_FIRST_MARK_CODE as usize];
                    mem!(cur_mark[SPLIT_FIRST_MARK_CODE as usize]).b32.s0 += 2;
                } else {
                    delete_token_ref(cur_mark[SPLIT_BOT_MARK_CODE as usize]);
                    cur_mark[SPLIT_BOT_MARK_CODE as usize] = mem!(p + 1).b32.s1;
                    mem!(cur_mark[SPLIT_BOT_MARK_CODE as usize]).b32.s0 += 1;
                }
            }
            if mem!(p).b32.s1 == q {
                mem!(p).b32.s1 = TEX_NULL;
                break;
            }
            p = LLIST_link!(p);
        }
    }
    q = prune_page_top(q, INTPAR!(INT_PAR__saving_vdiscards) > 0);
    p = mem!(v + 5).b32.s1;
    free_node(v, BOX_NODE_SIZE as i32);
    if q != TEX_NULL {
        q = vpackage(q, 0, ADDITIONAL as small_number, MAX_HALFWORD);
    }
    if cur_val < 256 {
        BOX_REG!(cur_val) = q;
    } else {
        find_sa_element(4, cur_val, false);
        if cur_ptr != TEX_NULL {
            mem!(cur_ptr + 1).b32.s1 = q;
            mem!(cur_ptr + 1).b32.s0 += 1;
            delete_sa_ref(cur_ptr);
        }
    }
    vpackage(
        p,
        h,
        EXACTLY as small_number,
        DIMENPAR!(DIMEN_PAR__split_max_depth),
    )
}

// 52. print_totals
#[unsafe(export_name = "print_totals")]
pub unsafe extern "C" fn print_totals() {
    print_scaled(page_so_far[1]);
    if page_so_far[2] != 0 {
        print_cstr(b" plus \0".as_ptr() as *const c_char);
        print_scaled(page_so_far[2]);
        print_cstr(b"\0".as_ptr() as *const c_char);
    }
    if page_so_far[3] != 0 {
        print_cstr(b" plus \0".as_ptr() as *const c_char);
        print_scaled(page_so_far[3]);
        print_cstr(b"fil\0".as_ptr() as *const c_char);
    }
    if page_so_far[4] != 0 {
        print_cstr(b" plus \0".as_ptr() as *const c_char);
        print_scaled(page_so_far[4]);
        print_cstr(b"fill\0".as_ptr() as *const c_char);
    }
    if page_so_far[5] != 0 {
        print_cstr(b" plus \0".as_ptr() as *const c_char);
        print_scaled(page_so_far[5]);
        print_cstr(b"filll\0".as_ptr() as *const c_char);
    }
    if page_so_far[6] != 0 {
        print_cstr(b" minus \0".as_ptr() as *const c_char);
        print_scaled(page_so_far[6]);
    }
}

// 53. box_error
#[unsafe(export_name = "box_error")]
pub unsafe extern "C" fn box_error(n: eight_bits) {
    error();
    begin_diagnostic();
    diagnostic_begin_capture_warning_here();
    print_nl_cstr(b"The following box has been deleted:\0".as_ptr() as *const c_char);
    show_box(BOX_REG!(n));
    capture_to_diagnostic(std::ptr::null_mut());
    end_diagnostic(true);
    flush_node_list(BOX_REG!(n));
    BOX_REG!(n) = TEX_NULL;
}

// 54. app_space
#[unsafe(export_name = "app_space")]
pub unsafe extern "C" fn app_space() {
    let q: i32;
    if cur_list.aux.b32.s0 >= 2000 && GLUEPAR!(GLUE_PAR__xspace_skip) != 0 {
        q = new_param_glue(GLUE_PAR__xspace_skip as small_number);
    } else {
        if GLUEPAR!(GLUE_PAR__space_skip) != 0 {
            main_p = GLUEPAR!(GLUE_PAR__space_skip);
        } else {
            main_p = font_glue!(eqtb!(CUR_FONT_LOC).b32.s1);
            if main_p == TEX_NULL {
                main_p = new_spec(0);
                main_k = param_base!(eqtb!(CUR_FONT_LOC).b32.s1) + 2;
                mem!(main_p + 1).b32.s1 = font_info!(main_k).b32.s1;
                mem!(main_p + 2).b32.s1 = font_info!(main_k + 1).b32.s1;
                mem!(main_p + 3).b32.s1 = font_info!(main_k + 2).b32.s1;
                font_glue!(eqtb!(CUR_FONT_LOC).b32.s1) = main_p;
            }
        }
        main_p = new_spec(main_p);
        if cur_list.aux.b32.s0 >= 2000 {
            mem!(main_p + 1).b32.s1 +=
                font_info!(EXTRA_SPACE_CODE as i32 + param_base!(eqtb!(CUR_FONT_LOC).b32.s1))
                    .b32
                    .s1;
        }
        mem!(main_p + 2).b32.s1 = xn_over_d(mem!(main_p + 2).b32.s1, cur_list.aux.b32.s0, 1000);
        mem!(main_p + 3).b32.s1 = xn_over_d(mem!(main_p + 3).b32.s1, 1000, cur_list.aux.b32.s0);
        q = new_glue(main_p);
        mem!(main_p).b32.s1 = TEX_NULL;
    }
    mem!(cur_list.tail).b32.s1 = q;
    cur_list.tail = q;
}

// 55. insert_dollar_sign
#[unsafe(export_name = "insert_dollar_sign")]
pub unsafe extern "C" fn insert_dollar_sign() {
    back_input();
    cur_tok = MATH_SHIFT_TOKEN + 36;
    error_here_with_diagnostic(b"Missing $ inserted\0".as_ptr() as *const c_char);
    capture_to_diagnostic(std::ptr::null_mut());
    help_ptr = 2;
    set_help_line!(
        1,
        b"I've inserted a begin-math/end-math symbol since I think\0".as_ptr() as *const c_char
    );
    set_help_line!(
        0,
        b"you left one out. Proceed, with fingers crossed.\0".as_ptr() as *const c_char
    );
    ins_error();
}

// 56. you_cant
#[unsafe(export_name = "you_cant")]
pub unsafe extern "C" fn you_cant() {
    error_here_with_diagnostic(b"You can't use `\0".as_ptr() as *const c_char);
    print_cmd_chr(cur_cmd as u16, cur_chr);
    print_in_mode(cur_list.mode as i32);
    capture_to_diagnostic(std::ptr::null_mut());
}

// 57. report_illegal_case
#[unsafe(export_name = "report_illegal_case")]
pub unsafe extern "C" fn report_illegal_case() {
    you_cant();
    help_ptr = 4;
    set_help_line!(
        3,
        b"Sorry, but I'm not programmed to handle this case;\0".as_ptr() as *const c_char
    );
    set_help_line!(
        2,
        b"I'll just pretend that you didn't ask for it.\0".as_ptr() as *const c_char
    );
    set_help_line!(
        1,
        b"If you're in the wrong mode, you might be able to\0".as_ptr() as *const c_char
    );
    set_help_line!(
        0,
        b"return to the right one by typing `I}' or `I$' or `I\\par'.\0".as_ptr() as *const c_char
    );
    error();
}

// 58. privileged
#[unsafe(export_name = "privileged")]
pub unsafe extern "C" fn privileged() -> bool {
    if cur_list.mode > 0 {
        true
    } else {
        report_illegal_case();
        false
    }
}

// 59. its_all_over
#[unsafe(export_name = "its_all_over")]
pub unsafe extern "C" fn its_all_over() -> bool {
    if privileged() {
        if PAGE_HEAD == page_tail && cur_list.head == cur_list.tail && dead_cycles == 0 {
            return true;
        }
        back_input();
        mem!(cur_list.tail).b32.s1 = new_null_box();
        cur_list.tail = LLIST_link!(cur_list.tail);
        mem!(cur_list.tail + 1).b32.s1 = DIMENPAR!(DIMEN_PAR__hsize);
        mem!(cur_list.tail).b32.s1 = new_glue(8);
        cur_list.tail = LLIST_link!(cur_list.tail);
        mem!(cur_list.tail).b32.s1 = new_penalty(NULL_FLAG);
        cur_list.tail = LLIST_link!(cur_list.tail);
        build_page();
    }
    false
}

// 60. append_glue
#[unsafe(export_name = "append_glue")]
pub unsafe extern "C" fn append_glue() {
    let s: small_number = cur_chr as small_number;
    match s as i32 {
        0 => {
            cur_val = 4;
        }
        1 => {
            cur_val = 8;
        }
        2 => {
            cur_val = 12;
        }
        3 => {
            cur_val = 16;
        }
        4 => {
            scan_glue(GLUE_VAL as u8);
        }
        5 => {
            scan_glue(MU_VAL as u8);
        }
        _ => {}
    }
    mem!(cur_list.tail).b32.s1 = new_glue(cur_val);
    cur_list.tail = LLIST_link!(cur_list.tail);
    if s as i32 >= SKIP_CODE {
        mem!(cur_val).b32.s1 -= 1;
        if s as i32 > SKIP_CODE {
            mem!(cur_list.tail).b16.s0 = MU_GLUE as u16;
        }
    }
}

macro_rules! BOX_list_ptr {
    ($p:expr) => {
        mem!(($p) + 5).b32.s1
    };
}

macro_rules! GLUE_NODE_glue_ptr {
    ($p:expr) => {
        mem!(($p) + 1).b32.s0
    };
}

macro_rules! GLUE_NODE_leader_ptr {
    ($p:expr) => {
        mem!(($p) + 1).b32.s1
    };
}

macro_rules! TOKEN_LIST_ref_count {
    ($p:expr) => {
        mem!($p).b32.s0
    };
}

macro_rules! WRITE_NODE_tokens {
    ($p:expr) => {
        mem!(($p) + 1).b32.s0
    };
}

macro_rules! PIC_NODE_total_size {
    ($p:expr) => {
        mem!(($p) + 4).b16.s3
    };
}

macro_rules! INTPAR {
    ($p:expr) => {
        eqtb!(INT_BASE + ($p) as i32).b32.s1
    };
}

macro_rules! DIMENPAR {
    ($p:expr) => {
        eqtb!(DIMEN_BASE + ($p) as i32).b32.s1
    };
}

macro_rules! GLUEPAR {
    ($p:expr) => {
        eqtb!(GLUE_BASE + ($p) as i32).b32.s1
    };
}

macro_rules! LOCAL {
    ($p:expr) => {
        eqtb!(LOCAL_BASE + ($p) as i32).b32.s1
    };
}

macro_rules! ETEX_PENALTIES_PAR {
    ($p:expr) => {
        eqtb!(ETEX_PEN_BASE + ($p) as i32).b32.s1
    };
}

macro_rules! set_help_line {
    ($idx:expr, $str:expr) => {
        *(&raw mut help_line)
            .cast::<*const libc::c_char>()
            .offset($idx) = $str.as_ptr() as *const libc::c_char;
    };
}

// 61. append_kern
#[unsafe(export_name = "append_kern")]
pub unsafe extern "C" fn append_kern() {
    let s: u16 = cur_chr as u16;
    scan_dimen(s == MU_GLUE as u16, false, false);
    mem!(cur_list.tail).b32.s1 = new_kern(cur_val);
    cur_list.tail = LLIST_link!(cur_list.tail);
    mem!(cur_list.tail).b16.s0 = s;
}

// 62. off_save
#[unsafe(export_name = "off_save")]
pub unsafe extern "C" fn off_save() {
    let mut p: i32;

    if cur_group == (BOTTOM_LEVEL) as u8 {
        error_here_with_diagnostic(b"Extra \0".as_ptr() as *const c_char);
        print_cmd_chr(cur_cmd as u16, cur_chr);
        capture_to_diagnostic(std::ptr::null_mut());

        help_ptr = 1;
        set_help_line!(
            0,
            b"Things are pretty mixed up, but I think the worst is over.\0"
        );
        error();
    } else {
        back_input();
        p = get_avail();
        mem!(TEMP_HEAD).b32.s1 = p;

        error_here_with_diagnostic(b"Missing \0".as_ptr() as *const c_char);

        match cur_group as i32 {
            SEMI_SIMPLE_GROUP => {
                mem!(p).b32.s0 = CS_TOKEN_FLAG + FROZEN_END_GROUP;
                print_esc_cstr(b"endgroup\0".as_ptr() as *const c_char);
            }
            MATH_SHIFT_GROUP => {
                mem!(p).b32.s0 = MATH_SHIFT_TOKEN + b'$' as i32;
                print_char(b'$' as i32);
            }
            MATH_LEFT_GROUP => {
                mem!(p).b32.s0 = CS_TOKEN_FLAG + FROZEN_RIGHT;
                mem!(p).b32.s1 = get_avail();
                p = LLIST_link!(p);
                mem!(p).b32.s0 = OTHER_TOKEN + b'.' as i32;
                print_esc_cstr(b"right.\0".as_ptr() as *const c_char);
            }
            _ => {
                mem!(p).b32.s0 = RIGHT_BRACE_TOKEN + b'}' as i32;
                print_char(b'}' as i32);
            }
        }

        print_cstr(b" inserted\0".as_ptr() as *const c_char);
        begin_token_list(mem!(TEMP_HEAD).b32.s1, INSERTED as u16);
        capture_to_diagnostic(std::ptr::null_mut());

        help_ptr = 5;
        set_help_line!(4, b"I've inserted something that you may have forgotten.\0");
        set_help_line!(3, b"(See the <inserted text> above.)\0");
        set_help_line!(2, b"With luck, this will get me unwedged. But if you\0");
        set_help_line!(
            1,
            b"really didn't forget anything, try typing `2' now; then\0"
        );
        set_help_line!(
            0,
            b"my insertion and my current dilemma will both disappear.\0"
        );
        error();
    }
}

// 63. extra_right_brace
#[unsafe(export_name = "extra_right_brace")]
pub unsafe extern "C" fn extra_right_brace() {
    error_here_with_diagnostic(b"Extra }, or forgotten \0".as_ptr() as *const c_char);

    match cur_group as i32 {
        SEMI_SIMPLE_GROUP => {
            print_esc_cstr(b"endgroup\0".as_ptr() as *const c_char);
        }
        MATH_SHIFT_GROUP => {
            print_char(b'$' as i32);
        }
        MATH_LEFT_GROUP => {
            print_esc_cstr(b"right\0".as_ptr() as *const c_char);
        }
        _ => {}
    }

    capture_to_diagnostic(std::ptr::null_mut());

    help_ptr = 5;
    set_help_line!(
        4,
        b"I've deleted a group-closing symbol because it seems to be\0"
    );
    set_help_line!(
        3,
        b"spurious, as in `$x}$'. But perhaps the } is legitimate and\0"
    );
    set_help_line!(
        2,
        b"you forgot something else, as in `\\hbox{$x}'. In such cases\0"
    );
    set_help_line!(
        1,
        b"the way to recover is to insert both the forgotten and the\0"
    );
    set_help_line!(0, b"deleted material, e.g., by typing `I$}'.\0");
    error();
    align_state += 1;
}

// 64. normal_paragraph
#[unsafe(export_name = "normal_paragraph")]
pub unsafe extern "C" fn normal_paragraph() {
    if INTPAR!(INT_PAR__looseness) != 0 {
        eq_word_define(INT_BASE + INT_PAR__looseness as i32, 0);
    }
    if DIMENPAR!(DIMEN_PAR__hang_indent) != 0 {
        eq_word_define(DIMEN_BASE + DIMEN_PAR__hang_indent as i32, 0);
    }
    if INTPAR!(INT_PAR__hang_after) != 1 {
        eq_word_define(INT_BASE + INT_PAR__hang_after as i32, 1);
    }
    if LOCAL!(LOCAL__par_shape) != TEX_NULL {
        eq_define(
            LOCAL_BASE + LOCAL__par_shape as i32,
            SHAPE_REF as u16,
            TEX_NULL,
        );
    }
    if ETEX_PENALTIES_PAR!(ETEX_PENALTIES_PAR__inter_line_penalties) != TEX_NULL {
        eq_define(
            ETEX_PEN_BASE + ETEX_PENALTIES_PAR__inter_line_penalties as i32,
            SHAPE_REF as u16,
            TEX_NULL,
        );
    }
}

// 65. box_end
#[unsafe(export_name = "box_end")]
pub unsafe extern "C" fn box_end(box_context: i32) {
    let mut p: i32;
    let mut a: small_number;

    if box_context < BOX_FLAG {
        if cur_box != TEX_NULL {
            mem!(cur_box + 4).b32.s1 = box_context;

            if cur_list.mode.abs() as i32 == VMODE {
                if pre_adjust_tail != TEX_NULL {
                    if PRE_ADJUST_HEAD != pre_adjust_tail {
                        mem!(cur_list.tail).b32.s1 = mem!(PRE_ADJUST_HEAD).b32.s1;
                        cur_list.tail = pre_adjust_tail;
                    }
                    pre_adjust_tail = TEX_NULL;
                }

                append_to_vlist(cur_box);

                if adjust_tail != TEX_NULL {
                    if ADJUST_HEAD != adjust_tail {
                        mem!(cur_list.tail).b32.s1 = mem!(ADJUST_HEAD).b32.s1;
                        cur_list.tail = adjust_tail;
                    }
                    adjust_tail = TEX_NULL;
                }

                if cur_list.mode > 0 {
                    build_page();
                }
            } else {
                if cur_list.mode.abs() as i32 == HMODE {
                    cur_list.aux.b32.s0 = 1000;
                } else {
                    p = new_noad();
                    mem!(p + 1).b32.s1 = SUB_BOX;
                    mem!(p + 1).b32.s0 = cur_box;
                    cur_box = p;
                }

                mem!(cur_list.tail).b32.s1 = cur_box;
                cur_list.tail = cur_box;
            }
        }
    } else if box_context < SHIP_OUT_FLAG {
        if box_context < GLOBAL_BOX_FLAG {
            cur_val = box_context - BOX_FLAG;
            a = 0;
        } else {
            cur_val = box_context - GLOBAL_BOX_FLAG;
            a = 4;
        }

        if cur_val < 256 {
            if a >= 4 {
                geq_define(BOX_BASE + cur_val, BOX_REF as u16, cur_box);
            } else {
                eq_define(BOX_BASE + cur_val, BOX_REF as u16, cur_box);
            }
        } else {
            find_sa_element(4, cur_val, true);
            if a >= 4 {
                gsa_def(cur_ptr, cur_box);
            } else {
                sa_def(cur_ptr, cur_box);
            }
        }
    } else if cur_box != TEX_NULL {
        if box_context > SHIP_OUT_FLAG {
            loop {
                get_x_token();
                if cur_cmd as i32 != SPACER && cur_cmd as i32 != RELAX {
                    break;
                }
            }

            if (cur_cmd as i32 == HSKIP && cur_list.mode.abs() as i32 != VMODE)
                || (cur_cmd as i32 == VSKIP && cur_list.mode.abs() as i32 == VMODE)
            {
                append_glue();
                mem!(cur_list.tail).b16.s0 = (box_context - (LEADER_FLAG - A_LEADERS)) as u16;
                mem!(cur_list.tail + 1).b32.s1 = cur_box;
            } else {
                error_here_with_diagnostic(
                    b"Leaders not followed by proper glue\0".as_ptr() as *const c_char
                );
                capture_to_diagnostic(std::ptr::null_mut());

                help_ptr = 3;
                set_help_line!(
                    2,
                    b"You should say `\\leaders <box or rule><hskip or vskip>'.\0"
                );
                set_help_line!(1, b"I found the <box or rule>, but there's no suitable\0");
                set_help_line!(0, b"<hskip or vskip>, so I'm ignoring these leaders.\0");
                back_error();
                flush_node_list(cur_box);
            }
        } else {
            ship_out(cur_box);
        }
    }
}

// 66. begin_box
#[unsafe(export_name = "begin_box")]
pub unsafe extern "C" fn begin_box(box_context: i32) {
    let mut p: i32;
    let mut q: i32;
    let mut r: i32;
    let mut fm: bool;
    let mut tx: i32;
    let mut m: u16;
    let mut k: i32;
    let mut n: i32;

    match cur_chr as i32 {
        BOX_CODE => {
            scan_register_num();

            if cur_val < 256 {
                cur_box = BOX_REG!(cur_val);
            } else {
                find_sa_element(4, cur_val, false);
                if cur_ptr == TEX_NULL {
                    cur_box = TEX_NULL;
                } else {
                    cur_box = mem!(cur_ptr + 1).b32.s1;
                }
            }

            if cur_val < 256 {
                BOX_REG!(cur_val) = TEX_NULL;
            } else {
                find_sa_element(4, cur_val, false);
                if cur_ptr != TEX_NULL {
                    mem!(cur_ptr + 1).b32.s1 = TEX_NULL;
                    mem!(cur_ptr + 1).b32.s0 += 1;
                    delete_sa_ref(cur_ptr);
                }
            }
        }

        COPY_CODE => {
            scan_register_num();

            if cur_val < 256 {
                q = BOX_REG!(cur_val);
            } else {
                find_sa_element(4, cur_val, false);
                if cur_ptr == TEX_NULL {
                    q = TEX_NULL;
                } else {
                    q = mem!(cur_ptr + 1).b32.s1;
                }
            }

            cur_box = copy_node_list(q);
        }

        LAST_BOX_CODE => {
            cur_box = TEX_NULL;

            if cur_list.mode.abs() as i32 == MMODE {
                you_cant();
                help_ptr = 1;
                set_help_line!(0, b"Sorry; this \\lastbox will be void.\0");
                error();
            } else if cur_list.mode as i32 == VMODE && cur_list.head == cur_list.tail {
                you_cant();
                help_ptr = 2;
                set_help_line!(
                    1,
                    b"Sorry...I usually can't take things from the current page.\0"
                );
                set_help_line!(0, b"This \\lastbox will therefore be void.\0");
                error();
            } else {
                tx = cur_list.tail;

                if tx < hi_mem_min {
                    if NODE_type!(tx) as i32 == MATH_NODE && mem!(tx).b16.s0 as i32 == END_M_CODE {
                        r = cur_list.head;
                        loop {
                            q = r;
                            r = mem!(q).b32.s1;
                            if r == tx {
                                break;
                            }
                        }
                        tx = q;
                    }
                }

                'block_done: {
                    if tx < hi_mem_min {
                        if NODE_type!(tx) as i32 == HLIST_NODE
                            || NODE_type!(tx) as i32 == VLIST_NODE
                        {
                            q = cur_list.head;
                            p = TEX_NULL;

                            loop {
                                r = p;
                                p = q;
                                fm = false;

                                if q < hi_mem_min {
                                    if NODE_type!(q) as i32 == DISC_NODE {
                                        for m in 1..=mem!(q).b16.s0 {
                                            p = LLIST_link!(p);
                                        }

                                        if p == tx {
                                            break 'block_done;
                                        }
                                    } else if NODE_type!(q) as i32 == MATH_NODE
                                        && mem!(q).b16.s0 as i32 == BEGIN_M_CODE
                                    {
                                        fm = true;
                                    }
                                }

                                q = mem!(p).b32.s1;
                                if q == tx {
                                    break;
                                }
                            }

                            q = mem!(tx).b32.s1;
                            mem!(p).b32.s1 = q;
                            mem!(tx).b32.s1 = TEX_NULL;

                            if q == TEX_NULL {
                                if fm {
                                    confusion(b"tail1\0".as_ptr() as *const c_char);
                                } else {
                                    cur_list.tail = p;
                                }
                            } else if fm {
                                cur_list.tail = r;
                                mem!(r).b32.s1 = TEX_NULL;
                                flush_node_list(p);
                            }

                            cur_box = tx;
                            mem!(cur_box + 4).b32.s1 = 0;
                        }
                    }
                }
            }
        }

        VSPLIT_CODE => {
            scan_register_num();
            n = cur_val;

            if !scan_keyword(b"to\0".as_ptr() as *const c_char) {
                error_here_with_diagnostic(b"Missing `to' inserted\0".as_ptr() as *const c_char);
                capture_to_diagnostic(std::ptr::null_mut());

                help_ptr = 2;
                set_help_line!(1, b"I'm working on `\\vsplit<box number> to <dimen>';\0");
                set_help_line!(0, b"will look for the <dimen> next.\0");
                error();
            }

            scan_dimen(false, false, false);
            cur_box = vsplit(n, cur_val);
        }

        _ => {
            k = cur_chr - 4;
            (*save_stack.offset((save_ptr + 0) as isize)).b32.s1 = box_context;
            if k == HMODE {
                if box_context < BOX_FLAG && cur_list.mode.abs() as i32 == VMODE {
                    scan_spec(ADJUSTED_HBOX_GROUP as group_code, true);
                } else {
                    scan_spec(HBOX_GROUP as group_code, true);
                }
            } else {
                if k == VMODE {
                    scan_spec(VBOX_GROUP as group_code, true);
                } else {
                    scan_spec(VTOP_GROUP as group_code, true);
                    k = VMODE;
                }
                normal_paragraph();
            }

            push_nest();
            cur_list.mode = -(k as i16);

            if k == VMODE {
                cur_list.aux.b32.s1 = IGNORE_DEPTH;
                if LOCAL!(LOCAL__every_vbox) != TEX_NULL {
                    begin_token_list(LOCAL!(LOCAL__every_vbox), EVERY_VBOX_TEXT as u16);
                }
            } else {
                cur_list.aux.b32.s0 = 1000;
                if LOCAL!(LOCAL__every_hbox) != TEX_NULL {
                    begin_token_list(LOCAL!(LOCAL__every_hbox), EVERY_HBOX_TEXT as u16);
                }
            }

            return;
        }
    }

    box_end(box_context);
}

// 67. scan_box
#[unsafe(export_name = "scan_box")]
pub unsafe extern "C" fn scan_box(box_context: i32) {
    loop {
        get_x_token();
        if cur_cmd as i32 != SPACER && cur_cmd as i32 != RELAX {
            break;
        }
    }

    if cur_cmd as i32 == MAKE_BOX {
        begin_box(box_context);
    } else if box_context >= LEADER_FLAG && (cur_cmd as i32 == HRULE || cur_cmd as i32 == VRULE) {
        cur_box = scan_rule_spec();
        box_end(box_context);
    } else {
        error_here_with_diagnostic(b"A <box> was supposed to be here\0".as_ptr() as *const c_char);
        capture_to_diagnostic(std::ptr::null_mut());

        help_ptr = 3;
        set_help_line!(
            2,
            b"I was expecting to see \\hbox or \\vbox or \\copy or \\box or\0"
        );
        set_help_line!(
            1,
            b"something like that. So you might find something missing in\0"
        );
        set_help_line!(
            0,
            b"your output. But keep trying; you can fix this later.\0"
        );
        back_error();
    }
}

// 68. package
#[unsafe(export_name = "package")]
pub unsafe extern "C" fn package(c: small_number) {
    let mut h: scaled_t;
    let mut p: i32;
    let d: scaled_t;
    let u: i32;
    let v: i32;

    d = DIMENPAR!(DIMEN_PAR__box_max_depth);
    u = INTPAR!(INT_PAR__xetex_upwards);
    unsave();
    save_ptr -= 3;
    v = INTPAR!(INT_PAR__xetex_upwards);
    INTPAR!(INT_PAR__xetex_upwards) = u;
    if cur_list.mode as i32 == -104 {
        cur_box = hpack(
            mem!(cur_list.head).b32.s1,
            (*save_stack.offset((save_ptr + 2) as isize)).b32.s1,
            ((*save_stack.offset((save_ptr + 1) as isize)).b32.s1) as i16,
        );
    } else {
        cur_box = vpackage(
            mem!(cur_list.head).b32.s1,
            (*save_stack.offset((save_ptr + 2) as isize)).b32.s1,
            ((*save_stack.offset((save_ptr + 1) as isize)).b32.s1) as i16,
            d,
        );
        if c as i32 == VTOP_CODE {
            h = 0;
            p = mem!(cur_box + 5).b32.s1;
            if p != TEX_NULL {
                if (NODE_type!(p) as i32) <= RULE_NODE {
                    h = mem!(p + 3).b32.s1;
                }
            }
            mem!(cur_box + 2).b32.s1 = mem!(cur_box + 2).b32.s1 - h + mem!(cur_box + 3).b32.s1;
            mem!(cur_box + 3).b32.s1 = h;
        }
    }
    INTPAR!(INT_PAR__xetex_upwards) = v;
    pop_nest();
    box_end((*save_stack.offset((save_ptr + 0) as isize)).b32.s1);
}

// 69. norm_min
#[unsafe(export_name = "norm_min")]
pub unsafe extern "C" fn norm_min(h: i32) -> small_number {
    if h <= 0 {
        1
    } else if h >= 63 {
        63
    } else {
        h as small_number
    }
}

// 70. new_graf
#[unsafe(export_name = "new_graf")]
pub unsafe extern "C" fn new_graf(indented: bool) {
    cur_list.prev_graf = 0;

    if cur_list.mode as i32 == VMODE || cur_list.head != cur_list.tail {
        mem!(cur_list.tail).b32.s1 = new_param_glue(GLUE_PAR__par_skip as small_number);
        cur_list.tail = LLIST_link!(cur_list.tail);
    }

    push_nest();
    cur_list.mode = HMODE as i16;
    cur_list.aux.b32.s0 = 1000;

    if INTPAR!(INT_PAR__language) <= 0 {
        cur_lang = 0;
    } else if INTPAR!(INT_PAR__language) > BIGGEST_LANG {
        cur_lang = 0;
    } else {
        cur_lang = INTPAR!(INT_PAR__language) as u8;
    }

    cur_list.aux.b32.s1 = (cur_lang) as i32;
    cur_list.prev_graf = ((norm_min(INTPAR!(INT_PAR__left_hyphen_min)) as i32) * 64
        + (norm_min(INTPAR!(INT_PAR__right_hyphen_min)) as i32))
        * 65536
        + (cur_lang) as i32;

    if indented {
        cur_list.tail = new_null_box();
        mem!(cur_list.head).b32.s1 = cur_list.tail;
        mem!(cur_list.tail + 1).b32.s1 = eqtb!(DIMEN_BASE).b32.s1;
        if insert_src_special_every_par {
            insert_src_special();
        }
    }

    if LOCAL!(LOCAL__every_par) != TEX_NULL {
        begin_token_list(LOCAL!(LOCAL__every_par), EVERY_PAR_TEXT as u16);
    }

    if nest_ptr == 1 {
        build_page();
    }
}

// 71. indent_in_hmode
#[unsafe(export_name = "indent_in_hmode")]
pub unsafe extern "C" fn indent_in_hmode() {
    let mut p: i32;
    let q: i32;

    if cur_chr > 0 {
        p = new_null_box();
        mem!(p + 1).b32.s1 = eqtb!(DIMEN_BASE).b32.s1;
        if cur_list.mode.abs() as i32 == HMODE {
            cur_list.aux.b32.s0 = 1000;
        } else {
            q = new_noad();
            mem!(q + 1).b32.s1 = SUB_BOX;
            mem!(q + 1).b32.s0 = p;
            p = q;
        }
        mem!(cur_list.tail).b32.s1 = p;
        cur_list.tail = LLIST_link!(cur_list.tail);
    }
}

// 72. head_for_vmode
#[unsafe(export_name = "head_for_vmode")]
pub unsafe extern "C" fn head_for_vmode() {
    if cur_list.mode < 0 {
        if cur_cmd as i32 != HRULE {
            off_save();
        } else {
            error_here_with_diagnostic(b"You can't use `\0".as_ptr() as *const c_char);
            print_esc_cstr(b"hrule\0".as_ptr() as *const c_char);
            print_cstr(b"' here except with leaders\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());

            help_ptr = 2;
            set_help_line!(1, b"To put a horizontal rule in an hbox or an alignment,\0");
            set_help_line!(
                0,
                b"you should use \\leaders or \\hrulefill (see The TeXbook).\0"
            );
            error();
        }
    } else {
        back_input();
        cur_tok = par_token;
        back_input();
        cur_input.index = INSERTED as u16;
    }
}

// 73. end_graf
#[unsafe(export_name = "end_graf")]
pub unsafe extern "C" fn end_graf() {
    if cur_list.mode as i32 == HMODE {
        if cur_list.head == cur_list.tail {
            pop_nest();
        } else {
            line_break(false);
        }

        if cur_list.eTeX_aux != TEX_NULL {
            flush_list(cur_list.eTeX_aux);
            cur_list.eTeX_aux = TEX_NULL;
        }

        normal_paragraph();
        error_count = 0;
    }
}

// 74. begin_insert_or_adjust
#[unsafe(export_name = "begin_insert_or_adjust")]
pub unsafe extern "C" fn begin_insert_or_adjust() {
    if cur_cmd as i32 == VADJUST {
        cur_val = 255;
    } else {
        scan_eight_bit_int();
        if cur_val == 255 {
            error_here_with_diagnostic(b"You can't \0".as_ptr() as *const c_char);
            print_esc_cstr(b"insert\0".as_ptr() as *const c_char);
            print_int(255);
            capture_to_diagnostic(std::ptr::null_mut());

            help_ptr = 1;
            set_help_line!(0, b"I'm changing to \\insert0; box 255 is special.\0");
            error();
            cur_val = 0;
        }
    }
    (*save_stack.offset((save_ptr + 0) as isize)).b32.s1 = cur_val;
    if cur_cmd as i32 == VADJUST && scan_keyword(b"pre\0".as_ptr() as *const c_char) {
        (*save_stack.offset((save_ptr + 1) as isize)).b32.s1 = 1;
    } else {
        (*save_stack.offset((save_ptr + 1) as isize)).b32.s1 = 0;
    }
    save_ptr += 2;
    new_save_level(INSERT_GROUP as group_code);
    scan_left_brace();
    normal_paragraph();
    push_nest();
    cur_list.mode = -1;
    cur_list.aux.b32.s1 = IGNORE_DEPTH;
}

// 75. make_mark
#[unsafe(export_name = "make_mark")]
pub unsafe extern "C" fn make_mark() {
    let mut p: i32;
    let c: i32;
    if cur_chr == 0 {
        c = 0;
    } else {
        scan_register_num();
        c = cur_val;
    }
    p = scan_toks(false, true);
    p = get_node(SMALL_NODE_SIZE);
    mem!(p + 1).b32.s0 = c;
    NODE_type!(p) = MARK_NODE as u16;
    mem!(p).b16.s0 = 0;
    mem!(p + 1).b32.s1 = def_ref;
    mem!(cur_list.tail).b32.s1 = p;
    cur_list.tail = p;
}

// 76. append_penalty
#[unsafe(export_name = "append_penalty")]
pub unsafe extern "C" fn append_penalty() {
    scan_int();
    mem!(cur_list.tail).b32.s1 = new_penalty(cur_val);
    cur_list.tail = LLIST_link!(cur_list.tail);
    if cur_list.mode as i32 == VMODE {
        build_page();
    }
}

// 77. delete_last
#[unsafe(export_name = "delete_last")]
pub unsafe extern "C" fn delete_last() {
    let mut p: i32;
    let mut q: i32;
    let mut r: i32;
    let mut fm: bool;
    let mut tx: i32;
    let mut m: u16;
    if cur_list.mode as i32 == VMODE && cur_list.tail == cur_list.head {
        if cur_chr != GLUE_NODE || last_glue != MAX_HALFWORD {
            you_cant();
            help_ptr = 2;
            set_help_line!(
                1,
                b"Sorry...I usually can't take things from the current page.\0"
            );
            if cur_chr == KERN_NODE {
                set_help_line!(0, b"Try `I\\kern-\\lastkern' instead.\0");
            } else if cur_chr != GLUE_NODE {
                set_help_line!(0, b"Perhaps you can make the output routine do it.\0");
            } else {
                set_help_line!(0, b"Try `I\\vskip-\\lastskip' instead.\0");
            }
            error();
        }
    } else {
        tx = cur_list.tail;
        if !is_char_node(tx) {
            if NODE_type!(tx) as i32 == MATH_NODE && mem!(tx).b16.s0 as i32 == END_M_CODE {
                r = cur_list.head;
                loop {
                    q = r;
                    r = mem!(q).b32.s1;
                    if r == tx {
                        break;
                    }
                }
                tx = q;
            }
        }
        if !is_char_node(tx) {
            if mem!(tx).b16.s1 as i32 == cur_chr {
                q = cur_list.head;
                p = TEX_NULL;
                loop {
                    r = p;
                    p = q;
                    fm = false;
                    if !is_char_node(q) {
                        if NODE_type!(q) as i32 == DISC_NODE {
                            let for_end = mem!(q).b16.s0 as i32;
                            m = 1;
                            while (m as i32) <= for_end {
                                p = LLIST_link!(p);
                                m += 1;
                            }
                            if p == tx {
                                return;
                            }
                        } else if NODE_type!(q) as i32 == MATH_NODE
                            && mem!(q).b16.s0 as i32 == BEGIN_M_CODE
                        {
                            fm = true;
                        }
                    }
                    q = mem!(p).b32.s1;
                    if q == tx {
                        break;
                    }
                }
                q = mem!(tx).b32.s1;
                mem!(p).b32.s1 = q;
                mem!(tx).b32.s1 = TEX_NULL;
                if q == TEX_NULL {
                    if fm {
                        confusion(b"tail1\0".as_ptr() as *const c_char);
                    } else {
                        cur_list.tail = p;
                    }
                } else if fm {
                    cur_list.tail = r;
                    mem!(r).b32.s1 = TEX_NULL;
                    flush_node_list(p);
                }
                flush_node_list(tx);
            }
        }
    }
}

// 78. unpackage
#[unsafe(export_name = "unpackage")]
pub unsafe extern "C" fn unpackage() {
    let mut p: i32;
    let mut r: i32;
    let c: u8;

    'body: {
        if cur_chr > COPY_CODE {
            mem!(cur_list.tail).b32.s1 = disc_ptr[cur_chr as usize];
            disc_ptr[cur_chr as usize] = TEX_NULL;
            break 'body;
        }
        c = cur_chr as u8;
        scan_register_num();
        if cur_val < 256 {
            p = BOX_REG!(cur_val);
        } else {
            find_sa_element(4, cur_val, false);
            if cur_ptr == TEX_NULL {
                p = TEX_NULL;
            } else {
                p = mem!(cur_ptr + 1).b32.s1;
            }
        }
        if p == TEX_NULL {
            return;
        }
        if cur_list.mode.abs() as i32 == MMODE
            || (cur_list.mode.abs() as i32 == VMODE && NODE_type!(p) as i32 != VLIST_NODE)
            || (cur_list.mode.abs() as i32 == HMODE && NODE_type!(p) as i32 != HLIST_NODE)
        {
            error_here_with_diagnostic(
                b"Incompatible list can't be unboxed\0".as_ptr() as *const c_char
            );
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 3;
            set_help_line!(2, b"Sorry, Pandora. (You sneaky devil.)\0");
            set_help_line!(
                1,
                b"I refuse to unbox an \\hbox in vertical mode or vice versa.\0"
            );
            set_help_line!(0, b"And I can't open any boxes in math mode.\0");
            error();
            return;
        }
        if c as i32 == COPY_CODE {
            mem!(cur_list.tail).b32.s1 = copy_node_list(mem!(p + 5).b32.s1);
        } else {
            mem!(cur_list.tail).b32.s1 = mem!(p + 5).b32.s1;
            if cur_val < 256 {
                BOX_REG!(cur_val) = TEX_NULL;
            } else {
                find_sa_element(4, cur_val, false);
                if cur_ptr != TEX_NULL {
                    mem!(cur_ptr + 1).b32.s1 = TEX_NULL;
                    mem!(cur_ptr + 1).b32.s0 += 1;
                    delete_sa_ref(cur_ptr);
                }
            }
            free_node(p, BOX_NODE_SIZE);
        }
    }

    while mem!(cur_list.tail).b32.s1 != TEX_NULL {
        r = mem!(cur_list.tail).b32.s1;
        if !is_char_node(r) && NODE_type!(r) as i32 == MARGIN_KERN_NODE {
            mem!(cur_list.tail).b32.s1 = mem!(r).b32.s1;
            free_node(r, MARGIN_KERN_NODE_SIZE);
        }
        cur_list.tail = LLIST_link!(cur_list.tail);
    }
}

// 79. append_italic_correction
#[unsafe(export_name = "append_italic_correction")]
pub unsafe extern "C" fn append_italic_correction() {
    let p: i32;
    let f: internal_font_number;
    if cur_list.tail != cur_list.head {
        if is_char_node(cur_list.tail) {
            p = cur_list.tail;
        } else if NODE_type!(cur_list.tail) as i32 == LIGATURE_NODE {
            p = cur_list.tail + 1;
        } else if NODE_type!(cur_list.tail) as i32 == WHATSIT_NODE {
            if mem!(cur_list.tail).b16.s0 as i32 == NATIVE_WORD_NODE
                || mem!(cur_list.tail).b16.s0 as i32 == NATIVE_WORD_NODE_AT
            {
                mem!(cur_list.tail).b32.s1 =
                    new_kern(super::xetex_ext::real_get_native_italic_correction(
                        mem.offset(cur_list.tail as isize).cast(),
                    ));
                cur_list.tail = LLIST_link!(cur_list.tail);
                NODE_subtype!(cur_list.tail) = EXPLICIT as u16;
            } else if mem!(cur_list.tail).b16.s0 as i32 == GLYPH_NODE {
                mem!(cur_list.tail).b32.s1 =
                    new_kern(super::xetex_ext::real_get_native_glyph_italic_correction(
                        mem.offset(cur_list.tail as isize).cast(),
                    ));
                cur_list.tail = LLIST_link!(cur_list.tail);
                NODE_subtype!(cur_list.tail) = EXPLICIT as u16;
            }
            return;
        } else {
            return;
        }
        f = CHAR_NODE_font!(p) as internal_font_number;
        let c_eff = effective_char(true, f, CHAR_NODE_character!(p) as u16);
        let char_info = FONT_CHARACTER_INFO!(f, c_eff);
        mem!(cur_list.tail).b32.s1 = new_kern(FONT_CHARINFO_ITALCORR!(f, char_info));
        cur_list.tail = LLIST_link!(cur_list.tail);
        NODE_subtype!(cur_list.tail) = EXPLICIT as u16;
    }
}

// 80. append_discretionary
#[unsafe(export_name = "append_discretionary")]
pub unsafe extern "C" fn append_discretionary() {
    let c: i32;

    mem!(cur_list.tail).b32.s1 = new_disc();
    cur_list.tail = LLIST_link!(cur_list.tail);

    if cur_chr == 1 {
        c = *hyphen_char.offset(eqtb!(CUR_FONT_LOC).b32.s1 as isize);
        if c >= 0 {
            if c <= BIGGEST_CHAR {
                mem!(cur_list.tail + 1).b32.s0 = new_character(
                    (eqtb!(CUR_FONT_LOC).b32.s1 as usize) as i32,
                    c as UTF16_code,
                );
            }
        }
    } else {
        save_ptr += 1;
        (*save_stack.offset((save_ptr - 1) as isize)).b32.s1 = 0;
        new_save_level(DISC_GROUP as group_code);
        scan_left_brace();
        push_nest();
        cur_list.mode = -104;
        cur_list.aux.b32.s0 = 1000;
    }
}

// 81. build_discretionary
#[unsafe(export_name = "build_discretionary")]
pub unsafe extern "C" fn build_discretionary() {
    let mut p: i32;
    let mut q: i32;
    let mut n: i32;
    unsave();
    q = cur_list.head;
    p = mem!(q).b32.s1;
    n = 0;
    'outer: while p != TEX_NULL {
        if !is_char_node(p) {
            if NODE_type!(p) as i32 > RULE_NODE {
                if NODE_type!(p) as i32 != KERN_NODE {
                    if NODE_type!(p) as i32 != LIGATURE_NODE {
                        if (NODE_type!(p) as i32 != WHATSIT_NODE)
                            || ((mem!(p).b16.s0 as i32 != NATIVE_WORD_NODE)
                                && (mem!(p).b16.s0 as i32 != NATIVE_WORD_NODE_AT)
                                && (mem!(p).b16.s0 as i32 != GLYPH_NODE))
                        {
                            error_here_with_diagnostic(
                                b"Improper discretionary list\0".as_ptr() as *const c_char
                            );
                            capture_to_diagnostic(std::ptr::null_mut());
                            help_ptr = 1;
                            set_help_line!(
                                0,
                                b"Discretionary lists must contain only boxes and kerns.\0"
                            );
                            error();

                            begin_diagnostic();
                            diagnostic_begin_capture_warning_here();

                            print_nl_cstr(
                                b"The following discretionary sublist has been deleted:\0".as_ptr()
                                    as *const c_char,
                            );
                            show_box(p);

                            capture_to_diagnostic(std::ptr::null_mut());
                            end_diagnostic(true);
                            flush_node_list(p);
                            mem!(q).b32.s1 = TEX_NULL;
                            break 'outer;
                        }
                    }
                }
            }
        }
        q = p;
        p = mem!(q).b32.s1;
        n += 1;
    }

    p = mem!(cur_list.head).b32.s1;
    pop_nest();
    match (*save_stack.offset((save_ptr - 1) as isize)).b32.s1 {
        0 => {
            mem!(cur_list.tail + 1).b32.s0 = p;
        }
        1 => {
            mem!(cur_list.tail + 1).b32.s1 = p;
        }
        2 => {
            if n > 0 && cur_list.mode.abs() as i32 == MMODE {
                error_here_with_diagnostic(b"Illegal math \0".as_ptr() as *const c_char);
                print_esc_cstr(b"discretionary\0".as_ptr() as *const c_char);
                capture_to_diagnostic(std::ptr::null_mut());

                help_ptr = 2;
                set_help_line!(
                    1,
                    b"Sorry: The third part of a discretionary break must be\0"
                );
                set_help_line!(
                    0,
                    b"empty, in math formulas. I had to delete your third part.\0"
                );
                flush_node_list(p);
                n = 0;
                error();
            } else {
                mem!(cur_list.tail).b32.s1 = p;
            }
            if n <= u16::MAX as i32 {
                mem!(cur_list.tail).b16.s0 = n as u16;
            } else {
                error_here_with_diagnostic(
                    b"Discretionary list is too long\0".as_ptr() as *const c_char
                );
                capture_to_diagnostic(std::ptr::null_mut());

                help_ptr = 2;
                set_help_line!(1, b"Wow---I never thought anybody would tweak me here.\0");
                set_help_line!(
                    0,
                    b"You can't seriously need such a huge discretionary list?\0"
                );
                error();
            }
            if n > 0 {
                cur_list.tail = q;
            }
            save_ptr -= 1;
            return;
        }
        _ => {}
    }
    (*save_stack.offset((save_ptr - 1) as isize)).b32.s1 += 1;
    new_save_level(DISC_GROUP as group_code);
    scan_left_brace();
    push_nest();
    cur_list.mode = -104;
    cur_list.aux.b32.s0 = 1000;
}

// 82. make_accent
#[unsafe(export_name = "make_accent")]
pub unsafe extern "C" fn make_accent() {
    let mut s: f64;
    let mut t: f64;
    let mut p: i32;
    let mut q: i32;
    let mut r: i32;
    let mut f: internal_font_number;
    let mut a: scaled_t;
    let mut h: scaled_t = 0;
    let x: scaled_t;
    let mut w: scaled_t;
    let mut local_delta: scaled_t = 0;
    let mut lsb: scaled_t = 0;
    let mut rsb: scaled_t = 0;
    let i: b16x4;

    scan_char_num();
    f = eqtb!(CUR_FONT_LOC).b32.s1 as internal_font_number;
    p = new_character(f, cur_val as UTF16_code);

    if p != TEX_NULL {
        x = (*font_info.offset((X_HEIGHT_CODE + *param_base.offset(f as isize)) as isize))
            .b32
            .s1;
        s = (*font_info.offset((SLANT_CODE + *param_base.offset(f as isize)) as isize))
            .b32
            .s1 as f64
            / 65536.0;
        let area = *font_area.offset(f as isize) as u32;
        if area == AAT_FONT_FLAG || area == OTGR_FONT_FLAG {
            a = mem!(p + 1).b32.s1;
            if a == 0 {
                get_native_char_sidebearings(f, cur_val, &mut lsb, &mut rsb);
            }
        } else {
            let eff = effective_char(true, f, CHAR_NODE_character!(p) as u16);
            a = FONT_CHARACTER_WIDTH!(f, eff);
        }
        do_assignments();
        q = TEX_NULL;
        f = eqtb!(CUR_FONT_LOC).b32.s1 as internal_font_number;
        if cur_cmd as i32 == LETTER || cur_cmd as i32 == OTHER_CHAR || cur_cmd as i32 == CHAR_GIVEN
        {
            q = new_character(f, cur_chr as UTF16_code);
            cur_val = cur_chr;
        } else if cur_cmd as i32 == CHAR_NUM {
            scan_char_num();
            q = new_character(f, cur_val as UTF16_code);
        } else {
            back_input();
        }
        if q != TEX_NULL {
            t = (*font_info.offset((SLANT_CODE + *param_base.offset(f as isize)) as isize))
                .b32
                .s1 as f64
                / 65536.0;
            let area_q = *font_area.offset(f as isize) as u32;
            if area_q == AAT_FONT_FLAG || area_q == OTGR_FONT_FLAG {
                w = mem!(q + 1).b32.s1;
                get_native_char_height_depth(f, cur_val, &mut h, &mut local_delta);
            } else {
                let eff_q = effective_char(true, f, CHAR_NODE_character!(q) as u16);
                i = FONT_CHARACTER_INFO!(f, eff_q);
                w = FONT_CHARINFO_WIDTH!(f, i);
                h = FONT_CHARINFO_HEIGHT!(f, i);
            }
            if h != x {
                p = hpack(p, 0, ADDITIONAL as small_number);
                mem!(p + 4).b32.s1 = x - h;
            }
            if (area_q == AAT_FONT_FLAG || area_q == OTGR_FONT_FLAG) && a == 0 {
                local_delta =
                    tex_round((w - lsb + rsb) as f64 / 2.0 + (h as f64) * t - (x as f64) * s);
            } else {
                local_delta = tex_round((w - a) as f64 / 2.0 + (h as f64) * t - (x as f64) * s);
            }
            r = new_kern(local_delta);
            NODE_subtype!(r) = ACC_KERN as u16;
            mem!(cur_list.tail).b32.s1 = r;
            mem!(r).b32.s1 = p;
            cur_list.tail = new_kern(-a - local_delta);
            NODE_subtype!(cur_list.tail) = ACC_KERN as u16;
            mem!(p).b32.s1 = cur_list.tail;
            p = q;
        }
        mem!(cur_list.tail).b32.s1 = p;
        cur_list.tail = p;
        cur_list.aux.b32.s0 = 1000;
    }
}

// 83. align_error
#[unsafe(export_name = "align_error")]
pub unsafe extern "C" fn align_error() {
    if align_state.abs() > 2 {
        error_here_with_diagnostic(b"Misplaced \0".as_ptr() as *const c_char);
        print_cmd_chr(cur_cmd as u16, cur_chr);
        capture_to_diagnostic(std::ptr::null_mut());

        if cur_tok == TAB_TOKEN + 38 {
            help_ptr = 6;
            set_help_line!(
                5,
                b"I can't figure out why you would want to use a tab mark\0"
            );
            set_help_line!(4, b"here. If you just want an ampersand, the remedy is\0");
            set_help_line!(
                3,
                b"simple: Just type `I\\&' now. But if some right brace\0"
            );
            set_help_line!(2, b"up above has ended a previous alignment prematurely,\0");
            set_help_line!(1, b"you're probably due for more error messages, and you\0");
            set_help_line!(
                0,
                b"might try typing `S' now just to see what is salvageable.\0"
            );
        } else {
            help_ptr = 5;
            set_help_line!(
                4,
                b"I can't figure out why you would want to use a tab mark\0"
            );
            set_help_line!(
                3,
                b"or \\cr or \\span just now. If something like a right brace\0"
            );
            set_help_line!(2, b"up above has ended a previous alignment prematurely,\0");
            set_help_line!(1, b"you're probably due for more error messages, and you\0");
            set_help_line!(
                0,
                b"might try typing `S' now just to see what is salvageable.\0"
            );
        }
        error();
    } else {
        back_input();
        if align_state < 0 {
            error_here_with_diagnostic(b"Missing { inserted\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());
            align_state += 1;
            cur_tok = LEFT_BRACE_TOKEN + 123;
        } else {
            error_here_with_diagnostic(b"Missing } inserted\0".as_ptr() as *const c_char);
            capture_to_diagnostic(std::ptr::null_mut());
            align_state -= 1;
            cur_tok = RIGHT_BRACE_TOKEN + 125;
        }
        help_ptr = 3;
        set_help_line!(2, b"I've put in what seems to be necessary to fix\0");
        set_help_line!(1, b"the current column of the current alignment.\0");
        set_help_line!(0, b"Try to go on, since this might almost work.\0");
        ins_error();
    }
}

// 84. no_align_error
#[unsafe(export_name = "no_align_error")]
pub unsafe extern "C" fn no_align_error() {
    error_here_with_diagnostic(b"Misplaced \0".as_ptr() as *const c_char);
    print_esc_cstr(b"noalign\0".as_ptr() as *const c_char);
    capture_to_diagnostic(std::ptr::null_mut());

    help_ptr = 2;
    set_help_line!(1, b"I expect to see \\noalign only after the \\cr of\0");
    set_help_line!(0, b"an alignment. Proceed, and I'll ignore this case.\0");
    error();
}

// 85. omit_error
#[unsafe(export_name = "omit_error")]
pub unsafe extern "C" fn omit_error() {
    error_here_with_diagnostic(b"Misplaced \0".as_ptr() as *const c_char);
    print_esc_cstr(b"omit\0".as_ptr() as *const c_char);
    capture_to_diagnostic(std::ptr::null_mut());

    help_ptr = 2;
    set_help_line!(
        1,
        b"I expect to see \\omit only after tab marks or the \\cr of\0"
    );
    set_help_line!(0, b"an alignment. Proceed, and I'll ignore this case.\0");
    error();
}

// 86. do_endv
#[unsafe(export_name = "do_endv")]
pub unsafe extern "C" fn do_endv() {
    base_ptr = input_ptr;
    *input_stack.offset(base_ptr as isize) = cur_input;
    while ((*input_stack.offset(base_ptr as isize)).index as i32 != V_TEMPLATE)
        && ((*input_stack.offset(base_ptr as isize)).loc == TEX_NULL)
        && ((*input_stack.offset(base_ptr as isize)).state as i32 == TOKEN_LIST as i32)
    {
        base_ptr -= 1;
    }
    if ((*input_stack.offset(base_ptr as isize)).index as i32 != V_TEMPLATE)
        || ((*input_stack.offset(base_ptr as isize)).loc != TEX_NULL)
        || ((*input_stack.offset(base_ptr as isize)).state as i32 != TOKEN_LIST as i32)
    {
        fatal_error(
            b"(interwoven alignment preambles are not allowed)\0".as_ptr() as *const c_char,
        );
    }
    if cur_group as i32 == ALIGN_GROUP {
        end_graf();
        if fin_col() {
            fin_row();
        }
    } else {
        off_save();
    }
}

// 87. cs_error
#[unsafe(export_name = "cs_error")]
pub unsafe extern "C" fn cs_error() {
    error_here_with_diagnostic(b"Extra \0".as_ptr() as *const c_char);
    print_esc_cstr(b"endcsname\0".as_ptr() as *const c_char);
    capture_to_diagnostic(std::ptr::null_mut());
    help_ptr = 1;
    set_help_line!(0, b"I'm ignoring this, since I wasn't doing a \\csname.\0");
    error();
}

// 88. push_math
#[unsafe(export_name = "push_math")]
pub unsafe extern "C" fn push_math(c: group_code) {
    push_nest();
    cur_list.mode = -207;
    cur_list.aux.b32.s1 = TEX_NULL;
    new_save_level(c);
}

// 89. just_copy
#[unsafe(export_name = "just_copy")]
pub unsafe extern "C" fn just_copy(mut p: i32, mut h: i32, t: i32) {
    let mut r: i32;
    let mut words: usize;

    while p != TEX_NULL {
        words = 1;

        if is_char_node(p) {
            r = get_avail();
        } else {
            match NODE_type!(p) as i32 {
                HLIST_NODE | VLIST_NODE => {
                    r = get_node(BOX_NODE_SIZE);
                    SYNCTEX_tag!(r, BOX_NODE_SIZE) = SYNCTEX_tag!(p, BOX_NODE_SIZE);
                    SYNCTEX_line!(r, BOX_NODE_SIZE) = SYNCTEX_line!(p, BOX_NODE_SIZE);
                    mem!(r + 6) = mem!(p + 6);
                    mem!(r + 5) = mem!(p + 5);
                    words = 5;
                    BOX_list_ptr!(r) = TEX_NULL;
                }

                RULE_NODE => {
                    r = get_node(RULE_NODE_SIZE);
                    words = RULE_NODE_SIZE as usize;
                }

                LIGATURE_NODE => {
                    r = get_avail();
                    mem!(r) = mem!(p + 1);
                    LLIST_link!(h) = r;
                    h = r;
                    p = LLIST_link!(p);
                    continue;
                }

                KERN_NODE | MATH_NODE => {
                    words = MEDIUM_NODE_SIZE as usize;
                    r = get_node(words as i32);
                }

                GLUE_NODE => {
                    r = get_node(MEDIUM_NODE_SIZE);
                    GLUE_SPEC_ref_count!(GLUE_NODE_glue_ptr!(p)) += 1;
                    SYNCTEX_tag!(r, MEDIUM_NODE_SIZE) = SYNCTEX_tag!(p, MEDIUM_NODE_SIZE);
                    SYNCTEX_line!(r, MEDIUM_NODE_SIZE) = SYNCTEX_line!(p, MEDIUM_NODE_SIZE);
                    GLUE_NODE_glue_ptr!(r) = GLUE_NODE_glue_ptr!(p);
                    GLUE_NODE_leader_ptr!(r) = TEX_NULL;
                }

                WHATSIT_NODE => match NODE_subtype!(p) as i32 {
                    OPEN_NODE => {
                        r = get_node(OPEN_NODE_SIZE);
                        words = OPEN_NODE_SIZE as usize;
                    }

                    WRITE_NODE | SPECIAL_NODE => {
                        r = get_node(WRITE_NODE_SIZE);
                        TOKEN_LIST_ref_count!(WRITE_NODE_tokens!(p)) += 1;
                        words = WRITE_NODE_SIZE as usize;
                    }

                    CLOSE_NODE | LANGUAGE_NODE => {
                        r = get_node(SMALL_NODE_SIZE);
                        words = SMALL_NODE_SIZE as usize;
                    }

                    NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT => {
                        words = NATIVE_NODE_size!(p) as usize;
                        r = get_node(words as i32);

                        let mut w = words;
                        while w > 0 {
                            w -= 1;
                            mem!(r + w as i32) = mem!(p + w as i32);
                        }

                        NATIVE_NODE_glyph_info_ptr!(r) = std::ptr::null_mut();
                        NATIVE_NODE_glyph_count!(r) = 0;
                        copy_native_glyph_info(p, r);
                    }

                    GLYPH_NODE => {
                        r = get_node(GLYPH_NODE_SIZE);
                        words = GLYPH_NODE_SIZE as usize;
                    }

                    PIC_NODE | PDF_NODE => {
                        words = PIC_NODE_total_size!(p) as usize;
                        r = get_node(words as i32);
                    }

                    PDF_SAVE_POS_NODE => {
                        r = get_node(SMALL_NODE_SIZE);
                    }

                    _ => {
                        confusion(b"ext2\0".as_ptr() as *const c_char);
                        unreachable!();
                    }
                },

                _ => {
                    p = LLIST_link!(p);
                    continue;
                }
            }
        }

        while words > 0 {
            words -= 1;
            mem!(r + words as i32) = mem!(p + words as i32);
        }

        LLIST_link!(h) = r;
        h = r;
        p = LLIST_link!(p);
    }

    LLIST_link!(h) = t;
}

// 90. just_reverse
#[unsafe(export_name = "just_reverse")]
pub unsafe extern "C" fn just_reverse(mut p: i32) {
    let mut l: i32;
    let t: i32;
    let mut q: i32;
    let mut m: i32;
    let mut n: i32;
    m = MIN_HALFWORD;
    n = MIN_HALFWORD;
    if mem!(TEMP_HEAD).b32.s1 == TEX_NULL {
        just_copy(mem!(p).b32.s1, TEMP_HEAD, TEX_NULL);
        q = mem!(TEMP_HEAD).b32.s1;
    } else {
        q = mem!(p).b32.s1;
        mem!(p).b32.s1 = TEX_NULL;
        flush_node_list(mem!(TEMP_HEAD).b32.s1);
    }
    t = new_edge(cur_dir as small_number, 0);
    l = t;
    cur_dir = 1 - cur_dir;
    'outer: while q != TEX_NULL {
        if is_char_node(q) {
            loop {
                p = q;
                q = mem!(p).b32.s1;
                mem!(p).b32.s1 = l;
                l = p;
                if !is_char_node(q) {
                    break;
                }
            }
        } else {
            p = q;
            q = mem!(p).b32.s1;
            if NODE_type!(p) as i32 == MATH_NODE {
                if odd(mem!(p).b16.s0 as i32) {
                    if mem!(LR_ptr).b32.s0 != (L_CODE * (mem!(p).b16.s0 as i32 / L_CODE) + 3) {
                        NODE_type!(p) = KERN_NODE as u16;
                        LR_problems += 1;
                    } else {
                        temp_ptr = LR_ptr;
                        LR_ptr = mem!(temp_ptr).b32.s1;
                        mem!(temp_ptr).b32.s1 = avail;
                        avail = temp_ptr;
                        if n > MIN_HALFWORD {
                            n -= 1;
                            mem!(p).b16.s0 -= 1;
                        } else {
                            if m > MIN_HALFWORD {
                                m -= 1;
                            } else {
                                mem!(t + 1).b32.s1 = mem!(p + 1).b32.s1;
                                mem!(t).b32.s1 = q;
                                free_node(p, MEDIUM_NODE_SIZE);
                                break 'outer;
                            }
                            NODE_type!(p) = KERN_NODE as u16;
                        }
                    }
                } else {
                    temp_ptr = get_avail();
                    mem!(temp_ptr).b32.s0 = L_CODE * (mem!(p).b16.s0 as i32 / L_CODE) + 3;
                    mem!(temp_ptr).b32.s1 = LR_ptr;
                    LR_ptr = temp_ptr;
                    if (n > MIN_HALFWORD) || ((mem!(p).b16.s0 as i32 / R_CODE) != cur_dir as i32) {
                        n += 1;
                        mem!(p).b16.s0 += 1;
                    } else {
                        NODE_type!(p) = KERN_NODE as u16;
                        m += 1;
                    }
                }
            }
            mem!(p).b32.s1 = l;
            l = p;
        }
    }
    mem!(TEMP_HEAD).b32.s1 = l;
}

pub(super) fn native_text_capacity(current: i32, required: i32) -> i32 {
    current
        .saturating_mul(2)
        .max(128)
        .max(required.saturating_add(1))
}

unsafe fn reserve_native_text(required: i32) {
    unsafe {
        if native_text_size <= required {
            if required == i32::MAX {
                overflow(c"native text buffer".as_ptr(), i32::MAX);
            }
            native_text_size = native_text_capacity(native_text_size, required);
            native_text = xrealloc(
                native_text.cast(),
                native_text_size as usize * std::mem::size_of::<UTF16_code>(),
            )
            .cast();
        }
    }
}

macro_rules! INTPAR {
    (xetex_use_glyph_metrics) => {
        eqtb!(INT_BASE + INT_PAR__xetex_use_glyph_metrics).b32.s1
    };
    (tracing_online) => {
        eqtb!(INT_BASE + INT_PAR__tracing_online).b32.s1
    };
    (texxet) => {
        eqtb!(INT_BASE + INT_PAR__texxet).b32.s1
    };
    (xetex_dash_break) => {
        eqtb!(INT_BASE + INT_PAR__xetex_dash_break).b32.s1
    };
    (tracing_lost_chars) => {
        eqtb!(INT_BASE + INT_PAR__tracing_lost_chars).b32.s1
    };
    (xetex_linebreak_locale) => {
        eqtb!(INT_BASE + INT_PAR__xetex_linebreak_locale).b32.s1
    };
    (xetex_default_input_mode) => {
        eqtb!(INT_BASE + INT_PAR__xetex_default_input_mode).b32.s1
    };
    (left_hyphen_min) => {
        eqtb!(INT_BASE + INT_PAR__left_hyphen_min).b32.s1
    };
    (floating_penalty) => {
        eqtb!(INT_BASE + INT_PAR__floating_penalty).b32.s1
    };
    (tracing_commands) => {
        eqtb!(INT_BASE + INT_PAR__tracing_commands).b32.s1
    };
    (xetex_inter_char_tokens) => {
        eqtb!(INT_BASE + INT_PAR__xetex_inter_char_tokens).b32.s1
    };
    (xetex_default_input_encoding) => {
        eqtb!(INT_BASE + INT_PAR__xetex_default_input_encoding)
            .b32
            .s1
    };
    (language) => {
        eqtb!(INT_BASE + INT_PAR__language).b32.s1
    };
    (xetex_interword_space_shaping) => {
        eqtb!(INT_BASE + INT_PAR__xetex_interword_space_shaping)
            .b32
            .s1
    };
    (new_line_char) => {
        eqtb!(INT_BASE + INT_PAR__new_line_char).b32.s1
    };
    (right_hyphen_min) => {
        eqtb!(INT_BASE + INT_PAR__right_hyphen_min).b32.s1
    };
}

macro_rules! DIMENPAR {
    (split_max_depth) => {
        eqtb!(DIMEN_BASE + DIMEN_PAR__split_max_depth).b32.s1
    };
}

macro_rules! GLUEPAR {
    (split_top_skip) => {
        eqtb!(GLUE_BASE + GLUE_PAR__split_top_skip).b32.s1
    };
    (space_skip) => {
        eqtb!(GLUE_BASE + GLUE_PAR__space_skip).b32.s1
    };
}

macro_rules! SF_CODE {
    ($n:expr) => {
        eqtb!(SF_CODE_BASE + ($n) as i32).b32.s1
    };
}

macro_rules! MATH_CODE {
    ($n:expr) => {
        eqtb!(MATH_CODE_BASE + ($n) as i32).b32.s1
    };
}

macro_rules! ETEX_SA_ptr {
    ($p:expr) => {
        mem!(($p) + 1).b32.s1
    };
}

macro_rules! set_help_line {
    ($idx:expr, $str:expr) => {
        *(&raw mut help_line).cast::<*const libc::c_char>().add($idx) =
            ($str).as_ptr() as *const libc::c_char;
    };
}

#[inline]
fn odd(n: i32) -> bool {
    (n & 1) != 0
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MainControlState {
    BigSwitch,
    Reswitch,
    MainLoop,
    MainTraditional,
    CollectNative,
    Collected,
    MainLoopWrapup,
    MainLoopMove,
    MainLoopMove1,
    MainLoopMove2,
    MainLoopLookahead,
    MainLoopLookahead1,
    MainLigLoop,
    MainLigLoop1,
    MainLigLoop2,
    MainLoopMoveLig,
    AppendNormalSpace,
}

// 91. get_r_token
#[unsafe(export_name = "get_r_token")]
pub unsafe extern "C" fn get_r_token() {
    'restart: loop {
        loop {
            get_token();
            if cur_tok != SPACE_TOKEN {
                break;
            }
        }

        if cur_cs == 0
            || cur_cs > eqtb_top
            || (cur_cs > FROZEN_CONTROL_SEQUENCE && cur_cs <= EQTB_SIZE)
        {
            error_here_with_diagnostic(
                b"Missing control sequence inserted\0".as_ptr() as *const c_char
            );
            capture_to_diagnostic(ptr::null_mut());

            help_ptr = 5;
            set_help_line!(
                4,
                b"Please don't say `\\def cs{...}', say `\\def\\cs{...}'.\0"
            );
            set_help_line!(
                3,
                b"I've inserted an inaccessible control sequence so that your\0"
            );
            set_help_line!(
                2,
                b"definition will be completed without mixing me up too badly.\0"
            );
            set_help_line!(
                1,
                b"You can recover graciously from this error, if you're\0"
            );
            set_help_line!(0, b"careful; see exercise 27.2 in The TeXbook.\0");

            if cur_cs == 0 {
                back_input();
            }

            cur_tok = CS_TOKEN_FLAG + FROZEN_PROTECTION;
            ins_error();
            continue 'restart;
        }
        break;
    }
}

// 92. trap_zero_glue
#[unsafe(export_name = "trap_zero_glue")]
pub unsafe extern "C" fn trap_zero_glue() {
    if (mem!(cur_val + 1).b32.s1 == 0)
        && (mem!(cur_val + 2).b32.s1 == 0)
        && (mem!(cur_val + 3).b32.s1 == 0)
    {
        GLUE_SPEC_ref_count!(0) += 1;
        delete_glue_ref(cur_val);
        cur_val = 0;
    }
}

// 93. do_register_command
#[unsafe(export_name = "do_register_command")]
pub unsafe extern "C" fn do_register_command(a: small_number) {
    let mut l: i32 = TEX_NULL;
    let mut q: i32;
    let mut r: i32;
    let mut s: i32 = TEX_NULL;
    let mut p: u8 = 0;
    let mut e: bool;
    let mut w: i32 = 0;

    q = cur_cmd as i32;
    e = false;

    'found: {
        if q != REGISTER {
            get_x_token();

            if (cur_cmd as i32) >= ASSIGN_INT && (cur_cmd as i32) <= ASSIGN_MU_GLUE {
                l = cur_chr;
                p = (cur_cmd as i32 - ASSIGN_INT) as u8;
                break 'found;
            }

            if cur_cmd as i32 != REGISTER {
                error_here_with_diagnostic(b"You can't use `\0".as_ptr() as *const c_char);
                print_cmd_chr(cur_cmd as u16, cur_chr);
                print_cstr(b"' after \0".as_ptr() as *const c_char);
                print_cmd_chr(q as u16, 0);
                capture_to_diagnostic(ptr::null_mut());

                help_ptr = 1;
                set_help_line!(
                    0,
                    b"I'm forgetting what you said and not changing anything.\0"
                );
                error();
                return;
            }
        }

        if cur_chr < 0 || cur_chr > 19
        /*lo_mem_stat_max*/
        {
            l = cur_chr;
            p = (mem!(l).b16.s1 / 64) as u8;
            e = true;
        } else {
            p = cur_chr as u8;
            scan_register_num();
            if cur_val > 255 {
                find_sa_element(p as small_number, cur_val, true);
                l = cur_ptr;
                e = true;
            } else {
                match p as i32 {
                    INT_VAL => l = cur_val + COUNT_BASE,
                    DIMEN_VAL => l = cur_val + SCALED_BASE,
                    GLUE_VAL => l = cur_val + SKIP_BASE,
                    MU_VAL => l = cur_val + MU_SKIP_BASE,
                    _ => {}
                }
            }
        }
    }

    if (p as i32) < GLUE_VAL {
        if e {
            w = mem!(l + 2).b32.s1;
        } else {
            w = eqtb!(l).b32.s1;
        }
    } else if e {
        s = mem!(l + 1).b32.s1;
    } else {
        s = eqtb!(l).b32.s1;
    }

    if q == REGISTER {
        scan_optional_equals();
    } else {
        scan_keyword(b"by\0".as_ptr() as *const c_char);
    }

    arith_error = false;

    if q < MULTIPLY {
        if (p as i32) < GLUE_VAL {
            if (p as i32) == INT_VAL {
                scan_int();
            } else {
                scan_dimen(false, false, false);
            }

            if q == ADVANCE {
                cur_val = cur_val + w;
            }
        } else {
            scan_glue((p as small_number) as u8);

            if q == ADVANCE {
                q = new_spec(cur_val);
                r = s;
                delete_glue_ref(cur_val);
                mem!(q + 1).b32.s1 = mem!(q + 1).b32.s1 + mem!(r + 1).b32.s1;

                if mem!(q + 2).b32.s1 == 0 {
                    mem!(q).b16.s1 = (NORMAL as u16) as u16;
                }

                if mem!(q).b16.s1 == mem!(r).b16.s1 {
                    mem!(q + 2).b32.s1 = mem!(q + 2).b32.s1 + mem!(r + 2).b32.s1;
                } else if mem!(q).b16.s1 < mem!(r).b16.s1 && mem!(r + 2).b32.s1 != 0 {
                    mem!(q + 2).b32.s1 = mem!(r + 2).b32.s1;
                    mem!(q).b16.s1 = mem!(r).b16.s1;
                }

                if mem!(q + 3).b32.s1 == 0 {
                    mem!(q).b16.s0 = (NORMAL as u16) as u16;
                }

                if mem!(q).b16.s0 == mem!(r).b16.s0 {
                    mem!(q + 3).b32.s1 = mem!(q + 3).b32.s1 + mem!(r + 3).b32.s1;
                } else if mem!(q).b16.s0 < mem!(r).b16.s0 && mem!(r + 3).b32.s1 != 0 {
                    mem!(q + 3).b32.s1 = mem!(r + 3).b32.s1;
                    mem!(q).b16.s0 = mem!(r).b16.s0;
                }

                cur_val = q;
            }
        }
    } else {
        scan_int();

        if (p as i32) < GLUE_VAL {
            if q == MULTIPLY {
                if (p as i32) == INT_VAL {
                    cur_val = mult_and_add(w, cur_val, 0, TEX_INFINITY);
                } else {
                    cur_val = mult_and_add(w, cur_val, 0, MAX_HALFWORD);
                }
            } else {
                cur_val = x_over_n(w, cur_val);
            }
        } else {
            r = new_spec(s);

            if q == MULTIPLY {
                mem!(r + 1).b32.s1 = mult_and_add(mem!(s + 1).b32.s1, cur_val, 0, MAX_HALFWORD);
                mem!(r + 2).b32.s1 = mult_and_add(mem!(s + 2).b32.s1, cur_val, 0, MAX_HALFWORD);
                mem!(r + 3).b32.s1 = mult_and_add(mem!(s + 3).b32.s1, cur_val, 0, MAX_HALFWORD);
            } else {
                mem!(r + 1).b32.s1 = x_over_n(mem!(s + 1).b32.s1, cur_val);
                mem!(r + 2).b32.s1 = x_over_n(mem!(s + 2).b32.s1, cur_val);
                mem!(r + 3).b32.s1 = x_over_n(mem!(s + 3).b32.s1, cur_val);
            }

            cur_val = r;
        }
    }

    if arith_error {
        error_here_with_diagnostic(b"Arithmetic overflow\0".as_ptr() as *const c_char);
        capture_to_diagnostic(ptr::null_mut());

        help_ptr = 2;
        set_help_line!(1, b"I can't carry out that multiplication or division,\0");
        set_help_line!(0, b"since the result is out of range.\0");
        if (p as i32) >= GLUE_VAL {
            delete_glue_ref(cur_val);
        }
        error();
        return;
    }

    if (p as i32) < GLUE_VAL {
        if e {
            if a >= 4 {
                gsa_w_def(l, cur_val);
            } else {
                sa_w_def(l, cur_val);
            }
        } else if a >= 4 {
            geq_word_define(l, cur_val);
        } else {
            eq_word_define(l, cur_val);
        }
    } else {
        trap_zero_glue();

        if e {
            if a >= 4 {
                gsa_def(l, cur_val);
            } else {
                sa_def(l, cur_val);
            }
        } else if a >= 4 {
            geq_define(l, GLUE_REF as u16, cur_val);
        } else {
            eq_define(l, GLUE_REF as u16, cur_val);
        }
    }
}

// 94. alter_aux
#[unsafe(export_name = "alter_aux")]
pub unsafe extern "C" fn alter_aux() {
    let mut c: i32;
    if cur_chr != (cur_list.mode as i32).abs() {
        report_illegal_case();
    } else {
        c = cur_chr;
        scan_optional_equals();
        if c == VMODE {
            scan_dimen(false, false, false);
            cur_list.aux.b32.s1 = cur_val;
        } else {
            scan_int();
            if cur_val <= 0 || cur_val > 32767 {
                let errmsg =
                    error_here_with_diagnostic(b"Bad space factor\0".as_ptr() as *const c_char);
                ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
                capture_to_diagnostic(ptr::null_mut());
                help_ptr = 1;
                set_help_line!(0, b"I allow only values in the range 1..32767 here.\0");
                int_error(cur_val);
            } else {
                cur_list.aux.b32.s0 = cur_val;
            }
        }
    }
}

// 95. alter_prev_graf
#[unsafe(export_name = "alter_prev_graf")]
pub unsafe extern "C" fn alter_prev_graf() {
    let mut p: i32;
    *nest.offset(nest_ptr as isize) = cur_list;
    p = nest_ptr;
    while (*nest.offset(p as isize)).mode.abs() as i32 != VMODE {
        p -= 1;
    }
    scan_optional_equals();
    scan_int();
    if cur_val < 0 {
        let errmsg = error_here_with_diagnostic(b"Bad \0".as_ptr() as *const c_char);
        ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
        print_esc_cstr(b"prevgraf\0".as_ptr() as *const c_char);
        capture_to_diagnostic(ptr::null_mut());
        help_ptr = 1;
        set_help_line!(0, b"I allow only nonnegative values here.\0");
        int_error(cur_val);
    } else {
        (*nest.offset(p as isize)).prev_graf = cur_val;
        cur_list = *nest.offset(nest_ptr as isize);
    }
}

// 96. alter_page_so_far
#[unsafe(export_name = "alter_page_so_far")]
pub unsafe extern "C" fn alter_page_so_far() {
    let mut c: u8;
    c = cur_chr as u8;
    scan_optional_equals();
    scan_dimen(false, false, false);
    *(&raw mut page_so_far).cast::<scaled_t>().add(c as usize) = cur_val;
}

// 97. alter_integer
#[unsafe(export_name = "alter_integer")]
pub unsafe extern "C" fn alter_integer() {
    let mut c: small_number;
    c = cur_chr as small_number;
    scan_optional_equals();
    scan_int();
    if c == 0 {
        dead_cycles = cur_val;
    } else if c == 2 {
        if cur_val < BATCH_MODE || cur_val > ERROR_STOP_MODE {
            let errmsg =
                error_here_with_diagnostic(b"Bad interaction mode\0".as_ptr() as *const c_char);
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(ptr::null_mut());
            help_ptr = 2;
            set_help_line!(1, b"Modes are 0=batch, 1=nonstop, 2=scroll, and\0");
            set_help_line!(0, b"3=errorstop. Proceed, and I'll ignore this case.\0");
            int_error(cur_val);
        } else {
            cur_chr = cur_val;
            new_interaction();
        }
    } else {
        insert_penalties = cur_val;
    }
}

// 98. alter_box_dimen
#[unsafe(export_name = "alter_box_dimen")]
pub unsafe extern "C" fn alter_box_dimen() {
    let mut c: small_number;
    let mut b: i32;

    c = cur_chr as small_number;
    scan_register_num();
    if cur_val < 256 {
        b = BOX_REG!(cur_val);
    } else {
        find_sa_element(4, cur_val, false);
        if cur_ptr == TEX_NULL {
            b = TEX_NULL;
        } else {
            b = mem!(cur_ptr + 1).b32.s1;
        }
    }
    scan_optional_equals();
    scan_dimen(false, false, false);
    if b != TEX_NULL {
        mem!(b + (c as i32)).b32.s1 = cur_val;
    }
}

// 99. new_font
#[unsafe(export_name = "new_font")]
pub unsafe extern "C" fn new_font(a: small_number) {
    let mut u: i32;
    let mut s: scaled_t;
    let mut f: internal_font_number;
    let mut t: str_number;
    let mut local_old_setting: u8;

    if job_name == 0 {
        open_log_file();
    }

    get_r_token();
    u = cur_cs;
    if u >= HASH_BASE {
        t = (*hash.offset(u as isize)).s1;
    } else if u >= SINGLE_BASE {
        if u == NULL_CS {
            t = maketexstring(b"FONT\0".as_ptr() as *const c_char);
        } else {
            t = u - SINGLE_BASE;
        }
    } else {
        local_old_setting = (selector) as u8;
        selector = (SELECTOR_NEW_STRING as u8) as i32;
        print_cstr(b"FONT\0".as_ptr() as *const c_char);
        print(u - 1);
        selector = (local_old_setting) as i32;
        if pool_ptr + 1 > pool_size {
            overflow(
                b"pool size\0".as_ptr() as *const c_char,
                (pool_size - init_pool_ptr) as i32,
            );
        }
        t = make_string();
    }

    if a >= 4 {
        geq_define(u, SET_FONT as u16, FONT_BASE);
    } else {
        eq_define(u, SET_FONT as u16, FONT_BASE);
    }

    scan_optional_equals();
    scan_file_name();
    name_in_progress = true;

    if scan_keyword(b"at\0".as_ptr() as *const c_char) {
        scan_dimen(false, false, false);
        s = cur_val;
        if s <= 0 || s >= 0x8000000 {
            error_here_with_diagnostic(b"Improper `at' size (\0".as_ptr() as *const c_char);
            print_scaled(s);
            print_cstr(b"pt), replaced by 10pt\0".as_ptr() as *const c_char);
            capture_to_diagnostic(ptr::null_mut());
            help_ptr = 2;
            set_help_line!(1, b"I can only handle fonts at positive sizes that are\0");
            set_help_line!(
                0,
                b"less than 2048pt, so I've changed what you said to 10pt.\0"
            );
            error();
            s = 10 * 65536;
        }
    } else if scan_keyword(b"scaled\0".as_ptr() as *const c_char) {
        scan_int();
        s = -cur_val;
        if cur_val <= 0 || cur_val > 32768 {
            let errmsg = error_here_with_diagnostic(
                b"Illegal magnification has been changed to 1000\0".as_ptr() as *const c_char,
            );
            ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
            capture_to_diagnostic(ptr::null_mut());
            help_ptr = 1;
            set_help_line!(0, b"The magnification ratio must be between 1 and 32768.\0");
            int_error(cur_val);
            s = -1000;
        }
    } else {
        s = -1000;
    }

    name_in_progress = false;

    'common_ending: {
        let for_end = font_ptr;
        f = FONT_BASE + 1;
        if f <= for_end {
            loop {
                if str_eq_str(*font_name.offset(f as isize), cur_name)
                    && (((length(cur_area) == 0)
                        && ((*font_area.offset(f as isize) == (AAT_FONT_FLAG) as i32)
                            || (*font_area.offset(f as isize) == (OTGR_FONT_FLAG) as i32)))
                        || str_eq_str(*font_area.offset(f as isize), cur_area))
                {
                    if s > 0 {
                        if s == *font_size.offset(f as isize) {
                            break 'common_ending;
                        }
                    } else if *font_size.offset(f as isize)
                        == xn_over_d(*font_dsize.offset(f as isize), -s, 1000)
                    {
                        break 'common_ending;
                    }
                }

                append_str(cur_area);
                append_str(cur_name);
                append_str(cur_ext);

                if str_eq_str(*font_name.offset(f as isize), make_string()) {
                    str_ptr -= 1;
                    pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
                    if (*font_area.offset(f as isize) == (AAT_FONT_FLAG) as i32)
                        || (*font_area.offset(f as isize) == (OTGR_FONT_FLAG) as i32)
                    {
                        if s > 0 {
                            if s == *font_size.offset(f as isize) {
                                break 'common_ending;
                            }
                        } else if *font_size.offset(f as isize)
                            == xn_over_d(*font_dsize.offset(f as isize), -s, 1000)
                        {
                            break 'common_ending;
                        }
                    }
                } else {
                    str_ptr -= 1;
                    pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
                }

                if f >= for_end {
                    break;
                }
                f += 1;
            }
        }
        f = read_font_info(u, cur_name, cur_area, s);
        break 'common_ending;
    }

    if a >= 4 {
        geq_define(u, SET_FONT as u16, f);
    } else {
        eq_define(u, SET_FONT as u16, f);
    }
    eqtb!(FONT_ID_BASE + (f as isize) as i32) = eqtb!(u as isize);
    (*hash.offset((FONT_ID_BASE + (f as isize) as i32) as isize)).s1 = t;
}

// 100. new_interaction
#[unsafe(export_name = "new_interaction")]
pub unsafe extern "C" fn new_interaction() {
    print_ln();
    interaction = (cur_chr) as u8;
    if interaction == (BATCH_MODE) as u8 {
        selector = (SELECTOR_NO_PRINT as u8) as i32;
    } else {
        selector = (SELECTOR_TERM_ONLY as u8) as i32;
    }
    if log_opened {
        selector = selector + 2;
    }
}

// 101. issue_message
#[unsafe(export_name = "issue_message")]
pub unsafe extern "C" fn issue_message() {
    let mut local_old_setting: u8;
    let mut c: u8;
    let mut s: str_number;

    c = cur_chr as u8;
    mem!(GARBAGE).b32.s1 = scan_toks(false, true);
    local_old_setting = (selector) as u8;
    selector = (SELECTOR_NEW_STRING as u8) as i32;
    token_show(def_ref);
    selector = (local_old_setting) as i32;
    flush_list(def_ref);
    if pool_ptr + 1 > pool_size {
        overflow(
            b"pool size\0".as_ptr() as *const c_char,
            (pool_size - init_pool_ptr) as i32,
        );
    }
    s = make_string();
    if c == 0 {
        if term_offset + length(s) > max_print_line - 2 {
            print_ln();
        } else if term_offset > 0 || file_offset > 0 {
            print_char(b' ' as i32);
        }
        print(s);
        ttstub_output_flush(rust_stdout);
    } else {
        error_here_with_diagnostic(b"\0".as_ptr() as *const c_char);
        print(s);
        capture_to_diagnostic(ptr::null_mut());

        if LOCAL!(LOCAL__err_help) != TEX_NULL {
            use_err_help = true;
        } else if long_help_seen {
            help_ptr = 1;
            set_help_line!(0, b"(That was another \\errmessage.)\0");
        } else {
            if interaction < (ERROR_STOP_MODE) as u8 {
                long_help_seen = true;
            }
            help_ptr = 4;
            set_help_line!(3, b"This error message was generated by an \\errmessage\0");
            set_help_line!(2, b"command, so I can't give any explicit help.\0");
            set_help_line!(
                1,
                b"Pretend that you're Hercule Poirot: Examine all clues,\0"
            );
            set_help_line!(0, b"and deduce the truth by order and method.\0");
        }
        error();
        use_err_help = false;
    }
    str_ptr -= 1;
    pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
}

// 102. shift_case
#[unsafe(export_name = "shift_case")]
pub unsafe extern "C" fn shift_case() {
    let mut b: i32;
    let mut p: i32;
    let mut t: i32;
    let mut c: i32;

    b = cur_chr;
    p = scan_toks(false, false);
    p = mem!(def_ref).b32.s1;

    while p != TEX_NULL {
        t = mem!(p).b32.s0;
        if t < CS_TOKEN_FLAG + SINGLE_BASE {
            c = t % MAX_CHAR_VAL;
            if eqtb!(b + c).b32.s1 != 0 {
                mem!(p).b32.s0 = t - c + eqtb!(b + c).b32.s1;
            }
        }
        p = LLIST_link!(p);
    }

    begin_token_list(mem!(def_ref).b32.s1, BACKED_UP as u16);
    mem!(def_ref).b32.s1 = avail;
    avail = def_ref;
}

// 103. show_whatever
#[unsafe(export_name = "show_whatever")]
pub unsafe extern "C" fn show_whatever() {
    let mut p: i32;
    let mut t: small_number;
    let mut m: u8;
    let mut l: i32;
    let mut n: i32;

    diagnostic_begin_capture_warning_here();

    'common_ending: {
        match cur_chr {
            3 => {
                begin_diagnostic();
                show_activities();
            }
            1 => {
                scan_register_num();
                if cur_val < 256 {
                    p = BOX_REG!(cur_val);
                } else {
                    find_sa_element(4, cur_val, false);
                    if cur_ptr == TEX_NULL {
                        p = TEX_NULL;
                    } else {
                        p = mem!(cur_ptr + 1).b32.s1;
                    }
                }
                begin_diagnostic();
                print_nl_cstr(b"> \\box\0".as_ptr() as *const c_char);
                print_int(cur_val);
                print_char(b'=' as i32);
                if p == TEX_NULL {
                    print_cstr(b"void\0".as_ptr() as *const c_char);
                } else {
                    show_box(p);
                }
            }
            0 => {
                get_token();
                print_nl_cstr(b"> \0".as_ptr() as *const c_char);
                if cur_cs != 0 {
                    sprint_cs(cur_cs);
                    print_char(b'=' as i32);
                }
                print_meaning();
                break 'common_ending;
            }
            4 => {
                begin_diagnostic();
                show_save_groups();
            }
            6 => {
                begin_diagnostic();
                print_nl_cstr(b"\0".as_ptr() as *const c_char);
                print_ln();
                if cond_ptr == TEX_NULL {
                    print_nl_cstr(b"### \0".as_ptr() as *const c_char);
                    print_cstr(b"no active conditionals\0".as_ptr() as *const c_char);
                } else {
                    p = cond_ptr;
                    n = 0;
                    loop {
                        n += 1;
                        p = LLIST_link!(p);
                        if p == TEX_NULL {
                            break;
                        }
                    }
                    p = cond_ptr;
                    t = cur_if as small_number;
                    l = if_line;
                    m = if_limit;
                    loop {
                        print_nl_cstr(b"### level \0".as_ptr() as *const c_char);
                        print_int(n);
                        print_cstr(b": \0".as_ptr() as *const c_char);
                        print_cmd_chr(IF_TEST as u16, t as i32);
                        if m == FI_CODE as u8 {
                            print_esc_cstr(b"else\0".as_ptr() as *const c_char);
                        }
                        if l != 0 {
                            print_cstr(b" entered on line \0".as_ptr() as *const c_char);
                            print_int(l);
                        }
                        n -= 1;
                        t = mem!(p).b16.s0 as small_number;
                        l = mem!(p + 1).b32.s1;
                        m = mem!(p).b16.s1 as u8;
                        p = LLIST_link!(p);
                        if p == TEX_NULL {
                            break;
                        }
                    }
                }
            }
            _ => {
                p = the_toks();
                print_nl_cstr(b"> \0".as_ptr() as *const c_char);
                token_show(TEMP_HEAD);
                flush_list(mem!(TEMP_HEAD).b32.s1);
                break 'common_ending;
            }
        }
        capture_to_diagnostic(ptr::null_mut());
        end_diagnostic(true);
        if (file_line_error_style_p) != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \0".as_ptr() as *const c_char);
        }
        print_cstr(b"OK\0".as_ptr() as *const c_char);
        if selector == (SELECTOR_TERM_AND_LOG as u8) as i32 {
            if INTPAR!(tracing_online) <= 0 {
                selector = (SELECTOR_TERM_ONLY as u8) as i32;
                print_cstr(b" (see the transcript file)\0".as_ptr() as *const c_char);
                selector = (SELECTOR_TERM_AND_LOG as u8) as i32;
            }
        }
        break 'common_ending;
    }

    capture_to_diagnostic(ptr::null_mut());
    if interaction < (ERROR_STOP_MODE) as u8 {
        help_ptr = 0;
        error_count -= 1;
    } else if INTPAR!(tracing_online) > 0 {
        help_ptr = 3;
        set_help_line!(
            2,
            b"This isn't an error message; I'm just \\showing something.\0"
        );
        set_help_line!(1, b"Type `I\\show...' to show more (e.g., \\show\\cs,\0");
        set_help_line!(0, b"\\showthe\\count10, \\showbox255, \\showlists).\0");
    } else {
        help_ptr = 5;
        set_help_line!(
            4,
            b"This isn't an error message; I'm just \\showing something.\0"
        );
        set_help_line!(3, b"Type `I\\show...' to show more (e.g., \\show\\cs,\0");
        set_help_line!(2, b"\\showthe\\count10, \\showbox255, \\showlists).\0");
        set_help_line!(
            1,
            b"And type `I\\tracingonline=1\\show...' to show boxes and\0"
        );
        set_help_line!(
            0,
            b"lists on your terminal as well as in the transcript file.\0"
        );
    }
    error();
}

// 104. new_write_whatsit
#[unsafe(export_name = "new_write_whatsit")]
pub unsafe extern "C" fn new_write_whatsit(w: small_number) {
    new_whatsit(cur_chr as small_number, w);
    if w != WRITE_NODE_SIZE as small_number {
        scan_four_bit_int();
    } else {
        scan_int();
        if cur_val < 0 {
            cur_val = 17;
        } else if cur_val > 15 && cur_val != 18 {
            cur_val = 16;
        }
    }
    mem!(cur_list.tail + 1).b32.s0 = cur_val;
}

// 105. scan_and_pack_name
#[unsafe(export_name = "scan_and_pack_name")]
pub unsafe extern "C" fn scan_and_pack_name() {
    scan_file_name();
    pack_file_name(cur_name, cur_area, cur_ext);
}

// 106. do_extension
#[unsafe(export_name = "do_extension")]
pub unsafe extern "C" fn do_extension() {
    let mut i: i32;
    let mut j: i32 = 0;
    let mut k: i32;
    let mut p: i32;

    match cur_chr {
        OPEN_NODE => {
            new_write_whatsit(OPEN_NODE_SIZE as small_number);
            scan_optional_equals();
            scan_file_name();
            mem!(cur_list.tail + 1).b32.s1 = cur_name;
            mem!(cur_list.tail + 2).b32.s0 = cur_area;
            mem!(cur_list.tail + 2).b32.s1 = cur_ext;
        }
        WRITE_NODE => {
            k = cur_cs;
            new_write_whatsit(WRITE_NODE_SIZE as small_number);
            cur_cs = k;
            p = scan_toks(false, false);
            mem!(cur_list.tail + 1).b32.s1 = def_ref;
        }
        CLOSE_NODE => {
            new_write_whatsit(WRITE_NODE_SIZE as small_number);
            mem!(cur_list.tail + 1).b32.s1 = TEX_NULL;
        }
        SPECIAL_NODE => {
            new_whatsit(
                SPECIAL_NODE as small_number,
                WRITE_NODE_SIZE as small_number,
            );
            mem!(cur_list.tail + 1).b32.s0 = TEX_NULL;
            p = scan_toks(false, true);
            mem!(cur_list.tail + 1).b32.s1 = def_ref;
        }
        IMMEDIATE_CODE => {
            get_x_token();
            if (cur_cmd as i32) == EXTENSION && cur_chr <= CLOSE_NODE {
                p = cur_list.tail;
                do_extension();
                out_what(cur_list.tail);
                flush_node_list(cur_list.tail);
                cur_list.tail = p;
                mem!(p).b32.s1 = TEX_NULL;
            } else {
                back_input();
            }
        }
        SET_LANGUAGE_CODE => {
            if (cur_list.mode as i32).abs() != HMODE {
                report_illegal_case();
            } else {
                new_whatsit(
                    LANGUAGE_NODE as small_number,
                    SMALL_NODE_SIZE as small_number,
                );
                scan_int();
                if cur_val <= 0 || cur_val > 255 {
                    cur_list.aux.b32.s1 = 0;
                } else {
                    cur_list.aux.b32.s1 = cur_val;
                }
                mem!(cur_list.tail + 1).b32.s1 = cur_list.aux.b32.s1;
                mem!(cur_list.tail + 1).b16.s1 = (norm_min(INTPAR!(left_hyphen_min)) as u16) as u16;
                mem!(cur_list.tail + 1).b16.s0 =
                    (norm_min(INTPAR!(right_hyphen_min)) as u16) as u16;
            }
        }
        PDF_SAVE_POS_NODE => {
            new_whatsit(
                PDF_SAVE_POS_NODE as small_number,
                SMALL_NODE_SIZE as small_number,
            );
        }
        RESET_TIMER_CODE => {
            get_seconds_and_micros(&raw mut epochseconds, &raw mut microseconds);
        }
        SET_RANDOM_SEED_CODE => {
            scan_int();
            if cur_val < 0 {
                cur_val = -cur_val;
            }
            random_seed = cur_val;
            init_randoms(random_seed);
        }
        PIC_FILE_CODE => {
            if (cur_list.mode as i32).abs() == MMODE {
                report_illegal_case();
            } else {
                load_picture(false);
            }
        }
        PDF_FILE_CODE => {
            if (cur_list.mode as i32).abs() == MMODE {
                report_illegal_case();
            } else {
                load_picture(true);
            }
        }
        GLYPH_CODE => {
            if (cur_list.mode as i32).abs() == VMODE {
                back_input();
                new_graf(true);
            } else if (cur_list.mode as i32).abs() == MMODE {
                report_illegal_case();
            } else {
                let local_cur_f = eqtb!(CUR_FONT_LOC).b32.s1;
                if *font_area.offset(local_cur_f as isize) == (AAT_FONT_FLAG) as i32
                    || *font_area.offset(local_cur_f as isize) == (OTGR_FONT_FLAG) as i32
                {
                    new_whatsit(GLYPH_NODE as small_number, GLYPH_NODE_SIZE as small_number);
                    scan_int();
                    if cur_val < 0 || cur_val > 65535 {
                        let errmsg = error_here_with_diagnostic(
                            b"Bad glyph number\0".as_ptr() as *const c_char
                        );
                        ttstub_diag_printf(errmsg, b" (%d)\0".as_ptr() as *const c_char, cur_val);
                        capture_to_diagnostic(ptr::null_mut());
                        help_ptr = 2;
                        set_help_line!(1, b"A glyph number must be between 0 and 65535.\0");
                        set_help_line!(0, b"I changed this one to zero.\0");
                        int_error(cur_val);
                        cur_val = 0;
                    }
                    mem!(cur_list.tail + 4).b16.s2 = (local_cur_f as u16) as u16;
                    mem!(cur_list.tail + 4).b16.s1 = (cur_val as u16) as u16;
                    set_native_glyph_metrics(
                        cur_list.tail,
                        (INTPAR!(xetex_use_glyph_metrics) > 0) as i32,
                    );
                } else {
                    not_native_font_error(EXTENSION as i32, GLYPH_CODE as i32, local_cur_f);
                }
            }
        }
        XETEX_INPUT_ENCODING_EXTENSION_CODE => {
            scan_and_pack_name();
            i = get_encoding_mode_and_info(&raw mut j);
            if i == XETEX_INPUT_MODE_AUTO {
                error_here_with_diagnostic(
                    b"Encoding mode `auto' is not valid for \\XeTeXinputencoding\0".as_ptr()
                        as *const c_char,
                );
                capture_to_diagnostic(ptr::null_mut());
                help_ptr = 2;
                set_help_line!(
                    1,
                    b"You can't use `auto' encoding here, only for \\XeTeXdefaultencoding.\0"
                );
                set_help_line!(
                    0,
                    b"I'll ignore this and leave the current encoding unchanged.\0"
                );
                error();
            } else {
                set_input_file_encoding(
                    (*input_file.offset((in_open as usize) as isize)),
                    (i as i8) as i32,
                    j,
                );
            }
        }
        XETEX_DEFAULT_ENCODING_EXTENSION_CODE => {
            scan_and_pack_name();
            i = get_encoding_mode_and_info(&raw mut j);
            INTPAR!(xetex_default_input_mode) = i;
            INTPAR!(xetex_default_input_encoding) = j;
        }
        XETEX_LINEBREAK_LOCALE_EXTENSION_CODE => {
            scan_file_name();
            if length(cur_name) == 0 {
                INTPAR!(xetex_linebreak_locale) = 0;
            } else {
                INTPAR!(xetex_linebreak_locale) = cur_name;
            }
        }
        _ => {
            confusion(b"ext1\0".as_ptr() as *const c_char);
        }
    }
}

// 107. fix_language
#[unsafe(export_name = "fix_language")]
pub unsafe extern "C" fn fix_language() {
    let mut l: UTF16_code;

    if INTPAR!(language) <= 0 || INTPAR!(language) > 255 {
        l = 0;
    } else {
        l = INTPAR!(language) as UTF16_code;
    }
    if l as i32 != cur_list.aux.b32.s1 {
        new_whatsit(
            LANGUAGE_NODE as small_number,
            SMALL_NODE_SIZE as small_number,
        );
        mem!(cur_list.tail + 1).b32.s1 = l as i32;
        cur_list.aux.b32.s1 = l as i32;
        mem!(cur_list.tail + 1).b16.s1 = (norm_min(INTPAR!(left_hyphen_min)) as u16) as u16;
        mem!(cur_list.tail + 1).b16.s0 = (norm_min(INTPAR!(right_hyphen_min)) as u16) as u16;
    }
}

// 108. insert_src_special
#[unsafe(export_name = "insert_src_special")]
pub unsafe extern "C" fn insert_src_special() {
    let mut toklist: i32;
    let mut p: i32;
    let mut q: i32;

    if *source_filename_stack.offset(in_open as isize) > 0
        && is_new_source(*source_filename_stack.offset(in_open as isize), line)
    {
        toklist = get_avail();
        p = toklist;
        mem!(p).b32.s0 = CS_TOKEN_FLAG + FROZEN_SPECIAL;
        mem!(p).b32.s1 = get_avail();
        p = LLIST_link!(p);
        mem!(p).b32.s0 = LEFT_BRACE_TOKEN + (b'{' as i32);
        q = str_toks(make_src_special(
            *source_filename_stack.offset(in_open as isize),
            line,
        ));
        mem!(p).b32.s1 = mem!(TEMP_HEAD).b32.s1;
        p = q;
        mem!(p).b32.s1 = get_avail();
        p = LLIST_link!(p);
        mem!(p).b32.s0 = RIGHT_BRACE_TOKEN + (b'}' as i32);
        begin_token_list(toklist, INSERTED as u16);
        remember_source_info(*source_filename_stack.offset(in_open as isize), line);
    }
}

// 109. append_src_special
#[unsafe(export_name = "append_src_special")]
pub unsafe extern "C" fn append_src_special() {
    if *source_filename_stack.offset(in_open as isize) > 0
        && is_new_source(*source_filename_stack.offset(in_open as isize), line)
    {
        new_whatsit(
            SPECIAL_NODE as small_number,
            WRITE_NODE_SIZE as small_number,
        );
        mem!(cur_list.tail + 1).b32.s0 = 0;
        def_ref = get_avail();
        mem!(def_ref).b32.s0 = TEX_NULL;
        str_toks(make_src_special(
            *source_filename_stack.offset(in_open as isize),
            line,
        ));
        mem!(def_ref).b32.s1 = mem!(TEMP_HEAD).b32.s1;
        mem!(cur_list.tail + 1).b32.s1 = def_ref;
        remember_source_info(*source_filename_stack.offset(in_open as isize), line);
    }
}

// 110. tt_insert_special
#[unsafe(export_name = "tt_insert_special")]
pub unsafe extern "C" fn tt_insert_special(mut ascii_text: *const c_char) {
    let mut toklist_start: i32;
    let mut p: i32;
    let mut q: i32;
    let start_pool_ptr: pool_pointer = pool_ptr;

    if (pool_ptr as usize) + strlen(ascii_text) >= pool_size as usize {
        _tt_abort(b"string pool overflow\0".as_ptr() as *const c_char);
    }

    while *ascii_text != 0 {
        *str_pool.offset(pool_ptr as isize) = *ascii_text as u8 as packed_UTF16_code;
        pool_ptr += 1;
        ascii_text = ascii_text.offset(1);
    }

    toklist_start = get_avail();
    p = toklist_start;
    LLIST_info!(p) = CS_TOKEN_FLAG + FROZEN_SPECIAL;

    q = get_avail();
    LLIST_info!(q) = LEFT_BRACE_TOKEN + (b'{' as i32);
    LLIST_link!(p) = q;

    p = str_toks(start_pool_ptr);
    LLIST_link!(q) = LLIST_link!(TEMP_HEAD);

    q = get_avail();
    LLIST_info!(q) = RIGHT_BRACE_TOKEN + (b'}' as i32);
    LLIST_link!(p) = q;

    begin_token_list(toklist_start, INSERTED as u16);
}

// 111. handle_right_brace
#[unsafe(export_name = "handle_right_brace")]
pub unsafe extern "C" fn handle_right_brace() {
    let mut p: i32;
    let mut q: i32;
    let mut d: scaled_t;
    let mut f: i32;

    match cur_group as i32 {
        SIMPLE_GROUP => {
            unsave();
        }
        BOTTOM_LEVEL => {
            error_here_with_diagnostic(b"Too many }'s\0".as_ptr() as *const c_char);
            capture_to_diagnostic(ptr::null_mut());
            help_ptr = 2;
            set_help_line!(1, b"You've closed more groups than you opened.\0");
            set_help_line!(0, b"Such booboos are generally harmless, so keep going.\0");
            error();
        }
        SEMI_SIMPLE_GROUP | MATH_SHIFT_GROUP | MATH_LEFT_GROUP => {
            extra_right_brace();
        }
        HBOX_GROUP => {
            package(0);
        }
        ADJUSTED_HBOX_GROUP => {
            adjust_tail = ADJUST_HEAD;
            pre_adjust_tail = PRE_ADJUST_HEAD;
            package(0);
        }
        VBOX_GROUP => {
            end_graf();
            package(0);
        }
        VTOP_GROUP => {
            end_graf();
            package((VTOP_CODE) as i16);
        }
        INSERT_GROUP => {
            end_graf();
            q = GLUEPAR!(split_top_skip);
            GLUE_SPEC_ref_count!(q) += 1;
            d = DIMENPAR!(split_max_depth);
            f = INTPAR!(floating_penalty);
            unsave();
            save_ptr = save_ptr - 2;
            p = vpackage(
                mem!(cur_list.head).b32.s1,
                0,
                (ADDITIONAL) as i16,
                MAX_HALFWORD,
            );
            pop_nest();

            if (*save_stack.offset((save_ptr as usize + 0) as isize))
                .b32
                .s1
                < 255
            {
                mem!(cur_list.tail).b32.s1 = get_node(INS_NODE_SIZE as i32);
                cur_list.tail = LLIST_link!(cur_list.tail);
                NODE_type!(cur_list.tail) = INS_NODE as u16;
                mem!(cur_list.tail).b16.s0 = ((*save_stack.offset((save_ptr as usize + 0) as isize))
                    .b32
                    .s1 as u16) as u16;
                mem!(cur_list.tail + 3).b32.s1 = mem!(p + 3).b32.s1 + mem!(p + 2).b32.s1;
                mem!(cur_list.tail + 4).b32.s0 = mem!(p + 5).b32.s1;
                mem!(cur_list.tail + 4).b32.s1 = q;
                mem!(cur_list.tail + 2).b32.s1 = d;
                mem!(cur_list.tail + 1).b32.s1 = f;
            } else {
                mem!(cur_list.tail).b32.s1 = get_node(SMALL_NODE_SIZE as i32);
                cur_list.tail = LLIST_link!(cur_list.tail);
                NODE_type!(cur_list.tail) = ADJUST_NODE as u16;
                mem!(cur_list.tail).b16.s0 = ((*save_stack.offset((save_ptr as usize + 1) as isize))
                    .b32
                    .s1 as u16) as u16;
                mem!(cur_list.tail + 1).b32.s1 = mem!(p + 5).b32.s1;
                delete_glue_ref(q);
            }

            free_node(p, BOX_NODE_SIZE as i32);
            if nest_ptr == 0 {
                build_page();
            }
        }
        OUTPUT_GROUP => {
            if cur_input.loc != TEX_NULL
                || (cur_input.index != (OUTPUT_TEXT) as u16
                    && cur_input.index != (BACKED_UP) as u16)
            {
                error_here_with_diagnostic(b"Unbalanced output routine\0".as_ptr() as *const c_char);
                capture_to_diagnostic(ptr::null_mut());
                help_ptr = 2;
                set_help_line!(
                    1,
                    b"Your sneaky output routine has problematic {'s and/or }'s.\0"
                );
                set_help_line!(0, b"I can't handle that very well; good luck.\0");
                error();

                loop {
                    get_token();
                    if cur_input.loc == TEX_NULL {
                        break;
                    }
                }
            }

            end_token_list();
            end_graf();
            unsave();
            output_active = false;
            insert_penalties = 0;

            if BOX_REG!(255) != TEX_NULL {
                error_here_with_diagnostic(
                    b"Output routine didn't use all of \0".as_ptr() as *const c_char
                );
                print_esc_cstr(b"box\0".as_ptr() as *const c_char);
                print_int(255);
                capture_to_diagnostic(ptr::null_mut());

                help_ptr = 3;
                set_help_line!(2, b"Your \\output commands should empty \\box255,\0");
                set_help_line!(1, b"e.g., by saying `\\shipout\\box255'.\0");
                set_help_line!(0, b"Proceed; I'll discard its present contents.\0");
                box_error(255);
            }

            if cur_list.tail != cur_list.head {
                mem!(page_tail).b32.s1 = mem!(cur_list.head).b32.s1;
                page_tail = cur_list.tail;
            }

            if mem!(PAGE_HEAD).b32.s1 != TEX_NULL {
                if mem!(CONTRIB_HEAD).b32.s1 == TEX_NULL {
                    (*nest.offset(0)).tail = page_tail;
                }
                mem!(page_tail).b32.s1 = mem!(CONTRIB_HEAD).b32.s1;
                mem!(CONTRIB_HEAD).b32.s1 = mem!(PAGE_HEAD).b32.s1;
                mem!(PAGE_HEAD).b32.s1 = TEX_NULL;
                page_tail = PAGE_HEAD;
            }

            flush_node_list(disc_ptr[LAST_BOX_CODE as usize]);
            disc_ptr[LAST_BOX_CODE as usize] = TEX_NULL;
            pop_nest();
            build_page();
        }
        DISC_GROUP => {
            build_discretionary();
        }
        ALIGN_GROUP => {
            back_input();
            cur_tok = CS_TOKEN_FLAG + FROZEN_CR;

            error_here_with_diagnostic(b"Missing \0".as_ptr() as *const c_char);
            print_esc_cstr(b"cr\0".as_ptr() as *const c_char);
            print_cstr(b" inserted\0".as_ptr() as *const c_char);
            capture_to_diagnostic(ptr::null_mut());

            help_ptr = 1;
            set_help_line!(
                0,
                b"I'm guessing that you meant to end an alignment here.\0"
            );
            ins_error();
        }
        NO_ALIGN_GROUP => {
            end_graf();
            unsave();
            align_peek();
        }
        VCENTER_GROUP => {
            end_graf();
            unsave();
            save_ptr = save_ptr - 2;
            p = vpackage(
                mem!(cur_list.head).b32.s1,
                (*save_stack.offset((save_ptr as usize + 1) as isize))
                    .b32
                    .s1,
                ((*save_stack.offset((save_ptr as usize + 0) as isize))
                    .b32
                    .s1) as i16,
                MAX_HALFWORD,
            );
            pop_nest();
            mem!(cur_list.tail).b32.s1 = new_noad();
            cur_list.tail = LLIST_link!(cur_list.tail);
            mem!(cur_list.tail).b16.s1 = (VCENTER_NOAD as u16) as u16;
            mem!(cur_list.tail + 1).b32.s1 = SUB_BOX;
            mem!(cur_list.tail + 1).b32.s0 = p;
        }
        MATH_CHOICE_GROUP => {
            build_choices();
        }
        MATH_GROUP => {
            unsave();
            save_ptr -= 1;
            mem!(
                (*save_stack.offset((save_ptr as usize + 0) as isize))
                    .b32
                    .s1
            )
            .b32
            .s1 = SUB_MLIST;
            p = fin_mlist(TEX_NULL);
            mem!(
                (*save_stack.offset((save_ptr as usize + 0) as isize))
                    .b32
                    .s1
            )
            .b32
            .s0 = p;

            if p != TEX_NULL {
                if mem!(p).b32.s1 == TEX_NULL {
                    if mem!(p).b16.s1 == (ORD_NOAD as u16) as u16 {
                        if mem!(p + 3).b32.s1 == EMPTY {
                            if mem!(p + 2).b32.s1 == EMPTY {
                                mem!(
                                    (*save_stack.offset((save_ptr as usize + 0) as isize))
                                        .b32
                                        .s1
                                )
                                .b32 = mem!(p + 1).b32;
                                free_node(p, NOAD_SIZE as i32);
                            }
                        }
                    } else if mem!(p).b16.s1 == (ACCENT_NOAD as u16) as u16 {
                        if (*save_stack.offset((save_ptr as usize + 0) as isize))
                            .b32
                            .s1
                            == cur_list.tail + 1
                        {
                            if mem!(cur_list.tail).b16.s1 == (ORD_NOAD as u16) as u16 {
                                q = cur_list.head;
                                while mem!(q).b32.s1 != cur_list.tail {
                                    q = LLIST_link!(q);
                                }
                                mem!(q).b32.s1 = p;
                                free_node(cur_list.tail, NOAD_SIZE as i32);
                                cur_list.tail = p;
                            }
                        }
                    }
                }
            }
        }
        _ => {
            confusion(b"rightbrace\0".as_ptr() as *const c_char);
        }
    }
}

// 112. main_control
#[unsafe(export_name = "main_control")]
pub unsafe extern "C" fn main_control() {
    let mut t: i32;

    if !tt_checkpoint_resumed && LOCAL!(LOCAL__every_job) != TEX_NULL {
        begin_token_list(LOCAL!(LOCAL__every_job), EVERY_JOB_TEXT as u16);
    }
    if tt_checkpoint_resumed {
        tt_checkpoint_continue();
    }

    let mut state = MainControlState::BigSwitch;
    'control: loop {
        match state {
            MainControlState::BigSwitch => {
                if tt_checkpoint_enabled {
                    tt_checkpoint_try();
                    tt_checkpoint_dispatch_fetch = true;
                }
                get_x_token();
                if tt_checkpoint_enabled {
                    tt_checkpoint_dispatch_fetch = false;
                }
                state = MainControlState::Reswitch;
            }
            MainControlState::Reswitch => {
                if INTPAR!(tracing_commands) > 0 {
                    show_cur_cmd_chr();
                }

                let mode = (cur_list.mode as i32).abs();
                let cmd = cur_cmd as i32;

                match (mode, cmd) {
                    (HMODE, LETTER) | (HMODE, OTHER_CHAR) | (HMODE, CHAR_GIVEN) => {
                        state = MainControlState::MainLoop;
                        continue 'control;
                    }
                    (HMODE, CHAR_NUM) => {
                        scan_usv_num();
                        cur_chr = cur_val;
                        state = MainControlState::MainLoop;
                        continue 'control;
                    }
                    (HMODE, NO_BOUNDARY) => {
                        get_x_token();
                        if (cur_cmd as i32) == LETTER
                            || (cur_cmd as i32) == OTHER_CHAR
                            || (cur_cmd as i32) == CHAR_GIVEN
                            || (cur_cmd as i32) == CHAR_NUM
                        {
                            cancel_boundary = true;
                        }
                        state = MainControlState::Reswitch;
                        continue 'control;
                    }
                    _ => {}
                }

                if mode == HMODE {
                    if INTPAR!(xetex_inter_char_tokens) > 0
                        && space_class != CHAR_CLASS_LIMIT
                        && prev_class != CHAR_CLASS_LIMIT - 1
                    {
                        prev_class = CHAR_CLASS_LIMIT - 1;
                        find_sa_element(
                            INTER_CHAR_VAL as small_number,
                            space_class * CHAR_CLASS_LIMIT + CHAR_CLASS_LIMIT - 1,
                            false,
                        );
                        if cur_ptr != TEX_NULL && ETEX_SA_ptr!(cur_ptr) != TEX_NULL {
                            if cur_cs == 0 {
                                if (cur_cmd as i32) == CHAR_NUM {
                                    cur_cmd = OTHER_CHAR as u8;
                                }
                                cur_tok = (cur_cmd as i32 * MAX_CHAR_VAL) + cur_chr;
                            } else {
                                cur_tok = CS_TOKEN_FLAG + cur_cs;
                            }
                            back_input();
                            begin_token_list(ETEX_SA_ptr!(cur_ptr), INTER_CHAR_TEXT as u16);
                            state = MainControlState::BigSwitch;
                            continue 'control;
                        }
                    }
                }

                match (mode, cmd) {
                    (HMODE, SPACER) => {
                        if (cur_list.aux.b32.s0 == 1000) {
                            state = MainControlState::AppendNormalSpace;
                            continue 'control;
                        } else {
                            app_space();
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (HMODE, EX_SPACE) | (MMODE, EX_SPACE) => {
                        state = MainControlState::AppendNormalSpace;
                        continue 'control;
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, RELAX) | (VMODE, SPACER) | (MMODE, SPACER) | (MMODE, NO_BOUNDARY) => {
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, IGNORE_SPACES) => {
                        if (cur_chr == 0) {
                            loop {
                                get_x_token();
                                if !(cur_cmd == (SPACER) as u8) {
                                    break;
                                }
                            }
                        } else {
                            t = (scanner_status) as i32;
                            scanner_status = (NORMAL) as u8;
                            get_next();
                            scanner_status = (t) as u8;

                            if (cur_cs < HASH_BASE) {
                                cur_cs = prim_lookup(cur_cs - SINGLE_BASE);
                            } else {
                                cur_cs = prim_lookup((*hash.offset((cur_cs) as isize)).s1);
                            }

                            if (cur_cs != UNDEFINED_PRIMITIVE) {
                                cur_cmd = (eqtb!(PRIM_EQTB_BASE + cur_cs).b16.s1) as u8;
                                cur_chr = eqtb!(PRIM_EQTB_BASE + cur_cs).b32.s1;
                                cur_tok = CS_TOKEN_FLAG + PRIM_EQTB_BASE + cur_cs;
                            }
                        }
                        state = MainControlState::Reswitch;
                        continue 'control;
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE, STOP) => {
                        if (its_all_over()) {
                            return;
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE, VMOVE)
                    | (HMODE, HMOVE)
                    | (MMODE, HMOVE)
                    | (_, LAST_ITEM)
                    | (VMODE, VADJUST)
                    | (VMODE, ITAL_CORR)
                    | (VMODE | HMODE, EQ_NO)
                    | (_, MAC_PARAM) => {
                        report_illegal_case();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE | HMODE, SUP_MARK)
                    | (VMODE | HMODE, SUB_MARK)
                    | (VMODE | HMODE, MATH_CHAR_NUM)
                    | (VMODE | HMODE, MATH_GIVEN)
                    | (VMODE | HMODE, XETEX_MATH_GIVEN)
                    | (VMODE | HMODE, MATH_COMP)
                    | (VMODE | HMODE, DELIM_NUM)
                    | (VMODE | HMODE, LEFT_RIGHT)
                    | (VMODE | HMODE, ABOVE)
                    | (VMODE | HMODE, RADICAL)
                    | (VMODE | HMODE, MATH_STYLE)
                    | (VMODE | HMODE, MATH_CHOICE)
                    | (VMODE | HMODE, VCENTER)
                    | (VMODE | HMODE, NON_SCRIPT)
                    | (VMODE | HMODE, MKERN)
                    | (VMODE | HMODE, LIMIT_SWITCH)
                    | (VMODE | HMODE, MSKIP)
                    | (VMODE | HMODE, MATH_ACCENT)
                    | (MMODE, ENDV)
                    | (MMODE, PAR_END)
                    | (MMODE, STOP)
                    | (MMODE, VSKIP)
                    | (MMODE, UN_VBOX)
                    | (MMODE, VALIGN)
                    | (MMODE, HRULE) => {
                        insert_dollar_sign();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE, HRULE) | (HMODE, VRULE) | (MMODE, VRULE) => {
                        mem!(cur_list.tail).b32.s1 = scan_rule_spec();
                        cur_list.tail = LLIST_link!(cur_list.tail);

                        if (abs((cur_list.mode) as i32) == VMODE) {
                            cur_list.aux.b32.s1 = IGNORE_DEPTH;
                        } else if (abs((cur_list.mode) as i32) == HMODE) {
                            cur_list.aux.b32.s0 = 1000;
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE, VSKIP) | (HMODE, HSKIP) | (MMODE, HSKIP) | (MMODE, MSKIP) => {
                        append_glue();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, KERN) | (MMODE, MKERN) => {
                        append_kern();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE | HMODE, LEFT_BRACE) => {
                        new_save_level((SIMPLE_GROUP) as u8);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, BEGIN_GROUP) => {
                        new_save_level((SEMI_SIMPLE_GROUP) as u8);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, END_GROUP) => {
                        if (cur_group == (SEMI_SIMPLE_GROUP) as u8) {
                            unsave();
                        } else {
                            off_save();
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, RIGHT_BRACE) => {
                        handle_right_brace();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE, HMOVE) | (HMODE, VMOVE) | (MMODE, VMOVE) => {
                        t = cur_chr;
                        scan_dimen(false, false, false);
                        if (t == 0) {
                            scan_box(cur_val);
                        } else {
                            scan_box(-cur_val);
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, LEADER_SHIP) => {
                        scan_box(LEADER_FLAG - A_LEADERS + cur_chr);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, MAKE_BOX) => {
                        begin_box(0);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE, START_PAR) => {
                        new_graf(cur_chr > 0);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE, LETTER)
                    | (VMODE, OTHER_CHAR)
                    | (VMODE, CHAR_NUM)
                    | (VMODE, CHAR_GIVEN)
                    | (VMODE, MATH_SHIFT)
                    | (VMODE, UN_HBOX)
                    | (VMODE, VRULE)
                    | (VMODE, ACCENT)
                    | (VMODE, DISCRETIONARY)
                    | (VMODE, HSKIP)
                    | (VMODE, VALIGN)
                    | (VMODE, EX_SPACE)
                    | (VMODE, NO_BOUNDARY) => {
                        back_input();
                        new_graf(true);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (HMODE, START_PAR) | (MMODE, START_PAR) => {
                        indent_in_hmode();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE, PAR_END) => {
                        normal_paragraph();
                        if (cur_list.mode > 0) {
                            build_page();
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (HMODE, PAR_END) => {
                        if (align_state < 0) {
                            off_save();
                        }
                        end_graf();
                        if (cur_list.mode == (VMODE) as i16) {
                            build_page();
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (HMODE, STOP)
                    | (HMODE, VSKIP)
                    | (HMODE, HRULE)
                    | (HMODE, UN_VBOX)
                    | (HMODE, HALIGN) => {
                        head_for_vmode();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, INSERT) | (HMODE, VADJUST) | (MMODE, VADJUST) => {
                        begin_insert_or_adjust();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, MARK) => {
                        make_mark();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, BREAK_PENALTY) => {
                        append_penalty();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, REMOVE_ITEM) => {
                        delete_last();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE, UN_VBOX) | (HMODE, UN_HBOX) | (MMODE, UN_HBOX) => {
                        unpackage();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (HMODE, ITAL_CORR) => {
                        append_italic_correction();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, ITAL_CORR) => {
                        mem!(cur_list.tail).b32.s1 = new_kern(0);
                        cur_list.tail = LLIST_link!(cur_list.tail);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (HMODE, DISCRETIONARY) | (MMODE, DISCRETIONARY) => {
                        append_discretionary();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (HMODE, ACCENT) => {
                        make_accent();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, CAR_RET) | (_, TAB_MARK) => {
                        align_error();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, NO_ALIGN) => {
                        no_align_error();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, OMIT) => {
                        omit_error();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE, HALIGN) => {
                        init_align();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (HMODE, VALIGN) => {
                        if (cur_chr > 0) {
                            if (eTeX_enabled(INTPAR!(texxet) > 0, (cur_cmd) as u16, cur_chr)) {
                                mem!(cur_list.tail).b32.s1 = new_math(0, (cur_chr) as i16);
                                cur_list.tail = LLIST_link!(cur_list.tail);
                            }
                        } else {
                            init_align();
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, HALIGN) => {
                        if (privileged()) {
                            if (cur_group == (MATH_SHIFT_GROUP) as u8) {
                                init_align();
                            } else {
                                off_save();
                            }
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (VMODE, ENDV) | (HMODE, ENDV) => {
                        do_endv();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, END_CS_NAME) => {
                        cs_error();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (HMODE, MATH_SHIFT) => {
                        init_math();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, EQ_NO) => {
                        if (privileged()) {
                            if (cur_group == (MATH_SHIFT_GROUP) as u8) {
                                start_eq_no();
                            } else {
                                off_save();
                            }
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, LEFT_BRACE) => {
                        mem!(cur_list.tail).b32.s1 = new_noad();
                        cur_list.tail = LLIST_link!(cur_list.tail);
                        back_input();
                        scan_math(cur_list.tail + 1);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, LETTER) | (MMODE, OTHER_CHAR) | (MMODE, CHAR_GIVEN) => {
                        set_math_char(MATH_CODE!(cur_chr));
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, CHAR_NUM) => {
                        scan_char_num();
                        cur_chr = cur_val;
                        set_math_char(MATH_CODE!(cur_chr));
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, MATH_CHAR_NUM) => {
                        if (cur_chr == 2) {
                            scan_math_class_int();
                            t = super::xetex_math::set_class(cur_val);
                            scan_math_fam_int();
                            t = t + super::xetex_math::set_family(cur_val);
                            scan_usv_num();
                            t = t + cur_val;
                            set_math_char(t);
                        } else if (cur_chr == 1) {
                            scan_xetex_math_char_int();
                            set_math_char(cur_val);
                        } else {
                            scan_fifteen_bit_int();
                            set_math_char(
                                super::xetex_math::set_class(cur_val / 4096)
                                    + super::xetex_math::set_family((cur_val % 4096) / 256)
                                    + (cur_val % 256),
                            );
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, MATH_GIVEN) => {
                        set_math_char(
                            super::xetex_math::set_class(cur_chr / 4096)
                                + super::xetex_math::set_family((cur_chr % 4096) / 256)
                                + (cur_chr % 256),
                        );
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, XETEX_MATH_GIVEN) => {
                        set_math_char(cur_chr);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, DELIM_NUM) => {
                        if (cur_chr == 1) {
                            scan_math_class_int();
                            t = super::xetex_math::set_class(cur_val);
                            scan_math_fam_int();
                            t = t + super::xetex_math::set_family(cur_val);
                            scan_usv_num();
                            t = t + cur_val;
                            set_math_char(t);
                        } else {
                            scan_delimiter_int();
                            cur_val = cur_val / 4096;
                            set_math_char(
                                super::xetex_math::set_class(cur_val / 4096)
                                    + super::xetex_math::set_family((cur_val % 4096) / 256)
                                    + (cur_val % 256),
                            );
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, MATH_COMP) => {
                        mem!(cur_list.tail).b32.s1 = new_noad();
                        cur_list.tail = LLIST_link!(cur_list.tail);
                        mem!(cur_list.tail).b16.s1 = (cur_chr) as u16;
                        scan_math(cur_list.tail + 1);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, LIMIT_SWITCH) => {
                        math_limit_switch();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, RADICAL) => {
                        math_radical();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, ACCENT) | (MMODE, MATH_ACCENT) => {
                        math_ac();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, VCENTER) => {
                        scan_spec((VCENTER_GROUP) as u8, false);
                        normal_paragraph();
                        push_nest();
                        cur_list.mode = -1;
                        cur_list.aux.b32.s1 = IGNORE_DEPTH;
                        if (insert_src_special_every_vbox) {
                            insert_src_special();
                        }
                        if (LOCAL!(LOCAL__every_vbox) != TEX_NULL) {
                            begin_token_list(LOCAL!(LOCAL__every_vbox), (EVERY_VBOX_TEXT) as u16);
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, MATH_STYLE) => {
                        mem!(cur_list.tail).b32.s1 = new_style((cur_chr) as i16);
                        cur_list.tail = LLIST_link!(cur_list.tail);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, NON_SCRIPT) => {
                        mem!(cur_list.tail).b32.s1 = new_glue(0);
                        cur_list.tail = LLIST_link!(cur_list.tail);
                        mem!(cur_list.tail).b16.s0 = (COND_MATH_GLUE) as u16;
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, MATH_CHOICE) => {
                        append_choices();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, SUB_MARK) | (MMODE, SUP_MARK) => {
                        sub_sup();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, ABOVE) => {
                        math_fraction();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, LEFT_RIGHT) => {
                        math_left_right();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, MATH_SHIFT) => {
                        if (cur_group == (MATH_SHIFT_GROUP) as u8) {
                            after_math();
                        } else {
                            off_save();
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, TOKS_REGISTER)
                    | (_, ASSIGN_TOKS)
                    | (_, ASSIGN_INT)
                    | (_, ASSIGN_DIMEN)
                    | (_, ASSIGN_GLUE)
                    | (_, ASSIGN_MU_GLUE)
                    | (_, ASSIGN_FONT_DIMEN)
                    | (_, ASSIGN_FONT_INT)
                    | (_, SET_AUX)
                    | (_, SET_PREV_GRAF)
                    | (_, SET_PAGE_DIMEN)
                    | (_, SET_PAGE_INT)
                    | (_, SET_BOX_DIMEN)
                    | (_, SET_SHAPE)
                    | (_, DEF_CODE)
                    | (_, XETEX_DEF_CODE)
                    | (_, DEF_FAMILY)
                    | (_, SET_FONT)
                    | (_, DEF_FONT)
                    | (_, REGISTER)
                    | (_, ADVANCE)
                    | (_, MULTIPLY)
                    | (_, DIVIDE)
                    | (_, PREFIX)
                    | (_, LET)
                    | (_, SHORTHAND_DEF)
                    | (_, READ_TO_CS)
                    | (_, DEF)
                    | (_, SET_BOX)
                    | (_, HYPH_DATA)
                    | (_, SET_INTERACTION) => {
                        prefixed_command();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, AFTER_ASSIGNMENT) => {
                        get_token();
                        after_token = cur_tok;
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, AFTER_GROUP) => {
                        get_token();
                        save_for_after(cur_tok);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, IN_STREAM) => {
                        open_or_close_in();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, MESSAGE) => {
                        issue_message();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, CASE_SHIFT) => {
                        shift_case();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, XRAY) => {
                        show_whatever();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (_, EXTENSION) => {
                        do_extension();
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    _ => {
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                }
                state = MainControlState::BigSwitch;
            }
            MainControlState::MainLoop => {
                if (cur_list.head == cur_list.tail && cur_list.mode > 0) {
                    if (insert_src_special_auto) {
                        append_src_special();
                    }
                }

                prev_class = CHAR_CLASS_LIMIT - 1;

                if (*font_area.offset((eqtb!(CUR_FONT_LOC).b32.s1) as isize)
                    == (AAT_FONT_FLAG) as i32
                    || *font_area.offset((eqtb!(CUR_FONT_LOC).b32.s1) as isize)
                        == (OTGR_FONT_FLAG) as i32)
                {
                    if (cur_list.mode > 0) {
                        if (INTPAR!(language) != cur_list.aux.b32.s1) {
                            fix_language();
                        }
                    }

                    main_h = 0;
                    main_f = eqtb!(CUR_FONT_LOC).b32.s1;
                    native_len = 0;

                    state = MainControlState::CollectNative;
                } else {
                    state = MainControlState::MainTraditional;
                }
            }
            MainControlState::CollectNative => {
                main_s = SF_CODE!(cur_chr) % 65536;

                if (main_s == 1000) {
                    cur_list.aux.b32.s0 = 1000;
                } else if (main_s < 1000) {
                    if (main_s > 0) {
                        cur_list.aux.b32.s0 = main_s;
                    }
                } else if (cur_list.aux.b32.s0 < 1000) {
                    cur_list.aux.b32.s0 = 1000;
                } else {
                    cur_list.aux.b32.s0 = main_s;
                }

                cur_ptr = TEX_NULL;
                space_class = SF_CODE!(cur_chr) / 65536;

                if (INTPAR!(xetex_inter_char_tokens) > 0 && space_class != CHAR_CLASS_LIMIT) {
                    if (prev_class == CHAR_CLASS_LIMIT - 1) {
                        if (cur_input.state != (TOKEN_LIST) as u16
                            || cur_input.index != (BACKED_UP_CHAR) as u16)
                        {
                            find_sa_element(
                                (INTER_CHAR_VAL) as i16,
                                (CHAR_CLASS_LIMIT - 1) * CHAR_CLASS_LIMIT + space_class,
                                false,
                            );
                            if (cur_ptr != TEX_NULL && ETEX_SA_ptr!(cur_ptr) != TEX_NULL) {
                                if (cur_cmd != (LETTER) as u8) {
                                    cur_cmd = (OTHER_CHAR) as u8;
                                }
                                cur_tok = i32::from(cur_cmd) * MAX_CHAR_VAL + cur_chr;
                                back_input();
                                cur_input.index = (BACKED_UP_CHAR) as u16;
                                begin_token_list(
                                    mem!(cur_ptr + 1).b32.s1,
                                    (INTER_CHAR_TEXT) as u16,
                                );
                                state = MainControlState::BigSwitch;
                                continue 'control;
                            }
                        }
                    } else {
                        find_sa_element(
                            (INTER_CHAR_VAL) as i16,
                            prev_class * CHAR_CLASS_LIMIT + space_class,
                            false,
                        );
                        if (cur_ptr != TEX_NULL && ETEX_SA_ptr!(cur_ptr) != TEX_NULL) {
                            if (cur_cmd != (LETTER) as u8) {
                                cur_cmd = (OTHER_CHAR) as u8;
                            }
                            cur_tok = i32::from(cur_cmd) * MAX_CHAR_VAL + cur_chr;
                            back_input();
                            cur_input.index = (BACKED_UP_CHAR) as u16;
                            begin_token_list(mem!(cur_ptr + 1).b32.s1, (INTER_CHAR_TEXT) as u16);
                            prev_class = (CHAR_CLASS_LIMIT - 1);
                            state = MainControlState::Collected;
                            continue 'control;
                        }
                    }
                    prev_class = space_class;
                }

                if (cur_chr > 65535) {
                    reserve_native_text(native_len + 2);

                    *native_text.offset((native_len) as isize) =
                        ((cur_chr - 65536) / 1024 + 0xD800) as u16;
                    {
                        let previous = native_len;
                        native_len += 1;
                        previous
                    };
                    *native_text.offset((native_len) as isize) =
                        ((cur_chr - 65536) % 1024 + 0xDC00) as u16;
                    {
                        let previous = native_len;
                        native_len += 1;
                        previous
                    };
                } else {
                    reserve_native_text(native_len + 1);

                    *native_text.offset((native_len) as isize) = (cur_chr) as u16;
                    {
                        let previous = native_len;
                        native_len += 1;
                        previous
                    };
                }

                is_hyph = (cur_chr == *hyphen_char.offset((main_f) as isize))
                    || ((INTPAR!(xetex_dash_break) > 0)
                        && ((cur_chr == 8212) || (cur_chr == 8211)));

                if ((main_h == 0) && is_hyph) {
                    main_h = native_len;
                }

                get_next();
                if (cur_cmd == (LETTER) as u8
                    || cur_cmd == (OTHER_CHAR) as u8
                    || cur_cmd == (CHAR_GIVEN) as u8)
                {
                    state = MainControlState::CollectNative;
                    continue 'control;
                }

                x_token();
                if (cur_cmd == (LETTER) as u8
                    || cur_cmd == (OTHER_CHAR) as u8
                    || cur_cmd == (CHAR_GIVEN) as u8)
                {
                    state = MainControlState::CollectNative;
                    continue 'control;
                }

                if (cur_cmd == (CHAR_NUM) as u8) {
                    scan_usv_num();
                    cur_chr = cur_val;
                    state = MainControlState::CollectNative;
                    continue 'control;
                }

                if ((INTPAR!(xetex_inter_char_tokens) > 0)
                    && (space_class != CHAR_CLASS_LIMIT)
                    && (prev_class != (CHAR_CLASS_LIMIT - 1)))
                {
                    prev_class = (CHAR_CLASS_LIMIT - 1);
                    find_sa_element(
                        (INTER_CHAR_VAL) as i16,
                        space_class * CHAR_CLASS_LIMIT + (CHAR_CLASS_LIMIT - 1),
                        false,
                    );
                    if (cur_ptr != TEX_NULL && ETEX_SA_ptr!(cur_ptr) != TEX_NULL) {
                        if (cur_cs == 0) {
                            if (cur_cmd == (CHAR_NUM) as u8) {
                                cur_cmd = (OTHER_CHAR) as u8;
                            }
                            cur_tok = i32::from(cur_cmd) * MAX_CHAR_VAL + cur_chr;
                        } else {
                            cur_tok = CS_TOKEN_FLAG + cur_cs;
                        }
                        back_input();
                        begin_token_list(mem!(cur_ptr + 1).b32.s1, (INTER_CHAR_TEXT) as u16);
                        state = MainControlState::Collected;
                        continue 'control;
                    }
                }

                state = MainControlState::Collected;
            }
            MainControlState::Collected => {
                if (*font_mapping.offset((main_f) as isize) != ptr::null_mut()) {
                    main_k = apply_mapping(
                        *font_mapping.offset((main_f) as isize),
                        native_text,
                        native_len,
                    );
                    native_len = 0;

                    reserve_native_text(native_len + main_k);

                    main_h = 0;

                    {
                        let mut for_end: i32;
                        main_p = 0;
                        for_end = main_k - 1;
                        if (main_p <= for_end) {
                            loop {
                                {
                                    *native_text.offset((native_len) as isize) =
                                        *mapped_text.offset((main_p) as isize);
                                    {
                                        let previous = native_len;
                                        native_len += 1;
                                        previous
                                    };
                                }
                                if ((main_h == 0)
                                    && ((*mapped_text.offset((main_p) as isize)
                                        == (*hyphen_char.offset((main_f) as isize)) as u16)
                                        || ((INTPAR!(xetex_dash_break) > 0)
                                            && ((*mapped_text.offset((main_p) as isize) == 8212)
                                                || (*mapped_text.offset((main_p) as isize)
                                                    == 8211)))))
                                {
                                    main_h = native_len;
                                }
                                if !({
                                    let previous = main_p;
                                    main_p += 1;
                                    previous
                                } < for_end)
                                {
                                    break;
                                }
                            }
                        }
                    }
                }

                if (INTPAR!(tracing_lost_chars) > 0) {
                    temp_ptr = 0;
                    while (temp_ptr < native_len) {
                        main_k = (*native_text.offset((temp_ptr) as isize)) as i32;
                        {
                            let previous = temp_ptr;
                            temp_ptr += 1;
                            previous
                        };
                        if ((main_k >= 0xD800) && (main_k < 0xDC00)) {
                            main_k = 65536 + (main_k - 0xD800) * 1024;
                            main_k =
                                main_k + (*native_text.offset((temp_ptr) as isize)) as i32 - 0xDC00;
                            {
                                let previous = temp_ptr;
                                temp_ptr += 1;
                                previous
                            };
                        }
                        if (map_char_to_glyph(main_f, main_k) == 0) {
                            char_warning(main_f, main_k);
                        }
                    }
                }

                main_k = native_len;
                main_pp = cur_list.tail;

                if (cur_list.mode == (HMODE) as i16) {
                    main_ppp = cur_list.head;

                    while (main_ppp != main_pp && mem!(main_ppp).b32.s1 != main_pp) {
                        if (!is_char_node(main_ppp) && NODE_type!(main_ppp) == (DISC_NODE) as u16) {
                            temp_ptr = main_ppp;
                            {
                                let mut for_end: i32;
                                main_p = 1;
                                for_end = (mem!(temp_ptr).b16.s0) as i32;
                                if (main_p <= for_end) {
                                    loop {
                                        main_ppp = LLIST_link!(main_ppp);
                                        if !({
                                            let previous = main_p;
                                            main_p += 1;
                                            previous
                                        } < for_end)
                                        {
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        if (main_ppp != main_pp) {
                            main_ppp = LLIST_link!(main_ppp);
                        }
                    }

                    temp_ptr = 0;

                    loop {
                        if (main_h == 0) {
                            main_h = main_k;
                        }

                        if (((main_pp) != TEX_NULL
                            && (!(is_char_node(main_pp)))
                            && (NODE_type!(main_pp) == (WHATSIT_NODE) as u16)
                            && ((mem!(main_pp).b16.s0 == (NATIVE_WORD_NODE) as u16)
                                || (mem!(main_pp).b16.s0 == (NATIVE_WORD_NODE_AT) as u16)))
                            && (mem!(main_pp + 4).b16.s2 == (main_f) as u16)
                            && (main_ppp != main_pp)
                            && (!(is_char_node(main_ppp)))
                            && (NODE_type(main_ppp) != (DISC_NODE) as i16))
                        {
                            main_k = main_h + (mem!(main_pp + 4).b16.s1) as i32;

                            reserve_native_text(native_len + main_k);

                            save_native_len = native_len;

                            {
                                let mut for_end: i32;
                                main_p = 0;
                                for_end = (mem!(main_pp + 4).b16.s1 - 1) as i32;
                                if (main_p <= for_end) {
                                    loop {
                                        *native_text.offset((native_len) as isize) =
                                            (*native_node_text(main_pp).offset((main_p) as isize));
                                        {
                                            let previous = native_len;
                                            native_len += 1;
                                            previous
                                        };
                                        if !({
                                            let previous = main_p;
                                            main_p += 1;
                                            previous
                                        } < for_end)
                                        {
                                            break;
                                        }
                                    }
                                }
                            }
                            {
                                let mut for_end: i32;
                                main_p = 0;
                                for_end = main_h - 1;
                                if (main_p <= for_end) {
                                    loop {
                                        *native_text.offset((native_len) as isize) =
                                            *native_text.offset((temp_ptr + main_p) as isize);
                                        {
                                            let previous = native_len;
                                            native_len += 1;
                                            previous
                                        };
                                        if !({
                                            let previous = main_p;
                                            main_p += 1;
                                            previous
                                        } < for_end)
                                        {
                                            break;
                                        }
                                    }
                                }
                            }

                            do_locale_linebreaks(save_native_len, main_k);
                            native_len = save_native_len;
                            main_k = native_len - main_h - temp_ptr;
                            temp_ptr = main_h;
                            main_h = 0;

                            while ((main_h < main_k)
                                && (*native_text.offset((temp_ptr + main_h) as isize)
                                    != (*hyphen_char.offset((main_f) as isize)) as u16)
                                && ((!(INTPAR!(xetex_dash_break) > 0))
                                    || ((*native_text.offset((temp_ptr + main_h) as isize)
                                        != 8212)
                                        && (*native_text.offset((temp_ptr + main_h) as isize)
                                            != 8211))))
                            {
                                {
                                    let previous = main_h;
                                    main_h += 1;
                                    previous
                                };
                            }

                            if (main_h < main_k) {
                                {
                                    let previous = main_h;
                                    main_h += 1;
                                    previous
                                };
                            }

                            mem!(main_ppp).b32.s1 = mem!(main_pp).b32.s1;
                            mem!(main_pp).b32.s1 = TEX_NULL;
                            flush_node_list(main_pp);
                            main_pp = cur_list.tail;

                            while (mem!(main_ppp).b32.s1 != main_pp) {
                                main_ppp = LLIST_link!(main_ppp);
                            }
                        } else {
                            do_locale_linebreaks(temp_ptr, main_h);
                            temp_ptr = temp_ptr + main_h;
                            main_k = main_k - main_h;
                            main_h = 0;
                            while ((main_h < main_k)
                                && (*native_text.offset((temp_ptr + main_h) as isize)
                                    != (*hyphen_char.offset((main_f) as isize)) as u16)
                                && ((!(INTPAR!(xetex_dash_break) > 0))
                                    || ((*native_text.offset((temp_ptr + main_h) as isize)
                                        != 8212)
                                        && (*native_text.offset((temp_ptr + main_h) as isize)
                                            != 8211))))
                            {
                                {
                                    let previous = main_h;
                                    main_h += 1;
                                    previous
                                };
                            }
                            if (main_h < main_k) {
                                {
                                    let previous = main_h;
                                    main_h += 1;
                                    previous
                                };
                            }
                        }

                        if ((main_k > 0) || is_hyph) {
                            mem!(cur_list.tail).b32.s1 = new_disc();
                            cur_list.tail = LLIST_link!(cur_list.tail);
                            main_pp = cur_list.tail;
                        }
                        if !(!(main_k == 0)) {
                            break;
                        }
                    }
                } else {
                    main_ppp = cur_list.head;

                    while (main_ppp != main_pp && mem!(main_ppp).b32.s1 != main_pp) {
                        if (!is_char_node(main_ppp) && NODE_type!(main_ppp) == (DISC_NODE) as u16) {
                            temp_ptr = main_ppp;
                            {
                                let mut for_end: i32;
                                main_p = 1;
                                for_end = (mem!(temp_ptr).b16.s0) as i32;
                                if (main_p <= for_end) {
                                    loop {
                                        main_ppp = LLIST_link!(main_ppp);
                                        if !({
                                            let previous = main_p;
                                            main_p += 1;
                                            previous
                                        } < for_end)
                                        {
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        if (main_ppp != main_pp) {
                            main_ppp = LLIST_link!(main_ppp);
                        }
                    }

                    if (((main_pp) != TEX_NULL
                        && (!(is_char_node(main_pp)))
                        && (NODE_type!(main_pp) == (WHATSIT_NODE) as u16)
                        && ((mem!(main_pp).b16.s0 == (NATIVE_WORD_NODE) as u16)
                            || (mem!(main_pp).b16.s0 == (NATIVE_WORD_NODE_AT) as u16)))
                        && (mem!(main_pp + 4).b16.s2 == (main_f) as u16)
                        && (main_ppp != main_pp)
                        && (!(is_char_node(main_ppp)))
                        && (NODE_type(main_ppp) != (DISC_NODE) as i16))
                    {
                        mem!(main_pp).b32.s1 = new_native_word_node(
                            main_f,
                            main_k + (mem!(main_pp + 4).b16.s1) as i32,
                        );
                        cur_list.tail = mem!(main_pp).b32.s1;
                        {
                            let mut for_end: i32;
                            main_p = 0;
                            for_end = (mem!(main_pp + 4).b16.s1 - 1) as i32;
                            if (main_p <= for_end) {
                                loop {
                                    (*native_node_text(cur_list.tail).offset((main_p) as isize)) =
                                        (*native_node_text(main_pp).offset((main_p) as isize));
                                    if !({
                                        let previous = main_p;
                                        main_p += 1;
                                        previous
                                    } < for_end)
                                    {
                                        break;
                                    }
                                }
                            }
                        }
                        {
                            let mut for_end: i32;
                            main_p = 0;
                            for_end = main_k - 1;
                            if (main_p <= for_end) {
                                loop {
                                    (*native_node_text(cur_list.tail).offset(
                                        (main_p + (mem!(main_pp + 4).b16.s1) as i32) as isize,
                                    )) = *native_text.offset((main_p) as isize);
                                    if !({
                                        let previous = main_p;
                                        main_p += 1;
                                        previous
                                    } < for_end)
                                    {
                                        break;
                                    }
                                }
                            }
                        }
                        set_native_metrics(cur_list.tail, (INTPAR!(xetex_use_glyph_metrics) > 0));
                        main_p = cur_list.head;
                        if (main_p != main_pp) {
                            while (mem!(main_p).b32.s1 != main_pp) {
                                main_p = LLIST_link!(main_p);
                            }
                        }
                        mem!(main_p).b32.s1 = mem!(main_pp).b32.s1;
                        mem!(main_pp).b32.s1 = TEX_NULL;
                        flush_node_list(main_pp);
                    } else {
                        mem!(main_pp).b32.s1 = new_native_word_node(main_f, main_k);
                        cur_list.tail = mem!(main_pp).b32.s1;
                        {
                            let mut for_end: i32;
                            main_p = 0;
                            for_end = main_k - 1;
                            if (main_p <= for_end) {
                                loop {
                                    (*native_node_text(cur_list.tail).offset((main_p) as isize)) =
                                        *native_text.offset((main_p) as isize);
                                    if !({
                                        let previous = main_p;
                                        main_p += 1;
                                        previous
                                    } < for_end)
                                    {
                                        break;
                                    }
                                }
                            }
                        }
                        set_native_metrics(cur_list.tail, (INTPAR!(xetex_use_glyph_metrics) > 0));
                    }
                }

                if (INTPAR!(xetex_interword_space_shaping) > 0) {
                    main_p = cur_list.head;
                    main_pp = TEX_NULL;
                    while (main_p != cur_list.tail) {
                        if ((main_p) != TEX_NULL
                            && (!(is_char_node(main_p)))
                            && (NODE_type!(main_p) == (WHATSIT_NODE) as u16)
                            && ((mem!(main_p).b16.s0 == (NATIVE_WORD_NODE) as u16)
                                || (mem!(main_p).b16.s0 == (NATIVE_WORD_NODE_AT) as u16)))
                        {
                            main_pp = main_p;
                        }
                        main_p = LLIST_link!(main_p);
                    }

                    if (main_pp != TEX_NULL) {
                        if (mem!(main_pp + 4).b16.s2 == (main_f) as u16) {
                            main_p = mem!(main_pp).b32.s1;
                            while (!(is_char_node(main_p))
                                && ((NODE_type!(main_p) == (PENALTY_NODE) as u16)
                                    || (NODE_type!(main_p) == (INS_NODE) as u16)
                                    || (NODE_type!(main_p) == (MARK_NODE) as u16)
                                    || (NODE_type!(main_p) == (ADJUST_NODE) as u16)
                                    || ((NODE_type!(main_p) == (WHATSIT_NODE) as u16)
                                        && (mem!(main_p).b16.s0 <= 4))))
                            {
                                main_p = LLIST_link!(main_p);
                            }
                            if (!(is_char_node(main_p))
                                && (NODE_type!(main_p) == (GLUE_NODE) as u16))
                            {
                                main_ppp = mem!(main_p).b32.s1;
                                while (!(is_char_node(main_ppp))
                                    && ((NODE_type!(main_ppp) == (PENALTY_NODE) as u16)
                                        || (NODE_type!(main_ppp) == (INS_NODE) as u16)
                                        || (NODE_type!(main_ppp) == (MARK_NODE) as u16)
                                        || (NODE_type!(main_ppp) == (ADJUST_NODE) as u16)
                                        || ((NODE_type!(main_ppp) == (WHATSIT_NODE) as u16)
                                            && (mem!(main_ppp).b16.s0 <= 4))))
                                {
                                    main_ppp = LLIST_link!(main_ppp);
                                }
                                if (main_ppp == cur_list.tail) {
                                    temp_ptr = new_native_word_node(
                                        main_f,
                                        (mem!(main_pp + 4).b16.s1
                                            + 1
                                            + (*mem.offset((cur_list.tail + 4) as isize)).b16.s1)
                                            as i32,
                                    );
                                    main_k = 0;
                                    {
                                        let mut for_end: i32;
                                        t = 0;
                                        for_end = (mem!(main_pp + 4).b16.s1 - 1) as i32;
                                        if (t <= for_end) {
                                            loop {
                                                (*native_node_text(temp_ptr)
                                                    .offset((main_k) as isize)) =
                                                    (*native_node_text(main_pp)
                                                        .offset((t) as isize));
                                                {
                                                    let previous = main_k;
                                                    main_k += 1;
                                                    previous
                                                };
                                                if !({
                                                    let previous = t;
                                                    t += 1;
                                                    previous
                                                } < for_end)
                                                {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    (*native_node_text(temp_ptr).offset((main_k) as isize)) =
                                        u16::from(b' ');
                                    {
                                        let previous = main_k;
                                        main_k += 1;
                                        previous
                                    };
                                    {
                                        let mut for_end: i32;
                                        t = 0;
                                        for_end = (mem!(cur_list.tail + 4).b16.s1 - 1) as i32;
                                        if (t <= for_end) {
                                            loop {
                                                (*native_node_text(temp_ptr)
                                                    .offset((main_k) as isize)) =
                                                    (*native_node_text(cur_list.tail)
                                                        .offset((t) as isize));
                                                {
                                                    let previous = main_k;
                                                    main_k += 1;
                                                    previous
                                                };
                                                if !({
                                                    let previous = t;
                                                    t += 1;
                                                    previous
                                                } < for_end)
                                                {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    set_native_metrics(
                                        temp_ptr,
                                        (INTPAR!(xetex_use_glyph_metrics) > 0),
                                    );
                                    t = mem!(temp_ptr + 1).b32.s1
                                        - mem!(main_pp + 1).b32.s1
                                        - mem!(cur_list.tail + 1).b32.s1;
                                    free_node(temp_ptr, (mem!(temp_ptr + 4).b16.s3) as i32);
                                    if (t
                                        != mem!((*font_glue.offset((main_f) as isize)) + 1).b32.s1)
                                    {
                                        temp_ptr = new_kern(
                                            t - mem!((*font_glue.offset((main_f) as isize)) + 1)
                                                .b32
                                                .s1,
                                        );
                                        NODE_subtype!(temp_ptr) = (SPACE_ADJUSTMENT) as u16;
                                        mem!(temp_ptr).b32.s1 = mem!(main_p).b32.s1;
                                        mem!(main_p).b32.s1 = temp_ptr;
                                    }
                                }
                            }
                        }
                    }
                }

                if (cur_ptr != TEX_NULL) {
                    state = MainControlState::BigSwitch;
                    continue 'control;
                } else {
                    state = MainControlState::Reswitch;
                    continue 'control;
                }
            }
            MainControlState::MainTraditional => {
                main_s = SF_CODE!(cur_chr) % 65536;

                if (main_s == 1000) {
                    cur_list.aux.b32.s0 = 1000;
                } else if (main_s < 1000) {
                    if (main_s > 0) {
                        cur_list.aux.b32.s0 = main_s;
                    }
                } else if (cur_list.aux.b32.s0 < 1000) {
                    cur_list.aux.b32.s0 = 1000;
                } else {
                    cur_list.aux.b32.s0 = main_s;
                }

                cur_ptr = TEX_NULL;
                space_class = SF_CODE!(cur_chr) / 65536;

                if ((INTPAR!(xetex_inter_char_tokens) > 0) && space_class != CHAR_CLASS_LIMIT) {
                    if (prev_class == (CHAR_CLASS_LIMIT - 1)) {
                        if ((cur_input.state != (TOKEN_LIST) as u16)
                            || (cur_input.index != (BACKED_UP_CHAR) as u16))
                        {
                            find_sa_element(
                                (INTER_CHAR_VAL) as i16,
                                (CHAR_CLASS_LIMIT - 1) * CHAR_CLASS_LIMIT + space_class,
                                false,
                            );
                            if (cur_ptr != TEX_NULL && ETEX_SA_ptr!(cur_ptr) != TEX_NULL) {
                                if (cur_cmd != (LETTER) as u8) {
                                    cur_cmd = (OTHER_CHAR) as u8;
                                }
                                cur_tok = i32::from(cur_cmd) * MAX_CHAR_VAL + cur_chr;
                                back_input();
                                cur_input.index = (BACKED_UP_CHAR) as u16;
                                begin_token_list(
                                    mem!(cur_ptr + 1).b32.s1,
                                    (INTER_CHAR_TEXT) as u16,
                                );
                                state = MainControlState::BigSwitch;
                                continue 'control;
                            }
                        }
                    } else {
                        find_sa_element(
                            (INTER_CHAR_VAL) as i16,
                            prev_class * CHAR_CLASS_LIMIT + space_class,
                            false,
                        );
                        if (cur_ptr != TEX_NULL && ETEX_SA_ptr!(cur_ptr) != TEX_NULL) {
                            if (cur_cmd != (LETTER) as u8) {
                                cur_cmd = (OTHER_CHAR) as u8;
                            }
                            cur_tok = i32::from(cur_cmd) * MAX_CHAR_VAL + cur_chr;
                            back_input();
                            cur_input.index = (BACKED_UP_CHAR) as u16;
                            begin_token_list(mem!(cur_ptr + 1).b32.s1, (INTER_CHAR_TEXT) as u16);
                            prev_class = (CHAR_CLASS_LIMIT - 1);
                            state = MainControlState::BigSwitch;
                            continue 'control;
                        }
                    }

                    prev_class = space_class;
                }

                main_f = eqtb!(CUR_FONT_LOC).b32.s1;
                bchar = (*font_bchar.offset((main_f) as isize));
                false_bchar = (*font_false_bchar.offset((main_f) as isize));

                if (cur_list.mode > 0) {
                    if (INTPAR!(language) != cur_list.aux.b32.s1) {
                        fix_language();
                    }
                }

                {
                    lig_stack = avail;
                    if (lig_stack == TEX_NULL) {
                        lig_stack = get_avail();
                    } else {
                        avail = mem!(lig_stack).b32.s1;
                        mem!(lig_stack).b32.s1 = TEX_NULL;
                    }
                }

                mem!(lig_stack).b16.s1 = (main_f) as u16;
                cur_l = cur_chr;
                mem!(lig_stack).b16.s0 = (cur_l) as u16;
                cur_q = cur_list.tail;

                if (cancel_boundary) {
                    cancel_boundary = false;
                    main_k = NON_ADDRESS;
                } else {
                    main_k = *bchar_label.offset((main_f) as isize);
                }

                if (main_k == NON_ADDRESS) {
                    state = MainControlState::MainLoopMove2;
                    continue 'control;
                }

                cur_r = cur_l;
                cur_l = TOO_BIG_CHAR;
                state = MainControlState::MainLigLoop1;
                continue 'control;

                state = MainControlState::MainLoopWrapup;
            }
            MainControlState::MainLoopWrapup => {
                if (cur_l < TOO_BIG_CHAR) {
                    if (mem!(cur_q).b32.s1 > TEX_NULL) {
                        if (mem!(cur_list.tail).b16.s0
                            == (*hyphen_char.offset((main_f) as isize)) as u16)
                        {
                            ins_disc = true;
                        }
                    }

                    if (ligature_present) {
                        main_p = new_ligature(main_f, (cur_l) as u16, mem!(cur_q).b32.s1);
                        if (lft_hit) {
                            mem!(main_p).b16.s0 = 2;
                            lft_hit = false;
                        }
                        if (rt_hit) {
                            if (lig_stack == TEX_NULL) {
                                mem!(main_p).b16.s0 += 1;
                                rt_hit = false;
                            }
                        }
                        mem!(cur_q).b32.s1 = main_p;
                        cur_list.tail = main_p;
                        ligature_present = false;
                    }

                    if (ins_disc) {
                        ins_disc = false;
                        if (cur_list.mode > 0) {
                            mem!(cur_list.tail).b32.s1 = new_disc();
                            cur_list.tail = LLIST_link!(cur_list.tail);
                        }
                    }
                }

                state = MainControlState::MainLoopMove;
            }
            MainControlState::MainLoopMove => {
                if (lig_stack == TEX_NULL) {
                    state = MainControlState::Reswitch;
                    continue 'control;
                }

                cur_q = cur_list.tail;
                cur_l = (mem!(lig_stack).b16.s0) as i32;

                state = MainControlState::MainLoopMove1;
            }
            MainControlState::MainLoopMove1 => {
                if (!is_char_node(lig_stack)) {
                    state = MainControlState::MainLoopMoveLig;
                    continue 'control;
                }

                state = MainControlState::MainLoopMove2;
            }
            MainControlState::MainLoopMove2 => {
                if ((effective_char(false, main_f, (cur_chr) as u16)
                    > (*font_ec.offset((main_f) as isize)) as i32)
                    || (effective_char(false, main_f, (cur_chr) as u16)
                        < (*font_bc.offset((main_f) as isize)) as i32))
                {
                    char_warning(main_f, cur_chr);
                    mem!(lig_stack).b32.s1 = avail;
                    avail = lig_stack;
                    state = MainControlState::BigSwitch;
                    continue 'control;
                }

                main_i = effective_char_info(main_f, (cur_l) as u16);

                if (!(main_i.s3 > 0)) {
                    char_warning(main_f, cur_chr);
                    mem!(lig_stack).b32.s1 = avail;
                    avail = lig_stack;
                    state = MainControlState::BigSwitch;
                    continue 'control;
                }

                mem!(cur_list.tail).b32.s1 = lig_stack;
                cur_list.tail = lig_stack;

                state = MainControlState::MainLoopLookahead;
            }
            MainControlState::MainLoopLookahead => {
                get_next();

                if (cur_cmd == (LETTER) as u8) {
                    state = MainControlState::MainLoopLookahead1;
                    continue 'control;
                }
                if (cur_cmd == (OTHER_CHAR) as u8) {
                    state = MainControlState::MainLoopLookahead1;
                    continue 'control;
                }
                if (cur_cmd == (CHAR_GIVEN) as u8) {
                    state = MainControlState::MainLoopLookahead1;
                    continue 'control;
                }

                x_token();
                if (cur_cmd == (LETTER) as u8) {
                    state = MainControlState::MainLoopLookahead1;
                    continue 'control;
                }
                if (cur_cmd == (OTHER_CHAR) as u8) {
                    state = MainControlState::MainLoopLookahead1;
                    continue 'control;
                }
                if (cur_cmd == (CHAR_GIVEN) as u8) {
                    state = MainControlState::MainLoopLookahead1;
                    continue 'control;
                }

                if (cur_cmd == (CHAR_NUM) as u8) {
                    scan_char_num();
                    cur_chr = cur_val;
                    state = MainControlState::MainLoopLookahead1;
                    continue 'control;
                }

                if (cur_cmd == (NO_BOUNDARY) as u8) {
                    bchar = TOO_BIG_CHAR;
                }

                cur_r = bchar;
                lig_stack = TEX_NULL;
                state = MainControlState::MainLigLoop;
                continue 'control;

                state = MainControlState::MainLoopLookahead1;
            }
            MainControlState::MainLoopLookahead1 => {
                main_s = SF_CODE!(cur_chr) % 65536;

                if (main_s == 1000) {
                    cur_list.aux.b32.s0 = 1000;
                } else if (main_s < 1000) {
                    if (main_s > 0) {
                        cur_list.aux.b32.s0 = main_s;
                    }
                } else if (cur_list.aux.b32.s0 < 1000) {
                    cur_list.aux.b32.s0 = 1000;
                } else {
                    cur_list.aux.b32.s0 = main_s;
                }

                cur_ptr = TEX_NULL;
                space_class = SF_CODE!(cur_chr) / 65536;

                if ((INTPAR!(xetex_inter_char_tokens) > 0) && space_class != CHAR_CLASS_LIMIT) {
                    if (prev_class == (CHAR_CLASS_LIMIT - 1)) {
                        if ((cur_input.state != (TOKEN_LIST) as u16)
                            || (cur_input.index != (BACKED_UP_CHAR) as u16))
                        {
                            find_sa_element(
                                (INTER_CHAR_VAL) as i16,
                                (CHAR_CLASS_LIMIT - 1) * CHAR_CLASS_LIMIT + space_class,
                                false,
                            );
                            if (cur_ptr != TEX_NULL && ETEX_SA_ptr!(cur_ptr) != TEX_NULL) {
                                if (cur_cmd != (LETTER) as u8) {
                                    cur_cmd = (OTHER_CHAR) as u8;
                                }
                                cur_tok = i32::from(cur_cmd) * MAX_CHAR_VAL + cur_chr;
                                back_input();
                                cur_input.index = (BACKED_UP_CHAR) as u16;
                                begin_token_list(
                                    mem!(cur_ptr + 1).b32.s1,
                                    (INTER_CHAR_TEXT) as u16,
                                );
                                state = MainControlState::BigSwitch;
                                continue 'control;
                            }
                        }
                    } else {
                        find_sa_element(
                            (INTER_CHAR_VAL) as i16,
                            prev_class * CHAR_CLASS_LIMIT + space_class,
                            false,
                        );
                        if (cur_ptr != TEX_NULL && ETEX_SA_ptr!(cur_ptr) != TEX_NULL) {
                            if (cur_cmd != (LETTER) as u8) {
                                cur_cmd = (OTHER_CHAR) as u8;
                            }
                            cur_tok = i32::from(cur_cmd) * MAX_CHAR_VAL + cur_chr;
                            back_input();
                            cur_input.index = (BACKED_UP_CHAR) as u16;
                            begin_token_list(mem!(cur_ptr + 1).b32.s1, (INTER_CHAR_TEXT) as u16);
                            prev_class = (CHAR_CLASS_LIMIT - 1);
                            state = MainControlState::BigSwitch;
                            continue 'control;
                        }
                    }
                    prev_class = space_class;
                }

                {
                    lig_stack = avail;
                    if (lig_stack == TEX_NULL) {
                        lig_stack = get_avail();
                    } else {
                        avail = mem!(lig_stack).b32.s1;
                        mem!(lig_stack).b32.s1 = TEX_NULL;
                    }
                }

                mem!(lig_stack).b16.s1 = (main_f) as u16;
                cur_r = cur_chr;
                mem!(lig_stack).b16.s0 = (cur_r) as u16;

                if (cur_r == false_bchar) {
                    cur_r = TOO_BIG_CHAR;
                }

                state = MainControlState::MainLigLoop;
            }
            MainControlState::MainLigLoop => {
                if (((main_i.s1) % 4) != (LIG_TAG) as u16) {
                    state = MainControlState::MainLoopWrapup;
                    continue 'control;
                }
                if (cur_r == TOO_BIG_CHAR) {
                    state = MainControlState::MainLoopWrapup;
                    continue 'control;
                }

                main_k = (*lig_kern_base.offset((main_f) as isize)) + (main_i.s0) as i32;
                main_j = (*font_info.offset((main_k) as isize)).b16;
                if (main_j.s3 <= 128) {
                    state = MainControlState::MainLigLoop2;
                    continue 'control;
                }

                main_k = (*lig_kern_base.offset((main_f) as isize))
                    + (256 * main_j.s1) as i32
                    + (main_j.s0) as i32
                    + 32768
                    - 256 * 128;

                state = MainControlState::MainLigLoop1;
            }
            MainControlState::MainLigLoop1 => {
                main_j = (*font_info.offset((main_k) as isize)).b16;

                state = MainControlState::MainLigLoop2;
            }
            MainControlState::MainLigLoop2 => {
                if (main_j.s2 == (cur_r) as u16) {
                    if (main_j.s3 <= 128) {
                        if (main_j.s1 >= 128) {
                            if (cur_l < TOO_BIG_CHAR) {
                                if (mem!(cur_q).b32.s1 > TEX_NULL) {
                                    if (mem!(cur_list.tail).b16.s0
                                        == (*hyphen_char.offset((main_f) as isize)) as u16)
                                    {
                                        ins_disc = true;
                                    }
                                }

                                if (ligature_present) {
                                    main_p =
                                        new_ligature(main_f, (cur_l) as u16, mem!(cur_q).b32.s1);
                                    if (lft_hit) {
                                        mem!(main_p).b16.s0 = 2;
                                        lft_hit = false;
                                    }
                                    if (rt_hit) {
                                        if (lig_stack == TEX_NULL) {
                                            mem!(main_p).b16.s0 += 1;
                                            rt_hit = false;
                                        }
                                    }
                                    mem!(cur_q).b32.s1 = main_p;
                                    cur_list.tail = main_p;
                                    ligature_present = false;
                                }

                                if (ins_disc) {
                                    ins_disc = false;
                                    if (cur_list.mode > 0) {
                                        mem!(cur_list.tail).b32.s1 = new_disc();
                                        cur_list.tail = LLIST_link!(cur_list.tail);
                                    }
                                }
                            }

                            mem!(cur_list.tail).b32.s1 = new_kern(
                                (*font_info.offset(
                                    ((*kern_base.offset((main_f) as isize))
                                        + (256 * main_j.s1) as i32
                                        + (main_j.s0) as i32)
                                        as isize,
                                ))
                                .b32
                                .s1,
                            );
                            cur_list.tail = LLIST_link!(cur_list.tail);
                            state = MainControlState::MainLoopMove;
                            continue 'control;
                        }

                        if (cur_l == TOO_BIG_CHAR) {
                            lft_hit = true;
                        } else if (lig_stack == TEX_NULL) {
                            rt_hit = true;
                        }

                        match main_j.s1 {
                            1 | 5 => {
                                cur_l = (main_j.s0) as i32;
                                main_i = crate::FONT_CHARACTER_INFO!(
                                    main_f,
                                    effective_char(true, main_f, (cur_l) as u16)
                                );
                                ligature_present = true;
                            }

                            2 | 6 => {
                                cur_r = (main_j.s0) as i32;

                                if (lig_stack == TEX_NULL) {
                                    lig_stack = new_lig_item((cur_r) as u16);
                                    bchar = TOO_BIG_CHAR;
                                } else if (is_char_node(lig_stack)) {
                                    main_p = lig_stack;
                                    lig_stack = new_lig_item((cur_r) as u16);
                                    mem!(lig_stack + 1).b32.s1 = main_p;
                                } else {
                                    mem!(lig_stack).b16.s0 = (cur_r) as u16;
                                }
                            }

                            3 => {
                                cur_r = (main_j.s0) as i32;
                                main_p = lig_stack;
                                lig_stack = new_lig_item((cur_r) as u16);
                                mem!(lig_stack).b32.s1 = main_p;
                            }

                            7 | 11 => {
                                if (cur_l < TOO_BIG_CHAR) {
                                    if (mem!(cur_q).b32.s1 > TEX_NULL) {
                                        if (mem!(cur_list.tail).b16.s0
                                            == (*hyphen_char.offset((main_f) as isize)) as u16)
                                        {
                                            ins_disc = true;
                                        }
                                    }
                                    if (ligature_present) {
                                        main_p = new_ligature(
                                            main_f,
                                            (cur_l) as u16,
                                            mem!(cur_q).b32.s1,
                                        );
                                        if (lft_hit) {
                                            mem!(main_p).b16.s0 = 2;
                                            lft_hit = false;
                                        }
                                        if (false) {
                                            if (lig_stack == TEX_NULL) {
                                                mem!(main_p).b16.s0 += 1;
                                                rt_hit = false;
                                            }
                                        }
                                        mem!(cur_q).b32.s1 = main_p;
                                        cur_list.tail = main_p;
                                        ligature_present = false;
                                    }
                                    if (ins_disc) {
                                        ins_disc = false;
                                        if (cur_list.mode > 0) {
                                            mem!(cur_list.tail).b32.s1 = new_disc();
                                            cur_list.tail = LLIST_link!(cur_list.tail);
                                        }
                                    }
                                }
                                cur_q = cur_list.tail;
                                cur_l = (main_j.s0) as i32;
                                main_i = crate::FONT_CHARACTER_INFO!(
                                    main_f,
                                    effective_char(true, main_f, (cur_l) as u16)
                                );
                                ligature_present = true;
                            }

                            _ => {
                                cur_l = (main_j.s0) as i32;
                                ligature_present = true;
                                if (lig_stack == TEX_NULL) {
                                    state = MainControlState::MainLoopWrapup;
                                    continue 'control;
                                } else {
                                    state = MainControlState::MainLoopMove1;
                                    continue 'control;
                                }
                            }
                        }

                        if (main_j.s1 > 4) {
                            if (main_j.s1 != 7) {
                                state = MainControlState::MainLoopWrapup;
                                continue 'control;
                            }
                        }

                        if (cur_l < TOO_BIG_CHAR) {
                            state = MainControlState::MainLigLoop;
                            continue 'control;
                        }

                        main_k = *bchar_label.offset((main_f) as isize);
                        state = MainControlState::MainLigLoop1;
                        continue 'control;
                    }
                }

                if (main_j.s3 == 0) {
                    {
                        let previous = main_k;
                        main_k += 1;
                        previous
                    };
                } else {
                    if (main_j.s3 >= 128) {
                        state = MainControlState::MainLoopWrapup;
                        continue 'control;
                    }
                    main_k = main_k + (main_j.s3) as i32 + 1;
                }

                state = MainControlState::MainLigLoop1;
                continue 'control;

                state = MainControlState::MainLoopMoveLig;
            }
            MainControlState::MainLoopMoveLig => {
                main_p = mem!(lig_stack + 1).b32.s1;

                if (main_p > TEX_NULL) {
                    mem!(cur_list.tail).b32.s1 = main_p;
                    cur_list.tail = LLIST_link!(cur_list.tail);
                }

                temp_ptr = lig_stack;
                lig_stack = mem!(temp_ptr).b32.s1;
                free_node(temp_ptr, SMALL_NODE_SIZE);
                main_i = crate::FONT_CHARACTER_INFO!(
                    main_f,
                    effective_char(true, main_f, (cur_l) as u16)
                );
                ligature_present = true;

                if (lig_stack == TEX_NULL) {
                    if (main_p > TEX_NULL) {
                        state = MainControlState::MainLoopLookahead;
                        continue 'control;
                    } else {
                        cur_r = bchar;
                    }
                } else {
                    cur_r = (mem!(lig_stack).b16.s0) as i32;
                }

                state = MainControlState::MainLigLoop;
                continue 'control;

                state = MainControlState::MainLigLoop;
            }
            MainControlState::AppendNormalSpace => {
                if ((INTPAR!(xetex_inter_char_tokens) > 0)
                    && (space_class != CHAR_CLASS_LIMIT)
                    && (prev_class != (CHAR_CLASS_LIMIT - 1)))
                {
                    prev_class = (CHAR_CLASS_LIMIT - 1);
                    find_sa_element(
                        (INTER_CHAR_VAL) as i16,
                        space_class * CHAR_CLASS_LIMIT + (CHAR_CLASS_LIMIT - 1),
                        false,
                    );
                    if (cur_ptr != TEX_NULL && ETEX_SA_ptr!(cur_ptr) != TEX_NULL) {
                        if (cur_cs == 0) {
                            if (cur_cmd == (CHAR_NUM) as u8) {
                                cur_cmd = (OTHER_CHAR) as u8;
                            }
                            cur_tok = i32::from(cur_cmd) * MAX_CHAR_VAL + cur_chr;
                        } else {
                            cur_tok = CS_TOKEN_FLAG + cur_cs;
                        }
                        back_input();
                        begin_token_list(mem!(cur_ptr + 1).b32.s1, (INTER_CHAR_TEXT) as u16);
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                }

                if (GLUEPAR!(space_skip) == 0) {
                    main_p = (*font_glue.offset((eqtb!(CUR_FONT_LOC).b32.s1) as isize));
                    if (main_p == TEX_NULL) {
                        main_p = new_spec(0);
                        main_k = (*param_base.offset((eqtb!(CUR_FONT_LOC).b32.s1) as isize)) + 2;
                        mem!(main_p + 1).b32.s1 = (*font_info.offset((main_k) as isize)).b32.s1;
                        mem!(main_p + 2).b32.s1 = (*font_info.offset((main_k + 1) as isize)).b32.s1;
                        mem!(main_p + 3).b32.s1 = (*font_info.offset((main_k + 2) as isize)).b32.s1;
                        (*font_glue.offset((eqtb!(CUR_FONT_LOC).b32.s1) as isize)) = main_p;
                    }
                    temp_ptr = new_glue(main_p);
                } else {
                    temp_ptr = new_param_glue((GLUE_PAR__space_skip) as i16);
                }

                mem!(cur_list.tail).b32.s1 = temp_ptr;
                cur_list.tail = temp_ptr;
                state = MainControlState::BigSwitch;
                continue 'control;
            }
        }
    }
}

// 113. give_err_help
#[unsafe(export_name = "give_err_help")]
pub unsafe extern "C" fn give_err_help() {
    token_show(LOCAL!(LOCAL__err_help));
}

// 114. close_files_and_terminate
#[unsafe(export_name = "close_files_and_terminate")]
pub unsafe extern "C" fn close_files_and_terminate() {
    let mut k: i32;

    for k in 0..=15 {
        if write_open[k as usize] {
            ttstub_output_close(write_file[k as usize]);
        }
    }

    INTPAR!(new_line_char) = -1;

    finalize_dvi_file();
    synctex_terminate(log_opened);

    if log_opened {
        ttstub_output_putc(log_file, b'\n' as i32);
        ttstub_output_close(log_file);
        selector = selector - 2;
        if selector == (SELECTOR_TERM_ONLY as u8) as i32 {
            print_nl_cstr(b"Transcript written on \0".as_ptr() as *const c_char);
            print(texmf_log_name);
            print_char(b'.' as i32);
        }
    }

    print_ln();
}

// 115. flush_str
#[unsafe(export_name = "flush_str")]
pub unsafe extern "C" fn flush_str(s: str_number) {
    if s == str_ptr - 1 {
        str_ptr -= 1;
        pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
    }
}

// 116. tokens_to_string
#[unsafe(export_name = "tokens_to_string")]
pub unsafe extern "C" fn tokens_to_string(p: i32) -> str_number {
    if selector == (SELECTOR_NEW_STRING as u8) as i32 {
        pdf_error(
            b"tokens\0".as_ptr() as *const c_char,
            b"tokens_to_string() called while selector = new_string\0".as_ptr() as *const c_char,
        );
    }
    old_setting = (selector) as u8;
    selector = (SELECTOR_NEW_STRING as u8) as i32;
    show_token_list(mem!(p).b32.s1, TEX_NULL, (pool_size - pool_ptr) as i32);
    selector = (old_setting) as i32;
    make_string()
}

// 117. scan_pdf_ext_toks
#[unsafe(export_name = "scan_pdf_ext_toks")]
pub unsafe extern "C" fn scan_pdf_ext_toks() {
    scan_toks(false, true);
}

// 118. compare_strings
#[unsafe(export_name = "compare_strings")]
pub unsafe extern "C" fn compare_strings() {
    let mut s1: str_number;
    let mut s2: str_number;
    let mut i1: pool_pointer;
    let mut i2: pool_pointer;
    let mut j1: pool_pointer;
    let mut j2: pool_pointer;
    let mut save_cur_cs: i32;

    save_cur_cs = cur_cs;
    scan_toks(false, true);
    s1 = tokens_to_string(def_ref);
    delete_token_ref(def_ref);
    cur_cs = save_cur_cs;
    scan_toks(false, true);
    s2 = tokens_to_string(def_ref);
    delete_token_ref(def_ref);
    i1 = *str_start.offset(((s1) - 65536) as isize);
    j1 = *str_start.offset(((s1 + 1) - 65536) as isize);
    i2 = *str_start.offset(((s2) - 65536) as isize);
    j2 = *str_start.offset(((s2 + 1) - 65536) as isize);

    'done: {
        while (i1 < j1) && (i2 < j2) {
            if *str_pool.offset(i1 as isize) < *str_pool.offset(i2 as isize) {
                cur_val = -1;
                break 'done;
            }
            if *str_pool.offset(i1 as isize) > *str_pool.offset(i2 as isize) {
                cur_val = 1;
                break 'done;
            }
            i1 += 1;
            i2 += 1;
        }
        if (i1 == j1) && (i2 == j2) {
            cur_val = 0;
        } else if i1 < j1 {
            cur_val = 1;
        } else {
            cur_val = -1;
        }
        break 'done;
    }
    flush_str(s2);
    flush_str(s1);
    cur_val_level = INT_VAL as u8;
}

// 119. prune_page_top
#[unsafe(export_name = "prune_page_top")]
pub unsafe extern "C" fn prune_page_top(p: i32, s: bool) -> i32 {
    let mut prev_p: i32;
    let mut q: i32;
    let mut r: i32 = TEX_NULL;
    let mut cur_p: i32 = p;

    prev_p = TEMP_HEAD;
    mem!(TEMP_HEAD).b32.s1 = cur_p;

    while cur_p != TEX_NULL {
        match mem!(cur_p).b16.s1 as i32 {
            HLIST_NODE | VLIST_NODE | RULE_NODE => {
                q = new_skip_param(GLUE_PAR__split_top_skip as small_number);
                mem!(prev_p).b32.s1 = q;
                mem!(q).b32.s1 = cur_p;
                if mem!(temp_ptr + 1).b32.s1 > mem!(cur_p + 3).b32.s1 {
                    mem!(temp_ptr + 1).b32.s1 = mem!(temp_ptr + 1).b32.s1 - mem!(cur_p + 3).b32.s1;
                } else {
                    mem!(temp_ptr + 1).b32.s1 = 0;
                }
                cur_p = TEX_NULL;
            }
            WHATSIT_NODE | MARK_NODE | INS_NODE => {
                prev_p = cur_p;
                cur_p = mem!(prev_p).b32.s1;
            }
            GLUE_NODE | KERN_NODE | PENALTY_NODE => {
                q = cur_p;
                cur_p = mem!(q).b32.s1;
                mem!(q).b32.s1 = TEX_NULL;
                mem!(prev_p).b32.s1 = cur_p;
                if s {
                    if disc_ptr[VSPLIT_CODE as usize] == TEX_NULL {
                        disc_ptr[VSPLIT_CODE as usize] = q;
                    } else {
                        mem!(r).b32.s1 = q;
                    }
                    r = q;
                } else {
                    flush_node_list(q);
                }
            }
            _ => {
                confusion(b"pruning\0".as_ptr() as *const c_char);
            }
        }
    }

    mem!(TEMP_HEAD).b32.s1
}

// 120. do_marks
#[unsafe(export_name = "do_marks")]
pub unsafe extern "C" fn do_marks(a: small_number, l: small_number, mut q: i32) -> bool {
    let mut i: small_number;

    if l < 4 {
        for i in 0..=15 {
            if odd(i as i32) {
                cur_ptr = mem!(q + ((i as i32) / 2) + 1).b32.s1;
            } else {
                cur_ptr = mem!(q + ((i as i32) / 2) + 1).b32.s0;
            }

            if cur_ptr != TEX_NULL {
                if do_marks(a, l + 1, cur_ptr) {
                    if odd(i as i32) {
                        mem!(q + ((i as i32) / 2) + 1).b32.s1 = TEX_NULL;
                    } else {
                        mem!(q + ((i as i32) / 2) + 1).b32.s0 = TEX_NULL;
                    }
                    mem!(q).b16.s0 -= 1;
                }
            }
        }

        if mem!(q).b16.s0 == 0 {
            free_node(q, INDEX_NODE_SIZE as i32);
            q = TEX_NULL;
        }
    } else {
        match a as i32 {
            VSPLIT_INIT => {
                if mem!(q + 2).b32.s1 != TEX_NULL {
                    delete_token_ref(mem!(q + 2).b32.s1);
                    mem!(q + 2).b32.s1 = TEX_NULL;
                    delete_token_ref(mem!(q + 3).b32.s0);
                    mem!(q + 3).b32.s0 = TEX_NULL;
                }
            }
            FIRE_UP_INIT => {
                if mem!(q + 2).b32.s0 != TEX_NULL {
                    if mem!(q + 1).b32.s0 != TEX_NULL {
                        delete_token_ref(mem!(q + 1).b32.s0);
                    }
                    delete_token_ref(mem!(q + 1).b32.s1);
                    mem!(q + 1).b32.s1 = TEX_NULL;
                    if mem!(mem!(q + 2).b32.s0).b32.s1 == TEX_NULL {
                        delete_token_ref(mem!(q + 2).b32.s0);
                        mem!(q + 2).b32.s0 = TEX_NULL;
                    } else {
                        mem!(mem!(q + 2).b32.s0).b32.s0 += 1;
                    }
                    mem!(q + 1).b32.s0 = mem!(q + 2).b32.s0;
                }
            }
            FIRE_UP_DONE => {
                if mem!(q + 1).b32.s0 != TEX_NULL && mem!(q + 1).b32.s1 == TEX_NULL {
                    mem!(q + 1).b32.s1 = mem!(q + 1).b32.s0;
                    mem!(mem!(q + 1).b32.s0).b32.s0 += 1;
                }
            }
            DESTROY_MARKS => {
                for i in TOP_MARK_CODE..=SPLIT_BOT_MARK_CODE {
                    if odd(i as i32) {
                        cur_ptr = mem!(q + ((i as i32) / 2) + 1).b32.s1;
                    } else {
                        cur_ptr = mem!(q + ((i as i32) / 2) + 1).b32.s0;
                    }

                    if cur_ptr != TEX_NULL {
                        delete_token_ref(cur_ptr);
                        if odd(i as i32) {
                            mem!(q + ((i as i32) / 2) + 1).b32.s1 = TEX_NULL;
                        } else {
                            mem!(q + ((i as i32) / 2) + 1).b32.s0 = TEX_NULL;
                        }
                    }
                }
            }
            _ => {}
        }

        if mem!(q + 2).b32.s0 == TEX_NULL {
            if mem!(q + 3).b32.s0 == TEX_NULL {
                free_node(q, MARK_CLASS_NODE_SIZE as i32);
                q = TEX_NULL;
            }
        }
    }

    q == TEX_NULL
}

// 121. do_assignments
#[unsafe(export_name = "do_assignments")]
pub unsafe extern "C" fn do_assignments() {
    loop {
        loop {
            get_x_token();
            if cur_cmd as i32 != SPACER && cur_cmd as i32 != RELAX {
                break;
            }
        }

        if (cur_cmd as i32) <= MAX_NON_PREFIXED_COMMAND {
            return;
        }

        set_box_allowed = false;
        prefixed_command();
        set_box_allowed = true;
    }
}

// 122. new_whatsit
#[unsafe(export_name = "new_whatsit")]
pub unsafe extern "C" fn new_whatsit(s: small_number, w: small_number) {
    let p = get_node(w as i32);
    NODE_type!(p) = WHATSIT_NODE as u16;
    mem!(p).b16.s0 = (s as u16) as u16;
    mem!(cur_list.tail).b32.s1 = p;
    cur_list.tail = p;
}
