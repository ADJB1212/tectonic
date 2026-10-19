#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use super::xetex_constants::{NORMAL, TEX_NULL, VMODE};
use super::xetex_output::{
    capture_to_diagnostic, error_here_with_diagnostic, print_cstr, print_esc_cstr, print_int,
};
use super::xetex_xetexd::{
    cur_list, help_ptr, mem, nest_ptr, output_active, semantic_pagination_enabled,
};

use libc::{c_char, c_int, c_void};
use std::ptr;

use super::xetex_checkpoint::*;
use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_output::*;
use super::xetex_xetexd::*;

static mut best_page_break: i32 = 0;
static mut best_size: scaled_t = 0;
static mut least_page_cost: i32 = 0;
static mut page_max_depth: scaled_t = 0;

pub unsafe fn tt_checkpoint_pagebuilder(cp: *mut tt_checkpoint) {
    unsafe {
        tt_checkpoint_field(
            cp,
            b"best_page_break\0".as_ptr() as *const c_char,
            &raw mut best_page_break as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"best_size\0".as_ptr() as *const c_char,
            &raw mut best_size as *mut scaled_t as *mut c_void,
            std::mem::size_of::<scaled_t>(),
        );
        tt_checkpoint_field(
            cp,
            b"least_page_cost\0".as_ptr() as *const c_char,
            &raw mut least_page_cost as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"page_max_depth\0".as_ptr() as *const c_char,
            &raw mut page_max_depth as *mut scaled_t as *mut c_void,
            std::mem::size_of::<scaled_t>(),
        );
    }
}

pub unsafe fn initialize_pagebuilder_variables() {
    unsafe {
        page_max_depth = 0;
    }
}

unsafe fn freeze_page_specs(s: u8) {
    unsafe {
        page_contents = (s as i32) as u8;
        page_so_far[0] = (*eqtb.add((DIMEN_BASE + DIMEN_PAR__vsize) as usize)).b32.s1;
        page_max_depth = (*eqtb.add((DIMEN_BASE + DIMEN_PAR__max_depth) as usize))
            .b32
            .s1;
        page_so_far[7] = 0;
        page_so_far[1] = 0;
        page_so_far[2] = 0;
        page_so_far[3] = 0;
        page_so_far[4] = 0;
        page_so_far[5] = 0;
        page_so_far[6] = 0;
        least_page_cost = MAX_HALFWORD;
    }
}

unsafe fn ensure_vbox(n: u8) {
    unsafe {
        let p = (*eqtb.add((BOX_BASE + n as i32) as usize)).b32.s1;
        if p == TEX_NULL {
            return;
        }
        if (*mem.add(p as usize)).b16.s1 as i32 != HLIST_NODE {
            return;
        }

        error_here_with_diagnostic(
            b"Insertions can only be added to a vbox\0".as_ptr() as *const c_char
        );
        capture_to_diagnostic(ptr::null_mut());
        help_ptr = 3;
        help_line[2] = b"Tut tut: You're trying to \\insert into a\0".as_ptr() as *const c_char;
        help_line[1] = b"\\box register that now contains an \\hbox.\0".as_ptr() as *const c_char;
        help_line[0] =
            b"Proceed, and I'll discard its present contents.\0".as_ptr() as *const c_char;
        box_error((n as i32) as u8);
    }
}

