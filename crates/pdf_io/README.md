# The `tectonic_pdf_io` crate

[![](http://meritbadge.herokuapp.com/tectonic_pdf_io)](https://crates.io/crates/tectonic_pdf_io)

This crate is part of [the Tectonic
project](https://tectonic-typesetting.github.io/en-US/). It provides the PDF,
XDV, and image I/O capabilities of [XeTeX]’s `xdvipdfmx` as a crate, *currently
providing only a C API*.

[XeTeX]: http://xetex.sourceforge.net/

- [API documentation](https://docs.rs/tectonic_pdf_io/).
- [Main Git repository](https://github.com/tectonic-typesetting/tectonic/).

This crate depends on [libpng], which it attempts to find externally using
[pkg-config] or [vcpkg]. It should be possible to create another "bridge" crate
so that this external dependency could be avoided, but this hasn’t been done
yet.

[libpng]: http://www.libpng.org/pub/png/libpng.html
[pkg-config]: https://www.freedesktop.org/wiki/Software/pkg-config/
[vcpkg]: https://github.com/microsoft/vcpkg

If your project depends on this crate, Cargo will export for your build script
an environment variable named `DEP_TECTONIC_PDF_IO_INCLUDE_PATH`, which will be
a semicolon-separated list of directories enabling your code to include the
`dpx-*.h` headers.

You will need to ensure that your Rust code actually references this crate in
order for the linker to include linked libraries. A `use` statement will
suffice:

```rust
#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use tectonic_pdf_io;
```

PNG and TIFF image predictors process rasters of at least 256 KiB in independent
row groups, using the available CPU count up to 16 workers. Workers read the
original raster and write separate output rows, preserving the serial predictor
bytes, including at group boundaries. Small rasters and systems where workers
cannot be created use the serial path. PDF object handling and output stay on
the engine thread.

Page content streams of at least 1 KiB and other streams of at least 16 KiB
are compressed concurrently in batches of up to 64 streams or 8 MiB of input.
Streams of at least 1 MiB retain the existing parallel chunk compression path.
Image streams requiring predictors retain their existing row-parallel path,
and metadata remains uncompressed. Batches finish before cross-reference offsets
are finalized. Object order can change, while decoded contents remain the same.
Compression and image filtering share a worker pool, with a serial fallback.
TeX layout and its dependent passes still run sequentially.

TIFF horizontal differencing supports 1, 2, 4, 8, and 16 bits per component.
This includes PNG transparency masks. Packed rows retain their byte alignment
and clear unused trailing bits.

Run the TIFF predictor benchmark with:

```sh
cargo test -p tectonic_pdf_io --release benchmark_pdf_tiff_predictor -- --ignored --nocapture
```

Compare TeX and PDF-generation time for complete documents with:

```sh
cargo test --test pdf_parallel benchmark_single_pdf_compilation -- --ignored --nocapture
```


## Cargo features

This crate currently provides no [Cargo features][features].

[features]: https://doc.rust-lang.org/cargo/reference/features.html
