// Copyright 2016-2026 The Tectonic Project
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

use super::xetex_checkpoint::{tt_checkpoint, tt_checkpoint_field};
use super::xetex_constants::{BATCH_MODE, CALL, ERROR_STOP_MODE, FONT_BASE, HASH_BASE, MEM_TOP, NORMAL, SCROLL_MODE, TEX_NULL, TOO_BIG_CHAR};
use super::xetex_core::scaled_t;
use super::xetex_errors::{HISTORY_FATAL_ERROR, confusion, error, overflow, ttstub_output_flush};
use super::xetex_ext::ttstub_input_read;
use super::xetex_ini::{compress_trie, first_fit, new_trie_op, trie_fix, trie_pack};
use super::xetex_io::{_tt_abort, INVALID_HANDLE, ttstub_input_close, ttstub_input_open, xcalloc, xmalloc};
use super::xetex_output::{
    capture_to_diagnostic, error_here_with_diagnostic, print, print_char, print_cstr, print_int, print_ln, print_nl_cstr, print_scaled,
};
use super::xetex_stringpool::{EMPTY_STRING, length, make_string};
use super::xetex_xetex0::{
    back_error, back_input, eq_define, eq_word_define, find_font_dimen, find_sa_element, flush_list, geq_define, geq_word_define, get_avail,
    get_node, get_r_token, get_token, get_x_token, gsa_def, make_name_string, max_hyphenatable_length, pack_job_name, print_cmd_chr, read_toks,
    sa_def, scan_char_class, scan_char_class_not_ignored, scan_char_num, scan_dimen, scan_fifteen_bit_int, scan_font_ident, scan_glue,
    scan_glyph_number, scan_int, scan_keyword, scan_left_brace, scan_math_class_int, scan_math_fam_int, scan_optional_equals, scan_register_num,
    scan_toks, scan_usv_num, scan_xetex_math_char_int, show_cur_cmd_chr, trap_zero_glue,
};
use super::xetex_xetexd::{
    _xeq_level_array, LR_problems, LR_ptr, TEX_format_default, UTF16_code, active_width, adjust_tail, after_token, align_ptr, align_state,
    area_delimiter, arith_error, avail, b16x4, b32x2, bad, base_ptr, bchar, bchar_label, best_height_plus_depth, breadth_max, buf_size, buffer,
    cancel_boundary, char_base, cond_ptr, cs_count, cur_align, cur_area, cur_boundary, cur_box, cur_c, cur_chr, cur_cmd, cur_cs, cur_dir, cur_ext,
    cur_f, cur_group, cur_h, cur_h_offset, cur_head, cur_i, cur_if, cur_input, cur_l, cur_lang, cur_level, cur_list, cur_loop, cur_mark, cur_name,
    cur_order, cur_page_height, cur_page_width, cur_pre_head, cur_pre_tail, cur_ptr, cur_q, cur_r, cur_span, cur_tail, cur_tok, cur_v, cur_v_offset,
    cur_val, cur_val_level, cur_val1, dead_cycles, def_ref, deletions_allowed, delta, depth_base, depth_threshold, dig, disc_ptr, doing_leaders,
    doing_special, dyn_used, edit_name_start, empty, eof_seen, epochseconds, eqtb, eqtb_top, error_count, error_line, expand_depth,
    expand_depth_count, ext_delimiter, exten_base, false_bchar, file_line_error_style_p, file_name_quote_char, file_offset, first, first_count,
    first_p, fmem_ptr, font_area, font_bc, font_bchar, font_check, font_dsize, font_ec, font_false_bchar, font_flags, font_glue,
    font_in_short_display, font_info, font_layout_engine, font_letter_space, font_mapping, font_max, font_mem_size, font_name, font_params, font_ptr,
    font_size, font_used, force_eof, format_default_length, format_ident, full_source_filename_stack, gave_char_warning_help, global_prev_p,
    grp_stack, half_error_line, halt_on_error_p, hash, hash_extra, hash_high, hash_top, hash_used, hc, height_base, help_line, help_ptr, hf,
    hi_mem_min, history, hlist_stack, hlist_stack_level, hu, hyf, hyf_distance, hyf_next, hyf_num, hyph_count, hyph_index, hyph_link, hyph_list,
    hyph_next, hyph_size, hyph_start, hyph_word, hyphen_char, hyphen_passed, if_limit, if_line, if_stack, in_initex_mode, in_open, init_lft,
    init_lig, init_list, init_pool_ptr, init_str_ptr, input_file, input_ptr, input_stack, ins_disc, insert_penalties, insert_src_special_auto,
    insert_src_special_every_math, insert_src_special_every_par, insert_src_special_every_vbox, interaction, is_hyph, is_in_csname, italic_base,
    j_random, job_name, just_box, kern_base, last, last_badness, last_bop, last_glue, last_kern, last_leftmost_char, last_node_type, last_penalty,
    last_rightmost_char, lft_hit, lig_kern_base, lig_stack, ligature_present, line, line_stack, lo_mem_max, loaded_font_flags,
    loaded_font_letter_space, loaded_font_mapping, log_file, log_opened, long_help_seen, long_state, mag_set, main_f, main_h, main_i, main_j, main_k,
    main_p, main_pp, main_ppp, main_s, max_buf_stack, max_h, max_hyph_char, max_in_open, max_in_stack, max_nest_stack, max_op_used, max_param_stack,
    max_print_line, max_push, max_reg_help_line, max_reg_num, max_save_stack, max_strings, max_v, mem, mem_end, memory_word, microseconds,
    name_in_progress, name_length, name_length16, name_of_file, name_of_file16, native_font_type_flag, native_len, native_text, native_text_size,
    nest, nest_ptr, nest_size, no_new_control_sequence, null_character, old_setting, op_start, open_parens, output_active, output_file_extension,
    pack_begin_line, packed_UTF16_code, page_contents, page_so_far, page_tail, par_loc, par_token, param_base, param_ptr, param_size, param_stack,
    pdf_last_x_pos, pdf_last_y_pos, pool_free, pool_pointer, pool_ptr, pool_size, pre_adjust_tail, prev_class, prim, prim_used, pseudo_files, pstack,
    quoted_filename, radix, random_seed, randoms, read_file, read_open, rover, rt_hit, rule_dp, rule_ht, rule_wd, rust_input_handle_t,
    rust_output_handle_t, rust_stdout, sa_chain, sa_level, sa_null, sa_root, save_native_len, save_ptr, save_size, save_stack, scanner_status,
    selector, semantic_pagination_enabled, set_box_allowed, shell_escape_enabled, shown_mode, skew_char, skip_line, source_filename_stack,
    space_class, spec_log, stack_size, stop_at_space, str_number, str_pool, str_ptr, str_start, string_vacancies, strings_free, synctex_enabled,
    tally, temp_ptr, term_offset, tex_remainder, texmf_log_name, total_pages, total_shrink, total_stretch, trick_buf, trick_count, trie_c, trie_hash,
    trie_l, trie_max, trie_min, trie_not_ready, trie_o, trie_op_lang, trie_op_ptr, trie_op_val, trie_ptr, trie_r, trie_size, trie_trc, trie_trl,
    trie_tro, trie_used, two_to_the, use_err_help, used_tectonic_coda_tokens, var_used, warning_index, width_base, write_file, write_loc, write_open,
    xtx_ligature_present,
};

use std::ffi::{c_char, c_int, c_uint, c_void};
use tectonic_bridge_core::FileFormat;

use super::bridge::*;
use super::teckit_c_engine::*;
use super::teckit_common::*;
use super::xetex_bindings::*;
use super::xetex_checkpoint::*;
use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_errors::*;
use super::xetex_ext::*;
use super::xetex_format::*;
use super::xetex_ini::*;
use super::xetex_io::*;
use super::xetex_scaledmath::*;
use super::xetex_stringpool::*;
use super::xetex_texmfmp::*;
use super::xetex_xetex0::*;
use super::xetex_xetexd::*;

unsafe extern "C" {
    pub static mut stdin_ufile: UFILE;
    pub static mut yhash: *mut b32x2;
    pub fn set_cp_code(font_num: i32, code: u32, side: i32, value: i32);
    pub fn do_register_command(a: small_number);
    pub fn scan_box(box_context: i32);
    pub fn alter_aux();
    pub fn alter_prev_graf();
    pub fn alter_page_so_far();
    pub fn alter_integer();
    pub fn alter_box_dimen();
    pub fn new_font(a: small_number);
    pub fn new_interaction();
    pub fn pseudo_close();
    pub fn close_files_and_terminate();
}

#[inline]
pub unsafe fn INTPAR(idx: i32) -> i32 {
    (*eqtb.add((INT_BASE + idx) as usize)).b32.s1
}

#[inline]
pub unsafe fn set_INTPAR(idx: i32, val: i32) {
    (*eqtb.add((INT_BASE + idx) as usize)).b32.s1 = val;
}

#[inline]
pub unsafe fn LC_CODE(c: i32) -> i32 {
    (*eqtb.add((LC_CODE_BASE + c) as usize)).b32.s1
}

#[inline]
pub unsafe fn SF_CODE(c: i32) -> i32 {
    (*eqtb.add((SF_CODE_BASE + c) as usize)).b32.s1
}

#[inline]
pub unsafe fn TOKS_REG(c: i32) -> i32 {
    (*eqtb.add((TOKS_BASE + c) as usize)).b32.s1
}

#[inline]
pub fn set_family(x: i32) -> i32 {
    (((x as u32) & 0xFF) << 24) as i32
}

#[inline]
pub fn set_class(x: i32) -> i32 {
    (((x as u32) & 0x07) << 21) as i32
}

#[inline]
unsafe fn xmalloc_array<T>(size: usize) -> *mut T {
    xmalloc((size + 1) * std::mem::size_of::<T>()) as *mut T
}

#[inline]
unsafe fn xcalloc_array<T>(size: usize) -> *mut T {
    xcalloc(size + 1, std::mem::size_of::<T>()) as *mut T
}

// ---------------------------------------------------------------------------
// 1. swap_items
// ---------------------------------------------------------------------------

#[cfg(target_endian = "big")]
#[unsafe(export_name = "swap_items")]
pub unsafe extern "C" fn swap_items(_p: *mut c_char, _nitems: usize, _size: usize) {}

#[cfg(not(target_endian = "big"))]
#[unsafe(export_name = "swap_items")]
pub unsafe extern "C" fn swap_items(p: *mut c_char, nitems: usize, size: usize) {
    if !matches!(size, 1 | 2 | 4 | 8 | 16) {
        _tt_abort(c"can't swap a %zu-byte item for (un)dumping".as_ptr(), size);
    }
    if size == 1 || nitems == 0 {
        return;
    }
    let bytes = std::slice::from_raw_parts_mut(p.cast::<u8>(), nitems * size);
    if bytes.len() >= 16 * 1024 * 1024 {
        use rayon::prelude::*;
        bytes.par_chunks_mut(256 * 1024).for_each(|chunk| swap_item_bytes(chunk, size));
    } else {
        swap_item_bytes(bytes, size);
    }
}

fn swap_item_bytes(bytes: &mut [u8], size: usize) {
    macro_rules! swap {
        ($ty:ty, $size:literal) => {
            for item in bytes.chunks_exact_mut($size) {
                let value = <$ty>::from_ne_bytes((&*item).try_into().unwrap());
                item.copy_from_slice(&value.swap_bytes().to_ne_bytes());
            }
        };
    }
    match size {
        2 => swap!(u16, 2),
        4 => swap!(u32, 4),
        8 => swap!(u64, 8),
        16 => swap!(u128, 16),
        _ => unreachable!(),
    }
}

// ---------------------------------------------------------------------------
// 2. do_dump
// ---------------------------------------------------------------------------

#[unsafe(export_name = "do_dump")]
pub unsafe extern "C" fn do_dump(p: *mut c_char, item_size: usize, nitems: usize, out_file: rust_output_handle_t) {
    swap_items(p, nitems, item_size);

    let r = ttstub_output_write(out_file, p, item_size * nitems);
    if r != item_size * nitems {
        _tt_abort(c"could not write %zu %zu-byte item(s) to %s".as_ptr(), nitems, item_size, name_of_file);
    }

    swap_items(p, nitems, item_size);
}

// ---------------------------------------------------------------------------
// 3. do_undump
// ---------------------------------------------------------------------------

#[unsafe(export_name = "do_undump")]
pub unsafe extern "C" fn do_undump(p: *mut c_char, item_size: usize, nitems: usize, in_file: rust_input_handle_t) {
    let r = ttstub_input_read(in_file, p, item_size * nitems);
    if r < 0 || (r as usize) != item_size * nitems {
        _tt_abort(c"could not undump %zu %zu-byte item(s) from %s".as_ptr(), nitems, item_size, name_of_file);
    }

    swap_items(p, nitems, item_size);
}

#[inline]
unsafe fn dump_slice<T>(ptr: *mut T, len: usize, fmt_out: rust_output_handle_t) {
    do_dump(ptr as *mut c_char, std::mem::size_of::<T>(), len, fmt_out);
}

#[inline]
unsafe fn undump_slice<T>(ptr: *mut T, len: usize, fmt_in: rust_input_handle_t) {
    do_undump(ptr as *mut c_char, std::mem::size_of::<T>(), len, fmt_in);
}

#[inline]
unsafe fn dump_int(x: i32, fmt_out: rust_output_handle_t) {
    let mut val = x;
    do_dump((&raw mut val) as *mut c_char, std::mem::size_of::<i32>(), 1, fmt_out);
}

#[inline]
unsafe fn undump_int(fmt_in: rust_input_handle_t) -> i32 {
    let mut val = 0i32;
    do_undump((&raw mut val) as *mut c_char, std::mem::size_of::<i32>(), 1, fmt_in);
    val
}

#[inline]
unsafe fn dump_b32(x: b32x2, fmt_out: rust_output_handle_t) {
    let mut val = x;
    do_dump((&raw mut val) as *mut c_char, std::mem::size_of::<b32x2>(), 1, fmt_out);
}