unsafe fn fire_up(c: i32) {
    unsafe {
        if (*mem.add(best_page_break as usize)).b16.s1 as i32 == PENALTY_NODE {
            geq_word_define(
                INT_BASE + INT_PAR__output_penalty,
                (*mem.add((best_page_break + 1) as usize)).b32.s1,
            );
            (*mem.add((best_page_break + 1) as usize)).b32.s1 = INF_PENALTY;
        } else {
            geq_word_define(INT_BASE + INT_PAR__output_penalty, INF_PENALTY);
        }

        if sa_root[MARK_VAL as usize] != TEX_NULL {
            if do_marks((FIRE_UP_INIT) as i16, 0, sa_root[MARK_VAL as usize]) {
                sa_root[MARK_VAL as usize] = TEX_NULL;
            }
        }

        if cur_mark[BOT_MARK_CODE as usize] != TEX_NULL {
            if cur_mark[TOP_MARK_CODE as usize] != TEX_NULL {
                delete_token_ref(cur_mark[TOP_MARK_CODE as usize]);
            }
            cur_mark[TOP_MARK_CODE as usize] = cur_mark[BOT_MARK_CODE as usize];
            (*mem.add(cur_mark[TOP_MARK_CODE as usize] as usize)).b32.s0 += 1;
            delete_token_ref(cur_mark[FIRST_MARK_CODE as usize]);
            cur_mark[FIRST_MARK_CODE as usize] = TEX_NULL;
        }

        if c == best_page_break {
            best_page_break = TEX_NULL;
        }

        if (*eqtb.add((BOX_BASE + 255) as usize)).b32.s1 != TEX_NULL {
            error_here_with_diagnostic(b"\0".as_ptr() as *const c_char);
            print_esc_cstr(b"box\0".as_ptr() as *const c_char);
            print_cstr(b"255 is not void\0".as_ptr() as *const c_char);
            capture_to_diagnostic(ptr::null_mut());
            help_ptr = 2;
            help_line[1] = b"You shouldn't use \\box255 except in \\output routines.\0".as_ptr()
                as *const c_char;
            help_line[0] =
                b"Proceed, and I'll discard its present contents.\0".as_ptr() as *const c_char;
            box_error(255);
        }

        insert_penalties = 0;
        let save_split_top_skip = (*eqtb.add((GLUE_BASE + GLUE_PAR__split_top_skip) as usize))
            .b32
            .s1;

        let process_inserts = ((*eqtb.add((INT_BASE + INT_PAR__holding_inserts) as usize))
            .b32
            .s1
            <= 0)
            && !semantic_pagination_enabled;

        if process_inserts {
            let mut r = (*mem.add(PAGE_INS_HEAD as usize)).b32.s1;
            while r != PAGE_INS_HEAD {
                if (*mem.add((r + 2) as usize)).b32.s0 != TEX_NULL {
                    let n = (*mem.add(r as usize)).b16.s0 as u8;
                    ensure_vbox(n);

                    if (*eqtb.add((BOX_BASE + n as i32) as usize)).b32.s1 == TEX_NULL {
                        (*eqtb.add((BOX_BASE + n as i32) as usize)).b32.s1 = new_null_box();
                    }

                    let mut p = (*eqtb.add((BOX_BASE + n as i32) as usize)).b32.s1 + 5;
                    while (*mem.add(p as usize)).b32.s1 != TEX_NULL {
                        p = (*mem.add(p as usize)).b32.s1;
                    }
                    (*mem.add((r + 2) as usize)).b32.s1 = p;
                }
                r = (*mem.add(r as usize)).b32.s1;
            }
        }

        let mut q = HOLD_HEAD;
        (*mem.add(q as usize)).b32.s1 = TEX_NULL;
        let mut prev_p = PAGE_HEAD;
        let mut p = (*mem.add(prev_p as usize)).b32.s1;

        while p != best_page_break {
            let node_type = (*mem.add(p as usize)).b16.s1 as i32;
            if node_type == INS_NODE {
                if process_inserts {
                    let mut r = (*mem.add(PAGE_INS_HEAD as usize)).b32.s1;
                    while (*mem.add(r as usize)).b16.s0 != (*mem.add(p as usize)).b16.s0 {
                        r = (*mem.add(r as usize)).b32.s1;
                    }

                    let mut wait: bool;
                    if (*mem.add((r + 2) as usize)).b32.s0 == TEX_NULL {
                        wait = true;
                    } else {
                        wait = false;
                        let mut s = (*mem.add((r + 2) as usize)).b32.s1;
                        (*mem.add(s as usize)).b32.s1 = (*mem.add((p + 4) as usize)).b32.s0;

                        if (*mem.add((r + 2) as usize)).b32.s0 == p {
                            if (*mem.add(r as usize)).b16.s1 as i32 == SPLIT_UP {
                                if (*mem.add((r + 1) as usize)).b32.s0 == p
                                    && (*mem.add((r + 1) as usize)).b32.s1 != TEX_NULL
                                {
                                    while (*mem.add(s as usize)).b32.s1
                                        != (*mem.add((r + 1) as usize)).b32.s1
                                    {
                                        s = (*mem.add(s as usize)).b32.s1;
                                    }
                                    (*mem.add(s as usize)).b32.s1 = TEX_NULL;
                                    (*eqtb.add((GLUE_BASE + GLUE_PAR__split_top_skip) as usize))
                                        .b32
                                        .s1 = (*mem.add((p + 4) as usize)).b32.s1;
                                    (*mem.add((p + 4) as usize)).b32.s0 =
                                        prune_page_top((*mem.add((r + 1) as usize)).b32.s1, false);

                                    if (*mem.add((p + 4) as usize)).b32.s0 != TEX_NULL {
                                        temp_ptr = vpackage(
                                            (*mem.add((p + 4) as usize)).b32.s0,
                                            0,
                                            (ADDITIONAL) as i16,
                                            MAX_HALFWORD,
                                        );
                                        (*mem.add((p + 3) as usize)).b32.s1 =
                                            (*mem.add((temp_ptr + 3) as usize)).b32.s1
                                                + (*mem.add((temp_ptr + 2) as usize)).b32.s1;
                                        free_node(temp_ptr, BOX_NODE_SIZE);
                                        wait = true;
                                    }
                                }
                            }

                            (*mem.add((r + 2) as usize)).b32.s0 = TEX_NULL;
                            let n = (*mem.add(r as usize)).b16.s0 as i32;
                            let box_reg_n = (*eqtb.add((BOX_BASE + n) as usize)).b32.s1;
                            temp_ptr = (*mem.add((box_reg_n + 5) as usize)).b32.s1;
                            free_node(box_reg_n, BOX_NODE_SIZE);
                            (*eqtb.add((BOX_BASE + n) as usize)).b32.s1 =
                                vpackage(temp_ptr, 0, (ADDITIONAL) as i16, MAX_HALFWORD);
                        } else {
                            while (*mem.add(s as usize)).b32.s1 != TEX_NULL {
                                s = (*mem.add(s as usize)).b32.s1;
                            }
                            (*mem.add((r + 2) as usize)).b32.s1 = s;
                        }
                    }

                    (*mem.add(prev_p as usize)).b32.s1 = (*mem.add(p as usize)).b32.s1;
                    (*mem.add(p as usize)).b32.s1 = TEX_NULL;

                    if wait {
                        (*mem.add(q as usize)).b32.s1 = p;
                        q = p;
                        insert_penalties += 1;
                    } else {
                        delete_glue_ref((*mem.add((p + 4) as usize)).b32.s1);
                        free_node(p, INS_NODE_SIZE);
                    }

                    p = prev_p;
                }
            } else if node_type == MARK_NODE {
                let mark_class = (*mem.add((p + 1) as usize)).b32.s0;
                if mark_class != 0 {
                    find_sa_element((MARK_VAL) as i16, mark_class, true);

                    if (*mem.add((cur_ptr + 1) as usize)).b32.s1 == TEX_NULL {
                        (*mem.add((cur_ptr + 1) as usize)).b32.s1 =
                            (*mem.add((p + 1) as usize)).b32.s1;
                        (*mem.add((*mem.add((p + 1) as usize)).b32.s1 as usize))
                            .b32
                            .s0 += 1;
                    }

                    if (*mem.add((cur_ptr + 2) as usize)).b32.s0 != TEX_NULL {
                        delete_token_ref((*mem.add((cur_ptr + 2) as usize)).b32.s0);
                    }

                    (*mem.add((cur_ptr + 2) as usize)).b32.s0 = (*mem.add((p + 1) as usize)).b32.s1;
                    (*mem.add((*mem.add((p + 1) as usize)).b32.s1 as usize))
                        .b32
                        .s0 += 1;
                } else {
                    if cur_mark[FIRST_MARK_CODE as usize] == TEX_NULL {
                        cur_mark[FIRST_MARK_CODE as usize] = (*mem.add((p + 1) as usize)).b32.s1;
                        (*mem.add(cur_mark[FIRST_MARK_CODE as usize] as usize))
                            .b32
                            .s0 += 1;
                    }

                    if cur_mark[BOT_MARK_CODE as usize] != TEX_NULL {
                        delete_token_ref(cur_mark[BOT_MARK_CODE as usize]);
                    }

                    cur_mark[BOT_MARK_CODE as usize] = (*mem.add((p + 1) as usize)).b32.s1;
                    (*mem.add(cur_mark[BOT_MARK_CODE as usize] as usize)).b32.s0 += 1;
                }
            }

            prev_p = p;
            p = (*mem.add(prev_p as usize)).b32.s1;
        }

        (*eqtb.add((GLUE_BASE + GLUE_PAR__split_top_skip) as usize))
            .b32
            .s1 = save_split_top_skip;

        if p != TEX_NULL {
            if (*mem.add(CONTRIB_HEAD as usize)).b32.s1 == TEX_NULL {
                if nest_ptr == 0 {
                    cur_list.tail = page_tail;
                } else {
                    (*nest.add(0)).tail = page_tail;
                }
            }

            (*mem.add(page_tail as usize)).b32.s1 = (*mem.add(CONTRIB_HEAD as usize)).b32.s1;
            (*mem.add(CONTRIB_HEAD as usize)).b32.s1 = p;
            (*mem.add(prev_p as usize)).b32.s1 = TEX_NULL;
        }

        let save_vbadness = (*eqtb.add((INT_BASE + INT_PAR__vbadness) as usize)).b32.s1;
        (*eqtb.add((INT_BASE + INT_PAR__vbadness) as usize)).b32.s1 = INF_BAD;
        let save_vfuzz = (*eqtb.add((DIMEN_BASE + DIMEN_PAR__vfuzz) as usize)).b32.s1;
        (*eqtb.add((DIMEN_BASE + DIMEN_PAR__vfuzz) as usize)).b32.s1 = MAX_HALFWORD;
        (*eqtb.add((BOX_BASE + 255) as usize)).b32.s1 = vpackage(
            (*mem.add(PAGE_HEAD as usize)).b32.s1,
            best_size,
            (EXACTLY) as i16,
            page_max_depth,
        );
        (*eqtb.add((INT_BASE + INT_PAR__vbadness) as usize)).b32.s1 = save_vbadness;
        (*eqtb.add((DIMEN_BASE + DIMEN_PAR__vfuzz) as usize)).b32.s1 = save_vfuzz;

        if last_glue != MAX_HALFWORD {
            delete_glue_ref(last_glue);
        }

        page_contents = (EMPTY as i32) as u8;
        page_tail = PAGE_HEAD;
        (*mem.add(PAGE_HEAD as usize)).b32.s1 = TEX_NULL;
        last_glue = MAX_HALFWORD;
        last_penalty = 0;
        last_kern = 0;
        last_node_type = -1;
        page_so_far[7] = 0;
        page_max_depth = 0;

        if q != HOLD_HEAD {
            (*mem.add(PAGE_HEAD as usize)).b32.s1 = (*mem.add(HOLD_HEAD as usize)).b32.s1;
            page_tail = q;
        }

        let mut r = (*mem.add(PAGE_INS_HEAD as usize)).b32.s1;
        while r != PAGE_INS_HEAD {
            let next_r = (*mem.add(r as usize)).b32.s1;
            free_node(r, PAGE_INS_NODE_SIZE);
            r = next_r;
        }
        (*mem.add(PAGE_INS_HEAD as usize)).b32.s1 = PAGE_INS_HEAD;

        if sa_root[MARK_VAL as usize] != TEX_NULL {
            if do_marks((FIRE_UP_DONE) as i16, 0, sa_root[MARK_VAL as usize]) {
                sa_root[MARK_VAL as usize] = TEX_NULL;
            }
        }

        if cur_mark[TOP_MARK_CODE as usize] != TEX_NULL
            && cur_mark[FIRST_MARK_CODE as usize] == TEX_NULL
        {
            cur_mark[FIRST_MARK_CODE as usize] = cur_mark[TOP_MARK_CODE as usize];
            (*mem.add(cur_mark[TOP_MARK_CODE as usize] as usize)).b32.s0 += 1;
        }

        if (*eqtb.add((LOCAL_BASE + LOCAL__output_routine) as usize))
            .b32
            .s1
            != TEX_NULL
        {
            if dead_cycles
                >= (*eqtb.add((INT_BASE + INT_PAR__max_dead_cycles) as usize))
                    .b32
                    .s1
            {
                error_here_with_diagnostic(b"Output loop---\0".as_ptr() as *const c_char);
                print_int(dead_cycles);
                print_cstr(b" consecutive dead cycles\0".as_ptr() as *const c_char);
                capture_to_diagnostic(ptr::null_mut());
                help_ptr = 3;
                help_line[2] = b"I've concluded that your \\output is awry; it never does a\0"
                    .as_ptr() as *const c_char;
                help_line[1] = b"\\shipout, so I'm shipping \\box255 out myself. Next time\0"
                    .as_ptr() as *const c_char;
                help_line[0] = b"increase \\maxdeadcycles if you want me to be more patient!\0"
                    .as_ptr() as *const c_char;
                error();
            } else {
                output_active = true;
                dead_cycles += 1;
                push_nest();
                cur_list.mode = (-VMODE) as i16;
                cur_list.aux.b32.s1 = IGNORE_DEPTH;
                cur_list.mode_line = -line;
                begin_token_list(
                    (*eqtb.add((LOCAL_BASE + LOCAL__output_routine) as usize))
                        .b32
                        .s1,
                    (OUTPUT_TEXT) as u16,
                );
                new_save_level((OUTPUT_GROUP) as u8);
                normal_paragraph();
                scan_left_brace();
                return;
            }
        }

        if (*mem.add(PAGE_HEAD as usize)).b32.s1 != TEX_NULL {
            if (*mem.add(CONTRIB_HEAD as usize)).b32.s1 == TEX_NULL {
                if nest_ptr == 0 {
                    cur_list.tail = page_tail;
                } else {
                    (*nest.add(0)).tail = page_tail;
                }
            } else {
                (*mem.add(page_tail as usize)).b32.s1 = (*mem.add(CONTRIB_HEAD as usize)).b32.s1;
            }

            (*mem.add(CONTRIB_HEAD as usize)).b32.s1 = (*mem.add(PAGE_HEAD as usize)).b32.s1;
            (*mem.add(PAGE_HEAD as usize)).b32.s1 = TEX_NULL;
            page_tail = PAGE_HEAD;
        }

        flush_node_list(disc_ptr[LAST_BOX_CODE as usize]);
        disc_ptr[LAST_BOX_CODE as usize] = TEX_NULL;
        ship_out((*eqtb.add((BOX_BASE + 255) as usize)).b32.s1);
        (*eqtb.add((BOX_BASE + 255) as usize)).b32.s1 = TEX_NULL;
    }
}

