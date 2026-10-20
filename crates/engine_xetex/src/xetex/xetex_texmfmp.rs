#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use super::xetex_ext::xmalloc;
use super::xetex_xetexd::str_number;

use libc::{c_char, c_int, size_t, ssize_t, time_t};
use std::ptr;

use super::xetex_checkpoint::*;
use super::xetex_constants::*;
use super::xetex_io::*;
use super::xetex_xetexd::*;

static mut last_source_name: *mut c_char = ptr::null_mut();
static mut last_lineno: c_int = 0;

const TIME_STR_SIZE: usize = 30;
static mut start_time_str: [c_char; TIME_STR_SIZE] = [0; TIME_STR_SIZE];

#[inline]
unsafe fn check_nprintf(size_get: c_int, size_want: usize) {
    unsafe {
        if size_get < 0 || (size_get as usize) >= size_want {
            crate::xetex::xetex_errors::_tt_abort(c"snprintf failed".as_ptr());
        }
    }
}

#[inline]
fn is_dir_sep(ch: c_char) -> bool {
    let u = ch as u8;
    u == b'/' || u == b'\\'
}

#[unsafe(export_name = "tt_checkpoint_clock")]
pub unsafe extern "C" fn tt_checkpoint_clock(cp: *mut tt_checkpoint) {
    unsafe {
        let mut seconds: i32 = 0;
        let mut micros: i32 = 0;
        get_seconds_and_micros(&mut seconds, &mut micros);
        let now: i64 = (seconds as i64) * 1_000_000 + (micros as i64);
        let mut elapsed: i64 = now - ((epochseconds as i64) * 1_000_000 + (microseconds as i64));
        tt_checkpoint_field(
            cp,
            b"elapsed\0".as_ptr() as *const c_char,
            &mut elapsed as *mut i64 as *mut libc::c_void,
            std::mem::size_of::<i64>(),
        );
        if elapsed < 0 || elapsed > (i32::MAX as i64) * 1_000_000 {
            (*cp).valid = false;
            return;
        }
        if (*cp).restoring {
            let start: i64 = now - elapsed;
            epochseconds = (start / 1_000_000) as i32;
            microseconds = (start % 1_000_000) as i32;
        }
        tt_checkpoint_field(
            cp,
            b"start_time_str\0".as_ptr() as *const c_char,
            (&raw mut start_time_str).cast::<c_char>() as *mut libc::c_void,
            TIME_STR_SIZE,
        );
        tt_checkpoint_string(cp, b"last_source_name\0".as_ptr() as *const c_char, &raw mut last_source_name);
        tt_checkpoint_field(
            cp,
            b"last_lineno\0".as_ptr() as *const c_char,
            &raw mut last_lineno as *mut c_int as *mut libc::c_void,
            std::mem::size_of::<c_int>(),
        );
    }
}

#[unsafe(export_name = "makepdftime")]
pub unsafe extern "C" fn makepdftime(t: time_t, time_str: *mut c_char, utc: bool) {
    unsafe {
        let mut lt: libc::tm = std::mem::zeroed();
        let mut gmt: libc::tm = std::mem::zeroed();

        if utc {
            let ptr = libc::gmtime(&t);
            if !ptr.is_null() {
                lt = *ptr;
            }
        } else {
            let ptr = libc::localtime(&t);
            if !ptr.is_null() {
                lt = *ptr;
            }
        }

        let fmt = b"D:%Y%m%d%H%M%S\0".as_ptr() as *const c_char;
        let mut size = libc::strftime(time_str, TIME_STR_SIZE, fmt, &lt);
        if size == 0 {
            *time_str = 0;
            return;
        }

        if *time_str.add(14) == b'6' as c_char {
            *time_str.add(14) = b'5' as c_char;
            *time_str.add(15) = b'9' as c_char;
            *time_str.add(16) = 0;
        }

        let gmt_ptr = libc::gmtime(&t);
        if !gmt_ptr.is_null() {
            gmt = *gmt_ptr;
        }

        let mut off = 60 * (lt.tm_hour - gmt.tm_hour) + lt.tm_min - gmt.tm_min;
        if lt.tm_year != gmt.tm_year {
            off += if lt.tm_year > gmt.tm_year { 1440 } else { -1440 };
        } else if lt.tm_yday != gmt.tm_yday {
            off += if lt.tm_yday > gmt.tm_yday { 1440 } else { -1440 };
        }

        if off == 0 {
            *time_str.add(size) = b'Z' as c_char;
            size += 1;
            *time_str.add(size) = 0;
        } else {
            let off_hours = off / 60;
            let off_mins = (off - off_hours * 60).abs();
            let tz_fmt = b"%+03d'%02d'\0".as_ptr() as *const c_char;
            let i = libc::snprintf(time_str.add(size), 9, tz_fmt, off_hours, off_mins);
            check_nprintf(i, 9);
        }
    }
}

