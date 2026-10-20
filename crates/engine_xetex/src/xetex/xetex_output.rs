#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use super::bridge::ttstub_output_putc;
use libc::{c_char, c_int, c_void};
use std::ptr;
use tectonic_bridge_core::{Diagnostic, ttbc_diag_append, ttbc_diag_begin_error, ttbc_diag_begin_warning};

use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_texmfmp::gettexstring;
use super::xetex_xetexd::*;

unsafe extern "C" {
    fn ttstub_diag_finish(diag: *mut Diagnostic);
}

static mut current_diagnostic: *mut Diagnostic = ptr::null_mut();

#[inline]
fn odd(x: i32) -> bool {
    (x & 1) != 0
}

#[unsafe(export_name = "capture_to_diagnostic")]
pub unsafe extern "C" fn capture_to_diagnostic(diagnostic: *mut Diagnostic) {
    unsafe {
        if !current_diagnostic.is_null() {
            ttstub_diag_finish(current_diagnostic);
        }
        current_diagnostic = diagnostic;
    }
}

unsafe fn diagnostic_print_file_line(diagnostic: *mut Diagnostic) {
    unsafe {
        let mut level = in_open;
        while level > 0 && (*full_source_filename_stack.add(level as usize)) == 0 {
            level -= 1;
        }

        if level == 0 {
            ttbc_diag_append(&mut *diagnostic, b"!\0".as_ptr() as *const c_char);
        } else {
            let source_line = if level != in_open { *line_stack.add((level + 1) as usize) } else { line };

            let filename = gettexstring(*full_source_filename_stack.add(level as usize));
            let mut buf = [0u8; 1024];
            let c_str = std::ffi::CStr::from_ptr(filename).to_bytes();
            let formatted = format!("{}:{}: \0", String::from_utf8_lossy(c_str), source_line);
            ttbc_diag_append(&mut *diagnostic, formatted.as_ptr() as *const c_char);
            libc::free(filename as *mut c_void);
        }
    }
}

#[unsafe(export_name = "diagnostic_begin_capture_warning_here")]
pub unsafe extern "C" fn diagnostic_begin_capture_warning_here() -> *mut Diagnostic {
    unsafe {
        let warning = ttbc_diag_begin_warning();
        diagnostic_print_file_line(warning);
        capture_to_diagnostic(warning);
        warning
    }
}

#[unsafe(export_name = "error_here_with_diagnostic")]
pub unsafe extern "C" fn error_here_with_diagnostic(message: *const c_char) -> *mut Diagnostic {
    unsafe {
        let error = ttbc_diag_begin_error();
        diagnostic_print_file_line(error);
        ttbc_diag_append(&mut *error, message);

        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! \0".as_ptr() as *const c_char);
        }
        print_cstr(message);

        capture_to_diagnostic(error);
        error
    }
}

unsafe fn warn_char(c: c_int) {
    unsafe {
        if !current_diagnostic.is_null() {
            let bytes = [c as u8 as c_char, 0];
            ttbc_diag_append(&mut *current_diagnostic, bytes.as_ptr());
        }
    }
}

#[unsafe(export_name = "print_ln")]
pub unsafe extern "C" fn print_ln() {
    unsafe {
        match selector {
            SELECTOR_TERM_AND_LOG => {
                warn_char(b'\n' as c_int);
                ttstub_output_putc(rust_stdout, b'\n' as c_int);
                ttstub_output_putc(log_file, b'\n' as c_int);
                term_offset = 0;
                file_offset = 0;
            }
            SELECTOR_LOG_ONLY => {
                warn_char(b'\n' as c_int);
                ttstub_output_putc(log_file, b'\n' as c_int);
                file_offset = 0;
            }
            SELECTOR_TERM_ONLY => {
                warn_char(b'\n' as c_int);
                ttstub_output_putc(rust_stdout, b'\n' as c_int);
                term_offset = 0;
            }
            SELECTOR_NO_PRINT | SELECTOR_PSEUDO | SELECTOR_NEW_STRING => {}
            _ => {
                ttstub_output_putc(write_file[selector as usize], b'\n' as c_int);
            }
        }
    }
}

