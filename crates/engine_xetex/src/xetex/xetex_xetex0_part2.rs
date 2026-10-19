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

use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_xetexd::*;
use libc::*;

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

macro_rules! set_help_line {
    ($idx:expr, $str:expr) => {
        *(&raw mut help_line)
            .cast::<*const libc::c_char>()
            .offset($idx) = $str;
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
            scan_glue(GLUE_VAL as small_number);
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
            scan_glue(GLUE_VAL as small_number);
        }
        5 => {
            scan_glue(MU_VAL as small_number);
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
