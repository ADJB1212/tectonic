/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

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
#![allow(warnings, clippy::all, unsafe_op_in_unsafe_fn, static_mut_refs, dangerous_implicit_autorefs)]
use crate::compat::allocation::free;
use crate::unicode::is_valid as UC_is_valid;
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn strcpy(__dst: *mut ::core::ffi::c_char, __src: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn time(_: *mut time_t) -> time_t;
    fn _tt_abort(format: *const ::core::ffi::c_char, ...) -> !;
    #[link_name = "__pdf_impl_MD5_init"]
    fn MD5_init(ctx: *mut MD5_CONTEXT);
    #[link_name = "__pdf_impl_MD5_write"]
    fn MD5_write(ctx: *mut MD5_CONTEXT, inbuf: *const ::core::ffi::c_uchar, inlen: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_MD5_final"]
    fn MD5_final(outbuf: *mut ::core::ffi::c_uchar, ctx: *mut MD5_CONTEXT);
    #[link_name = "__pdf_impl_SHA256_init"]
    fn SHA256_init(ctx: *mut SHA256_CONTEXT);
    #[link_name = "__pdf_impl_SHA256_write"]
    fn SHA256_write(ctx: *mut SHA256_CONTEXT, inbuf: *const ::core::ffi::c_uchar, inlen: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_SHA256_final"]
    fn SHA256_final(outbuf: *mut ::core::ffi::c_uchar, ctx: *mut SHA256_CONTEXT);
    #[link_name = "__pdf_impl_SHA384_init"]
    fn SHA384_init(ctx: *mut SHA512_CONTEXT);
    #[link_name = "__pdf_impl_SHA512_init"]
    fn SHA512_init(ctx: *mut SHA512_CONTEXT);
    #[link_name = "__pdf_impl_SHA512_write"]
    fn SHA512_write(ctx: *mut SHA512_CONTEXT, inbuf: *const ::core::ffi::c_uchar, inlen: ::core::ffi::c_uint);
    #[link_name = "__pdf_impl_SHA512_final"]
    fn SHA512_final(outbuf: *mut ::core::ffi::c_uchar, ctx: *mut SHA512_CONTEXT);
    #[link_name = "__pdf_impl_ARC4"]
    fn ARC4(ctx: *mut ARC4_CONTEXT, len: ::core::ffi::c_uint, inbuf: *const ::core::ffi::c_uchar, outbuf: *mut ::core::ffi::c_uchar);
    #[link_name = "__pdf_impl_ARC4_set_key"]
    fn ARC4_set_key(ctx: *mut ARC4_CONTEXT, keylen: ::core::ffi::c_uint, key: *const ::core::ffi::c_uchar);
    #[link_name = "__pdf_impl_AES_ecb_encrypt"]
    fn AES_ecb_encrypt(
        key: *const ::core::ffi::c_uchar, key_len: size_t, plain: *const ::core::ffi::c_uchar, plain_len: size_t,
        cipher: *mut *mut ::core::ffi::c_uchar, cipher_len: *mut size_t,
    );
    #[link_name = "__pdf_impl_AES_cbc_encrypt_tectonic"]
    fn AES_cbc_encrypt_tectonic(
        key: *const ::core::ffi::c_uchar, key_len: size_t, iv: *const ::core::ffi::c_uchar, padding: ::core::ffi::c_int,
        plain: *const ::core::ffi::c_uchar, plain_len: size_t, cipher: *mut *mut ::core::ffi::c_uchar, cipher_len: *mut size_t,
    );
    #[link_name = "__pdf_impl_dpx_util_get_unique_time_if_given"]
    fn dpx_util_get_unique_time_if_given() -> time_t;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_init_genrand"]
    fn init_genrand(s: ::core::ffi::c_ulonglong);
    #[link_name = "__pdf_impl_genrand_int31"]
    fn genrand_int31() -> ::core::ffi::c_long;
    #[link_name = "__pdf_impl_pdf_get_version"]
    fn pdf_get_version() -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_pdf_new_number"]
    fn pdf_new_number(value: ::core::ffi::c_double) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_string"]
    fn pdf_new_string(str: *const ::core::ffi::c_void, length: size_t) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_name"]
    fn pdf_new_name(name: *const ::core::ffi::c_char) -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_new_dict"]
    fn pdf_new_dict() -> *mut pdf_obj;
    #[link_name = "__pdf_impl_pdf_add_dict"]
    fn pdf_add_dict(dict: *mut pdf_obj, key: *mut pdf_obj, value: *mut pdf_obj) -> ::core::ffi::c_int;
    #[link_name = "__pdf_impl_UC_UTF8_decode_char"]
    fn UC_UTF8_decode_char(pp: *mut *const ::core::ffi::c_uchar, endptr: *const ::core::ffi::c_uchar) -> int32_t;
}
pub type int32_t = i32;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type __darwin_size_t = usize;
pub type __darwin_time_t = ::core::ffi::c_long;
pub type size_t = __darwin_size_t;
pub type time_t = libc::time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_CONTEXT {
    pub A: uint32_t,
    pub B: uint32_t,
    pub C: uint32_t,
    pub D: uint32_t,
    pub nblocks: size_t,
    pub buf: [::core::ffi::c_uchar; 64],
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA256_CONTEXT {
    pub h0: uint32_t,
    pub h1: uint32_t,
    pub h2: uint32_t,
    pub h3: uint32_t,
    pub h4: uint32_t,
    pub h5: uint32_t,
    pub h6: uint32_t,
    pub h7: uint32_t,
    pub nblocks: size_t,
    pub buf: [::core::ffi::c_uchar; 64],
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA512_STATE {
    pub h0: uint64_t,
    pub h1: uint64_t,
    pub h2: uint64_t,
    pub h3: uint64_t,
    pub h4: uint64_t,
    pub h5: uint64_t,
    pub h6: uint64_t,
    pub h7: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA512_CONTEXT {
    pub state: SHA512_STATE,
    pub nblocks: size_t,
    pub buf: [::core::ffi::c_uchar; 128],
    pub count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARC4_CONTEXT {
    pub idx_i: ::core::ffi::c_int,
    pub idx_j: ::core::ffi::c_int,
    pub sbox: [::core::ffi::c_uchar; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_sec {
    pub key: [::core::ffi::c_uchar; 32],
    pub key_size: ::core::ffi::c_int,
    pub ID: [::core::ffi::c_uchar; 16],
    pub O: [::core::ffi::c_uchar; 48],
    pub U: [::core::ffi::c_uchar; 48],
    pub OE: [::core::ffi::c_uchar; 32],
    pub UE: [::core::ffi::c_uchar; 32],
    pub V: ::core::ffi::c_int,
    pub R: ::core::ffi::c_int,
    pub P: int32_t,
    pub setting: C2RustUnnamed_0,
    pub label: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub objnum: uint64_t,
    pub gennum: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub use_aes: ::core::ffi::c_int,
    pub encrypt_metadata: ::core::ffi::c_int,
    pub need_adobe_extension: ::core::ffi::c_int,
}
pub type Stringprep_profile_flags = ::core::ffi::c_int;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const AES_BLOCKSIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const INVALID_EPOCH_VALUE: time_t = -(1 as ::core::ffi::c_int) as time_t;
const PADDING_BYTES: [::core::ffi::c_uchar; 32] = [
    0x28 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xbf as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x5e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x75 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8a as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x41 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x4e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x56 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xff as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x8 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xb6 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xd0 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x68 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x3e as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x80 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x2f as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xc as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xa9 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0xfe as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x64 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x53 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x69 as ::core::ffi::c_int as ::core::ffi::c_uchar,
    0x7a as ::core::ffi::c_int as ::core::ffi::c_uchar,
];
#[unsafe(export_name = "__pdf_impl_pdf_enc_init")]
pub unsafe extern "C-unwind" fn pdf_enc_init(
    mut id: *const ::core::ffi::c_uchar, mut keybits: ::core::ffi::c_int, mut permission: uint32_t, mut opasswd: *const ::core::ffi::c_char,
    mut upasswd: *const ::core::ffi::c_char, mut use_aes: ::core::ffi::c_int, mut encrypt_metadata: ::core::ffi::c_int,
) -> *mut pdf_sec {
    let mut current_time: time_t = 0;
    let mut p_sec: *mut pdf_sec = ::core::ptr::null_mut::<pdf_sec>();
    p_sec = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<pdf_sec>() as usize) as uint32_t) as *mut pdf_sec;
    memset(
        p_sec as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<pdf_sec>() as size_t,
    );
    current_time = dpx_util_get_unique_time_if_given();
    if current_time == INVALID_EPOCH_VALUE {
        current_time = libc::time(::core::ptr::null_mut::<time_t>());
    }
    init_genrand(current_time as ::core::ffi::c_ulonglong);
    (*p_sec).setting.use_aes = use_aes;
    (*p_sec).setting.encrypt_metadata = encrypt_metadata;
    (*p_sec).setting.need_adobe_extension = 0 as ::core::ffi::c_int;
    memcpy(
        &raw mut (*p_sec).ID as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        id as *const ::core::ffi::c_void,
        16 as size_t,
    );
    pdf_enc_set_password(p_sec, keybits as ::core::ffi::c_uint, permission as ::core::ffi::c_uint, opasswd, upasswd);
    return p_sec;
}
#[unsafe(export_name = "__pdf_impl_pdf_enc_close")]
pub unsafe extern "C-unwind" fn pdf_enc_close(mut p_sec: *mut *mut pdf_sec) {
    if !p_sec.is_null() && !(*p_sec).is_null() {
        free(*p_sec as *mut ::core::ffi::c_void);
        *p_sec = ::core::ptr::null_mut::<pdf_sec>();
    }
}
unsafe extern "C-unwind" fn passwd_padding(mut src: *const ::core::ffi::c_char, mut dst: *mut ::core::ffi::c_uchar) {
    let mut len: ::core::ffi::c_int = 0;
    len = (if (32 as size_t) < strlen(src) { 32 as size_t } else { strlen(src) }) as ::core::ffi::c_int;
    memcpy(dst as *mut ::core::ffi::c_void, src as *const ::core::ffi::c_void, len as size_t);
    memcpy(
        dst.offset(len as isize) as *mut ::core::ffi::c_void,
        PADDING_BYTES.as_ptr() as *const ::core::ffi::c_void,
        (32 as ::core::ffi::c_int - len) as size_t,
    );
}
unsafe extern "C-unwind" fn compute_owner_password(
    mut p_sec: *mut pdf_sec, mut opasswd: *const ::core::ffi::c_char, mut upasswd: *const ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut padded: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    let mut arc4: ARC4_CONTEXT = ARC4_CONTEXT {
        idx_i: 0,
        idx_j: 0,
        sbox: [0; 256],
    };
    let mut hash: [::core::ffi::c_uchar; 32] = [0; 32];
    passwd_padding(
        if strlen(opasswd) > 0 as size_t { opasswd } else { upasswd },
        &raw mut padded as *mut ::core::ffi::c_uchar,
    );
    MD5_init(&raw mut md5);
    MD5_write(&raw mut md5, &raw mut padded as *mut ::core::ffi::c_uchar, 32 as ::core::ffi::c_uint);
    MD5_final(&raw mut hash as *mut ::core::ffi::c_uchar, &raw mut md5);
    if (*p_sec).R >= 3 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 50 as ::core::ffi::c_int {
            MD5_init(&raw mut md5);
            MD5_write(
                &raw mut md5,
                &raw mut hash as *mut ::core::ffi::c_uchar,
                (*p_sec).key_size as ::core::ffi::c_uint,
            );
            MD5_final(&raw mut hash as *mut ::core::ffi::c_uchar, &raw mut md5);
            i += 1;
        }
    }
    ARC4_set_key(
        &raw mut arc4,
        (*p_sec).key_size as ::core::ffi::c_uint,
        &raw mut hash as *mut ::core::ffi::c_uchar,
    );
    passwd_padding(upasswd, &raw mut padded as *mut ::core::ffi::c_uchar);
    let mut tmp1: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut tmp2: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut key: [::core::ffi::c_uchar; 16] = [0; 16];
    ARC4(
        &raw mut arc4,
        32 as ::core::ffi::c_uint,
        &raw mut padded as *mut ::core::ffi::c_uchar,
        &raw mut tmp1 as *mut ::core::ffi::c_uchar,
    );
    if (*p_sec).R >= 3 as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_int;
        while i <= 19 as ::core::ffi::c_int {
            memcpy(
                &raw mut tmp2 as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                &raw mut tmp1 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                32 as size_t,
            );
            j = 0 as ::core::ffi::c_int;
            while j < (*p_sec).key_size {
                key[j as usize] = (hash[j as usize] as ::core::ffi::c_int ^ i) as ::core::ffi::c_uchar;
                j += 1;
            }
            ARC4_set_key(
                &raw mut arc4,
                (*p_sec).key_size as ::core::ffi::c_uint,
                &raw mut key as *mut ::core::ffi::c_uchar,
            );
            ARC4(
                &raw mut arc4,
                32 as ::core::ffi::c_uint,
                &raw mut tmp2 as *mut ::core::ffi::c_uchar,
                &raw mut tmp1 as *mut ::core::ffi::c_uchar,
            );
            i += 1;
        }
    }
    memcpy(
        &raw mut (*p_sec).O as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut tmp1 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        32 as size_t,
    );
}
unsafe extern "C-unwind" fn compute_encryption_key(mut p_sec: *mut pdf_sec, mut passwd: *const ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    let mut hash: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut padded: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    passwd_padding(passwd, &raw mut padded as *mut ::core::ffi::c_uchar);
    MD5_init(&raw mut md5);
    MD5_write(&raw mut md5, &raw mut padded as *mut ::core::ffi::c_uchar, 32 as ::core::ffi::c_uint);
    MD5_write(&raw mut md5, &raw mut (*p_sec).O as *mut ::core::ffi::c_uchar, 32 as ::core::ffi::c_uint);
    let mut tmp: [::core::ffi::c_uchar; 4] = [0; 4];
    tmp[0 as ::core::ffi::c_int as usize] =
        ((*p_sec).P as ::core::ffi::c_uchar as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    tmp[1 as ::core::ffi::c_int as usize] =
        (((*p_sec).P >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    tmp[2 as ::core::ffi::c_int as usize] =
        (((*p_sec).P >> 16 as ::core::ffi::c_int) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    tmp[3 as ::core::ffi::c_int as usize] =
        (((*p_sec).P >> 24 as ::core::ffi::c_int) as ::core::ffi::c_uchar as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    MD5_write(&raw mut md5, &raw mut tmp as *mut ::core::ffi::c_uchar, 4 as ::core::ffi::c_uint);
    MD5_write(&raw mut md5, &raw mut (*p_sec).ID as *mut ::core::ffi::c_uchar, 16 as ::core::ffi::c_uint);
    MD5_final(&raw mut hash as *mut ::core::ffi::c_uchar, &raw mut md5);
    if (*p_sec).R >= 3 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 50 as ::core::ffi::c_int {
            MD5_init(&raw mut md5);
            MD5_write(
                &raw mut md5,
                &raw mut hash as *mut ::core::ffi::c_uchar,
                (*p_sec).key_size as ::core::ffi::c_uint,
            );
            MD5_final(&raw mut hash as *mut ::core::ffi::c_uchar, &raw mut md5);
            i += 1;
        }
    }
    memcpy(
        &raw mut (*p_sec).key as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut hash as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        (*p_sec).key_size as size_t,
    );
}
unsafe extern "C-unwind" fn compute_user_password(mut p_sec: *mut pdf_sec, mut uplain: *const ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut arc4: ARC4_CONTEXT = ARC4_CONTEXT {
        idx_i: 0,
        idx_j: 0,
        sbox: [0; 256],
    };
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    let mut upasswd: [::core::ffi::c_uchar; 32] = [0; 32];
    compute_encryption_key(p_sec, uplain);
    match (*p_sec).R {
        2 => {
            ARC4_set_key(
                &raw mut arc4,
                (*p_sec).key_size as ::core::ffi::c_uint,
                &raw mut (*p_sec).key as *mut ::core::ffi::c_uchar,
            );
            ARC4(
                &raw mut arc4,
                32 as ::core::ffi::c_uint,
                PADDING_BYTES.as_ptr(),
                &raw mut upasswd as *mut ::core::ffi::c_uchar,
            );
        }
        3 | 4 => {
            let mut hash: [::core::ffi::c_uchar; 32] = [0; 32];
            let mut tmp1: [::core::ffi::c_uchar; 32] = [0; 32];
            let mut tmp2: [::core::ffi::c_uchar; 32] = [0; 32];
            MD5_init(&raw mut md5);
            MD5_write(&raw mut md5, PADDING_BYTES.as_ptr(), 32 as ::core::ffi::c_uint);
            MD5_write(&raw mut md5, &raw mut (*p_sec).ID as *mut ::core::ffi::c_uchar, 16 as ::core::ffi::c_uint);
            MD5_final(&raw mut hash as *mut ::core::ffi::c_uchar, &raw mut md5);
            ARC4_set_key(
                &raw mut arc4,
                (*p_sec).key_size as ::core::ffi::c_uint,
                &raw mut (*p_sec).key as *mut ::core::ffi::c_uchar,
            );
            ARC4(
                &raw mut arc4,
                16 as ::core::ffi::c_uint,
                &raw mut hash as *mut ::core::ffi::c_uchar,
                &raw mut tmp1 as *mut ::core::ffi::c_uchar,
            );
            i = 1 as ::core::ffi::c_int;
            while i <= 19 as ::core::ffi::c_int {
                let mut key: [::core::ffi::c_uchar; 16] = [0; 16];
                memcpy(
                    &raw mut tmp2 as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                    &raw mut tmp1 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                    16 as size_t,
                );
                j = 0 as ::core::ffi::c_int;
                while j < (*p_sec).key_size {
                    key[j as usize] = ((*p_sec).key[j as usize] as ::core::ffi::c_int ^ i) as ::core::ffi::c_uchar;
                    j += 1;
                }
                ARC4_set_key(
                    &raw mut arc4,
                    (*p_sec).key_size as ::core::ffi::c_uint,
                    &raw mut key as *mut ::core::ffi::c_uchar,
                );
                ARC4(
                    &raw mut arc4,
                    16 as ::core::ffi::c_uint,
                    &raw mut tmp2 as *mut ::core::ffi::c_uchar,
                    &raw mut tmp1 as *mut ::core::ffi::c_uchar,
                );
                i += 1;
            }
            memcpy(
                &raw mut upasswd as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                &raw mut tmp1 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                32 as size_t,
            );
        }
        _ => {
            pdf_abort!(b"Invalid revision number.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    }
    memcpy(
        &raw mut (*p_sec).U as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut upasswd as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        32 as size_t,
    );
}
unsafe extern "C-unwind" fn compute_hash_V5(
    mut hash: *mut ::core::ffi::c_uchar, mut passwd: *const ::core::ffi::c_char, mut salt: *const ::core::ffi::c_uchar,
    mut user_key: *const ::core::ffi::c_uchar, mut R: ::core::ffi::c_int,
) {
    let mut K: [::core::ffi::c_uchar; 64] = [0; 64];
    let mut K_len: size_t = 0;
    let mut nround: ::core::ffi::c_int = 0;
    let mut sha: SHA256_CONTEXT = SHA256_CONTEXT {
        h0: 0,
        h1: 0,
        h2: 0,
        h3: 0,
        h4: 0,
        h5: 0,
        h6: 0,
        h7: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    SHA256_init(&raw mut sha);
    SHA256_write(&raw mut sha, passwd as *const ::core::ffi::c_uchar, strlen(passwd) as ::core::ffi::c_uint);
    SHA256_write(&raw mut sha, salt, 8 as ::core::ffi::c_uint);
    if !user_key.is_null() {
        SHA256_write(&raw mut sha, user_key, 48 as ::core::ffi::c_uint);
    }
    SHA256_final(hash, &raw mut sha);
    if !(R == 5 as ::core::ffi::c_int || R == 6 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(
            c"assertion failed: %s".as_ptr(),
            b"R ==5 || R == 6\0" as *const u8 as *const ::core::ffi::c_char
        );
    } else {
    };
    if R == 5 as ::core::ffi::c_int {
        return;
    }
    memcpy(
        &raw mut K as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        hash as *const ::core::ffi::c_void,
        32 as size_t,
    );
    K_len = 32 as size_t;
    nround = 1 as ::core::ffi::c_int;
    loop {
        let mut K1: [::core::ffi::c_uchar; 256] = [0; 256];
        let mut Kr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut E: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut K1_len: size_t = 0;
        let mut E_len: size_t = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_int = 0;
        let mut E_mod3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        K1_len = strlen(passwd).wrapping_add(K_len).wrapping_add(
            (if !user_key.is_null() {
                48 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as size_t,
        );
        if !(K1_len < 240 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            pdf_abort!(
                c"assertion failed: %s".as_ptr(),
                b"K1_len < 240\0" as *const u8 as *const ::core::ffi::c_char
            );
        } else {
        };
        memcpy(
            &raw mut K1 as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            passwd as *const ::core::ffi::c_void,
            strlen(passwd),
        );
        memcpy(
            (&raw mut K1 as *mut ::core::ffi::c_uchar).offset(strlen(passwd) as isize) as *mut ::core::ffi::c_void,
            &raw mut K as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            K_len,
        );
        if !user_key.is_null() {
            memcpy(
                (&raw mut K1 as *mut ::core::ffi::c_uchar)
                    .offset(strlen(passwd) as isize)
                    .offset(K_len as isize) as *mut ::core::ffi::c_void,
                user_key as *const ::core::ffi::c_void,
                48 as size_t,
            );
        }
        Kr = new(
            (K1_len.wrapping_mul(64 as size_t) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
                as uint32_t,
        ) as *mut ::core::ffi::c_uchar;
        i = 0 as ::core::ffi::c_int;
        while i < 64 as ::core::ffi::c_int {
            memcpy(
                Kr.offset((i as size_t).wrapping_mul(K1_len) as isize) as *mut ::core::ffi::c_void,
                &raw mut K1 as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                K1_len,
            );
            i += 1;
        }
        AES_cbc_encrypt_tectonic(
            &raw mut K as *mut ::core::ffi::c_uchar,
            16 as size_t,
            (&raw mut K as *mut ::core::ffi::c_uchar).offset(16 as ::core::ffi::c_int as isize),
            0 as ::core::ffi::c_int,
            Kr,
            K1_len.wrapping_mul(64 as size_t),
            &raw mut E,
            &raw mut E_len,
        );
        free(Kr as *mut ::core::ffi::c_void);
        i = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            E_mod3 += *E.offset(i as isize) as ::core::ffi::c_int;
            i += 1;
        }
        E_mod3 %= 3 as ::core::ffi::c_int;
        match E_mod3 {
            0 => {
                let mut sha_0: SHA256_CONTEXT = SHA256_CONTEXT {
                    h0: 0,
                    h1: 0,
                    h2: 0,
                    h3: 0,
                    h4: 0,
                    h5: 0,
                    h6: 0,
                    h7: 0,
                    nblocks: 0,
                    buf: [0; 64],
                    count: 0,
                };
                SHA256_init(&raw mut sha_0);
                SHA256_write(&raw mut sha_0, E, E_len as ::core::ffi::c_uint);
                SHA256_final(&raw mut K as *mut ::core::ffi::c_uchar, &raw mut sha_0);
                K_len = 32 as size_t;
            }
            1 => {
                let mut sha_1: SHA512_CONTEXT = SHA512_CONTEXT {
                    state: SHA512_STATE {
                        h0: 0,
                        h1: 0,
                        h2: 0,
                        h3: 0,
                        h4: 0,
                        h5: 0,
                        h6: 0,
                        h7: 0,
                    },
                    nblocks: 0,
                    buf: [0; 128],
                    count: 0,
                };
                SHA384_init(&raw mut sha_1);
                SHA512_write(&raw mut sha_1, E, E_len as ::core::ffi::c_uint);
                SHA512_final(&raw mut K as *mut ::core::ffi::c_uchar, &raw mut sha_1);
                K_len = 48 as size_t;
            }
            2 => {
                let mut sha_2: SHA512_CONTEXT = SHA512_CONTEXT {
                    state: SHA512_STATE {
                        h0: 0,
                        h1: 0,
                        h2: 0,
                        h3: 0,
                        h4: 0,
                        h5: 0,
                        h6: 0,
                        h7: 0,
                    },
                    nblocks: 0,
                    buf: [0; 128],
                    count: 0,
                };
                SHA512_init(&raw mut sha_2);
                SHA512_write(&raw mut sha_2, E, E_len as ::core::ffi::c_uint);
                SHA512_final(&raw mut K as *mut ::core::ffi::c_uchar, &raw mut sha_2);
                K_len = 64 as size_t;
            }
            _ => {}
        }
        c = *E.offset(E_len.wrapping_sub(1 as size_t) as isize) as uint8_t as ::core::ffi::c_int;
        free(E as *mut ::core::ffi::c_void);
        if nround >= 64 as ::core::ffi::c_int && c <= nround - 32 as ::core::ffi::c_int {
            break;
        }
        nround += 1;
    }
    memcpy(
        hash as *mut ::core::ffi::c_void,
        &raw mut K as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        32 as size_t,
    );
}
unsafe extern "C-unwind" fn compute_owner_password_V5(mut p_sec: *mut pdf_sec, mut oplain: *const ::core::ffi::c_char) {
    let mut vsalt: [::core::ffi::c_uchar; 8] = [0; 8];
    let mut ksalt: [::core::ffi::c_uchar; 8] = [0; 8];
    let mut hash: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut OE: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut iv: [::core::ffi::c_uchar; 16] = [0; 16];
    let mut OE_len: size_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        vsalt[i as usize] = (genrand_int31() % 256 as ::core::ffi::c_long) as ::core::ffi::c_uchar;
        ksalt[i as usize] = (genrand_int31() % 256 as ::core::ffi::c_long) as ::core::ffi::c_uchar;
        i += 1;
    }
    compute_hash_V5(
        &raw mut hash as *mut ::core::ffi::c_uchar,
        oplain,
        &raw mut vsalt as *mut ::core::ffi::c_uchar,
        &raw mut (*p_sec).U as *mut ::core::ffi::c_uchar,
        (*p_sec).R,
    );
    memcpy(
        &raw mut (*p_sec).O as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut hash as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        32 as size_t,
    );
    memcpy(
        (&raw mut (*p_sec).O as *mut ::core::ffi::c_uchar).offset(32 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        &raw mut vsalt as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        (&raw mut (*p_sec).O as *mut ::core::ffi::c_uchar).offset(40 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        &raw mut ksalt as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        8 as size_t,
    );
    compute_hash_V5(
        &raw mut hash as *mut ::core::ffi::c_uchar,
        oplain,
        &raw mut ksalt as *mut ::core::ffi::c_uchar,
        &raw mut (*p_sec).U as *mut ::core::ffi::c_uchar,
        (*p_sec).R,
    );
    memset(
        &raw mut iv as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        AES_BLOCKSIZE as size_t,
    );
    AES_cbc_encrypt_tectonic(
        &raw mut hash as *mut ::core::ffi::c_uchar,
        32 as size_t,
        &raw mut iv as *mut ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int,
        &raw mut (*p_sec).key as *mut ::core::ffi::c_uchar,
        (*p_sec).key_size as size_t,
        &raw mut OE,
        &raw mut OE_len,
    );
    memcpy(
        &raw mut (*p_sec).OE as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        OE as *const ::core::ffi::c_void,
        32 as size_t,
    );
    free(OE as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn compute_user_password_V5(mut p_sec: *mut pdf_sec, mut uplain: *const ::core::ffi::c_char) {
    let mut vsalt: [::core::ffi::c_uchar; 8] = [0; 8];
    let mut ksalt: [::core::ffi::c_uchar; 8] = [0; 8];
    let mut hash: [::core::ffi::c_uchar; 32] = [0; 32];
    let mut UE: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut iv: [::core::ffi::c_uchar; 16] = [0; 16];
    let mut UE_len: size_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        vsalt[i as usize] = (genrand_int31() % 256 as ::core::ffi::c_long) as ::core::ffi::c_uchar;
        ksalt[i as usize] = (genrand_int31() % 256 as ::core::ffi::c_long) as ::core::ffi::c_uchar;
        i += 1;
    }
    compute_hash_V5(
        &raw mut hash as *mut ::core::ffi::c_uchar,
        uplain,
        &raw mut vsalt as *mut ::core::ffi::c_uchar,
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        (*p_sec).R,
    );
    memcpy(
        &raw mut (*p_sec).U as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut hash as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        32 as size_t,
    );
    memcpy(
        (&raw mut (*p_sec).U as *mut ::core::ffi::c_uchar).offset(32 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        &raw mut vsalt as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        8 as size_t,
    );
    memcpy(
        (&raw mut (*p_sec).U as *mut ::core::ffi::c_uchar).offset(40 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        &raw mut ksalt as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        8 as size_t,
    );
    compute_hash_V5(
        &raw mut hash as *mut ::core::ffi::c_uchar,
        uplain,
        &raw mut ksalt as *mut ::core::ffi::c_uchar,
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        (*p_sec).R,
    );
    memset(
        &raw mut iv as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        AES_BLOCKSIZE as size_t,
    );
    AES_cbc_encrypt_tectonic(
        &raw mut hash as *mut ::core::ffi::c_uchar,
        32 as size_t,
        &raw mut iv as *mut ::core::ffi::c_uchar,
        0 as ::core::ffi::c_int,
        &raw mut (*p_sec).key as *mut ::core::ffi::c_uchar,
        (*p_sec).key_size as size_t,
        &raw mut UE,
        &raw mut UE_len,
    );
    memcpy(
        &raw mut (*p_sec).UE as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        UE as *const ::core::ffi::c_void,
        32 as size_t,
    );
    free(UE as *mut ::core::ffi::c_void);
}
unsafe extern "C-unwind" fn check_version(mut p_sec: *mut pdf_sec, mut version: ::core::ffi::c_int) {
    if (*p_sec).V > 2 as ::core::ffi::c_int && version < 14 as ::core::ffi::c_int {
        pdf_warning!(b"Current encryption setting requires PDF version >= 1.4.\0" as *const u8 as *const ::core::ffi::c_char,);
        (*p_sec).V = 1 as ::core::ffi::c_int;
        (*p_sec).key_size = 5 as ::core::ffi::c_int;
    } else if (*p_sec).V == 4 as ::core::ffi::c_int && version < 15 as ::core::ffi::c_int {
        pdf_warning!(b"Current encryption setting requires PDF version >= 1.5.\0" as *const u8 as *const ::core::ffi::c_char,);
        (*p_sec).V = 2 as ::core::ffi::c_int;
    } else if (*p_sec).V == 5 as ::core::ffi::c_int && version < 17 as ::core::ffi::c_int {
        pdf_warning!(
            b"Current encryption setting requires PDF version >= 1.7 (plus Adobe Extension Level 3).\0" as *const u8 as *const ::core::ffi::c_char,
        );
        (*p_sec).V = 4 as ::core::ffi::c_int;
        (*p_sec).key_size = 16 as ::core::ffi::c_int;
    }
    if (*p_sec).V == 5 as ::core::ffi::c_int && version < 20 as ::core::ffi::c_int {
        (*p_sec).setting.need_adobe_extension = 1 as ::core::ffi::c_int;
    }
}
pub const STRINGPREP_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STRINGPREP_ERROR: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C-unwind" fn stringprep_profile(
    mut input: *const ::core::ffi::c_char, mut output: *mut *mut ::core::ffi::c_char, mut profile: *const ::core::ffi::c_char,
    mut flags: Stringprep_profile_flags,
) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = input;
    endptr = p.offset(strlen(p) as isize);
    while p < endptr {
        let mut ucv: int32_t = UC_UTF8_decode_char(&raw mut p as *mut *const ::core::ffi::c_uchar, endptr as *const ::core::ffi::c_uchar);
        if !UC_is_valid(ucv) {
            return STRINGPREP_ERROR;
        }
    }
    *output = new(
        (strlen(input).wrapping_add(1 as size_t) as uint32_t as usize).wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as usize)
            as uint32_t,
    ) as *mut ::core::ffi::c_char;
    strcpy(*output, input);
    return STRINGPREP_OK;
}
unsafe extern "C-unwind" fn preproc_password(
    mut passwd: *const ::core::ffi::c_char, mut outbuf: *mut ::core::ffi::c_char, mut V: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut saslpwd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    memset(outbuf as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, 128 as size_t);
    match V {
        1 | 2 | 3 | 4 => {
            let passwd_len = strlen(passwd);
            let mut i: size_t = 0;
            i = 0 as size_t;
            while i < passwd_len {
                if (*passwd.offset(i as isize) as ::core::ffi::c_int) < 0x20 as ::core::ffi::c_int
                    || *passwd.offset(i as isize) as ::core::ffi::c_int > 0x7e as ::core::ffi::c_int
                {
                    pdf_warning!(b"Non-ASCII-printable character found in password.\0" as *const u8 as *const ::core::ffi::c_char,);
                }
                i = i.wrapping_add(1);
            }
            memcpy(
                outbuf as *mut ::core::ffi::c_void,
                passwd as *const ::core::ffi::c_void,
                passwd_len.min(127),
            );
        }
        5 => {
            if stringprep_profile(
                passwd,
                &raw mut saslpwd,
                b"SASLprep\0" as *const u8 as *const ::core::ffi::c_char,
                0 as Stringprep_profile_flags,
            ) != STRINGPREP_OK
            {
                return -(1 as ::core::ffi::c_int);
            } else if !saslpwd.is_null() {
                memcpy(
                    outbuf as *mut ::core::ffi::c_void,
                    saslpwd as *const ::core::ffi::c_void,
                    if (127 as size_t) < strlen(saslpwd) {
                        127 as size_t
                    } else {
                        strlen(saslpwd)
                    },
                );
                free(saslpwd as *mut ::core::ffi::c_void);
            }
        }
        _ => {
            error = -(1 as ::core::ffi::c_int);
        }
    }
    return error;
}
unsafe extern "C-unwind" fn pdf_enc_set_password(
    mut p_sec: *mut pdf_sec, mut bits: ::core::ffi::c_uint, mut perm: ::core::ffi::c_uint, mut oplain: *const ::core::ffi::c_char,
    mut uplain: *const ::core::ffi::c_char,
) {
    let mut opasswd: [::core::ffi::c_char; 128] = [0; 128];
    let mut upasswd: [::core::ffi::c_char; 128] = [0; 128];
    let mut version: ::core::ffi::c_int = 0;
    let mut empty_passwd: [::core::ffi::c_char; 1] = crate::backend::c_char_array(*b"\0");
    version = pdf_get_version();
    (*p_sec).key_size = bits.wrapping_div(8 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    if (*p_sec).key_size == 5 as ::core::ffi::c_int {
        (*p_sec).V = 1 as ::core::ffi::c_int;
    } else if (*p_sec).key_size < 16 as ::core::ffi::c_int {
        (*p_sec).V = 2 as ::core::ffi::c_int;
    } else if (*p_sec).key_size == 16 as ::core::ffi::c_int {
        (*p_sec).V = if (*p_sec).setting.use_aes != 0 {
            4 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        };
    } else if (*p_sec).key_size == 32 as ::core::ffi::c_int {
        (*p_sec).V = 5 as ::core::ffi::c_int;
    } else {
        pdf_warning!(b"Key length %d unsupported.\0" as *const u8 as *const ::core::ffi::c_char, bits,);
        (*p_sec).key_size = 5 as ::core::ffi::c_int;
        (*p_sec).V = 2 as ::core::ffi::c_int;
    }
    check_version(p_sec, version);
    (*p_sec).P = (perm | 0xc0 as ::core::ffi::c_uint) as int32_t;
    if version >= 20 as ::core::ffi::c_int {
        (*p_sec).P = ((*p_sec).P as ::core::ffi::c_int | (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as int32_t;
    }
    match (*p_sec).V {
        1 => {
            (*p_sec).R = if ((*p_sec).P as ::core::ffi::c_long) < 0x100 as ::core::ffi::c_long {
                2 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            };
        }
        2 | 3 => {
            (*p_sec).R = 3 as ::core::ffi::c_int;
        }
        4 => {
            (*p_sec).R = 4 as ::core::ffi::c_int;
        }
        5 => {
            (*p_sec).R = 6 as ::core::ffi::c_int;
        }
        _ => {
            (*p_sec).R = 3 as ::core::ffi::c_int;
        }
    }
    memset(
        &raw mut opasswd as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        128 as size_t,
    );
    memset(
        &raw mut upasswd as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        128 as size_t,
    );
    if !oplain.is_null() {
        if preproc_password(oplain, &raw mut opasswd as *mut ::core::ffi::c_char, (*p_sec).V) < 0 as ::core::ffi::c_int {
            pdf_warning!(b"Invaid UTF-8 string for password.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    } else {
        preproc_password(
            &raw mut empty_passwd as *mut ::core::ffi::c_char,
            &raw mut opasswd as *mut ::core::ffi::c_char,
            (*p_sec).V,
        );
    }
    if !uplain.is_null() {
        if preproc_password(uplain, &raw mut upasswd as *mut ::core::ffi::c_char, (*p_sec).V) < 0 as ::core::ffi::c_int {
            pdf_warning!(b"Invalid UTF-8 string for passowrd.\0" as *const u8 as *const ::core::ffi::c_char,);
        }
    } else {
        preproc_password(
            &raw mut empty_passwd as *mut ::core::ffi::c_char,
            &raw mut upasswd as *mut ::core::ffi::c_char,
            (*p_sec).V,
        );
    }
    if (*p_sec).R >= 3 as ::core::ffi::c_int {
        (*p_sec).P = ((*p_sec).P as ::core::ffi::c_uint | 0xfffff000 as ::core::ffi::c_uint) as int32_t;
    }
    if (*p_sec).V < 5 as ::core::ffi::c_int {
        compute_owner_password(
            p_sec,
            &raw mut opasswd as *mut ::core::ffi::c_char,
            &raw mut upasswd as *mut ::core::ffi::c_char,
        );
        compute_user_password(p_sec, &raw mut upasswd as *mut ::core::ffi::c_char);
    } else if (*p_sec).V == 5 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 32 as ::core::ffi::c_int {
            (*p_sec).key[i as usize] = (genrand_int31() % 256 as ::core::ffi::c_long) as ::core::ffi::c_uchar;
            i += 1;
        }
        (*p_sec).key_size = 32 as ::core::ffi::c_int;
        compute_user_password_V5(p_sec, &raw mut upasswd as *mut ::core::ffi::c_char);
        compute_owner_password_V5(p_sec, &raw mut opasswd as *mut ::core::ffi::c_char);
    }
}
unsafe extern "C-unwind" fn calculate_key(mut p_sec: *mut pdf_sec, mut key: *mut ::core::ffi::c_uchar) {
    let mut len: ::core::ffi::c_int = (*p_sec).key_size + 5 as ::core::ffi::c_int;
    let mut tmp: [::core::ffi::c_uchar; 25] = [0; 25];
    let mut md5: MD5_CONTEXT = MD5_CONTEXT {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    memcpy(
        &raw mut tmp as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        &raw mut (*p_sec).key as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        (*p_sec).key_size as size_t,
    );
    tmp[(*p_sec).key_size as usize] =
        ((*p_sec).label.objnum as ::core::ffi::c_uchar as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    tmp[((*p_sec).key_size + 1 as ::core::ffi::c_int) as usize] = (((*p_sec).label.objnum >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar
        as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    tmp[((*p_sec).key_size + 2 as ::core::ffi::c_int) as usize] = (((*p_sec).label.objnum >> 16 as ::core::ffi::c_int) as ::core::ffi::c_uchar
        as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    tmp[((*p_sec).key_size + 3 as ::core::ffi::c_int) as usize] =
        ((*p_sec).label.gennum as ::core::ffi::c_uchar as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    tmp[((*p_sec).key_size + 4 as ::core::ffi::c_int) as usize] = (((*p_sec).label.gennum as ::core::ffi::c_int >> 8 as ::core::ffi::c_int)
        as ::core::ffi::c_uchar as ::core::ffi::c_int
        & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    if (*p_sec).V >= 4 as ::core::ffi::c_int {
        tmp[((*p_sec).key_size + 5 as ::core::ffi::c_int) as usize] = 0x73 as ::core::ffi::c_uchar;
        tmp[((*p_sec).key_size + 6 as ::core::ffi::c_int) as usize] = 0x41 as ::core::ffi::c_uchar;
        tmp[((*p_sec).key_size + 7 as ::core::ffi::c_int) as usize] = 0x6c as ::core::ffi::c_uchar;
        tmp[((*p_sec).key_size + 8 as ::core::ffi::c_int) as usize] = 0x54 as ::core::ffi::c_uchar;
        len += 4 as ::core::ffi::c_int;
    }
    MD5_init(&raw mut md5);
    MD5_write(&raw mut md5, &raw mut tmp as *mut ::core::ffi::c_uchar, len as ::core::ffi::c_uint);
    MD5_final(key, &raw mut md5);
}
pub(crate) unsafe fn encrypt_owned(security: *mut pdf_sec, plain: &[u8]) -> Result<Vec<u8>, crate::error::PdfError> {
    use crate::error::{PdfError, PdfErrorKind};
    let security_ref =
        unsafe { security.as_ref() }.ok_or_else(|| PdfError::new(PdfErrorKind::InvalidState, c"Invalid PDF encryption state".into()))?;
    let version = security_ref.V;
    if !matches!(version, 1 | 2 | 4 | 5) {
        return Err(PdfError::new(
            PdfErrorKind::InvalidState,
            std::ffi::CString::new(format!("pdfencrypt: Unexpected V value: {version}")).unwrap(),
        ));
    }
    let key_size = usize::try_from(security_ref.key_size)
        .ok()
        .filter(|&size| size > 0 && size <= if version == 5 { 32 } else { 16 })
        .ok_or_else(|| PdfError::new(PdfErrorKind::InvalidState, c"Invalid PDF encryption key size".into()))?;
    let mut key = [0; 32];
    let key = if version == 5 {
        &security_ref.key[..key_size]
    } else {
        crate::backend::runtime::catch_backend(|| {
            unsafe {
                calculate_key(security, key.as_mut_ptr());
            }
            Ok(())
        })?;
        &key[..(key_size + 5).min(16)]
    };
    if matches!(version, 1 | 2) {
        if plain.len() > u32::MAX as usize {
            return Err(PdfError::new(PdfErrorKind::InvalidState, c"PDF encrypted data is too large".into()));
        }
        let mut context = super::dpx_dpxcrypt::ARC4_CONTEXT {
            idx_i: 0,
            idx_j: 0,
            sbox: [0; 256],
        };
        let mut output = vec![0; plain.len()];
        context.encrypt_bytes(key, plain, &mut output);
        return Ok(output);
    }
    let mut iv = [0; 16];
    for byte in &mut iv {
        *byte = unsafe { super::dpx_mt19937ar::genrand_int31() } as u8;
    }
    super::dpx_dpxcrypt::aes_cbc_owned(key, plain, iv)
}

#[unsafe(export_name = "__pdf_impl_pdf_encrypt_data")]
pub unsafe extern "C-unwind" fn pdf_encrypt_data(
    security: *mut pdf_sec, plain: *const u8, plain_len: usize, cipher: *mut *mut u8, cipher_len: *mut usize,
) {
    let plain = if plain_len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(plain, plain_len) }
    };
    let output = unsafe { encrypt_owned(security, plain) }.unwrap_or_else(|error| pdf_abort!("{}", error.message.to_string_lossy()));
    unsafe {
        let bytes = new(output.len() as u32).cast::<u8>();
        core::ptr::copy_nonoverlapping(output.as_ptr(), bytes, output.len());
        *cipher = bytes;
        *cipher_len = output.len();
    }
}
#[unsafe(export_name = "__pdf_impl_pdf_enc_get_encrypt_dict")]
pub unsafe extern "C-unwind" fn pdf_enc_get_encrypt_dict(mut p_sec: *mut pdf_sec) -> *mut pdf_obj {
    let mut doc_encrypt: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if p_sec.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"p_sec\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    doc_encrypt = pdf_new_dict();
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"Filter\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_name(b"Standard\0" as *const u8 as *const ::core::ffi::c_char),
    );
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"V\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number((*p_sec).V as ::core::ffi::c_double),
    );
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"Length\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number(((*p_sec).key_size * 8 as ::core::ffi::c_int) as ::core::ffi::c_double),
    );
    if (*p_sec).V >= 4 as ::core::ffi::c_int {
        let mut CF: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        let mut StdCF: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
        CF = pdf_new_dict();
        StdCF = pdf_new_dict();
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"CFM\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(if (*p_sec).V == 4 as ::core::ffi::c_int {
                b"AESV2\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"AESV3\0" as *const u8 as *const ::core::ffi::c_char
            }),
        );
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"AuthEvent\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"DocOpen\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_dict(
            StdCF,
            pdf_new_name(b"Length\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number((*p_sec).key_size as ::core::ffi::c_double),
        );
        pdf_add_dict(CF, pdf_new_name(b"StdCF\0" as *const u8 as *const ::core::ffi::c_char), StdCF);
        pdf_add_dict(doc_encrypt, pdf_new_name(b"CF\0" as *const u8 as *const ::core::ffi::c_char), CF);
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"StmF\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"StdCF\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"StrF\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"StdCF\0" as *const u8 as *const ::core::ffi::c_char),
        );
    }
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"R\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number((*p_sec).R as ::core::ffi::c_double),
    );
    if (*p_sec).V < 5 as ::core::ffi::c_int {
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"O\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                &raw mut (*p_sec).O as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                32 as size_t,
            ),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"U\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                &raw mut (*p_sec).U as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                32 as size_t,
            ),
        );
    } else if (*p_sec).V == 5 as ::core::ffi::c_int {
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"O\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                &raw mut (*p_sec).O as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                48 as size_t,
            ),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"U\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                &raw mut (*p_sec).U as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                48 as size_t,
            ),
        );
    }
    pdf_add_dict(
        doc_encrypt,
        pdf_new_name(b"P\0" as *const u8 as *const ::core::ffi::c_char),
        pdf_new_number((*p_sec).P as ::core::ffi::c_double),
    );
    if (*p_sec).V == 5 as ::core::ffi::c_int {
        let mut perms: [::core::ffi::c_uchar; 16] = [0; 16];
        let mut cipher: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        let mut cipher_len: size_t = 0 as size_t;
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"OE\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                &raw mut (*p_sec).OE as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                32 as size_t,
            ),
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"UE\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(
                &raw mut (*p_sec).UE as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                32 as size_t,
            ),
        );
        perms[0 as ::core::ffi::c_int as usize] = ((*p_sec).P & 0xff as int32_t) as ::core::ffi::c_uchar;
        perms[1 as ::core::ffi::c_int as usize] = ((*p_sec).P >> 8 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
        perms[2 as ::core::ffi::c_int as usize] = ((*p_sec).P >> 16 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
        perms[3 as ::core::ffi::c_int as usize] = ((*p_sec).P >> 24 as ::core::ffi::c_int & 0xff as int32_t) as ::core::ffi::c_uchar;
        perms[4 as ::core::ffi::c_int as usize] = 0xff as ::core::ffi::c_uchar;
        perms[5 as ::core::ffi::c_int as usize] = 0xff as ::core::ffi::c_uchar;
        perms[6 as ::core::ffi::c_int as usize] = 0xff as ::core::ffi::c_uchar;
        perms[7 as ::core::ffi::c_int as usize] = 0xff as ::core::ffi::c_uchar;
        perms[8 as ::core::ffi::c_int as usize] = (if (*p_sec).setting.encrypt_metadata != 0 {
            'T' as i32
        } else {
            'F' as i32
        }) as ::core::ffi::c_uchar;
        perms[9 as ::core::ffi::c_int as usize] = 'a' as i32 as ::core::ffi::c_uchar;
        perms[10 as ::core::ffi::c_int as usize] = 'd' as i32 as ::core::ffi::c_uchar;
        perms[11 as ::core::ffi::c_int as usize] = 'b' as i32 as ::core::ffi::c_uchar;
        perms[12 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
        perms[13 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
        perms[14 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
        perms[15 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
        AES_ecb_encrypt(
            &raw mut (*p_sec).key as *mut ::core::ffi::c_uchar,
            (*p_sec).key_size as size_t,
            &raw mut perms as *mut ::core::ffi::c_uchar,
            16 as size_t,
            &raw mut cipher,
            &raw mut cipher_len,
        );
        pdf_add_dict(
            doc_encrypt,
            pdf_new_name(b"Perms\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_string(cipher as *const ::core::ffi::c_void, cipher_len),
        );
        free(cipher as *mut ::core::ffi::c_void);
    }
    return doc_encrypt;
}
#[unsafe(export_name = "__pdf_impl_pdf_enc_get_extension_dict")]
pub unsafe extern "C-unwind" fn pdf_enc_get_extension_dict(mut p_sec: *mut pdf_sec) -> *mut pdf_obj {
    let mut ext: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    let mut adbe: *mut pdf_obj = ::core::ptr::null_mut::<pdf_obj>();
    if p_sec.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"p_sec\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if (*p_sec).setting.need_adobe_extension != 0 {
        ext = pdf_new_dict();
        adbe = pdf_new_dict();
        pdf_add_dict(
            adbe,
            pdf_new_name(b"BaseVersion\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_name(b"1.7\0" as *const u8 as *const ::core::ffi::c_char),
        );
        pdf_add_dict(
            adbe,
            pdf_new_name(b"ExtensionLevel\0" as *const u8 as *const ::core::ffi::c_char),
            pdf_new_number(
                (if (*p_sec).R == 5 as ::core::ffi::c_int {
                    3 as ::core::ffi::c_int
                } else {
                    8 as ::core::ffi::c_int
                }) as ::core::ffi::c_double,
            ),
        );
        pdf_add_dict(ext, pdf_new_name(b"ADBE\0" as *const u8 as *const ::core::ffi::c_char), adbe);
    }
    return ext;
}
#[unsafe(export_name = "__pdf_impl_pdf_enc_set_label")]
pub unsafe extern "C-unwind" fn pdf_enc_set_label(mut p_sec: *mut pdf_sec, mut label: ::core::ffi::c_uint) {
    if p_sec.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"p_sec\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    (*p_sec).label.objnum = label as uint64_t;
}
#[unsafe(export_name = "__pdf_impl_pdf_enc_set_generation")]
pub unsafe extern "C-unwind" fn pdf_enc_set_generation(mut p_sec: *mut pdf_sec, mut generation: ::core::ffi::c_uint) {
    if p_sec.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"p_sec\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    (*p_sec).label.gennum = generation as uint16_t;
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
unsafe extern "C" {
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
    fn memset(__b: *mut ::core::ffi::c_void, __c: ::core::ffi::c_int, __len: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[repr(C)]
pub struct pdf_obj {
    _private: [u8; 0],
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_enc_init(
    mut id: *const ::core::ffi::c_uchar, mut keybits: ::core::ffi::c_int, mut permission: uint32_t, mut opasswd: *const ::core::ffi::c_char,
    mut upasswd: *const ::core::ffi::c_char, mut use_aes: ::core::ffi::c_int, mut encrypt_metadata: ::core::ffi::c_int, _result: *mut *mut pdf_sec,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_enc_init(id, keybits, permission, opasswd, upasswd, use_aes, encrypt_metadata) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_enc_close(mut p_sec: *mut *mut pdf_sec) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_enc_close(p_sec) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_encrypt_data(
    mut p_sec: *mut pdf_sec, mut plain: *const ::core::ffi::c_uchar, mut plain_len: size_t, mut cipher: *mut *mut ::core::ffi::c_uchar,
    mut cipher_len: *mut size_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_encrypt_data(p_sec, plain, plain_len, cipher, cipher_len) }) {
        Ok(value) => {
            unsafe {
                crate::compat::allocation::release((*cipher).cast());
            }
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_enc_get_encrypt_dict(mut p_sec: *mut pdf_sec, _result: *mut *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_enc_get_encrypt_dict(p_sec) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_enc_get_extension_dict(mut p_sec: *mut pdf_sec, _result: *mut *mut pdf_obj) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_enc_get_extension_dict(p_sec) }) {
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
pub unsafe extern "C" fn __pdf_entry_pdf_enc_set_label(mut p_sec: *mut pdf_sec, mut label: ::core::ffi::c_uint) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_enc_set_label(p_sec, label) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_pdf_enc_set_generation(mut p_sec: *mut pdf_sec, mut generation: ::core::ffi::c_uint) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { pdf_enc_set_generation(p_sec, generation) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