#[unsafe(export_name = "init_start_time")]
pub unsafe extern "C" fn init_start_time(source_date_epoch: time_t) {
    unsafe {
        makepdftime(source_date_epoch, (&raw mut start_time_str).cast::<c_char>(), true);
    }
}

#[unsafe(export_name = "getcreationdate")]
pub unsafe extern "C" fn getcreationdate() {
    unsafe {
        let len = libc::strlen((&raw const start_time_str).cast::<c_char>());
        if (pool_ptr as usize + len) >= pool_size as usize {
            pool_ptr = pool_size;
            return;
        }

        for i in 0..len {
            (*str_pool.add(pool_ptr as usize)) = start_time_str[i] as u8 as u16;
            pool_ptr += 1;
        }
    }
}

#[unsafe(export_name = "get_date_and_time")]
pub unsafe extern "C" fn get_date_and_time(source_date_epoch: time_t, minutes: *mut i32, day: *mut i32, month: *mut i32, year: *mut i32) {
    unsafe {
        let tmptr = libc::localtime(&source_date_epoch);
        if !tmptr.is_null() {
            *minutes = (*tmptr).tm_hour * 60 + (*tmptr).tm_min;
            *day = (*tmptr).tm_mday;
            *month = (*tmptr).tm_mon + 1;
            *year = (*tmptr).tm_year + 1900;
        }
    }
}

#[unsafe(export_name = "get_seconds_and_micros")]
pub unsafe extern "C" fn get_seconds_and_micros(seconds: *mut i32, micros: *mut i32) {
    unsafe {
        let mut tv: libc::timeval = std::mem::zeroed();
        libc::gettimeofday(&mut tv, ptr::null_mut());
        *seconds = tv.tv_sec as i32;
        *micros = tv.tv_usec as i32;
    }
}

#[unsafe(export_name = "getfilemoddate")]
pub unsafe extern "C" fn getfilemoddate(s: str_number) {
    unsafe {
        let name = gettexstring(s);
        let handle = super::bridge::ttstub_input_open(name, super::bridge::TTBC_FILE_FORMAT_TEX, 0);
        libc::free(name as *mut libc::c_void);

        if handle == super::bridge::INVALID_HANDLE {
            return;
        }

        let mtime = super::bridge::ttstub_input_get_mtime(handle);
        super::bridge::ttstub_input_close(handle);

        let mut buf: [c_char; 20] = [0; 20];
        makepdftime(mtime as time_t, buf.as_mut_ptr(), true);
        let text_len = libc::strlen(buf.as_ptr());

        if (pool_ptr as usize + text_len) >= pool_size as usize {
            pool_ptr = pool_size;
        } else {
            for i in 0..text_len {
                (*str_pool.add(pool_ptr as usize)) = buf[i] as u8 as u16;
                pool_ptr += 1;
            }
        }
    }
}

