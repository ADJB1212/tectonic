// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

use super::xetex_cleanup::final_cleanup;
use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_format::{
    XETEX_FORMAT_PRIMITIVE_INITIALIZERS, xf_prim_init_none_i32, xf_prim_init_par_i32,
    xf_prim_init_write_i32,
};
use super::xetex_initialize::{initialize_more_initex_variables, initialize_more_variables};
use super::xetex_xetexd::*;
use libc::{c_char, c_void, time_t};

unsafe extern "C" {
    static mut yhash: *mut b32x2;
    static mut stdin_ufile: UFILE;
    fn xmalloc(size: usize) -> *mut c_void;
    fn xcalloc(count: usize, size: usize) -> *mut c_void;
    fn _tt_abort(format: *const c_char, ...) -> !;
    fn ttstub_output_open_stdout() -> rust_output_handle_t;
    fn tt_checkpoint_resume() -> bool;
    fn pdf_files_init();
    fn synctex_init_command();
    fn maketexstring(value: *const c_char) -> str_number;
    fn tt_xetex_native_load_format() -> bool;
}

macro_rules! int_par {
    ($parameter:ident) => {
        (*eqtb.add((INT_BASE + $parameter) as usize)).b32.s1
    };
}

unsafe fn malloc_array<T>(last_index: i32) -> *mut T {
    unsafe { xmalloc((last_index as usize + 1) * std::mem::size_of::<T>()).cast() }
}

unsafe fn calloc_array<T>(last_index: i32) -> *mut T {
    unsafe { xcalloc(last_index as usize + 1, std::mem::size_of::<T>()).cast() }
}

unsafe fn init_io() {
    unsafe {
        stdin_ufile.handle = 0;
        stdin_ufile.savedChar = -1;
        stdin_ufile.skipNextLF = 0;
        stdin_ufile.encodingMode = super::xetex_io::UTF8 as i16;
        stdin_ufile.conversionData = std::ptr::null_mut();
        input_file.write(&raw mut stdin_ufile);
        buffer.add(first as usize).write(0);
        last = first;
        cur_input.loc = first;
        cur_input.limit = last;
        first = last + 1;
    }
}

unsafe fn get_strings_started() {
    unsafe {
        pool_ptr = 0;
        str_start.write(0);
        str_ptr = TOO_BIG_CHAR;
        if load_pool_strings(pool_size - string_vacancies) == 0 {
            _tt_abort(c"must increase pool_size".as_ptr());
        }
    }
}

