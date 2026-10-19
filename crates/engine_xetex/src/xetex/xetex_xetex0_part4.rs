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
    A_LEADERS, ACCENT, ACCENT_NOAD, ADDITIONAL, ADJUST_HEAD, ADJUST_NODE, BATCH_MODE,
    BOX_NODE_SIZE, CLOSE_NODE, COND_MATH_GLUE, COUNT_BASE, CS_TOKEN_FLAG, CUR_FONT_LOC, DELIM_NUM,
    DIMEN_BASE, DISC_NODE, EMPTY, ERROR_STOP_MODE, FONT_BASE, FROZEN_CONTROL_SEQUENCE, GARBAGE,
    GLUE_BASE, GLUE_NODE, GLYPH_NODE, GLYPH_NODE_SIZE, HASH_BASE, HLIST_NODE, HMODE, INS_NODE,
    INSERTED, INT_BASE, INT_PAR__language, INT_PAR__left_hyphen_min, INT_PAR__right_hyphen_min,
    INT_PAR__texxet, INT_PAR__tracing_online, INT_PAR__xetex_interword_space_shaping,
    INT_PAR__xetex_use_glyph_metrics, KERN_NODE, LANGUAGE_NODE, LEFT_BRACE_TOKEN, LETTER, LIG_TAG,
    LOCAL_BASE, MARK_NODE, MATH_CHOICE_GROUP, MATH_LEFT_GROUP, MATH_SHIFT, MATH_SHIFT_GROUP,
    MAX_HALFWORD, MMODE, NATIVE_WORD_NODE, NATIVE_WORD_NODE_AT, NOAD_SIZE, NON_ADDRESS, NORMAL,
    OPEN_NODE, ORD_NOAD, OTHER_CHAR, PDF_SAVE_POS_NODE, PENALTY_NODE, PRE_ADJUST_HEAD, RELAX,
    RIGHT_BRACE_TOKEN, RULE_NODE, SEMI_SIMPLE_GROUP, SMALL_NODE_SIZE, SPACE_ADJUSTMENT, SPACER,
    SPECIAL_NODE, SUB_BOX, SUB_MLIST, SUP_MARK, TEMP_HEAD, TEX_INFINITY, TEX_NULL, TOKEN_LIST,
    TOO_BIG_CHAR, VCENTER_NOAD, VLIST_NODE, VMODE, WHATSIT_NODE, WRITE_NODE,
};
use super::xetex_core::{SELECTOR_NEW_STRING, SELECTOR_TERM_AND_LOG, SELECTOR_TERM_ONLY, scaled_t};
use super::xetex_errors::{confusion, error, overflow, pdf_error};
use super::xetex_ext::{
    AAT_FONT_FLAG, OTGR_FONT_FLAG, map_char_to_glyph, tt_checkpoint_enabled, xrealloc,
};
use super::xetex_io::open_or_close_in;
use super::xetex_math::{
    after_math, append_choices, build_choices, build_page, fin_mlist, init_math, is_char_node,
    math_ac, math_fraction, math_left_right, math_limit_switch, math_radical, set_class,
    set_family, start_eq_no, sub_sup,
};
use super::xetex_output::{
    capture_to_diagnostic, diagnostic_begin_capture_warning_here, error_here_with_diagnostic,
    print, print_char, print_cstr, print_esc_cstr, print_file_line, print_int, print_ln,
    print_nl_cstr, print_scaled, sprint_cs,
};
use super::xetex_pic::load_picture;
use super::xetex_scaledmath::{init_randoms, mult_and_add, x_over_n, xn_over_d};
use super::xetex_shipout::{_tt_abort, finalize_dvi_file, out_what};
use super::xetex_stringpool::{append_str, length, make_string, str_eq_str};
use super::xetex_texmfmp::{
    get_seconds_and_micros, is_new_source, make_src_special, maketexstring, remember_source_info,
};
use super::xetex_xetex0::{
    back_input, begin_diagnostic, begin_token_list, delete_glue_ref, delete_token_ref,
    effective_char, end_diagnostic, end_token_list, eq_define, eq_word_define, find_sa_element,
    flush_list, flush_node_list, free_node, geq_define, geq_word_define, get_avail, get_next,
    get_node, get_token, get_x_token, gsa_def, gsa_w_def, ins_error, new_disc, new_glue, new_kern,
    new_lig_item, new_ligature, new_math, new_param_glue, new_save_level, new_skip_param, new_spec,
    not_native_font_error, pop_nest, prim_lookup, print_cmd_chr, print_meaning, push_nest, sa_def,
    sa_w_def, save_for_after, scan_char_num, scan_delimiter_int, scan_dimen, scan_fifteen_bit_int,
    scan_four_bit_int, scan_glue, scan_int, scan_keyword, scan_math, scan_math_class_int,
    scan_math_fam_int, scan_optional_equals, scan_register_num, scan_rule_spec, scan_toks,
    scan_usv_num, scan_xetex_math_char_int, set_math_char, show_activities, show_box,
    show_cur_cmd_chr, show_token_list, str_toks, the_toks, token_show, unsave, x_token,
};
use super::xetex_xetex0_part1::{
    char_warning, do_locale_linebreaks, effective_char_info, new_native_word_node, open_log_file,
    pack_file_name, read_font_info, scan_file_name, scan_spec, vpackage,
};
use super::xetex_xetex0_part2::{
    align_peek, app_space, append_glue, box_error, eTeX_enabled, init_align, insert_dollar_sign,
    its_all_over, new_noad, new_style, privileged, report_illegal_case, show_save_groups,
};
use super::xetex_xetex0_part3::{
    align_error, append_discretionary, append_italic_correction, append_kern, append_penalty,
    begin_box, begin_insert_or_adjust, build_discretionary, cs_error, delete_last, do_endv,
    end_graf, extra_right_brace, head_for_vmode, indent_in_hmode, make_accent, make_mark, new_graf,
    no_align_error, norm_min, normal_paragraph, off_save, omit_error, package, scan_box, unpackage,
};
use super::xetex_xetexd::{
    UTF16_code, adjust_tail, after_token, align_state, arith_error, avail, bchar, bchar_label,
    cancel_boundary, cond_ptr, cur_area, cur_chr, cur_cmd, cur_cs, cur_ext, cur_f, cur_group,
    cur_if, cur_input, cur_l, cur_list, cur_name, cur_ptr, cur_q, cur_r, cur_tok, cur_val,
    cur_val_level, dead_cycles, def_ref, disc_ptr, epochseconds, eqtb, eqtb_top, error_count,
    false_bchar, file_line_error_style_p, file_offset, font_area, font_bc, font_bchar, font_dsize,
    font_ec, font_false_bchar, font_glue, font_info, font_mapping, font_name, font_ptr, font_size,
    hash, help_line, help_ptr, hyphen_char, if_limit, if_line, in_open, init_pool_ptr, input_file,
    ins_disc, insert_penalties, insert_src_special_auto, insert_src_special_every_vbox,
    interaction, internal_font_number, is_hyph, job_name, kern_base, lft_hit, lig_kern_base,
    lig_stack, ligature_present, line, log_file, log_opened, long_help_seen, main_f, main_h,
    main_i, main_j, main_k, main_p, main_pp, main_ppp, main_s, mapped_text, max_print_line, mem,
    microseconds, name_in_progress, native_len, native_text, native_text_size, nest, nest_ptr,
    old_setting, output_active, packed_UTF16_code, page_so_far, page_tail, param_base,
    pool_pointer, pool_ptr, pool_size, pre_adjust_tail, prev_class, random_seed, rt_hit,
    rust_stdout, save_native_len, save_ptr, save_stack, scanner_status, selector, set_box_allowed,
    small_number, source_filename_stack, space_class, str_number, str_pool, str_ptr, str_start,
    temp_ptr, term_offset, texmf_log_name, use_err_help, write_file, write_open,
};

