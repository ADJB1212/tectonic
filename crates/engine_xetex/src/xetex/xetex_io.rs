#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use crate::xetex::teckit_c_engine::*;
use crate::xetex::teckit_common::*;
use crate::xetex::xetex_checkpoint::memory_word;
use crate::xetex::xetex_core::*;
use crate::xetex::xetex_format::*;
use std::ffi::{c_char, c_int, c_void};

pub type rust_input_handle_t = usize;
pub type UnicodeScalar = i32;
pub type UTF16_code = u16;
pub type str_number = i32;

pub const INVALID_HANDLE: usize = 0;
pub const TTBC_FILE_FORMAT_TECTONIC_PRIMARY: c_int = 59;
pub const TTBC_FILE_FORMAT_TEX: c_int = 26;

pub const AUTO: i32 = 0;
pub const UTF8: i32 = 1;
pub const UTF16BE: i32 = 2;
pub const UTF16LE: i32 = 3;
pub const RAW: i32 = 4;
pub const ICUMAPPING: i32 = 5;

pub const CLOSED: u8 = 2;
pub const JUST_OPEN: i32 = 1;

#[cfg(target_endian = "big")]
pub const NATIVE_UTF32: UInt16 = kForm_UTF32BE;
#[cfg(not(target_endian = "big"))]
pub const NATIVE_UTF32: UInt16 = kForm_UTF32LE;

#[cfg(target_endian = "big")]
pub const UCNV_UTF32_NativeEndian: i32 = 7;
#[cfg(not(target_endian = "big"))]
pub const UCNV_UTF32_NativeEndian: i32 = 8;

#[repr(C)]
pub struct ttbc_buffered_input {
    pub handle: rust_input_handle_t,
    pub position: usize,
    pub length: usize,
    pub bytes: [u8; 8192],
}

#[repr(C)]
pub struct UFILE {
    pub handle: rust_input_handle_t,
    pub savedChar: libc::c_long,
    pub skipNextLF: libc::c_short,
    pub encodingMode: libc::c_short,
    pub conversionData: *mut c_void,
    pub input: ttbc_buffered_input,
}

#[unsafe(export_name = "name_of_input_file")]
pub static mut name_of_input_file: *mut c_char = std::ptr::null_mut();

#[unsafe(export_name = "abspath_of_input_file")]
pub static mut abspath_of_input_file: [c_char; 1024] = [0; 1024];

#[unsafe(export_name = "offsetsFromUTF8")]
pub static offsetsFromUTF8: [u32; 6] = [
    0x00000000, 0x00003080, 0x000E2080, 0x03C82080, 0xFA082080, 0x82082080,
];

#[unsafe(export_name = "bytesFromUTF8")]
pub static bytesFromUTF8: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5,
];

#[unsafe(export_name = "firstByteMark")]
pub static firstByteMark: [u8; 7] = [0x00, 0x00, 0xC0, 0xE0, 0xF0, 0xF8, 0xFC];

static mut normalizers: [TECkit_Converter; 2] = [std::ptr::null_mut(), std::ptr::null_mut()];
static mut byteBuffer: *mut u8 = std::ptr::null_mut();
static mut utf32Buf: *mut u32 = std::ptr::null_mut();
static mut name16len: i32 = 0;

