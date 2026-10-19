// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_xetexd::*;
use super::xetex_xetexd::{MIN_TRIE_OP, TRIE_OP_SIZE};
use libc::{c_char, c_void};

unsafe extern "C" {
    static mut _trie_op_hash_array: [i32; (2 * TRIE_OP_SIZE + 1) as usize];
    fn xmalloc(size: usize) -> *mut c_void;
    fn maketexstring(value: *const c_char) -> str_number;
}

pub(super) unsafe fn initialize_more_variables() {
    unsafe {
        doing_special = false;
        native_text_size = 128;
        native_text = xmalloc(native_text_size as usize * std::mem::size_of::<UTF16_code>()).cast();

        interaction = ERROR_STOP_MODE as u8;

        deletions_allowed = true;
        set_box_allowed = true;
        error_count = 0;
        help_ptr = 0;
        use_err_help = false;

        two_to_the[(0) as usize] = 1;
        for k in 1..=30 {
            two_to_the[(k) as usize] = 2 * two_to_the[(k - 1) as usize];
        }

        spec_log[(1) as usize] = 93032640;
        spec_log[(2) as usize] = 38612034;
        spec_log[(3) as usize] = 17922280;
        spec_log[(4) as usize] = 8662214;
        spec_log[(5) as usize] = 4261238;
        spec_log[(6) as usize] = 2113709;
        spec_log[(7) as usize] = 1052693;
        spec_log[(8) as usize] = 525315;
        spec_log[(9) as usize] = 262400;
        spec_log[(10) as usize] = 131136;
        spec_log[(11) as usize] = 65552;
        spec_log[(12) as usize] = 32772;
        spec_log[(13) as usize] = 16385;
        for k in 14..=27 {
            spec_log[(k) as usize] = two_to_the[(27 - k) as usize];
        }
        spec_log[(28) as usize] = 1;

        nest_ptr = 0;
        max_nest_stack = 0;
        cur_list.mode = VMODE as i16;
        cur_list.head = CONTRIB_HEAD;
        cur_list.tail = CONTRIB_HEAD;
        cur_list.eTeX_aux = TEX_NULL;
        cur_list.aux.b32.s1 = IGNORE_DEPTH;
        cur_list.mode_line = 0;
        cur_list.prev_graf = 0;
        shown_mode = 0;
        page_contents = EMPTY as u8;
        page_tail = PAGE_HEAD;
        last_glue = MAX_HALFWORD;
        last_penalty = 0;
        last_kern = 0;
        last_node_type = -1;
        page_so_far[(7) as usize] = 0;

        for k in INT_BASE..=EQTB_SIZE {
            _xeq_level_array[((k - INT_BASE) as usize) as usize] = LEVEL_ONE as u16;
        }

        no_new_control_sequence = true;
        prim[(0) as usize].s0 = 0;
        prim[(0) as usize].s1 = 0;

        for k in 1..=PRIM_SIZE {
            prim[(k) as usize] = prim[(0) as usize];
        }

        save_ptr = 0;
        cur_level = LEVEL_ONE as u16;
        cur_group = BOTTOM_LEVEL as u8;
        cur_boundary = 0;
        max_save_stack = 0;
        mag_set = 0;
        expand_depth_count = 0;
        is_in_csname = false;
        cur_mark[(TOP_MARK_CODE) as usize] = TEX_NULL;
        cur_mark[(FIRST_MARK_CODE) as usize] = TEX_NULL;
        cur_mark[(BOT_MARK_CODE) as usize] = TEX_NULL;
        cur_mark[(SPLIT_FIRST_MARK_CODE) as usize] = TEX_NULL;
        cur_mark[(SPLIT_BOT_MARK_CODE) as usize] = TEX_NULL;
        cur_val = 0;
        cur_val_level = INT_VAL as u8;
        radix = 0;
        cur_order = NORMAL as u8;

        for k in 0..=16 {
            read_open[(k) as usize] = CLOSED as u8;
        }

        cond_ptr = TEX_NULL;
        if_limit = NORMAL as u8;
        cur_if = 0;
        if_line = 0;
        null_character.s3 = 0;
        null_character.s2 = 0;
        null_character.s1 = 0;
        null_character.s0 = 0;
        total_pages = 0;
        max_v = 0;
        max_h = 0;
        max_push = 0;
        last_bop = -1;
        doing_leaders = false;
        dead_cycles = 0;
        adjust_tail = TEX_NULL;
        last_badness = 0;
        pre_adjust_tail = TEX_NULL;
        pack_begin_line = 0;
        empty.s1 = EMPTY;
        empty.s0 = TEX_NULL;
        align_ptr = TEX_NULL;
        cur_align = TEX_NULL;
        cur_span = TEX_NULL;
        cur_loop = TEX_NULL;
        cur_head = TEX_NULL;
        cur_tail = TEX_NULL;
        cur_pre_head = TEX_NULL;
        cur_pre_tail = TEX_NULL;
        cur_f = 0;
        max_hyph_char = TOO_BIG_LANG;

        for z in 0..=hyph_size {
            (*hyph_word.add((z) as usize)) = 0;
            (*hyph_list.add((z) as usize)) = TEX_NULL;
            (*hyph_link.add((z) as usize)) = 0;
        }

        hyph_count = 0;
        hyph_next = (HYPH_PRIME + 1);
        if hyph_next > hyph_size {
            hyph_next = HYPH_PRIME;
        }

        output_active = false;
        insert_penalties = 0;
        ligature_present = false;
        cancel_boundary = false;
        lft_hit = false;
        rt_hit = false;
        ins_disc = false;
        after_token = 0;
        long_help_seen = false;
        format_ident = 0;

        for k in 0..=17 {
            write_open[(k) as usize] = false;
        }

        LR_ptr = TEX_NULL;
        LR_problems = 0;
        cur_dir = LEFT_TO_RIGHT as i16;
        pseudo_files = TEX_NULL;
        sa_root[(MARK_VAL) as usize] = TEX_NULL;
        sa_null.b32.s0 = TEX_NULL;
        sa_null.b32.s1 = TEX_NULL;
        sa_chain = TEX_NULL;
        sa_level = LEVEL_ZERO as u16;
        disc_ptr[(LAST_BOX_CODE) as usize] = TEX_NULL;
        disc_ptr[(VSPLIT_CODE) as usize] = TEX_NULL;
        edit_name_start = 0;
        stop_at_space = true;
    }
}

