#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

use super::xetex_xetexd::{cur_input, mem};

use super::bridge::{
    INVALID_HANDLE, rust_output_handle_t, ttstub_output_close, ttstub_output_open,
    ttstub_output_write,
};
use libc::{c_char, c_int, c_uint, c_void};
use std::ptr;

use super::xetex_bindings::tt_xetex_checkpoint_handle_valid;
use super::xetex_checkpoint::*;
use super::xetex_constants::*;
use super::xetex_core::*;
use super::xetex_io::abspath_of_input_file;
use super::xetex_texmfmp::gettexstring;
use super::xetex_xetexd::*;

pub const SYNCTEX_VERSION: i32 = 1;

#[inline]
unsafe fn synctex_curh() -> i32 {
    unsafe { cur_h + 4736287 }
}

#[inline]
unsafe fn synctex_curv() -> i32 {
    unsafe { cur_v + 4736287 }
}

#[inline]
unsafe fn synctex_tag_box(node: i32) -> i32 {
    unsafe { (*mem.add((node + 8 - 1) as usize)).b32.s0 }
}

#[inline]
unsafe fn synctex_line_box(node: i32) -> i32 {
    unsafe { (*mem.add((node + 8 - 1) as usize)).b32.s1 }
}

#[inline]
unsafe fn synctex_tag_rule(node: i32) -> i32 {
    unsafe { (*mem.add((node + 5 - 1) as usize)).b32.s0 }
}

#[inline]
unsafe fn synctex_line_rule(node: i32) -> i32 {
    unsafe { (*mem.add((node + 5 - 1) as usize)).b32.s1 }
}

#[inline]
unsafe fn synctex_tag_medium(node: i32) -> i32 {
    unsafe { (*mem.add((node + 3 - 1) as usize)).b32.s0 }
}

#[inline]
unsafe fn synctex_line_medium(node: i32) -> i32 {
    unsafe { (*mem.add((node + 3 - 1) as usize)).b32.s1 }
}

#[inline]
unsafe fn synctex_type(node: i32) -> u16 {
    unsafe { (*mem.add(node as usize)).b16.s1 }
}

#[inline]
unsafe fn synctex_width(node: i32) -> i32 {
    unsafe { (*mem.add((node + 1) as usize)).b32.s1 }
}

#[inline]
unsafe fn synctex_depth(node: i32) -> i32 {
    unsafe { (*mem.add((node + 2) as usize)).b32.s1 }
}

#[inline]
unsafe fn synctex_height(node: i32) -> i32 {
    unsafe { (*mem.add((node + 3) as usize)).b32.s1 }
}

type synctex_recorder_t = unsafe fn(i32);

#[derive(Copy, Clone, Default)]
struct SynctexFlags {
    content_ready: bool,
    off: bool,
    not_void: bool,
    warn: bool,
    output_p: bool,
}

struct SynctexContext {
    file: rust_output_handle_t,
    root_name: *mut c_char,
    count: i32,
    node: i32,
    recorder: Option<synctex_recorder_t>,
    tag: i32,
    line: i32,
    curh: i32,
    curv: i32,
    magnification: i32,
    unit: i32,
    total_length: i32,
    lastv: i32,
    form_depth: i32,
    synctex_tag_counter: c_uint,
    flags: SynctexFlags,
}

static mut synctex_ctxt: SynctexContext = SynctexContext {
    file: INVALID_HANDLE,
    root_name: ptr::null_mut(),
    count: 0,
    node: 0,
    recorder: None,
    tag: 0,
    line: 0,
    curh: 0,
    curv: 0,
    magnification: 0,
    unit: 0,
    total_length: 0,
    lastv: -1,
    form_depth: 0,
    synctex_tag_counter: 0,
    flags: SynctexFlags {
        content_ready: false,
        off: false,
        not_void: false,
        warn: false,
        output_p: false,
    },
};

