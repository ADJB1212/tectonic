/*
   A C-program for MT19937, with initialization improved 2002/1/26.
   Coded by Takuji Nishimura and Makoto Matsumoto.

   Before using, initialize the state by using init_genrand(seed)
   or init_by_array(init_key, key_length).

   Copyright (C) 1997 - 2002, Makoto Matsumoto and Takuji Nishimura,
   All rights reserved.

   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

     1. Redistributions of source code must retain the above copyright
        notice, this list of conditions and the following disclaimer.

     2. Redistributions in binary form must reproduce the above copyright
        notice, this list of conditions and the following disclaimer in the
        documentation and/or other materials provided with the distribution.

     3. The names of its contributors may not be used to endorse or promote
        products derived from this software without specific prior written
        permission.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR
   CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


   Any feedback is very welcome.
   http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/emt.html
   email: m-mat @ math.sci.hiroshima-u.ac.jp (remove space)
*/
#![allow(
    warnings,
    clippy::all,
    unsafe_op_in_unsafe_fn,
    static_mut_refs,
    dangerous_implicit_autorefs
)]
pub const N: ::core::ffi::c_int = 624 as ::core::ffi::c_int;
pub const M: ::core::ffi::c_int = 397 as ::core::ffi::c_int;
pub const MATRIX_A: ::core::ffi::c_ulong = 0x9908b0df as ::core::ffi::c_ulong;
pub const UPPER_MASK: ::core::ffi::c_ulong = 0x80000000 as ::core::ffi::c_ulong;
pub const LOWER_MASK: ::core::ffi::c_ulong = 0x7fffffff as ::core::ffi::c_ulong;

#[unsafe(export_name = "__pdf_impl_init_genrand")]
pub unsafe extern "C-unwind" fn init_genrand(mut s: ::core::ffi::c_ulonglong) {
    let __pdf_state = unsafe { document_state() };
    (*__pdf_state).mt[0 as ::core::ffi::c_int as usize] =
        (s & 0xffffffff as ::core::ffi::c_ulonglong) as ::core::ffi::c_ulong;
    (*__pdf_state).mti = 1 as ::core::ffi::c_int;
    while (*__pdf_state).mti < N {
        (*__pdf_state).mt[(*__pdf_state).mti as usize] = (1812433253 as ::core::ffi::c_ulong)
            .wrapping_mul(
                (*__pdf_state).mt[((*__pdf_state).mti - 1 as ::core::ffi::c_int) as usize]
                    ^ (*__pdf_state).mt[((*__pdf_state).mti - 1 as ::core::ffi::c_int) as usize]
                        >> 30 as ::core::ffi::c_int,
            )
            .wrapping_add((*__pdf_state).mti as ::core::ffi::c_ulong);
        (*__pdf_state).mt[(*__pdf_state).mti as usize] &= 0xffffffff as ::core::ffi::c_ulong;
        (*__pdf_state).mti += 1;
    }
}
unsafe extern "C-unwind" fn genrand_int32() -> ::core::ffi::c_ulong {
    let __pdf_state = unsafe { document_state() };
    let mut y: ::core::ffi::c_ulong = 0;
    static mut mag01: [::core::ffi::c_ulong; 2] = [0 as ::core::ffi::c_ulong, MATRIX_A];
    if (*__pdf_state).mti >= N {
        let mut kk: ::core::ffi::c_int = 0;
        if (*__pdf_state).mti == N + 1 as ::core::ffi::c_int {
            init_genrand(5489 as ::core::ffi::c_ulonglong);
        }
        kk = 0 as ::core::ffi::c_int;
        while kk < N - M {
            y = (*__pdf_state).mt[kk as usize] & UPPER_MASK
                | (*__pdf_state).mt[(kk + 1 as ::core::ffi::c_int) as usize] & LOWER_MASK;
            (*__pdf_state).mt[kk as usize] = (*__pdf_state).mt[(kk + M) as usize]
                ^ y >> 1 as ::core::ffi::c_int
                ^ mag01[(y & 0x1 as ::core::ffi::c_ulong) as usize];
            kk += 1;
        }
        while kk < N - 1 as ::core::ffi::c_int {
            y = (*__pdf_state).mt[kk as usize] & UPPER_MASK
                | (*__pdf_state).mt[(kk + 1 as ::core::ffi::c_int) as usize] & LOWER_MASK;
            (*__pdf_state).mt[kk as usize] = (*__pdf_state).mt[(kk + (M - N)) as usize]
                ^ y >> 1 as ::core::ffi::c_int
                ^ mag01[(y & 0x1 as ::core::ffi::c_ulong) as usize];
            kk += 1;
        }
        y = (*__pdf_state).mt[(N - 1 as ::core::ffi::c_int) as usize] & UPPER_MASK
            | (*__pdf_state).mt[0 as ::core::ffi::c_int as usize] & LOWER_MASK;
        (*__pdf_state).mt[(N - 1 as ::core::ffi::c_int) as usize] = (*__pdf_state).mt
            [(M - 1 as ::core::ffi::c_int) as usize]
            ^ y >> 1 as ::core::ffi::c_int
            ^ mag01[(y & 0x1 as ::core::ffi::c_ulong) as usize];
        (*__pdf_state).mti = 0 as ::core::ffi::c_int;
    }
    let fresh0 = (*__pdf_state).mti;
    (*__pdf_state).mti = (*__pdf_state).mti + 1;
    y = (*__pdf_state).mt[fresh0 as usize];
    y ^= y >> 11 as ::core::ffi::c_int;
    y ^= y << 7 as ::core::ffi::c_int & 0x9d2c5680 as ::core::ffi::c_ulong;
    y ^= y << 15 as ::core::ffi::c_int & 0xefc60000 as ::core::ffi::c_ulong;
    y ^= y >> 18 as ::core::ffi::c_int;
    return y;
}
#[unsafe(export_name = "__pdf_impl_genrand_int31")]
pub unsafe extern "C-unwind" fn genrand_int31() -> ::core::ffi::c_long {
    return (genrand_int32() >> 1 as ::core::ffi::c_int) as ::core::ffi::c_long;
}
unsafe extern "C" {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_init_genrand(mut s: ::core::ffi::c_ulonglong) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { init_genrand(s) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_genrand_int31(_result: *mut ::core::ffi::c_long) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { genrand_int31() }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}

pub(crate) struct DocumentState {
    mt: [::core::ffi::c_ulong; 624],
    mti: ::core::ffi::c_int,
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        unsafe {
            Self {
                mt: [0; 624],
                mti: N + 1 as ::core::ffi::c_int,
            }
        }
    }
}

#[inline]
unsafe fn document_state() -> *mut DocumentState {
    unsafe {
        (*crate::compat::document::current())
            .backend
            .dpx_mt19937ar
            .pointer
    }
}
