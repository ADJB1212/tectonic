// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

use std::{env, fs, path::PathBuf};

fn main() {
    let mut build = cc::Build::new();
    build
        .include("pdf_io")
        .include(env::var("DEP_TECTONIC_BRIDGE_CORE_INCLUDE").unwrap())
        .include(env::var("DEP_TECTONIC_BRIDGE_FLATE_INCLUDE").unwrap())
        .flag_if_supported("-std=gnu11");

    for path in env::var("DEP_PNG_INCLUDE_PATH").unwrap().split(';') {
        build.include(path);
    }

    println!("cargo:rerun-if-changed=compat");
    println!("cargo:rerun-if-changed=pdf_io");
    let mut adapters: Vec<_> = fs::read_dir("compat")
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "c"))
        .collect();
    adapters.sort();
    build.files(adapters).compile("tectonic_pdf_io_compat");

    let headers = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("pdf_io");
    println!("cargo:include-path={}", headers.display());
}