#[unsafe(export_name = "tt_checkpoint_synctex")]
pub unsafe extern "C" fn tt_checkpoint_synctex(cp: *mut tt_checkpoint) {
    unsafe {
        if !(*cp).restoring && { synctex_ctxt.recorder }.is_some() {
            (*cp).valid = false;
            return;
        }

        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.file\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.file as *mut rust_output_handle_t as *mut c_void,
            std::mem::size_of::<rust_output_handle_t>(),
        );
        tt_checkpoint_string(
            cp,
            b"synctex_ctxt.root_name\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.root_name,
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.count\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.count as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.node\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.node as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.tag\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.tag as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.line\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.line as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.curh\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.curh as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.curv\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.curv as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.magnification\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.magnification as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.unit\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.unit as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.total_length\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.total_length as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.lastv\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.lastv as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.form_depth\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.form_depth as *mut i32 as *mut c_void,
            std::mem::size_of::<i32>(),
        );
        tt_checkpoint_field(
            cp,
            b"synctex_ctxt.synctex_tag_counter\0".as_ptr() as *const c_char,
            &raw mut synctex_ctxt.synctex_tag_counter as *mut c_uint as *mut c_void,
            std::mem::size_of::<c_uint>(),
        );

        let mut flags: c_uint = (synctex_ctxt.flags.content_ready as c_uint)
            | ((synctex_ctxt.flags.off as c_uint) << 1)
            | ((synctex_ctxt.flags.not_void as c_uint) << 2)
            | ((synctex_ctxt.flags.warn as c_uint) << 3)
            | ((synctex_ctxt.flags.output_p as c_uint) << 4);
        tt_checkpoint_field(
            cp,
            b"flags\0".as_ptr() as *const c_char,
            &mut flags as *mut c_uint as *mut c_void,
            std::mem::size_of::<c_uint>(),
        );

        if (*cp).restoring
            && synctex_ctxt.file != INVALID_HANDLE
            && !tt_xetex_checkpoint_handle_valid(synctex_ctxt.file as usize, true)
        {
            (*cp).valid = false;
        }

        if (*cp).restoring {
            synctex_ctxt.flags.content_ready = (flags & 1) != 0;
            synctex_ctxt.flags.off = ((flags >> 1) & 1) != 0;
            synctex_ctxt.flags.not_void = ((flags >> 2) & 1) != 0;
            synctex_ctxt.flags.warn = ((flags >> 3) & 1) != 0;
            synctex_ctxt.flags.output_p = ((flags >> 4) & 1) != 0;
            synctex_ctxt.recorder = None;
        }
    }
}

unsafe fn get_current_name() -> *mut c_char {
    unsafe { libc::strdup((&raw const abspath_of_input_file).cast::<c_char>()) }
}