use super::bridge::{
    ttstub_diag_printf, ttstub_output_close, ttstub_output_flush, ttstub_output_putc,
};
use super::xetex_checkpoint::*;
use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_errors::*;
use super::xetex_ext::*;
use super::xetex_format::*;
use super::xetex_ini::*;
use super::xetex_io::*;
use super::xetex_linebreak::*;
use super::xetex_math::*;
use super::xetex_output::*;
use super::xetex_pagebuilder::*;
use super::xetex_pic::*;
use super::xetex_scaledmath::*;
use super::xetex_shipout::*;
use super::xetex_stringpool::*;
use super::xetex_synctex::*;
use super::xetex_texmfmp::*;
use super::xetex_xetex_ot_math::*;
use super::xetex_xetex0::*;
use super::xetex_xetex0_part1::*;
use super::xetex_xetex0_part2::*;
use super::xetex_xetex0_part3::*;
use super::xetex_xetexd::*;
use super::*;
use libc::*;
use std::ptr;

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

macro_rules! font_info {
    ($p:expr) => {
        *font_info.offset(($p) as isize)
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

macro_rules! GLUE_SPEC_ref_count {
    ($p:expr) => {
        mem!($p).b32.s1
    };
}

macro_rules! BOX_REG {
    ($n:expr) => {
        eqtb!(BOX_BASE + ($n) as i32).b32.s1
    };
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
    ($p:expr) => {
        eqtb!(INT_BASE + ($p) as i32).b32.s1
    };
}

macro_rules! DIMENPAR {
    (split_max_depth) => {
        eqtb!(DIMEN_BASE + DIMEN_PAR__split_max_depth).b32.s1
    };
    ($p:expr) => {
        eqtb!(DIMEN_BASE + ($p) as i32).b32.s1
    };
}

macro_rules! GLUEPAR {
    (split_top_skip) => {
        eqtb!(GLUE_BASE + GLUE_PAR__split_top_skip).b32.s1
    };
    (space_skip) => {
        eqtb!(GLUE_BASE + GLUE_PAR__space_skip).b32.s1
    };
    ($p:expr) => {
        eqtb!(GLUE_BASE + ($p) as i32).b32.s1
    };
}

macro_rules! LOCAL {
    ($p:expr) => {
        eqtb!(LOCAL_BASE + ($p) as i32).b32.s1
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
pub unsafe fn get_r_token() {
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
pub unsafe fn trap_zero_glue() {
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
pub unsafe fn do_register_command(a: small_number) {
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
pub unsafe fn alter_aux() {
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
pub unsafe fn alter_prev_graf() {
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
pub unsafe fn alter_page_so_far() {
    let mut c: u8;
    c = cur_chr as u8;
    scan_optional_equals();
    scan_dimen(false, false, false);
    *(&raw mut page_so_far).cast::<scaled_t>().add(c as usize) = cur_val;
}

// 97. alter_integer
pub unsafe fn alter_integer() {
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
pub unsafe fn alter_box_dimen() {
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
pub unsafe fn new_font(a: small_number) {
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
pub unsafe fn new_interaction() {
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
pub unsafe fn issue_message() {
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
pub unsafe fn shift_case() {
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
pub unsafe fn show_whatever() {
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
pub unsafe fn new_write_whatsit(w: small_number) {
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
pub unsafe fn scan_and_pack_name() {
    scan_file_name();
    pack_file_name(cur_name, cur_area, cur_ext);
}

// 106. do_extension
pub unsafe fn do_extension() {
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
pub unsafe fn fix_language() {
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
pub unsafe fn insert_src_special() {
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
pub unsafe fn append_src_special() {
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
pub unsafe fn tt_insert_special(mut ascii_text: *const c_char) {
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
pub unsafe fn handle_right_brace() {
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
pub unsafe fn main_control() {
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
                            t = set_class(cur_val);
                            scan_math_fam_int();
                            t = t + set_family(cur_val);
                            scan_usv_num();
                            t = t + cur_val;
                            set_math_char(t);
                        } else if (cur_chr == 1) {
                            scan_xetex_math_char_int();
                            set_math_char(cur_val);
                        } else {
                            scan_fifteen_bit_int();
                            set_math_char(
                                set_class(cur_val / 4096)
                                    + set_family((cur_val % 4096) / 256)
                                    + (cur_val % 256),
                            );
                        }
                        state = MainControlState::BigSwitch;
                        continue 'control;
                    }
                    (MMODE, MATH_GIVEN) => {
                        set_math_char(
                            set_class(cur_chr / 4096)
                                + set_family((cur_chr % 4096) / 256)
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
                            t = set_class(cur_val);
                            scan_math_fam_int();
                            t = t + set_family(cur_val);
                            scan_usv_num();
                            t = t + cur_val;
                            set_math_char(t);
                        } else {
                            scan_delimiter_int();
                            cur_val = cur_val / 4096;
                            set_math_char(
                                set_class(cur_val / 4096)
                                    + set_family((cur_val % 4096) / 256)
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
                    while (native_text_size <= native_len + 2) {
                        native_text_size = native_text_size + 128;
                        native_text = xrealloc(
                            native_text.cast(),
                            native_text_size as usize * std::mem::size_of::<UTF16_code>(),
                        )
                        .cast();
                    }

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
                    while (native_text_size <= native_len + 1) {
                        native_text_size = native_text_size + 128;
                        native_text = xrealloc(
                            native_text.cast(),
                            native_text_size as usize * std::mem::size_of::<UTF16_code>(),
                        )
                        .cast();
                    }

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

                    while (native_text_size <= native_len + main_k) {
                        native_text_size = native_text_size + 128;
                        native_text = xrealloc(
                            native_text.cast(),
                            native_text_size as usize * std::mem::size_of::<UTF16_code>(),
                        )
                        .cast();
                    }

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

                            while (native_text_size <= native_len + main_k) {
                                native_text_size = native_text_size + 128;
                                native_text = xrealloc(
                                    native_text.cast(),
                                    native_text_size as usize * std::mem::size_of::<UTF16_code>(),
                                )
                                .cast();
                            }

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

                state = MainControlState::MainLoopMoveLig;
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
pub unsafe fn give_err_help() {
    token_show(LOCAL!(LOCAL__err_help));
}

// 114. close_files_and_terminate
pub unsafe fn close_files_and_terminate() {
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
pub unsafe fn flush_str(s: str_number) {
    if s == str_ptr - 1 {
        str_ptr -= 1;
        pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
    }
}

// 116. tokens_to_string
pub unsafe fn tokens_to_string(p: i32) -> str_number {
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
pub unsafe fn scan_pdf_ext_toks() {
    scan_toks(false, true);
}

// 118. compare_strings
pub unsafe fn compare_strings() {
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
pub unsafe fn prune_page_top(p: i32, s: bool) -> i32 {
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
pub unsafe fn do_marks(a: small_number, l: small_number, mut q: i32) -> bool {
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
pub unsafe fn do_assignments() {
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
pub unsafe fn new_whatsit(s: small_number, w: small_number) {
    let p = get_node(w as i32);
    NODE_type!(p) = WHATSIT_NODE as u16;
    mem!(p).b16.s0 = (s as u16) as u16;
    mem!(cur_list.tail).b32.s1 = p;
    cur_list.tail = p;
}
