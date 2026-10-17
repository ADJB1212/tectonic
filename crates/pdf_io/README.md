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

PDF generation separates ordered document interpretation from page rendering.
The coordinator resolves fonts, character maps, glyph usage, annotations, named
destinations, image references, and graphics state into owned page plans. Workers
render text and rules using explicit device contexts, then compress the page.
They never access the bridge, mutable PDF objects, or the output file. Results
are committed in submission order. Completed payloads become document storage;
object release is deferred until finalization so worker and queue limits do not
change deterministic output ordering.

A side-effect-free scan selects the legacy page renderer for unknown specials,
arbitrary PDF object mutation, raw PDF/PostScript, and virtual fonts. Common
page-size, color, image, annotation, destination, and outline specials resolve
on the coordinator. No special is replayed. Independent resource streams still
use the worker pipeline in compatibility mode.

PNG and TIFF image predictors process rasters of at least 256 KiB in independent
row groups, using the configured PDF executor. Workers read the
original raster and write separate output rows, preserving the serial predictor
bytes, including at group boundaries. Small rasters and systems where workers
cannot be created use the serial path. PDF object handling and output stay on
the engine thread.

Pages, image filtering, independent streams, and large-stream compression chunks
share one PDF worker budget. Workers default to the available CPU count. The
queue admits at most twice the worker count and 64 MiB of input payload, with
backpressure and an allowance for one oversized item when empty. This excludes
active-job scratch space and completed document storage. A worker limit of one,
or thread creation failure, uses synchronous execution. Metadata remains
uncompressed. All jobs finish before cross-reference offsets are finalized.
TeX layout and dependent passes retain their existing scheduling.

`PdfBuildOptions` configures worker and queue limits through `XdvipdfmxEngine`
and `ProcessingSessionBuilder`. The CLI exposes `--pdf-workers` and
`--pdf-queue-mib`. `XdvipdfmxEngine::statistics()` reports phase timings, summed
worker timings, queue wait, page/stream counts, and peak queued payload/jobs.
Worker elapsed times overlap each other and coordinator phases.

TIFF horizontal differencing supports 1, 2, 4, 8, and 16 bits per component.
This includes PNG transparency masks. Packed rows retain their byte alignment
and clear unused trailing bits.

Run the TIFF predictor benchmark with:

```sh
cargo test -p tectonic_pdf_io --release benchmark_pdf_tiff_predictor -- --ignored --nocapture
```

Compare TeX and PDF-generation time for complete documents with:

```sh
cargo test --release --test pdf_parallel benchmark_single_pdf_compilation -- --ignored --nocapture
```

The benchmark compares legacy, one-worker, and automatic-worker backends for
text, native fonts, graphics, images, and attachments. Set
`TECTONIC_PDF_BENCH_BACKEND` or `TECTONIC_PDF_BENCH_WORKLOAD` to run one case,
and `TECTONIC_PDF_BENCH_DIR` to export the PDFs for independent rendering checks.
Use `/usr/bin/time -l` on macOS or `/usr/bin/time -v` on Linux to record peak
memory for each backend in a separate process.

On a macOS host with 10 available CPUs, five warm release runs produced these
median PDF-stage times. The legacy column uses the original scheduling with the
shared number-formatting improvement also applied.

| Workload | Legacy | Automatic workers |
| --- | ---: | ---: |
| 200 text pages | 68.04 ms | 68.65 ms |
| 24 native-font pages | 46.48 ms | 45.96 ms |
| 100 rule-heavy pages | 53.37 ms | 53.17 ms |
| 16 independent 1024x1024 images | 148.98 ms | 122.36 ms |
| 128 attachments | 62.25 ms | 61.99 ms |

The image workload improved about 18% in the PDF stage and 16% end to end.
Separate image-only processes measured peak RSS of 253 MB for legacy scheduling
and 313 MB for automatic workers, including fixtures, font loading, and repeated
builds. Reduce workers or the queue budget when memory is constrained. Plain
text remains dominated by ordered interpretation and setup; these results do
not imply a comparable speedup for every document.


## Cargo features

This crate currently provides no [Cargo features][features].

[features]: https://doc.rust-lang.org/cargo/reference/features.html
