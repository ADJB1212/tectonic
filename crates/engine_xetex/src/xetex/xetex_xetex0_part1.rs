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
    A_LEADERS, ADDITIONAL, ADJUST_NODE, BEFORE, BIGGEST_CHAR, BOX_NODE_SIZE, DIMEN_BASE, DISC_NODE,
    EXACTLY, EXPLICIT, FIL, FILL, FILLL, FONT_BASE, GARBAGE, GLUE_BASE, GLYPH_NODE, HLIST_NODE,
    INT_BASE, INT_PAR__texxet, INT_PAR__tracing_online, INT_PAR__xetex_use_glyph_metrics,
    KERN_NODE, L_CODE, LIGATURE_NODE, LIST_TAG, MARGIN_KERN_NODE, NATIVE_NODE_SIZE,
    NATIVE_WORD_NODE, NATIVE_WORD_NODE_AT, NON_ADDRESS, NORMAL, OTHER_CHAR, PDF_NODE, PIC_NODE,
    QUAD_CODE, RELAX, RULE_NODE, SHRINKING, SMALL_NODE_SIZE, SPACER, STRETCHING, TEX_NULL,
    TOO_BIG_CHAR, VLIST_NODE, WHATSIT_NODE,
};
use super::xetex_core::{SELECTOR_LOG_ONLY, SELECTOR_NEW_STRING, scaled_t};
use super::xetex_errors::{confusion, error, overflow};
use super::xetex_ext::{
    AAT_FONT_FLAG, OTGR_FONT_FLAG, PreparedLayouts, apply_tfm_font_mapping, map_char_to_glyph,
    real_get_native_word_cp, xrealloc,
};
use super::xetex_io::INVALID_HANDLE;
use super::xetex_math::{is_char_node, odd};
use super::xetex_output::{
    capture_to_diagnostic, diagnostic_begin_capture_warning_here, error_here_with_diagnostic,
    print, print_char, print_cstr, print_file_line, print_file_name, print_hex, print_int,
    print_ln, print_nl_cstr, print_raw_char, print_scaled, print_ucs_code, sprint_cs,
};
use super::xetex_scaledmath::{round_xn_over_d, xn_over_d};
use super::xetex_shipout::{_tt_abort, cur_length, print_c_string, xmalloc};
use super::xetex_stringpool::length;
use super::xetex_texmfmp::{gettexstring, maketexstring};
use super::xetex_xetex0::{
    back_input, badness, begin_diagnostic, begin_file_reading, delete_token_ref, effective_char,
    end_diagnostic, flush_node_list, free_node, get_avail, get_node, get_x_token, new_math,
    new_param_glue, new_penalty, new_rule, new_save_level, scan_dimen, scan_keyword,
    scan_left_brace, scan_toks, short_display, show_box, show_token_list,
};
use super::xetex_xetexd::{
    LR_problems, LR_ptr, UTF16_code, UnicodeScalar, adjust_tail, area_delimiter, avail, b16x4,
    bchar_label, buffer, char_base, cur_area, cur_chr, cur_cmd, cur_cs, cur_ext, cur_input,
    cur_list, cur_name, cur_val, def_ref, depth_base, eqtb, ext_delimiter, exten_base,
    file_line_error_style_p, file_name_quote_char, file_offset, first, fmem_ptr, font_area,
    font_bc, font_bchar, font_check, font_dsize, font_ec, font_false_bchar, font_flags, font_glue,
    font_in_short_display, font_index, font_info, font_layout_engine, font_letter_space,
    font_mapping, font_max, font_mem_size, font_name, font_params, font_ptr, font_size, font_used,
    full_source_filename_stack, gave_char_warning_help, glue_ord, group_code, height_base,
    help_line, help_ptr, hyphen_char, in_open, init_pool_ptr, init_str_ptr, input_file, input_ptr,
    input_stack, internal_font_number, italic_base, job_name, kern_base, last, last_badness,
    last_leftmost_char, last_rightmost_char, lig_kern_base, line, loaded_font_flags,
    loaded_font_letter_space, loaded_font_mapping, log_file, log_opened, main_f, mapped_text,
    max_print_line, max_strings, mem, memory_word, name_in_progress, name_length, name_length16,
    name_of_file, name_of_file16, native_font_type_flag, native_text, old_setting, open_parens,
    output_active, pack_begin_line, param_base, pool_pointer, pool_ptr, pool_size, pre_adjust_tail,
    quoted_filename, rust_input_handle_t, rust_stdout, save_ptr, save_stack, scanner_status,
    selector, skew_char, small_number, source_filename_stack, stop_at_space, str_number, str_pool,
    str_ptr, str_start, temp_ptr, term_offset, texmf_log_name, total_shrink, total_stretch,
    warning_index, width_base, xtx_ligature_present,
};