unsafe extern "C" {
    pub static mut name_of_file: *mut c_char;
    pub static mut name_of_file16: *mut UTF16_code;
    pub static mut name_length: i32;
    pub static mut name_length16: i32;
    pub static mut buffer: *mut UnicodeScalar;
    pub static mut first: i32;
    pub static mut last: i32;
    pub static mut max_buf_stack: i32;
    pub static mut buf_size: i32;
    pub static mut cur_chr: i32;
    pub static mut cur_val: i32;
    pub static mut read_open: [u8; 17];
    pub static mut read_file: [*mut UFILE; 16];
    pub static mut cur_name: str_number;
    pub static mut cur_area: str_number;
    pub static mut cur_ext: str_number;
    pub static mut name_in_progress: bool;
    pub static mut stop_at_space: bool;
    pub static mut eqtb: *mut memory_word;

    pub fn xmalloc(size: usize) -> *mut c_void;
    pub fn xcalloc(count: usize, size: usize) -> *mut c_void;
    pub fn xstrdup(s: *const c_char) -> *mut c_char;
    pub fn _tt_abort(format: *const c_char, ...) -> !;

    pub fn print_nl_cstr(s: *const c_char);
    pub fn print_cstr(s: *const c_char);
    pub fn print_nl(c: i32);
    pub fn print_int(n: i32);
    pub fn begin_diagnostic();
    pub fn end_diagnostic(blank_line: bool);
    pub fn diagnostic_begin_capture_warning_here() -> *mut tectonic_bridge_core::Diagnostic;
    pub fn capture_to_diagnostic(diagnostic: *mut tectonic_bridge_core::Diagnostic);

    pub fn ttstub_input_open_primary() -> rust_input_handle_t;
    pub fn ttstub_input_open(
        path: *const c_char,
        format: c_int,
        is_gz: c_int,
    ) -> rust_input_handle_t;
    pub fn ttstub_input_getc(handle: rust_input_handle_t) -> c_int;
    pub fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: libc::ssize_t,
        whence: c_int,
    ) -> usize;
    pub fn ttstub_input_close(handle: rust_input_handle_t) -> c_int;
    pub fn ttstub_input_read_partial(
        handle: rust_input_handle_t,
        buf: *mut c_char,
        len: usize,
    ) -> libc::ssize_t;
    pub fn ttstub_get_last_input_abspath(buf: *mut c_char, len: usize) -> c_int;

    pub fn gettexstring(s: str_number) -> *mut c_char;
    pub fn get_input_normalization_state() -> i32;
    pub fn bad_utf8_warning();

    pub fn scan_four_bit_int();
    pub fn scan_optional_equals();
    pub fn scan_file_name();
    pub fn pack_file_name(n: str_number, a: str_number, e: str_number);
    pub fn begin_name();
    pub fn more_name(c: UTF16_code) -> bool;
    pub fn end_name();

    #[link_name = "tt_icu_ucnv_open"]
    pub fn ucnv_open(converterName: *const c_char, err: *mut i32) -> *mut c_void;
    #[link_name = "tt_icu_ucnv_close"]
    pub fn ucnv_close(converter: *mut c_void);
    #[link_name = "tt_icu_ucnv_to_algorithmic"]
    pub fn ucnv_toAlgorithmic(
        alg: i32,
        cnv: *mut c_void,
        target: *mut c_char,
        targetCapacity: i32,
        source: *const c_char,
        sourceLength: i32,
        pErrorCode: *mut i32,
    ) -> i32;
}

#[inline]
pub unsafe fn ttbc_buffered_input_init(
    input: *mut ttbc_buffered_input,
    handle: rust_input_handle_t,
) {
    unsafe {
        (*input).handle = handle;
        (*input).position = 0;
        (*input).length = 0;
    }
}

#[inline]
pub unsafe fn ttbc_buffered_input_getc(input: *mut ttbc_buffered_input) -> c_int {
    unsafe {
        if (*input).position == (*input).length {
            let count = ttstub_input_read_partial(
                (*input).handle,
                (*input).bytes.as_mut_ptr() as *mut c_char,
                (*input).bytes.len(),
            );
            (*input).position = 0;
            (*input).length = 0;
            if count <= 0 {
                return -1;
            }
            (*input).length = count as usize;
        }
        let b = (*input).bytes[(*input).position];
        (*input).position += 1;
        b as c_int
    }
}

#[inline]
pub unsafe fn ttbc_buffered_input_ungetc(input: *mut ttbc_buffered_input, byte: c_int) {
    unsafe {
        if (*input).position > 0 {
            (*input).position -= 1;
            (*input).bytes[(*input).position] = byte as u8;
        }
    }
}

#[unsafe(export_name = "tt_xetex_open_input")]
pub unsafe extern "C" fn tt_xetex_open_input(filefmt: c_int) -> rust_input_handle_t {
    unsafe {
        let handle: rust_input_handle_t;

        if filefmt == TTBC_FILE_FORMAT_TECTONIC_PRIMARY {
            handle = ttstub_input_open_primary();
        } else if *name_of_file == b'|' as c_char {
            print_nl_cstr(c"Warning: ".as_ptr());
            diagnostic_begin_capture_warning_here();
            print_cstr(
                c"piped inputs from external commands are not implemented in Tectonic".as_ptr(),
            );
            capture_to_diagnostic(std::ptr::null_mut());
            return INVALID_HANDLE;
        } else {
            handle = ttstub_input_open(name_of_file, filefmt, 0);
        }

        if handle == INVALID_HANDLE {
            return INVALID_HANDLE;
        }

        if ttstub_get_last_input_abspath((&raw mut abspath_of_input_file).cast::<c_char>(), 1024)
            < 1
        {
            abspath_of_input_file[0] = 0;
        }

        name_length = libc::strlen(name_of_file) as i32;
        libc::free(name_of_input_file as *mut c_void);
        name_of_input_file = xstrdup(name_of_file);
        handle
    }
}

