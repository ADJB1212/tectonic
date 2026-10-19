// Copyright 2016-2026 The Tectonic Project
// Licensed under the MIT License.

#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use super::xetex_constants::LEVEL_ONE;
use super::xetex_io::UFILE;
use super::xetex_xetexd::{
    UTF16_code, UnicodeScalar, b16x4, b32x2, input_state_t, list_state_record, memory_word,
    str_number,
};

use super::xetex_texmfmp::maketexstring;
use std::ffi::{c_char, c_int, c_void};
use tectonic_bridge_core::FileFormat;

use super::teckit_c_engine::*;
use super::teckit_common::*;
use super::xetex_bindings::*;
use super::xetex_checkpoint::*;
use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_format::*;
use super::xetex_io::*;
use super::xetex_xetexd::*;

/* Global variables originally declared in xetex-xetexd.h and defined in xetex-ini.c */

#[unsafe(export_name = "tectonic_rust_shell_escape_enabled")]
pub static mut shell_escape_enabled: bool = false;
#[unsafe(export_name = "tectonic_rust_eqtb")]
pub static mut eqtb: *mut memory_word = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_bad")]
pub static mut bad: i32 = 0;
#[unsafe(export_name = "tectonic_rust_name_of_file")]
pub static mut name_of_file: *mut libc::c_char = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_name_of_file16")]
pub static mut name_of_file16: *mut UTF16_code = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_name_length")]
pub static mut name_length: i32 = 0;
#[unsafe(export_name = "tectonic_rust_name_length16")]
pub static mut name_length16: i32 = 0;
#[unsafe(export_name = "tectonic_rust_buffer")]
pub static mut buffer: *mut UnicodeScalar = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_first")]
pub static mut first: i32 = 0;
#[unsafe(export_name = "tectonic_rust_last")]
pub static mut last: i32 = 0;
#[unsafe(export_name = "tectonic_rust_max_buf_stack")]
pub static mut max_buf_stack: i32 = 0;
#[unsafe(export_name = "tectonic_rust_in_initex_mode")]
pub static mut in_initex_mode: bool = false;
#[unsafe(export_name = "tectonic_rust_error_line")]
pub static mut error_line: i32 = 0;
#[unsafe(export_name = "tectonic_rust_half_error_line")]
pub static mut half_error_line: i32 = 0;
#[unsafe(export_name = "tectonic_rust_max_print_line")]
pub static mut max_print_line: i32 = 0;
#[unsafe(export_name = "tectonic_rust_max_strings")]
pub static mut max_strings: i32 = 0;
#[unsafe(export_name = "tectonic_rust_strings_free")]
pub static mut strings_free: i32 = 0;
#[unsafe(export_name = "tectonic_rust_string_vacancies")]
pub static mut string_vacancies: i32 = 0;
#[unsafe(export_name = "tectonic_rust_pool_size")]
pub static mut pool_size: i32 = 0;
#[unsafe(export_name = "tectonic_rust_pool_free")]
pub static mut pool_free: i32 = 0;
#[unsafe(export_name = "tectonic_rust_font_mem_size")]
pub static mut font_mem_size: i32 = 0;
#[unsafe(export_name = "tectonic_rust_font_max")]
pub static mut font_max: i32 = 0;
#[unsafe(export_name = "tectonic_rust_hyph_size")]
pub static mut hyph_size: i32 = 0;
#[unsafe(export_name = "tectonic_rust_trie_size")]
pub static mut trie_size: i32 = 0;
#[unsafe(export_name = "tectonic_rust_buf_size")]
pub static mut buf_size: i32 = 0;
#[unsafe(export_name = "tectonic_rust_stack_size")]
pub static mut stack_size: i32 = 0;
#[unsafe(export_name = "tectonic_rust_max_in_open")]
pub static mut max_in_open: i32 = 0;
#[unsafe(export_name = "tectonic_rust_param_size")]
pub static mut param_size: i32 = 0;
#[unsafe(export_name = "tectonic_rust_nest_size")]
pub static mut nest_size: i32 = 0;
#[unsafe(export_name = "tectonic_rust_save_size")]
pub static mut save_size: i32 = 0;
#[unsafe(export_name = "tectonic_rust_expand_depth")]
pub static mut expand_depth: i32 = 0;
#[unsafe(export_name = "tectonic_rust_file_line_error_style_p")]
pub static mut file_line_error_style_p: libc::c_int = 0;
#[unsafe(export_name = "tectonic_rust_halt_on_error_p")]
pub static mut halt_on_error_p: libc::c_int = 0;
#[unsafe(export_name = "tectonic_rust_quoted_filename")]
pub static mut quoted_filename: bool = false;
#[unsafe(export_name = "tectonic_rust_insert_src_special_auto")]
pub static mut insert_src_special_auto: bool = false;
#[unsafe(export_name = "tectonic_rust_insert_src_special_every_par")]
pub static mut insert_src_special_every_par: bool = false;
#[unsafe(export_name = "tectonic_rust_insert_src_special_every_math")]
pub static mut insert_src_special_every_math: bool = false;
#[unsafe(export_name = "tectonic_rust_insert_src_special_every_vbox")]
pub static mut insert_src_special_every_vbox: bool = false;
#[unsafe(export_name = "tectonic_rust_str_pool")]
pub static mut str_pool: *mut packed_UTF16_code = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_str_start")]
pub static mut str_start: *mut pool_pointer = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_pool_ptr")]
pub static mut pool_ptr: pool_pointer = 0;
#[unsafe(export_name = "tectonic_rust_str_ptr")]
pub static mut str_ptr: str_number = 0;
#[unsafe(export_name = "tectonic_rust_init_pool_ptr")]
pub static mut init_pool_ptr: pool_pointer = 0;
#[unsafe(export_name = "tectonic_rust_init_str_ptr")]
pub static mut init_str_ptr: str_number = 0;
#[unsafe(export_name = "tectonic_rust_rust_stdout")]
pub static mut rust_stdout: rust_output_handle_t = 0;
#[unsafe(export_name = "tectonic_rust_log_file")]
pub static mut log_file: rust_output_handle_t = 0;
#[unsafe(export_name = "tectonic_rust_selector")]
pub static mut selector: selector_t = SELECTOR_TERM_ONLY;
#[unsafe(export_name = "tectonic_rust_dig")]
pub static mut dig: [u8; 23] = [0; 23];
#[unsafe(export_name = "tectonic_rust_tally")]
pub static mut tally: i32 = 0;
#[unsafe(export_name = "tectonic_rust_term_offset")]
pub static mut term_offset: i32 = 0;
#[unsafe(export_name = "tectonic_rust_file_offset")]
pub static mut file_offset: i32 = 0;
#[unsafe(export_name = "tectonic_rust_trick_buf")]
pub static mut trick_buf: [UTF16_code; 256] = [0; 256];
#[unsafe(export_name = "tectonic_rust_trick_count")]
pub static mut trick_count: i32 = 0;
#[unsafe(export_name = "tectonic_rust_first_count")]
pub static mut first_count: i32 = 0;
#[unsafe(export_name = "tectonic_rust_doing_special")]
pub static mut doing_special: bool = false;
#[unsafe(export_name = "tectonic_rust_native_text")]
pub static mut native_text: *mut UTF16_code = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_native_text_size")]
pub static mut native_text_size: i32 = 0;
#[unsafe(export_name = "tectonic_rust_native_len")]
pub static mut native_len: i32 = 0;
#[unsafe(export_name = "tectonic_rust_save_native_len")]
pub static mut save_native_len: i32 = 0;
#[unsafe(export_name = "tectonic_rust_interaction")]
pub static mut interaction: u8 = 0;
#[unsafe(export_name = "tectonic_rust_deletions_allowed")]
pub static mut deletions_allowed: bool = false;
#[unsafe(export_name = "tectonic_rust_set_box_allowed")]
pub static mut set_box_allowed: bool = false;
#[unsafe(export_name = "tectonic_rust_history")]
pub static mut history: tt_history_t = HISTORY_SPOTLESS;
#[unsafe(export_name = "tectonic_rust_error_count")]
pub static mut error_count: i8 = 0;
#[unsafe(export_name = "tectonic_rust_help_line")]
pub static mut help_line: [*const libc::c_char; 6] = [std::ptr::null(); 6];
#[unsafe(export_name = "tectonic_rust_help_ptr")]
pub static mut help_ptr: u8 = 0;
#[unsafe(export_name = "tectonic_rust_use_err_help")]
pub static mut use_err_help: bool = false;
#[unsafe(export_name = "tectonic_rust_arith_error")]
pub static mut arith_error: bool = false;
#[unsafe(export_name = "tectonic_rust_tex_remainder")]
pub static mut tex_remainder: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_randoms")]
pub static mut randoms: [i32; 55] = [0; 55];
#[unsafe(export_name = "tectonic_rust_j_random")]
pub static mut j_random: u8 = 0;
#[unsafe(export_name = "tectonic_rust_random_seed")]
pub static mut random_seed: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_two_to_the")]
pub static mut two_to_the: [i32; 31] = [0; 31];
#[unsafe(export_name = "tectonic_rust_spec_log")]
pub static mut spec_log: [i32; 29] = [0; 29];
#[unsafe(export_name = "tectonic_rust_temp_ptr")]
pub static mut temp_ptr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_mem")]
pub static mut mem: *mut memory_word = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_lo_mem_max")]
pub static mut lo_mem_max: i32 = 0;
#[unsafe(export_name = "tectonic_rust_hi_mem_min")]
pub static mut hi_mem_min: i32 = 0;
#[unsafe(export_name = "tectonic_rust_var_used")]
pub static mut var_used: i32 = 0;
#[unsafe(export_name = "tectonic_rust_dyn_used")]
pub static mut dyn_used: i32 = 0;
#[unsafe(export_name = "tectonic_rust_avail")]
pub static mut avail: i32 = 0;
#[unsafe(export_name = "tectonic_rust_mem_end")]
pub static mut mem_end: i32 = 0;
#[unsafe(export_name = "tectonic_rust_rover")]
pub static mut rover: i32 = 0;
#[unsafe(export_name = "tectonic_rust_last_leftmost_char")]
pub static mut last_leftmost_char: i32 = 0;
#[unsafe(export_name = "tectonic_rust_last_rightmost_char")]
pub static mut last_rightmost_char: i32 = 0;
#[unsafe(export_name = "tectonic_rust_hlist_stack")]
pub static mut hlist_stack: [i32; (MAX_HLIST_STACK + 1) as usize] =
    [0; (MAX_HLIST_STACK + 1) as usize];
