/* This is DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

    Copyright (C) 2003-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
/*
 * The following codes for MD5 Message-Digest Algorithm were modified
 * by Jin-Hwan Cho on August 5, 2003 based on libgrypt-1.1.42.
 *
 * Copyright (C) 1995,1996,1998,1999,2001,2002,2003 Free Software Foundation, Inc.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA
 *
 * According to the definition of MD5 in RFC 1321 from April 1992.
 * NOTE: This is *not* the same file as the one from glibc.
 * Written by Ulrich Drepper <drepper@gnu.ai.mit.edu>, 1995.
 * heavily modified for GnuPG by Werner Koch <wk@gnupg.org>
 */
/*
 * The following codes for the SHA256 hash function are taken from
 * libgrypt-1.6.3. (slightly modified)
 *
 * sha256.c - SHA256 hash function
 * Copyright (C) 2003, 2006, 2008, 2009 Free Software Foundation, Inc.
 *
 * This file is part of Libgcrypt.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, see <http://www.gnu.org/licenses/>.
 */
/* The following code are taken from libgcrypt-1.6.3. (slightly modified):
 *
 * sha512.c - SHA384 and SHA512 hash functions
 * Copyright (C) 2003, 2008, 2009 Free Software Foundation, Inc.
 *
 * This file is part of Libgcrypt.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser general Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, see <http://www.gnu.org/licenses/>.
 */
/*
 * The following codes for the arcfour stream cipher were modified
 * by Jin-Hwan Cho on August 5, 2003 based on libgrypt-1.1.42.
 *
 * Copyright (C) 2000,2001,2002,2003 Free Software Foundation, Inc.
 *
 * Libgcrypt is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * Libgcrypt is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA
 *
 * According to the definition of MD5 in RFC 1321 from April 1992.
 * NOTE: This is *not* the same file as the one from glibc.
 * Written by Ulrich Drepper <drepper@gnu.ai.mit.edu>, 1995.
 * heavily modified for GnuPG by Werner Koch <wk@gnupg.org>
 */