#[unsafe(export_name = "set_input_file_encoding")]
pub unsafe extern "C" fn set_input_file_encoding(f: *mut UFILE, mode: i32, encodingData: i32) {
    unsafe {
        if (*f).encodingMode as i32 == ICUMAPPING && !(*f).conversionData.is_null() {
            ucnv_close((*f).conversionData);
        }
        (*f).conversionData = std::ptr::null_mut();

        match mode {
            UTF8 | UTF16BE | UTF16LE | RAW => {
                (*f).encodingMode = mode as i16;
            }
            ICUMAPPING => {
                let name = gettexstring(encodingData);
                let mut err: i32 = 0;
                let cnv = ucnv_open(name, &mut err);
                if cnv.is_null() {
                    begin_diagnostic();
                    print_nl(b'E' as i32);
                    print_cstr(c"rror ".as_ptr());
                    print_int(err);
                    print_cstr(c" creating Unicode converter for `".as_ptr());
                    print_cstr(name);
                    print_cstr(c"'; reading as raw bytes".as_ptr());
                    end_diagnostic(true);
                    (*f).encodingMode = RAW as i16;
                } else {
                    (*f).encodingMode = ICUMAPPING as i16;
                    (*f).conversionData = cnv;
                }
                libc::free(name as *mut c_void);
            }
            _ => {}
        }
    }
}

#[unsafe(export_name = "u_open_in")]
pub unsafe extern "C" fn u_open_in(
    f: *mut *mut UFILE,
    filefmt: i32,
    fopen_mode: *const c_char,
    mut mode: i32,
    encodingData: i32,
) -> c_int {
    unsafe {
        let handle = tt_xetex_open_input(filefmt);
        if handle == INVALID_HANDLE {
            return 0;
        }

        let ufile = xmalloc(std::mem::size_of::<UFILE>()) as *mut UFILE;
        *f = ufile;
        (*ufile).encodingMode = 0;
        (*ufile).conversionData = std::ptr::null_mut();
        (*ufile).savedChar = -1;
        (*ufile).skipNextLF = 0;
        (*ufile).handle = handle;

        if mode == AUTO {
            let B1 = ttstub_input_getc((*ufile).handle);
            let B2 = ttstub_input_getc((*ufile).handle);

            if B1 == 0xfe && B2 == 0xff {
                mode = UTF16BE;
            } else if B2 == 0xfe && B1 == 0xff {
                mode = UTF16LE;
            } else if B1 == 0 && B2 != 0 {
                mode = UTF16BE;
                ttstub_input_seek((*ufile).handle, 0, libc::SEEK_SET);
            } else if B2 == 0 && B1 != 0 {
                mode = UTF16LE;
                ttstub_input_seek((*ufile).handle, 0, libc::SEEK_SET);
            } else if B1 == 0xef && B2 == 0xbb {
                let B3 = ttstub_input_getc((*ufile).handle);
                if B3 == 0xbf {
                    mode = UTF8;
                }
            }

            if mode == AUTO {
                ttstub_input_seek((*ufile).handle, 0, libc::SEEK_SET);
                mode = UTF8;
            }
        }

        ttbc_buffered_input_init(&mut (*ufile).input, handle);
        set_input_file_encoding(*f, mode, encodingData);
        1
    }
}

unsafe fn buffer_overflow() {
    unsafe {
        _tt_abort(
            c"unable to read an entire line (buf_size=%u)".as_ptr(),
            buf_size as libc::c_uint,
        );
    }
}

unsafe fn conversion_error(errcode: c_int) {
    unsafe {
        begin_diagnostic();
        print_nl(b'U' as i32);
        print_cstr(c"nicode conversion failed (ICU error code = ".as_ptr());
        print_int(errcode);
        print_cstr(c") discarding any remaining text".as_ptr());
        end_diagnostic(true);
    }
}

