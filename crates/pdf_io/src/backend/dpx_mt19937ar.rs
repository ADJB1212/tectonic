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
#[unsafe(export_name = "__pdf_impl_init_genrand")]
pub unsafe extern "C-unwind" fn init_genrand(s: ::core::ffi::c_ulonglong) {
    unsafe { &mut *document_state() }.generator.reseed(s as u32);
}
#[unsafe(export_name = "__pdf_impl_genrand_int31")]
pub unsafe extern "C-unwind" fn genrand_int31() -> ::core::ffi::c_long {
    (unsafe { &mut *document_state() }.generator.next_u32() >> 1) as ::core::ffi::c_long
}
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
    generator: rand_mt::Mt,
}

impl DocumentState {
    pub(crate) fn new() -> Self {
        Self {
            generator: rand_mt::Mt::new_unseeded(),
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