#[inline]
unsafe fn undump_b32(fmt_in: rust_input_handle_t) -> b32x2 {
    let mut val = std::mem::zeroed::<b32x2>();
    do_undump((&raw mut val) as *mut c_char, std::mem::size_of::<b32x2>(), 1, fmt_in);
    val
}

macro_rules! undump_checked_things {
    ($low:expr, $high:expr, $base:expr, $len:expr, $fmt_in:expr) => {{
        let len = ($len) as usize;
        let local_base_ptr = $base;
        let low = $low;
        let high = $high;
        undump_slice(local_base_ptr, len, $fmt_in);
        for i in 0..len {
            let item = *local_base_ptr.add(i);
            if item < low || item > high {
                _tt_abort(c"item %u of .fmt array out of bounds".as_ptr(), i as libc::c_uint);
            }
        }
    }};
}

macro_rules! undump_upper_check_things {
    ($high:expr, $base:expr, $len:expr, $fmt_in:expr) => {{
        let len = ($len) as usize;
        let local_base_ptr = $base;
        let high = $high;
        undump_slice(local_base_ptr, len, $fmt_in);
        for i in 0..len {
            let item = *local_base_ptr.add(i);
            if i64::from(item) > i64::from(high) {
                _tt_abort(
                    c"item %u of .fmt array %s exceeds upper bound (%lld > %lld)".as_ptr(),
                    i as libc::c_uint,
                    concat!(stringify!($base), "\0").as_ptr().cast::<c_char>(),
                    i64::from(item),
                    i64::from(high),
                );
            }
        }
    }};
}

// ---------------------------------------------------------------------------
// 4. new_patterns
// ---------------------------------------------------------------------------

#[unsafe(export_name = "new_patterns")]
pub unsafe extern "C" fn new_patterns() {
    let mut k: i16;
    let mut l: i16;
    let mut digit_sensed: bool;
    let mut v: trie_opcode;
    let mut p: trie_pointer;
    let mut q: trie_pointer;
    let mut first_child: bool;
    let mut c: UTF16_code;

    if trie_not_ready {
        if INTPAR(INT_PAR__language) <= 0 {
            cur_lang = 0;
        } else if INTPAR(INT_PAR__language) > BIGGEST_LANG {
            cur_lang = 0;
        } else {
            cur_lang = INTPAR(INT_PAR__language) as u8;
        }

        scan_left_brace();
        k = 0;
        hyf[0] = 0;
        digit_sensed = false;

        'outer: loop {
            get_x_token();

            match i32::from(cur_cmd) {
                LETTER | OTHER_CHAR => {
                    if digit_sensed || cur_chr < (b'0' as i32) || cur_chr > (b'9' as i32) {
                        if cur_chr == (b'.' as i32) {
                            cur_chr = 0;
                        } else {
                            cur_chr = LC_CODE(cur_chr);

                            if cur_chr == 0 {
                                error_here_with_diagnostic(c"Nonletter".as_ptr());
                                capture_to_diagnostic(std::ptr::null_mut());
                                help_ptr = 1;
                                help_line[0] = c"(See Appendix H.)".as_ptr();
                                error();
                            }
                        }

                        if cur_chr > max_hyph_char {
                            max_hyph_char = cur_chr;
                        }

                        if (k as i32) < max_hyphenatable_length() {
                            k += 1;
                            hc[k as usize] = cur_chr;
                            hyf[k as usize] = 0;
                            digit_sensed = false;
                        }
                    } else if (k as i32) < max_hyphenatable_length() {
                        hyf[k as usize] = (cur_chr - 48) as u8;
                        digit_sensed = true;
                    }
                }

                SPACER | RIGHT_BRACE => {
                    if k > 0 {
                        if hc[1] == 0 {
                            hyf[0] = 0;
                        }
                        if hc[k as usize] == 0 {
                            hyf[k as usize] = 0;
                        }

                        l = k;
                        v = MIN_TRIE_OP as trie_opcode;

                        loop {
                            if hyf[l as usize] != 0 {
                                v = new_trie_op((k - l) as small_number, hyf[l as usize] as small_number, v);
                            }
                            if l > 0 {
                                l -= 1;
                            } else {
                                break;
                            }
                        }

                        q = 0;
                        hc[0] = cur_lang as i32;

                        while l <= k {
                            c = hc[l as usize] as UTF16_code;
                            l += 1;
                            p = *trie_l.add(q as usize);
                            first_child = true;

                            while p > 0 && c > *trie_c.add(p as usize) {
                                q = p;
                                p = *trie_r.add(q as usize);
                                first_child = false;
                            }

                            if p == 0 || c < *trie_c.add(p as usize) {
                                if trie_ptr == trie_size {
                                    overflow(c"pattern memory".as_ptr(), trie_size);
                                }
                                trie_ptr += 1;
                                *trie_r.add(trie_ptr as usize) = p;
                                p = trie_ptr;
                                *trie_l.add(p as usize) = 0;
                                if first_child {
                                    *trie_l.add(q as usize) = p;
                                } else {
                                    *trie_r.add(q as usize) = p;
                                }
                                *trie_c.add(p as usize) = c;
                                *trie_o.add(p as usize) = MIN_TRIE_OP as trie_opcode;
                            }

                            q = p;
                        }

                        if *trie_o.add(q as usize) != MIN_TRIE_OP as trie_opcode {
                            error_here_with_diagnostic(c"Duplicate pattern".as_ptr());
                            capture_to_diagnostic(std::ptr::null_mut());
                            help_ptr = 1;
                            help_line[0] = c"(See Appendix H.)".as_ptr();
                            error();
                        }

                        *trie_o.add(q as usize) = v;
                    }

                    if cur_cmd == (RIGHT_BRACE) as u8 {
                        break 'outer;
                    }

                    k = 0;
                    hyf[0] = 0;
                    digit_sensed = false;
                }

                _ => {
                    error_here_with_diagnostic(c"Bad ".as_ptr());
                    print_esc_cstr(c"patterns".as_ptr());
                    capture_to_diagnostic(std::ptr::null_mut());
                    help_ptr = 1;
                    help_line[0] = c"(See Appendix H.)".as_ptr();
                    error();
                }
            }
        }

        if INTPAR(INT_PAR__saving_hyph_codes) > 0 {
            c = cur_lang as UTF16_code;
            first_child = false;
            p = 0;

            loop {
                q = p;
                p = *trie_r.add(q as usize);
                if p == 0 || c <= *trie_c.add(p as usize) {
                    break;
                }
            }

            if p == 0 || c < *trie_c.add(p as usize) {
                if trie_ptr == trie_size {
                    overflow(c"pattern memory".as_ptr(), trie_size);
                }
                trie_ptr += 1;
                *trie_r.add(trie_ptr as usize) = p;
                p = trie_ptr;
                *trie_l.add(p as usize) = 0;
                if first_child {
                    *trie_l.add(q as usize) = p;
                } else {
                    *trie_r.add(q as usize) = p;
                }
                *trie_c.add(p as usize) = c;
                *trie_o.add(p as usize) = MIN_TRIE_OP as trie_opcode;
            }

            q = p;
            p = *trie_l.add(q as usize);
            first_child = true;

            for c_val in 0..=255 {
                c = c_val as UTF16_code;
                if LC_CODE(c as i32) > 0 || (c == 255 && first_child) {
                    if p == 0 {
                        if trie_ptr == trie_size {
                            overflow(c"pattern memory".as_ptr(), trie_size);
                        }
                        trie_ptr += 1;
                        *trie_r.add(trie_ptr as usize) = p;
                        p = trie_ptr;
                        *trie_l.add(p as usize) = 0;
                        if first_child {
                            *trie_l.add(q as usize) = p;
                        } else {
                            *trie_r.add(q as usize) = p;
                        }
                        *trie_c.add(p as usize) = c;
                        *trie_o.add(p as usize) = MIN_TRIE_OP as trie_opcode;
                    } else {
                        *trie_c.add(p as usize) = c;
                    }

                    *trie_o.add(p as usize) = LC_CODE(c as i32) as trie_opcode;
                    q = p;
                    p = *trie_r.add(q as usize);
                    first_child = false;
                }
            }

            if first_child {
                *trie_l.add(q as usize) = 0;
            } else {
                *trie_r.add(q as usize) = 0;
            }
        }
    } else {
        error_here_with_diagnostic(c"Too late for ".as_ptr());
        print_esc_cstr(c"patterns".as_ptr());
        capture_to_diagnostic(std::ptr::null_mut());
        help_ptr = 1;
        help_line[0] = c"All patterns must be given before typesetting begins.".as_ptr();
        error();

        (*mem.add(GARBAGE as usize)).b32.s1 = scan_toks(false, false);
        flush_list(def_ref);
    }
}

// ---------------------------------------------------------------------------
// 5. init_trie
// ---------------------------------------------------------------------------

#[unsafe(export_name = "init_trie")]
pub unsafe extern "C" fn init_trie() {
    let mut p: trie_pointer;
    let mut j: i32;
    let mut k: i32;
    let mut t: i32;
    let mut r: trie_pointer;
    let mut s: trie_pointer;

    max_hyph_char += 1;
    op_start[0] = -(MIN_TRIE_OP as i32);

    for j_val in 1..=BIGGEST_LANG {
        op_start[j_val as usize] = op_start[(j_val - 1) as usize] + trie_used[(j_val - 1) as usize] as i32;
    }

    for j_val in 1..=trie_op_ptr {
        *TRIE_OP_HASH(j_val) = op_start[trie_op_lang[j_val as usize] as usize] + trie_op_val[j_val as usize] as i32;
    }

    for j_val in 1..=trie_op_ptr {
        while *TRIE_OP_HASH(j_val) > j_val {
            k = *TRIE_OP_HASH(j_val);
            t = hyf_distance[k as usize] as i32;
            hyf_distance[k as usize] = hyf_distance[j_val as usize];
            hyf_distance[j_val as usize] = t as small_number;

            t = hyf_num[k as usize] as i32;
            hyf_num[k as usize] = hyf_num[j_val as usize];
            hyf_num[j_val as usize] = t as small_number;

            t = hyf_next[k as usize] as i32;
            hyf_next[k as usize] = hyf_next[j_val as usize];
            hyf_next[j_val as usize] = t as trie_opcode;

            *TRIE_OP_HASH(j_val) = *TRIE_OP_HASH(k);
            *TRIE_OP_HASH(k) = k;
        }
    }

    for p_val in 0..=trie_size {
        *trie_hash.add(p_val as usize) = 0;
    }

    *trie_r.add(0) = compress_trie(*trie_r.add(0));
    *trie_l.add(0) = compress_trie(*trie_l.add(0));

    for p_val in 0..=trie_ptr {
        *trie_hash.add(p_val as usize) = 0;
    }

    for p_val in 0..=BIGGEST_CHAR {
        trie_min[p_val as usize] = (p_val + 1) as trie_pointer;
    }

    *trie_trl.add(0) = 1;
    trie_max = 0;

    if *trie_l.add(0) != 0 {
        first_fit(*trie_l.add(0));
        trie_pack(*trie_l.add(0));
    }

    if *trie_r.add(0) != 0 {
        if *trie_l.add(0) == 0 {
            for p_val in 0..=255 {
                trie_min[p_val as usize] = (p_val + 2) as trie_pointer;
            }
        }
        first_fit(*trie_r.add(0));
        trie_pack(*trie_r.add(0));
        hyph_start = *trie_hash.add(*trie_r.add(0) as usize);
    }

    if trie_max == 0 {
        for r_val in 0..=max_hyph_char {
            *trie_trl.add(r_val as usize) = 0;
            *trie_tro.add(r_val as usize) = MIN_TRIE_OP as trie_pointer;
            *trie_trc.add(r_val as usize) = 0;
        }
        trie_max = max_hyph_char;
    } else {
        if *trie_r.add(0) > 0 {
            trie_fix(*trie_r.add(0));
        }
        if *trie_l.add(0) > 0 {
            trie_fix(*trie_l.add(0));
        }
        r = 0;
        loop {
            s = *trie_trl.add(r as usize);
            *trie_trl.add(r as usize) = 0;
            *trie_tro.add(r as usize) = MIN_TRIE_OP as trie_pointer;
            *trie_trc.add(r as usize) = 0;
            r = s;
            if r > trie_max {
                break;
            }
        }
    }

    *trie_trc.add(0) = b'?' as u16;
    trie_not_ready = false;
}

// ---------------------------------------------------------------------------
// 6. new_hyph_exceptions
// ---------------------------------------------------------------------------