#[unsafe(export_name = "tectonic_rust_hlist_stack_level")]
pub static mut hlist_stack_level: i16 = 0;
#[unsafe(export_name = "tectonic_rust_first_p")]
pub static mut first_p: i32 = 0;
#[unsafe(export_name = "tectonic_rust_global_prev_p")]
pub static mut global_prev_p: i32 = 0;
#[unsafe(export_name = "tectonic_rust_font_in_short_display")]
pub static mut font_in_short_display: i32 = 0;
#[unsafe(export_name = "tectonic_rust_depth_threshold")]
pub static mut depth_threshold: i32 = 0;
#[unsafe(export_name = "tectonic_rust_breadth_max")]
pub static mut breadth_max: i32 = 0;
#[unsafe(export_name = "tectonic_rust_nest")]
pub static mut nest: *mut list_state_record = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_nest_ptr")]
pub static mut nest_ptr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_max_nest_stack")]
pub static mut max_nest_stack: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_list")]
pub static mut cur_list: list_state_record = list_state_record {
    mode: 0,
    head: 0,
    tail: 0,
    eTeX_aux: 0,
    prev_graf: 0,
    mode_line: 0,
    aux: memory_word {
        b32: b32x2 { s0: 0, s1: 0 },
    },
};
#[unsafe(export_name = "tectonic_rust_shown_mode")]
pub static mut shown_mode: i16 = 0;
#[unsafe(export_name = "tectonic_rust_old_setting")]
pub static mut old_setting: u8 = 0;
#[unsafe(export_name = "tectonic_rust_hash")]
pub static mut hash: *mut b32x2 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_hash_used")]
pub static mut hash_used: i32 = 0;
#[unsafe(export_name = "tectonic_rust_hash_extra")]
pub static mut hash_extra: i32 = 0;
#[unsafe(export_name = "tectonic_rust_hash_top")]
pub static mut hash_top: i32 = 0;
#[unsafe(export_name = "tectonic_rust_eqtb_top")]
pub static mut eqtb_top: i32 = 0;
#[unsafe(export_name = "tectonic_rust_hash_high")]
pub static mut hash_high: i32 = 0;
#[unsafe(export_name = "tectonic_rust_no_new_control_sequence")]
pub static mut no_new_control_sequence: bool = false;
#[unsafe(export_name = "tectonic_rust_cs_count")]
pub static mut cs_count: i32 = 0;
#[unsafe(export_name = "tectonic_rust_prim")]
pub static mut prim: [b32x2; (PRIM_SIZE + 1) as usize] =
    [b32x2 { s0: 0, s1: 0 }; (PRIM_SIZE + 1) as usize];
