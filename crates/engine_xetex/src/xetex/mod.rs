// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Preserve the source structure of the translated engine modules.
#![allow(clippy::all)]
#![warn(clippy::correctness)]
#![allow(unused_assignments, unused_macros, unused_parens, unreachable_code)]
// Rust bridge values cross FFI only behind opaque pointers.
#![allow(improper_ctypes)]

mod bridge;
pub mod teckit_c_engine;
pub mod teckit_common;
pub mod teckit_compiler;
pub mod teckit_cxx_engine;
pub mod teckit_engine;
pub mod teckit_format;
pub mod xetex_bindings;
pub mod xetex_checkpoint;
mod xetex_cleanup;
pub mod xetex_constants;
pub mod xetex_core;
pub mod xetex_engine_interface;
pub mod xetex_errors;
pub mod xetex_ext;
pub mod xetex_format;
pub mod xetex_ini;
mod xetex_initialize;
pub mod xetex_io;
pub mod xetex_linebreak;
pub mod xetex_macos;
pub mod xetex_math;
pub mod xetex_output;
pub mod xetex_pagebuilder;
pub mod xetex_pic;
mod xetex_runtime;
pub mod xetex_scaledmath;
pub mod xetex_shipout;
pub mod xetex_stringpool;
pub mod xetex_swap;
pub mod xetex_synctex;
pub mod xetex_texmfmp;
pub mod xetex_xetex0;
pub mod xetex_xetex_ot_math;
pub mod xetex_xetexd;

#[cfg(test)]
mod tests;
