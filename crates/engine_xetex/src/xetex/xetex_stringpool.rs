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

#[derive(Default)]
struct StringIndex {
    heads: rustc_hash::FxHashMap<u64, i32>,
    entries: Vec<(u64, i32)>,
}

impl StringIndex {
    fn truncate(&mut self, strings: i32) {
        let count = (strings - TOO_BIG_CHAR).max(0) as usize;
        while self.entries.len() > count {
            let (hash, previous) = self.entries.pop().unwrap();
            if previous == 0 {
                self.heads.remove(&hash);
            } else {
                self.heads.insert(hash, previous);
            }
        }
    }

    fn insert(&mut self, hash: u64) {
        let string = TOO_BIG_CHAR + self.entries.len() as i32;
        let previous = self.heads.insert(hash, string).unwrap_or(0);
        self.entries.push((hash, previous));
    }
}

thread_local! {
    static STRING_INDEX: std::cell::RefCell<StringIndex> = Default::default();
}

pub(super) fn reset_string_index() {
    STRING_INDEX.with(|index| *index.borrow_mut() = StringIndex::default());
}

pub(super) fn truncate_string_index(strings: i32) {
    STRING_INDEX.with(|index| index.borrow_mut().truncate(strings));
}

unsafe fn pooled_string<'a>(s: i32) -> &'a [u16] {
    unsafe {
        let start = *str_start.add((s - TOO_BIG_CHAR) as usize) as usize;
        let end = *str_start.add((s + 1 - TOO_BIG_CHAR) as usize) as usize;
        std::slice::from_raw_parts(str_pool.add(start), end - start)
    }
}

pub(super) unsafe fn str_eq_buf_bmp(s: str_number, start: i32, len: i32) -> bool {
    unsafe { super::xetex_simd::eq_utf16_utf32(pooled_string(s), std::slice::from_raw_parts(buffer.add(start as usize), len as usize)) }
}

#[unsafe(export_name = "load_pool_strings")]
pub unsafe extern "C" fn load_pool_strings(spare_size: i32) -> libc::c_int {
    unsafe {
        reset_string_index();
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

#[unsafe(export_name = "length")]
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

#[unsafe(export_name = "make_string")]
pub unsafe extern "C" fn make_string() -> str_number {
    unsafe {
        if str_ptr == max_strings {
            overflow(c"number of strings".as_ptr(), max_strings - init_str_ptr);
        }

        truncate_string_index(str_ptr);
        str_ptr += 1;
        *str_start.add((str_ptr - TOO_BIG_CHAR) as usize) = pool_ptr;
        str_ptr - 1
    }
}

#[unsafe(export_name = "append_str")]
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

#[unsafe(export_name = "str_eq_buf")]
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

#[unsafe(export_name = "str_eq_str")]
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

#[unsafe(export_name = "search_string")]
pub unsafe extern "C" fn search_string(search: str_number) -> str_number {
    unsafe {
        if search < TOO_BIG_CHAR {
            return 0;
        }
        let len = length(search);

        if len == 0 {
            EMPTY_STRING
        } else {
            use std::hash::BuildHasher;
            STRING_INDEX.with(|index| {
                let mut index = index.borrow_mut();
                index.truncate(search);
                while index.entries.len() < (search - TOO_BIG_CHAR) as usize {
                    let s = TOO_BIG_CHAR + index.entries.len() as i32;
                    let hash = index.heads.hasher().hash_one(pooled_string(s));
                    index.insert(hash);
                }
                let text = pooled_string(search);
                let hash = index.heads.hasher().hash_one(text);
                let mut s = index.heads.get(&hash).copied().unwrap_or(0);
                while s != 0 {
                    if pooled_string(s) == text {
                        return s;
                    }
                    s = index.entries[(s - TOO_BIG_CHAR) as usize].1;
                }
                0
            })
        }
    }
}

#[unsafe(export_name = "slow_make_string")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "manual performance measurement"]
    fn benchmark_string_index_hashing() {
        use std::{collections::HashMap, hash::BuildHasher, hint::black_box, time::Instant};
        fn measure<H: BuildHasher + Default>(strings: &[Vec<u16>]) -> std::time::Duration {
            let mut heads: HashMap<u64, usize, H> = HashMap::default();
            for (index, string) in strings.iter().enumerate() {
                let hash = heads.hasher().hash_one(string.as_slice());
                heads.insert(hash, index);
            }
            let start = Instant::now();
            for _ in 0..100 {
                for string in strings {
                    let hash = heads.hasher().hash_one(black_box(string.as_slice()));
                    black_box(heads.get(&hash));
                }
            }
            start.elapsed()
        }
        let strings: Vec<Vec<u16>> = (0..20_000)
            .map(|index| format!("control@sequence@{index}").encode_utf16().collect())
            .collect();
        eprintln!(
            "string index standard: {:?}",
            measure::<std::collections::hash_map::RandomState>(&strings)
        );
        eprintln!("string index Fx: {:?}", measure::<rustc_hash::FxBuildHasher>(&strings));
    }

    #[test]
    fn string_index_restores_collision_chains_after_rollback() {
        let mut index = StringIndex::default();
        index.insert(7);
        index.insert(11);
        index.insert(7);
        assert_eq!(index.heads[&7], TOO_BIG_CHAR + 2);
        assert_eq!(index.entries[2].1, TOO_BIG_CHAR);
        index.truncate(TOO_BIG_CHAR + 1);
        assert_eq!(index.heads[&7], TOO_BIG_CHAR);
        assert!(!index.heads.contains_key(&11));
        index.insert(13);
        assert_eq!(index.heads[&13], TOO_BIG_CHAR + 1);
        index.truncate(TOO_BIG_CHAR);
        assert!(index.heads.is_empty());
        assert!(index.entries.is_empty());
    }
}