#[unsafe(export_name = "tectonic_rust_prim_used")]
pub static mut prim_used: i32 = 0;
#[unsafe(export_name = "tectonic_rust_save_stack")]
pub static mut save_stack: *mut memory_word = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_save_ptr")]
pub static mut save_ptr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_max_save_stack")]
pub static mut max_save_stack: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_level")]
pub static mut cur_level: u16 = 0;
#[unsafe(export_name = "tectonic_rust_cur_group")]
pub static mut cur_group: group_code = 0;
#[unsafe(export_name = "tectonic_rust_cur_boundary")]
pub static mut cur_boundary: i32 = 0;
#[unsafe(export_name = "tectonic_rust_mag_set")]
pub static mut mag_set: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_cmd")]
pub static mut cur_cmd: eight_bits = 0;
#[unsafe(export_name = "tectonic_rust_cur_chr")]
pub static mut cur_chr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_cs")]
pub static mut cur_cs: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_tok")]
pub static mut cur_tok: i32 = 0;
#[unsafe(export_name = "tectonic_rust_input_stack")]
pub static mut input_stack: *mut input_state_t = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_input_ptr")]
pub static mut input_ptr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_max_in_stack")]
pub static mut max_in_stack: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_input")]
pub static mut cur_input: input_state_t = input_state_t {
    state: 0,
    index: 0,
    start: 0,
    loc: 0,
    limit: 0,
    name: 0,
    synctex_tag: 0,
};
#[unsafe(export_name = "tectonic_rust_in_open")]
pub static mut in_open: i32 = 0;
#[unsafe(export_name = "tectonic_rust_open_parens")]
pub static mut open_parens: i32 = 0;
#[unsafe(export_name = "tectonic_rust_input_file")]
pub static mut input_file: *mut *mut UFILE = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_line")]
pub static mut line: i32 = 0;
#[unsafe(export_name = "tectonic_rust_line_stack")]
pub static mut line_stack: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_source_filename_stack")]
pub static mut source_filename_stack: *mut str_number = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_full_source_filename_stack")]
pub static mut full_source_filename_stack: *mut str_number = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_scanner_status")]
pub static mut scanner_status: u8 = 0;
#[unsafe(export_name = "tectonic_rust_warning_index")]
pub static mut warning_index: i32 = 0;
#[unsafe(export_name = "tectonic_rust_def_ref")]
pub static mut def_ref: i32 = 0;
#[unsafe(export_name = "tectonic_rust_param_stack")]
pub static mut param_stack: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_param_ptr")]
pub static mut param_ptr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_max_param_stack")]
pub static mut max_param_stack: i32 = 0;
#[unsafe(export_name = "tectonic_rust_align_state")]
pub static mut align_state: i32 = 0;
#[unsafe(export_name = "tectonic_rust_base_ptr")]
pub static mut base_ptr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_par_loc")]
pub static mut par_loc: i32 = 0;
#[unsafe(export_name = "tectonic_rust_par_token")]
pub static mut par_token: i32 = 0;
#[unsafe(export_name = "tectonic_rust_force_eof")]
pub static mut force_eof: bool = false;
#[unsafe(export_name = "tectonic_rust_expand_depth_count")]
pub static mut expand_depth_count: i32 = 0;
#[unsafe(export_name = "tectonic_rust_is_in_csname")]
pub static mut is_in_csname: bool = false;
#[unsafe(export_name = "tectonic_rust_cur_mark")]
pub static mut cur_mark: [i32; 5] = [0; 5];
#[unsafe(export_name = "tectonic_rust_long_state")]
pub static mut long_state: u8 = 0;
#[unsafe(export_name = "tectonic_rust_pstack")]
pub static mut pstack: [i32; 9] = [0; 9];
#[unsafe(export_name = "tectonic_rust_cur_val")]
pub static mut cur_val: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_val1")]
pub static mut cur_val1: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_val_level")]
pub static mut cur_val_level: u8 = 0;
#[unsafe(export_name = "tectonic_rust_radix")]
pub static mut radix: small_number = 0;
#[unsafe(export_name = "tectonic_rust_cur_order")]
pub static mut cur_order: glue_ord = 0;
#[unsafe(export_name = "tectonic_rust_read_file")]
pub static mut read_file: [*mut UFILE; 16] = [std::ptr::null_mut(); 16];
#[unsafe(export_name = "tectonic_rust_read_open")]
pub static mut read_open: [u8; 17] = [0; 17];
#[unsafe(export_name = "tectonic_rust_cond_ptr")]
pub static mut cond_ptr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_if_limit")]
pub static mut if_limit: u8 = 0;
#[unsafe(export_name = "tectonic_rust_cur_if")]
pub static mut cur_if: small_number = 0;
#[unsafe(export_name = "tectonic_rust_if_line")]
pub static mut if_line: i32 = 0;
#[unsafe(export_name = "tectonic_rust_skip_line")]
pub static mut skip_line: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_name")]
pub static mut cur_name: str_number = 0;
#[unsafe(export_name = "tectonic_rust_cur_area")]
pub static mut cur_area: str_number = 0;
#[unsafe(export_name = "tectonic_rust_cur_ext")]
pub static mut cur_ext: str_number = 0;
#[unsafe(export_name = "tectonic_rust_area_delimiter")]
pub static mut area_delimiter: pool_pointer = 0;
#[unsafe(export_name = "tectonic_rust_ext_delimiter")]
pub static mut ext_delimiter: pool_pointer = 0;
#[unsafe(export_name = "tectonic_rust_file_name_quote_char")]
pub static mut file_name_quote_char: UTF16_code = 0;
#[unsafe(export_name = "tectonic_rust_format_default_length")]
pub static mut format_default_length: i32 = 0;
#[unsafe(export_name = "tectonic_rust_TEX_format_default")]
pub static mut TEX_format_default: *mut libc::c_char = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_name_in_progress")]
pub static mut name_in_progress: bool = false;
#[unsafe(export_name = "tectonic_rust_job_name")]
pub static mut job_name: str_number = 0;
#[unsafe(export_name = "tectonic_rust_log_opened")]
pub static mut log_opened: bool = false;
#[unsafe(export_name = "tectonic_rust_output_file_extension")]
pub static mut output_file_extension: *const libc::c_char = std::ptr::null();
#[unsafe(export_name = "tectonic_rust_texmf_log_name")]
pub static mut texmf_log_name: str_number = 0;
#[unsafe(export_name = "tectonic_rust_font_info")]
pub static mut font_info: *mut memory_word = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_fmem_ptr")]
pub static mut fmem_ptr: font_index = 0;
#[unsafe(export_name = "tectonic_rust_font_ptr")]
pub static mut font_ptr: internal_font_number = 0;
#[unsafe(export_name = "tectonic_rust_font_check")]
pub static mut font_check: *mut b16x4 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_size")]
pub static mut font_size: *mut scaled_t = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_dsize")]
pub static mut font_dsize: *mut scaled_t = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_params")]
pub static mut font_params: *mut font_index = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_name")]
pub static mut font_name: *mut str_number = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_area")]
pub static mut font_area: *mut str_number = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_bc")]
pub static mut font_bc: *mut UTF16_code = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_ec")]
pub static mut font_ec: *mut UTF16_code = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_glue")]
pub static mut font_glue: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_used")]
pub static mut font_used: *mut bool = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_hyphen_char")]
pub static mut hyphen_char: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_skew_char")]
pub static mut skew_char: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_bchar_label")]
pub static mut bchar_label: *mut font_index = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_bchar")]
pub static mut font_bchar: *mut nine_bits = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_false_bchar")]
pub static mut font_false_bchar: *mut nine_bits = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_layout_engine")]
pub static mut font_layout_engine: *mut *mut libc::c_void = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_mapping")]
pub static mut font_mapping: *mut *mut libc::c_void = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_flags")]
pub static mut font_flags: *mut libc::c_char = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_font_letter_space")]
pub static mut font_letter_space: *mut scaled_t = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_loaded_font_mapping")]
pub static mut loaded_font_mapping: *mut libc::c_void = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_loaded_font_flags")]
pub static mut loaded_font_flags: libc::c_char = 0;
#[unsafe(export_name = "tectonic_rust_loaded_font_letter_space")]
pub static mut loaded_font_letter_space: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_mapped_text")]
pub static mut mapped_text: *mut UTF16_code = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_xdv_buffer")]
pub static mut xdv_buffer: *mut libc::c_char = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_char_base")]
pub static mut char_base: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_width_base")]
pub static mut width_base: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_height_base")]
pub static mut height_base: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_depth_base")]
pub static mut depth_base: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_italic_base")]
pub static mut italic_base: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_lig_kern_base")]
pub static mut lig_kern_base: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_kern_base")]
pub static mut kern_base: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_exten_base")]
pub static mut exten_base: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_param_base")]
pub static mut param_base: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_null_character")]
pub static mut null_character: b16x4 = b16x4 {
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
};
#[unsafe(export_name = "tectonic_rust_total_pages")]
pub static mut total_pages: i32 = 0;
#[unsafe(export_name = "tectonic_rust_max_v")]
pub static mut max_v: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_max_h")]
pub static mut max_h: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_max_push")]
pub static mut max_push: i32 = 0;
#[unsafe(export_name = "tectonic_rust_last_bop")]
pub static mut last_bop: i32 = 0;
#[unsafe(export_name = "tectonic_rust_dead_cycles")]
pub static mut dead_cycles: i32 = 0;
#[unsafe(export_name = "tectonic_rust_doing_leaders")]
pub static mut doing_leaders: bool = false;
#[unsafe(export_name = "tectonic_rust_rule_ht")]
pub static mut rule_ht: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_rule_dp")]
pub static mut rule_dp: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_rule_wd")]
pub static mut rule_wd: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_cur_h")]
pub static mut cur_h: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_cur_v")]
pub static mut cur_v: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_epochseconds")]
pub static mut epochseconds: i32 = 0;
#[unsafe(export_name = "tectonic_rust_microseconds")]
pub static mut microseconds: i32 = 0;
#[unsafe(export_name = "tectonic_rust_total_stretch")]
pub static mut total_stretch: [scaled_t; 4] = [0; 4];
#[unsafe(export_name = "tectonic_rust_total_shrink")]
pub static mut total_shrink: [scaled_t; 4] = [0; 4];
#[unsafe(export_name = "tectonic_rust_last_badness")]
pub static mut last_badness: i32 = 0;
#[unsafe(export_name = "tectonic_rust_adjust_tail")]
pub static mut adjust_tail: i32 = 0;
#[unsafe(export_name = "tectonic_rust_pre_adjust_tail")]
pub static mut pre_adjust_tail: i32 = 0;
#[unsafe(export_name = "tectonic_rust_pack_begin_line")]
pub static mut pack_begin_line: i32 = 0;
#[unsafe(export_name = "tectonic_rust_empty")]
pub static mut empty: b32x2 = b32x2 { s0: 0, s1: 0 };
#[unsafe(export_name = "tectonic_rust_cur_f")]
pub static mut cur_f: internal_font_number = 0;
#[unsafe(export_name = "tectonic_rust_cur_c")]
pub static mut cur_c: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_i")]
pub static mut cur_i: b16x4 = b16x4 {
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
};
#[unsafe(export_name = "tectonic_rust_cur_align")]
pub static mut cur_align: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_span")]
pub static mut cur_span: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_loop")]
pub static mut cur_loop: i32 = 0;
#[unsafe(export_name = "tectonic_rust_align_ptr")]
pub static mut align_ptr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_head")]
pub static mut cur_head: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_tail")]
pub static mut cur_tail: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_pre_head")]
pub static mut cur_pre_head: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_pre_tail")]
pub static mut cur_pre_tail: i32 = 0;
#[unsafe(export_name = "tectonic_rust_just_box")]
pub static mut just_box: i32 = 0;
#[unsafe(export_name = "tectonic_rust_active_width")]
pub static mut active_width: [scaled_t; 7] = [0; 7];
#[unsafe(export_name = "tectonic_rust_hc")]
pub static mut hc: [i32; 4099] = [0; 4099];
#[unsafe(export_name = "tectonic_rust_hf")]
pub static mut hf: internal_font_number = 0;
#[unsafe(export_name = "tectonic_rust_hu")]
pub static mut hu: [i32; 4097] = [0; 4097];
#[unsafe(export_name = "tectonic_rust_cur_lang")]
pub static mut cur_lang: u8 = 0;
#[unsafe(export_name = "tectonic_rust_max_hyph_char")]
pub static mut max_hyph_char: i32 = 0;
#[unsafe(export_name = "tectonic_rust_hyf")]
pub static mut hyf: [u8; 4097] = [0; 4097];
#[unsafe(export_name = "tectonic_rust_init_list")]
pub static mut init_list: i32 = 0;
#[unsafe(export_name = "tectonic_rust_init_lig")]
pub static mut init_lig: bool = false;
#[unsafe(export_name = "tectonic_rust_init_lft")]
pub static mut init_lft: bool = false;
#[unsafe(export_name = "tectonic_rust_hyphen_passed")]
pub static mut hyphen_passed: small_number = 0;
#[unsafe(export_name = "tectonic_rust_cur_l")]
pub static mut cur_l: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_r")]
pub static mut cur_r: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_q")]
pub static mut cur_q: i32 = 0;
#[unsafe(export_name = "tectonic_rust_lig_stack")]
pub static mut lig_stack: i32 = 0;
#[unsafe(export_name = "tectonic_rust_ligature_present")]
pub static mut ligature_present: bool = false;
#[unsafe(export_name = "tectonic_rust_lft_hit")]
pub static mut lft_hit: bool = false;
#[unsafe(export_name = "tectonic_rust_rt_hit")]
pub static mut rt_hit: bool = false;
#[unsafe(export_name = "tectonic_rust_trie_trl")]
pub static mut trie_trl: *mut trie_pointer = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_trie_tro")]
pub static mut trie_tro: *mut trie_pointer = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_trie_trc")]
pub static mut trie_trc: *mut u16 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_hyf_distance")]
pub static mut hyf_distance: [small_number; (TRIE_OP_SIZE + 1) as usize] =
    [0; (TRIE_OP_SIZE + 1) as usize];