#[unsafe(export_name = "synctex_init_command")]
pub unsafe extern "C" fn synctex_init_command() {
    unsafe {
        synctex_ctxt.file = INVALID_HANDLE;
        synctex_ctxt.root_name = ptr::null_mut();
        synctex_ctxt.count = 0;
        synctex_ctxt.node = 0;
        synctex_ctxt.recorder = None;
        synctex_ctxt.tag = 0;
        synctex_ctxt.line = 0;
        synctex_ctxt.curh = 0;
        synctex_ctxt.curv = 0;
        synctex_ctxt.magnification = 0;
        synctex_ctxt.unit = 0;
        synctex_ctxt.total_length = 0;
        synctex_ctxt.lastv = -1;
        synctex_ctxt.form_depth = 0;
        synctex_ctxt.synctex_tag_counter = 0;
        synctex_ctxt.flags.content_ready = false;
        synctex_ctxt.flags.off = false;
        synctex_ctxt.flags.not_void = false;
        synctex_ctxt.flags.warn = false;
        synctex_ctxt.flags.output_p = false;

        if synctex_enabled != 0 {
            (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 = 1;
        } else {
            (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 = 0;
        }
    }
}

unsafe fn synctexabort() {
    unsafe {
        if synctex_ctxt.file != INVALID_HANDLE {
            ttstub_output_close(synctex_ctxt.file);
            synctex_ctxt.file = INVALID_HANDLE;
        }

        if !synctex_ctxt.root_name.is_null() {
            libc::free(synctex_ctxt.root_name as *mut c_void);
            synctex_ctxt.root_name = ptr::null_mut();
        }

        synctex_ctxt.flags.off = true;
    }
}

pub unsafe fn synctex_abort(_log_opened: bool) {
    unsafe {
        synctexabort();
    }
}

unsafe fn synctex_write_str(s: &str) -> c_int {
    unsafe {
        let bytes = s.as_bytes();
        let written = ttstub_output_write(
            synctex_ctxt.file,
            bytes.as_ptr() as *const c_char,
            bytes.len(),
        );
        if written == bytes.len() {
            bytes.len() as c_int
        } else {
            -1
        }
    }
}

unsafe fn synctex_record_preamble() -> c_int {
    unsafe {
        let s = format!("SyncTeX Version:{}\n", SYNCTEX_VERSION);
        let len = synctex_write_str(&s);
        if len > 0 {
            synctex_ctxt.total_length = len;
            0
        } else {
            synctexabort();
            -1
        }
    }
}

unsafe fn synctex_record_input(tag: i32, name: *const c_char) -> c_int {
    unsafe {
        let name_str = if !name.is_null() {
            std::ffi::CStr::from_ptr(name).to_string_lossy()
        } else {
            std::borrow::Cow::Borrowed("")
        };
        let s = format!("Input:{}:{}\n", tag, name_str);
        let len = synctex_write_str(&s);
        if len > 0 {
            synctex_ctxt.total_length += len;
            0
        } else {
            synctexabort();
            -1
        }
    }
}

unsafe fn synctex_record_anchor() -> c_int {
    unsafe {
        let s = format!("!{}\n", { synctex_ctxt.total_length });
        let len = synctex_write_str(&s);
        if len > 0 {
            synctex_ctxt.total_length = len;
            synctex_ctxt.count += 1;
            0
        } else {
            synctexabort();
            -1
        }
    }
}

unsafe fn synctex_record_content() -> c_int {
    unsafe {
        let len = synctex_write_str("Content:\n");
        if len > 0 {
            synctex_ctxt.total_length += len;
            0
        } else {
            synctexabort();
            -1
        }
    }
}

unsafe fn synctex_record_settings() -> c_int {
    unsafe {
        if synctex_ctxt.file == INVALID_HANDLE {
            return 0;
        }
        let s = format!(
            "Output:pdf\nMagnification:{}\nUnit:{}\nX Offset:0\nY Offset:0\n",
            { synctex_ctxt.magnification },
            { synctex_ctxt.unit }
        );
        let len = synctex_write_str(&s);
        if len > 0 {
            synctex_ctxt.total_length += len;
            0
        } else {
            synctexabort();
            -1
        }
    }
}

unsafe fn synctex_record_sheet(sheet: i32) -> c_int {
    unsafe {
        if synctex_record_anchor() == 0 {
            let s = format!("{{{sheet}\n");
            let len = synctex_write_str(&s);
            if len > 0 {
                synctex_ctxt.total_length += len;
                synctex_ctxt.count += 1;
                return 0;
            }
        }
        synctexabort();
        -1
    }
}

unsafe fn synctex_record_teehs(sheet: i32) -> c_int {
    unsafe {
        if synctex_record_anchor() == 0 {
            let s = format!("}}{sheet}\n");
            let len = synctex_write_str(&s);
            if len > 0 {
                synctex_ctxt.total_length += len;
                synctex_ctxt.count += 1;
                return 0;
            }
        }
        synctexabort();
        -1
    }
}

unsafe fn synctex_record_count() -> c_int {
    unsafe {
        let s = format!("Count:{}\n", { synctex_ctxt.count });
        let len = synctex_write_str(&s);
        if len > 0 {
            synctex_ctxt.total_length += len;
            0
        } else {
            synctexabort();
            -1
        }
    }
}

unsafe fn synctex_record_postamble() -> c_int {
    unsafe {
        if synctex_record_anchor() == 0 {
            let len = synctex_write_str("Postamble:\n");
            if len > 0 {
                synctex_ctxt.total_length += len;
                if synctex_record_count() == 0 && synctex_record_anchor() == 0 {
                    let len2 = synctex_write_str("Post scriptum:\n");
                    if len2 > 0 {
                        synctex_ctxt.total_length += len2;
                        return 0;
                    }
                }
            }
        }
        synctexabort();
        -1
    }
}

unsafe fn synctex_record_node_vlist(p: i32) {
    unsafe {
        synctex_ctxt.flags.not_void = true;
        let s = format!(
            "[{},{}:{},{}:{},{},{}\n",
            synctex_tag_box(p),
            synctex_line_box(p),
            synctex_ctxt.curh / synctex_ctxt.unit,
            synctex_ctxt.curv / synctex_ctxt.unit,
            synctex_width(p) / synctex_ctxt.unit,
            synctex_height(p) / synctex_ctxt.unit,
            synctex_depth(p) / synctex_ctxt.unit
        );
        let len = synctex_write_str(&s);
        synctex_ctxt.lastv = synctex_curv();
        if len > 0 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
        } else {
            synctexabort();
        }
    }
}

unsafe fn synctex_record_node_tsilv(_p: i32) {
    unsafe {
        let len = synctex_write_str("]\n");
        if len > 0 {
            synctex_ctxt.total_length += len;
        } else {
            synctexabort();
        }
    }
}

unsafe fn synctex_record_node_void_vlist(p: i32) {
    unsafe {
        let s = format!(
            "v{},{}:{},{}:{},{},{}\n",
            synctex_tag_box(p),
            synctex_line_box(p),
            synctex_ctxt.curh / synctex_ctxt.unit,
            synctex_ctxt.curv / synctex_ctxt.unit,
            synctex_width(p) / synctex_ctxt.unit,
            synctex_height(p) / synctex_ctxt.unit,
            synctex_depth(p) / synctex_ctxt.unit
        );
        let len = synctex_write_str(&s);
        synctex_ctxt.lastv = synctex_curv();
        if len > 0 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
        } else {
            synctexabort();
        }
    }
}