#[unsafe(export_name = "getfilesize")]
pub unsafe extern "C" fn getfilesize(s: str_number) {
    unsafe {
        let name = gettexstring(s);
        let handle = super::bridge::ttstub_input_open(name, super::bridge::TTBC_FILE_FORMAT_TEX, 0);
        libc::free(name as *mut libc::c_void);

        if handle == super::bridge::INVALID_HANDLE {
            return;
        }

        let file_len = super::bridge::ttstub_input_get_size(handle);
        super::bridge::ttstub_input_close(handle);

        let mut decimal_buffer = itoa::Buffer::new();
        let text = decimal_buffer.format(file_len);
        let text_len = text.len();

        if (pool_ptr as usize + text_len) >= pool_size as usize {
            pool_ptr = pool_size;
        } else {
            for byte in text.bytes() {
                (*str_pool.add(pool_ptr as usize)) = u16::from(byte);
                pool_ptr += 1;
            }
        }
    }
}

#[unsafe(export_name = "getfiledump")]
pub unsafe extern "C" fn getfiledump(s: i32, offset: c_int, length: c_int) {
    unsafe {
        if length <= 0 {
            return;
        }

        if (pool_ptr as usize) + 2 * (length as usize) + 1 >= pool_size as usize {
            pool_ptr = pool_size;
            return;
        }

        let mut file_buffer = vec![0u8; length as usize];

        let name = gettexstring(s);
        let handle = super::bridge::ttstub_input_open(name, super::bridge::TTBC_FILE_FORMAT_TEX, 0);
        libc::free(name as *mut libc::c_void);

        if handle == super::bridge::INVALID_HANDLE {
            return;
        }

        super::bridge::ttstub_input_seek(handle, (offset as libc::c_long) as isize, libc::SEEK_SET);
        let actual = super::bridge::ttstub_input_read(handle, file_buffer.as_mut_ptr().cast(), length as size_t);
        super::bridge::ttstub_input_close(handle);

        if actual > 0 {
            append_hex_to_pool(&file_buffer[..actual as usize]);
        }
    }
}

unsafe fn checkpool_pointer(pool_ptr_val: pool_pointer, len: size_t) {
    unsafe {
        if (pool_ptr_val as usize) + len >= pool_size as usize {
            crate::xetex::xetex_errors::_tt_abort(c"string pool overflow".as_ptr());
        }
    }
}

#[unsafe(export_name = "maketexstring")]
pub unsafe extern "C" fn maketexstring(s: *const c_char) -> c_int {
    unsafe {
        if s.is_null() || *s == 0 {
            return EMPTY_STRING;
        }

        let bytes = std::ffi::CStr::from_ptr(s).to_bytes();
        checkpool_pointer(pool_ptr, bytes.len());
        for unit in String::from_utf8_lossy(bytes).encode_utf16() {
            *str_pool.add(pool_ptr as usize) = unit;
            pool_ptr += 1;
        }

        make_string()
    }
}

#[unsafe(export_name = "gettexstring")]
pub unsafe extern "C" fn gettexstring(s: str_number) -> *mut c_char {
    unsafe {
        let units = if s >= 65536 {
            let start = *str_start.add((s - 65536) as usize);
            let end = *str_start.add((s + 1 - 65536) as usize);
            std::slice::from_raw_parts(str_pool.add(start as usize), (end - start) as usize)
        } else {
            &[]
        };

        let name = xmalloc(units.len() * 3 + 1) as *mut u8;
        let mut written = 0;
        for c in char::decode_utf16(units.iter().copied()) {
            let mut encoded = [0; 4];
            let bytes = c.unwrap_or(char::REPLACEMENT_CHARACTER).encode_utf8(&mut encoded).as_bytes();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), name.add(written), bytes.len());
            written += bytes.len();
        }
        *name.add(written) = 0;
        name.cast()
    }
}

unsafe fn compare_paths(mut p1: *const c_char, mut p2: *const c_char) -> c_int {
    unsafe {
        if p1.is_null() || p2.is_null() {
            return if p1 == p2 {
                0
            } else if p1.is_null() {
                -1
            } else {
                1
            };
        }
        let mut ret: c_int;
        loop {
            ret = (*p1 as c_int) - (*p2 as c_int);
            let advance = (ret == 0 && *p2 != 0) || (is_dir_sep(*p1) && is_dir_sep(*p2));
            if !advance {
                break;
            }
            p1 = p1.add(1);
            p2 = p2.add(1);
        }
        if ret < 0 {
            -1
        } else if ret > 0 {
            1
        } else {
            0
        }
    }
}

