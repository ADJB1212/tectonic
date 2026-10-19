#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
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

use super::bridge::{ttstub_diag_printf, ttstub_output_flush};
use super::xetex_ext::{
    AAT_FONT_FLAG, LEFT_SIDE, OTGR_FONT_FLAG, RIGHT_SIDE, aat_font_get, aat_font_get_1,
    aat_font_get_2, aat_font_get_named, aat_font_get_named_1, aat_print_font_name,
    apply_tfm_font_mapping, count_pdf_file_pages, get_cp_code, get_font_char_range,
    get_glyph_bounds, getnativechardp, getnativecharht, getnativecharic, getnativecharwd,
    gr_font_get_named, gr_font_get_named_1, gr_print_font_name, map_char_to_glyph,
    map_glyph_to_index, ot_font_get, ot_font_get_1, ot_font_get_2, ot_font_get_3, print_glyph_name,
    usingGraphite, usingOpenType,
};
use super::xetex_io::{input_line, u_close};
use super::xetex_xetexd::{HISTORY_WARNING_ISSUED, b32x2};
use super::*;

use super::xetex_checkpoint::*;
use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_synctex::*;
use super::xetex_xetexd::*;
use libc::*;

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