use super::bridge::{
    ttstub_diag_printf, ttstub_input_close, ttstub_issue_warning, ttstub_output_flush,
    ttstub_output_open,
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
use super::xetex_scaledmath::*;
use super::xetex_shipout::*;
use super::xetex_synctex::*;
use super::xetex_xetex0::*;
use super::xetex_xetexd::*;
use super::*;
use libc::*;

#[inline]
unsafe fn mfree<T>(ptr: *mut T) -> *mut T {
    if !ptr.is_null() {
        libc::free(ptr as *mut libc::c_void);
    }
    std::ptr::null_mut()
}

#[inline]
fn IS_DIR_SEP(c: u32) -> bool {
    c == b'/' as u32
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

macro_rules! BOX_width {
    ($p:expr) => {
        mem!(($p) + 1).b32.s1
    };
}

macro_rules! CHAR_NODE_character {
    ($p:expr) => {
        mem!($p).b16.s0
    };
}

macro_rules! CHAR_NODE_font {
    ($p:expr) => {
        mem!($p).b16.s1
    };
}

macro_rules! NATIVE_NODE_font {
    ($p:expr) => {
        mem!(($p) + 4).b16.s2
    };
}

macro_rules! NATIVE_NODE_length {
    ($p:expr) => {
        mem!(($p) + 4).b16.s1
    };
}

macro_rules! NATIVE_NODE_glyph_count {
    ($p:expr) => {
        mem!(($p) + 4).b16.s0
    };
}

macro_rules! NATIVE_NODE_glyph_info_ptr {
    ($p:expr) => {
        mem!(($p) + 5).ptr
    };
}

macro_rules! NATIVE_NODE_size {
    ($p:expr) => {
        mem!(($p) + 4).b16.s3
    };
}

macro_rules! NATIVE_NODE_text {
    ($p:expr) => {
        mem.offset((($p) + NATIVE_NODE_SIZE) as isize) as *mut u16
    };
}

macro_rules! DISCRETIONARY_NODE_replace_count {
    ($p:expr) => {
        mem!($p).b16.s0
    };
}

macro_rules! FONT_CHARACTER_INFO {
    ($f:expr, $c:expr) => {
        (*font_info.offset((*char_base.offset(($f) as isize) + ($c as i32)) as isize)).b16
    };
}

macro_rules! FONT_CHARINFO_WIDTH {
    ($f:expr, $info:expr) => {
        (*font_info.offset((*width_base.offset(($f) as isize) + (($info).s3 as i32)) as isize))
            .b32
            .s1
    };
}

macro_rules! FONT_CHARINFO_HEIGHT {
    ($f:expr, $info:expr) => {
        (*font_info
            .offset((*height_base.offset(($f) as isize) + (($info).s2 as i32 / 16)) as isize))
        .b32
        .s1
    };
}

macro_rules! FONT_CHARINFO_DEPTH {
    ($f:expr, $info:expr) => {
        (*font_info.offset((*depth_base.offset(($f) as isize) + (($info).s2 as i32 % 16)) as isize))
            .b32
            .s1
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

macro_rules! set_help_line {
    ($idx:expr, $str:expr) => {
        *(&raw mut help_line)
            .cast::<*const libc::c_char>()
            .offset($idx) = $str;
    };
}

macro_rules! get_native_word_cp {
    ($p:expr, $side:expr) => {
        real_get_native_word_cp(mem.offset(($p) as isize) as *mut c_void, ($side) as c_int)
    };
}

#[repr(C)]
pub struct NativeLayoutRequest {
    pub engine: *mut c_void,
    pub text: *const u16,
    pub length: usize,
}

unsafe extern "C" {
    fn ttstub_input_getc(handle: rust_input_handle_t) -> c_int;
    fn get_loaded_font_design_size() -> i32;
    pub fn ttxl_prepare_layouts(
        requests: *const NativeLayoutRequest,
        count: usize,
    ) -> *mut PreparedLayouts;
}

// 1. begin_name
#[unsafe(export_name = "begin_name")]
pub unsafe extern "C" fn begin_name() {
    area_delimiter = 0;
    ext_delimiter = 0;
    quoted_filename = false;
    file_name_quote_char = 0;
}

// 2. more_name
#[unsafe(export_name = "more_name")]
pub unsafe extern "C" fn more_name(c: UTF16_code) -> bool {
    if stop_at_space && file_name_quote_char == 0 && c == b' ' as UTF16_code {
        return false;
    }

    if stop_at_space && file_name_quote_char != 0 && c == file_name_quote_char {
        file_name_quote_char = 0;
        return true;
    }

    if stop_at_space
        && file_name_quote_char == 0
        && (c == b'"' as UTF16_code || c == b'\'' as UTF16_code)
    {
        file_name_quote_char = c;
        quoted_filename = true;
        return true;
    }

    if pool_ptr + 1 > pool_size {
        overflow(
            b"pool size\0".as_ptr() as *const c_char,
            pool_size - init_pool_ptr,
        );
    }

    *str_pool.offset(pool_ptr as isize) = c;
    pool_ptr += 1;

    if IS_DIR_SEP(c as u32) {
        area_delimiter = cur_length();
        ext_delimiter = 0;
    } else if c == b'.' as UTF16_code {
        ext_delimiter = cur_length();
    }

    true
}

// 3. end_name
#[unsafe(export_name = "end_name")]
pub unsafe extern "C" fn end_name() {
    super::xetex_stringpool::truncate_string_index(str_ptr);
    let mut temp_str: str_number;
    let mut j: pool_pointer;

    if str_ptr + 3 > max_strings {
        overflow(
            b"number of strings\0".as_ptr() as *const c_char,
            max_strings - init_str_ptr,
        );
    }

    if area_delimiter == 0 {
        cur_area = EMPTY_STRING;
    } else {
        cur_area = str_ptr;
        *str_start.offset(((str_ptr + 1) - 65536) as isize) =
            *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) + area_delimiter;
        str_ptr += 1;
        temp_str = search_string(cur_area);

        if temp_str > 0 {
            cur_area = temp_str;
            str_ptr -= 1;

            j = *str_start.offset(((str_ptr + 1) - 65536) as isize);
            while j <= pool_ptr - 1 {
                *str_pool.offset((j - area_delimiter) as isize) = *str_pool.offset(j as isize);
                j += 1;
            }

            pool_ptr = pool_ptr - area_delimiter;
        }
    }

    if ext_delimiter == 0 {
        cur_ext = EMPTY_STRING;
        cur_name = slow_make_string();
    } else {
        cur_name = str_ptr;
        *str_start.offset(((str_ptr + 1) - 65536) as isize) =
            *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) + ext_delimiter
                - area_delimiter
                - 1;
        str_ptr += 1;

        cur_ext = make_string();
        str_ptr -= 1;
        temp_str = search_string(cur_name);

        if temp_str > 0 {
            cur_name = temp_str;
            str_ptr -= 1;

            j = *str_start.offset(((str_ptr + 1) - 65536) as isize);
            while j <= pool_ptr - 1 {
                *str_pool.offset((j - ext_delimiter + area_delimiter + 1) as isize) =
                    *str_pool.offset(j as isize);
                j += 1;
            }

            pool_ptr = pool_ptr - ext_delimiter + area_delimiter + 1;
        }

        cur_ext = slow_make_string();
    }
}

// 4. pack_file_name
#[unsafe(export_name = "pack_file_name")]
pub unsafe extern "C" fn pack_file_name(n: str_number, a: str_number, e: str_number) {
    let buf_len = ((length(a) + length(n) + length(e)) * 3 + 1) as usize;
    let work_buffer = xmalloc(buf_len) as *mut c_char;
    *work_buffer = 0;

    let a_utf8 = gettexstring(a);
    strcat(work_buffer, a_utf8);
    free(a_utf8 as *mut c_void);

    let n_utf8 = gettexstring(n);
    strcat(work_buffer, n_utf8);
    free(n_utf8 as *mut c_void);

    let e_utf8 = gettexstring(e);
    strcat(work_buffer, e_utf8);
    free(e_utf8 as *mut c_void);

    name_length = strlen(work_buffer) as i32;

    free(name_of_file as *mut c_void);
    name_of_file = xmalloc((name_length as usize) + 1) as *mut c_char;
    strcpy(name_of_file, work_buffer);
    free(work_buffer as *mut c_void);
}

// 5. make_name_string
#[unsafe(export_name = "make_name_string")]
pub unsafe extern "C" fn make_name_string() -> str_number {
    let mut k: i32;
    let save_area_delimiter: pool_pointer;
    let save_ext_delimiter: pool_pointer;
    let save_name_in_progress: bool;
    let save_stop_at_space: bool;

    if pool_ptr + name_length > pool_size || str_ptr == max_strings || cur_length() > 0 {
        return b'?' as str_number;
    }

    make_utf16_name();

    k = 0;
    while k < name_length16 {
        *str_pool.offset(pool_ptr as isize) = *name_of_file16.offset(k as isize);
        pool_ptr += 1;
        k += 1;
    }

    let Result: str_number = make_string();

    save_area_delimiter = area_delimiter;
    save_ext_delimiter = ext_delimiter;
    save_name_in_progress = name_in_progress;
    save_stop_at_space = stop_at_space;
    name_in_progress = true;
    begin_name();
    stop_at_space = false;
    k = 0;

    while k < name_length16 && more_name(*name_of_file16.offset(k as isize)) {
        k += 1;
    }

    stop_at_space = save_stop_at_space;
    end_name();
    name_in_progress = save_name_in_progress;
    area_delimiter = save_area_delimiter;
    ext_delimiter = save_ext_delimiter;

    Result
}

// 6. scan_file_name_braced
#[unsafe(export_name = "scan_file_name_braced")]
pub unsafe extern "C" fn scan_file_name_braced() {
    let save_scanner_status: u8 = scanner_status;
    let save_def_ref: i32 = def_ref;
    let save_cur_cs: i32 = cur_cs;
    let s: str_number;
    let mut i: pool_pointer;
    let save_stop_at_space: bool;

    cur_cs = warning_index;
    scan_toks(false, true);

    old_setting = (selector) as u8;
    selector = SELECTOR_NEW_STRING;
    show_token_list(mem!(def_ref).b32.s1, TEX_NULL, pool_size - pool_ptr);
    selector = (old_setting) as i32;
    s = make_string();
    delete_token_ref(def_ref);
    def_ref = save_def_ref;
    cur_cs = save_cur_cs;
    scanner_status = save_scanner_status;
    save_stop_at_space = stop_at_space;

    begin_name();

    i = *str_start.offset((s - TOO_BIG_CHAR) as isize);
    let end_i = *str_start.offset((s + 1 - TOO_BIG_CHAR) as isize);
    while i < end_i {
        more_name(*str_pool.offset(i as isize));
        i += 1;
    }

    stop_at_space = save_stop_at_space;
}

// 7. scan_file_name
#[unsafe(export_name = "scan_file_name")]
pub unsafe extern "C" fn scan_file_name() {
    let save_warning_index: i32 = warning_index;
    warning_index = cur_cs;

    loop {
        get_x_token();
        if !(cur_cmd == (SPACER) as u8 || cur_cmd == (RELAX) as u8) {
            break;
        }
    }

    back_input();

    if cur_cmd == (LEFT_BRACE) as u8 {
        scan_file_name_braced();
    } else {
        name_in_progress = true;
        begin_name();

        loop {
            get_x_token();
            if cur_cmd != (SPACER) as u8 {
                break;
            }
        }

        loop {
            if cur_cmd > (OTHER_CHAR) as u8 || cur_chr > BIGGEST_CHAR {
                back_input();
                break;
            }

            if !more_name(cur_chr as UTF16_code) {
                break;
            }

            get_x_token();
        }
    }

    end_name();
    name_in_progress = false;
    warning_index = save_warning_index;
}

// 8. pack_job_name
#[unsafe(export_name = "pack_job_name")]
pub unsafe extern "C" fn pack_job_name(s: *const c_char) {
    cur_area = EMPTY_STRING;
    cur_ext = maketexstring(s);
    cur_name = job_name;
    pack_file_name(cur_name, cur_area, cur_ext);
}

// 9. open_log_file
#[unsafe(export_name = "open_log_file")]
pub unsafe extern "C" fn open_log_file() {
    let local_old_setting: u8;
    let mut k: i32;
    let mut l: i32;

    local_old_setting = (selector) as u8;
    if job_name == 0 {
        job_name = maketexstring(b"texput\0".as_ptr() as *const c_char);
    }

    pack_job_name(b".log\0".as_ptr() as *const c_char);

    log_file = ttstub_output_open(name_of_file, 0);
    if log_file == INVALID_HANDLE {
        _tt_abort(
            b"cannot open log file output \"%s\"\0".as_ptr() as *const c_char,
            name_of_file,
        );
    }

    texmf_log_name = make_name_string();
    selector = SELECTOR_LOG_ONLY;
    log_opened = true;

    *input_stack.offset(input_ptr as isize) = cur_input;

    print_nl_cstr(b"**\0".as_ptr() as *const c_char);
    l = (*input_stack.offset(0)).limit;
    if *buffer.offset(l as isize) as i32 == INTPAR!(INT_PAR__end_line_char) {
        l -= 1;
    }

    k = 1;
    while k <= l {
        print(*buffer.offset(k as isize) as i32);
        k += 1;
    }

    print_ln();
    selector = (local_old_setting + 2) as i32;
}

// 10. start_input
#[unsafe(export_name = "start_input")]
pub unsafe extern "C" fn start_input(primary_input_name: *const c_char) {
    let mut format: c_int = TTBC_FILE_FORMAT_TEX;
    let mut temp_str: str_number;

    if !primary_input_name.is_null() {
        format = TTBC_FILE_FORMAT_TECTONIC_PRIMARY;

        name_in_progress = true;
        begin_name();
        stop_at_space = false;

        let mut cp = primary_input_name as *const u8;

        if pool_ptr + (strlen(primary_input_name) * 2) as i32 >= pool_size {
            _tt_abort(
                b"string pool overflow [%i bytes]\0".as_ptr() as *const c_char,
                pool_size as c_int,
            );
        }

        while *cp != 0 {
            let mut rval = *cp as u32;
            cp = cp.add(1);
            let extraBytes = bytesFromUTF8[rval as usize];
            match extraBytes {
                5 => {
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                }
                4 => {
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                }
                3 => {
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                }
                2 => {
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                }
                1 => {
                    rval <<= 6;
                    if *cp != 0 {
                        rval += *cp as u32;
                        cp = cp.add(1);
                    }
                }
                _ => {}
            }
            rval -= offsetsFromUTF8[extraBytes as usize];
            if rval > 0xffff {
                rval -= 0x10000;
                *str_pool.offset(pool_ptr as isize) = (0xd800 + rval / 0x0400) as u16;
                pool_ptr += 1;
                *str_pool.offset(pool_ptr as isize) = (0xdc00 + rval % 0x0400) as u16;
                pool_ptr += 1;
            } else {
                *str_pool.offset(pool_ptr as isize) = rval as u16;
                pool_ptr += 1;
            }

            if IS_DIR_SEP(rval) {
                area_delimiter = cur_length();
                ext_delimiter = 0;
            } else if rval == b'.' as u32 {
                ext_delimiter = cur_length();
            }
        }

        stop_at_space = true;
        end_name();
        name_in_progress = false;
    } else {
        scan_file_name();
    }

    pack_file_name(cur_name, cur_area, cur_ext);

    begin_file_reading();

    if u_open_in(
        input_file.offset(cur_input.index as isize),
        format,
        b"rb\0".as_ptr() as *const c_char,
        INTPAR!(INT_PAR__xetex_default_input_mode),
        INTPAR!(INT_PAR__xetex_default_input_encoding),
    ) == 0
    {
        _tt_abort(
            b"failed to open input file \"%s\"\0".as_ptr() as *const c_char,
            name_of_file,
        );
    }

    make_utf16_name();
    name_in_progress = true;
    begin_name();
    stop_at_space = false;
    let mut k = 0;
    while k < name_length16 && more_name(*name_of_file16.offset(k as isize)) {
        k += 1;
    }
    stop_at_space = true;
    end_name();
    name_in_progress = false;

    cur_input.name = make_name_string();
    *source_filename_stack.offset(in_open as isize) = cur_input.name;

    *full_source_filename_stack.offset(in_open as isize) = maketexstring(name_of_input_file);
    if cur_input.name == str_ptr - 1 {
        temp_str = search_string(cur_input.name);
        if temp_str > 0 {
            cur_input.name = temp_str;
            str_ptr -= 1;
            pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);
        }
    }

    if job_name == 0 {
        job_name = cur_name;
        open_log_file();
    }

    if term_offset + length(*full_source_filename_stack.offset(in_open as isize))
        > max_print_line - 2
    {
        print_ln();
    } else if term_offset > 0 || file_offset > 0 {
        print_char(b' ' as i32);
    }
    print_char(b'(' as i32);
    open_parens += 1;
    print(*full_source_filename_stack.offset(in_open as isize));
    ttstub_output_flush(rust_stdout);

    if INTPAR!(INT_PAR__tracing_stack_levels) > 0 {
        let mut v: i32;

        begin_diagnostic();
        diagnostic_begin_capture_warning_here();
        print_ln();
        print_char(b'~' as i32);
        v = input_ptr - 1;
        if v < INTPAR!(INT_PAR__tracing_stack_levels) {
            while v > 0 {
                print_char(b'.' as i32);
                v -= 1;
            }
        } else {
            print_char(b'~' as i32);
        }
        print_cstr(b"INPUT \0".as_ptr() as *const c_char);
        print(cur_name);
        print(cur_ext);
        print_ln();
        capture_to_diagnostic(std::ptr::null_mut());
        end_diagnostic(false);
    }

    cur_input.state = (NEW_LINE as u8) as u16;

    synctex_start_input();

    line = 1;
    input_line(*input_file.offset(cur_input.index as isize));
    cur_input.limit = last;

    if INTPAR!(INT_PAR__end_line_char) < 0 || INTPAR!(INT_PAR__end_line_char) > 255 {
        cur_input.limit -= 1;
    } else {
        *buffer.offset(cur_input.limit as isize) = INTPAR!(INT_PAR__end_line_char) as UnicodeScalar;
    }

    first = cur_input.limit + 1;
    cur_input.loc = cur_input.start;
}

// 11. effective_char_info
#[unsafe(export_name = "effective_char_info")]
pub unsafe extern "C" fn effective_char_info(f: internal_font_number, mut c: u16) -> b16x4 {
    if !xtx_ligature_present && !(*font_mapping.offset(f as isize)).is_null() {
        c = apply_tfm_font_mapping(*font_mapping.offset(f as isize), c as i32) as u16;
    }

    xtx_ligature_present = false;
    FONT_CHARACTER_INFO!(f, c)
}

// 12. char_warning
#[unsafe(export_name = "char_warning")]
pub unsafe extern "C" fn char_warning(f: internal_font_number, c: i32) {
    let mut local_old_setting: i32;

    if INTPAR!(INT_PAR__tracing_lost_chars) > 0 {
        local_old_setting = INTPAR!(INT_PAR__tracing_online);
        if INTPAR!(INT_PAR__tracing_lost_chars) > 1 {
            INTPAR!(INT_PAR__tracing_online) = 1;
        }

        if INTPAR!(INT_PAR__tracing_lost_chars) > 2 {
            if (file_line_error_style_p) != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! \0".as_ptr() as *const c_char);
            }
            print_cstr(b"Missing character: There is no \0".as_ptr() as *const c_char);
        } else {
            begin_diagnostic();
            diagnostic_begin_capture_warning_here();
            print_nl_cstr(b"Missing character: There is no \0".as_ptr() as *const c_char);
        }

        if c < 65536 {
            print(c);
        } else {
            print_char(c);
        }

        print_cstr(b" (\0".as_ptr() as *const c_char);

        if *font_area.offset(f as isize) == (AAT_FONT_FLAG) as i32
            || *font_area.offset(f as isize) == (OTGR_FONT_FLAG) as i32
        {
            print_ucs_code(c);
        } else {
            print_hex(c);
        }

        print_char(b')' as i32);
        print_cstr(b" in font \0".as_ptr() as *const c_char);
        print(*font_name.offset(f as isize));

        if INTPAR!(INT_PAR__tracing_lost_chars) < 3 {
            print_char(b'!' as i32);
        }

        INTPAR!(INT_PAR__tracing_online) = local_old_setting;

        if INTPAR!(INT_PAR__tracing_lost_chars) > 2 {
            help_ptr = 0;
            error();
        } else {
            capture_to_diagnostic(std::ptr::null_mut());
            end_diagnostic(false);
        }
    }

    {
        let fn_str = gettexstring(*font_name.offset(f as isize));
        let prev_selector = selector;
        selector = SELECTOR_NEW_STRING;
        if c < 0x10000 {
            print(c);
        } else {
            print_char(c);
        }
        selector = prev_selector;
        let s = make_string();
        let chr = gettexstring(s);
        str_ptr -= 1;
        pool_ptr = *str_start.offset((str_ptr - 0x10000) as isize);

        ttstub_issue_warning(
            b"could not represent character \"%s\" (0x%x) in font \"%s\"\0".as_ptr()
                as *const c_char,
            chr,
            c,
            fn_str,
        );

        free(fn_str as *mut c_void);
        free(chr as *mut c_void);

        if !gave_char_warning_help {
            ttstub_issue_warning(
                b"  you may need to load the `fontspec` package and use (e.g.) \\setmainfont to\0"
                    .as_ptr() as *const c_char,
            );
            ttstub_issue_warning(
                b"  choose a different font that covers the unrepresentable character(s)\0".as_ptr()
                    as *const c_char,
            );
            gave_char_warning_help = true;
        }
    }
}