#![allow(non_camel_case_types, non_snake_case)]
unsafe extern "C-unwind" {
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
    #[link_name = "__pdf_impl_genrand_int31"]
    fn genrand_int31() -> ::core::ffi::c_long;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
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
fn _gcry_burn_stack(bytes: ::core::ffi::c_int) {
    let mut buffer = [0u8; 64];
    if bytes > 64 {
        _gcry_burn_stack(bytes - 64);
    }
    core::hint::black_box(&mut buffer);
}
fn write_digest_blocks<const N: usize>(
    buffer: &mut [u8; N],
    count: &mut ::core::ffi::c_int,
    nblocks: &mut usize,
    mut input: &[u8],
    mut compress: impl FnMut(&[[u8; N]]),
) {
    if *count != 0 {
        let offset = *count as usize;
        let copied = input.len().min(N - offset);
        buffer[offset..offset + copied].copy_from_slice(&input[..copied]);
        *count += copied as ::core::ffi::c_int;
        input = &input[copied..];
        if *count as usize != N {
            return;
        }
        compress(core::slice::from_ref(buffer));
        *nblocks = nblocks.wrapping_add(1);
        *count = 0;
    }
    let (blocks, remainder) = input.as_chunks::<N>();
    compress(blocks);
    *nblocks = nblocks.wrapping_add(blocks.len());
    buffer[..remainder.len()].copy_from_slice(remainder);
    *count = remainder.len() as ::core::ffi::c_int;
}

fn finish_digest_blocks<const N: usize>(
    buffer: &mut [u8; N],
    count: &mut ::core::ffi::c_int,
    nblocks: &mut usize,
    bit_length: &[u8],
    mut compress: impl FnMut(&[[u8; N]]),
) {
    write_digest_blocks(buffer, count, nblocks, &[], &mut compress);
    let offset = *count as usize;
    buffer[offset] = 0x80;
    buffer[offset + 1..].fill(0);
    let length_offset = N - bit_length.len();
    if offset >= length_offset {
        compress(core::slice::from_ref(buffer));
        *nblocks = nblocks.wrapping_add(1);
        buffer.fill(0);
        *count = 0;
    } else {
        *count = length_offset as ::core::ffi::c_int;
    }
    buffer[length_offset..].copy_from_slice(bit_length);
    compress(core::slice::from_ref(buffer));
}

impl MD5_CONTEXT {
    fn write(&mut self, input: &[u8]) {
        let mut state = [self.A, self.B, self.C, self.D];
        write_digest_blocks(
            &mut self.buf,
            &mut self.count,
            &mut self.nblocks,
            input,
            |blocks| md5::block_api::compress(&mut state, blocks),
        );
        [self.A, self.B, self.C, self.D] = state;
    }

    fn finish(&mut self) -> [u8; 16] {
        let bit_length = (self.nblocks as u64)
            .wrapping_mul(64)
            .wrapping_add(self.count as u64)
            .wrapping_mul(8)
            .to_le_bytes();
        let mut state = [self.A, self.B, self.C, self.D];
        finish_digest_blocks(
            &mut self.buf,
            &mut self.count,
            &mut self.nblocks,
            &bit_length,
            |blocks| md5::block_api::compress(&mut state, blocks),
        );
        [self.A, self.B, self.C, self.D] = state;
        let mut output = [0; 16];
        let (words, _) = output.as_chunks_mut::<4>();
        for (bytes, word) in words.iter_mut().zip(state) {
            bytes.copy_from_slice(&word.to_le_bytes());
        }
        output
    }
}

#[unsafe(export_name = "__pdf_impl_MD5_init")]
pub unsafe extern "C-unwind" fn MD5_init(ctx: *mut MD5_CONTEXT) {
    let state = MD5_CONTEXT {
        A: 0x67452301,
        B: 0xefcdab89,
        C: 0x98badcfe,
        D: 0x10325476,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    unsafe { ctx.write(state) };
}

#[unsafe(export_name = "__pdf_impl_MD5_write")]
pub unsafe extern "C-unwind" fn MD5_write(
    ctx: *mut MD5_CONTEXT,
    input: *const u8,
    len: ::core::ffi::c_uint,
) {
    let input = if input.is_null() || len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(input, len as usize) }
    };
    unsafe { &mut *ctx }.write(input);
}

#[unsafe(export_name = "__pdf_impl_MD5_final")]
pub unsafe extern "C-unwind" fn MD5_final(output: *mut u8, ctx: *mut MD5_CONTEXT) {
    let digest = unsafe { &mut *ctx }.finish();
    unsafe { core::ptr::copy_nonoverlapping(digest.as_ptr(), output, digest.len()) };
}

impl SHA256_CONTEXT {
    fn write(&mut self, input: &[u8]) {
        let mut state = [
            self.h0, self.h1, self.h2, self.h3, self.h4, self.h5, self.h6, self.h7,
        ];
        write_digest_blocks(
            &mut self.buf,
            &mut self.count,
            &mut self.nblocks,
            input,
            |blocks| sha2::block_api::compress256(&mut state, blocks),
        );
        [
            self.h0, self.h1, self.h2, self.h3, self.h4, self.h5, self.h6, self.h7,
        ] = state;
    }

    fn finish(&mut self) -> [u8; 32] {
        let bit_length = (self.nblocks as u64)
            .wrapping_mul(64)
            .wrapping_add(self.count as u64)
            .wrapping_mul(8)
            .to_be_bytes();
        let mut state = [
            self.h0, self.h1, self.h2, self.h3, self.h4, self.h5, self.h6, self.h7,
        ];
        finish_digest_blocks(
            &mut self.buf,
            &mut self.count,
            &mut self.nblocks,
            &bit_length,
            |blocks| sha2::block_api::compress256(&mut state, blocks),
        );
        [
            self.h0, self.h1, self.h2, self.h3, self.h4, self.h5, self.h6, self.h7,
        ] = state;
        let mut output = [0; 32];
        let (words, _) = output.as_chunks_mut::<4>();
        for (bytes, word) in words.iter_mut().zip(state) {
            bytes.copy_from_slice(&word.to_be_bytes());
        }
        output
    }
}