#[unsafe(export_name = "tectonic_rust_hyf_num")]
pub static mut hyf_num: [small_number; (TRIE_OP_SIZE + 1) as usize] =
    [0; (TRIE_OP_SIZE + 1) as usize];
#[unsafe(export_name = "tectonic_rust_hyf_next")]
pub static mut hyf_next: [trie_opcode; (TRIE_OP_SIZE + 1) as usize] =
    [0; (TRIE_OP_SIZE + 1) as usize];
#[unsafe(export_name = "tectonic_rust_op_start")]
pub static mut op_start: [i32; 256] = [0; 256];
#[unsafe(export_name = "tectonic_rust_hyph_word")]
pub static mut hyph_word: *mut str_number = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_hyph_list")]
pub static mut hyph_list: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_hyph_link")]
pub static mut hyph_link: *mut hyph_pointer = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_hyph_count")]
pub static mut hyph_count: i32 = 0;
#[unsafe(export_name = "tectonic_rust_hyph_next")]
pub static mut hyph_next: i32 = 0;
#[unsafe(export_name = "tectonic_rust_trie_used")]
pub static mut trie_used: [trie_opcode; 256] = [0; 256];
#[unsafe(export_name = "tectonic_rust_trie_op_lang")]
pub static mut trie_op_lang: [u8; (TRIE_OP_SIZE + 1) as usize] = [0; (TRIE_OP_SIZE + 1) as usize];
#[unsafe(export_name = "tectonic_rust_trie_op_val")]
pub static mut trie_op_val: [trie_opcode; (TRIE_OP_SIZE + 1) as usize] =
    [0; (TRIE_OP_SIZE + 1) as usize];