unsafe fn apply_normalization(buf: *mut u32, len: c_int, norm: c_int) {
    unsafe {
        let norm_idx = (norm - 1) as usize;
        if normalizers[norm_idx].is_null() {
            let out_form =
                (NATIVE_UTF32 as u32) | (if norm == 1 { kForm_NFC } else { kForm_NFD } as u32);
            let status = TECkit_CreateConverter(
                std::ptr::null_mut(),
                0,
                1,
                (NATIVE_UTF32 as u32) as u16,
                (out_form) as u16,
                &mut normalizers[norm_idx],
            );
            if status != kStatus_NoError {
                _tt_abort(
                    c"failed to create normalizer: error code = %d".as_ptr(),
                    status as c_int,
                );
            }
        }

        let norm_conv = normalizers[norm_idx];
        let mut in_used: UInt32 = 0;
        let mut out_used: UInt32 = 0;
        let in_bytes = (len as usize) * std::mem::size_of::<u32>();
        let out_cap =
            (std::mem::size_of::<UnicodeScalar>() * ((buf_size - first) as usize)) as UInt32;
        let out_ptr = buffer.add(first as usize) as *mut Byte;

        let status = TECkit_ConvertBuffer(
            norm_conv,
            buf as *mut Byte,
            in_bytes as UInt32,
            &mut in_used,
            out_ptr,
            out_cap,
            &mut out_used,
            1,
        );
        TECkit_ResetConverter(norm_conv);
        if status != kStatus_NoError {
            buffer_overflow();
        }
        last = first + (out_used as i32) / (std::mem::size_of::<UnicodeScalar>() as i32);
    }
}

#[inline]
fn is_spc_or_eol(c: UnicodeScalar) -> bool {
    c == b' ' as i32 || c == b'\r' as i32 || c == b'\n' as i32
}

fn copy_ascii_line(input: &[u8], output: &mut [UnicodeScalar]) -> usize {
    let input = &input[..input.len().min(output.len())];
    if input.first().is_some_and(|byte| !byte.is_ascii()) {
        return 0;
    }
    let mut count = 0;
    for chunk in input.chunks(64) {
        if !chunk.is_ascii() {
            count += chunk
                .iter()
                .position(|&byte| !byte.is_ascii() || byte == b'\r' || byte == b'\n')
                .unwrap();
            break;
        }
        if let Some(end) = memchr::memchr2(b'\r', b'\n', chunk) {
            count += end;
            break;
        }
        count += chunk.len();
    }
    for (dst, &src) in output[..count].iter_mut().zip(&input[..count]) {
        *dst = src as UnicodeScalar;
    }
    count
}

#[inline]
unsafe fn read_buffered_ascii(f: *mut UFILE, output: *mut UnicodeScalar, capacity: usize) -> usize {
    unsafe {
        if (*f).savedChar != -1 || !matches!((*f).encodingMode as i32, UTF8 | RAW) {
            return 0;
        }
        let input = &mut (*f).input;
        let count = copy_ascii_line(
            &input.bytes[input.position..input.length],
            std::slice::from_raw_parts_mut(output, capacity),
        );
        input.position += count;
        count
    }
}

#[cfg(test)]
mod ascii_tests {
    use super::*;