// 13. new_native_word_node
#[unsafe(export_name = "new_native_word_node")]
pub unsafe extern "C" fn new_native_word_node(f: internal_font_number, n: i32) -> i32 {
    let l = NATIVE_NODE_SIZE
        + (n * std::mem::size_of::<UTF16_code>() as i32
            + std::mem::size_of::<memory_word>() as i32
            - 1)
            / std::mem::size_of::<memory_word>() as i32;
    let q = get_node(l);
    NODE_type!(q) = (WHATSIT_NODE as i16) as u16;

    if INTPAR!(INT_PAR__xetex_generate_actual_text) > 0 {
        NODE_subtype!(q) = (NATIVE_WORD_NODE_AT as i16) as u16;
    } else {
        NODE_subtype!(q) = (NATIVE_WORD_NODE as i16) as u16;
    }

    NATIVE_NODE_size!(q) = (l as i16) as u16;
    NATIVE_NODE_font!(q) = (f as i16) as u16;
    NATIVE_NODE_length!(q) = (n as i16) as u16;
    NATIVE_NODE_glyph_count!(q) = 0;
    NATIVE_NODE_glyph_info_ptr!(q) = std::ptr::null_mut();
    q
}

// 14. new_native_character
#[unsafe(export_name = "new_native_character")]
pub unsafe extern "C" fn new_native_character(
    f: internal_font_number,
    mut c: UnicodeScalar,
) -> i32 {
    let mut p: i32;
    let mut i: i32;
    let len: i32;

    if !(*font_mapping.offset(f as isize)).is_null() {
        if c > 65535 {
            if pool_ptr + 2 > pool_size {
                overflow(
                    b"pool size\0".as_ptr() as *const c_char,
                    pool_size - init_pool_ptr,
                );
            }

            *str_pool.offset(pool_ptr as isize) = ((c - 65536) / 1024 + 0xD800) as u16;
            pool_ptr += 1;
            *str_pool.offset(pool_ptr as isize) = ((c - 65536) % 1024 + 0xDC00) as u16;
            pool_ptr += 1;
        } else {
            if pool_ptr + 1 > pool_size {
                overflow(
                    b"pool size\0".as_ptr() as *const c_char,
                    pool_size - init_pool_ptr,
                );
            }

            *str_pool.offset(pool_ptr as isize) = c as u16;
            pool_ptr += 1;
        }

        len = apply_mapping(
            *font_mapping.offset(f as isize),
            str_pool.offset(*str_start.offset((str_ptr - TOO_BIG_CHAR) as isize) as isize),
            cur_length() as c_int,
        );
        pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);

        i = 0;
        while i < len {
            let mapped_val = *mapped_text.offset(i as isize);
            if mapped_val >= 0xD800 && mapped_val < 0xDC00 {
                c = ((mapped_val - 0xD800) as UnicodeScalar) * 1024
                    + (*mapped_text.offset((i + 1) as isize) as UnicodeScalar)
                    + 9216;
                if map_char_to_glyph(f as c_int, c) == 0 {
                    char_warning(f, c as i32);
                }
                i += 2;
            } else {
                if map_char_to_glyph(f as c_int, mapped_val as UnicodeScalar) == 0 {
                    char_warning(f, mapped_val as i32);
                }
                i += 1;
            }
        }

        p = new_native_word_node(f, len);

        i = 0;
        while i < len {
            *NATIVE_NODE_text!(p).offset(i as isize) = *mapped_text.offset(i as isize);
            i += 1;
        }
    } else {
        if INTPAR!(INT_PAR__tracing_lost_chars) > 0 {
            if map_char_to_glyph(f as c_int, c) == 0 {
                char_warning(f, c as i32);
            }
        }

        p = get_node(NATIVE_NODE_SIZE + 1);
        NODE_type!(p) = (WHATSIT_NODE as i16) as u16;
        NODE_subtype!(p) = (NATIVE_WORD_NODE as i16) as u16;
        NATIVE_NODE_size!(p) = ((NATIVE_NODE_SIZE + 1) as i16) as u16;
        NATIVE_NODE_glyph_count!(p) = 0;
        NATIVE_NODE_glyph_info_ptr!(p) = std::ptr::null_mut();
        NATIVE_NODE_font!(p) = (f as i16) as u16;

        if c > 65535 {
            NATIVE_NODE_length!(p) = 2;
            *NATIVE_NODE_text!(p).offset(0) = ((c - 65536) / 1024 + 0xD800) as u16;
            *NATIVE_NODE_text!(p).offset(1) = ((c - 65536) % 1024 + 0xDC00) as u16;
        } else {
            NATIVE_NODE_length!(p) = 1;
            *NATIVE_NODE_text!(p).offset(0) = c as u16;
        }
    }

    set_native_metrics(p, INTPAR!(INT_PAR__xetex_use_glyph_metrics) > 0);
    p
}

// 15. font_feature_warning
#[unsafe(export_name = "font_feature_warning")]
pub unsafe extern "C" fn font_feature_warning(
    featureNameP: *const c_void,
    featLen: i32,
    settingNameP: *const c_void,
    setLen: i32,
) {
    begin_diagnostic();
    diagnostic_begin_capture_warning_here();
    print_nl_cstr(b"Unknown \0".as_ptr() as *const c_char);
    if setLen > 0 {
        print_cstr(b"selector `\0".as_ptr() as *const c_char);
        print_utf8_str(settingNameP as *const u8, setLen);
        print_cstr(b"' for \0".as_ptr() as *const c_char);
    }
    print_cstr(b"feature `\0".as_ptr() as *const c_char);
    print_utf8_str(featureNameP as *const u8, featLen);
    print_cstr(b"' in font `\0".as_ptr() as *const c_char);
    let mut i = 0;
    while *name_of_file.offset(i) != 0 {
        print_raw_char(*name_of_file.offset(i) as u8 as UTF16_code, true);
        i += 1;
    }
    print_cstr(b"'.\0".as_ptr() as *const c_char);
    capture_to_diagnostic(std::ptr::null_mut());
    end_diagnostic(false);
}

// 16. font_mapping_warning
#[unsafe(export_name = "font_mapping_warning")]
pub unsafe extern "C" fn font_mapping_warning(
    mappingNameP: *const c_void,
    mappingNameLen: i32,
    warningType: i32,
) {
    begin_diagnostic();
    diagnostic_begin_capture_warning_here();
    if warningType == 0 {
        print_nl_cstr(b"Loaded mapping `\0".as_ptr() as *const c_char);
    } else {
        print_nl_cstr(b"Font mapping `\0".as_ptr() as *const c_char);
    }
    print_utf8_str(mappingNameP as *const u8, mappingNameLen);
    print_cstr(b"' for font `\0".as_ptr() as *const c_char);

    let mut i = 0;
    while *name_of_file.offset(i) != 0 {
        print_raw_char(*name_of_file.offset(i) as u8 as UTF16_code, true);
        i += 1;
    }

    match warningType {
        1 => {
            print_cstr(b"' not found.\0".as_ptr() as *const c_char);
        }
        2 => {
            print_cstr(b"' not usable;\0".as_ptr() as *const c_char);
            print_nl_cstr(
                b"bad mapping file or incorrect mapping type.\0".as_ptr() as *const c_char
            );
        }
        _ => {
            print_cstr(b"'.\0".as_ptr() as *const c_char);
        }
    }
    capture_to_diagnostic(std::ptr::null_mut());
    end_diagnostic(false);
}

// 17. graphite_warning
#[unsafe(export_name = "graphite_warning")]
pub unsafe extern "C" fn graphite_warning() {
    begin_diagnostic();
    diagnostic_begin_capture_warning_here();
    print_nl_cstr(b"Font `\0".as_ptr() as *const c_char);

    let mut i = 0;
    while *name_of_file.offset(i) != 0 {
        print_raw_char(*name_of_file.offset(i) as u8 as UTF16_code, true);
        i += 1;
    }

    print_cstr(
        b"' does not support Graphite. Trying OpenType layout instead.\0".as_ptr() as *const c_char,
    );
    capture_to_diagnostic(std::ptr::null_mut());
    end_diagnostic(false);
}

