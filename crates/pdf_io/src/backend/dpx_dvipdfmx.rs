/* This is (x)dvipdfmx, an extended version of...

    DVIPDFMx, an eXtended version of DVIPDFM by Mark A. Wicks.

    Copyright (C) 2002-2020 by Jin-Hwan Cho, Matthias Franz, Shunsaku Hirata,
    the DVIPDFMx project team.

    Copyright (c) 2006 SIL. (xdvipdfmx extensions for XeTeX support)

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
pub type __darwin_time_t = ::core::ffi::c_long;
pub type time_t = libc::time_t;
#[unsafe(no_mangle)]
pub static mut ttpi_source_date_epoch: time_t = -(1 as ::core::ffi::c_int) as time_t;
#[unsafe(no_mangle)]
pub static mut landscape_mode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[unsafe(no_mangle)]
pub static mut paper_height: ::core::ffi::c_double = 842.0f64;
#[unsafe(no_mangle)]
pub static mut paper_width: ::core::ffi::c_double = 595.0f64;
unsafe extern "C" {}