#[unsafe(export_name = "tectonic_rust_trie_op_ptr")]
pub static mut trie_op_ptr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_max_op_used")]
pub static mut max_op_used: trie_opcode = 0;
#[unsafe(export_name = "tectonic_rust_trie_c")]
pub static mut trie_c: *mut packed_UTF16_code = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_trie_o")]
pub static mut trie_o: *mut trie_opcode = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_trie_l")]
pub static mut trie_l: *mut trie_pointer = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_trie_r")]
pub static mut trie_r: *mut trie_pointer = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_trie_ptr")]
pub static mut trie_ptr: trie_pointer = 0;
#[unsafe(export_name = "tectonic_rust_trie_hash")]
pub static mut trie_hash: *mut trie_pointer = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_trie_taken")]
pub static mut trie_taken: *mut bool = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_trie_min")]
pub static mut trie_min: [trie_pointer; 65536] = [0; 65536];
#[unsafe(export_name = "tectonic_rust_trie_max")]
pub static mut trie_max: trie_pointer = 0;
#[unsafe(export_name = "tectonic_rust_trie_not_ready")]
pub static mut trie_not_ready: bool = false;
#[unsafe(export_name = "tectonic_rust_best_height_plus_depth")]
pub static mut best_height_plus_depth: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_main_f")]
pub static mut main_f: internal_font_number = 0;
#[unsafe(export_name = "tectonic_rust_main_i")]
pub static mut main_i: b16x4 = b16x4 {
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
};
#[unsafe(export_name = "tectonic_rust_main_j")]
pub static mut main_j: b16x4 = b16x4 {
    s0: 0,
    s1: 0,
    s2: 0,
    s3: 0,
};
#[unsafe(export_name = "tectonic_rust_main_k")]
pub static mut main_k: font_index = 0;
#[unsafe(export_name = "tectonic_rust_main_p")]
pub static mut main_p: i32 = 0;
#[unsafe(export_name = "tectonic_rust_main_pp")]
pub static mut main_pp: i32 = 0;
#[unsafe(export_name = "tectonic_rust_main_ppp")]
pub static mut main_ppp: i32 = 0;
#[unsafe(export_name = "tectonic_rust_main_h")]
pub static mut main_h: i32 = 0;
#[unsafe(export_name = "tectonic_rust_is_hyph")]
pub static mut is_hyph: bool = false;
#[unsafe(export_name = "tectonic_rust_space_class")]
pub static mut space_class: i32 = 0;
#[unsafe(export_name = "tectonic_rust_prev_class")]
pub static mut prev_class: i32 = 0;
#[unsafe(export_name = "tectonic_rust_main_s")]
pub static mut main_s: i32 = 0;
#[unsafe(export_name = "tectonic_rust_bchar")]
pub static mut bchar: i32 = 0;
#[unsafe(export_name = "tectonic_rust_false_bchar")]
pub static mut false_bchar: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cancel_boundary")]
pub static mut cancel_boundary: bool = false;
#[unsafe(export_name = "tectonic_rust_ins_disc")]
pub static mut ins_disc: bool = false;
#[unsafe(export_name = "tectonic_rust_cur_box")]
pub static mut cur_box: i32 = 0;
#[unsafe(export_name = "tectonic_rust_after_token")]
pub static mut after_token: i32 = 0;
#[unsafe(export_name = "tectonic_rust_long_help_seen")]
pub static mut long_help_seen: bool = false;
#[unsafe(export_name = "tectonic_rust_format_ident")]
pub static mut format_ident: str_number = 0;
#[unsafe(export_name = "tectonic_rust_write_file")]
pub static mut write_file: [rust_output_handle_t; 16] = [0; 16];
#[unsafe(export_name = "tectonic_rust_write_open")]
pub static mut write_open: [bool; 18] = [false; 18];
#[unsafe(export_name = "tectonic_rust_write_loc")]
pub static mut write_loc: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_page_width")]
pub static mut cur_page_width: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_cur_page_height")]
pub static mut cur_page_height: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_cur_h_offset")]
pub static mut cur_h_offset: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_cur_v_offset")]
pub static mut cur_v_offset: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_pdf_last_x_pos")]
pub static mut pdf_last_x_pos: i32 = 0;
#[unsafe(export_name = "tectonic_rust_pdf_last_y_pos")]
pub static mut pdf_last_y_pos: i32 = 0;
#[unsafe(export_name = "tectonic_rust_eof_seen")]
pub static mut eof_seen: *mut bool = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_LR_ptr")]
pub static mut LR_ptr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_LR_problems")]
pub static mut LR_problems: i32 = 0;
#[unsafe(export_name = "tectonic_rust_cur_dir")]
pub static mut cur_dir: small_number = 0;
#[unsafe(export_name = "tectonic_rust_pseudo_files")]
pub static mut pseudo_files: i32 = 0;
#[unsafe(export_name = "tectonic_rust_grp_stack")]
pub static mut grp_stack: *mut save_pointer = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_if_stack")]
pub static mut if_stack: *mut i32 = std::ptr::null_mut();
#[unsafe(export_name = "tectonic_rust_max_reg_num")]
pub static mut max_reg_num: i32 = 0;
#[unsafe(export_name = "tectonic_rust_max_reg_help_line")]
pub static mut max_reg_help_line: *const libc::c_char = std::ptr::null();
#[unsafe(export_name = "tectonic_rust_sa_root")]
pub static mut sa_root: [i32; 8] = [0; 8];
#[unsafe(export_name = "tectonic_rust_cur_ptr")]
pub static mut cur_ptr: i32 = 0;
#[unsafe(export_name = "tectonic_rust_sa_null")]
pub static mut sa_null: memory_word = memory_word {
    b32: b32x2 { s0: 0, s1: 0 },
};
#[unsafe(export_name = "tectonic_rust_sa_chain")]
pub static mut sa_chain: i32 = 0;
#[unsafe(export_name = "tectonic_rust_sa_level")]
pub static mut sa_level: u16 = 0;
#[unsafe(export_name = "tectonic_rust_hyph_start")]
pub static mut hyph_start: trie_pointer = 0;
#[unsafe(export_name = "tectonic_rust_hyph_index")]
pub static mut hyph_index: trie_pointer = 0;
#[unsafe(export_name = "tectonic_rust_disc_ptr")]
pub static mut disc_ptr: [i32; 4] = [0; 4];
#[unsafe(export_name = "tectonic_rust_edit_name_start")]
pub static mut edit_name_start: pool_pointer = 0;
#[unsafe(export_name = "tectonic_rust_stop_at_space")]
pub static mut stop_at_space: bool = false;
#[unsafe(export_name = "tectonic_rust_native_font_type_flag")]
pub static mut native_font_type_flag: i32 = 0;
#[unsafe(export_name = "tectonic_rust_xtx_ligature_present")]
pub static mut xtx_ligature_present: bool = false;
#[unsafe(export_name = "tectonic_rust_delta")]
pub static mut delta: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_synctex_enabled")]
pub static mut synctex_enabled: libc::c_int = 0;
#[unsafe(export_name = "tectonic_rust_used_tectonic_coda_tokens")]
pub static mut used_tectonic_coda_tokens: bool = false;
#[unsafe(export_name = "tectonic_rust_semantic_pagination_enabled")]
pub static mut semantic_pagination_enabled: bool = false;
#[unsafe(export_name = "tectonic_rust_gave_char_warning_help")]
pub static mut gave_char_warning_help: bool = false;

