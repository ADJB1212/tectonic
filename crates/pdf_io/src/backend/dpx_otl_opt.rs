/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
unsafe extern "C-unwind" {
    fn __assert_rtn(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> !;
    fn isalpha(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isdigit(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dpx_warning(fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "__pdf_impl_new"]
    fn new(size: uint32_t) -> *mut ::core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct otl_opt {
    pub rule: *mut bt_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bt_node {
    pub flag: ::core::ffi::c_int,
    pub left: *mut bt_node,
    pub right: *mut bt_node,
    pub data: [::core::ffi::c_char; 4],
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const FLAG_NOT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const FLAG_AND: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
unsafe extern "C-unwind" fn match_expr(mut expr: *mut bt_node, mut key: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut retval: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    if !expr.is_null() {
        if (*expr).left.is_null() && (*expr).right.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                if (*expr).data[i as usize] as ::core::ffi::c_int != '?' as i32
                    && (*expr).data[i as usize] as ::core::ffi::c_int != *key.offset(i as isize) as ::core::ffi::c_int
                {
                    retval = 0 as ::core::ffi::c_int;
                    break;
                } else {
                    i += 1;
                }
            }
        } else {
            if !(*expr).left.is_null() {
                retval = match_expr((*expr).left, key);
            }
            if !(*expr).right.is_null() {
                if retval != 0 && (*expr).flag & FLAG_AND != 0 {
                    retval &= match_expr((*expr).right, key);
                } else if retval == 0 && (*expr).flag & FLAG_AND == 0 {
                    retval = match_expr((*expr).right, key);
                }
            }
        }
        if (*expr).flag & FLAG_NOT != 0 {
            retval = if retval != 0 { 0 as ::core::ffi::c_int } else { 1 as ::core::ffi::c_int };
        }
    }
    return retval;
}
unsafe extern "C-unwind" fn bt_new_tree() -> *mut bt_node {
    let mut expr: *mut bt_node = ::core::ptr::null_mut::<bt_node>();
    expr = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<bt_node>() as usize) as uint32_t) as *mut bt_node;
    (*expr).flag = 0 as ::core::ffi::c_int;
    (*expr).left = ::core::ptr::null_mut::<bt_node>();
    (*expr).right = ::core::ptr::null_mut::<bt_node>();
    memset(
        &raw mut (*expr).data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        4 as size_t,
    );
    return expr;
}
unsafe extern "C-unwind" fn bt_release_tree(mut tree: *mut bt_node) {
    if !tree.is_null() {
        if !(*tree).left.is_null() {
            bt_release_tree((*tree).left);
        }
        if !(*tree).right.is_null() {
            bt_release_tree((*tree).right);
        }
        free(tree as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C-unwind" fn parse_expr(mut pp: *mut *const ::core::ffi::c_char, mut endptr: *const ::core::ffi::c_char) -> *mut bt_node {
    let mut root: *mut bt_node = ::core::ptr::null_mut::<bt_node>();
    let mut curr: *mut bt_node = ::core::ptr::null_mut::<bt_node>();
    if *pp >= endptr {
        return ::core::ptr::null_mut::<bt_node>();
    }
    curr = bt_new_tree();
    root = curr;
    while *pp < endptr {
        match **pp as ::core::ffi::c_int {
            33 => {
                if (*curr).flag & 2 as ::core::ffi::c_int != 0 {
                    (*curr).flag &= !FLAG_NOT;
                } else {
                    (*curr).flag |= FLAG_NOT;
                }
                *pp = (*pp).offset(1);
            }
            40 => {
                *pp = (*pp).offset(1);
                if *pp < endptr {
                    let mut expr: *mut bt_node = ::core::ptr::null_mut::<bt_node>();
                    expr = parse_expr(pp, endptr);
                    if expr.is_null() {
                        pdf_warning!(b"Syntax error: %s\n\0" as *const u8 as *const ::core::ffi::c_char, *pp,);
                        return ::core::ptr::null_mut::<bt_node>();
                    }
                    if **pp as ::core::ffi::c_int != ')' as i32 {
                        pdf_warning!(b"Syntax error: Unbalanced ()\n\0" as *const u8 as *const ::core::ffi::c_char,);
                        return ::core::ptr::null_mut::<bt_node>();
                    }
                    (*curr).left = (*expr).left;
                    (*curr).right = (*expr).right;
                    memcpy(
                        &raw mut (*curr).data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        &raw mut (*expr).data as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                        4 as size_t,
                    );
                    free(expr as *mut ::core::ffi::c_void);
                } else {
                    pdf_warning!(b"Syntax error: Unbalanced ()\n\0" as *const u8 as *const ::core::ffi::c_char,);
                    bt_release_tree(root);
                    return ::core::ptr::null_mut::<bt_node>();
                }
                *pp = (*pp).offset(1);
            }
            41 => return root,
            124 | 38 => {
                if *pp >= endptr {
                    pdf_warning!(b"Syntax error: %s\n\0" as *const u8 as *const ::core::ffi::c_char, *pp,);
                    bt_release_tree(root);
                    return ::core::ptr::null_mut::<bt_node>();
                } else {
                    let mut tmp: *mut bt_node = ::core::ptr::null_mut::<bt_node>();
                    tmp = bt_new_tree();
                    (*tmp).left = root;
                    curr = bt_new_tree();
                    (*tmp).right = curr;
                    if **pp as ::core::ffi::c_int == '&' as i32 {
                        (*tmp).flag = 1 as ::core::ffi::c_int;
                    } else {
                        (*tmp).flag = 0 as ::core::ffi::c_int;
                    }
                    root = tmp;
                }
                *pp = (*pp).offset(1);
            }
            42 => {
                memset(
                    &raw mut (*curr).data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    '?' as i32,
                    4 as size_t,
                );
                *pp = (*pp).offset(1);
            }
            _ => {
                if (*pp).offset(4 as ::core::ffi::c_int as isize) <= endptr {
                    let mut i: ::core::ffi::c_int = 0;
                    i = 0 as ::core::ffi::c_int;
                    while i < 4 as ::core::ffi::c_int {
                        if **pp as ::core::ffi::c_int == ' ' as i32
                            || **pp as ::core::ffi::c_int == '?' as i32
                            || isalpha(**pp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
                            || isdigit(**pp as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0
                        {
                            (*curr).data[i as usize] = **pp;
                        } else if **pp as ::core::ffi::c_int == '_' as i32 {
                            (*curr).data[i as usize] = ' ' as i32 as ::core::ffi::c_char;
                        } else {
                            pdf_warning!(
                                b"Invalid char in tag: %c\n\0" as *const u8 as *const ::core::ffi::c_char,
                                **pp as ::core::ffi::c_int,
                            );
                            bt_release_tree(root);
                            return ::core::ptr::null_mut::<bt_node>();
                        }
                        *pp = (*pp).offset(1);
                        i += 1;
                    }
                } else {
                    pdf_warning!(b"Syntax error: %s\n\0" as *const u8 as *const ::core::ffi::c_char, *pp,);
                    bt_release_tree(root);
                    return ::core::ptr::null_mut::<bt_node>();
                }
            }
        }
    }
    return root;
}
#[unsafe(export_name = "__pdf_impl_otl_new_opt")]
pub unsafe extern "C-unwind" fn otl_new_opt() -> *mut otl_opt {
    let mut opt: *mut otl_opt = ::core::ptr::null_mut::<otl_opt>();
    opt = new((1 as ::core::ffi::c_int as uint32_t as usize).wrapping_mul(::core::mem::size_of::<otl_opt>() as usize) as uint32_t) as *mut otl_opt;
    (*opt).rule = ::core::ptr::null_mut::<bt_node>();
    return opt as *mut otl_opt;
}
#[unsafe(export_name = "__pdf_impl_otl_release_opt")]
pub unsafe extern "C-unwind" fn otl_release_opt(mut opt: *mut otl_opt) {
    if !(*opt).rule.is_null() {
        bt_release_tree((*opt).rule);
    }
    (*opt).rule = ::core::ptr::null_mut::<bt_node>();
    free(opt as *mut ::core::ffi::c_void);
}
#[unsafe(export_name = "__pdf_impl_otl_parse_optstring")]
pub unsafe extern "C-unwind" fn otl_parse_optstring(mut opt: *mut otl_opt, mut optstr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut endptr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if opt.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"opt\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if !optstr.is_null() {
        p = optstr;
        endptr = p.offset(strlen(optstr) as isize);
        (*opt).rule = parse_expr(&raw mut p, endptr);
    }
    return 0 as ::core::ffi::c_int;
}
#[unsafe(export_name = "__pdf_impl_otl_match_optrule")]
pub unsafe extern "C-unwind" fn otl_match_optrule(mut opt: *mut otl_opt, mut tag: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if tag.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        pdf_abort!(c"assertion failed: %s".as_ptr(), b"tag\0" as *const u8 as *const ::core::ffi::c_char);
    } else {
    };
    if opt.is_null() || (*opt).rule.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    return match_expr((*opt).rule, tag);
}
unsafe extern "C" {
    fn memcpy(__dst: *mut ::core::ffi::c_void, __src: *const ::core::ffi::c_void, __n: size_t) -> *mut ::core::ffi::c_void;
    fn memset(__b: *mut ::core::ffi::c_void, __c: ::core::ffi::c_int, __len: size_t) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_otl_new_opt(_result: *mut *mut otl_opt) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otl_new_opt() }) {
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
pub unsafe extern "C" fn __pdf_entry_otl_release_opt(mut opt: *mut otl_opt) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otl_release_opt(opt) }) {
        Ok(value) => {
            let _ = value;
            0
        }
        Err(()) => -1,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __pdf_entry_otl_parse_optstring(
    mut opt: *mut otl_opt, mut optstr: *const ::core::ffi::c_char, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otl_parse_optstring(opt, optstr) }) {
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
pub unsafe extern "C" fn __pdf_entry_otl_match_optrule(
    mut opt: *mut otl_opt, mut tag: *const ::core::ffi::c_char, _result: *mut ::core::ffi::c_int,
) -> i32 {
    match crate::backend::runtime::boundary(|| unsafe { otl_match_optrule(opt, tag) }) {
        Ok(value) => {
            unsafe {
                _result.write(value);
            }
            0
        }
        Err(()) => -1,
    }
}
