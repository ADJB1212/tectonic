#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

pub type str_number = i32;
pub type pool_pointer = i32;
pub type packed_UTF16_code = u16;
pub type UnicodeScalar = i32;

pub const EMPTY_STRING: i32 = 65536 + 1;
pub const TOO_BIG_CHAR: i32 = 65536;

unsafe extern "C" {
    pub static mut str_pool: *mut packed_UTF16_code;
    pub static mut str_start: *mut pool_pointer;
    pub static mut pool_ptr: pool_pointer;
    pub static mut str_ptr: str_number;
    pub static mut init_str_ptr: str_number;
    pub static mut init_pool_ptr: pool_pointer;
    pub static mut max_strings: i32;
    pub static mut pool_size: i32;
    pub static mut buffer: *mut UnicodeScalar;

    pub fn overflow(s: *const libc::c_char, n: i32) -> !;
}

static STRING_CONSTANTS: &[&[u8]] = &[b"this marks the start of the stringpool", b""];

#[unsafe(export_name = "tectonic_rust_load_pool_strings")]
pub unsafe extern "C" fn load_pool_strings(spare_size: i32) -> libc::c_int {
    unsafe {
        let mut total_len: usize = 0;
        let mut g: str_number = 0;

        for &s in STRING_CONSTANTS {
            let len = s.len();
            total_len += len;
            if total_len >= spare_size as usize {
                return 0;
            }

            for &b in s {
                *str_pool.add(pool_ptr as usize) = b as u16;
                pool_ptr += 1;
            }

            g = make_string();
        }

        g
    }
}

#[unsafe(export_name = "tectonic_rust_length")]
pub unsafe extern "C" fn length(s: str_number) -> i32 {
    unsafe {
        if s >= 65536 {
            return *str_start.add((s + 1 - 65536) as usize) - *str_start.add((s - 65536) as usize);
        }
        if s >= 32 && s < 127 {
            return 1;
        }
        if s <= 127 {
            return 3;
        }
        if s < 256 {
            return 4;
        }
        8
    }
}

#[unsafe(export_name = "tectonic_rust_make_string")]
pub unsafe extern "C" fn make_string() -> str_number {
    unsafe {
        if str_ptr == max_strings {
            overflow(c"number of strings".as_ptr(), max_strings - init_str_ptr);
        }

        str_ptr += 1;
        *str_start.add((str_ptr - TOO_BIG_CHAR) as usize) = pool_ptr;
        str_ptr - 1
    }
}

#[unsafe(export_name = "tectonic_rust_append_str")]
pub unsafe extern "C" fn append_str(s: str_number) {
    unsafe {
        let mut i = length(s);

        if pool_ptr + i > pool_size {
            overflow(c"pool size".as_ptr(), pool_size - init_pool_ptr);
        }

        let mut j = *str_start.add((s - 65536) as usize);

        while i > 0 {
            *str_pool.add(pool_ptr as usize) = *str_pool.add(j as usize);
            pool_ptr += 1;
            j += 1;
            i -= 1;
        }
    }
}

#[unsafe(export_name = "tectonic_rust_str_eq_buf")]
pub unsafe extern "C" fn str_eq_buf(s: str_number, mut k: i32) -> bool {
    unsafe {
        let mut j = *str_start.add((s - 65536) as usize);
        let end = *str_start.add((s + 1 - 65536) as usize);

        while j < end {
            let bk = *buffer.add(k as usize);
            let sj = *str_pool.add(j as usize) as i32;
            if bk >= 65536 {
                if sj != 55296 + (bk - 65536) / 1024 {
                    return false;
                } else if (*str_pool.add((j + 1) as usize) as i32) != 56320 + (bk - 65536) % 1024 {
                    return false;
                } else {
                    j += 1;
                }
            } else if sj != bk {
                return false;
            }

            j += 1;
            k += 1;
        }

        true
    }
}

#[unsafe(export_name = "tectonic_rust_str_eq_str")]
pub unsafe extern "C" fn str_eq_str(s: str_number, t: str_number) -> bool {
    unsafe {
        if length(s) != length(t) {
            return false;
        }

        if length(s) == 1 {
            if s < 65536 {
                if t < 65536 {
                    if s != t {
                        return false;
                    }
                } else {
                    let sj = *str_pool.add(*str_start.add((t - 65536) as usize) as usize) as i32;
                    if s != sj {
                        return false;
                    }
                }
            } else if t < 65536 {
                let sj = *str_pool.add(*str_start.add((s - 65536) as usize) as usize) as i32;
                if sj != t {
                    return false;
                }
            } else {
                let s_char = *str_pool.add(*str_start.add((s - 65536) as usize) as usize);
                let t_char = *str_pool.add(*str_start.add((t - 65536) as usize) as usize);
                if s_char != t_char {
                    return false;
                }
            }
        } else {
            let mut j = *str_start.add((s - 65536) as usize);
            let mut k = *str_start.add((t - 65536) as usize);
            let end = *str_start.add((s + 1 - 65536) as usize);

            while j < end {
                if *str_pool.add(j as usize) != *str_pool.add(k as usize) {
                    return false;
                }

                j += 1;
                k += 1;
            }
        }

        true
    }
}

#[unsafe(export_name = "tectonic_rust_search_string")]
pub unsafe extern "C" fn search_string(search: str_number) -> str_number {
    unsafe {
        let len = length(search);

        if len == 0 {
            EMPTY_STRING
        } else {
            let mut s = search - 1;

            while s > 65535 {
                if length(s) == len && str_eq_str(s, search) {
                    return s;
                }

                s -= 1;
            }

            0
        }
    }
}

#[unsafe(export_name = "tectonic_rust_slow_make_string")]
pub unsafe extern "C" fn slow_make_string() -> str_number {
    unsafe {
        let t = make_string();
        let s = search_string(t);

        if s > 0 {
            str_ptr -= 1;
            pool_ptr = *str_start.add((str_ptr - TOO_BIG_CHAR) as usize);
            return s;
        }

        t
    }
}