#[unsafe(export_name = "is_new_source")]
pub unsafe extern "C" fn is_new_source(srcfilename: str_number, lineno: c_int) -> bool {
    unsafe {
        let name = gettexstring(srcfilename);
        let diff = compare_paths(name, last_source_name) != 0 || lineno != last_lineno;
        libc::free(name as *mut libc::c_void);
        diff
    }
}

#[unsafe(export_name = "remember_source_info")]
pub unsafe extern "C" fn remember_source_info(srcfilename: str_number, lineno: c_int) {
    unsafe {
        if !last_source_name.is_null() {
            libc::free(last_source_name as *mut libc::c_void);
        }
        last_source_name = gettexstring(srcfilename);
        last_lineno = lineno;
    }
}

#[unsafe(export_name = "make_src_special")]
pub unsafe extern "C" fn make_src_special(srcfilename: str_number, lineno: c_int) -> pool_pointer {
    unsafe {
        let oldpool_ptr = pool_ptr;
        let filename = gettexstring(srcfilename);
        let mut buf: [c_char; 40] = [0; 40];
        let fmt = b"src:%d \0".as_ptr() as *const c_char;
        libc::snprintf(buf.as_mut_ptr(), buf.len(), fmt, lineno);

        let buf_len = libc::strlen(buf.as_ptr());
        let fn_len = libc::strlen(filename);

        if (pool_ptr as usize) + buf_len + fn_len >= pool_size as usize {
            crate::xetex::xetex_errors::_tt_abort(c"string pool overflow".as_ptr());
        }

        let mut s = buf.as_ptr();
        while *s != 0 {
            (*str_pool.add(pool_ptr as usize)) = *s as u8 as u16;
            pool_ptr += 1;
            s = s.add(1);
        }

        let mut s2 = filename;
        while *s2 != 0 {
            (*str_pool.add(pool_ptr as usize)) = *s2 as u8 as u16;
            pool_ptr += 1;
            s2 = s2.add(1);
        }

        libc::free(filename as *mut libc::c_void);
        oldpool_ptr
    }
}

unsafe fn append_hex_to_pool(bytes: &[u8]) {
    unsafe {
        let mut encoded = [0; 512];
        for chunk in bytes.chunks(encoded.len() / 2) {
            let text = faster_hex::hex_encode_upper(chunk, &mut encoded).unwrap();
            for byte in text.bytes() {
                *str_pool.add(pool_ptr as usize) = u16::from(byte);
                pool_ptr += 1;
            }
        }
    }
}

const DIGEST_SIZE: usize = 16;

#[cfg(test)]
mod tests {
    use super::*;

    struct PoolState {
        pool: *mut u16,
        starts: *mut i32,
        position: i32,
        size: i32,
        strings: i32,
        limit: i32,
    }

    impl Drop for PoolState {
        fn drop(&mut self) {
            super::super::xetex_stringpool::reset_string_index();
            unsafe {
                str_pool = self.pool;
                str_start = self.starts;
                pool_ptr = self.position;
                pool_size = self.size;
                str_ptr = self.strings;
                max_strings = self.limit;
            }
        }
    }

