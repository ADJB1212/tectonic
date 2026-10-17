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
#![allow(
    warnings,
    clippy::all,
    unsafe_op_in_unsafe_fn,
    static_mut_refs,
    dangerous_implicit_autorefs
)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AES_CONTEXT {
    pub nrounds: ::core::ffi::c_int,
    pub rk: [uint32_t; 60],
    pub iv: [::core::ffi::c_uchar; 16],
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const AES_BLOCKSIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn _gcry_burn_stack(mut bytes: ::core::ffi::c_int) {
    let mut buf: [::core::ffi::c_char; 64] = [0; 64];
    memset(
        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
    );
    bytes = (bytes as ::core::ffi::c_ulong).wrapping_sub(::core::mem::size_of::<
        [::core::ffi::c_char; 64],
    >() as usize as ::core::ffi::c_ulong) as ::core::ffi::c_int as ::core::ffi::c_int;
    if bytes > 0 as ::core::ffi::c_int {
        _gcry_burn_stack(bytes);
    }
}
#[unsafe(export_name = "__pdf_impl_MD5_init")]
pub unsafe extern "C-unwind" fn MD5_init(mut ctx: *mut MD5_CONTEXT) {
    (*ctx).A = 0x67452301 as ::core::ffi::c_uint as uint32_t;
    (*ctx).B = 0xefcdab89 as ::core::ffi::c_uint as uint32_t;
    (*ctx).C = 0x98badcfe as ::core::ffi::c_uint as uint32_t;
    (*ctx).D = 0x10325476 as ::core::ffi::c_uint as uint32_t;
    (*ctx).nblocks = 0 as size_t;
    (*ctx).count = 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn transform(
    mut ctx: *mut MD5_CONTEXT,
    mut data: *const ::core::ffi::c_uchar,
) {
    let mut correct_words: [uint32_t; 16] = [0; 16];
    let mut A: uint32_t = (*ctx).A;
    let mut B: uint32_t = (*ctx).B;
    let mut C: uint32_t = (*ctx).C;
    let mut D: uint32_t = (*ctx).D;
    let mut cwp: *mut uint32_t = &raw mut correct_words as *mut uint32_t;
    {
        for (index, word) in correct_words.iter_mut().enumerate() {
            *word = std::ptr::read_unaligned(data.add(index * 4).cast::<u32>()).to_le();
        }
    };
    let fresh4 = cwp;
    cwp = cwp.offset(1);
    A = (A as ::core::ffi::c_uint).wrapping_add(
        (D ^ B & (C ^ D))
            .wrapping_add(*fresh4)
            .wrapping_add(0xd76aa478 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    A = A << 7 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    let fresh5 = cwp;
    cwp = cwp.offset(1);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (C ^ A & (B ^ C))
            .wrapping_add(*fresh5)
            .wrapping_add(0xe8c7b756 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 12 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    let fresh6 = cwp;
    cwp = cwp.offset(1);
    C = C.wrapping_add(
        (B ^ D & (A ^ B))
            .wrapping_add(*fresh6)
            .wrapping_add(0x242070db as uint32_t),
    );
    C = C << 17 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    let fresh7 = cwp;
    cwp = cwp.offset(1);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (A ^ C & (D ^ A))
            .wrapping_add(*fresh7)
            .wrapping_add(0xc1bdceee as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 22 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    let fresh8 = cwp;
    cwp = cwp.offset(1);
    A = (A as ::core::ffi::c_uint).wrapping_add(
        (D ^ B & (C ^ D))
            .wrapping_add(*fresh8)
            .wrapping_add(0xf57c0faf as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    A = A << 7 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    let fresh9 = cwp;
    cwp = cwp.offset(1);
    D = D.wrapping_add(
        (C ^ A & (B ^ C))
            .wrapping_add(*fresh9)
            .wrapping_add(0x4787c62a as uint32_t),
    );
    D = D << 12 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    let fresh10 = cwp;
    cwp = cwp.offset(1);
    C = (C as ::core::ffi::c_uint).wrapping_add(
        (B ^ D & (A ^ B))
            .wrapping_add(*fresh10)
            .wrapping_add(0xa8304613 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    C = C << 17 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    let fresh11 = cwp;
    cwp = cwp.offset(1);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (A ^ C & (D ^ A))
            .wrapping_add(*fresh11)
            .wrapping_add(0xfd469501 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 22 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    let fresh12 = cwp;
    cwp = cwp.offset(1);
    A = A.wrapping_add(
        (D ^ B & (C ^ D))
            .wrapping_add(*fresh12)
            .wrapping_add(0x698098d8 as uint32_t),
    );
    A = A << 7 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    let fresh13 = cwp;
    cwp = cwp.offset(1);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (C ^ A & (B ^ C))
            .wrapping_add(*fresh13)
            .wrapping_add(0x8b44f7af as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 12 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    let fresh14 = cwp;
    cwp = cwp.offset(1);
    C = (C as ::core::ffi::c_uint).wrapping_add(
        (B ^ D & (A ^ B))
            .wrapping_add(*fresh14)
            .wrapping_add(0xffff5bb1 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    C = C << 17 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    let fresh15 = cwp;
    cwp = cwp.offset(1);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (A ^ C & (D ^ A))
            .wrapping_add(*fresh15)
            .wrapping_add(0x895cd7be as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 22 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    let fresh16 = cwp;
    cwp = cwp.offset(1);
    A = A.wrapping_add(
        (D ^ B & (C ^ D))
            .wrapping_add(*fresh16)
            .wrapping_add(0x6b901122 as uint32_t),
    );
    A = A << 7 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    let fresh17 = cwp;
    cwp = cwp.offset(1);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (C ^ A & (B ^ C))
            .wrapping_add(*fresh17)
            .wrapping_add(0xfd987193 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 12 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 12 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    let fresh18 = cwp;
    cwp = cwp.offset(1);
    C = (C as ::core::ffi::c_uint).wrapping_add(
        (B ^ D & (A ^ B))
            .wrapping_add(*fresh18)
            .wrapping_add(0xa679438e as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    C = C << 17 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    let fresh19 = cwp;
    cwp = cwp.offset(1);
    B = B.wrapping_add(
        (A ^ C & (D ^ A))
            .wrapping_add(*fresh19)
            .wrapping_add(0x49b40821 as uint32_t),
    );
    B = B << 22 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = (A as ::core::ffi::c_uint).wrapping_add(
        (C ^ D & (B ^ C))
            .wrapping_add(correct_words[1 as ::core::ffi::c_int as usize])
            .wrapping_add(0xf61e2562 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    A = A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (B ^ C & (A ^ B))
            .wrapping_add(correct_words[6 as ::core::ffi::c_int as usize])
            .wrapping_add(0xc040b340 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 9 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = C.wrapping_add(
        (A ^ B & (D ^ A))
            .wrapping_add(correct_words[11 as ::core::ffi::c_int as usize])
            .wrapping_add(0x265e5a51 as uint32_t),
    );
    C = C << 14 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (D ^ A & (C ^ D))
            .wrapping_add(correct_words[0 as ::core::ffi::c_int as usize])
            .wrapping_add(0xe9b6c7aa as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 20 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = (A as ::core::ffi::c_uint).wrapping_add(
        (C ^ D & (B ^ C))
            .wrapping_add(correct_words[5 as ::core::ffi::c_int as usize])
            .wrapping_add(0xd62f105d as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    A = A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = D.wrapping_add(
        (B ^ C & (A ^ B))
            .wrapping_add(correct_words[10 as ::core::ffi::c_int as usize])
            .wrapping_add(0x2441453 as uint32_t),
    );
    D = D << 9 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = (C as ::core::ffi::c_uint).wrapping_add(
        (A ^ B & (D ^ A))
            .wrapping_add(correct_words[15 as ::core::ffi::c_int as usize])
            .wrapping_add(0xd8a1e681 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    C = C << 14 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (D ^ A & (C ^ D))
            .wrapping_add(correct_words[4 as ::core::ffi::c_int as usize])
            .wrapping_add(0xe7d3fbc8 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 20 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = A.wrapping_add(
        (C ^ D & (B ^ C))
            .wrapping_add(correct_words[9 as ::core::ffi::c_int as usize])
            .wrapping_add(0x21e1cde6 as uint32_t),
    );
    A = A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (B ^ C & (A ^ B))
            .wrapping_add(correct_words[14 as ::core::ffi::c_int as usize])
            .wrapping_add(0xc33707d6 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 9 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = (C as ::core::ffi::c_uint).wrapping_add(
        (A ^ B & (D ^ A))
            .wrapping_add(correct_words[3 as ::core::ffi::c_int as usize])
            .wrapping_add(0xf4d50d87 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    C = C << 14 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = B.wrapping_add(
        (D ^ A & (C ^ D))
            .wrapping_add(correct_words[8 as ::core::ffi::c_int as usize])
            .wrapping_add(0x455a14ed as uint32_t),
    );
    B = B << 20 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = (A as ::core::ffi::c_uint).wrapping_add(
        (C ^ D & (B ^ C))
            .wrapping_add(correct_words[13 as ::core::ffi::c_int as usize])
            .wrapping_add(0xa9e3e905 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    A = A << 5 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 5 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (B ^ C & (A ^ B))
            .wrapping_add(correct_words[2 as ::core::ffi::c_int as usize])
            .wrapping_add(0xfcefa3f8 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 9 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = C.wrapping_add(
        (A ^ B & (D ^ A))
            .wrapping_add(correct_words[7 as ::core::ffi::c_int as usize])
            .wrapping_add(0x676f02d9 as uint32_t),
    );
    C = C << 14 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 14 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (D ^ A & (C ^ D))
            .wrapping_add(correct_words[12 as ::core::ffi::c_int as usize])
            .wrapping_add(0x8d2a4c8a as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 20 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 20 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = (A as ::core::ffi::c_uint).wrapping_add(
        (B ^ C ^ D)
            .wrapping_add(correct_words[5 as ::core::ffi::c_int as usize])
            .wrapping_add(0xfffa3942 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    A = A << 4 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (A ^ B ^ C)
            .wrapping_add(correct_words[8 as ::core::ffi::c_int as usize])
            .wrapping_add(0x8771f681 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 11 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = C.wrapping_add(
        (D ^ A ^ B)
            .wrapping_add(correct_words[11 as ::core::ffi::c_int as usize])
            .wrapping_add(0x6d9d6122 as uint32_t),
    );
    C = C << 16 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (C ^ D ^ A)
            .wrapping_add(correct_words[14 as ::core::ffi::c_int as usize])
            .wrapping_add(0xfde5380c as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 23 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = (A as ::core::ffi::c_uint).wrapping_add(
        (B ^ C ^ D)
            .wrapping_add(correct_words[1 as ::core::ffi::c_int as usize])
            .wrapping_add(0xa4beea44 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    A = A << 4 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = D.wrapping_add(
        (A ^ B ^ C)
            .wrapping_add(correct_words[4 as ::core::ffi::c_int as usize])
            .wrapping_add(0x4bdecfa9 as uint32_t),
    );
    D = D << 11 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = (C as ::core::ffi::c_uint).wrapping_add(
        (D ^ A ^ B)
            .wrapping_add(correct_words[7 as ::core::ffi::c_int as usize])
            .wrapping_add(0xf6bb4b60 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    C = C << 16 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (C ^ D ^ A)
            .wrapping_add(correct_words[10 as ::core::ffi::c_int as usize])
            .wrapping_add(0xbebfbc70 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 23 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = A.wrapping_add(
        (B ^ C ^ D)
            .wrapping_add(correct_words[13 as ::core::ffi::c_int as usize])
            .wrapping_add(0x289b7ec6 as uint32_t),
    );
    A = A << 4 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (A ^ B ^ C)
            .wrapping_add(correct_words[0 as ::core::ffi::c_int as usize])
            .wrapping_add(0xeaa127fa as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 11 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = (C as ::core::ffi::c_uint).wrapping_add(
        (D ^ A ^ B)
            .wrapping_add(correct_words[3 as ::core::ffi::c_int as usize])
            .wrapping_add(0xd4ef3085 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    C = C << 16 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = B.wrapping_add(
        (C ^ D ^ A)
            .wrapping_add(correct_words[6 as ::core::ffi::c_int as usize])
            .wrapping_add(0x4881d05 as uint32_t),
    );
    B = B << 23 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = (A as ::core::ffi::c_uint).wrapping_add(
        (B ^ C ^ D)
            .wrapping_add(correct_words[9 as ::core::ffi::c_int as usize])
            .wrapping_add(0xd9d4d039 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    A = A << 4 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (A ^ B ^ C)
            .wrapping_add(correct_words[12 as ::core::ffi::c_int as usize])
            .wrapping_add(0xe6db99e5 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 11 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = C.wrapping_add(
        (D ^ A ^ B)
            .wrapping_add(correct_words[15 as ::core::ffi::c_int as usize])
            .wrapping_add(0x1fa27cf8 as uint32_t),
    );
    C = C << 16 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 16 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (C ^ D ^ A)
            .wrapping_add(correct_words[2 as ::core::ffi::c_int as usize])
            .wrapping_add(0xc4ac5665 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 23 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 23 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = (A as ::core::ffi::c_uint).wrapping_add(
        (C ^ (B | !D))
            .wrapping_add(correct_words[0 as ::core::ffi::c_int as usize])
            .wrapping_add(0xf4292244 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    A = A << 6 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = D.wrapping_add(
        (B ^ (A | !C))
            .wrapping_add(correct_words[7 as ::core::ffi::c_int as usize])
            .wrapping_add(0x432aff97 as uint32_t),
    );
    D = D << 10 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = (C as ::core::ffi::c_uint).wrapping_add(
        (A ^ (D | !B))
            .wrapping_add(correct_words[14 as ::core::ffi::c_int as usize])
            .wrapping_add(0xab9423a7 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    C = C << 15 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (D ^ (C | !A))
            .wrapping_add(correct_words[5 as ::core::ffi::c_int as usize])
            .wrapping_add(0xfc93a039 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 21 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = A.wrapping_add(
        (C ^ (B | !D))
            .wrapping_add(correct_words[12 as ::core::ffi::c_int as usize])
            .wrapping_add(0x655b59c3 as uint32_t),
    );
    A = A << 6 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (B ^ (A | !C))
            .wrapping_add(correct_words[3 as ::core::ffi::c_int as usize])
            .wrapping_add(0x8f0ccc92 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 10 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = (C as ::core::ffi::c_uint).wrapping_add(
        (A ^ (D | !B))
            .wrapping_add(correct_words[10 as ::core::ffi::c_int as usize])
            .wrapping_add(0xffeff47d as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    C = C << 15 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (D ^ (C | !A))
            .wrapping_add(correct_words[1 as ::core::ffi::c_int as usize])
            .wrapping_add(0x85845dd1 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 21 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = A.wrapping_add(
        (C ^ (B | !D))
            .wrapping_add(correct_words[8 as ::core::ffi::c_int as usize])
            .wrapping_add(0x6fa87e4f as uint32_t),
    );
    A = A << 6 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (B ^ (A | !C))
            .wrapping_add(correct_words[15 as ::core::ffi::c_int as usize])
            .wrapping_add(0xfe2ce6e0 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 10 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = (C as ::core::ffi::c_uint).wrapping_add(
        (A ^ (D | !B))
            .wrapping_add(correct_words[6 as ::core::ffi::c_int as usize])
            .wrapping_add(0xa3014314 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    C = C << 15 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = B.wrapping_add(
        (D ^ (C | !A))
            .wrapping_add(correct_words[13 as ::core::ffi::c_int as usize])
            .wrapping_add(0x4e0811a1 as uint32_t),
    );
    B = B << 21 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    A = (A as ::core::ffi::c_uint).wrapping_add(
        (C ^ (B | !D))
            .wrapping_add(correct_words[4 as ::core::ffi::c_int as usize])
            .wrapping_add(0xf7537e82 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    A = A << 6 as ::core::ffi::c_int | A >> 32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int;
    A = A.wrapping_add(B);
    D = (D as ::core::ffi::c_uint).wrapping_add(
        (B ^ (A | !C))
            .wrapping_add(correct_words[11 as ::core::ffi::c_int as usize])
            .wrapping_add(0xbd3af235 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    D = D << 10 as ::core::ffi::c_int | D >> 32 as ::core::ffi::c_int - 10 as ::core::ffi::c_int;
    D = D.wrapping_add(A);
    C = C.wrapping_add(
        (A ^ (D | !B))
            .wrapping_add(correct_words[2 as ::core::ffi::c_int as usize])
            .wrapping_add(0x2ad7d2bb as uint32_t),
    );
    C = C << 15 as ::core::ffi::c_int | C >> 32 as ::core::ffi::c_int - 15 as ::core::ffi::c_int;
    C = C.wrapping_add(D);
    B = (B as ::core::ffi::c_uint).wrapping_add(
        (D ^ (C | !A))
            .wrapping_add(correct_words[9 as ::core::ffi::c_int as usize])
            .wrapping_add(0xeb86d391 as uint32_t) as ::core::ffi::c_uint,
    ) as uint32_t as uint32_t;
    B = B << 21 as ::core::ffi::c_int | B >> 32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int;
    B = B.wrapping_add(C);
    (*ctx).A = (*ctx).A.wrapping_add(A);
    (*ctx).B = (*ctx).B.wrapping_add(B);
    (*ctx).C = (*ctx).C.wrapping_add(C);
    (*ctx).D = (*ctx).D.wrapping_add(D);
}
#[unsafe(export_name = "__pdf_impl_MD5_write")]
pub unsafe extern "C-unwind" fn MD5_write(
    mut hd: *mut MD5_CONTEXT,
    mut inbuf: *const ::core::ffi::c_uchar,
    mut inlen: ::core::ffi::c_uint,
) {
    if (*hd).count == 64 as ::core::ffi::c_int {
        transform(hd, &raw mut (*hd).buf as *mut ::core::ffi::c_uchar);
        _gcry_burn_stack((80 as usize).wrapping_add(
            (6 as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize),
        ) as ::core::ffi::c_int);
        (*hd).count = 0 as ::core::ffi::c_int;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1);
    }
    if inbuf.is_null() {
        return;
    }
    if (*hd).count != 0 {
        while inlen != 0 && (*hd).count < 64 as ::core::ffi::c_int {
            let fresh0 = inbuf;
            inbuf = inbuf.offset(1);
            let fresh1 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh1 as usize] = *fresh0;
            inlen = inlen.wrapping_sub(1);
        }
        MD5_write(
            hd,
            ::core::ptr::null::<::core::ffi::c_uchar>(),
            0 as ::core::ffi::c_uint,
        );
        if inlen == 0 {
            return;
        }
    }
    _gcry_burn_stack((80 as usize).wrapping_add(
        (6 as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize),
    ) as ::core::ffi::c_int);
    while inlen >= 64 as ::core::ffi::c_uint {
        transform(hd, inbuf);
        (*hd).count = 0 as ::core::ffi::c_int;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1);
        inlen = inlen.wrapping_sub(64 as ::core::ffi::c_uint);
        inbuf = inbuf.offset(64 as ::core::ffi::c_int as isize);
    }
    while inlen != 0 && (*hd).count < 64 as ::core::ffi::c_int {
        let fresh2 = inbuf;
        inbuf = inbuf.offset(1);
        let fresh3 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh3 as usize] = *fresh2;
        inlen = inlen.wrapping_sub(1);
    }
}
#[unsafe(export_name = "__pdf_impl_MD5_final")]
pub unsafe extern "C-unwind" fn MD5_final(
    mut outbuf: *mut ::core::ffi::c_uchar,
    mut hd: *mut MD5_CONTEXT,
) {
    let mut t: uint32_t = 0;
    let mut msb: uint32_t = 0;
    let mut lsb: uint32_t = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    MD5_write(
        hd,
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        0 as ::core::ffi::c_uint,
    );
    t = (*hd).nblocks as uint32_t;
    lsb = t << 6 as ::core::ffi::c_int;
    msb = t >> 26 as ::core::ffi::c_int;
    t = lsb;
    lsb = lsb.wrapping_add((*hd).count as uint32_t);
    if lsb < t {
        msb = msb.wrapping_add(1);
    }
    t = lsb;
    lsb <<= 3 as ::core::ffi::c_int;
    msb <<= 3 as ::core::ffi::c_int;
    msb |= t >> 29 as ::core::ffi::c_int;
    if (*hd).count < 56 as ::core::ffi::c_int {
        let fresh20 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh20 as usize] = 0x80 as ::core::ffi::c_uchar;
        while (*hd).count < 56 as ::core::ffi::c_int {
            let fresh21 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh21 as usize] = 0 as ::core::ffi::c_uchar;
        }
    } else {
        let fresh22 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh22 as usize] = 0x80 as ::core::ffi::c_uchar;
        while (*hd).count < 64 as ::core::ffi::c_int {
            let fresh23 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh23 as usize] = 0 as ::core::ffi::c_uchar;
        }
        MD5_write(
            hd,
            ::core::ptr::null::<::core::ffi::c_uchar>(),
            0 as ::core::ffi::c_uint,
        );
        memset(
            &raw mut (*hd).buf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            56 as size_t,
        );
    }
    (*hd).buf[56 as ::core::ffi::c_int as usize] = (lsb & 0xff as uint32_t) as ::core::ffi::c_uchar;
    (*hd).buf[57 as ::core::ffi::c_int as usize] =
        (lsb >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_uchar;
    (*hd).buf[58 as ::core::ffi::c_int as usize] =
        (lsb >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_uchar;
    (*hd).buf[59 as ::core::ffi::c_int as usize] =
        (lsb >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_uchar;
    (*hd).buf[60 as ::core::ffi::c_int as usize] = (msb & 0xff as uint32_t) as ::core::ffi::c_uchar;
    (*hd).buf[61 as ::core::ffi::c_int as usize] =
        (msb >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_uchar;
    (*hd).buf[62 as ::core::ffi::c_int as usize] =
        (msb >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_uchar;
    (*hd).buf[63 as ::core::ffi::c_int as usize] =
        (msb >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_uchar;
    transform(hd, &raw mut (*hd).buf as *mut ::core::ffi::c_uchar);
    _gcry_burn_stack((80 as usize).wrapping_add(
        (6 as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize),
    ) as ::core::ffi::c_int);
    p = outbuf;
    (p as *mut uint32_t).write_unaligned(((*hd).A).to_le());
    p = p.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
    (p as *mut uint32_t).write_unaligned(((*hd).B).to_le());
    p = p.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
    (p as *mut uint32_t).write_unaligned(((*hd).C).to_le());
    p = p.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
    (p as *mut uint32_t).write_unaligned(((*hd).D).to_le());
    p = p.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
}
unsafe extern "C-unwind" fn _gcry_bswap32(mut x: uint32_t) -> uint32_t {
    x.to_be()
}
unsafe extern "C-unwind" fn _gcry_bswap64(mut x: uint64_t) -> uint64_t {
    x.to_be()
}
unsafe extern "C-unwind" fn buf_get_be32(mut _buf: *const ::core::ffi::c_void) -> uint32_t {
    let mut in_0: *const uint8_t = _buf as *const uint8_t;
    return (*in_0.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*in_0.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int
        | (*in_0.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *in_0.offset(3 as ::core::ffi::c_int as isize) as uint32_t;
}
unsafe extern "C-unwind" fn buf_put_be32(mut _buf: *mut ::core::ffi::c_void, mut val: uint32_t) {
    let mut out: *mut uint8_t = _buf as *mut uint8_t;
    *out.offset(0 as ::core::ffi::c_int as isize) = (val >> 24 as ::core::ffi::c_int) as uint8_t;
    *out.offset(1 as ::core::ffi::c_int as isize) = (val >> 16 as ::core::ffi::c_int) as uint8_t;
    *out.offset(2 as ::core::ffi::c_int as isize) = (val >> 8 as ::core::ffi::c_int) as uint8_t;
    *out.offset(3 as ::core::ffi::c_int as isize) = val as uint8_t;
}
unsafe extern "C-unwind" fn buf_get_be64(mut _buf: *const ::core::ffi::c_void) -> uint64_t {
    let mut in_0: *const uint8_t = _buf as *const uint8_t;
    return (*in_0.offset(0 as ::core::ffi::c_int as isize) as uint64_t)
        << 56 as ::core::ffi::c_int
        | (*in_0.offset(1 as ::core::ffi::c_int as isize) as uint64_t) << 48 as ::core::ffi::c_int
        | (*in_0.offset(2 as ::core::ffi::c_int as isize) as uint64_t) << 40 as ::core::ffi::c_int
        | (*in_0.offset(3 as ::core::ffi::c_int as isize) as uint64_t) << 32 as ::core::ffi::c_int
        | (*in_0.offset(4 as ::core::ffi::c_int as isize) as uint64_t) << 24 as ::core::ffi::c_int
        | (*in_0.offset(5 as ::core::ffi::c_int as isize) as uint64_t) << 16 as ::core::ffi::c_int
        | (*in_0.offset(6 as ::core::ffi::c_int as isize) as uint64_t) << 8 as ::core::ffi::c_int
        | *in_0.offset(7 as ::core::ffi::c_int as isize) as uint64_t;
}
unsafe extern "C-unwind" fn buf_put_be64(mut _buf: *mut ::core::ffi::c_void, mut val: uint64_t) {
    let mut out: *mut uint8_t = _buf as *mut uint8_t;
    *out.offset(0 as ::core::ffi::c_int as isize) = (val >> 56 as ::core::ffi::c_int) as uint8_t;
    *out.offset(1 as ::core::ffi::c_int as isize) = (val >> 48 as ::core::ffi::c_int) as uint8_t;
    *out.offset(2 as ::core::ffi::c_int as isize) = (val >> 40 as ::core::ffi::c_int) as uint8_t;
    *out.offset(3 as ::core::ffi::c_int as isize) = (val >> 32 as ::core::ffi::c_int) as uint8_t;
    *out.offset(4 as ::core::ffi::c_int as isize) = (val >> 24 as ::core::ffi::c_int) as uint8_t;
    *out.offset(5 as ::core::ffi::c_int as isize) = (val >> 16 as ::core::ffi::c_int) as uint8_t;
    *out.offset(6 as ::core::ffi::c_int as isize) = (val >> 8 as ::core::ffi::c_int) as uint8_t;
    *out.offset(7 as ::core::ffi::c_int as isize) = val as uint8_t;
}
#[unsafe(export_name = "__pdf_impl_SHA256_init")]
pub unsafe extern "C-unwind" fn SHA256_init(mut hd: *mut SHA256_CONTEXT) {
    (*hd).h0 = 0x6a09e667 as uint32_t;
    (*hd).h1 = 0xbb67ae85 as ::core::ffi::c_uint as uint32_t;
    (*hd).h2 = 0x3c6ef372 as uint32_t;
    (*hd).h3 = 0xa54ff53a as ::core::ffi::c_uint as uint32_t;
    (*hd).h4 = 0x510e527f as uint32_t;
    (*hd).h5 = 0x9b05688c as ::core::ffi::c_uint as uint32_t;
    (*hd).h6 = 0x1f83d9ab as uint32_t;
    (*hd).h7 = 0x5be0cd19 as uint32_t;
    (*hd).nblocks = 0 as size_t;
    (*hd).count = 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn _SHA256_transform(
    mut hd: *mut SHA256_CONTEXT,
    mut data: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    static mut K: [uint32_t; 64] = [
        0x428a2f98 as ::core::ffi::c_int as uint32_t,
        0x71374491 as ::core::ffi::c_int as uint32_t,
        0xb5c0fbcf as ::core::ffi::c_uint,
        0xe9b5dba5 as ::core::ffi::c_uint,
        0x3956c25b as ::core::ffi::c_int as uint32_t,
        0x59f111f1 as ::core::ffi::c_int as uint32_t,
        0x923f82a4 as ::core::ffi::c_uint,
        0xab1c5ed5 as ::core::ffi::c_uint,
        0xd807aa98 as ::core::ffi::c_uint,
        0x12835b01 as ::core::ffi::c_int as uint32_t,
        0x243185be as ::core::ffi::c_int as uint32_t,
        0x550c7dc3 as ::core::ffi::c_int as uint32_t,
        0x72be5d74 as ::core::ffi::c_int as uint32_t,
        0x80deb1fe as ::core::ffi::c_uint,
        0x9bdc06a7 as ::core::ffi::c_uint,
        0xc19bf174 as ::core::ffi::c_uint,
        0xe49b69c1 as ::core::ffi::c_uint,
        0xefbe4786 as ::core::ffi::c_uint,
        0xfc19dc6 as ::core::ffi::c_int as uint32_t,
        0x240ca1cc as ::core::ffi::c_int as uint32_t,
        0x2de92c6f as ::core::ffi::c_int as uint32_t,
        0x4a7484aa as ::core::ffi::c_int as uint32_t,
        0x5cb0a9dc as ::core::ffi::c_int as uint32_t,
        0x76f988da as ::core::ffi::c_int as uint32_t,
        0x983e5152 as ::core::ffi::c_uint,
        0xa831c66d as ::core::ffi::c_uint,
        0xb00327c8 as ::core::ffi::c_uint,
        0xbf597fc7 as ::core::ffi::c_uint,
        0xc6e00bf3 as ::core::ffi::c_uint,
        0xd5a79147 as ::core::ffi::c_uint,
        0x6ca6351 as ::core::ffi::c_int as uint32_t,
        0x14292967 as ::core::ffi::c_int as uint32_t,
        0x27b70a85 as ::core::ffi::c_int as uint32_t,
        0x2e1b2138 as ::core::ffi::c_int as uint32_t,
        0x4d2c6dfc as ::core::ffi::c_int as uint32_t,
        0x53380d13 as ::core::ffi::c_int as uint32_t,
        0x650a7354 as ::core::ffi::c_int as uint32_t,
        0x766a0abb as ::core::ffi::c_int as uint32_t,
        0x81c2c92e as ::core::ffi::c_uint,
        0x92722c85 as ::core::ffi::c_uint,
        0xa2bfe8a1 as ::core::ffi::c_uint,
        0xa81a664b as ::core::ffi::c_uint,
        0xc24b8b70 as ::core::ffi::c_uint,
        0xc76c51a3 as ::core::ffi::c_uint,
        0xd192e819 as ::core::ffi::c_uint,
        0xd6990624 as ::core::ffi::c_uint,
        0xf40e3585 as ::core::ffi::c_uint,
        0x106aa070 as ::core::ffi::c_int as uint32_t,
        0x19a4c116 as ::core::ffi::c_int as uint32_t,
        0x1e376c08 as ::core::ffi::c_int as uint32_t,
        0x2748774c as ::core::ffi::c_int as uint32_t,
        0x34b0bcb5 as ::core::ffi::c_int as uint32_t,
        0x391c0cb3 as ::core::ffi::c_int as uint32_t,
        0x4ed8aa4a as ::core::ffi::c_int as uint32_t,
        0x5b9cca4f as ::core::ffi::c_int as uint32_t,
        0x682e6ff3 as ::core::ffi::c_int as uint32_t,
        0x748f82ee as ::core::ffi::c_int as uint32_t,
        0x78a5636f as ::core::ffi::c_int as uint32_t,
        0x84c87814 as ::core::ffi::c_uint,
        0x8cc70208 as ::core::ffi::c_uint,
        0x90befffa as ::core::ffi::c_uint,
        0xa4506ceb as ::core::ffi::c_uint,
        0xbef9a3f7 as ::core::ffi::c_uint,
        0xc67178f2 as ::core::ffi::c_uint,
    ];
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut d: uint32_t = 0;
    let mut e: uint32_t = 0;
    let mut f: uint32_t = 0;
    let mut g: uint32_t = 0;
    let mut h: uint32_t = 0;
    let mut t1: uint32_t = 0;
    let mut t2: uint32_t = 0;
    let mut w: [uint32_t; 64] = [0; 64];
    let mut i: ::core::ffi::c_int = 0;
    a = (*hd).h0;
    b = (*hd).h1;
    c = (*hd).h2;
    d = (*hd).h3;
    e = (*hd).h4;
    f = (*hd).h5;
    g = (*hd).h6;
    h = (*hd).h7;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        w[i as usize] = buf_get_be32(
            data.offset((i * 4 as ::core::ffi::c_int) as isize) as *const ::core::ffi::c_void
        );
        i += 1;
    }
    while i < 64 as ::core::ffi::c_int {
        w[i as usize] = ((w[(i - 2 as ::core::ffi::c_int) as usize]
            >> (17 as ::core::ffi::c_int & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            | w[(i - 2 as ::core::ffi::c_int) as usize]
                << (32 as ::core::ffi::c_int - 17 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (w[(i - 2 as ::core::ffi::c_int) as usize]
                >> (19 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | w[(i - 2 as ::core::ffi::c_int) as usize]
                    << (32 as ::core::ffi::c_int - 19 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ w[(i - 2 as ::core::ffi::c_int) as usize] >> 10 as ::core::ffi::c_int)
            .wrapping_add(w[(i - 7 as ::core::ffi::c_int) as usize])
            .wrapping_add(
                (w[(i - 15 as ::core::ffi::c_int) as usize]
                    >> (7 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    | w[(i - 15 as ::core::ffi::c_int) as usize]
                        << (32 as ::core::ffi::c_int - 7 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (w[(i - 15 as ::core::ffi::c_int) as usize]
                        >> (18 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | w[(i - 15 as ::core::ffi::c_int) as usize]
                            << (32 as ::core::ffi::c_int - 18 as ::core::ffi::c_int
                                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ w[(i - 15 as ::core::ffi::c_int) as usize] >> 3 as ::core::ffi::c_int,
            )
            .wrapping_add(w[(i - 16 as ::core::ffi::c_int) as usize]);
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 64 as ::core::ffi::c_int {
        t1 = h
            .wrapping_add(
                (e >> (6 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    | e << (32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (e >> (11 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | e << (32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (e >> (25 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | e << (32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)),
            )
            .wrapping_add(g ^ e & (f ^ g))
            .wrapping_add(K[i as usize])
            .wrapping_add(w[i as usize]);
        t2 = ((a
            >> (2 as ::core::ffi::c_int & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            | a << (32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (a >> (13 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | a << (32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (a >> (22 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | a << (32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)))
            .wrapping_add(a & b | c & (a | b));
        d = d.wrapping_add(t1);
        h = t1.wrapping_add(t2);
        t1 = g
            .wrapping_add(
                (d >> (6 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    | d << (32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (d >> (11 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | d << (32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (d >> (25 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | d << (32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)),
            )
            .wrapping_add(f ^ d & (e ^ f))
            .wrapping_add(K[(i + 1 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[(i + 1 as ::core::ffi::c_int) as usize]);
        t2 = ((h
            >> (2 as ::core::ffi::c_int & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            | h << (32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (h >> (13 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | h << (32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (h >> (22 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | h << (32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)))
            .wrapping_add(h & a | b & (h | a));
        c = c.wrapping_add(t1);
        g = t1.wrapping_add(t2);
        t1 = f
            .wrapping_add(
                (c >> (6 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    | c << (32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (c >> (11 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | c << (32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (c >> (25 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | c << (32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)),
            )
            .wrapping_add(e ^ c & (d ^ e))
            .wrapping_add(K[(i + 2 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[(i + 2 as ::core::ffi::c_int) as usize]);
        t2 = ((g
            >> (2 as ::core::ffi::c_int & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            | g << (32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (g >> (13 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | g << (32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (g >> (22 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | g << (32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)))
            .wrapping_add(g & h | a & (g | h));
        b = b.wrapping_add(t1);
        f = t1.wrapping_add(t2);
        t1 = e
            .wrapping_add(
                (b >> (6 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    | b << (32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (b >> (11 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | b << (32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (b >> (25 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | b << (32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)),
            )
            .wrapping_add(d ^ b & (c ^ d))
            .wrapping_add(K[(i + 3 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[(i + 3 as ::core::ffi::c_int) as usize]);
        t2 = ((f
            >> (2 as ::core::ffi::c_int & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            | f << (32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (f >> (13 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | f << (32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (f >> (22 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | f << (32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)))
            .wrapping_add(f & g | h & (f | g));
        a = a.wrapping_add(t1);
        e = t1.wrapping_add(t2);
        t1 = d
            .wrapping_add(
                (a >> (6 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    | a << (32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (a >> (11 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | a << (32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (a >> (25 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | a << (32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)),
            )
            .wrapping_add(c ^ a & (b ^ c))
            .wrapping_add(K[(i + 4 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[(i + 4 as ::core::ffi::c_int) as usize]);
        t2 = ((e
            >> (2 as ::core::ffi::c_int & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            | e << (32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (e >> (13 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | e << (32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (e >> (22 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | e << (32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)))
            .wrapping_add(e & f | g & (e | f));
        h = h.wrapping_add(t1);
        d = t1.wrapping_add(t2);
        t1 = c
            .wrapping_add(
                (h >> (6 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    | h << (32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (h >> (11 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | h << (32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (h >> (25 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | h << (32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)),
            )
            .wrapping_add(b ^ h & (a ^ b))
            .wrapping_add(K[(i + 5 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[(i + 5 as ::core::ffi::c_int) as usize]);
        t2 = ((d
            >> (2 as ::core::ffi::c_int & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            | d << (32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (d >> (13 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | d << (32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (d >> (22 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | d << (32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)))
            .wrapping_add(d & e | f & (d | e));
        g = g.wrapping_add(t1);
        c = t1.wrapping_add(t2);
        t1 = b
            .wrapping_add(
                (g >> (6 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    | g << (32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (g >> (11 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | g << (32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (g >> (25 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | g << (32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)),
            )
            .wrapping_add(a ^ g & (h ^ a))
            .wrapping_add(K[(i + 6 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[(i + 6 as ::core::ffi::c_int) as usize]);
        t2 = ((c
            >> (2 as ::core::ffi::c_int & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            | c << (32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (c >> (13 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | c << (32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (c >> (22 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | c << (32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)))
            .wrapping_add(c & d | e & (c | d));
        f = f.wrapping_add(t1);
        b = t1.wrapping_add(t2);
        t1 = a
            .wrapping_add(
                (f >> (6 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    | f << (32 as ::core::ffi::c_int - 6 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (f >> (11 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | f << (32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                    ^ (f >> (25 as ::core::ffi::c_int
                        & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        | f << (32 as ::core::ffi::c_int - 25 as ::core::ffi::c_int
                            & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)),
            )
            .wrapping_add(h ^ f & (g ^ h))
            .wrapping_add(K[(i + 7 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[(i + 7 as ::core::ffi::c_int) as usize]);
        t2 = ((b
            >> (2 as ::core::ffi::c_int & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            | b << (32 as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (b >> (13 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | b << (32 as ::core::ffi::c_int - 13 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            ^ (b >> (22 as ::core::ffi::c_int
                & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                | b << (32 as ::core::ffi::c_int - 22 as ::core::ffi::c_int
                    & 32 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)))
            .wrapping_add(b & c | d & (b | c));
        e = e.wrapping_add(t1);
        a = t1.wrapping_add(t2);
        i += 8 as ::core::ffi::c_int;
    }
    (*hd).h0 = (*hd).h0.wrapping_add(a);
    (*hd).h1 = (*hd).h1.wrapping_add(b);
    (*hd).h2 = (*hd).h2.wrapping_add(c);
    (*hd).h3 = (*hd).h3.wrapping_add(d);
    (*hd).h4 = (*hd).h4.wrapping_add(e);
    (*hd).h5 = (*hd).h5.wrapping_add(f);
    (*hd).h6 = (*hd).h6.wrapping_add(g);
    (*hd).h7 = (*hd).h7.wrapping_add(h);
    return (74 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 32 as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
}
#[unsafe(export_name = "__pdf_impl_SHA256_write")]
pub unsafe extern "C-unwind" fn SHA256_write(
    mut hd: *mut SHA256_CONTEXT,
    mut inbuf: *const ::core::ffi::c_uchar,
    mut inlen: ::core::ffi::c_uint,
) {
    let mut stack_burn: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if (*hd).count == 64 as ::core::ffi::c_int {
        stack_burn = _SHA256_transform(hd, &raw mut (*hd).buf as *mut ::core::ffi::c_uchar);
        _gcry_burn_stack(stack_burn as ::core::ffi::c_int);
        (*hd).count = 0 as ::core::ffi::c_int;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1);
    }
    if inbuf.is_null() {
        return;
    }
    if (*hd).count != 0 {
        while inlen != 0 && (*hd).count < 64 as ::core::ffi::c_int {
            let fresh24 = inbuf;
            inbuf = inbuf.offset(1);
            let fresh25 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh25 as usize] = *fresh24;
            inlen = inlen.wrapping_sub(1);
        }
        SHA256_write(
            hd,
            ::core::ptr::null::<::core::ffi::c_uchar>(),
            0 as ::core::ffi::c_uint,
        );
        if inlen == 0 {
            return;
        }
    }
    _gcry_burn_stack(stack_burn as ::core::ffi::c_int);
    while inlen >= 64 as ::core::ffi::c_uint {
        stack_burn = _SHA256_transform(hd, inbuf);
        (*hd).count = 0 as ::core::ffi::c_int;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1);
        inlen = inlen.wrapping_sub(64 as ::core::ffi::c_uint);
        inbuf = inbuf.offset(64 as ::core::ffi::c_int as isize);
    }
    while inlen != 0 && (*hd).count < 64 as ::core::ffi::c_int {
        let fresh26 = inbuf;
        inbuf = inbuf.offset(1);
        let fresh27 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh27 as usize] = *fresh26;
        inlen = inlen.wrapping_sub(1);
    }
}
#[unsafe(export_name = "__pdf_impl_SHA256_final")]
pub unsafe extern "C-unwind" fn SHA256_final(
    mut outbuf: *mut ::core::ffi::c_uchar,
    mut hd: *mut SHA256_CONTEXT,
) {
    let mut t: uint32_t = 0;
    let mut msb: uint32_t = 0;
    let mut lsb: uint32_t = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut burn: ::core::ffi::c_uint = 0;
    SHA256_write(
        hd,
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        0 as ::core::ffi::c_uint,
    );
    t = (*hd).nblocks as uint32_t;
    lsb = t << 6 as ::core::ffi::c_int;
    msb = t >> 26 as ::core::ffi::c_int;
    t = lsb;
    lsb = lsb.wrapping_add((*hd).count as uint32_t);
    if lsb < t {
        msb = msb.wrapping_add(1);
    }
    t = lsb;
    lsb <<= 3 as ::core::ffi::c_int;
    msb <<= 3 as ::core::ffi::c_int;
    msb |= t >> 29 as ::core::ffi::c_int;
    if (*hd).count < 56 as ::core::ffi::c_int {
        let fresh28 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh28 as usize] = 0x80 as ::core::ffi::c_uchar;
        while (*hd).count < 56 as ::core::ffi::c_int {
            let fresh29 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh29 as usize] = 0 as ::core::ffi::c_uchar;
        }
    } else {
        let fresh30 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh30 as usize] = 0x80 as ::core::ffi::c_uchar;
        while (*hd).count < 64 as ::core::ffi::c_int {
            let fresh31 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh31 as usize] = 0 as ::core::ffi::c_uchar;
        }
        SHA256_write(
            hd,
            ::core::ptr::null::<::core::ffi::c_uchar>(),
            0 as ::core::ffi::c_uint,
        );
        memset(
            &raw mut (*hd).buf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            56 as size_t,
        );
    }
    buf_put_be32(
        (&raw mut (*hd).buf as *mut ::core::ffi::c_uchar).offset(56 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        msb,
    );
    buf_put_be32(
        (&raw mut (*hd).buf as *mut ::core::ffi::c_uchar).offset(60 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        lsb,
    );
    burn = _SHA256_transform(hd, &raw mut (*hd).buf as *mut ::core::ffi::c_uchar);
    _gcry_burn_stack(burn as ::core::ffi::c_int);
    p = outbuf;
    (p as *mut uint32_t).write_unaligned(_gcry_bswap32((*hd).h0));
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (p as *mut uint32_t).write_unaligned(_gcry_bswap32((*hd).h1));
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (p as *mut uint32_t).write_unaligned(_gcry_bswap32((*hd).h2));
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (p as *mut uint32_t).write_unaligned(_gcry_bswap32((*hd).h3));
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (p as *mut uint32_t).write_unaligned(_gcry_bswap32((*hd).h4));
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (p as *mut uint32_t).write_unaligned(_gcry_bswap32((*hd).h5));
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (p as *mut uint32_t).write_unaligned(_gcry_bswap32((*hd).h6));
    p = p.offset(4 as ::core::ffi::c_int as isize);
    (p as *mut uint32_t).write_unaligned(_gcry_bswap32((*hd).h7));
    p = p.offset(4 as ::core::ffi::c_int as isize);
}
#[unsafe(export_name = "__pdf_impl_SHA512_init")]
pub unsafe extern "C-unwind" fn SHA512_init(mut ctx: *mut SHA512_CONTEXT) {
    let mut hd: *mut SHA512_STATE = &raw mut (*ctx).state;
    (*hd).h0 = 0x6a09e667f3bcc908 as uint64_t;
    (*hd).h1 = 0xbb67ae8584caa73b as uint64_t;
    (*hd).h2 = 0x3c6ef372fe94f82b as uint64_t;
    (*hd).h3 = 0xa54ff53a5f1d36f1 as uint64_t;
    (*hd).h4 = 0x510e527fade682d1 as uint64_t;
    (*hd).h5 = 0x9b05688c2b3e6c1f as uint64_t;
    (*hd).h6 = 0x1f83d9abfb41bd6b as uint64_t;
    (*hd).h7 = 0x5be0cd19137e2179 as uint64_t;
    (*ctx).nblocks = 0 as size_t;
    (*ctx).count = 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_SHA384_init")]
pub unsafe extern "C-unwind" fn SHA384_init(mut ctx: *mut SHA512_CONTEXT) {
    let mut hd: *mut SHA512_STATE = &raw mut (*ctx).state;
    (*hd).h0 = 0xcbbb9d5dc1059ed8 as uint64_t;
    (*hd).h1 = 0x629a292a367cd507 as uint64_t;
    (*hd).h2 = 0x9159015a3070dd17 as uint64_t;
    (*hd).h3 = 0x152fecd8f70e5939 as uint64_t;
    (*hd).h4 = 0x67332667ffc00b31 as uint64_t;
    (*hd).h5 = 0x8eb44a8768581511 as uint64_t;
    (*hd).h6 = 0xdb0c2e0d64f98fa7 as uint64_t;
    (*hd).h7 = 0x47b5481dbefa4fa4 as uint64_t;
    (*ctx).nblocks = 0 as size_t;
    (*ctx).count = 0 as ::core::ffi::c_int;
}
static mut k: [uint64_t; 80] = [
    0x428a2f98d728ae22 as ::core::ffi::c_ulong as uint64_t,
    0x7137449123ef65cd as ::core::ffi::c_ulong as uint64_t,
    0xb5c0fbcfec4d3b2f as ::core::ffi::c_ulong as uint64_t,
    0xe9b5dba58189dbbc as ::core::ffi::c_ulong as uint64_t,
    0x3956c25bf348b538 as ::core::ffi::c_ulong as uint64_t,
    0x59f111f1b605d019 as ::core::ffi::c_ulong as uint64_t,
    0x923f82a4af194f9b as ::core::ffi::c_ulong as uint64_t,
    0xab1c5ed5da6d8118 as ::core::ffi::c_ulong as uint64_t,
    0xd807aa98a3030242 as ::core::ffi::c_ulong as uint64_t,
    0x12835b0145706fbe as ::core::ffi::c_ulong as uint64_t,
    0x243185be4ee4b28c as ::core::ffi::c_ulong as uint64_t,
    0x550c7dc3d5ffb4e2 as ::core::ffi::c_ulong as uint64_t,
    0x72be5d74f27b896f as ::core::ffi::c_ulong as uint64_t,
    0x80deb1fe3b1696b1 as ::core::ffi::c_ulong as uint64_t,
    0x9bdc06a725c71235 as ::core::ffi::c_ulong as uint64_t,
    0xc19bf174cf692694 as ::core::ffi::c_ulong as uint64_t,
    0xe49b69c19ef14ad2 as ::core::ffi::c_ulong as uint64_t,
    0xefbe4786384f25e3 as ::core::ffi::c_ulong as uint64_t,
    0xfc19dc68b8cd5b5 as ::core::ffi::c_ulong as uint64_t,
    0x240ca1cc77ac9c65 as ::core::ffi::c_ulong as uint64_t,
    0x2de92c6f592b0275 as ::core::ffi::c_ulong as uint64_t,
    0x4a7484aa6ea6e483 as ::core::ffi::c_ulong as uint64_t,
    0x5cb0a9dcbd41fbd4 as ::core::ffi::c_ulong as uint64_t,
    0x76f988da831153b5 as ::core::ffi::c_ulong as uint64_t,
    0x983e5152ee66dfab as ::core::ffi::c_ulong as uint64_t,
    0xa831c66d2db43210 as ::core::ffi::c_ulong as uint64_t,
    0xb00327c898fb213f as ::core::ffi::c_ulong as uint64_t,
    0xbf597fc7beef0ee4 as ::core::ffi::c_ulong as uint64_t,
    0xc6e00bf33da88fc2 as ::core::ffi::c_ulong as uint64_t,
    0xd5a79147930aa725 as ::core::ffi::c_ulong as uint64_t,
    0x6ca6351e003826f as ::core::ffi::c_ulong as uint64_t,
    0x142929670a0e6e70 as ::core::ffi::c_ulong as uint64_t,
    0x27b70a8546d22ffc as ::core::ffi::c_ulong as uint64_t,
    0x2e1b21385c26c926 as ::core::ffi::c_ulong as uint64_t,
    0x4d2c6dfc5ac42aed as ::core::ffi::c_ulong as uint64_t,
    0x53380d139d95b3df as ::core::ffi::c_ulong as uint64_t,
    0x650a73548baf63de as ::core::ffi::c_ulong as uint64_t,
    0x766a0abb3c77b2a8 as ::core::ffi::c_ulong as uint64_t,
    0x81c2c92e47edaee6 as ::core::ffi::c_ulong as uint64_t,
    0x92722c851482353b as ::core::ffi::c_ulong as uint64_t,
    0xa2bfe8a14cf10364 as ::core::ffi::c_ulong as uint64_t,
    0xa81a664bbc423001 as ::core::ffi::c_ulong as uint64_t,
    0xc24b8b70d0f89791 as ::core::ffi::c_ulong as uint64_t,
    0xc76c51a30654be30 as ::core::ffi::c_ulong as uint64_t,
    0xd192e819d6ef5218 as ::core::ffi::c_ulong as uint64_t,
    0xd69906245565a910 as ::core::ffi::c_ulong as uint64_t,
    0xf40e35855771202a as ::core::ffi::c_ulong as uint64_t,
    0x106aa07032bbd1b8 as ::core::ffi::c_ulong as uint64_t,
    0x19a4c116b8d2d0c8 as ::core::ffi::c_ulong as uint64_t,
    0x1e376c085141ab53 as ::core::ffi::c_ulong as uint64_t,
    0x2748774cdf8eeb99 as ::core::ffi::c_ulong as uint64_t,
    0x34b0bcb5e19b48a8 as ::core::ffi::c_ulong as uint64_t,
    0x391c0cb3c5c95a63 as ::core::ffi::c_ulong as uint64_t,
    0x4ed8aa4ae3418acb as ::core::ffi::c_ulong as uint64_t,
    0x5b9cca4f7763e373 as ::core::ffi::c_ulong as uint64_t,
    0x682e6ff3d6b2b8a3 as ::core::ffi::c_ulong as uint64_t,
    0x748f82ee5defb2fc as ::core::ffi::c_ulong as uint64_t,
    0x78a5636f43172f60 as ::core::ffi::c_ulong as uint64_t,
    0x84c87814a1f0ab72 as ::core::ffi::c_ulong as uint64_t,
    0x8cc702081a6439ec as ::core::ffi::c_ulong as uint64_t,
    0x90befffa23631e28 as ::core::ffi::c_ulong as uint64_t,
    0xa4506cebde82bde9 as ::core::ffi::c_ulong as uint64_t,
    0xbef9a3f7b2c67915 as ::core::ffi::c_ulong as uint64_t,
    0xc67178f2e372532b as ::core::ffi::c_ulong as uint64_t,
    0xca273eceea26619c as ::core::ffi::c_ulong as uint64_t,
    0xd186b8c721c0c207 as ::core::ffi::c_ulong as uint64_t,
    0xeada7dd6cde0eb1e as ::core::ffi::c_ulong as uint64_t,
    0xf57d4f7fee6ed178 as ::core::ffi::c_ulong as uint64_t,
    0x6f067aa72176fba as ::core::ffi::c_ulong as uint64_t,
    0xa637dc5a2c898a6 as ::core::ffi::c_ulong as uint64_t,
    0x113f9804bef90dae as ::core::ffi::c_ulong as uint64_t,
    0x1b710b35131c471b as ::core::ffi::c_ulong as uint64_t,
    0x28db77f523047d84 as ::core::ffi::c_ulong as uint64_t,
    0x32caab7b40c72493 as ::core::ffi::c_ulong as uint64_t,
    0x3c9ebe0a15c9bebc as ::core::ffi::c_ulong as uint64_t,
    0x431d67c49c100d4c as ::core::ffi::c_ulong as uint64_t,
    0x4cc5d4becb3e42b6 as ::core::ffi::c_ulong as uint64_t,
    0x597f299cfc657e2a as ::core::ffi::c_ulong as uint64_t,
    0x5fcb6fab3ad6faec as ::core::ffi::c_ulong as uint64_t,
    0x6c44198c4a475817 as ::core::ffi::c_ulong as uint64_t,
];
unsafe extern "C-unwind" fn __transform(
    mut hd: *mut SHA512_STATE,
    mut data: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    let mut a: uint64_t = 0;
    let mut b: uint64_t = 0;
    let mut c: uint64_t = 0;
    let mut d: uint64_t = 0;
    let mut e: uint64_t = 0;
    let mut f: uint64_t = 0;
    let mut g: uint64_t = 0;
    let mut h: uint64_t = 0;
    let mut w: [uint64_t; 16] = [0; 16];
    let mut t: ::core::ffi::c_int = 0;
    a = (*hd).h0;
    b = (*hd).h1;
    c = (*hd).h2;
    d = (*hd).h3;
    e = (*hd).h4;
    f = (*hd).h5;
    g = (*hd).h6;
    h = (*hd).h7;
    t = 0 as ::core::ffi::c_int;
    while t < 16 as ::core::ffi::c_int {
        w[t as usize] = buf_get_be64(
            data.offset((t * 8 as ::core::ffi::c_int) as isize) as *const ::core::ffi::c_void
        );
        t += 1;
    }
    t = 0 as ::core::ffi::c_int;
    while t < 80 as ::core::ffi::c_int - 16 as ::core::ffi::c_int {
        let mut t1: uint64_t = 0;
        let mut t2: uint64_t = 0;
        t1 = h
            .wrapping_add(
                (e >> 14 as ::core::ffi::c_int
                    | e << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (e >> 18 as ::core::ffi::c_int
                        | e << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (e >> 41 as ::core::ffi::c_int
                        | e << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(k[t as usize])
            .wrapping_add(w[0 as ::core::ffi::c_int as usize]);
        w[0 as ::core::ffi::c_int as usize] = w[0 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[14 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[14 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[14 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[14 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[14 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[9 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[1 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[1 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[1 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[1 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[1 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((a >> 28 as ::core::ffi::c_int
            | a << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (a >> 34 as ::core::ffi::c_int
                | a << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (a >> 39 as ::core::ffi::c_int
                | a << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(a & b ^ a & c ^ b & c);
        d = d.wrapping_add(t1);
        h = t1.wrapping_add(t2);
        t1 = g
            .wrapping_add(
                (d >> 14 as ::core::ffi::c_int
                    | d << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (d >> 18 as ::core::ffi::c_int
                        | d << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (d >> 41 as ::core::ffi::c_int
                        | d << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(d & e ^ !d & f)
            .wrapping_add(k[(t + 1 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[1 as ::core::ffi::c_int as usize]);
        w[1 as ::core::ffi::c_int as usize] = w[1 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[15 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[15 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[15 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[15 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[15 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[10 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[2 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[2 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[2 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[2 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[2 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((h >> 28 as ::core::ffi::c_int
            | h << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (h >> 34 as ::core::ffi::c_int
                | h << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (h >> 39 as ::core::ffi::c_int
                | h << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(h & a ^ h & b ^ a & b);
        c = c.wrapping_add(t1);
        g = t1.wrapping_add(t2);
        t1 = f
            .wrapping_add(
                (c >> 14 as ::core::ffi::c_int
                    | c << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (c >> 18 as ::core::ffi::c_int
                        | c << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (c >> 41 as ::core::ffi::c_int
                        | c << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(c & d ^ !c & e)
            .wrapping_add(k[(t + 2 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[2 as ::core::ffi::c_int as usize]);
        w[2 as ::core::ffi::c_int as usize] = w[2 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[0 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[0 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[0 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[0 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[0 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[11 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[3 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[3 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[3 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[3 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[3 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((g >> 28 as ::core::ffi::c_int
            | g << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (g >> 34 as ::core::ffi::c_int
                | g << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (g >> 39 as ::core::ffi::c_int
                | g << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(g & h ^ g & a ^ h & a);
        b = b.wrapping_add(t1);
        f = t1.wrapping_add(t2);
        t1 = e
            .wrapping_add(
                (b >> 14 as ::core::ffi::c_int
                    | b << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (b >> 18 as ::core::ffi::c_int
                        | b << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (b >> 41 as ::core::ffi::c_int
                        | b << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(b & c ^ !b & d)
            .wrapping_add(k[(t + 3 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[3 as ::core::ffi::c_int as usize]);
        w[3 as ::core::ffi::c_int as usize] = w[3 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[1 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[1 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[1 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[1 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[1 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[12 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[4 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[4 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[4 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[4 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[4 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((f >> 28 as ::core::ffi::c_int
            | f << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (f >> 34 as ::core::ffi::c_int
                | f << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (f >> 39 as ::core::ffi::c_int
                | f << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(f & g ^ f & h ^ g & h);
        a = a.wrapping_add(t1);
        e = t1.wrapping_add(t2);
        t1 = d
            .wrapping_add(
                (a >> 14 as ::core::ffi::c_int
                    | a << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (a >> 18 as ::core::ffi::c_int
                        | a << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (a >> 41 as ::core::ffi::c_int
                        | a << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(a & b ^ !a & c)
            .wrapping_add(k[(t + 4 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[4 as ::core::ffi::c_int as usize]);
        w[4 as ::core::ffi::c_int as usize] = w[4 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[2 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[2 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[2 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[2 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[2 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[13 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[5 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[5 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[5 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[5 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[5 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((e >> 28 as ::core::ffi::c_int
            | e << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (e >> 34 as ::core::ffi::c_int
                | e << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (e >> 39 as ::core::ffi::c_int
                | e << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(e & f ^ e & g ^ f & g);
        h = h.wrapping_add(t1);
        d = t1.wrapping_add(t2);
        t1 = c
            .wrapping_add(
                (h >> 14 as ::core::ffi::c_int
                    | h << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (h >> 18 as ::core::ffi::c_int
                        | h << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (h >> 41 as ::core::ffi::c_int
                        | h << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(h & a ^ !h & b)
            .wrapping_add(k[(t + 5 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[5 as ::core::ffi::c_int as usize]);
        w[5 as ::core::ffi::c_int as usize] = w[5 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[3 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[3 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[3 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[3 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[3 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[14 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[6 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[6 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[6 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[6 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[6 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((d >> 28 as ::core::ffi::c_int
            | d << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (d >> 34 as ::core::ffi::c_int
                | d << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (d >> 39 as ::core::ffi::c_int
                | d << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(d & e ^ d & f ^ e & f);
        g = g.wrapping_add(t1);
        c = t1.wrapping_add(t2);
        t1 = b
            .wrapping_add(
                (g >> 14 as ::core::ffi::c_int
                    | g << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (g >> 18 as ::core::ffi::c_int
                        | g << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (g >> 41 as ::core::ffi::c_int
                        | g << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(g & h ^ !g & a)
            .wrapping_add(k[(t + 6 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[6 as ::core::ffi::c_int as usize]);
        w[6 as ::core::ffi::c_int as usize] = w[6 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[4 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[4 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[4 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[4 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[4 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[15 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[7 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[7 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[7 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[7 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[7 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((c >> 28 as ::core::ffi::c_int
            | c << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (c >> 34 as ::core::ffi::c_int
                | c << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (c >> 39 as ::core::ffi::c_int
                | c << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(c & d ^ c & e ^ d & e);
        f = f.wrapping_add(t1);
        b = t1.wrapping_add(t2);
        t1 = a
            .wrapping_add(
                (f >> 14 as ::core::ffi::c_int
                    | f << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (f >> 18 as ::core::ffi::c_int
                        | f << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (f >> 41 as ::core::ffi::c_int
                        | f << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(f & g ^ !f & h)
            .wrapping_add(k[(t + 7 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[7 as ::core::ffi::c_int as usize]);
        w[7 as ::core::ffi::c_int as usize] = w[7 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[5 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[5 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[5 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[5 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[5 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[0 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[8 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[8 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[8 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[8 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[8 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((b >> 28 as ::core::ffi::c_int
            | b << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (b >> 34 as ::core::ffi::c_int
                | b << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (b >> 39 as ::core::ffi::c_int
                | b << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(b & c ^ b & d ^ c & d);
        e = e.wrapping_add(t1);
        a = t1.wrapping_add(t2);
        t1 = h
            .wrapping_add(
                (e >> 14 as ::core::ffi::c_int
                    | e << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (e >> 18 as ::core::ffi::c_int
                        | e << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (e >> 41 as ::core::ffi::c_int
                        | e << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(k[(t + 8 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[8 as ::core::ffi::c_int as usize]);
        w[8 as ::core::ffi::c_int as usize] = w[8 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[6 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[6 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[6 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[6 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[6 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[1 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[9 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[9 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[9 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[9 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[9 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((a >> 28 as ::core::ffi::c_int
            | a << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (a >> 34 as ::core::ffi::c_int
                | a << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (a >> 39 as ::core::ffi::c_int
                | a << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(a & b ^ a & c ^ b & c);
        d = d.wrapping_add(t1);
        h = t1.wrapping_add(t2);
        t1 = g
            .wrapping_add(
                (d >> 14 as ::core::ffi::c_int
                    | d << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (d >> 18 as ::core::ffi::c_int
                        | d << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (d >> 41 as ::core::ffi::c_int
                        | d << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(d & e ^ !d & f)
            .wrapping_add(k[(t + 9 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[9 as ::core::ffi::c_int as usize]);
        w[9 as ::core::ffi::c_int as usize] = w[9 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[7 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[7 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[7 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[7 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[7 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[2 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[10 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[10 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[10 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[10 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[10 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((h >> 28 as ::core::ffi::c_int
            | h << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (h >> 34 as ::core::ffi::c_int
                | h << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (h >> 39 as ::core::ffi::c_int
                | h << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(h & a ^ h & b ^ a & b);
        c = c.wrapping_add(t1);
        g = t1.wrapping_add(t2);
        t1 = f
            .wrapping_add(
                (c >> 14 as ::core::ffi::c_int
                    | c << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (c >> 18 as ::core::ffi::c_int
                        | c << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (c >> 41 as ::core::ffi::c_int
                        | c << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(c & d ^ !c & e)
            .wrapping_add(k[(t + 10 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[10 as ::core::ffi::c_int as usize]);
        w[10 as ::core::ffi::c_int as usize] = w[10 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[8 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[8 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[8 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[8 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[8 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[3 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[11 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[11 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[11 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[11 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[11 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((g >> 28 as ::core::ffi::c_int
            | g << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (g >> 34 as ::core::ffi::c_int
                | g << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (g >> 39 as ::core::ffi::c_int
                | g << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(g & h ^ g & a ^ h & a);
        b = b.wrapping_add(t1);
        f = t1.wrapping_add(t2);
        t1 = e
            .wrapping_add(
                (b >> 14 as ::core::ffi::c_int
                    | b << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (b >> 18 as ::core::ffi::c_int
                        | b << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (b >> 41 as ::core::ffi::c_int
                        | b << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(b & c ^ !b & d)
            .wrapping_add(k[(t + 11 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[11 as ::core::ffi::c_int as usize]);
        w[11 as ::core::ffi::c_int as usize] = w[11 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[9 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[9 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[9 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[9 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[9 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[4 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[12 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[12 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[12 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[12 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[12 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((f >> 28 as ::core::ffi::c_int
            | f << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (f >> 34 as ::core::ffi::c_int
                | f << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (f >> 39 as ::core::ffi::c_int
                | f << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(f & g ^ f & h ^ g & h);
        a = a.wrapping_add(t1);
        e = t1.wrapping_add(t2);
        t1 = d
            .wrapping_add(
                (a >> 14 as ::core::ffi::c_int
                    | a << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (a >> 18 as ::core::ffi::c_int
                        | a << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (a >> 41 as ::core::ffi::c_int
                        | a << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(a & b ^ !a & c)
            .wrapping_add(k[(t + 12 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[12 as ::core::ffi::c_int as usize]);
        w[12 as ::core::ffi::c_int as usize] = w[12 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[10 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[10 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[10 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[10 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[10 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[5 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[13 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[13 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[13 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[13 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[13 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((e >> 28 as ::core::ffi::c_int
            | e << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (e >> 34 as ::core::ffi::c_int
                | e << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (e >> 39 as ::core::ffi::c_int
                | e << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(e & f ^ e & g ^ f & g);
        h = h.wrapping_add(t1);
        d = t1.wrapping_add(t2);
        t1 = c
            .wrapping_add(
                (h >> 14 as ::core::ffi::c_int
                    | h << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (h >> 18 as ::core::ffi::c_int
                        | h << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (h >> 41 as ::core::ffi::c_int
                        | h << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(h & a ^ !h & b)
            .wrapping_add(k[(t + 13 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[13 as ::core::ffi::c_int as usize]);
        w[13 as ::core::ffi::c_int as usize] = w[13 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[11 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[11 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[11 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[11 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[11 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[6 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[14 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[14 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[14 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[14 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[14 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((d >> 28 as ::core::ffi::c_int
            | d << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (d >> 34 as ::core::ffi::c_int
                | d << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (d >> 39 as ::core::ffi::c_int
                | d << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(d & e ^ d & f ^ e & f);
        g = g.wrapping_add(t1);
        c = t1.wrapping_add(t2);
        t1 = b
            .wrapping_add(
                (g >> 14 as ::core::ffi::c_int
                    | g << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (g >> 18 as ::core::ffi::c_int
                        | g << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (g >> 41 as ::core::ffi::c_int
                        | g << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(g & h ^ !g & a)
            .wrapping_add(k[(t + 14 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[14 as ::core::ffi::c_int as usize]);
        w[14 as ::core::ffi::c_int as usize] = w[14 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[12 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[12 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[12 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[12 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[12 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[7 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[15 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[15 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[15 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[15 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[15 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((c >> 28 as ::core::ffi::c_int
            | c << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (c >> 34 as ::core::ffi::c_int
                | c << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (c >> 39 as ::core::ffi::c_int
                | c << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(c & d ^ c & e ^ d & e);
        f = f.wrapping_add(t1);
        b = t1.wrapping_add(t2);
        t1 = a
            .wrapping_add(
                (f >> 14 as ::core::ffi::c_int
                    | f << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (f >> 18 as ::core::ffi::c_int
                        | f << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (f >> 41 as ::core::ffi::c_int
                        | f << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(f & g ^ !f & h)
            .wrapping_add(k[(t + 15 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[15 as ::core::ffi::c_int as usize]);
        w[15 as ::core::ffi::c_int as usize] = w[15 as ::core::ffi::c_int as usize].wrapping_add(
            ((w[13 as ::core::ffi::c_int as usize] >> 19 as ::core::ffi::c_int
                | w[13 as ::core::ffi::c_int as usize]
                    << 64 as ::core::ffi::c_int - 19 as ::core::ffi::c_int)
                ^ (w[13 as ::core::ffi::c_int as usize] >> 61 as ::core::ffi::c_int
                    | w[13 as ::core::ffi::c_int as usize]
                        << 64 as ::core::ffi::c_int - 61 as ::core::ffi::c_int)
                ^ w[13 as ::core::ffi::c_int as usize] >> 6 as ::core::ffi::c_int)
                .wrapping_add(w[8 as ::core::ffi::c_int as usize])
                .wrapping_add(
                    (w[0 as ::core::ffi::c_int as usize] >> 1 as ::core::ffi::c_int
                        | w[0 as ::core::ffi::c_int as usize]
                            << 64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        ^ (w[0 as ::core::ffi::c_int as usize] >> 8 as ::core::ffi::c_int
                            | w[0 as ::core::ffi::c_int as usize]
                                << 64 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        ^ w[0 as ::core::ffi::c_int as usize] >> 7 as ::core::ffi::c_int,
                ),
        );
        t2 = ((b >> 28 as ::core::ffi::c_int
            | b << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (b >> 34 as ::core::ffi::c_int
                | b << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (b >> 39 as ::core::ffi::c_int
                | b << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(b & c ^ b & d ^ c & d);
        e = e.wrapping_add(t1);
        a = t1.wrapping_add(t2);
        t += 16 as ::core::ffi::c_int;
    }
    while t < 80 as ::core::ffi::c_int {
        let mut t1_0: uint64_t = 0;
        let mut t2_0: uint64_t = 0;
        t1_0 = h
            .wrapping_add(
                (e >> 14 as ::core::ffi::c_int
                    | e << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (e >> 18 as ::core::ffi::c_int
                        | e << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (e >> 41 as ::core::ffi::c_int
                        | e << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(k[t as usize])
            .wrapping_add(w[0 as ::core::ffi::c_int as usize]);
        t2_0 = ((a >> 28 as ::core::ffi::c_int
            | a << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (a >> 34 as ::core::ffi::c_int
                | a << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (a >> 39 as ::core::ffi::c_int
                | a << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(a & b ^ a & c ^ b & c);
        d = d.wrapping_add(t1_0);
        h = t1_0.wrapping_add(t2_0);
        t1_0 = g
            .wrapping_add(
                (d >> 14 as ::core::ffi::c_int
                    | d << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (d >> 18 as ::core::ffi::c_int
                        | d << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (d >> 41 as ::core::ffi::c_int
                        | d << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(d & e ^ !d & f)
            .wrapping_add(k[(t + 1 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[1 as ::core::ffi::c_int as usize]);
        t2_0 = ((h >> 28 as ::core::ffi::c_int
            | h << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (h >> 34 as ::core::ffi::c_int
                | h << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (h >> 39 as ::core::ffi::c_int
                | h << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(h & a ^ h & b ^ a & b);
        c = c.wrapping_add(t1_0);
        g = t1_0.wrapping_add(t2_0);
        t1_0 = f
            .wrapping_add(
                (c >> 14 as ::core::ffi::c_int
                    | c << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (c >> 18 as ::core::ffi::c_int
                        | c << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (c >> 41 as ::core::ffi::c_int
                        | c << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(c & d ^ !c & e)
            .wrapping_add(k[(t + 2 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[2 as ::core::ffi::c_int as usize]);
        t2_0 = ((g >> 28 as ::core::ffi::c_int
            | g << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (g >> 34 as ::core::ffi::c_int
                | g << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (g >> 39 as ::core::ffi::c_int
                | g << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(g & h ^ g & a ^ h & a);
        b = b.wrapping_add(t1_0);
        f = t1_0.wrapping_add(t2_0);
        t1_0 = e
            .wrapping_add(
                (b >> 14 as ::core::ffi::c_int
                    | b << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (b >> 18 as ::core::ffi::c_int
                        | b << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (b >> 41 as ::core::ffi::c_int
                        | b << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(b & c ^ !b & d)
            .wrapping_add(k[(t + 3 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[3 as ::core::ffi::c_int as usize]);
        t2_0 = ((f >> 28 as ::core::ffi::c_int
            | f << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (f >> 34 as ::core::ffi::c_int
                | f << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (f >> 39 as ::core::ffi::c_int
                | f << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(f & g ^ f & h ^ g & h);
        a = a.wrapping_add(t1_0);
        e = t1_0.wrapping_add(t2_0);
        t1_0 = d
            .wrapping_add(
                (a >> 14 as ::core::ffi::c_int
                    | a << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (a >> 18 as ::core::ffi::c_int
                        | a << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (a >> 41 as ::core::ffi::c_int
                        | a << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(a & b ^ !a & c)
            .wrapping_add(k[(t + 4 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[4 as ::core::ffi::c_int as usize]);
        t2_0 = ((e >> 28 as ::core::ffi::c_int
            | e << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (e >> 34 as ::core::ffi::c_int
                | e << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (e >> 39 as ::core::ffi::c_int
                | e << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(e & f ^ e & g ^ f & g);
        h = h.wrapping_add(t1_0);
        d = t1_0.wrapping_add(t2_0);
        t1_0 = c
            .wrapping_add(
                (h >> 14 as ::core::ffi::c_int
                    | h << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (h >> 18 as ::core::ffi::c_int
                        | h << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (h >> 41 as ::core::ffi::c_int
                        | h << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(h & a ^ !h & b)
            .wrapping_add(k[(t + 5 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[5 as ::core::ffi::c_int as usize]);
        t2_0 = ((d >> 28 as ::core::ffi::c_int
            | d << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (d >> 34 as ::core::ffi::c_int
                | d << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (d >> 39 as ::core::ffi::c_int
                | d << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(d & e ^ d & f ^ e & f);
        g = g.wrapping_add(t1_0);
        c = t1_0.wrapping_add(t2_0);
        t1_0 = b
            .wrapping_add(
                (g >> 14 as ::core::ffi::c_int
                    | g << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (g >> 18 as ::core::ffi::c_int
                        | g << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (g >> 41 as ::core::ffi::c_int
                        | g << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(g & h ^ !g & a)
            .wrapping_add(k[(t + 6 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[6 as ::core::ffi::c_int as usize]);
        t2_0 = ((c >> 28 as ::core::ffi::c_int
            | c << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (c >> 34 as ::core::ffi::c_int
                | c << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (c >> 39 as ::core::ffi::c_int
                | c << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(c & d ^ c & e ^ d & e);
        f = f.wrapping_add(t1_0);
        b = t1_0.wrapping_add(t2_0);
        t1_0 = a
            .wrapping_add(
                (f >> 14 as ::core::ffi::c_int
                    | f << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (f >> 18 as ::core::ffi::c_int
                        | f << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (f >> 41 as ::core::ffi::c_int
                        | f << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(f & g ^ !f & h)
            .wrapping_add(k[(t + 7 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[7 as ::core::ffi::c_int as usize]);
        t2_0 = ((b >> 28 as ::core::ffi::c_int
            | b << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (b >> 34 as ::core::ffi::c_int
                | b << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (b >> 39 as ::core::ffi::c_int
                | b << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(b & c ^ b & d ^ c & d);
        e = e.wrapping_add(t1_0);
        a = t1_0.wrapping_add(t2_0);
        t1_0 = h
            .wrapping_add(
                (e >> 14 as ::core::ffi::c_int
                    | e << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (e >> 18 as ::core::ffi::c_int
                        | e << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (e >> 41 as ::core::ffi::c_int
                        | e << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(k[(t + 8 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[8 as ::core::ffi::c_int as usize]);
        t2_0 = ((a >> 28 as ::core::ffi::c_int
            | a << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (a >> 34 as ::core::ffi::c_int
                | a << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (a >> 39 as ::core::ffi::c_int
                | a << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(a & b ^ a & c ^ b & c);
        d = d.wrapping_add(t1_0);
        h = t1_0.wrapping_add(t2_0);
        t1_0 = g
            .wrapping_add(
                (d >> 14 as ::core::ffi::c_int
                    | d << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (d >> 18 as ::core::ffi::c_int
                        | d << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (d >> 41 as ::core::ffi::c_int
                        | d << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(d & e ^ !d & f)
            .wrapping_add(k[(t + 9 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[9 as ::core::ffi::c_int as usize]);
        t2_0 = ((h >> 28 as ::core::ffi::c_int
            | h << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (h >> 34 as ::core::ffi::c_int
                | h << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (h >> 39 as ::core::ffi::c_int
                | h << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(h & a ^ h & b ^ a & b);
        c = c.wrapping_add(t1_0);
        g = t1_0.wrapping_add(t2_0);
        t1_0 = f
            .wrapping_add(
                (c >> 14 as ::core::ffi::c_int
                    | c << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (c >> 18 as ::core::ffi::c_int
                        | c << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (c >> 41 as ::core::ffi::c_int
                        | c << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(c & d ^ !c & e)
            .wrapping_add(k[(t + 10 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[10 as ::core::ffi::c_int as usize]);
        t2_0 = ((g >> 28 as ::core::ffi::c_int
            | g << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (g >> 34 as ::core::ffi::c_int
                | g << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (g >> 39 as ::core::ffi::c_int
                | g << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(g & h ^ g & a ^ h & a);
        b = b.wrapping_add(t1_0);
        f = t1_0.wrapping_add(t2_0);
        t1_0 = e
            .wrapping_add(
                (b >> 14 as ::core::ffi::c_int
                    | b << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (b >> 18 as ::core::ffi::c_int
                        | b << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (b >> 41 as ::core::ffi::c_int
                        | b << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(b & c ^ !b & d)
            .wrapping_add(k[(t + 11 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[11 as ::core::ffi::c_int as usize]);
        t2_0 = ((f >> 28 as ::core::ffi::c_int
            | f << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (f >> 34 as ::core::ffi::c_int
                | f << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (f >> 39 as ::core::ffi::c_int
                | f << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(f & g ^ f & h ^ g & h);
        a = a.wrapping_add(t1_0);
        e = t1_0.wrapping_add(t2_0);
        t1_0 = d
            .wrapping_add(
                (a >> 14 as ::core::ffi::c_int
                    | a << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (a >> 18 as ::core::ffi::c_int
                        | a << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (a >> 41 as ::core::ffi::c_int
                        | a << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(a & b ^ !a & c)
            .wrapping_add(k[(t + 12 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[12 as ::core::ffi::c_int as usize]);
        t2_0 = ((e >> 28 as ::core::ffi::c_int
            | e << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (e >> 34 as ::core::ffi::c_int
                | e << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (e >> 39 as ::core::ffi::c_int
                | e << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(e & f ^ e & g ^ f & g);
        h = h.wrapping_add(t1_0);
        d = t1_0.wrapping_add(t2_0);
        t1_0 = c
            .wrapping_add(
                (h >> 14 as ::core::ffi::c_int
                    | h << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (h >> 18 as ::core::ffi::c_int
                        | h << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (h >> 41 as ::core::ffi::c_int
                        | h << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(h & a ^ !h & b)
            .wrapping_add(k[(t + 13 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[13 as ::core::ffi::c_int as usize]);
        t2_0 = ((d >> 28 as ::core::ffi::c_int
            | d << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (d >> 34 as ::core::ffi::c_int
                | d << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (d >> 39 as ::core::ffi::c_int
                | d << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(d & e ^ d & f ^ e & f);
        g = g.wrapping_add(t1_0);
        c = t1_0.wrapping_add(t2_0);
        t1_0 = b
            .wrapping_add(
                (g >> 14 as ::core::ffi::c_int
                    | g << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (g >> 18 as ::core::ffi::c_int
                        | g << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (g >> 41 as ::core::ffi::c_int
                        | g << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(g & h ^ !g & a)
            .wrapping_add(k[(t + 14 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[14 as ::core::ffi::c_int as usize]);
        t2_0 = ((c >> 28 as ::core::ffi::c_int
            | c << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (c >> 34 as ::core::ffi::c_int
                | c << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (c >> 39 as ::core::ffi::c_int
                | c << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(c & d ^ c & e ^ d & e);
        f = f.wrapping_add(t1_0);
        b = t1_0.wrapping_add(t2_0);
        t1_0 = a
            .wrapping_add(
                (f >> 14 as ::core::ffi::c_int
                    | f << 64 as ::core::ffi::c_int - 14 as ::core::ffi::c_int)
                    ^ (f >> 18 as ::core::ffi::c_int
                        | f << 64 as ::core::ffi::c_int - 18 as ::core::ffi::c_int)
                    ^ (f >> 41 as ::core::ffi::c_int
                        | f << 64 as ::core::ffi::c_int - 41 as ::core::ffi::c_int),
            )
            .wrapping_add(f & g ^ !f & h)
            .wrapping_add(k[(t + 15 as ::core::ffi::c_int) as usize])
            .wrapping_add(w[15 as ::core::ffi::c_int as usize]);
        t2_0 = ((b >> 28 as ::core::ffi::c_int
            | b << 64 as ::core::ffi::c_int - 28 as ::core::ffi::c_int)
            ^ (b >> 34 as ::core::ffi::c_int
                | b << 64 as ::core::ffi::c_int - 34 as ::core::ffi::c_int)
            ^ (b >> 39 as ::core::ffi::c_int
                | b << 64 as ::core::ffi::c_int - 39 as ::core::ffi::c_int))
            .wrapping_add(b & c ^ b & d ^ c & d);
        e = e.wrapping_add(t1_0);
        a = t1_0.wrapping_add(t2_0);
        t += 16 as ::core::ffi::c_int;
    }
    (*hd).h0 = (*hd).h0.wrapping_add(a);
    (*hd).h1 = (*hd).h1.wrapping_add(b);
    (*hd).h2 = (*hd).h2.wrapping_add(c);
    (*hd).h3 = (*hd).h3.wrapping_add(d);
    (*hd).h4 = (*hd).h4.wrapping_add(e);
    (*hd).h5 = (*hd).h5.wrapping_add(f);
    (*hd).h6 = (*hd).h6.wrapping_add(g);
    (*hd).h7 = (*hd).h7.wrapping_add(h);
    return ((8 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as usize)
        .wrapping_mul(::core::mem::size_of::<uint64_t>() as usize)
        .wrapping_add(::core::mem::size_of::<uint32_t>() as usize)
        .wrapping_add(
            (3 as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize),
        ) as ::core::ffi::c_uint;
}
unsafe extern "C-unwind" fn _SHA512_transform(
    mut ctx: *mut SHA512_CONTEXT,
    mut data: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    return (__transform(&raw mut (*ctx).state, data) as usize).wrapping_add(
        (3 as usize).wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as usize),
    ) as ::core::ffi::c_uint;
}
#[unsafe(export_name = "__pdf_impl_SHA512_write")]
pub unsafe extern "C-unwind" fn SHA512_write(
    mut hd: *mut SHA512_CONTEXT,
    mut inbuf: *const ::core::ffi::c_uchar,
    mut inlen: ::core::ffi::c_uint,
) {
    let mut stack_burn: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if (*hd).count == 128 as ::core::ffi::c_int {
        stack_burn = _SHA512_transform(hd, &raw mut (*hd).buf as *mut ::core::ffi::c_uchar);
        _gcry_burn_stack(stack_burn as ::core::ffi::c_int);
        (*hd).count = 0 as ::core::ffi::c_int;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1);
    }
    if inbuf.is_null() {
        return;
    }
    if (*hd).count != 0 {
        while inlen != 0 && (*hd).count < 128 as ::core::ffi::c_int {
            let fresh32 = inbuf;
            inbuf = inbuf.offset(1);
            let fresh33 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh33 as usize] = *fresh32;
            inlen = inlen.wrapping_sub(1);
        }
        SHA512_write(
            hd,
            ::core::ptr::null::<::core::ffi::c_uchar>(),
            0 as ::core::ffi::c_uint,
        );
        if inlen == 0 {
            return;
        }
    }
    _gcry_burn_stack(stack_burn as ::core::ffi::c_int);
    while inlen >= 128 as ::core::ffi::c_uint {
        stack_burn = _SHA512_transform(hd, inbuf);
        (*hd).count = 0 as ::core::ffi::c_int;
        (*hd).nblocks = (*hd).nblocks.wrapping_add(1);
        inlen = inlen.wrapping_sub(128 as ::core::ffi::c_uint);
        inbuf = inbuf.offset(128 as ::core::ffi::c_int as isize);
    }
    while inlen != 0 && (*hd).count < 128 as ::core::ffi::c_int {
        let fresh34 = inbuf;
        inbuf = inbuf.offset(1);
        let fresh35 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh35 as usize] = *fresh34;
        inlen = inlen.wrapping_sub(1);
    }
}
#[unsafe(export_name = "__pdf_impl_SHA512_final")]
pub unsafe extern "C-unwind" fn SHA512_final(
    mut outbuf: *mut ::core::ffi::c_uchar,
    mut hd: *mut SHA512_CONTEXT,
) {
    let mut stack_burn_depth: ::core::ffi::c_uint = 0;
    let mut t: uint64_t = 0;
    let mut msb: uint64_t = 0;
    let mut lsb: uint64_t = 0;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    SHA512_write(
        hd,
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        0 as ::core::ffi::c_uint,
    );
    t = (*hd).nblocks as uint64_t;
    lsb = t << 7 as ::core::ffi::c_int;
    msb = t >> 57 as ::core::ffi::c_int;
    t = lsb;
    lsb = lsb.wrapping_add((*hd).count as uint64_t);
    if lsb < t {
        msb = msb.wrapping_add(1);
    }
    t = lsb;
    lsb <<= 3 as ::core::ffi::c_int;
    msb <<= 3 as ::core::ffi::c_int;
    msb |= t >> 61 as ::core::ffi::c_int;
    if (*hd).count < 112 as ::core::ffi::c_int {
        let fresh36 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh36 as usize] = 0x80 as ::core::ffi::c_uchar;
        while (*hd).count < 112 as ::core::ffi::c_int {
            let fresh37 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh37 as usize] = 0 as ::core::ffi::c_uchar;
        }
    } else {
        let fresh38 = (*hd).count;
        (*hd).count = (*hd).count + 1;
        (*hd).buf[fresh38 as usize] = 0x80 as ::core::ffi::c_uchar;
        while (*hd).count < 128 as ::core::ffi::c_int {
            let fresh39 = (*hd).count;
            (*hd).count = (*hd).count + 1;
            (*hd).buf[fresh39 as usize] = 0 as ::core::ffi::c_uchar;
        }
        SHA512_write(
            hd,
            ::core::ptr::null::<::core::ffi::c_uchar>(),
            0 as ::core::ffi::c_uint,
        );
        memset(
            &raw mut (*hd).buf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            112 as size_t,
        );
    }
    buf_put_be64(
        (&raw mut (*hd).buf as *mut ::core::ffi::c_uchar).offset(112 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        msb,
    );
    buf_put_be64(
        (&raw mut (*hd).buf as *mut ::core::ffi::c_uchar).offset(120 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        lsb,
    );
    stack_burn_depth = _SHA512_transform(hd, &raw mut (*hd).buf as *mut ::core::ffi::c_uchar);
    _gcry_burn_stack(stack_burn_depth as ::core::ffi::c_int);
    p = outbuf;
    (p as *mut uint64_t).write_unaligned(_gcry_bswap64((*hd).state.h0));
    p = p.offset(8 as ::core::ffi::c_int as isize);
    (p as *mut uint64_t).write_unaligned(_gcry_bswap64((*hd).state.h1));
    p = p.offset(8 as ::core::ffi::c_int as isize);
    (p as *mut uint64_t).write_unaligned(_gcry_bswap64((*hd).state.h2));
    p = p.offset(8 as ::core::ffi::c_int as isize);
    (p as *mut uint64_t).write_unaligned(_gcry_bswap64((*hd).state.h3));
    p = p.offset(8 as ::core::ffi::c_int as isize);
    (p as *mut uint64_t).write_unaligned(_gcry_bswap64((*hd).state.h4));
    p = p.offset(8 as ::core::ffi::c_int as isize);
    (p as *mut uint64_t).write_unaligned(_gcry_bswap64((*hd).state.h5));
    p = p.offset(8 as ::core::ffi::c_int as isize);
    (p as *mut uint64_t).write_unaligned(_gcry_bswap64((*hd).state.h6));
    p = p.offset(8 as ::core::ffi::c_int as isize);
    (p as *mut uint64_t).write_unaligned(_gcry_bswap64((*hd).state.h7));
    p = p.offset(8 as ::core::ffi::c_int as isize);
}
unsafe extern "C-unwind" fn do_encrypt_stream(
    mut ctx: *mut ARC4_CONTEXT,
    mut outbuf: *mut ::core::ffi::c_uchar,
    mut inbuf: *const ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
) {
    let mut i: ::core::ffi::c_int = (*ctx).idx_i;
    let mut j: ::core::ffi::c_int = (*ctx).idx_j;
    let mut sbox: *mut ::core::ffi::c_uchar = &raw mut (*ctx).sbox as *mut ::core::ffi::c_uchar;
    let mut t: ::core::ffi::c_int = 0;
    loop {
        let fresh40 = len;
        len = len.wrapping_sub(1);
        if !(fresh40 != 0) {
            break;
        }
        i += 1;
        i = i & 255 as ::core::ffi::c_int;
        j += *sbox.offset(i as isize) as ::core::ffi::c_int;
        j &= 255 as ::core::ffi::c_int;
        t = *sbox.offset(i as isize) as ::core::ffi::c_int;
        *sbox.offset(i as isize) = *sbox.offset(j as isize);
        *sbox.offset(j as isize) = t as ::core::ffi::c_uchar;
        let fresh41 = inbuf;
        inbuf = inbuf.offset(1);
        let fresh42 = outbuf;
        outbuf = outbuf.offset(1);
        *fresh42 = (*fresh41 as ::core::ffi::c_int
            ^ *sbox.offset(
                (*sbox.offset(i as isize) as ::core::ffi::c_int
                    + *sbox.offset(j as isize) as ::core::ffi::c_int
                    & 255 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int) as ::core::ffi::c_uchar;
    }
    (*ctx).idx_i = i;
    (*ctx).idx_j = j;
}
#[unsafe(export_name = "__pdf_impl_ARC4")]
pub unsafe extern "C-unwind" fn ARC4(
    mut ctx: *mut ARC4_CONTEXT,
    mut len: ::core::ffi::c_uint,
    mut inbuf: *const ::core::ffi::c_uchar,
    mut outbuf: *mut ::core::ffi::c_uchar,
) {
    do_encrypt_stream(ctx, outbuf, inbuf, len);
    _gcry_burn_stack(64 as ::core::ffi::c_int);
}
unsafe extern "C-unwind" fn do_arcfour_setkey(
    mut ctx: *mut ARC4_CONTEXT,
    mut key: *const ::core::ffi::c_uchar,
    mut keylen: ::core::ffi::c_uint,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut karr: [::core::ffi::c_uchar; 256] = [0; 256];
    (*ctx).idx_j = 0 as ::core::ffi::c_int;
    (*ctx).idx_i = (*ctx).idx_j;
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        (*ctx).sbox[i as usize] = i as ::core::ffi::c_uchar;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        karr[i as usize] = *key.offset((i as ::core::ffi::c_uint).wrapping_rem(keylen) as isize);
        i += 1;
    }
    j = 0 as ::core::ffi::c_int;
    i = j;
    while i < 256 as ::core::ffi::c_int {
        let mut t: ::core::ffi::c_int = 0;
        j = (j
            + (*ctx).sbox[i as usize] as ::core::ffi::c_int
            + karr[i as usize] as ::core::ffi::c_int)
            % 256 as ::core::ffi::c_int;
        t = (*ctx).sbox[i as usize] as ::core::ffi::c_int;
        (*ctx).sbox[i as usize] = (*ctx).sbox[j as usize];
        (*ctx).sbox[j as usize] = t as ::core::ffi::c_uchar;
        i += 1;
    }
    memset(
        &raw mut karr as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        256 as size_t,
    );
}
#[unsafe(export_name = "__pdf_impl_ARC4_set_key")]
pub unsafe extern "C-unwind" fn ARC4_set_key(
    mut ctx: *mut ARC4_CONTEXT,
    mut keylen: ::core::ffi::c_uint,
    mut key: *const ::core::ffi::c_uchar,
) {
    do_arcfour_setkey(ctx, key, keylen);
    _gcry_burn_stack(300 as ::core::ffi::c_int);
}
#[unsafe(export_name = "__pdf_impl_AES_ecb_encrypt")]
pub unsafe extern "C-unwind" fn AES_ecb_encrypt(
    mut key: *const ::core::ffi::c_uchar,
    mut key_len: size_t,
    mut plain: *const ::core::ffi::c_uchar,
    mut plain_len: size_t,
    mut cipher: *mut *mut ::core::ffi::c_uchar,
    mut cipher_len: *mut size_t,
) {
    let mut ctx: *mut AES_CONTEXT = ::core::ptr::null_mut::<AES_CONTEXT>();
    let mut aes: AES_CONTEXT = AES_CONTEXT {
        nrounds: 0,
        rk: [0; 60],
        iv: [0; 16],
    };
    let mut inptr: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut outptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut len: size_t = 0;
    ctx = &raw mut aes;
    *cipher_len = plain_len;
    *cipher = new((*cipher_len as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    (*ctx).nrounds = rijndaelSetupEncrypt(
        &raw mut (*ctx).rk as *mut uint32_t,
        key as *const uint8_t,
        key_len.wrapping_mul(8 as size_t) as ::core::ffi::c_int,
    );
    inptr = plain;
    outptr = *cipher;
    len = plain_len;
    while len >= AES_BLOCKSIZE as size_t {
        rijndaelEncrypt(
            &raw mut (*ctx).rk as *mut uint32_t,
            (*ctx).nrounds,
            inptr as *const uint8_t,
            outptr as *mut uint8_t,
        );
        inptr = inptr.offset(AES_BLOCKSIZE as isize);
        outptr = outptr.offset(AES_BLOCKSIZE as isize);
        len = len.wrapping_sub(AES_BLOCKSIZE as size_t);
    }
    if len > 0 as size_t {
        let mut block: [::core::ffi::c_uchar; 16] = [0; 16];
        memcpy(
            &raw mut block as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            inptr as *const ::core::ffi::c_void,
            len,
        );
        rijndaelEncrypt(
            &raw mut (*ctx).rk as *mut uint32_t,
            (*ctx).nrounds,
            &raw mut block as *mut ::core::ffi::c_uchar as *const uint8_t,
            outptr as *mut uint8_t,
        );
        inptr = inptr.offset(len as isize);
        outptr = outptr.offset(AES_BLOCKSIZE as isize);
    }
}
#[unsafe(export_name = "__pdf_impl_AES_cbc_encrypt_tectonic")]
pub unsafe extern "C-unwind" fn AES_cbc_encrypt_tectonic(
    mut key: *const ::core::ffi::c_uchar,
    mut key_len: size_t,
    mut iv: *const ::core::ffi::c_uchar,
    mut padding: ::core::ffi::c_int,
    mut plain: *const ::core::ffi::c_uchar,
    mut plain_len: size_t,
    mut cipher: *mut *mut ::core::ffi::c_uchar,
    mut cipher_len: *mut size_t,
) {
    let mut ctx: *mut AES_CONTEXT = ::core::ptr::null_mut::<AES_CONTEXT>();
    let mut aes: AES_CONTEXT = AES_CONTEXT {
        nrounds: 0,
        rk: [0; 60],
        iv: [0; 16],
    };
    let mut inptr: *const ::core::ffi::c_uchar = ::core::ptr::null::<::core::ffi::c_uchar>();
    let mut outptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut block: [::core::ffi::c_uchar; 16] = [0; 16];
    let mut len: size_t = 0;
    let mut i: size_t = 0;
    let mut padbytes: ::core::ffi::c_int = 0;
    ctx = &raw mut aes;
    if !iv.is_null() {
        memcpy(
            &raw mut (*ctx).iv as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            iv as *const ::core::ffi::c_void,
            AES_BLOCKSIZE as size_t,
        );
    } else {
        i = 0 as size_t;
        while i < AES_BLOCKSIZE as size_t {
            (*ctx).iv[i as usize] =
                (genrand_int31() % 256 as ::core::ffi::c_long) as ::core::ffi::c_uchar;
            i = i.wrapping_add(1);
        }
    }
    padbytes = (if padding != 0 {
        (AES_BLOCKSIZE as size_t).wrapping_sub(plain_len.wrapping_rem(AES_BLOCKSIZE as size_t))
    } else if plain_len.wrapping_rem(AES_BLOCKSIZE as size_t) != 0 {
        (AES_BLOCKSIZE as size_t).wrapping_sub(plain_len.wrapping_rem(AES_BLOCKSIZE as size_t))
    } else {
        0 as size_t
    }) as ::core::ffi::c_int;
    *cipher_len = plain_len
        .wrapping_add(
            (if !iv.is_null() {
                0 as ::core::ffi::c_int
            } else {
                AES_BLOCKSIZE
            }) as size_t,
        )
        .wrapping_add(padbytes as size_t);
    *cipher = new((*cipher_len as uint32_t as usize)
        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as usize)
        as uint32_t) as *mut ::core::ffi::c_uchar;
    (*ctx).nrounds = rijndaelSetupEncrypt(
        &raw mut (*ctx).rk as *mut uint32_t,
        key as *const uint8_t,
        key_len.wrapping_mul(8 as size_t) as ::core::ffi::c_int,
    );
    inptr = plain;
    outptr = *cipher;
    if iv.is_null() {
        memcpy(
            outptr as *mut ::core::ffi::c_void,
            &raw mut (*ctx).iv as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
            AES_BLOCKSIZE as size_t,
        );
        outptr = outptr.offset(AES_BLOCKSIZE as isize);
    }
    len = plain_len;
    while len >= AES_BLOCKSIZE as size_t {
        i = 0 as size_t;
        while i < AES_BLOCKSIZE as size_t {
            block[i as usize] = (*inptr.offset(i as isize) as ::core::ffi::c_int
                ^ (*ctx).iv[i as usize] as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            i = i.wrapping_add(1);
        }
        rijndaelEncrypt(
            &raw mut (*ctx).rk as *mut uint32_t,
            (*ctx).nrounds,
            &raw mut block as *mut ::core::ffi::c_uchar as *const uint8_t,
            outptr as *mut uint8_t,
        );
        memcpy(
            &raw mut (*ctx).iv as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            outptr as *const ::core::ffi::c_void,
            AES_BLOCKSIZE as size_t,
        );
        inptr = inptr.offset(AES_BLOCKSIZE as isize);
        outptr = outptr.offset(AES_BLOCKSIZE as isize);
        len = len.wrapping_sub(AES_BLOCKSIZE as size_t);
    }
    if len > 0 as size_t || padding != 0 {
        i = 0 as size_t;
        while i < len {
            block[i as usize] = (*inptr.offset(i as isize) as ::core::ffi::c_int
                ^ (*ctx).iv[i as usize] as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            i = i.wrapping_add(1);
        }
        i = len;
        while i < AES_BLOCKSIZE as size_t {
            block[i as usize] =
                (padbytes ^ (*ctx).iv[i as usize] as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            i = i.wrapping_add(1);
        }
        rijndaelEncrypt(
            &raw mut (*ctx).rk as *mut uint32_t,
            (*ctx).nrounds,
            &raw mut block as *mut ::core::ffi::c_uchar as *const uint8_t,
            outptr as *mut uint8_t,
        );
        memcpy(
            &raw mut (*ctx).iv as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            outptr as *const ::core::ffi::c_void,
            AES_BLOCKSIZE as size_t,
        );
        inptr = inptr.offset(AES_BLOCKSIZE as isize);
        outptr = outptr.offset(AES_BLOCKSIZE as isize);
    }
}
static mut Te0: [uint32_t; 256] = [
    0xc66363a5 as ::core::ffi::c_uint,
    0xf87c7c84 as ::core::ffi::c_uint,
    0xee777799 as ::core::ffi::c_uint,
    0xf67b7b8d as ::core::ffi::c_uint,
    0xfff2f20d as ::core::ffi::c_uint,
    0xd66b6bbd as ::core::ffi::c_uint,
    0xde6f6fb1 as ::core::ffi::c_uint,
    0x91c5c554 as ::core::ffi::c_uint,
    0x60303050 as ::core::ffi::c_uint,
    0x2010103 as ::core::ffi::c_uint,
    0xce6767a9 as ::core::ffi::c_uint,
    0x562b2b7d as ::core::ffi::c_uint,
    0xe7fefe19 as ::core::ffi::c_uint,
    0xb5d7d762 as ::core::ffi::c_uint,
    0x4dababe6 as ::core::ffi::c_uint,
    0xec76769a as ::core::ffi::c_uint,
    0x8fcaca45 as ::core::ffi::c_uint,
    0x1f82829d as ::core::ffi::c_uint,
    0x89c9c940 as ::core::ffi::c_uint,
    0xfa7d7d87 as ::core::ffi::c_uint,
    0xeffafa15 as ::core::ffi::c_uint,
    0xb25959eb as ::core::ffi::c_uint,
    0x8e4747c9 as ::core::ffi::c_uint,
    0xfbf0f00b as ::core::ffi::c_uint,
    0x41adadec as ::core::ffi::c_uint,
    0xb3d4d467 as ::core::ffi::c_uint,
    0x5fa2a2fd as ::core::ffi::c_uint,
    0x45afafea as ::core::ffi::c_uint,
    0x239c9cbf as ::core::ffi::c_uint,
    0x53a4a4f7 as ::core::ffi::c_uint,
    0xe4727296 as ::core::ffi::c_uint,
    0x9bc0c05b as ::core::ffi::c_uint,
    0x75b7b7c2 as ::core::ffi::c_uint,
    0xe1fdfd1c as ::core::ffi::c_uint,
    0x3d9393ae as ::core::ffi::c_uint,
    0x4c26266a as ::core::ffi::c_uint,
    0x6c36365a as ::core::ffi::c_uint,
    0x7e3f3f41 as ::core::ffi::c_uint,
    0xf5f7f702 as ::core::ffi::c_uint,
    0x83cccc4f as ::core::ffi::c_uint,
    0x6834345c as ::core::ffi::c_uint,
    0x51a5a5f4 as ::core::ffi::c_uint,
    0xd1e5e534 as ::core::ffi::c_uint,
    0xf9f1f108 as ::core::ffi::c_uint,
    0xe2717193 as ::core::ffi::c_uint,
    0xabd8d873 as ::core::ffi::c_uint,
    0x62313153 as ::core::ffi::c_uint,
    0x2a15153f as ::core::ffi::c_uint,
    0x804040c as ::core::ffi::c_uint,
    0x95c7c752 as ::core::ffi::c_uint,
    0x46232365 as ::core::ffi::c_uint,
    0x9dc3c35e as ::core::ffi::c_uint,
    0x30181828 as ::core::ffi::c_uint,
    0x379696a1 as ::core::ffi::c_uint,
    0xa05050f as ::core::ffi::c_uint,
    0x2f9a9ab5 as ::core::ffi::c_uint,
    0xe070709 as ::core::ffi::c_uint,
    0x24121236 as ::core::ffi::c_uint,
    0x1b80809b as ::core::ffi::c_uint,
    0xdfe2e23d as ::core::ffi::c_uint,
    0xcdebeb26 as ::core::ffi::c_uint,
    0x4e272769 as ::core::ffi::c_uint,
    0x7fb2b2cd as ::core::ffi::c_uint,
    0xea75759f as ::core::ffi::c_uint,
    0x1209091b as ::core::ffi::c_uint,
    0x1d83839e as ::core::ffi::c_uint,
    0x582c2c74 as ::core::ffi::c_uint,
    0x341a1a2e as ::core::ffi::c_uint,
    0x361b1b2d as ::core::ffi::c_uint,
    0xdc6e6eb2 as ::core::ffi::c_uint,
    0xb45a5aee as ::core::ffi::c_uint,
    0x5ba0a0fb as ::core::ffi::c_uint,
    0xa45252f6 as ::core::ffi::c_uint,
    0x763b3b4d as ::core::ffi::c_uint,
    0xb7d6d661 as ::core::ffi::c_uint,
    0x7db3b3ce as ::core::ffi::c_uint,
    0x5229297b as ::core::ffi::c_uint,
    0xdde3e33e as ::core::ffi::c_uint,
    0x5e2f2f71 as ::core::ffi::c_uint,
    0x13848497 as ::core::ffi::c_uint,
    0xa65353f5 as ::core::ffi::c_uint,
    0xb9d1d168 as ::core::ffi::c_uint,
    0 as ::core::ffi::c_uint,
    0xc1eded2c as ::core::ffi::c_uint,
    0x40202060 as ::core::ffi::c_uint,
    0xe3fcfc1f as ::core::ffi::c_uint,
    0x79b1b1c8 as ::core::ffi::c_uint,
    0xb65b5bed as ::core::ffi::c_uint,
    0xd46a6abe as ::core::ffi::c_uint,
    0x8dcbcb46 as ::core::ffi::c_uint,
    0x67bebed9 as ::core::ffi::c_uint,
    0x7239394b as ::core::ffi::c_uint,
    0x944a4ade as ::core::ffi::c_uint,
    0x984c4cd4 as ::core::ffi::c_uint,
    0xb05858e8 as ::core::ffi::c_uint,
    0x85cfcf4a as ::core::ffi::c_uint,
    0xbbd0d06b as ::core::ffi::c_uint,
    0xc5efef2a as ::core::ffi::c_uint,
    0x4faaaae5 as ::core::ffi::c_uint,
    0xedfbfb16 as ::core::ffi::c_uint,
    0x864343c5 as ::core::ffi::c_uint,
    0x9a4d4dd7 as ::core::ffi::c_uint,
    0x66333355 as ::core::ffi::c_uint,
    0x11858594 as ::core::ffi::c_uint,
    0x8a4545cf as ::core::ffi::c_uint,
    0xe9f9f910 as ::core::ffi::c_uint,
    0x4020206 as ::core::ffi::c_uint,
    0xfe7f7f81 as ::core::ffi::c_uint,
    0xa05050f0 as ::core::ffi::c_uint,
    0x783c3c44 as ::core::ffi::c_uint,
    0x259f9fba as ::core::ffi::c_uint,
    0x4ba8a8e3 as ::core::ffi::c_uint,
    0xa25151f3 as ::core::ffi::c_uint,
    0x5da3a3fe as ::core::ffi::c_uint,
    0x804040c0 as ::core::ffi::c_uint,
    0x58f8f8a as ::core::ffi::c_uint,
    0x3f9292ad as ::core::ffi::c_uint,
    0x219d9dbc as ::core::ffi::c_uint,
    0x70383848 as ::core::ffi::c_uint,
    0xf1f5f504 as ::core::ffi::c_uint,
    0x63bcbcdf as ::core::ffi::c_uint,
    0x77b6b6c1 as ::core::ffi::c_uint,
    0xafdada75 as ::core::ffi::c_uint,
    0x42212163 as ::core::ffi::c_uint,
    0x20101030 as ::core::ffi::c_uint,
    0xe5ffff1a as ::core::ffi::c_uint,
    0xfdf3f30e as ::core::ffi::c_uint,
    0xbfd2d26d as ::core::ffi::c_uint,
    0x81cdcd4c as ::core::ffi::c_uint,
    0x180c0c14 as ::core::ffi::c_uint,
    0x26131335 as ::core::ffi::c_uint,
    0xc3ecec2f as ::core::ffi::c_uint,
    0xbe5f5fe1 as ::core::ffi::c_uint,
    0x359797a2 as ::core::ffi::c_uint,
    0x884444cc as ::core::ffi::c_uint,
    0x2e171739 as ::core::ffi::c_uint,
    0x93c4c457 as ::core::ffi::c_uint,
    0x55a7a7f2 as ::core::ffi::c_uint,
    0xfc7e7e82 as ::core::ffi::c_uint,
    0x7a3d3d47 as ::core::ffi::c_uint,
    0xc86464ac as ::core::ffi::c_uint,
    0xba5d5de7 as ::core::ffi::c_uint,
    0x3219192b as ::core::ffi::c_uint,
    0xe6737395 as ::core::ffi::c_uint,
    0xc06060a0 as ::core::ffi::c_uint,
    0x19818198 as ::core::ffi::c_uint,
    0x9e4f4fd1 as ::core::ffi::c_uint,
    0xa3dcdc7f as ::core::ffi::c_uint,
    0x44222266 as ::core::ffi::c_uint,
    0x542a2a7e as ::core::ffi::c_uint,
    0x3b9090ab as ::core::ffi::c_uint,
    0xb888883 as ::core::ffi::c_uint,
    0x8c4646ca as ::core::ffi::c_uint,
    0xc7eeee29 as ::core::ffi::c_uint,
    0x6bb8b8d3 as ::core::ffi::c_uint,
    0x2814143c as ::core::ffi::c_uint,
    0xa7dede79 as ::core::ffi::c_uint,
    0xbc5e5ee2 as ::core::ffi::c_uint,
    0x160b0b1d as ::core::ffi::c_uint,
    0xaddbdb76 as ::core::ffi::c_uint,
    0xdbe0e03b as ::core::ffi::c_uint,
    0x64323256 as ::core::ffi::c_uint,
    0x743a3a4e as ::core::ffi::c_uint,
    0x140a0a1e as ::core::ffi::c_uint,
    0x924949db as ::core::ffi::c_uint,
    0xc06060a as ::core::ffi::c_uint,
    0x4824246c as ::core::ffi::c_uint,
    0xb85c5ce4 as ::core::ffi::c_uint,
    0x9fc2c25d as ::core::ffi::c_uint,
    0xbdd3d36e as ::core::ffi::c_uint,
    0x43acacef as ::core::ffi::c_uint,
    0xc46262a6 as ::core::ffi::c_uint,
    0x399191a8 as ::core::ffi::c_uint,
    0x319595a4 as ::core::ffi::c_uint,
    0xd3e4e437 as ::core::ffi::c_uint,
    0xf279798b as ::core::ffi::c_uint,
    0xd5e7e732 as ::core::ffi::c_uint,
    0x8bc8c843 as ::core::ffi::c_uint,
    0x6e373759 as ::core::ffi::c_uint,
    0xda6d6db7 as ::core::ffi::c_uint,
    0x18d8d8c as ::core::ffi::c_uint,
    0xb1d5d564 as ::core::ffi::c_uint,
    0x9c4e4ed2 as ::core::ffi::c_uint,
    0x49a9a9e0 as ::core::ffi::c_uint,
    0xd86c6cb4 as ::core::ffi::c_uint,
    0xac5656fa as ::core::ffi::c_uint,
    0xf3f4f407 as ::core::ffi::c_uint,
    0xcfeaea25 as ::core::ffi::c_uint,
    0xca6565af as ::core::ffi::c_uint,
    0xf47a7a8e as ::core::ffi::c_uint,
    0x47aeaee9 as ::core::ffi::c_uint,
    0x10080818 as ::core::ffi::c_uint,
    0x6fbabad5 as ::core::ffi::c_uint,
    0xf0787888 as ::core::ffi::c_uint,
    0x4a25256f as ::core::ffi::c_uint,
    0x5c2e2e72 as ::core::ffi::c_uint,
    0x381c1c24 as ::core::ffi::c_uint,
    0x57a6a6f1 as ::core::ffi::c_uint,
    0x73b4b4c7 as ::core::ffi::c_uint,
    0x97c6c651 as ::core::ffi::c_uint,
    0xcbe8e823 as ::core::ffi::c_uint,
    0xa1dddd7c as ::core::ffi::c_uint,
    0xe874749c as ::core::ffi::c_uint,
    0x3e1f1f21 as ::core::ffi::c_uint,
    0x964b4bdd as ::core::ffi::c_uint,
    0x61bdbddc as ::core::ffi::c_uint,
    0xd8b8b86 as ::core::ffi::c_uint,
    0xf8a8a85 as ::core::ffi::c_uint,
    0xe0707090 as ::core::ffi::c_uint,
    0x7c3e3e42 as ::core::ffi::c_uint,
    0x71b5b5c4 as ::core::ffi::c_uint,
    0xcc6666aa as ::core::ffi::c_uint,
    0x904848d8 as ::core::ffi::c_uint,
    0x6030305 as ::core::ffi::c_uint,
    0xf7f6f601 as ::core::ffi::c_uint,
    0x1c0e0e12 as ::core::ffi::c_uint,
    0xc26161a3 as ::core::ffi::c_uint,
    0x6a35355f as ::core::ffi::c_uint,
    0xae5757f9 as ::core::ffi::c_uint,
    0x69b9b9d0 as ::core::ffi::c_uint,
    0x17868691 as ::core::ffi::c_uint,
    0x99c1c158 as ::core::ffi::c_uint,
    0x3a1d1d27 as ::core::ffi::c_uint,
    0x279e9eb9 as ::core::ffi::c_uint,
    0xd9e1e138 as ::core::ffi::c_uint,
    0xebf8f813 as ::core::ffi::c_uint,
    0x2b9898b3 as ::core::ffi::c_uint,
    0x22111133 as ::core::ffi::c_uint,
    0xd26969bb as ::core::ffi::c_uint,
    0xa9d9d970 as ::core::ffi::c_uint,
    0x78e8e89 as ::core::ffi::c_uint,
    0x339494a7 as ::core::ffi::c_uint,
    0x2d9b9bb6 as ::core::ffi::c_uint,
    0x3c1e1e22 as ::core::ffi::c_uint,
    0x15878792 as ::core::ffi::c_uint,
    0xc9e9e920 as ::core::ffi::c_uint,
    0x87cece49 as ::core::ffi::c_uint,
    0xaa5555ff as ::core::ffi::c_uint,
    0x50282878 as ::core::ffi::c_uint,
    0xa5dfdf7a as ::core::ffi::c_uint,
    0x38c8c8f as ::core::ffi::c_uint,
    0x59a1a1f8 as ::core::ffi::c_uint,
    0x9898980 as ::core::ffi::c_uint,
    0x1a0d0d17 as ::core::ffi::c_uint,
    0x65bfbfda as ::core::ffi::c_uint,
    0xd7e6e631 as ::core::ffi::c_uint,
    0x844242c6 as ::core::ffi::c_uint,
    0xd06868b8 as ::core::ffi::c_uint,
    0x824141c3 as ::core::ffi::c_uint,
    0x299999b0 as ::core::ffi::c_uint,
    0x5a2d2d77 as ::core::ffi::c_uint,
    0x1e0f0f11 as ::core::ffi::c_uint,
    0x7bb0b0cb as ::core::ffi::c_uint,
    0xa85454fc as ::core::ffi::c_uint,
    0x6dbbbbd6 as ::core::ffi::c_uint,
    0x2c16163a as ::core::ffi::c_uint,
];
static mut Te1: [uint32_t; 256] = [
    0xa5c66363 as ::core::ffi::c_uint,
    0x84f87c7c as ::core::ffi::c_uint,
    0x99ee7777 as ::core::ffi::c_uint,
    0x8df67b7b as ::core::ffi::c_uint,
    0xdfff2f2 as ::core::ffi::c_uint,
    0xbdd66b6b as ::core::ffi::c_uint,
    0xb1de6f6f as ::core::ffi::c_uint,
    0x5491c5c5 as ::core::ffi::c_uint,
    0x50603030 as ::core::ffi::c_uint,
    0x3020101 as ::core::ffi::c_uint,
    0xa9ce6767 as ::core::ffi::c_uint,
    0x7d562b2b as ::core::ffi::c_uint,
    0x19e7fefe as ::core::ffi::c_uint,
    0x62b5d7d7 as ::core::ffi::c_uint,
    0xe64dabab as ::core::ffi::c_uint,
    0x9aec7676 as ::core::ffi::c_uint,
    0x458fcaca as ::core::ffi::c_uint,
    0x9d1f8282 as ::core::ffi::c_uint,
    0x4089c9c9 as ::core::ffi::c_uint,
    0x87fa7d7d as ::core::ffi::c_uint,
    0x15effafa as ::core::ffi::c_uint,
    0xebb25959 as ::core::ffi::c_uint,
    0xc98e4747 as ::core::ffi::c_uint,
    0xbfbf0f0 as ::core::ffi::c_uint,
    0xec41adad as ::core::ffi::c_uint,
    0x67b3d4d4 as ::core::ffi::c_uint,
    0xfd5fa2a2 as ::core::ffi::c_uint,
    0xea45afaf as ::core::ffi::c_uint,
    0xbf239c9c as ::core::ffi::c_uint,
    0xf753a4a4 as ::core::ffi::c_uint,
    0x96e47272 as ::core::ffi::c_uint,
    0x5b9bc0c0 as ::core::ffi::c_uint,
    0xc275b7b7 as ::core::ffi::c_uint,
    0x1ce1fdfd as ::core::ffi::c_uint,
    0xae3d9393 as ::core::ffi::c_uint,
    0x6a4c2626 as ::core::ffi::c_uint,
    0x5a6c3636 as ::core::ffi::c_uint,
    0x417e3f3f as ::core::ffi::c_uint,
    0x2f5f7f7 as ::core::ffi::c_uint,
    0x4f83cccc as ::core::ffi::c_uint,
    0x5c683434 as ::core::ffi::c_uint,
    0xf451a5a5 as ::core::ffi::c_uint,
    0x34d1e5e5 as ::core::ffi::c_uint,
    0x8f9f1f1 as ::core::ffi::c_uint,
    0x93e27171 as ::core::ffi::c_uint,
    0x73abd8d8 as ::core::ffi::c_uint,
    0x53623131 as ::core::ffi::c_uint,
    0x3f2a1515 as ::core::ffi::c_uint,
    0xc080404 as ::core::ffi::c_uint,
    0x5295c7c7 as ::core::ffi::c_uint,
    0x65462323 as ::core::ffi::c_uint,
    0x5e9dc3c3 as ::core::ffi::c_uint,
    0x28301818 as ::core::ffi::c_uint,
    0xa1379696 as ::core::ffi::c_uint,
    0xf0a0505 as ::core::ffi::c_uint,
    0xb52f9a9a as ::core::ffi::c_uint,
    0x90e0707 as ::core::ffi::c_uint,
    0x36241212 as ::core::ffi::c_uint,
    0x9b1b8080 as ::core::ffi::c_uint,
    0x3ddfe2e2 as ::core::ffi::c_uint,
    0x26cdebeb as ::core::ffi::c_uint,
    0x694e2727 as ::core::ffi::c_uint,
    0xcd7fb2b2 as ::core::ffi::c_uint,
    0x9fea7575 as ::core::ffi::c_uint,
    0x1b120909 as ::core::ffi::c_uint,
    0x9e1d8383 as ::core::ffi::c_uint,
    0x74582c2c as ::core::ffi::c_uint,
    0x2e341a1a as ::core::ffi::c_uint,
    0x2d361b1b as ::core::ffi::c_uint,
    0xb2dc6e6e as ::core::ffi::c_uint,
    0xeeb45a5a as ::core::ffi::c_uint,
    0xfb5ba0a0 as ::core::ffi::c_uint,
    0xf6a45252 as ::core::ffi::c_uint,
    0x4d763b3b as ::core::ffi::c_uint,
    0x61b7d6d6 as ::core::ffi::c_uint,
    0xce7db3b3 as ::core::ffi::c_uint,
    0x7b522929 as ::core::ffi::c_uint,
    0x3edde3e3 as ::core::ffi::c_uint,
    0x715e2f2f as ::core::ffi::c_uint,
    0x97138484 as ::core::ffi::c_uint,
    0xf5a65353 as ::core::ffi::c_uint,
    0x68b9d1d1 as ::core::ffi::c_uint,
    0 as ::core::ffi::c_uint,
    0x2cc1eded as ::core::ffi::c_uint,
    0x60402020 as ::core::ffi::c_uint,
    0x1fe3fcfc as ::core::ffi::c_uint,
    0xc879b1b1 as ::core::ffi::c_uint,
    0xedb65b5b as ::core::ffi::c_uint,
    0xbed46a6a as ::core::ffi::c_uint,
    0x468dcbcb as ::core::ffi::c_uint,
    0xd967bebe as ::core::ffi::c_uint,
    0x4b723939 as ::core::ffi::c_uint,
    0xde944a4a as ::core::ffi::c_uint,
    0xd4984c4c as ::core::ffi::c_uint,
    0xe8b05858 as ::core::ffi::c_uint,
    0x4a85cfcf as ::core::ffi::c_uint,
    0x6bbbd0d0 as ::core::ffi::c_uint,
    0x2ac5efef as ::core::ffi::c_uint,
    0xe54faaaa as ::core::ffi::c_uint,
    0x16edfbfb as ::core::ffi::c_uint,
    0xc5864343 as ::core::ffi::c_uint,
    0xd79a4d4d as ::core::ffi::c_uint,
    0x55663333 as ::core::ffi::c_uint,
    0x94118585 as ::core::ffi::c_uint,
    0xcf8a4545 as ::core::ffi::c_uint,
    0x10e9f9f9 as ::core::ffi::c_uint,
    0x6040202 as ::core::ffi::c_uint,
    0x81fe7f7f as ::core::ffi::c_uint,
    0xf0a05050 as ::core::ffi::c_uint,
    0x44783c3c as ::core::ffi::c_uint,
    0xba259f9f as ::core::ffi::c_uint,
    0xe34ba8a8 as ::core::ffi::c_uint,
    0xf3a25151 as ::core::ffi::c_uint,
    0xfe5da3a3 as ::core::ffi::c_uint,
    0xc0804040 as ::core::ffi::c_uint,
    0x8a058f8f as ::core::ffi::c_uint,
    0xad3f9292 as ::core::ffi::c_uint,
    0xbc219d9d as ::core::ffi::c_uint,
    0x48703838 as ::core::ffi::c_uint,
    0x4f1f5f5 as ::core::ffi::c_uint,
    0xdf63bcbc as ::core::ffi::c_uint,
    0xc177b6b6 as ::core::ffi::c_uint,
    0x75afdada as ::core::ffi::c_uint,
    0x63422121 as ::core::ffi::c_uint,
    0x30201010 as ::core::ffi::c_uint,
    0x1ae5ffff as ::core::ffi::c_uint,
    0xefdf3f3 as ::core::ffi::c_uint,
    0x6dbfd2d2 as ::core::ffi::c_uint,
    0x4c81cdcd as ::core::ffi::c_uint,
    0x14180c0c as ::core::ffi::c_uint,
    0x35261313 as ::core::ffi::c_uint,
    0x2fc3ecec as ::core::ffi::c_uint,
    0xe1be5f5f as ::core::ffi::c_uint,
    0xa2359797 as ::core::ffi::c_uint,
    0xcc884444 as ::core::ffi::c_uint,
    0x392e1717 as ::core::ffi::c_uint,
    0x5793c4c4 as ::core::ffi::c_uint,
    0xf255a7a7 as ::core::ffi::c_uint,
    0x82fc7e7e as ::core::ffi::c_uint,
    0x477a3d3d as ::core::ffi::c_uint,
    0xacc86464 as ::core::ffi::c_uint,
    0xe7ba5d5d as ::core::ffi::c_uint,
    0x2b321919 as ::core::ffi::c_uint,
    0x95e67373 as ::core::ffi::c_uint,
    0xa0c06060 as ::core::ffi::c_uint,
    0x98198181 as ::core::ffi::c_uint,
    0xd19e4f4f as ::core::ffi::c_uint,
    0x7fa3dcdc as ::core::ffi::c_uint,
    0x66442222 as ::core::ffi::c_uint,
    0x7e542a2a as ::core::ffi::c_uint,
    0xab3b9090 as ::core::ffi::c_uint,
    0x830b8888 as ::core::ffi::c_uint,
    0xca8c4646 as ::core::ffi::c_uint,
    0x29c7eeee as ::core::ffi::c_uint,
    0xd36bb8b8 as ::core::ffi::c_uint,
    0x3c281414 as ::core::ffi::c_uint,
    0x79a7dede as ::core::ffi::c_uint,
    0xe2bc5e5e as ::core::ffi::c_uint,
    0x1d160b0b as ::core::ffi::c_uint,
    0x76addbdb as ::core::ffi::c_uint,
    0x3bdbe0e0 as ::core::ffi::c_uint,
    0x56643232 as ::core::ffi::c_uint,
    0x4e743a3a as ::core::ffi::c_uint,
    0x1e140a0a as ::core::ffi::c_uint,
    0xdb924949 as ::core::ffi::c_uint,
    0xa0c0606 as ::core::ffi::c_uint,
    0x6c482424 as ::core::ffi::c_uint,
    0xe4b85c5c as ::core::ffi::c_uint,
    0x5d9fc2c2 as ::core::ffi::c_uint,
    0x6ebdd3d3 as ::core::ffi::c_uint,
    0xef43acac as ::core::ffi::c_uint,
    0xa6c46262 as ::core::ffi::c_uint,
    0xa8399191 as ::core::ffi::c_uint,
    0xa4319595 as ::core::ffi::c_uint,
    0x37d3e4e4 as ::core::ffi::c_uint,
    0x8bf27979 as ::core::ffi::c_uint,
    0x32d5e7e7 as ::core::ffi::c_uint,
    0x438bc8c8 as ::core::ffi::c_uint,
    0x596e3737 as ::core::ffi::c_uint,
    0xb7da6d6d as ::core::ffi::c_uint,
    0x8c018d8d as ::core::ffi::c_uint,
    0x64b1d5d5 as ::core::ffi::c_uint,
    0xd29c4e4e as ::core::ffi::c_uint,
    0xe049a9a9 as ::core::ffi::c_uint,
    0xb4d86c6c as ::core::ffi::c_uint,
    0xfaac5656 as ::core::ffi::c_uint,
    0x7f3f4f4 as ::core::ffi::c_uint,
    0x25cfeaea as ::core::ffi::c_uint,
    0xafca6565 as ::core::ffi::c_uint,
    0x8ef47a7a as ::core::ffi::c_uint,
    0xe947aeae as ::core::ffi::c_uint,
    0x18100808 as ::core::ffi::c_uint,
    0xd56fbaba as ::core::ffi::c_uint,
    0x88f07878 as ::core::ffi::c_uint,
    0x6f4a2525 as ::core::ffi::c_uint,
    0x725c2e2e as ::core::ffi::c_uint,
    0x24381c1c as ::core::ffi::c_uint,
    0xf157a6a6 as ::core::ffi::c_uint,
    0xc773b4b4 as ::core::ffi::c_uint,
    0x5197c6c6 as ::core::ffi::c_uint,
    0x23cbe8e8 as ::core::ffi::c_uint,
    0x7ca1dddd as ::core::ffi::c_uint,
    0x9ce87474 as ::core::ffi::c_uint,
    0x213e1f1f as ::core::ffi::c_uint,
    0xdd964b4b as ::core::ffi::c_uint,
    0xdc61bdbd as ::core::ffi::c_uint,
    0x860d8b8b as ::core::ffi::c_uint,
    0x850f8a8a as ::core::ffi::c_uint,
    0x90e07070 as ::core::ffi::c_uint,
    0x427c3e3e as ::core::ffi::c_uint,
    0xc471b5b5 as ::core::ffi::c_uint,
    0xaacc6666 as ::core::ffi::c_uint,
    0xd8904848 as ::core::ffi::c_uint,
    0x5060303 as ::core::ffi::c_uint,
    0x1f7f6f6 as ::core::ffi::c_uint,
    0x121c0e0e as ::core::ffi::c_uint,
    0xa3c26161 as ::core::ffi::c_uint,
    0x5f6a3535 as ::core::ffi::c_uint,
    0xf9ae5757 as ::core::ffi::c_uint,
    0xd069b9b9 as ::core::ffi::c_uint,
    0x91178686 as ::core::ffi::c_uint,
    0x5899c1c1 as ::core::ffi::c_uint,
    0x273a1d1d as ::core::ffi::c_uint,
    0xb9279e9e as ::core::ffi::c_uint,
    0x38d9e1e1 as ::core::ffi::c_uint,
    0x13ebf8f8 as ::core::ffi::c_uint,
    0xb32b9898 as ::core::ffi::c_uint,
    0x33221111 as ::core::ffi::c_uint,
    0xbbd26969 as ::core::ffi::c_uint,
    0x70a9d9d9 as ::core::ffi::c_uint,
    0x89078e8e as ::core::ffi::c_uint,
    0xa7339494 as ::core::ffi::c_uint,
    0xb62d9b9b as ::core::ffi::c_uint,
    0x223c1e1e as ::core::ffi::c_uint,
    0x92158787 as ::core::ffi::c_uint,
    0x20c9e9e9 as ::core::ffi::c_uint,
    0x4987cece as ::core::ffi::c_uint,
    0xffaa5555 as ::core::ffi::c_uint,
    0x78502828 as ::core::ffi::c_uint,
    0x7aa5dfdf as ::core::ffi::c_uint,
    0x8f038c8c as ::core::ffi::c_uint,
    0xf859a1a1 as ::core::ffi::c_uint,
    0x80098989 as ::core::ffi::c_uint,
    0x171a0d0d as ::core::ffi::c_uint,
    0xda65bfbf as ::core::ffi::c_uint,
    0x31d7e6e6 as ::core::ffi::c_uint,
    0xc6844242 as ::core::ffi::c_uint,
    0xb8d06868 as ::core::ffi::c_uint,
    0xc3824141 as ::core::ffi::c_uint,
    0xb0299999 as ::core::ffi::c_uint,
    0x775a2d2d as ::core::ffi::c_uint,
    0x111e0f0f as ::core::ffi::c_uint,
    0xcb7bb0b0 as ::core::ffi::c_uint,
    0xfca85454 as ::core::ffi::c_uint,
    0xd66dbbbb as ::core::ffi::c_uint,
    0x3a2c1616 as ::core::ffi::c_uint,
];
static mut Te2: [uint32_t; 256] = [
    0x63a5c663 as ::core::ffi::c_uint,
    0x7c84f87c as ::core::ffi::c_uint,
    0x7799ee77 as ::core::ffi::c_uint,
    0x7b8df67b as ::core::ffi::c_uint,
    0xf20dfff2 as ::core::ffi::c_uint,
    0x6bbdd66b as ::core::ffi::c_uint,
    0x6fb1de6f as ::core::ffi::c_uint,
    0xc55491c5 as ::core::ffi::c_uint,
    0x30506030 as ::core::ffi::c_uint,
    0x1030201 as ::core::ffi::c_uint,
    0x67a9ce67 as ::core::ffi::c_uint,
    0x2b7d562b as ::core::ffi::c_uint,
    0xfe19e7fe as ::core::ffi::c_uint,
    0xd762b5d7 as ::core::ffi::c_uint,
    0xabe64dab as ::core::ffi::c_uint,
    0x769aec76 as ::core::ffi::c_uint,
    0xca458fca as ::core::ffi::c_uint,
    0x829d1f82 as ::core::ffi::c_uint,
    0xc94089c9 as ::core::ffi::c_uint,
    0x7d87fa7d as ::core::ffi::c_uint,
    0xfa15effa as ::core::ffi::c_uint,
    0x59ebb259 as ::core::ffi::c_uint,
    0x47c98e47 as ::core::ffi::c_uint,
    0xf00bfbf0 as ::core::ffi::c_uint,
    0xadec41ad as ::core::ffi::c_uint,
    0xd467b3d4 as ::core::ffi::c_uint,
    0xa2fd5fa2 as ::core::ffi::c_uint,
    0xafea45af as ::core::ffi::c_uint,
    0x9cbf239c as ::core::ffi::c_uint,
    0xa4f753a4 as ::core::ffi::c_uint,
    0x7296e472 as ::core::ffi::c_uint,
    0xc05b9bc0 as ::core::ffi::c_uint,
    0xb7c275b7 as ::core::ffi::c_uint,
    0xfd1ce1fd as ::core::ffi::c_uint,
    0x93ae3d93 as ::core::ffi::c_uint,
    0x266a4c26 as ::core::ffi::c_uint,
    0x365a6c36 as ::core::ffi::c_uint,
    0x3f417e3f as ::core::ffi::c_uint,
    0xf702f5f7 as ::core::ffi::c_uint,
    0xcc4f83cc as ::core::ffi::c_uint,
    0x345c6834 as ::core::ffi::c_uint,
    0xa5f451a5 as ::core::ffi::c_uint,
    0xe534d1e5 as ::core::ffi::c_uint,
    0xf108f9f1 as ::core::ffi::c_uint,
    0x7193e271 as ::core::ffi::c_uint,
    0xd873abd8 as ::core::ffi::c_uint,
    0x31536231 as ::core::ffi::c_uint,
    0x153f2a15 as ::core::ffi::c_uint,
    0x40c0804 as ::core::ffi::c_uint,
    0xc75295c7 as ::core::ffi::c_uint,
    0x23654623 as ::core::ffi::c_uint,
    0xc35e9dc3 as ::core::ffi::c_uint,
    0x18283018 as ::core::ffi::c_uint,
    0x96a13796 as ::core::ffi::c_uint,
    0x50f0a05 as ::core::ffi::c_uint,
    0x9ab52f9a as ::core::ffi::c_uint,
    0x7090e07 as ::core::ffi::c_uint,
    0x12362412 as ::core::ffi::c_uint,
    0x809b1b80 as ::core::ffi::c_uint,
    0xe23ddfe2 as ::core::ffi::c_uint,
    0xeb26cdeb as ::core::ffi::c_uint,
    0x27694e27 as ::core::ffi::c_uint,
    0xb2cd7fb2 as ::core::ffi::c_uint,
    0x759fea75 as ::core::ffi::c_uint,
    0x91b1209 as ::core::ffi::c_uint,
    0x839e1d83 as ::core::ffi::c_uint,
    0x2c74582c as ::core::ffi::c_uint,
    0x1a2e341a as ::core::ffi::c_uint,
    0x1b2d361b as ::core::ffi::c_uint,
    0x6eb2dc6e as ::core::ffi::c_uint,
    0x5aeeb45a as ::core::ffi::c_uint,
    0xa0fb5ba0 as ::core::ffi::c_uint,
    0x52f6a452 as ::core::ffi::c_uint,
    0x3b4d763b as ::core::ffi::c_uint,
    0xd661b7d6 as ::core::ffi::c_uint,
    0xb3ce7db3 as ::core::ffi::c_uint,
    0x297b5229 as ::core::ffi::c_uint,
    0xe33edde3 as ::core::ffi::c_uint,
    0x2f715e2f as ::core::ffi::c_uint,
    0x84971384 as ::core::ffi::c_uint,
    0x53f5a653 as ::core::ffi::c_uint,
    0xd168b9d1 as ::core::ffi::c_uint,
    0 as ::core::ffi::c_uint,
    0xed2cc1ed as ::core::ffi::c_uint,
    0x20604020 as ::core::ffi::c_uint,
    0xfc1fe3fc as ::core::ffi::c_uint,
    0xb1c879b1 as ::core::ffi::c_uint,
    0x5bedb65b as ::core::ffi::c_uint,
    0x6abed46a as ::core::ffi::c_uint,
    0xcb468dcb as ::core::ffi::c_uint,
    0xbed967be as ::core::ffi::c_uint,
    0x394b7239 as ::core::ffi::c_uint,
    0x4ade944a as ::core::ffi::c_uint,
    0x4cd4984c as ::core::ffi::c_uint,
    0x58e8b058 as ::core::ffi::c_uint,
    0xcf4a85cf as ::core::ffi::c_uint,
    0xd06bbbd0 as ::core::ffi::c_uint,
    0xef2ac5ef as ::core::ffi::c_uint,
    0xaae54faa as ::core::ffi::c_uint,
    0xfb16edfb as ::core::ffi::c_uint,
    0x43c58643 as ::core::ffi::c_uint,
    0x4dd79a4d as ::core::ffi::c_uint,
    0x33556633 as ::core::ffi::c_uint,
    0x85941185 as ::core::ffi::c_uint,
    0x45cf8a45 as ::core::ffi::c_uint,
    0xf910e9f9 as ::core::ffi::c_uint,
    0x2060402 as ::core::ffi::c_uint,
    0x7f81fe7f as ::core::ffi::c_uint,
    0x50f0a050 as ::core::ffi::c_uint,
    0x3c44783c as ::core::ffi::c_uint,
    0x9fba259f as ::core::ffi::c_uint,
    0xa8e34ba8 as ::core::ffi::c_uint,
    0x51f3a251 as ::core::ffi::c_uint,
    0xa3fe5da3 as ::core::ffi::c_uint,
    0x40c08040 as ::core::ffi::c_uint,
    0x8f8a058f as ::core::ffi::c_uint,
    0x92ad3f92 as ::core::ffi::c_uint,
    0x9dbc219d as ::core::ffi::c_uint,
    0x38487038 as ::core::ffi::c_uint,
    0xf504f1f5 as ::core::ffi::c_uint,
    0xbcdf63bc as ::core::ffi::c_uint,
    0xb6c177b6 as ::core::ffi::c_uint,
    0xda75afda as ::core::ffi::c_uint,
    0x21634221 as ::core::ffi::c_uint,
    0x10302010 as ::core::ffi::c_uint,
    0xff1ae5ff as ::core::ffi::c_uint,
    0xf30efdf3 as ::core::ffi::c_uint,
    0xd26dbfd2 as ::core::ffi::c_uint,
    0xcd4c81cd as ::core::ffi::c_uint,
    0xc14180c as ::core::ffi::c_uint,
    0x13352613 as ::core::ffi::c_uint,
    0xec2fc3ec as ::core::ffi::c_uint,
    0x5fe1be5f as ::core::ffi::c_uint,
    0x97a23597 as ::core::ffi::c_uint,
    0x44cc8844 as ::core::ffi::c_uint,
    0x17392e17 as ::core::ffi::c_uint,
    0xc45793c4 as ::core::ffi::c_uint,
    0xa7f255a7 as ::core::ffi::c_uint,
    0x7e82fc7e as ::core::ffi::c_uint,
    0x3d477a3d as ::core::ffi::c_uint,
    0x64acc864 as ::core::ffi::c_uint,
    0x5de7ba5d as ::core::ffi::c_uint,
    0x192b3219 as ::core::ffi::c_uint,
    0x7395e673 as ::core::ffi::c_uint,
    0x60a0c060 as ::core::ffi::c_uint,
    0x81981981 as ::core::ffi::c_uint,
    0x4fd19e4f as ::core::ffi::c_uint,
    0xdc7fa3dc as ::core::ffi::c_uint,
    0x22664422 as ::core::ffi::c_uint,
    0x2a7e542a as ::core::ffi::c_uint,
    0x90ab3b90 as ::core::ffi::c_uint,
    0x88830b88 as ::core::ffi::c_uint,
    0x46ca8c46 as ::core::ffi::c_uint,
    0xee29c7ee as ::core::ffi::c_uint,
    0xb8d36bb8 as ::core::ffi::c_uint,
    0x143c2814 as ::core::ffi::c_uint,
    0xde79a7de as ::core::ffi::c_uint,
    0x5ee2bc5e as ::core::ffi::c_uint,
    0xb1d160b as ::core::ffi::c_uint,
    0xdb76addb as ::core::ffi::c_uint,
    0xe03bdbe0 as ::core::ffi::c_uint,
    0x32566432 as ::core::ffi::c_uint,
    0x3a4e743a as ::core::ffi::c_uint,
    0xa1e140a as ::core::ffi::c_uint,
    0x49db9249 as ::core::ffi::c_uint,
    0x60a0c06 as ::core::ffi::c_uint,
    0x246c4824 as ::core::ffi::c_uint,
    0x5ce4b85c as ::core::ffi::c_uint,
    0xc25d9fc2 as ::core::ffi::c_uint,
    0xd36ebdd3 as ::core::ffi::c_uint,
    0xacef43ac as ::core::ffi::c_uint,
    0x62a6c462 as ::core::ffi::c_uint,
    0x91a83991 as ::core::ffi::c_uint,
    0x95a43195 as ::core::ffi::c_uint,
    0xe437d3e4 as ::core::ffi::c_uint,
    0x798bf279 as ::core::ffi::c_uint,
    0xe732d5e7 as ::core::ffi::c_uint,
    0xc8438bc8 as ::core::ffi::c_uint,
    0x37596e37 as ::core::ffi::c_uint,
    0x6db7da6d as ::core::ffi::c_uint,
    0x8d8c018d as ::core::ffi::c_uint,
    0xd564b1d5 as ::core::ffi::c_uint,
    0x4ed29c4e as ::core::ffi::c_uint,
    0xa9e049a9 as ::core::ffi::c_uint,
    0x6cb4d86c as ::core::ffi::c_uint,
    0x56faac56 as ::core::ffi::c_uint,
    0xf407f3f4 as ::core::ffi::c_uint,
    0xea25cfea as ::core::ffi::c_uint,
    0x65afca65 as ::core::ffi::c_uint,
    0x7a8ef47a as ::core::ffi::c_uint,
    0xaee947ae as ::core::ffi::c_uint,
    0x8181008 as ::core::ffi::c_uint,
    0xbad56fba as ::core::ffi::c_uint,
    0x7888f078 as ::core::ffi::c_uint,
    0x256f4a25 as ::core::ffi::c_uint,
    0x2e725c2e as ::core::ffi::c_uint,
    0x1c24381c as ::core::ffi::c_uint,
    0xa6f157a6 as ::core::ffi::c_uint,
    0xb4c773b4 as ::core::ffi::c_uint,
    0xc65197c6 as ::core::ffi::c_uint,
    0xe823cbe8 as ::core::ffi::c_uint,
    0xdd7ca1dd as ::core::ffi::c_uint,
    0x749ce874 as ::core::ffi::c_uint,
    0x1f213e1f as ::core::ffi::c_uint,
    0x4bdd964b as ::core::ffi::c_uint,
    0xbddc61bd as ::core::ffi::c_uint,
    0x8b860d8b as ::core::ffi::c_uint,
    0x8a850f8a as ::core::ffi::c_uint,
    0x7090e070 as ::core::ffi::c_uint,
    0x3e427c3e as ::core::ffi::c_uint,
    0xb5c471b5 as ::core::ffi::c_uint,
    0x66aacc66 as ::core::ffi::c_uint,
    0x48d89048 as ::core::ffi::c_uint,
    0x3050603 as ::core::ffi::c_uint,
    0xf601f7f6 as ::core::ffi::c_uint,
    0xe121c0e as ::core::ffi::c_uint,
    0x61a3c261 as ::core::ffi::c_uint,
    0x355f6a35 as ::core::ffi::c_uint,
    0x57f9ae57 as ::core::ffi::c_uint,
    0xb9d069b9 as ::core::ffi::c_uint,
    0x86911786 as ::core::ffi::c_uint,
    0xc15899c1 as ::core::ffi::c_uint,
    0x1d273a1d as ::core::ffi::c_uint,
    0x9eb9279e as ::core::ffi::c_uint,
    0xe138d9e1 as ::core::ffi::c_uint,
    0xf813ebf8 as ::core::ffi::c_uint,
    0x98b32b98 as ::core::ffi::c_uint,
    0x11332211 as ::core::ffi::c_uint,
    0x69bbd269 as ::core::ffi::c_uint,
    0xd970a9d9 as ::core::ffi::c_uint,
    0x8e89078e as ::core::ffi::c_uint,
    0x94a73394 as ::core::ffi::c_uint,
    0x9bb62d9b as ::core::ffi::c_uint,
    0x1e223c1e as ::core::ffi::c_uint,
    0x87921587 as ::core::ffi::c_uint,
    0xe920c9e9 as ::core::ffi::c_uint,
    0xce4987ce as ::core::ffi::c_uint,
    0x55ffaa55 as ::core::ffi::c_uint,
    0x28785028 as ::core::ffi::c_uint,
    0xdf7aa5df as ::core::ffi::c_uint,
    0x8c8f038c as ::core::ffi::c_uint,
    0xa1f859a1 as ::core::ffi::c_uint,
    0x89800989 as ::core::ffi::c_uint,
    0xd171a0d as ::core::ffi::c_uint,
    0xbfda65bf as ::core::ffi::c_uint,
    0xe631d7e6 as ::core::ffi::c_uint,
    0x42c68442 as ::core::ffi::c_uint,
    0x68b8d068 as ::core::ffi::c_uint,
    0x41c38241 as ::core::ffi::c_uint,
    0x99b02999 as ::core::ffi::c_uint,
    0x2d775a2d as ::core::ffi::c_uint,
    0xf111e0f as ::core::ffi::c_uint,
    0xb0cb7bb0 as ::core::ffi::c_uint,
    0x54fca854 as ::core::ffi::c_uint,
    0xbbd66dbb as ::core::ffi::c_uint,
    0x163a2c16 as ::core::ffi::c_uint,
];
static mut Te3: [uint32_t; 256] = [
    0x6363a5c6 as ::core::ffi::c_uint,
    0x7c7c84f8 as ::core::ffi::c_uint,
    0x777799ee as ::core::ffi::c_uint,
    0x7b7b8df6 as ::core::ffi::c_uint,
    0xf2f20dff as ::core::ffi::c_uint,
    0x6b6bbdd6 as ::core::ffi::c_uint,
    0x6f6fb1de as ::core::ffi::c_uint,
    0xc5c55491 as ::core::ffi::c_uint,
    0x30305060 as ::core::ffi::c_uint,
    0x1010302 as ::core::ffi::c_uint,
    0x6767a9ce as ::core::ffi::c_uint,
    0x2b2b7d56 as ::core::ffi::c_uint,
    0xfefe19e7 as ::core::ffi::c_uint,
    0xd7d762b5 as ::core::ffi::c_uint,
    0xababe64d as ::core::ffi::c_uint,
    0x76769aec as ::core::ffi::c_uint,
    0xcaca458f as ::core::ffi::c_uint,
    0x82829d1f as ::core::ffi::c_uint,
    0xc9c94089 as ::core::ffi::c_uint,
    0x7d7d87fa as ::core::ffi::c_uint,
    0xfafa15ef as ::core::ffi::c_uint,
    0x5959ebb2 as ::core::ffi::c_uint,
    0x4747c98e as ::core::ffi::c_uint,
    0xf0f00bfb as ::core::ffi::c_uint,
    0xadadec41 as ::core::ffi::c_uint,
    0xd4d467b3 as ::core::ffi::c_uint,
    0xa2a2fd5f as ::core::ffi::c_uint,
    0xafafea45 as ::core::ffi::c_uint,
    0x9c9cbf23 as ::core::ffi::c_uint,
    0xa4a4f753 as ::core::ffi::c_uint,
    0x727296e4 as ::core::ffi::c_uint,
    0xc0c05b9b as ::core::ffi::c_uint,
    0xb7b7c275 as ::core::ffi::c_uint,
    0xfdfd1ce1 as ::core::ffi::c_uint,
    0x9393ae3d as ::core::ffi::c_uint,
    0x26266a4c as ::core::ffi::c_uint,
    0x36365a6c as ::core::ffi::c_uint,
    0x3f3f417e as ::core::ffi::c_uint,
    0xf7f702f5 as ::core::ffi::c_uint,
    0xcccc4f83 as ::core::ffi::c_uint,
    0x34345c68 as ::core::ffi::c_uint,
    0xa5a5f451 as ::core::ffi::c_uint,
    0xe5e534d1 as ::core::ffi::c_uint,
    0xf1f108f9 as ::core::ffi::c_uint,
    0x717193e2 as ::core::ffi::c_uint,
    0xd8d873ab as ::core::ffi::c_uint,
    0x31315362 as ::core::ffi::c_uint,
    0x15153f2a as ::core::ffi::c_uint,
    0x4040c08 as ::core::ffi::c_uint,
    0xc7c75295 as ::core::ffi::c_uint,
    0x23236546 as ::core::ffi::c_uint,
    0xc3c35e9d as ::core::ffi::c_uint,
    0x18182830 as ::core::ffi::c_uint,
    0x9696a137 as ::core::ffi::c_uint,
    0x5050f0a as ::core::ffi::c_uint,
    0x9a9ab52f as ::core::ffi::c_uint,
    0x707090e as ::core::ffi::c_uint,
    0x12123624 as ::core::ffi::c_uint,
    0x80809b1b as ::core::ffi::c_uint,
    0xe2e23ddf as ::core::ffi::c_uint,
    0xebeb26cd as ::core::ffi::c_uint,
    0x2727694e as ::core::ffi::c_uint,
    0xb2b2cd7f as ::core::ffi::c_uint,
    0x75759fea as ::core::ffi::c_uint,
    0x9091b12 as ::core::ffi::c_uint,
    0x83839e1d as ::core::ffi::c_uint,
    0x2c2c7458 as ::core::ffi::c_uint,
    0x1a1a2e34 as ::core::ffi::c_uint,
    0x1b1b2d36 as ::core::ffi::c_uint,
    0x6e6eb2dc as ::core::ffi::c_uint,
    0x5a5aeeb4 as ::core::ffi::c_uint,
    0xa0a0fb5b as ::core::ffi::c_uint,
    0x5252f6a4 as ::core::ffi::c_uint,
    0x3b3b4d76 as ::core::ffi::c_uint,
    0xd6d661b7 as ::core::ffi::c_uint,
    0xb3b3ce7d as ::core::ffi::c_uint,
    0x29297b52 as ::core::ffi::c_uint,
    0xe3e33edd as ::core::ffi::c_uint,
    0x2f2f715e as ::core::ffi::c_uint,
    0x84849713 as ::core::ffi::c_uint,
    0x5353f5a6 as ::core::ffi::c_uint,
    0xd1d168b9 as ::core::ffi::c_uint,
    0 as ::core::ffi::c_uint,
    0xeded2cc1 as ::core::ffi::c_uint,
    0x20206040 as ::core::ffi::c_uint,
    0xfcfc1fe3 as ::core::ffi::c_uint,
    0xb1b1c879 as ::core::ffi::c_uint,
    0x5b5bedb6 as ::core::ffi::c_uint,
    0x6a6abed4 as ::core::ffi::c_uint,
    0xcbcb468d as ::core::ffi::c_uint,
    0xbebed967 as ::core::ffi::c_uint,
    0x39394b72 as ::core::ffi::c_uint,
    0x4a4ade94 as ::core::ffi::c_uint,
    0x4c4cd498 as ::core::ffi::c_uint,
    0x5858e8b0 as ::core::ffi::c_uint,
    0xcfcf4a85 as ::core::ffi::c_uint,
    0xd0d06bbb as ::core::ffi::c_uint,
    0xefef2ac5 as ::core::ffi::c_uint,
    0xaaaae54f as ::core::ffi::c_uint,
    0xfbfb16ed as ::core::ffi::c_uint,
    0x4343c586 as ::core::ffi::c_uint,
    0x4d4dd79a as ::core::ffi::c_uint,
    0x33335566 as ::core::ffi::c_uint,
    0x85859411 as ::core::ffi::c_uint,
    0x4545cf8a as ::core::ffi::c_uint,
    0xf9f910e9 as ::core::ffi::c_uint,
    0x2020604 as ::core::ffi::c_uint,
    0x7f7f81fe as ::core::ffi::c_uint,
    0x5050f0a0 as ::core::ffi::c_uint,
    0x3c3c4478 as ::core::ffi::c_uint,
    0x9f9fba25 as ::core::ffi::c_uint,
    0xa8a8e34b as ::core::ffi::c_uint,
    0x5151f3a2 as ::core::ffi::c_uint,
    0xa3a3fe5d as ::core::ffi::c_uint,
    0x4040c080 as ::core::ffi::c_uint,
    0x8f8f8a05 as ::core::ffi::c_uint,
    0x9292ad3f as ::core::ffi::c_uint,
    0x9d9dbc21 as ::core::ffi::c_uint,
    0x38384870 as ::core::ffi::c_uint,
    0xf5f504f1 as ::core::ffi::c_uint,
    0xbcbcdf63 as ::core::ffi::c_uint,
    0xb6b6c177 as ::core::ffi::c_uint,
    0xdada75af as ::core::ffi::c_uint,
    0x21216342 as ::core::ffi::c_uint,
    0x10103020 as ::core::ffi::c_uint,
    0xffff1ae5 as ::core::ffi::c_uint,
    0xf3f30efd as ::core::ffi::c_uint,
    0xd2d26dbf as ::core::ffi::c_uint,
    0xcdcd4c81 as ::core::ffi::c_uint,
    0xc0c1418 as ::core::ffi::c_uint,
    0x13133526 as ::core::ffi::c_uint,
    0xecec2fc3 as ::core::ffi::c_uint,
    0x5f5fe1be as ::core::ffi::c_uint,
    0x9797a235 as ::core::ffi::c_uint,
    0x4444cc88 as ::core::ffi::c_uint,
    0x1717392e as ::core::ffi::c_uint,
    0xc4c45793 as ::core::ffi::c_uint,
    0xa7a7f255 as ::core::ffi::c_uint,
    0x7e7e82fc as ::core::ffi::c_uint,
    0x3d3d477a as ::core::ffi::c_uint,
    0x6464acc8 as ::core::ffi::c_uint,
    0x5d5de7ba as ::core::ffi::c_uint,
    0x19192b32 as ::core::ffi::c_uint,
    0x737395e6 as ::core::ffi::c_uint,
    0x6060a0c0 as ::core::ffi::c_uint,
    0x81819819 as ::core::ffi::c_uint,
    0x4f4fd19e as ::core::ffi::c_uint,
    0xdcdc7fa3 as ::core::ffi::c_uint,
    0x22226644 as ::core::ffi::c_uint,
    0x2a2a7e54 as ::core::ffi::c_uint,
    0x9090ab3b as ::core::ffi::c_uint,
    0x8888830b as ::core::ffi::c_uint,
    0x4646ca8c as ::core::ffi::c_uint,
    0xeeee29c7 as ::core::ffi::c_uint,
    0xb8b8d36b as ::core::ffi::c_uint,
    0x14143c28 as ::core::ffi::c_uint,
    0xdede79a7 as ::core::ffi::c_uint,
    0x5e5ee2bc as ::core::ffi::c_uint,
    0xb0b1d16 as ::core::ffi::c_uint,
    0xdbdb76ad as ::core::ffi::c_uint,
    0xe0e03bdb as ::core::ffi::c_uint,
    0x32325664 as ::core::ffi::c_uint,
    0x3a3a4e74 as ::core::ffi::c_uint,
    0xa0a1e14 as ::core::ffi::c_uint,
    0x4949db92 as ::core::ffi::c_uint,
    0x6060a0c as ::core::ffi::c_uint,
    0x24246c48 as ::core::ffi::c_uint,
    0x5c5ce4b8 as ::core::ffi::c_uint,
    0xc2c25d9f as ::core::ffi::c_uint,
    0xd3d36ebd as ::core::ffi::c_uint,
    0xacacef43 as ::core::ffi::c_uint,
    0x6262a6c4 as ::core::ffi::c_uint,
    0x9191a839 as ::core::ffi::c_uint,
    0x9595a431 as ::core::ffi::c_uint,
    0xe4e437d3 as ::core::ffi::c_uint,
    0x79798bf2 as ::core::ffi::c_uint,
    0xe7e732d5 as ::core::ffi::c_uint,
    0xc8c8438b as ::core::ffi::c_uint,
    0x3737596e as ::core::ffi::c_uint,
    0x6d6db7da as ::core::ffi::c_uint,
    0x8d8d8c01 as ::core::ffi::c_uint,
    0xd5d564b1 as ::core::ffi::c_uint,
    0x4e4ed29c as ::core::ffi::c_uint,
    0xa9a9e049 as ::core::ffi::c_uint,
    0x6c6cb4d8 as ::core::ffi::c_uint,
    0x5656faac as ::core::ffi::c_uint,
    0xf4f407f3 as ::core::ffi::c_uint,
    0xeaea25cf as ::core::ffi::c_uint,
    0x6565afca as ::core::ffi::c_uint,
    0x7a7a8ef4 as ::core::ffi::c_uint,
    0xaeaee947 as ::core::ffi::c_uint,
    0x8081810 as ::core::ffi::c_uint,
    0xbabad56f as ::core::ffi::c_uint,
    0x787888f0 as ::core::ffi::c_uint,
    0x25256f4a as ::core::ffi::c_uint,
    0x2e2e725c as ::core::ffi::c_uint,
    0x1c1c2438 as ::core::ffi::c_uint,
    0xa6a6f157 as ::core::ffi::c_uint,
    0xb4b4c773 as ::core::ffi::c_uint,
    0xc6c65197 as ::core::ffi::c_uint,
    0xe8e823cb as ::core::ffi::c_uint,
    0xdddd7ca1 as ::core::ffi::c_uint,
    0x74749ce8 as ::core::ffi::c_uint,
    0x1f1f213e as ::core::ffi::c_uint,
    0x4b4bdd96 as ::core::ffi::c_uint,
    0xbdbddc61 as ::core::ffi::c_uint,
    0x8b8b860d as ::core::ffi::c_uint,
    0x8a8a850f as ::core::ffi::c_uint,
    0x707090e0 as ::core::ffi::c_uint,
    0x3e3e427c as ::core::ffi::c_uint,
    0xb5b5c471 as ::core::ffi::c_uint,
    0x6666aacc as ::core::ffi::c_uint,
    0x4848d890 as ::core::ffi::c_uint,
    0x3030506 as ::core::ffi::c_uint,
    0xf6f601f7 as ::core::ffi::c_uint,
    0xe0e121c as ::core::ffi::c_uint,
    0x6161a3c2 as ::core::ffi::c_uint,
    0x35355f6a as ::core::ffi::c_uint,
    0x5757f9ae as ::core::ffi::c_uint,
    0xb9b9d069 as ::core::ffi::c_uint,
    0x86869117 as ::core::ffi::c_uint,
    0xc1c15899 as ::core::ffi::c_uint,
    0x1d1d273a as ::core::ffi::c_uint,
    0x9e9eb927 as ::core::ffi::c_uint,
    0xe1e138d9 as ::core::ffi::c_uint,
    0xf8f813eb as ::core::ffi::c_uint,
    0x9898b32b as ::core::ffi::c_uint,
    0x11113322 as ::core::ffi::c_uint,
    0x6969bbd2 as ::core::ffi::c_uint,
    0xd9d970a9 as ::core::ffi::c_uint,
    0x8e8e8907 as ::core::ffi::c_uint,
    0x9494a733 as ::core::ffi::c_uint,
    0x9b9bb62d as ::core::ffi::c_uint,
    0x1e1e223c as ::core::ffi::c_uint,
    0x87879215 as ::core::ffi::c_uint,
    0xe9e920c9 as ::core::ffi::c_uint,
    0xcece4987 as ::core::ffi::c_uint,
    0x5555ffaa as ::core::ffi::c_uint,
    0x28287850 as ::core::ffi::c_uint,
    0xdfdf7aa5 as ::core::ffi::c_uint,
    0x8c8c8f03 as ::core::ffi::c_uint,
    0xa1a1f859 as ::core::ffi::c_uint,
    0x89898009 as ::core::ffi::c_uint,
    0xd0d171a as ::core::ffi::c_uint,
    0xbfbfda65 as ::core::ffi::c_uint,
    0xe6e631d7 as ::core::ffi::c_uint,
    0x4242c684 as ::core::ffi::c_uint,
    0x6868b8d0 as ::core::ffi::c_uint,
    0x4141c382 as ::core::ffi::c_uint,
    0x9999b029 as ::core::ffi::c_uint,
    0x2d2d775a as ::core::ffi::c_uint,
    0xf0f111e as ::core::ffi::c_uint,
    0xb0b0cb7b as ::core::ffi::c_uint,
    0x5454fca8 as ::core::ffi::c_uint,
    0xbbbbd66d as ::core::ffi::c_uint,
    0x16163a2c as ::core::ffi::c_uint,
];
static mut Te4: [uint32_t; 256] = [
    0x63636363 as ::core::ffi::c_uint,
    0x7c7c7c7c as ::core::ffi::c_uint,
    0x77777777 as ::core::ffi::c_uint,
    0x7b7b7b7b as ::core::ffi::c_uint,
    0xf2f2f2f2 as ::core::ffi::c_uint,
    0x6b6b6b6b as ::core::ffi::c_uint,
    0x6f6f6f6f as ::core::ffi::c_uint,
    0xc5c5c5c5 as ::core::ffi::c_uint,
    0x30303030 as ::core::ffi::c_uint,
    0x1010101 as ::core::ffi::c_uint,
    0x67676767 as ::core::ffi::c_uint,
    0x2b2b2b2b as ::core::ffi::c_uint,
    0xfefefefe as ::core::ffi::c_uint,
    0xd7d7d7d7 as ::core::ffi::c_uint,
    0xabababab as ::core::ffi::c_uint,
    0x76767676 as ::core::ffi::c_uint,
    0xcacacaca as ::core::ffi::c_uint,
    0x82828282 as ::core::ffi::c_uint,
    0xc9c9c9c9 as ::core::ffi::c_uint,
    0x7d7d7d7d as ::core::ffi::c_uint,
    0xfafafafa as ::core::ffi::c_uint,
    0x59595959 as ::core::ffi::c_uint,
    0x47474747 as ::core::ffi::c_uint,
    0xf0f0f0f0 as ::core::ffi::c_uint,
    0xadadadad as ::core::ffi::c_uint,
    0xd4d4d4d4 as ::core::ffi::c_uint,
    0xa2a2a2a2 as ::core::ffi::c_uint,
    0xafafafaf as ::core::ffi::c_uint,
    0x9c9c9c9c as ::core::ffi::c_uint,
    0xa4a4a4a4 as ::core::ffi::c_uint,
    0x72727272 as ::core::ffi::c_uint,
    0xc0c0c0c0 as ::core::ffi::c_uint,
    0xb7b7b7b7 as ::core::ffi::c_uint,
    0xfdfdfdfd as ::core::ffi::c_uint,
    0x93939393 as ::core::ffi::c_uint,
    0x26262626 as ::core::ffi::c_uint,
    0x36363636 as ::core::ffi::c_uint,
    0x3f3f3f3f as ::core::ffi::c_uint,
    0xf7f7f7f7 as ::core::ffi::c_uint,
    0xcccccccc as ::core::ffi::c_uint,
    0x34343434 as ::core::ffi::c_uint,
    0xa5a5a5a5 as ::core::ffi::c_uint,
    0xe5e5e5e5 as ::core::ffi::c_uint,
    0xf1f1f1f1 as ::core::ffi::c_uint,
    0x71717171 as ::core::ffi::c_uint,
    0xd8d8d8d8 as ::core::ffi::c_uint,
    0x31313131 as ::core::ffi::c_uint,
    0x15151515 as ::core::ffi::c_uint,
    0x4040404 as ::core::ffi::c_uint,
    0xc7c7c7c7 as ::core::ffi::c_uint,
    0x23232323 as ::core::ffi::c_uint,
    0xc3c3c3c3 as ::core::ffi::c_uint,
    0x18181818 as ::core::ffi::c_uint,
    0x96969696 as ::core::ffi::c_uint,
    0x5050505 as ::core::ffi::c_uint,
    0x9a9a9a9a as ::core::ffi::c_uint,
    0x7070707 as ::core::ffi::c_uint,
    0x12121212 as ::core::ffi::c_uint,
    0x80808080 as ::core::ffi::c_uint,
    0xe2e2e2e2 as ::core::ffi::c_uint,
    0xebebebeb as ::core::ffi::c_uint,
    0x27272727 as ::core::ffi::c_uint,
    0xb2b2b2b2 as ::core::ffi::c_uint,
    0x75757575 as ::core::ffi::c_uint,
    0x9090909 as ::core::ffi::c_uint,
    0x83838383 as ::core::ffi::c_uint,
    0x2c2c2c2c as ::core::ffi::c_uint,
    0x1a1a1a1a as ::core::ffi::c_uint,
    0x1b1b1b1b as ::core::ffi::c_uint,
    0x6e6e6e6e as ::core::ffi::c_uint,
    0x5a5a5a5a as ::core::ffi::c_uint,
    0xa0a0a0a0 as ::core::ffi::c_uint,
    0x52525252 as ::core::ffi::c_uint,
    0x3b3b3b3b as ::core::ffi::c_uint,
    0xd6d6d6d6 as ::core::ffi::c_uint,
    0xb3b3b3b3 as ::core::ffi::c_uint,
    0x29292929 as ::core::ffi::c_uint,
    0xe3e3e3e3 as ::core::ffi::c_uint,
    0x2f2f2f2f as ::core::ffi::c_uint,
    0x84848484 as ::core::ffi::c_uint,
    0x53535353 as ::core::ffi::c_uint,
    0xd1d1d1d1 as ::core::ffi::c_uint,
    0 as ::core::ffi::c_uint,
    0xedededed as ::core::ffi::c_uint,
    0x20202020 as ::core::ffi::c_uint,
    0xfcfcfcfc as ::core::ffi::c_uint,
    0xb1b1b1b1 as ::core::ffi::c_uint,
    0x5b5b5b5b as ::core::ffi::c_uint,
    0x6a6a6a6a as ::core::ffi::c_uint,
    0xcbcbcbcb as ::core::ffi::c_uint,
    0xbebebebe as ::core::ffi::c_uint,
    0x39393939 as ::core::ffi::c_uint,
    0x4a4a4a4a as ::core::ffi::c_uint,
    0x4c4c4c4c as ::core::ffi::c_uint,
    0x58585858 as ::core::ffi::c_uint,
    0xcfcfcfcf as ::core::ffi::c_uint,
    0xd0d0d0d0 as ::core::ffi::c_uint,
    0xefefefef as ::core::ffi::c_uint,
    0xaaaaaaaa as ::core::ffi::c_uint,
    0xfbfbfbfb as ::core::ffi::c_uint,
    0x43434343 as ::core::ffi::c_uint,
    0x4d4d4d4d as ::core::ffi::c_uint,
    0x33333333 as ::core::ffi::c_uint,
    0x85858585 as ::core::ffi::c_uint,
    0x45454545 as ::core::ffi::c_uint,
    0xf9f9f9f9 as ::core::ffi::c_uint,
    0x2020202 as ::core::ffi::c_uint,
    0x7f7f7f7f as ::core::ffi::c_uint,
    0x50505050 as ::core::ffi::c_uint,
    0x3c3c3c3c as ::core::ffi::c_uint,
    0x9f9f9f9f as ::core::ffi::c_uint,
    0xa8a8a8a8 as ::core::ffi::c_uint,
    0x51515151 as ::core::ffi::c_uint,
    0xa3a3a3a3 as ::core::ffi::c_uint,
    0x40404040 as ::core::ffi::c_uint,
    0x8f8f8f8f as ::core::ffi::c_uint,
    0x92929292 as ::core::ffi::c_uint,
    0x9d9d9d9d as ::core::ffi::c_uint,
    0x38383838 as ::core::ffi::c_uint,
    0xf5f5f5f5 as ::core::ffi::c_uint,
    0xbcbcbcbc as ::core::ffi::c_uint,
    0xb6b6b6b6 as ::core::ffi::c_uint,
    0xdadadada as ::core::ffi::c_uint,
    0x21212121 as ::core::ffi::c_uint,
    0x10101010 as ::core::ffi::c_uint,
    0xffffffff as ::core::ffi::c_uint,
    0xf3f3f3f3 as ::core::ffi::c_uint,
    0xd2d2d2d2 as ::core::ffi::c_uint,
    0xcdcdcdcd as ::core::ffi::c_uint,
    0xc0c0c0c as ::core::ffi::c_uint,
    0x13131313 as ::core::ffi::c_uint,
    0xecececec as ::core::ffi::c_uint,
    0x5f5f5f5f as ::core::ffi::c_uint,
    0x97979797 as ::core::ffi::c_uint,
    0x44444444 as ::core::ffi::c_uint,
    0x17171717 as ::core::ffi::c_uint,
    0xc4c4c4c4 as ::core::ffi::c_uint,
    0xa7a7a7a7 as ::core::ffi::c_uint,
    0x7e7e7e7e as ::core::ffi::c_uint,
    0x3d3d3d3d as ::core::ffi::c_uint,
    0x64646464 as ::core::ffi::c_uint,
    0x5d5d5d5d as ::core::ffi::c_uint,
    0x19191919 as ::core::ffi::c_uint,
    0x73737373 as ::core::ffi::c_uint,
    0x60606060 as ::core::ffi::c_uint,
    0x81818181 as ::core::ffi::c_uint,
    0x4f4f4f4f as ::core::ffi::c_uint,
    0xdcdcdcdc as ::core::ffi::c_uint,
    0x22222222 as ::core::ffi::c_uint,
    0x2a2a2a2a as ::core::ffi::c_uint,
    0x90909090 as ::core::ffi::c_uint,
    0x88888888 as ::core::ffi::c_uint,
    0x46464646 as ::core::ffi::c_uint,
    0xeeeeeeee as ::core::ffi::c_uint,
    0xb8b8b8b8 as ::core::ffi::c_uint,
    0x14141414 as ::core::ffi::c_uint,
    0xdededede as ::core::ffi::c_uint,
    0x5e5e5e5e as ::core::ffi::c_uint,
    0xb0b0b0b as ::core::ffi::c_uint,
    0xdbdbdbdb as ::core::ffi::c_uint,
    0xe0e0e0e0 as ::core::ffi::c_uint,
    0x32323232 as ::core::ffi::c_uint,
    0x3a3a3a3a as ::core::ffi::c_uint,
    0xa0a0a0a as ::core::ffi::c_uint,
    0x49494949 as ::core::ffi::c_uint,
    0x6060606 as ::core::ffi::c_uint,
    0x24242424 as ::core::ffi::c_uint,
    0x5c5c5c5c as ::core::ffi::c_uint,
    0xc2c2c2c2 as ::core::ffi::c_uint,
    0xd3d3d3d3 as ::core::ffi::c_uint,
    0xacacacac as ::core::ffi::c_uint,
    0x62626262 as ::core::ffi::c_uint,
    0x91919191 as ::core::ffi::c_uint,
    0x95959595 as ::core::ffi::c_uint,
    0xe4e4e4e4 as ::core::ffi::c_uint,
    0x79797979 as ::core::ffi::c_uint,
    0xe7e7e7e7 as ::core::ffi::c_uint,
    0xc8c8c8c8 as ::core::ffi::c_uint,
    0x37373737 as ::core::ffi::c_uint,
    0x6d6d6d6d as ::core::ffi::c_uint,
    0x8d8d8d8d as ::core::ffi::c_uint,
    0xd5d5d5d5 as ::core::ffi::c_uint,
    0x4e4e4e4e as ::core::ffi::c_uint,
    0xa9a9a9a9 as ::core::ffi::c_uint,
    0x6c6c6c6c as ::core::ffi::c_uint,
    0x56565656 as ::core::ffi::c_uint,
    0xf4f4f4f4 as ::core::ffi::c_uint,
    0xeaeaeaea as ::core::ffi::c_uint,
    0x65656565 as ::core::ffi::c_uint,
    0x7a7a7a7a as ::core::ffi::c_uint,
    0xaeaeaeae as ::core::ffi::c_uint,
    0x8080808 as ::core::ffi::c_uint,
    0xbabababa as ::core::ffi::c_uint,
    0x78787878 as ::core::ffi::c_uint,
    0x25252525 as ::core::ffi::c_uint,
    0x2e2e2e2e as ::core::ffi::c_uint,
    0x1c1c1c1c as ::core::ffi::c_uint,
    0xa6a6a6a6 as ::core::ffi::c_uint,
    0xb4b4b4b4 as ::core::ffi::c_uint,
    0xc6c6c6c6 as ::core::ffi::c_uint,
    0xe8e8e8e8 as ::core::ffi::c_uint,
    0xdddddddd as ::core::ffi::c_uint,
    0x74747474 as ::core::ffi::c_uint,
    0x1f1f1f1f as ::core::ffi::c_uint,
    0x4b4b4b4b as ::core::ffi::c_uint,
    0xbdbdbdbd as ::core::ffi::c_uint,
    0x8b8b8b8b as ::core::ffi::c_uint,
    0x8a8a8a8a as ::core::ffi::c_uint,
    0x70707070 as ::core::ffi::c_uint,
    0x3e3e3e3e as ::core::ffi::c_uint,
    0xb5b5b5b5 as ::core::ffi::c_uint,
    0x66666666 as ::core::ffi::c_uint,
    0x48484848 as ::core::ffi::c_uint,
    0x3030303 as ::core::ffi::c_uint,
    0xf6f6f6f6 as ::core::ffi::c_uint,
    0xe0e0e0e as ::core::ffi::c_uint,
    0x61616161 as ::core::ffi::c_uint,
    0x35353535 as ::core::ffi::c_uint,
    0x57575757 as ::core::ffi::c_uint,
    0xb9b9b9b9 as ::core::ffi::c_uint,
    0x86868686 as ::core::ffi::c_uint,
    0xc1c1c1c1 as ::core::ffi::c_uint,
    0x1d1d1d1d as ::core::ffi::c_uint,
    0x9e9e9e9e as ::core::ffi::c_uint,
    0xe1e1e1e1 as ::core::ffi::c_uint,
    0xf8f8f8f8 as ::core::ffi::c_uint,
    0x98989898 as ::core::ffi::c_uint,
    0x11111111 as ::core::ffi::c_uint,
    0x69696969 as ::core::ffi::c_uint,
    0xd9d9d9d9 as ::core::ffi::c_uint,
    0x8e8e8e8e as ::core::ffi::c_uint,
    0x94949494 as ::core::ffi::c_uint,
    0x9b9b9b9b as ::core::ffi::c_uint,
    0x1e1e1e1e as ::core::ffi::c_uint,
    0x87878787 as ::core::ffi::c_uint,
    0xe9e9e9e9 as ::core::ffi::c_uint,
    0xcececece as ::core::ffi::c_uint,
    0x55555555 as ::core::ffi::c_uint,
    0x28282828 as ::core::ffi::c_uint,
    0xdfdfdfdf as ::core::ffi::c_uint,
    0x8c8c8c8c as ::core::ffi::c_uint,
    0xa1a1a1a1 as ::core::ffi::c_uint,
    0x89898989 as ::core::ffi::c_uint,
    0xd0d0d0d as ::core::ffi::c_uint,
    0xbfbfbfbf as ::core::ffi::c_uint,
    0xe6e6e6e6 as ::core::ffi::c_uint,
    0x42424242 as ::core::ffi::c_uint,
    0x68686868 as ::core::ffi::c_uint,
    0x41414141 as ::core::ffi::c_uint,
    0x99999999 as ::core::ffi::c_uint,
    0x2d2d2d2d as ::core::ffi::c_uint,
    0xf0f0f0f as ::core::ffi::c_uint,
    0xb0b0b0b0 as ::core::ffi::c_uint,
    0x54545454 as ::core::ffi::c_uint,
    0xbbbbbbbb as ::core::ffi::c_uint,
    0x16161616 as ::core::ffi::c_uint,
];
static mut rcon: [uint32_t; 10] = [
    0x1000000 as ::core::ffi::c_int as uint32_t,
    0x2000000 as ::core::ffi::c_int as uint32_t,
    0x4000000 as ::core::ffi::c_int as uint32_t,
    0x8000000 as ::core::ffi::c_int as uint32_t,
    0x10000000 as ::core::ffi::c_int as uint32_t,
    0x20000000 as ::core::ffi::c_int as uint32_t,
    0x40000000 as ::core::ffi::c_int as uint32_t,
    0x80000000 as ::core::ffi::c_uint,
    0x1b000000 as ::core::ffi::c_int as uint32_t,
    0x36000000 as ::core::ffi::c_int as uint32_t,
];
unsafe extern "C-unwind" fn rijndaelSetupEncrypt(
    mut rk: *mut uint32_t,
    mut key: *const uint8_t,
    mut keybits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut temp: uint32_t = 0;
    *rk.offset(0 as ::core::ffi::c_int as isize) = (*key.offset(0 as ::core::ffi::c_int as isize)
        as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*key.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int
        ^ (*key.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        ^ *key.offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    *rk.offset(1 as ::core::ffi::c_int as isize) = (*key
        .offset(4 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize)
        as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*key
            .offset(4 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        ^ (*key
            .offset(4 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        ^ *key
            .offset(4 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    *rk.offset(2 as ::core::ffi::c_int as isize) = (*key
        .offset(8 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize)
        as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*key
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        ^ (*key
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        ^ *key
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    *rk.offset(3 as ::core::ffi::c_int as isize) = (*key
        .offset(12 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize)
        as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*key
            .offset(12 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        ^ (*key
            .offset(12 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        ^ *key
            .offset(12 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    if keybits == 128 as ::core::ffi::c_int {
        loop {
            temp = *rk.offset(3 as ::core::ffi::c_int as isize);
            *rk.offset(4 as ::core::ffi::c_int as isize) = *rk
                .offset(0 as ::core::ffi::c_int as isize)
                ^ Te4[(temp >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                    & 0xff000000 as uint32_t
                ^ Te4[(temp >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                    & 0xff0000 as uint32_t
                ^ Te4[(temp & 0xff as uint32_t) as usize] & 0xff00 as uint32_t
                ^ Te4[(temp >> 24 as ::core::ffi::c_int) as usize] & 0xff as uint32_t
                ^ rcon[i as usize];
            *rk.offset(5 as ::core::ffi::c_int as isize) = *rk
                .offset(1 as ::core::ffi::c_int as isize)
                ^ *rk.offset(4 as ::core::ffi::c_int as isize);
            *rk.offset(6 as ::core::ffi::c_int as isize) = *rk
                .offset(2 as ::core::ffi::c_int as isize)
                ^ *rk.offset(5 as ::core::ffi::c_int as isize);
            *rk.offset(7 as ::core::ffi::c_int as isize) = *rk
                .offset(3 as ::core::ffi::c_int as isize)
                ^ *rk.offset(6 as ::core::ffi::c_int as isize);
            i = i.wrapping_add(1);
            if i == 10 as ::core::ffi::c_uint {
                return 10 as ::core::ffi::c_int;
            }
            rk = rk.offset(4 as ::core::ffi::c_int as isize);
        }
    }
    *rk.offset(4 as ::core::ffi::c_int as isize) = (*key
        .offset(16 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize)
        as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*key
            .offset(16 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        ^ (*key
            .offset(16 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        ^ *key
            .offset(16 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    *rk.offset(5 as ::core::ffi::c_int as isize) = (*key
        .offset(20 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize)
        as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*key
            .offset(20 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        ^ (*key
            .offset(20 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        ^ *key
            .offset(20 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    if keybits == 192 as ::core::ffi::c_int {
        loop {
            temp = *rk.offset(5 as ::core::ffi::c_int as isize);
            *rk.offset(6 as ::core::ffi::c_int as isize) = *rk
                .offset(0 as ::core::ffi::c_int as isize)
                ^ Te4[(temp >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                    & 0xff000000 as uint32_t
                ^ Te4[(temp >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                    & 0xff0000 as uint32_t
                ^ Te4[(temp & 0xff as uint32_t) as usize] & 0xff00 as uint32_t
                ^ Te4[(temp >> 24 as ::core::ffi::c_int) as usize] & 0xff as uint32_t
                ^ rcon[i as usize];
            *rk.offset(7 as ::core::ffi::c_int as isize) = *rk
                .offset(1 as ::core::ffi::c_int as isize)
                ^ *rk.offset(6 as ::core::ffi::c_int as isize);
            *rk.offset(8 as ::core::ffi::c_int as isize) = *rk
                .offset(2 as ::core::ffi::c_int as isize)
                ^ *rk.offset(7 as ::core::ffi::c_int as isize);
            *rk.offset(9 as ::core::ffi::c_int as isize) = *rk
                .offset(3 as ::core::ffi::c_int as isize)
                ^ *rk.offset(8 as ::core::ffi::c_int as isize);
            i = i.wrapping_add(1);
            if i == 8 as ::core::ffi::c_uint {
                return 12 as ::core::ffi::c_int;
            }
            *rk.offset(10 as ::core::ffi::c_int as isize) = *rk
                .offset(4 as ::core::ffi::c_int as isize)
                ^ *rk.offset(9 as ::core::ffi::c_int as isize);
            *rk.offset(11 as ::core::ffi::c_int as isize) = *rk
                .offset(5 as ::core::ffi::c_int as isize)
                ^ *rk.offset(10 as ::core::ffi::c_int as isize);
            rk = rk.offset(6 as ::core::ffi::c_int as isize);
        }
    }
    *rk.offset(6 as ::core::ffi::c_int as isize) = (*key
        .offset(24 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize)
        as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*key
            .offset(24 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        ^ (*key
            .offset(24 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        ^ *key
            .offset(24 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    *rk.offset(7 as ::core::ffi::c_int as isize) = (*key
        .offset(28 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize)
        as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*key
            .offset(28 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        ^ (*key
            .offset(28 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        ^ *key
            .offset(28 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    if keybits == 256 as ::core::ffi::c_int {
        loop {
            temp = *rk.offset(7 as ::core::ffi::c_int as isize);
            *rk.offset(8 as ::core::ffi::c_int as isize) = *rk
                .offset(0 as ::core::ffi::c_int as isize)
                ^ Te4[(temp >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                    & 0xff000000 as uint32_t
                ^ Te4[(temp >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                    & 0xff0000 as uint32_t
                ^ Te4[(temp & 0xff as uint32_t) as usize] & 0xff00 as uint32_t
                ^ Te4[(temp >> 24 as ::core::ffi::c_int) as usize] & 0xff as uint32_t
                ^ rcon[i as usize];
            *rk.offset(9 as ::core::ffi::c_int as isize) = *rk
                .offset(1 as ::core::ffi::c_int as isize)
                ^ *rk.offset(8 as ::core::ffi::c_int as isize);
            *rk.offset(10 as ::core::ffi::c_int as isize) = *rk
                .offset(2 as ::core::ffi::c_int as isize)
                ^ *rk.offset(9 as ::core::ffi::c_int as isize);
            *rk.offset(11 as ::core::ffi::c_int as isize) = *rk
                .offset(3 as ::core::ffi::c_int as isize)
                ^ *rk.offset(10 as ::core::ffi::c_int as isize);
            i = i.wrapping_add(1);
            if i == 7 as ::core::ffi::c_uint {
                return 14 as ::core::ffi::c_int;
            }
            temp = *rk.offset(11 as ::core::ffi::c_int as isize);
            *rk.offset(12 as ::core::ffi::c_int as isize) = *rk
                .offset(4 as ::core::ffi::c_int as isize)
                ^ Te4[(temp >> 24 as ::core::ffi::c_int) as usize] & 0xff000000 as uint32_t
                ^ Te4[(temp >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                    & 0xff0000 as uint32_t
                ^ Te4[(temp >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                    & 0xff00 as uint32_t
                ^ Te4[(temp & 0xff as uint32_t) as usize] & 0xff as uint32_t;
            *rk.offset(13 as ::core::ffi::c_int as isize) = *rk
                .offset(5 as ::core::ffi::c_int as isize)
                ^ *rk.offset(12 as ::core::ffi::c_int as isize);
            *rk.offset(14 as ::core::ffi::c_int as isize) = *rk
                .offset(6 as ::core::ffi::c_int as isize)
                ^ *rk.offset(13 as ::core::ffi::c_int as isize);
            *rk.offset(15 as ::core::ffi::c_int as isize) = *rk
                .offset(7 as ::core::ffi::c_int as isize)
                ^ *rk.offset(14 as ::core::ffi::c_int as isize);
            rk = rk.offset(8 as ::core::ffi::c_int as isize);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C-unwind" fn rijndaelEncrypt(
    mut rk: *const uint32_t,
    mut nrounds: ::core::ffi::c_int,
    mut plaintext: *const uint8_t,
    mut ciphertext: *mut uint8_t,
) {
    let mut s0: uint32_t = 0;
    let mut s1: uint32_t = 0;
    let mut s2: uint32_t = 0;
    let mut s3: uint32_t = 0;
    let mut t0: uint32_t = 0;
    let mut t1: uint32_t = 0;
    let mut t2: uint32_t = 0;
    let mut t3: uint32_t = 0;
    s0 = (*plaintext.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*plaintext.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        ^ (*plaintext.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        ^ *plaintext.offset(3 as ::core::ffi::c_int as isize) as uint32_t
        ^ *rk.offset(0 as ::core::ffi::c_int as isize);
    s1 = (*plaintext
        .offset(4 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*plaintext
            .offset(4 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        ^ (*plaintext
            .offset(4 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        ^ *plaintext
            .offset(4 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as uint32_t
        ^ *rk.offset(1 as ::core::ffi::c_int as isize);
    s2 = (*plaintext
        .offset(8 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*plaintext
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        ^ (*plaintext
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        ^ *plaintext
            .offset(8 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as uint32_t
        ^ *rk.offset(2 as ::core::ffi::c_int as isize);
    s3 = (*plaintext
        .offset(12 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) as uint32_t)
        << 24 as ::core::ffi::c_int
        ^ (*plaintext
            .offset(12 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as uint32_t)
            << 16 as ::core::ffi::c_int
        ^ (*plaintext
            .offset(12 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as uint32_t)
            << 8 as ::core::ffi::c_int
        ^ *plaintext
            .offset(12 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as uint32_t
        ^ *rk.offset(3 as ::core::ffi::c_int as isize);
    t0 = Te0[(s0 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s3 & 0xff as uint32_t) as usize]
        ^ *rk.offset(4 as ::core::ffi::c_int as isize);
    t1 = Te0[(s1 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s0 & 0xff as uint32_t) as usize]
        ^ *rk.offset(5 as ::core::ffi::c_int as isize);
    t2 = Te0[(s2 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s1 & 0xff as uint32_t) as usize]
        ^ *rk.offset(6 as ::core::ffi::c_int as isize);
    t3 = Te0[(s3 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s2 & 0xff as uint32_t) as usize]
        ^ *rk.offset(7 as ::core::ffi::c_int as isize);
    s0 = Te0[(t0 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t3 & 0xff as uint32_t) as usize]
        ^ *rk.offset(8 as ::core::ffi::c_int as isize);
    s1 = Te0[(t1 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t0 & 0xff as uint32_t) as usize]
        ^ *rk.offset(9 as ::core::ffi::c_int as isize);
    s2 = Te0[(t2 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t1 & 0xff as uint32_t) as usize]
        ^ *rk.offset(10 as ::core::ffi::c_int as isize);
    s3 = Te0[(t3 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t2 & 0xff as uint32_t) as usize]
        ^ *rk.offset(11 as ::core::ffi::c_int as isize);
    t0 = Te0[(s0 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s3 & 0xff as uint32_t) as usize]
        ^ *rk.offset(12 as ::core::ffi::c_int as isize);
    t1 = Te0[(s1 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s0 & 0xff as uint32_t) as usize]
        ^ *rk.offset(13 as ::core::ffi::c_int as isize);
    t2 = Te0[(s2 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s1 & 0xff as uint32_t) as usize]
        ^ *rk.offset(14 as ::core::ffi::c_int as isize);
    t3 = Te0[(s3 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s2 & 0xff as uint32_t) as usize]
        ^ *rk.offset(15 as ::core::ffi::c_int as isize);
    s0 = Te0[(t0 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t3 & 0xff as uint32_t) as usize]
        ^ *rk.offset(16 as ::core::ffi::c_int as isize);
    s1 = Te0[(t1 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t0 & 0xff as uint32_t) as usize]
        ^ *rk.offset(17 as ::core::ffi::c_int as isize);
    s2 = Te0[(t2 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t1 & 0xff as uint32_t) as usize]
        ^ *rk.offset(18 as ::core::ffi::c_int as isize);
    s3 = Te0[(t3 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t2 & 0xff as uint32_t) as usize]
        ^ *rk.offset(19 as ::core::ffi::c_int as isize);
    t0 = Te0[(s0 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s3 & 0xff as uint32_t) as usize]
        ^ *rk.offset(20 as ::core::ffi::c_int as isize);
    t1 = Te0[(s1 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s0 & 0xff as uint32_t) as usize]
        ^ *rk.offset(21 as ::core::ffi::c_int as isize);
    t2 = Te0[(s2 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s1 & 0xff as uint32_t) as usize]
        ^ *rk.offset(22 as ::core::ffi::c_int as isize);
    t3 = Te0[(s3 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s2 & 0xff as uint32_t) as usize]
        ^ *rk.offset(23 as ::core::ffi::c_int as isize);
    s0 = Te0[(t0 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t3 & 0xff as uint32_t) as usize]
        ^ *rk.offset(24 as ::core::ffi::c_int as isize);
    s1 = Te0[(t1 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t0 & 0xff as uint32_t) as usize]
        ^ *rk.offset(25 as ::core::ffi::c_int as isize);
    s2 = Te0[(t2 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t1 & 0xff as uint32_t) as usize]
        ^ *rk.offset(26 as ::core::ffi::c_int as isize);
    s3 = Te0[(t3 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t2 & 0xff as uint32_t) as usize]
        ^ *rk.offset(27 as ::core::ffi::c_int as isize);
    t0 = Te0[(s0 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s3 & 0xff as uint32_t) as usize]
        ^ *rk.offset(28 as ::core::ffi::c_int as isize);
    t1 = Te0[(s1 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s0 & 0xff as uint32_t) as usize]
        ^ *rk.offset(29 as ::core::ffi::c_int as isize);
    t2 = Te0[(s2 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s1 & 0xff as uint32_t) as usize]
        ^ *rk.offset(30 as ::core::ffi::c_int as isize);
    t3 = Te0[(s3 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s2 & 0xff as uint32_t) as usize]
        ^ *rk.offset(31 as ::core::ffi::c_int as isize);
    s0 = Te0[(t0 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t3 & 0xff as uint32_t) as usize]
        ^ *rk.offset(32 as ::core::ffi::c_int as isize);
    s1 = Te0[(t1 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t0 & 0xff as uint32_t) as usize]
        ^ *rk.offset(33 as ::core::ffi::c_int as isize);
    s2 = Te0[(t2 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t1 & 0xff as uint32_t) as usize]
        ^ *rk.offset(34 as ::core::ffi::c_int as isize);
    s3 = Te0[(t3 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(t0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(t1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(t2 & 0xff as uint32_t) as usize]
        ^ *rk.offset(35 as ::core::ffi::c_int as isize);
    t0 = Te0[(s0 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s3 & 0xff as uint32_t) as usize]
        ^ *rk.offset(36 as ::core::ffi::c_int as isize);
    t1 = Te0[(s1 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s0 & 0xff as uint32_t) as usize]
        ^ *rk.offset(37 as ::core::ffi::c_int as isize);
    t2 = Te0[(s2 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s1 & 0xff as uint32_t) as usize]
        ^ *rk.offset(38 as ::core::ffi::c_int as isize);
    t3 = Te0[(s3 >> 24 as ::core::ffi::c_int) as usize]
        ^ Te1[(s0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te2[(s1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
        ^ Te3[(s2 & 0xff as uint32_t) as usize]
        ^ *rk.offset(39 as ::core::ffi::c_int as isize);
    if nrounds > 10 as ::core::ffi::c_int {
        s0 = Te0[(t0 >> 24 as ::core::ffi::c_int) as usize]
            ^ Te1[(t1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te2[(t2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te3[(t3 & 0xff as uint32_t) as usize]
            ^ *rk.offset(40 as ::core::ffi::c_int as isize);
        s1 = Te0[(t1 >> 24 as ::core::ffi::c_int) as usize]
            ^ Te1[(t2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te2[(t3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te3[(t0 & 0xff as uint32_t) as usize]
            ^ *rk.offset(41 as ::core::ffi::c_int as isize);
        s2 = Te0[(t2 >> 24 as ::core::ffi::c_int) as usize]
            ^ Te1[(t3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te2[(t0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te3[(t1 & 0xff as uint32_t) as usize]
            ^ *rk.offset(42 as ::core::ffi::c_int as isize);
        s3 = Te0[(t3 >> 24 as ::core::ffi::c_int) as usize]
            ^ Te1[(t0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te2[(t1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te3[(t2 & 0xff as uint32_t) as usize]
            ^ *rk.offset(43 as ::core::ffi::c_int as isize);
        t0 = Te0[(s0 >> 24 as ::core::ffi::c_int) as usize]
            ^ Te1[(s1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te2[(s2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te3[(s3 & 0xff as uint32_t) as usize]
            ^ *rk.offset(44 as ::core::ffi::c_int as isize);
        t1 = Te0[(s1 >> 24 as ::core::ffi::c_int) as usize]
            ^ Te1[(s2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te2[(s3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te3[(s0 & 0xff as uint32_t) as usize]
            ^ *rk.offset(45 as ::core::ffi::c_int as isize);
        t2 = Te0[(s2 >> 24 as ::core::ffi::c_int) as usize]
            ^ Te1[(s3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te2[(s0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te3[(s1 & 0xff as uint32_t) as usize]
            ^ *rk.offset(46 as ::core::ffi::c_int as isize);
        t3 = Te0[(s3 >> 24 as ::core::ffi::c_int) as usize]
            ^ Te1[(s0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te2[(s1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
            ^ Te3[(s2 & 0xff as uint32_t) as usize]
            ^ *rk.offset(47 as ::core::ffi::c_int as isize);
        if nrounds > 12 as ::core::ffi::c_int {
            s0 = Te0[(t0 >> 24 as ::core::ffi::c_int) as usize]
                ^ Te1[(t1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te2[(t2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te3[(t3 & 0xff as uint32_t) as usize]
                ^ *rk.offset(48 as ::core::ffi::c_int as isize);
            s1 = Te0[(t1 >> 24 as ::core::ffi::c_int) as usize]
                ^ Te1[(t2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te2[(t3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te3[(t0 & 0xff as uint32_t) as usize]
                ^ *rk.offset(49 as ::core::ffi::c_int as isize);
            s2 = Te0[(t2 >> 24 as ::core::ffi::c_int) as usize]
                ^ Te1[(t3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te2[(t0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te3[(t1 & 0xff as uint32_t) as usize]
                ^ *rk.offset(50 as ::core::ffi::c_int as isize);
            s3 = Te0[(t3 >> 24 as ::core::ffi::c_int) as usize]
                ^ Te1[(t0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te2[(t1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te3[(t2 & 0xff as uint32_t) as usize]
                ^ *rk.offset(51 as ::core::ffi::c_int as isize);
            t0 = Te0[(s0 >> 24 as ::core::ffi::c_int) as usize]
                ^ Te1[(s1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te2[(s2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te3[(s3 & 0xff as uint32_t) as usize]
                ^ *rk.offset(52 as ::core::ffi::c_int as isize);
            t1 = Te0[(s1 >> 24 as ::core::ffi::c_int) as usize]
                ^ Te1[(s2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te2[(s3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te3[(s0 & 0xff as uint32_t) as usize]
                ^ *rk.offset(53 as ::core::ffi::c_int as isize);
            t2 = Te0[(s2 >> 24 as ::core::ffi::c_int) as usize]
                ^ Te1[(s3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te2[(s0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te3[(s1 & 0xff as uint32_t) as usize]
                ^ *rk.offset(54 as ::core::ffi::c_int as isize);
            t3 = Te0[(s3 >> 24 as ::core::ffi::c_int) as usize]
                ^ Te1[(s0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te2[(s1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize]
                ^ Te3[(s2 & 0xff as uint32_t) as usize]
                ^ *rk.offset(55 as ::core::ffi::c_int as isize);
        }
    }
    rk = rk.offset((nrounds << 2 as ::core::ffi::c_int) as isize);
    s0 = Te4[(t0 >> 24 as ::core::ffi::c_int) as usize] & 0xff000000 as uint32_t
        ^ Te4[(t1 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize] & 0xff0000 as uint32_t
        ^ Te4[(t2 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize] & 0xff00 as uint32_t
        ^ Te4[(t3 & 0xff as uint32_t) as usize] & 0xff as uint32_t
        ^ *rk.offset(0 as ::core::ffi::c_int as isize);
    *ciphertext.offset(0 as ::core::ffi::c_int as isize) =
        (s0 >> 24 as ::core::ffi::c_int) as uint8_t;
    *ciphertext.offset(1 as ::core::ffi::c_int as isize) =
        (s0 >> 16 as ::core::ffi::c_int) as uint8_t;
    *ciphertext.offset(2 as ::core::ffi::c_int as isize) =
        (s0 >> 8 as ::core::ffi::c_int) as uint8_t;
    *ciphertext.offset(3 as ::core::ffi::c_int as isize) = s0 as uint8_t;
    s1 = Te4[(t1 >> 24 as ::core::ffi::c_int) as usize] & 0xff000000 as uint32_t
        ^ Te4[(t2 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize] & 0xff0000 as uint32_t
        ^ Te4[(t3 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize] & 0xff00 as uint32_t
        ^ Te4[(t0 & 0xff as uint32_t) as usize] & 0xff as uint32_t
        ^ *rk.offset(1 as ::core::ffi::c_int as isize);
    *ciphertext
        .offset(4 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) = (s1 >> 24 as ::core::ffi::c_int) as uint8_t;
    *ciphertext
        .offset(4 as ::core::ffi::c_int as isize)
        .offset(1 as ::core::ffi::c_int as isize) = (s1 >> 16 as ::core::ffi::c_int) as uint8_t;
    *ciphertext
        .offset(4 as ::core::ffi::c_int as isize)
        .offset(2 as ::core::ffi::c_int as isize) = (s1 >> 8 as ::core::ffi::c_int) as uint8_t;
    *ciphertext
        .offset(4 as ::core::ffi::c_int as isize)
        .offset(3 as ::core::ffi::c_int as isize) = s1 as uint8_t;
    s2 = Te4[(t2 >> 24 as ::core::ffi::c_int) as usize] & 0xff000000 as uint32_t
        ^ Te4[(t3 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize] & 0xff0000 as uint32_t
        ^ Te4[(t0 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize] & 0xff00 as uint32_t
        ^ Te4[(t1 & 0xff as uint32_t) as usize] & 0xff as uint32_t
        ^ *rk.offset(2 as ::core::ffi::c_int as isize);
    *ciphertext
        .offset(8 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) = (s2 >> 24 as ::core::ffi::c_int) as uint8_t;
    *ciphertext
        .offset(8 as ::core::ffi::c_int as isize)
        .offset(1 as ::core::ffi::c_int as isize) = (s2 >> 16 as ::core::ffi::c_int) as uint8_t;
    *ciphertext
        .offset(8 as ::core::ffi::c_int as isize)
        .offset(2 as ::core::ffi::c_int as isize) = (s2 >> 8 as ::core::ffi::c_int) as uint8_t;
    *ciphertext
        .offset(8 as ::core::ffi::c_int as isize)
        .offset(3 as ::core::ffi::c_int as isize) = s2 as uint8_t;
    s3 = Te4[(t3 >> 24 as ::core::ffi::c_int) as usize] & 0xff000000 as uint32_t
        ^ Te4[(t0 >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as usize] & 0xff0000 as uint32_t
        ^ Te4[(t1 >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as usize] & 0xff00 as uint32_t
        ^ Te4[(t2 & 0xff as uint32_t) as usize] & 0xff as uint32_t
        ^ *rk.offset(3 as ::core::ffi::c_int as isize);
    *ciphertext
        .offset(12 as ::core::ffi::c_int as isize)
        .offset(0 as ::core::ffi::c_int as isize) = (s3 >> 24 as ::core::ffi::c_int) as uint8_t;
    *ciphertext
        .offset(12 as ::core::ffi::c_int as isize)
        .offset(1 as ::core::ffi::c_int as isize) = (s3 >> 16 as ::core::ffi::c_int) as uint8_t;
    *ciphertext
        .offset(12 as ::core::ffi::c_int as isize)
        .offset(2 as ::core::ffi::c_int as isize) = (s3 >> 8 as ::core::ffi::c_int) as uint8_t;
    *ciphertext
        .offset(12 as ::core::ffi::c_int as isize)
        .offset(3 as ::core::ffi::c_int as isize) = s3 as uint8_t;
}
unsafe extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __b: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __len: size_t,
    ) -> *mut ::core::ffi::c_void;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_MD5_init(mut ctx: *mut MD5_CONTEXT) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { MD5_init(ctx) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_MD5_write(
    mut hd: *mut MD5_CONTEXT,
    mut inbuf: *const ::core::ffi::c_uchar,
    mut inlen: ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { MD5_write(hd, inbuf, inlen) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_MD5_final(
    mut outbuf: *mut ::core::ffi::c_uchar,
    mut hd: *mut MD5_CONTEXT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { MD5_final(outbuf, hd) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA256_init(mut hd: *mut SHA256_CONTEXT) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA256_init(hd) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA256_write(
    mut hd: *mut SHA256_CONTEXT,
    mut inbuf: *const ::core::ffi::c_uchar,
    mut inlen: ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA256_write(hd, inbuf, inlen) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA256_final(
    mut outbuf: *mut ::core::ffi::c_uchar,
    mut hd: *mut SHA256_CONTEXT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA256_final(outbuf, hd) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA512_init(mut ctx: *mut SHA512_CONTEXT) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA512_init(ctx) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA384_init(mut ctx: *mut SHA512_CONTEXT) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA384_init(ctx) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA512_write(
    mut hd: *mut SHA512_CONTEXT,
    mut inbuf: *const ::core::ffi::c_uchar,
    mut inlen: ::core::ffi::c_uint,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA512_write(hd, inbuf, inlen) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_SHA512_final(
    mut outbuf: *mut ::core::ffi::c_uchar,
    mut hd: *mut SHA512_CONTEXT,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { SHA512_final(outbuf, hd) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_ARC4(
    mut ctx: *mut ARC4_CONTEXT,
    mut len: ::core::ffi::c_uint,
    mut inbuf: *const ::core::ffi::c_uchar,
    mut outbuf: *mut ::core::ffi::c_uchar,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ARC4(ctx, len, inbuf, outbuf) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_ARC4_set_key(
    mut ctx: *mut ARC4_CONTEXT,
    mut keylen: ::core::ffi::c_uint,
    mut key: *const ::core::ffi::c_uchar,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { ARC4_set_key(ctx, keylen, key) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_AES_ecb_encrypt(
    mut key: *const ::core::ffi::c_uchar,
    mut key_len: size_t,
    mut plain: *const ::core::ffi::c_uchar,
    mut plain_len: size_t,
    mut cipher: *mut *mut ::core::ffi::c_uchar,
    mut cipher_len: *mut size_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        AES_ecb_encrypt(key, key_len, plain, plain_len, cipher, cipher_len)
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_AES_cbc_encrypt_tectonic(
    mut key: *const ::core::ffi::c_uchar,
    mut key_len: size_t,
    mut iv: *const ::core::ffi::c_uchar,
    mut padding: ::core::ffi::c_int,
    mut plain: *const ::core::ffi::c_uchar,
    mut plain_len: size_t,
    mut cipher: *mut *mut ::core::ffi::c_uchar,
    mut cipher_len: *mut size_t,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe {
        AES_cbc_encrypt_tectonic(
            key, key_len, iv, padding, plain, plain_len, cipher, cipher_len,
        )
    }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