#[unsafe(export_name = "__pdf_impl_SHA256_init")]
pub unsafe extern "C-unwind" fn SHA256_init(ctx: *mut SHA256_CONTEXT) {
    let state = SHA256_CONTEXT {
        h0: 0x6a09e667,
        h1: 0xbb67ae85,
        h2: 0x3c6ef372,
        h3: 0xa54ff53a,
        h4: 0x510e527f,
        h5: 0x9b05688c,
        h6: 0x1f83d9ab,
        h7: 0x5be0cd19,
        nblocks: 0,
        buf: [0; 64],
        count: 0,
    };
    unsafe { ctx.write(state) };
}

#[unsafe(export_name = "__pdf_impl_SHA256_write")]
pub unsafe extern "C-unwind" fn SHA256_write(
    ctx: *mut SHA256_CONTEXT,
    input: *const u8,
    len: ::core::ffi::c_uint,
) {
    let input = if input.is_null() || len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(input, len as usize) }
    };
    unsafe { &mut *ctx }.write(input);
}

#[unsafe(export_name = "__pdf_impl_SHA256_final")]
pub unsafe extern "C-unwind" fn SHA256_final(output: *mut u8, ctx: *mut SHA256_CONTEXT) {
    let digest = unsafe { &mut *ctx }.finish();
    unsafe { core::ptr::copy_nonoverlapping(digest.as_ptr(), output, digest.len()) };
}

impl SHA512_CONTEXT {
    fn write(&mut self, input: &[u8]) {
        let mut state = [
            self.state.h0,
            self.state.h1,
            self.state.h2,
            self.state.h3,
            self.state.h4,
            self.state.h5,
            self.state.h6,
            self.state.h7,
        ];
        write_digest_blocks(
            &mut self.buf,
            &mut self.count,
            &mut self.nblocks,
            input,
            |blocks| sha2::block_api::compress512(&mut state, blocks),
        );
        [
            self.state.h0,
            self.state.h1,
            self.state.h2,
            self.state.h3,
            self.state.h4,
            self.state.h5,
            self.state.h6,
            self.state.h7,
        ] = state;
    }

    fn finish(&mut self) -> [u8; 64] {
        let bit_length = (self.nblocks as u128)
            .wrapping_mul(128)
            .wrapping_add(self.count as u128)
            .wrapping_mul(8)
            .to_be_bytes();
        let mut state = [
            self.state.h0,
            self.state.h1,
            self.state.h2,
            self.state.h3,
            self.state.h4,
            self.state.h5,
            self.state.h6,
            self.state.h7,
        ];
        finish_digest_blocks(
            &mut self.buf,
            &mut self.count,
            &mut self.nblocks,
            &bit_length,
            |blocks| sha2::block_api::compress512(&mut state, blocks),
        );
        [
            self.state.h0,
            self.state.h1,
            self.state.h2,
            self.state.h3,
            self.state.h4,
            self.state.h5,
            self.state.h6,
            self.state.h7,
        ] = state;
        let mut output = [0; 64];
        let (words, _) = output.as_chunks_mut::<8>();
        for (bytes, word) in words.iter_mut().zip(state) {
            bytes.copy_from_slice(&word.to_be_bytes());
        }
        output
    }
}

#[unsafe(export_name = "__pdf_impl_SHA512_init")]
pub unsafe extern "C-unwind" fn SHA512_init(ctx: *mut SHA512_CONTEXT) {
    let state = SHA512_CONTEXT {
        state: SHA512_STATE {
            h0: 0x6a09e667f3bcc908,
            h1: 0xbb67ae8584caa73b,
            h2: 0x3c6ef372fe94f82b,
            h3: 0xa54ff53a5f1d36f1,
            h4: 0x510e527fade682d1,
            h5: 0x9b05688c2b3e6c1f,
            h6: 0x1f83d9abfb41bd6b,
            h7: 0x5be0cd19137e2179,
        },
        nblocks: 0,
        buf: [0; 128],
        count: 0,
    };
    unsafe { ctx.write(state) };
}