    #[test]
    fn string_pool_unicode_and_hex_conversions() {
        let mut pool = [0u16; 1024];
        let mut starts = [0i32; 32];
        unsafe {
            let _restore = PoolState {
                pool: str_pool,
                starts: str_start,
                position: pool_ptr,
                size: pool_size,
                strings: str_ptr,
                limit: max_strings,
            };
            str_pool = pool.as_mut_ptr();
            str_start = starts.as_mut_ptr();
            pool_ptr = 0;
            pool_size = pool.len() as i32;
            str_ptr = 65538;
            max_strings = 65536 + starts.len() as i32 - 1;

            for (bytes, expected) in [
                (b"ASCII".as_slice(), "ASCII"),
                ("\u{e4}\u{20ac}\u{1f600}\u{10ffff}".as_bytes(), "\u{e4}\u{20ac}\u{1f600}\u{10ffff}"),
                (b"\xf0\x9f".as_slice(), "\u{fffd}"),
                (b"\xc0\xaf".as_slice(), "\u{fffd}\u{fffd}"),
            ] {
                let input = std::ffi::CString::new(bytes).unwrap();
                let string = maketexstring(input.as_ptr());
                let output = gettexstring(string);
                let actual = std::ffi::CStr::from_ptr(output).to_bytes().to_vec();
                libc::free(output.cast());
                assert_eq!(actual, expected.as_bytes());
            }
            let before = pool_ptr;
            assert_eq!(maketexstring(std::ptr::null()), EMPTY_STRING);
            assert_eq!(maketexstring(c"".as_ptr()), EMPTY_STRING);
            assert_eq!({ pool_ptr }, before);

            let input = maketexstring(c"hello".as_ptr());
            let start = pool_ptr as usize;
            getmd5sum(input, false);
            assert_eq!(
                String::from_utf16(&pool[start..pool_ptr as usize]).unwrap(),
                "5D41402ABC4B2A76B9719D911017C592"
            );

            use super::super::xetex_stringpool::{search_string, slow_make_string};
            pool_ptr = start as i32;
            let duplicate = maketexstring(c"hello".as_ptr());
            assert_eq!(search_string(duplicate), input);
            str_ptr -= 1;
            pool_ptr = starts[(str_ptr - 65536) as usize];
            let replacement = maketexstring(c"other".as_ptr());
            assert_eq!(replacement, duplicate);
            assert_eq!(search_string(replacement), 0);
            let duplicate = maketexstring(c"hello".as_ptr());
            assert_eq!(search_string(duplicate), input);
            let before = pool_ptr;
            for unit in "other".encode_utf16() {
                pool[pool_ptr as usize] = unit;
                pool_ptr += 1;
            }
            assert_eq!(slow_make_string(), replacement);
            assert_eq!({ pool_ptr }, before);

            for (units, expected) in [
                (&[0xd83d, 0xde00][..], "\u{1f600}"),
                (&[0xd800, 0x41][..], "\u{fffd}A"),
                (&[0xdc00][..], "\u{fffd}"),
                (&[0xd800][..], "\u{fffd}"),
                (&[0xffff][..], "\u{ffff}"),
                (&[][..], ""),
            ] {
                pool[..units.len()].copy_from_slice(units);
                starts[0] = 0;
                starts[1] = units.len() as i32;
                let output = gettexstring(65536);
                let actual = std::ffi::CStr::from_ptr(output).to_bytes().to_vec();
                libc::free(output.cast());
                assert_eq!(actual, expected.as_bytes());
            }

            for length in [0, 1, 16, 31, 32, 33, 255, 256, 257, 510] {
                let bytes: Vec<u8> = (0..length).map(|value| value as u8).collect();
                use std::fmt::Write;
                let expected = bytes.iter().fold(String::new(), |mut output, byte| {
                    write!(output, "{byte:02X}").unwrap();
                    output
                });
                pool[2] = 0x1234;
                pool_ptr = 3;
                append_hex_to_pool(&bytes);
                assert_eq!({ pool_ptr }, (3 + length * 2) as i32);
                assert_eq!(pool[2], 0x1234);
                assert_eq!(String::from_utf16(&pool[3..3 + length * 2]).unwrap(), expected);
            }
        }
    }
}

#[unsafe(export_name = "getmd5sum")]
pub unsafe extern "C" fn getmd5sum(s: str_number, file: bool) {
    unsafe {
        let mut digest = [0u8; DIGEST_SIZE];

        let xname = gettexstring(s);
        let ret: c_int = if file {
            super::bridge::ttstub_get_file_md5(xname, digest.as_mut_ptr().cast())
        } else {
            let len = libc::strlen(xname);
            tectonic_bridge_core::ttbc_get_data_md5(xname as *const u8, len, digest.as_mut_ptr())
        };
        libc::free(xname as *mut libc::c_void);

        if ret != 0 {
            return;
        }

        if (pool_ptr as usize) + 2 * DIGEST_SIZE >= pool_size as usize {
            return;
        }

        append_hex_to_pool(&digest);
    }
}