#[unsafe(export_name = "new_hyph_exceptions")]
pub unsafe extern "C" fn new_hyph_exceptions() {
    let mut n: i16;
    let mut j: i16;
    let mut h: hyph_pointer;
    let mut k: str_number;
    let mut p: i32;
    let mut q: i32;
    let mut s: str_number;
    let mut u: pool_pointer;
    let mut v: pool_pointer;

    scan_left_brace();

    if INTPAR(INT_PAR__language) <= 0 {
        cur_lang = 0;
    } else if INTPAR(INT_PAR__language) > BIGGEST_LANG {
        cur_lang = 0;
    } else {
        cur_lang = INTPAR(INT_PAR__language) as u8;
    }

    if trie_not_ready {
        hyph_index = 0;
    } else if *trie_trc.add((hyph_start + cur_lang as i32) as usize) != cur_lang as u16 {
        hyph_index = 0;
    } else {
        hyph_index = *trie_trl.add((hyph_start + cur_lang as i32) as usize);
    }

    n = 0;
    p = TEX_NULL;

    loop {
        get_x_token();

        'reswitch: loop {
            match i32::from(cur_cmd) {
                LETTER | OTHER_CHAR | CHAR_GIVEN => {
                    if cur_chr == (b'-' as i32) {
                        if (n as i32) < max_hyphenatable_length() {
                            q = get_avail();
                            (*mem.add(q as usize)).b32.s1 = p;
                            (*mem.add(q as usize)).b32.s0 = n as i32;
                            p = q;
                        }
                    } else {
                        if hyph_index == 0 || cur_chr > 255 {
                            hc[0] = LC_CODE(cur_chr);
                        } else if *trie_trc.add((hyph_index + cur_chr) as usize) != cur_chr as u16 {
                            hc[0] = 0;
                        } else {
                            hc[0] = *trie_tro.add((hyph_index + cur_chr) as usize);
                        }

                        if hc[0] == 0 {
                            error_here_with_diagnostic(c"Not a letter".as_ptr());
                            capture_to_diagnostic(std::ptr::null_mut());
                            help_ptr = 2;
                            help_line[1] = c"Letters in \\hyphenation words must have \\lccode>0.".as_ptr();
                            help_line[0] = c"Proceed; I'll ignore the character I just read.".as_ptr();
                            error();
                        } else if (n as i32) < max_hyphenatable_length() {
                            n += 1;

                            if hc[0] < 65536 {
                                hc[n as usize] = hc[0];
                            } else {
                                hc[n as usize] = (hc[0] - 65536) / 1024 + 55296;
                                n += 1;
                                hc[n as usize] = hc[0] % 1024 + 56320;
                            }
                        }
                    }
                    break 'reswitch;
                }

                CHAR_NUM => {
                    scan_char_num();
                    cur_chr = cur_val;
                    cur_cmd = (CHAR_GIVEN) as u8;
                    continue 'reswitch;
                }

                SPACER | RIGHT_BRACE => {
                    if n > 1 {
                        n += 1;
                        hc[n as usize] = cur_lang as i32;
                        if pool_ptr + (n as i32) > pool_size {
                            overflow(c"pool size".as_ptr(), pool_size - init_pool_ptr);
                        }
                        h = 0;

                        for j_idx in 1..=n {
                            h = ((h + h + (hc[j_idx as usize] as hyph_pointer)) % (HYPH_PRIME as hyph_pointer));
                            *str_pool.add(pool_ptr as usize) = hc[j_idx as usize] as packed_UTF16_code;
                            pool_ptr += 1;
                        }

                        s = make_string();

                        if hyph_next <= HYPH_PRIME {
                            while hyph_next > 0 && *hyph_word.add((hyph_next - 1) as usize) > 0 {
                                hyph_next -= 1;
                            }
                        }

                        if hyph_count == hyph_size || hyph_next == 0 {
                            overflow(c"exception dictionary".as_ptr(), hyph_size);
                        }

                        hyph_count += 1;

                        'search: while *hyph_word.add(h as usize) != 0 {
                            k = *hyph_word.add(h as usize);
                            if length(k) != length(s) {
                                // goto not_found
                            } else {
                                u = *str_start.add((k - 65536) as usize);
                                v = *str_start.add((s - 65536) as usize);

                                let mut matches = true;
                                loop {
                                    if *str_pool.add(u as usize) != *str_pool.add(v as usize) {
                                        matches = false;
                                        break;
                                    }
                                    u += 1;
                                    v += 1;
                                    if u == *str_start.add(((k + 1) - 65536) as usize) {
                                        break;
                                    }
                                }

                                if matches {
                                    str_ptr -= 1;
                                    pool_ptr = *str_start.add((str_ptr - TOO_BIG_CHAR) as usize);
                                    s = *hyph_word.add(h as usize);
                                    hyph_count -= 1;
                                    break 'search;
                                }
                            }

                            // not_found:
                            if *hyph_link.add(h as usize) == 0 {
                                *hyph_link.add(h as usize) = (hyph_next) as u16;
                                if hyph_next >= hyph_size {
                                    hyph_next = HYPH_PRIME;
                                }
                                if hyph_next > HYPH_PRIME {
                                    hyph_next += 1;
                                }
                            }
                            h = (*hyph_link.add(h as usize) - 1) as hyph_pointer;
                        }

                        // found:
                        *hyph_word.add(h as usize) = s;
                        *hyph_list.add(h as usize) = p;
                    }

                    if cur_cmd == (RIGHT_BRACE) as u8 {
                        return;
                    }

                    n = 0;
                    p = TEX_NULL;
                    break 'reswitch;
                }

                _ => {
                    error_here_with_diagnostic(c"Improper ".as_ptr());
                    print_esc_cstr(c"hyphenation".as_ptr());
                    print_cstr(c" will be flushed".as_ptr());
                    capture_to_diagnostic(std::ptr::null_mut());
                    help_ptr = 2;
                    help_line[1] = c"Hyphenation exceptions must contain only letters".as_ptr();
                    help_line[0] = c"and hyphens. But continue; I'll forgive and forget.".as_ptr();
                    error();
                    break 'reswitch;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 7. prefixed_command
// ---------------------------------------------------------------------------

#[unsafe(export_name = "prefixed_command")]
pub unsafe extern "C" fn prefixed_command() {
    let mut a: small_number;
    let mut f: internal_font_number;
    let mut j: i32;
    let mut k: font_index;
    let mut p: i32;
    let mut q: i32;
    let mut n: i32;
    let mut e: bool;

    a = 0;

    while cur_cmd == (PREFIX) as u8 {
        if ((a / cur_chr as small_number) % 2) == 0 {
            a += cur_chr as small_number;
        }

        loop {
            get_x_token();
            if cur_cmd != (SPACER) as u8 && cur_cmd != (RELAX) as u8 {
                break;
            }
        }

        if (cur_cmd as i32) <= MAX_NON_PREFIXED_COMMAND {
            error_here_with_diagnostic(c"You can't use a prefix with `".as_ptr());
            print_cmd_chr((cur_cmd) as u16, cur_chr);
            print_char(b'\'' as i32);
            capture_to_diagnostic(std::ptr::null_mut());
            help_ptr = 1;
            help_line[0] = c"I'll pretend you didn't say \\long or \\outer or \\global or \\protected.".as_ptr();
            back_error();
            return;
        }

        if INTPAR(INT_PAR__tracing_commands) > 2 {
            show_cur_cmd_chr();
        }
    }

    if a >= 8 {
        j = PROTECTED_TOKEN;
        a -= 8;
    } else {
        j = 0;
    }

    if cur_cmd != (DEF) as u8 && (a % 4 != 0 || j != 0) {
        error_here_with_diagnostic(c"You can't use `".as_ptr());
        print_esc_cstr(c"long".as_ptr());
        print_cstr(c"' or `".as_ptr());
        print_esc_cstr(c"outer".as_ptr());
        print_cstr(c"' or `".as_ptr());
        print_esc_cstr(c"protected".as_ptr());
        print_cstr(c"' with `".as_ptr());
        print_cmd_chr((cur_cmd) as u16, cur_chr);
        print_char(b'\'' as i32);
        capture_to_diagnostic(std::ptr::null_mut());
        help_ptr = 1;
        help_line[0] = c"I'll pretend you didn't say \\long or \\outer or \\protected here.".as_ptr();
        error();
    }

    if INTPAR(INT_PAR__global_defs) != 0 {
        if INTPAR(INT_PAR__global_defs) < 0 {
            if a >= 4 {
                a -= 4;
            }
        } else {
            if a < 4 {
                a += 4;
            }
        }
    }

    match i32::from(cur_cmd) {
        SET_FONT => {
            if a >= 4 {
                geq_define(CUR_FONT_LOC, (DATA) as u16, cur_chr);
            } else {
                eq_define(CUR_FONT_LOC, (DATA) as u16, cur_chr);
            }
        }

        DEF => {
            if (cur_chr % 2 != 0) && a < 4 && INTPAR(INT_PAR__global_defs) >= 0 {
                a += 4;
            }

            e = cur_chr >= 2;
            get_r_token();
            p = cur_cs;
            q = scan_toks(true, e);

            if j != 0 {
                q = get_avail();
                (*mem.add(q as usize)).b32.s0 = j;
                (*mem.add(q as usize)).b32.s1 = (*mem.add(def_ref as usize)).b32.s1;
                (*mem.add(def_ref as usize)).b32.s1 = q;
            }

            if a >= 4 {
                geq_define(p, (CALL + i32::from(a % 4)) as u16, def_ref);
            } else {
                eq_define(p, (CALL + i32::from(a % 4)) as u16, def_ref);
            }
        }

        LET => {
            n = cur_chr;
            get_r_token();
            p = cur_cs;

            if n == NORMAL as i32 {
                loop {
                    get_token();
                    if cur_cmd != (SPACER) as u8 {
                        break;
                    }
                }

                if cur_tok == (OTHER_TOKEN + b'=' as i32) {
                    get_token();
                    if cur_cmd == (SPACER) as u8 {
                        get_token();
                    }
                }
            } else {
                get_token();
                q = cur_tok;
                get_token();
                back_input();
                cur_tok = q;
                back_input();
            }

            if cur_cmd >= (CALL) as u8 {
                (*mem.add(cur_chr as usize)).b32.s0 += 1;
            } else if cur_cmd == (REGISTER) as u8 || cur_cmd == TOKS_REGISTER as u8 {
                if cur_chr < 0 || cur_chr > 19 {
                    (*mem.add((cur_chr + 1) as usize)).b32.s0 += 1;
                }
            }

            if a >= 4 {
                geq_define(p, (cur_cmd) as u16, cur_chr);
            } else {
                eq_define(p, (cur_cmd) as u16, cur_chr);
            }
        }

        SHORTHAND_DEF => {
            if cur_chr == CHAR_SUB_DEF_CODE {
                error_here_with_diagnostic(c"vestigial MLTeX shorthand encountered??".as_ptr());
                capture_to_diagnostic(std::ptr::null_mut());
                help_ptr = 1;
                help_line[0] = c"This should never happen in Tectonic, where MLTeX has been excised.".as_ptr();
                error();
            } else {
                n = cur_chr;
                get_r_token();
                p = cur_cs;

                if a >= 4 {
                    geq_define(p, (RELAX) as u16, TOO_BIG_USV);
                } else {
                    eq_define(p, (RELAX) as u16, TOO_BIG_USV);
                }

                scan_optional_equals();

                match n {
                    CHAR_DEF_CODE => {
                        scan_usv_num();
                        if a >= 4 {
                            geq_define(p, (CHAR_GIVEN) as u16, cur_val);
                        } else {
                            eq_define(p, (CHAR_GIVEN) as u16, cur_val);
                        }
                    }

                    MATH_CHAR_DEF_CODE => {
                        scan_fifteen_bit_int();
                        if a >= 4 {
                            geq_define(p, (MATH_GIVEN) as u16, cur_val);
                        } else {
                            eq_define(p, (MATH_GIVEN) as u16, cur_val);
                        }
                    }

                    XETEX_MATH_CHAR_NUM_DEF_CODE => {
                        scan_xetex_math_char_int();
                        if a >= 4 {
                            geq_define(p, (XETEX_MATH_GIVEN) as u16, cur_val);
                        } else {
                            eq_define(p, (XETEX_MATH_GIVEN) as u16, cur_val);
                        }
                    }

                    XETEX_MATH_CHAR_DEF_CODE => {
                        scan_math_class_int();
                        n = set_class(cur_val);
                        scan_math_fam_int();
                        n += set_family(cur_val);
                        scan_usv_num();
                        n += cur_val;
                        if a >= 4 {
                            geq_define(p, (XETEX_MATH_GIVEN) as u16, n);
                        } else {
                            eq_define(p, (XETEX_MATH_GIVEN) as u16, n);
                        }
                    }

                    _ => {
                        scan_register_num();

                        if cur_val > 255 {
                            j = n - 2;

                            if j > MU_VAL {
                                j = TOK_VAL;
                            }

                            find_sa_element((j) as i16, cur_val, true);
                            (*mem.add((cur_ptr + 1) as usize)).b32.s0 += 1;

                            if j == TOK_VAL {
                                j = TOKS_REGISTER;
                            } else {
                                j = REGISTER as i32;
                            }

                            if a >= 4 {
                                geq_define(p, (j as u8) as u16, cur_ptr);
                            } else {
                                eq_define(p, (j as u8) as u16, cur_ptr);
                            }
                        } else {
                            match n {
                                COUNT_DEF_CODE => {
                                    if a >= 4 {
                                        geq_define(p, (ASSIGN_INT) as u16, COUNT_BASE + cur_val);
                                    } else {
                                        eq_define(p, (ASSIGN_INT) as u16, COUNT_BASE + cur_val);
                                    }
                                }
                                DIMEN_DEF_CODE => {
                                    if a >= 4 {
                                        geq_define(p, (ASSIGN_DIMEN) as u16, SCALED_BASE + cur_val);
                                    } else {
                                        eq_define(p, (ASSIGN_DIMEN) as u16, SCALED_BASE + cur_val);
                                    }
                                }
                                SKIP_DEF_CODE => {
                                    if a >= 4 {
                                        geq_define(p, (ASSIGN_GLUE) as u16, SKIP_BASE + cur_val);
                                    } else {
                                        eq_define(p, (ASSIGN_GLUE) as u16, SKIP_BASE + cur_val);
                                    }
                                }
                                MU_SKIP_DEF_CODE => {
                                    if a >= 4 {
                                        geq_define(p, (ASSIGN_MU_GLUE) as u16, MU_SKIP_BASE + cur_val);
                                    } else {
                                        eq_define(p, (ASSIGN_MU_GLUE) as u16, MU_SKIP_BASE + cur_val);
                                    }
                                }
                                TOKS_DEF_CODE => {
                                    if a >= 4 {
                                        geq_define(p, (ASSIGN_TOKS) as u16, TOKS_BASE + cur_val);
                                    } else {
                                        eq_define(p, (ASSIGN_TOKS) as u16, TOKS_BASE + cur_val);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        READ_TO_CS => {
            j = cur_chr;
            scan_int();
            n = cur_val;
            if !scan_keyword(c"to".as_ptr()) {
                error_here_with_diagnostic(c"Missing `to' inserted".as_ptr());
                capture_to_diagnostic(std::ptr::null_mut());
                help_ptr = 2;
                help_line[1] = c"You should have said `\\read<number> to \\cs'.".as_ptr();
                help_line[0] = c"I'm going to look for the \\cs now.".as_ptr();
                error();
            }

            get_r_token();
            p = cur_cs;
            read_toks(n, p, j);

            if a >= 4 {
                geq_define(p, (CALL) as u16, cur_val);
            } else {
                eq_define(p, (CALL) as u16, cur_val);
            }
        }

        _ if cur_cmd == TOKS_REGISTER as u8 || cur_cmd == ASSIGN_TOKS as u8 => {
            q = cur_cs;
            e = false;

            if cur_cmd == TOKS_REGISTER as u8 {
                if cur_chr == 0 {
                    scan_register_num();
                    if cur_val > 255 {
                        find_sa_element((TOK_VAL) as i16, cur_val, true);
                        cur_chr = cur_ptr;
                        e = true;
                    } else {
                        cur_chr = TOKS_BASE + cur_val;
                    }
                } else {
                    e = true;
                }
            } else if cur_chr == LOCAL_BASE + LOCAL__xetex_inter_char_toks {
                scan_char_class_not_ignored();
                cur_ptr = cur_val;
                scan_char_class_not_ignored();
                find_sa_element((INTER_CHAR_VAL) as i16, cur_ptr * CHAR_CLASS_LIMIT + cur_val, true);
                cur_chr = cur_ptr;
                e = true;
            }

            p = cur_chr;
            scan_optional_equals();

            loop {
                get_x_token();
                if cur_cmd != (SPACER) as u8 && cur_cmd != (RELAX) as u8 {
                    break;
                }
            }

            'toks_done: {
                if cur_cmd != (LEFT_BRACE) as u8 {
                    if cur_cmd == TOKS_REGISTER as u8 || cur_cmd == (ASSIGN_TOKS) as u8 {
                        if cur_cmd == TOKS_REGISTER as u8 {
                            if cur_chr == 0 {
                                scan_register_num();
                                if cur_val < 256 {
                                    q = TOKS_REG(cur_val);
                                } else {
                                    find_sa_element((TOK_VAL) as i16, cur_val, false);
                                    if cur_ptr == TEX_NULL {
                                        q = TEX_NULL;
                                    } else {
                                        q = (*mem.add((cur_ptr + 1) as usize)).b32.s1;
                                    }
                                }
                            } else {
                                q = (*mem.add((cur_chr + 1) as usize)).b32.s1;
                            }
                        } else if cur_chr == LOCAL_BASE + LOCAL__xetex_inter_char_toks {
                            scan_char_class_not_ignored();
                            cur_ptr = cur_val;
                            scan_char_class_not_ignored();
                            find_sa_element((INTER_CHAR_VAL) as i16, cur_ptr * CHAR_CLASS_LIMIT + cur_val, false);
                            if cur_ptr == TEX_NULL {
                                q = TEX_NULL;
                            } else {
                                q = (*mem.add((cur_ptr + 1) as usize)).b32.s1;
                            }
                        } else {
                            q = (*eqtb.add(cur_chr as usize)).b32.s1;
                        }

                        if q == TEX_NULL {
                            if e {
                                if a >= 4 {
                                    gsa_def(p, TEX_NULL);
                                } else {
                                    sa_def(p, TEX_NULL);
                                }
                            } else if a >= 4 {
                                geq_define(p, (UNDEFINED_CS) as u16, TEX_NULL);
                            } else {
                                eq_define(p, (UNDEFINED_CS) as u16, TEX_NULL);
                            }
                        } else {
                            (*mem.add(q as usize)).b32.s0 += 1;
                            if e {
                                if a >= 4 {
                                    gsa_def(p, q);
                                } else {
                                    sa_def(p, q);
                                }
                            } else if a >= 4 {
                                geq_define(p, (CALL) as u16, q);
                            } else {
                                eq_define(p, (CALL) as u16, q);
                            }
                        }

                        break 'toks_done;
                    }
                }

                back_input();
                cur_cs = q;
                q = scan_toks(false, false);

                if (*mem.add(def_ref as usize)).b32.s1 == TEX_NULL {
                    if e {
                        if a >= 4 {
                            gsa_def(p, TEX_NULL);
                        } else {
                            sa_def(p, TEX_NULL);
                        }
                    } else if a >= 4 {
                        geq_define(p, (UNDEFINED_CS) as u16, TEX_NULL);
                    } else {
                        eq_define(p, (UNDEFINED_CS) as u16, TEX_NULL);
                    }

                    (*mem.add(def_ref as usize)).b32.s1 = avail;
                    avail = def_ref;
                } else {
                    if p == LOCAL_BASE + LOCAL__output_routine && !e {
                        (*mem.add(q as usize)).b32.s1 = get_avail();
                        q = (*mem.add(q as usize)).b32.s1;
                        (*mem.add(q as usize)).b32.s0 = RIGHT_BRACE_TOKEN + 125;
                        q = get_avail();
                        (*mem.add(q as usize)).b32.s0 = LEFT_BRACE_TOKEN + 123;
                        (*mem.add(q as usize)).b32.s1 = (*mem.add(def_ref as usize)).b32.s1;
                        (*mem.add(def_ref as usize)).b32.s1 = q;
                    }

                    if e {
                        if a >= 4 {
                            gsa_def(p, def_ref);
                        } else {
                            sa_def(p, def_ref);
                        }
                    } else if a >= 4 {
                        geq_define(p, (CALL) as u16, def_ref);
                    } else {
                        eq_define(p, (CALL) as u16, def_ref);
                    }
                }
            }
        }

        ASSIGN_INT => {
            p = cur_chr;
            scan_optional_equals();
            scan_int();
            if a >= 4 {
                geq_word_define(p, cur_val);
            } else {
                eq_word_define(p, cur_val);
            }
        }

        ASSIGN_DIMEN => {
            p = cur_chr;
            scan_optional_equals();
            scan_dimen(false, false, false);
            if a >= 4 {
                geq_word_define(p, cur_val);
            } else {
                eq_word_define(p, cur_val);
            }
        }

        ASSIGN_GLUE | ASSIGN_MU_GLUE => {
            p = cur_chr;
            n = cur_cmd as i32;
            scan_optional_equals();
            if n == ASSIGN_MU_GLUE as i32 {
                scan_glue((MU_VAL) as u8);
            } else {
                scan_glue((GLUE_VAL) as u8);
            }
            trap_zero_glue();
            if a >= 4 {
                geq_define(p, (GLUE_REF) as u16, cur_val);
            } else {
                eq_define(p, (GLUE_REF) as u16, cur_val);
            }
        }

        XETEX_DEF_CODE => {
            if cur_chr == SF_CODE_BASE {
                p = cur_chr;
                scan_usv_num();
                p += cur_val;
                n = SF_CODE(cur_val) % 65536;
                scan_optional_equals();
                scan_char_class();
                if a >= 4 {
                    geq_define(p, (DATA) as u16, cur_val * 65536 + n);
                } else {
                    eq_define(p, (DATA) as u16, cur_val * 65536 + n);
                }
            } else if cur_chr == MATH_CODE_BASE {
                p = cur_chr;
                scan_usv_num();
                p += cur_val;
                scan_optional_equals();
                scan_xetex_math_char_int();
                if a >= 4 {
                    geq_define(p, (DATA) as u16, cur_val);
                } else {
                    eq_define(p, (DATA) as u16, cur_val);
                }
            } else if cur_chr == MATH_CODE_BASE + 1 {
                p = cur_chr - 1;
                scan_usv_num();
                p += cur_val;
                scan_optional_equals();
                scan_math_class_int();
                n = set_class(cur_val);
                scan_math_fam_int();
                n += set_family(cur_val);
                scan_usv_num();
                n += cur_val;
                if a >= 4 {
                    geq_define(p, (DATA) as u16, n);
                } else {
                    eq_define(p, (DATA) as u16, n);
                }
            } else if cur_chr == DEL_CODE_BASE {
                p = cur_chr;
                scan_usv_num();
                p += cur_val;
                scan_optional_equals();
                scan_int();
                if a >= 4 {
                    geq_word_define(p, cur_val);
                } else {
                    eq_word_define(p, cur_val);
                }
            } else {
                p = cur_chr - 1;
                scan_usv_num();
                p += cur_val;
                scan_optional_equals();
                n = 0x40000000;
                scan_math_fam_int();
                n += cur_val * 0x200000;
                scan_usv_num();
                n += cur_val;
                if a >= 4 {
                    geq_word_define(p, n);
                } else {
                    eq_word_define(p, n);
                }
            }
        }

        DEF_CODE => {
            if cur_chr == CAT_CODE_BASE {
                n = MAX_CHAR_CODE;
            } else if cur_chr == MATH_CODE_BASE {
                n = 0x8000;
            } else if cur_chr == SF_CODE_BASE {
                n = 0x7FFF;
            } else if cur_chr == DEL_CODE_BASE {
                n = 0xFFFFFF;
            } else {
                n = BIGGEST_USV;
            }

            p = cur_chr;
            scan_usv_num();
            p += cur_val;
            scan_optional_equals();
            scan_int();

            if (cur_val < 0 && p < DEL_CODE_BASE) || cur_val > n {
                error_here_with_diagnostic(c"Invalid code (".as_ptr());
                print_int(cur_val);
                if p < DEL_CODE_BASE {
                    print_cstr(c"), should be in the range 0..".as_ptr());
                } else {
                    print_cstr(c"), should be at most ".as_ptr());
                }
                print_int(n);
                capture_to_diagnostic(std::ptr::null_mut());
                help_ptr = 1;
                help_line[0] = c"I'm going to use 0 instead of that illegal code value.".as_ptr();
                error();
                cur_val = 0;
            }

            if p < MATH_CODE_BASE {
                if p >= SF_CODE_BASE {
                    n = (*eqtb.add(p as usize)).b32.s1 / 65536;
                    if a >= 4 {
                        geq_define(p, (DATA) as u16, n * 65536 + cur_val);
                    } else {
                        eq_define(p, (DATA) as u16, n * 65536 + cur_val);
                    }
                } else if a >= 4 {
                    geq_define(p, (DATA) as u16, cur_val);
                } else {
                    eq_define(p, (DATA) as u16, cur_val);
                }
            } else if p < DEL_CODE_BASE {
                if cur_val == 32768 {
                    cur_val = ACTIVE_MATH_CHAR;
                } else {
                    cur_val = set_class(cur_val / 4096) + set_family((cur_val % 4096) / 256) + (cur_val % 256);
                }

                if a >= 4 {
                    geq_define(p, (DATA) as u16, cur_val);
                } else {
                    eq_define(p, (DATA) as u16, cur_val);
                }
            } else if a >= 4 {
                geq_word_define(p, cur_val);
            } else {
                eq_word_define(p, cur_val);
            }
        }

        DEF_FAMILY => {
            p = cur_chr;
            scan_math_fam_int();
            p += cur_val;
            scan_optional_equals();
            scan_font_ident();
            if a >= 4 {
                geq_define(p, (DATA) as u16, cur_val);
            } else {
                eq_define(p, (DATA) as u16, cur_val);
            }
        }

        REGISTER | ADVANCE | MULTIPLY | DIVIDE => {
            do_register_command(a);
        }

        SET_BOX => {
            scan_register_num();
            if a >= 4 {
                n = GLOBAL_BOX_FLAG + cur_val;
            } else {
                n = BOX_FLAG + cur_val;
            }

            scan_optional_equals();

            if set_box_allowed {
                scan_box(n);
            } else {
                error_here_with_diagnostic(c"Improper ".as_ptr());
                print_esc_cstr(c"setbox".as_ptr());
                capture_to_diagnostic(std::ptr::null_mut());
                help_ptr = 2;
                help_line[1] = c"Sorry, \\setbox is not allowed after \\halign in a display,".as_ptr();
                help_line[0] = c"or between \\accent and an accented character.".as_ptr();
                error();
            }
        }

        SET_AUX => {
            alter_aux();
        }

        SET_PREV_GRAF => {
            alter_prev_graf();
        }

        SET_PAGE_DIMEN => {
            alter_page_so_far();
        }

        SET_PAGE_INT => {
            alter_integer();
        }

        SET_BOX_DIMEN => {
            alter_box_dimen();
        }

        SET_SHAPE => {
            q = cur_chr;
            scan_optional_equals();
            scan_int();
            n = cur_val;

            if n <= 0 {
                p = TEX_NULL;
            } else if q > LOCAL_BASE + LOCAL__par_shape {
                n = (cur_val / 2) + 1;
                p = get_node(2 * n + 1);
                (*mem.add(p as usize)).b32.s0 = n;
                n = cur_val;
                (*mem.add((p + 1) as usize)).b32.s1 = n;

                for j_idx in (p + 2)..=(p + n + 1) {
                    scan_int();
                    (*mem.add(j_idx as usize)).b32.s1 = cur_val;
                }

                if n % 2 == 0 {
                    (*mem.add((p + n + 2) as usize)).b32.s1 = 0;
                }
            } else {
                p = get_node(2 * n + 1);
                (*mem.add(p as usize)).b32.s0 = n;

                for j_idx in 1..=n {
                    scan_dimen(false, false, false);
                    (*mem.add((p + 2 * j_idx - 1) as usize)).b32.s1 = cur_val;
                    scan_dimen(false, false, false);
                    (*mem.add((p + 2 * j_idx) as usize)).b32.s1 = cur_val;
                }
            }

            if a >= 4 {
                geq_define(q, (SHAPE_REF) as u16, p);
            } else {
                eq_define(q, (SHAPE_REF) as u16, p);
            }
        }

        HYPH_DATA => {
            if cur_chr == 1 {
                if in_initex_mode {
                    new_patterns();
                } else {
                    error_here_with_diagnostic(c"Patterns can be loaded only by INITEX".as_ptr());
                    capture_to_diagnostic(std::ptr::null_mut());
                    help_ptr = 0;
                    error();

                    loop {
                        get_token();
                        if cur_cmd == (RIGHT_BRACE) as u8 {
                            break;
                        }
                    }

                    return;
                }
            } else {
                new_hyph_exceptions();
            }
        }

        ASSIGN_FONT_DIMEN => {
            find_font_dimen(true);
            k = cur_val;
            scan_optional_equals();
            scan_dimen(false, false, false);
            (*font_info.add(k as usize)).b32.s1 = cur_val;
        }

        ASSIGN_FONT_INT => {
            n = cur_chr;
            scan_font_ident();
            f = cur_val;

            if n < 2 {
                scan_optional_equals();
                scan_int();
                if n == 0 {
                    *hyphen_char.add(f as usize) = cur_val;
                } else {
                    *skew_char.add(f as usize) = cur_val;
                }
            } else {
                if *font_area.add(f as usize) == (AAT_FONT_FLAG) as i32 || *font_area.add(f as usize) == (OTGR_FONT_FLAG) as i32 {
                    scan_glyph_number(f);
                } else {
                    scan_char_num();
                }

                p = cur_val;
                scan_optional_equals();
                scan_int();
                match n {
                    LP_CODE_BASE => {
                        set_cp_code(f, p as u32, LEFT_SIDE, cur_val);
                    }
                    RP_CODE_BASE => {
                        set_cp_code(f, p as u32, RIGHT_SIDE, cur_val);
                    }
                    _ => {}
                }
            }
        }

        DEF_FONT => {
            new_font(a);
        }

        SET_INTERACTION => {
            new_interaction();
        }

        _ => {
            confusion(c"prefix".as_ptr());
        }
    }

    if after_token != 0 {
        cur_tok = after_token;
        back_input();
        after_token = 0;
    }
}

// ---------------------------------------------------------------------------
// 8. store_fmt_file
// ---------------------------------------------------------------------------

#[unsafe(export_name = "store_fmt_file")]
pub unsafe extern "C" fn store_fmt_file() {
    let mut j: i32;
    let mut k: i32;
    let mut l: i32;
    let mut p: i32;
    let mut q: i32;
    let mut x: i32;
    let fmt_out: rust_output_handle_t;

    if save_ptr != 0 {
        error_here_with_diagnostic(c"You can't dump inside a group".as_ptr());
        capture_to_diagnostic(std::ptr::null_mut());
        help_ptr = 1;
        help_line[0] = c"`{...\\dump}' is a no-no.".as_ptr();

        if interaction == ERROR_STOP_MODE as u8 {
            interaction = SCROLL_MODE as u8;
        }
        if log_opened {
            error();
        }

        history = super::xetex_xetexd::HISTORY_FATAL_ERROR;
        close_files_and_terminate();
        ttstub_output_flush(rust_stdout);
        _tt_abort(c"\\dump inside a group".as_ptr());
    }

    selector = SELECTOR_NEW_STRING;
    print_cstr(c" (preloaded format=".as_ptr());
    print(job_name);
    print_char(b' ' as i32);
    print_int(INTPAR(INT_PAR__year));
    print_char(b'.' as i32);
    print_int(INTPAR(INT_PAR__month));
    print_char(b'.' as i32);
    print_int(INTPAR(INT_PAR__day));
    print_char(b')' as i32);

    if interaction == BATCH_MODE as u8 {
        selector = SELECTOR_LOG_ONLY;
    } else {
        selector = SELECTOR_TERM_AND_LOG;
    }

    if pool_ptr + 1 > pool_size {
        overflow(c"pool size".as_ptr(), pool_size - init_pool_ptr);
    }

    format_ident = make_string();
    pack_job_name(c".fmt".as_ptr());

    fmt_out = ttstub_output_open(name_of_file, 0);
    if fmt_out == INVALID_HANDLE {
        _tt_abort(c"cannot open format output file \"%s\"".as_ptr(), name_of_file);
    }

    print_nl_cstr(c"Beginning to dump on file ".as_ptr());
    print(make_name_string());

    str_ptr -= 1;
    pool_ptr = *str_start.add((str_ptr - TOO_BIG_CHAR) as usize);

    print_nl_cstr(c"".as_ptr());
    print(format_ident);

    /* Header */

    dump_int(FORMAT_HEADER_MAGIC as i32, fmt_out);
    dump_int((FORMAT_SERIAL) as i32, fmt_out);
    dump_int(hash_high, fmt_out);

    while pseudo_files != TEX_NULL {
        pseudo_close();
    }

    dump_int(MEM_TOP, fmt_out);
    dump_int(EQTB_SIZE, fmt_out);
    dump_int(HASH_PRIME, fmt_out);
    dump_int(HYPH_PRIME, fmt_out);

    /* string pool */

    dump_int(pool_ptr, fmt_out);
    dump_int(str_ptr, fmt_out);
    dump_slice(str_start, (str_ptr - TOO_BIG_CHAR + 1) as usize, fmt_out);
    dump_slice(str_pool, pool_ptr as usize, fmt_out);

    print_ln();
    print_int(str_ptr);
    print_cstr(c" strings of total length ".as_ptr());
    print_int(pool_ptr);

    /* memory locations */

    sort_avail();
    var_used = 0;
    dump_int(lo_mem_max, fmt_out);
    dump_int(rover, fmt_out);

    for k_val in INT_VAL..=INTER_CHAR_VAL {
        dump_int(sa_root[k_val as usize], fmt_out);
    }

    p = 0;
    q = rover;
    x = 0;
    loop {
        dump_slice(mem.add(p as usize), (q + 2 - p) as usize, fmt_out);
        x = x + q + 2 - p;
        var_used = var_used + q - p;
        p = q + (*mem.add(q as usize)).b32.s0;
        q = (*mem.add((q + 1) as usize)).b32.s1;
        if q == rover {
            break;
        }
    }

    var_used = var_used + lo_mem_max - p;
    dyn_used = mem_end + 1 - hi_mem_min;
    dump_slice(mem.add(p as usize), (lo_mem_max + 1 - p) as usize, fmt_out);

    x = x + lo_mem_max + 1 - p;
    dump_int(hi_mem_min, fmt_out);
    dump_int(avail, fmt_out);
    dump_slice(mem.add(hi_mem_min as usize), (mem_end + 1 - hi_mem_min) as usize, fmt_out);

    x = x + mem_end + 1 - hi_mem_min;
    p = avail;
    while p != TEX_NULL {
        dyn_used -= 1;
        p = (*mem.add(p as usize)).b32.s1;
    }

    dump_int(var_used, fmt_out);
    dump_int(dyn_used, fmt_out);

    print_ln();
    print_int(x);
    print_cstr(c" memory locations dumped; current usage is ".as_ptr());
    print_int(var_used);
    print_char(b'&' as i32);
    print_int(dyn_used);

    /* equivalents table / primitive */

    k = ACTIVE_BASE;

    loop {
        j = k;

        'find1: loop {
            if j < INT_BASE - 1 {
                if (*eqtb.add(j as usize)).b32.s1 == (*eqtb.add((j + 1) as usize)).b32.s1
                    && (*eqtb.add(j as usize)).b16.s1 == (*eqtb.add((j + 1) as usize)).b16.s1
                    && (*eqtb.add(j as usize)).b16.s0 == (*eqtb.add((j + 1) as usize)).b16.s0
                {
                    j += 1;
                    l = j;
                    while j < INT_BASE - 1 {
                        if (*eqtb.add(j as usize)).b32.s1 != (*eqtb.add((j + 1) as usize)).b32.s1
                            || (*eqtb.add(j as usize)).b16.s1 != (*eqtb.add((j + 1) as usize)).b16.s1
                            || (*eqtb.add(j as usize)).b16.s0 != (*eqtb.add((j + 1) as usize)).b16.s0
                        {
                            break;
                        }
                        j += 1;
                    }
                    break 'find1;
                }
                j += 1;
            } else {
                l = INT_BASE;
                break 'find1;
            }
        }

        dump_int(l - k, fmt_out);
        dump_slice(eqtb.add(k as usize), (l - k) as usize, fmt_out);
        k = j + 1;
        dump_int(k - l, fmt_out);

        if k == INT_BASE {
            break;
        }
    }

    loop {
        j = k;

        'find2: loop {
            if j < EQTB_SIZE {
                if (*eqtb.add(j as usize)).b32.s1 == (*eqtb.add((j + 1) as usize)).b32.s1 {
                    j += 1;
                    l = j;
                    while j < EQTB_SIZE {
                        if (*eqtb.add(j as usize)).b32.s1 != (*eqtb.add((j + 1) as usize)).b32.s1 {
                            break;
                        }
                        j += 1;
                    }
                    break 'find2;
                }
                j += 1;
            } else {
                l = EQTB_SIZE + 1;
                break 'find2;
            }
        }

        dump_int(l - k, fmt_out);
        dump_slice(eqtb.add(k as usize), (l - k) as usize, fmt_out);
        k = j + 1;
        dump_int(k - l, fmt_out);

        if k > EQTB_SIZE {
            break;
        }
    }

    if hash_high > 0 {
        dump_slice(eqtb.add((EQTB_SIZE + 1) as usize), hash_high as usize, fmt_out);
    }

    dump_int(par_loc, fmt_out);
    dump_int(write_loc, fmt_out);

    dump_slice((&raw mut prim).cast::<b32x2>(), (PRIM_SIZE + 1) as usize, fmt_out);

    /* control sequences */

    dump_int(hash_used, fmt_out);
    cs_count = (FROZEN_CONTROL_SEQUENCE - 1) - hash_used + hash_high;

    for p_idx in HASH_BASE..=hash_used {
        if (*hash.add(p_idx as usize)).s1 != 0 {
            dump_int(p_idx, fmt_out);
            dump_b32(*hash.add(p_idx as usize), fmt_out);
            cs_count += 1;
        }
    }

    dump_slice(
        hash.add((hash_used + 1) as usize),
        ((UNDEFINED_CONTROL_SEQUENCE - 1) - hash_used) as usize,
        fmt_out,
    );
    if hash_high > 0 {
        dump_slice(hash.add((EQTB_SIZE + 1) as usize), hash_high as usize, fmt_out);
    }

    dump_int(cs_count, fmt_out);

    print_ln();
    print_int(cs_count);
    print_cstr(c" multiletter control sequences".as_ptr());

    /* fonts */

    dump_int(fmem_ptr, fmt_out);
    dump_slice(font_info, fmem_ptr as usize, fmt_out);
    dump_int(font_ptr, fmt_out);
    dump_slice(font_check.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(font_size.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(font_dsize.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(font_params.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(hyphen_char.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(skew_char.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(font_name.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(font_area.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(font_bc.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(font_ec.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(char_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(width_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(height_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(depth_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(italic_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(lig_kern_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(kern_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(exten_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(param_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(font_glue.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(bchar_label.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(font_bchar.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);
    dump_slice(font_false_bchar.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_out);

    for k_val in FONT_BASE..=font_ptr {
        print_nl_cstr(c"\\font".as_ptr());
        print_esc((*hash.add((FONT_ID_BASE + k_val) as usize)).s1);
        print_char(b'=' as i32);

        if *font_area.add(k_val as usize) == (AAT_FONT_FLAG) as i32
            || *font_area.add(k_val as usize) == (OTGR_FONT_FLAG) as i32
            || !(*font_mapping.add(k_val as usize)).is_null()
        {
            print_file_name(*font_name.add(k_val as usize), EMPTY_STRING, EMPTY_STRING);

            error_here_with_diagnostic(c"Can't \\dump a format with native fonts or font-mappings".as_ptr());
            capture_to_diagnostic(std::ptr::null_mut());

            help_ptr = 3;
            help_line[2] = c"You really, really don't want to do this.".as_ptr();
            help_line[1] = c"It won't work, and only confuses me.".as_ptr();
            help_line[0] = c"(Load them at runtime, not as part of the format file.)".as_ptr();
            error();
        } else {
            print_file_name(*font_name.add(k_val as usize), *font_area.add(k_val as usize), EMPTY_STRING);
        }

        if *font_size.add(k_val as usize) != *font_dsize.add(k_val as usize) {
            print_cstr(c" at ".as_ptr());
            print_scaled(*font_size.add(k_val as usize));
            print_cstr(c"pt".as_ptr());
        }
    }

    print_ln();
    print_int(fmem_ptr - 7);
    print_cstr(c" words of font info for ".as_ptr());
    print_int(font_ptr);
    if font_ptr != FONT_BASE + 1 {
        print_cstr(c" preloaded fonts".as_ptr());
    } else {
        print_cstr(c" preloaded font".as_ptr());
    }

    /* hyphenation info */

    dump_int(hyph_count, fmt_out);
    if hyph_next <= HYPH_PRIME {
        hyph_next = hyph_size;
    }
    dump_int(hyph_next, fmt_out);

    for k_val in 0..=hyph_size {
        if *hyph_word.add(k_val as usize) != 0 {
            dump_int(k_val + 65536 * (*hyph_link.add(k_val as usize) as i32), fmt_out);
            dump_int(*hyph_word.add(k_val as usize), fmt_out);
            dump_int(*hyph_list.add(k_val as usize), fmt_out);
        }
    }

    print_ln();
    print_int(hyph_count);
    if hyph_count != 1 {
        print_cstr(c" hyphenation exceptions".as_ptr());
    } else {
        print_cstr(c" hyphenation exception".as_ptr());
    }

    if trie_not_ready {
        init_trie();
    }

    dump_int(trie_max, fmt_out);
    dump_int(hyph_start, fmt_out);
    dump_slice(trie_trl, (trie_max + 1) as usize, fmt_out);
    dump_slice(trie_tro, (trie_max + 1) as usize, fmt_out);
    dump_slice(trie_trc, (trie_max + 1) as usize, fmt_out);
    dump_int(max_hyph_char, fmt_out);
    dump_int(trie_op_ptr, fmt_out);
    dump_slice((&raw mut hyf_distance).cast::<small_number>().add(1), trie_op_ptr as usize, fmt_out);
    dump_slice((&raw mut hyf_num).cast::<small_number>().add(1), trie_op_ptr as usize, fmt_out);
    dump_slice((&raw mut hyf_next).cast::<u16>().add(1), trie_op_ptr as usize, fmt_out);

    print_nl_cstr(c"Hyphenation trie of length ".as_ptr());
    print_int(trie_max);
    print_cstr(c" has ".as_ptr());
    print_int(trie_op_ptr);
    if trie_op_ptr != 1 {
        print_cstr(c" ops".as_ptr());
    } else {
        print_cstr(c" op".as_ptr());
    }
    print_cstr(c" out of ".as_ptr());
    print_int(TRIE_OP_SIZE);

    for k_val in (0..=BIGGEST_LANG).rev() {
        if trie_used[k_val as usize] > 0 {
            print_nl_cstr(c"  ".as_ptr());
            print_int(trie_used[k_val as usize] as i32);
            print_cstr(c" for language ".as_ptr());
            print_int(k_val);
            dump_int(k_val, fmt_out);
            dump_int(trie_used[k_val as usize] as i32, fmt_out);
        }
    }

    /* footer */

    dump_int(FORMAT_FOOTER_MAGIC as i32, fmt_out);

    set_INTPAR(INT_PAR__tracing_stats, 0);
    ttstub_output_close(fmt_out);
}

// ---------------------------------------------------------------------------
// 9. pack_buffered_name
// ---------------------------------------------------------------------------

#[unsafe(export_name = "pack_buffered_name")]
pub unsafe extern "C" fn pack_buffered_name(n: small_number, a: i32, b: i32) {
    libc::free(name_of_file as *mut c_void);
    name_of_file = xmalloc_array::<c_char>((format_default_length + 1) as usize);

    libc::strcpy(name_of_file, TEX_format_default);
    name_length = libc::strlen(name_of_file) as i32;
}

// ---------------------------------------------------------------------------
// 10. load_fmt_file
// ---------------------------------------------------------------------------

#[unsafe(export_name = "load_fmt_file")]
pub unsafe extern "C" fn load_fmt_file() -> bool {
    super::xetex_stringpool::reset_string_index();
    let mut j: i32;
    let mut k: i32;
    let mut p: i32;
    let mut q: i32;
    let mut x: i32;
    let fmt_in: rust_input_handle_t;

    j = cur_input.loc;

    pack_buffered_name((format_default_length - 4) as small_number, 1, 0);

    fmt_in = ttstub_input_open(name_of_file, FileFormat::Format as i32, 0);
    if fmt_in == INVALID_HANDLE {
        _tt_abort(c"cannot open the format file \"%s\"".as_ptr(), name_of_file);
    }

    cur_input.loc = j;

    if in_initex_mode {
        libc::free(font_info as *mut c_void);
        libc::free(str_pool as *mut c_void);
        libc::free(str_start as *mut c_void);
        libc::free(yhash as *mut c_void);
        libc::free(eqtb as *mut c_void);
        libc::free(mem as *mut c_void);
        mem = std::ptr::null_mut();
    }

    /* start reading the header */

    x = undump_int(fmt_in);
    if x != FORMAT_HEADER_MAGIC as i32 {
        _tt_abort(c"fatal format file error".as_ptr());
    }

    x = undump_int(fmt_in);
    if x != (FORMAT_SERIAL) as i32 {
        _tt_abort(
            c"format file \"%s\" is of the wrong version: expected %d, found %d".as_ptr(),
            name_of_file,
            FORMAT_SERIAL,
            x,
        );
    }

    /* hash table parameters */

    hash_high = undump_int(fmt_in);
    if hash_high < 0 || hash_high > sup_hash_extra {
        _tt_abort(c"fatal format file error".as_ptr());
    }
    if hash_extra < hash_high {
        hash_extra = hash_high;
    }

    eqtb_top = EQTB_SIZE + hash_extra;
    if hash_extra == 0 {
        hash_top = UNDEFINED_CONTROL_SEQUENCE;
    } else {
        hash_top = eqtb_top;
    }

    yhash = xcalloc_array::<b32x2>((1 + hash_top - hash_offset) as usize);
    hash = yhash.offset(-hash_offset as isize);

    eqtb = xmalloc_array::<memory_word>((eqtb_top + 1) as usize);
    (*eqtb.add(UNDEFINED_CONTROL_SEQUENCE as usize)).b16.s1 = UNDEFINED_CS as u16;
    (*eqtb.add(UNDEFINED_CONTROL_SEQUENCE as usize)).b32.s1 = TEX_NULL;
    (*eqtb.add(UNDEFINED_CONTROL_SEQUENCE as usize)).b16.s0 = LEVEL_ZERO as u16;

    let undefined = *eqtb.add(UNDEFINED_CONTROL_SEQUENCE as usize);
    std::slice::from_raw_parts_mut(eqtb.add((EQTB_SIZE + 1) as usize), hash_extra as usize).fill(undefined);

    max_reg_num = 32767;
    max_reg_help_line = c"A register number must be between 0 and 32767.".as_ptr();

    /* memory locations */

    x = undump_int(fmt_in);
    if x != MEM_TOP {
        _tt_abort(c"fatal format file error".as_ptr());
    }

    cur_list.head = CONTRIB_HEAD;
    cur_list.tail = CONTRIB_HEAD;
    page_tail = PAGE_HEAD;
    mem = xmalloc_array::<memory_word>((MEM_TOP + 1) as usize);

    x = undump_int(fmt_in);
    if x != EQTB_SIZE {
        _tt_abort(c"fatal format file error".as_ptr());
    }

    x = undump_int(fmt_in);
    if x != HASH_PRIME {
        _tt_abort(c"fatal format file error".as_ptr());
    }

    x = undump_int(fmt_in);
    if x != HYPH_PRIME {
        _tt_abort(c"fatal format file error".as_ptr());
    }

    /* string pool */

    x = undump_int(fmt_in);
    if x < 0 {
        _tt_abort(c"fatal format file error".as_ptr());
    }
    if x > sup_pool_size - pool_free {
        _tt_abort(c"must increase string_pool_size".as_ptr());
    }
    pool_ptr = x;

    if pool_size < pool_ptr + pool_free {
        pool_size = pool_ptr + pool_free;
    }

    x = undump_int(fmt_in);
    if x < 0 {
        _tt_abort(c"fatal format file error".as_ptr());
    }
    if x > sup_max_strings - strings_free {
        _tt_abort(c"must increase sup_strings".as_ptr());
    }
    str_ptr = x;

    if max_strings < str_ptr + strings_free {
        max_strings = str_ptr + strings_free;
    }

    str_start = xmalloc_array::<pool_pointer>(max_strings as usize);
    undump_checked_things!(0, pool_ptr, str_start, str_ptr - TOO_BIG_CHAR + 1, fmt_in);
    str_pool = xmalloc_array::<packed_UTF16_code>(pool_size as usize);

    undump_slice(str_pool, pool_ptr as usize, fmt_in);

    init_str_ptr = str_ptr;
    init_pool_ptr = pool_ptr;

    x = undump_int(fmt_in);
    if x < 1019 || x > MEM_TOP - HI_MEM_STAT_USAGE {
        _tt_abort(c"fatal format file error".as_ptr());
    } else {
        lo_mem_max = x;
    }

    x = undump_int(fmt_in);
    if x < 20 || x > lo_mem_max {
        _tt_abort(c"fatal format file error".as_ptr());
    } else {
        rover = x;
    }

    for k_val in INT_VAL..=INTER_CHAR_VAL {
        x = undump_int(fmt_in);
        if x < MIN_HALFWORD || x > lo_mem_max {
            _tt_abort(c"fatal format file error".as_ptr());
        } else {
            sa_root[k_val as usize] = x;
        }
    }

    p = 0;
    q = rover;

    loop {
        undump_slice(mem.add(p as usize), (q + 2 - p) as usize, fmt_in);
        p = q + (*mem.add(q as usize)).b32.s0;
        if p > lo_mem_max || (q >= (*mem.add((q + 1) as usize)).b32.s1 && (*mem.add((q + 1) as usize)).b32.s1 != rover) {
            _tt_abort(c"fatal format file error".as_ptr());
        }
        q = (*mem.add((q + 1) as usize)).b32.s1;
        if q == rover {
            break;
        }
    }

    undump_slice(mem.add(p as usize), (lo_mem_max + 1 - p) as usize, fmt_in);

    x = undump_int(fmt_in);
    if x < lo_mem_max + 1 || x > PRE_ADJUST_HEAD {
        _tt_abort(c"fatal format file error".as_ptr());
    } else {
        hi_mem_min = x;
    }

    x = undump_int(fmt_in);
    if x < MIN_HALFWORD || x > MEM_TOP {
        _tt_abort(c"fatal format file error".as_ptr());
    } else {
        avail = x;
    }

    mem_end = MEM_TOP;

    undump_slice(mem.add(hi_mem_min as usize), (mem_end + 1 - hi_mem_min) as usize, fmt_in);
    var_used = undump_int(fmt_in);
    dyn_used = undump_int(fmt_in);

    /* equivalents table / primitives */

    k = ACTIVE_BASE;

    loop {
        x = undump_int(fmt_in);
        if x < 1 || k + x > EQTB_SIZE + 1 {
            _tt_abort(c"fatal format file error".as_ptr());
        }

        undump_slice(eqtb.add(k as usize), x as usize, fmt_in);
        k += x;

        x = undump_int(fmt_in);
        if x < 0 || k + x > EQTB_SIZE + 1 {
            _tt_abort(c"fatal format file error".as_ptr());
        }

        let value = *eqtb.add((k - 1) as usize);
        std::slice::from_raw_parts_mut(eqtb.add(k as usize), x as usize).fill(value);

        k += x;

        if k > EQTB_SIZE {
            break;
        }
    }

    if hash_high > 0 {
        undump_slice(eqtb.add((EQTB_SIZE + 1) as usize), hash_high as usize, fmt_in);
    }

    x = undump_int(fmt_in);
    if x < HASH_BASE || x > hash_top {
        _tt_abort(c"fatal format file error".as_ptr());
    } else {
        par_loc = x;
    }

    par_token = CS_TOKEN_FLAG + par_loc;

    x = undump_int(fmt_in);
    if x < HASH_BASE || x > hash_top {
        _tt_abort(c"fatal format file error".as_ptr());
    } else {
        write_loc = x;
    }

    /* control sequence names */

    for p_idx in 0..=PRIM_SIZE {
        prim[p_idx as usize] = undump_b32(fmt_in);
    }

    x = undump_int(fmt_in);
    if x < HASH_BASE || x > FROZEN_CONTROL_SEQUENCE {
        _tt_abort(c"fatal format file error".as_ptr());
    } else {
        hash_used = x;
    }

    p = HASH_BASE - 1;

    loop {
        x = undump_int(fmt_in);
        if x < p + 1 || x > hash_used {
            _tt_abort(c"fatal format file error".as_ptr());
        } else {
            p = x;
        }
        *hash.add(p as usize) = undump_b32(fmt_in);
        if p == hash_used {
            break;
        }
    }

    undump_slice(
        hash.add((hash_used + 1) as usize),
        ((UNDEFINED_CONTROL_SEQUENCE - 1) - hash_used) as usize,
        fmt_in,
    );

    if hash_high > 0 {
        undump_slice(hash.add((EQTB_SIZE + 1) as usize), hash_high as usize, fmt_in);
    }

    cs_count = undump_int(fmt_in);

    /* font info */

    x = undump_int(fmt_in);
    if x < 7 {
        _tt_abort(c"fatal format file error".as_ptr());
    }
    if x > sup_font_mem_size {
        _tt_abort(c"must increase font_mem_size".as_ptr());
    }

    fmem_ptr = x;
    if fmem_ptr > font_mem_size {
        font_mem_size = fmem_ptr;
    }

    font_info = xmalloc_array::<memory_word>(font_mem_size as usize);
    undump_slice(font_info, fmem_ptr as usize, fmt_in);

    x = undump_int(fmt_in);
    if x < FONT_BASE {
        _tt_abort(c"fatal format file error".as_ptr());
    }
    if x > FONT_BASE + MAX_FONT_MAX {
        _tt_abort(c"must increase font_max".as_ptr());
    }

    font_ptr = x;

    font_mapping = xcalloc_array::<*mut c_void>(font_max as usize);
    font_layout_engine = xcalloc_array::<*mut c_void>(font_max as usize);
    font_flags = xmalloc_array::<c_char>(font_max as usize);
    font_letter_space = xmalloc_array::<scaled_t>(font_max as usize);
    font_check = xmalloc_array::<b16x4>(font_max as usize);
    font_size = xmalloc_array::<scaled_t>(font_max as usize);
    font_dsize = xmalloc_array::<scaled_t>(font_max as usize);
    font_params = xmalloc_array::<font_index>(font_max as usize);
    font_name = xmalloc_array::<str_number>(font_max as usize);
    font_area = xmalloc_array::<str_number>(font_max as usize);
    font_bc = xmalloc_array::<UTF16_code>(font_max as usize);
    font_ec = xmalloc_array::<UTF16_code>(font_max as usize);
    font_glue = xmalloc_array::<i32>(font_max as usize);
    hyphen_char = xmalloc_array::<i32>(font_max as usize);
    skew_char = xmalloc_array::<i32>(font_max as usize);
    bchar_label = xmalloc_array::<font_index>(font_max as usize);
    font_bchar = xmalloc_array::<nine_bits>(font_max as usize);
    font_false_bchar = xmalloc_array::<nine_bits>(font_max as usize);
    char_base = xmalloc_array::<i32>(font_max as usize);
    width_base = xmalloc_array::<i32>(font_max as usize);
    height_base = xmalloc_array::<i32>(font_max as usize);
    depth_base = xmalloc_array::<i32>(font_max as usize);
    italic_base = xmalloc_array::<i32>(font_max as usize);
    lig_kern_base = xmalloc_array::<i32>(font_max as usize);
    kern_base = xmalloc_array::<i32>(font_max as usize);
    exten_base = xmalloc_array::<i32>(font_max as usize);
    param_base = xmalloc_array::<i32>(font_max as usize);

    for k_idx in FONT_BASE..=font_ptr {
        *font_mapping.add(k_idx as usize) = std::ptr::null_mut();
    }

    undump_slice(font_check.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(font_size.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(font_dsize.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_checked_things!(MIN_HALFWORD, MAX_HALFWORD, font_params.add(FONT_BASE as usize), font_ptr + 1, fmt_in);
    undump_slice(hyphen_char.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(skew_char.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_upper_check_things!(str_ptr, font_name.add(FONT_BASE as usize), font_ptr + 1, fmt_in);
    undump_upper_check_things!(str_ptr, font_area.add(FONT_BASE as usize), font_ptr + 1, fmt_in);
    undump_slice(font_bc.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(font_ec.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(char_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(width_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(height_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(depth_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(italic_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(lig_kern_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(kern_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(exten_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_slice(param_base.add(FONT_BASE as usize), (font_ptr + 1) as usize, fmt_in);
    undump_checked_things!(MIN_HALFWORD, lo_mem_max, font_glue.add(FONT_BASE as usize), font_ptr + 1, fmt_in);
    undump_checked_things!(0, fmem_ptr - 1, bchar_label.add(FONT_BASE as usize), font_ptr + 1, fmt_in);
    undump_checked_things!(0, TOO_BIG_CHAR, font_bchar.add(FONT_BASE as usize), font_ptr + 1, fmt_in);
    undump_checked_things!(0, TOO_BIG_CHAR, font_false_bchar.add(FONT_BASE as usize), font_ptr + 1, fmt_in);

    /* hyphenations */

    x = undump_int(fmt_in);
    if x < 0 {
        _tt_abort(c"fatal format file error".as_ptr());
    }
    if x > hyph_size {
        _tt_abort(c"must increase hyph_size".as_ptr());
    }
    hyph_count = x;

    x = undump_int(fmt_in);
    if x < HYPH_PRIME {
        _tt_abort(c"fatal format file error".as_ptr());
    }
    if x > hyph_size {
        _tt_abort(c"must increase hyph_size".as_ptr());
    }
    hyph_next = x;

    j = 0;

    for k_idx in 1..=hyph_count {
        j = undump_int(fmt_in);
        if j < 0 {
            _tt_abort(c"fatal format file error".as_ptr());
        }
        if j > 65535 {
            hyph_next = j / 65536;
            j -= hyph_next * 65536;
        } else {
            hyph_next = 0;
        }

        if j >= hyph_size || hyph_next > hyph_size {
            _tt_abort(c"fatal format file error".as_ptr());
        }

        *hyph_link.add(j as usize) = (hyph_next) as u16;

        x = undump_int(fmt_in);
        if x < 0 || x > str_ptr {
            _tt_abort(c"fatal format file error".as_ptr());
        } else {
            *hyph_word.add(j as usize) = x;
        }

        x = undump_int(fmt_in);
        if x < MIN_HALFWORD || x > MAX_HALFWORD {
            _tt_abort(c"fatal format file error".as_ptr());
        } else {
            *hyph_list.add(j as usize) = x;
        }
    }

    j += 1;
    if j < HYPH_PRIME {
        j = HYPH_PRIME;
    }

    hyph_next = j;
    if hyph_next >= hyph_size {
        hyph_next = HYPH_PRIME;
    } else if hyph_next >= HYPH_PRIME {
        hyph_next += 1;
    }

    x = undump_int(fmt_in);
    if x < 0 {
        _tt_abort(c"fatal format file error".as_ptr());
    }
    if x > trie_size {
        _tt_abort(c"must increase trie_size".as_ptr());
    }

    j = x;
    trie_max = j;

    x = undump_int(fmt_in);
    if x < 0 || x > j {
        _tt_abort(c"fatal format file error".as_ptr());
    } else {
        hyph_start = x;
    }

    if trie_trl.is_null() {
        trie_trl = xmalloc_array::<trie_pointer>((j + 1) as usize);
    }
    undump_slice(trie_trl, (j + 1) as usize, fmt_in);

    if trie_tro.is_null() {
        trie_tro = xmalloc_array::<trie_pointer>((j + 1) as usize);
    }
    undump_slice(trie_tro, (j + 1) as usize, fmt_in);

    if trie_trc.is_null() {
        trie_trc = xmalloc_array::<u16>((j + 1) as usize);
    }
    undump_slice(trie_trc, (j + 1) as usize, fmt_in);

    max_hyph_char = undump_int(fmt_in);

    x = undump_int(fmt_in);
    if x < 0 {
        _tt_abort(c"fatal format file error".as_ptr());
    }
    if x > TRIE_OP_SIZE {
        _tt_abort(c"must increase TRIE_OP_SIZE".as_ptr());
    }

    j = x;
    trie_op_ptr = j;

    undump_slice((&raw mut hyf_distance).cast::<small_number>().add(1), j as usize, fmt_in);
    undump_slice((&raw mut hyf_num).cast::<small_number>().add(1), j as usize, fmt_in);
    undump_upper_check_things!(MAX_TRIE_OP, (&raw mut hyf_next).cast::<u16>().add(1), j, fmt_in);

    for k_idx in 0..=BIGGEST_LANG {
        trie_used[k_idx as usize] = 0;
    }

    k = BIGGEST_LANG + 1;

    while j > 0 {
        x = undump_int(fmt_in);
        if x < 0 || x > k - 1 {
            _tt_abort(c"fatal format file error".as_ptr());
        } else {
            k = x;
        }

        x = undump_int(fmt_in);
        if x < 1 || x > j {
            _tt_abort(c"fatal format file error".as_ptr());
        }

        trie_used[k as usize] = x as trie_opcode;
        j -= x;
        op_start[k as usize] = j;
    }

    trie_not_ready = false;

    /* trailer */

    x = undump_int(fmt_in);
    if x != FORMAT_FOOTER_MAGIC as i32 {
        _tt_abort(c"fatal format file error".as_ptr());
    }

    ttstub_input_close(fmt_in);
    true
}

#[unsafe(export_name = "tt_xetex_native_load_format")]
pub unsafe extern "C" fn tt_xetex_native_load_format() -> bool {
    load_fmt_file()
}

#[unsafe(export_name = "tt_xetex_native_store_format")]
pub unsafe extern "C" fn tt_xetex_native_store_format() {
    store_fmt_file();
}

// ---------------------------------------------------------------------------
// 11. tt_checkpoint_engine
// ---------------------------------------------------------------------------

#[inline(always)]
unsafe fn cp_val<T>(cp: *mut tt_checkpoint, name: *const c_char, val: *mut T) {
    tt_checkpoint_field(cp, name, val as *mut c_void, std::mem::size_of::<T>());
}

macro_rules! cp_value {
    ($cp:expr, $var:ident) => {
        cp_val($cp, concat!(stringify!($var), "\0").as_ptr() as *const c_char, &raw mut $var);
    };
}

macro_rules! cp_array {
    ($cp:expr, $var:ident, $count:expr) => {
        tt_checkpoint_array(
            $cp,
            concat!(stringify!($var), "\0").as_ptr() as *const c_char,
            (&raw mut $var) as *mut *mut c_void,
            $count as usize,
            checkpoint_element_size($var),
        );
    };
}

macro_rules! cp_used {
    ($cp:expr, $var:ident, $capacity:expr, $used:expr) => {
        tt_checkpoint_used_array(
            $cp,
            concat!(stringify!($var), "\0").as_ptr() as *const c_char,
            (&raw mut $var) as *mut *mut c_void,
            $capacity as usize,
            $used as usize,
            checkpoint_element_size($var),
        );
    };
}

macro_rules! cp_rle {
    ($cp:expr, $var:ident, $count:expr) => {
        tt_checkpoint_rle(
            $cp,
            concat!(stringify!($var), "\0").as_ptr() as *const c_char,
            (&raw mut $var) as *mut *mut c_void,
            $count as usize,
            checkpoint_element_size($var),
        );
    };
}

macro_rules! cp_string {
    ($cp:expr, $var:ident) => {
        tt_checkpoint_string(
            $cp,
            concat!(stringify!($var), "\0").as_ptr() as *const c_char,
            (&raw mut $var) as *mut *mut c_char,
        );
    };
}

fn checkpoint_element_size<T>(_: *const T) -> usize {
    std::mem::size_of::<T>()
}

#[unsafe(export_name = "tt_checkpoint_engine")]
pub unsafe extern "C" fn tt_checkpoint_engine(cp: *mut tt_checkpoint) {
    let cp_ref = &mut *cp;

    // Checkpoint globals (from generated xetex-checkpoint-globals.h)
    cp_value!(cp, shell_escape_enabled);
    cp_value!(cp, bad);
    cp_value!(cp, name_length);
    cp_value!(cp, name_length16);
    cp_value!(cp, first);
    cp_value!(cp, last);
    cp_value!(cp, max_buf_stack);
    cp_value!(cp, in_initex_mode);
    cp_value!(cp, error_line);
    cp_value!(cp, half_error_line);
    cp_value!(cp, max_print_line);
    cp_value!(cp, max_strings);
    cp_value!(cp, strings_free);
    cp_value!(cp, string_vacancies);
    cp_value!(cp, pool_size);
    cp_value!(cp, pool_free);
    cp_value!(cp, font_mem_size);
    cp_value!(cp, font_max);
    cp_value!(cp, hyph_size);
    cp_value!(cp, trie_size);
    cp_value!(cp, buf_size);
    cp_value!(cp, stack_size);
    cp_value!(cp, max_in_open);
    cp_value!(cp, param_size);
    cp_value!(cp, nest_size);
    cp_value!(cp, save_size);
    cp_value!(cp, expand_depth);
    cp_value!(cp, file_line_error_style_p);
    cp_value!(cp, halt_on_error_p);
    cp_value!(cp, quoted_filename);
    cp_value!(cp, insert_src_special_auto);
    cp_value!(cp, insert_src_special_every_par);
    cp_value!(cp, insert_src_special_every_math);
    cp_value!(cp, insert_src_special_every_vbox);
    cp_value!(cp, pool_ptr);
    cp_value!(cp, str_ptr);
    cp_value!(cp, init_pool_ptr);
    cp_value!(cp, init_str_ptr);
    cp_value!(cp, rust_stdout);
    cp_value!(cp, log_file);
    cp_value!(cp, selector);
    cp_value!(cp, dig);
    cp_value!(cp, tally);
    cp_value!(cp, term_offset);
    cp_value!(cp, file_offset);
    cp_value!(cp, trick_buf);
    cp_value!(cp, trick_count);
    cp_value!(cp, first_count);
    cp_value!(cp, doing_special);
    cp_value!(cp, native_text_size);
    cp_value!(cp, native_len);
    cp_value!(cp, save_native_len);
    cp_value!(cp, interaction);
    cp_value!(cp, deletions_allowed);
    cp_value!(cp, set_box_allowed);
    cp_value!(cp, history);
    cp_value!(cp, error_count);
    cp_value!(cp, help_ptr);
    cp_value!(cp, use_err_help);
    cp_value!(cp, arith_error);
    cp_value!(cp, tex_remainder);
    cp_value!(cp, randoms);
    cp_value!(cp, j_random);
    cp_value!(cp, random_seed);
    cp_value!(cp, two_to_the);
    cp_value!(cp, spec_log);
    cp_value!(cp, temp_ptr);
    cp_value!(cp, lo_mem_max);
    cp_value!(cp, hi_mem_min);
    cp_value!(cp, var_used);
    cp_value!(cp, dyn_used);
    cp_value!(cp, avail);
    cp_value!(cp, mem_end);
    cp_value!(cp, rover);
    cp_value!(cp, last_leftmost_char);
    cp_value!(cp, last_rightmost_char);
    cp_value!(cp, hlist_stack);
    cp_value!(cp, hlist_stack_level);
    cp_value!(cp, first_p);
    cp_value!(cp, global_prev_p);
    cp_value!(cp, font_in_short_display);
    cp_value!(cp, depth_threshold);
    cp_value!(cp, breadth_max);
    cp_value!(cp, nest_ptr);
    cp_value!(cp, max_nest_stack);
    cp_value!(cp, cur_list);
    cp_value!(cp, shown_mode);
    cp_value!(cp, old_setting);
    cp_value!(cp, hash_used);
    cp_value!(cp, hash_extra);
    cp_value!(cp, hash_top);
    cp_value!(cp, eqtb_top);
    cp_value!(cp, hash_high);
    cp_value!(cp, no_new_control_sequence);
    cp_value!(cp, cs_count);
    cp_value!(cp, prim);
    cp_value!(cp, prim_used);
    cp_value!(cp, save_ptr);
    cp_value!(cp, max_save_stack);
    cp_value!(cp, cur_level);
    cp_value!(cp, cur_group);
    cp_value!(cp, cur_boundary);
    cp_value!(cp, mag_set);
    cp_value!(cp, cur_cmd);
    cp_value!(cp, cur_chr);
    cp_value!(cp, cur_cs);
    cp_value!(cp, cur_tok);
    cp_value!(cp, input_ptr);
    cp_value!(cp, max_in_stack);
    cp_value!(cp, cur_input);
    cp_value!(cp, in_open);
    cp_value!(cp, open_parens);
    cp_value!(cp, line);
    cp_value!(cp, scanner_status);
    cp_value!(cp, warning_index);
    cp_value!(cp, def_ref);
    cp_value!(cp, param_ptr);
    cp_value!(cp, max_param_stack);
    cp_value!(cp, align_state);
    cp_value!(cp, base_ptr);
    cp_value!(cp, par_loc);
    cp_value!(cp, par_token);
    cp_value!(cp, force_eof);
    cp_value!(cp, expand_depth_count);
    cp_value!(cp, is_in_csname);
    cp_value!(cp, cur_mark);
    cp_value!(cp, long_state);
    cp_value!(cp, pstack);
    cp_value!(cp, cur_val);
    cp_value!(cp, cur_val1);
    cp_value!(cp, cur_val_level);
    cp_value!(cp, radix);
    cp_value!(cp, cur_order);
    cp_value!(cp, read_open);
    cp_value!(cp, cond_ptr);
    cp_value!(cp, if_limit);
    cp_value!(cp, cur_if);
    cp_value!(cp, if_line);
    cp_value!(cp, skip_line);
    cp_value!(cp, cur_name);
    cp_value!(cp, cur_area);
    cp_value!(cp, cur_ext);
    cp_value!(cp, area_delimiter);
    cp_value!(cp, ext_delimiter);
    cp_value!(cp, file_name_quote_char);
    cp_value!(cp, format_default_length);
    cp_value!(cp, name_in_progress);
    cp_value!(cp, job_name);
    cp_value!(cp, log_opened);
    cp_value!(cp, texmf_log_name);
    cp_value!(cp, fmem_ptr);
    cp_value!(cp, font_ptr);
    cp_value!(cp, loaded_font_flags);
    cp_value!(cp, loaded_font_letter_space);
    cp_value!(cp, null_character);
    cp_value!(cp, total_pages);
    cp_value!(cp, max_v);
    cp_value!(cp, max_h);
    cp_value!(cp, max_push);
    cp_value!(cp, last_bop);
    cp_value!(cp, dead_cycles);
    cp_value!(cp, doing_leaders);
    cp_value!(cp, rule_ht);
    cp_value!(cp, rule_dp);
    cp_value!(cp, rule_wd);
    cp_value!(cp, cur_h);
    cp_value!(cp, cur_v);
    cp_value!(cp, epochseconds);
    cp_value!(cp, microseconds);
    cp_value!(cp, total_stretch);
    cp_value!(cp, total_shrink);
    cp_value!(cp, last_badness);
    cp_value!(cp, adjust_tail);
    cp_value!(cp, pre_adjust_tail);
    cp_value!(cp, pack_begin_line);
    cp_value!(cp, empty);
    cp_value!(cp, cur_f);
    cp_value!(cp, cur_c);
    cp_value!(cp, cur_i);
    cp_value!(cp, cur_align);
    cp_value!(cp, cur_span);
    cp_value!(cp, cur_loop);
    cp_value!(cp, align_ptr);
    cp_value!(cp, cur_head);
    cp_value!(cp, cur_tail);
    cp_value!(cp, cur_pre_head);
    cp_value!(cp, cur_pre_tail);
    cp_value!(cp, just_box);
    cp_value!(cp, active_width);
    cp_value!(cp, hc);
    cp_value!(cp, hf);
    cp_value!(cp, hu);
    cp_value!(cp, cur_lang);
    cp_value!(cp, max_hyph_char);
    cp_value!(cp, hyf);
    cp_value!(cp, init_list);
    cp_value!(cp, init_lig);
    cp_value!(cp, init_lft);
    cp_value!(cp, hyphen_passed);
    cp_value!(cp, cur_l);
    cp_value!(cp, cur_r);
    cp_value!(cp, cur_q);
    cp_value!(cp, lig_stack);
    cp_value!(cp, ligature_present);
    cp_value!(cp, lft_hit);
    cp_value!(cp, rt_hit);
    cp_value!(cp, hyf_distance);
    cp_value!(cp, hyf_num);
    cp_value!(cp, hyf_next);
    cp_value!(cp, op_start);
    cp_value!(cp, hyph_count);
    cp_value!(cp, hyph_next);
    cp_value!(cp, trie_used);
    cp_value!(cp, trie_op_lang);
    cp_value!(cp, trie_op_val);
    cp_value!(cp, trie_op_ptr);
    cp_value!(cp, max_op_used);
    cp_value!(cp, trie_ptr);
    cp_value!(cp, trie_min);
    cp_value!(cp, trie_max);
    cp_value!(cp, trie_not_ready);
    cp_value!(cp, best_height_plus_depth);
    cp_value!(cp, main_f);
    cp_value!(cp, main_i);
    cp_value!(cp, main_j);
    cp_value!(cp, main_k);
    cp_value!(cp, main_p);
    cp_value!(cp, main_pp);
    cp_value!(cp, main_ppp);
    cp_value!(cp, main_h);
    cp_value!(cp, is_hyph);
    cp_value!(cp, space_class);
    cp_value!(cp, prev_class);
    cp_value!(cp, main_s);
    cp_value!(cp, bchar);
    cp_value!(cp, false_bchar);
    cp_value!(cp, cancel_boundary);
    cp_value!(cp, ins_disc);
    cp_value!(cp, cur_box);
    cp_value!(cp, after_token);
    cp_value!(cp, long_help_seen);
    cp_value!(cp, format_ident);
    cp_value!(cp, write_file);
    cp_value!(cp, write_open);
    cp_value!(cp, write_loc);
    cp_value!(cp, cur_page_width);
    cp_value!(cp, cur_page_height);
    cp_value!(cp, cur_h_offset);
    cp_value!(cp, cur_v_offset);
    cp_value!(cp, pdf_last_x_pos);
    cp_value!(cp, pdf_last_y_pos);
    cp_value!(cp, LR_ptr);
    cp_value!(cp, LR_problems);
    cp_value!(cp, cur_dir);
    cp_value!(cp, pseudo_files);
    cp_value!(cp, max_reg_num);
    cp_value!(cp, sa_root);
    cp_value!(cp, cur_ptr);
    cp_value!(cp, sa_null);
    cp_value!(cp, sa_chain);
    cp_value!(cp, sa_level);
    cp_value!(cp, hyph_start);
    cp_value!(cp, hyph_index);
    cp_value!(cp, disc_ptr);
    cp_value!(cp, edit_name_start);
    cp_value!(cp, stop_at_space);
    cp_value!(cp, native_font_type_flag);
    cp_value!(cp, xtx_ligature_present);
    cp_value!(cp, delta);
    cp_value!(cp, synctex_enabled);
    cp_value!(cp, used_tectonic_coda_tokens);
    cp_value!(cp, semantic_pagination_enabled);
    cp_value!(cp, gave_char_warning_help);
    cp_value!(cp, page_tail);
    cp_value!(cp, page_contents);
    cp_value!(cp, page_so_far);
    cp_value!(cp, last_glue);
    cp_value!(cp, last_penalty);
    cp_value!(cp, last_kern);
    cp_value!(cp, last_node_type);
    cp_value!(cp, insert_penalties);
    cp_value!(cp, output_active);
    cp_val(cp, c"_xeq_level_array".as_ptr(), &raw mut _xeq_level_array);

    let trie_op_hash_ptr = &raw mut *TRIE_OP_HASH(NEG_TRIE_OP_SIZE);
    tt_checkpoint_field(
        cp,
        c"_trie_op_hash_array".as_ptr(),
        trie_op_hash_ptr as *mut c_void,
        (TRIE_OP_SIZE - NEG_TRIE_OP_SIZE + 1) as usize * std::mem::size_of::<i32>(),
    );

    if pool_size != 6250000
        || max_strings != 565536
        || font_max != 9000
        || buf_size != 200000
        || nest_size != 500
        || stack_size != 5000
        || param_size != 10000
        || save_size != 80000
        || trie_size != 1000000
        || hyph_size != 8191
        || font_mem_size < 0
        || font_mem_size > 32000000
        || hash_extra < 0
        || hash_extra > 600000
    {
        cp_ref.valid = false;
        return;
    }

    if !cp_ref.valid
        || in_open < 0
        || in_open > max_in_open
        || max_in_open != 15
        || input_ptr < 0
        || input_ptr > stack_size
        || font_ptr < FONT_BASE
        || font_ptr > font_max
        || pool_ptr < 0
        || pool_ptr > pool_size
        || str_ptr < TOO_BIG_CHAR
        || str_ptr - TOO_BIG_CHAR > max_strings
        || save_ptr < 0
        || save_ptr > save_size
        || param_ptr != 0
        || nest_ptr != 0
        || hash_top < hash_offset
        || hash_top > EQTB_SIZE + 600000
        || eqtb_top != EQTB_SIZE + hash_extra
        || trie_max > 1000000
        || fmem_ptr < 0
        || fmem_ptr > font_mem_size
        || hash_high < 0
        || hash_high > hash_extra
    {
        cp_ref.valid = false;
        return;
    }

    cp_array!(cp, buffer, buf_size + 1);
    cp_used!(cp, nest, nest_size + 1, nest_ptr);
    cp_used!(cp, save_stack, save_size + 1, save_ptr);
    cp_used!(cp, input_stack, stack_size + 1, input_ptr);
    cp_used!(cp, param_stack, param_size + 1, param_ptr);
    cp_array!(cp, line_stack, max_in_open + 1);
    cp_array!(cp, eof_seen, max_in_open + 1);
    cp_array!(cp, grp_stack, max_in_open + 1);
    cp_array!(cp, if_stack, max_in_open + 1);
    cp_array!(cp, source_filename_stack, max_in_open + 1);
    cp_array!(cp, full_source_filename_stack, max_in_open + 1);
    cp_array!(cp, hyph_word, hyph_size + 1);
    cp_array!(cp, hyph_list, hyph_size + 1);
    cp_array!(cp, hyph_link, hyph_size + 1);

    if cp_ref.restoring {
        yhash = xmalloc_array::<b32x2>((1 + hash_top - hash_offset) as usize);
        hash = yhash.offset(-hash_offset as isize);
        libc::memset(
            hash.add(HASH_BASE as usize) as *mut c_void,
            0,
            ((UNDEFINED_CONTROL_SEQUENCE + 1 - HASH_BASE) as usize) * std::mem::size_of::<b32x2>(),
        );
        if hash_extra != 0 {
            libc::memset(
                hash.add((EQTB_SIZE + 1) as usize) as *mut c_void,
                0,
                (hash_extra as usize) * std::mem::size_of::<b32x2>(),
            );
        }
    }

    tt_checkpoint_field(
        cp,
        c"hash".as_ptr(),
        hash.add(HASH_BASE as usize) as *mut c_void,
        ((UNDEFINED_CONTROL_SEQUENCE + 1 - HASH_BASE) as usize) * std::mem::size_of::<b32x2>(),
    );

    let hash_extra_ptr = if hash_high != 0 { hash.add((EQTB_SIZE + 1) as usize) } else { hash };
    tt_checkpoint_field(
        cp,
        c"hash_extra".as_ptr(),
        hash_extra_ptr as *mut c_void,
        (hash_high as usize) * std::mem::size_of::<b32x2>(),
    );

    cp_rle!(cp, eqtb, eqtb_top + 1);
    tt_checkpoint_memory(cp);
    cp_used!(cp, str_start, max_strings + 1, str_ptr - TOO_BIG_CHAR + 1);
    cp_used!(cp, str_pool, pool_size + 1, pool_ptr);
    cp_used!(cp, font_info, font_mem_size + 1, fmem_ptr);
    cp_array!(cp, native_text, native_text_size);
    cp_array!(cp, trie_trl, trie_max + 1);
    cp_array!(cp, trie_tro, trie_max + 1);
    cp_array!(cp, trie_trc, trie_max + 1);
    cp_array!(cp, font_flags, font_max + 1);
    cp_array!(cp, font_letter_space, font_max + 1);
    cp_array!(cp, font_check, font_max + 1);
    cp_array!(cp, font_size, font_max + 1);
    cp_array!(cp, font_dsize, font_max + 1);
    cp_array!(cp, font_params, font_max + 1);
    cp_array!(cp, font_name, font_max + 1);
    cp_array!(cp, font_area, font_max + 1);
    cp_array!(cp, font_bc, font_max + 1);
    cp_array!(cp, font_ec, font_max + 1);
    cp_array!(cp, font_glue, font_max + 1);
    cp_array!(cp, font_used, font_max + 1);
    cp_array!(cp, hyphen_char, font_max + 1);
    cp_array!(cp, skew_char, font_max + 1);
    cp_array!(cp, bchar_label, font_max + 1);
    cp_array!(cp, font_bchar, font_max + 1);
    cp_array!(cp, font_false_bchar, font_max + 1);
    cp_array!(cp, char_base, font_max + 1);
    cp_array!(cp, width_base, font_max + 1);
    cp_array!(cp, height_base, font_max + 1);
    cp_array!(cp, depth_base, font_max + 1);
    cp_array!(cp, italic_base, font_max + 1);
    cp_array!(cp, lig_kern_base, font_max + 1);
    cp_array!(cp, kern_base, font_max + 1);
    cp_array!(cp, exten_base, font_max + 1);
    cp_array!(cp, param_base, font_max + 1);
    cp_string!(cp, name_of_file);
    cp_string!(cp, name_of_input_file);
    cp_string!(cp, TEX_format_default);

    if cp_ref.restoring {
        name_of_file16 = std::ptr::null_mut();
        font_layout_engine = xcalloc_array::<*mut c_void>(font_max as usize);
        font_mapping = xcalloc_array::<*mut c_void>(font_max as usize);
        loaded_font_mapping = std::ptr::null_mut();
        libc::memset((&raw mut read_file).cast::<c_void>(), 0, std::mem::size_of::<[*mut UFILE; 16]>());
        libc::memset(
            (&raw mut help_line).cast::<*const c_char>() as *mut c_void,
            0,
            std::mem::size_of::<[*const c_char; 6]>(),
        );
        max_reg_help_line = c"A register number must be between 0 and 32767.".as_ptr();
        output_file_extension = c".xdv".as_ptr();
        input_file = xcalloc_array::<*mut UFILE>(max_in_open as usize);
        *input_file.add(0) = &raw mut stdin_ufile;
    }

    cp_val(cp, c"stdin_ufile".as_ptr(), &raw mut stdin_ufile);

    for i in 1..=in_open {
        if cp_ref.restoring {
            *input_file.add(i as usize) = xcalloc(1, std::mem::size_of::<UFILE>()) as *mut UFILE;
        }
        let f = *input_file.add(i as usize);
        tt_checkpoint_field(cp, c"input".as_ptr(), f as *mut c_void, std::mem::size_of::<UFILE>());
        if cp_ref.restoring
            && (!(*f).conversionData.is_null()
                || (*f).input.length > std::mem::size_of_val(&(*f).input.bytes)
                || (*f).input.position > (*f).input.length
                || (*f).input.handle != (*f).handle
                || !tt_xetex_checkpoint_handle_valid((*f).handle, false))
        {
            cp_ref.valid = false;
        }
    }

    if cp_ref.restoring && (!tt_xetex_checkpoint_handle_valid(rust_stdout, true) || (log_opened && !tt_xetex_checkpoint_handle_valid(log_file, true)))
    {
        cp_ref.valid = false;
    }

    for i in 0..16 {
        if cp_ref.restoring && write_open[i] && !tt_xetex_checkpoint_handle_valid(write_file[i], true) {
            cp_ref.valid = false;
        }
    }

    if cp_ref.valid {
        super::xetex_ext::tt_checkpoint_fonts(cp.cast());
    }
}