#[unsafe(export_name = "print_raw_char")]
pub unsafe extern "C" fn print_raw_char(s: u16, incr_offset: bool) {
    unsafe {
        let ch = s as c_int;
        match selector {
            SELECTOR_TERM_AND_LOG => {
                warn_char(ch);
                ttstub_output_putc(rust_stdout, ch);
                ttstub_output_putc(log_file, ch);
                if incr_offset {
                    term_offset += 1;
                    file_offset += 1;
                }
                if term_offset == max_print_line {
                    ttstub_output_putc(rust_stdout, b'\n' as c_int);
                    term_offset = 0;
                }
                if file_offset == max_print_line {
                    ttstub_output_putc(log_file, b'\n' as c_int);
                    file_offset = 0;
                }
            }
            SELECTOR_LOG_ONLY => {
                warn_char(ch);
                ttstub_output_putc(log_file, ch);
                if incr_offset {
                    file_offset += 1;
                }
                if file_offset == max_print_line {
                    ttstub_output_putc(log_file, b'\n' as c_int);
                    file_offset = 0;
                }
            }
            SELECTOR_TERM_ONLY => {
                warn_char(ch);
                ttstub_output_putc(rust_stdout, ch);
                if incr_offset {
                    term_offset += 1;
                }
                if term_offset == max_print_line {
                    ttstub_output_putc(rust_stdout, b'\n' as c_int);
                    term_offset = 0;
                }
            }
            SELECTOR_NO_PRINT => {}
            SELECTOR_PSEUDO => {
                if tally < trick_count {
                    trick_buf[(tally % error_line) as usize] = s;
                }
            }
            SELECTOR_NEW_STRING => {
                if pool_ptr < pool_size {
                    (*str_pool.add(pool_ptr as usize)) = s;
                    pool_ptr += 1;
                }
            }
            _ => {
                ttstub_output_putc(write_file[selector as usize], ch);
            }
        }
        tally += 1;
    }
}

#[unsafe(export_name = "print_char")]
pub unsafe extern "C" fn print_char(s: i32) {
    unsafe {
        let new_line_c = (*eqtb.add((INT_BASE + INT_PAR__new_line_char) as usize)).b32.s1;

        if selector > SELECTOR_PSEUDO && !doing_special {
            if s >= 0x10000 {
                print_raw_char((0xD800 + (s - 0x10000) / 1024) as u16, true);
                print_raw_char((0xDC00 + (s - 0x10000) % 1024) as u16, true);
            } else {
                print_raw_char(s as u16, true);
            }
            return;
        }

        if s == new_line_c {
            if selector < SELECTOR_PSEUDO {
                print_ln();
                return;
            }
        }

        if s < 32 && !doing_special {
            print_raw_char(b'^' as u16, true);
            print_raw_char(b'^' as u16, true);
            print_raw_char((s + 64) as u16, true);
        } else if s < 127 {
            print_raw_char(s as u16, true);
        } else if s == 127 {
            if !doing_special {
                print_raw_char(b'^' as u16, true);
                print_raw_char(b'^' as u16, true);
                print_raw_char(b'?' as u16, true);
            } else {
                print_raw_char(s as u16, true);
            }
        } else if s < 160 && !doing_special {
            print_raw_char(b'^' as u16, true);
            print_raw_char(b'^' as u16, true);

            let l1 = (s % 256) / 16;
            if l1 < 10 {
                print_raw_char((b'0' + l1 as u8) as u16, true);
            } else {
                print_raw_char((b'a' + (l1 - 10) as u8) as u16, true);
            }

            let l2 = s % 16;
            if l2 < 10 {
                print_raw_char((b'0' + l2 as u8) as u16, true);
            } else {
                print_raw_char((b'a' + (l2 - 10) as u8) as u16, true);
            }
        } else if selector == SELECTOR_PSEUDO {
            print_raw_char(s as u16, true);
        } else if s < 2048 {
            print_raw_char((192 + s / 64) as u16, false);
            print_raw_char((128 + s % 64) as u16, true);
        } else if s < 0x10000 {
            print_raw_char((224 + s / 4096) as u16, false);
            print_raw_char((128 + (s % 4096) / 64) as u16, false);
            print_raw_char((128 + s % 64) as u16, true);
        } else {
            print_raw_char((240 + s / 0x40000) as u16, false);
            print_raw_char((128 + (s % 0x40000) / 4096) as u16, false);
            print_raw_char((128 + (s % 4096) / 64) as u16, false);
            print_raw_char((128 + s % 64) as u16, true);
        }
    }
}

