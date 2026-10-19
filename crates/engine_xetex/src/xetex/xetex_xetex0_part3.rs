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
    A_LEADERS, ADDITIONAL, ADJUST_HEAD, BEGIN_M_CODE, BIGGEST_CHAR, BIGGEST_LANG, BOX_NODE_SIZE,
    CS_TOKEN_FLAG, CUR_FONT_LOC, DIMEN_BASE, DIMEN_PAR__hang_indent, DISC_NODE, END_M_CODE,
    ETEX_PEN_BASE, ETEX_PENALTIES_PAR__inter_line_penalties, EXPLICIT, GLYPH_NODE_SIZE, HMODE,
    INSERTED, INT_BASE, INT_PAR__hang_after, INT_PAR__language, INT_PAR__left_hyphen_min,
    INT_PAR__looseness, INT_PAR__right_hyphen_min, L_CODE, LEFT_BRACE_TOKEN, LETTER,
    LOCAL__par_shape, LOCAL_BASE, MARGIN_KERN_NODE, MARK_NODE, MAX_HALFWORD, MEDIUM_NODE_SIZE,
    MIN_HALFWORD, MMODE, MU_GLUE, OTHER_CHAR, PRE_ADJUST_HEAD, R_CODE, RELAX, RIGHT_BRACE_TOKEN,
    SMALL_NODE_SIZE, SPACER, SUB_BOX, SYNCTEX_FIELD_SIZE, TEMP_HEAD, TEX_NULL, X_HEIGHT_CODE,
};
use super::xetex_core::scaled_t;
use super::xetex_errors::{confusion, error, fatal_error};
use super::xetex_ext::{AAT_FONT_FLAG, OTGR_FONT_FLAG};
use super::xetex_linebreak::line_break;
use super::xetex_math::{build_page, insert_src_special, is_char_node, odd};
use super::xetex_output::{
    capture_to_diagnostic, diagnostic_begin_capture_warning_here, error_here_with_diagnostic,
    print_char, print_cstr, print_esc_cstr, print_int, print_nl_cstr,
};
use super::xetex_scaledmath::tex_round;
use super::xetex_shipout::{new_edge, ship_out};
use super::xetex_xetex0::{
    back_error, back_input, begin_diagnostic, begin_token_list, copy_native_glyph_info,
    copy_node_list, delete_sa_ref, effective_char, end_diagnostic, eq_define, eq_word_define,
    find_sa_element, flush_list, flush_node_list, free_node, geq_define, get_avail, get_node,
    get_x_token, gsa_def, ins_error, new_disc, new_kern, new_null_box, new_param_glue, new_penalty,
    new_save_level, pop_nest, print_cmd_chr, push_nest, sa_def, scan_char_num, scan_dimen,
    scan_eight_bit_int, scan_int, scan_keyword, scan_left_brace, scan_register_num, scan_rule_spec,
    scan_toks, show_box, unsave,
};
use super::xetex_xetex0_part1::{hpack, new_character, vpackage};
use super::xetex_xetex0_part2::{append_to_vlist, new_noad};
use super::xetex_xetexd::{
    LR_problems, LR_ptr, UTF16_code, adjust_tail, avail, b16x4, cur_chr, cur_cmd, cur_dir,
    cur_group, cur_lang, cur_list, cur_tok, cur_val, def_ref, eqtb, font_area, font_info,
    group_code, help_line, help_ptr, hi_mem_min, hyphen_char, internal_font_number, mem, nest_ptr,
    param_base, pre_adjust_tail, save_ptr, save_stack, small_number, temp_ptr,
};
use crate::{
    BOX_REG, FONT_CHARACTER_INFO, FONT_CHARACTER_WIDTH, FONT_CHARINFO_HEIGHT,
    FONT_CHARINFO_ITALCORR, FONT_CHARINFO_WIDTH,
};

use super::bridge::{ttstub_diag_printf, ttstub_output_flush};
use super::xetex_constants::*;
use super::xetex_constants::{BOTTOM_LEVEL, MATH_LEFT_GROUP, MATH_SHIFT_GROUP, SEMI_SIMPLE_GROUP};
use super::xetex_core::*;
use super::xetex_ext::*;
use super::xetex_format::*;
use super::xetex_format::{
    CLOSE_NODE, GLUE_NODE, GLYPH_NODE, HLIST_NODE, KERN_NODE, LANGUAGE_NODE, LIGATURE_NODE,
    MATH_NODE, NATIVE_WORD_NODE, NATIVE_WORD_NODE_AT, OPEN_NODE, PDF_NODE, PDF_SAVE_POS_NODE,
    PIC_NODE, RULE_NODE, SPECIAL_NODE, VLIST_NODE, WHATSIT_NODE, WRITE_NODE,
};
use super::xetex_io::*;
use super::xetex_linebreak::*;
use super::xetex_math::*;
use super::xetex_pagebuilder::*;
use super::xetex_scaledmath::*;
use super::xetex_shipout::*;
use super::xetex_synctex::*;
use super::xetex_xetex0::*;
use super::xetex_xetexd::input_state_t;
use super::xetex_xetexd::*;
use super::*;
use libc::*;

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

macro_rules! BOX_list_ptr {
    ($p:expr) => {
        mem!(($p) + 5).b32.s1
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

macro_rules! CHAR_NODE_font {
    ($p:expr) => {
        mem!($p).b16.s1
    };
}

macro_rules! CHAR_NODE_character {
    ($p:expr) => {
        mem!($p).b16.s0
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
