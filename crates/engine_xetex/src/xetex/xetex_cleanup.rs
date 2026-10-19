// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_xetexd::*;
use libc::{c_int, c_long, c_void};

unsafe extern "C" {
    static mut yhash: *mut b32x2;
    fn pdf_files_close();
    fn destroy_font_manager();
    fn release_font_engine(engine: *mut c_void, type_flag: c_int);
    fn TECkit_DisposeConverter(converter: *mut c_void) -> c_long;
    fn u_close(file: *mut UFILE);
    fn tt_checkpoint_release();
    fn tt_xetex_native_store_format();
}

pub(super) unsafe fn final_cleanup() {
    unsafe {
        let command = cur_chr as small_number;
        if command != 1 {
            (*eqtb.add((INT_BASE + INT_PAR__new_line_char) as usize))
                .b32
                .s1 = -1;
        }
        if job_name == 0 {
            open_log_file();
        }
        while input_ptr > 0 {
            if cur_input.state == TOKEN_LIST as u16 {
                end_token_list();
            } else {
                end_file_reading();
            }
        }
        while open_parens > 0 {
            print_cstr(c" )".as_ptr());
            open_parens -= 1;
        }
        if cur_level > LEVEL_ONE as u16 {
            print_nl(b'(' as i32);
            print_esc_cstr(c"end occurred ".as_ptr());
            print_cstr(c"inside a group at level ".as_ptr());
            print_int(cur_level as i32 - 1);
            print_char(b')' as i32);
            show_save_groups();
        }
        while cond_ptr != TEX_NULL {
            print_nl(b'(' as i32);
            print_esc_cstr(c"end occurred ".as_ptr());
            print_cstr(c"when ".as_ptr());
            print_cmd_chr(IF_TEST as u16, cur_if as i32);
            if if_line != 0 {
                print_cstr(c" on line ".as_ptr());
                print_int(if_line);
            }
            print_cstr(c" was incomplete)".as_ptr());
            if_line = (*mem.add((cond_ptr + 1) as usize)).b32.s1;
            cur_if = (*mem.add(cond_ptr as usize)).b16.s0 as i16;
            temp_ptr = cond_ptr;
            cond_ptr = (*mem.add(cond_ptr as usize)).b32.s1;
            free_node(temp_ptr, IF_NODE_SIZE);
        }
        if history != HISTORY_SPOTLESS
            && (history == HISTORY_WARNING_ISSUED || interaction < ERROR_STOP_MODE as u8)
            && selector == SELECTOR_TERM_AND_LOG
        {
            selector = SELECTOR_TERM_ONLY;
            print_nl_cstr(c"(see the transcript file for additional information)".as_ptr());
            selector = SELECTOR_TERM_AND_LOG;
        }
        if command == 1 {
            if in_initex_mode {
                for index in TOP_MARK_CODE..=SPLIT_BOT_MARK_CODE {
                    let token = cur_mark[index as usize];
                    if token != TEX_NULL {
                        delete_token_ref(token);
                    }
                }
                let root = sa_root[MARK_VAL as usize];
                if root != TEX_NULL && do_marks(3, 0, root) {
                    sa_root[MARK_VAL as usize] = TEX_NULL;
                }
                for index in LAST_BOX_CODE..=VSPLIT_CODE {
                    flush_node_list(disc_ptr[index as usize]);
                }
                if last_glue != MAX_HALFWORD {
                    delete_glue_ref(last_glue);
                }
                tt_xetex_native_store_format();
            } else {
                print_nl_cstr(c"(\\dump is performed only by INITEX)".as_ptr());
            }
        }
    }
}

macro_rules! free_arrays {
    ($($array:ident),+ $(,)?) => {
        $(libc::free($array.cast());)+
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tt_cleanup() {
    unsafe {
        super::xetex_stringpool::reset_string_index();
        pdf_files_close();
        free_arrays!(TEX_format_default, font_used);
        deinitialize_shipout_variables();
        destroy_font_manager();

        for index in 0..font_max as usize {
            let engine = font_layout_engine.add(index);
            if !(*engine).is_null() {
                release_font_engine(*engine, *font_area.add(index));
                engine.write(std::ptr::null_mut());
            }
            let mapping = font_mapping.add(index);
            if !(*mapping).is_null() {
                TECkit_DisposeConverter(*mapping);
                mapping.write(std::ptr::null_mut());
            }
        }
        for index in 1..=in_open as usize {
            let file = *input_file.add(index);
            if !file.is_null() {
                u_close(file);
            }
        }

        free_arrays!(
            buffer,
            nest,
            save_stack,
            input_stack,
            input_file,
            line_stack,
            eof_seen,
            grp_stack,
            if_stack,
            source_filename_stack,
            full_source_filename_stack,
            param_stack,
            hyph_word,
            hyph_list,
            hyph_link,
            native_text,
            yhash,
            eqtb,
            mem,
            str_start,
            str_pool,
            font_info,
            font_mapping,
            font_layout_engine,
            font_flags,
            font_letter_space,
            font_check,
            font_size,
            font_dsize,
            font_params,
            font_name,
            font_area,
            font_bc,
            font_ec,
            font_glue,
            hyphen_char,
            skew_char,
            bchar_label,
            font_bchar,
            font_false_bchar,
            char_base,
            width_base,
            height_base,
            depth_base,
            italic_base,
            lig_kern_base,
            kern_base,
            exten_base,
            param_base,
            trie_trl,
            trie_tro,
            trie_trc,
        );
        trie_trl = std::ptr::null_mut();
        trie_tro = std::ptr::null_mut();
        trie_trc = std::ptr::null_mut();
        tt_checkpoint_release();
    }
}