#[unsafe(export_name = "__pdf_impl_SHA384_init")]
pub unsafe extern "C-unwind" fn SHA384_init(ctx: *mut SHA512_CONTEXT) {
    let state = SHA512_CONTEXT {
        state: SHA512_STATE {
            h0: 0xcbbb9d5dc1059ed8,
            h1: 0x629a292a367cd507,
            h2: 0x9159015a3070dd17,
            h3: 0x152fecd8f70e5939,
            h4: 0x67332667ffc00b31,
            h5: 0x8eb44a8768581511,
            h6: 0xdb0c2e0d64f98fa7,
            h7: 0x47b5481dbefa4fa4,
        },
        nblocks: 0,
        buf: [0; 128],
        count: 0,
    };
    unsafe { ctx.write(state) };
}

#[unsafe(export_name = "__pdf_impl_SHA512_write")]
pub unsafe extern "C-unwind" fn SHA512_write(
    ctx: *mut SHA512_CONTEXT,
    input: *const u8,
    len: ::core::ffi::c_uint,
) {
    let input = if input.is_null() || len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(input, len as usize) }
    };
    unsafe { &mut *ctx }.write(input);
}

#[unsafe(export_name = "__pdf_impl_SHA512_final")]
pub unsafe extern "C-unwind" fn SHA512_final(output: *mut u8, ctx: *mut SHA512_CONTEXT) {
    let digest = unsafe { &mut *ctx }.finish();
    unsafe { core::ptr::copy_nonoverlapping(digest.as_ptr(), output, digest.len()) };
}
impl ARC4_CONTEXT {
    fn next_byte(&mut self) -> u8 {
        let i = (self.idx_i as u8).wrapping_add(1);
        let j = (self.idx_j as u8).wrapping_add(self.sbox[i as usize]);
        self.sbox.swap(i as usize, j as usize);
        self.idx_i = i as ::core::ffi::c_int;
        self.idx_j = j as ::core::ffi::c_int;
        self.sbox[self.sbox[i as usize].wrapping_add(self.sbox[j as usize]) as usize]
    }