/* Pagebuilder globals */
#[unsafe(export_name = "tectonic_rust_page_tail")]
pub static mut page_tail: i32 = 0;
#[unsafe(export_name = "tectonic_rust_page_contents")]
pub static mut page_contents: u8 = 0;
#[unsafe(export_name = "tectonic_rust_page_so_far")]
pub static mut page_so_far: [scaled_t; 8] = [0; 8];
#[unsafe(export_name = "tectonic_rust_last_glue")]
pub static mut last_glue: i32 = 0;
#[unsafe(export_name = "tectonic_rust_last_penalty")]
pub static mut last_penalty: i32 = 0;
#[unsafe(export_name = "tectonic_rust_last_kern")]
pub static mut last_kern: scaled_t = 0;
#[unsafe(export_name = "tectonic_rust_last_node_type")]
pub static mut last_node_type: i32 = 0;
#[unsafe(export_name = "tectonic_rust_insert_penalties")]
pub static mut insert_penalties: i32 = 0;
#[unsafe(export_name = "tectonic_rust_output_active")]
pub static mut output_active: bool = false;

#[unsafe(export_name = "tectonic_rust__xeq_level_array")]
pub static mut _xeq_level_array: [u16; (EQTB_SIZE - INT_BASE + 1) as usize] =
    [0; (EQTB_SIZE - INT_BASE + 1) as usize];