#[unsafe(export_name = "print")]
pub unsafe extern "C" fn print(s: i32) {
    unsafe {
        if s >= str_ptr {
            return print_cstr(b"???\0".as_ptr() as *const c_char);
        } else if s <= BIGGEST_CHAR as i32 {
            if s < 0 {
                return print_cstr(b"???\0".as_ptr() as *const c_char);
            } else {
                if selector > SELECTOR_PSEUDO {
                    print_char(s);
                    return;
                }

                let new_line_c = (*eqtb.add((INT_BASE + INT_PAR__new_line_char) as usize)).b32.s1;
                if s == new_line_c {
                    if selector < SELECTOR_PSEUDO {
                        print_ln();
                        return;
                    }
                }

                let nl = (*eqtb.add((INT_BASE + INT_PAR__new_line_char) as usize)).b32.s1;
                (*eqtb.add((INT_BASE + INT_PAR__new_line_char) as usize)).b32.s1 = -1;
                print_char(s);
                (*eqtb.add((INT_BASE + INT_PAR__new_line_char) as usize)).b32.s1 = nl;
                return;
            }
        }

        let pool_idx = (s - 0x10000) as usize;
        let mut i = *str_start.add(pool_idx);
        let end = *str_start.add(pool_idx + 1);

        while i < end {
            let ch = (*str_pool.add(i as usize)) as i32;
            if (0xD800..0xDC00).contains(&ch) && (i + 1 < end) && (0xDC00..0xE000).contains(&((*str_pool.add((i + 1) as usize)) as i32)) {
                let ch2 = (*str_pool.add((i + 1) as usize)) as i32;
                print_char(0x10000 + (ch - 0xD800) * 1024 + ch2 - 0xDC00);
                i += 2;
            } else {
                print_char(ch);
                i += 1;
            }
        }
    }
}

#[unsafe(export_name = "print_cstr")]
pub unsafe extern "C" fn print_cstr(str: *const c_char) {
    unsafe {
        let len = libc::strlen(str);
        for i in 0..len {
            print_char(*str.add(i) as u8 as i32);
        }
    }
}

#[unsafe(export_name = "print_nl")]
pub unsafe extern "C" fn print_nl(s: str_number) {
    unsafe {
        if (term_offset > 0 && odd(selector)) || (file_offset > 0 && selector >= SELECTOR_LOG_ONLY) {
            print_ln();
        }
        print(s);
    }
}

#[unsafe(export_name = "print_nl_cstr")]
pub unsafe extern "C" fn print_nl_cstr(str: *const c_char) {
    unsafe {
        if (term_offset > 0 && odd(selector)) || (file_offset > 0 && selector >= SELECTOR_LOG_ONLY) {
            print_ln();
        }
        print_cstr(str);
    }
}

#[unsafe(export_name = "print_esc")]
pub unsafe extern "C" fn print_esc(s: str_number) {
    unsafe {
        let c = (*eqtb.add((INT_BASE + INT_PAR__escape_char) as usize)).b32.s1;
        if (0..=BIGGEST_USV).contains(&c) {
            print_char(c);
        }
        print(s);
    }
}

#[unsafe(export_name = "print_esc_cstr")]
pub unsafe extern "C" fn print_esc_cstr(s: *const c_char) {
    unsafe {
        let c = (*eqtb.add((INT_BASE + INT_PAR__escape_char) as usize)).b32.s1;
        if (0..=BIGGEST_USV).contains(&c) {
            print_char(c);
        }
        print_cstr(s);
    }
}