    fn set_key(&mut self, key: &[u8]) {
        assert!(!key.is_empty());
        self.idx_i = 0;
        self.idx_j = 0;
        for (index, byte) in self.sbox.iter_mut().enumerate() {
            *byte = index as u8;
        }
        let mut j = 0u8;
        for i in 0..self.sbox.len() {
            j = j
                .wrapping_add(self.sbox[i])
                .wrapping_add(key[i % key.len()]);
            self.sbox.swap(i, j as usize);
        }
    }
}
#[unsafe(export_name = "__pdf_impl_ARC4")]
pub unsafe extern "C-unwind" fn ARC4(
    ctx: *mut ARC4_CONTEXT,
    len: ::core::ffi::c_uint,
    inbuf: *const ::core::ffi::c_uchar,
    outbuf: *mut ::core::ffi::c_uchar,
) {
    let ctx = unsafe { &mut *ctx };
    for index in 0..len as usize {
        let mask = ctx.next_byte();
        // Read before writing so the C API continues to support overlapping buffers.
        unsafe { outbuf.add(index).write(inbuf.add(index).read() ^ mask) };
    }
    _gcry_burn_stack(64 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_ARC4_set_key")]
pub unsafe extern "C-unwind" fn ARC4_set_key(
    ctx: *mut ARC4_CONTEXT,
    keylen: ::core::ffi::c_uint,
    key: *const ::core::ffi::c_uchar,
) {
    assert!(keylen > 0);
    let key = unsafe { core::slice::from_raw_parts(key, keylen.min(256) as usize) };
    unsafe { &mut *ctx }.set_key(key);
    _gcry_burn_stack(300 as ::core::ffi::c_int);
}
enum AesCipher {
    Aes128(aes::Aes128),
    Aes192(aes::Aes192),
    Aes256(aes::Aes256),
}

impl AesCipher {
    fn new(key: &[u8]) -> Self {
        use aes::cipher::KeyInit;
        match key.len() {
            16 => Self::Aes128(aes::Aes128::new_from_slice(key).unwrap()),
            24 => Self::Aes192(aes::Aes192::new_from_slice(key).unwrap()),
            32 => Self::Aes256(aes::Aes256::new_from_slice(key).unwrap()),
            _ => crate::backend::runtime::raise(c"Invalid AES key length".to_owned()),
        }
    }

    fn encrypt(&self, input: &[u8], output: &mut [u8], mut iv: Option<[u8; 16]>, padding: u8) {
        use aes::cipher::{Array, BlockCipherEncrypt};
        let (blocks, _) = output.as_chunks_mut::<16>();
        for (index, destination) in blocks.iter_mut().enumerate() {
            let start = index * 16;
            let source = &input[start..input.len().min(start + 16)];
            let mut block = [padding; 16];
            block[..source.len()].copy_from_slice(source);
            if let Some(previous) = iv {
                for (byte, previous) in block.iter_mut().zip(previous) {
                    *byte ^= previous;
                }
            }
            let mut block = Array::from(block);
            match self {
                Self::Aes128(cipher) => cipher.encrypt_block(&mut block),
                Self::Aes192(cipher) => cipher.encrypt_block(&mut block),
                Self::Aes256(cipher) => cipher.encrypt_block(&mut block),
            }
            destination.copy_from_slice(&block);
            if iv.is_some() {
                iv = Some(block.into());
            }
        }
    }
}

fn aes_padded_length(length: usize, padding: bool) -> usize {
    let remainder = length % 16;
    let padbytes = if padding || remainder != 0 {
        16 - remainder
    } else {
        0
    };
    length
        .checked_add(padbytes)
        .unwrap_or_else(|| crate::backend::runtime::raise(c"AES output length overflow".to_owned()))
}

unsafe fn with_ciphertext(
    cipher: *mut *mut u8,
    cipher_len: *mut usize,
    length: usize,
    encrypt: impl FnOnce(&mut [u8]),
) {
    let allocation_size = u32::try_from(length)
        .ok()
        .filter(|_| length <= isize::MAX as usize)
        .unwrap_or_else(|| crate::backend::runtime::raise(c"AES output is too large".to_owned()));
    let output = unsafe { new(allocation_size).cast::<u8>() };
    unsafe { output.write_bytes(0, length) };
    encrypt(unsafe { core::slice::from_raw_parts_mut(output, length) });
    unsafe {
        cipher.write(output);
        cipher_len.write(length);
    }
}

#[unsafe(export_name = "__pdf_impl_AES_ecb_encrypt")]
pub unsafe extern "C-unwind" fn AES_ecb_encrypt(
    key: *const u8,
    key_len: usize,
    plain: *const u8,
    plain_len: usize,
    cipher: *mut *mut u8,
    cipher_len: *mut usize,
) {
    let key = unsafe { core::slice::from_raw_parts(key, key_len) };
    let aes = AesCipher::new(key);
    let plain = if plain_len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(plain, plain_len) }
    };
    let length = aes_padded_length(plain.len(), false);
    unsafe {
        with_ciphertext(cipher, cipher_len, length, |output| {
            aes.encrypt(plain, output, None, 0)
        });
    }
}