pub const NEG_TRIE_OP_SIZE: i32 = -35111;
pub const MAX_TRIE_OP: i32 = 65535;

static mut _trie_op_hash_array: [i32; (TRIE_OP_SIZE - NEG_TRIE_OP_SIZE + 1) as usize] =
    [0; (TRIE_OP_SIZE - NEG_TRIE_OP_SIZE + 1) as usize];

#[inline]
pub unsafe fn TRIE_OP_HASH(i: i32) -> &'static mut i32 {
    unsafe { &mut _trie_op_hash_array[(i - NEG_TRIE_OP_SIZE) as usize] }
}

static mut yhash: *mut b32x2 = std::ptr::null_mut();

pub const FORMAT_HEADER_MAGIC: u32 = 0x54544E43; // "TTNC"
pub const FORMAT_FOOTER_MAGIC: u32 = 0x0000029A;

pub const hash_offset: i32 = 514;
pub const sup_max_strings: i32 = 2097151;
pub const sup_font_mem_size: i32 = 147483647;
pub const sup_pool_size: i32 = 40000000;
pub const sup_string_vacancies: i32 = sup_pool_size - 23000;
pub const sup_hash_extra: i32 = sup_max_strings;
pub const inf_expand_depth: i32 = 10;
pub const sup_expand_depth: i32 = 10000000;

pub unsafe fn sort_avail() {
    unsafe {
        let mut p: i32;
        let mut q: i32;
        let mut r: i32;
        let old_rover: i32;

        p = get_node(0x40000000);
        p = (*mem.add((rover + 1) as usize)).b32.s1;
        (*mem.add((rover + 1) as usize)).b32.s1 = MAX_HALFWORD;
        old_rover = rover;

        while p != old_rover {
            if p < rover {
                q = p;
                p = (*mem.add((q + 1) as usize)).b32.s1;
                (*mem.add((q + 1) as usize)).b32.s1 = rover;
                rover = q;
            } else {
                q = rover;
                while (*mem.add((q + 1) as usize)).b32.s1 < p {
                    q = (*mem.add((q + 1) as usize)).b32.s1;
                }
                r = (*mem.add((p + 1) as usize)).b32.s1;
                (*mem.add((p + 1) as usize)).b32.s1 = (*mem.add((q + 1) as usize)).b32.s1;
                (*mem.add((q + 1) as usize)).b32.s1 = p;
                p = r;
            }
        }

        p = rover;
        while (*mem.add((p + 1) as usize)).b32.s1 != MAX_HALFWORD {
            (*mem.add(((*mem.add((p + 1) as usize)).b32.s1 + 1) as usize))
                .b32
                .s0 = p;
            p = (*mem.add((p + 1) as usize)).b32.s1;
        }

        (*mem.add((p + 1) as usize)).b32.s1 = rover;
        (*mem.add((rover + 1) as usize)).b32.s0 = p;
    }
}

pub unsafe fn primitive(ident: *const libc::c_char, c: u16, o: i32) {
    use super::xetex_xetexd::{
        buf_size, buffer, cur_val, eqtb, first, hash, pool_ptr, str_ptr, str_start,
    };
    unsafe extern "C" {
        fn maketexstring(value: *const libc::c_char) -> str_number;
    }
    unsafe {
        let mut prim_val: i32;
        let len = libc::strlen(ident);
        if len > 1 {
            let s = maketexstring(ident);
            if first as usize + len > (buf_size + 1) as usize {
                overflow(c"buffer size".as_ptr(), (buf_size as usize) as i32);
            }
            for i in 0..len {
                *buffer.add(first as usize + i) = *ident.add(i) as UnicodeScalar;
            }
            cur_val = id_lookup(first, len as i32);
            str_ptr -= 1;
            pool_ptr = *str_start.add((str_ptr - TOO_BIG_CHAR) as usize);
            (*hash.add(cur_val as usize)).s1 = s;
            prim_val = prim_lookup(s);
        } else {
            cur_val = *ident as i32 + SINGLE_BASE;
            prim_val = prim_lookup(*ident as i32);
        }

        (*eqtb.add(cur_val as usize)).b16.s0 = (LEVEL_ONE) as u16;
        (*eqtb.add(cur_val as usize)).b16.s1 = c;
        (*eqtb.add(cur_val as usize)).b32.s1 = o;
        (*eqtb.add((PRIM_EQTB_BASE + prim_val) as usize)).b16.s0 = (LEVEL_ONE) as u16;
        (*eqtb.add((PRIM_EQTB_BASE + prim_val) as usize)).b16.s1 = c;
        (*eqtb.add((PRIM_EQTB_BASE + prim_val) as usize)).b32.s1 = o;
    }
}

