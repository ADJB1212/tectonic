#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

#[inline]
pub fn SWAP16(p: u16) -> u16 {
    p.to_be()
}

#[inline]
pub fn SWAP32(p: u32) -> u32 {
    p.to_be()
}

pub trait SwapExt {
    fn swap_xetex(self) -> Self;
}

impl SwapExt for u16 {
    #[inline]
    fn swap_xetex(self) -> Self {
        SWAP16(self)
    }
}

impl SwapExt for i16 {
    #[inline]
    fn swap_xetex(self) -> Self {
        SWAP16(self as u16) as i16
    }
}

impl SwapExt for u32 {
    #[inline]
    fn swap_xetex(self) -> Self {
        SWAP32(self)
    }
}

impl SwapExt for i32 {
    #[inline]
    fn swap_xetex(self) -> Self {
        SWAP32(self as u32) as i32
    }
}

#[inline]
pub fn SWAP<T: SwapExt>(p: T) -> T {
    p.swap_xetex()
}