#[unsafe(export_name = "__pdf_impl_AES_cbc_encrypt_tectonic")]
pub unsafe extern "C-unwind" fn AES_cbc_encrypt_tectonic(
    key: *const u8,
    key_len: usize,
    iv: *const u8,
    padding: ::core::ffi::c_int,
    plain: *const u8,
    plain_len: usize,
    cipher: *mut *mut u8,
    cipher_len: *mut usize,
) {
    let key = unsafe { core::slice::from_raw_parts(key, key_len) };
    let aes = AesCipher::new(key);
    let plain = if plain_len == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(plain, plain_len) }
    };
    let prepend_iv = iv.is_null();
    let iv = if prepend_iv {
        let mut bytes = [0; 16];
        for byte in &mut bytes {
            *byte = unsafe { genrand_int31() } as u8;
        }
        bytes
    } else {
        unsafe { iv.cast::<[u8; 16]>().read() }
    };
    let encrypted_len = aes_padded_length(plain.len(), padding != 0);
    let prefix_len = if prepend_iv { 16 } else { 0 };
    let length = encrypted_len.checked_add(prefix_len).unwrap_or_else(|| {
        crate::backend::runtime::raise(c"AES output length overflow".to_owned())
    });
    let padbytes = (encrypted_len - plain.len()) as u8;
    unsafe {
        with_ciphertext(cipher, cipher_len, length, |output| {
            if prepend_iv {
                output[..16].copy_from_slice(&iv);
            }
            aes.encrypt(plain, &mut output[prefix_len..], Some(iv), padbytes);
        });
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_MD5_init(ctx: *mut MD5_CONTEXT) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { MD5_init(ctx) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_MD5_write(
    hd: *mut MD5_CONTEXT,
    inbuf: *const ::core::ffi::c_uchar,
    inlen: ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { MD5_write(hd, inbuf, inlen) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_MD5_final(
    outbuf: *mut ::core::ffi::c_uchar,
    hd: *mut MD5_CONTEXT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { MD5_final(outbuf, hd) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA256_init(hd: *mut SHA256_CONTEXT) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA256_init(hd) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA256_write(
    hd: *mut SHA256_CONTEXT,
    inbuf: *const ::core::ffi::c_uchar,
    inlen: ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA256_write(hd, inbuf, inlen) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA256_final(
    outbuf: *mut ::core::ffi::c_uchar,
    hd: *mut SHA256_CONTEXT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA256_final(outbuf, hd) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA512_init(ctx: *mut SHA512_CONTEXT) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA512_init(ctx) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA384_init(ctx: *mut SHA512_CONTEXT) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA384_init(ctx) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA512_write(
    hd: *mut SHA512_CONTEXT,
    inbuf: *const ::core::ffi::c_uchar,
    inlen: ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA512_write(hd, inbuf, inlen) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA512_final(
    outbuf: *mut ::core::ffi::c_uchar,
    hd: *mut SHA512_CONTEXT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA512_final(outbuf, hd) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_ARC4(
    ctx: *mut ARC4_CONTEXT,
    len: ::core::ffi::c_uint,
    inbuf: *const ::core::ffi::c_uchar,
    outbuf: *mut ::core::ffi::c_uchar,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ARC4(ctx, len, inbuf, outbuf) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_ARC4_set_key(
    ctx: *mut ARC4_CONTEXT,
    keylen: ::core::ffi::c_uint,
    key: *const ::core::ffi::c_uchar,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ARC4_set_key(ctx, keylen, key) }) {
        Ok(()) => 0,
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_AES_ecb_encrypt(
    key: *const ::core::ffi::c_uchar,
    key_len: size_t,
    plain: *const ::core::ffi::c_uchar,
    plain_len: size_t,
    cipher: *mut *mut ::core::ffi::c_uchar,
    cipher_len: *mut size_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        AES_ecb_encrypt(key, key_len, plain, plain_len, cipher, cipher_len)
    }) {
        Ok(()) => {
            unsafe {
                crate::compat::allocation::release((*cipher).cast());
            }
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_AES_cbc_encrypt_tectonic(
    key: *const ::core::ffi::c_uchar,
    key_len: size_t,
    iv: *const ::core::ffi::c_uchar,
    padding: ::core::ffi::c_int,
    plain: *const ::core::ffi::c_uchar,
    plain_len: size_t,
    cipher: *mut *mut ::core::ffi::c_uchar,
    cipher_len: *mut size_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        AES_cbc_encrypt_tectonic(
            key, key_len, iv, padding, plain, plain_len, cipher, cipher_len,
        )
    }) {
        Ok(()) => {
            unsafe {
                crate::compat::allocation::release((*cipher).cast());
            }
            0
        }
        Err(()) => -1,
    }
}