unsafe fn print_the_digs(mut k: u8) {
    unsafe {
        while k > 0 {
            k -= 1;
            if dig[k as usize] < 10 {
                print_char((b'0' + dig[k as usize]) as i32);
            } else {
                print_char((55 + dig[k as usize]) as i32);
            }
        }
    }
}

#[unsafe(export_name = "print_int")]
pub unsafe extern "C" fn print_int(n: i32) {
    unsafe {
        let mut decimal_buffer = itoa::Buffer::new();
        for byte in decimal_buffer.format(n).bytes() {
            print_char(i32::from(byte));
        }
    }
}

#[unsafe(export_name = "print_cs")]
pub unsafe extern "C" fn print_cs(p: i32) {
    unsafe {
        if p < HASH_BASE {
            if p >= SINGLE_BASE {
                if p == NULL_CS {
                    print_esc_cstr(b"csname\0".as_ptr() as *const c_char);
                    print_esc_cstr(b"endcsname\0".as_ptr() as *const c_char);
                    print_char(b' ' as i32);
                } else {
                    print_esc(p - SINGLE_BASE);
                    if (*eqtb.add((CAT_CODE_BASE + (p - SINGLE_BASE)) as usize)).b32.s1 == LETTER as i32 {
                        print_char(b' ' as i32);
                    }
                }
            } else if p < ACTIVE_BASE {
                print_esc_cstr(b"IMPOSSIBLE.\0".as_ptr() as *const c_char);
            } else {
                print_char(p - 1);
            }
        } else if (p >= UNDEFINED_CONTROL_SEQUENCE && p <= EQTB_SIZE) || p > eqtb_top {
            print_esc_cstr(b"IMPOSSIBLE.\0".as_ptr() as *const c_char);
        } else if (*hash.add(p as usize)).s1 >= str_ptr {
            print_esc_cstr(b"NONEXISTENT.\0".as_ptr() as *const c_char);
        } else {
            if p >= PRIM_EQTB_BASE && p < FROZEN_NULL_FONT {
                print_esc(prim[(p - PRIM_EQTB_BASE) as usize].s1 - 1);
            } else {
                print_esc((*hash.add(p as usize)).s1);
            }
            print_char(b' ' as i32);
        }
    }
}

#[unsafe(export_name = "sprint_cs")]
pub unsafe extern "C" fn sprint_cs(p: i32) {
    unsafe {
        if p < HASH_BASE {
            if p < SINGLE_BASE {
                print_char(p - 1);
            } else if p < NULL_CS {
                print_esc(p - SINGLE_BASE);
            } else {
                print_esc_cstr(b"csname\0".as_ptr() as *const c_char);
                print_esc_cstr(b"endcsname\0".as_ptr() as *const c_char);
            }
        } else if p >= PRIM_EQTB_BASE && p < FROZEN_NULL_FONT {
            print_esc(prim[(p - PRIM_EQTB_BASE) as usize].s1 - 1);
        } else {
            print_esc((*hash.add(p as usize)).s1);
        }
    }
}

