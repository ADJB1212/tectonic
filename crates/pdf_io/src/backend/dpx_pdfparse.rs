/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
#![allow(
    warnings,
    clippy::all,
    unsafe_op_in_unsafe_fn,
    static_mut_refs,
    dangerous_implicit_autorefs
)]
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isxdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pow(_: ::core::ffi::c_double, _: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
    -> *mut ::core::ffi::c_char;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_release_obj"]
    fn pdf_release_obj(object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_obj_typeof"]
    fn pdf_obj_typeof(object: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_null"]
    fn pdf_new_null() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_boolean"]
    fn pdf_new_boolean(value: ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_number_value"]
    fn pdf_number_value(number: *mut pdf_obj) -> ::core::ffi::c_double;
    #[link_name = "__pdf_impl_pdf_new_string"]
    fn pdf_new_string(str: *const ::core::ffi::c_void, length: size_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_array"]
    fn pdf_new_array() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_array"]
    fn pdf_add_array(array: *mut pdf_obj, object: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_merge_dict"]
    fn pdf_merge_dict(dict1: *mut pdf_obj, dict2: *mut pdf_obj);
    #[link_name = "__pdf_impl_pdf_lookup_dict"]
    fn pdf_lookup_dict(dict: *mut pdf_obj, key: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(
        dict: *mut pdf_obj,
        key: *mut pdf_obj,
        value: *mut pdf_obj,
    ) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_stream"]
    fn pdf_new_stream(flags: ::core::ffi::c_int) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_stream"]
    fn pdf_add_stream(
        stream: *mut pdf_obj,
        stream_data_ptr: *const ::core::ffi::c_void,
        stream_data_len: ::core::ffi::c_int,
    );
    #[link_name = "__pdf_impl_pdf_stream_dict"]
    fn pdf_stream_dict(stream: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_deref_obj"]
    fn pdf_deref_obj(object: *mut pdf_obj) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_indirect"]
    fn pdf_new_indirect(pf: *mut pdf_file, label: uint32_t, generation: uint16_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_xtoi"]
    fn xtoi(c: ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint16_t = u16;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub tainted: ::core::ffi::c_int,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
#[inline]
unsafe extern "C-unwind" fn strstartswith(
    mut s: *const ::core::ffi::c_char,
    mut prefix: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut length: size_t = 0;
    length = strlen(prefix);
    if strncmp(s, prefix, length) == 0 as ::core::ffi::c_int {
        return s.offset(length as isize);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
pub const PDF_NUMBER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;

#[unsafe(export_name = "__pdf_impl_dump")]
pub unsafe extern "C-unwind" fn dump(
    start: *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) {
    let bytes = unsafe { input_bytes(start, end) };
    pdf_message!("\nCurrent input buffer is -->");
    for &byte in bytes.iter().take(DUMP_LIMIT as usize) {
        pdf_message!(c"%c".as_ptr(), byte as i32);
    }
    if bytes.len() >= DUMP_LIMIT as usize {
        pdf_message!("...");
    }
    pdf_message!("<--\n");
}
pub const DUMP_LIMIT: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
// The caller supplies a live buffer with both pointers in the same allocation.
unsafe fn input_bytes<'a>(
    start: *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) -> &'a [u8] {
    if start >= end {
        return &[];
    }
    unsafe { core::slice::from_raw_parts(start.cast(), end.offset_from(start) as usize) }
}

#[unsafe(export_name = "__pdf_impl_pdfparse_skip_line")]
pub unsafe extern "C-unwind" fn pdfparse_skip_line(
    start: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) {
    let consumed = crate::syntax::skip_line(unsafe { input_bytes(*start, end) });
    unsafe {
        *start = (*start).add(consumed);
    }
}

#[unsafe(export_name = "__pdf_impl_skip_white")]
pub unsafe extern "C-unwind" fn skip_white(
    start: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) {
    let consumed = crate::syntax::skip_white(unsafe { input_bytes(*start, end) });
    unsafe {
        *start = (*start).add(consumed);
    }
}
unsafe extern "C-unwind" fn parsed_string(
    start: *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    let bytes = unsafe { input_bytes(start, end) };
    if bytes.is_empty() {
        return core::ptr::null_mut();
    }
    let size = bytes
        .len()
        .checked_add(1)
        .and_then(|len| u32::try_from(len).ok())
        .unwrap_or_else(|| pdf_abort!("PDF token is too large."));
    unsafe {
        let result = new(size).cast::<core::ffi::c_char>();
        core::ptr::copy_nonoverlapping(start, result, bytes.len());
        *result.add(bytes.len()) = 0;
        result
    }
}
#[unsafe(export_name = "__pdf_impl_parse_number")]
pub unsafe extern "C-unwind" fn parse_number(
    start: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    unsafe { parse_numeric_token(start, end, false) }
}

#[unsafe(export_name = "__pdf_impl_parse_unsigned")]
pub unsafe extern "C-unwind" fn parse_unsigned(
    start: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    unsafe { parse_numeric_token(start, end, true) }
}

unsafe fn parse_numeric_token(
    start: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
    unsigned: bool,
) -> *mut core::ffi::c_char {
    unsafe {
        skip_white(start, end);
        let consumed = crate::syntax::number_length(input_bytes(*start, end), unsigned);
        let next = (*start).add(consumed);
        let result = parsed_string(*start, next);
        *start = next;
        result
    }
}

unsafe extern "C-unwind" fn parse_gen_ident(
    start: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
    valid_chars: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    unsafe {
        let valid = std::ffi::CStr::from_ptr(valid_chars).to_bytes_with_nul();
        let consumed = input_bytes(*start, end)
            .iter()
            .take_while(|b| valid.contains(b))
            .count();
        let next = (*start).add(consumed);
        let result = parsed_string(*start, next);
        *start = next;
        result
    }
}
#[unsafe(export_name = "__pdf_impl_parse_ident")]
pub unsafe extern "C-unwind" fn parse_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    const valid_chars: *const ::core::ffi::c_char =
        b"!\"#$&'*+,-.0123456789:;=?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz|~\0"
            as *const u8 as *const ::core::ffi::c_char;
    return parse_gen_ident(start, end, valid_chars);
}
#[unsafe(export_name = "__pdf_impl_parse_val_ident")]
pub unsafe extern "C-unwind" fn parse_val_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    const valid_chars: *const ::core::ffi::c_char =
        b"!\"#$&'*+,-./0123456789:;?@ABCDEFGHIJKLMNOPQRSTUVWXYZ\\^_`abcdefghijklmnopqrstuvwxyz|~\0"
            as *const u8 as *const ::core::ffi::c_char;
    return parse_gen_ident(start, end, valid_chars);
}
#[unsafe(export_name = "__pdf_impl_parse_opt_ident")]
pub unsafe extern "C-unwind" fn parse_opt_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if *start < end && **start as ::core::ffi::c_int == '@' as i32 {
        *start = (*start).offset(1);
        return parse_ident(start, end);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_number")]
pub unsafe extern "C-unwind" fn parse_pdf_number(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) -> *mut pdf_obj {
    match crate::syntax::number(unsafe { input_bytes(*pp, end) }) {
        Ok((value, consumed)) => unsafe {
            *pp = (*pp).add(consumed);
            pdf_new_number(value)
        },
        Err(error) => {
            pdf_warning!("{}", error.message.to_string_lossy());
            core::ptr::null_mut()
        }
    }
}
pub const PDF_NAME_LEN_MAX: core::ffi::c_int = 128;
pub const PDF_STRING_LEN_MAX: core::ffi::c_int = 65535;

#[unsafe(export_name = "__pdf_impl_parse_pdf_name")]
pub unsafe extern "C-unwind" fn parse_pdf_name(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) -> *mut pdf_obj {
    unsafe {
        skip_white(pp, end);
    }
    let bytes = unsafe { input_bytes(*pp, end) };
    let parsed = crate::syntax::name(bytes, |message| {
        pdf_warning!("{}", message);
    });
    match parsed {
        Ok((mut name, consumed)) => unsafe {
            name.push(0);
            *pp = (*pp).add(consumed);
            pdf_new_name(name.as_ptr().cast())
        },
        Err(error) => {
            pdf_warning!("{}", error.message.to_string_lossy());
            core::ptr::null_mut()
        }
    }
}

#[unsafe(export_name = "__pdf_impl_parse_pdf_boolean")]
pub unsafe extern "C-unwind" fn parse_pdf_boolean(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) -> *mut pdf_obj {
    unsafe {
        skip_white(pp, end);
    }
    let bytes = unsafe { input_bytes(*pp, end) };
    for (word, value) in [(b"true".as_slice(), 1), (b"false".as_slice(), 0)] {
        if crate::syntax::keyword(bytes, word) {
            return unsafe {
                *pp = (*pp).add(word.len());
                pdf_new_boolean(value)
            };
        }
    }
    pdf_warning!("Could not find a boolean object.");
    core::ptr::null_mut()
}

#[unsafe(export_name = "__pdf_impl_parse_pdf_null")]
pub unsafe extern "C-unwind" fn parse_pdf_null(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) -> *mut pdf_obj {
    unsafe {
        skip_white(pp, end);
    }
    if crate::syntax::keyword(unsafe { input_bytes(*pp, end) }, b"null") {
        return unsafe {
            *pp = (*pp).add(4);
            pdf_new_null()
        };
    }
    pdf_warning!("Could not find a null object.");
    core::ptr::null_mut()
}

unsafe extern "C-unwind" fn parse_pdf_literal_string(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
    tainted: bool,
) -> *mut pdf_obj {
    let mut start = unsafe { *pp };
    unsafe {
        skip_white(&mut start, end);
    }
    match crate::syntax::literal(unsafe { input_bytes(start, end) }, 65533, tainted, false) {
        Ok((bytes, consumed)) => unsafe {
            *pp = start.add(consumed);
            pdf_new_string(bytes.as_ptr().cast(), bytes.len())
        },
        Err(error) => {
            if error.message.to_bytes() == b"PDF string length too long." {
                pdf_warning!("PDF string length too long. (limit: 65535)");
            } else {
                pdf_warning!("{}", error.message.to_string_lossy());
            }
            core::ptr::null_mut()
        }
    }
}
fn decode_pdf_hex_string(bytes: &[u8]) -> Option<(Vec<u8>, usize)> {
    if bytes.first() != Some(&b'<') {
        return None;
    }
    let mut encoded = Vec::new();
    let mut offset = 1;
    while let Some(&byte) = bytes.get(offset) {
        match byte {
            b'>' => break,
            b' ' | b'\t' | b'\x0c' | b'\r' | b'\n' | 0 => offset += 1,
            b'%' => {
                while bytes
                    .get(offset)
                    .is_some_and(|byte| !matches!(byte, b'\r' | b'\n'))
                {
                    offset += 1;
                }
            }
            _ => {
                if encoded.len() >= PDF_STRING_LEN_MAX as usize * 2 {
                    pdf_warning!(
                        "PDF string length too long. (limit: {})",
                        PDF_STRING_LEN_MAX
                    );
                    return None;
                }
                encoded.push(byte);
                offset += 1;
            }
        }
    }
    if offset == bytes.len() {
        pdf_warning!("Premature end of input hex string.");
        return None;
    }
    if encoded.len() % 2 != 0 {
        encoded.push(b'0');
    }
    let mut output = vec![0; encoded.len() / 2];
    if faster_hex::hex_decode(&encoded, &mut output).is_err() {
        pdf_warning!("Invalid hexadecimal digit in PDF string.");
        return None;
    }
    Some((output, offset + 1))
}

unsafe extern "C-unwind" fn parse_pdf_hex_string(
    pp: *mut *const ::core::ffi::c_char,
    endptr: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut start = unsafe { *pp };
    unsafe { skip_white(&mut start, endptr) };
    if start >= endptr {
        return core::ptr::null_mut();
    }
    let bytes = unsafe {
        core::slice::from_raw_parts(start.cast::<u8>(), endptr.offset_from(start) as usize)
    };
    let Some((decoded, consumed)) = decode_pdf_hex_string(bytes) else {
        return core::ptr::null_mut();
    };
    unsafe {
        *pp = start.add(consumed);
        pdf_new_string(decoded.as_ptr().cast(), decoded.len())
    }
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_string")]
pub unsafe extern "C-unwind" fn parse_pdf_string(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
) -> *mut pdf_obj {
    unsafe {
        skip_white(pp, end);
    }
    let bytes = unsafe { input_bytes(*pp, end) };
    if bytes.first() == Some(&b'(') {
        return unsafe { parse_pdf_literal_string(pp, end, false) };
    }
    if bytes.first() == Some(&b'<')
        && bytes
            .get(1)
            .is_some_and(|b| *b == b'>' || b.is_ascii_hexdigit())
    {
        return unsafe { parse_pdf_hex_string(pp, end) };
    }
    pdf_warning!("Could not find a string object.");
    core::ptr::null_mut()
}

type UnknownHandler = Option<
    unsafe extern "C-unwind" fn(
        *mut *const core::ffi::c_char,
        *const core::ffi::c_char,
        *mut core::ffi::c_void,
    ) -> *mut pdf_obj,
>;

struct ParsedObject(*mut pdf_obj);

impl ParsedObject {
    fn take(mut self) -> *mut pdf_obj {
        core::mem::replace(&mut self.0, core::ptr::null_mut())
    }
}

impl Drop for ParsedObject {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                pdf_release_obj(self.0);
            }
        }
    }
}

unsafe fn parse_container(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
    pf: *mut pdf_file,
    handler: UnknownHandler,
    data: *mut core::ffi::c_void,
    depth: usize,
    dict: bool,
    tainted: bool,
) -> *mut pdf_obj {
    if depth >= 256 {
        pdf_warning!("PDF object nesting too deep.");
        return core::ptr::null_mut();
    }
    let mut p = unsafe { *pp };
    unsafe {
        skip_white(&mut p, end);
    }
    let opening = if dict {
        b"<<".as_slice()
    } else {
        b"[".as_slice()
    };
    if !unsafe { input_bytes(p, end) }.starts_with(opening) {
        return core::ptr::null_mut();
    }
    unsafe {
        p = p.add(opening.len());
    }
    let result = ParsedObject(unsafe {
        if dict {
            pdf_new_dict()
        } else {
            pdf_new_array()
        }
    });
    loop {
        unsafe {
            skip_white(&mut p, end);
        }
        let bytes = unsafe { input_bytes(p, end) };
        let closing = if dict {
            b">>".as_slice()
        } else {
            b"]".as_slice()
        };
        if bytes.starts_with(closing) {
            unsafe {
                *pp = p.add(closing.len());
            }
            return result.take();
        }
        if bytes.is_empty() || (dict && bytes.first() == Some(&b'>')) {
            pdf_warning!(
                "{}",
                if dict {
                    "Syntax error: Dictionary object ended prematurely."
                } else {
                    "Array object ended prematurely."
                }
            );
            return core::ptr::null_mut();
        }
        let key = ParsedObject(if dict {
            unsafe { parse_pdf_name(&mut p, end) }
        } else {
            core::ptr::null_mut()
        });
        if dict && key.0.is_null() {
            pdf_warning!("Could not find a key in dictionary object.");
            return core::ptr::null_mut();
        }
        let before = p;
        let value = ParsedObject(unsafe {
            parse_object(&mut p, end, pf, handler, data, depth + 1, tainted)
        });
        if value.0.is_null() || p <= before || p > end {
            pdf_warning!(
                "{}",
                if dict {
                    "Could not find a value in dictionary object."
                } else {
                    "Could not find a valid object in array object."
                }
            );
            return core::ptr::null_mut();
        }
        unsafe {
            if dict {
                pdf_add_dict(result.0, key.take(), value.take());
            } else {
                pdf_add_array(result.0, value.take());
            }
        }
    }
}

unsafe extern "C-unwind" fn parse_pdf_dict_extended(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
    pf: *mut pdf_file,
    handler: UnknownHandler,
    data: *mut core::ffi::c_void,
) -> *mut pdf_obj {
    unsafe { parse_container(pp, end, pf, handler, data, 0, true, false) }
}

unsafe extern "C-unwind" fn parse_pdf_array_extended(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
    pf: *mut pdf_file,
    handler: UnknownHandler,
    data: *mut core::ffi::c_void,
) -> *mut pdf_obj {
    unsafe { parse_container(pp, end, pf, handler, data, 0, false, false) }
}

unsafe extern "C-unwind" fn parse_pdf_stream(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
    dict: *mut pdf_obj,
) -> *mut pdf_obj {
    let start = unsafe { *pp };
    let bytes = unsafe { input_bytes(start, end) };
    let mut pos = crate::syntax::skip_white(bytes);
    if !bytes[pos..].starts_with(b"stream") {
        return core::ptr::null_mut();
    }
    pos += 6;
    if bytes.get(pos) == Some(&b'\n') {
        pos += 1;
    } else if bytes[pos..].starts_with(b"\r\n") {
        pos += 2;
    }
    let length = unsafe { pdf_lookup_dict(dict, c"Length".as_ptr()) };
    if length.is_null() {
        return core::ptr::null_mut();
    }
    let length = ParsedObject(unsafe { pdf_deref_obj(length) });
    if unsafe { pdf_obj_typeof(length.0) } != PDF_NUMBER {
        return core::ptr::null_mut();
    }
    let length = unsafe { pdf_number_value(length.0) };
    if !length.is_finite() || length < 0.0 || length > i32::MAX as f64 {
        return core::ptr::null_mut();
    }
    let length = length as usize;
    let Some(stream_end) = pos.checked_add(length).filter(|&n| n <= bytes.len()) else {
        return core::ptr::null_mut();
    };
    let mut next = stream_end;
    if bytes.get(next) == Some(&b'\r') {
        next += 1;
    }
    if bytes.get(next) == Some(&b'\n') {
        next += 1;
    }
    if !bytes[next..].starts_with(b"endstream") {
        return core::ptr::null_mut();
    }
    let filter = unsafe { pdf_lookup_dict(dict, c"Filter".as_ptr()) };
    let result = ParsedObject(unsafe {
        pdf_new_stream(if filter.is_null() && length > 10 {
            STREAM_COMPRESS
        } else {
            0
        })
    });
    unsafe {
        pdf_merge_dict(pdf_stream_dict(result.0), dict);
        pdf_add_stream(
            result.0,
            bytes[pos..stream_end].as_ptr().cast(),
            length as i32,
        );
        *pp = start.add(next + 9);
    }
    result.take()
}
unsafe extern "C-unwind" fn try_pdf_reference(
    start: *const core::ffi::c_char,
    end: *const core::ffi::c_char,
    endptr: *mut *const core::ffi::c_char,
    pf: *mut pdf_file,
) -> *mut pdf_obj {
    if !endptr.is_null() {
        unsafe {
            *endptr = start;
        }
    }
    let Some((id, generation, consumed)) =
        crate::syntax::reference(unsafe { input_bytes(start, end) })
    else {
        return core::ptr::null_mut();
    };
    unsafe {
        if !endptr.is_null() {
            *endptr = start.add(consumed);
        }
        pdf_new_indirect(pf, id, generation)
    }
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_object_extended")]
pub unsafe extern "C-unwind" fn parse_pdf_object_extended(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
    pf: *mut pdf_file,
    handler: UnknownHandler,
    data: *mut core::ffi::c_void,
) -> *mut pdf_obj {
    unsafe { parse_object(pp, end, pf, handler, data, 0, false) }
}

unsafe fn parse_object(
    pp: *mut *const core::ffi::c_char,
    end: *const core::ffi::c_char,
    pf: *mut pdf_file,
    handler: UnknownHandler,
    data: *mut core::ffi::c_void,
    depth: usize,
    tainted: bool,
) -> *mut pdf_obj {
    unsafe {
        skip_white(pp, end);
    }
    let bytes = unsafe { input_bytes(*pp, end) };
    let Some(&first) = bytes.first() else {
        pdf_warning!("Could not find any valid object.");
        return core::ptr::null_mut();
    };
    unsafe {
        match first {
            b'<' if bytes.get(1) == Some(&b'<') => {
                let dict = ParsedObject(parse_container(
                    pp, end, pf, handler, data, depth, true, tainted,
                ));
                skip_white(pp, end);
                if !dict.0.is_null() && input_bytes(*pp, end).starts_with(b"stream") {
                    parse_pdf_stream(pp, end, dict.0)
                } else {
                    dict.take()
                }
            }
            b'<' => parse_pdf_hex_string(pp, end),
            b'(' => parse_pdf_literal_string(pp, end, tainted),
            b'[' => parse_container(pp, end, pf, handler, data, depth, false, tainted),
            b'/' => parse_pdf_name(pp, end),
            b'n' => parse_pdf_null(pp, end),
            b't' | b'f' => parse_pdf_boolean(pp, end),
            b'+' | b'-' | b'.' => parse_pdf_number(pp, end),
            b'0'..=b'9' => {
                let mut next = *pp;
                let reference = if pf.is_null() {
                    core::ptr::null_mut()
                } else {
                    try_pdf_reference(*pp, end, &mut next, pf)
                };
                if !reference.is_null() {
                    *pp = next;
                    reference
                } else {
                    parse_pdf_number(pp, end)
                }
            }
            _ => {
                if let Some(handler) = handler {
                    handler(pp, end, data)
                } else {
                    pdf_warning!("Unknown PDF object type.");
                    core::ptr::null_mut()
                }
            }
        }
    }
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_dict")]
pub unsafe extern "C-unwind" fn parse_pdf_dict(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    return parse_pdf_dict_extended(pp, endptr, pf, None, NULL);
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_array")]
pub unsafe extern "C-unwind" fn parse_pdf_array(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    return parse_pdf_array_extended(pp, endptr, pf, None, NULL);
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_object")]
pub unsafe extern "C-unwind" fn parse_pdf_object(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
) -> *mut pdf_obj {
    return parse_pdf_object_extended(pp, endptr, pf, None, NULL);
}
#[unsafe(export_name = "__pdf_impl_parse_pdf_tainted_dict")]
pub unsafe extern "C-unwind" fn parse_pdf_tainted_dict(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut unknown_handler: Option<
        unsafe extern "C-unwind" fn(
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> *mut pdf_obj,
    >,
    mut user_data: *mut ::core::ffi::c_void,
) -> *mut pdf_obj {
    unsafe {
        parse_container(
            pp,
            endptr,
            core::ptr::null_mut(),
            unknown_handler,
            user_data,
            0,
            true,
            true,
        )
    }
}
unsafe extern "C" {
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pdf_file {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_dump(
    mut start: *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { dump(start, end) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdfparse_skip_line(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdfparse_skip_line(start, end) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_skip_white(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { skip_white(start, end) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_number(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_number(start, end) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_unsigned(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_unsigned(start, end) }) {
        Ok(value) => {
            unsafe {
                crate::compat::allocation::release(value.cast());
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_ident(start, end) }) {
        Ok(value) => {
            unsafe {
                crate::compat::allocation::release(value.cast());
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_val_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_val_ident(start, end) }) {
        Ok(value) => {
            unsafe {
                crate::compat::allocation::release(value.cast());
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_opt_ident(
    mut start: *mut *const ::core::ffi::c_char,
    mut end: *const ::core::ffi::c_char,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_opt_ident(start, end) }) {
        Ok(value) => {
            unsafe {
                crate::compat::allocation::release(value.cast());
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_pdf_number(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_number(pp, endptr) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_pdf_name(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_name(pp, endptr) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_pdf_boolean(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_boolean(pp, endptr) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_pdf_null(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_null(pp, endptr) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_pdf_string(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_string(pp, endptr) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_pdf_object_extended(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
    mut unknown_handler: Option<
        unsafe extern "C-unwind" fn(
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> *mut pdf_obj,
    >,
    mut user_data: *mut ::core::ffi::c_void,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        parse_pdf_object_extended(pp, endptr, pf, unknown_handler, user_data)
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_pdf_dict(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_dict(pp, endptr, pf) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_pdf_array(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_array(pp, endptr, pf) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_pdf_object(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut pf: *mut pdf_file,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { parse_pdf_object(pp, endptr, pf) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_parse_pdf_tainted_dict(
    mut pp: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
    mut unknown_handler: Option<
        unsafe extern "C-unwind" fn(
            *mut *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> *mut pdf_obj,
    >,
    mut user_data: *mut ::core::ffi::c_void,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        parse_pdf_tainted_dict(pp, endptr, unknown_handler, user_data)
    }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

pub(crate) struct DocumentState;

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self
    }
}