pub(super) unsafe fn initialize_more_initex_variables() {
    unsafe {
        for k in 1..=19 {
            (*mem.add((k) as usize)).b32.s1 = 0;
        }

        for k in (0..=19).step_by(4) {
            (*mem.add((k) as usize)).b32.s1 = TEX_NULL + 1;
            (*mem.add((k) as usize)).b16.s1 = NORMAL as u16;
            (*mem.add((k) as usize)).b16.s0 = NORMAL as u16;
        }

        (*mem.add((6) as usize)).b32.s1 = 65536;
        (*mem.add((4) as usize)).b16.s1 = FIL as u16;
        (*mem.add((10) as usize)).b32.s1 = 65536;
        (*mem.add((8) as usize)).b16.s1 = FILL as u16;
        (*mem.add((14) as usize)).b32.s1 = 65536;
        (*mem.add((12) as usize)).b16.s1 = FIL as u16;
        (*mem.add((15) as usize)).b32.s1 = 65536;
        (*mem.add((12) as usize)).b16.s0 = FIL as u16;
        (*mem.add((18) as usize)).b32.s1 = -65536;
        (*mem.add((16) as usize)).b16.s1 = FIL as u16;
        rover = 20;
        (*mem.add((rover) as usize)).b32.s1 = MAX_HALFWORD;
        (*mem.add((rover) as usize)).b32.s0 = 1000;
        (*mem.add((rover + 1) as usize)).b32.s0 = rover;
        (*mem.add((rover + 1) as usize)).b32.s1 = rover;
        lo_mem_max = rover + 1000;
        (*mem.add((lo_mem_max) as usize)).b32.s1 = TEX_NULL;
        (*mem.add((lo_mem_max) as usize)).b32.s0 = TEX_NULL;

        for k in PRE_ADJUST_HEAD..=MEM_TOP {
            (*mem.add((k) as usize)) = (*mem.add((lo_mem_max) as usize));
        }

        (*mem.add((OMIT_TEMPLATE) as usize)).b32.s0 = CS_TOKEN_FLAG + FROZEN_END_TEMPLATE;
        (*mem.add((END_SPAN) as usize)).b32.s1 = u16::MAX as i32 + 1;
        (*mem.add((END_SPAN) as usize)).b32.s0 = TEX_NULL;
        (*mem.add((ACTIVE_LIST) as usize)).b16.s1 = HYPHENATED as u16;
        (*mem.add((ACTIVE_LIST + 1) as usize)).b32.s0 = MAX_HALFWORD;
        (*mem.add((ACTIVE_LIST) as usize)).b16.s0 = 0;
        (*mem.add((PAGE_INS_HEAD) as usize)).b16.s0 = 255;
        (*mem.add((PAGE_INS_HEAD) as usize)).b16.s1 = SPLIT_UP as u16;
        (*mem.add((PAGE_INS_HEAD) as usize)).b32.s1 = PAGE_INS_HEAD;
        (*mem.add((PAGE_HEAD) as usize)).b16.s1 = GLUE_NODE as u16;
        (*mem.add((PAGE_HEAD) as usize)).b16.s0 = NORMAL as u16;
        avail = TEX_NULL;
        mem_end = MEM_TOP;
        hi_mem_min = PRE_ADJUST_HEAD;
        var_used = 20;
        dyn_used = HI_MEM_STAT_USAGE;
        (*eqtb.add((UNDEFINED_CONTROL_SEQUENCE) as usize)).b16.s1 = UNDEFINED_CS as u16;
        (*eqtb.add((UNDEFINED_CONTROL_SEQUENCE) as usize)).b32.s1 = TEX_NULL;
        (*eqtb.add((UNDEFINED_CONTROL_SEQUENCE) as usize)).b16.s0 = LEVEL_ZERO as u16;

        for k in ACTIVE_BASE..=eqtb_top {
            (*eqtb.add((k) as usize)) = (*eqtb.add((UNDEFINED_CONTROL_SEQUENCE) as usize));
        }

        (*eqtb.add((GLUE_BASE) as usize)).b32.s1 = 0;
        (*eqtb.add((GLUE_BASE) as usize)).b16.s0 = LEVEL_ONE as u16;
        (*eqtb.add((GLUE_BASE) as usize)).b16.s1 = GLUE_REF as u16;

        for k in GLUE_BASE + 1..=LOCAL_BASE - 1 {
            (*eqtb.add((k) as usize)) = (*eqtb.add((GLUE_BASE) as usize));
        }

        (*mem.add((0) as usize)).b32.s1 += 531;
        (*eqtb.add((LOCAL_BASE + LOCAL__par_shape) as usize)).b32.s1 = TEX_NULL;
        (*eqtb.add((LOCAL_BASE + LOCAL__par_shape) as usize)).b16.s1 = SHAPE_REF as u16;
        (*eqtb.add((LOCAL_BASE + LOCAL__par_shape) as usize)).b16.s0 = LEVEL_ONE as u16;

        for k in ETEX_PEN_BASE..ETEX_PEN_BASE + NUM_ETEX_PENALTIES {
            (*eqtb.add((k) as usize)) = (*eqtb.add((LOCAL_BASE + LOCAL__par_shape) as usize));
        }

        for k in LOCAL_BASE + LOCAL__output_routine..=TOKS_BASE + NUMBER_REGS - 1 {
            (*eqtb.add((k) as usize)) = (*eqtb.add((UNDEFINED_CONTROL_SEQUENCE) as usize));
        }

        (*eqtb.add((BOX_BASE) as usize)).b32.s1 = TEX_NULL;
        (*eqtb.add((BOX_BASE) as usize)).b16.s1 = BOX_REF as u16;
        (*eqtb.add((BOX_BASE) as usize)).b16.s0 = LEVEL_ONE as u16;

        for k in BOX_BASE + 1..=BOX_BASE + NUMBER_REGS - 1 {
            (*eqtb.add((k) as usize)) = (*eqtb.add((BOX_BASE) as usize));
        }

        (*eqtb.add((CUR_FONT_LOC) as usize)).b32.s1 = FONT_BASE;
        (*eqtb.add((CUR_FONT_LOC) as usize)).b16.s1 = DATA as u16;
        (*eqtb.add((CUR_FONT_LOC) as usize)).b16.s0 = LEVEL_ONE as u16;

        for k in MATH_FONT_BASE..=MATH_FONT_BASE + NUMBER_MATH_FONTS - 1 {
            (*eqtb.add((k) as usize)) = (*eqtb.add((CUR_FONT_LOC) as usize));
        }

        (*eqtb.add((CAT_CODE_BASE) as usize)).b32.s1 = 0;
        (*eqtb.add((CAT_CODE_BASE) as usize)).b16.s1 = DATA as u16;
        (*eqtb.add((CAT_CODE_BASE) as usize)).b16.s0 = LEVEL_ONE as u16;

        for k in CAT_CODE_BASE + 1..=INT_BASE - 1 {
            (*eqtb.add((k) as usize)) = (*eqtb.add((CAT_CODE_BASE) as usize));
        }

        for k in 0..=NUMBER_USVS - 1 {
            (*eqtb.add((CAT_CODE_BASE + k) as usize)).b32.s1 = OTHER_CHAR;
            (*eqtb.add((MATH_CODE_BASE + k) as usize)).b32.s1 = k;
            (*eqtb.add((SF_CODE_BASE + k) as usize)).b32.s1 = 1000;
        }

        (*eqtb.add((CAT_CODE_BASE + 13) as usize)).b32.s1 = CAR_RET;
        (*eqtb.add((CAT_CODE_BASE + 32) as usize)).b32.s1 = SPACER;
        (*eqtb.add((CAT_CODE_BASE + 92) as usize)).b32.s1 = ESCAPE;
        (*eqtb.add((CAT_CODE_BASE + 37) as usize)).b32.s1 = COMMENT;
        (*eqtb.add((CAT_CODE_BASE + 127) as usize)).b32.s1 = INVALID_CHAR;
        (*eqtb.add((CAT_CODE_BASE) as usize)).b32.s1 = IGNORE;

        for k in (b'0' as i32)..=(b'9' as i32) {
            (*eqtb.add((MATH_CODE_BASE + k) as usize)).b32.s1 =
                k + ((VAR_FAM_CLASS as u32 & 0x07) << 21) as i32;
        }

        for k in (b'A' as i32)..=(b'Z' as i32) {
            (*eqtb.add((CAT_CODE_BASE + k) as usize)).b32.s1 = LETTER;
            (*eqtb.add((CAT_CODE_BASE + k + 32) as usize)).b32.s1 = LETTER;
            (*eqtb.add((MATH_CODE_BASE + k) as usize)).b32.s1 =
                k + (1 << 24) + ((VAR_FAM_CLASS as u32 & 0x07) << 21) as i32;
            (*eqtb.add((MATH_CODE_BASE + k + 32) as usize)).b32.s1 =
                k + 32 + (1 << 24) + ((VAR_FAM_CLASS as u32 & 0x07) << 21) as i32;
            (*eqtb.add((LC_CODE_BASE + k) as usize)).b32.s1 = k + 32;
            (*eqtb.add((LC_CODE_BASE + k + 32) as usize)).b32.s1 = k + 32;
            (*eqtb.add((UC_CODE_BASE + k) as usize)).b32.s1 = k;
            (*eqtb.add((UC_CODE_BASE + k + 32) as usize)).b32.s1 = k;
            (*eqtb.add((SF_CODE_BASE + k) as usize)).b32.s1 = 999;
        }

        for k in INT_BASE..=DEL_CODE_BASE - 1 {
            (*eqtb.add((k) as usize)).b32.s1 = 0;
        }

        (*eqtb.add((INT_BASE + INT_PAR__mag) as usize)).b32.s1 = 1000;
        (*eqtb.add((INT_BASE + INT_PAR__tolerance) as usize)).b32.s1 = 10000;
        (*eqtb.add((INT_BASE + INT_PAR__hang_after) as usize))
            .b32
            .s1 = 1;
        (*eqtb.add((INT_BASE + INT_PAR__max_dead_cycles) as usize))
            .b32
            .s1 = 25;
        (*eqtb.add((INT_BASE + INT_PAR__escape_char) as usize))
            .b32
            .s1 = b'\\' as i32;
        (*eqtb.add((INT_BASE + INT_PAR__end_line_char) as usize))
            .b32
            .s1 = CARRIAGE_RETURN;

        for k in 0..=NUMBER_USVS - 1 {
            (*eqtb.add((DEL_CODE_BASE + k) as usize)).b32.s1 = -1;
        }

        (*eqtb.add((DEL_CODE_BASE + 46/* '.' */) as usize)).b32.s1 = 0;

        for k in DIMEN_BASE..=EQTB_SIZE {
            (*eqtb.add((k) as usize)).b32.s1 = 0;
        }

        prim_used = PRIM_SIZE;
        hash_used = FROZEN_CONTROL_SEQUENCE;
        hash_high = 0;
        cs_count = 0;

        std::ptr::write_bytes(
            (&raw mut _trie_op_hash_array).cast::<i32>(),
            0,
            (2 * TRIE_OP_SIZE + 1) as usize,
        );

        for k in 0..=BIGGEST_LANG {
            trie_used[(k) as usize] = MIN_TRIE_OP as u16;
        }

        max_op_used = MIN_TRIE_OP as u16;
        trie_op_ptr = 0;
        trie_not_ready = true;

        format_ident = maketexstring(c" (INITEX)".as_ptr());

        max_reg_num = 32767;
        max_reg_help_line = c"A register number must be between 0 and 32767.".as_ptr();

        for i in INT_VAL..=INTER_CHAR_VAL {
            sa_root[(i) as usize] = TEX_NULL;
        }

        (*eqtb.add((INT_BASE + INT_PAR__xetex_hyphenatable_length) as usize))
            .b32
            .s1 = 63;
    }
}