#[unsafe(export_name = "print_file_name")]
pub unsafe extern "C" fn print_file_name(n: i32, a: i32, e: i32) {
    unsafe {
        let mut must_quote = false;
        let mut quote_char: i32 = 0;

        let check_quote = |mut q_must: bool, mut q_char: i32, str_id: i32| -> (bool, i32) {
            if str_id != 0 {
                let mut j = *str_start.add((str_id - 0x10000) as usize);
                let end = *str_start.add((str_id + 1 - 0x10000) as usize);
                while (!q_must || q_char == 0) && j < end {
                    let c = (*str_pool.add(j as usize)) as i32;
                    if c == b' ' as i32 {
                        q_must = true;
                    } else if c == b'"' as i32 || c == b'\'' as i32 {
                        q_must = true;
                        q_char = 73 - c;
                    }
                    j += 1;
                }
            }
            (q_must, q_char)
        };

        let (mq1, qc1) = check_quote(must_quote, quote_char, a);
        let (mq2, qc2) = check_quote(mq1, qc1, n);
        let (mq3, mut qc3) = check_quote(mq2, qc2, e);
        must_quote = mq3;
        quote_char = qc3;

        if must_quote {
            if quote_char == 0 {
                quote_char = b'"' as i32;
            }
            print_char(quote_char);
        }

        let mut print_component = |str_id: i32, q_c: &mut i32| {
            if str_id != 0 {
                let j_start = *str_start.add((str_id - 0x10000) as usize);
                let for_end = (*str_start.add((str_id + 1 - 0x10000) as usize)) - 1;
                let mut j = j_start;
                while j <= for_end {
                    let ch = (*str_pool.add(j as usize)) as i32;
                    if ch == *q_c {
                        print(*q_c);
                        *q_c = 73 - *q_c;
                        print(*q_c);
                    }
                    print(ch);
                    j += 1;
                }
            }
        };

        print_component(a, &mut quote_char);
        print_component(n, &mut quote_char);
        print_component(e, &mut quote_char);

        if quote_char != 0 {
            print_char(quote_char);
        }
    }
}

#[unsafe(export_name = "print_size")]
pub unsafe extern "C" fn print_size(s: i32) {
    unsafe {
        if s == TEXT_SIZE {
            print_esc_cstr(b"textfont\0".as_ptr() as *const c_char);
        } else if s == SCRIPT_SIZE {
            print_esc_cstr(b"scriptfont\0".as_ptr() as *const c_char);
        } else {
            print_esc_cstr(b"scriptscriptfont\0".as_ptr() as *const c_char);
        }
    }
}

#[unsafe(export_name = "print_write_whatsit")]
pub unsafe extern "C" fn print_write_whatsit(s: *const c_char, p: i32) {
    unsafe {
        print_esc_cstr(s);
        let val = (*mem.add((p + 1) as usize)).b32.s0;
        if val < 16 {
            print_int(val);
        } else if val == 16 {
            print_char(b'*' as i32);
        } else {
            print_char(b'-' as i32);
        }
    }
}

#[unsafe(export_name = "print_native_word")]
pub unsafe extern "C" fn print_native_word(p: i32) {
    unsafe {
        let length = (*mem.add((p + 4) as usize)).b16.s1 as i32;
        let for_end = length - 1;
        let text_ptr = mem.add((p + NATIVE_NODE_SIZE as i32) as usize) as *const u16;

        let mut i = 0;
        while i <= for_end {
            let mut c = *text_ptr.add(i as usize) as i32;
            if (0xD800..0xDC00).contains(&c) {
                if i < length - 1 {
                    let cc = *text_ptr.add((i + 1) as usize) as i32;
                    if (0xDC00..0xE000).contains(&cc) {
                        c = 0x10000 + (c - 0xD800) * 1024 + (cc - 0xDC00);
                        print_char(c);
                        i += 2;
                        continue;
                    } else {
                        print(b'.' as i32);
                    }
                } else {
                    print(b'.' as i32);
                }
            } else {
                print_char(c);
            }
            i += 1;
        }
    }
}

#[unsafe(export_name = "print_sa_num")]
pub unsafe extern "C" fn print_sa_num(mut q: i32) {
    unsafe {
        let n = if (*mem.add(q as usize)).b16.s1 < DIMEN_VAL_LIMIT as u16 {
            (*mem.add((q + 1) as usize)).b32.s1
        } else {
            let mut n_val = ((*mem.add(q as usize)).b16.s1 % 64) as i32;
            q = (*mem.add(q as usize)).b32.s1;
            n_val += 64 * ((*mem.add(q as usize)).b16.s1 as i32);
            q = (*mem.add(q as usize)).b32.s1;
            n_val + 64 * 64 * ((*mem.add(q as usize)).b16.s1 as i32 + 64 * ((*mem.add((*mem.add(q as usize)).b32.s1 as usize)).b16.s1 as i32))
        };
        print_int(n);
    }
}