unsafe fn synctex_record_node_hlist(p: i32) {
    unsafe {
        synctex_ctxt.flags.not_void = true;
        let s = format!(
            "({},{}:{},{}:{},{},{}\n",
            synctex_tag_box(p),
            synctex_line_box(p),
            synctex_ctxt.curh / synctex_ctxt.unit,
            synctex_ctxt.curv / synctex_ctxt.unit,
            synctex_width(p) / synctex_ctxt.unit,
            synctex_height(p) / synctex_ctxt.unit,
            synctex_depth(p) / synctex_ctxt.unit
        );
        let len = synctex_write_str(&s);
        synctex_ctxt.lastv = synctex_curv();
        if len > 0 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
        } else {
            synctexabort();
        }
    }
}

unsafe fn synctex_record_node_tsilh(_p: i32) {
    unsafe {
        let len = synctex_write_str(")\n");
        if len > 0 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
        } else {
            synctexabort();
        }
    }
}

unsafe fn synctex_record_node_void_hlist(p: i32) {
    unsafe {
        let s = format!(
            "h{},{}:{},{}:{},{},{}\n",
            synctex_tag_box(p),
            synctex_line_box(p),
            synctex_ctxt.curh / synctex_ctxt.unit,
            synctex_ctxt.curv / synctex_ctxt.unit,
            synctex_width(p) / synctex_ctxt.unit,
            synctex_height(p) / synctex_ctxt.unit,
            synctex_depth(p) / synctex_ctxt.unit
        );
        let len = synctex_write_str(&s);
        synctex_ctxt.lastv = synctex_curv();
        if len > 0 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
        } else {
            synctexabort();
        }
    }
}

unsafe fn synctex_record_node_glue(p: i32) {
    unsafe {
        let s = format!(
            "g{},{}:{},{}\n",
            synctex_tag_medium(p),
            synctex_line_medium(p),
            synctex_ctxt.curh / synctex_ctxt.unit,
            synctex_ctxt.curv / synctex_ctxt.unit
        );
        let len = synctex_write_str(&s);
        synctex_ctxt.lastv = synctex_curv();
        if len > 0 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
        } else {
            synctexabort();
        }
    }
}

unsafe fn synctex_record_node_kern(p: i32) {
    unsafe {
        let s = format!(
            "k{},{}:{},{}:{}\n",
            synctex_tag_medium(p),
            synctex_line_medium(p),
            synctex_ctxt.curh / synctex_ctxt.unit,
            synctex_ctxt.curv / synctex_ctxt.unit,
            synctex_width(p) / synctex_ctxt.unit
        );
        let len = synctex_write_str(&s);
        synctex_ctxt.lastv = synctex_curv();
        if len > 0 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
        } else {
            synctexabort();
        }
    }
}

unsafe fn synctex_record_node_rule(p: i32) {
    unsafe {
        let s = format!(
            "r{},{}:{},{}:{},{},{}\n",
            synctex_tag_rule(p),
            synctex_line_rule(p),
            synctex_ctxt.curh / synctex_ctxt.unit,
            synctex_ctxt.curv / synctex_ctxt.unit,
            rule_wd / synctex_ctxt.unit,
            rule_ht / synctex_ctxt.unit,
            rule_dp / synctex_ctxt.unit
        );
        let len = synctex_write_str(&s);
        synctex_ctxt.lastv = synctex_curv();
        if len > 0 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
        } else {
            synctexabort();
        }
    }
}