    #[test]
    fn ascii_line_copy_stops_at_unicode_newlines_and_capacity() {
        for input in [
            b"abc\rdef".as_slice(),
            b"abc\ndef",
            b"abc\xc3\xa9",
            b"abc\xff",
        ] {
            for capacity in 0..=input.len() + 1 {
                let mut output = vec![-1; capacity + 1];
                let count = copy_ascii_line(input, &mut output[..capacity]);
                assert_eq!(count, 3.min(capacity));
                assert_eq!(&output[..count], &[97, 98, 99][..count]);
                assert!(output[count..].iter().all(|&value| value == -1));
            }
        }
        let mut output = [0; 256];
        let input: Vec<u8> = (0..=255).collect();
        assert_eq!(copy_ascii_line(&input, &mut output), 10);
        assert_eq!(&output[..10], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn ascii_line_copy_matches_scalar_at_scan_boundaries() {
        for length in [0, 1, 63, 64, 65, 127, 128, 129, 8192] {
            for position in [0, 1, 62, 63, 64, 65, 126, 127, 128, 8191] {
                for marker in [b'\r', b'\n', 0x80, 0xff] {
                    let mut input = vec![b'a'; length];
                    if position < length {
                        input[position] = marker;
                    }
                    for capacity in [0, 1, 63, 64, 65, length, length + 1] {
                        let mut output = vec![-1; capacity + 1];
                        let expected: Vec<i32> = input
                            .iter()
                            .take(capacity)
                            .take_while(|&&byte| byte.is_ascii() && byte != b'\r' && byte != b'\n')
                            .map(|&byte| byte as i32)
                            .collect();
                        let count = copy_ascii_line(&input, &mut output[..capacity]);
                        assert_eq!(count, expected.len());
                        assert_eq!(&output[..count], expected);
                        assert!(output[count..].iter().all(|&value| value == -1));
                    }
                }
            }
        }
    }

    #[test]
    fn buffered_ascii_preserves_decoder_state_and_buffer_boundaries() {
        let mut file = UFILE {
            handle: 1,
            savedChar: -1,
            skipNextLF: 0,
            encodingMode: UTF8 as i16,
            conversionData: std::ptr::null_mut(),
            input: ttbc_buffered_input {
                handle: 1,
                position: 2,
                length: 8,
                bytes: [0; 8192],
            },
        };
        file.input.bytes[..8].copy_from_slice(b"xxabc\xc3\xa9\n");
        let mut output = [0; 8];
        unsafe {
            assert_eq!(read_buffered_ascii(&mut file, output.as_mut_ptr(), 8), 3);
            assert_eq!(file.input.position, 5);
            assert_eq!(get_uni_c(&mut file), 0xe9);
            assert_eq!(read_buffered_ascii(&mut file, output.as_mut_ptr(), 8), 0);
            assert_eq!(get_uni_c(&mut file), 10);
            assert_eq!(read_buffered_ascii(&mut file, output.as_mut_ptr(), 8), 0);
            for mode in [UTF8, UTF16LE, UTF16BE, RAW] {
                file.encodingMode = mode as i16;
                file.input.position = 2;
                file.savedChar = 42;
                assert_eq!(read_buffered_ascii(&mut file, output.as_mut_ptr(), 8), 0);
                assert_eq!(file.input.position, 2);
                assert_eq!(get_uni_c(&mut file), 42);
                if mode == UTF16LE || mode == UTF16BE {
                    assert_eq!(read_buffered_ascii(&mut file, output.as_mut_ptr(), 8), 0);
                    assert_eq!(file.input.position, 2);
                }
            }
        }
    }

    #[test]
    #[ignore = "manual performance measurement"]
    fn benchmark_ascii_line_decoding() {
        use std::{hint::black_box, time::Instant};
        let mut file = UFILE {
            handle: 1,
            savedChar: -1,
            skipNextLF: 0,
            encodingMode: UTF8 as i16,
            conversionData: std::ptr::null_mut(),
            input: ttbc_buffered_input {
                handle: 1,
                position: 0,
                length: 8192,
                bytes: [b'a'; 8192],
            },
        };
        for length in [40, 120, 1024, 8192] {
            for (label, input) in [
                ("ASCII", vec![b'a'; length]),
                ("mixed", "abc\u{e9}".repeat(length / 5).into_bytes()),
                ("Unicode", "\u{e9}".repeat(length / 2).into_bytes()),
            ] {
                file.input.bytes[..input.len()].copy_from_slice(&input);
                file.input.length = input.len();
                let mut output = vec![0; std::str::from_utf8(&input).unwrap().chars().count()];
                for bulk in [false, true] {
                    let start = Instant::now();
                    for _ in 0..(32 * 1024 * 1024 / length) {
                        file.input.position = 0;
                        let file = black_box(&mut file);
                        if bulk {
                            let mut written = 0;
                            while written < output.len() {
                                let value = unsafe { get_uni_c(file) };
                                output[written] = value;
                                written += 1;
                                if (0..128).contains(&value) {
                                    written += unsafe {
                                        read_buffered_ascii(
                                            file,
                                            output.as_mut_ptr().add(written),
                                            output.len() - written,
                                        )
                                    };
                                }
                            }
                        } else {
                            for dst in &mut output {
                                *dst = unsafe { get_uni_c(file) };
                            }
                        }
                        black_box(&output);
                    }
                    eprintln!("{label} length={length} bulk={bulk}: {:?}", start.elapsed());
                }
            }
        }
    }
}

#[unsafe(export_name = "input_line")]
pub unsafe extern "C" fn input_line(f: *mut UFILE) -> c_int {
    unsafe {
        let mut i: c_int;
        let mut tmpLen: c_int;
        let norm = get_input_normalization_state();

        if (*f).handle == INVALID_HANDLE {
            _tt_abort(c"reads from synthetic \"terminal\" file #0 should never happen".as_ptr());
        }

        last = first;

        if (*f).encodingMode as i32 == ICUMAPPING {
            let mut bytesRead: usize = 0;
            let mut errorCode: i32 = 0;

            if byteBuffer.is_null() {
                byteBuffer = xmalloc((buf_size + 1) as usize) as *mut u8;
            }

            i = ttbc_buffered_input_getc(&mut (*f).input);
            if (*f).skipNextLF != 0 {
                (*f).skipNextLF = 0;
                if i == b'\n' as c_int {
                    i = ttbc_buffered_input_getc(&mut (*f).input);
                }
            }

            if i != -1 && i != b'\n' as c_int && i != b'\r' as c_int {
                *byteBuffer.add(bytesRead) = i as u8;
                bytesRead += 1;
            }
            if i != -1 && i != b'\n' as c_int && i != b'\r' as c_int {
                while bytesRead < buf_size as usize {
                    i = ttbc_buffered_input_getc(&mut (*f).input);
                    if i == -1 || i == b'\n' as c_int || i == b'\r' as c_int {
                        break;
                    }
                    *byteBuffer.add(bytesRead) = i as u8;
                    bytesRead += 1;
                }
            }

            let last_err = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
            if i == -1 && last_err != libc::EINTR && bytesRead == 0 {
                return 0;
            }

            if i != -1 && i != b'\n' as c_int && i != b'\r' as c_int {
                buffer_overflow();
            }

            let cnv = (*f).conversionData;
            match norm {
                1 | 2 => {
                    if utf32Buf.is_null() {
                        utf32Buf =
                            xcalloc(buf_size as usize, std::mem::size_of::<u32>()) as *mut u32;
                    }
                    tmpLen = ucnv_toAlgorithmic(
                        UCNV_UTF32_NativeEndian,
                        cnv,
                        utf32Buf as *mut c_char,
                        (buf_size as usize * std::mem::size_of::<u32>()) as i32,
                        byteBuffer as *const c_char,
                        bytesRead as i32,
                        &mut errorCode,
                    );
                    if errorCode != 0 {
                        conversion_error(errorCode);
                        return 0;
                    }
                    apply_normalization(
                        utf32Buf,
                        tmpLen / (std::mem::size_of::<u32>() as i32),
                        norm,
                    );
                }
                _ => {
                    let mut outLen = ucnv_toAlgorithmic(
                        UCNV_UTF32_NativeEndian,
                        cnv,
                        buffer.add(first as usize) as *mut c_char,
                        ((buf_size - first) as usize * std::mem::size_of::<UnicodeScalar>()) as i32,
                        byteBuffer as *const c_char,
                        bytesRead as i32,
                        &mut errorCode,
                    );
                    if errorCode != 0 {
                        conversion_error(errorCode);
                        return 0;
                    }
                    outLen /= std::mem::size_of::<UnicodeScalar>() as i32;
                    last = first + outLen;
                }
            }
        } else {
            i = get_uni_c(f);
            if (*f).skipNextLF != 0 {
                (*f).skipNextLF = 0;
                if i == b'\n' as c_int {
                    i = get_uni_c(f);
                }
            }

            match norm {
                1 | 2 => {
                    if utf32Buf.is_null() {
                        utf32Buf =
                            xcalloc(buf_size as usize, std::mem::size_of::<u32>()) as *mut u32;
                    }
                    tmpLen = 0;
                    if i != -1 && i != b'\n' as c_int && i != b'\r' as c_int {
                        *utf32Buf.add(tmpLen as usize) = i as u32;
                        tmpLen += 1;
                    }
                    if i != -1 && i != b'\n' as c_int && i != b'\r' as c_int {
                        while tmpLen < buf_size {
                            let copied = if (0..128).contains(&i) {
                                read_buffered_ascii(
                                    f,
                                    utf32Buf.add(tmpLen as usize).cast(),
                                    (buf_size - tmpLen) as usize,
                                )
                            } else {
                                0
                            };
                            tmpLen += copied as i32;
                            if tmpLen == buf_size {
                                break;
                            }
                            i = get_uni_c(f);
                            if i == -1 || i == b'\n' as c_int || i == b'\r' as c_int {
                                break;
                            }
                            *utf32Buf.add(tmpLen as usize) = i as u32;
                            tmpLen += 1;
                        }
                    }

                    let last_err = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
                    if i == -1 && last_err != libc::EINTR && tmpLen == 0 {
                        return 0;
                    }

                    if i != -1 && i != b'\n' as c_int && i != b'\r' as c_int {
                        buffer_overflow();
                    }
                    apply_normalization(utf32Buf, tmpLen, norm);
                }
                _ => {
                    if last < buf_size && i != -1 && i != b'\n' as c_int && i != b'\r' as c_int {
                        *buffer.add(last as usize) = i;
                        last += 1;
                    }
                    if i != -1 && i != b'\n' as c_int && i != b'\r' as c_int {
                        while last < buf_size {
                            let copied = if (0..128).contains(&i) {
                                read_buffered_ascii(
                                    f,
                                    buffer.add(last as usize),
                                    (buf_size - last) as usize,
                                )
                            } else {
                                0
                            };
                            last += copied as i32;
                            if last == buf_size {
                                break;
                            }
                            i = get_uni_c(f);
                            if i == -1 || i == b'\n' as c_int || i == b'\r' as c_int {
                                break;
                            }
                            *buffer.add(last as usize) = i;
                            last += 1;
                        }
                    }

                    let last_err = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
                    if i == -1 && last_err != libc::EINTR && last == first {
                        return 0;
                    }

                    if i != -1 && i != b'\n' as c_int && i != b'\r' as c_int {
                        buffer_overflow();
                    }
                }
            }
        }

        if i == b'\r' as c_int {
            (*f).skipNextLF = 1;
        }

        *buffer.add(last as usize) = b' ' as i32;
        if last >= max_buf_stack {
            max_buf_stack = last;
        }

        while last > first && is_spc_or_eol(*buffer.add((last - 1) as usize)) {
            last -= 1;
        }

        1
    }
}

#[unsafe(export_name = "u_close")]
pub unsafe extern "C" fn u_close(f: *mut UFILE) {
    unsafe {
        if f.is_null() || (*f).handle == INVALID_HANDLE {
            return;
        }

        ttstub_input_close((*f).handle);

        if (*f).encodingMode as i32 == ICUMAPPING && !(*f).conversionData.is_null() {
            ucnv_close((*f).conversionData);
        }

        libc::free(f as *mut c_void);
    }
}

#[unsafe(export_name = "get_uni_c")]
pub unsafe extern "C" fn get_uni_c(f: *mut UFILE) -> c_int {
    unsafe {
        let mut rval: c_int;
        let mut c: c_int;

        if (*f).savedChar != -1 {
            rval = (*f).savedChar as c_int;
            (*f).savedChar = -1;
            return rval;
        }

        match (*f).encodingMode as i32 {
            UTF8 => {
                rval = ttbc_buffered_input_getc(&mut (*f).input);
                c = rval;
                if rval != -1 {
                    let extraBytes = bytesFromUTF8[rval as usize];
                    let mut bad = false;
                    match extraBytes {
                        3 => {
                            c = ttbc_buffered_input_getc(&mut (*f).input);
                            if c < 0x80 || c >= 0xC0 {
                                bad = true;
                            } else {
                                rval <<= 6;
                                rval += c;
                                c = ttbc_buffered_input_getc(&mut (*f).input);
                                if c < 0x80 || c >= 0xC0 {
                                    bad = true;
                                } else {
                                    rval <<= 6;
                                    rval += c;
                                    c = ttbc_buffered_input_getc(&mut (*f).input);
                                    if c < 0x80 || c >= 0xC0 {
                                        bad = true;
                                    } else {
                                        rval <<= 6;
                                        rval += c;
                                    }
                                }
                            }
                        }
                        2 => {
                            c = ttbc_buffered_input_getc(&mut (*f).input);
                            if c < 0x80 || c >= 0xC0 {
                                bad = true;
                            } else {
                                rval <<= 6;
                                rval += c;
                                c = ttbc_buffered_input_getc(&mut (*f).input);
                                if c < 0x80 || c >= 0xC0 {
                                    bad = true;
                                } else {
                                    rval <<= 6;
                                    rval += c;
                                }
                            }
                        }
                        1 => {
                            c = ttbc_buffered_input_getc(&mut (*f).input);
                            if c < 0x80 || c >= 0xC0 {
                                bad = true;
                            } else {
                                rval <<= 6;
                                rval += c;
                            }
                        }
                        0 => {}
                        4 | 5 => {
                            bad_utf8_warning();
                            return 0xFFFD;
                        }
                        _ => {}
                    }

                    if bad {
                        if c != -1 {
                            ttbc_buffered_input_ungetc(&mut (*f).input, c);
                        }
                        bad_utf8_warning();
                        return 0xFFFD;
                    }

                    rval -= offsetsFromUTF8[extraBytes as usize] as c_int;

                    if rval < 0 || rval > 0x10ffff {
                        bad_utf8_warning();
                        return 0xfffd;
                    }
                }
            }
            UTF16BE => {
                rval = ttbc_buffered_input_getc(&mut (*f).input);
                if rval != -1 {
                    rval <<= 8;
                    rval += ttbc_buffered_input_getc(&mut (*f).input);
                    if rval >= 0xd800 && rval <= 0xdbff {
                        let mut lo = ttbc_buffered_input_getc(&mut (*f).input);
                        lo <<= 8;
                        lo += ttbc_buffered_input_getc(&mut (*f).input);
                        if lo >= 0xdc00 && lo <= 0xdfff {
                            rval = 0x10000 + (rval - 0xd800) * 0x400 + (lo - 0xdc00);
                        } else {
                            rval = 0xfffd;
                            (*f).savedChar = lo as libc::c_long;
                        }
                    } else if rval >= 0xdc00 && rval <= 0xdfff {
                        rval = 0xfffd;
                    }
                }
            }
            UTF16LE => {
                rval = ttbc_buffered_input_getc(&mut (*f).input);
                if rval != -1 {
                    rval += ttbc_buffered_input_getc(&mut (*f).input) << 8;
                    if rval >= 0xd800 && rval <= 0xdbff {
                        let mut lo = ttbc_buffered_input_getc(&mut (*f).input);
                        lo += ttbc_buffered_input_getc(&mut (*f).input) << 8;
                        if lo >= 0xdc00 && lo <= 0xdfff {
                            rval = 0x10000 + (rval - 0xd800) * 0x400 + (lo - 0xdc00);
                        } else {
                            rval = 0xfffd;
                            (*f).savedChar = lo as libc::c_long;
                        }
                    } else if rval >= 0xdc00 && rval <= 0xdfff {
                        rval = 0xfffd;
                    }
                }
            }
            RAW => {
                rval = ttbc_buffered_input_getc(&mut (*f).input);
            }
            _ => {
                _tt_abort(
                    c"internal error; file input mode=%d".as_ptr(),
                    (*f).encodingMode as c_int,
                );
            }
        }

        rval
    }
}

#[unsafe(export_name = "make_utf16_name")]
pub unsafe extern "C" fn make_utf16_name() {
    unsafe {
        let mut s = name_of_file as *mut u8;
        let end = (name_of_file as *mut u8).add(name_length as usize);
        if name_of_file16.is_null() || name16len <= name_length {
            libc::free(name_of_file16 as *mut c_void);
            name16len = name_length + 10;
            name_of_file16 = xcalloc(name16len as usize, std::mem::size_of::<u16>()) as *mut u16;
        }
        let mut t = name_of_file16;

        while s < end {
            let mut rval = *s as u32;
            s = s.add(1);
            let extra_bytes = bytesFromUTF8[rval as usize] as usize;
            match extra_bytes {
                5 => {
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                }
                4 => {
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                }
                3 => {
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                }
                2 => {
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                }
                1 => {
                    rval <<= 6;
                    if *s != 0 {
                        rval += *s as u32;
                        s = s.add(1);
                    }
                }
                _ => {}
            }
            rval = rval.wrapping_sub(offsetsFromUTF8[extra_bytes]);
            if rval > 0xffff {
                rval -= 0x10000;
                *t = (0xd800 + rval / 0x0400) as u16;
                t = t.add(1);
                *t = (0xdc00 + rval % 0x0400) as u16;
                t = t.add(1);
            } else {
                *t = rval as u16;
                t = t.add(1);
            }
        }
        name_length16 = t.offset_from(name_of_file16) as i32;
    }
}

#[unsafe(export_name = "open_or_close_in")]
pub unsafe extern "C" fn open_or_close_in() {
    unsafe {
        let c = cur_chr as u8;
        scan_four_bit_int();
        let n = cur_val as usize;

        if read_open[n] != CLOSED {
            u_close(read_file[n]);
            read_open[n] = CLOSED;
        }

        if c != 0 {
            scan_optional_equals();
            scan_file_name();
            pack_file_name(cur_name, cur_area, cur_ext);

            let default_mode = (*eqtb
                .offset((INT_BASE + INT_PAR__xetex_default_input_mode) as isize))
            .b32
            .s1;
            let default_encoding = (*eqtb
                .offset((INT_BASE + INT_PAR__xetex_default_input_encoding) as isize))
            .b32
            .s1;

            if u_open_in(
                &mut read_file[n],
                TTBC_FILE_FORMAT_TEX,
                c"rb".as_ptr(),
                default_mode,
                default_encoding,
            ) != 0
            {
                make_utf16_name();
                name_in_progress = true;
                begin_name();
                stop_at_space = false;
                let mut k: usize = 0;
                while (k < name_length16 as usize) && more_name(*name_of_file16.add(k)) {
                    k += 1;
                }
                stop_at_space = true;
                end_name();
                name_in_progress = false;
                read_open[n] = JUST_OPEN as u8;
            }
        }
    }
}
