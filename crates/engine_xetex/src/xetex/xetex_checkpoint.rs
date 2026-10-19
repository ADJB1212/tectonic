#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use crate::xetex::xetex_io::UFILE;
use std::ffi::{c_char, c_int, c_void};

pub const MEM_TOP: i32 = 4999999;
pub const NATIVE_GLYPH_INFO_SIZE: usize = 10;
pub const LEVEL_ONE: u16 = 1;
pub const SEMI_SIMPLE_GROUP: u8 = 14;
pub const VMODE: i32 = 1;
pub const TEX_NULL: i32 = -0x0FFFFFFF;
pub const NORMAL: u8 = 0;
pub const TOKEN_LIST: u16 = 0;
pub const HISTORY_WARNING_ISSUED: i32 = 1;
pub const CLOSED: u8 = 2;
pub const INPUT: u8 = 106;
pub const CALL: u8 = 113;
pub const END_TEMPLATE: u8 = 117;
pub const HASH_BASE: i32 = 2228226;

#[cfg(target_endian = "big")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2 {
    pub s1: i32,
    pub s0: i32,
}

#[cfg(not(target_endian = "big"))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b32x2 {
    pub s0: i32,
    pub s1: i32,
}

#[cfg(target_endian = "big")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b16x4 {
    pub s3: u16,
    pub s2: u16,
    pub s1: u16,
    pub s0: u16,
}

