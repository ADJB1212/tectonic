// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! This crate exports a *C* API for I/O on PDF, XDV, and graphics files, with
//! Rust workers for independent image processing, within the Cargo build
//! framework used by [Tectonic].
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/

mod predictor;

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