#[unsafe(export_name = "tectonic_rust_new_trie_op")]
pub unsafe extern "C" fn new_trie_op(
    d: small_number,
    n: small_number,
    v: trie_opcode,
) -> trie_opcode {
    unsafe {
        let mut h: i32;
        let mut u: trie_opcode;
        let mut l: i32;
        h = (n as i32 + 313 * d as i32 + 361 * v as i32 + 1009 * cur_lang as i32).abs()
            % (TRIE_OP_SIZE - NEG_TRIE_OP_SIZE)
            + NEG_TRIE_OP_SIZE;
        loop {
            l = *TRIE_OP_HASH(h);
            if l == 0 {
                if trie_op_ptr == TRIE_OP_SIZE {
                    overflow(
                        c"pattern memory ops".as_ptr(),
                        (TRIE_OP_SIZE as usize) as i32,
                    );
                }
                u = trie_used[cur_lang as usize];
                if u as i32 == MAX_TRIE_OP {
                    overflow(
                        c"pattern memory ops per language".as_ptr(),
                        ((MAX_TRIE_OP - MIN_TRIE_OP) as usize) as i32,
                    );
                }
                trie_op_ptr += 1;
                u += 1;
                trie_used[cur_lang as usize] = u;
                if u > max_op_used {
                    max_op_used = u;
                }
                hyf_distance[trie_op_ptr as usize] = d;
                hyf_num[trie_op_ptr as usize] = n;
                hyf_next[trie_op_ptr as usize] = v;
                trie_op_lang[trie_op_ptr as usize] = cur_lang;
                *TRIE_OP_HASH(h) = trie_op_ptr;
                trie_op_val[trie_op_ptr as usize] = u;
                return u;
            }
            if hyf_distance[l as usize] == d
                && hyf_num[l as usize] == n
                && hyf_next[l as usize] == v
                && trie_op_lang[l as usize] == cur_lang
            {
                return trie_op_val[l as usize];
            }
            if h > -TRIE_OP_SIZE {
                h -= 1;
            } else {
                h = TRIE_OP_SIZE;
            }
        }
    }
}

pub unsafe fn trie_node(p: trie_pointer) -> trie_pointer {
    unsafe {
        let mut h: trie_pointer;
        let mut q: trie_pointer;
        h = ((*trie_c.add(p as usize) as i32)
            + 1009 * (*trie_o.add(p as usize) as i32)
            + 2718 * (*trie_l.add(p as usize) as i32)
            + 3142 * (*trie_r.add(p as usize) as i32))
            .abs() as trie_pointer
            % (trie_size as u32) as i32;
        loop {
            q = *trie_hash.add(h as usize);
            if q == 0 {
                *trie_hash.add(h as usize) = p;
                return p;
            }
            if *trie_c.add(q as usize) == *trie_c.add(p as usize)
                && *trie_o.add(q as usize) == *trie_o.add(p as usize)
                && *trie_l.add(q as usize) == *trie_l.add(p as usize)
                && *trie_r.add(q as usize) == *trie_r.add(p as usize)
            {
                return q;
            }
            if h > 0 {
                h -= 1;
            } else {
                h = (trie_size as u32) as i32;
            }
        }
    }
}

pub unsafe fn compress_trie(p: trie_pointer) -> trie_pointer {
    unsafe {
        if p == 0 {
            0
        } else {
            *trie_l.add(p as usize) = compress_trie(*trie_l.add(p as usize));
            *trie_r.add(p as usize) = compress_trie(*trie_r.add(p as usize));
            trie_node(p)
        }
    }
}

pub unsafe fn first_fit(p: trie_pointer) {
    unsafe {
        let mut h: trie_pointer;
        let mut z: trie_pointer;
        let mut q: trie_pointer;
        let c: UTF16_code;
        let mut l: trie_pointer;
        let mut r: trie_pointer;
        let mut ll: i32;

        c = *trie_c.add(p as usize);
        z = trie_min[c as usize];
        loop {
            h = z.wrapping_sub((c as u32) as i32);
            if trie_max < h + (max_hyph_char as u32) as i32 {
                if trie_size as u32 <= (h + (max_hyph_char as u32) as i32) as u32 {
                    overflow(c"pattern memory".as_ptr(), (trie_size as usize) as i32);
                }
                loop {
                    trie_max += 1;
                    *trie_taken.add(trie_max as usize) = false;
                    *trie_trl.add(trie_max as usize) = trie_max + 1;
                    *trie_tro.add(trie_max as usize) = trie_max - 1;
                    if trie_max == h + (max_hyph_char as u32) as i32 {
                        break;
                    }
                }
            }
            if *trie_taken.add(h as usize) {
                z = *trie_trl.add(z as usize);
                continue;
            }
            q = *trie_r.add(p as usize);
            let mut not_found = false;
            while q > 0 {
                if *trie_trl.add((h + (*trie_c.add(q as usize) as u32) as i32) as usize) == 0 {
                    not_found = true;
                    break;
                }
                q = *trie_r.add(q as usize);
            }
            if not_found {
                z = *trie_trl.add(z as usize);
                continue;
            }
            break;
        }

        *trie_taken.add(h as usize) = true;
        *trie_hash.add(p as usize) = h;
        q = p;
        loop {
            z = h + (*trie_c.add(q as usize) as u32) as i32;
            l = *trie_tro.add(z as usize);
            r = *trie_trl.add(z as usize);
            *trie_tro.add(r as usize) = l;
            *trie_trl.add(l as usize) = r;
            *trie_trl.add(z as usize) = 0;
            if l < (max_hyph_char as u32) as i32 {
                if z < (max_hyph_char as u32) as i32 {
                    ll = z as i32;
                } else {
                    ll = max_hyph_char;
                }
                loop {
                    trie_min[l as usize] = r;
                    l += 1;
                    if l == (ll as u32) as i32 {
                        break;
                    }
                }
            }
            q = *trie_r.add(q as usize);
            if q == 0 {
                break;
            }
        }
    }
}

pub unsafe fn trie_pack(mut p: trie_pointer) {
    unsafe {
        let mut q: trie_pointer;
        loop {
            q = *trie_l.add(p as usize);
            if q > 0 && *trie_hash.add(q as usize) == 0 {
                first_fit(q);
                trie_pack(q);
            }
            p = *trie_r.add(p as usize);
            if p == 0 {
                break;
            }
        }
    }
}

pub unsafe fn trie_fix(mut p: trie_pointer) {
    unsafe {
        let mut q: trie_pointer;
        let mut c: UTF16_code;
        let z: trie_pointer;
        z = *trie_hash.add(p as usize);
        loop {
            q = *trie_l.add(p as usize);
            c = *trie_c.add(p as usize);
            *trie_trl.add((z + (c as u32) as i32) as usize) = *trie_hash.add(q as usize);
            *trie_trc.add((z + (c as u32) as i32) as usize) = c;
            *trie_tro.add((z + (c as u32) as i32) as usize) =
                (*trie_o.add(p as usize) as u32) as i32;
            if q > 0 {
                trie_fix(q);
            }
            p = *trie_r.add(p as usize);
            if p == 0 {
                break;
            }
        }
    }
}
