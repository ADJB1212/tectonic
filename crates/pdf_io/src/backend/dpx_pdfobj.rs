/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2007-2020 by Jin-Hwan Cho and Shunsaku Hirata,
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
use crate::backend::dpx_pdfdev::pdf_sprint_number;
unsafe extern "C-unwind" {
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn isprint(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn floor(_: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn abs(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn atof(_: *const ::core::ffi::c_char) -> ::core::ffi::c_double;
    fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strtoul(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    fn ttstub_output_open(
        path: *const ::core::ffi::c_char,
        is_gz: ::core::ffi::c_int,
    ) -> rust_output_handle_t;
    fn ttstub_output_putc(
        handle: rust_output_handle_t,
        c: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ttstub_output_write(
        handle: rust_output_handle_t,
        data: *const ::core::ffi::c_char,
        len: size_t,
    ) -> size_t;
    fn ttstub_output_close(handle: rust_output_handle_t) -> ::core::ffi::c_int;
    fn ttstub_input_get_size(handle: rust_input_handle_t) -> size_t;
    fn ttstub_input_seek(
        handle: rust_input_handle_t,
        offset: ssize_t,
        whence: ::core::ffi::c_int,
    ) -> size_t;
    fn ttstub_input_read(
        handle: rust_input_handle_t,
        data: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> ssize_t;
    fn ttstub_input_getc(handle: rust_input_handle_t) -> ::core::ffi::c_int;
    fn ttstub_input_ungetc(
        handle: rust_input_handle_t,
        ch: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut dpx_conf: _dpx_conf;
    #[link_name = "__pdf_impl_ht_init_table"]
    fn ht_init_table(ht: *mut ht_table, hval_free_fn: hval_free_func);
    #[link_name = "__pdf_impl_ht_clear_table"]
    fn ht_clear_table(ht: *mut ht_table);
    #[link_name = "__pdf_impl_ht_lookup_table"]
    fn ht_lookup_table(
        ht: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_ht_append_table"]
    fn ht_append_table(
        ht: *mut ht_table,
        key: *const ::core::ffi::c_void,
        keylen: ::core::ffi::c_int,
        value: *mut ::core::ffi::c_void,
    );
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn dpx_message(fmt: *const ::core::ffi::c_char, ...);
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_renew"]
    fn renew(p: *mut ::core::ffi::c_void, size: uint32_t) -> *mut ::core::ffi::c_void;
    static mut work_buffer: [::core::ffi::c_char; 0];
    #[link_name = "__pdf_impl_tt_mfgets"]
    fn tt_mfgets(
        buffer: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_int,
        file: rust_input_handle_t,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_pdf_doc_get_dictionary"]
    fn pdf_doc_get_dictionary(category: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_enc_init"]
    fn pdf_enc_init(
        id: *const ::core::ffi::c_uchar,
        keybits: ::core::ffi::c_int,
        permission: uint32_t,
        opasswd: *const ::core::ffi::c_char,
        upasswd: *const ::core::ffi::c_char,
        use_aes: ::core::ffi::c_int,
        encrypt_metadata: ::core::ffi::c_int,
    ) -> *mut pdf_sec;
    #[link_name = "__pdf_impl_pdf_enc_close"]
    fn pdf_enc_close(p_sec: *mut *mut pdf_sec);
    #[link_name = "__pdf_impl_pdf_enc_set_label"]
    fn pdf_enc_set_label(p_sec: *mut pdf_sec, label: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_enc_set_generation"]
    fn pdf_enc_set_generation(p_sec: *mut pdf_sec, generation: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_pdf_encrypt_data"]
    fn pdf_encrypt_data(
        p_sec: *mut pdf_sec,
        plain: *const ::core::ffi::c_uchar,
        plain_len: size_t,
        cipher: *mut *mut ::core::ffi::c_uchar,
        cipher_len: *mut size_t,
    );
    #[link_name = "__pdf_impl_pdf_enc_get_encrypt_dict"]
    fn pdf_enc_get_encrypt_dict(p_sec: *mut pdf_sec) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_enc_get_extension_dict"]
    fn pdf_enc_get_extension_dict(p_sec: *mut pdf_sec) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_skip_white"]
    fn skip_white(start: *mut *const ::core::ffi::c_char, end: *const ::core::ffi::c_char);
    #[link_name = "__pdf_impl_parse_number"]
    fn parse_number(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_unsigned"]
    fn parse_unsigned(
        start: *mut *const ::core::ffi::c_char,
        end: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "__pdf_impl_parse_pdf_dict"]
    fn parse_pdf_dict(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_parse_pdf_object"]
    fn parse_pdf_object(
        pp: *mut *const ::core::ffi::c_char,
        endptr: *const ::core::ffi::c_char,
        pf: *mut pdf_file,
    ) -> *mut pdf_obj;
    fn tectonic_flate_compress(
        output_ptr: *mut uint8_t,
        output_len: *mut uint64_t,
        input_ptr: *const uint8_t,
        input_len: uint64_t,
        compression_level: uint32_t,
    ) -> FlateResult;
    fn tectonic_flate_new_decompressor(
        input_ptr: *const uint8_t,
        input_len: uint64_t,
    ) -> *mut ::core::ffi::c_void;
    fn tectonic_flate_decompress_chunk(
        handle: *mut ::core::ffi::c_void,
        output_ptr: *mut uint8_t,
        output_len: *mut uint64_t,
    ) -> ::core::ffi::c_int;
    fn tectonic_flate_free_decompressor(handle: *mut ::core::ffi::c_void);
    fn tt_pdf_filter_png_parallel(
        raster: *const ::core::ffi::c_uchar,
        destination: *mut ::core::ffi::c_uchar,
        row_bytes: int32_t,
        rows: int32_t,
        bytes_per_pixel: ::core::ffi::c_int,
    );
    fn tt_pdf_filter_tiff_parallel(
        raster: *const ::core::ffi::c_uchar,
        destination: *mut ::core::ffi::c_uchar,
        columns: int32_t,
        rows: int32_t,
        bits_per_component: ::core::ffi::c_int,
        colors: ::core::ffi::c_int,
    );
    fn tt_pdf_compress_streams(jobs: *mut tt_pdf_compression_job, count: size_t, level: uint32_t);
    fn tt_pdf_pipeline_active() -> ::core::ffi::c_int;
    fn tt_pdf_pipeline_has_capacity(bytes: size_t) -> ::core::ffi::c_int;
    fn tt_pdf_page_submit(
        target: *mut ::core::ffi::c_void,
        compression_level: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn tt_pdf_stream_submit(
        target: *mut ::core::ffi::c_void,
        data: *const ::core::ffi::c_uchar,
        length: size_t,
        level: ::core::ffi::c_uint,
        predictor: ::core::ffi::c_int,
        columns: ::core::ffi::c_int,
        bpc: ::core::ffi::c_int,
        colors: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tt_pdf_pipeline_take(result: *mut tt_pdf_task_result) -> ::core::ffi::c_int;
    fn tt_pdf_pipeline_release(result: *mut ::core::ffi::c_void);
}
pub type __darwin_size_t = usize;
pub type __darwin_ssize_t = isize;
pub type size_t = __darwin_size_t;
pub type int8_t = i8;
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type uintptr_t = usize;
pub type ssize_t = __darwin_ssize_t;
pub type rust_input_handle_t = uintptr_t;
pub type rust_output_handle_t = uintptr_t;
pub type dpx_mode = ::core::ffi::c_uint;
pub const dpx_mode_mpost_mode: dpx_mode = 3;
pub const dpx_mode_xdv_mode: dpx_mode = 2;
pub const dpx_mode_compat_mode: dpx_mode = 1;
pub const dpx_mode_normal_mode: dpx_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dpx_conf {
    pub is_xbb: ::core::ffi::c_int,
    pub verbose_level: ::core::ffi::c_int,
    pub compat_mode: dpx_mode,
    pub ignore_font_license: ::core::ffi::c_int,
    pub file: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub keep_cache: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *mut ::core::ffi::c_char,
    pub keylen: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_void,
    pub next: *mut ht_entry,
}
pub type hval_free_func = Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_table {
    pub count: ::core::ffi::c_int,
    pub hval_free_fn: hval_free_func,
    pub table: [*mut ht_entry; 503],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_out {
    pub state: C2RustUnnamed_5,
    pub id1: [::core::ffi::c_uchar; 16],
    pub id2: [::core::ffi::c_uchar; 16],
    pub version: C2RustUnnamed_4,
    pub options: C2RustUnnamed_2,
    pub output: C2RustUnnamed_1,
    pub obj: C2RustUnnamed_0,
    pub sec_data: *mut pdf_sec,
    pub trailer: *mut pdf_obj,
    pub startxref: uint32_t,
    pub xref_table: *mut xref_entry,
    pub xref_stream: *mut pdf_obj,
    pub output_stream: *mut pdf_obj,
    pub current_objstm: *mut pdf_obj,
    pub compression_batch: [*mut pdf_obj; 64],
    pub compression_batch_count: size_t,
    pub compression_batch_bytes: size_t,
    pub flushing_compression_batch: ::core::ffi::c_int,
    pub defer_compression: ::core::ffi::c_int,
    pub completed_head: *mut completed_object,
    pub completed_tail: *mut completed_object,
    pub free_list: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct completed_object {
    pub object: *mut pdf_obj,
    pub next: *mut completed_object,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_obj {
    pub type_0: ::core::ffi::c_int,
    pub label: uint32_t,
    pub generation: uint16_t,
    pub refcount: ::core::ffi::c_int,
    pub flags: int32_t,
    pub data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xref_entry {
    pub type_0: ::core::ffi::c_uchar,
    pub field2: ::core::ffi::c_uint,
    pub field3: ::core::ffi::c_ushort,
    pub direct: *mut pdf_obj,
    pub indirect: *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub next_label: uint32_t,
    pub max_ind_objects: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub handle: rust_output_handle_t,
    pub file_position: size_t,
    pub line_position: ::core::ffi::c_int,
    pub compression_saved: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub compression: C2RustUnnamed_3,
    pub enable_encrypt: ::core::ffi::c_int,
    pub use_objstm: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub level: ::core::ffi::c_int,
    pub use_predictor: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub enc_mode: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_file {
    pub handle: rust_input_handle_t,
    pub trailer: *mut pdf_obj,
    pub xref_table: *mut xref_entry,
    pub catalog: *mut pdf_obj,
    pub num_obj: ::core::ffi::c_int,
    pub file_size: ::core::ffi::c_int,
    pub version: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_pdf_task_result {
    pub target: *mut ::core::ffi::c_void,
    pub data: *const ::core::ffi::c_uchar,
    pub length: size_t,
    pub kind: ::core::ffi::c_int,
    pub status: ::core::ffi::c_int,
    pub handle: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_indirect {
    pub pf: *mut pdf_file,
    pub obj: *mut pdf_obj,
    pub label: uint32_t,
    pub generation: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_stream {
    pub dict: *mut pdf_obj,
    pub stream: *mut ::core::ffi::c_uchar,
    pub objstm_data: *mut ::core::ffi::c_int,
    pub stream_length: size_t,
    pub max_length: size_t,
    pub _flags: int32_t,
    pub decodeparms: decode_parms,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct decode_parms {
    pub predictor: ::core::ffi::c_int,
    pub colors: ::core::ffi::c_int,
    pub bits_per_component: ::core::ffi::c_int,
    pub columns: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_dict {
    pub key: *mut pdf_obj,
    pub value: *mut pdf_obj,
    pub next: *mut pdf_dict,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_array {
    pub max: size_t,
    pub size: size_t,
    pub values: *mut *mut pdf_obj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_name {
    pub name: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_string {
    pub string: *mut ::core::ffi::c_uchar,
    pub length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_number {
    pub value: ::core::ffi::c_double,
}
pub type FlateResult = ::core::ffi::c_int;
pub const FlateResult_OtherError: FlateResult = -2;
pub const FlateResult_BufError: FlateResult = -1;
pub const FlateResult_StreamEnd: FlateResult = 1;
pub const FlateResult_Success: FlateResult = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_boolean {
    pub value: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tt_pdf_compression_job {
    pub input: *const ::core::ffi::c_uchar,
    pub input_len: uint64_t,
    pub output: *mut ::core::ffi::c_uchar,
    pub output_len: uint64_t,
    pub result: int32_t,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INVALID_HANDLE: uintptr_t = 0 as ::core::ffi::c_int as uintptr_t;
#[inline]
unsafe extern "C-unwind" fn mfree(mut ptr: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    free(ptr);
    return NULL;
}
#[inline]
unsafe extern "C-unwind" fn streq_ptr(
    mut s1: *const ::core::ffi::c_char,
    mut s2: *const ::core::ffi::c_char,
) -> bool {
    if !s1.is_null() && !s2.is_null() {
        return strcmp(s1, s2) == 0 as ::core::ffi::c_int;
    }
    return false_0 != 0;
}
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
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PDF_VERSION_MIN: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const PDF_VERSION_MAX: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const PDF_VERSION_DEFAULT: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const PDF_NUM_INDIRECT_MAX: ::core::ffi::c_ulong = 8388607 as ::core::ffi::c_ulong;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const WORK_BUFFER_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const PDF_BOOLEAN: ::core::ffi::c_int = 1;
pub const PDF_NUMBER: ::core::ffi::c_int = 2;
pub const PDF_STRING: ::core::ffi::c_int = 3;
pub const PDF_NAME: ::core::ffi::c_int = 4;
pub const PDF_ARRAY: ::core::ffi::c_int = 5;
pub const PDF_DICT: ::core::ffi::c_int = 6;
pub const PDF_STREAM: ::core::ffi::c_int = 7;
pub const PDF_NULL: ::core::ffi::c_int = 8;
pub const PDF_INDIRECT: ::core::ffi::c_int = 9;
pub const PDF_UNDEFINED: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PDF_OBJ_INVALID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STREAM_COMPRESS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const STREAM_USE_PREDICTOR: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const STREAM_PAGE_CONTENT: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const PDF_OBJ_MAX_DEPTH: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const STREAM_ALLOC_SIZE: ::core::ffi::c_uint = 4096 as ::core::ffi::c_uint;
pub const ARRAY_ALLOC_SIZE: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const IND_OBJECTS_ALLOC_SIZE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const OBJ_NO_OBJSTM: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const OBJ_NO_ENCRYPT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const OBJSTM_MAX_OBJS: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const COMPRESSION_BATCH_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const COMPRESSION_BATCH_BYTES: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
static mut pout: pdf_out = pdf_out {
    state: C2RustUnnamed_5 { enc_mode: 0 },
    id1: [0; 16],
    id2: [0; 16],
    version: C2RustUnnamed_4 { major: 0, minor: 0 },
    options: C2RustUnnamed_2 {
        compression: C2RustUnnamed_3 {
            level: 0,
            use_predictor: 0,
        },
        enable_encrypt: 0,
        use_objstm: 0,
    },
    output: C2RustUnnamed_1 {
        handle: 0,
        file_position: 0,
        line_position: 0,
        compression_saved: 0,
    },
    obj: C2RustUnnamed_0 {
        next_label: 0,
        max_ind_objects: 0,
    },
    sec_data: ::core::ptr::null::<pdf_sec>() as *mut pdf_sec,
    trailer: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
    startxref: 0,
    xref_table: ::core::ptr::null::<xref_entry>() as *mut xref_entry,
    xref_stream: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
    output_stream: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
    current_objstm: ::core::ptr::null::<pdf_obj>() as *mut pdf_obj,
    compression_batch: [::core::ptr::null::<pdf_obj>() as *mut pdf_obj; 64],
    compression_batch_count: 0,
    compression_batch_bytes: 0,
    flushing_compression_batch: 0,
    defer_compression: 0,
    completed_head: ::core::ptr::null::<completed_object>() as *mut completed_object,
    completed_tail: ::core::ptr::null::<completed_object>() as *mut completed_object,
    free_list: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
};
static mut tectonic_pout_initialized: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn current_output() -> *mut pdf_out {
    if tectonic_pout_initialized == 0 {
        init_pdf_out_struct(&raw mut pout);
    }
    return &raw mut pout;
}
unsafe extern "C-unwind" fn init_pdf_out_struct(mut p: *mut pdf_out) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*p).state.enc_mode = 0 as ::core::ffi::c_int;
    memset(
        &raw mut (*p).id1 as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        16 as size_t,
    );
    memset(
        &raw mut (*p).id2 as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        16 as size_t,
    );
    (*p).version.major = 1 as ::core::ffi::c_int;
    (*p).version.minor = PDF_VERSION_DEFAULT % 10 as ::core::ffi::c_int;
    (*p).options.compression.level = 9 as ::core::ffi::c_int;
    (*p).options.compression.use_predictor = 1 as ::core::ffi::c_int;
    (*p).options.enable_encrypt = 0 as ::core::ffi::c_int;
    (*p).options.use_objstm = 1 as ::core::ffi::c_int;
    (*p).output.handle = INVALID_HANDLE as rust_output_handle_t;
    (*p).output.file_position = 0 as size_t;
    (*p).output.line_position = 0 as ::core::ffi::c_int;
    (*p).output.compression_saved = 0 as size_t;
    (*p).obj.next_label = 1 as uint32_t;
    (*p).obj.max_ind_objects = 0 as uint32_t;
    (*p).sec_data = ::core::ptr::null_mut::<pdf_sec>();
    (*p).trailer = ::core::ptr::null_mut::<pdf_obj>();
    (*p).startxref = 0 as uint32_t;
    (*p).xref_table = ::core::ptr::null_mut::<xref_entry>();
    (*p).xref_stream = ::core::ptr::null_mut::<pdf_obj>();
    (*p).output_stream = ::core::ptr::null_mut::<pdf_obj>();
    (*p).current_objstm = ::core::ptr::null_mut::<pdf_obj>();
    (*p).compression_batch_count = 0 as size_t;
    (*p).compression_batch_bytes = 0 as size_t;
    (*p).flushing_compression_batch = 0 as ::core::ffi::c_int;
    (*p).defer_compression = 1 as ::core::ffi::c_int;
    (*p).completed_tail = ::core::ptr::null_mut::<completed_object>();
    (*p).completed_head = (*p).completed_tail;
    (*p).free_list = new(((8388607 as ::core::ffi::c_ulong)
        .wrapping_add(1 as ::core::ffi::c_ulong)
        .wrapping_div(8 as ::core::ffi::c_ulong) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    memset(
        (*p).free_list as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (PDF_NUM_INDIRECT_MAX as size_t)
            .wrapping_add(1 as size_t)
            .wrapping_div(8 as size_t),
    );
    tectonic_pout_initialized = 1 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn clean_pdf_out_struct(mut p: *mut pdf_out) {
    if !(*p).free_list.is_null() {
        free((*p).free_list as *mut ::core::ffi::c_void);
    }
    memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pdf_out>() as size_t,
    );
}
unsafe extern "C-unwind" fn pdf_out_set_compression(
    mut p: *mut pdf_out,
    mut level: ::core::ffi::c_int,
) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if level >= 0 as ::core::ffi::c_int && level <= 9 as ::core::ffi::c_int {
        (*p).options.compression.level = level;
    } else {
        pdf_abort!(
            b"set_compression: invalid compression level: %d\0" as *const u8
                as *const ::core::ffi::c_char,
            level,
        );
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_get_output_file")]
pub unsafe extern "C-unwind" fn pdf_get_output_file() -> rust_output_handle_t {
    let mut p: *mut pdf_out = current_output();
    return (*p).output.handle;
}
unsafe extern "C-unwind" fn pdf_out_set_version(
    mut p: *mut pdf_out,
    mut ver_major: ::core::ffi::c_int,
    mut ver_minor: ::core::ffi::c_int,
) {
    let mut version: ::core::ffi::c_int = 0;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    version = ver_major * 10 as ::core::ffi::c_int + ver_minor;
    if version >= PDF_VERSION_MIN && version <= PDF_VERSION_MAX {
        (*p).version.major = ver_major;
        (*p).version.minor = ver_minor;
    } else {
        pdf_warning!(
            b"Unsupported PDF version %d.%d ... Ignoring.\0" as *const u8
                as *const ::core::ffi::c_char,
            ver_major,
            ver_minor,
        );
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_get_version")]
pub unsafe extern "C-unwind" fn pdf_get_version() -> ::core::ffi::c_int {
    let mut p: *mut pdf_out = current_output();
    return (*p).version.major * 10 as ::core::ffi::c_int + (*p).version.minor;
}
#[unsafe(export_name = "__pdf_impl_pdf_get_version_major")]
pub unsafe extern "C-unwind" fn pdf_get_version_major() -> ::core::ffi::c_int {
    let mut p: *mut pdf_out = current_output();
    return (*p).version.major;
}
#[unsafe(export_name = "__pdf_impl_pdf_get_version_minor")]
pub unsafe extern "C-unwind" fn pdf_get_version_minor() -> ::core::ffi::c_int {
    let mut p: *mut pdf_out = current_output();
    return (*p).version.minor;
}
#[unsafe(export_name = "__pdf_impl_pdf_check_version")]
pub unsafe extern "C-unwind" fn pdf_check_version(
    mut major: ::core::ffi::c_int,
    mut minor: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut pdf_out = current_output();
    if (*p).version.major > major {
        return 0 as ::core::ffi::c_int;
    } else if (*p).version.major < major {
        return -(1 as ::core::ffi::c_int);
    } else {
        return if (*p).version.minor >= minor {
            0 as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        };
    };
}
unsafe extern "C-unwind" fn add_xref_entry(
    mut p: *mut pdf_out,
    mut label: uint32_t,
    mut type_0: uint8_t,
    mut field2: uint32_t,
    mut field3: uint16_t,
) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if label >= (*p).obj.max_ind_objects {
        (*p).obj.max_ind_objects = label
            .wrapping_div(IND_OBJECTS_ALLOC_SIZE as uint32_t)
            .wrapping_add(1 as uint32_t)
            .wrapping_mul(IND_OBJECTS_ALLOC_SIZE as uint32_t);
        (*p).xref_table = renew(
            (*p).xref_table as *mut ::core::ffi::c_void,
            ((*p).obj.max_ind_objects as usize)
                .wrapping_mul(::core::mem::size_of::<xref_entry>() as usize)
                as uint32_t,
        ) as *mut xref_entry;
    }
    (*(*p).xref_table.offset(label as isize)).type_0 = type_0 as ::core::ffi::c_uchar;
    (*(*p).xref_table.offset(label as isize)).field2 = field2 as ::core::ffi::c_uint;
    (*(*p).xref_table.offset(label as isize)).field3 = field3 as ::core::ffi::c_ushort;
    let ref mut fresh15 = (*(*p).xref_table.offset(label as isize)).direct;
    *fresh15 = ::core::ptr::null_mut::<pdf_obj>();
    let ref mut fresh16 = (*(*p).xref_table.offset(label as isize)).indirect;
    *fresh16 = ::core::ptr::null_mut::<pdf_obj>();
}
pub const BINARY_MARKER: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"%\xE4\xF0\xED\xF8\n\0")
};
#[unsafe(export_name = "__pdf_impl_pdf_out_init")]
pub unsafe extern "C-unwind" fn pdf_out_init(
    mut filename: *const ::core::ffi::c_char,
    mut id1: *const ::core::ffi::c_uchar,
    mut id2: *const ::core::ffi::c_uchar,
    mut ver_major: ::core::ffi::c_int,
    mut ver_minor: ::core::ffi::c_int,
    mut compression_level: ::core::ffi::c_int,
    mut enable_encrypt: ::core::ffi::c_int,
    mut enable_objstm: ::core::ffi::c_int,
    mut enable_predictor: ::core::ffi::c_int,
) -> *mut pdf_out {
    let mut p: *mut pdf_out = current_output();
    let mut v: ::core::ffi::c_char = 0;
    init_pdf_out_struct(p);
    pdf_out_set_version(p, ver_major, ver_minor);
    pdf_out_set_compression(p, compression_level);
    add_xref_entry(
        p,
        0 as uint32_t,
        0 as uint8_t,
        0 as uint32_t,
        0xffff as uint16_t,
    );
    (*p).options.enable_encrypt = enable_encrypt;
    if pdf_check_version(1 as ::core::ffi::c_int, 5 as ::core::ffi::c_int)
        == 0 as ::core::ffi::c_int
    {
        if enable_objstm != 0 {
            (*p).xref_stream = pdf_new_stream(STREAM_COMPRESS);
            (*(*p).xref_stream).flags =
                ((*(*p).xref_stream).flags as ::core::ffi::c_int | OBJ_NO_ENCRYPT) as int32_t;
            (*p).trailer = pdf_stream_dict((*p).xref_stream);
            pdf_add_dict(
                (*p).trailer,
                pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
                pdf_new_name(b"XRef\0" as *const u8 as *const ::core::ffi::c_char),
            );
            (*p).options.use_objstm = 1 as ::core::ffi::c_int;
        } else {
            (*p).xref_stream = ::core::ptr::null_mut::<pdf_obj>();
            (*p).trailer = pdf_new_dict();
            (*p).options.use_objstm = 0 as ::core::ffi::c_int;
        }
    } else {
        (*p).xref_stream = ::core::ptr::null_mut::<pdf_obj>();
        (*p).trailer = pdf_new_dict();
        (*p).options.use_objstm = 0 as ::core::ffi::c_int;
    }
    (*p).output_stream = ::core::ptr::null_mut::<pdf_obj>();
    if filename.is_null() {
        pdf_abort!(b"stdout PDF output not supported\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    (*p).output.handle = ttstub_output_open(filename, 0 as ::core::ffi::c_int);
    if (*p).output.handle == 0 {
        if strlen(filename) < 128 as size_t {
            pdf_abort!(
                b"Unable to open \"%s\".\0" as *const u8 as *const ::core::ffi::c_char,
                filename,
            );
        } else {
            pdf_abort!(b"Unable to open file.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    pdf_out_str(
        p,
        b"%PDF-\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"%PDF-\0" as *const u8 as *const ::core::ffi::c_char),
    );
    v = ('0' as i32 + (*p).version.major) as ::core::ffi::c_char;
    pdf_out_str(p, &raw mut v as *const ::core::ffi::c_void, 1 as size_t);
    pdf_out_str(
        p,
        b".\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as size_t,
    );
    v = ('0' as i32 + (*p).version.minor) as ::core::ffi::c_char;
    pdf_out_str(p, &raw mut v as *const ::core::ffi::c_void, 1 as size_t);
    pdf_out_str(
        p,
        b"\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as size_t,
    );
    pdf_out_str(
        p,
        BINARY_MARKER.as_ptr() as *const ::core::ffi::c_void,
        strlen(BINARY_MARKER.as_ptr()),
    );
    let mut id_array: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    memcpy(
        &raw mut (*p).id1 as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        id1 as *const ::core::ffi::c_void,
        16 as size_t,
    );
    memcpy(
        &raw mut (*p).id2 as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        id2 as *const ::core::ffi::c_void,
        16 as size_t,
    );
    id_array = pdf_new_array();
    pdf_add_array(
        id_array,
        pdf_new_string(
            &raw mut (*p).id1 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            16 as size_t,
        ),
    );
    pdf_add_array(
        id_array,
        pdf_new_string(
            &raw mut (*p).id2 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            16 as size_t,
        ),
    );
    pdf_add_dict(
        (*p).trailer,
        pdf_new_name(b"ID\0" as *const u8 as *const ::core::ffi::c_char),
        id_array,
    );
    (*p).state.enc_mode = 0 as ::core::ffi::c_int;
    (*p).options.compression.use_predictor = enable_predictor;
    return p;
}
#[unsafe(export_name = "__pdf_impl_pdf_out_set_encrypt")]
pub unsafe extern "C-unwind" fn pdf_out_set_encrypt(
    mut keybits: ::core::ffi::c_int,
    mut permission: int32_t,
    mut opasswd: *const ::core::ffi::c_char,
    mut upasswd: *const ::core::ffi::c_char,
    mut use_aes: ::core::ffi::c_int,
    mut encrypt_metadata: ::core::ffi::c_int,
) {
    let mut p: *mut pdf_out = current_output();
    let mut encrypt: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut extension: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut catalog: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    (*p).sec_data = pdf_enc_init(
        &raw mut (*p).id1 as *mut ::core::ffi::c_uchar,
        keybits,
        permission as uint32_t,
        opasswd,
        upasswd,
        use_aes,
        encrypt_metadata,
    );
    if (*p).sec_data.is_null() {
        (*p).options.enable_encrypt = 0 as ::core::ffi::c_int;
        return;
    }
    encrypt = pdf_enc_get_encrypt_dict((*p).sec_data);
    pdf_add_dict(
        (*p).trailer,
        pdf_new_name(b"Encrypt\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj(encrypt),
    );
    (*encrypt).flags = ((*encrypt).flags as ::core::ffi::c_int | OBJ_NO_ENCRYPT) as int32_t;
    (*encrypt).flags = ((*encrypt).flags as ::core::ffi::c_int | OBJ_NO_OBJSTM) as int32_t;
    pdf_release_obj(encrypt);
    extension = pdf_enc_get_extension_dict((*p).sec_data);
    if !extension.is_null() {
        catalog = pdf_doc_get_dictionary(b"Catalog\0" as *const u8 as *const ::core::ffi::c_char);
        pdf_add_dict(
            catalog,
            pdf_new_name(b"Extensions\0" as *const u8 as *const ::core::ffi::c_char),
            extension,
        );
    }
}
unsafe extern "C-unwind" fn dump_xref_table(mut p: *mut pdf_out) {
    let mut i: ::core::ffi::c_int = 0;
    let mut length: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 32] = [0; 32];
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_out_str(
        p,
        b"xref\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        5 as size_t,
    );
    length = sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%d %u\n\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        (*p).obj.next_label,
    );
    pdf_out_str(
        p,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        length as size_t,
    );
    i = 0 as ::core::ffi::c_int;
    while (i as uint32_t) < (*p).obj.next_label {
        let mut type_0: uint8_t = (*(*p).xref_table.offset(i as isize)).type_0 as uint8_t;
        if type_0 as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
            pdf_abort!(
                b"object type %c not allowed in xref table\0" as *const u8
                    as *const ::core::ffi::c_char,
                type_0 as ::core::ffi::c_int,
            );
        }
        length = sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%010u %05hu %c \n\0" as *const u8 as *const ::core::ffi::c_char,
            (*(*p).xref_table.offset(i as isize)).field2,
            (*(*p).xref_table.offset(i as isize)).field3 as ::core::ffi::c_int,
            if type_0 as ::core::ffi::c_int != 0 {
                'n' as i32
            } else {
                'f' as i32
            },
        );
        pdf_out_str(
            p,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            length as size_t,
        );
        i += 1;
    }
}
unsafe extern "C-unwind" fn dump_trailer(mut p: *mut pdf_out) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_out_str(
        p,
        b"trailer\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        8 as size_t,
    );
    (*p).state.enc_mode = false_0;
    write_dict(p, (*(*p).trailer).data as *mut pdf_dict);
    pdf_release_obj((*p).trailer);
    (*p).trailer = ::core::ptr::null_mut::<pdf_obj>();
    pdf_out_char(p, '\n' as i32 as ::core::ffi::c_char);
}
unsafe extern "C-unwind" fn dump_xref_stream(mut p: *mut pdf_out) {
    let mut pos: uint32_t = 0;
    let mut i: uint32_t = 0;
    let mut poslen: uint32_t = 0;
    let mut buf: [::core::ffi::c_uchar; 7] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0,
        0,
    ];
    let mut w: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pos = (*p).startxref;
    poslen = 1 as uint32_t;
    loop {
        pos >>= 8 as ::core::ffi::c_int;
        if !(pos != 0) {
            break;
        }
        poslen = poslen.wrapping_add(1);
    }
    w = pdf_new_array();
    pdf_add_array(
        w,
        pdf_new_number(1 as ::core::ffi::c_int as ::core::ffi::c_double),
    );
    pdf_add_array(w, pdf_new_number(poslen as ::core::ffi::c_double));
    pdf_add_array(
        w,
        pdf_new_number(2 as ::core::ffi::c_int as ::core::ffi::c_double),
    );
    pdf_add_dict(
        (*p).trailer,
        pdf_new_name(b"W\0" as *const u8 as *const ::core::ffi::c_char),
        w,
    );
    add_xref_entry(
        p,
        (*p).obj.next_label.wrapping_sub(1 as uint32_t),
        1 as uint8_t,
        (*p).startxref,
        0 as uint16_t,
    );
    i = 0 as uint32_t;
    while i < (*p).obj.next_label {
        let mut j: size_t = 0;
        let mut f3: uint16_t = 0;
        buf[0 as ::core::ffi::c_int as usize] = (*(*p).xref_table.offset(i as isize)).type_0;
        pos = (*(*p).xref_table.offset(i as isize)).field2 as uint32_t;
        j = poslen as size_t;
        loop {
            let fresh22 = j;
            j = j.wrapping_sub(1);
            if !(fresh22 != 0) {
                break;
            }
            buf[(1 as size_t).wrapping_add(j) as usize] = pos as ::core::ffi::c_uchar;
            pos >>= 8 as ::core::ffi::c_int;
        }
        f3 = (*(*p).xref_table.offset(i as isize)).field3 as uint16_t;
        buf[poslen.wrapping_add(1 as uint32_t) as usize] =
            (f3 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        buf[poslen.wrapping_add(2 as uint32_t) as usize] = f3 as ::core::ffi::c_uchar;
        pdf_add_stream(
            (*p).xref_stream,
            &raw mut buf as *const ::core::ffi::c_void,
            poslen.wrapping_add(3 as uint32_t) as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
    pdf_release_obj((*p).xref_stream);
    (*p).xref_stream = ::core::ptr::null_mut::<pdf_obj>();
}
#[unsafe(export_name = "__pdf_impl_pdf_out_flush")]
pub unsafe extern "C-unwind" fn pdf_out_flush() {
    let mut p: *mut pdf_out = current_output();
    let mut buf: [::core::ffi::c_char; 16] = [0; 16];
    if (*p).output.handle != 0 {
        let mut length: ::core::ffi::c_int = 0;
        (*p).defer_compression = 0 as ::core::ffi::c_int;
        if !(*p).current_objstm.is_null() {
            release_objstm((*p).current_objstm);
            (*p).current_objstm = ::core::ptr::null_mut::<pdf_obj>();
        }
        flush_compression_batch(p);
        pdf_pipeline_drain(1 as ::core::ffi::c_int);
        if !(*p).xref_stream.is_null() {
            pdf_label_obj(p, (*p).xref_stream);
        }
        (*p).startxref = (*p).output.file_position as uint32_t;
        pdf_add_dict(
            (*p).trailer,
            pdf_new_name(b"Size\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number((*p).obj.next_label as ::core::ffi::c_double),
        );
        if !(*p).xref_stream.is_null() {
            dump_xref_stream(p);
        } else {
            dump_xref_table(p);
            dump_trailer(p);
        }
        free((*p).xref_table as *mut ::core::ffi::c_void);
        (*p).xref_table = ::core::ptr::null_mut::<xref_entry>();
        pdf_out_str(
            p,
            b"startxref\n\0" as *const u8 as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            10 as size_t,
        );
        length = sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%u\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).startxref,
        );
        pdf_out_str(
            p,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            length as size_t,
        );
        pdf_out_str(
            p,
            b"%%EOF\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            6 as size_t,
        );
        if dpx_conf.verbose_level > 0 as ::core::ffi::c_int {
            if (*p).options.compression.level > 0 as ::core::ffi::c_int {
                pdf_message!(
                    b"Compression saved %zu bytes\n\0" as *const u8 as *const ::core::ffi::c_char,
                    (*p).output.compression_saved,
                );
            }
        }
        pdf_message!(
            b"%zu bytes written\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).output.file_position,
        );
        ttstub_output_close((*p).output.handle);
        (*p).output.handle = INVALID_HANDLE as rust_output_handle_t;
        (*p).output.file_position = 0 as size_t;
        (*p).output.line_position = 0 as ::core::ffi::c_int;
    }
    if !(*p).sec_data.is_null() {
        pdf_enc_close(&raw mut (*p).sec_data);
    }
    clean_pdf_out_struct(p);
}
#[unsafe(export_name = "__pdf_impl_pdf_error_cleanup")]
pub unsafe extern "C-unwind" fn pdf_error_cleanup() {
    let mut p: *mut pdf_out = current_output();
    if (*p).output.handle != 0 {
        ttstub_output_close((*p).output.handle);
        (*p).output.handle = INVALID_HANDLE as rust_output_handle_t;
    }
    (*p).defer_compression = 0 as ::core::ffi::c_int;
    while (*p).compression_batch_count != 0 {
        (*p).compression_batch_count = (*p).compression_batch_count.wrapping_sub(1);
        let mut object: *mut pdf_obj =
            (*p).compression_batch[(*p).compression_batch_count as usize];
        (*object).label = 0 as uint32_t;
        pdf_release_obj(object);
    }
    (*p).compression_batch_bytes = 0 as size_t;
    while !(*p).completed_head.is_null() {
        let mut entry: *mut completed_object = (*p).completed_head;
        (*p).completed_head = (*entry).next as *mut completed_object;
        (*(*entry).object).label = 0 as uint32_t;
        pdf_release_obj((*entry).object);
        free(entry as *mut ::core::ffi::c_void);
    }
    (*p).completed_tail = ::core::ptr::null_mut::<completed_object>();
    let mut result: tt_pdf_task_result = tt_pdf_task_result {
        target: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data: ::core::ptr::null::<::core::ffi::c_uchar>(),
        length: 0,
        kind: 0,
        status: 0,
        handle: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    while tt_pdf_pipeline_take(&raw mut result) != 0 {
        let mut object_0: *mut pdf_obj = result.target as *mut pdf_obj;
        tt_pdf_pipeline_release(result.handle);
        (*object_0).label = 0 as uint32_t;
        pdf_release_obj(object_0);
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_set_root")]
pub unsafe extern "C-unwind" fn pdf_set_root(mut object: *mut pdf_obj) {
    let mut p: *mut pdf_out = current_output();
    if !pdf_lookup_dict(
        (*p).trailer,
        b"Root\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        pdf_abort!(b"Root object already set!\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    pdf_add_dict(
        (*p).trailer,
        pdf_new_name(b"Root\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj(object),
    );
    if (*p).options.enable_encrypt != 0 {
        (*object).flags = ((*object).flags as ::core::ffi::c_int | OBJ_NO_OBJSTM) as int32_t;
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_set_info")]
pub unsafe extern "C-unwind" fn pdf_set_info(mut object: *mut pdf_obj) {
    let mut p: *mut pdf_out = current_output();
    if !pdf_lookup_dict(
        (*p).trailer,
        b"Info\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        pdf_abort!(b"Info object already set!\0" as *const u8 as *const ::core::ffi::c_char,);
    }
    pdf_add_dict(
        (*p).trailer,
        pdf_new_name(b"Info\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_ref_obj(object),
    );
}
unsafe extern "C-unwind" fn pdf_out_char(mut p: *mut pdf_out, mut c: ::core::ffi::c_char) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(*p).output_stream.is_null() {
        pdf_add_stream(
            (*p).output_stream,
            &raw mut c as *const ::core::ffi::c_void,
            1 as ::core::ffi::c_int,
        );
    } else {
        ttstub_output_putc((*p).output.handle, c as ::core::ffi::c_int);
        (*p).output.file_position = (*p).output.file_position.wrapping_add(1 as size_t);
        if c as ::core::ffi::c_int == '\n' as i32 {
            (*p).output.line_position = 0 as ::core::ffi::c_int;
        } else {
            (*p).output.line_position += 1 as ::core::ffi::c_int;
        }
    };
}
static mut xchar: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
};
unsafe extern "C-unwind" fn pdf_out_xchar(mut p: *mut pdf_out, mut c: ::core::ffi::c_char) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_out_char(
        p,
        xchar[(c as ::core::ffi::c_int >> 4 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
            as usize],
    );
    pdf_out_char(
        p,
        xchar[(c as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize],
    );
}
unsafe extern "C-unwind" fn pdf_out_str(
    mut p: *mut pdf_out,
    mut buffer: *const ::core::ffi::c_void,
    mut length: size_t,
) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(*p).output_stream.is_null() {
        pdf_add_stream((*p).output_stream, buffer, length as ::core::ffi::c_int);
    } else {
        ttstub_output_write(
            (*p).output.handle,
            buffer as *const ::core::ffi::c_char,
            length,
        );
        (*p).output.file_position = (*p).output.file_position.wrapping_add(length);
        (*p).output.line_position = ((*p).output.line_position as size_t).wrapping_add(length)
            as ::core::ffi::c_int as ::core::ffi::c_int;
        if length > 0 as size_t
            && *(buffer as *const ::core::ffi::c_char)
                .offset(length.wrapping_sub(1 as size_t) as isize)
                as ::core::ffi::c_int
                == '\n' as i32
        {
            (*p).output.line_position = 0 as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C-unwind" fn pdf_need_white(
    mut type1: ::core::ffi::c_int,
    mut type2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return !(type1 == PDF_STRING
        || type1 == PDF_ARRAY
        || type1 == PDF_DICT
        || type2 == PDF_STRING
        || type2 == PDF_NAME
        || type2 == PDF_ARRAY
        || type2 == PDF_DICT) as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdf_out_white(mut p: *mut pdf_out) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*p).output.line_position >= 80 as ::core::ffi::c_int {
        pdf_out_char(p, '\n' as i32 as ::core::ffi::c_char);
    } else {
        pdf_out_char(p, ' ' as i32 as ::core::ffi::c_char);
    };
}
unsafe extern "C-unwind" fn pdf_new_obj(mut type_0: ::core::ffi::c_int) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if type_0 > PDF_UNDEFINED || type_0 < 0 as ::core::ffi::c_int {
        pdf_abort!(
            b"Invalid object type: %d\0" as *const u8 as *const ::core::ffi::c_char,
            type_0,
        );
    }
    result = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_obj>() as usize) as uint32_t)
        as *mut pdf_obj;
    (*result).type_0 = type_0;
    (*result).data = NULL;
    (*result).label = 0 as uint32_t;
    (*result).generation = 0 as uint16_t;
    (*result).refcount = 1 as ::core::ffi::c_int;
    (*result).flags = 0 as ::core::ffi::c_int as int32_t;
    return result;
}
#[unsafe(export_name = "__pdf_impl_pdf_obj_typeof")]
pub unsafe extern "C-unwind" fn pdf_obj_typeof(mut object: *mut pdf_obj) -> ::core::ffi::c_int {
    if object.is_null()
        || (*object).type_0 <= 0 as ::core::ffi::c_int
        || (*object).type_0 > PDF_UNDEFINED
    {
        return PDF_OBJ_INVALID;
    }
    return (*object).type_0;
}
unsafe extern "C-unwind" fn pdf_label_obj(mut p: *mut pdf_out, mut object: *mut pdf_obj) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if object.is_null()
        || (*object).type_0 <= 0 as ::core::ffi::c_int
        || (*object).type_0 > PDF_UNDEFINED
    {
        pdf_abort!(
            b"pdf_label_obj(): passed invalid object.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*object).label == 0 as uint32_t {
        if (*p).obj.next_label as ::core::ffi::c_ulong == PDF_NUM_INDIRECT_MAX {
            pdf_abort!(
                b"Number of indirect object has reached its maximum value!\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        let fresh17 = (*p).obj.next_label;
        (*p).obj.next_label = (*p).obj.next_label.wrapping_add(1);
        (*object).label = fresh17;
        (*object).generation = 0 as uint16_t;
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_transfer_label")]
pub unsafe extern "C-unwind" fn pdf_transfer_label(mut dst: *mut pdf_obj, mut src: *mut pdf_obj) {
    if !(!dst.is_null() && (*dst).label == 0 && !src.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"dst && !dst->label && src\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*dst).label = (*src).label;
    (*dst).generation = (*src).generation;
    (*src).label = 0 as uint32_t;
    (*src).generation = 0 as uint16_t;
}
#[unsafe(export_name = "__pdf_impl_pdf_link_obj")]
pub unsafe extern "C-unwind" fn pdf_link_obj(mut object: *mut pdf_obj) -> *mut pdf_obj {
    if object.is_null()
        || (*object).type_0 <= 0 as ::core::ffi::c_int
        || (*object).type_0 > PDF_UNDEFINED
    {
        pdf_abort!(
            b"pdf_link_obj(): passed invalid object.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    (*object).refcount += 1 as ::core::ffi::c_int;
    return object;
}
#[unsafe(export_name = "__pdf_impl_pdf_ref_obj")]
pub unsafe extern "C-unwind" fn pdf_ref_obj(mut object: *mut pdf_obj) -> *mut pdf_obj {
    let mut p: *mut pdf_out = current_output();
    if object.is_null()
        || (*object).type_0 <= 0 as ::core::ffi::c_int
        || (*object).type_0 > PDF_UNDEFINED
    {
        pdf_abort!(
            b"pdf_ref_obj(): passed invalid object.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*object).refcount == 0 as ::core::ffi::c_int {
        pdf_message!(
            b"\nTrying to refer already released object!!!\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        pdf_abort!(b"Cannot continue...\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !object.is_null() && pdf_obj_typeof(object) == PDF_INDIRECT {
        return pdf_link_obj(object);
    } else {
        return pdf_new_ref(p, object);
    };
}
unsafe extern "C-unwind" fn release_indirect(mut data: *mut pdf_indirect) {
    free(data as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn write_indirect(mut p: *mut pdf_out, mut indirect: *mut pdf_indirect) {
    let mut length: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 64] = [0; 64];
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(*indirect).pf.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"!indirect->pf\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    length = sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%u %hu R\0" as *const u8 as *const ::core::ffi::c_char,
        (*indirect).label,
        (*indirect).generation as ::core::ffi::c_int,
    );
    pdf_out_str(
        p,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        length as size_t,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_new_undefined")]
pub unsafe extern "C-unwind" fn pdf_new_undefined() -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    result = pdf_new_obj(PDF_UNDEFINED);
    (*result).data = NULL;
    return result;
}
#[unsafe(export_name = "__pdf_impl_pdf_new_null")]
pub unsafe extern "C-unwind" fn pdf_new_null() -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    result = pdf_new_obj(PDF_NULL);
    (*result).data = NULL;
    return result;
}
unsafe extern "C-unwind" fn write_null(mut p: *mut pdf_out) {
    pdf_out_str(
        p,
        b"null\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        4 as size_t,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_new_boolean")]
pub unsafe extern "C-unwind" fn pdf_new_boolean(mut value: ::core::ffi::c_char) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut data: *mut pdf_boolean = ::core::ptr::null_mut::<pdf_boolean>();
    result = pdf_new_obj(PDF_BOOLEAN);
    data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_boolean>() as usize) as uint32_t)
        as *mut pdf_boolean;
    (*data).value = value;
    (*result).data = data as *mut ::core::ffi::c_void;
    return result;
}
unsafe extern "C-unwind" fn release_boolean(mut data: *mut pdf_obj) {
    free(data as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn write_boolean(mut p: *mut pdf_out, mut data: *mut pdf_boolean) {
    if (*data).value != 0 {
        pdf_out_str(
            p,
            b"true\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
    } else {
        pdf_out_str(
            p,
            b"false\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            5 as size_t,
        );
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_boolean_value")]
pub unsafe extern "C-unwind" fn pdf_boolean_value(mut object: *mut pdf_obj) -> ::core::ffi::c_char {
    let mut data: *mut pdf_boolean = ::core::ptr::null_mut::<pdf_boolean>();
    if object.is_null() || (*object).type_0 != 1 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            1 as ::core::ffi::c_int,
            1057 as ::core::ffi::c_int,
        );
    }
    data = (*object).data as *mut pdf_boolean;
    return (*data).value;
}
#[unsafe(export_name = "__pdf_impl_pdf_new_number")]
pub unsafe extern "C-unwind" fn pdf_new_number(mut value: ::core::ffi::c_double) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut data: *mut pdf_number = ::core::ptr::null_mut::<pdf_number>();
    result = pdf_new_obj(PDF_NUMBER);
    data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_number>() as usize) as uint32_t)
        as *mut pdf_number;
    (*data).value = value;
    (*result).data = data as *mut ::core::ffi::c_void;
    return result;
}
unsafe extern "C-unwind" fn release_number(mut data: *mut pdf_number) {
    free(data as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn write_number(mut p: *mut pdf_out, mut number: *mut pdf_number) {
    let mut count: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 512] = [0; 512];
    count = pdf_sprint_number(&raw mut buf as *mut ::core::ffi::c_char, (*number).value);
    pdf_out_str(
        p,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        count as size_t,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_set_number")]
pub unsafe extern "C-unwind" fn pdf_set_number(
    mut object: *mut pdf_obj,
    mut value: ::core::ffi::c_double,
) {
    let mut data: *mut pdf_number = ::core::ptr::null_mut::<pdf_number>();
    if object.is_null() || (*object).type_0 != 2 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            2 as ::core::ffi::c_int,
            1101 as ::core::ffi::c_int,
        );
    }
    data = (*object).data as *mut pdf_number;
    (*data).value = value;
}
#[unsafe(export_name = "__pdf_impl_pdf_number_value")]
pub unsafe extern "C-unwind" fn pdf_number_value(
    mut object: *mut pdf_obj,
) -> ::core::ffi::c_double {
    let mut data: *mut pdf_number = ::core::ptr::null_mut::<pdf_number>();
    if object.is_null() || (*object).type_0 != 2 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            2 as ::core::ffi::c_int,
            1112 as ::core::ffi::c_int,
        );
    }
    data = (*object).data as *mut pdf_number;
    return (*data).value;
}
#[unsafe(export_name = "__pdf_impl_pdf_new_string")]
pub unsafe extern "C-unwind" fn pdf_new_string(
    mut str: *const ::core::ffi::c_void,
    mut length: size_t,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut data: *mut pdf_string = ::core::ptr::null_mut::<pdf_string>();
    if str.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"str\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    result = pdf_new_obj(PDF_STRING);
    data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_string>() as usize) as uint32_t)
        as *mut pdf_string;
    (*result).data = data as *mut ::core::ffi::c_void;
    (*data).length = length;
    if length != 0 {
        (*data).string = new((length.wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        memcpy((*data).string as *mut ::core::ffi::c_void, str, length);
        *(*data).string.offset(length as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    } else {
        (*data).string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    }
    return result;
}
#[unsafe(export_name = "__pdf_impl_pdf_string_value")]
pub unsafe extern "C-unwind" fn pdf_string_value(
    mut object: *mut pdf_obj,
) -> *mut ::core::ffi::c_void {
    let mut data: *mut pdf_string = ::core::ptr::null_mut::<pdf_string>();
    if object.is_null() || (*object).type_0 != 3 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            3 as ::core::ffi::c_int,
            1149 as ::core::ffi::c_int,
        );
    }
    data = (*object).data as *mut pdf_string;
    return (*data).string as *mut ::core::ffi::c_void;
}
#[unsafe(export_name = "__pdf_impl_pdf_string_length")]
pub unsafe extern "C-unwind" fn pdf_string_length(mut object: *mut pdf_obj) -> ::core::ffi::c_uint {
    let mut data: *mut pdf_string = ::core::ptr::null_mut::<pdf_string>();
    if object.is_null() || (*object).type_0 != 3 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            3 as ::core::ffi::c_int,
            1161 as ::core::ffi::c_int,
        );
    }
    data = (*object).data as *mut pdf_string;
    return (*data).length as ::core::ffi::c_uint;
}
#[unsafe(export_name = "__pdf_impl_pdfobj_escape_str")]
pub unsafe extern "C-unwind" fn pdfobj_escape_str(
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
    mut s: *const ::core::ffi::c_uchar,
    mut len: size_t,
) -> size_t {
    let mut result: size_t = 0 as size_t;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < len {
        let mut ch: ::core::ffi::c_uchar = 0;
        ch = *s.offset(i as isize);
        if result > bufsize.wrapping_sub(4 as size_t) {
            pdf_abort!(
                b"pdfobj_escape_str: Buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if (ch as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
            || ch as ::core::ffi::c_int > 126 as ::core::ffi::c_int
        {
            let fresh7 = result;
            result = result.wrapping_add(1);
            *buffer.offset(fresh7 as isize) = '\\' as i32 as ::core::ffi::c_char;
            result = result.wrapping_add(sprintf(
                buffer.offset(result as isize),
                b"%03o\0" as *const u8 as *const ::core::ffi::c_char,
                ch as ::core::ffi::c_int,
            ) as size_t);
        } else {
            match ch as ::core::ffi::c_int {
                40 => {
                    let fresh8 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh8 as isize) = '\\' as i32 as ::core::ffi::c_char;
                    let fresh9 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh9 as isize) = '(' as i32 as ::core::ffi::c_char;
                }
                41 => {
                    let fresh10 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh10 as isize) = '\\' as i32 as ::core::ffi::c_char;
                    let fresh11 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh11 as isize) = ')' as i32 as ::core::ffi::c_char;
                }
                92 => {
                    let fresh12 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh12 as isize) = '\\' as i32 as ::core::ffi::c_char;
                    let fresh13 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh13 as isize) = '\\' as i32 as ::core::ffi::c_char;
                }
                _ => {
                    let fresh14 = result;
                    result = result.wrapping_add(1);
                    *buffer.offset(fresh14 as isize) = ch as ::core::ffi::c_char;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return result;
}
unsafe extern "C-unwind" fn write_string(mut p: *mut pdf_out, mut str: *mut pdf_string) {
    let mut s: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut i: size_t = 0;
    let mut nescc: size_t = 0 as size_t;
    let mut len: size_t = 0 as size_t;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*p).state.enc_mode != 0 {
        pdf_encrypt_data(
            (*p).sec_data,
            (*str).string,
            (*str).length,
            &raw mut s,
            &raw mut len,
        );
    } else {
        s = (*str).string;
        len = (*str).length;
    }
    i = 0 as size_t;
    while i < len {
        if isprint(*s.offset(i as isize) as ::core::ffi::c_int) == 0 {
            nescc = nescc.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if nescc > len.wrapping_div(3 as size_t) {
        pdf_out_char(p, '<' as i32 as ::core::ffi::c_char);
        i = 0 as size_t;
        while i < len {
            pdf_out_xchar(p, *s.offset(i as isize) as ::core::ffi::c_char);
            i = i.wrapping_add(1);
        }
        pdf_out_char(p, '>' as i32 as ::core::ffi::c_char);
    } else {
        let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut size: size_t = 0;
        let mut count: size_t = 0;
        size = len.wrapping_mul(2 as size_t).wrapping_add(3 as size_t);
        buf = new((size as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        pdf_out_char(p, '(' as i32 as ::core::ffi::c_char);
        i = 0 as size_t;
        while i < len {
            count = pdfobj_escape_str(
                buf,
                size,
                s.offset(i as isize) as *mut ::core::ffi::c_uchar,
                1 as size_t,
            );
            pdf_out_str(p, buf as *const ::core::ffi::c_void, count);
            i = i.wrapping_add(1);
        }
        pdf_out_char(p, ')' as i32 as ::core::ffi::c_char);
        free(buf as *mut ::core::ffi::c_void);
    }
    if (*p).state.enc_mode != 0 && !s.is_null() {
        free(s as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C-unwind" fn release_string(mut data: *mut pdf_string) {
    (*data).string = mfree((*data).string as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_uchar;
    free(data as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_pdf_set_string")]
pub unsafe extern "C-unwind" fn pdf_set_string(
    mut object: *mut pdf_obj,
    mut str: *mut ::core::ffi::c_uchar,
    mut length: size_t,
) {
    let mut data: *mut pdf_string = ::core::ptr::null_mut::<pdf_string>();
    if object.is_null() || (*object).type_0 != 3 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            3 as ::core::ffi::c_int,
            1288 as ::core::ffi::c_int,
        );
    }
    data = (*object).data as *mut pdf_string;
    if !(*data).string.is_null() {
        free((*data).string as *mut ::core::ffi::c_void);
    }
    if length != 0 as size_t {
        (*data).length = length;
        (*data).string = new((length.wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        memcpy(
            (*data).string as *mut ::core::ffi::c_void,
            str as *const ::core::ffi::c_void,
            length,
        );
        *(*data).string.offset(length as isize) = '\0' as i32 as ::core::ffi::c_uchar;
    } else {
        (*data).length = 0 as size_t;
        (*data).string = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    };
}
#[unsafe(export_name = "__pdf_impl_pdf_new_name")]
pub unsafe extern "C-unwind" fn pdf_new_name(mut name: *const ::core::ffi::c_char) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut length: size_t = 0;
    let mut data: *mut pdf_name = ::core::ptr::null_mut::<pdf_name>();
    result = pdf_new_obj(PDF_NAME);
    data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_name>() as usize) as uint32_t)
        as *mut pdf_name;
    (*result).data = data as *mut ::core::ffi::c_void;
    length = strlen(name);
    if length != 0 as size_t {
        (*data).name = new((length.wrapping_add(1 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t) as *mut ::core::ffi::c_char;
        memcpy(
            (*data).name as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            length,
        );
        *(*data).name.offset(length as isize) = '\0' as i32 as ::core::ffi::c_char;
    } else {
        (*data).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return result;
}
unsafe extern "C-unwind" fn write_name(mut p: *mut pdf_out, mut name: *mut pdf_name) {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: size_t = 0;
    let mut length: size_t = 0;
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    s = (*name).name;
    length = if !(*name).name.is_null() {
        strlen((*name).name)
    } else {
        0 as size_t
    };
    pdf_out_char(p, '/' as i32 as ::core::ffi::c_char);
    i = 0 as size_t;
    while i < length {
        if (*s.offset(i as isize) as ::core::ffi::c_int) < '!' as i32
            || *s.offset(i as isize) as ::core::ffi::c_int > '~' as i32
            || *s.offset(i as isize) as ::core::ffi::c_int == '#' as i32
            || (*s.offset(i as isize) as ::core::ffi::c_int == '(' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == ')' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '/' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '<' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '>' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '[' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == ']' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '{' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '}' as i32
                || *s.offset(i as isize) as ::core::ffi::c_int == '%' as i32)
        {
            pdf_out_char(p, '#' as i32 as ::core::ffi::c_char);
            pdf_out_xchar(p, *s.offset(i as isize));
        } else {
            pdf_out_char(p, *s.offset(i as isize));
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C-unwind" fn release_name(mut data: *mut pdf_name) {
    (*data).name = mfree((*data).name as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_char;
    free(data as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_pdf_name_value")]
pub unsafe extern "C-unwind" fn pdf_name_value(
    mut object: *mut pdf_obj,
) -> *mut ::core::ffi::c_char {
    let mut data: *mut pdf_name = ::core::ptr::null_mut::<pdf_name>();
    if object.is_null() || (*object).type_0 != 4 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !object.is_null() {
                (*object).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            4 as ::core::ffi::c_int,
            1382 as ::core::ffi::c_int,
        );
    }
    data = (*object).data as *mut pdf_name;
    return (*data).name;
}
#[unsafe(export_name = "__pdf_impl_pdf_new_array")]
pub unsafe extern "C-unwind" fn pdf_new_array() -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut data: *mut pdf_array = ::core::ptr::null_mut::<pdf_array>();
    result = pdf_new_obj(PDF_ARRAY);
    data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_array>() as usize) as uint32_t)
        as *mut pdf_array;
    (*data).values = ::core::ptr::null_mut::<*mut pdf_obj>();
    (*data).max = 0 as size_t;
    (*data).size = 0 as size_t;
    (*result).data = data as *mut ::core::ffi::c_void;
    return result;
}
unsafe extern "C-unwind" fn write_array(mut p: *mut pdf_out, mut array: *mut pdf_array) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pdf_out_char(p, '[' as i32 as ::core::ffi::c_char);
    if (*array).size > 0 as size_t {
        let mut i: size_t = 0;
        let mut type1: ::core::ffi::c_int = PDF_UNDEFINED;
        let mut type2: ::core::ffi::c_int = 0;
        i = 0 as size_t;
        while i < (*array).size {
            if !(*(*array).values.offset(i as isize)).is_null() {
                type2 = (**(*array).values.offset(i as isize)).type_0;
                if type1 != PDF_UNDEFINED && pdf_need_white(type1, type2) != 0 {
                    pdf_out_white(p);
                }
                type1 = type2;
                pdf_write_obj(p, *(*array).values.offset(i as isize) as *mut pdf_obj);
            } else {
                pdf_warning!(
                    b"PDF array element %zu undefined.\0" as *const u8
                        as *const ::core::ffi::c_char,
                    i,
                );
            }
            i = i.wrapping_add(1);
        }
    }
    pdf_out_char(p, ']' as i32 as ::core::ffi::c_char);
}
#[unsafe(export_name = "__pdf_impl_pdf_get_array")]
pub unsafe extern "C-unwind" fn pdf_get_array(
    mut array: *mut pdf_obj,
    mut idx: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut data: *mut pdf_array = ::core::ptr::null_mut::<pdf_array>();
    if array.is_null() || (*array).type_0 != 5 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !array.is_null() {
                (*array).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            5 as ::core::ffi::c_int,
            1440 as ::core::ffi::c_int,
        );
    }
    data = (*array).data as *mut pdf_array;
    if idx < 0 as ::core::ffi::c_int {
        result = *(*data)
            .values
            .offset((idx as size_t).wrapping_add((*data).size) as isize)
            as *mut pdf_obj;
    } else if (idx as size_t) < (*data).size {
        result = *(*data).values.offset(idx as isize) as *mut pdf_obj;
    }
    return result;
}
#[unsafe(export_name = "__pdf_impl_pdf_array_length")]
pub unsafe extern "C-unwind" fn pdf_array_length(mut array: *mut pdf_obj) -> ::core::ffi::c_uint {
    let mut data: *mut pdf_array = ::core::ptr::null_mut::<pdf_array>();
    if array.is_null() || (*array).type_0 != 5 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !array.is_null() {
                (*array).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            5 as ::core::ffi::c_int,
            1457 as ::core::ffi::c_int,
        );
    }
    data = (*array).data as *mut pdf_array;
    return (*data).size as ::core::ffi::c_uint;
}
unsafe extern "C-unwind" fn release_array(mut data: *mut pdf_array) {
    let mut i: size_t = 0;
    if !(*data).values.is_null() {
        i = 0 as size_t;
        while i < (*data).size {
            pdf_release_obj(*(*data).values.offset(i as isize) as *mut pdf_obj);
            let ref mut fresh2 = *(*data).values.offset(i as isize);
            *fresh2 = ::core::ptr::null_mut::<pdf_obj>();
            i = i.wrapping_add(1);
        }
        (*data).values = mfree((*data).values as *mut ::core::ffi::c_void) as *mut *mut pdf_obj;
    }
    free(data as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_pdf_add_array")]
pub unsafe extern "C-unwind" fn pdf_add_array(mut array: *mut pdf_obj, mut object: *mut pdf_obj) {
    let mut data: *mut pdf_array = ::core::ptr::null_mut::<pdf_array>();
    if array.is_null() || (*array).type_0 != 5 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !array.is_null() {
                (*array).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            5 as ::core::ffi::c_int,
            1488 as ::core::ffi::c_int,
        );
    }
    data = (*array).data as *mut pdf_array;
    if (*data).size >= (*data).max {
        (*data).max = (*data).max.wrapping_add(ARRAY_ALLOC_SIZE as size_t);
        (*data).values = renew(
            (*data).values as *mut ::core::ffi::c_void,
            ((*data).max as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<*mut pdf_obj>() as usize)
                as uint32_t,
        ) as *mut *mut pdf_obj as *mut *mut pdf_obj;
    }
    let ref mut fresh20 = *(*data).values.offset((*data).size as isize);
    *fresh20 = object as *mut pdf_obj;
    (*data).size = (*data).size.wrapping_add(1);
}
unsafe extern "C-unwind" fn pdf_unshift_array(mut array: *mut pdf_obj, mut object: *mut pdf_obj) {
    let mut data: *mut pdf_array = ::core::ptr::null_mut::<pdf_array>();
    if array.is_null() || (*array).type_0 != 5 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !array.is_null() {
                (*array).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            5 as ::core::ffi::c_int,
            1508 as ::core::ffi::c_int,
        );
    }
    data = (*array).data as *mut pdf_array;
    if (*data).size >= (*data).max {
        (*data).max = (*data).max.wrapping_add(ARRAY_ALLOC_SIZE as size_t);
        (*data).values = renew(
            (*data).values as *mut ::core::ffi::c_void,
            ((*data).max as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<*mut pdf_obj>() as usize)
                as uint32_t,
        ) as *mut *mut pdf_obj as *mut *mut pdf_obj;
    }
    memmove(
        (*data).values.offset(1 as ::core::ffi::c_int as isize) as *mut *mut pdf_obj
            as *mut ::core::ffi::c_void,
        (*data).values as *const ::core::ffi::c_void,
        (*data)
            .size
            .wrapping_mul(::core::mem::size_of::<*mut pdf_obj>() as size_t),
    );
    let ref mut fresh6 = *(*data).values.offset(0 as ::core::ffi::c_int as isize);
    *fresh6 = object as *mut pdf_obj;
    (*data).size = (*data).size.wrapping_add(1);
}
unsafe extern "C-unwind" fn write_dict(mut p: *mut pdf_out, mut dict: *mut pdf_dict) {
    pdf_out_str(
        p,
        b"<<\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        2 as size_t,
    );
    while !(*dict).key.is_null() {
        pdf_write_obj(p, (*dict).key as *mut pdf_obj);
        if pdf_need_white(PDF_NAME, (*(*dict).value).type_0) != 0 {
            pdf_out_white(p);
        }
        pdf_write_obj(p, (*dict).value as *mut pdf_obj);
        dict = (*dict).next as *mut pdf_dict;
    }
    pdf_out_str(
        p,
        b">>\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        2 as size_t,
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_new_dict")]
pub unsafe extern "C-unwind" fn pdf_new_dict() -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut data: *mut pdf_dict = ::core::ptr::null_mut::<pdf_dict>();
    result = pdf_new_obj(PDF_DICT);
    data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_dict>() as usize) as uint32_t)
        as *mut pdf_dict;
    (*data).key = ::core::ptr::null_mut::<pdf_obj>();
    (*data).value = ::core::ptr::null_mut::<pdf_obj>();
    (*data).next = ::core::ptr::null_mut::<pdf_dict>();
    (*result).data = data as *mut ::core::ffi::c_void;
    return result;
}
unsafe extern "C-unwind" fn release_dict(mut data: *mut pdf_dict) {
    let mut next: *mut pdf_dict = ::core::ptr::null_mut::<pdf_dict>();
    while !data.is_null() && !(*data).key.is_null() {
        pdf_release_obj((*data).key as *mut pdf_obj);
        pdf_release_obj((*data).value as *mut pdf_obj);
        (*data).key = ::core::ptr::null_mut::<pdf_obj>();
        (*data).value = ::core::ptr::null_mut::<pdf_obj>();
        next = (*data).next as *mut pdf_dict;
        free(data as *mut ::core::ffi::c_void);
        data = next;
    }
    free(data as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_pdf_add_dict")]
pub unsafe extern "C-unwind" fn pdf_add_dict(
    mut dict: *mut pdf_obj,
    mut key: *mut pdf_obj,
    mut value: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut data: *mut pdf_dict = ::core::ptr::null_mut::<pdf_dict>();
    let mut new_node: *mut pdf_dict = ::core::ptr::null_mut::<pdf_dict>();
    if dict.is_null() || (*dict).type_0 != 6 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !dict.is_null() {
                (*dict).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            6 as ::core::ffi::c_int,
            1577 as ::core::ffi::c_int,
        );
    }
    if key.is_null() || (*key).type_0 != 4 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !key.is_null() {
                (*key).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            4 as ::core::ffi::c_int,
            1578 as ::core::ffi::c_int,
        );
    }
    if !value.is_null()
        && (value.is_null()
            || (*value).type_0 <= 0 as ::core::ffi::c_int
            || (*value).type_0 > PDF_UNDEFINED)
    {
        pdf_abort!(
            b"pdf_add_dict(): Passed invalid value\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    data = (*dict).data as *mut pdf_dict;
    while !(*data).key.is_null() {
        if streq_ptr(
            pdf_name_value(key),
            pdf_name_value((*data).key as *mut pdf_obj),
        ) {
            pdf_release_obj((*data).value as *mut pdf_obj);
            pdf_release_obj(key);
            (*data).value = value as *mut pdf_obj;
            return 1 as ::core::ffi::c_int;
        }
        data = (*data).next as *mut pdf_dict;
    }
    new_node = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_dict>() as usize) as uint32_t)
        as *mut pdf_dict;
    (*new_node).key = ::core::ptr::null_mut::<pdf_obj>();
    (*new_node).value = ::core::ptr::null_mut::<pdf_obj>();
    (*new_node).next = ::core::ptr::null_mut::<pdf_dict>();
    (*data).next = new_node as *mut pdf_dict;
    (*data).key = key as *mut pdf_obj;
    (*data).value = value as *mut pdf_obj;
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_merge_dict")]
pub unsafe extern "C-unwind" fn pdf_merge_dict(mut dict1: *mut pdf_obj, mut dict2: *mut pdf_obj) {
    let mut data: *mut pdf_dict = ::core::ptr::null_mut::<pdf_dict>();
    if dict1.is_null() || (*dict1).type_0 != 6 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !dict1.is_null() {
                (*dict1).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            6 as ::core::ffi::c_int,
            1616 as ::core::ffi::c_int,
        );
    }
    if dict2.is_null() || (*dict2).type_0 != 6 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !dict2.is_null() {
                (*dict2).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            6 as ::core::ffi::c_int,
            1617 as ::core::ffi::c_int,
        );
    }
    data = (*dict2).data as *mut pdf_dict;
    while !(*data).key.is_null() {
        pdf_add_dict(
            dict1,
            pdf_link_obj((*data).key as *mut pdf_obj),
            pdf_link_obj((*data).value as *mut pdf_obj),
        );
        data = (*data).next as *mut pdf_dict;
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_foreach_dict")]
pub unsafe extern "C-unwind" fn pdf_foreach_dict(
    mut dict: *mut pdf_obj,
    mut proc_0: Option<
        unsafe extern "C-unwind" fn(
            *mut pdf_obj,
            *mut pdf_obj,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut pdata: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut data: *mut pdf_dict = ::core::ptr::null_mut::<pdf_dict>();
    if proc_0.is_none() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"proc\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if dict.is_null() || (*dict).type_0 != 6 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !dict.is_null() {
                (*dict).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            6 as ::core::ffi::c_int,
            1635 as ::core::ffi::c_int,
        );
    }
    data = (*dict).data as *mut pdf_dict;
    while error == 0 && !(*data).key.is_null() {
        error = proc_0.expect("non-null function pointer")(
            (*data).key as *mut pdf_obj,
            (*data).value as *mut pdf_obj,
            pdata,
        );
        data = (*data).next as *mut pdf_dict;
    }
    return error;
}
#[unsafe(export_name = "__pdf_impl_pdf_lookup_dict")]
pub unsafe extern "C-unwind" fn pdf_lookup_dict(
    mut dict: *mut pdf_obj,
    mut name: *const ::core::ffi::c_char,
) -> *mut pdf_obj {
    let mut data: *mut pdf_dict = ::core::ptr::null_mut::<pdf_dict>();
    if name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"name\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if dict.is_null() || (*dict).type_0 != 6 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !dict.is_null() {
                (*dict).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            6 as ::core::ffi::c_int,
            1655 as ::core::ffi::c_int,
        );
    }
    data = (*dict).data as *mut pdf_dict;
    while !(*data).key.is_null() {
        if streq_ptr(name, pdf_name_value((*data).key as *mut pdf_obj)) {
            return (*data).value as *mut pdf_obj;
        }
        data = (*data).next as *mut pdf_dict;
    }
    return ::core::ptr::null_mut::<pdf_obj>();
}
#[unsafe(export_name = "__pdf_impl_pdf_dict_keys")]
pub unsafe extern "C-unwind" fn pdf_dict_keys(mut dict: *mut pdf_obj) -> *mut pdf_obj {
    let mut keys: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut data: *mut pdf_dict = ::core::ptr::null_mut::<pdf_dict>();
    if dict.is_null() || (*dict).type_0 != 6 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !dict.is_null() {
                (*dict).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            6 as ::core::ffi::c_int,
            1675 as ::core::ffi::c_int,
        );
    }
    keys = pdf_new_array();
    data = (*dict).data as *mut pdf_dict;
    while !data.is_null() && !(*data).key.is_null() {
        pdf_add_array(
            keys,
            pdf_new_name(pdf_name_value((*data).key as *mut pdf_obj)),
        );
        data = (*data).next as *mut pdf_dict;
    }
    return keys;
}
#[unsafe(export_name = "__pdf_impl_pdf_remove_dict")]
pub unsafe extern "C-unwind" fn pdf_remove_dict(
    mut dict: *mut pdf_obj,
    mut name: *const ::core::ffi::c_char,
) {
    let mut data: *mut pdf_dict = ::core::ptr::null_mut::<pdf_dict>();
    let mut data_p: *mut *mut pdf_dict = ::core::ptr::null_mut::<*mut pdf_dict>();
    if dict.is_null() || (*dict).type_0 != 6 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !dict.is_null() {
                (*dict).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            6 as ::core::ffi::c_int,
            1694 as ::core::ffi::c_int,
        );
    }
    data = (*dict).data as *mut pdf_dict;
    data_p = &raw mut (*dict).data as *mut ::core::ffi::c_void as *mut *mut pdf_dict;
    while !(*data).key.is_null() {
        if !(*data).key.is_null()
            && !name.is_null()
            && streq_ptr((*((*(*data).key).data as *mut pdf_name)).name, name) as ::core::ffi::c_int
                != 0
        {
            pdf_release_obj((*data).key as *mut pdf_obj);
            pdf_release_obj((*data).value as *mut pdf_obj);
            *data_p = (*data).next as *mut pdf_dict;
            free(data as *mut ::core::ffi::c_void);
            break;
        } else {
            data_p = &raw mut (*data).next as *mut *mut pdf_dict;
            data = (*data).next as *mut pdf_dict;
        }
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_new_stream")]
pub unsafe extern "C-unwind" fn pdf_new_stream(mut flags: ::core::ffi::c_int) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut data: *mut pdf_stream = ::core::ptr::null_mut::<pdf_stream>();
    result = pdf_new_obj(PDF_STREAM);
    data = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_stream>() as usize) as uint32_t)
        as *mut pdf_stream;
    (*data).dict = pdf_new_dict() as *mut pdf_obj;
    (*data)._flags = flags as int32_t;
    (*data).stream = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    (*data).stream_length = 0 as size_t;
    (*data).max_length = 0 as size_t;
    (*data).objstm_data = ::core::ptr::null_mut::<::core::ffi::c_int>();
    (*data).decodeparms.predictor = 2 as ::core::ffi::c_int;
    (*data).decodeparms.columns = 0 as ::core::ffi::c_int as int32_t;
    (*data).decodeparms.bits_per_component = 0 as ::core::ffi::c_int;
    (*data).decodeparms.colors = 0 as ::core::ffi::c_int;
    (*result).data = data as *mut ::core::ffi::c_void;
    (*result).flags = ((*result).flags as ::core::ffi::c_int | OBJ_NO_OBJSTM) as int32_t;
    return result;
}
#[unsafe(export_name = "__pdf_impl_pdf_stream_set_predictor")]
pub unsafe extern "C-unwind" fn pdf_stream_set_predictor(
    mut stream: *mut pdf_obj,
    mut predictor: ::core::ffi::c_int,
    mut columns: int32_t,
    mut bpc: ::core::ffi::c_int,
    mut colors: ::core::ffi::c_int,
) {
    let mut data: *mut pdf_stream = ::core::ptr::null_mut::<pdf_stream>();
    if pdf_obj_typeof(stream) != PDF_STREAM {
        return;
    } else if columns < 0 as int32_t
        || bpc < 0 as ::core::ffi::c_int
        || colors < 0 as ::core::ffi::c_int
    {
        return;
    }
    data = (*stream).data as *mut pdf_stream;
    (*data).decodeparms.predictor = predictor;
    (*data).decodeparms.columns = columns;
    (*data).decodeparms.bits_per_component = bpc;
    (*data).decodeparms.colors = colors;
    (*data)._flags = ((*data)._flags as ::core::ffi::c_int | STREAM_USE_PREDICTOR) as int32_t;
}
#[unsafe(export_name = "__pdf_impl_tt_pdf_filter_png_rows")]
pub unsafe extern "C-unwind" fn tt_pdf_filter_png_rows(
    mut raster: *const ::core::ffi::c_uchar,
    mut dst: *mut ::core::ffi::c_uchar,
    mut rowbytes: int32_t,
    mut bytes_per_pixel: ::core::ffi::c_int,
    mut first_row: int32_t,
    mut last_row: int32_t,
) {
    let mut i: int32_t = 0;
    let mut j: int32_t = 0;
    j = first_row;
    while j < last_row {
        let mut type_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut pp: *mut ::core::ffi::c_uchar = dst.offset(
            ((j - first_row) as size_t).wrapping_mul((rowbytes as size_t).wrapping_add(1 as size_t))
                as isize,
        );
        let mut p: *const ::core::ffi::c_uchar =
            raster.offset((j as size_t).wrapping_mul(rowbytes as size_t) as isize);
        let mut sum: [uint32_t; 5] = [
            0 as ::core::ffi::c_int as uint32_t,
            0 as ::core::ffi::c_int as uint32_t,
            0 as ::core::ffi::c_int as uint32_t,
            0 as ::core::ffi::c_int as uint32_t,
            0 as ::core::ffi::c_int as uint32_t,
        ];
        i = 0 as ::core::ffi::c_int as int32_t;
        while i < rowbytes {
            let mut left: ::core::ffi::c_int = if i - bytes_per_pixel as int32_t >= 0 as int32_t {
                *p.offset((i - bytes_per_pixel as int32_t) as isize) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            let mut up: ::core::ffi::c_int = if j > 0 as int32_t {
                *p.offset(i as isize).offset(-(rowbytes as isize)) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            let mut uplft: ::core::ffi::c_int = if j > 0 as int32_t {
                if i - bytes_per_pixel as int32_t >= 0 as int32_t {
                    *p.offset(i as isize)
                        .offset(-(rowbytes as isize))
                        .offset(-(bytes_per_pixel as isize))
                        as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }
            } else {
                0 as ::core::ffi::c_int
            };
            sum[0 as ::core::ffi::c_int as usize] = sum[0 as ::core::ffi::c_int as usize]
                .wrapping_add(*p.offset(i as isize) as uint32_t);
            sum[1 as ::core::ffi::c_int as usize] = sum[1 as ::core::ffi::c_int as usize]
                .wrapping_add(abs(*p.offset(i as isize) as ::core::ffi::c_int - left) as uint32_t);
            sum[2 as ::core::ffi::c_int as usize] = sum[2 as ::core::ffi::c_int as usize]
                .wrapping_add(abs(*p.offset(i as isize) as ::core::ffi::c_int - up) as uint32_t);
            let mut tmp: ::core::ffi::c_int =
                floor(((up + left) / 2 as ::core::ffi::c_int) as ::core::ffi::c_double)
                    as ::core::ffi::c_int;
            sum[3 as ::core::ffi::c_int as usize] = sum[3 as ::core::ffi::c_int as usize]
                .wrapping_add(abs(*p.offset(i as isize) as ::core::ffi::c_int - tmp) as uint32_t);
            let mut q: ::core::ffi::c_int = left + up - uplft;
            let mut qa: ::core::ffi::c_int = abs(q - left);
            let mut qb: ::core::ffi::c_int = abs(q - up);
            let mut qc: ::core::ffi::c_int = abs(q - uplft);
            if qa <= qb && qa <= qc {
                sum[4 as ::core::ffi::c_int as usize] = sum[4 as ::core::ffi::c_int as usize]
                    .wrapping_add(
                        abs(*p.offset(i as isize) as ::core::ffi::c_int - left) as uint32_t
                    );
            } else if qb <= qc {
                sum[4 as ::core::ffi::c_int as usize] =
                    sum[4 as ::core::ffi::c_int as usize]
                        .wrapping_add(
                            abs(*p.offset(i as isize) as ::core::ffi::c_int - up) as uint32_t
                        );
            } else {
                sum[4 as ::core::ffi::c_int as usize] = sum[4 as ::core::ffi::c_int as usize]
                    .wrapping_add(
                        abs(*p.offset(i as isize) as ::core::ffi::c_int - uplft) as uint32_t
                    );
            }
            i += 1;
        }
        let mut min: ::core::ffi::c_int =
            sum[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        let mut min_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int as int32_t;
        while i < 5 as int32_t {
            if sum[i as usize] < min as uint32_t {
                min = sum[i as usize] as ::core::ffi::c_int;
                min_idx = i as ::core::ffi::c_int;
            }
            i += 1;
        }
        type_0 = min_idx;
        *pp.offset(0 as ::core::ffi::c_int as isize) = type_0 as ::core::ffi::c_uchar;
        match type_0 {
            0 => {
                memcpy(
                    pp.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    rowbytes as size_t,
                );
            }
            1 => {
                i = 0 as ::core::ffi::c_int as int32_t;
                while i < rowbytes {
                    let mut left_0: ::core::ffi::c_int = if i - bytes_per_pixel as int32_t
                        >= 0 as int32_t
                    {
                        *p.offset((i - bytes_per_pixel as int32_t) as isize) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    *pp.offset((i + 1 as int32_t) as isize) =
                        (*p.offset(i as isize) as ::core::ffi::c_int - left_0)
                            as ::core::ffi::c_uchar;
                    i += 1;
                }
            }
            2 => {
                i = 0 as ::core::ffi::c_int as int32_t;
                while i < rowbytes {
                    let mut up_0: ::core::ffi::c_int = if j > 0 as int32_t {
                        *p.offset(i as isize).offset(-(rowbytes as isize)) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    *pp.offset((i + 1 as int32_t) as isize) =
                        (*p.offset(i as isize) as ::core::ffi::c_int - up_0)
                            as ::core::ffi::c_uchar;
                    i += 1;
                }
            }
            3 => {
                i = 0 as ::core::ffi::c_int as int32_t;
                while i < rowbytes {
                    let mut up_1: ::core::ffi::c_int = if j > 0 as int32_t {
                        *p.offset(i as isize).offset(-(rowbytes as isize)) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    let mut left_1: ::core::ffi::c_int = if i - bytes_per_pixel as int32_t
                        >= 0 as int32_t
                    {
                        *p.offset((i - bytes_per_pixel as int32_t) as isize) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    let mut tmp_0: ::core::ffi::c_int =
                        floor(((up_1 + left_1) / 2 as ::core::ffi::c_int) as ::core::ffi::c_double)
                            as ::core::ffi::c_int;
                    *pp.offset((i + 1 as int32_t) as isize) =
                        (*p.offset(i as isize) as ::core::ffi::c_int - tmp_0)
                            as ::core::ffi::c_uchar;
                    i += 1;
                }
            }
            4 => {
                i = 0 as ::core::ffi::c_int as int32_t;
                while i < rowbytes {
                    let mut up_2: ::core::ffi::c_int = if j > 0 as int32_t {
                        *p.offset(i as isize).offset(-(rowbytes as isize)) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    let mut left_2: ::core::ffi::c_int = if i - bytes_per_pixel as int32_t
                        >= 0 as int32_t
                    {
                        *p.offset((i - bytes_per_pixel as int32_t) as isize) as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    let mut uplft_0: ::core::ffi::c_int = if j > 0 as int32_t {
                        if i - bytes_per_pixel as int32_t >= 0 as int32_t {
                            *p.offset(i as isize)
                                .offset(-(rowbytes as isize))
                                .offset(-(bytes_per_pixel as isize))
                                as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    let mut q_0: ::core::ffi::c_int = left_2 + up_2 - uplft_0;
                    let mut qa_0: ::core::ffi::c_int = abs(q_0 - left_2);
                    let mut qb_0: ::core::ffi::c_int = abs(q_0 - up_2);
                    let mut qc_0: ::core::ffi::c_int = abs(q_0 - uplft_0);
                    if qa_0 <= qb_0 && qa_0 <= qc_0 {
                        *pp.offset((i + 1 as int32_t) as isize) =
                            (*p.offset(i as isize) as ::core::ffi::c_int - left_2)
                                as ::core::ffi::c_uchar;
                    } else if qb_0 <= qc_0 {
                        *pp.offset((i + 1 as int32_t) as isize) =
                            (*p.offset(i as isize) as ::core::ffi::c_int - up_2)
                                as ::core::ffi::c_uchar;
                    } else {
                        *pp.offset((i + 1 as int32_t) as isize) =
                            (*p.offset(i as isize) as ::core::ffi::c_int - uplft_0)
                                as ::core::ffi::c_uchar;
                    }
                    i += 1;
                }
            }
            _ => {}
        }
        j += 1;
    }
}
unsafe extern "C-unwind" fn filter_PNG15_apply_filter(
    mut raster: *mut ::core::ffi::c_uchar,
    mut columns: int32_t,
    mut rows: int32_t,
    mut bpc: int8_t,
    mut colors: int8_t,
    mut length: *mut int32_t,
) -> *mut ::core::ffi::c_uchar {
    let mut bytes_per_pixel: ::core::ffi::c_int =
        (colors as ::core::ffi::c_int * bpc as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int;
    let mut rowbytes: int32_t = columns * bytes_per_pixel as int32_t;
    let mut dst: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !(!raster.is_null() && !length.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"raster && length\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    dst = new((((rowbytes + 1 as int32_t) * rows) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    *length = (rowbytes + 1 as int32_t) * rows;
    tt_pdf_filter_png_parallel(raster, dst, rowbytes, rows, bytes_per_pixel);
    return dst;
}
unsafe extern "C-unwind" fn filter_TIFF2_apply_filter(
    mut raster: *mut ::core::ffi::c_uchar,
    mut columns: int32_t,
    mut rows: int32_t,
    mut bpc: int8_t,
    mut colors: int8_t,
    mut length: *mut int32_t,
) -> *mut ::core::ffi::c_uchar {
    let mut dst: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut rowbytes: int32_t =
        (bpc as int32_t * colors as int32_t * columns + 7 as int32_t) / 8 as int32_t;
    if !(!raster.is_null() && !length.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"raster && length\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    dst = new(((rowbytes * rows) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    *length = rowbytes * rows;
    tt_pdf_filter_tiff_parallel(
        raster,
        dst,
        columns,
        rows,
        bpc as ::core::ffi::c_int,
        colors as ::core::ffi::c_int,
    );
    return dst;
}
unsafe extern "C-unwind" fn filter_create_predictor_dict(
    mut predictor: ::core::ffi::c_int,
    mut columns: int32_t,
    mut bpc: ::core::ffi::c_int,
    mut colors: ::core::ffi::c_int,
) -> *mut pdf_obj {
    let mut parms: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    parms = pdf_new_dict();
    pdf_add_dict(
        parms,
        pdf_new_name(b"BitsPerComponent\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(bpc as ::core::ffi::c_double),
    );
    pdf_add_dict(
        parms,
        pdf_new_name(b"Colors\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(colors as ::core::ffi::c_double),
    );
    pdf_add_dict(
        parms,
        pdf_new_name(b"Columns\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(columns as ::core::ffi::c_double),
    );
    pdf_add_dict(
        parms,
        pdf_new_name(b"Predictor\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(predictor as ::core::ffi::c_double),
    );
    return parms;
}
unsafe extern "C-unwind" fn write_stream(mut p: *mut pdf_out, mut stream: *mut pdf_stream) {
    let mut filtered: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut filtered_length: size_t = 0;
    let mut buffer_length: size_t = 0;
    let mut buffer_length64: uint64_t = 0;
    let mut buffer: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    filtered = new(((*stream).stream_length as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    memcpy(
        filtered as *mut ::core::ffi::c_void,
        (*stream).stream as *const ::core::ffi::c_void,
        (*stream).stream_length,
    );
    filtered_length = (*stream).stream_length;
    let mut type_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    type_0 = pdf_lookup_dict(
        (*stream).dict as *mut pdf_obj,
        b"Type\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !type_0.is_null()
        && streq_ptr(
            b"Metadata\0" as *const u8 as *const ::core::ffi::c_char,
            pdf_name_value(type_0),
        ) as ::core::ffi::c_int
            != 0
    {
        (*stream)._flags = ((*stream)._flags as ::core::ffi::c_int & !STREAM_COMPRESS) as int32_t;
    }
    if (*stream).stream_length > 0 as size_t
        && (*stream)._flags & STREAM_COMPRESS as int32_t != 0
        && (*p).options.compression.level > 0 as ::core::ffi::c_int
    {
        let mut filters: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        if (*p).options.compression.use_predictor != 0
            && (*stream)._flags & STREAM_USE_PREDICTOR as int32_t != 0
            && pdf_lookup_dict(
                (*stream).dict as *mut pdf_obj,
                b"DecodeParms\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            let mut bits_per_pixel: ::core::ffi::c_int =
                (*stream).decodeparms.colors * (*stream).decodeparms.bits_per_component;
            let mut len: int32_t = ((*stream).decodeparms.columns * bits_per_pixel as int32_t
                + 7 as int32_t)
                / 8 as int32_t;
            let mut rows: int32_t = (*stream).stream_length.wrapping_div(len as size_t) as int32_t;
            let mut filtered2: *mut ::core::ffi::c_uchar =
                ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            let mut length2: int32_t = (*stream).stream_length as int32_t;
            let mut parms: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            parms = filter_create_predictor_dict(
                (*stream).decodeparms.predictor,
                (*stream).decodeparms.columns,
                (*stream).decodeparms.bits_per_component,
                (*stream).decodeparms.colors,
            );
            match (*stream).decodeparms.predictor {
                2 => {
                    filtered2 = filter_TIFF2_apply_filter(
                        filtered,
                        (*stream).decodeparms.columns,
                        rows,
                        (*stream).decodeparms.bits_per_component as int8_t,
                        (*stream).decodeparms.colors as int8_t,
                        &raw mut length2,
                    );
                }
                15 => {
                    filtered2 = filter_PNG15_apply_filter(
                        filtered,
                        (*stream).decodeparms.columns,
                        rows,
                        (*stream).decodeparms.bits_per_component as int8_t,
                        (*stream).decodeparms.colors as int8_t,
                        &raw mut length2,
                    );
                }
                _ => {
                    pdf_warning!(
                        b"Unknown/unsupported Predictor function %d.\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*stream).decodeparms.predictor,
                    );
                }
            }
            if !parms.is_null() && !filtered2.is_null() {
                free(filtered as *mut ::core::ffi::c_void);
                filtered = filtered2;
                filtered_length = length2 as size_t;
                pdf_add_dict(
                    (*stream).dict as *mut pdf_obj,
                    pdf_new_name(b"DecodeParms\0" as *const u8 as *const ::core::ffi::c_char),
                    parms,
                );
            }
        }
        filters = pdf_lookup_dict(
            (*stream).dict as *mut pdf_obj,
            b"Filter\0" as *const u8 as *const ::core::ffi::c_char,
        );
        buffer_length = filtered_length
            .wrapping_add(filtered_length.wrapping_div(1000 as size_t))
            .wrapping_add(14 as size_t);
        buffer = new((buffer_length as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        let mut filter_name: *mut pdf_obj =
            pdf_new_name(b"FlateDecode\0" as *const u8 as *const ::core::ffi::c_char);
        if !filters.is_null() {
            pdf_unshift_array(filters, filter_name);
        } else {
            pdf_add_dict(
                (*stream).dict as *mut pdf_obj,
                pdf_new_name(b"Filter\0" as *const u8 as *const ::core::ffi::c_char),
                filter_name,
            );
        }
        buffer_length64 = buffer_length as uint64_t;
        if (tectonic_flate_compress(
            buffer as *mut uint8_t,
            &raw mut buffer_length64,
            filtered,
            filtered_length as uint64_t,
            (*p).options.compression.level as uint32_t,
        ) as ::core::ffi::c_int)
            < 0 as ::core::ffi::c_int
        {
            pdf_abort!(b"Zlib error\0" as *const u8 as *const ::core::ffi::c_char);
        }
        buffer_length = buffer_length64 as size_t;
        free(filtered as *mut ::core::ffi::c_void);
        (*p).output.compression_saved = (*p).output.compression_saved.wrapping_add(
            filtered_length.wrapping_sub(buffer_length).wrapping_sub(
                (if !filters.is_null() {
                    strlen(b"/FlateDecode \0" as *const u8 as *const ::core::ffi::c_char)
                } else {
                    strlen(b"/Filter/FlateDecode\n\0" as *const u8 as *const ::core::ffi::c_char)
                }),
            ),
        );
        filtered = buffer;
        filtered_length = buffer_length;
    }
    if (*p).state.enc_mode != 0 {
        let mut cipher: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut cipher_len: size_t = 0 as size_t;
        pdf_encrypt_data(
            (*p).sec_data,
            filtered,
            filtered_length,
            &raw mut cipher,
            &raw mut cipher_len,
        );
        free(filtered as *mut ::core::ffi::c_void);
        filtered = cipher;
        filtered_length = cipher_len;
    }
    pdf_add_dict(
        (*stream).dict as *mut pdf_obj,
        pdf_new_name(b"Length\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(filtered_length as ::core::ffi::c_double),
    );
    pdf_write_obj(p, (*stream).dict as *mut pdf_obj);
    pdf_out_str(
        p,
        b"\nstream\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        8 as size_t,
    );
    if filtered_length > 0 as size_t {
        pdf_out_str(p, filtered as *const ::core::ffi::c_void, filtered_length);
    }
    free(filtered as *mut ::core::ffi::c_void);
    pdf_out_str(
        p,
        b"\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        1 as size_t,
    );
    pdf_out_str(
        p,
        b"endstream\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        9 as size_t,
    );
}
unsafe extern "C-unwind" fn release_stream(mut stream: *mut pdf_stream) {
    pdf_release_obj((*stream).dict as *mut pdf_obj);
    (*stream).dict = ::core::ptr::null_mut::<pdf_obj>();
    (*stream).stream =
        mfree((*stream).stream as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_uchar;
    (*stream).objstm_data =
        mfree((*stream).objstm_data as *mut ::core::ffi::c_void) as *mut ::core::ffi::c_int;
    free(stream as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_pdf_stream_dict")]
pub unsafe extern "C-unwind" fn pdf_stream_dict(mut stream: *mut pdf_obj) -> *mut pdf_obj {
    let mut data: *mut pdf_stream = ::core::ptr::null_mut::<pdf_stream>();
    if stream.is_null() || (*stream).type_0 != 7 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !stream.is_null() {
                (*stream).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            7 as ::core::ffi::c_int,
            2099 as ::core::ffi::c_int,
        );
    }
    data = (*stream).data as *mut pdf_stream;
    return (*data).dict as *mut pdf_obj;
}
#[unsafe(export_name = "__pdf_impl_pdf_stream_dataptr")]
pub unsafe extern "C-unwind" fn pdf_stream_dataptr(
    mut stream: *mut pdf_obj,
) -> *const ::core::ffi::c_void {
    let mut data: *mut pdf_stream = ::core::ptr::null_mut::<pdf_stream>();
    if stream.is_null() || (*stream).type_0 != 7 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !stream.is_null() {
                (*stream).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            7 as ::core::ffi::c_int,
            2111 as ::core::ffi::c_int,
        );
    }
    data = (*stream).data as *mut pdf_stream;
    return (*data).stream as *const ::core::ffi::c_void;
}
#[unsafe(export_name = "__pdf_impl_pdf_stream_length")]
pub unsafe extern "C-unwind" fn pdf_stream_length(mut stream: *mut pdf_obj) -> ::core::ffi::c_int {
    let mut data: *mut pdf_stream = ::core::ptr::null_mut::<pdf_stream>();
    if stream.is_null() || (*stream).type_0 != 7 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !stream.is_null() {
                (*stream).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            7 as ::core::ffi::c_int,
            2123 as ::core::ffi::c_int,
        );
    }
    data = (*stream).data as *mut pdf_stream;
    return (*data).stream_length as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_objstm_data(
    mut objstm: *mut pdf_obj,
    mut data: *mut ::core::ffi::c_int,
) {
    if objstm.is_null() || (*objstm).type_0 != 7 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !objstm.is_null() {
                (*objstm).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            7 as ::core::ffi::c_int,
            2132 as ::core::ffi::c_int,
        );
    }
    let ref mut fresh18 = (*((*objstm).data as *mut pdf_stream)).objstm_data;
    *fresh18 = data;
}
unsafe extern "C-unwind" fn get_objstm_data(mut objstm: *mut pdf_obj) -> *mut ::core::ffi::c_int {
    if objstm.is_null() || (*objstm).type_0 != 7 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !objstm.is_null() {
                (*objstm).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            7 as ::core::ffi::c_int,
            2139 as ::core::ffi::c_int,
        );
    }
    return (*((*objstm).data as *mut pdf_stream)).objstm_data;
}
#[unsafe(export_name = "__pdf_impl_pdf_add_stream")]
pub unsafe extern "C-unwind" fn pdf_add_stream(
    mut stream: *mut pdf_obj,
    mut stream_data: *const ::core::ffi::c_void,
    mut length: ::core::ffi::c_int,
) {
    let mut data: *mut pdf_stream = ::core::ptr::null_mut::<pdf_stream>();
    if stream.is_null() || (*stream).type_0 != 7 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !stream.is_null() {
                (*stream).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            7 as ::core::ffi::c_int,
            2149 as ::core::ffi::c_int,
        );
    }
    if length < 1 as ::core::ffi::c_int {
        return;
    }
    data = (*stream).data as *mut pdf_stream;
    if (*data).stream_length.wrapping_add(length as size_t) > (*data).max_length {
        (*data).max_length = (*data).max_length.wrapping_add(
            (length as ::core::ffi::c_uint).wrapping_add(STREAM_ALLOC_SIZE) as size_t,
        );
        (*data).stream = renew(
            (*data).stream as *mut ::core::ffi::c_void,
            ((*data).max_length as uint32_t as usize)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_uchar;
    }
    memcpy(
        (*data).stream.offset((*data).stream_length as isize) as *mut ::core::ffi::c_void,
        stream_data,
        length as size_t,
    );
    (*data).stream_length = (*data).stream_length.wrapping_add(length as size_t);
}
unsafe extern "C-unwind" fn filter_get_DecodeParms_FlateDecode(
    mut parms: *mut decode_parms,
    mut dict: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if !(!dict.is_null() && !parms.is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"dict && parms\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(!dict.is_null() && pdf_obj_typeof(dict) == 6 as ::core::ffi::c_int) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"PDF_OBJ_DICTTYPE(dict)\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    (*parms).predictor = 1 as ::core::ffi::c_int;
    (*parms).colors = 1 as ::core::ffi::c_int;
    (*parms).bits_per_component = 8 as ::core::ffi::c_int;
    (*parms).columns = 1 as ::core::ffi::c_int as int32_t;
    tmp = pdf_deref_obj(pdf_lookup_dict(
        dict,
        b"Predictor\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    if !tmp.is_null() {
        (*parms).predictor = pdf_number_value(tmp) as ::core::ffi::c_int;
        pdf_release_obj(tmp);
    }
    tmp = pdf_deref_obj(pdf_lookup_dict(
        dict,
        b"Colors\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    if !tmp.is_null() {
        (*parms).colors = pdf_number_value(tmp) as ::core::ffi::c_int;
        pdf_release_obj(tmp);
    }
    tmp = pdf_deref_obj(pdf_lookup_dict(
        dict,
        b"BitsPerComponent\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    if !tmp.is_null() {
        (*parms).bits_per_component = pdf_number_value(tmp) as ::core::ffi::c_int;
        pdf_release_obj(tmp);
    }
    tmp = pdf_deref_obj(pdf_lookup_dict(
        dict,
        b"Columns\0" as *const u8 as *const ::core::ffi::c_char,
    ));
    if !tmp.is_null() {
        (*parms).columns = pdf_number_value(tmp) as int32_t;
        pdf_release_obj(tmp);
    }
    if (*parms).bits_per_component != 1 as ::core::ffi::c_int
        && (*parms).bits_per_component != 2 as ::core::ffi::c_int
        && (*parms).bits_per_component != 4 as ::core::ffi::c_int
        && (*parms).bits_per_component != 8 as ::core::ffi::c_int
        && (*parms).bits_per_component != 16 as ::core::ffi::c_int
    {
        pdf_warning!(
            b"Invalid BPC value in DecodeParms: %d\0" as *const u8 as *const ::core::ffi::c_char,
            (*parms).bits_per_component,
        );
        return -(1 as ::core::ffi::c_int);
    } else if (*parms).predictor <= 0 as ::core::ffi::c_int
        || (*parms).colors <= 0 as ::core::ffi::c_int
        || (*parms).columns <= 0 as int32_t
    {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn filter_row_TIFF2(
    mut dst: *mut ::core::ffi::c_uchar,
    mut src: *const ::core::ffi::c_uchar,
    mut parms: *mut decode_parms,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_uchar = src;
    let mut col: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut mask: ::core::ffi::c_int =
        ((1 as ::core::ffi::c_int) << (*parms).bits_per_component) - 1 as ::core::ffi::c_int;
    let mut inbuf: ::core::ffi::c_int = 0;
    let mut outbuf: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ci: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut inbits: ::core::ffi::c_int = 0;
    let mut outbits: ::core::ffi::c_int = 0;
    col = new(((*parms).colors as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    memset(
        col as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (*parms).colors as size_t,
    );
    outbuf = 0 as ::core::ffi::c_int;
    inbuf = outbuf;
    outbits = 0 as ::core::ffi::c_int;
    inbits = outbits;
    k = 0 as ::core::ffi::c_int;
    j = k;
    i = 0 as ::core::ffi::c_int;
    while (i as int32_t) < (*parms).columns {
        ci = 0 as ::core::ffi::c_int;
        while ci < (*parms).colors {
            if inbits < (*parms).bits_per_component {
                let fresh23 = j;
                j = j + 1;
                inbuf = inbuf << 8 as ::core::ffi::c_int
                    | *p.offset(fresh23 as isize) as ::core::ffi::c_int;
                inbits += 8 as ::core::ffi::c_int;
            }
            *col.offset(ci as isize) = (*col.offset(ci as isize) as ::core::ffi::c_int
                + (inbuf >> inbits - (*parms).bits_per_component)
                & mask) as ::core::ffi::c_uchar;
            inbits -= (*parms).bits_per_component;
            outbuf = outbuf << (*parms).bits_per_component
                | *col.offset(ci as isize) as ::core::ffi::c_int;
            outbits += (*parms).bits_per_component;
            if outbits >= 8 as ::core::ffi::c_int {
                let fresh24 = k;
                k = k + 1;
                *dst.offset(fresh24 as isize) =
                    (outbuf >> outbits - 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                outbits -= 8 as ::core::ffi::c_int;
            }
            ci += 1;
        }
        i += 1;
    }
    if outbits > 0 as ::core::ffi::c_int {
        *dst.offset(k as isize) =
            (outbuf << 8 as ::core::ffi::c_int - outbits) as ::core::ffi::c_uchar;
    }
    free(col as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn filter_stream_decode_Predictor(
    mut src: *const ::core::ffi::c_void,
    mut srclen: size_t,
    mut parms: *mut decode_parms,
) -> *mut pdf_obj {
    let mut dst: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut p: *const ::core::ffi::c_uchar = src as *const ::core::ffi::c_uchar;
    let mut endptr: *const ::core::ffi::c_uchar = p.offset(srclen as isize);
    let mut prev: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut buf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut bits_per_pixel: ::core::ffi::c_int = (*parms).colors * (*parms).bits_per_component;
    let mut bytes_per_pixel: ::core::ffi::c_int =
        (bits_per_pixel + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int;
    let mut length: ::core::ffi::c_int = ((*parms).columns as ::core::ffi::c_int * bits_per_pixel
        + 7 as ::core::ffi::c_int)
        / 8 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    dst = pdf_new_stream(0 as ::core::ffi::c_int);
    prev = new((length as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    buf = new((length as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    memset(
        prev as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        length as size_t,
    );
    match (*parms).predictor {
        1 => {
            pdf_add_stream(dst, src, srclen as ::core::ffi::c_int);
        }
        2 => {
            if (*parms).bits_per_component == 8 as ::core::ffi::c_int {
                while p.offset(length as isize) < endptr {
                    i = 0 as ::core::ffi::c_int;
                    while i < length {
                        let mut pv: ::core::ffi::c_int =
                            if i - bytes_per_pixel >= 0 as ::core::ffi::c_int {
                                *buf.offset((i - bytes_per_pixel) as isize) as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            };
                        *buf.offset(i as isize) = (*p.offset(i as isize) as ::core::ffi::c_int + pv
                            & 0xff as ::core::ffi::c_int)
                            as ::core::ffi::c_uchar;
                        i += 1;
                    }
                    pdf_add_stream(dst, buf as *const ::core::ffi::c_void, length);
                    p = p.offset(length as isize);
                }
            } else if (*parms).bits_per_component == 16 as ::core::ffi::c_int {
                while p.offset(length as isize) < endptr {
                    i = 0 as ::core::ffi::c_int;
                    while i < length {
                        let mut b: ::core::ffi::c_int = i - bytes_per_pixel;
                        let mut hi: ::core::ffi::c_char = (if b >= 0 as ::core::ffi::c_int {
                            *buf.offset(b as isize) as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                            as ::core::ffi::c_char;
                        let mut lo: ::core::ffi::c_char = (if b >= 0 as ::core::ffi::c_int {
                            *buf.offset((b + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                            as ::core::ffi::c_char;
                        let mut pv_0: ::core::ffi::c_int = (hi as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | lo as ::core::ffi::c_int;
                        let mut cv: ::core::ffi::c_int = (*p.offset(i as isize)
                            as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *p.offset((i + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int;
                        let mut c: ::core::ffi::c_int = pv_0 + cv;
                        *buf.offset(i as isize) =
                            (c >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        *buf.offset((i + 1 as ::core::ffi::c_int) as isize) =
                            (c & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                        i += 2 as ::core::ffi::c_int;
                    }
                    pdf_add_stream(dst, buf as *const ::core::ffi::c_void, length);
                    p = p.offset(length as isize);
                }
            } else {
                while error == 0 && p.offset(length as isize) < endptr {
                    error = filter_row_TIFF2(buf, p, parms);
                    if error == 0 {
                        pdf_add_stream(dst, buf as *const ::core::ffi::c_void, length);
                        p = p.offset(length as isize);
                    }
                }
            }
        }
        10 | 11 | 12 | 13 | 14 | 15 => {
            let mut type_0: ::core::ffi::c_int = (*parms).predictor - 10 as ::core::ffi::c_int;
            while error == 0 && p.offset(length as isize) < endptr {
                if (*parms).predictor == 15 as ::core::ffi::c_int {
                    type_0 = *p as ::core::ffi::c_int;
                } else if *p as ::core::ffi::c_int != type_0 {
                    pdf_warning!(
                        b"Mismatched Predictor type in data stream.\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    error = -(1 as ::core::ffi::c_int);
                }
                p = p.offset(1);
                match type_0 {
                    0 => {
                        memcpy(
                            buf as *mut ::core::ffi::c_void,
                            p as *const ::core::ffi::c_void,
                            length as size_t,
                        );
                    }
                    1 => {
                        i = 0 as ::core::ffi::c_int;
                        while i < length {
                            let mut pv_1: ::core::ffi::c_int = if i - bytes_per_pixel
                                >= 0 as ::core::ffi::c_int
                            {
                                *buf.offset((i - bytes_per_pixel) as isize) as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            *buf.offset(i as isize) = (*p.offset(i as isize) as ::core::ffi::c_int
                                + pv_1
                                & 0xff as ::core::ffi::c_int)
                                as ::core::ffi::c_uchar;
                            i += 1;
                        }
                    }
                    2 => {
                        i = 0 as ::core::ffi::c_int;
                        while i < length {
                            *buf.offset(i as isize) = (*p.offset(i as isize) as ::core::ffi::c_int
                                + *prev.offset(i as isize) as ::core::ffi::c_int
                                & 0xff as ::core::ffi::c_int)
                                as ::core::ffi::c_uchar;
                            i += 1;
                        }
                    }
                    3 => {
                        i = 0 as ::core::ffi::c_int;
                        while i < length {
                            let mut up: ::core::ffi::c_int =
                                *prev.offset(i as isize) as ::core::ffi::c_int;
                            let mut left: ::core::ffi::c_int = if i - bytes_per_pixel
                                >= 0 as ::core::ffi::c_int
                            {
                                *buf.offset((i - bytes_per_pixel) as isize) as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            let mut tmp: ::core::ffi::c_int = floor(
                                ((up + left) / 2 as ::core::ffi::c_int) as ::core::ffi::c_double,
                            )
                                as ::core::ffi::c_int;
                            *buf.offset(i as isize) = (*p.offset(i as isize) as ::core::ffi::c_int
                                + tmp
                                & 0xff as ::core::ffi::c_int)
                                as ::core::ffi::c_uchar;
                            i += 1;
                        }
                    }
                    4 => {
                        i = 0 as ::core::ffi::c_int;
                        while i < length {
                            let mut a: ::core::ffi::c_int = if i - bytes_per_pixel
                                >= 0 as ::core::ffi::c_int
                            {
                                *buf.offset((i - bytes_per_pixel) as isize) as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            let mut b_0: ::core::ffi::c_int =
                                *prev.offset(i as isize) as ::core::ffi::c_int;
                            let mut c_0: ::core::ffi::c_int = if i - bytes_per_pixel
                                >= 0 as ::core::ffi::c_int
                            {
                                *prev.offset((i - bytes_per_pixel) as isize) as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            };
                            let mut q: ::core::ffi::c_int = a + b_0 - c_0;
                            let mut qa: ::core::ffi::c_int = q - a;
                            let mut qb: ::core::ffi::c_int = q - b_0;
                            let mut qc: ::core::ffi::c_int = q - c_0;
                            qa = if qa < 0 as ::core::ffi::c_int {
                                -qa
                            } else {
                                qa
                            };
                            qb = if qb < 0 as ::core::ffi::c_int {
                                -qb
                            } else {
                                qb
                            };
                            qc = if qc < 0 as ::core::ffi::c_int {
                                -qc
                            } else {
                                qc
                            };
                            if qa <= qb && qa <= qc {
                                *buf.offset(i as isize) =
                                    (*p.offset(i as isize) as ::core::ffi::c_int + a
                                        & 0xff as ::core::ffi::c_int)
                                        as ::core::ffi::c_uchar;
                            } else if qb <= qc {
                                *buf.offset(i as isize) =
                                    (*p.offset(i as isize) as ::core::ffi::c_int + b_0
                                        & 0xff as ::core::ffi::c_int)
                                        as ::core::ffi::c_uchar;
                            } else {
                                *buf.offset(i as isize) =
                                    (*p.offset(i as isize) as ::core::ffi::c_int + c_0
                                        & 0xff as ::core::ffi::c_int)
                                        as ::core::ffi::c_uchar;
                            }
                            i += 1;
                        }
                    }
                    _ => {
                        pdf_warning!(
                            b"Unknown PNG predictor type: %d\0" as *const u8
                                as *const ::core::ffi::c_char,
                            type_0,
                        );
                        error = -(1 as ::core::ffi::c_int);
                    }
                }
                if error == 0 {
                    pdf_add_stream(dst, buf as *const ::core::ffi::c_void, length);
                    memcpy(
                        prev as *mut ::core::ffi::c_void,
                        buf as *const ::core::ffi::c_void,
                        length as size_t,
                    );
                    p = p.offset(length as isize);
                }
            }
        }
        _ => {
            pdf_warning!(
                b"Unknown Predictor type value :%d\0" as *const u8 as *const ::core::ffi::c_char,
                (*parms).predictor,
            );
            error = -(1 as ::core::ffi::c_int);
        }
    }
    free(prev as *mut ::core::ffi::c_void);
    free(buf as *mut ::core::ffi::c_void);
    if error != 0 {
        pdf_release_obj(dst);
        dst = ::core::ptr::null_mut::<pdf_obj>();
    }
    return dst;
}
pub const WBUF_SIZE: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn filter_stream_decode_FlateDecode(
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
    mut parms: *mut decode_parms,
) -> *mut pdf_obj {
    let mut dst: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut wbuf: [uint8_t; 4096] = [0; 4096];
    let mut flate_handle: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    flate_handle = tectonic_flate_new_decompressor(data as *const uint8_t, len as uint64_t);
    if flate_handle.is_null() {
        pdf_warning!(
            b"tectonic_flate_new_decompressor() failed\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    tmp = pdf_new_stream(0 as ::core::ffi::c_int);
    loop {
        let mut out_size: uint64_t = WBUF_SIZE as uint64_t;
        if tectonic_flate_decompress_chunk(
            flate_handle,
            &raw mut wbuf as *mut uint8_t,
            &raw mut out_size,
        ) != 0
        {
            pdf_warning!(
                b"tectonic_flate_decompress() failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            tectonic_flate_free_decompressor(flate_handle);
            pdf_release_obj(tmp);
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        if out_size == 0 as uint64_t {
            break;
        }
        pdf_add_stream(
            tmp,
            &raw mut wbuf as *mut uint8_t as *const ::core::ffi::c_void,
            out_size as ::core::ffi::c_int,
        );
    }
    tectonic_flate_free_decompressor(flate_handle);
    if !parms.is_null() {
        dst = filter_stream_decode_Predictor(
            pdf_stream_dataptr(tmp),
            pdf_stream_length(tmp) as size_t,
            parms,
        );
    } else {
        dst = pdf_link_obj(tmp);
    }
    pdf_release_obj(tmp);
    return dst;
}
unsafe extern "C-unwind" fn filter_stream_decode_ASCIIHexDecode(
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) -> *mut pdf_obj {
    let mut dst: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut eod: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0;
    let mut p: *const ::core::ffi::c_char = data as *const ::core::ffi::c_char;
    let mut endptr: *const ::core::ffi::c_char = p.offset(len as isize);
    let mut buf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut ch: ::core::ffi::c_uchar = 0;
    let mut pos: size_t = 0;
    let mut n: size_t = 0;
    buf = new(
        (len.wrapping_add(1 as size_t).wrapping_div(2 as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_uchar;
    skip_white(&raw mut p, endptr);
    ch = 0 as ::core::ffi::c_uchar;
    n = 0 as size_t;
    pos = 0 as size_t;
    eod = 0 as ::core::ffi::c_int;
    error = 0 as ::core::ffi::c_int;
    while p < endptr && error == 0 && eod == 0 {
        let mut c1: ::core::ffi::c_char = 0;
        let mut val: ::core::ffi::c_char = 0;
        c1 = *p.offset(0 as ::core::ffi::c_int as isize);
        if c1 as ::core::ffi::c_int >= 'A' as i32 && c1 as ::core::ffi::c_int <= 'F' as i32 {
            val = (c1 as ::core::ffi::c_int - 'A' as i32 + 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
        } else if c1 as ::core::ffi::c_int >= 'a' as i32 && c1 as ::core::ffi::c_int <= 'f' as i32 {
            val = (c1 as ::core::ffi::c_int - 'a' as i32 + 10 as ::core::ffi::c_int)
                as ::core::ffi::c_char;
        } else if c1 as ::core::ffi::c_int >= '0' as i32 && c1 as ::core::ffi::c_int <= '9' as i32 {
            val = (c1 as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_char;
        } else if c1 as ::core::ffi::c_int == '>' as i32 {
            val = 0 as ::core::ffi::c_char;
            eod = 1 as ::core::ffi::c_int;
            if pos.wrapping_rem(2 as size_t) == 0 as size_t {
                break;
            }
        } else {
            error = -(1 as ::core::ffi::c_int);
            break;
        }
        if pos.wrapping_rem(2 as size_t) != 0 {
            *buf.offset(n as isize) =
                (ch as ::core::ffi::c_int + val as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            n = n.wrapping_add(1);
            ch = 0 as ::core::ffi::c_uchar;
        } else {
            ch = ((val as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        }
        pos = pos.wrapping_add(1);
        p = p.offset(1);
        skip_white(&raw mut p, endptr);
    }
    if error != 0 || eod == 0 {
        pdf_warning!(
            b"Invalid ASCIIHex data seen: %s\0" as *const u8 as *const ::core::ffi::c_char,
            if error != 0 {
                b"Invalid character\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"No EOD marker\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
        dst = ::core::ptr::null_mut::<pdf_obj>();
    } else {
        dst = pdf_new_stream(0 as ::core::ffi::c_int);
        pdf_add_stream(
            dst,
            buf as *const ::core::ffi::c_void,
            n as ::core::ffi::c_int,
        );
    }
    free(buf as *mut ::core::ffi::c_void);
    return dst;
}
unsafe extern "C-unwind" fn skip_white_a85(
    mut p: *mut *const ::core::ffi::c_char,
    mut endptr: *const ::core::ffi::c_char,
) {
    while *p < endptr
        && (**p as ::core::ffi::c_int == ' ' as i32
            || **p as ::core::ffi::c_int == '\t' as i32
            || **p as ::core::ffi::c_int == '\u{c}' as i32
            || **p as ::core::ffi::c_int == '\r' as i32
            || **p as ::core::ffi::c_int == '\n' as i32
            || **p as ::core::ffi::c_int == '\0' as i32)
    {
        *p = (*p).offset(1);
    }
}
unsafe extern "C-unwind" fn filter_stream_decode_ASCII85Decode(
    mut data: *const ::core::ffi::c_void,
    mut len: size_t,
) -> *mut pdf_obj {
    let mut dst: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut eod: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0;
    let mut p: *const ::core::ffi::c_char = data as *const ::core::ffi::c_char;
    let mut endptr: *const ::core::ffi::c_char = p.offset(len as isize);
    let mut buf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut n: size_t = 0;
    buf = new((len
        .wrapping_add(4 as size_t)
        .wrapping_div(5 as size_t)
        .wrapping_mul(4 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    skip_white_a85(&raw mut p, endptr);
    n = 0 as size_t;
    eod = 0 as ::core::ffi::c_int;
    error = 0 as ::core::ffi::c_int;
    while p < endptr && error == 0 && eod == 0 {
        let mut q: [::core::ffi::c_char; 5] = [
            'u' as i32 as ::core::ffi::c_char,
            'u' as i32 as ::core::ffi::c_char,
            'u' as i32 as ::core::ffi::c_char,
            'u' as i32 as ::core::ffi::c_char,
            'u' as i32 as ::core::ffi::c_char,
        ];
        let mut m: ::core::ffi::c_int = 0;
        let mut ch: ::core::ffi::c_char = 0;
        ch = *p.offset(0 as ::core::ffi::c_int as isize);
        p = p.offset(1);
        skip_white_a85(&raw mut p, endptr);
        if ch as ::core::ffi::c_int == 'z' as i32 {
            memset(
                buf.offset(n as isize) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                4 as size_t,
            );
            n = n.wrapping_add(4 as size_t);
        } else if ch as ::core::ffi::c_int == '~' as i32 {
            if p < endptr
                && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '>' as i32
            {
                eod = 1 as ::core::ffi::c_int;
                p = p.offset(1);
            } else {
                error = -(1 as ::core::ffi::c_int);
            }
            break;
        } else {
            q[0 as ::core::ffi::c_int as usize] = ch;
            m = 1 as ::core::ffi::c_int;
            while m < 5 as ::core::ffi::c_int && p < endptr {
                ch = *p.offset(0 as ::core::ffi::c_int as isize);
                p = p.offset(1);
                skip_white_a85(&raw mut p, endptr);
                if ch as ::core::ffi::c_int == '~' as i32 {
                    if p < endptr
                        && *p.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                            == '>' as i32
                    {
                        eod = 1 as ::core::ffi::c_int;
                        p = p.offset(1);
                    } else {
                        error = -(1 as ::core::ffi::c_int);
                    }
                    break;
                } else if (ch as ::core::ffi::c_int) < '!' as i32
                    || ch as ::core::ffi::c_int > 'u' as i32
                {
                    error = -(1 as ::core::ffi::c_int);
                    break;
                } else {
                    q[m as usize] = ch;
                    m += 1;
                }
            }
            if !(error == 0) {
                continue;
            }
            let mut val: uint32_t = 0 as uint32_t;
            let mut i: ::core::ffi::c_int = 0;
            if m <= 1 as ::core::ffi::c_int {
                error = -(1 as ::core::ffi::c_int);
                break;
            } else {
                val = (85 as ::core::ffi::c_int
                    * 85 as ::core::ffi::c_int
                    * 85 as ::core::ffi::c_int
                    * (q[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int - '!' as i32)
                    + 85 as ::core::ffi::c_int
                        * 85 as ::core::ffi::c_int
                        * (q[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int - '!' as i32)
                    + 85 as ::core::ffi::c_int
                        * (q[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int - '!' as i32)
                    + (q[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int - '!' as i32))
                    as uint32_t;
                if val > (UINT32_MAX as uint32_t).wrapping_div(85 as uint32_t) {
                    error = -(1 as ::core::ffi::c_int);
                    break;
                } else {
                    val = (85 as uint32_t).wrapping_mul(val);
                    if val
                        > (UINT32_MAX as uint32_t).wrapping_sub(
                            (q[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int - '!' as i32)
                                as uint32_t,
                        )
                    {
                        error = -(1 as ::core::ffi::c_int);
                        break;
                    } else {
                        val = val.wrapping_add(
                            (q[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int - '!' as i32)
                                as uint32_t,
                        );
                        if error == 0 {
                            i = 3 as ::core::ffi::c_int;
                            while i >= 0 as ::core::ffi::c_int {
                                *buf.offset(n.wrapping_add(i as size_t) as isize) =
                                    (val & 0xff as uint32_t) as ::core::ffi::c_uchar;
                                val = val.wrapping_div(256 as uint32_t);
                                i -= 1;
                            }
                            n = n.wrapping_add((m - 1 as ::core::ffi::c_int) as size_t);
                        }
                    }
                }
            }
        }
    }
    if error != 0 {
        pdf_warning!(
            b"Error in reading ASCII85 data.\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if eod == 0 {
        pdf_warning!(
            b"Error in reading ASCII85 data: No EOD\0" as *const u8 as *const ::core::ffi::c_char,
        );
        dst = ::core::ptr::null_mut::<pdf_obj>();
    } else {
        dst = pdf_new_stream(0 as ::core::ffi::c_int);
        pdf_add_stream(
            dst,
            buf as *const ::core::ffi::c_void,
            n as ::core::ffi::c_int,
        );
    }
    free(buf as *mut ::core::ffi::c_void);
    return dst;
}
unsafe extern "C-unwind" fn filter_stream_decode(
    mut filter_name: *const ::core::ffi::c_char,
    mut src: *mut pdf_obj,
    mut parm: *mut pdf_obj,
) -> *mut pdf_obj {
    let mut dec: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_data: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut stream_length: size_t = 0;
    if filter_name.is_null() {
        return pdf_link_obj(src);
    }
    stream_data = pdf_stream_dataptr(src) as *const ::core::ffi::c_char;
    stream_length = pdf_stream_length(src) as size_t;
    if strcmp(
        filter_name,
        b"ASCIIHexDecode\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        dec = filter_stream_decode_ASCIIHexDecode(
            stream_data as *const ::core::ffi::c_void,
            stream_length,
        );
    } else if strcmp(
        filter_name,
        b"ASCII85Decode\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        dec = filter_stream_decode_ASCII85Decode(
            stream_data as *const ::core::ffi::c_void,
            stream_length,
        );
    } else if strcmp(
        filter_name,
        b"FlateDecode\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        let mut decode_parm: decode_parms = decode_parms {
            predictor: 0,
            colors: 0,
            bits_per_component: 0,
            columns: 0,
        };
        if !parm.is_null() {
            filter_get_DecodeParms_FlateDecode(&raw mut decode_parm, parm);
        }
        dec = filter_stream_decode_FlateDecode(
            stream_data as *const ::core::ffi::c_void,
            stream_length,
            if !parm.is_null() {
                &raw mut decode_parm
            } else {
                ::core::ptr::null_mut::<decode_parms>()
            },
        );
    } else {
        pdf_warning!(
            b"DecodeFilter \"%s\" not supported.\0" as *const u8 as *const ::core::ffi::c_char,
            filter_name,
        );
        dec = ::core::ptr::null_mut::<pdf_obj>();
    }
    return dec;
}
#[unsafe(export_name = "__pdf_impl_pdf_concat_stream")]
pub unsafe extern "C-unwind" fn pdf_concat_stream(
    mut dst: *mut pdf_obj,
    mut src: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut filtered: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut filter: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut parms: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(!dst.is_null() && pdf_obj_typeof(dst) == PDF_STREAM)
        || !(!src.is_null() && pdf_obj_typeof(src) == PDF_STREAM)
    {
        pdf_warning!(
            b"Passed invalid type in pdf_concat_stream().\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    stream_dict = pdf_stream_dict(src);
    filter = pdf_lookup_dict(
        stream_dict,
        b"Filter\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if filter.is_null() {
        pdf_add_stream(dst, pdf_stream_dataptr(src), pdf_stream_length(src));
        return 0 as ::core::ffi::c_int;
    }
    if !pdf_lookup_dict(
        stream_dict,
        b"DecodeParms\0" as *const u8 as *const ::core::ffi::c_char,
    )
    .is_null()
    {
        parms = pdf_deref_obj(pdf_lookup_dict(
            stream_dict,
            b"DecodeParms\0" as *const u8 as *const ::core::ffi::c_char,
        ));
        if parms.is_null() {
            pdf_warning!(
                b"Failed to deref DeocdeParms...\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        } else if !(!parms.is_null() && pdf_obj_typeof(parms) == PDF_ARRAY)
            && !(!parms.is_null() && pdf_obj_typeof(parms) == PDF_DICT)
        {
            pdf_warning!(
                b"PDF dict or array expected for DecodeParms...\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            pdf_release_obj(parms);
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        parms = ::core::ptr::null_mut::<pdf_obj>();
    }
    if !filter.is_null() && pdf_obj_typeof(filter) == PDF_ARRAY {
        let mut i: ::core::ffi::c_int = 0;
        let mut num: ::core::ffi::c_int = 0;
        let mut prev: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        num = pdf_array_length(filter) as ::core::ffi::c_int;
        if !parms.is_null() {
            if !(!parms.is_null() && pdf_obj_typeof(parms) == PDF_ARRAY)
                || pdf_array_length(parms) != num as ::core::ffi::c_uint
            {
                pdf_warning!(
                    b"Invalid DecodeParam object found.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                pdf_release_obj(parms);
                return -(1 as ::core::ffi::c_int);
            }
        }
        if num == 0 as ::core::ffi::c_int {
            filtered = pdf_link_obj(src);
        } else {
            filtered = ::core::ptr::null_mut::<pdf_obj>();
            prev = pdf_link_obj(src);
            i = 0 as ::core::ffi::c_int;
            while i < num && !prev.is_null() {
                let mut tmp1: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                let mut tmp2: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                tmp1 = pdf_deref_obj(pdf_get_array(filter, i));
                if !parms.is_null() {
                    tmp2 = pdf_deref_obj(pdf_get_array(parms, i));
                } else {
                    tmp2 = ::core::ptr::null_mut::<pdf_obj>();
                }
                if !tmp1.is_null() && pdf_obj_typeof(tmp1) == PDF_NAME {
                    filtered = filter_stream_decode(pdf_name_value(tmp1), prev, tmp2);
                } else if !tmp1.is_null() && pdf_obj_typeof(tmp1) == PDF_NULL {
                    filtered = pdf_link_obj(prev);
                } else {
                    pdf_warning!(
                        b"Unexpected object found for /Filter...\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    filtered = ::core::ptr::null_mut::<pdf_obj>();
                }
                if !prev.is_null() {
                    pdf_release_obj(prev);
                }
                if !tmp1.is_null() {
                    pdf_release_obj(tmp1);
                }
                if !tmp2.is_null() {
                    pdf_release_obj(tmp2);
                }
                prev = filtered;
                i += 1;
            }
        }
    } else if !filter.is_null() && pdf_obj_typeof(filter) == PDF_NAME {
        filtered = filter_stream_decode(pdf_name_value(filter), src, parms);
    } else {
        pdf_warning!(
            b"Invalid value for /Filter found.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        filtered = ::core::ptr::null_mut::<pdf_obj>();
    }
    if !parms.is_null() {
        pdf_release_obj(parms);
    }
    if !filtered.is_null() {
        pdf_add_stream(
            dst,
            pdf_stream_dataptr(filtered),
            pdf_stream_length(filtered),
        );
        pdf_release_obj(filtered);
        error = 0 as ::core::ffi::c_int;
    } else {
        error = -(1 as ::core::ffi::c_int);
    }
    return error;
}
unsafe extern "C-unwind" fn pdf_stream_uncompress(mut src: *mut pdf_obj) -> *mut pdf_obj {
    let mut dst: *mut pdf_obj = pdf_new_stream(0 as ::core::ffi::c_int);
    if src.is_null() || (*src).type_0 != 7 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !src.is_null() {
                (*src).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            7 as ::core::ffi::c_int,
            2748 as ::core::ffi::c_int,
        );
    }
    pdf_merge_dict(pdf_stream_dict(dst), pdf_stream_dict(src));
    pdf_remove_dict(
        pdf_stream_dict(dst),
        b"Length\0" as *const u8 as *const ::core::ffi::c_char,
    );
    pdf_concat_stream(dst, src);
    return dst;
}
unsafe extern "C-unwind" fn pdf_write_obj(mut p: *mut pdf_out, mut object: *mut pdf_obj) {
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if object.is_null() {
        write_null(p);
        return;
    }
    if object.is_null()
        || (*object).type_0 <= 0 as ::core::ffi::c_int
        || (*object).type_0 > PDF_UNDEFINED
        || !object.is_null() && pdf_obj_typeof(object) == PDF_UNDEFINED
    {
        pdf_abort!(
            b"pdf_write_obj: Invalid object, type = %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*object).type_0,
        );
    }
    match (*object).type_0 {
        PDF_BOOLEAN => {
            write_boolean(p, (*object).data as *mut pdf_boolean);
        }
        PDF_NUMBER => {
            write_number(p, (*object).data as *mut pdf_number);
        }
        PDF_STRING => {
            write_string(p, (*object).data as *mut pdf_string);
        }
        PDF_NAME => {
            write_name(p, (*object).data as *mut pdf_name);
        }
        PDF_ARRAY => {
            write_array(p, (*object).data as *mut pdf_array);
        }
        PDF_DICT => {
            write_dict(p, (*object).data as *mut pdf_dict);
        }
        PDF_STREAM => {
            write_stream(p, (*object).data as *mut pdf_stream);
        }
        PDF_NULL => {
            write_null(p);
        }
        PDF_INDIRECT => {
            write_indirect(p, (*object).data as *mut pdf_indirect);
        }
        _ => {}
    };
}
unsafe extern "C-unwind" fn pdf_flush_obj(mut p: *mut pdf_out, mut object: *mut pdf_obj) {
    let mut length: size_t = 0;
    let mut buf: [::core::ffi::c_char; 64] = [0; 64];
    add_xref_entry(
        p,
        (*object).label,
        1 as uint8_t,
        (*p).output.file_position as uint32_t,
        (*object).generation,
    );
    length = sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"%u %hu obj\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*object).label,
        (*object).generation as ::core::ffi::c_int,
    ) as size_t;
    (*p).state.enc_mode =
        if (*p).options.enable_encrypt != 0 && (*object).flags & OBJ_NO_ENCRYPT as int32_t == 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    if (*p).state.enc_mode != 0 {
        pdf_enc_set_label((*p).sec_data, (*object).label as ::core::ffi::c_uint);
        pdf_enc_set_generation((*p).sec_data, (*object).generation as ::core::ffi::c_uint);
    }
    pdf_out_str(
        p,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        length,
    );
    pdf_write_obj(p, object);
    pdf_out_str(
        p,
        b"\nendobj\n\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        8 as size_t,
    );
}
unsafe extern "C-unwind" fn pdf_add_objstm(
    mut p: *mut pdf_out,
    mut objstm: *mut pdf_obj,
    mut object: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut data: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut pos: ::core::ffi::c_int = 0;
    if objstm.is_null() || (*objstm).type_0 != 7 as ::core::ffi::c_int {
        pdf_abort!(
            b"typecheck: Invalid object type: %d %d (line %d)\0" as *const u8
                as *const ::core::ffi::c_char,
            if !objstm.is_null() {
                (*objstm).type_0
            } else {
                -(1 as ::core::ffi::c_int)
            },
            7 as ::core::ffi::c_int,
            2830 as ::core::ffi::c_int,
        );
    }
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    data = get_objstm_data(objstm);
    let ref mut fresh5 = *data.offset(0 as ::core::ffi::c_int as isize);
    *fresh5 += 1;
    pos = *fresh5;
    *data.offset((2 as ::core::ffi::c_int * pos) as isize) = (*object).label as ::core::ffi::c_int;
    *data.offset((2 as ::core::ffi::c_int * pos + 1 as ::core::ffi::c_int) as isize) =
        pdf_stream_length(objstm);
    add_xref_entry(
        p,
        (*object).label,
        2 as uint8_t,
        (*objstm).label,
        (pos - 1 as ::core::ffi::c_int) as uint16_t,
    );
    (*p).output_stream = objstm;
    (*p).state.enc_mode = false_0;
    pdf_write_obj(p, object);
    pdf_out_char(p, '\n' as i32 as ::core::ffi::c_char);
    (*p).output_stream = ::core::ptr::null_mut::<pdf_obj>();
    return pos;
}
unsafe extern "C-unwind" fn release_objstm(mut objstm: *mut pdf_obj) {
    let mut data: *mut ::core::ffi::c_int = get_objstm_data(objstm);
    let mut pos: ::core::ffi::c_int = *data.offset(0 as ::core::ffi::c_int as isize);
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut stream: *mut pdf_stream = ::core::ptr::null_mut::<pdf_stream>();
    let mut old_buf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut old_length: size_t = 0;
    stream = (*objstm).data as *mut pdf_stream;
    old_buf = (*stream).stream;
    old_length = (*stream).stream_length;
    (*stream).stream = new(
        (old_length.wrapping_add((22 as ::core::ffi::c_int * pos) as size_t) as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_uchar;
    (*stream).stream_length = 0 as size_t;
    let mut i: ::core::ffi::c_int = 2 as ::core::ffi::c_int * pos;
    let mut val: *mut ::core::ffi::c_int = data.offset(2 as ::core::ffi::c_int as isize);
    loop {
        let fresh3 = i;
        i = i - 1;
        if !(fresh3 != 0) {
            break;
        }
        let mut buf: [::core::ffi::c_char; 32] = [0; 32];
        let fresh4 = val;
        val = val.offset(1);
        let mut length: size_t = sprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"%d \0" as *const u8 as *const ::core::ffi::c_char,
            *fresh4,
        ) as size_t;
        pdf_add_stream(
            objstm,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            length as ::core::ffi::c_int,
        );
    }
    dict = pdf_stream_dict(objstm);
    pdf_add_dict(
        dict,
        pdf_new_name(b"Type\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"ObjStm\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"N\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(pos as ::core::ffi::c_double),
    );
    pdf_add_dict(
        dict,
        pdf_new_name(b"First\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number((*stream).stream_length as ::core::ffi::c_double),
    );
    pdf_add_stream(
        objstm,
        old_buf as *const ::core::ffi::c_void,
        old_length as ::core::ffi::c_int,
    );
    free(old_buf as *mut ::core::ffi::c_void);
    pdf_release_obj(objstm);
}
#[unsafe(export_name = "__pdf_impl_pdf_pipeline_drain")]
pub unsafe extern "C-unwind" fn pdf_pipeline_drain(mut all: ::core::ffi::c_int) {
    let mut p: *mut pdf_out = current_output();
    let mut result: tt_pdf_task_result = tt_pdf_task_result {
        target: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        data: ::core::ptr::null::<::core::ffi::c_uchar>(),
        length: 0,
        kind: 0,
        status: 0,
        handle: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    loop {
        if all != 0 && !(*p).completed_head.is_null() {
            let mut entry: *mut completed_object = (*p).completed_head;
            (*p).completed_head = (*entry).next as *mut completed_object;
            if (*p).completed_head.is_null() {
                (*p).completed_tail = ::core::ptr::null_mut::<completed_object>();
            }
            pdf_release_obj((*entry).object);
            free(entry as *mut ::core::ffi::c_void);
        } else {
            if tt_pdf_pipeline_take(&raw mut result) == 0 {
                break;
            }
            let mut object: *mut pdf_obj = result.target as *mut pdf_obj;
            if result.status < 0 as ::core::ffi::c_int {
                tt_pdf_pipeline_release(result.handle);
                (*object).label = 0 as uint32_t;
                pdf_release_obj(object);
                pdf_abort!(b"PDF worker failed\0" as *const u8 as *const ::core::ffi::c_char,);
            }
            if result.kind == 0 as ::core::ffi::c_int {
                if result.length > INT_MAX as size_t {
                    tt_pdf_pipeline_release(result.handle);
                    (*object).label = 0 as uint32_t;
                    pdf_release_obj(object);
                    pdf_abort!(
                        b"PDF page is too large\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                pdf_add_stream(
                    object,
                    result.data as *const ::core::ffi::c_void,
                    result.length as ::core::ffi::c_int,
                );
            } else {
                let mut stream: *mut pdf_stream = (*object).data as *mut pdf_stream;
                let mut filters: *mut pdf_obj = pdf_lookup_dict(
                    (*stream).dict as *mut pdf_obj,
                    b"Filter\0" as *const u8 as *const ::core::ffi::c_char,
                );
                let mut name: *mut pdf_obj =
                    pdf_new_name(b"FlateDecode\0" as *const u8 as *const ::core::ffi::c_char);
                if !filters.is_null() {
                    pdf_unshift_array(filters, name);
                } else {
                    pdf_add_dict(
                        (*stream).dict as *mut pdf_obj,
                        pdf_new_name(b"Filter\0" as *const u8 as *const ::core::ffi::c_char),
                        name,
                    );
                }
                (*stream).stream = new((result.length as uint32_t as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
                    as uint32_t) as *mut ::core::ffi::c_uchar;
                memcpy(
                    (*stream).stream as *mut ::core::ffi::c_void,
                    result.data as *const ::core::ffi::c_void,
                    result.length,
                );
                if (*stream).stream_length > result.length {
                    (*p).output.compression_saved = (*p)
                        .output
                        .compression_saved
                        .wrapping_add((*stream).stream_length.wrapping_sub(result.length));
                }
                (*stream).max_length = result.length;
                (*stream).stream_length = (*stream).max_length;
                (*stream)._flags =
                    ((*stream)._flags as ::core::ffi::c_int & !STREAM_COMPRESS) as int32_t;
            }
            tt_pdf_pipeline_release(result.handle);
            if all != 0 {
                pdf_release_obj(object);
            } else {
                let mut entry_0: *mut completed_object =
                    new((1 as ::core::ffi::c_int as uint32_t as usize)
                        .wrapping_mul(::core::mem::size_of::<completed_object>() as usize)
                        as uint32_t) as *mut completed_object;
                (*entry_0).object = object;
                (*entry_0).next = ::core::ptr::null_mut::<completed_object>();
                if !(*p).completed_tail.is_null() {
                    (*(*p).completed_tail).next = entry_0 as *mut completed_object;
                } else {
                    (*p).completed_head = entry_0;
                }
                (*p).completed_tail = entry_0;
                break;
            }
        }
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_submit_page")]
pub unsafe extern "C-unwind" fn pdf_submit_page(mut contents: *mut pdf_obj) -> ::core::ffi::c_int {
    let mut p: *mut pdf_out = current_output();
    return tt_pdf_page_submit(
        pdf_link_obj(contents) as *mut ::core::ffi::c_void,
        (*p).options.compression.level as ::core::ffi::c_uint,
    );
}
unsafe extern "C-unwind" fn flush_compression_batch(mut p: *mut pdf_out) {
    let mut jobs: [tt_pdf_compression_job; 64] = [tt_pdf_compression_job {
        input: ::core::ptr::null::<::core::ffi::c_uchar>(),
        input_len: 0,
        output: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        output_len: 0,
        result: 0,
    }; 64];
    let mut i: size_t = 0;
    let mut count: size_t = (*p).compression_batch_count;
    if count == 0 {
        return;
    }
    (*p).flushing_compression_batch = 1 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < count {
        let mut stream: *mut pdf_stream =
            (*(*p).compression_batch[i as usize]).data as *mut pdf_stream;
        let mut capacity: size_t = (*stream)
            .stream_length
            .wrapping_add((*stream).stream_length.wrapping_div(1000 as size_t))
            .wrapping_add(14 as size_t);
        jobs[i as usize].input = (*stream).stream;
        jobs[i as usize].input_len = (*stream).stream_length as uint64_t;
        jobs[i as usize].output = new((capacity as uint32_t as usize)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
            as uint32_t) as *mut ::core::ffi::c_uchar;
        jobs[i as usize].output_len = capacity as uint64_t;
        jobs[i as usize].result = -(2 as ::core::ffi::c_int) as int32_t;
        i = i.wrapping_add(1);
    }
    tt_pdf_compress_streams(
        &raw mut jobs as *mut tt_pdf_compression_job,
        count,
        (*p).options.compression.level as uint32_t,
    );
    i = 0 as size_t;
    while i < count {
        if jobs[i as usize].result < 0 as int32_t {
            let mut j: size_t = 0;
            j = 0 as size_t;
            while j < count {
                free(jobs[j as usize].output as *mut ::core::ffi::c_void);
                j = j.wrapping_add(1);
            }
            pdf_abort!(b"Zlib error\0" as *const u8 as *const ::core::ffi::c_char);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < count {
        let mut stream_0: *mut pdf_stream =
            (*(*p).compression_batch[i as usize]).data as *mut pdf_stream;
        let mut filters: *mut pdf_obj = pdf_lookup_dict(
            (*stream_0).dict as *mut pdf_obj,
            b"Filter\0" as *const u8 as *const ::core::ffi::c_char,
        );
        let mut filter_name: *mut pdf_obj =
            pdf_new_name(b"FlateDecode\0" as *const u8 as *const ::core::ffi::c_char);
        if !filters.is_null() {
            pdf_unshift_array(filters, filter_name);
        } else {
            pdf_add_dict(
                (*stream_0).dict as *mut pdf_obj,
                pdf_new_name(b"Filter\0" as *const u8 as *const ::core::ffi::c_char),
                filter_name,
            );
        }
        (*p).output.compression_saved = ((*p).output.compression_saved as uint64_t).wrapping_add(
            ((*stream_0).stream_length as uint64_t)
                .wrapping_sub(jobs[i as usize].output_len)
                .wrapping_sub(
                    (if !filters.is_null() {
                        strlen(b"/FlateDecode \0" as *const u8 as *const ::core::ffi::c_char)
                    } else {
                        strlen(
                            b"/Filter/FlateDecode\n\0" as *const u8 as *const ::core::ffi::c_char,
                        )
                    }) as uint64_t,
                ),
        ) as size_t as size_t;
        free((*stream_0).stream as *mut ::core::ffi::c_void);
        (*stream_0).stream = jobs[i as usize].output;
        (*stream_0).stream_length = jobs[i as usize].output_len as size_t;
        (*stream_0).max_length = jobs[i as usize].output_len as size_t;
        (*stream_0)._flags =
            ((*stream_0)._flags as ::core::ffi::c_int & !STREAM_COMPRESS) as int32_t;
        i = i.wrapping_add(1);
    }
    while (*p).compression_batch_count != 0 {
        let mut object: *mut pdf_obj = (*p).compression_batch[0 as ::core::ffi::c_int as usize];
        (*p).compression_batch_count = (*p).compression_batch_count.wrapping_sub(1);
        memmove(
            &raw mut (*p).compression_batch as *mut *mut pdf_obj as *mut ::core::ffi::c_void,
            (&raw mut (*p).compression_batch as *mut *mut pdf_obj)
                .offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (*p).compression_batch_count
                .wrapping_mul(::core::mem::size_of::<*mut pdf_obj>() as size_t),
        );
        pdf_release_obj(object);
    }
    (*p).compression_batch_bytes = 0 as size_t;
    (*p).flushing_compression_batch = 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn defer_stream_compression(
    mut p: *mut pdf_out,
    mut object: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut stream: *mut pdf_stream = ::core::ptr::null_mut::<pdf_stream>();
    let mut type_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if (*p).defer_compression == 0
        || (*p).flushing_compression_batch != 0
        || (*object).type_0 != PDF_STREAM
        || (*object).label == 0
        || (*p).output.handle == INVALID_HANDLE
        || (*p).options.compression.level <= 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    stream = (*object).data as *mut pdf_stream;
    if tt_pdf_pipeline_active() != 0
        && (*stream)._flags & STREAM_COMPRESS as int32_t != 0
        && (*stream).stream_length > 0 as size_t
    {
        let mut predictor: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        type_0 = pdf_lookup_dict(
            (*stream).dict as *mut pdf_obj,
            b"Type\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !type_0.is_null()
            && streq_ptr(
                b"Metadata\0" as *const u8 as *const ::core::ffi::c_char,
                pdf_name_value(type_0),
            ) as ::core::ffi::c_int
                != 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*p).options.compression.use_predictor != 0
            && (*stream)._flags & STREAM_USE_PREDICTOR as int32_t != 0
            && pdf_lookup_dict(
                (*stream).dict as *mut pdf_obj,
                b"DecodeParms\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
        {
            predictor = (*stream).decodeparms.predictor;
            if predictor != 2 as ::core::ffi::c_int && predictor != 15 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
        }
        while tt_pdf_pipeline_has_capacity((*stream).stream_length) == 0 {
            pdf_pipeline_drain(0 as ::core::ffi::c_int);
        }
        if predictor != 0 {
            let mut parms: *mut pdf_obj = filter_create_predictor_dict(
                predictor,
                (*stream).decodeparms.columns,
                (*stream).decodeparms.bits_per_component,
                (*stream).decodeparms.colors,
            );
            pdf_add_dict(
                (*stream).dict as *mut pdf_obj,
                pdf_new_name(b"DecodeParms\0" as *const u8 as *const ::core::ffi::c_char),
                parms,
            );
        }
        (*object).refcount = 1 as ::core::ffi::c_int;
        if tt_pdf_stream_submit(
            object as *mut ::core::ffi::c_void,
            (*stream).stream,
            (*stream).stream_length,
            (*p).options.compression.level as ::core::ffi::c_uint,
            predictor,
            (*stream).decodeparms.columns as ::core::ffi::c_int,
            (*stream).decodeparms.bits_per_component,
            (*stream).decodeparms.colors,
        ) == 0
        {
            pdf_abort!(b"Could not submit PDF stream\0" as *const u8 as *const ::core::ffi::c_char,);
        }
        free((*stream).stream as *mut ::core::ffi::c_void);
        (*stream).stream = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        return 1 as ::core::ffi::c_int;
    }
    if (*stream)._flags & STREAM_COMPRESS as int32_t == 0
        || (*stream).stream_length
            < (if (*stream)._flags & STREAM_PAGE_CONTENT as int32_t != 0 {
                1024 as ::core::ffi::c_int
            } else {
                16 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int
            }) as size_t
        || (*stream).stream_length
            >= (1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t
        || (*p).options.compression.use_predictor != 0
            && (*stream)._flags & STREAM_USE_PREDICTOR as int32_t != 0
            && pdf_lookup_dict(
                (*stream).dict as *mut pdf_obj,
                b"DecodeParms\0" as *const u8 as *const ::core::ffi::c_char,
            )
            .is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    type_0 = pdf_lookup_dict(
        (*stream).dict as *mut pdf_obj,
        b"Type\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !type_0.is_null()
        && streq_ptr(
            b"Metadata\0" as *const u8 as *const ::core::ffi::c_char,
            pdf_name_value(type_0),
        ) as ::core::ffi::c_int
            != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*p)
        .compression_batch_bytes
        .wrapping_add((*stream).stream_length)
        > COMPRESSION_BATCH_BYTES as size_t
    {
        flush_compression_batch(p);
    }
    (*object).refcount = 1 as ::core::ffi::c_int;
    let fresh19 = (*p).compression_batch_count;
    (*p).compression_batch_count = (*p).compression_batch_count.wrapping_add(1);
    (*p).compression_batch[fresh19 as usize] = object;
    (*p).compression_batch_bytes = (*p)
        .compression_batch_bytes
        .wrapping_add((*stream).stream_length);
    if (*p).compression_batch_count == COMPRESSION_BATCH_SIZE as size_t
        || (*p).compression_batch_bytes >= COMPRESSION_BATCH_BYTES as size_t
    {
        flush_compression_batch(p);
    }
    return 1 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_release_obj")]
pub unsafe extern "C-unwind" fn pdf_release_obj(mut object: *mut pdf_obj) {
    let mut p: *mut pdf_out = current_output();
    if object.is_null() {
        return;
    }
    if object.is_null()
        || (*object).type_0 <= 0 as ::core::ffi::c_int
        || (*object).type_0 > PDF_UNDEFINED
        || (*object).refcount <= 0 as ::core::ffi::c_int
    {
        pdf_message!(
            b"\npdf_release_obj: object=%p, type=%d, refcount=%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            object,
            (*object).type_0,
            (*object).refcount,
        );
        pdf_abort!(
            b"pdf_release_obj:  Called with invalid object.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*object).refcount -= 1 as ::core::ffi::c_int;
    if (*object).refcount == 0 as ::core::ffi::c_int {
        if defer_stream_compression(p, object) != 0 {
            return;
        }
        if (*object).label != 0 {
            let ref mut fresh0 = *(*p)
                .free_list
                .offset((*object).label.wrapping_div(8 as uint32_t) as isize);
            *fresh0 = (*fresh0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int)
                    << (7 as uint32_t).wrapping_sub((*object).label.wrapping_rem(8 as uint32_t)))
                as ::core::ffi::c_char;
            if (*p).output.handle != INVALID_HANDLE {
                if (*p).options.use_objstm == 0
                    || (*object).flags & OBJ_NO_OBJSTM as int32_t != 0
                    || (*p).options.enable_encrypt != 0
                        && (*object).flags & OBJ_NO_ENCRYPT as int32_t != 0
                    || (*object).generation as ::core::ffi::c_int != 0
                {
                    pdf_flush_obj(p, object);
                } else {
                    if (*p).current_objstm.is_null() {
                        let mut data: *mut ::core::ffi::c_int =
                            new(((2 as ::core::ffi::c_int * 200 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int)
                                as uint32_t as usize)
                                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                as uint32_t) as *mut ::core::ffi::c_int;
                        let ref mut fresh1 = *data.offset(1 as ::core::ffi::c_int as isize);
                        *fresh1 = 0 as ::core::ffi::c_int;
                        *data.offset(0 as ::core::ffi::c_int as isize) = *fresh1;
                        (*p).current_objstm = pdf_new_stream(STREAM_COMPRESS);
                        set_objstm_data((*p).current_objstm, data);
                        pdf_label_obj(p, (*p).current_objstm);
                    }
                    if pdf_add_objstm(p, (*p).current_objstm, object) == OBJSTM_MAX_OBJS {
                        release_objstm((*p).current_objstm);
                        (*p).current_objstm = ::core::ptr::null_mut::<pdf_obj>();
                    }
                }
            }
        }
        match (*object).type_0 {
            PDF_BOOLEAN => {
                release_boolean((*object).data as *mut pdf_obj);
            }
            PDF_NUMBER => {
                release_number((*object).data as *mut pdf_number);
            }
            PDF_STRING => {
                release_string((*object).data as *mut pdf_string);
            }
            PDF_NAME => {
                release_name((*object).data as *mut pdf_name);
            }
            PDF_ARRAY => {
                release_array((*object).data as *mut pdf_array);
            }
            PDF_DICT => {
                release_dict((*object).data as *mut pdf_dict);
            }
            PDF_STREAM => {
                release_stream((*object).data as *mut pdf_stream);
            }
            PDF_INDIRECT => {
                release_indirect((*object).data as *mut pdf_indirect);
            }
            PDF_NULL | _ => {}
        }
        (*object).type_0 = -(1 as ::core::ffi::c_int);
        (*object).data = NULL;
        free(object as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C-unwind" fn tt_mfreadln(
    mut buf: *mut ::core::ffi::c_char,
    mut size: ::core::ffi::c_int,
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        c = ttstub_input_getc(handle);
        if !(c != EOF && c != '\n' as i32 && c != '\r' as i32) {
            break;
        }
        if len >= size {
            return -(2 as ::core::ffi::c_int);
        }
        let fresh40 = len;
        len = len + 1;
        *buf.offset(fresh40 as isize) = c as ::core::ffi::c_char;
    }
    if c == EOF && len == 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if c == '\r' as i32
        && {
            c = ttstub_input_getc(handle);
            c >= 0 as ::core::ffi::c_int
        }
        && c != '\n' as i32
    {
        ttstub_input_ungetc(handle, c);
    }
    return len;
}
unsafe extern "C-unwind" fn backup_line(mut handle: rust_input_handle_t) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_CUR) > 1 as size_t {
        loop {
            crate::backend::runtime::input_seek(
                handle,
                -(2 as ::core::ffi::c_int) as ssize_t,
                SEEK_CUR,
            );
            if !(crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_CUR) > 0 as size_t
                && {
                    ch = ttstub_input_getc(handle);
                    ch >= 0 as ::core::ffi::c_int
                }
                && (ch != '\n' as i32 && ch != '\r' as i32))
            {
                break;
            }
        }
    }
    if ch < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn find_xref(
    mut handle: rust_input_handle_t,
    mut file_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut xref_pos: size_t = 0 as size_t;
    let mut len: ::core::ffi::c_int = 0;
    let mut tries: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
    let mut start: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut end: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut number: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    loop {
        let mut currentpos: size_t = 0;
        let mut n: ::core::ffi::c_int = 0;
        if backup_line(handle) == 0 {
            tries = 0 as ::core::ffi::c_int;
            break;
        } else {
            currentpos = crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_CUR);
            n = (if strlen(b"startxref\0" as *const u8 as *const ::core::ffi::c_char)
                < (file_size as size_t).wrapping_sub(currentpos)
            {
                strlen(b"startxref\0" as *const u8 as *const ::core::ffi::c_char)
            } else {
                (file_size as size_t).wrapping_sub(currentpos)
            }) as ::core::ffi::c_int;
            ttstub_input_read(
                handle,
                &raw mut work_buffer as *mut ::core::ffi::c_char,
                n as size_t,
            );
            crate::backend::runtime::input_seek(handle, currentpos as ssize_t, SEEK_SET);
            tries -= 1;
            if !(tries > 0 as ::core::ffi::c_int
                && strstartswith(
                    &raw mut work_buffer as *mut ::core::ffi::c_char,
                    b"startxref\0" as *const u8 as *const ::core::ffi::c_char,
                )
                .is_null())
            {
                break;
            }
        }
    }
    if tries <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    tt_mfgets(
        &raw mut work_buffer as *mut ::core::ffi::c_char,
        WORK_BUFFER_SIZE,
        handle,
    );
    len = tt_mfreadln(
        &raw mut work_buffer as *mut ::core::ffi::c_char,
        WORK_BUFFER_SIZE,
        handle,
    );
    if len <= 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Reading xref location data failed... Not a PDF file?\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    start = &raw mut work_buffer as *mut ::core::ffi::c_char;
    end = start.offset(len as isize);
    skip_white(&raw mut start, end);
    number = parse_number(&raw mut start, end);
    xref_pos = atof(number) as ::core::ffi::c_int as size_t;
    free(number as *mut ::core::ffi::c_void);
    return xref_pos as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn parse_trailer(mut pf: *mut pdf_file) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut cur_pos: ::core::ffi::c_int = 0;
    let mut nmax: ::core::ffi::c_int = 0;
    let mut nread: ::core::ffi::c_int = 0;
    cur_pos = crate::backend::runtime::input_seek((*pf).handle, 0 as ssize_t, SEEK_CUR)
        as ::core::ffi::c_int;
    nmax = if (*pf).file_size - cur_pos < 1024 as ::core::ffi::c_int {
        (*pf).file_size - cur_pos
    } else {
        1024 as ::core::ffi::c_int
    };
    nread = ttstub_input_read(
        (*pf).handle,
        &raw mut work_buffer as *mut ::core::ffi::c_char,
        nmax as size_t,
    ) as ::core::ffi::c_int;
    if nread == 0 as ::core::ffi::c_int
        || strstartswith(
            &raw mut work_buffer as *mut ::core::ffi::c_char,
            b"trailer\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
    {
        pdf_warning!(
            b"No trailer.  Are you sure this is a PDF file?\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        pdf_warning!(
            b"buffer:\n->%s<-\n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut work_buffer as *mut ::core::ffi::c_char,
        );
        result = ::core::ptr::null_mut::<pdf_obj>();
    } else {
        let mut p: *const ::core::ffi::c_char = (&raw mut work_buffer as *mut ::core::ffi::c_char)
            .offset(strlen(b"trailer\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        skip_white(
            &raw mut p,
            (&raw mut work_buffer as *mut ::core::ffi::c_char).offset(nread as isize),
        );
        result = parse_pdf_dict(
            &raw mut p,
            (&raw mut work_buffer as *mut ::core::ffi::c_char).offset(nread as isize),
            pf,
        );
    }
    return result;
}
unsafe extern "C-unwind" fn next_object_offset(
    mut pf: *mut pdf_file,
    mut obj_num: uint32_t,
) -> ::core::ffi::c_int {
    let mut next: uint32_t = (*pf).file_size as uint32_t;
    let mut i: size_t = 0;
    let mut curr: uint32_t = 0;
    curr = (*(*pf).xref_table.offset(obj_num as isize)).field2 as uint32_t;
    i = 0 as size_t;
    while i < (*pf).num_obj as size_t {
        if (*(*pf).xref_table.offset(i as isize)).type_0 as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
            && (*(*pf).xref_table.offset(i as isize)).field2 as uint32_t > curr
            && ((*(*pf).xref_table.offset(i as isize)).field2 as uint32_t) < next
        {
            next = (*(*pf).xref_table.offset(i as isize)).field2 as uint32_t;
        }
        i = i.wrapping_add(1);
    }
    return next as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_new_indirect")]
pub unsafe extern "C-unwind" fn pdf_new_indirect(
    mut pf: *mut pdf_file,
    mut obj_num: uint32_t,
    mut obj_gen: uint16_t,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut indirect: *mut pdf_indirect = ::core::ptr::null_mut::<pdf_indirect>();
    indirect = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_indirect>() as usize)
        as uint32_t) as *mut pdf_indirect;
    (*indirect).pf = pf;
    (*indirect).obj = ::core::ptr::null_mut::<pdf_obj>();
    (*indirect).label = obj_num;
    (*indirect).generation = obj_gen;
    result = pdf_new_obj(PDF_INDIRECT);
    (*result).data = indirect as *mut ::core::ffi::c_void;
    return result;
}
unsafe extern "C-unwind" fn pdf_read_object(
    mut obj_num: uint32_t,
    mut obj_gen: uint16_t,
    mut pf: *mut pdf_file,
    mut offset: size_t,
    mut limit: size_t,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut length: size_t = 0;
    let mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    length = limit.wrapping_sub(offset);
    if length <= 0 as size_t {
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    buffer = new((length.wrapping_add(1 as size_t) as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
        as uint32_t) as *mut ::core::ffi::c_char;
    crate::backend::runtime::input_seek((*pf).handle, offset as ssize_t, SEEK_SET);
    ttstub_input_read((*pf).handle, buffer, length);
    p = buffer;
    endptr = p.offset(length as isize);
    let mut q: *const ::core::ffi::c_char = p;
    let mut sp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: uint32_t = 0;
    let mut g: uint32_t = 0;
    skip_white(&raw mut q, endptr);
    sp = parse_unsigned(&raw mut q, endptr);
    if sp.is_null() {
        free(buffer as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    n = strtoul(
        sp,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        10 as ::core::ffi::c_int,
    ) as uint32_t;
    free(sp as *mut ::core::ffi::c_void);
    skip_white(&raw mut q, endptr);
    sp = parse_unsigned(&raw mut q, endptr);
    if sp.is_null() {
        free(buffer as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    g = strtoul(
        sp,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        10 as ::core::ffi::c_int,
    ) as uint32_t;
    free(sp as *mut ::core::ffi::c_void);
    if obj_num != 0 && (n != obj_num || g != obj_gen as uint32_t) {
        free(buffer as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    p = q;
    skip_white(&raw mut p, endptr);
    if memcmp(
        p as *const ::core::ffi::c_void,
        b"obj\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"obj\0" as *const u8 as *const ::core::ffi::c_char),
    ) != 0
    {
        pdf_warning!(b"Didn't find \"obj\".\0" as *const u8 as *const ::core::ffi::c_char,);
        free(buffer as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<pdf_obj>();
    }
    p = p.offset(strlen(b"obj\0" as *const u8 as *const ::core::ffi::c_char) as isize);
    result = parse_pdf_object(&raw mut p, endptr, pf);
    skip_white(&raw mut p, endptr);
    if memcmp(
        p as *const ::core::ffi::c_void,
        b"endobj\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"endobj\0" as *const u8 as *const ::core::ffi::c_char),
    ) != 0
    {
        pdf_warning!(b"Didn't find \"endobj\".\0" as *const u8 as *const ::core::ffi::c_char,);
        pdf_release_obj(result);
        result = ::core::ptr::null_mut::<pdf_obj>();
    }
    free(buffer as *mut ::core::ffi::c_void);
    return result;
}
unsafe extern "C-unwind" fn read_objstm(mut pf: *mut pdf_file, mut num: uint32_t) -> *mut pdf_obj {
    let mut current_block: u64;
    let mut offset: size_t = (*(*pf).xref_table.offset(num as isize)).field2 as size_t;
    let mut r#gen: uint16_t = (*(*pf).xref_table.offset(num as isize)).field3 as uint16_t;
    let mut limit: size_t = next_object_offset(pf, num) as size_t;
    let mut n: ::core::ffi::c_int = 0;
    let mut first: ::core::ffi::c_int = 0;
    let mut header: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut data: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut objstm: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut type_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut n_obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut first_obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    objstm = pdf_read_object(num, r#gen, pf, offset, limit);
    if !objstm.is_null() && pdf_obj_typeof(objstm) == PDF_STREAM {
        let mut tmp: *mut pdf_obj = pdf_stream_uncompress(objstm);
        if !tmp.is_null() {
            pdf_release_obj(objstm);
            objstm = tmp;
            dict = pdf_stream_dict(objstm);
            type_0 = pdf_lookup_dict(dict, b"Type\0" as *const u8 as *const ::core::ffi::c_char);
            if !(!(!type_0.is_null() && pdf_obj_typeof(type_0) == PDF_NAME)
                || strcmp(
                    pdf_name_value(type_0),
                    b"ObjStm\0" as *const u8 as *const ::core::ffi::c_char,
                ) != 0)
            {
                n_obj = pdf_lookup_dict(dict, b"N\0" as *const u8 as *const ::core::ffi::c_char);
                if !n_obj.is_null() && pdf_obj_typeof(n_obj) == PDF_NUMBER {
                    n = pdf_number_value(n_obj) as ::core::ffi::c_int;
                    first_obj = pdf_lookup_dict(
                        dict,
                        b"First\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !first_obj.is_null() && pdf_obj_typeof(first_obj) == PDF_NUMBER {
                        first = pdf_number_value(first_obj) as ::core::ffi::c_int;
                        if !(first >= pdf_stream_length(objstm)) {
                            header = new(((2 as ::core::ffi::c_int * (n + 1 as ::core::ffi::c_int))
                                as uint32_t as usize)
                                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                as uint32_t)
                                as *mut ::core::ffi::c_int;
                            set_objstm_data(objstm, header);
                            let fresh28 = header;
                            header = header.offset(1);
                            *fresh28 = n;
                            let fresh29 = header;
                            header = header.offset(1);
                            *fresh29 = first;
                            data =
                                new(((first + 1 as ::core::ffi::c_int) as uint32_t as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<::core::ffi::c_char>() as usize
                                    ) as uint32_t)
                                    as *mut ::core::ffi::c_char;
                            memcpy(
                                data as *mut ::core::ffi::c_void,
                                pdf_stream_dataptr(objstm),
                                first as size_t,
                            );
                            *data.offset(first as isize) = 0 as ::core::ffi::c_char;
                            p = data;
                            endptr = p.offset(first as isize);
                            i = 2 as ::core::ffi::c_int * n;
                            loop {
                                let fresh30 = i;
                                i = i - 1;
                                if !(fresh30 != 0) {
                                    current_block = 9828876828309294594;
                                    break;
                                }
                                let fresh31 = header;
                                header = header.offset(1);
                                *fresh31 = strtoul(p, &raw mut q, 10 as ::core::ffi::c_int)
                                    as ::core::ffi::c_int;
                                if q == p as *mut ::core::ffi::c_char {
                                    current_block = 5887382598006845503;
                                    break;
                                }
                                p = q;
                            }
                            match current_block {
                                5887382598006845503 => {}
                                _ => {
                                    skip_white(&raw mut p, endptr);
                                    if !(p != endptr) {
                                        free(data as *mut ::core::ffi::c_void);
                                        let ref mut fresh32 =
                                            (*(*pf).xref_table.offset(num as isize)).direct;
                                        *fresh32 = objstm;
                                        return *fresh32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pdf_warning!(b"Cannot parse object stream.\0" as *const u8 as *const ::core::ffi::c_char,);
    free(data as *mut ::core::ffi::c_void);
    pdf_release_obj(objstm);
    return ::core::ptr::null_mut::<pdf_obj>();
}
unsafe extern "C-unwind" fn pdf_get_object(
    mut pf: *mut pdf_file,
    mut obj_num: uint32_t,
    mut obj_gen: uint16_t,
) -> *mut pdf_obj {
    let mut current_block: u64;
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if !(obj_num > 0 as uint32_t
        && obj_num < (*pf).num_obj as uint32_t
        && ((*(*pf).xref_table.offset(obj_num as isize)).type_0 as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
            && (*(*pf).xref_table.offset(obj_num as isize)).field3 as ::core::ffi::c_int
                == obj_gen as ::core::ffi::c_int
            || (*(*pf).xref_table.offset(obj_num as isize)).type_0 as ::core::ffi::c_int
                == 2 as ::core::ffi::c_int
                && obj_gen == 0))
    {
        pdf_warning!(
            b"Trying to read nonexistent or deleted object: %u %hu\0" as *const u8
                as *const ::core::ffi::c_char,
            obj_num,
            obj_gen as ::core::ffi::c_int,
        );
        return pdf_new_null();
    }
    result = (*(*pf).xref_table.offset(obj_num as isize)).direct;
    if !result.is_null() {
        return pdf_link_obj(result);
    }
    if (*(*pf).xref_table.offset(obj_num as isize)).type_0 as ::core::ffi::c_int
        == 1 as ::core::ffi::c_int
    {
        let mut offset: uint32_t = 0;
        let mut limit: size_t = 0;
        offset = (*(*pf).xref_table.offset(obj_num as isize)).field2 as uint32_t;
        limit = next_object_offset(pf, obj_num) as size_t;
        result = pdf_read_object(obj_num, obj_gen, pf, offset as size_t, limit);
    } else {
        let mut objstm_num: uint32_t =
            (*(*pf).xref_table.offset(obj_num as isize)).field2 as uint32_t;
        let mut index: uint16_t = (*(*pf).xref_table.offset(obj_num as isize)).field3 as uint16_t;
        let mut objstm: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut data: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
        let mut n: ::core::ffi::c_int = 0;
        let mut first: ::core::ffi::c_int = 0;
        let mut length: ::core::ffi::c_int = 0;
        let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut q: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if objstm_num >= (*pf).num_obj as uint32_t
            || (*(*pf).xref_table.offset(objstm_num as isize)).type_0 as ::core::ffi::c_int
                != 1 as ::core::ffi::c_int
            || {
                objstm = (*(*pf).xref_table.offset(objstm_num as isize)).direct;
                !(!objstm.is_null() || {
                    objstm = read_objstm(pf, objstm_num);
                    !objstm.is_null()
                })
            }
        {
            current_block = 11420283775579471589;
        } else {
            data = get_objstm_data(objstm);
            let fresh25 = data;
            data = data.offset(1);
            n = *fresh25;
            let fresh26 = data;
            data = data.offset(1);
            first = *fresh26;
            if index as ::core::ffi::c_int >= n
                || *data.offset((2 as ::core::ffi::c_int * index as ::core::ffi::c_int) as isize)
                    as uint32_t
                    != obj_num
            {
                current_block = 11420283775579471589;
            } else {
                length = pdf_stream_length(objstm);
                p = (pdf_stream_dataptr(objstm) as *const ::core::ffi::c_char)
                    .offset(first as isize)
                    .offset(*data.offset(
                        (2 as ::core::ffi::c_int * index as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as isize,
                    ) as isize);
                q = p.offset(
                    (if index as ::core::ffi::c_int == n - 1 as ::core::ffi::c_int {
                        length
                    } else {
                        first
                            + *data.offset(
                                (2 as ::core::ffi::c_int * index as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int)
                                    as isize,
                            )
                    }) as isize,
                );
                result = parse_pdf_object(&raw mut p, q, pf);
                if result.is_null() {
                    current_block = 11420283775579471589;
                } else {
                    current_block = 2370887241019905314;
                }
            }
        }
        match current_block {
            2370887241019905314 => {}
            _ => {
                pdf_warning!(
                    b"Could not read object from object stream.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return pdf_new_null();
            }
        }
    }
    let ref mut fresh27 = (*(*pf).xref_table.offset(obj_num as isize)).direct;
    *fresh27 = pdf_link_obj(result);
    return result;
}
unsafe extern "C-unwind" fn pdf_new_ref(
    mut p: *mut pdf_out,
    mut object: *mut pdf_obj,
) -> *mut pdf_obj {
    let mut result: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if p.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"p\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if (*object).label == 0 as uint32_t {
        pdf_label_obj(p, object);
    }
    result = pdf_new_indirect(
        ::core::ptr::null_mut::<pdf_file>(),
        (*object).label,
        (*object).generation,
    );
    let ref mut fresh21 = (*((*result).data as *mut pdf_indirect)).obj;
    *fresh21 = object;
    return result;
}
#[unsafe(export_name = "__pdf_impl_pdf_deref_obj")]
pub unsafe extern "C-unwind" fn pdf_deref_obj(mut obj: *mut pdf_obj) -> *mut pdf_obj {
    let mut count: ::core::ffi::c_int = PDF_OBJ_MAX_DEPTH;
    if !obj.is_null() {
        obj = pdf_link_obj(obj);
    }
    while !obj.is_null() && pdf_obj_typeof(obj) == PDF_INDIRECT && {
        count -= 1;
        count != 0
    } {
        let mut pf: *mut pdf_file = (*((*obj).data as *mut pdf_indirect)).pf;
        if !pf.is_null() {
            let mut obj_num: uint32_t = (*((*obj).data as *mut pdf_indirect)).label;
            let mut obj_gen: uint16_t = (*((*obj).data as *mut pdf_indirect)).generation;
            pdf_release_obj(obj);
            obj = pdf_get_object(pf, obj_num, obj_gen);
        } else {
            let mut p: *mut pdf_out = current_output();
            let mut data: *mut pdf_indirect = (*obj).data as *mut pdf_indirect;
            if *(*p)
                .free_list
                .offset((*data).label.wrapping_div(8 as uint32_t) as isize)
                as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int)
                    << (7 as uint32_t).wrapping_sub((*data).label.wrapping_rem(8 as uint32_t))
                != 0
            {
                pdf_release_obj(obj);
                return ::core::ptr::null_mut::<pdf_obj>();
            } else {
                let mut next_obj: *mut pdf_obj = (*((*obj).data as *mut pdf_indirect)).obj;
                if next_obj.is_null() {
                    pdf_abort!(
                        b"Undefined object reference\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
                pdf_release_obj(obj);
                obj = pdf_link_obj(next_obj);
            }
        }
    }
    if count == 0 {
        pdf_abort!(
            b"Loop in object hierarchy detected. Broken PDF file?\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if !obj.is_null() && pdf_obj_typeof(obj) == PDF_NULL {
        pdf_release_obj(obj);
        return ::core::ptr::null_mut::<pdf_obj>();
    } else {
        return obj;
    };
}
unsafe extern "C-unwind" fn extend_xref(mut pf: *mut pdf_file, mut new_size: size_t) {
    let mut i: size_t = 0;
    (*pf).xref_table = renew(
        (*pf).xref_table as *mut ::core::ffi::c_void,
        (new_size as uint32_t as usize).wrapping_mul(::core::mem::size_of::<xref_entry>() as usize)
            as uint32_t,
    ) as *mut xref_entry;
    i = (*pf).num_obj as size_t;
    while i < new_size {
        let ref mut fresh38 = (*(*pf).xref_table.offset(i as isize)).direct;
        *fresh38 = ::core::ptr::null_mut::<pdf_obj>();
        let ref mut fresh39 = (*(*pf).xref_table.offset(i as isize)).indirect;
        *fresh39 = ::core::ptr::null_mut::<pdf_obj>();
        (*(*pf).xref_table.offset(i as isize)).type_0 = 0 as ::core::ffi::c_uchar;
        (*(*pf).xref_table.offset(i as isize)).field3 = 0 as ::core::ffi::c_ushort;
        (*(*pf).xref_table.offset(i as isize)).field2 = 0 as ::core::ffi::c_uint;
        i = i.wrapping_add(1);
    }
    (*pf).num_obj = new_size as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn parse_xref_table(
    mut pf: *mut pdf_file,
    mut xref_pos: size_t,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut len: ::core::ffi::c_int = 0;
    crate::backend::runtime::input_seek((*pf).handle, xref_pos as ssize_t, SEEK_SET);
    len = tt_mfreadln(
        &raw mut buf as *mut ::core::ffi::c_char,
        255 as ::core::ffi::c_int,
        (*pf).handle,
    );
    if len < 0 as ::core::ffi::c_int {
        pdf_warning!(
            b"Something went wrong while reading xref table...giving up.\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    p = &raw mut buf as *mut ::core::ffi::c_char;
    endptr = (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize);
    if memcmp(
        p as *const ::core::ffi::c_void,
        b"xref\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"xref\0" as *const u8 as *const ::core::ffi::c_char),
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    p = p.offset(strlen(b"xref\0" as *const u8 as *const ::core::ffi::c_char) as isize);
    skip_white(&raw mut p, endptr);
    if p != endptr {
        pdf_warning!(
            b"Garbage after \"xref\" keyword found.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    loop {
        let mut flag: ::core::ffi::c_char = 0;
        let mut current_pos: size_t = 0;
        let mut size: size_t = 0;
        let mut offset: size_t = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut first: uint32_t = 0;
        let mut obj_gen: uint32_t = 0;
        current_pos = crate::backend::runtime::input_seek((*pf).handle, 0 as ssize_t, SEEK_CUR);
        len = tt_mfreadln(
            &raw mut buf as *mut ::core::ffi::c_char,
            255 as ::core::ffi::c_int,
            (*pf).handle,
        );
        if len == 0 as ::core::ffi::c_int {
            continue;
        }
        if len < 0 as ::core::ffi::c_int {
            pdf_warning!(
                b"Reading a line failed in xref table.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        p = &raw mut buf as *mut ::core::ffi::c_char;
        endptr = (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize);
        skip_white(&raw mut p, endptr);
        if p == endptr {
            continue;
        }
        if !strstartswith(p, b"trailer\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            current_pos = current_pos
                .wrapping_add(p.offset_from(&raw mut buf as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long as size_t);
            crate::backend::runtime::input_seek((*pf).handle, current_pos as ssize_t, SEEK_SET);
            break;
        } else {
            let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            q = parse_unsigned(&raw mut p, endptr);
            if q.is_null() {
                pdf_warning!(
                    b"An unsigned integer expected but could not find. (xref)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            first = atoi(q) as uint32_t;
            free(q as *mut ::core::ffi::c_void);
            skip_white(&raw mut p, endptr);
            q = parse_unsigned(&raw mut p, endptr);
            if q.is_null() {
                pdf_warning!(
                    b"An unsigned integer expected but could not find. (xref)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            size = atoi(q) as size_t;
            free(q as *mut ::core::ffi::c_void);
            skip_white(&raw mut p, endptr);
            if p != endptr {
                pdf_warning!(
                    b"Unexpected token found in xref table.\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            if ((*pf).num_obj as size_t) < (first as size_t).wrapping_add(size) {
                extend_xref(pf, (first as size_t).wrapping_add(size));
            }
            i = first as ::core::ffi::c_int;
            while (i as size_t) < (first as size_t).wrapping_add(size) {
                len = tt_mfreadln(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    255 as ::core::ffi::c_int,
                    (*pf).handle,
                );
                if len == 0 as ::core::ffi::c_int {
                    continue;
                }
                if len < 0 as ::core::ffi::c_int {
                    pdf_warning!(
                        b"Something went wrong while reading xref subsection...\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                p = &raw mut buf as *mut ::core::ffi::c_char;
                endptr = (&raw mut buf as *mut ::core::ffi::c_char).offset(len as isize);
                skip_white(&raw mut p, endptr);
                if p == endptr {
                    continue;
                }
                offset = 0 as ::core::ffi::c_ulong as size_t;
                obj_gen = 0 as uint32_t;
                flag = 0 as ::core::ffi::c_char;
                let mut q_0: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                q_0 = parse_unsigned(&raw mut p, endptr);
                if q_0.is_null() {
                    pdf_warning!(
                        b"An unsigned integer expected but could not find. (xref)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                } else if strlen(q_0) != 10 as size_t {
                    pdf_warning!(
                        b"Offset must be a 10 digits number. (xref)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    free(q_0 as *mut ::core::ffi::c_void);
                    return -(1 as ::core::ffi::c_int);
                }
                offset = atoi(q_0) as size_t;
                free(q_0 as *mut ::core::ffi::c_void);
                skip_white(&raw mut p, endptr);
                q_0 = parse_unsigned(&raw mut p, endptr);
                if q_0.is_null() {
                    pdf_warning!(
                        b"An unsigned integer expected but could not find. (xref)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                } else if strlen(q_0) != 5 as size_t {
                    pdf_warning!(
                        b"Expecting a 5 digits number. (xref)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    free(q_0 as *mut ::core::ffi::c_void);
                    return -(1 as ::core::ffi::c_int);
                }
                obj_gen = atoi(q_0) as uint32_t;
                free(q_0 as *mut ::core::ffi::c_void);
                skip_white(&raw mut p, endptr);
                if p == endptr {
                    pdf_warning!(
                        b"Unexpected EOL reached while reading a xref subsection entry.\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                flag = *p;
                p = p.offset(1);
                skip_white(&raw mut p, endptr);
                if p < endptr {
                    pdf_warning!(
                        b"Garbage in xref subsection entry found...\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                } else if flag as ::core::ffi::c_int != 'n' as i32
                    && flag as ::core::ffi::c_int != 'f' as i32
                    || flag as ::core::ffi::c_int == 'n' as i32
                        && (offset >= (*pf).file_size as size_t
                            || offset > 0 as size_t && offset < 4 as size_t)
                {
                    pdf_warning!(
                        b"Invalid xref table entry [%u]. PDF file is corrupt...\0" as *const u8
                            as *const ::core::ffi::c_char,
                        i,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                if (*(*pf).xref_table.offset(i as isize)).field2 == 0 {
                    (*(*pf).xref_table.offset(i as isize)).type_0 =
                        (flag as ::core::ffi::c_int == 'n' as i32) as ::core::ffi::c_int
                            as ::core::ffi::c_uchar;
                    (*(*pf).xref_table.offset(i as isize)).field2 = offset as ::core::ffi::c_uint;
                    (*(*pf).xref_table.offset(i as isize)).field3 =
                        obj_gen as ::core::ffi::c_ushort;
                }
                i += 1;
            }
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn parse_xrefstm_field(
    mut p: *mut *const ::core::ffi::c_char,
    mut length: size_t,
    mut def: uint32_t,
) -> uint32_t {
    let mut val: uint32_t = 0 as uint32_t;
    if length == 0 {
        return def;
    }
    loop {
        let fresh36 = length;
        length = length.wrapping_sub(1);
        if !(fresh36 != 0) {
            break;
        }
        val <<= 8 as ::core::ffi::c_int;
        let fresh37 = *p;
        *p = (*p).offset(1);
        val |= *fresh37 as ::core::ffi::c_uchar as uint32_t;
    }
    return val;
}
unsafe extern "C-unwind" fn parse_xrefstm_subsec(
    mut pf: *mut pdf_file,
    mut p: *mut *const ::core::ffi::c_char,
    mut length: *mut size_t,
    mut W: *mut ::core::ffi::c_int,
    mut wsum: ::core::ffi::c_int,
    mut first: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut e: *mut xref_entry = ::core::ptr::null_mut::<xref_entry>();
    *length = (*length).wrapping_sub((wsum * size) as size_t);
    if (*pf).num_obj < first + size {
        extend_xref(pf, (first + size) as size_t);
    }
    e = (*pf).xref_table.offset(first as isize);
    loop {
        let fresh35 = size;
        size = size - 1;
        if !(fresh35 != 0) {
            break;
        }
        let mut type_0: uint8_t = 0;
        let mut field2: uint32_t = 0;
        let mut field3: uint16_t = 0;
        type_0 = parse_xrefstm_field(
            p,
            *W.offset(0 as ::core::ffi::c_int as isize) as size_t,
            1 as uint32_t,
        ) as ::core::ffi::c_uchar as uint8_t;
        if type_0 as ::core::ffi::c_int > 2 as ::core::ffi::c_int {
            pdf_warning!(
                b"Unknown cross-reference stream entry type.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        field2 = parse_xrefstm_field(
            p,
            *W.offset(1 as ::core::ffi::c_int as isize) as size_t,
            0 as uint32_t,
        );
        field3 = parse_xrefstm_field(
            p,
            *W.offset(2 as ::core::ffi::c_int as isize) as size_t,
            0 as uint32_t,
        ) as uint16_t;
        if (*e).field2 == 0 {
            (*e).type_0 = type_0 as ::core::ffi::c_uchar;
            (*e).field2 = field2 as ::core::ffi::c_uint;
            (*e).field3 = field3 as ::core::ffi::c_ushort;
        }
        e = e.offset(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn parse_xref_stream(
    mut pf: *mut pdf_file,
    mut xref_pos: size_t,
    mut trailer: *mut *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut xrefstm: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut size_obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut W_obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut index_obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut size: uint32_t = 0;
    let mut length: size_t = 0;
    let mut W: [::core::ffi::c_int; 3] = [0; 3];
    let mut i: ::core::ffi::c_int = 0;
    let mut wsum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    xrefstm = pdf_read_object(
        0 as uint32_t,
        0 as uint16_t,
        pf,
        xref_pos,
        (*pf).file_size as size_t,
    );
    if !xrefstm.is_null() && pdf_obj_typeof(xrefstm) == PDF_STREAM {
        let mut tmp: *mut pdf_obj = pdf_stream_uncompress(xrefstm);
        if !tmp.is_null() {
            pdf_release_obj(xrefstm);
            xrefstm = tmp;
            *trailer = pdf_link_obj(pdf_stream_dict(xrefstm));
            size_obj = pdf_lookup_dict(
                *trailer,
                b"Size\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !size_obj.is_null() && pdf_obj_typeof(size_obj) == PDF_NUMBER {
                size = pdf_number_value(size_obj) as uint32_t;
                length = pdf_stream_length(xrefstm) as size_t;
                W_obj =
                    pdf_lookup_dict(*trailer, b"W\0" as *const u8 as *const ::core::ffi::c_char);
                if !(!(!W_obj.is_null() && pdf_obj_typeof(W_obj) == PDF_ARRAY)
                    || pdf_array_length(W_obj) != 3 as ::core::ffi::c_uint)
                {
                    i = 0 as ::core::ffi::c_int;
                    loop {
                        if !(i < 3 as ::core::ffi::c_int) {
                            current_block = 8831408221741692167;
                            break;
                        }
                        let mut tmp_0: *mut pdf_obj = pdf_get_array(W_obj, i);
                        if !(!tmp_0.is_null() && pdf_obj_typeof(tmp_0) == PDF_NUMBER) {
                            current_block = 1409276481103137046;
                            break;
                        }
                        W[i as usize] = pdf_number_value(tmp_0) as ::core::ffi::c_int;
                        wsum += W[i as usize];
                        i += 1;
                    }
                    match current_block {
                        1409276481103137046 => {}
                        _ => {
                            p = pdf_stream_dataptr(xrefstm) as *const ::core::ffi::c_char;
                            index_obj = pdf_lookup_dict(
                                *trailer,
                                b"Index\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            if !index_obj.is_null() {
                                let mut index_len: size_t = 0;
                                if !(!index_obj.is_null() && pdf_obj_typeof(index_obj) == PDF_ARRAY)
                                    || {
                                        index_len = pdf_array_length(index_obj) as size_t;
                                        index_len.wrapping_rem(2 as size_t) != 0
                                    }
                                {
                                    current_block = 1409276481103137046;
                                } else {
                                    i = 0 as ::core::ffi::c_int;
                                    loop {
                                        if !((i as size_t) < index_len) {
                                            current_block = 7172762164747879670;
                                            break;
                                        }
                                        let fresh33 = i;
                                        i = i + 1;
                                        let mut first: *mut pdf_obj =
                                            pdf_get_array(index_obj, fresh33);
                                        let fresh34 = i;
                                        i = i + 1;
                                        size_obj = pdf_get_array(index_obj, fresh34);
                                        if !(!first.is_null()
                                            && pdf_obj_typeof(first) == PDF_NUMBER)
                                            || !(!size_obj.is_null()
                                                && pdf_obj_typeof(size_obj) == PDF_NUMBER)
                                            || parse_xrefstm_subsec(
                                                pf,
                                                &raw mut p,
                                                &raw mut length,
                                                &raw mut W as *mut ::core::ffi::c_int,
                                                wsum,
                                                pdf_number_value(first) as ::core::ffi::c_int,
                                                pdf_number_value(size_obj) as ::core::ffi::c_int,
                                            ) != 0
                                        {
                                            current_block = 1409276481103137046;
                                            break;
                                        }
                                    }
                                }
                            } else if parse_xrefstm_subsec(
                                pf,
                                &raw mut p,
                                &raw mut length,
                                &raw mut W as *mut ::core::ffi::c_int,
                                wsum,
                                0 as ::core::ffi::c_int,
                                size as ::core::ffi::c_int,
                            ) != 0
                            {
                                current_block = 1409276481103137046;
                            } else {
                                current_block = 7172762164747879670;
                            }
                            match current_block {
                                1409276481103137046 => {}
                                _ => {
                                    if length != 0 {
                                        pdf_warning!(
                                            b"Garbage in xref stream.\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                    pdf_release_obj(xrefstm);
                                    return 1 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pdf_warning!(
        b"Cannot parse cross-reference stream.\0" as *const u8 as *const ::core::ffi::c_char,
    );
    pdf_release_obj(xrefstm);
    if !(*trailer).is_null() {
        pdf_release_obj(*trailer);
        *trailer = ::core::ptr::null_mut::<pdf_obj>();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn read_xref(mut pf: *mut pdf_file) -> *mut pdf_obj {
    let mut current_block: u64;
    let mut trailer: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut main_trailer: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut xref_pos: size_t = 0;
    xref_pos = find_xref((*pf).handle, (*pf).file_size) as size_t;
    if xref_pos == 0 {
        current_block = 4828155016069157118;
    } else {
        current_block = 6239978542346980191;
    }
    loop {
        match current_block {
            6239978542346980191 => {
                if xref_pos != 0 {
                    let mut prev: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                    let mut res: ::core::ffi::c_int = parse_xref_table(pf, xref_pos);
                    if res > 0 as ::core::ffi::c_int {
                        let mut xrefstm: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                        trailer = parse_trailer(pf);
                        if trailer.is_null() {
                            current_block = 4828155016069157118;
                            continue;
                        }
                        if main_trailer.is_null() {
                            main_trailer = pdf_link_obj(trailer);
                        }
                        xrefstm = pdf_lookup_dict(
                            trailer,
                            b"XRefStm\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        if !xrefstm.is_null() {
                            let mut new_trailer: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                            if !xrefstm.is_null()
                                && pdf_obj_typeof(xrefstm) == PDF_NUMBER
                                && parse_xref_stream(
                                    pf,
                                    pdf_number_value(xrefstm) as ::core::ffi::c_int as size_t,
                                    &raw mut new_trailer,
                                ) != 0
                            {
                                pdf_release_obj(new_trailer);
                            } else {
                                pdf_warning!(
                                    b"Skipping hybrid reference section.\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                            }
                        }
                    } else {
                        if !(res == 0 && parse_xref_stream(pf, xref_pos, &raw mut trailer) != 0) {
                            current_block = 4828155016069157118;
                            continue;
                        }
                        if main_trailer.is_null() {
                            main_trailer = pdf_link_obj(trailer);
                        }
                    }
                    prev = pdf_lookup_dict(
                        trailer,
                        b"Prev\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !prev.is_null() {
                        if !(!prev.is_null() && pdf_obj_typeof(prev) == PDF_NUMBER) {
                            current_block = 4828155016069157118;
                            continue;
                        }
                        xref_pos = pdf_number_value(prev) as size_t;
                    } else {
                        xref_pos = 0 as size_t;
                    }
                    pdf_release_obj(trailer);
                    current_block = 6239978542346980191;
                } else {
                    return main_trailer;
                }
            }
            _ => {
                pdf_warning!(
                    b"Error while parsing PDF file.\0" as *const u8 as *const ::core::ffi::c_char,
                );
                pdf_release_obj(trailer);
                pdf_release_obj(main_trailer);
                return ::core::ptr::null_mut::<pdf_obj>();
            }
        }
    }
}
static mut pdf_files: *mut ht_table = ::core::ptr::null::<ht_table>() as *mut ht_table;
unsafe extern "C-unwind" fn pdf_file_new(mut handle: rust_input_handle_t) -> *mut pdf_file {
    let mut pf: *mut pdf_file = ::core::ptr::null_mut::<pdf_file>();
    if (handle == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"handle\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    pf = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<pdf_file>() as usize) as uint32_t)
        as *mut pdf_file;
    (*pf).handle = handle;
    (*pf).trailer = ::core::ptr::null_mut::<pdf_obj>();
    (*pf).xref_table = ::core::ptr::null_mut::<xref_entry>();
    (*pf).catalog = ::core::ptr::null_mut::<pdf_obj>();
    (*pf).num_obj = 0 as ::core::ffi::c_int;
    (*pf).version = 0 as ::core::ffi::c_uint;
    (*pf).file_size = ttstub_input_get_size(handle) as ::core::ffi::c_int;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_END);
    return pf;
}
unsafe extern "C-unwind" fn pdf_file_free(mut pf: *mut pdf_file) {
    let mut i: size_t = 0;
    if pf.is_null() {
        return;
    }
    i = 0 as size_t;
    while i < (*pf).num_obj as size_t {
        pdf_release_obj((*(*pf).xref_table.offset(i as isize)).direct);
        pdf_release_obj((*(*pf).xref_table.offset(i as isize)).indirect);
        i = i.wrapping_add(1);
    }
    free((*pf).xref_table as *mut ::core::ffi::c_void);
    pdf_release_obj((*pf).trailer);
    pdf_release_obj((*pf).catalog);
    free(pf as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_pdf_files_init")]
pub unsafe extern "C-unwind" fn pdf_files_init() {
    pdf_files = new((1 as ::core::ffi::c_int as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<ht_table>() as usize) as uint32_t)
        as *mut ht_table;
    ht_init_table(
        pdf_files,
        ::core::mem::transmute::<
            Option<unsafe extern "C-unwind" fn(*mut pdf_file) -> ()>,
            hval_free_func,
        >(Some(
            pdf_file_free as unsafe extern "C-unwind" fn(*mut pdf_file) -> (),
        )),
    );
}
#[unsafe(export_name = "__pdf_impl_pdf_file_get_version")]
pub unsafe extern "C-unwind" fn pdf_file_get_version(mut pf: *mut pdf_file) -> ::core::ffi::c_int {
    if pf.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"pf\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*pf).version as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_file_get_trailer")]
pub unsafe extern "C-unwind" fn pdf_file_get_trailer(mut pf: *mut pdf_file) -> *mut pdf_obj {
    if pf.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"pf\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return pdf_link_obj((*pf).trailer);
}
#[unsafe(export_name = "__pdf_impl_pdf_file_get_catalog")]
pub unsafe extern "C-unwind" fn pdf_file_get_catalog(mut pf: *mut pdf_file) -> *mut pdf_obj {
    if pf.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"pf\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    return (*pf).catalog;
}
#[unsafe(export_name = "__pdf_impl_pdf_open")]
pub unsafe extern "C-unwind" fn pdf_open(
    mut ident: *const ::core::ffi::c_char,
    mut handle: rust_input_handle_t,
) -> *mut pdf_file {
    let mut current_block: u64;
    let mut p: *mut pdf_out = current_output();
    let mut pf: *mut pdf_file = ::core::ptr::null_mut::<pdf_file>();
    if pdf_files.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"pdf_files\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !ident.is_null() {
        pf = ht_lookup_table(
            pdf_files,
            ident as *const ::core::ffi::c_void,
            strlen(ident) as ::core::ffi::c_int,
        ) as *mut pdf_file;
    }
    if !pf.is_null() {
        (*pf).handle = handle;
    } else {
        let mut new_version: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut version: ::core::ffi::c_int = check_for_pdf_version(handle);
        if dpx_conf.is_xbb == 0 {
            let mut ver_major: ::core::ffi::c_int = 0;
            let mut ver_minor: ::core::ffi::c_int = 0;
            ver_major = version / 10 as ::core::ffi::c_int;
            ver_minor = version % 10 as ::core::ffi::c_int;
            if version < 10 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Unrecognized PDF version specified for input PDF file: %d.%d\0" as *const u8
                        as *const ::core::ffi::c_char,
                    ver_major,
                    ver_minor,
                );
            } else if pdf_check_version(ver_major, ver_minor) < 0 as ::core::ffi::c_int {
                pdf_warning!(
                    b"Trying to include PDF file with version (%d.%d), which is newer than current output PDF setting (%d.%d).\0"
                    as * const u8 as * const ::core::ffi::c_char, ver_major, ver_minor,
                    (* p).version.major, (* p).version.minor,
                );
            }
        }
        pf = pdf_file_new(handle);
        (*pf).version = version as ::core::ffi::c_uint;
        (*pf).trailer = read_xref(pf);
        if (*pf).trailer.is_null() {
            current_block = 3573971190450902173;
        } else if !pdf_lookup_dict(
            (*pf).trailer,
            b"Encrypt\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            pdf_warning!(
                b"PDF document is encrypted.\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block = 3573971190450902173;
        } else {
            (*pf).catalog = pdf_deref_obj(pdf_lookup_dict(
                (*pf).trailer,
                b"Root\0" as *const u8 as *const ::core::ffi::c_char,
            ));
            if !(!(*pf).catalog.is_null() && pdf_obj_typeof((*pf).catalog) == PDF_DICT) {
                pdf_warning!(
                    b"Cannot read PDF document catalog. Broken PDF file?\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                current_block = 3573971190450902173;
            } else {
                new_version = pdf_deref_obj(pdf_lookup_dict(
                    (*pf).catalog,
                    b"Version\0" as *const u8 as *const ::core::ffi::c_char,
                ));
                if !new_version.is_null() {
                    let mut major: ::core::ffi::c_uint = 0;
                    let mut minor: ::core::ffi::c_uint = 0;
                    if !(!new_version.is_null() && pdf_obj_typeof(new_version) == PDF_NAME)
                        || sscanf(
                            pdf_name_value(new_version),
                            b"%u.%u\0" as *const u8 as *const ::core::ffi::c_char,
                            &raw mut major,
                            &raw mut minor,
                        ) != 2 as ::core::ffi::c_int
                    {
                        pdf_release_obj(new_version);
                        pdf_warning!(
                            b"Illegal Version entry in document catalog. Broken PDF file?\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        current_block = 3573971190450902173;
                    } else {
                        if (*pf).version
                            < major
                                .wrapping_mul(10 as ::core::ffi::c_uint)
                                .wrapping_add(minor)
                        {
                            (*pf).version = major
                                .wrapping_mul(10 as ::core::ffi::c_uint)
                                .wrapping_add(minor);
                        }
                        pdf_release_obj(new_version);
                        current_block = 14763689060501151050;
                    }
                } else {
                    current_block = 14763689060501151050;
                }
                match current_block {
                    3573971190450902173 => {}
                    _ => {
                        if !ident.is_null() {
                            ht_append_table(
                                pdf_files,
                                ident as *const ::core::ffi::c_void,
                                strlen(ident) as ::core::ffi::c_int,
                                pf as *mut ::core::ffi::c_void,
                            );
                        }
                        current_block = 5689316957504528238;
                    }
                }
            }
        }
        match current_block {
            5689316957504528238 => {}
            _ => {
                pdf_file_free(pf);
                return ::core::ptr::null_mut::<pdf_file>();
            }
        }
    }
    return pf;
}
#[unsafe(export_name = "__pdf_impl_pdf_close")]
pub unsafe extern "C-unwind" fn pdf_close(mut pf: *mut pdf_file) {
    if !pf.is_null() {
        (*pf).handle = INVALID_HANDLE as rust_input_handle_t;
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_files_close")]
pub unsafe extern "C-unwind" fn pdf_files_close() {
    if pdf_files.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"pdf_files\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    ht_clear_table(pdf_files);
    free(pdf_files as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn check_for_pdf_version(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut buffer: [::core::ffi::c_char; 10] =
        ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"\0\0\0\0\0\0\0\0\0\0");
    let mut major: ::core::ffi::c_uint = 0;
    let mut minor: ::core::ffi::c_uint = 0;
    crate::backend::runtime::input_seek(handle, 0 as ssize_t, SEEK_SET);
    if ttstub_input_read(
        handle,
        &raw mut buffer as *mut ::core::ffi::c_char,
        (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as size_t).wrapping_sub(1 as size_t),
    ) as usize
        != (::core::mem::size_of::<[::core::ffi::c_char; 10]>() as usize).wrapping_sub(1 as usize)
    {
        return -(1 as ::core::ffi::c_int);
    }
    if sscanf(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b"%%PDF-%u.%u\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut major,
        &raw mut minor,
    ) != 2 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    return major
        .wrapping_mul(10 as ::core::ffi::c_uint)
        .wrapping_add(minor) as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_check_for_pdf")]
pub unsafe extern "C-unwind" fn check_for_pdf(
    mut handle: rust_input_handle_t,
) -> ::core::ffi::c_int {
    let mut version: ::core::ffi::c_int = 0;
    version = check_for_pdf_version(handle);
    if version < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if version <= pdf_get_version() {
        return 1 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C-unwind" fn import_dict(
    mut key: *mut pdf_obj,
    mut value: *mut pdf_obj,
    mut pdata: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut copy: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    copy = pdata as *mut pdf_obj;
    tmp = pdf_import_object(value);
    if tmp.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    pdf_add_dict(copy, pdf_link_obj(key), tmp);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn pdf_import_indirect(mut object: *mut pdf_obj) -> *mut pdf_obj {
    let mut p: *mut pdf_out = current_output();
    let mut pf: *mut pdf_file = (*((*object).data as *mut pdf_indirect)).pf;
    let mut obj_num: uint32_t = (*((*object).data as *mut pdf_indirect)).label;
    let mut obj_gen: uint16_t = (*((*object).data as *mut pdf_indirect)).generation;
    let mut ref_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if pf.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"pf\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if !(obj_num > 0 as uint32_t
        && obj_num < (*pf).num_obj as uint32_t
        && ((*(*pf).xref_table.offset(obj_num as isize)).type_0 as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
            && (*(*pf).xref_table.offset(obj_num as isize)).field3 as ::core::ffi::c_int
                == obj_gen as ::core::ffi::c_int
            || (*(*pf).xref_table.offset(obj_num as isize)).type_0 as ::core::ffi::c_int
                == 2 as ::core::ffi::c_int
                && obj_gen == 0))
    {
        pdf_warning!(
            b"Can't resolve object: %u %u\0" as *const u8 as *const ::core::ffi::c_char,
            obj_num,
            obj_gen as ::core::ffi::c_int,
        );
        return pdf_new_null();
    }
    ref_0 = (*(*pf).xref_table.offset(obj_num as isize)).indirect;
    if ref_0.is_null() {
        let mut obj: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut reserved: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut imported: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        obj = pdf_get_object(pf, obj_num, obj_gen);
        if obj.is_null() {
            pdf_warning!(
                b"Could not read object: %u %u\0" as *const u8 as *const ::core::ffi::c_char,
                obj_num,
                obj_gen as ::core::ffi::c_int,
            );
            return ::core::ptr::null_mut::<pdf_obj>();
        }
        reserved = pdf_new_null();
        ref_0 = pdf_new_ref(p, reserved);
        let ref mut fresh41 = (*(*pf).xref_table.offset(obj_num as isize)).indirect;
        *fresh41 = ref_0;
        imported = pdf_import_object(obj);
        if !imported.is_null() {
            if (*imported).label != 0 {
                pdf_warning!(
                    b"Imported object already has a label: obj_id=%u\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*imported).label,
                );
            }
            let ref mut fresh42 = (*((*ref_0).data as *mut pdf_indirect)).obj;
            *fresh42 = imported;
            (*imported).label = (*reserved).label;
            (*imported).generation = (*reserved).generation;
            (*reserved).label = 0 as uint32_t;
            (*reserved).generation = 0 as uint16_t;
            pdf_release_obj(imported);
        }
        pdf_release_obj(reserved);
        pdf_release_obj(obj);
    }
    return pdf_link_obj(ref_0);
}
#[unsafe(export_name = "__pdf_impl_pdf_import_object")]
pub unsafe extern "C-unwind" fn pdf_import_object(mut object: *mut pdf_obj) -> *mut pdf_obj {
    let mut imported: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut tmp: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut i: size_t = 0;
    match pdf_obj_typeof(object) {
        PDF_INDIRECT => {
            if !(*((*object).data as *mut pdf_indirect)).pf.is_null() {
                imported = pdf_import_indirect(object);
            } else {
                imported = pdf_link_obj(object);
            }
        }
        PDF_STREAM => {
            let mut stream_dict: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            tmp = pdf_import_object(pdf_stream_dict(object));
            if tmp.is_null() {
                return ::core::ptr::null_mut::<pdf_obj>();
            }
            imported = pdf_new_stream(0 as ::core::ffi::c_int);
            stream_dict = pdf_stream_dict(imported);
            pdf_merge_dict(stream_dict, tmp);
            pdf_release_obj(tmp);
            pdf_add_stream(
                imported,
                pdf_stream_dataptr(object),
                pdf_stream_length(object),
            );
        }
        PDF_DICT => {
            imported = pdf_new_dict();
            if pdf_foreach_dict(
                object,
                Some(
                    import_dict
                        as unsafe extern "C-unwind" fn(
                            *mut pdf_obj,
                            *mut pdf_obj,
                            *mut ::core::ffi::c_void,
                        )
                            -> ::core::ffi::c_int,
                ),
                imported as *mut ::core::ffi::c_void,
            ) < 0 as ::core::ffi::c_int
            {
                pdf_release_obj(imported);
                return ::core::ptr::null_mut::<pdf_obj>();
            }
        }
        PDF_ARRAY => {
            imported = pdf_new_array();
            i = 0 as size_t;
            while i < pdf_array_length(object) as size_t {
                tmp = pdf_import_object(pdf_get_array(object, i as ::core::ffi::c_int));
                if tmp.is_null() {
                    pdf_release_obj(imported);
                    return ::core::ptr::null_mut::<pdf_obj>();
                }
                pdf_add_array(imported, tmp);
                i = i.wrapping_add(1);
            }
        }
        _ => {
            imported = pdf_link_obj(object);
        }
    }
    return imported;
}
#[unsafe(export_name = "__pdf_impl_pdf_compare_reference")]
pub unsafe extern "C-unwind" fn pdf_compare_reference(
    mut ref1: *mut pdf_obj,
    mut ref2: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut data1: *mut pdf_indirect = ::core::ptr::null_mut::<pdf_indirect>();
    let mut data2: *mut pdf_indirect = ::core::ptr::null_mut::<pdf_indirect>();
    if !(!ref1.is_null()
        && pdf_obj_typeof(ref1) == 9 as ::core::ffi::c_int
        && (!ref2.is_null() && pdf_obj_typeof(ref2) == 9 as ::core::ffi::c_int))
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"PDF_OBJ_INDIRECTTYPE(ref1) && PDF_OBJ_INDIRECTTYPE(ref2)\0" as *const u8
                as *const ::core::ffi::c_char
        );
    } else {
    };
    data1 = (*ref1).data as *mut pdf_indirect;
    data2 = (*ref2).data as *mut pdf_indirect;
    return ((*data1).pf != (*data2).pf
        || (*data1).label != (*data2).label
        || (*data1).generation as ::core::ffi::c_int != (*data2).generation as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_pdf_compare_object")]
pub unsafe extern "C-unwind" fn pdf_compare_object(
    mut obj1: *mut pdf_obj,
    mut obj2: *mut pdf_obj,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if obj1.is_null() && obj2.is_null() {
        return 0 as ::core::ffi::c_int;
    } else if obj1.is_null() && !obj2.is_null() {
        return 1 as ::core::ffi::c_int;
    } else if !obj1.is_null() && obj2.is_null() {
        return 1 as ::core::ffi::c_int;
    } else if pdf_obj_typeof(obj1) != pdf_obj_typeof(obj2) {
        return 1 as ::core::ffi::c_int;
    }
    match pdf_obj_typeof(obj1) {
        PDF_BOOLEAN => {
            r = pdf_boolean_value(obj1) as ::core::ffi::c_int
                - pdf_boolean_value(obj2) as ::core::ffi::c_int;
        }
        PDF_NUMBER => {
            if pdf_number_value(obj1) < pdf_number_value(obj2) {
                r = -(1 as ::core::ffi::c_int);
            } else if pdf_number_value(obj1) > pdf_number_value(obj2) {
                r = 1 as ::core::ffi::c_int;
            } else {
                r = 0 as ::core::ffi::c_int;
            }
        }
        PDF_STRING => {
            if pdf_string_length(obj1) < pdf_string_length(obj2) {
                r = -(1 as ::core::ffi::c_int);
            } else if pdf_string_length(obj1) > pdf_string_length(obj2) {
                r = 1 as ::core::ffi::c_int;
            } else {
                r = memcmp(
                    pdf_string_value(obj1),
                    pdf_string_value(obj2),
                    pdf_string_length(obj1) as size_t,
                );
            }
        }
        PDF_NAME => {
            r = strcmp(pdf_name_value(obj1), pdf_name_value(obj2));
        }
        PDF_NULL => {
            r = 0 as ::core::ffi::c_int;
        }
        PDF_INDIRECT => {
            r = pdf_compare_reference(obj1, obj2);
        }
        PDF_ARRAY => {
            if pdf_array_length(obj1) < pdf_array_length(obj2) {
                r = -(1 as ::core::ffi::c_int);
            } else if pdf_array_length(obj1) > pdf_array_length(obj2) {
                r = 1 as ::core::ffi::c_int;
            } else {
                let mut i: ::core::ffi::c_int = 0;
                i = 0 as ::core::ffi::c_int;
                while r == 0 as ::core::ffi::c_int
                    && (i as ::core::ffi::c_uint) < pdf_array_length(obj1)
                {
                    let mut v1: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                    let mut v2: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                    v1 = pdf_get_array(obj1, i);
                    v2 = pdf_get_array(obj2, i);
                    r = pdf_compare_object(v1, v2);
                    i += 1;
                }
            }
        }
        PDF_DICT => {
            let mut keys1: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            let mut keys2: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
            keys1 = pdf_dict_keys(obj1);
            keys2 = pdf_dict_keys(obj2);
            r = pdf_compare_object(keys1, keys2);
            if r == 0 as ::core::ffi::c_int {
                let mut i_0: ::core::ffi::c_int = 0;
                i_0 = 0 as ::core::ffi::c_int;
                while r == 0 as ::core::ffi::c_int
                    && (i_0 as ::core::ffi::c_uint) < pdf_array_length(keys1)
                {
                    let mut key: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                    let mut v1_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                    let mut v2_0: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
                    key = pdf_get_array(keys1, i_0);
                    v1_0 = pdf_lookup_dict(obj1, pdf_name_value(key));
                    v2_0 = pdf_lookup_dict(obj2, pdf_name_value(key));
                    r = pdf_compare_object(v1_0, v2_0);
                    i_0 += 1;
                }
            }
            pdf_release_obj(keys1);
            pdf_release_obj(keys2);
        }
        PDF_STREAM => {
            r = pdf_compare_object(pdf_stream_dict(obj1), pdf_stream_dict(obj2));
            if r == 0 as ::core::ffi::c_int {
                let mut len1: size_t = 0;
                let mut len2: size_t = 0;
                len1 = pdf_stream_length(obj1) as size_t;
                len2 = pdf_stream_length(obj2) as size_t;
                if len1 < len2 {
                    r = -(1 as ::core::ffi::c_int);
                } else if len1 > len2 {
                    r = 1 as ::core::ffi::c_int;
                } else {
                    r = 0 as ::core::ffi::c_int;
                }
            }
        }
        _ => {
            r = 1 as ::core::ffi::c_int;
        }
    }
    return r;
}
#[unsafe(export_name = "__pdf_impl_pdf_obj_reset_global_state")]
pub unsafe extern "C-unwind" fn pdf_obj_reset_global_state() {
    let mut p: *mut pdf_out = current_output();
    pdf_error_cleanup();
    (*p).output.file_position = 0 as size_t;
    (*p).output.line_position = 0 as ::core::ffi::c_int;
    (*p).output.compression_saved = 0 as size_t;
    tectonic_pout_initialized = 0 as ::core::ffi::c_int;
}
pub const INT_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
unsafe extern "C" {
    fn free(_: *mut ::core::ffi::c_void);
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
    fn memmove(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_sec {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_get_output_file(
    _result: *mut rust_output_handle_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_output_file() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_version(_result: *mut ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_version() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_version_major(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_version_major() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_version_minor(
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_version_minor() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_check_version(
    mut major: ::core::ffi::c_int,
    mut minor: ::core::ffi::c_int,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_check_version(major, minor) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_out_init(
    mut filename: *const ::core::ffi::c_char,
    mut id1: *const ::core::ffi::c_uchar,
    mut id2: *const ::core::ffi::c_uchar,
    mut ver_major: ::core::ffi::c_int,
    mut ver_minor: ::core::ffi::c_int,
    mut compression_level: ::core::ffi::c_int,
    mut enable_encrypt: ::core::ffi::c_int,
    mut enable_objstm: ::core::ffi::c_int,
    mut enable_predictor: ::core::ffi::c_int,
    _result: *mut *mut pdf_out,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_out_init(
            filename,
            id1,
            id2,
            ver_major,
            ver_minor,
            compression_level,
            enable_encrypt,
            enable_objstm,
            enable_predictor,
        )
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
pub unsafe extern "C" fn __pdf_entry_pdf_out_set_encrypt(
    mut keybits: ::core::ffi::c_int,
    mut permission: int32_t,
    mut opasswd: *const ::core::ffi::c_char,
    mut upasswd: *const ::core::ffi::c_char,
    mut use_aes: ::core::ffi::c_int,
    mut encrypt_metadata: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_out_set_encrypt(
            keybits,
            permission,
            opasswd,
            upasswd,
            use_aes,
            encrypt_metadata,
        )
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_out_flush() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_out_flush() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_error_cleanup() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_error_cleanup() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_set_root(mut object: *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_set_root(object) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_set_info(mut object: *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_set_info(object) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_obj_typeof(
    mut object: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_obj_typeof(object) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_transfer_label(
    mut dst: *mut pdf_obj,
    mut src: *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_transfer_label(dst, src) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_link_obj(
    mut object: *mut pdf_obj,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_link_obj(object) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_ref_obj(
    mut object: *mut pdf_obj,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_ref_obj(object) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_new_undefined(_result: *mut *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_new_undefined() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_new_null(_result: *mut *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_new_null() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_new_boolean(
    mut value: ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_new_boolean(value) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_boolean_value(
    mut object: *mut pdf_obj,
    _result: *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_boolean_value(object) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_new_number(
    mut value: ::core::ffi::c_double,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_new_number(value) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_set_number(
    mut object: *mut pdf_obj,
    mut value: ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_set_number(object, value) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_number_value(
    mut object: *mut pdf_obj,
    _result: *mut ::core::ffi::c_double,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_number_value(object) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_new_string(
    mut str: *const ::core::ffi::c_void,
    mut length: size_t,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_new_string(str, length) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_string_value(
    mut object: *mut pdf_obj,
    _result: *mut *mut ::core::ffi::c_void,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_string_value(object) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_string_length(
    mut object: *mut pdf_obj,
    _result: *mut ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_string_length(object) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdfobj_escape_str(
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsize: size_t,
    mut s: *const ::core::ffi::c_uchar,
    mut len: size_t,
    _result: *mut size_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdfobj_escape_str(buffer, bufsize, s, len)
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
pub unsafe extern "C" fn __pdf_entry_pdf_set_string(
    mut object: *mut pdf_obj,
    mut str: *mut ::core::ffi::c_uchar,
    mut length: size_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_set_string(object, str, length) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_new_name(
    mut name: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_new_name(name) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_name_value(
    mut object: *mut pdf_obj,
    _result: *mut *mut ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_name_value(object) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_new_array(_result: *mut *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_new_array() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_get_array(
    mut array: *mut pdf_obj,
    mut idx: ::core::ffi::c_int,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_get_array(array, idx) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_array_length(
    mut array: *mut pdf_obj,
    _result: *mut ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_array_length(array) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_add_array(
    mut array: *mut pdf_obj,
    mut object: *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_add_array(array, object) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_new_dict(_result: *mut *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_new_dict() }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_add_dict(
    mut dict: *mut pdf_obj,
    mut key: *mut pdf_obj,
    mut value: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_add_dict(dict, key, value) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_merge_dict(
    mut dict1: *mut pdf_obj,
    mut dict2: *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_merge_dict(dict1, dict2) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_foreach_dict(
    mut dict: *mut pdf_obj,
    mut proc_0: Option<
        unsafe extern "C-unwind" fn(
            *mut pdf_obj,
            *mut pdf_obj,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut pdata: *mut ::core::ffi::c_void,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_foreach_dict(dict, proc_0, pdata) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_lookup_dict(
    mut dict: *mut pdf_obj,
    mut name: *const ::core::ffi::c_char,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_lookup_dict(dict, name) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_dict_keys(
    mut dict: *mut pdf_obj,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_dict_keys(dict) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_remove_dict(
    mut dict: *mut pdf_obj,
    mut name: *const ::core::ffi::c_char,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_remove_dict(dict, name) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_new_stream(
    mut flags: ::core::ffi::c_int,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_new_stream(flags) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_stream_set_predictor(
    mut stream: *mut pdf_obj,
    mut predictor: ::core::ffi::c_int,
    mut columns: int32_t,
    mut bpc: ::core::ffi::c_int,
    mut colors: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_stream_set_predictor(stream, predictor, columns, bpc, colors)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_tt_pdf_filter_png_rows(
    mut raster: *const ::core::ffi::c_uchar,
    mut dst: *mut ::core::ffi::c_uchar,
    mut rowbytes: int32_t,
    mut bytes_per_pixel: ::core::ffi::c_int,
    mut first_row: int32_t,
    mut last_row: int32_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        tt_pdf_filter_png_rows(raster, dst, rowbytes, bytes_per_pixel, first_row, last_row)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_stream_dict(
    mut stream: *mut pdf_obj,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_stream_dict(stream) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_stream_dataptr(
    mut stream: *mut pdf_obj,
    _result: *mut *const ::core::ffi::c_void,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_stream_dataptr(stream) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_stream_length(
    mut stream: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_stream_length(stream) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_add_stream(
    mut stream: *mut pdf_obj,
    mut stream_data: *const ::core::ffi::c_void,
    mut length: ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        pdf_add_stream(stream, stream_data, length)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_concat_stream(
    mut dst: *mut pdf_obj,
    mut src: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_concat_stream(dst, src) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_pipeline_drain(mut all: ::core::ffi::c_int) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_pipeline_drain(all) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_submit_page(
    mut contents: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_submit_page(contents) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_release_obj(mut object: *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_release_obj(object) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_new_indirect(
    mut pf: *mut pdf_file,
    mut obj_num: uint32_t,
    mut obj_gen: uint16_t,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_new_indirect(pf, obj_num, obj_gen) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_deref_obj(
    mut obj: *mut pdf_obj,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_deref_obj(obj) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_files_init() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_files_init() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_file_get_version(
    mut pf: *mut pdf_file,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_file_get_version(pf) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_file_get_trailer(
    mut pf: *mut pdf_file,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_file_get_trailer(pf) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_file_get_catalog(
    mut pf: *mut pdf_file,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_file_get_catalog(pf) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_open(
    mut ident: *const ::core::ffi::c_char,
    mut handle: rust_input_handle_t,
    _result: *mut *mut pdf_file,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_open(ident, handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_close(mut pf: *mut pdf_file) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_close(pf) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_files_close() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_files_close() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_check_for_pdf(
    mut handle: rust_input_handle_t,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { check_for_pdf(handle) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_import_object(
    mut object: *mut pdf_obj,
    _result: *mut *mut pdf_obj,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_import_object(object) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_compare_reference(
    mut ref1: *mut pdf_obj,
    mut ref2: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_compare_reference(ref1, ref2) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_compare_object(
    mut obj1: *mut pdf_obj,
    mut obj2: *mut pdf_obj,
    _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_compare_object(obj1, obj2) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_obj_reset_global_state() -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_obj_reset_global_state() }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