unsafe fn synctex_record_node_math(p: i32) {
    unsafe {
        let s = format!(
            "${},{}:{},{}\n",
            synctex_tag_medium(p),
            synctex_line_medium(p),
            synctex_ctxt.curh / synctex_ctxt.unit,
            synctex_ctxt.curv / synctex_ctxt.unit
        );
        let len = synctex_write_str(&s);
        synctex_ctxt.lastv = synctex_curv();
        if len > 0 {
            synctex_ctxt.total_length += len;
            synctex_ctxt.count += 1;
        } else {
            synctexabort();
        }
    }
}

unsafe fn synctex_record_pdfxform(_form: i32) -> c_int {
    unsafe {
        if synctex_ctxt.flags.off
            || (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 == 0
            || synctex_ctxt.file == INVALID_HANDLE
        {
            0
        } else {
            synctex_ctxt.form_depth += 1;
            let s = format!("<{}\n", { synctex_ctxt.form_depth });
            let len = synctex_write_str(&s);
            if len > 0 {
                synctex_ctxt.total_length += len;
                synctex_ctxt.count += 1;
                return 0;
            }
            synctexabort();
            -1
        }
    }
}

unsafe fn synctex_record_mrofxfdp() -> c_int {
    unsafe {
        if synctex_record_anchor() == 0 {
            synctex_ctxt.form_depth -= 1;
            let len = synctex_write_str(">\n");
            if len > 0 {
                synctex_ctxt.total_length += len;
                synctex_ctxt.count += 1;
                return 0;
            }
        }
        synctexabort();
        -1
    }
}

unsafe fn synctex_record_node_pdfrefxform(objnum: c_int) -> c_int {
    unsafe {
        synctex_ctxt.curh = synctex_curh();
        synctex_ctxt.curv = synctex_curv();

        if synctex_ctxt.flags.off
            || (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 == 0
            || synctex_ctxt.file == INVALID_HANDLE
        {
            0
        } else {
            let s = format!(
                "f{}:{}:{}\n",
                objnum,
                synctex_curh() / synctex_ctxt.unit,
                synctex_curv() / synctex_ctxt.unit
            );
            let len = synctex_write_str(&s);
            synctex_ctxt.lastv = synctex_curv();
            if len > 0 {
                synctex_ctxt.total_length += len;
                synctex_ctxt.count += 1;
                return 0;
            }
            synctexabort();
            -1
        }
    }
}

unsafe fn synctex_dot_open() -> rust_output_handle_t {
    unsafe {
        if synctex_ctxt.flags.off || (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 == 0
        {
            return INVALID_HANDLE;
        }
        if synctex_ctxt.file != INVALID_HANDLE {
            return synctex_ctxt.file;
        }

        let tmp = gettexstring(job_name);
        let len = libc::strlen(tmp);
        if len == 0 {
            libc::free(tmp as *mut c_void);
            synctexabort();
            return INVALID_HANDLE;
        }

        let tmp_str = std::ffi::CStr::from_ptr(tmp).to_string_lossy();
        let the_name = format!("{}.synctex.gz\0", tmp_str);
        libc::free(tmp as *mut c_void);

        synctex_ctxt.file = ttstub_output_open(the_name.as_ptr() as *const c_char, 1);
        if synctex_ctxt.file == INVALID_HANDLE {
            synctexabort();
            return INVALID_HANDLE;
        }

        if synctex_record_preamble() != 0 {
            synctexabort();
            return INVALID_HANDLE;
        }

        if synctex_ctxt.magnification == 0 {
            synctex_ctxt.magnification = 1000;
        }
        synctex_ctxt.unit = 1;

        if !synctex_ctxt.root_name.is_null() {
            synctex_record_input(1, synctex_ctxt.root_name);
            libc::free(synctex_ctxt.root_name as *mut c_void);
            synctex_ctxt.root_name = ptr::null_mut();
        }

        synctex_ctxt.count = 0;
        synctex_ctxt.file
    }
}

unsafe fn synctex_prepare_content() -> rust_output_handle_t {
    unsafe {
        if synctex_ctxt.flags.content_ready {
            return synctex_ctxt.file;
        }

        if INVALID_HANDLE != synctex_dot_open()
            && synctex_record_settings() == 0
            && synctex_record_content() == 0
        {
            synctex_ctxt.flags.content_ready = true;
            return synctex_ctxt.file;
        }

        synctexabort();
        INVALID_HANDLE
    }
}

#[unsafe(export_name = "synctex_start_input")]
pub unsafe extern "C" fn synctex_start_input() {
    unsafe {
        if synctex_ctxt.flags.off {
            return;
        }

        if !synctex_ctxt.synctex_tag_counter > 0 {
            synctex_ctxt.synctex_tag_counter += 1;
        } else {
            synctex_ctxt.synctex_tag_counter = 0;
            return;
        }

        cur_input.synctex_tag = synctex_ctxt.synctex_tag_counter as i32;
        if synctex_ctxt.synctex_tag_counter == 1 {
            synctex_ctxt.root_name = get_current_name();
            if libc::strlen(synctex_ctxt.root_name) == 0 {
                libc::free(synctex_ctxt.root_name as *mut c_void);
                synctex_ctxt.root_name = libc::strdup(b"texput\0".as_ptr() as *const c_char);
            }
            return;
        }

        if synctex_ctxt.file != INVALID_HANDLE || INVALID_HANDLE != synctex_dot_open() {
            let tmp = get_current_name();
            synctex_record_input(cur_input.synctex_tag, tmp);
            libc::free(tmp as *mut c_void);
        }
    }
}

#[unsafe(export_name = "synctex_terminate")]
pub unsafe extern "C" fn synctex_terminate(_log_opened: bool) {
    unsafe {
        if synctex_ctxt.file != INVALID_HANDLE {
            synctex_record_postamble();
            ttstub_output_close(synctex_ctxt.file);
            synctex_ctxt.file = INVALID_HANDLE;
        }
        synctexabort();
    }
}

#[unsafe(export_name = "synctex_sheet")]
pub unsafe extern "C" fn synctex_sheet(mag: i32) {
    unsafe {
        if synctex_ctxt.flags.off {
            if (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 != 0
                && !synctex_ctxt.flags.warn
            {
                synctex_ctxt.flags.warn = true;
                let msg =
                    b"SyncTeX was disabled -- changing the value of \\synctex has no effect\0";
                super::bridge::ttstub_issue_warning(msg.as_ptr() as *const c_char);
            }
            return;
        }

        if total_pages == 0 && mag > 0 {
            synctex_ctxt.magnification = mag;
        }

        if INVALID_HANDLE != synctex_prepare_content() {
            synctex_record_sheet(total_pages + 1);
        }
    }
}

#[unsafe(export_name = "synctex_teehs")]
pub unsafe extern "C" fn synctex_teehs() {
    unsafe {
        if synctex_ctxt.flags.off || synctex_ctxt.file == INVALID_HANDLE {
            return;
        }
        synctex_record_teehs(total_pages);
    }
}

#[inline]
unsafe fn synctex_ignore_general() -> bool {
    unsafe {
        synctex_ctxt.flags.off
            || (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 == 0
            || synctex_ctxt.file == INVALID_HANDLE
    }
}

#[unsafe(export_name = "synctex_vlist")]
pub unsafe extern "C" fn synctex_vlist(this_box: i32) {
    unsafe {
        if synctex_ignore_general() {
            return;
        }
        synctex_ctxt.node = this_box;
        synctex_ctxt.recorder = None;
        synctex_ctxt.tag = synctex_tag_box(this_box);
        synctex_ctxt.line = synctex_line_box(this_box);
        synctex_ctxt.curh = synctex_curh();
        synctex_ctxt.curv = synctex_curv();
        synctex_record_node_vlist(this_box);
    }
}

#[unsafe(export_name = "synctex_tsilv")]
pub unsafe extern "C" fn synctex_tsilv(this_box: i32) {
    unsafe {
        if synctex_ignore_general() {
            return;
        }
        synctex_ctxt.node = this_box;
        synctex_ctxt.tag = synctex_tag_box(this_box);
        synctex_ctxt.line = synctex_line_box(this_box);
        synctex_ctxt.curh = synctex_curh();
        synctex_ctxt.curv = synctex_curv();
        synctex_ctxt.recorder = None;
        synctex_record_node_tsilv(this_box);
    }
}

#[unsafe(export_name = "synctex_void_vlist")]
pub unsafe extern "C" fn synctex_void_vlist(p: i32, _this_box: i32) {
    unsafe {
        if synctex_ignore_general() {
            return;
        }
        synctex_ctxt.node = p;
        synctex_ctxt.tag = synctex_tag_box(p);
        synctex_ctxt.line = synctex_line_box(p);
        synctex_ctxt.curh = synctex_curh();
        synctex_ctxt.curv = synctex_curv();
        synctex_ctxt.recorder = None;
        synctex_record_node_void_vlist(p);
    }
}

#[unsafe(export_name = "synctex_hlist")]
pub unsafe extern "C" fn synctex_hlist(this_box: i32) {
    unsafe {
        if synctex_ignore_general() {
            return;
        }
        synctex_ctxt.node = this_box;
        synctex_ctxt.tag = synctex_tag_box(this_box);
        synctex_ctxt.line = synctex_line_box(this_box);
        synctex_ctxt.curh = synctex_curh();
        synctex_ctxt.curv = synctex_curv();
        synctex_ctxt.recorder = None;
        synctex_record_node_hlist(this_box);
    }
}

#[unsafe(export_name = "synctex_tsilh")]
pub unsafe extern "C" fn synctex_tsilh(this_box: i32) {
    unsafe {
        if synctex_ignore_general() {
            return;
        }
        synctex_ctxt.node = this_box;
        synctex_ctxt.tag = synctex_tag_box(this_box);
        synctex_ctxt.line = synctex_line_box(this_box);
        synctex_ctxt.curh = synctex_curh();
        synctex_ctxt.curv = synctex_curv();
        synctex_ctxt.recorder = None;
        synctex_record_node_tsilh(this_box);
    }
}

#[unsafe(export_name = "synctex_void_hlist")]
pub unsafe extern "C" fn synctex_void_hlist(p: i32, _this_box: i32) {
    unsafe {
        if synctex_ignore_general() {
            return;
        }
        if let Some(recorder) = synctex_ctxt.recorder {
            recorder(synctex_ctxt.node);
        }
        synctex_ctxt.node = p;
        synctex_ctxt.tag = synctex_tag_box(p);
        synctex_ctxt.line = synctex_line_box(p);
        synctex_ctxt.curh = synctex_curh();
        synctex_ctxt.curv = synctex_curv();
        synctex_ctxt.recorder = None;
        synctex_record_node_void_hlist(p);
    }
}

#[inline]
unsafe fn synctex_context_did_change_medium(node: i32) -> bool {
    unsafe {
        synctex_ctxt.node == 0
            || synctex_tag_medium(node) != synctex_ctxt.tag
            || synctex_line_medium(node) != synctex_ctxt.line
    }
}

#[unsafe(export_name = "synctex_math")]
pub unsafe extern "C" fn synctex_math(p: i32, _this_box: i32) {
    unsafe {
        if synctex_ignore_general() {
            return;
        }
        if { synctex_ctxt.recorder }.is_some() && synctex_context_did_change_medium(p) {
            if let Some(recorder) = synctex_ctxt.recorder {
                recorder(synctex_ctxt.node);
            }
        }
        synctex_ctxt.node = p;
        synctex_ctxt.tag = synctex_tag_medium(p);
        synctex_ctxt.line = synctex_line_medium(p);
        synctex_ctxt.curh = synctex_curh();
        synctex_ctxt.curv = synctex_curv();
        synctex_ctxt.recorder = None;
        synctex_record_node_math(p);
    }
}

#[unsafe(export_name = "synctex_horizontal_rule_or_glue")]
pub unsafe extern "C" fn synctex_horizontal_rule_or_glue(p: i32, _this_box: i32) {
    unsafe {
        let t = synctex_type(p) as i32;
        match t {
            2 => {
                // rule_node
                if synctex_ctxt.flags.off
                    || (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 == 0
                    || synctex_tag_rule(p) <= 0
                    || synctex_line_rule(p) <= 0
                {
                    return;
                }
            }
            10 => {
                // glue_node
                if synctex_ctxt.flags.off
                    || (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 == 0
                    || synctex_tag_medium(p) <= 0
                    || synctex_line_medium(p) <= 0
                {
                    return;
                }
            }
            11 => {
                // kern_node
                if synctex_ctxt.flags.off
                    || (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 == 0
                    || synctex_tag_medium(p) <= 0
                    || synctex_line_medium(p) <= 0
                {
                    return;
                }
            }
            _ => {
                let msg = b"unknown node type in SyncTeX\0";
                super::bridge::ttstub_issue_error(msg.as_ptr() as *const c_char);
            }
        }

        synctex_ctxt.node = p;
        synctex_ctxt.curh = synctex_curh();
        synctex_ctxt.curv = synctex_curv();
        synctex_ctxt.recorder = None;

        match t {
            2 => {
                synctex_ctxt.tag = synctex_tag_rule(p);
                synctex_ctxt.line = synctex_line_rule(p);
                synctex_record_node_rule(p);
            }
            10 => {
                synctex_ctxt.tag = synctex_tag_medium(p);
                synctex_ctxt.line = synctex_line_medium(p);
                synctex_record_node_glue(p);
            }
            11 => {
                synctex_ctxt.tag = synctex_tag_medium(p);
                synctex_ctxt.line = synctex_line_medium(p);
                synctex_record_node_kern(p);
            }
            _ => {
                let msg = b"unknown node type in SyncTeX\0";
                super::bridge::ttstub_issue_error(msg.as_ptr() as *const c_char);
            }
        }
    }
}

#[unsafe(export_name = "synctex_kern")]
pub unsafe extern "C" fn synctex_kern(p: i32, this_box: i32) {
    unsafe {
        if synctex_ctxt.flags.off
            || (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 == 0
            || synctex_tag_medium(p) <= 0
            || synctex_line_medium(p) <= 0
        {
            return;
        }

        if synctex_context_did_change_medium(p) {
            if let Some(recorder) = synctex_ctxt.recorder {
                recorder(synctex_ctxt.node);
            }
            if synctex_ctxt.node == this_box {
                synctex_ctxt.node = p;
                synctex_ctxt.tag = synctex_tag_medium(p);
                synctex_ctxt.line = synctex_line_medium(p);
                synctex_ctxt.recorder = Some(synctex_record_node_kern);
            } else {
                synctex_ctxt.node = p;
                synctex_ctxt.tag = synctex_tag_medium(p);
                synctex_ctxt.line = synctex_line_medium(p);
                synctex_ctxt.recorder = None;
                synctex_record_node_kern(p);
            }
        } else {
            synctex_ctxt.node = p;
            synctex_ctxt.tag = synctex_tag_medium(p);
            synctex_ctxt.line = synctex_line_medium(p);
            synctex_ctxt.recorder = Some(synctex_record_node_kern);
        }
    }
}

#[unsafe(export_name = "synctex_current")]
pub unsafe extern "C" fn synctex_current() {
    unsafe {
        if synctex_ignore_general() {
            return;
        }
        let s = format!(
            "x{},{}:{},{}\n",
            { synctex_ctxt.tag },
            { synctex_ctxt.line },
            synctex_curh() / synctex_ctxt.unit,
            synctex_curv() / synctex_ctxt.unit
        );
        synctex_ctxt.lastv = synctex_curv();
        let len = synctex_write_str(&s);
        if len > 0 {
            synctex_ctxt.total_length += len;
        } else {
            synctexabort();
        }
    }
}

#[unsafe(export_name = "synctex_pdfxform")]
pub unsafe extern "C" fn synctex_pdfxform(p: i32) {
    unsafe {
        if synctex_ctxt.flags.off {
            if (*eqtb.add((INT_BASE + INT_PAR__synctex) as usize)).b32.s1 != 0
                && !synctex_ctxt.flags.warn
            {
                synctex_ctxt.flags.warn = true;
                let msg = b"SyncTeX was disabled - changing the value of \\synctex has no effect\0";
                super::bridge::ttstub_issue_warning(msg.as_ptr() as *const c_char);
            }
            return;
        }

        if INVALID_HANDLE != synctex_prepare_content() {
            synctex_record_pdfxform(p);
        }
    }
}

#[unsafe(export_name = "synctex_mrofxfdp")]
pub unsafe extern "C" fn synctex_mrofxfdp() {
    unsafe {
        if synctex_ctxt.file != INVALID_HANDLE {
            synctex_record_mrofxfdp();
        }
    }
}

#[unsafe(export_name = "synctex_pdfrefxform")]
pub unsafe extern "C" fn synctex_pdfrefxform(objnum: c_int) {
    unsafe {
        if synctex_ctxt.file != INVALID_HANDLE {
            synctex_record_node_pdfrefxform(objnum);
        }
    }
}