unsafe fn initialize_primitives() {
    unsafe {
        no_new_control_sequence = false;
        first = 0;
        for definition in XETEX_FORMAT_PRIMITIVE_INITIALIZERS {
            let Some(name) = definition.name else {
                continue;
            };
            super::xetex_ini::primitive(name.as_ptr(), definition.cmd as u16, definition.chr);
            match definition.extra_init {
                xf_prim_init_none_i32 => {}
                xf_prim_init_par_i32 => {
                    par_loc = cur_val;
                    par_token = CS_TOKEN_FLAG + par_loc;
                }
                xf_prim_init_write_i32 => write_loc = cur_val,
                index => {
                    (*hash.add(index as usize)).s1 = maketexstring(name.as_ptr());
                    eqtb.add(index as usize).write(*eqtb.add(cur_val as usize));
                }
            }
        }
        for (index, command) in [(FROZEN_END_TEMPLATE, END_TEMPLATE), (FROZEN_ENDV, ENDV)] {
            (*hash.add(index as usize)).s1 = maketexstring(c"endtemplate".as_ptr());
            let entry = eqtb.add(index as usize);
            (*entry).b16.s1 = command as u16;
            (*entry).b32.s1 = NULL_LIST;
            (*entry).b16.s0 = LEVEL_ONE as u16;
        }
        (*hash.add(FROZEN_DONT_EXPAND as usize)).s1 = maketexstring(c"notexpanded:".as_ptr());
        (*eqtb.add(FROZEN_DONT_EXPAND as usize)).b16.s1 = DONT_EXPAND as u16;
        (*hash.add(FROZEN_PRIMITIVE as usize)).s1 = maketexstring(c"primitive".as_ptr());
        (*eqtb.add(FROZEN_PRIMITIVE as usize)).b16.s1 = IGNORE_SPACES as u16;
        (*eqtb.add(FROZEN_PRIMITIVE as usize)).b32.s1 = 1;
        (*eqtb.add(FROZEN_PRIMITIVE as usize)).b16.s0 = LEVEL_ONE as u16;
        (*hash.add(FROZEN_PROTECTION as usize)).s1 = maketexstring(c"inaccessible".as_ptr());
        (*hash.add(END_WRITE as usize)).s1 = maketexstring(c"endwrite".as_ptr());
        (*eqtb.add(END_WRITE as usize)).b16.s0 = LEVEL_ONE as u16;
        (*eqtb.add(END_WRITE as usize)).b16.s1 = OUTER_CALL as u16;
        (*eqtb.add(END_WRITE as usize)).b32.s1 = TEX_NULL;
        no_new_control_sequence = true;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tt_run_engine(
    dump_name: *const c_char,
    input_file_name: *const c_char,
    build_date: time_t,
) -> tt_history_t {
    unsafe {
        if tt_checkpoint_resume() {
            main_control();
            final_cleanup();
            close_files_and_terminate();
            tt_cleanup();
            return history;
        }
        rust_stdout = ttstub_output_open_stdout();
        let len = libc::strlen(dump_name);
        TEX_format_default = xmalloc(len + 1).cast();
        libc::strcpy(TEX_format_default, dump_name);
        if len > i32::MAX as usize {
            _tt_abort(c"failed internal consistency check #31".as_ptr());
        }
        format_default_length = len as i32;
        if file_line_error_style_p < 0 {
            file_line_error_style_p = 0;
        }
        pool_size = 6250000;
        string_vacancies = 90000;
        pool_free = 47500;
        max_strings = 565536;
        strings_free = 100;
        font_mem_size = 8000000;
        font_max = 9000;
        trie_size = 1000000;
        hyph_size = 8191;
        buf_size = 200000;
        nest_size = 500;
        max_in_open = 15;
        param_size = 10000;
        save_size = 80000;
        stack_size = 5000;
        error_line = 79;
        half_error_line = 50;
        max_print_line = 79;
        hash_extra = 600000;
        expand_depth = 10000;
        buffer = malloc_array::<UnicodeScalar>(buf_size);
        nest = malloc_array::<list_state_record>(nest_size);
        save_stack = malloc_array::<memory_word>(save_size);
        input_stack = malloc_array::<input_state_t>(stack_size);
        input_file = malloc_array::<*mut UFILE>(max_in_open);
        line_stack = malloc_array::<i32>(max_in_open);
        eof_seen = malloc_array::<bool>(max_in_open);
        grp_stack = malloc_array::<save_pointer>(max_in_open);
        if_stack = malloc_array::<i32>(max_in_open);
        source_filename_stack = malloc_array::<str_number>(max_in_open);
        full_source_filename_stack = malloc_array::<str_number>(max_in_open);
        param_stack = malloc_array::<i32>(param_size);
        hyph_word = malloc_array::<str_number>(hyph_size);
        hyph_list = malloc_array::<i32>(hyph_size);
        hyph_link = malloc_array::<hyph_pointer>(hyph_size);
        if in_initex_mode {
            mem = malloc_array::<memory_word>(MEM_TOP + 1);
            eqtb_top = EQTB_SIZE + hash_extra;
            if hash_extra == 0 {
                hash_top = UNDEFINED_CONTROL_SEQUENCE;
            } else {
                hash_top = eqtb_top;
            }
            yhash = malloc_array::<b32x2>(1 + hash_top - HASH_OFFSET);
            hash = yhash.wrapping_sub(HASH_OFFSET as usize);
            (*hash.add(HASH_BASE as usize)).s0 = 0;
            (*hash.add(HASH_BASE as usize)).s1 = 0;
            hash_used = HASH_BASE + 1;
            while hash_used <= hash_top {
                *hash.add(hash_used as usize) = (*hash.add(HASH_BASE as usize));
                hash_used += 1;
            }
            eqtb = calloc_array::<memory_word>(eqtb_top);
            str_start = malloc_array::<pool_pointer>(max_strings);
            str_pool = malloc_array::<packed_UTF16_code>(pool_size);
            font_info = malloc_array::<memory_word>(font_mem_size);
        }
        history = HISTORY_FATAL_ERROR;
        bad = 0;
        if half_error_line < 30 || half_error_line > error_line - 15 {
            bad = 1;
        }
        if max_print_line < 60 {
            bad = 2;
        }
        if 1100 > MEM_TOP {
            bad = 4;
        }
        if HASH_PRIME > HASH_SIZE {
            bad = 5;
        }
        if max_in_open >= 128 {
            bad = 6;
        }
        if MEM_TOP < 267 {
            bad = 7;
        }
        if MIN_HALFWORD > 0 {
            bad = 12;
        }
        if MAX_FONT_MAX < MIN_HALFWORD || MAX_FONT_MAX > MAX_HALFWORD {
            bad = 15;
        }
        if font_max > FONT_BASE + 9000 {
            bad = 16;
        }
        if save_size > MAX_HALFWORD || max_strings > MAX_HALFWORD {
            bad = 17;
        }
        if buf_size > MAX_HALFWORD {
            bad = 18;
        }
        if CS_TOKEN_FLAG + EQTB_SIZE + hash_extra > MAX_HALFWORD {
            bad = 21;
        }
        if HASH_OFFSET < 0 || HASH_OFFSET > HASH_BASE {
            bad = 42;
        }
        if 2 * MAX_HALFWORD < MEM_TOP {
            bad = 41;
        }
        if bad > 0 {
            _tt_abort(c"failed internal consistency check #%d".as_ptr(), bad);
        }
        initialize_more_variables();
        if in_initex_mode {
            get_strings_started();
            initialize_more_initex_variables();
            initialize_primitives();
            init_str_ptr = str_ptr;
            init_pool_ptr = pool_ptr;
        }
        initialize_math_variables();
        initialize_pagebuilder_variables();
        initialize_shipout_variables();
        get_seconds_and_micros(&raw mut epochseconds, &raw mut microseconds);
        init_start_time(build_date);
        selector = SELECTOR_TERM_ONLY;
        tally = 0;
        term_offset = 0;
        file_offset = 0;
        job_name = 0;
        name_in_progress = false;
        log_opened = false;
        if semantic_pagination_enabled {
            output_file_extension = c".spx".as_ptr();
        } else {
            output_file_extension = c".xdv".as_ptr();
        }
        input_ptr = 0;
        max_in_stack = 0;
        *source_filename_stack.add(0 as usize) = 0;
        *full_source_filename_stack.add(0 as usize) = 0;
        in_open = 0;
        open_parens = 0;
        max_buf_stack = 0;
        *grp_stack.add(0 as usize) = 0;
        *if_stack.add(0 as usize) = TEX_NULL;
        param_ptr = 0;
        max_param_stack = 0;
        used_tectonic_coda_tokens = false;
        gave_char_warning_help = false;
        std::ptr::write_bytes(buffer, 0, buf_size as usize);
        first = 0;
        scanner_status = NORMAL as u8;
        warning_index = TEX_NULL;
        first = 1;
        cur_input.state = NEW_LINE as u16;
        cur_input.start = 1;
        cur_input.index = 0;
        line = 0;
        cur_input.name = 0;
        force_eof = false;
        align_state = 1000000;
        init_io();
        if in_initex_mode {
            no_new_control_sequence = false;
            max_reg_num = 32767;
            max_reg_help_line = c"A register number must be between 0 and 32767.".as_ptr();
        }
        no_new_control_sequence = true;
        if !in_initex_mode {
            if !tt_xetex_native_load_format() {
                return history;
            }
        }
        if int_par!(INT_PAR__end_line_char) < 0 || int_par!(INT_PAR__end_line_char) > BIGGEST_CHAR {
            cur_input.limit -= 1;
        } else {
            (*buffer.add((cur_input.limit) as usize)) = int_par!(INT_PAR__end_line_char);
        }
        if in_initex_mode {
            int_par!(INT_PAR__time) = 0;
            int_par!(INT_PAR__day) = 0;
            int_par!(INT_PAR__month) = 0;
            int_par!(INT_PAR__year) = 0;
        } else {
            get_date_and_time(
                build_date,
                &raw mut int_par!(INT_PAR__time),
                &raw mut int_par!(INT_PAR__day),
                &raw mut int_par!(INT_PAR__month),
                &raw mut int_par!(INT_PAR__year),
            );
        }
        if trie_not_ready {
            trie_trl = malloc_array::<trie_pointer>(trie_size);
            trie_tro = malloc_array::<trie_pointer>(trie_size);
            trie_trc = malloc_array::<u16>(trie_size);
            trie_c = malloc_array::<packed_UTF16_code>(trie_size);
            trie_o = malloc_array::<trie_opcode>(trie_size);
            trie_l = malloc_array::<trie_pointer>(trie_size);
            trie_r = malloc_array::<trie_pointer>(trie_size);
            trie_hash = malloc_array::<trie_pointer>(trie_size);
            trie_taken = malloc_array::<bool>(trie_size);
            *trie_l.add(0 as usize) = 0;
            *trie_c.add(0 as usize) = 0;
            trie_ptr = 0;
            *trie_r.add(0 as usize) = 0;
            hyph_start = 0;
            font_mapping = calloc_array::<*mut c_void>(font_max);
            font_layout_engine = calloc_array::<*mut c_void>(font_max);
            font_flags = calloc_array::<c_char>(font_max);
            font_letter_space = calloc_array::<scaled_t>(font_max);
            font_check = calloc_array::<b16x4>(font_max);
            font_size = calloc_array::<scaled_t>(font_max);
            font_dsize = calloc_array::<scaled_t>(font_max);
            font_params = calloc_array::<font_index>(font_max);
            font_name = calloc_array::<str_number>(font_max);
            font_area = calloc_array::<str_number>(font_max);
            font_bc = calloc_array::<UTF16_code>(font_max);
            font_ec = calloc_array::<UTF16_code>(font_max);
            font_glue = calloc_array::<i32>(font_max);
            hyphen_char = calloc_array::<i32>(font_max);
            skew_char = calloc_array::<i32>(font_max);
            bchar_label = calloc_array::<font_index>(font_max);
            font_bchar = calloc_array::<nine_bits>(font_max);
            font_false_bchar = calloc_array::<nine_bits>(font_max);
            char_base = calloc_array::<i32>(font_max);
            width_base = calloc_array::<i32>(font_max);
            height_base = calloc_array::<i32>(font_max);
            depth_base = calloc_array::<i32>(font_max);
            italic_base = calloc_array::<i32>(font_max);
            lig_kern_base = calloc_array::<i32>(font_max);
            kern_base = calloc_array::<i32>(font_max);
            exten_base = calloc_array::<i32>(font_max);
            param_base = calloc_array::<i32>(font_max);
            font_ptr = FONT_BASE;
            fmem_ptr = 7;
            *font_name.add(FONT_BASE as usize) = maketexstring(c"nullfont".as_ptr());
            *font_area.add(FONT_BASE as usize) = EMPTY_STRING;
            *hyphen_char.add(FONT_BASE as usize) = b'-' as i32;
            *skew_char.add(FONT_BASE as usize) = -1;
            *bchar_label.add(FONT_BASE as usize) = NON_ADDRESS;
            *font_bchar.add(FONT_BASE as usize) = TOO_BIG_CHAR;
            *font_false_bchar.add(FONT_BASE as usize) = TOO_BIG_CHAR;
            *font_bc.add(FONT_BASE as usize) = 1;
            *font_ec.add(FONT_BASE as usize) = 0;
            *font_size.add(FONT_BASE as usize) = 0;
            *font_dsize.add(FONT_BASE as usize) = 0;
            *char_base.add(FONT_BASE as usize) = 0;
            *width_base.add(FONT_BASE as usize) = 0;
            *height_base.add(FONT_BASE as usize) = 0;
            *depth_base.add(FONT_BASE as usize) = 0;
            *italic_base.add(FONT_BASE as usize) = 0;
            *lig_kern_base.add(FONT_BASE as usize) = 0;
            *kern_base.add(FONT_BASE as usize) = 0;
            *exten_base.add(FONT_BASE as usize) = 0;
            *font_glue.add(FONT_BASE as usize) = TEX_NULL;
            *font_params.add(FONT_BASE as usize) = 7;
            *font_mapping.add(FONT_BASE as usize) = std::ptr::null_mut();
            *param_base.add(FONT_BASE as usize) = -1;
            for font_k in 0..=6 {
                (*font_info.add(font_k as usize)).b32.s1 = 0;
            }
        }
        font_used = malloc_array::<bool>(font_max);
        for font_k in 0..=font_max {
            *font_used.add(font_k as usize) = false;
        }
        random_seed = (microseconds * 1000) + (epochseconds % 1000000);
        init_randoms(random_seed);
        if interaction == BATCH_MODE as u8 {
            selector = SELECTOR_NO_PRINT;
        } else {
            selector = SELECTOR_TERM_ONLY;
        }
        if semantic_pagination_enabled {
            int_par!(INT_PAR__xetex_generate_actual_text) = 1;
        }
        pdf_files_init();
        synctex_init_command();
        start_input(input_file_name);
        history = HISTORY_SPOTLESS;
        main_control();
        final_cleanup();
        close_files_and_terminate();
        tt_cleanup();
        return history;
    }
}
