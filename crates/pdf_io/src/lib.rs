// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Rust implementation of PDF, XDV, and graphics I/O with a C compatibility API.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/

#[cfg(panic = "abort")]
compile_error!(
    "tectonic_pdf_io requires panic = \"unwind\" to recover PDF errors before returning to C"
);

#[macro_use]
mod backend;
mod byte_cursor;
mod compat;
mod compress;
mod diagnostics;
mod error;
mod fontmap;
mod input;
mod line;
mod numbers;
mod options;
mod pipeline;
mod predictor;
mod preflight;
mod renderer;
mod sfnt;
mod tfm;
mod unicode;

pub use options::PdfBuildOptions;
pub use pipeline::{PdfBuildStatistics, PdfRuntime};

/// Release Rust-owned inputs after the native PDF engine returns, including on abort.
///
/// # Safety
/// Native code must no longer use pointers borrowed from Rust-owned input storage.
pub unsafe fn clear_input_state() {
    if compat::document::active() {
        preflight::clear_page_scanner();
        fontmap::tt_fontmap_clear();
    }
}

/// Import things from our bridge crates to ensure that we actually link with
/// them.
mod linkage {
    #[allow(unused_imports)]
    use tectonic_bridge_core as clippyrenamehack1;

    #[allow(unused_imports)]
    use tectonic_bridge_flate as clippyrenamehack2;

    #[allow(unused_imports)]
    use tectonic_bridge_png as clippyrenamehack3;
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