#[unsafe(export_name = "print_file_line")]
pub unsafe extern "C" fn print_file_line() {
    unsafe {
        let mut level = in_open;
        while level > 0 && (*full_source_filename_stack.add(level as usize)) == 0 {
            level -= 1;
        }

        if level == 0 {
            print_nl_cstr(b"! \0".as_ptr() as *const c_char);
        } else {
            print_nl_cstr(b"\0".as_ptr() as *const c_char);
            print(*full_source_filename_stack.add(level as usize));
            print(b':' as i32);
            if level == in_open {
                print_int(line);
            } else {
                print_int(*line_stack.add((level + 1) as usize));
            }
            print_cstr(b": \0".as_ptr() as *const c_char);
        }
    }
}

#[unsafe(export_name = "print_two")]
pub unsafe extern "C" fn print_two(n: i32) {
    unsafe {
        let val = n.abs() % 100;
        print_char((b'0' + (val / 10) as u8) as i32);
        print_char((b'0' + (val % 10) as u8) as i32);
    }
}

#[unsafe(export_name = "print_hex")]
pub unsafe extern "C" fn print_hex(mut n: i32) {
    unsafe {
        let mut k: u8 = 0;
        print_char(b'"' as i32);
        loop {
            dig[k as usize] = (n % 16) as u8;
            n /= 16;
            k += 1;
            if n == 0 {
                break;
            }
        }
        print_the_digs(k);
    }
}

#[unsafe(export_name = "print_roman_int")]
pub unsafe extern "C" fn print_roman_int(mut n: i32) {
    unsafe {
        const ROMAN_DATA: &[u8] = b"m2d5c2l5x2v5i";
        let mut j: usize = 0;
        let mut v: i32 = 1000;

        loop {
            while n >= v {
                print_char(ROMAN_DATA[j] as i32);
                n -= v;
            }

            if n <= 0 {
                return;
            }

            let mut k = j + 2;
            let mut u = v / ((ROMAN_DATA[k - 1] - b'0') as i32);
            if ROMAN_DATA[k - 1] == b'2' {
                k += 2;
                u /= (ROMAN_DATA[k - 1] - b'0') as i32;
            }

            if n + u >= v {
                print_char(ROMAN_DATA[k] as i32);
                n += u;
            } else {
                j += 2;
                v /= (ROMAN_DATA[j - 1] - b'0') as i32;
            }
        }
    }
}

#[unsafe(export_name = "print_current_string")]
pub unsafe extern "C" fn print_current_string() {
    unsafe {
        let mut j = *str_start.add((str_ptr - 0x10000) as usize);
        while j < pool_ptr {
            print_char((*str_pool.add(j as usize)) as i32);
            j += 1;
        }
    }
}

#[unsafe(export_name = "print_scaled")]
pub unsafe extern "C" fn print_scaled(mut s: scaled_t) {
    unsafe {
        if s < 0 {
            print_char(b'-' as i32);
            s = -s;
        }

        print_int(s / 0x10000);
        print_char(b'.' as i32);
        s = 10 * (s % 0x10000) + 5;
        let mut digit_delta: scaled_t = 10;

        loop {
            if digit_delta > 0x10000 {
                s = s + 0x8000 - 50000;
            }
            print_char((b'0' + (s / 0x10000) as u8) as i32);
            s = 10 * (s % 0x10000);
            digit_delta *= 10;
            if s <= digit_delta {
                break;
            }
        }
    }
}

#[unsafe(export_name = "print_ucs_code")]
pub unsafe extern "C" fn print_ucs_code(mut n: UnicodeScalar) {
    unsafe {
        let mut k: u8 = 0;
        print_cstr(b"U+\0".as_ptr() as *const c_char);

        loop {
            dig[k as usize] = (n % 16) as u8;
            n /= 16;
            k += 1;
            if n == 0 {
                break;
            }
        }

        while k < 4 {
            dig[k as usize] = 0;
            k += 1;
        }

        print_the_digs(k);
    }
}