// 18. load_native_font
#[unsafe(export_name = "load_native_font")]
pub unsafe extern "C" fn load_native_font(
    u: i32,
    nom: str_number,
    aire: str_number,
    mut s: scaled_t,
) -> internal_font_number {
    let mut k: i32;
    let num_font_dimens: i32;
    let font_engine: *mut c_void;
    let actual_size: scaled_t;
    let p: i32;
    let mut ascent: scaled_t = 0;
    let mut descent: scaled_t = 0;
    let mut font_slant: scaled_t = 0;
    let mut x_ht: scaled_t = 0;
    let mut cap_ht: scaled_t = 0;
    let mut f: internal_font_number;
    let full_name: str_number;

    font_engine = find_native_font(name_of_file, s);
    if font_engine.is_null() {
        return FONT_BASE;
    }

    if s >= 0 {
        actual_size = s;
    } else if s != -1000 {
        actual_size = xn_over_d(get_loaded_font_design_size(), -s, 1000);
    } else {
        actual_size = get_loaded_font_design_size();
    }

    if pool_ptr + name_length > pool_size {
        overflow(
            b"pool size\0".as_ptr() as *const c_char,
            pool_size - init_pool_ptr,
        );
    }

    k = 0;
    while k < name_length {
        *str_pool.offset(pool_ptr as isize) = *name_of_file.offset(k as isize) as u8 as u16;
        pool_ptr += 1;
        k += 1;
    }

    full_name = make_string();

    f = FONT_BASE + 1;
    while f <= font_ptr {
        if *font_area.offset(f as isize) == native_font_type_flag
            && str_eq_str(*font_name.offset(f as isize), full_name)
            && *font_size.offset(f as isize) == actual_size
        {
            release_font_engine(font_engine, native_font_type_flag);

            str_ptr -= 1;
            pool_ptr = *str_start.offset((str_ptr - TOO_BIG_CHAR) as isize);

            return f;
        }
        f += 1;
    }

    if native_font_type_flag == (OTGR_FONT_FLAG) as i32 && isOpenTypeMathFont(font_engine) {
        num_font_dimens = 65;
    } else {
        num_font_dimens = 8;
    }

    if font_ptr == font_max || fmem_ptr + num_font_dimens > font_mem_size {
        error_here_with_diagnostic(b"Font \0".as_ptr() as *const c_char);
        sprint_cs(u);
        print_char(b'=' as i32);
        if file_name_quote_char != 0 {
            print_char(file_name_quote_char as i32);
        }
        print_file_name(nom, aire, cur_ext);
        if file_name_quote_char != 0 {
            print_char(file_name_quote_char as i32);
        }
        if s >= 0 {
            print_cstr(b" at \0".as_ptr() as *const c_char);
            print_scaled(s);
            print_cstr(b"pt\0".as_ptr() as *const c_char);
        } else if s != -1000 {
            print_cstr(b" scaled \0".as_ptr() as *const c_char);
            print_int(-s);
        }
        print_cstr(b" not loaded: Not enough room left\0".as_ptr() as *const c_char);
        capture_to_diagnostic(std::ptr::null_mut());

        help_ptr = 4;
        set_help_line!(
            3,
            b"I'm afraid I won't be able to make use of this font,\0".as_ptr() as *const c_char
        );
        set_help_line!(
            2,
            b"because my memory for character-size data is too small.\0".as_ptr() as *const c_char
        );
        set_help_line!(
            1,
            b"If you're really stuck, ask a wizard to enlarge me.\0".as_ptr() as *const c_char
        );
        set_help_line!(
            0,
            b"Or maybe try `I\\font<same font id>=<name of loaded font>'.\0".as_ptr()
                as *const c_char
        );

        error();
        return FONT_BASE;
    }

    font_ptr += 1;
    *font_area.offset(font_ptr as isize) = native_font_type_flag;
    *font_name.offset(font_ptr as isize) = full_name;
    (*font_check.offset(font_ptr as isize)).s3 = 0;
    (*font_check.offset(font_ptr as isize)).s2 = 0;
    (*font_check.offset(font_ptr as isize)).s1 = 0;
    (*font_check.offset(font_ptr as isize)).s0 = 0;
    *font_glue.offset(font_ptr as isize) = TEX_NULL;
    *font_dsize.offset(font_ptr as isize) = get_loaded_font_design_size();
    *font_size.offset(font_ptr as isize) = actual_size;

    if native_font_type_flag == (AAT_FONT_FLAG) as i32 {
        aat_get_font_metrics(
            font_engine,
            &mut ascent,
            &mut descent,
            &mut x_ht,
            &mut cap_ht,
            &mut font_slant,
        );
    } else {
        ot_get_font_metrics(
            font_engine,
            &mut ascent,
            &mut descent,
            &mut x_ht,
            &mut cap_ht,
            &mut font_slant,
        );
    }

    *height_base.offset(font_ptr as isize) = ascent;
    *depth_base.offset(font_ptr as isize) = -descent;
    *font_params.offset(font_ptr as isize) = num_font_dimens;
    *font_bc.offset(font_ptr as isize) = 0;
    *font_ec.offset(font_ptr as isize) = 65535;
    *font_used.offset(font_ptr as isize) = false;
    *hyphen_char.offset(font_ptr as isize) = INTPAR!(INT_PAR__default_hyphen_char);
    *skew_char.offset(font_ptr as isize) = INTPAR!(INT_PAR__default_skew_char);
    *param_base.offset(font_ptr as isize) = fmem_ptr - 1;
    *font_layout_engine.offset(font_ptr as isize) = font_engine;
    *font_mapping.offset(font_ptr as isize) = std::ptr::null_mut();
    *font_letter_space.offset(font_ptr as isize) = loaded_font_letter_space;

    p = new_native_character(font_ptr, b' ' as UnicodeScalar);
    s = BOX_width!(p) + loaded_font_letter_space;
    if !NATIVE_NODE_glyph_info_ptr!(p).is_null() {
        NATIVE_NODE_glyph_info_ptr!(p) = mfree(NATIVE_NODE_glyph_info_ptr!(p));
    }
    free_node(p, NATIVE_NODE_size!(p) as i32);

    (*font_info.offset(fmem_ptr as isize)).b32.s1 = font_slant;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = s;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = s / 2;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = s / 3;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = x_ht;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = *font_size.offset(font_ptr as isize);
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = s / 3;
    fmem_ptr += 1;
    (*font_info.offset(fmem_ptr as isize)).b32.s1 = cap_ht;
    fmem_ptr += 1;

    if num_font_dimens == 65 {
        (*font_info.offset(fmem_ptr as isize)).b32.s1 = num_font_dimens;
        fmem_ptr += 1;

        k = 0;
        while k <= 55 {
            (*font_info.offset(fmem_ptr as isize)).b32.s1 = get_ot_math_constant(font_ptr, k);
            fmem_ptr += 1;
            k += 1;
        }
    }

    *font_mapping.offset(font_ptr as isize) = loaded_font_mapping;
    *font_flags.offset(font_ptr as isize) = loaded_font_flags;
    font_ptr
}

// 19. do_locale_linebreaks
#[unsafe(export_name = "do_locale_linebreaks")]
pub unsafe extern "C" fn do_locale_linebreaks(s: i32, len: i32) {
    let mut offs: i32;
    let mut prevOffs: i32;
    let mut i: i32;
    let use_penalty: bool;
    let use_skip: bool;

    if INTPAR!(INT_PAR__xetex_linebreak_locale) == 0 || len == 1 {
        mem!(cur_list.tail).b32.s1 = new_native_word_node(main_f, len);
        cur_list.tail = LLIST_link!(cur_list.tail);

        i = 0;
        while i < len {
            *NATIVE_NODE_text!(cur_list.tail).offset(i as isize) =
                *native_text.offset((s + i) as isize);
            i += 1;
        }
        set_native_metrics(cur_list.tail, INTPAR!(INT_PAR__xetex_use_glyph_metrics) > 0);
    } else {
        use_skip = GLUEPAR!(GLUE_PAR__xetex_linebreak_skip) != 0;
        use_penalty = INTPAR!(INT_PAR__xetex_linebreak_penalty) != 0 || !use_skip;
        linebreak_start(
            main_f as c_int,
            INTPAR!(INT_PAR__xetex_linebreak_locale),
            native_text.offset(s as isize),
            len,
        );
        offs = 0;
        loop {
            prevOffs = offs;
            offs = linebreak_next(main_f as c_int);
            if offs > 0 {
                if prevOffs != 0 {
                    if use_penalty {
                        mem!(cur_list.tail).b32.s1 =
                            new_penalty(INTPAR!(INT_PAR__xetex_linebreak_penalty));
                        cur_list.tail = LLIST_link!(cur_list.tail);
                    }
                    if use_skip {
                        mem!(cur_list.tail).b32.s1 =
                            new_param_glue(GLUE_PAR__xetex_linebreak_skip as small_number);
                        cur_list.tail = LLIST_link!(cur_list.tail);
                    }
                }
                mem!(cur_list.tail).b32.s1 = new_native_word_node(main_f, offs - prevOffs);
                cur_list.tail = LLIST_link!(cur_list.tail);

                i = prevOffs;
                while i < offs {
                    *NATIVE_NODE_text!(cur_list.tail).offset((i - prevOffs) as isize) =
                        *native_text.offset((s + i) as isize);
                    i += 1;
                }
                set_native_metrics(cur_list.tail, INTPAR!(INT_PAR__xetex_use_glyph_metrics) > 0);
            }
            if offs < 0 {
                break;
            }
        }
    }
}

// 20. bad_utf8_warning
#[unsafe(export_name = "bad_utf8_warning")]
pub unsafe extern "C" fn bad_utf8_warning() {
    begin_diagnostic();
    diagnostic_begin_capture_warning_here();
    print_nl_cstr(b"Invalid UTF-8 byte or sequence\0".as_ptr() as *const c_char);
    if cur_input.name == 0 {
        print_cstr(b" in terminal input\0".as_ptr() as *const c_char);
    } else {
        print_cstr(b" at line \0".as_ptr() as *const c_char);
        print_int(line);
    }
    print_cstr(b" replaced by U+FFFD.\0".as_ptr() as *const c_char);
    capture_to_diagnostic(std::ptr::null_mut());
    end_diagnostic(false);
}

// 21. get_input_normalization_state
#[unsafe(export_name = "get_input_normalization_state")]
pub unsafe extern "C" fn get_input_normalization_state() -> i32 {
    if eqtb.is_null() {
        0
    } else {
        INTPAR!(INT_PAR__xetex_input_normalization)
    }
}

// 22. get_tracing_fonts_state
#[unsafe(export_name = "get_tracing_fonts_state")]
pub unsafe extern "C" fn get_tracing_fonts_state() -> i32 {
    INTPAR!(INT_PAR__xetex_tracing_fonts)
}