#[cfg(not(target_endian = "big"))]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct b16x4 {
    pub s0: u16,
    pub s1: u16,
    pub s2: u16,
    pub s3: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union memory_word {
    pub b32: b32x2,
    pub b16: b16x4,
    pub gr: f64,
    pub ptr: *mut c_void,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_state_record {
    pub mode: i32,
    pub head: i32,
    pub tail: i32,
    pub eTeX_aux: i32,
    pub prev_graf: i32,
    pub mode_line: i32,
    pub aux: memory_word,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_state_t {
    pub state: u16,
    pub index: u16,
    pub start: i32,
    pub loc: i32,
    pub limit: i32,
    pub name: i32,
    pub synctex_tag: i32,
}

#[repr(C)]
pub struct tt_checkpoint {
    pub bytes: *mut u8,
    pub length: usize,
    pub capacity: usize,
    pub position: usize,
    pub restoring: bool,
    pub valid: bool,
}

static mut resume_bytes: *const u8 = std::ptr::null();
static mut resume_length: usize = 0;
#[unsafe(export_name = "tt_checkpoint_enabled")]
pub static mut tt_checkpoint_enabled: bool = false;
static mut allowed: bool = false;
#[unsafe(export_name = "tt_checkpoint_resumed")]
pub static mut tt_checkpoint_resumed: bool = false;
#[unsafe(export_name = "tt_checkpoint_dispatch_fetch")]
pub static mut tt_checkpoint_dispatch_fetch: bool = false;
static mut resume_action: u8 = 0;
static mut dispatches: u64 = 0;
static mut skipped_dispatches: u64 = 0;
static mut last_capture: u64 = 0;
static mut capture_bytes: *mut u8 = std::ptr::null_mut();
static mut capture_capacity: usize = 0;
static mut pending: bool = false;
static mut checkpoint_input_level: i32 = 0;
static mut glyph_nodes: [u8; (MEM_TOP + 1) as usize] = [0; (MEM_TOP + 1) as usize];

unsafe extern "C" {
    pub static mut lo_mem_max: i32;
    pub static mut hi_mem_min: i32;
    pub static mut mem: *mut memory_word;
    pub static mut rover: i32;
    pub static mut in_initex_mode: bool;
    pub static mut shell_escape_enabled: bool;
    pub static mut semantic_pagination_enabled: bool;
    pub static mut in_open: i32;
    pub static mut expand_depth_count: i32;
    pub static mut cur_level: u16;
    pub static mut cur_group: u8;
    pub static mut cur_list: list_state_record;
    pub static mut nest_ptr: i32;
    pub static mut cond_ptr: i32;
    pub static mut output_active: bool;
    pub static mut scanner_status: u8;
    pub static mut cur_input: input_state_t;
    pub static mut param_ptr: i32;
    pub static mut history: i32;
    pub static mut help_ptr: u8;
    pub static mut pseudo_files: i32;
    pub static mut trie_not_ready: bool;
    pub static mut read_open: [u8; 17];
    pub static mut input_file: *mut *mut UFILE;
    pub static mut cur_cmd: u8;
    pub static mut cur_chr: i32;
    pub static mut cur_cs: i32;
    pub static mut hash_top: i32;
    pub static mut hash: *mut b32x2;

    pub fn xmalloc(size: usize) -> *mut c_void;
    pub fn xcalloc(count: usize, size: usize) -> *mut c_void;
    pub fn xrealloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn _tt_abort(format: *const c_char, ...) -> !;

    pub fn pdf_files_init();
    pub fn tt_checkpoint_ext_reset();
    pub fn tt_checkpoint_engine(cp: *mut tt_checkpoint);
    pub fn tt_checkpoint_shipout(cp: *mut tt_checkpoint);
    pub fn tt_checkpoint_pagebuilder(cp: *mut tt_checkpoint);
    pub fn tt_checkpoint_synctex(cp: *mut tt_checkpoint);
    pub fn tt_checkpoint_clock(cp: *mut tt_checkpoint);
    pub fn tt_xetex_checkpoint_layout(checkpoint: *mut c_void, restoring: bool) -> bool;
    pub fn tt_xetex_checkpoint_ready() -> bool;
    pub fn tt_xetex_checkpoint_emit(bytes: *const u8, length: usize);
    pub fn gettexstring(s: i32) -> *mut c_char;
    pub fn expand();
    pub fn macro_call();
}

#[unsafe(export_name = "tt_checkpoint_skipped")]
pub unsafe extern "C" fn tt_checkpoint_skipped() -> u64 {
    unsafe { skipped_dispatches }
}

#[unsafe(export_name = "tt_checkpoint_restored")]
pub unsafe extern "C" fn tt_checkpoint_restored() -> bool {
    unsafe { tt_checkpoint_resumed }
}

#[unsafe(export_name = "tt_checkpoint_register_glyph")]
pub unsafe extern "C" fn tt_checkpoint_register_glyph(node: i32) {
    unsafe {
        glyph_nodes[node as usize] = if !(*mem.add((node + 5) as usize)).ptr.is_null() {
            1
        } else {
            0
        };
    }
}

#[unsafe(export_name = "tt_checkpoint_unregister_glyph")]
pub unsafe extern "C" fn tt_checkpoint_unregister_glyph(node: i32) {
    unsafe {
        glyph_nodes[node as usize] = 0;
    }
}

#[unsafe(export_name = "tt_checkpoint_release")]
pub unsafe extern "C" fn tt_checkpoint_release() {
    unsafe {
        libc::free(capture_bytes as *mut c_void);
        capture_bytes = std::ptr::null_mut();
        capture_capacity = 0;
    }
}

#[unsafe(export_name = "tt_checkpoint_set")]
pub unsafe extern "C" fn tt_checkpoint_set(bytes: *const u8, length: usize, record: bool) {
    unsafe {
        tt_checkpoint_release();
        resume_bytes = bytes;
        resume_length = length;
        tt_checkpoint_enabled = record;
        allowed = true;
        checkpoint_input_level = -1;
        pending = false;
        tt_checkpoint_resumed = false;
        tt_checkpoint_dispatch_fetch = false;
        resume_action = 0;
        dispatches = 0;
        skipped_dispatches = 0;
        last_capture = 0;
        if record || !bytes.is_null() {
            std::ptr::write_bytes(
                (&raw mut glyph_nodes).cast::<u8>(),
                0,
                (MEM_TOP + 1) as usize,
            );
            tt_checkpoint_ext_reset();
        }
    }
}

#[unsafe(export_name = "tt_checkpoint_disallow")]
pub unsafe extern "C" fn tt_checkpoint_disallow() {
    unsafe {
        allowed = false;
    }
}

#[unsafe(export_name = "tt_checkpoint_field")]
pub unsafe extern "C" fn tt_checkpoint_field(
    cp: *mut tt_checkpoint,
    name: *const c_char,
    value: *mut c_void,
    length: usize,
) {
    unsafe {
        let cp = &mut *cp;
        let mut header: [u64; 2] = [14695981039346656037u64, length as u64];
        let mut p = name as *const u8;
        while *p != 0 {
            header[0] = (header[0] ^ (*p as u64)).wrapping_mul(1099511628211u64);
            p = p.add(1);
        }
        if !cp.valid || length > 256 * 1024 * 1024 {
            cp.valid = false;
            return;
        }
        let header_size = std::mem::size_of::<[u64; 2]>();
        if cp.restoring {
            if cp.position > cp.length
                || header_size > cp.length - cp.position
                || libc::memcmp(
                    cp.bytes.add(cp.position) as *const c_void,
                    header.as_ptr() as *const c_void,
                    header_size,
                ) != 0
                || length > cp.length - cp.position - header_size
            {
                cp.valid = false;
                return;
            }
            cp.position += header_size;
            std::ptr::copy_nonoverlapping(cp.bytes.add(cp.position), value as *mut u8, length);
            cp.position += length;
        } else {
            let required = cp.length + header_size + length;
            if required > 512 * 1024 * 1024 {
                cp.valid = false;
                return;
            }
            if required > cp.capacity {
                cp.capacity = required + 1024 * 1024;
                cp.bytes = xrealloc(cp.bytes as *mut c_void, cp.capacity) as *mut u8;
            }
            std::ptr::copy_nonoverlapping(
                header.as_ptr() as *const u8,
                cp.bytes.add(cp.length),
                header_size,
            );
            cp.length += header_size;
            std::ptr::copy_nonoverlapping(value as *const u8, cp.bytes.add(cp.length), length);
            cp.length += length;
        }
    }
}

#[unsafe(export_name = "tt_checkpoint_array")]
pub unsafe extern "C" fn tt_checkpoint_array(
    cp: *mut tt_checkpoint,
    name: *const c_char,
    value: *mut *mut c_void,
    count: usize,
    item_size: usize,
) {
    unsafe {
        let cp_ref = &mut *cp;
        if !cp_ref.valid || item_size == 0 || count > (256 * 1024 * 1024) / item_size {
            cp_ref.valid = false;
            return;
        }
        let length = count * item_size;
        if cp_ref.restoring {
            if cp_ref.position > cp_ref.length || length > cp_ref.length - cp_ref.position {
                cp_ref.valid = false;
                return;
            }
            *value = xcalloc(if count == 0 { 1 } else { count }, item_size);
        } else if (*value).is_null() && length != 0 {
            cp_ref.valid = false;
            return;
        }
        tt_checkpoint_field(cp, name, *value, length);
    }
}

#[unsafe(export_name = "tt_checkpoint_used_array")]
pub unsafe extern "C" fn tt_checkpoint_used_array(
    cp: *mut tt_checkpoint,
    name: *const c_char,
    value: *mut *mut c_void,
    capacity: usize,
    used: usize,
    item_size: usize,
) {
    unsafe {
        let cp_ref = &mut *cp;
        if !cp_ref.valid
            || used > capacity
            || item_size == 0
            || capacity > (256 * 1024 * 1024) / item_size
        {
            cp_ref.valid = false;
            return;
        }
        if cp_ref.restoring {
            if cp_ref.position > cp_ref.length || used * item_size > cp_ref.length - cp_ref.position
            {
                cp_ref.valid = false;
                return;
            }
            *value = xcalloc(if capacity == 0 { 1 } else { capacity }, item_size);
        }
        tt_checkpoint_field(cp, name, *value, used * item_size);
    }
}

#[unsafe(export_name = "tt_checkpoint_rle")]
pub unsafe extern "C" fn tt_checkpoint_rle(
    cp: *mut tt_checkpoint,
    name: *const c_char,
    value: *mut *mut c_void,
    count: usize,
    item_size: usize,
) {
    unsafe {
        let cp_ref = &mut *cp;
        if !cp_ref.valid || item_size == 0 || count > (256 * 1024 * 1024) / item_size {
            cp_ref.valid = false;
            return;
        }
        if cp_ref.restoring {
            *value = xcalloc(count, item_size);
        }
        let items = *value as *mut u8;
        let mut position: usize = 0;
        while position < count && (*cp).valid {
            let mut repeated: usize = 0;
            let mut literal: usize = 0;
            let mut stride: u64 = 0;
            if !(*cp).restoring {
                repeated = 1;
                if item_size == std::mem::size_of::<u64>() && position + 1 < count {
                    let mut previous: u64 = 0;
                    let mut next: u64 = 0;
                    std::ptr::copy_nonoverlapping(
                        items.add(position * item_size),
                        &mut previous as *mut u64 as *mut u8,
                        std::mem::size_of::<u64>(),
                    );
                    std::ptr::copy_nonoverlapping(
                        items.add((position + 1) * item_size),
                        &mut next as *mut u64 as *mut u8,
                        std::mem::size_of::<u64>(),
                    );
                    stride = next.wrapping_sub(previous);
                    while position + repeated < count {
                        std::ptr::copy_nonoverlapping(
                            items.add((position + repeated) * item_size),
                            &mut next as *mut u64 as *mut u8,
                            std::mem::size_of::<u64>(),
                        );
                        if next.wrapping_sub(previous) != stride {
                            break;
                        }
                        previous = next;
                        repeated += 1;
                    }
                } else {
                    while position + repeated < count
                        && libc::memcmp(
                            items.add(position * item_size) as *const c_void,
                            items.add((position + repeated) * item_size) as *const c_void,
                            item_size,
                        ) == 0
                    {
                        repeated += 1;
                    }
                }
                if repeated < 3 {
                    repeated = 0;
                    literal = 1;
                    while position + literal < count {
                        let next = position + literal;
                        if next + 2 < count {
                            if item_size == std::mem::size_of::<u64>() {
                                let (mut a, mut b, mut c): (u64, u64, u64) = (0, 0, 0);
                                std::ptr::copy_nonoverlapping(
                                    items.add(next * item_size),
                                    &mut a as *mut u64 as *mut u8,
                                    std::mem::size_of::<u64>(),
                                );
                                std::ptr::copy_nonoverlapping(
                                    items.add((next + 1) * item_size),
                                    &mut b as *mut u64 as *mut u8,
                                    std::mem::size_of::<u64>(),
                                );
                                std::ptr::copy_nonoverlapping(
                                    items.add((next + 2) * item_size),
                                    &mut c as *mut u64 as *mut u8,
                                    std::mem::size_of::<u64>(),
                                );
                                if b.wrapping_sub(a) == c.wrapping_sub(b) {
                                    break;
                                }
                            } else if libc::memcmp(
                                items.add(next * item_size) as *const c_void,
                                items.add((next + 1) * item_size) as *const c_void,
                                item_size,
                            ) == 0
                                && libc::memcmp(
                                    items.add(next * item_size) as *const c_void,
                                    items.add((next + 2) * item_size) as *const c_void,
                                    item_size,
                                ) == 0
                            {
                                break;
                            }
                        }
                        literal += 1;
                    }
                }
            }
            tt_checkpoint_field(
                cp,
                c"repeated".as_ptr(),
                (&mut repeated as *mut usize).cast(),
                std::mem::size_of::<usize>(),
            );
            tt_checkpoint_field(
                cp,
                c"literal".as_ptr(),
                (&mut literal as *mut usize).cast(),
                std::mem::size_of::<usize>(),
            );
            tt_checkpoint_field(
                cp,
                c"stride".as_ptr(),
                (&mut stride as *mut u64).cast(),
                std::mem::size_of::<u64>(),
            );
            if !(*cp).valid
                || repeated > count - position
                || literal > count - position
                || (repeated == 0) == (literal == 0)
            {
                (*cp).valid = false;
                return;
            }
            let length = if repeated != 0 { 1 } else { literal };
            tt_checkpoint_field(
                cp,
                name,
                items.add(position * item_size) as *mut c_void,
                length * item_size,
            );
            if (*cp).restoring && (*cp).valid && repeated != 0 {
                if item_size == std::mem::size_of::<u64>() {
                    let mut word: u64 = 0;
                    std::ptr::copy_nonoverlapping(
                        items.add(position * item_size),
                        &mut word as *mut u64 as *mut u8,
                        std::mem::size_of::<u64>(),
                    );
                    if word != 0 || stride != 0 {
                        for i in 1..repeated {
                            word = word.wrapping_add(stride);
                            std::ptr::copy_nonoverlapping(
                                &word as *const u64 as *const u8,
                                items.add((position + i) * item_size),
                                std::mem::size_of::<u64>(),
                            );
                        }
                    }
                } else {
                    if stride != 0 {
                        (*cp).valid = false;
                        return;
                    }
                    for i in 1..repeated {
                        std::ptr::copy_nonoverlapping(
                            items.add(position * item_size),
                            items.add((position + i) * item_size),
                            item_size,
                        );
                    }
                }
            }
            position += repeated + literal;
        }
    }
}

#[unsafe(export_name = "tt_checkpoint_memory")]
pub unsafe extern "C" fn tt_checkpoint_memory(cp: *mut tt_checkpoint) {
    unsafe {
        if !(*cp).valid || lo_mem_max < 0 || hi_mem_min <= lo_mem_max || hi_mem_min > MEM_TOP {
            (*cp).valid = false;
            return;
        }
        let mut low: *mut memory_word = std::ptr::null_mut();
        let mut high: *mut memory_word = std::ptr::null_mut();
        if (*cp).restoring {
            tt_checkpoint_rle(
                cp,
                c"low".as_ptr(),
                (&mut low as *mut *mut memory_word).cast(),
                (lo_mem_max + 1) as usize,
                std::mem::size_of::<memory_word>(),
            );
            tt_checkpoint_rle(
                cp,
                c"high".as_ptr(),
                (&mut high as *mut *mut memory_word).cast(),
                (MEM_TOP + 1 - hi_mem_min) as usize,
                std::mem::size_of::<memory_word>(),
            );
            if (*cp).valid {
                mem = xmalloc(((MEM_TOP + 2) as usize) * std::mem::size_of::<memory_word>())
                    as *mut memory_word;
                std::ptr::copy_nonoverlapping(low, mem, (lo_mem_max + 1) as usize);
                std::ptr::copy_nonoverlapping(
                    high,
                    mem.add(hi_mem_min as usize),
                    (MEM_TOP + 1 - hi_mem_min) as usize,
                );
            }
            libc::free(low as *mut c_void);
            libc::free(high as *mut c_void);
            return;
        }
        let original = mem;
        mem =
            xcalloc((MEM_TOP + 1) as usize, std::mem::size_of::<memory_word>()) as *mut memory_word;
        std::ptr::copy_nonoverlapping(original, mem, (lo_mem_max + 1) as usize);
        std::ptr::copy_nonoverlapping(
            original.add(hi_mem_min as usize),
            mem.add(hi_mem_min as usize),
            (MEM_TOP + 1 - hi_mem_min) as usize,
        );
        let mut free_block = rover;
        let mut visited: usize = 0;
        loop {
            visited += 1;
            if free_block < 0 || free_block >= lo_mem_max || visited > MEM_TOP as usize {
                (*cp).valid = false;
                break;
            }
            let size = (*mem.add(free_block as usize)).b32.s0;
            if size < 2 || size > lo_mem_max + 1 - free_block {
                (*cp).valid = false;
                break;
            }
            std::ptr::write_bytes(mem.add((free_block + 2) as usize), 0, (size - 2) as usize);
            free_block = (*mem.add((free_block + 1) as usize)).b32.s1;
            if free_block == rover {
                break;
            }
        }
        for node in 0..=MEM_TOP {
            if glyph_nodes[node as usize] != 0 {
                (*mem.add((node + 5) as usize)).ptr = std::ptr::null_mut();
            }
        }
        low = mem;
        high = mem.add(hi_mem_min as usize);
        tt_checkpoint_rle(
            cp,
            c"low".as_ptr(),
            (&mut low as *mut *mut memory_word).cast(),
            (lo_mem_max + 1) as usize,
            std::mem::size_of::<memory_word>(),
        );
        tt_checkpoint_rle(
            cp,
            c"high".as_ptr(),
            (&mut high as *mut *mut memory_word).cast(),
            (MEM_TOP + 1 - hi_mem_min) as usize,
            std::mem::size_of::<memory_word>(),
        );
        libc::free(mem as *mut c_void);
        mem = original;
    }
}

#[unsafe(export_name = "tt_checkpoint_string")]
pub unsafe extern "C" fn tt_checkpoint_string(
    cp: *mut tt_checkpoint,
    name: *const c_char,
    value: *mut *mut c_char,
) {
    unsafe {
        if !(*cp).valid {
            return;
        }
        let mut length: usize = if (*cp).restoring {
            0
        } else if (*value).is_null() {
            0
        } else {
            libc::strlen(*value) + 1
        };
        tt_checkpoint_field(
            cp,
            c"length".as_ptr(),
            (&mut length as *mut usize).cast(),
            std::mem::size_of::<usize>(),
        );
        if !(*cp).valid {
            return;
        }
        if length > 1024 * 1024 {
            (*cp).valid = false;
            return;
        }
        tt_checkpoint_array(cp, name, value.cast(), length, 1);
        if (*cp).restoring && length == 0 {
            libc::free(*value as *mut c_void);
            *value = std::ptr::null_mut();
        }
        if (*cp).restoring && (*cp).valid && length != 0 {
            if *(*value).add(length - 1) != 0 {
                (*cp).valid = false;
            }
        }
    }
}

unsafe fn checkpoint_glyphs(cp: *mut tt_checkpoint) {
    unsafe {
        tt_checkpoint_field(
            cp,
            c"glyph_nodes".as_ptr(),
            (&raw mut glyph_nodes).cast::<u8>().cast(),
            (MEM_TOP + 1) as usize,
        );
        for node in 0..=MEM_TOP {
            if glyph_nodes[node as usize] == 0 {
                continue;
            }
            if node > MEM_TOP - 6 {
                (*cp).valid = false;
                return;
            }
            let length = ((*mem.add((node + 4) as usize)).b16.s0 as usize) * NATIVE_GLYPH_INFO_SIZE;
            if (*cp).restoring {
                (*mem.add((node + 5) as usize)).ptr = std::ptr::null_mut();
            }
            tt_checkpoint_array(
                cp,
                c"glyph".as_ptr(),
                &mut (*mem.add((node + 5) as usize)).ptr,
                length,
                1,
            );
        }
    }
}

unsafe fn transfer(cp: *mut tt_checkpoint) {
    unsafe {
        tt_checkpoint_field(
            cp,
            c"resume_action".as_ptr(),
            (&raw mut resume_action as *mut u8).cast(),
            std::mem::size_of::<u8>(),
        );
        tt_checkpoint_field(
            cp,
            c"dispatches".as_ptr(),
            (&raw mut dispatches as *mut u64).cast(),
            std::mem::size_of::<u64>(),
        );
        tt_checkpoint_engine(cp);
        if !(*cp).valid {
            return;
        }
        checkpoint_glyphs(cp);
        tt_checkpoint_shipout(cp);
        tt_checkpoint_pagebuilder(cp);
        tt_checkpoint_synctex(cp);
        tt_checkpoint_clock(cp);
        if (*cp).valid && !tt_xetex_checkpoint_layout(cp.cast(), (*cp).restoring) {
            (*cp).valid = false;
        }
    }
}

#[unsafe(export_name = "tt_checkpoint_resume")]
pub unsafe extern "C" fn tt_checkpoint_resume() -> bool {
    unsafe {
        if resume_bytes.is_null() || resume_length == 0 {
            return false;
        }
        pdf_files_init();
        let mut cp = tt_checkpoint {
            bytes: resume_bytes as *mut u8,
            length: resume_length,
            capacity: 0,
            position: 0,
            restoring: true,
            valid: true,
        };
        transfer(&mut cp);
        if !cp.valid || cp.position != cp.length {
            _tt_abort(c"invalid engine checkpoint".as_ptr());
        }
        super::xetex_stringpool::reset_string_index();
        tt_checkpoint_ext_reset();
        tt_checkpoint_resumed = true;
        skipped_dispatches = dispatches;
        last_capture = dispatches;
        true
    }
}

unsafe fn capture(action: u8) {
    unsafe {
        if !tt_checkpoint_enabled
            || !allowed
            || in_initex_mode
            || shell_escape_enabled
            || semantic_pagination_enabled
        {
            return;
        }
        if in_open < 1 || expand_depth_count != 0 {
            return;
        }
        let document_group = cur_level == LEVEL_ONE + 1 && cur_group == SEMI_SIMPLE_GROUP;
        if cur_list.mode != VMODE
            || nest_ptr != 0
            || (cur_level != LEVEL_ONE && !document_group)
            || cond_ptr != TEX_NULL
            || output_active
            || scanner_status != NORMAL
            || cur_input.state == TOKEN_LIST
            || param_ptr != 0
            || history > HISTORY_WARNING_ISSUED
            || help_ptr != 0
            || pseudo_files != TEX_NULL
            || trie_not_ready
        {
            return;
        }
        for i in 0..16 {
            if read_open[i] != CLOSED {
                return;
            }
        }
        for i in 1..=in_open {
            let f = *input_file.add(i as usize);
            if f.is_null() || !(*f).conversionData.is_null() {
                return;
            }
        }
        if !tt_xetex_checkpoint_ready() {
            tt_checkpoint_enabled = false;
            return;
        }
        resume_action = action;
        let mut cp = tt_checkpoint {
            bytes: capture_bytes,
            length: 0,
            capacity: capture_capacity,
            position: 0,
            restoring: false,
            valid: true,
        };
        transfer(&mut cp);
        if cp.valid {
            tt_xetex_checkpoint_emit(cp.bytes, cp.length);
            last_capture = dispatches;
        }
        capture_bytes = cp.bytes;
        capture_capacity = cp.capacity;
        resume_action = 0;
        pending = false;
    }
}

#[unsafe(export_name = "tt_checkpoint_try")]
pub unsafe extern "C" fn tt_checkpoint_try() {
    unsafe {
        if !tt_checkpoint_enabled {
            return;
        }
        dispatches += 1;
        if checkpoint_input_level < 0 {
            checkpoint_input_level = in_open;
        }
        if in_open < checkpoint_input_level {
            pending = true;
        }
        checkpoint_input_level = in_open;
        if pending || (in_open == 1 && dispatches - last_capture >= 262144) {
            capture(0);
        }
    }
}

#[unsafe(export_name = "tt_checkpoint_before_input")]
pub unsafe extern "C" fn tt_checkpoint_before_input() {
    unsafe {
        if !tt_checkpoint_enabled
            || !tt_checkpoint_dispatch_fetch
            || expand_depth_count != 0
            || cur_input.state == TOKEN_LIST
            || scanner_status != NORMAL
            || cur_list.mode != VMODE
        {
            return;
        }
        if cur_cmd == INPUT && cur_chr == 0 {
            capture(1);
            return;
        }
        if cur_cmd < CALL || cur_cmd >= END_TEMPLATE || cur_cs < HASH_BASE || cur_cs > hash_top {
            return;
        }
        let name = gettexstring((*hash.add(cur_cs as usize)).s1);
        if !name.is_null() {
            let input = libc::strcmp(name, c"input".as_ptr()) == 0
                || libc::strcmp(name, c"include".as_ptr()) == 0;
            libc::free(name as *mut c_void);
            if input {
                capture(2);
            }
        }
    }
}

#[unsafe(export_name = "tt_checkpoint_continue")]
pub unsafe extern "C" fn tt_checkpoint_continue() {
    unsafe {
        let action = resume_action;
        resume_action = 0;
        if action == 1 {
            expand();
        } else if action == 2 {
            macro_call();
        } else if action != 0 {
            _tt_abort(c"invalid checkpoint continuation".as_ptr());
        }
    }
}