const AWFUL_BAD: i32 = MAX_HALFWORD;

#[inline]
fn is_non_discardable_node(p: i32) -> bool {
    unsafe { ((*mem.add(p as usize)).b16.s1 as i32) < MATH_NODE }
}

pub unsafe fn build_page() {
    unsafe {
        if (*mem.add(CONTRIB_HEAD as usize)).b32.s1 == TEX_NULL || output_active {
            return;
        }

        loop {
            let p = (*mem.add(CONTRIB_HEAD as usize)).b32.s1;

            if last_glue != MAX_HALFWORD {
                delete_glue_ref(last_glue);
            }

            last_penalty = 0;
            last_kern = 0;
            last_node_type = ((*mem.add(p as usize)).b16.s1 as i32) + 1;

            let node_type = (*mem.add(p as usize)).b16.s1 as i32;
            if node_type == GLUE_NODE {
                last_glue = (*mem.add((p + 1) as usize)).b32.s0;
                (*mem.add(last_glue as usize)).b32.s1 += 1;
            } else {
                last_glue = MAX_HALFWORD;
                if node_type == PENALTY_NODE {
                    last_penalty = (*mem.add((p + 1) as usize)).b32.s1;
                } else if node_type == KERN_NODE {
                    last_kern = (*mem.add((p + 1) as usize)).b32.s1;
                }
            }

            let mut pi: i32 = 0;
            let mut goto_contribute = false;
            let mut goto_done1 = false;
            let mut goto_update_heights = false;

            match node_type {
                HLIST_NODE | VLIST_NODE | RULE_NODE => {
                    if page_contents < (BOX_THERE) as u8 {
                        if page_contents == (EMPTY as i32) as u8 {
                            freeze_page_specs(BOX_THERE as u8);
                        } else {
                            page_contents = (BOX_THERE) as u8;
                        }

                        let q = new_skip_param((GLUE_PAR__top_skip) as i16);
                        if (*mem.add((temp_ptr + 1) as usize)).b32.s1
                            > (*mem.add((p + 3) as usize)).b32.s1
                        {
                            (*mem.add((temp_ptr + 1) as usize)).b32.s1 -=
                                (*mem.add((p + 3) as usize)).b32.s1;
                        } else {
                            (*mem.add((temp_ptr + 1) as usize)).b32.s1 = 0;
                        }

                        (*mem.add(q as usize)).b32.s1 = p;
                        (*mem.add(CONTRIB_HEAD as usize)).b32.s1 = q;
                        continue;
                    } else {
                        page_so_far[1] += page_so_far[7] + (*mem.add((p + 3) as usize)).b32.s1;
                        page_so_far[7] = (*mem.add((p + 2) as usize)).b32.s1;
                        goto_contribute = true;
                    }
                }

                WHATSIT_NODE => {
                    let subtype = (*mem.add(p as usize)).b16.s0 as i32;
                    if subtype == PIC_NODE || subtype == PDF_NODE {
                        page_so_far[1] += page_so_far[7] + (*mem.add((p + 3) as usize)).b32.s1;
                        page_so_far[7] = (*mem.add((p + 2) as usize)).b32.s1;
                    }
                    goto_contribute = true;
                }

                GLUE_NODE => {
                    if page_contents < (BOX_THERE) as u8 {
                        goto_done1 = true;
                    } else if is_non_discardable_node(page_tail) {
                        pi = 0;
                    } else {
                        goto_update_heights = true;
                    }
                }

                KERN_NODE => {
                    if page_contents < (BOX_THERE) as u8 {
                        goto_done1 = true;
                    } else if (*mem.add(p as usize)).b32.s1 == TEX_NULL {
                        return;
                    } else if (*mem.add((*mem.add(p as usize)).b32.s1 as usize)).b16.s1 as i32
                        == GLUE_NODE
                    {
                        pi = 0;
                    } else {
                        goto_update_heights = true;
                    }
                }

                PENALTY_NODE => {
                    if page_contents < (BOX_THERE) as u8 {
                        goto_done1 = true;
                    } else {
                        pi = (*mem.add((p + 1) as usize)).b32.s1;
                    }
                }

                MARK_NODE => {
                    goto_contribute = true;
                }

                INS_NODE => {
                    if page_contents == (EMPTY as i32) as u8 {
                        freeze_page_specs(INSERTS_ONLY as u8);
                    }

                    let n = (*mem.add(p as usize)).b16.s0 as u8;
                    let mut r = PAGE_INS_HEAD;
                    while n >= (*mem.add((*mem.add(r as usize)).b32.s1 as usize)).b16.s0 as u8 {
                        r = (*mem.add(r as usize)).b32.s1;
                    }

                    if (*mem.add(r as usize)).b16.s0 as u8 != n {
                        let q = get_node(PAGE_INS_NODE_SIZE);
                        (*mem.add(q as usize)).b32.s1 = (*mem.add(r as usize)).b32.s1;
                        (*mem.add(r as usize)).b32.s1 = q;
                        r = q;

                        (*mem.add(r as usize)).b16.s0 = n as u16;
                        (*mem.add(r as usize)).b16.s1 = INSERTING as u16;
                        ensure_vbox(n);

                        let box_reg_n = (*eqtb.add((BOX_BASE + n as i32) as usize)).b32.s1;
                        if box_reg_n == TEX_NULL {
                            (*mem.add((r + 3) as usize)).b32.s1 = 0;
                        } else {
                            (*mem.add((r + 3) as usize)).b32.s1 =
                                (*mem.add((box_reg_n + 3) as usize)).b32.s1
                                    + (*mem.add((box_reg_n + 2) as usize)).b32.s1;
                        }

                        (*mem.add((r + 2) as usize)).b32.s0 = TEX_NULL;
                        let skip_q = (*eqtb.add((SKIP_BASE + n as i32) as usize)).b32.s1;

                        let h: scaled_t =
                            if (*eqtb.add((COUNT_BASE + n as i32) as usize)).b32.s1 == 1000 {
                                (*mem.add((r + 3) as usize)).b32.s1
                            } else {
                                x_over_n((*mem.add((r + 3) as usize)).b32.s1, 1000)
                                    * (*eqtb.add((COUNT_BASE + n as i32) as usize)).b32.s1
                            };

                        page_so_far[0] -= h + (*mem.add((skip_q + 1) as usize)).b32.s1;
                        page_so_far[(2 + (*mem.add(skip_q as usize)).b16.s1) as usize] +=
                            (*mem.add((skip_q + 2) as usize)).b32.s1;
                        page_so_far[6] += (*mem.add((skip_q + 3) as usize)).b32.s1;

                        if (*mem.add(skip_q as usize)).b16.s0 as i32 != NORMAL
                            && (*mem.add((skip_q + 3) as usize)).b32.s1 != 0
                        {
                            error_here_with_diagnostic(
                                b"Infinite glue shrinkage inserted from \0".as_ptr()
                                    as *const c_char,
                            );
                            print_esc_cstr(b"skip\0".as_ptr() as *const c_char);
                            print_int(n as i32);
                            capture_to_diagnostic(ptr::null_mut());
                            help_ptr = 3;
                            help_line[2] =
                                b"The correction glue for page breaking with insertions\0".as_ptr()
                                    as *const c_char;
                            help_line[1] = b"must have finite shrinkability. But you may proceed,\0"
                                .as_ptr()
                                as *const c_char;
                            help_line[0] =
                                b"since the offensive shrinkability has been made finite.\0"
                                    .as_ptr() as *const c_char;
                            error();
                        }
                    }

                    if (*mem.add(r as usize)).b16.s1 as i32 == SPLIT_UP {
                        insert_penalties += (*mem.add((p + 1) as usize)).b32.s1;
                    } else {
                        (*mem.add((r + 2) as usize)).b32.s1 = p;
                        let page_delta =
                            page_so_far[0] - page_so_far[1] - page_so_far[7] + page_so_far[6];

                        let h: scaled_t =
                            if (*eqtb.add((COUNT_BASE + n as i32) as usize)).b32.s1 == 1000 {
                                (*mem.add((p + 3) as usize)).b32.s1
                            } else {
                                x_over_n((*mem.add((p + 3) as usize)).b32.s1, 1000)
                                    * (*eqtb.add((COUNT_BASE + n as i32) as usize)).b32.s1
                            };

                        let scaled_n = (*eqtb.add((SCALED_BASE + n as i32) as usize)).b32.s1;
                        if (h <= 0 || h <= page_delta)
                            && ((*mem.add((p + 3) as usize)).b32.s1
                                + (*mem.add((r + 3) as usize)).b32.s1
                                <= scaled_n)
                        {
                            page_so_far[0] -= h;
                            (*mem.add((r + 3) as usize)).b32.s1 +=
                                (*mem.add((p + 3) as usize)).b32.s1;
                        } else {
                            let mut w: scaled_t =
                                if (*eqtb.add((COUNT_BASE + n as i32) as usize)).b32.s1 <= 0 {
                                    MAX_HALFWORD
                                } else {
                                    let mut val = page_so_far[0] - page_so_far[1] - page_so_far[7];
                                    if (*eqtb.add((COUNT_BASE + n as i32) as usize)).b32.s1 != 1000
                                    {
                                        val = x_over_n(
                                            val,
                                            (*eqtb.add((COUNT_BASE + n as i32) as usize)).b32.s1,
                                        ) * 1000;
                                    }
                                    val
                                };

                            if w > scaled_n - (*mem.add((r + 3) as usize)).b32.s1 {
                                w = scaled_n - (*mem.add((r + 3) as usize)).b32.s1;
                            }

                            let split_q = vert_break(
                                (*mem.add((p + 4) as usize)).b32.s0,
                                w,
                                (*mem.add((p + 2) as usize)).b32.s1,
                            );
                            (*mem.add((r + 3) as usize)).b32.s1 += best_height_plus_depth;

                            if (*eqtb.add((COUNT_BASE + n as i32) as usize)).b32.s1 != 1000 {
                                best_height_plus_depth = x_over_n(best_height_plus_depth, 1000)
                                    * (*eqtb.add((COUNT_BASE + n as i32) as usize)).b32.s1;
                            }

                            page_so_far[0] -= best_height_plus_depth;
                            (*mem.add(r as usize)).b16.s1 = SPLIT_UP as u16;
                            (*mem.add((r + 1) as usize)).b32.s1 = split_q;
                            (*mem.add((r + 1) as usize)).b32.s0 = p;

                            if split_q == TEX_NULL {
                                insert_penalties += EJECT_PENALTY;
                            } else if (*mem.add(split_q as usize)).b16.s1 as i32 == PENALTY_NODE {
                                insert_penalties += (*mem.add((split_q + 1) as usize)).b32.s1;
                            }
                        }
                    }
                    goto_contribute = true;
                }

                _ => {
                    confusion(b"page\0".as_ptr() as *const c_char);
                }
            }

            if !goto_contribute && !goto_done1 && !goto_update_heights {
                if pi < INF_PENALTY {
                    let b: i32 = if page_so_far[1] < page_so_far[0] {
                        if page_so_far[3] != 0 || page_so_far[4] != 0 || page_so_far[5] != 0 {
                            0
                        } else {
                            badness(page_so_far[0] - page_so_far[1], page_so_far[2])
                        }
                    } else if page_so_far[1] - page_so_far[0] > page_so_far[6] {
                        AWFUL_BAD
                    } else {
                        badness(page_so_far[1] - page_so_far[0], page_so_far[6])
                    };

                    let mut c: i32 = if b < AWFUL_BAD {
                        if pi <= EJECT_PENALTY {
                            pi
                        } else if b < INF_BAD {
                            b + pi + insert_penalties
                        } else {
                            100_000
                        }
                    } else {
                        b
                    };

                    if insert_penalties >= 10000 {
                        c = MAX_HALFWORD;
                    }

                    if c <= least_page_cost {
                        best_page_break = p;
                        best_size = page_so_far[0];
                        least_page_cost = c;
                        let mut r = (*mem.add(PAGE_INS_HEAD as usize)).b32.s1;
                        while r != PAGE_INS_HEAD {
                            (*mem.add((r + 2) as usize)).b32.s0 =
                                (*mem.add((r + 2) as usize)).b32.s1;
                            r = (*mem.add(r as usize)).b32.s1;
                        }
                    }

                    if c == AWFUL_BAD || pi <= EJECT_PENALTY {
                        fire_up(p);
                        if output_active {
                            return;
                        }
                        if (*mem.add(CONTRIB_HEAD as usize)).b32.s1 == TEX_NULL {
                            break;
                        }
                        continue;
                    }
                }

                let cur_node_type = (*mem.add(p as usize)).b16.s1 as i32;
                if cur_node_type < GLUE_NODE || cur_node_type > KERN_NODE {
                    goto_contribute = true;
                } else {
                    goto_update_heights = true;
                }
            }

            if goto_update_heights {
                let q_node = if (*mem.add(p as usize)).b16.s1 as i32 == KERN_NODE {
                    p
                } else {
                    let mut glue_q = (*mem.add((p + 1) as usize)).b32.s0;
                    page_so_far[(2 + (*mem.add(glue_q as usize)).b16.s1) as usize] +=
                        (*mem.add((glue_q + 2) as usize)).b32.s1;
                    page_so_far[6] += (*mem.add((glue_q + 3) as usize)).b32.s1;

                    if (*mem.add(glue_q as usize)).b16.s0 as i32 != NORMAL
                        && (*mem.add((glue_q + 3) as usize)).b32.s1 != 0
                    {
                        error_here_with_diagnostic(
                            b"Infinite glue shrinkage found on current page\0".as_ptr()
                                as *const c_char,
                        );
                        capture_to_diagnostic(ptr::null_mut());
                        help_ptr = 4;
                        help_line[3] = b"The page about to be output contains some infinitely\0"
                            .as_ptr() as *const c_char;
                        help_line[2] =
                            b"shrinkable glue, e.g., `\\vss' or `\\vskip 0pt minus 1fil'.\0"
                                .as_ptr() as *const c_char;
                        help_line[1] =
                            b"Such glue doesn't belong there; but you can safely proceed,\0"
                                .as_ptr() as *const c_char;
                        help_line[0] = b"since the offensive shrinkability has been made finite.\0"
                            .as_ptr() as *const c_char;
                        error();

                        let r = new_spec(glue_q);
                        (*mem.add(r as usize)).b16.s0 = NORMAL as u16;
                        delete_glue_ref(glue_q);
                        (*mem.add((p + 1) as usize)).b32.s0 = r;
                        glue_q = r;
                    }
                    glue_q
                };

                page_so_far[1] += page_so_far[7] + (*mem.add((q_node + 1) as usize)).b32.s1;
                page_so_far[7] = 0;
                goto_contribute = true;
            }

            if goto_contribute {
                if page_so_far[7] > page_max_depth {
                    page_so_far[1] += page_so_far[7] - page_max_depth;
                    page_so_far[7] = page_max_depth;
                }

                (*mem.add(page_tail as usize)).b32.s1 = p;
                page_tail = p;
                (*mem.add(CONTRIB_HEAD as usize)).b32.s1 = (*mem.add(p as usize)).b32.s1;
                (*mem.add(p as usize)).b32.s1 = TEX_NULL;
            } else if goto_done1 {
                (*mem.add(CONTRIB_HEAD as usize)).b32.s1 = (*mem.add(p as usize)).b32.s1;
                (*mem.add(p as usize)).b32.s1 = TEX_NULL;

                if (*eqtb.add((INT_BASE + INT_PAR__saving_vdiscards) as usize))
                    .b32
                    .s1
                    <= 0
                {
                    flush_node_list(p);
                } else {
                    if disc_ptr[LAST_BOX_CODE as usize] == TEX_NULL {
                        disc_ptr[LAST_BOX_CODE as usize] = p;
                    } else {
                        (*mem.add(disc_ptr[COPY_CODE as usize] as usize)).b32.s1 = p;
                    }
                    disc_ptr[COPY_CODE as usize] = p;
                }
            }

            if (*mem.add(CONTRIB_HEAD as usize)).b32.s1 == TEX_NULL {
                break;
            }
        }

        if nest_ptr == 0 {
            cur_list.tail = CONTRIB_HEAD;
        } else {
            (*nest.add(0)).tail = CONTRIB_HEAD;
        }
    }
}