// 23. read_font_info
#[unsafe(export_name = "read_font_info")]
pub unsafe extern "C" fn read_font_info(
    u: i32,
    nom: str_number,
    aire: str_number,
    mut s: scaled_t,
) -> internal_font_number {
    const TTBC_FILE_FORMAT_TFM: c_int = 3;

    let mut k: font_index;
    let mut name_too_long: bool = false;
    let mut file_opened: bool = false;
    let mut lf: i32 = 0;
    let mut lh: i32 = 0;
    let mut bc: i32 = 0;
    let mut ec: i32 = 0;
    let mut nw: i32 = 0;
    let mut nh: i32 = 0;
    let mut nd: i32 = 0;
    let mut ni: i32 = 0;
    let mut nl: i32 = 0;
    let mut nk: i32 = 0;
    let mut ne: i32 = 0;
    let mut np: i32 = 0;
    let f: internal_font_number;
    let mut other_font: internal_font_number = FONT_BASE;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;
    let mut qw: b16x4 = b16x4 {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
    };
    let mut sw: scaled_t;
    let mut bch_label: i32;
    let mut local_bchar: i16;
    let mut z: scaled_t = 0;
    let mut alpha: i32;
    let mut beta: u8;
    let mut tfm_file: rust_input_handle_t = INVALID_HANDLE;
    let mut is_bad_tfm = false;

    pack_file_name(nom, aire, cur_ext);

    if INTPAR!(INT_PAR__xetex_tracing_fonts) > 0 {
        begin_diagnostic();
        diagnostic_begin_capture_warning_here();
        print_nl_cstr(b"Requested font \"\0".as_ptr() as *const c_char);
        print_c_string(name_of_file);
        print(b'"' as i32);
        if s < 0 {
            print_cstr(b" scaled \0".as_ptr() as *const c_char);
            print_int(-s);
        } else {
            print_cstr(b" at \0".as_ptr() as *const c_char);
            print_scaled(s);
            print_cstr(b"pt\0".as_ptr() as *const c_char);
        }
        capture_to_diagnostic(std::ptr::null_mut());
        end_diagnostic(false);
    }

    'done: {
        if quoted_filename {
            other_font = load_native_font(u, nom, aire, s);
            if other_font != FONT_BASE {
                break 'done;
            }
        }

        name_too_long = length(nom) > 255 || length(aire) > 255;
        if name_too_long {
            is_bad_tfm = true;
            break 'done;
        }

        pack_file_name(nom, aire, EMPTY_STRING);
        check_for_tfm_font_mapping();

        tfm_file = tt_xetex_open_input(TTBC_FILE_FORMAT_TFM);
        if tfm_file == INVALID_HANDLE {
            if !quoted_filename {
                other_font = load_native_font(u, nom, aire, s);
                if other_font != FONT_BASE {
                    break 'done;
                }
            }
            is_bad_tfm = true;
            break 'done;
        }

        file_opened = true;

        'read_tfm: {
            macro_rules! readfifteen {
                ($x:ident) => {
                    $x = ttstub_input_getc(tfm_file);
                    if $x > 127 || $x == libc::EOF {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    $x *= 256;
                    $x += ttstub_input_getc(tfm_file);
                };
            }

            readfifteen!(lf);
            readfifteen!(lh);
            readfifteen!(bc);
            readfifteen!(ec);

            if bc > ec + 1 || ec > 255 {
                is_bad_tfm = true;
                break 'read_tfm;
            }

            if bc > 255 {
                bc = 1;
                ec = 0;
            }

            readfifteen!(nw);
            readfifteen!(nh);
            readfifteen!(nd);
            readfifteen!(ni);
            readfifteen!(nl);
            readfifteen!(nk);
            readfifteen!(ne);
            readfifteen!(np);

            if lf != 6 + lh + (ec - bc + 1) + nw + nh + nd + ni + nl + nk + ne + np {
                is_bad_tfm = true;
                break 'read_tfm;
            }
            if nw == 0 || nh == 0 || nd == 0 || ni == 0 {
                is_bad_tfm = true;
                break 'read_tfm;
            }

            lf = lf - 6 - lh;
            if np < 7 {
                lf = lf + 7 - np;
            }

            if font_ptr == font_max || fmem_ptr + lf > font_mem_size {
                _tt_abort(b"not enough memory to load another font\0".as_ptr() as *const c_char);
            }

            f = font_ptr + 1;
            *char_base.offset(f as isize) = fmem_ptr - bc;
            *width_base.offset(f as isize) = *char_base.offset(f as isize) + ec + 1;
            *height_base.offset(f as isize) = *width_base.offset(f as isize) + nw;
            *depth_base.offset(f as isize) = *height_base.offset(f as isize) + nh;
            *italic_base.offset(f as isize) = *depth_base.offset(f as isize) + nd;
            *lig_kern_base.offset(f as isize) = *italic_base.offset(f as isize) + ni;
            *kern_base.offset(f as isize) = *lig_kern_base.offset(f as isize) + nl - 256 * 128;
            *exten_base.offset(f as isize) = *kern_base.offset(f as isize) + 256 * 128 + nk;
            *param_base.offset(f as isize) = *exten_base.offset(f as isize) + ne;

            if lh < 2 {
                is_bad_tfm = true;
                break 'read_tfm;
            }

            a = ttstub_input_getc(tfm_file);
            qw.s3 = a as u16;
            b = ttstub_input_getc(tfm_file);
            qw.s2 = b as u16;
            c = ttstub_input_getc(tfm_file);
            qw.s1 = c as u16;
            d = ttstub_input_getc(tfm_file);
            qw.s0 = d as u16;
            if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                is_bad_tfm = true;
                break 'read_tfm;
            }
            *font_check.offset(f as isize) = qw;

            readfifteen!(z);
            z = z * 256 + ttstub_input_getc(tfm_file);
            z = (z * 16) + (ttstub_input_getc(tfm_file) / 16);
            if z < 65536 {
                is_bad_tfm = true;
                break 'read_tfm;
            }

            while lh > 2 {
                ttstub_input_getc(tfm_file);
                ttstub_input_getc(tfm_file);
                ttstub_input_getc(tfm_file);
                ttstub_input_getc(tfm_file);
                lh -= 1;
            }

            *font_dsize.offset(f as isize) = z;
            if s != -1000 {
                if s >= 0 {
                    z = s;
                } else {
                    z = xn_over_d(z, -s, 1000);
                }
            }

            *font_size.offset(f as isize) = z;

            k = fmem_ptr;
            while k <= *width_base.offset(f as isize) - 1 {
                a = ttstub_input_getc(tfm_file);
                qw.s3 = a as u16;
                b = ttstub_input_getc(tfm_file);
                qw.s2 = b as u16;
                c = ttstub_input_getc(tfm_file);
                qw.s1 = c as u16;
                d = ttstub_input_getc(tfm_file);
                qw.s0 = d as u16;
                if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                (*font_info.offset(k as isize)).b16 = qw;

                if a >= nw || b / 16 >= nh || b % 16 >= nd || c / 4 >= ni {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }

                match c % 4 {
                    1 => {
                        if d >= nl {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }
                    }
                    3 => {
                        if d >= ne {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }
                    }
                    2 => {
                        if d < bc || d > ec {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }

                        let mut not_found = false;
                        while d < k + bc - fmem_ptr {
                            qw = FONT_CHARACTER_INFO!(f, d);
                            if (qw.s1 % 4) != (LIST_TAG as i8) as u16 {
                                not_found = true;
                                break;
                            }
                            d = qw.s0 as i32;
                        }

                        if !not_found && d == k + bc - fmem_ptr {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }
                    }
                    _ => {}
                }
                k += 1;
            }

            alpha = 16;
            while z >= 0x800000 {
                z = z / 2;
                alpha = alpha + alpha;
            }
            beta = (256 / alpha) as u8;
            alpha = alpha * z;

            k = *width_base.offset(f as isize);
            while k <= *lig_kern_base.offset(f as isize) - 1 {
                a = ttstub_input_getc(tfm_file);
                b = ttstub_input_getc(tfm_file);
                c = ttstub_input_getc(tfm_file);
                d = ttstub_input_getc(tfm_file);
                if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                sw = (((((d * z) / 256) + c * z) / 256) + b * z) / (beta as i32);

                if a == 0 {
                    (*font_info.offset(k as isize)).b32.s1 = sw;
                } else if a == 255 {
                    (*font_info.offset(k as isize)).b32.s1 = sw - alpha;
                } else {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                k += 1;
            }

            if (*font_info.offset(*width_base.offset(f as isize) as isize))
                .b32
                .s1
                != 0
            {
                is_bad_tfm = true;
                break 'read_tfm;
            }
            if (*font_info.offset(*height_base.offset(f as isize) as isize))
                .b32
                .s1
                != 0
            {
                is_bad_tfm = true;
                break 'read_tfm;
            }
            if (*font_info.offset(*depth_base.offset(f as isize) as isize))
                .b32
                .s1
                != 0
            {
                is_bad_tfm = true;
                break 'read_tfm;
            }
            if (*font_info.offset(*italic_base.offset(f as isize) as isize))
                .b32
                .s1
                != 0
            {
                is_bad_tfm = true;
                break 'read_tfm;
            }

            bch_label = 32767;
            local_bchar = 256;

            if nl > 0 {
                k = *lig_kern_base.offset(f as isize);
                while k <= *kern_base.offset(f as isize) + 256 * 128 - 1 {
                    a = ttstub_input_getc(tfm_file);
                    qw.s3 = a as u16;
                    b = ttstub_input_getc(tfm_file);
                    qw.s2 = b as u16;
                    c = ttstub_input_getc(tfm_file);
                    qw.s1 = c as u16;
                    d = ttstub_input_getc(tfm_file);
                    qw.s0 = d as u16;
                    if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    (*font_info.offset(k as isize)).b16 = qw;

                    if a > 128 {
                        if 256 * c + d >= nl {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }

                        if a == 255 && k == *lig_kern_base.offset(f as isize) {
                            local_bchar = b as i16;
                        }
                    } else {
                        if b != local_bchar as i32 {
                            if b < bc || b > ec {
                                is_bad_tfm = true;
                                break 'read_tfm;
                            }

                            qw = FONT_CHARACTER_INFO!(f, b);
                            if !(qw.s3 > 0) {
                                is_bad_tfm = true;
                                break 'read_tfm;
                            }
                        }

                        if c < 128 {
                            if d < bc || d > ec {
                                is_bad_tfm = true;
                                break 'read_tfm;
                            }
                            qw = FONT_CHARACTER_INFO!(f, d);
                            if !(qw.s3 > 0) {
                                is_bad_tfm = true;
                                break 'read_tfm;
                            }
                        } else if 256 * (c - 128) + d >= nk {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }

                        if a < 128 && k - *lig_kern_base.offset(f as isize) + a + 1 >= nl {
                            is_bad_tfm = true;
                            break 'read_tfm;
                        }
                    }
                    k += 1;
                }

                if a == 255 {
                    bch_label = 256 * c + d;
                }
            }

            k = *kern_base.offset(f as isize) + 256 * 128;
            while k <= *exten_base.offset(f as isize) - 1 {
                a = ttstub_input_getc(tfm_file);
                b = ttstub_input_getc(tfm_file);
                c = ttstub_input_getc(tfm_file);
                d = ttstub_input_getc(tfm_file);
                if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                sw = (((((d * z) / 256) + c * z) / 256) + b * z) / (beta as i32);

                if a == 0 {
                    (*font_info.offset(k as isize)).b32.s1 = sw;
                } else if a == 255 {
                    (*font_info.offset(k as isize)).b32.s1 = sw - alpha;
                } else {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                k += 1;
            }

            k = *exten_base.offset(f as isize);
            while k <= *param_base.offset(f as isize) - 1 {
                a = ttstub_input_getc(tfm_file);
                qw.s3 = a as u16;
                b = ttstub_input_getc(tfm_file);
                qw.s2 = b as u16;
                c = ttstub_input_getc(tfm_file);
                qw.s1 = c as u16;
                d = ttstub_input_getc(tfm_file);
                qw.s0 = d as u16;
                if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                (*font_info.offset(k as isize)).b16 = qw;

                if a != 0 {
                    if a < bc || a > ec {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    qw = FONT_CHARACTER_INFO!(f, a);
                    if !(qw.s3 > 0) {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                }

                if b != 0 {
                    if b < bc || b > ec {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    qw = FONT_CHARACTER_INFO!(f, b);
                    if !(qw.s3 > 0) {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                }

                if c != 0 {
                    if c < bc || c > ec {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    qw = FONT_CHARACTER_INFO!(f, c);
                    if !(qw.s3 > 0) {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                }

                if d < bc || d > ec {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                qw = FONT_CHARACTER_INFO!(f, d);
                if !(qw.s3 > 0) {
                    is_bad_tfm = true;
                    break 'read_tfm;
                }
                k += 1;
            }

            k = 1;
            while k <= np {
                if k == 1 {
                    sw = ttstub_input_getc(tfm_file);
                    if sw == libc::EOF {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    if sw > 127 {
                        sw = sw - 256;
                    }

                    sw = sw * 256 + ttstub_input_getc(tfm_file);
                    sw = sw * 256 + ttstub_input_getc(tfm_file);
                    (*font_info.offset(*param_base.offset(f as isize) as isize))
                        .b32
                        .s1 = (sw * 16) + (ttstub_input_getc(tfm_file) / 16);
                } else {
                    a = ttstub_input_getc(tfm_file);
                    b = ttstub_input_getc(tfm_file);
                    c = ttstub_input_getc(tfm_file);
                    d = ttstub_input_getc(tfm_file);
                    if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                    sw = (((((d * z) / 256) + c * z) / 256) + b * z) / (beta as i32);

                    if a == 0 {
                        (*font_info.offset((*param_base.offset(f as isize) + k - 1) as isize))
                            .b32
                            .s1 = sw;
                    } else if a == 255 {
                        (*font_info.offset((*param_base.offset(f as isize) + k - 1) as isize))
                            .b32
                            .s1 = sw - alpha;
                    } else {
                        is_bad_tfm = true;
                        break 'read_tfm;
                    }
                }
                k += 1;
            }

            k = np + 1;
            while k <= 7 {
                (*font_info.offset((*param_base.offset(f as isize) + k - 1) as isize))
                    .b32
                    .s1 = 0;
                k += 1;
            }

            if np >= 7 {
                *font_params.offset(f as isize) = np;
            } else {
                *font_params.offset(f as isize) = 7;
            }

            *hyphen_char.offset(f as isize) = INTPAR!(INT_PAR__default_hyphen_char);
            *skew_char.offset(f as isize) = INTPAR!(INT_PAR__default_skew_char);
            if bch_label < nl {
                *bchar_label.offset(f as isize) = bch_label + *lig_kern_base.offset(f as isize);
            } else {
                *bchar_label.offset(f as isize) = NON_ADDRESS;
            }
            *font_bchar.offset(f as isize) = local_bchar as i32;
            *font_false_bchar.offset(f as isize) = local_bchar as i32;

            if local_bchar <= ec as i16 {
                if local_bchar >= bc as i16 {
                    qw = FONT_CHARACTER_INFO!(f, local_bchar);
                    if qw.s3 > 0 {
                        *font_false_bchar.offset(f as isize) = TOO_BIG_CHAR;
                    }
                }
            }

            *font_name.offset(f as isize) = nom;
            *font_area.offset(f as isize) = aire;
            *font_bc.offset(f as isize) = (bc) as u16;
            *font_ec.offset(f as isize) = (ec) as u16;
            *font_glue.offset(f as isize) = TEX_NULL;
            *param_base.offset(f as isize) -= 1;
            fmem_ptr = fmem_ptr + lf;
            font_ptr = f;
            other_font = f;
            *font_mapping.offset(f as isize) = load_tfm_font_mapping();
        }
    }

    if is_bad_tfm {
        if INTPAR!(INT_PAR__suppress_fontnotfound_error) == 0 {
            error_here_with_diagnostic(b"Font \0".as_ptr() as *const c_char);
            sprint_cs(u);
            print_char(b'=' as i32);
            if file_name_quote_char != 0 {
                print_char(file_name_quote_char as i32);
            }
            print_file_name(nom, aire, cur_ext);
            if file_name_quote_char != 0 {
                print_char(file_name_quote_char as i32);
            }
            if s >= 0 {
                print_cstr(b" at \0".as_ptr() as *const c_char);
                print_scaled(s);
                print_cstr(b"pt\0".as_ptr() as *const c_char);
            } else if s != -1000 {
                print_cstr(b" scaled \0".as_ptr() as *const c_char);
                print_int(-s);
            }

            if file_opened {
                print_cstr(b" not loadable: Bad metric (TFM) file\0".as_ptr() as *const c_char);
            } else if name_too_long {
                print_cstr(
                    b" not loadable: Metric (TFM) file name too long\0".as_ptr() as *const c_char
                );
            } else {
                print_cstr(
                    b" not loadable: Metric (TFM) file or installed font not found\0".as_ptr()
                        as *const c_char,
                );
            }
            capture_to_diagnostic(std::ptr::null_mut());

            help_ptr = 5;
            set_help_line!(
                4,
                b"I wasn't able to read the size data for this font,\0".as_ptr() as *const c_char
            );
            set_help_line!(
                3,
                b"so I will ignore the font specification.\0".as_ptr() as *const c_char
            );
            set_help_line!(
                2,
                b"[Wizards can fix TFM files using TFtoPL/PLtoTF.]\0".as_ptr() as *const c_char
            );
            set_help_line!(
                1,
                b"You might try inserting a different font spec;\0".as_ptr() as *const c_char
            );
            set_help_line!(
                0,
                b"e.g., type `I\\font<same font id>=<substitute font name>'.\0".as_ptr()
                    as *const c_char
            );

            error();
        }
    }

    if file_opened {
        ttstub_input_close(tfm_file);
    }

    if INTPAR!(INT_PAR__xetex_tracing_fonts) > 0 {
        if other_font == FONT_BASE || file_opened {
            begin_diagnostic();
            diagnostic_begin_capture_warning_here();

            print_nl_cstr(b" -> \0".as_ptr() as *const c_char);
            if other_font == FONT_BASE {
                print_c_string(b"font not found, using \"nullfont\"\0".as_ptr() as *const c_char);
            } else {
                print_c_string(name_of_file);
            }

            capture_to_diagnostic(std::ptr::null_mut());
            end_diagnostic(false);
        }
    }

    other_font
}

// 24. new_character
#[unsafe(export_name = "new_character")]
pub unsafe extern "C" fn new_character(f: internal_font_number, c: UTF16_code) -> i32 {
    let p: i32;
    let ec: u16;
    if *font_area.offset(f as isize) == (AAT_FONT_FLAG) as i32
        || *font_area.offset(f as isize) == (OTGR_FONT_FLAG) as i32
    {
        return new_native_character(f, c as UnicodeScalar);
    }
    ec = effective_char(false, f, c) as u16;
    if (*font_bc.offset(f as isize) as u32) <= ec as u32 {
        if (*font_ec.offset(f as isize) as u32) >= ec as u32 {
            if FONT_CHARACTER_INFO!(f, ec).s3 > 0 {
                p = get_avail();
                mem!(p).b16.s1 = (f as i16) as u16;
                mem!(p).b16.s0 = (c as i16) as u16;
                return p;
            }
        }
    }
    char_warning(f, c as i32);
    TEX_NULL
}

// 25. scan_spec
#[unsafe(export_name = "scan_spec")]
pub unsafe extern "C" fn scan_spec(c: group_code, three_codes: bool) {
    let mut s: i32 = 0;
    let spec_code: u8;
    if three_codes {
        s = (*save_stack.offset(save_ptr as isize)).b32.s1;
    }
    if scan_keyword(b"to\0".as_ptr() as *const c_char) {
        spec_code = EXACTLY as u8;
        scan_dimen(false, false, false);
    } else if scan_keyword(b"spread\0".as_ptr() as *const c_char) {
        spec_code = ADDITIONAL as u8;
        scan_dimen(false, false, false);
    } else {
        spec_code = ADDITIONAL as u8;
        cur_val = 0;
    }

    if three_codes {
        (*save_stack.offset(save_ptr as isize)).b32.s1 = s;
        save_ptr += 1;
    }
    (*save_stack.offset(save_ptr as isize)).b32.s1 = spec_code as i32;
    (*save_stack.offset((save_ptr + 1) as isize)).b32.s1 = cur_val;
    save_ptr += 2;
    new_save_level(c);
    scan_left_brace();
}

// 26. char_pw
#[unsafe(export_name = "char_pw")]
pub unsafe extern "C" fn char_pw(mut p: i32, side: small_number) -> scaled_t {
    let f: internal_font_number;
    let c: i32;
    if side == 0 {
        last_leftmost_char = TEX_NULL;
    } else {
        last_rightmost_char = TEX_NULL;
    }
    if p == TEX_NULL {
        return 0;
    }
    if p != TEX_NULL
        && !is_char_node(p)
        && NODE_type!(p) == (WHATSIT_NODE as i16) as u16
        && (NODE_subtype!(p) == (NATIVE_WORD_NODE as i16) as u16
            || NODE_subtype!(p) == (NATIVE_WORD_NODE_AT as i16) as u16)
    {
        if !NATIVE_NODE_glyph_info_ptr!(p).is_null() {
            f = NATIVE_NODE_font!(p) as internal_font_number;
            return round_xn_over_d(
                (*font_info.offset((QUAD_CODE + *param_base.offset(f as isize)) as isize))
                    .b32
                    .s1,
                get_native_word_cp!(p, side),
                1000,
            );
        } else {
            return 0;
        }
    }
    if p != TEX_NULL
        && !is_char_node(p)
        && NODE_type!(p) == (WHATSIT_NODE as i16) as u16
        && mem!(p).b16.s0 == (GLYPH_NODE as i16) as u16
    {
        f = mem!(p + 4).b16.s2 as internal_font_number;
        return round_xn_over_d(
            (*font_info.offset((QUAD_CODE + *param_base.offset(f as isize)) as isize))
                .b32
                .s1,
            get_cp_code(
                f as c_int,
                (mem!(p + 4).b16.s1 as c_int) as u32,
                side as c_int,
            ),
            1000,
        );
    }
    if !is_char_node(p) {
        if NODE_type!(p) == (LIGATURE_NODE as i16) as u16 {
            p = p + 1;
        } else {
            return 0;
        }
    }
    f = CHAR_NODE_font!(p) as internal_font_number;
    c = get_cp_code(f as c_int, (mem!(p).b16.s0 as c_int) as u32, side as c_int);
    match side {
        0 => {
            last_leftmost_char = p;
        }
        1 => {
            last_rightmost_char = p;
        }
        _ => {}
    }
    if c == 0 {
        return 0;
    }
    round_xn_over_d(
        (*font_info.offset((QUAD_CODE + *param_base.offset(f as isize)) as isize))
            .b32
            .s1,
        c,
        1000,
    )
}

// 27. new_margin_kern
#[unsafe(export_name = "new_margin_kern")]
pub unsafe extern "C" fn new_margin_kern(w: scaled_t, p: i32, side: small_number) -> i32 {
    let k = get_node(MARGIN_KERN_NODE_SIZE);
    NODE_type!(k) = (MARGIN_KERN_NODE as i16) as u16;
    mem!(k).b16.s0 = (side as i16) as u16;
    mem!(k + 1).b32.s1 = w;
    k
}

// 28. prepare_native_line_layouts
#[unsafe(export_name = "prepare_native_line_layouts")]
pub unsafe extern "C" fn prepare_native_line_layouts(
    lines: *const i32,
    count: usize,
) -> *mut PreparedLayouts {
    let mut requests: *mut NativeLayoutRequest = std::ptr::null_mut();
    let mut request_count: usize = 0;
    let mut capacity: usize = 0;

    for line_index in 0..count {
        let mut p = *lines.add(line_index);
        let mut remaining: i32 = 0;
        while p != TEX_NULL {
            if !is_char_node(p)
                && NODE_type!(p) == (WHATSIT_NODE as i16) as u16
                && (NODE_subtype!(p) == (NATIVE_WORD_NODE as i16) as u16
                    || NODE_subtype!(p) == (NATIVE_WORD_NODE_AT as i16) as u16)
                && *font_area.offset(NATIVE_NODE_font!(p) as isize) == (OTGR_FONT_FLAG) as i32
                && remaining <= 0
            {
                let mut end = LLIST_link!(p);
                let mut cursor: i32;
                let mut length: usize = 0;
                loop {
                    if end == TEX_NULL || is_char_node(end) {
                        break;
                    }
                    cursor = end;
                    if NODE_type!(cursor) == (DISC_NODE as i16) as u16 {
                        cursor = LLIST_link!(cursor);
                    }
                    if cursor == TEX_NULL
                        || is_char_node(cursor)
                        || NODE_type!(cursor) != (WHATSIT_NODE as i16) as u16
                        || (NODE_subtype!(cursor) != (NATIVE_WORD_NODE as i16) as u16
                            && NODE_subtype!(cursor) != (NATIVE_WORD_NODE_AT as i16) as u16)
                        || NATIVE_NODE_font!(cursor) != NATIVE_NODE_font!(p)
                    {
                        break;
                    }
                    end = LLIST_link!(cursor);
                }
                if end != LLIST_link!(p) {
                    cursor = p;
                    while cursor != end {
                        if NODE_type!(cursor) == (WHATSIT_NODE as i16) as u16 {
                            length += NATIVE_NODE_length!(cursor) as usize;
                        }
                        cursor = LLIST_link!(cursor);
                    }
                    let text = xmalloc(length * std::mem::size_of::<u16>()) as *mut u16;
                    length = 0;
                    cursor = p;
                    while cursor != end {
                        if NODE_type!(cursor) == (WHATSIT_NODE as i16) as u16 {
                            let cur_len = NATIVE_NODE_length!(cursor) as usize;
                            libc::memcpy(
                                text.add(length) as *mut c_void,
                                NATIVE_NODE_text!(cursor) as *const c_void,
                                cur_len * std::mem::size_of::<u16>(),
                            );
                            length += cur_len;
                        }
                        cursor = LLIST_link!(cursor);
                    }
                    if request_count == capacity {
                        capacity = if capacity != 0 { 2 * capacity } else { 16 };
                        requests = xrealloc(
                            requests as *mut c_void,
                            capacity * std::mem::size_of::<NativeLayoutRequest>(),
                        ) as *mut NativeLayoutRequest;
                    }
                    (*requests.add(request_count)).engine =
                        *font_layout_engine.offset(NATIVE_NODE_font!(p) as isize);
                    (*requests.add(request_count)).text = text;
                    (*requests.add(request_count)).length = length;
                    request_count += 1;
                }
                p = end;
                continue;
            }
            if !is_char_node(p) && NODE_type!(p) == (DISC_NODE as i16) as u16 {
                remaining = DISCRETIONARY_NODE_replace_count!(p) as i32;
            } else {
                remaining -= 1;
            }
            p = LLIST_link!(p);
        }
    }

    let prepared = ttxl_prepare_layouts(requests, request_count);
    for line_index in 0..request_count {
        libc::free((*requests.add(line_index)).text as *mut c_void);
    }
    libc::free(requests as *mut c_void);
    prepared
}

// 29. hpack
#[unsafe(export_name = "hpack")]
pub unsafe extern "C" fn hpack(mut p: i32, mut w: scaled_t, m: small_number) -> i32 {
    let r: i32;
    let mut q: i32;
    let mut h: scaled_t;
    let mut d: scaled_t;
    let mut x: scaled_t;
    let mut s: scaled_t;
    let mut other_font: i32;
    let mut o: glue_ord;
    let mut f: internal_font_number;
    let mut i: b16x4;
    let mut pp: i32;
    let mut ppp: i32 = TEX_NULL;
    let mut total_chars: i32;
    let mut k: i32;

    last_badness = 0;
    r = get_node(BOX_NODE_SIZE);
    NODE_type!(r) = (HLIST_NODE as i16) as u16;
    mem!(r).b16.s0 = 0;
    mem!(r + 4).b32.s1 = 0;
    q = r + 5;
    mem!(q).b32.s1 = p;
    h = 0;
    d = 0;
    x = 0;
    total_stretch[NORMAL as usize] = 0;
    total_shrink[NORMAL as usize] = 0;
    total_stretch[FIL as usize] = 0;
    total_shrink[FIL as usize] = 0;
    total_stretch[FILL as usize] = 0;
    total_shrink[FILL as usize] = 0;
    total_stretch[FILLL as usize] = 0;
    total_shrink[FILLL as usize] = 0;

    if INTPAR!(INT_PAR__texxet) > 0 {
        temp_ptr = get_avail();
        mem!(temp_ptr).b32.s0 = BEFORE;
        mem!(temp_ptr).b32.s1 = LR_ptr;
        LR_ptr = temp_ptr;
    }

    while p != TEX_NULL {
        loop {
            while is_char_node(p) {
                f = CHAR_NODE_font!(p) as internal_font_number;
                i = FONT_CHARACTER_INFO!(f, effective_char(true, f, CHAR_NODE_character!(p)));
                x = x + FONT_CHARINFO_WIDTH!(f, i);
                s = FONT_CHARINFO_HEIGHT!(f, i);
                if s > h {
                    h = s;
                }
                s = FONT_CHARINFO_DEPTH!(f, i);
                if s > d {
                    d = s;
                }
                p = LLIST_link!(p);
            }

            if p != TEX_NULL {
                match mem!(p).b16.s1 as i32 {
                    0 | 1 | 2 | 13 => {
                        x = x + mem!(p + 1).b32.s1;
                        if NODE_type!(p) >= (RULE_NODE as i16) as u16 {
                            s = 0;
                        } else {
                            s = mem!(p + 4).b32.s1;
                        }
                        if mem!(p + 3).b32.s1 - s > h {
                            h = mem!(p + 3).b32.s1 - s;
                        }
                        if mem!(p + 2).b32.s1 + s > d {
                            d = mem!(p + 2).b32.s1 + s;
                        }
                    }
                    3 | 4 | 5 => {
                        if adjust_tail != TEX_NULL || pre_adjust_tail != TEX_NULL {
                            while mem!(q).b32.s1 != p {
                                q = LLIST_link!(q);
                            }
                            if NODE_type!(p) == (ADJUST_NODE as i16) as u16 {
                                if mem!(p).b16.s0 != 0 {
                                    if pre_adjust_tail == TEX_NULL {
                                        confusion(b"pre vadjust\0".as_ptr() as *const c_char);
                                    }
                                    mem!(pre_adjust_tail).b32.s1 = mem!(p + 1).b32.s1;
                                    while mem!(pre_adjust_tail).b32.s1 != TEX_NULL {
                                        pre_adjust_tail = LLIST_link!(pre_adjust_tail);
                                    }
                                } else {
                                    if adjust_tail == TEX_NULL {
                                        confusion(b"pre vadjust\0".as_ptr() as *const c_char);
                                    }
                                    mem!(adjust_tail).b32.s1 = mem!(p + 1).b32.s1;
                                    while mem!(adjust_tail).b32.s1 != TEX_NULL {
                                        adjust_tail = LLIST_link!(adjust_tail);
                                    }
                                }
                                p = LLIST_link!(p);
                                free_node(mem!(q).b32.s1, SMALL_NODE_SIZE);
                            } else {
                                mem!(adjust_tail).b32.s1 = p;
                                adjust_tail = p;
                                p = LLIST_link!(p);
                            }
                            mem!(q).b32.s1 = p;
                            p = q;
                        }
                    }
                    8 => match mem!(p).b16.s0 as i32 {
                        40 | 41 => {
                            if q != r + 5 && NODE_type!(q) == (DISC_NODE as i16) as u16 {
                                k = mem!(q).b16.s0 as i32;
                            } else {
                                k = 0;
                            }
                            while mem!(q).b32.s1 != p {
                                k -= 1;
                                q = LLIST_link!(q);
                                if NODE_type!(q) == (DISC_NODE as i16) as u16 {
                                    k = mem!(q).b16.s0 as i32;
                                }
                            }
                            pp = mem!(p).b32.s1;

                            while k <= 0 && pp != TEX_NULL && !is_char_node(pp) {
                                if NODE_type!(pp) == (WHATSIT_NODE as i16) as u16
                                    && (mem!(pp).b16.s0 == (NATIVE_WORD_NODE as i16) as u16
                                        || mem!(pp).b16.s0 == (NATIVE_WORD_NODE_AT as i16) as u16)
                                    && mem!(pp + 4).b16.s2 == mem!(p + 4).b16.s2
                                {
                                    pp = LLIST_link!(pp);
                                } else if NODE_type!(pp) == (DISC_NODE as i16) as u16 {
                                    ppp = mem!(pp).b32.s1;
                                    if ppp != TEX_NULL
                                        && !is_char_node(ppp)
                                        && NODE_type!(ppp) == (WHATSIT_NODE as i16) as u16
                                        && (mem!(ppp).b16.s0 == (NATIVE_WORD_NODE as i16) as u16
                                            || mem!(ppp).b16.s0
                                                == (NATIVE_WORD_NODE_AT as i16) as u16)
                                        && mem!(ppp + 4).b16.s2 == mem!(p + 4).b16.s2
                                    {
                                        pp = mem!(ppp).b32.s1;
                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            }

                            if pp != mem!(p).b32.s1 {
                                total_chars = 0;
                                p = mem!(q).b32.s1;
                                while p != pp {
                                    if NODE_type!(p) == (WHATSIT_NODE as i16) as u16 {
                                        total_chars = total_chars + mem!(p + 4).b16.s1 as i32;
                                    }
                                    ppp = p;
                                    p = LLIST_link!(p);
                                }
                                p = mem!(q).b32.s1;
                                pp = new_native_word_node(
                                    mem!(p + 4).b16.s2 as internal_font_number,
                                    total_chars,
                                );
                                mem!(pp).b16.s0 = mem!(p).b16.s0;
                                mem!(q).b32.s1 = pp;
                                mem!(pp).b32.s1 = mem!(ppp).b32.s1;
                                mem!(ppp).b32.s1 = TEX_NULL;
                                total_chars = 0;
                                ppp = p;
                                loop {
                                    if NODE_type!(ppp) == (WHATSIT_NODE as i16) as u16 {
                                        let for_end = mem!(ppp + 4).b16.s1 as i32 - 1;
                                        k = 0;
                                        while k <= for_end {
                                            *NATIVE_NODE_text!(pp).offset(total_chars as isize) =
                                                *NATIVE_NODE_text!(ppp).offset(k as isize);
                                            total_chars += 1;
                                            k += 1;
                                        }
                                    }
                                    ppp = LLIST_link!(ppp);
                                    if ppp == TEX_NULL {
                                        break;
                                    }
                                }
                                flush_node_list(p);
                                p = mem!(q).b32.s1;
                                set_native_metrics(
                                    p,
                                    INTPAR!(INT_PAR__xetex_use_glyph_metrics) > 0,
                                );
                            }
                            if mem!(p + 3).b32.s1 > h {
                                h = mem!(p + 3).b32.s1;
                            }
                            if mem!(p + 2).b32.s1 > d {
                                d = mem!(p + 2).b32.s1;
                            }
                            x = x + mem!(p + 1).b32.s1;
                        }
                        42 | 43 | 44 => {
                            if mem!(p + 3).b32.s1 > h {
                                h = mem!(p + 3).b32.s1;
                            }
                            if mem!(p + 2).b32.s1 > d {
                                d = mem!(p + 2).b32.s1;
                            }
                            x = x + mem!(p + 1).b32.s1;
                        }
                        _ => {}
                    },
                    10 => {
                        other_font = mem!(p + 1).b32.s0;
                        x = x + mem!(other_font + 1).b32.s1;
                        o = mem!(other_font).b16.s1 as glue_ord;
                        total_stretch[o as usize] =
                            total_stretch[o as usize] + mem!(other_font + 2).b32.s1;
                        o = mem!(other_font).b16.s0 as glue_ord;
                        total_shrink[o as usize] =
                            total_shrink[o as usize] + mem!(other_font + 3).b32.s1;
                        if mem!(p).b16.s0 >= (A_LEADERS as i16) as u16 {
                            other_font = mem!(p + 1).b32.s1;
                            if mem!(other_font + 3).b32.s1 > h {
                                h = mem!(other_font + 3).b32.s1;
                            }
                            if mem!(other_font + 2).b32.s1 > d {
                                d = mem!(other_font + 2).b32.s1;
                            }
                        }
                    }
                    11 | 40 => {
                        x = x + mem!(p + 1).b32.s1;
                    }
                    9 => {
                        x = x + mem!(p + 1).b32.s1;
                        if INTPAR!(INT_PAR__texxet) > 0 {
                            if odd(mem!(p).b16.s0 as i32) {
                                if mem!(LR_ptr).b32.s0
                                    == (L_CODE * (mem!(p).b16.s0 as i32 / L_CODE) + 3)
                                {
                                    temp_ptr = LR_ptr;
                                    LR_ptr = mem!(temp_ptr).b32.s1;
                                    mem!(temp_ptr).b32.s1 = avail;
                                    avail = temp_ptr;
                                } else {
                                    LR_problems += 1;
                                    NODE_type!(p) = (KERN_NODE as i16) as u16;
                                    NODE_subtype!(p) = (EXPLICIT as i16) as u16;
                                }
                            } else {
                                temp_ptr = get_avail();
                                mem!(temp_ptr).b32.s0 =
                                    L_CODE * (mem!(p).b16.s0 as i32 / L_CODE) + 3;
                                mem!(temp_ptr).b32.s1 = LR_ptr;
                                LR_ptr = temp_ptr;
                            }
                        }
                    }
                    6 => {
                        mem!(GARBAGE) = mem!(p + 1);
                        mem!(GARBAGE).b32.s1 = mem!(p).b32.s1;
                        p = GARBAGE;
                        xtx_ligature_present = true;
                        continue;
                    }
                    _ => {}
                }
                p = LLIST_link!(p);
            }
            break;
        }
    }

    if adjust_tail != TEX_NULL {
        mem!(adjust_tail).b32.s1 = TEX_NULL;
    }
    if pre_adjust_tail != TEX_NULL {
        mem!(pre_adjust_tail).b32.s1 = TEX_NULL;
    }
    mem!(r + 3).b32.s1 = h;
    mem!(r + 2).b32.s1 = d;
    if m as i32 == ADDITIONAL {
        w = x + w;
    }
    mem!(r + 1).b32.s1 = w;
    x = w - x;

    let do_common_ending = |r: i32| {
        if output_active {
            print_cstr(b") has occurred while \\output is active\0".as_ptr() as *const c_char);
        } else {
            if pack_begin_line != 0 {
                if pack_begin_line > 0 {
                    print_cstr(b") in paragraph at lines \0".as_ptr() as *const c_char);
                } else {
                    print_cstr(b") in alignment at lines \0".as_ptr() as *const c_char);
                }
                print_int(pack_begin_line.abs());
                print_cstr(b"--\0".as_ptr() as *const c_char);
            } else {
                print_cstr(b") detected at line \0".as_ptr() as *const c_char);
            }
            print_int(line);
        }

        capture_to_diagnostic(std::ptr::null_mut());

        print_ln();
        font_in_short_display = FONT_BASE;
        short_display(mem!(r + 5).b32.s1);
        print_ln();
        begin_diagnostic();
        show_box(r);
        end_diagnostic(true);
    };

    'check_box: {
        if x == 0 {
            mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
            mem!(r + 5).b16.s0 = (NORMAL as i16) as u16;
            BOX_glue_set!(r) = 0.0;
            break 'check_box;
        } else if x > 0 {
            if total_stretch[FILLL as usize] != 0 {
                o = (FILLL) as u8;
            } else if total_stretch[FILL as usize] != 0 {
                o = (FILL) as u8;
            } else if total_stretch[FIL as usize] != 0 {
                o = (FIL) as u8;
            } else {
                o = 0;
            }
            mem!(r + 5).b16.s0 = (o as i16) as u16;
            mem!(r + 5).b16.s1 = (STRETCHING as i16) as u16;
            if total_stretch[o as usize] != 0 {
                BOX_glue_set!(r) = (x as f64) / (total_stretch[o as usize] as f64);
            } else {
                mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
                BOX_glue_set!(r) = 0.0;
            }

            if o == (NORMAL) as u8 {
                if mem!(r + 5).b32.s1 != TEX_NULL {
                    last_badness = badness(x, total_stretch[NORMAL as usize]);
                    if last_badness > INTPAR!(INT_PAR__hbadness) {
                        print_ln();
                        diagnostic_begin_capture_warning_here();
                        if last_badness > 100 {
                            print_nl_cstr(b"Underfull\0".as_ptr() as *const c_char);
                        } else {
                            print_nl_cstr(b"Loose\0".as_ptr() as *const c_char);
                        }
                        print_cstr(b" \\hbox (badness \0".as_ptr() as *const c_char);
                        print_int(last_badness);
                        do_common_ending(r);
                    }
                }
            }
            break 'check_box;
        } else {
            if total_shrink[FILLL as usize] != 0 {
                o = (FILLL) as u8;
            } else if total_shrink[FILL as usize] != 0 {
                o = (FILL) as u8;
            } else if total_shrink[FIL as usize] != 0 {
                o = (FIL) as u8;
            } else {
                o = 0;
            }
            mem!(r + 5).b16.s0 = (o as i16) as u16;
            mem!(r + 5).b16.s1 = (SHRINKING as i16) as u16;
            if total_shrink[o as usize] != 0 {
                BOX_glue_set!(r) = ((-x) as f64) / (total_shrink[o as usize] as f64);
            } else {
                mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
                BOX_glue_set!(r) = 0.0;
            }
            if total_shrink[o as usize] < -x
                && o == (NORMAL) as u8
                && mem!(r + 5).b32.s1 != TEX_NULL
            {
                last_badness = 1000000;
                BOX_glue_set!(r) = 1.0;
                if -x - total_shrink[NORMAL as usize] > DIMENPAR!(DIMEN_PAR__hfuzz)
                    || INTPAR!(INT_PAR__hbadness) < 100
                {
                    if DIMENPAR!(DIMEN_PAR__overfull_rule) > 0
                        && -x - total_shrink[NORMAL as usize] > DIMENPAR!(DIMEN_PAR__hfuzz)
                    {
                        while mem!(q).b32.s1 != TEX_NULL {
                            q = LLIST_link!(q);
                        }
                        mem!(q).b32.s1 = new_rule();
                        mem!(mem!(q).b32.s1 + 1).b32.s1 = DIMENPAR!(DIMEN_PAR__overfull_rule);
                    }
                    print_ln();
                    diagnostic_begin_capture_warning_here();
                    print_nl_cstr(b"Overfull \\hbox (\0".as_ptr() as *const c_char);
                    print_scaled(-x - total_shrink[NORMAL as usize]);
                    print_cstr(b"pt too wide\0".as_ptr() as *const c_char);
                    do_common_ending(r);
                }
            } else if o == (NORMAL) as u8 {
                if mem!(r + 5).b32.s1 != TEX_NULL {
                    last_badness = badness(-x, total_shrink[NORMAL as usize]);
                    if last_badness > INTPAR!(INT_PAR__hbadness) {
                        print_ln();
                        diagnostic_begin_capture_warning_here();
                        print_nl_cstr(b"Tight \\hbox (badness \0".as_ptr() as *const c_char);
                        print_int(last_badness);
                        do_common_ending(r);
                    }
                }
            }
            break 'check_box;
        }
    }

    if INTPAR!(INT_PAR__texxet) > 0 {
        if mem!(LR_ptr).b32.s0 != BEFORE {
            while mem!(q).b32.s1 != TEX_NULL {
                q = LLIST_link!(q);
            }
            loop {
                temp_ptr = q;
                q = new_math(0, (mem!(LR_ptr).b32.s0) as i16);
                mem!(temp_ptr).b32.s1 = q;
                LR_problems = LR_problems + 10000;
                temp_ptr = LR_ptr;
                LR_ptr = mem!(temp_ptr).b32.s1;
                mem!(temp_ptr).b32.s1 = avail;
                avail = temp_ptr;
                if mem!(LR_ptr).b32.s0 == BEFORE {
                    break;
                }
            }
        }
        if LR_problems > 0 {
            print_ln();
            diagnostic_begin_capture_warning_here();
            print_nl_cstr(b"\\endL or \\endR problem (\0".as_ptr() as *const c_char);
            print_int(LR_problems / 10000);
            print_cstr(b" missing, \0".as_ptr() as *const c_char);
            print_int(LR_problems % 10000);
            print_cstr(b" extra\0".as_ptr() as *const c_char);
            LR_problems = 0;
            do_common_ending(r);
        }
        temp_ptr = LR_ptr;
        LR_ptr = mem!(temp_ptr).b32.s1;
        mem!(temp_ptr).b32.s1 = avail;
        avail = temp_ptr;
        if LR_ptr != TEX_NULL {
            confusion(b"LR1\0".as_ptr() as *const c_char);
        }
    }

    r
}

// 30. vpackage
#[unsafe(export_name = "vpackage")]
pub unsafe extern "C" fn vpackage(
    mut p: i32,
    mut h: scaled_t,
    m: small_number,
    l: scaled_t,
) -> i32 {
    let r: i32;
    let mut w: scaled_t;
    let mut d: scaled_t;
    let mut x: scaled_t;
    let mut s: scaled_t;
    let mut other_font: i32;
    let mut o: glue_ord;

    last_badness = 0;
    r = get_node(BOX_NODE_SIZE);
    NODE_type!(r) = (VLIST_NODE as i16) as u16;
    if INTPAR!(INT_PAR__xetex_upwards) > 0 {
        mem!(r).b16.s0 = 1;
    } else {
        mem!(r).b16.s0 = 0;
    }
    mem!(r + 4).b32.s1 = 0;
    mem!(r + 5).b32.s1 = p;
    w = 0;
    d = 0;
    x = 0;
    total_stretch[NORMAL as usize] = 0;
    total_shrink[NORMAL as usize] = 0;
    total_stretch[FIL as usize] = 0;
    total_shrink[FIL as usize] = 0;
    total_stretch[FILL as usize] = 0;
    total_shrink[FILL as usize] = 0;
    total_stretch[FILLL as usize] = 0;
    total_shrink[FILLL as usize] = 0;

    while p != TEX_NULL {
        if is_char_node(p) {
            confusion(b"vpack\0".as_ptr() as *const c_char);
        } else {
            match mem!(p).b16.s1 as i32 {
                0 | 1 | 2 | 13 => {
                    x = x + d + mem!(p + 3).b32.s1;
                    d = mem!(p + 2).b32.s1;
                    if NODE_type!(p) >= (RULE_NODE as i16) as u16 {
                        s = 0;
                    } else {
                        s = mem!(p + 4).b32.s1;
                    }
                    if mem!(p + 1).b32.s1 + s > w {
                        w = mem!(p + 1).b32.s1 + s;
                    }
                }
                8 => {
                    if mem!(p).b16.s0 == (PIC_NODE as i16) as u16
                        || mem!(p).b16.s0 == (PDF_NODE as i16) as u16
                    {
                        x = x + d + mem!(p + 3).b32.s1;
                        d = mem!(p + 2).b32.s1;
                        if mem!(p + 1).b32.s1 > w {
                            w = mem!(p + 1).b32.s1;
                        }
                    }
                }
                10 => {
                    x = x + d;
                    d = 0;
                    other_font = mem!(p + 1).b32.s0;
                    x = x + mem!(other_font + 1).b32.s1;
                    o = mem!(other_font).b16.s1 as glue_ord;
                    total_stretch[o as usize] =
                        total_stretch[o as usize] + mem!(other_font + 2).b32.s1;
                    o = mem!(other_font).b16.s0 as glue_ord;
                    total_shrink[o as usize] =
                        total_shrink[o as usize] + mem!(other_font + 3).b32.s1;
                    if mem!(p).b16.s0 >= (A_LEADERS as i16) as u16 {
                        other_font = mem!(p + 1).b32.s1;
                        if mem!(other_font + 1).b32.s1 > w {
                            w = mem!(other_font + 1).b32.s1;
                        }
                    }
                }
                11 => {
                    x = x + d + mem!(p + 1).b32.s1;
                    d = 0;
                }
                _ => {}
            }
        }
        p = LLIST_link!(p);
    }

    mem!(r + 1).b32.s1 = w;
    if d > l {
        x = x + d - l;
        mem!(r + 2).b32.s1 = l;
    } else {
        mem!(r + 2).b32.s1 = d;
    }
    if m as i32 == ADDITIONAL {
        h = x + h;
    }
    mem!(r + 3).b32.s1 = h;
    x = h - x;

    let do_common_ending = |r: i32| {
        if output_active {
            print_cstr(b") has occurred while \\output is active\0".as_ptr() as *const c_char);
        } else {
            if pack_begin_line != 0 {
                print_cstr(b") in alignment at lines \0".as_ptr() as *const c_char);
                print_int(pack_begin_line.abs());
                print_cstr(b"--\0".as_ptr() as *const c_char);
            } else {
                print_cstr(b") detected at line \0".as_ptr() as *const c_char);
            }
            print_int(line);
            print_ln();
        }

        capture_to_diagnostic(std::ptr::null_mut());

        begin_diagnostic();
        show_box(r);
        end_diagnostic(true);
    };

    if x == 0 {
        mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
        mem!(r + 5).b16.s0 = (NORMAL as i16) as u16;
        BOX_glue_set!(r) = 0.0;
        return r;
    } else if x > 0 {
        if total_stretch[FILLL as usize] != 0 {
            o = (FILLL) as u8;
        } else if total_stretch[FILL as usize] != 0 {
            o = (FILL) as u8;
        } else if total_stretch[FIL as usize] != 0 {
            o = (FIL) as u8;
        } else {
            o = 0;
        }
        mem!(r + 5).b16.s0 = (o as i16) as u16;
        mem!(r + 5).b16.s1 = (STRETCHING as i16) as u16;
        if total_stretch[o as usize] != 0 {
            BOX_glue_set!(r) = (x as f64) / (total_stretch[o as usize] as f64);
        } else {
            mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
            BOX_glue_set!(r) = 0.0;
        }
        if o == (NORMAL) as u8 {
            if mem!(r + 5).b32.s1 != TEX_NULL {
                last_badness = badness(x, total_stretch[NORMAL as usize]);
                if last_badness > INTPAR!(INT_PAR__vbadness) {
                    print_ln();
                    diagnostic_begin_capture_warning_here();
                    if last_badness > 100 {
                        print_nl_cstr(b"Underfull\0".as_ptr() as *const c_char);
                    } else {
                        print_nl_cstr(b"Loose\0".as_ptr() as *const c_char);
                    }
                    print_cstr(b" \\vbox (badness \0".as_ptr() as *const c_char);
                    print_int(last_badness);
                    do_common_ending(r);
                }
            }
        }
        return r;
    } else {
        if total_shrink[FILLL as usize] != 0 {
            o = (FILLL) as u8;
        } else if total_shrink[FILL as usize] != 0 {
            o = (FILL) as u8;
        } else if total_shrink[FIL as usize] != 0 {
            o = (FIL) as u8;
        } else {
            o = 0;
        }
        mem!(r + 5).b16.s0 = (o as i16) as u16;
        mem!(r + 5).b16.s1 = (SHRINKING as i16) as u16;
        if total_shrink[o as usize] != 0 {
            BOX_glue_set!(r) = ((-x) as f64) / (total_shrink[o as usize] as f64);
        } else {
            mem!(r + 5).b16.s1 = (NORMAL as i16) as u16;
            BOX_glue_set!(r) = 0.0;
        }
        if total_shrink[o as usize] < -x && o == (NORMAL) as u8 && mem!(r + 5).b32.s1 != TEX_NULL {
            last_badness = 1000000;
            BOX_glue_set!(r) = 1.0;
            if -x - total_shrink[NORMAL as usize] > DIMENPAR!(DIMEN_PAR__vfuzz)
                || INTPAR!(INT_PAR__vbadness) < 100
            {
                print_ln();
                diagnostic_begin_capture_warning_here();
                print_nl_cstr(b"Overfull \\vbox (\0".as_ptr() as *const c_char);
                print_scaled(-x - total_shrink[NORMAL as usize]);
                print_cstr(b"pt too high\0".as_ptr() as *const c_char);
                do_common_ending(r);
            }
        } else if o == (NORMAL) as u8 {
            if mem!(r + 5).b32.s1 != TEX_NULL {
                last_badness = badness(-x, total_shrink[NORMAL as usize]);
                if last_badness > INTPAR!(INT_PAR__vbadness) {
                    print_ln();
                    diagnostic_begin_capture_warning_here();
                    print_nl_cstr(b"Tight \\vbox (badness \0".as_ptr() as *const c_char);
                    print_int(last_badness);
                    do_common_ending(r);
                }
            }
        }
        return r;
    }
}
