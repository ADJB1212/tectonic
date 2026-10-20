// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Rust implementation of PDF, XDV, and graphics I/O with a C compatibility API.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/

#[cfg(panic = "abort")]
compile_error!("tectonic_pdf_io requires panic = \"unwind\" to recover PDF errors before returning to C");

#[macro_use]
mod backend;
mod bmp;
mod byte_cursor;
mod cff;
mod compat;
mod compress;
mod diagnostics;
mod error;
mod fontmap;
mod input;
mod jp2;
mod jpeg;
mod line;
mod numbers;
mod objects;
mod options;
mod pipeline;
mod pk;
mod predictor;
mod preflight;
mod raster;
mod renderer;
mod sfnt;
mod syntax;
mod tfm;
mod type1;
mod type2;
mod unicode;
mod vfont;

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
